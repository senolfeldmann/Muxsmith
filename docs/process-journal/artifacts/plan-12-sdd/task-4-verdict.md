# Task 4 verdict

Independent review of `.superpowers/sdd/plan-12/task-4-brief.md` against
`.superpowers/sdd/plan-12/review-85902c7..1092eb7.diff` (commit `1092eb7`,
base `85902c7`) and `.superpowers/sdd/plan-12/task-4-report.md`. All checks
below were re-run with my own instruments, either directly against the
tracked repository at `/home/senol/Git/Muxsmith` (read-only throughout) or
against a full filesystem copy at
`/tmp/claude-1000/-home-senol-agents-peter/a1386daa-bdbc-4366-b18d-375daf90cf89/scratchpad/muxsmith-copy`
(excludes `target/` and `.git/`, which `pnpm test:e2e` does not need) for
every check that required a mutated file. Every mutation in that copy was
restored immediately after its test run and the restore was verified by
`diff` against a saved original, never by exit code alone. The tracked repo
was confirmed clean (`git status --short`, `git diff --stat`) after every
mutation round.

## Verdict 1: spec compliance

**MET**, requirement by requirement, Steps 1 through 8. Every fenced code
block, fenced string, fenced selector and fenced message in the brief was
extracted programmatically from the brief and matched byte-for-byte against
the shipped file (push rule, `nothingOpenedOrCreated`, the eslint entry, the
two template gate expressions, the `doSave` mark line, both `.ftl` fences).
The commit message, trailer and file scope match the brief's Step 8 block
exactly. The catalog recount (49 -> 51), `pnpm check:i18n` (218 ids, 0
unused), `pnpm lint` (clean) and `pnpm test:e2e` (93/93) were independently
re-run against the tracked repository and reproduce the report's figures
exactly. `git diff --stat` covers exactly the seven Files-list paths.

One premise inside Step 7 does not reproduce (D1's broad pre-state figure,
see Q2 below) but the requirement it supports (R28, no second dirty-tracking
mechanism) is independently carried by the narrow acceptance expression,
which I re-verified end-to-end (0 on the real end state, both alternation
members fire against synthetic probes). This is graded as a documentation/
premise finding, not a spec-compliance failure.

## Verdict 2: task quality

**5 findings**: 1 high, 2 moderate, 2 low. None blocks acceptance of the
shipped code; two (findings 1 and 2) materially change what a reviewer of
Tasks 5-6 should assume Task 4 already validated.

### Finding 1 (high) - the residual paragraph understates the actual gap

**File:** `src/views/EditorView.vue` (`doSave`, the `savedSnapshot.value =
JSON.stringify(profile);` line) and the brief's own Step 7 "behavioural gap"
paragraph.

**Summary:** the brief's residual note frames the untested part of D108
decision 3 narrowly - "a test that moves the model inside one of `doSave`'s
two `await` windows" - implying the ordinary, non-racing case (does `doSave`
mark `savedSnapshot` at all, correctly) is otherwise covered. It is not.

**Failure scenario, reproduced:** in the scratch copy, deleting the entire
line `savedSnapshot.value = JSON.stringify(profile);` from `doSave` (not
just corrupting its right-hand side - removing it outright) still leaves
`pnpm build` green and all 93 e2e tests green, including the case named
"save marks rather than clears". The reason: `dirty` is the only consumer of
`savedSnapshot`'s string value (as opposed to its nullness), and `dirty` has
zero consumers anywhere in this task's shipped code or tests - the spec
file's own header says so explicitly ("`dirty`, consumed by Tasks 5-6, not
asserted here"). "Save marks rather than clears" only asserts Undo stays
enabled and one Undo restores the pre-edit state, both of which are
independent of whether `savedSnapshot` is ever written. The only instrument
that would catch a full omission is S1 - and S1 is a one-shot grep pasted
into the report, not a standing gate check, and it only matches two specific
wrong spellings, not an absent line or a wrong value in another shape.

This directly instantiates the house rule
`a-normative-claim-is-scoped-down-to-its-producers-reach`
(`docs/decision-ledger.yaml`): the paragraph's claim ("the fix makes the
property structural, S1 pins the structure") has broader reach than what S1
actually produces.

**Consequence for the plan:** Task 5/6's reviewer must not treat Task 4 as
having validated any part of the save-marking mechanism dynamically, racy or
not. Their own tests are the first real producers for this property.

### Finding 2 (moderate) - "a failed open clears" is satisfied by a fallback, not by the property it names

**File:** `e2e/editor-undo-redo.spec.ts` (`a failed open clears rather than
keeps`), `src/views/EditorView.vue` (button `:disabled="!model ||
!canUndo"` / `!canRedo"`).

**Summary:** the test's Undo/Redo-disabled assertions are meant to show
D108 decision 9 clears `history`/`position` on a failed load. They cannot
show that, because the buttons' `:disabled` binding is satisfied by the
`!model` term alone once a failed load clears `model.value`, independent of
`canUndo`'s real value.

**Failure scenario, reproduced:** in the scratch copy, mutating
`resetHistory`'s `profile === undefined` branch to leave `history`/`position`
untouched (while still nulling `savedSnapshot`) does not fail this test - it
still passes. This is the review brief's own named "structural trap"
(a fallback between a mutation and an assertion) found empirically, in a
case the brief did not name.

On inspection this specific defect shape appears to have no other reachable
observation path today: `undo()`/`redo()` carry their own `!model.value`
guard (D108 decision 10), so nothing downstream can read the leaked state
while `model` stays falsy, and the next `resetHistory` call (from any
funnel) unconditionally overwrites it. So the practical risk is low, but the
test's name and the brief's own "Assert... Undo and Redo are both disabled"
instruction both read as verifying more than they do.

### Finding 3 (low-moderate) - createBlank's own reset is unexercised by any Task-4 assertion

**File:** `e2e/editor-undo-redo.spec.ts`, `src/views/EditorView.vue`
(`createBlank`).

**Summary:** the re-derived mutation-path sweep found an eighth
whole-value-assignment site, `createBlank`'s own `model.value =
blankProfile();`, and the report classifies it as excluded from the
six-function mutation table on the same "session-start funnel" ground as
`openPath`. That classification is correct (see Q1 below) - but no test in
this diff exercises `createBlank` together with undo/redo state at all, not
even the simple case (click New, assert Undo disabled), let alone the
asymmetric case the "open resets" test covers for `openPath` (build history,
then reset via the OTHER funnel). Reproduced: deleting `resetHistory(profile)`
from `createBlank` entirely is caught only by Task 3's own pre-existing
regression tests in `smoke.spec.ts` (which test validation-on-New, not
undo/redo state) - zero of Task 4's own 14 new tests fail.

Not a spec-compliance defect: Step 6 does not enumerate a "create resets"
case. Recorded as a genuine, if minor, coverage asymmetry between the two
funnels D108 decision 8 governs identically.

### Finding 4 (moderate) - D1's cited pre-state figure does not reproduce against the task's actual base commit

**File:** `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`
(source of the "already measured: 0 lines" claim, carried into
`task-4-brief.md` Step 7) and `task-4-report.md`'s own recount.

**Summary:** see Q2 below for the full analysis; the figure is 3, not 0
(brief) and not 2 (report's own recount, which missed one of the three
hits). The narrow, actually-gating expression is unaffected.

### Finding 5 (low) - the implementer's own D1 recount is itself off by one

**File:** `task-4-report.md`, the "Divergence found and reported" paragraph.

**Summary:** the report states the broad D1 expression "returns 2 lines,
both hits on the substring 'unsaved'" on the Task-3 baseline. My own run of
the identical expression against `85902c7` (`git show
85902c7:src/views/EditorView.vue | command grep -nEi
"dirty|isDirty|unsaved|modified"`) returns 3 lines: the two the report
found (the `data-testid`/`$t` call) plus a third, a doc comment at line 69
that also contains the substring `editor-unsaved`. This does not change the
report's correct qualitative conclusion (not a second dirty-tracking
mechanism) or the acceptance gate (the narrow expression, unaffected), but
it is a factual inaccuracy carried into a delivered artifact - exactly the
class `feedback_zitat_und_zahl_pruefen`-style scrutiny exists to catch, one
door down from this repo's own house convention.

## Adjudication answers

**Q1 (the eighth mutation-path site).** The classification is correct at the
code: `createBlank` calls `resetHistory(profile)` synchronously, immediately
before `model.value = profile`, with no `await` between them (identical
shape to `openPath`), so `history[0]` already equals the serialized model by
the time the queued `watch(model)` callback runs - the write cannot reach
the push rule under any reachable state. I independently re-ran both
authoring expressions against the actual base commit `85902c7` and got the
same 8-line/8-function result the report cites, with the same six-function
mutation set. The six-function set is still exhaustive and still correct.
The one gap this reclassification leaves is Finding 3 above (untested, not
miscategorized).

**Q2 (D1's broad pre-state figure).** The figure is **3**, not 0 (brief) and
not 2 (report). I ran `git show 85902c7:src/views/EditorView.vue | command
grep -nEi "dirty|isDirty|unsaved|modified"` directly and got three hits:
lines 69, 632, 634 - matching exactly what the review brief itself already
quoted. The brief's "0 lines... already measured" premise was measured
against the pre-Task-1 authoring-time tree (`docs/superpowers/plans/
2026-07-30-plan-12-qa-round-3-findings.md`), before Task 3 shipped the
`editor-unsaved` catalog key and the comment mentioning it - not against
Task 4's actual base commit. The falsified premise does **not** leave a
hole: the narrow, structural acceptance expression is a separate check,
independently verified by me to return 0 on the real end state with both
alternation members firing against synthetic probes, so it still fully
carries R28's requirement (no second dirty-tracking mechanism) on its own
terms.

**Q3 (the `dirty` unused-var disable).** The disable's shape is correct: a
single-line, rule-scoped `// eslint-disable-next-line
@typescript-eslint/no-unused-vars -- see above`, matching ESLint's supported
directive-comment syntax (confirmed: `pnpm lint` parses and accepts it
cleanly) and the spirit of the cited `HelpSidebar.vue` precedent (narrow,
justified, single site), even though that precedent is a template-level
block disable for a different rule, not a script-level line disable. No
disable-free form exists within this task's constraints: `tsconfig.json` has
`noUnusedLocals` off (tsc is not the gate here), so the only active rule is
ESLint's `@typescript-eslint/no-unused-vars` with default options: no
`varsIgnorePattern` exists to exempt an underscore-prefixed name, and adding
one would require editing `eslint.config.js` beyond the brief's explicit
fence ("nothing else in the file changes"). Inventing an artificial consumer
(e.g. `defineExpose({ dirty })`) would be worse - a new, unspecified public
surface, itself a latitude violation. On whether this was a fork that should
have been returned: **in one direction**, the brief's "Must not decide" list
never mentions this specific lint-mechanics question, so resolving it
unilaterally is, on its face, a decision made without an explicit license.
**In the other direction**, the plan's own Interfaces section pre-licenses
the unused state ("Produces: `dirty`... nothing in this file reads it yet"),
`eslint.config.js`'s fence forecloses every alternative but a scoped
disable, and `docs/process-conventions.yaml`'s own precedent (line 421: "a
single mechanical fix... adjudicated correct keyboard-level resolution") is
closely on point for exactly this class of lint-mechanics resolution. I lean
toward the second reading - correctly resolved at the keyboard, not a fork -
but the first reading is not unreasonable and I would not fault an
escalation either.

**Q4 (the brief's own named residual).** The residual as stated is **not**
the whole residual - the shipped code leaves substantially more uncovered
than the paragraph admits. See Finding 1: removing the entire `doSave` mark
line (not just the racy-window scenario) is caught by nothing in the full
93-test suite, because `dirty`'s value has zero consumers anywhere in this
task. The two absence checks plus the structural fix cover exactly what a
careful reading of the paragraph claims for them (S1 "pins the structure" -
a one-time textual demonstration, not a standing or dynamic guard; D1 covers
a different property entirely, no second mechanism) - the paragraph does not
literally overclaim in its own sentences, but its framing ("what ships
instead is stronger than a comment and weaker than that test") reads as
narrowing the gap to the racy case specifically, when the true gap is the
entire dynamic-correctness surface of Step 1c.

## Harvest

- **Dominant pattern:** every fenced value in this task's brief (code
  blocks, selectors, messages, testids) was reproduced byte-exact; the
  Fence-plus-"Must not decide" mechanism is working as designed for this
  task; zero latitude findings surfaced in that dimension.
- **Repeated defect shape worth carrying forward:** a derived/internal value
  produced in one task for a later task's consumption (here, `dirty`) is, by
  construction, unobservable to that task's OWN e2e suite - there is no UI
  surface to assert against yet. A residual paragraph naming only the
  hardest-to-build scenario (the racy await window) reads as implying the
  easy scenario is covered, when in this producer/consumer split BOTH are
  equally untested by the producing task. Recommend a house-knowledge
  addition: a residual note for a value with zero in-task consumers should
  say so explicitly ("no test in this package can observe X's value at
  all"), not name only the hardest uncovered case.
- **Over-restriction / no real fork:** none found. Every place the brief's
  "Must not decide" list forecloses a decision, the foreclosed decision had
  a genuinely settled, unambiguous answer already recorded in D108/D112 -
  I did not find a fenced value that was actually still open in spirit.
- **A structural trap the brief itself named, found in the wild, in a case
  the brief did not name:** Finding 2 (the `:disabled="!model || ..."`
  fallback masking `canUndo`'s real value in the failed-open test) is a live
  instance of exactly the pattern the review brief describes generically for
  a different case (D1). Worth flagging in-house: any assertion of the form
  "button X is disabled" as a proxy for an internal derived boolean is
  suspect wherever that button's `:disabled` expression is itself a
  disjunction with an unrelated term.

## Instruments used

Full listing of independently-run commands, mutations, fire tests and
restores is in the tool-call history of this review session; the key
reproductions (all against `85902c7` unless noted, all fire-tested against a
synthetic/known-present probe before trusting an empty result) are: the
mutation-path whole-value and in-place expressions; the external-writer
check; S1, D1 (both forms), P1 with RED/GREEN; the eslint rule's RED, GREEN,
over-match control and per-alternation-member fire; the catalog recount (49
-> 51) and `pnpm check:i18n`/`pnpm lint`/`pnpm test:e2e` re-runs on the
tracked repo; and, on the scratch copy, mutation rounds against: `createBlank`'s
`resetHistory` call, `doSave`'s mark line (both corrupted-to-`model.value`
and fully deleted), `addRule`'s coalesce reset, the `@focusout` handler, the
depth-cap boundary (`>` to `>=`), the text-entry exemption (both the
negative and the positive control), the D112 template gates (reverted to
bare `!model`, independent of the lint rule), and `resetHistory`'s
failed-load branch (history/position left standing). Every mutation was
restored and the restore verified by `diff`, never by exit code.
