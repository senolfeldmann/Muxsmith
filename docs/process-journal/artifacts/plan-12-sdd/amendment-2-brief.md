# Amendment 2 brief: repair a criterion my own ruling falsified, and place one stale comment

You are the AUTHOR of a second mid-run amendment to Plan 12. You wrote amendment 1
and its fix round; this corrects one of its enumerations and settles one ownership
question. An independent reviewer grades the delta afterwards.

## What happened, stated plainly, because the cause is a controller error

Amendment 1's Task 5 Step 4b enumerated the cases in
`e2e/editor-undo-redo.spec.ts` that Task 5's discard guard would front with a
confirm, and fixed the set at two: "Open resets" and "A failed open clears rather
than keeps". That enumeration was correct when you drew it.

**Afterwards, Task 4's review found a coverage gap and I ruled a new case into
Task 4's fix round** - `createBlank resets: New after edited history clears both
Undo and Redo`. It builds history, so the editor is dirty, and it clicks **New**,
which Task 5's Step 2 guards exactly as it guards Open. So the file now holds a
third affected case, and Step 4b says "exactly two ... No other case in that file
is touched".

Task 5's implementer met the collision at code contact, returned NEEDS_CONTEXT
with a decision memo rather than resolving it, and left the tree in the failing
state. That was correct. Its report is at
`.superpowers/sdd/plan-12/task-5-report.md`.

**The root is not the count. It is the criterion.** Step 4b's rule for membership
is scoped to a second *Open* click, and the property that actually decides
membership is entering a funnel this task guards while the editor is dirty - a set
that includes New. A criterion narrower than the mandate it serves regenerates
this defect on the next addition, which is what the house entry
`a-normative-claim-is-scoped-down-to-its-producers-reach` is about, one level up.

## What the amendment must produce

**B-1. A criterion a third party can apply to get the same set, deterministically.**
Rewrite Step 4b's membership rule so it derives from what Task 5's Step 2 actually
guards, rather than from one of the two entry points. Then re-derive the
enumeration from your own rewritten criterion, against the file as it stands, and
state both. Do not take my set or the implementer's on trust; both are stated
below as corroboration to be checked.

**B-2. The repair pattern for the added member.** Step 4b already fences a repair
for its existing members. Say whether the added member takes the identical pattern
or needs a different one, and why - it enters its funnel through a different
control, so the answer is not automatic.

**B-3. No assertion is weakened anywhere.** The repair answers a confirm that now
stands between the click and the state under test. It does not remove, soften,
reword or skip an existing assertion, and it does not change what any case is
about. A safeguard the plan proposed stays.

**B-4. One ownership question, settled.** `src/views/EditorView.vue` carries a
catalog-budget comment reading "`gui-editor.ftl` carries 49 ids today". Task 4's
Step 7 required correcting it - it names that file and `e2e/smoke.spec.ts`
together - and Task 4 corrected only the second one; its review and its delta
review both graded spec compliance MET. So the comment is stale by two packages.
Task 5's own verification step tells its implementer to recompute the count and
correct the budget comments, while its Files list scopes that file to "the dialog
mount, the two guarded call sites", and the implementer read the narrower scope
and left the comment alone. Settle it: make Task 5's step and its Files list agree
on whether that comment is corrected in Task 5, and if it is, say to what and by
which authority. Record what Task 4 left undone, so the miss is on the record
rather than quietly repaired.

**B-5. The sweep.** Anything else your edit falsifies: counts, the acceptance map,
Task 5's `Must not decide` list, the plan's close-actions list, cross-references
to the two-case enumeration. Report it as an enumeration with the expression you
used and a fired control, deriving the terms from the artifact rather than from
your memory of what you wrote.

**B-6. No task is added, removed or re-cut**, and no design decision is re-opened.
If your analysis says otherwise, STOP and return it rather than doing it: it
changes who must author and review this amendment.

## Corroboration, to be checked and not inherited

Measured by me at the current tree. If one is wrong, that is a finding.

- Three cases in `e2e/editor-undo-redo.spec.ts` build history and then enter a
  guarded funnel: `open resets`, `createBlank resets`, and the failed-open case.
  The D112 three-leg case opens twice but never edits, so it stays clean and is
  unaffected; the truncation case and the save-marks case perform no second entry.
- The stale comment is the only one of its kind left in the tree: a wide search for
  a catalog-budget figure across the view and the spec files returns exactly the
  one site named in B-4. Both catalogs currently carry 54 ids, with Task 5's three
  present but uncommitted.

## Boundaries

- **Documents only.** You edit the plan, and the decisions document only if the
  record belongs there. **Anything you find that lies outside that edit surface
  goes into your report as a controller item** - never into a conclusion that no
  vehicle exists.
- **The working tree is NOT yours and it is not clean.** Task 5's implementer has
  uncommitted work in `src/`, `locales/` and `e2e/`, and it is paused waiting for
  this ruling. You must not stash, check out, reset, restore, clean, or otherwise
  touch anything outside `docs/`. Your commit is pathspec-scoped to exactly the
  documents you edit - two writers in one tree share one index, so a bare
  `git commit` would take Task 5's half-finished work with it.
- No design-latitude clause in what you write, in either form: not an explicit
  permission, and not the commoner omission - a mandated set never enumerated, a
  list that trails off, a step needing a name or a string the implementer would
  have to invent. Ask of every normative sentence: must the implementer invent
  something it is not allowed to invent?
- Typography: ASCII hyphens, straight quotes, no Unicode ellipsis, no em-dash or
  en-dash. A document never cites a line number inside itself; a comment locates
  code by symbol.
- Every empirical claim is pasted from the run that produced it. A fence carries
  only what a re-run reproduces.

## Deliverables

1. The edited plan document, and the decisions document if the record lands there.
2. One commit, pathspec-scoped, staged explicitly, never `git add -A`,
   `git -c commit.gpgsign=false`. The dispatch names your trailer. Do not push.
3. Your report at `.superpowers/sdd/plan-12/amendment-2-report.md`: what you
   changed, the criterion you wrote and why it is deterministic, your own
   re-derivation of the set from it, the B-4 settlement with its authority, and the
   B-5 sweep with its expression and fired control.

Return only status, the commit, and any concerns.
