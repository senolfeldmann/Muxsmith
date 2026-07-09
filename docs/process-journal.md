# Muxsmith process journal

Raw material for a future process-focused writeup: decisions with their why,
what the review process caught, mechanics, friction. Append-only; entries are
lab-notebook register, not publication prose. See the entry prompt for rules.

## 2026-07-08 | Spec + Plan 1 complete, repo live | session 1

**Scope.** Entire first session: requirements interview, spec, Plan 1
authoring, subagent-driven execution of all 13 tasks, rustdoc backfill,
GitHub setup, CI trim. Commits 61249f9..97ae031 (33 commits). First-run
salvage pass done: all 49 files of `.superpowers/sdd/` survived a reboot
(repo dir, not /tmp) and are archived under
`docs/process-journal/artifacts/plan-1-sdd/`; nothing lost.

**Decisions and why (not obvious from artifacts).**
- Strict independent uniqueness over ordered consumption or a global solver:
  Şenol chose maximal explicitness knowing configs need explicit exclusions
  (forced_track: false); compensated by making error quality and the
  suggestion engine first-class. Ordered consumption would have made his own
  example work unmodified; he rejected the implicitness.
- Suggestion engine contract (his amendment): suggestions are simulated
  against the whole batch before being shown; an applied suggestion must
  survive the next dry run. Turned a hint feature into a verified-edit feature.
- Full MKV-structure control in v1 (attachments/chapters/tags/title) against
  agent recommendation of global toggles; his scope call.
- Stack: Tauri 2 + Rust core + React/TS over Wails v3 (alpha risk vs his Go
  learning goal) and Avalonia (delivery certainty, smaller OSS pull). Rust
  accepted although not on his learning list. MIT over Apache-2.0.
- mkvmerge identification schema: build-time fact extraction via xtask with
  committed generated.rs; schema never vendored (licensing sidestep), no
  build.rs network dependency. Runtime skew handled as untyped-match warning.
- i18n: Fluent chosen because it is the one system with first-class Rust AND
  JS implementations, so one catalog serves CLI and future GUI. Core emits
  code+params only; this rule later forced a real fix (see below).
- Docs: deny(missing_docs) for public API; private items documented by
  judgment only (blanket private-doc lint rejected as comment-noise).
- CI while private: dynamic fromJSON matrix, Linux on branch pushes, 3-OS on
  PR/tags/dispatch (macOS bills 10x); revert on going public.

**What the process caught.** Real defects, all origin=plan-authored code:
- serde deny_unknown_fields silently ineffective on untagged inline struct
  variants (task review T4) -> TemplateBlock/ExternalBlock newtype fix.
- `gen` reserved in edition 2024 (implementer T5); implementer's
  edition-2021 downgrade workaround rejected by controller -> module renamed.
- key()/serde kebab encodings unlinked (task review T2) -> DiagCode::ALL +
  exhaustive consistency tests.
- --json output unsorted while text sorted (task review T12).
- Template-error params carried English prose out of core, violating the
  plan's own exit criterion (final review; spec-wins rule applied).
- match_to_source: false raised a spurious LocatorConflict (final review).
- UnknownProperty name collision between spec table and code (final review)
  -> spec amended, code kept.
Evidence-integrity finds: one synthesized test transcript (T3 review);
haiku implementers mis-totaled workspace test counts in 5 of 13 reports
(code correct each time) -> controller began independently re-running every
suite; that verification never contradicted a green claim, only the counts.

**Mechanics and metrics (approx where labeled).**
- ~31 subagent dispatches: 13 implementers, 13 task reviews, ~7 fix/re-review
  waves, 1 docs pass, 1 final whole-branch review + confirmation.
- Model split: haiku for transcription implementers (plan contained complete
  code), sonnet for judgment implementers (T9, T12, fix wave, docs) and all
  task reviewers, fable for the final review. Subagent token use 37k-210k
  each; largest was the docs backfill (~209k).
- Tests 0 -> 81; fmt + clippy -D warnings clean throughout; first CI run
  green on all 3 OSes in 2m44s; trimmed run verified as exactly one job.
- Rustdoc: 151 public items backfilled in one pass + per-variant DiagCode
  docs via macro meta-forwarding; 5 restatement-style docs caught by review.

**Friction and failure.**
- Monitor script used zsh read-only variable `status`; watch died silently.
- gh initially missing and repo private -> first CI verdict unobservable
  until Şenol installed gh; anonymous API returned Not Found.
- GPG signing blocks agent commits; standing workaround
  `-c commit.gpgsign=false`.
- Session limit interrupted one turn mid-bookkeeping; ledger + git log made
  resume trivial (the ledger earned its existence here).
- Mid-session rule change (no unrequested commits) later explicitly reversed
  for this repo; gh-log.md audit convention added the same day.

**Moments.**
- T2 reviewer suspected supply-chain anomaly in Cargo.lock (`zmij`), checked
  crates.io, found it is dtolnay's renamed `ryu`; suspicion correctly
  withdrawn (artifacts: review in plan-1-sdd).
- T5 implementer independently discovered the edition-2024 keyword collision
  the plan missed; wrong fix, right diagnosis.
- Final reviewer graded the plan's own code against the plan's constraints
  and failed it on three counts; "the plan does not grade its own work"
  proved to be the load-bearing process rule.

**Deltas.** yaml-serde crate name resolved to yaml_serde 0.10.4 (plan
anticipated the fallback); T13 catalog guard upgraded from the plan's
hand-copied key list to DiagCode::ALL iteration; audio_sampling_frequency
type assumption in a plan comment was moot by test construction.

**Open threads.** Plan 2 handoff notes (6 items, see
artifacts/plan-1-sdd/progress.md); deferred minors: Fluent plurals, schema
keyword domains for the GUI, substring-test hardening, xtask dedup
tie-break, unused `at` param in load.rs, invalid-template selector default
variant. Pending decisions: go-public timing + CI matrix revert; Dependabot
cadence if enabled. Plan 4 installs parked: Tauri system deps, pnpm via
corepack. Plan 2 awaits explicit go; Şenol announced additional context
before that go.

**Addendum, same day.** The salvage pass itself nearly failed twice: the
archive commit initially contained only the journal because the copied sdd
directory carried the tooling's own `.gitignore` (a bare `*`), silently
excluding all 49 artifacts; caught by reading the commit stat line, fixed in
411087f. Lesson recorded: after any archive/copy commit, verify the file
count in the commit, not the working tree.

## 2026-07-09 | Plan 2 written and implemented | session (Peter, Opus 4.8)

**Scope.** Plan 2 design finalization, plan authoring, and full implementation.
Commits `3b71a71..e1bfba7`. Design memo + spec fold-in, the 12-task plan doc,
then all 12 tasks executed. 125 workspace tests green; CI green at e1bfba7
(`test` + new `deny`).

**Decisions and why.**
- Session opened under a Fable quota crunch; Şenol asked what was worth the last
  Fable tokens. Decided: spend model strength on the DECISIONS (D1-D6) not the
  transcription. Wrote the design memo first, then folded D1-D5 into the
  authoritative spec so "spec wins on conflict" could not silently override them
  (4.3+4.4 previously implied codec_kind under substring; a bad enum value
  degraded to MissingTrack). Then quota moved to Opus and the rest proceeded.
- D2 dropped an xtask codegen path: the v20 identification schema types `type`
  as a plain string (no enum) and only `aac_is_sbr` carries a schema enum, so
  generating value domains for one irrelevant field was an abstraction the scale
  had not earned. `type`/`codec_kind` domains are curated in `capability`
  instead. Grounded in the source Şenol dropped at ~/Downloads/mkvtoolnix.
- Execution deviated from Plan 1's subagent-driven-development: the controller
  (Opus) executed the tasks inline with live `cargo` verification, rather than
  dispatching a fresh subagent per task. Rationale: the plan's code was fully
  specified and every task was locally compile/test-verifiable, so the SDD
  apparatus (implementer + reviewer subagents) was overhead for mechanical
  transcription. Tradeoff: no independent per-task reviewer; a whole-branch
  adversarial review is still owed (open thread).
- mkvmerge `-J` shape, `--list-languages`/`--list-types` formats, and the track
  `type` domain were confirmed empirically by running the installed v99 binary
  rather than trusting memory or the schema alone. Both the identify parser and
  the runtime parsers passed against real output first try.

**What the process caught.**
- CI (test job) caught fmt-dirty commits: tasks 3 and 5 were pushed after
  running clippy but not `cargo fmt --check`; the intermediate run at 0e64c1e
  failed only on `cargo fmt --all --check`. Fixed by rustfmt commit 72c59d2.
  Origin: controller discipline gap, not the plan.
- clippy caught two collapsible-if blocks in the matcher (let-chains), pre-push;
  fixed by amend. Origin: implementer (plan code was pre-let-chain style).
- cargo-deny caught the intra-workspace path dependency as a wildcard; resolved
  by marking the workspace crates `publish = false` (Muxsmith ships as app
  bundles, not crates.io packages), which is also correct on its own terms.
  Also trimmed the license allow-list from a speculative 8 to the 3 actually
  used (MIT, Apache-2.0, Unicode-3.0).

**Process mechanics.** 12 tasks, 0 subagent dispatches (controller-inline
execution). Model: Opus 4.8 (1M) throughout after the Fable handoff. One clippy
amend, one rustfmt catch-up commit, one style commit. ~14 commits in the range.
Every push logged in gh-log.md. Live end-to-end dry-run demonstrated the
suggestion engine emitting three forced_track discriminators for an ambiguous
English-SRT rule.

**Deltas.** Task 4 shrank from an xtask-codegen task to a curated-domains task
after checking the real schema (above). Plan's `candidates_for_rule` iterator
chain had a temporary-borrow bug flagged in the plan itself; implemented via the
owned-Vec workaround the plan noted. Plan's Task 10/11 split (extract plan_core)
was collapsed: planner written with plan_core/plan_batch in final shape from the
start, suggestion engine added as the placeholder->real step.

**Open threads.** Whole-branch adversarial review still owed (executed inline,
no independent reviewer). Deferred: OverlappingRules auto-suggestions and the
no-single-fix partition report (D6 remainder); Plan 3 (attachments/chapters/
tags/title, command generation, executor); `--list-types` extension validation
(no diag code yet); CI does not install mkvtoolnix so the gated tests self-skip
there. See HANDOFF.md.

## 2026-07-09 | Plan 2 fix pass (SDD, corrective) | session (Peter, Opus 4.8)

**Scope.** Corrective pass after Plan 2 was executed inline (no independent review) and then given a retrofit review that found ~11 bugs + 3 design questions. Commits `847b476..59d24c8`. Executed via subagent-driven-development this time: fresh implementer subagent per task, independent reviewer subagent for the substantive tasks, fix waves, final whole-branch review on opus. Artifacts archived at docs/process-journal/artifacts/plan-2-fixes-sdd/ (the per-task trail inline Plan 2 never produced).

**Decisions (Şenol) folded into the spec.** #1 absent boolean matchable property compares false for exact (mirror mkvmerge, 4.4); #2 empty any/not is a config error EmptyMatchList (4.3); #3 two-planned output collision always errors, on_collision governs on-disk only (4.8). Plus 5.5 made explicit dry-run is a strict superset of validate.

**What the process caught (the evidence).** Independent review found real defects at 5 of the review gates, none caught by the implementer's own tests:
- F1 (dry-run/validate): reviewer FAILED spec on the mkvmerge-not-found path silently dropping config diagnostics - the implementer had explicitly waved it off as a judgment call. Fixed; the fixer then found the branch WAS testable (PATH override).
- F5 (SourceOverwrite): reviewer found Critical - donor paths scoped per-primary, not batch-wide (my own dispatch said batch-wide; implementer narrowed it). One primary's output could overwrite another's donor. Fixed via a batch-wide post-pass.
- F6 (output/collision): reviewer found the keep-name .mkv handling regressed when the two arms were unified, plus the valid-append path was untested. Fixed.
- F8 (symlinks): reviewer found the broken-symlink skip path untested (code correct, coverage gap). Tests added.
- FINAL whole-branch review (opus) caught what EVERY per-task review missed: a template rendering to literal `.mkv` yields a hidden empty-stem `.mkv` output at exit 0 (F6 checked the pre-append value, not the final stem). This is the whole-branch stage earning its place. Fixed (59d24c8).
- F4, F7 passed per-task review clean. F2, F3, F9 controller-verified (mechanical), covered by the final review. 6 Minor items from the final review recorded in the archived FINAL-review.md for a follow-up.

**Process mechanics.** 9 planned tasks (F1-F9) + 1 final-review fix. ~13 implementer/fixer dispatches, 6 reviewer dispatches (F4/F5/F6/F7/F8 per-task + 1 final). Models: sonnet for implementers and per-task reviewers, opus for the final whole-branch review. Fix waves on F1, F5, F6, F8, and the final catch. Controller verified every task's suite itself (never trusted report arithmetic). 164 tests green at close, fmt/clippy/deny clean, CI green (test + deny) at 59d24c8. One transient: the F2 implementer died on a 500 mid-commit; its edits were complete, controller finished the commit.

**Contrast with inline Plan 2.** Same author-quality implementers, but the independent reviewer/controller separation turned "125 tests green, shipped ~11 bugs" into caught-before-merge. This is the concrete before/after for the multi-stage-review claim.

**Deltas.** F2 (new codes) done inline-by-controller-commit after the subagent's 500. F4/F7 needed no fix wave. F7 added a third diag code (SuggestionsCapped) to log the cap non-silently - a small scope growth the task invited.

**Open threads.** 6 Minor final-review items (see archived FINAL-review.md); the mkvmerge-query-failed path still drops config diags (same class as the F1 fix, logged in the ledger); nits from the original review (OverlappingRules >=3 claimants, lint-vs-planner rule-ref formatting, regex recompiled per call, proptest coverage). Plan 3 (attachments/chapters/tags/title, command generation, executor, run) is next - execute via SDD per the HANDOFF standing instruction.

## 2026-07-09 | Plan 3 complete (pure layer: resolution + command) | session 3

**Scope.** Plan 3 impl `62d4956..7d46547` (14 commits); design docs `497502e` (D7-D11), `d039e24` (plan), `62d4956` (D12). First Muxsmith plan executed fully via superpowers subagent-driven-development (SI-1), in contrast to Plan 2's inline execution.

**Decisions and why.**
- Plan 3/4/5 split (D7): pure layer (resolution + command, golden-testable) banked and reviewed before the process layer (executor/run, Plan 4). Şenol chose split over one combined plan; the executor is the riskiest, least-testable part and earns a clean review boundary. GUI slid to Plan 5.
- `add` cardinality (D12): decided via a two-round debate. Şenol pushed back on the font special-case ("everything the same, no special cases"). Resolved by reframing the real invariant as slot-vs-collection, not tracks-vs-attachments: track/chapters donors fill a unique slot (uniqueness-constrained); attachment select/drop/add populate a collection (already multi in the shipped model). So all-matched makes `add` consistent with its siblings; exactly-one would have been the special case. Şenol ratified collection-populator (all matched + zero-match warning + dedup).
- Settable-language validated per-file at the application point, not folded into the batch-level `validate_language_values` walk (Task 5). Deliberate: simpler, catches it where applied; leaves an invalid language on an unmatched optional rule uncaught but inert. Recorded for review, accepted for v1.
- `command` kept a pure `Plan -> Vec<String>` (D8): the enriched `Plan` carries resolved changes/attachments/chapters/tags/title so command never sees the Profile; mkvmerge CLI knowledge stays in one module.

**What the process caught.**
- Task 9 `input_groups` included track-less donor sources as empty input groups (Important). Caught by task review (named-check dispatched after the implementer flagged the ambiguity in DONE report); would have put an empty `( donor )` group into real mkvmerge commands and broken Task 12's round trip. Origin: plan ambiguity (canonical reference did not spell out the primary-only carve-out). Fixed d55f19d + regression test.
- Task 2 `&&Track` unification (implementer-caught during impl): planner's `.iter().filter(|t| ...)` hands `&&Track`, so `M` unified to `&Track`; solved with a blanket `impl<M: Matchable> Matchable for &M`, no planner change. The brief's stated regression-guard ("resolves via type inference") was factually wrong.
- Whole-branch review (opus) drove real mkvmerge v100 beyond the goldens: confirmed the `-J` attachment `id` equals the `--attachments` selector id (the one silent-wrong-file risk), all 26 flag spellings, rich argv exit 0. Also found a comment stating a FALSE reason (mkvmerge rejects empty groups - it does not; the exclusion is still right). Real bug: none survived. Noise separated: several "minors" were pre-existing test conventions.
- Task 3 reviewer flagged a trait DOC-COMMENT edit under "no trait changes"; controller adjudicated accept (a truthfulness fix, not a contract change).

**Mechanics/metrics.** 12 tasks. Dispatches: 12 implementers + 1 fix (T9) + 1 final-minor-fix = 14 build; 12 task reviews + 1 T9 re-review + 1 whole-branch = 14 reviews. Models: sonnet for all implementers and task reviewers, opus for the whole-branch review. Fix waves: 1 (T9 Important) + 1 (3 final-review minors). Tests 164 -> 204. Controller re-ran the gate after every task (SI-1: never trust the report's arithmetic); all green each time. No CI runs during the loop (commits local; single push at completion).

**Friction/failure.**
- `.superpowers/sdd/` still held Plan-1/2 reports at the same task-N-report.md paths; every implementer overwrote a stale same-named report and noted it. The salvage pass had to select Plan 3 files by name and drop a 2-byte `.gitignore` (the same ignore-file trap that nearly lost Plan 1's artifacts).
- `scripts/task-brief` extracts only the per-task section; Plan 3's shared reference blocks (enriched-Plan types, canonical argv contract) live above Task 1, so briefs for Tasks 4 and 9-12 had to have the reference blocks manually appended before dispatch. A per-task brief is not self-contained when the plan front-loads shared references.

**Moments.**
- The D12 slot-vs-collection reframe (chat, pre-impl): "you convinced me and I convinced you back."
- Task 9 empty-donor-group: implementer flagged it as a DONE concern rather than shipping silently; the reviewer then confirmed it Important with the primary-carve-out argument. The SI-1 process working as designed.
- opus final reviewer re-running mkvmerge to check attachment-id identity rather than trusting the golden string.

**Deltas.** Task 9 (single-group) vs Task 10 (multi-group) split blurred: Task 9 already built general group iteration, so Task 10 was mostly per-track props + multi-group golden coverage. The plan's incremental-golden design worked: each command task extended the argv and updated the prior task's golden (Task 11 added donor `--no-attachments` to Task 10's golden), verified not a regression.

**Open threads (Plan 4 inherits).** Deferred minors: richer gated live test (attachment + changes) - highest value; zero-track-plan renders an empty MKV with no diagnostic (planner empty-plan warning?); FakeIdent+lang() duplicated 3x -> tests/support.rs; tests `std::mem::forget(tempdir)` leaks; with-attachments.json uses 0-based ids (mkvmerge is 1-based; code id-agnostic); optional batch-level settable-language check. Next: Plan 4 = executor + run subcommand + FIFO queue + SIGINT cleanup; job-log persistence deferred to Plan 5 (GUI). mkvtoolnix still not installed in CI (gated integration tests self-skip there).

## 2026-07-09 | Plan 3.5 complete (mkvtoolnix parity fixes) | session 4 (Peter; Fable 5 -> Opus 4.8 1M mid-session)

**Scope.** Plan 3.5 (D19-D21), commits 91b19eb..aa75025 (8). Inserted BEFORE Plan 4: three parity fixes to the pure layer, surfaced by a from-scratch mkvtoolnix parity audit. 7 SDD tasks + 1 review-fix.

**Decisions and why.**
- Plan 3.5 exists because Şenol asked for a full mkvtoolnix parity audit before Plan 4 (2 parallel recon subagents: Muxsmith decision inventory + mkvtoolnix-gui source). Framing rule, now SI-3: mkvtoolnix is INTERACTIVE (pre-fills guesses the user reviews), Muxsmith DECLARATIVE BATCH (no per-file review) -> only muxing semantics/output are parity targets; input-time convenience guesses (filename-derived lang/flags, auto-title, unique-name suffix, sequence append) are not, shelved to docs/IDEAS.md.
- D20 tracks bare-list -> `{ unmatched, rules }` block. Şenol drove placement: nested block (not a top-level `unmatched_tracks` key) because the profile already keeps policies in their section (output.on_collision, tags.global). I'd recommended top-level for the bare-list ergonomic; conceded on whole-profile consistency.
- D20 `keep` kept despite Şenol questioning the need. One real case: additive bulk ops ("add a sub to a library, keep the rest") are inexpressible under drop (one-track-per-rule forbids a catch-all).
- D19 matcher (canonicalize) was nearly cut as a no-op - the raw fallback already matched equal tags case-insensitively. Trace showed the cited fragility (pt-Latn-BR vs pt-BR) needs canonicalize(), which language-tags bundles (cheap). Şenol chose to do it now (option B). Surfaced "exact = typed value-equality, not string equality" -> spec 4.3, README-flagged.
- D20 keep TRACK ORDER reversed at close-out. Memo assumption #3 (matched-first, unmatched-appended) made a donor-only keep profile put the added sub FIRST. Whole-branch review flagged it; Şenol called B (donors trail): "keep = match what's already there" makes kept-unmatched primary tracks count as matched, so --track-order lists them (invariant holds), primary first. Task 7 built it.

**What the process caught.**
- Whole-branch (opus) flagged donor+keep ordering as a usability trap on the marquee use case (verified vs mkvmerge v100). Correct-per-assumption, not a code bug, but drove the D20-B reversal + Task 7. Origin: the design assumption.
- Task 4: implementer found `zz`/`zzz` invalid-language fixtures are WELL-FORMED BCP-47, so the widened predicate would silently accept them -> those tests stop testing. Changed to `notalanguage` (len>8 fails well-formedness). Reviewer re-probed the crate to confirm. Origin: plan.
- Task 5: implementer corrected a brief doc-comment (`xx-YY` canonicalizes to itself, not None; crate errors only on MultipleExtendedLanguageSubtags). Reviewer cross-checked crate source (Suppress-Script table). Origin: my brief.
- Task 7 review (Important): keep+donor track-order had only the gated live test (skips w/o mkvmerge -> unguarded in CI). Fix aa75025 = deterministic unit test; implementer verified it as a real guard by scratch-reverting the branch two ways. Origin: implementer.
- D21 gate = `!container_recognized || !container_supported`, NOT is_identifiable() (recognized zero-track stays MissingTrack); confirmatory test asserts both directions.

**Mechanics.** 7 tasks + 1 fix. ~3 recon + 7 implementer + 1 fixer dispatches; 7 task reviews + 1 whole-branch. Models: sonnet implementers/task-reviewers, opus whole-branch. 1 fix wave. Tests 204 -> ~214, 0 failed each gate; controller re-ran full gate (test/fmt/clippy -D warnings/deny) after every task. New dep language-tags 0.3.2 (MIT/Apache-2.0, no transitive deps, deny.toml untouched). Single push at completion.

**Friction/failure.**
- STRICTLY SERIAL execution. After Task 1 the D19/D20/D21 streams were independent (disjoint planner.rs regions), parallelizable in worktrees. Şenol: "I am waiting for something that could have been faster." SI-1 rewritten (Superpowers-throughout + parallelize-independent) + cross-project memory feedback_superpowers_throughout. The clearest miss of the session.
- Permission classifier BLOCKED the first planning-docs commit (global never-commit rule, blind to the repo authorization); cleared by an explicit in-session commit + later a session-wide override.
- Same stale-report trap as Plan 3: task-6-report.md held an earlier plan's content; overwritten and noted.

**Moments.**
- "exact already means value-equality" - Şenol asked why exact matches non-identical strings; the typed-value-equality answer (6==6.0, de==ger already) he called README-worthy. -> spec 4.3.
- D20-B "keep = match what's already there" collapsed a two-zone ordering model to one rule and dissolved the invariant-break objection.
- Tasks 3 and 7 implementers both hand-ran mkvmerge before encoding the order assertion, unprompted (SI-3 holding).

**Deltas.** Plan 3.5 itself is a delta (did not exist until the audit inserted it). Keep track-order was specced (assumption #3), built (Tasks 2-3), reviewed green, THEN reversed (D20-B, Task 7) - the plan's own assumption changed, not its implementation.

**Open threads (Plan 4 inherits).** Plan 4 memo (D13-D18) written, parked, UNREVIEWED by Şenol - review first. Standing Plan 1-3 minors still open (FakeIdent/lang() 3x dup, mem::forget tempdir leaks, mkvtoolnix in CI). docs/IDEAS.md holds 4 shelved parity features. Parallelize Plan 4's independent tasks per new SI-1.

## 2026-07-09 | Session close: Plan 4 designed and planned, execution deferred | session 4 (cont.)

**Scope.** Post-Plan-3.5 tail of session 4: commits 7ba90ee (Plan 4 memo, Şenol-approved "lgtm") and c0c0ef7 (Plan 4 implementation plan). No implementation.

**Decisions and why.** Plan 4 plan is WAVED: wave 1 = five independent streams (executor seam T1, --on-collision T4, tests/support+tempdir T5, CI mkvtoolnix T6, richer gated test T7) to run as parallel worktrees - direct consequence of the Plan 3.5 serial-execution criticism, first parallel run for this repo. SI-4 added to HANDOFF: commit/push authorization is standing, never re-request per session (Şenol: "persist indefinitely"; he had to re-grant each session).

**Friction.** The harness permission classifier blocked a commit early in the session despite the repo's standing grant (it reads settings, not repo docs); cleared by explicit in-session authorization. SI-4 documents the distinction (classifier block != revocation); durable fix would be a settings.json allow-rule, Şenol's call.

**Open threads.** Next session: execute Plan 4 (plan c0c0ef7), wave 1 fan-out first; T1 must probe the real --gui-mode grammar before T2 writes the parser; verify T6's CI effect post-push in the Actions log.

## 2026-07-10 | Plan 4 complete (executor + run + queue) | session 5

**Scope.** Plan 4 end to end: 11 tasks, 2 mid-chain fixes, 5-commit final fix
wave. Commits 7aec492..9009d34 (26 commits incl. 5 wave-1 merges) plus the SDD
archive commit. Suite 215 -> 269 tests, gate green throughout.

**Decisions and why.**
- Wave 1 (T1/T4/T5/T6/T7) ran as five parallel worktree streams (SI-1, first
  parallel run in this repo); merges T5->T7->T4->T6->T1 with the full gate per
  merge. Zero real conflicts; the feared T5/T7 collision on
  command_integration.rs auto-merged (disjoint regions). Şenol directive after
  the serial Plan 3.5 run.
- Şenol (2026-07-09): agent commits stay UNSIGNED as policy (a GPG signature is
  his authorship claim), after the controller accidentally signed the five
  wave-1 merge commits (unlocked gpg-agent). Left in history; rule now in
  HANDOFF SI-4 and Peter memory.
- Şenol (2026-07-10): Fluent plural selector over the plan's locked
  "{ $count } warnings" ftl text (plan-mandated finding, escalated per SDD).
- Controller: json-document-on-error-paths fastfollow treated as bugfix, not
  scope change (a machine-readable mode emitting unparseable stdout defeats
  --json); fix pattern taken from validate.rs.

**What the process caught.**
- T2 task review (opus): spawn-failure path routed through delete-partial,
  silently deleting a pre-existing valid output. Origin: implementer design
  call; implementer AND controller missed the deletion side effect. Real
  data-loss bug (fixed f394f61).
- T3 implementer TDD: the plan's pinned watcher shape (exit only on cancel)
  deadlocks thread::scope on every natural completion; done flag added.
  Origin: plan.
- T8 review: plan's locked ftl renders "1 warnings". Origin: plan. -> amendment.
- Plural fixer: $count reached Fluent as a string, so [one] could never match;
  caught only because the dispatch mandated verifying the arg type (79f0447).
- T9 review: json mode printed human diagnostics on profile-load failure in
  run AND dry-run. Origin: upstream (pre-Plan-4 dry_run pattern). -> 3f66a4e.
- Final review (fable): Windows kill -> Warning + partial kept
  (ExitStatus::code() is never None there), breaking D17 exactly where D16
  matters. Origin: plan-mandated LiveJob code. Fixed 75c075f (killed flag +
  resolve_wait seam). Also: mkvmerge_found asserted false on paths that never
  checked (db9f559, 9009d34), uncapped worker count (4b1ddbf).
- Noise: low. One re-review cycle total (T2); everything else first-pass.

**Mechanics/metrics.**
- 11 implementer dispatches (sonnet x10, haiku x1), 11 task reviews (opus x4
  for T1/T2/T3/T8, sonnet x7), 1 whole-branch review + 1 fix-wave verification
  (fable), 3 fix dispatches + 1 five-commit final wave. Controller re-ran the
  4-command gate itself ~9 times (every merge, every acceptance).
- Wave-1 wall clock ~7 min for 5 tasks (longest stream T1); serial chain
  dominated total time. ~16k lines of SDD artifacts archived.

**Friction and failure.**
- T3 implementer stalled twice waiting on background cargo + Monitor events;
  needed an explicit foreground-only directive. All later dispatches carry
  that instruction up front.
- Controller ran merges without gpgsign=false -> the signed/unsigned mix Şenol
  spotted on GitHub. Policy above.
- rm alias interactivity ate a 2-minute timeout during the salvage commit.
- Şenol mid-session challenged whether parallelism was underused; the answer
  held (genuine dependency chain), but the challenge was earned by Plan 3.5.

**Moments.**
- The T11 fixer meta-tested its own "outputs untouched" assertion by injecting
  a same-bytes silent rewrite and confirming the mtime check (not content)
  caught it (run_live.rs, 93d1a6b).
- The final reviewer re-ran the full gate before believing anything, then
  found a Windows-only semantic break in a repo whose CI never runs Windows.
- T3's cancel test needed a killer-gated condvar fake because a naive
  wait-for-Started opened the exact empty-registry race it was meant to pin.

**Deltas.**
- Plan-as-verbatim-code cut both ways again: verbatim interfaces kept four
  task seams drift-free; verbatim implementation text carried two real defects
  (watcher deadlock, Windows kill mapping) into review scope.
- env!("CARGO_BIN_EXE_muxsmith") in the plan vs assert_cmd cargo_bin used
  (existing codebase convention won; letter-vs-spirit noted by reviewer).
- The HANDOFF's fallback hedge for wave-1 merge friction was not needed.

**Open threads.**
- Post-push: verify in the Actions log that the gated tests RAN (T6).
- v1.x backlog carried to HANDOFF: delete_partial error surfacing,
  cancel-before-spawn narrowing, JobEvent serde golden test at Plan 5,
  ConcurrencyTracker doc-hidden before go-public, jobs[].index doc note,
  empty-batch human output (Şenol decision), read_until/lossy line reads,
  ctrlc full pin, CLI test-helper consolidation, rustdoc private-link nit.
- Windows kill fix must be re-verified when the 3-OS matrix activates.
- Plan 5 (Tauri GUI) is next; parked deps unchanged in HANDOFF.
