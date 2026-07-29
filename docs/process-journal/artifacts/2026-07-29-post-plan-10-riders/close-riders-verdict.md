# Review verdict - the four close riders (`d9a4fa2`)

**Verdict: APPROVED_WITH_MINORS**

All four riders are correctly implemented and no edit to the four committed
files is required. Rider 1's content is provably unchanged, rider 2's two new
routing phrases are true against the distributions' own package data, rider 3
moved one token and left the rule intact and valid, rider 4 removed both figures
while both claims and the register survive. The gate reproduces 11/11 and the
validator is green in both modes.

Every finding below sits **outside the four edited files** or inside a region
the rider's own fence excluded. That is the shape of this change: the commit is
clean, the tree around it carries propagation debt. Three of those sites the
implementer raised; five it did not, and two of those are the sharpest -
`docs/INSTALL.md`'s own AppImage routing, which defeats rider 1 inside rider 1's
own document, and the fact that no decision record anywhere in the house states
the reach change that all of this follows from.

**On citation form.** This verdict cites `<file>:<line>` at the named commit
`d9a4fa2`. The owner's widened comment-citation ruling of 2026-07-29 binds
source, CI and configuration comments; its scope boundary
(`docs/conventions.yaml`, the comment-line-citation entry) exempts process
artifacts doing the citing - "a review verdict ... may cite `<file>:<line>`,
because those record where something was at one moment and the moment is part of
the claim". The review brief requires the form. Both agree; nothing here
contradicts the implementer's own no-citation fence, which governed what the
implementer wrote.

---

## Findings

### 1. MEDIUM - `docs/INSTALL.md:83`: the AppImage still routes to "any distro", above the floor paragraph that contradicts it

The Linux artifact list reads:

```
- `muxsmith-<version>-linux-x86_64.AppImage` - any distro: `chmod +x` the file, then run it
```

This is the identical overclaim rider 2 removed from the release table, surviving
in the document whose floor notice rider 1 just promoted, **fourteen lines
above** that notice. Reading order is the aggravating factor: a reader scanning
the artifact list to pick a download meets the false claim at `:83` and acts on
it; the paragraph that corrects it is at `:97`. Rider 1 moved the floor above the
skip-invitation and thereby fixed the *list-intro* problem, but the *list of
artifacts* higher up still tells the AppImage reader there is no constraint.

Verified rather than assumed: `.github/workflows/release.yml:84-85` builds
`deb,rpm,appimage` on the single `os: ubuntu-24.04` leg, and `:172-177` packs the
tar.gz from that same leg's binaries. All four Linux artifacts carry the 2.39
floor; the AppImage bundles webkitgtk and gtk3, not glibc, which the tarball
README already states at `packaging/linux-tarball-README.txt:24-27`.

The implementer raised this (concern 1) and correctly declined to act: the file
was fenced to rider 1 and the standing rules forbid design latitude. It is
routed here, not graded against the implementer.

**Exact required change** (follow-up rider, `docs/INSTALL.md:83`):

```
- `muxsmith-<version>-linux-x86_64.AppImage` - any distro meeting the glibc floor below: `chmod +x` the file, then run it
```

### 2. MEDIUM - `.github/release/draft-body.md:11` and `docs/INSTALL.md:82`: "Fedora & co." is false for a measurable part of the set it names

Both live texts route the rpm to `Fedora & co.`. Fedora itself clears the floor;
"& co." does not, and this row was left untouched by rider 2's fence while the
two rows beside it were corrected for exactly this class of overclaim.

Measured against the distributions' own repositories:

| distro family | glibc | vs. 2.39 floor | source |
|---|---|---|---|
| Fedora 44 | `2.43-7.fc44` | above | packages.fedoraproject.org |
| Fedora 43 | `2.42-13.fc43` | above | packages.fedoraproject.org |
| Rocky/RHEL/Alma 9 | `glibc-2.34-274.el9_8.x86_64.rpm` | **below** | download.rockylinux.org BaseOS index |
| openSUSE Leap 15.6 | `glibc-2.38-150600.11.1.x86_64` | **below** | download.opensuse.org Leap 15.6 oss |
| openSUSE Leap 16.0 | `glibc-2.40-160000.2.2.x86_64` | above | download.opensuse.org Leap 16.0 oss |

The rpm is the artifact whose "& co." is doing the most work, and it is the one
row in the table that now makes a false positive assertion. The tar.gz row
(finding 6) is merely silent; this one is wrong.

**Exact required change**: give the rpm row a bound consistent with its
siblings, in both texts. `Fedora & co. (glibc 2.39+)` in the draft body; in
`docs/INSTALL.md:82`, `Fedora & co.` becomes `Fedora & co. (see the glibc floor
below)`.

### 3. LOW - `docs/INSTALL.md:81`: the deb row still says "Debian/Ubuntu"

The exact phrase rider 2 replaced at `.github/release/draft-body.md:10`, still
live one document over. Debian 12 (glibc `2.36-9+deb12u14`) and Ubuntu 22.04
(`2.35-0ubuntu3.14`) are below the floor, both measured below. This row matters
more than the release-table row it mirrors, because the deb reader gets no other
warning: see the evidence appendix, where the deb's `Depends` set is verified at
the vendor source to contain no `libc6`.

**Exact required change**: `- ... .deb` - Debian 13+ / Ubuntu 24.04+:` matching
the release table's now-corrected phrasing.

### 4. LOW - `.github/workflows/release.yml:27-28`: the floor's documentation enumeration is short, and so is its ROADMAP twin

```
# 2.39; docs/INSTALL.md and the tar.gz README state it, so moving this
# pin moves the product's reach and those two texts with it.
```

After rider 2, `.github/release/draft-body.md` states the floor as well
(`:12` names `glibc 2.39+` outright, `:10` states a reach derived from it). The
next pin move now has at least three texts to carry and the comment that exists
to say so names two, with the count welded into the sentence.

The implementer raised this (concern 4). What it did not raise, and what makes
this worth a commit rather than a note, is that **the same enumeration exists a
second time** at `docs/ROADMAP.md:704-706`:

```
  the Linux release leg to `ubuntu-24.04` AND record the raised glibc/webkit
  floor in docs/INSTALL.md and the tar.gz README requirement line in the same
  change. (Plan-8 design trigger 1, D85.)
```

That is the prescription the author of `e260845` actually worked from. Fixing
only the workflow comment leaves the trigger re-arming the identical omission on
the next fire. Both move together or neither does. See adjudication 3 for the
recommended phrasing, which is not "two -> three".

### 5. LOW - `docs/ROADMAP.md:711-721`: the fired trigger still says the decision is pending

```
**NOT YET CONSUMED - it is an owner decision, because the trigger's own
prescription moves the product's REACH, not just a pin.**
...
stops testing what current systems actually run. Decision pending.
```

The decision was taken and executed the same day: `e260845` moved the Linux
release leg to `ubuntu-24.04` and updated `docs/INSTALL.md` and the tarball
README, which is precisely this trigger's prescription. The tracker's live
forward-looking section now tells the next session that an owner call is
outstanding on a change that shipped two commits ago.

Nobody raised this. It is controller-owned (`docs/ROADMAP.md` is single-writer),
which is why it is routed rather than graded.

**Exact required change**: mark the trigger FIRED AND CONSUMED with the
consuming commit, in the house's established form (compare the consumed
salvage trigger at `docs/ROADMAP.md:693-702`, whose FIRED AND CONSUMED marker
sits at `:697`), and record the owner's
accepted reach loss - Ubuntu 22.04 LTS and Debian 12 - as the decision it was.

### 6. LOW - `.github/release/draft-body.md:13`: the tar.gz row is silent where its three siblings now speak

See adjudication 2 for the full argument and verdict. Measured: the table has 7
rows, 4 Linux and 3 non-Linux. Rider 2's brief accounts for 6 of the 7 - two
corrected, the rpm row, and "the three non-Linux rows" - and never names the
tar.gz. The implementer flagged the gap (concern 2) instead of exercising
latitude, which is the correct move; the defect is in the brief's enumeration,
not in the implementation.

### 7. LOW - no decision record exists for the reach change

`docs/decision-ledger.yaml` and the Tier-2 nature files carry no entry for the
ubuntu-24.04 move or the 2.39 floor. The house recorded two owner rulings from
this session (`077239f`) - the widened comment-citation scope and the README
growth-prone-figures ruling - but not the one that changed what the product
runs on. Consequences already visible in the tree:

- `renovate.jsonc:51-52` cites `D85: the oldest supported base for the AppImage
  glibc floor` for a base D85's own leg table does not name. The citation
  survives on the natural reading - the parenthetical glosses D85's *principle*
  ("test on the newest, build releases on the oldest supported", stated verbatim
  in D85's section), not its version - so this is **not** a defect in rider 3.
  But it is only sound while nothing supersedes D85, and something has.
- `.github/workflows/release.yml:21` records the 24.04 divergence as a "recorded
  deviation (D85)" on the same borrowed authority.

**Exact required change**: a decision-ledger entry recording the move, its
accepted cost (Ubuntu 22.04 LTS and Debian 12 dropped), the counter-argument on
record (vendor AppImage guidance, and that testing only on the release base
stops testing what current systems run), and its supersession relationship to
D85's linux leg. Controller-owned.

### 8. NIT - `docs/decision-ledger.yaml:5274`: the README ruling's own scope note is now stale

```
Scope as ruled: the verdict-file count. Whether the neighbouring decision-series
figure in the same paragraph follows is a question put to him at the same close
and is NOT decided here.
```

It was decided at that same close - the rider brief records the owner's words,
"ja, auch raus damit, das muss nicht in die readme" - and rider 4 executed it.
Off the reach axis, so outside the dimension-5 sweep proper, but the same
falsified-reference class and nobody raised it. Controller-owned.

### 9. NIT - the report's verification surface is narrower than the claim it carries

The report proves "not one of the four edited files is in any gate part's input
set" with greps over `scripts crates src src-tauri e2e`. That five-directory
enumeration is itself a claim, and it omits the repo root (where
`playwright.config.ts`, `vite.config.ts`, `eslint.config.js` and `package.json`
live, i.e. the configuration of gate parts 7-10), `.github/`, and `packaging/`.

The conclusion survives - I re-ran it tree-wide excluding only `docs/` and the
lockfiles, with a firing control, and separately swept `include_str!` /
`include_bytes!` tree-wide, which is the one mechanism that would pull a
Markdown file into `cargo test`/`cargo doc` invisibly to a basename grep. Both
came back clean against fired controls. The report was right; its stated surface
did not entitle it to be. This is the `proc-sweep-surface-completeness` class the
house already carries at count 8.

### 10. NIT - `renovate.jsonc` version-token claim is imprecise

The report writes that a grep for `22.04|24.04|26.04` "shows the file carries no
other version token". The file does carry another one: `26.04` at `:53`. The
intended and correct claim is that no stale `22.04` survives. Measured: exactly
two version-shaped tokens in the whole file, `24.04` at `:51` and `26.04` at
`:53`, both inside the edited comment block.

### 11. NIT - `README.md:106-108` is a future instance of the corrected defect

```
<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
     docs/INSTALL.md, which already carries the per-OS install-hurdle
```

Not false today - it asserts nothing. But it is a standing instruction to build
a fourth artifact table at the 1.0 tag, in the most-read document in the repo,
and the two tables that already exist both had to be corrected for reach
overclaim. Worth one clause in the placeholder so the third one is born correct.

---

## Dimension 5: the falsified-reference sweep

**Axis, as the brief defines it:** texts asserting (A) the artifacts' reach,
(B) the glibc floor, (C) where the floor is documented.

**Surface.** All 265 tracked files outside `docs/process-journal/` and
`docs/superpowers/` and outside the two lockfiles - the live shipped-or-consumed
surface. Derived, not recalled: `git ls-files` piped through the two path
exclusions and tallied by top-level directory, so the sweep saw `help/` (44
files) and `locales/` (14) as well as the obvious documents.

**Patterns.** Derived from the artifacts, not from memory of what should be in
them: the distro names this repo actually uses in reach text (Ubuntu, Debian,
Fedora), the reach phrasings it actually uses (`any distro`, `any Linux`, `any
current distribution`, `& co.`), the floor terms (`glibc`, `floor`, the version
numerals), and the four artifact names. Where a pattern's result was an absence
I ran a control that fired; where the surface was one short document
(`README.md`'s paragraph) I dumped every distinct token rather than matching a
recalled numeral word list, since an enumerated word set is exactly what a fire
test cannot validate.

### A. Reach assertions

| # | Site | Says | Today |
|---|---|---|---|
| 1 | `.github/release/draft-body.md:10` | `Debian 13+ / Ubuntu 24.04+` | **TRUE** - Debian 13 `2.41-12+deb13u3`, Ubuntu 24.04 `2.39-0ubuntu8.8`, and trixie provides `libwebkit2gtk-4.1-0` (2.52.5), so the deb's declared dependency resolves there |
| 2 | `.github/release/draft-body.md:11` | `Fedora & co.` | **INCOMPLETE/FALSE** - true for Fedora, false for EL9 (2.34) and Leap 15.6 (2.38). Finding 2 |
| 3 | `.github/release/draft-body.md:12` | `any distro with glibc 2.39+` | **TRUE** |
| 4 | `.github/release/draft-body.md:13` | `portable, CLI + GUI` | **INCOMPLETE** - carries the same floor, says nothing. Finding 6, adjudication 2 |
| 5 | `docs/INSTALL.md:81` | `Debian/Ubuntu` | **INCOMPLETE** - the phrase rider 2 just retired. Finding 3 |
| 6 | `docs/INSTALL.md:82` | `Fedora & co.` | **INCOMPLETE/FALSE** - same set as #2. Finding 2 |
| 7 | `docs/INSTALL.md:83` | `any distro` | **FALSE** - Finding 1 |
| 8 | `docs/INSTALL.md:84` | `portable archive with both binaries; see its README.txt` | **TRUE** - asserts form, and routes to the text that does carry the floor |
| 9 | `docs/INSTALL.md:104-113` | `the deb/rpm declare these as hard dependencies` (webkitgtk 4.1, gtk3) | **TRUE** - verified at `tauri-cli-v2.11.4`, which pushes `libwebkit2gtk-4.1-0` and `libgtk-3-0` into `depends_deb` and the matching sonames into `depends_rpm` |
| 10 | `docs/INSTALL.md:17-20` | AppImage carries both binaries, only the GUI directly runnable | **TRUE** - form, not reach |

### B. Floor assertions

| # | Site | Today |
|---|---|---|
| 11 | `docs/INSTALL.md:97-100` (the promoted paragraph) | **TRUE** - 2.39/2.35/2.36 all re-measured independently |
| 12 | `packaging/linux-tarball-README.txt:22-27` | **TRUE** - including "The AppImage ... is built on the same base and does not lift it", corroborated at `release.yml:84-85` |
| 13 | `.github/workflows/release.yml:26-27` "sets the shipped glibc floor at 2.39" | **TRUE** |
| 14 | `renovate.jsonc:51-54` | **TRUE** - `release.yml` pins 24.04 in all three jobs, `ci.yml` runs 26.04 in its matrix and both single-runner jobs. See finding 7 on the D85 pointer |
| 15 | `docs/ROADMAP.md:713-716` (measured floors table) | **TRUE** - all five figures re-measured: Ubuntu 22.04 2.35, 24.04 2.39, Debian 12 2.36, Debian 13 2.41; 26.04's 2.43 not independently checked (no released package index), flagged as the one unverified number on this line |

### C. Where the floor is documented

| # | Site | Today |
|---|---|---|
| 16 | `.github/workflows/release.yml:27-28` "docs/INSTALL.md and the tar.gz README ... those two texts" | **INCOMPLETE** - Finding 4, adjudication 3 |
| 17 | `docs/ROADMAP.md:704-706` "record ... in docs/INSTALL.md and the tar.gz README requirement line" | **INCOMPLETE** - same enumeration, second copy. Finding 4. **Raised by nobody** |

### D. Status assertions about the move itself

| # | Site | Today |
|---|---|---|
| 18 | `docs/ROADMAP.md:711-721` "NOT YET CONSUMED ... Decision pending" | **FALSE** - Finding 5. **Raised by nobody** |
| 19 | `docs/process-conventions.yaml:315` "release.yml pins ubuntu-22.04 on purpose for the AppImage glibc floor (D85)" | **FROZEN-BUT-MISLEADING** - a dated (`2026-07-29`) occurrence `ref`, which the house convention explicitly protects as a record of the moment. Not a defect by that rule. Flagged because it is present-tense prose inside a live, settled Tier-2 entry, so a reader of `ci-04-dependabot-cadence` takes away a pin that no longer exists. Controller's call. **Raised by nobody** |
| 20 | the decision ledger | **MISSING** - no entry records the reach change at all. Finding 7. **Raised by nobody** |

### E. Swept, on-axis assertion absent (the negative half of the sweep)

`README.md` - carries no Linux reach or floor claim anywhere; its Install
section is a placeholder (finding 11). `help/` (44 files) and `locales/` (14) -
zero hits on any axis pattern; the in-app help and the user-facing catalogs make
no platform-reach claim. `docs/product-boundaries.yaml`, `docs/conventions.yaml`,
`docs/IDEAS.md` - zero hits. `docs/process-conventions.yaml:322`
(`ci-13-packaging-deferred`) - describes the release mechanism, the four legs and
the eight artifacts, and makes no reach or floor claim, so it survives the move
untouched. `BUILDING.md:26,32` - build-host dependencies, not artifact reach.
`.github/workflows/ci.yml` - test matrix, not artifact reach.
`src-tauri/tauri.conf.json:60-61` - `recommends` only, no reach claim.

### F. Excluded as dated records, with the reason

`docs/process-journal/artifacts/**`, `docs/superpowers/specs/**`,
`docs/superpowers/plans/**`, `docs/process-journal.md`, and dated occurrence
`ref`s in the house YAML. The house treats these as frozen provenance -
`docs/process-conventions.yaml` names "frozen SDD records, the mandating plan
doc" as deliberate exclusions from tree-wide old-string sweeps, and
`docs/ROADMAP.md:697-702` records keeping a pre-salvage path on purpose "because
they record what the trigger said rather than pointing a reader at a live
artifact". They carry `ubuntu-22.04` and `glibc 2.35` in quantity; none of it is
a defect.

**One call-out from inside that exclusion**, because it is load-bearing for a
live file: D85's definition site is
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:845-855`,
whose leg table names `ubuntu-22.04` for `linux-x86_64` at `:855`. Correct as
history.
It means `renovate.jsonc` and `release.yml` now both cite D85 for a base D85
does not name - which is finding 7's real content, and is fixed by recording the
superseding decision, not by editing the frozen design.

---

## Adjudications

### 1. Where the floor paragraph landed - the chosen placement is RIGHT

**Verdict: the placement before the intro is correct, and the literal reading
would have been wrong.**

Stated in both directions:

*The case for the literal reading* (between the colon-ended intro and the
bullets): the prescription said "immediately BEFORE it", and "it" grammatically
refers to the list. That reading puts the paragraph closest to the text it was
extracted from and needs no judgment call at all. Anyone auditing the rider
against its own words finds an exact match.

*The case for the chosen reading*, which wins: the skip-invitation is the intro's
final clause - "the deb/rpm packages declare mkvtoolnix as a recommended
dependency; the AppImage and tar.gz do not manage dependencies, so install the
runtime requirements yourself:". Inserting the floor after that clause puts it on
the far side of the sentence that tells the deb/rpm reader this material is not
theirs. The finding exists precisely because the floor sat where that reader was
told not to look; the literal reading reproduces the defect one paragraph lower
and additionally severs a colon from its list. The chosen placement puts the
floor in front of the skip-invitation, so position and text now assert the same
thing.

The weight on this went **up** during review, not down. The MEDIUM finding's
load-bearing premise - the deb declares no `libc6`, so `apt install` succeeds
below the floor with no warning - was borrowed by the implementer and is now
verified at the vendor source (evidence appendix E). The document really is the
only channel. Given that, a placement that reaches the deb/rpm reader is not a
stylistic preference; it is the whole point of the rider.

The implementer surfaced the fork rather than resolving it silently (concern 3),
which is the behaviour the standing rules ask for.

**Residual, not a placement defect:** the placement is right relative to the
list, and still leaves finding 1 standing - the artifact list fourteen lines
higher routes the AppImage to "any distro". Fixing that is what completes rider
1's intent; moving the paragraph again is not.

### 2. The tar.gz row - the table DOES now imply the tar.gz is exempt

**Verdict: a real defect, LOW severity. The enumeration is internally
inconsistent and the inconsistency points the wrong way.**

*The case that it is fine*: "portable, CLI + GUI" is a claim about the archive's
form - no installer, both binaries - not about distro reach, and readers do not
parse a form claim as a reach claim. The row is not false; it is silent, and a
routing table is allowed to route on different dimensions per row. The rpm row
routes by distro family, the Windows rows route by OS version, the macOS row
routes by hardware.

*The case that it is not*, which wins: before this change every Linux row routed
by reach, and the reader could take the column as answering one question. After
it, three of four Linux rows answer "which systems run this" and one answers
"what shape is it", with no marker that the question changed. Silence in a
column whose neighbours all carry a bound reads as absence of a bound. Worse, the
one word in that cell - "portable" - is the table's closest synonym for "runs
anywhere", sitting in the only row with nothing to contradict it; "any Linux
distro" was corrected in the row above for asserting exactly that. And the
tar.gz reader is the most exposed of the four: no package manager will even
attempt a dependency check on an archive, so nothing between the download and
the failed `./muxsmith-gui` can warn them.

The verification confirms the exposure rather than softening it: `release.yml`
packs the tar.gz from the same `ubuntu-24.04` leg's `target/release` binaries, so
it carries the identical floor - it is not a different build with different
properties.

**Required change**, respecting rider 2's fence (short phrase, no new row, no new
column, floor not restated everywhere): `| ... .tar.gz | portable, CLI + GUI,
same floor |`. That routes on the same dimension as its siblings without a
fourth statement of the number.

**Grading note:** this is a defect in the rider brief, whose untouched
enumeration - the rpm row plus "the three non-Linux rows" - accounts for 6 of 7
rows. Measured: `awk` over the table gives 7 artifact rows, 4 Linux, 3 non-Linux.
The implementer measured the same gap and flagged it rather than filling it,
which is correct under a no-latitude fence.

### 3. `release.yml`'s policy comment - a FACTUAL DEFECT worth a follow-up commit, and the fix is not "two -> three"

**Verdict: worth a commit, on evidence that the enumeration is consumed rather
than decorative - and the same commit must carry its ROADMAP twin.**

*The case for bookkeeping prose nobody consumes*: it is a comment in a workflow
file, read by whoever is already editing the pin, who can see the tree in front
of them. No tool reads it, no gate checks it, and an off-by-one in a prose count
inside a comment has never broken anything.

*The case that it is consumed*, which wins, and I can name the consumer: the
comment is the in-file half of a maintenance contract whose other half is the
ROADMAP trigger at `docs/ROADMAP.md:704-706`, carrying the **same two-text
enumeration** - "record the raised glibc/webkit floor in docs/INSTALL.md and the
tar.gz README requirement line in the same change". That trigger fired eight days
after it was written and is what the author of `e260845` worked from; `e260845`
touched exactly `release.yml`, `docs/INSTALL.md` and
`packaging/linux-tarball-README.txt`. The enumeration was executed, precisely,
today. It is the single most demonstrably consumed sentence in this whole review.

The next fire will do the same thing and leave the release draft body - the first
text a downloader reads - asserting a stale floor and a stale distro reach on
the release page itself, which is a strictly worse outcome than today's, because
today's `INSTALL.md` and tarball README at least got updated.

**Recommended change, and it is not incrementing the count.** This house ruled
today, on the owner's own words, that a figure the project's own process keeps
moving loses the number rather than gaining a maintenance duty
(`readme-growth-prone-figures-lose-the-number`), and it carries a live
non-decision, `a-count-a-close-action-moves-needs-growth-proof-phrasing-or-a-standing-duty`,
about exactly this repair-and-re-arm cycle. "Two" becoming "three" re-arms on the
fourth text. Phrase it so growth cannot falsify it, in both places:

- `release.yml`: `# 2.39; the shipped texts that state the floor or a reach
  derived from it move with this pin - grep the tree for the floor before
  changing it.`
- `docs/ROADMAP.md`: same substitution in the trigger's prescription, so the
  trigger prescribes a sweep instead of a list.

Fixing one and not the other is the defect this adjudication is about,
one level up.

---

## Evidence appendix

Instruments under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/ridersrev-independent/`.
No instrument written by the implementer was re-run; every check below is mine.

### A. Tree identity per file, against blobs

```
$ git ls-tree -r d9a4fa2 --name-only | wc -l
1400
$ paste <(git ls-tree -r --name-only d9a4fa2) <(git ls-tree -r --name-only d9a4fa2 -z | xargs -0 git hash-object) \
  | while IFS=$'\t' read -r f h; do t=$(git rev-parse "d9a4fa2:$f"); [ "$t" != "$h" ] && echo "MISMATCH $f"; done | wc -l
0
```

Per-file blob comparison, not a `git status` read: each of the 1400 tracked
paths hashed from the working tree and compared to the blob the commit records.
Run at review start and again at review end; 0 mismatches both times. Nothing
was written into the repository except this verdict file, which
`.gitignore:2` excludes (`git check-ignore -v` -> `.gitignore:2:.superpowers/`,
`git ls-files .superpowers` -> 0).

### B. The gate, re-run, foreground, in BUILDING.md's order

Derived from `BUILDING.md`'s three marked gate blocks, not from the report's
table. 11/11, all exit 0:

| # | Part | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| 3 | `cargo test --workspace` | 0 - 39 suites, 505 passed, 0 failed, 0 ignored |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 |
| 5 | `cargo deny check` | 0 - `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy ... --target x86_64-pc-windows-msvc -- -D warnings` | 0 |
| 7 | `pnpm lint` | 0 |
| 8 | `pnpm build` | 0 |
| 9 | `pnpm check:i18n` | 0 - `41 source files scanned, 212 catalog ids` |
| 10 | `pnpm test:e2e` | 0 - `68 passed (2.9s)` |
| 11 | `python3 scripts/ledger-lint.py` | 0 - `546 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

Every figure reproduces the report's exactly. `git status --short` empty after
the run.

### C. Checking the report's gate-irrelevance measurement, not its conclusion

Tree-wide (all tracked files except `docs/`, the lockfiles and `.superpowers/`),
which is wider than the report's five directories:

```
$ git grep -rnIiE 'draft-body|renovate\.jsonc|INSTALL\.md|README\.md' -- . ':!docs/**' ':!pnpm-lock.yaml' ':!Cargo.lock' ':!.superpowers/**'
```

7 hits: the draft body's own INSTALL links, `release.yml:27` (the policy
comment) and `:220` (`sed __VERSION__ ... draft-body.md`, CI-only),
`README.md:108` (its own placeholder), two comments in
`crates/muxsmith-cli/tests/run_live.rs`, and the tarball README's INSTALL link.
Nothing reads any of the four at gate time.

**Control** (the result above is a presence, but the *conclusion* is an absence):
the identical pattern shape against a token known to be read at build time,
`cli\.ftl`, fires immediately on `crates/muxsmith-cli/src/i18n.rs:8`.

**Second instrument, aimed at the one mechanism a basename grep cannot see** - a
Markdown file compiled into the binary:

```
$ git grep -rnI 'include_str!\|include_bytes!' -- .
```

24 matched lines outside `docs/`, all `locales/*.ftl` and test fixtures; no
`.md`, no `.jsonc`. The control is the hit list itself - the pattern
demonstrably fires.

### D. Rider-by-rider

**Rider 1, content identity.** Own instrument (`floor_identity.py`), anchored by
content on both sides, stripping only the list marker, the two-space
continuation indent and line wrapping:

```
blocks found  old(bullet form)=1  new(paragraph form)=1
normalised lengths  old=265  new=265
VERDICT: IDENTICAL
CONTROL (one digit changed in NEW): DIFFERENT -> comparator is live
CONTROL (bullet form surviving in the new file): 0 (must be 0); same pattern against the old file: 1 (must be 1)
```

Three guards against a vacuous pass: both sides asserted non-empty (265 chars),
the comparator proven able to report DIFFERENT, and the bullet-form pattern
proven to fire against the pre-edit file while returning 0 against the post-edit
one - so "it is no longer a list item" is measured, not assumed.

**Rider 2, table integrity.**

```
$ diff <(git show d9a4fa2^:.github/release/draft-body.md | grep '^| `muxsmith') <(grep '^| `muxsmith' .github/release/draft-body.md)
4c4  deb    | Debian/Ubuntu       -> Debian 13+ / Ubuntu 24.04+
6c6  AppImage | any Linux distro  -> any distro with glibc 2.39+
```

Exactly two rows changed; the other five byte-identical. Row accounting by `awk`
over the table: 7 artifact rows, 4 Linux, 3 non-Linux.

Distro package data for the two new phrases (each from the distribution's own
index): Debian 13 `libc6 (2.41-12+deb13u3)`, source `glibc_2.41.orig.tar.xz`;
Ubuntu 24.04 `libc6 2.39-0ubuntu8.8`, source `glibc_2.39.orig.tar.xz`; Debian 12
`libc6 (2.36-9+deb12u14)`; Ubuntu 22.04 `libc6 2.35-0ubuntu3.14`. Debian 13 also
provides `libwebkit2gtk-4.1-0` at 2.52.5, so the deb's declared dependency
resolves there - the row's claim is not merely glibc-true but installable-true.
Rocky 9 BaseOS `glibc-2.34-274.el9_8.x86_64.rpm`, openSUSE Leap 15.6
`glibc-2.38-150600.11.1`, Leap 16.0 `glibc-2.40-160000.2.2`, Fedora 43
`2.42-13.fc43`, Fedora 44 `2.43-7.fc44`.

**Rider 3, validator both modes with my own fired control.**

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator renovate.jsonc
 INFO: Config validated successfully against 1 file(s)          EXIT=0
$ npx ... --strict renovate.jsonc
 INFO: Config validated successfully against 1 file(s)          EXIT=0
```

Control, deliberately different from the implementer's (which mutated
`enabled`): a scratchpad copy with a bogus top-level key and a bogus
`matchManagers` value.

```
ERROR: Found errors in configuration
  "message": "Invalid configuration option: reviewerXyzBogus"     EXIT=1   (plain)
  "message": "Invalid configuration option: reviewerXyzBogus"     EXIT=1   (--strict)
```

Both modes read the handed file and reject it, so both green results are
meaningful. Style, measured rather than eyeballed: `packageRules` holds 7 rule
objects, 5 of which carry a comment (`:32` cargo, `:50` the edited
github-runner, `:59` packageManager, `:70` mise, `:76` rust-toolchain). The
edited comment keeps its leading position inside the rule object and its `// `
form at the same indent as its four commented siblings. Keys and values
(`matchManagers`, `matchDepTypes`, `enabled`) byte-identical.
Version tokens in the file: exactly `24.04:51` and `26.04:53`, no stale `22.04`.

**Rider 4, README.** Paragraph located by its own opening words, then the
surface-complete method for a 79-word block - every distinct token printed,
because an enumerated numeral-word list is precisely the claim a fire test
cannot validate:

```
digits in NEW: NONE
digits in OLD (control): ['103', '105', '225']
EVERY DISTINCT TOKEN: a agents alternatives and apart asking at briefs carries code
context controller decision design docs/](docs/ every fresh ground human hurt
implementation implementer in including independent is it its journal matters
merges not numbered ones orchestrates plan plans preserved process public
rationale recorded rejected repo review reviewer session setup spec tear that
the their this truth verdicts whole with write
```

No numeral in any form, digit or word. Both claims survive verbatim - "Every
design decision is numbered and recorded with its rationale and rejected
alternatives" and "the preserved review verdicts, including the ones that hurt".
Whole-file scan of every numeral-bearing line in `README.md`: the survivors are
product facts (`mkvmerge >= 86.0`, 14-day log retention, exit codes 0/1/2,
`--jobs 8`, `profile_version: 1`), none of which the project's own process moves,
so the ruling's class has no second instance in the file.

### E. The MEDIUM finding's borrowed premise, verified at the vendor source

The brief asserts the shipped deb declares `Depends: libwebkit2gtk-4.1-0,
libgtk-3-0` and no `libc6`. The implementer marked it borrowed and unverifiable
in-tree. It is verifiable, at the exact pinned CLI version - `package.json:30`
pins `"@tauri-apps/cli": "2.11.4"`, so the tag is `tauri-cli-v2.11.4`:

- `crates/tauri-bundler/src/bundle/linux/debian.rs` writes the control file's
  `Depends` line **only** from config:
  `let dependencies = settings.deb().depends.as_ref().cloned().unwrap_or_default();`
  No `webkit`, `gtk` or `libc` literal in the file (control: `writeln` appears 18
  times, so the grep surface is live).
- `crates/tauri-cli/src/interface/rust.rs` supplies them:
  `depends_deb.push("libwebkit2gtk-4.1-0".to_string());`
  `depends_deb.push("libgtk-3-0".to_string());`
  and pushes the matching sonames into `depends_rpm`.
- Every `libc`/`glibc` mention in that file: **none** (control: `depends_deb`
  appears 7 times).
- `src-tauri/tauri.conf.json:60-61` authors `"recommends": ["mkvtoolnix"]` for
  deb and rpm and **no** `depends` key.

So the deb declares exactly `libwebkit2gtk-4.1-0, libgtk-3-0` (plus a
conditional tray indicator) and `Recommends: mkvtoolnix`, and nothing constrains
glibc. `apt install` succeeds on Debian 12 and the binary fails at runtime with
nothing having warned the user. The premise is confirmed and promoted from
borrowed to verified; it also confirms `docs/INSTALL.md:112`'s "the deb/rpm
declare these as hard dependencies", which is true and was worth checking rather
than assuming.

### F. All four Linux artifacts share one build leg

`.github/workflows/release.yml:84-85` - `leg: linux-x86_64`, `os: ubuntu-24.04`,
`bundles: deb,rpm,appimage`; `:172-177` packs the tar.gz from that leg's
`target/release/muxsmith` and `muxsmith-gui`. There is no second Linux base, so
the floor is uniform across deb, rpm, AppImage and tar.gz. This is what makes
findings 1, 2, 3 and 6 defects rather than open questions.

---

## HARVEST

1. **A rider that corrects a claim in one text should be scoped by the CLAIM,
   not by the FILE.** Rider 2 corrected "any Linux distro" in the release table
   while the identical phrase sat in `docs/INSTALL.md` - the file rider 1 was
   editing in the same commit. Two riders touched the two texts, and the
   overclaim survived in the one that was open at the time. The readable
   trigger: a rider's brief quotes a phrase it is retiring. The handle: grep the
   live surface for that phrase before writing the file list, and either fence
   the hits explicitly with a reason or put them in scope.

2. **A two-item enumeration written into a maintenance contract is a count, and
   counts in this repo re-arm.** `release.yml` and the ROADMAP trigger both said
   "these two texts"; the trigger fired, was executed exactly, and produced a
   third text - so the contract that fired correctly today is wrong for
   tomorrow. The house already ruled the general form (`readme-growth-prone-
   figures-lose-the-number`) for the README's sell-tone. The same reasoning
   applies harder to a maintenance instruction, where the number is not
   evidence for a reader but a work list for an implementer.

3. **The same enumeration existing in two places is not redundancy; it is two
   sites to visit.** The trigger's prescription and the workflow comment carry
   the same list for a reason - one is read when planning the move, one when
   making it. Repairing either alone leaves the other to re-arm. Recognising the
   pair is cheap (`grep` for the enumeration's distinctive phrase); noticing
   after the next fire is not.

4. **A frozen historical record is not a defect, but a live pointer INTO one
   can be.** D85's design section correctly still names `ubuntu-22.04`. The
   problem is that two live files now cite D85 for a base it does not name,
   because the superseding decision was executed without being recorded. The
   general shape: when a decision is superseded by action rather than by a new
   decision entry, every live citation of the old decision silently becomes a
   claim its source does not support.

5. **"Fedora & co." is the reach claim that a version-based correction pass
   misses**, because it names no version and so matches no version-shaped
   pattern. Both texts kept it while their neighbours were corrected. The
   generalisation for sweeps on this axis: an open-ended set token (`& co.`,
   `and friends`, `etc.`, `any`) is a reach assertion with the bound removed,
   and it needs its own pattern - the numeral sweep cannot see it.

6. **A negative claim about vendor behaviour is measurable more often than it
   looks.** "I have no release artifact here" was true, but the deb's `Depends`
   set is fully determined by two files in the vendor's source at the tag the
   repo pins, both fetchable in one command. Reaching for the pin (`package.json`
   -> `tauri-cli-v2.11.4`) turned a borrowed premise into a verified one and
   incidentally confirmed a second live claim in `docs/INSTALL.md`.

---

# DELTA REVIEW - commit `c38bb0b` (`docs: every artifact row states the reach it actually has`)

**Delta verdict: APPROVED_WITH_MINORS**

The change is correct and is a real improvement on every routed site. Diff
derived independently: three files, 7 insertions / 7 deletions, and every
changed line is one of the sites routed to it. All seven distribution figures
the implementer reports are confirmed against the distributions' own
repositories, and the boundary it picked is exact rather than approximate.

Three things do not survive the check: one new claim is incomplete about the
world (`RHEL 10+`), the growth-proof phrasing is growth-proof but covers a
narrower set than the pin actually drags, and one violation of the rule remains
in the tree - at a site my own sweep table told the implementer was fine.

## What the delta was checked against

| Question asked | Answer |
|---|---|
| Is every routed site now true? | Yes, with one qualifier - finding D1 |
| Do the two tables give ONE answer per artifact? | 3 of 4. tar.gz diverges - finding D3 |
| Is the `release.yml` phrasing really unfalsifiable by a fourth text? | Yes as to growth; no as to coverage - finding D2 |
| Is the completion check sound? | Reproduced independently: 6 pre, 1 residual |
| Anything still unaddressed and unrouted? | Two things - see reconciliation |

## Delta findings

### D1. LOW - `RHEL 10+` clears the glibc floor but cannot resolve the rpm's webkit dependency

Both edited tables now route the rpm to `rpm distributions with glibc 2.39+
(Fedora 40+, RHEL 10+)`, and in `docs/INSTALL.md:82` that phrase is followed on
the same line by `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`.

The glibc half is right. The installability half is not, and I held the deb row
to exactly this standard in the original review (where I checked that Debian 13
actually provides `libwebkit2gtk-4.1-0` before calling `Debian 13+` true), so
the rpm row owes the same check.

Measured, each from the distribution's own repository index, with the surface
confirmed by HTTP status and byte count rather than assumed:

| Repo | webkit packages under `/w/` |
|---|---|
| Rocky 10 AppStream (HTTP 200, 9128 B) | none |
| Rocky 10 BaseOS (HTTP 200, 3617 B) | none |
| Rocky 10 CRB (HTTP 200, 3965 B) | none |
| Rocky 9 AppStream (HTTP 200, 17032 B) | `webkit2gtk3`, `-devel`, `-jsc`, `-jsc-devel` |
| EPEL 10 (HTTP 200, 12057 B) | **`webkit2gtk4.1`**, `-devel`, `-doc`, `webkitgtk6.0`, ... |
| Fedora 40 Everything (HTTP 200, 98430 B) | `webkit2gtk4.0`, **`webkit2gtk4.1`**, `webkitgtk6.0`, ... |

The Rocky 9 and Fedora 40 rows are the fired controls: the same pattern against
the same kind of index returns packages, so the three empty EL10 results are
findings and not a broken instrument.

That the rpm hard-requires the soname is verified at the pinned vendor tag, not
assumed: `crates/tauri-cli/src/interface/rust.rs` pushes
`libwebkit2gtk-4.1.so.0` into `depends_rpm` with the `()(64bit)` suffix, and
`crates/tauri-bundler/src/bundle/linux/rpm.rs` turns each into a hard
requirement - `builder = builder.requires(Dependency::any(dep));`.

So on a stock RHEL 10 or Rocky 10 with only the default repositories,
`dnf install` refuses: nothing provides `libwebkit2gtk-4.1.so.0()(64bit)`.
It resolves only with EPEL 10 enabled. `Fedora 40+` has no such problem - the
base repo ships `webkit2gtk4.1`, so the two named families are not symmetric
even though the row presents them as equivalent.

Graded LOW rather than MEDIUM deliberately: the failure is loud. `dnf` refuses
and names the missing dependency, unlike the deb/glibc case that motivated this
whole series, where `apt install` succeeded and the binary died later. The row
is also a large improvement on `Fedora & co.`, which named no requirement at all
and swept in EL9.

**Exact required change**, both files: `rpm distributions with glibc 2.39+
(Fedora 40+; RHEL 10+ with EPEL for webkitgtk 4.1)`.

### D2. LOW - the growth-proof phrasing is growth-proof, and covers less than the pin drags

New text at `.github/workflows/release.yml:26-28`:

```
# tests run on the newest. That base sets the shipped glibc floor at
# 2.39, so moving this pin moves the product's reach - and with it every
# user-facing text that states that floor or an artifact's reach.
```

**On the question asked: yes, it is growth-proof.** "every user-facing text that
states that floor or an artifact's reach" is a membership predicate, not an
enumeration. A fourth, fifth or tenth such text joins the category without
falsifying the sentence, which is exactly what the old "those two texts" could
not do. Adjudication 3 is satisfied on its own terms.

**But the category is narrower than the set the pin actually moves**, and this
is measurable rather than hypothetical. The union of tracked files the pin move
dragged across its three commits (`e260845`, `d9a4fa2`, `c38bb0b`), excluding
`README.md` which moved for the unrelated figures ruling:

| Text | Covered by the new category? |
|---|---|
| `docs/INSTALL.md` | yes - user-facing, states floor and reach |
| `.github/release/draft-body.md` | yes - user-facing, states reach |
| `packaging/linux-tarball-README.txt` | yes - user-facing, states floor |
| `.github/workflows/release.yml` | the comment itself |
| `renovate.jsonc` | **no** |

`renovate.jsonc:51-53` states neither the floor nor a reach. It states the
**pin**: "release.yml pins ubuntu-24.04 on purpose (D85: the oldest supported
base for the AppImage glibc floor) while the test matrix runs ubuntu-26.04."
It is a config comment, not user-facing, so it falls outside the category on
both of the predicate's clauses - and it is the text that went stale within the
hour on the last pin move, which is why rider 3 of the previous change existed
at all. The one demonstrated member of the drag set is the one the new sentence
does not reach.

Second, smaller point: my adjudication-3 recommendation paired the category with
a handle ("grep the tree for the floor before changing it"). The implementer
took the category and left the handle. The old sentence was falsifiable but told
a maintainer which two files to open; the new one is unfalsifiable but tells
them only what kind of file to look for. Net still positive, but the actionable
half is now owed by the ROADMAP twin, which is controller-owned and unwritten.

**Exact required change**: extend the predicate past user-facing text, e.g.
`- and with it every text that states that floor, an artifact's reach, or this
pin.` The third clause is what pulls `renovate.jsonc` in.

### D3. LOW - `docs/INSTALL.md:84`, the tar.gz row, is the rule's one surviving violation - and my sweep table is why it was not on the list

The row is unchanged: `portable archive with both binaries; see its
`README.txt``. Its three siblings in the same list now all state the bound
inline, and its counterpart row in the other table was rewritten to
`any distro with glibc 2.39+, CLI + GUI`.

My own instrument, encoding the rule as stated, flags it - on the word
`portable`, which is precisely the word the draft-body row lost. It is also the
only artifact of four that now gets two different answers across the two texts.

**The site was not on the implementer's list because my sweep table marked it
TRUE.** Original row 8: "asserts form, and routes to the text that does carry
the floor". That marking is inconsistent with my own adjudication 2 in the same
document, where I argued that "portable" is "the table's closest synonym for
'runs anywhere'" and used exactly that to require the draft-body row's change.
I applied the argument to one table and not to its sibling. The implementer's
concern 1 is them catching my error, and it is well founded.

I record this rather than quietly reclassifying: an instrument I wrote to test
the implementer refuted a call I had made, which is the only reason it surfaced.

**Exact required change**: `- `muxsmith-<version>-linux-x86_64.tar.gz` - any
distro with glibc 2.39+; archive with both binaries, see its `README.txt``.

## Rulings on the four concerns

### Concern 1 - `docs/INSTALL.md`'s tar.gz row was outside the site list: SUSTAINED, fix it

*Against fixing*: the row defers explicitly to `README.txt`, which does carry the
floor, and `docs/INSTALL.md`'s own floor paragraph sits thirteen lines below it,
so no reader of that document is left without the requirement. The site list was
the site list and the implementer had no latitude.

*For fixing*, which wins: the rule as written says a form row must "say nothing
that reads as reach", and "portable archive" reads as reach - that is not a
reinterpretation, it is the argument that justified changing the sibling row.
Leaving it produces the fourth iteration of the exact shape this rule was
written to end: a change repairs one text and leaves another asserting the old
frame. The cross-table measurement is the cleanest statement of the defect -
three artifacts get one answer, tar.gz gets two.

The implementer behaved correctly: flagged, did not act.

### Concern 2 - the archived Plan-8 design spec still carries all three retired phrases: NO ACTION

*For editing it*: it is the definition site of D85, two live files cite D85, and
a reader following that pointer lands on `ubuntu-22.04` and "any distro".

*For leaving it*, which wins: it is a dated, frozen SDD record. The house names
"frozen SDD records, the mandating plan doc" as deliberate exclusions from
tree-wide old-string sweeps, and `docs/ROADMAP.md:693-702` records keeping a
pre-salvage path on purpose for the same reason. Editing it would falsify the
record of what Plan 8 actually decided, which is the one thing the archive is
for.

The residue is not in the spec; it is that nothing supersedes it. That is
original finding 7 (no decision record for the reach change), controller-owned
and acknowledged as unwritten. Concern 2 resolves to no-action **conditionally**:
if finding 7 is never written, the frozen spec remains the only statement of a
superseded decision that two live files still cite, and the problem returns
wearing a different hat.

### Concern 3 - `Fedora 40+` names an end-of-life release: OVERRULED, keep it

*For changing it*: F40 is EOL - current Fedora is 43/44, measured - so the
boundary names a release nobody runs, and a reader may read the doc as stale.

*For keeping it*, which wins decisively: this is a **floor**, not a
recommendation, and the floor is exactly right. Measured: Fedora 39 ships
`glibc-2.38-7.fc39` (below), Fedora 40 ships `glibc-2.39-6.fc40` (at the floor).
40 is the precise first qualifying release. Naming a currently-supported release
instead would be actively wrong - it would exclude qualifying systems - and
would need updating every six months, re-arming the maintenance treadmill this
entire rule exists to end. A floor stated at its true boundary is stable
forever; a floor stated at "whatever is current" is a chore.

The EOL status of the boundary release is irrelevant to the claim being made.
Close the concern.

### Concern 4 - the completion check is enforced nowhere: CORRECTLY ROUTED as an owner call; my recommendation is to defer

*For promoting into `ledger-lint.py`*: the rule now has a mechanical check, the
script is already the house docs-invariant checker and already gate part 11, and
this defect class has cost four commits in one session. A gate is the only thing
that fires without someone remembering.

*Against, which is my recommendation*: the check parses English prose, and this
repo has a named entry for how that fails - `proc-check-green-state-reachable`,
where a pattern that over-matches makes the green state unreachable. It would
need its own green-reachability proof, and it would freeze the wording of two
user-facing documents against ordinary editing. The decisive objection is that
the boundary it would encode is **contested right now**: my own instrument's
recorded DISAGREEMENT probe is the demonstration - whether "portable archive ...
see its README.txt" reads as reach is exactly the question concern 1 turns on.
Promoting a check that hard-codes one reading of an unsettled boundary buys
enforcement at the price of deciding the question by implementation.

**Recommendation**: keep it as a one-shot sweep instrument, which is what it was
good for here - two independently written versions both found the same residual.
Revisit promotion only after the "reads as reach" boundary is settled by a
ruling, and then only with a green-reachability proof. The implementer was right
that this is not theirs to decide.

## Reconciliation against the original sweep table

- **Fixed by `c38bb0b`**: rows 2, 4, 5, 6, 7 (findings 1, 2, 3, 6) and row 16
  (adjudication 3). Verified individually; all now state a requirement.
- **Still true, unaffected**: rows 1, 3, 9, 10, 11, 12, 13, 14, 15. Re-swept on
  the new tree; the change introduced no new site on the axis.
- **Controller-owned, still false by design** (the coordinator holds these):
  rows 17, 18, 19, 20 - finding 4's ROADMAP twin, the pending-decision entry,
  the `process-conventions.yaml` occurrence, and the missing ledger entry.
- **Row 8 was mismarked by me** and is finding D3.

**Unaddressed and unrouted, answering the question directly - two items:**

1. `docs/INSTALL.md:84`, the tar.gz row (finding D3). Not on any list because my
   table said it was fine.
2. `README.md:106-108`, original NIT 11 - the `placeholder(1.0)` comment
   mandating a fourth artifact table at the 1.0 tag. It appears in neither the
   routed nor the not-routed set. It asserts nothing today, so it is not false;
   it is a standing instruction to rebuild, in the most-read file in the repo,
   the exact surface that has now been corrected twice. One clause in the
   placeholder ("rows state the glibc requirement, not a distro family") would
   make the third table born correct.

## Delta evidence appendix

**Independent completion instrument** (`reach_check.py`, mine, not the
implementer's script). Extracts the four Linux artifact cells from both files by
per-file regex, joins `docs/INSTALL.md`'s wrapped tar.gz continuation line,
parses both revisions with the same code, prints every extracted cell so the
verdict is auditable against raw text, and derives the distro-family and
reach-word vocabulary from the union of the cells themselves rather than from
recall:

```
vocabulary derived from the cells themselves:
  distro families seen : ['Debian', 'Fedora', 'RHEL', 'Ubuntu']
  reach words seen     : ['any', 'portable']

SUMMARY  pre(d9a4fa2)=6  post(c38bb0b)=1
```

Pre-state violations, 6, matching the implementer's reported figure by
independent construction: draft-body rpm (bare family), draft-body tar.gz
(unbounded `portable`), INSTALL deb (bare family), INSTALL rpm (bare family),
INSTALL AppImage (unbounded `any`), INSTALL tar.gz (unbounded `portable`).
Post-state: 1, the INSTALL tar.gz row.

Controls, all 7 passing - four known-bad cells flagged (`Fedora & co.`,
`Debian/Ubuntu`, `any distro`, `portable, CLI + GUI`), three known-good cells
passed (`any distro with glibc 2.39+`, `Debian 13+ / Ubuntu 24.04+`, the new rpm
phrasing). Plus the recorded DISAGREEMENT probe described in finding D3, left
untuned on purpose.

**One answer per artifact** (post state): deb SAME, rpm SAME, AppImage SAME,
tar.gz DIVERGES. 3 of 4.

**World-claims, all seven confirmed independently** and all agreeing with the
implementer's report: Ubuntu 24.04 `2.39-0ubuntu8.8`; Debian 13
`2.41-12+deb13u3`; Fedora 40 `glibc-2.39-6.fc40`; Fedora 39 `glibc-2.38-7.fc39`
(so the `40+` boundary is exact); EL10 `glibc-2.39-128.el10_2`; EL9
`glibc-2.34-274.el9_8`; openSUSE Leap 15.6 `glibc-2.38-150600.11.1`.

**Gate.** Not re-run in full: the three edited files were established in the
original review to sit outside every gate part's input set, and this change
touches no new file. The one part that reads documents was run -
`python3 scripts/ledger-lint.py` -> exit 0,
`546 entries across 4 files plus BUILDING.md's gate enumeration, all invariants
hold`, unchanged from the pre-delta run.

**Tree identity.** 1400 tracked files hashed from the working tree and compared
per file to `c38bb0b`'s blobs: 0 mismatches at delta start and 0 at delta end,
with a fired control. `HEAD` at `c38bb0b`, `git status --short` empty. The only
file written is this verdict, excluded by `.gitignore:2`.

## Delta HARVEST

1. **A reviewer's own sweep table becomes a fence when it is handed on as
   complete.** Row 8's TRUE marking is why the tar.gz row was not on the site
   list, and it contradicted adjudication 2 in the same document. A table that
   will be consumed as an enumeration should be checked against the document's
   own findings before it ships - the internal contradiction was visible without
   any new measurement.
2. **Growth-proofing a count answers falsifiability, not coverage.** The new
   comment cannot be falsified by a fourth text and still misses the one text
   that empirically moved. When replacing an enumeration with a category, test
   the category against the set the enumeration was standing in for - here, the
   file list of the commits that already happened.
3. **A version-bounded distro claim carries a second, silent claim: that the
   package's other dependencies resolve there.** `RHEL 10+` is true about glibc
   and false about webkitgtk, and only the glibc half was checked because glibc
   was the fact that moved. When a row names a distro as qualifying, every hard
   dependency the artifact declares is part of that claim.
4. **A floor should name its true boundary even when that release is EOL.**
   Naming the currently-supported release instead is both wrong and
   self-re-arming. EOL-ness is a property of the release, not of the boundary.
