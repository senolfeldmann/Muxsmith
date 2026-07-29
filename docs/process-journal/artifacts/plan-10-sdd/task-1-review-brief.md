# Task 1 review brief - Plan 10

**Role:** independent reviewer of Plan 10, Task 1 (W3: `BUILDING.md` states the
pre-push gate's total once, canonically, and `scripts/ledger-lint.py` checks
that statement against the commands the file's three marked gate blocks
enumerate). You did not write this code. Model tier: mid (dispatch model:
Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content as your final message.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Absolute paths, **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `ddb8f42`; the
  controller has made no product edit since.
- **Independent instruments.** Build every harness you use - scripts, mutated
  copies, fixture files - under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t1rev-independent/`
  (create it). **Never re-run an instrument the implementer wrote, and never use
  a shared default path**: agents in one session converge on the same names, so
  a re-run that silently executes the implementer's own script produces
  agreement by construction. Any absence-shaped check you rely on needs its own
  fire.
- If you mutate a tracked file to fire something, take the baseline FIRST
  (`sha256sum`), restore non-interactively (`git checkout --`, or
  `command cp -f` - a bare `cp` is aliased interactive here and hangs with the
  tree still mutated), and prove the restoration.
- **Shell note, surfaced by the implementer:** this shell is zsh, where
  `${PIPESTATUS[0]}` is empty (it is bash-only; zsh spells it `$pipestatus[1]`).
  If you build a pipeline whose exit status matters, do not rely on it.
- The tree must be byte-identical to `ddb8f42` when you finish. Prove it.

## Ground truth, in precedence order

1. The plan,
   `/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`:
   its **Global Constraints**, its **Authoring-time verification** section (the
   `BUILDING.md` measurements), **Task 1** in full - Files list, the two-extra-
   regions paragraph, Steps 1 through 6, "Must not decide" - and acceptance rows
   **W3-a through W3-f**, which are the halves this task must produce.
2. `.superpowers/sdd/plan-10/plan-brief.md`, section 4's **W3** item.
3. `docs/ROADMAP.md`, the **"Gate-count derivation has no check"** section
   including its MEASURED and NARROWED FORM blocks.
4. The four house-knowledge YAML files (`docs/decision-ledger.yaml`,
   `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`) as ground truth alongside them; cite entries
   by id, and re-verify any `:line` you rely on.

The implementer's brief (`task-1-brief.md`) and its report
(`task-1-report.md`) are **evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/review-754cb73..ddb8f42.diff`
carries the commit list, the stat summary and the full diff with context. Read
it in one call rather than re-deriving the range with git commands.

## Dimensions

1. **Contract compliance, character for character.** The canonical sentence and
   its marker (Step 1a), the three `gate-block` markers (Step 1c), the heading
   rewrite (Step 1b), both replacement blocks (Step 1d), the one replacement
   string (Step 1e), the CI job's comment and step-name repairs (Step 3), the
   summary line and the docstring widening (Step 2). Where the plan fences text,
   compare byte for byte with your own extraction, not by eye.
2. **The counting rule as the plan FIXES it** (Step 2), each behaviour checked
   against the implementation: exactly-one-occurrence per marker; the next
   non-empty line after a marker must be the bash fence; unterminated fence is a
   violation; a counted command line is non-empty and does not start with `#`;
   any backslash-continued line inside a gate block is a violation naming the
   continuation. And the SKIP logic, which is where a wrong implementation still
   looks green: a missing block marker skips BOTH that block's comparison and
   the total comparison, so exactly one violation names the cause; a missing
   total marker skips every comparison. Verify the skips by construction, not
   only through the implementer's two fires that walk into them.
3. **Re-measure the enumeration yourself.** Count the three gate blocks' command
   lines with your own expression and compare against the sentence's
   `11 / 6 / 4 / 1`. If your count differs from both the plan's and the
   implementer's, that is a finding of the first order.
4. **Re-fire the check with your own instruments.** At minimum reproduce F3, F4
   and F5 - the three whose expected violation SETS the plan states exactly
   (F3 and F4: exactly one violation; F5: the continuation message must be
   present alongside the two count mismatches). A check whose passing result is
   an absence proves nothing until it has produced output under your hand, not
   the author's.
5. **Latitude, both forms**, including the inverse: did the implementer decide
   at the keyboard something that should have returned as NEEDS_CONTEXT? Its
   numbered concerns are where to look hardest. Conversely, did it return or
   omit something the plan had already settled?
6. **House dimension.** Tier-2 conformance, in particular
   `ledger-lint-runs-before-every-push`,
   `latitude-carveout-zero-content-structural-forks` (the named-region repairs
   in `ci.yml` and `BUILDING.md`'s CI parenthetical - in scope, or over-reach?),
   `proc-normative-count-recomputed`, `proc-verification-step-must-be-falsifiable`
   and `proc-check-green-state-reachable`. Flag deviations from a recorded
   convention by id.
7. **The no-work-needed check.** Wherever the report or a code comment concludes
   that something is unnecessary, already covered, or cannot happen - run the
   premise that makes it so. Do not weigh it.
8. **Verification quality.** Re-run the full gate as `BUILDING.md` enumerates it,
   foreground, and recompute every aggregate the report states. The report's
   numbers are claims until you have reproduced them.
9. **Blast radius.** `scripts/ledger-lint.py` is a gate part that every later
   task in this plan runs. Satisfy yourself the new check cannot fail on a tree
   the four remaining tasks will legitimately produce (Tasks 2-5 touch tests,
   `renovate.jsonc`, `README.md`, `docs/INSTALL.md` and source comments - none
   of them `BUILDING.md`), and that the pre-existing YAML checks are unchanged
   in behaviour.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **The 86-character line.** Step 1(e)'s fenced replacement leaves what the
   implementer measured as the file's only non-fenced prose line over 80
   characters, and it did not reflow, on the ground that the plan fences that
   paragraph. Is leaving it correct fidelity to a fenced text, or is a reflow
   the in-scope structural conformance the house grant covers?
2. **`states 1 commands`.** The violation message is not pluralized, because the
   plan fences the message shape by example
   (`... states 4 commands but enumerates 5`). Acceptable as fenced, or a defect
   the fence did not intend?
3. **The two supplementary fires.** The implementer added two fires beyond the
   plan's five, for branches it judged the five do not reach. Were they in scope
   as evidence, or an unrequested extension of a task the owner approved at its
   current size? Note the asymmetry that matters here: evidence is not an
   artifact, and `proc-proposed-safeguard-stays` protects a proposed safeguard
   rather than mandating new ones.
4. **Step 1(f)'s surfacing duty against a set of THREE.** The implementer
   reported a third positional ordinal in `BUILDING.md` (`:135`, "what part 6
   cross-checks"), inside the same paragraph Step 1(e) edits, where Step 1(f)
   and the ROADMAP routing entry both enumerate two. **The controller has
   already corrected the ROADMAP entry to the measured three; that part is
   settled and is not your question.** Yours is: did the task correctly EDIT
   none of the three, and is the third site's presence inside an edited
   paragraph a reason it should have been handled differently within this task?
5. **The heading rewrite's reach.** Step 1(b) removes "six parts" from the Rust
   gate heading. Verify independently that no live consumer follows that
   wording - the plan asserts 57 hits, all historical or in this plan - and say
   whether the assertion holds at the current tree.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-1-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings (Critical / Important / Minor), each with
  `file:line`, the evidence you ran, and the exact required change.
- The five adjudications, one explicit verdict each.
- An evidence appendix naming your instrument paths and the commands you ran.
- A **HARVEST** section: observed dominant patterns, repeated rejections, and
  anything the remaining four tasks of this plan must carry. The controller is
  the single writer into the house-knowledge files; you surface, you do not
  write.

Your final message carries the same content in short form: verdict, the
findings as one-liners with severity, the adjudications, and the verdict file
path.
