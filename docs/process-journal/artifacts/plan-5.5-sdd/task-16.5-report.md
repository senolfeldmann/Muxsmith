# Task 16.5 report: once-per-batch schema-drift notice (D32 addendum, Şenol)

Rebuilt the general "your mkvmerge's identification schema is newer than this
build pins" signal as its own diagnostic: `SchemaDrift`, once per batch, info
severity, decoupled entirely from `raw:` consumption (that stays
`UnknownPropertySkew`'s job, per consumed property). Worktree
`.worktrees/t165`, branch `plan55-t165`, based on post-T21 master. TDD: three
failing tests written against the not-yet-existing `DiagCode::SchemaDrift`
(compile failure), then implemented to green; additionally verified true
red/green by temporarily commenting out the new call site and re-running
(one test failed with the expected assertion mismatch, the other two stayed
green by construction) before restoring.

## Emission placement

`plan_core` (`planner.rs`) now collects `found_versions: Vec<u64>` across the
primaries loop: `resolve_file`'s return tuple grows a third element,
`Option<u64>` - `Some(ident.format_version)` the moment identification
succeeds (before the container-recognized/supported check, matching the old
removed code's point of capture exactly), `None` only when `identify()`
itself errors. After the loop, `detect_schema_drift(&found_versions, &mut
batch_diagnostics)` runs next to `validate_language_values`/
`validate_extension_values` in the batch-walk group - it can't run *before*
the loop like those two, since format versions only exist once identification
has happened; that's an unavoidable ordering fact, not a placement choice.
`detect_schema_drift` takes the max of every version exceeding
`PINNED_IDENTIFICATION_FORMAT_VERSION` (`capability::mod::PINNED_IDENTIFICATION_FORMAT_VERSION`,
reused, not re-hardcoded) and pushes exactly one `Diagnostic::info(SchemaDrift,
"input")` with `found_version`/`pinned` params when that max exists, none
otherwise - naturally at-most-one and naturally degrading when
`found_versions` is empty (no file identified at all).

**Scope decision, made explicit:** `found_versions` only ever receives
*primary* files' versions, never donors' (external sources, chapters/
attachments locators). This mirrors the exact scope of the removed T16 code
verbatim - I diffed it (`git show 27c8b79 -- planner.rs`): the old check sat
immediately after `let ident = id.identify(&primary.path)` succeeded, never
touched the donor `id.identify(&donor)` call sites elsewhere in
`resolve_file`. The brief's "ANY identified file in the batch" and the
acceptance framing ("two newer-schema files -> exactly ONE SchemaDrift") both
read as batch-of-primaries language, consistent with this. Flagging this as a
deliberate reconstruction of removed behavior, not an independent design
call - if donor coverage was actually wanted, that's a real scope extension
(donors identify deep inside the external-source branch, not near the
primaries loop) worth a separate look, not a redo of this task.

## Message wording (EN + DE, both born this commit)

EN (`locales/en/diagnostics.ftl`):
> This build pins mkvmerge identification schema version { $pinned }; at
> least one identified file in this batch reports schema version
> { $found_version }. Any property the newer schema adds sits outside the
> capability model; use a raw: prefix to match it untyped.

DE (`locales/de/diagnostics.ftl`):
> Dieser Build fixiert die mkvmerge-Identifikationsschema-Version { $pinned
> }; mindestens eine identifizierte Datei in diesem Stapel meldet
> Schema-Version { $found_version }. Jede von der neueren Version
> hinzugefügte Eigenschaft liegt außerhalb des Fähigkeitsmodells; nutze ein
> raw:-Präfix, um sie untypisiert abzugleichen.

Structure mirrors `unknown-property-skew`'s existing "this build pins ...;
this file reports ..." phrasing (batch-of-files variant), and carries the
required raw: discovery hint as its second sentence. German reuses the
catalog's established terms verified against the T21 corrections commit
(`362db2d`) and the surrounding catalog: "Fähigkeitsmodell" (capability
model, already used by `raw-property`/`raw-on-known-property`/
`unknown-property-skew`), "Stapel" (batch, matches `gui-batch.ftl`'s
`batch-view-heading = Stapel`), du-Imperativ for the actionable hint
("nutze ...", same register as `locator-conflict`'s "setze nur eines").

## TDD evidence

Three tests added to `crates/muxsmith-core/tests/planner_resolution.rs`,
placed right after the B-9..B-11 `raw:` skew block (thematically the closest
existing group):

| Test | Setup | Asserts |
|---|---|---|
| `schema_drift_fires_once_per_batch_with_the_max_found_version` | two primaries, versions 21 and 23 | exactly one `SchemaDrift` in `batch_diagnostics`, `Severity::Info`, `found_version="23"` (the max, not first/last), `pinned="20"` |
| `schema_drift_all_pinned_batch_yields_none` | two primaries, both version 20 | no `SchemaDrift` |
| `schema_drift_degrades_when_no_file_in_the_batch_identifies` | empty `FakeIdent::by_name` (mirrors `unidentifiable_primary_yields_unidentifiable_source_not_missing_track`'s idiom for "mkvmerge absent") | no `SchemaDrift`; per-file `UnidentifiableSource` still fires (degrade is silent, not a masking bug) |

Red confirmed concretely: commented out the `detect_schema_drift(...)` call
site, re-ran the three tests -
`schema_drift_fires_once_per_batch_with_the_max_found_version` failed
(`assertion left == right failed: batch diags: [] / left: 0 / right: 1`), the
other two passed vacuously (they assert absence, which holds trivially
without the call - expected, not a gap: the positive-case test is the one
that actually exercises the new code path). Restored the call, all three
green, full `cargo test --workspace` re-run clean (no regressions across the
415+ existing tests).

## Files changed

- `crates/muxsmith-core/src/report/mod.rs`: `SchemaDrift => "schema-drift"`
  variant + doc comment, inserted right after `UnknownPropertySkew`.
- `crates/muxsmith-core/src/planner.rs`: `resolve_file` return type gains
  `Option<u64>` (all three return sites updated); `plan_core` collects
  `found_versions` and calls the new `detect_schema_drift`; new
  `detect_schema_drift` function next to `validate_extension_list`/before
  `walk_exact_languages`.
- `crates/muxsmith-cli/tests/catalog_completeness.rs`: `fixture_args` arm for
  `DiagCode::SchemaDrift` (`found_version="21"`, `pinned="20"`) - the
  exhaustive match forced this in the same commit, as expected.
- `crates/muxsmith-core/tests/planner_resolution.rs`: the three tests above.
- `locales/en/diagnostics.ftl`, `locales/de/diagnostics.ftl`: `schema-drift`
  entry, both catalogs, same commit (bilingual from birth per the
  hardening-block rule).

## Gate (nine parts, all green)

1. `cargo fmt --all --check` - clean.
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean.
3. `cargo test --workspace` - all green (32/4/2/5/11/11/2/116/15/4/2/1/1/1/12/7/69/8/8/7/3/3/12/10/19/15/78 across the workspace's test binaries, 0 failed).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean.
5. `cargo deny check` - advisories/bans/licenses/sources ok.
6. `pnpm install --frozen-lockfile` - ran once (fresh worktree, node_modules absent).
7. `pnpm lint` - clean.
8. `pnpm build` - `vue-tsc --noEmit && vite build`, clean.
9. `pnpm check:i18n` - ok (12 pre-existing "unused" warnings, all pre-dating this task - the shell IpcError codes the script's own header comment documents as a known residual false positive; parity check passed, `schema-drift` present in both `locales/en/` and `locales/de/diagnostics.ftl`).
10. `pnpm test:e2e` - 5/5 passed, including the German-locale test; `e2e/i18n-en.ts` (imported at module load by `smoke.spec.ts`) is the real-Fluent-parser completeness+parity guard over every `locales/<tag>/` directory (en+de) - it hard-parses both new catalog entries and would have thrown at import time on a malformed/dropped id. It stayed green.

(Nine BUILDING.md parts plus the frozen-lockfile install step, all foreground.)

## Self-review

- No frontend (TS/Vue) code change needed: the GUI globs
  `locales/*/diagnostics.ftl` and renders by `code`, so `schema-drift` is
  picked up automatically - confirmed by the e2e run, not just asserted.
- `check-i18n.mjs`'s cross-locale parity check is id-set-only (no
  placeable/selector comparison); the real placeable-equality guarantee for
  `schema-drift` comes from `e2e/i18n-en.ts`'s real Fluent parse of both
  catalogs succeeding, which it did.
- Considered gating `detect_schema_drift` behind `id.known_extensions()`
  (`Option<_>`, "mkvmerge absent" signal used elsewhere) to literally mirror
  `validate_extension_values`'s degrade mechanism. Rejected: format_version
  comes from each file's own successful `identify()` call, not from the
  separate `--list-types` capability query: gating on the latter would be an
  unrelated coupling with no semantic basis, and it would also diverge from
  every existing `FakeIdent`-based test (which already leaves
  `known_extensions()` at its `None` default throughout the suite, degrade
  path already implicitly exercised by every other test). The natural
  `Option<u64>`-per-file collection already produces the wanted degrade
  (empty `found_versions` when nothing identified) without that coupling,
  and it is what the `schema_drift_degrades_when_no_file_in_the_batch_identifies`
  test actually exercises.
- Considered whether the container-not-recognized/supported early return in
  `resolve_file` should still contribute its `format_version`. Decided yes
  (matches old code's capture point, which ran *before* that check) - a file
  mkvmerge genuinely identified but whose container isn't muxable still
  proves the runtime speaks a newer schema; excluding it would silently
  under-report drift on exactly the files most likely to be schema-novel
  (an unrecognized/unsupported container is itself often a symptom of
  runtime skew).

## Concerns

- **Primaries-only scope** (see "Scope decision" above) is my read of an
  underspecified brief phrase ("ANY identified file in the batch"),
  reconstructed from the removed code's actual behavior rather than
  independently decided. High confidence given the git-archaeology match,
  but it is an interpretive call on an open dimension, not a instruction
  Şenol stated directly for the batch case - worth a one-line nod at review
  in case donor coverage was actually intended.
- `config_path` is set to the pre-existing `"input"` bucket (shared with
  `UnidentifiableSource`/`UnsupportedSource`), since there is no profile
  field this diagnostic is about at all - it is purely a runtime-vs-file
  fact. Reasonable within the existing convention, but noting it since no
  directly analogous *batch-wide, no-profile-field* precedent existed to
  copy verbatim (`DuplicateIdentifier` uses `"input.pattern"`, which does
  name a real profile field).

## T16.5 review: specification and documentation (2026-07-12)

Spec lockstep and stale doc fixes per the T16.5 review:

1. **§5.2 diagnostic catalog:** added `SchemaDrift` row (warning severity, batch-level once-per-batch notice) between `UnknownPropertySkew` and `SuggestionsCapped`, mirroring existing row format: code, severity, condition, params (`found_version`/`pinned`), spec cross-reference (9.2).

2. **§9 runtime prose:** added 1 sentence describing `SchemaDrift` batch-level check alongside the per-raw:-property `UnknownPropertySkew` path, clarifying that planning emits `SchemaDrift` once per batch when any source file's `identification_format_version` exceeds pinned, alerting the user of property-model gaps and opt-in via `raw:` prefix.

3. **Doc comment** (`identify.rs`, `Identification::format_version`): updated to name `SchemaDrift` batch diagnostic as the second consumer of `format_version`, alongside `UnknownPropertySkew`'s per-property use (was: "only consumer is UnknownPropertySkew's found_version").

Verification: `cargo doc --workspace --no-deps` (Rust docs, `-D warnings`) and `cargo test -p muxsmith-core --lib` (116 tests) both pass. Commit `ce4fae1`, branch `plan55-t165`, unsigned.
