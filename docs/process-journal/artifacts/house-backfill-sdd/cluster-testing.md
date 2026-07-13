# cluster-testing — house-knowledge clusters for the `testing` domain

Clustered from the per-occurrence `find-E*.md` records (one record per
(topic, approach) occurrence). Records that are the SAME (topic, approach)
across eras are merged here into one cluster; identical `date+ref` deduped,
distinct dates/refs kept as the recurrence signal, never collapsed.

Legend: kind = pattern (adopted) / restraint (rejected, steelman = case for the
loser) / non-decision (deferred, blocked_on). occ_kind = decided / reinforced /
violated-corrected / deferred. status = settled / contested / blocked.
promoted = count >= 3 (promoted_at = 3).

Era→date anchors (verified against the sibling era files): E0 = 2026-07-08
(spec / Plan 1); E3 = 2026-07-09 (Plan 3); E4 = 2026-07-09 (Plan 3.5); E5 =
2026-07-10 (Plan 4 impl/verdicts/journal), with Plan-4 design-memo D18 decisions
dated 2026-07-09; E6 = 2026-07-10 (Plan 5 GUI); E7 = 2026-07-11 (Plan 5.5); E8 =
2026-07-12 (process/meta, CONVENTIONS seed).

**Merges (13).** Four reach the promotion threshold; nine are count-2
deferral→resolution or spec→impl threads. Everything else is a singleton.

Promoted (count ≥ 3):
- `testing-si3-run-binary` (5) — confirm mkvmerge behavior against the real v100 binary, never from memory.
- `testing-support-helpers` (4) — dedup test helpers into tests/support/mod.rs; same-crate dup is a defect.
- `testing-command-golden` (3) — command argv verified by fixture-JSON→argv golden tests, built incrementally.
- `testing-rich-gated-live-guard` (3) — the one-off manual v100 validation becomes a standing attachment+changes gated live test.

Count-2 (spec→impl or deferral→resolution):
- `testing-self-skip-idiom` — gated live tests locate-or-self-skip when mkvmerge is absent.
- `testing-proptest-core` — proptest over matcher/language/planner correctness core.
- `testing-gui-shallow-smoke` — GUI tests stay a shallow smoke; logic lives in core.
- `testing-cli-insta-snapshots` — CLI rendering via insta snapshots.
- `testing-diagcode-all-guard` — exhaustive DiagCode::ALL registry-consistency → param-drift guard.
- `testing-tempdir-hygiene` — replace std::mem::forget(tempdir) with held drop guards.
- `testing-attachment-id-fixture` — attachment-id fixture grounded in real v100 (1-based).
- `testing-deterministic-concurrency` — barriers/condvars/index-keyed, no sleeps.
- `testing-jobevent-golden` — JobEvent serde golden pinned when the GUI consumer lands.

No cluster is `contested`: every merge is a clean deferral→adoption or
spec→implementation thread that resolved, and the standing rules (SI-3, the gate)
were consistently upheld, never re-litigated.

Count discipline: `testing-command-golden` claims record 12 (its dominant
contribution is the reference-example *pure golden*); 12's secondary gated-live
facet is **not** re-counted into `testing-self-skip-idiom`. Record 4 (integration
strategy) is kept a singleton rather than padded into the self-skip idiom, since
its primary approach (synthesize fixtures + e2e) differs from the gating
mechanism 23/30 reinforce.

---

## Patterns (adopted)

### testing-si3-run-binary — confirm mkvmerge behavior against the real binary (v100), never from memory
pattern · settled · count 5 · **promoted (at 3)**
mkvmerge behavior (flag spellings, argv semantics, -J shape, cross-file donor+keep
ordering) is confirmed by running the installed real binary (v100), never asserted
from memory; golden tests then lock the exact strings and the -J shape is pinned to
identification schema v20. Standing SI-3 house rule, re-issued as a per-plan global
constraint and independently upheld by implementers (who hand-ran mkvmerge before
encoding order assertions, unprompted) and by whole-branch reviewers (who drove
real v100 beyond the goldens, e.g. confirming the -J attachment id equals
mkvmerge's --attachments selector id — the one silent-wrong-file risk).

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | plan Global Constraints + memo D9/D11 (Plan 3) | "Confirm mkvmerge behavior by running the binary (v100 installed), never from memory. The -J shape is pinned to identification schema v20." |
| 2026-07-09 | reinforced | whole-branch-review-verdict.md (Plan 3, Strengths) | "I verified all 26 flag spellings exist in v100 ... confirmed the -J id is identical to mkvmerge's --attachments selector id ... the one place a numbering mismatch would have silently selected the wrong files; it is correct." |
| 2026-07-09 | decided | plan Global Constraints (Plan 3.5) | "Confirm mkvmerge behavior by RUNNING the binary, never from memory (gated tests self-skip when mkvmerge is absent)." |
| 2026-07-09 | reinforced | whole-branch-review-verdict.md (Plan 3.5, header + Important #1) | "I verified the one load-bearing runtime claim (donor+keep ordering) against the real mkvmerge v100 rather than trusting the single-file test." |
| 2026-07-09 | reinforced | journal 2026-07-09 (Moments) + task-3/task-7 verdicts | "Tasks 3 and 7 implementers both hand-ran mkvmerge before encoding the order assertion, unprompted (SI-3 holding)." |

### testing-support-helpers — dedup test helpers into a shared tests/support/mod.rs
pattern · settled · count 4 · **promoted (at 3)**
Cross-file test helpers (FakeIdent, lang()) are consolidated into a shared
tests/support/mod.rs subdirectory module — deliberately a submodule, not its own
tests/*.rs binary, to avoid Cargo autodiscovery — and duplicating a helper within a
crate is a defect. Flagged at 3x duplication in Plan 3, scheduled (D18),
implemented byte-identical in Plan 4, codified in CONVENTIONS.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | whole-branch-review-verdict.md (Plan 3, Minor 7) + task-12-review-verdict.md | "the FakeIdent+lang() helper is now duplicated across three test files ... past the three-similar-lines threshold where a shared tests/support.rs would pay for itself." |
| 2026-07-09 | decided | memo D18 | "Dedup the FakeIdent/lang() test helpers (3 copies) into tests/support.rs; new executor/run tests want the same helpers." |
| 2026-07-10 | reinforced | task-5-review-verdict.md | "Subdirectory module avoids Cargo tests/*.rs autodiscovery; FakeIdent/lang byte-identical to the canonical source; local copies deleted." |
| 2026-07-12 | reinforced | CONVENTIONS.md Patterns (b38a46f); idiomacy finding run_cli.rs:L498 | finding flags same-crate verbatim fake_mkvmerge_that_fails_queries dup (with an in-code soft counter-preference, Şenol to decide). |

### testing-command-golden — command argv via fixture-JSON→argv golden tests, built incrementally
pattern · settled · count 3 · **promoted (at 3)**
Command generation (Plan → mkvmerge argv) is verified by golden tests mapping
fixture identification JSON to expected argv; built incrementally with full-vector
assert_eq goldens where each command task extends and re-locks the prior task's
golden as an intended contract change (Task 11 added donor --no-attachments to Task
10's golden), culminating in the full spec 4.1 reference example locked as a pure
golden.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §10 | "command: golden tests, fixture identification JSON -> expected argv." |
| 2026-07-09 | decided | plan Tasks 9-12 + journal Deltas + task-11-review-verdict.md | "each command task extended the argv and updated the prior task's golden (Task 11 added donor --no-attachments to Task 10's golden), verified not a regression." |
| 2026-07-09 | decided | plan Task 12 + task-12-review-verdict.md | "Pure golden = FULL spec 4.1 reference example ... locks the reference example end-to-end as a pure golden (no binary)." |

### testing-rich-gated-live-guard — one-off manual v100 validation becomes a standing attachment+changes gated live test
pattern · settled · count 3 · **promoted (at 3)**
The Plan-3 one-off manual v100 validation is converted into a standing gated live
guard: the test builds a primary MKV with a real attachment (--attach-file /
--attachment-mime-type text/plain, SI-3 probed), renames/changes a track, keeps the
attachment, and re-identifies via -J (asserting track_name==Renamed, default_track
true, attachment file_name preserved) — guarding the real argv surface rather than
only golden string equality. Deferred in Plan 3, scheduled (D18), implemented in
Plan 4.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | whole-branch-review-verdict.md (Plan 3, Minor 1) + journal open threads | "a future argv refactor could regress the shape and the golden test would just be updated to the new (wrong) string with nobody re-running the binary. Fix: add one gated live case with an attachment + a couple of changes." |
| 2026-07-09 | decided | memo D18 | "Richer gated live test (attachment + changes): converts the one-off manual mkvmerge-v100 validation from the Plan 3 review into a standing guard." |
| 2026-07-10 | reinforced | task-7-review-verdict.md | "Real mkvmerge with --attach-file/--attachment-mime-type text/plain (SI-3 probed); asserts track_name==Renamed, default_track true, attachment file_name preserved." |

### testing-self-skip-idiom — gated live tests locate-or-self-skip when mkvmerge is absent
pattern · settled · count 2
Gated live/integration tests locate mkvmerge and self-skip cleanly (have_mkvmerge()
probe as the first statement, eprintln + early return before any tempdir/Command)
when the binary is absent, matching sibling gated tests so CI without mkvmerge stays
green and no assertion is silently skipped.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | reinforced | task-3-review-verdict.md + plan Task 3 | "Self-skip idiom identical to sibling (CI parity) ... same helper, same message pattern, no assertion silently skipped." |
| 2026-07-10 | reinforced | task-11-review-verdict.md | "have_mkvmerge() probe is the first statement, eprintln+return before any tempdir/Command; identical idiom across executor_live/run_live/command_integration." |

### testing-integration-real-mkvmerge — integration tests synthesize fixtures with the real binary and run e2e
pattern · settled · count 1
Integration tests use the real mkvmerge binary in CI to synthesize tiny fixture
MKVs from srt/wav seeds and run end-to-end dry-run and run against them; the gated
tests self-skip until mkvtoolnix is installed in CI.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §10 | "Integration: real mkvmerge in CI generates tiny fixture MKVs ... end-to-end dry-run and run against them." |

### testing-proptest-core — property-based tests over the matcher/language/planner correctness core
pattern · settled · count 2
The correctness core (match algebra, language idempotence/symmetry, planner
determinism + D6 suggestion-survival) is covered by property-based tests (proptest)
alongside unit tests, with deterministic seeds, an exact pin, and non-vacuity
traced to ~100% suggestion-path reach.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §10 + journal 2026-07-08 testing bullet | "matcher + planning semantics: unit tests plus property-based tests (proptest); this is the correctness core." |
| 2026-07-11 | decided | task-14-verdict.md / plan T14 | "all mandated properties present... ~100% of cases reach the suggestion path." |

### testing-gui-shallow-smoke — GUI tests stay a shallow smoke because logic lives in core
pattern · settled · count 2
GUI tests stay a shallow smoke (Playwright via mockIPC against the Vite build, no
tauri-driver; plus FakeSpawner shell tests, i18n and axe) because all logic lives in
core and the UI is a renderer.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §10 | "GUI: thin Playwright smoke; logic lives in core, so UI tests stay shallow." |
| 2026-07-10 | decided | memo D29 / spec 10 | "spec 10 keeps GUI tests shallow because logic lives in core." |

### testing-cli-insta-snapshots — CLI human-readable rendering via insta snapshot tests
pattern · settled · count 2
CLI human-readable rendering is covered by insta snapshot tests with redaction
filters for paths/version/durations (load-bearing), EN locale pinned, CI strict
compare (default CI=true / no INSTA_UPDATE), and review-before-accept enforced.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §10 | "CLI rendering: snapshot tests (insta)." |
| 2026-07-11 | decided | task-22-verdict.md / plan T22 | "redaction complete against all 11 committed snapshots... CI strict via default CI=true / no INSTA_UPDATE; EN pinned by construction." |

### testing-diagcode-all-guard — exhaustive DiagCode::ALL registry-consistency, extended to param-drift
pattern · settled · count 2
The diagnostic-code registry's key()/serde encodings were unlinked; an exhaustive
DiagCode::ALL consistency test guarantees they stay in sync, later reused/extended
into a no-wildcard exhaustive match over all DiagCodes that builds a param fixture
per code and (with a real Fluent parser enumerating all cli.ftl keys) asserts no
rendered `{$` leak.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | violated-corrected | journal 2026-07-08 'What the process caught' (T2) + commit a7c0d89 | "key()/serde kebab encodings unlinked (task review T2) -> DiagCode::ALL + exhaustive consistency tests." |
| 2026-07-11 | decided | task-10-verdict.md / plan T10 | "exhaustive no-wildcard match over all 38 DiagCodes... real Fluent parser for key enumeration; {$ is the correct leak marker." |

### testing-tempdir-hygiene — replace std::mem::forget(tempdir) with held drop guards
pattern · settled · count 2
std::mem::forget(tempdir) leak sites are replaced by returning/holding the TempDir
so it cleans on drop; helpers return the TempDir alongside their value and callers
bind `let (x, _dir) = ...`. 15 sites fixed (13 in planner_resolution.rs, 2 in
suggestions.rs).

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | whole-branch-review-verdict.md (Plan 3, Minor 7) + progress.md Task 7 roll-up | "Several planner_resolution.rs tests forget the TempDir guard ... permanently leaks /tmp dirs across runs. Prefer letting dir drop at function end." |
| 2026-07-10 | decided | memo D18 + task-5-review-verdict.md | "13 sites in planner_resolution.rs, 2 in suggestions.rs; helpers return the TempDir alongside their value, callers bind let (x, _dir) = ..." |

### testing-attachment-id-fixture — attachment-id fixture grounded in real v100 (1-based)
pattern · settled · count 2
The with-attachments fixture is grounded in real mkvmerge v100 -J (attachment ids
1-based, track ids 0-based) with id-agnostic assertions; the earlier 0-based fixture
modeled an id space that cannot occur (functionally harmless because command passes
-J ids through, corrected in the test-hardening pass).

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | deferred | whole-branch-review-verdict.md (Plan 3, Minor 5) | "with-attachments.json uses 0-based attachment ids (0,1,2); real mkvmerge numbers from 1 ... the code is id-agnostic ... functionally harmless, but the fixture models an id space that cannot occur." |
| 2026-07-11 | decided | task-11-verdict.md / plan T11 | "real mkvmerge v100 -J: attachment ids 1-based, track ids 0-based." |

### testing-deterministic-concurrency — barriers/condvars/index-keyed, no sleeps
pattern · settled · count 2
Concurrency tests are deterministic via barriers, condvar gates and index-keyed
scripting with no sleep-based timing bets (recv_timeout documented as a
hang-to-failure converter, not a race window); the cancel test needs a killer-gated
condvar fake because a naive wait-for-Started opens the exact empty-registry race it
is meant to pin.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | task-3-review-verdict.md | "RendezvousSpawner (Barrier), ScriptByIndexSpawner (index via argv not call order); recv_timeout documented as 'hang-to-failure converter, not a race window.'" |
| 2026-07-10 | reinforced | journal 2026-07-10 Plan-4-complete (Moments) | "T3's cancel test needed a killer-gated condvar fake because a naive wait-for-Started opened the exact empty-registry race it was meant to pin." |

### testing-jobevent-golden — JobEvent serde wire shape pinned when the GUI consumer lands
pattern · settled · count 2
The JobEvent serde wire shape (event tag / variant / field names) is pinned by a
golden test once the GUI consumes the event stream, making the shape a contract;
deferred while it had no consumer, added exactly when Plan 5's Tauri consumer landed.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | whole-branch-review-verdict.md (Plan 4, ledger #8) | "add the JobEvent serde golden test when Plan 5's consumer lands; that is when the shape becomes load-bearing." |
| 2026-07-10 | decided | plan T1 / memo D29; task-1-review-verdict.md | "golden test asserts {\"event\":\"output\",\"index\":0,...} verbatim... pins the concrete required behavior." |

### testing-completeness-gates — CI i18n/help completeness gates + eslint no-literal-string
pattern · settled · count 1
CI enforces i18n/help completeness (referenced-but-missing catalog keys, diagnostic
codes without message templates, help-ids without a topic file) and an eslint
no-literal-string rule keeps hardcoded strings out of the frontend; core is
prose-free by construction.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §10 + commit c7a70f7 | "CI fails on catalog keys referenced but missing ... on diagnostic codes without message templates, and on help-ids without a help topic file." |

### testing-invalid-language-fixture — invalid-language fixtures must fail BCP-47 well-formedness
pattern · settled · count 1
Invalid-language regression fixtures must exceed the 8-char primary-subtag limit
(notalanguage), not short zz/zzz: zz/zzz are well-formed BCP-47 and would silently
pass the widened predicate, gutting the test.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | task-4-review-verdict.md (Named-risk verification) + journal (What the process caught, Task 4) | "implementer found zz/zzz invalid-language fixtures are WELL-FORMED BCP-47, so the widened predicate would silently accept them -> those tests stop testing. Changed to notalanguage." |

### testing-keep-donor-deterministic-guard — the fixed branch needs a guaranteed-run unit guard, not only a gated live test
pattern · settled · count 1
The keep+donor track-order branch (the exact scenario the task exists to fix) must
carry a deterministic guaranteed-run unit guard (Plan-literal, asserting
0:0,0:1,1:0), not only a gated live test that silently skips without mkvmerge (e.g.
in CI).

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | task-7-review-verdict.md (Important) -> fix commit aa75025 | "the exact scenario this task exists to fix ... has zero guaranteed-run regression coverage"; fix aa75025 verified "a real guard (scratch-reverted keep branch two ways -> both FAIL)." |

### testing-per-commit-gate — every commit passes TDD RED/GREEN + the 4-command gate
pattern · settled · count 1
Every commit passes TDD RED/GREEN and the full 4-command gate (test / fmt / clippy
-D warnings / cargo-deny); the controller re-ran the gate itself at every merge and
acceptance.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | reinforced | plan Global Constraints | "Standing convention re-applied across all 11 tasks + fixes; controller re-ran the 4-command gate itself ~9 times (every merge, every acceptance)." |

### testing-assert-cmd-cargo-bin — invoke the built binary via assert_cmd cargo_bin, not the plan's env! macro
pattern · settled · count 1
Tests invoke the built binary via assert_cmd's Command::cargo_bin("muxsmith") (the
house convention, as run_cli.rs already does), not the plan's literally named
env!(CARGO_BIN_EXE_muxsmith) macro; the house convention wins over the plan's letter.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | task-11-review-verdict.md + journal Deltas | Implementer used Command::cargo_bin("muxsmith") as run_cli.rs already does; letter-vs-spirit gap noted, house convention took precedence over the plan's env!(CARGO_BIN_EXE_muxsmith). |

### testing-e2e-tsc-drift — a claimed type-drift guard is inert unless tsc actually runs
pattern · settled · count 1
The e2e fixtures' `satisfies JobEvent` annotations only catch contract drift if tsc
actually runs; the report claimed protection while tsc was uninvoked (the guard was
inert), fixed by wiring the e2e tsconfig type-check into test:e2e.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | task-12-review-verdict.md (Important); fix commit 945ee96 | "the specific safety mechanism the report claims protects mock fidelity... is currently inert." |

### testing-unix-file-level-cfg — a standalone unix-only integration crate must gate at file level
pattern · settled · count 1
A standalone unix-only integration crate must gate at file level (#![cfg(unix)]);
round-1 module-level imports under a fn-level cfg(unix) broke Windows clippy -D
warnings and were corrected by fix 3353028.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | violated-corrected | task-3-verdict.md | "a standalone unix-only integration crate must gate at FILE level." |

### testing-regex-wording-snapshot — snapshots embedding third-party wording break on cargo update as intended signal
pattern · settled · count 1
Two snapshots embedding regex-crate wording deliberately break on cargo update as an
intended change signal (the §10 third-party exception), not a fragility bug.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | task-22-verdict.md concern 1 / whole-branch funnel T22-m1 | "breaks only on deliberate cargo update; breaking on regex bump is intended signal." |

### testing-de-locale-presence-guard — de-locale message presence via real-parser column-0 cross-check
pattern · settled · count 1
The de-locale message-presence guard cross-checks column-0 presence in the real
Fluent parser output; the reviewer's suggested reliance on addResource reporting
Junk drops was disproven from library source (addResource errors only on id
collisions; Junk is silently dropped) and the reviewer conceded.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | violated-corrected | task-21-verdict.md fix wave 2 / journal | "addResource errors ONLY on id collisions; Junk silently dropped - verified from library source and reproduced." |

---

## Restraints (rejected — steelman kept)

### testing-keep-track-order-assumption — assumption #3 confirmed against v100, later reversed on usability
restraint · settled · count 1
Assumption #3 (keep-mode --track-order lists only matched tracks; mkvmerge appends
kept-unmatched tracks after, in source order) was confirmed against real mkvmerge
v100 — ordering only id1 of a 3-track source yields BRAVO,ALPHA,CHARLIE — and needed
no planner change, before it was later reversed on usability grounds.
steelman: the assumption was empirically correct as to mkvmerge behavior; the reversal was about usability, not correctness.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | reinforced | task-3-review-verdict.md + progress ledger Task 3 | "SI-3 CONFIRMED against mkvmerge v100 ... yields BRAVO,ALPHA,CHARLIE (ordered first, unlisted after in source order). D20 assumption holds; NO planner change needed." |

### testing-catalog-completeness-rejected — full cli.ftl key-completeness test rejected (wontfix)
restraint · settled · count 1
A completeness test over all cli.ftl keys was rejected as unnecessary: the renderer
unit tests assert rendered fragments, so a missing run-* key fails as a raw-id
render, and milestone tests already cover the run-* keys — that guard is adequate.
steelman: catalog_completeness only iterates DiagCode::ALL, so a completeness test over all cli.ftl keys would close the future-keys gap; but milestone tests already cover the run-* keys.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | whole-branch-review-verdict.md (#9) + task-8 Minor | "the renderer unit tests assert rendered fragments, so a missing run-* key fails as a raw-id render; that guard is adequate." |

---

## Non-decisions (deferred — still open)

### testing-coverage-tooling — cargo-llvm-cov left out, revisit at v1.x
non-decision · blocked · count 1 · blocked_on: no measured coverage need in v1 (revisit at v1.x planning)
Coverage tooling (cargo-llvm-cov) was left out of the Plan-1 tooling stock-take as a
signal not yet needed, with a revisit at v1.x planning.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | deferred | BUILDING.md 'Deliberately not used' (2026-07-08) | "Coverage tooling (cargo-llvm-cov): revisit at v1.x planning." |

### testing-warning-path-coverage — exit-1 warning path output-kept + on_progress surfacing untested
non-decision · blocked · count 1 · blocked_on: v1.x test work
The exit-1 warning path lacks an output-kept assertion and WarningLine/ErrorLine
on_progress surfacing is untested; deferred to v1.x.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | task-2-review-verdict.md (ledger #4 whole-branch) | "exit_one_is_warning_with_captured_lines checks state + captured text but not that the output survives; no test asserts WarningLine/ErrorLine reach on_progress." |

### testing-concurrencytracker-hygiene — doc-hide or relocate ConcurrencyTracker before go-public
non-decision · blocked · count 1 · blocked_on: before go-public
ConcurrencyTracker is pure test instrumentation in the public lib surface;
doc-hiding (#[doc(hidden)]) or relocating it is deferred until before go-public.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | whole-branch-review-verdict.md (Minor) | "ConcurrencyTracker is pure test instrumentation in the public surface - #[doc(hidden)] or relocation before go-public." |

### testing-cli-helper-dedup — apply the T5 consolidation principle to the CLI crate
non-decision · blocked · count 1 · blocked_on: next CLI test file (each tests/*.rs is its own crate; no shared common module yet)
have_mkvmerge/muxsmith/fake-stub helpers are duplicated 2-3x across CLI test files;
applying the T5 consolidation to the CLI crate is deferred until the next CLI test
file appears.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | whole-branch-review-verdict.md (Minor) | "the exact pattern T5 consolidated in core ... apply the T5 principle to the CLI crate when the next test file appears." |

### testing-comment-fixture-convention — promote the _comment fixture-note convention on second use
non-decision · blocked · count 1 · blocked_on: second use (trigger-gated, internal)
The _comment fixture-note convention introduced ad hoc for one file is left ad hoc;
promotion to a written convention is trigger-gated on a second fixture needing it.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | task-11-verdict.md m1 / whole-branch funnel T11-m1 | "if a second fixture needs it, promote to a written convention." |

### testing-proptest-assert-locality — prop_assume → prop_assert in D6 guards to localize regressions
non-decision · blocked · count 1 · blocked_on: idiomacy review (internal)
D6 property guards use prop_assume where prop_assert would localize a future
generator regression (which currently dies opaquely as "too many global rejects");
the switch was deferred.

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | task-14-verdict.md m1 / whole-branch funnel T14-m1 | "prop_assume -> prop_assert would localize generator regressions." |

### testing-check-i18n-self-test — add a fixture self-test for the grown check-i18n.mjs logic
non-decision · blocked · count 1 · blocked_on: v1.x test-hardening (internal)
check-i18n.mjs has no fixture self-test despite grown logic; adding one was deferred
(the e2e guard covers only the parser-blindness half).

| date | occ_kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | task-20-verdict.md m2 / whole-branch funnel T20-m2 | "no check-i18n.mjs self-test despite grown logic." |
