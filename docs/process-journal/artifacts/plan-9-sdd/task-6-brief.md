# Task 6 implementer brief - Plan 9

**Role:** fresh implementer for Plan 9, Task 6 (make `JobsView` mountable in the
e2e mount harness, add a reactive-props hook, write the four ruled D23 reset/
gating tests plus the worker-panic render test, and hoist the three local
`name()` helper copies into one shared export. D104; amendment 2). Model tier:
mid (dispatch model: Opus 5). Effort: xhigh. An independent reviewer grades your
work; the controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). `master`, main worktree, `/home/senol/Git/Muxsmith`.
- Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.**
- A shell hazard this plan already hit: a bare `cp` here is aliased interactive
  and blocks on overwrite, leaving a mutated tree behind a hung command. If you
  mutate anything to fire a check, restore with `git checkout --` or
  `command cp -f` and prove it (`sha256sum -c`, `git status --porcelain`),
  taking the baseline BEFORE mutating.

## What to read first

1. The plan (`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`):
   Global Constraints, then **Task 6** in full - Files list, Steps 1-5, "Must
   not decide".
2. The design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`):
   **D104** in full - the harness enumeration, the four assertions, the two
   deliberately-not-duplicated orderings, and the rationale for not passing
   `runActive` as a prop; section 0 note 3; the `## Amendment log` at its
   current state.
3. The ROADMAP anchor's harness-scope correction paragraph, and the ROADMAP
   trigger about a fourth spec file needing the local `name()` helper - **NOT
   FIRED; consumed early by owner ruling.** Your Step 3 executes an owner
   ruling, not a fired trigger, and the plan says why.
4. The four house-knowledge YAMLs as ground truth alongside them.

## Scope

Exactly Task 6's Files list (EXHAUSTIVE: six modified, one created), Steps 1-5
and "Must not decide". D104's `mount-entry.ts` fence, the hook name, the four
test names and their assertions, and the hoisted helper's body are
character-for-character contracts. Transcribe, do not compose.

**The one carried conditional, so you do not rediscover it:** the reactive-props
hook is controller-judged in-scope as mechanics the ruled test requires. If that
judgment is overturned, the spec drops its double-dispatch test and keeps the
rest. It has not been overturned; build all four.

## Anchors: re-derived at dispatch time, verified against the tree

The plan's line numbers date from amendment time and Task 5 has landed since.
Measured by the controller on the current tree just now, and all of them still
reproduce:

- the three local helper copies, `grep -rn "^function name(" e2e/*.spec.ts`:
  `editor-markers.spec.ts:29`, `editor-rule-add-remove.spec.ts:41`,
  `smoke.spec.ts:60`. Task 5's addition landed below smoke's copy, so nothing
  shifted.
- `e2e/i18n-en.ts`: `export function en(` at `:155`, the `FluentVariable` type
  import at `:61`, and `grep -c "export function name(" e2e/i18n-en.ts` -> **0**
  (the red state of your Step-4 presence control).
- `grep -c FluentVariable` is **2** in each of the three spec files, so in each
  the type's only use is the helper signature you delete - the plan's premise
  for dropping the import holds.
- `ls e2e/*.spec.ts | wc -l` -> **9**; your new spec is the tenth.
- The two testids your assertions target exist: `cancel-batch` at
  `src/views/JobsView.vue:263`, `job-panic` at `src/components/JobRow.vue:63`.

Treat these as pointers, not contract: locate by content and re-derive before
editing (`proc-57-briefs-not-ground-truth`). A pointer that fails to reproduce
is a finding for your report, not a reason to stop.

## Standing rules

- **No design latitude**, in either form (explicit permission, or omission - an
  unenumerated set in a normative position, a name or string you would have to
  invent). A fork found on code contact returns as **NEEDS_CONTEXT with a
  decision memo** (options, costs against the named invariants, a
  recommendation), routed by the controller, never resolved at the keyboard.
- **The three ordering tests codify already-adjudicated behaviour. If one fails
  against the unmodified views, that contradicts the adjudicated premise and
  returns as NEEDS_CONTEXT with the failure pasted.** It is never "fixed" at the
  keyboard - not in the view, not in the test. No code fix is in scope for this
  task.
- **Test-coverage precedence** (Tier-2 `tests-ship-with-the-feature-never-after`,
  owner ruling): if this task's own diff creates a user-visible consequence the
  plan's enumerated tests do not carry, you BUILD the producer when all four
  hold - additive, existing infrastructure, the consequence comes from this
  diff, named in your report. Outside those four the enumeration binds and the
  fork returns. This task is mostly tests already, so state the check's result
  even when nothing triggers it.
- **Structural-conformance grant** (`latitude-carveout-zero-content-structural-forks`,
  read the entry): zero outward effect only; the Files-list boundary runs over
  FILES; repairing a reference your own enumerated edit invalidated inside a
  LISTED file is in scope, and so is adding a symbol import your own enumerated
  addition requires inside a LISTED file where it would not compile otherwise
  (owner ruling 2026-07-28). Your Step 3 is the mirror case and the plan already
  enumerates it: add `name` to each file's existing `./i18n-en` import, drop the
  then-unused `FluentVariable` type import. Weakening, deleting, skipping or
  rewording an EXISTING assertion, mutating existing fixture values, and new
  test infrastructure beyond what D104 fences all still stop and return.
- **The hoist is a pure move and the rationale comment is its load-bearing
  half.** The doc comment that travels is smoke's verbatim - the one explaining
  why `exact: true` exists (the "Run" / "Dry run" / `run-demo.yaml` substring
  collision) - not either of the two shorter mirror comments. Losing that reason
  in the move is the one cost this step must not pay.
- **This task does NOT write, edit or duplicate the `gui-d23-reset-gating-form`
  ledger entry.** It exists already; a task writing it again is a duplicate-id
  defect. No task edits any house-knowledge YAML - surface ledger-worthy
  observations in your report instead.
- Counts recomputed from their enumerations; every observed value pasted from
  its run.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.

## Verification (plan Step 4, foreground, no subsets)

`pnpm lint`; `pnpm test:e2e`.

Three things about how the e2e leg actually runs, because getting them wrong
produces green results that mean nothing:

- **`pnpm test:e2e` regenerates the harness bundles** (`tsc --noEmit -p
  e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && vite
  build --config e2e/vite.mount.config.ts && playwright test`). A bare
  `pnpm exec playwright test ...` does NOT: it runs the previously built
  `e2e/.generated/*`. Your Step-1 harness change lives in exactly those bundles,
  so any scoped run you paste as evidence either goes through `pnpm test:e2e`
  or shows the two `vite build` steps before it
  (ledger `frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`).
- **Every pre-existing e2e suite must pass unchanged.** The baselines, measured
  at the current HEAD: `pnpm test:e2e` -> **64 passed**; your four new tests
  should take it to 68, with no other count moving. `e2e/mount.ts` is not
  touched and the existing mount specs never call the new hook - their staying
  green in the same run is the no-regression evidence for Step 1.
- The Rust side is untouched by this task, so `cargo test --workspace` must
  still show **39 `test result:` lines, all ok**. Run it once at the end as a
  control if you touch anything outside `e2e/`.

**The absence check and its fire** (Step 4): `grep -rn "^function name(" e2e/*.spec.ts`
-> 0 after the hoist. Run it BEFORE your edit and paste the three hits, then
treat the post-edit zero as evidence. The local `grep` is **ugrep 7.5.0**, where
`\b` plus bounded repetition under `-E` silently returns zero - use `-P` or a
script for anything beyond a plain pattern. Presence control after the move:
`grep -c "export function name(" e2e/i18n-en.ts` -> 1, whose red state (0) the
controller already measured pre-hoist and is quoted above.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push. `git -c commit.gpgsign=false`,
stage each file by name (never `git add -A`), the plan's Task-6 message, exactly
one trailer `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no
`Claude-Session` line.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-6-report.md`, same
content as your final message (read as data): status (DONE /
DONE_WITH_CONCERNS / NEEDS_CONTEXT); per-file changes against the Files list;
evidence with pasted commands and output, including the absence check's fire and
the pure-move claim's support; the four-condition test-coverage check and its
result; divergences and judgment calls, each named; numbered concerns a reviewer
can rule on yes/no; what you surface for the controller; commit hash and
`git show --stat`.
