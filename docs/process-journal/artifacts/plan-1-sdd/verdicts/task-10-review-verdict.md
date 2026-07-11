<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-10  (round 1 of 1)
  session_uuid:       3836dae8-154c-4f10-a808-f79207b38a3f
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f.jsonl
  tool_use_id:        toolu_013L1555AAdQNeAxidhrnczn
  agent_id:           a64c39fa7bc3acfc4
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/3836dae8-154c-4f10-a808-f79207b38a3f/subagents/agent-a64c39fa7bc3acfc4.jsonl
  dispatch_desc:      Review Task 10 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-08T00:15:17.081Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

# Task 10 Review: Static Overlap Lint

## Spec Compliance

The diff is a byte-for-byte transcription of the brief's Step 3 code (`lint.rs` lines 29-69 in the diff match brief lines 120-161 verbatim) plus the mandated `pub mod lint;` wiring. Checked against the global constraints:

| Constraint | Verified |
|---|---|
| Both rules primary-source | `SourceCfg::Keyword(k) if k == "primary"` filter applied before pairing — a non-primary rule never enters either role of a pair |
| Both exact-only (no substring/regex/any/not, non-empty exact) | `is_exact_only` checks all four exclusion fields `.is_none()` plus `exact.is_some_and(|m| !m.is_empty())` |
| One map subset of the other, including equality | `subset_of(a,b) \|\| subset_of(b,a)`, where `subset_of` is `a.iter().all(|(k,v)| b.get(k) == Some(v))` — correct ⊆-or-equal semantics |
| Warning severity, `ProvableOverlap` code | `Diagnostic::warning(DiagCode::ProvableOverlap, ...)` |
| `config_path` = `tracks[{b_idx}]` | exact literal match, using `b_idx` not `a_idx` |
| `rule_a`/`rule_b` params as indices | `a_idx.to_string()` / `b_idx.to_string()` |
| Negation/regex/any/external skipped entirely | filtered out before pairing, not just excluded per-pair |
| ASCII identifiers/comments | scanned the diff; no em-dashes, curly quotes, or other non-ASCII found |

Traced the pairwise loop by hand for the untested direction (superset-conditions rule at the lower index, e.g. `tracks[0]={type,language}`, `tracks[1]={type}`): `subset_of(a,b)` correctly returns false, `subset_of(b,a)` correctly returns true, one diagnostic emitted with `rule_a=0, rule_b=1`. The filtered vector preserves original track order (filter never reorders), so `a_idx < b_idx` always holds — the indices are positional, not "which side is the subset," matching the spec's letter exactly.

Identical-maps case: single `if` with `||`, not two separate pushes → exactly one diagnostic for `a==b`, confirmed against the brief's own `identical_exact_rules_are_provable_overlap` test (asserts `len()==1`).

No spec deviation found.

## Strengths

- Correct, minimal implementation: filter-then-pairwise-compare, no unnecessary abstraction for what is inherently an O(n^2) lint over a small rule count.
- `subset_of` computes true set-containment via per-key comparison rather than map equality, which is the only way to get "subset including equality" right in one code path.
- Faithful, verifiable transcription from brief to diff — no drift introduced during implementation.

## Issues

#### Important
- **Test suite never exercises the `subset_of(b, a)` branch specifically.** All 5 tests only construct cases where the earlier-indexed rule (lower `tracks[]` index) is the narrower/superset-conditions one; none put the more-specific rule first and the more-general rule second. The code is correct in that branch (verified above by hand), but a future refactor that broke only that half of the `||` (e.g. an accidental `&&`, or reversing which of `a`/`b` gets passed) would pass the full test suite undetected. **Plan-mandated**: these are the exact 5 tests specified verbatim in the brief's Step 1; the implementer did not choose to omit coverage, they transcribed the brief's own test list.
- **Skip-category tests are not exhaustive.** `rules_with_negations_or_regex_are_skipped` despite its name only exercises `not:`; no test independently exercises a bare `regex:`, `any:`, or `substring:` field. All four are correctly checked in `is_exact_only`, but only one of four is under regression protection. **Plan-mandated**, same reasoning as above.

#### Minor
- Report overclaims: "Tests exhaustive" is not accurate given the two gaps above, and the workspace-wide test count ("51 tests") is wrong — already caught by the controller (actual: 63 green). Doesn't affect the code, but is a second instance of the report's self-verification being unreliable; worth weighting future reports from this implementer down accordingly.

## Assessment

**Task quality:** Approved

**Reasoning:** The lint logic is a correct, verbatim-faithful implementation of the brief's subset-overlap algorithm; every global constraint (primary-source, exact-only, subset-or-equal, diagnostic shape, ASCII) is satisfied and all 5 brief-mandated tests pass. The coverage gaps are properties of the brief's own test list, not implementation defects, and don't block this task-scoped gate — flagged above for whoever owns test-hardening follow-up.