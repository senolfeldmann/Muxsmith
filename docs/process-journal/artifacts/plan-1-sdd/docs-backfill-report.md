# Rustdoc backfill report (2026-07-08)

## Scope

Documented every public item flagged by `RUSTFLAGS="-W missing-docs" cargo check --workspace`
(151 warnings across muxsmith-core, muxsmith-cli, xtask) and gated the workspace with
`#![deny(missing_docs)]` in the three library crate roots.

## Per-crate summary

**muxsmith-core** (largest share): capability model (`PropType`, `matchable_type`,
`settable`, `codec_kind_prefixes`), the entire profile data model (`Profile`, `Meta`,
`Input`, `OutputCfg`, `FilenameCfg`/`CollisionPolicy`/`KeepDrop`, `TrackRule`,
`SourceCfg`, `Locator`, `AttachmentsCfg`/`AttachmentRule`, `ChaptersCfg`, `TagsCfg`,
`TitleCfg` and every field), the match algebra (`Scalar`, `MatchExpr` and its fields,
`is_empty`), `profile::load` (`Format`, `from_str`, `from_file`), `profile::lint`
(`provable_overlaps`), `profile::validate` (`validate`), the diagnostics catalog
(`Severity`, `DiagCode` incl. the `diag_codes!` macro, `Diagnostic` and its builder
methods, `worst_severity`), and the template engine (`Filter`, `TemplateError`,
`Template`, `Ctx`, `parse`/`field_names`/`render_literal`). Added module-level `//!`
docs to `profile` and `profile::load` (previously undocumented), plus the crate-root
`//!` overview and `#![deny(missing_docs)]` in `lib.rs`.

**muxsmith-cli**: `cli::Cli`/`cli::Cmd` (incl. the previously-undocumented `profile`
field), `commands::validate::run`, `i18n::Renderer` and its `msg`/`diagnostic` methods.
Added module docs to `cli.rs`, `commands/mod.rs`, `commands/validate.rs`, plus the
crate-root `//!` overview, `#![deny(missing_docs)]` in `lib.rs`, and a `//!` on
`main.rs` (binary crate root; no deny needed there per task instructions).

**xtask**: added the crate-root `//!` overview + `#![deny(missing_docs)]` to `lib.rs`,
a module `//!` to `codegen.rs` (the `generate` fn itself was already documented), and
a `//!` on `main.rs` (binary, no deny).

## Gate output (verbatim tails)

```
$ cargo check --workspace
    Checking muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s

$ cargo doc --workspace --no-deps        (also re-checked with RUSTDOCFLAGS="-D warnings", zero output)
 Documenting xtask v0.1.0 (/home/senol/Git/Muxsmith/crates/xtask)
 Documenting muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
 Documenting muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 3 other files

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s

$ cargo test --workspace
total passed: 81
total failed: 0
```

## Diff shape

`git diff --stat -- crates`: 19 files changed, 392 insertions(+), 4 deletions(-).
Every file is pure addition except two mechanical reformattings needed to attach
per-field/per-variant doc comments to previously single-line declarations (no
behavioral change, confirmed by the identical 81/81 test pass before and after):

- `report.rs`: the `diag_codes!` macro's `$($variant),+` enum body became
  `$( #[doc = "..."] $variant ),+` so every generated `DiagCode` variant gets a doc
  attribute. All 29 macro *call-site* lines (`Variant => "key",`) are untouched.
- `template.rs`: `TemplateError`'s three struct-like variants (`UnclosedBrace { pos:
  usize }` etc.) were broken onto multiple lines so each named field could carry its
  own `///`.

## Judgment calls worth reviewing

1. **`DiagCode` variants share one generic doc string.** The macro that generates
   `DiagCode` (`report.rs`) takes only `variant => "key"` pairs; there is no channel
   to carry distinct per-variant prose without changing the macro's call-site grammar
   (which the task's "diff must be doc-comments only" gate rules out). Each of the 29
   variants got the same `#[doc = "See the spec 5.2 catalog table for this code's
   condition and severity."]` rather than bespoke text; the enum-level doc explains
   why and points at the spec 5.2 table as the authoritative source of per-code
   semantics.
2. **Two mechanical reformattings** (macro body enum, `TemplateError` variants) were
   necessary to physically place field/variant-level doc comments; see "Diff shape"
   above. Confirmed behavior-neutral via identical test count/results before and
   after formatting the diff was produced.
3. **Binaries got `//!` overviews despite "binaries need nothing."** The task's
   enforcement section exempts `main.rs` from `#![deny(missing_docs)]`, but the
   documentation-standard section separately counts "5 crate roots" missing docs
   (matching the 5 `missing documentation for the crate` warnings found: both
   lib.rs's-with-main.rs pairs plus xtask's). Added a `//!` to both `muxsmith-cli/src/
   main.rs` and `xtask/src/main.rs` to satisfy that count; no `#![deny(missing_docs)]`
   added to either, per the enforcement instruction.
4. **`Scalar`'s untagged variant ordering** documented per the task's explicit
   requirement ("Scalar untagged ordering relevance"): `Bool` before `Int`/`Float`
   before `Str`, so a bare `true` binds as boolean and a plain integer literal binds
   as `Int` rather than `Float`.
5. Private items (e.g. `Segment`, `flatten_regex_error`, `Ctx::get`, `collect` in
   `commands/validate.rs`) were left undocumented per instructions ("do not document
   private items unless genuinely non-obvious") — none of them cleared that bar.

## Follow-up: per-variant DiagCode rustdoc (coordinator request)

Judgment call 1 above is superseded. The `diag_codes!` macro grammar was extended to
`$($(#[$meta:meta])* $variant:ident => $key:literal),+ $(,)?` with the enum body
forwarding `$($(#[$meta])* $variant),+`; `ALL` and `key()` keep bare `$variant`. All
30 variants now carry real one-line `///` docs at the call site: config-time codes
sourced from actual validate.rs/lint.rs/load.rs behavior (e.g. UnknownProperty is
documented as match-condition-only, since changes go through UnknownSettableProperty
in this implementation, diverging slightly from the spec 5.2 row that folds both),
planning-time codes from the spec 5.2 catalog table. The generic shared
`#[doc = "See the spec 5.2 catalog table..."]` attribute and the stale macro
justification doc were removed. Section comments (config-time / planning-time) kept.

Gates re-run, all green: `cargo check --workspace` clean (deny active);
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` clean; rendered
`enum.DiagCode.html` verified to carry the per-variant text; `cargo fmt --all
--check` clean; `cargo clippy --workspace --all-targets -- -D warnings` clean;
`cargo test --workspace` 81 passed / 0 failed.

Diff: 1 file (report.rs), 35 insertions, 9 deletions (the 9 removed lines are the
stale macro doc and the generic doc attribute block).
