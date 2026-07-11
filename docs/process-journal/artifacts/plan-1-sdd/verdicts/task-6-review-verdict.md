<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-6  (round 1 of 1)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01CUjCyzMna4vjEaxmTYdLcN
  agent_id:           a9a2157f96341a109
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a9a2157f96341a109.jsonl
  dispatch_desc:      Review Task 6 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-07T23:40:41.975Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance

**Missing:** None. All five public API elements are present with the exact signatures specified: `PropType {String, Boolean, Integer, Float}`, `matchable_type`, `settable`, `codec_kind_prefixes`, `ATTACHMENT_PROPERTIES`.

**Extra:** `CODEC_KINDS` carries 17 entries (only `srt`/`pgs`/`h264` are pinned by tests). This is within the spec's own license ("friendly alias... plus common audio/video kinds", 4.4) — not scope creep.

**Misunderstood:** None found.

**⚠️ Unverifiable:**
- Whether `generated.rs`'s 60-property table is a complete, faithful extraction of the actual upstream v20 schema (no network access to the schema in this review; can't diff against a live regeneration).
- The report's claim that v21 404'd and v20 was the version actually fetched (external HTTP fact, not visible in the diff).

**Verified good:**
- `generated.rs` header is byte-for-byte identical to the literal strings emitted by `xtask/src/codegen.rs`'s `generate()` (lines 37-40 of `crates/xtask/src/codegen.rs` vs. lines 19-23 of `generated.rs`), and the entries are alphabetically sorted with no visible duplicates — strong evidence this is genuine generator output, not hand-authored. Field names (`chromaticity_coordinates`, `projection_pose_pitch/roll/yaw`, `max_content_light`, `stream_id`/`sub_stream_id`) are real, obscure mkvmerge/Matroska JSON fields, reinforcing that.
- Test-relied-upon properties present with correct types: `language` String, `forced_track` Boolean, `audio_channels` Integer, `type` String.
- No schema JSON committed anywhere; diff stat confirms exactly 3 files touched (`generated.rs`, `capability/mod.rs`, `lib.rs`), +201/-0.
- `SETTABLE` is a byte-for-byte match (name, type, option, order) to spec 4.4's 10-row table, cross-checked against `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:151-162`.
- `codec_kind` correctly absent from `SETTABLE` and explicitly asserted `None` in a test.
- `ATTACHMENT_PROPERTIES` matches exactly: `content_type`/`description`/`file_name` String, `id`/`size` Integer.
- `mod.rs` comment reads "Generated from identification schema v20." — consistent with the report's claimed obtained version, and satisfies the global constraint to record the actual version.
- No em-dashes, curly quotes, or non-ASCII identifiers anywhere in the diff.

## Strengths

- `generated.rs` is demonstrably real generator output rather than fabricated to satisfy the tests: header match, sort order, and dedup all line up with `codegen.rs`'s actual logic, and `sub_charset` is correctly absent from the matchable table (it's a settable-only mkvmerge input, not an identification-schema field) — a detail easy to get wrong by copying the settable list wholesale.
- `mod generated;` (private) rather than `pub mod` keeps the raw table an implementation detail behind `matchable_type()`, consistent with the brief and with sound encapsulation.
- `matchable_type`/`settable`/`codec_kind_prefixes` use plain linear `.find()` over static slices, no premature `HashMap` or macro-generated lookup table for ~70 entries — right-sized for current scale.

## Issues

#### Minor

1. **Public functions lack rustdoc.** `matchable_type`, `settable`, and `codec_kind_prefixes` (`crates/muxsmith-core/src/capability/mod.rs`, the three `pub fn` items) have no `///` doc comments, unlike the adjacent `SETTABLE`/`CODEC_KINDS`/`ATTACHMENT_PROPERTIES` statics which do. These are the module's primary entry points for Tasks 8-10; a one-line doc (e.g. "returns None for names not in the generated schema table") would cost little and isn't blocked by anything.

2. **Test coverage only spot-checks `SETTABLE`.** `settable_maps_to_mkvmerge_options` (mod.rs tests) exercises 3 of the 10 curated entries (`track_name`, `default_track`, `forced_track`) plus the `codec_kind` negative case; `language`, `flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`, `flag_original`, `enabled_track`, `sub_charset` are unexercised. A transposed option string for any of those 7 would slip through. **Plan-mandated**: this is exactly the test file the brief specified verbatim (task-6-brief.md step 2); the implementer reproduced it faithfully rather than narrowing it. Worth widening in a follow-up, not a fault of this task's execution.

## Assessment

**Task quality:** Approved

**Reasoning:** The public API, `SETTABLE` table, and `ATTACHMENT_PROPERTIES` match the spec exactly; `generated.rs` shows strong structural evidence of being genuine, unedited generator output with no schema file leaked into the tree. Remaining gaps are cosmetic (missing rustdoc) or inherited directly from the brief's own test file (narrow `SETTABLE` coverage), not implementation defects.