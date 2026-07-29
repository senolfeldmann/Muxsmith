# Task 3 implementer brief - Plan 10

**Role:** fresh implementer for Plan 10, Task 3 (W4: `renovate.jsonc` at the
repo root - the ruled monthly, per-ecosystem Renovate configuration). Model
tier: mid (dispatch model: Opus 5). Effort: xhigh. An independent reviewer
grades your work afterwards; the controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools. Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.** Tasks 1 and 2 have landed.
- Shell hazards, both measured in this project: a bare `cp` is aliased
  interactive here and blocks on overwrite; and this shell is **zsh**, where
  `${PIPESTATUS[0]}` is empty (bash-only, zsh spells it `$pipestatus[1]`).
  Capture exit codes directly.

## What to read first

1. The plan,
   `/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`:
   the **Global Constraints**, the **Authoring-time verification** section's
   **Renovate premises** block and its **Repo facts** block (both are the
   authoring measurement you re-verify), corrections 3 and 4 in the corrections
   table, **Task 3** in full - Files list, Steps 1 through 6, "Must not decide"
   - and acceptance rows **W4-a, W4-b, W4-c**.
2. `.superpowers/sdd/plan-10/plan-brief.md`, section 4's **W4** item in full.
3. The Tier-2 entry **`ci-04-dependabot-cadence`** (grep the id in
   `docs/process-conventions.yaml`) - the ruled cadence, grouping, major-PR and
   manager decisions. This is the owner's ruling the file implements.
4. `docs/ROADMAP.md`, the **Renovate entry in the Pre-1.0 release gates
   section** and its two riders (the deny.toml RUSTSEC pruning and the TS-7
   ceiling), plus the ROADMAP trigger line "Renovate/Dependabot activated -> ...".
5. The repo facts the file's comments assert: `mise.toml`, `package.json`'s
   `packageManager` field, `rust-toolchain.toml`, and `.github/workflows/release.yml`'s
   ubuntu-22.04 pins with their in-file rationale comment.

## Scope

**Files (EXHAUSTIVE): create `renovate.jsonc` at the repo root. Nothing else.**
No workflow, no gate part, no CI job, no dependency, no lockfile entry, and no
edit to `deny.toml`.

The file's content is fenced in Task 3 Step 1 character for character:
every key, every value, every comment, and the ORDER of the `packageRules`
entries (later rules win, so order is semantic). Transcribe it, do not compose
it, and do not "helpfully" add an option - the absent options
(`separateMajorMinor`, `vulnerabilityAlerts`, `config:best-practices`,
`security:minimumReleaseAgeNpm`) are as fixed as the present ones, each for a
reason the plan states.

## The two things this task actually decides nothing about but must MEASURE

1. **Step 2: re-verify every option against current vendor documentation before
   trusting the fence.** This is fast-moving vendor tooling and the plan's own
   verification is a day old. The plan enumerates the items to check; for each,
   name the URL or source file read and paste the value observed.
   **One named trap, so the re-verification does not "correct" a value that is
   already right:** the rendered configuration-options page yields
   `prHourlyLimit` default `10` under naive reading - that is
   `prConcurrentLimit`'s default, picked up from the neighbouring section. Only
   `lib/config/options/index.ts` settles it, where the entry reads `default: 2`.
   The fence's `prHourlyLimit: 0` and its comment's "the schema default is 2"
   are CORRECT. The general rule this instance teaches, binding for the whole
   list: **where a rendered docs page and the source disagree,
   `lib/config/options/index.ts` and `lib/config/presets/internal/` are the
   ground truth**, and your report names which of the two you read per item.
2. **Step 3: validate, twice, and paste both runs with their exact commands and
   full output** - plain and `--strict`, at the pinned version. If the pinned
   version no longer resolves, read `https://registry.npmjs.org/renovate/latest`
   and use what it returns, pasting both the URL read and the version used.

**Three forks are pre-routed and you resolve none of them:**

- **A premise refuted at the source is a valid completion**, not a failure:
  report it with the evidence and return NEEDS_CONTEXT rather than building
  around it silently.
- **A `--strict` finding is not fixed at the keyboard.** A migration or
  deprecation warning means one of the fenced options has moved, which is a
  design question -> NEEDS_CONTEXT with the pasted output.
- **If the `vulnerabilityAlerts` defaults have moved** so that immediate,
  ungrouped security PRs are no longer the shipped default, that behaviour must
  be configured explicitly - which is a change to the fence, so it returns
  rather than being written.

## What this task must NOT claim (Step 4)

Writing this file does not activate Renovate. Your report states plainly that
activation needs two OWNER actions - installing the Renovate app against this
repository, and enabling the repository's dependency graph and Dependabot alerts
so the immediate-security half exists at all - and that the ordering matters:
the config must be on `master` BEFORE the app is installed, which is what
suppresses the vendor's onboarding PR. Your report does **not** claim the
ROADMAP trigger "Renovate/Dependabot activated -> prune deny.toml RUSTSEC
ignores; TS-7 bump arrives via its PRs" has fired. It has not: its condition is
activation, not configuration.

## Standing rules

- **No design latitude**, in either form - explicit permission, or omission (an
  unenumerated set in a normative position, a value you would have to invent).
  A fork found on contact returns as **NEEDS_CONTEXT with a decision memo**.
- **No task edits any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md`. Ledger-worthy observations go in your report.
- **Every observed value in your report is pasted from the run or the page that
  produced it**, never recalled, and never attributed to a source that was not
  the one read.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis - in the
  file's comments as well as in your report.
- **GitHub interaction rules:** nothing that costs money, and no repository
  administration. The validator runs locally through `npx`; it leaves nothing
  behind - no gate part, no CI job, no dependency, no lockfile entry. Verify
  that claim rather than assuming it (`git status --porcelain` after the runs).

## Verification bar

1. Both validator runs pasted, with commands and full output (Step 3).
2. The per-option re-verification table (Step 2), one row per enumerated item,
   each naming its source and the value observed.
3. **The full gate as `BUILDING.md` enumerates it**, foreground, green, before
   the commit. `renovate.jsonc` is consumed by no gate part, so the gate proves
   only that nothing else broke - say so rather than overclaiming.
4. `git status --porcelain` and `git diff --stat` showing exactly one new file.
   A `FAIL BUILDING.md: ...` line from the gate's house-knowledge part would
   mean something wrote to `BUILDING.md` outside its owner's Files list: that is
   a defect signal -> NEEDS_CONTEXT, not a local fix.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply here. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...` - agent commits are deliberately
  unsigned, as policy.
- Stage explicitly by name, **never `git add -A`**.
- The commit command and message are fenced in Step 6.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
  No `Claude-Session` line, no context-window suffix.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-3-report.md`:

- Status: DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT.
- The per-option re-verification table, with sources and pasted values, plus an
  explicit note per item saying whether the fence held or is refuted.
- Both validator runs, verbatim.
- The transcription check: how you verified the committed file matches the
  fence character for character (name your method).
- The activation statement of Step 4, in the exact scope the plan allows.
- Full gate result; `git status --porcelain`; `git diff --stat`.
- Divergences and judgment calls, each named.
- Numbered concerns a reviewer can rule on yes/no.
- What you surface for the controller.
- Commit hash and `git show --stat`.
