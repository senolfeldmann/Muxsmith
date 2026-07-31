# Task 4 review brief

You are the independent reviewer of Task 4 of Plan 12. You did not implement it
and you have no stake in it standing. Two verdicts are required and neither is
optional: **spec compliance** against the task's requirements, and **task
quality** as graded findings.

## The three artifacts

- **Requirements:** `.superpowers/sdd/plan-12/task-4-brief.md` - the task's full
  text, with the exact values the implementer was told to use verbatim. This is
  what compliance is graded against, requirement by requirement.
- **The implementer's report:** `.superpowers/sdd/plan-12/task-4-report.md`.
- **The diff:** `.superpowers/sdd/plan-12/review-85902c7..1092eb7.diff` - commit
  list, stat and full diff with context in one file.

Your ground truth is the brief and the spec, never the code. Where the code and
the brief disagree, the brief wins unless you can refute the brief with evidence,
which is a valid and wanted finding.

## What this task ships, so you know where to aim

Undo/redo over the editor's single mutation funnel, the unsaved-changes state
derived from that history rather than from a hand-set flag, a one-line standing
lint guard, and **fourteen new end-to-end cases**. The tests are the highest-value
target in this diff, and the question per case is **not whether it passes but
whether it can FAIL.** The feature they cover is what resumes the owner's manual
QA pass, so a case that cannot fail will be read as coverage that does not exist.
Build mutations and run them.

Two structural traps this feature sits on, both worth attacking directly:

- **The save state is derived, and the mark must record the profile that was
  WRITTEN, not the live one.** Two awaits sit between the capture and the mark and
  the editing surface stays live across both, so a mark taken from live state
  reports "no unsaved changes" over content the file does not hold - and every
  guard built on it in the next two tasks disarms in the data-loss direction. Two
  absence checks exist for this. Check that they can fire, and check the shipped
  line itself.
- **A fallback between a mutation and an assertion makes the assertion green under
  every mutation upstream of it.** Where a red state disturbs a fallback's input
  rather than defeating the fallback, the check passes in exactly the state it
  exists to forbid.

## Dimensions to run

**Spec compliance**, requirement by requirement from the brief, each MET or NOT
MET with the evidence you used. The brief's steps carry fenced code and fenced
catalog values; byte-check the fenced ones.

**Test-power.** Per new case: can it fail? Prefer a mutation over an argument. The
implementer reports firing several; re-run them with **your own** instruments and
add your own. A mutation that fails to discriminate looks identical to one that
does.

**Latitude, in both forms**, over anything the diff introduces that the brief did
not write out: an invented name, string, key, selector, constant or file is a
finding, and so is a set the brief mandated but never enumerated that the
implementer closed by choosing.

**House conformance.** The four house-knowledge files (`docs/product-boundaries.yaml`,
`docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`)
are review ground truth alongside the brief; cite entries by id. Nearest this
diff: `comments-locate-by-symbol-never-by-line-number`,
`a-document-never-cites-a-line-number-inside-itself`,
`proc-normative-count-recomputed`, `proc-proposed-safeguard-stays`,
`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`,
`a-search-whose-terms-come-from-memory-produces-a-false-absence`,
`a-normative-claim-is-scoped-down-to-its-producers-reach`,
`editor-generic-action-keys` (the editor catalog budget is a hard boundary),
`gitignored-paths-need-command-grep`, `bash-isms-run-under-bash`,
`tests-ship-with-the-feature-never-after`.

**The no-work-needed check, standing.** Wherever the diff, a comment or the report
concludes that a guard, an enumeration or a check is unnecessary - "so no X is
needed", "that cannot happen here", "the existing check covers it" - **verify the
claim that makes it unnecessary. Run it; do not weigh it.**

**Typography.** ASCII hyphens, straight quotes, no Unicode ellipsis, no em-dash or
en-dash, in every comment, string and test name the diff adds. German orthography
inside German catalog values is orthography and is copied exactly.

## Adjudication questions - one required verdict each, phrased open

The implementer raised four concerns and I reproduced two of them myself with
different results. Each needs a ruling from you; a concern merely carried dies as
noted-without-ruling. None of these is pre-rated.

**Q1. The mutation-path sweep returned 8 whole-value assignments where the plan
cites 7.** The implementer reports the eighth is Task 3's own `createBlank`,
added after the plan's authoring-time sweep, and argues it belongs in the same
excluded session-start-funnel category as `openPath` because `resetHistory`
pre-seeds the baseline before the watcher can observe it. Is that classification
correct at the code, or does the eighth site reach the push rule in some state? Is
the six-function mutation set the brief fences still exhaustive and still correct?

**Q2. Absence check D1's broad pre-state figure does not reproduce, and the two
non-reproductions disagree with each other.** The brief states the expression
`grep -nEi "dirty|isDirty|unsaved|modified"` over `src/views/EditorView.vue` was
"already measured: 0 lines" on the baseline. The implementer reports it returns 2.
My own run, against the file as it stood at the task's base commit `85902c7`,
returns 3:

```
69:// `editor-empty`, `editor-unsaved`, D107). The Open button, the
632:      data-testid="editor-unsaved"
634:      {{ $t("editor-unsaved") }}
```

Measure it yourself and rule on the figure. Then rule on what follows: the brief
built D1's credibility on that pre-state being a real absence, and it is not one.
Does D1's actual acceptance expression - the narrow structural one - still carry
the requirement it is there for, or does the falsified premise leave a hole?

**Q3. One decision was made at the keyboard rather than returned.** `dirty` is
produced by this task with no consumer inside it, by design, so
`@typescript-eslint/no-unused-vars` flags it; the implementer added a scoped
`eslint-disable-next-line` and cites `src/components/HelpSidebar.vue` as this
repo's one precedent for a narrowly-justified disable. I verified that precedent
exists. Rule on three things: whether the disable is the right shape here, whether
a form exists that needs no disable at all, and whether this was a fork the
implementer should have returned rather than resolved. The last part is a process
judgment and I want it answered in both directions.

**Q4. The brief's own named residual.** The direct behavioural producer for the
save-marking property - a test that moves the model inside one of `doSave`'s two
await windows - was not built, on the brief's argued ground that the existing mock
harness cannot hold a queued response open and that building one is new test
infrastructure. I have routed the deferral and it is not your decision. What IS
yours: is the residual as stated actually the residual, or does the shipped code
leave more uncovered than the brief's paragraph admits? And do the two absence
checks plus the structural fix cover what that paragraph claims they cover?

## What is reproducible now

Everything. This task's code exists, its tests run, and its checks are runnable.
There is no prescribed-for-later evidence in it, so re-run rather than re-derive -
but **build your own instruments**, outside the repository, at a path the
implementer did not use. Re-running someone else's script produces agreement by
construction. Where an expression contains an enumerated set, derive that set from
the artifact rather than from the implementer's list.

Two environment facts: `grep` here is a shell function honouring `.gitignore`, so
use `command grep` when a sweep must reach ignored paths; and never read a
command's exit status through a pipeline, which yields the tail's status rather
than the command's.

You are read-only on the repository. Do not edit, stage, commit or push anything
except your own verdict file. The implementer already ran the full gate and its
output is in the report - do not re-run the whole gate, run the checks your
findings need.

## Harvest

Separate section at the end. Report the dominant patterns and any repeated defect
shape a future implementer of this kind of task should know; the controller writes
these into the house ledger and you never write to it yourself. Include explicitly
any place where a boundary in the brief forced a stop on a fork that in your
judgement had no real decision content - an over-restriction finding is wanted
here, not second-guessing.

## Output

Write your verdict to `.superpowers/sdd/plan-12/task-4-verdict.md`. Return to the
controller only: the two verdicts, finding counts by severity, your four
adjudication answers in one line each, and nothing else.
