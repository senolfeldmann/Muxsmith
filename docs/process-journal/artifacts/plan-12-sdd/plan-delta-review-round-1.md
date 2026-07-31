# Plan 12 delta review, round 1 - same reviewer, same standards

Graded artifact: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` at
`e5cb799` (1210 lines). The version I graded in round 1 was untracked and `e5cb799`
adds the file whole (+1210), so no git diff exists between them; each finding is
therefore graded against the location and resolution my round-1 verdict named, not
against a diff.
Prior verdict: `.superpowers/sdd/plan-12/plan-review-round-1.md`.
Tree at review: `e5cb799`, working tree clean.
Independent instruments, all at
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr12-delta-independent/`:
a fresh Rust probe (`probe/`, path dependency on `muxsmith-core`, its own
`CARGO_TARGET_DIR`, so nothing was written into the repo's `target/`), a Python
catalog differ producing the en/de identical-value set, and the round-1 gate-count
expression re-extracted from the plan and diffed byte for byte against my own copy.
Settled non-findings from round 1 are not re-litigated.

---

## Overall verdict: NEEDS_FIXES

The round did the structural work rather than the patching work, and the centre of
it holds: the shell lookup now splits into a fallback-free `lookup_in` and a
chain-composing `ftl_message`, the parity check asserts on the row step and never
through the chain, each of the three parts names the red state it defeats, and the
rule that a red state producing no failure is a defect in the test closes the
wrong-assertion-satisfies-the-mutation hole for the two mutations that matter. I
traced all three red states against the split and they behave as the plan claims,
including the one my round-1 finding said could not fire. The German-rendering half
now has a producer of its own, the failed-open branch is decided in both directions
with a control that cannot be mistaken for a vacuous pass, the model gate on undo
and redo is kept for the right reason, the surface set is the union of two published
expressions with the blind spot stated, and every count I recounted matches. **What
blocks approval is one line introduced by this round's own repair.** Step 1c marks
the saved position from the LIVE history (`history.value[position.value]`) rather
than from the profile that `doSave` captured and actually wrote, so a model change
during the save-dialog gap or the write await marks the editor clean while the file
on disk holds the older profile. That is the same mechanism as F4 with the sign
flipped, it fails toward data loss where D108 decision 4 claims annoyance, no
prescribed check can see it, and the fix is one expression. Six minor items follow
it, five of them one-clause repairs and one of them the restated-count class
recurring inside the paragraph that enumerates the very checks it miscounts.

**Round-1 dispositions: 13 ADDRESSED, 2 ADDRESSED_WITH_CONCERN, 0 NOT_ADDRESSED.
New findings: 1 major, 6 minor, 2 nits.**

---

## Round-1 findings, per-finding disposition

### F1 - the Task 6 Step 3 contradiction - ADDRESSED

The paragraph is replaced. Step 3 now reads "**Both locales' values are read**,
through the locale-aware lookup Step 1b builds, so a German user reads the German
text; Step 5's part (c) pins one of them", and it disposes of the de header's
forward-looking clause explicitly ("**consumed by this task, not extended**"). The
two properties that genuinely survive - single-line and column-0 in both files, no
attributes - are carried instead, which is what the paragraph should always have
said. No sentence anywhere in the task now asserts an en-only read; I swept the
whole file for the claim and it is gone.

### F2 - both red states green - ADDRESSED

The fix is structural, as the dispatch says. Graded as a design, part by part,
against the split Step 1b prescribes (`lookup_in(catalog, key) -> Option<&'static str>`,
"the existing `ftl_message` body with its `unwrap_or` removed"; `ftl_message(key, locale)`
walking `[requested, en]` over it):

| prescribed red state | (a) directory vs table | (b) key present per row, on `lookup_in` | (c) pinned German value | plan claims |
|---|---|---|---|---|
| `de` row points at the en catalog | passes (row exists) | passes (en catalog holds every key) | **fails**: `ftl_message("close-abort-title","de")` returns `Abort running jobs` | (c) fails, (a) and (b) pass - **correct** |
| one de key deleted | passes | **fails**: `lookup_in(de_catalog, key)` is `None`, and the chain is not in the path | passes, unless the deleted key is the pinned one | (b) fails, (a) and (c) pass - **correct with a caveat, see N-F2** |
| `de` row deleted from the table | **fails**: `locales/de/` exists with no row | passes (only the en row is iterated) | **also fails**: the chain finds no de row and falls through to en | (a) fails; the must-not-fail half is not stated - **see N-F3** |

Each red state now defeats the fallback rather than disturbing its input, because
(b)'s assertion sits below the chain by construction and (c) pins a concrete value
on the far side instead of a non-empty-non-key result. The must-not-fail halves are
what close the wrong-assertion hole: if (b) were written through `ftl_message`, red
state 2 would produce no failure; if (c) were written as non-empty-non-key, red
state 1 would produce no failure; and "**A red state that produces no failure is a
defect in the test, not in the mutation**" catches both. The seam's own doc comment
is required to name the check as its reason, which keeps a later simplifier from
re-merging the two functions without meeting the argument. Two residual
under-specifications are new minors, not a reopening of F2.

I also checked the one place the chain is still asserted through - W4-n's extended
`close_abort_strings_resolve_from_the_ftl_catalog`, which Step 5 has pass `"en"`
explicitly. There the chain has no masking effect, because `en` is the last link:
deleting an en key makes both chain steps miss and `ftl_message` return the key, so
`assert_ne!(value, key)` fires. Sound, and worth recording so a later round does not
"fix" it.

### F3 - no producer for the German-rendering half - ADDRESSED

W4-t is a row of its own: "**A German user reads a German dialog** - the user-visible
half of D110, and the one an en-fallback assertion cannot see", produced by Step 5
part (c) with its red state named. D110 decision 4 marks the same part as "the
producing check for D110's user-visible half ... which no other check in this package
covers", and the self-review's halves paragraph names it as the sixth localization row
"because it is the only one an English fallback cannot satisfy". The half is covered
and the reason it needed its own row is on the page.

### F4 - `savedSnapshot` never written on save - ADDRESSED_WITH_CONCERN

Addressed as a coverage and Files-list matter, and correctly:

- Task 4's Files entry gains "**`doSave`'s post-write region** - the one line that
  marks the saved position, and the only part of `doSave` this task touches", so the
  exhaustive enumeration now admits the region.
- Step 1c fences the write, states the insertion point ("Immediately after
  `await saveProfile(path, profile)` resolves"), and places it **inside the existing
  `try`, so a failed save leaves the state dirty** - which is the behaviour that
  matters and is right.
- W3-q now says out loud that it "**passes whether or not the saved position moves**,
  which is why it is not the whole observable", and W3-q2 carries the other half.
- The consequence of omission is stated where an implementer will read it: "Without
  it `dirty` never returns to false and every guard in the D109 family fires on a
  profile that was just saved, which is the disposition the owner overruled".

**The concern is the expression itself and it is finding N-F1 below:** the line marks
the live position rather than the written profile, which is a new defect in the
opposite direction. Placement, fencing, Files-list admission and coverage are all
correct; only the value assigned is wrong.

### F5 - failed-open history and Undo with no model - ADDRESSED

Both halves are decided and both are fenced.

- D108 decision 9 rules the failed-load branch: history empty, `savedSnapshot` null.
  I checked the four consequences it claims arithmetically against the definitions in
  Step 1: `sessionActive` false, `dirty` false, `canUndo` (`position > 0`, i.e. `-1 > 0`)
  false, `canRedo` (`position < history.length - 1`, i.e. `-1 < -1`) false. All four hold.
  `resetHistory(profile: Profile | undefined)` has both branches fenced and is called
  "with `doc.profile ?? undefined` on the same value it assigns to the model, so the two
  can never disagree", which removes the invention.
- D108 decision 10 gates undo and redo on `model` in the functions and in the buttons'
  `:disabled`, and keeps the gate although decision 9 makes it unreachable, on the
  ground that "'currently unreachable' is precisely the claim a later change
  falsifies". **That is `proc-proposed-safeguard-stays` applied correctly**: the
  safeguard is retained with its reason recorded rather than argued away, it costs one
  term in two conditions, and it is listed in the plan's own may-not-be-argued-away
  set. I confirm the application and I do not recommend removing it.
- W3-u covers it, and Step 6's failed-open case carries its own control - "the state
  before the failed open, where Undo was enabled in the same test - so a test that
  passes because Undo is never enabled anywhere cannot be mistaken for this one
  passing." That is the right control for a check whose green state is four disabled
  things.

One ordering detail did not travel with the fix: finding N-F7.

### F6 - L1's two figures - ADDRESSED

Task 2 Step 6 now reads "exactly **2** lines, both in `src/main.ts` - `resolveLocale`'s
try branch and its catch branch", and the soundness control "must return exactly **2**
lines from `src/i18n/index.ts` on the end state - the pre-existing occurrence in
`primarySubtag`'s doc comment plus Step 1's `return saved ?? navigator.language;`".
Both match my measurement exactly, and I verified the anchor: the pre-existing
occurrence is indeed inside `primarySubtag`'s doc comment. The self-review's
restatement of the same figure was not swept - finding N-F4.

### F7 - the string-surface instrument - ADDRESSED

Replaced rather than patched, and honestly. The set is now the union of E1 (the
loader-call form) and E2 (the path-literal form), each published with its output and
its blind spot: "E1's blind spot, and it is where a real surface hides: the pattern
requires the path on the same line as the opening paren ... which is exactly the
frontend's own loader", and "E2 is blind where E1 is not: `join(ROOT, "locales")` has
no trailing slash, so E2 misses the gate's own three sites."

I re-ran both. E1 returns nine lines in four files, matching the plan's description
and my round-1 measurement. E2 returns `src/i18n/index.ts:18` - the wrapped argument
array - plus the CLI, shell and help sites and the classified noise the plan names.
The union covers all five surfaces, and my own multi-line-tolerant sweep from round 1
finds no sixth. **The carry-through the dispatch asked about is present and correct**:
"**Neither control proves completeness** - a fired control shows the pattern works, not
that the search surface was whole - which is why the set is the UNION of two
differently-shaped expressions plus one member neither can reach". The same
distinction is drawn at the mutation enumeration ("The blind spot this expression has,
checked rather than assumed") and in the self-review's absence-check paragraph. I
looked for a place where a fired control is still offered as completeness evidence and
found none.

### F8 - four stale counts - ADDRESSED

All four fixed, each verified against its own enumeration by my recount: Task 1's
Interfaces now says "the **five** decision records (D106-D110)"; the corrections lead
sentence says "None of the **seven**"; the close's ROADMAP bullet says "the corrections
table's **seven** items"; the self-review says "**41** requirements (counted from the
requirement table's rows, highest `R41`)". One further site carrying a superseded
figure was created by this round's own fixes - N-F4.

### F9 - one locale assertion counted as two - ADDRESSED

The authoring bullet is retitled "The existing locale assertions - **TWO** of them" and
names both with their mocks (`localeSelect` under the mock default's `"en"`,
`reloadedLocaleSelect` under the case's own `DE_SETTINGS`). W1-h now reads "the **two**
EXISTING `smoke.spec.ts` assertions", and Task 2's Step 6 report duty says "Report
**both** existing `smoke.spec.ts` locale-control assertions by name". The shared
disposition is stated once and correctly: shape A changes the display of neither.

### F10 - the ruled README-example disposition - ADDRESSED

Correction 7 now carries "**Its disposition is already RULED and is not this plan's to
route**", names the ROADMAP section, the owner's ruling and the Plan-11 vehicle, and
keeps the measurement in the only role left to it: "an independent confirmation of a
record that predates it, which is also why the seed's own `pattern` is written out
rather than defaulted." The item is gone from the close's harvest list. That is the
right disposition and it also strengthens the seed's justification.

### F11 - `ConfirmDiscard`'s exit code - ADDRESSED

Step 2 fences "exactly `app.exit(0)` for `ConfirmDiscard`" and states why the literal
is written down: "the neighbouring site passes one through a closure ... so an unwritten
literal would be an invented value."

### F12 - no absence check for R28 - ADDRESSED

Absence check D1 exists, with a pre-state whose control is the mkvtoolnix comparison
already measured, an end-state expression distinct from the pre-state one (necessary,
since the pre-state pattern must match after the task), and its own fire "against a
synthetic line carrying `dirty.value = true` to prove it matches an assignment when one
exists". D1 joins the absence-check enumeration, which is now eight. One residual is a
nit, N-N1.

### F13 - the 430-byte figure - ADDRESSED, and my diagnosis was refuted correctly

**The refutation is right and my proposed cause was wrong.** I proposed that 430 was
the serde-normalized wire form and that only the method was missing. The author's
measurement shows the cause was a truncated fixture - the extracting range dropped the
example's last line - and the wire form is **419**, not ~430.

I reproduced 419 independently, through core's own loader in my own probe crate:

```
seed.yaml:        from_file+to_string = 101 bytes
readme4rule.yaml: from_file+to_string = 419 bytes
```

The three figures are kept in genuinely distinct units rather than reconciled: the
authoring bullet names the method for the load-bearing one ("compact, through
`muxsmith-core`'s own `load` plus `serde_json::to_string`"), gives 419, and then states
"For comparison, and to keep the units apart, compact JSON of that YAML block as
written is 492 bytes and 507 with the pattern line added" - both of which are my
round-1 figures exactly. D108 decision 5 carries 419 with a pointer to the method
rather than a second copy of it. The self-correction is recorded as an instrument
defect of the same class as the surface-set finding, which is the correct
classification.

### N1 - "unreachable in a real session" - ADDRESSED

D110 decision 3 now reads "It is not reachable in practice, though not impossible in
principle - the window is shown by the shell rather than by the webview's bootstrap, so
a close request in the first frames is conceivable". That is the accurate form.

### N2 - the two asymmetries with no row - ADDRESSED_WITH_CONCERN

The acceptance map gains a preamble naming all three cases (W2's and W4a's catalog
coverage, the New-before-Open ordering, the two reworded values) so the asymmetry does
not read as a gap. Two of the three are answered by pointing at coverage that exists.
The third is answered differently: "'New renders immediately before Open', the
SI-3-derived ordering, has no assertion because DOM order is the one property the
parity precedent fixes and no test in this suite asserts sibling order." That is a
truthful statement of a gap rather than a closure of it - an SI-3-derived decision
with no producing check. I raise no finding: it was a nit, the asymmetry is now
declared rather than silent, and a sibling-order assertion is cheap but would be the
only one of its kind in the suite. Recorded as a concern so a later round does not read
the preamble as coverage.

---

## Ruling on the divergence

**The divergence is ACCEPTED, and it is better than what I prescribed.**

Thank you for the attribution correction; it changes what I am judging. My round-1
F4(b) said "assert `dirty`'s observable proxy in the same test rather than deferring it
to Task 6", and the author put the proxy in Task 5 instead. Judged on the merits:

1. **My prescription was under-argued on one point the author is right about.** At the
   end of Task 4, `dirty` has no user-visible reader: the guards do not exist until
   Task 5 and the shell sync does not exist until Task 6. A Task-4-local assertion
   would have to observe `dirty` through something the product does not have. The only
   routes are exposing it through the mount harness or asserting an internal via
   `page.evaluate`, and both add a test-only observation surface to ship a check - which
   is a mechanism the product does not need, in a plan whose whole posture is that the
   editor's state is observed through what it renders. My phrase "same test" carried an
   assumption I did not check, namely that Task 4 had a surface to read.
2. **The replacement is strictly stronger than a Task-4 proxy would have been**, because
   it is the real consequence rather than a stand-in. Task 5 case 3 leg (ii) makes the
   two Open clicks each other's control: the first (after an edit, before the save)
   proves the guard can fire in that scenario, the second (after the save) proves the
   state cleared. A frozen `savedSnapshot` fails leg (ii) and passes everything else -
   which the plan says in as many words: "Without leg (ii) a `savedSnapshot` frozen at
   the load baseline ships silently, because every other assertion in this package
   passes with it frozen."
3. **The one-task delay costs nothing that matters.** Task 4's own reviewer still has
   Step 1c fenced in the plan and in Task 4's Files list, so the omission is a review
   finding at Task 4 even without a red test; and the tasks are strictly serial, so the
   window in which the defect could exist unnoticed is one task wide with no merge in
   it.
4. The row is honest about being cross-task: W3-q2's producer column says "**Task 5**"
   in bold and states the reason inline, so nobody reading the map thinks Task 4 covers
   it.

I therefore do not hold that a Task-4-local assertion is required, and I withdraw that
half of F4's prescribed resolution. Recorded so a later round does not re-impose it:
the check for a derived value belongs at its first real reader, not at its definition,
when the alternative is a test-only surface.

---

## New findings

### N-F1 (MAJOR) Step 1c marks the live position, not the profile that was written

**Location.** Task 4, Step 1c; D108 decisions 3 and 4.

**What is wrong.** The fenced line is
`savedSnapshot.value = history.value[position.value];`. `doSave` captures what it will
write before its awaits - Task 3's fence: `const profile = model.value;` - and there are
two awaits between the capture and Step 1c's insertion point: the save dialog on the
needs-path branch, and `await saveProfile(path, profile)` itself. If the model changes
inside either window, the watcher pushes a new entry and advances `position`, and Step
1c then marks the NEW state as saved. `dirty` becomes false while the file on disk holds
the older `profile`. The guard family disarms over unsaved changes, and the app quits
without warning.

**Why this is not a theoretical window in this codebase.** The capture discipline Step 1c
sits inside exists precisely because that window is real, and the house comment it
conforms to says so about the very same dialog: `RunHistory.saveLog` - "the native save
dialog can stay open indefinitely, and the user may select a different job meanwhile".
`saving.value = true` disables `editor-save`, `editor-new` and `editor-open`, not the
widgets, so the editing surface stays live through both awaits.

**It contradicts an explicit claim and its own parity precedent.** D108 decision 4: "Its
failure direction is annoyance, never data loss - a spurious dirty warns where nothing
was at risk, a hand-set boolean would silently fail to warn where something was." This
line produces exactly the second behaviour. And the precedent Step 1c cites licenses the
live read only because it has no gap: in `mkvtoolnix-gui`'s `Tab::onSaveConfig` the
sequence `updateConfigFromControlValues(); p.config.save(); p.savedState = currentState();`
is fully synchronous, so there `currentState()` **is** what was written. Translating it
into an async flow requires marking the written value, not the current one.

**No prescribed check sees it.** Leg (ii) of Task 5 case 3 - the round's own new
safeguard - edits, saves, then clicks Open with no edit during the gap, so
`history[position]` and the captured profile are the same string and the leg passes.
Every other assertion passes too. This is a second instance of the shape F4 named: a
two-sided mechanism whose covered side cannot fail.

**Ruling.** FIX, one expression: `savedSnapshot.value = JSON.stringify(profile);`. It is
also simpler and self-evidently correct - the same value that was written, in the same
serialization the history uses. When nothing moved during the gap, `profile` is the same
object reference the last push serialized, so the string equals `history[position]` and
`dirty` is false, as intended. When the model did move, the two differ and `dirty` stays
true, which is the annoyance direction D108 decision 4 promises. Add one clause to Step
1c stating that the written value is marked rather than the live one, and why the parity
precedent's synchrony does not carry over. Consider one acceptance half for it (edit
during the dialog gap leaves the editor dirty) - I do not require it, since the fix
makes the property structural rather than behavioural.

### N-F2 (MINOR) Red state 2 does not name which de key is deleted, and its must-not-fail half depends on the choice

**Location.** Task 6, Step 5, third bullet: "delete one de key - **(b) fails**, (a) and (c)
still pass".

**What is wrong.** (c) pins `close-abort-title`. If the implementer deletes that key to
produce red state 2, (c) fails as well and the stated must-not-fail half is falsified -
which, under the plan's own rule that an unexpected result returns as NEEDS_CONTEXT,
costs a round trip on a non-finding. The mutation is an unnamed member of a set in a
normative position, and the must-not-fail half is what makes the omission bite.

**Ruling.** FIX by naming the key. `close-discard-title` is the better choice than any
pre-existing one: it is a key this package adds, so the red state exercises the new
surface, and it is not the pinned key, so (a) and (c) pass as stated.

### N-F3 (MINOR) Red state 3's matrix is incomplete, and (c) also fails under it

**Location.** Task 6, Step 5, third bullet: "delete the `de` row from the table - **(a)
fails**", with no must-not-fail half where the other two red states have one.

**What is wrong.** Traced against Step 1b's chain: with no `de` row, `ftl_message("close-abort-title","de")`
finds no requested row, falls through to the `en` row, and returns `Abort running jobs`,
so **(c) fails too**. (b) passes, because it iterates the rows that exist. The plan does
not claim otherwise, so it is not wrong - but an implementer who sees two failures where
one was named has no statement telling them whether that is expected, and the plan's own
disposition rule covers only the opposite case ("A red state that produces no failure is
a defect in the test").

**Ruling.** FIX by completing the row: "(a) fails, and (c) fails with it because the chain
then falls through to en; (b) passes." Two failures from one mutation is a property of the
design worth stating, not a defect.

### N-F4 (MINOR) The self-review still carries L1's superseded figure

**Location.** The self-review's absence-check paragraph: "L1 (the single resolution rule)
fires on its pre-state run of exactly one line".

**What is wrong.** Task 2 Step 6 was corrected to 2 (F6). The self-review's restatement was
not. This is the restated-count class in its exact shape - correct where it is computed,
stale where it is restated - inside the paragraph that enumerates the eight absence checks,
in a document whose round produced the house rule about it. I am not re-harvesting the rule;
I am reporting a live instance of it created by this round.

**Ruling.** FIX. Either state the figure ("two lines, one per branch of `resolveLocale`") or
drop the number and point at the step that computes it, which is what the rule prescribes.

### N-F5 (MINOR) Task 2 Step 5's `de()` branch does not defeat the frontend fallback for every id

**Location.** Task 2, Step 5: "Every asserted German string comes from the file's existing
`de()` helper **or** is a literal that exists ONLY in de".

**What is wrong.** Only the second branch is fallback-defeating in general. `de(id)` formats
the id through a de bundle and compares against the rendering; where the de value equals the
en value, the assertion passes even if the interface fell back to English entirely - which
is the frontend instance of the handle the Global Constraints now carry. I measured the set:
**18 gui-* ids have identical en/de values**, among them `settings-locale-option-en = English`
and `settings-locale-option-de = Deutsch`, both in the very dialog W1 tests.

**The prescribed assertions are safe in fact**, which is why this is minor: Case 1 uses the
batch heading (`batch-view-heading`, en `Batch` / de `Stapel`) and the de
`settings-locale-option-system` (`System language` / `Systemsprache`); Case 3 uses the de and
en headings. None lands on a fallback-blind id. It is the permission that is wider than its
safe set.

**Ruling.** FIX by scoping the disjunction: assert through `de(id)` only where the de value
differs from the en value, which is measurable, and note that the option labels for English
and German are among the ids for which it does not hold. The plan already applies the right
rule at W1-a; Step 5's wording should match it.

### N-F6 (MINOR) W4-o claims a concrete expected tag; its producing step still says "the applied locale"

**Location.** Acceptance row W4-o versus Task 6, Step 6.

**What is wrong.** W4-o reads "asserted against the concrete expected tag rather than against
'whatever was applied'". Step 6, the step it names as its producer, reads "a recorded
`set_shell_locale` call at startup carrying the applied locale". The live half of the same
step is concrete (`"de"`); the startup half is not, and an implementer following the step
writes the self-referential assertion the row says was replaced. The value is determined -
under `smoke.spec.ts`'s mock default `get_settings` returns `locale: "en"`, so
`effectiveLocale("en")` is `"en"` and `"en"` is what the shell must be told - so this is an
unwritten value, not an open decision.

**Ruling.** FIX by naming `"en"` in Step 6 with the one clause that derives it from the
scenario's mock, so both halves of the row are concrete.

### N-F7 (MINOR) `resetHistory`'s position relative to the model assignment is not carried into Task 4

**Location.** Task 4, Step 1's `resetHistory` bullet, against Task 3, Step 2's fenced comment.

**What is wrong.** Task 3 fences `sessionActive.value = true` before the model assignment and
calls the order load-bearing in a comment the implementer writes: "`sessionActive` is set
before the model, so the watcher that fires on the assignment validates the seed instead of
returning early." Task 4 replaces that assignment ("drop their `sessionActive.value = true`
in favour of establishing the baseline") without restating the ordering requirement.
`resetHistory` must run before the model assignment for two reasons: `sessionActive` must
already be true when the watcher fires, and the push rule must see `history[0]` equal to the
serialized model so the load does not push a second entry. Placed after, a created or opened
profile is never validated.

Contained rather than dangerous: the existing served-app cases in
`e2e/editor-rule-add-remove.spec.ts` queue a `validate_profile_model` response for "the
open's own model assignment" and assert on it, and W2-b asserts the same for New, so the
suite catches it. But the plan fences ordering at this level of detail everywhere else, and
it is deleting the sentence that carried this one.

**Ruling.** FIX with one clause in Step 1's `resetHistory` bullet: it is called at the site
Task 3's `sessionActive` assignment occupied, before the model assignment, and Task 3's
load-bearing-order comment is updated to name it.

### N-N1 (NIT) D1's end-state alternation is a name list

D1's end state is
`grep -nE "dirty\.value *=|(isDirty|unsavedChanges|modified) *= *ref\(" src/views/EditorView.vue`.
The first alternative is structural and will catch any reassignment of the derived computed.
The second is three plausible names, so a second boolean called something else escapes.
Worth one clause acknowledging that the check is exact for the derived value and heuristic
for a rival name; not worth widening, and per `proc-proposed-safeguard-stays` I am not
suggesting its removal.

### N-N2 (NIT) Part (c) pins a pre-existing key, so the six new German values are not pinned

(c) pins `close-abort-title`, which proves the shell reads the de catalog at all - the
mechanism, which is what D110 needs. It does not detect a new de value accidentally copied
from en, and no check in this repo does that for any catalog (the frontend gate checks id
parity, not value distinctness), so the depth is house-consistent. Worth naming as a residual
beside the two Step 5 already names, so a later reader does not read (c) as a translation
check.

---

## Reproductions

All figures from my own instruments at the delta path above.

| # | Claim | My measurement | Verdict |
|---|---|---|---|
| D1 | 66 acceptance halves, 10 + 12 + 22 + 20 + 2 | 10, 12, 22, 20, 2; total 66 | reproduced |
| D2 | 41 requirements, highest `R41` | 41 rows, max `R41` | reproduced |
| D3 | 7 corrections | 7 rows | reproduced |
| D4 | 5 ADRs (D106-D110) | 5, and the previously stale Task-1 site now says five | reproduced |
| D5 | 8 absence checks (L1, D1, E1, E2, U1, G1, G2, H1) | 8 named, each with a fire; D1 is the new one | reproduced |
| D6 | The gate-count expression returns 1 and was not narrowed | Extracted the expression from the plan and diffed it byte for byte against my round-1 copy: **IDENTICAL**. Run over the file: **1** hit, the self-audit sentence. The 1 -> 4 -> 1 history is consistent with removal by rewording | reproduced; **pattern not narrowed** |
| D7 | The seed serializes to 101 bytes in the wire form | 101, through `muxsmith_core::profile::load::from_file` + `serde_json::to_string` in my own probe crate | reproduced |
| D8 | The README four-rule example is **419** bytes in the wire form | **419**, same probe. The block is 23 lines and its last line is the `changes: { track_name: German forced, ... }` line, consistent with a truncating range having dropped it | **reproduced; my round-1 diagnosis refuted** |
| D9 | 492 and 507 are kept beside it as a different unit | 492 as written, 507 with the pattern line - identical to my round-1 figures, and the plan labels all three by method | reproduced |
| D10 | L1's pre-state is 2 lines, both in `src/main.ts`; the control's end state is 2 in `src/i18n/index.ts` | 2 and 2; the pre-existing occurrence is inside `primarySubtag`'s doc comment as the plan states | reproduced |
| D11 | E1 returns nine lines in four files and misses the frontend loader; E2 reaches `src/i18n/index.ts:18` and misses the gate's `join(ROOT, "locales")` sites | both reproduced exactly; the union covers five surfaces and my independent multi-line sweep finds no sixth | reproduced |
| D12 | The three red states behave as the matrix claims | traced against Step 1b's split: red state 1 and 2 exactly as claimed; red state 3 fails (a) **and** (c), which the plan does not state (N-F3) | reproduced with one divergence |
| D13 | W1's de-only literals defeat `buildBundles`'s chain | measured the fallback-blind set: **18** gui-* ids have identical en/de values. None is used by a prescribed W1 assertion; `batch-view-heading` is `Batch`/`Stapel` and `settings-locale-option-system` will be `System language`/`Systemsprache`. The permission in Step 5 is wider than its safe set (N-F5) | reproduced in fact, over-wide in wording |
| D14 | Step 1c is placed inside the existing `try` after the write | confirmed; and Task 3's capture is `const profile = model.value;`, which is what makes N-F1 bite | reproduced; new finding |

Nothing else diverged. No round-1 non-finding changed under re-examination.

---

## The handle, run over the rest of the plan

The dispatch asked me to run the fallback handle myself rather than accept the author's
negative claim. I walked every prescribed check and asked whether a fallback clause sits
between the thing mutated and the thing asserted.

**Fallbacks that are in a check's path, and how each is answered.**

1. **The shell's `[requested, en]` chain.** Answered structurally: (b) calls `lookup_in`, (c)
   pins a concrete far-side value. The one place the chain is still asserted through, W4-n
   with locale `"en"`, is safe because `en` is the chain's last link, so a missing key returns
   the key and the existing `assert_ne!` fires.
2. **`buildBundles`'s `[requested, en]` chain in the frontend.** Answered by de-only literals
   at W1-a/b/e/g, stated at the producers. The wording that admits a weaker branch is N-F5;
   the assertions actually named are sound.
3. **`e2e/mocks.ts`'s default `get_settings`.** Named as the third instance in the Global
   Constraints. Verified the direction: the new scenario supplies `locale: null` explicitly and
   asserts de-only strings, so a fixture that silently reverted to the default `"en"` would fail
   rather than pass.
4. **`effectiveLocale`'s `??`.** The fallback is the thing under test in W1-a/c, and the
   assertion is a de-only rendering, so a broken seam fails. Sound.
5. **`ftl_message`'s terminal return-the-key.** Asserted directly by the pre-existing
   never-prefix-match test. Sound.
6. **`rememberRecentProfile`'s swallow-and-return-null.** Downstream of what W2-j asserts (the
   recorded `set_settings` call), so it cannot mask the mutation. Sound.

**One fallback the plan does not name, checked and found not to bite.** `e2e/mocks.ts`'s mock
queue "entries are consumed per call and the last one repeats" is fallback-shaped: an
unexpected extra call silently receives the previous response. I walked the checks that could
be affected - the six mutation-path cases, the granularity flow, the depth-cap case, W3-q2's
two Open clicks - and in each the repeated value is a diagnostics response while the assertion
is on rendered control values, Undo enabled-ness or the presence of the confirm dialog. No
prescribed assertion sits downstream of it. Recorded because the honest output of running a
sweep includes the site that turned out clean.

**Absence checks with no fallback in their path**, verified individually: L1, D1, E1, E2, G1,
G2, U1, H1 are greps or DOM counts; Task 1's three sweep expressions carry a must-return rather
than a zero. No further site needs the handle.

---

## Harvest for the controller

The fallback rule and the restated-count rule are ledgered, so they are not re-harvested. Three
items remain, all new to this round.

1. **The async translation of a synchronous parity precedent is its own defect class**, and
   N-F1 is its first recorded instance. `Tab::onSaveConfig` licenses "mark the current state
   after the write" only because nothing can intervene between the two statements. Copied into a
   flow with an `await` between the capture and the mark, the same shape inverts the failure
   direction. The readable trigger: **the precedent's two statements are adjacent and yours are
   separated by an await.** Worth a Tier-2 candidate on its face - the project has now imported
   three mkvtoolnix behaviours by parity (New before Open, Save-as fallback, the save-state
   comparison) and this is the first where the reference tool's synchrony was load-bearing and
   unstated.
2. **A two-sided mechanism whose covered side cannot fail has now appeared twice in one plan** -
   F4's W3-q, and N-F1 where the round's own replacement safeguard also cannot see the new
   defect. Both were found by asking of a producer "would this pass if the mechanism were
   broken?" rather than by asking "is there a producer?". That question is a sharper instrument
   than the halves count and is not currently a recorded handle anywhere; the halves rule tells
   you to split the observable, not to test whether each side's check can fail.
3. **Deleting a sentence that carries an ordering requirement is a distinct sweep target.**
   N-F7's shape: Task 4 legitimately removes Task 3's `sessionActive` assignment and inherits its
   ordering constraint without inheriting the sentence that stated it. The trigger is readable -
   **you are replacing a statement whose own comment calls its position load-bearing** - and it is
   a near neighbour of the ledgered restated-count rule, since both are about what a later edit
   owes to an earlier statement it supersedes.
