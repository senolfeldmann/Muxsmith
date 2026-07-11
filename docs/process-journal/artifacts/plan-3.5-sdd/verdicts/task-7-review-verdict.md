<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 1 of 1)
  session_uuid:       ffc7c915-ea2a-432b-9ffd-64ac313a2a6f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f.jsonl
  tool_use_id:        toolu_01M7Z6VKPCbRtCYohCx9D3GY
  agent_id:           abc7a08dbfc571cde
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f/subagents/agent-abc7a08dbfc571cde.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T18:36:03.284Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance Verdict: PASS

All stated requirements for Task 7 are met. Verified directly against the diff and, for the five named risks, against the surrounding (undiffed) source.

## Spec Compliance

| Requirement | Status |
|---|---|
| `Plan.primary_track_ids: Vec<u64>`, doc-commented | met - `crates/muxsmith-core/src/planner.rs:106-112` |
| Populated from primary `Identification`, source order | met - `planner.rs:534` (`ident` = `id.identify(&primary.path)` at line 348, i.e. the primary; `Track` fields are parsed straight from the `-J` JSON array with no re-sort, `identify.rs:162-166`) |
| `push_track_order` branches on `keep_unmatched`; keep emits primary ids then donor assignments, no double-listing | met - `command.rs:52-90` |
| `drop` branch unchanged | met - byte-identical logic, just moved into the `else` arm; existing drop goldens (`unmatched_donor_rule_opens_no_input_group`, `per_track_properties_and_multi_group`, etc.) still assert the old exact strings |
| `push_track_selection` / `push_track_properties` untouched | met - no hunks touch either function |
| Live test restructured to primary+donor, donor trails, verified against real mkvmerge | met - `command_integration.rs`'s `live_keep_donor_trails_primary`, self-skips via `mkvmerge()` guard, asserts `["PA","PB","DONOR"]` |
| `validate.rs` `NoTrackRules` config_path `"tracks"` -> `"tracks.rules"` | met - confirmed no test asserts the old string (only `DiagCode` is asserted in `validate_semantics.rs:54`) |
| spec 4.5 updated, ASCII-only | met - `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:190`; grepped the whole diff for non-ASCII bytes, zero hits |

⚠️ Full-workspace `cargo test`/`fmt`/`clippy -D warnings`/`deny` results were not independently re-run per the task's instruction (controller already confirmed green); taken on trust as instructed.

## Strengths

- `push_track_order_keep` (`command.rs:76-90`) is a direct, literal transcription of the D20-B decision text (verified against `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md:127-136`), including the "matched-primary assignments already covered, don't repeat" comment matching the actual filter logic.
- The hard-coded `"0:{tid}"` group index for the primary is justified by `input_groups` (`command.rs:75-83`), which unconditionally seeds `groups[0]` with `plan.source` - checked, not assumed.
- `keep_unmatched_suppresses_primary_selection_flags` upgraded from a presence check to an exact-string assertion (`0:0,0:1,0:2`) that specifically exercises the no-double-listing path (one assignment matches primary track id 1).
- The live test rewrite is a genuine two-file (primary + separate donor) cross-mux, hand-verified against real mkvmerge before being encoded as a Rust assertion (SI-3 discipline), and the old test name/references are cleanly replaced, not left dangling.
- Scope discipline: unrelated already-dirty working-tree files (`HANDOFF.md`, plan-4 design decisions) were deliberately excluded from the commit.

## Issues

**Important**
- No fast, pure unit test exercises the donor branch of `push_track_order_keep` (`command.rs:82-88`: the `a.source != plan.source` extend-with-donor path). The only two `keep_unmatched: true` occurrences in the whole test suite are `command.rs`'s `keep_unmatched_suppresses_primary_selection_flags` (primary-only, no donor) and `command_integration.rs`'s `live_keep_donor_trails_primary` (primary+donor, but gated on `mkvmerge()` and **silently returns** if it's absent). The exact scenario this task exists to fix (primary + donor mixed under `keep`) therefore has zero guaranteed-run regression coverage in an environment without mkvmerge installed - a cheap, deterministic `Plan`-literal test asserting the exact `--track-order` string (e.g. two groups, primary ids `[0,1]` + one donor id, expect `"0:0,0:1,1:0"`) was available and not added. Not a correctness defect (the feature is right, and the live test did run for real in this gate), but a durability gap in the fast test suite for the task's core new logic.

No Critical issues found.

**Minor**
- Pre-existing, not introduced by this diff: `command_integration.rs`'s module doc still says "Two tests:" while listing three bullets. Cosmetic, out of this task's scope.

## Task Quality Verdict

Approve. The implementation is a precise, minimal-diff transcription of decision B, with no scope creep (`push_track_selection`/`push_track_properties` genuinely untouched, drop path genuinely unchanged), correct primary-group indexing, and a real mkvmerge-verified live guard. The one gap worth acting on before calling this durably done is adding a fast unit test for the primary+donor keep-mode mix so the core new branch has a regression guard that runs unconditionally, not only when mkvmerge happens to be installed.