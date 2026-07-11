<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-8  (round 1 of 1)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_01CG9TaC5XjV2eQu1bb5YNKo
  agent_id:           a03ef6ed20452fede
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a03ef6ed20452fede.jsonl
  dispatch_desc:      Review Task 8 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-07T23:59:14.866Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

## Spec Compliance

**Missing:** None. All required diagnostic codes (`UnsupportedProfileVersion`, `NoTrackRules`, `EmptyMatchExpression`, `UnknownProperty`, `NotStringProperty`, `ValueTypeMismatch`, `InvalidRegex`, `UnknownSettableProperty`, `AttachmentRuleShape`) are wired, and all four binding `config_path` shapes from the brief are reproduced exactly (`tracks[0].match.exact.colour_depth`, `tracks[0].match.any[0].exact.nonexistent_prop`, `attachments.rules[0]`, `tracks[0].changes.bitrate`-style).

**Extra:** None.

**Misunderstood:** None. `crates/muxsmith-core/src/profile/validate.rs` in the diff is a verbatim transcription of the brief's Step 3 code, and `tests/validate_semantics.rs` is a verbatim transcription of the brief's Step 1 tests — I diffed both against the brief line by line and found no divergence.

Targeted verification per the "do not trust the report" checklist, all confirmed correct in the diff itself:
- Recursion into `any`/`not` (`validate.rs` lines ~167-176 in diff) correctly threads the `prop_type: fn(&str) -> Option<PropType>` parameter and appends `.any[{i}]` / `.not[{i}]` to the path before recursing.
- `scalar_fits` (lines ~209-218) truth table: `(Int, Float)` -> true, `(Float, Integer)` not in the match arms -> false, `Bool` only paired with `Boolean`. Matches the required coercion rule exactly.
- Regex compile check (`regex::Regex::new(value)`) sits inside `if kind == "regex"`, so it never fires for `substring`.
- `validate_changes` dispatches through `capability::settable(prop)`, not `matchable_type` — correctly using the settable-property surface rather than the matchable one.
- Attachment action count uses a single `actions != 1` check over a 3-element boolean array, so both 0 and >2 correctly trigger `AttachmentRuleShape` via one code path rather than special-cased branches.

⚠️ Unverifiable from this diff alone (files not touched by this change, so not in the diff, and I did not crawl the repo per instructions): whether `capability::matchable_type`, `capability::settable`, and `capability::ATTACHMENT_PROPERTIES` (Task 6) actually expose the signatures this code assumes; and whether `MatchExpr::is_empty()`, `Scalar::type_name()`, and `Diagnostic::error/warning` (Tasks 2/4) behave as assumed. Taken on faith from the fact that the reported test run (and the controller-confirmed 48-green workspace run) passed, since a signature mismatch would fail to compile.

Confirmed via `grep -P '[^\x00-\x7F]'` over the diff file: zero non-ASCII characters, so the em-dash/curly-quote constraint is clean.

## Strengths

- Faithful, exact transcription of the brief into working code — no scope creep, no shortcuts.
- Correct handling of the two easy-to-get-wrong rules: Int->Float one-way coercion, and "changes validate against settable, not matchable" (a natural place to accidentally reuse `track_prop_type`).
- Attachment rule shape check treats 0 and >1 actions uniformly via `!= 1`, avoiding duplicated error-emission branches.
- Path construction is centralized through `format!("{base}...")` chaining, giving consistent, predictable `config_path` strings that match the brief's binding shapes exactly.
- Test file covers both `codes()`-only assertions and, where the brief calls for it, structural assertions on `config_path` and `params["property"]` (e.g. `unknown_match_property_is_flagged_with_path`, `nested_any_and_not_are_validated_recursively`).

## Issues

#### Critical
None.

#### Important
1. **(Plan-mandated)** `InvalidRegex`'s `"detail"` param is populated with the raw `regex::Error::to_string()` output (`validate.rs`, the `if kind == "regex"` block). The `regex` crate's syntax-error `Display` impl typically renders a multi-line message with a source excerpt and a caret pointing at the offending character, not a short technical token comparable to a type name. This risks breaking the "code+params only, no prose sentences (short technical strings only)" constraint and could misbehave in a GUI expecting single-line param values. This is exactly the code given in the brief's Step 3, so it's a design choice inherited from the plan, not something the implementer introduced or deviated on.

#### Minor
2. Duplicate string construction: `format!("{base}.match")` is built twice per track rule — once for the `EmptyMatchExpression` diagnostic, once for the `validate_expr` call — instead of being bound to one local (`let match_path = ...`) and reused. Harmless, just a small redundant allocation. Also inherited verbatim from the brief.
3. Report-accuracy: the implementer's report claims "Full workspace test: All 30 tests pass," but per the controller's note the actual run is 48 green. The undercount looks like it only tallied files the implementer was aware of rather than the whole workspace. No effect on this diff's correctness, but the self-reported numbers shouldn't be trusted without the controller's cross-check.

## Assessment

**Task quality:** Approved

**Reasoning:** The diff is a correct, complete, verbatim implementation of the brief's Step 3 code and Step 1 tests, with all binding `config_path` shapes, type-coercion rules, and attachment-shape logic verified directly against the diff text. The one substantive concern (verbose regex-error text in a diagnostic param) is a property of the plan's own prescribed code, not an implementer defect, and is flagged for follow-up rather than as grounds to reject this task.