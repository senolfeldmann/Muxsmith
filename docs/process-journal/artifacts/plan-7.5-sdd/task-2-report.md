# Task 2 report: D71 help-mode conformance - additive case 9 in `e2e/help-mode.spec.ts`

**Status:** DONE_WITH_CONCERNS (one reportable finding, no fork opened, nothing decided at the keyboard)
**Worktree:** `/home/senol/Git/Muxsmith/.worktrees/plan75-a` (branch `plan75-a`), base `fc9e9a4` (Task 1)
**Commit:** `92ba1e79a179415089afbdbdc8a9fae7fc0b66ff` - `e2e/help-mode.spec.ts` only, +83/-0, unsigned (`%GG` empty), repo trailer present, staged explicitly by path.

---

## Step 1: anchors re-verified by quoted text

Grepped `test.describe` in the current worktree tree, not by line number:

- `test.describe("help mode (D52)", () => {`
- `test.describe("help mode annotations (D54)", () => {`
- `test.describe("help mode drag suppression (I1)", () => {`

All three present with the exact titles the brief quotes. The new test is appended as the **third and last** test inside the `"help mode (D52)"` block, confirmed by the runner's own path: `help mode (D52) > the rule-grid Add button mutates outside help mode; both activation channels are suppressed inside it`. Describe count is unchanged at three, so the design's "the file's three describes" statement needs no recount (`proc-normative-count-recomputed` checked: no live normative count over this file's case set exists in `docs/`; the `help-mode.spec.ts` hits there are plan/verdict records, i.e. evidentiary, never re-pointed).

Interfaces consumed as specified, no new imports needed - `MKVMERGE_INFO`, `topicMarkup`, `normalizeInPage`, `installTauriMocks`, `resolveWith`, and the `Profile`/`LoadProfileDocument`/`ReportDocument` types were already in the file's import set.

## Step 2: the test

Title verbatim from the brief. Shape per the I1 sibling: real served app + `installTauriMocks`, its own inline opened-profile fixture, mutation control and suppression assertion in the SAME test and harness.

- Fixture: one-rule profile `{ match: { exact: { type: "video" } } }` (Task 1's `oneRuleProfile` shape), `input: { pattern: ".*", extensions: ["mkv"] }`.
- Mocks: `detect_mkvmerge`, `"plugin:dialog|open"`, `load_profile`, `validate_profile_model` - single-entry queues, which `mocks.ts` repeats on exhaustion, so the two control mutations' revalidations are covered without an exact call count.
- Sequence: `nav-editor` -> `editor-open` -> count 1; **controls (help OFF)** click Add -> 2, `focus()` + `keyboard.press("Enter")` -> 3; `help-toggle` -> sidebar visible; **help ON** click Add -> count still 3 and `sidebar.innerHTML()` equals `normalizeInPage(page, topicMarkup("view-editor"))`; `focus()` + Enter -> count still 3.
- The Add-not-Remove rationale is transcribed as a comment, as the brief requires.

Zero production code, zero new surface: the commit touches exactly one file. No help-id, no `data-help-id`, no topic file, no catalog id, no registry, no listener, no button-side condition.

Verified against the shipped delegation in `src/App.vue` (read, not assumed): `onHelpClick` does `preventDefault` + `stopPropagation` then pins `helpTarget(event)`; `onHelpKeydown` intercepts Enter/Space when `helpTarget(event) !== null`; `helpTarget` walks `closest("[data-help-id]")`. The buttons sit inside `<section data-testid="view-editor" data-help-id="view-editor">` and the `editor-tracks-rules` annotation is on the `<caption>` INSIDE the table, which is not an ancestor of the buttons - so the fallthrough resolves to `view-editor`, exactly as D71 states.

## Step 3: runs (foreground, corrected invocation form)

Ran `pnpm exec playwright test ...` per the binding correction, never the `pnpm test:e2e -- --grep` separator form.

| Command | Result |
| --- | --- |
| `pnpm exec playwright test --grep "both activation channels are suppressed"` | 1 passed (751ms) |
| `pnpm exec playwright test help-mode` | 9 passed (905ms), all 8 pre-existing cases green |

## Step 4: additive-only check, fire-verified

- Baseline: `git diff --numstat e2e/help-mode.spec.ts` -> `83	0`.
- Fire: deleted one existing line (the I1 sibling's `expect(await readRuleOrder(page)).toEqual(["alpha", "beta", "gamma"]);`) -> `83	1`. The deletion column is demonstrably live, not structurally always-zero.
- Restored via `command cp -f` from a scratchpad backup -> `83	0`, and `git diff --stat` shows the single file, 83 insertions, 0 deletions.

## Extra fire-verification: are the two suppression assertions themselves falsifiable?

The brief's Step 4 fire-verifies the numstat check. The two unchanged-count assertions are also absence-shaped, so I broke the shipped suppression deliberately and watched them (dist is served by `vite preview`, so each round rebuilt).

| Round | Neutralized in `src/App.vue` | Result |
| --- | --- | --- |
| A | `preventDefault`+`stopPropagation` in `onHelpClick` | Test FAILED at the click-half assertion: `Expected: 3, Received: 4` |
| B | `preventDefault`+`stopPropagation` in `onHelpKeydown`'s Enter branch | Test PASSED - the assertion did NOT fire |
| C | both of the above, with the click-half expectation temporarily relaxed to 4 so the run reaches the Enter half | Test FAILED at the Enter-half assertion: `Expected: 3, Received: 5` |

Both assertions are therefore live and reachable, and neither is racy (the mutation is visible at the very assertion that follows the activation - no scheduler flush helper was needed, unlike `attemptDrag`'s own rAF pair). `src/App.vue` was restored from a `command cp -f` backup and verified byte-identical to HEAD (`git diff --exit-code src/App.vue` clean) before the gate ran; the working tree carried only `e2e/help-mode.spec.ts` at commit time.

Round B is the finding below.

## Step 5: frontend gate, foreground

`pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e` - all green.

- `eslint .` clean; `vue-tsc --noEmit && vite build` clean.
- `check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s))` - the help-id count is unchanged at 22, the zero-new-help-ids claim of D71 confirmed by the gate itself.
- `pnpm test:e2e`: **62 passed**.

The five cargo parts were not run: this task touches no Rust and the brief's Step 5 enumerates the four frontend parts. The nine-part gate is the controller's pre-merge/pre-push duty.

## Step 6: commit

`git add e2e/help-mode.spec.ts` then `git -c commit.gpgsign=false commit` with the brief's exact subject and the `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer. Never `git add -A`. No session-relocation tool was called at any point; every command used an explicit `cd` into the worktree root.

## Self-review

| Check | Result |
| --- | --- |
| Exactly one test appended, inside `"help mode (D52)"` | yes (runner path confirms) |
| Task 1's files untouched | yes - commit contains only `e2e/help-mode.spec.ts` |
| Zero new surface (help-id, topic, catalog id, registry, component, listener, Rust, DiagCode, eslint config) | yes - one test file, and `check:i18n` reports the unchanged 22 help ids / 211 catalog ids |
| No new dependency | yes |
| Typography: ASCII hyphens, straight quotes, no Unicode ellipsis | yes - a strict `LC_ALL=C grep '[^ -~]'` over the added lines returns nothing, and that scan was fire-verified against a known-present em-dash line (positive control) |
| `code-comment-line-citations-drift` | conformant - comments anchor on symbol names (`onHelpClick`, `onHelpKeydown`, `@click="addRule"`, `helpTarget()`) plus the file, never a bare `file:line` span |
| `help-mode-suppression-pointer-scope` | comment states the live channels (typing, keyboard select changes) and why a button has neither; no claim beyond the boundary's wording |
| No design-latitude clause added, no fork resolved at the keyboard | yes - the enumerated assertion set was implemented as given; the one deviation candidate (see the finding) is reported, not acted on |

## Finding (the reason for DONE_WITH_CONCERNS)

**The Enter-half assertion is over-determined: it cannot distinguish a keydown-interception regression.** Round B removed `onHelpKeydown`'s Enter suppression entirely and the case still passed, because Enter on a focused `<button>` synthesizes a click, and that synthesized click is caught by the capture-phase `onHelpClick` listener on `<main>`, which `stopPropagation`s it before `@click="addRule"` runs. Only round C, with BOTH layers down, moved the row count.

Consequences, stated precisely:

- D71's actual claim - "both mutation paths into the model are therefore closed in help mode by the existing capture-phase delegation" - is what case 9 asserts, and that claim is fully verified: the keyboard channel does not mutate the model. Round C proves the assertion is live rather than structurally always-true.
- What case 9 does NOT pin is the *attribution* inside D71's prose ("Enter/Space ... is intercepted by `onHelpKeydown`, `preventDefault()`ed - so the native button activation click is never synthesized"). If that interception regressed, this case would stay green.
- That mechanism is not unguarded repo-wide: the pre-existing D54 case `keyboard: focusin swaps the topic (focusin equivalence); Enter on a focused annotated element pins it` exercises `onHelpKeydown`'s own effect directly.

I did not strengthen the case (e.g. asserting the keydown's `defaultPrevented`, or that no second pin/activation path fires), because the brief's Step 2 presents the assertion set as "design case 9, complete" and the plan's Global Constraints forbid an implementer widening a closed enumeration. This is handed to the controller as a possible content decision, not as a blocker: the case as specified is green, non-vacuous and correct about the outcome it asserts.

Also worth recording for the reviewer: this over-determination means the design's D71 sentence describing the keydown route as *the* thing that closes the keyboard channel is incomplete rather than wrong - the click delegation closes it too, redundantly. No spec or Tier-2 text needs changing for the code to be correct; it is a precision issue in a rationale paragraph.

---

# Fix round 1 (resumed Task-2 implementer, 2026-07-27)

**Status:** DONE
**Commit:** `ae24589850002136f074f9a5598a04206938bf2b` on `plan75-a` - `e2e/help-mode.spec.ts` only, +57/-3, unsigned (`%GG` empty), repo trailer, staged by path.
**Inputs read:** `.superpowers/sdd/plan-7.5/task-2-verdict.md`; the AMENDED design from the main tree `/home/senol/Git/Muxsmith/docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md` (case 9 witness extension and amendment-1 log, commits `89782cd` + `e525813`) - read-only from main, every edit in the worktree. No session-relocation tool called; all runs foreground, direct `pnpm exec playwright test` form.

## M1: the false evidence claim, corrected

The three comment lines the verdict quoted are the ONLY deletions in this commit:

```
-    // (D71's fallthrough). The pinned topic is this half's evidence that the
-    // listener actually handled the click, so the unchanged row count is a
-    // suppression rather than an event that never arrived.
```

Replaced by a statement of what the assertion actually checks: topic IDENTITY only, explicitly not evidence that the listener ran, because `pinnedId ?? hoverId ?? VIEW_TOPICS[activeView]` resolves to `view-editor` in this view whether the click pinned it, the pointer merely hovered it, or the listener never saw it - and the row count is what carries the suppression evidence. No evidence claim remains anywhere in the half.

## M2: the witness, implemented as amended

`probeEnterKeydown(page)` added at module level beside `attemptDrag`, with the amendment's body verbatim (synthetic `KeyboardEvent("keydown", { key: "Enter", bubbles: true, cancelable: true })` dispatched at `[data-testid="editor-rule-add"]`, `defaultPrevented` returned synchronously, no rAF). Its doc comment records why the row count cannot attribute the closure, why `cancelable: true` is load-bearing, and why `defaultPrevented` discriminates.

The discrimination premise was re-verified in this worktree rather than borrowed from the amendment: `grep -rn "keydown\|@keyup\|@keydown" src/` returns exactly two lines, the `addEventListener`/`removeEventListener` pair for `onHelpKeydown` in `src/App.vue`. It is the only keydown listener in `src/`.

Three assertions added, none changed:

- Outside help mode, after the Enter mutation control: `expect(await probeEnterKeydown(page)).toBe(false)` (the paired absence control), then `await expect(rows).toHaveCount(3)` - the probe's own side effect tested rather than assumed (an untrusted event triggers no activation behavior).
- Inside help mode, after the landed Enter suppression assertions: `expect(await probeEnterKeydown(page)).toBe(true)` - the witness, with the pin side effect recorded in the comment (already pinned by the click half; no later assertion reads pin state).

## Acceptance fire-test (mandatory), real outputs

Both neutralizations applied to `src/App.vue` only, each from a `command cp -f` restore, each followed by `pnpm exec vite build` (`vite preview` serves `dist/`), each run over the WHOLE file so "every previously landed assertion stays green" is measured against the coverage set, not just the case under test.

| Round | Mutation | Result |
| --- | --- | --- |
| B shape | `preventDefault` + `stopPropagation` removed from `onHelpKeydown`'s Enter/Space branch, pin kept | `1 failed, 8 passed` - the only failure is case 9 at `e2e/help-mode.spec.ts:305`, `Expected: true / Received: false`, i.e. the witness line |
| B2 shape | the ENTIRE Enter/Space branch removed (suppression and pin) | `1 failed, 8 passed` - identical: case 9 at `:305`, `Expected: true / Received: false` |

Because the witness is the last assertion in the case and Playwright aborts at the first failure, a failure at `:305` means every previously landed assertion in case 9 passed in both rounds; the other 8 cases in the file passed explicitly. Under B2 the pre-existing D54 keyboard case still passes - the M2 gap the verdict proved - so the witness is now the only thing in the repo that observes that branch's death.

**Restore verified:** `command cp -f` from backup, then `git diff --exit-code src/App.vue` clean and `md5sum src/App.vue` identical to the pre-fix-round backup (`c520a02f4c215ae86130ddb504d57f02`). `dist/` rebuilt from restored source and `pnpm exec playwright test help-mode` re-run: **9 passed**.

## Runs (foreground)

| Command | Result |
| --- | --- |
| `pnpm lint` | clean |
| `pnpm exec tsc --noEmit -p e2e/tsconfig.json` | clean (the `!` non-null assertion the amendment specifies is accepted by this eslint config; the amendment's code landed verbatim) |
| `pnpm exec playwright test --grep "both activation channels are suppressed"` | `1 passed (712ms)`, selection exactly one test |
| `pnpm exec playwright test help-mode` | `9 passed (893ms)` |
| `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e` | all green; `check-i18n: ok (... 211 catalog ids, ... 22 help id(s) x 2 help locale(s) ...)` unchanged; `62 passed` |

## Additive-only accounting

`git diff --numstat e2e/help-mode.spec.ts` against `92ba1e7`: **`57  3`**. The deletion column is non-zero by design this round - M1 required rewriting three comment lines - and the accounting is exact:

- Every deleted line is quoted above; all three are comment text.
- `git diff -U0 ... | grep '^-' | grep -E 'expect|await '` returns nothing, and that grep was fire-verified by piping a real assertion-deletion line through the same pattern (it fires).
- Stronger positive check, since the negative alone is weak: the `expect(` line sets of `HEAD:e2e/help-mode.spec.ts` and the working file diff as **additions only** - `+ expect(await probeEnterKeydown(page)).toBe(false)`, `+ await expect(rows).toHaveCount(3)`, `+ expect(await probeEnterKeydown(page)).toBe(true)`. No landed assertion was modified or removed, matching the amendment's "landed contract assertions stay exactly as landed".

## Scope and self-review

Still zero production code, zero new surface (one test file; `check:i18n` reports the unchanged 22 help ids x 2 locales and 211 catalog ids). No new dependency. Typography: `LC_ALL=C grep '[^ -~]'` over the added lines empty, pattern positively controlled earlier. Comments anchor on symbol names, no `file:line` spans. Nothing beyond the amendment's enumerated extension was added; the L-series findings (L1 brief filter form, L3 ledger tier label) are controller-side and untouched here.

The verdict's Nits item is worth flagging as still open: the doc comment's positional cross-reference "in the I1 sibling's shape below" is unchanged, and this round inserted a module-level helper above the describe block, which does not affect that phrase's target (the sibling is still below and still named by its describe title). Left as landed rather than silently rewording committed prose the verdict classified as a nit.
