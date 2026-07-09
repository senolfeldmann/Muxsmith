### Task 8: resolve attachments (rules + unmatched + external adds)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs`; `crates/muxsmith-core/src/report.rs` (one doc-comment tweak on `MissingExternal`)
- Test: `crates/muxsmith-core/tests/planner_resolution.rs`

**Interfaces:**
- Consumes: `ident.attachments: Vec<Attachment>` (primary's, Task 1), `profile.attachments: AttachmentsCfg` (`unmatched`, `rules`), `matcher::matches` for attachments (Task 3), `discovery::resolve_locator`, `KeepDrop`, `DiagCode::MissingExternal`.
- Produces: populated `Plan.attachments: AttachmentPlan`.

**Decisions locked here:**
- **Attachment scope (design-decisions D10):** rules and `unmatched` apply to the **primary file's** attachments only; donor attachments never flow in (command emits `--no-attachments` on donor groups, Task 11).
- **`add` cardinality and zero-match (design-decisions D12):** an `add` locator attaches **all** files it matches (a `Locator` is a query that populates the attachment collection, like `select`/`drop`; not a unique slot-filler like a track/chapters donor), appended to `add_files` in resolution order and **deduplicated by path** (two rules matching one file attach it once). An `add` that matches **zero** files emits a **warning** `MissingExternal` at `attachments.rules[i].add` (auxiliary payload, not an error: it does not suppress the plan). `content_type`/name are left to mkvmerge to infer.

- [ ] **Step 1: Write the failing tests.**

```rust
// Primary has attachments: id0 "a.ttf", id1 "b.otf", id2 "cover.jpg".
// rules: [ { select: { substring: { file_name: .ttf } } } ], unmatched: drop
// -> keep only id0:
assert_eq!(plan.attachments.primary, PrimaryAttachments::Subset(vec![0]));
// rules: [], unmatched: keep -> KeepAll
// rules: [], unmatched: drop -> DropAll
// rules: [ { drop: { substring: { file_name: cover } } } ], unmatched: keep
//   -> keep id0,id1 (all but the dropped) = Subset(vec![0,1])
// add locator matching two font files beside the primary -> add_files has both, sorted
// two add rules matching the same file -> that path appears once (dedup by path)
// add locator matching zero -> warning MissingExternal at attachments.rules[i].add,
//   plan still present
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core --test planner_resolution attachment` -> FAIL.
- [ ] **Step 3: Implement.** Add `resolve_attachments(profile, primary, primary_dir, primary_attachments: &[Attachment], diags) -> AttachmentPlan`:
  - **Existing attachments (select/drop/unmatched):** for each attachment of the primary, walk `profile.attachments.rules` in order; the **first** rule with a `select` expr that matches -> keep; the first with a `drop` expr that matches -> drop; `add` rules are skipped in this pass. If no `select`/`drop` rule matches, fall to `unmatched` (`Keep` -> keep, `Drop` -> drop). Collect kept ids. Then reduce to `PrimaryAttachments`: kept == all ids -> `KeepAll`; kept empty -> `DropAll`; else `Subset(sorted kept ids)`. (Reducing to `KeepAll` when everything is kept keeps the argv minimal; command emits no filter.)
  - **Adds:** for each rule with `add: Some(locator)`, `resolve_locator(locator, primary_dir, &primary.identifier)`; extend `add_files` with all hits (already sorted by `walk_files`); if that rule's hits are empty, push `Diagnostic::warning(MissingExternal, format!("attachments.rules[{i}].add")).for_file(&primary.path)`. After all rules, dedup `add_files` by path preserving first-seen order (two rules matching one file must attach it once).
  - Return `AttachmentPlan { primary, add_files }`. Wire into `Plan`. Update the `MissingExternal` doc comment in `report.rs` to read "track rule, chapters, or attachment add".
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(planner): resolve attachments (rules, unmatched, external adds)`.

---

