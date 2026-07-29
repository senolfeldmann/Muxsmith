# Reach-sweep residue fixes - implementation report

Base tree: `c38bb0b`, branch `master`, main worktree. Four prescribed text
repairs, no design latitude taken.

## Repair 1 - the rpm row names EPEL for RHEL 10

Two sites, deliberately character-identical, both changed identically.

`docs/INSTALL.md`, Linux artifact list.

Before:

```
- `muxsmith-<version>-linux-x86_64.rpm` - rpm distributions with glibc 2.39+ (Fedora 40+, RHEL 10+): `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`
```

After:

```
- `muxsmith-<version>-linux-x86_64.rpm` - rpm distributions with glibc 2.39+ (Fedora 40+; RHEL 10+ with EPEL for webkitgtk 4.1): `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`
```

`.github/release/draft-body.md`, artifact table.

Before:

```
| `muxsmith-__VERSION__-linux-x86_64.rpm` | rpm distributions with glibc 2.39+ (Fedora 40+, RHEL 10+) |
```

After:

```
| `muxsmith-__VERSION__-linux-x86_64.rpm` | rpm distributions with glibc 2.39+ (Fedora 40+; RHEL 10+ with EPEL for webkitgtk 4.1) |
```

## Repair 2 - the tar.gz row states its reach

`docs/INSTALL.md`. The one surviving violation of the rule: three Linux
siblings state reach, this one stated only shape.

Before:

```
- `muxsmith-<version>-linux-x86_64.tar.gz` - portable archive with both binaries; see its
  `README.txt`
```

After:

```
- `muxsmith-<version>-linux-x86_64.tar.gz` - any distro with glibc 2.39+, portable archive with
  both binaries: see its `README.txt`
```

Register kept: the siblings read `<artifact> - <reach>: <what you do>`, and
the reach clause is now the same string the release table's tar.gz row
already carried (`any distro with glibc 2.39+`), with the form clause
joined by comma exactly as that row joins `, CLI + GUI`.

## Repair 3 - the pinning-policy comment covers the pin itself

`.github/workflows/release.yml`, pinning-policy header comment. No pin, no
step, no action SHA touched.

Before:

```
# 2.39, so moving this pin moves the product's reach - and with it every
# user-facing text that states that floor or an artifact's reach.
```

After:

```
# 2.39, so moving this pin moves the product's reach - and with it every
# text that states that floor, an artifact's reach, or this pin. Find
# those texts by grepping the tree for the outgoing values (the old
# glibc version, the old image label); they sit in docs, release
# templates and bot config, and the set is not one anybody recalls.
```

Two changes, both prescribed: the category is widened by `or this pin` and
loses the `user-facing` qualifier (`renovate.jsonc` states the pin and is
not user-facing, so the old qualifier excluded exactly the file that went
stale); and the grep handle is added. The predicate form is preserved -
still a predicate over texts, not an enumeration of files, so a new file
that states the pin is covered the day it is written. `renovate.jsonc` is
deliberately not named for that reason.

The comment carries no historical count. An earlier draft asserted the
set "has already gone stale between two commits"; measuring it against
`git log` showed the residue actually spanned more than one follow-up
commit, so the unmeasured claim was dropped rather than restated with a
number that goes stale on the next repair.

## Repair 4 - the 1.0 README artifact table is born correct

`README.md`, the `placeholder(1.0)` install-table comment. One clause
added; no visible prose changed, no other paragraph touched, the two
figures retired earlier today stay retired.

Before:

```
<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
     docs/INSTALL.md, which already carries the per-OS install-hurdle
     steps; drop the WIP banner in the same pass -->
```

After:

```
<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
     docs/INSTALL.md, which already carries the per-OS install-hurdle
     steps; every row states the artifact's reach the way the install
     notes and the release body state it - the requirement itself, never
     a distro family standing in for it; drop the WIP banner in the same
     pass -->
```

## Verification of the RHEL/EPEL facts

Every claim in the new parenthetical was re-derived from an authoritative
source rather than taken from the brief.

**The rpm hard-requires the webkit 4.1 soname.** `package.json` pins
`@tauri-apps/cli` at `2.11.4`. In the tauri source at tag
`tauri-cli-v2.11.4`, `crates/tauri-bundler/src/bundle/linux/rpm.rs` adds
no default dependency at all - it only forwards `settings.rpm().depends`.
The defaults are injected one layer up, in
`crates/tauri-cli/src/interface/rust.rs`, inside the
`#[cfg(target_os = "linux")]` block:

```rust
    depends_deb.push("libwebkit2gtk-4.1-0".to_string());
    depends_deb.push("libgtk-3-0".to_string());

    libs.push("libwebkit2gtk-4.1.so.0".into());
    libs.push("libgtk-3.so.0".into());

    for lib in libs {
      let mut requires = lib;
      if arch64bits {
        requires.push_str("()(64bit)");
      }
      depends_rpm.push(requires);
    }
```

So the rpm carries `Requires: libwebkit2gtk-4.1.so.0()(64bit)` (and the
gtk3 soname). It is a hard soname requirement, not a recommendation, and
the requirement is on the soname rather than on a package name - which is
why the row names the capability (`webkitgtk 4.1`) rather than a distro's
package name.

**Stock RHEL 10 cannot satisfy it; EPEL 10 can.** Directory listings of
the published repositories, x86_64, `Packages/w/`:

| Repository | webkit packages found |
|---|---|
| Rocky 10 BaseOS | none |
| Rocky 10 AppStream | none |
| Rocky 10 CRB | none |
| EPEL 10 Everything | `webkit2gtk4.1`, `webkit2gtk4.1-devel`, `webkit2gtk4.1-doc`, `webkitgtk6.0` (+devel/doc) |
| Rocky 9 AppStream (control) | `webkit2gtk3`, `webkit2gtk3-devel` (no 4.1) |
| Fedora 40 (control) | `webkit2gtk4.1` (+devel/doc), `webkit2gtk4.0` |

The three Rocky 10 negatives are not empty-listing artifacts: the same
fetches return other `w` packages from those directories (`watchdog`,
`wavpack`, `wayland-*`, `waypipe` in AppStream; `wavpack-devel`,
`webrtc-audio-processing-devel`, `wireshark-devel`, `woff2-devel` in CRB),
so the listing was reached and parsed and the webkit absence is real.
The Rocky 9 control also fires the same grep against a known-present
package, which rules out a malformed pattern.

Conclusion: `Fedora 40+` stands unchanged and verified; `RHEL 10+` is only
true with EPEL enabled, which is what the row now says. No disagreement
with the reviewer's finding.

## The identity/agreement check and its fire

Instrument: `reach-agree.sh` (kept in the session scratchpad, not added to
the repo - it is a one-off check for this edit, not a gate part). It
extracts the rpm reach text from both documents and compares them
byte-for-byte, extracts the tar.gz reach clause from both and compares
those, prints every extracted value in brackets, and fails loudly if any
extraction comes back empty.

Real run, after the edits:

```
rpm    install: [rpm distributions with glibc 2.39+ (Fedora 40+; RHEL 10+ with EPEL for webkitgtk 4.1)]
rpm    draft  : [rpm distributions with glibc 2.39+ (Fedora 40+; RHEL 10+ with EPEL for webkitgtk 4.1)]
tar.gz install: [any distro with glibc 2.39+]
tar.gz draft  : [any distro with glibc 2.39+]
OK   rpm cells are character-identical
OK   tar.gz reach clauses agree
exit=0
```

Fired three ways against deliberately broken copies, each time confirming
the check reports the defect rather than passing silently:

1. **rpm mismatch** - `Fedora 40+` changed to `Fedora 41+` in the draft
   copy: `FAIL rpm cells differ`, exit 1, while the tar.gz check correctly
   stayed OK.
2. **tar.gz disagreement** - the install copy's reach changed to
   `glibc 2.35+`: `FAIL tar.gz reach clauses disagree`, exit 1, while the
   rpm check correctly stayed OK.
3. **extraction guard** - the tar.gz row deleted from the install copy:
   `EXTRACTION FAILED: tar.gz reach (install) is empty` plus exit 1. This
   is the fire that matters most, since a passing result here is an
   absence: it proves a silently missed row is reported rather than read
   as agreement.

Each fire was run on a copy; the working tree was never mutated.

## Gate result and its limit

All 11 parts as `BUILDING.md` enumerates them, foreground, green:

| # | Part | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | pass |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| 3 | `cargo test --workspace` | pass |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | pass |
| 5 | `cargo deny check` | pass (advisories ok, bans ok, licenses ok, sources ok) |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | pass |
| 7 | `pnpm lint` | pass |
| 8 | `pnpm build` | pass |
| 9 | `pnpm check:i18n` | pass |
| 10 | `pnpm test:e2e` | pass, 68 passed |
| 11 | `python3 scripts/ledger-lint.py` | pass, 546 entries across 4 files plus BUILDING.md's gate enumeration |

**Limit, stated plainly: no gate part reads any of the four files this
change touches.** Parts 1-6 read Rust sources, 7-10 read the frontend and
e2e sources, and part 11 reads the four house-knowledge YAML files plus
`BUILDING.md` (confirmed by reading `scripts/ledger-lint.py`'s file
handling, whose only non-YAML target is `BUILDING.md`). A green gate is
therefore evidence that nothing else broke, and no evidence at all about
the correctness of these edits. That evidence comes from the fact
verification and the fired identity check above.

Two checks were run outside the gate because the gate has no equivalent:
`release.yml` was parsed with `yaml.safe_load` (parses; jobs `guard`,
`bundle`, `assemble`) so a comment edit cannot have broken the workflow
document, and `ledger-lint.py` was re-run after the final wording change.

## git diff --stat

```
 .github/release/draft-body.md | 2 +-
 .github/workflows/release.yml | 5 ++++-
 README.md                     | 5 ++++-
 docs/INSTALL.md               | 6 +++---
 4 files changed, 12 insertions(+), 6 deletions(-)
```

## Concerns

1. **The brief's verification bar asks for a diff covering "the three
   files", but the four repairs necessarily touch four.** Repair 1 is a
   two-site repair: the brief states the draft-body rpm cell must stay
   character-identical after the edit, which cannot be achieved without
   editing `.github/release/draft-body.md`. I edited all four rather than
   dropping a site, because dropping one would break the equality the
   brief calls deliberate. Reading the miscount as a typo, not as an
   instruction to leave a site stale.
2. **The reviewer's stated reason for repair 1 needed a correction of
   locus, not of substance.** The soname requirement is not in
   `tauri-bundler`'s rpm module (which has no defaults at all at the
   pinned tag) but in `tauri-cli`'s interface layer. The finding itself -
   a hard `libwebkit2gtk-4.1.so.0` requirement - holds. Recording it
   because a future session re-checking "the packaging source" will look
   in the bundler and find nothing.
3. **The rpm row now names a capability the rpm requires by soname, not
   by package name.** `webkitgtk 4.1` matches how the runtime-requirements
   section further down the same document already phrases it, so the
   document is internally consistent; but a reader on a distro whose
   package name differs from both `webkit2gtk4.1` and
   `libwebkit2gtk-4.1-0` still has to map capability to package name
   themselves. That is inherent to a soname requirement and not worth
   more words in a table row.
4. **The EPEL qualifier is verified against Rocky 10 as the RHEL 10
   rebuild, not against a Red Hat subscription repository.** Rocky's
   BaseOS/AppStream/CRB are rebuilds of RHEL's, so the absence carries
   over, but a RHEL subscriber with additional entitled repositories is
   not covered by the measurement. The practical answer is unchanged:
   EPEL is where `webkit2gtk4.1` comes from on EL10.
5. **The tar.gz reach agreement is now checked by nothing standing.** The
   two documents agree today, and the check that proved it lives in the
   session scratchpad. Nothing in the gate would catch a future edit that
   moves one and not the other - the same class of drift this whole sweep
   is repairing. Whether that deserves a gate part is a controller
   decision, not one I took at the keyboard.

## Commit

`709929ce1c89ce4de0d11efafdb5cba440bb1dd6` (`709929c`) -
`docs: the rpm needs EPEL on RHEL, the tar.gz row states its reach, the pin comment covers itself`

Single trailer verified (`git log -1 --format='%(trailers)'` returns only
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`), staged by name,
not pushed. The report itself is untracked by design: `.gitignore` line 2
excludes `.superpowers/`.
