### Task 7.5: SourceOverwrite protection for attachment donors (added 2026-07-11, T7-review routing)

T7 closed the track-donor half of the data-loss class (#7); the T7 review
found the attachment half was never covered: the protection set is built
solely from Assignment.source, so an attachment donor
(AttachmentPlan.add_files, resolved via resolve_attachments) referenced by
a render-failed file has the same silent-overwrite exposure. Narrow in
practice (outputs are .mkv; requires an .mkv-named attachment source) but
the same class. Stream B, after T5.9.

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (extend the resolved-sources
  gathering to attachment donors, same pre-render capture point as T7)
- Test: planner tests (three-way constellation with an .mkv-named
  attachment source on a render-failed file)

- [ ] Failing test first, minimal fix, full gate; commit
  `fix(planner): protect attachment donors of render-failed files (#7 completion)`.

