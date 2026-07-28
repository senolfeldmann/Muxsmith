# Plan 9 plan brief

Controller-authored brief for the Plan 9 execution plan. Written 2026-07-28
(session 24), immediately after the owner approved the design. You are the
plan's AUTHOR; a separate independent reviewer grades this plan against this
brief and against the design - running a coverage dimension that walks the
design section by section and names the task implementing each - before the
owner sees it, and a fix loop runs until it is approved.

## 1. Deliverable

`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`

House shape, same as `docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md`
(read it as the form reference, not for content): the agentic-worker header
with the house deviation on progress, Goal, Architecture, Tech Stack, Global
Constraints, Execution method, Model tiers, the sequencing/parallelism
section, the tasks, and the close actions.

Write the file. Do NOT run any git command: the controller commits.

## 2. Ground truth, exhaustively enumerated

1. **The v1 spec**, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` -
   authoritative on conflict, above the design.
2. **The approved design**,
   `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
   (D91-D105, owner-approved 2026-07-28 after a four-eyes loop). Its
   D-entries AND every entry in its `## Amendment log`, at the log's state at
   EXECUTION time, bind this plan. Do not enumerate the log's membership in
   the plan: the pointer is the contract, because an enumeration goes stale
   the moment the log grows (house ruling, plan-8.5 task-2 review round).
   Sections 5 (what the implementer must not decide), 6 (triggers) and 7
   (acceptance observables) are load-bearing for you specifically.
3. **The ROADMAP Plan 9 anchor** (`docs/ROADMAP.md`): the owner's eight IN
   rulings, the two OUT items, the recorded close action about ledger
   promotion, and the harness-scope correction.
4. **The four house-knowledge files** as ground truth alongside the spec:
   `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`. Cite entries
   by id; re-verify any `:line` you attach.
5. **BUILDING.md** for the ten-part gate, verbatim.
6. **The recon inventory**, `.superpowers/sdd/plan-9/recon-inventory.md`, as
   the measured evidence base (spans, call sites, divergences). Re-verify any
   anchor you make load-bearing: it was written against a commit that is now
   several behind.
7. **The design brief and both review rounds** in `.superpowers/sdd/plan-9/`
   for what was already adjudicated - so you do not re-open it.

## 3. What is already done, and must NOT be re-created

- **`gui-d23-reset-gating-form` is already written** into
  `docs/decision-ledger.yaml` by the controller (commit `b4daed6`). The
  design's section 4 lists it as an obligation; half of that ruled item is
  therefore closed already. The plan owns only the TEST half. A task that
  writes this entry again is a duplicate-id defect.
- **The design's four triggers are already mirrored** into the ROADMAP
  Triggers section (same commit). No task mirrors them again.
- **`core-d49-g1g2-experiment`** is NOT yet written: it records a measurement
  that does not exist until the experiment runs. The ledger is
  controller-written, so the plan's D49 task produces and reports the
  measurement, and the CONTROLLER writes the entry at the close. Say that
  explicitly in the task so no implementer tries to edit the ledger.
- **`core-121-planner-seam-and-hoist`'s `blocked_on`** clears at the plan
  close, controller-side. Same rule: no task edits house YAML.

## 4. What the plan must contain

**Task decomposition covering the whole design.** Every decision D91-D105
and every acceptance observable in the design's section 7 is implemented by a
named task. The reviewer will walk the design section by section and treat a
section with no task as a finding, so do that walk yourself first.

**Per task, at minimum:**
- Its goal in one sentence, and the D-numbers it implements.
- **Files and interfaces it owns**, as a list explicitly marked EXHAUSTIVE or
  EXEMPLARY. An unmarked list reads as exhaustive by house rule, and an
  exemplary list that is not marked is omission latitude.
- Steps, using checkbox syntax as structure only (progress never enters the
  plan document; the tracker is `.superpowers/sdd/plan-9/progress.md`).
- Its verification: the exact commands, and for every check whose passing
  result is an ABSENCE, the fire-verification (break it, watch it fire,
  restore, confirm the restore).
- What it must not decide, where the design's section 5 has an entry that
  touches it - referenced, not restated in full.
- Its model tier with the ground for it (see below).
- Cross-task constraints it must carry verbatim into dependent tasks.

**Sequencing and parallelism, argued rather than assumed.** The doctrine's
handle is a comparison, not a count: a worktree stream costs a setup, a
merge, a full ten-part gate run on the merged state, and the controller
choreography around both, so it earns its place only when the task's own work
exceeds that. Tasks that compile and run the suite usually clear it;
doc/config tasks almost never do. This package is code-heavy but strongly
sequential at its base - the seam has to exist before four call sites can be
migrated onto it, and `JobOutcome.panic` has to exist before CLI and GUI can
render it. Decide it explicitly and write the reasoning down. Two riders:
- If you cut a stream serial, say so as a ruling, and note that a serial
  ruling binds the CONTROLLER's dispatch concurrency too, not merely the task
  order (`a-serial-ruling-binds-dispatch-concurrency-too`).
- If two writers ever share one working tree, pathspec-scoped commits
  (`git commit -- <paths>`) are mandatory, because one tree means one index
  (`concurrent-writers-need-pathspec-scoped-commits`). Prefer separate
  worktrees over that discipline where a stream earns one at all.

**Model tiers per `proc-03-model-assignment`,** as a table with a ground
column: cheapest tier only where this plan carries the work verbatim
(transcription); mid tier for judgment implementers and EVERY task reviewer;
top tier for the whole-branch review at the close. The controller sets the
model parameter explicitly at every dispatch - an omitted parameter inherits
the session default, which is not an assignment.

**Global constraints section,** covering at least: ground-truth precedence;
the owner rulings and design decisions as binding and not re-litigated; the
latitude ban in both forms with the NEEDS_CONTEXT return path; the ten-part
gate verbatim from BUILDING.md, foreground, no subsets, before any push and
after every merge; no new runtime or product dependency in either ecosystem;
the SI-4 commit grant restated in every dispatch that expects a commit,
including the unsigned-commit rule and the exact trailer derived from that
dispatch's model parameter, never written as a literal; explicit staging,
never `git add -A`; counts recomputed from their enumerations; the
fire-verification rule; typography; and the implementer preamble (no
session-relocation tools, absolute paths, foreground runs only).

**Close actions,** as an explicit list the controller executes at the plan
close: the roll-up funnel over every ledger/review minor; the promotion sweep
of the FIVE owner-ruled entries the ROADMAP anchor records (corrected
2026-07-28 from "six" by the plan author, who counted the anchor's
enumeration: exec-36, exec-37, cli-08, exec-43 and
empty-bare-raw-property-rejected-at-validate; core-121 is the separate
close action below, not a sixth member); `core-121`'s
`blocked_on` clearing with its decided occurrence; the
`core-d49-g1g2-experiment` entry from the task's measurement; the whole-branch
review on the top tier; the SDD salvage with its `diff -r` re-check; the
journal entry; and the HANDOFF snapshot.

**Acceptance.** Map the design's section-7 observables onto tasks, and state
which of them are machine-verifiable and which are not. Two are explicitly
recorded in the design as having NO producer today (the GUI Run-gate
consequence of the new error severity, and the branch D103 edits). Do not
invent producers for them, and do not let them silently disappear from the
acceptance list either: carry them as named, uncovered consequences.

## 5. What the plan must NOT do

- **No design decision is re-opened, softened, or "improved".** The design's
  section 5 enumerates what the implementer must not decide; the plan
  inherits that list and adds to it where a task could otherwise wander.
- **No latitude clause, in either form.** Not an explicit permission, and not
  an omission: an unenumerated set in a normative position, a list ending
  open, a "one per X" with no X list, a step that requires inventing a name,
  a string, or a file that is not written down somewhere the implementer can
  read.
- **No new test scenarios beyond the ruled D23 tests** (D104) and the tests
  the design's D-entries pin. No Vitest, no `tauri::test`, no
  `src-tauri/tests/`, no IpcError funnel work.
- **No task edits any house-knowledge YAML** (`decision-ledger.yaml`, the
  three Tier-2 files). The controller is the single writer; a task that finds
  something ledger-worthy SURFACES it in its report.
- **No task creates a tag, publishes or edits a release, or resolves a README
  `placeholder(1.0)` comment.**
- **No progress markers in the plan document.** Checkbox syntax is structure;
  the tracker is the SDD scratch `progress.md`.
- Execution starts only at the owner's plan-approval gate. Say so in the
  header, as plan 8.5 does.

## 6. House rules that bind the plan document itself

- Counts are recomputed from the enumeration they summarize, at
  plan-authoring; a task that changes a set re-recounts the consuming line in
  the same change.
- An evidence line carries output that was PASTED from a run, never recalled
  or reconstructed, and never attributed to a command that was not the one
  run. This is the round-1 review's harvest and it is now recorded house
  knowledge; three counts failed it in one round on this very design.
- A completion check is stated ONCE, and every other passage cites it rather
  than restating it (`design-states-a-completion-check-once`). The design's
  acceptance observable 1 is the canonical instance: do not paraphrase it
  into a task.
- A check whose passing result is an absence must have a reachable GREEN
  state, not merely a fire-verified red one
  (`proc-check-green-state-reachable`). Run each such check against the
  intended END state, or reason member by member against the enumerated
  survivors, before it enters the plan. The design's D91 shipped exactly this
  defect in round 1.
- Typography: ASCII hyphens, straight quotes, no Unicode ellipsis, in the
  plan and in every string it prescribes.

## 7. Refuting this brief is a valid completion

If a premise here is wrong - a task boundary that cannot hold, a design
section that contradicts the spec, an already-done item I claim is open -
refute it with evidence in the plan document and say so. Do not resolve a
discovered fork at the keyboard: return NEEDS_CONTEXT with a decision memo
(options, costs against the named invariants, a recommendation) and let the
controller route it.
