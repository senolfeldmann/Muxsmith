# Task 7 verdict - Plan 9: the D49 G1/G2 removal experiment (D105)

**Verdict: APPROVED_WITH_MINORS.** No fix round is required before the plan
close; both minors are citation/labelling defects in the report that do not
touch the measurement, the branch selection or the restore. The controller can
absorb both corrections when it writes the close.

Every load-bearing claim in the report was reproduced independently, not
re-read: I re-derived the `AddExact` arm, applied the mutation with my own
instrument, ran the control, the mutated suite, the three isolated per-guard
runs and the restored suite, and restored against a reference the task could
not have written (`git show a8fe11f:...`). **The report's per-guard table
matches mine in every cell.** The mutated file's blob hash in my own
`git diff` (`c83f29c..3d3d2fa`) is byte-identical to the one in the report's
pasted diff, which is a stronger statement than agreement of the diff text:
the mutated tree content was the same tree content.

I also ran the deeper probe the brief permits, in an isolated copy of the
crate under my own instrument directory (never in the repo, no repo test file
edited). It settles the one thing the report had to leave as a hypothesis, and
it produces the single most decision-relevant finding of this review, which is
about the DESIGN, not about the implementer: **D105's fenced mutation site
cannot test D105's premise, because `delta_for` feeds both the engine's
candidate simulation and the applier. The experiment is self-censoring.**

Tree state at hand-off: HEAD `a8fe11f`, `git status --porcelain -uall` empty,
`git diff --stat` empty, `crates/muxsmith-core/src/planner.rs` sha256
`20cb4f58462db98220a7d1aec36dc8c1011c4e30e83b6a65392d1e1ba9a6efa7`, identical
to the `a8fe11f` blob. I committed nothing and edited no product file.

---

## Dimension-by-dimension result

| # | dimension | result |
|---|---|---|
| 1 | mutation is the fenced one, and only it | PASS (re-derived + blob-hash identical) |
| 2 | all three runs reproduced, per guard | PASS (every cell matches) |
| 3 | branch selection | PASS (anomaly is correct under every reading) |
| 4 | G2's failure mode (anti-vacuity, not type-equality) | PASS (verified at source and at run, plus measured directly) |
| 5 | mechanism separated from measurement; four verified claims | PASS on separation; all four claims re-verified; one labelling minor (finding 2) |
| 6 | restore complete | PASS (own reference, `cmp` byte-identical) |
| 7 | prohibitions held | PASS (no commit in reflog; no tracked or untracked repo change; no ledger text composed) |
| 8 | no-work-needed premises | PASS (isolated == in-suite reproduced; design's cited span re-derived, not stale in any load-bearing way) |

### 1. The mutation

Re-derived at execution state, independently of the report and of the plan's
authoring-time numbers:

- `crates/muxsmith-core/src/planner.rs:1820` is `fn delta_for`; the
  `StructuredEdit::AddExact` arm is `:1823-1827`; the target line is `:1825`.
- The literal `map.insert(property.clone(), value.clone());` occurs at exactly
  two lines file-wide, `:1825` and `:1831` (the `AddNotExact` arm). My
  instrument anchors on the whole five-line `AddExact` block, asserts the
  anchor occurs exactly once, and refuses to run unless the file matches the
  baseline hash. It reported `identical insert lines file-wide before edit:
  [1825, 1831]` and `mutated line 1825`.
- Resulting diff, produced by me, not copied from the report:

```
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

One line, one file, the arm D105 names; `AddNotExact` still inserts
`value.clone()` at `:1831`. Line count unchanged (2289 before and after).

### 2. The three runs, reproduced

| guard | test | control | mutated (in-suite) | mutated (isolated) | restored |
|---|---|---|---|---|---|
| G1 | `apply_splices_the_simulated_scalar_for_a_bool_property` (`suggestions.rs:1037`) | ok | **ok** | **ok** | ok |
| G2 | `apply_splices_the_simulated_scalar_for_an_int_property` (`suggestions.rs:1074`) | ok | **FAILED at `:1101`** | **FAILED at `:1101`** | ok |
| G3 | `every_applied_suggestion_survives_the_next_dry_run_at_the_model_level` (`suggestions.rs:1113`) | ok | **ok** | **ok** | ok |

Aggregates I measured: control `19 passed; 0 failed`; mutated
`16 passed; 3 failed`; restored `19 passed; 0 failed`. The three mutated reds
were `apply_returns_ok_when_the_edit_reaches_the_model` (`:1219`, type
equality: `left: Some(Str("true"))` / `right: Some(Bool(true))`),
`ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions` (`:511`,
set composition), and G2 (`:1101`). Identical to the report, including the
panic messages and the pasted suggestion set. **No cell differs.**

### 4. G2's failure mode, verified twice

At the source: `suggestions.rs:1101-1104` is
`assert!(checked > 0, "no id suggestion in the fixture; this guard would pass
vacuously")`; the type-equality `assert_eq!` is `:1094-1098` and the
type-shape `assert!` is `:1088-1091`. The panic came from `:1101`, so neither
comparison was ever reached: `checked == 0`.

Measured directly (probe, isolated copy): under the mutation
`P_SUBS_BY_LANGUAGE`'s suggestion set contains **zero** `id` suggestions. The
report's central claim - the experiment measured set composition, not type
degradation - is correct.

### 6. The restore, on my own reference

`git show a8fe11f:crates/muxsmith-core/src/planner.rs` (a reference the task
could not have written) hashes to
`20cb4f58462db98220a7d1aec36dc8c1011c4e30e83b6a65392d1e1ba9a6efa7`, which is
the same baseline the report pasted, and `cmp` against the working file after
the restore reports byte identity. Both absence checks fired first on the
mutated state (` M crates/muxsmith-core/src/planner.rs` and
`1 file changed, 1 insertion(+), 1 deletion(-)`) and are empty after.

### 7. The prohibitions

- **No commit:** `git reflog` head is `a8fe11f` at `2026-07-29 00:17:03`; the
  task's control run is stamped `00:19:09`. Nothing was committed during or
  after the task.
- **No repo file:** `git status --porcelain --untracked-files=all` is empty;
  `.superpowers/` is gitignored (`.gitignore:2`), so the report is scratch, as
  the brief allows. The four house YAMLs (`docs/conventions.yaml`,
  `docs/decision-ledger.yaml`, `docs/process-conventions.yaml`,
  `docs/product-boundaries.yaml`) are unmodified by the same check, and
  `core-d49-g1g2-experiment` does not exist in any of them - the entry was not
  pre-empted.
- **No composed ledger text:** I searched the report for a proposed statement
  for the anomaly branch. There is none. It names the gap and routes it
  (`proc-latitude-clause-boundary`, which exists at
  `docs/process-conventions.yaml:326` and whose routing clause covers exactly
  this) without filling it. This is the latitude breach the brief flagged as
  most likely, and the implementer did not commit it.

### 8. The no-work-needed premises

- "Isolated per-guard runs agree with the in-suite ones": reproduced by me,
  guard for guard, on my own mutation.
- "The design's cited line span is not stale in any load-bearing way": D105
  cites the `AddExact` arm at `planner.rs:1820-1827`; the arm proper is
  `:1823-1827` and the span's first three lines are `fn`, `let mut m` and
  `match edit {`. The span contains the mutated line `:1825`. Claim holds.
  D105's other four anchors re-verified at execution state: `scalar_display`
  `:856-863`, G1 `:1037`, G2 `:1074`, G3 `:1113` - all exact.

---

## Findings

### 1. LOW - `task-7-report.md`: a `:line` citation is off by one

`.superpowers/sdd/plan-9/task-7-report.md`, "Non-guard reds" table, row 1:
`apply_returns_ok_when_the_edit_reaches_the_model` is cited at
`suggestions.rs:1211`. `:1211` is the `#[test]` attribute;
`crates/muxsmith-core/tests/suggestions.rs:1212` is the function. The same
table's second row cites `:487`, which IS the function line (the attribute is
`:486`), and the guard table cites `:1037`, `:1074`, `:1113`, all function
lines. So the convention is the function line and this one row breaks it.

Evidence: `grep -n "#\[test\]" -A1` over the file gives `1211:#[test]` /
`1212:fn apply_returns_ok_when_the_edit_reaches_the_model()`.

Required change (report only, no product file): `suggestions.rs:1211` ->
`suggestions.rs:1212`.

### 2. LOW - a mechanism INFERENCE is filed under "Verified at the source"

`task-7-report.md`, "Mechanism" section, item 4: "`id` is an Integer property,
so its simulated delta became `Str("1")`-shaped and **the candidate no longer
survives the engine's own validation**." The pasted suggestion set verifies the
`id` suggestion's ABSENCE; the reason for the absence is an inference, and the
report names no site for the engine's validation. Under a heading that reads
"Verified at the source", that is the borrowed-claim shape the rest of the
report is scrupulous about.

The inference is CORRECT, and here is the site it should have cited:
`crates/muxsmith-core/src/planner.rs:1406-1413` -

```
let candidates = candidates_for_rule(profile, ri, primaries, id, lang, SeedMode::Ambiguous);
let mut accepted: Vec<Candidate> = Vec::new();
for cand in &candidates {
    let edited = with_rule_match(profile, ri, &cand.apply);
    let sim = plan_core(&edited, run, primaries, id, lang);
    if resolves_without_regression(&sim, ri, &base_sig) {
        accepted.push(cand.clone());
    }
}
```

`cand.apply` is the delta built by the mutated `delta_for` (`:1772`, `:1801`),
so a degraded candidate fails `resolves_without_regression` here and never
reaches the guard.

Required change (report only): either cite `planner.rs:1406-1413` for the
mechanism, or move the sentence out of the "Verified" list into the
hypothesis paragraph.

### 3. INFO (not the implementer's defect; routed to the controller) - the report's one open hypothesis is now MEASURED, and it confirms the report

The report left open which suggestions G1 iterated under the mutation, and
correctly declined to conclude from it. I measured it in an isolated copy
(brief-permitted probe; no repo test file touched):

| fixture | unmutated | under the D105 mutation |
|---|---|---|
| `P_AMBIGUOUS` (G1's) | `AddExact flag_hearing_impaired Bool(true)`, `AddExact forced_track Bool(false)`, `AddExact forced_track Bool(true)` - 3 AddExact, 0 AddNotExact | `AddNotExact flag_hearing_impaired Bool(true)`, `AddNotExact forced_track Bool(false)`, `AddNotExact forced_track Bool(true)` - 0 AddExact, 3 AddNotExact |
| `P_SUBS_BY_LANGUAGE` (G2's) | `AddExact codec Str("SubRip/SRT")`, `AddExact codec Str("SubStationAlpha/ASS")`, `AddExact id Int(1)` - 1 id suggestion | `AddExact codec Str("SubRip/SRT")`, `AddExact codec Str("SubStationAlpha/ASS")`, `AddNotExact codec Str("SubRip/SRT")` - 0 id suggestions |

The report's hypothesis ("consistent with G1 having inspected only
`AddNotExact` suggestions") is **confirmed**, with one refinement it could not
have guessed: the degraded `AddExact` candidates were not merely dropped, they
were **replaced by their NOT-polarity twins**, so the set size stayed at 3 and
G1's anti-vacuity counter still reported `checked == 3`. G1 was silenced by set
composition exactly as G2 was, and its own anti-vacuity assertion could not
notice, because it counts both polarities.

Consequence for the guard, for whenever the owner rules: at HEAD G1 does cover
the `AddExact` path (3 checked candidates, measured), so it is not vacuous
today; but its anti-vacuity assertion cannot detect the coverage collapsing to
the untouched arm. If the owner ever keeps G1, counting per polarity would be
the cheap hardening. **Not a recommendation to change anything in this plan.**

### 4. MEDIUM, against the DESIGN (D105), not against Task 7 - the fenced mutation site cannot test the fenced premise

`delta_for` is called from three places in core
(`grep -rn "delta_for" crates/muxsmith-core/src/`, complete: `:1772`, `:1801`,
`:1903`, plus the definition `:1820` and two comment mentions `:1887`,
`:1917`). Two of them (`:1772`, `:1801`) build the CANDIDATE's simulated
`apply`; one (`:1903`) is the APPLIER inside `apply_suggestion`. G1 and G2
assert that the applier splices what the engine simulated. Mutating
`delta_for` moves both sides at once, and because the engine re-validates its
own candidates at `planner.rs:1406-1413`, the degraded ones are filtered out
before any guard sees them. **The instrument censors its own input.**

Measured, in the isolated copy, with the D105 mutation reverted and the same
defect class applied at the applier only (stringify the applied delta's
`exact` map after `delta_for`, engine simulation left typed):

```
---- apply_splices_the_simulated_scalar_for_a_bool_property stdout ----
panicked at crates/muxsmith-core/tests/suggestions.rs:1057:9:
assertion `left == right` failed: apply spliced Some(Str("true")) for flag_hearing_impaired; the engine simulated Bool(true)

---- apply_splices_the_simulated_scalar_for_an_int_property stdout ----
panicked at crates/muxsmith-core/tests/suggestions.rs:1094:9:
assertion `left == right` failed: apply spliced Some(Str("1")) for id; the engine simulated Int(1)

---- every_applied_suggestion_survives_the_next_dry_run_at_the_model_level stdout ----
panicked at crates/muxsmith-core/tests/suggestions.rs:1140:13:
applied suggestion AddExact { property: "flag_hearing_impaired", value: Bool(true) } over-narrowed into MissingTrack

test result: FAILED. 15 passed; 4 failed
```

All three guards red, **each through its own assertion** - G1 at `:1057`, G2 at
`:1094` (the type-equality assert, not the anti-vacuity one), G3 at `:1140`.
That is the all-fail PATTERN D105 hypothesized, obtained under a mutation D105
did not fence.

**This does NOT license recording the all-fail branch.** D105 fixes the
mutation and the statement text together, and the all-fail text asserts a
measurement under the fenced mutation that did not occur. Only the owner can
re-fence the experiment. What this measurement licenses is exactly one thing:
the anomaly is caused by the mutation SITE, not by the guards being weak, and
the controller should say so rather than leave "the premise failed" reading as
"the guards failed".

### 5. INFO (pre-existing tree defect, outside Task 7's empty Files list) - stale line citation in a test comment

`crates/muxsmith-core/tests/suggestions.rs:1015` cites "delta_for's two
exact-bearing arms, planner.rs:1812, :1817". At HEAD those arms are `:1823`
(`AddExact`) and `:1828` (`AddNotExact`); `:1812`/`:1817` fall inside
`candidates_for_rule`'s closing braces. Task 7 correctly did not touch it (its
Files list is empty). Routed to the controller as tree hygiene, to be fixed by
a task that owns the file, never as a drive-by.

---

## Adjudications

### 1. Is the anomaly branch the correct selection?

**Yes, and unambiguously.** D105 step 4 selects by which of the three named
guards went red. Observed and independently reproduced: G1 green, G2 red, G3
green.

- **All-fail branch** requires G1, G2 and G3 all red. G1 and G3 are green in
  the suite AND in isolation. Not reachable under any reading.
- **Only-G3 branch** requires G3 red and G1/G2 green. G3 is green and G2 is
  red: the observed pattern is the inverse on both members. The only reading
  that could reach it is a purposive one ("the branch is really about G1/G2
  not adding detection over G3"), and it fails on its own terms too, because
  G3 detected nothing either. Reaching for it would also be the exact move
  D105's anomaly branch was written to forbid, since it ends in a
  removal-candidate record for G1/G2.
- **Anomaly branch:** its condition is "any other outcome", which is met, and
  the design's own worked example of one - "e.g. G3 passes under the mutation"
  - is literally what happened.

The implementer applied the rule as written and did not narrate its way to a
clean branch. That is the behaviour the branch exists to produce.

### 2. Does the experiment, as designed, test what D105 says it tests?

**No. The design's premise does not survive, and the report's argument for
that is sound - I strengthened it from an argument into a measurement.**

D105's premise is that re-stringifying the `AddExact` delta presents G1/G2/G3
with a type-degradation. It does not, for two compounding reasons:

1. **Identity on strings.** `scalar_display` (`planner.rs:856-863`) maps
   `Scalar::Str(s) -> s.clone()`, so for every string-valued property the
   mutation is a no-op by construction. Confirmed empirically: the `codec`
   suggestions survive the mutation unchanged in G2's fixture.
2. **Self-censoring on non-strings.** For `Bool`/`Int`/`Float` the mutation
   degrades the candidate's own simulated `apply`, so the candidate fails
   `resolves_without_regression` at `planner.rs:1406-1413` and is replaced by
   its NOT-polarity twin (measured, finding 3). The guards therefore never
   receive a degraded value to compare.

The net effect is that the only test which reached a type comparison,
`apply_returns_ok_when_the_edit_reaches_the_model` (`:1212`, panic `:1219`),
is the one that constructs its edit by hand instead of taking it from the
engine - precisely as the report says.

The premise IS testable; it just needs the applier-site mutation (finding 4),
under which all three guards go red through their own assertions. So the
honest characterisation for the close is: **the experiment was inconclusive
because of its instrument, not because the guards are weak.**

### 3. Is the unverified G1 hypothesis material?

**The branch selection does not depend on it, and the plan close does not need
it - but it is now measured anyway, and the close should carry the result.**

- **Branch selection:** independent. D105 selects on red/green per named
  guard. Why G1 stayed green changes no cell.
- **Report discipline:** the separation held. The report's Step 4 derives the
  branch from the pattern alone, the mechanism section is explicitly labelled
  context, and "Surfaced" item 4 states in terms that it draws no conclusion
  about G1/G2 being load-bearing. Nothing load-bearing rests on the
  hypothesis. That is the correct handling of an untested link, and the
  implementer declining to ask for latitude to test it was also correct: Step
  2 says "Nothing else is touched" and the Files list is empty.
- **Now measured** (finding 3): the hypothesis was right, and the refinement
  (polarity substitution, so the anti-vacuity counter stays satisfied) is what
  makes the anomaly legible. Without it, "G1 stayed green" invites the reading
  "G1 is weak". With it, the reading is "G1 was never shown the degraded
  input". That distinction is decision-relevant for whatever the owner later
  rules about G1/G2, so it belongs in the close even though it was not needed
  to select the branch.

### 4. Was returning NEEDS_CONTEXT right?

**Yes; it was mandatory, and the task could not have completed by recording
the anomaly itself.**

D105 step 4 for the anomaly branch: "no removal in any direction, the anomaly
is recorded and routed to the controller as NEEDS_CONTEXT." The plan's Task 7
Step 4 repeats it. The status word is design-fixed for this branch, not the
implementer's judgment call.

What the task MAY write: its report in the SDD scratch, with the runs, the
per-guard table, the selected branch and what it surfaces. It did exactly
that.

What the task may NOT write: the `core-d49-g1g2-experiment` ledger entry (plan
Task 7 Files: none; "This task writes NO ledger entry and NO repo file"), the
ROADMAP trigger updates (controller close actions), and - the sharp one - any
statement TEXT for a branch the design fixed no text for. D105 fixes two texts
"selected by the measurement (no implementer wording)". With the anomaly
measured, there is no text to select, and composing one would have been a
latitude breach. The implementer named the gap and routed it instead. Correct.

One reading note for the controller: NEEDS_CONTEXT here means "a fork is open",
not "the task is unfinished". Steps 1-6 were all executed and all six are
reproducible.

### 5. Is the restore genuinely complete?

**Yes.** On my own reference, not the report's baseline:
`git show a8fe11f:crates/muxsmith-core/src/planner.rs` written to my instrument
directory, `cmp` against the working file after the restore reports byte
identity, and the file's sha256 is
`20cb4f58462db98220a7d1aec36dc8c1011c4e30e83b6a65392d1e1ba9a6efa7`. Both
absence checks are empty and both were fired non-empty on the mutated state by
me, not merely quoted from the report. `git status --porcelain -uall` is empty,
so nothing was left behind anywhere in the tree, tracked or untracked. HEAD is
`a8fe11f` and the reflog shows no commit after the task began. The restored
suite is `19 passed; 0 failed`, the same 19 as the control including the three
step-3 reds.

---

## Evidence appendix

All instruments are mine, written for this review, under
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t7rev-independent/`:

| path | what it is |
|---|---|
| `planner.rs.a8fe11f` | independent baseline, `git show a8fe11f:crates/muxsmith-core/src/planner.rs` |
| `mutate.py` | my mutation instrument: refuses unless the file matches the baseline hash and the five-line `AddExact` block occurs exactly once; reports the mutated line number and every identical insert line file-wide |
| `fire-nonascii.txt` | fire for the typography absence check (`grep -nP '[^\x00-\x7F]'` returns a hit on this file, nothing on the report) |
| `iso/` | isolated source copy of `a8fe11f` (`git archive a8fe11f \| tar -x`), used for both probes; never the repo |
| `iso/crates/muxsmith-core/tests/probe.rs` | my probe test (copy of `suggestions.rs` plus one printing test), isolated copy ONLY - no repo test file was edited |
| `iso-target/` | separate `CARGO_TARGET_DIR` for the isolated builds |

Runs performed in the repo, all foreground, all on `master` in the main
worktree: green control; mutated suite; three isolated per-guard runs;
restored suite. Runs performed in the isolated copy: probe unmutated; probe
under the D105 mutation; full suite under the applier-site variant (both the
broad and the `exact`-only form, same result).

Absence checks I made fire before trusting: `git status --porcelain` and
`git diff --stat` (both shown non-empty on the mutated tree), and the
non-ASCII scan (fired on a purpose-built file). The `delta_for` enumeration is
a positive grep, not an absence check.

---

## HARVEST

### For the plan close - how to record an outcome the design fixed no text for

This is the open fork and the reason the task returned NEEDS_CONTEXT. My
recommendation, with the tradeoff named:

1. **Write `core-d49-g1g2-experiment` at the close, with a controller-composed,
   strictly factual statement, and mark it as such.** D105 step 4 requires the
   anomaly to be RECORDED; D105 step 6 supplies text for two branches that were
   not measured. Deferring the entry entirely would satisfy step 6 by leaving
   step 4 unexecuted, and the measurement would live only in gitignored
   scratch. The statement should carry only what was measured (mutation site,
   the three guard results, the aggregate, and that the premise failed because
   the mutated function also feeds candidate construction), assert nothing
   about whether G1/G2 are load-bearing, and say in the entry itself that it is
   not one of D105's two fixed texts. Tradeoff: the controller is composing
   doctrine text the design meant to fix, which is exactly what D105's
   "no implementer wording" clause distrusted - so it should be flagged to the
   owner at the close rather than absorbed silently.
2. **The D105 text gap is a design defect worth naming.** A decision rule with
   three branches supplied fixed text for two and mandated recording for the
   third. Whoever writes the next experiment protocol fixes text for the
   anomaly branch too, or states explicitly that the anomaly branch's record is
   controller-composed. This generalises beyond D105.
3. **ROADMAP `docs/ROADMAP.md:750-756`** (the consumed D49 trigger) names only
   the two clean branches. Its close update must record: FIRED and consumed by
   Plan 9 Task 7, outcome = anomaly, experiment inconclusive because the
   mutation site is shared between the engine's candidate simulation and the
   applier, guards stay per `proc-proposed-safeguard-stays`.
4. **ROADMAP `docs/ROADMAP.md:728-731`** ("Plan-9 design trigger 4") stays
   registered and NOT fired - its condition is the only-G3 branch, verbatim,
   and the implementer's reading is correct. But note that its condition is now
   unreachable without a re-fenced experiment, so it is a trigger that can only
   fire on a protocol nobody has scheduled. Consider re-aiming it at the
   re-fenced experiment instead of leaving it dormant against a dead condition.
5. **The re-fenced experiment, if the owner wants one:** mutate the APPLIER's
   delta only (`planner.rs:1903` site), leaving `:1772`/`:1801` typed.
   Measured in the isolated copy: G1, G2 and G3 all red, each through its own
   assertion. Offered as a candidate protocol for an owner ruling, NOT as
   D105's result and NOT as grounds to record the all-fail text.

### Ledger candidates

1. **A mutation experiment must mutate a site that ONLY the guard's subject
   reads.** If the mutated function also feeds the fixture generator, the
   degraded input is filtered out before the guard sees it and the guard passes
   for a reason unrelated to its strength - the instrument censors its own
   input, and the result is uninterpretable in both directions. The trigger is
   readable at design time: you are fencing a mutation, so enumerate the
   mutation target's call sites and check whether any of them is upstream of the
   fixture the guard consumes. Here `delta_for` had three call sites, two of
   them upstream of the suggestion set, and D105 fenced the shared function
   instead of the applier's call site. Complements
   `proc-check-green-state-reachable` (that one demands the check's GREEN state
   be reachable; this one demands the guard's RED state be reachable under the
   experiment's own instrument) and `proc-verification-step-must-be-falsifiable`.
2. **An anti-vacuity counter must count the dimension the guard is about.**
   G1's `assert!(checked > 0, ...)` counts `AddExact` and `AddNotExact` hits
   together, so when the mutation replaced all three `AddExact` candidates with
   NOT-polarity twins, coverage of the guarded arm went to zero while the
   counter still read 3. Measured, not argued (finding 3). Generalises: a
   vacuity guard keyed on "did I check anything" cannot see coverage collapsing
   onto a sibling case.
3. **Worth an occurrence, not a new entry, on `proc-proposed-safeguard-stays`:**
   this is the entry's own scenario running to completion for the first time -
   a design-phase vacuity claim was re-aimed onto a measurement, the
   measurement was run, and it came back inconclusive for an instrument reason.
   The lesson the entry already carries ("keep the safeguard until it is
   MEASURED redundant") held: the guards stay, and the plan-6 vacuity analysis
   is neither confirmed nor refuted.

### Tree hygiene, routed

`crates/muxsmith-core/tests/suggestions.rs:1015` cites `planner.rs:1812, :1817`
for `delta_for`'s two exact-bearing arms; they are at `:1823` and `:1828`.
Pre-existing, not Task 7's to fix.

### Review-minors for the roll-up funnel

Findings 1 and 2 (the off-by-one `:line` and the inference filed under
"Verified at the source"). Both report-local, neither affecting the
measurement.
