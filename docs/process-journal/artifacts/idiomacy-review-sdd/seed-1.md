# Seed [T2-m1] - skip-marker shared const

**Verdict: CONFIRMED** (still applies at HEAD; grown since the original finding)

## Current state on disk

The live-test skip marker `mkvmerge not found; skipping` is a bare string literal at **21 call sites across 8 files in 3 compilation units**, plus the CI enforcement grep. No shared constant exists anywhere (both `tests/support/mod.rs` files checked: nothing).

| Location | Occurrences |
|---|---|
| `crates/muxsmith-core/tests/command_integration.rs` (204, 290, 461) | 3 |
| `crates/muxsmith-core/tests/identify_live.rs` (36) | 1 |
| `crates/muxsmith-core/tests/executor_live.rs` (19) | 1 |
| `crates/muxsmith-core/tests/mkvmerge_runtime.rs` (112, 206) | 2 |
| `crates/muxsmith-cli/tests/run_live.rs` (90, 173) | 2 |
| `crates/muxsmith-cli/tests/dry_run_cli.rs` (22, 85, 128, 215, 337, 704) | 6 |
| `crates/muxsmith-cli/tests/run_cli.rs` (132, 284, 331) | 3 |
| `src-tauri/src/lib.rs` (645, 704, 790) | 3 |
| `.github/workflows/ci.yml` (105, `grep -c 'mkvmerge not found; skipping'`) | 1 |

The original finding counted 19 call sites; two more were added since (dry_run_cli conversions), which demonstrates the failure mode: every new gated test re-types the literal, and one reworded copy silently escapes the CI "no silent skips" gate (false negative, the exact defect the T2 gate exists to prevent).

## Replacement

The original defer note said "shared const in tests/support", but `tests/support` is per-crate and the sites span three crates. Both `muxsmith-cli` and `src-tauri` already depend on `muxsmith-core`, so the const belongs there:

1. In `crates/muxsmith-core/src/lib.rs` (or a `#[doc(hidden)] pub mod testkit`):
   ```rust
   /// Printed by gated live tests when mkvmerge is absent.
   /// CI greps for this exact string (.github/workflows/ci.yml, "Assert no gated tests silently skip").
   #[doc(hidden)]
   pub const MKVMERGE_SKIP_MARKER: &str = "mkvmerge not found; skipping";
   ```
2. Replace all 21 `eprintln!("mkvmerge not found; skipping");` sites with `eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);` (core tests import from their own crate).
3. `ci.yml` line ~105: the grep literal must stay a literal in YAML; add a comment pointing at `MKVMERGE_SKIP_MARKER` so a reword updates both ends (this is the remedy the whole-branch verdict specified).

## Estimates

- lines_cut: 0 (net roughly +5; this is contract hardening / literal dedup, not size reduction)
- deps_cut: 0
- Tag: `dup` (duplicated cross-file string contract)

## Provenance

- `docs/process-journal/artifacts/plan-5.5-sdd/progress.md:52` (T2-m1 defer)
- `docs/process-journal/artifacts/plan-5.5-sdd/whole-branch-verdict.md:46,64` (defer stands, remedy: shared const + ci.yml comment)
