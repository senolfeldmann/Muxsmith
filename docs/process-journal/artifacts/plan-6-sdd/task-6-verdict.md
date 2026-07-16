# Verdict (extracted from the reviewer transcript at write-time)

I have everything I need. The Q2 command reproduced the reported output exactly, and I've verified D49 in full, both house-relevant negatives, and the named risks.

### Spec Compliance

**StructuredEdit reshape** (`planner.rs:207-234` post-diff)
- ✅ `AddExact`/`AddNotExact` carry `value: Scalar`; `AddSubstring`/`AddNotSubstring` keep `value: String` (settled asymmetry, D49 "The wire shape").
- ✅ Derives exactly `Debug, Clone, PartialEq, Serialize, Deserialize`; `#[serde(tag = "kind", rename_all = "snake_case")]` present.
- ✅ **No `ts` residue anywhere.** No `#[cfg_attr(feature = "ts", ...)]` line, no `#[cfg(feature = "ts")] use ts_rs::TS;`. Confirmed by full-diff read and by the diff touching only the two allowed files (ts_rs lives nowhere in them). Correct per the Task-5 cross-task constraint.
- ✅ Doc comment matches D49 `:346-354` verbatim.
- ✅ `planner.rs:10` -> `use serde::{Deserialize, Serialize};`.

**delta_for** (`planner.rs:163-...`)
- ✅ Loses the `scalar: &Scalar` param, single-argument, reads the edit's own `value` on both exact arms. **Stays private** (no `pub`). Matches D49 `:418-447`.

**Four engine call sites**
- ✅ `:1746`/`:1753` -> `value: scalar.clone()` (diff lines 100, 108); `:1762`/`:1791` -> `delta_for(&edit)` (diff lines 119, 142); synthetic `Scalar::Str(tok.to_string())` at `:1791` dropped. `prop_value_as` untouched; `display` still keys `seen` (diff 116) and `rank` (diff 120).

**apply_suggestion / edit_key / ApplyError**
- ✅ `apply_suggestion` `pub`, three exits, no-op detected by one `applied == *profile` comparison, splices through `with_rule_match(profile, index, &delta_for(edit))`. Rustdoc present. Matches D49 `:477-499`.
- ✅ `edit_key` private, total over four variants.
- ✅ `ApplyError` `pub`, **exactly three variants** (`UnparsableConfigPath`, `RuleIndexOutOfRange { index, rules }`, `EditChangedNothing { index, property }`), derives `Debug, Clone, PartialEq`, **NOT `Deserialize`**. Rustdoc on enum and every variant field.

**Tests / fixture / helpers** (all seven present, verbatim modulo rustfmt wrapping)
- ✅ Harness split `plan_model`/`plan_multi`/`plan` (D49 `:819-845`); old hand-rolled `plan` duplicate removed.
- ✅ `yaml_scalar` (D49 `:1268-1275`), `spliced_scalar` (D49 `:869-886`), `P_ALREADY_CONSTRAINED` (D49 `:1073-1079`).
- ✅ Seven guard tests, each matching its D49 section: `apply_splices_..._bool_property` (G1, `:895`), `..._int_property` (G2, `:943`), `every_applied_suggestion_survives_..._model_level` (G3, `:989`), `apply_rejects_an_unparsable_config_path` (`:1034`), `apply_rejects_a_rule_index_past_the_end` (`:1047`), `apply_rejects_an_edit_the_no_clobber_merge_drops` (G4, `:1081`), `apply_returns_ok_when_the_edit_reaches_the_model` (control, `:1100`). G1/G2 carry their `checked > 0` anti-vacuity asserts (diff 608-611, 645-648).

**Seven existing value-binding sites**
- ✅ Four template sites now interpolate `yaml_scalar(value)` (diff 355-363 in `apply_edit_to_first_rule`, 433-441 in `apply_edit_to_no_clobber_rule`); substring arms correctly still use bare `{value}`.
- ✅ `:325` -> `&Scalar::Str("Chapter 1: Intro".to_string())`; `:722` -> `&Scalar::Bool(true)` (Boolean, deliberately not following `:325`); `:890` -> `&Scalar::Str("eng".to_string())`.
- ✅ **Named risk (old `:890` negative assertion) verified:** diff line 510 is `Scalar::Str`, not `Scalar::Bool`. A wrong type there would have passed vacuously (`!any(...)`); the literal is `Str` as `language` is String-typed. Correct.

**Suggestion / DiagCode** — ✅ Verified directly: `Suggestion` (`planner.rs:238`) derives `Debug, Clone, PartialEq, Serialize`; `DiagCode` (`report/mod.rs:40`) derives `Debug, Clone, Copy, PartialEq, Eq, Serialize`. Neither gains `Deserialize`. The negative grep was validated against a known-present control (the same grep shows `Deserialize` on `StructuredEdit:208`), so the absence is real, not a malformed check. `core-37-prose-free-core` intact.

**House dimension** — ✅ No deviation. `ApplyError` follows the `SettingsError` operational-error precedent (`core-124`); splice reuses the engine's own helpers (`core-44`/`core-33`/`core-72` preserved, per D49 "Not changed by this ADR"); `plan_model` split matches `testing-support-helpers` (removes a duplicate rather than adding one). Typography in added code is ASCII-clean.

⚠️ Cannot be shown by the diff alone: the nine-part gate results (fmt/clippy/test/doc/deny/pnpm) are the implementer's claim; the diff is consistent with them but I did not re-run the suite per instructions.

### Adjudications

**Q1: Correct local-idiom adaptation, properly disclosed — not a deviation requiring escalation.**

The brief and D49 (`:855-857`) both phrase the imports as a prose directive ("add ... to the file's imports"), not a fenced code block. The drift-surface / verbatim-copy rule the brief invokes is explicitly scoped to *settled test code* ("a second copy of settled test code would be a drift surface"); it binds the fixture, helpers, and seven test bodies, which are given as code blocks. An import directive is a symbol list, not a transcribable block, so nothing verbatim was contradicted. Merging `ApplyError`/`apply_suggestion` into the file's existing `use muxsmith_core::planner::{...}` is the idiomatic Rust form and matches the house pattern — the file already carried a single consolidated `planner` import. On the rustfmt/clippy point the verdict asked me to check: the report's reasoning is sound in substance — rustfmt's default `imports_granularity = Preserve` does **not** merge two separate same-module `use` statements, so writing them as two lines would have persisted a non-idiomatic double-import. (One correction to the framing: default clippy has no lint that *rejects* split same-module imports, so the case rests on idiom + house pattern + rustfmt-won't-fix-it, not on a clippy error.) The result is symbol-identical and behavior-identical, and the implementer disclosed it in the report's Concerns section, which is exactly the right handling for a judgment call at this altitude.

**Q2: The awk range captures the whole function body; the reported output is exact and reproducible.**

I ran, against the committed worktree file:
```
awk '/pub fn apply_suggestion/,/^}/' .../planner.rs | grep -nE "with_rule_match|delta_for|extend\("
```
Output:
```
12:    let applied = with_rule_match(profile, index, &delta_for(edit));
```
Identical to the report. On the `/^}/` concern: `apply_suggestion`'s body has no column-0 brace before its own closing brace — every interior block (`if index >= rules { … }`, `if applied == *profile { … }`) closes at 4-space indent — so the first `^}` is the function's own terminator and the range is exactly the function, not truncated early nor overrunning. The `.extend(add.clone())` that does exist in the file is inside `with_rule_match` (a separate function ending before `apply_suggestion` begins), correctly outside the range, exactly as the brief's comment predicted. One `with_rule_match`+`delta_for` line, zero `extend(` lines: the no-clobber reuse is structural (splices through the engine's helpers), not a `BTreeMap::extend` in the applier. Bug C shape absent.

### Strengths
- The three comparison-site literals are all correct, including the two non-obvious ones that `proc-latitude-clause-boundary`'s own evidence log flagged as the exact failure point (`:722` Boolean, `:890` Str-under-negative-assertion). This task's single highest-risk spot was handled right.
- Faithful verbatim transcription of D49; the report's line-by-line self-diff against each D49 section is good discipline, and the only deltas are rustfmt-mandated wrapping (`spliced_scalar`'s `AddNotExact` arm, the G3 tuple array).
- Clean RED/GREEN TDD evidence with a plausible 16-error RED matching the brief's expectation; correct scoping (two files, no `src-tauri`, no ts residue).
- Anti-vacuity `checked > 0` guards and the `EditChangedNothing` control test are all present — the applier cannot pass on a vacuous or unconditional-error implementation.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **Stale internal line-references in copied comments** (`edit_key` comment "planner.rs:1824, :1829", diff 232-233; `spliced_scalar` comment "planner.rs:1812, :1817", diff 559-560). Both point at pre-edit locations and are now stale. This is *plan-mandated*: the brief's copy-verbatim instruction requires keeping D49's text as-is, and D49 carries these exact refs. Harmless, but a standing micro-drift surface. Not the implementer's error to fix under this brief.
- **G1/G2 are construction-level identity tests** ("test the compiler," in D49's own words at `:1130`): under D49, `delta_for` reads the edit's own field, so "engine emits typed scalar" and "apply splices that same scalar" are true by construction. D49 records this and `proc-proposed-safeguard-stays` mandates keeping them until an implementation-time measurement (flip `delta_for` to re-stringify and observe whether only G3 fails). Keeping them is correct; flagged only so the recorded removal-experiment is not lost.

### HARVEST
- **Verbatim-copy scope clarification (convention candidate).** This task exposed a real tension: D49 gave test *bodies* as fenced code blocks (verbatim-binding) but *import* changes as prose directives. The implementer correctly treated only the fenced blocks as subject to the drift-surface rule and applied local idiom to the prose directive. Worth codifying explicitly: "the verbatim-copy / drift-surface obligation binds fenced code blocks an ADR hands over, not its prose directives; prose directives are implemented idiomatically." Prevents a future implementer from mechanically producing a non-idiomatic double-import to satisfy a literal reading.
- **Stale line-refs in copied comments are a recurring drift surface.** Internal `planner.rs:NNNN` references inside comments go stale the instant the file is edited, and the copy-verbatim rule propagates them. Candidate convention: prefer symbol references over line-number references in comments destined to be copied across an edit (`delta_for`'s exact arms, not `planner.rs:1812`). Reported for the ledger; I did not act on it.
- **Dominant pattern reinforced:** implementer report carrying an explicit per-section self-diff against the source ADR, plus a disclosed judgment-call section, is the shape that made this review fast and left no unverifiable claim. Consistent with `proc-57` behavior.

### Assessment
**Task quality:** Approved
**Reasoning:** Every brief and D49 requirement is met, both adjudication checks resolve in the implementation's favor (the awk proof is reproducible and the import merge is correct local idiom), and the single highest-risk element — the three typed comparison literals, including the vacuity trap at `:890` — is correct. The only findings are two Minors, one of them plan-mandated.
