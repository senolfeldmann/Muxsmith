# Amendment 1, fix round 1: delta verdict

**All 8 findings ADDRESSED.** Both new-content items are SOUND. **New breakage: 0
Critical, 0 Important** (4 Minor observations, none of which extends the loop).

Instruments for this round were built fresh at
`/tmp/.../scratchpad/delta-pruefstand-r2/`, a path neither the author nor my
round-1 self used. The lint probe was rebuilt from scratch rather than reused: I
extracted the fenced rule from the plan with `awk` and executed **those
characters**, inserted into a copy of the repo's real `eslint.config.js` at the
prescribed position, rather than a retyped equivalent.

---

## 1. Per-finding verdicts

**I-1 (the universal claim its own task falsifies): ADDRESSED.** Both documents now
read "No RENDER gate in the view - no `v-if`, no `v-else-if` - reads a bare
`!model` afterwards", and both name why the `:disabled` bindings (D108 decision
10) and `saveDisabled` (D107 decision 3a) are deliberately outside that reach.
Measured: `grep -nE 'No (other )?gate in the view reads'` over both documents at
HEAD returns nothing, exit 1; the same expression against the two files at
round-1 head returns 1 each - my own fire control, not the author's. The narrowing
went in the direction the finding asked for: the claim was scoped to the check,
not the check widened to the claim. The two citations the new sentence adds both
resolve identically in both documents (D108 decision 10: ten items in each, same
text; D107 decision 3: same clause with the same (a) sub-item).

**I-2 (the no-work-needed premise about the toolchain): ADDRESSED, and the fix
exceeds the finding.** The assertion is not merely retracted - the guard is
prescribed in Task 4 per the controller ruling, and the abandoned position is
preserved as a rejected alternative carrying its steelman and the general handle.
I re-derived the whole measurement independently (section 3 below): red, green,
the over-match control and the per-member selector fire all reproduce against the
plan's own fenced text, plus one scope check the plan does not make and I did.

**M-1 (Read-first pointed at a locator not in the named file): ADDRESSED.** Task 4's
`Read first` now names `editor-recent-profile` as the only one of the two testids
any spec uses, states that `editor-recents` appears nowhere under `e2e/`, and
points at `src/views/EditorView.vue`'s template where both are defined. That
matches my round-1 measurement exactly.

**M-2 (report described the artifact wrongly; fences carried authored text):
ADDRESSED in substance, with a defect in the correction's own form** - see
observation O-2. The correction is accurate: the Step-6-to-Step-7 range does
contain zero fenced blocks, the specificity itself matches the plan's house level,
and this round's own fences carry only reproducible text (I re-extracted one and
ran it).

**M-3 (W2-p's observable broader than its producer): ADDRESSED, and measured rather
than assumed.** The row is rewritten to "No render gate in the editor asks `!model`
directly", names both producers, and states the one class outside both. I
reproduced the resolution itself: against a fixture whose gate reads
`v-if="currentPath === null && !model"`, the rule reports it (1 error, 637:38)
while `grep -cE 'v-if="!model'` returns 0. The term-order hole is genuinely closed
by the rule. One narrower reach the row still overstates is O-1.

**M-4 (concern 2 under-enumerated the tree): ADDRESSED.** The corrected figure is
right and its pasted grep matches mine character for character: three assignments
at `openPath`, `createBlank` (the `null`) and `doSave`, so **two** new write sites
falsify the ROADMAP sentence. The report states explicitly that the close action
takes this figure and not the original.

**M-5 (concern 1 over-enumerated): ADDRESSED, and better than asked.** The
corrected enumeration is not only in the report; it is in the plan at Task 5 Step
4b, where it is load-bearing rather than archival. I re-derived it independently
(section 2) and got the same two.

**M-6 (the residual registered nowhere that would announce it): ADDRESSED.** The
residual the finding was about no longer exists - `pnpm lint` now turns red and
names file and line. D112 carries a `Triggers created` slot in the ADR, in the
same form D110 uses for its own surfaced gap, and the plan close's known-inputs
list names the new standing guard as a check class the verdict harvest should see.
Measured at the artifact rather than taken from the report: the slot is present,
its single bullet states the self-firing property, and the close-list sentence is
in the diff.

---

## 2. New content 1: the authorized scope extension (Task 5 Step 4b)

**Judgment: SOUND.** It is new plan content of the right shape, and it works.

**The enumeration holds.** I walked every Task-4 case against the stated criterion
(builds history, so `dirty` is true under D108 decision 4's formula, then performs
a second Open through `pickAndOpen`) from the plan's own step text, not from the
author's table:

- Step 5's six mutation-path cases: "open a profile ... perform the mutation" -
  the open precedes the mutation, no second open. Not affected.
- Granularity: types and clicks Add, no second Open. Not affected.
- Truncation: "undo once, then edit; Redo is disabled" - no Open at all. Not
  affected, which is the M-5 correction standing up.
- Save marks rather than clears: "open, edit, Save (mocked)" - ends at the Save.
  Not affected.
- **Open resets**: "with history built, open another profile" - **affected**.
- **A failed open clears rather than keeps**: "with history built and Undo
  enabled, open a second path" - **affected**.
- Depth cap: Add repeated, no second Open. U1: no Open. Mount-harness property:
  another file, no IPC. Amendment 1's D112 case: opens twice, never edits, so
  `savedSnapshot === history[position]` at both clicks and `dirty` is false.

Exactly two. I also checked the criterion's own blind spots, because `createBlank`
gains the same guard in Task 5: no Task-4 case clicks New at all, and the
neighbouring files are clean too - `editor-rule-add-remove.spec.ts` is bare-mount
with no `pickAndOpen`, and Task 3's smoke cases each reach their Open or New on a
clean editor (`savedSnapshot` is null before the first load, so `dirty` is false).

**The repair works once the guard exists.** Task 5 Step 2 prescribes
`if (dirty.value && !(await confirmEl.value?.ask())) { return; }` ahead of the open
dialog. Asserting `confirm-dialog` visible and then clicking
`confirm-dialog-confirm` resolves `ask()` to `true`, the negation is false, and
`pickAndOpen` proceeds to the mocked `plugin:dialog|open`. Both testids are the
ones Step 1 prescribes on the component, so nothing is invented.

**No assertion is weakened.** The insertion sits between the click and the
assertions, so the failed-open case's ordered list (diagnostic renders, Undo and
Redo disabled, editing surface gone) and its in-test control (Undo was enabled
before the failed open) are untouched, and both cases still exercise a genuinely
dirty editor - which is the state a user reaches.

**The rejected alternative's mechanism is accurate; its steelman is not a
steelman.** Saving before the second Open would indeed leave the history intact
and `dirty` false (D108 decision 3: saving marks the written profile and does not
clear the history), so the guard would never fire - the claim checks out, and the
rejection ground is right: it swaps the case's subject to dodge the mechanism. But
the passage states the alternative and rejects it in one clause without giving it
its strongest form, which here is real: it would keep the seven Files lists
disjoint and spare Task 5 from writing a spec file it does not otherwise own. That
is O-3, Minor.

**The rest checks out.** The house-pattern ground is true - Task 3's Files list
already carries "the two doc-comment regions this task's own change falsifies" and
Task 4's carries `e2e/editor-rule-add-remove.spec.ts` for the header sentence this
package falsified. The no-new-row claim is true: W4-a, W4-b and W4-c already grade
the guard's ordering, its confirm leg and its cancel leg, so the repair keeps two
producers alive rather than producing an observable. Task 5's Files list, its
commit pathspec and its "exactly the six files" sentence are mutually consistent
at six (measured). And `pnpm test:e2e` is a gate part (`BUILDING.md`), so the
step's claim that a missed repair shows up as a red gate at Task 5 holds.

---

## 3. New content 2: the standing lint guard (Task 4 Step 4c)

**Judgment: SOUND. The fenced text is the text that works, and I measured that
directly rather than inferring it.**

Method, aimed at the gap the controller named. I extracted the fenced block from
the plan with `awk` (18 lines, comment plus rule), then built a probe config that
is `eslint.config.js` **verbatim** with only the four imports absolutised and that
extracted block inserted at the prescribed position - first in the existing
`**/*.vue` rules block, immediately above `@intlify/vue-i18n/no-raw-text`. The
config parsed and ran, which by itself retires the "does the fenced JS drop into
that slot" risk.

```
RED, the plan's fenced rule against src/views/EditorView.vue as Task 3 left it:
  637:14  error  A render gate must not read `!model` directly: the pre-session
                 state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax
  644:14  error  (same)                                    vue/no-restricted-syntax
-> exactly 2 vue/no-restricted-syntax errors, the two gates. Matches the
   prescribed figure.

GREEN, the same config against Task 4's end state (both gates rewritten to
nothingOpenedOrCreated, PLUS two :disabled="!model || !can*" bindings present
as the over-match control):
exit=0, no output.

SELECTOR SET, member 2 alone (the file's existing v-else-if pointed at !model):
  631:19  error  ...  vue/no-restricted-syntax        -> the v-else-if branch fires.

M-3's term-order case (v-if="currentPath === null && !model"):
  637:38  error  ...  vue/no-restricted-syntax        -> the rule sees it,
  grep -cE 'v-if="!model' -> 0                        -> P1's grep does not.
```

Every prescribed figure is reachable against the tree as Task 3 left it, the red
state defeats the mechanism rather than disturbing its input (there is no fallback
between the gate and the assertion - the rule reads the parsed template directly),
and the over-match control is genuine: the selector's directive-name scoping is
what keeps it and D108 decision 10 mutually satisfiable, and the green run with
the bindings present is what proves that rather than asserting it.

**One scope check the plan asserts and does not measure, which I closed.** Step 7
now binds the rule's green state to the gate's own run ("`pnpm lint` is inside the
gate below, so the gate's own green run is the rule's green run"), and `pnpm lint`
is `eslint .` over the whole repo - while every figure in Step 4c is scoped to one
file. If any other component carried such a gate, the gate would be permanently
red and the green claim false (`proc-check-green-state-reachable`). Measured: the
rule over **all 24 tracked `.vue` files** produces exactly those 2 hits, both in
`EditorView.vue`, and nothing anywhere else. The claim holds; it was reasoned, not
run.

**Latitude: nothing to invent.** The fence carries the comment, the selector and
the message; the placement is named by its neighbour rule; the red state is fenced
by the two gates rather than by line numbers, with the restore-and-re-apply
ordering stated because Step 4b runs first; the `v-else-if` fire names an existing
directive in the file (there is exactly one). The `Must not decide` list gains the
rule's scope and shape. I swept the round's 70 added lines for both latitude forms
and found none, with a fired control.

**No-work-needed claims in the new text, checked rather than weighed.** "No gate
part is added" - `package.json` line 11 is `"lint": "eslint ."` and `BUILDING.md`
line 113 lists `pnpm lint` as a gate step: true. "The two tooling files this
package edits" - the seven Files lists contain exactly two tooling/config files,
`scripts/check-i18n.mjs` and `eslint.config.js`: true, derived from the lists.
"No check in this repo detects [the duplicate-under-another-name] class for any
derived value" - the repo's check surface is `check-i18n.mjs`,
`check-version-sync.sh`, `ledger-lint.py`, eslint, clippy and the test suites;
none does semantic duplicate detection, so the depth is house-consistent as
claimed.

---

## 4. The three sweep-driven corrections, verified independently

**Task 4's file count, 6 to 7.** I parsed all seven Files lists and all seven `git
add` pathspecs out of the plan and compared them pairwise: **identical for every
task**, counts `2, 6, 4, 7, 6, 9, 4` - the same figures the report states. Task 4's
Step 7 diff-scope sentence now says seven and Task 5's says six, both matching.

**The sequencing file-overlap enumeration.** I derived the complete set of files
written by more than one task from the Files lists rather than checking the
sentence's members: `src/views/EditorView.vue` {3,4,5,6}, `e2e/smoke.spec.ts`
{3,4,5,6}, `locales/en/gui-editor.ftl` {3,4,5}, `locales/de/gui-editor.ftl`
{3,4,5}, `e2e/editor-undo-redo.spec.ts` {4,5}. That is exactly the set the
corrected sentence names, with exactly the right task sets, and there is no sixth
overlap it omits. The pre-existing understatement was real (three and two, where
both are four) and Task 6 is the task both lists were missing.

**The gate-part-count audit.** Running the plan's own expression over the plan
returns **1** line, and it is the audit's own sentence. Each alternative fires
alone (`11 parts` -> 1, `an eleven-part gate` -> 1, `a 6-part block` -> 1) and the
negative control returns 0. The paragraph's new history claim is consistent with
the artifact: Step 4c's falsifiability sentence now names the three things
("the expression, a red state with its exact expected non-zero result, and the
green end state") instead of counting them, which is what the claim says the
rewording did.

**The eight ordinal renames.** End state verified: `grep -rnE "D112('s)?
(decision|dec\.?) *[0-9]"` over both documents and `docs/ROADMAP.md` returns
nothing, exit 1, with my own fired control against a synthetic. The divergence
that motivated the policy is real and I measured it: the plan register's D112
carries **9** numbered items, the ADR's carries **7**, so an ordinal into that
list resolves differently per document. The two surviving named references
("D112's standing-guard decision") resolve in both - plan item 9 and ADR item 7
carry the same statement. **The figure "eight" itself is not verifiable**: no
ordinal cross-reference into D112 existed at the round-1 head, so the eight were
renamed inside an uncommitted draft. See O-4.

---

## 5. Standing dimensions

- **Typography:** 0 hits over this round's 70 added lines with my own character
  class, and my synthetic control returns 1.
- **Counts:** 43 requirement rows, 73 acceptance rows, 0 duplicate ids - unchanged,
  recomputed from the tables.
- **House rules:** no self line-number citation in the added lines (fired control);
  the ADR's new `Triggers created` slot follows D110's form; the D112 edits are a
  same-round correction of an entry no task has consumed, not a reversal of a
  recorded decision, and the abandoned position is preserved as a rejected
  alternative with its steelman and the general handle - which serves the
  reconstructibility `proc-supersede-never-overwrite` exists to protect. The
  proposed-safeguard rule is honoured: P1 is kept beside the rule rather than
  replaced by it, and both are added to the do-not-argue-away list.
- **Commit hygiene:** exactly the two tracked documents, unsigned (`%G?` = `N`),
  one trailer.
- **Claimed vs prescribed:** everything reproducible against the tree today
  reproduced. Everything prescribed for a future implementer is graded above as a
  design against its specification, and I have not reported agreement with any run
  I could not perform.

---

## 6. New observations (Minor; none extends the loop)

**O-1. `v-show` is the one render directive outside the rule, and the prose does
not say so.** Measured: `v-show="!model"` produces 0 hits from the rule and 0 from
P1's grep. D112 decision 1 is safe because it enumerates ("no `v-if`, no
`v-else-if`"), but W2-p's row says "No render gate in the editor asks `!model`
directly" without that scoping, and the uncovered paragraph names only the
different-NAME escape while claiming "spacing, term order and a differently-spelled
duplicate cannot slip past it" - a different DIRECTIVE is a fourth class it does
not name. This is the I-1 shape surviving one level down, in the paragraph
describing the fix for it. It does not block: behaviourally a `v-show` rewrite
leaves the node in the DOM, so P2's leg-3 count of 0 fails and the test catches it.
One clause fixes the prose, or one alternation member (`^(if|else-if|show)$`) fixes
the rule.

**O-2. The report's in-place corrections are not in place.** The report describes
its own form as "the wrong sentence stays and is marked, with its correction beside
it". The wrong sentences do stay - section 6's "The fenced mock set is complete",
its summary-table row repeating "fenced mock set", concern 1's "three cases" and
concern 2's single write site - but none carries a marker at its own location, and
each correction sits several hundred lines later. A reader landing on the original
reads it as true. The house rule the round invokes elsewhere
(`proc-supersede-never-overwrite`) requires the superseded text to keep its
statement **with a pointer**; the pointer is what is missing.

**O-3. The rejected repair alternative is stated, not steelmanned.** Its strongest
argument - disjoint Files lists, no second task writing a spec file it does not own
- is the one not given.

**O-4. A count over an intermediate draft cannot be checked.** "Eight ordinal
cross-references renamed" describes edits made and undone inside an uncommitted
draft; no reader can open anything that would falsify it. The end state is
verifiable and I verified it. Report figures are worth more when their denominator
is something on disk.

---

## 7. Harvest

**The round's dominant pattern, and it is the round-1 pattern one notch smaller.**
The author now measures what it prescribes - every figure in Step 4c reproduced
against the plan's own fenced text, and the two counts it corrected (the affected
Task-4 cases, the file overlaps) it re-derived from the artifact rather than from
either count offered to it, landing on mine in one case and refuting its own in the
other. What survives is the reach problem: a sentence describing an instrument
still runs one notch wider than the instrument's own enumeration. I-1 fixed that at
the ADR sentence; O-1 is the same defect in the row and the uncovered paragraph
that describe the fix.

**The reusable handle, because it is mechanical rather than a matter of care:**
when an instrument's configuration contains an **enumerated set** - two directive
names in a selector, two file extensions in a grep, three keywords in an
alternation - every sentence that describes that instrument's reach either carries
the same enumeration or is explicitly scoped by it. The trigger is readable at the
keyboard: you are writing a prose noun ("render gate", "source file", "gate part")
where the instrument has a list. The check is to read the list out of the
configuration and ask which members of the noun are not in it.

**The model instance this round produced, worth keeping.** D112's new rejected
alternative is the best-formed one in the package: it states the abandoned position
at full strength, names **which clause** of its own steelman was false, says the
clause was refuted by running it rather than by reasoning, and generalises to a
handle ("a no-work-needed conclusion whose enabling premise is a claim about what
the toolchain can or cannot do is settled by invoking the toolchain"). That is the
shape a reversal record should take.

**Second harvest item, on scope and instrument.** Step 7 binds the lint rule's
green state to a repo-wide gate run while every figure supporting it was taken
against one file. The conclusion survived - I ran it over all 24 `.vue` files - but
the pattern is worth naming: when a check's red state is measured at file scope and
its green state is asserted at the scope the gate actually runs, the green state
has not been measured at all. It is one command to close.

**Observation carried forward, outside this diff.** The validation-response race
(`watch(model)` does not increment `validationGeneration` on the `undefined`
branch, so an in-flight `validate_profile_model` can overwrite the parse
diagnostic after a failed load) remains a real, pre-existing product defect in
shipped code. The report now carries it as a controller item, which is the right
routing for a documents-only round; it still needs one.
