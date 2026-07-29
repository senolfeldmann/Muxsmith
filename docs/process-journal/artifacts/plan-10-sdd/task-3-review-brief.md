# Task 3 review brief - Plan 10

**Role:** independent reviewer of Plan 10, Task 3 (W4: `renovate.jsonc` at the
repo root, the ruled monthly per-ecosystem Renovate configuration). You did not
write this file. Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Preamble (binding)

- Never call session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `630d418`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t3rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never a shared
  default path. Fetch vendor sources yourself rather than trusting a pasted
  quotation.
- If you mutate anything, baseline first (`sha256sum`), restore
  non-interactively, prove it. A bare `cp` is aliased interactive here.
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty. Capture exit codes
  directly.
- The tree must be byte-identical to `630d418` when you finish. Prove it.

## Ground truth, in precedence order

1. The owner's ruling, Tier-2 **`ci-04-dependabot-cadence`** in
   `docs/process-conventions.yaml` - the cadence, the grouping, the major-PR
   handling, the security handling, and which managers are deliberately
   unmanaged. The file exists to express THIS.
2. `docs/ROADMAP.md`, the Renovate entry in the Pre-1.0 release gates section
   and its two riders, plus the ROADMAP trigger line
   "Renovate/Dependabot activated -> ...".
3. The plan,
   `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`: Global
   Constraints, the Authoring-time verification section's **Renovate premises**
   and **Repo facts** blocks, corrections 3 and 4, **Task 3** in full (Files
   list, Steps 1-6, "Must not decide"), and acceptance rows **W4-a, W4-b, W4-c**.
4. The vendor's CURRENT documentation and source, which outrank the plan's
   authoring snapshot on any factual question about Renovate's behaviour.
5. The four house-knowledge YAML files; cite entries by id.

The implementer's brief (`task-3-brief.md`) and report (`task-3-report.md`) are
**evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/review-9ff4173..630d418.diff`

## Dimensions

1. **Transcription fidelity.** The committed `renovate.jsonc` against the plan's
   Task 3 Step 1 fence, character for character, including comment text and the
   ORDER of `packageRules` entries (later rules win, so order is semantic).
   Extract both and diff them with your own instrument; do not compare by eye.
2. **Does the file express the owner's ruling?** Walk `ci-04-dependabot-cadence`
   clause by clause against the file: monthly cadence, per-ecosystem grouping,
   majors on their own PRs, security immediate and ungrouped, `mise` unmanaged,
   `rust-toolchain` in its own group, runner images unmanaged. Name the key that
   carries each clause, or the deliberate absence that does. **An absent option
   is as load-bearing as a present one here** (`separateMajorMinor`,
   `vulnerabilityAlerts`, `config:best-practices`,
   `security:minimumReleaseAgeNpm`), and each absence has a stated reason -
   check the reason, not just the absence.
3. **Re-verify the premises yourself, at the source.** The report claims all 15
   held. Spot-check at minimum: `prHourlyLimit`'s default (the named trap - the
   rendered docs page yields `prConcurrentLimit`'s 10 under naive reading);
   `separateMajorMinor`'s default and its stated priority over package groups;
   what `config:best-practices` extends; the literal value of `schedule:monthly`;
   the `mise` and `rust-toolchain` managers' existence and file matching; the
   `action`, `github-runner` and `packageManager` depTypes; and
   `vulnerabilityAlerts`' shipped defaults. Where a rendered page and the source
   disagree, `lib/config/options/index.ts` and `lib/config/presets/internal/`
   are ground truth. Say which you read per item.
4. **The repo facts the file's COMMENTS assert** - they are claims like any
   other and a wrong one misleads the next reader: `mise.toml`'s node and pnpm
   pins, `package.json`'s `packageManager` mirror, `rust-toolchain.toml`'s
   channel, and `release.yml`'s three ubuntu-22.04 pin sites with their in-file
   rationale. Verify each against the tree.
5. **The validation evidence.** Two runs are prescribed, plain and `--strict`.
   Adjudication question 2 below asks whether they validated what the plan
   thinks they validated - measure it rather than reasoning about it.
6. **Nothing became permanent.** No gate part, no CI job, no dependency, no
   lockfile entry, no leftover from `npx`. Verify with your own commands
   (`git status --porcelain`, the lockfile's diff, `package.json`), not from the
   report's assertion.
7. **The activation boundary.** The report must not claim Renovate is active,
   and must not claim the ROADMAP trigger has fired. Check what it actually
   says. The two owner actions and their ORDER (config on `master` first, app
   installed second, which is what suppresses the onboarding PR) must be stated
   accurately.
8. **House dimension:** Tier-2 conformance;
   `proc-07`/`proc-57`-style premise verification;
   `latitude-carveout-zero-content-structural-forks`; typography (ASCII hyphens,
   straight quotes, no Unicode ellipsis) inside the file's comments.
9. **The no-work-needed check.** Wherever the report concludes something is
   unnecessary, already covered, or left nothing behind, run the premise.
10. **Blast radius on Task 5**, which sweeps line-number citations out of source
    comments and whose corpus expressions cover `*.js`, `*.mjs`, `*.ts`, `*.vue`,
    `*.rs`, `*.py`. Confirm `renovate.jsonc` adds no corpus member and that the
    corpus is still 20 lines / 13 files (expression A) and 4 lines / 4 files
    (expression B), each with a fired control.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **The pinned validator version is one major behind.** The plan pins
   `renovate@43.287.0`, read from the registry at authoring; current is
   `44.0.1`. The implementer ran the pinned version as fenced and additionally
   validated clean against `44.0.1`. Is running the fenced pin correct fidelity
   (the pin is an instrument, nothing persists, and the plan fences the
   invocation), or should a version this stale have returned as NEEDS_CONTEXT
   before validating? Say what, if anything, the report should record about
   `44.0.1`.
2. **What the fenced invocation actually validates.** The implementer reports
   that `renovate-config-validator <file>` treats a named file as GLOBAL config
   by the validator's documented default, not as repo config, and added
   supplementary repo-config runs of its own. **Verify that claim yourself**,
   then rule: does the fenced invocation validate the thing this file will
   actually be used as? If it does not, is that a defect in the plan's Step 3
   worth routing, and does the supplementary run close it?
3. **The `EBADENGINE` warning and the two depType description drifts.** The
   validator warns that renovate wants node `^24.11.0` while this repo pins
   `26.5.0`; two depType descriptions have drifted in wording (names unchanged).
   Both were reported rather than acted on. Correct restraint, or does either
   change a fact the file's comments assert?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-3-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with `file:line`, evidence run, exact required change;
the three adjudications; an evidence appendix naming instrument paths and
commands; and a **HARVEST** (patterns, repeated rejections, what Tasks 4-5 must
carry). The controller is the single writer into the house-knowledge files.

Your final message carries the same in short form.
