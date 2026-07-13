# Seed 9 — [T3-m1] redundant fn-level cfg(unix) attribute

**Verdict: CONFIRMED** (at HEAD 2f17880)

## Finding

- **File:** `crates/muxsmith-core/tests/executor_no_hang_live.rs`
- **Line:** 30
- **Tag:** dup

The fn-level `#[cfg(unix)]` on the file's single test function duplicates the
file-level inner attribute `#![cfg(unix)]` at line 20. The file-level gate
(added by commit 3353028, "test(fix): cfg-gate executor_no_hang_live at file
level for non-unix (T3 review)") already excludes the entire integration-test
file from non-unix builds, so the fn-level gate can never have an effect.

```rust
20  #![cfg(unix)]
...
29  #[test]
30  #[cfg(unix)]        // <- redundant; file already gated at line 20
31  fn run_job_survives_a_non_utf8_line_and_a_pipe_filling_tail_without_hanging() {
```

## Replacement

Delete line 30 (`    #[cfg(unix)]` — actually unindented, `#[cfg(unix)]`),
keeping `#[test]` on line 29 directly above the fn.

- lines_cut: 1
- deps_cut: 0

## Provenance / status trail

- Origin: Plan 5.5 T3 review minor, recorded in
  `docs/process-journal/artifacts/plan-5.5-sdd/progress.md:66` ("remove at
  next touch").
- Whole-branch verdict (`whole-branch-verdict.md:70`): DEFER, "next touch of
  executor_no_hang_live.rs; cosmetic, file-level gate governs".
- The file has not been touched since 3353028; the deferral was never
  consumed, so the redundancy persists at HEAD.
