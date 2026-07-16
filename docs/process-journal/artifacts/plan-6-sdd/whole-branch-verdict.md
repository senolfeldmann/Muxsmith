# Whole-branch verdict, waves 3+4 (extracted from the reviewer transcript; fable, resumed twice across 529s)

### Merge verdict
**READY-WITH-FIX-WAVE** — one fixer, eight mechanical items (list below). Two spec-completeness findings additionally need **owner routing at plan close** (they are user-visible-surface decisions a fixer cannot take, per the house's own `proc-latitude` routing precedent recorded in `editor-generic-action-keys` occurrence 1); they do not block the merge because they are absent additions, not defects in what is built, and the house has an established channel for exactly this class (the 13b gap rode it).

### Spec completeness (dimension 1)
Walked spec 8.2 view 1 (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:374`) clause by clause against the as-built surface:

| 8.2 promise | Status |
|---|---|
| track-rule grid (order, source, match summary, changes, optional) | Built (`src/views/EditorView.vue:392-450`). Columns: source/match/optional/changes; "order" is carried by row position only, no ordinal column — cosmetic, noted for the Plan-7 comfort pass. |
| drag to reorder | Built, semantic model edit, selection cleared on reorder. |
| detail editor per rule | Built (T13b), `SectionWidget` over `trackRule`, same code path as `attachments.rules`. |
| panels for attachments/chapters/tags/title | Built via registry dispatch; meta/input/output additionally exposed — that is D45's recorded exposing-all-42 decision (design `:783-800`), not over-building. |
| open/save YAML, canonical-save note | Built (T13); `editor-save-note` names the whole behavior (comments, key order, formatting, default omission — D41+D48). |
| **recent profiles** | **ABSENT and unrouted** (Finding 1). Editor has only a pick button; `pickAndOpen` also never feeds the shared `recent_profiles` memory BatchView maintains (`src/views/BatchView.vue:93`). Zero hits for "recent" in the plan; no ROADMAP item; design cites the clause only inside an unrelated argument (`:786`). |
| **inline validation markers** | **Partially satisfied, unrouted** (Finding 2). Diagnostics render live in a panel (`DiagnosticsPanel`, third consumer) and Save gates on errors — spec 7's load-bearing halves — but nothing anchors a diagnostic to its field despite `config_path` being available. No design/plan text scopes "inline markers" anywhere. |

Spec 7 wiring: clean. Frontend renders core diagnostics, Save-disable is exactly the sanctioned affordance (`EditorView.vue:120-124`), zero frontend semantic validation (StringListWidget split/trim and PropertyMapWidget `Number()` coercion are structural typing per core-generated tables, sanctioned by Ruling 2). Validate-on-edit watcher with generation guard against out-of-order responses is correct.

### Cross-task seams (dimension 2)
- **Registry → dispatcher → widget → view chain (T9→T13b):** all 10 `FieldWidget` variants are dispatch-reachable in the real editor: text/bool/stringList (input), directoryPath/select/keywordOrBlock (output), optionalFlag (locator.match_to_source via chapters' always-present block), propertyMap+list (matchExpr/changes via the detail panel and `attachments.rules`), section (everywhere). No orphaned variant; the never-arm dispatcher plus `registryByName: Record<RegistryName, …>` closes both directions.
- **Typed-cell chain (12a → 12 → 13b → 14):** traced end to end. An applied `StructuredEdit` carries the typed `Scalar` (D49) into `match.exact`; canonical save and reopen land it in the detail panel's `propertyMap(matchable, scalar)`; `cellKindFor` resolves via the emitted `MATCHABLE_TYPES` (including the appended `codec_kind` virtual) and renders checkbox/number/number-step-any/text with real typed values (`:checked="value === true"`, `Number()` on input). Unknown/`raw:` keys fall back to text before the switch — core diagnoses. The chain holds.
- **Mount-harness vs real-app duality:** deliberately near-disjoint (widgets/grid/sections/typed-cells/panel mount-only; nav/open/save/gating/apply real-app-only), and the T13 review-check held (no mock injected, watcher gated on `currentPath`). One overlap gap with teeth: **the populated grid and detail panel never render in the served-app path at all** — T13's fixture has `rules: []` (`e2e/smoke.spec.ts:1115`). This is the same hole as agenda-a and closes with the same wave task.
- **Cross-view coherence after apply** (batch saves while the editor may hold the same file's model): confirmed already routed — ROADMAP carries the apply-vs-editor concurrency guard and auto-refresh candidates (`docs/ROADMAP.md:578,584`).

### Agenda triage (dimension 3)
**a. Axe gap — fix now (wave task 1).** Extend the T13 real-app test: populated two-rule fixture, select a row, `assertNoSeriousA11yViolations` with grid + selection button + panel rendered. One fixture edit plus ~5 lines; it simultaneously gives the served app its first render of the grid/panel (the duality gap above) and satisfies `a11y-claims-need-witnessed-scans`. Recording it as accepted limitation would leave the editor's principal interaction surface permanently unscanned for the cost of avoiding a trivial edit — wrong trade.

**b. Minors:**
| Item | Triage |
|---|---|
| T9 de-header depth (`locales/de/gui-editor.ftl:1-8`, `##` where en uses `#`) | fold — wave 7 |
| T10 multiline assertion breadth (`e2e/smoke.spec.ts:767-777`, role-only, passes on `<input>` too) | fold — wave 2 |
| T10 PropertyMapWidget dup-key collapse comment (`PropertyMapWidget.vue:89,95,117`) | fold — wave 4 |
| T10 ListWidget dragIndex stale on aborted drag (`ListWidget.vue:52-70`; EditorView same pattern `:323-345`) | fold — wave 3 |
| T13 redundant post-Open validate round-trip (`EditorView.vue:145-160`) | **accept**: one local IPC call, spec 7's "every profile edit" arguably covers the load, and fixing it perturbs a green test queue for no user-visible gain |
| T14 silent no-op on contract violation (`BatchView.vue:245-252`) | fold — wave 6, defensive `console.error` only; a user-visible fallback string is the owner's pending surface |
| T14 non-clicked apply buttons visually enabled in flight (`SuggestionCard.vue:86-87`) | fold — wave 5 |
| T6 stale line-refs (`planner.rs:1917-1918` cites `:1824, :1829`; actual `track_name` inserts at `:1837, :1843` — measured) | fold — wave 8; misleading citations in a house that leans on `:NNN` refs are worth one comment edit |

**c. Rendered-surface strings:** the pending owner set is confirmed complete as scoped — six reused keys (`batch-profile-pick`, `batch-profile-current`, `batch-profile-filter-name`, `settings-save`, `batch-diagnostics-heading` in EditorView; `batch-profile-heading` as the nav tab), the two apply keys, 45 editor keys (grep-counted 45/45 en/de), the nav-tab question. Two additions for the same pass: (i) the grid's summary **notation** (`key=value`, `key~value`, `~/…/`, `any(n)`, `not(n)` in `EditorView.vue:258-278`) — profile-token-derived per the in-file rationale, but it is app-authored notation on a rendered surface the owner has not seen; (ii) if the recents routing lands option (a), `batch-recents-heading` becomes a seventh reused key. Nothing else user-visible slipped: all other template text is `$t()`-keyed, widget option tokens are raw domain values by D45's rule.

### Gate honesty (dimension 4)
Clean. The branch is strictly additive on tests: +735 lines in `e2e/smoke.spec.ts`, 40 in `ts_export.rs`; the diff's 60 deleted lines contain zero `expect`/`test(`/`assert` occurrences (extraction method positively controlled); no `.only`/`.skip`/`.fixme`/`#[ignore]` anywhere in `e2e/` or `crates/`. No pre-existing fixture mutated — the T14 test builds its own distinct fixtures (`PROFILE_PATH` vs `SUGGESTION_CONFIG_PATH`, per amendment 4's echo-mock lesson) and leaves the copy-only fixture untouched. The paired-control pattern (apply button present on a `Suggestion`, absent on the no-fix diagnostic, identical selector) is the falsifiability template applied correctly.

### House conformance (dimension 5)
- `gui-table-caption`: the one new table carries a visible caption (`EditorView.vue:396-398`); the entry's "exceptionless" claim holds post-35d844d.
- `gui-closed-domain-dropdowns`: KEEP_DROP/COLLISION_POLICIES/keyword domains all render as selects; `type`/`codec_kind` text cells conform to the recorded Plan-7 deferral.
- `editor-generic-action-keys`: exactly implemented — ListWidget/PropertyMapWidget render the two generic keys; `editor-attachment-rule-add/-drop` caption only their registry fields; budget 45/45 en/de verified by count.
- Typography: banned-glyph scan over every added line: zero hits (pattern validated against a known-present control first). De catalog uses real umlauts correctly.

### Findings (ranked)
1. **HIGH (spec completeness) — spec 8.2 "recent profiles" absent from the editor and unrouted.** `src/views/EditorView.vue:163-189` (no recents UI, no recents write); zero plan/design/ROADMAP routing. Two spec clauses surviving 13 task reviews unrouted is the exact blind spot this review exists for. **Fix: owner routing at plan close**, options enumerated: (a) a 13c-style amendment task — render `AppSettings.recent_profiles` in the editor, extract `rememberRecentProfile` (`BatchView.vue:93-106`) to a shared module, remember on editor open, reuse `batch-recents-heading` (joins the pending string pass); or (b) record the batch view's recents as satisfying 8.2 and amend spec 8.2 accordingly. My recommendation: (a) — the plumbing exists, it is task-sized, and (b) leaves editor opens invisible to the recents memory, a real UX asymmetry.
2. **MODERATE (spec completeness) — spec 8.2 "inline validation markers" built as a panel, per-field anchoring unrouted.** `EditorView.vue:370-375`. Diagnostics carry `config_path`; nothing maps it to a field. **Fix: owner sign-off recording panel-rendering as the Plan-6 shape, with field-anchored markers routed to Plan 7** (the comfort/tooltip/help-id pass is the natural home — the mapping is registry work, a feature not a fix).
3. **MODERATE — agenda-a axe gap** (see triage a): `e2e/smoke.spec.ts:1115` fixture empty; no scan reaches grid/selection/panel. Fix: wave 1.
4. **LOW — stale `dragIndex` on aborted drag**, `ListWidget.vue:52-70` (early-return branch keeps it; no `dragend` reset) and `EditorView.vue:323-345` (no `dragend` reset): a drag entering from outside the list can pair a stray drop with a stale index. Fix: `@dragend` reset in both, wave 3.
5. **LOW — apply buttons on non-clicked cards visually enabled during an in-flight apply**, `SuggestionCard.vue:86` / `BatchView.vue` card bindings: functionally guarded by the `busy` early return, visually violating the established busy idiom. Fix: pass `busy` down as `disabled`, keep `aria-busy` clicked-only, wave 5.
6. **LOW — silent no-op when `load_profile` returns `profile: null` with empty diagnostics**, `BatchView.vue:245-252`: contract violation per D42's envelope, currently invisible. Fix: `console.error` branch, wave 6.
7. **LOW — multiline widget test passes against a single-line input**, `e2e/smoke.spec.ts:767-777`. Fix: assert `TEXTAREA`, wave 2.
8. **LOW — three doc/comment items**: dup-key collapse comment (`PropertyMapWidget.vue`), de header depth (`gui-editor.ftl:1-8`), planner.rs refs `:1824/:1829` → `:1837/:1843`. Waves 4, 7, 8.

### Fix-wave list (single fixer, complete)
1. `e2e/smoke.spec.ts` (Task-13 describe): populate the open/save fixture with two rules, select a row, assert the detail panel, run `assertNoSeriousA11yViolations` on that state.
2. `e2e/smoke.spec.ts:767-777`: multiline spec additionally asserts the element is a `<textarea>`.
3. `src/editor/widgets/ListWidget.vue` + `src/views/EditorView.vue`: reset `dragIndex` on `dragend` (and in ListWidget's early-return drop branch).
4. `src/editor/widgets/PropertyMapWidget.vue`: comment documenting the `Object.fromEntries` duplicate-key last-write-wins collapse (core diagnoses duplicates).
5. `src/components/SuggestionCard.vue` + `src/views/BatchView.vue`: apply button disabled while any batch action is in flight (`busy` passed down), `aria-busy` stays clicked-card-only; adjust the T14 assertion if it pins the enabled state.
6. `src/views/BatchView.vue:245-252`: `console.error` on the null-profile/no-diagnostic contract violation.
7. `locales/de/gui-editor.ftl:1-8`: opening comment block `##` → `#` to match en's depth.
8. `crates/muxsmith-core/src/planner.rs:1918`: correct `:1824, :1829` → `:1837, :1843` (or drop the numbers for "the two substring arms").

Separately, **two owner routings at plan close** (not fixer work): findings 1 and 2, plus the summary-notation addendum to the pending string pass.

### HARVEST
- **spec-clause-sweep-at-plan-close**: a plan implementing a spec section must carry a clause-by-clause checklist of that section; every clause gets a task, an amendment, or a recorded deferral. Plan 6 cited 8.2 selectively; the one gap with a ledger entry (`registry-slot-capability-delta` → 13b) got caught, the two without entries (recents, inline markers) survived 13 task reviews unrouted. The ledger entry was the only difference.
- **fixture-reachable-states-need-one-served-render**: when a view's populated states are covered only through a mount harness, the served-app path silently ships states no gate ever rendered (here: grid-with-rows + detail panel — which is also why the axe gap existed). One real-app render per composed state class; extends `a11y-claims-need-witnessed-scans` from scans to rendering generally.


---

# Post-verdict delta re-review (resumed reviewer): fix wave 6674089 + task 13c ef19fd9

**Branch: READY**

**(a) Eight items — all resolved as specified.** Spot-checks on the load-bearing three: **Item 1** — T13 served fixture populated with two rules (`e2e/smoke.spec.ts:1113-1121`), row selected, all four panel fields asserted, `assertNoSeriousA11yViolations` run on the composed grid+selection+panel state, correctly placed before the validate-queue sequence with the recorded proof that selection consumes no queue entry; the original post-error scan is retained, strictly additive. **Item 5** — `SuggestionCard.vue` gains `busy?: boolean`, `:disabled="busy"` while `aria-busy` stays `applying` (clicked-card-only, the right split), `BatchView` passes `:busy="busy"`; no assertion adjustment was needed since no test pinned the enabled state mid-flight — the conditional in my wave item is correctly a no-op. **Item 3** — `onDragEnd` reset in both `ListWidget.vue` (plus the early-return branch reset in `onDrop`) and `EditorView.vue`, wired via `@dragend` on both drag surfaces. Items 2 (tagName `TEXTAREA` pin), 4 (last-write-wins comment with the spec-7 rationale), 6 (`console.error` else-branch, no user-visible string), 7 (`##`→`#`), 8 (line numbers replaced by the arm description — my offered alternative) all land exactly as mandated.

**(b) Ninth fix — correct, and zero new strings verified.** The defect (key/value inputs with no accessible name since T10/T12, axe `label`/critical, two nodes) is exactly the class the fixture-reachable-states rule predicted, surfaced by item 1's first served render of a populated `exact` map. The fix is the right mechanism: `useId()` + `aria-labelledby` to the existing `$t(spec.labelKey)` legend for the key input, legend + the row's own key-input id for the value cell (the WAI-ARIA name computation including the referenced textbox's live value is correctly used — rows self-distinguish by user-typed data). Grep-verified zero catalog additions in the delta (the only `.ftl` change is item 7's comment markers); all added template attributes are id references. NEEDS_CONTEXT stop instead of keyboard scope-creep was the correct process move; red-to-green captured in the fix-wave report.

**(c) 13c matches the option-(a) sketch fully.** Shared `src/recentProfiles.ts` with the never-clobber round trip; BatchView re-point is genuinely behavior-identical (`settings.value = (await rememberRecentProfile(path)) ?? settings.value` reproduces both the success and swallowed-failure paths; `updateSettings`/`persistDir`/template untouched); the editor **feeds** the memory on every open and **renders** it pre-Open, both through one `openPath` funnel shared with the dialog pick — the UX asymmetry (editor opens invisible to recents) is gone. Zero new keys; `batch-recents-heading` correctly enumerated as the seventh reused key in the pending owner string pass, with the duplicate-DOM-id fork (v-show keeps BatchView's `#batch-recents-heading` live) explicitly closed via the distinct `editor-recents-heading` element id. Tests follow the house disciplines: distinct fixture paths (`echo-mock-distinct-fixture-values`), paired absence control on the same selector, echo asserted on the recorded `set_settings` write rather than a UI reflection.

**(d) Nothing weakened, nothing reopened.** Zero `.only`/`.skip`/`.fixme` in the delta; the single deleted line matching an assertion keyword is a doc-comment fragment in `PropertyMapWidget.vue`'s rework (`diff:801`), not a test; all e2e changes are additive; my accepted non-findings (redundant post-Open validate, ordinal column, summary notation) stay untouched; Finding 2's owner ruling 2a is properly recorded as a written Plan-7 ROADMAP item (`docs/ROADMAP.md:72-77`) plus the ledger occurrence, and Amendment 5 correctly declares itself a post-verdict delta that leaves the wave structure intact.

**(e) Verdict resolves to READY.** The fix wave landed complete-plus-one, both owner routings are closed (1a built and tested, 2a recorded), and the two harvest patterns are in the ledger with the ninth fix already standing as `fixture-reachable-states`' first confirmed catch. Two non-blocking observations, both acceptable as-is: the editor renders recents only in the pre-Open empty state (recorded rationale in Amendment 5; BatchView shows them always — a deliberate scope choice, not an asymmetry in the memory feed), and the T13c tests leave `validate_profile_model`/`set_settings` unmocked in some scenarios, relying on the components' documented background tolerance — additive and green, but worth remembering if a future test starts asserting on diagnostics after a recents-driven open. No new findings at any blocking severity.
