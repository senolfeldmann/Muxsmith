# Task 5 implementer brief - Plan 10

**Role:** fresh implementer for Plan 10, Task 5 (W2: the comment line-citation
sweep - every source comment that locates code by a line number is rewritten to
name the symbol instead, under the owner's
`comments-locate-by-symbol-never-by-line-number` ruling). Model tier: mid
(dispatch model: Opus 5). Effort: xhigh. An independent reviewer grades your
work afterwards; the controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools. Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.** Tasks 1 through 4 have landed, and
  Task 4 edited `README.md`, which several of your sites cite.
- Shell hazards, both measured here: a bare `cp` is aliased interactive and
  blocks on overwrite; this shell is **zsh**, where `${PIPESTATUS[0]}` is empty
  (bash-only, zsh spells it `$pipestatus[1]`).

## What to read first

1. The plan,
   `/home/senol/Git/Muxsmith/docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`:
   the **Global Constraints**; the **Authoring-time verification** section's
   corpus block (both expressions, their pasted results, and the union);
   **Task 5** in full - the Files list with its per-file A/B markers, the
   "Why expression B's four sites are IN" note, Steps 1 through 5, "Must not
   decide"; and acceptance rows **W2-a, W2-b, W2-c**.
2. `.superpowers/sdd/plan-10/plan-brief.md`, section 4's **W2** item.
3. The Tier-2 entry **`comments-locate-by-symbol-never-by-line-number`** in full
   (grep the id in `docs/conventions.yaml`), including its handle and its
   **SCOPE BOUNDARY** sentence. That entry is the transformation's contract.
4. `docs/ROADMAP.md`'s **"Docs accuracy"** stale-citation entry with its RULED
   block.
5. **`README.md` as Task 4 committed it** - three of your sites must name its
   section heading and bullet text rather than a line span, and those anchors
   cannot be named without reading the shipped file.
6. **Both files named `run.rs`** - `crates/muxsmith-cli/src/commands/run.rs` and
   `src-tauri/src/run.rs` - because Step 2's disambiguation bullet turns on
   which of the two an existing comment means, and that cannot be decided
   without opening both.

## Scope

**Files (EXHAUSTIVE): the sixteen the plan lists**, subject to Step 1's
re-measurement, which is ground truth if it differs. **In every one of them the
edit is COMMENT TEXT ONLY.** Not one line of code, markup or test logic changes
- that is what makes the gate's green run meaningful evidence here.

- A hit **outside a comment** returns as NEEDS_CONTEXT: this task may not touch
  code, so a code-line hit would make the green state unreachable within its own
  constraints.
- A hit **in a file not on the list** returns as NEEDS_CONTEXT: the Files list
  is exhaustive.
- **No file under `docs/` is edited.** The convention's scope boundary separates
  by the artifact DOING the citing, not by the artifact cited: a source comment
  pointing INTO a design document is still a source comment, which is why
  expression B's three design citations are swept rather than exempted.

## Cross-task constraints from Task 4, verbatim (the controller verified each on the tree)

1. `crates/muxsmith-cli/tests/run_live.rs` carries, above the `--json`
   assertion, the comment `per README.md:91 "every command takes --json"`.
   **Task 4 corrected exactly that README claim, because it is false for
   `schema`. The QUOTATION is therefore dropped, not re-anchored**: the Task-4
   reviewer measured that the quoted phrase now exists nowhere in the README
   (`grep -c` returns 0, fired against a present phrase returning 1), and line
   91 is a different bullet today. Name the "What you get" section's
   "Scriptable everything" bullet instead. This is the one site where the
   ruling's mechanical handle - replace the number with the symbol - is not
   sufficient on its own. If the truthful rewrite would change what the comment
   ASSERTS about the code under it, return NEEDS_CONTEXT with the options rather
   than deciding.
2. The two `run_live.rs` comments citing `README.md:71-78` name the README's own
   anchor as Task 4 committed it, never a span. Unit note, because the numbers
   in circulation compare two different things: `71-78` was the FENCE span;
   post-task the fence is 78-85 and the content 79-84. You delete the span
   either way, so this only matters if you were tempted to update it instead.
   The recipe bytes are unchanged (177 bytes, identical to the test literal).
3. The corpus was re-measured with fired controls in three consecutive reviews
   during Tasks 1-3 and did not move: 20 lines / 13 files (expression A),
   4 / 4 (expression B), union 24 / 16. Re-measure anyway; that is your Step 1.

## The transformation, from the ruling's own handle

"Replace the number with the symbol the line sits in; where no symbol names it,
name the nearest one plus what you mean ('the third arm of `scalar_eq`')."
Naming the FILE stays normal and wanted. Five rules make this a sweep rather
than a judgment call, all from the plan:

- **Historical citations lose their numbers too.** "pre-fix: panics at
  `report/json.rs:44`" becomes "pre-fix: panicked in `batch_document` while
  building the `Set` plan value". **Nothing is classified as live-versus-
  historical**; the distinction disappears rather than being made.
- **Ambiguous file references are disambiguated while you are there** (the two
  `run.rs` files). Where a file name survives the rewrite, it survives as a path
  unambiguous in the repo.
- **The rewrite unit is the COMMENT, not the matched line.** A citation can
  continue onto a line neither expression matches - `suggestions.rs`'s
  `planner.rs:1812,` finishes on the next line with a bare `:1817`. Rewrite the
  whole comment.
- **Bare spans lose the span and keep what they already say.** The three
  design-document citations name their D-entry already (D48, D44, D45); the
  surviving text is the identifier the comment itself supplies. Nothing is
  invented.
- **Read the cited code before naming its symbol.** Several citations are
  already stale - `suggestions.rs` cites `planner.rs:1812, :1817` for
  `delta_for`'s two exact-bearing arms while `delta_for` begins at `:1820` at
  HEAD - so naming the symbol from the cited LINE would name the wrong symbol.
  Name the symbol the comment MEANS, verified by opening the target.

## Standing rules

- **No design latitude**, in either form. The corpus is measured, not chosen;
  the transformation is the convention's handle; expression B's four sites are
  IN by controller ruling and are not re-argued at the keyboard.
- **No task edits any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md`.
- **Every observed value pasted from the run that produced it.**
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.
- A claim about the tree is a COUNT: measure before writing "the only",
  "every", "no X exists".

## Verification bar

1. **Step 1 first, before any edit**: both expressions run and both outputs
   pasted in full. Expression B runs PER FILE with the `file:line:` prefix
   stripped before the second filter - the one-pipeline form matches its own
   prefix and reports a clean tree that is not clean. That failure was observed
   at plan-authoring, which is why the form is prescribed.
2. **Two absence checks, each with its own fire.** Both Step-1 commands must
   return NOTHING on the end state, and each command's own Step-1 run on the
   pre-state IS its fire. Paste the pre-state counts and the empty end-state
   results.
3. **The full gate as `BUILDING.md` enumerates it**, foreground, green. Since
   this task changes comment text only, `pnpm lint`, `pnpm build`,
   `pnpm test:e2e` and `cargo test --workspace` are behaviour-preserving by
   construction; any failure is a real finding -> NEEDS_CONTEXT. A
   `FAIL BUILDING.md: ...` line means something wrote to that file outside its
   owner's Files list: defect signal, not a local fix.
4. `git diff --stat` covers exactly the sixteen files (or the re-measured set);
   anything else is a defect signal.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply here. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...`, agent commits deliberately unsigned.
- Stage explicitly by name, **never `git add -A`**. The pathspec set is fenced
  in Step 5 and was verified SET-EQUAL to the Files list at plan-authoring.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-5-report.md`:

- Status: DONE / DONE_WITH_CONCERNS / BLOCKED / NEEDS_CONTEXT.
- Step 1's two pasted measurements, with the corpus stated as lines and files
  per expression and as a union.
- **A per-site table**: file, the comment before, the comment after, and the
  symbol you named plus how you verified it points at what the comment MEANS.
- The three README-citing sites, with the anchors you used and why.
- The two absence checks with their fires and their empty end states.
- Full gate result; `git diff --stat`.
- Divergences and judgment calls, each named.
- Numbered concerns a reviewer can rule on yes/no.
- What you surface for the controller.
- Commit hash and `git show --stat`.
