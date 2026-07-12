### Task 7: SourceOverwrite completeness + S11 guard comment (#7)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs:893-899` (`detect_source_overwrites`)
- Test: planner tests

**Interfaces:** none new; diagnostic `SourceOverwrite` now also protects donors of render-failed files.

- [ ] Step 1: Failing test (the three-way constellation): file A references donor D and A's filename template fails to render; file B's output path collides with D's path. Assert `SourceOverwrite` fires. Current code: passes silently (protection set built only from `plan == Some`).
- [ ] Step 2: Implement: gather protected source paths from ALL files' resolved sources (primaries + donors), independent of plan render success - sources are known before rendering; only outputs need a rendered plan.
- [ ] Step 3: Add the S11 guard comment at the function: the ambiguous-external branch deliberately contributes no donor paths WHILE it is fatal; revisit if it ever becomes non-fatal (F5 report).
- [ ] Step 4: Full gate; commit `fix(planner): protect donors of render-failed files from output collisions (#7)`.

