# Task 9 report: skip-marker shared const (T2-m1)

Parent commit: `82e58d8` (merge: plan-5.6 stream D). Executed serially on
`master`, no worktree.

## Re-grep result (before implementation)

`grep -rn "mkvmerge not found; skipping" --include="*.rs" .` -> **21 sites,
8 files, 3 crates** (matches the brief's "grew 19 -> 21" note; nothing
further grew since):

| File | Sites |
|---|---|
| `crates/muxsmith-cli/tests/dry_run_cli.rs` | 6 |
| `crates/muxsmith-core/tests/command_integration.rs` | 3 |
| `crates/muxsmith-cli/tests/run_cli.rs` | 3 |
| `src-tauri/src/lib.rs` | 3 |
| `crates/muxsmith-cli/tests/run_live.rs` | 2 |
| `crates/muxsmith-core/tests/mkvmerge_runtime.rs` | 2 |
| `crates/muxsmith-core/tests/executor_live.rs` | 1 |
| `crates/muxsmith-core/tests/identify_live.rs` | 1 |

Crates: `muxsmith-cli` (14), `muxsmith-core` (7 across its own `tests/`
integration files), `muxsmith-gui`/`src-tauri` (3, inline `#[cfg(test)]`
module).

## Implementation

- **`crates/muxsmith-core/src/lib.rs`**: added
  `#[doc(hidden)] pub const MKVMERGE_SKIP_MARKER: &str = "mkvmerge not found; skipping";`
  at the crate root, after the `pub mod` list, with a `//` comment (not
  `///`) explaining the doc-hidden-pub cross-crate-sharing rationale. This
  follows the existing house pattern already used for cross-file test
  sharing inside `muxsmith-core` itself (`planner.rs`'s `with_rule_match`,
  `resolves_without_regression`, and `executor/spawn.rs:257`): a `//`
  comment plus `#[doc(hidden)] pub`, which `missing_docs` does not flag
  (confirmed: `cargo doc --no-deps` with `-D warnings` is clean). This is
  the first case where that pattern crosses a crate boundary rather than
  just a file boundary within one crate - `muxsmith-cli` and
  `muxsmith-gui` both already depend on `muxsmith-core`, so no new
  dependency edge was added; a per-crate `tests/support` module (the
  existing `testing-support-helpers` convention,
  `docs/conventions.yaml`) cannot reach across crates, which is why the
  const lives in the one crate all three already share.
- All 21 `eprintln!("mkvmerge not found; skipping");` sites replaced
  mechanically (leading whitespace preserved, `cargo fmt --check` clean
  afterward) with `eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);`.
  Every site already treats `muxsmith_core` as an external crate (the
  `muxsmith-core` integration tests under `tests/` do too, by Cargo's
  integration-test convention; `src-tauri/src/lib.rs` already has
  `use muxsmith_core::...` imports), so no new `use` lines were needed.
- **`.github/workflows/ci.yml:105`** (now :106 after the insertion): added
  a one-line comment directly above the grep,
  `# This literal must match muxsmith_core::MKVMERGE_SKIP_MARKER (crates/muxsmith-core/src/lib.rs) byte-for-byte.`
  The grep literal itself is untouched (YAML can't reference a Rust
  const; the comment is the only feasible tie between the two).

Zero behavior change: the emitted string is still exactly
`mkvmerge not found; skipping`, confirmed by running the gate live.

## Self-review

`grep -rn "mkvmerge not found; skipping\|MKVMERGE_SKIP_MARKER" --include="*.rs" --include="*.yml" .`
returns exactly: the one const definition, the one ci.yml grep literal
plus its new comment, and 21 call sites all reading
`eprintln!("{}", muxsmith_core::MKVMERGE_SKIP_MARKER);` - no site missed,
no stray bare-string duplicate reintroduced.

## Nine-part gate (all foreground, all green)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | clean, no diff |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| 3 | `cargo test --workspace -- --nocapture --test-threads=1` | **all green**: every `test result: ok` block, 0 `FAILED`, live-mkvmerge tests exercised for real (mkvmerge v100.0 present on PATH via linuxbrew) - skip-marker count in the captured output is **0** |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean (confirms `#[doc(hidden)]` const needs no rustdoc comment under `missing_docs`) |
| 5 | `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `pnpm lint` | clean |
| 7 | `pnpm build` | clean, `vue-tsc` + `vite build` succeed |
| 8 | `pnpm check:i18n` | ok (12 pre-existing unused-key warnings, unrelated to this change) |
| 9 | `pnpm test:e2e` | 7/7 Playwright specs pass |

## Files changed

- `crates/muxsmith-core/src/lib.rs` (new const)
- `crates/muxsmith-cli/tests/dry_run_cli.rs` (6 sites)
- `crates/muxsmith-cli/tests/run_cli.rs` (3 sites)
- `crates/muxsmith-cli/tests/run_live.rs` (2 sites)
- `crates/muxsmith-core/tests/command_integration.rs` (3 sites)
- `crates/muxsmith-core/tests/executor_live.rs` (1 site)
- `crates/muxsmith-core/tests/identify_live.rs` (1 site)
- `crates/muxsmith-core/tests/mkvmerge_runtime.rs` (2 sites)
- `src-tauri/src/lib.rs` (3 sites)
- `.github/workflows/ci.yml` (comment only, grep literal unchanged)

## New pattern / deviation to surface

Not a deviation - this is the plan's specified fix - but worth recording
as a house-pattern instance for `docs/conventions.yaml`'s
`testing-support-helpers` entry (or a sibling entry): cross-crate
test-only sharing, where per-crate `tests/support` cannot reach, resolves
by putting a `#[doc(hidden)] pub` item in the one crate every consumer
already depends on, same rationale as the existing single-crate
`with_rule_match` precedent in `planner.rs`. Leaving the actual
conventions.yaml edit to the controller/reviewer per the doctrine's
promotion rules, since this task brief was a decided plan item
(`source: controller-adr`-equivalent), not an emergent pattern I'm
promoting unilaterally.

## Commit

`refactor: single source for the mkvmerge skip marker (T2-m1)`, unsigned
(`git -c commit.gpgsign=false commit`), explicit file staging (no
`git add -A`), not pushed.
