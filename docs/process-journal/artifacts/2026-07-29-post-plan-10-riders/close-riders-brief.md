# Implementer brief - four close riders

**Role:** fresh implementer for four small owner-ruled or review-ruled text
changes, outside any plan. Model tier: mid (dispatch model: Opus 5). Effort:
xhigh. An independent reviewer grades your work.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**. You are the only writer in this tree.
- **Read the files, not a commit hash.**
- zsh: `${PIPESTATUS[0]}` is empty. A bare `cp` is aliased interactive.

## Files (EXHAUSTIVE)

- Modify: `docs/INSTALL.md` (rider 1)
- Modify: `.github/release/draft-body.md` (rider 2)
- Modify: `renovate.jsonc` (rider 3, comment text only - **no key, no value**)
- Modify: `README.md` (rider 4, one paragraph)

## Rider 1 - the glibc floor must reach the deb/rpm reader (MEDIUM finding)

`docs/INSTALL.md` gained a glibc-floor bullet in commit `e260845`. It is the
document's only floor notice, and it sits inside a list whose intro says the
deb/rpm packages declare their dependency and invites those readers to skip -
so the audience that most needs the floor is told not to read it.

**This is measured, not stylistic:** the shipped deb declares
`Depends: libwebkit2gtk-4.1-0, libgtk-3-0` and **no `libc6`**, so `apt install`
succeeds on a system below the floor and the binary then fails at runtime with
nothing having warned the user. The document is the only channel that can warn
them.

**The fix, as the reviewer prescribed it: lift the bullet OUT of the list into a
standalone sentence immediately BEFORE it, without rewording its content.** The
floor applies to every Linux artifact including deb and rpm, and its placement
must say so by position, not only by its own text.

## Rider 2 - the release page asserts a reach the artifacts no longer have (LOW)

`.github/release/draft-body.md`'s artifact table routes the AppImage to "any
Linux distro" and the deb row to "Debian/Ubuntu". The AppImage is built on the
same runner as everything else, so it carries the same glibc floor; "any Linux
distro" is the same claim the tarball README just retracted, and this table is
the first text a downloader reads.

**Controller decision, so you do not weigh it:** the two rows are corrected to
stop asserting a reach the artifacts do not have. Keep the table's register -
short routing phrases, not sentences - and let the existing INSTALL.md link in
line 1 carry the detail. Do not add a column, do not add a row, do not restate
the floor in every row. The `rpm` row's "Fedora & co." and the three non-Linux
rows are untouched.

## Rider 3 - a comment that went stale within the hour

`renovate.jsonc`'s `github-runner` rule carries a comment saying
`release.yml pins ubuntu-22.04 on purpose (D85: the oldest supported base for
the AppImage glibc floor)`. `release.yml` now pins `ubuntu-24.04`. **Only the
comment changes; the rule, its keys and its values stay exactly as they are** -
the reason the rule exists (Renovate cannot know that the release base is chosen
for a glibc floor rather than for freshness) is unaffected and stays stated.

`renovate.jsonc` is a JSONC file validated by the vendor's validator; keep it
valid, keep the comment style identical to its siblings.

## Rider 4 - two growth-prone figures leave the README (owner ruling)

**Owner ruling 2026-07-29, verbatim reasoning: "einfach eine wachstumsfeste
formulierung. unnoetig, das immer zu updaten" - and, on the second figure, "ja,
auch raus damit, das muss nicht in die readme".**

The README's "How this got built" paragraph carries two numbers that the
project's own process keeps moving:

- the count of preserved review verdicts (currently `225 files under docs/ with
  verdict in the name`), which every plan close falsifies by salvaging more
  verdict files;
- the decision-series figure (`103 of them so far, running up to D105`), which
  every new ADR falsifies.

**Both lose the NUMBER and keep the CLAIM.** The paragraph must still say that
every design decision is numbered and recorded with its rationale and rejected
alternatives, and that the review verdicts are preserved in the repo including
the ones that hurt - it just says so without a figure that needs maintaining.
Nothing else in that paragraph changes, and no other paragraph is touched.

Register: the README is written in the owner's sell-tone, a case-scoped
exception recorded on its ROADMAP entry. Match the surrounding voice.

## Standing rules

- **No design latitude.** Each rider's WHAT is fixed above; only the wording is
  yours. A fork found on contact returns as NEEDS_CONTEXT with a decision memo.
- **No edits to any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md` - the controller is the single writer there.
- **No `file:line` citation anywhere in what you write** - the rule was widened
  by owner ruling today to CI and configuration files as well.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.
- A claim about the tree is a COUNT: measure before writing "the only", "every",
  "no X exists".

## Verification bar

1. The full gate as `BUILDING.md` enumerates it, foreground, green. Say what it
   proves here and what it does not: no gate part reads `draft-body.md` or
   `renovate.jsonc`, and no test asserts README prose.
2. `renovate.jsonc` re-validated with the vendor validator, both plain and
   `--strict`, output pasted:
   `npx --yes --package renovate@43.287.0 -- renovate-config-validator renovate.jsonc`
   (if that version no longer resolves, read
   `https://registry.npmjs.org/renovate/latest`, use what it returns, and paste
   both the URL and the version).
3. A grep proving no figure survives in the edited README paragraph, with a
   fired control.
4. `git diff --stat` covering exactly the four files.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...`, deliberately unsigned.
- Stage explicitly by name, **never `git add -A`**.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
- Suggested subject:
  `docs: the glibc floor reaches deb/rpm readers, the release table stops overclaiming, two README figures retire`.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/close-riders-report.md`: per
rider, the before/after text and why; the validator runs; the gate result with
its stated limits; the README grep and its control; `git diff --stat`; numbered
concerns; the commit hash.
