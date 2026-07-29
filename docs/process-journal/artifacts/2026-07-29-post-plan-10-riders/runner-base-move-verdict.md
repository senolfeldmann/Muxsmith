# Verdict - the Linux release base move to ubuntu-24.04 (`e260845`)

**Reviewer:** independent, did not author the change. **Date:** 2026-07-29.
**Tree reviewed:** `e260845`, read from the files. **Verdict:**

## APPROVED_WITH_MINORS

The change is correct and complete within its file list. All three pins moved
and only those; the workflow's parsed structure is otherwise byte-for-byte the
same shape; the policy comment is true after the edit; both user-facing texts
name the same floor and **every factual claim in them verifies against the
distributions' own package databases and against Tauri's own guidance**. No
finding blocks or requires a revert.

One MEDIUM is worth fixing before 1.0 and it is a one-sentence move, not a
rewrite. Everything else is a controller follow-up outside the implementer's
exhaustive file list, plus one open item of the implementer's that I closed by
measurement rather than leaving on recall.

---

## Findings

### 1. MEDIUM - `docs/INSTALL.md:109-113`: the floor is stated where half its audience is told not to read

The new bullet is the only glibc notice in the document, and it sits inside a
list whose intro at `:99-101` scopes itself away from deb/rpm readers:

> the deb/rpm packages declare **mkvtoolnix** as a recommended dependency; the
> AppImage and tar.gz do not manage dependencies, so install the runtime
> requirements yourself:

A deb/rpm user is explicitly invited to skip that list. The bullet repairs the
scope inside its own text ("every Linux artifact here, the AppImage included"),
which only helps a reader who already read it.

**Why this matters rather than being a style quibble - measured, not assumed.**
I checked whether the package manager itself would carry the message instead.
It does not. `src-tauri/tauri.conf.json` declares only
`linux.deb = {"section": "video", "recommends": ["mkvtoolnix"]}` - no `depends`
- and the control file actually produced by a real release rehearsal, captured
at `docs/process-journal/artifacts/plan-8-sdd/task-6-report.md:926`, reads:

```
 Depends: libwebkit2gtk-4.1-0, libgtk-3-0
```

No `libc6`. So `apt install ./muxsmith-*.deb` succeeds on Debian 12, and the
binary then fails at runtime with a `GLIBC_2.39 not found` message the user has
no documented path to. The doc is the only channel, and it is placed behind a
door marked "not for you".

Boundary, stated fairly: this is an **incomplete improvement, not a
regression**. Before this change `docs/INSTALL.md` stated no floor at all, so
the change strictly improves the file.

Caveat on the measurement: the captured control file is from a 22.04-built deb.
Tauri's depends generation lists the GUI libraries and is not base-dependent in
kind, so the conclusion is expected to carry to 24.04, but that is a reasoned
expectation and the next rehearsal is what confirms it.

**Exact required change** (this is adjudication 2's minimal fix): delete the
bullet at `:109-113` and place the same content as a standalone sentence
immediately before the requirements list, so it is not gated by that list's
intro:

```
All Linux artifacts require **glibc 2.39 or newer** - not a package you
install but what your distribution ships. Every artifact here, the AppImage
included, is built on Ubuntu 24.04 (glibc 2.39), so systems below that floor
- Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot run them.
```

Same text, same figures, moved out of a scope that excludes half its audience.

### 2. LOW - `.github/release/draft-body.md:10,12`: the release page still routes "any Linux distro"

The release body is the first text a downloader reads. Its artifact table says:

- `:10` `muxsmith-__VERSION__-linux-x86_64.deb` | **Debian/Ubuntu**
- `:12` `muxsmith-__VERSION__-linux-x86_64.AppImage` | **any Linux distro**

"any Linux distro" is the same shape of claim the tarball README just had to
retract, and "Debian/Ubuntu" now silently excludes Debian 12 and Ubuntu 22.04
LTS.

Graded LOW rather than higher, for two reasons I checked rather than assumed.
First, the column is headed "For" and the rows read as artifact-to-distro-family
routing (deb -> Debian family, rpm -> Fedora family, AppImage -> neither), not
as a glibc reach claim. Second, the claim was **already** imprecise at the old
floor: 2.35 already excluded RHEL 8 (2.28), Debian 11 (2.31) and Ubuntu 20.04
(2.31), so the change widens an existing gap rather than newly falsifying the
row. Mitigation already in the file: line 1 links `docs/INSTALL.md#linux`.

What is new is that the gap now swallows a currently-supported LTS. Outside the
implementer's exhaustive file list, so not its defect. Owner's call whether one
clause is worth it.

### 3. LOW - `renovate.jsonc:51-54`: confirmed stale, as the implementer flagged

Present and reading, verbatim:

```
      // release.yml pins ubuntu-22.04 on purpose (D85: the oldest supported
      // base for the AppImage glibc floor) while the test matrix runs
      // ubuntu-26.04. Renovate cannot know that distinction, so runner
      // images are not managed at all.
```

Confirmed as reported. The rule it justifies (`matchDepTypes: ["github-runner"]`,
`enabled: false`) is unaffected and still correct; only the named version is
wrong. Controller is handling it. One-word fix.

### 4. INFO - `docs/process-conventions.yaml:315`: the second copy, also confirmed

House-knowledge YAML, controller's sole write. The 2026-07-29 plan-10 entry
carries the same sentence ("release.yml pins ubuntu-22.04 on purpose for the
AppImage glibc floor (D85)"). Confirmed present.

### 5. INFO - `docs/ROADMAP.md:703-721`: trigger says pending while master carries the ruling

The entry still reads "**NOT YET CONSUMED** ... Decision pending." Controller's
write, and known.

One substantive note for that write, which is dimension 7 applied to the
trigger's own text. The prescription at `:704-705` is to record "the raised
glibc/**webkit** floor". Only the glibc half exists. I ran the premise rather
than assuming the webkit half was dropped correctly:

- the requirement in both texts is a **package name** (`webkitgtk 4.1` /
  `libwebkit2gtk-4.1-0` / `webkit2gtk4.1`), and that name is identical on both
  bases;
- the distribution the owner's decision was designed to keep, Debian 13, ships
  `libwebkit2gtk-4.1-0` at **2.52.5-1~deb13u1**, which is *newer* than the
  build-time `libwebkit2gtk-4.1-dev` **2.52.3-0ubuntu0.24.04.1** on noble/amd64.

So there is no raised webkit floor to record - the trigger's wording anticipated
one that did not materialize. The consumption note should **say that
explicitly** rather than silently satisfying half a two-part prescription.

### 6. INFO (closing an implementer open item) - the apt set does resolve on 24.04

The report left its own named breakage candidate on recall ("that is recall, not
a measurement"). I measured it. Every non-trivial package in the "Install Tauri
Linux build dependencies" step at `release.yml:96` exists in noble:

| package | noble version |
|---|---|
| `libwebkit2gtk-4.1-dev` | `2.52.3-0ubuntu0.24.04.1` (amd64) |
| `libayatana-appindicator3-dev` | `0.5.93-1build3` |
| `libxdo-dev` | `1:3.20160805.1-5build1` |
| `librsvg2-dev` | `2.58.0+dfsg-1build1` |

(`build-essential`, `curl`, `wget`, `file`, `libssl-dev` are base-archive
staples.) Note for the record: `libwebkit2gtk-4.1-dev` is `2.44.0-2` on
arm64/armhf/ppc64el/riscv64/s390x, which does not affect this leg - the
`linux-x86_64` leg is amd64 only.

**No finding is charged against the implementer for anything in 2-5**; all four
sit outside its declared file list, and it surfaced 3 and 4 itself.

---

## Dimensions

### 1. Every pin moved, and only the intended ones - PASS

Set derived from the file, not from the report. All seven `runs-on:` / `os:`
sites in `release.yml`:

| line | value | status |
|---|---|---|
| `:32` | `ubuntu-24.04` (guard `runs-on`) | moved |
| `:75` | `windows-2025` | untouched |
| `:78` | `windows-11-arm` | untouched |
| `:81` | `macos-15` | untouched |
| `:84` | `ubuntu-24.04` (matrix `os`, linux-x86_64 leg) | moved |
| `:86` | `${{ matrix.os }}` (bundle `runs-on`) | untouched |
| `:189` | `ubuntu-24.04` (assemble `runs-on`) | moved |

Three sites, exactly the three the brief names, derived independently. The three
non-Linux legs are untouched.

`.github/workflows/ci.yml` is **blob-identical across the commit** -
`278bc545d5d813973b35cf5cc34b22f3e3dabbc6` before and after - and still carries
`ubuntu-26.04` at all four of its sites (`:29` matrix, `:30`, `:164`, `:179`).
`ls .github/workflows/` confirms only `ci.yml` and `release.yml` exist, so the
pin universe is fully enumerated.

### 2. Workflow parses; structure unchanged - PASS

Not read off the diff. I wrote an independent instrument
(`runnerrev-independent/structdump.py`) that parses the YAML and emits every
leaf as a canonical `path = value` line, then diffed the two revisions
structurally.

- 78 leaf paths before, **78 after**.
- The full diff is **exactly three lines**, all three pins. Nothing else in the
  parsed tree moved: no key, no step index, no action SHA, no `run:` body, no
  `permissions`, no `needs`, no matrix shape.

**Fired control on the instrument** (a passing structural diff is an absence
claim, so the instrument had to be proven able to fire): I perturbed a scratch
copy in two independent ways - an action SHA and a `run:` body - and the same
diff caught both:

```
-.'jobs'.'guard'.'steps'[0].'uses' = 'actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0'
+.'jobs'.'guard'.'steps'[0].'uses' = 'actions/checkout@DEADBEEF'
-.'jobs'.'assemble'.'steps'[2].'run' = 'cd assets\nsha256sum * > SHA256SUMS\ncat SHA256SUMS\n'
+.'jobs'.'assemble'.'steps'[2].'run' = 'cd assets\nsha256sum * > PERTURBED\ncat SHA256SUMS\n'
```

### 3. The policy comment is TRUE after the edit - PASS

Both halves checked.

**The `windows-11-arm` half is untouched in substance.** The diff shows its line
changed only in the *other* clause on the same physical line; the text "Two
recorded deviations (D85): windows-11-arm has no dated label (the only GA
windows-arm64 image)" is identical. The claim still holds: the matrix at `:78`
still carries the undated `windows-11-arm`. "Two recorded deviations" still
counts two.

**The divergence half now describes 24.04-versus-26.04 and its rationale still
holds.** Verified against the vendor, not against the report: Tauri's AppImage
page states you "must build your Tauri application using the oldest base system
you intend to support that also provides Tauri v2's required WebKitGTK 4.1
packages", and that "Building on a newer base system can raise the minimum
glibc version required by your app, so when running on an older system, you may
face a runtime error like `/usr/lib/libc.so.6: version 'GLIBC_2.33' not found`."
The comment's new clause ("glibc being backward but not forward compatible") is
that mechanism stated correctly, and the swap from "oldest supported base" to
"oldest base we intend to support" is the vendor's own formulation.

The added floor figure (2.39) is verified (see dimension 4). The added binding
sentence names `docs/INSTALL.md` and the tar.gz README; both do state it. The
comment locates nothing by line number, per the owner ruling.

**D85 citation checked, not assumed.** `D85` is not a decision-ledger id; it is
the plan-8 design decision at
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:845`,
titled "Four native build legs with pinned runners; toolchain setup without
mise; no build cache on release legs". D85's substance is *pinned runners plus
its deviation set*, not a specific version, so the citation remains apt after
the version change. No citation drift.

### 4. The two user-facing requirement texts - PASS

**Same floor, verbatim in both:** "glibc 2.39 or newer"
(`packaging/linux-tarball-README.txt:22`, `docs/INSTALL.md:109`). Same two named
distributions with the same parenthetical figures in both.

**Every claim about the world, verified at the distributions' own package
databases** rather than accepted from the report or from the ROADMAP's measured
line:

| claim in shipped text | source | measured |
|---|---|---|
| Ubuntu 24.04 -> glibc 2.39 | `packages.ubuntu.com/noble/libc6` | `2.39-0ubuntu8.8` - **confirms** |
| Ubuntu 22.04 LTS -> 2.35 | `packages.ubuntu.com/jammy/libc6` | `2.35-0ubuntu3.14` - **confirms** |
| Debian 12 -> 2.36 | `packages.debian.org/bookworm/libc6` | `2.36-9+deb12u14`, bookworm (oldstable) - **confirms** |
| Debian 13 -> 2.41 (decision basis, not shipped text) | `packages.debian.org/trixie/libc6` | `2.41-12+deb13u3`, trixie (stable) - **confirms** |

Ubuntu 24.04 released April 2024, so "(the version in Ubuntu 24.04, 2024)" is
correct.

**The two claims that had gone false, and whether the replacement is accurate.**
The old bullet read:

```
- glibc 2.35 or newer (the version in Ubuntu 22.04, 2022); any current
  distribution qualifies. If this archive does not run on your system,
  the AppImage from the same release bundles its dependencies.
```

Both remaining statements were falsified by the floor rise, and the replacement
is accurate on both counts:

- *"any current distribution qualifies"* - flatly false at 2.39. Ubuntu 22.04
  LTS is current (standard support into 2027) and Debian 12 is oldstable; both
  are now below. The replacement names them, with figures I verified above.
- *"the AppImage ... bundles its dependencies"*, sitting directly after a glibc
  sentence, read as **the AppImage rescues an old-glibc system**. It does not,
  and this is the review's most consequential correctness point, so I verified
  it at the vendor rather than reasoning about it: Tauri's AppImage page names
  the exact failure (`GLIBC_2.33 not found`) as the reason to build on the
  oldest base, and nowhere claims glibc is bundled. Had that sentence stayed, a
  Debian 12 user blocked on glibc would have been routed to a second artifact
  built on the same runner, which cannot help. The replacement - "built on the
  same base and does not lift it; what it does bundle is webkitgtk and gtk3" -
  is correct and is consistent with the repo's own pre-existing claim at
  `docs/INSTALL.md:106-108` that the AppImage bundles those two.

Minor, not charged as a finding: naming exactly two distributions could be
misread as exhaustive (Debian 11, Ubuntu 20.04, RHEL 8 are also below). The
general rule is stated first ("2.39 or newer"), so the pair reads as examples.

### 5. No invented webkit version floor - PASS

Both texts still express the webkit requirement purely as package names -
`webkitgtk 4.1`, `libwebkit2gtk-4.1-0`, `libgtk-3-0`, `webkit2gtk4.1`. No
version number appears anywhere near webkit in either file. The implementer
followed the instruction exactly.

I additionally confirmed no webkit floor was *needed* (see finding 5): Debian
13's runtime webkit is newer than the build base's.

### 6. Nothing else moved - PASS

`git diff-tree --no-commit-id --name-status -r e260845`, derived rather than
taken from the report:

```
M	.github/workflows/release.yml
M	docs/INSTALL.md
M	packaging/linux-tarball-README.txt
```

Three files, all modifications, no additions/deletions/renames. Combined with
the structural diff in dimension 2: no action SHA, no job, no step logic, no
dependency, no permission changed.

### 7. The no-work-needed check - PASS, with one premise corrected from recall to measurement

Every "needed no change" or "could not break" conclusion in the report had its
premise run:

- *"ci.yml does not change"* - blob-identical, proven above.
- *"no webkit version was invented / none is needed"* - premise run at the
  package databases; confirmed (finding 5).
- *"the apt package set resolves on 24.04"* - the report explicitly labelled
  this **recall, not a measurement**. I measured it: all four non-trivial
  packages exist in noble (finding 6). Premise holds, and this open item is now
  closed.
- *"the binaries carry a 2.39 and not higher floor"* - the report lists this as
  unverified. It is in fact bounded by construction: a binary cannot require a
  glibc newer than the one it was linked against, and the base's own glibc *is*
  2.39. The published floor cannot be an understatement. It could in principle
  be an overstatement (the binaries might need less), which is the safe
  direction for a documented requirement.

### 8. The limit of the evidence - PASS, claim independently confirmed

The report's claim is that no gate part reads any of the three files, so a green
gate proves only that nothing else broke. **Confirmed, with a stronger and
different instrument than the report's grep.**

Rather than grepping for filename strings (which cannot see a dynamic or globbed
read), I traced the one gate part most likely to read documentation -
`python3 scripts/ledger-lint.py`, which by its own description reads
"BUILDING.md's gate enumeration" - under `strace -f -e trace=openat`. The
complete set of repository files it opens:

```
BUILDING.md
docs/conventions.yaml
docs/decision-ledger.yaml
docs/process-conventions.yaml
docs/product-boundaries.yaml
```

None of the three. This is a **positive-output** instrument: it produces the
list it is claimed to exclude from, so the absence is not an empty result whose
meaning depends on the invocation being well-formed.

Two supporting sweeps, each with a fired control:

- every `include_str!` / `include_bytes!` in `crates`, `src-tauri`, `scripts`
  (the only way a Rust gate part could embed a file at compile time): 22 hits,
  all locale `.ftl` files, test fixtures, or a schema. None is one of the three.
  Control: `assert_eq!` over the same scope returns hits.
- `linux-tarball-README|INSTALL\.md|release\.yml` across `crates`, `src-tauri`,
  `src`, `e2e`, `scripts`, `package.json`: no hits, exit 1. **Control fired:**
  the same instrument and scope for `BUILDING\.md|mise\.toml` returns
  `scripts/ledger-lint.py`.

**What remains unproven until a draft release run happens.** The gate cannot
touch it; only a `workflow_dispatch` with `rehearse-draft-release: true` can.
Open: that the bundle step's AppImage/deb/rpm production actually succeeds on
24.04; that `gh`, `jq`, `sha256sum` and `tar` behave identically on the new
image for `guard` and `assemble` (ROADMAP trigger 9 exists for exactly this
class); and that the produced deb's `Depends` is unchanged in shape.

**The most likely breakage point.** The report nominated the apt
build-dependency install at `release.yml:92-96`; that was the right candidate
*class*, and I have now closed it by measurement, so it is no longer the answer.
With apt resolution established, the highest residual risk is the **AppImage
half of the bundle step** at `release.yml:119-121`
(`pnpm exec tauri build --bundles deb,rpm,appimage`). It is the only step whose
behavior depends on the host's library layout rather than on package names: the
Tauri AppImage bundler fetches linuxdeploy at build time and copies the host's
WebKitGTK and GTK libraries out of distribution-specific paths, and 24.04 moved
several of those relative to 22.04. Stated as a reasoned prior from the
workflow's own steps, not a measurement; the rehearsal is what settles it.

---

## Adjudications

### 1. The tarball README rewrite's breadth - **correct repair, not scope creep**

The brief's mandate was that the requirement texts "must agree with each other
**and with the new base**". That makes the truth of the bullet in scope, not
just its version token. Both remaining sentences were falsified by the change
itself:

- "any current distribution qualifies" became flatly false, since a
  currently-supported LTS now fails.
- the AppImage fallback became actively harmful, routing a glibc-blocked reader
  to an artifact built on the same runner - verified against Tauri's own page,
  which names that exact failure mode as the reason the build-on-oldest rule
  exists.

Swapping only the number would have shipped two false statements to satisfy a
narrow reading of "one line". A brief that asks for agreement with the new base
cannot simultaneously require leaving statements that the new base contradicts.

The decisive evidence that this was repair and not creep is what the implementer
did **not** touch: the neighbouring mkvtoolnix and webkit/gtk bullets in the same
list are untouched, because the change did not falsify them. Every sentence
edited was one the edit itself broke. That is the exact boundary scope creep
would have crossed.

### 2. `docs/INSTALL.md`'s new bullet placement - **mildly misleading; yes, fix it, and the fix is one sentence**

Misleading, though not by being wrong. The bullet's content is accurate and it
even repairs its own scope internally. The defect is structural: it lives under
an intro that tells deb/rpm readers the list is for AppImage and tar.gz users,
and it is the document's only glibc notice.

The reason this is not merely theoretical is the measurement in finding 1: the
generated deb declares `Depends: libwebkit2gtk-4.1-0, libgtk-3-0` and **no
libc6**, so nothing else in the chain warns a deb user. The install succeeds and
the binary then fails at runtime. The doc is the only channel, and it is filed
under a heading that reader was told to skip.

Weighed against it: the glibc floor genuinely *is* most acute for tar.gz and
AppImage users, who get the raw and cryptic failure, and those are exactly the
readers the list addresses. So the placement is not absurd - it is the
second-best spot.

**Minimal fix** (full text in finding 1): lift the bullet out of the list into a
standalone sentence immediately before it. No rewording, no restructuring of the
section, no new claims - the same sentence, moved out of a scope that excludes
half its audience. The implementer was right that restructuring the section
would have exceeded its brief, and right to flag this rather than do it.

### 3. Naming two distributions as unsupported - **right service to the reader; keep it**

Keep it, and the dating objection inverts on inspection.

"glibc 2.39" is not a figure a media-library curator can act on. Nobody
downloading a bulk MKV muxer runs `ldd --version` first. "Ubuntu 22.04 LTS" and
"Debian 12" are names a reader recognizes instantly as their own system or not.
The named pair converts an unevaluable number into a decision the reader can
actually make, and the number is still there for anyone who wants it.

On dating: the *named versions with figures* are permanently true. Ubuntu 22.04
shipped glibc 2.35 and always will have; Debian 12 shipped 2.36 and always will
have. These statements cannot rot. What rots is a floor stated *relatively* -
and that is precisely what the old text did: "any current distribution
qualifies" dated itself the moment "current" moved, which is exactly the defect
this change had to repair. The plain version floor is the option that dated
badly here, not the named one.

The residual risk is not staleness but *incompleteness* when the floor next
moves - the names must be re-checked, not just the number. That risk is already
addressed inside the change: the new policy comment at `release.yml:26-28` binds
the pin to both texts explicitly, so the next person moving the pin finds the
reach consequence in the file that causes it.

---

## What else in the tree still asserts the old base

Swept the full tracked tree plus untracked and ignored files, for `22.04`,
`ubuntu-22`, `jammy` and `2.35`. **Confirmed: the `renovate.jsonc` comment the
implementer surfaced does exist** (finding 3). Beyond it, in live files:

| file | line | nature |
|---|---|---|
| `docs/process-conventions.yaml` | `:315` | **stale**, second copy of the renovate sentence. House-knowledge YAML, controller-only. |
| `docs/ROADMAP.md` | `:703-721` | **not stale** - the trigger entry, correctly recording the pre-decision state; needs the controller's consumption write. |
| `docs/INSTALL.md`, `packaging/linux-tarball-README.txt` | `:112`, `:23` | **not stale** - the new, correct texts naming 22.04 as *below* the floor. |

Not stale, correctly frozen, no action: every hit under
`docs/process-journal/artifacts/**` (dated per-plan records), the plan-8 design
spec `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`, and
the plan docs under `docs/superpowers/plans/`. These record what was decided
then and should keep saying 22.04.

**One live file asserts the old *reach* without naming the old base**, which a
literal `22.04` sweep does not catch: `.github/release/draft-body.md:10,12` -
see finding 2. Flagging it as the answer to "anything ELSE", since it is the
only remaining user-facing text whose Linux claims the floor rise touches.

Method note: my first untracked-file sweep used `rg -E`, which is
`--encoding`, not a pattern flag - it returned empty for a malformed reason. I
caught it and re-ran with `-e` plus a fired `ubuntu-24` control before trusting
any absence here.

---

## Evidence appendix

Instruments under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/runnerrev-independent/`.
No instrument written by the implementer was re-run.

| # | Instrument | Result |
|---|---|---|
| E1 | `structdump.py` (mine) over `e260845^` vs `e260845` | 78 leaves each; diff = exactly 3 pin lines |
| E2 | E1 against a doubly-perturbed scratch copy | **control fired**: caught both an action SHA and a `run:` body |
| E3 | `grep -nE '^\s*(runs-on:\|os:)'` on both workflows | 7 sites in release.yml, 4 in ci.yml; set derived from the file |
| E4 | `git rev-parse e260845{^,}:.github/workflows/ci.yml` | `278bc545...` both sides - blob-identical |
| E5 | `git diff-tree --name-status -r e260845` | exactly 3 `M` entries |
| E6 | `packages.ubuntu.com/{noble,jammy}/libc6` | `2.39-0ubuntu8.8`, `2.35-0ubuntu3.14` |
| E7 | `packages.debian.org/{bookworm,trixie}/libc6` | `2.36-9+deb12u14`, `2.41-12+deb13u3` |
| E8 | `packages.ubuntu.com/noble/` x4 build deps | all present; versions in finding 6 |
| E9 | `packages.debian.org/trixie/libwebkit2gtk-4.1-0` | `2.52.5-1~deb13u1` > noble's build-time `2.52.3` |
| E10 | Tauri AppImage distribution guide, fetched | confirms oldest-base rule + `GLIBC_2.33 not found` failure mode; no glibc bundling claim |
| E11 | `strace -f -e trace=openat python3 scripts/ledger-lint.py` | opens 5 repo files; none of the three |
| E12 | `include_str!`/`include_bytes!` sweep + `assert_eq!` control | 22 hits, all locales/fixtures/schema; **control fired** |
| E13 | basename grep over source trees + `BUILDING\.md` control | no hits, exit 1; **control fired** (`scripts/ledger-lint.py`) |
| E14 | tree-wide `22.04\|ubuntu-22\|jammy\|2.35` sweep, tracked + ignored | table above; `ubuntu-24` control fired |
| E15 | `tauri.conf.json` bundle reader + `productName` control | `recommends` only, no `depends`; **control fired** |
| E16 | `task-6-report.md:926`, captured rehearsal deb control file | `Depends: libwebkit2gtk-4.1-0, libgtk-3-0` - no `libc6` |

Two instruments of mine failed and were caught before their results were used:
a decision-ledger reader keyed on `id: D85` (whose `D77` control also returned
nothing, exposing the wrong schema assumption - `D85` is a design-doc number,
not a ledger id), and the `rg -E` sweep noted above. Both re-run correctly.

## Tree identity

The tracked tree is byte-identical to `e260845`. Proven per file against blobs,
not by a clean `git status`:

```
HEAD                                     e26084557f9351fe9e33bfe37b1092ebbabbe1b9
git hash-object <working tree>  ==  git ls-tree HEAD
  .github/workflows/release.yml       cd314a4ed59d560a3f23fce153deff2880c56dc7
  docs/INSTALL.md                     70e5aed98985ee45080b43155d2bdfacd01e6225
  packaging/linux-tarball-README.txt  6ab56d479e312971524a84143ac9a86dec0083a6
```

Full `git ls-tree -r HEAD` (1400 entries) captured before the review and
re-compared after. This verdict file is the only thing I wrote into the
repository path, and `.superpowers/` is gitignored
(`git check-ignore -v` -> `.gitignore:2`), so it cannot alter the tracked tree.
I committed nothing and edited no product file.

## HARVEST

1. **A green gate that cannot see the change is worth stating as a limit, and
   worth proving with a positive-output instrument.** The report proved it with
   greps that return nothing; `strace` on the one gate part that reads docs
   returns the *list* of files it opens, and the three are absent from a list
   that exists. An absence inside produced output is a different quality of
   evidence than an absent output.

2. **"Recall, not a measurement" is an honest label and also a to-do.** The
   report correctly flagged its apt-resolution prior as unmeasured. It was four
   package-page fetches away from being measured. When a briefed constraint says
   a figure "is not yours to re-derive", that scopes the *decision basis*, not
   every adjacent checkable fact.

3. **When a version floor rises, the falsified claims are rarely where the
   version number is.** The number lived in one clause; the damage lived in
   "any current distribution qualifies" and in a fallback pointing at an
   artifact that cannot help. A brief that names "the glibc requirement line"
   under-describes its own blast radius. Better instruction shape: name the
   claim, not the line.

4. **A relative floor dates a document; an absolute one with named examples does
   not.** The rotted sentence was the one using the word "current". Worth
   carrying into future user-facing requirement text.

5. **A doc bullet can be correct and still unreachable.** Correctness review of
   documentation has to include *who is told to read this section*, because a
   true sentence under a heading the affected reader was invited to skip
   communicates nothing. The measurement that turned this from taste into a
   finding was the deb's real `Depends` field, sitting in a journal artifact
   from a previous rehearsal.

6. **A second stale copy of a sentence is the normal case, not the exception.**
   The renovate comment had a twin in `process-conventions.yaml`. When a fact is
   worth a comment in a config file, grep for its other homes at the moment the
   fact changes.
