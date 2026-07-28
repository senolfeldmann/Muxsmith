# Task 7 report - Plan 9: the D49 G1/G2 removal experiment (D105)

**Status: NEEDS_CONTEXT** - the measurement landed on D105 step 4's **anomaly
branch**, not on either clean branch. No removal in any direction; the guards
stay in the tree; the controller has a fork at the plan close (section
"Surfaced for the controller", item 1).

Measurement run on `master`, main worktree, `/home/senol/Git/Muxsmith`, HEAD
`a8fe11f`, working tree clean at start. Clock at the control run:
`2026-07-29T00:19:09+02:00`.

---

## Headline

| guard | test | under the mutation |
|---|---|---|
| G1 | `apply_splices_the_simulated_scalar_for_a_bool_property` | **green** (ok) |
| G2 | `apply_splices_the_simulated_scalar_for_an_int_property` | **red** - but through its anti-vacuity assertion (`suggestions.rs:1101`), not through its type-equality assertion (`:1094`) |
| G3 | `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level` | **green** (ok) |

D105 step 4's own illustration of the anomaly class is "e.g. G3 passes under the
mutation". G3 passed. The observed pattern is inside the anomaly class as the
design itself names it.

Two tests that are **not** guards went red under the same mutation:
`apply_returns_ok_when_the_edit_reaches_the_model` (`suggestions.rs:1219`) and
`ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions`
(`suggestions.rs:511`). Aggregate under the mutation: 16 passed, 3 failed.

---

## Step 1: green control (`proc-check-green-state-reachable`)

Command: `date -Is && cargo test -p muxsmith-core --test suggestions 2>&1 | tail -60`

```
2026-07-29T00:19:09+02:00
   Compiling muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.93s
     Running tests/suggestions.rs (target/debug/deps/suggestions-1efa447a116ec06a)

running 19 tests
test apply_rejects_an_unparsable_config_path ... ok
test apply_returns_ok_when_the_edit_reaches_the_model ... ok
test apply_rejects_a_rule_index_past_the_end ... ok
test apply_rejects_an_edit_the_no_clobber_merge_drops ... ok
test ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions ... ok
test apply_splices_the_simulated_scalar_for_an_int_property ... ok
test yaml_fragment_round_trips_a_value_containing_a_colon ... ok
test apply_splices_the_simulated_scalar_for_a_bool_property ... ok
test suggestion_cap_truncation_is_logged_not_silent ... ok
test tc_b_two_required_overlap_yields_no_suggestion_and_names_all_claimants ... ok
test ambiguous_rule_gets_a_validated_suggestion ... ok
test no_single_fix_produces_a_two_group_partition ... ok
test tc_d_three_claimant_overlap_yields_no_suggestion_and_names_all_three ... ok
test tc_a_overlap_optional_rule_yields_not_narrowings_on_that_rule ... ok
test tc_c_batch_unsafe_overlap_narrowing_is_rejected_by_the_multiset_guard ... ok
test with_rule_match_never_widens_an_existing_substring_constraint ... ok
test every_suggestion_survives_the_next_dry_run ... ok
test ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule ... ok
test every_applied_suggestion_survives_the_next_dry_run_at_the_model_level ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

All three guards are present and green on the unmutated tree, so a red in step 3
is attributable to the mutation and not to a pre-existing red suite.

---

## Step 2: the exact mutation

Located by the `AddExact` match arm, not by the plan's authoring-time line
numbers (`proc-57-briefs-not-ground-truth`). Re-derived anchors, read at
execution time:

- `fn delta_for` at `crates/muxsmith-core/src/planner.rs:1820`; its
  `StructuredEdit::AddExact` arm at `:1823-1827`; the mutated line is `:1825`.
  The design's cited span (`planner.rs:1820-1827`) contains the changed line, so
  there is no drift that affects the mutation; only the arm's own start is
  `:1823`, one line below the `fn`.
- `fn scalar_display` at `:856` (design says `:856`; confirmed by reading it).
- `map.insert(property.clone(), value.clone());` occurs twice in `delta_for`
  (`:1825` in the `AddExact` arm, `:1831` in the `AddNotExact` arm). The edit was
  anchored on the whole `AddExact` block so only `:1825` changed - see the diff.

Command: `git diff -- crates/muxsmith-core/src/planner.rs`

```
diff --git a/crates/muxsmith-core/src/planner.rs b/crates/muxsmith-core/src/planner.rs
index c83f29c..3d3d2fa 100644
--- a/crates/muxsmith-core/src/planner.rs
+++ b/crates/muxsmith-core/src/planner.rs
@@ -1822,7 +1822,7 @@ fn delta_for(edit: &StructuredEdit) -> MatchExpr {
     match edit {
         StructuredEdit::AddExact { property, value } => {
             let mut map = BTreeMap::new();
-            map.insert(property.clone(), value.clone());
+            map.insert(property.clone(), Scalar::Str(scalar_display(value)));
             m.exact = Some(map);
         }
         StructuredEdit::AddNotExact { property, value } => {
```

One line, one file, the arm D105 names. Nothing else touched.

---

## Step 3: the suite under the mutation

Command: `cargo test -p muxsmith-core --test suggestions 2>&1 | tail -80`

```
   Compiling muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running tests/suggestions.rs (target/debug/deps/suggestions-1efa447a116ec06a)

running 19 tests
test apply_rejects_a_rule_index_past_the_end ... ok
test apply_rejects_an_edit_the_no_clobber_merge_drops ... ok
test apply_returns_ok_when_the_edit_reaches_the_model ... FAILED
test apply_rejects_an_unparsable_config_path ... ok
test ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions ... FAILED
test yaml_fragment_round_trips_a_value_containing_a_colon ... ok
test ambiguous_rule_gets_a_validated_suggestion ... ok
test apply_splices_the_simulated_scalar_for_an_int_property ... FAILED
test apply_splices_the_simulated_scalar_for_a_bool_property ... ok
test tc_d_three_claimant_overlap_yields_no_suggestion_and_names_all_three ... ok
test suggestion_cap_truncation_is_logged_not_silent ... ok
test tc_b_two_required_overlap_yields_no_suggestion_and_names_all_claimants ... ok
test no_single_fix_produces_a_two_group_partition ... ok
test tc_a_overlap_optional_rule_yields_not_narrowings_on_that_rule ... ok
test tc_c_batch_unsafe_overlap_narrowing_is_rejected_by_the_multiset_guard ... ok
test every_suggestion_survives_the_next_dry_run ... ok
test with_rule_match_never_widens_an_existing_substring_constraint ... ok
test ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule ... ok
test every_applied_suggestion_survives_the_next_dry_run_at_the_model_level ... ok

failures:

---- apply_returns_ok_when_the_edit_reaches_the_model stdout ----

thread 'apply_returns_ok_when_the_edit_reaches_the_model' (1618269) panicked at crates/muxsmith-core/tests/suggestions.rs:1219:5:
assertion `left == right` failed
  left: Some(Str("true"))
 right: Some(Bool(true))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions stdout ----

thread 'ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions' (1618263) panicked at crates/muxsmith-core/tests/suggestions.rs:511:5:
expected both a codec-based and an id-based suggestion, got [AddExact { property: "codec", value: Str("SubRip/SRT") }, AddExact { property: "codec", value: Str("SubStationAlpha/ASS") }, AddNotExact { property: "codec", value: Str("SubRip/SRT") }]

---- apply_splices_the_simulated_scalar_for_an_int_property stdout ----

thread 'apply_splices_the_simulated_scalar_for_an_int_property' (1618271) panicked at crates/muxsmith-core/tests/suggestions.rs:1101:5:
no id suggestion in the fixture; this guard would pass vacuously


failures:
    ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions
    apply_returns_ok_when_the_edit_reaches_the_model
    apply_splices_the_simulated_scalar_for_an_int_property

test result: FAILED. 16 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p muxsmith-core --test suggestions`
```

### Supplementary: each guard run in isolation, same mutation

Not a substitute for the pinned suite invocation above; run because an
in-suite pass and an isolated pass are different claims and the decision rule
reads per guard.

Command: `for t in <the three guard names>; do cargo test -p muxsmith-core --test suggestions -- --exact "$t" 2>&1 | tail -12; done`

```
--- apply_splices_the_simulated_scalar_for_a_bool_property ---
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running tests/suggestions.rs (target/debug/deps/suggestions-1efa447a116ec06a)

running 1 test
test apply_splices_the_simulated_scalar_for_a_bool_property ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

--- apply_splices_the_simulated_scalar_for_an_int_property ---

thread 'apply_splices_the_simulated_scalar_for_an_int_property' (1618736) panicked at crates/muxsmith-core/tests/suggestions.rs:1101:5:
no id suggestion in the fixture; this guard would pass vacuously
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    apply_splices_the_simulated_scalar_for_an_int_property

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p muxsmith-core --test suggestions`

--- every_applied_suggestion_survives_the_next_dry_run_at_the_model_level ---
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running tests/suggestions.rs (target/debug/deps/suggestions-1efa447a116ec06a)

running 1 test
test every_applied_suggestion_survives_the_next_dry_run_at_the_model_level ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 18 filtered out; finished in 0.01s
```

Isolated results agree with the in-suite results guard for guard.

### Per-guard table, with the failure mode

| guard | name | line | in-suite | isolated | assertion that fired |
|---|---|---|---|---|---|
| G1 | `apply_splices_the_simulated_scalar_for_a_bool_property` | `suggestions.rs:1037` | green | green | none |
| G2 | `apply_splices_the_simulated_scalar_for_an_int_property` | `suggestions.rs:1074` | red | red | `suggestions.rs:1101` - `assert!(checked > 0, "no id suggestion in the fixture; this guard would pass vacuously")`, i.e. the anti-vacuity guard, NOT the type-equality `assert_eq!` at `:1094` |
| G3 | `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level` | `suggestions.rs:1113` | green | green | none |

Non-guard reds under the same mutation, recorded because they are the tests that
did catch the degradation:

| test | line | assertion that fired |
|---|---|---|
| `apply_returns_ok_when_the_edit_reaches_the_model` | `suggestions.rs:1211`, panic at `:1219` | `assert_eq!` on the spliced scalar: `left: Some(Str("true"))` vs `right: Some(Bool(true))` - the type degradation itself, on an explicitly constructed `AddExact { default_track, Bool(true) }` |
| `ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions` | `suggestions.rs:487`, panic at `:511` | `has_codec && has_id` - the emitted suggestion set lost every `id` suggestion; pasted set: `[AddExact codec Str("SubRip/SRT"), AddExact codec Str("SubStationAlpha/ASS"), AddNotExact codec Str("SubRip/SRT")]` |

---

## Step 4: the decision rule applied

D105 step 4, verbatim: "G1+G2+G3 all fail -> they are load-bearing and **stay for
good**; only G3 fails -> G1/G2 become **removal candidates as localizers**. Any
other outcome (e.g. G3 passes under the mutation) is an anomaly: the
experiment's own premise failed; no removal in any direction, the anomaly is
recorded and routed to the controller as NEEDS_CONTEXT."

Observed: G1 green, G2 red, G3 green.

- Not the all-fail branch: G1 and G3 are green, in-suite and isolated.
- Not the only-G3 branch: G3 is green and G2 is red - the inverse of that
  branch's shape on both members.
- Therefore the **anomaly branch**, and it matches the design's own worked
  example of one ("G3 passes under the mutation").

**Selected branch: anomaly.** No removal in any direction. No guard is removed,
proposed for removal, or argued about here. The guards stay in the tree, as they
do in every branch of D105.

I did not reason from this pattern toward either clean branch, and the mechanism
section below is explicitly separated from the measurement: it is context for
the controller's ruling, not a re-derivation of a branch.

---

## Step 5: restore, and the proof

Restore command: `git checkout -- crates/muxsmith-core/src/planner.rs` (never a
bare `cp`).

**Byte proof.** `sha256sum crates/muxsmith-core/src/planner.rs` taken BEFORE the
mutation:

```
20cb4f58462db98220a7d1aec36dc8c1011c4e30e83b6a65392d1e1ba9a6efa7  crates/muxsmith-core/src/planner.rs
```

`sha256sum -c` against that baseline after the restore:

```
crates/muxsmith-core/src/planner.rs: OK
```

**Both absence checks, fired first, then green.**

Fire (taken while the mutation was in the tree, same commands):

```
=== status ===
 M crates/muxsmith-core/src/planner.rs
=== diffstat ===
 crates/muxsmith-core/src/planner.rs | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)
```

Green (after the restore; the `(end ...)` markers are echoed by the command so
that "no output" is distinguishable from "command did not run"):

```
=== git status --porcelain ===
(end status)
=== git diff --stat ===
(end diffstat)
```

**Suite green again.** Command: `cargo test -p muxsmith-core --test suggestions 2>&1 | tail -30`

```
   Compiling muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.57s
     Running tests/suggestions.rs (target/debug/deps/suggestions-1efa447a116ec06a)

running 19 tests
test apply_rejects_an_edit_the_no_clobber_merge_drops ... ok
test apply_returns_ok_when_the_edit_reaches_the_model ... ok
test apply_rejects_an_unparsable_config_path ... ok
test apply_rejects_a_rule_index_past_the_end ... ok
test apply_splices_the_simulated_scalar_for_an_int_property ... ok
test suggestion_cap_truncation_is_logged_not_silent ... ok
test apply_splices_the_simulated_scalar_for_a_bool_property ... ok
test yaml_fragment_round_trips_a_value_containing_a_colon ... ok
test ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions ... ok
test ambiguous_rule_gets_a_validated_suggestion ... ok
test tc_a_overlap_optional_rule_yields_not_narrowings_on_that_rule ... ok
test tc_b_two_required_overlap_yields_no_suggestion_and_names_all_claimants ... ok
test tc_d_three_claimant_overlap_yields_no_suggestion_and_names_all_three ... ok
test no_single_fix_produces_a_two_group_partition ... ok
test tc_c_batch_unsafe_overlap_narrowing_is_rejected_by_the_multiset_guard ... ok
test every_suggestion_survives_the_next_dry_run ... ok
test with_rule_match_never_widens_an_existing_substring_constraint ... ok
test ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule ... ok
test every_applied_suggestion_survives_the_next_dry_run_at_the_model_level ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

19 passed, 0 failed - the same 19 as the control, including the three step-3
reds, so the restore is fire-verified by those reds.

---

## Mechanism: what is verified, and the one link that is not

Context for the controller's ruling. Kept apart from the measurement above on
purpose; the anomaly stands on the pasted runs alone.

**Verified at the source:**

1. The mutation is the **identity on string-valued scalars**.
   `scalar_display` (`planner.rs:856-863`) maps `Scalar::Str(s) -> s.clone()`, so
   `Scalar::Str(scalar_display(&Scalar::Str(s))) == Scalar::Str(s)`. Only
   `Bool`, `Int` and `Float` values are degraded. Every property whose value is a
   string (`codec`, `language`, `track_name`, ...) is untouched by the
   experiment.
2. `apply_suggestion` really does route through the mutated function:
   `let applied = with_rule_match(profile, index, &delta_for(edit));`
   (`planner.rs:1903`). Call sites of `delta_for` in core, complete
   (`grep -rn "delta_for" crates/muxsmith-core/src/`): `:1772`, `:1801` (the
   candidate's `apply:` field, i.e. the engine's own simulation), `:1820`
   (definition), `:1903` (`apply_suggestion`), plus two comment mentions
   (`:1887`, `:1917`).
3. The mutation degrades **only** `AddExact`. The `AddNotExact` arm
   (`planner.rs:1828-1834`) still inserts `value.clone()` - visible in the diff's
   trailing context line.
4. In the codec/id fixture the mutation removed the `id` suggestion from the
   engine's output entirely (pasted set in step 3). `id` is an Integer property,
   so its simulated delta became `Str("1")`-shaped and the candidate no longer
   survives the engine's own validation. That is why G2 went red through its
   anti-vacuity assertion rather than through its type-equality assertion: G2
   never reached a comparison, because there was nothing left for it to inspect.

**Not verified, and I could not verify it inside this task's prohibitions:**
which suggestions G1 actually iterated in `P_AMBIGUOUS` under the mutation.
G1 passed with `checked > 0`, so it inspected at least one Boolean-property
suggestion and found the spliced scalar equal to the simulated one. Given (1)-(3)
that is consistent with G1 having inspected only `AddNotExact` suggestions - the
arm the design-fixed mutation deliberately leaves intact - possibly because the
`AddExact` Boolean candidates were dropped from the set the same way the `id` one
was. **That is a hypothesis, not a measurement.** Confirming it needs the
suggestion set printed for `P_AMBIGUOUS`, which requires touching a test file,
which Step 2 ("Nothing else is touched") and the task's Files list forbid. I did
not do it and I am not asking for latitude to do it; if the controller wants it,
it is a separate, cheap probe.

---

## Surfaced for the controller

1. **The ledger entry has no design-fixed text for this branch.** D105 step 6
   fixes exactly two statement texts, one per clean branch. The measured branch
   is the anomaly, so neither applies and, per the brief, **no implementer
   wording** may substitute. This is a fork: it returns to the controller
   (`proc-latitude-clause-boundary`) rather than being resolved here. As a
   reminder of the standing division of labour: the controller writes
   `core-d49-g1g2-experiment` at the plan close, not this task. I wrote no
   ledger entry and composed no entry text.
2. **ROADMAP "Plan-9 design trigger 4" is NOT live.** Its condition, read to its
   last clause (`docs/ROADMAP.md:728-731`): "The D49 experiment lands on the
   only-G3 branch -> the owner rules on G1/G2 removal at a plan close". The
   only-G3 branch was not measured, so the trigger does not fire. I name it
   explicitly because the brief instructs me to name it live only in the
   only-G3 case, and a silent omission would be indistinguishable from an
   oversight.
3. **The consumed D49 trigger line** (`docs/ROADMAP.md:750-756`) states only the
   two clean branches; the anomaly branch exists only in D105. Whatever the
   controller records at the close, that line's update is the controller's
   action, and its wording will have to cover an outcome the trigger's own text
   does not anticipate.
4. **The experiment's premise, stated as measured.** The premise D105 tests is
   that re-stringifying the `AddExact` delta reaches G1/G2/G3 as a
   type-degradation. Measured: it reaches the suite (three reds), but it does
   not reach it as a clean type-degradation - for string-valued properties it is
   a no-op by construction, and for non-string ones it removes candidates from
   the engine's output before the guards can compare anything. Both G2's red and
   the `ambiguity_resolvable...` red are set-composition failures, not
   type-equality failures. The one test that failed on type equality
   (`apply_returns_ok_when_the_edit_reaches_the_model`, `:1219`) constructs its
   edit explicitly instead of taking it from the engine, which is exactly why it
   is immune to the set-composition effect. **I draw no conclusion from this
   about whether G1/G2 are load-bearing** - that is the question the experiment
   was supposed to answer and did not.
5. **Anchor note, no action needed.** D105 cites the `AddExact` arm at
   `planner.rs:1820-1827`; at execution `:1820` is the `fn delta_for` line and
   the arm is `:1823-1827`. The cited span contains the mutated line (`:1825`),
   so the design's anchor is not stale in any load-bearing way. Recorded only so
   the reviewer's own re-derivation does not read a discrepancy where there is
   none.

---

## Compliance statement

- **I committed nothing.** No `git commit`, no `git add`, no `git push`, no tag,
  no branch operation. `git log` head is unchanged at `a8fe11f`.
- **The tree is byte-identical to its start.** Proven three ways above:
  `sha256sum -c` OK against the pre-mutation baseline of the only file touched;
  `git status --porcelain` empty; `git diff --stat` empty - each of the latter
  two shown firing on the mutated state first.
- **No repo file was created or modified permanently.** The only file this task
  created is this report, at
  `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-7-report.md`, in the
  SDD scratch. No house-knowledge YAML was touched, no ledger entry written, no
  ROADMAP line edited, no guard removed.
- No session-relocation tool was called; all runs were foreground on `master` in
  the main worktree.
