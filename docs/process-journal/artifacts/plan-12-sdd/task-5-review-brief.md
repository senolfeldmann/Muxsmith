# Task 5 review brief

You are the independent reviewer of Task 5 of Plan 12. You did not implement it and
you have no stake in it standing. Two verdicts are required and neither is optional:
**spec compliance** against the task's requirements, and **task quality** as graded
findings.

## The three artifacts

- **Requirements:** `.superpowers/sdd/plan-12/task-5-brief.md` - the task's full
  text as amended, with the exact values the implementer was told to use verbatim.
  This is what compliance is graded against, requirement by requirement.
- **The implementer's report:** `.superpowers/sdd/plan-12/task-5-report.md`. Note
  its shape: the task returned NEEDS_CONTEXT mid-run over a genuine plan
  contradiction, the plan was amended, and the completion is appended as section 9
  with the original submission left standing above it.
- **The diff:** `.superpowers/sdd/plan-12/review-9ee3b9e..0b00262.diff` - commit
  list, stat and full diff with context in one file.

Your ground truth is the brief and the spec, never the code. Where the code and the
brief disagree, the brief wins unless you can refute the brief with evidence, which
is a valid and wanted finding.

## What this task ships, and where to aim

A reusable confirm dialog, two guarded call sites, three catalog strings per locale,
and the guard cases. Three targets, in order of what they cost if wrong.

**1. Case 3 leg (ii) is the FIRST AND ONLY producer anywhere in this plan for the
property that saving marks the profile that was actually written.** Task 4
introduced that mark and validated none of it - its reviewer deleted the entire
marking line and the build plus all 93 cases stayed green, because nothing consumed
the value until this task. So this leg is the whole coverage of a mechanism whose
failure direction is data loss: a guard that silently does not fire over unsaved
changes. **Break the marking mechanism yourself and watch this leg go red.** The
implementer reports doing exactly that; do it with your own instrument and do not
accept its account. If the leg passes with the mark broken, that is a Critical
finding and the plan's own acceptance map is falsified with it.

**2. The ordering: the confirmation precedes the destructive action.** The guard
sits before the file dialog, so a cancelled file dialog after a confirmed discard
loses nothing. Two absence checks cover it. Check that each can fire, and check the
shipped call sites rather than the tests' description of them.

**3. The three repaired cases in `e2e/editor-undo-redo.spec.ts`.** This task's guard
fronts three cases that Task 4 wrote. The repair answers a confirm that now stands
between a click and the state under test. **No existing assertion may be weakened,
removed, reworded or skipped by that repair**, and the repaired cases must still be
able to fail for the reasons they were written for. Verify by mutation, per case.

## Dimensions to run

**Spec compliance**, requirement by requirement from the brief, each MET or NOT MET
with the evidence you used. Byte-check every fenced value: the component contract,
the two guard call sites, the six catalog lines across both locales, the commit
block.

**A step that names several sites is checked AT EACH SITE.** This is not general
advice: the immediately preceding task failed exactly here. Its Step 7 named two
catalog-budget comments, only one was corrected, and both its task review and its
delta re-review graded spec compliance MET because they asked whether the step had
been performed rather than checking each site it names. This task's Step 4c names
two halves. Check both, at the file, and say so per half.
(House entry: `a-normative-sentence-naming-a-set-is-discharged-member-by-member`.)

**Test-power.** Per new or changed case: can it fail? Prefer a mutation over an
argument. A mutation that fails to discriminate looks identical to one that does.

**The disjunction trap, because this task's neighbourhood is full of it.** An
assertion that a control is disabled proves only that SOME term of its disabled
expression holds; where that expression is a disjunction, a case using the control
as a proxy for one term passes on the other. The same shape reaches confirm-dialog
assertions: a count of zero is satisfied by a page that never rendered the dialog
for an unrelated reason. (House entry:
`a-disabled-assertion-over-a-disjunction-proves-only-its-weakest-term`.)

**Latitude, in both forms**, over anything the diff introduces that the brief did
not write out: an invented name, string, key, selector, constant or file is a
finding, and so is a mandated set the brief never enumerated that the implementer
closed by choosing.

**House conformance.** The four house-knowledge files are review ground truth
alongside the brief; cite entries by id. Nearest this diff:
`comments-locate-by-symbol-never-by-line-number`,
`a-document-never-cites-a-line-number-inside-itself`,
`proc-normative-count-recomputed`, `proc-proposed-safeguard-stays`,
`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`,
`a-normative-claim-is-scoped-down-to-its-producers-reach`,
`editor-generic-action-keys` (the editor catalog budget is a hard boundary),
`gitignored-paths-need-command-grep`, `bash-isms-run-under-bash`,
`frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`,
`tests-ship-with-the-feature-never-after`, `design-empirical-claims-reproducible`.
Also check the new component against the house modal pattern the brief names, and
the German catalog values against German orthography.

**The no-work-needed check, standing.** Wherever the diff, a comment or the report
concludes that a guard, an enumeration or a check is unnecessary - "so no X is
needed", "that cannot happen here", "the existing guard covers it" - **verify the
claim that makes it unnecessary. Run it; do not weigh it.** The report contains at
least one such conclusion by construction: the implementer checked and dismissed a
concurrency question about the now-async creation funnel.

**Typography.** ASCII hyphens, straight quotes, no Unicode ellipsis, no em-dash or
en-dash, in every comment, string and test name the diff adds. German orthography
inside German catalog values is orthography and is copied exactly.

## Adjudication questions - one required verdict each, phrased open

**Q1.** The implementer checked the forward note about concurrent entry into the
now-async creation funnel and concluded no mechanism is needed, on the ground that
the native modal makes the rest of the page inert before that function ever yields.
Run that premise rather than weighing it. Is it true of the shipped code, and does
it cover every path into that funnel, including the one the guard itself introduces?

**Q2.** The catalog recount must land at 54 and the budget comments in two files
must say so. Confirm the figure by your own count from the artifact, and confirm
each of the two sites independently. State the decomposition you got.

**Q3.** Task 5's guard was added to two call sites and deliberately NOT to a third
path, on the recorded ground that the third is unreachable while the editor holds a
model. Is that reachability claim true against the shipped code as this task leaves
it, or does anything this task adds make it reachable?

## What is reproducible now

Everything: the code exists, the tests run, the checks are runnable. Re-run rather
than re-derive - but **build your own instruments**, outside the repository, at a
path the implementer did not use.

Three environment facts, all of which have cost this project a wrong result:
`grep` here is a shell function honouring `.gitignore`, so use `command grep` when a
sweep must reach ignored paths; never read a command's exit status through a
pipeline; and **the suite runs against the BUILT bundle**, so a mutation on frontend
source needs a build before the run, and so does the restore afterwards - an
unrebuilt restore leaves the suite measuring the broken bundle while `git status`
reads clean. `cp` and `rm` are aliased to their interactive forms here: use
`command cp` and `command rm` and verify a restore by content, not by exit code.

You are read-only on the repository except your own verdict file. The implementer
already ran the full gate and its output is in the report - do not re-run the whole
gate; run the checks your findings need.

## Harvest

Separate section at the end. Report the dominant patterns and any repeated defect
shape a future implementer of this kind of task should know; the controller writes
these into the house ledger and you never write to it yourself. Include explicitly
any place where a boundary in the brief forced a stop on a fork that in your
judgement had no real decision content.

## Output

Write your verdict to `.superpowers/sdd/plan-12/task-5-verdict.md`. Return to the
controller only: the two verdicts, finding counts by severity, your three
adjudication answers in one line each, and nothing else.
