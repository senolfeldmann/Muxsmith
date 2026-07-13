# Verify-15: worker_count manual clamp (queue.rs:335) — CONFIRMED

**Finding:** `worker_count` computes `jobs.max(1).min(spec_count.max(1))` — the manual-clamp pattern `clippy::manual_clamp` exists for; proposed replacement `jobs.clamp(1, spec_count.max(1))`.

**Verdict: CONFIRMED**

## Checks

### (a) Cited code matches
`crates/muxsmith-core/src/executor/queue.rs:335` reads exactly `jobs.max(1).min(spec_count.max(1))` at HEAD (2f17880). Match.

### (b) Replacement is current idiom for the pinned toolchain (1.96.1 / edition 2024)
- `Ord::clamp` is stable std API (since 1.50); the manual `x.max(min).min(max)` chain is precisely what `clippy::manual_clamp` was written to rewrite.
- The finding's lint-silence explanation is accurate per current clippy sources (verified via web, not training memory): `manual_clamp` sat in nursery after breaking code (rust#106731), then was moved to `complexity` (warn-by-default) in rust-clippy PR #12543 **restricted to const bounds with provable `max >= min`** — a lint-precision decision about clamp's panic condition, not a statement that the manual pattern is preferred for non-const bounds. Here the upper bound `spec_count.max(1)` is non-const, so the lint stays silent despite CI's `cargo clippy --workspace --all-targets -- -D warnings` (pinned 1.96.1, `rust-toolchain.toml`). "Silent here only because the upper bound is non-const" — correct.
- Panic precondition trivially holds: `clamp(1, spec_count.max(1))` has `min = 1 <= spec_count.max(1) = max` for every `usize` input, so no panic path.
- Semantic equivalence for all inputs (`x.max(a).min(b) == x.clamp(a, b)` whenever `a <= b`): jobs=0/spec=0 → 1 both; jobs=0/spec=2 → 1 both; jobs=100_000/spec=2 → 2 both; in-range passes through both.
- The referenced test `worker_count_is_capped_at_spec_count` (queue.rs:1217) covers these cases and passes unchanged; its own assertion message already uses the clamp vocabulary ("jobs is clamped to >= 1 before the cap is applied"), as does the function's rustdoc ("jobs clamped to at least 1, then capped"). Neither needs to change.
- Only occurrence of the pattern in the workspace (grep over `crates/`), so the fix is a single expression.

### (c) Duplication difference
N/A — not a duplication finding.

### (d) yagni completeness
N/A — tag is `idiom`; concrete construct and concrete replacement are both named.

## Decision guard
- `docs/superpowers/specs/*.md` (D1-D35): only "clamp" hit is 2026-07-09-plan-3 line 187 (track/chapters slot clamping in the rule model) — unrelated.
- `docs/ROADMAP.md`: queue.rs hit is T4-i1 (replacing the single eprintln) — unrelated. Cosmetic-cleanup group K (load.rs `at` param, TracksCfg placement, stale module doc, etc.) does not include worker_count. No worker_count / manual_clamp entry anywhere.
- `docs/IDEAS.md`: no hit.

No recorded decision conflicts with the finding; it is not already tracked.

## Sources
- https://github.com/rust-lang/rust-clippy/pull/12543 (manual_clamp → complexity, const-only restriction)
- https://github.com/rust-lang/rust/pull/106731 (earlier nursery move)
- https://github.com/rust-lang/rust-clippy/blob/master/clippy_lints/src/manual_clamp.rs
