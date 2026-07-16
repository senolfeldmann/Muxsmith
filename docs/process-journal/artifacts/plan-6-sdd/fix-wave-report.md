# Plan 6 whole-branch fix wave: report

Worktree `.worktrees/plan6-e`, branch `plan6-e`, base `bf46932`. Executes
the eight mechanical items from `whole-branch-verdict.md`'s fix-wave list,
plus a ninth the controller authorized mid-wave after item 1's mandated
test surfaced a genuine, pre-existing, unrelated defect (see "Item 9"
below). All nine items implemented, gate-clean, committed in one commit.

## Per-item results

### Item 1: T13 served-app fixture + row selection + panel + a11y scan

`e2e/smoke.spec.ts`, describe `editor view: open/save (Task 13, D45/D41)`.

- `editorProfile.tracks.rules` changed from `[]` to two real rules
  (`{ match: { exact: { type: "video" } } }`, `{ ... type: "audio" ... }`),
  mirroring the T13b mount-harness fixture.
- In the main test, after "Save starts enabled", added: select the first
  row (`editor-rule-select`), assert `editor-rule-detail` visible with its
  four `trackRule` fields (source combobox, optional checkbox, changes
  group, match-expr group), then `assertNoSeriousA11yViolations(page)` on
  that composed state (grid + selection + panel, first served-app render
  of this state per `fixture-reachable-states-need-one-served-render`).
- **Red-run verified (panel assertion)**: forced `v-if="false &&
  selectedRule"` in `EditorView.vue`, rebuilt, ran the test -- failed
  exactly at the panel `toBeVisible()` assertion (`e2e/smoke.spec.ts:1207`).
  Reverted, rebuilt, confirmed clean.
- **Second red-to-green cycle, this time on the a11y assertion itself**:
  with the panel assertion fixed and real content in the fixture, the same
  test went red again -- not from a bug in the new assertions, but because
  the axe scan legitimately caught a real defect one directory over
  (`PropertyMapWidget.vue`, see item 9). Confirmed red first (`label`/
  critical violation, two nodes), then green after item 9's fix, both
  captured verbatim below.
- **Final status: GREEN.** `e2e/smoke.spec.ts:1165` passes in the full
  suite run (see gate table).

### Item 2: multiline widget asserts `<textarea>`

`e2e/smoke.spec.ts:767-781`. Added, after the existing `toHaveValue`
assertion: `expect(await field.evaluate((el) => el.tagName)).toBe(
"TEXTAREA")`, with a comment explaining why the role alone doesn't
discriminate.

- **Red-run verified**: forced `v-if="false && spec.widget.multiline"` in
  `TextWidget.vue`, rebuilt the mount harness, ran the test -- failed with
  `Expected: "TEXTAREA", Received: "INPUT"` at the new assertion. Reverted,
  rebuilt, confirmed clean.
- **Covering test**: `e2e/smoke.spec.ts:767` (`text widget (multiline)
  renders a textbox for a multiline field`) -- **PASS**.

### Item 3: stale `dragIndex` on aborted drag

`src/editor/widgets/ListWidget.vue`, `src/views/EditorView.vue`.

- `ListWidget.vue`: added an `onDragEnd` handler (`dragIndex = null`) wired
  to `@dragend` on the reorderable row `<div>`; also reset `dragIndex` in
  the early-return branch of `onDrop` (previously only the real-reorder
  path reset it).
- `EditorView.vue`: `onDrop`'s early-return branch already reset
  `dragIndex`; added the same `onDragEnd` handler wired to `@dragend` on
  the grid row `<tr>`, for the same reason (a drag that leaves the surface
  or is cancelled fires no `drop` at all).
- **Covering test**: `e2e/smoke.spec.ts:935` (`editor view: rule grid +
  drag-reorder (Task 11, D45) > renders tracks.rules in order; a
  drag-reorder swaps the rows and updates the held model`) -- **PASS**,
  unaffected (dispatches `dragstart`/`drop` only, doesn't exercise
  `dragend`, so this is a non-regression check, not a positive
  demonstration of the fix).
- **No existing e2e spec exercises `ListWidget.vue`'s own drag-and-drop
  path** (no test dispatches `dragstart`/`drop` on a `reorderable: true`
  list, e.g. `attachments.rules`) -- grepped the whole spec file for
  `reorderable`/`dataTransfer`, only the `EditorView` grid test uses
  `dataTransfer`. The `ListWidget.vue` side of this fix is verified by
  `vue-tsc` (via `pnpm build`) and `eslint` (via `pnpm lint`), both clean,
  plus manual code reading -- not by a red/green e2e demonstration. Flagging
  this honestly rather than claiming coverage that doesn't exist.

### Item 4: `PropertyMapWidget.vue` dup-key collapse comment

Added one comment block above `setKey` (which covers `setValue` and
`removeRow`'s identical `Object.fromEntries` rebuild by the same
reasoning) documenting: a rename via `setKey` that collides with another
row's key silently collapses to the last write, and this is left uncaught
deliberately (spec 7, zero frontend semantic validation) since core
diagnoses the resulting duplicate/missing property against the saved YAML.

- Comment-only; no template/logic change. Verified via `vue-tsc` (clean)
  and `eslint` (clean); no test impact expected or observed.

### Item 5: apply button disabled while any batch action is in flight

`src/components/SuggestionCard.vue`, `src/views/BatchView.vue`.

- `SuggestionCard.vue`: added a `busy?: boolean` prop (batch-wide "any
  action in flight", distinct from `applying`, this card's own round
  trip). Template: `:disabled="busy"` (was `:disabled="applying"`);
  `:aria-busy="applying"` unchanged (stays clicked-card-only).
- `BatchView.vue`: passes `:busy="busy"` (the existing `validating ||
  dryRunning || applyingIndex !== null` computed) alongside the existing
  `:applying="applyingIndex === i"`.
- Net effect: previously only the clicked card's apply button was disabled
  during its own round trip, and no card was disabled during a
  validate/dry-run; now every apply button is disabled for the whole
  window any batch action is in flight, matching the busy idiom every
  other control in this view already follows (Run, Dry run, profile pick,
  etc. all bind `:disabled="busy"` or a superset).
- **T14 assertion check**: grepped for `toBeEnabled`/`toBeDisabled` on the
  apply button in the T14 test (`e2e/smoke.spec.ts:406-474`) -- none exist,
  only `not.toHaveAttribute("aria-busy", "true")` after the round trip
  completes, which is unaffected (different attribute, unchanged binding).
  No adjustment needed.
- **Covering test**: `e2e/smoke.spec.ts:406` (T14 apply-wiring test) --
  **PASS**.

### Item 6: `console.error` on the null-profile/no-diagnostic contract violation

`src/views/BatchView.vue:240-263` (`onApplySuggestion`'s `!doc.profile`
branch). Previously: if `doc.config_diagnostics[0]` was falsy, the
function returned silently (no-op). Now: an `else` branch logs
`console.error("[batch] load_profile returned profile: null with no
diagnostics", selectedProfile.value)`, documented as a D42 envelope
contract violation with no user-facing string to show (nothing else
exists to surface).

- No existing e2e test drives `load_profile` to return `profile: null`
  with empty `config_diagnostics` for the apply flow -- this exact branch
  has no dedicated spec either before or after the change. Verified by
  `vue-tsc`/`eslint` (clean) and code reading; the existing T14 test
  (`e2e/smoke.spec.ts:406`) still passes unchanged (it doesn't touch this
  branch, `doc.profile` is always present in that fixture).

### Item 7: `locales/de/gui-editor.ftl` comment depth

Lines 1-8 changed from `##` (Fluent GroupComment) to `#` (Comment), to
match `locales/en/gui-editor.ftl`'s depth for the equivalent file-level
doc comment (the `## Profile (top-level sections)` group comment at line
10 is unchanged, matching en's own `## Profile (top-level sections)`).

- **Covering test**: `pnpm check:i18n` -- **PASS** ("233 catalog ids, ...
  1 other locale(s) checked for parity against 7 en/ catalog(s)", no
  parity errors). Verified `gui-editor` stays 45 en / 45 de (`grep -cE
  '^[a-z][a-z0-9-]* =' locales/{en,de}/gui-editor.ftl`), `gui-batch`
  untouched (39/39, and `git diff --stat -- locales/` shows only
  `gui-editor.ftl` changed).

### Item 8: `planner.rs` stale line-ref comment

`crates/muxsmith-core/src/planner.rs:1917-1918`. Verified the actual
current locations of the two `track_name` inserts the comment describes
(`grep -n track_name`): lines 1837 and 1843, inside `delta_for`'s
`AddSubstring`/`AddNotSubstring` arms (not `:1824, :1829` as the stale
comment claimed). Per the house entry
`comments-symbol-refs-not-line-refs` (`docs/decision-ledger.yaml:3491`,
which explicitly recommends "naming the construct ... over its
coordinates"), dropped the line numbers entirely rather than updating
them to the new ones:

```rust
// The match key an edit targets: the named property for the two exact variants,
// the fixed `track_name` key for the two substring ones (`delta_for`'s two
// substring arms).
```

- **Covering tests**: `cargo test -p muxsmith-core` -- **PASS** (119 unit
  tests + 21 integration test binaries, 0 failed); `cargo fmt --all
  --check` -- clean; `cargo clippy --workspace --all-targets -- -D
  warnings` -- clean.

### Item 9: label `PropertyMapWidget.vue`'s key/value inputs (controller-routed)

**Origin.** Item 1's mandated axe scan, run for the first time against a
served-app render of a populated `propertyMap` widget, caught a real,
pre-existing, critical accessibility defect unrelated to any of the eight
original items:

```
label (critical): Form elements must have labels
  input[data-testid="property-map-key"]    <input data-testid="property-map-key" type="text" value="type">
  input[data-testid="property-map-value"]  <input data-testid="property-map-value" type="text" value="video">
```

Neither the free-text property-name input nor the typed value input in a
`propertyMap` row had any accessible name (no `<label>`, `aria-label`,
`aria-labelledby`, or `title`); the enclosing `<fieldset><legend>` groups
the widget but does not label individual rows. `git blame` traced both
input elements to commits `c50866a7` and `0ba894a6` (Task 10 / Task 12a),
well before this fix wave, and confirmed via `git diff` that item 4's own
touch of this file was comment-only. I reported this as NEEDS_CONTEXT
rather than fixing or routing around it unilaterally, since it was a
ninth, unscoped, user-visible-surface decision. The controller routed it
back as an authorized ninth item (internal-technical lane, no new
user-visible wording), with a hard constraint: zero new Fluent keys, zero
new English/German strings, no `aria-label` with literal prose.

**Design chosen: `useId()` + `aria-labelledby`, reusing existing text and
values.** `src/editor/widgets/PropertyMapWidget.vue`:

- The `<fieldset>`'s `<legend>` (already rendering `$t(spec.labelKey)`,
  e.g. "Changes"/"Match expression") gets an `:id="legendId"`
  (`legendId = useId()`), the same `useId()` primitive `TextWidget.vue`'s
  `useId()`+`<label :for>` pattern already uses elsewhere in this codebase
  -- adapted here because a single `<legend>` can label the whole fieldset
  but not, via `for`, more than one input per row, so `aria-labelledby`
  stands in for the missing 1:1 `<label for>` relationship.
- Each row's key input gets `:id="keyInputId(index)"` (`keyIdBase =
  useId()`, `` `${keyIdBase}-${index}` ``) and `:aria-labelledby="legendId"`
  -- its accessible name becomes the widget's own existing heading (e.g.
  "Changes").
- Each row's value input (all four typed variants: checkbox/integer/
  float/text) gets `:aria-labelledby="`${legendId} ${keyInputId(index)}`"`
  -- two IDREFS. Per the WAI-ARIA accessible-name computation, a
  `aria-labelledby` reference to another textbox includes that textbox's
  live VALUE in the computed name, so a row with key "forced" gives its
  value control the name "Changes forced": the property name the user
  already typed, not new copy, and it distinguishes rows from each other
  for free. Verified this actually works in the real accname
  implementation (not just per my reading of the spec) via the axe
  scan below, rather than trusting the spec text alone.
- Zero new catalog keys, zero new strings, zero `aria-label` literals --
  every accessible name traces to `$t(spec.labelKey)` (already rendered)
  or a row's own already-typed key value.

**Red confirmed, then green (the axe scan as witness).** Ran item 1's
test against the pre-fix code first: failed with exactly the `label`
violation quoted above (`e2e/smoke.spec.ts:1214`,
`assertNoSeriousA11yViolations`). Applied the fix, rebuilt (`pnpm build`
picked up the change, JS bundle hash changed from `index-C1TMWD85.js` to
`index-C-xlZges.js`), reran the identical test in isolation:

```
✓ 1 [chromium] › e2e/smoke.spec.ts:1165:3 › editor view: open/save (Task 13, D45/D41) › the nav opens the editor; ...
1 passed (1.4s)
```

Red-to-green confirmed on the real axe engine, not assumed from the ARIA
spec text.

**Lint cleanup.** `pnpm lint` flagged `vue/singleline-html-element-
content-newline` on the now-attributed `<legend :id="legendId">` (0
errors, 2 warnings); applied `eslint --fix`, which reformatted the legend
across three lines (`<legend :id="legendId">` / `{{ $t(spec.labelKey) }}`
/ `</legend>`) with no functional change -- confirmed the compiled dist
JS hash was byte-identical before and after (`index-C-xlZges.js` both
times, since Vue's template compiler normalizes interpolation whitespace).
`pnpm lint` is now silent (0 errors, 0 warnings).

**Covering tests**: item 1's a11y scan (above, the actual witness for
this fix) plus `e2e/smoke.spec.ts:866` (T10 propertyMap add/remove test,
unaffected -- still locates rows by `data-testid`, doesn't touch
`aria-labelledby`) and `e2e/smoke.spec.ts:1040`/`:1060` (Task 12
propertyMap typed-cell tests) -- all **PASS** in the full suite run.

## Full gate run (all nine parts, foreground, final state)

Run in order against the exact final source (post item-9, post
`eslint --fix`), each part run fresh:

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | PASS (no output) |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| 3 | `cargo test --workspace` | PASS (0 failed across all unit + integration binaries) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | PASS |
| 5 | `cargo deny check` | PASS (`advisories ok, bans ok, licenses ok, sources ok`) |
| 6 | `pnpm lint` | PASS (no output) |
| 7 | `pnpm build` | PASS |
| 8 | `pnpm check:i18n` | PASS (233 catalog ids, parity ok, gui-editor 45/45) |
| 9 | `pnpm test:e2e` | **PASS: 27/27** |

Typography: `git diff -U0` over the entire final diff, grepped for the
banned glyph class (em/en-dash, curly quotes, ellipsis, NBSP, Unicode
minus) -- zero hits, pattern validated first against a known-present
em-dash control. De catalog change uses only real umlauts/ASCII, no
transliteration needed.

## Files changed (git diff --stat, final)

```
crates/muxsmith-core/src/planner.rs      |  4 +--
e2e/smoke.spec.ts                        | 34 +++++++++++++++++-
locales/de/gui-editor.ftl                | 16 ++++-----
src/components/SuggestionCard.vue        | 13 +++++--
src/editor/widgets/ListWidget.vue        | 11 ++++++
src/editor/widgets/PropertyMapWidget.vue | 60 +++++++++++++++++++++++++++-----
src/views/BatchView.vue                  | 12 +++++++
src/views/EditorView.vue                 |  9 +++++
8 files changed, 138 insertions(+), 21 deletions(-)
```

Same 8 files as the original eight-item wave (item 9 lands in
`PropertyMapWidget.vue`, already touched by item 4). No `dist/`,
`e2e/.generated/`, or `test-results/` artifacts staged (all gitignored,
confirmed via `git status --short --ignored`).

## Status: committed

One commit, `6674089` ("gui/core: whole-branch fix wave (plan 6, verdict
waves 3+4), 9 items"), unsigned (`git -c commit.gpgsign=false`), all eight
changed files explicitly staged by name (never `git add -A`), trailer
`Co-Authored-By: Claude Sonnet <noreply@anthropic.com>`. Not pushed.
Working tree clean post-commit (`git status --short` empty).
