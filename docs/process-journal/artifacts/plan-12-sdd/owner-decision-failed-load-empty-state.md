# Owner decision, parked: what the editor shows after a profile fails to load

**Status: RULED 2026-07-31 - option A.** The ruling and its routing are in
`owner-ruling-1-failed-load-empty-state.md`; this file stands as the costed
record of the three options and is no longer a parked question.

**Status before the ruling:** parked for Şenol. Not blocking - Tasks 4 and 5
proceed; the fix lands wherever he rules, and Task 5 already touches the same
recents surface, so it is a natural vehicle.

**Why this is his and not the controller's:** the options differ in what the
user sees, and one of them costs a catalog key against a budget he approves per
key. Everything else in this finding is settled.

## The state, and what it shows today

A user opens a profile file whose YAML does not parse. The load resolves, the
diagnostic renders, and no model enters the editor.

Three surfaces are on screen at once, each gated on a different fact:

- **"Selected profile: `<path>`"** - gated on a path having been set, which a
  failed open still does.
- **"No profile open. Create one with New profile, or choose an existing profile
  file."** - the empty state Task 3 introduced, gated on there being no model.
- **The recent-profiles list** - gated, since Task 3, on there being no model.
  Before Task 3 it was gated on the path, so it stayed hidden after any open.

So the user is told simultaneously that a profile is selected and that none is
open, and a list that used to disappear after opening anything comes back.

**What is NOT in question:** the parse error itself still renders. D107 decision
7 ruled that deliberately, and it is right.

**The measurement that decides the cost of the options:** the rendered error is
`parse-error = The profile could not be parsed: { $detail }`. It carries a
detail, **not the file path**. The only place the failing file's name appears on
that screen is the "Selected profile" line.

## Options

**A. Keep the path line; hide the empty state and the recents list after a
failed open.** They would show only before anything has been opened or created
at all. The user sees the file name and the error, and reaches a different
profile through the Open button, which is always in the action row.
*Cost:* one term in two conditions. Plus one thing that must be decided with it:
Task 4 changes what "a session is active" means (a failed load clears it), so
these two gates need an explicit definition rather than inheriting one, or the
contradiction returns two tasks later.
*Loses:* the recents shortcut in that state - re-picking costs a click through
Open instead of a click on the list.

**B. Keep the empty state and the recents list; drop the path line after a
failed open.** The user sees "no profile open", the error, and a list to pick
from.
*Cost:* the file name disappears from the screen entirely, because the error
message does not carry it. On a batch of similarly-named profiles that is a real
loss.

**C. Keep both, and give the failed state its own sentence** instead of the
generic empty-state text - something that says this profile could not be opened.
*Cost:* one new catalog key in both locales, against the editor budget you
approve per key (this package already takes it from 46 to 54), plus its German
translation. It is the only option that adds a string.

## Recommendation: A

It keeps the one place the failing file is named, it adds no string to a budget
under pressure, and the action the user most likely wants next - open a different
profile - is one click away regardless. The tradeoff is honest: the recents
shortcut is not offered in that one state.

If you would rather the recents list stay reachable there, B and C both do that,
and C is the only one that also keeps the file name - at the price of a key.
