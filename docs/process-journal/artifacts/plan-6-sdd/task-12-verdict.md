# Verdict (extracted from the reviewer transcript)

I have everything needed. All checks are complete. Commit is unsigned (`[N]`), explicitly staged, exactly the 6 brief-mandated files.

---

### Spec Compliance
- ✅ **45/45 keys, both locales** (counted directly: `grep -cE` = 45 each). Budget = 42 labels + 1 save-note + 2 action keys.
- ✅ **Two new action keys, exact strings**: EN `editor-action-add = Add` / `editor-action-remove = Remove`; DE `= Hinzufügen` / `= Entfernen` (owner-mandated infinitive/nominalized forms, straight ASCII).
- ✅ **Re-pointing complete**: both widgets' `$t()` calls now use `editor-action-add/-remove`; the only surviving `editor-attachment-rule-add/-drop` references in the widgets are in doc comments. Old keys remain defined and still used as `attachmentRuleFields` labels (`registries.ts:186,189`), so `check:i18n` still counts them.
- ✅ **Typed value cells on BOTH maps** (settable `changes` + matchable `exact`): one `cellKindFor` switch over `PropScalarType` → checkbox / `type="number"` / `type="number" step="any"` / text, with the house `const _exhaustive: never` arm; unknown-property and `values:"string"` (substring/regex) both fall back to text before the switch.
- ✅ **No dropdowns** anywhere (`type`/`codec_kind` resolve to the string→text arm).
- ✅ **Anti-vacuity assertions real**: settable `forced_track` `.toBe(true)` (not `"true"`), matchable `forced_track` likewise, matchable `min_luminance` `1→1.5` `.toBe(1.5)` + `typeof === "number"`; bonus integer `audio_channels` arm exercised.
- ✅ **T10 mount specs repointed** (mandated, not weakened); **T11 grid test byte-untouched**; RED evidence (5 failures) names the correct pre-implementation states.
- ✅ **Hygiene**: unsigned commit (`%G? = N`), explicit staging, exactly the 6 brief files.

### Adjudications

**Q1 — the `tracks` special case.**
(a) **Justified, properly-recorded composition exception — not a mandate violation, no NEEDS_CONTEXT needed.** `ListWidget.vue`'s doc comment scopes the generic widget as explicitly *not* the `tracks.rules` grid; Task 11's grid test (`data-testid="editor-rule-row"`, `toContainText("video"/"audio")` on summary cells) is protected — it is not in Task 12's Files list, and the amendment's review-check forbids weakening mount specs. Routing `tracks.rules` through the generic `ListWidget` would replace the plain-text summary cells (`textContent`) with live sub-widgets (whose `<input>` values are never in `textContent`), breaking that protected assertion. The exception is documented in both `EditorView.vue:25-41` and `ListWidget.vue:4-17`. The mechanical reason is sound.
(b) **Everything else is genuinely registry-driven.** `EditorView` iterates `Object.entries(profileFields)`, filtering only `tracks` and `FixedField`, dispatching each through `FieldWidgetDispatcher`. Spot-checks: (1) a field added to `outputFields` surfaces via the `output` section dispatch → `SectionWidget`; (2) a field added to `attachmentRuleFields` surfaces via `attachments` → `attachments.rules` (generic `ListWidget`) → `SectionWidget`. Both surface with no view edit. The single dead spot is `trackRuleFields` (feeds the bespoke grid) — which is exactly the Q2 gap.

**Q2 — the claimed spec-coverage gap. Verdict: plan-coverage gap. The implementer is correct; route a plan amendment, not a T12 fix.**

(a) **Spec 8.2** (`/home/senol/Git/Muxsmith/.worktrees/plan6-e/docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:374`, post-Task-7) promises two *distinct* elements: "track-rule grid (order, source, match summary, changes, optional; drag to reorder), **detail editor per rule**, panels for attachments/chapters/tags/title...". Task 7's amendment left "detail editor per rule" in place.

(b) **Design D45** requires editable track-rule fields through the registry path. `2026-07-15-plan-6-design.md`: (b) `:794` "Exposing all 42 is the only option that cannot strand a field"; (d) table `:925` assigns `TracksCfg.rules` the widget `list { item: "trackRule", reorderable: true }` — byte-identical to `attachments.rules` `:923`. The `list` widget's `item` parameter exists precisely so `ListWidget` synthesizes a `SectionWidget` against the item's registry and recurses (`ListWidget.vue:27-30`); `trackRuleFields` carries all four fields (`source`/`match`/`optional`/`changes`) as `EditableField` (`registries.ts:139`). So the design's approved structure delivers per-rule rule-field editing via the generic dispatch of `tracks.rules`. Line `:230` explicitly names spec 8.2's editor "a drag-reorderable rule grid with per-rule detail editing." No design text defers or removes it; the ROADMAP's Plan-7 editor items are tooltips (`gui-26`) and the closed-domain dropdowns only.

(c) **As-built, the editor cannot edit a track rule at all.** `EditorView.vue` special-cases `tracks`: `tracks.unmatched` dispatches generically, but `tracks.rules` renders *only* Task 11's bespoke grid — `sourceSummary`/`matchSummary`/`changesSummary` as read-only text and a **`disabled`** optional checkbox (`EditorView.vue:88-121, 196-201`). There is no path to change a rule's `match`/`changes`/`source`/`optional`. By contrast `attachments.rules` renders through the generic `ListWidget` → each `AttachmentRule` fully editable. The editor edits attachment rules but not track rules — an asymmetry with no design basis.

(d) **Classification: plan-coverage gap.** No Plan-6 task delivers spec-8.2 per-rule track editing: Task 11's brief is "the grid and its reordering only"; Task 12 preserves that grid (correctly, since replacing it would weaken the protected T11 test the amendment forbids touching); Task 13 is save/nav/ipc; Task 14 is batch-view apply. The design (b)/(d) require editable track fields and spec 8.2 names the detail editor, yet the decomposition assigns it to no task. The plan's coverage review verified *registry-level* completeness (every field has a widget spec) but not *dispatch-level* completeness — Task 11's bespoke summary grid silently occupied the `tracks.rules` slot the design gave an editable list, and nothing restored editing.
- *Not a T12 scope shortfall*: T12's concrete brief (Files list, the four Step-2 test assertions, the protected grid) does not require track editing, and its two mandates — "registry-driven, no hand-listed fields" vs. "do not weaken the T11 grid" — are mutually exclusive for `tracks`. That contradiction is the plan's, not the implementer's; T12 executed its scoped brief.
- *Not deliberate restructuring* (no re-scoping design text) and *not "no gap"* (editing genuinely absent).

**The implementer's report frames Q1 and Q2 as two concerns; they share one root** — the read-only grid holds the slot the design assigned an editable list, and no task bridges back to editing. The controller should route a plan amendment (a new task / owner call) to build the per-rule detail editor — mkvtoolnix-gui-style detail panel beneath the grid, or expandable grid rows dispatching `trackRuleFields`. Scope is narrow: only the `TrackRule` sub-tree; every other field is editable.

**Q3 — mechanical.**
(a) ✅ 45/45 both locales; new key strings exact; re-point complete (widget `$t` all `editor-action-*`; old keys retained for section labels).
(b) ✅ Switch total over the four tags + `never`-arm + text fallback (both for `values:"string"` and unknown property); no `<select>`/dropdown/option in either widget.
(c) ✅ Real `true` (`=== true`), real `1.5` (`toBe(1.5)` + `typeof "number"`); RED evidence names the right failures (test aborts at the first `checkbox not found`, which is honest).
(d) ✅ No mount spec deleted/skipped/weakened; no `.skip`/`.only`; T10 repoint is the mandated caption change, T11 grid untouched.

### Strengths
- Correctly **escalated** the Q2 gap instead of silently absorbing it into a task that never asked for it — the right call under the open-dimensions/escalate principle.
- The `tracks` exception is documented in two places with the concrete mechanical reason (`textContent` vs. `<input>` value) for why generic dispatch would break the protected test.
- Anti-vacuity done properly (real boolean + real float round-trips, plus the optional integer arm), and the house `const _exhaustive: never` pattern reused from `FieldWidgetDispatcher.vue`.

### Issues
#### Critical (Must Fix)
- None *within Task 12*. The Q2 finding is a plan-coverage gap for the controller to route as an amendment, not a T12 defect.

#### Important (Should Fix)
- **Plan amendment required (Q2):** the per-rule track-rule detail editor that spec 8.2 names and D45 (b)/(d) require is built by no Plan-6 task. Until it is added, the profile editor cannot edit any track rule's `match`/`changes`/`source`/`optional` through the GUI. This is the review's deliverable, owned by the controller/owner, not a fix for this implementer.

#### Minor (Nice to Have)
- The report presents Q1 and Q2 as separate; noting to the controller that they are one root gap (the bespoke grid displacing the editable list) prevents a fix that addresses only one half.

### HARVEST
- **Registry coverage was checked at the wrong altitude.** The D45 forcing function guarantees every field has a widget *spec*; it does not guarantee every spec is *dispatched*. A bespoke component can occupy a registry-assigned slot and silently drop the capability the spec implied, and the compile-time completeness check cannot see it. Pattern worth recording: when a task is authorized to hand-build a component in place of a registry-dispatched widget, the plan must explicitly account for the capability delta (here: editability), or a later "compose from the registry" task inherits a contradiction it cannot resolve without violating a protected test.
- No rejection or house-pattern deviation observed in the T12 code itself; tier-2 boundaries (`gui-typed-scalar-needs-typed-input`, `gui-closed-domain-dropdowns`) are honored, and the structural-conformance grant produced no improper stop (the implementer proceeded and flagged, rather than halting).

### Assessment
**Task quality:** Approved
**Reasoning:** Task 12 meets every requirement of its brief with correct TDD, a green gate, and clean commit hygiene, and its one divergence (the `tracks` exception) is a justified, doubly-documented composition exception forced by a protected test. The single substantive finding — track rules are not editable — is a plan-coverage gap the implementer correctly identified and escalated rather than under-built, and it routes a plan amendment at the controller level, not a fix in this task.
