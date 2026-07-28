# Task 4 implementer brief - Plan 9

**Role:** fresh implementer for Plan 9, Task 4 (`EmptyRawProperty`: a bare
`raw:` with an empty property name becomes a config-time ERROR with its own
diagnostic code, plus the GUI Run-gate e2e scenario; D101, spec S-1's new row,
S-3, S-5, S-6). Model tier: mid (dispatch model: Opus 5). Effort: xhigh. An
independent reviewer grades your work; the controller re-runs your claims.

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
   Global Constraints, then **Task 4** in full - Files list, Steps 1-8, "Must
   not decide".
2. The design (`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`):
   **D101** in full (both forks, the boundary paragraph, the accepted
   consequences, and the amendment-1 producer paragraph with its
   scenarios-in / infrastructure-out boundary), design section 5 (the raw
   bullets and the two-scenarios bullet), the authoring-time fixture probes and
   the amendment-1 anchors in the plan's verification section, and the
   `## Amendment log` at its current state.
3. The v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) -
   authoritative above the design; your Step 6 amends four of its passages.
4. The four house-knowledge YAMLs as ground truth alongside them.

## Scope

Exactly Task 4's Files list (EXHAUSTIVE), Steps 1-8 and "Must not decide".
Note what the plan hands you as already-fixed text: D101's three-branch
`raw_opt_in_diagnostic` fence, its variant doc, and its two Fluent lines are
character-for-character contracts. Transcribe, do not compose.

Two boundaries the plan states and the reviewer will check:

- **New test SCENARIOS on the existing Playwright + mock-IPC harness are in
  scope; new test INFRASTRUCTURE is not.** No Vitest, no `tauri::test`, no
  `src-tauri/tests/`, no harness change.
- The Run-gate e2e scenario **passes on today's tree** and carries no
  red-today claim: it asserts existing gating behaviour that nothing asserted
  before. The red-today half of this task is the core and CLI exit-code flip.
  Do not manufacture a red state for the scenario, and do not report one.

## Standing rules

- **No design latitude**, in either form (explicit permission, or omission -
  an unenumerated set in a normative position, a name or string you would have
  to invent). A fork found on code contact returns as **NEEDS_CONTEXT with a
  decision memo** (options, costs against the named invariants, a
  recommendation), routed by the controller, never resolved at the keyboard.
- **One fork is pre-routed, because it is currently with the owner.** If you
  find that this task introduces a user-visible consequence whose coverage the
  plan's enumerated tests do not carry, do NOT resolve it either way - neither
  by adding the test on your own authority nor by recording the gap and moving
  on. Return it as NEEDS_CONTEXT naming the consequence and the smallest
  additive test that would cover it. The precedence between a task's pinned-test
  enumeration and the Tier-2 rule that a feature's tests ship with the feature
  is an open owner question; Task 3 cost a review round and a fix round to
  exactly this shape.
- **Structural-conformance grant** as the owner amended it 2026-07-28
  (`latitude-carveout-zero-content-structural-forks`, read the entry): zero
  outward effect only; the Files-list boundary runs over FILES, and repairing a
  reference your OWN enumerated edit invalidated inside a LISTED file is in
  scope. Weakening, deleting, skipping or rewording existing assertions,
  mutating existing fixture values, new test files beyond the two snapshot
  files the plan lists, and new test infrastructure all stop and return.
- **If your diff deletes a `use` line**, sweep the touched files for intra-doc
  links naming the removed symbols
  (`an-import-removal-sweeps-the-doc-links-that-named-the-symbol`).
- **A count-word sweep on any set you extend reaches the CALLERS' docs**, not
  only the block you edited (`proc-normative-count-recomputed`, trigger 2).
- **A null-assertion over a parsed document proves nothing about presence**
  (`a-null-assertion-over-a-dynamic-map-proves-nothing-without-a-presence-check`):
  if you need to assert a key exists, assert its presence or a non-null value.
- **No task edits any house-knowledge YAML.** Surface ledger-worthy
  observations in your report.
- Counts recomputed from their enumerations; absence checks fire-verified with
  a reachable green state; every observed value pasted from its run.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis. The
  German Fluent line carries real orthography (`ä`, `ü`, `ß`) - copy it exactly.

## Verification bar (plan Step 7, foreground, no subsets)

`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
`cargo test --workspace` (the new core and CLI tests green; the existing B-2/B-3
controls green in the same run - they are the discriminating evidence that the
new branch fires on emptiness and not on `raw:` generally; no snapshot churn
beyond the two new files); `pnpm check:i18n`; `pnpm lint`; `pnpm test:e2e`.

The two `.code(2)` subprocess tests are red on today's tree by construction -
the authoring probe measured exit 0 with an info diagnostic on the identical
profile. Show that: run them (or the equivalent binary invocation) against the
pre-change state, paste the observed exit 0, then the post-change exit 2.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push.
`git -c commit.gpgsign=false`, pathspec-scoped, stage each file by name (never
`git add -A`), the plan's Task-4 message, exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-4-report.md`, same
content as your final message (read as data): status (DONE /
DONE_WITH_CONCERNS / NEEDS_CONTEXT); per-file changes against the Files list;
evidence with pasted commands, output and the red-then-green demonstration;
divergences and judgment calls, each named; numbered concerns a reviewer can
rule on yes/no; what you surface for the controller; commit hash and
`git show --stat`.
