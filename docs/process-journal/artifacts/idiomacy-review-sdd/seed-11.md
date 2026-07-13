# Seed 11 [whole-branch M3] — UnknownExtension rustdoc "once per batch" vs per-entry emission

**Verdict: CONFIRMED**

- **File/line:** `crates/muxsmith-core/src/report/mod.rs:165`
- **Tag:** doc

## Current state at HEAD (2f17880)

The `UnknownExtension` rustdoc still claims:

> "Batch-wide, once per batch; skipped (not raised) when the runtime capability is unavailable."

Actual behavior, verified on disk:

- `validate_extension_list` (`crates/muxsmith-core/src/planner.rs:361-377`) pushes one `Diagnostic::warning(DiagCode::UnknownExtension, ...)` **per offending list entry**, at path `{path_prefix}[i]`.
- The comment above `validate_extension_values` (`planner.rs:306-318`) states it explicitly: "No dedup by extension value: the same unknown extension repeated across `input.extensions` and a locator (or across two locators) gets one diagnostic per occurrence, each at its own `config_path`."
- What IS once per batch is the *check* (one `validate_extension_values` run per `plan_core` call), not the *emission*. Contrast the adjacent `SchemaDrift` doc (`report/mod.rs:163`), where "Batch-wide, once per batch" is genuinely true of the emission.

`profile/model.rs:74` ("Checked once per batch against the local mkvmerge `--list-types`") is accurate as written (it describes the check) and needs no change.

## Fix

Replace the frequency clause in the `report/mod.rs:165` rustdoc:

- Old: `Batch-wide, once per batch; skipped (not raised) when the runtime capability is unavailable.`
- New: `Checked once per batch; emitted once per offending list entry at its own config path (no dedup by extension value); skipped (not raised) when the runtime capability is unavailable.`

Wording-only change: lines_cut 0, deps_cut 0. The whole-branch verdict (`docs/process-journal/artifacts/plan-5.5-sdd/whole-branch-verdict.md`, M3) classified it "doc nuance, next touch"; it was never applied.
