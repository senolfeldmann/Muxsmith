# Plan 7.5 design: track-rule add/remove in the profile editor

Status: DRAFT 2026-07-22. Numbering starts at **D65**; the last existing
ADR is D64 (`2026-07-21-plan7-help-i18n-design.md`), verified by sweeping
`^## D` across `docs/superpowers/specs/` at write time. Plan 8's parallel
design owns D75 upward; no spec file dated 2026-07-22 existed in
`docs/superpowers/specs/` at write time, and this document uses D65-D72
only - no collision is possible.

Scope per the ROADMAP Plan-7.5 anchor and its S22 KICKOFF block
(`docs/ROADMAP.md`, "Plan 7.5" section): the editor edits and
drag-reorders existing `tracks.rules` entries and can add/remove
ATTACHMENT rules through the generic `ListWidget`, but offers no way to
create or delete a TRACK rule; spec 8.2 lists no such affordance either.
Surfaced by the plan-7 T10 help-content review
(`docs/process-journal/artifacts/plan-7-sdd/task-10-verdict.md`
finding 1): the help topics fabricated Add/Remove buttons that did not
exist. This design makes them exist.

**Five owner rulings (2026-07-22, S22) are binding and not re-litigable**;
each is recorded as a D-entry below with its rationale (D65-D69). The
design decisions on top of them are D70-D72. **Every fork in this
document is closed**; no design-latitude clause appears in either form
(explicit permission or omission), per `proc-latitude-clause-boundary`.
The one deferred surface - final user-visible topic wording - is covered
by the standing owner rendered-surface pass (the D54 route,
`latitude-carveout` occurrence 2026-07-16), with the claims each sentence
must carry enumerated in D71 so nothing is invented at the keyboard.

Grounding: v1 design spec §8.2/§8.3/§5.2/§4.5 (authoritative); Tier-2
`docs/product-boundaries.yaml` (`editor-generic-action-keys`,
`gui-closed-domain-dropdowns`, `help-mode-suppression-pointer-scope`,
`core-83-zero-rule-keep-passthrough`), `docs/conventions.yaml`
(`help-topic-h1-scheme`, `code-comment-line-citations-drift`,
`content-claims-anchor-bound`), `docs/process-conventions.yaml`
(`proc-04-spec-wins`, `proc-06-mkvtoolnix-parity`,
`proc-no-work-needed-check`, `proc-proposed-safeguard-stays`,
`proc-latitude-clause-boundary`); ledger `gui-helpid-equals-labelkey`;
plan-7 design D51-D57 (help mechanics, marker anchoring); mkvtoolnix
source at v100.0 (`~/Downloads/mkvtoolnix`, `NEWS.md` header
"Version 100.0", the same tree the plan-7 parity run matched against the
binary 2026-07-21; no muxing-semantics surface exists in this plan, so
source citation suffices per `proc-06`). Every empirical claim below was
produced by running the repo's own debug CLI or reading the current tree
on 2026-07-22.

---

## 0. Corrections to the brief

Each brief claim was checked against the tree before anything was built
on it (`proc-57-briefs-not-ground-truth`). **No correction is needed**;
the load-bearing claims verified true:

- The T10 finding-1 quote and mechanics: verified verbatim in
  `task-10-verdict.md` finding 1; the generic buttons indeed live only in
  `ListWidget.vue` and `PropertyMapWidget.vue` (grep over `src/`: exactly
  those two files consume `editor-action-*`), and `EditorView.vue`'s
  bespoke grid renders no add/remove control.
- The 46-key budget: `gui-editor.ftl` carries exactly 46 message ids in
  en and 46 in de (counted with check-i18n's own `^id =` regex shape).
- `editor-action-add`/`editor-action-remove` exist
  (`locales/en/gui-editor.ftl` "Generic list/map actions" block; de
  "Hinzufügen"/"Entfernen") and carry **no** `.tooltip` attribute in
  either locale (read directly; the same file's `editor-tracks-rules`
  message at en:100-101 DOES carry one, serving as the positive control
  for the read).
- `help-mode-suppression-pointer-scope`, `core-83`, `gui-closed-domain-
  dropdowns`, `help-topic-h1-scheme`, `code-comment-line-citations-drift`
  all read at their entries; statements as the brief summarized.

One precision worth recording (not a correction): ruling 1's
"invalid-until-filled" is, at the measured code level, **warning**
severity, not error - see D65, which records the verified semantics and
why they satisfy the ruling.

---

## 1. Verified ground truth

Established by reading the current tree and running the repo's binaries,
not by reading about them.

**The bespoke grid and its detail panel** (`src/views/EditorView.vue`).
The grid is a `<table>` inside a `<fieldset>` whose `<caption>` carries
`data-help-id="editor-tracks-rules"` and the caption-level diagnostic
marker anchored at exactly `"tracks.rules"` (`rulesCaptionDiags`). Rows
(`data-testid="editor-rule-row"`) are draggable (`onDragStart`/`onDrop`/
`onDragEnd`), carry an ordinal cell (`index + 1`, D59), a row-selection
button (`data-testid="editor-rule-select"`, native `<button>` with
`:aria-current`), and a row marker anchored at exactly `` `tracks[${index}]` ``.
Selection state is `selectedIndex` (`ref<number | null>`); `onDrop`
clears it ("A reorder invalidates any prior selection index mapping to a
rule identity, not a position" - the in-file rationale D66/D67 reuse).
The detail panel (`data-testid="editor-rule-detail"`) renders
`v-if="selectedRule"`: a `SectionWidget` over the `trackRule` registry at
root path `` `tracks[${selectedIndex}]` `` with `suppress-self-anchor`,
writing back immutably via `setRuleValue`. Open/save/validate state:
`saveDisabled` gates on `hasErrors` (**error severity only**), plus
`!model`/`!currentPath`/`saving`/`opening`; the shallow `watch(model)`
revalidates every model swap through `validateProfileModel` (gated on
`currentPath`, so the bare mount harness never fires IPC).
**Both halves of this description are superseded by D107 decisions 3(a)
and 3(b)** (`docs/superpowers/specs/2026-07-30-plan-12-decisions.md`):
`saveDisabled` no longer reads `!currentPath.value` (it is
`!model.value || hasErrors.value || saving.value || opening.value`,
`EditorView.vue:330-332`), and the `watch(model)` revalidation gate moved
from `currentPath` to `sessionActive` (`EditorView.vue:349`). The
paragraph stands as the record of the mechanism as this task shipped it.

**The ListWidget add/remove precedent**
(`src/editor/widgets/ListWidget.vue`). `addItem()` appends `{}`
immutably; `removeItem(index)` splices; the remove button renders per
item, the add button after the items, both inside the widget's
`<fieldset>`, both rendering the generic keys, neither carrying a
tooltip. `PropertyMapWidget.vue` rows follow the same shape.

**The empty skeleton's emission, run, not assumed.** Fixtures were run
through the repo's own debug CLI (`target/debug/muxsmith validate
--json`, 2026-07-22):

- `rules: [ { match: {} } ]` emits **exactly one** diagnostic:

  ```json
  {"code":"empty-match-expression","config_path":"tracks[0].match",
   "params":{},"severity":"warning"}
  ```

  rendered "This match expression is empty and would match every track."
  Exit code 1 (warnings). Code path: `validate()` in
  `crates/muxsmith-core/src/profile/validate.rs`, the
  `rule.match_expr.is_empty()` branch (`Diagnostic::warning(DiagCode::
  EmptyMatchExpression, format!("{base}.match"))`); `MatchExpr::is_empty`
  in `profile/match_expr.rs` is true when all five parts are absent or
  empty.
- The skeleton **beside** an exact-match rule
  (`- match: { exact: { language: de } }` then `- match: {}`) emits the
  same single warning at `tracks[1].match` and **nothing else** - no
  `ProvableOverlap`, because `lint.rs::is_exact_only` requires a
  non-empty `exact` map (`e.exact.as_ref().is_some_and(|m| !m.is_empty())`),
  so an empty expression never enters the overlap scan.
- `rules: []` with `unmatched` defaulted (`drop`) emits
  `no-track-rules` **error** at `tracks.rules`, rendered "The profile
  defines no track rules; add at least one rule, or set
  tracks.unmatched: keep for a pure passthrough remux." Exit 2.
- `rules: []` with `unmatched: keep` emits `passthrough-profile`
  **info** at `tracks.rules`, rendered "This profile defines no track
  rules and tracks.unmatched is keep: a pure passthrough remux; every
  primary track is copied unchanged. If this is not intended, add track
  rules." Exit 0.

**The skeleton's Rust-side shape.** `TrackRule` in
`crates/muxsmith-core/src/profile/model.rs`: `source` defaults to
`SourceCfg::primary()` (`#[serde(default = "SourceCfg::primary",
skip_serializing_if = "is_primary")]`), `optional` defaults `false`
(`skip_serializing_if = "is_default"`), `changes` defaults `None`
(`skip_serializing_if = "Option::is_none"`), `match_expr` is required
(serialized key `match`). Consequence: the frontend skeleton
`{ match: {} }` deserializes to source=primary/optional=false/
changes=None, and a canonical save (D41/D48) writes it back as
`- match: {}` - the exact YAML form the fixture round-trip above loaded.

**The skeleton's TS-side shape.** Generated `src/bindings/profile.ts`:
`TrackRule` requires only `match: MatchExpr`; all five `MatchExpr` parts
are optional, so the object literal `{ match: {} }` typechecks as
`TrackRule` with **no cast** (unlike `ListWidget.addItem`'s untyped `{}`
against `unknown[]`). Grid rendering of the skeleton row:
`sourceSummary` falls back to `SOURCE_KEYWORDS[0]` = `"primary"`
(`src/bindings/keywords.ts`), `matchSummary` and `changesSummary` render
empty strings, the optional checkbox is unchecked, the ordinal is the
new row count.

**Where a `tracks[{i}].match` marker lands.** The detail panel's root
`SectionWidget` suppresses its own anchor and builds
`childPath("match")` = `` `tracks[{i}].match` ``; the `match` field
dispatches as `{ kind: "section", of: "matchExpr" }`, and that nested
`SectionWidget` anchors its own path at its legend (D57 exact-string
equality, `useDiagAnchor`). The grid **row** marker anchors only the
bare `` `tracks[{i}]` `` and therefore does NOT fire for the skeleton's
warning; the visible guidance is the open detail panel's Match-section
marker plus the never-filtered `DiagnosticsPanel`.

**Help-mode delegation mechanics** (`src/App.vue`). All help-mode
handlers are capture-phase; the pointer and hover handlers (mouseover,
focusin, click, dragstart) register on `<main>`, the keydown handler on
`document` (both in the `watch(helpMode)` block). `helpTarget()` resolves
`closest("[data-help-id]")` from the event target; `onHelpClick`
`preventDefault()`s and `stopPropagation()`s every click and pins the
resolved id; `onHelpKeydown` intercepts Enter/Space whenever
`helpTarget()` is non-null and pins. `EditorView`'s root `<section>`
carries `data-help-id="view-editor"`, so an unannotated element inside
the editor resolves to `view-editor` - the shipped fallthrough the
existing e2e already asserts (`e2e/help-mode.spec.ts`, "an unannotated
hover falls to the view topic").

**Verified negatives (each fire-verified per the house discipline).**

- Zero programmatic focus management anywhere in `src/`: grep
  `focus(|autofocus` (excluding `focusin`) returns nothing; the same
  pattern fires on a control file containing `el.focus()` and
  `autofocus`.
- No `.tooltip` attribute on either generic action key (both locales);
  positive control: `editor-tracks-rules`'s `.tooltip` in the same file.
- mkvtoolnix's attachment add/remove actions carry no tooltips:
  `setToolTip` appears once in `merge/attachments.cpp` (`:490`, on the
  list view itself: "Right-click for attachment actions"); positive
  control: 26 `setToolTip` hits in `merge/output.cpp`.
- No rule abstraction exists in mkvtoolnix's merge model: 0 of the
  `merge/*.h` headers contain the substring "rule" (case-insensitive)
  while 12 contain "track" (the control).
- `EmptyMatchExpression` is absent from spec 5.2's diagnostics table:
  grep over the v1 spec finds neither spelling, while the identical
  multi-file grep hits `locales/en/diagnostics.ftl:9` and the plan-7
  design's D57 emission table (the built-in positive control). The code
  is real (`report/mod.rs` `diag_codes!`:
  `EmptyMatchExpression => "empty-match-expression"`). Spec staleness,
  pre-existing; amendment 2 below closes it (`proc-04-spec-wins`:
  flagged and folded, not improvised around).

---

## Decision log

## D65: Owner ruling 1 - the fresh rule is the empty skeleton `{ match: {} }`, guided by the existing diagnostics plumbing

**Decision (ruling, binding).** A fresh rule is an EMPTY SKELETON,
invalid-until-filled; the existing diagnostics/inline-marker plumbing
guides the user. No prefilled guesses.

**The concrete value, closed here:** the frontend appends the object
literal `{ match: {} }` (typechecks as `TrackRule` with no cast, ground
truth). Nothing else is set: `source`, `optional`, `changes` stay absent
and fall to their serde defaults on the Rust side
(primary/false/None, `model.rs` `TrackRule` attributes).

**Verified semantics (empirical, section 1):** the skeleton emits exactly
one config-time diagnostic - `EmptyMatchExpression`, **warning**
severity, at `` `tracks[{i}].match` `` - and never `ProvableOverlap`
(the lint's `is_exact_only` gate excludes empty expressions). The marker
lands at the Match section legend inside the auto-opened detail panel
(D67), and the diagnostic is listed in the never-filtered panel; the
grid row marker (bare `` `tracks[{i}]` ``) deliberately does not fire -
exact-string anchoring, no normalization (D57).

**"Invalid-until-filled" means guided, not save-blocked - recorded
explicitly.** Warning severity leaves Save enabled (`saveDisabled` gates
on error severity only, D41's one sanctioned frontend affordance), and a
saved skeleton serializes as `- match: {}` (D48 default omission),
loading back to the same warning. This is correct, not a gap: an empty
match expression is *legal* profile syntax whose plan-time behavior is
"matches every track of its source" - on a single-track source that is a
legal unique match; on a multi-track source the dry run raises
`AmbiguousRule`. Error severity at config time would falsely reject the
single-track case; the warning (whose rendered text says exactly "would
match every track") is core's deliberate shape. The ruling's own
mechanism sentence ("the existing diagnostics/inline-marker plumbing
guides") names guidance, not blocking, and this design changes **no**
core severity.

**Rejected: prefilled guesses (the ruling's rejected alternative).**
Steelman: a prefilled starter condition (say `type: video`) saves
keystrokes for the most common rule, gives the fresh detail panel a
concrete value to show instead of empty widgets, and avoids the
transient warning entirely. Rejected by the owner: input-time magic
crosses the declarative-batch boundary (the same interactive-vs-
declarative line `proc-06` draws for parity); a guessed condition is a
plausible-but-unverified claim planted into the user's profile - valid,
silently wrong, and muxing the wrong track is strictly worse than a
visible warning; and the diagnostics plumbing already provides the
guided path from empty to valid, which is exactly what it is for.

**Interface changes:** none - no wire, no Rust, no catalog.

---

## D66: Owner ruling 2 - Remove has no confirmation; disabled without a selection; selection clears after removal

**Decision (ruling, binding).** Remove deletes the selected rule with NO
confirmation dialog. House precedent: `ListWidget.removeItem` deletes
per-item without confirmation; explicit save bounds the loss (D41 - an
unsaved model mutation touches no file); the durable answer to
accidental destruction is the v1.x editor-undo/redo entry (ROADMAP,
ruled S22), and **this design builds nothing toward undo/redo**.

**Resolved on top of the ruling:**

- The Remove button is disabled exactly when `selectedIndex === null`
  (`:disabled="selectedIndex === null"`). No other disable condition:
  Add and Remove do not gate on `opening`/`saving`, consistent with
  every other model-mutating control in the view (widgets and
  drag-reorder stay live during a save; the in-flight IPC serialized its
  snapshot already).
- After removal, `selectedIndex` is set to `null`, closing the detail
  panel. Same rationale as the in-file `onDrop` precedent: indices after
  the removed rule shift by one, so any retained index would silently
  point at a different rule - selection maps to an identity, not a
  position.
- Permitted interaction note (ruling allows noting, not building): the
  editor's immutable whole-model swaps (every mutation replaces
  `model.value`) are exactly the command/snapshot boundary the v1.x
  undo/redo entry names; nothing in this design obstructs it.

**mkvtoolnix parity (source-verified):** their selection-scoped
attachment removal is also unconfirmed - `Tab::onRemoveAttachments`
(`merge/attachments.cpp`) goes straight to
`removeSelectedAttachments(...selection())`, and the Del key is bound to
the same slot (`BasicTreeView::deletePressed` connect) - and their
remove action disables without a selection
(`enableAttachmentsActions`: `removeAttachmentsAction->setEnabled(hasSelection)`).
Classification: MATCH (section 2).

**Rejected: confirmation dialog.** Steelman: one unconfirmed click can
destroy a rule that took minutes of detail-panel work, the rows sit
adjacent to drag targets so slips are real, and a dialog is the cheapest
guard that exists today, while undo/redo is a v1.x promise. Rejected by
the owner: a dialog punishes every deliberate removal to guard the rare
accidental one, the explicit-save model already bounds the worst case to
the unsaved delta, mkvtoolnix removes list entries unconfirmed too, and
the durable fix is undo/redo, ruled v1.x - a dialog built now would be
throwaway UI the undo layer obsoletes.

**Rejected: auto-select a successor after removal.** Steelman: keeping a
live selection (the next row, as list UIs often do) makes repeated
deletion fast and keeps the detail panel populated. Rejected: successor
selection re-introduces exactly the index-identity ambiguity the house
pattern (`onDrop`) resolves by clearing - the panel would silently swap
to a rule the user never selected, which is the same defect class the
clearing comment documents; a re-selection click is cheap; and bulk
rule deletion is not a plausible flow at profile scale (a handful of
rules, spec 4.1's own reference example).

**Interface changes:** none.

---

## D67: Owner ruling 3 - Add appends at the end, auto-selects, opens the detail panel; no programmatic focus move

**Decision (ruling, binding).** Add APPENDS the skeleton at the end of
`tracks.rules`; the new rule is auto-selected and its detail panel
opens. Reordering stays the existing drag-reorder - no up/down buttons,
no insert-at-position.

**Resolved on top of the ruling:**

- Implementation shape (closed; mirrors `ListWidget.addItem` +
  `EditorView.onDrop`'s immutable rebuild):

  ```ts
  function addRule() {
    if (!model.value) return;
    const next = [...rules.value, { match: {} }];
    model.value = {
      ...model.value,
      tracks: { ...model.value.tracks, rules: next },
    };
    selectedIndex.value = next.length - 1;
  }

  function removeSelectedRule() {
    if (selectedIndex.value === null || !model.value) return;
    const next = [...rules.value];
    next.splice(selectedIndex.value, 1);
    model.value = {
      ...model.value,
      tracks: { ...model.value.tracks, rules: next },
    };
    selectedIndex.value = null;
  }
  ```

  The panel opens purely reactively (`v-if="selectedRule"` turns truthy
  when `selectedIndex` lands on the new row); no new panel state exists.
  The model swap triggers the existing `watch(model)` revalidation
  (when a profile is open), which produces D65's warning and marker.
- **Focus target: none - focus stays on the Add button.** No
  programmatic `.focus()` call, no `autofocus`. Grounds: the tree
  contains zero focus management anywhere (fire-verified negative,
  section 1), so introducing it here would be a lone deviation from the
  house pattern; and the keyboard path is already correct - in DOM
  order, Tab from Add reaches Remove, then the open detail panel's
  first control (the `source` keyword-or-block select).

**Rejected: move focus into the detail panel's first field.** Steelman:
create-then-edit flows conventionally focus the new form; it saves one
Tab stop, and screen readers announce the panel context immediately.
Rejected: zero focus-management precedent in the tree (a `nextTick` DOM
query would be the first of its kind, for one button); unsolicited focus
moves are their own defect class (stealing focus from a user who is
mid-double-click adding two rules); and the visible signals already
carry the state change - the panel appears, the new row's select button
carries `:aria-current`, and D65's marker sits in the opened panel.

**Rejected: insert after the selected row.** Steelman: a user refining
the middle of an ordered list (rule order IS output track order,
spec 4.5) wants the new rule near its neighbours, not at the end.
Rejected by the owner's ruling (append; reordering stays drag): append
is position-predictable, the auto-opened panel means the user is editing
content first, and drag-reorder is one gesture away with the fresh
selection cleared on drop by the existing mechanics.

**Interface changes:** none.

---

## D68: Owner ruling 4 - the buttons render the generic action keys; zero new label keys; the 46-id budget stands

**Decision (ruling, binding).** Both buttons render
`editor-action-add` / `editor-action-remove` (en "Add"/"Remove", de
"Hinzufügen"/"Entfernen") - the rule grid becomes the **third** consumer
site after `ListWidget.vue` and `PropertyMapWidget.vue`. ZERO new label
keys; this design adds **no** catalog id and **no** Fluent attribute in
any locale, so the `editor-generic-action-keys` budget stays exactly 46
ids per locale (counted, section 0) and **no key-budget ADR or
occurrence is needed** - the default (none) applies. The boundary
entry's statement should still learn the third site: registered as
trigger 1 for the controller when consuming this design (mirroring the
plan-7 trigger-10 precedent of updating the entry's statement in place).

**Rejected: dedicated `editor-track-rule-add`/`-remove` keys.**
Steelman: site-specific wording ("Add rule" / "Remove rule") is more
explicit than bare "Add"/"Remove", and dedicated keys decouple a future
rewording of the grid pair from the list/map sites - the exact latent
coupling the `editor-generic-action-keys` entry's own steelman records
against cross-purpose key reuse. Rejected by the owner ruling: zero new
label keys; the budget is a hard boundary against prose growth; the
buttons sit inside the Rules fieldset directly beneath the rules table,
so context disambiguates the bare verbs; and at this scale the coupling
argument cuts the other way - one add/remove wording across the whole
editor is the consistency a user learns once.

**Interface changes:** none (no catalog change at all).

---

## D69: Owner ruling 5 - no last-rule protection; the zero-rule state is legal and its surfacing is verified

**Decision (ruling, binding).** Remove works down to zero rules. The
zero-rule state is legal, handled entirely by existing semantics
(`core-83-zero-rule-keep-passthrough`): under `tracks.unmatched: keep`
it is a legal pure-passthrough remux; under `drop` it is the
`NoTrackRules` error. No guard, no floor, no special-casing in the
editor.

**What the editor surfaces at zero rules - verified (empirical runs in
section 1, mechanics from `EditorView.vue`):**

- The grid renders its caption and headers with zero
  `editor-rule-row` rows; the detail panel is closed (selection was
  cleared by the removal, D66); the Add button remains, so the state is
  recoverable in one click.
- Under `drop` (the default): core emits the `no-track-rules` **error**
  at config path `tracks.rules`; the caption marker (anchored at exactly
  `"tracks.rules"`) renders as an error with the rendered message in its
  `title`; the diagnostics panel lists it; **Save disables**
  (`hasErrors`). The rendered text itself carries the recovery path:
  "add at least one rule, or set tracks.unmatched: keep for a pure
  passthrough remux."
- Under `keep`: core emits the `passthrough-profile` **info** at
  `tracks.rules`; the caption marker renders as info; **Save stays
  enabled** - deleting all rules and saving a passthrough profile is a
  sanctioned workflow, not an accident path, and the info text flags the
  accidental case ("If this is not intended, add track rules").
- The `editor-tracks-rules` help topic's "When the list may be empty"
  section already documents the legality in both locales (read at the
  current tree); D71's content pass extends it with how the state is
  reached.

**Rejected: disable Remove at one remaining rule.** Steelman: it blocks
the single most destructive click in the editor, and an empty-with-drop
profile can never produce output, so why allow reaching it. Rejected by
the owner ruling: zero rules under `keep` is a LEGAL, deliberately
supported product state (`core-83`: change only title/attachments/
chapters, or normalize the container) - a last-rule floor would make the
sanctioned passthrough workflow UNREACHABLE through the editor; the
`drop` case is already guarded by the existing error plus the save gate;
and the guard would special-case the editor against core semantics,
which is the wrong layer.

**Interface changes:** none.

---

## D70: Button placement, order, selection-scoped Remove, testids, keyboard reachability

**Decision.** One Add + one Remove button, rendered as native
`<button type="button">` elements **inside the rules `<fieldset>`,
immediately after `</table>`** (between the grid and the detail-panel
section that follows the fieldset). DOM and reading order: **Add, then
Remove**. Remove acts on the selected rule (D66); Add is never disabled
while the grid renders (the whole block sits inside `v-if="model"`).

Template shape (closed):

```html
<button
  type="button"
  data-testid="editor-rule-add"
  @click="addRule"
>
  {{ $t("editor-action-add") }}
</button>
<button
  type="button"
  data-testid="editor-rule-remove"
  :disabled="selectedIndex === null"
  @click="removeSelectedRule"
>
  {{ $t("editor-action-remove") }}
</button>
```

- **Placement rationale.** The ListWidget precedent places its Add at
  the end of the item list inside the widget's fieldset; the grid pair
  sits in the same position relative to its rows. Below-the-table also
  puts the pair adjacent to the detail panel that Add opens.
- **Order rationale.** ListWidget's DOM order (per-item Removes before
  the trailing Add) is an artifact of per-item placement, not an
  ordering precedent for a paired row. Add-before-Remove follows the
  boundary entry's own enumeration order, the catalog block order, and
  mkvtoolnix's menu order (`attachmentsMenu` adds
  `addAttachmentsAction` before `removeAttachmentsAction`,
  `merge/attachments.cpp` context-menu construction).
- **Accessible names.** Visible text via `$t` only - no `aria-label`.
  The in-file precedent governs (`editor-open`/`editor-save` buttons in
  the same view carry visible text and no `aria-label`); `ListWidget`'s
  redundant `:aria-label` duplicating its visible text is not a pattern
  worth propagating, and the eslint `no-raw-text` rule is satisfied
  since the only text nodes are `$t` calls.
- **testids.** `editor-rule-add` / `editor-rule-remove` - siblings of
  the house scheme (`editor-rule-row`, `editor-rule-select`,
  `editor-rule-detail`).
- **Keyboard reachability.** Native buttons are keyboard-reachable for
  free - the same rationale the Task 13b row-select button records
  (EditorView doc comment, RunHistory precedent). A disabled Remove is
  skipped by Tab, exactly like the disabled Save button in the same
  view; no `aria-disabled` sleight is added.

**Rejected: per-row Remove buttons (strict ListWidget parity).**
Steelman: ListWidget's remove needs no selection and no disabled state;
one click fewer; the target rule is visually unambiguous because the
button sits in it. Rejected: the grid's editing grammar is
selection-plus-detail (unlike ListWidget's always-open inline item
sections), and a selection-scoped Remove keeps ONE "current rule"
concept shared by grid highlight, detail panel and removal; per-row
buttons would add a sixth column of unconfirmed destructive targets
directly on draggable rows (D66 removed the dialog, so the design owes
accidental-click parsimony); and the table+detail idiom's platform
precedent removes by selection (mkvtoolnix attachments, source-cited in
D66).

**Rejected: buttons in the caption / above the table.** Steelman: a
toolbar above the content is a common desktop idiom and keeps the
actions visible without scrolling on long grids. Rejected: no house
precedent (no view has a toolbar row), the caption is a semantic table
label carrying a help-id and a marker - loading it with controls
muddies both - and profile-scale grids (a handful of rules) have no
scrolling problem to solve.

**Interface changes:** none.

---

## D71: Help-id strategy - no new help-ids; the `editor-tracks-rules` topic carries the content; help-mode conformance by construction

**Decision.** The buttons get **no** help-id and **no** new topic files.
The affordance is documented in the existing `editor-tracks-rules` topic
(`help/en/editor-tracks-rules.md` + `help/de/editor-tracks-rules.md`) -
the very topic whose fabricated Add/Remove sentence T10 finding 1
deleted; this design restores the sentence as truth.

**Why dedicated help-ids are ruled out (both routes walked, each
closed):**

1. `gui-helpid-equals-labelkey` (Tier 1): a registry control's help-id
   IS its labelKey, written out literally. The buttons' labelKeys are
   the generic action keys, shared by three sites - as help-id anchors
   they collide: one topic would annotate the ListWidget,
   PropertyMapWidget and grid instances alike, and the grid semantics
   (selection-scoped Remove, skeleton append) are FALSE at the per-item
   sites. That is precisely the shared-id overreach class T10 finding 3
   recorded (one id serving three surfaces, promising behavior only one
   has).
2. The non-labelKey route is mechanically open, not structurally
   barred - `gui-helpid-equals-labelkey` scopes the identity to a
   REGISTRY control's help-id, and a sanctioned non-labelKey help-id
   class already exists: `view-batch`/`view-jobs`/`view-editor` and
   `batch-suggestion-card` are help-ids and none is a catalog id
   (verified: the anchored id grep over `locales/en/*.ftl` returns
   nothing for them while hitting `editor-tracks-rules` as the
   positive control). A template `data-help-id` on the buttons would
   need no new labelKey (ruling 4 untouched) and no Tier-1 exception.
   What it needs is membership in the annotated set - and that set is
   owner-closed, constraint 3.
3. D54's owner-approved classification already EXCLUDES the
   "`editor-action-add`/`-remove` buttons" row by name (reason cell:
   "generic list actions / presentation column; no per-instance
   content") and closes the id and host sets - D54's own closure
   sentence: "the id set, file set and host elements above are closed
   here"; the plan-7 design's §9 implementer boundary restates it as
   "adding or dropping a member is an owner change, not an
   implementation nicety" (`2026-07-21-plan7-help-i18n-design.md` §9,
   the annotated-set bullet). This design conforms to the standing
   classification instead of reopening it.

**Topic content (both locales, same change - the bilingual duty).** The
sentences are drafted at implementation and finalized through the
owner's rendered-surface pass (the standing D54 route); the CLAIMS they
must carry are closed here so nothing is invented at the keyboard, each
anchor-bound per `content-claims-anchor-bound`:

- "Editing a rule" section gains the affordance mechanics: Add appends
  a new empty rule at the end, selects it and opens the detail panel;
  the empty rule is announced by a warning until its match expression
  is filled (anchor: D65/D67, the empirical emission). Remove deletes
  the SELECTED rule, is unavailable until a row is selected, and asks
  no confirmation - saving is what makes changes permanent (anchor:
  D66, D41 save-note surface).
- "When the list may be empty" gains one clause: removing the last rule
  is allowed and lands in exactly the legality described there (anchor:
  D69/core-83).
- h1s are NOT touched (content-only edit), so `help-topic-h1-scheme` is
  unaffected; the edited files must still pass the D62 content bans
  (no URLs, no pipes, no raw HTML - the new sentences are plain prose).

**Help-mode interaction - zero new code, conformance verified against
the shipped delegation (section 1):** the buttons are unannotated
activation controls in the content area. In help mode:

- Hover/focus resolves via `closest("[data-help-id]")` to the nearest
  annotated ancestor - the view root `view-editor` - so the sidebar
  shows the editor view topic; this is the shipped unannotated
  fallthrough the existing e2e asserts.
- Click is suppressed at capture phase (`onHelpClick`:
  `preventDefault` + `stopPropagation`, so `@click="addRule"` never
  fires) and pins `view-editor`. Conforms to
  `help-mode-suppression-pointer-scope`: click activation is the
  pointer channel the boundary closes.
- Enter/Space on a focused button is intercepted by `onHelpKeydown`
  (`helpTarget()` is non-null via the same ancestor walk),
  `preventDefault()`ed - so the native button activation click is never
  synthesized - and pins. This is the shipped D52 keydown shape, inside
  the ruled boundary (the channels the boundary keeps LIVE are typing
  and keyboard select changes, which the buttons do not have). The
  closure is redundant, not single-layer (recorded post-T2): were the
  synthesis not prevented, the synthesized click would itself be
  stopped by the capture-phase click listener - T2 rounds B/B2 measured
  exactly that - which is why case 9 carries an event-level witness for
  the keydown layer specifically (section 5, witness extension).

Both mutation paths into the model are therefore closed in help mode by
the existing capture-phase delegation, with no new listener and no
button-side condition.

**Rejected: dedicated help-ids + two new topic pairs.** Steelman, in
its strongest form - through the sanctioned non-labelKey class
(constraint 2): the view topics and `batch-suggestion-card` prove that
template `data-help-id`s without catalog ids are house-legal, so two
dedicated ids cost no labelKey and no scheme change - just two topic
pairs and a D54 set change - and buy precise hover targeting in help
mode (a topic instead of the `view-editor` fallthrough), with the D62
gate tracking the files by construction and room for the content to
deepen later. Rejected: the content is two sentences, below owner
decision B's inclusion criterion (D54's own test: genuine "when to use
/ interactions with other settings" content beyond tooltip depth); the
grid topic is the natural home the user already reaches (the grid
caption is the annotated element of record for rule mechanics); and
the ripple (a D54 owner change to both the id and host sets + 4 topic
files + D62 set growth) buys a hover target for content that would
duplicate the grid topic.

**Rejected: annotating the buttons with the existing
`editor-tracks-rules` id (a second host element).** Steelman: precise
hover targeting onto the right topic with zero new files. Rejected:
D54 closes the host-element set (one host per id; a change is an owner
change), and a duplicate `data-help-id` would break the highlight
mechanics silently - `setHelpClass` resolves the id by
`querySelector` (first match), so the pinned/hover highlight would land
on the caption while the user hovers the button. The fallthrough to
`view-editor` is the designed behavior for unannotated controls and the
buttons are classified unannotated.

**Interface changes:** none.

---

## D72: No tooltips on the two buttons - the obviousness premise verified, not assumed

**Decision.** The buttons carry **no** `title`/`.tooltip`. Spec 8.3's
baseline is "every **non-obvious** control carries a tooltip"; the
premise "these controls are obvious" is verified rather than asserted
(`proc-no-work-needed-check` - the runnable parts were run):

1. **The label is the action.** "Add"/"Remove" (de
   "Hinzufügen"/"Entfernen") on buttons inside the Rules fieldset
   directly beneath the rules table - the verb plus the position states
   the complete behavior. The one genuinely non-obvious part - Remove
   targets the SELECTED rule - is communicated structurally at the
   moment it matters: Remove is disabled until a row is selected
   (enabling visibly follows selection), and the detail panel tracks
   the same selection.
2. **House classification precedent, read at the tree.** The identical
   controls render tooltip-less at both existing sites, through the
   owner-approved exhaustive D54/D55 pass: D54's exclusion table names
   "`editor-action-add`/`-remove` buttons" with "no per-instance
   content", and neither key carries a `.tooltip` attribute in either
   locale (verified with positive control, section 0). The
   `editor-track-rule-order` key's catalog comment records the same
   judgment class for the grid's other non-registry element
   ("Presentation-only ... no .tooltip").
3. **The shared-key mechanics make a precise tooltip impossible without
   new keys.** A Fluent `.tooltip` is an attribute of the message - one
   text for all three consumer sites. Selection-scoped wording
   ("removes the selected rule") is false at the per-item sites;
   site-neutral wording ("removes an entry") restates the label and
   adds nothing (noise tooltips train users to stop reading - D54's own
   anti-inflation rationale). Site-specific keys are ruled out
   (ruling 4, D68).
4. **mkvtoolnix parity.** Their attachment add/remove actions carry no
   tooltips; the file's single tooltip sits on the list view and exists
   to advertise a hidden affordance ("Right-click for attachment
   actions") - a job that does not exist here because our buttons are
   visible (section 1, fire-verified counts).

**Rejected: `.tooltip` attributes on the generic keys.** Steelman:
spec 8.3's baseline is deliberately aggressive, "Removes the selected
rule" is real information beyond the label, and mkvtoolnix tooltips
aggressively elsewhere (~62 `setToolTip` calls under `merge/`, plan-7
parity table). Rejected: point 3 above is mechanical, not judgmental -
the precise text cannot exist on a shared key, and the imprecise text
carries no information; the structural signal (disabled-until-selected)
communicates the selection scope better than a hover-only hint; and the
owner-approved house pass already classified these exact controls.

**Interface changes:** none.

---

## 2. mkvtoolnix parity audit (SI-3)

Method per `proc-06-mkvtoolnix-parity`: source read at
`~/Downloads/mkvtoolnix` (v100.0 per `NEWS.md`, the tree the plan-7
audit matched against the running binary on 2026-07-21). No
muxing-semantics surface exists in this plan (pure editor UI), so no
binary run was owed; every citation is source. Licensing boundary:
behavior and facts only, no literal text or code taken, no wording
modeled. **The load-bearing frame: mkvtoolnix-gui is interactive-per-job,
Muxsmith is declarative-batch** - their merge tool operates on concrete
identified tracks of concretely added files; Muxsmith edits rules that
resolve per file at plan time.

| Plan-7.5 surface | mkvtoolnix-gui reality (cited) | classification |
|---|---|---|
| Add/remove of declarative track RULES | No rule abstraction exists: 0 of the `merge/*.h` headers contain "rule" (12 contain "track" - the control); the model is `Track` objects bound to added `SourceFile`s with per-track `m_muxThis` toggling (`merge/track.h`) | **No analogue - the interactive-vs-declarative divergence itself.** Their "add" adds concrete files to one job; our Add adds a rule applied across a batch. Divergence is the product premise, not a gap |
| Selection-scoped Remove, disabled without selection (D66/D70) | Attachment removal acts on the selection (`Tab::onRemoveAttachments` -> `removeSelectedAttachments(...selection())`) and the action disables without one (`enableAttachmentsActions`: `removeAttachmentsAction->setEnabled(hasSelection)`), `merge/attachments.cpp` | **MATCH** - the table+detail idiom's platform-standard removal grammar |
| Remove without confirmation (D66) | `onRemoveAttachments` mutates the model directly, no dialog; Del key bound to the same slot (`deletePressed` connect) | **MATCH** |
| Add appends + auto-select + detail panel (D67) | Their attachment detail form populates from the selection (`onAttachmentSelectionChanged` -> `setAttachmentControlValues`) and disables with none (`enableAttachmentControls(false)`) | **MATCH in idiom** (selection drives a detail form); their add opens a file dialog instead of creating an empty entry - nothing empty exists in an interactive model, so the skeleton path (D65) has no analogue |
| Reorder stays drag-only (ruling 3) | Attachments move via Ctrl+Up/Down and dedicated move buttons (`moveAttachmentsUp/Down` connects) | **Justified divergence**: spec 8.2 mandates "drag to reorder" and the shipped grid/ListWidget both implement HTML5 drag; button/keyboard reordering is a possible future comfort, not owed here (no anchor input asks for it) |
| No tooltips on the pair (D72) | Their attachment add/remove actions carry no tooltips; the file's one `setToolTip` (`attachments.cpp:490`) advertises the hidden context-menu affordance | **MATCH**, with the visibility nuance recorded: our buttons are visible, so even the advertisement tooltip's job does not exist |
| Zero-entry state legal (D69) | An empty attachment list is simply empty; their job validity is gated elsewhere (start-mux checks) | **Divergence in kind**: Muxsmith's zero-rule state has declared semantics (`core-83` passthrough / `NoTrackRules`) because a declarative profile must say what zero rules MEANS; an interactive job has no such question |

---

## 3. Change inventory and serialization/API surface

The complete set of artifacts implementation touches. Anything not
listed is not touched.

| artifact | change |
|---|---|
| `src/views/EditorView.vue` | `addRule`/`removeSelectedRule` (D67's closed shapes) + the two-button template block after `</table>` (D70's closed shape) |
| `help/en/editor-tracks-rules.md`, `help/de/editor-tracks-rules.md` | content-only edits per D71's enumerated claims; h1 untouched |
| `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` | amendments 1-2 below (controller lands them when consuming this design, per house practice) |
| `e2e/editor-rule-add-remove.spec.ts` (new), `e2e/help-mode.spec.ts` (additive) | section 5 |

**Explicitly zero:** no new/changed catalog ids or attributes (any
locale), no registry change (`registries.ts` untouched - the buttons are
bespoke-grid chrome like the row-select button, not a `FieldSpec`), no
new component, no `check-i18n` change, no Rust change, no `DiagCode`
change, no eslint config change.

**Serialization/API surface - confirmed pure frontend, no memo-worthy
finding.** Add/remove are in-memory model mutations
(`model.value = { ...}` swaps, identical in kind to the shipped
`onDrop`/`setRuleValue`/`ListWidget` mutations). The wire is exercised
only through the two **existing** IPC commands, unchanged in shape:
`validateProfileModel(profile)` (fired by the existing `watch(model)`
when a profile is open; the skeleton crosses the wire as
`{"match":{}}` and deserializes via the serde defaults, ground truth)
and `saveProfile(path, profile)` (D41 canonical save; the skeleton
serializes as `- match: {}` under D48 default omission). No new
command, no payload shape change, no `src-tauri` edit. The expectation
in the brief holds; nothing contrary was found.

---

## 4. Spec amendments proposed

Per `proc-04-spec-wins`; verbatim-ready. Self-contradiction sweep at the
end.

1. **Spec 8.2, view 1 ("Profile editor"), first sentence.** Replace

   > track-rule grid (order, source, match summary, changes, optional;
   > drag to reorder), detail editor per rule,

   with

   > track-rule grid (order, source, match summary, changes, optional;
   > drag to reorder; Add appends an empty rule - invalid until filled,
   > announced by validation - selects it and opens its detail editor;
   > Remove deletes the selected rule without confirmation, legal down
   > to zero rules per 4.5), detail editor per rule,

   **Superseded 2026-07-27:** the owner's wording ruling (commit
   `406e91b`) replaced "invalid until filled, announced by validation"
   with "incomplete until filled, announced by a validation warning"
   and expanded the zero-rule clause with its 4.5 consequences. The
   quoted block above is the wording this document mandated and Task 4
   was graded against; the shipped spec text is authoritative.

   The rest of the item (save semantics, inline markers) is unchanged.

2. **Spec 5.2, diagnostics table: add the missing
   `EmptyMatchExpression` row** (pre-existing staleness, surfaced by
   this design's dependence on the code; section 1's verified negative).
   Insert after the `EmptyMatchList` row:

   > | `EmptyMatchExpression` | warning | a rule's `match` expression
   > has no conditions at all (no exact/substring/regex/any/not): it
   > would match every track of its source (config-time; suppressed
   > when the emptiness is a present-but-empty top-level `any`/`not`
   > list, which already raises its own `EmptyMatchList` for the same
   > node) |

   The suppression clause transcribes `validate.rs`'s own comment and
   guard (the `empty_list_here` check), verified against the code.

   Scoping, so this row does not read as completing the table: the 5.2
   table is not exhaustive - **17 of `diag_codes!`'s 47 members have no
   row** (measured 2026-07-22 against `report/mod.rs`;
   `ParseError`, `EmptyExtensions`, `ProvableOverlap`, the template
   family and `NoTrackRules` among them). This amendment adds exactly
   one row because `EmptyMatchExpression` had ZERO spec presence
   anywhere while this design's guidance mechanism (D65) depends on it;
   `NoTrackRules`, which D69 depends on equally, is already spec'd
   with severity and condition in §4.5 prose ("Empty rules under
   `drop` remain a `NoTrackRules` error") and needs no table row to be
   citable. The wholesale table-staleness question is the round-1
   review's controller watch item
   (`docs/process-journal/artifacts/plan-7.5-sdd/design-review-round-1.md` HARVEST),
   outside this plan's scope.

**Self-contradiction sweep (spec-wide, for the two amendments):**
spec 8.2's view-1 sentence is the spec's only description of the
editor's rule affordances (grep "grid" / "detail editor": no other
site); §11 non-goals carry no add/remove claim; §8.3's baseline
sentence is satisfied per D72 without spec change; spec 4.5's
empty-rules paragraph already matches D69's semantics (it IS the
authority core-83 mirrors) and needs no edit; the 5.2 row's wording
introduces no conflict with 5.4's static-lint paragraph (which speaks
of provable overlaps, a different check). The help-topic sentence the
T10 fix removed returns via D71's content pass, not via spec text.

---

## 5. Test plan

House pattern: one new per-feature spec file (the plan-7 shape:
`editor-dropdowns` / `editor-markers` / `editor-tooltips`), plus one
additive case in the existing help-mode spec. Existing specs stay
untouched and must stay green - the buttons render outside the
`<table>`, so Task 11's `editor-rule-row` assertions and the Task
13b detail-panel spec see no DOM change inside their selectors.

**New file `e2e/editor-rule-add-remove.spec.ts`:**

Mount-harness cases (model-injected, no IPC, the Task 11/13b sibling
pattern):

1. Add appends: row count +1, new last row's ordinal renders, source
   cell shows `primary`, match and changes cells empty, optional
   unchecked; the held model's `tracks.rules` gained exactly
   `{ match: {} }` (anti-vacuity: assert the model value, not just the
   DOM).
2. Add auto-selects and opens the panel: the new row's select button
   carries `aria-current="true"`; `editor-rule-detail` is visible and
   shows the four `trackRule` fields.
3. Remove disabled without selection (`toBeDisabled`), enabled after
   selecting a row.
4. Remove deletes the selected rule: count -1, the removed rule's
   summary text gone, the OTHER rules' summaries still present (the
   right-rule assertion); selection cleared (`aria-current` on no row,
   panel hidden).
5. Remove down to zero: zero `editor-rule-row` rows, caption and
   headers still render, Add still present; Add from zero renders row 1
   selected with its panel open.

Mocked-IPC cases (real app + `installTauriMocks`, the
`editor-markers.spec.ts` pattern):

6. Open a profile, click Add: `validate_profile_model` is invoked with
   the appended skeleton (assert the payload's last `rules` member is
   `{"match":{}}` - the wire-truth assertion); the mock responds with
   `empty-match-expression` (warning) at `tracks[N].match`; the marker
   renders inside the open detail panel with
   `data-diag-path="tracks[N].match"`, the grid ROW marker for the bare
   `tracks[N]` is absent (the exact-anchoring negative, with case 6's
   own panel marker as the in-test positive control), the diagnostics
   panel lists the code, and **Save stays enabled** (D65's
   warning-severity consequence, pinned so a future core severity
   change fails loudly - trigger 4).
7. Remove to zero under `drop`: mock responds `no-track-rules` (error)
   at `tracks.rules`; caption marker renders error, Save disabled.
8. Remove to zero under `keep`: mock responds `passthrough-profile`
   (info) at `tracks.rules`; caption marker renders info, Save enabled.

Help-mode (one additive test in `e2e/help-mode.spec.ts`, appended to
the existing `test.describe("help mode (D52)")` block - the
activation-suppression family; the file's three describes are
"help mode (D52)", "help mode annotations (D54)" and "help mode drag
suppression (I1)", read at the current tree. The case follows the I1
sibling's in-test counterpart shape ("a drag-reorder mutates the rule
grid outside help mode but is suppressed inside it"): real app +
`installTauriMocks`, its own opened-profile fixture, mutation control
and suppression assertion in the SAME test and harness):

9. Outside help mode: click Add -> row count +1; focus Add, press
   Enter -> +1 again (both channels demonstrably mutate - the
   non-vacuity controls). Toggle help mode on: click Add -> count
   unchanged and the sidebar shows the `view-editor` topic (topic
   identity only, NOT pin evidence: in the editor view the unpinned
   fallback renders the same topic, so the row count carries the
   suppression evidence - T2 verdict M1/round D); focus Add, press
   Enter -> count unchanged. Add, not Remove, carries the assertions
   deliberately: Remove is disabled without a selection, so a
   suppression check against it could pass vacuously.

   **Witness extension (amendment 1, 2026-07-27, post-T2; controller
   ruling `redundant-layers-need-mechanism-witness`).** The Enter half
   above is over-determined: Enter on a focused `<button>` synthesizes
   a click, which the capture-phase click listener stops, so the
   unchanged count cannot attribute the closure to the keydown layer,
   and T2 round B2 proved that layer unguarded repo-wide (the entire
   Enter/Space branch removed leaves all 62 e2e tests green). The pin
   cannot discriminate either: both routes pin `view-editor` (T2 Q1).
   Case 9 therefore additionally carries an EVENT-LEVEL WITNESS,
   mirroring the I1 sibling's own mechanism witness (`attemptDrag`
   dispatches a synthetic `dragstart` with
   `{ bubbles: true, cancelable: true }` and reads
   `defaultPrevented` - the shipped in-file precedent this extension
   copies):

   - Helper, module level beside `attemptDrag`:

     ```ts
     function probeEnterKeydown(page: Page): Promise<boolean> {
       return page.evaluate(() => {
         const add = document.querySelector('[data-testid="editor-rule-add"]')!;
         const keydown = new KeyboardEvent("keydown", {
           key: "Enter",
           bubbles: true,
           cancelable: true,
         });
         add.dispatchEvent(keydown);
         return keydown.defaultPrevented;
       });
     }
     ```

     `cancelable: true` is load-bearing: without it `preventDefault()`
     is a no-op and the witness is structurally red on correct code.
     No rAF flush (unlike `attemptDrag`): the value is event state read
     synchronously after the synchronous dispatch, no rendering is
     involved.
   - Step, outside help mode (after the Enter mutation control):
     `expect(await probeEnterKeydown(page)).toBe(false)` - the paired
     absence control (no help-mode keydown listener registered), then
     re-assert the row count is unchanged by the probe: the dispatch is
     untrusted (`isTrusted: false`), so the browser runs no activation
     behavior on it, and this count assertion tests that premise
     instead of assuming it.
   - Step, inside help mode (after the landed Enter suppression
     assertions): `expect(await probeEnterKeydown(page)).toBe(true)` -
     THE witness. Discrimination is structural: `onHelpKeydown`
     (registered capture-phase on `document`, `App.vue` `watch(helpMode)`
     block) is the ONLY keydown listener in `src/` (grep-verified, both
     trees), its Escape branch calls no `preventDefault`, and the click
     layer never sees a keydown - so a `defaultPrevented` Enter keydown
     is attributable to exactly the Enter/Space branch. Recorded side
     effect: the probe pins `view-editor`, which the landed click half
     already pinned; no landed assertion reads pin state afterward.
   - **Acceptance criterion (the fix-round fire-test):** with the
     Enter/Space branch alone neutralized - both the T2 round-B shape
     (its `preventDefault`/`stopPropagation` removed) and the round-B2
     shape (the whole branch removed) - the inside-phase witness
     assertion FAILS while every previously-landed assertion stays
     green; restored, the full file passes. The outside-phase `false`
     control and the inside-phase `true` assertion form the same
     discriminating pair as the I1 sibling's
     outside-`dragstartPrevented: false` / inside-`true`.
   - The landed contract assertions stay exactly as landed; the
     extension is additive within the same test.

**Gate ripple, enumerated (the D62/D55 duty):**

- New help-ids: **none** -> D62's topic-tree gate (both directions, per
  locale) sees an unchanged id set; no new topic files exist to check.
- Edited topic files (2): must still pass the D62 content bans -
  external-URL ban, pipe/table ban, raw-HTML ban (`check-i18n.mjs` help
  checks); the new sentences are plain prose, and the reviewer's
  standing fire-verification discipline applies to the absence scans.
- Catalog ids/attributes: **zero added, renamed or attributed** -> the
  check-i18n id-parity and attribute/placeable/selector checks see
  byte-identical catalogs; id counts stay 46/46; no de catalog pass
  rides this plan.
- `catalog_completeness.rs` (Rust): no `DiagCode` change, unaffected.
- eslint `no-raw-text`: the buttons' only text nodes are `$t()` calls.
- `help-topic-h1-scheme`: no h1 touched.
- Bilingual duty (i18n-16 shape): the en and de topic edits land in the
  same change.

---

## 6. Triggers created (for the controller to mirror into the ROADMAP)

1. **Consuming this design** -> the controller extends
   `editor-generic-action-keys`' statement to record the rule grid as
   the third render site of the generic pair (occurrence with this
   design's approval as ref), mirroring the plan-7 trigger-10 precedent
   of updating the entry in place. No budget change accompanies it
   (D68: zero new ids).
2. **Amendment 2 lands (or is declined)** -> if declined, the
   `EmptyMatchExpression` spec-5.2 gap is registered as a one-liner
   instead; it must not stay unrecorded (this design's guidance
   mechanism cites the code).
3. **A request for site-specific wording or tooltips on the generic
   action keys** (any site) -> reopens the shared-key question as an
   owner decision; the latent-coupling steelman recorded in
   `editor-generic-action-keys` is the argument that fires. Default
   remains shared keys (D68/D72).
4. **Core changes `EmptyMatchExpression`'s severity or the skeleton's
   emission set** -> D65's recorded semantics and test case 6's
   Save-enabled assertion re-verify; the save-gate consequence flips if
   the severity ever becomes error, and that is an owner-visible
   product change, not a silent ride-along.
5. **An accidental-rule-deletion report arrives** -> route to the v1.x
   editor-undo/redo entry (ruling 2's durable answer), not to a
   confirmation dialog (D66 records the rejection).
6. **The owner wants the grid buttons help-annotated after all** ->
   that is a D54 id/host-set owner change reopening D71's resolution,
   not an implementation nicety (the plan-7 design §9 annotated-set
   boundary; D54's own closure sentence is "the id set, file set and
   host elements above are closed here").

---

## 7. Deliberately out of scope

- **Undo/redo, in any form** - ruled v1.x wholesale (S22); D66 notes
  the interaction and builds nothing.
- **Insert-at-position, up/down reorder buttons, multi-select removal**
  - ruling 3 fixes append + drag-reorder; multi-select has no house
  selection model to build on and no anchor input.
- **Attachment-rule add/remove** - already shipped via `ListWidget`;
  untouched.
- **The spec-8.2 grid-column-order cosmetic** (spec lists "order,
  source, match summary, changes, optional"; the shipped column order
  differs) - pre-existing, cosmetic, not opened by this plan.
- **A tooltip-disable setting, help search, F1** - plan-7's recorded
  non-goals, unchanged.
- **Any core/validate change** - the skeleton's semantics are consumed
  as-is (D65); severity changes are trigger 4's owner question.

---

## 8. What the implementer must not decide

Every fork below is closed above; a fork discovered on code contact
returns as NEEDS_CONTEXT with a decision memo, it is not resolved at
the keyboard (`proc-latitude-clause-boundary`).

- The skeleton is the object literal `{ match: {} }`, uncast, nothing
  prefilled (D65).
- `addRule`/`removeSelectedRule` are D67's shapes verbatim: immutable
  rebuilds, `selectedIndex = next.length - 1` on add, `= null` on
  remove; no other state is introduced.
- No programmatic focus call, no `autofocus`, anywhere (D67).
- The buttons are exactly D70's template block: native
  `<button type="button">`, testids `editor-rule-add`/
  `editor-rule-remove`, Add before Remove, inside the fieldset after
  `</table>`; visible `$t` text, no `aria-label`, no `title`; Remove's
  only disable condition is `selectedIndex === null`.
- Zero catalog changes: no id, no attribute, no rename, either locale
  (D68/D72).
- Zero help-id changes: no `data-help-id`, no registry `helpId`, no new
  topic files; only the two `editor-tracks-rules` topic bodies change,
  h1s untouched, carrying exactly D71's enumerated claims (draft
  wording at implementation, final strings via the owner's
  rendered-surface pass).
- No help-mode code: the existing delegation covers the buttons by
  construction (D71); adding a button-side help-mode condition is a
  defect, not diligence.
- No editor-side guard against zero rules, and no core change of any
  kind (D69/D65).
- The e2e set is section 5's nine cases in the two named files;
  extending existing editor specs beyond the one additive help-mode
  case is out of scope (`proc-proposed-safeguard-stays` runs the other
  way too: the nine cases are not argued down at implementation).

---

## 9. Open items

**None.** Every fork the brief names is resolved above (UI placement
and disabled state D70, skeleton value and verified emission D65,
help-id strategy D71, tooltips D72, panel/focus behavior D67, help-mode
interaction D71, diagnostics landing D65/D69, e2e and gate ripple
section 5, spec amendment section 4, serialization surface section 3);
the five owner rulings are recorded as D65-D69 and folded throughout.
No NEEDS_CONTEXT item leaves this design.

---

**Amendment 1 (mid-run, 2026-07-27, post-T2): case 9 gains an
event-level witness for the keydown-suppression layer.**

- **Defect.** Case 9's Enter half was over-determined and its assumed
  separate guard false: Enter on a focused `<button>` synthesizes a
  click that the capture-phase click listener stops, so the
  unchanged-count assertion cannot attribute the closure to
  `onHelpKeydown`; with the entire Enter/Space branch removed, all 9
  help-mode cases and all 62 e2e tests stay green, and the pin state
  cannot discriminate the routes (both pin `view-editor`).
- **Evidence.** `docs/process-journal/artifacts/plan-7.5-sdd/task-2-verdict.md` - Q1,
  findings M1/M2, fire-verification rounds A-D (round B2 decisive).
- **Ruling.** Controller, internal technical fork, recorded as ledger
  entry `redundant-layers-need-mechanism-witness`: case 9 gains an
  event-level witness so a single layer's death becomes observable.
- **Change.** Section 5 case 9: witness extension appended (the
  `probeEnterKeydown` helper mirroring the I1 `attemptDrag` precedent -
  synthetic cancelable Enter keydown dispatched at the Add button,
  `defaultPrevented` read synchronously; outside-phase `false` control
  with a probe-side-effect count guard; inside-phase `true` witness;
  acceptance criterion: the witness fails under the round-B and
  round-B2 neutralizations while every landed assertion stays green).
  Landed contract assertions unchanged.
- **Dependent-sentence sweep.** Changed beyond the case-9 block: case
  9's click-half phrase ("`view-editor` pinned" -> "sidebar shows the
  `view-editor` topic", scoped as topic identity per M1/round D);
  D71's keydown bullet (single-layer attribution widened to the
  measured redundant closure, pointing at the witness). Walked and
  verified-unaffected: D71's closing outcome claim ("Both mutation
  paths into the model are therefore closed", an outcome the T2 rounds
  re-verified); section
  8's "nine cases in the two named files" (the witness extends case 9,
  adding no tenth case and no third file); section 8's zero-production-
  code enumeration (the witness is test-side only); the section-5 gate
  ripple (unchanged - no new ids, files, or catalogs); trigger 4
  (unrelated core-severity trigger). Feasibility anchors: `attemptDrag`
  (`e2e/help-mode.spec.ts`, module-level helper) is the shipped
  in-repo precedent for synthetic-dispatch + `defaultPrevented`
  witnessing; `onHelpKeydown` is the sole keydown listener in `src/`
  (grep over both the main tree and worktree `plan75-a`; registered
  capture-phase on `document`, `App.vue` `watch(helpMode)` block, its
  Escape branch calling no `preventDefault`).
