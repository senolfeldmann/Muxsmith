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

## Pre-1.0 release gates

Must be resolved before the first tagged release; none blocks Plan 6 work.
The code gates from the 2026-07-11 sweep are covered by **Plan 5.5**
(docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md, authored
2026-07-11, awaiting Şenol's execution go). Tracked individually outside
Plan 5.5: README, guide/blogs, the whole-codebase idiomacy review, and the
D35 auto-prune implementation. (Resolved so far: CSP 2026-07-11 -> D34,
policy set and verified; log-pruning decided 2026-07-11 -> D35,
implementation entry below.)

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
    it is uttered; fold into the README on next touch): none open.
- **Guide + blog posts (process + product)**: 1.0 deliverables, written in
  fresh sessions from the process journal as primary source. Named input
  for the format interview (Şenol 2026-07-11): user-docs architecture -
  the README now carries the full CLI usage reference (tool is thin, no
  other user docs exist); decide at the interview whether the guide
  absorbs it, deepens it, or the reference splits into its own markdown
  "man page" if it outgrows the front page. Şenol has
  specific ideas about how he wants these - PROPOSALS welcome, but the
  format/scope/voice interview with him comes FIRST. Not to be produced
  unprompted. Source note (residue R3): the strategic rationale behind
  the journaling/blog split (decay-rate argument, fresh-reader-vs-
  Betriebsblindheit, disjoint audiences) was never persisted - it is
  reconstructable from the Plan-1 session transcript (2026-07-08, late
  morning) and worth mining when the format interview happens. The
  process-learnings distillate in Şenol's Nextcloud project folder is a
  second primary source.
- **Run-log auto-prune implementation (D35)**: decided 2026-07-11 (D35,
  pre-1.0 memo) - core auto-prunes run dirs older than 14 days, fixed, no
  configuration in v1 (configurability parked in IDEAS #7); parity match
  with mkvtoolnix defaults. Small core task with regression tests;
  vehicle open (Şenol's call at the Plan 5.5 go: ride the plan as an added
  wave-1 task, or standalone) - the milestone gate blocks on this entry
  either way.
- **mkvtoolnix version pin in CI**: currently floats with the distro
  (backlog note in ci.yml, Şenol 2026-07-10). Decided 2026-07-11: resolve
  in the same touch as the mac/win runner gate below (walkthrough #14).
- **mkvtoolnix on macOS/Windows CI runners (fired go-public trigger)**:
  ci.yml:36 still gates the mkvtoolnix install on `runner.os == 'Linux'`
  ("minute economy" while private, explicit trigger "go-public
  follow-up" - the repo went public 2026-07-10 and the trigger was
  missed). All gated integration tests self-skip on windows-2025/macos-15;
  the Windows kill-mapping fix (75c075f) and platform_candidates()
  win/mac paths have never run against a real binary in CI, so "3-OS
  green" currently means compile+unit+fakes on two of three legs. Şenol
  2026-07-11 (sweep walkthrough #14): FIRST task of the pre-1.0 hardening
  block - remove the guard, per-OS install steps (choco/brew), decide the
  version pin in the same touch, and VERIFY the gated tests actually run
  on all legs by comparing skip counts (otherwise silent skipping is
  traded for silent skipping).
- **`.gitattributes` (`* text=auto eol=lf`)**: promised for Plan 2's
  first commit (2026-07-08 CRLF risk assessment), never created. A public
  repo inherits every contributor's autocrlf config, and the insta
  snapshots decided in walkthrough #2 are byte-exact artifacts vulnerable
  to CRLF drift. Şenol 2026-07-11 (sweep walkthrough #15): add before
  1.0, early in the hardening block (before the snapshot work); mark
  binary test assets `-text`; renormalization (`git add --renormalize .`)
  as its own isolated commit.
- **Catalog param-drift guard**: catalog_completeness.rs renders every
  DiagCode message with EMPTY args, so emitter-vs-message param drift is
  structurally invisible - the class already reached a user once (literal
  `{$property}`, fixed as F9 without closing the guard gap; the guard fix
  was prescribed verbatim by the Plan-1 final review and its trigger
  fired unnoticed). Şenol 2026-07-11 (sweep walkthrough #16): before
  1.0 - per-code param fixtures asserting no unreplaced `{$...}` in
  rendered output, plus coverage for non-DiagCode keys (the 8 `run-*`
  keys in cli.ftl are currently entirely unguarded; docs-tree find S3).
- **German locale (de) before 1.0**: only locales/en/ exists; the Fluent
  infrastructure is locale-agnostic and ready. Şenol 2026-07-11 (sweep
  walkthrough #17): 1.0 ships bilingual. Work order: (1) convert the
  "error(s)"-style strings to Fluent plural selectors FIRST (cli.ftl:2,
  gui-batch.ftl:31 - their recorded trigger "first real second locale
  lands" fires with this decision), (2) extend check:i18n to enforce key
  parity across locales, (3) translate the six catalogs (agent draft,
  Şenol reviews terminology), (4) loader/scanner primary-subtag
  normalization - the mechanics half of the "locale #2 lands" trigger
  (docs-tree S15). Every new message from the hardening block onward is
  born bilingual.
- **Test-hardening rider (sweep group T)**: (i) donor-ordering golden for
  mixed `track_id: None/Some` assignments (behavior currently unpinned);
  (ii) identify parse-edge tests (wrong-typed id, non-numeric
  num_entries, absent properties key - documented at identify.rs:225,
  untested); (iii) Plan-4 test gaps: exit-1 output-kept assertion,
  fail-fast-with-non-first-failing-job queue test, dry_run_cli
  default-branch severity assertion; (iv) fix with-attachments.json to
  1-based attachment ids (real mkvmerge wire format; code id-agnostic
  today, fixture fidelity regardless). Şenol 2026-07-11 (walkthrough #21,
  split decision, group T).
- **rustdoc gate step + dead intra-doc link**: cargo doc warnings are
  invisible to the eight-part gate although the repo enforces
  #![deny(missing_docs)] - presence is gated, correctness is not
  (queue.rs:73 links private [worker_count], rotting since Plan 4). Şenol
  2026-07-11 (sweep walkthrough #18b): fix the link in the hardening
  block AND add `cargo doc --no-deps` with RUSTDOCFLAGS="-D warnings" as
  the ninth gate part (locally and in CI).
- **Packaging pipeline**: msi/dmg/deb/rpm/AppImage on release tags (spec 10;
  deliberately deferred out of Plan 5's CI work). Lands via Plan 6.
- **Spec-§10 test mandates: proptest + insta**: both spec-mandated since
  the root spec commit, deferred out of Plans 2/3, dropped by the D18
  condensation, never implemented. Şenol 2026-07-10 (sweep walkthrough
  #1+#2): implement before 1.0 as one correctness-hardening SDD block.
  proptest scope: match-algebra laws, language-normalization idempotence,
  planner determinism / rendered-name invariants, D6 "suggestion survives
  next dry-run" property. insta scope: CLI human-output snapshots with
  path/version redactions, replacing the wording-coupled asserts the
  Plan-1 final review flagged as "don't grow the pattern"
  (cli_validate.rs:18,52 and successors); strict compare in CI.
- **`--list-types` extension validation (spec §4.2)**: capability layer
  built in Plan 2 (`list_types()`), never wired to a consumer - profile
  `extensions` are accepted unchecked (typo = silent file exclusion), and
  model.rs:73-75 doc-claims the validation exists. Şenol 2026-07-11 (sweep
  walkthrough #3): wire it before 1.0 next to the language-validation walk
  (new diagnostic for unknown extensions; degrade-with-warning when
  mkvmerge is absent, consistent with the existing pattern) and make the
  model.rs doc comment true.
- **UnknownPropertySkew forward-compat path (spec §9.2)**: the spec
  promises unknown newer-mkvmerge properties become matchable as untyped
  values with an UnknownPropertySkew warning, but validate.rs:345
  hard-rejects unknown property names config-time - the promised path is
  unreachable (flagged by the Plan-2 review as "worth spec'ing", then
  lost). Şenol 2026-07-11 (sweep walkthrough #4): implement as spec'd
  before 1.0. Opens with a design round (brainstorming): how to keep
  typo protection while allowing skew matching (did-you-mean suggestion
  vs explicit opt-in syntax vs warning-only).
- **Suggestion-engine D6 completion (spec §5.3 + D6 remainders)**: three
  fragments, ONE engine block before 1.0 (Şenol 2026-07-11, sweep
  walkthrough #5 + #12). (i) No-single-fix partition report (spec §5.3):
  when no single suggestion survives the whole batch, list the files
  grouped by the resolution each would need - flagged-deferred into the
  HANDOFF chain and lost, sole trace planner.rs:985. (ii) Suggestions for
  external-source rules: planner.rs:1014 skips them ("external deferred"),
  an unexplained asymmetry vs primary rules; symmetry completion with the
  existing algorithm. (iii) OverlappingRules auto-suggestions: algorithm
  not yet designed (D6 never specified what a good overlap-narrowing
  proposal is) - opens with its own design round / D-decision inside the
  block. Plus two latent D6 gaps from the Plan-2 review archive (residue
  round R1, 2026-07-11): (iv) candidate generation ignores top-level
  `codec`/`id` as narrowing dimensions; (v) `diag_signature` uses a
  BTreeSet where a multiset is meant (duplicate diagnostics collapse).
- **Diagnostics polish block (review minors, Plans 1-4)**: five
  loud-but-suboptimal presentation defects, one pass before 1.0 (Şenol
  2026-07-11, sweep walkthrough #13): (i) OverlappingRules names only 2
  of >=3 claimants (planner.rs:526-533; implement inside the D6 engine
  block - overlap suggestions need all claimants anyway); (ii)
  `any: []` double-reports EmptyMatchExpression + EmptyMatchList
  (validate.rs:65+301); (iii) human mode prints the filename twice per
  diagnostic (commands/mod.rs:46ff); (iv) dry-run/run human output not
  severity-sorted while validate sorts errors-first; (v) lint `0` vs
  planner `tracks[0]` rule-reference formatting; (vi) donor-side
  UnsupportedSource gate (D21 remainder: identifiable-but-unmuxable donor
  falls back to UnidentifiableSource / per-rule noise, planner.rs ~:431,
  wrong remediation for the user - walkthrough #11, revised 2026-07-11
  into this block); (vii) mkvmerge-query-failed human-mode path still
  drops config diagnostics while JSON emits them (residue R1 - pre-check
  whether the asymmetry is deliberate, then fix or document); (viii)
  IdentifyError writes English into `detail`, a prose-free-core violation
  accepted by review but never documented (residue R1 - fix or record as
  a spec-8.4-family exception).
- **Zero-track plan warning**: a plan resolving to zero output tracks
  muxes a valid-but-empty MKV, exit 0, no diagnostic (verified live in
  the Plan-3 whole-branch review; deferred via D18's cleanup-pass list,
  tracker-orphaned). Şenol 2026-07-11 (sweep walkthrough #6): implement
  before 1.0 as a per-file WARNING plus batch-report visibility. Decided:
  warning only, one sane default, no error/skip options (option
  proliferation rejected; skip/error-as-option parked in IDEAS #5).
- **SourceOverwrite completeness (Plan-2 FINAL minor M2)**:
  `detect_source_overwrites` (planner.rs:893) gathers protected source
  paths only from files whose plan rendered; a donor referenced solely by
  a render-failed file drops out of the protection set, so a colliding
  output overwrites a source silently (code-verified still open). Şenol
  2026-07-11 (sweep walkthrough #7): fix before 1.0 - feed the protection
  set from ALL files including render-failed ones, plus a regression test
  for the three-way constellation. Only audit finding with data-loss
  potential. While touching detect_source_overwrites: add the guard
  comment for the ambiguous-external reconsider-trigger (F5 report - that
  branch deliberately contributes no donor paths while it is fatal;
  revisit if it ever becomes non-fatal; docs-tree S11).
- **Empty-batch human output (D15 gap)**: human mode prints nothing on a
  zero-file batch (exit 0, silent success) while JSON prints a zeroed
  summary (run.rs:168-179; ledgered 2026-07-09 as "Şenol decision for
  v1.x", never actually put to him). Şenol 2026-07-11 (sweep walkthrough
  #8): fix before 1.0 - ALWAYS print the batch summary line in human
  mode, so the empty case reads "0 files matched (searched <root>,
  extensions ...)"; info level, exit code unchanged. Recorded divergence
  from mkvtoolnix: the interactive GUI shows "nothing to do" by
  construction, a batch tool must say it.
- **Robust event-stream reads (Plan-4 T1 minor, severity upgraded)**:
  spawn.rs:104 treats a read_line Err (non-UTF-8 line, e.g. a
  broken-encoding filename quoted in a mkvmerge warning) as EOF. Verified
  escalation (2026-07-11): LiveJob keeps the stdout pipe open during
  wait(), so once the consumer stops reading, a child with >pipe-buffer
  pending output blocks on write and wait() never returns - one bad line
  can hang a worker indefinitely mid-batch (short remainders: silent
  log truncation only). Şenol 2026-07-11 (sweep walkthrough #9): fix
  before 1.0 - read_until(b'\n') + from_utf8_lossy, keep consuming after
  a decode-degraded line; TWO regression tests: truncation and no-hang
  (the hang path additionally assumes run_job's post-None wait(),
  spawn.rs-verified, job.rs loop to be pinned by the test).
- **Whole-codebase idiomacy review**: one large, deliberately costly review
  pass before the 1.0 tag - Rust workspace, TS/Vue frontend, configs -
  against ecosystem idiom. Dimensions: unidiomatic constructs;
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
- **Worker-panic handling + mutex-poison hygiene (Plan-4 T3 minor)**: the
  queue swallows worker panics (`let _ = handle.join()`, queue.rs:270) -
  a panicked worker's job is backfilled as Cancelled (wrong label), and a
  poisoned killers/outcomes mutex later panics the whole process via
  into_inner().unwrap() (inconsistent failure mode). Şenol 2026-07-11
  (sweep walkthrough #10): fix before 1.0 - check join() results, record
  a panic as Failed with a distinct "worker panicked (internal error)"
  outcome, poison-recovery instead of unwrap. Bundled with the
  AppState.active poison-recovery item promoted out of the v1.x list
  (same hygiene class, both spots in one pass).

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
