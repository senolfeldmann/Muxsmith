## Task 4: undo/redo over the mutation funnel, and the save state derived from it (W3)

Read first: this plan's D108 in full and the authoring section's mutation-path enumeration with both expressions; **this plan's D112 in full** (amendment 1, the owner's failed-load ruling of 2026-07-31); `docs/ROADMAP.md`'s v1.x entry "Editor undo/redo, all operations" (the requirement set); `src/views/EditorView.vue` as Task 3 left it, in full; `src/editor/widgets/TextWidget.vue` (the per-keystroke binding this task's granularity rule works around); `e2e/editor-rule-add-remove.spec.ts` in full (its bare-mount cases, its header doc, and the Add/Remove interactions the mutation-path cases reuse); **`e2e/smoke.spec.ts`'s recents describe** (its `settingsWith` helper, which is where the `AppSettings` fixture shape D112's case needs is written out, and its `editor-recent-profile` locator, the only one of the recents surface's two testids that any spec currently uses - **`editor-recents` appears nowhere under `e2e/`** and is read from `src/views/EditorView.vue`'s template, where both are defined); `eslint.config.js` (its per-`.vue` rules block, which Step 4c extends, and the comment above the existing rule, which is the form the new one follows); `e2e/mount.ts` (`mountComponent`, `readModel`); the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src/views/EditorView.vue` (the history state, the push rule inside the existing watcher, the coalescing boundary, undo/redo functions, the keyboard handler, `sessionActive` becoming a computed, the two funnels' baseline calls **and the load-bearing-order comment above them**, the two buttons, **`doSave`'s post-write region** - the one line that marks the written profile, and the only part of `doSave` this task touches - **and, per amendment 1, the `nothingOpenedOrCreated` computed plus the two template gates that read it**: the `editor-empty` paragraph's and the `editor-recents` section's)
- Modify: `locales/en/gui-editor.ftl` (two new ids)
- Modify: `locales/de/gui-editor.ftl` (the same two)
- Modify: `e2e/editor-rule-add-remove.spec.ts` (**the header doc sentence this package falsified**: the bare-mount cases' reason is `sessionActive`, not `currentPath`)
- Create: `e2e/editor-undo-redo.spec.ts` (the mutation-path table, the undo/redo cases, **and amendment 1's pre-session/failed-load case**)
- Modify: `e2e/smoke.spec.ts` (**the catalog-budget comment only**, recomputed)
- Modify: `eslint.config.js` (**the one `vue/no-restricted-syntax` entry of Step 4c, inside the EXISTING per-`.vue` rules block, and nothing else in the file** - amendment 1)

**What amendment 1 moved in this list, stated so it is checkable rather than asserted.** The gate condition and its two gates live in `src/views/EditorView.vue` and its case lives in `e2e/editor-undo-redo.spec.ts`; both were already members, and D112 adds no catalog string, so neither `.ftl` moves. **`eslint.config.js` is the one addition**, and it is therefore also added to the commit block below. It is a named region: the rules block gains one entry, no other rule and no other part of the config changes, and **no gate part is added** - `pnpm lint` already runs `eslint .`.

**Interfaces:**
- Consumes: Task 3's `createBlank` and `sessionActive`.
- Produces: `dirty`, which Tasks 5 and 6 gate their guards on, and `savedSnapshot`, which `sessionActive` now derives from.

- [ ] **Step 1: the history state and the push rule.** Add to the script, with doc comments carrying the reasoning D108 records:
  - `const history = ref<string[]>([]);`, `const position = ref(-1);`, `const savedSnapshot = ref<string | null>(null);`, `let coalesce = false;`
  - `sessionActive` changes from a ref to `const sessionActive = computed(() => savedSnapshot.value !== null);`, and Task 3's two assignment sites drop their `sessionActive.value = true` in favour of establishing the baseline (below). The doc comment states that the two facts are established at the same two moments by construction.
  - `const dirty = computed(() => savedSnapshot.value !== null && history.value[position.value] !== savedSnapshot.value);`
  - `const canUndo = computed(() => position.value > 0);` and `const canRedo = computed(() => position.value < history.value.length - 1);`
  - A `resetHistory(profile: Profile | undefined)` helper used by `openPath` and `createBlank`, with **both branches fenced** (D108 decisions 8 and 9): given a profile, history becomes a single entry holding `JSON.stringify(profile)`, `position` 0, `savedSnapshot` that same string, `coalesce` false; given `undefined` - the failed-load branch, where `doc.profile` is null and the diagnostic carries the parse error - history becomes empty, `position` `-1`, `savedSnapshot` `null`, `coalesce` false. `openPath` calls it with `doc.profile ?? undefined` on the same value it assigns to the model, so the two can never disagree.
    **The call site's POSITION is load-bearing and is inherited rather than invented**: `resetHistory` takes the place of the `sessionActive.value = true` assignment Task 3 put **before** the model assignment, and it must stay before it for the two reasons that made that order load-bearing there and one more of its own - `sessionActive` (now derived from `savedSnapshot`) must already be true when the watcher fires on the assignment, or the loaded profile is never validated; and `history[0]` must already equal the serialized model, or the push rule sees a difference and appends a second entry, so a freshly opened profile would start one step deep and dirty. **Task 3's comment naming that order as load-bearing is updated in this task to name `resetHistory`** rather than the assignment it replaces, since the requirement outlives the statement that carried it.
  - Inside the EXISTING `watch(model)`, before the validation round trip, the push rule:

```ts
  const serialized = JSON.stringify(value);
  if (serialized !== history.value[position.value]) {
    if (coalesce) {
      history.value = [...history.value.slice(0, position.value), serialized];
    } else {
      history.value = [...history.value.slice(0, position.value + 1), serialized];
      position.value = history.value.length - 1;
      if (history.value.length > HISTORY_DEPTH) {
        history.value = history.value.slice(1);
        position.value -= 1;
      }
      coalesce = true;
    }
  }
```

    with `const HISTORY_DEPTH = 100;` and a comment stating why the rule is a comparison rather than a flag (an undo-driven assignment equals the entry at the new position, so it cannot push; no applying-history latch exists to get wrong), and what the depth costs (the measured snapshot sizes).

- [ ] **Step 1c: mark the profile that was WRITTEN, in `doSave`.** Immediately after `await saveProfile(path, profile)` resolves - **inside the existing `try`, so a failed save leaves the state dirty** - add exactly:

```ts
    savedSnapshot.value = JSON.stringify(profile);
```

  **The value is the captured `profile`, never `history.value[position.value]`, and the difference is the whole point of the line.** Two awaits sit between Task 3's capture (`const profile = model.value;`) and this point - the save dialog on the needs-path branch, and the write itself - and `saving.value = true` disables only `editor-save`, `editor-new` and `editor-open`, not the widgets, so the editing surface stays live through both. Marking the live position would therefore record a state that was never written whenever the model moved inside either window: `dirty` would go false while the file holds the older profile, and **every guard in the D109 family would disarm over unsaved changes** - the data-loss direction, where D108 decision 4 promises annoyance. Marking the captured value instead makes the property structural: when nothing moved, `profile` is the same object the last push serialized, so the string equals `history[position]` and `dirty` is false as intended; when it moved, the two differ and `dirty` stays true.

  **Why the parity precedent does not license the live read**, stated because this plan cites that precedent and an earlier draft over-read it: in `mkvtoolnix-gui` the sequence `updateConfigFromControlValues(); p.config.save(); p.savedState = currentState();` is **fully synchronous**, so there `currentState()` IS what was written. The borrowed shape carries that condition with it, and a flow with an `await` between the capture and the mark does not meet it. Comment the line accordingly: it names the written value, and names synchrony as the condition the precedent's own form depends on.

  **Nothing else in `doSave` changes**: Task 3 owns its dialog branch, its capture discipline and its recents write, and this task adds one line after the write. Omitting the line entirely is the opposite defect and is just as visible: `dirty` would never return to false and every guard would fire on a profile that was just saved, which is the disposition the owner overruled when he gated the family on save state.

- [ ] **Step 2: the coalescing boundary.** `addRule`, `removeSelectedRule` and `onDrop` each set `coalesce = false;` as their first statement, with one shared comment stating why focus alone is not enough (two consecutive clicks of the same button never move focus). The editor's root `<section>` gains `@focusout="coalesce = false"`, with a comment recording the measured fact that `focusout` bubbles where `blur` does not.

- [ ] **Step 3: undo, redo, and the keyboard.** Two functions applying a history entry - moving `position`, assigning `model.value = JSON.parse(history.value[position.value]) as Profile`, clearing `selectedIndex` (a selection maps to an identity, not a position, the rule `onDrop` already follows) and setting `coalesce = false` - each guarded by `canUndo`/`canRedo` **and by `model.value` being set** (D108 decision 10: the action row and the keyboard handler both sit outside the `v-if="model"` wrapper, so neither may apply an entry while the editor holds nothing). Then a `@keydown` handler on the editor's root section, whose condition set is fixed here and whose comment states the no-per-OS-branch reasoning:
  - ignore the event entirely when its target is a text-entry control - `TEXTAREA`, or `INPUT` whose `type` is one of `text`, `search`, `url`, `tel`, `password`, `email`, `number` - so the browser's native character-level undo keeps working while typing;
  - undo on `(ctrlKey || metaKey) && !shiftKey && key.toLowerCase() === "z"`;
  - redo on `(ctrlKey || metaKey) && shiftKey && key.toLowerCase() === "z"`, or `(ctrlKey || metaKey) && key.toLowerCase() === "y"`;
  - `preventDefault()` only on a handled combination.

- [ ] **Step 4: the two buttons and their catalog ids.** In the action row after New and Open: `data-testid="editor-undo"` / `editor-redo`, `:disabled="!model || !canUndo"` / `:disabled="!model || !canRedo"` (the model term per D108 decision 10, since this row renders outside the editing surface's own gate), labels `{{ $t("editor-action-undo") }}` / `{{ $t("editor-action-redo") }}`, no `title`. Append to `locales/en/gui-editor.ftl`'s generic-action section exactly

```
editor-action-undo = Undo
editor-action-redo = Redo
```

  and to `locales/de/gui-editor.ftl` exactly

```
editor-action-undo = Rückgängig
editor-action-redo = Wiederholen
```

- [ ] **Step 4b (amendment 1): the pre-session gate condition, defined once (D112).** Two edits, both fenced, and nothing else about the template moves.
  - In the script, immediately below `sessionActive`'s new computed form from Step 1:

```ts
// D112 (owner ruling 2026-07-31): the pre-session state -- nothing has
// been opened or created at all -- and the ONE definition both surfaces
// that may appear only in that state read. Two terms, because two facts
// have to be absent at once: `model` carries "the editor holds
// something", `currentPath` carries "a file has been bound to the
// editor". A load that resolves but fails to parse leaves the second set
// and clears the first (`openPath` sets `currentPath`, then assigns
// `doc.profile ?? undefined`), and that state is the one the ruling is
// about: the path line names the failing file and the panel carries the
// parse error, so a second sentence saying no profile is open, plus a
// recents list offering a fresh start, contradict what is already on
// screen.
//
// NOT `sessionActive`: this task derives it from `savedSnapshot`, which
// the failed-load branch nulls (D108 decision 9), so `!sessionActive` is
// TRUE in exactly the state these two surfaces must stay hidden in. NOT
// `!model` alone: that is the gate Task 3 shipped, and it is what renders
// both surfaces over a failed load today.
const nothingOpenedOrCreated = computed(
  () => !model.value && currentPath.value === null,
);
```

  - In the template, the two gates Task 3 wrote become the two below, and no other gate is touched:

```
      v-if="nothingOpenedOrCreated"
```

    on the `editor-empty` paragraph (Task 3 wrote `v-if="!model"`), and

```
      v-if="nothingOpenedOrCreated && recents.length"
```

    on the `editor-recents` section (Task 3 wrote `v-if="!model && recents.length"`).

  **What stays exactly as it is, because the ruling keeps it rendering in the state it changes:** `<template v-if="model">` on the editing surface, `v-if="currentPath"` on the path line with its `v-else-if="sessionActive"` unsaved branch, and `v-if="diagnostics.length"` on the diagnostics section. **Task 3's own steps are not rewritten** - they record what that task built, measured and committed, and the gates above name the state they start from so the sequence stays legible (`proc-supersede-never-overwrite`).

- [ ] **Step 4c (amendment 1): the standing guard for the property Step 4b establishes (D112's standing-guard decision).** Add exactly this entry to `eslint.config.js`, as the FIRST rule in the existing `**/*.vue` rules block, immediately above `@intlify/vue-i18n/no-raw-text`. Nothing else in that file changes, and **no gate part is added**: `pnpm lint` already runs `eslint .`.

```js
      // D112 (owner ruling 2026-07-31): the pre-session state is ONE named
      // computed, `nothingOpenedOrCreated`, and a render gate that asks
      // `!model` directly is the defect that decision exists to remove --
      // that expression is also true after a load that failed to parse,
      // where the editor must NOT offer its pre-session surfaces. Scoped by
      // directive name to `v-if`/`v-else-if`, so the `:disabled="!model ||
      // !canUndo"` bindings D108 decision 10 requires stay legal: those gate
      // an ACTION on whether there is content, not a RENDER on whether
      // anything was ever opened or created.
      "vue/no-restricted-syntax": [
        "error",
        {
          selector:
            "VAttribute[directive=true][key.name.name=/^(if|else-if)$/] UnaryExpression[operator='!'] > Identifier[name='model']",
          message:
            "A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112).",
        },
      ],
```

  **Falsifiability, in the form this plan requires - the expression, a red state with its exact expected non-zero result, and the green end state - with every figure measured at amendment time against the tree as Task 3 left it** (the runs are pasted in the amendment report; re-derive rather than trust them):
  - **The instrument exists before the rule is written.** `eslint-plugin-vue` is a direct devDependency pinned in `package.json` (**10.9.2** at amendment time, read from the installed package's own manifest), and that version ships `vue/no-restricted-syntax`. If either is false in the tree this task meets, that is a finding: NEEDS_CONTEXT with the run pasted, not a rule adjusted at the keyboard.
  - **RED, the pre-state: exactly 2 errors of `vue/no-restricted-syntax`** in `src/views/EditorView.vue`, one on the `editor-empty` paragraph's gate and one on the `editor-recents` section's. **The two gates are the fence, not their line numbers**, which move as this task's other steps add script above them. If Step 4b is already applied when this step runs, restore those two gates for the red run and re-apply them: the red state is the tree as Task 3 left it.
  - **GREEN, the end state: 0**, with Step 4's two `:disabled="!model || !canUndo"` / `:disabled="!model || !canRedo"` bindings PRESENT in the file. That is the over-match control and it is not optional: a selector that also caught those would make this rule and D108 decision 10 mutually unsatisfiable, and the green run is what proves it does not.
  - **The selector's own enumerated set is fired member by member**, because a set inside an instrument is a claim like any other: `v-if` fires in the red run above, and `v-else-if` is fired separately by pointing the existing `v-else-if` at `!model` once and watching the rule report it. A regex branch that never fires is a branch that was never tested.

- [ ] **Step 5: the mutation-path coverage, as an enumerated table.** In `e2e/editor-undo-redo.spec.ts`, a table with one entry per mutation path measured at authoring, each generating its own `test()` on a fresh page. **The set is the six functions the authoring expression returned, and it is closed:** `setFieldValue`, `setTracksUnmatched`, `setRuleValue`, `onDrop`, `addRule`, `removeSelectedRule`. Each case, on the served app with mocked IPC: open a profile whose fixture reaches that path, perform the mutation through the real control, then assert (a) Undo is enabled, (b) one Undo restores the pre-mutation rendering, (c) Redo re-applies it. **Per path, not one test for the mechanism**, and the entry names the function it covers so a reviewer can map the table to the expression's output.
  - **Re-derive the set before writing the table**, with both authoring expressions pasted into the report: the whole-value expression and the in-place-mutation expression with its own fired control. **A seventh mutation path is a finding: NEEDS_CONTEXT, not a seventh row invented at the keyboard.**

- [ ] **Step 6: the remaining cases in the same file.**
  - **Granularity, three halves in one flow.** Type several characters into the pattern field: one Undo restores the field's pre-typing value in full (the burst is one step). Then type, move focus to another control, type again: two Undos are needed, one per burst. Then click Add twice: two Undos are needed. Playwright's `fill()` dispatches one input event and its own `change`, so a test that needs two bursts moves focus explicitly between them (measured).
  - **Truncation:** undo once, then edit; Redo is disabled.
  - **Save marks rather than clears:** open, edit, Save (mocked), then Undo is still enabled and one Undo restores the pre-edit state.
  - **Open resets:** with history built, open another profile; Undo and Redo are both disabled.
  - **A failed open clears rather than keeps** (D108 decisions 9 and 10): with history built and Undo enabled, open a second path whose mocked `load_profile` returns a document with `profile: null` and a parse diagnostic. Assert, in this order: the diagnostic renders (the section is gated on content, so a failed open still explains itself); Undo and Redo are both disabled; and the editing surface is gone. **Its own control is the state before the failed open**, where Undo was enabled in the same test - so a test that passes because Undo is never enabled anywhere cannot be mistaken for this one passing.
  - **The depth cap:** drive more than `HISTORY_DEPTH` distinct discrete mutations (Add repeated), then assert that Undo cannot reach the original state - the count is derived from the constant, not hardcoded twice.
  - **Absence check U1, the text-entry exemption.** With focus in the pattern field, the undo combination must NOT change the model (`readModel` on a mount-harness case, or the rendered rule count on the served app). **Its fire is in the same test:** the identical combination with focus on a button DOES undo. Two runs of one exemption, so a branch that swallows everything cannot pass.
  - **The mount-harness property is preserved and asserted by its own file:** `e2e/editor-rule-add-remove.spec.ts`'s bare-mount cases must pass unchanged, and its header doc is corrected to name `sessionActive` as the reason.
  - **Amendment 1's case: a failed load hides both pre-session surfaces, and the pre-session state still shows them** (D112). ONE test, three legs in one flow, so each leg is a control for the others. Its scenario mocks exactly five commands: `detect_mkvmerge` with `MKVMERGE_INFO`; `get_settings` resolving with a settings object whose `recent_profiles` carries the path leg 2 opens (the `settingsWith` shape `e2e/smoke.spec.ts`'s recents describe writes out); `plugin:dialog|open` queued as `[<that path>, <the failing path>]`; `load_profile` queued as `[<a document carrying a profile>, <a document whose profile is null and whose config_diagnostics carries one parse-error entry>]`; and `validate_profile_model` resolving with a diagnostic-free report. `set_settings` is deliberately NOT mocked - `e2e/mocks.ts`'s own fallback answers it, which is the fixture shape the shipped recents cases already use. **The model is never edited in this flow**, so `dirty` stays false and the discard guard Task 5 later adds to `pickAndOpen` cannot change what this test does.
    - **Leg 1, before any click - this is absence check P2's FIRE:** `editor-empty` is visible and `editor-recents` has count **1**.
    - **Leg 2, after the successful open:** `editor-empty` and `editor-recents` both count **0**. This is the gate Task 3 shipped, asserted so that leg 3 cannot be read as covering it.
    - **Leg 3, after the failing open - P2's zero:** `editor-empty` and `editor-recents` both count **0**, AND the open-path line renders the failing path (`batch-profile-current`), AND the parse-error diagnostic renders.
    - **Why leg 3's zero on the recents section is not vacuous:** that gate is a conjunction, so a zero could mean either term is false. `recents` is non-empty throughout this flow - the `get_settings` mock seeds one path, and each successful open writes another through `rememberRecentProfile` - so the term that is false in leg 3 is the gate, and leg 1's count of 1 is that same list measured through the same locator before the flow starts.

- [ ] **Step 7: verification.**
  - **Absence check S1, the saved position is never marked from LIVE state, by either route.** `grep -nE 'savedSnapshot\.value *= *(history|JSON\.stringify\(model)' src/views/EditorView.vue`, expected **0** on the end state. **The alternation has two members and both are fired**, each against its own synthetic line - `savedSnapshot.value = history.value[position.value];` and `savedSnapshot.value = JSON.stringify(model.value);` - because firing one member leaves the other unproven, the same rule this plan applies to its own gate audit's per-alternative fires. The second member is not decoration: it is the **likelier** re-break, since the fixed line is `JSON.stringify(profile)` and a simplifier reaching for the model finds `model.value` in scope right there, so the wrong version differs from the right one by one word. **This is not the widening D1 declined:** a second exact expression adds no prose surface and cannot produce triage, where a broader *name* pattern would. **Its pre-state is empty by construction** (neither line exists yet), so the two fires are the only thing that makes the zero mean anything. **What it does not cover, because a grep cannot follow a binding:** the same defect written through a local const (`const live = history.value[position.value]; savedSnapshot.value = live;`) escapes both members. **This check exists because the defect it catches was introduced by a fix round of this very plan**, which is where a structural property gets quietly re-broken: the assignment reads correct in isolation and is wrong only in the presence of the two awaits above it.
  - **The behavioural gap check, and why it is not prescribed here - a deliberate, argued gap rather than an omission.** A test that moves the model INSIDE one of the two awaits would be the direct producer, and the existing harness cannot express it: `e2e/mocks.ts` resolves a queued value immediately and its scenario crosses a `page.addInitScript` serialization boundary ("it must not close over anything from this module's scope beyond the `scenario` argument"), so there is no way to hold a mocked `plugin:dialog|save` open until the test releases it; a Playwright action issued after the click races the microtask that resolves it, and this project has an owner call against flakiness. A releasable mock response is **new test infrastructure**, which is the one exemption `tests-ship-with-the-feature-never-after` names, so it is surfaced for controller routing rather than written here as "coverage follows later". What ships instead is stronger than a comment and weaker than that test: the fix makes the property structural, S1 pins the structure, and the report states the residual.
  - **Absence check D1, no second save-state mechanism** (R28, and it costs one line because its red state is already measured). `grep -nEi "dirty|isDirty|unsaved|modified" src/views/EditorView.vue`. **RED, the pre-state, already measured on the baseline: 0 lines**, with the control that the same pattern returns `hasBeenModified`/`savedState` against `~/Downloads/mkvtoolnix/src/mkvtoolnix-gui/merge/tab.cpp`, so an empty result is a real absence. **GREEN, the end state: exactly the derived members and nothing else** - the `dirty` computed and its doc comment, and no assignment of the form `dirty.value =` or `<name>Dirty = ref(`. The end-state expression is therefore `grep -nE "dirty\.value *=|(isDirty|unsavedChanges|modified) *= *ref\(" src/views/EditorView.vue`, whose expected result is **0**, fired once against a synthetic line carrying `dirty.value = true` to prove it matches an assignment when one exists. **A second boolean introduced in a later fix round is what this catches**, which is the whole content of R28. **The two alternatives are not equally strong, and the asymmetry is stated rather than widened:** the first is structural and catches any reassignment of the derived value whatever it is called; the second is three plausible names, so a rival flag called something else escapes it. Not widened, because a broader name pattern would match ordinary prose and turn the check into triage; recorded so a later reader does not read D1 as exhaustive over rival mechanisms.
  - **Absence check P1 (amendment 1), no surface still carries a bare `!model` gate** - D112's "defined once", pinned structurally. `grep -nE 'v-if="!model' src/views/EditorView.vue`. **RED, the pre-state, measured on the tree as Task 3 left it: exactly 2 lines**, the `editor-empty` paragraph's gate and the `editor-recents` section's, which are the two Step 4b replaces. **GREEN, the end state: 0.** **That pre-state run IS the fire** - the same expression on the same file returning a non-zero result - so the zero afterwards cannot be a pattern that matches nothing anywhere. **An implementer whose pre-state recount disagrees with 2 returns NEEDS_CONTEXT with both runs pasted** rather than adjusting the fence. **What it does not cover, recorded so it is not read as more than it is:** it matches the exact spelling `v-if="!model`, so a gate written with a space after the bang, or one whose terms are ordered the other way (`v-if="currentPath === null && !model"`), escapes it. **Those are exactly the cases Step 4c's lint rule catches**, because that rule reads the parsed template rather than the file's characters, and it is the standing guard where P1 is the one-shot demonstration that the pre-state was red. P1 is kept beside it rather than replaced by it (`proc-proposed-safeguard-stays`): it is one line, its pre-state is already measured, and a grep and a lint rule fail for different reasons.
  - **The lint rule's green end state is not a separate run:** `pnpm lint` is inside the gate below, and Step 4c's rule is an error-severity rule in it, so the gate's own green run is the rule's green run. Paste Step 4c's red run and this green one side by side.
  - The full gate as `BUILDING.md` enumerates it, foreground, green. The `gui-editor.ftl` recount after this task must be 51; recompute it and correct the two comments (`src/views/EditorView.vue` was corrected by Task 3 and is corrected again here; `e2e/smoke.spec.ts`'s budget comment likewise). A disagreement is a finding -> NEEDS_CONTEXT. `git diff --stat` covers exactly the seven files in the Files list (six before amendment 1 added `eslint.config.js`).

- [ ] **Step 8: commit.**

```bash
git add src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/editor-rule-add-remove.spec.ts e2e/editor-undo-redo.spec.ts e2e/smoke.spec.ts eslint.config.js
git -c commit.gpgsign=false commit -m "editor: undo/redo over the one mutation funnel, and the unsaved state derived from its history" -- src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/editor-rule-add-remove.spec.ts e2e/editor-undo-redo.spec.ts e2e/smoke.spec.ts eslint.config.js
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the push rule's comparison form; the coalescing boundary and its three explicit resets; the depth constant; the keyboard condition set including the text-entry exemption and its enumerated input types; that the save state is derived and no second flag exists; that saving marks a position; that opening resets; the two fenced labels; that the mutation set is the six the expression returned and a seventh is a finding; **and, from amendment 1**: the pre-session condition's two terms and its fenced expression; that it is one named computed both gates read rather than two inline conditions; that it is NOT derived from `sessionActive` and adds no third state flag; that the path line, the unsaved line, the diagnostics section and the editing surface keep their own gates; that the failed-load state adds no catalog string; that Task 3's shipped steps are not rewritten; that the standing guard is one `vue/no-restricted-syntax` entry in the existing per-`.vue` rules block and adds no gate part, no dependency and no second config file; the selector and the message, both fenced; that it is scoped to `v-if`/`v-else-if` so the `:disabled` bindings stay legal; and that absence check P1 stays beside it rather than being replaced by it.

---

