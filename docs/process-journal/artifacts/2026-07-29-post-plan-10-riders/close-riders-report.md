# Implementer report - four close riders

**Commit:** `d9a4fa2` - `docs: the glibc floor reaches deb/rpm readers, the
release table stops overclaiming, two README figures retire`
Unsigned (`git log -1 --format='%G?'` prints `N`), one trailer
(`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`), four files staged by
name. Not pushed. Working tree clean afterwards.

**Status:** all four riders implemented, gate green (11/11), validator green in
both modes. Four concerns at the end, none of them blocking; concern 1 is the
one I would act on next.

---

## Rider 1 - `docs/INSTALL.md`, the glibc floor lifted out of the list

**Before** - last bullet of the runtime-requirements list, i.e. inside the block
whose intro reads "The deb/rpm packages declare **mkvtoolnix** as a recommended
dependency; the AppImage and tar.gz do not manage dependencies, so install the
runtime requirements yourself:":

```markdown
- **glibc 2.39 or newer** - not a package you install but what your
  distribution ships: every Linux artifact here, the AppImage
  included, is built on Ubuntu 24.04 (glibc 2.39), so systems below
  that floor - Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot
  run them.
```

**After** - a standalone paragraph between the "Unsigned packages (Fedora)"
paragraph and the "No gatekeeping dialog exists on Linux" paragraph that
introduces the list:

```markdown
**glibc 2.39 or newer** - not a package you install but what your
distribution ships: every Linux artifact here, the AppImage included,
is built on Ubuntu 24.04 (glibc 2.39), so systems below that floor -
Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot run them.
```

**Content is provably unchanged** - only the list marker, the two-space
continuation indent and the line wrapping moved:

```
$ norm() { tr '\n' ' ' | tr -s ' ' | sed 's/^ //; s/ $//'; }
$ old=$(git show HEAD~1:docs/INSTALL.md | awk '/^- \*\*glibc/,/run them\.$/' | sed 's/^- //; s/^  //' | norm)
$ new=$(awk '/^\*\*glibc/,/run them\.$/' docs/INSTALL.md | norm)
OLD: **glibc 2.39 or newer** - not a package you install but what your distribution ships: every Linux artifact here, the AppImage included, is built on Ubuntu 24.04 (glibc 2.39), so systems below that floor - Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot run them.
NEW: **glibc 2.39 or newer** - not a package you install but what your distribution ships: every Linux artifact here, the AppImage included, is built on Ubuntu 24.04 (glibc 2.39), so systems below that floor - Ubuntu 22.04 LTS (2.35), Debian 12 (2.36) - cannot run them.
IDENTICAL after whitespace normalisation
```

Both ranges are content-anchored, and both select a non-empty region (5 lines
before, 4 after) - so the identity is not the vacuous equality of two empty
strings.

**Why this placement.** The prescription is "immediately BEFORE it". The list's
intro paragraph ends in a colon and hands directly into the bullets, so an
insertion between intro and bullets would both break that sentence and leave the
notice inside exactly the region the deb/rpm reader has just been told to skip.
The paragraph therefore sits immediately before the intro+list block, which is
the closest grammatical placement that also achieves what the finding asks for:
the floor is now stated before the sentence that invites deb/rpm readers to skip
the requirements, so its position asserts what its text asserts. See concern 3
if the controller meant the other reading.

**Measured, after the edit:** `grep -c glibc docs/INSTALL.md` -> 2, both
occurrences inside this one paragraph. It remains the document's only floor
notice; it is no longer inside a list.

**Borrowed, not verified by me:** the reviewer's `Depends: libwebkit2gtk-4.1-0,
libgtk-3-0` measurement on the shipped deb - I have no release artifact here.
What I could check in-tree corroborates it partially and is stated as such in
the commit body: `grep -rni depends src-tauri/*.json` finds nothing, so no
`libc6` (nor any other) dependency is authored in the bundler config; whatever
the deb declares, it is the Tauri bundler's auto-derived set.

## Rider 2 - `.github/release/draft-body.md`, two table rows stop overclaiming

**Before:**

```markdown
| `muxsmith-__VERSION__-linux-x86_64.deb` | Debian/Ubuntu |
| `muxsmith-__VERSION__-linux-x86_64.rpm` | Fedora & co. |
| `muxsmith-__VERSION__-linux-x86_64.AppImage` | any Linux distro |
```

**After:**

```markdown
| `muxsmith-__VERSION__-linux-x86_64.deb` | Debian 13+ / Ubuntu 24.04+ |
| `muxsmith-__VERSION__-linux-x86_64.rpm` | Fedora & co. |
| `muxsmith-__VERSION__-linux-x86_64.AppImage` | any distro with glibc 2.39+ |
```

**Why these two phrasings.** Both rows had to stop asserting a reach the
artifacts do not have, while the constraint says: short routing phrases, no new
column, no new row, and the floor not restated in every row.

- The deb row's overclaim is that *all* Debian/Ubuntu can install it. Its reader
  is at an `apt install` prompt and checks a distro version, not a glibc
  version, so the floor is expressed the way that reader can act on it. The
  version claims follow from the floor and from the texts already in this repo:
  Ubuntu 24.04 is glibc 2.39 (the base itself), Debian 12 is 2.36 and therefore
  excluded (INSTALL.md and the tarball README both name it), Debian 13 is above
  the floor.
- The AppImage row's overclaim is universality, so "any" keeps its place and
  gains the condition that makes it true. This is the row that names the floor,
  and it is the only one that does - the constraint against restating it in
  every row holds.

The rpm row and the three non-Linux rows are byte-identical; the detail stays
with the per-OS INSTALL.md links in the body's opening sentence, which are
untouched. Concern 2 covers the
tar.gz row.

## Rider 3 - `renovate.jsonc`, the stale runner version in a comment

**Before / after** - one token, inside the comment only:

```diff
-      // release.yml pins ubuntu-22.04 on purpose (D85: the oldest supported
+      // release.yml pins ubuntu-24.04 on purpose (D85: the oldest supported
```

The remaining three comment lines, the `matchManagers` / `matchDepTypes` /
`enabled` keys and all their values are unchanged, so the stated reason (the
test matrix runs ubuntu-26.04, Renovate cannot know that distinction, runner
images are therefore unmanaged) still stands and still explains why the rule
exists.

**Why the rest is left alone.** `release.yml` pins `ubuntu-24.04` in all three
jobs, and `ci.yml` still pins `ubuntu-26.04` in its matrix and its two
single-runner jobs, so every other clause of the comment is still true. D85 is
still the right citation: release.yml's own pinning-policy comment records the
ubuntu-24.04 divergence as a D85 deviation. `grep -n '22\.04\|24\.04\|26\.04'
renovate.jsonc` shows the file carries no other version token.

## Rider 4 - `README.md`, both figures retired from "How this got built"

**Before** (single paragraph, sentences 2 and 3):

> Every design decision is numbered and recorded with its rationale and rejected
> alternatives: 103 of them so far, running up to `D105` because two numbers
> were reserved for a plan that never spent them. The whole process is public in
> this repo: [docs/](docs/) carries the process journal, every plan, and the
> preserved review verdicts - 225 files under `docs/` with `verdict` in the
> name, including the ones that hurt.

**After:**

> Every design decision is numbered and recorded with its rationale and rejected
> alternatives. The whole process is public in this repo: [docs/](docs/) carries
> the process journal, every plan, and the preserved review verdicts, including
> the ones that hurt.

Both claims survive intact: decisions are numbered and recorded with rationale
and rejected alternatives; verdicts are preserved in the repo, including the
ones that hurt. The `D105` clause goes with the count it existed to reconcile -
"two numbers were reserved for a plan that never spent them" only explains the
gap between 103 and D105, and with neither figure present it explains nothing.
The dash before "including the ones that hurt" becomes a comma because the
appositive it introduced (the file count) is gone. Sentence 1 of the paragraph
and the two neighbouring paragraphs are untouched; register unchanged.

---

## Verification

### 1. The full gate, foreground, 11/11 green

Run in the order `BUILDING.md` enumerates, each in its own foreground
invocation, exit status captured per part:

| # | Part | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | exit 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| 3 | `cargo test --workspace` | exit 0 - 39 suites, 505 passed, 0 failed, 0 ignored |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | exit 0 |
| 5 | `cargo deny check` | exit 0 - `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | exit 0 |
| 7 | `pnpm lint` | exit 0 |
| 8 | `pnpm build` | exit 0 - vue-tsc clean, 165 modules |
| 9 | `pnpm check:i18n` | exit 0 - `check-i18n: ok (41 source files scanned, 212 catalog ids, ...)` |
| 10 | `pnpm test:e2e` | exit 0 - 68 passed |
| 11 | `python3 scripts/ledger-lint.py` | exit 0 - `546 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

**What this proves, and what it does not.** The gate proves the tree is still
green - these edits broke nothing the gate observes. It verifies **none** of the
four riders, because **not one of the four edited files is in any gate part's
input set.** That is measured here, not assumed:

- `grep -rn draft-body scripts crates src src-tauri e2e` -> no match.
- `grep -rni renovate scripts crates src src-tauri e2e` -> no match. `renovate.jsonc`
  is covered only by the vendor validator, run separately below.
- `grep -rn README scripts crates src src-tauri e2e` -> five matches, all
  comments. Four are in `crates/muxsmith-cli/tests/run_live.rs`, and they
  describe a **transcribed literal** of the README's passthrough YAML block, not
  a read of the file: that test's only filesystem reads are of run outputs
  (`fs::read_dir(&runs_root)`, the summary path, the two muxed outputs). So the
  one test coupled to README content is coupled to a code block I did not touch,
  and no test asserts README prose. The fifth match is about a bundle's own
  `README.macOS.txt`, unrelated.
- `grep -rn INSTALL scripts crates src src-tauri e2e` -> one match, a comment
  about mkvtoolnix's own INSTALL file. Nothing reads `docs/INSTALL.md`.
- `scripts/ledger-lint.py` reads exactly `docs/conventions.yaml`,
  `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`,
  `docs/decision-ledger.yaml` and `BUILDING.md` - none of my four.
- `pnpm lint` does not lint Markdown or JSONC. Measured rather than read off the
  config: `pnpm exec eslint . -f json` lists 37 `.ts`, 24 `.vue`, 1 `.mjs`,
  1 `.js` - no `.md`, no `.jsonc`.

So rider 1, 2 and 4 rest on reading the rendered text; rider 3 rests on the
validator below. The gate is a regression guard here, nothing more.

### 2. Renovate validator, both modes

`renovate@43.287.0` still resolves; no fallback to `latest` was needed.

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator renovate.jsonc
 INFO: Validating renovate.jsonc as global config
 INFO: Config validated successfully against 1 file(s)
EXIT: 0

$ npx --yes --package renovate@43.287.0 -- renovate-config-validator --strict renovate.jsonc
 INFO: Validating renovate.jsonc as global config
 INFO: Config validated successfully against 1 file(s)
EXIT: 0
```

**Control** (a passing validator is an absence, so it was made to fire once): a
copy of the edited file with `"enabled": false` replaced by
`"enabled": "false-not-a-bool", "bogusKeyXyz": 1` in all three occurrences:

```
$ npx --yes --package renovate@43.287.0 -- renovate-config-validator renovate-broken.jsonc
...
           "message": "Configuration option `packageRules[5].enabled` should be boolean. Found: \"false-not-a-bool\" (string)"
           "message": "Invalid configuration option: packageRules[5].bogusKeyXyz"
EXIT: 1
```

The validator does read the file it is handed and does fail it.

### 3. README figure grep, with a fired control

The edited paragraph is located by content, not by line number:

```
$ n=$(grep -n 'The setup: implementer agents' README.md | cut -d: -f1)
$ sed -n "${n}p" README.md | grep -o '[0-9][0-9]*'                       # exit 1, no output
$ sed -n "${n}p" README.md | grep -oEi '\b(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|dozen|hundred|thousand)\b'
                                                                          # exit 1, no output
```

Two patterns because a retired figure could survive as a spelled-out word
("two numbers were reserved" did): the digit grep is deliberately unanchored to
any particular numeral so it cannot miss a member of an enumerated set, and the
word grep carries the enumeration that the digit grep does not need.

**Control** - the same two greps against the pre-edit paragraph
(`git show HEAD~1:README.md`, `HEAD` at the time of the run):

```
--- digit grep (control) ---     103, 105, 225      exit 0
--- spelled-number grep (control) ---   two         exit 0
```

Both fire on the known-present case, so both are sound instruments; note the
word grep does *not* match "the ones that hurt", which is why the after-state
result of "no output" is meaningful rather than an artifact of `\b`.

Whole-file check as well: `grep -n '225\|D10[0-9]\|103 of them' README.md` ->
exit 1, so neither retired figure survives elsewhere in the README.

### 4. Typography

`git diff -U0 -- <the four files> | grep '^+' | grep -P '[^\x00-\x7F]'` -> exit
1: every added line is pure ASCII, so no en/em dash, no smart quote, no Unicode
ellipsis, no non-breaking space. Control: the same pattern against `README.md`
fires immediately (the emoji headings). No `file:line` citation appears in any
text I wrote, here or in the tree.

### 5. `git diff --stat`

Pre-commit, working tree against `HEAD`:

```
 .github/release/draft-body.md |  4 ++--
 README.md                     |  2 +-
 docs/INSTALL.md               | 10 +++++-----
 renovate.jsonc                |  2 +-
 4 files changed, 9 insertions(+), 9 deletions(-)
```

Exactly the four files the brief permits; `git show --stat --format= d9a4fa2`
reproduces it, and `git status --short` after the commit is empty. The report
you are reading is not in the commit: `git check-ignore -v` confirms
`.superpowers/` is matched by a `.gitignore` rule, and `git ls-files .superpowers`
returns nothing.

---

## Concerns

1. **`docs/INSTALL.md`'s own artifact list still says "any distro".** Its Linux
   artifacts list routes the AppImage as "any distro: `chmod +x` the file, then
   run it" - the identical overclaim rider 2 just removed from the release
   table, in the same document whose floor notice rider 1 just promoted, three
   paragraphs above it. `grep -rn 'any distro\|any Linux'` over README.md,
   docs/INSTALL.md, the draft body, packaging/ and the workflows returns exactly
   two hits: that line and the AppImage row I corrected. I left it: rider 1's
   WHAT is fixed at lifting the bullet, and the brief scopes INSTALL.md to rider
   1, so touching it would be design latitude I do not have. It is a one-phrase
   follow-up rider.

2. **The tar.gz row is in neither of rider 2's sets.** The rider names two rows
   to correct (deb, AppImage) and enumerates the untouched ones as "the rpm row
   and the three non-Linux rows" - six of the table's seven. The tar.gz row
   ("portable, CLI + GUI") is named neither way. I left it unchanged, reading
   "portable" as a claim about the archive's form (no installer, both binaries)
   rather than about distro reach, and reading "the two rows are corrected" as
   exhaustive. Flagged because the untouched enumeration is one short of
   accounting for the table.

3. **Rider 1's placement has two readings and I picked one.** "Immediately
   BEFORE it" could mean between the colon-ended intro sentence and the bullets.
   That reading requires rewriting the intro sentence to not hand into a list it
   no longer touches - rewording beyond the rider - and it would leave the floor
   inside the passage the deb/rpm reader was just told to skip, defeating the
   finding. I placed the paragraph before the intro instead. Cheap to move if
   the controller meant the other one; the paragraph is self-contained.

4. **A neighbouring enumeration is now one text short.** `release.yml`'s
   pinning-policy comment states that the base "sets the shipped glibc floor at
   2.39; docs/INSTALL.md and the tar.gz README state it, so moving this pin
   moves the product's reach and those two texts with it." After rider 2 the
   release draft body states the floor as well, so the next floor move has three
   texts to carry, not two, and the comment that exists to say so names two.
   `release.yml` is outside my exhaustive file list, so I did not touch it. This
   is the change's one propagation debt and worth a rider of its own.
