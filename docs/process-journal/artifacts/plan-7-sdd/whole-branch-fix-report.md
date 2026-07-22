# Plan 7 whole-branch fix report

Single fixer, one pass, the six in-scope findings from the whole-branch
verdict. Working tree: main repo `/home/senol/Git/Muxsmith` on `master`
(baseline HEAD `c151390`). No worktree/session-relocation calls; absolute
paths throughout. No Rust files in scope (cargo untouched).

**Status: DONE.** Commit `13e138c`. Full e2e 53 passed (was 52; the new I1
case adds one). check:i18n green, eslint clean on all touched files.

## Per-item

### I1 (code half only) - help-mode dragstart suppression
- **Where:** `src/App.vue`. Added `onHelpDragstart(event)` calling
  `event.preventDefault()`, and registered/unregistered it as a
  capture-phase `dragstart` listener on `<main>` inside the existing
  `watch(helpMode)` block (same capture set as `click`/`focusin`).
- **Scope held exactly:** only `dragstart`, preventDefault only. No
  stopPropagation, no `beforeinput`/select gating - the keyboard/text
  residual is the explicitly out-of-scope OWNER decision. The handler's
  comment names focus/text entry as the owner-ruled residual (spec 8.3).
- **Mechanism:** preventDefault on dragstart aborts the drag before any
  drop fires, so `EditorView.onDrop` (the immutable `tracks.rules`
  rebuild) never runs - closing the only leak that could silently reach
  disk via a later Save.

### M1 - false "no live locale swap" narratives
- **`e2e/mocks.ts`** (`installMockIPC` doc): the parenthetical "(there is
  no live in-session catalog swap; ...)" replaced by prose scoping the
  helper to the RESTART/bootstrap path and naming the live D56 swap
  (`SettingsDialog.save()` -> `applyLocale`) as covered separately in
  `e2e/locale-switch.spec.ts`.
- **`e2e/smoke.spec.ts`** (the "German from the settings UI" test comment):
  "nothing in the app swaps the live FluentBundles afterwards" replaced by
  a RESTART-path framing plus an explicit note that the app DOES swap in
  place (D56), covered in `locale-switch.spec.ts`. Mechanisms and
  assertions unchanged; both tests still pass.

### M2 - stale "budget 45" at both sites
- **`src/editor/widgets/PropertyMapWidget.vue`** and
  **`src/editor/widgets/ListWidget.vue`**: numeral dropped, each now cites
  the boundary entry by name ("the closed editor catalog budget,
  `editor-generic-action-keys`"). Verdict's preferred option (a count in a
  code comment already re-staled once). The `registries.ts:185-189`
  reference in PropertyMapWidget was left untouched (out of scope, and no
  new file:line span was written anywhere).

### M3 (= triage 6, T18-nit) - check-i18n.mjs:56 overgeneralization
- **`scripts/check-i18n.mjs`** (check-2 comment): "reached only via a
  generic `$t(err.code, err.params)` pattern and never spelled out
  literally in src/" replaced by "reached via the generic pattern; most
  never appear as literals (FirstRun's two detect codes,
  mkvmerge-not-found/mkvmerge-too-old, are the exception - spelled out as
  switch cases in FirstRun.vue)". Matches `src/views/FirstRun.vue:39,41`.
  Zero behavioral effect (literalAnywhereIds already catches them).

### M5 - stale check-3 "current tree" paragraph
- **`scripts/check-i18n.mjs`** (check-3 comment): the "With only
  `locales/en/` present (current tree) ... passes trivially" paragraph
  rephrased to describe what check 3 scans today - the loop iterates every
  non-en `locales/<tag>/`, `locales/de/` is now present (this run reports
  "1 other locale(s) checked") and validated against en - with the
  trivial-pass history kept in past tense.

## I1 TDD evidence (red -> green, with the control)

New test: `e2e/help-mode.spec.ts`, describe "help mode drag suppression
(I1)". Loads the editor with a 3-rule profile (`source`: alpha/beta/gamma),
reads rule order off the source-cell buttons (`editor-rule-select`).
Helper `attemptDrag` dispatches a synthetic HTML5 drag and models the
browser drag state machine faithfully: it dispatches `drop`/`dragend` only
when the app left `dragstart` un-prevented (a real webview aborts the drag
otherwise). Two rAFs flush Vue's scheduler before the order is read.

**Structure (control + suppression in one test):**
1. Baseline order `[alpha, beta, gamma]`.
2. **Control, help OFF:** `attemptDrag(0, 2)` ->
   `dragstartPrevented === false`, `dropDispatched === true`, order becomes
   `[beta, gamma, alpha]`. Proves the synthetic drag genuinely reorders, so
   the help-mode assertion is not a vacuous pass (the required
   outside-help-mode control; expected value differs from the pre-drag
   default state).
3. **Suppression, help ON:** `attemptDrag(0, 2)` ->
   `dragstartPrevented === true`, `dropDispatched === false`, order stays
   `[beta, gamma, alpha]`.

**RED (against current code, before the App.vue fix):**
```
pnpm build   # unfixed App.vue into dist/
pnpm exec vite build --config e2e/vite.harness.config.ts   # + mount config
pnpm exec playwright test e2e/help-mode.spec.ts --grep "drag suppression"
```
Result: `1 failed`. The control assertions (help OFF, drop dispatched,
order reordered) PASSED; failure was at the suppression assertion
`expect(inside.dragstartPrevented).toBe(true)` -> `Received: false`
(help-mode.spec.ts:449). Exactly the I1 leak: current code does not prevent
dragstart in help mode.

**GREEN (after the App.vue fix):**
```
pnpm build   # fixed App.vue into dist/
pnpm exec playwright test e2e/help-mode.spec.ts --grep "drag suppression"
```
Result: `1 passed (671ms)`.

## Verification commands and results

| Command | Result |
|---|---|
| `pnpm build` (vue-tsc --noEmit && vite build) | GREEN - built, 165 modules, dist emitted |
| `pnpm exec tsc --noEmit -p e2e/tsconfig.json` | GREEN - e2e specs typecheck (incl. new test) |
| `pnpm test:e2e` (full, foreground) | GREEN - **53 passed** (52 prior + 1 new I1 case) |
| `pnpm check:i18n` | GREEN - "41 source files scanned, 211 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)" (identical to reviewer gate #8) |
| `pnpm exec eslint` on the 7 touched files | GREEN - exit 0, no output |
| cargo | Not run - no Rust files in scope (correct) |

Comment-only fixes (M1/M2/M3/M5) proven behavior-neutral by the gate: same
53 e2e passes and the byte-identical check:i18n summary line.

## Commit

`13e138c` on `master`, unsigned (`git -c commit.gpgsign=false`), explicit
7-file `git add` (no `-A`), house trailer `Co-Authored-By: Claude Fable 5
<noreply@anthropic.com>`. Subject: "gui/e2e: whole-branch fix wave - I1
help-mode dragstart suppression + M1/M2/M3/M5 comment sweep".

Files: `src/App.vue`, `e2e/help-mode.spec.ts`, `e2e/mocks.ts`,
`e2e/smoke.spec.ts`, `src/editor/widgets/PropertyMapWidget.vue`,
`src/editor/widgets/ListWidget.vue`, `scripts/check-i18n.mjs`.

## Surfaced / notes

- **New test infrastructure:** `attemptDrag`/`readRuleOrder` helpers plus a
  new describe block in an existing spec file. The preamble's structural-
  conformance grant excludes "new test infrastructure" from silent
  additive scope, but this is the plan-mandated I1 TDD case (the fix's
  required failing-test-first), not a silent extension - surfaced here per
  the grant's stop-and-report boundary. It adds coverage, weakens no
  existing assertion, mutates no fixture.
- **Drag test technique:** synthetic DragEvent dispatch gated on
  `defaultPrevented`, chosen over pointer-driven drag because the
  whole-branch review itself flagged headless HTML5 drag simulation as
  unreliable. The gate models real-browser semantics (a prevented
  dragstart yields no drop); the outside-help-mode control proves the
  synthetic drag is genuinely effective, so the suppression pass cannot be
  vacuous.
- No plan/design/ledger/spec files edited (controller-owned). I1's
  keyboard/text wording residual and M4/M6/triage items left untouched
  (routed elsewhere).
