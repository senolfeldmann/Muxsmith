## Task 5: the discard guards in the editor (W4a)

Read first: this plan's D109 in full; **this plan's D112** (amendment 1), whose pre-session condition is the gate this task's step 2 argument and its case 5 both consume; `docs/ROADMAP.md`'s round-3 finding 2 second owner ruling (the guard's shape and its ordering) and the sharpened first one; `src/views/EditorView.vue` as Task 4 left it; `src/components/SettingsDialog.vue` (the native-`<dialog>` house pattern, its `defineExpose` shape and its Esc note); `e2e/smoke.spec.ts`'s existing view-switch case (which this task extends); the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Create: `src/components/ConfirmDialog.vue`
- Modify: `src/views/EditorView.vue` (the dialog mount, the two guarded call sites, **and the stale catalog-budget sentence in the file's own header doc block - Step 4c, half 2**)
- Modify: `locales/en/gui-editor.ftl` (three new ids)
- Modify: `locales/de/gui-editor.ftl` (the same three)
- Modify: `e2e/smoke.spec.ts` (the guard cases; the extended view-switch case; **the catalog-budget comment, recomputed - Step 4c, half 1**)
- Modify: `e2e/editor-undo-redo.spec.ts` (**the three named cases this task's own guard fronts with a confirm**, and only those: "open resets", "createBlank resets" and the failed-open case; Step 4b derives the set from a stated criterion, so a reviewer can reproduce it rather than trust the list. No other case in that file is touched - amendment 1, set corrected by amendment 2)

**Why the header-comment region is in scope rather than fenced off, and what it records.** This task adds three ids to `gui-editor.ftl`, which falsifies that file's own sentence about how many the catalog carries - the same ground Task 3's Files list gives for the two comment regions it repaired, and what `proc-normative-count-recomputed` exists to prevent. **The miss it also closes is on the record rather than quietly repaired:** Task 4's Step 7 named `src/views/EditorView.vue` and `e2e/smoke.spec.ts` together and required both budget comments recomputed; Task 4 corrected only the second, and its review and its delta review both graded that step MET, so the sentence has been stale across two packages. Task 5 does not inherit Task 4's authority for that - **its own authority is that its own three ids falsify the sentence again**, which is why the correction belongs here and not in a sweep with no owner.

**Interfaces:**
- Consumes: Task 4's `dirty`.
- Produces: `ConfirmDialog`, whose props are the minimum a second caller needs.

- [ ] **Step 1: `ConfirmDialog.vue`.** A native `<dialog data-testid="confirm-dialog">` with `showModal()`, mirroring `SettingsDialog.vue`'s imperative pattern: props `title`, `message`, `confirmLabel`, `cancelLabel` (all strings, so the caller resolves its own Fluent text and the no-raw-text rule is satisfied by binding), `defineExpose({ ask })` returning `Promise<boolean>`, resolved `true` by the confirm button and `false` by the cancel button, by `close` and by Esc (the native cancel, which reads as "do not discard" - the safe direction, and the same Esc semantics the settings dialog documents). The confirm button carries `data-testid="confirm-dialog-confirm"`, the cancel button `confirm-dialog-cancel`. A doc comment states that the component exists rather than an inline dialog so a second caller can reuse it, and that its props are the minimum for that.

- [ ] **Step 2: the two guarded call sites.** `EditorView` mounts one `ConfirmDialog` with the three discard strings and `settings-cancel` as the cancel label (the same cross-view reuse the editor's Save already makes of `settings-save`). Then:
  - `pickAndOpen` begins, after its existing busy guard, with `if (dirty.value && !(await confirmEl.value?.ask())) { return; }` - **before** the open dialog, so the ordering is confirm, then file dialog, then replace. A cancelled file dialog after a confirmed discard leaves the model untouched, because nothing is discarded until a load succeeds.
  - `createBlank` gains the same guard. Because `createBlank` is synchronous today, it becomes `async` and its click handler awaits it; nothing else about it changes.
  - `openPath` gains NO guard: it runs after the dialog, and the recents-click path that also reaches it is unreachable while the editor holds a model (the recents section is gated on the pre-session condition Task 4 defines under D112, whose `!model` term alone carries that unreachability).

- [ ] **Step 3: the catalog, both locales, fenced.** Append to `locales/en/gui-editor.ftl` exactly:

```
## Discard confirmation (D109)

editor-discard-title = Unsaved changes
editor-discard-message = The profile in the editor has unsaved changes. Continuing replaces it and the changes are lost.
editor-discard-confirm = Discard changes
```

  and to `locales/de/gui-editor.ftl` exactly:

```
## Verwerfen-Bestätigung (D109)

editor-discard-title = Nicht gespeicherte Änderungen
editor-discard-message = Das Profil im Editor hat nicht gespeicherte Änderungen. Wenn du fortfährst, wird es ersetzt und die Änderungen sind verloren.
editor-discard-confirm = Änderungen verwerfen
```

- [ ] **Step 4: the tests, in `e2e/smoke.spec.ts`'s new guard describe.**
  - **Case 1, Open over unsaved changes, confirmed.** Open a profile, edit a field, click Open. **Absence check G1:** no `plugin:dialog|open` call recorded yet, while `confirm-dialog` is visible with its en message. **Its fire is the same counter after clicking confirm**, which must be non-zero, and the second profile must then be in the editor.
  - **Case 2, Open over unsaved changes, cancelled.** Same up to the dialog, then cancel: `plugin:dialog|open` still not recorded, the edited value still in the field, the dirty state still true (Undo still enabled).
  - **Case 3, Open with no unsaved changes, in both of its reachable shapes.** (i) Open a profile and click Open again without editing: **absence check G2**, `confirm-dialog` count 0, and `plugin:dialog|open` recorded immediately. (ii) **The after-a-save shape, which is W3-q2's producer:** open, edit (the confirm now appears on an Open click - assert it, then cancel), Save successfully, then click Open again and assert the confirm does NOT appear. **The two clicks in one test are each other's control**: the first proves the guard can fire in this scenario, the second proves the save cleared the state. Without leg (ii) a `savedSnapshot` frozen at the load baseline ships silently, because every other assertion in this package passes with it frozen. **Its fire is leg (ii)'s own first click**, not case 1's.
  - **Case 4, New over unsaved changes.** Edit, click New, confirm: the seed replaces the edited profile. Cancel: it does not. (If the owner strikes D109 decision 2, this case inverts to asserting no dialog - **and it is not the only thing that moves**: the `createBlank resets` case in `e2e/editor-undo-redo.spec.ts` inverts with it and Step 4b's criterion loses a guarded function, both named in that decision's own sensitivity clause.)
  - **Case 5, the recents affordance is unreachable while a profile is held.** With a profile open, `editor-recents` count 0; **its fire** is count 1 (or the seeded recent button's presence) in the pre-session state of the same test.
  - **Case 6, the view-switch invariant, extended rather than duplicated.** The existing case "the editor tab stays mounted across a switch to Jobs and back" gains: the field is edited before the switch, and after the round trip the edited value, the enabled Undo button and the absence of any `confirm-dialog` are all asserted. This is R22's assertion and the reason nothing was built for it.
  - Run `assertNoSeriousA11yViolations` with the confirm dialog open.

- [ ] **Step 4b (amendment 1, criterion corrected by amendment 2): repair the cases in `e2e/editor-undo-redo.spec.ts` that this task's own guard fronts.**

  **The criterion, and it derives from what Step 2 guards rather than from one of the ways in.** Step 2 puts the confirm on **two functions**, `pickAndOpen` and `createBlank`, and on neither `openPath` nor anything else. A case in that file is a member of the affected set **iff it activates a control bound to one of those two functions at a moment when `dirty` is true**. Three facts make that decidable by reading, with no judgement left over:
  - **The controls are exactly two**, read from `EditorView.vue`'s template rather than assumed: `editor-open` is bound to `pickAndOpen` and `editor-new` to `createBlank`. `editor-recent-profile` is bound to `openPath`, which this task deliberately leaves unguarded, so a recents click is never a member - and it is unreachable while the editor holds a model anyway (D112's condition).
  - **`dirty` is true at a click iff a model mutation lies between that click and the most recent baseline before it.** A baseline is established by a successful open, a create, or a successful save (D108 decisions 3 and 8); a mutation is any of the six funnel functions D108 decision 1 enumerates, reached in a test through a field edit, a rule Add or Remove, or a drag-reorder.
  - **The derivation is therefore mechanical:** list every activation of those two controls in the file, and for each read backwards to the nearest baseline and ask whether a mutation lies between. **A case with no such activation is not a member however dirty it gets**, and a case that activates a guarded control from a clean baseline is not a member either.

  **What this criterion depends on, named so it cannot go stale silently.** It is closed over one fact and one only: **the set of functions Step 2 guards**, which D109 decisions 1 and 2 fix at `pickAndOpen` and `createBlank`. A new test case can never falsify the rule - that is the whole point of writing it against the mandate - but a change to that pair can, and the plan already contains the one documented event that would do it: **D109 decision 2's sensitivity clause**, which is the recorded route by which `createBlank` stops being guarded. That clause names this criterion in return. **If the guarded pair ever changes, this rule and its enumeration are re-derived from the new pair before anything else is done with them**, and the enumeration below is a measurement of the current pair rather than an independent claim.

  **Why the criterion is stated this way rather than as "builds history, then opens again":** that earlier phrasing was scoped to one of the two entry points, so it structurally could not see a case that replaces the editor's content through New. It was correct for the file as it stood and was falsified by a later ruling that added exactly such a case to Task 4. **A criterion narrower than the mandate it serves regenerates the defect on the next addition** (`a-normative-claim-is-scoped-down-to-its-producers-reach`, one level up), which is why the rule above is written against the guarded functions and not against a control.

  **The set, re-derived from that criterion against the file as it stands: THREE cases.**
  - **"open resets: opening a second profile clears both Undo and Redo"** - its first Open runs from a clean baseline and is not a member; its second Open follows a field edit and is.
  - **"createBlank resets: New after edited history clears both Undo and Redo"** - a field edit, then New. **This is the member the earlier criterion could not see**, and it entered the file through Task 4's own fix round after amendment 1 was written.
  - **"a failed open ..."** (the failed-open case D108 decisions 9 and 10 own) - its first Open is clean and is not a member; its second, after a field edit, is.

  **Every other case in the file fails the criterion, and the reason is stated per case rather than by exclusion:** the shared open helper every test starts with runs before any model exists, so its Open is clean; the six mutation-path cases mutate after that single open and activate no guarded control again; granularity, truncation, the depth cap and U1 activate no guarded control **beyond that shared helper's own Open**, which is the clean activation classified in the clause before this one; "save marks rather than clears" edits and then saves, which re-establishes the baseline, and activates no guarded control afterwards; and amendment 1's D112 three-leg case activates Open twice but never mutates the model, so both clicks run from a baseline and `dirty` is false at each.

  - **The repair, in each of the three:** between the activation of the guarded control and the assertions that follow it, assert that `confirm-dialog` is visible, then click `confirm-dialog-confirm`.
  - **The added member takes the IDENTICAL repair, and that is a measurement rather than an assumption** (amendment 2, B-2). Step 2 mounts **one** `ConfirmDialog` in `EditorView`, and both guarded functions await that same instance's `ask()`, so the dialog that appears carries the same `confirm-dialog` and `confirm-dialog-confirm` testids whichever control opened it. The one difference between the entry points does **not** reach the repair, and the ground is what is already mocked rather than what is called: after a confirmed Open the flow continues into the file dialog (`plugin:dialog|open`), while after a confirmed New the seed assignment queues the validate-on-edit watcher, so `validate_profile_model` DOES fire - **both commands are already in the shared `openProfile` helper's mock set**, which is why the New case needs **no additional mock, no additional wait and no different locator**. The operative word is *additional*: an implementer who reads "New fires no IPC" would be wrong about the mechanism and could raise a question against a call that is already covered. If a genuinely uncovered command appears, that is NEEDS_CONTEXT, not a mock invented at the keyboard.
  - **No existing assertion in any of the three is removed, weakened, reordered or reworded, and no case changes what it is about.** Each keeps testing exactly what it was written to test, over an editor that is genuinely dirty - which is the state a real user reaches, and the reason the alternative repair (re-establishing a baseline before the guarded click so the confirm never fires) is rejected: it would swap the case's own subject to dodge a mechanism. `proc-proposed-safeguard-stays` binds here too - the assertions these cases already carry are the safeguard.
  - **Task 4 does not pre-adapt them.** `ConfirmDialog` does not exist until this task's Step 1, so those clicks would fail there. The task that introduces the guard repairs the cases its guard changes, which is the same shape Task 3 and Task 4 already use for the comment regions their own edits falsify.
  - **This produces no new observable and the acceptance map gains no row:** W4-a to W4-c already grade the guard on the Open path and W4-e on the New path. The repair keeps three existing producers alive rather than producing anything new.

- [ ] **Step 4c (amendment 2, both halves of the pair after its fix round): the two catalog-budget comments this task's own three ids falsify.** **There are exactly two, they are named here rather than anywhere else, and this step carries both** - `src/views/EditorView.vue`'s header sentence and `e2e/smoke.spec.ts`'s budget comment. Splitting the instruction across two steps is what produced the ambiguity a Task-5 implementer met on code contact; one owner for the pair is the repair.

  **Half 1, `e2e/smoke.spec.ts`'s budget comment.** Its END STATE is fenced rather than a replacement pair, because this task's Step 4 edits that file and the comment may already carry the new figure by the time this step runs; either way the file must end with exactly:

```
// budget is 54 (42 labels + 1 save-surface note + 4 generic action keys +
// 1 rule-grid ordinal + 3 profile-creation keys + 3 discard-confirmation
// keys, D109).
```

  The decomposition is checked rather than copied: 42 + 1 + 4 + 1 + 3 + 3 = 54, and the four generic action keys are add, remove, undo and redo. **A recount that disagrees is a finding -> NEEDS_CONTEXT with both numbers pasted.**

  **Half 2, `src/views/EditorView.vue`'s header sentence.** Recompute the count from the file first - `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl` after Step 3, which must equal **54**; **a disagreement is a finding -> NEEDS_CONTEXT with both numbers pasted**, not a fence adjusted at the keyboard. Then replace exactly

```
// packages did add to it: `gui-editor.ftl` carries 49 ids today, three of
// them this view's own New affordance (`editor-action-new`,
// `editor-empty`, `editor-unsaved`, D107). The Open button, the
```

  with exactly

```
// packages did add to it: `gui-editor.ftl` carries 54 ids today, eight
// of them this view's own affordances: profile creation
// (`editor-action-new`, `editor-empty`, `editor-unsaved`, D107),
// undo/redo (`editor-action-undo`, `editor-action-redo`, D108) and
// the discard confirmation (`editor-discard-title`,
// `editor-discard-message`, `editor-discard-confirm`,
// D109). The Open button, the
```

  Nothing else in that doc block changes. The eight decompose as this package's own additions to this view (3 + 2 + 3), and 46 + 8 = 54 against the count the authoring section measured before the package started.

- [ ] **Step 5: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green - **and the three repaired cases in `e2e/editor-undo-redo.spec.ts` are part of it**, so a repair that missed one shows up as a red gate here rather than at the push. **The `gui-editor.ftl` recount must be 54, and the budget comments it governs are exactly two - `e2e/smoke.spec.ts`'s and `src/views/EditorView.vue`'s header sentence - both of which Step 4c carries, both halves fenced there.** This step asserts that they were corrected; **Step 4c is the only place that instructs it**, which is the pair kept in one owner rather than split across two steps. `git diff --stat` covers exactly the six files in the Files list.

- [ ] **Step 6: commit.**

```bash
git add src/components/ConfirmDialog.vue src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts e2e/editor-undo-redo.spec.ts
git -c commit.gpgsign=false commit -m "editor: confirm before unsaved changes are replaced, ahead of the file dialog" -- src/components/ConfirmDialog.vue src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts e2e/editor-undo-redo.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** that the surface is an in-app `<dialog>` rather than a native dialog (and that therefore no capability changes); that the guard sits in `pickAndOpen` and `createBlank` and not in `openPath`; that a cancelled file dialog discards nothing; the three fenced strings and the reuse of `settings-cancel`; that there is no third "save first" button; that the view-switch case is extended rather than duplicated; that no rule-removal confirmation is added anywhere; **and, from amendment 1 as amendment 2 corrects it**: the membership criterion in Step 4b and the three cases it yields; that the repair is a visibility assertion plus a confirm click and never a removed, weakened, reordered or reworded assertion; that the New case takes the identical repair rather than a different one; that no case there is rewritten to avoid the guard instead; that the file is otherwise untouched; and, from amendment 2, that **both** budget comments are corrected here and that Step 4c is their single owner, its two fenced texts, that the count is recomputed from the catalog rather than copied from this plan, and that the criterion's dependency on the guarded pair is stated rather than assumed.

---

