### Task 7: resolve chapters (keep / drop / external locator)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `profile.chapters: ChaptersCfg`, `discovery::resolve_locator`, `DiagCode::{MissingExternal, AmbiguousExternal}`.
- Produces: populated `Plan.chapters`. External chapters reuse the locator machinery and its diagnostics; a chapters file is NOT identified via mkvmerge (it is XML/simple, passed straight to `--chapters`).

- [ ] **Step 1: Write the failing tests.** `chapters: drop` -> `ChapterSource::Drop`; `chapters: keep` -> `Keep`; `chapters: { external: { path: ".", extensions: [xml], match_to_source: true } }` with exactly one matching `<id>.xml` beside the primary -> `ChapterSource::External(<that path>)`; zero matches -> `MissingExternal` diagnostic at `chapters.external` and the file gets no plan; two matches -> `AmbiguousExternal`.

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution chapters` -> FAIL.
- [ ] **Step 3: Implement.** Add `resolve_chapters(profile, primary, primary_dir, diags) -> ChapterSource`:
  - `ChaptersCfg::Keyword(k)`: `"keep"` -> `Keep`, `"drop"` -> `Drop`.
  - `ChaptersCfg::External(block)`: `discovery::resolve_locator(&block.external, primary_dir, &primary.identifier)`; match `hits.len()`: `1` -> `External(path)`; `0` -> push `Diagnostic::error(MissingExternal, "chapters.external").for_file(&primary.path)`, return `Keep` (a placeholder; the error already suppresses the plan); `n` -> push `Diagnostic::error(AmbiguousExternal, "chapters.external").for_file(&primary.path).with("count", n)`, return `Keep`. Wire into `Plan`. Because an error diagnostic already forces `plan: None`, the returned placeholder is never emitted.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve chapters (keep/drop/external)`.

---

