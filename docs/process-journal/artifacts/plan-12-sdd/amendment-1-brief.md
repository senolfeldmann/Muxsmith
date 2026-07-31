# Amendment 1 brief: record the owner's failed-load ruling in the plan

You are the AUTHOR of a mid-run amendment to an owner-approved, in-execution
plan. This brief is your requirements. An independent reviewer grades your work
against it afterwards, and a fix loop runs until it approves.

## Where this sits

`docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` is Plan 12,
owner-approved and in execution: **tasks 1, 2 and 3 are closed and committed;
tasks 4 to 7 have not started.** Task 3's review parked one finding as an owner
decision. The owner has now ruled it, and the ruling changes user-visible
behaviour in a task that has not run yet.

Your job is the amendment: the plan (and whatever normative document the record
belongs in) must describe what will now be built, before the affected task is
dispatched. You are NOT implementing anything. No product source, no test, no
catalog is touched by this round.

## Read first

1. `.superpowers/sdd/plan-12/owner-ruling-1-failed-load-empty-state.md` - the
   ruling, with the chosen option quoted verbatim from the memo.
2. `.superpowers/sdd/plan-12/owner-decision-failed-load-empty-state.md` - the
   costed memo behind it, including the measurement that decided the option.
3. `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` in full.
   You are editing it and you owe it a self-contradiction sweep, so read all of
   it, not only the task you change.
4. `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` in full - the shipped
   ADR file Task 1 wrote (D106 to D110), and the house pattern any new decision
   record follows.
5. `src/views/EditorView.vue` in full, as Task 3 left it - the four surfaces this
   ruling concerns and the conditions that gate them today.
6. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, section 8.2, as
   Task 1 amended it.

## What the amendment must produce

**A-1. The behaviour, stated normatively.** After a profile fails to load, the
editor renders the "Selected profile" line and the parse error, and renders
neither the empty-state paragraph nor the recents section. The two hidden
surfaces show only before anything has been opened or created at all.

**A-2. An explicit gate condition, defined once.** The memo names this as the
thing that must be decided together with the option, and it is the substance of
your design work: the two gates must carry a stated condition rather than
inherit a term whose meaning another task changes. Derive the condition, state
it in the normative record, and show it is well-defined in **every** editor
state reachable after tasks 3, 4 and 5 - at minimum: nothing opened or created
yet; a blank profile created and unsaved; a profile opened successfully; an open
that failed on a fresh editor; an open that failed while a profile was already
held. Name the states you walked; a state you did not walk is a gap.

**A-3. The decision record.** The ruling is recorded in the normative layer, not
only in the plan's prose. Decide and justify **where**: a new numbered decision,
or an in-place amendment of the existing decision that owns this surface. The
house pattern for numbered decisions - required slots, immutability, dated
event-log semantics, `superseded by` links - governs, and the decisions file
itself shows the shape. Corroboration you must re-measure rather than trust: the
highest decision number in use across `docs/` appears to be D111, with no D112
or higher anywhere. Re-derive that yourself before using any number.

**A-4. Placement in a task.** State which task builds this, and update that
task's sections so the task is executable exactly as the plan's other tasks are:
`Read first`, the EXHAUSTIVE Files list, the steps, the verification step, the
commit block's pathspecs, and the `Must not decide` list. The memo's own view is
that the natural vehicle is the task that already touches this surface; verify
that against the plan and say so, or place it elsewhere and say why.

**A-5. Acceptance rows with named producers, walked in halves.** The plan's
acceptance map walks each observable in its halves and names a producing test
per half. A rendered-versus-hidden pair is two halves: a producer that only
asserts the hidden side passes against code that hides the surfaces in every
state, including the pre-session state where they must still appear. Where a
producer is an absence check, it carries the plan's three parts - the
expression, a pre-state or in-test fire with its exact expected non-zero result,
and the end state with its expected zero.

**A-6. The producer ships in the same task as the behaviour.** The plan's global
constraint on this is absolute and its one exemption is new test
INFRASTRUCTURE. A scenario the existing Playwright plus mock-IPC harness can
already express is not exempt. Do not write "coverage follows later" anywhere.

**A-7. No new catalog string.** Part of what the owner bought with this option is
that it adds none, so the editor catalog budget arithmetic in the plan is
unchanged by this amendment. If you measure that the behaviour cannot be built
without a string, that is a finding you RETURN (see "If you find a fork" below),
not a string you add.

**A-8. The spec question, answered with a measurement.** Determine whether the v1
spec's section 8.2, as Task 1 amended it, asserts anything about the state this
ruling changes. If it does, the amendment says what changes, in which task, and
adds the edit to that task's Files list. If it does not, say so and paste the
measurement that shows it.

**A-9. A self-contradiction sweep over the whole plan.** Your change falsifies or
touches more than the task you edit. Sweep for and reconcile: the work-item
coverage map, the acceptance map, the sequencing and dependency rationale, the
affected task's `Must not decide` list, any count or enumeration your edit
moves, any cross-reference to the parked decision, and any statement elsewhere in
the plan that describes the failed-load state. Report the sweep as an
enumeration with the expression you used and a fired control, per the plan's own
rule for absence-shaped checks - a search whose terms come from your memory of
what you wrote produces a false absence.

**A-10. No task is added, removed or re-cut.** The amendment's role count assumes
this. If your analysis concludes a task must be added, removed or re-cut, STOP
and return that as a finding rather than doing it: it changes who must author
and review this amendment.

## If you find a fork

Return it. Do not resolve a design fork at the keyboard, do not soften a
requirement, and do not "improve" a decision this plan already settled. Write a
short decision memo - the options, their costs against the invariants the plan
names, and a recommendation - and return status NEEDS_CONTEXT with it. The
controller routes it. This applies with full force to a contradiction you find
between this brief and the plan: refute it with evidence and return it.

## Boundaries

- **Documents only.** You edit the plan and, if that is where the record belongs,
  the plan's decisions document. You do not touch product source, tests, Fluent
  catalogs, `docs/ROADMAP.md`, `docs/process-journal.md`, or any of the four
  house-knowledge YAML files. Something ledger-worthy goes in your report.
- **Nothing else is re-opened.** D107's ruling that the parse error still renders
  stands. The "Selected profile" line stays. The option choice is the owner's and
  is not re-argued. Every other decision of D106 to D110 stands.
- **No design-latitude clause, in either form**, in anything you write: not an
  explicit permission ("the implementer may choose", "if a simpler equivalent
  exists"), and not the commoner omission - a mandated set that is never
  enumerated, a list that trails off, a "one per X" with no X list, a step that
  needs a name, string, key or file the implementer would have to invent. The
  test is "must the implementer invent something it is not allowed to invent?"
  Ask it of every normative sentence you add.
- **A proposed safeguard stays.** A guard, test, enumeration or check the plan
  already proposes is not argued out here; it is removed only after it is built
  and measured redundant.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash or en-dash. German orthography inside German strings is orthography and
  is copied exactly.
- **A document never cites a line number inside itself**, and a code comment
  locates code by symbol, never by line number. Both are owner-ruled house rules
  and both bind everything you write.
- **Every empirical claim you make is pasted from the run that produced it**,
  never recalled, and never attributed to a command that was not the one run.

## Corroboration, which does not replace your own measurement

Measured by the controller at HEAD before this dispatch. Re-derive each yourself;
if one is wrong, that is a finding worth reporting.

- The four surfaces are gated in `src/views/EditorView.vue` as follows today:
  the selected-profile line on `currentPath`; the unsaved line on `sessionActive`
  as an `else` branch of that; the empty-state paragraph on `!model`; the recents
  section on `!model && recents.length`.
- `sessionActive` is a `ref` today, set true by both funnels. Task 4 converts it
  to a computed over `savedSnapshot`, and Task 4's failed-open branch sets
  `savedSnapshot` to null, so after Task 4 a failed open makes `sessionActive`
  false where today it is true.
- Highest decision number in use across `docs/`: D111. No D112 or higher exists.

## Deliverables

1. The edited plan document, and the decisions document if the record lands
   there.
2. One commit, pathspec-scoped to exactly the files you edited. Stage explicitly;
   never `git add -A`. Commits on this repo are standing-authorized by the owner
   and agent commits are deliberately unsigned: use
   `git -c commit.gpgsign=false commit -- <paths>`. The dispatch names your
   trailer.
3. Your report at `.superpowers/sdd/plan-12/amendment-1-report.md`: what you
   changed and where, the design reasoning behind the gate condition, the state
   walk of A-2, the sweep of A-9 with its expression and fired control, every
   measurement pasted, and anything you deliberately did not do with its reason.

Return only status, the commit, and any concerns. The report file carries the
detail.
