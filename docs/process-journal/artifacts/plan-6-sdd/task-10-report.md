# Task 10 report: D45 - the widget components

**Status: NEEDS_CONTEXT.** Blocked before Step 1 (writing the failing e2e
assertions). No code written, no files touched, worktree unchanged from
Task 9's commit (`57cc117`).

## What was verified (not in question)

- Design ground truth re-anchored by TEXT, not the brief's stale line
  numbers: the brief cites `FieldWidget` at `:806-819`; in the current
  design doc (`docs/superpowers/specs/2026-07-15-plan-6-design.md`) the
  same union now sits at `:850-860`, ten variants, identical text to what
  Task 9 already encoded in `src/editor/fieldSpec.ts:58-70`. No drift
  between design and Task 9's types.
- The 43-row widget table (design `:891-936`) and Task 9's 13 registries
  (`src/editor/registries.ts`) are read and cross-checked; the ten
  `FieldWidget` kinds this task must build a component for are: `text`,
  `bool`, `optionalFlag`, `select`, `keywordOrBlock`, `directoryPath`,
  `stringList`, `propertyMap`, `list`, `section`.
- House component conventions read (`src/components/SuggestionCard.vue`,
  `src/components/JobRow.vue`): `defineProps<{...}>()`/`defineEmits<{...}>()`
  generic-typed (no runtime `defineProps({...})` call), all template text
  through `$t`, `data-testid` as the fallback locator only where no
  distinct accessible role/name exists, doc-comment header explaining the
  component's place in the surrounding contract.
- `jobRowState.ts:44-55`'s `jobStateKey` switch read: exhaustive via
  TS's control-flow narrowing over a string-literal union, no explicit
  `never` arm. Task 10's dispatcher must add one (`const _exhaustive: never
  = x`) as "a deliberate, minimal improvement," per the brief - confirmed
  this is additive, not a mandate to retrofit `jobRowState.ts`.
- The `withDefaults` + `T | null` vue-tsc quirk (BUILDING.md `:99-102`,
  detailed in `docs/process-journal/artifacts/plan-5-sdd/task-11-report.md`
  `:97-104`): a plain optional prop (`foo?: T | null`, no `withDefaults`)
  is the known-working shape; noted for any widget that needs an
  optional/nullable prop.
- `AttachmentRule`'s one-of (`select`/`drop`/`add`) is realized in the
  registry as three independent optional `section` widgets
  (`registries.ts:180-190`), not a single widget kind - the brief's "a
  component MAY present the one-of as a mode selector" is optional UX
  polish on top of that, not a required shape. Read as: ship the three
  independent optional sections (matches scale-appropriate design); a mode
  selector is not required to satisfy the brief.
- `section`/`list` widgets need to resolve a `RegistryName` to its actual
  registry object (`RegistryName -> Record<keyof T, FieldSpec>`) to
  recurse. No such lookup exists yet (`registries.ts` exports 13 named
  consts, no keyed map). Building one is in-scope work inside
  `src/editor/widgets/` (this task's own file scope), not a
  `registries.ts` edit.

None of the above blocks the work. The blocker is entirely the TDD/e2e
step.

## The blocking gap

**There is no reachable page, route, or mount point in the running app
that could render any editor widget, and Task 10's own file scope
(`src/editor/widgets/`, `e2e/smoke.spec.ts`) forbids creating one.**

Traced the full reachability chain:

- `src/main.ts` unconditionally does `createApp(App).use(fluent).mount("#app")`
  - the only mount in the app, no router, no query-param/hash gate.
- `src/App.vue`'s `View` union is `"batch" | "jobs"` (`:10`); nav renders
  exactly those two buttons plus Settings (`:71-96`). No editor entry.
  Confirmed no `EditorView`/`editor-view`/`view-editor` string exists
  anywhere in `src/`.
- `src/views/EditorView.vue` does not exist yet - Task 11 creates it (its
  own brief: "No nav wiring yet - `App.vue` is untouched until Task 13").
- Task 13 is the task that adds the `View` union member, the nav button,
  and the `v-show` mount block (`task-13-brief.md` Files list: `App.vue`
  `:10`, `:71-96`, `:98-112`).
- `vite.config.ts` is a plain single-entry SPA build (`index.html` ->
  `src/main.ts`), no `build.rollupOptions.input` multi-entry, no library
  mode. `vite build`'s tree-shaking means anything not imported from
  `main.ts`/`App.vue` never lands in `dist/`.
- `playwright.config.ts`'s `webServer` runs `vite preview` over that same
  `dist/` on port 4173; `page.goto("/")` always resolves to `index.html`
  and therefore always mounts `App.vue`. There is no second HTML entry,
  no dev-server proxy, nothing else Playwright's `page.goto` could reach.
- No component-testing package is installed
  (`@playwright/experimental-ct-vue` or similar) - `package.json`
  devDependencies confirmed: only `@playwright/test`, no `-ct-*` package,
  no Vitest, no `@vue/test-utils`.
- The one existing precedent for injecting non-page content into a test
  (`e2e/vite.harness.config.ts` + `e2e/tauri-mock-entry.ts`, bundled to
  `e2e/.generated/tauri-mock-harness.js`, injected via
  `page.addInitScript({ path })`) exists specifically to re-export
  `@tauri-apps/api/mocks`/`event` functions onto `window.__muxsmithE2E__`
  for `mocks.ts` to call - it does not mount Vue components and is not
  wired to do so. Its own doc comment stresses it "never touches
  `dist/`/`public/`, so it can never leak into the shipped Tauri bundle" -
  the codebase deliberately keeps test scaffolding out of the shipped
  build.

Conclusion: as specified, Task 10's Step 1 ("Extend `e2e/smoke.spec.ts`
with per-widget rendering assertions... using the harness's existing
mock") cannot produce a real RED-then-GREEN DOM-rendering test without
adding new test infrastructure outside the task's stated file scope - a
second bundle/entry point, a mount helper injected via
`page.addInitScript`/`page.evaluate`, or similar. That is unambiguously a
**test-surface** change under this task's own binding constraint: "No
design latitude beyond the standing grant above: a genuine fork
(wire-format, API-surface, test-surface, or multiple plausible answers)
returns as NEEDS_CONTEXT with a decision memo BEFORE resolution." The
standing structural-conformance grant explicitly excludes anything with
test-surface effect, so this doesn't qualify for silent resolution even
though it would extend an existing local pattern (the harness-bundle
approach) stylistically.

**This is not a Task-10-only problem.** Task 11's own brief has the
identical shape: it creates `EditorView.vue` with explicitly "No nav
wiring yet," yet its Step 1 says "opening the editor renders the rule
grid" as an e2e assertion. Task 12 modifies `EditorView.vue` further,
still with no nav wiring (that's Task 13), and its Step 1 says "opening
the editor renders each section." All three tasks (10, 11, 12) inherit
the same unreachable-page problem from the plan's own sequencing (nav
wiring deliberately deferred to Task 13, per the plan's F4/wave-4 note
and Task 13's brief). A ruling here should cover the mechanism for all
three, not just Task 10, or each will invent its own one-off fix and the
branch ends up with two or three inconsistent test-mounting mechanisms
merged in sequence.

## Candidate resolutions

**A. A dedicated widget/view test-mount harness**, structurally parallel
to `e2e/vite.harness.config.ts`/`tauri-mock-entry.ts`: a new `e2e/`-scoped
Vite entry that imports `createApp`, `fluent-vue`'s bundle setup (widgets
render through `$t`), and the widget/dispatcher (or `EditorView.vue` for
Tasks 11-12) under test, bundled to its own file under
`e2e/.generated/` and mounted via `page.setContent()` +
`page.addScriptTag({ path })`, never touching `dist/` or `vite.config.ts`.
Reusable verbatim by Tasks 11 and 12.
- Pro: real DOM rendering assertions, genuine TDD RED (component doesn't
  exist) -> GREEN, matches the literal ask ("per-widget rendering
  assertions").
- Con: net-new build/test infrastructure outside the stated file list for
  all three tasks; needs its own review; duplicates a slice of
  `main.ts`'s bootstrap (locale/fluent bundle) in a second place that has
  to be kept in sync.

**B. Defer DOM rendering assertions to Task 13**, when the editor is
actually nav-reachable. Tasks 10-12 verify by construction instead:
compiler-level completeness proofs (the `never`-arm dispatcher; a
deliberate-break falsifiability proof showing TS2322 names the
unhandled variant, same style as Task 9's registry-completeness proof),
`pnpm build`/`pnpm lint`/`pnpm check:i18n` gates, and no DOM-level
`smoke.spec.ts` additions until Task 13 can reach the surface for real.
- Pro: zero new test infrastructure, stays inside each task's stated file
  scope, avoids three tasks converging on possibly-different harnesses.
- Con: "extend `e2e/smoke.spec.ts` with... rendering assertions" and the
  literal TDD RED/GREEN-via-Playwright instruction in three consecutive
  briefs go unfulfilled as written; three tasks worth of UI code (widgets,
  rule grid, section composition) ship with no empirical rendering
  evidence until Task 13, which is a wide blast radius for a rendering bug
  to hide in.

**C. Pull forward a minimal slice of Task 13's nav wiring into Task 10**
so the surface is reachable now. Not recommending this one: it directly
contradicts Task 11's brief text ("No nav wiring yet - `App.vue` is
untouched until Task 13") and Task 13's own file list, and would have
Task 10 partially pre-empt three later tasks' scoped work.

**Recommendation:** leaning A over B - it's the only option that actually
delivers what all three briefs ask for (real rendering assertions with a
falsifiable RED), and the isolation precedent (`vite.harness.config.ts`
never touching `dist/`) is already established and directly reusable.
But this is exactly the kind of test-surface fork the task's own rules
say is not mine to resolve alone, and the choice has to be made once and
communicated to whoever implements Tasks 11 and 12, not re-decided three
times.

## What is ready the moment this is ruled on

- Component list: `TextWidget.vue`, `BoolWidget.vue`,
  `OptionalFlagWidget.vue`, `SelectWidget.vue`, `KeywordOrBlockWidget.vue`,
  `DirectoryPathWidget.vue`, `StringListWidget.vue`, `PropertyMapWidget.vue`,
  `ListWidget.vue`, `SectionWidget.vue`, plus a `FieldWidgetDispatcher.vue`
  (or equivalent) with the `never`-arm switch over `FieldWidget["kind"]`.
- A small `registryByName.ts` (or inline in the dispatcher/section/list
  widgets) mapping `RegistryName -> Record<string, FieldSpec>` from the 13
  named exports in `registries.ts`, needed for `section`/`list` recursion,
  scoped inside `src/editor/widgets/`.
- No mode-selector for `AttachmentRule`'s one-of; the three independent
  optional `section` widgets from the registry are sufficient per the
  brief's "MAY," matching scale-appropriate design.

## Files changed

None. Worktree is unchanged from Task 9's commit `57cc117`; `git status`
clean.

---

## Resumption: amendment routed, mount-harness amendment implemented (2026-07-16, same day)

The coordinator reported the NEEDS_CONTEXT above was routed to the
controller and resolved as **Option A** (my lean): a minimal test-mount
harness extending the established `e2e/vite.harness.config.ts` precedent.
A four-eyes plan amendment (two review rounds, APPROVED, commit `add3ab9`
on master) reworked Tasks 10-12's verification steps; a merge commit
(`80d410c`, docs-only: the amendment plus ledger harvests, `f36ac5f`)
landed on this worktree's branch `plan6-e` before I resumed. The
re-extracted brief (`.superpowers/sdd/plan-6/task-10-brief.md`) specifies
six exact harness files (`e2e/mount-entry.ts`, `e2e/vite.mount.config.ts`,
`e2e/mount.ts` new; `e2e/global.d.ts`, `e2e/tsconfig.json`, `package.json`
modified) with "no latitude to substitute another mechanism" for their
contents, plus the ten widgets exposing their value via the standard Vue
`modelValue`/`update:modelValue` v-model.

### Step 1: the harness, as specified, plus one bug found and fixed

Created the three harness files and applied the three modifications
verbatim to the brief's given content. `mount-entry.ts` bundles Vue,
`fluent-vue` and the app's own `buildBundles` (`src/i18n/index.ts`) via
`import.meta.glob(["../src/editor/widgets/*.vue", "../src/views/
EditorView.vue"], { eager: true })` into a component registry, exposing
`window.__muxsmithMount__`/`__muxsmithModel__`/`__muxsmithEmitted__`.
`vite.mount.config.ts` bundles it as a second IIFE into
`e2e/.generated/mount-harness.js`, parallel to `tauri-mock-harness.js`
(`emptyOutDir: false` confirmed not to clobber it: both files coexist
after a full `test:e2e` run). `mount.ts` drives it from Playwright via
`page.setContent` + `page.addScriptTag` + `page.evaluate`, exactly as
given.

**One real bug in the prescribed config, found by actually running it (not
a design fork, a mechanical fix to the same mechanism):** the first build
of `mount-harness.js` (430.66 kB) threw in the browser before assigning
any of the three globals:

```
PAGEERROR: process is not defined ReferenceError: process is not defined
    at e2e/.generated/mount-harness.js:14:18
```

Confirmed the cause: Vue's `esm-bundler` build leaves
`process.env.NODE_ENV` unreplaced by design (dev-mode warning guards;
`grep -c "process.env.NODE_ENV" mount-harness.js` found 5 occurrences pre-fix,
e.g. `var EMPTY_OBJ = !!(process.env.NODE_ENV !== "production") ? ...`),
expecting the consuming bundler to define it. `vite build` on the app's
own `index.html` does this automatically; `build.lib` (both this config
and `vite.harness.config.ts`) does not get the same automatic
substitution for a bundled dependency -- unexercised until this config,
since `vite.harness.config.ts` bundles only `@tauri-apps/api`, which has
no `process.env` reference. Fix: added `define: { "process.env.NODE_ENV":
JSON.stringify("production") }` to `vite.mount.config.ts` (the standard,
documented Vite recipe for this exact class of problem). Rebuilt: bundle
dropped to 247.46 kB (dead-code elimination of the now-unreachable dev
branches) and `window.__muxsmithMount__` resolved to a function; verified
via a throwaway debug spec with `page.on("pageerror", ...)` before
deleting it. Final size with all ten widgets registered: 288.11 kB.

### TDD evidence

**RED** (`pnpm test:e2e`, 12 new tests, before any widget existed):

```
Error: page.evaluate: Error: unknown mount component "BoolWidget"
Error: page.evaluate: Error: unknown mount component "DirectoryPathWidget"
Error: page.evaluate: Error: unknown mount component "FieldWidgetDispatcher"
Error: page.evaluate: Error: unknown mount component "KeywordOrBlockWidget"
Error: page.evaluate: Error: unknown mount component "ListWidget"
Error: page.evaluate: Error: unknown mount component "OptionalFlagWidget"
Error: page.evaluate: Error: unknown mount component "PropertyMapWidget"
Error: page.evaluate: Error: unknown mount component "SectionWidget"
Error: page.evaluate: Error: unknown mount component "SelectWidget"
Error: page.evaluate: Error: unknown mount component "StringListWidget"
Error: page.evaluate: Error: unknown mount component "TextWidget" (x2 tests)
12 failed, 7 passed (the 7 pre-existing tests, untouched)
```

Exactly the brief's predicted RED shape ("`unknown mount component` because
`src/editor/widgets/` is still empty").

**GREEN** (`pnpm build && pnpm test:e2e`, after implementing the ten
widgets + dispatcher + `shared.ts`): 19/19 passed (12 new + 7 pre-existing,
1.1-1.2s), reproduced on a clean re-run after the restore below.

### Falsifiability proof: the dispatcher's `never` arm

Deliberately removed the `case "text": return TextWidget;` arm from
`FieldWidgetDispatcher.vue`'s switch, ran `pnpm exec vue-tsc --noEmit`:

```
src/editor/widgets/FieldWidgetDispatcher.vue(54,13): error TS2322: Type '"text"' is not assignable to type 'never'.
```

Matches the brief's claimed shape exactly (TS2322, naming the unhandled
variant `"text"`). Restored the case; `vue-tsc --noEmit` returned to a
clean exit (0 output).

### Incident: a backgrounded verification command left a file mid-break, caught and fixed

A combined `eslint ... | python3 ...` + deliberate-break + restore
one-liner (verifying `no-raw-text` genuinely fires, not just that my code
happens to pass) exceeded the tool's 120s foreground timeout and moved to
background mid-sequence, because the trailing plain `cp` prompted
interactively for an overwrite confirmation that never arrived. This left
`TextWidget.vue`'s label hardcoded to `"Hardcoded Label"` on disk. Caught
immediately via the tool's own file-change notification, confirmed the
control had already fired correctly before the hang (`18:22  error  raw
text 'Hardcoded Label' is used  @intlify/vue-i18n/no-raw-text`), restored
the line via `Edit` (not the stuck `cp`), killed the background task, and
re-ran the full nine-part gate foreground afterward to confirm nothing
else was left inconsistent. `git diff` against the final committed file
carries no trace of the break (never staged, never committed).

### Design decisions disclosed (not design forks -- judgment calls within an unambiguous brief, verifiable by any reviewer)

1. **Widget prop shape**: each of the ten widgets takes `spec:
   EditableFieldOf<K>` (a `shared.ts` helper narrowing `EditableField`'s
   `widget` to one `FieldWidget` variant) and a standard `defineModel()`
   v-model, matching the brief's own Step 2 illustration
   (`props: { spec: <FieldSpec>, modelValue: <value> }` against
   `TextWidget` directly, not the dispatcher) -- each widget renders its
   own label via `$t(spec.labelKey)`, the dispatcher does not duplicate it.
2. **`directoryPath` is a plain text input, no browse-dialog button.** The
   amendment states widgets "install no Tauri IPC mock... fed their model
   as a prop"; a real directory picker needs `@tauri-apps/plugin-dialog`
   IPC, which is Task 13's job alongside the rest of the IPC surface.
3. **`keywordOrBlock` shows the keyword combobox and the nested block
   section simultaneously, no mode toggle.** Mirrors the registry's own
   precedent for `AttachmentRule`'s one-of (`select`/`drop`/`add` as three
   independent optional sections, `registries.ts:180-190`): no widget-level
   exclusivity, core diagnoses an over-set model (spec 7, cross-field
   constraints stay in core).
4. **`stringList` is one comma-separated textbox, not per-item add/remove
   rows.** A common, idiomatic pattern for a short flat token list
   (extensions/tags-shaped fields); avoids inventing add/remove chrome
   text entirely.
5. **`propertyMap`/`list` add/remove buttons reuse the EXISTING
   `editor-attachment-rule-add`/`editor-attachment-rule-drop` catalog keys
   ("Add"/"Drop") rather than adding new ones.** `gui-editor.ftl` stays at
   43 keys (D45's own constraint, unchanged by this task); no generic
   "add"/"remove" wording exists anywhere in any catalog (checked
   `gui-common.ftl`, `gui-batch.ftl`, `gui-jobs.ftl`, `gui-settings.ftl`
   directly) and this task's file scope does not include any `.ftl` file.
   "Drop" is already this app's own established exclude-this-item
   vocabulary (`KEEP_DROP`, used by `attachmentsFields.unmatched`,
   `tracksFields.unmatched`, `tagsFields.global`/`track`), so the reuse is
   a semantic fit, not a stretch, and mirrors `browse-button`'s existing
   cross-view reuse (`gui-common.ftl`, reused by `FirstRun`/
   `SettingsDialog`/`BatchView`). `check-i18n`'s unused-key count stayed at
   the Task-9 baseline of 18 (these two keys were never "unused" -- they
   already had a `labelKey:` reference in `registries.ts`), confirming the
   reuse resolves to real, catalog-verified ids.
   PropertyMap's per-row key/value inputs use `data-testid` (no distinct
   accessible role/name exists for a free-text property name -- the house
   fallback convention already established by `SuggestionCard.vue`).
6. **`list`'s `reorderable: true` case uses native HTML5 drag-and-drop on
   the item itself**, not move-up/down buttons: no translated chrome text
   needed (matches spec 8.2's own "drag to reorder" wording for the
   top-level rule grid), and neither current registry consumer of the
   generic `list` widget (`matchExpr.any`/`matchExpr.not`) sets
   `reorderable: true`, so this path is implemented for contract
   completeness against the full `FieldWidget` type, not exercised by a
   current field.
7. **`propertyMap` values are always edited as strings**, even when
   `values: "scalar"` (Rust's `Scalar = boolean | number | string`): a
   plain string is a legal `Scalar`, so this is type-valid; building
   type-selection UI (string/number/boolean per row) was judged over-scope
   for this pass and is the same accepted-gap class the registry itself
   already names ("a mismatched widget is a visible rendering bug caught
   the first time the panel opens").
8. **`section`'s `optional` is not a create/remove toggle.** An absent
   section (`modelValue` undefined) still renders its sub-fields against
   an empty object; editing any one of them implicitly creates the
   section. Avoids a further "add this section" chrome-text need.

None of these touch wire format, IPC/API surface, or the registry/catalog
mechanism Task 9 settled; all are local to how a widget presents/edits its
own slice of the model, verifiable against the design doc and the actual
registry contents cited above.

## Gate results (nine parts, foreground, no subsets, re-run clean after the incident above)

1. `cargo fmt --all --check` -- PASS.
2. `cargo clippy --workspace --all-targets -- -D warnings` -- PASS, zero
   warnings (no Rust files touched; ran anyway per "no subsets").
3. `cargo test --workspace` -- PASS, all crates green.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- PASS.
5. `cargo deny check` -- PASS ("advisories ok, bans ok, licenses ok,
   sources ok").
6. `pnpm lint` (eslint, including D27 `no-raw-text`) -- PASS, no output;
   verified NOT a false-negative by deliberately breaking it (see
   incident above) and confirming it fires, then restoring.
7. `pnpm build` (`vue-tsc --noEmit && vite build`) -- PASS. The widgets are
   NOT present in the shipped `dist/` bundle (`154.70 kB`, unchanged from
   Task 9's baseline): nothing in the running app imports them yet
   (`App.vue` is untouched, Task 13's job), confirming they stay
   unreachable from the production build exactly as the harness's own
   design intends.
8. `pnpm check:i18n` -- PASS (exit 0), "ok (33 source files scanned, 229
   catalog ids, 18 unused warning(s), 1 other locale(s) checked for parity
   against 7 en/ catalog(s))" -- identical 18-unused-key baseline to
   Task 9's report, confirming the `editor-attachment-rule-add`/`-drop`
   reuse added no new unused/orphaned reference.
9. `pnpm test:e2e` -- PASS, 19/19 (12 new mount-harness widget tests + 7
   pre-existing smoke tests, all green).

Typography: grepped every new/changed file for the AI-tell glyph set
(em/en-dash, minus sign, curly quotes, ellipsis, NBSP) via a Perl-regex
grep -- clean; verified the pattern fires against a known-present em-dash
control before trusting the clean result (per standing doctrine on
negative-result checks).

## Files changed (this resumption)

- `e2e/mount-entry.ts` (new), `e2e/vite.mount.config.ts` (new),
  `e2e/mount.ts` (new): the test-mount harness, per the amended brief's
  exact specification (plus the `process.env.NODE_ENV` `define` fix in
  `vite.mount.config.ts`, disclosed above).
- `e2e/global.d.ts`, `e2e/tsconfig.json`, `package.json`: the three
  amendment-specified modifications (mount globals, `vite/client` types,
  `test:e2e` chain).
- `src/editor/widgets/` (new, 12 files): `TextWidget.vue`, `BoolWidget.vue`,
  `OptionalFlagWidget.vue`, `SelectWidget.vue`, `KeywordOrBlockWidget.vue`,
  `DirectoryPathWidget.vue`, `StringListWidget.vue`, `PropertyMapWidget.vue`,
  `ListWidget.vue`, `SectionWidget.vue` (the ten `FieldWidget` variants),
  `FieldWidgetDispatcher.vue` (the exhaustive-by-`never`-arm dispatcher),
  `shared.ts` (`EditableFieldOf<K>` prop-narrowing helper,
  `registryByName: Record<RegistryName, ...>` lookup for recursion).
- `e2e/smoke.spec.ts`: extended with a `"editor widgets: mount-harness
  rendering"` describe block, 12 tests (one per widget kind, one
  dispatcher-dispatch test, plus a second `text`-multiline case), all
  fixtures pulled from Task 9's real registries (`src/editor/registries.ts`)
  rather than hand-rolled `FieldSpec` literals.

Committed as `c50866a` "gui: the ten field widgets, exhaustive by
never-arm, plus the wave-3 test-mount harness (D45)" on branch `plan6-e`,
staged explicitly per the brief's list (`git diff --cached --stat`: 19
files changed, 962 insertions, 2 deletions), unsigned
(`commit.gpgsign=false`), `Co-Authored-By: Claude Sonnet
<noreply@anthropic.com>` trailer. Not pushed. `git status` clean after
commit.

## Self-review

- 10/10 `FieldWidget` variants have a component; cross-checked against
  `fieldSpec.ts`'s union (`:58-70`) and the design doc's `:850-860` list
  name-by-name.
- Dispatcher exhaustive with an explicit `never` arm; falsifiability proof
  above shows TS2322 naming the unhandled variant, then restored.
- No semantic validation anywhere: no widget rejects, clamps, or
  reinterprets a value beyond structural parsing (comma-split for
  `stringList`, `Object.fromEntries`/`Object.entries` for `propertyMap`) --
  every widget holds and forwards whatever the user enters.
- No new Fluent keys added; `gui-editor.ftl`/`locales/de/gui-editor.ftl`
  untouched (not in the staged file list, confirmed by
  `git diff --cached --stat` above). `select`/`keywordOrBlock` option
  tokens render raw (`{{ option }}`/`{{ keyword }}`, not `$t(...)`),
  confirmed by `pnpm lint` passing D27 with these exact templates.
- House prop/emit/`$t` style matched: `defineProps<{...}>()` bare where
  `spec` is template-only (`DiagnosticsPanel.vue`/`ResolutionTable.vue`'s
  pattern), captured as `const props = ...` where script-side logic needs
  it (`JobRow.vue`/`SuggestionCard.vue`'s pattern); bare `spec.x` in
  templates, `props.spec.x` in script, matching both house examples
  exactly; `useId()` used for label/control association since these
  widgets are not singletons (unlike `SettingsDialog.vue`'s hardcoded
  ids, which is a singleton and cannot collide).
- `withDefaults` + `T | null` quirk: not encountered/not applicable --
  no widget uses `withDefaults`; nullable props are plain optional
  `defineModel<T | null>()` calls, the known-working shape per
  `BUILDING.md`.
- Cross-field constraints (`AttachmentRule` one-of, `Locator.match_to_source`
  xor `match_pattern`) get no dedicated widget; `AttachmentRule`'s
  registry entry already expresses the one-of as three independent
  `section` widgets (Task 9's own encoding), and `keywordOrBlock`'s
  design (disclosure item 3 above) follows the same no-toggle precedent.

## Concerns

None blocking. Two follow-ups worth a cheap look before Task 12/13 build
on top of this surface:

1. **The `editor-attachment-rule-add`/`-drop` reuse (disclosure 5) is a
   genuine, disclosed judgment call, not a rubber-stamped brief
   instruction** -- the brief is silent on exactly how `list`/`propertyMap`
   should source add/remove chrome text under the "43 keys, don't touch
   the catalogs" constraint. Worth a second pair of eyes on whether
   reusing an AttachmentRule-scoped key for a generic action across every
   `list`/`propertyMap` instance is the right long-term call, or whether a
   small, deliberate `gui-editor.ftl` amendment (2-4 genuinely generic
   keys) should be proposed for a later task once the pattern's real
   usage surface (Task 12's section composition) is visible.
2. **`propertyMap`'s always-string values** (disclosure 7) mean a
   `TrackRule.changes` entry that should carry a real boolean/number
   (e.g. a `forced`/`default` flag) can currently only be typed as the
   string `"true"`/`"false"` through this widget. Functionally valid
   (`Scalar` includes `string`) and core will diagnose a genuine type
   mismatch, but it is a real UX gap for the most common `changes` case;
   worth revisiting once Task 12/13 show how `changes` actually gets
   exercised end-to-end.
