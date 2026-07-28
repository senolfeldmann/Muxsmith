# Amendment 3, plan side - author brief

**Role:** author of the plan-side half of Plan-9 amendment 3. The design half
is written, reviewed and APPROVED; you carry it into the execution plan. An
independent reviewer grades your amendment afterwards. Model tier: top
(dispatch model: Fable 5). Effort: xhigh.

**Why you and not the plan's original author:** that agent lived in the
previous session and cannot be resumed across sessions. Read the plan and the
amended design before writing.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Repo `/home/senol/Git/Muxsmith`, `master`, main worktree.
- Absolute paths, foreground runs only.
- **Do not pin your reading to a commit hash.** House-knowledge commits land
  between dispatches. Read the FILES.
- **You edit exactly one file:** the plan
  `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`.
  Not the design (its half is done and approved), not the spec, not source,
  never a house-knowledge YAML - surface ledger-worthy observations in your
  report.
- You commit. Commits on this repo are **standing-authorized by the owner**:
  `git -c commit.gpgsign=false`, **pathspec-scoped** (`git commit -- <the plan
  file>`; another writer may hold staged files in this shared index), exactly
  one trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, no
  `Claude-Session` line. Do not push.

## Context

Task 2 hoisted `run_batch` into `muxsmith_core::executor::queue` carrying its
rustdoc verbatim, as D96 and the plan's Task-2 Step 1 required. The same commit
gave the function its second caller (the CLI), which falsified three passages
of that doc. **Owner ruling 2026-07-28: this is a design change, and the code
edit rides Task 3.**

The design amendment is in
`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`:
a rider at the end of D96 carrying the exact replacement doc comment as a
transcription fence, a pointer in section 5's D96 bullet, and a Round-4
entry in the `## Amendment log`. Its delta review
(`.superpowers/sdd/plan-9/amendment-3-verdict.md`) is APPROVED with no
findings; read it, its dimension-7 section is about your work.

## What your amendment must carry

Items 1-3 were verified against the plan by the design reviewer; verify them
yourself before relying on them (`proc-57-briefs-not-ground-truth`).

1. **Task 3 gains one instruction**: replace the `///` block immediately above
   `pub fn run_batch` in `crates/muxsmith-core/src/executor/queue.rs` with the
   fence in D96's amendment-3 rider, transcribed character for character,
   located by the `pub fn run_batch` anchor rather than by line number, with no
   other change to the function. Where that instruction lives - an addition to
   an existing step or its own step - is your call; make it, and say why.
2. **Task 3's "Read first" line** gains D96's amendment-3 rider.
3. **No Files-list change.** `crates/muxsmith-core/src/executor/queue.rs` is
   already on Task 3's exhaustive list and in its `git add` line.
   `src-tauri/src/run.rs` is NOT on that list and does not join it: the design
   deliberately adds no src-tauri sentence. (An earlier controller brief
   claimed that file was listed; it is not - the design author refuted it and
   the reviewer re-measured. Do not inherit the error.)
4. **The plan's own amendment log** gains its amendment-3 entry in the log's
   existing shape. Note the offset the design log records: amendment 2 was
   plan-only, so the two logs are not numbered in lockstep - follow what each
   file already does rather than forcing symmetry.
5. **Two questions you must DECIDE and record, not leave open** (the design
   reviewer raised both; either answer is defensible, silence is not):
   - Does Task 2's executed Step-1 wording ("rustdoc moved with it", plan
     `:193` at review time) get a historical qualifier, or stay as the record
     of what Task 2 was told? A silent leave-as-is invites the same
     "moves as-is" misreading the design side just closed.
   - Does the Files-entry parenthetical for `queue.rs` in Task 3 get the
     rustdoc replacement added to its work description? Optional under the
     owner's 2026-07-28 file-vs-within-file ruling (an entry without a
     within-file qualifier does not constrain within-file work), and cheap
     insurance against a future reader treating the parenthetical as
     exhaustive.

## Constraints

- **Do not duplicate the fence into the plan.** The plan POINTS at D96's
  amendment-3 rider; two copies of a character-for-character contract drift.
- **No design latitude reaches Task 3's implementer**, in either form: no
  explicit permission, and no omission - nothing the implementer would have to
  invent, no unenumerated set, no ellipsis.
- **Nothing else in the plan changes.** Not Task 3's other steps, not another
  task, not the Global Constraints (which already bind every task to the
  design's amendment log at execution time - verify that sentence is there
  rather than assuming it).
- Counts recomputed from their enumerations; observed values pasted from the
  run that produced them; ASCII hyphens, straight quotes, no Unicode ellipsis.
- The v1 spec stays authoritative above the design; the design above the plan.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/amendment-3-plan-report.md`
and make your final message the same content (read as data): status; the exact
diff of what you changed, quoted; your two decisions from item 5 with their
reasons; the premise checks you ran on items 1-3 with evidence; anything
surfaced for the controller; commit hash and `git show --stat`.
