### Task 6: Zero-track plan warning (#6 - warning only, decided)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (after per-file plan resolution, where the plan's assignment count is known)
- Modify: `report/mod.rs` (DiagCode `EmptyPlan`, warning), `locales/en/diagnostics.ftl`
- Test: planner tests + batch-report test

**Interfaces:** warning appears in per-file diagnostics AND is visible in the batch report/summary counts (walkthrough #6: "batch-report visibility").

- [ ] Step 1: Failing test: all-optional profile, source file matching nothing; assert plan still renders (unchanged), plus one `EmptyPlan` warning attached to that file.
- [ ] Step 2: Implement (fires when a rendered plan has zero track assignments; keep/attachment-only plans: decide by reading D20 semantics - a keep-mode plan always carries the primary's tracks, so the warning naturally cannot fire there; note this in the test).
- [ ] Step 3: SI-3 note for the memo: mkvmerge itself exits 0 writing the empty file (verified live in Plan 3); mkvtoolnix-gui behavior at zero selected tracks - check once in the GUI source, record match/divergence one-liner.
- [ ] Step 4: Full gate; commit `feat(planner): warn when a plan resolves to zero tracks (#6)`.

