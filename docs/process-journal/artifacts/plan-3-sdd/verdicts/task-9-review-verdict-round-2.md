<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-9  (round 2 of 2)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01JbdQGXbfuxQASuW2QP98KN
  agent_id:           a2bea7bbaa0a70345
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a2bea7bbaa0a70345.jsonl
  dispatch_desc:      Re-review Task 9 after fix
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T12:23:44.490Z
  continuation_trigger: You are re-reviewing Task 9 of Muxsmith Plan 3 after a fix. A prior review found ONE Impor
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

### Strengths
- `input_groups` (crates/muxsmith-core/src/command.rs:94-102) now gates donor-source inclusion on `a.track_id.is_some()` while keeping the primary unconditional (`vec![plan.source.clone()]` at line 95) - exactly the fix requested, and it correctly preserves the spec's "primary always group 0 even if it contributes no tracks" rule (task-9-brief.md:168) since that special case is untouched.
- `group_index`'s invariant (command.rs:104-113) holds: every call site is `push_track_order` (command.rs:183-198), which only calls `group_index` for assignments where `a.track_id` is `Some`. Any such assignment's `source` is guaranteed present in `groups` - either it's `plan.source` (always group 0), or the `input_groups` loop's own condition (`a.track_id.is_some() && !already present`) added it. No path to a lookup miss; the `.expect(...)` is unreachable by construction, as the doc comment claims.
- Regression test `unmatched_donor_rule_opens_no_input_group` (crates/muxsmith-core/tests/command.rs:284-328) does pin the bug. I traced it against the pre-fix (unconditional) algorithm by hand: with the old code, `/m/e.tr.srt` (track_id: None) would still get pushed into `groups` at first appearance, `push_track_selection` would find zero ids for it, and `push_group` would render `--no-video --no-audio --no-subtitles --no-buttons ( /m/e.tr.srt )` - so the assertion `!argv.iter().any(|a| a == "/m/e.tr.srt")` (test.rs:318-321) would fail under the old code and passes under the fix. Confirmed regression coverage.
- The prior review's four ✅ areas hold unchanged in this diff: global section (`push_global`, command.rs:117-142) matches contract item 1; track selection (`push_track_selection`, command.rs:160-177) matches item 2d including the non-mechanical `--no-video`/`--no-subtitles`/`--no-buttons` flag names; `--track-order` (`push_track_order`, command.rs:183-198) matches item 3, correctly omitted when no assignment has a track; no Task 10 (per-track property) or Task 11 (chapters/tags/attachment-filter) flags leak into argv - `push_group` (command.rs:149-155) contains only track selection and the `( source )` bracket, matching the documented slot-in comment.
- ASCII/typography: grepped the full diff for em/en-dashes, curly quotes, ellipsis, NBSP, and any non-ASCII byte - zero hits. `#![deny(missing_docs)]` unaffected; only `pub fn command` is public and carries a doc comment; `pub mod command;` inserted alphabetically in lib.rs (command.rs:212, between capability and discovery) as claimed.

### Issues
None Critical or Important. One Minor observation, not blocking:

#### Minor
- **command.rs:94-102** - ordering subtlety in `input_groups`, not exercised by any test. When a single donor source has multiple assignments with mixed `track_id` (e.g., one rule producing `track_id: None` for that source, a later rule producing `track_id: Some(..)` for the same source), the donor's position in `groups` is now determined by the index of its *first `Some`* assignment, not its first appearance overall - and if a *different* donor's first `Some` assignment appears earlier in `plan.assignments` than that, the two donors' relative group order (and thus their `--track-order` group indices and their `( path )` render order) can differ from what "first-appearance order across assignments" would give if read literally against the unfiltered assignment list. The canonical reference (task-9-brief.md:168) predates this None-donor case, so there's no golden test to arbitrate it either way, and the chosen semantics (first-appearance among assignments that actually contribute a track) is a defensible reading, arguably the more sensible one. Flagging only so it's a conscious choice going into Task 12's real-mkvmerge round trip, not a silent side effect. No fix required now; not in scope of this re-review's checklist.

### Assessment
**Task quality:** Approved
**Reasoning:** The fix is exactly what was requested (condition is `track_id.is_some()` plus already-present check, primary unconditional), the invariant `group_index` depends on is structurally guaranteed, the regression test provably fails on the pre-fix code and passes on the fix, and none of the four previously-approved areas regressed. The diff is ASCII-clean and scoped strictly to `input_groups` + its doc comment + the new test, matching the report's stated scope.