# Wording pass, fix round: four documentation defects + one design bookkeeping entry

The independent review of the owner's wording pass returned APPROVED with one
MEDIUM and four LOW findings, all in `.superpowers/sdd/plan-8/owner-wording-verdict.md`.
Read that verdict first - it carries the measurements behind each finding.
Four of them are text defects in shipped documentation and one is design
bookkeeping. You fix all five.

Repo: `/home/senol/Git/Muxsmith`, master, main worktree, absolute paths. No
branch, no worktree, no session-relocation tools.

## 1 (MEDIUM). The checksum instruction fails loudly for the normal case

`SHA256SUMS` lists seven files. A user who downloaded ONE artifact - the
ordinary case - runs the documented command and gets `FAILED open or read`
for the six absent files plus a non-zero exit. That is indistinguishable at a
glance from the tampering the check exists to detect, and it is the first
thing a new user does. The reviewer measured it on both the Linux and macOS
commands.

`--ignore-missing` is the fix and exists on both: coreutils since 8.25, and
documented in `shasum`'s own usage text. Add it to both commands, in
`docs/INSTALL.md` and in `.github/release/draft-body.md`:

- `sha256sum --ignore-missing -c SHA256SUMS` on Linux
- `shasum -a 256 --ignore-missing -c SHA256SUMS` on macOS

The Windows `Get-FileHash` form is unaffected - it already names a single
file.

**Verify before writing, because this is a safety instruction and a wrong one
is worse than none:** that both commands accept the flag as spelled, that
with a single artifact present they exit 0 and report that one file OK, and -
this is the part that matters - that a TAMPERED file still fails under
`--ignore-missing`. A flag that suppressed real failures alongside missing
ones would be a catastrophic edit. Run the tamper control; do not reason
about it.

## 2 (LOW). Uppercase versus lowercase on Windows

`Get-FileHash` prints uppercase hex; `SHA256SUMS` carries lowercase. The
instruction says to compare the hash "against that file's line in
`SHA256SUMS`" and never says the comparison is case-insensitive, so a careful
user sees two strings that do not match. Add that clause to the Windows half,
in both files, in as few words as carry it.

## 3 (LOW). The Windows example hard-codes one of two artifacts

The Windows example names `muxsmith-<version>-windows-x86_64.msi` while the
file's own Windows section documents two artifacts (x86_64 and arm64).
`.github/release/draft-body.md` already solves this with a generic
`<file>` placeholder. Make `docs/INSTALL.md` consistent with it.

## 4 (LOW). The tar.gz does not put anything on PATH

The AppImage exception sentence ends "use the deb, rpm or tar.gz if you want
the CLI on your PATH". True of deb and rpm, which install to `/usr/bin`;
false of the tar.gz, whose own README tells the user to do it themselves.
This is a controller-authored sentence and the error is mine. Correct it so
the claim matches each format: name deb and rpm as the ones that put
`muxsmith` on PATH, and the tar.gz as carrying both binaries for the user to
place. Keep it to one sentence; do not grow the paragraph.

## 5. Design bookkeeping: section 4 has diverged from the tree

`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` section
4 is headed "Documentation artifacts (verbatim)" and carries the prose the
implementer transcribed. The owner's rendered-surface pass has since changed
that prose in the tree at seven sites (the verdict enumerates them - use its
list, and verify each by content).

**Ruling, so this is not a choice: record the supersession, do not rewrite
section 4.** Add an amendment-log entry **A3** in the same form as A1 and A2,
stating that the owner's rendered-surface pass of 2026-07-27 supersedes
section 4's verbatim blocks at the enumerated sites, that the tree is
authoritative for that prose, and naming the commits (`e477e37`, `4716b0c`,
and whatever this fix round adds). Section 4's blocks stay byte-unchanged -
they are the wording the transcription was graded against, and this is the
same rule applied at the plan-7.5 close and again to the plan's frozen
fences. Add the pointer at section 4's head as one line so a reader arriving
there is not misled, exactly as the two supersession notes elsewhere do.

Recount the site list yourself; if it is not seven, report the real number.

## Verification you owe

- Each old string measured non-zero before its edit and zero after, each new
  string counted after; wrap-aware substring counting, not line-oriented
  regex, because this prose wraps mid-sentence.
- The tamper control from item 1, explicitly, with its output.
- `git diff --numstat` matching your enumeration.
- Nothing changed outside `docs/INSTALL.md`,
  `.github/release/draft-body.md` and the design document. In particular
  `draft-body.md` lines 2-4 stay untouched: that item is reserved for the
  owner's own inspection of the rendered draft.

## Commits

Two commits on master, staged explicitly, unsigned
(`git -c commit.gpgsign=false`), trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`: one for the
documentation fixes (items 1-4), one for the design amendment (item 5). Do
not push. Report to `.superpowers/sdd/plan-8/wording-fix-round-report.md`.

A caution this session has earned three times, twice from reviewers auditing
themselves: an empty search result is evidence of nothing until you have seen
the same invocation return a hit. The concrete failures were a pattern that
could not match a parenthesized title, and a pathspec list passed unquoted
through a shell that does not word-split, which silently searched one
nonsense path and returned four clean empties.

If a premise here fails against the tree, stop and report it.
