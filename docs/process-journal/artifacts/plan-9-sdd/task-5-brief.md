# Task 5 implementer brief - Plan 9

**Role:** fresh implementer for Plan 9, Task 5 (central errors-first sort:
`severity_sorted` hoisted from the CLI into core and applied to
`config_diagnostics` in both JSON builders; `BatchView.vue` fetches the
parse-error diagnostic by code instead of by position; plus the enumerated
parse-failure apply e2e scenario. D102, D103; spec S-7). Model tier: mid
(dispatch model: Opus 5). Effort: xhigh. An independent reviewer grades your
work; the controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). `master`, main worktree, `/home/senol/Git/Muxsmith`.
- Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.**
- A shell hazard this plan already hit: a bare `cp` here is aliased
  interactive and blocks on overwrite, leaving a mutated tree behind a hung
  command. If you mutate anything to fire a check, restore with
  `git checkout --` or `command cp -f` and prove it (`sha256sum -c`,
  `git status --porcelain`), taking the baseline BEFORE mutating.

## What to read first

1. The plan (`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`):
   Global Constraints, then **Task 5** in full - Files list, Steps 1-8, "Must
   not decide".
2. The design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`):
   **D102** in full (site, stability contract, scope boundary, consumers
   sweep) and **D103** in full (evidence, rejected alternatives, and the
   amendment-1 producer paragraph, which enumerates the e2e scenario down to
   the fixture and both assertions); section 0 note 2; design section 5 (the
   sort/fetch bullets and the two-scenarios bullet); the `## Amendment log`
   at its current state.
3. The v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) -
   authoritative above the design; your Step 6 amends S-7.
4. The four house-knowledge YAMLs as ground truth alongside them.

## Scope

Exactly Task 5's Files list (EXHAUSTIVE), Steps 1-8 and "Must not decide".
Note what the plan and design hand you as already-fixed text: D102's doc
comment, D103's `find` line, the Step-3 profile YAML and the Step-5 scenario's
document fields and assertions are character-for-character contracts.
Transcribe, do not compose.

Two boundaries the plan states and the reviewer will check:

- **New test SCENARIOS on the existing Playwright + mock-IPC harness are in
  scope; new test INFRASTRUCTURE is not.** No new spec file, no new harness,
  no new mocks module.
- **The two new tests have opposite starting colours, and the plan says which
  is which.** Step 3's `dry-run`/`validate` parity test is **red on today's
  tree** (the authoring probe measured the dry-run side in collection order) -
  that red is its fire verification, so demonstrate it. Step 5's e2e apply
  scenario **passes on today's tree** and carries no red-today claim: the
  current positional fetch surfaces the same singleton `parse-error` and the
  `!doc.profile` branch already returns before any apply or save. Its
  discriminating power targets a defective code-keyed rewrite. Do not
  manufacture a red state for it, and do not report one.

## Anchors: re-derived at dispatch time, verified against the tree

The plan was authored before Tasks 3 and 4 landed, so its line numbers for
`e2e/smoke.spec.ts` are stale by construction. Measured on the current tree
just now:

- the `batch view: dry run` describe still opens at `e2e/smoke.spec.ts:140`
  (the plan's number holds);
- the apply-flow test whose scaffold you replay is at
  `e2e/smoke.spec.ts:460`, **not** the plan's `:406`;
- `src/views/BatchView.vue:225` carries the positional fetch
  `const parseDiagnostic = doc.config_diagnostics[0];` and is the file's only
  hit for that pattern.

Treat all three as pointers, not as contract: locate by content and re-derive
before editing (`proc-57-briefs-not-ground-truth`). If any of them fails to
reproduce, that is a finding for your report, not a reason to stop.

## Standing rules

- **No design latitude**, in either form (explicit permission, or omission -
  an unenumerated set in a normative position, a name or string you would have
  to invent). A fork found on code contact returns as **NEEDS_CONTEXT with a
  decision memo** (options, costs against the named invariants, a
  recommendation), routed by the controller, never resolved at the keyboard.
- **Test-coverage precedence - the owner ruled this on 2026-07-28, and the
  ruling is now live** (Tier-2 `tests-ship-with-the-feature-never-after`; it
  was an open question during Task 4, which is why Task 4's brief pre-routed
  it). If this task's own diff creates a user-visible consequence that the
  plan's enumerated tests do not carry, you **BUILD** the missing producer
  when **all four** hold: it is additive; it rides existing test
  infrastructure; the consequence is created by this package's own diff; and
  you name it in your report for the reviewer to rule on. Outside those four
  the plan's enumeration binds and the fork returns as NEEDS_CONTEXT. Run the
  four conditions explicitly and write the result into your report either way,
  including the case where nothing triggers them.
- **Structural-conformance grant** as the owner amended it 2026-07-28 and
  again today (`latitude-carveout-zero-content-structural-forks`, read the
  entry): zero outward effect only; the Files-list boundary runs over FILES;
  repairing a reference your OWN enumerated edit invalidated inside a LISTED
  file is in scope, and so - as of today's ruling - is **adding a symbol
  import your OWN enumerated addition requires inside a LISTED file, where
  that addition does not compile without it**. That second case survives the
  `smoke.spec.ts` entry's "nothing else in the file" qualifier, because the
  qualifier fences the OTHER tasks' regions in that file (Task 3's fixture
  sweep, Task 4's Run-gate scenario), not your addition's own prerequisites.
  For the record: `en` and `enAttr` are both already imported at
  `e2e/smoke.spec.ts:24`, so on the enumerated assertions this should not
  arise. Everything else still stops and returns: weakening, deleting,
  skipping or rewording an existing assertion, mutating existing fixture
  values, new test files, new test infrastructure.
- **If your diff deletes a `use` line** - Step 1 deletes the CLI's own
  `severity_sorted` definition - sweep the touched files for intra-doc links
  naming the removed or moved symbols
  (`an-import-removal-sweeps-the-doc-links-that-named-the-symbol`).
- **A count-word sweep on any set you extend reaches the CALLERS' docs**, not
  only the block you edited (`proc-normative-count-recomputed`, trigger 2).
- **No task edits any house-knowledge YAML.** Surface ledger-worthy
  observations in your report.
- Counts recomputed from their enumerations; every observed value pasted from
  its run.
- **Absence checks need a control, and the local instruments have two measured
  traps** (Task 4's harvest, now in the ledger): the shell `grep` here is
  **ugrep 7.5.0**, where `\b` plus bounded repetition under `-E` silently
  returns zero instead of erroring, and a checker carrying invisible glyphs as
  literals can degrade in transit. So for Step 7's
  `grep -n 'config_diagnostics\[0\]' src/views/BatchView.vue` -> 0: run it
  BEFORE your edit, paste the `:225` hit, and only then treat the post-edit
  zero as evidence. Prefer `grep -P` or a script for anything beyond a plain
  pattern.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.

## Verification bar (plan Step 7, foreground, no subsets)

`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
`cargo test --workspace`; `pnpm lint`; `pnpm build`; `pnpm test:e2e`.

Two predictions the plan makes that you are asked to confirm or refute, not to
accommodate:

- The design's consumers sweep predicts the existing CLI JSON tests pass
  unchanged, because they assert membership rather than position. **A
  positional failure there is a real finding, not a test to relax** - return it
  as NEEDS_CONTEXT.
- Every pre-existing e2e suite passes unchanged: the existing fixtures are mock
  documents that never pass through core, so outside your one added scenario
  any e2e diff is a defect signal.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push.
`git -c commit.gpgsign=false`, pathspec-scoped, stage each file by name (never
`git add -A`), the plan's Task-5 message, exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-5-report.md`, same
content as your final message (read as data): status (DONE /
DONE_WITH_CONCERNS / NEEDS_CONTEXT); per-file changes against the Files list;
evidence with pasted commands, output and the Step-3 red-then-green
demonstration; the four-condition test-coverage check and its result;
divergences and judgment calls, each named; numbered concerns a reviewer can
rule on yes/no; what you surface for the controller; commit hash and
`git show --stat`.
