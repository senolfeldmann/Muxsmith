# Task 3 review brief - Plan 9

**Role:** independent reviewer of Plan 9, Task 3 (the worker-panic payload end
to end: D98, D99, D100, D96's amendment-3 rustdoc rider; spec S-1's
WorkerPanicked row and S-2). You did not write this code. Model tier: mid
(dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus your final message, precise enough that a fix round needs no re-derivation.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Repo `/home/senol/Git/Muxsmith`, `master`, main worktree, clean
  tree; the task's commit is `9e5e112` (16 files). **Read the files, not a
  hash** - house commits land between dispatches.
- Absolute paths, **foreground runs only**.
- **Independent instruments.** Any empirical claim of the implementer's that
  carries a conclusion, you reproduce with your own extraction, your own
  fixture, your own grep, at a path it could not have written. Your scratch
  root: `/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t3rev-independent/`
  (create it). Never re-run its script; never a shared default path.
- Any check whose pass is an absence needs its own fire. Every number and every
  quotation in your verdict is measured or copied, never recalled.
- A shell caution this task hit: a bare `cp` here is aliased interactive and
  blocks on overwrite. If you mutate-and-restore anything, use a
  non-interactive form (`command cp -f`, `git checkout --`) and end with an
  explicit `diff -q` or `git status` proof of restoration.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (v1 spec; this
   task amended two of its passages, S-1's WorkerPanicked row and S-2).
2. The Plan-9 design: **D98** (field, wire memo, forks 9 and 12), **D99** (the
   four Fluent fences, catalog obligations, both rejected alternatives),
   **D100** (render-site fence, scope boundaries), **D96's amendment-3 rider**
   (the replacement `run_batch` doc fence), section 0 notes 4 and 5, section 2's
   "stay discarded" paragraph, section 5, section 7 item 4, and the
   `## Amendment log`.
3. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` -
   Global Constraints and **Task 3** (Files list, Steps 1-11, "Must not
   decide"). Note the plan was amended mid-execution: Task 3 has ELEVEN steps
   and Step 2 is the rustdoc restatement.
4. The four house-knowledge YAMLs. Cite ids, re-verify any `:line`.

The implementer's brief (`task-3-brief.md`) and report (`task-3-report.md`) are
evidence, not ground truth.

## Dimensions, each reported on explicitly

1. **Contract compliance.** D98's field shape and always-serialized form; the
   byte-identical `errors` token; `delete_partial_failed` untouched; the four
   D99 Fluent texts and D100's render semantics character for character; the
   catalog obligations complete; S-1 (WorkerPanicked row only - the
   EmptyRawProperty row belongs to Task 4) and S-2 as the design's fences write
   them.
2. **The Step-2 transcription.** Extract the installed `///` block above
   `pub fn run_batch` and the rider's fence yourself and compare them; also
   check the rest of the function is byte-identical to its pre-state. Build
   your own extractor.
3. **The compiler-driven sweep's completeness.** The plan says the compiler
   enumerates the `JobOutcome` constructor set. Verify the set is complete NOW
   (a constructor the compiler could not reach - behind a cfg, in an unbuilt
   target - would be missed), and that `recover_panicked_worker` is the only
   `Some`-setter outside tests.
4. **Test integrity.** No assertion weakened, deleted, skipped or reworded. Two
   existing serialization expectations were EXTENDED (concern 1) - check that
   every previously asserted value survives byte-identical and the assertions
   are strictly stronger.
5. **Latitude, both forms**, including the inverse: did the implementer resolve
   at the keyboard something that should have returned as NEEDS_CONTEXT? Its
   own concerns 1-4 are where to look hardest.
6. **House dimension**: Tier-2 conformance, including the amended
   `latitude-carveout-zero-content-structural-forks` (the file-vs-within-file
   boundary) and `core-docs-name-callers-illustratively-never-exclusively`. The
   `recover_panicked_worker` licence-doc rewrite is composed prose with no
   fence - grade its content against D98 fork 12 and its form against the
   module's neighbours.
7. **The no-work-needed check (standing).** Wherever the report or a comment
   concludes something is unnecessary, unobserved, already covered or
   structurally true - run the premise, do not weigh it.
8. **Verification quality.** Re-run the task's bar yourself: the acceptance-4
   emitters, the core-stdio absence check with your own fire and control,
   `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `pnpm lint`, `pnpm build`, `pnpm check:i18n`,
   `pnpm test:e2e`, and no insta churn. Recompute every aggregate the report
   states.

## Adjudication questions (one explicit verdict each, not pre-rated)

1. **Extending two existing serialization expectations.**
   `report_json.rs`'s exact-JSON equality and `executor_events.rs`'s exact
   serialized-string equality went red once the field was on the wire; the
   implementer added `"panic": null` / `,"panic":null` to the expected values
   rather than returning NEEDS_CONTEXT, arguing the files are Files-list members
   through the EXEMPLARY compiler entry, that the edits are strictly additive,
   and that D98's always-on-the-wire mandate left exactly one consistent
   action. Was that in scope - or a fixture/assertion change that should have
   been routed?
2. **`render_finished` now returns `Vec<String>` instead of `String`.** The
   implementer reads D99 as requiring both the branch inside `render_finished`
   and two rendered lines, which one `String` cannot express. Correct reading of
   D99 - or an interface change D99 did not sanction?
3. **`run-job-panicked` joined an existing fixture arm** in
   `catalog_completeness.rs` (the arm already producing exactly D99's prescribed
   args for two other keys) instead of getting its own. House-pattern
   conformance, or does D99's obligations list want the key's fence textually
   visible at its own arm?
4. **Field position.** D98 fixes everything about the field except where it
   sits; it was placed last in `JobOutcome` and after `duration_ms` in
   `JobRecord`/`ipc.ts`, changing serialized key order. Right call - or does
   something in the spec, the design or a consumer constrain the position?

## Required ruling: the persisted half of acceptance 4

The implementer surfaced this rather than acting on it, and it is the most
consequential item in the report. Design acceptance 4 names as user-observable
that the run's `job-<index>.json` contains `"panic": "<payload>"`. The field is
now structurally on the record, but **no test asserts the persisted key** -
`crates/muxsmith-core/tests/joblog.rs` asserts every other `JobRecord` key
individually and gained no `panic` line. The implementer did not add one
because the plan's Step 7 enumerates the pinned tests, and an enumeration in a
normative position wins over the structural grant.

Rule on it, with the owner's standing ruling in view (Tier-2
`tests-ship-with-the-feature-never-after`: a feature's tests ship with the
feature, not in a later package):

- Does the persisted half of acceptance 4 need an assertion in THIS task?
- If yes, name the exact addition and its site, to the line, so a fix round can
  execute it without re-deriving anything - and say explicitly whether it is
  additive-only (no existing assertion touched).
- If no, say what makes the persisted claim verified without it.

Verify the premise before ruling: check yourself whether `joblog.rs` really
asserts every other key and really has no `panic` assertion, and whether any
other test in the tree covers it.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-3-verdict.md` and
make your final message the same content:

1. **Verdict**: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
2. **Findings**, numbered, severity-tagged (BLOCKING / MEDIUM / LOW), each with
   file:line, the evidence you ran, and exactly what must change.
3. **The four adjudication verdicts** and **the required ruling** above.
4. **Evidence appendix**, your instrument paths named.
5. **HARVEST**: observed patterns, repeated rejections, over-restriction
   findings if any, and what Tasks 4-7 must carry - Task 7 in particular, which
   is a mutate-measure-restore experiment and inherits the interactive-`cp`
   hazard this task hit.
