# Plan 12 - controller addenda to the plan brief, consolidated

Four addenda reached the plan author after `plan-brief.md` was issued, as
messages. **That was a write-at-creation defect on the controller's part**: the
doctrine's rule that a reviewer verdict existing only as a subagent message is
unsalvageable by construction applies identically to a controller instruction.
They are recorded here so the requirement set is on disk, salvageable, and
checkable by a reviewer who never saw the message channel.

Order of authority: `plan-brief.md`, then this file (later supersedes earlier
where they collide), then the owner rulings recorded in `docs/ROADMAP.md`'s
"OWNER QA PASS, round 3" entry, which outrank both.

## Addendum 1 - the derivation package's shape (information duty only)

The owner sharpened the out-of-scope derivation package: the derived rules
POPULATE THE PROFILE IN THE EDITOR (the editor is the review surface) and the
profile is not yet saved at that point.

Consequence for Plan 12, and the only one: brief decision 14 is being decided for
an unsaved BLANK profile, while the later package produces an unsaved POPULATED
one through whatever mechanism this plan chooses. **Added obligation is an
INFORMATION duty, not a design mandate**: state whether the chosen mechanism
carries a populated unsaved profile, and if not, name that as a known
consequence. Building for the later package, adding a speculative seam, or
generalising beyond what the blank case needs is a finding.

## Addendum 2 - the guard's recorded shape (superseded in part by addendum 3)

The owner ruled a guard for the derivation package: with a not-new/not-empty
profile in the editor, a confirmation precedes selecting the container, stating
that the editor's profile will be overwritten. **Ordering is part of the ruling**:
confirm first, then the file dialog, then populate.

Consequences for Plan 12: decision 14 keeps only the triggers the brief named (tab
switch, opening another profile, closing the app) and does not acquire the
container trigger; where its decision produces a confirm-style guard, the
confirmation precedes the destructive action and its text names what is
overwritten; and reuse-over-near-duplication applies without earning an
abstraction the blank case does not need.

The open question this addendum carried (whether the guard extends to the other
triggers) was answered by the owner and is recorded in addendum 3.

## Addendum 3 - the discard-guard family, owner-settled

Brief decision 14 is no longer the plan's to decide. The owner ruled:

- **All guards in this family are gated on SAVE STATE** - the warning appears only
  when unsaved changes exist. This supersedes the unconditional reading in
  addendum 2 and in the ROADMAP, **including for the container trigger**.
- Opening another profile over the current one: warns.
- Switching tabs: must not affect the editor's content at all, so warns not at
  all.
- Closing the app: warns.

Three controller measurements supplied with the addendum, to be verified not
inherited:

1. **Tab switching already preserves content** - `App.vue` mounts all three views
   with `v-show`, not `v-if`, and says so in a comment. The work is to ASSERT the
   invariant with a test, not to build a mechanism.
2. **A window-close handler already exists** - `src-tauri/src/run.rs` handles
   `WindowEvent::CloseRequested` from the run slot. So the guard integrates rather
   than introduces, and **the plan must settle a decision neither the ruling nor
   the brief names**: the precedence between the two reasons to block a close, and
   whether the user sees one prompt or two when both hold.
3. **No change tracking exists** - zero occurrences of dirty/isDirty/unsaved/
   modified in `EditorView.vue`.

**Safeguard carried by the plan** (proposed, therefore not argued out during
planning): every path that mutates the profile model is enumerated FROM THE TREE
with its expression stated, and covered per path, not by one test for the flag.
Rationale: the guard's correctness is the flag's correctness, and a missed
mutation path makes the warning silently not fire, failing toward data loss.

## Addendum 4 - undo/redo added, scope then closed

The owner declined pulling the unblocking half forward and added editor undo/redo
instead, on the reasoning that change tracking is being built anyway. **He closed
the package's scope in the same breath.** This reverses his own S22 ruling
(2026-07-22) that put undo/redo wholesale in 1.x on the ground that the
explicit-save model bounds the loss at 1.0.

The requirement set already existed in writing: `docs/ROADMAP.md`'s v1.x entry
"Editor undo/redo, all operations" enumerates field edits, rule add/remove
including the deliberately unconfirmed delete, drag-reorder and list/map widget
mutations, and names the editor's single in-memory model as the natural
command/snapshot boundary.

**The undo history subsumes the save-state flag**: unsaved-changes becomes "the
current history position differs from the position at the last save". The gate is
DERIVED, not separately maintained - which makes addendum 3's safeguard cheaper
rather than doubled (one enumeration, one coverage obligation) and inverts the
failure direction from silent to visible.

**D66's premise is consumed, not reopened**: it removed the rule-removal
confirmation precisely because undo/redo is the durable answer, so the plan
introduces no removal confirmation and does not weaken spec 8.2's "Remove deletes
the selected rule without confirmation".

Seven decisions the plan must settle for this item: granularity; whether saving
clears the history or marks a position in it; a depth limit or none with its
memory consequence; keyboard shortcuts and whether they follow per-OS convention;
whether undo/redo is also surfaced as visible controls, with catalog keys in both
locales; what happens to the history when a different profile is opened; and
whether the derivation package's population would be one undoable step
(information duty only). **SI-3 binds granularity in particular** - measure
mkvtoolnix-gui rather than assuming.

## Addendum 5 - German ships in the same change, always

The plan had accepted an English-only shell dialog with a recorded reason. **The
owner rejected the acceptance and stated it generally: German translations always
ship in the same change, without exception.** So "accepted with a recorded reason"
is not an available disposition for any user-visible string this package adds.

Required: the shell's warning is localized, conforming to the existing CLI-side
house pattern rather than inventing a mechanism, and stating how the shell obtains
the locale and how that relates to the `effectiveLocale` seam; every user-visible
string the package adds is enumerated BY SURFACE from the tree with its expression
stated, and ships in both locales in the task that introduces it; and the
catalog-budget itemization covers the shell strings too, so the owner's
strike-at-approval option covers the whole package.

### Correction to this addendum, and it was the controller's error

The addendum asserted that `scripts/check-i18n.mjs` "enforces en/de lockstep for
the FRONTEND catalogs and cannot see the shell", and asked for a check closing
that gap. **The plan author refuted it and the controller reproduced the
refutation.** Check 3 in that script covers ALL `.ftl` files under `locales/en/`
by its own documented statement, `gui-common.ftl` among them, and
`locales/de/gui-common.ftl` exists - so a MISSING German shell string was already
a hard failure. The real defect is narrower and different in kind: the shell reads
`include_str!` of the **en** catalog only, so the German values that parity
already enforces ship dead.

**The check the addendum asked for would therefore have been green before AND
after the fix - it would have tested nothing.** That is the same
absence-shaped-check failure this project has a standing rule against, committed
in the instruction that demanded the check. Recorded rather than quietly replaced.

The safeguard that belongs there instead exercises the shell's own lookup: fail if
a shipped locale directory has no row in the shell's table, and derive the
consumed key set from the shell's source rather than from a hand list, asserting
every key resolves to a non-empty non-key value in every shipped locale, with
prescribed red states.

### Adjacent, routed and not swept in

`crates/muxsmith-cli/src/i18n.rs` embeds its catalogs through a HAND-WRITTEN
per-locale table (`include_str!` has no glob form), so a third locale directory
would silently go unserved there. **This is a known and registered gap, not a new
finding**: the ROADMAP Triggers section already carries "A third locale directory
is added -> the renderer's embed table gains the locale's row (D63)". What the
Plan-12 safeguard adds is that the shell's half becomes CHECKED rather than
trigger-remembered, while the CLI's half stays trigger-remembered. Controller
routing decision, not Plan 12's work.
