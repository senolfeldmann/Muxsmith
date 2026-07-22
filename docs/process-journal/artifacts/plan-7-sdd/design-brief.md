# Plan 7 design brief: help mode + i18n cluster

Controller brief for the four-eyes design phase (S20, 2026-07-21). You are
the design AUTHOR: a fresh implementer who authors the Plan 7 design
document. An independent reviewer will grade it (latitude in both forms,
coverage, house conformance) before the governing human reviews it.

**Verify every premise of this brief against the tree before building on
it; refuting a brief premise with evidence is a valid and wanted outcome**
(house precedent: the Plan 6 design author killed two phantom tasks that
way).

## Repo and ground truth

- Repo: `/home/senol/Git/Muxsmith` (Rust core + CLI + Tauri 2/Vue 3 GUI,
  Fluent i18n, en+de). Work on current master; do not commit - report back,
  the controller commits.
- Spec (authoritative over plans):
  `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, esp. §8.2,
  §8.3 (help-mode mechanics - binding), §8.4, §10.
- Tier-2 house files (conform; surface any deviation explicitly):
  `docs/product-boundaries.yaml` (note `gui-closed-domain-dropdowns`,
  line ~419), `docs/conventions.yaml`, `docs/process-conventions.yaml`.
- ROADMAP Plan 7 anchor: `docs/ROADMAP.md` (11 named inputs incl. the two
  folded 2026-07-21). Ledger context: `docs/decision-ledger.yaml`
  (i18n-05, i18n-10, i18n-12 among others).
- House design-doc pattern: read `2026-07-15-plan-6-design.md` in the same
  specs folder for format and ADR-block style. ADR numbering continues
  from the highest existing D-number (D49 last known - verify).

## Deliverable

A design document `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`
covering the full Plan 7 scope, with ADR blocks (decision / rationale /
rejected alternatives each with its steelman / triggers created /
interface or wire-format changes), plus the exact list of spec amendments
it requires. It must be complete enough that a plan can be written from it
without inventing anything: **every set it mandates is enumerated, no
ellipses, no "for example" lists unless explicitly marked exemplary, no
implementer-choice clauses.** The reviewer will hunt latitude in both
forms (explicit permission AND omission).

## Scope: the 11 anchor inputs + 1 rider

1. Help-mode sidebar, spec 8.3 full mechanics (help-ids, per-topic
   markdown per locale, hover-to-explain, click-to-pin, Esc, prominent
   toggle button in every view).
2. Editor tooltips: all 42 editable controls get tooltip keys (spec 8.3
   baseline; the 43rd registry field is fixed profile_version).
3. Spec-10 help-id completeness CI guard (fail on help-ids without a
   topic file, per locale).
4. Live in-session locale switch (today bootstrap-once; ledger gui-26).
5. Fluent message-attribute reorganization (widget facets as
   `.attribute` instead of suffixed siblings; `$ta` helper; check-i18n
   parser + parity extension).
6. resolvedTrackLabel punctuation outside Fluent
   (`src/components/ResolutionTable.vue:23-25`).
7. Curated-domain dropdowns in the editor's exact-match cells: `type`
   (4 values) and `codec_kind` (17 aliases) as selects, per decree
   gui-closed-domain-dropdowns.
8. Field-anchored inline validation markers: map `Diagnostic.config_path`
   to its control via the registries, marker at the control alongside the
   panel.
9. Rule-grid ordinal column (spec 8.2 "order" column; today row position
   only).
10. IpcError-code presence gate (today: DiagCode gated exhaustively,
    IpcError codes plain strings, no gate) + number promotion for
    IpcError params (today `Record<string,string>` everywhere, so Fluent
    plurals can never fire; "the profile has 1 rules").
11. check-i18n placeable-set and selector-structure parity per message id
    across locales (ledger i18n-12; would have auto-guarded the D39
    selector change).
- Rider: one-line spec sweep - spec 8.4 + non-goal 11 still claim
  "v1 ships English content only" (stale since de shipped); the Renderer
  rustdoc carries the same claim; spec 10 credits "eslint
  (no-literal-string rule)" where the real rule is
  `@intlify/vue-i18n/no-raw-text` (D27) and its scope claim needs
  precision (template text nodes + configured static attributes only).

## Owner decisions, binding (Şenol, 2026-07-21)

- **Curated help annotation (option B).** Tooltips are universal for the
  editor's 42 controls. Help topics (long-form markdown) exist for every
  VIEW plus every control that has genuine "when to use / interactions
  with other settings" content beyond its tooltip. Explicitly NOT
  inflationary: self-explanatory controls get no help-id. The design must
  ENUMERATE the annotated set completely, control by control, with a
  one-line justification per included control (the test criterion above);
  an unenumerated set is a defect. In help mode, hovering an unannotated
  element does nothing; the sidebar keeps its current topic (view topic
  by default).
- Both gate items (inputs 10, 11) are in scope by owner call 2026-07-21
  (fired-trigger fold).
- The spec-staleness rider is in scope.
- All new/changed Fluent messages AND all help topics land bilingual
  (en+de), same convention as everything since Plan 5.5.

## Design questions the document must decide

Each an explicit ADR-grade decision with rationale and steelmanned
rejected alternatives. This list is exhaustive for what the controller
knows today; add decisions the design work uncovers, and escalate any
product-scope/user-visible fork as an open question to the governing
human instead of deciding it.

1. Markdown renderer for the sidebar: earned-dependency analysis
   (established library vs minimal hand-rolled subset), weighing the
   house dependency rules (conventions.yaml), bundle size, and the CSP
   posture (D34). Help content is first-party, not user input - say what
   that does and does not license.
2. FieldSpec extension shape for tooltip key and help-id (optional
   fields vs new variants; what the 13 registries carry per control).
3. help-id scheme (naming, stability) and topic file layout
   `help/<locale>/<help-id>.md`; load mechanism in the Tauri context
   (build-time glob/import vs runtime fetch) with the same drift/parity
   guarantees the Fluent catalogs enjoy.
4. Help-mode interaction: highlight, pin, Esc, keyboard/a11y semantics;
   how annotation attaches to DOM elements (directive? component prop?).
5. config_path -> control mapping: parser locus (TS vs reusing the
   Rust-side parsing that ApplyError already does), registry-driven
   resolution, behavior for paths with no control (panel-only fallback).
6. Live locale switch: fluent-vue bundle swap mechanics
   (`src/main.ts:24-30` today builds once pre-mount), settings write,
   help-topic re-render, and what happens to open views.
7. Attribute reorg: exact target shape (which suffixed siblings become
   attributes of which messages - enumerate the migration set; recon
   says ~19 `-tooltip` keys exist, zero Fluent attributes anywhere yet),
   the `$ta` helper design, and the check-i18n extension for attributes
   (parser at `scripts/check-i18n.mjs:118-127` is line-based, extension
   path pre-documented in the script).
8. Curated-domain export: extend the ts_export mechanism
   (`crates/muxsmith-core/tests/ts_export.rs`, drift gate
   `ci.yml:121-138`) with TYPE_VALUES/CODEC_KIND_NAMES; dropdown
   rendering in the exact-match value cells only (the decree's boundary);
   interaction with raw:-prefixed properties (which bypass domains).
9. Field-anchored marker UX: what renders at the control (icon, border,
   severity mapping), interplay with the diagnostics panel as the
   recorded Plan-6 shape of spec 8.2's markers.
10. Ordinal column: presentation only (1-based index) - confirm no data
    change.
11. resolvedTrackLabel: move the ` (` `)` punctuation into Fluent; the
    bare "-" placeholder is a recorded deliberate un-Fluent decision
    (comment at ResolutionTable.vue:12-16) - respect or supersede
    explicitly, don't drift past it.
12. IpcError presence gate design (extend check-i18n vs a Rust-side
    exhaustiveness test vs both - note IpcError codes are stringly,
    unlike DiagCode's enum) and params number promotion (typed
    `Record<string, string|number>` end to end; enumerate the promotion
    sites - recon: FirstRun.vue:94, RunHistory.vue:155,
    JobsView.vue:246/252, DiagnosticsPanel.vue:34).
13. Help-id completeness CI guard: where it runs (check-i18n vs own
    script), what it checks (annotated set <-> topic files, per locale,
    both directions - orphan topics too?).
14. The annotated-control enumeration itself (per owner decision B).

## Standing duties

- **SI-3 parity**: compare against mkvtoolnix-gui (source at
  `~/Downloads/mkvtoolnix`, cite file:line; run mkvmerge v100.0 for
  behavior, never memory) for its tooltip/help approach; classify
  match / justified divergence / genuine gap. The help-mode sidebar is
  presumably a deliberate divergence (mkvtoolnix links out to online
  docs) - verify what mkvtoolnix-gui actually does before recording
  that. Licensing boundary: behavior/facts fair game, never literal text;
  deliberately modeled wording is an explicit ADR decision.
- A safeguard this design proposes is not argued back out during design
  (removal only after built + measured redundant).
- Where a passage concludes a guard/enumeration/check is unnecessary,
  the claim that makes it unnecessary must be verified and cited.
- Recon facts in this brief come from a read-only inventory pass
  (2026-07-21) - treat every file:line as a claim to re-verify, not a
  fact.

## Recon summary (verify before relying)

- Catalogs: `locales/{en,de}/` x 7 files; en counts: cli 26,
  diagnostics 50, gui-batch 39, gui-common 41, gui-editor 45,
  gui-jobs 46, gui-settings 13. Zero Fluent attributes anywhere.
- Frontend loader `src/i18n/index.ts` (globs gui-* + diagnostics,
  excludes cli.ftl); bootstrap-once in `src/main.ts:24-30`; locale from
  settings (`src/ipc.ts:33,46`) via `SettingsDialog.vue` select.
- check-i18n.mjs: hard-fail literal-id check (incl. `labelKey:` per
  D45), warn-only unused-id check (IpcError codes land here, documented
  residual at `:42-50`), hard-fail cross-locale id parity (`:247-297`).
  No attribute handling.
- Editor: 13 registries in `src/editor/registries.ts`,
  `FieldSpec = { labelKey, widget } | { fixed, why }` in
  `src/editor/fieldSpec.ts`; 42 editable + 1 fixed field; 10 widget
  variants. Zero tooltips in the editor; ~19 native-`title` tooltips
  elsewhere with `-tooltip` suffix keys.
- No markdown renderer dep in package.json; no help/ dir; no
  config_path->field mapping anywhere in src/ (only Rust-side
  ApplyError parsing in `src-tauri/src/error.rs:164-166`).
- Curated domains in `crates/muxsmith-core/src/capability/mod.rs`
  (TYPE_VALUES :64, CODEC_KINDS :114-132, 17 aliases); NOT exported to
  frontend today (settables.ts types them plain "string").
- Rule grid inline in `src/views/EditorView.vue` (4 columns, :469-479);
  order = array position, drag/drop rebuild (:358-372).
- 19 real IpcError codes (error.rs + lib.rs/run.rs); all 19 currently
  HAVE catalog messages (14 gui-common, 5 gui-jobs) - the presence gate
  codifies a holding invariant.

## Report back

Your final report: what the design decides per question above, which
brief premises you refuted (with evidence), open escalations for the
governing human (product-scope forks only), and the list of spec
amendments. The design document itself is the deliverable; the report is
the summary.
