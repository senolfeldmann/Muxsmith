# Task 4 report: undo/redo over the mutation funnel, and the save state derived from it (W3)

Status: DONE.

## What shipped

`src/views/EditorView.vue`:
- History state (`history`, `position`, `savedSnapshot`, `coalesce`, `HISTORY_DEPTH = 100`), `sessionActive` turned into `computed(() => savedSnapshot.value !== null)`, the D112 `nothingOpenedOrCreated` computed (inserted verbatim from the brief's fence), `dirty`, `canUndo`, `canRedo`, and `resetHistory(profile)`.
- The push rule inside the existing `watch(model)`, before the validation round trip, exactly as fenced in the brief, with `HISTORY_DEPTH` gating the cap.
- `openPath` and `createBlank` now call `resetHistory(...)` in the position their old `sessionActive.value = true` occupied (before the model write); the load-bearing-order comment above `createBlank` is corrected to name `resetHistory` instead of the assignment it replaces, with the measured figures (79/79/3-failed) carried over unchanged.
- `doSave`'s post-write line: `savedSnapshot.value = JSON.stringify(profile);`, added immediately after `await saveProfile(path, profile);` resolves, inside the existing `try`.
- The coalescing boundary: `coalesce = false;` as the first statement of `onDrop`, `addRule`, `removeSelectedRule`; `@focusout="coalesce = false"` and `@keydown="onEditorKeydown"` on the editor's root `<section>`.
- `undo()`/`redo()`, gated on `canUndo`/`canRedo` and `model.value`, clearing `selectedIndex` and `coalesce`; `onEditorKeydown` with the exact condition set from the brief (text-entry exemption, `Ctrl/Cmd+Z`, `Ctrl/Cmd+Shift+Z`/`Ctrl/Cmd+Y`, `preventDefault` only on a handled combination).
- Two buttons (`editor-undo`/`editor-redo`) in the action row after New/Open, `:disabled="!model || !canUndo"` / `!canRedo"`, no `title`.
- The two D112 template gates (`editor-empty`, `editor-recents`) switched from `!model` to `nothingOpenedOrCreated`.

`locales/en/gui-editor.ftl` / `locales/de/gui-editor.ftl`: `editor-action-undo`/`editor-action-redo` appended to the `## Generic list/map actions` section, immediately after `editor-action-remove`, per the dispatch's placement ruling.

`eslint.config.js`: the one `vue/no-restricted-syntax` entry from the brief's fence, as the first rule in the existing `**/*.vue` rules block, above `@intlify/vue-i18n/no-raw-text`. Nothing else in the file changed.

`e2e/editor-rule-add-remove.spec.ts`: header doc corrected, "gated on `currentPath`" -> "gated on `sessionActive`".

`e2e/smoke.spec.ts`: the catalog-budget comment recomputed, "2 generic action keys" -> "4 generic action keys", 49 -> 51.

`e2e/editor-undo-redo.spec.ts` (new): Step 5's six mutation-path cases, Step 6's granularity/truncation/save/open/failed-open/depth-cap/U1 cases, and amendment 1's three-leg D112 case.

## Step 5: re-deriving the mutation-path set

Both authoring expressions were re-run, against the correct target: the tree as Task 3 left it (`git show HEAD:src/views/EditorView.vue`), not the tree with this task's own edits already applied -- Task 4 itself adds two more `model.value =` assignments (inside `undo`/`redo`), so running the sweep after implementing would corrupt the very measurement it is supposed to ground.

**Whole-value expression**, `grep -nE '^\s*model\.value = ' <tree>`:

```
250:    model.value = doc.profile ?? undefined;
355:  model.value = blankProfile();
435:  model.value = { ...(model.value ?? ({} as Profile)), [key]: value } as Profile;
442:  model.value = {
533:  model.value = { ...model.value, tracks: { ...model.value.tracks, rules: next } };
550:  model.value = {
580:  model.value = {
597:  model.value = {
```

**Finding, reported per the dispatch's instruction 3 (count vs. enumeration):** this is **8** lines, not the plan's authoring-time figure of "seven whole-value assignments." Mapped to enclosing symbols: `openPath` (250), `createBlank` (355), `setFieldValue` (435), `setTracksUnmatched` (442), `setRuleValue` (533), `onDrop` (550), `addRule` (580), `removeSelectedRule` (597) -- eight functions, not seven. The eighth is `createBlank`'s own `model.value = blankProfile();`. This is not a contradiction of the brief's closed six-function mutation set: the plan's authoring-time sweep was run before Task 1, when `createBlank`/New did not exist yet (D107 and `createBlank` are Task 3's own build). `createBlank` belongs in the same excluded category the brief already names for `openPath` ("a load, not an edit"): it is a session-start funnel whose `model.value` write is a structural no-op for the push rule, because `resetHistory` seeds `history[0]` to the exact same serialization *before* the watcher can observe the write (Step 1's own load-bearing-order requirement). The six MUTATION functions the brief names are exactly six of these eight, unchanged; the count drifted, the enumeration did not, and the enumeration is what the table below is built from.

**In-place-mutation expression** (the blind-spot check), against the same tree:

```
$ grep -nE 'model\.value\.[A-Za-z_]+ *=|model\.value\.[A-Za-z_.]*\.(push|splice|pop|shift|unshift|sort|reverse)\(' <tree>
(no output, exit 1)
```

Its fire, against a synthetic file containing `model.value.input = y;` and `model.value.tracks.rules.push(x);`:

```
1:model.value.input = y;
2:model.value.tracks.rules.push(x);
(exit 0, both lines matched)
```

External-writer control, `grep -n "<EditorView" -A2 src/App.vue`:

```
254:        <EditorView v-show="activeView === 'editor'" />
255-      </main>
```

No `v-model`; App.vue never writes the editor's model. No seventh mutation path exists. The mutation-path table in `e2e/editor-undo-redo.spec.ts` covers exactly the six functions the brief names: `setFieldValue`, `setTracksUnmatched`, `setRuleValue`, `onDrop`, `addRule`, `removeSelectedRule`.

## Step 4c: the standing lint rule, falsifiability

`eslint-plugin-vue` version, read from the installed package's own manifest: `10.9.2` (`node_modules/.pnpm/eslint-plugin-vue@10.9.2.../package.json`), matching the brief's cited figure. `vue/no-restricted-syntax` ships in it: `node_modules/eslint-plugin-vue/dist/rules/no-restricted-syntax.js` exists.

**RED** (the two gates from Task 3's committed baseline temporarily restored, new `eslint.config.js` in place):

```
$ pnpm exec eslint src/views/EditorView.vue
  903:14  error  A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax
  910:14  error  A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax
✖ 3 problems (3 errors, 0 warnings)
```

(The third error, `'nothingOpenedOrCreated' is assigned a value but never used`, is an artifact of the temporary revert making both real gates point at `!model` again, leaving the computed genuinely unread; it is not part of the rule's own count.) Exactly 2 `vue/no-restricted-syntax` errors, matching the brief's fence.

**Both selector members fired individually**, per the alternation-firing rule: `v-if` fires in the RED run above; `v-else-if` fired separately by pointing the shipped `v-else-if="sessionActive"` (the unsaved-path paragraph) at `!model`:

```
  897:19  error  A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax
```

**GREEN** (the shipped end state, `:disabled="!model || !canUndo"` / `!canRedo"` present):

```
$ pnpm exec eslint src/views/EditorView.vue
(no output, exit 0)
```

The over-match control holds: the `:disabled` bindings are present in the file and the rule does not flag them (scoped to `v-if`/`v-else-if` by directive name).

## Step 7 verification

**S1** (`savedSnapshot` never marked from live state):

```
$ grep -nE 'savedSnapshot\.value *= *(history|JSON\.stringify\(model)' src/views/EditorView.vue
(no output, exit 1)
```

Both alternation members fired against synthetic lines:

```
$ echo 'savedSnapshot.value = history.value[position.value];' | grep -nE '...'
1:savedSnapshot.value = history.value[position.value];   (exit 0)
$ echo 'savedSnapshot.value = JSON.stringify(model.value);' | grep -nE '...'
1:savedSnapshot.value = JSON.stringify(model.value);      (exit 0)
```

Pre-state control: `savedSnapshot` does not appear at all in the Task-3 baseline (`grep -n "savedSnapshot" <tree>`, exit 1) -- the zero above is a real absence, not an unfired pattern.

**D1** (no second save-state mechanism), GREEN (the end-state expression):

```
$ grep -nE "dirty\.value *=|(isDirty|unsavedChanges|modified) *= *ref\(" src/views/EditorView.vue
(no output, exit 1)
```

Fired against a synthetic `dirty.value = true;` (exit 0, matched). mkvtoolnix-gui control confirmed non-empty: `hasBeenModified() { return currentState() != p_func()->savedState; }` at `tab.cpp:654-655`.

**Divergence found and reported, not escalated:** the brief's own RED-state claim for D1's *broad* expression (`grep -nEi "dirty|isDirty|unsaved|modified" src/views/EditorView.vue`, "already measured on the baseline: 0 lines") does not reproduce. On the actual Task-3 committed baseline it returns 2 lines, both hits on the substring "unsaved" inside `data-testid="editor-unsaved"` and `{{ $t("editor-unsaved") }}` -- Task 3's own D107 "New profile, not saved yet" indicator, shipped after the plan's authoring-time sweep was measured (which predates Task 1). This is not a second dirty-tracking mechanism; it is a UI catalog key whose name happens to contain "unsaved." The GREEN check that actually gates this task uses the narrower, structural expression above, which I ran and confirmed at 0 both before and after this task's edits (the broad expression was never the acceptance gate, per the brief's own "the two alternatives are not equally strong" paragraph). Recorded here rather than silently reused, per the standing rule that a borrowed measurement answers its own question, not the current one.

> **CORRECTION, 2026-07-31, fix round 1 (task-4-verdict.md Finding 5).** The
> sentence above is wrong and is left standing rather than edited into
> agreement, because the wrong version is what a later reader would otherwise
> reconstruct. The count is **3 lines, not 2** -- my own raw tool output at
> the time already showed all three (lines 69, 632, 634) and the paragraph
> above under-transcribed it when compressing that output into prose, the
> exact "compression invents precision" failure this project's own
> `feedback_zitat_und_zahl_pruefen` memory names. Re-measured now, independently,
> against the task's actual base commit rather than trusting either my
> original figure or the reviewer's:
> ```
> $ git show 85902c7:src/views/EditorView.vue | command grep -nEi "dirty|isDirty|unsaved|modified"
> 69:// `editor-empty`, `editor-unsaved`, D107). The Open button, the
> 632:      data-testid="editor-unsaved"
> 634:      {{ $t("editor-unsaved") }}
> ```
> Three lines: the doc comment at 69 (missed by the original count) plus the
> `data-testid` and the `$t()` call. All three are the same "unsaved"
> substring from Task 3's `editor-unsaved` key; the qualitative conclusion is
> unchanged (not a second dirty-tracking mechanism) and the acceptance gate
> (the narrow, structural expression) is unaffected either way.

**P1** (no bare `v-if="!model` left):

```
RED, Task-3 baseline: grep -nE 'v-if="!model' <tree>
637:      v-if="!model"
644:      v-if="!model && recents.length"
(exit 0, 2 lines -- matches the brief's fence exactly)

GREEN, current tree:
$ grep -nE 'v-if="!model' src/views/EditorView.vue
(no output, exit 1)
```

**The full gate**, foreground, all eleven parts green:

```
cargo fmt --all --check                                                    exit 0
cargo clippy --workspace --all-targets -- -D warnings                      exit 0
cargo test --workspace                                                     exit 0 (507 tests across all crates and suites, 0 failed -- summed from every "test result: ok. N passed" line in the run)
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items   exit 0
cargo deny check                                                           exit 0 (advisories ok, bans ok, licenses ok, sources ok)
cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings  exit 0
pnpm lint                                                                   exit 0
pnpm build                                                                  exit 0 (vue-tsc clean, vite build succeeded)
pnpm check:i18n                                                             exit 0 (218 catalog ids, 0 unused warnings)
pnpm test:e2e                                                               exit 0 (93 tests, 93 passed, 0 failed)
python3 scripts/ledger-lint.py                                              exit 0 (574 entries across 4 files, all invariants hold)
```

`pnpm test:e2e`'s 93-test run includes every case in the new `e2e/editor-undo-redo.spec.ts` (14 tests) and the corrected `e2e/editor-rule-add-remove.spec.ts` (8 tests, unchanged behaviour), alongside the full pre-existing suite (71 other tests), all green.

**Catalog recount**: `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl` returns **51**, matching the brief's required figure. The `e2e/smoke.spec.ts` comment and `src/views/EditorView.vue`'s own catalog-count comment (unchanged from Task 3, still correctly citing 49 *for that task's own end state*, not touched by this task's Files list) are consistent with this.

**`git diff --stat`** covers exactly the seven files in the Files list (six modified, `e2e/editor-undo-redo.spec.ts` new):

```
 e2e/editor-rule-add-remove.spec.ts |   2 +-
 e2e/smoke.spec.ts                  |   2 +-
 eslint.config.js                   |  18 ++
 locales/de/gui-editor.ftl          |   2 +
 locales/en/gui-editor.ftl          |   2 +
 src/views/EditorView.vue           | 290 +++++++++++++++++++++++++++++++++++--
 6 files changed, 302 insertions(+), 14 deletions(-)
 e2e/editor-undo-redo.spec.ts       new file, 590 lines
```

## Residual named, not built (per the brief's own instruction)

**The behavioural-gap check** (a mutation landing inside one of `doSave`'s two `await` windows) is not written, per the brief's own argued disposition: the existing mock harness cannot hold a queued `plugin:dialog|save`/`save_profile` response open for a test to act inside, and building a releasable mock is new test infrastructure, out of this task's scope, surfaced for controller routing. What ships instead: the fix makes the marked value structural (the captured `profile`, never a live read), S1 pins that structure by grep, and this residual is stated here as the report requires.

> **CORRECTION, 2026-07-31, fix round 1 (task-4-verdict.md Finding 1, high).**
> The paragraph above is left standing rather than edited into agreement,
> because the wrong version is what a later reader would otherwise
> reconstruct, and because Tasks 5-6's reviewer needs to see exactly what was
> claimed and how it understated the gap. **The paragraph's frame is wrong,
> not just narrow.** It names the racy await-window case as the residual,
> which reads as "the ordinary, non-racing case -- does `doSave` mark
> `savedSnapshot` at all, correctly -- is otherwise covered." It is not. The
> actual residual is broader: **no test in this package can observe
> `savedSnapshot`'s value at all, in any scenario, racing or not.**
> `dirty` is the only consumer of that value (as opposed to its nullness),
> and `dirty` has zero consumers anywhere in this task's shipped code or
> tests -- the interfaces section of this task's own brief says so
> explicitly ("Produces: ... `dirty`, which Tasks 5 and 6 gate their guards
> on"). Independently reproduced (not just re-quoted from the review): with
> the entire line `savedSnapshot.value = JSON.stringify(profile);` deleted
> from `doSave`, `pnpm build` stays green and all 93 e2e tests stay green,
> including "save marks rather than clears" -- that case only asserts Undo
> stays enabled and one Undo restores the pre-edit state, both independent
> of whether `savedSnapshot` is ever written at all.
> ```
> $ pnpm build
> ✓ built in 153ms  (exit 0, mark line deleted outright)
> $ pnpm exec playwright test --reporter=list
> 93 passed (8.0s)  (exit 0, same mutation still in place)
> ```
> **The sentence "S1 pins that structure by grep" also overclaims its own
> reach**, corrected in the same breath: S1 is a one-shot grep pasted into
> this report at authoring time, not a standing gate check that runs on
> every future change, and it only matches two specific wrong *spellings* of
> the assignment (a live read from `history` or from `model.value`) -- it
> does not, and structurally cannot, catch an *absent* line or a right-shaped
> assignment carrying a wrong value in some other form. "Pins the structure"
> is true only for the narrow textual property S1 actually checks; it is not
> true for "the mechanism works," which is the reading the surrounding prose
> invites. This instantiates the house rule
> `a-normative-claim-is-scoped-down-to-its-producers-reach`
> (`docs/decision-ledger.yaml`, newly promoted to tier 2 on the strength of
> this review): the claim's reach is now stated as no wider than what S1 and
> the shipped code structurally guarantee.
>
> **No new test is added in Task 4 for this**, per the controller's ruling:
> the plan assigns this property's first real producer to Task 5's
> after-a-save leg, on the recorded ground that a test-only surface to
> observe `savedSnapshot`/`dirty` earlier would be a mechanism the product
> does not have yet. Task 5/6's reviewer must not treat Task 4 as having
> dynamically validated any part of the save-marking mechanism, racy or not
> -- their own tests are the first real producers for this property.

## Self-check against "Must not decide"

Everything in that list was implemented exactly as fenced or as explicitly ruled by D108/D112: the push rule's comparison form, the three explicit coalescing resets, `HISTORY_DEPTH = 100`, the keyboard condition set and its seven-member text-entry type set, that the save state is derived with no second flag, that saving marks a position, that opening resets, the two fenced button labels, the closed six-function mutation set, `nothingOpenedOrCreated`'s two terms and fenced expression as one named computed (not two inline conditions, not derived from `sessionActive`, no third state flag), that the path/unsaved/diagnostics/editing-surface gates were left untouched, that no catalog string was added for the failed-load state, that Task 3's shipped steps were not rewritten, and the standing guard's exact selector/message/scope with P1 kept beside it.

## One implementation decision made at the keyboard, disclosed

`dirty` is produced by this task and has no consumer inside it (Tasks 5-6 read it), which `@typescript-eslint/no-unused-vars` flags. Resolved with a single scoped `// eslint-disable-next-line @typescript-eslint/no-unused-vars` immediately above the `dirty` declaration, carrying a comment naming why (this codebase's own precedent for a narrowly-scoped, justified disable: `src/components/HelpSidebar.vue`'s `eslint-disable vue/no-v-html` block is the one other instance in the repo). Not a design decision the plan owns; a lint-mechanics resolution the plan's own Interfaces section already licenses ("Produces: `dirty`, which Tasks 5 and 6 gate their guards on").

## Fix round 1 (2026-07-31), against `task-4-verdict.md`

Three findings entered this round per the controller's dispatch; findings 4 and 5's remaining consequences were report-only and are the `> **CORRECTION`  blockquotes inline above, at their original locations, in this project's house form (`proc-supersede-never-overwrite`'s reporting counterpart: the wrong sentence stays standing, corrected beside it, never silently edited).

**Finding 1 (high, report-side only, no test owed).** The residual paragraph's understatement is corrected in place above (the "Residual named, not built" section), including the overclaiming "S1 pins that structure" sentence. No code or test change: the controller's ruling assigns this property's first producer to Task 5.

**Finding 2 (moderate, real coverage work -- attempted the stronger route first, then took the licensed fallback).** Tried to make the assertion defeat the fallback: is there any product surface that reads `canUndo`/`canRedo`/`history`/`position` while `model` is falsy, other than the two `:disabled` bindings? Traced every consumer: `undo()`/`redo()` carry the identical `!model.value` short-circuit (so a bail-early there is indistinguishable whichever way `canUndo` actually reads); no other template binding reads these refs; the mount harness (`__muxsmithModel__`) exposes only `model`, never internal refs, and there is zero precedent anywhere in this test suite for reaching into Vue's internal component state (`command grep -rn "setupState\|__vueParentComponent\|getCurrentInstance" e2e/ src/` returns nothing) -- inventing that path now would itself be the "invent one" the ruling forbids. **Measured: no such observation path exists in the shipped product.** Took the licensed second route: the test's name and its in-file comments are scoped down to what the assertions actually prove (the buttons read disabled via the model gate, the editing surface is gone, the diagnostic explains why), and the unobservable half (whether `history`/`position` were actually cleared) is stated explicitly beside it, instantiating `a-disabled-assertion-over-a-disjunction-proves-only-its-weakest-term`. The discriminating mutation, independently reproduced (not just quoted from the review):

```
$ # resetHistory's profile===undefined branch mutated to null only savedSnapshot,
$ # leaving history/position standing
$ pnpm build && pnpm exec playwright test e2e/editor-undo-redo.spec.ts -g "a failed open"
✓ 1 passed   <- still green under the defect; confirms the pre-fix test does not discriminate it
```

**Finding 3 (low-moderate, coverage work, case added).** `governs`: `tests-ship-with-the-feature-never-after` per the controller's ruling. Added `createBlank resets: New after edited history clears both Undo and Redo`, mirroring "open resets" through the other funnel (build history via Open + an edit, then New, assert both buttons disabled). Checked rather than assumed whether this sits behind the same fallback as Finding 2: `createBlank` unconditionally ends with `model.value = profile` (a fresh, truthy Profile), so `:disabled="!model || !canUndo"` never short-circuits on `!model` here -- `canUndo`/`canRedo`'s real values are what the asserts read. Discriminating mutation, independently reproduced:

```
$ # resetHistory(profile) call deleted outright from createBlank
$ pnpm build && pnpm exec playwright test e2e/editor-undo-redo.spec.ts --reporter=list
...
✘ createBlank resets: New after edited history clears both Undo and Redo
    Expected: disabled
    Received: enabled
14 passed, 1 failed   <- exactly the new case, none of the other 14, matching the
                          reviewer's own reproduction that zero of Task 4's pre-fix
                          tests caught this regression
```

**Verification, foreground, after restoring the source to its committed state (`git diff --stat -- src/views/EditorView.vue` empty throughout this round -- only `e2e/editor-undo-redo.spec.ts` carries a tracked change):**

```
$ pnpm lint
$ eslint .
(exit 0)

$ pnpm build
✓ built in 154ms (exit 0)

$ pnpm exec playwright test e2e/editor-undo-redo.spec.ts --reporter=list
15 passed (7.4s)

$ pnpm test:e2e
Running 94 tests using 16 workers
...
94 passed (8.1s)

$ pnpm check:i18n
check-i18n: ok (41 source files scanned, 218 catalog ids, ... 0 unused warning(s), ...)
```

Only `e2e/editor-undo-redo.spec.ts` changed in this round (`src/views/EditorView.vue` is untouched relative to commit `1092eb7`); per the dispatch's own instruction, the full eleven-part gate was not re-run since no change reached beyond the e2e file.
