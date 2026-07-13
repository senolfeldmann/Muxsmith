# Roadmap

Living, forward-looking work tracker: commitments with their milestone, and
candidates for later. Items here are DISCUSSION ANCHORS, not execution
licenses - each is talked through with Şenol when its turn comes, unless it
already carries a settled decision reference. History lives in the process
journal; decisions live in the specs/memos; unbuilt product ideas with full
analysis live in IDEAS.md.

## Plan 6 (next initiative)

Scope fixed by D22 (plan-5 memo): profile editor (comment-preserving YAML
round-trip is the hard design question and the reason apply-suggestion
waited), help-mode sidebar (spec 8.3 full mechanics), one-click
apply-suggestion, packaging/release pipeline. Starts with brainstorming.

Named design input for that brainstorming (2026-07-11, sweep walkthrough
#23): decide schema-driven vs UI-model editor, and whether the JSON
schema ships as a user artifact (e.g. yaml-language-server autocomplete);
if either lands schema-side, add `schemars(schema_with)` overrides for
the keep/drop/clear keyword domains - FilenameCfg/ChaptersCfg/TitleCfg
schematize as `anyOf [object, string]`, the keywords live only in
validate.rs (Plan-1 final review minor #7, trigger "a GUI generating an
editor from the schema" now fires with Plan 6).

Further named inputs (2026-07-12, Plan 5.5 roll-up funnel): route
JobOutcome.errors codes through the diagnostics catalog so
worker-panicked renders on live surfaces (T4-i2); core logging facade
replacing the single eprintln in queue.rs (T4-i1); reject bare `raw:`
with empty property name at validate (T16-m1); live in-session locale
switch (T21.5-m1, bootstrap-once today); sort JSON config_diagnostics
errors-first for validate parity (T9-m-iv).

Further named inputs (2026-07-11, docs-tree sweep): the GUI test-harness
question as ONE block - no Vitest component harness, no tauri::test
integration harness, start_run's orchestration body untested including
the never-decided injectable-planner-seam interface question ("to raise
at merge time"; the merge gate passed without it) (S4/S5/S6). Re-check
the final fix wave's self-flagged deviation from D23's frontend contract
(reset gated on runActive instead of "reset after resolve Ok" - "worth a
second look", never taken) (S12). resolvedTrackLabel punctuation outside
Fluent (locale-formatting revisit, S16). The help-mode sidebar includes
the spec-10 help-id completeness guard (CI fail on help-ids without a
topic file - S17, previously unnamed in this anchor).

Further named inputs (2026-07-12, idiomacy review triage): hoist the
four-copy planning pipeline into a shared plan_pipeline() core fn - this IS
the injectable-planner-seam (S4/S5/S6), spec 5.5/7 parity-critical (~100
lines); hoist run_batch into muxsmith_core::executor (the CLI inlines what
src-tauri already factored); hoist runs-root resolution to core (D26
debug-only seam duplicated CLI vs src-tauri). Plus the deferred Fluent
message-attribute reorganization (widget facets as .attribute instead of
suffixed siblings; touches frontend $ta + check-i18n parity). plan_pipeline
consumes profile::validate::config_diagnostics (landed Plan 5.6 T11).

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
- Plan 6 starts -> consume the named design inputs in the Plan-6 anchor.
- First real-world report of unwanted empty outputs, or a request to
  fail batches on empty plans -> IDEAS #5.
- Next parity-audit round or output-plausibility work -> IDEAS #6.
- A second fixture needs a _comment-style source-of-truth note ->
  promote the T11 ad-hoc pattern to a written convention (BUILDING.md
  test section).
- Next push to origin -> verify all three CI legs green (Plan 5.6 T8
  rewrote the toolchain install step: rustup toolchain install +
  explicit component add per rustup #4216; on-runner proof deferred
  because pushes were blocked in the executing session).

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
  deliberately deferred out of Plan 5's CI work). Lands via Plan 6.
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
- **Mixed-language `allowed` param (pre-1.0 polish, whole-branch I2)**:
  core emits English prose via the `allowed` param at planner.rs:428/:841
  ("a valid ISO 639/BCP-47 language code") - in de mode
  invalid-property-value renders mixed German/English; not on the spec
  §8.4 exception list. Fix catalog-side (kind selector or dedicated
  language-domain message), bilingual. Small task before the tag
  (bilingual launch makes it user-visible).
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
  journal entry; the Plan-6-folded and deferred items below stay open):
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
  - FOLDED INTO PLAN 6 (share the run/plan-orchestration territory of the
    planner seam): the four-copy planning pipeline (~100 lines, spec 5.5/7
    parity-critical - a shared plan_pipeline() IS the never-decided
    injectable-planner-seam S4/S5/S6); run_batch hoist to
    muxsmith_core::executor; runs-root resolution hoist to core.
  - DEFERRED (tracked v1.x/Plan-6): the Fluent message-attribute
    reorganization (widget facets as .attribute vs suffixed siblings) -
    changes message IDs, needs coordinated frontend $ta + check-i18n parity
    work; distinct from the comment-level fix that ships now.
  - KEPT as deliberate scaffold: the ci.yml `v*` tag trigger (Plan 6
    packaging will consume it).
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
- **Worker-panic handling + mutex-poison hygiene (Plan-4 T3 minor)**: DONE 2026-07-12 (Plan 5.5 T4 + whole-branch killer-invoke fix; poison recovery centralized incl. AppState.active).

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
- **Cosmetic cleanup, one pass (sweep group K)**: dead `at` param
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
