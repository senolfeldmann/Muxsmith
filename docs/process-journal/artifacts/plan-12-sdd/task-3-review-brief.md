# Task 3 review brief (Plan 12): New creates a blank profile

**Your role.** Independent reviewer. You did not write this change. You grade
it, you do not fix it: a fix you would have made is a finding. You commit
nothing and edit no product artifact.

**Why this task carries more weight than its size suggests.** The owner's manual
QA pass is STOPPED on exactly this defect: the GUI can only open a profile and
never create one, so everything behind a profile is unreachable. This task is
what unblocks it. A test here that cannot fail is worse than a missing one,
because it will be read as coverage of the feature that resumes his pass.

**Ground truth, in this order:** the v1 spec section 8.2 as Task 1 amended it;
**the controller ruling `task-3-ruling-1.md`**, which authorises exactly one
deviation from the brief's fenced code and is silent on everything else; the
task brief; decision **D107** in the plan's `## Decision register` and the
plan's `## Authoring-time verification` seed measurement; the plan's
`## Global Constraints`; and the four house-knowledge files, cited by entry id.

**Read the ruling before the brief**, or you will report the one deviation as an
unrouted decision.

## What to read

| What | Path |
|---|---|
| The controller ruling (authority for the one deviation) | `.superpowers/sdd/plan-12/task-3-ruling-1.md` |
| The task's requirements | `.superpowers/sdd/plan-12/task-3-brief.md` |
| The implementer's report, including its NEEDS_CONTEXT memo | `.superpowers/sdd/plan-12/task-3-report.md` |
| The diff, with commit list and stat | `.superpowers/sdd/plan-12/review-10c819c..2cc0650.diff` |
| The decision this task implements | the plan document, `## Decision register`, D107, plus the seed measurement in `## Authoring-time verification` |

Do not read the whole plan file.

## The consolidated requirement set for THIS task

| # | Requirement |
|---|---|
| T3-a | A New action creates a blank profile in the editor and touches no file |
| T3-b | The seed is the measured one, produced fresh per call by a module-level factory - never a shared constant - and its one diagnostic is `empty-match-expression` at WARNING severity, so Save is enabled |
| T3-c | `currentPath`'s four non-path duties each move to their named replacement: `saveDisabled` drops it, the validation watcher gates on `sessionActive`, `doSave`'s re-guard drops it, the recents-section gate moves to `!model`. The save TARGET stays `currentPath` |
| T3-d | Save with no path opens the save dialog; there is NO separate Save-as action; the capture-before-the-dialog-gap discipline is intact and the recents write fires only when the path was NEWLY established |
| T3-e | New renders immediately before Open; both stay visible in every state |
| T3-f | The editor's empty state names both entry paths, and the diagnostics section is gated on `diagnostics.length` so no heading renders over nothing - in EVERY state, not only pre-session |
| T3-g | No "no diagnostics" line is added; `DiagnosticsPanel.vue` is not edited; Batch gains no New button |
| T3-h | The six fenced catalog values land byte-exact in both locales, with correct German orthography |
| T3-i | Both stale "45" comments are corrected, and the recount is recomputed FROM THE FILE, not copied from the plan |
| T3-j | Absence checks E1 and E2 each with their in-test fire |
| T3-k | Every pre-existing test passes unchanged |
| T3-l | The commit is unsigned, one trailer, pathspec-scoped to the four Files-list paths |
| T3-m | **Under the ruling:** exactly one token deviates from the fenced `doSave` - the first guard inside the `try` is `if (path === null)`. `const needsPath` and the SECOND `if (needsPath)` are untouched, and a comment at the site states why the two are deliberately not unified |

## Review dimensions

1. **Spec compliance** against the table.
2. **Quality**: are the comments true against the code they describe? Is the
   ordering inside `createBlank` - which the brief calls load-bearing - actually
   load-bearing as commented?
3. **`house`**: deviations from a recorded Tier-2 convention, with the entry id.
4. **Latitude, both forms**: an explicit permission, and the omission form (an
   unenumerated set in a normative position, a placeholder, a step requiring an
   invented name).
5. **The no-work-needed check**: wherever a passage concludes something is
   unnecessary, **run the claim that makes it unnecessary; do not weigh it.**
6. **Harvest**: dominant patterns and repeated rejections. You surface; you
   never write to the ledger.

## The tests are the thing to attack

Six new cases plus two absence checks ship here. **For each, ask whether it can
fail**, not whether it passes:

- **The two absence checks (E1, E2)** each claim an in-test fire. Verify the
  fire actually fires and that it exercises the same locator or counter as the
  absence half. An absence check whose fire uses a different expression proves
  nothing about the absence.
- **The seed test** asserts a warning-severity diagnostic renders and Save is
  enabled. Would it still pass if the seed silently changed to one producing an
  error? Say so either way.
- **The save-dialog cases** assert recorded IPC calls. Check that the assertions
  read the recorded ARGUMENTS and not merely that a call happened - `PICKED_PATH`
  is chosen distinct from every other path literal in the file precisely so an
  identity assertion cannot pass on a shared value; verify that property holds.
- **Mutation is welcome and is the strongest evidence you can produce.** If you
  mutate, build your own instrument at a path the implementer could not have
  used, restore the tree afterwards, and confirm the restoration by re-running
  the suite. Never execute a fixture the implementer left behind.

## Adjudication questions - one required verdict each

**Q1. The ruled deviation.** Is it implemented exactly as ruled and no wider -
first guard only, `needsPath` and the second guard untouched, the comment true
and located by symbol? And independently: is the ruling's own claim correct,
that the two conditions are equal at the first site because nothing writes
`path` between them?

**Q2. Two counts in the brief diverged from their own sources** - it said "four
candidate seeds" where the authoring section enumerates five, and the model-tier
table said "seven new tests" where Step 7 enumerates six. The implementer ran
five and built six. Rule whether those were the right resolutions, and whether
six cases actually cover the acceptance halves this task owns.

**Q3. The recount.** 49 ids in both locales is asserted. Recompute it yourself
from the files, and rule on whether the corrected decomposition comment is now
true - including the term that was missing before this task touched it.

## Your verdict

Write it to `.superpowers/sdd/plan-12/task-3-verdict.md` and return a short
summary.

It carries: an overall **spec compliance** verdict and an overall **task
quality** verdict (both required); findings graded Critical / Important / Minor,
located by symbol or test name, never by line number; your three adjudication
verdicts; your judgment on whether each new test can fail; your harvest; and
anything you could not verify, named with the reason.

Every quotation copied from the artifact; every number one you measured.

**Two environment facts that have cost agents real time in this session:** in
this shell `grep` is a function honouring `.gitignore`, so a rooted recursive
sweep silently skips ignored trees - use `command grep` or `git grep`. And
`${PIPESTATUS[0]}` reads blank here because this is zsh; capture exit codes with
`$?` directly.
