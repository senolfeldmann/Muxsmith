# F4 review: absent boolean-typed property compares equal to `false` in `exact` matching

Reviewer: independent, read-only. No modifications made.

## Verdict

- **SPEC: pass**
- **QUALITY: approved**

## What was checked

- Diff (`F4-review-package.txt`) against current `crates/muxsmith-core/src/matcher.rs` (HEAD `213e1e9`) -- identical, already committed, working tree otherwise clean (only an unrelated untracked `HANDOFF.md`).
- `crates/muxsmith-core/src/capability/mod.rs` (`PropType`, `matchable_type`, `SETTABLE`) and `capability/generated.rs` (`MATCHABLE_PROPERTIES`) for the type table backing the gate.
- `crates/muxsmith-core/src/identify.rs` (`Track::get`, `PropValue`) to confirm `type`/`codec`/`id` are always `Some` (never hit the changed arm's absent branch) and that `properties.get` absence is the only path in.
- Spec 4.3/4.4 in `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` and the D-decision record in `2026-07-09-plan-2-design-decisions.md`.
- Ran `cargo test --package muxsmith-core --lib matcher::` (11/11 pass, including the 3 new tests), `cargo test --workspace` (all green), `cargo clippy --workspace --all-targets -- -D warnings` (clean), `cargo fmt --all --check` (clean).

## Spec conformance

Spec 4.4 states the rule generically for any boolean-typed matchable property ("a boolean-typed matchable property that a track's `-J` output omits compares equal to `false`"), not narrowly for the four named vanity flags -- the named flags are the motivating example, not the scope boundary. The implementation's gate on `matchable_type(prop) == Some(PropType::Boolean)` matches that generality exactly: it fires for any absent boolean matchable property (e.g. `text_subtitles`, `enabled_track`), not just the four vanity flags. Correct scope, not overfit to the example.

Polarity is correct and verified against spec text and both new tests: absent matches `false`, not `true`. `scalar_eq(want, &PropValue::Bool(false))` correctly rejects `want = true` (`true == false` is `false`) and accepts `want = false`.

The change is confined to the `_ =>` fallback arm of `exact_matches`, as required:
- `language` and `codec_kind` special-case arms are byte-for-byte unchanged (confirmed by diff context lines).
- `substring`/`regex`/`any`/`not` in `matches()` are untouched (diff touches only `exact_matches`).
- `track_str`, `lang_eq`, `scalar_eq` are untouched.

No scope creep found.

### Edge case: absent non-matchable (unknown) property

An unknown property name has `matchable_type(prop) == None`. The inner match's `_` arm catches both `None` (unknown) and `Some(PropType::String | Integer | Float)` (typed-but-non-boolean), returning `false` for all of them -- identical to pre-change behavior (`None => false`). This is correct: matcher.rs's module doc states it "assumes a validated expression," and `UnknownProperty` is a config-time error (spec 5.2) that would have already rejected such a profile before `exact_matches` ever runs. Even if an unvalidated expression reached this code defensively, the result is unchanged from before the fix -- no regression, no new behavior, correctly conservative.

### Type-table cross-check

Verified in `capability/generated.rs` that all four named vanity flags plus the other boolean matchables (`default_track`, `forced_track`, `enabled_track`, `text_subtitles`) are `PropType::Boolean`, and that `track_name` (used in the non-boolean absent test) is `PropType::String`. The gate reads the correct table.

## Quality / test hygiene

Ran independently, not just read: `cargo test --package muxsmith-core --lib matcher::` shows all 11 tests passing, including:

- `absent_boolean_property_compares_equal_to_false` -- track without `flag_hearing_impaired`; asserts both polarities in one test (`false` matches, `true` does not). Per the implementer's report this was the one test confirmed red against the pre-fix code (`None => false` unconditional), then green after. I did not independently re-run it against a reverted matcher.rs, but the assertion shape is such that it could not pass under the old unconditional `None => false`: `matches(exact: {flag: false})` would have returned `false` (no match) under the old code, and the test asserts `assert!(matches(...))` -- so vacuous-pass under the old code is not possible. Not vacuous.
- `present_boolean_property_still_matches_its_real_value` -- guards the untouched `Some(have)` branch; correctly does not exercise the new code path, and the report is honest that it "passed trivially" pre-fix. Legitimate regression guard, not mislabeled as new-behavior coverage.
- `absent_non_boolean_property_still_does_not_match` -- guards that the non-boolean side of the new nested `match` still returns `false`. Also passed pre-fix (same reason: behavior unchanged for non-boolean), but it is not vacuous as a guard against a plausible implementation mistake (e.g. an errant `Some(_) =>` instead of `Some(PropType::Boolean) =>` in the nested match, which this test would catch and the other two would not distinguish from correct).

Together the three tests cover: absent+boolean+false-want (match), absent+boolean+true-want (no match), present+boolean (both wants, unaffected), absent+non-boolean (no match, unaffected). That is the full decision matrix the task specifies; nothing is missing (e.g. no test for an absent boolean under a *non*-`exact` condition, but `substring`/`regex` are explicitly string-only per 4.3/4.4 and validate.rs rejects boolean properties there at config time, so that combination is not constructible in a valid profile -- correctly out of scope, not a gap).

No determinism or soundness concerns: `matchable_type` and `scalar_eq` are pure, total, side-effect-free lookups/comparisons; no ordering or iteration-dependent behavior introduced.

Checked for a parallel implementation that should have received the same fix and didn't: `planner.rs`'s `rank_of` (near-miss diagnostic ranking for `MissingTrack` hints, lines ~807-822) lists the same flag names but is an unrelated ranking heuristic, not a second match-evaluation path; it does not need this fix and correctly wasn't touched.

## Findings

None. No Critical, Important, or Minor findings.
