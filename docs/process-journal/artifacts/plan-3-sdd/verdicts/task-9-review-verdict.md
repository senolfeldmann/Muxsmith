<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-9  (round 1 of 2)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01Jv19jQrhbca5EPtVkC1efX
  agent_id:           a5f4f6a0bac9a9e38
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a5f4f6a0bac9a9e38.jsonl
  dispatch_desc:      Review Task 9 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T12:17:43.544Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ❌ Issues found — `crates/muxsmith-core/src/command.rs:71` (`input_groups`): includes donor sources whose only assignment has `track_id: None`, producing an empty input group not warranted by the canonical reference's intended reading.
- ✅ Global section ordering/content (`--output`, title, `--chapters`, `--attach-file`) — `command.rs:94-109`, matches contract item 1 exactly.
- ✅ Track selection per group, all four categories, fixed order, correct flag names, ascending ids — `command.rs:137-149`.
- ✅ `--track-order` mapping (assignment order, correct group index, omitted when empty) — `command.rs:160-170`.
- ✅ No out-of-scope argv: no per-track property options, no `--no-chapters`/`--no-*-tags`/attachment-filter flags anywhere in `push_group` — confirmed by reading `command.rs:126-132` (only calls `push_track_selection` then emits `( source )`).
- ✅ Golden test asserts the full `Vec<String>` via `assert_eq!` against a literal, not a partial check — `tests/command.rs:255-276`.

### Strengths
- `Category` table (`command.rs:21-49`) correctly captures the irregular select/no-flag naming (`--subtitle-tracks`/`--no-subtitles`, `--button-tracks`/`--no-buttons`) as data instead of trying to derive it mechanically — the right call, avoids a broken "smart" transform.
- `push_group` (`command.rs:126`) is commented with exactly where Tasks 10/11 slot in (chapters/tags/attachments before track selection, per-track props after), and the structure genuinely supports that without rework: `push_group` already receives `plan` and `source`, enough for Task 11 to distinguish primary vs donor by path comparison.
- Doc-comment discipline is correct: module `//!` doc, `pub fn command` documented, all other items private so `#![deny(missing_docs)]` is satisfied without over-documenting internals — verified no stray `pub` on `Category`, `input_groups`, `group_index`, `push_global`, `push_group`, `push_track_selection`, `push_track_order`.
- The implementer surfaced the empty-donor-group ambiguity in the report instead of silently shipping it — good instinct, even though (see below) the ambiguity resolves against their chosen reading.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
1. **`crates/muxsmith-core/src/command.rs:71-79`** (`input_groups`) — unconditionally includes every distinct `assignment.source`, including a donor whose only assignment(s) have `track_id: None`. That produces an input group that renders as `--no-video --no-audio --no-subtitles --no-buttons ( <donor> )`, contributing nothing to the mux.
   - **Why it's a misreading, not just an edge case:** the canonical reference says "primary first, then donor sources in first-appearance order across assignments," immediately followed by "The primary (`plan.source`) is always group 0 **even if it contributes no tracks**." That carve-out is only meaningful if the general rule for *non-primary* groups excludes zero-track sources — otherwise the primary wouldn't need a special exception, since it would already be covered by "every distinct source, track or not." The intended group set is: primary always, plus donor sources with at least one `track_id = Some` assignment.
   - **Failure scenario:** an external rule matches a donor file but selects zero tracks inside it (a real, documented case per the brief's own note: "an external rule CAN produce a donor source with track_id=None"). The resulting argv opens a file with mkvmerge and explicitly excludes every track category from it — at best dead weight in the command, at worst grounds for mkvmerge to warn/reject on an input contributing nothing. This directly risks Task 12's real-mkvmerge round trip, and both `group_index` (`command.rs:85`) and Task 10/11's per-group emission build on `input_groups`'s output, so the wrong group set propagates forward.
   - **Fix (minimal, contained to Task 9):**
     ```rust
     fn input_groups(plan: &Plan) -> Vec<PathBuf> {
         let mut groups = vec![plan.source.clone()];
         for a in &plan.assignments {
             if a.track_id.is_some() && !groups.iter().any(|g| g.as_path() == a.source.as_path()) {
                 groups.push(a.source.clone());
             }
         }
         groups
     }
     ```
     This preserves `group_index`'s invariant: every assignment `push_track_order` looks up has `track_id = Some`, and such an assignment's source is now guaranteed to be in `groups` by the same condition. No other function needs to change.
   - **Must be fixed in Task 9**, since group computation is entirely local to this diff; deferring it would let Task 10/11 extend a wrong group set.

#### Minor (Nice to Have)
1. **`command.rs:94-109` and `:166-170`** — the `argv.push(flag.to_string()); argv.push(value)` pair repeats four times already (title, chapters, attach-file loop, track-selection select-branch) and Task 10/11 will add several more (attachment subset, no-chapters, no-tags, per-track props). A small `fn push_flag(argv: &mut Vec<String>, flag: &str, value: String)` helper would remove the repetition once those land. Not warranted yet at this scale (matches "don't introduce an abstraction the current scale hasn't earned") — flagging only so Task 10/11's implementer considers it rather than compounding the duplication further.

### Assessment
**Task quality:** Needs fixes
**Reasoning:** The global section, track-selection, and track-order logic are all correct and cleanly structured for the follow-on tasks; the one real defect is `input_groups` including track-less donor sources as full input groups, a misreading of the canonical reference's primary-only carve-out, with real consequences for Task 12's mkvmerge round trip and for every later task that iterates the group list.