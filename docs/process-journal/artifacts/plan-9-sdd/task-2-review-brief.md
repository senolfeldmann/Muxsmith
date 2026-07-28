# Task 2 review brief - Plan 9

**Role:** independent reviewer of Plan 9, Task 2 (`run_batch` hoisted into
`muxsmith_core::executor::queue`, the src-tauri runs-root seam deleted).
You did not write this code and you are not the implementer's editor: you
grade the artifact against its contract, adjudicate the implementer's five
concerns, and harvest patterns. Model tier: mid (dispatch model: Opus 5).
Effort: xhigh.

**You commit nothing and change no product file.** Your output is a verdict
file plus your final message. If a fix is needed, you name it precisely
enough for a fix dispatch to execute it.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). The repo is `/home/senol/Git/Muxsmith`, branch `master`,
  main worktree, currently at commit `9b2843f` with a clean tree.
- Absolute paths, **foreground runs only**, no background-plus-monitor.
- **Independent instruments, not merely an independent context.** When you
  reproduce an empirical claim of the implementer's, you build your own
  harness - script, fixture, extracted region, scratch copy - **at a path the
  implementer could not have written**, and never at the shared scratch path
  both of you would default to. Your scratch root is
  `/tmp/claude-1000/-home-senol-agents-peter/d901d396-2a64-4eed-a8ac-e7a9673cf07b/scratchpad/t2rev-independent/`
  (create it). A re-run that silently executes the implementer's own
  instrument agrees by construction and is worth nothing.
- A check whose passing result is an absence (an empty grep, a clean exit)
  is only evidence if you have watched it fire once on a case where it must
  hit. Paste both halves.
- Every number and every quotation in your verdict is measured or copied from
  the artifact, never recalled.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - the v1 spec,
   authoritative on conflict.
2. `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md` -
   **D96 and D97 in full**, section 2's "stay discarded" paragraph, section 5's
   `run_batch` and runs-root bullets, section 7 items 2 and 3 (the acceptance
   observables), and the `## Amendment log` at its current state (the log
   binds; the pointer is the contract, not an enumeration of it).
3. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` -
   Global Constraints and the **Task 2** section (Files EXHAUSTIVE, Steps 1-7,
   "Must not decide").
4. `docs/ROADMAP.md`, the Plan-9 anchor (the owner's IN rulings, including
   DELETE-not-hoist for the runs-root seam).
5. The four house-knowledge files as ground truth alongside the above:
   `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`. Cite entries
   by id and re-verify any `:line` before relying on it.

The implementer's brief is `.superpowers/sdd/plan-9/task-2-brief.md` and its
report is `.superpowers/sdd/plan-9/task-2-report.md`. **Both are evidence, not
ground truth** - a brief can be wrong, and a report's pasted output is a claim
until you reproduce it.

## What to review

The diff is `git show 9b2843f` (three files: `crates/muxsmith-core/src/executor/queue.rs`,
`src-tauri/src/run.rs`, `crates/muxsmith-cli/src/commands/run.rs`).

Dimensions, each reported on explicitly:

1. **Contract compliance.** Does the change do exactly what D96/D97 and the
   plan's Task 2 specify - the verbatim move with today's signature, the
   not-absorbed pair (`run_document`, `finalize_joblog`, the CLI's joblog
   messages), `TeardownGuard` and `fail_fast` caller-side, the two named tests
   moved, exactly D97's function plus three call sites deleted/converted, the
   CLI gate untouched? Verify the move's byte-identity yourself, with your own
   extraction; do not re-run the implementer's comparison.
2. **Behavior preservation on the CLI path.** The inline queue block became a
   `run_batch` call with an `on_event` closure. Check the logger tee ORDER
   against the pre-state (tee before `on_event`), that persistence stays
   unconditional under `--json`, that the milestone rendering is unchanged,
   and that nothing about the panic-`expect` path changed. The subprocess
   suites and insta snapshots are the machine half; the reading is yours.
3. **Latitude survival, in both forms.** Any explicit permission left in the
   code/comments, and any omission-shaped fork the implementer had to fill by
   inventing something. Also the inverse: did the implementer resolve at the
   keyboard something that should have returned as NEEDS_CONTEXT?
4. **House dimension.** Conformance to the Tier-2 files, including the ones
   the implementer invokes by name (`testing-support-helpers`, the cross-crate
   doc-link pattern it counted). Verify its count of the dominant doc-link
   form yourself if you rely on it.
5. **Test integrity.** No assertion weakened, deleted, skipped or reworded; the
   moved tests still assert what they asserted in src-tauri; the fixture-value
   change the implementer describes (`spec(0, ...)`, argv `["x"]` -> `["0"]`)
   is genuinely unobserved by any assertion.
6. **The no-work-needed check (standing).** Wherever the report or a code
   comment concludes that something is unnecessary, redundant, unobserved or
   already covered - **run the premise, do not weigh it**. The implementer's
   "verified unobserved" claim about `FakeSpawner::spawned()` is exactly this
   shape.
7. **Verification quality.** Re-run the two acceptance observables (design
   section 7 items 2 and 3) with your own fires, plus `cargo fmt --all --check`,
   `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`, `cargo test -p muxsmith-gui`. Recompute every
   aggregate the report states (it claims 494 tests total, 121 in
   muxsmith-core, 80 in muxsmith-gui) - controller experience is that report
   arithmetic is where errors hide.

## Adjudication questions (required verdicts)

The implementer returned **DONE_WITH_CONCERNS** with five numbered concerns.
Each needs one explicit verdict from you. They are phrased in both directions
deliberately and are **not pre-rated**: a concern is not a defect because it
was raised, and not acceptable because it was disclosed.

1. **The de-linked rustdoc.** The moved `run_batch` doc had
   `` [`finish_teardown`] ``, which names a src-tauri-private function
   unreachable from core, so the link was reduced to a plain code span
   `` `finish_teardown` ``. Is that the correct reading of the plan's "rustdoc
   moved with it" - or was the correct move to keep the link and return the
   rustdoc gate failure as a finding, or to point the link at something that
   resolves?
2. **Shell-specific prose surviving into core.** The moved rustdoc still
   describes the caller's world (`on_event` as "the shell's window-emit", the
   `#[tauri::command]` wrapper and the detached thread, the
   `finish_teardown`/D31 paragraph). Is keeping them right, because the plan
   said move the rustdoc and rewriting is latitude the implementer must not
   take - or is doc prose that names symbols a core reader cannot look up a
   defect that this task should have returned for a ruling?
3. **The repaired doc link at `src-tauri/src/run.rs:4`.** The task's own import
   removal made `` [`run_queue`] `` dangle; the implementer re-pointed it to the
   fully qualified core path. Was that repair in scope under the
   structural-conformance grant (zero outward effect, the file's dominant
   pattern) - or does the exhaustive Files list mean it should have come back
   as a finding instead?
4. **The rewritten CLI comments.** The deletion invalidated "see the drain loop
   below"; the implementer rewrote that comment and its neighbour. In scope for
   the same reason as question 3, or out of scope for the same reason?
5. **The test-helper adaptation.** The moved tests now use the destination
   module's `spec(index, output)` helper, so the fixture argv changes from
   `["x"]` to `["0"]`; the implementer argues `testing-support-helpers`
   forbade adding a second `spec` to the crate, and that no assertion observes
   argv. Is that a legitimate paths/imports adaptation - or a fixture mutation
   the grant explicitly stops, which should have returned?

For 3 and 4 additionally answer the **over-restriction question**, which the
house wants surfaced rather than suppressed: if the grant's boundary forced (or
would have forced) a NEEDS_CONTEXT round-trip whose fork had no real decision
content, say so in your harvest. The Task-1 review already produced an entry in
this area; the boundary is deliberately tight and revises on evidence like this,
so an over-restriction finding is wanted, not second-guessing.

## Harvest (required section)

Report the dominant patterns you observed and any repeated rejection, plus:

- Your independent judgment on the implementer's two surfaced items: (a) the
  claim that the private-module rustdoc blind spot has now fired in two
  consecutive tasks and that the ROADMAP's "three one-line fixes" cost figure
  is stale (it says the true figure is now two, both `` [`run`] `` ambiguity
  errors at `src-tauri/src/lib.rs:54`/`:87`) - **measure it, do not relay it**;
  (b) the proposal that a verbatim cross-crate move should carry a doc-link
  sweep in its file list the way it carries an import sweep.
- Anything the next task (Task 3: the worker-panic path, which edits both
  `queue.rs` and src-tauri `run.rs`) must carry.

You do not write to any house-knowledge YAML; the controller is the single
writer and routes your harvest.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-2-verdict.md` and
make your final message the same content (read as data, not as chat):

1. **Verdict**: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
2. **Findings**, numbered and severity-tagged (BLOCKING / MEDIUM / LOW), each
   with the file:line, the evidence you ran, and what specifically must change.
3. **The five adjudication verdicts**, one per concern, each a clear ruling
   with its reason.
4. **Evidence appendix**: your commands and their pasted output, fires
   included, with your instrument paths named so the controller can see they
   were yours.
5. **HARVEST** as specified above.
