# Task 5 report: the discard guards in the editor (W4a)

**Status: DONE.** Commit **`0b00262`**, six files, unsigned, one trailer. The full 11-part gate
is green on the committed state. See **section 9 (amendment 2 close-out)** for how this
resolved; sections 1-8 are the original NEEDS_CONTEXT submission and are kept standing rather
than rewritten, per the house pattern of not overwriting a superseded report (`proc-supersede-
never-overwrite`).

> **Original status at first submission: NEEDS_CONTEXT.** Not committed. Every step was built
> exactly as the brief then specified, and ten of the eleven gate parts were green
> (paste-verified below). The eleventh part (`pnpm test:e2e`) was red on exactly one test, and
> that test was not one Step 4b then authorized me to touch. The red test was a real,
> reproducible consequence of Step 2 (mandatory) colliding with Step 4b's own scope list (also
> mandatory, "no other case in that file is touched"), discovered on code contact. Reproduced,
> isolated, and written up as a decision memo in section 6 rather than resolved at the keyboard,
> per the dispatch's own rule for exactly this situation.

Everything else -- `ConfirmDialog.vue`, the two guarded call sites, both catalogs, the six new
smoke cases, the extended view-switch case, and the two authorized `editor-undo-redo.spec.ts`
repairs -- is built, passing, and ready to commit the moment the one open question is settled.

---

## 1. What was built

### Step 1: `src/components/ConfirmDialog.vue` (new)

A native `<dialog data-testid="confirm-dialog">` with `showModal()`, mirroring
`SettingsDialog.vue`'s imperative pattern. Props `title`/`message`/`confirmLabel`/`cancelLabel`
(all strings -- the caller resolves its own Fluent text). `defineExpose({ ask })` where
`ask(): Promise<boolean>`.

Resolution mechanics: the confirm button (`confirm-dialog-confirm`) settles `true` and then
calls `.close()`. The cancel button (`confirm-dialog-cancel`) only calls `.close()`; the
native `close` event is the single place `false` is actually resolved, which covers the cancel
button, a programmatic close, and Esc uniformly (Esc's default action is to close the dialog,
which fires the same `close` event) -- one path instead of one per trigger, and idempotent by
construction (`settleAsk` is nulled the moment it is used, so a `close` event arriving after
`onConfirm` already settled `true` is a no-op).

### Step 2: the two guarded call sites, `src/views/EditorView.vue`

`EditorView` mounts one `ConfirmDialog` in its template with the three discard strings and
`settings-cancel` as the cancel label:

```html
<ConfirmDialog
  ref="confirmDialog"
  :title="$t('editor-discard-title')"
  :message="$t('editor-discard-message')"
  :confirm-label="$t('editor-discard-confirm')"
  :cancel-label="$t('settings-cancel')"
/>
```

`pickAndOpen`, after its existing busy guard:

```ts
if (dirty.value && !(await confirmDialog.value?.ask())) {
  return;
}
```

placed before the `openDialog(...)` call -- confirm, then file dialog, then replace, exactly
the ordering D109 decision 1 and the ROADMAP's second owner ruling both require.

`createBlank` becomes `async`, gains the identical guard ahead of the seed, and its click
binding (`@click="createBlank"`) is unchanged -- the same "async function, no wrapper" shape
`pickAndOpen`/`doSave` already use, so "the click handler awaits it" is the framework's own
existing handling of a Promise-returning handler, not new template syntax.

`openPath` gained no guard, per the brief.

Two comments elsewhere in the same file that this change made false were corrected as a direct
consequence of the edit (not a separate sweep): the `dirty` computed's own doc comment said
"nothing in this file reads it yet", which stopped being true the moment `pickAndOpen`/
`createBlank` started consuming it; and `createBlank`'s header comment said the function "is
synchronous", which stopped being true once it became `async`. Both are one-paragraph
corrections, not restatements of the code below them.

### Step 3: catalogs

`grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl` / same for `de`:

```
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl; echo "exit:$?"
54
exit:0
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/de/gui-editor.ftl; echo "exit:$?"
54
exit:0
```

Both catalogs carry the three fenced strings byte-identical to the brief, appended after the
"Profile creation and the pre-session state" section as directed. `e2e/smoke.spec.ts`'s budget
comment now reads:

```
// budget is 54 (42 labels + 1 save-surface note + 4 generic action keys +
// 1 rule-grid ordinal + 3 profile-creation keys + 3 discard-confirmation
// keys, D109).
```

### Step 4 + 4b: the tests

Six new cases in a new `test.describe("editor view: discard guards (Task 5, D109)")` block at
the end of `e2e/smoke.spec.ts` (Cases 1-5), plus the extension of the existing view-switch case
in place (Case 6, `editor view: open/save (Task 13, D45/D41)` describe -- not duplicated).
Case 6 gained an Undo-enabled assertion and a `confirm-dialog` hidden assertion after the round
trip, on top of the edited-value assertion that already existed there.

`e2e/editor-undo-redo.spec.ts`: the two cases the brief names ("open resets", "a failed open
...") each gained exactly two lines between the second Open click and their pre-existing
assertions -- assert `confirm-dialog` visible, click `confirm-dialog-confirm`. No existing
assertion in either case was removed, weakened, or reordered (diffs in section 4).

---

## 2. Verification: 10 of 11 gate parts, pasted

Commands and outputs below are copy-pasted from the runs that produced them, in the order
`BUILDING.md` enumerates.

### Frontend part 1/4: `pnpm lint`

```
$ pnpm lint
$ eslint .
```
Exit 0, no output (clean).

### Frontend part 2/4: `pnpm build`

```
$ pnpm build
$ vue-tsc --noEmit && vite build
vite v8.1.4 building client environment for production...
transforming...✓ 167 modules transformed.
rendering chunks...
computing gzip size...
dist/index.html                   0.39 kB │ gzip:   0.26 kB
dist/assets/index-DGn2eD1R.css    1.31 kB │ gzip:   0.49 kB
dist/assets/index-CEyJLAxY.js   331.15 kB │ gzip: 107.53 kB

✓ built in 154ms
```
Exit 0.

### Frontend part 3/4: `pnpm check:i18n`

```
$ pnpm check:i18n
$ node scripts/check-i18n.mjs
check-i18n: ok (42 source files scanned, 221 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
```
Exit 0.

### Frontend part 4/4: `pnpm test:e2e` -- RED, one test, isolated below

```
$ pnpm test:e2e
...
  ✘   32 [chromium] › e2e/editor-undo-redo.spec.ts:422:3 › editor undo/redo: granularity, truncation, save/open, the depth cap (Task 4, D108) › createBlank resets: New after edited history clears both Undo and Redo (5.3s)
...
  1) [chromium] › e2e/editor-undo-redo.spec.ts:422:3 › ... › createBlank resets: New after edited history clears both Undo and Redo

    Error: expect(locator).toBeDisabled() failed

    Locator:  getByTestId('view-editor').getByTestId('editor-undo')
    Expected: disabled
    Received: enabled
    Timeout:  5000ms

      428 |
      429 |     await editor.getByTestId("editor-new").click();
    > 430 |     await expect(editor.getByTestId("editor-undo")).toBeDisabled();
          |                                                     ^
      431 |     await expect(editor.getByTestId("editor-redo")).toBeDisabled();
      432 |   });

  1 failed
    [chromium] › e2e/editor-undo-redo.spec.ts:422:3 › ... › createBlank resets: New after edited history clears both Undo and Redo
  99 passed (8.1s)
```

99 passed, 1 failed. Every discard-guard case this task added (Cases 1-6, both `editor-undo-
redo.spec.ts` repairs) is in the 99. Section 6 below is this one failure's full analysis.

### Rust part 1/6: `cargo fmt --all --check`

```
$ cargo fmt --all --check
```
Exit 0, no output.

### Rust part 2/6: `cargo clippy --workspace --all-targets -- -D warnings`

```
$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
```
Exit 0.

### Rust part 3/6: `cargo test --workspace`

```
$ cargo test --workspace
...
test result: ok. 80 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
...
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Every suite `ok`, 0 failed throughout. Exit 0. (This task touched no Rust source; included
per Step 5's "full gate" instruction.)

### Rust part 4/6: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items`

```
$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files
```
Exit 0.

### Rust part 5/6: `cargo deny check`

```
$ cargo deny check
...
advisories ok, bans ok, licenses ok, sources ok
```
Exit 0 (the duplicate-dependency tree it prints is pre-existing `toml`/`winnow` fan-out from
the Tauri dependency graph, not new).

### Rust part 6/6: `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`

```
$ cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
warning: muxsmith-gui@0.1.0: GNU compiler is not supported for this target
warning: muxsmith-gui@0.1.0: GNU compiler is not supported for this target
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
```
Exit 0 (the two lines are a pre-existing build-script advisory printed on every cross-target
run on this machine's Linux/GNU host, not a clippy warning).

### House-knowledge part: `python3 scripts/ledger-lint.py`

```
$ python3 scripts/ledger-lint.py
ledger-lint: 575 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```
Exit 0.

**Summary: 10/11 green, 1/11 (`pnpm test:e2e`) red on exactly one pre-existing test the brief
did not authorize this task to touch.**

---

## 3. `git diff --stat`: exactly the six files in the brief's list

```
$ git status --porcelain=v1
 M e2e/editor-undo-redo.spec.ts
 M e2e/smoke.spec.ts
 M locales/de/gui-editor.ftl
 M locales/en/gui-editor.ftl
 M src/views/EditorView.vue
?? src/components/ConfirmDialog.vue

$ git diff --stat
 e2e/editor-undo-redo.spec.ts |  13 ++
 e2e/smoke.spec.ts            | 303 ++++++++++++++++++++++++++++++++++++++++++-
 locales/de/gui-editor.ftl    |   6 +
 locales/en/gui-editor.ftl    |   6 +
 src/views/EditorView.vue     |  63 ++++++---
 5 files changed, 370 insertions(+), 21 deletions(-)
```

`git diff --stat` shows only tracked files (5); the new file (`ConfirmDialog.vue`, untracked)
is the sixth. 5 + 1 = 6, matching the brief's exhaustive Files list exactly -- no stray file
touched.

---

## 4. The leg-(ii) mutation attack (Case 3(ii), W3-q2's producer)

Case 3(ii) (`e2e/smoke.spec.ts`, "after a successful save the guard clears") is the first and
only producer anywhere in this plan for the property that saving marks the profile that was
written. Attacked as instructed: broke the mark, rebuilt, watched it go red in isolation;
restored, rebuilt, confirmed the restore by content (matching build hash), watched it go green
again.

### The mutation

```diff
-    savedSnapshot.value = JSON.stringify(profile);
+    // MUTATION (task-5 leg-(ii) attack, reverted immediately after the run
+    // it produces): savedSnapshot.value = JSON.stringify(profile);
     currentPath.value = path;
```

in `doSave`, `src/views/EditorView.vue` -- the exact line D108 decision 3 names, and the exact
line Task 4's own reviewer deleted with the whole suite staying green (per this dispatch's
brief).

### Rebuild (the environment trap: `test:e2e` serves `dist/`, which only `pnpm build` refreshes)

```
$ pnpm build
$ vue-tsc --noEmit && vite build
vite v8.1.4 building client environment for production...
transforming...✓ 167 modules transformed.
dist/assets/index-ZAInoytH.js   331.13 kB │ gzip: 107.52 kB
✓ built in 154ms
```

Asset hash `index-ZAInoytH.js` -- different from the pre-mutation `index-CEyJLAxY.js` above,
confirming the bundle actually changed.

### The run, mutated: only Case 3(ii) fails

```
$ pnpm exec playwright test e2e/smoke.spec.ts -g "discard guards" --reporter=list
Running 6 tests using 6 workers

  ✓  6 ... Case 5: the recents affordance is unreachable while a profile is held (D112's !model term) (206ms)
  ✓  5 ... Case 3(i): Open with no unsaved changes reaches the file dialog directly, no confirm (240ms)
  ✓  2 ... Case 2: Open over unsaved changes, cancelled -- the file dialog never opens and the edit survives (268ms)
  ✓  4 ... Case 4: New over unsaved changes -- confirmed replaces the edited profile, cancelled does not (360ms)
  ✓  1 ... Case 1: Open over unsaved changes, confirmed -- the file dialog waits for the confirm, then the second profile replaces the editor (491ms)
  ✘  3 ... Case 3(ii): after a successful save the guard clears -- the two Open clicks in this test are each other's control (5.3s)

  1) [chromium] › e2e/smoke.spec.ts:2164:3 › ... Case 3(ii) ...

    Error: expect(locator).toBeHidden() failed

    Locator:  getByTestId('view-editor').getByTestId('confirm-dialog')
    Expected: hidden
    Received: visible
    Timeout:  5000ms

      2201 |     // reaches the file dialog directly.
      2202 |     await editor.getByTestId("editor-open").click();
    > 2203 |     await expect(editor.getByTestId("confirm-dialog")).toBeHidden();
           |                                                        ^

  1 failed
  5 passed (5.8s)
```

Exactly the predicted failure: with the mark broken, the save never clears `dirty`, and the
SECOND Open click (which should reach the file dialog directly) shows a confirm instead --
data loss in the safe direction becomes a spurious warning here, but a marking bug that goes
the OTHER way (marking too early, e.g. at `model.value` instead of the captured `profile`)
would instead make a guard silently NOT fire over real unsaved changes, the actual data-loss
direction this whole family exists to prevent. Every other case (1, 2, 3(i), 4, 5) stayed
green, confirming leg (ii) is the ONLY place in the suite that observes this property.

### Revert and rebuild, verified by content

```diff
-    // MUTATION (task-5 leg-(ii) attack, reverted immediately after the run
-    // it produces): savedSnapshot.value = JSON.stringify(profile);
+    savedSnapshot.value = JSON.stringify(profile);
     currentPath.value = path;
```

```
$ pnpm build
$ vue-tsc --noEmit && vite build
dist/assets/index-CEyJLAxY.js   331.15 kB │ gzip: 107.53 kB
✓ built in 155ms
```

Asset hash `index-CEyJLAxY.js` is byte-identical to the ORIGINAL pre-mutation build (section 2
above) -- the source is verified back to its exact original state by content, not by the
edit's exit code (per the sandbox's `cp`/`rm` aliasing trap this environment is known to carry
elsewhere; here it is the `Edit` tool, and the check is still content, not "the edit
succeeded").

```
$ pnpm exec playwright test e2e/smoke.spec.ts -g "discard guards" --reporter=list
Running 6 tests using 6 workers
  ✓  3 ... Case 5 ...
  ✓  2 ... Case 3(i) ...
  ✓  5 ... Case 2 ...
  ✓  4 ... Case 3(ii): after a successful save the guard clears -- the two Open clicks in this test are each other's control (345ms)
  ✓  6 ... Case 4 ...
  ✓  1 ... Case 1 ...

  6 passed (946ms)
```

Green again, all six.

---

## 5. `grep`/exit-status hygiene (house traps)

- Every grep pasted above (`grep -cE ...`) is a direct `grep` invocation, not piped through
  anything that would hide a nonzero exit or scope out ignored paths; both catalog files are
  tracked, not `.gitignore`d, so the shell-function-vs-`command grep` distinction does not
  apply to this task's own searches.
- No command's exit status in this report was read through a pipeline; every `echo "exit:$?"`
  or bare command shown above ran as the last element of its own invocation.

---

## 6. The one open question: NEEDS_CONTEXT

### What was observed

Step 4b's own criterion for the affected set in `e2e/editor-undo-redo.spec.ts` is "builds
history, then opens again" -- i.e. a SECOND `editor-open` click while `dirty` is true -- and it
names exactly two matches ("open resets", "a failed open ..."), with the Files list and the
Must-not-decide list both stating "no other case in that file is touched."

But Step 2 (mandatory, this same brief) puts the identical guard on `createBlank`, not only on
`pickAndOpen`. `createBlank` is reached by clicking `editor-new`, not `editor-open` -- so
Step 4b's criterion, phrased around a second OPEN, structurally cannot see a case that replaces
via NEW instead. One exists, in the same file, in the same describe block, sitting between the
two acknowledged cases:

```
test("createBlank resets: New after edited history clears both Undo and Redo", async ({ page }) => {
  const { editor } = await openProfile(page, "/profiles/create-blank-resets.yaml", emptyRulesProfile);

  const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
  await pattern.fill("edited");
  await expect(editor.getByTestId("editor-undo")).toBeEnabled();   // dirty IS true here

  await editor.getByTestId("editor-new").click();                  // now guarded (Step 2)
  await expect(editor.getByTestId("editor-undo")).toBeDisabled();  // never reached: guard is waiting
  await expect(editor.getByTestId("editor-redo")).toBeDisabled();
});
```

I verified this is the complete set (not a guess) by grepping every `editor-new`/`editor-open`
click site in the file and checking each against the actual dirty state at that point:

```
$ grep -n "editor-new\|editor-open" e2e/editor-undo-redo.spec.ts
99:  await editor.getByTestId("editor-open").click();     -- shared helper's FIRST open, dirty always false
390: await editor.getByTestId("editor-open").click();     -- "open resets" first open, dirty false
400: await editor.getByTestId("editor-open").click();     -- "open resets" SECOND open, dirty true (REPAIRED)
429: await editor.getByTestId("editor-new").click();       -- "createBlank resets", dirty true (NOT in brief, NOT repaired)
472: await editor.getByTestId("editor-open").click();      -- "a failed open" first open, dirty false
486: await editor.getByTestId("editor-open").click();      -- "a failed open" SECOND open, dirty true (REPAIRED)
630: await editor.getByTestId("editor-open").click();      -- D112 leg 2, model never edited, dirty false
638: await editor.getByTestId("editor-open").click();      -- D112 leg 3, model never edited, dirty false
```

Three sites hit a truly dirty editor, not two; the third is `editor-new`, which Step 4b's
Open-scoped criterion does not enumerate by construction.

### Why this is a contradiction, not a judgment call

Two written statements in the same brief collide against the actual tree: Step 2 ("createBlank
gains the same guard... nothing else about it changes") and Step 4b + the Must-not-decide list
("the affected set is exactly two... no other case in that file is touched... which two"). This
is exactly the first of Peter's own standing escalation criteria (two fixed statements
colliding), which is why I did not resolve it myself: extending the repair to a third case
would violate the letter of "exactly two... no other case", and NOT extending it leaves the
gate red, which Step 5 requires to be green before Step 6's commit.

### Options

**A. Extend the affected set to three; repair "createBlank resets" identically** (assert
`confirm-dialog` visible after the New click, click `confirm-dialog-confirm`, then keep every
existing assertion unchanged). Same repair shape Step 4b already prescribes for the other two,
by the same criterion, corrected to cover New as well as Open. Cost: one more three-line edit
in the same file, and a literal breach of "exactly two... no other case is touched" -- but the
premise that sentence rests on (only Open reaches a dirty guard) is what turned out to be
incomplete.

**B. Leave the file exactly as Task 4 shipped it.** Zero deviation from the brief's literal
scope. Cost: the gate stays red on this one test permanently, since nothing else in this or a
later task is positioned to fix it -- `pnpm test:e2e` cannot be pushed green.

**C. Narrow `createBlank`'s guard so it does not fire in this specific scenario.** Not a real
option: it reopens Step 2 / D109 decision 2 and the Must-not-decide list's protection of "the
guard sits in pickAndOpen and createBlank... and not in openPath" -- forbidden regardless of
this finding.

### Recommendation

**A.** It is mechanical (the identical two-line insertion already authorized twice in this same
file), changes no assertion's meaning, and the only thing being corrected is the enumeration's
completeness -- not the guard's shape, its ordering, or which functions it guards. I left it
unapplied pending this ruling rather than applying it and reporting it after the fact, per the
dispatch's explicit instruction for this exact situation.

---

## 7. Two items considered and closed by analysis (not escalated)

**Task 3 reviewer's forward note (concurrent entry into async `createBlank`).** Making
`createBlank` async does, in the abstract, open a second-call window the existing
`opening`/`saving` busy guard does not cover (neither flag is set while `createBlank` awaits
the confirm). I checked whether this materializes here: `ask()`'s executor calls
`dialogEl.value?.showModal()` synchronously, inside the same microtask that first invoked
`createBlank` -- `dirty.value && !(await ...)` short-circuits entirely (no `await` reached at
all) whenever `dirty` is false, and whenever `dirty` is true the modal is already open, by
native `<dialog>`/`showModal()` semantics inert to pointer and keyboard input outside itself,
before `createBlank` yields control back to the event loop. So no second click on New or Open
can land between the guard's entry and the modal actually blocking the page. This closes the
gap for the concrete shape this component ships in (native modal, `showModal()`, no non-modal
`show()` path anywhere) without adding any mechanism -- consistent with "if it matters, return
it as NEEDS_CONTEXT, never build a mechanism unasked": it doesn't matter for this build, so
nothing was added.

**Stale catalog-count comment in `EditorView.vue` (out of this task's Files-list scope).** The
header doc block still reads "`gui-editor.ftl` carries 49 ids today" (line ~67) -- correct at
Task 3, never bumped to 51 by Task 4, and now also stale against this task's 54. Files list
scopes my `EditorView.vue` edits to "the dialog mount, the two guarded call sites"; this
comment is neither. Left untouched, flagged here for a future sweep, matching the house
precedent recorded in `task-15-verdict.md` for the identical class of drift in
`PropertyMapWidget.vue`.

---

## 8. Commit

Not run. `git add`/`git commit` per the brief's exact block is staged and ready the moment
section 6 is ruled; nothing else in the diff needs to change either way.

---

## 9. Amendment 2 close-out

The coordinator's ruling: the collision in section 6 was real, its root was a plan-authoring
gap (a case ruled into Task 4's fix round after amendment 1 drew the two-case enumeration,
without revisiting that enumeration), and it was closed by amending the plan itself -- amendment
2, independently reviewed over two fix rounds, approved with zero open findings -- rather than
by a controller override of the fenced text I was dispatched with.

**Regenerated brief.** Per the coordinator, `task-5-brief.md` was regenerated against the
amended plan (127 lines, up from the 73 I was dispatched with) and read fresh rather than reused.
Cross-checked against `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`'s own
Task 5 section: identical content. Working tree re-derived rather than trusted on the
coordinator's word: `git status --porcelain=v1` / `git diff --stat` after four new commits had
landed on `master` showed my six paths exactly as I had left them, nothing else touched.

**What changed and what I did about it, against the regenerated brief:**

1. **Step 4b's membership criterion**, corrected from "a second Open click" to "activates a
   control bound to `pickAndOpen` or `createBlank` while `dirty` is true" -- derived from the two
   functions Step 2 actually guards, not from one of the two ways into them. Re-derived against
   the file as it stands (**not** taken from the brief's own enumeration) by grepping every
   `editor-new`/`editor-open` click site and checking the dirty state at each (pasted in
   section 6): three members, matching the brief's own re-derivation exactly --
   `"open resets"` and `"a failed open ..."` (already repaired in the first submission) plus
   `"createBlank resets"` (the case section 6 flagged).

2. **`e2e/editor-undo-redo.spec.ts`: `"createBlank resets"` repaired identically** to the other
   two -- `confirm-dialog` visible assertion, then `confirm-dialog-confirm` click, inserted
   between the `editor-new` click and the pre-existing assertions. No existing assertion
   removed, weakened, reordered or reworded. No additional mock needed: the shared `openProfile`
   helper already mocks both `plugin:dialog|open` and `validate_profile_model`, and the brief's
   own Step 4b confirms the New path only needs the latter, already present.

```diff
     const pattern = editor.getByRole("textbox", name("editor-input-pattern"));
     await pattern.fill("edited");
     await expect(editor.getByTestId("editor-undo")).toBeEnabled();

+    // Task 5 (D109) repair, amendment 2: New activates the same guarded
+    // control as Open (`createBlank`, Step 2), and the editor is dirty (the
+    // fill above) -- confirmed here so the assertions below still test the
+    // RESET this case is named for, not the guard itself.
     await editor.getByTestId("editor-new").click();
+    await expect(editor.getByTestId("confirm-dialog")).toBeVisible();
+    await editor.getByTestId("confirm-dialog-confirm").click();
     await expect(editor.getByTestId("editor-undo")).toBeDisabled();
     await expect(editor.getByTestId("editor-redo")).toBeDisabled();
   });
```

3. **Step 4c (new): both catalog-budget comments, one owner.** `e2e/smoke.spec.ts`'s comment
   (section 1/3 above) already matched half 1's fenced end-state byte-for-byte -- confirmed by
   grep, no edit needed. `src/views/EditorView.vue`'s header sentence (half 2, the item section 7
   flagged as correctly out of THIS task's original scope) is now explicitly this task's, via the
   exact fenced replacement:

```diff
-// packages did add to it: `gui-editor.ftl` carries 49 ids today, three of
-// them this view's own New affordance (`editor-action-new`,
-// `editor-empty`, `editor-unsaved`, D107). The Open button, the
+// packages did add to it: `gui-editor.ftl` carries 54 ids today, eight
+// of them this view's own affordances: profile creation
+// (`editor-action-new`, `editor-empty`, `editor-unsaved`, D107),
+// undo/redo (`editor-action-undo`, `editor-action-redo`, D108) and
+// the discard confirmation (`editor-discard-title`,
+// `editor-discard-message`, `editor-discard-confirm`,
+// D109). The Open button, the
```

   Recount, re-run rather than trusted from section 3 above:

```
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl; echo "exit:$?"
54
exit:0
```

### Full 11-part gate, re-run after a rebuild (the environment trap this project is known for)

`pnpm build` was run immediately before `pnpm test:e2e`, per the standing rule that
`playwright.config.ts` serves `dist/` and runs no build of its own.

```
$ pnpm lint
$ eslint .
(exit 0, no output)

$ pnpm build
$ vue-tsc --noEmit && vite build
dist/assets/index-CEyJLAxY.js   331.15 kB │ gzip: 107.53 kB
✓ built in 153ms

$ pnpm check:i18n
$ node scripts/check-i18n.mjs
check-i18n: ok (42 source files scanned, 221 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).

$ pnpm test:e2e
...
  100 passed (8.2s)

$ pnpm exec playwright test e2e/editor-undo-redo.spec.ts -g "createBlank resets" --reporter=list
  ✓  1 [chromium] › ... › createBlank resets: New after edited history clears both Undo and Redo (249ms)
  1 passed (599ms)

$ cargo fmt --all --check
(exit 0, no output)

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s

$ cargo test --workspace 2>&1 | grep -E "FAILED|test result"
(39 suites, all "ok", 0 failed each -- 575 tests total across the workspace)

$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok

$ cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
warning: muxsmith-gui@0.1.0: GNU compiler is not supported for this target (x2, pre-existing)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s

$ python3 scripts/ledger-lint.py
ledger-lint: 579 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

**11/11 green.** (The house-knowledge entry count rose 575 -> 579 from the four amendment-2
commits that landed on `master` around this working tree, not from anything this task touched.)

### `git diff --stat`: exactly the six files

```
$ git status --porcelain=v1
 M e2e/editor-undo-redo.spec.ts
 M e2e/smoke.spec.ts
 M locales/de/gui-editor.ftl
 M locales/en/gui-editor.ftl
 M src/views/EditorView.vue
?? src/components/ConfirmDialog.vue

$ git diff --stat
 e2e/editor-undo-redo.spec.ts |  19 +++
 e2e/smoke.spec.ts            | 303 ++++++++++++++++++++++++++++++++++++++++++-
 locales/de/gui-editor.ftl    |   6 +
 locales/en/gui-editor.ftl    |   6 +
 src/views/EditorView.vue     |  73 ++++++++---
 5 files changed, 383 insertions(+), 24 deletions(-)
```

### Commit

```
$ git add src/components/ConfirmDialog.vue src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts e2e/editor-undo-redo.spec.ts
$ git -c commit.gpgsign=false commit -m "editor: confirm before unsaved changes are replaced, ahead of the file dialog

Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>" -- src/components/ConfirmDialog.vue src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts e2e/editor-undo-redo.spec.ts
[master 0b00262] editor: confirm before unsaved changes are replaced, ahead of the file dialog
 6 files changed, 479 insertions(+), 24 deletions(-)
 create mode 100644 src/components/ConfirmDialog.vue

$ git status --porcelain=v1
(clean)
```

Not pushed, per standing instruction.

### The other three concerns from section 7, as the coordinator resolved them

- **Concurrent entry into async `createBlank`:** accepted as reasoning, no mechanism added --
  unchanged from section 7.
- **Stale `EditorView.vue` header comment:** was correctly out of scope at first submission
  (Files list then didn't cover it); now IS this task's, via the new Step 4c, and is corrected
  above.
- **The `watch(model)`/`validationGeneration` race:** stays routed to a later plan, untouched,
  as it was from the start.

---

## 10. Fix round 1 (`task-5-verdict.md`)

**Status: DONE.** Commit **`57bcc41`**, on top of `0b00262` (and the intervening house-knowledge
commit `5ba09e3`, ledger-only, touched none of this task's files). Two files, unsigned, one
trailer.

Spec compliance came back MET in full; quality came back 0 Critical, 2 Moderate, 1 Low. Two
items were mine (Findings A and B); the third (Finding C) is the coordinator's, deferred with a
named trigger rather than fixed here.

### Item 1 (Finding A, Moderate): Esc-cancel coverage

Step 1's own contract resolves `ask()` false "by `close` and by Esc"; nothing in the six smoke
cases or the three `editor-undo-redo.spec.ts` repairs ever pressed Escape. Added one case to
`e2e/smoke.spec.ts`'s `"editor view: discard guards (Task 5, D109)"` describe -- the file this
task already owns that carries the rest of `ConfirmDialog`'s behavioral coverage (Cases 1-6),
placed directly after Case 2 since it is the same scenario (Open over unsaved changes,
cancelled) through a different channel (Esc instead of the `confirm-dialog-cancel` button). No
new test infrastructure: `page.keyboard.press("Escape")` is the existing pattern
`e2e/help-mode.spec.ts:200` already uses against a different modal `<dialog>` in this repo.

**Discriminating.** Breaking Esc specifically (not the buttons) means intervening on the native
`cancel` event, which only Escape dispatches -- the cancel button never goes through it, calling
`.close()` directly. Mutation: added `@cancel.prevent` to the `<dialog>` in `ConfirmDialog.vue`,
neutralizing Escape's default close action while leaving both buttons' explicit `.close()` calls
untouched.

```
$ pnpm build   # after adding @cancel.prevent
dist/assets/index-DzduOr0v.js   331.19 kB │ gzip: 107.53 kB   # hash changed, mutation live

$ pnpm exec playwright test e2e/smoke.spec.ts -g "discard guards" --reporter=list
Running 7 tests using 7 workers
  ✓ Case 5 ...
  ✓ Case 3(i) ...
  ✓ Case 2 ...
  ✓ Case 4 ...
  ✓ Case 3(ii) ...
  ✓ Case 1 ...
  ✘ Esc cancels the guard the same way the cancel button does -- Step 1's own contract

    Error: expect(locator).toBeHidden() failed
    Locator:  getByTestId('view-editor').getByTestId('confirm-dialog')
    Expected: hidden
    Received: visible
      2152 |     await expect(editor.getByTestId("confirm-dialog")).toBeVisible();
      2153 |     await page.keyboard.press("Escape");
    > 2154 |     await expect(editor.getByTestId("confirm-dialog")).toBeHidden();

  1 failed
  6 passed (5.7s)
```

Exactly the new case failed; all six siblings green. Reverted, rebuilt, content-verified:

```
$ git diff -- src/components/ConfirmDialog.vue | grep -c "cancel.prevent"
0
$ pnpm build   # after removing @cancel.prevent
dist/assets/index-CEyJLAxY.js   331.15 kB │ gzip: 107.53 kB   # hash matches the pre-mutation build exactly

$ pnpm exec playwright test e2e/smoke.spec.ts -g "discard guards" --reporter=list
Running 7 tests using 7 workers
  ✓ Case 5 ...
  ✓ Case 3(i) ...
  ✓ Esc cancels the guard the same way the cancel button does -- Step 1's own contract (257ms)
  ✓ Case 2 ...
  ✓ Case 3(ii) ...
  ✓ Case 4 ...
  ✓ Case 1 ...

  7 passed (941ms)
```

### Item 2 (Finding B, Low): the false citation

`ConfirmDialog.vue`'s `onClose` comment claimed Esc followed "the same Esc-consumed-by-the-
native-cancel semantics `SettingsDialog.vue` documents for its own dialog." The reviewer grepped
`SettingsDialog.vue` and its full history for Esc/cancel-event/dismiss semantics and found
nothing; I reproduced that independently:

```
$ grep -n "Esc\|cancel" src/components/SettingsDialog.vue
185:        :title="$ta('settings-cancel').tooltip"
188:        {{ $t("settings-cancel") }}
```

(both hits are the unrelated `settings-cancel` Fluent key, not Esc semantics)

```
$ git log --all -p -- src/components/SettingsDialog.vue | grep -ni "esc\b\|escape\|cancel.event\|native cancel"
114:    gui: help mode - sidebar, toggle, delegated hover/pin/Esc, ruled activation suppression (D52, E3)
```

(a commit-message hit for an unrelated task, not a documentation line in the file itself)

**Where the false premise came from, per the coordinator's instruction to note it before
rewriting:** the task-5-brief.md Read-first line itself named `SettingsDialog.vue` as carrying
"its Esc note", and Step 1 repeated "the same Esc semantics the settings dialog documents" --
both inherited into the shipped comment rather than invented at the keyboard. Those planning
documents are not shipped code and are not this fix round's target; only the comment in
`ConfirmDialog.vue` is corrected here.

**What actually documents this semantic** -- found by searching for the exact fact rather than
assuming no source exists:

```
$ grep -n "Esc closes it, which reads as cancel" docs/superpowers/specs/2026-07-30-plan-12-decisions.md
100:6. **The confirm surface for the two in-app guards is an in-app `<dialog>`**, ... Esc closes
it, which reads as cancel: the safe direction. The component's props are the minimum a second
caller needs.
```

D109 decision 6 states the exact fact `ConfirmDialog` implements. The comment now cites that
instead of `SettingsDialog.vue`:

```diff
-// way and fires this same `close` event, exactly the Esc-consumed-by-the-
-// native-cancel semantics `SettingsDialog.vue` documents for its own
-// dialog. A no-op once `onConfirm` already settled `true` and nulled
-// `settleAsk`.
+// way and fires this same `close` event. Esc reading as cancel is the
+// safe "do not discard" direction, per D109 decision 6
+// (docs/superpowers/specs/2026-07-30-plan-12-decisions.md): "Esc closes
+// it, which reads as cancel: the safe direction." A no-op once
+// `onConfirm` already settled `true` and nulled `settleAsk`.
```

A pure comment edit -- confirmed by the build hash staying `index-CEyJLAxY.js` before and after
(Vite strips comments in production output), and by `pnpm lint`/the full `pnpm test:e2e` run
below staying green.

### Item 3 (Finding C, Moderate): NOT touched -- restated as the coordinator's deferred residual

**No guard was added to `ConfirmDialog.ask()`.** This is the coordinator's own decision, held
back explicitly from this round as product-behaviour latitude this plan does not prescribe. The
finding itself is restated below at full strength, not softened:

`ConfirmDialog.ask()` has no internal reentrancy guard. A second call to `ask()` while a first
is still pending overwrites the single `settleAsk` closure variable unconditionally, orphaning
the first caller's promise forever and answering the second caller's confirm-click instead. This
is unreachable through any real user interaction in the shipped app today -- a hit-tested
mouse/keyboard input cannot land on a control covered by an open native modal `<dialog>`, and no
keyboard shortcut in this view bypasses that (`onEditorKeydown` only handles Ctrl+Z/Ctrl+Y) --
but the gap is real at the level of the function's own state machine, reachable by any
script-level (non-hit-tested) second call, and reproduced by the reviewer against the actual,
unmutated shipped bundle.

**Deferred, not dropped, with its trigger named:** the observable event that makes this gap
reachable is the arrival of `ConfirmDialog`'s own anticipated second caller (its doc comment:
"and any second caller this component was built for"). It goes on the plan's tracker at close,
in those terms, per the coordinator's instruction -- not fixed in this task, and not weakened
into "unreachable, so it's fine" in this report.

The house entry this finding produced, carried into future concurrency review rather than
restated here: **a concurrency answer names the function's own defenses, not merely whether its
current callers happen to be safe.**

### Verification run after both edits

Both changed files (`e2e/smoke.spec.ts`, `src/components/ConfirmDialog.vue`) stay inside "the
spec files and the component," so the full 11-part gate was not required; `pnpm lint`,
`pnpm build`, and the full `pnpm test:e2e` were run instead, in that order, with a build
immediately before the test run:

```
$ pnpm lint
$ eslint .
(exit 0, no output)

$ pnpm build
dist/assets/index-CEyJLAxY.js   331.15 kB │ gzip: 107.53 kB
✓ built in 153ms

$ pnpm test:e2e
...
  101 passed (8.1s)
```

101 passed (100 from the prior round plus the new Esc case), 0 failed.

### `git diff --stat` / commit

```
$ git status --porcelain=v1
 M e2e/smoke.spec.ts
 M src/components/ConfirmDialog.vue

$ git diff --stat
 e2e/smoke.spec.ts                | 40 ++++++++++++++++++++++++++++++++++++++++
 src/components/ConfirmDialog.vue |  9 +++++----
 2 files changed, 45 insertions(+), 4 deletions(-)

$ git add e2e/smoke.spec.ts src/components/ConfirmDialog.vue
$ git -c commit.gpgsign=false commit -m "editor: cover Esc-cancel on the discard guard, correct a false Esc-semantics citation

Co-Authored-By: Claude Sonnet 5 <noreply@anthropic.com>" -- e2e/smoke.spec.ts src/components/ConfirmDialog.vue
[master 57bcc41] editor: cover Esc-cancel on the discard guard, correct a false Esc-semantics citation
 2 files changed, 45 insertions(+), 4 deletions(-)

$ git status --porcelain=v1
(clean)
```

Not pushed, per standing instruction.
