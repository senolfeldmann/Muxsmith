# Owner ruling 1: what the editor shows after a profile fails to load

**Ruled:** 2026-07-31, by the owner, in session 32's opening message.
**Ruling:** **option A**, the recommendation on record in
`owner-decision-failed-load-empty-state.md`. That memo is superseded as a
parked question and stands as the costed record of the three options.

## What was ruled, in the memo's own words

The memo's option A, quoted verbatim:

> **A. Keep the path line; hide the empty state and the recents list after a
> failed open.** They would show only before anything has been opened or created
> at all. The user sees the file name and the error, and reaches a different
> profile through the Open button, which is always in the action row.
> *Cost:* one term in two conditions. Plus one thing that must be decided with it:
> Task 4 changes what "a session is active" means (a failed load clears it), so
> these two gates need an explicit definition rather than inheriting one, or the
> contradiction returns two tasks later.
> *Loses:* the recents shortcut in that state - re-picking costs a click through
> Open instead of a click on the list.

The owner accepted the option as costed, including its named loss and the
condition attached to it. Nothing in the memo's option B or C was chosen, and no
new string is authorized: the ruling's own economy is that A adds none.

## What the ruling does NOT touch

- The parse error still renders. D107 decision 7 ruled that deliberately and the
  memo puts it outside the question.
- The "Selected profile: `<path>`" line stays, in every state where it renders
  today. It is the only place the failing file is named, which is the
  measurement that decided the option.
- No decision of D106 to D110 is re-opened beyond what recording this ruling
  requires.

## Routing, and why it is a plan amendment

The ruling arrives mid-run, changes user-visible behaviour, and lands in a task
that has not started. That is the doctrine's amendment case, not a controller
constraint passed into a dispatch: the gate condition it needs is plan content,
and the controller does not author plan content.

**One pair, not two.** The amendment changes what a task contains; it adds,
removes and re-cuts no task. Per the doctrine's role-count rule, one author and
one reviewer cover both the design half and the plan half.

**Fresh pair, not resumed.** The doctrine names the resumed original author and
the resumed original reviewer for an amendment. Both authored in an earlier
session and cannot be resumed across the session boundary, so this amendment
runs with a fresh author and a fresh independent reviewer against a controller
brief. Stated here so the deviation is on the record and not read as a shortcut.

**Top tier for both roles.** The plan's model-tier table assigns tiers to its
seven implementation tasks and says the top tier serves only the whole-branch
review; that sentence is scoped to the task table it heads. An amendment is a
design-and-plan four-eyes round, which the doctrine's execution spine puts on
the strongest model. Controller decision, no owner criterion met.

Brief: `amendment-1-brief.md`.

## Order of authority

Where this file and an earlier artifact disagree about the failed-load state,
this file wins. It is below the owner-approved plan and the v1 spec on
everything else.
