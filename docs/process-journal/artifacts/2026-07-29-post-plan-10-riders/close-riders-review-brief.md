# Review brief - the four close riders

**Role:** independent reviewer of four small text changes, commit `d9a4fa2`. You
did not write them. Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Preamble (binding)

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `d9a4fa2`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/ridersrev-independent/`
  (create it). Never re-run an instrument the implementer wrote.
- zsh: `${PIPESTATUS[0]}` is empty. A bare `cp` is aliased interactive.
- **Prove tree identity per FILE against blobs**, not by a clean `git status`.

## What the riders were for

Two decisions landed today. The Linux release artifacts moved from Ubuntu 22.04
to 24.04 because GitHub is retiring the 22.04 runner, which raised the minimum
glibc from 2.35 to 2.39 - a product-reach decision the owner took knowingly,
dropping Ubuntu 22.04 LTS and Debian 12. And the owner ruled that two
growth-prone figures leave the README, because the project's own process keeps
moving them.

The riders' brief is `.superpowers/sdd/close-riders-brief.md`; the report is
`.superpowers/sdd/close-riders-report.md`. **Evidence, not ground truth.**

The MEDIUM finding rider 1 answers is worth restating, because it decides how
strictly you grade placement: the shipped deb declares no `libc6` dependency, so
`apt install` succeeds on a system below the floor and the binary fails later
with nothing having warned the user. `docs/INSTALL.md` is the only channel that
can warn them.

## Dimensions

1. **Rider 1, placement and content.** The bullet had to leave the
   runtime-requirements list (whose intro invites deb/rpm readers to skip) and
   become a standalone statement, content unchanged. Verify the content really
   is unchanged (not merely similar), and rule on the placement - see
   adjudication 1.
2. **Rider 2, the release table.** Two rows changed. Verify every factual claim
   in the new routing text against the distributions' own package data - a row
   naming a distro version is a claim about the world. Check the untouched rows
   are byte-identical, and check whether the enumeration is now internally
   consistent (see adjudication 2).
3. **Rider 3, the Renovate comment.** One token. The rule's keys, values and
   stated reason must be unchanged, the file still valid, and the comment style
   identical to its siblings.
4. **Rider 4, the README paragraph.** Both figures gone, both claims intact, the
   register unchanged. Run your own search for surviving numerals in that
   paragraph, with a fired control, and read the paragraph for whether it still
   says what it said.
5. **The falsified-reference sweep, which is the highest-value dimension here.**
   Three of the implementer's four concerns are references that this change or
   its predecessor made false, in files outside the edited list. **Do not stop
   at its list.** Derive the set yourself: which texts in this repository assert
   the artifacts' reach, the glibc floor, or where the floor is documented?
   Sweep them and report every one that is now false or incomplete, including
   the ones nobody has mentioned.
6. **The no-work-needed check** over both the report and the brief.
7. **Verification quality.** Re-run the gate and the validator yourself. The
   report claims the gate proves nothing about these four files and measured
   that claim; check the measurement rather than the conclusion.

## Adjudication questions (one explicit verdict each, phrased in both directions)

1. **Where the floor paragraph landed.** The prescription said "immediately
   BEFORE" the list; the intro ends in a colon, so the implementer put the
   paragraph before the INTRO rather than between intro and bullets. Which
   placement serves the deb/rpm reader who is about to skip the list, and is the
   chosen one right?
2. **The tar.gz row.** Rider 2 gave the deb and AppImage rows explicit reach and
   left the tar.gz row ("portable, CLI + GUI") saying nothing about reach,
   although it carries the same floor. Is that consistent enough for a routing
   table, or does the table now imply the tar.gz is exempt?
3. **`release.yml`'s policy comment**, which says the floor lives in two texts
   where it now lives in three. Is that a factual defect worth a follow-up
   commit, or bookkeeping prose whose count nobody consumes?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/close-riders-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with `file:line`, evidence run, exact required change -
**and for the sweep in dimension 5, the complete list you derived, marked by
whether each site is true, false or incomplete today**; the three adjudications;
an evidence appendix; a HARVEST.

Your final message carries the same in short form.
