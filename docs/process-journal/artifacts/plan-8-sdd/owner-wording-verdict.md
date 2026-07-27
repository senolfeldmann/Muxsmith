# Plan-8 owner rendered-surface wording pass: review verdict

## VERDICT: APPROVED

Both graded commits land the eight ruled edits faithfully, touch nothing
outside the three named files, leave `.github/release/draft-body.md`
lines 2-4 byte-identical, and rest on factual claims that all hold when
checked at the source. The two refuted premises were refuted correctly and
what replaced them is right. Findings below are routed forward, none of
them blocks this pass.

## What I graded

- Range `adb0f6e..4716b0c`, i.e. exactly `e477e371a90321d979f21a8fe7ef3e0829d864b1`
  (`e477e37`) and `4716b0cb5cf837f47a1bdbd558086e577cb95333` (`4716b0c`).
- HEAD moved during the review: `0b8e70e` ("house: plan-8 blocked-pool
  dispositions") landed on top. `git diff --name-status 4716b0c HEAD` lists
  only `docs/decision-ledger.yaml` and `docs/process-conventions.yaml`, so
  the three graded files are still exactly as committed.
- `origin/master` is at `adb0f6e`; `git rev-list --count @{u}..HEAD` = 3.
  Neither wording commit has been pushed.

## Externally verified claims (not accepted from the report)

### 1. The checksum commands

- **`sha256sum` is GNU coreutils.** GNU coreutils manual, sha2-utilities
  node: "The commands `sha224sum`, `sha256sum`, `sha384sum` and `sha512sum`
  compute checksums of various lengths (respectively 224, 256, 384 and 512
  bits), collectively known as the SHA-2 hashes." **Holds.**
- **macOS ships `shasum`, and `-a 256 -c` is real.** Apple's shipped
  `shasum(1)`, NAME "shasum - Print or Check SHA Checksums", `-a` accepting
  "1 (default), 224, 256, 384, 512, 512224, 512256", `-c, --check` = "read
  SHA sums from the FILEs and check them", footer "perl v5.30.3".
  **Holds.**
- **The brief's macOS premise was indeed wrong, as the implementer found.**
  Apple's `md5(1)`, footer "February 13, 2024 | Mac OS X 14", NAME line
  carries `sha256sum`, DESCRIPTION: "The md5sum, sha1sum, sha224sum,
  sha256sum, sha384sum, and sha512sum utilities do the same, but with
  command-line options and an output format that match those of their
  similary named GNU utilities", and its `-c` reads a digest file "in either
  classical BSD format or in GNU coreutils format". The refutation is
  correct, and the replacement (`shasum -a 256`) is the form that also
  covers macOS 11/12, which `docs/INSTALL.md` still claims support for.
  **The shipped text is right independently of how the 11/12 question
  resolves**, which is the property that matters.
- **`Get-FileHash` is the Windows equivalent.** Microsoft Learn,
  `Get-FileHash` (Microsoft.PowerShell.Utility): syntax
  `Get-FileHash [-Path] <String[]> [[-Algorithm] <String>]`, Accepted values
  "SHA1, SHA256, SHA384, SHA512, MD5", Default value SHA256, and the page
  carries a `powershell-5.1` moniker (the in-box Windows PowerShell).
  **Holds.**

### 2. The glibc floor

Two steps, both re-derived, not borrowed:

1. `.github/workflows/release.yml:80-81` - `- leg: linux-x86_64` /
   `os: ubuntu-22.04`; the same label on the assemble job (`:186`) and the
   pinning-policy comment at `:21-23` recording the deliberate divergence
   from the test matrix.
2. `https://packages.ubuntu.com/jammy/libc6` fetched directly:
   "Package: libc6 (2.35-0ubuntu3.14 and others)", source tarball
   `2.35.orig.tar.xz` - upstream **2.35**.

**Holds.** Positive note: `docs/ROADMAP.md:319-322` already carries the
registered trigger that updates this number ("move the Linux release leg to
`ubuntu-24.04` AND record the raised glibc/webkit floor in docs/INSTALL.md
and the tar.gz README requirement line in the same change"), so the new
hard-coded 2.35 is not an orphan fact.

### 3. One file, three commands

I did not re-read the implementer's regex extraction; I ran the real script.

- `Digest::SHA` 6.02 is what macOS's perl ships: fetched
  `perl5@v5.30.3:cpan/Digest-SHA/lib/Digest/SHA.pm` (`$VERSION = '6.02'`)
  and `perl5@v5.30.3:cpan/Digest-SHA/shasum`; the latter is **byte-identical**
  (`diff` clean) to the MSHELOR `Digest-SHA-6.02` release script.
- GNU output shape, measured not assumed: `sha256sum * > SHA256SUMS` with
  coreutils 9.10, then `od -c` on line 1 shows 64 hex bytes followed by
  **two** spaces then the filename.
- End-to-end run, not parser-reading: `Digest::SHA` is absent on this box and
  installing it would be a system change, so I stood up a 55-line
  `Digest/SHA.pm` shim providing only the three calls `shasum` makes
  (`new`/`addfile`/`hexdigest`, digest delegated to coreutils) and ran the
  **unmodified** perl-5.30.3 `shasum` against the GNU-produced file:

  ```
  $ perl -I<shim> p5303-shasum.pl -a 256 -c SHA256SUMS
  muxsmith-1.0.0-linux-x86_64.tar.gz: OK
  muxsmith-1.0.0-macos-arm64.dmg: OK
  muxsmith-1.0.0-windows-x86_64.msi: OK          exit=0
  ```

  Same result with no `-a` at all, confirming the algorithm comes from the
  64-char digest length (`%len2alg`), not from `-a`.
- Acceptance alone proves nothing, so four controls were run through the same
  real code path, all firing: a tampered artifact (`FAILED`, exit 1); a single
  space instead of two, i.e. no mode symbol ("no properly formatted SHA
  checksum lines found", exit 1); a 63-hex truncation (same, exit 1); a
  non-hex digest (same, exit 1).

**Holds**, and now on stronger evidence than the report carried.

## Findings

### MEDIUM - the instruction fails loudly for the ordinary single-artifact download

Not introduced by this pass, but this pass put it in front of the reader three
times instead of once, so it should be ruled now rather than after a user hits
it. `SHA256SUMS` lists all seven artifacts. A user who downloads one artifact
plus `SHA256SUMS` and follows the doc gets, measured on both commands:

```
$ sha256sum -c SHA256SUMS                     # Linux clause
sha256sum: muxsmith-1.0.0-linux-x86_64.tar.gz: No such file or directory
muxsmith-1.0.0-linux-x86_64.tar.gz: FAILED open or read
muxsmith-1.0.0-macos-arm64.dmg: OK
sha256sum: muxsmith-1.0.0-windows-x86_64.msi: No such file or directory
muxsmith-1.0.0-windows-x86_64.msi: FAILED open or read
sha256sum: WARNING: 2 listed files could not be read        exit=1
```

The real `shasum -a 256 -c` (macOS clause) produces the identical shape and
exit 1. A reader checking whether a download was tampered with sees `FAILED`
and a non-zero exit in the normal case. The rehearsal never surfaced this
because R4 downloaded all eight assets.

Both commands support the fix, verified by running it: `--ignore-missing`
(GNU coreutils, feature introduced in coreutils-8.25 per the coreutils NEWS
file; Digest::SHA 6.02 documents it in `shasum`'s own usage text at line 48
and honours it at line 281) turns both into a single `OK` line, exit 0.
Windows is unaffected (`Get-FileHash` names one file).

Routing: shipped prose, so the owner's call - either add `--ignore-missing`
to the two `-c` forms, or a clause saying a partial download reports the
absent files. Out of scope for this pass; the reviewer's nine items did not
contain it.

### LOW - the Windows comparison crosses a case boundary silently

`Get-FileHash` prints uppercase hex (Microsoft Learn's own example output:
`Hash : 3CBCFDDEC145E3382D592266BE193E5BE53443138EE6AB6CA09FF20DF609E268`);
`SHA256SUMS` is lowercase (measured above, `ca978112...`). "compared against
that file's line in `SHA256SUMS`" leaves a reader eyeballing two strings that
differ in case throughout. One clause ("the comparison is case-insensitive")
removes the hesitation.

### LOW - the Windows example names one of the two Windows artifacts

`docs/INSTALL.md:10` hard-codes `muxsmith-<version>-windows-x86_64.msi` while
the file's own Windows section (`:25-26`) names both the x86_64 and the arm64
msi. A Windows-on-ARM reader has to notice that the artifact name, not just
`<version>`, needs substituting. `.github/release/draft-body.md:24` already
does the cleaner thing with `<file>`. The concrete name was what the brief
asked for, so this is a suggestion, not a defect in execution.

### LOW - "the deb, rpm or tar.gz if you want the CLI on your PATH"

Precision nit on owner-ruled text (`docs/INSTALL.md:16-17`). The deb and rpm
do put the CLI on PATH - the same file says so at `:82-83` ("`/usr/bin`
(already on PATH)"). The tar.gz does not: its own README
(`packaging/linux-tarball-README.txt:10-12`) says "put the directory on your
PATH / symlink the binaries into `~/.local/bin`". The sentence reads as
"these three give you the CLI on PATH" when the tar.gz gives you the option
of it. Charitable reading is defensible; flagged because the neighbouring
sentence in the same file draws the distinction explicitly.

### LOW (controller-routed, the implementer was barred from this) - the frozen design now diverges from the shipped files at seven sites

`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
section 4 is headed "Documentation artifacts (verbatim)" and transcribes the
three shipped files in full. After this pass it no longer matches the tree.
Measured by extracting the fences and diffing against the working files:

- 4.1 `docs/INSTALL.md` (design fence 83 lines vs tree 93): five divergent
  hunks, carrying all six INSTALL.md edits.
- 4.2 release-body template: design lines 1695-1696 still read "Verify
  downloads: put `SHA256SUMS` beside the files and run
  `sha256sum -c SHA256SUMS`."
- 4.3 tarball README: design line 1736 still reads "- glibc from Ubuntu 22.04
  (2022) or newer; any current distribution".

Two reasons this is worth a line rather than shrugging off as normal
historical drift: the plan has an **Amendment log** used for exactly this
(A1 for a section-2 fence, A2 for the WiX supersede), and the immediately
preceding plan set the opposite precedent - `docs/ROADMAP.md:129-131` records
that plan-7.5's owner rendered-surface pass was "seven verbatim wording edits
across five files, **including the v1 spec's 8.2 sentence**". The implementer
could not act: `owner-wording-brief.md` explicitly forbids touching the design
or plan documents. So this is a controller/owner disposition - amend, or
record that section 4 is superseded by the tree.

### OBSERVATION - the pending draft cannot answer for the new lines, and does not need to

Both wording commits are unpushed (`origin/master` = `adb0f6e`), so the draft
`rehearsal-30273529210` was necessarily built from the pre-pass template and
carries the old one-line "Verify downloads" sentence. That does not
invalidate the owner's item-7 inspection: lines 1-20 of
`.github/release/draft-body.md` are byte-identical to `adb0f6e` (verified by
comparing the blobs, with the whole-file-changed control confirming the
comparison is live), so lines 2-4 render exactly as he will see them. Worth
knowing only because the answer now applies to three wrapped regions in that
file, not one: lines 2-4 (item 7), the pre-existing paragraph at 6-9, and the
new four-line sentence at 21-25. GitHub's own docs are explicit that the
rendering differs by context - a single newline is "render[ed as] a line break
automatically" in comments/issues/PRs, whereas "if you are writing in an .md
file, the example above would render on one line" - which is why the item was
deferred in the first place.

## Dimensions

**1. Fidelity - PASS.** Fourteen wrap-aware, whitespace-normalised
exact-substring checks over the three files (`scratchpad/fidelity.py`): every
one of the eight ruled replacements present exactly once in the brief's own
words, every corresponding old string at zero. The check harness is
demonstrably sensitive rather than vacuous: `one-time steps per OS` counts 1
while the one-character variant `one-time stepS per OS` counts 0. Nothing
outside the three files changed in either commit
(`git diff --name-status adb0f6e e477e37` = the three files;
`e477e37 4716b0c` = two of them). `draft-body.md` lines 2-4 untouched, as
above.

The two brief deviations are both correct and both were surfaced rather than
silently applied: `windows-x64` occurs nowhere in the tree (swept
tree-wide, zero hits, with `windows-x86_64` as the positive control returning
hits in six files), and the macOS clause was withheld until the owner ruled it
back in.

**2. Correctness in context - PASS.** The AppImage sentence is the one I
pushed hardest on. Two halves:

- *Both binaries are inside the image*: `task-6-report.md` R6, both the first
  run and the re-run, shows `--appimage-extract` yielding
  `squashfs-root/usr/bin/muxsmith` and `squashfs-root/usr/bin/muxsmith-gui`.
- *Only the GUI is directly runnable*: verified at the bundler source rather
  than assumed. `src-tauri/Cargo.toml` names the main binary `muxsmith-gui`
  and `src-tauri/tauri.bundle.conf.json` makes `binaries/muxsmith` an
  `externalBin` sidecar. In tauri-bundler at tag `tauri-cli-v2.11.4`,
  `linux/appimage/linuxdeploy.rs` installs AppImageKit's `AppRun` at the
  AppDir root beside `{product_name}.desktop`, and
  `linux/freedesktop/mod.rs:103,176` sets that desktop file's `Exec` to
  `settings.main_binary_name()`. So an executed AppImage starts
  `muxsmith-gui`; the sidecar is only reachable by extracting (or mounting)
  the image. The sentence holds.

The glibc floor, the three checksum commands and the one-file/three-commands
claim are covered above. The `/usr/local/bin` fix, the User-variables pane and
the `GUI only:` label are each true of the artifacts and consistent with the
rest of the file.

**3. Completeness - PASS, on an explicitly named surface.** Surface swept =
the repo's shipped and user-facing text: `README.md`, `BUILDING.md`,
`LICENSE`, `docs/INSTALL.md`, `docs/ROADMAP.md`, `docs/IDEAS.md`,
`packaging/`, `.github/release/` (both `draft-body.md` and
`rehearsal-banner.md`), `help/`, `locales/`. Deliberately excluded and named
as such: `docs/superpowers/` and `docs/process-journal/` (frozen SDD and
historical records - see the LOW finding for what that exclusion costs here),
`.github/workflows/release.yml` (the emitter, correctly still using
`sha256sum`), and `crates/`/`src-tauri/`/`src/` (code).

Results: checksum-command instructions exist at exactly the two fixed sites.
`/usr/local/bin` appears once more as an install instruction (the fixed
macOS symlink) plus four `locales/*.ftl` hits that are mkvmerge *detection
hints* ("typically ... /usr/local/bin/mkvmerge"), a different class - a
non-existent probe path costs nothing. No further "one-time step" singular
(`docs/IDEAS.md:106` is "one timeline", a false positive). No further
glibc/distro shorthand on the shipped surface.

The first run of this sweep returned four empty results and was wrong. I had
written the pathspec list into a shell variable and passed it unquoted; zsh
does not word-split, so `git grep -- $SURFACE` searched one nonsense pathspec
and reported nothing. Proven, not assumed: `git grep -c sha256sum -- $SURFACE`
exits 1 with zero hits while `git grep -c sha256sum -- README.md
docs/INSTALL.md` returns `docs/INSTALL.md:1`. Every number above comes from
the re-run with explicit arguments.

**4. House conformance - PASS.** By entry id:

- `proc-05-commit-signing` - both commits `%G?` = `N`.
- `proc-07-verify-against-source` and `proc-57-briefs-not-ground-truth` -
  the load-bearing premises were taken to Apple's man pages, Microsoft Learn,
  the GNU manual, the Ubuntu package index and the perl5 source tree; two were
  refuted and the refutations were recorded, which that entry defines as a
  valid completion.
- `brief-drafts-verified-against-tree` - the `windows-x64` literal was
  checked against the tree, corrected, and surfaced in the report and the
  commit message rather than transcribed.
- `proc-verification-step-must-be-falsifiable` /
  `proc-check-green-state-reachable` - every old-string zero in the report was
  preceded by an observed non-zero, and the implementer disclosed the one
  probe it had to correct mid-run rather than hiding it.
- `proc-wrapped-prose-quote-grep` - the implementer's counting was
  whitespace-tolerant and it caught its own line-oriented blind spot on the
  edit-6 probe; my own checks use the same discipline.
- `proc-sweep-surface-completeness` - satisfied by naming the surface and its
  exclusions explicitly above; see also HARVEST H1.
- `proc-latitude-clause-boundary` - the one genuine fork the task hit (the
  macOS clause) was routed to the owner as a decision memo with three options
  and a recommendation, not resolved at the keyboard.

Typography: all three files are pure ASCII (`grep -P '[^\x00-\x7F]'` over each
file returns nothing; the pattern is sound - the same expression fires on
`src-tauri/tauri.conf.json`, which carries "Şenol"). No em/en dash, curly
quote, Unicode ellipsis or NBSP in any added line. `docs/process-journal/PROMPT.md:26`
is the recorded rule ("ASCII punctuation only (no em-dashes, no curly quotes) -
proper names keep their real orthography").

**5. Commit hygiene - PASS.** Two commits, not one: `4716b0c` exists because
the owner issued a further ruling after `e477e37`, which the report discloses
in a "Superseded in part" headline at the top of the affected section and a
"Follow-up" section, and which this review brief itself directs me to grade.
Both are unsigned (`%G?` = `N`), both carry
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, neither is pushed,
and no path outside the three files appears in either. Explicit staging is
not retroactively provable from git, but its observable consequence - no
foreign path in either commit, while the working tree carried other agents'
changes throughout - holds. `python3 scripts/ledger-lint.py` exits 0 here
("474 entries across 4 files, all invariants hold"; the report's 473 is
explained by `0b8e70e` landing afterwards). No GitHub release was created,
edited, published or deleted by this pass or by this review; I did not call
`gh` at all.

## HARVEST

**H1. A shell variable holding a pathspec list is a false-empty generator in
zsh.** `SURFACE="a.md b.md"; git grep -n pat -- $SURFACE` silently searches
one nonsense pathspec and reports zero hits; the same call with the paths
written out returns them. This is a new mechanism in the
`proc-sweep-surface-completeness` false-empty family (previous recorded
mechanisms: `grep -I` classifying minified JS as binary, `see the [^)]*topic`
blind to parenthesised titles, a non-Rust sweep missing a `.ts` file). The
handgriff is mechanical, not attentional: **a surface sweep names its paths as
literal arguments, and its first run is against a file known to contain the
pattern.** If the control does not fire, the sweep is not evidence yet.

**H2. Same class, my own tooling, same session.** My extractor for the design's
verbatim fences assumed four-backtick fences; section 4.3 uses three, so it
returned an empty region and would have "proved" the tarball README matches
the design. It was visible only because the diff header printed the extracted
size ("design fence 0 lines vs tree 27 lines"). Handgriff: **an extractor
reports the size of what it extracted**, so an empty extraction is a visible
anomaly rather than a clean diff.

**H3. A missing dependency is a reason to shim, not a reason to downgrade the
verification.** The implementer read `shasum`'s check-mode regex because
`Digest::SHA` is not installed and installing it is Şenol's call. A 55-line
stand-in providing the three methods the script calls, with the digest
delegated to coreutils, let the *unmodified* upstream script run end to end -
same no-system-change constraint, an order of magnitude more evidence, and the
malformed-line controls fire through the real code path instead of through a
transcribed regex. Registerable as a general move: when the blocked
verification has one missing leaf dependency and the thing under test is the
caller, substitute the leaf.

**H4. Test the shape the user has, not the shape the rehearsal produced.**
Every checksum check in this plan - R4 in the rehearsal, the implementer's
run, the design's D90 acceptance - was performed with **all** artifacts
present, because that is what a CI assemble job leaves in a directory. No one
ran the shape a user actually has: one artifact plus `SHA256SUMS`. That single
untried case is where the MEDIUM finding lives. The trigger is readable: you
are verifying an instruction aimed at an end user, and your fixture came from
the build pipeline.
