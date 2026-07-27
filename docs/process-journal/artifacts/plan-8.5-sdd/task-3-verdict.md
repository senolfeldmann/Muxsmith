# Task 3 verdict: Join the draft-body's wrapped regions (ruling 3)

**Commit graded:** `87c1dee` - "release: join draft-body wrapped regions onto single lines (Plan 8.5 ruling 3)"
**Reviewer:** independent (did not author the change). Model tier: mid (Opus 5), per `proc-03-model-assignment`.
**Verdict: APPROVED.** No major findings. Two minor findings, both in the report's prose, neither touching the artifact.

Ground truth used: the plan's Global Constraints, its "Owner steps" section and all of "## Task 3" (the verbatim post-state fence); `docs/ROADMAP.md` Plan-8.5 ruling 3 (:244-246) and the release-body finding entry (:580-585); the four `docs/*.yaml` house files by entry id. The implementer's report was read first and then verified rather than graded.

---

## 1. Fidelity to the carried end state - byte-exact, with the comparison proven to fire

The plan's post-state fence was extracted by structural anchor, not by a memorised line range (`proc-wrapped-prose-quote-grep`: fixed ranges produce phantom deltas once surrounding edits shift them). The fence boundaries were located with `command grep -n '^```'` on the plan file: opener at :514, closer at :532, so the carried content is exactly :515-531.

| pair | result |
|---|---|
| plan fence (:515-531) vs `git show 87c1dee:.github/release/draft-body.md` | `cmp` exit 0 |
| plan fence vs the working-tree file at HEAD | `cmp` exit 0 |

All three carry the same sha256 `cd363466...1fb492`, 17 lines, 1435 bytes.

**Negative control (four mutations, each had to fire, and did):**

| mutation | `cmp` result |
|---|---|
| one byte case-flipped inside the checksum line | differ at byte 1417, line 15 |
| trailing newline stripped | EOF after byte 1434 |
| one extra trailing blank line appended | EOF on expected after byte 1435 |
| trailing space appended to line 1 (invisible in a diff) | differ at byte 377, line 1 |
| unmutated pair re-run afterwards | exit 0 |

The two whitespace-only controls matter here: a `diff -w` style comparison would have passed all three of the byte-level defects this task could plausibly have introduced (stray trailing space on a joined line, lost or doubled final newline). `cmp` catches them, and it was watched catching them.

**Independent check that the joins changed structure only, not prose.** Byte-equality with the plan proves the implementer transcribed the plan; it does not prove the *plan's* fence preserved the tree's wording. So the pre-state blob (`bbe23f7`, the file at `50e08cd`/`29ef17b`) and the post-state blob (`f3095a6`) were both whitespace-normalised (`tr -s ' \t\n' ' '`) and compared:

- normalised streams: 1431 bytes each, `cmp` exit 0. Word count 149 before, 149 after.
- controls: a one-word case change (`recommended dependency` -> `recommended Dependency`) fired at byte 528; a dropped word (`case-insensitive` -> `insensitive`) fired at byte 1409.

Not one word of prose was added, dropped or altered. The plan's own claim ("wording is byte-identical to the current tree text except the joins and Task 1's line-1 rewording") holds at measurement, not on assertion.

**The three regions, measured against the pre-state file (28 lines):** 1-4 title + three OS links (lines 3-4 begin `| `), 6-9 runtime requirement, 21-26 checksum. That is exactly the plan's "checked, not assumed" description, and it confirms the plan's own correction-table row 1 (line 2 begins `before first launch:`, not `|`; only 3-4 are `|`-leading). Post-state: those three regions are one line each.

## 2. The table and the placeholder survived

**Artifact table, byte-identical.** Extracted from both blobs by content anchors (`/^| Artifact | For |$/` to the `tar.gz` row), not line numbers:

- 9 lines, 493 bytes, sha256 `0173b8a9...0745c` on **both** sides; `cmp` exit 0.
- Negative control: `Fedora & co.` -> `Fedora & Co.` in a copy fired at byte 351, line 7.

Header row and the `|---|---|` delimiter row are both intact and adjacent, so the GFM table construct is unbroken.

**Placeholder and separator.**

| check | pre (`bbe23f7`) | post (`f3095a6`) | plan expects post |
|---|---|---|---|
| `grep -c '__VERSION__'` (lines) | 8 | 8 | 8 |
| `__VERSION__` **occurrences** (`grep -o \| wc`) | 8 | 8 | (unchanged) |
| `grep -c '^---$'` | 1 | 1 | 1 |
| `grep -c '^\|'` | 11 | 9 | 9 |
| `grep -cE '^\\\| \['` | 2 | 0 | 0 |

Occurrences were counted separately from lines, because `grep -c` counts matching lines and would hide a doubled or lost token on a single line - the exact defect a whole-file rewrite can introduce quietly. Both counters were fire-verified on a mutated copy: injecting a second `__VERSION__` into line 1 moved the occurrence count to 9; breaking the separator to `--- ` moved `^---$` to 0.

The `^\| \[` -> 0 absence check is fire-verified by its own invocation, not a neighbouring one: the identical pattern returns 2 on the pre-state blob. Same for the three-anchor check below (0 on pre, 1 on post).

## 3. Does the fix address the defect? - mechanism-neutral, and yes

I do **not** claim ruling 3 accepted. The acceptance is owner step **O3** on the rendered draft; nothing below is a substitute for it, and no machine check in this task can be.

What can be established from the artifact:

- The defect's necessary input was the **newline inside the region**. Two mechanisms are consistent with the owner's rendered screenshot: GitHub's release-body pipeline turning intra-paragraph newlines into visible breaks, or the leading `|` opening a block-level construct in that pipeline. I cannot distinguish them from here, and I do not need to: the joined line contains no intra-region newline **and** no line-initial `|` (the pipes are now mid-line ` | ` separators). Both candidate mechanisms lose their trigger. The observable must change.
- The owner's own evidence rules out the third hypothesis. Under pure CommonMark paragraph continuation, lines 1-4 would already have rendered as one paragraph, which is not what the screenshot showed - so whatever renderer produced the split consumed the line break, and the line break is gone.
- **No new construct was introduced.** The joined title line cannot become a GFM table: a table needs the delimiter row on the immediately following line, and the following line is blank. The artifact table's own header/delimiter pair is untouched (section 2).
- **The consumer is line-agnostic.** `git grep 'draft-body'` over the tracked tree outside `docs/superpowers/` and `docs/process-journal` returns exactly one functional consumer: `.github/workflows/release.yml:217`, `sed "s/__VERSION__/${version}/g" ... >> body.md`. A global per-line substitution has no line-count, line-number or line-length dependency, so the joins cannot disturb it. (The grep's control: the same pattern returns 13/19/5 hits in the plan-8 plan, the plan-8.5 plan and the design.)
- **Composed-body simulation** (the Task 4 RR2 input, run here so the controller has it before dispatch): `cat rehearsal-banner.md > body.md; sed s/__VERSION__/0.1.0/g draft-body.md >> body.md`, version recomputed from `Cargo.toml`'s `[workspace.package]`, never hardcoded. Result: `^\| \[` count 0, `__VERSION__` count 0, and exactly **one** line carrying all three `INSTALL.md#windows` / `#macos` / `#linux` anchors (control: 0 on the pre-state file). The banner ends `...draft.\n\n---\n`, so the joined title line follows a thematic break with a blank line before it - no gluing of banner and title.

Left for O3 by construction, and correctly so: whether the rendered pipes between the three links read the way the owner wants. That is the plan's specified end state, i.e. the owner's wording, not the implementer's choice.

## 4. No claim beyond the machine half

- **Commit message:** describes the structural change only ("join draft-body wrapped regions onto single lines"). It is the plan's Step 4 text verbatim; no rendering or acceptance claim.
- **File:** carries no claim.
- **Report:** scanned for `render|github\.com|acceptance|O3|proves the fix|defect fixed|verified the fix|now displays|will render` - **no hits** (control: the same pattern returns 22 hits in the plan file, so the pattern is not malformed). The report confines itself to regions joined, counts measured and the commit landed.

Clean on this dimension. The report also does not route O3, which is correct: routing the owner steps is Task 4 Step 8's duty and the plan close's, not Task 3's.

## 5. Commit hygiene and house conformance

| item | observed | house entry |
|---|---|---|
| paths touched | exactly one: `.github/release/draft-body.md` (control: the same `diff-tree` invocation lists 7 paths on `9460daf`) | plan Task 3 "Files" |
| numstat | 3 insertions, 14 deletions | - |
| message | byte-exact against the plan's Step 4 fence (`cmp` exit 0 on the raw commit object body; control mutation `Fable`->`Opus` fired at byte 104) | plan Task 3 Step 4 |
| trailer | `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, matching the four sibling plan-8.5 commits | SI-4 |
| signature | `%G?` = `N`, unsigned | `proc-05-commit-signing` |
| author/committer | `Şenol Feldmann <senol.feldmann@gmail.com>`, consistent with siblings; `Ş` correct in the author field | D86 typography exception |
| typography in message and file | zero non-ASCII bytes; zero AI-tell glyphs (em/en dash, curly quotes, ellipsis, NBSP, U+2212). Controls: the same pattern hits `Ş` in `LICENSE:3`, and an injected em-dash in a copy of the file scored 1 | Global Constraints "Typography" |
| scratch not committed | `.superpowers/` is git-ignored (`.gitignore:2`); control: `git check-ignore` correctly exits 1 on the tracked `draft-body.md` | - |
| counts recomputed | every count in the report is reproduced from its enumeration here; one arithmetic slip found, see M1 | `proc-normative-count-recomputed` |
| absence checks fired | `^\| \[` -> 0 fire-verified by the same invocation returning 2 pre-edit, by the implementer and again independently here | `proc-verification-step-must-be-falsifiable` |
| alias-proof forms | report states `command grep` throughout; this review used `command`-prefixed `cp`/`rm`/`sed`/`grep` for every mutate-and-restore | `proc-noninteractive-file-ops-in-agents` |
| push state | not pushed; master is 11 commits ahead of `origin/master` | plan Task 4 precondition 2 (controller pushes) |
| latitude | no design-latitude clause, no unenumerated normative set in the change | `proc-latitude-clause-boundary` |

Working tree at review time: `git status --porcelain` empty.

## 6. End state after the shared-index race - correct, nothing lost

Not re-litigated; the incident is already ledgered under `concurrent-writers-need-pathspec-scoped-commits` (`docs/decision-ledger.yaml`, occurrence dated 2026-07-28). Only the end state was verified:

| blob of `.github/release/draft-body.md` | commit |
|---|---|
| `bbe23f7` (pre-Task-3) | `50e08cd`, `29ef17b` |
| `f3095a6` (post-Task-3) | `86bfd69` (transient), `87c1dee`, `338e779`, HEAD |

- **`87c1dee` carries the change**: its blob is `f3095a6`, byte-identical to the plan's post-state (section 1).
- **No unrelated path rode along**: `git diff-tree -r 87c1dee` lists one path.
- **Nothing lost in the repair**: `git diff 86bfd69 87c1dee` is empty - the transient commit's tree and the final tree are identical, so the repair round-tripped the content exactly. The other agent's own plan-doc edit also survived intact (`git diff 86bfd69 29ef17b -- <plan file>` empty).
- **The transient commit is unreferenced**: `git branch -a --contains 86bfd69` returns nothing, and the same invocation returns `* master` for `87c1dee` - so the empty result is a measurement, not a malformed check.
- `29ef17b` restored the file to its pre-Task-3 blob before `87c1dee` re-applied it, which is why the final history contains the change exactly once.

The implementer's handling was correct on the point that mattered: no history surgery on a commit belonging to a live process, the re-staged diff re-verified before committing, and the fork disclosed rather than absorbed.

---

## Findings

**Major:** none.

**Minor**

- **M1 - report arithmetic.** The "Diff landed" section states "12 lines removed, 3 lines added net". Measured: **14** removed, **3** added, net **11** (28 lines -> 17). `git show --numstat` reports `3 14`, and the report's own "Final state" section states `3 insertions/14 deletions` - so the report contradicts itself, and 12 matches neither figure. A stated number is a measurement (`proc-normative-count-recomputed`). The artifact is unaffected; this is report hygiene. Fix is one word if a fix round runs, otherwise carry it to the plan close as noted.
- **M2 - a promised verbatim reproduction that is not there.** The report says "verified via `git diff`/`git show`, reproduced below", and the following heading reads "## Diff landed (verbatim, `git show 87c1dee -- ...`)" - but the section contains a paraphrase, no diff text. Same class as M1: the claim is about an artifact the reader cannot see. Either paste the diff or drop "verbatim"/"reproduced below".

**Observations (no action)**

- The plan states post-edit expectations for `__VERSION__` and `^---$` but no pre-edit expectations. The implementer measured both pre-edit anyway, which converts two self-verifying presence checks into fired ones. Correct instinct, noted as such.
- The report's fork disclosure is the right shape: refuted premise reported, no unilateral repair, no claim that the incident was harmless beyond what it verified.
- For the controller, ahead of Task 4: the composed-body simulation in section 3 already satisfies RR2's shape checks against the committed template (`^\| \[` 0, `__VERSION__` 0, all three anchors on one line). That is the template-side half only; RR2 still has to run against the real draft body the workflow produces.

## My own before/after counts

Measured directly from the two blobs, not copied from the report.

| measurement | before (`bbe23f7`) | after (`f3095a6`) |
|---|---|---|
| lines | 28 | 17 |
| bytes | - | 1435 |
| words | 149 | 149 |
| `grep -c '^\|'` | 11 | 9 |
| `grep -cE '^\\\| \['` | 2 | 0 |
| `__VERSION__` lines | 8 | 8 |
| `__VERSION__` occurrences | 8 | 8 |
| `grep -c '^---$'` | 1 | 1 |
| artifact table (lines / bytes / sha256) | 9 / 493 / `0173b8a9...` | 9 / 493 / `0173b8a9...` |
| whitespace-normalised stream (bytes) | 1431 | 1431 |
| diff of the commit | 14 removed | 3 added |

**APPROVED.** The committed file is byte-identical to the plan's carried post-state, the table and the placeholder surface are provably untouched, the fix removes the defect's necessary input under every candidate rendering mechanism, nothing is claimed beyond the machine half, and the post-race end state is correct with nothing lost. Ruling 3's acceptance remains owner step O3.
