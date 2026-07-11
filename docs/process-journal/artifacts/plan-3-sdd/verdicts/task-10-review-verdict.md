<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-10  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01WyZkG6eq1tryim2vLqLzpX
  agent_id:           a9073865e4a4060d7
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a9073865e4a4060d7.jsonl
  dispatch_desc:      Review Task 10 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T12:36:00.671Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
✅ Spec compliant

Verified against the diff and the current source (`crates/muxsmith-core/src/command.rs`, `crates/muxsmith-core/tests/command.rs`):

- **Option lookup**: `push_track_properties` (command.rs:167-187) destructures `capability::settable(&c.property).expect(...)` into `(_, option)`, equivalent to the spec's `.unwrap().1`. Cross-checked against `capability::mod.rs:93-98` and its own locked test table (`mod.rs:175-202`): `default_track` -> `--default-track-flag`, `language` -> `--language`, `track_name` -> `--track-name`, `forced_track` -> `--forced-display-flag`. All four properties used in the new golden tests resolve correctly; no wrong-option-lookup risk.
- **Value encoding** (`value_str`, command.rs:192-199): `Scalar::Bool` -> `"1"`/`"0"`, `Scalar::Str` -> raw clone (no quoting), `Int`/`Float` -> `to_string()`. Matches the spec exactly. `Scalar` (profile/match_expr.rs:19-28) has exactly these four variants, so the match is exhaustive with no silent catch-all.
- **Ordering**: tracks sorted by `track_id` ascending (command.rs:174), then each track's changes sorted by `property` ascending (command.rs:178) — defensive re-sort as the brief requires even though the planner already emits BTreeMap order.
- **Golden test full-argv assertion** (`per_track_properties_and_multi_group`, tests/command.rs:116-197): asserts the complete `Vec<String>` from `--output` through `--track-order 0:0,0:1,1:0`, including the donor group at index 1 and fixed category order (video, audio, subtitles, buttons) per group. Property order verified: `--default-track-flag 1:1` before `--language 1:de` (primary), `--language 0:tr` before `--track-name 0:Turkce` (donor) — both alphabetically correct.
- **Bool true/false coverage split across two tests**: `default_track=true` -> `"1:1"` in the multi-group test; `forced_track=false` -> `"0:0"` in `boolean_and_string_value_encoding` (tests/command.rs:201-263). Both encodings exercised.
- **Out-of-scope check**: no `--no-chapters`, `--no-global-tags`, `--no-track-tags`, or attachment-filter flags anywhere in the diff or the new tests. The donor group in the golden test correctly omits `--no-attachments` (Task 11's job per the brief) — this matches the brief's explicit scope cut, not an omission.
- **Task 9 logic untouched**: confirmed by reading the full current `command.rs` — `input_groups` (76-84), `group_index` (90-95), `push_track_order` (205-220) are byte-identical to what the diff's context lines show; only `push_group` (130-137) changed, by inserting one call (`push_track_properties`) between the existing `push_track_selection` call and the `(` bracket, exactly the slot Task 9's own comment reserved.
- `#![deny(missing_docs)]`: `push_track_properties` and `value_str` are private (no `pub`), consistent with every other helper in the file (`push_group`, `push_track_selection`, etc., none of which carry doc comments either) — lint not triggered.
- ASCII typography: grepped the diff for em/en-dash, curly quotes, ellipsis, nbsp, unicode minus — none found.

### Strengths
- Full-argv golden assertions (not substring checks) lock the exact contract the brief calls for, including the multi-group skeleton inherited from Task 9.
- Two tests cleanly partition coverage: multi-group + mixed types in one, isolated true/false boolean encoding in the other, without redundant overlap.
- Doc comments (module header, `push_group`) were updated in lockstep with the added functionality rather than left stale.
- Correctly identified and preserved the Task 9/Task 11 scope boundaries down to where the `--no-attachments` flag is *supposed* to still be missing.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- `crates/muxsmith-core/src/command.rs:194` — `Scalar::Bool(b) => if *b { "1" } else { "0" }.to_string(),` relies on `.to_string()` binding to the whole if/else expression rather than just the `"0"` branch, which is correct but reads at a glance like it only stringifies the else arm. Wrapping in parens, `(if *b { "1" } else { "0" }).to_string()`, removes the ambiguity for the next reader without changing behavior.

### Assessment
**Task quality:** Approved
**Reasoning:** Per-track property emission matches the canonical argv contract exactly (option lookup, value encoding, track/property ordering), the golden test asserts the full multi-group argv as required, and Task 9's grouping/track-order logic is provably untouched. Only a cosmetic readability nit found.