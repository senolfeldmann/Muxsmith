# Plan-8 owner rendered-surface pass: implementer report

Brief: `.superpowers/sdd/plan-8/owner-wording-brief.md`.
Branch: `master`, main worktree, no branch/worktree/relocation.
Commit: `e477e371a90321d979f21a8fe7ef3e0829d864b1` (`e477e37`) - one commit,
unsigned (`git show --format=%G?` -> `N`), three files staged explicitly,
not pushed (`git rev-list --count @{u}..HEAD` -> 1).

## Headline

> **Superseded in part.** The owner has since ruled that the macOS clause be
> added in the `shasum -a 256` form. It was added in a second commit; see
> "Follow-up" at the end. The withholding described below was the correct
> state at the time of commit `e477e37`.

Seven of the eight ruled items applied in full. The eighth (edit 1) applied
**without its macOS clause**, because the brief's macOS premise is refuted
against Apple's own shipped documentation and the brief closes that fork
explicitly: *"If the macOS claim does not hold, apply only the Windows half
and report the refutation."* Details in "Refuted premise" below.

One further correction against the tree: the brief's replacement text names
the Windows artifact `muxsmith-<version>-windows-x64.msi`. No such artifact
exists. Corrected to `muxsmith-<version>-windows-x86_64.msi` (see
"Corrected value" below).

## Edits applied

| # | File | What changed |
|---|---|---|
| 1a + 5 | `docs/INSTALL.md` | Intro checksum sentence: "one-time **step**" -> "one-time **steps**"; the single `sha256sum -c SHA256SUMS` parenthetical replaced by a per-OS form naming Linux (`sha256sum -c SHA256SUMS`) and Windows (`Get-FileHash muxsmith-<version>-windows-x86_64.msi -Algorithm SHA256`, compared against that file's line in `SHA256SUMS`). **macOS clause withheld** - see refutation. |
| 1b | `.github/release/draft-body.md` | Trailing command clause of the "Verify downloads" sentence extended to the same form, compressed to one sentence: `sha256sum -c SHA256SUMS` on Linux, or `Get-FileHash <file> -Algorithm SHA256` in PowerShell on Windows. Lines 2-4 untouched. macOS clause withheld, same reason. |
| 2 | `docs/INSTALL.md` | Linux runtime-requirements bullet label `**GUI only, deb/rpm/tar.gz:**` -> `**GUI only:**`. |
| 3 | `docs/INSTALL.md` | macOS CLI fence body -> `sudo mkdir -p /usr/local/bin && sudo ln -s /Applications/Muxsmith.app/Contents/MacOS/muxsmith /usr/local/bin/muxsmith`. |
| 8 | `docs/INSTALL.md` | New paragraph immediately after that fence: the no-sudo alternative (`export PATH="/Applications/Muxsmith.app/Contents/MacOS:$PATH"` appended to the shell profile), verbatim as specified. |
| 4 | `docs/INSTALL.md` | Windows PATH walk-through: `select \`Path\`` -> `select \`Path\` under **User variables**`. Rest of the sentence unchanged. |
| 6 | `docs/INSTALL.md` | "two programs" sentence extended with the AppImage exception, verbatim as specified (rewrapped to the file's ~70-column prose wrap). |
| 9 | `packaging/linux-tarball-README.txt` | `- glibc from Ubuntu 22.04 (2022) or newer; ...` -> `- glibc 2.35 or newer (the version in Ubuntu 22.04, 2022); any current distribution qualifies.` |

Item 7 (release-body continuation lines) not touched, as instructed.
`.github/release/draft-body.md` lines 2-4 verified unchanged in the diff.

## Factual claims: how they were verified

### `sha256sum` is GNU coreutils

GNU coreutils manual, "sha2 utilities" node
(<https://www.gnu.org/software/coreutils/manual/html_node/sha2-utilities.html>):
"The commands `sha224sum`, `sha256sum`, `sha384sum` and `sha512sum` compute
checksums of various lengths (respectively 224, 256, 384 and 512 bits),
collectively known as the SHA-2 hashes." Holds.

### PowerShell `Get-FileHash` is the Windows equivalent

Microsoft Learn, `Get-FileHash` (Microsoft.PowerShell.Utility)
(<https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.utility/get-filehash>).
Syntax `Get-FileHash [-Path] <String[]> [[-Algorithm] <String>]`;
`-Algorithm` accepted values "SHA1, SHA256, SHA384, SHA512, MD5", default
SHA256. The page carries monikers for powershell-5.1 (the in-box Windows
PowerShell on Windows 10/11) through 7.7, so the cmdlet is available without
installing anything. Holds.

Note the doc text only asserts what `Get-FileHash` does; it does not assert
that Windows lacks `sha256sum` (a Windows box with Git-for-Windows, MSYS or
WSL does have it). The ruled defect - that the old text handed every reader a
GNU-only command - is fixed without making an unverifiable negative claim.

### glibc floor = 2.35

Two steps, both from source:

1. `.github/workflows/release.yml`, `bundle` job matrix: the Linux leg is
   `- leg: linux-x86_64` / `os: ubuntu-22.04`. The pinning-policy comment
   above the `jobs:` block records this deliberately ("`ubuntu-22.04` here
   deliberately diverges from the test matrix's `ubuntu-26.04` - release
   artifacts are built on the oldest supported base"). So the floor is
   Ubuntu 22.04's glibc.
2. Ubuntu package index for jammy, `libc6`
   (<https://packages.ubuntu.com/jammy/libc6>): upstream version **2.35**,
   full package version `2.35-0ubuntu3.14` on amd64.

2.35 confirmed; written as specified, no correction needed.

## Refuted premise: `sha256sum` on macOS

The brief states `sha256sum` "is **not** present on a stock macOS either,
which ships `shasum` instead", and asks for verification before writing. It
does not hold as stated.

**Current macOS ships `sha256sum` in the base system.** Apple's own
`md5(1)` man page - the one installed on macOS, mirrored at
<https://keith.github.io/xcode-man-pages/md5.1.html>, page footer
"February 13, 2024 | Mac OS X 14" - carries in its NAME line:

> md5, sha1, sha224, sha256, sha384, sha512, md5sum, sha1sum, sha224sum,
> sha256sum, sha384sum, sha512sum - calculate a message-digest fingerprint
> (checksum) for a file

and in DESCRIPTION:

> The md5sum, sha1sum, sha224sum, sha256sum, sha384sum, and sha512sum
> utilities do the same, but with command-line options and an output format
> that match those of their similarly named GNU utilities.

Its "GNU OPTIONS" section documents `-c, --check` as reading a digest file
"in either classical BSD format or in GNU coreutils format" and printing
`name: OK`/`FAILED` per line. That is exactly `sha256sum -c SHA256SUMS`.
The man-page index of the same mirror carries a dedicated `sha256sum.1`
entry (alongside `sha1sum.1`, `sha224sum.1`, `sha384sum.1`, `sha512sum.1`),
i.e. macOS installs man links for these names.

**It is a recent addition, though.** The Wayback capture of the same page
from 2020-10-28 (macOS 11 Big Sur era,
<https://web.archive.org/web/20201028110942/https://keith.github.io/xcode-man-pages/md5.1.html>)
has NAME "md5 - calculate a message-digest fingerprint (checksum) for a
file", SYNOPSIS `md5 [-pqrtx] [-s string] [file ...]`, and the string
"sha256sum" does not occur anywhere on it. So the GNU-mode aliases (an
Apple pull of the FreeBSD `md5` rewrite - the ACKNOWLEDGMENTS credit
"Compatibility with GNU coreutils was added by Warner Losh ... and much
expanded by Dag-Erling Smørgrav") landed somewhere after macOS 11 and no
later than macOS 14.

I did not pin the exact introducing version. I could not locate the `md5`
sources in `apple-oss-distributions` (`file_cmds`, `adv_cmds`, `system_cmds`
and `shell_cmds` all checked at their current trees; none contains an `md5`
directory), and the two dated man-page snapshots above are the tightest
bracket I have. Stating a precise version would be invented precision.

**Consequence for the doc.** `docs/INSTALL.md` declares macOS 11+ support,
so the picture is:

| macOS | `sha256sum` | `shasum -a 256 -c` |
|---|---|---|
| 11, 12 | absent (per the 2020 man page) | present |
| 13/14+ | present, GNU-compatible `-c` | present |

The brief's *replacement text* for the macOS clause
(`shasum -a 256 -c SHA256SUMS`) is in fact correct across the whole
supported range - `shasum(1)` on macOS documents `-a` accepting
"1 (default), 224, 256, 384, 512, 512224, 512256" and `-c` as "read SHA sums
from the FILEs and check them"
(<https://keith.github.io/xcode-man-pages/shasum.1.html>). But the *premise*
that motivated it is wrong for a current macOS, which is precisely the
condition the brief made decisive, and the ruling was "apply only the
Windows half and report the refutation". So the macOS clause is withheld in
both files, and the question goes back to the owner:

- **Keep the withholding** - the intro names Linux and Windows only, macOS
  readers are unserved.
- **Add `shasum -a 256 -c SHA256SUMS` for macOS** (the brief's text) -
  correct on macOS 11 through current, slightly redundant on 13+.
- **Fold macOS into the Linux clause** (`sha256sum -c SHA256SUMS` on Linux
  and macOS 13+) - shortest, but silently wrong for macOS 11-12, which the
  doc still claims to support.

My recommendation is the second: `shasum` is the one command correct on
every macOS the doc supports, and the doc's job is a working instruction,
not a minimal one. Trade-off: it tells a macOS 15 reader to use a Perl
wrapper when the coreutils-compatible command is right there.

## Corrected value: the Windows artifact name

The brief's replacement text reads
`Get-FileHash muxsmith-<version>-windows-x64.msi -Algorithm SHA256`.
`windows-x64` occurs nowhere in `docs/`, `.github/`, `packaging/` or
`scripts/` (swept; zero hits, with `windows-x86_64` as the positive control
returning hits in the same sweep). The repo's two Windows artifacts are
`muxsmith-<version>-windows-x86_64.msi` (Intel/AMD) and
`muxsmith-<version>-windows-arm64.msi` - named that way in
`docs/INSTALL.md` line 19, in the `.github/release/draft-body.md` artifact
table, and by the `leg: windows-x86_64` matrix entry in `release.yml`.

Writing `windows-x64.msi` into a shipped install document would name a file
that no release contains - a new instance of the defect class this pass
exists to remove. I applied the same policy the brief itself sets for a
wrong factual value ("If it is not 2.35, use the correct number and report
the correction") and wrote `muxsmith-<version>-windows-x86_64.msi`. Flagged
here rather than resolved silently; revert to whatever the owner prefers if
this reads as overreach.

In `.github/release/draft-body.md` the question does not arise: the artifact
table sits three lines above the sentence and lists both Windows msi names,
so the compressed release-body form uses `<file>`.

## Verification

### Pattern measurement (before and after)

Every old/new string was counted with a whole-file exact-substring count in
Python (`scratchpad/check.py`), not a line-oriented regex, so no line
boundary or metacharacter can silently exclude a match. **Every OLD pattern
was run before the edit and observed returning a non-zero count**, so the
post-edit zero is a real absence rather than a malformed pattern.

| Pattern | before | after |
|---|---|---|
| `show the one-time step per OS` | 1 | 0 |
| ``(`sha256sum -c SHA256SUMS` with the files beside it).`` | 1 | 0 |
| `show the one-time steps per OS` | 0 | 1 |
| `Get-FileHash muxsmith-<version>-windows-x86_64.msi -Algorithm SHA256` | 0 | 1 |
| draft-body old two-line "Verify downloads ... `sha256sum -c SHA256SUMS`." | 1 | 0 |
| ``\`Get-FileHash <file> -Algorithm SHA256\``` | 0 | 1 |
| `- **GUI only, deb/rpm/tar.gz:** webkitgtk 4.1 and gtk3` | 1 | 0 |
| `- **GUI only:** webkitgtk 4.1 and gtk3` | 0 | 1 |
| `\nsudo ln -s /Applications/...:/usr/local/bin/muxsmith\n` (bare fence line) | 1 | 0 |
| `sudo mkdir -p /usr/local/bin && sudo ln -s ...` | 0 | 1 |
| ``Without `sudo`: add the directory to your PATH instead`` | 0 | 1 |
| ``Variables > select `Path` > Edit > New >`` | 1 | 0 |
| ``Variables > select `Path` under **User variables** > Edit > New >`` | 0 | 1 |
| ``**`muxsmith`** (the command-line tool).\n`` (sentence ends the line) | 1 | 0 |
| edit-6 appended sentence (4-line wrap-aware pattern) | 0 | 1 |
| `- glibc from Ubuntu 22.04 (2022) or newer; ...` | 1 | 0 |
| `- glibc 2.35 or newer (the version in Ubuntu 22.04, 2022); ...` | 0 | 1 |

One pattern needed correcting mid-run and the correction is recorded here
rather than hidden: the first edit-6 NEW probe was the single-line string
`The AppImage is the practical exception`, which returned 0 after the edit
because the prose wraps between "practical" and "exception". Re-measured
with the wrap-aware four-line string: count 1. The blind spot was in the
probe, not the edit.

Withheld/consistency sweep over the concatenated text of all three files
after the edits: `shasum` 0 (macOS clause confirmed withheld),
`windows-x64` 0, `windows-x86_64` 3, `one-time step per OS` 0,
`GUI only, deb/rpm/tar.gz` 0, `glibc from Ubuntu` 0.

### Diff accounting

`git diff --numstat` for the three files:

```
3	1	.github/release/draft-body.md
16	7	docs/INSTALL.md
3	3	packaging/linux-tarball-README.txt
```

Matches the per-edit enumeration exactly:

- `docs/INSTALL.md` -7/+16 = edit 1a (-3/+5) + edit 6 (-1/+4, first line of
  the sentence is unchanged) + edit 4 (-1/+1) + edit 3&8 (-1/+5) + edit 2
  (-1/+1).
- `.github/release/draft-body.md` -1/+3 (one line becomes three).
- `packaging/linux-tarball-README.txt` -3/+3 (rewrap of one bullet).

### Tree sanity and scope

- `python3 scripts/ledger-lint.py` -> `ledger-lint: 473 entries across 4
  files, all invariants hold`, exit 0.
- Line widths: no new prose line exceeds the files' existing ~70-column
  wrap. The only new long line is the macOS fence command (117 chars), a
  shell command that cannot wrap; it replaces an existing 89-char one.
- Nothing outside the three files was staged. The working tree also carries
  the parallel implementer's and the controller's changes
  (`docs/decision-ledger.yaml`, `docs/process-journal.md`,
  `docs/process-journal/artifacts/plan-7.5-sdd/...` plus untracked files
  there); all left unstaged and untouched. `git diff --cached --numstat`
  after staging lists exactly the three paths.
- No GitHub release created, edited, published or deleted; no `gh` release
  command run at all. Draft `rehearsal-30273529210` untouched.
- Not pushed.

---

# Follow-up: the macOS clause, added on the owner's ruling

Commit: `4716b0cb5cf837f47a1bdbd558086e577cb95333` (`4716b0c`), its own commit
on `master`, unsigned (`%G?` -> `N`), two files staged explicitly, not pushed
(`git rev-list --count @{u}..HEAD` -> 2, i.e. `e477e37` and this one).

## Ruling

Both refutations accepted. The owner ruled that the macOS clause be added in
the `shasum -a 256 -c SHA256SUMS` form, on the argument the refutation itself
exposed: `docs/INSTALL.md` names macOS 11+ as supported, so a `sha256sum`
instruction would fail on 11 and 12, while `shasum -a 256` covers the whole
documented range. Linux and Windows halves unchanged, artifact name kept at
the corrected `windows-x86_64` form, `draft-body.md` lines 2-4 untouched.

## The claim verified first: one `SHA256SUMS` file serves all three commands

The sentence now asserts that the same `SHA256SUMS` works for `sha256sum -c`,
`shasum -a 256 -c` and the `Get-FileHash` comparison. The `shasum` half is the
one a user would discover false the hard way, so it was verified before
writing - and verified at the level of the code that actually parses the file,
not from documentation alone.

**What produces the file.** `.github/workflows/release.yml`, step "Generate
SHA256SUMS (D90)": `sha256sum * > SHA256SUMS`, run on the `ubuntu-22.04`
assemble leg. So the file is GNU coreutils text-mode output:
`<64 hex><space><space><filename>`.

**What consumes it on macOS.** `shasum` is the Perl script from the
`Digest::SHA` distribution; the macOS man page footer names `perl v5.30.3`,
which bundles Digest-SHA **6.02**. I fetched that exact release's script
(<https://fastapi.metacpan.org/source/MSHELOR/Digest-SHA-6.02/shasum>, header
`## Version: 6.02`). `Digest::SHA` is not installed on this Fedora box and I
did not install it, so instead of running the script I read and exercised its
check-mode parser, which is pure Perl and does not touch the digest engine:

- `sub verify` (line 247) parses each non-BSD line with
  `/^[ \t]*(\\?)([\da-fA-F]+)[ \t]([ *^U])(.+)/` (line 268): hex digest, one
  space or tab, then a **mode symbol** that must be one of `` `` `*` `^` `U`,
  then the filename. GNU text-mode output's *second* space is that mode
  symbol (` ` = text), so the format matches exactly.
- The algorithm is taken from the digest **length**, not from `-a`:
  `$alg = $len2alg{length($sum)}` (line 269) with
  `%len2alg = (40 => 1, 56 => 224, 64 => 256, 96 => 384, 128 => 512)`
  (line 229). `-a` only overrides the 56/64 entries when it is `512224` or
  `512256` (lines 230-231). With `-a 256`, a 64-hex line resolves to SHA-256.
  So `-a 256` is correct and harmless here; the man page's caveat ("When
  verifying SHA-512/224 or SHA-512/256 checksums, indicate the algorithm
  explicitly") is precisely about the collision `-a 256` does not have.

**Empirical run.** Built a `SHA256SUMS` the way release.yml does
(`sha256sum * > SHA256SUMS` over three dummy artifacts named like the real
ones), then ran the verbatim 6.02 regex + `%len2alg` + `%isAlg` +
definedness gate over it:

```
=== A. real SHA256SUMS (produced by GNU sha256sum *) ===
  PARSED alg=SHA-256 mode=text file=muxsmith-1.0.0-linux-x86_64.tar.gz
  PARSED alg=SHA-256 mode=text file=muxsmith-1.0.0-macos-arm64.dmg
  PARSED alg=SHA-256 mode=text file=muxsmith-1.0.0-windows-x86_64.msi
  -> 3 of 3 lines accepted as SHA-256
```

Acceptance alone proves nothing (a parser that accepts everything would print
the same), so four controls were run that the parser must **reject**; all four
fired: a single space with no mode symbol, a non-hex digest, a 63-hex
truncation (no `%len2alg` entry), and a digest with no filename. Real
`sha256sum -c SHA256SUMS` on the same file also returned three `OK` lines,
exit 0, as the Linux-side control.

Claim holds: one `SHA256SUMS`, three commands. Nothing to stop and report.

Residual, stated rather than hidden: the parser was exercised, the digest
computation was not (no `Digest::SHA` locally). The digest side is not in
question - SHA-256 of a byte string is SHA-256 - and `shasum`'s own header
states it "mimics the behavior of the combined GNU sha1sum ... sha256sum ...
programs, you can install this script as a convenient drop-in replacement".

## Edit applied

| File | Change |
|---|---|
| `docs/INSTALL.md` | Intro sentence: `` `shasum -a 256 -c SHA256SUMS` on macOS`` inserted between the Linux and Windows clauses. |
| `.github/release/draft-body.md` | Same insertion in the one-sentence release-body form (keeping that file's `or` connective). |

Both are the brief's original edit-1 replacement text restored in full, with
the artifact name at the corrected `windows-x86_64`.

## Verification

Wrap-aware exact-substring counts again (`scratchpad/check2.py`), every OLD
pattern observed non-zero before the edit:

| Pattern | before | after |
|---|---|---|
| INSTALL.md ``…on Linux, and in PowerShell on Windows\n`` | 1 | 0 |
| INSTALL.md ``…on Linux, `shasum -a 256 -c SHA256SUMS` on\nmacOS, and in PowerShell on Windows\n`` | 0 | 1 |
| draft-body ``…on Linux, or in PowerShell on Windows\n`` | 1 | 0 |
| draft-body ``…on Linux, `shasum -a 256 -c SHA256SUMS` on\nmacOS, or in PowerShell on Windows\n`` | 0 | 1 |

Invariants that had to stay at 1 and did, before and after: the INSTALL.md
Windows code span plus its following line; the draft-body Windows code span
plus its following line; the draft-body "Verify downloads: put `SHA256SUMS`
beside the files and run" lead-in. `draft-body.md` lines 2-4 printed before
and after and byte-identical; the diff hunk starts at line 19, so they are
outside it.

`git diff --numstat`: `2 1 .github/release/draft-body.md`, `2 1
docs/INSTALL.md` - one line becomes two in each file, which is the whole
change. No other path modified.

`python3 scripts/ledger-lint.py` -> exit 0. Not pushed. No release touched.
