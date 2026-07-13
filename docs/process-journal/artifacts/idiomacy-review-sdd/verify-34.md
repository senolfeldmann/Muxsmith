# Verify-34: `all_diags` single-caller impl-Iterator layer (yagni, F3)

**Finding:** `crates/muxsmith-cli/src/commands/mod.rs:32` - `all_diags` is a `pub(crate)` impl-Iterator helper with exactly one caller (`diag_exit_code` directly below); inline the chain and move the ordering rationale to `diag_exit_code`'s doc comment.

**Verdict: CONFIRMED**

## Checks

**(a) Code says what the finding claims - yes.**
- `all_diags` is defined at mod.rs:32-40 (`pub(crate) fn all_diags<'a>(...) -> impl Iterator<Item = &'a Diagnostic>`) and called exactly once, at mod.rs:47 inside `diag_exit_code`.
- Workspace-wide grep: `dry_run.rs` and `run.rs` import and call only `diag_exit_code` (dry_run.rs:130; run.rs:191,301). The `all_diags` hits in `tests/dry_run_cli.rs:184ff` are a local `Vec` variable of the same name, not this function. Being `pub(crate)`, it cannot have callers outside muxsmith-cli.
- "dry-run and run share it only through that fold" is accurate: the sharing the doc comment cites (spec 5.5) happens entirely via `diag_exit_code`, which the replacement keeps shared.

**(b) Replacement is current idiom - yes, trivially.**
The replacement is character-for-character the existing function body (`iter().chain(...).chain(... flat_map ...)`) moved into its sole call site; it compiles at HEAD on the pinned toolchain today, so no external idiom check applies. The `<'a>` lifetime plumbing exists only to unify the two borrows for the RPIT return type - machinery whose sole justification would be multiple callers.

**Strengthening observation:** `all_diags`'s doc comment documents an ordering guarantee ("config-time, then batch-level, then per-file, in that order") that its only consumer ignores - `diag_exit_code` folds via `.map(|d| d.severity).max()`, which is order-independent. The report surfaces that do care about ordering (dry-run JSON/human) build from the `Batch` struct directly and never touch `all_diags`. The abstraction carries a contract nobody consumes.

**(c) Duplication difference - n/a**, no duplication claimed.

**(d) Concrete construct + replacement named - yes.** Construct: the `all_diags` helper incl. generic signature and lifetimes. Replacement: inline the chain into `diag_exit_code`, ordering rationale onto its doc comment. `lines_cut: 7` is plausible (doc comment + signature + body minus the inlined chain).

## Decision guard

- `grep all_diags|diag_exit_code` over `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md`: zero hits. No memo names these functions.
- ROADMAP "layer with one caller" (L170-171) is the idiomacy review's own yagni taxonomy definition, not a tracked entry for this construct.
- Cosmetic-cleanup group K (ROADMAP L260-267) lists other sites (load.rs, model.rs, planner.rs, command_integration.rs); `commands/mod.rs` is not among them.
- D15 (plan-4 memo) mandates the worst-of exit fold *behavior*; it says nothing about code structure. The replacement preserves the behavior and the shared `diag_exit_code` exactly.

No conflict, not tracked.
