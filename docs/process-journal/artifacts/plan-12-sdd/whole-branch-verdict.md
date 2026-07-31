# Plan 12 whole-branch review verdict

Package: `review-bd3aa34..50ae53f.diff` (48 commits, 13 touching product files).
Reviewed on `master`, main worktree, read-only except this file. Every in-tree
mutation below was restored and verified BY CONTENT (`md5sum` against a pristine
copy outside the repository, plus `git status`), and the frontend was rebuilt after
every edit and every restore before any e2e run.

**Merge verdict: NEEDS_FIXES.** 0 Critical, 7 Important, 7 Minor.
Nothing on this branch is broken; what is missing is coverage in the direction that
loses data, and four normative statements that no longer describe the tree.

---

## Closing state of the tree as I left it

| Check | Result |
|---|---|
| `git status --short` | empty |
| `md5sum` of every mutated file vs pristine copy | identical (4 files) |
| `pnpm build` | exit 0 |
| `npx eslint .` | exit 0 |
| `node scripts/check-i18n.mjs` | exit 0, `0 unused warning(s)` |
| `npx playwright test` | 101 passed |
| `cargo test --workspace` | all suites ok (86 in `src-tauri`) |

Exit codes captured per command, never through a pipeline.

---

## Findings

### Important

**I-1. The reworded batch empty-state string ships with no producer.**
`locales/{en,de}/gui-batch.ftl` -> `batch-profile-none`.
Routed to this wave; confirmed at the artifact before accepting it.
`command grep -rn "batch-profile-none" e2e/ src/ scripts/ src-tauri/` returns exactly
two hits, both in the gitignored vendored catalog copy inside
`e2e/.generated/mount-harness.js`, plus the render site `src/views/BatchView.vue:348`.
Zero hits in any spec file. Controls from the same catalog block in the same
directory: `batch-profile-current` 37 hits in `e2e/`, `batch-recents-heading` 3.
The absence is real and the pattern works. Acceptance row **W5-b** claims this string
is "asserted through `en(id)` in the existing batch scenario"; it is not.
`tests-ship-with-the-feature-never-after` binds: user-visible, reworded by this plan,
expressible with the existing harness.

**I-2. The redo shortcuts and the Meta modifier are exercised nowhere, and the branch
documents all four spellings.**
Acceptance row **W3-o** ("Both modifier keys and both redo spellings work | Task 4,
the keyboard test over the enumerated combinations | yes"). There is no such test.
`command grep -rn "\.press(" e2e/ --exclude-dir=.generated` returns 8 lines in total;
the only two that carry a modifier are `e2e/editor-undo-redo.spec.ts:576` and `:583`,
both `Control+z`, both inside the U1 text-entry-exemption case.
`command grep -rni "meta+\|control+\|ctrl+" e2e/ --exclude-dir=.generated` returns
those same two plus two prose lines. `Control+Shift+z`, `Control+y` and every `Meta+`
combination appear nowhere.
This is not a paper gap: `help/en/view-editor.md:11` and `help/de/view-editor.md:11`,
both added by this branch, tell the user "Ctrl+Z (Cmd+Z on macOS) for Undo, and
Ctrl+Shift+Z or Ctrl+Y (Cmd+Shift+Z or Cmd+Y on macOS) for Redo".
I verified the behaviour is correct with a temporary probe spec (created, run,
`command rm`'d, absence verified): a single case drove `Control+z`, `Control+Shift+z`,
`Control+y`, `Meta+z`, `Meta+Shift+z` and `Meta+y` against the rule count and passed.
So the code is right and the coverage claim is false. The probe is the missing case.

**Note for the harvest:** W3-o names a producer this plan would BUILD, which the
house entry `an-acceptance-row-naming-an-existing-producer-is-verified-by-finding-it`
treats as the self-correcting kind. It did not self-correct. Nothing in a task run
opens the acceptance map, so neither kind is self-correcting; what distinguishes them
is only that a built producer is more likely to be prescribed in a step list, and
Task 4's step list did not prescribe this one.

**I-3. The save-marking property is covered in its presence and uncovered in its
dangerous direction.** `src/views/EditorView.vue:582`.
The shipped comment names the inversion itself: "marking the live entry would invert
it exactly, reporting clean over content the file does not hold" (`:210-213`).
- Fire test (presence): deleting the line entirely -> `pnpm build` exit 0, `npx
  playwright test` **2 failed / 99 passed**, the failures being
  `smoke.spec.ts:2243` (Case 3(ii)) and `smoke.spec.ts:1320`. The producer fires.
- Direction test: `savedSnapshot.value = JSON.stringify(profile)` ->
  `savedSnapshot.value = history.value[position.value]` -> **101 passed**.
Case 3(ii) proves only "after a save the guard clears", which both configurations
satisfy whenever nothing is edited during the save. No test creates an edit inside the
save window, which is the only state in which the two differ, and it is the state the
comment was written for. The failure direction is data loss (a dirty editor reporting
clean, no discard warning, no close warning).

**I-4. The close dialog's decision-to-strings mapping has no producer, and its failure
direction is the one D109 exists to prevent.** `src-tauri/src/run.rs`,
`show_close_dialog`'s match arms.
Mutation: point `CloseDecision::ConfirmDiscard` at `close-abort-title` /
`close-abort-message` / `close-abort-confirm` instead of the `close-discard-*` keys ->
`cargo test --quiet` in `src-tauri`: **86 passed, 0 failed.**
Consequence of that state in production: a user with unsaved changes and no running
job is asked "There is currently a job running. Do you really want to abort all
currently running jobs and quit?", confirms, and loses the profile without the dialog
ever mentioning it.
The four-state decision itself IS covered per cell -- I ran the control:
`(false, true) => CloseDecision::Close` reddens
`close_decision_confirms_discard_while_idle_and_dirty` and nothing else. So the plan
factored `close_decision` out for testability and left the variant-to-strings mapping
(and `confirm_close`'s variant-to-action mapping) inside the `AppHandle`-taking
functions, where nothing reaches them. Acceptance rows **W4-j/k/l** read as covering
what the prompt SAYS ("asks about the run only", "ONE prompt naming both"); their
producer establishes only which enum variant is chosen.
Fix shape: extract the key triple as a pure `fn close_dialog_keys(CloseDecision) ->
(&'static str, &'static str, &'static str)` and assert the four rows, exactly the way
`close_decision` was already factored. Same for `confirm_close` if it is cheap.

**I-5. The two synonym-direction terminology instances, with their site counts.**
`help/de/view-editor.md`, `help/en/view-editor.md`.
Confirmed at the corpus, and the list of two TERMS is complete -- I derived the
content-word set from the diff myself and checked `Erweiterung`, `Kandidat`,
`Prüfung`, `Festplatte`, `Anwendung`, `Speicherort`, `Speichervorgang`,
`Aktionsleiste`, `Tastenkürzel`, `Dialog`, `candidate extension`, `validation`,
`disk`, `action row`, `keyboard shortcuts`, `quit` against `locales/` and `help/`.
All clean; `Aktionsleiste`/`action row` and `Tastenkürzel` have no corpus term at all,
so they are coinages, not synonyms. No third term.
What the routed wording does not carry is that each term has SEVERAL sites, so a
dispatch that says "two instances" invites a two-line fix:
- German `anlegen` for the create affordance, **3 sites**: `help/de/view-editor.md:3`
  ("Lege ein neues Profil an"), `:7` ("Neues Profil legt ein Profil ... an"), `:9`
  ("durch Anlegen eines neuen"). The corpus term is `erstellen`
  (`locales/de/gui-editor.ftl:156` "Erstelle eines mit Neues Profil",
  `locales/de/gui-batch.ftl:13` "oder erstelle eines in der Editor-Ansicht"), and the
  same file's own heading `:5` already uses it.
- English `start` for the create affordance, **2 sites**: `help/en/view-editor.md:3`
  ("Start a new profile"), `:7` ("The New profile button starts a profile"). The
  corpus term is `create` (`locales/en/gui-editor.ftl:152`,
  `locales/en/gui-batch.ftl:17`), and the same file's `:5` and `:9` already use it.
- **The English half is additionally a COLLISION, not only a synonym.** The corpus
  uses `start` for beginning a run or a batch (`locales/en/gui-jobs.ftl:13` "Start a
  run from the Batch view", `batch-profile-pick.tooltip` "validate and run"), and the
  single sentence this same plan wrote contrasts the two verbs directly:
  "Choose one below to validate it and start a batch, **or create one** in the Editor
  view". So `a-new-term-is-checked-against-the-corpus-in-both-directions` fires in both
  directions on this one word, which the routed classification records as synonym only.

**I-6. A live design document carries two mechanism statements this branch falsified.**
`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md:99-102`:

> `saveDisabled` gates on `hasErrors` (**error severity only**), plus
> `!model`/`!currentPath`/`saving`/`opening`; the shallow `watch(model)`
> revalidates every model swap through `validateProfileModel` (gated on
> `currentPath`, so the bare mount harness never fires IPC).

Both halves are now false, by this plan's own D107 decision 3(a) and 3(b):
`saveDisabled` is `!model.value || hasErrors.value || saving.value || opening.value`
(`EditorView.vue:330-332`) and the watcher gate is `sessionActive`
(`EditorView.vue:349`). The branch DID sweep the sibling comment in
`e2e/editor-rule-add-remove.spec.ts:8` (commit `1092eb7`) and the two doc-comment
regions inside `EditorView.vue` that Task 4's Files list names -- so the sweep was
anchored on the FILES the change touched and stopped there, while the fact lives in a
design document that the swept test file implements.
`an-edit-to-a-set-walks-to-its-neighbours-not-only-to-its-enumerations` and
`a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact` both bind.
Fix shape: the in-place supersession marker this plan already uses on D107 decision
3(f), not a rewrite.
**Give the fix wave the RULE, not this file list**: the facts this branch changed are
`saveDisabled`'s term set, the `watch(model)` gate, the recents-section gate,
`close_decision`'s arity, `ftl_message`'s signature and catalog set, and the editor
catalog count -- grep `docs/superpowers/` for each. I ran that sweep and found one
further, weaker instance
(`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:245-252`: `ftl_message`
described as reading `locales/en/gui-common.ftl` for "the four `close-abort-*` keys",
with a `:539-560` line citation). That paragraph reads as a dated recon inventory --
it cites line numbers into four files -- so it falls on the same side as the ROADMAP
and journal 43-figures the controller ruled owe nothing. I name it so the wave decides
once rather than meeting it later; my own read is that it owes nothing and `:99-102`
does, because `:99-102` describes the mechanism as standing ground truth.

**I-7. The reworded settings locale hint has no producer either, and the claim made
for it cannot be true in the form it is stated.**
`locales/{en,de}/gui-settings.ftl`, `settings-locale-label.hint`.
This plan appended a whole sentence to it ("System language follows your operating
system and falls back to English where a translation is missing."), which is the
user-facing explanation D106 owes. The plan's pre-table note (plan document line 526)
covers it with: "the two reworded values (`batch-profile-none`, the locale hint) ride
the assertions that already read them through `en(id)`."
Measured: `command grep -rn "enAttr" e2e/ | grep -v .generated` returns 7 lines
(`editor-tooltips.spec.ts` x2, `help-mode.spec.ts` x2, `smoke.spec.ts` x1, plus the
helper's own definition and one import), none for this id;
`command grep -rn "settings-locale-label" e2e/ | grep -v .generated`
returns exactly one, `smoke.spec.ts:832`, which uses `name(...)` on the LABEL to find
the combobox and never reads the attribute. No spec anywhere asserts a `.hint`.
And `en(id)` structurally cannot read an attribute -- `e2e/i18n-en.ts:181` defines
`enAttr(id, attr)` as the separate helper for that -- so the sentence names a
mechanism that could not have covered either value.
Same reasoning as I-1 applies (user-visible, reworded here, expressible: my probe
asserted `dialog.locator("#settings-locale-hint")` against the new sentence in one
line and passed). Either assert it or correct the note; I recommend asserting it,
because the note is now wrong about both of the two values it covers.

### Minor

**M-1. The same pre-table note over-claims on two more sub-claims.** Plan document
line 526.
(a) "W2's three new editor ids and W4a's three discard ids ... their values are
additionally asserted through rendered text (W2-e, W4-a)". Measured, hits in `e2e/`
excluding `.generated`: `editor-empty` 6, `editor-unsaved` 4,
`editor-discard-message` 1, and **`editor-action-new` 0, `editor-discard-title` 0,
`editor-discard-confirm` 0**. Three of six ids have no rendered-value assertion; their
BEHAVIOUR is covered through `data-testid`, so nothing is broken, but the catalog value
behind the New button and behind the discard dialog's title and confirm label is
unpinned. `catalogs.spec.ts` does not close this -- it asserts parse cleanliness only.
(b) "no test in this suite asserts sibling order" -- the drag-reorder cases assert
exactly that (`editor-undo-redo.spec.ts:198-199, 209-210, 214-215`, `rows.nth(0)` /
`rows.nth(1)`). The conclusion (that two distinct sibling buttons' DOM order is not the
house pattern) survives; the premise as written does not.
Recommend correcting the sentence rather than adding three assertions.

**M-2. Acceptance row W3-h names a producer that does not exist and cannot.**
"...and in the model the shell would receive | same test, `readModel` on the
mount-harness case". `command grep -rn "readModel" e2e/` shows it used in
`editor-dropdowns.spec.ts`, `editor-rule-add-remove.spec.ts` and `smoke.spec.ts`, and
**not once in `editor-undo-redo.spec.ts`** -- whose own header (`:5-10`) states that
the mount harness never reaches this watcher at all. No IPC-arg assertion follows any
undo anywhere (`.args` appears once in that file, on a `save_profile` count).
Materially the observable rides the rendered assertions, because the widgets render
from the model, so this is a row-accuracy defect rather than a coverage hole.

**M-3. A measured figure in shipped source no longer reproduces.**
`src/views/EditorView.vue:486-492`, the `createBlank` ordering comment: "gate first
with an `await` between gate and model: **79 passed**. Gate first with the `await`
after both: **79 passed**. Model first with an `await` between them: **3 failed** (the
first three cases of the New describe in `e2e/smoke.spec.ts`)." Undated and unscoped.
I ran the third configuration verbatim (model assignment first, `await
Promise.resolve()` between it and `resetHistory`): **3 failed, 98 passed**, and the
three are exactly `smoke.spec.ts:1765`, `:1804`, `:1830` -- the first three cases of
the New describe. So the discriminating claim reproduces perfectly and the absolute
counts do not; the suite is 101 today. Same class as the `EN_OVERRIDE_SETTINGS` comment
Task 2 escalated to its own fix round, one notch weaker: date-and-scope the counts, or
drop them and keep the failing set.

**M-4. A falsified measurement still stands in the shipped ADR.**
`docs/superpowers/specs/2026-07-30-plan-12-decisions.md:141`: "every current call site
is a literal, measured". Measured today with
`command grep -nE 'ftl_message\([a-z_]' src-tauri/src/run.rs`: three non-literal call
sites, `:1611` (`ftl_message(key, "en")` in the resolve loop), `:1635` and `:1639`
(the two probes bound to locals). M-6 recorded one of these at Task 1 and ruled the
consequence for the derived key set nil -- correct, and still correct: the derivation
finds 12 literal sites and every one of the ten distinct keys resolves in both rows.
What M-6 did not do is correct the ADR sentence, and Task 6 then ADDED two more
non-literal sites deliberately, documenting them at the source but not here. The ADR
outlives the plan.

**M-5. Keyboard undo/redo reaches the editor while the discard confirmation is open.**
`src/views/EditorView.vue`: Task 4 put `@keydown="onEditorKeydown"` on
`<section data-testid="view-editor">` (`:897`), Task 5 mounted `<ConfirmDialog>` as a
DOM child of that same section (`:901`). A native modal blocks pointer input but
keydown from a focused element inside the dialog still bubbles to the ancestor.
Probe (run, then removed): with unsaved changes and the discard dialog open, focusing
the dialog's cancel button and pressing `Control+z` undoes the model -- the rule count
went 2 -> 1 while the confirmation was still on screen. Recoverable via redo and
overwritten anyway if the user confirms, so it is not data loss, but the user is being
asked about a state that changes under the question. One-line fix
(`if (confirmDialog.value?.isOpen) return;` in `onEditorKeydown`, or a
`closest("dialog")` guard). Same family as the deferred reentrancy item; a different
mechanism, so it does not re-open that deferral.

**M-6. The shell's primary-subtag collapse is the default path and has no test.**
`src-tauri/src/run.rs`, `ftl_message`'s first line.
`applyLocale` stores the RAW tag (`src/i18n/fluent.ts:17`, `currentLocale.value =
locale`), and after D106 the default state is no stored override, so
`effectiveLocale(null)` is `navigator.language` -- `"de-DE"` on a German machine. That
is what `set_shell_locale` sends. Everything between that user and an English close
dialog is `locale.split('-').next().unwrap_or(locale).to_lowercase()`.
Every existing assertion passes a bare tag: `ftl_message(..., "en")` and
`ftl_message("close-abort-title", "de")`; the e2e side asserts
`["en", "de"]`. So D106 made the system language the default and D110 tested the
override case.
Verified correct by an out-of-band probe added to the test module and removed:
`de-DE` and `DE-at` both resolve German, `fr-FR` falls back to English. Three
assertions, ready to land.

**M-7. `RUST_ONLY_IDS` has no hard gate behind it.** `scripts/check-i18n.mjs:118-129`.
Removing `close-discard-title` from the allowlist: `node scripts/check-i18n.mjs` exits
**0** and prints "gui-* catalog keys with no detected reference in src/ (warning
only): close-discard-title". Acceptance row **W4-s** is accurate -- it names two pasted
runs, and the difference between them is real and reproducible -- but the property is
advisory, not standing. Non-blocking; recorded so the row is not later read as a gate.

---

## Ruling per deferred and parked item

Format: item -> ruling -> the ground.

1. **ConfirmDialog `ask()` has no reentrancy guard** (deferred with a trigger, Task 5)
   -> **Disposition stands.** Verified at the artifact: `settleAsk` is overwritten
   before `showModal()`, so a second entry orphans the first promise. The component has
   exactly one caller pair, both inside `EditorView.vue` and both gated on `dirty`, and
   the trigger (a second caller) is written into the component's own doc comment
   (`:10-12`) rather than into anyone's memory -- the self-announcing form. M-5 above
   shows the surrounding section is not inert to the KEYBOARD, which is a different
   mechanism and does not re-open this.
2. **Task 7's sweep table framed wider than its list** -> **Disposition stands.**
   Scratch artifact; and I re-derived the corpus terms independently (I-5) and found no
   third term, which is the only thing that could have made it consequential.
3. **The reworded batch empty-state string has no producer** (required, not deferrable)
   -> **Fix now.** See I-1. Plus its sibling, I-7.
4. **The stale Tier-2 catalog-budget entry** (`docs/product-boundaries.yaml:410`,
   `editor-generic-action-keys`, recording 43 -> 45 -> 46) -> **Disposition stands, and
   the close must RECOMPUTE.** Measured: `locales/{en,de}/gui-editor.ftl` carry **54**
   ids each. This branch itself moved it 51 -> 54, so the entry is falsified further
   here, which makes it a close action this branch owes rather than an inherited one.
   The decomposition is already written twice in the tree and both agree
   (`EditorView.vue:67`, `e2e/smoke.spec.ts:884-886`: 42 labels + 1 save note + 4
   generic action keys + 1 rule-grid ordinal + 3 profile-creation keys + 3
   discard-confirmation keys = 54), so the close has a source and need not re-derive.
   Not a merge blocker: house knowledge, not shipped product.
5. **`src/editor/registries.ts`, the two field-spec statements** ("42 of the 43 fields",
   "the 43-row table") -> **Disposition stands; nothing owed.** Independently measured:
   `command grep -c "labelKey:"` returns **42**, `command grep -n "fixed:"` returns
   exactly one line (`:73`, `Profile.profile_version`). 42 + 1 = 43. Both hold.
6. **`src/editor/widgets/SelectWidget.vue:5`** ("gui-editor.ftl stays at its 43 label
   keys") -> **Disposition stands (close-time comment correction).** Stale on either
   reading: 54 ids, 42 label keys. Pre-existing and untouched by this branch, so not a
   merge blocker -- but it is a false statement in shipped source, which this plan's own
   Task 2 fix round 2 treated as worth a dispatched commit, so do not let it slip past
   the close.
7. **`src/editor/widgets/StringListWidget.vue:5`** ("no generic add/remove chrome text
   exists in gui-editor.ftl's 43 keys") -> **Disposition stands (its own vehicle).**
   Verified dead, not merely stale: `editor-action-add` and `editor-action-remove` sit
   at `locales/en/gui-editor.ftl:138-139` and have since an earlier package. That
   absence is the stated REASON for the comma-separated design, so the premise under a
   design choice has expired; a reworded sentence would paper over a design question.
   Not a blocker.
8. **Task 4's evidence gap** (the fix report pastes a `command grep` that returns 27
   lines when run verbatim) -> **Disposition stands, with one qualification.** Scratch
   artifact, conclusion independently confirmed. But the report is salvage material and
   the pasted command is exactly the `design-empirical-claims-reproducible` shape; a
   one-line in-place correction at the close is cheap and preserves the artifact's
   evidentiary value. Not a blocker.
9. **O-1, `v-show` is outside the prescribed lint rule and the prose does not say so**
   -> **Disposition stands; I recommend closing it in this wave anyway.** Measured both
   directions: `v-if="!model"` errors with the D112 message
   (`EditorView.vue:951:14`, `vue/no-restricted-syntax`); `v-show="!model"` exits 0
   with zero findings. The residual is real, and `v-show` is the directive this
   codebase actually uses for view gating (`App.vue`). The SHIPPED config comment does
   name its own scope ("Scoped by directive name to `v-if`/`v-else-if`"), so a reader
   of the rule can derive it -- which is why the disposition stands rather than
   escalates. Closing it is one token in the selector regex
   (`/^(if|else-if|show)$/`); at that price I would take it here.
10. **O-2, in-place corrections leave the wrong sentence standing with no forward
    pointer** -> **Disposition stands.** Scratch artifact; the house form deliberately
    leaves the wrong version standing.
11. **O-3, the rejected alternative stated without its steelman** -> **Disposition
    stands.** The omitted argument (it would have kept the two Files lists disjoint)
    does not move the ruling, and the plan is retired at the close.
12. **O-4, the "eight ordinal cross-references" figure is unverifiable** -> **Disposition
    stands.** Scratch artifact; the end state was verified with a fired control.
13. **The validation-response race** (`watch(model)` returns early without incrementing
    `validationGeneration`) -> **Disposition stands: Plan 13, fifth member.**
    Reproduced at the source myself: `EditorView.vue:349-351` returns before
    `:373`'s `++validationGeneration`, so an in-flight response still matches at `:376`
    and `:377` overwrites the parse diagnostic `openPath` wrote at `:401`.
    **And I checked the one thing that would have changed the ruling:** it is genuinely
    pre-existing and this branch did not widen it. At `bd3aa34` the same early return
    reads `if (!currentPath.value || !value) { return; }` and also precedes the
    increment; after a failed load `model` is undefined either way, so the same branch
    was taken before this plan. Failure direction is a lost error message, not lost
    data. The drafted ROADMAP text already names the right thing (it is the observable
    D112 shapes that the race can erase); keep that clause.
14. **M-2, two counts in the ADR without their enumeration** -> **Disposition stands.**
    Both verified correct and both closed downstream.
15. **M-3, D108's steelman omits the CPU cost** -> **Ruling stands; re-derived, not
    borrowed.** The same watcher body performs `JSON.stringify(value)` at
    `EditorView.vue:358` and the `validate_profile_model` IPC round trip at `:375` on
    the same path, so the marginal cost of the design is one local serialisation of a
    101-419 byte object per write that already pays for a cross-process call to a
    blocking thread pool. Dominated, as ruled.
16. **M-4, the spec's "in one prompt" carries no per-state qualifier** -> **Fix at the
    close, as recorded.** A reader holding only the amended spec sentence
    (`2026-07-08-muxsmith-v1-design.md:382`) concludes "never two prompts", which D109
    decision 9's conditional re-prompt contradicts. The spec outlives both. Not a
    blocker.
17. **M-5, the spec's seed enumeration omits `input.pattern`** -> **Fix at the close, as
    recorded.** Confirmed: the spec says "the format version, one candidate extension
    and one empty track rule"; the shipped seed (`EditorView.vue:457-463`) also carries
    `input.pattern: ".*"`, which D107 justifies at length. Three named, four shipped.
    Not a blocker.
18. **M-6, D110's "every current call site is a literal"** -> **The cross-task half
    discharged correctly; the ADR sentence is NOT closed.** See finding M-4 above. The
    deferral answered the derived-key-set question and left the false sentence standing
    in a durable artifact, where this plan's own Task 6 then made it more false.
19. **Task 2's deferred minor** (the corrected comment's escape clause worded as
    "without a reload or a second fixture state" where fix round 1 said "new test
    infrastructure") -> **Disposition stands.** A wording divergence between two
    compatible statements, both in place, neither load-bearing.
20. **Task 1's N-1** (the delta round's class attribution for M-1) -> **Disposition
    stands.** Scratch artifact, corrected in the same round's C-2, nothing downstream
    turns on it.

**Summary: 18 dispositions stand, 2 require work now** -- item 3 (the routed required
fix, plus its sibling I-7), and item 18's ADR sentence. Items 4, 6, 16 and 17 are close
actions the plan already owns; item 9 is a recommended pull-forward.

---

## The acceptance-map walk

**73 rows checked** (W1 10, W2 16, W3 22, W4 23, W5 2), plus the pre-table prose note.
**4 rows with a gap**, 2 of them already known.

Method: for every row I opened the named producer and read it. For the rows naming an
EXISTING producer (the class the house entry says is believed because it describes the
past) I additionally ran the mechanism:

| Row | Existing producer named | What I ran | Result |
|---|---|---|---|
| W1-h | the two unchanged `smoke.spec.ts` assertions | located both (`:833` `toHaveValue("en")`, `:864` `toHaveValue("de")`) and grepped the branch diff for them | present, and neither line appears in the diff, so "unchanged" holds |
| W1-i | Task 2's absence check L1 | `command grep -rn "navigator.language" src/` | 1 code site (`i18n/index.ts:46`) + 1 doc mention; the rule lives in one place |
| W1-j, W3-t | `pnpm check:i18n` cross-locale parity | deleted `settings-locale-option-system` from the de catalog | exit 1, "missing id ... (present in locales/en/gui-settings.ftl)"; restored, exit 0 |
| W2-p | the `vue/no-restricted-syntax` rule | `v-if="!model"` in place of the shipped gate | exit 1 with the D112 message; and `v-show="!model"` exits 0, which is O-1's residual measured |
| W4-s | two pasted `check:i18n` runs | removed `close-discard-title` from `RUST_ONLY_IDS` | the id appears under "warning only" and the run still exits 0 (see M-7) |
| W5-a | `check:i18n`'s D62 gate + a reviewer read | read the gate's six conditions (`scripts/check-i18n.mjs:533-646`) and both topics | existence/hygiene covered as the row's MV column honestly says; the content half is the reviewer read, whose findings are I-5 |
| W5-b | the existing batch scenario | see I-1 | **GAP (known)** |
| W2-l | Task 3 Step 1's pasted validator re-run | artifact, not a mechanism | as described |
| W4-m | the dialog-callback unit coverage | -- | **GAP (known)**; the row below it states in its own text why the surface needs the Tauri runtime. Correct the row at the close; the observable rides the 1.x GUI-test-harness item |
| W4-w | explicitly "no, by nature" | -- | honest as written |

The two NEW gaps are both in W3 and both name producers this plan would BUILD:

- **W3-o** -> see I-2. No keyboard test over the enumerated combinations exists; only
  `Control+z` is ever pressed, and only inside U1.
- **W3-h** -> see M-2. `readModel` is unused in the file the row points at.

Everything else resolved to a real producer. Spot-verified in full rather than by name:
W1-a to W1-g (`locale-switch.spec.ts:175-300`), W2-a/b (`smoke.spec.ts:1765-1801`),
W2-e/f (E1 and its fire, `:1843-1855`), W2-g to W2-j (`:1859-1920`), W2-k (E2 and its
fire, `:1938-1952`), W2-m to W2-p (`editor-undo-redo.spec.ts:593-660` legs 1-3 plus the
lint rule), W3-a to W3-f (six named `test()` blocks, one per mutation path), W3-g/i/j,
W3-k/l/m (the granularity test's three labelled halves), W3-n, W3-p (U1, with its
in-test positive control), W3-q, W3-q2 (`smoke.spec.ts:2243`), W3-r, W3-s, W3-u,
W4-a to W4-g (`smoke.spec.ts:2043-2380` plus `:1429`), W4-h to W4-l (the four
`close_decision` unit cases; `set_editor_dirty` asserted as exactly `[true, false]` in
order at `smoke.spec.ts:1426`), W4-n, W4-u/v (the twelve-cell `reconfirm_decision`
matrix), W4-o/p, W4-q/r/t (the three-part parity split).

**Pre-table prose note (plan line 526):** it makes four claims. Two are false (the
locale hint, I-7; the batch string, I-1), one over-reaches (three of six ids have no
rendered-value assertion, M-1a) and one rests on a premise the suite contradicts
(sibling order, M-1b). Same paragraph, four claims, four defects of the
`a-normative-claim-is-scoped-down-to-its-producers-reach` family. Rewrite the paragraph
rather than patching clauses.

---

## Dimension summary

**Spec compliance across the branch: MET.** The amended spec 8.2 (both regions) and
D106-D112 describe the shipped tree, with the two enumeration imprecisions already
recorded as close items (M-4, M-5 of Task 1). D106's three-state control, D107's blank
seed and save-as flow, D108's derived save state and history, D109's guards and the
four-state close, D110's locale-aware shell, D112's two-term pre-session condition are
each present and each reachable in the shipped bundle.

**Test power / direction: two failures, both named above (I-3, I-4).** Presence is well
covered throughout -- the branch's mutation discipline is visible in the source and it
holds up: I re-ran the cell control on `close_decision` and the fire test on the save
marking, and both discriminate. What is missing is the second half in exactly the two
places the brief predicted, and in both the failure direction is data loss.

**House conformance:** conformant, with the exceptions recorded as findings.
`a-normative-claim-is-scoped-down-to-its-producers-reach` is violated four times in one
paragraph (M-1, I-7) and once in the ADR (M-4);
`an-edit-to-a-set-walks-to-its-neighbours-not-only-to-its-enumerations` and
`a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact` once (I-6);
`design-empirical-claims-reproducible` once, in shipped source (M-3);
`tests-ship-with-the-feature-never-after` three times (I-1, I-2, I-7).
`a-disabled-assertion-over-a-disjunction-proves-only-its-weakest-term`,
`a-fire-test-on-a-two-direction-surface-attacks-the-direction-not-the-presence`,
`a-failure-cost-comment-does-not-inherit-its-neighbours-guarantee` and
`a-statement-living-in-two-documents-is-diffed-against-its-twin-not-swept` are each
visibly applied in the shipped artifacts.

**Latitude:** no breach found. The one product file in the branch I could not place in a
task by inspection, `e2e/editor-rule-add-remove.spec.ts` (2 lines), turns out to be the
correct neighbour repair for the gate rename, landed in Task 4's own commit `1092eb7` --
the behaviour the house asks for, not a scope excursion. Per-task Files-list conformance
was verified on disk by the controller at each commit and I did not re-derive it.

**No-work-needed premises, run rather than weighed:**
- "Section 8.4 is deliberately NOT edited: its locale sentence is already true" ->
  RUN. `2026-07-08-muxsmith-v1-design.md:418` reads "system locale with manual override
  in app settings (takes effect live, without restart; D56) ... falls back to English
  per message". True of the shipped behaviour. Premise holds.
- "the same hard gate every task runs covers them" (the six new ids) -> RUN. Fire-tested:
  parity fails on a deleted de key. Holds.
- "the two reworded values ride the assertions that already read them through `en(id)`"
  -> RUN. False for both, and impossible in that form. See I-1, I-7.
- "no test in this suite asserts sibling order" -> RUN. Contradicted. See M-1b.
- "every current call site is a literal, measured" -> RUN. Three counter-examples. See M-4.
- "a second caller of that component" is the reentrancy trigger -> RUN. One caller pair
  today, both in `EditorView.vue`, both gated on `dirty`. Holds.

**Typography: clean.** Scripted over every ADDED line of the branch, product paths and
`docs/` separately, for em-dash, en-dash, figure dash, horizontal bar, Unicode minus,
curly quotes, ellipsis and NBSP: zero hits in both passes. German orthography inside
German values is correct throughout and is not a violation.

---

## Blocking list

1. I-1 -- a producer for `batch-profile-none` (both locales), in the existing batch scenario.
2. I-7 -- a producer for the reworded `settings-locale-label.hint`, via `enAttr`.
3. I-2 -- a keyboard case covering `Control+Shift+z`, `Control+y` and the `Meta`
   modifier, matching what `help/{en,de}/view-editor.md` promises.
4. I-3 -- a case that edits the model inside the save window, so the save-marking line's
   direction is pinned and not only its presence.
5. I-4 -- extract the `CloseDecision` -> dialog-key mapping and assert its four rows.
6. I-5 -- the terminology repair at all five sites (3 German, 2 English), not two.
7. I-6 -- the supersession marker on
   `2026-07-22-plan75-track-rule-add-remove-design.md:99-102`, dispatched with the
   fact-grep rule rather than a file list.
8. M-4 -- correct the ADR's "every current call site is a literal" sentence.

Recommended in the same wave, non-blocking: M-3 (date or drop the comment's counts),
M-5 (guard the keydown handler while the confirm is open), M-6 (three assertions for the
region-qualified locale path, ready to paste), M-1 (rewrite the pre-table paragraph),
item 9 of the rulings (one token in the lint selector).

Close actions the plan already owns and must not lose: the W4-m and W5-b row
corrections, the W3-o and W3-h row corrections (new), the Tier-2 catalog-budget
recompute to 54, the two widget 43-figures, M-4/M-5 of Task 1 on the spec, and the
Plan 13 fifth member.

---

## Harvest

Patterns of this branch as one artifact, for the house ledger.

**H-1. An acceptance row is believed for its own reasons regardless of which kind it
is; the built/existing split does not predict which ones fail.** This branch produced
four false rows: two naming an existing producer (W4-m, W5-b) and two naming a producer
the plan would build (W3-o, W3-h). The existing/built asymmetry that
`an-acceptance-row-naming-an-existing-producer-is-verified-by-finding-it` records is
real as a mechanism but does not bound the defect, because the actual protection is
elsewhere: a row survives if and only if some task's STEP LIST prescribes its producer
by name. Task 4's step list prescribed the six mutation-path cases individually and each
one exists; it did not prescribe a keyboard case, and W3-o's producer does not exist.
Suggested statement: *an acceptance row is discharged by the step that builds it, so a
row whose producer no step names is unbuilt whichever kind it is -- walk the map against
the step lists at plan authoring, not against the tree at plan close.*

**H-2. A mutation that leaves the mechanism working and swaps its DESTINATION is the
untested half of every dispatch table.** Both direction failures on this branch have the
same shape and neither is a missing test of the mechanism: `close_decision` picks the
right variant and every cell is pinned, but the variant-to-strings mapping one function
away is unreachable by any test; `doSave` marks a snapshot and the marking is pinned,
but WHICH snapshot it marks is not. In both cases the plan factored the decision out for
testability and left the dispatch inside the untestable caller. Suggested statement:
*where a decision is factored out of an untestable caller for testability, the mapping
from that decision to its effect is a second decision and moves out with it -- otherwise
the tests prove the branch is chosen and never that the chosen branch does the named
thing.*

**H-3. A sweep anchored on the changed FILES stops at the repository's documentation
boundary.** The gate-rename sweep found `EditorView.vue`'s own two doc-comment regions
(the task's Files list named them) and the neighbouring test file's header comment -- and
stopped, leaving the design document that the swept test file implements. The existing
entry `an-edit-to-a-set-walks-to-its-neighbours-not-only-to-its-enumerations` covers the
neighbour direction inside one document; this instance walks OUT of code into
`docs/superpowers/specs/`, where a falsified mechanism statement becomes ground truth for
the next plan. Suggested clause on that entry: *the neighbour set of a code change
includes the design documents that describe the mechanism, and they are found by grepping
for the FACT, never from the task's Files list, which by construction names only what the
task writes.*

**H-4. One paragraph, four claims, four reaches.** The plan's pre-table note is the
highest-density instance of `a-normative-claim-is-scoped-down-to-its-producers-reach`
this branch produced -- and it is the paragraph written specifically to explain why
certain observables need no row, i.e. the paragraph whose entire job is a no-work-needed
argument. Suggested clause: *a paragraph that exists to justify an absence carries no
producer of its own and is therefore the densest place a reach defect hides; run every
clause of it, not the conclusion.*

**H-5. A default made default by one task is tested by another task that never learns it
became the default.** D106 (Task 2) made "no stored override" the default state, which
means the shell now receives `navigator.language` verbatim. D110 (Task 6) built and
tested the shell lookup against bare tags. Neither review could see it: Task 2's did not
know a shell consumer existed, Task 6's saw a locale string arriving and had no reason to
ask what shape it arrives in. Suggested statement: *when a task changes which value is
the DEFAULT, every downstream consumer's test set is re-derived from the new default, not
from the value the consumer was written against.*

---

*Written by the whole-branch reviewer. Every figure in this document was measured at the
artifact named beside it; every mutation was restored and the restore verified by
content.*
