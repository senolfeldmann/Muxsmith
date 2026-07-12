### Task 16.5: schema-drift batch notice (added 2026-07-12, Şenol decision at the D32-addendum gate)

Şenol: the general "your mkvmerge's identification schema is newer than
this build pins" notice is important - rebuild it as its OWN diagnostic,
ONCE PER BATCH (not per file, the old mis-scoping), info severity, message
includes the raw: discovery hint (newer properties matchable via raw:).

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (batch-level emission next to the other batch walks; fires when any identified file's format_version > PINNED, once), `report/mod.rs` (new DiagCode SchemaDrift, info; params found_version/pinned), `locales/en/diagnostics.ftl` + `locales/de/diagnostics.ftl` (BILINGUAL - post-T19/T21 cutoff), `catalog_completeness.rs` (fixture, same commit)
- Test: planner batch test (two newer-schema files -> exactly ONE SchemaDrift; pinned-version files -> none)

- [ ] Failing test first, implement, fixture lockstep, full gate; commit `feat(core): once-per-batch schema-drift notice (D32 addendum, Şenol)`.

