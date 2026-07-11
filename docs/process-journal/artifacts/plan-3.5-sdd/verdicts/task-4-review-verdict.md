<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-4  (round 1 of 1)
  session_uuid:       ffc7c915-ea2a-432b-9ffd-64ac313a2a6f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f.jsonl
  tool_use_id:        toolu_014ERDg1LV7EboyaJeXhdBJA
  agent_id:           ad9257438001e35f3
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/ffc7c915-ea2a-432b-9ffd-64ac313a2a6f/subagents/agent-ad9257438001e35f3.jsonl
  dispatch_desc:      Review Task 4 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T17:28:35.105Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

All other language values used in the test suite are real ISO fixture codes (`en`, `de`, `tr`) or the two fixed fixtures already reviewed (`notalanguage`) plus the new `pt-BR` positive case. No other test relies on a non-ISO placeholder that the predicate swap would silently flip. Review complete.

---

## Spec Compliance Verdict: PASS

All brief requirements are met exactly as specified, with one in-scope, correctly-justified deviation (the `zz`/`zzz` → `notalanguage` fixture fix) that the implementer flagged rather than hid.

## Spec Compliance

| Requirement | Status |
|---|---|
| `language-tags = "0.3.2"` added to muxsmith-core | ✅ Cargo.toml + Cargo.lock, alphabetically ordered |
| `LanguageIndex::is_valid_value` = `normalize().is_some() OR LanguageTag::parse().is_ok()`, doc-commented | ✅ verbatim match to brief |
| `walk_exact_languages` swapped to `is_valid_value` | ✅ `planner.rs:142` (mechanical, monotonic widening) |
| `resolve_changes` swapped to `is_valid_value` | ✅ `planner.rs:165` (same) |
| Accept-side is well-formedness only (D19), not registry `validate()` | ✅ uses `LanguageTag::parse`, not `validate()`; doc comment states the deliberate scope cut |
| Existing ISO-code tests stay green as REAL passes | ✅ verified below |
| `#![deny(missing_docs)]` satisfied | ✅ doc comment present; ⚠️ not independently re-run (relying on controller's green gate per instructions) |
| ASCII-only typography, umlauts/Ş untouched | ✅ diff contains no non-ASCII |
| `deny.toml` untouched, MIT/Apache-2.0 already allowed | ✅ no change, matches brief's conditional |

## Strengths

- The doc comment is not just present but epistemically honest: it states the D19 scope cut (well-formed-but-fictitious tags pass) explicitly at the call site, so a future reader doesn't have to reconstruct the design decision from the diff.
- The `is_valid_value` swap is provably monotonic: `self.normalize(token).is_some() || ...` means every token previously accepted (real ISO code) is still accepted, and the new clause only widens. There is no path by which the swap could regress the `de`/`ger` ISO behavior it's built on top of.
- The fixture investigation is genuinely rigorous, not asserted: the implementer empirically probed `language-tags` (`zz`/`zzz`/`xx`/`nolang` → `Ok`, `notalanguage` → `Err(SubtagTooLong)`) rather than trusting RFC 5646 from memory, and reported the exact probe outputs.
- The new positive test asserts a concrete negative (`!... .any(|d| d.code == DiagCode::InvalidPropertyValue)`) rather than the weaker "a plan exists," which is exactly what's needed to prove the regression is fixed and not just coincidentally not-crashing.
- Report transparently flags the one out-of-brief edit as a judgment call rather than folding it in silently.

## Issues

None at Critical or Important severity.

**Minor:**
- `crates/muxsmith-core/tests/planner_resolution.rs:998` (`bad_language_value_is_batch_invalid_property_value`) and `:157` (`invalid_changes_language_is_plan_time_invalid_property_value`): reusing the same literal fixture string (`notalanguage`) in both regression tests is fine functionally, but a future reader skimming the file has no way to see they're deliberately using an identical string versus that being coincidental. Not worth a change now, just noted for anyone touching these tests later.

## Named-risk verification (details)

1. **Fixture substitution correctness and necessity** — confirmed independently, not just taken on the implementer's word. The test-local `LanguageIndex` fixture (`crates/muxsmith-core/tests/planner_resolution.rs:26-32`) contains only `en`/`de`/`tr` (English/German/Turkish), so `normalize()` returns `None` for `zz`, `zzz`, and `notalanguage` alike under both old and new predicates — the ISO-index half of the risk is satisfied. For the BCP-47 half: RFC 5646's `language` production is `2*3ALPHA / 4ALPHA / 5*8ALPHA`, so `zz` (2 letters) and `zzz` (3 letters) are both well-formed primary-language subtags and would newly parse `Ok` under `is_valid_value`, while `notalanguage` (12 letters) exceeds the 8-letter ceiling and fails well-formedness on both sides of the swap. This makes the fixture change both correct (still exercises the invalid path) and necessary (the old fixtures would have started silently passing plan-time validation without it). Assertions in both tests are unchanged, only the literal value.
2. **pt-BR test asserts absence, not just presence** — confirmed: `crates/muxsmith-core/tests/planner_resolution.rs:249-256` asserts `fr.plan.is_some()` AND a concrete negative (`!fr.diagnostics.iter().any(|d| d.code == DiagCode::InvalidPropertyValue)`), which is a real regression guard, not a weaker "produces a plan" smoke test.
3. **ISO-code path preserved at both call sites** — confirmed by inspection: `is_valid_value`'s first disjunct is `self.normalize(token).is_some()`, identical to the pre-existing predicate; the swap only adds an `||`, so `de == ger` and all other real-code behavior is untouched. Cross-checked against every other `language:` fixture in the test file (`en`, `de`, `tr` at lines 69, 121-123, 233, 252, 272, 325) — none of them rely on the removed strictness, so no other test silently changed meaning.

## Task Quality Verdict

**Approved.** The implementation matches the brief precisely, the one deviation is well-reasoned, empirically verified rather than assumed, correctly scoped (only the two fixtures whose meaning the swap actually broke), and self-disclosed. No issues block acceptance.