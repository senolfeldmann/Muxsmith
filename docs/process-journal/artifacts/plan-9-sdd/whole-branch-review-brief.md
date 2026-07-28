# Plan 9 whole-branch review brief

**Role:** independent whole-branch reviewer closing Plan 9 - core/orchestration
hoists plus the planner seam. You wrote none of this code and reviewed none of
its tasks. **Model tier: top** (the one role this project's tiering reserves the
strongest model for). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Why this review exists and what it is for

Task reviews grade one task against its own brief. This one grades the BRANCH
against the spec and the design: the cross-task breaks no single task review
could see, the seams where one task's assumption met another task's change, the
design entries whose implementation is spread across tasks, and the question
each task review had to take on faith - does the whole thing, assembled, do what
the plan promised.

The house record says this is where the only cross-task breaks have ever been
caught, including a run-path break on a platform no local run exercises. Read
adversarially. A finding you can prove beats ten you can argue.

## Scope

The branch is `9bbe53d~1..HEAD` on `master`: **41 commits, 47 files, roughly
2976 insertions and 576 deletions**, spanning the seven tasks of Plan 9, five
amendments with their four-eyes rounds, the fix rounds, and the controller's
house-knowledge and tracker commits. `9bbe53d~1` is `629dc64`.

The tree is at HEAD, clean, and the full ten-part gate plus `ledger-lint` ran
green foreground before the push. Do not take that on trust - re-running it is
part of your job.

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.**
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/wbr-independent/`
  (create it). Never re-run an instrument another agent wrote, never a shared
  default path, never a path a report or verdict names. Any absence check needs
  its own fire; the local `grep` is **ugrep 7.5.0**, where `\b` plus bounded
  repetition under `-E` returns zero silently - use `-P` or a script.
- The Playwright suite runs against BUILT bundles: `pnpm test:e2e` regenerates
  them, a bare `pnpm exec playwright test` does not. Any frontend mutation you
  fire needs the rebuild between edit and run, and your evidence must show it.
- If you mutate anything, restore non-interactively (`git checkout --`, never a
  bare `cp` - it is aliased interactive here) with a baseline taken first, and
  prove the restore. Leave the tree clean at HEAD.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - the v1 spec. This
   plan amended eight of its passages (S-1..S-8); the spec is authoritative over
   the design and the plan on any conflict.
2. `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md` -
   D91 to D105 **and its `## Amendment log` at its current state**. Every
   D-entry and every amendment-log entry binds; the pointer is the contract, not
   an enumeration of it. Amendment 5's rider under D104 re-cut one assertion
   mid-execution.
3. `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md` - the
   seven tasks as amended five times, their acceptance and coverage maps, and
   the eight acceptance observables.
4. `docs/ROADMAP.md`'s Plan-9 anchor, including its IN/OUT lists and the
   scope-correction paragraphs.
5. The four house-knowledge YAMLs (`docs/product-boundaries.yaml`,
   `docs/conventions.yaml`, `docs/process-conventions.yaml`,
   `docs/decision-ledger.yaml`). Cite ids; re-verify any `:line`.

The task reports and verdicts in `.superpowers/sdd/plan-9/` are evidence, not
ground truth. They are also the record of what each task's reviewer already
ruled - do not re-litigate a settled adjudication, but do check whether a
ruling's premise survived a later task.

## Dimensions

1. **The eight acceptance observables.** Walk each one to a producing test that
   exists and runs today, and walk each observable's HALVES where it has two
   sides (rendered and persisted, CLI and GUI, returned and logged). An
   observable whose named producer covers one side only is a finding - that
   exact shape cost this plan a blocking round already.
2. **Design coverage.** Walk D91 to D105 section by section against the tree.
   For each: is it implemented, is it implemented as written, and where an
   amendment re-cut it, is the amended form what landed. A D-entry with no
   corresponding change is a finding even if no task was assigned it.
3. **Spec amendments S-1..S-8.** Each landed, each says what the design fenced,
   and the spec does not now contradict itself - a spec amendment sweeps the
   spec for self-contradiction, and this plan made eight.
4. **Cross-task seams - the reason this review exists.** Task 1 built the
   planner seam; Task 2 hoisted `run_batch` into core and deleted a src-tauri
   seam; Task 3 carried the worker-panic payload end to end; Task 4 added a
   config-time error; Task 5 centralized diagnostic ordering and re-keyed a
   frontend fetch; Task 6 made a view mountable and hoisted a test helper. Look
   hardest where one task's output is another's input, and at the four files
   more than one task edited. Two individually correct changes can conflict
   semantically while both branches test green.
5. **The Windows and macOS paths.** No local run exercises them. Anything in
   this branch touching process spawning, path handling, runs-root resolution or
   the CLI's exit codes gets read for platform assumptions rather than run.
6. **The gate, re-run by you**, all ten parts foreground plus `ledger-lint`,
   and every aggregate recomputed from its enumeration rather than quoted:
   `cargo test --workspace` 39 `test result:` lines all ok, `pnpm test:e2e` 68
   passed, `check:i18n` 212 catalog ids, `ledger-lint` 515 entries.
7. **House dimension.** The branch against the Tier-2 files, and the ledger
   entries this plan created or amended - are the statements true of the tree
   they now describe? Several were written mid-plan from a single measurement;
   a statement that has since gone stale is a finding.
8. **The no-work-needed check**, standing: wherever a report, verdict or
   design passage concludes that a guard, an enumeration, a test or a check is
   unnecessary, run the premise that makes it unnecessary. Do not weigh it.
9. **Latitude, both forms**, across the branch: an unenumerated set in a
   normative position, a placeholder, a name someone had to invent.

## What is already known and NOT yours to re-open

These are settled; check that they held, do not re-decide them:

- Task 7's experiment landed on D105's anomaly branch. The guards stay. Its
  reviewer measured why the instrument could not answer the question. The
  ledger entry for that outcome is a controller close action with an open owner
  question; it is not a review finding.
- Amendment 5 ruled item 2's second assertion to the control's absence paired
  with a positive bearer; both the amendment and Task 6 were independently
  reviewed and approved.
- Three items are already routed to the plan close and need no finding from
  you, though a MEASUREMENT that sharpens any of them is welcome: D102's
  unguarded scope boundary, BatchView's else-branch text, and the three
  overclaiming assertion messages in `dry_run_cli.rs`.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/whole-branch-review-verdict.md`:
verdict (READY / NEEDS_FIXES) with the justification stated as what you
verified and how; numbered severity-tagged findings with file:line, the
evidence you ran and the exact required change; an evidence appendix naming
your instrument paths and every command you ran; and a HARVEST for the
controller including anything the close must carry and any observation worth a
ledger entry.

If you find nothing blocking, say so plainly and say what you did to earn that
conclusion - a READY verdict with a thin evidence trail is worth less than a
NEEDS_FIXES with a thick one.
