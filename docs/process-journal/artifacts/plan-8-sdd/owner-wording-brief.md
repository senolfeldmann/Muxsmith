# Plan-8 owner rendered-surface pass: eight ruled wording edits

The owner reviewed the plan-8 whole-branch verdict's nine wording items and
ruled eight of them. You apply exactly those eight. Item 7 (whether two
continuation lines in the release-body template render inline) is deliberately
NOT in scope: the owner inspects the rendered draft himself. Do not touch
`.github/release/draft-body.md` lines 2-4.

Repo: `/home/senol/Git/Muxsmith`, work on `master` in the main worktree. No
branch, no worktree, no session-relocation tools. Absolute paths.

Files in scope, complete list: `docs/INSTALL.md`,
`.github/release/draft-body.md` (one line only, see edit 1),
`packaging/linux-tarball-README.txt`. Nothing else.

These are shipped, user-facing documents. Every replacement below is
specified; where a replacement rests on a factual claim, the claim is named
and you verify it before writing. Locate by content - the line numbers are
orientation only.

## Edit 1: checksum verification is not a one-command story across OSes

`docs/INSTALL.md`, the intro paragraph currently reading:

> The sections below show the one-time step per OS. Verify
> downloads against the release's `SHA256SUMS`
> (`sha256sum -c SHA256SUMS` with the files beside it).

Two defects in one sentence. `sha256sum` is a GNU coreutils command:
**Windows has no such command** (the ruled item), and it is **not present on
a stock macOS** either, which ships `shasum` instead. The macOS half is an
extension of the ruled item, flagged to the owner as an assumption he can
reverse; it is included because leaving a verification instruction that fails
on his own primary OS while fixing the Windows half would be the worse
outcome.

**Verify both claims before writing**, and say in your report how: that
`sha256sum` is GNU coreutils and absent from a stock macOS (`shasum -a 256`
is the platform's equivalent), and that PowerShell's `Get-FileHash` is the
Windows equivalent. Authoritative sources, not memory. **If the macOS claim
does not hold, apply only the Windows half and report the refutation.**

Replacement (edit 5 is folded in here - "step" becomes "steps", because
Windows has two and macOS up to three):

> The sections below show the one-time steps per OS. Verify downloads
> against the release's `SHA256SUMS`, with the files beside it:
> `sha256sum -c SHA256SUMS` on Linux, `shasum -a 256 -c SHA256SUMS` on
> macOS, and in PowerShell on Windows
> `Get-FileHash muxsmith-<version>-windows-x64.msi -Algorithm SHA256`
> compared against that file's line in `SHA256SUMS`.

Second site, same defect: `.github/release/draft-body.md`, the line reading
"Verify downloads: put `SHA256SUMS` beside the files and run
`sha256sum -c SHA256SUMS`." Replace the trailing command clause with the same
three-way form, compressed to one sentence appropriate to a release body.
The rest of that file, and in particular its lines 2-4, stays untouched.

## Edit 2: a bullet label that contradicts its own body

`docs/INSTALL.md`, Linux runtime requirements, the bullet labeled
`**GUI only, deb/rpm/tar.gz:**` whose body then explains all four package
formats including the AppImage. Replace the label with `**GUI only:**`. The
body already distinguishes the formats; the format list in the label is what
made it wrong.

## Edit 3 and 8: the macOS symlink step

`docs/INSTALL.md`, macOS CLI section. The fence currently reads:

```sh
sudo ln -s /Applications/Muxsmith.app/Contents/MacOS/muxsmith /usr/local/bin/muxsmith
```

`/usr/local/bin` does not exist on a clean Apple-Silicon macOS, so the
command fails there. Replace the fence body with:

```sh
sudo mkdir -p /usr/local/bin && sudo ln -s /Applications/Muxsmith.app/Contents/MacOS/muxsmith /usr/local/bin/muxsmith
```

Then add, immediately after that fence, the design-sanctioned no-sudo
alternative (D82 records it: "`/usr/local/bin` (or add the `Contents/MacOS`
dir to PATH)"):

> Without `sudo`: add the directory to your PATH instead by appending
> `export PATH="/Applications/Muxsmith.app/Contents/MacOS:$PATH"` to your
> shell profile.

## Edit 4: which PATH the Windows instruction means

`docs/INSTALL.md`, the Windows PATH walk-through. It says "select `Path`"
without naming the pane, while D82 specifies the **user** PATH. In the
walk-through, replace `select \`Path\`` with
`select \`Path\` under **User variables**`. Nothing else in that sentence
changes.

## Edit 6: what an AppImage user actually gets

`docs/INSTALL.md`, the sentence "Every install ships two programs:
**Muxsmith** (the GUI) and **`muxsmith`** (the command-line tool)."

The claim is literally true of every artifact - the rehearsal extracted the
AppImage and found both `usr/bin/muxsmith` and `usr/bin/muxsmith-gui` inside
it - but an AppImage is a single self-contained file, so its CLI is not
reachable as a `muxsmith` command without extracting the image. The sentence
sets an expectation the AppImage cannot meet in normal use. Append to it:

> The AppImage is the practical exception: it carries both binaries inside a
> single self-contained file, so only the GUI is directly runnable - use the
> deb, rpm or tar.gz if you want the CLI on your PATH.

## Edit 9: name the actual glibc floor

`packaging/linux-tarball-README.txt`, the line "- glibc from Ubuntu 22.04
(2022) or newer; any current distribution qualifies." Replace the shorthand
with the version beside it:

> - glibc 2.35 or newer (the version in Ubuntu 22.04, 2022); any current
>   distribution qualifies.

**Verify 2.35 before writing it**: the floor is the glibc of the runner image
the Linux leg builds on, which `.github/workflows/release.yml` names. Read
the runner label there and confirm the mapping from an authoritative source.
If it is not 2.35, use the correct number and report the correction.

## Verification you owe

- For each edit: the old string counted its expected non-zero number before
  the edit and zero after; the new string counts what it should. Run each
  pattern BEFORE editing and watch it return that non-zero count, so the
  later zero is a real absence rather than a malformed pattern.
- `git diff --numstat` matches your enumeration of changed lines per file.
- `pnpm lint` is not owed (no code changes), but run
  `python3 scripts/ledger-lint.py` once (expect exit 0) as a cheap tree-sanity
  check, since another implementer has been committing to this tree in
  parallel.
- Confirm no file outside the three named files changed.

## Commits

One commit on master, staged explicitly (never `git add -A`), unsigned
(`git -c commit.gpgsign=false`), trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`. Do not push.

Do not touch `docs/ROADMAP.md`, `docs/process-journal.md`, any
`docs/*.yaml`, `BUILDING.md`, `scripts/`, the design or plan documents, or
`.superpowers/sdd/plan-8/progress.md` - those carry other agents' and the
controller's uncommitted or just-committed work. Your only `.superpowers`
write is your report at
`.superpowers/sdd/plan-8/owner-wording-report.md`.

Do not create, edit, publish or delete any GitHub release; the draft
`rehearsal-30273529210` is the owner's pending inspection input.

If any premise here fails against the tree or against the sources you check,
stop and report it. Refuting a premise with evidence is a valid completion.
