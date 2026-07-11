<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 2 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01FnTcFSSSb13PueBtCZzqzS
  agent_id:           a810b23dffefd05a9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a810b23dffefd05a9.jsonl
  dispatch_desc:      Final whole-branch review Plan 1
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-08T08:37:35.934Z
  continuation_trigger: The coordinator sent a message while you were working: The fix wave for your final review 
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

**Confirmed ready.** All three Important findings verified resolved against cd3f239; independently re-verified: prose grep on `muxsmith-core/src` is clean, 81/81 tests pass, tree clean.

- **#1**: `validate.rs:177-192` now emits `name` / `kind`+`pos` params; `diagnostics.ftl:16-19` uses a valid Fluent selector (missing-args render still differs from the key, so the completeness guard keeps working). Exit criterion "prose-free core" now passes.
- **#2**: spec 5.2 table and 9.2 amended exactly as recommended; code untouched.
- **#3**: `validate.rs:327` conflicts only on `Some(true)`; `Some(false)` → `InvalidKeyword` at `{path}.match_to_source`; both semantics locked by `validate_structure.rs:277,287`.
- Triage items 1, 2, 3-partial, 5-partial, 7 all present and correct (duplicate-key naming, deep content asserts, locator-default asserts, full SETTABLE table lock, locator-EmptyExtensions + filename-keyword tests). Test delta +5 = 81, consistent.

Two non-blocking nits, no action required: `load.rs` still sets the now-unused `at` param (harmless duplicate of `config_path` in `--json`; drop on next touch), and the `invalid-template` selector's default variant is `*[empty-field]`, so an unknown `kind` would mislabel — only reachable by adding a `TemplateError` variant without a catalog update.

No remaining blockers. Plan 1 can be declared done; the deferred list matches my triage.