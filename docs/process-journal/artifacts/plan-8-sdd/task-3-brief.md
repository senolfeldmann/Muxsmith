### Task 3: D75/D77/D79/D88 collateral - INSTALL.md, release-body templates, tar.gz README, README rider

**Stream B** (`.worktrees/plan8-b`). Read D75, D80, D82 (the documented PATH steps), D88, and design sections 4.1, 4.2, 4.3, 4.5 in full. Model tier: mid.

This is a transcription task with a fidelity duty, not an authoring task: design section 4 carries every file verbatim and section 11 makes content changes owner changes. The implementer transcribes, then PROVES the transcription (Step 6). Final wording rides the owner's rendered-surface pass at the plan close; structure and content are frozen now.

**Files:**
- Create: `docs/INSTALL.md` (design 4.1 - the content inside its outer ````markdown fence)
- Create: `.github/release/draft-body.md` (design 4.2 first block, including the trailing `---` line)
- Create: `.github/release/rehearsal-banner.md` (design 4.2 second block, including the trailing `---` line)
- Create: `packaging/linux-tarball-README.txt` (design 4.3 - plain text, no fence markers)
- Modify: `README.md` (the release-artifacts placeholder comment's text, design 4.5)

**Interfaces:**
- Consumes: nothing from other streams (all-new files + one comment edit).
- Produces: the `#windows`/`#macos`/`#linux` anchors D77's template links; the two template files Task 4's assemble job reads at run time; the tar.gz README Task 4's Linux leg packs (D88 layout).

- [ ] **Step 1: Create `docs/INSTALL.md`** - the exact content of design 4.1. Note the nesting: 4.1 is wrapped in a ````markdown fence in the design; the file content is what is INSIDE that fence (from `# Installing Muxsmith` to the final Fedora line), including the embedded HTML comment (the file names its own obsolescence condition) and the inner ```sh fence.

- [ ] **Step 2: Create the two release-body templates** under a new `.github/release/` directory - design 4.2, each block verbatim including its closing `---` horizontal rule (the composition order rehearsal-banner -> template -> generated notes depends on those rules as separators; design section 2 notes).

- [ ] **Step 3: Create `packaging/linux-tarball-README.txt`** - design 4.3 verbatim (new top-level `packaging/` directory; release-channel collateral is neither a Tauri artifact nor CI logic, D88).

- [ ] **Step 4: Edit the README placeholder comment** (design 4.5). Locate it by content, not line number (measured at README.md:99 on 2026-07-23; re-verify): `grep -n 'placeholder(1.0): release artifacts per OS' README.md`. Replace that comment with:

```markdown
<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
     docs/INSTALL.md, which already carries the per-OS unsigned-install
     steps; drop the WIP banner in the same pass -->
```

This is a rider edit, not a resolution: the comment stays a `placeholder(1.0)`, and the placeholder count stays 4:

```bash
grep -c 'placeholder(1.0)' README.md
# Expected: 4 (unchanged; fire-verify by deleting one placeholder comment
# in the working copy, seeing 3, restoring)
```

- [ ] **Step 5: Structural checks**

```bash
grep -E '^## ' docs/INSTALL.md
# Expected: exactly three headings - "## Windows", "## macOS", "## Linux" -
# GitHub derives the #windows/#macos/#linux anchors the draft-body links
# target from exactly these.
grep -c '^| `muxsmith-__VERSION__' .github/release/draft-body.md
# Expected: 7 (the artifact table rows; recomputed - matches D89's seven files)
grep -c '__VERSION__' .github/release/draft-body.md
# Expected: 8 (7 table rows + 1 in the heading line; the assemble job's sed
# replaces every occurrence)
```

- [ ] **Step 6: Transcription-fidelity proof** (the anti-truncation duty): extract each source block from the design file with sed (between its fence markers) into scratch files, diff each against the created file, and state in the task report that every diff was empty. For the README rider, diff the replaced comment block against 4.5's text. A non-empty diff is a defect in the transcription - fix and re-diff; never "improve" the design's text (content changes are owner changes, section 11).

- [ ] **Step 7: Typography scan** (absence check, fire-verified):

```bash
grep -rnP '\x{2014}|\x{2013}|\x{2026}|[\x{201C}\x{201D}\x{2018}\x{2019}]|\x{00A0}' docs/INSTALL.md .github/release/draft-body.md .github/release/rehearsal-banner.md packaging/linux-tarball-README.txt
# Expected: no output. Fire-verify first: plant an em-dash in a scratch copy,
# run the same grep against it, see the hit, discard the scratch copy.
```

- [ ] **Step 8: Commit**

```bash
git add docs/INSTALL.md .github/release/draft-body.md .github/release/rehearsal-banner.md packaging/linux-tarball-README.txt README.md
git -c commit.gpgsign=false commit -m "release: INSTALL.md + draft-body/rehearsal-banner templates + tar.gz README + README placeholder rider (D75/D77/D79/D88, design section 4 verbatim)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

