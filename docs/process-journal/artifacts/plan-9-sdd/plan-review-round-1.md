# Plan 9 plan review, round 1 (independent reviewer)

Artifact: `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`
(432 lines by `wc -l`, matching the brief; seven tasks). Reviewed 2026-07-28
against the design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`,
read in full, 1515 lines), the v1 spec (anchors read), the plan brief, the
ROADMAP Plan-9 anchor, and the four house-knowledge files. All probes were
built fresh at `/tmp/plan9-indep-review-c9k4/` (my own path, this pass); no
instrument of the plan author or design reviewer was re-run.

## Verdict: APPROVED

Three Minor findings, none blocking. The coverage walk (done from the design,
then checked against the plan's map) found no design section without a task, no
map row claiming coverage the tasks do not deliver, and no invented producer
for the two uncovered acceptance observables. Every code anchor and both
fixture probes in the plan's authoring-time verification section reproduced
exactly on the current tree, with one exception (M-1, a post-authoring tree
change, not an authoring error).

## Coverage table (walked from the design first, map checked second)

| Design section | Task/actor (my walk) | Plan's map agrees | Gap |
|---|---|---|---|
| Section 0 notes 1-5 | Informational; note 3 -> Task 6 (harness deviation), note 5 -> Task 3 (no JobRowData change) | yes | none |
| Section 1 SI-3 parity audit | Informational, feeds D99/D100/D102 | yes | none |
| D91 seam + 4 call-site mappings | Task 1 steps 1, 3, 4 | yes | none |
| D92 `mkvmerge_found` meaning + builder doc | Task 1 step 6 (doc at `json.rs:66-73`, verified present) | yes | none |
| D93 borrowed cache + GUI session cache | Task 1 steps 2, 5 (incl. disjoint-mutex comment duty) | yes | none |
| D94 `job_specs` + per-surface empty-specs | Task 1 steps 1, 3, 4; end-state queue call Task 2 | yes | none |
| D95 single `"."` default | Task 1 step 1 + D95 absence check in step 8 | yes | none |
| D96 `run_batch` hoist + 2 tests move | Task 2 steps 1-4 | yes | none |
| D97 runs-root deletion, 3 call sites | Task 2 step 5 | yes | none |
| D98 `JobOutcome.panic` + wire mirrors + eprintln deletion + licence rewrite | Task 3 steps 1-3 | yes | none |
| D99 CLI two-line render + catalog obligations | Task 3 steps 4-6 | yes | none |
| D100 GUI job-row render | Task 3 step 7 | yes | none |
| D101 `EmptyRawProperty` + pinned tests | Task 4 (all steps) | yes | none |
| D102 central sort + `severity_sorted` hoist | Task 5 steps 1-3 | yes | none |
| D103 BatchView code-keyed fetch | Task 5 step 4 | yes | none |
| D104 harness widening + 4 e2e tests | Task 6; ledger half pre-existing (verified: `gui-d23-reset-gating-form` in ledger) | yes | none |
| D105 D49 experiment protocol | Task 7; ledger entry controller-written at close | yes | none |
| Section 2 "stay discarded" recount | Carried in Task 2's block addressed into Task 3 | yes | see M-3 |
| S-1 WorkerPanicked replacement / EmptyRawProperty insertion | Task 3 step 8 / Task 4 step 5 (both spec anchors verified: line 289 row, `RawOnKnownProperty` row at :280) | yes | none |
| S-2 (spec 8.4 exceptions) | Task 3 step 8 (anchor fragment verified at spec :409) | yes | none |
| S-3, S-5, S-6 (spec 4.4/5.4/9.2) | Task 4 step 5 (all three anchor sentences verified at spec :176/:301/:419) | yes | none |
| S-4 (spec 7 module table) | Task 1 step 7 (`planner` row verified at :342; no `pipeline` row exists yet) | yes | none |
| S-7 (spec 5.2 paragraph) | Task 5 step 5 (anchor sentence verified at :255) | yes | none |
| Section 4 ledger obligations | `gui-d23` done; `core-d49-g1g2-experiment` + `core-121` at close (both close actions present) | yes | none |
| Section 5 must-not-decide | Global Constraints + per-task blocks; walked all 16 bullets, each lands in the owning task's block or Global Constraints | yes | none |
| Section 6 triggers | Done pre-plan; all four "Plan-9 design trigger 1..4" lines verified in ROADMAP (:635, :639, :643, :648) | yes | none |
| Section 7 observables 1-8 | Acceptance map: T1, T2, T2, T3+T6, T4, T5, T6, T7+controller; all MV claims hold | yes | none |
| The 2 uncovered observables | Carried as named, uncovered consequences (acceptance map, Task 4/5 must-not-decide, close-action ROADMAP note); no producer invented, none dropped | yes | none |
| Amendment log | Bound via pointer contract at execution time (Global Constraints) | yes | none |

## Findings

### Minor

**M-1. The routed exec-43 discrepancy no longer exists on the tree; the plan's
evidence line does not reproduce.** Location: authoring-verification
corrections table row 2, and the promotion-sweep close action's "Routed
discrepancy" rider. The plan states `exec-43-runsroot-debug-gated` "carries NO
`source` field at all (its fields run id, kind, tier, domain, statement,
steelman, blocked_on, status, promoted_at, count, occurrences)". On the current
tree it carries both `source: human` and `nature: technical-code`:

```
$ sed -n '1123,1129p' docs/decision-ledger.yaml
- id: exec-43-runsroot-debug-gated
  kind: pattern
  tier: 1
  source: human
  nature: technical-code
  domain: executor
  statement: "..."
```

This looks like a post-authoring controller edit, not an authoring error:
`stat` shows `docs/decision-ledger.yaml` modified 05:06:14, after the plan file
(05:04:51), and the exact 2-line insertion explains the plan's other ledger
anchor drifting (`gui-d23-reset-gating-form` cited ":4535 at authoring", now
`:4537`). I could not confirm the intervening commit itself (no git commands
under this brief). Fix: the controller records the routing as already satisfied
(progress-tracker note or a one-line plan amendment) so the close action's
premise is not false at execution; no task is affected, and the close action's
"or records why" arm makes it self-limiting either way.

**M-2. Task 4's commit stages a directory, not files.** Location: Task 4 step 7
(`git add ... crates/muxsmith-cli/tests/snapshots ...`). Under the
explicit-staging rule (Global Constraints, "stage files explicitly"), staging
the snapshots directory would also silently pick up any stray insta pending
file (`*.snap.new`) left by an unrelated red run. The two snapshot names are
deterministic from the fixed test names and the file's existing convention
(verified: 11 files, all `<test_file>__<test_fn>.snap`):
`cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap` and
`cli_validate__bare_raw_property_renders_german_with_locale_flag.snap`. Fix:
name the two files in the `git add`.

**M-3. The "stay discarded" constraint reaches Task 3 only by an ambiguous
back-reference, while the coverage map claims it is "carried verbatim in Tasks
2 and 3".** Location: Task 3's Interfaces block: "Carries verbatim into Task 6:
'the RunHistory export ...'; and the Task-2 'stay discarded' constraint above."
Grammatically that sentence routes the executor-discard constraint into Task 6
(an e2e-only task where it has no bite), while Task 3 - which edits `job.rs`,
`joblog.rs` and `queue.rs`, exactly where the four discarded failures live -
holds the text only via Task 2's block addressed at it. Task 3's must-not-decide
names `delete_partial_failed` but not the four discarded failures. Substance is
present in the plan and the design's section 2 is Task-3 read-first material,
so the risk is small. Fix: paste the constraint sentence into Task 3's own
section (Interfaces "Consumes" or must-not-decide) and drop the trailing clause
from the Task-6 carry sentence.

## Verification log (what I re-ran, all from my own probes)

- **Hit sets** (all reproduce the plan's pasted values exactly): funnel-inline
  grep -> 4 hits at `dry_run.rs:60`, CLI `run.rs:85`, src-tauri `run.rs:263`,
  `lib.rs:211`; source-default grep -> 4 hits (`:109/:143/:230/:291`, none in
  core); core-stdio grep -> 2 hits (`lib.rs:23` comment + `queue.rs:396` call),
  control over CLI src -> exactly the 6 named files; `MUXSMITH_RUNS_ROOT`
  tree set -> exactly the 9 enumerated lines; `fn run_batch` `:782`; the two
  tests `:1228/:1334`; `resolve_runs_root` `:326/:529/:535/:827`; join-expect
  CLI `:271` / src-tauri `:799`; panic test `:738`.
- **Other anchors**: `severity_sorted` `commands/mod.rs:21`; builders
  `json.rs:35/:78/:112/:176`; `BatchView.vue:218/:225` (`:225` the only
  `config_diagnostics[0]` in the file); `raw_opt_in_diagnostic` `:408` in
  two-branch form, arms `:268` (with `continue`) and `:310`; `"n/a"` `:487`;
  `AddExact` arm `planner.rs:1823`, `scalar_display` `:856`; G1/G2/G3
  `suggestions.rs:1037/:1074/:1113`; `LiveIdentifier` constructions
  `command_integration.rs:231/:493`; `AppState` `lib.rs:89`;
  `load_profile_body` Err arm `lib.rs:316-320`; `Diagnostic::error` `:242`;
  `#[cfg(test)]` `report/mod.rs:305`; `worker-panicked` en `:80` / de `:87`;
  i18n glob `index.ts:18`; `WorkerPanicked => vec![]` `:152`,
  `ALLOWLISTED_CLI_KEYS` `:182`; smoke describe `:477`; `installMockIPC`
  `mocks.ts:84`; harness bundles `mocks.ts:24` / `mount.ts:14`; `tempfile`
  in core dev-deps; `muxsmith-gui` is the src-tauri package name; `outcome()`
  helper `:522` and `finished_failed_renders_exit_code` `:747`;
  `rejectWith`/`resolveWith`/`__muxsmithE2E__.mockIPC`/`.emit` all exist;
  spec S-anchors all present; ten gate parts match BUILDING.md (`:77-82`,
  `:65/:99/:100/:101`); `dry_run.rs` takes `on_collision` (mapping 1's value
  exists); JobOutcome-shaped e2e literals confirmed in `smoke.spec.ts` only
  (`:554/:559/:569/:574`), so Task 3's EXHAUSTIVE list holds; the only
  `profile: null` e2e fixture is `help-mode.spec.ts:363` with empty
  diagnostics, as D103 claims.
- **Fixture probes**, rebuilt at `/tmp/plan9-indep-review-c9k4/` and run against
  `target/debug/muxsmith`: bare `'raw:'` exact and substring each return
  exactly one `raw-property` info at `tracks[0].match.exact.raw:` /
  `tracks[0].match.substring.raw:`, `params.property` `""`, exit 0 - the
  today-state Task 4 flips. The Task-5 parity fixture returns
  `validate --json` = `[unknown-property, invalid-regex, raw-on-known-property,
  raw-property]` and `dry-run --json` (empty PATH) =
  `[raw-property, raw-on-known-property, unknown-property, invalid-regex]` -
  the fixture discriminates exactly as the plan claims (parity test red today).
- **Negative results, fire-verified**: the typography scan of the plan found
  none of the denylisted glyphs; the same pattern fired on a constructed
  em-dash control (`printf 'a \xe2\x80\x94 b' | grep -cP` -> 1). The
  no-positional-`config_diagnostics` claim over the CLI test suites found
  nothing; the same pattern fired on `BatchView.vue:225` (control -> 1). The
  `core-d49-g1g2-experiment` absence grep ran in the same invocation that found
  `core-121-planner-seam-and-hoist` and `gui-d23-reset-gating-form` (present
  controls). The latitude-word sweep of the plan hit only the self-review line
  that names the search terms; the pattern fired on the design's D101 fence
  ("for example", control -> 1).
- **Membership-not-position claim** (Task 5): all `config_diagnostics`
  assertions in `dry_run_cli.rs`/`run_cli.rs` are `.iter().any(...)` shapes;
  no positional read exists in the test suites.
- **Sequencing**: file-overlap graph checked from the EXHAUSTIVE lists - 1->2
  (CLI `run.rs`, src-tauri `run.rs`), 2->3 (`queue.rs`), 3->6 (panic wire +
  render), 3/4 share `catalog_completeness.rs` + `diagnostics.ftl` + the spec,
  4/5 share the spec and CLI test crate; the serial ruling's premise holds. The
  staged 1/2 cut: the retained inline block consumes `specs` (same
  `Vec<JobSpec>` type from `pipeline::job_specs`), `logger`, `milestones`,
  `json`, `renderer`, `total` - all still in scope after Task 1's migration, so
  the intermediate state compiling is a sound claim (block read at
  `run.rs:236-275`).
- **Model tiers**: a tier and ground per task; the no-cheap-tier claim tested
  against Task 4 (tests composed against idioms, not carried) and Task 7
  (live evaluation/anomaly routing is judgment; matches `proc-03`'s
  "cheapest for tasks whose code the plan already carries" boundary). Holds.
- **lib.rs:23 survivor comment**: refers to the `MKVMERGE_SKIP_MARKER` string's
  ~21 eprintln sites in tests across three crates - it stays true after D98
  deletes core's one src call, so Task 3's expected exactly-one-hit green
  state is genuinely reachable and not stale-comment-dependent.

## HARVEST

- **Dominant pattern**: unusually high plan fidelity - every one of the ~40
  re-run anchors and both binary probes reproduced byte-for-byte. The one
  non-reproducing evidence line (M-1) traces to a post-authoring tree edit,
  not to recalled evidence. The design-cycle failure mode (counts attributed
  to commands never run) did not recur.
- **Process gap this round surfaced**: a controller edit to ground truth
  between plan-authoring and plan-review (the exec-43 `source` fix at 05:06)
  invalidates plan evidence lines with no record the reviewer can consult
  under a no-git constraint. A one-line "resolved post-authoring" note in the
  SDD scratch at the moment of such an edit would have converted M-1 from a
  forensic reconstruction (mtimes + line-drift arithmetic) into a lookup.
- **Boundary hit**: the plan's claims about git history (`b4daed6`/`39d7c42`
  doc-only; the artifact commit id `2155c1d`) are unverifiable under this
  brief's no-git rule. I verified their present-tree consequences instead
  (ledger entry exists, triggers mirrored, all code anchors unmoved), which
  bounds the risk but does not confirm the history statements themselves. If a
  future brief wants those checked, it must permit read-only git.
- **Repeated near-findings that dissolved on verification**: the Task-5 parity
  fixture's fourth diagnostic (`unknown-property`) looks unexplained on paper
  (three rules, four codes) but reproduces empirically; the `lib.rs:23`
  comment looks like it would go stale under D98 but is about test-side
  eprintln sites. Both are the class where running the premise beats weighing
  it.

---

# Round 2: delta review of the three round-1 findings (same reviewer)

Reviewed 2026-07-28 against plan commit `811c1df` (was `2155c1d`); diff of the
two read in full - 8 insertions, 8 deletions, all inside the three findings'
scope, no task step, count, or constraint changed beyond them. Read-only git
was permitted this pass (coordinator's correction of the round-1 brief);
fresh probes at `/tmp/plan9-indep-delta-r2-m3v/` scratch naming, none of the
round-1 instruments re-used where a fresh command could re-measure.

## Verdict: APPROVED

All three fixes close their findings. Neither ripple claim had to be taken on
faith and both verified. No fix introduced a new defect.

## Per-finding verdicts

**M-1 - CLOSED.** The corrections-table row 2, the promotion-sweep rider, the
`gui-d23` line citation, and the self-review parenthetical are all rewritten
to record the gap as found-at-authoring, routed, and satisfied in commit
`2155c1d`. Every factual claim in the new text verified against history (now
possible):

```
$ git show --stat 2155c1d   # "plan: Plan 9 draft ...; fix exec-43's missing source field"
 docs/decision-ledger.yaml                          |   2 +
 .../2026-07-28-plan-9-core-hoists-planner-seam.md  | 432 ++++++++++
$ git show 2155c1d -- docs/decision-ledger.yaml | grep "^+" (body)
+  source: human
+  nature: technical-code
```

`.superpowers/sdd/plan-9/progress.md` exists (05:22) and carries the exact
"Post-authoring ground-truth edits" note both plan passages cite. The commit
timestamp (05:06:41) matches the plan's "resolved it at 05:06" and my round-1
mtime reconstruction. Residual-staleness sweep: `grep "carries NO \`source\`"`
on the fixed plan returns nothing; the identical pattern fires once on the
`2155c1d` version (fired control). Ripple claim VERIFIED: all four consumers
of the old statement are updated and no fifth exists (`grep -n exec-43` over
the fixed plan returns only the four rewritten sites).

**M-2 - CLOSED.** Task 4's Files list and its `git add` both name the two
snapshot files; the names match the fixed test fn names in steps 4 and the
directory's convention. The author's re-verification claim VERIFIED by fresh
probe: all 11 existing files in `crates/muxsmith-cli/tests/snapshots/` follow
`<test_file>__<test_fn>.snap` (prefixes `cli_validate`, `dry_run_cli`,
`run_cli`, `run_live`). The `.snap.new` stray-staging hazard is gone.

**M-3 - CLOSED.** The four-discarded-failures constraint now sits verbatim in
Task 3's own must-not-decide block (string identical to Task 2's carry block,
plus a correct new locator parenthetical), and the ambiguous trailing clause
is removed from the Task-6 carry sentence, which now carries only the
RunHistory constraint - the one with Task-6 relevance. The coverage-map row
"carried verbatim in Tasks 2 and 3" is now literally true.

## New-defect scan of the changed lines

- The intermediate commit `4511a3a` (mines the round-1 harvest, +28 ledger
  lines) inserted at ledger `:4548`, AFTER `gui-d23-reset-gating-form`, so the
  plan's corrected `:4537` citation is accurate on the current tree (verified:
  `grep -n` returns `:4537` today).
- The plan now cites the git-ignored scratch path
  `.superpowers/sdd/plan-9/progress.md` from a committed document, twice.
  Judged acceptable, not a finding: the citations are consumed at the plan
  close while the scratch is still live, and the house precedent (ROADMAP
  plan-7.5 trigger note) explicitly keeps pre-salvage paths in
  records-of-what-was-said. Worth one glance in the salvage sweep, nothing
  more.
- Self-review's "Brief refutations: two" still counts its rows correctly; no
  other count consumes any changed line.

## Round-1 residue now closed by git access

The two history statements round 1 could only bound indirectly both verify
exactly as the plan states them: `b4daed6` touched `docs/ROADMAP.md` +
`docs/decision-ledger.yaml` only; `39d7c42` touched `docs/ROADMAP.md` +
`docs/process-conventions.yaml` + the design file only. The round-1 HARVEST
boundary item is thereby resolved for this plan.

---

# Amendment 1: delta review (same reviewer, third pass)

Reviewed 2026-07-28: routing brief
(`.superpowers/sdd/plan-9/amendment-1-brief.md`), amended design at `064da74`
(diffed against `811c1df`'s tree state, read in full), amended plan at
`5d05560` (diffed against `811c1df`, read in full). Fresh probes at
`/tmp/plan9-amend1-delta-k7q2/`; every empirical claim below re-run this pass.
The owner's two rulings themselves are not reviewed.

## Verdict: NEEDS FIXES

One Important finding (a plan-introduced false fire-verification claim in
Task 4), one Minor (region names that do not resolve on the tree). Both are
surgical wording fixes; no coverage gap, no cut dependency, no count error.

## Per-item verdicts

**1. Coverage, delta only - PASS.** My own walk of the changed design
sections, map checked second: S-8 -> Task 1 (files list "S-4 and S-8 only",
step 5, coverage-map row "S-4 + S-8 -> Task 1"); the rewritten D93 -> Task 1
(step 1's S6 `IdentifyCache::new()`, the Interfaces non-change list, the
rewritten must-not-decide bullet); D101's producer paragraph -> Task 4 (files
list, step 5, verification, acceptance row 5, must-not-decide); D103's
producer paragraph -> Task 5 (same five sites); the design's new section-5
two-scenarios bullet -> both tasks' must-not-decide plus the Global
Constraints scenarios-in/infrastructure-out boundary. The plan's map matches
the walk; nothing in the delta lacks a task.

**2. Placement - holds operationally; the region NAMES do not resolve (see
A1-2).** The serial order makes each write safe: Task 3's region is
mechanically enumerated (the type-check flags the live-run `JobOutcome`
literals, `smoke.spec.ts:554-574+`), and neither new scenario contains a
`JobOutcome` literal (Run-gate mocks `validate_profile`, apply mocks
`load_profile`), so Task 3's sweep can never touch them; Tasks 4 and 5 each
add exactly one new named test and start from the previous task's commit, so
no write collides. But the names: `test.describe` enumeration shows exactly
ONE batch describe, `"batch view: dry run"` (`smoke.spec.ts:140`), and the
apply test (`:406`) is a test INSIDE it - there is no "batch-view apply
describe". Both scenarios therefore land in the same describe, which serial
order handles fine, but an implementer told to find the "apply describe"
finds none.

**3. Task 1's cuts - PASS, complete and dependency-clean.** Gone from every
required site: the signature (step 1 names the five-parameter form; the
design fence at `064da74` has no cache parameter), the S6 construction
(per-call `IdentifyCache::new()`), the two deleted steps (borrow change,
session cache; steps renumbered 1-7), the files list (`identify.rs` and
`command_integration.rs` removed; recount: 1 create + 7 modify = 8), the
commit pathspec (8 files, matching the list exactly), the must-not-decide
list, and the Interfaces block (now records the non-changes explicitly).
Nothing needed survived the cut: no other task consumed the borrowed
`LiveIdentifier` (Task 2 consumes the migrated CLI `run.rs` only; Task 3's
sweep is `JobOutcome`-keyed), and the strengthened verification bar ("this
task touches NO test file, so any test edit at all is a defect signal") is
correct under the amended D93. The stale-anchor cleanup is right too: the
authoring anchors dropped `LiveIdentifier :231/:493` and `AppState lib.rs:89`
while keeping `load_profile_body :316-320` (still load-bearing for D103).

**4. The two scenarios as the plan states them - D103 PASS, D101 carries a
plan-introduced false claim (A1-1).** All amendment-1 anchors reproduce:
`hasErrors` `BatchView.vue:282`, `tooltip-errors` `:303-304`, `batch-run`
testid `:511`, `.tooltip-errors = Fix every error-severity diagnostic before
running.` `gui-batch.ftl:69`, apply scaffold `smoke.spec.ts:406`, en
`parse-error` template `diagnostics.ftl:6`. The D103 translation is faithful
and its mechanics verify end to end: the `!doc.profile` branch sits in the
apply handler (`BatchView.vue:210-241`), a missed `find` lands in the
`console.error` else-branch so assertion (a) goes red, the branch `return`s
before `applySuggestion` so assertion (b) is real, `installTauriMocks`
returns the recorded-invoke accessor (`:410`), and core's `ParseError`
emitter carries exactly `detail` + `at` (`load.rs:62-67/:71-74`, the I/O arm
with `at: ""` matching the design fixture). The D101 translation is faithful
in substance (mocks constructible: `MKVMERGE_INFO` `smoke.spec.ts:92`, the
dialog-open pattern at `:205/:313`; the title assertion discriminates the
errors reason) - except for the finding below.

**5. Counts - PASS, all re-run.** Spec amendments 7 -> 8 (S-1..S-8; the S-8
fence exists); Task 1 files 10 -> 8 and steps 9 -> 7; Tasks 4 and 5 both
7 -> 8 steps (recounted from the diff); three `smoke.spec.ts` writers (Tasks
3/4/5 files lists); self-review's new counts (8 amendments, 8 observables, 0
producer-less, 2 scenarios) consistent. The self-review's own stale-form grep
(`six parameter|cache: &mut|ident_cache|LiveIdentifier|command_integration`)
reproduces exactly as it claims: only deliberate mentions. My shape-keyed
sweep (`session cache|Arc<Mutex|spawn_blocking|IdentifyCache|borrow`) and a
leftover-phrase sweep (`S-1..S-7|seven spec|uncovered|rides the v1.x|no
producer`) likewise find only deliberate replacement text; no stale "six"
survives in plan or design.

## Findings

### Important

**A1-1. Task 4 step 7 asserts a fire-verification that is empirically
false.** Location: Task 4 step 7's closing sentence: "the Run-gate scenario
is red today by the same evidence (no error-severity document reaches
BatchView in any existing scenario)". The parenthetical is disproven by the
existing pluralization test:

```
$ sed -n '271,278p' e2e/smoke.spec.ts     # pluralReport's first diagnostic
  const pluralReport: ReportDocument = {
    config_diagnostics: [
      { code: "unsupported-source", severity: "error", ... }
$ sed -n '310,323p' e2e/smoke.spec.ts     # the :307 test feeds it to BatchView
        dry_run: [resolveWith(pluralReport)],
    ...
    const batch = page.getByTestId("view-batch");
    ... pick -> dry-run ...
```

An error-severity `config_diagnostics` document reaches BatchView today, and
after that dry-run the Run button IS disabled - merely unasserted. The
"red today" conclusion is also unsupported: the scenario's assertions
exercise gate mechanics that predate Task 4 against a mocked document, so
run on today's tree it would pass. This sentence is plan-introduced: the
amended design carries no such claim (it dropped the old coverage paragraph
wholesale). What IS true, and what the fix should say: no existing test
ASSERTS `batch-run` disabled (`grep -n "toBeDisabled" e2e/*.spec.ts` returns
only editor-save/remove sites, `smoke.spec.ts:1241`,
`editor-rule-add-remove.spec.ts:151/:317`), so the scenario adds real
assertion coverage, and its red state is a regression of the gate - the
framing the plan's own acceptance row 5 and the design already use. Replace
the sentence; do not keep any "red today" claim for this scenario.

### Minor

**A1-2. The two named `smoke.spec.ts` regions do not resolve on the tree.**
Task 5's files list and step 5 say "the batch-view apply describe" - no such
describe exists; the apply test (`:406`) is a test inside the single
`"batch view: dry run"` describe (`:140`). Task 4's files list says "in the
batch-view describe, beside the existing enabled assertion" - the enabled
assertion (`toBeEnabled` at `:511`; the plan's anchor line cites the `:510`
locator line) sits in the `"jobs view: live run"` describe's first test, not
in the batch-view describe, so "beside" mislocates it. The imprecise describe
names originate in the design's producer paragraphs (out of scope here), but
the plan owns executable precision: name the real region - e.g. Task 4 "a
new test in the `batch view: dry run` describe (`:140`)", Task 5 "a new test
in the same describe, following the `:406` apply test" - and drop "beside
the existing enabled assertion". Ownership stays unambiguous at test
granularity either way (serial order, one named test per task), which is why
this is Minor.

## New-defect scan beyond the findings

The Global-Constraints scenario boundary, the acceptance map's "No observable
is producer-less" paragraph, the close-action ROADMAP bookkeeping change, and
the amendment-1 note are mutually consistent; Task 4/5's commit blocks stage
`e2e/smoke.spec.ts` and their messages name the scenarios; Task 3's region
needs no own-entry note (its edits precede both scenarios and are
type-check-enumerated). The design-side count fix (`064da74`: five explicit
parameters) is in and no stale count survives.

---

# Amendment 1, fix delta (same reviewer, fourth pass)

Reviewed 2026-07-28: plan commit `0997eba` diffed against `5d05560` in full.
The fix commit touches only the plan (6 hunks) and `docs/decision-ledger.yaml`
(+14 lines: the new controller-written entry
`a-new-test-says-whether-the-behavior-or-the-assertion-is-new`, ledger
`:4607`, whose statement and A1-1-citing occurrence I read and which matches
the plan's two citations of it). Fresh scratch at
`/tmp/plan9-a1fix-delta-p8w3/`; every empirical claim in the rewritten text
re-run this pass. Narrow scope per the routing: the two findings, their
ripple, and the author's Task-5 parallel check - coverage, Task 1's cuts and
the counts were not re-run.

## Verdict: APPROVED

## Per-finding verdicts

**A1-1 - CLOSED.** Step 7's red-today sentence is gone and replaced by an
explicit disclaimer ("carries NO red-today claim: it passes on today's
tree"). Step 5's three statements are all TRUE against the tree:

- *Assertion new, not behavior:* `hasErrors` gating exists today
  (`BatchView.vue:282/:303-304`); `pluralReport` carries
  `severity: "error"` (measured at `:274`, inside the plan's cited
  `:272-277` object range) and is fed to BatchView via
  `dry_run: [resolveWith(pluralReport)]` at `:315`; no test anywhere asserts
  `batch-run` disabled (`grep -rn "toBeDisabled" e2e/*.spec.ts` -> smoke
  `:1241` saveButton, editor-rule-add-remove `:151` remove / `:317`
  editor-save - none targets `batch-run`).
- *Passes on today's tree:* follows from the above; the mock supplies the
  document, the gate mechanics predate Task 4.
- *The two-link chain closes the consequence rather than papering it:* link 1
  (core/CLI tests: bare `raw:` -> error severity) is genuinely new behavior
  and red today - my round-1 binary probe measured exit 0 with an info
  diagnostic on the identical profile; link 2 (error-severity document ->
  gate disabled) is existing behavior newly asserted. The links are separate
  by construction (the mock hand-supplies the severity), and stating that
  names the seam honestly - the only uncovered composition is the real-IPC
  integration, which is exactly the OUT-ruled `start_run`/`tauri::test`
  territory this plan may not enter. No red-today claim survives anywhere in
  Task 4 attached to the scenario; the remaining red-today claims in the plan
  (`:64` parity fixture, `:296` link-1 core/CLI tests) are ones my probes
  verified true.

**A1-2 - CLOSED.** Both files-list entries and both step 5s now name the
`batch view: dry run` describe (`:140`), Task 5 additionally "beside the
apply-flow test (`:406`)", and the enabled-assertion pairing is restated
by-assertion with the correct split (`toBeEnabled` `:511`, locator `:510`)
and its true location named (`jobs view: live run` describe `:477`, test
`:491`). Every name and line resolves on the tree; the rewritten
amendment-anchors bullet's new region facts all reproduce (single batch
describe `:140`, `:307`/`:406` inside it, next describe `:477`).

**Task-5 parallel statement (coordinator's third item) - CONFIRMED, and the
new text is TRUE.** The apply scenario passes on today's tree: the current
`config_diagnostics[0]` fetch surfaces the same singleton `parse-error` the
code-keyed `find` would (`BatchView.vue:225` is the file's only such read),
and the `!doc.profile` branch returns before `applySuggestion`/`save_profile`
(branch head at `:218-227` as the plan cites; the return sits at the branch
end, verified in my amendment-1 pass). The retained discriminator claim ("red
if the new `find` misses `parse-error`") correctly targets a defective
rewrite - a missed find lands in the `console.error` else-branch and
assertion (a) fails - and the behavior-vs-assertion statement mirrors Task
4's, per the new ledger entry both steps cite.

## Observation (no finding)

Step 5's parenthetical "the only `toBeDisabled` in the suite is the editor
Save, `:1241`" is true under the file-scoped reading its smoke `:1241` anchor
implies, but "the suite" read as the whole e2e suite would also have to name
`editor-rule-add-remove.spec.ts:151/:317`. The load-bearing claim ("nothing
anywhere asserts `batch-run` disabled") is unambiguously true either way; a
future edit could say "in this file" for precision. Not gating.

---

# Amendment 2: delta review (same reviewer, fifth pass)

Reviewed 2026-07-28: plan commit `0033146` diffed against `0997eba` in full
(the fix commit touches only the plan, +27/-9). Fresh scratch at
`/tmp/plan9-a2-delta-v5r8/`; every measurement below re-run this pass. Narrow
scope per the routing: the hoist addition and its ripple only.

## Verdict: NEEDS FIXES

The hoist itself is sound on all five briefed items - pure move confirmed,
regions disjoint, spans and imports exact, both absence-check halves
fire-verified, the no-consume claim properly hedged. One Important finding:
the plan's stated justification ("this task creates the FOURTH e2e spec
file, which is exactly the trigger's firing condition") is false by direct
measurement, in the same false-premise class as A1-1. Wording-only fix; no
task work changes in any branch.

## Per-item verdicts

**1. Pure move - PASS.** The three function bodies are textually identical
(read side by side: `smoke.spec.ts:60-62`, `editor-markers.spec.ts:29-31`,
`editor-rule-add-remove.spec.ts:41-43` - same signature
`(id: string, args?: Record<string, FluentVariable>)`, same return
`{ name: en(id, args), exact: true }`); only the doc comments differ (smoke
carries the 7-line substring-collision rationale `:53-59`, the other two are
2-line mirrors), which the plan handles by moving smoke's verbatim. The
plan's export line matches the copies token for token. The target module
already has `en()` at `i18n-en.ts:155` and `FluentVariable` imported at
`:61`. Call sites keep resolving: 67/8/6 `name(` uses in the three files,
each file already imports from `./i18n-en` (`smoke:24`, `markers:23`,
`rule-add-remove:35`), and no shadowing symbol exists (`const name|let
name|import { name|as name` over `e2e/*.spec.ts` returns nothing; control:
the same grep shape finds the helper definitions).

**2. Placement - PASS.** Task 6 is the fourth and LAST smoke-writer; its
smoke region (the `:53-62` deletion plus the `:24` import line, both at the
file top) is disjoint from Task 3's literals (`:554+`, live-run describe),
and Tasks 4/5's new tests (inside the `:140` describe, all below `:62`).
Serial order plus content-locating ("locate by the `function name(` line")
makes the write safe even against the line shifts Tasks 4/5 introduce.

**3. Deletion spans and import surgery - PASS, re-measured.** Boundaries
checked on both sides of each span: smoke `:52` blank / `:53-59` comment /
`:60-62` function / `:63` blank (the `:64` `visibleText` comment survives);
markers `:26` blank / `:27-28` comment / `:29-31` function / `:32` blank
(`MKVMERGE_INFO` survives); rule-add-remove `:38` blank / `:39-40` comment /
`:41-43` function (same neighbour). No span swallows a neighbouring
declaration. `FluentVariable` has exactly two hits per file (import
`smoke:25`/`markers:21`/`rule-add-remove:31` + the helper signature), so the
type import genuinely goes unused on deletion; `help-mode.spec.ts` has zero
hits and is correctly untouched.

**4. The absence check - PASS, both halves fire-verified by my own runs.**
Red today: `grep -rn "^function name(" e2e/*.spec.ts` returns exactly the
three copies (`editor-markers.spec.ts:29`, `editor-rule-add-remove.spec.ts:41`,
`smoke.spec.ts:60`). Green reachable: the survivor is excluded twice over -
the post-hoist line reads `export function name(`, which the `^function`
anchor cannot match, and `i18n-en.ts` is outside the `*.spec.ts` glob; the
presence control `grep -c "export function name(" e2e/i18n-en.ts` measures 0
today (its red state) with the pattern shape proven live by the file's three
existing `^export function` lines (`:125`, `:155`, `:170`). Not
green-unreachable.

**5. The no-consume claim - PASS as hedged.** `data-testid="cancel-batch"`
verified at `src/views/JobsView.vue:263`; `job-panic` is created by Task 3
(D100 fence). The claim is stated as expectation plus a closed fallback (if
code contact needs a localized accessible-name match, import the SHARED
export; a local copy is a defect in every branch), so no latitude opens
even if the expectation fails.

## Findings

### Important

**A2-1. The "FOURTH e2e spec file" premise is false by direct measurement,
and the trigger's registered condition has not literally fired.** The plan
states it three times: Task 6 Step 3's intro ("this task creates the FOURTH
e2e spec file, which is exactly the trigger's firing condition"), the
sequencing bullet ("the fourth spec file is created there"), and the
amendment-2 note ("the FOURTH e2e spec file, the exact firing condition").
Measured:

```
$ ls e2e/*.spec.ts
catalogs  editor-dropdowns  editor-markers  editor-rule-add-remove
editor-tooltips  help-mode  help-topics  locale-switch  smoke     (9 files)
```

`jobsview-reset.spec.ts` will be the TENTH spec file, not the fourth. Nor is
file count the registered condition: the trigger (`docs/ROADMAP.md:628-632`)
reads "A FOURTH e2e spec file NEEDS the local `name()` helper ... the fourth
COPY is where the pattern stops paying" - the condition is a fourth CONSUMER,
and the plan's own Step 3 states the new spec "does NOT consume the helper
... no fourth copy is created and no fourth consumer either". The plan
therefore simultaneously claims the firing condition is met and describes
why it is not - jointly incoherent; under either reading at least one
statement is false. What is true and should replace the premise at all three
sites (plus the close action's "fourth-e2e-spec-file" label): the owner
ruled the hoist done NOW as an early consumption of the registered trigger
while Task 6 already touches every affected file - the action is owner-ruled
and stands in every branch; only the recorded rationale is wrong. Ripple to
route to the controller (house file, outside the plan): the ROADMAP's own
"FIRED AND CONSUMED" note (`:633-641`) repeats the same "the fourth spec
file" mis-transcription and should be corrected in the same sweep, since it
is the permanent trigger record. Even the generous "mock-harness family"
reading fails the count: five spec files already use
`installTauriMocks`/`installMockIPC` (smoke, editor-markers,
editor-rule-add-remove, locale-switch, help-mode).

## Ripple noted

The fix commit also upgraded the amendment-1 "in the suite" parenthetical
(my fourth-pass observation) to the precise two-scope form - smoke-only
`:1241`, suite-wide three adding `editor-rule-add-remove.spec.ts:151/:317` -
which my whole-suite grep confirms true. The amendment-2 anchors bullet's
measurements all reproduce (trigger line at `:628`, copies at
`:53-62`/`:27-31`/`:39-43`, two `FluentVariable` hits per file, three
`^function name(` hits, `export function name(` count 0 today).

---

# A2-1 closure (same reviewer, sixth pass)

Reviewed 2026-07-28: plan commit `629dc64` diffed against `0033146` (the fix
touches only the plan; the ROADMAP and ledger corrections came in the
controller's `99b557b`). Fresh scratch at `/tmp/plan9-a21close-q4n6/`.
Narrowest scope per the routing: the two checks only; the hoist mechanics
cleared in the fifth pass are untouched by this diff.

## Verdict: A2-1 CLOSED - overall APPROVED

**Check 1 - no fired-trigger or file-count justification survives.** I
grepped `fourth|firing|fired` (case-insensitive) over the plan and read all
ten hits in context. Every surviving "FOURTH" is either the trigger's quoted
registered title (`:367`, `:451`), the corrected condition stated as UNMET
("a fourth spec file that NEEDS the helper, is not met", `:390`, `:434`), or
the no-consumer statement (`:394`, `:454`); every "fired/firing" is a
negation ("never fired", "not on a fired trigger", "NOT FIRED"), part of the
corrected-error record, or the unrelated grep-control idiom at `:62`. The
seven rewritten sites all now justify the hoist by the owner's
early-consumption ruling. The claims in the rewritten text reproduce:
`ls e2e/*.spec.ts | wc -l` -> 9 (tenth file), the ROADMAP trigger line now
carries the NOT FIRED / CONSUMED EARLY record with the controller's own
first-note correction and the copies-vs-files clarification (read at
`:628-643`), and `a-triggers-condition-is-read-to-its-last-clause` exists at
ledger `:4621` exactly as the plan cites.

**Check 2 - the placement reasoning stands on its own.** The sequencing
sentence (`:112`) now rests on exactly two trigger-independent facts: Task 6
creates the new spec file whose arrival prompted the ruling (true - Task 6's
Create list; a statement of provenance, not of firing), and Task 6 is the
last smoke-writer slot so the move cannot race Tasks 3-5's committed regions
(verified structurally in the fifth pass: serial order, disjoint
content-located regions). Each fact carries only the load it can: the first
explains why the work sits in this plan and task, the second why the slot is
safe. Neither leans on the trigger, and the sentence says so explicitly.

No new defect in the changed text; the one unverifiable clause (the owner's
"right moment" rationale at `:390`) is presented as the ruling's substance,
consistent with the routing, and decides nothing an implementer executes.
