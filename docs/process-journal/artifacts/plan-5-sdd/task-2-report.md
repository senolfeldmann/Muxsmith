# Task 2 report: Hoist report JSON assembly into core (`report::json`)

## Status

DONE

## What was implemented

Turned `crates/muxsmith-core/src/report.rs` into `crates/muxsmith-core/src/report/mod.rs` (`git mv`, content unchanged except one new `pub mod json;` line) and added `crates/muxsmith-core/src/report/json.rs` with the three functions lifted from the CLI, renamed per the brief's interface:

- `batch_document(config_diags, batch, renderer)` (was `dry_run::batch_json`)
- `config_only_document(config_diags, mkvmerge_found, renderer)` (was `dry_run::config_only_json`)
- `run_document(base, outcomes, outputs)` (was `run::run_json_document`)
- private `rendered_diags` helper, moved verbatim

Assembly logic (field order, structure, values) is byte-for-byte the same as the CLI originals; only the module and the renderer type changed.

### The one design decision beyond a literal 1:1 move: `DiagnosticRenderer` trait

The CLI's `Renderer` (Fluent-based, `crates/muxsmith-cli/src/i18n.rs`) is CLI-only: it depends on `fluent-bundle`, which `muxsmith-core`'s `Cargo.toml` does not and must not carry (core's own doc comment: "This crate emits no user-facing prose... for the CLI and GUI renderers to turn into localized text"). `batch_document`/`config_only_document` still need to call *something* to fill each diagnostic's `"rendered"` field (existing, unchanged behavior), so I introduced

```rust
pub trait DiagnosticRenderer {
    fn diagnostic(&self, diagnostic: &Diagnostic) -> String;
}
```

in `report::json`, and the two functions take `renderer: &dyn DiagnosticRenderer` instead of `&Renderer`. The CLI's `Renderer` gets one small `impl DiagnosticRenderer for Renderer` in `i18n.rs` that delegates to its existing inherent `diagnostic` method. This is the standard dependency-inversion shape for exactly this situation (core defines the interface, the presentation layer supplies the implementation) and is what makes the eventual GUI reuse (spec 7, T7/T8) possible without core ever depending on a UI-side i18n stack. `run_document` needed no such change: it only re-serializes already-built `JobOutcome`/`serde_json::Value` data, no rendering.

I flag this because the brief's "signatures lifted 1:1" could be read as literally reusing the CLI `Renderer` type, which is structurally impossible (would make `muxsmith-core` depend on `muxsmith-cli`, backwards from the actual crate graph, or make core depend on `fluent-bundle`, contradicting core's no-prose contract). The trait keeps "same inputs, same output, byte-identical documents" true while resolving that constraint the way the crate's own architecture already implies.

### CLI call sites

`dry_run.rs` and `run.rs` now import and call `muxsmith_core::report::json::{batch_document, config_only_document, run_document}` directly (no more `pub(crate)` wrapper functions, no more `dry_run::batch_json`/`dry_run::config_only_json` cross-file reuse inside the CLI crate). `run.rs`'s `use crate::commands::{diag_exit_code, dry_run, print_batch_human}` lost the now-unneeded `dry_run` import; `dry_run.rs`'s `use muxsmith_core::planner::{Batch, ...}` lost the now-unneeded `Batch` import (it was only used in the removed function signatures).

### Rustdoc note on the queue-vs-source-list index

`run_document`'s doc comment states explicitly:

> `jobs[].index` indexes the QUEUE (the job spec slice `run_queue` was given: only the files that planned cleanly enough to mux), not the source-file list `batch.files` enumerates. A file skipped because one of its diagnostics is error-severity has no queue entry, so it has no `jobs[].index` at all; do not treat this index as a `batch.files` offset.

## Tests

- New `crates/muxsmith-core/tests/report_json.rs`, direct core coverage:
  - `run_document_jobs_carry_index_output_state_and_summary_carries_all_four_counts` — the brief's Step 2 field-presence test, written first against the not-yet-existing `report::json::run_document` (confirmed it failed to compile, i.e. red, before `json.rs` existed).
  - `run_document_adds_indexed_jobs_and_a_zeroed_summary_when_empty` and `run_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary` — relocated verbatim (same assertions, same fixture data) from `muxsmith-cli/src/commands/run.rs`'s `#[cfg(test)] mod tests`, since the function they exercised moved out of that file. This is a relocation of existing coverage, not new TDD.
- Existing `crates/muxsmith-cli/tests/dry_run_cli.rs` (9 tests) and `crates/muxsmith-cli/tests/run_cli.rs` (10 tests) are unmodified and are the byte-identical-output proof: they invoke the CLI binary end-to-end and parse its `--json` output, so any drift in the hoisted assembly would show there. Both pass unchanged.

## Gate (foreground, all four)

```
cargo test --workspace       -> all suites pass (muxsmith-core: 95 unit + 9 integration files incl. the new report_json.rs 3/3;
                                 muxsmith-cli: 19 unit + dry_run_cli 9/9, run_cli 10/10, catalog_completeness, cli_schema, cli_validate, run_live)
cargo fmt --all --check      -> clean, no output
cargo clippy --workspace --all-targets -- -D warnings -> clean, no warnings
cargo deny check             -> advisories ok, bans ok, licenses ok, sources ok
```

Re-ran the full gate again after the commit (clean working tree) to confirm nothing was left uncommitted or broken.

## Files changed

- `crates/muxsmith-core/src/report.rs` -> `crates/muxsmith-core/src/report/mod.rs` (rename, `+2` lines: `pub mod json;`)
- `crates/muxsmith-core/src/report/json.rs` (new): `DiagnosticRenderer` trait, `batch_document`, `config_only_document`, `run_document`, `rendered_diags`
- `crates/muxsmith-core/tests/report_json.rs` (new): direct core tests
- `crates/muxsmith-cli/src/commands/dry_run.rs`: functions removed, call sites delegate to core, unused `Batch`/`Diagnostic` imports dropped
- `crates/muxsmith-cli/src/commands/run.rs`: `run_json_document` and its two unit tests removed (tests relocated), call sites delegate to core, unused `dry_run` import dropped
- `crates/muxsmith-cli/src/i18n.rs`: `impl DiagnosticRenderer for Renderer` added

## Self-review

- **Completeness**: all three functions moved, all call sites updated, both `dry_run.rs` and `run.rs` touched exactly where required, `run.rs` footprint kept to the JSON-document lines plus the necessary import/test cleanup (no unrelated changes, consistent with "another parallel task adds a small match arm elsewhere in that file").
- **YAGNI**: did not add a `batch_document`/`config_only_document` direct-in-core test beyond what the brief asked for (Step 2 names only `run_document` field presence); those two are already proven byte-identical via the CLI integration tests, which is the brief's stated harness. Did not generalize the trait beyond what's needed (no extra methods, no blanket impls).
- **Test rigor**: the new field-presence test was written and confirmed failing (red) before `json.rs` existed, per TDD discipline for the one piece of genuinely new-to-core coverage; the hoist itself relies on the pre-existing CLI integration tests as the brief specifies.
- **Pristine test output**: `cargo test --workspace` output has zero warnings, zero dead-code notices (an initial unused `StubRenderer` struct in the test file was caught and removed before it could trip clippy).
- **Doc comments**: every new public item in `report/json.rs` is documented (`DiagnosticRenderer`, `batch_document`, `config_only_document`, `run_document`); `cargo doc -p muxsmith-core --no-deps` builds without a `missing_docs` violation (the one warning it does emit, a private intra-doc link in `executor/queue.rs`, predates this change and is outside this task's files).
- **ASCII punctuation**: grepped all changed/new files for em/en dashes, curly quotes, ellipsis; none found.

## Concerns

None outstanding. The `DiagnosticRenderer` trait is the one non-mechanical decision in this task; documented above for visibility, but I'm confident it's the correct (and essentially only workable) shape given the existing crate dependency direction and core's no-prose contract.
