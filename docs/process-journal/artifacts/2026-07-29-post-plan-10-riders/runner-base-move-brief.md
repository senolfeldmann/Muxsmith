# Implementer brief - move the Linux release base to ubuntu-24.04

**Role:** fresh implementer for a single owner-ruled change outside any plan.
Model tier: mid (dispatch model: Opus 5). Effort: xhigh. An independent reviewer
grades your work; the controller re-runs your claims.

## Preamble (binding)

- Never call session-relocation tools. Work on `master` in the main worktree,
  `/home/senol/Git/Muxsmith`. No branch, no worktree.
- Absolute paths, **foreground runs only**. You are the only writer in this tree.
- **Read the files, not a commit hash.**
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty (bash-only). A bare `cp` is
  aliased interactive and hangs on overwrite.

## Why this change exists

The release workflow builds the Linux artifacts on `ubuntu-22.04` while the test
matrix runs `ubuntu-26.04`. That split is deliberate and is the vendor's own
instruction - Tauri's AppImage guide says to build "using the oldest base system
you intend to support", because glibc is backward but not forward compatible, so
a binary built on a newer base will not start on an older system.

**GitHub has now announced the deprecation of the Ubuntu-22 runner images:
deprecation begins 2026-09-17 and they are fully unsupported 2027-04-17, with
brownout periods failing jobs in between** (actions/runner-images issue 14254).
That fired a registered trigger in `docs/ROADMAP.md`, whose prescription is to
move the Linux release leg to `ubuntu-24.04` and record the raised floor.

**The owner ruled it 2026-07-29: release on 24.04, tests stay on 26.04.** The
consequence he accepted: the minimum glibc rises from 2.35 to 2.39, which drops
Ubuntu 22.04 LTS (2.35) and Debian 12 (2.36); Debian 13 carries 2.41 and is
unaffected. Building on 26.04 was rejected because its 2.43 floor would drop
Debian 13 too.

## Files (EXHAUSTIVE)

- Modify: `.github/workflows/release.yml` - the three `ubuntu-22.04` pins
  (the `guard` job's `runs-on`, the `bundle` matrix's `os:` entry for the
  `linux-x86_64` leg, and the `assemble` job's `runs-on`) **and** the file-top
  pinning-policy comment block, which names `ubuntu-22.04` and states the
  divergence rationale that this change updates. Nothing else - no action SHA,
  no other job, no step logic.
- Modify: `packaging/linux-tarball-README.txt` - its glibc requirement line,
  which today reads "glibc 2.35 or newer (the version in Ubuntu 22.04, 2022)".
- Modify: `docs/INSTALL.md` - the Linux runtime-requirements list, which today
  states no glibc floor at all. Add one, in the register of the surrounding
  bullets.

## What to get right

1. **Every pin moves, not only the artifact-building one.** All three jobs sit
   on the deprecated image; a brownout fails whichever runs during it. Verify by
   grep that no `22.04` remains in the file, and fire that grep against a
   known-present string so an empty result means something.
2. **The policy comment must stay TRUE after the edit.** It currently records
   the 22.04-versus-26.04 divergence as a deliberate D85 deviation. The
   divergence still exists (24.04 versus 26.04) and the reason is unchanged - it
   is the base-oldest-supported rule, not a preference - so the comment is
   updated, not deleted. Its neighbouring statement about `windows-11-arm`
   having no dated label is untouched.
3. **The two requirement texts must agree with each other and with the new
   base.** The tar.gz README states the floor as a version plus the distribution
   it came from; `docs/INSTALL.md` currently states none. Both end up naming the
   same floor. **Ubuntu 24.04 carries glibc 2.39** - that figure is the owner's
   decision basis and is not yours to re-derive from a package list, but if you
   find a source that contradicts it, report it rather than writing around it.
4. **Do not invent a webkit version claim.** The runtime requirement is the
   package (`webkitgtk 4.1` / `libwebkit2gtk-4.1-0` / `webkit2gtk4.1`) and that
   package name does not change with the build base. If you cannot establish a
   precise minimum webkit VERSION from the tree, say nothing about one - an
   unmeasured version floor is worse than no sentence.
5. **The test matrix in `.github/workflows/ci.yml` does NOT change.** It stays
   on `ubuntu-26.04`. Touching it is out of scope.

## Standing rules

- **No design latitude**, in either form. A fork found on contact returns as
  **NEEDS_CONTEXT with a decision memo**, not resolved at the keyboard.
- **No edits to any house-knowledge YAML**, `docs/ROADMAP.md` or
  `docs/process-journal.md` - the controller is the single writer there and will
  consume the trigger and record the ruling.
- **A comment or doc line never locates code by a line number** (owner ruling,
  widened at this session's close to CI and configuration files too).
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.
- Every observed value in your report is pasted from the run that produced it.

## Verification bar, stated honestly because it has a real limit

1. The full gate as `BUILDING.md` enumerates it, foreground, green. **It does
   not exercise the release workflow at all** - no gate part reads
   `release.yml` - so say so rather than letting a green gate imply more than it
   proves.
2. YAML validity of the edited workflow, checked with a parser rather than by
   eye (`python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/release.yml'))"`),
   and a diff review that no key moved.
3. The grep proving no `22.04` remains, with its fired control.
4. `git diff --stat` covering exactly the three files.
5. **State the open verification explicitly in your report:** the only real
   proof that the 24.04 base builds is a draft release run, which needs a
   `workflow_dispatch` the owner triggers. Do not claim the change is verified
   beyond what you ran.

## Commit (SI-4, restated because you cannot see the grant)

Commits on this repository are **standing-authorized by the owner**; your global
never-commit default does not apply. You commit; you do NOT push.

- `git -c commit.gpgsign=false commit ...`, agent commits deliberately unsigned.
- Stage explicitly by name, **never `git add -A`**.
- Exactly one trailer: `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`.
- Suggested subject: `release: build the Linux artifacts on ubuntu-24.04, raising the glibc floor to 2.39`.

## Report

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/runner-base-move-report.md`:
per file, what changed and why; the grep and its control; the YAML parse; the
gate result with its stated limit; the open verification; numbered concerns; the
commit hash and `git show --stat`.
