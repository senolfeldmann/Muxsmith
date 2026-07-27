# Task 3 report: Join the draft-body's wrapped regions (ruling 3)

**Commit:** `87c1dee` - "release: join draft-body wrapped regions onto single lines (Plan 8.5 ruling 3)"
**File touched:** `.github/release/draft-body.md` (only file staged/committed)

## Preconditions checked

- Tree was clean on `master` in the main worktree at start; Tasks 1 (`9460daf` + review-round fix `5060ef5`) and 2 (`50e08cd`) already committed, confirmed via `git log`.
- Read the current file before editing (Read tool, mandatory before Edit/Write). Its state matched the plan's "checked, not assumed" description exactly: lines 1-4 title+links (post-Task-1 wording already present on line 1), lines 6-9 runtime requirement, lines 11-19 the GFM artifact table (untouched target), lines 21-26 checksum paragraph, line 28 the trailing `---` separator. Trailing-byte check (`xxd`) confirmed the file ends `...case-insensitive).\n\n---\n` with no extra trailing blank line - preserved in the rewrite.

## Regions joined

Three wrapped prose regions joined onto single lines, exactly as the plan's verbatim end-state specifies:

1. Title + three OS install links (was 4 lines, lines 3-4 beginning `| [`) -> 1 line.
2. Runtime-requirement paragraph (was 4 lines) -> 1 line.
3. Checksum/verify-downloads paragraph (was 6 lines) -> 1 line.

The artifact table (7 data rows + 2 header/separator lines) was left untouched, one row per line, per the plan's explicit exclusion ("the artifact table is NOT a wrapped region and stays"). The `---` trailing separator is unchanged. Wording is byte-identical to the prior tree text except for the joins themselves (verified via `git diff`/`git show`, reproduced below).

## Counts measured (all via `command grep`, alias-proof)

| Check | Pre-edit measured | Plan's expected pre-edit | Post-edit measured | Plan's expected post-edit |
|---|---|---|---|---|
| `grep -c '^|'` | 11 | 11 | 9 | 9 |
| `grep -cE '^\| \['` | 2 | 2 | 0 | 0 |
| `grep -c '__VERSION__'` | 8 | (unstated, implicitly unchanged) | 8 | 8 |
| `grep -c '^---$'` | 1 | (unstated, implicitly unchanged) | 1 | 1 |

Post-edit counts were re-measured a second time directly against the committed blob (`git show HEAD:.github/release/draft-body.md \| grep ...`) after the commit landed, reproducing the same four values (9, 0, 8, 1) - the check is not a one-shot artifact of the working-tree file.

## Diff landed (verbatim, `git show 87c1dee -- .github/release/draft-body.md`)

Matches the plan's Step 2 replacement text exactly: three regions collapsed (12 lines removed, 3 lines added net across the title/links, runtime-requirement and checksum regions), table and separator untouched. `git diff` shown against the pre-Task-3 tree confirmed no other byte changed.

## Premise refuted / fork encountered (git mechanics, not plan content)

Between my `git add .github/release/draft-body.md` and my subsequent `git commit`, a concurrently running agent (working on the plan document itself, per the dispatch brief's stated concurrent-editor) executed its own `git commit` without a pathspec. Because both of our changes sat staged in the same shared index (main worktree, no worktree isolation per the plan's Task 3-4 sequencing section), that commit (transient SHA `86bfd69`, message "plan: amendment-log references become pointers, not enumerations (Task-2 review routing)") swept up my staged `draft-body.md` change alongside their own plan-file edit. This is NOT a content defect - `git show 86bfd69 -- .github/release/draft-body.md` showed my exact intended diff, byte for byte - but it violated Task 3 Step 4's contract of a standalone commit carrying the plan's specified message, and violated the dispatch's stated assumption "your file and theirs are disjoint."

I did not attempt any history surgery (no reset/rebase/amend) since the file content was correct and the affected commit belonged to another live process. The other agent self-corrected before I acted further: they rewrote their commit as a clean, single-file commit (`29ef17b`, same message, only the plan-doc path), which put my `draft-body.md` change back into the index unmodified. I verified the re-staged diff was still exactly my intended change (nothing else in the index) before running my own commit, which then landed cleanly as `87c1dee` containing only `.github/release/draft-body.md` with the plan's exact Step 4 message.

Net effect: no plan premise about the FILE content was wrong; the fork was a shared-index git race with the concurrent editor, self-resolved without any destructive action on my part. Flagging it per "every fork is closed" / "STOP and report a refuted premise" - worth the controller's attention if Tasks 1-4's serial/no-worktree design continues to run one implementer at a time overlapping with plan-document bookkeeping edits on the same index.

## Final state

- `git status --porcelain` clean.
- HEAD is `87c1dee`, one file changed (`draft-body.md`, 3 insertions/14 deletions), commit message exactly the plan's Step 4 text plus the required trailer.
- Not pushed (no task instruction to push at this stage; Task 4's preconditions push after the ten-part gate).
