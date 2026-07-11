<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01MJnh6ykYAoycHfuwM8xmvb
  agent_id:           a69f9660149b5d5ef
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a69f9660149b5d5ef.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T20:10:49.207Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

All findings verified against the actual file (not just the diff excerpt). Report.

### Spec Compliance
- ✅ Spec compliant.
  - Primary built via real mkvmerge with SRT + `--attach-file` and explicit `--attachment-mime-type text/plain` (command_integration.rs:483-508).
  - Plan driven via `plan_batch` with a profile (`ATTACHMENT_PROFILE`, line 473) selecting `exact: { type: subtitles }` with `changes: { track_name: Renamed, default_track: true }` — schema shape matches the pre-existing `fixtures/reference.yaml` (`changes: { track_name: English forced, default_track: true }`).
  - `command(plan)` run through real mkvmerge a second time (:526-533), output re-identified via `-J` (:535-538).
  - All three required assertions present: `track_name == "Renamed"` (:554), `default_track == true` via `PropValue::Bool(true)` (:555), attachment present with the **original** file_name `"note.txt"` (:566) — traced back to the fixture's `note.txt` written at :493, confirming it's the original name, not a mkvmerge-derived one.
  - Attachment survival relies on the profile's documented default (`attachments.unmatched: keep`, no rules → `AttachmentsCfg::default()`, verified at `profile/model.rs:318-325`, consumed at `planner.rs:847`), satisfying "keeping the attachment" without extra config — a legitimate reading of the brief, not a gap.
  - SI-3 probe against the real binary documented in both the report and a pinned source comment (:454-465... actually block comment precedes const at file top of new section) directly above the test.
- Diff is exactly one file, +109/-0 (confirmed via `git diff --stat`), matching "test-only change."
- No ⚠️ items — everything requested is verifiable directly from the diff plus the surrounding file it appends to.

### Strengths
- End-to-end real behavior: two genuine `mkvmerge` invocations (fixture build, planned command) plus a real `-J` re-identify — no fixture-only shortcuts.
- Self-skip idiom copied verbatim from the file's established pattern (:484-487, matches :227 and :313 exactly).
- Reuses `mkvmerge()` and `track_name()` helpers rather than duplicating (confirmed: no new helper functions added, `track_name` reused from the donor-order test).
- Independently verified library-level claims the report makes: `Identification.attachments: Vec<Attachment>` with `file_name: String` (identify.rs:83-88), `AttachmentsCfg` default-keep behavior (profile/model.rs:318-325, planner.rs:847), and `PropValue::from_json` mapping JSON booleans to `PropValue::Bool` (identify.rs:34) — all check out, so the test's assertions are checking real, existing plumbing, not asserting against something that doesn't compile-check the way claimed.
- Typography scan of the added lines (`grep -P` for em/en-dash, curly quotes, ellipsis, NBSP) found nothing; the one ` -- ` usage is the file's own pre-existing ASCII substitution convention (6 total instances in the file including this one, i.e. 5 pre-existing as the report claims).
- Hermetic: own `tempfile::tempdir()`, own output subdir, no shared/global state, no dependency on other tests' ordering.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- The block comment (command_integration.rs, directly above `ATTACHMENT_PROFILE` at :473) records the SI-3 probe reasoning in prose but the actual probe transcript only lives in the task report, not in git history as a runnable script — consistent with how the file's other live tests document probes (prose comment, not a checked-in script), so this is stylistic parity, not a gap.
- `let lang = m.list_languages().expect(...)` is fetched but the profile never matches on language — dead weight, but required by `plan_batch`'s/`LiveIdentifier`'s signature and copied unchanged from the two sibling tests, so it's parity with established local convention, not a defect introduced here.

### Assessment
**Task quality:** Approved
**Reasoning:** All three required assertions are present and correctly wired to real mkvmerge output; the diff is scoped to exactly the one file/one test the brief asked for; independent verification of the library types and defaults the test relies on (`Attachment.file_name`, `AttachmentsCfg` default, `PropValue::Bool` mapping) confirms the assertions check real behavior, not assumed shapes.