<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-5  (round 1 of 1)
  session_uuid:       2b4312c5-80eb-4fec-b4dd-a8963ceda7c2
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2.jsonl
  tool_use_id:        toolu_01H34BAtaaRKjFZsCSJAwWSg
  agent_id:           a004edcaf91f8f708
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/2b4312c5-80eb-4fec-b4dd-a8963ceda7c2/subagents/agent-a004edcaf91f8f708.jsonl
  dispatch_desc:      Review Task 5 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T11:24:03.205Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

Verified directly against source (not just the diff hunks, since the diff's context didn't show the full `resolve_file` signature):

- **Matched-branch-only population**: exhaustively checked all 6 `Assignment { .. }` construction sites in `resolve_file` (`crates/muxsmith-core/src/planner.rs:380,412,432,460,475,489`). Only line 475 (the `1 =>` single-match arm) sets `track_id: Some(tid)`, and it is the only one that now uses `changes` instead of `changes: vec![]`. All five other sites (external missing/unidentifiable/ambiguous, no-match, ambiguous-match) keep `track_id: None` and `changes: vec![]`. Matches the brief exactly.
- **Order**: `resolve_changes` (`planner.rs:551-573`) does `changes.iter().map(...)` over `rule.changes: BTreeMap<String, Scalar>` — key-ascending by construction, no re-sort needed. Test `changes_resolve_to_applied_changes_in_property_order` asserts exact `Vec` equality with `language` before `track_name`, confirming both the order and that `AppliedChange` derives `PartialEq` (pre-existing, `planner.rs:59`).
- **Language validation condition**: `let valid = matches!(value, Scalar::Str(s) if lang.normalize(s).is_some());` — for `Scalar::Str(s)` the guard enforces `normalize(s).is_some()`; for any other `Scalar` variant the pattern itself fails to match. This is logically identical to the brief's "Str+normalize==None OR not-Str" condition. Diagnostic on invalid: `Diagnostic::error(DiagCode::InvalidPropertyValue, format!("{base}.changes.language")).for_file(primary_path).with("property","language").with("value", scalar_display(value))` — `base = format!("tracks[{ri}]")` (pre-existing, `planner.rs:363`), so `config_path` renders exactly `"tracks[N].changes.language"` as specified.
- **Only `language` validated**: the `if property == "language"` guard is the only branch that validates; every other property (`sub_charset`, `track_name`, etc.) is pushed through unchanged as `AppliedChange { property: property.clone(), value: value.clone() }`.
- **Scope**: diff touches only `planner.rs` (the `1 =>` arm + two new private helpers) and the test file. `validate_language_values`/`walk_exact_languages` (batch-level walk, `planner.rs:283-320`) are untouched — confirmed by reading them directly, not just their absence from the diff. No title/tags/chapters/attachments resolution logic added.
- **Error forces `plan: None`**: pre-existing `finalize_plans` (`planner.rs:762-765`) already sets `plan = None` on any `Severity::Error` diagnostic; nothing new added or needed here, matches "no separate suppression."
- **ASCII typography**: `git diff` through `LC_ALL=C grep -n '[^ -~\t]'` on both changed files returns no matches — clean.
- **`#![deny(missing_docs)]`**: both new functions (`resolve_changes`, `scalar_display`) are private, not subject to the lint; not an issue.
- `Scalar` has exactly 4 variants (`Bool`, `Int`, `Float`, `Str`, `match_expr.rs:19-28`) and `scalar_display` matches all 4 exhaustively — no missing-arm risk, no `Display` impl duplicated elsewhere (grepped; the closest existing similar code at `planner.rs:304-308` renders a plain `String` field, not a `Scalar`, so this is a genuinely new, non-duplicative helper).

### Strengths
- Exhaustive, verifiable coverage of every `Assignment` construction site; the report's self-review claim ("grepped `changes:` ... all four remaining `vec![]` sites are the intended no-op ones") checks out against the actual six sites.
- The `matches!` boolean condition cleanly folds the "wrong type OR unrecognized string" spec branch into one expression instead of two `if`s — no over-engineering, no missed case.
- Config-path/diagnostic-parameter shape (`for_file`, `.with("property", ...)`, `.with("value", ...)`) is copied faithfully from the brief's own snippet and matches the codebase's existing diagnostic idiom (`walk_exact_languages`).
- Test fixture extension (`lang()` gains a Turkish row) is minimal and justified: needed a normalizable non-match-language value to exercise the settable-language happy path distinctly from the existing `en`/`de` match-language fixtures.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. No out-of-scope resolution logic was added; the deliberate per-file-not-batch validation scope choice is explicitly not a defect per the brief.

#### Minor (Nice to Have)
- `crates/muxsmith-core/tests/planner_resolution.rs`: no test exercises the non-`Str` `changes.language` branch (e.g. `changes: { language: true }` or `language: 5`) — only the "recognized-but-wrong string" case (`zzz`) is tested. The brief explicitly calls this out as a required behavior ("Non-string language value: also `InvalidPropertyValue`"), and the implementer's own report discloses the gap and reasons it's "trivially symmetric." I independently traced the `matches!` logic and confirmed it is correct for that branch, so this is a coverage gap, not a live bug — but it leaves that spec line without regression protection. Add one more case to either existing test or a new one asserting `InvalidPropertyValue` for a non-string `changes.language` value.
- Neither test asserts the diagnostic's `.with("property", "language")` / `.with("value", ...)` parameters, only `code` and `config_path`. This matches the brief's own test example verbatim, so not a deviation, but a slightly stronger assertion (checking the params dict) would catch a future regression where the diagnostic fires with the wrong parameter keys.

### Assessment
**Task quality:** Approved
**Reasoning:** Implementation matches every binding constraint in the brief exactly (matched-branch-only population, property-ascending order, the precise Str+normalize-OR-not-Str validation condition, exact config_path, no scope creep into batch-level validation or later tasks' resolution logic), verified by reading the actual source beyond the diff's visible context. The only gap is a disclosed, non-blocking test-coverage hole for the non-string-language branch, whose logic I independently verified is correct.