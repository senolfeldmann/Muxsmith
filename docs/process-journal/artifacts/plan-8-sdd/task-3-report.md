# Task 3 report: D75/D77/D79/D88 collateral - INSTALL.md, release-body templates, tar.gz README, README rider

Stream B, worktree `.worktrees/plan8-b` (branch `plan8-b`). Status: **DONE**.

Commit: **c890b0f** (`c890b0f9372517d7a716390fb76101b66f94e24a`), unsigned
(`%G?` = `N`), trailer present, 5 files changed / 144 insertions / 1 deletion.

## Reading done before edits

All from the worktree copy of the design,
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`:

- `implementer-preamble.md` (Global Constraints) - read in full.
- `task-3-brief.md` - read in full.
- D75 (lines 278-337), D80 (532-570), D82 (592-713), D88 (1078-1135) - in full.
- Design sections 4.1 (1576-1665), 4.2 (1667-1707), 4.3 (1709-1739),
  4.5 (1766-1775) - in full. Section 4.4 (BUILDING.md rider) was read but is
  **not** this task's file (it belongs to another stream); untouched here.

No fork encountered. Every content decision was carried verbatim by design
section 4; nothing required a NEEDS_CONTEXT escalation. Cross-stream files
referenced elsewhere in the plan are absent from this worktree by
construction - expected, not a defect.

Pre-transcription byte survey of the source range (design lines 1571-1780):
`grep -nP '[ \t]+$|\t'` and `grep -nP '[^\x00-\x7F]'` both returned nothing,
so the blocks carry no trailing whitespace, no tabs and no non-ASCII. This
was established before writing, so a transcription diff could not be
"explained away" by invisible characters later.

## Steps executed

### Step 1: `docs/INSTALL.md` (design 4.1)

Created from the content INSIDE the outer ` ````markdown ` fence (design lines
1579-1661): `# Installing Muxsmith` heading through the closing tar.gz/webkitgtk
line, including the embedded HTML obsolescence comment (D75's "the file names
its own obsolescence condition") and the inner ```` ```sh ```` fence carrying the
macOS `ln -s` command. 3624 bytes, terminating newline present.

### Step 2: `.github/release/draft-body.md` + `.github/release/rehearsal-banner.md` (design 4.2)

New directory `.github/release/`. Both blocks transcribed verbatim including
the trailing `---` horizontal rule, which is load-bearing as the separator in
the rehearsal-banner -> template -> generated-notes composition order (design
section 2). 1177 and 231 bytes; both end with a newline after the `---`.

### Step 3: `packaging/linux-tarball-README.txt` (design 4.3)

New top-level `packaging/` directory (D88: release-channel collateral is neither
a Tauri artifact under `src-tauri/` nor CI logic under `.github/`). Plain text,
no fence markers, 1088 bytes, terminating newline present.

### Step 4: README placeholder rider (design 4.5)

Located by content, not line number, as the brief directs:

```
$ grep -n 'placeholder(1.0): release artifacts per OS' README.md
99:<!-- placeholder(1.0): release artifacts per OS (msi/dmg/deb/rpm/AppImage) once the packaging pipeline lands -->
```

Line 99 confirmed (matches the brief's 2026-07-23 measurement). Replaced with
design 4.5's four-line comment. The `git diff` is exactly one line removed,
four added, nothing else in the file:

```
-<!-- placeholder(1.0): release artifacts per OS (msi/dmg/deb/rpm/AppImage) once the packaging pipeline lands -->
+<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
+     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
+     docs/INSTALL.md, which already carries the per-OS unsigned-install
+     steps; drop the WIP banner in the same pass -->
```

Rider, not resolution: the comment remains a `placeholder(1.0)` and the count
stays 4.

**Placeholder-count check, fire-verified** (break-observe-restore on the working
copy, with a byte-exact backup taken first):

```
count (post-rider state):                    4
count (GUI-screenshot placeholder deleted):  3   <- check fired
count (restored):                            4
diff backup vs restored README.md:           empty
```

Execution note: the restore `cp` first tripped the interactive `cp -i` alias in
the profile-initialized shell and blocked until the tool timeout, leaving the
README in the mutated 3-placeholder state. It was restored with an explicit
`/usr/bin/cp -f` and the restoration verified two ways - `grep -c` back to 4
with the screenshot placeholder present again, and an empty `diff` against the
pre-mutation backup. `git diff` after restoration showed only the intended
rider hunk. No content was lost; recording the incident because the mutated
state existed on disk for the duration of the timeout.

### Step 5: structural checks (against the committed tree)

```
$ git grep -cE '^## ' HEAD -- docs/INSTALL.md
3          # exactly "## Windows", "## macOS", "## Linux"
$ git grep -c '^| `muxsmith-__VERSION__' HEAD -- .github/release/draft-body.md
7          # D89's seven files
$ git grep -c '__VERSION__' HEAD -- .github/release/draft-body.md
8          # 7 table rows + the heading line
$ git grep -c 'placeholder(1.0)' HEAD -- README.md
4          # unchanged
```

The heading grep printed exactly the three expected headings and nothing else,
so GitHub derives precisely the `#windows` / `#macos` / `#linux` anchors the
draft-body links target. All four are positive checks (a malformed pattern would
yield 0, not the expected value), so each is self-evidencing.

### Step 6: transcription-fidelity proof

Harness: `scratchpad/fidelity.sh`. For each block it (a) asserts the opening and
closing fence lines at the expected line numbers - so a design-file line shift
cannot silently produce a wrong extraction - then (b) `sed -n 'START,ENDp'`
extracts the block into a scratch file and (c) `diff -u` compares it with the
created file.

| block | extracted range | diff |
|---|---|---|
| 4.1 -> `docs/INSTALL.md` | 1579-1661 (fences 1578/1662) | **empty** |
| 4.2 first -> `.github/release/draft-body.md` | 1672-1695 (fences 1671/1696) | **empty** |
| 4.2 second -> `.github/release/rehearsal-banner.md` | 1701-1706 (fences 1700/1707) | **empty** |
| 4.3 -> `packaging/linux-tarball-README.txt` | 1712-1738 (fences 1711/1739) | **empty** |
| 4.5 -> README rider comment block | 1771-1774 (fences 1770/1775) | **empty** |

**Every one of the five diffs was empty.** All five files were authored by hand
from the design text and only then diffed against an independent sed extraction,
so the comparison is a genuine transcription check, not a tautology (extracting
with sed and diffing against the same extraction would prove nothing). Re-run
against the committed state after the commit: still five empty diffs.

An empty diff is an absence-shaped result, so the harness itself was
fire-verified rather than trusted:

```
fire test A (anti-truncation): docs/INSTALL.md truncated to 70 lines
  -> "Files ... differ"                        <- fired
fire test B (single word):     "unsigned build" -> "unsigned builds"
  -> 27c27 < for an unsigned build. / > for an unsigned builds.   <- fired
control (unmutated file)
  -> diff empty
```

Both mutation classes the check exists to catch were observed firing, with the
control confirming the empty result is not the harness silently failing.

### Step 7: typography scan (absence check, fire-verified)

Fire-verification first, on a scratch file built with `printf` escapes so it
carried one line per glyph class (U+2014, U+2013, U+2026, the four curly quotes
U+201C/201D/2018/2019, U+00A0). The grep returned all five lines - one hit per
class, line numbers 1 through 5 - so every alternative in the pattern is live,
not just the first. (Described rather than quoted verbatim: reproducing the
output here would plant the banned glyphs in this report.) Scratch file
discarded. The real scan over the four new files then produced no output
(exit 1):

```
$ grep -rnP '...' docs/INSTALL.md .github/release/draft-body.md \
    .github/release/rehearsal-banner.md packaging/linux-tarball-README.txt
(no output, exit 1)
```

The working files were byte-identical to the committed ones at scan time
(`git status --short` clean), so the result holds for the committed text.

### Step 8: commit

Explicit staging, five named files, no `git add -A`:

```
$ git -C <worktree> add docs/INSTALL.md .github/release/draft-body.md \
    .github/release/rehearsal-banner.md packaging/linux-tarball-README.txt README.md
$ git -C <worktree> status --short
A  .github/release/draft-body.md
A  .github/release/rehearsal-banner.md
M  README.md
A  docs/INSTALL.md
A  packaging/linux-tarball-README.txt
```

```
[plan8-b c890b0f] release: INSTALL.md + draft-body/rehearsal-banner templates + tar.gz README + README placeholder rider (D75/D77/D79/D88, design section 4 verbatim)
 5 files changed, 144 insertions(+), 1 deletion(-)
 create mode 100644 .github/release/draft-body.md
 create mode 100644 .github/release/rehearsal-banner.md
 create mode 100644 docs/INSTALL.md
 create mode 100644 packaging/linux-tarball-README.txt
```

Post-commit `git status --short`: empty (clean).

## Self-review

- **Diff scope**: the commit touches exactly the five briefed files. `ci.yml`
  never opened for writing (read only, to check whether any gate part globs the
  new paths). No cross-stream file created or modified.
- **Produced interfaces, verified**: the `#windows` / `#macos` / `#linux`
  anchors exist (Step 5 heading grep, exactly three); both template files exist
  at the paths Task 4's assemble job reads; the tar.gz README exists at the path
  D88's Linux leg packs.
- **Two load-bearing claims inside the frozen text, checked against the repo**
  rather than assumed: the URL slug `senolfeldmann/Muxsmith` matches
  `origin` (`git@github.com:senolfeldmann/Muxsmith.git`), and the templates'
  `blob/master/docs/INSTALL.md` matches the actual branch (`remotes/origin/master`
  is the only remote branch - a `main`-default repo would have made every
  template link 404). Both correct as written; no edit needed. Flagged here
  because a mismatch would have been a design-text defect, i.e. an owner change,
  never an implementer fix.
- **Design text not "improved"**: no wording, punctuation or line-wrap change
  anywhere - proven by the five empty diffs, not asserted. Section 11 makes
  content changes owner changes; the owner's rendered-surface pass at plan close
  owns final wording.
- **Placeholder status preserved**: still a `placeholder(1.0)`, count still 4;
  no `v*` tag, no release touched, no placeholder resolved.
- **No dependencies**: no cargo or npm manifest touched; the four new files are
  static documentation/collateral.
- **Nine-part gate**: not run, and not a step of this brief (it is the
  controller's pre-push / post-merge duty). Verified as inert for this change
  rather than assumed: `eslint .` is scoped to `**/*.vue` / TS-JS by
  `eslint.config.js` and never sees markdown or `.txt`; ci.yml's only
  "intra-doc link" concern is the rustdoc leg (`RUSTDOCFLAGS=-D warnings`), not
  a markdown link checker; no gate part reads `docs/*.md`,
  `.github/release/*`, or `packaging/*`. The change adds no compiled or
  bundled input.
- **Trailing newlines**: all four new files end with exactly one `\n` (verified
  via `tail -c 1 | od -c`); the diffs would have flagged a missing one.
- **Session-relocation ban**: no `EnterWorktree` / `ExitWorktree` or equivalent
  called at any point. Worktree treated as a plain directory; every git command
  ran as pure git via `git -C /home/senol/Git/Muxsmith/.worktrees/plan8-b` (no
  non-git segment chained into a git compound), other commands via absolute-path
  `cd` inside the Bash tool. No sibling worktree and no main tree working file
  touched; the report path `.superpowers/` is gitignored and untracked in the
  main tree, so writing it changes no git state there.
- **Foreground only**: every command ran synchronously; no `run_in_background`
  anywhere.
- **Commit trailer/signing**: unsigned (`-c commit.gpgsign=false`, `%G?` = `N`)
  with `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.
- **gh usage**: none in this task; no `gh-log.md` entry required. No push made
  (a controller action).

## Concerns

One, procedural and already resolved, recorded because it happened on disk:
the Step 4 fire-verification's restore `cp` hit the interactive `cp -i` alias
and blocked to the tool timeout, so `README.md` sat in the mutated
3-placeholder state until it was restored with `/usr/bin/cp -f`. Restoration
verified by both `grep -c` (back to 4, screenshot placeholder present) and an
empty `diff` against the pre-mutation backup, and the committed diff contains
only the rider hunk. Nothing outstanding; the note is here so nobody
re-discovers the alias the same way. Generalizable: destructive-then-restore
fire tests in this environment should use `/usr/bin/cp -f` or a scratch copy,
never bare `cp`.

No design fork, no content ambiguity, no cross-stream dependency. All named
checks green, all five transcription diffs empty, both absence-shaped checks
(placeholder count, typography scan) plus the fidelity harness itself
fire-verified.
