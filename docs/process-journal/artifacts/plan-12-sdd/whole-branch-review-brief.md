# Plan 12 whole-branch review brief

You are the whole-branch reviewer closing Plan 12. Every task in it has already been
built and independently reviewed with its own fix rounds; your job is what no task
review could do - judge the branch as one artifact, find the breaks that live between
tasks, and rule on what must be fixed before this ships.

You run on the strongest model, and you are the last reviewer before the plan closes
and the branch is pushed.

## What this branch is

Plan 12 repairs the findings of the owner's manual QA round 3 on Windows, which
stopped because the profile editor could open a profile but never create one. Seven
tasks: the normative documents, a three-state language control, blank-profile
creation, undo and redo with the unsaved state derived from its history, the discard
guards, the shell close prompt with its localization, and the user documentation.

**Ground truth, in order:** the v1 spec as this plan's Task 1 amended it; the plan
document `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` including
its Global Constraints, its decision register and its acceptance map; the decisions
file `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` (D106 to D112); and the
four house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`,
`docs/process-conventions.yaml`, `docs/decision-ledger.yaml`), cited by entry id.

## The package, and one thing about its shape

The diff file is named in the dispatch. **Of the 45 commits in this range, 13 touch
product files.** The rest are the controller's own process artifacts committed
alongside: house-knowledge ledger entries, two four-eyes plan amendments with their
fix rounds, a journal entry and two handoff snapshots. Do not read a ledger or
amendment commit as task work, and do not grade the plan document's own commits as
code. If you want the product-only view, restrict to `src`, `src-tauri`, `crates`,
`locales`, `help`, `e2e`, `scripts` and `eslint.config.js`.

## What only you can do

**1. Cross-task breaks.** Three tasks wrote the same view file and three wrote the
same catalogs, serially. Each task review saw its own diff. You see all of them
against each other: a helper one task introduced and another quietly re-derived, a
comment one task made true and a later one falsified, a test one task wrote that a
later task's guard now fronts, an assertion that passes only because of what a
neighbouring task happens to leave behind. This class is the recorded reason
whole-branch reviews exist here.

**2. The acceptance-map walk, and it is not a formality this time.** Walk every row
of the plan's acceptance map against the branch and name the producer you actually
find. **Two rows are already known to be false, and I want you to check the rest
against that pattern rather than to re-find these two:**

- The row claiming unit coverage for "confirming a discard-only close quits,
  cancelling does not" names a producer that cannot exist - the row directly below
  it says in its own text why that surface needs the Tauri runtime. Disposition:
  the row is corrected at the plan close and the observable rides the owner's
  existing 1.x GUI-test-harness item. **Not a fix for your wave.**
- The row claiming the reworded batch empty-state string is asserted through the
  catalog helper in the existing batch scenario. That id appears in **zero test
  files**; I reproduced it with a control from the same catalog block returning 23
  hits in the same spec file, so the absence is real. **This one IS a required fix
  for your wave**, because the string is user-visible, this plan reworded it, and
  the producer is expressible with the existing harness. Task 7's Files list is
  exhaustive and contains no test file, which is why it was withheld from that task
  rather than built there.

The general shape, now a house entry
(`an-acceptance-row-naming-an-existing-producer-is-verified-by-finding-it`): a row
naming a producer the plan will BUILD is self-correcting, a row naming an EXISTING
producer is believed because it describes the past. **Both known instances are of
the second kind. Walk every remaining row of that kind and open it.**

**3. Rule on the deferred and parked items.** They are enumerated in
`.superpowers/sdd/plan-12/progress.md` under its "deferred minor" and
"deferred with a trigger" headings, plus the two dispositions recorded in prose:
the stale Tier-2 catalog-budget entry, the four sites carrying a stale figure of
which two hold and two do not, the confirm component's missing reentrancy guard,
and the validation-response race. For each: must it be fixed before this ships, or
does its recorded disposition stand? Say which and why. A roll-up nobody reads is a
silent discard, and I am the one who wrote these dispositions, so an independent
ruling on them is worth more than my own.

## Work already routed to your fix wave, which you own

Three items are known and confirmed, and they belong in your wave rather than in any
task, because each falls outside the exhaustive Files list of the task that would
otherwise own it. **Confirm each at the artifact before it is fixed** - a routed item
is still a borrowed claim - and then include them in your findings list so one fix
dispatch carries them together with whatever you find yourself.

1. **The missing producer for the reworded batch empty-state string** (see above).
2. **Two terminology instances of the synonym direction**, in the editor help topic:
   a German verb and its English mirror name the create-a-profile affordance with a
   word other than the one the shipped catalog uses for that same affordance. The
   classification was confirmed independently by Task 7's reviewer at the corpus, and
   it stress-tested the most promising candidate for a third instance and found it
   resolves clean, so **the list is two and is believed complete** - but you are the
   last reader, and if a third exists it should be in the same wave rather than in a
   follow-up.

The general rule behind item 2 was written during this plan rather than after it
(`a-new-term-is-checked-against-the-corpus-in-both-directions`): a new term owes a
search for the WORD (does the corpus use it for something else) and a search for the
CONCEPT (does the corpus already name this thing). The second direction is the one
that survives review, because nothing contradicts and each sentence reads fine alone.

## Dimensions

**Spec compliance across the whole branch**, not per task: does the branch, as one
artifact, satisfy the spec sections this plan amended and the decisions D106 to D112?

**Test power, and DIRECTION rather than presence.** A mutation that removes a
mechanism proves the mechanism exists; where a behaviour has a dangerous direction
and a safe one, the mutation that earns the claim leaves it working and inverts it.
Two properties on this branch are worth attacking that way because their failure
direction is data loss: the save-marking line in the save funnel, whose only producer
anywhere is one leg of one guard case; and the four-state close decision, where a
wrong cell means no prompt over unsaved changes.

**House conformance**, citing entries by id. Several were written during this plan's
own execution and now bind it: `a-normative-claim-is-scoped-down-to-its-producers-reach`
(tier 2), `a-normative-sentence-naming-a-set-is-discharged-member-by-member`,
`an-edit-to-a-set-walks-to-its-neighbours-not-only-to-its-enumerations`,
`a-statement-living-in-two-documents-is-diffed-against-its-twin-not-swept`,
`a-disabled-assertion-over-a-disjunction-proves-only-its-weakest-term`,
`a-fire-test-on-a-two-direction-surface-attacks-the-direction-not-the-presence`,
`a-failure-cost-comment-does-not-inherit-its-neighbours-guarantee`,
`a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact`.

**Latitude, in both forms**, over the branch's normative artifacts and its code.

**The no-work-needed check, standing**: wherever any artifact on this branch
concludes something is unnecessary, run the premise rather than weighing it.

**Typography** across every string, comment and document the branch adds. German
orthography inside German values is orthography and is never a violation.

## Output contract

Findings graded Critical, Important, Minor, each with the file, what is wrong, and
how you established it. Then, separately and explicitly:

1. **Your ruling per deferred and parked item** (fix now / disposition stands).
2. **The acceptance-map walk**, row by row, with the producer you found or the gap.
3. **A merge verdict**: READY, or NEEDS_FIXES with the blocking list.
4. **A harvest section** - the patterns of this branch as a whole, which the
   controller writes into the house ledger.

## Working rules

Read-only on the repository except your verdict file. Mutate copies outside it under
the scratch path in the dispatch; where a mutation must be in-tree for the suite to
see it, restore immediately and **verify the restore by CONTENT** - `cp` and `rm` are
aliased to their interactive forms here, so use `command cp` and `command rm`.
**The end-to-end suite serves the built bundle**, so a build goes between every edit
and every run, the restore included; after any `help/` or catalog edit this is not
optional. `command grep` reaches gitignored paths where the plain shell function
cannot, and the command you paste must be the command whose output you report. Never
read an exit status through a pipeline.

Write your verdict to `.superpowers/sdd/plan-12/whole-branch-verdict.md`.
