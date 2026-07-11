<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-11  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01D3ga2VRTrG8Cdo4xX5PpBT
  agent_id:           ad4fb968c143666e0
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-ad4fb968c143666e0.jsonl
  dispatch_desc:      Review Task 11 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T12:49:58.473Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

Verified against `/home/senol/Git/Muxsmith/.superpowers/sdd/task-11-brief.md` and the appended canonical-argv contract:

- **Per-group order (a/b/c before d/e/f):** `push_group` (command.rs:128-138) calls `push_group_chapters` → `push_group_tags` → `push_group_attachments` → `push_track_selection` → `push_track_properties` → bracket. Matches canonical a→f exactly.
- **`--no-chapters` on every group for Drop and External:** `push_group_chapters` (command.rs:142-149) matches `Drop | External(_)` in one arm, `Keep` emits nothing; called unconditionally for every group. Confirmed by both `chapters_drop_...` (tests/command.rs:447) and `chapters_external_...` (tests/command.rs:494), each showing `--no-chapters` on primary *and* donor.
- **Tag flags for both booleans:** `push_group_tags` (command.rs:153-160) independently checks `!global_keep` → `--no-global-tags`, `!track_keep` → `--no-track-tags`, correct order, applied to every group. Confirmed by `tags_dropped_emit_flags_on_every_group` (tests/command.rs:543).
- **Attachment filter, primary vs. donor:** `push_group_attachments` (command.rs:166-180) branches on `source != plan.source.as_path()`. Donor: unconditional `--no-attachments`. Primary: `KeepAll`→nothing, `Subset(ids)`→`--attachments`,`"id,id"`, `DropAll`→`--no-attachments`. All three primary dispositions and the donor case are each exercised by a dedicated full-argv golden test (tests/command.rs:310, 357, 387).
- **Global `--chapters`/`--attach-file` unchanged:** `push_global` (command.rs:98-123) is untouched context in the diff (only the two new-type import and doc-comment lines changed above it); confirmed live in the file too.
- **Tasks 9-10 untouched:** `input_groups`, `group_index`, `push_track_selection`, `push_track_properties`, `push_track_order` are all present in the diff only as unchanged context (verified by reading the full file) — no hunk touches their bodies.

**NAMED CHECK — `per_track_properties_and_multi_group` (tests/command.rs:162-200):** confirmed. The only change is a single inserted line, `"--no-attachments"`, at line 180, positioned immediately after the primary group's closing `")"` (line 179) and before the donor group's `"--no-video"` (line 181) — i.e. slot (c) of the donor group, exactly where the canonical order puts it. No other line in that test's plan construction or expected-argv literal changed. This is a legitimate incremental golden-test lock, not a masked regression.

### Strengths
- Canonical order encoded as a literal, readable sequence of calls in `push_group` rather than any inferred/derived ordering — matches the "explicit over magic" convention.
- Doc comments (module header, `push_group`) updated accurately to drop the "not yet implemented" language and describe the full a-f order.
- All 7 required brief cases (Subset+donor, DropAll, KeepAll, add_files, chapters Drop, chapters External, tags both-false) are present as separate tests, each a full `assert_eq!` against the complete argv vector, not partial/contains-style assertions.
- Scope discipline: diff touches exactly two files, no signature changes, no changes to `planner.rs` (types were pre-existing per Task 4), consistent with the task's stated boundaries.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
1. **DRY duplication of the id-join one-liner** — `crates/muxsmith-core/src/command.rs:176` (`push_group_attachments`, new) and `crates/muxsmith-core/src/command.rs:199` (`push_track_selection`, pre-existing) both contain `ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",")` verbatim. Only two occurrences, so under the project's own scale-appropriate-design rule ("three similar lines beat a premature abstraction") this doesn't clear the bar for an extracted helper yet — flagging for awareness only, no action needed now.
2. **Report overstates its own justification** — the report (task-11-report.md, "Concerns"/self-review section) claims the brief "explicitly documents ids as already sorted ascending from resolution" for `PrimaryAttachments::Subset`, used to justify skipping a defensive sort. The two appended references given for this task make no such claim for attachments — only track-selection ids (item 2d) are explicitly called out as "ascending"; item 2c (attachments) says nothing about order. This isn't a spec violation (the canonical contract doesn't require sorted attachment ids in the output, so the code is compliant regardless), and functionally harmless either way since mkvmerge's attachment filter doesn't care about list order. Worth a quick check against whatever upstream task resolves `AttachmentPlan.primary` if `--attachments` ID ordering ever needs to be deterministic for golden tests beyond what's covered here.

### Assessment
**Task quality:** Approved
**Reasoning:** Per-group flag content, order, and primary/donor determination all match the canonical contract exactly; all 7 required golden-test cases assert full argv; the one pre-existing Task 10 test was updated with precisely the single required line in the correct position, with no other change to its plan or assertions. Only cosmetic, non-blocking observations remain.