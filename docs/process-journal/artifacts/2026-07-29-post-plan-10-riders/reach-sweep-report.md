# Implementer report - make every artifact-reach claim true, once

**Status:** COMPLETE. Commit `c38bb0b` (`c38bb0b01587e00933aecfff7b20d01a67931d77`),
subject `docs: every artifact row states the reach it actually has`, unsigned,
one trailer, not pushed. Tree before the change: `d9a4fa2`.

Three files modified, five sites, exactly as the brief enumerated. No fork was
found on contact, so nothing returned as NEEDS_CONTEXT.

## Per site: before and after

### `docs/INSTALL.md` - the artifact list's three Linux rows

| Row | Before | After |
|---|---|---|
| deb | `Debian/Ubuntu` | `Debian 13+ / Ubuntu 24.04+` |
| rpm | `Fedora & co.` | `rpm distributions with glibc 2.39+ (Fedora 40+, RHEL 10+)` |
| AppImage | `any distro` | `any distro with glibc 2.39+` |

The deb row now carries the release table's phrase verbatim, so the two
documents agree where one of them had already been repaired. The rpm row states
the requirement and anchors it in the two rpm families whose floor release is
verifiable (below); "& co." is gone because it was the false part - EL9 and
openSUSE Leap 15.6 are "& co." and both sit under the floor. The AppImage row
names the floor compactly rather than pointing forward, because it is the row a
reader meets before the floor paragraph.

### `.github/release/draft-body.md` - two rows

| Row | Before | After |
|---|---|---|
| rpm | `Fedora & co.` | `rpm distributions with glibc 2.39+ (Fedora 40+, RHEL 10+)` |
| tar.gz | `portable, CLI + GUI` | `any distro with glibc 2.39+, CLI + GUI` |

The rpm cell is character-identical to INSTALL.md's, so the two tables cannot
give two answers to "which systems run the rpm". The tar.gz cell keeps its form
statement and gains the reach its three siblings state; "any distro with glibc
2.39+" is the phrase the AppImage row already used, so the table now uses one
formulation for one fact.

Untouched, as required: the three non-Linux rows, the deb row (already true),
the AppImage row (already true), and the rpm-versus-deb split.

### `.github/workflows/release.yml` - the pinning-policy comment

Before:

```
# tests run on the newest. That base sets the shipped glibc floor at
# 2.39; docs/INSTALL.md and the tar.gz README state it, so moving this
# pin moves the product's reach and those two texts with it.
```

After:

```
# tests run on the newest. That base sets the shipped glibc floor at
# 2.39, so moving this pin moves the product's reach - and with it every
# user-facing text that states that floor or an artifact's reach.
```

The count and the enumeration are both gone rather than corrected: a category
("every user-facing text that states the floor or an artifact's reach") cannot
be falsified by a text being added, which is what "two" and "those two texts"
were doing. Nothing else in the comment or the file changed - no pin, no step,
no action SHA; the YAML parse below proves it.

## Verification of every distribution figure written

Five figures appear in the new text: Debian 13, Ubuntu 24.04, Fedora 40,
RHEL 10, glibc 2.39. Each was measured against a source, not transcribed.

| Claim written | Measured | Source |
|---|---|---|
| Debian 13+ meets the floor | Debian 13 (trixie) ships `libc6 2.41-12+deb13u3` | packages.debian.org/trixie/libc6 |
| Ubuntu 24.04+ meets the floor | noble ships glibc `2.39-0ubuntu8.8` (upstream 2.39) | launchpad.net/ubuntu/noble/+source/glibc |
| Fedora 40+ meets the floor | Fedora 40 = 2.39; Fedora 39 = 2.38, so 40 is exactly the boundary | Repology glibc versions (Fedora 39/40/41 = 2.38/2.39/2.40) |
| RHEL 10+ meets the floor | EL10 = 2.39: CentOS Stream 10 = 2.39, AlmaLinux 10 ships `glibc-2.39-*.el10.alma.1` | Repology (CentOS Stream 10); rpmfind AlmaLinux 10 baseos |
| glibc 2.39 is the floor | Build base is `ubuntu-24.04` = glibc 2.39 (same Launchpad figure) | release.yml's runner pin + Launchpad |

The brief's two motivating measurements, re-measured independently, and both
agree with the reviewer:

| Reviewer's figure | My measurement | Source |
|---|---|---|
| EL9 at `glibc-2.34-274.el9_8` | EL9 = 2.34 (CentOS Stream 9 = 2.34, AlmaLinux 9 = 2.34) | Repology glibc versions |
| openSUSE Leap 15.6 at 2.38 | Leap 15.6 ships `glibc 2.38-150600.14.26.1` | SUSE update announcement SUSE-RU-2025:1198-1 |

No disagreement to report. Debian 12 (2.36) and Ubuntu 22.04 (2.35) appear in
the unchanged floor paragraph and tar.gz README; I did not re-measure them
because I did not write them, but the Repology table showed Debian 12 = 2.36
consistent with the existing text.

Deliberately **not** written: any openSUSE Leap figure. Leap 15.6 is below the
floor and Leap 16's glibc I could not measure from a source I could read
directly, so the rpm row states the requirement and names only families whose
boundary release I verified. That is also why the row leads with "rpm
distributions with glibc 2.39+" rather than a family list: the requirement is
the true statement, the two families are the reader's handle.

## Completion check, with its fire

Criterion: after the change, no Linux artifact row in either table asserts a
reach the artifact does not have.

The check is a script
(`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/reach_check.py`,
deliberately outside the repo so the diff stays at three files). It parses the
Linux artifact rows out of both documents - bullets in INSTALL.md with
continuation lines joined, table rows in the draft body - and applies two rules:

- **R1, both tables:** a reach text that names a distribution family or a scope
  quantifier (`any`, `all`, `every`, `& co.`, `distro`, `distributions`) must
  carry a floor qualifier: literally `glibc 2.39+`, or a family named with a
  version at or above that family's measured floor release. The
  family-to-floor-release table is the measured one above, so `Debian 12+`
  would fail where `Debian 13+` passes.
- **R2, release table only:** every Linux row must carry a floor qualifier at
  all. Its three siblings state reach, so a row stating none reads as an
  exemption - this is the rule that catches `portable, CLI + GUI`, which R1
  cannot see because it asserts nothing.

Guards on the instrument itself: the script exits 2 if it does not find exactly
four Linux rows per document (so a regex that silently matches nothing cannot
pass as "clean"), and it prints a WARN for any capitalised token in a reach
text that is neither a known family nor a known non-family word, so the family
enumeration cannot silently miss one.

**Fire on the pre-state** (working tree at `d9a4fa2`, before any edit): exit 1,
six violations across the five sites in the two tables:

```
R1 docs/INSTALL.md: ...deb -> 'Debian/Ubuntu' asserts reach without the glibc 2.39 floor
R1 docs/INSTALL.md: ...rpm -> 'Fedora & co.' asserts reach without the glibc 2.39 floor
R1 docs/INSTALL.md: ...AppImage -> 'any distro' asserts reach without the glibc 2.39 floor
R1 .github/release/draft-body.md: ...rpm -> 'Fedora & co.' asserts reach without the glibc 2.39 floor
R2 .github/release/draft-body.md: ...rpm -> 'Fedora & co.' states no reach while its Linux siblings do
R2 .github/release/draft-body.md: ...tar.gz -> 'portable, CLI + GUI' states no reach while its Linux siblings do
```

Six violations, five sites: the release rpm row trips both rules. The rows that
were already true (the release deb and AppImage rows) did not fire, which is
the second half of the control - the check discriminates rather than flagging
everything.

**End state:** exit 0, `OK: every Linux row states a reach the artifact has`,
over the same eight rows.

Limit of the check: it is a review instrument, not a gate part. It is not
committed, so nothing stops a future row from re-introducing a bare family
name; only the rule in the brief and a reviewer's eye do. Making it a twelfth
gate part is a scope decision for the owner, listed as a concern below.

## The gate

All 11 parts as `BUILDING.md` enumerates them, foreground, in this tree, each
exit code captured explicitly (not through a pipe - zsh does not populate
`${PIPESTATUS[0]}` the way the brief warns):

| # | Part | Exit |
|---|---|---|
| 1 | `cargo fmt --all --check` | 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| 3 | `cargo test --workspace` | 0 (all suites green) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 |
| 5 | `cargo deny check` | 0 (advisories ok, bans ok, licenses ok, sources ok) |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 |
| 7 | `pnpm lint` | 0 |
| 8 | `pnpm build` | 0 |
| 9 | `pnpm check:i18n` | 0 |
| 10 | `pnpm test:e2e` | 0 (68 passed) |
| 11 | `python3 scripts/ledger-lint.py` | 0 (546 entries across 4 files plus BUILDING.md's gate enumeration) |

**Its limit, verified rather than assumed:** no gate part reads any of the three
files this change touches, so a green gate is evidence about the rest of the
repo, not about this diff. Measured two ways: `scripts/ledger-lint.py` reads the
four house-knowledge YAML files and `BUILDING.md` only (its own file list), and
a repo-wide grep for `INSTALL`, `.github/release` and `workflows/release` across
everything but `target/`, `node_modules/`, `.git/`, `docs/` and `.superpowers/`
returns no test, script or source file - the only consumer of `draft-body.md` is
`release.yml`'s own assemble step at release time. That grep is a negative
result, so it was fired first: the same grep mechanics against a known-present
string (`BUILDING.md` under `scripts/`) returned `scripts/ledger-lint.py`.

The typography check is a negative result too, and was fired the same way: no
added line contains a non-ASCII byte, and the identical `grep -P "[^\x00-\x7F]"`
against a constructed em-dash line matched.

### YAML parse of `release.yml`

`yaml.safe_load` of the working-tree file parses to a dict with top-level keys
`['name', True, 'permissions', 'jobs']` (the `True` is YAML 1.1 coercing the `on`
key, unchanged by this diff) and jobs `guard`, `bundle`, `assemble`, and
compares **equal** to `yaml.safe_load` of `git show HEAD~1:.github/workflows/release.yml`
(the parse discards comments, so structural equality is exactly the proof that
only the comment moved).

### `git diff --stat`

```
 .github/release/draft-body.md | 4 ++--
 .github/workflows/release.yml | 4 ++--
 docs/INSTALL.md               | 6 +++---
 3 files changed, 7 insertions(+), 7 deletions(-)
```

Exactly the three files; the working tree carried no other modification and no
untracked file when the commit was staged by name.

## Concerns

1. **`docs/INSTALL.md`'s tar.gz row is the one Linux reach a reader must
   assemble from two places.** It was not in the site list, so it still reads
   `portable archive with both binaries; see its README.txt` while the release
   table's tar.gz row now states `any distro with glibc 2.39+, CLI + GUI`. The
   two do not contradict - INSTALL.md's floor paragraph says "every Linux
   artifact here", which covers the tar.gz - and the row asserts no reach, so it
   does not violate the rule. But it is the only Linux row across the two tables
   whose answer is not on the same line as the artifact, and if the owner wants
   the tables to match row for row, that row is the remaining edit. I did not
   make it: the site list is fixed and a fourth row is scope, not wording.
2. **Two archived copies of both tables still carry the retired phrases.**
   `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` contains
   `Fedora & co.`, `any distro`, `portable, CLI + GUI` and a `glibc from Ubuntu
   22.04` requirement, because it is Plan 8's design document as written then.
   Out of my file list and, as history, arguably correct as it stands - but a
   sweep that greps the repo for reach claims will keep hitting it, so whether
   archived specs are exempt from the rule is worth one explicit ruling rather
   than a rediscovery every round.
3. **`docs/process-conventions.yaml` still asserts the old pin.** One entry
   reads `release.yml pins ubuntu-22.04 on purpose for the AppImage glibc floor
   (D85)`. That is now false in the same way the rows I fixed were false. It is a
   house-knowledge YAML file, explicitly the controller's to write, so I left it
   untouched and report it instead. `renovate.jsonc`'s neighbouring comment is
   already current (`ubuntu-24.04`), which is what makes the YAML entry stand
   out.
4. **`Fedora 40+` names an end-of-life release.** It is a floor statement and
   true (Fedora 40 is exactly where 2.39 lands, 39 ships 2.38), but a reader may
   read a named version as a recommendation rather than a boundary. The
   requirement clause in front of it is what carries the row; if that reading
   worries the owner, dropping the parenthetical to leave `rpm distributions
   with glibc 2.39+` costs nothing but the reader's handle.
5. **The completion check is not enforced anywhere.** It caught five real
   violations in seconds and would catch the next one, but it lives in a
   scratchpad. Promoting it to a gate part (or folding the two rules into
   `scripts/ledger-lint.py`, which already checks a docs invariant of exactly
   this shape) would convert the rule from something a reviewer must notice into
   something the tree enforces. That is a new gate part and therefore an owner
   decision, including the `BUILDING.md` gate-total sentence it would move.
