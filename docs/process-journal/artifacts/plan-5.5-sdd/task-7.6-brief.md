### Task 7.6: SourceOverwrite protection for chapters donors - class closure (added 2026-07-11, T7.5 finding)

The third and LAST member of the #7 class: ChapterSource::External
(planner.rs ~:866) resolves a chapters donor via the same resolve_locator
mechanism at the same pre-render point, uncovered by resolved_sources.
After this task the class is CLOSED by construction: model.rs has exactly
two Locator field sites (ExternalBlock.external, shared by track rules AND
chapters; AttachmentRule.add - verified in the T5.9 review), and all their
resolution points then feed the protection set. Stream B, after T7.5.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (wire the chapters-donor
  resolution into T7's resolved_sources chain)
- Test: planner tests (three-way constellation with a chapters donor on a
  render-failed file)

- [ ] Failing test first, minimal fix. Add a completeness comment at the
  resolved_sources gathering site enumerating the three donor kinds and
  the two Locator field sites, so a future Locator position visibly
  demands a fourth entry. Full gate; commit
  `fix(planner): protect chapters donors of render-failed files (#7 class closure)`.

