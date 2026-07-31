# Task 5 verdict (independent review)

Reviewer instruments live outside the repo at
`/tmp/claude-1000/-home-senol-agents-peter/a1386daa-bdbc-4366-b18d-375daf90cf89/scratchpad/task5-review/`
(a standalone `playwright.config.ts`, `concurrency.probe.ts`, `inert-check.html` /
`inert-check.probe.ts`, `showmodal-twice.probe.ts`, `esc-debug.probe.ts`, with
`node_modules`/`e2e`/`src`/`locales` symlinked in from the repo so the repo's own
`e2e/mocks.ts` and `e2e/i18n-en.ts` run unmodified). Every in-tree mutation below was
applied with `Edit`, rebuilt (`pnpm build`), run, then reverted and re-verified by
`md5sum` (not by `git status`/exit code) before the next one, per the working rules.
The repo was left clean (`git status --porcelain=v1` empty, `HEAD` still `0b00262`)
at every checkpoint.

## Verdict 1: spec compliance

**MET, requirement by requirement, against `task-5-brief.md` (127-line, amendment-2
version).**

- **Step 1 (`ConfirmDialog.vue`).** MET. Native `<dialog data-testid="confirm-dialog">`
  with `showModal()`; props `title`/`message`/`confirmLabel`/`cancelLabel` exactly;
  `defineExpose({ ask })` returns `Promise<boolean>`; confirm button
  `confirm-dialog-confirm` resolves `true`, cancel button `confirm-dialog-cancel` and
  Esc both resolve `false` via the shared `close` handler. Esc's functional behavior
  verified empirically (own instrument, see Findings A/B below) since its citation in
  the shipped comment does not hold up.
- **Step 2 (two guarded call sites).** MET. `pickAndOpen`'s guard sits before
  `openDialog(...)` (byte-read at the shipped call site, not the tests' description of
  it); `createBlank` is `async` with the identical guard ahead of the seed;
  `openPath` carries no guard. Ordering verified by mutation (below): swapping the
  guard to after the file dialog call breaks exactly Cases 1, 2 and 3(ii) and no
  others.
- **Step 3 (catalog, both locales).** MET. Both `.ftl` additions byte-identical to the
  brief's fence (diff-read, not paraphrased).
- **Step 4 (smoke cases 1-6).** MET. All six present with the specified shape; the
  ordering absence checks (G1/G2) and Case 5's fire-then-zero pair independently
  re-run and confirmed to have real teeth (below).
- **Step 4b (repair the three named cases).** MET. All three repairs
  (`"open resets"`, `"createBlank resets"`, `"a failed open ..."`) are the identical
  two-line pattern (visibility assertion + confirm click), no pre-existing assertion
  touched. Test-power independently verified per case by three separate, isolated
  mutations (below) - each case fails for exactly its own documented reason and no
  other case regresses.
- **Step 4c, both halves, checked separately.**
  - Half 1 (`e2e/smoke.spec.ts` budget comment): MET, byte match confirmed by direct
    read against the fence.
  - Half 2 (`EditorView.vue` header sentence): MET, byte match confirmed by direct
    read against the fence; `46 + 8 = 54` decomposition checked.
  - Recount: own `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/{en,de}/gui-editor.ftl`
    run independently, both **54**. Full decomposition
    `42 + 1 + 4 + 1 + 3 + 3 = 54` verified; the four generic action keys confirmed by
    grep to be `add`/`remove`/`undo`/`redo`.
- **Step 5 (gate green, six-file stat).** MET. `git show --stat 0b00262` independently
  confirms exactly the six files in the brief's exhaustive Files list, matching
  `479 insertions(+), 24 deletions(-)`. Did not re-run the whole 11-part gate (per the
  brief); re-ran the parts each finding needed (targeted Playwright runs, `pnpm build`)
  and all were green on the unmutated tree.
- **Step 6 (commit).** MET. `0b00262` on `master`, unsigned, correct file list,
  correct subject line, not pushed.
- **Must-not-decide list.** Checked item by item against the diff; all held (native
  `<dialog>`, guard on exactly `pickAndOpen`/`createBlank` and not `openPath`, the
  three fenced strings and `settings-cancel` reuse, no third "save first" button,
  Case 6 extended not duplicated, no rule-removal confirmation added, the Step 4b
  three-case set and its non-weakening repair, both Step 4c halves owned in one step
  with the count recomputed from the catalog).

**Target 1 (Case 3(ii), the leg-3(ii) marking-mechanism coverage) — independently
reproduced, not trusted from the report.** Commented out
`savedSnapshot.value = JSON.stringify(profile);` in `doSave` (`src/views/EditorView.vue`),
rebuilt (asset hash changed `...CEyJLAxY.js` -> `...ZAInoytH.js`, matching the report's
own mutation build byte for byte), ran `pnpm exec playwright test e2e/smoke.spec.ts -g
"discard guards"`: **exactly Case 3(ii) failed, the other five stayed green.** Reverted,
confirmed restore by `md5sum` (`...3c53acba...`, matching the pre-mutation file), rebuilt
(asset hash back to `...CEyJLAxY.js`), re-ran: all six green. The plan's own acceptance
map is not falsified.

**Target 2 (ordering) — independently attacked.** Swapped `pickAndOpen`'s guard to run
after `openDialog(...)` instead of before. Rebuilt, ran the discard-guards describe
block: **Cases 1, 2 and 3(ii) failed, Cases 3(i), 4 and 5 stayed green** - exactly the
cases whose absence checks depend on the ordering. Reverted and content-verified.

**Target 3 (the three repaired `editor-undo-redo.spec.ts` cases) — independently
attacked, one isolated mutation per case:**
- `"open resets"`: made `openPath` skip `resetHistory` on any open after the session's
  first (a `wasAlreadyOpen` gate). Ran the full 15-case file: **only `"open resets"`
  failed**, the other 14 (including `"a failed open ..."` and the D112 three-leg case,
  which share the same `openPath`) stayed green.
- `"createBlank resets"`: dropped `createBlank`'s own `resetHistory(profile)` call.
  Ran the full file: **only `"createBlank resets"` failed**, 14 others green.
- `"a failed open ..."`: this case's own header comment already establishes (and a
  prior reviewer already reproduced) that its Undo/Redo assertions cannot discriminate
  `resetHistory` at all, because `!model` alone gates them - so I targeted the claim it
  actually makes instead: dropped `diagnostics.value = doc.config_diagnostics` on a
  failed load. Ran the full file: **exactly `"a failed open ..."` and the unrelated
  D112 three-leg case (which also reads the same diagnostic text) failed**, 13 others
  green.

Each mutation reverted and content-verified (`md5sum` match) before the next; the full
15-case `editor-undo-redo.spec.ts` file is green on the restored, original source.

## Verdict 2: task quality

**0 Critical, 2 Moderate, 1 Low.**

**Finding A (Moderate) - Esc is an explicit part of Step 1's contract and ships
completely untested.** Step 1 specifies the `ask()` contract explicitly: resolved
`false` "by `close` and by Esc." No case anywhere in the six new `smoke.spec.ts`
cases or the three `editor-undo-redo.spec.ts` repairs exercises Escape. This is the
exact trigger the house entry `tests-ship-with-the-feature-never-after` names ("you are
about to write... about behavior THIS package introduces" with no producer) - Esc-as-
cancel is a user-visible consequence Step 1 itself specifies, and this package is the
only place it could be tested. I built my own instrument and confirmed the mechanism
works correctly (Escape on the open dialog fires `cancel` then `close`, `onClose`
resolves `false`, no destructive action proceeds) - so this is a coverage gap, not a
live bug, but a real one against a standing rule.

**Finding B (Low) - a false citation in the shipped `ConfirmDialog.vue` comment.**
`onClose`'s doc comment claims Esc follows "the same Esc-consumed-by-the-native-cancel
semantics `SettingsDialog.vue` documents for its own dialog." I read the entirety of
`SettingsDialog.vue` and grepped it (and its full git history) for any mention of Esc,
cancel-event, or dismiss semantics: **zero hits.** `SettingsDialog.vue`'s Cancel button
just calls `close()`; the file documents no Esc semantics at all. The task-5-brief.md's
own required-reading line ("its Esc note") repeats the same mistaken premise, so this
is not a fabrication invented at the keyboard so much as an inherited one - but it
still ships as a false statement in the codebase, and a future reader following the
citation to see how `SettingsDialog.vue` "documents" this will find nothing there.

**Finding C (Moderate) - `ConfirmDialog.ask()` has no internal reentrancy guard; a
second, non-hit-tested call while one is pending silently steals the confirmation.**
The report's Q1 conclusion ("no second click on New or Open can land... before the
modal actually blocks the page") is **true for every real-input path in the shipped
UI** - verified with my own instrument down to the browser-primitive level: an
isolated bare `<dialog>` test shows a real, hit-tested mouse click on a button covered
by an open modal dialog does **not** reach its handler, while the exact same test shows
the DOM `.click()` IDL method (a script-level call, not hit-tested) **does** reach it
regardless of modality. Applying that to the shipped app: clicking New, then - after a
real task-queue tick, not the same synchronous turn - clicking Open, then confirming
once, causes **Open's own request to win and New's to vanish silently** (`plugin:dialog|
open` fires, the seed is never created), reproduced against the actual, unmutated
shipped bundle. Mechanism: `ask()`'s single `settleAsk` closure variable is
unconditionally overwritten by a second call while the first is still pending (calling
`showModal()` on an already-open `<dialog>` is a no-op in this Chromium, not a throw),
so the first caller's promise is orphaned forever and the second caller's confirm-click
resolves instead. **Not reachable today through any real user interaction** - no
keyboard shortcut routes to either guarded function (`onEditorKeydown` only handles
Ctrl+Z/Ctrl+Y), and genuine pointer/keyboard input is correctly blocked by the native
modal, confirmed both in isolation and against the app. This is a robustness gap in the
component itself (it defends via the caller's environment, not its own state) rather
than a live bug, worth hardening before `ConfirmDialog` gets the "second caller" its
own doc comment already anticipates.

No other findings survived verification. The disjunction-trap dimension, the
`editor-recents` conjunction gate, typography, German orthography, the house
`editor-generic-action-keys` budget arithmetic, and the `frontend-mutation-evidence-
needs-a-rebuild-before-the-e2e-run` / `gitignored-paths-need-command-grep` process
entries were all checked and held.

## Adjudication answers

**Q1.** True for every real-input path in the shipped app (verified down to the
browser-primitive level, not just weighed): a real mouse/keyboard click cannot land on
a button covered by an open modal `<dialog>`, and no keyboard shortcut bypasses that.
False as a universal claim about the mechanism itself: `ConfirmDialog.ask()` has no
internal reentrancy guard, and a second call reaching it other than through a
hit-tested click (reproduced via a script-level `.click()`, one real task tick apart)
silently discards the first caller's request and answers the second's instead - a real,
reproduced gap in the shipped, unmutated code, currently unreachable by any real user
action.

**Q2.** Confirmed 54 in both locale files by my own `grep -cE` run, independent of the
report's own paste. Decomposition: 42 labels + 1 save-surface note + 4 generic action
keys (`add`/`remove`/`undo`/`redo`, confirmed by grep) + 1 rule-grid ordinal + 3
profile-creation keys + 3 discard-confirmation keys = 54. Both budget-comment sites
(`e2e/smoke.spec.ts`, `EditorView.vue`'s header block) match the brief's fenced text
byte for byte on direct read.

**Q3.** True, and Task 5 adds nothing that changes it. `editor-recents`' render gate
(`nothingOpenedOrCreated = !model.value && currentPath.value === null`) is a
conjunction that already excludes any state where a model is held; nothing in either
guard's own body touches `model.value` before the confirm resolves, so the section
stays absent from the DOM (count 0, not merely hidden) through the entire pending-
confirm window - confirmed with my own instrument, seeding one real recent profile and
checking DOM presence both before any profile is opened (count 1, the "fire") and while
dirty with a confirm outstanding (count 0), including with the section's own testid
queried directly rather than through a click.

## Harvest

- **The report's own strongest empirical claim (Q1's "no mechanism needed") was
  correct in its practical conclusion but not in its universal phrasing, and the gap
  only surfaces below the level of Playwright's own `.click()` API** - a reviewer who
  stops at "does a real double-click work" (it does) will miss that the component's
  own state machine has no defense of its own. Where a design note frames a
  concurrency question as "does the modal make X unreachable," the sharper version to
  ask is "does the GUARDED FUNCTION defend itself, or only its current callers" -
  because the second caller the component's own doc comment anticipates ("and any
  second caller this component was built for") is exactly the one that would exercise
  this.
- **A comment that cites another file as documenting something is a claim to verify,
  not just code prose to skim** - `feedback_zitat_und_zahl_pruefen`'s discipline
  (verify a quote against its artifact) applies to source comments citing sibling
  files, not only to review prose citing evidence. Here the false premise originated in
  the brief's own required-reading line and propagated into shipped code unchecked.
- **This task's own escalation (Step 4b's NEEDS_CONTEXT over the two-vs-three-case
  collision) was well-founded, not an invented one** - two fixed, contradicting
  written statements in the same brief, resolved by an owner-ruled plan amendment
  rather than a keyboard judgment call. No fork in this task struck me as a stop with
  no real decision content behind it.
- **`docs/product-boundaries.yaml`'s `editor-generic-action-keys` entry is stale and
  orphaned, but not by this task.** It last records the catalog budget's history as
  `43 -> 45 -> 46`; the real count was already 51 before Task 5 started (Task 4 added
  undo/redo as two more generic action keys, never reflected there) and is 54 now.
  The brief explicitly and correctly scopes Task 5's own correction to exactly two
  files (`e2e/smoke.spec.ts`, `EditorView.vue`), so this is not a Task 5 compliance
  defect - but the entry has no current owner, and `proc-normative-count-recomputed`'s
  "sweep reaches the callers' docs" principle suggests a future task should either
  fold it in or explicitly retire it as historical.

Verdict file written to `.superpowers/sdd/plan-12/task-5-verdict.md`.
