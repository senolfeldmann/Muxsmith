# Roadmap

Living, forward-looking work tracker: commitments with their milestone, and
candidates for later. Items here are DISCUSSION ANCHORS, not execution
licenses - each is talked through with Şenol when its turn comes, unless it
already carries a settled decision reference. History lives in the process
journal; decisions live in the specs/memos; unbuilt product ideas with full
analysis live in IDEAS.md.

## Plan-6 scope re-cut (2026-07-15)

Şenol's call at the Plan-6 brainstorming start: the single Plan-6 anchor
had accumulated **20 named inputs spanning four independent subsystems**,
so it is decomposed into Plans 6-9, each with its own spec and
brainstorming. All 20 are distributed below and every one is its own bullet
under its plan, so the split (6 / 5 / 1 / 8) is recountable rather than
asserted. Nothing was dropped in the re-cut. (Split now 5 / 5 / 1 / 9:
input #6 re-pointed from Plan 6 to Plan 9 by owner call 2026-07-16.)

D22's editor+apply pairing is KEPT, but **not on D22's stated reason**: that
reason was "one-click apply IS comment-preserving YAML mutation, which the
editor owns anyway", and the 2026-07-15 save-fidelity ruling (canonical
save, comments not preserved) killed the premise. The pairing survives on a
different argument - both mutate the same in-memory model and must share its
ownership - recorded in the Plan-6 design ADRs. D22's
help-mode-in-the-same-plan half is superseded by this cut. The ledger
entries that pointed at "Plan 6" were re-pointed in the same turn (exec-36,
exec-37, cli-08, core-121 -> Plan 9; gui-26 -> Plan 7).

## Plan 6: profile editor + apply-suggestion - EXECUTED AND CLOSED 2026-07-17

**Executed 2026-07-16/17** (16 tasks incl. 12a/13b/13c added by five four-eyes
mid-run amendments; 9-item whole-branch fix wave; whole-branch verdict READY
after the post-verdict delta re-review; merged 962005b, gate green, pushed).
Salvage in docs/process-journal/artifacts/plan-6-sdd/. Owner surface pass
(45 editor keys de, 7 reused keys, 2 apply keys, grid notation, nav-tab
question) runs at the close. The anchor below is history.

Scope: profile editor, one-click apply-suggestion, and the schema
keyword-domain fix (owner call 2026-07-15, moved in with the
schema-as-user-artifact decision). Save is canonical; comments are not
preserved (owner ruling 2026-07-15, rationale in the design ADRs: YAML
1.2.2 §6.6 defines no comment-to-node association, and drag-to-reorder
would silently make surviving comments describe the wrong rule).

Named design inputs (5; former #6 re-pointed to Plan 9 by owner call
2026-07-16):

1. Decide schema-driven vs UI-model editor. (2026-07-11, sweep walkthrough
   #23) **RESOLVED 2026-07-15:** UI-model, hand-built components, with
   ts-rs-generated types and a `Record<keyof T, FieldSpec>` registry as the
   forcing function.
2. Decide whether the JSON schema ships as a user artifact (e.g.
   yaml-language-server autocomplete). (2026-07-11, sweep walkthrough #23)
   **RESOLVED 2026-07-15 (owner): yes**, it becomes a supported user
   feature.
3. If either of the above lands schema-side, add `schemars(schema_with)`
   overrides for the keep/drop/clear keyword domains -
   FilenameCfg/ChaptersCfg/TitleCfg schematize as `anyOf [object, string]`,
   the keywords live only in validate.rs (Plan-1 final review minor #7).
   (2026-07-11, sweep walkthrough #23) **RESOLVED 2026-07-15:** in scope -
   but NOT via the recorded trigger ("a GUI generating an editor from the
   schema"), which input 1's resolution means would never have fired. The
   real reason is input 2: `muxsmith schema` ships the artifact today.
4. The profile editor itself. (D22)
5. One-click apply-suggestion. (D22)

## Plan 7: help mode + i18n cluster

Grouped because all five inputs land in the localization layer (Fluent
catalogs, check-i18n, per-locale content). Sequenced after Plan 6 so the
editor's controls get their help-ids in this pass instead of a retrofit.

- Help-mode sidebar, spec 8.3 full mechanics: help-ids, per-topic markdown
  per locale, hover-to-explain, click-to-pin. (D22)
- **The profile editor's own tooltips, spec 8.3 baseline** (owner ruling
  2026-07-16): the editor ships in Plan 6 WITHOUT tooltips, and its 42
  controls get their tooltip keys here, in the same pass as their help-ids,
  rather than as a retrofit - which is the re-cut's own stated reason for
  sequencing Plan 7 after Plan 6. So `gui-editor.ftl` carries 43 keys in
  Plan 6 (42 labels + 1 save-surface note) and grows by the tooltip set
  here. Raised by the plan-6 plan review (F8): spec 8.3 requires "every
  non-obvious control carries a tooltip" and D22 calls the tooltip baseline
  "NOT deferred", which collided with the 43-key budget; the editor is
  between Plan 6 and Plan 7 without tooltips, which is consequenceless on an
  untagged pre-1.0 tool with no users. (2026-07-16, plan-6 plan review)
- The spec-10 help-id completeness guard (CI fail on help-ids without a
  topic file - S17). (2026-07-11, docs-tree sweep)
- Live in-session locale switch (T21.5-m1, bootstrap-once today; ledger
  gui-26). (2026-07-12, Plan 5.5 roll-up funnel)
- The deferred Fluent message-attribute reorganization (widget facets as
  .attribute instead of suffixed siblings; touches frontend $ta +
  check-i18n parity). (2026-07-12, idiomacy review triage)
- resolvedTrackLabel punctuation outside Fluent (locale-formatting
  revisit, S16). (2026-07-11, docs-tree sweep)
- Curated-domain dropdowns in the editor's exact-match cells: `type`
  (4 values) and `codec_kind` (17 aliases) render as selects instead of
  typed text fields, per the standing decree gui-closed-domain-dropdowns
  (product-boundaries.yaml). Scalar typing of those cells ships in Plan 6;
  this is the comfort layer on top. (Owner ruling 2026-07-16, plan-6
  matchable-cell routing.)
- Field-anchored inline validation markers in the editor: map each
  diagnostic's `config_path` to its field via the registries and render
  the marker at the control, alongside the panel. Owner ruling 2026-07-16
  (whole-branch finding 2): the diagnostics panel IS the recorded Plan-6
  shape of spec 8.2's "inline validation markers"; the field anchoring is
  this comfort-layer item, natural sibling of the help-ids/tooltips pass.
- Rule-grid ordinal column: spec 8.2 lists "order" among the grid columns;
  Plan 6 carries order by row position only. Cosmetic comfort item.
  (Plan-6 whole-branch verdict, spec-completeness walk.)
- IpcError-code presence gate + number promotion - both separable pieces
  of the "Nothing gates IpcError codes against gui-common.ftl" entry under
  v1.x candidates (full context there). Folded in by owner scope call
  2026-07-21 (S20 session-start trigger check): its trigger "next CI/gate
  structural work" fired with this plan's help-id guard and check-i18n
  work. (2026-07-16, D49 review harvest item 5)
- check-i18n placeable-set and selector-structure parity per message id
  across locales - the sharper variant of the "Gate part that
  Fluent-parses ALL catalogs" entry under v1.x candidates (full context
  there; ledger i18n-12). Same fired trigger and owner scope call as the
  IpcError item above. (T4 verdict harvest H1 + plan-5.8 design-review
  harvest, 2026-07-14)

## Plan 7.5: track-rule add/remove in the editor (pre-1.0, owner ruling S21)

Owner ruling 2026-07-22 (S21, at the plan-7 close): the editor's
track-rule gap lands PRE-1.0 as its own small package - moved here from
the v1.x discussion anchor. The gap: the editor edits and drag-reorders
existing `tracks.rules` entries (bespoke grid, detail panel) and can add/
remove ATTACHMENT rules (generic ListWidget), but offers no way to create
or delete a TRACK rule; spec 8.2 lists no such affordance either.
Surfaced by the plan-7 T10 help-content review (task-10-verdict.md
finding 1 side-product). The asymmetry reads as an oversight, not a
decision - the owner's rationale for pre-1.0. Scope sketch (kickoff
decides the design): add/remove affordance on the bespoke grid following
the ListWidget add/remove precedent (generic action keys), detail-panel
integration for a fresh rule, spec-8.2 amendment, e2e coverage incl. the
D62/D55 gates' ripple (help topic for the new buttons). Ordering vs
Plan 8 is the next kickoff's call.

**KICKOFF 2026-07-22 (S22, parallel with Plan 8 by owner call)** - owner
rulings, binding for the design: fresh rule = empty skeleton,
invalid-until-filled, the existing diagnostics/marker plumbing guides
(no prefill guesses); remove without confirmation (explicit save bounds
the loss; the durable answer is the v1.x undo/redo entry below); new
rule appends at the end, auto-selected, detail panel opens; buttons
render the generic editor-action-add/-remove keys (zero new label
keys); no last-rule protection (core-83 passthrough / NoTrackRules
semantics carry it). Design D65-D72 four-eyes complete (one fix round,
delta APPROVED) and **owner-APPROVED 2026-07-23**; four-eyes plan
authoring in progress.

## Plan 8: packaging / release pipeline

Carries its own constraint set (code signing, notarization) that no other
plan shares; orthogonal to all GUI work.

- msi/dmg/deb/rpm/AppImage on release tags (spec 10; deliberately deferred
  out of Plan 5's CI work). (D22) State established at the re-cut: the
  ci.yml `v*` tag trigger exists but drives only the test matrix; no
  workflow calls `tauri build`; tauri.conf.json declares
  `bundle.targets: "all"` with all five icons present but no per-OS blocks,
  no publisher/category, no signing or updater config; the version is
  declared independently in Cargo.toml, tauri.conf.json and package.json
  with no sync mechanism.

**KICKOFF 2026-07-22 (S22, parallel with Plan 7.5 by owner call)** - owner
rulings, binding for the design: UNSIGNED artifacts on all three OS at
1.0 (per-OS install-hurdle documentation ships with the release docs;
signing revisit is a registered trigger below); NO auto-updater at 1.0
(v1.x); a `v*` tag builds bundles and attaches them to a DRAFT GitHub
release - the owner publishes manually; artifact matrix: Windows x64 +
arm64 (msi), macOS arm64 ONLY (dmg; an Intel request is a registered
trigger below), Linux x64 as deb + rpm + AppImage + a portable tar.gz
("just runs" archive); pipeline verification runs via workflow_dispatch
(artifacts + draft-release rehearsal), never a test tag; deb/rpm declare
mkvtoolnix as Recommends at 1.0 (hard Depends = v1.x entry below);
Plan 8 builds the pipeline and does NOT tag 1.0. Design D75-D90
four-eyes complete (one fix round, delta APPROVED) and **owner-APPROVED
2026-07-23**; four-eyes plan authoring in progress.

**CLI-distribution ruling (owner, S22 second round, after a source-level
mkvtoolnix parity check)**: msi and dmg bundle the CLI binary alongside
the GUI - mkvtoolnix parity (its Windows installer ships every tool via
`File "../*.exe"`, its dmg carries all binaries in Contents/MacOS) -
with NO add-to-PATH installer option (mkvtoolnix offers none either;
the manual PATH step is documented in the install docs). The Linux
CLI/GUI package split of mkvtoolnix is deliberately NOT adopted: one
package per format, both binaries (recorded divergence); the tar.gz
carries both binaries as already ruled. Homebrew-Cask distribution is
a v1.x entry below.

## Plan 9: core/orchestration hoists + planner seam

Orthogonal to every GUI plan; can run as a parallel worktree stream rather
than in sequence.

- Hoist the four-copy planning pipeline into a shared plan_pipeline() core
  fn - this IS the injectable-planner-seam (S4/S5/S6), spec 5.5/7
  parity-critical (~100 lines); plan_pipeline consumes
  profile::validate::config_diagnostics (landed Plan 5.6 T11). The seam
  INTERFACE is this plan's design question (ledger
  core-121-planner-seam-and-hoist). (2026-07-12, idiomacy review triage)
- The GUI test-harness question as ONE block - no Vitest component
  harness, no tauri::test integration harness, start_run's orchestration
  body untested ("to raise at merge time"; the merge gate passed without
  it) (S4/S5/S6). (2026-07-11, docs-tree sweep)
- Hoist run_batch into muxsmith_core::executor (the CLI inlines what
  src-tauri already factored). (2026-07-12, idiomacy review triage)
- Hoist runs-root resolution to core (D26 debug-only seam duplicated CLI
  vs src-tauri). (2026-07-12, idiomacy review triage)
- Core logging facade replacing the single eprintln in queue.rs (T4-i1;
  ledger exec-36). (2026-07-12, Plan 5.5 roll-up funnel)
- Route JobOutcome.errors codes through the diagnostics catalog so
  worker-panicked renders on live surfaces (T4-i2; ledger exec-37).
  (2026-07-12, Plan 5.5 roll-up funnel)
- Reject bare `raw:` with empty property name at validate (T16-m1).
  (2026-07-12, Plan 5.5 roll-up funnel)
- Sort JSON config_diagnostics errors-first for validate parity (T9-m-iv;
  ledger cli-08). (2026-07-12, Plan 5.5 roll-up funnel)
- Re-check the final fix wave's self-flagged deviation from D23's frontend
  contract (reset gated on runActive instead of "reset after resolve Ok" -
  "worth a second look", never taken) (S12). (2026-07-11, docs-tree sweep;
  re-pointed here from the Plan-6 anchor by owner call 2026-07-16 - the
  plan-6 design review established it is run-path only, JobsView.vue:150-200,
  touching nothing Plan 6 builds.)
- One error-display funnel for IpcError rendering in the frontend: the
  plan-7 design review counted 8 scattered render sites each hand-rendering
  `$t(err.code, err.params)` (enumeration in its verdict file, salvaged
  with the plan-7 archive); a shared component/composable is the hoist.
  Discussion anchor, not a commitment. (2026-07-21, plan-7 design review
  round 1 harvest.)

## Triggers

Observable events with registered consequences - CONSULT AT EVERY
MILESTONE GATE (visibility flip, release, tag, publication; doctrine §3).
Details live at the pointed-to entries; this list only names event ->
action:

- Renovate/Dependabot activated -> prune deny.toml RUSTSEC ignores; TS-7
  bump arrives via its PRs (near-1.0 entry riders).
- A bulk profile shows regex compilation dominating -> promote the
  regex-cache line from v1.x into hardening scope (v1.x entry).
- mkvtoolnix/mkvmerge version bump (dev machine or CI) -> re-verify the
  reference memory + pinned identification_format_version (spec §9.2;
  the reference memory went stale at v99 once).
- Crates ever published to crates.io -> docs.rs implications of the
  rustdoc gate (Plan 5.5 T12).
- Fake-mkvmerge test helpers grow beyond three copies -> shared
  test-support crate (v1.x entry).
- Plan 7, 8 or 9 starts -> consume the named design inputs in that plan's
  anchor. (Plan 6's instance of this trigger fired 2026-07-15 and was
  consumed at its brainstorming: the anchor was re-cut into Plans 6-9 and
  the inputs distributed.)
- First real-world report of unwanted empty outputs, or a request to
  fail batches on empty plans -> IDEAS #5.
- Next parity-audit round or output-plausibility work -> IDEAS #6.
- A second fixture needs a _comment-style source-of-truth note ->
  promote the T11 ad-hoc pattern to a written convention (BUILDING.md
  test section).
- Any change moving a live-test gate from Mkvmerge::locate() to detect()
  -> fix the bare-PATH fixture spawn in identify_live.rs make_sample
  (routed-items item 4; becomes a real panic-instead-of-skip bug at that
  moment and must land in the same diff).
- The v1.x mise-out-of-CI structural work starts -> run the one-off
  single-parallel-run verification (falsify the ci.yml interleaving
  rationale empirically) and drop one of the two test runs per leg
  (routed-items item 11, re-deferred past plan 5.7's 2-line ci.yml touch).
- A profile-model field gains a `#[serde(default)]` -> it joins D48's
  17-row table with ALL THREE attributes naming the same function (serde
  default, skip_serializing_if, schemars extend) - the one place where
  getting it wrong loses user data silently. (Plan-6 close, plan trigger 1.)
- tauri-specta publishes a stable non-RC Tauri-2 release -> re-evaluate
  D44's rejection. (Plan-6 close, plan trigger 3.)
- 1.0 is tagged, or a user asks for zero-config schema autocompletion ->
  re-evaluate SchemaStore publication. (Plan-6 close, plan trigger 5.)
- A third locale directory is added -> D62's lockstep gate and D55's
  parity rules fire by construction; one manual check: if the locale's
  CLDR plural categories exceed {one, other}, verify D55 rule 5's
  category carve-out passes its catalogs before blaming them. CLI-side,
  the renderer's embed table gains the locale's row (D63) and D64's
  pinned suite stays green by construction. (Plan-7 design triggers
  2 + 11.)
- A labelKey is renamed -> D51's coupling renames the help-id and both
  topic files; D62 is the tracker (red until they agree). (Plan-7 design
  trigger 3.)
- The plan-7 SDD salvage runs (plan close) -> dispatch a re-pointing of
  the plan-7 design doc's three `.superpowers/sdd` citations (round-8
  review adjudication: correct at commit time, must move WITH the
  salvage in the same change; the amendment blocks quote their evidence
  inline, so the citations are corroboration, not sole carrier).
  (Round-8 fix delta, 2026-07-22. CONSUMED same day: salvage 4cc128a,
  re-pointing 9d01862, all three sites, same push.)
- The plan-7.5 SDD salvage runs (plan close) -> re-point the plan-7.5
  design's citation of `.superpowers/sdd/plan-7.5/design-review-round-1.md`
  (amendment-2 scoping paragraph) to the salvaged artifact path, in the
  same change as the salvage - per the ruled round-8 house pattern.
  (Plan-7.5 delta review observation, 2026-07-22 S22.)
- fluent-vue or @fluent/bundle is bumped -> re-verify the `bundles`
  setter contract and the `$ta` global (D56/D55 rest on installed
  3.8.2's verified behavior). (Plan-7 design trigger 4.)
- marked major release arrives -> re-check D50's "0 dependencies, no
  sanitizer needed for first-party input" premises before merging.
  (Plan-7 design trigger 5.)
- A second `v-html` site is proposed anywhere -> reopens D50's
  single-site license as a design decision, never a per-case judgment
  call. (Plan-7 design trigger 6.)
- The attachment-context propertyMap type lookup is fixed properly
  (pre-existing flaw: track type tables used for attachment
  select/drop maps; Plan-9 capability/registry neighborhood) -> revisit
  D58's path gate in the same change. (Plan-7 design trigger 7.)
- A user asks for quieter UI / tooltip suppression -> the mkvtoolnix
  `uiDisableToolTips` parity gap becomes a v1.x candidate with precedent
  cited (plan-7 design section 3). (Plan-7 design trigger 8.)
- First external-user complaint about unsigned-install hurdles (or a
  comparable adoption signal) -> re-evaluate code signing/notarization
  per OS. (Plan-8 kickoff ruling F1, 2026-07-22 S22.)
- A macOS Intel (x64) user asks for a build -> add the x64 dmg leg to
  the release matrix. (Plan-8 kickoff ruling F4, 2026-07-22 S22.)
- Next core/planner-touching plan -> run the D49 G1/G2 removal experiment
  (mutate delta_for's AddExact arm to re-stringify, run the suite: G1+G2+G3
  all fail -> they stay for good; only G3 fails -> G1/G2 are removal
  candidates as localizers). Runnable since D49 landed (T6); guards stay
  until the run per proc-proposed-safeguard-stays. Disposition of the other
  plan-6 triggers at the close: trigger 2 SETTLED by measurement during T4
  (guard 2 fires in its literal-expected form, stays for good; recorded in
  the falsifiability entry); triggers 4 and 7 FIRED and were consumed
  mid-plan (12a extended the ts export set; settables.ts = second
  committed-generated-plus-drift-check instance, own ledger entry at count
  2); trigger 6 needs no entry by design (drift check and registry fail by
  construction).

## Pre-1.0 release gates

Must be resolved before the first tagged release; none blocks Plan 6 work.
**Plan 5.5 EXECUTED AND CLOSED 2026-07-12** (30 tasks incl. seven added
in-flight by review routings and Şenol gate decisions; commits
e8e85d9..close; per-task verdicts + whole-branch review in the salvaged
plan-5.5 archive). Still OPEN before the tag: README placeholder items
(at-tag), guide/blogs (at 1.0), the whole-codebase idiomacy review (with
its named-input list below), and the mixed-language `allowed`-param
polish entry.

- **README**: v1 draft shipped 2026-07-11 (sell-tone per Şenol's register
  override - a case-scoped exception to the writeup-stimme rule; WIP
  banner; full CLI usage reference; AI-collab story section; links
  BUILDING.md instead of absorbing it; all four 2026-07-11 content
  anchors implemented, provenance in this entry's git history). Remaining
  at the 1.0 tag: resolve the four `placeholder(1.0)` comments (GIF,
  dry-run output snippet, release artifacts, GUI screenshot), drop the
  WIP banner, re-check the CLI reference and the exact-typed-matching
  paragraph against the shipped surface (reviewer warning: "easy to
  lose").
  - Content anchors (append every "README-worthy" remark HERE the moment
    it is uttered; fold into the README on next touch):
    - Şenol 2026-07-12: properties with language-like matching MAGIC must
      be EXPLICITLY LISTED in the README - language (ISO/BCP-47
      normalization + dual-field language/language_ietf lookup), absent
      boolean flags comparing false for exact, type/codec_kind curated
      domains; contrast with raw:'s no-magic byte-exact single-field rule
      (D32 addendum, B-8 ratification).
- **Guide + blog posts (process + product)**: 1.0 deliverables, produced
  at 1.0 on Şenol's go. Format interview DONE 2026-07-11; decisions:
  - **Guide**: single `docs/GUIDE.md`, English, maximal scope - cookbook/
    workflows AND architecture/contributor part AND exhaustive reference;
    Şenol prunes at review ("I want them all, I decide what I keep").
    Deepens the README CLI reference without duplicating it; splits into
    its own "man page" file only if it outgrows one file.
  - **Blog posts**: two, separate and cross-linked (process + product),
    written at 1.0 as markdown into the blog project folder, published
    when the blog launches (platform decision open there). English first,
    German versions additionally.
  - **Authoring pipeline** (per recovered R3): three FRESH sessions at
    1.0, one per deliverable, each fed the process journal + repo + git
    history + artifact archive - not raw transcripts, not the sessions
    that did the work (Betriebsblindheit; the guide especially wants a
    fresh reader who stumbles where contributors would).
  - **R3 recovered 2026-07-11** (transcript mined, quotes verified;
    discussion was 15:49-15:55 CEST 2026-07-08, not "late morning"):
    docs/process-journal/artifacts/r3-journal-blog-rationale.md carries
    the verbatim extract + condensation. Core: deliverables decay at
    different rates (process story fast and irreproducibly -> journal
    immediately; product docs pre-1.0 describe a torso -> write at 1.0);
    disjoint audiences and half-lives -> two posts.
  - The process-learnings distillate kept with the project's non-repo material
    remains a second primary source.
- **Run-log auto-prune implementation (D35)**: DONE 2026-07-12 (Plan 5.5
  Task 4.5, merged d18f1b7): name-parse-based 14-day prune in
  RunLogger::create, parser moved to core, shell delegates.
- **mkvtoolnix version pin in CI + mac/win runners**: DONE 2026-07-11
  (Plan 5.5 Task 2, commits 374005a+19deec3+24ac702, verified in run
  29165610230): per-OS install steps, pins apt 97.0-1build1 /
  choco 100.0.0 / brew floats (no install-time pin possible), decision
  comment in ci.yml. The fired go-public trigger is CONSUMED: gated
  tests run LIVE on all three legs, skip-marker assertion counts 0
  everywhere ("3-OS green" now means live-binary tests on 3 of 3).
  First live Windows run immediately surfaced and fixed a real
  Windows-only defect (read-only handle + set_modified, 24ac702).
- **`.gitattributes` (`* text=auto eol=lf`)**: DONE 2026-07-12 (Plan 5.5 T1, 209218c; repo was already LF-clean, wav asset marked -text).
- **Catalog param-drift guard**: DONE 2026-07-12 (Plan 5.5 T10 + merge-time fixtures for every later DiagCode; exhaustive-match guard live; known single-site blind spot documented at fixture_args, instance fixed via T9-ix).
- **German locale (de) before 1.0**: DONE 2026-07-12 (Plan 5.5 T19/T20/T21/T21.5: plural selectors, parity gate, six catalogs Şenol-reviewed incl. corrections Starten/Meldungen/Verweis, UI-selectable with endonym labels).
- **Test-hardening rider (sweep group T)**: DONE 2026-07-12 (Plan 5.5 T11; attachment ids empirically verified 1-based).
- **rustdoc gate step + dead intra-doc link**: DONE 2026-07-12 (Plan 5.5 T12; gate is nine parts, four dead links fixed).
- **Packaging pipeline**: msi/dmg/deb/rpm/AppImage on release tags (spec 10;
  deliberately deferred out of Plan 5's CI work). Lands via Plan 8
  (re-pointed from Plan 6 by the 2026-07-15 scope re-cut).
- **Spec-§10 test mandates: proptest + insta**: DONE 2026-07-12 (Plan 5.5 T14 proptest =1.11.0, 18 properties + T22 insta =1.48.0, 11 redacted snapshots, CI strict).
- **`--list-types` extension validation (spec §4.2)**: DONE 2026-07-12 (Plan 5.5 T5 input + T5.9 locators; UnknownExtension warning, model docs true).
- **UnknownPropertySkew forward-compat path (spec §9.2)**: DONE 2026-07-12 (Plan 5.5 T15/T16 per D32 raw: opt-in + T16.5 once-per-batch SchemaDrift notice; B-8 single-field ratified; spec §9.2 amended).
- **Suggestion-engine D6 completion (spec §5.3 + D6 remainders)**: DONE 2026-07-12 (Plan 5.5 T13 mechanical parts + T17/T18 per D33 overlap narrowings; suggestion-keyed partition ratified via §5.3).
- **Diagnostics polish block (review minors, Plans 1-4)**: DONE 2026-07-12 (Plan 5.5 T9 nine items + T9.5 donor naming; vii fixed as not-deliberate, viii kept with spec §8.4 entry).
- **Zero-track plan warning**: DONE 2026-07-12 (Plan 5.5 T6; EmptyPlan decided post-finalize, batch-report test; deliberate divergence recorded in the plan-5.5 memo).
- **SourceOverwrite completeness (Plan-2 FINAL minor M2)**: DONE 2026-07-12 (Plan 5.5 T7 + T7.5 + T7.6; class closed by construction over all three donor kinds, completeness comment at the gathering site).
- **Empty-batch human output (D15 gap)**: DONE 2026-07-12 (Plan 5.5 T8; always-print summary, recorded divergence).
- **Robust event-stream reads (Plan-4 T1 minor, severity upgraded)**: DONE 2026-07-12 (Plan 5.5 T3; read_until + lossy, no-hang live regression).
- **Zero-rule keep = legal passthrough (Şenol ruling, escalation 2026-07-13)**:
  a profile with zero track rules and `tracks.unmatched: keep` is a LEGAL
  pure-passthrough remux, not a `NoTrackRules` error (use it to change only
  title / attachments / chapters, or normalize the container, without any
  track rule). `unmatched: drop` + zero rules stays `NoTrackRules`.
  Implement: lift the `validate.rs` `NoTrackRules` error for the keep case
  (the executor already has the passthrough path, job.rs / D20 "passthrough
  counts as matched"). MUST be documented and hinted - the passthrough via
  `unmatched: keep` is non-obvious (README + a validate-time hint, and the
  GUIDE at 1.0). Recorded in product-boundaries.yaml (core-83). Timing:
  Şenol's scope call (small).
  **DONE 2026-07-15** (Plan 5.8 T1+T2 per ADR D38: conditional validate,
  `PassthroughProfile` info diagnostic, bilingual catalogs, no-track-rules
  hint, spec 4.5/5.2 amendments, D20 supersession note, README recipe,
  gated e2e; archive docs/process-journal/artifacts/plan-5.8-sdd/).
  GUIDE mention stays an at-1.0 item with the GUIDE itself.
- **Mixed-language `allowed` param (pre-1.0 polish, whole-branch I2)**:
  core emits English prose via the `allowed` param at planner.rs:428/:841
  ("a valid ISO 639/BCP-47 language code") - in de mode
  invalid-property-value renders mixed German/English; not on the spec
  §8.4 exception list. Fix catalog-side (kind selector or dedicated
  language-domain message), bilingual. Small task before the tag
  (bilingual launch makes it user-visible).
  **DONE 2026-07-15** (Plan 5.8 T3 per ADR D39: Fluent select on the
  existing $property param, prose `allowed` param off the wire for
  language emissions, coupled-comment sweep, diagnosticFluentParams.ts
  strictness comment fixed = routed-items item-7 trigger consumed;
  archive docs/process-journal/artifacts/plan-5.8-sdd/). Rider ADR D40
  (same plan, whole-branch finding): plan-JSON serialization panic fixed
  (struct variants TitleAction/ChapterSource/PrimaryAttachments,
  report/json.rs hardening, README-recipe regression e2e).
- **Whole-codebase idiomacy review**: EXECUTED 2026-07-12 (session 10; the
  journal carries the run + the usage-limit incident; triage routing in
  the STATUS entry right after this one). Original dispatch spec: one
  large, deliberately costly review pass before the 1.0 tag - Rust
  workspace, TS/Vue frontend, configs - against ecosystem idiom. SIX
  dimensions (four original + two adopted
  2026-07-12 from mining the ponytail rule set, Şenol-approved; the
  full mining analysis incl. evidence assessment is kept with the
  project's non-repo material):
  unidiomatic constructs; near-duplicate reimplementations; hand-rolled
  vs stdlib/established library; inverse dependency sweep; `yagni`
  (over-abstraction: interface/trait with one impl, factory with one
  product, wrapper that only delegates, config nobody sets, layer with
  one caller, dead flags); `native` (platform reinvention: code or a dep
  doing what the language runtime, browser, CSS or DB does natively).
  OUTPUT CONTRACT (adopted from ponytail's review/audit skills): one line
  per finding `<file>:L<n>: <tag> <what to cut>. <replacement>.`, ranked
  biggest-cut-first, ending `net: -<N> lines, -<M> deps possible`; a
  clean subsystem returns "Lean already. Ship." Correctness/security/
  perf explicitly OUT of scope for this pass (complexity hunt and bug
  hunt want different mindsets); route such finds to a normal review. NAMED INPUTS from the Plan 5.5 roll-up funnel
  (2026-07-12, whole-branch triage; details in the salvaged
  whole-branch-verdict): skip-marker shared const (T2-m1);
  known_extensions required-method idiom (T5-m2); prop_assume->prop_assert in D6 property
  (T14-m1); test-side logic mirrors (T14-m3); partition best=None
  invariant comment (T13-m1); lock_active doc precision (T4-m1);
  attachments/chapters-only EmptyPlan test (T6-m1); redundant fn-level
  cfg(unix) (T3-m1); overlap_conflicts re-parses claimants from the
  rendered param string (whole-branch M2); UnknownExtension "once per
  batch" rustdoc vs once-per-entry (M3); de catalog headers overclaim
  what check-i18n enforces (M1); §5.2 WorkerPanicked severity cell says
  info where "n/a (job-error token, not a rendered diagnostic)" is
  tighter (final-verification nit). Dimensions: unidiomatic constructs;
  near-duplicate reimplementations (reuse violations); hand-rolled code
  where the stdlib or an established library is the human-normal solution;
  the inverse dependency sweep (is every current dependency earned and
  healthy). Multi-agent review per the process package, findings triaged
  with Şenol. Decided 2026-07-11 (Şenol: important, so weird
  LLM-shaped code does not slip through the cracks before release; the
  conventions idiomacy directive only governs code written AFTER it
  existed - this pass covers everything written before). Timing anchor:
  after the feature plans (5.5, 6) land, immediately before the
  release-facing gates.
- **Whole-codebase idiomacy review - STATUS 2026-07-12 (EXECUTED, triaged
  with Şenol); FIX WAVE EXECUTED AND CLOSED 2026-07-13** (Plan 5.6,
  commits 0b3149a..a5d506b: 12 tasks + final fix wave, 64 findings + 13
  seeds applied, whole-branch verdict READY, zero behavior change except
  three sanctioned interface deltas recorded in ADR D36 + the plan-5.6
  journal entry; the plan-folded and deferred items below stay open):
  74 raw -> 73 deduped findings; 70 confirmed, 1 refuted, 2
  already-tracked, 11 routed out (correctness/security/perf), 13/13 funnel
  seeds confirmed still-open; net -483 lines, -0 deps (dep sweep clean -
  every direct Cargo/npm dep earned and healthy). Ranked findings report
  kept with the project's non-repo material. Triage routing:
  - PRE-1.0 idiomacy fix wave (own SDD plan, mechanical + low-risk): the
    ~58 mechanical findings (byte-identical dups, dead config/flags, local
    idiom/stdlib cleanups) + self-contained refactors (config-diagnostics
    helper across 6 sites; the spawn_blocking IPC wrapper, 5 copies; run-id
    parse via the time crate) + the Fluent comment-level fix (# -> ###) +
    sharing the same-crate test helper + dropping restated action defaults.
    The two deprecation-claim config nits (tseslint config(), deny.toml
    version key) ship after a quick doc re-check (drop-in either way).
    Funnel seeds folded in by nature; two also appear under tracked -
    dedupe when planning.
  - FOLDED INTO PLAN 9 (re-pointed from Plan 6 by the 2026-07-15 scope
    re-cut; they share the run/plan-orchestration territory of the planner
    seam): the four-copy planning pipeline (~100 lines, spec 5.5/7
    parity-critical - a shared plan_pipeline() IS the never-decided
    injectable-planner-seam S4/S5/S6); run_batch hoist to
    muxsmith_core::executor; runs-root resolution hoist to core.
  - DEFERRED (tracked Plan-7; re-pointed 2026-07-15): the Fluent message-attribute
    reorganization (widget facets as .attribute vs suffixed siblings) -
    changes message IDs, needs coordinated frontend $ta + check-i18n parity
    work; distinct from the comment-level fix that ships now.
  - KEPT as deliberate scaffold: the ci.yml `v*` tag trigger (Plan 8
    packaging will consume it; re-pointed 2026-07-15).
  - Refutation accepted: BatchView's withDefaults is not deprecated in Vue
    3.5, and reactive-props-destructure would make it the lone outlier vs
    five sibling components (house convention).
- **Correctness/security/perf review of the idiomacy pass's 11 routed-out
  items**: a SEPARATE normal review before the 1.0 tag (bug-hunt mindset,
  deliberately not blended into the idiomacy complexity hunt). Release-
  relevant heads: mise-action fetches a floating mise binary at CI run time
  (SHA pin covers the action JS, not the binary - contradicts
  pin-everything; superseded by the post-1.0 mise-out-of-CI item in v1.x
  candidates); ci.yml has no workflow `permissions:` block (GITHUB_TOKEN
  gets the repo default); settings.rs save() claims crash/power-loss safety
  but never fsyncs before the atomic rename. Full list (incl. non-UTF-8
  argv lossiness, swallowed ctrlc registration, editorial nits) in the
  non-repo findings report. Decided 2026-07-12.
  **DONE 2026-07-14**: bug-hunt adjudication against current master (1
  already fixed by Plan 5.6, 4 should-fix, 6 can-wait, 0 refuted, 0
  release blockers; ci.yml token premise downgraded by live API check -
  repo default is read) + **Plan 5.7 executed and closed** (owner triage
  2026-07-14: four fixes + ctrlc warning bundled because its own deferral
  trigger fired inside the plan; dry-run indent ruled YES). Landed: ci.yml
  least-privilege permissions, settings fsync-before-rename, dry-run
  indent via placeables + run-signal-handler-unavailable (bilingual),
  DiagCode::NonUtf8Path per ADR D37. All four task reviews APPROVED,
  whole-branch READY, nine-part gate green after every merge. Archive:
  docs/process-journal/artifacts/plan-5.7-sdd/ (incl. the routed-items
  adjudication verdict). Four re-deferrals below (v1.x + Triggers).
- **Worker-panic handling + mutex-poison hygiene (Plan-4 T3 minor)**: DONE 2026-07-12 (Plan 5.5 T4 + whole-branch killer-invoke fix; poison recovery centralized incl. AppState.active).

## Ledger hygiene

- **ledger-lint duplicate-key gap (S21, 2026-07-22)**: a duplicated YAML
  key inside an entry (observed: a doubled `steelman:` line) passes the
  lint silently - the parser's later-key-wins swallows it. Extend
  ledger-lint with a per-entry duplicate-key check; trigger: the next
  ledger-lint or house-file-schema touch dispatches it alongside.
  **TRIGGER FIRED AND CONSUMED 2026-07-22 (S22)**: Plan 8's ledger-lint
  CI wiring (next bullet) is the ledger-lint touch; the extension rides
  the same Plan-8 rider task.
- **RESOLVED 2026-07-16 (session 16, owner-disposed per entry; commit e24759b).**
  The 2026-07-15 blocked-pool audit (report:
  `docs/process-journal/artifacts/2026-07-15-ledger-blocked-pool-audit.md`,
  commit ac5db2b) checked all 27 `status: blocked` ledger entries against the
  real tree: 12 ALREADY-DONE, 12 STILL-BLOCKED, 3 UNCLEAR, 0 FIRED. Each
  ALREADY-DONE claim was re-verified against the tree at disposition time.
  Dispositions: **12 closed** (settled + resolving occurrence); **2
  re-pointed** (core-56, core-66: `blocked_on` was a non-event justification,
  now `v1.x planning`); **1 reclassified** (exec-23: the one-shot watcher is
  the deliberate v1 design -> settled restraint). The 12 STILL-BLOCKED stay
  blocked (real future vehicles). Blocked pool in the ledger: 27 -> 14.
  Also resolved: **gui-22 vs exec-44 recorded-statement collision** - gui-22
  (keep all run logs) was superseded by D35 (exec-44, 14-day auto-prune,
  shipped); gui-22's statement now records the supersession, exec-44 carries
  the live rule.
  **Structural fix (Şenol ruling 2026-07-16): the plan-close gate gains a
  blocked-pool sweep step** (doctrine §3, plan-close step 1b). The audit's
  finding was that zero entries FIRED - the `blocked_on` condition does not
  drive the work, plans do, and the ledger learns late or never - so a
  "watch the condition" mechanism is the wrong shape and a periodic sweep at
  plan close is the right one. The deeper `blocked_on` redesign is not taken
  now; the sweep addresses the symptom, and whether the condition mechanism
  needs redesign can be revisited if the sweep keeps finding much stale.
- **Open rider: the audit covered only `decision-ledger.yaml`.** The other
  three house files carry 12 more `status: blocked` entries never swept
  (product-boundaries 6, conventions 3, process-conventions 3). The new
  plan-close sweep step covers all four going forward; a one-off sweep of
  these 12 is a small owner-disposition task whenever convenient (not
  blocking).

- **`ledger-lint` script (v1.x, or a rider on the next plan that touches
  `scripts/`)**: the ledger's anti-fabrication rule -
  `count == len(occurrences)`, every `ref` citing a real artifact - is enforced
  by nothing but controller care. It was hand-checked three times in session 15
  and held each time, which is exactly the state that ends badly once. A
  ~30-line script over the three YAML files makes it mechanical: count
  integrity, no empty refs, no `status: blocked` without a `blocked_on`, no
  Tier-2 entry with `promoted_at: null`. Same formal move as every other
  finding this session: replace "be careful" with a handle. Needs a dispatch
  (scripts/ is a product artifact), so it rides a plan rather than being
  written controller-side. (2026-07-15, session-15 extraction sweep)
  **SCRIPT WRITTEN 2026-07-16** (`scripts/ledger-lint.py`, commit 18decfe),
  ad-hoc without four-eyes on the owner's explicit authorization, overriding
  the dispatch-a-plan default above for this one helper. Checks all FOUR files
  (the three Tier-2 + the Tier-1 ledger, which the "three" above undercounted)
  plus duplicate-id; proven to fire on a wrong count. **Still open: CI wiring.**
  It is Python (a real YAML parse beats a fragile line parser for a linter that
  must be trusted), so gating it in CI adds a Python leg to a Rust+Node matrix -
  a deliberate deferral to the next CI-touching plan, so the gate is not just a
  script someone must remember to run. Until then it is a manual/local check.
  **DEFERRAL TRIGGER FIRED 2026-07-22 (S22, surfaced by the plan-8 design
  author's NEEDS_CONTEXT): Plan 8 adds release.yml = the next CI-touching
  plan. Controller ruling: Plan 8 absorbs the CI wiring as a rider task,
  bundled with the duplicate-key extension (entry above). The rider enters
  the plan brief at plan authoring; not a design amendment (nothing in the
  plan-8 design depends on it).**
  **PREDICTION CONFIRMED 2026-07-16, and the entry is now evidence-backed
  rather than precautionary.** An ad-hoc validator of exactly this shape,
  written to check two new entries, found a real pre-existing defect on its
  first run: `core-47-with-severity-builder` carried `count: 3` against 4
  occurrences - a `reinforced` occurrence appended 2026-07-13 (commit 89f346b,
  plan-5.6 task-2 verdict) without bumping the derived count. It had survived
  every hand-check since. Fixed in the same turn (count := len(occurrences) =
  4; `promoted_at` stays 3, which is correct - the list keeps growing past
  promotion). All 386 entries across the four files now pass the four checks
  above. The script's value is no longer hypothetical: it is a defect the
  human-care mechanism demonstrably missed for three days and a machine caught
  in one run.

## Near-1.0

- **Requirements-catalog derivation (product-baseline-desktop)**: at 1.0,
  mine this repo's registers, memos, and spec into a NEW
  product-baseline-desktop skill - the desktop-app counterpart to
  product-baseline-saas (renamed from product-baseline 2026-07-11; stays
  SaaS-scoped and is NOT fed from here). D34 (CSP for webview apps) is a
  named input. Decided 2026-07-11. Safety requirement (Şenol, same day):
  both skills carry mutually pointing, mutually exclusive descriptions -
  each names the other for the out-of-scope case - so skill selection
  never rests on inference alone.
- **Dependabot/Renovate activation**: Şenol's call, timing "when 1.0 is
  essentially done". Free since the repo went public; SHA-pinned actions
  and exact dep pins are ready for it. Two riders (2026-07-11, docs-tree
  sweep): prune the 18 commented RUSTSEC ignores in deny.toml as Renovate
  PRs obsolete them (S8); TypeScript is deliberately held at 6.0.3 under
  the typescript-eslint ceiling - Renovate will offer the TS-7 bump when
  the ecosystem catches up (S14). Cadence rationale (residue R3,
  recovered from Plan 1): every dep PR triggers the full 3-OS matrix, so
  the cadence choice is a CI-cost decision - the "monthly" lean exists
  because of that cost, not preference.

## v1.x candidates

Deferred with reasons; source: Plan-5 whole-branch triage (ledger, archived
at docs/process-journal/artifacts/plan-5-sdd/progress.md) and design memos.

- **UI polish pass ("schick machen") - deliberate 1.x item (owner ruling
  2026-07-21, S20)**: v1's visual bar is a reasonably good, usable layout;
  visual refinement beyond the existing design language is deferred
  wholesale to 1.x. Companion of the presentation-token carve-out
  (process-conventions latitude-carveout-presentation-tokens).
- **h1-scheme gate (machine-enforce the ratified topic-title form)**: the
  "Label (section)" scheme is entirely review-enforced today - check:i18n
  /D62 give zero signal on h1 text. A cheap check (h1 label-half ==
  catalog label for the labelled topics, exempt classes as a small
  allowlist) would make it structural. h1-normalization review harvest
  (S21, 2026-07-22). The section-derivation ruling landed 2026-07-22
  (S22): scheme + derivation rule + the closed exemption allowlist are
  now conventions.yaml `help-topic-h1-scheme`, so the gate's allowlist
  is enumerated, not open.
- **Block-specific tooltip in KeywordOrBlockWidget (budget +2 keys)**:
  the inner block section currently inherits the widget's generic
  tooltip (innermost-title-wins, benign); a dedicated block labelKey
  would sharpen the hover text at the cost of two catalog keys beyond
  the 46-key budget. Plan-7 T4 verdict harvest + whole-branch triage
  item 3; owner acknowledged as 1.x budget decision (S21, 2026-07-22).

- **Editor undo/redo, all operations (owner ruling S22, 2026-07-22,
  plan-7.5 kickoff)**: full undo/redo across every editor mutation -
  field edits, rule add/remove (incl. the deliberately unconfirmed
  delete), drag-reorder, list/map widget mutations. Ruled 1.x wholesale
  at the kickoff; at 1.0 the explicit-save model bounds the loss, and
  undo/redo - not a confirmation dialog - is the durable answer to
  accidental destruction. Design note for the 1.x pass: the editor's
  single in-memory model (Plan 6) is the natural command/snapshot
  boundary.
- **deb/rpm hard Depends on mkvtoolnix (owner S22, 2026-07-22)**: 1.0
  ships Recommends (first-run detection + per-OS guidance carries the
  absent-mkvmerge case); revisit promoting to a hard Depends in 1.x.
- **Homebrew Cask distribution (owner S22, 2026-07-22)**: publish the
  macOS dmg as a Homebrew cask in 1.x (own tap vs homebrew/cask
  submission, and the signing/notarization implications cask acceptance
  may raise, are checked then - not 1.0 packaging scope).
- **Remove mise from CI (post-1.0, Şenol 2026-07-12)**: mise is a
  dev-machine runtime manager, not a CI tool; CI should install node/pnpm
  directly (pinned setup action) rather than fetch a floating mise binary
  at run time via jdx/mise-action. Supersedes the routed pre-1.0
  supply-chain finding (the SHA pin covers the action JS, not the mise
  binary it downloads). Structural, hence post-1.0, not a pre-tag patch.
- NDJSON `--json-events` stream; `--fail-fast=now` (deferred pre-Plan-5).
- Joblog atomic writes (settings half was hardened in the Plan-5 fix
  wave). The AppState.active poison-recovery half of this line was
  promoted 2026-07-11 into the pre-1.0 worker-panic/mutex-hygiene gate.
- (Donor-side UnsupportedSource gate: briefly here per walkthrough #11,
  revised same day into the pre-1.0 diagnostics polish block when that
  wave moved to pre-1.0.)
- i18n check false-positive noise for shell IpcError codes (11 warn-only).
- RunMeta/summary.json cannot express joblog_status=incomplete (history
  view nuance).
- Per-run jobs control in the GUI (currently settings default only).
- Dialog-suppression setting (mkvtoolnix `m_warnBeforeAbortingJobs` parity,
  D31 note).
- Shared test-support crate if the mirrored fake-mkvmerge helpers grow
  beyond three copies.
- On-disk identification cache (spec 5.5 note).
- Richer joblog error type (distinguish per-job-write-lost vs summary-failed).
- Input-convenience parity features: see IDEAS.md 1-4 (kept there with
  their full analysis; NOT targets without a new decision).
- **Coverage tooling (cargo-llvm-cov)**: deliberately left out of the
  Plan-1 tooling stock-take ("signal we don't need yet") and never given
  a revisit condition. Şenol 2026-07-11 (sweep walkthrough #18a):
  re-discuss at v1.x planning - discussion anchor, not a commitment.
- **Spec §11 carries five named v1.x candidates** (pointer; full context
  and rationale live in the spec, single source): open-in-mkvtoolnix-gui
  escape hatch, `--sync` delay/stretch, Windows convenience downloader,
  `multi: true` wildcard rules, mkvpropedit metadata-only fast path.
  Mirrored 2026-07-11 (sweep walkthrough #19) so the forward tracker
  knows they exist.
- **Spec 8.4 / Renderer rustdoc still claim "v1 ships English content only"**:
  stale since the de locale shipped (Plan 5.5); one-line sweep of spec 8.4,
  non-goal 11 and the Renderer rustdoc on the next spec-touching plan.
  (plan-5.8 whole-branch verdict, out-of-range observation, 2026-07-14)
  Rider for the same sweep: **spec 10 names a lint rule that does not
  exist** - it credits "eslint (no-literal-string rule)" with keeping
  hardcoded strings out of the frontend, but eslint.config.js runs
  `@intlify/vue-i18n/no-raw-text` (deliberately, per D27, taking that one
  rule rather than the vue-i18n presets). Correct the rule name and, while
  there, the claimed scope: no-raw-text scans Vue template text nodes plus
  the configured static attributes, so it cannot see a runtime-generated
  label. (2026-07-15, Plan-6 brainstorming)
- **Spec 5.2 diagnostics table completeness**: the table omits rows for
  exactly 17 of 47 `diag_codes!` members (measured mechanically in the
  plan-7.5 fix round; cumulative drift - additions never got rows;
  no gate ties the table to the enum). If the table is the catalog of
  record, it wants ONE wholesale amendment batch on the next spec-touching
  plan, not one row per plan; the plan-7.5 design's amendment-2 row is the
  local symptom. (2026-07-22 S22, plan-7.5 design review round-1 harvest,
  controller watch item.) Cheap recount method for the batch, proven in
  the plan-7.5 plan review: extract the table's codes and the
  `diag_codes!` members, `comm -23` the sorted sets.
- **Cosmetic cleanup, one pass (sweep group K)**: duplicate section-header
  comment suggestions.rs:325 vs :291 (routed-items item 3, 2026-07-14);
  spec 5.2 NonUtf8Path row wording "per file" vs implemented "per path"
  precision (plan-5.7 whole-branch I3); dead `at` param
  (load.rs:56,64); invalid-template `*[empty-field]` default-variant
  mislabel trap; TracksCfg placement splitting the AttachmentsCfg group
  (model.rs); stale "Two tests:" module doc (command_integration.rs:4);
  Plan-1 archive remnants (xtask dedup tie-break, substring payload
  asserts - partially unreconstructable); eager chapters/attachments
  resolve on the discarded-plan path (planner.rs:541ff). (Walkthrough
  #21, group K.)
- **Batch-level settable-language check (D18 remainder)**:
  `changes.language` is validated only per-file at the application point;
  an invalid language on a never-matching optional rule stays undetected
  (inert-but-silent - never touches a muxed file; the match side has the
  batch walk already). (Walkthrough #21, group D.)
- **BCP-47 registry validate() (D19 remainder)**: well-formed-but-
  fictitious tags (`xx-YY`) pass plan-time and only fail when mkvmerge
  rejects them at mux time.
- **Signal/kill e2e decision (docs-tree S7)**: SIGINT end-to-end,
  second-Ctrl-C force-exit (exit 130) and Windows-kill e2e have no
  automated coverage; "not cheaply automatable" lived only in frozen
  briefs. Decide at v1.x planning: build vs deliberately never - record
  either way.
- **User-facing runs-root override (docs-tree S9)**: MUXSMITH_RUNS_ROOT
  is debug-only by design (run.rs:306, D26); a release-build override is
  a deliberate v1.x decision, not a side door. Şenol 2026-07-11: keep
  visible here.
- **Subsumption-lint extension (docs-tree S10, D2 scope cut)**:
  substring/regex values that provably never match a domain value are
  silent-never-match today ("subsumption-lint territory, out of v1
  scope" - D2 memo).
- **GUI settings-persistence errors are silent (docs-tree S13)**:
  background writes fail to console.warn only (BatchView.vue:96/117) -
  recent-profiles/directory-memory loss is invisible to the user.
- **Plan-5 whole-branch minors, one line (residue R2)**: default_jobs
  staleness (#1), double `--version` spawn (#6), dialog stacking (#9),
  missing NotFound test (#15) - loud/harmless GUI leftovers from the
  Plan-5 triage that never reached this tracker.
- **Apply-vs-editor concurrency guard**: one-click apply (Plan 6 T14)
  writes the profile file while the editor tab may hold an older model of
  the same file; nothing detects the divergence (last-writer-wins on the
  next save). A guard (mtime check, dirty-flag prompt, or shared model)
  is a deliberate v1.x design question. (2026-07-16, T14 report concern +
  amendment-4 routed note.)
- **Batch view auto-refresh after apply**: whether a successful one-click
  apply re-runs the dry run so the suggestion list reflects the new
  profile is design-silent (core-03 guarantees survival of the NEXT
  user-initiated dry run only); Plan 6 ships the null option. Adding
  auto-refresh is a user-visible product decision. (2026-07-16,
  amendment-4 routed note.)
- **regex compile cache (matcher.rs:74, residue R4)**: patterns are
  recompiled per matches() evaluation (per track x rule x file in bulk
  runs); fix is compile-once at validate/plan setup. v1.x WITH promotion
  condition: a bulk profile showing the compilation dominating promotes
  this into the pre-1.0 hardening block. (Recovered twice: four sweep
  reports carried it, then Peter's own 23-point condensation dropped it -
  the M3 mechanism catching its own auditor.)
- **check-i18n.mjs fixture self-test (T20-m2)**: the script grew real
  logic (parity + drift classes); its own fixture-based self-test per the
  plan T20 condition (the e2e real-parse guard covers only the
  parser-blindness half).
- **Test-hygiene collection (docs-tree B-minors, one pass)**:
  yaml_fragment type fidelity untested (bool/int suggestions must render
  unquoted, B1); substring-precondition lockout deserves an explaining
  line in the D6 memo (B2); Plan-3 ledger nits - blanket &M/&&Track path,
  attachment int-property (id/size) match tests, InvalidPropertyValue
  params dict unasserted, UnsupportedSource+UnknownPropertySkew co-firing
  untested (B4); --json document on list-languages failure tested
  cfg(unix)-only, Windows CI never runs it (B6); killer double-invocation
  idempotency contract untested (B7); save_settings without
  load_settings_from symmetry - naming unverified (B8); three
  close-abort-* keys not wording-pinned (B9); save-as fs-plugin path
  without direct e2e assertion (B10); RECENT_PROFILES_CAP duplicated
  TS/Rust with comment-only guard (B11). Discarded with reason: dead
  !any.is_empty() matcher guard (B3), i18n .attr scanner gap - fails
  loud (B13), C-list cleanup remnants. (Walkthrough #21, group D.)

- **UnknownExtension warning misfires on attachment `add` locators**:
  pre-existing, surfaced by the plan-5.7 whole-branch E2E (a recursive
  attachment locator under a non-media directory draws the warning meant
  for track-input extension lists). Trigger: next parity-audit round or
  diagnostics-surface work. (whole-branch verdict harvest, 2026-07-14)
- **Nothing gates `IpcError` codes against `gui-common.ftl`**: `DiagCode` is
  gated exhaustively (`catalog_completeness.rs` matches the enum, so a new
  code cannot compile without its catalog message), but the shell's
  `IpcError` codes are plain strings with no such guard, and
  `check-i18n.mjs` downgrades them to a warning by design. So an `IpcError`
  code can ship with no message, or with a message nothing renders. The
  visible cost, found by the D49 review: its proposed
  `apply-rule-index-out-of-range` line renders "the profile has 1 rules" in
  the singular case its own test constructs, and a Fluent plural selector
  **cannot** fix it - `IpcError.params` is `Record<string,string>` at every
  call site (`FirstRun.vue:94`, `RunHistory.vue:155`, `JobsView.vue:246/252`)
  and only `DiagnosticsPanel.vue:34` promotes numbers, keyed by *diagnostic*
  code, so `[one]` would always fall to `*[other]`. The house pluralizes
  everywhere else (`i18n-05-plural-selectors`). Two separable pieces: a
  presence gate for IpcError codes, and number promotion for IpcError params.
  Trigger FIRED and consumed 2026-07-21 (Plan 7 carries the help-id CI
  guard and check-i18n parity work = CI/gate structural work): FOLDED INTO
  PLAN 7 by owner scope call (S20; see the Plan 7 anchor). (2026-07-16,
  D49 review harvest item 5)
- **Gate part that Fluent-parses ALL catalogs**: the parse-all half is
  ALREADY BUILT - e2e/catalogs.spec.ts real-Fluent-parses every catalog
  of every locale incl. cli.ftl (the i18n-10 closure; this entry's
  original premise "today no gate Fluent-parses the de CLI/diagnostics
  catalogs" was refuted by plan-7 design correction #5, 2026-07-21, and
  the sentence is corrected here in place per design trigger 9). The
  remaining live content is the sharper plan-5.8 design-review variant:
  extend scripts/check-i18n.mjs to assert placeable-set and
  selector-structure parity per message id across locales (the de catalog
  header currently declares that parity manually reviewed, not
  machine-checked); would have auto-guarded the D39 selector change.
  Trigger FIRED and consumed 2026-07-21, same call as the IpcError entry
  above: FOLDED INTO PLAN 7 (S20 owner scope call; see the Plan 7 anchor;
  ledger i18n-12; design D55 rule 5). (T4 verdict harvest H1, 2026-07-14;
  plan-5.8 design-review harvest, 2026-07-14)
