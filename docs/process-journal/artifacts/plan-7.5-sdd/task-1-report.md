# Task 1 report: D65-D70/D72 - the Add/Remove affordance and `e2e/editor-rule-add-remove.spec.ts` (cases 1-8)

**Status:** DONE_WITH_CONCERNS (no design fork opened; the concerns are two
disclosed, zero-outward-effect deltas and one tooling observation about the
brief's own test command - see section 6.)

**Stream:** A, worktree `/home/senol/Git/Muxsmith/.worktrees/plan75-a`, branch
`plan75-a`. No session-relocation tool was used; every command ran with an
explicit `cd` into the worktree or absolute paths. All test runs foreground.

**Commit:** `fc9e9a41dcbeabdbbd34f1b6a8518e525710679c` (short `fc9e9a4`),
unsigned (`git log --format=%G?` -> `N`), trailer
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, files staged
explicitly. One commit; it was `--amend`ed once (nothing pushed) to add a
ledger-id citation to the spec's module doc, so an earlier local hash
(`1933cf3`) no longer exists. Tree clean after the commit.

**Files:** `src/views/EditorView.vue` (modified, +50 lines),
`e2e/editor-rule-add-remove.spec.ts` (new, 362 lines). Nothing else.

---

## 1. Environment precondition (not a step, recorded for reproducibility)

The fresh worktree had no `node_modules` and no `dist`, and `pnpm test:e2e`
serves `dist/` through `vite preview` (playwright.config.ts's `webServer`).
Run once before Step 4:

```
pnpm install --frozen-lockfile   # Done in 405ms
pnpm build                       # vue-tsc --noEmit && vite build, green
```

Playwright browsers were already present in `~/.cache/ms-playwright`
(chromium-1228).

## 2. Step 1: template anchor re-verified at the execution tree

```
$ grep -n -A1 '</table>' /home/senol/Git/Muxsmith/.worktrees/plan75-a/src/views/EditorView.vue
598:        </table>
599-      </fieldset>
```

Exactly one hit; the successor line is `</fieldset>`. Matches the
plan-authoring measurement (:598/:599) exactly. No NEEDS_CONTEXT condition.

## 3. Steps 2-3: the eight cases

One new file, one describe titled `editor rule add/remove (D65-D70, D72)`.

Cases 1-5 use the mount harness (`mountComponent`/`readModel`, no IPC mock,
no `currentPath`, so `watch(model)` never fires). Fixtures are the smoke
sibling's own shape: a two-rule profile (`exact: { type: "video" }` /
`exact: { type: "audio" }`) plus a one-rule variant for case 5. Row-cell
indices used as the brief measured them (0 ordinal, 1 select button, 2 match,
3 optional checkbox, 4 changes).

Cases 6-8 drive the served app with `installTauriMocks`
(`detect_mkvmerge` / `plugin:dialog|open` / `load_profile` /
`validate_profile_model: [clean, case-report]`), open via `editor-open`, and
assert the marker anchors by `data-diag-path` plus the severity class, exactly
the `editor-markers.spec.ts` mechanism. Diagnostic prose is asserted via
`en(code)`; no fixture's `rendered` field is read anywhere
(`e2e-diagnostic-rendered-is-wire-ballast`).

Every assertion the brief enumerates is present, including the ones easy to
drop: case 1's `readModel` anti-vacuity (`tracks.rules` length 3, member [2]
`toEqual({ match: {} })`), case 4's right-rule assertion (removed summary gone
AND the other summary still present), case 6's wire-truth payload check on the
LAST recorded `validate_profile_model` call, case 6(c)'s bare-`tracks[1]`
count-0 negative, case 6(e)'s `editor-save` still enabled, and cases 7/8's
`drop`-error / `keep`-info severity pair with the opposite Save gating.

## 4. Step 4: the red run (the fire event)

```
$ cd /home/senol/Git/Muxsmith/.worktrees/plan75-a && pnpm test:e2e -- --grep "editor rule add/remove"
...
  8 failed
    [chromium] > e2e/editor-rule-add-remove.spec.ts:96:3  ... Add appends the empty skeleton ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:126:3 ... Add auto-selects the new rule ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:144:3 ... Remove is disabled without a selection ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:156:3 ... Remove deletes the selected rule ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:179:3 ... Remove works down to zero rules ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:209:3 ... Add wires the skeleton onto the wire ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:277:3 ... Removing the last rule under drop ...
    [chromium] > e2e/editor-rule-add-remove.spec.ts:319:3 ... Removing the last rule under keep ...
  53 passed (31.0s)
[ELIFECYCLE] Command failed with exit code 1.
```

All eight red, each on a locator timeout waiting for the not-yet-existing
`editor-rule-add` / `editor-rule-remove` testid (e.g. `waiting for
getByTestId('view-editor').getByTestId('editor-rule-remove')`). This red run is
the fire event for every green below.

**Tooling observation (see section 6, concern 3):** the `--grep` did not reach
playwright - the run executed the full 61-test suite (53 pre-existing green + 8
new red). A repeat with `--list` appended was likewise ignored. The same flag
works under `pnpm exec playwright test --grep ...`, which is what section 6's
filtered green run used. The step's intent was satisfied either way (a superset
of the named run).

## 5. Step 5: implementation

`src/views/EditorView.vue`, two insertions and nothing else:

- **Script**, immediately after `onDragEnd`'s closing brace, before
  `</script>`: `addRule` and `removeSelectedRule`, D67's shapes transcribed
  verbatim (single-line guards kept - the repo has no `curly` rule and no
  prettier config, so no reformat was demanded; `pnpm lint` green confirms it).
  No new state: no new `ref`, no new `computed`, no new panel state.
- **Template**, between `</table>` and `</fieldset>`: D70's block, indented to
  the surrounding depth (8 spaces, sibling of `<table>`). Add before Remove,
  native `<button type="button">`, visible `$t` text only, no `aria-label`, no
  `title`, `:disabled="selectedIndex === null"` on Remove only.

## 6. Step 6: the landed blocks diffed against the design

Extraction: the two functions by their own `^function ... {` / `^}` ranges (so
the Step-5 comments, which sit above the functions, are excluded by
construction), and the template block by the `</table>` / `</fieldset>`
delimiters. Design side: D67's fenced `ts` block (dedented) and D70's fenced
`html` block, both read from the worktree copy of the design.

```
$ diff -wB design-d67.ts landed-funcs.ts
IDENTICAL (exit 0)
$ diff -wB design-d70.html landed-template.html
IDENTICAL (exit 0)
```

**Fire-verification of the diff method itself** (its passing result is an
absence): injecting `title="Add a rule"` into the extracted template block made
the same `diff -wB` produce output -

```
3c3
<   data-testid="editor-rule-add"
---
>           data-testid="editor-rule-add" title="Add a rule"
```

- then the probe file was discarded. So the two clean diffs are evidence, not a
malformed comparison. No attribute delta, no order swap, no changed `:disabled`
expression.

## 7. Step 7: green, and the untouched-spec invariant

Filtered run (via `pnpm exec`, since the brief's `pnpm test:e2e -- --grep`
form does not forward the flag):

```
$ pnpm exec playwright test --grep "editor rule add/remove"
  8 passed (705ms)
```

All eight named cases listed individually as passing. Full suite:

```
$ pnpm build && pnpm test:e2e
  61 passed (2.9s)
```

61 = the 53 pre-existing tests (unchanged, all green) + the 8 new ones. Zero
existing spec files edited:

```
$ git status --porcelain
 M src/views/EditorView.vue
?? e2e/editor-rule-add-remove.spec.ts
```

The existing-spec invariant holds structurally too: the buttons render outside
the `<table>`, so `editor-markers.spec.ts`'s exact `diag-marker` counts,
`smoke.spec.ts`'s `editor-rule-row` / `getByRole("columnheader")` assertions and
the Task-13b detail-panel spec see no DOM change inside their selectors.

## 8. Step 8: the focus-management negative, fire-verified

```
$ grep -rnE '\.focus\(|autofocus' /home/senol/Git/Muxsmith/.worktrees/plan75-a/src/
(no output, exit 1)
```

Fire-verified: appending `// fire-verification probe: el.focus()` to
`EditorView.vue` made the scan fire
(`src/views/EditorView.vue:678:// fire-verification probe: el.focus()`, exit
0); after removing the probe line the scan is empty again (exit 1) and
`git diff --stat` shows the implementation's 50 insertions only. The probe's
first restore attempt hit an interactive `cp -i` alias and did not take; it was
then removed by a targeted edit and re-verified (probe string count 0, file
tail `</template>`).

## 9. Step 9: the zero-surface check, fire-verified

```
$ git diff --name-only
src/views/EditorView.vue
$ git status --porcelain
 M src/views/EditorView.vue
?? e2e/editor-rule-add-remove.spec.ts
$ git diff --name-only -- src/            # positive control
src/views/EditorView.vue
$ git diff --name-only -- locales/ help/  # (empty)
$ git diff --name-only -- src/editor/ scripts/ src-tauri/ crates/  # (empty)
```

The positive control proves the command surfaces changes in the scope it is
given, so the empty scoped runs are evidence. Nothing under `locales/`,
`help/`, `src/editor/`, `scripts/`, `src-tauri/`, `crates/`. No registry
change, no new component, no `check-i18n` change, no Rust change, no
`DiagCode` change, no eslint config change, no new help-id or `data-help-id`,
no new topic file, no dependency (npm or cargo) added.

Catalog budget re-counted from the enumeration (`proc-normative-count-
recomputed`), not carried over:

```
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*[[:space:]]*=' locales/{en,de}/gui-editor.ftl
locales/en/gui-editor.ftl: 46 ids
locales/de/gui-editor.ftl: 46 ids
```

46/46, and `git status --porcelain -- locales/` is empty, so the catalogs are
byte-identical.

## 10. Step 10: frontend gate, foreground

```
$ pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e
$ eslint .                       -> clean, no output
$ vue-tsc --noEmit && vite build -> built in 183ms
$ node scripts/check-i18n.mjs
check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s)
gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s)
checked for parity against 7 en/ catalog(s)).
$ tsc --noEmit -p e2e/tsconfig.json && vite build x2 && playwright test
  61 passed (2.8s)
=== pipeline status: 0 ===
```

Run twice green: once before the commit, and again after the module-doc edit
that the amend carried, so the committed byte state is the gated one. eslint
`no-raw-text` is satisfied - the buttons' only text nodes are `$t()` calls.

**The five cargo gate parts were not run.** The diff touches zero Rust and zero
Cargo files (Step 9's scoped absence checks cover `src-tauri/` and `crates/`),
and Step 10 enumerates exactly the four frontend parts; the nine-part gate is a
pre-push obligation and no push happened. Flagging it so the controller runs the
full gate before merging rather than assuming it ran here.

## 11. Self-review

**Design section 8 ("what the implementer must not decide"), item by item:**

| Item | Status |
| --- | --- |
| Skeleton is `{ match: {} }`, uncast, nothing prefilled | held (diff -wB identical; case 1/6 assert the value) |
| `addRule`/`removeSelectedRule` are D67's shapes verbatim, no other state | held (diff -wB identical; no new ref/computed) |
| No programmatic focus call, no `autofocus` | held (Step 8, fire-verified) |
| Buttons are exactly D70's block (testids, order, `$t` text, no `aria-label`/`title`, Remove's only disable condition) | held (diff -wB identical) |
| Zero catalog changes, either locale | held (46/46, `locales/` untouched) |
| Zero help-id changes, no topic files | held (`help/` untouched, no `data-help-id` added) |
| No help-mode code | held (no button-side help-mode condition) |
| No editor-side zero-rule guard, no core change | held (Remove runs to zero; cases 5/7/8 exercise it) |
| E2e set is section 5's cases in the named file; no existing spec extended | held (`git status` shows two paths) |

**Anti-vacuity of the absence assertions in the new spec.** Three assertions
pass by absence, so each has a control:

- Case 4's "no row carries `aria-current='true'`" - fire-verified directly: a
  temporary probe asserting `toHaveCount(1)` immediately after the select click
  passed (`1 passed`), proving the selector
  `[data-testid="editor-rule-select"][aria-current="true"]` matches a real
  selected row; the probe was then removed and the case re-run green.
- Case 6(c)'s `[data-diag-path="tracks[1]"]` count 0 - the brief's designated
  in-test positive control is (b), the same marker layer and locator helper
  demonstrably rendering at `tracks[1].match`. Additionally
  `editor-markers.spec.ts` already asserts a marker DOES render at the bare
  `tracks[1]` for a bare-path diagnostic, a pre-existing positive control for
  this exact selector and path.
- Case 4/5's zero counts (`editor-rule-detail`, `editor-rule-row`) - each is
  preceded in the same test by a nonzero count of the same locator, so the
  transition is real.

**Typography.** Both changed files are pure ASCII: `grep -P '[^\x00-\x7F]'`
empty on the spec and on the `EditorView.vue` addition hunks, and the AI-tell
glyph class (em/en dash, smart quotes, ellipsis, NBSP) likewise empty. Both
scans fire-verified against known-present cases
(`locales/de/gui-editor.ftl` -> 39 non-ASCII lines; a hand-written probe line
containing an em dash, curly quotes and an ellipsis -> matched). No
line-number citations in any added comment
(`code-comment-line-citations-drift`).

**House pattern.** The functions sit in the rule-grid mutation region beside
`onDrop`/`onDragEnd` and reuse that region's immutable-whole-model-swap idiom
verbatim; the buttons follow the in-file `editor-open`/`editor-save` precedent
(visible `$t` text, no `aria-label`) and the `editor-rule-*` testid scheme; the
spec follows the two sibling harness patterns (`smoke.spec.ts` Task 11/13b for
mount, `editor-markers.spec.ts` for mocked IPC) including the `name()` exact
accessible-name helper and the `en()` catalog binding.

## 12. Concerns and disclosed deltas

1. **Two comment blocks, not one (disclosed, zero outward effect).** Step 5
   permits a comment above `removeSelectedRule` noting the D66 clearing
   rationale. I wrote that one, and additionally a region header above
   `addRule` (`// --- Plan 7.5 (D65-D70, D72): Add/Remove for track rules ---`
   plus six lines on the skeleton/auto-select/no-focus rationale), matching the
   file's own `// --- Task N ... ---` region-header convention that every other
   block in this file carries. Both sit outside the function bodies and outside
   Step 6's extraction (the diffs confirm it), and neither adds design latitude.
   Read as structural conformance under the standing implementer-brief
   house-pattern grant; flagged rather than assumed, since Step 5's permission
   names one comment.
2. **The commit was amended once (disclosed).** After the first commit I added
   the `e2e-diagnostic-rendered-is-wire-ballast` ledger id to the spec's module
   doc (house pattern: sibling specs cite ledger ids by bare id), re-ran the
   full frontend gate green, and amended. Net effect is one commit,
   `fc9e9a4`; the pre-amend hash `1933cf3` is unreachable and was never pushed.
3. **`pnpm test:e2e -- --grep "<pattern>"` does not filter (tooling finding for
   the controller).** Measured twice in this worktree: the flag ran the full
   61-test suite, and appending `--list` was likewise ignored, while
   `pnpm exec playwright test --grep "<pattern>"` filters correctly. I state
   only the observation, not a mechanism. Consequence for this plan: Steps 4
   and 7 of any brief that names the `pnpm test:e2e -- --grep` form will run
   the whole suite. That is a superset of the intent (and arguably better
   evidence), but the follow-on tasks' briefs may want the `pnpm exec` form for
   a fast red/green loop. No design fork; nothing was decided at the keyboard.
4. **Minor line-ref drift in the brief (informational, no action taken).**
   `e2e-diagnostic-rendered-is-wire-ballast` is at
   `docs/decision-ledger.yaml:4111` in this worktree, not `:4110`. The entry id
   exists and is unambiguous; I cite it without a line number, so nothing in
   the code depends on the ref.

## 13. Interfaces produced (for Task 2's case 9 and Task 4's amendment 1)

- `addRule()` / `removeSelectedRule()` in `src/views/EditorView.vue` (script
  region after `onDragEnd`).
- `<button data-testid="editor-rule-add">` and
  `<button data-testid="editor-rule-remove">`, rendered inside the rules
  `<fieldset>` between `</table>` and `</fieldset>`, Add first. Remove is
  disabled exactly when `selectedIndex === null`; Add is never disabled while
  the grid renders. Both render `$t("editor-action-add")` /
  `$t("editor-action-remove")` as their only text node.
- Both are keyboard-reachable native buttons; a disabled Remove is skipped by
  Tab (no `aria-disabled`). Add is the non-vacuous target for Task 2's
  help-mode suppression case, exactly as the design notes.
