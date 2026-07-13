# Verify-5: dead `#[allow(clippy::too_many_arguments)]` suppressions in planner.rs

**Finding:** planner.rs carries three `#[allow(clippy::too_many_arguments)]`; two are dead
(`suggest` at line 1312 with 6 params, `partition_for_overlap` at line 1565 with 7 params)
because the lint fires only at 8+ params; only `partition_for_rule` (9 params) needs its allow.
Replacement: delete the attributes at lines 1312 and 1565.

**Verdict: CONFIRMED**

## Evidence

### (a) Cited code says what the finding claims

Read at HEAD (`crates/muxsmith-core/src/planner.rs`):

- Line 1312: `#[allow(clippy::too_many_arguments)]` on `fn suggest(profile, run, primaries, id, lang, baseline)` — **6 params**.
- Line 1565: same attribute on `fn partition_for_overlap(profile, run, primaries, id, lang, conflict, tagged)` — **7 params**.
- Line 1481: same attribute on `fn partition_for_rule(...)` — **9 params** (lines 1482-1491), correctly kept by the finding.

Neither `suggest` (body 1313-1480) nor `partition_for_overlap` (body ends before line 1612)
contains a nested `fn` item the attribute could be covering; their closures take 1-2 params
(and `too_many_arguments` targets fn items/methods, not closures).

### (b) Threshold claim verified empirically on the pinned toolchain, not from memory

- `rust-toolchain.toml` pins `1.96.1`; `cargo clippy --version` in the repo reports `clippy 0.1.96`.
- No `clippy.toml` / `.clippy.toml` anywhere in the repo (`find` over the tree), no `[lints]`
  tables in any `Cargo.toml`, no crate-level lint attributes beyond `#![deny(missing_docs)]`.
- CI gate (`.github/workflows/ci.yml:79`) is plain `cargo clippy --workspace --all-targets -- -D warnings`
  — default lint set, default threshold.
- **Empirical probe** (scratchpad crate, edition 2024, `RUSTUP_TOOLCHAIN=1.96.1`, signatures
  mimicking the real ones incl. `&mut dyn Trait`): 6-param and 7-param fns produce **no**
  warning; 8-param and 9-param fns warn `this function has too many arguments (8/7)` / `(9/7)`.
  So the default threshold is 7 and the lint fires at 8+, exactly as the finding states.
- Deleting a never-firing `#[allow]` is safe under `-D warnings`: unlike `#[expect]`, an
  unused `allow` produces no warning, so nothing new fires. Removal also restores the guard —
  if a future edit pushes either fn to 8 params, the lint fires and forces a deliberate
  decision instead of being pre-suppressed. That is the idiomatic state.

### (c) Duplication claim

Not applicable (no duplication asserted).

### (d) yagni concreteness

Concrete constructs (two named attribute lines) and concrete replacement (delete both) are named.

## Decision guard

- Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for
  `too_many_arguments`, `allow(clippy`, lint suppression: **no hits**. No D-memo, IDEAS, or
  ROADMAP entry (incl. group K / test-hygiene / deliberate-restraint) covers these attributes.
- Process-journal artifacts (not decision sources) mention the topic twice:
  - `docs/process-journal/artifacts/plan-2-fixes-sdd/FINAL-review.md:223` calls the `suggest`
    allow "pre-existing and justified" — a reviewer's factual claim, and factually wrong: git
    history shows the attribute was added at `suggest`'s birth (`007c3ac`) already with 6
    params ("suggestion engine is added in the next commit"), i.e. speculative and dead from
    day one. `partition_for_overlap` likewise was born (`f68e5d7`) with 7 params + the allow.
  - `plan-4-sdd/task-8-report.md:212` justifies the *fourth* allow in the repo,
    `muxsmith-cli/src/commands/run.rs:49` — that one is live (verified: `run` has 8 params)
    and is outside this finding's scope. The finding correctly leaves it alone.

Neither artifact is a recorded decision; no DECISION_CONFLICT, no TRACKED entry.

## Conclusion

Every claim in the finding checks out against the code at HEAD and against an empirical run of
the pinned clippy. Deleting the attributes at planner.rs:1312 and planner.rs:1565 removes two
dead suppressions and re-arms the lint for future signature growth. **CONFIRMED.**
