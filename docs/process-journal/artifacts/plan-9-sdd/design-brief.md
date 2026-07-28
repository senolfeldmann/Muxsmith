# Plan 9 design brief

Controller-authored brief for the Plan 9 design document. Written 2026-07-28
(session 24) after the owner's scope kickoff. You are the design's AUTHOR; a
separate independent reviewer grades this document against this brief before
the owner sees it, and a fix loop runs until it is approved.

## 1. Deliverable

`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`

House shape, same as `2026-07-22-plan8-packaging-release-design.md` (read it
as the form reference, not for content): a status line, a scope paragraph
naming what is binding, a grounding paragraph naming every ground-truth
artifact, then one `## Dn` section per decision, then the closing sections
listed in section 7 below.

**ADR numbering starts at D91.** D90 is the last ADR in use (plan-8 design);
the only `D91` string in the tree is a hypothetical mention in
`docs/process-journal/artifacts/plan-8.5-sdd/plan-amendment-verdict.md:60`
and reserves nothing. Verify that before you number, and say in the status
line what you verified.

Write the file. Do NOT run any git command: the controller commits.

## 2. Ground truth, exhaustively enumerated

This list is exhaustive, not exemplary. If you need a source that is not on
it, that is a NEEDS_CONTEXT return, not a judgement call.

1. **The v1 spec**, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
   - authoritative above this design on any conflict. Flag conflicts, never
   improvise around them. Sections you will certainly touch: 5.2 (diagnostic
   severity table), 5.5 and 7 (planner seam / parity), 6 and 7 (core's
   prose-free rule and presentation-time rendering), 9.2 (identification
   pinning) if the cache decision reaches it.
2. **The ROADMAP Plan 9 anchor**, `docs/ROADMAP.md`, including its RECON
   block and the IN/OUT lists ruled by the owner on 2026-07-28 (commit
   `90bc3ae`). The eight IN rulings are binding inputs (section 3).
3. **The recon inventory**, `.superpowers/sdd/plan-9/recon-inventory.md`
   (1119 lines, produced 2026-07-27 against `c2514e7`). It is read-only
   reconnaissance: measured spans, a step-by-step comparison of the four
   pipeline copies, the divergence classification, and per-input OPEN
   QUESTIONS. Its line references were verified by content at the time of
   writing, against a commit two commits behind current master - re-verify
   any anchor you make load-bearing, and say which ones you re-verified.
4. **The five ledger entries the rulings touch**, in
   `docs/decision-ledger.yaml`: `core-121-planner-seam-and-hoist`,
   `exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`,
   `cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`, plus
   the new `empty-bare-raw-property-rejected-at-validate`. Each carries the
   ruled position in its `statement` and the losing argument in its
   `steelman`; do not re-litigate either.
5. **The Tier-2 nature files as review ground truth alongside the spec**:
   `docs/conventions.yaml`, `docs/process-conventions.yaml`,
   `docs/product-boundaries.yaml`. Conform to them; where you establish a new
   pattern or deviate deliberately, SURFACE it in the document rather than
   resolving it silently.
6. **The source of record for every claim about the tree**: the files the
   recon's appendix enumerates. Read them; do not take a shape from the recon
   without opening the file when the decision rests on it.
7. **SI-3 parity sources**: the mkvtoolnix source at `~/Downloads/mkvtoolnix`
   (cite by symbol anchor plus tag, not bare line numbers, per
   `code-comment-line-citations-drift`) and the installed `mkvmerge v100.0`,
   run to confirm behavior rather than recalled. Section 6 names where parity
   is actually meaningful here.

## 3. Binding rulings, not re-litigated

The owner ruled these at the kickoff. Each becomes a `Dn` section that
designs its MECHANICS; none is reopened, and none is weakened by a "unless
the design finds otherwise" clause.

1. **The four-copy pipeline is hoisted** into a shared core
   `plan_pipeline()`, which IS the injectable planner seam. The migration of
   the four still-inlined `validate::validate` + `lint::provable_overlaps`
   pairs onto the existing `profile::validate::config_diagnostics` rides with
   it (recon A-1: the Plan-5.6 hoist was half done).
2. **`run_batch` is hoisted** into `muxsmith_core::executor`.
3. **Runs-root: the src-tauri copy is DELETED, not hoisted to core.** The CLI
   gate stays exactly as it is.
4. **The worker-panic path is one item**: the payload travels in the
   `JobOutcome` and the SURFACES render it; there is **no logging facade in
   core**. Rendering scope: CLI human output renders the catalog message in
   place of today's `n/a`, the GUI job row carries the code in its failure
   state, the RunHistory export stays raw-output-only.
5. **Bare `raw:` with an empty property name is an ERROR at validate, under
   its own new DiagCode** - not a warning, not reused `UnknownProperty`. The
   exit-code change for profiles that pass today is accepted.
6. **`config_diagnostics` sorts errors-first CENTRALLY in core's two document
   builders**, not per caller; `BatchView`'s index-0 parse-failure detection
   moves to code-keyed detection in the same change.
7. **The D23 item is a test plus a ledger entry, not a code fix.** The
   deviation was already adjudicated by the plan-5 whole-branch review's
   round 2 and is correct; the mount-glob widening for `JobsView.vue` is the
   only test-harness work in scope.
8. **The D49 G1/G2 removal experiment runs in this plan** (its trigger fired
   with it).

## 4. The forks this design MUST close

Every one of these gets a decision with its rationale and its rejected
alternatives. A fork left open, or handed to an implementer as a choice, is a
defect the reviewer will find. Where a fork's answer changes a serialized
field or a public interface, say so explicitly in that section - those are
memo-recorded at decision time by house rule.

**Seam shape.**

1. `plan_pipeline`'s signature, parameters, and return type; which crate and
   module it lives in; what the four call sites become.
2. Each of the recon's SEVEN deliberate divergences (D-1 to D-7) is
   individually resolved as "becomes parameter X" or "stays per-surface
   because Y". All seven, by name. A hoist that flattens one of them silently
   changes behavior.
3. **D-2 is the sharp one**: `mkvmerge_found: Some(false)` today means
   "absent from PATH" on the CLI surfaces and "absent, or present but too
   old" on the GUI surfaces, in the same field of the same shared document.
   One shared pipeline must pick one meaning or carry the resolver's failure
   reason as data. This is a wire-contract decision.
4. `IdentifyCache` ownership: the seam owns one per call (today's behavior,
   which re-identifies every file when a GUI dry-run is followed by a run) or
   takes one from the caller. Name the cost you accept.
5. Where the `specs` gate lives: duplicated orchestration today (copies 2 and
   4 only), arguably planner output, since `batch.files` already carries
   `plan: None` for error-severity files.
6. The `PathBuf::from(".")` source default: kept, required from the caller, or
   parameterized. `src-tauri/src/lib.rs` documents it as questionable for a
   bundled app; `plan_run` has the same fallback without the note. If the
   resulting GUI behavior changes at all, say what a user would observe.
7. `run_batch`'s hoist boundary: does the shared function end where today's
   `run_batch` ends, or absorb the `run_document` + logger-finish pair both
   surfaces duplicate in different shapes (`finalize_joblog` folds it into a
   `JoblogStatus`; the CLI folds it into two messages)? `TeardownGuard` and
   `fail_fast` are caller-side facts to confirm, not to move silently.

**Worker-panic path.**

8. How the payload travels: a new typed field on `JobOutcome`, a
   discriminated entry in today's untyped `Vec<String>`, or another shape.
   `errors` is mixed today - catalog tokens and raw mkvmerge prose with no
   discriminator beyond a string prefix a test already relies on.
9. What happens to `delete_partial_failed`, the SECOND code-shaped token in
   the same vector. `exec-37` never mentioned it; the recon flags it. It is in
   scope for a decision, even if the decision is "unchanged, because Z".
10. The CLI human rendering: which message, with which params, replacing the
    `n/a` code today. The `worker-panicked` Fluent message already exists in
    both locales - state whether it is reused as is or needs new params, and
    if params change, write both locales' final text verbatim.
11. The GUI carrier: `JobRowData` and `JobRow.vue` both drop `errors` before
    rendering. Name the field, the render site, and the exact user-visible
    result. No new user-visible string may be invented by an implementer, so
    any new key's en and de text is written out here verbatim.
12. Whether core's `eprintln!` removal changes what `recover_panicked_worker`
    returns, and what the spec's §6/§7 prose-free rule then says (the current
    code comment cites that rule as its licence).

**Validation and ordering.**

13. The new DiagCode: its variant name, its catalog key, its severity row in
    spec 5.2, its `catalog_completeness` fixture row, and its en + de Fluent
    text verbatim.
14. Whether the fix covers only the `exact` arm (`validate.rs:268`, which
    `continue`s before any existence check) or the substring/regex arm
    (`:310`) as well, and what happens at match time and at planning time
    (`matcher.rs`, `planner.rs:796` inserts the empty name into the
    capability-warning set).
15. The sort's exact site and stability: the two document builders, or
    `rendered_diags` inside them; and what "stable within a severity" means
    for the vector's collection order.
16. `BatchView`'s replacement detection: which code identifies a parse
    failure, and why that is exhaustive for the cases the current index-0
    read covered.

**Tests and the experiment.**

17. The mount-glob widening: which glob entries, whether an IPC mock is
    installed (today `e2e/mount.ts` installs none, deliberately), and which
    of the five orderings that round-2 verdict traced the new JobsView test
    asserts.
18. The D49 experiment protocol: the exact mutation, the exact suite
    invocation, the decision rule (all three guards fail -> they stay for
    good; only G3 fails -> G1/G2 become removal candidates), and where the
    result is recorded. Per `proc-proposed-safeguard-stays` the guards stay
    until the run measures them redundant - the experiment measures, it does
    not argue.
19. The runs-root deletion's blast radius: `resolve_runs_root` has THREE GUI
    call sites (`plan_run`, `list_runs`, `get_job_log`). Say what each calls
    instead and what debug-build behavior they lose, with the evidence that
    nothing consumes it.

## 5. What this design must NOT do

- **No design-latitude clause, in either form.** Not an explicit permission
  ("the implementer may choose", "either approach works"), and not an
  omission: an unenumerated set in a normative position, a list ending in
  "...", a placeholder, a "one per X" without the X list. Test every
  normative sentence with "must the implementer invent something it is not
  allowed to invent?"
- **No work on the two OUT items.** No Vitest, no `tauri::test`, no
  `mock_builder`, no `src-tauri/tests/` directory, no IpcError render funnel.
  The mount-glob widening in fork 17 is the only harness change in scope, and
  it is not a step toward the 1.x harness decision.
- **No new runtime or product dependency**, cargo or npm.
- No product-boundary change, no release/tag/publish action, no README
  `placeholder(1.0)` resolution.
- No plan or task structure: sequencing, task boundaries and stream layout
  belong to the plan that follows this document, authored separately.
- **Do not drop a safeguard you or the reviewer proposed** during the design
  rounds. It is removed only after it is built and measured redundant, never
  by agreement in the design phase.
- **Where a passage concludes that a guard, an enumeration or a check is
  unnecessary, verify the claim that makes it unnecessary** - run it, do not
  weigh it. This shape ("so we need no X", "X cannot happen here", "the work
  already exists") has produced false claims in this project's design rounds
  before.

## 6. SI-3 parity, where it is actually meaningful

A blanket "compare wherever meaningful" would be latitude. Here are the
comparisons this document owes, and they are the complete set:

1. **Per-job failure reporting.** mkvtoolnix-gui's job queue surfaces per-job
   errors and warnings to the user; Muxsmith's GUI job row currently shows a
   state chip and a warning count and drops `errors`. Compare and classify
   (match / justified divergence / genuine gap) before deciding forks 10-11.
2. **Diagnostic ordering in output.** How mkvmerge orders warnings and errors
   in its own output, as evidence for or against errors-first as the shared
   order (fork 15).
3. **Nothing else.** The hoist itself, the planner seam, the runs-root debug
   gate and the `raw:` opt-in have no mkvtoolnix analogue: mkvmerge is a
   single-shot binary with no declarative profile, and `raw:` is a
   Muxsmith-only construct. State that explicitly rather than leaving the
   reader to wonder whether parity was consulted.

Licensing boundary: behavior, facts and interfaces are fair game; literal
code or text passages are never taken; a deliberately modeled wording is
recorded as an explicit decision.

## 7. Closing sections the document must carry

- **Spec amendments**: every spec section this design changes, with the exact
  replacement text, plus a sweep of the spec for self-contradictions the
  amendment creates (a spec amendment has contradicted a neighbouring section
  in this project before).
- **Triggers created**: each as `<observable event> -> <action>`, for
  mirroring into the ROADMAP Triggers section in the same change. An event
  someone has to notice is not a trigger.
- **What the implementer must not decide**: the explicit list, as plan-8's
  design section 11 did.
- **Acceptance observables**: for each ruled item, what is observably true
  when it is built, and which emitter produces that observable. An acceptance
  item whose observable has no producer is not acceptance.
- **Amendment log**: empty at first draft, appended per fix round.

## 8. House rules that bind the document itself

- **Counts are recomputed from the enumeration they summarize.** The recon
  corrected this anchor's own "~100 lines" to a measured 260/199 - do not
  re-import a figure from prose.
- **A verification step whose expected result is an absence is
  fire-verified**: break it deliberately, watch it fire, restore, and say so.
- **Empirical claims are reproducible**: name the command and its output, do
  not assert behavior from reading.
- **Citations**: prefer primary artifacts (source file, spec, ledger,
  journal) over the recon path. The recon lives in `.superpowers/`, which is
  git-ignored and moves at the plan close, so where the recon itself is the
  carrier of an argument, quote its evidence INLINE rather than pointing at
  it - the house rule from the plan-7 round-8 adjudication.
- **Typography**: ASCII hyphens, straight quotes, no Unicode ellipsis,
  everywhere including code comments and YAML.
- Subagents never call session-relocation tools; worktrees are plain
  directories; use absolute paths.

## 9. Refuting this brief is a valid completion

If a premise in this brief is wrong - a claim about the tree, a fork that is
already closed elsewhere, a ruling that contradicts the spec - refute it with
evidence and say so in the document. That has been the highest-value
correction path in this project: brief defects are the one class four-eyes
does not cover by construction, and every one found so far was found by the
agent receiving the brief. Do not resolve a discovered fork at the keyboard:
return NEEDS_CONTEXT with a decision memo (the options, their costs against
the named invariants, a recommendation) and let the controller route it.
