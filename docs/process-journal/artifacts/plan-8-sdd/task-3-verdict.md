# Task 3 verdict: D75/D77/D79/D88 collateral (INSTALL.md, release-body templates, tar.gz README, README rider)

Reviewer: independent task reviewer. Branch `plan8-b`, commit
`c890b0f9372517d7a716390fb76101b66f94e24a`, range `aec4cef..c890b0f`.
Ground truth read from the **main-tree** design copy
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(2061 lines, A1 present).

- **Spec compliance: PASS**
- **Task quality: PASS**

No blocker, high or medium finding. Nothing routes to a fix loop.

---

## 1. A1 non-interference (verified, not assumed)

The implementer transcribed from the worktree design copy, which predates A1.
Whole-file diff main-tree vs worktree copy carries exactly three hunks: the
status line (A1 mention), section 2 fence line `100` (the quoted step-name
scalar), and the appended amendment log. Section 4 is untouched.

Content-anchored comparator (`## 4. Documentation artifacts (verbatim)` ->
`## 5. mkvtoolnix parity audit`), 205 lines both sides:

```
diff -u sec4-main.txt sec4-wt.txt   -> empty
```

**Firing control** for that comparator, same method over section 2 (the region
A1 *did* touch): fires on the one quoted-scalar line. The empty section-4
result is therefore a real absence, not a broken comparator. The design text
the implementer transcribed from is byte-identical to ground truth.

---

## 2. Spec compliance

### 2.1 Transcription fidelity - re-diffed independently, content-anchored

Extraction keyed on structural anchors (section heading / path-label line ->
opening fence -> matching close), never on line numbers, per
`proc-wrapped-prose-quote-grep`. Compared against `git show c890b0f:<path>`,
not the working tree.

| design block | file | lines | diff |
|---|---|---|---|
| 4.1 (inside the outer ````markdown fence) | `docs/INSTALL.md` | 83 | **empty** |
| 4.2 first (`.github/release/draft-body.md`:) | `.github/release/draft-body.md` | 24 | **empty** |
| 4.2 second (`.github/release/rehearsal-banner.md`:) | `.github/release/rehearsal-banner.md` | 6 | **empty** |
| 4.3 (```text fence) | `packaging/linux-tarball-README.txt` | 27 | **empty** |
| 4.5 (```markdown fence) | README rider added lines (from `git diff`) | 4 | **empty** |

The 4.1 extraction preserves the inner ` ```sh ` fence and its close (verified
by grepping the extract for backtick runs: hits at extract lines 58 and 60),
so the nested-fence hazard the brief names was not silently swallowed.

**Firing controls on my own diff harness**, all three fired against
`docs/INSTALL.md`:

| mutation class | result |
|---|---|
| truncation to 70 lines | FIRED |
| single word (`unsigned build.` -> `unsigned builds.`) | FIRED (`27c27`) |
| trailing space added to line 5 | FIRED |

Mutations applied to scratch copies only; no repo file touched.

Byte sizes and trailing newline, read from the committed blobs: 3624 / 1177 /
231 / 1088 bytes, last byte `\n` in all four. Matches the report.

### 2.2 D75 content completeness (per-OS install hurdles + the CLI/PATH ruling)

D75's outline walked item by item against the committed `docs/INSTALL.md`;
D82 §4 supplies the manual-PATH ruling.

Line numbers below are counted in the committed `docs/INSTALL.md` (83 lines).

| D75 / D82 requirement | present | location |
|---|---|---|
| Windows: SmartScreen "Windows protected your PC" -> More info -> Run anyway | yes | INSTALL.md:24-26 |
| Windows: unknown-publisher framing | yes | INSTALL.md:26-27 |
| Windows: per-machine install to `C:\Program Files\Muxsmith` | yes | INSTALL.md:21-22 (anchor: design 124-125, `InstallScope="perMachine"` -> `ProgramFiles64Folder\<productName>`) |
| Windows: **manual PATH step** (D82 §4) | yes | INSTALL.md:29-34, full Settings click path, ends "then open a new terminal" |
| macOS: unsigned and not notarized | yes | INSTALL.md:43 |
| macOS 15+: first open blocked -> System Settings > Privacy & Security > Open Anyway | yes | INSTALL.md:45-48 |
| macOS 11-14: Control-click -> Open | yes, marked pre-15 only | INSTALL.md:49-50 |
| macOS: `xattr -d com.apple.quarantine` terminal alternative | yes | INSTALL.md:51-52 |
| macOS: CLI location inside the bundle + **manual PATH step** (D82 §4) | yes | INSTALL.md:54-60, `Contents/MacOS/muxsmith` + `sudo ln -s` into `/usr/local/bin` |
| Linux: no gatekeeping hurdle | yes | INSTALL.md:72 |
| Linux: AppImage `chmod +x` | yes | INSTALL.md:68 |
| Linux: mkvtoolnix runtime install per distro | yes | INSTALL.md:78-79 (anchor: D80) |
| Linux: deb/rpm already place both binaries on PATH | yes | INSTALL.md:72-73, `/usr/bin` (anchor: D82 §3) |
| Linux tar.gz: run from extraction dir or link it (D82 §4) | yes, by pointer | INSTALL.md:69-70 -> `README.txt`; the tar.gz README (lines 10-12) carries the run/PATH/symlink text |
| HTML comment naming the file's own obsolescence condition (D75 interface changes) | yes | INSTALL.md:13-15 |

D82 §4's macOS parenthetical alternative ("or add the `Contents/MacOS` dir to
PATH") is not carried. It is an `or`, and D75's own outline demands only "the
manual PATH step" - satisfied by the `ln -s`. Not a gap; listed in the owner
pass as an optional addition.

### 2.3 Structural checks - recounted from their iteration domains, not trusted

The gate outputs were re-derived by reading the *domain* each count summarizes,
per the brief's instruction, not by re-running the count alone.

**INSTALL.md headings** - full heading domain (`^#+ `), not just `^## `:

```
1:# Installing Muxsmith
17:## Windows
36:## macOS
62:## Linux
```

Exactly three H2s, no fourth heading at any level. GitHub derives `#windows`,
`#macos`, `#linux` from precisely these, and the draft-body's three links
target exactly those three fragments (verified by extracting the URLs). D77's
link contract holds.

**`__VERSION__` in draft-body** - full occurrence domain printed, then counted
two ways because `grep -c` counts lines, not occurrences:

```
line  1  heading line
lines 13-19  the seven table rows
grep -c (lines):       8
grep -o (occurrences): 8      <- no line carries two, so 8 == 8 is meaningful
table rows:            7
```

**Recount against D89, not against the brief's number.** D89 enumerates 8
release assets, items 1-7 being the `muxsmith-` files and item 8 `SHA256SUMS`.
The draft-body table carries assets 1-7; `SHA256SUMS` correctly appears in the
verify line, not the table. Set equality proven by sorted diff of D89's
names 1-7 (with `X.Y.Z` -> `__VERSION__`) against the table's names: **empty**.
Firing control: perturbing `arm64` -> `aarch64` in one set fires the
comparator. So 7 and 8 are correct, and correct *for the right reason*.

`rehearsal-banner.md` carries zero `__VERSION__` - correct, since design
section 2 line 1456 `cat`s it without a `sed` pass; only draft-body is
substituted (line 1458).

**Placeholder count** - domain read, not just the count:

| | line | comment |
|---|---|---|
| `aec4cef` | 7, 61, 99, 184 | count 4 |
| `c890b0f` | 7, 61, 99, 187 | count 4 |

Same four placeholders, same identities; only the line-99 comment's *text*
changed (the 4-line rider shifts the later one from 184 to 187). This is the
check the bare count cannot make: a count of 4 also survives deleting one
placeholder and adding another. Status preserved: still `placeholder(1.0)`,
not resolved (D81 / preamble bullet 11).

**Fire test, corrected.** My first attempt used the pattern
`placeholder(1.0): GUI screenshot` and returned 4 - it did **not** fire, because
the real text is `one GUI screenshot`. A malformed check producing a
pass-looking result, caught only because a fire test's expected outcome is a
*change*. Re-run with the correct pattern: **3**. Fired. (Performed on a
scratch copy of the committed blob; no repo file mutated.)

**README diff scope**: `--numstat` = `4 1 README.md`; the hunk is one line
removed, four added, nothing else. The commit touches exactly the five briefed
files (144 insertions, 1 deletion). No cross-stream file, no `ci.yml`, no
`v*` tag, no release object.

### 2.4 Typography

Fire control **first**, per-class, on a scratch file built with `printf`
escapes carrying one line per glyph class:

```
U+2014 hits=1   U+2013 hits=1   U+2026 hits=1
[U+201C U+201D U+2018 U+2019] hits=1   U+00A0 hits=1
total matching lines = 5
```

Every alternative in the pattern is live, not only the first - the failure mode
where a compound pattern silently tests one branch. The real scan over the four
new files plus the committed `README.md`: **no output**. Stronger check also
run: `grep -nP '[^\x00-\x7F]'` over the four new files returns nothing, i.e. no
non-ASCII byte at all. No trailing whitespace, no tabs.

### 2.5 Interfaces produced, against their consumers

| produced | consumer in the design | match |
|---|---|---|
| `#windows` / `#macos` / `#linux` anchors | draft-body's three links (design 1674-1676) | yes |
| `.github/release/rehearsal-banner.md` | design 1456 `cat .github/release/rehearsal-banner.md > body.md` | path exact |
| `.github/release/draft-body.md` | design 1458 `sed "s/__VERSION__/${version}/g" .github/release/draft-body.md >> body.md` | path exact |
| `packaging/linux-tarball-README.txt` | design 1414 `cp packaging/linux-tarball-README.txt "$stage/README.txt"` | path exact |
| trailing `---` in both templates | composition separator, banner -> template -> generated notes | both files' last line is `---` |

`packaging/` as a new top-level directory is D88's ruled placement.

### 2.6 Claims inside the frozen text that would have been design defects

Both re-verified against the tree, independent of the report:

- `origin` = `git@github.com:senolfeldmann/Muxsmith.git`, so the URL slug in
  every template link resolves.
- `origin/master` is the only remote branch, so `blob/master/docs/INSTALL.md`
  is correct; a `main`-default repo would have 404'd all four links.
- Repo `LICENSE` line 1 is `MIT License`, matching the tar.gz README's
  `LICENSE  MIT license` inventory line.
- The README rider's "drop the WIP banner in the same pass" has a referent:
  `README.md:5` carries the Work-in-progress blockquote.

---

## 3. Task quality

**Anchor-binding of INSTALL.md's behavioral claims.** Every non-trivial claim
walked to a design anchor:

| claim | anchor |
|---|---|
| per-machine install to `C:\Program Files\Muxsmith` | design 124-125 (WiX template, verified at the installed tag) |
| installer does not modify PATH | D82 §4 + design 1.2 (stock WiX PATH pseudo-feature verified inert) |
| `muxsmith.exe` beside the app | D82 §3 |
| macOS 11+, no Intel build | D78 + design 963-964 (`minimumSystemVersion: "11.0"`, arm64-only) |
| Sequoia blocks, Control-click gone pre-15 only | design line 51, Apple developer note `developer.apple.com/news/?id=saqachfa` (fetched 2026-07-22) |
| CLI at `Contents/MacOS/muxsmith` | D82 §3 |
| deb/rpm put both binaries in `/usr/bin` | D82 §3 |
| deb/rpm recommend mkvtoolnix | D80 |
| deb/rpm hard-depend on webkitgtk/gtk3 | design 150-153 (tauri-cli `interface/rust.rs` auto-injection) |
| the AppImage bundles them | design 154-158 (`linuxdeploy.rs`, webkit bundled into the image) |
| glibc floor from the 22.04 build base | design 843, D88 |
| `sha256sum -c SHA256SUMS` verification | D90 / design 1201-1206 |

No unanchored behavioral claim found. The HTML obsolescence comment cites the
ROADMAP trigger by name, not by `file:line` - the survivable-anchor form
`code-comment-line-citations-drift` prescribes.

**Register.** Sober, imperative, no marketing voice, no hedging, no AI-tell
phrasing. Per-OS sections are symmetric (artifact -> hurdle -> CLI). The
`Requirements` block in the tar.gz README uses plain-text underlining
appropriate to a `.txt` consumed in a terminal. Wording items that remain are
all in frozen design text, so they belong to the owner's rendered-surface pass
(section 5), never to a fix loop - section 11 makes content changes owner
changes, and the implementer correctly changed nothing.

**Report accuracy.** Every quantitative claim in `task-3-report.md` that I
re-ran reproduced: five empty transcription diffs, 3/7/8/4 structural counts,
four byte sizes, trailing newlines, typography absence, clean worktree, commit
unsigned (`%G?` = `N`) with the required trailer and no `git add -A`. The
worktree is clean at `c890b0f`; the fidelity harness left no untracked file
behind. The report's self-review is candid about the one on-disk incident
rather than eliding it.

---

## 4. Findings by severity

**BLOCKER: none. HIGH: none. MEDIUM: none.**

**LOW-1 - design cross-reference points to the wrong section (pre-existing, not
a T3 defect).** D75 (design lines 303-304) reads "(Apple citation in section 1)".
The Apple citation lives in **section 0**, line 51 (the brief-corrections
table). Section 1 (lines 58-278) contains no Apple/Gatekeeper/Sequoia/
quarantine/notarization reference. Control: the same grep over the whole
document does find the citation at line 51, so the section-1 absence is real
and not a dead pattern. This is `code-comment-line-citations-drift` territory
in the owner-approved design, outside T3's diff and outside T3's authority to
change. Recorded for the design's next amendment, not for a fix round.

**INFO-1 - brief boundary descriptor off by one line.** `task-3-brief.md`
Step 1 describes 4.1's content as running "to the final Fedora line". The
block's actual last line is the webkitgtk continuation
(`them; for the tar.gz install them via your package manager.`); "Fedora"
appears on the line above. The authoritative boundary is the closing fence, the
implementer used the fence, and the report describes the endpoint accurately
("through the closing tar.gz/webkitgtk line"). No consequence; noted so a
future brief writes fence-bounded, not prose-bounded, extraction limits.

**INFO-2 - fidelity harness method.** The report's harness used fixed line
ranges (`sed -n 'START,ENDp'`) with fence-line assertions at the expected line
numbers. `proc-wrapped-prose-quote-grep` prescribes structural anchors over
fixed ranges - but that entry is absent from `docs/process-conventions.yaml` at
the base commit `aec4cef` (verified: 0 hits), so it did not bind this
implementer, and the fence assertions convert the failure mode the entry warns
about (a silent phantom delta) into a loud assertion. My independent re-diff
used pure content anchors and reproduces byte-exactness, so the two methods
agree - which is itself the corroboration `proc-wrapped-prose-quote-grep`'s
third occurrence describes.

**INFO-3 - the `cp -i` incident is already ledgered.** The report's Concerns
section is recorded verbatim as occurrence 1 of
`proc-noninteractive-file-ops-in-agents` (Tier-2, `promoted_at: 3`,
`docs/process-conventions.yaml:578-593`), alongside the T4 and T5 instances of
the same day. Nothing outstanding; no second harvest owed.

**INFO-4 - diff package format.** `review-aec4cef..c890b0f.diff` is a wrapper
(header + commit list + `--stat` + a wider-context diff body), not raw
`git diff` output. Its changed lines are identical to
`git diff aec4cef c890b0f` (verified by comparing the `+`/`-` line sets). No
integrity problem; noted because a naive whole-file comparison against
`git diff` reports a false mismatch.

---

## 5. Owner-pass list (rendered-surface pass at plan close)

All frozen design text. None of these is an implementer defect; each is an
owner wording/content call under section 11. Ordered by user impact.

1. **`sha256sum` is not a Windows command.** `docs/INSTALL.md:6-8` and
   `.github/release/draft-body.md:21-22` give `sha256sum -c SHA256SUMS` as *the*
   verification step, in an all-OS intro and an all-OS release body. Windows
   users have neither `sha256sum` nor a `-c` equivalent out of the box
   (`Get-FileHash` / `certutil -hashfile` compare one file at a time). Consider
   a one-line PowerShell alternative in the Windows section.
2. **The webkitgtk bullet contradicts its own label.** `docs/INSTALL.md:80-83`:
   the bullet reads "**GUI only, deb/rpm/tar.gz:**" and its body then explains
   what the AppImage does. Either widen the label to all four Linux artifacts or
   move the AppImage clause out.
3. **`sudo ln -s ... /usr/local/bin/muxsmith` can fail on a clean Mac.**
   `docs/INSTALL.md:59`. `/usr/local/bin` is on the default macOS PATH
   (`/etc/paths`) but is not necessarily *present* on an Apple-Silicon machine
   without Homebrew (Apple-Silicon brew uses `/opt/homebrew/bin` -
   `core-89-homebrew-apple-silicon-path` records the split). If the directory is
   absent the `ln` errors out. `sudo mkdir -p /usr/local/bin && sudo ln -s ...`
   removes the failure mode. I could not test this from Linux; worth confirming
   on real hardware during R8, which already puts the owner in front of a Mac.
4. **User vs System `Path` not disambiguated.** `docs/INSTALL.md:32-33`: the
   Environment Variables dialog shows a `Path` in both panes. D82 §4 says "user
   PATH"; the instruction says only "select `Path`".
5. **"the one-time step per OS" (singular), `docs/INSTALL.md:6`.** Windows has
   two (SmartScreen, PATH) and macOS three. "the one-time steps" reads truer.
6. **"Every install ships two programs", `docs/INSTALL.md:10-11`.** True of the
   AppImage in the literal sense (D82 §3: `usr/bin/muxsmith` rides along) but
   D82 names the supported Linux CLI channels as deb, rpm and tar.gz only. A
   reader who buys "every install" may go looking for a CLI in the AppImage.
7. **draft-body continuation lines beginning with `|`.**
   `.github/release/draft-body.md:3-4`. In GFM these are lazy paragraph
   continuations (no delimiter row follows, so no table is formed) and should
   render as `Windows | macOS | Linux` inline. This is a rendered-surface
   assumption, and **R5 already puts the assembled body in front of the owner** -
   confirm it there rather than reasoning about it.
8. **Optional: carry D82 §4's macOS alternative.** "or add the `Contents/MacOS`
   dir to PATH" is in D82 but not in `docs/INSTALL.md`. Not required (D75's
   outline is satisfied by the `ln -s`), but it is the option that needs no
   `sudo`, which pairs with item 3.
9. **"glibc from Ubuntu 22.04 (2022) or newer"**,
   `packaging/linux-tarball-README.txt:22-24`. Distro-version-as-glibc-version
   is loose shorthand; naming the actual floor (glibc 2.35) alongside it is more
   useful to the one reader who hits it.

---

## HARVEST

- **A negative fire test is itself an absence-shaped result.** My first
  placeholder fire test used a pattern that did not match the file
  (`GUI screenshot` vs `one GUI screenshot`) and returned the *unchanged* count.
  A fire test is supposed to produce a change; when it produces "no change" the
  first suspect is the test, not the finding. The discipline generalizes:
  `proc-verification-step-must-be-falsifiable` tells you to break the thing and
  watch the check fire - it does not say what to do when the break itself is a
  no-op. Proposed sharpening of that entry: **a fire test that fails to fire is
  a defective fire test until the mutation is proven to have landed** (grep the
  mutated artifact for the change before reading the check's output). Candidate
  for a `proc-` entry occurrence rather than a new id.
- **Compound absence patterns need per-alternative liveness, not one control.**
  A single planted em-dash proves the pattern compiles and that its *first*
  alternative works; it says nothing about the other four. Running the control
  once per alternative (5 separate `grep -c` runs here) is the cheap complete
  form. Fits under `proc-verification-step-must-be-falsifiable` as a refinement:
  the control must cover the pattern's branch set, not the pattern's existence.
  Same shape as `proc-sweep-surface-completeness` ("a firing positive control
  proves a sweep PATTERN is valid, never that its SEARCH SURFACE is complete") -
  this is the third axis: not the surface, the pattern's own alternatives.
- **A count check is validated by its iteration domain, not by re-running the
  count.** `grep -c 'placeholder(1.0)' == 4` is satisfied by "deleted one, added
  one". Printing the four matching lines at base and at HEAD proved identity
  preservation, which is what D81's "no placeholder resolved" invariant actually
  asserts. Strengthens `proc-normative-count-recomputed`: recomputing from the
  enumeration means *reading* the enumeration, and where the count guards an
  identity invariant, comparing the enumerations across the two states.
  Similarly, `grep -c` counts lines while the invariant is about occurrences -
  the two agree here only because no line carries two, which itself had to be
  checked (`grep -o | wc -l`).
- **Verify a "the source did not change" premise with the comparator you then
  trust for the empty result.** The A1-non-interference claim was checked by
  running the same section-extract-and-diff over section 2, the region A1 *did*
  touch, and watching it fire. Cheap, and it converts "diff was empty" from an
  assertion into evidence. Generalizes to every review that reads ground truth
  from a copy other than the one the implementer used.
- **Cross-references inside an owner-approved design are claims too.** D75's
  "(Apple citation in section 1)" points at section 0.
  `code-comment-line-citations-drift` already covers "file:line in a durable
  artifact"; this is the same defect one abstraction up, a *section* pointer, and
  the entry's trigger ("you are typing file:line") does not fire on it. Proposed
  widening: the trigger is **any positional pointer into a durable artifact -
  line, section, table, step number** - since all of them drift and none of them
  survive a reorder. Candidate occurrence on this evidence.
- **Reviewer self-finding: I drafted this verdict's `INSTALL.md:NN` citations
  from reading position, not from measurement, and every one of the sixteen was
  off by roughly two lines** - a systematic bias, since I was counting from the
  design file's numbering rather than the created file's. Caught only by piping
  the committed blob through `cat -n` before shipping. A `file:line` in a verdict
  is the same kind of claim `code-comment-line-citations-drift` governs in a
  design, and a verdict is a durable artifact that later rounds cite back.
  Operational form for reviewers: **do not type a line number into a verdict that
  you have not read off a numbered dump of the exact artifact you are citing**,
  and when the artifact is an *extract* of a larger file, state which numbering
  you mean (this verdict now says "counted in the committed `docs/INSTALL.md`").
  Fits `code-comment-line-citations-drift` as a new occurrence in a new host
  class (review verdicts), not a new id.
- **Brief boundaries for transcription tasks should be structural, not
  descriptive.** "to the final Fedora line" is prose that happened to be off by
  one; "the content between the fence markers" cannot be. Recommend that future
  verbatim-transcription briefs state the block boundary as the fence pair only,
  and drop the prose gloss - a gloss that disagrees with the fence creates an
  ambiguity the implementer must silently adjudicate, which is a latitude clause
  by omission (`proc-latitude-clause-boundary`).
