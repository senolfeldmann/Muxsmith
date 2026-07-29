# Task 2 review brief - Plan 10

**Role:** independent reviewer of Plan 10, Task 2 (W1: the D102 preserved-order
producers, selected by a four-mutation measurement). You did not write this
code. Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content as your final message.

## Preamble (binding)

- Never call session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `35bc363`.
- **Independent instruments.** Build every harness, mutated copy and probe under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t2rev-independent/`
  (create it). Never re-run an instrument the implementer wrote, and never a
  shared default path: agents in one session converge on the same names, so a
  re-run that silently executes the implementer's own script agrees by
  construction.
- If you mutate a tracked file, take the baseline FIRST (`sha256sum`), restore
  non-interactively (`git checkout --` or `command cp -f` - a bare `cp` is
  aliased interactive here and hangs with the tree still mutated), and prove the
  restoration. **This task's whole method is mutation of a production file**, so
  the tree must be byte-identical to `35bc363` when you finish. Prove it.
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty (bash-only; zsh spells it
  `$pipestatus[1]`). Capture exit codes directly.

## Ground truth, in precedence order

1. The v1 spec, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`,
   **section 5.2 "Diagnostics"** - the ordering sentence IS the contract, and it
   is authoritative above plan and design on conflict.
2. `docs/ROADMAP.md`, the **D102 paragraph in the Plan-9 anchor** through its
   "RULED 2026-07-29 ... BUILD IT" close.
3. The plan,
   `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`: Global
   Constraints, the Authoring-time verification section's D102 block, **Task 2**
   in full (Files list, Steps 1-6, "Must not decide"), and acceptance rows
   **W1-a through W1-d**.
4. `.superpowers/sdd/plan-10/plan-brief.md` section 4's W1 item.
5. The four house-knowledge YAML files; cite entries by id.

The implementer's brief (`task-2-brief.md`) and report (`task-2-report.md`) are
**evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/review-39a9055..35bc363.diff`

## Dimensions

1. **Re-run the measurement yourself.** This is the crux: the disposition of
   four contract halves - two producers written, two not - rests entirely on
   which mutation reddened. Apply all four mutations independently, one at a
   time, with your own edits and your own restores, and confirm the reported
   pattern (M1 red, M2 red, M3 green, M4 green) and the named guarding tests.
   A disagreement here is a Critical finding.
2. **The mkvmerge dependency of that measurement.** `mkvmerge` was present, so
   M1 is a real measurement. Verify that, and satisfy yourself the reported
   guarding test for W1-a is the one that actually reddened rather than a
   plausible neighbour.
3. **The two producers against the plan's fence**: `KeyRenderer`,
   `mixed_severity()`, `codes()`, P1 and P2 - names, homes, fixtures,
   assertions, and the import list. Compare with your own extraction, not by
   eye. Where the committed text deviates from the fence, that deviation is
   adjudication question 1 and must be measured, not argued.
4. **Do the producers assert what they claim?** Each must fail when its own
   mutation is applied and only then. Verify the symmetry claim independently:
   apply M3 and confirm P2 alone reddens; apply M4 and confirm P1 alone reddens.
   A test that passes under its own mutation is a Critical finding; a test that
   reddens under the other's is a real coupling worth naming.
5. **No production code changed.** `crates/muxsmith-core/src/report/json.rs`
   must be byte-identical to its state at `39a9055`; diff it yourself rather
   than trusting the report's checksums. Every pre-existing test must be
   unchanged in behaviour.
6. **House dimension.** `tests-ship-with-the-feature-never-after`;
   `comments-locate-by-symbol-never-by-line-number` (the assertion messages must
   name spec section 5.2 by SECTION, never by line - the plan states this
   explicitly, and the package's own W2 sweeps that class out);
   `latitude-carveout-zero-content-structural-forks` (adjudication question 2);
   `proc-verification-step-must-be-falsifiable` PER ASSERTION.
7. **The no-work-needed check.** Wherever the report concludes something is
   unnecessary, already covered, or cannot happen, run the premise. Do not weigh
   it.
8. **Verification quality.** Re-run the full gate as `BUILDING.md` enumerates it
   and recompute every aggregate the report states (it claims 503 -> 505 passing
   with the delta being exactly the two new tests).
9. **Blast radius on Task 5**, which sweeps line-number citations out of source
   comments across the repo. Does anything this task added introduce a NEW
   corpus member - a `file.rs:123` form, or a bare `:123` span - in a comment or
   an assertion message? Run both of Task 5's expressions (they are fenced in
   the plan's Task 5 Step 1) against the current tree with a fired control, and
   state whether the corpus is still 20 lines / 13 files under expression A and
   4 lines / 4 files under expression B.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **The fence versus `cargo fmt`.** The implementer reports that the plan's
   fenced `mixed_severity()` body cannot survive gate part 1: `cargo fmt --all
   --check` exits 1 on the fenced form, and rustfmt itself produces the wrapped
   form that was committed. **Verify that conflict independently** - construct
   the fenced form in your own scratch copy and run rustfmt on it - and then
   rule: was writing the formatter's output correct fidelity to the plan taken
   as a whole (whose Global Constraints make the gate binding and whose fence
   exists so that no CONTENT is invented), or is a fenced text a fence
   character for character, so the collision should have returned as
   NEEDS_CONTEXT before anything was committed? Say explicitly whether the
   committed form differs from the fence in anything but line breaks and
   indentation.
2. **The module doc comment.** `crates/muxsmith-core/tests/report_json.rs`
   carries a module-level doc comment describing the file's contents, and this
   task's own addition arguably falsifies it (the file no longer covers
   `run_document` only). The implementer left it untouched and surfaced it,
   reading the task's three named regions as a within-file fence. Is that
   correct restraint, or is this the grant's named in-scope case - repairing a
   reference the task's OWN enumerated edit invalidated, inside a LISTED file?
   Note this is the second instance of the same shape in two tasks; your ruling
   is calibration data for the over-restriction watch either way.
3. **The `have_mkvmerge()`-gated sorted-half guard.** W1-a's guarding test is
   gated on mkvmerge being present, so on a machine without it that half is
   unguarded and the measurement would have selected a producer for the wrong
   reason. The implementer built nothing, because Task 2 forbids duplicating
   those guards. Confirm whether the ROADMAP already tracks this coverage fact
   (Plan-9 anchor, D102 paragraph) and rule: correct handling, or does something
   belong in this task after all?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-2-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with `file:line`, the evidence you ran and the exact
required change; the three adjudications; an evidence appendix naming your
instrument paths and commands; and a **HARVEST** section (observed patterns,
repeated rejections, what Tasks 3-5 must carry). The controller is the single
writer into the house-knowledge files; you surface, you do not write.

Your final message carries the same in short form.
