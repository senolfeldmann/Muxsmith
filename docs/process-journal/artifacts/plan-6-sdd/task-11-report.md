# Task 11 Report: D45 - the editor view, part a: the rule grid and drag-reorder

## What was implemented

`src/views/EditorView.vue` (new): the profile editor's top-level `tracks.rules`
grid (spec 8.2 view 4's "track-rule grid ... drag to reorder"), bespoke against
`profile.ts` types directly (not through the field-widget dispatcher, per
`ListWidget.vue`'s own doc distinguishing its generic `list` widget from this
task's bespoke grid).

- **v-model contract**: `defineModel<Profile>()` -- the WHOLE profile is the
  held model (not just the rules array), matching the brief's "EditorView
  therefore takes the profile as its modelValue prop and emits
  update:modelValue on reorder" and leaving Tasks 12-13 free to extend this
  file's rendering without reshaping its interface.
- **Grid**: a `<table>` with one row per `tracks.rules` entry (`data-testid="editor-rule-row"`),
  columns Source / Match / Optional / Changes. Column headers reuse the four
  EXISTING `TrackRule` field labels (`editor-track-rule-source`,
  `-match-expr`, `-optional`, `-changes`) verbatim; the view's own `<h2>`
  reuses `editor-tracks-rules` ("Rules"). Zero new or changed catalog keys;
  `gui-editor.ftl` stays at 43 keys in both locales (verified below).
- **Row summaries** (zero frontend semantic validation, spec 7): every cell
  renders either a real profile-format token (`SOURCE_KEYWORDS[0]` =
  "primary"), an actual value already in the model (an external path, a
  matchable property name + its scalar for Match, a settable property name
  for Changes), or a plain count -- never invented UI prose. Optional renders
  as a disabled, unlabelled-by-`<label>` checkbox with `:aria-label` bound to
  the existing `editor-track-rule-optional` key (matching `JobRow.vue`'s
  `<progress :aria-label="$t(...)">` precedent for a labelless native
  control).
- **Drag-reorder**: native HTML5 DnD on each `<tr>` (`draggable="true"`,
  `@dragstart`/`@dragover.prevent`/`@drop`), a closure `dragIndex` (no
  `dataTransfer` read) exactly mirroring `ListWidget.vue`'s mechanics. On
  drop, `tracks.rules` is rebuilt immutably (splice-out/splice-in on a copy)
  and the whole profile is re-emitted via `update:modelValue` -- a semantic
  model edit, not a DOM mutation, per the binding note.
- No sections, no widget dispatch, no save/open IPC, no `App.vue` nav wiring
  -- all deferred to Tasks 12-13 as scoped.

`e2e/smoke.spec.ts` (extended): one new `test.describe` block, mounting
`EditorView` through the Task-10 harness (`mountComponent(page, { component:
"EditorView", props: { modelValue: <two-rule profile> } })`) with a
two-rule fixture differing by `match.exact.type` (a real matchable property,
`capability/mod.rs::TYPE_VALUES`). Asserts both rows render in order, then
drag-reorders row 0 onto row 1 via Playwright's documented programmatic-DnD
pattern (`page.evaluateHandle(() => new DataTransfer())` shared across a
`dragstart` dispatch on the source row and a `drop` dispatch on the target
row -- verified against current Playwright docs via context7, since the
component's own handlers never read `dataTransfer` but the officially
documented pattern is followed regardless as the correct cross-browser way
to fire these two event types synthetically), and asserts both the rendered
row swap and `readModel(page)`'s `tracks.rules` order.

## TDD evidence

**RED** (`pnpm test:e2e -- -g "editor view: rule grid"`, before
`EditorView.vue` existed):

```
Error: page.evaluate: Error: unknown mount component "EditorView"
  at .generated/mount-harness.js:6873
    at mountComponent (.../e2e/mount.ts:24:14)
    at .../e2e/smoke.spec.ts:796:5
1 failed, 19 passed
```

Exactly the specified RED (the mount driver's `unknown mount component`
throw); all 19 pre-existing tests stayed green.

**GREEN** (`pnpm test:e2e`, full suite, after implementation): 20 passed, 0
failed, including the new test.

## Gate results (all FOREGROUND, no subsets)

Nine-part gate:

1. `cargo fmt --all --check` -- clean (no output)
2. `cargo clippy --workspace --all-targets -- -D warnings` -- clean
3. `cargo test --workspace` -- 81 passed (core) + all other crates 0/0 or
   passing, 2 codegen tests passed, doc-tests 0/0 across all crates
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- clean
5. `cargo deny check` -- `advisories ok, bans ok, licenses ok, sources ok`,
   exit 0
6. `pnpm lint` -- clean (D27 `no-raw-text` included; no violations)
7. `pnpm build` -- `vue-tsc --noEmit && vite build` clean
8. `pnpm check:i18n` -- ok (34 source files, 229 catalog ids, 18 pre-existing
   unused-key warnings unrelated to this task's keys -- none of the five
   reused `gui-editor.ftl` keys appear in the unused list, confirming they
   are now detected as referenced; 1 other locale checked for parity)
9. `pnpm test:e2e` -- 20 passed, 0 failed

Rust-side items (1-5) touch nothing this task changed (frontend-only diff);
run in full per the binding "no subsets" instruction regardless.

## Files changed

- `src/views/EditorView.vue` (new)
- `e2e/smoke.spec.ts` (extended: one new `import type { Profile }`, one new
  `test.describe` block)

Commit: `af9ebc3` "gui: the profile editor's rule grid with drag-reorder
(D45)" on branch `plan6-e`, worktree
`/home/senol/Git/Muxsmith/.worktrees/plan6-e`. Not pushed.

## Self-review

- **Mounts from `modelValue` alone**: confirmed. No `onMounted`, no IPC call,
  no `load_profile` anywhere in the file; `defineModel<Profile>()` is the
  sole source of state.
- **Reorder is a model emit, not a DOM mutation**: confirmed. `onDrop`
  constructs a new `rules` array and a new `Profile` object
  (`{ ...model.value, tracks: { ...model.value.tracks, rules: nextRules } }`)
  and assigns it to `model.value`, which `defineModel` emits as
  `update:modelValue`; nothing manipulates the DOM directly for the reorder
  itself (Vue's own re-render handles the visual swap from the new array
  order).
- **No validation**: confirmed. No semantic checks anywhere (no uniqueness,
  no type checks, no required-field checks); the view holds and displays
  whatever `modelValue` it is given.
- **No new/changed catalog keys**: confirmed via `git diff -- locales/*/gui-editor.ftl`
  (empty) and a line count (43 keys in both `locales/en/gui-editor.ftl` and
  `locales/de/gui-editor.ftl`, unchanged).
- **Grid strings all from existing keys or tokens**: confirmed. Five `$t()`
  calls in the template, all five keys pre-existing in `gui-editor.ftl`
  (`editor-tracks-rules`, `editor-track-rule-source`, `-match-expr`,
  `-optional`, `-changes`). All other rendered text is raw model data (an
  external path, matchable/settable property names, scalar values, a
  domain-default keyword token) via plain `{{ }}` interpolation of script
  functions, not template text nodes, and thus outside the `no-raw-text`
  lint's scope by construction (confirmed empirically: `pnpm lint` is clean).

## Concerns

None blocking. Two judgment calls worth flagging for the controller/Task 12
author's awareness, both within the standing structural-conformance grant
(no wire-format, API-surface, or test-surface effect):

1. **Row summary formatting is this task's own invention** (e.g.
   `type=video`, `any(2)`, comma-joined change keys) -- structural punctuation
   over raw domain data, not translatable prose, matching
   `ResolutionTable.vue`'s `resolvedTrackLabel` precedent
   (`` `${a.track_id} (${a.track_kind})` ``). Task 12's full per-rule detail
   editor (widget dispatch) will likely make these grid-row summaries
   redundant/superseded rather than reused; noting this so it is not
   mistaken for a locked format the next task must preserve.
2. **Drag test technique**: since `EditorView`'s handlers never read
   `dataTransfer`, the test's shared `DataTransfer` JSHandle is technically
   inert for THIS component, but was kept anyway as the officially
   documented Playwright pattern for synthetic `dragstart`/`drop` dispatch
   (verified via context7 against current Playwright docs) rather than an
   ad hoc simplification, in case a future task's drag target ever inspects
   it.

## Fix round (review verdict: Needs fixes -- one Important, one Minor)

**Important -- missing table caption.** `src/views/EditorView.vue`'s rule
grid `<table>` had no `<caption>`, breaking a 3-for-3 house pattern
(`ResolutionTable.vue:34`, `JobsView.vue:270`, `RunHistory.vue:193` all
caption their tables with a `$t()` accessible name). Fixed by adding

```html
<caption>
  {{ $t("editor-tracks-rules") }}
</caption>
```

as the table's first child, reusing the exact key already on the view's
`<h2>` -- zero new catalog keys. Checked all three precedents for a
visually-hidden variant before adding it visible: none of the three hides
its caption (all render plain, visible text identical in kind to their
neighbouring heading), so per the coordinator's instruction the new caption
stays visible too, matching them exactly rather than inventing a new
(hidden-caption) pattern.

**Minor -- broken `expect.poll` consistency.** `e2e/smoke.spec.ts`'s
drag-reorder assertion read `readModel(page)` directly
(`const model = (await readModel(page)) as Profile;` then a bare `expect(...)`),
breaking the file's otherwise-unbroken `expect.poll(() => readModel(page))`
pattern (ten other call sites, e.g. lines 615, 626, 637, 648...). Rewrapped:

```ts
await expect
  .poll(async () => {
    const model = (await readModel(page)) as Profile;
    return model.tracks.rules.map((r) => (r.match.exact as Record<string, unknown> | null | undefined)?.type);
  })
  .toEqual(["audio", "video"]);
```

The `.map()` transform needed for this assertion (unlike the file's other,
simpler `expect.poll` call sites) is kept inside the poll callback itself,
so the read-and-poll shape matches even though the assertion body is
slightly richer.

**Diff scope check.** `git diff -- src/views/EditorView.vue e2e/smoke.spec.ts`
confirms exactly these two changes (a 3-line caption addition, a 4-line ->
6-line assertion rewrap); `gui-editor.ftl` key counts unchanged at 43/43
in both locales (`grep -cE "^[a-z][a-z0-9-]* =" locales/{en,de}/gui-editor.ftl`).

**Gate re-run (foreground, full nine-part, no subsets):**

1. `cargo fmt --all --check` -- clean
2. `cargo clippy --workspace --all-targets -- -D warnings` -- clean
3. `cargo test --workspace` -- all crates passing (unchanged from the first
   round; this fix touches no Rust)
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- clean
5. `cargo deny check` -- `advisories ok, bans ok, licenses ok, sources ok`
6. `pnpm lint` -- clean (caption text node passes `no-raw-text` the same
   way every other `$t()`-driven text node in the codebase does)
7. `pnpm build` -- `vue-tsc --noEmit && vite build` clean
8. `pnpm check:i18n` -- ok, same 18 pre-existing unrelated unused-key
   warnings, none of this task's keys among them
9. `pnpm test:e2e` -- **20 passed, 0 failed** (all pre-existing tests plus
   the Task 11 test, still green with the caption present and the
   `expect.poll`-wrapped assertion)

Commit: `35d844d` "gui: fix-round - rule grid table caption + expect.poll
consistency (D45)", staged explicitly (`src/views/EditorView.vue`,
`e2e/smoke.spec.ts`), on top of `af9ebc3` on branch `plan6-e`. Not pushed.
