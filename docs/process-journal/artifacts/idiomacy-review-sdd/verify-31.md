# Verify-31: redundant per-test `#[cfg(unix)]` in executor_no_hang_live.rs

**Verdict: TRACKED** (ref: ROADMAP "Whole-codebase idiomacy review" named input **T3-m1**, "redundant fn-level cfg(unix)")

## Finding under test

`crates/muxsmith-core/tests/executor_no_hang_live.rs:31` (attribute on line 30): `#[cfg(unix)]` on the file's only test duplicates the file-level inner attribute `#![cfg(unix)]` (line 20). Replacement: delete the per-test attribute. Tag: idiom, slice F2b.

## Technical verification (the finding is substantively correct)

- **(a) Cited code says what the finding claims:** verified at HEAD `2f17880`. Line 20 carries `#![cfg(unix)]` as a crate-root inner attribute of the integration-test binary; line 30 repeats `#[cfg(unix)]` on `run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging`, the file's only test.
- **(b) Replacement is current idiom:** confirmed against the current Rust Reference (doc.rust-lang.org/stable/reference/conditional-compilation.html, via context7): a crate-level `#![cfg]` with a false predicate removes the crate's contents following the attribute; with a true predicate the attribute is removed and the contents compile. Therefore the per-test attribute is unconditionally dead: it is only ever evaluated when the file's contents compile at all, i.e. when `unix` is already true. Deleting it is semantics-preserving on every target.
- **(c) No load-bearing difference between the two sites:** same predicate, same file, same gate. The per-test form is needed only where no file-level gate covers the item, e.g. the unit test `spawn.rs:365` (`live_killer_then_wait_returns_none`) inside the main crate, which this file's doc comment names as its pattern donor; that explains the duplication's origin, not a function for it.
- **(d)** tag is `idiom`, not `yagni`; concrete construct and replacement are named. Not applicable.

## Decision guard (the reason for the verdict)

`docs/ROADMAP.md`, entry **"Whole-codebase idiomacy review"** (v1.x block), NAMED INPUTS from the Plan 5.5 roll-up funnel (2026-07-12, whole-branch triage):

> "... attachments/chapters-only EmptyPlan test (T6-m1); **redundant fn-level cfg(unix) (T3-m1)**; overlap_conflicts re-parses claimants ..."

Plan 5.5 Task 3 produced exactly this file/test (`docs/process-journal/artifacts/plan-5.5-sdd/task-3-report.md`), so T3-m1 is this construct. The finding is a rediscovery of an item already recorded and deliberately funneled into this very pass. No contradiction with any design memo (D1-D35), IDEAS.md, or a deliberate-restraint entry; nothing in the specs mentions the file or construct.

**Note for triage:** T3-m1 is tracked *as input to this pass*, not as work parked elsewhere. TRACKED here means "known and expected, attribute finding to T3-m1"; the fix itself (delete line 30's attribute) remains valid and is this pass's to apply.
