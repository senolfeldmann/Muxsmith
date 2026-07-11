<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 2 of 2)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01FiKAXfK4UqVuM4mU1ZVHQV
  agent_id:           a379436f841447550
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a379436f841447550.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-07T23:53:15.806Z
  continuation_trigger: The coordinator sent a message while you were working: Your two Important findings (plus t
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance (fix scope)

All five verification points confirmed against the diff:

1. **Error precedence** — `split_once(':')` now separates name/filter first, `name.is_empty()` is checked before filter resolution. Traced: `{:}` and `{:int}` -> `EmptyField`; `{x:}` -> name nonempty, filter `Some("")` -> wildcard -> `UnknownFilter`. Exactly as directed. No regression risk: `{x:}` produced `UnknownFilter { name: "" }` before the fix too (old wildcard arm caught it), so only the empty-name cases changed behavior.
2. **Int filter on empty input** — early `return String::new()` on `value.is_empty()`; the non-empty all-zeros collapse (`"000"` -> `"0"`) is untouched and its original test (`int_filter_keeps_single_zero`) retained.
3. **`pos` semantics** — doc comment on `TemplateError` declares it a character offset and explicitly forbids byte-slicing with it. Documentation-only resolution (offset stays char-based), which is a legitimate contract choice; the hazard is now a stated contract instead of a trap.
4. **New tests** — exactly the three fixed cases: `empty_name_with_filter_is_empty_field` (`{:int}`, `{:}`), `int_filter_on_missing_field_renders_empty` (first test in the suite that exercises a genuinely missing Ctx field), and the `{x:}` -> `UnknownFilter` assertion added to the existing rejects test.
5. **Nothing else changed** — single file (`crates/muxsmith-core/src/template.rs`), 44+/8-; hunks are the doc comment, the parse reorder, the Int-arm early return, and tests. No API, Cargo, or lib.rs changes.

Report arithmetic now checks out: 10 template + 16 pre-existing core = 26 lib, +6 profile_load +2 xtask = 34, matching the controller's verified 34 green. The report includes a genuine failing-first TDD run (2 failed pre-fix with the exact expected assertions). No non-ASCII in the fix diff.

### Strengths

- Minimal, surgical fix: restructures only the decision order, no gratuitous refactor of the surrounding parser.
- The two-phase split (name check, then filter resolution) makes the precedence deliberate and readable instead of an accident of match-arm order, with a comment stating the intent.
- The new missing-field test closes the original coverage gap where every test populated every field.

### Issues

#### Critical

None.

#### Important

None.

#### Minor

1. `{x:}` yields `UnknownFilter { name: "" }` — an empty-string payload. Whatever renders this downstream will print something like `unknown filter ""`. Correct per the directive, but when Task 9 builds error display, a dedicated message for the empty-filter case would read better. Cosmetic, deferred concern.
2. Carried over, out of fix scope: error tests still assert variant only (`matches!(..., Err(Variant { .. })`), never the `pos`/`name` payload values.

### Assessment

**Task quality:** Approved

**Reasoning:** All three findings are fixed exactly as specified, each with a failing-first test, and the diff contains nothing beyond the fix scope. Remaining items are cosmetic and belong to Task 9's error-display work, not this gate.