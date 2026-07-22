# Task 13 verdict - D54: the annotated set

**Reviewer:** independent SDD task reviewer (Opus). **Commit:** 97f707f (branch plan7-f, parent 906260b = approved T12). **Worktree:** `/home/senol/Git/Muxsmith/.worktrees/plan7-f`.

## VERDICT: APPROVED

The T13 deliverable is correct and complete against the plan's Task 13, the Global Constraints, and D54/D52. No implementation defect. One design-level gap surfaced by T13's annotations is **routed to the controller** (finding F2 / Q2b) - it is a D52 mechanics defect, not a T13 fix owed, and the implementer correctly flagged rather than patched it (progress.md: "stale-hoverId observation -> reviewer Q2"; Global Constraint 16 honored). One non-blocking comment-accuracy nit (F1 / Q1).

All checks below were run foreground; the verdict file is the only write; every probe (a broken-helpId control in scratchpad, a neutered `onHelpClick`, five throwaway `e2e/zz-focusin-probe.spec.ts` specs) was removed/restored and the tree confirmed byte-identical clean (`App.vue` sha `4478d8a...` unchanged via `command cp -f` + `cmp`).

---

## Dimension 1 - the closed set, recounted

- **18 registry helpIds, each literal, each === its labelKey.** Ran the programmatic check myself over `src/editor/registries.ts`: 18 `helpId:` literals, 0 mismatches against the nearest-preceding `labelKey` (all in-object, distance <= 1), 0 `helpId: labelKey` shorthand occurrences. The 18 map exactly onto the plan's Step-2 table / D54 editor-controls table (`editor-input-pattern`, `-input-extensions`, `-output-filename`, `-output-on-collision`, `-template-block-template`, `-track-rule-source`, `-track-rule-match-expr`, `-track-rule-optional`, `-track-rule-changes`, `-match-expr-exact`, `-locator-match-to-source`, `-locator-match-pattern`, `-tracks-unmatched`, `-tracks-rules`, `-attachments-unmatched`, `-attachments-rules`, `-profile-chapters`, `-profile-title`).
- **24 excluded entries carry nothing.** `grep -c labelKey: registries.ts` = 42 (= 18 + 24); `grep -c helpId:` = 18. The other 24 fields have no `helpId`.
- **5 hand-written template literals, exactly the plan's set, on the specified hosts:** `view-batch` (BatchView root `<section>`), `view-jobs` (JobsView root), `view-editor` (EditorView root), `editor-tracks-rules` (the grid `<caption>` - the plan named `<caption>`, not the sibling `<h2>`; caption is what shipped), `batch-suggestion-card` (SuggestionCard root `<article>`). Plus the dispatcher fallthrough `:data-help-id="spec.helpId"` on the `<component>` element.
- **No other `data-help-id` anywhere.** `grep -rn data-help-id src/` yields only the 5 literals + the dispatcher binding; the remaining hits are T12-landed selectors (`App.vue:54/55/118`) and a `style.css` comment - none are annotations.
- **Fire-verify (dimension 5):** copied `registries.ts` to scratchpad, mutated one `helpId` to `editor-WRONG-pattern`, re-ran the closed-set check -> it reported `count:18 mismatches:1 editor-WRONG-pattern!=editor-input-pattern`. The check fires; the real file is clean.

## Dimension 2 - behavioral, run it myself

- **`--grep "help mode"` = exactly 6 tests** (`npx playwright test --grep "help mode" --list`): 2 in `help mode (D52)` (toggle/nav/suppress/Esc; dialog-Esc) + 4 in `help mode annotations (D54)` (card hover/pin/apply-suppress; focusin/Enter; view-switch pin-clear; editor pattern-widget fallthrough).
- **Full suite = 40 passed** (`pnpm build` green, then the e2e run). Note: `pnpm test:e2e -- --grep ...` does NOT forward `--grep` to playwright (the `sh -c "A && B && C"` script swallows trailing args as positionals), so that invocation ran the whole suite - which is how I confirmed the 40 count directly; the grep-6 count came from `npx playwright test` invoked directly.
- **The apply-in-card suppression assertion genuinely exercises E3 - fire-verified differentially.** Backed up `App.vue`, removed `event.preventDefault()`/`stopPropagation()` from `onHelpClick`, rebuilt, re-ran `help-mode.spec.ts:194`: it **failed** at line 226 (`load_profile` fired -> `expect(false)` received `true`). The neighbouring positive control (`smoke.spec.ts:406`) proves the apply button is wired to `load_profile -> apply_suggestion -> save_profile` in the non-help path, so the help-mode assertion is a real differential, not vacuous. Restored `App.vue` byte-identical.
- `pnpm lint` clean; `pnpm check:i18n` -> `ok`; `pnpm build` (incl. `vue-tsc --noEmit`) green. Rust gates unaffected (7 frontend-only files).

## Dimension 3 - adjudications

### Q1 - synthetic `dispatchEvent("focusin")`: SOUND technique, not a material weakening.

I reproduced the headless behaviour with throwaway probes:
- A **real** `copy.focus()` fires a focusin that **does reach a `<main>` capture listener** with `target=batch-suggestion-copy, closest=batch-suggestion-card` (probe1: count 0->1; probe3: logged). So the implementer's stated *mechanism* - "no focusin reaches the listener" - is **imprecise/incorrect**.
- Yet a real `copy.focus()` **does not drive the app's hover state**: sidebar stays `view-batch` and the card gets no `help-hover` class (probe2, probe4 `realFocus=false`). A **synthetic** `dispatchEvent("focusin")` **does** (`synthetic=true`; the D54 test passes), and a **real mouseover** does (`realHover=true`).
- So the implementer's *operational conclusion* - real focus cannot exercise this path in this harness, synthetic is required and reaches the same capture listener the a11y contract targets - is **correct**. The residual asymmetry (a real-focus focusin reaches a raw `<main>` listener but not the app's effect) is a headless/Playwright artifact; in a production webview `focusin`-on-focus is standard DOM and the same handler that answers synthetic focusin and real mouseover answers it. The synthetic technique validly verifies the app's `focusin -> topic` wiring and is the only viable option here. **Non-blocking nit (F1):** the code/test comment's mechanism wording should be corrected to "a focusin reaches `<main>` but real programmatic focus does not drive the delegated handler in this harness."

### Q2 - stale `hoverId` on view switch.

**(a) The test is honest and does prove pin-cleared-on-switch.** Resolution is `pinnedId ?? hoverId ?? VIEW_TOPICS[activeView]`. The test pins the card, clicks `nav-jobs`, hovers into the Jobs view, then asserts `view-jobs`. A surviving pin (`batch-suggestion-card`) would **outrank** the post-switch hover, so the `view-jobs` assertion can only pass if the pin was cleared. The implementer's argument holds. Hovering into the new view (a realistic pointer path) is legitimate; it refreshes `hoverId` and thereby *masks* the stale window, but proving pin-clear is all this case is charged with.

**(b) DESIGN GAP -> route to controller (NEEDS_CONTEXT), not within the shipped semantics.** I read D52's exact clear-condition enumeration: *"Switching views clears `pinnedId` ... and the sidebar falls to the new view's topic."* Only `pinnedId` is cleared (confirmed in `App.vue:107-109`, `watch(activeView, () => { pinnedId.value = null; })`). Once T13 annotates the view roots, `event.target.closest("[data-help-id]")` never returns null inside a view, so `hoverId` is non-null and **retained** across a nav switch (nav tabs live outside `<main>`, so switching fires no `<main>` mouseover to refresh it). **Reproduced (probe5):** hover the card (no pin), click `nav-jobs`, do not move the pointer -> the sidebar still shows `batch-suggestion-card` (STALE), not `view-jobs`. D52's asserted outcome ("falls to the new view's topic") is **not delivered** by its own enumerated mechanics. Candidate fix: a one-line `hoverId.value = null` in the `watch(activeView)` alongside the `pinnedId` clear, in D52's mechanics - **design-silent today**. This is a D52/controller decision, out of T13's scope, and must not be decided at the keyboard (Global Constraint 16). T13's annotations merely make the latent D52 inconsistency observable; the implementer routed it correctly. **Route with a decision memo.**

### Q3 - dual placement for `tracksFields.rules`: CORRECT and plan-specified.

Two placements, verified against the plan text:
- **Registry literal** `helpId: "editor-tracks-rules"` on `tracksFields.rules` (Step-2 table) - for D62's registry-literal scan.
- **Hand-written** `data-help-id="editor-tracks-rules"` on the grid `<caption>` (Step-4) - the DOM anchor, needed because `tracks.rules` renders through the **bespoke grid**, which bypasses `FieldWidgetDispatcher` (so the registry `helpId` never reaches the DOM via fallthrough).

**No duplicate DOM anchor:** `topLevelFields` filters out `"tracks"` (`EditorView.vue:249-252`), so the tracks section is not dispatched; `tracks.unmatched` is dispatched separately (its own `editor-tracks-unmatched`), `tracks.rules` is the grid; the detail panel calls `SectionWidget` directly with `ruleDetailSpec` (labelKey `editor-tracks-rules`, **no** `helpId`), which emits no attribute. `editor-tracks-rules` has exactly one host element (the caption). The detail-panel's child widgets correctly carry the included #6-9 track-rule helpIds via the dispatcher.

## Dimension 4 - quality

- **Diff scope = 7 files** (`git diff --stat 906260b HEAD`): the 6 named sources + `e2e/help-mode.spec.ts`. Matches the plan exactly.
- **No catalog / topic / doc changes** in the commit (no `locales/`, `.ftl`, `help/`, `.md`). Correct - T13 is annotation-only; content landed in T8-T10.
- **Commit discipline:** message verbatim per Step 6; `Co-Authored-By: Claude Fable 5` trailer present; unsigned; files staged explicitly (no `git add -A` evidence).
- **Plan-premise spot-checks:** `helpId?: string` (T4's produced field) present at `src/editor/fieldSpec.ts:15`. Single-root-widget premise: `TextWidget.vue` has a single `<div>` root, so the dispatcher fallthrough lands the attribute there - confirmed by inspection *and* empirically by the passing editor test, which locates `[data-help-id="editor-input-pattern"]` (a TextWidget) and hovers it to swap the sidebar.

## Dimension 5 - fire-verify

Done twice against controls: (1) the closed-set `helpId===labelKey` check fired on a scratchpad copy with one mutated literal; (2) the E3 apply-suppression assertion fired when `onHelpClick`'s suppression was removed. Both restored. Additionally reproduced the Q1 headless-focus limitation and the Q2 stale-hover window rather than trusting the borrowed claims.

---

## Findings

1. **F1 (non-blocking, HARVEST) - Q1 comment mechanism wording.** The justification comment in `help-mode.spec.ts` (~240-245) states no focusin reaches the listener under real focus; reproduction shows a focusin *does* reach a `<main>` capture listener, it simply does not drive the app's delegated handler under programmatic focus in this harness. Conclusion and technique are correct; only the wording is off. Suggest the implementer tighten it. Not owed as a fix for approval.

2. **F2 (ROUTED to controller, NEEDS_CONTEXT - not a T13 fix).** D52 stale-`hoverId`-on-view-switch. Reproduced. D52 enumerates only `pinnedId` clearing on view switch yet asserts the sidebar falls to the new view's topic; with view roots now annotated, `hoverId` is non-null and retained across a nav switch, so the sidebar shows the previous view's last hovered topic until the pointer re-enters `<main>`. Candidate one-line fix: clear `hoverId` in `watch(activeView)`. Design-silent; requires a controller/D52 decision. Decision memo owed by the controller before the whole-branch review, per Global Constraint 16.

## HARVEST

- **Fire-verify negatives by breaking them once.** Two absence/negative checks here were only trustworthy after being made to fire against a control (closed-set check; E3 suppression). The pattern generalizes: a green suppression test or an empty grep proves nothing until you watch it go red.
- **Harness fact for this repo's e2e:** in Playwright/headless-Chromium against the built app, programmatic `.focus()`/`Tab` does **not** drive `focusin`-delegated app state even though a `focusin` reaches a `<main>` capture listener; use synthetic `dispatchEvent("focusin")` (or real mouseover) to exercise focus-delegation. Relevant to any future a11y/focus test.
- **Tooling gotcha:** `pnpm <script> -- --grep X` does not forward `--grep` to a `&&`-chained script's last command (the outer `sh -c` takes it as a positional). Invoke `npx playwright test --grep` directly when filtering.
- **Over-restriction watch.** T13 itself introduces no over-broad ban: the "18 in / 24 nothing" split is an enumerated closed set backed by D62's completeness gate - correctly scoped. The watch item is on the *routed* F2 fix: if adopted, clear `hoverId` **only** on the enumerated view-switch trigger, not by broadening to "clear on any unannotated hover," which would break the pinned-branch fallback the D54 amendment depends on. Enumerate the trigger; do not ban the category.
