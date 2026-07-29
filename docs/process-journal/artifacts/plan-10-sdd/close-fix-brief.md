# Plan-10 close fix wave - implementer brief

**Role:** fresh implementer for the close fix wave of Plan 10. Three findings
from the whole-branch review, all written out to the line. Model tier: mid
(dispatch model: Opus 5). Effort: xhigh. The whole-branch reviewer, resumed,
judges your delta.

## Preamble (binding)

- Never call session-relocation tools. Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.**
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty (bash-only; zsh spells it
  `$pipestatus[1]`). A bare `cp` is aliased interactive and hangs on overwrite.

## Context you need and cannot see

Plan 10 is the last planned product package before Muxsmith's 1.0 tag: five
serial tasks, all executed, reviewed and pushed; CI green on the head SHA. The
whole-branch review at the plan close returned READY_WITH_MINORS with three
repairs to land before the package closes. The SDD scratch has already been
salvaged into `docs/process-journal/artifacts/plan-10-sdd/` and the house-YAML
citations re-pointed at it, which matters for finding 3 below.

The full verdict is `docs/process-journal/artifacts/plan-10-sdd/whole-branch-verdict.md`
- read the three findings there rather than working from this brief alone.

## Files (EXHAUSTIVE)

- Modify: `README.md` (two regions: the exit-code sentence, and the verdict
  figure in the "How this got built" paragraph)
- Modify: `e2e/smoke.spec.ts` (comment text only)
- Modify: `src/views/EditorView.vue` (comment text only)

Nothing else. No code, no markup, no test logic - every edit in the two source
files is comment text, which is what makes the gate's green run meaningful
evidence here.

## The three findings

**Finding 1, `README.md`, the exit-code sentence.** The sentence opens
"Interrupt any of them", and *them* has no antecedent inside its paragraph: the
only `command` token before the pronoun sits inside "command-line" as an
adjective, and the nearest bindable plural is "your scripts", which is a
coherent but wrong reading. **The repair is fixed:** the clause becomes
`Interrupt any subcommand with Ctrl-C`. Keep the rest of the sentence as it
stands.

**Finding 2, the two ragged wraps.** `e2e/smoke.spec.ts` and
`src/views/EditorView.vue` each carry a comment line left ragged mid-sentence by
the line-citation sweep (the verdict names both sites). Re-wrap each comment so
no line breaks mid-phrase, matching the surrounding comment style in each file.
No wording change - the words stay, only the line breaks move.

**Finding 3, `README.md`, the verdict figure.** The paragraph states a count of
files under `docs/` with `verdict` in the name. It was true when written and the
plan-10 salvage has just falsified it, because the salvage added verdict-named
files under `docs/`. **Re-measure it against the current tree and write the new
figure**, using the unit the sentence itself names. The measuring command the
package established:

```bash
git ls-files 'docs/*' | grep -icE '/[^/]*verdict[^/]*$'
```

Three properties the package verified about that figure, all of which must still
hold after your edit, and each of which you re-run and paste:

- every match is markdown (the same expression with `-v '\.md$'` returns nothing);
- no review BRIEF is caught by it (the same list filtered for `brief` returns
  nothing);
- **both plausible readings of the sentence's unit agree** - basename and full
  path return the same number, with an empty set difference. That convergence is
  what makes the sentence re-derivable by a later reader; if the two readings
  have diverged, do not paper over it: report it and return NEEDS_CONTEXT.

Do not change the unit the sentence names, and do not turn the figure into a
range.

## Standing rules

- **No design latitude.** The three repairs are written out; anything else you
  find returns as NEEDS_CONTEXT with a decision memo rather than being fixed.
- **No new `file:line` citation anywhere** - the package just swept that class
  out of the tree's source comments, and its own close must not create a member.
- **No task edits any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md`.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.
- The README's register is the owner's sell-tone, a case-scoped exception
  recorded on its ROADMAP entry. Match the surrounding voice.
- Every observed value in your report is pasted from the run that produced it.

## Verification bar

1. The three measurements for finding 3, pasted with their commands.
2. **The full gate as `BUILDING.md` enumerates it**, foreground, green, before
   the commit. A `FAIL BUILDING.md:` line would mean something wrote to that
   file outside this wave's Files list: defect signal -> NEEDS_CONTEXT.
3. `git diff --stat` covering exactly the three files.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...`, agent commits deliberately unsigned.
- Stage explicitly by name, **never `git add -A`**.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
- Suggested subject: `docs+comments: the whole-branch review's three close repairs`.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/close-fix-report.md`:
per finding, what you changed and the evidence; the three pasted measurements;
the gate result; `git diff --stat`; numbered concerns; the commit hash.
