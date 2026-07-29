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

## Plan 7.5: track-rule add/remove in the editor - EXECUTED AND CLOSED 2026-07-27

**Executed 2026-07-23/27** (4 tasks over two parallel worktree streams that
interleaved on master with Plan 8; one fix round on Task 2 after a mid-run
design amendment; merges e36885f + 33be397, nine-part gate green after each;
path-scoped whole-branch verdict READY). The owner rendered-surface pass was
ruled and executed in-session (406e91b: seven verbatim wording edits across
five files, including the v1 spec's 8.2 sentence). Salvage in
docs/process-journal/artifacts/plan-7.5-sdd/ (31 files). Roll-up funnel: the
whole-branch verdict's 15 triage items closed, landed via the owner pass, or
carried by a trigger below; blocked-pool sweep run across all four
house-knowledge files (24 blocked entries; one re-pointed off a non-event
justification, one flagged for resolution at the Plan-8 close). The anchor
below is history.

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
1.0 [superseded in part 2026-07-27, Plan 8.5 ruling 1: macOS ships
ad-hoc signed via `bundle.macOS.signingIdentity: "-"` - still no Apple
account, certificate or notarization; Windows and Linux stay unsigned]
(per-OS install-hurdle documentation ships with the release docs;
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

## Plan 8.5: macOS packaging fixes - EXECUTED AND CLOSED 2026-07-28

**Executed 2026-07-27/28**: four serial tasks on one tree (no worktrees, by
the plan's own reasoning - three config/doc tasks do not amortize four
worktree setups plus merge gates), one fix round on Task 1, one mid-run plan
amendment, whole-branch verdict READY with no blocking finding. Commits
9460daf, 5060ef5, 50e08cd, 87c1dee, plus the amendment 29ef17b; rehearsal run
30312889098 green, 6/6 jobs, all six machine halves with fired controls and
two differential measurements against the original defect artifact. **The
owner accepted all three rulings at their acceptance surface** - the installer
shows the unidentified-developer path instead of "damaged", the dmg mounts
with no licence dialog, the rendered body carries the three links on one line,
and INSTALL.md matches the flow he walked. Salvage in
docs/process-journal/artifacts/plan-8.5-sdd/. The anchor below is history.

## Plan 8.5: macOS packaging fixes (pre-1.0, owner rulings 2026-07-27)

The three findings of the owner's R8 walk-through, as their own small
package rather than folded into Plan 9: packaging domain, its own
verification loop (a rehearsal re-run plus a second walk-through on real
Apple-Silicon hardware), and that human latency is worth incurring early
rather than at the tag.

**KICKOFF RULINGS (owner, 2026-07-27), binding:**

1. **Ad-hoc signing YES.** `bundle.macOS.signingIdentity` gets Tauri's
   documented pseudo-identity `"-"`. This does NOT reopen the S22
   unsigned-at-1.0 ruling in substance - no Apple account, no certificate,
   no notarization - but it DOES change that ruling's wording, and the
   ruling text is updated in the same change rather than left contradicting
   the tree. Acceptance: quarantine a freshly built bundle and observe that
   the "unidentified developer" dialog appears where "damaged" appeared, so
   the flow `docs/INSTALL.md` documents is the flow that occurs.
2. **The dmg's pre-mount license goes away.** Owner's words: mounting a dmg
   and being met with a license dialog is odd under MIT. Preferred route:
   drop it for macOS only, keeping the Windows dialog that the same
   walk-through confirmed correct. **Owner tiebreaker, binding, so the
   implementer does not have to weigh it:** if removing it for macOS while
   keeping it on Windows turns out to need contortions, do NOT build the
   contortion - fix the rendering on the macOS side instead. KISS decides,
   not completeness.
   The empirical question this cascade turns on (established at the vendor
   reference, not from memory): there is NO per-platform license key -
   `bundle.license` and `bundle.licenseFile` are global, and no
   DmgConfig/WixConfig/NsisConfig/Deb/Rpm/AppImage section carries a
   license property. The documented lever is a platform-specific config
   file that overrides the bundle section, which is the same overlay
   mechanism this project already uses for the CLI sidecar (D82). Whether
   an overlay can CLEAR an inherited key, rather than only set one, is the
   thing to test first - and its answer picks the branch above.
3. **The release-body OS links** are joined onto one line (confirmed broken
   on the rendered draft; the same treatment covers the two other wrapped
   regions in that file).

Vehicle: one four-eyes plan, no separate design document - both design
questions are owner-ruled above and the third item is a one-line text fix,
so a design round would have nothing left to decide. Execution waits at the
owner's plan-approval gate as usual.

## Plan 9: core/orchestration hoists + planner seam - EXECUTED AND CLOSED 2026-07-29

**Executed 2026-07-28/29** over sessions 24 to 26: seven tasks, five amendments
(three design-side, two plan-only; amendment 5 is the first of this plan ruled
by the controller rather than the owner), four task fix rounds, and a
whole-branch fix round. Plan
`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`, design
`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
(D91-D105, eight spec amendments S-1..S-8). The whole-branch review returned
NEEDS_FIXES on two rustdoc claims the S-8 amendment had falsified, and READY
after the fix wave. Salvage in `docs/process-journal/artifacts/plan-9-sdd/`.
The anchor below is history; its open close items live in the close-action
blocks at the end of this section.

**RECON 2026-07-27 (session 23), inventory at
`.superpowers/sdd/plan-9/recon-inventory.md`, 1119 lines.** Read it before
designing; the anchor's own figures were measured and several are wrong.

Corrections to what this anchor and the ledger claim:

- The duplication is **260 lines (199 non-comment)** for the load-to-plan
  stretch, or 322/246 counting the specs gate - not the "~100 lines" the
  ledger entry `core-121` states. That figure originated as an estimated net
  CUT in an earlier analysis and was restated as the size of the
  duplication; off by a factor of 2.5 to 3.
- The `MUXSMITH_RUNS_ROOT` gate is at
  `crates/muxsmith-cli/src/commands/run.rs:330-335`, not the `run.rs:306`
  cited below.
- "Four copies", "eight IpcError render sites", "the single eprintln in
  queue.rs" and the JobsView line range all check out.

Findings that change what the design has to decide:

- **The four copies differ in seven DELIBERATE ways and six accidental
  ones**, enumerated and classified in the inventory. A hoist is safe only
  where a difference is a parameter; the seven deliberate ones include
  locate-vs-detect, a semantic split on `mkvmerge_found` the CLI cannot
  express, and a specs gate that exists on only two of the four. The design
  decides which become parameters and which stay separate - it does not get
  to discover them.
- **The chief accident: `profile::validate::config_diagnostics` has existed
  since Plan 5.6 T11, and only the four NON-pipeline callers were migrated.**
  All four pipeline copies still inline `validate::validate` plus
  `lint::provable_overlaps`. The earlier hoist was half done, which is a
  large part of why there are four copies to begin with.
- **The GUI's copy of the runs-root seam has no consumer at all.** Only five
  CLI test sites set the variable. Deleting is a candidate outcome, not
  merely hoisting.
- **`JobOutcome.errors` reaches no GUI surface and no CLI human output.** On
  a worker panic the CLI's human arm prints only `exit_code`, which is
  `None`, so the user sees `n/a`. The `worker-panicked` message exists in
  both locales and is looked up by nothing but the catalog-completeness
  test. This is a user-visible hole, not the cosmetic item the entry implies.
- **Bare `raw:` with an empty property name is silently never-matching**, not
  merely unvalidated: the rule emits an info diagnostic, then `get("")`
  returns false at match time, so the rule quietly matches nothing. Neither
  a rejection code nor its severity is fixed anywhere, and no ledger entry
  exists for it.
- **The `config_diagnostics` ordering gap is wider than recorded** - the
  GUI's `validate_profile` is the direct analogue of CLI `validate` and
  returns unsorted - and "ordering is cosmetic" is not obviously true,
  because `BatchView.vue:225` indexes `config_diagnostics[0]`.
- **The D23 re-check may be closeable without building anything**: the
  deviation was already adjudicated in round 2 of the plan-5 whole-branch
  review ("the implemented form is strictly better than my literal
  wording"), and the divergent branch is unreachable from the UI because the
  Run button is disabled by the same flag that gates it. What is missing is
  a test and a ledger entry recording the correction's form, not a fix.
- **The untested part of `start_run` is precisely its composition** -
  acquire, blocking plan, Soft/Ready, commit, runner thread - since
  everything not needing an `AppHandle` is already factored out. No Vitest,
  no `tauri::test`, and the Playwright mount harness globs only the editor
  widgets, so `JobsView` and `BatchView` cannot be mounted as they stand.


The "orthogonal to every GUI plan, can run as a parallel worktree stream"
note this anchor carried since 2026-07-12 is moot: Plan 9 is the last
planned package, so there is no concurrent plan to stream against. How its
own TASKS run is the plan's judgement call under the doctrine's earn-the-
overhead handle.

**DESIGN OWNER-APPROVED 2026-07-28**:
`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
(D91-D105), after one four-eyes review round (FOUR blocking findings, five
minor - counted from the verdict's `### I-` and `### M-` headings after the
first write of this line said three), one fix round, an APPROVED delta
review, and two closed wording notes. Its four triggers are mirrored into the Triggers section below; its
`gui-d23-reset-gating-form` ledger obligation is written;
`core-d49-g1g2-experiment` waits for the experiment's measurement and
`core-121`'s `blocked_on` clears at the plan close. The design is the
executable contract for the plan; the v1 spec stays authoritative above it.

**TWO LATER OWNER RULINGS, 2026-07-28, after both approvals** (routed as
amendment 1, `.superpowers/sdd/plan-9/amendment-1-brief.md`; design and plan
each amended by their own author and delta-reviewed by their own reviewer):

1. **A feature's tests ship with the feature.** The two acceptance
   observables recorded as having no producer - D101's GUI Run-gate
   consequence and the branch D103 edits - get real e2e scenarios in THIS
   plan. This overturns a controller routing, not an owner one: the fix round
   had restated them honestly and added no tests. Both scenarios fit the
   existing Playwright plus mock-IPC harness, so the infrastructure boundary
   below is untouched - scenarios in, infrastructure still 1.x. Tier-2
   `tests-ship-with-the-feature-never-after`.
2. **The GUI identification session cache is out** as overengineering. Both
   halves of D93 go: no caller-owned cache in the seam, no `AppState` field,
   `LiveIdentifier` unchanged. Spec 5.5's "shared between dry-run and run"
   clause is amended to describe what the product does rather than left
   contradicted. Ledger `gui-identification-cache-per-call-not-per-session`
   carries the accepted cost as its steelman: a GUI dry-run followed by a run
   identifies every file twice.

**SCOPE RULED BY THE OWNER 2026-07-28 (S24 kickoff), on the recon above.**
Eight of the ten named design inputs are IN; two are OUT and carry their
vehicles. The per-item rulings below are binding design inputs, not
suggestions: an item's ruled FORM (delete vs hoist, error vs warning, test
vs fix) is settled, its mechanism is the design's question.

IN:

- Hoist the four-copy planning pipeline into a shared plan_pipeline() core
  fn - this IS the injectable-planner-seam (S4/S5/S6), spec 5.5/7
  parity-critical; the duplication is 260 lines / 199 non-comment, NOT the
  "~100" this bullet claimed until 2026-07-28 (see the RECON block above for
  the measurement and for the seven deliberate divergences a hoist may only
  turn into parameters). plan_pipeline consumes
  profile::validate::config_diagnostics (landed Plan 5.6 T11), and migrating
  the four still-inlined `validate::validate` + `lint::provable_overlaps`
  pairs onto it is part of this item, not a separate one: the Plan-5.6 hoist
  was half done and that is part of why four copies exist. The seam
  INTERFACE is this plan's design question (ledger
  core-121-planner-seam-and-hoist). (2026-07-12, idiomacy review triage)
- Hoist run_batch into muxsmith_core::executor (the CLI inlines what
  src-tauri already factored). (2026-07-12, idiomacy review triage)
- Runs-root (D26 debug-only seam): **DELETE the src-tauri copy, do NOT hoist
  to core** (owner ruling 2026-07-28). The GUI copy has no consumer - only
  five CLI test sites set MUXSMITH_RUNS_ROOT, and the src-tauri tests pass an
  explicit path - and hoisting would move a cfg(debug_assertions) env read
  out of the two binaries and into the library, on a semantics claim the
  recon explicitly did not verify. The CLI gate stays as it is. Recorded as
  an ADR with the hoist as its steelman. (2026-07-12, idiomacy review
  triage; form ruled 2026-07-28)
- The worker-panic path, as ONE item (owner ruling 2026-07-28): core's single
  eprintln (T4-i1, ledger exec-36) and the never-rendered JobOutcome.errors
  codes (T4-i2, ledger exec-37) are two views of one hole - core prints the
  rich payload to a stream a bundled app has no console for, while the stable
  `worker-panicked` token renders on no surface and its Fluent message is
  looked up by nothing but the catalog-completeness test, so a panicked job
  shows the CLI user `n/a` and the GUI user nothing. Ruled form: the payload
  travels in the outcome and the SURFACES render it; **no logging facade in
  core** (a dependency for one producer, rejected - the steelman is the four
  silently-discarded failures in the recon's 4.3, which stay discarded).
  Scope of the rendering: CLI human output renders the catalog message in
  place of `n/a`, and the GUI job row carries the code in its failure state;
  the RunHistory export stays as it is. (2026-07-12, Plan 5.5 roll-up
  funnel; fused and ruled 2026-07-28)
- Reject bare `raw:` with an empty property name at validate (T16-m1), at
  **error severity with its own DiagCode** (owner ruling 2026-07-28) - not a
  warning, and not reused UnknownProperty, whose message reads as nonsense
  with an empty name. Today it emits an info diagnostic, exits 0, and then
  `get("")` returns false at match time, so the rule silently never matches;
  that is always a typo, never expressible intent. The exit-code change for
  profiles that pass today is accepted deliberately, because pre-1.0 is the
  cheap moment for it. Downstream obligations the new code carries: the
  catalog-completeness fixture table, both locales' diagnostics.ftl, and the
  spec's 5.2 severity table. No ledger entry existed for T16-m1; it gets
  one. (2026-07-12, Plan 5.5 roll-up funnel; form ruled 2026-07-28)
- Sort JSON config_diagnostics errors-first for validate parity (T9-m-iv;
  ledger cli-08), **centrally in the two core document builders, not per
  caller** (owner ruling 2026-07-28) - eight call sites would re-drift. The
  gap is not CLI-internal: the GUI's validate_profile is the direct analogue
  of CLI `validate` and returns unsorted for the same profile. The
  index-reading consumer moves in the same change as hardening: BatchView
  reads config_diagnostics[0] and switches to a code lookup. **Corrected
  2026-07-28 after the design author checked the site**: this line first
  claimed BatchView DETECTS the parse failure by that index read, which is
  wrong - detection is `!doc.profile` (BatchView.vue:218) and the index only
  FETCHES the diagnostic to display. Since the `profile: null` envelope
  always carries exactly the one ParseError diagnostic (its only two
  constructors are load.rs:62 and :71), the sort changes no BatchView
  behavior today; the code lookup is defence against a future second entry,
  not a fix. (2026-07-12, Plan 5.5 roll-up funnel; form ruled 2026-07-28)
- The D23 frontend-contract item is a **test plus a ledger entry, not a code
  fix** (owner ruling 2026-07-28). The recon established that round 2 of the
  plan-5 whole-branch review already adjudicated the deviation ("the
  implemented form is strictly better than my literal wording"), that the
  divergent branch is unreachable from the UI because the same flag disables
  the Run button, and that the event-ordering premise still holds in the
  code. What is missing is coverage and a record of the correction's FORM
  (gui-11 records the defect and "Corrected", not the form). The only
  harness work in scope rides here: widen the Playwright mount glob to
  JobsView.vue so the reset logic is mountable at all. **Scope correction
  2026-07-28, controller decision, recorded because it extends an
  owner-ruled boundary:** the glob widening alone cannot carry the test. The
  mount harness passes `spec.props` once at mount time and makes only
  `modelValue` reactive (`e2e/mount-entry.ts`), so a SECOND `pendingRun` -
  the double-dispatch case that is this item's whole reason to exist - is
  undeliverable. The design therefore adds a minimal reactive-props hook to
  the harness beside the glob entry. Judged in-scope as mechanics the ruled
  test requires, not as the 1.x harness the same ruling cut; if the owner
  overturns it, the test drops its double-dispatch assertion and keeps the
  rest. (2026-07-11, docs-tree sweep; re-pointed here from the Plan-6 anchor
  by owner call 2026-07-16; form ruled 2026-07-28)
- Run the D49 G1/G2 removal experiment - the "next core/planner-touching
  plan" trigger below fires with this plan. (Trigger consumed 2026-07-28.)

OUT, with vehicles:

- The GUI test-harness question as ONE block - no Vitest component harness,
  no tauri::test integration harness, start_run's orchestration body
  untested ("to raise at merge time"; the merge gate passed without it)
  (S4/S5/S6). (2026-07-11, docs-tree sweep.) **A FIRM 1.x item by owner
  ruling 2026-07-28, not trigger-gated**, vehicle: the v1.x entry "GUI test
  harness for the run path" below. New test infrastructure is its own package
  and does not block the tag; whether tauri::test's mock_builder even works
  on the pinned Tauri version was not established. No trigger, deliberately:
  the only event that would fire one is a run-path regression a component
  test should have caught, which is exactly the notice-it-yourself shape a
  trigger must not have. The cheap part that does land at 1.0 is the
  mount-glob widening under the D23 item above.
- One error-display funnel for IpcError rendering in the frontend: the
  plan-7 design review counted 8 scattered render sites each hand-rendering
  `$t(err.code, err.params)` (enumeration in its verdict file, salvaged
  with the plan-7 archive); a shared component/composable is the hoist.
  Discussion anchor, not a commitment. (2026-07-21, plan-7 design review
  round 1 harvest.) **Deferred to 1.x by owner ruling 2026-07-28**, vehicle:
  the v1.x entry "IpcError render funnel" below, with its trigger. It is
  design work rather than a mechanical hoist because one of the eight
  consumers is mixed: BatchView fills the same ref from a core Diagnostic as
  well as from an IpcError.

**A close action recorded at the ruling, so it is not left to anyone
noticing:** the five ledger entries these rulings touched -
`exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`,
`cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated` and the
new `empty-bare-raw-property-rejected-at-validate` - now all carry
`source: human`, so the promotion matrix promotes them at count 1. Promotion
is deliberately deferred to THIS plan's close rather than skipped: until the
work lands, each statement describes a tree that does not exist yet, and a
Tier-2 entry is always-loaded and binds every task that reads it. At the
close the roll-up funnel promotes them into their nature files, or records
per entry why one stays Tier 1.

**A second close action, surfaced by the Task-5 review 2026-07-28:** D102's
scope boundary - `config_diagnostics` sorts, while per-file `diagnostics` and
`batch_diagnostics` stay in collection order - is asserted in three normative
places (D102, spec S-7 which Task 5 itself added, and both builder rustdocs)
and guarded in none. Measured rather than argued: the reviewer widened
`batch_document` to sort all three arrays and `cargo test --workspace` stayed
at exit 0 with zero failures (`task-5-verdict.md`, no-work-needed check 2).
It correctly did not ride Task 5 - the boundary is preserved behaviour, not a
consequence that diff creates, so the four-condition execution-time rule does
not reach it. The cheap producer the review names: a `batch_document` case
with a mixed-severity `batch_diagnostics` vector asserting it is NOT
reordered. Disposition at the close is the owner's; the measurement is
recorded here so it cannot evaporate. **STILL OPEN after the plan-9 close
2026-07-29** - deliberately, and this line says why so nobody reads it as an
oversight: the close pass was scoped to text corrections and the gate's
definition, and this is a new test. The owner's test-coverage ruling does not
reach it (the boundary is preserved behaviour, not a consequence the plan's
diff created), so building it is a discretionary call that was never put to
him. **Carry it into the pre-1.0 gates as an owner question**, together with
the measurement above: widening the sort to all three arrays leaves the whole
suite green.
**RULED 2026-07-29 (owner, session-27 kickoff): BUILD IT.** The producer is an
IN item of the pre-1.0 product package, in the shape the Task-5 review named: a
`batch_document` case with a mixed-severity `batch_diagnostics` vector
asserting it is NOT reordered. The controller's argument that carried it: a
contract asserted in three normative places, one of them the spec, with no
producer anywhere is a claim nobody can rely on, and this producer is cheap.

**DONE 2026-07-29 (session 28), Plan 10 Task 2, commit `35bc363`** - and the
disposition records WHICH halves got producers, because a measurement decided
that rather than the plan. Four mutations, each applied alone to
`report/json.rs` and followed by the full workspace suite: the two SORTED
halves came back RED, i.e. already guarded, by
`dry_run_cli.rs::dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`
and `::dry_run_and_validate_json_agree_on_config_diagnostics_ordering`
respectively, so no producer was written for either. The two PRESERVED-ORDER
halves came back GREEN, i.e. unguarded, and each got its enumerated producer in
`crates/muxsmith-core/tests/report_json.rs`:
`batch_document_preserves_batch_diagnostics_collection_order` and
`batch_document_preserves_per_file_diagnostics_collection_order`. The task
reviewer re-ran all four mutations with its own function-scoped driver - needed
because the sort block occurs twice in the file, once per builder - and
reproduced the pattern exactly; the whole-branch reviewer reproduced it a third
time. No production code changed.

**A neighbouring coverage fact, surfaced by the Plan-10 author 2026-07-29 and
recorded here because the HANDOFF is volatile and a fact with no vehicle
evaporates.** The existing guard for the SORTED half of the same contract,
`dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`, is
`have_mkvmerge()`-gated. So on any machine without mkvmerge - and in principle
on a CI leg that lost it - that half is unguarded while the suite still reports
green. **MEASURED 2026-07-29 (session 28), so this is no longer a claim:** the
Task-2 reviewer applied the sort-removing mutation and ran that test under an
emptied PATH; it printed `mkvmerge not found; skipping` and exited 0, i.e. the
defect passes the suite. Not Plan 10's problem and deliberately not folded into
it: the fix is either a second producer on the no-mkvmerge path or a
gate-independent construction, and both are their own decision.
**Vehicle, reworded 2026-07-29 (session 28) because the first form fired on the
package it exempts by name:** whichever package AFTER Plan 10 next touches the
diagnostics ordering contract, or the owner QA pass if it surfaces the symptom
first. Plan 10's Task 2 does touch that contract and is exempt by the sentence
above, so the original wording made this entry contradict itself; the Task-2
reviewer surfaced it.

**DONE at the close 2026-07-29** (`9dc3a4d` + `c8dfc6d`). **Two text corrections routed to the close, from the Task-5 review and its
delta (2026-07-28).** Neither is user-visible; both are developer-facing prose
that a change falsified. They are held to the close rather than amended into
the running plan, on the same don't-fork-the-contract reasoning the gate edits
above use.

- **BatchView's else-branch** (`src/views/BatchView.vue`, the `!doc.profile`
  branch): after Task 5's code-keyed fetch it fires on two triggers - an empty
  `config_diagnostics`, and a non-empty one carrying no `parse-error` - while
  its comment and its `console.error` string still name only the first. D103
  anticipated the widening and ordered no text change, and Task 5's Step 4
  positively fenced the string, so the implementer correctly left both;
  amendment 3 is the precedent that a text a change falsifies is a design
  matter rather than a keyboard fix. The comment should name both triggers and
  the string should read as "no parse-error diagnostic" instead of "no
  diagnostics". No test asserts either string, so nothing else catches this.
- **Three overclaiming strings in `crates/muxsmith-cli/tests/dry_run_cli.rs`**
  (delta-review LOW-4): the `files`-is-an-array assertion the Task-5 fix round
  added is a SHAPE guard, not a builder discriminator - `config_only_document`
  emits `files: []` as well, measured against the real profile-load-failure
  document - so its message, the comment above it, and the pre-existing
  `mkvmerge_found` message each claim an identification neither assertion
  performs. The assertions stay; only the three texts change, and the delta
  verdict carries the exact replacement. The pre-existing message was off
  limits to the fix round, so this close action carries its licence.

**The anchor's IN items against their commits (close bookkeeping, 2026-07-29).**
Each ruled-IN item and where it landed: the planner seam and the four-copy hoist
in `9bbe53d` + `fed55be` (Task 1); the `run_batch` hoist into core and the
deletion of the src-tauri runs-root seam in `9b2843f` (Task 2); the worker-panic
payload end to end, plus the amendment-3 rustdoc restatement, in `9e5e112` +
`4e73739` (Task 3); `EmptyRawProperty` at error severity with its Run-gate e2e
scenario in `d768657` + `3412fcc` (Task 4); the central errors-first sort and
BatchView's code-keyed fetch with its parse-failure scenario in `e134fdc` +
`17505d8` (Task 5); the D23 tests on the widened mount harness plus the `name()`
hoist in `a2c1028` (Task 6); the D49 experiment measured and reported without a
commit, by design (Task 7). The two OUT items received their v1.x vehicles at
the S24 kickoff and are untouched by this plan. The whole-branch fix wave is
`b40db26`, `96dbcf6`, `e255d40`.

**DONE at the close 2026-07-29** (`9dc3a4d` + `c8dfc6d`; the disclosure was rewritten once more because its own safety clause enumerated the settings consumers exclusively - the very shape the neighbouring item exists to prevent). **A third text item, from the Task-6 review (LOW-1, 2026-07-29).** The
spec-local IPC installer in `e2e/jobsview-reset.spec.ts` answers `start_run` and
`list_runs` and throws on everything else, where the shared `installMockIPC`
also sets the OS-plugin platform global, forwards to the invoke recorder, and
answers the `get_settings` family. The omission is measured harmless today - the
only `platform()` consumer is outside `JobsView`'s subtree - and D104 fences the
spec-local mock composition, so the code stands. What is missing is the
disclosure: one sentence in that installer's doc comment naming what it
deliberately omits relative to the shared one and why that is safe, so the day a
new mounted read reaches it, the failure (one test throwing "unmocked command"
while its three siblings stay green) reads as expected rather than as a puzzle.

## Triggers

Observable events with registered consequences - CONSULT AT EVERY
MILESTONE GATE (visibility flip, release, tag, publication; doctrine §3).
Details live at the pointed-to entries; this list only names event ->
action:

- Renovate/Dependabot activated -> prune deny.toml RUSTSEC ignores; TS-7
  bump arrives via its PRs (riders on the activation entry, which moved into
  the pre-1.0 gates section by the 2026-07-29 owner ruling).
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
  the inputs distributed. **Plan 9's instance FIRED AND WAS CONSUMED
  2026-07-28** at the S24 kickoff: all ten inputs were recon'd first, then
  ruled item by item into the anchor's IN/OUT lists, and the two OUT items
  received their v1.x vehicles in the same change. This retires the trigger:
  no plan after 9 is planned.)
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
  (Plan-7.5 delta review observation, 2026-07-22 S22. FIRED AND CONSUMED
  2026-07-27: salvage 8e2c044, re-pointing d5a6470, same push. The
  consuming sweep found a SECOND design citation - task-2-verdict.md -
  and re-pointed both; the plan's own close-actions bullet and this entry
  keep the pre-salvage path deliberately, because they record what the
  trigger said rather than pointing a reader at a live artifact.)
- The `ubuntu-22.04` runner's deprecation or retirement is announced -> move
  the Linux release leg to `ubuntu-24.04` AND record the raised glibc/webkit
  floor in docs/INSTALL.md and the tar.gz README requirement line in the same
  change. (Plan-8 design trigger 1, D85.) **FIRED, discovered 2026-07-29
  (session 28) while answering an owner question about the runner split.**
  GitHub's announcement (actions/runner-images issue 14254): the Ubuntu-22-based
  images begin deprecation **2026-09-17** and are fully unsupported
  **2027-04-17**, with brownout periods failing jobs in between; the recommended
  targets are `ubuntu-24.04`, `ubuntu-26.04` or `ubuntu-latest`. **CONSUMED
  2026-07-29 (session 28) by owner ruling: release on 24.04, tests stay on
  26.04.** It was his decision and not the controller's because the trigger's
  prescription moves the product's REACH, not just a pin. Landed in `e260845`
  (the three pins, the policy comment, the two requirement texts), `d9a4fa2` and
  `c38bb0b` (the reach claims in the install notes and the release body) and
  `709929c` (the rpm's EPEL requirement on RHEL, the tar.gz row, the pin
  comment's own coverage). **One half of the prescription is a measured no-op
  and is recorded as such rather than silently dropped:** the trigger names a
  raised "glibc/**webkit**" floor, and the webkit half does not move - Debian 13
  ships webkit 2.52.5 against noble's build-time 2.52.3 and the package name is
  unchanged. What the follow-on reviews then found is not in the trigger's
  prescription at all: on RHEL 10 the rpm needs EPEL for `webkit2gtk4.1`, which
  stock repositories do not carry. Measured floors: Ubuntu 22.04
  carries glibc 2.35, 24.04 carries 2.39, 26.04 carries 2.43; Debian 12 carries
  2.36, Debian 13 carries 2.41. So building on 24.04 drops Ubuntu 22.04 and
  Debian 12 users; building on 26.04 would drop Debian 13 as well. The owner's
  stated instinct at the session-28 close was that the release and test legs
  should run ONE sensible distro version rather than two; the counter-argument
  on record is the vendor's own AppImage guidance to build on the oldest base
  you intend to support, plus the fact that testing only on the release base
  stops testing what current systems actually run. **He ruled the middle:** the
  release leg moves to a supported LTS, the test leg stays on the newest, and
  the split survives with its reason intact. **Where the floor is now stated is
  deliberately NOT enumerated here** - an enumeration of texts is what went
  stale twice in one afternoon; the handle is to grep the tree, which is also
  what `release.yml`'s own policy comment now says.
- A DATED windows-arm64 runner label appears (today only the undated
  `windows-11-arm` is GA) -> pin it, closing D85's recorded deviation from
  pin-everything. (Plan-8 design trigger 2.)
- A `@tauri-apps/cli` / `tauri` major or minor bump lands -> re-verify the
  four bundler facts the design pins at tauri-cli-v2.11.4 before the next
  release: externalBin landing spots (msi INSTALLDIR, `Contents/MacOS`,
  `/usr/bin`), the wix template's PATH-feature inertness, the config-version
  to Cargo fallback, and `--bundles` value coverage. (Plan-8 design
  trigger 3.)
- The signing revisit fires (the unsigned-install-hurdle trigger above) ->
  beside the per-OS signing config: shrink docs/INSTALL.md per its embedded
  comment, and evaluate GitHub artifact attestations FIRST (D90's record).
  (Plan-8 design trigger 4.)
- The Intel-dmg request fires (the macOS x64 trigger above) -> add a
  `macos-x86_64` leg on `macos-15-intel`, extend D89's asset set and D77's
  body table, and decide universal-vs-second-dmg then (D78 records why
  universal lost at arm64-only scale). Per the owner's 2026-07-27 call the
  work is a v1.x commitment in its own right, so this trigger only
  accelerates it. (Plan-8 design trigger 5.)
- A user asks for a portable Windows build -> the mkvtoolnix Windows-7z
  parity gap becomes a v1.x candidate: a `windows-x86_64.zip` from the
  existing msi leg's binaries, D88-style. (Plan-8 design trigger 6.)
- A user asks for a German installer UI -> reopens D86's single-language
  `wix.language` decision; the mechanism is a per-language map carrying a
  locale file (amendment A2), and the cost is more msi artifacts or a
  transform decision, which is why it waits for a request. (Plan-8 design
  trigger 7.)
- A tar.gz-equivalent bundler lands in Tauri, or cargo-dist earns its keep
  for the archive leg -> revisit D88's hand-packed step. (Plan-8 design
  trigger 8.)
- The `gh` CLI on the runner images breaks a release-ops invocation (it
  floats with the pinned runner image - the one unpinned tool in the path)
  -> pin gh by direct versioned download in release.yml, same shape as every
  other pin. (Plan-8 design trigger 9.)
- A request for site-specific wording or tooltips on the editor's generic
  action keys (any site) -> reopens the shared-key question as an OWNER
  decision; the latent-coupling steelman recorded in
  `editor-generic-action-keys` is the argument that fires. Default stays
  shared keys. (Plan-7.5 design trigger 3, D68/D72.)
- Core changes `EmptyMatchExpression`'s severity or the skeleton rule's
  emission set -> D65's recorded semantics and e2e case 6's
  Save-stays-enabled assertion re-verify; a flip to error severity changes
  the save gate, which is an owner-visible product change, not a silent
  ride-along. (Plan-7.5 design trigger 4.)
- An accidental-rule-deletion report arrives -> route to the v1.x editor
  undo/redo entry below, NOT to a confirmation dialog (D66 records that
  rejection with its steelman). (Plan-7.5 design trigger 5.)
- The owner wants the rule grid's Add/Remove buttons help-annotated after
  all -> that is a D54 id/host-set owner change reopening D71's resolution,
  not an implementation nicety. (Plan-7.5 design trigger 6.)
- A FOURTH e2e spec file needs the local `name()` helper -> hoist it into a
  shared e2e helper module. Three local copies (smoke, editor-markers,
  editor-rule-add-remove) are the established house pattern and hoisting now
  would touch every site; the fourth copy is where the pattern stops paying.
  (Plan-7.5 T1 review I4, 2026-07-27; same shape as the fake-mkvmerge
  three-copy trigger above.) **NOT FIRED; CONSUMED EARLY BY OWNER RULING
  2026-07-28** (Plan 9 amendment 2, Task 6 Step 3). The condition was never
  met and the controller's first note here wrongly said it had fired: the
  trigger requires a fourth spec file that NEEDS the helper, and Plan 9's new
  `e2e/jobsview-reset.spec.ts` asserts through `data-testid`/role selectors,
  so it consumes nothing. (It is also not the fourth spec file - nine exist
  today, `ls e2e/*.spec.ts`; the "fourth" in this trigger always counted
  HELPER COPIES, of which there are three.) The controller read the condition
  to the end of its first clause, the plan review caught it, and the owner had
  already ruled the hoist done now anyway - so the work stands on his ruling,
  not on a fired condition. Ledger:
  `a-triggers-condition-is-read-to-its-last-clause`.
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
  D58's path gate in the same change. (Plan-7 design trigger 7. Did NOT fire
  at the Plan 9 kickoff 2026-07-28: that fix is not among the ten inputs the
  owner ruled into Plan 9, so despite the "Plan-9 neighborhood" wording this
  trigger still waits for whichever change actually does the lookup fix.)
- A JSON consumer asks to distinguish absent-from-too-old mkvmerge in a
  report document (an issue, a script author's request) -> add the resolver
  failure reason as a document field, the mechanism D92 designates (a reason
  on `MkvmergeUnavailable` surfaced as an `mkvmerge_error` value), as an
  additive wire change. Until then `mkvmerge_found: false` means "no usable
  mkvmerge was resolved" on every surface. (Plan-9 design trigger 1, D92.)
- A user-filed report about a stale partial output or an unexplained cleanup
  failure arrives -> promote `delete_partial_failed` from a raw `errors`
  pass-through to a catalog-rendered condition, using D98-D100's
  typed-field-plus-catalog pattern as the template. (Plan-9 design trigger 2,
  D98's recorded deferral.)
- A user asks for collision-policy control in the GUI -> wire the existing
  `on_collision` seam parameter to a Batch-view control; the pipeline needs
  no change, the parameter is already there. (Plan-9 design trigger 3,
  D91/D-3.)
- The D49 experiment lands on the only-G3 branch -> the owner rules on G1/G2
  removal at a plan close, against the `core-d49-g1g2-experiment` ledger
  entry; until ruled, the guards stay per `proc-proposed-safeguard-stays`.
  (Plan-9 design trigger 4, D105.) **NOT FIRED 2026-07-29, and its condition is
  now unreachable as written:** Task 7 measured the anomaly branch, and the
  reason was the instrument - the fenced mutation site also feeds the engine's
  candidate construction, so the degraded candidates never reached the guards.
  No re-run of D105's protocol can produce the only-G3 branch. **RE-AIMED by
  owner ruling 2026-07-29**, on the recommendation that the D49 question stays
  open until after 1.0 rather than being re-run now: the new condition is **a
  re-fenced D49 experiment is run at all** (mutating the APPLIER site only -
  `apply_suggestion`'s call to `delta_for` - which the Task-7 reviewer measured
  in an isolated crate copy turns all three guards red through their own
  assertions). Then, and only then, the owner rules on G1/G2 removal against
  `core-d49-g1g2-experiment`. Until such a run happens, all three guards stay
  per `proc-proposed-safeguard-stays`. The re-fencing itself is not scheduled:
  its most likely outcome is "all three load-bearing, keep everything", so it
  buys a confirmation that nothing changes, which is not what the run-up to 1.0
  is for.
- A NINTH IpcError render site is added anywhere in the frontend -> the v1.x
  "IpcError render funnel" entry stops being a candidate and becomes the
  answer: eight hand-rolled `$t(err.code, err.params)` alerts are the
  established shape, and the ninth is where the pattern stops paying. Same
  shape as the fake-mkvmerge three-copy and e2e `name()` helper triggers.
  (Plan 9 kickoff, owner ruling 2026-07-28.)
- A user asks for quieter UI / tooltip suppression -> the mkvtoolnix
  `uiDisableToolTips` parity gap becomes a v1.x candidate with precedent
  cited (plan-7 design section 3). (Plan-7 design trigger 8.)
- First external-user complaint about unsigned-install hurdles (or a
  comparable adoption signal) -> re-evaluate code signing/notarization
  per OS. (Plan-8 kickoff ruling F1, 2026-07-22 S22.)
- A macOS Intel (x64) user asks for a build -> PULL FORWARD the v1.x Intel
  entry below rather than treat the request as the decision. Superseded in
  its gating role by the owner's 2026-07-27 call: Intel support is wanted
  regardless of whether anyone asks and is a 1.x commitment, so a real
  request only changes its timing. (Plan-8 kickoff ruling F4, 2026-07-22
  S22.)
- Next core/planner-touching plan -> run the D49 G1/G2 removal experiment
  (mutate delta_for's AddExact arm to re-stringify, run the suite: G1+G2+G3
  all fail -> they stay for good; only G3 fails -> G1/G2 are removal
  candidates as localizers). Runnable since D49 landed (T6); guards stay
  until the run per proc-proposed-safeguard-stays. **FIRED 2026-07-28**:
  Plan 9 is core/planner-touching, so the experiment is an IN item of its
  anchor above and the trigger is consumed by that plan's task, not by this
  line. **RUN AND CONSUMED 2026-07-29, outcome INCONCLUSIVE.** The two
  outcomes this line enumerates are the design's two clean branches, and
  neither occurred: measured G1 green, G2 red, G3 green, which is D105's
  anomaly branch. The cause was measured and is the instrument, not the
  guards - `delta_for` feeds both the engine's candidate construction and the
  applier, and the engine re-validates its own candidates, so every degraded
  candidate was replaced by its NOT-polarity twin before a guard could compare
  anything. All three guards stay. Recorded in ledger
  `core-d49-g1g2-experiment` with a controller-composed statement, because
  D105 fixed wording for its two clean branches and none for the anomaly it
  also mandated be recorded. Disposition of the other
  plan-6 triggers at the close: trigger 2 SETTLED by measurement during T4
  (guard 2 fires in its literal-expected form, stays for good; recorded in
  the falsifiability entry); triggers 4 and 7 FIRED and were consumed
  mid-plan (12a extended the ts export set; settables.ts = second
  committed-generated-plus-drift-check instance, own ledger entry at count
  2); trigger 6 needs no entry by design (drift check and registry fail by
  construction).

- A gate command is written across two lines with a `|` or `&&` at the end
  (rather than a backslash) -> the gate-count check counts it as two commands
  and the stated total goes red for the wrong reason, or worse, a rewritten
  block silently reconciles. The counter models backslash continuations
  explicitly and REFUSES on them; these two forms it does not model at all.
  Known boundary, ruled no-action at the Plan-10 close (whole-branch review
  NIT 5, 2026-07-29): no gate block uses either form today, and widening the
  counter to shell grammar is more machinery than the invariant is worth.
  Readable event: you are about to wrap a gate command.

- A FOURTH GATE BLOCK is added to `BUILDING.md` (a fourth marked command block
  beyond `rust`, `frontend` and `house`) -> widen `scripts/ledger-lint.py`'s
  fixed three-marker set AND the canonical sentence's four-number shape in the
  same change. Until then a fourth marked block is INVISIBLE to the check: the
  stated total is compared against the sum of the three known blocks, so it
  stays green while the real gate has grown. This is a property of what Plan 10
  Task 1 shipped, not a defect in it - the plan fixed the marker set at three -
  and it is recorded here so the next author of a gate section knows the check
  will not catch them. Readable event: you are adding a `<!-- gate-block: ... -->`
  marker. (Plan-10 Task-1 review finding 5, 2026-07-29.)

- A FOURTH DECLARATION SITE of a `.generated/` harness-bundle path constant
  appears -> export `HARNESS_PATH` and `MOUNT_HARNESS_PATH` from their owning
  files (`e2e/mocks.ts`, `e2e/mount.ts`) and import them everywhere instead.
  Three declaration sites exist today - those two plus
  `e2e/jobsview-reset.spec.ts`, which had to re-declare both because neither
  constant is exported and both owners were off Task 6's Files list. Same
  three-copy shape as the fake-mkvmerge and `name()` triggers. **The condition
  counts DECLARATION SITES, not consumers**, stated explicitly because the
  `name()` trigger's recorded defect was exactly that mismatch between its prose
  and what it counted. Readable event: you are typing
  `resolve(import.meta.dirname, ".generated/` in a file that owns neither
  constant. (Plan-9 Task-6 review INFO-2, 2026-07-29.)

## Pre-1.0 release gates

Must be resolved before the first tagged release.

- **OWNER QA GATE, ruled 2026-07-29 (session-27 kickoff): no 1.0 release
  happens before Şenol has run a manual QA and bug-hunting pass himself.**
  This is a precondition on the tag, not a review of finished work, and its
  output is first-class scope input in the three shapes he named: real bugs;
  behaviour he does not like even where it matches the spec; and items
  currently parked in v1.x that he decides he wants in 1.0 after all.
  Consequence for planning, stated because it is the part that is easy to
  forget: finishing every other item in this section does NOT close the
  pre-1.0 scope, so any claim that 1.0 is content-complete before that pass
  has run is premature by construction. The pass needs a current build on his
  hardware; arranging that build is itself pre-1.0 work.
  **TIMING, owner 2026-07-29: the full product pass comes LATER, probably once
  the next plan is implemented.** Round 1 covered the install paths only - three
  OSes installed and launched, the documented steps and SHA commands confirmed,
  the macOS CLI symlink verified, one finding. The product itself is untested:
  a real dry-run and run over his own library, the profile editor including rule
  add and remove, suggestion apply, the jobs view during a live batch with a
  mid-run cancel, run history, the locale switch, help mode. This timing does
  NOT relax the gate - it moves when the gate is satisfied, not whether - so the
  window between now and that pass is the window in which 1.0 scope is still
  unknown.
- **OWNER QA PASS, round 1 (2026-07-29): install paths CLEAN on all three OSes,
  one finding.** He installed and launched on all three platforms, confirmed the
  documented steps and the SHA256SUMS commands in `docs/INSTALL.md` are correct,
  and verified the macOS CLI symlink. **The finding, on Fedora:**
  `Warning: skipped OpenPGP checks for 1 package from repository: @commandline`
  during `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`, the command
  `docs/INSTALL.md:82` documents. **The tool is `dnf`, not the `rpm` binary** -
  this line first said "during the rpm install" meaning the rpm package, which
  the Plan-10 author flagged as readable the other way; `@commandline` is dnf's
  pseudo-repository for a package given by path, which is what makes the tool
  identifiable from the warning at all. Expected in substance - agent and CI builds are
  deliberately unsigned, which is the same policy the macOS ad-hoc signing note
  documents - but `docs/INSTALL.md`'s Linux section does not mention it, so a
  first-time installer meets an unexplained security-shaped warning and has no
  way to tell it from a real problem. **RULED 2026-07-29: signing is deferred as
  a 1.x item** (see the v1.x entry below), so the disposition here is to DOCUMENT
  the warning in `docs/INSTALL.md`'s Linux section the way the macOS Gatekeeper
  flow is documented - what it means, that unsigned artifacts are the deliberate
  policy, and that it is not a defect. **Vehicle: Plan 10 amendment 2**, which
  widens Task 4 from a README pass to a user-facing-documentation pass.
  **THE FINDING IS CLOSED 2026-07-29 (session 28), commit `e657263`**: the note
  sits beside the Linux section's existing "no gatekeeping dialog exists on
  Linux" sentence, carries the warning string byte-identical to the record in
  this entry (checked by grep against this entry rather than against the plan),
  attributes it to `dnf` rather than the `rpm` binary - `@commandline` is dnf's
  pseudo-repository for a package given by path - and claims nothing about what
  the deb path prints, because nobody measured that. The file-top comment's
  enumeration of the sections that shrink when signing lands was extended by the
  same edit, since the note is a third member. **THE ENTRY ITSELF DOES NOT
  CLOSE:** the owner QA gate is a standing precondition on the tag, further
  rounds are expected, and artifact signing stays a ruled 1.x item.
- **TWO OPEN VULNERABILITY ALERTS, surfaced 2026-07-29 within minutes of the
  owner enabling the alert feed.** The feed's first act was to find something,
  which is the argument for having enabled it before Renovate rather than with it.
  - **`postcss` HIGH**, npm, transitive in `pnpm-lock.yaml` at 8.5.16, patched
    at 8.5.18. Path traversal in source-map auto-loading leading to arbitrary
    `.map` file disclosure. **Build-time only** - postcss runs during
    `pnpm build` and ships in nothing - and the attack needs untrusted CSS
    input, which this project does not have. So the real exposure is low while
    the severity label is high, and the fix is a transitive lockfile bump rather
    than a pinned-dependency decision. Recommend fixing it anyway and soon: it
    is cheap, it is a public repo where the alert is visible, and "low exposure"
    is an argument that ages badly.
  - **`glib` MEDIUM**, Rust, transitive in `Cargo.lock`, vulnerable
    `>= 0.15.0, < 0.20.0`, patched at 0.20.0. Unsoundness in the `Iterator` and
    `DoubleEndedIterator` impls for `VariantStrIter`. It arrives through the
    Tauri/GTK stack, so whether it can move independently of Tauri's own tree is
    the first thing to establish, not the fix.
  - **The gap worth more than either alert:** `cargo deny check` is gate part 5
    and it is GREEN on this tree, while GitHub reports a Rust advisory.
    `deny.toml`'s ignore list does not mention glib - it covers the unic family,
    proc-macro-error, the archived GTK3 crates and quick-xml - so this is not a
    silenced advisory. The likely explanation is that RustSec classes
    unsoundness as `informational` rather than as a vulnerability and our
    `cargo deny` configuration does not fail on that class, but that is a
    hypothesis and must be measured before it is repeated. **Two mechanisms we
    rely on disagree, and until the disagreement is explained neither can be
    quoted as coverage.**
  - **RULED 2026-07-29: its OWN one-task vehicle, not a Plan-10 rider**, on the
    controller's recommendation and its reasoning - the fix has nothing to do
    with Plan 10's five work items, and a package reopened for every incoming
    finding stops being a contract. Three parts, in one task:
    1. **Bump `postcss`** past 8.5.17 through the lockfile. It is transitive, so
       this is a lockfile decision rather than a pinned-dependency one.
    2. **Measure the `cargo deny` disagreement in the same task**, because it is
       cheap to measure and the result decides whether gate part 5 has a hole.
       The hypothesis on offer - RustSec's `informational` class for unsoundness
       not failing our configuration - is a hypothesis, and the task's output is
       the measurement, not the hypothesis restated.
    3. **`glib`: investigate only, do not fix.** Establish whether it can move
       independently of Tauri's own dependency tree. If it cannot, the finding is
       that this is an upgrade project rather than a bump, and it gets its own
       vehicle rather than being forced here.
    Sequencing against Plan 10 is deliberately unconstrained: neither touches the
    other's files. Renovate would raise the postcss fix by itself once live -
    security updates bypass the schedule by the owner's ruling - but it is not
    live and its config lands in Plan 10's Task 3, so this vehicle exists to own
    the window rather than to duplicate what Renovate will later do.
- **CONFIG DONE 2026-07-29 (session 28), Plan 10 Task 3, commit `630d418`.**
  `renovate.jsonc` is on `master`, expressing the ruled cadence and shape,
  validated clean by the vendor's own validator in both plain and `--strict`
  mode, and all fifteen premises the plan fenced were re-verified at the vendor's
  SOURCE (not its rendered docs, where `prHourlyLimit`'s default reads as
  `prConcurrentLimit`'s).
  **THE OWNER'S TWO ACTIONS WERE ALREADY DONE IN SESSION 27, and the controller
  wrongly carried them as open** through the plan-10 close, into this
  disposition and into the HANDOFF - corrected on the owner's objection and
  verified on GitHub: the Renovate app opened and closed PR #1 on 2026-07-29,
  which only an installed app can do, and the vulnerability alerts arriving the
  same day are the dependency-graph half. The session-27 HANDOFF said so
  plainly; the plan's acceptance row said the opposite, and the row won because
  nobody re-read the HANDOFF against it. **What remains is a wait-and-see, not a
  task:** the vendor documents a config commit as a route to onboarding but does
  not say it overrides an ALREADY-CLOSED onboarding PR, so confirm Renovate
  actually starts now that the config is on `master` - the observable is a
  dependency-dashboard issue appearing, and none exists yet (checked
  2026-07-29). The documented fallback, if it stays silent, is renaming the
  closed PR. **The trigger below therefore still has not fired in its recorded
  sense** - the deny.toml RUSTSEC pruning and the TS-7 bump wait for Renovate's
  first PRs, not for an owner action.

- **Dependabot/Renovate activation: FIRM PRE-1.0, owner ruling 2026-07-29**
  (session-27 kickoff), superseding the earlier "Şenol's call, when 1.0 is
  essentially done" formulation, which left the timing to a later decision.
  Free since the repo went public; SHA-pinned actions and exact dep pins are
  ready for it. Two riders (2026-07-11, docs-tree sweep): prune the 18
  commented RUSTSEC ignores in deny.toml as Renovate PRs obsolete them (S8);
  TypeScript is deliberately held at 6.0.3 under the typescript-eslint ceiling
  - Renovate will offer the TS-7 bump when the ecosystem catches up (S14).
  Cadence rationale (residue R3, recovered from Plan 1): every dep PR triggers
  the full 3-OS matrix, so the cadence choice is a CI-cost decision - the
  "monthly" lean exists because of that cost, not preference. **The cadence is
  SETTLED 2026-07-29 (session 28), when `renovate.jsonc` landed (`630d418`):**
  monthly, written as the three-day window `* * 1-3 * *` rather than the
  `schedule:monthly` preset, because that preset is a four-hour window on the
  1st and the hosted service documents that repo config cannot tighten its own
  backend cadence - so a four-hour window can be missed entirely, for months.
  **Re-validating that file later takes `--no-global`**: naming a file sets the
  validator's `configType` to `global`, measured at tag `43.287.0` in
  `lib/config-validator.ts`, so the plain form is silently blind to
  repo-config defect classes (a `globalOnly` option, a `global:` preset). The
  Task-3 reviewer proved both directions with probes - `repositoryCache` and
  `global:disableInherit` each pass the plain form and fail under `--no-global`
  - and confirmed this file is clean under both.
- **DONE 2026-07-28 (Plan 8.5, owner-accepted on hardware). BLOCKER: the macOS dmg's app does not launch - "the app is damaged"**
  (owner R8 walk-through on real Apple-Silicon hardware, 2026-07-27; the
  first human execution of the documented install path). Measured on the
  rehearsal artifact from the Linux side: the two Mach-O binaries in
  `Contents/MacOS` each carry an embedded signature blob (the arm64
  linker's ad-hoc signature), but the bundle has **no
  `_CodeSignature/CodeResources`** - it is not sealed as a bundle - and
  `bundle.macOS` configures no `signingIdentity`. **CONFIRMED on the owner's Mac 2026-07-27**: `xattr -dr
  com.apple.quarantine` on the installed app makes it launch. So the chain
  holds - a quarantined bundle whose executable looks signed while the
  bundle carries no seal fails Gatekeeper validation, which produces
  "damaged" instead of the "unidentified developer" prompt
  `docs/INSTALL.md` documents, and the documented Settings > Open Anyway
  flow therefore never appears. The app itself is sound; the packaging
  state is the defect.
  Candidate fix, mechanism verified at the vendor's current docs rather
  than from memory: set `bundle.macOS.signingIdentity` to the
  pseudo-identity `"-"`, Tauri's documented ad-hoc form - one config line,
  no Apple account, no certificate, no notarization, so NOT the code
  signing the S22 ruling deferred. The same docs state ad-hoc signing
  still requires the user to whitelist the app in Privacy & Security,
  which is exactly the flow INSTALL.md already describes. It IS a change
  to the "unsigned on all three OS at 1.0" wording and wants an explicit
  owner decision; the result then needs its own verification (quarantine a
  freshly built bundle, observe which of the two dialogs appears).
  Whatever lands, `docs/INSTALL.md`'s macOS section is re-verified against
  the real flow afterwards.
- **DONE 2026-07-28 (Plan 8.5, owner-accepted on hardware; the licence was REMOVED rather than repaired - mkvtoolnix ships none either). BLOCKER-adjacent: the dmg's pre-mount license (SLA) mis-renders the
  publisher name** (same walk-through). `bundle.licenseFile` is
  `../LICENSE`, whose copyright line carries `Ş` as UTF-8 `c5 9e`
  (verified by hexdump). Observed: the name garbles at that character and
  the first three letters of the following word render bold - consistent
  with a style-run table keyed on offsets that the two-byte character
  shifts, not merely a wrong code page. **This is the third sink the WiX
  diagnosis named** (publisher, the LICENSE text inlined into the
  installer, copyright): Windows was fixed by pinning the installer
  database code page, and nobody then asked whether the macOS installer
  carries the same sink. It does.
  **Parity input (SI-3, read at the source):** mkvtoolnix's
  `packaging/macos/build.sh` builds its dmg with a plain
  `hdiutil create -srcfolder ... -volname ...` and attaches NO license
  agreement - no SLA or LPic resource exists anywhere in its macOS
  packaging. The pre-mount click-through is therefore not parity; it is a
  Tauri default we opted into via `licenseFile`, and MIT requires no
  acceptance step. Dropping it removes the defect class instead of
  repairing it. **Constraint any fix must respect:** `bundle.licenseFile`
  is GLOBAL, not per-OS, and the Windows license dialog - which the same
  walk-through confirmed correct - is fed from it, so deleting the key
  outright would take the working Windows dialog with it. (Second parity
  note from the same file: mkvtoolnix DOES codesign both the .app contents
  and the dmg when an identity is configured, and re-signs after
  `install_name_tool` invalidates the signature.)
- **DONE 2026-07-28 (Plan 8.5, owner-accepted on the rendered draft). Release-body OS links break into separate paragraphs** - confirmed on
  the rendered draft, which is what the deferred owner wording item asked.
  `.github/release/draft-body.md` lines 2-4 begin with `|`, and GitHub
  renders them as their own blocks instead of one line. Fix: join the three
  links onto one line (the same question applies to the two other wrapped
  regions in that file). (Owner R8 inspection, 2026-07-27.)
- **CLOSED by the same walk-through**: the `ansicpg1252` residual recorded
  at the plan-8 close (whether the Windows installer's license dialog
  renders `Ş` correctly, unverifiable without real hardware) is resolved -
  the owner reports the Windows install correct throughout, license dialog
  and Programs-and-Features publisher included. The upstream cosmetic
  concern does not materialize.
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
  anchors implemented, provenance in this entry's git history). **Split by
  owner ruling 2026-07-29** (session-27 kickoff), because the two halves have
  different preconditions:
  - **Rides the pre-1.0 product package now**: re-check the CLI reference and
    the exact-typed-matching paragraph against the shipped surface (reviewer
    warning: "easy to lose"), and write in the content anchor below.
    **DONE 2026-07-29 (session 28), Plan 10 Task 4, commits `e657263` +
    `845cf89` + the close fix wave `1805949`.** The CLI surface was re-derived
    from the BINARY rather than the source: twelve rows examined at flag
    granularity, five divergent and corrected - the two blanket claims that
    every subcommand takes `--json` and `--locale` (false for `schema`, which
    accepts only `-h`), `--on-collision`'s value domain, `--locale`'s default,
    and exit code `130`, which the sentence had omitted entirely. The
    exact-typed-matching paragraph lost two absolutes the code does not support,
    and the content anchor's four items were written in, each verified against
    its spec section AND its core symbol. Widened by amendment 1 to carry the
    paragraph's two stale counts, both now naming the unit they count: the
    decision series as a count plus a reach (103 decisions reaching D105, the
    series being non-contiguous, so a range would state a false count as a side
    effect) and the verdict figure re-measured at 225 after this close's own
    salvage moved it. Two review rounds found four further defects in the prose
    the pass produced, including an exit-code claim that would have misled
    someone scripting `dry-run`; all landed. **The second bullet below stays
    open for the tag pass.**
  - **Stays for the tag pass, deliberately**: the four `placeholder(1.0)`
    comments (GIF, dry-run output snippet, release artifacts, GUI screenshot)
    and the WIP banner. The owner ruled the placeholders may remain as
    placeholders for now, on the reasoning that the README already states
    publicly that the project is a work in progress. Two of the four need
    artifacts that do not exist yet (real release asset names, a running GUI)
    and two are his taste call (what the GIF and the screenshot show). The
    banner's own text ties it to the tag ("until the 1.0 tag"), so dropping it
    is a tag-time edit and a 1.0 README still carrying it would contradict
    itself.
  - Content anchors (append every "README-worthy" remark HERE the moment
    it is uttered; fold into the README on next touch):
    - Şenol 2026-07-12: properties with language-like matching MAGIC must
      be EXPLICITLY LISTED in the README - language (ISO/BCP-47
      normalization + dual-field language/language_ietf lookup), absent
      boolean flags comparing false for exact, type/codec_kind curated
      domains; contrast with raw:'s no-magic byte-exact single-field rule
      (D32 addendum, B-8 ratification).
- **Guide + blog posts (process + product): MOVED TO 1.x, owner ruling
  2026-07-29** (session-27 kickoff). They are written after 1.0 has shipped,
  not as a condition of the tag - so this entry no longer gates the release
  and only lives in this section because its format decisions were taken
  here. Everything below stays valid and is what the 1.x round executes;
  only the timing changed. Format interview DONE 2026-07-11; decisions:
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
    seam): the four-copy planning pipeline (260 lines / 199 non-comment as
    measured 2026-07-27, not the "~100" this line carried until 2026-07-28;
    spec 5.5/7 parity-critical - a shared plan_pipeline() IS the
    never-decided injectable-planner-seam S4/S5/S6); run_batch hoist to
    muxsmith_core::executor; runs-root resolution - ruled 2026-07-28 as a
    DELETION of the src-tauri copy rather than a hoist to core (see the Plan
    9 anchor).
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

## CI: make the workflow path-aware - firm 1.x item (owner ruling 2026-07-28)

Today every push runs the full three-OS matrix, including a push that changes
only Markdown or the house YAMLs. The owner ruled the LOCAL gate stays
exception-free (measured: 11 seconds on an unchanged tree, so the exemption
would buy nothing and cost an arguable boundary - ledger
`does-the-ten-part-gate-bind-doc-only-pushes`), and ruled that making CI
itself skip what a diff cannot affect is a 1.x item rather than pre-1.0 work.

Not trigger-gated: it is committed for 1.x. Two things for that round to get
right, both known traps rather than research: a `paths-ignore` filter makes a
job SKIP rather than pass, which turns any required status check into a
permanent pending on doc-only pushes (the usual cure is a no-op sibling job
reporting the same name); and `ledger-lint` must keep running on exactly the
pushes the rest of the matrix would skip, since a house-YAML edit is the one
diff shape that can turn it red.

## Gate: rustdoc does not link-check private items - DONE at the plan-9 close 2026-07-29

**Landed in `9dc3a4d` with its fix round `c8dfc6d`:** `--document-private-items` on the doc step at BOTH consuming sites, `ledger-lint` in the gate block as its eleventh part, and the two pre-existing ambiguous `[`run`]` links in `src-tauri/src/lib.rs` repaired in the same change - without which the first run under the flag would have gone red. The reviewer proved the flag load-bearing by restoring the broken links and measuring the same tree red with the flag and green without it. `BUILDING.md` also gained `pnpm build` in its frontend block, because the file enumerated ten commands while every consumer derived eleven from it. The record below is the history.

Measured 2026-07-28 (Plan 9 Task 1 review, HARVEST): gate part 4
(`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`) skips the docs
of private items, and `src-tauri` puts `run`, `error` and `settings` behind
private `mod` declarations - so a dangling intra-doc link in any of them
passes every gate run and every CI leg. The task's own LOW-2 was exactly
that: an import removal left `[`plan_batch`]` unresolvable at
`src-tauri/src/run.rs:359` and nothing could see it.

**The heading said "private modules" until 2026-07-28 and the blind spot is
wider than that**: it covers any private ITEM, including one inside a PUBLIC
module of a public crate. Measured at the Plan-9 Task-2 review with its fire
(the reviewer injected a broken link on the private `fn worker_count` inside
the public `muxsmith_core::executor::queue`: the plain gate passed it, the
same run with `--document-private-items` failed). `queue.rs` alone holds
three such private functions, so `src-tauri` is not the only exposed crate.

**The recurrence, recorded because it is the argument for the flag:** the
same defect fired again one task later, same file, same cause - Task 2's
import removal broke `` [`run_queue`] `` at `src-tauri/src/run.rs:4`
(ledger `an-import-removal-sweeps-the-doc-links-that-named-the-symbol`).
Two consecutive tasks, both invisible to the gate, both caught only by a
review.

The fix is one flag: `--document-private-items` on the doc step - **at BOTH
consuming sites**, `BUILDING.md:76` and `.github/workflows/ci.yml:94-98`
(Task-1 delta review, 2026-07-28; this line first named BUILDING.md alone,
which would have left CI carrying the blind spot the change exists to
remove). Windows-leg cost and doctest exposure were both measured empty. **Its cost is
TWO one-line fixes** - this line said one, then three, and both figures were
measured at a moment that has since moved. The one unresolved link was Task
1's own LOW-2 and commit `fed55be` fixed it, so it stopped counting; what
remains are the two ambiguity errors, `src-tauri/src/lib.rs:54` and `:87`,
where ```[`run`]``` is ambiguous between the `run` function and the private
`run` module. Ambiguity is a different diagnostic under the same
`broken_intra_doc_links` lint and is equally fatal once warnings deny, which
is why `grep -c "unresolved link"` is the wrong instrument for this cost.
Both predate Plan 9. The repair is ```[`mod@run`]``` or ```[`run()`]``` per
site. Controller measurement 2026-07-28 at commit `9b2843f`, pasted from the
run of `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
--document-private-items`: exactly those two errors plus the resulting
`could not document muxsmith-gui`, and no unresolved link anywhere in the
workspace; the control run without the flag passes.

**Deliberately NOT done mid-plan**, because the plan quotes the ten-part gate
verbatim in its Global Constraints and every task's verification cites it;
changing the gate under a running plan would fork the contract every task
executes against. **Vehicle: done at the Plan 9 close**, in the same pass as
the close actions, or by whichever plan next touches BUILDING.md - whichever
comes first.

**A second edit rides the same pass** (owner ruling 2026-07-28): the gate
block gains `python3 scripts/ledger-lint.py` as an ELEVENTH part. It is
already binding as a pre-push duty from that ruling (Tier-2
`ledger-lint-runs-before-every-push`) - what waits for the close is only its
appearance in BUILDING.md's list, for the same don't-fork-the-contract reason.
Both edits touch the same block, so they are one change; the plan's own
"ten-part gate" wording is updated in the same pass, and any count that says
ten is recomputed.

## Docs accuracy

- **ONE member of the comment line-citation class SURVIVES Plan 10's sweep, and
  it is outside the corpus's file selector** (Task-5 review finding 1,
  2026-07-29). `.github/workflows/ci.yml` carries a comment citing
  `queue.rs:73`, which is itself already stale - at HEAD that line is
  `pub struct QueueOpts {` while the link the comment means sits two lines
  later. Both prescribed corpus expressions read only `*.rs *.ts *.vue *.mjs
  *.js *.py`, so no sweep of that corpus could ever see it; the reviewer found
  it by deriving the cited-extension set from the tree's own extension tally and
  running it over every non-`docs/` file (exactly one hit, fired control 1,
  negative control 0). **Consequence for how this package is described: the
  class is NOT closed tree-wide.** Any statement that Plan 10 closed it names
  the selector - source files in those six extensions - or it is false.
  **Vehicle: whichever package next edits `.github/workflows/ci.yml`.**
  **OPEN OWNER QUESTION riding the same line:** the ruling
  `comments-locate-by-symbol-never-by-line-number` was scoped by the owner to
  SOURCE comments and explicitly not widened, and its comment-form enumeration
  names `//`, `///`, `//!` and `/* */` - not `#` or `<!-- -->`. Whether a CI
  workflow comment is in scope is his call, not the controller's.

- **"byte-exact" overstates what `raw:` does for NUMERIC scalars** (Plan-10
  whole-branch review finding 4, 2026-07-29; verified by the controller at the
  source the same turn). Spec 4.4 / 9.2 and `matcher.rs`'s own comment at the
  `raw:` arm call the comparison an untyped byte-literal value equality, and the
  README's matching-magic list item 4 repeats it - but that arm calls
  `scalar_eq`, which carries two CROSS arms: `Scalar::Int` against
  `PropValue::Float` and `Scalar::Float` against `PropValue::Int`, each
  comparing after an `as f64` conversion. So `raw:x: 6` matches a reported
  `6.0`, which is exactly the coercion "byte-exact" promises not to do. The
  behaviour is old and deliberate-looking; only the WORDING is wrong, in three
  places at once, and the README's version is a faithful transcription of the
  spec's (correct by the precedence rule, so not a Plan-10 defect). **Vehicle:
  whichever package next amends the v1 spec** - same vehicle as the 8.1
  synopsis item below; both are spec-wording repairs and the README follows
  whatever the spec settles.

- **The v1 spec's section 8.1 synopsis omits `validate`'s flags** (surfaced by
  Plan 10's Task 4, 2026-07-29, while re-deriving the CLI surface from the
  binary; verified by the controller the same turn). The spec line reads
  `muxsmith validate <profile>` while `dry-run`, `run` and `identify` each carry
  `[--json]`; the shipped binary's `validate --help` lists `--json` AND
  `--locale`. So the drift runs OPPOSITE to the README's, which overclaimed the
  same flags for `schema`: here the authoritative document underclaims a real
  surface. Small, developer-facing, and outside every Plan-10 task's Files list
  (no task edits the spec). **Vehicle: whichever package next amends the v1
  spec** - a spec amendment sweeps the spec for self-contradictions anyway, and
  this is one.

- **DONE at the plan-9 close 2026-07-29** (`9dc3a4d` + `c8dfc6d`): all four sites corrected in count and in kind, the recount independently reproduced twice (13 snapshots, 5/3/4/1/0), and the two commits behind the `cli_validate` delta named. **D64's snapshot claim went stale in COUNT and in KIND when Plan 9 Task 4
  landed** (surfaced by that task, sharpened by its review, 2026-07-28; every
  affected file is outside every Plan-9 task's Files list). Four sites, each
  measured at the Task-4 review:
  - `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1505`
    ("the 11 insta snapshots stay en-pinned"), `:1556` ("`cli_validate.rs`
    (1 constructor, 3 snapshots)"), `:1563` ("covers all **11 insta
    snapshots**").
  - `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md:80` - the same
    enumeration with its arithmetic spelled out ("5 helper call sites, 3
    snapshots ... 3+3+4+1+0 = 11").

  Recounted 2026-07-28 at commit `3412fcc`: 13 snapshots total
  (`cli_validate` 5, `dry_run_cli` 3, `run_cli` 4, `run_live` 1, so
  5+3+4+1+0 = 13), and `cli_validate.rs` now has 6 funnel call sites plus one
  through the locale-parameterized helper. **The controller's first note here
  said 12/4 and went stale within the hour, when the amendment-4 fix round
  added the thirteenth - recorded because it is the same defect one level up.**

  **The KIND half matters more than the count**: after amendment 4 the German
  snapshot does not ride the `muxsmith` funnel at all, it rides
  `muxsmith_localized`, so recounting to 13 would still assert a false
  coverage claim. **Vehicle: the Plan-9 close**, and it needs TWO edits, not
  one - the numbers, and a restated coverage sentence along the lines of
  "every CLI-invoking snapshot test rides a pinned helper, the en funnel or
  its locale-parameterized construction site". D64's actual invariant holds
  unchanged, re-measured: `cargo_bin("muxsmith")` appears in exactly one file,
  which amendment 4 preserves by construction.

- **`per planning call` now reaches one step past its source** (Plan-9
  close-pass delta review, 2026-07-29). Both `identify.rs` docs were corrected
  to "per call", and the phrase survives in exactly three prose sites: spec 5.5
  itself, the plan-9 design quoting that spec sentence verbatim, and the Tier-2
  entry `core-20-ondisk-cache`. **Nothing here is false**: spec 5.5's sentence
  is scoped to the plan/run flow, where it is true. What changed is that the
  corrected type doc cites spec 5.5 for a property stated more broadly than the
  spec states it - a citation reaching past its source, not a contradiction.
  Closing it would be a spec amendment, so it is RECORDED here rather than
  given a vehicle; the reviewer recommended exactly that. Reconsider if a
  reader is ever misled by the gap, or when spec 5.5 is next amended for
  another reason.

- **A stale line citation in a test comment** (Plan-9 Task-7 review, finding 5,
  2026-07-29): `crates/muxsmith-core/tests/suggestions.rs:1015` cites
  "delta_for's two exact-bearing arms, planner.rs:1812, :1817"; at HEAD those
  arms are at `:1823` and `:1828`, and the cited lines fall inside a different
  function's closing braces. Pre-existing and outside Task 7's empty Files list,
  so it was correctly routed rather than fixed as a drive-by. **Vehicle:
  whichever task next owns `suggestions.rs`**, never a drive-by edit. The
  underlying shape - a comment citing line numbers in another file - is the
  reason the house convention is to locate by content.

  **The class is bigger than this one site, measured 2026-07-29 (session 27)
  while verifying the citation for a plan brief.** Ten comments in tracked
  `.rs` files cite a `<file>.rs:<line>`; four of them point inside their own
  file and are cheap to keep true. Of the cross-file ones:
  - `crates/muxsmith-cli/tests/run_live.rs:361` reads "same `Set` value,
    run.rs:274-275". It is a present-tense pointer, it does not say WHICH
    `run.rs`, and the cli crate's `commands/run.rs:272-277` is unrelated code
    (the debug-only `MUXSMITH_RUNS_ROOT` override). Same defect as the
    `suggestions.rs` one, and currently tracked nowhere.
  - Three citations of `report/json.rs:44` (`run_live.rs:273`, `:326`,
    `planner.rs:2226`, plus one same-file at `report/json.rs:161`) mean the
    PRE-FIX panic site of D40 - `:326` says "pre-fix: panics at" in so many
    words. Those are historical records, not live pointers, and repairing them
    to today's line numbers would falsify them. Exactly the live-claim versus
    historical-record distinction the gate-count entry above ran into.
  - `suggestions.rs:1035` citing `matcher.rs:202-212` still lands on
    `scalar_eq`'s match arms and is currently accurate.

  **RULED 2026-07-29, and the ruling is a convention, not a disposition of this
  site.** The owner: a comment referencing a file AND a line number is not how a
  developer writes, because lines move; name the symbol - which method, which
  class, which namespace - and referencing the FILE is fine, its staleness risk
  being real but far smaller. Recorded as Tier-2
  `comments-locate-by-symbol-never-by-line-number`. Two consequences:
  - The scope question above is answered: **sweep the class, all of it.** It is
    its own task in the pre-1.0 package, not a rider on `suggestions.rs`.
  - **Corpus: 20 comment lines across 13 files**, in Rust, TypeScript and Vue.
    The controller's first count said 17 and was wrong: the search enumerated
    the CITED file extensions and left out `.md`, so it widened the source
    languages and not the cited types. Refuted by the plan author against the
    tree and re-measured by the controller. The three missed sites
    (`crates/muxsmith-cli/tests/run_live.rs`, citing `README.md` line spans) are
    the sharpest case for the ruling that exists: the same package's README task
    edits that file, so those spans would go stale INSIDE the package that is
    repairing the class. A mis-enumerated set in a measuring position produced
    the undercount - the same defect shape the ruling addresses, one level up.
  - The live-pointer versus historical-record judgment **disappears** rather than
    being made: under the ruling the historical ones lose their line numbers too,
    and "pre-fix: panicked in `batch_document` while building the `Set` plan
    value" is both true and durable. Nothing has to be classified.
  - **SWEPT 2026-07-29 (session 28), Plan 10 Task 5, commit `1a23283`**, and the
    corpus was larger than this entry's own figure: **24 matched lines across 16
    files**, rewritten as **21 comments**, because the rewrite unit is the
    comment and three citations continue onto a line the pattern cannot see. The
    20/13 above counts only the filename-citation form; a second expression
    catches four bare `:<line>` spans with no filename, which a controller ruling
    folded IN as the worse form of the same defect rather than exempting on a
    property of the search pattern. Both absence checks are empty on the end
    state and each is fired by its own pre-state run; the controller reproduced
    both independently (0 on the end state, 20 and 4 against the pre-state).
    **NINE of the 24 citations were already stale**, each verified at its target,
    and one had never pointed at its target in any committed tree - its target
    moved in the same commit that wrote the comment. **The class is closed WITHIN
    THE CORPUS'S SELECTOR** - source files in six extensions - and NOT tree-wide:
    one member survives in `.github/workflows/ci.yml`, with its own entry and
    vehicle at the top of this section, plus an open owner question about whether
    the ruling reaches CI and config comments at all.

## Gate-count derivation has no check (candidate, from the plan-9 close pass 2026-07-29)

`BUILDING.md` is the gate's single authoritative enumeration, and at least six
other files derive a number from it with the words "per BUILDING.md". Nothing
checks that the derivation still matches: the close pass produced exactly that
divergence (the file enumerated ten while the ruling and every consumer said
eleven), and it was caught by a reviewer reading both, not by any tooling. The
house statements that could drop the number have since been rewritten to name
the file instead, which shrinks the surface but does not close it - a HANDOFF or
a plan still has to state a count to be useful.

**A neighbouring class, surfaced by the Plan-10 author and ROUTED here
2026-07-29 so it is not left in a plan step, which is not a tracker.**
`BUILDING.md` also carries POSITIONAL gate ordinals. **Three sites, not the two
this entry named until 2026-07-29 (session 28)** - re-measured at commit
`ddb8f42` with `grep -nE 'part [0-9]|parts [0-9]' BUILDING.md`, which is the
enumeration this entry now carries: `:102` "The cross-target clippy run
(part 6)", `:134` "runs Rust-gate parts 1-4", and `:135` "what part 6
cross-checks". The third was found by Task 1's implementer, not by the author of
this entry, and it hides in the shape this house already has a rule for: it sits
in the SAME paragraph as `:134`, hard-wrapped across the line break, so a
by-paragraph reading sees one ordinal where there are two
(`proc-wrapped-prose-quote-grep`). They are Rust-block-local
positions rather than totals, so the canonical-total check does not cover them
and covering them would need a second parser over a numbering the canonical
sentence does not define. They are also newly ambiguous once the file states a
total: "part 6" acquires a second possible referent, the sixth of eleven, which
only section context resolves. **A fourth item rides the same vehicle, added
2026-07-29 (session 28):** Task 1's fenced Step-1(e) replacement leaves
`BUILDING.md:138` at 86 characters, the file's only non-fenced prose line over
80 (the pre-state had none). A reflow has zero rendered effect and was correctly
NOT done inside Task 1, because the fenced paragraph carries a within-file
qualifier naming the very ordinals a reflow would move. **Vehicle: whichever
package next edits `BUILDING.md`'s gate blocks after Plan 10's Task 1 lands** - deliberately not
Task 1 itself, whose scope is the total and its check, and where adding a second
numbering concern would widen a task the owner approved at its current size.

**Candidate, not a commitment:** `scripts/ledger-lint.py` is the natural home
for a cheap invariant - parse `BUILDING.md`'s check blocks, count the commands,
and compare against any "N parts per BUILDING.md" claim in the tracked docs.
Weigh it against the alternative of simply never writing the number outside
`BUILDING.md`. The reviewer that surfaced this explicitly left it out of its own
pass's scope.

**MEASURED 2026-07-29 (session 27), and the measurement kills the cross-file
form above.** Tracked files stating a gate part count: 12 outside the process
journal, 143 including it. Ten of the twelve are retired plan documents, which
are history by principle - their "nine-part gate" was true when written.
`ROADMAP.md`'s own six hits are historical statements about closed plans, and
`process-conventions.yaml`'s single hit is an occurrence `ref`, i.e. a dated
event log entry. So EVERY current occurrence is a record of what the gate was
at the time, and a lint comparing them against today's count would fire on all
of them and demand that history be falsified. The distinction between a live
normative claim and a historical record is not visible in the text, so the
cross-file check cannot be built correctly.

**What the measurement exposes instead is the real root:** `BUILDING.md` is
called the single authoritative enumeration but never states the TOTAL. It says
"The Rust gate (six parts...)" per section and leaves the reader to assemble
eleven from three sections. A derived number with no canonical statement is
exactly what diverged at the close pass.

**NARROWED FORM, controller decision 2026-07-29, IN the pre-1.0 package:**
`BUILDING.md` states the total once, canonically, and a check verifies that the
stated total equals the number of commands actually enumerated in the gate
blocks - one file, no cross-file matching, no history problem, no false
positives. The check must be fire-verified (change the stated total, watch it go
red) rather than shipped on the strength of a green run, since a check whose
passing result is an absence proves nothing until it has been made to fire once.
New documents keep citing "per BUILDING.md"; nothing polices their prose,
because the only live consumers are plan documents and a plan's four-eyes review
reads BUILDING.md anyway.

**DONE 2026-07-29 (session 28), Plan 10 Task 1, commit `ddb8f42`.** `BUILDING.md`
states the total once behind a `gate-total` marker; each of the three command
blocks carries a `gate-block` marker; `scripts/ledger-lint.py` - already a gate
part, so no new part and no recursion - compares the stated per-block numbers
and the stated total against the commands the marked blocks enumerate, refuses
on a backslash continuation rather than miscounting, and skips the comparisons
whose inputs a missing marker makes underivable. Five fires, each with its
pasted red state, plus two the implementer added and a tolerate-green exclusion
probe the reviewer added; the count moved into the canonical sentence and out of
the Rust-gate heading. **The cross-file form stays killed** - the MEASURED block
above is the record, and the count in every other tracked file is a historical
statement about a closed plan. **Two boundaries of what shipped are recorded as
ROADMAP triggers rather than left implicit:** a fourth marked gate block would be
invisible to the check, and a command wrapped with a trailing `|` or `&&` is not
modelled.

## Ledger hygiene

- **ledger-lint duplicate-key gap (S21, 2026-07-22)**: a duplicated YAML
  key inside an entry (observed: a doubled `steelman:` line) passes the
  lint silently - the parser's later-key-wins swallows it. Extend
  ledger-lint with a per-entry duplicate-key check; trigger: the next
  ledger-lint or house-file-schema touch dispatches it alongside.
  **TRIGGER FIRED AND CONSUMED 2026-07-22 (S22)**: Plan 8's ledger-lint
  CI wiring (next bullet) is the ledger-lint touch; the extension rides
  the same Plan-8 rider task.
  **DONE 2026-07-27** (Plan 8 Task 5, commit 92c62f1): per-entry
  duplicate-key check via a SafeLoader subclass, fixture fire-verified.
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
- **Rider CLOSED 2026-07-27 (plan-7.5 close sweep).** The 2026-07-15 audit
  had covered only `decision-ledger.yaml`, leaving 12 `status: blocked`
  entries in the other three house files never swept (product-boundaries 6,
  conventions 3, process-conventions 3). The plan-7.5 close ran the
  plan-close sweep over all four files - 24 blocked entries, the recorded
  12 among them, counts matching per file. Result: one entry re-pointed
  (`core-84-regex-recompile`, whose `blocked_on` read "later cleanup pass",
  a non-event justification of the same class the 2026-07-15 audit
  re-pointed twice; now the v1.x entry plus its promotion trigger); one
  flagged for resolution at the Plan-8 close (`ci-13-packaging-deferred`,
  blocked on Plan 8, whose work has landed - and whose statement still
  carries the stale "while the repo is private" premise); one occurrence
  owed to `ci-16-mise-not-ci` from D85's mise-free release legs. Everything
  else stays legitimately blocked on a real future vehicle.

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
  **DONE 2026-07-27** (Plan 8 Task 5, commit 92c62f1): a `ledger-lint` CI
  job with a pinned PyYAML in a throwaway venv, green on the pinned head
  (run 30276189309). A red ledger now mechanically blocks a release
  rehearsal, since the release guard consumes the ci-run conclusion.
  Rider from the plan-8 whole-branch review (commit ecab53a): the inlined
  loader construction had moved outside the parse `try`, so a file with an
  illegal control character raised a traceback instead of the linter's own
  does-not-parse line - introduced by the wiring commit itself, not
  pre-existing, and fixed with the fire test that proves both states.
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

## v1.x candidates

Deferred with reasons; source: Plan-5 whole-branch triage (ledger, archived
at docs/process-journal/artifacts/plan-5-sdd/progress.md) and design memos.

- **Requirements-catalog derivation (product-baseline-desktop): 1.x, owner
  ruling 2026-07-29** (session-27 kickoff; it stood at "at 1.0" from
  2026-07-11 until then, and moved for the same reason as the guide and blog
  posts - it is work ABOUT the finished product, so it is not a condition of
  the tag). Mine this repo's registers, memos, and spec into a NEW
  product-baseline-desktop skill - the desktop-app counterpart to
  product-baseline-saas (renamed from product-baseline 2026-07-11; stays
  SaaS-scoped and is NOT fed from here). D34 (CSP for webview apps) is a
  named input. Decided 2026-07-11. Safety requirement (Şenol, same day):
  both skills carry mutually pointing, mutually exclusive descriptions -
  each names the other for the out-of-scope case - so skill selection
  never rests on inference alone.
- **Artifact signing: firm 1.x, owner ruling 2026-07-29.** Previously
  trigger-gated on a first external complaint about unsigned-install hurdles;
  that trigger stays as an accelerator but no longer decides whether the work
  happens. The occasion was his own QA pass hitting Fedora's
  `skipped OpenPGP checks` warning on the unsigned rpm, and the same pass
  confirming the ad-hoc-signed macOS bundle needs its Gatekeeper detour. Scope
  when it runs: per-OS, and the plan-8 record's recommendation to evaluate
  GitHub artifact attestations FIRST still stands. Pre-1.0 the warnings are
  documented rather than removed.
- **macOS on Intel: serve x64 users (owner call 2026-07-27, 1.x)**. The
  1.0 matrix is Apple-Silicon only (D78, ruled at the plan-8 kickoff). The
  owner wants Intel Macs served - not gated on someone asking, which is how
  it was recorded until now, but as a 1.x commitment. Two routes, and which
  one is buildable decides:
  - **A second x64 dmg leg** on the `macos-15-intel` runner label (verified
    to exist in the plan-8 design's runner survey). Simplest: one more
    matrix leg, one more artifact, each binary native to its arch. Costs a
    ninth release artifact and a row in every doc table that enumerates the
    set.
  - **A universal binary** (`universal-apple-darwin`): one dmg for both
    arches, no wrong-download mistakes. **Reopens D78**, which rejected it
    at arm64-only scale for doubling binary size for every arm64 user and
    needing both toolchains. The wrinkle to check first: the CLI sidecar
    ships via `externalBin`, whose file naming is target-triple-keyed, so a
    universal build needs both sidecar binaries staged - that, not the lipo
    step, is where this route gets expensive.
  Decide at the 1.x planning round with an SI-3 parity read (what does
  mkvtoolnix ship for macOS today - one universal dmg or two?) and with the
  ad-hoc signing of Plan 8.5 already in place, since it applies to any new
  leg. Whichever route wins carries the D78 amendment as a task rather than
  diverging from it silently.

- **One joint proof that joblog persistence stays unconditional under
  `--json`** (Plan-9 T2 review harvest, 2026-07-28). The invariant (spec 6,
  D26) is covered by two halves that never meet: the tee itself by
  `run_batch_writes_job_log_files` in core, and `--json`'s suppression of
  human output by `crates/muxsmith-cli/tests/run_cli.rs:125`
  (`run_json_on_a_real_mux_...`), which does set `MUXSMITH_RUNS_ROOT` but
  asserts only the JSON document and the output file. The one CLI test that
  reads the runs root, `run_live.rs:398` inside
  `readme_passthrough_recipe_...`, asserts it for a `run` invocation that
  carries no `--json` - the `--json` in that test is on the preceding
  `dry-run` (controller-verified 2026-07-28 by reading both call sites, not
  relayed). Plan 9 did not weaken this; it is the plan that moved the tee
  across a crate boundary, which is why the gap is recorded now. Cost: one
  subprocess test on the existing `run_live.rs` harness - the same run with
  `--json` added, asserting the run directory and its `summary.json`.

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

- **GUI test harness for the run path - firm 1.x item (owner ruling
  2026-07-28, Plan 9 kickoff)**: not trigger-gated and not a candidate; it
  is committed for 1.x and cut out of Plan 9 because new test
  infrastructure is its own package. What is missing, measured by the Plan
  9 recon: no Vitest and no @vue/test-utils in the tree, no tauri::test /
  mock_builder, no src-tauri/tests directory, and start_run's orchestration
  body - acquire, blocking plan, Soft/Ready, commit, runner thread - has no
  test of its COMPOSITION, because everything in it that does not need an
  AppHandle has already been factored out and is tested piecewise (40
  #[test] in run.rs), while several of those tests simulate what start_run
  does rather than invoke it. The frontend side is the same gap from the
  other end: the only run-path e2e mocks start_run outright and drives job
  events from the Playwright side, so the Rust body never executes there
  either. Two routes for the 1.x round to decide: widen the existing
  Playwright mount harness (its glob reaches only the editor widgets and
  EditorView today, and it installs no IPC mock) versus introduce a real
  component/integration harness (Vitest, and tauri::test's mock_builder for
  the AppHandle problem - whether it works on the pinned Tauri version and
  what it costs was NOT established and is the first thing to check).
  Neither question has a ledger entry. At 1.0 only the mount-glob widening
  for JobsView lands, under Plan 9's D23 item.

- **IpcError render funnel (candidate, trigger-gated)**: eight frontend
  sites hand-render `$t(err.code, err.params)` into a per-file alert; a
  shared component or composable is the house one-funnel answer, and it
  would have made D61's sweep enumerable by construction. Cut from Plan 9
  by owner ruling 2026-07-28 as design work rather than a mechanical
  hoist, because one consumer is mixed: BatchView fills the same
  `ipcErrorParams` ref from a core Diagnostic (config_diagnostics[0]) as
  well as from an IpcError, and the two wire types differ deliberately
  (Diagnostic params stay Record<string, string>, IpcError params were
  widened to string | number by D61). A funnel typed strictly to IpcError
  has to move that path elsewhere. Trigger below: a ninth render site.
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
  **Rider, gated on the next ci.yml-touching change whichever it is** (this
  item is the expected carrier; an earlier ci.yml edit inherits the duty -
  the edit is the trigger): ci.yml's dated Plan-5.5 comment block cites a
  BUILDING.md section by a title that no longer exists, deleted when the
  cross-target lint rule became gate part 6 (commit bcb67f3). Nothing there
  is false, but the pointer is ungreppable. Exact replacement, so nobody
  re-derives it: `# legs, matching the cross-target clippy gate part
  (BUILDING.md, Rust gate part 6; cfg-gated items can differ per platform).`
  (Plan-8 whole-branch delta F1, 2026-07-27.)
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

## Test flakiness (owner call 2026-07-28: no flakiness at 1.x; this is the tracked instance)

- **A SECOND, DIFFERENT flake class, observed 2026-07-29 (session 28): the CI
  Windows leg's mkvmerge INSTALL step, not a test.** Run `30469546031` on commit
  `ad4746d` failed with
  `mkvmerge.exe not found at C:\Program Files\MKVToolNix\mkvmerge.exe after choco install`
  while the four other jobs passed. The commit was docs-only, and the runs on
  both neighbouring commits - `8b98b86` before it and `fac3b6c` after - were
  green on all five jobs, so nothing in the tree explains it. The same job's log
  carries npm registry read errors with retries in the same minutes, which is
  the ordinary signature of a network-flaky runner window. **Why it is worth
  tracking rather than shrugging off:** this step is what makes the project's
  "3-OS green" claim mean live-binary tests on three of three, so when it fails
  the guarantee silently degrades to two of three - and it fails LOUDLY today
  only because the step asserts the binary's path afterwards. First occurrence;
  if it recurs, the fix is a retry around the install plus a re-assert, not a
  weaker check. No action now, per the owner's 1.x flakiness call.

- **`dry_run_json_emits_a_document_when_the_language_query_fails` failed once
  under load, then passed four times** (Plan 8.5 pre-push gate, 2026-07-28).
  Observed: the ten-part gate's `cargo test --workspace` reported
  `mkvmerge_found: false` where the test requires `true` - i.e. `locate()`
  behaved as if the fake mkvmerge on the test's `PATH` were absent. Not
  reproducible since: green in isolation, and green in three subsequent
  `cargo test --workspace` runs (the last with exit 0, zero `test result:
  FAILED` lines, 39 suites ok, the FAILED pattern fire-controlled against the
  red log). No Rust file changed in Plan 8.5 - measured, zero `.rs`/`Cargo.*`
  paths in `f627105..HEAD` - and CI had run this suite green on the identical
  Rust hours earlier, so the plan did not introduce it.
  The test writes an executable shell script into a tempdir and immediately
  execs it through a `PATH` set to that dir alone
  (`support::fake_mkvmerge_that_fails_queries`). **No mechanism is claimed
  here** - a write-then-exec race under parallel load is the obvious suspect
  and is exactly the kind of guess this project does not record as a finding.
  Trigger: the next time this test, or any other fake-binary test using that
  helper, fails without a code change -> that second data point is worth a
  systematic-debugging pass; a single non-reproducible failure is not.
  **Owner call 2026-07-28: fix it, timing 1.x - flakiness is not accepted
  as a standing condition.** Candidate fix, named so the 1.x pass does not
  re-derive it: the helper writes the script directly at the path it then
  execs, which is the shape a write-then-exec race takes; writing to a
  temporary name, setting the mode, and `rename()`-ing it into place makes
  the exec'd path one that was never open for writing. That removes a known
  race CLASS - it is explicitly NOT a confirmed fix for this observation,
  whose cause remains unestablished, and the 1.x pass should say which of
  the two it is claiming. Cheap either way: three lines in one test-support
  helper, no product code.
