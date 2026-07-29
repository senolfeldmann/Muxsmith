# Implementer brief - make every artifact-reach claim true, once

**Role:** fresh implementer. Model tier: mid (dispatch model: Opus 5). Effort:
xhigh. An independent reviewer grades your work.

## Preamble (binding)

- No session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**. You are the only writer in this tree.
- **Read the files, not a commit hash.** The tree is at `d9a4fa2`.
- zsh: `${PIPESTATUS[0]}` is empty. A bare `cp` is aliased interactive.

## Why this exists, and why it is ONE task rather than a fifth round of one-line fixes

Today the Linux release base moved from Ubuntu 22.04 to 24.04, raising the
minimum glibc from 2.35 to 2.39 - a product-reach change the owner took
knowingly. Three consecutive changes have now each repaired one text and left
another asserting the old reach, because the scope unit each time was a FILE
LIST while the thing that moved is a FACT that several texts assert
independently.

So this task does not chase members. **The rule below is the deliverable; the
enumerated sites are its application, and an independent reviewer's sweep over
all 265 candidate files produced that enumeration, so it is complete as of
`d9a4fa2`.**

## The rule

**Every row or sentence that tells a reader which systems an artifact runs on
states the requirement, not a distribution family that stands in for it - and
where a floor exists, the floor is what it states.** A routing phrase may stay
short, but it may not assert a reach the artifact does not have. Rows that
describe an artifact's FORM rather than its reach ("portable, CLI + GUI") either
gain the reach or say nothing that reads as reach.

## Files (EXHAUSTIVE) and the sites in them

- Modify: `docs/INSTALL.md` - the artifact list's three Linux rows:
  - the deb row, today `Debian/Ubuntu`: INCOMPLETE. The same phrase was retired
    from the release table one document over; make the two agree.
  - the rpm row, today `Fedora & co.`: **measurably FALSE** - the reviewer
    measured EL9 at `glibc-2.34-274.el9_8` and openSUSE Leap 15.6 at `2.38`,
    both below the floor, and both are "& co."
  - the AppImage row, today `any distro`: **FALSE**, and it is 14 lines above
    the floor paragraph, so the reader choosing a download meets the false claim
    first.
- Modify: `.github/release/draft-body.md` - two rows:
  - the rpm row, today `Fedora & co.`: same false claim as above.
  - the tar.gz row, today `portable, CLI + GUI`: its three Linux siblings now
    state reach and it does not, which reads as exemption; it carries the same
    floor.
- Modify: `.github/workflows/release.yml` - the pinning-policy comment's
  sentence saying the floor lives in "those two texts". It now lives in more.
  **Do not change two to three: phrase it so growth cannot falsify it**, which
  is the same ruling the owner gave the README's figures today. Nothing else in
  that comment or file changes - no pin, no step, no action SHA.

Nothing else. In particular `README.md`, `packaging/linux-tarball-README.txt`,
`renovate.jsonc`, `docs/ROADMAP.md` and the house-knowledge YAML files are NOT
yours: the first four are already true, and the tracker and ledger are the
controller's to write.

## What to get right

1. **Consistency across the two tables is the point.** `docs/INSTALL.md` and
   `.github/release/draft-body.md` describe the same seven artifacts. After your
   change, a reader comparing them must not find two different answers to "which
   systems run the rpm".
2. **Do not restate the floor in every row.** It is stated once per document
   already (INSTALL.md's floor paragraph; the release body's INSTALL.md link).
   Rows point at it or name it compactly.
3. **Every distribution version you name is a claim about the world.** Verify it
   before writing it - the figures above are the reviewer's measurements, and if
   yours disagree, report that rather than writing around it.
4. **The three non-Linux rows and the rpm-versus-deb split are untouched.**
5. **No `file:line` citation anywhere** - the rule was widened by owner ruling
   today to CI and configuration files too.

## Standing rules

- **No design latitude.** The rule and the site list are fixed; the wording is
  yours. A fork found on contact returns as NEEDS_CONTEXT with a decision memo.
- No edits to house-knowledge YAML, `docs/ROADMAP.md` or
  `docs/process-journal.md`.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.
- A claim about the tree is a COUNT: measure before writing "the only", "every".

## Verification bar

1. The full gate as `BUILDING.md` enumerates it, foreground, green - and state
   its limit: no gate part reads any of these three files.
2. **A completion check with a fired control:** after your edit, no Linux
   artifact row in either table asserts a reach the artifact does not have.
   Derive the check yourself, run it on the pre-state to see it fire, and run it
   on the end state.
3. A YAML parse of `release.yml` proving structure unchanged.
4. `git diff --stat` covering exactly the three files.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...`, deliberately unsigned.
- Stage explicitly by name, **never `git add -A`**.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
- Suggested subject: `docs: every artifact row states the reach it actually has`.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/reach-sweep-report.md`: per
site, before and after; your verification of every distribution figure you
wrote; the completion check with its fire; the gate result and its limit;
`git diff --stat`; numbered concerns; the commit hash.
