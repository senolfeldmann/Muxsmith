# Task 3 reviewer verdict (model: opus, 2026-07-11)

Diff: 374005a..3fbaa9e, fix 3353028 (review-374005a..3353028.diff)

## Round 1: Needs fixes
Spec compliant in the mandated shape (read_until + lossy decode, Ok(0)-only
EOF, distinguishable Err arm, extracted read_next_line, unit sequence test,
live no-hang test pinning job.rs's drain-then-wait path - verified against
job.rs:136/154, cannot pass vacuously). Named risks: post-None wait() PASS;
line framing PASS (read_line == read_until + UTF-8 check, identical BufRead
framing, trim_end identical); mkvmerge-gating deviation sound on the merits
(test never invokes mkvmerge).
CRITICAL: executor_no_hang_live.rs had unconditional module-level imports
under a #[cfg(unix)]-only test fn - Windows clippy -D warnings
(--all-targets) fails on unused imports; reviewer reproduced the mechanism.
Root insight: the cited precedent (live_killer_then_wait_returns_none)
lives in an inline #[cfg(test)] mod where use super::* keeps imports live;
a standalone unix-only integration crate must gate at FILE level.

## Fix wave
3353028: #![cfg(unix)] after the module doc. Re-review verified: placement
valid/idiomatic, spawn.rs hunks byte-identical to round 1, only the
attribute + blank line added; gated file compiles clean under -D warnings,
ungated control still errors (causal confirmation).

## Residual Minor
- fn-level #[cfg(unix)] now redundant under the file-level gate; remove at
  next touch, no fix wave.

## Final Assessment
Spec compliance ✅. Task quality: Approved.
