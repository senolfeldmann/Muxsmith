# Plan 12 delta review, round 2 - same reviewer, same standards

Graded artifact: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`, working
tree, 1237 lines, untracked.
Baseline: the same file at `e5cb799` (1210 lines), extracted to my own path and diffed - 11
hunks, 12 replaced lines, 39 added.
Prior verdicts: `plan-review-round-1.md`, `plan-delta-review-round-1.md`.
Repo HEAD at review: `5914352`.
Independent instruments, all at
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr12-delta-r2-independent/`:
the extracted baseline and full diff, a three-unit catalog differ, five synthetic `.ts`
fixtures probing S1's expression, a Python paragraph-member differ over the three
enumerating paragraphs, a Files-list-versus-`git add` comparator over all seven tasks, and
the gate expression re-extracted and diffed against my round-1 copy.
Settled non-findings are not re-litigated: F2's split and red states, the shell locale
route, F5's unreachable model gate, the 419-byte figure, the task cut and the sequencing
were confirmed in earlier rounds and were not re-examined except where this round's diff
touched them.

---

## Overall verdict: NEEDS_FIXES - narrowly, and no design change is implied

All nine round-2 findings are addressed, several of them more thoroughly than I asked: N-F7's
fix supplies a third reason for the ordering requirement that I had not identified (a
`resetHistory` called after the model assignment would let the push rule append a second
entry, so a freshly opened profile would start "one step deep and dirty"), and N-F1's fix
carries the failure-direction reasoning into D108 decision 4 as a stated precondition rather
than leaving the claim standing unqualified. The precedent sweep is real work with a real
result, and I judge all three non-trivial classifications sound. **Two mechanical items block
approval, neither touching a design decision.** The first is the one my own standing duty
turned up: the modality classification concludes the shell's read-then-ask gap is benign over
an enumeration of two directions, and there is a third - a clean-at-read close with a run
active, edited while the OS dialog is up - which loses unsaved changes with no dialog having
mentioned them, against R23 and in the data-loss direction. The second is measured rather
than argued: S1 catches the exact regression it was written for and one variant of it, and
misses the semantically identical `JSON.stringify(model.value)` spelling, which is the more
likely re-break given the shape of the fix it guards. Both are one-paragraph or
one-expression repairs; I would approve on sight of them, and I recommend the controller
route a minimal fix round rather than another full pass.

**Round-2 dispositions: 9 ADDRESSED, 0 ADDRESSED_WITH_CONCERN, 0 NOT_ADDRESSED.
New findings: 2 minor, 1 nit.**

---

## Ruling on the divergence: I agree with the author, plainly

The requirement was yours and the author diverged from it with evidence. I verified the
evidence at the source myself rather than taking either of your readings, and then asked
whether the substitute is proportionate. My answer to all three of your questions favours the
author.

**The harness constraint is real, and it is stronger than the plan states.** `e2e/mocks.ts`
carries the comment the plan quotes, verbatim: the function body is serialized via
`page.addInitScript(fn, arg)` and "must not close over anything from this module's scope
beyond the `scenario` argument itself". And `MockResult` is a plain discriminated union of
`{ kind: "resolve", value }` / `{ kind: "reject", error }` - pre-computed values that
`nextResult` shifts off a queue - so there is no promise the test retains a handle to. To hold
a mocked `plugin:dialog|save` open you would need a new scenario kind plus a release channel;
the file already uses `page.exposeFunction` for invoke recording, so the seam exists, which is
exactly what makes a releasable response *infrastructure* rather than a scenario.

I also tested the escape the plan does not consider, because "a Playwright action after the
click races the microtask" understates the problem and I wanted to know whether a
single-round-trip formulation escapes it. It does not. Driving the click and the model
mutation inside one `page.evaluate` body would put the mutation in the same task as the
click - the handler suspends at `await saveDialog(...)` and control returns to the body - but
the mock's promise continuation was queued when `saveDialog` was called, i.e. *before* the
input dispatch, while Vue's watcher flush is queued *after* it. Whether Step 1c or the history
push runs first then depends on how many microtask hops the mock chain has, which is
unknowable from the test and free to change. So the race is a microtask-ordering race that
survives the obvious workaround, and it is worse than the CDP race the plan names: a test
built on it would pass or fail on scheduler internals. **The owner's no-flakiness call
forecloses it, and the author's conclusion is right for a stronger reason than the one
given.**

**Question 1 - is S1's fire anchor-exercising or does it only prove the grep works?**
Anchor-exercising. The synthetic it fires against is `savedSnapshot.value = history.value[position.value];`,
which is not a proxy for the defect - it *is* the defect, character for character. The fire
therefore demonstrates that the check would have caught the exact regression this plan
committed, which is the strongest form available to a lint. The plan is also honest about what
that buys: "Its pre-state is by construction empty (the line does not exist yet), so the fire
against the synthetic is the only thing that makes the zero mean anything." That is the correct
characterization - S1 is a regression lint, not a check of shipped behaviour - and it is stated
rather than blurred.

**Question 2 - does a lint over one wrong expression cover the behaviour, or one spelling of
one route?** One spelling plus one near variant, measured rather than reasoned. Five synthetic
fixtures against S1's prescribed expression:

| variant | S1 as prescribed | correct? |
|---|---|---|
| `savedSnapshot.value = history.value[position.value];` (the actual regression) | **caught** | must catch |
| `savedSnapshot.value = history.value.at(position.value) ?? "";` | **caught** | must catch |
| `const marked = history.value[position.value]; savedSnapshot.value = marked;` | missed | should catch |
| `savedSnapshot.value = JSON.stringify(model.value);` | **missed** | should catch - same defect |
| `savedSnapshot.value = JSON.stringify(profile);` (the fix) | passes | must pass |

The fourth row is the finding (N2-F1): reading the live model is the identical defect and is
the likelier re-break, because a later simplifier reaching for the model has `JSON.stringify(profile)`
in front of them and `model.value` in scope. So S1 covers the behaviour partially, and the gap
is closable without turning the check into triage.

**Question 3 - is the infrastructure exemption correctly invoked?** Yes. The rule's boundary is
"a scenario the existing infrastructure can already express is not deferrable", and I have
verified the existing infrastructure cannot express it deterministically. The exemption is also
used as written rather than as cover: the plan does not say "coverage follows later", it names
the missing infrastructure, surfaces it for controller routing, states the residual in the
report, and ships two things in its place - a fix that makes the property structural (the
captured value cannot be the live one, because the expression names a different binding) and a
lint that pins the structure. That is the honest shape of the exemption.

**So: no behavioural check is required here, and I withdraw nothing.** What I would ask instead
is the widening in N2-F1, which is cheap and raises the lint from one spelling to both routes.
Recorded for a later round: if the releasable-mock infrastructure is ever built for another
reason, this behaviour is its first customer - a test that edits inside the dialog gap and
asserts the editor stays dirty.

---

## The precedent sweep: three classifications judged

The sweep's expression is derived from the reference symbols the plan actually cites rather
than from a list of what a precedent can be called, and it carries a fired control. Five
precedents, three non-trivial.

### 1. `Tab::onSaveConfig`'s `savedState = currentState()` - "YES, and this is the instance that produced the rule" - CORRECT

Already settled as N-F1. The classification is right and the fix is in the right place.

### 2. `Tab::onSaveConfig` delegating to `onSaveConfigAs` on an empty filename - "YES on the condition, already compensated" - CORRECT

I ran the premise. `Util::getSaveFileName` blocks, so the reference has no gap; `await saveDialog(...)`
opens one. The compensation claim holds, and the sweep's phrasing is more precise than it needed
to be: `RunHistory.saveLog` "is itself already async and therefore imports no synchrony to lose",
which is the correct reason the in-repo pattern is safe to copy where the Qt one is not.

Worth recording because it is a genuine behavioural divergence rather than only a condition: the
reference calls `updateConfigFromControlValues()` *after* the dialog returns, so mkvtoolnix writes
what the controls hold when the user picks a path, while Muxsmith writes what they held when Save
was clicked. The plan chooses the in-repo pattern over the reference deliberately, and after
N-F1's fix the save-state mark uses the same captured value, so the two are now consistent with
each other. Nothing to fix; the divergence is correctly resolved in favour of the house pattern.

### 3. `Tool::closeTab`'s warn-before-closing-a-modified-tab - "modality, not synchrony" - CORRECT ON ONE SURFACE, INCOMPLETE ON THE OTHER

**The in-app half is right, and I checked it rather than accepting it.** `showModal()` makes
everything outside the dialog inert, so pointer and focus channels cannot reach the widgets
between reading `dirty` and receiving the answer. I also checked for a non-user write during that
await, since inertness does not stop a programmatic one: the validation watcher writes
`diagnostics`, not `model`; `pickAndOpen` and `createBlank` are the only callers and both open the
confirm before any load starts; BatchView's apply-suggestion writes the file, not the editor's
model. So "the model cannot move" holds by construction, exactly as claimed.

**The shell half is where the paragraph ends in "benign", and running the premise finds a third
direction that is not.** The two named directions both hold:

- computed dirty, then saved during the dialog, then confirmed: warns where nothing was at risk.
  Annoyance. ✓
- computed clean: `on_close_requested` returns before `prevent_close()` and the window closes in
  the same synchronous event, with no window for the state to move in. ✓

The case the pair does not cover is the third one, and it is the data-loss direction - finding
N2-F2. The classification's *label* is still right (the condition is modality, not synchrony);
what is wrong is the completeness of the benignity argument built on it.

### 4-5. `hasBeenModified()` as a pure comparison, and the `actionMergeNew` ordering plus the empty state - "NO" - CORRECT

The comparison itself has no gap; the gap was in what feeds `savedState`, which is precedent 1,
and separating the two is the right cut. The UI-structure precedents carry no timing content.

---

## Round-2 findings, per-finding disposition

**N-F1 (Step 1c marked the live position) - ADDRESSED.** Step 1c now fences
`savedSnapshot.value = JSON.stringify(profile);`, retitled "mark the profile that was WRITTEN",
still inside the existing `try`. It states the two awaits, that `saving.value = true` disables
only the three buttons and not the widgets, both failure directions, and the structural property
that makes the fix correct ("when nothing moved, `profile` is the same object the last push
serialized, so the string equals `history[position]`"). D108 decision 3 is retitled to "Saving
MARKS the profile it WROTE", and decision 4's failure-direction claim now names its precondition
explicitly: "**That claim holds only because decision 3 marks the written profile rather than the
live one**". The parity clause states that the mkvtoolnix form is licensed by being fully
synchronous and that the condition does not carry. Nothing in `doSave` else changes, and the
opposite defect (omitting the line) is named too.

**N-F2 (red state 2's unnamed key) - ADDRESSED.** The key is named and the choice is justified in
both directions: `close-discard-title`, "it is one this package adds, so the mutation exercises
the new surface, and it is deliberately NOT the key (c) pins", with the consequence of choosing
otherwise spelled out.

**N-F3 (red state 3's incomplete matrix) - ADDRESSED, and the reading verified.** The row now
reads "(a) fails, and (c) fails with it - the chain then finds no requested row and falls through
to en, so the pinned German value is not returned. (b) passes, because it iterates only the rows
that exist." That is exactly my trace. The matrix preamble gained the general clause "and where
one mutation trips a second assertion as well, that is stated rather than left to inference", and
the row closes with "**Two failures from one mutation is a property of the design, not a
defect**" - which is the right disposition, since the plan's own rule only covers a red state
producing *too few* failures.

**N-F4 (self-review's stale L1 figure) - ADDRESSED.** Now "fires on its pre-state run - **two
lines, one per branch of `resolveLocale`**". I checked the whole file for any other site carrying
the superseded figure and found none.

**N-F5 (the `de()` permission wider than its safe set) - ADDRESSED; the figures reconcile
exactly.** Task 2 Step 5 now requires that "**Every asserted German string must be one whose
German value DIFFERS from its English value**", gives the mechanism (`buildBundles` negotiates
`[requested, en]` per message), names both language option labels as members of the excluded set,
and confirms the three prescribed cases are already clear of it. See the reproduction section for
the unit reconciliation and my recommendation on the method statement.

**N-F6 (W4-o's concrete tag) - ADDRESSED.** Step 6 now asserts both halves against concrete
values: the startup call's argument equals `"en"`, with the derivation stated ("`smoke.spec.ts`'s
scenarios take `get_settings` from the mock default, which returns `locale: "en"`, so
`effectiveLocale("en")` is `"en"`"), and the live call equals `"de"`. The row and its producer
now say the same thing.

**N-F7 (the ordering requirement that rode a replaced sentence) - ADDRESSED, and improved.**
Step 1's `resetHistory` bullet now carries "**The call site's POSITION is load-bearing and is
inherited rather than invented**", the two reasons I named, a third I had not (the push rule would
append a second entry, so "a freshly opened profile would start one step deep and dirty"), and the
instruction that Task 3's comment naming that order is updated in this task to name `resetHistory`
"since the requirement outlives the statement that carried it". That last clause is the general
form of the finding, in the right place.

**N-N1 (D1's alternation) - ADDRESSED.** D1 now states the asymmetry: the first alternative is
structural and catches any reassignment of the derived value; the second is three plausible names,
so a rival flag named otherwise escapes. It says why it is not widened (a broader name pattern
would match prose and turn the check into triage) and records the limit "so a later reader does not
read D1 as exhaustive over rival mechanisms". Correct, and consistent with `proc-proposed-safeguard-stays` -
the check is qualified, not weakened.

**N-N2 ((c)'s residual) - ADDRESSED, and the repo-wide negative claim CONFIRMED.** (c) now carries
"**What (c) is NOT, named so it is not read as more than it is**": it pins a pre-existing key and
therefore proves the shell reads the de catalog at all, and "does not detect a NEW German value
accidentally copied from its English source. No check in this repo does that for any catalog." I
verified the second half myself, since it is a negative claim about the whole repo - see the
reproduction section.

---

## New findings

### N2-F1 (MINOR) S1 misses the live-model spelling of the same defect

**Location.** Task 4, Step 7, absence check S1.

**What is wrong.** S1's expression is `savedSnapshot\.value *= *history`. Measured against five
synthetic fixtures (table in the divergence ruling above), it catches the exact regression and the
`.at()` variant, correctly passes the fix, and misses `savedSnapshot.value = JSON.stringify(model.value);` -
which is the same defect by a different route, reads the live state just as `history.value[position.value]`
did, and is the more likely re-break, because the fixed line is `JSON.stringify(profile)` and a
simplifier reaching for the model finds `model.value` in scope. S1 is described as pinning "the
structure"; it currently pins one spelling of one route to breaking it.

**Why this is not the D1 problem.** Widening D1 was rejected because a broader *name* pattern
matches prose. Here the addition is a second exact expression, not a heuristic: it adds no prose
surface and cannot produce triage.

**Ruling.** FIX by widening the expression to
`savedSnapshot\.value *= *(history|JSON\.stringify\(model)`, which I measured: it catches rows 1, 2
and 4 and still passes the fix. Fire it against both synthetics rather than one, since the
enumeration now has two members and firing against one leaves the other unproven - the same rule
the plan applies to its own gate audit's per-alternative fires. The local-const route (row 3) stays
uncovered and is worth one clause saying so, since a lint cannot follow a binding.

### N2-F2 (MINOR) The shell's read-then-ask gap has a third direction, and it is not benign

**Location.** The borrowed-precedent sweep, the `Tool::closeTab` entry: "its direction is benign -
a decision computed as dirty stays dirty, and a decision computed as clean closes the window in
the same event - and it is the pre-existing D31 shape rather than something this package
introduces."

**What is wrong.** Both named directions hold; the enumeration is missing one, and the missing one
is the data-loss direction. On the plan's own premise that the OS dialog leaves the webview live:

1. `close_decision` reads **run slot occupied, editor clean** and returns `ConfirmAbort`.
2. `on_close_requested` prevents the close and shows the abort dialog through a callback, so the
   webview stays live. The dialog's text is about running jobs and says nothing about the editor.
3. The user edits the profile while it is up. `dirty` becomes true and the watcher pushes
   `set_editor_dirty(true)`, so `AppState.editor_dirty` is now true.
4. The user confirms. The callback calls `abort_and_quit` immediately and **never re-reads the
   decision**. The app exits and the unsaved changes are gone, with no dialog having mentioned
   them.

That is R23 ("Closing the app WARNS when unsaved changes exist") defeated in the direction the plan
says this family never fails in. The codebase already knows the state moves during this dialog:
`abort_and_quit`'s own doc comment covers "when the run already tore down **while the dialog was
open**".

**Why "benign and pre-existing" does not carry it.** The gap's *shape* is pre-existing D31; the
*consequence* is new, because before this package there was no editor state the close was required
to protect. And the owner explicitly overruled the position that an opened-and-edited profile's loss
is acceptable - that is why the family is gated on save state at all. So a case where the guard
silently does not fire is a coverage gap against his ruling, not an inherited condition.

**Ruling.** FIX the paragraph, not necessarily the code. Name the third direction with its
consequence stated, and say why it is accepted: closing it means either re-reading `editor_dirty` in
the confirm callback and chaining a second prompt, which D109 decision 5 rejects, or widening the
abort dialog's text to mention changes that may not exist, which is worse than the two-prompt
option. **The tradeoff between the one-prompt property and complete coverage of R23 is the owner's
to make, not mine and not the plan's**, so it belongs in front of him as a named residual in the
same form D110 decision 3 uses for its push window - not concluded as benign. If he wants it closed,
that is a decision, not a defect.

### N2-N1 (NIT) The identical-value method statement needs one more qualifier

The plan states "15 gui-* ids carry identical en/de values". The figure is right and its method is
the right method; the method as stated ("full multi-line value comparison, attributes excluded" per
the author's report) reaches 16 unless value-less messages are also excluded. Worth one clause -
see the reproduction below for the exact reconciliation.

---

## Reproductions

| # | Claim | My measurement | Verdict |
|---|---|---|---|
| E1 | 66 acceptance halves, 10 + 12 + 22 + 20 + 2 | 10, 12, 22, 20, 2; total 66 | reproduced |
| E2 | 41 requirements, highest `R41` | 41 rows, max `R41` | reproduced |
| E3 | 7 corrections | 7 rows | reproduced |
| E4 | 5 ADRs (D106-D110) | 5; the Task-1 site says "the five decision records (D106-D110)" | reproduced |
| E5 | **9** absence checks | 9: L1, D1, S1, E1, E2, U1, G1, G2, H1. **My own first pass found only 7** - a case-sensitive `Absence check [A-Z][0-9]` scan misses E1 and G2, which are introduced in lowercase prose. Re-run case-insensitively: 9, matching the self-review's enumeration. Recorded because it is my instrument's blind spot, not the plan's | reproduced on the second pass |
| E6 | Gate audit returns 1, expression not narrowed | Extracted the expression from the plan and diffed it against my round-1 copy: **IDENTICAL**. Hits: **1** (the self-audit sentence). The plan's own history now reads two -> four -> two -> one, each new match a false positive removed by rewording | reproduced; **not narrowed** |
| E7 | Typography 0 | 0 hits for em/en dash, curly quotes, ellipsis, NBSP, Unicode minus, figure dash, horizontal bar. **Control:** the same pattern against a synthetic em-dash line returns 1, so the zero is a real absence | reproduced with a fired control |
| E8 | Files list equals `git add` for all seven tasks | Extracted both sets per task with my own comparator: 2/2, 6/6, 4/4, 6/6, 5/5, 9/9, 4/4 - **all seven match**, no extras in either direction | reproduced |
| E9 | The sentence-replacement sweep carried six requirements and found one loss | Spot-checked against the diff's 12 replaced lines. The three enumerating paragraphs verified member by member: **SAFEGUARDS** loses only the "eight absence checks" restatement, replaced by the nine-member version containing every prior member plus S1 - no safeguard dropped, and S1 joins the may-not-argue-away list; **ABSENCE** loses only the old L1 sentence, replaced with the corrected figure plus new D1 and S1 descriptions - no fire description lost; **GATE AUDIT** loses only the superseded history sentence, replaced by the corrected sequence, with the per-alternative fires and the negative control intact. N-F7 is the one loss, and it is fixed with the requirement restated plus a third reason | reproduced |
| E10 | 15 gui-* ids carry identical en/de values; both language option labels are among them | **Reconciled, and both of the plan's figures are right under one unit.** My round-2 figure was 18, first-line-only. Measured three ways: first-line-only **18**; full multi-line value, attributes excluded **16**; the same excluding value-less messages **15**. The 18 -> 16 delta is exactly two selector messages whose first lines match and whose German branches differ (`batch-diagnostics-summary`, `jobs-row-warning-count`); the 16 -> 15 delta is exactly one attribute-only message (`batch-recents-select =` with only a `.tooltip`). So the plan's 15 and its stated 17 for the first-line variant both reconcile with mine under the single rule "messages that carry a value of their own". Both option labels are in the set under every unit | **reproduced under the plan's unit; my 18 was the coarser one** |
| E11 | Red state 3 fails (a) and (c), (b) passes | traced against Step 1b's chain: with no `de` row the chain finds no requested row, falls through to en, returns `Abort running jobs`, so (c) fails; (a) fails on the directory-versus-table comparison; (b) iterates existing rows only and passes | reproduced |
| E12 | The harness cannot express a releasable mock response | `e2e/mocks.ts` carries the quoted `addInitScript` comment verbatim; `MockResult` is `{kind:"resolve",value}`/`{kind:"reject",error}` and `nextResult` shifts values off a queue - no test-held promise. `page.exposeFunction` is already used for invoke recording, so the seam exists and building on it is infrastructure | reproduced |
| E13 | S1 catches the regression | measured against five synthetics: catches the exact line and the `.at()` variant, passes the fix, **misses `JSON.stringify(model.value)`** and the local-const route (N2-F1) | reproduced with a gap |
| E14 | "No check in this repo" compares a German value against its English source, for any catalog | **CONFIRMED.** `check-i18n.mjs`'s cross-locale comparisons are: file presence, id sets, attribute-name-set equality, and `comparePatterns`, which compares placeable sets and select-expression counts - structure, never text. `e2e/i18n-en.ts`'s `assertAllCatalogsParseCleanly` walks every locale directory but asserts per-locale parse/value-or-attribute presence. `e2e/locale-switch.spec.ts`'s `de()` formats through the de bundle alone. `crates/muxsmith-cli/tests/catalog_completeness.rs` renders through `Renderer::new(Some("en"))` and asserts key resolution, with no de leg. The shell's tests pin one en value and, after this package, one de value. **My control:** the same sweep does surface the one cross-locale comparison that exists (`comparePatterns`), so the negative is a real absence rather than a search that could not have found one | reproduced, with a control |

**Nothing diverged** except E10's unit, which reconciles exactly rather than disagreeing, and E5,
where my own first instrument was the blind one.

**Recommended method statement for E10**, since you asked which the plan should state: full
multi-line value comparison, attributes excluded, over messages that carry a value of their own.
The reason is the question the number answers - would an assertion on the rendered value pass under
an English fallback? A selector's branches are part of the rendered value, so two messages whose
first lines match but whose branches differ are not fallback-blind; an attribute is a separate
accessor (`$ta(id).tooltip`) that a `de(id)` value assertion never reads; and a message with no
value cannot be asserted as a value at all. Under that unit the answer is 15, which is what the
plan states.

---

## Harvest for the controller

The three items from my round-2 verdict are ledgered, so they are not re-harvested. Two new ones.

1. **"Benign and pre-existing" is a two-part claim and both parts can be true while the conclusion
   is false.** N2-F2's shape: the gap's mechanism is genuinely inherited and its named directions
   are genuinely harmless, and the conclusion still fails because a third direction was not
   enumerated. The readable trigger is sharper than the general
   run-the-premise rule and worth recording next to it: **a paragraph that calls a gap benign by
   listing directions is making a completeness claim about the direction set, so count the states
   the gap can be entered in rather than the ones that came to mind.** Here the entry states are the
   product of two facts the decision reads, so there are four, and three were considered.
2. **A regression lint pins a spelling, and the spelling it pins is the one that already happened -
   which is the least likely one to happen again.** S1 is correct and worth keeping, but the defect
   it guards was found and fixed, so the next instance will arrive by a different route. The handle:
   **when a lint is written because a specific defect occurred, enumerate the other spellings of the
   same semantics before fixing the expression**, since the occurred one is now the one everybody is
   watching. Cheap where the alternatives are exact expressions, as here; not applicable where they
   would be name heuristics, which is why D1 correctly declines to widen and S1 correctly should.

---
---

# CONFIRMATION PASS - round 4 repairs (same reviewer)

Narrow confirmation of the three repairs I said I would approve on sight. Not a
new round: nothing outside the three was re-audited, re-swept or re-graded, as
instructed.

Artifact: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`, 1242
lines. It is committed as `0325923`; the 1237-line version I graded in round 2 was
never committed, so my baseline for "nothing else changed" is `e5cb799` (1210
lines) plus the round-2 hunk map I recorded when I verified it.
Instruments, fresh and at a new path,
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr12-confirm-independent/`:
five synthetic `.ts` fixtures for S1 tested against the full expression and each
member alone, and a **new node instrument** (`units.mjs`) for the catalog figures -
deliberately a different language and a different algorithm from the Python parser
I used in round 2, so the third restatement of those figures is checked by a
decorrelated measurement rather than by my own earlier one.

## Verdict: APPROVED

All three repairs hold. No new findings. **Gate signal: the plan is ready for the
governing human.**

## Scope check: nothing else changed

The diff `e5cb799` -> working tree carries the **same eleven hunks at the same
baseline positions** as the round-2 diff I verified. Ten are unchanged in size;
only the precedent-sweep hunk grew, by exactly the five lines of the N2-F2
enumeration. The N2-F1 and N2-N1 repairs are in-place edits inside hunks whose
size is unchanged. No hunk appeared, disappeared or moved to a new location, so the
three repairs are the whole of this round. Standing counts spot-confirmed
unchanged: 66 halves, 41 requirements, 7 corrections, 9 absence checks; the gate
expression is byte-identical to my round-1 copy and returns **1** hit; the AI-tell
glyph scan returns **0** with a firing control.

## N2-F1 - CONFIRMED, and both members proven independently

S1 now reads
`grep -nE 'savedSnapshot\.value *= *(history|JSON\.stringify\(model)' src/views/EditorView.vue`,
retitled "the saved position is never marked from LIVE state, by either route", with
both members fired against their own synthetic and the rule stated ("firing one
member leaves the other unproven, the same rule this plan applies to its own gate
audit's per-alternative fires").

I tested the expression verbatim against five fresh fixtures, and ran each member
**alone** as well as the full alternation, so that a fire cannot be silently carried
by the wrong alternative:

| fixture | full expression | member 1 alone | member 2 alone |
|---|---|---|---|
| `savedSnapshot.value = history.value[position.value];` | 1 | **1** | 0 |
| `savedSnapshot.value = JSON.stringify(model.value);` | 1 | 0 | **1** |
| `savedSnapshot.value = JSON.stringify(profile);` (the fix) | 0 | 0 | 0 |
| `const live = history.value[position.value]; savedSnapshot.value = live;` | 0 | 0 | 0 |
| neutral line | 0 | 0 | 0 |

Each member fires against its own synthetic and is not matched by the other, so both
fires are attributable. The fixed line passes cleanly, so the widening introduces no
false positive on the very line it guards. The local-const route returns 0, exactly
as the plan states it is uncovered, with the correct reason given (a grep cannot
follow a binding). The "not the widening D1 declined" distinction is stated and is
right: a second exact expression adds no prose surface.

## N2-F2 - CONFIRMED as an honest residual that routes the choice

The entry no longer concludes benignity. Its heading now reads "it holds on one
surface by construction **and not on the other**", the in-app half is stated as
closed for the Open and New guards, and the shell half enumerates **three**
directions with the third spelled out step by step - `ConfirmAbort` returned, the
callback dialog speaking only about running jobs, `set_editor_dirty(true)` landing
while it is up, and confirming running `abort_and_quit` "**without re-reading the
decision**". That is my trace, step for step.

It then states direction 3 "**is a residual, not a benign case, and it is stated
rather than concluded away**", carries the shape-versus-consequence distinction (the
D31 flow is inherited, the consequence is new because before this package there was
no editor state the close had to protect), names R23 as the requirement it defeats
and the direction as data loss, and names both closing costs with the ground each
was rejected on - a chained second prompt against D109 decision 5, or a widened
abort text that "trades a false statement for a missing one". It closes with
"**This plan picks neither**" and puts the tradeoff in the owner's hands "in the
same form D110 decision 3 carries its push window".

I am confirming that the paragraph states the residual honestly and routes the
choice, not that the residual is closed. It is not closed, by design, and that is
correct: the tradeoff is between his own one-prompt ruling and complete coverage of
his own warn-on-close ruling, which is his to weigh and not the plan's. No side is
picked and no code changed.

## N2-N1 - CONFIRMED, and all three figures reproduce on a decorrelated instrument

The method now reads, in the plan's own words, "**full multi-line value comparison,
attributes excluded, over messages that carry a value of their own**", with both
coarser readings reconciled under that one rule.

Because this is the third restatement of those figures, I re-measured them with a
new instrument in a different language and with a different block-extraction
algorithm rather than citing my round-2 Python run:

| reading | plan | my fresh measurement |
|---|---|---|
| full value, attributes excluded, value-bearing only (the stated rule) | 15 | **15** |
| the same, counting value-less messages | 16 | **16** |
| first lines only | 18 | **18** |

And the member attributions, which are the part a restatement most easily gets
wrong:

- the single extra member at 16 is `gui-batch:batch-recents-select`, and my
  instrument independently reports it as attribute-only (`hasAttr=true`, empty
  value) - exactly as the plan says;
- the two further members at 18 are `gui-batch:batch-diagnostics-summary` and
  `gui-jobs:jobs-row-warning-count`, both selector messages whose first lines match
  (`{ $errors ->`, `{ $count ->`) while their full values differ - exactly as the
  plan says;
- both language option labels are in the 15-set under the stated rule.

Every attribution is right, and the figure the plan states is the one its stated
method produces.

## Anything new

Nothing. No new finding, and no earlier disposition changes.
