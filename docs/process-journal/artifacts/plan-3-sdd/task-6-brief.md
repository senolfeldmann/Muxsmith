### Task 6: resolve title and tags

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `profile.title: TitleCfg`, `profile.tags: TagsCfg`, `template::{Template, Ctx}`, `primary.identifier.to_ctx()`, `KeepDrop`.
- Produces: populated `Plan.title` and `Plan.tags`.

- [ ] **Step 1: Write the failing tests.** `title: clear` -> `TitleAction::Clear`; `title: { template: "Show S{season}" }` on a primary with `season=03` -> `TitleAction::Set("Show S03")` (raw capture; filters as spec 4.7); `title: keep` -> `TitleAction::Keep`. `tags: { global: drop, track: keep }` -> `TagFlags { global_keep: false, track_keep: true }`.

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution title` -> FAIL.
- [ ] **Step 3: Implement.** Add a helper `resolve_title(profile, primary, diags) -> TitleAction`:
  - `TitleCfg::Keyword(k)` where `k == "keep"` -> `Keep`; `k == "clear"` -> `Clear`. (Validate.rs already rejects other keywords, so no diagnostic needed here; an unexpected keyword defaults to `Keep`.)
  - `TitleCfg::Template(block)` -> parse via `Template::parse`, render with `primary.identifier.to_ctx()` via `render_literal`, return `Set(rendered)`. A template that fails to parse cannot occur post-validate; on the off chance, fall back to `Keep` (no panic). Title has no path-separator / empty-name invariants (unlike filenames): an empty rendered title is a legitimate empty title.
  Add `resolve_tags(profile) -> TagFlags`: `TagFlags { global_keep: profile.tags.global == KeepDrop::Keep, track_keep: profile.tags.track == KeepDrop::Keep }`. Wire both into the `Plan { ... }` construction.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve title and tags`.

---

