# Plan 5.5 SDD progress ledger

Plan: docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md
Started: 2026-07-11 (session 9), base commit e8e85d9.
Execution: waves per the plan's dependency graph. Wave 0 serial on master
(standing SI-4 authorization; T2 needs push-to-CI verification). Wave 1 as
parallel worktree streams under .worktrees/, merged sequentially, full gate
re-run per merge.

## Cross-task constraints (travel verbatim in dependent dispatches)

- C1 (plan T5 note): T10 builds an exhaustive-match param-fixture table over
  ALL DiagCodes. Every task adding a DiagCode (T4 worker-panicked, T5
  UnknownExtension, T6 EmptyPlan, T13 suggestion-partition, T16/T18 per
  design) either lands before T10's merge and gets its fixture there, or -
  when merging after T10 - MUST add its fixture entry in the same task
  (the exhaustive match makes the build fail otherwise; that is the guard
  working, not a defect).
- C2 (plan global constraint): new Fluent messages in Waves 1-2 are EN-only
  until T19; from T19 on bilingual (en + de).
- C3 (plan T12): from T12 on, the full gate has NINE parts (cargo doc
  -D warnings added). Tasks merging after T12 run nine.
- C4 (from T8): new directly-rendered cli.ftl key `dry-run-summary`
  (params count/root/extensions) needs a param fixture + allowlist entry
  in T10's guard - added at stream-D merge (D merges last), same
  mechanism as C1.

## Task status

- Wave 0: T1 complete; T2 COMPLETE (run 29165610230 green 3-leg,
  skip-marker 0 everywhere, ROADMAP trigger consumed in 765a013)
- Wave 1: A: T3 COMPLETE (3fbaa9e + fix 3353028, re-review approved),
  T4 dispatched (sonnet), T4.5 next | B: T5 done (d9db161, opus review
  running - design deviation: known_extensions via Identify trait default
  method + Locator.extensions doc-claim question routed to reviewer),
  T6 next | C: T8 COMPLETE, MERGED (f335621, CI run 29165814026 green) |
  D: T10 done, review found REAL production bug planner.rs:600
  (InvalidPropertyValue missing `allowed` param -> literal {$allowed}
  reaches users; fix routed to plan T9 item (ix), amended in plan doc);
  T10 fix wave running (report overclaim correction + limitation note);
  T11 after | E: T13 running | F: T14 running
- RESEQUENCED: T9 pulled OUT of stream C - its file list (planner.rs:526,
  ~:431, validate.rs:65+301) overlaps stream B's region; the plan's
  disjointness note is wrong for T9. T9 runs SERIALLY after stream B
  merges to master. Stream C = T8 only, ready to merge.
- Wave 2: T15, T16, T17, T18 pending (design-gated)
- Wave 3: T19, T20, T21, T22 pending
- Close: T23 pending

## Reviewer minors (for T23 roll-up funnel; n-in/n-out)

- T2-m1: skip-marker string is an unenforced cross-file contract (19 call
  sites + 1 CI grep, no shared constant); future reword silently
  reintroduces the false-negative. Hardening: shared const.
- T2-m2: assertion rerun doubles Rust test wall-clock; alternatives exist,
  neither clearly better (verdict file has the analysis).
- T2-m3: apt exact-build pin 97.0-1build1 fragile against archive rebuilds;
  correct under pin preference, cost disclosed in ci.yml comment.
- T2-m4: cross-leg mkvmerge version divergence (apt 97 vs choco/brew 100);
  sanctioned by per-manager pin policy, recorded.
- T8-m1: empty-batch tests pin fields via three independent contains()
  instead of the composed line (run_cli.rs:279ff, dry_run_cli.rs:207ff);
  loose pin, T22 snapshot conversion will supersede.
- T8-m2: key dry-run-summary serves both run and dry-run (existing
  shared-naming precedent; latent catalog-skimming trap; T10 heads-up).
- T3-m1: fn-level #[cfg(unix)] in executor_no_hang_live.rs redundant under
  the file-level gate added by the fix; remove at next touch.

## Log

- Task 1: complete (commit e8e85d9..209218c, review clean, 0 findings). Repo was already LF-clean, no renormalization commit (plan-anticipated). NOT yet pushed - wave-0 push after T2 with controller gate re-run.
- Task 2: code complete (commit 209218c..374005a, review approved, 4 minors
  recorded above; verdict file task-2-verdict.md). Implementer corrected the
  brief: runner is ubuntu-26.04, marker inlined at 19 sites. Steps 4-5
  (push, live 3-leg verification, ROADMAP trigger consumption) = controller,
  pending gate + push. Live-CI checklist in the verdict file.
- Controller note: first gate-runner attempt failed on zsh no-word-splitting
  (unquoted $c = one command name), not on the code; rewritten as bash
  script (scratchpad gate.sh). Doctrine §4 environment-assumption case.
- Controller gate re-run GREEN (8/8) at 374005a; wave 0 pushed
  (e8e85d9..374005a); CI run 29165166725 watch running (T2 steps 4-5
  pending its result: 3-leg live verification, ROADMAP trigger
  consumption).
- T2 fix wave 1: Windows leg failed run 29165166725 with marker count 18 -
  choco writes machine PATH, running job never re-reads it; the assertion
  caught exactly its target case. Fix 19deec3 (existence check +
  GITHUB_PATH append, pwsh). macOS/ubuntu legs verified count 0 with LIVE
  runs (macOS gated tests live-green for the first time). Controller gate
  + push + CI re-watch running; the CI re-run IS the fix's re-review
  (YAML-only, only live CI can prove it).
- Wave 1 started from base 374005a, six worktrees .worktrees/stream-a..f
  (branches plan55-stream-a..f). First tasks dispatched in parallel:
  T3(A, sonnet), T5(B, sonnet), T8(C, sonnet), T10(D, sonnet),
  T13(E, opus), T14(F, opus). Planned merge order: D LAST among
  DiagCode/key-adders (C1+C4). Status: T3 DONE on stream-a (3fbaa9e,
  opus review running); T8 DONE on stream-c (8188bf8, sonnet review
  running). T2 fix wave 2: 24ac702 (writable handle for set_modified,
  Windows PermissionDenied at run_live.rs backdate_mtime - first-ever
  live Windows run surfaced it; only set_modified site in repo). Gate +
  push + CI watch running for 24ac702.
  Rationale for D-last: its exhaustive guard forces fixture entries for
  T4/T5/T6/T13 codes at its merge - controller adds them there (C1) plus
  the C4 key. T12 note:
  touches queue.rs:73 (doc link) while T4 touches queue.rs:270ff -
  different hunks, merge fine, order D after A anyway.
- T5-m1 -> ROUTED into plan Task 5.9 (locator walk + model.rs:254 doc).
- T5-m2: known_extensions default-None on Identify inverts the idiom (sole
  production impl overrides; future production impl silently vacuous).
  Fix candidate: required method, FakeIdent returns None. T23 funnel.
- T14-scope-notes (implementer): D6 property single-primary per case;
  deltas applied via emitted yaml_fragment. Review (opus) running.
- T5 COMPLETE (d9db161, approved). Plan amended: Task 5.9 added (stream B
  after T7, spec §4.6 locator completion). T6 dispatched.
- T14-m1: D6 prop_assume -> prop_assert would localize generator
  regressions (now: opaque reject-cap death). T23 funnel.
- T14-m2: D4 separator-injection only exercisable with free-text template
  tokens (none in v1 surface); inherent limit, recorded.
- T14-m3: test-side duplication of private with_rule_match/diag_signature
  logic; shared-bug escape hatch, covered by suggestions.rs unit tests.
- T14 COMPLETE (25f2657, approved). Stream F merged to master; gate+push
  +CI watch running.
- T13-m1: fixless affected file silently dropped from partition (best=None
  skip; unreachable under v1 id-uniqueness). Harden: invariant comment or
  "unresolvable" group. T23 funnel.
- T13-m2: Fluent { $fix } column-0 in group pattern - rendered indentation
  cosmetics. T23 funnel.
- T13-m3: partition trigger per-rule (correct), brief phrase "batch-wide"
  misreadable - report note only.
- MEMO-PENDING (for the plan-5.5 memo when T15 creates it): D6's
  "conflict-signature multiset" partition-key wording superseded by
  suggestion-keyed grouping, RATIFIED via spec §5.3:282 (outcome mandate,
  not key mandate; reviewer: suggestion-keying is the more literal
  reading). Record as memo residue with this rationale.
- T13 COMPLETE (0ddd945, approved; controller verified per-commit scoping
  via git show --stat). Stream E ready to merge after merge-F CI settles.
- MEMO-PENDING (SI-3, from T6): mkvtoolnix-gui has NO general
  zero-selected-tracks warning (tab.cpp:489-522 audio-only opt-out
  confirm dialog; isReadyForMerging never checks total selection);
  mkvmerge exits 0 on empty output. Muxsmith's unconditional per-file
  EmptyPlan warning = DELIBERATE DIVERGENCE (batch tool must say it).
- T6 DONE (c09875e, stream B), review dispatched. Merge-E CI green.
- T11-m1: _comment fixture-note convention ad hoc for one file; promote to
  written convention on second use. T23 funnel.
- T11 COMPLETE (543259b, approved). T12 dispatched on stream D.
- T4 COMPLETE (a4ab647, approved). Whole-branch items (Important,
  non-blocking): T4-i1 eprintln-in-core (facade candidate); T4-i2
  worker-panicked rich message renders on no live surface (route
  JobOutcome.errors codes through the catalog - cross-cutting). Minors:
  T4-m1 lock_active doc over-claims the abort/cancel arms; T4-m2
  child-process leak on post-spawn panic (invoke killer before removing).
- D32 DECIDED (Şenol): raw:-opt-in (shape B), RawProperty kept. D33
  DECIDED: symmetric acceptance-filtered (policy 3), tiebreak
  broader-then-index, NOT-seed. Memo committed ff11e01 with acceptance
  test cases + residues (D6 wording, T6 divergence). T15/T17 done.
- T6 fix wave running (post-finalize relocation + batch-report test).
- T4.5 dispatched (stream A). T12 done (004e1e8, 3 extra dead links
  surfaced+fixed), review running.
- T12 COMPLETE (004e1e8, approved; delink-vs-publicize judgment ratified,
  pub-use middle path named+rejected). Minors: "module-private" phrasing;
  --workspace deviation documented-inert. STREAM D COMPLETE (T10+T11+T12),
  holds for LAST merge per C1/C4.
- T6 COMPLETE (c09875e + fix a60e9a0, re-review approved; relocation also
  closed the unenumerated skip-collision false-warning case; beneficial
  ripple: suggest() sims now reject candidates newly producing empty
  plans). Minor: no attachments/chapters-only test (doc value). T7
  dispatched.
- T7 COMPLETE (0456f72, approved; RED/GREEN manually traced by reviewer).
  Minor routed: attachment-donor exposure (same class, pre-existing) ->
  NEW plan Task 7.5 (stream B after T5.9); plan amended, Şenol gets the
  scope note in the next summary. T5.9 dispatched.
- T5.9 COMPLETE (3df7fc1, approved, zero findings). T7.5 dispatched (last
  stream-B task).
- T7.5 DONE (ca238dc; capture point already pre-render, wiring only).
  Implementer found the THIRD class member: chapters donors
  (ChapterSource::External ~:866) -> NEW plan Task 7.6 (class closure by
  construction: exactly two Locator field sites, completeness comment).
  T7.6 dispatched; T7.5+T7.6 get ONE combined review (same mechanism,
  both briefs, both verdicts).
- T7.5+T7.6 COMPLETE (ca238dc+99b2e34, combined review approved). #7 class
  CLOSED by construction. Minor: resolve_file top doc stale re chapters
  (T23 funnel). STREAM B COMPLETE (T5,T6,T7,T5.9,T7.5,T7.6). Merging.
- Journal correction package COMMITTED 0706b09 (Şenol-approved; user.md +
  doctrine edits agent-side, in his pending ~/agents commit).
- STREAM D MERGED (240fd35): queue.rs conflict kept both streams' tests +
  restored T12's worker_count delink (controller's first resolution lost
  it - caught by the fixture-fixer's doc check); joblog.rs prune doc +
  delink combined; guard forced fixtures for UnknownExtension/EmptyPlan/
  WorkerPanicked/SuggestionPartition + dry-run-summary allowlist (C1+C4
  consumed). Gate is NINE parts from here (C3). WAVE 1 COMPLETE pending
  CI green.
- Next: T9 serial on post-D master (BEFORE T18 - ROADMAP #13(i) claimants
  listing feeds overlap suggestions), then T16 + T18 (parallelizable,
  disjoint regions), then wave 3 serial (T19-T22; T21 has a HARD Şenol
  terminology gate before merge - if he is away, draft waits and T22 may
  run ahead with EN-pinned snapshots, deviation to be noted), then T23.
- T9 COMPLETE (408593e..697dd70, 9 commits, approved; lockstep
  controller-verified via git show). (vii) FIXED not-deliberate, (viii)
  KEPT + spec §8.4 entry. Important routed -> Task 9.5 (donor name in
  UnsupportedSource, before T21). Minors: (iv) flat config_diags JSON
  unsorted vs validate parity; (viii) exception broader than pure
  pass-through, named explicitly. T9 merged; T9.5 + T16 + T18 next in
  three worktrees (planner-region conflicts expected additive: ftl +
  fixture guard).
- T9.5 COMPLETE (d5b8bef + test 8846d34, approved; fix wave additive-test
  only, controller-verified instead of re-review dispatch). Merged.
  Minor: report phrasing overstated the no-alternative claim (recorded).
- T16 COMPLETE (27c8b79, approved-conditional; merged 2ec98d2). TWO Şenol
  ratifications PENDING (memo D32 addendum + ROADMAP entry): schema-drift
  advisory drop; B-8 single- vs dual-field. Bundled with the T21 gate.
  Minor: bare raw: accepted with empty property (cosmetic; no panic).
  Review also validated the implementer's stale-premise catch (live :445
  emitter, memo said dead code).
- T18 COMPLETE (f68e5d7, approved; both deviations adjudicated in favor,
  memo TC-A wording + partition-branch note corrected cecd47b). Minors:
  dead fixture string in TC-A test; SuggestionsCapped filing-convention
  comment line. T23 funnel. WAVE 2 COMPLETE. Wave 3 next (serial:
  T19 -> T20 -> T21[Şenol-Gate] -> T22).
- T19 COMPLETE (bf4dbc5 + fix 0d19dc4, approved). Numeric-promotion
  mirror pinned (Rust-side test names the TS file). Minors: report
  fixture-value misquote; visibleText over-wide BiDi range. T23 funnel.
  T20 dispatched (same wave3 worktree, serial).
- T20 COMPLETE (47cb078, approved; cli.ftl parity extension ratified with
  independent verification). Minors on record FOR T21: regex-vs-parser
  divergence on malformed multiline indentation; no fixture self-test.
- T19+T20 merged to master. RESEQUENCING (deviation, reasoned): T21 (de
  locale) has the HARD Şenol terminology gate before merge and he is out
  of the loop -> T21 and T22 run PARALLEL on own branches from post-T20
  master; T22 (EN-pinned snapshots) may merge before T21 - safe because
  de/ files cannot affect EN-pinned snapshots, and T20's parity gate
  guards T21's catalogs whenever they merge. Plan's serial order was
  motivated by message-stability (T19 before T22), which is preserved.
- T22 COMPLETE (aba7f4f + fix 5a1bd8f, approved). 11 snapshots, 13
  converted, 8 kept-semantic; regex-wording snapshots deliberate (breaks
  on cargo update = intended). Merged.
- T21 draft DONE (1262c14): tech review NEEDS-FIXES on two test-only gaps
  (S15 normalization untested; nested-selector real-parse unguarded) ->
  fixer running. German itself + terminology anchoring APPROVED by tech
  review (zero objective errors, zero sentence overlap with de.po).
  AFTER the fix: T21 waits on the ŞENOL BUNDLE (terminology table +
  register mix; dropdown/locale-hint gap decision; D32 sub-decisions).
- ŞENOL GATE PASSED (2026-07-12): terminology approved with 3 corrections
  (Starten, Meldungen, Verweis - applied 362db2d); T21.5 SOFORT; skew
  notice REBUILT once-per-batch (T16.5); B-8 single-field RATIFIED (no
  change); NEW README content anchor: magic properties explicitly listed
  (ROADMAP). T21 MERGED (2ae62dd). T21.5 + T16.5 dispatched parallel.
- T21.5 COMPLETE (4d946f4 + fix b833f2a, approved). Endonym-Labels,
  evergreen Hint (2 Sätze), Neustart-Hinweis - Şenol-Veto offen auf die
  Hint-Texte (im Summary gemeldet). Live-locale-switch als Plan-6-Kandidat
  recorded. MERGED.
- T16.5 COMPLETE (a86eecb + doc fix ce4fae1, approved; primaries-only
  structurally proven correct; suggest()-interaction clean by
  construction). MERGED. ALLE TASKS DURCH - nur T23 (Close-out) offen.
- WHOLE-BRANCH REVIEW (fable): 1 Critical (spec severity drift vs owner
  ruling - introduced by the T16.5 doc fix, each side locally consistent),
  3 Important, 3 new minors; triage 37-in -> 3 FIX-NOW / 16 DEFER / 14
  DISCARD / 4 RESOLVED. Fix wave 98e869a (five items, nine-part gate
  green) + controller row f25b02d (UnsupportedSource - fixer-surfaced
  fourth member of the §5.2 gap class). Final verification: all four
  points VERIFIED (incl. killer-invoke soundness: Arc survives unwind,
  idempotent contract), M1-M3 landed, verdict READY.
- ROADMAP funnel committed a6976d0; WorkerPanicked-cell nit -> idiomacy
  list. PLAN 5.5 CODE-COMPLETE AND ACCEPTED. Close-out: salvage, journal,
  HANDOFF, final push.
