# Idiomacy review - slice F3: crates/muxsmith-cli/ (src + tests)

Scope: all 16 `.rs` files under `crates/muxsmith-cli/` (4383 lines), read completely.
Dimensions hunted: idiom, stdlib, yagni, native. Excluded per brief: correctness/security/performance, behavior changes, line-golf, cross-file duplication, dependency health, and the recorded non-findings (D26 env seam, fake-mkvmerge copies, pins, B11).

## Files covered

- src: `cli.rs`, `main.rs`, `lib.rs`, `i18n.rs`, `commands/{mod,validate,dry_run,identify,run}.rs`
- tests: `catalog_completeness.rs`, `cli_schema.rs`, `cli_validate.rs`, `dry_run_cli.rs`, `run_cli.rs`, `run_live.rs`, `support/mod.rs`

## Findings

### F3-1 (dup) - `Renderer::msg` re-implements `msg_with_counts` minus the counts loop

`src/i18n.rs:45`. `msg` builds a `FluentArgs` from the string pairs and calls `render`; `msg_with_counts` does exactly the same plus the numeric loop. `msg(id, args)` is semantically identical to `msg_with_counts(id, args, &[])`.

Replacement:

```rust
pub fn msg(&self, id: &str, args: &[(&str, &str)]) -> String {
    self.msg_with_counts(id, args, &[])
}
```

lines_cut: 4, deps_cut: 0.

### F3-2 (yagni) - `catalog_completeness.rs` builds `FluentArgs` fixtures only to convert them back to string pairs

`tests/catalog_completeness.rs:453` (`string_pairs`), consuming `fixture_args` (line 48) and `allowlisted_cli_key_args` (line 267). Every fixture value in the file is a string literal set via `args.set(k, "...")`; `render_and_find_leaks` then round-trips each `FluentArgs` through `string_pairs` (which carries a panic arm for the non-string case that structurally cannot occur) back into the `&[(&str, &str)]` shape `Renderer::msg` actually takes. The `FluentArgs` intermediate representation is unused generality.

Replacement: have both fixture functions return `Vec<(&'static str, &'static str)>` (tuple pushes or vec literals per arm), change `render_and_find_leaks` to take `(&str, Vec<(&str, &str)>)` and call `renderer.msg(id, &pairs)` directly, delete `string_pairs` and the now-unneeded `FluentValue` import. (`FluentArgs`/`FluentResource` stay imported: the ftl-parsing test still needs `fluent_syntax`/`FluentResource`.)

lines_cut: 15, deps_cut: 0.

### F3-3 (yagni) - `all_diags` is a layer with exactly one caller

`src/commands/mod.rs:32`. `all_diags` (a generic `impl Iterator<Item = &'a Diagnostic>` with explicit lifetime plumbing and its own doc block) is `pub(crate)` but called only by `diag_exit_code` directly below it; `dry-run` and `run` share it solely through that fold. The chain fits inline:

```rust
pub(crate) fn diag_exit_code(config_diags: &[Diagnostic], batch: &Batch) -> i32 {
    let worst = config_diags
        .iter()
        .chain(&batch.batch_diagnostics)
        .chain(batch.files.iter().flat_map(|f| &f.diagnostics))
        .map(|d| d.severity)
        .max();
    match worst { ... }
}
```

The ordering rationale from `all_diags`'s doc moves onto `diag_exit_code`. If a second caller ever materializes, re-extracting is mechanical.

lines_cut: 7, deps_cut: 0.

### F3-4 (yagni) - dead `mod support;` in `cli_validate.rs`, and it is the sole reason for two `#[allow(dead_code)]`s

`tests/cli_validate.rs:3`. The file declares `mod support;` but never references `support::` - its own module doc even explains why no redaction is needed ("validate never touches mkvmerge or the queue ... never the profile file's own filesystem location"). Because each `tests/*.rs` file is its own binary, this unused inclusion is exactly what forces the `#[allow(dead_code)]` on both helpers in `tests/support/mod.rs` (lines 35, 47): in the `cli_validate` binary both are dead; in every real consumer (`dry_run_cli`, `run_cli`) both are live (`insta_settings` via `insta_settings_with_tmp`).

Replacement: delete `mod support;` from `cli_validate.rs` and remove both `#[allow(dead_code)]` attributes from `tests/support/mod.rs`.

lines_cut: 3, deps_cut: 0.

## Routed (out-of-scope incidentals, not findings)

- `src/commands/run.rs:220`: `let _ = ctrlc::set_handler(...)` swallows a handler-registration failure. If registration fails, the first SIGINT falls back to the OS default kill - no in-flight job kill, no partial-output deletion, no exit-130 - and the user gets no warning that graceful-cancel semantics are absent. Unlikely in practice (fails mainly if a handler is already registered), but the documented D16 contract silently degrades. A `eprintln!` warning on `Err` would make the degradation visible. Correctness/operability, outside this sweep's dimensions.

## Explicit non-findings checked and passed

- Rust edition-2024 idioms: let-else, let-chains (`i18n.rs` render_diagnostic_message), `std::thread::scope`, `sort_by_key(Reverse(...))`, `filter_map`, `File::set_modified` (std, no `filetime` crate) - all current and correct for the pinned 1.96.1 toolchain.
- fluent-bundle usage (`add_resource_overriding`, `set_use_isolating(false)`, `format_pattern`, numeric `FluentValue` promotion for CLDR plural selectors) matches the crate's documented API and Fluent's guidance; passing runtime-variant sub-messages (severity, message) as args into `diagnostic-line` is the accepted Fluent pattern, since message references cannot be parameterized.
- insta usage (`Settings::clone_current`, `add_filter` with `regex::escape`d tempdir literals, `bind`) is the documented filters idiom; the deliberate exact-literal-over-generic-path-regex choice is sound and documented in `support/mod.rs`.
- clap derive structure (subcommand enum, `value_enum`, doc-comment help) is standard; per-subcommand `--json`/`--locale` instead of `global = true` args was considered and not flagged - a global arg would change the CLI surface (`schema` would start accepting them), which is a behavior change and plausibly deliberate.
- `CollisionArg::policy()` as a named conversion instead of `impl From`: within Rust API-guideline latitude, not flagged.
- `empty_path_dir()` in `dry_run_cli.rs` delegates to `tempfile::tempdir()`, but its name and hand-verified doc comment do genuine documentation work for two call sites; inlining would move, not remove, content. Not flagged.
- `identify.rs` using `IdentifyCache` for a single lookup: the cache is core's composed entry point (spawn + parse + stat-key); hand-composing `identify_json` + `from_json` in the CLI would duplicate that composition. Not flagged.
- Known non-findings honored: `MUXSMITH_RUNS_ROOT` debug seam (D26), fake-mkvmerge helper copies (tracked, trigger >3), version pins.

## Verdict

4 findings (1 dup, 3 yagni), ~29 net lines removable, 0 deps. No idiom, stdlib, or native violations against the pinned toolchain. `clean: false` only by virtue of the four minor cuts above; the crate is otherwise in unusually disciplined shape for a pre-1.0 gate.
