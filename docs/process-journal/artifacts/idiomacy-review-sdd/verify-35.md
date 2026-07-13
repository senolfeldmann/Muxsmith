# Verify-35: dead `mod support;` in cli_validate.rs — CONFIRMED

**Finding (F3, yagni):** `mod support;` in `crates/muxsmith-cli/tests/cli_validate.rs:3` is declared but never referenced, and this dead inclusion is the sole reason both helpers in `crates/muxsmith-cli/tests/support/mod.rs` carry `#[allow(dead_code)]` (lines 35, 47).

**Verified at HEAD** `2f17880a956e05f833a3afdec2c650c176e391e5` (clean tree).

## Evidence

1. **Cited code matches.** `cli_validate.rs` (95 lines) contains exactly one occurrence of `support`: the `mod support;` declaration at line 3. No `support::` path, no `use support`, nothing macro-shaped that could reach it. The file's own doc comment (lines 13-19) states validate needs no path/version/duration redaction — consistent with the finding.
2. **Both helpers are live in every real consumer binary.** `dry_run_cli.rs:247`, `run_cli.rs:83`, `run_cli.rs:362` each call `support::insta_settings_with_tmp`, which itself calls `insta_settings` — so both functions are reachable in both remaining binaries.
3. **Replacement compiles clean on the pinned toolchain.** In a scratch copy of the repo (real repo untouched), applied exactly the proposed replacement (delete `mod support;` from cli_validate.rs; delete both `#[allow(dead_code)]` lines from support/mod.rs), then ran `cargo check --tests -p muxsmith-cli` with `RUSTFLAGS="-Dwarnings"` on the repo-pinned Rust 1.96.1: **zero warnings, zero errors**.
4. **Causality proven by counterfactual.** Re-adding only `mod support;` to cli_validate.rs (allows still removed) fails with exactly:
   - ``error: function `insta_settings` is never used``
   - ``error: function `insta_settings_with_tmp` is never used``
   - ``could not compile `muxsmith-cli` (test "cli_validate")``

   The errors originate from the `cli_validate` binary specifically — the dead inclusion is the *sole* reason for both allows, as claimed. (Cargo compiles each `tests/*.rs` as its own binary; `dead_code` is judged per binary. Verified empirically against 1.96.1 rather than from training memory.)
5. **Idiom check (b).** The replacement is pure dead-code deletion; nothing toolchain-version-sensitive beyond the per-binary dead_code mechanics verified in (3)/(4). The `#[allow(dead_code)]`-on-partially-consumed-support-module pattern (plan-4 brief) legitimately applies only while a partial consumer exists; after removal no consumer is partial, so the allows lose their justification.

## Decision guard

- `docs/superpowers/specs/*.md` (D1-D35): the only `tests/support` hit is `2026-07-09-plan-4-design-decisions.md:148` — the **muxsmith-core** `FakeIdent`/`lang()` helper dedup, a different crate and module. No decision covers cli_validate's inclusion or the CLI allows.
- `docs/ROADMAP.md`: sweep group K (cosmetic cleanup) enumerates other items (load.rs `at` param, TracksCfg placement, stale module doc in command_integration.rs, ...); the test-hygiene collection (B-minors) likewise. Neither lists this construct.
- `docs/IDEAS.md`: no hit.

Not tracked, no conflicting decision.

## Verdict

**CONFIRMED.** The finding is factually accurate, the replacement is correct and compiles warning-free under the repo's `-D warnings` gate on Rust 1.96.1, the causal claim is empirically proven, and no recorded decision or tracker entry covers it.

One execution note: deleting line 3 leaves a double blank line between the `use assert_cmd::Command;` block and `fn muxsmith()`; `cargo fmt` will collapse it.
