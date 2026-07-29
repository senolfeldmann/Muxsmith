# Implementer report - move the Linux release base to ubuntu-24.04

**Status:** DONE. Three files changed, all eleven gate parts green, committed
`e260845` on `master` in `/home/senol/Git/Muxsmith`, not pushed.

## Per file: what changed and why

### 1. `.github/workflows/release.yml`

**Three runner pins**, all `ubuntu-22.04` -> `ubuntu-24.04`:

- the `guard` job's `runs-on`
- the `bundle` matrix's `os:` for the `linux-x86_64` leg (which
  `runs-on: ${{ matrix.os }}` consumes)
- the `assemble` job's `runs-on`

All three moved, not only the artifact-building one: a brownout fails whichever
job happens to run during it, and `guard` and `assemble` sit on the same image.

**The file-top pinning-policy comment block.** Kept as a D85 deviation record
rather than deleted - the divergence still exists (24.04 versus 26.04) and its
reason is unchanged. Before:

```
# Two recorded deviations (D85): windows-11-arm has no dated label
# (the only GA windows-arm64 image); ubuntu-22.04 here deliberately
# diverges from the test matrix's ubuntu-26.04 - release artifacts are
# built on the oldest supported base (Tauri AppImage guidance) while
# tests run on the newest.
```

After:

```
# Two recorded deviations (D85): windows-11-arm has no dated label
# (the only GA windows-arm64 image); ubuntu-24.04 here deliberately
# diverges from the test matrix's ubuntu-26.04 - release artifacts are
# built on the oldest base we intend to support (Tauri AppImage
# guidance, glibc being backward but not forward compatible) while
# tests run on the newest. That base sets the shipped glibc floor at
# 2.39; docs/INSTALL.md and the tar.gz README state it, so moving this
# pin moves the product's reach and those two texts with it.
```

Three deliberate properties of that wording:

- "oldest **supported** base" became "oldest base **we intend to support**",
  which is Tauri's own formulation and stays true independently of which images
  GitHub currently offers.
- The last sentence binds the pin to the two user-facing texts, so the next
  person bumping the pin finds the reach consequence in the file that causes it.
- It contains **no** literal `22.04`. My first draft named the deprecation as
  the reason 24.04 is now that base; that left one `22.04` occurrence in the
  file and would have failed the brief's own grep bar, so the clause came out.
  The deprecation history belongs to `docs/ROADMAP.md` and the journal, which
  are the controller's to write.

The neighbouring `windows-11-arm` statement is untouched, "Two recorded
deviations" still counts two, and no action SHA, job, step or key moved.

### 2. `packaging/linux-tarball-README.txt`

Before:

```
- glibc 2.35 or newer (the version in Ubuntu 22.04, 2022); any current
  distribution qualifies. If this archive does not run on your system,
  the AppImage from the same release bundles its dependencies.
```

After:

```
- glibc 2.39 or newer (the version in Ubuntu 24.04, 2024): these
  binaries are built on that base, so Ubuntu 22.04 LTS (2.35) and
  Debian 12 (2.36) are below the floor. The AppImage from the same
  release is built on the same base and does not lift it; what it
  does bundle is webkitgtk and gtk3, so use it when only those are
  missing.
```

The bullet changed beyond the bare version swap because both of its remaining
statements stopped being true at the higher floor, and the brief requires the
text to agree with the new base:

- "any current distribution qualifies" is false at 2.39. Ubuntu 22.04 LTS is
  current (supported into 2027) and Debian 12 is oldstable; both are now below
  the floor. They are named explicitly, which is also the consequence the owner
  accepted.
- "If this archive does not run on your system, the AppImage ... bundles its
  dependencies" read, after a glibc sentence, as *the AppImage rescues an
  old-glibc system*. It does not: the AppImage is built on the same runner and
  AppImages bundle libraries but not glibc - which is precisely why the
  build-on-the-oldest-base rule exists at all. The sentence is kept but scoped
  to what the AppImage genuinely bundles.

Named libraries stayed the package names already in the file (`webkitgtk`,
`gtk3`); no webkit version was invented.

### 3. `docs/INSTALL.md`

One bullet added at the end of the Linux runtime-requirements list, in the
register of the two above it (bold lead, hyphen, explanation):

```
- **glibc 2.39 or newer** - not a package you install but what your
  distribution ships: every Linux artifact here, the AppImage
  included, is built on Ubuntu 24.04 (glibc 2.39), so systems below
  that floor - Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot
  run them.
```

The list's intro sentence is scoped to the AppImage and tar.gz ("install the
runtime requirements yourself"), but the glibc floor binds deb/rpm too, so the
bullet says "every Linux artifact here" and states outright that it is not an
installable package. Same floor, same two dropped distributions, same
parenthetical figures as the tar.gz README, so the two texts agree with each
other and with the new base.

**The glibc figure.** 2.39 for Ubuntu 24.04 is the owner's decision basis and I
did not re-derive it. **No contradicting source was encountered**; the ROADMAP
trigger entry's own measured line agrees ("Ubuntu 22.04 carries glibc 2.35,
24.04 carries 2.39, 26.04 carries 2.43; Debian 12 carries 2.36, Debian 13
carries 2.41").

### Not changed

`.github/workflows/ci.yml` - the test matrix stays on `ubuntu-26.04`, confirmed
by reading it after the edits:

```
19:# ubuntu-26.04 is a preview image (Şenol's call, newest line).
29:        os: ["ubuntu-26.04", "windows-2025", "macos-15"]
164:    runs-on: ubuntu-26.04
179:    runs-on: ubuntu-26.04
```

No house-knowledge YAML, no `docs/ROADMAP.md`, no `docs/process-journal.md`.

## The grep, with its fired control

Run after the edits, from the repo root:

```
=== A: grep 22.04 in release.yml (expect: no hits, exit 1) ===
grep exit: 1

=== B: CONTROL, same invocation, pattern known present (24.04) ===
22:# (the only GA windows-arm64 image); ubuntu-24.04 here deliberately
32:    runs-on: ubuntu-24.04
84:            os: ubuntu-24.04
189:    runs-on: ubuntu-24.04
grep exit: 0

=== C: CONTROL, same 22.04 pattern, file known to contain it ===
51:      // release.yml pins ubuntu-22.04 on purpose (D85: the oldest supported
grep exit: 0
```

Control B fires the same `grep -n` against `release.yml` with a pattern that is
present, so the empty result in A is not an unreadable path or a malformed
invocation. Control C fires the **exact A pattern** (`22\.04`) against a file
that does contain it (`renovate.jsonc`), so the empty result is not a broken
pattern either. B also enumerates all four surviving `24.04` sites: the comment
plus the three pins.

## The YAML parse

```
$ python3 -c "import yaml; d=yaml.safe_load(open('.github/workflows/release.yml')); ..."
YAML OK
guard runs-on: ubuntu-24.04
assemble runs-on: ubuntu-24.04
bundle runs-on: ${{ matrix.os }}
matrix: [('windows-x86_64', 'windows-2025', 'msi'), ('windows-arm64', 'windows-11-arm', 'msi'), ('macos-arm64', 'macos-15', 'dmg'), ('linux-x86_64', 'ubuntu-24.04', 'deb,rpm,appimage')]
guard steps: 3 bundle steps: 12 assemble steps: 4
```

The parse resolves all three pins to `ubuntu-24.04` from the parsed tree rather
than from the text, and the non-Linux matrix legs, the bundle sets and the step
counts are unchanged. `git diff` (below, and reviewed line by line) shows only
the comment block and the three pin lines; no key moved, no indentation shifted.

One pre-existing artifact, stated so it is not read as a finding: PyYAML reports
the top-level keys as `['name', True, 'permissions', 'jobs']` - `on:` is a YAML
1.1 boolean under PyYAML. That is the parser's quirk, present before this change
and irrelevant to GitHub's own parser.

## `git diff --stat` - exactly three files

```
 .github/workflows/release.yml      | 15 +++++++++------
 docs/INSTALL.md                    |  5 +++++
 packaging/linux-tarball-README.txt |  9 ++++++---
 3 files changed, 20 insertions(+), 9 deletions(-)
```

## The gate, and what it does not prove

All eleven parts as `BUILDING.md` enumerates them, foreground, from the repo
root, each green:

| # | Part | Result |
|---|------|--------|
| 1 | `cargo fmt --all --check` | exit 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| 3 | `cargo test --workspace` | exit 0; 39 suites, 505 passed, 0 failed |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | exit 0 |
| 5 | `cargo deny check` | exit 0; `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy ... --target x86_64-pc-windows-msvc -- -D warnings` | exit 0 |
| 7 | `pnpm lint` | exit 0 |
| 8 | `pnpm build` | exit 0; `built in 156ms` |
| 9 | `pnpm check:i18n` | exit 0; `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| 10 | `pnpm test:e2e` | exit 0; `68 passed (2.9s)` |
| 11 | `python3 scripts/ledger-lint.py` | exit 0; `ledger-lint: 546 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

Part 6 emits two pre-existing informational lines, `warning: muxsmith-gui@0.1.0:
GNU compiler is not supported for this target`, and still exits 0.

**The limit, stated plainly: a green gate says nothing about this change.** No
gate part reads `release.yml`, `docs/INSTALL.md` or
`packaging/linux-tarball-README.txt`. Verified rather than assumed, with fired
controls:

```
=== A: do gate-relevant sources reference the edited files? (expect no hits) ===
exit: 1
=== B: CONTROL, same command, patterns known present in those trees ===
scripts/ledger-lint.py
exit: 0
=== C: CONTROL, the same 'release.yml' pattern where it IS present ===
BUILDING.md
docs/ROADMAP.md
exit: 0
```

(A: `grep -rn "release\.yml|INSTALL\.md|linux-tarball"` over `scripts/`,
`crates/`, `src-tauri/src/`, `src/`, `e2e/`. B: the same recursive grep over the
same trees for a pattern that is present there. C: A's own `release\.yml`
pattern against files that do contain it.) The gate therefore proves only that
the change broke nothing else in the workspace.

## Open verification (not closed by anything I ran)

**The only real proof that a 24.04 base builds Muxsmith's Linux artifacts is a
draft release run**, which needs a `workflow_dispatch` on `release.yml` with
`rehearse-draft-release: true` - an owner action. Until then, unverified:

- that the apt package set in the "Install Tauri Linux build dependencies" step
  (`libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev
  libayatana-appindicator3-dev librsvg2-dev`) resolves on the 24.04 image. This
  is the most likely breakage point of a base move, and nothing in the repo
  measures it. My prior is that it resolves - 24.04 is the release that dropped
  webkit2gtk **4.0** while keeping 4.1, which is the package this step wants -
  but that is recall, not a measurement, and the rehearsal is what settles it.
- that the produced binaries actually carry a 2.39 (not higher) symbol floor.
  The published floor is derived from the build base, as before; nobody has run
  an `objdump`-style check on the artifacts, and this change does not add one.
- that `gh`, `jq`, `sha256sum` and `tar` behave identically on the 24.04 image
  for the guard and assemble jobs (ROADMAP trigger 9 exists for exactly this
  class: "runner-image gh breaks a release-ops invocation").

## Concerns

1. **`renovate.jsonc` now carries a stale comment, and it is outside my
   exhaustive file list.** Lines 51-54 read: `// release.yml pins ubuntu-22.04
   on purpose (D85: the oldest supported // base for the AppImage glibc floor)
   while the test matrix runs // ubuntu-26.04. Renovate cannot know that
   distinction, so runner // images are not managed at all.` The *rule* it
   justifies (`github-runner` updates disabled) is unaffected and still right;
   only the version named is now wrong. This is a live configuration file, not a
   historical artifact, so I flag it rather than fix it: the brief declares its
   file list exhaustive and forbids design latitude. Recommend a one-word follow
   -up edit (`ubuntu-22.04` -> `ubuntu-24.04`) by whoever owns the next change
   to that file. **Second copy of the same sentence:** `docs/process-conventions
   .yaml` (the 2026-07-29 plan-10 entry) states "release.yml pins ubuntu-22.04
   on purpose for the AppImage glibc floor (D85)"; that is house-knowledge YAML
   and thus the controller's alone.
2. **I rewrote more of the tar.gz README bullet than a version swap.** "any
   current distribution qualifies" and the AppImage fallback both became false
   at the higher floor (reasoning in section 2 above). I judged that inside the
   brief's "must agree with the new base" mandate rather than a fork worth
   returning as NEEDS_CONTEXT, but it is the one place where a reviewer may
   reasonably want different wording. The floor figure itself is not at issue.
3. **Placement judgment in `docs/INSTALL.md`.** The new bullet sits under an
   intro sentence scoped to the AppImage and tar.gz, while the glibc floor binds
   deb/rpm as well. I restored the scope inside the bullet's own text ("every
   Linux artifact here, the AppImage included") rather than restructure the
   section, which would have exceeded the brief. A reviewer preferring the floor
   as a standalone sentence above the list would be making a defensible call.
4. **The two dropped distributions are now named in shipped user-facing text.**
   That is the owner's accepted consequence, but it is the first time Ubuntu
   22.04 LTS and Debian 12 appear by name as unsupported in artifacts users
   read. Worth an owner glance for tone before 1.0.
5. **No `docs/ROADMAP.md` trigger consumption, by instruction.** The trigger
   entry still reads "NOT YET CONSUMED ... Decision pending." That is the
   controller's write, but it means the tree currently carries an implemented
   ruling whose ROADMAP entry still says pending.

## Commit

`e260845` (unsigned, `git -c commit.gpgsign=false`, three explicit pathspecs,
one trailer). Not pushed.

```
commit e26084557f9351fe9e33bfe37b1092ebbabbe1b9
Author: Şenol Feldmann <senol.feldmann@gmail.com>
Date:   Wed Jul 29 19:05:16 2026 +0200

    release: build the Linux artifacts on ubuntu-24.04, raising the glibc floor to 2.39

    GitHub's runner-image deprecation (actions/runner-images 14254: deprecation
    from 2026-09-17, unsupported 2027-04-17, brownouts in between) fired the
    registered ROADMAP trigger for the ubuntu-22.04 release base. All three
    release jobs move to ubuntu-24.04 - guard, the linux-x86_64 bundle leg and
    assemble - since a brownout fails whichever job runs during it, not only the
    artifact-building one. The test matrix in ci.yml stays on ubuntu-26.04: the
    D85 divergence is unchanged in kind, release artifacts are built on the
    oldest base we intend to support while tests run on the newest.

    The consequence is product reach, not a version bump: the shipped glibc
    floor rises from 2.35 to 2.39, dropping Ubuntu 22.04 LTS (2.35) and Debian
    12 (2.36); Debian 13 (2.41) is unaffected. Both user-facing requirement
    texts now name that floor - the tar.gz README raises its line, docs/INSTALL.md
    gains one where it had none - and the pinning-policy comment records that
    moving this pin moves those texts with it.

    Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

 .github/workflows/release.yml      | 15 +++++++++------
 docs/INSTALL.md                    |  5 +++++
 packaging/linux-tarball-README.txt |  9 ++++++---
 3 files changed, 20 insertions(+), 9 deletions(-)
```

`.superpowers/` is gitignored (`git check-ignore -v` -> `.gitignore:2`), so this
report is not part of the commit.
