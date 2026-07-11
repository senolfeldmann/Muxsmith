<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-5  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01CSq16bMtgpvQtLiLvwxxSP
  agent_id:           a32837981ecd98b06
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a32837981ecd98b06.jsonl
  dispatch_desc:      Review Task 5 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T20:17:43.336Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant

**Verified against the diff:**
- `std::mem::forget` removals: exactly 15, zero remaining, zero newly added (verified by grepping every `-`/`+` line in the diff for `std::mem::forget`). Per-file split: 13 in `planner_resolution.rs`, 2 in `suggestions.rs` — matches the brief's per-file counts exactly.
- No production (non-`tests/`) file touched: diff stat lists only `command_integration.rs`, `planner_resolution.rs`, `suggestions.rs`, `tests/support/mod.rs`, all under `crates/muxsmith-core/tests/`.
- `crates/muxsmith-core/tests/support/mod.rs` is a subdirectory module (not `tests/support.rs`), correctly avoiding Cargo's `tests/*.rs` autodiscovery, per its own doc comment (diff lines 1563-1569).
- `FakeIdent`'s `identify()` body (diff lines 1583-1591) is byte-identical to `command_integration.rs`'s pre-change version (diff lines 75-83) — the brief's designated canonical source. `pub`/doc-comment/`#[allow(dead_code)]` additions are the only deltas, all required by the brief.
- `lang()` (diff lines 1596-1602) is byte-identical to both files' pre-change 3-row en/de/tr versions.
- All three consumer files use `mod support; use support::{FakeIdent, lang};` (`command_integration.rs` diff:45-46, `planner_resolution.rs` diff:141-142, `suggestions.rs` diff:1324-1325) with local struct/fn definitions fully deleted, not left dead.
- No assertion text, fixture content, or profile YAML changed anywhere in the diff — every hunk outside the support-module extraction is exactly import trimming, `let batch = X` → `let (batch, _dir) = X`, and helper-signature tuple returns.
- Single commit `65eef3c`, matching the brief's exact required message.

**Scrutinized claim (task's named risk): does the 1-row → 3-row `LanguageIndex` change `suggestions.rs` test semantics?**
Checked the production matcher/suggestion code (unchanged by this diff, but the risk requires reading it): `candidates_for_rule` (`crates/muxsmith-core/src/planner.rs:1052-1114`) derives suggestion candidates from the *actual property values present on identified tracks* (`t.properties`), not by enumerating `LanguageIndex` rows. `lang` is only threaded into `matcher::matches`/`lang_eq` (`crates/muxsmith-core/src/matcher.rs:94-104,142-151`) to normalize/compare language tokens that already appear in the fixture data. Since `suggestions.rs`'s fixtures and profiles (`P_AMBIGUOUS`, `P_NO_CLOBBER`, `P_COLON_AMBIGUOUS`) only ever reference `en`/`eng`/`English`, the extra German/Turkish rows are inert: they can't add candidates that weren't already discriminable from the English-only data, and `lang.normalize("eng")` is unaffected by unrelated rows existing in the same table. Confirms the report's claim.

**Second, unflagged behavioral micro-difference found (inert, but not called out in the report's self-review):** `suggestions.rs`'s original `FakeIdent::identify` error message was `IdentifyError::Json("no fixture".into())` (diff line 1317), distinct from `command_integration.rs`/`planner_resolution.rs`'s `format!("no fixture for {name}")`. The shared module adopts the latter (the brief's designated canonical form), so `suggestions.rs` silently gets a different error string on its no-fixture path. Verified this path is never exercised: `plan()` and `plan_multi()` (`suggestions.rs`) always insert a `by_name` entry for every file they write to disk, so `identify()` never errors in this file's tests. Spec-compliant per the brief's letter (command_integration.rs is the named canonical source) and behaviorally inert, but the report's "behaviorally inert" self-review only checked the language-index row count, not this message-text divergence — a gap in the stated verification, not in the actual code.

### Strengths
- Every one of the ~35 `plan_one(...)`/`plan_two_same_output(...)`/`plan_one_with_existing_output(...)`/`plan(...)`/`plan_multi(...)` call sites was correctly rebound to `let (x, _dir) = ...`; the compiler would have hard-failed on any miss (tuple vs. struct field access), which is corroborating evidence for the report's 215/215 claim.
- The inline forget removals were checked for a genuine risk (does dropping the tempdir early break a later filesystem read) and are all safe: every `expected`/comparison value derived from `dir.path()` is captured into an owned `PathBuf`/`Vec<PathBuf>` before the drop point, and `batch` is fully computed (a plain in-memory value) before the tempdir goes out of scope at function end.
- Nested `_dir` shadowing inside loops (`suggestions.rs`, `every_suggestion_survives_the_next_dry_run` and `with_rule_match_never_widens_an_existing_substring_constraint`) is valid, idiomatic Rust — each iteration's tempdir drops independently, no double-drop or premature-drop risk.
- Import trimming is precise and consistent across all three files (`Path`, `Identify`, `IdentifyError`, `LanguageIndex` dropped only where genuinely unused post-extraction).
- Support module is minimal — exactly the two required items, no speculative helpers.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- Report's forget-site breakdown ("6 helpers, 6 forget sites... plan, plan_multi, no_clobber_batch") doesn't reconcile with the diff: `no_clobber_batch` (`suggestions.rs`) has no `std::mem::forget` call of its own — it only forwards `plan_multi`'s already-converted tuple return and needed a signature-only ripple update. The true split is 5 helper-direct forget removals + 10 inline test-body removals = 15, not "6 + 9." Cosmetic inaccuracy in the report's self-description; the code and the 15-site total are correct.
- The unflagged `FakeIdent` error-message divergence noted above (`"no fixture"` → `"no fixture for {name}"` in `suggestions.rs`'s dead-code path) would be worth one added line in the self-review for completeness, since the report explicitly claims to have checked "behaviorally inert" but only verified the language-index angle.

### Assessment
**Task quality:** Approved
**Reasoning:** All spec requirements verified against the diff (exact 15/15 forget-site count, verbatim `FakeIdent`/`lang()`, zero production-code touch, zero assertion changes, single correctly-named commit); the one named scrutiny risk (3-row language index affecting `suggestions.rs` semantics) checked against the actual production matcher/suggestion code and confirmed inert. Remaining findings are report-accuracy nits, not code defects.