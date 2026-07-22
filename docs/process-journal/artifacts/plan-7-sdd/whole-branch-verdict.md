# Plan 7 whole-branch review verdict

Reviewer: whole-branch (top model), fresh session. Scope: 0fea107..cc0e6d7
(71 commits, all 21 tasks merged), graded against the amended v1 spec
(authoritative), the plan-7 design incl. round-7/8 amendments, and the
Tier-2 files (product-boundaries / conventions / process-conventions).
Gate run, probes, and all findings below were produced under this
reviewer's own build on the merged master at cc0e6d7; every probe edit was
restored byte-identically (verified with `cmp` after each probe, final
`check:i18n` green re-run).

## Verdict

**NOT READY** - blocking set: **I1** (help-mode suppression vs amended
spec 8.3: one-line `dragstart` suppression fix plus an owner wording
ruling on the keyboard residual). Everything else is non-blocking:
M1-M6 ride two existing dispatch shapes (a comment sweep, the queued
plan-close design one-liner batch), the triage items are ruled below,
and the owner rendered-surface pass is already the standing gate for
the content items.

## Gate run (nine parts, foreground, this reviewer's build)

| # | Gate | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | GREEN |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | GREEN |
| 3 | `cargo test --workspace` | GREEN - exit 0, 494 passed / 0 failed, no mkvmerge self-skips (real v100 present) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | GREEN |
| 5 | `cargo deny check` | GREEN (advisories/bans/licenses/sources ok) |
| 6 | `pnpm lint` | GREEN (exit 0) |
| 7 | `pnpm build` (vue-tsc --noEmit && vite build) | GREEN |
| 8 | `pnpm check:i18n` | GREEN - "211 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)" |
| 9 | `pnpm test:e2e` | GREEN - 52 passed |

Auxiliary: `cargo test -p muxsmith-core --features ts --test ts_export`
re-emits `src/bindings/settables.ts` with zero diff (D58 committed
generated artifact in sync); `marked` pinned exact `18.0.7`, sole
lockfile addition, zero transitive deps (`marked@18.0.7: {}` in
pnpm-lock.yaml).

### Gate fire-probes (falsifiability of the new checks, all restored)

The green gate is an absence result; each new check was made to fire once:

- **D62 check 5 (zero-pipe)**: appended `a | b` to `help/en/view-batch.md`
  -> `help/en/view-batch.md:21: contains a table/pipe character (banned,
  D62 check 5)`. RED confirmed, restored.
- **D55 rule 5 (placeable parity)**: renamed `$index` -> `$idx` in
  `locales/de/gui-common.ftl` -> `"apply-rule-index-out-of-range" value:
  placeable set differs from en ({index,rules} vs {idx,rules})`. RED
  confirmed, restored.
- **D55 rules 3+4 (tooltip completeness + attr parity)**: removed the
  `.tooltip` under `editor-input-pattern` in en ->
  both `labelKey "editor-input-pattern" has no .tooltip attribute` and
  the de attribute-set parity error fired. RED confirmed, restored.
- **D61 (IpcError presence)**: inserted `IpcError::new("zz-probe-missing-code")`
  into `src-tauri/src/error.rs` above `#[cfg(test)]` -> gate names code
  and site. RED confirmed, restored. Two probe by-products, both
  benign-direction: (a) a line appended BELOW `#[cfg(test)]` is correctly
  ignored (the documented cutoff behaves as written); (b) the scan matches
  a *commented-out* `IpcError::new("...")` line (my probe was a comment) -
  false-RED direction only, consistent with the line-based charter,
  recorded under watch items.

Post-sweep invariants re-verified with positive-control greps:
`cargo_bin("muxsmith")` only in `tests/support/mod.rs` (:90 funnel, :110
bare); `muxsmith_bare()` has exactly the two ruled callers
(`cli_schema.rs:7`, `:27`); exactly one `v-html` site
(`src/components/HelpSidebar.vue:16`); `createFluentVue` only in
`src/i18n/fluent.ts` plus the exempt e2e `mount-entry.ts` harness (T7 H4);
`IpcError.params` typed `string | number` (`src/ipc.ts:22`) while
`Diagnostic.params` stays `Record<string, string>` (`src/ipc.ts:73`).

## Findings by severity

### Important

**I1 - Help-mode "activation suppressed" does not cover the non-click
mutation channels; shipped behavior contradicts the amended spec 8.3
sentence, and the layers disagree among themselves.**

- The amended spec (8.3, amendment 6(c), owner ruling E3): "While help
  mode is active, control activation inside the main content area is
  suppressed" (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:392-395`).
- Shipped suppression is exactly: capture-phase `click` (all targets) +
  `keydown` Enter/Space on annotated targets + Escape
  (`src/App.vue:60-101`; the only registered listeners are
  `mouseover`/`focusin`/`click` on `<main>` and `keydown` on document,
  `src/App.vue:91-94`). `helpMode` is consumed nowhere else (grep:
  no view or widget gates on it).
- Leak vectors, verified by construction (no live-run repro; modality in
  the completeness note):
  1. **Drag-reorder still works in help mode**: rule-grid rows
     (`src/views/EditorView.vue:559` `draggable="true"`, `onDrop`
     `:416` rebuilds `tracks.rules`) and reorderable list items
     (`src/editor/widgets/ListWidget.vue:109`) - HTML5 drag events never
     pass through the click listener. A drag mutates the in-memory
     profile silently; after exiting help mode a later legitimate Save
     persists it. This is the destructive class the E3 ruling exists to
     close ("safe inspection overlay"), reachable by the accidental
     click-with-movement gesture on a row the user meant to pin.
  2. **Text/number entry still works**: mousedown (not suppressed)
     focuses any input; `@input`-driven model writes fire (e.g.
     `PropertyMapWidget` `onTextInput`, BatchView dir fields
     `src/views/BatchView.vue:401,424`), including the validate IPC
     round-trips.
  3. **Select value change via keyboard**: arrow keys on a focused
     `<select>` fire `change` without any click.
  (Checkbox/button keyboard activation IS covered: their activation
  synthesizes a `click`, which the capture listener eats - verified in
  the e2e suppression tests.)
- The normative layers conflict: the spec sentence is broad
  ("control activation ... suppressed"), D52's mechanics define
  suppression as the capture click listener, and the recorded E3 ruling
  is click-scoped ("no click can start a run, apply a suggestion, or
  save a file" - which shipped behavior satisfies). Per proc-04 the spec
  wins and the conflict is flagged, not improvised around.
- **Fix (blocking half)**: suppress `dragstart` in the same capture set
  (one listener in the `watch(helpMode)` block calling `preventDefault`),
  closing the only leak that can silently reach disk via a later save.
- **Routing (wording half)**: user-visible behavior -> governing human.
  Either extend suppression to the typing/select channels
  (capture-phase `beforeinput` + select-keydown gating) or amend the
  8.3 sentence to its ruled scope ("activation" = clicks, keyboard
  activation, drags; focus and text entry remain live) and record the
  residual. Recommendation: the wording narrowing - typing into a field
  in help mode has no accidental-gesture path (it requires deliberate
  focus + keystrokes) and full input suppression starts fighting the
  a11y focus-equivalence the design mandates.

### Minor

**M1 - Two e2e comments state a mechanism D56 falsified.**
`e2e/mocks.ts:73-74` ("there is no live in-session catalog swap;
`main.ts` resolves the locale once, before mount") and
`e2e/smoke.spec.ts:673-676` ("nothing in the app swaps the live
`FluentBundle`s afterwards -- a saved locale change takes effect on the
next start") are false since `SettingsDialog.save()` calls
`applyLocale` (`src/components/SettingsDialog.vue:64-70`). Both tests
remain valid (they exercise the restart/bootstrap path) and pass; only
the narratives are wrong. Fix: reword both to name the restart-path
scope and point live-switch coverage at `e2e/locale-switch.spec.ts`.

**M2 - Stale "budget 45" in two widget comments.**
`src/editor/widgets/PropertyMapWidget.vue:45` ("catalog budget 45") and
`src/editor/widgets/ListWidget.vue:15` ("budget 45") predate the D59
45->46 revision (`editor-generic-action-keys`,
`product-boundaries.yaml:410`, now 46; verified 46 ids on disk in both
locales). Fix: cite the boundary entry by name without the numeral (a
count in a code comment re-staled once already - proc-normative-count-
recomputed trigger 2), or update to 46 with the D59 note.

**M3 - check-i18n.mjs:56 overgeneralizes.** "Shell IpcError codes ...
never spelled out literally in src/" - `src/views/FirstRun.vue:39,41`
carries `mkvmerge-not-found`/`mkvmerge-too-old` as case literals. Zero
behavioral consequence (literalAnywhereIds catches them); the claim is
wrong as written. Fix: "reached via the generic `$t(err.code, ...)`
pattern; most never appear as literals (FirstRun's two detect codes are
the exception)".

**M4 - Design D51 citation stale after D63 rewrote the cited file.**
`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:355-356`
cites the raw-id fallback at `muxsmith-cli/src/i18n.rs:41-46`; post-D63
it sits at `i18n.rs:121-123` (contract prose `:73-78`). `src/help/topics.ts`
itself cites by name only - no code fix needed. Fix: rides the queued
plan-close design one-liner batch (with the D54 `OutputCfg.directory`
parenthetical already queued in controller-notes).

**M5 - Pre-existing stale paragraph survived the check-3 block edit.**
`scripts/check-i18n.mjs:97-102` still opens "With only `locales/en/`
present (current tree), the comparison loop ... passes trivially" -
stale since plan-5.5 T21 landed `locales/de/` (this very run prints
"1 other locale(s) checked"). Pre-existing, not introduced by this
branch; the branch edited the surrounding header without sweeping it.
Fix: fold into the M1-M3 comment sweep (rephrase to past tense or
delete).

**M6 - Shipped focus-equals-hover is spec-unrecorded.** D52 mandates
and T12 shipped `focusin` as the keyboard-equivalent hover trigger
(`src/App.vue:92`; e2e keyboard test asserts it); amended spec 8.3
records hover, pin, Esc, view-switch and suppression, but not the focus
trigger. One-sentence spec 8.3 addition ("keyboard focus is equivalent
to hover for topic selection"); batch with I1's wording amendment.

### Low / observations (no action)

- **L1**: `tracksFields.rules.helpId` (`src/editor/registries.ts:207`)
  never reaches the DOM - the tracks section is excluded from
  `topLevelFields` (`EditorView.vue:289-293`) and the bespoke grid's
  caption literal (`EditorView.vue`, `data-help-id="editor-tracks-rules"`)
  is the live annotation. Both literals are design-mandated (D53's
  18-entry list + D54 #14's caption note), same id, both seen by D62's
  scan. Inert by construction, would go live if the tracks section were
  ever dispatcher-rendered.
- **L2**: the D61 scan matches commented-out `IpcError::new` lines
  (probe-verified). False-RED direction only; consistent with the
  line-based charter.
- **L3**: `src-tauri/src/lib.rs` test `with_attaches_and_overwrites_params`
  passes `"index"` as a Str while production promotes `index` to Num -
  it tests builder overwrite semantics, not the wire; fine as is.
- **L4**: spec 8.3's "faint border" vs shipped `outline` - the spec word
  describes appearance, the CSS mechanism is an enumerated D52 semantic
  ("outline, never border - no layout shift") under the ratified
  presentation-token carve-out. Checked, not a conflict.

## Cross-task semantics (duty 2, what task reviews could not see)

- **D56 live switch x D52/D54 help mode**: clean. The sidebar re-renders
  its topic through `currentLocale` (`HelpSidebar.vue` computed over
  `topicHtml`); a pinned topic survives a locale switch and re-renders in
  the new locale; the settings button is allowlisted, the open dialog
  consumes Esc (`isOpen()` expose), and `applyLocale` touches no help
  state. Only fallout: the two stale e2e narratives (M1).
- **D57 markers x D58 dropdowns x D59 ordinal on the grid**: clean. Five
  columns (order leading, spec 8.2's own list); ordinal is `index + 1`
  presentation-only; row markers anchor `tracks[{i}]` in the source cell;
  the caption anchors `tracks.rules` while keeping its Fluent accessible
  name (gui-table-caption intact); dropdowns live only in
  PropertyMapWidget exact cells, path-gated `tracks[`-prefix so the
  attachment matchExpr maps (the recorded pre-existing type-table flaw)
  never grow a wrong dropdown; `raw:type` byte-inequality keeps the
  bypass. The T14-F1 double-anchor (grid row vs detail root) is
  suppressed via `suppress-self-anchor` and the e2e fixture reaches the
  collision (`editor-markers.spec.ts`, fixture #17, count-based
  assertion).
- **T20 ParamValue x every render site**: the wire types end-to-end
  (`error.rs` ParamValue untagged, four promotion sites exactly as
  enumerated - `error.rs` ApplyError impl x3, `run.rs:935`; all eight
  frontend sites typed `string | number`, `vue-tsc` in gate 7 is the
  enforcing gate); the e2e mock reject shape widened
  (`e2e/mocks.ts` `Record<string, string | number>`); `Diagnostic`
  wire untouched. The D55-attribute surfaces never carry params, no
  interaction.
- **T17/T18/T19 gates x the merged tree**: green on the real catalogs +
  topic tree, and all four new check families fire-verified RED by this
  review's probes (above). The en-vs-de parity form of rule 5 goes green
  on the three nested/sibling-select messages the round-7 amendment
  names, while still catching placeable drift (probe 2).
- **D63/D64 en-pin funnel x wave-1+ CLI changes**: the funnel invariant
  holds tree-wide (grep with positive control); no post-wave-1 test
  bypasses it; `cargo test` green under a de-capable binary proves the
  11 snapshots are locale-pinned in practice, not just by convention.

## Spec conformance of the whole (duty 3)

Walked amended 8.3 + 8.4 + §10 sentence-by-sentence against shipped
behavior (e2e evidence + probes): toggle always visible in the three
views; sidebar opens on the current view's topic and scrolls
independently (`src/style.css` aside block); hover/pin/Esc/view-switch
semantics match the amended bullets exactly (e2e help-mode suite,
including the round-6 instant hover-clear case); the null-hover fallback
chain matches amendment 6(a); Esc-with-dialog matches 6(b);
suppression matches 6(c) for every click-derived path and diverges on
the non-click channels (I1); help mechanics (`help/<locale>/<help-id>.md`,
stable ids, per-view ids) shipped verbatim; 8.4's amended bullets are all
true on this tree (bilingual both surfaces incl. CLI embed, live GUI
switch, per-message fallback both surfaces - CLI chain walk verified in
`i18n.rs` with the four enumerated unit tests); §10's amended eslint
sentence matches `eslint.config.js` reality; spec-silent shipped
behaviors: focus-equals-hover (M6) - everything else shipped is either
spec-recorded or Tier-2-decreed (dropdowns). Catalog structure matches
the design's section-2 table exactly (38/8/28/41/46/50/26 ids, both
locales, recounted on disk); the four `close-abort-*` messages untouched;
`batch-recents-select` value-less shape shipped; the one selector gained
is `apply-rule-index-out-of-range` en+de.

## Triage of collected inputs (duty 4)

| # | Item | Verdict | Action |
|---|---|---|---|
| 1 | Stale topics.ts/D51 citation (T11) | **FIX-NOW** | The stale span lives in the DESIGN (D51, design doc :355-356, `i18n.rs:41-46` -> now `:121-123`); `topics.ts` itself cites by name only, no code change. One-liner rides the queued plan-close design-tidy batch (M4). |
| 2 | mocks.ts / smoke.spec narratives (T7) | **FIX-NOW** | Both narratives assert "no live swap exists" - falsified by D56 (M1). Comment sweep: scope both to the restart path, point live coverage at locale-switch.spec. Mechanisms and tests stay. |
| 3 | KeywordOrBlockWidget double-title (T4) | **RECORDED** | Benign: nested block section reuses the same labelKey, so fieldset and select carry the same tooltip; innermost title wins on hover. The 1.x polish option (distinct block labelKey) is an id-budget owner decision under `editor-generic-action-keys`; no v1 action. |
| 4 | h1-scheme cross-stream alignment (T9/T10) | **OWNER-PASS** | Real inconsistency confirmed on tree: T9 topics use "Label (section)" (e.g. `# Pattern (input)`), T10/T8 use natural phrases (e.g. `# Exact match`, `# Batch view`). All topic wording is already gated by the owner's rendered-surface pass; present both schemes there with the two concurring verdicts' recommendation (T9's form). |
| 5 | Budget-45 comment (T15) | **FIX-NOW** | Two sites, not one: `PropertyMapWidget.vue:45` AND `ListWidget.vue:15` (second found by this review). Comment sweep (M2): drop the numeral or update to 46. |
| 6 | T18 retired-comment nit | **FIX-NOW** | `check-i18n.mjs:56` "never spelled out literally" vs `FirstRun.vue:39,41` literals (M3). One-line precision fix in the same comment sweep. Zero behavioral effect either way. |
| 7a | T19 watch: grammar-constrained id capture blind spot | **RECORDED** | A malformed non-kebab `data-help-id`/`helpId` literal is silently unscanned; bounded by the runtime raw-id render (visible text, never blank) and by the closed 22-id set being entirely well-formed. Revisit only if the id grammar ever widens. |
| 7b | T19 watch: RAW_HTML_RE fenced-block gap | **RECORDED** | Inline-span-only stripping means a fenced code block containing `<` (or `a<b` prose) would go false-RED - the safe, self-announcing direction; the D62 amendment already records rephrase-not-exempt as the posture. No current topic trips it (gate green). |

Comment-sweep dispatch contents (one small fix round): triage 2 + 5 + 6,
plus M5 (pre-existing check-3 paragraph). Design-tidy batch: triage 1 +
the queued D54 parenthetical. I1's fix + wording ruling is its own
dispatch (blocking).

## House dimension + latitude (duty 5)

- Tier-2 sweep found no violated entry the task reviews missed:
  `gui-closed-domain-dropdowns` delivered (exact cells only, path-gated);
  `editor-generic-action-keys` budget 46 verified on disk both locales
  (only the two stale code comments, M2); `cli-multilang-rendering`
  delivered with the companion pin constraint held tree-wide;
  `core-37-prose-free-core` untouched (no core src changes in the
  branch); `gui-table-caption` intact; `i18n-05` applied to the one
  genuinely-count param and correctly NOT to `$index`;
  `ci-10-pin-everything` (marked exact pin); `core-derive-dont-restate`
  (domains emitted, drift-checked, `DOMAINS` in the widget references
  the generated arrays); `proc-verification-step-must-be-falsifiable`
  applied to this review's own gate trust (four probes).
- Latitude sweep of the amended design (both forms): the round-7 and
  round-8 amendment blocks enumerate their semantics completely (rule-5
  parity parts (b)-(d) with the carve-out list; D62 checks 5-6 with the
  deliberate code-span asymmetry and its consequence); no unenumerated
  set in a normative position found; the presentation-token carve-out is
  cited where used (`style.css`, D52 highlight clause). One mechanics-
  level omission surfaced as I1: "activation" was never enumerated as a
  DOM-event-class set, which is where the suppression gap hid - recorded
  in HARVEST as the generalizable lesson rather than as a latitude
  violation (the design believed it had closed the fork; the enumeration
  it closed was targets, not channels).

## HARVEST

- **Ledger candidate (pattern)**: *A "suppress all user actions over a
  region" decision must enumerate the DOM event CLASSES that can mutate
  state (click, keyboard activation, drag/drop, text input, change),
  not just the target set - "activation" is not a DOM primitive, and a
  click-capture implementation silently exempts every non-click channel.*
  Evidence: I1 - three green task reviews (T12, T13, T12-fix) each
  verified the click paths thoroughly and none asked about drag or
  typing, because no artifact enumerated the channel set. Sibling of
  proc-latitude-clause-boundary (the unenumerated set was one level
  below the vocabulary the reviews scanned).
- **Occurrence candidate** on the T11-harvested comment-citation entry
  (code-comment-line-citations-drift, tier 1): M4's design-doc flavor -
  a design that rewrites a file it cites elsewhere re-stales its own
  citation in the same plan; the sweep trigger is "this plan edits a
  file another section of the same plan cites by line".
- **Occurrence candidate (mechanism-claim comments)**: M1 - when a plan
  ships a capability a standing comment denies ("there is no live
  swap"), the denial is greppable at design time by the old mechanism's
  description; a changed-mechanism comment sweep belongs in the plan's
  final task. Same family as the conventions' sweep-dependencies rule,
  applied to prose claims in test/mock layers.
- **Over-restriction watch**: nothing new fired; D62 check 5's zero-pipe
  strictness and check 1's grammar constraint both behaved (no
  legitimate content gated). Carried items 7a/7b stay watch-only. New
  entry for the same list: the D61 scan's commented-out-code match (L2),
  false-RED direction, harmless until someone comments out a dead
  construction site and CI demands a catalog entry for it.
- **Positive pattern worth keeping**: the e2e suite's binding discipline
  (assert against the real catalogs/topics via `en()`/`enAttr()`/
  `topicMarkup()`, never hand-duplicated strings) held across all five
  new spec files and made the D55 migration reviewable at catalog level.

## Completeness note (what this review did NOT cover)

- **No live GUI session**: help mode, markers, dropdowns and the locale
  switch were verified through the e2e suite (real built bundle, mocked
  IPC) plus code reading; I1's leak vectors are verified by construction
  (handler/listener topology), not by a live drag reproduction - HTML5
  drag simulation is unreliable in the harness and the mechanism claim
  is directly readable from the listener set.
- **Linux only**: the Windows/macOS gate legs and the Tauri shell
  (`cargo tauri build`/dev, webkit behavior, native dialog Esc handling
  on each OS) were not exercised; the settings-dialog Esc test runs in
  Chromium, not the packaged webview.
- **Help-topic content accuracy** (technical correctness and de wording
  of the 44 files) was not re-reviewed beyond the T8-T10 task reviews,
  the h1 audit, and the gate's structural bans; the owner
  rendered-surface pass is the standing gate for wording.
- **Bookkeeping diffs** (decision-ledger, journal, handoffs, plan-doc
  edits, ~700 lines) were skimmed for shape, not audited entry-by-entry.
- **No adversarial markdown fuzzing** of `marked` output; D50's
  first-party trust model plus the D62 bans were accepted as the
  boundary (probe-verified the bans fire).
- **Insta snapshots** verified by the green run, not re-read
  individually; `catalog_completeness.rs` trusted as en-reference per
  D63's recorded gate-coverage decision.
- **Claim classes**: performance (marked parse cost per hover swap,
  bundle growth ~100 kB raw topics) not measured; axe coverage taken
  from the e2e suite's existing runs, no additional manual a11y audit.

---

# Delta verdict (2026-07-22, resumed whole-branch reviewer)

Scope: fix wave `13e138c` on master (7 files, on top of ledger commit
`c151390`), graded against this verdict's concrete fixes. Fix report:
`.superpowers/sdd/plan-7/whole-branch-fix-report.md`.

## Verdict

**READY-subject-to** - subject to exactly the two pending owner items:
(1) the I1 keyboard/text residual ruling (extend suppression to
typing/select channels, or amend spec 8.3's "control activation ...
suppressed" sentence to its ruled scope - the shipped code half closes
the drag leak either way), and (2) the owner rendered-surface /
plan-close pass, which carries M4 + M6 (the queued design/spec
one-liners), the h1-scheme decision (triage 4, OWNER-PASS), and all
topic/tooltip wording. Nothing else remains open from this review.

## Per-item verification (each against the verdict's concrete fix)

- **I1 (code half)**: `src/App.vue` adds `onHelpDragstart` =
  `event.preventDefault()` registered/unregistered as a capture-phase
  `dragstart` listener in the existing `watch(helpMode)` set - exactly
  the prescribed one-listener fix, nothing more (no stopPropagation, no
  beforeinput/select gating; the handler comment names the owner-ruled
  residual). VERIFIED.
- **M1**: `e2e/mocks.ts` and `e2e/smoke.spec.ts` narratives now scope to
  the restart/bootstrap path, state that the app DOES swap live (D56,
  `SettingsDialog.save()` -> `applyLocale`), and point live coverage at
  `e2e/locale-switch.spec.ts`. VERIFIED (matches the prescribed rewording).
- **M2**: both sites (`PropertyMapWidget.vue`, `ListWidget.vue`) drop the
  numeral and cite `editor-generic-action-keys` by name - the verdict's
  preferred option. VERIFIED.
- **M3 / T18 nit**: `check-i18n.mjs` check-2 comment now names FirstRun's
  two detect-code literals as the exception - matches
  `FirstRun.vue:39,41`. VERIFIED.
- **M5**: the check-3 "current tree" paragraph rewritten to the actual
  de-present tree with the trivial-pass history in past tense; accurate
  against the live run ("1 other locale(s) checked"). VERIFIED.
- **Out-of-scope confirmed untouched**: `git diff --name-only
  cc0e6d7..13e138c -- docs/` shows only `conventions.yaml` +
  `decision-ledger.yaml` (the controller's harvest/ledger commit
  c151390); the v1 spec, plan-7 design, and plan files are untouched -
  I1's wording half, M4 and M6 remain genuinely pending as routed. No
  Rust file in the diff (7 files: 3x e2e/*.ts, check-i18n.mjs, App.vue,
  2x widgets/*.vue), so cargo gates are unaffected by construction.

## Adjudications

**(a) New test helpers + describe block vs the new-test-infrastructure
boundary: MANDATED COVERAGE, not overreach.** The failing-test-first case
was in the fix dispatch's own mandate, so the addition is pre-authorized
work, not a silent self-classified grant exercise; the helpers
(`attemptDrag`, `readRuleOrder`) are file-local functions in the existing
spec, shaped like their siblings, additive-only (no assertion weakened, no
fixture mutated, no new harness/mock/gate-config file). Surfacing it
anyway was the correct move under the boundary's stop-and-report letter -
calibration data that the boundary works without over-restricting when
the mandate is explicit.

**(b) Synthetic DragEvent model: FAITHFUL; the suppression assertion is
load-bearing.** The assertion that matters
(`inside.dragstartPrevented === true`) exercises the real app code
end-to-end: a real `dragstart` dispatched on the real row, propagated
through the real capture listener on `<main>`. The only modeled link is
the browser's abort-on-prevented-dragstart, which is the HTML
drag-and-drop processing model's own guarantee (platform behavior, not
app code), and the gating on `defaultPrevented` encodes it correctly.
Non-vacuity is proven twice over: the outside-help-mode control genuinely
permutes the grid (`[beta, gamma, alpha]` asserted against a
differs-from-default order), and this reviewer independently re-ran the
RED state - reverted the App.vue hunk, rebuilt, ran the filtered spec:
**1 failed at exactly `help-mode.spec.ts:449`
(`dragstartPrevented` received `false`) with the control assertions
passing** - matching the fix report's claim precisely; hunk restored
byte-identically, tree clean.

Observation, no action: a prevented `dragstart` still lets the row's own
bubbling `@dragstart` handler set `dragIndex`, and no `dragend` follows a
canceled drag, so a stale index persists after a suppressed attempt.
Unreachable-harmful: a `drop` cannot occur without a new drag, and any
new drag overwrites the index before its own drop pairs with it.

## Delta gate run (this reviewer's build, pristine tree at 13e138c)

| Gate | Result |
|---|---|
| `pnpm build` (vue-tsc + vite) | GREEN |
| `pnpm lint` | GREEN (exit 0) |
| `pnpm check:i18n` | GREEN - summary line byte-identical to gate #8 |
| `pnpm test:e2e` (full) | GREEN - **53 passed** (52 prior + the new I1 case) |
| cargo | Not re-run - no Rust/Cargo file in the diff (verified from the file list) |
| RED probe | App.vue hunk reverted -> rebuilt -> filtered I1 spec FAILED at :449; restored -> full suite green, `git status` clean |
