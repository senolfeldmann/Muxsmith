# Task 4 report: D83/D84/D85/D77/D79/D88/D89/D90 - release.yml

Stream C, worktree `.worktrees/plan8-c` (branch `plan8-c`).
Status: **DONE** (final; see "Completion after amendment A1" at the end).

Final commit: `fe46424d4cf17faf24fdfd0c5bcb902b347d5221` (unsigned, `%G?` = `N`,
repo trailer present, one file, tree clean). This amends the pre-A1
`9449447...`; the branch carries exactly one commit, the one the brief
prescribes.

The body below is the original run, kept as written (its status line read
**NEEDS_CONTEXT** and its commit was `9449447...`); the fork it raised was
routed and closed. Read the final section for the resolution and the re-run
evidence.

## Reading done before edits

- `implementer-preamble.md` (Global Constraints) - in full.
- `task-4-brief.md` - in full.
- Design `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
  (worktree copy): section 1.3 (runner labels, lines 167-191), section 1.4
  (action pins, 193-209), section 1.5 (release-ops mechanics, 211-238), D77
  (370-427), D78 (429-474), D79 (477-529), D83 (715-782), D84 (785-828), D85
  (832-918), D88 (1078-1134), D89 (1137-1194), D90 (1197-1232), **section 2 in
  full (1235-1471)**, section 6 (1798-1811), section 7 (1814-1844), section 8
  (1848-1932), section 9 (1936-1973), section 11 (1988-2025).
- `.github/workflows/ci.yml` (house checkout pin, negative-space positive
  control), `mise.toml`, `package.json`, `rust-toolchain.toml`, `Cargo.toml`.

Not modified: `ci.yml` (D83), or any file other than the one created workflow.
No session-relocation tool called; all paths absolute or an explicit
`cd`-in-command into the worktree; all runs foreground.

## The fork (NEEDS_CONTEXT): design section 2's YAML does not parse

### The defect

Design line 1334 = `release.yml` line 94, transcribed verbatim:

```
      - name: Read pinned node version from mise.toml (D85: no mise in release legs)
                                                          ^ col 59
```

A plain (unquoted) YAML scalar may not contain the sequence `": "` (colon +
space). This is a core YAML rule in both 1.1 and 1.2, not a strict-mode nit.
Two independent parsers reject the committed file at exactly that position:

```
$ python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('yaml-ok')"
yaml.scanner.ScannerError: mapping values are not allowed here
  in ".github/workflows/release.yml", line 94, column 59
        (PyYAML 6.0.3 - the version the brief's step 3 names)

$ ruby -ryaml -e '... YAML.safe_load(File.read(".github/workflows/release.yml")) ...'
ruby: REJECTS -> Psych::SyntaxError: (<unknown>): mapping values are not allowed in this context at line 94 column 59
        (Psych/libyaml, second independent implementation)
```

Scope of the defect, measured: exactly one line in the file has this shape.

```
$ grep -nP '^\s*(-\s+)?[a-z_-]+:\s+[^"'"'"'|>&*\[{].*:\s' .github/workflows/release.yml
94:      - name: Read pinned node version from mise.toml (D85: no mise in release legs)
```

**Inference, marked as such:** GitHub Actions will also fail to load this
workflow, because the violation is at the YAML-spec level rather than in a
lenient extension. I cannot execute the workflow (rehearsal is Task 6), so this
is a strong inference from the spec rule plus two conformant parsers, **not** an
observed GitHub error message.

### Why this is not a keyboard fix

Two normative steps of my own brief contradict each other on this line:

- **Step 1 + Step 2 + section 11**: transcribe section 2 verbatim; "A non-empty
  diff is a defect; the design text wins (section 11)". Section 11 additionally
  binds section 2's YAML "verbatim" including comment forms.
- **Step 3**: the parse check must print `yaml-ok`.

Both cannot hold simultaneously. Step 3's green state is unreachable against
the mandated text - the exact shape `proc-check-green-state-reachable` names.
Resolving it means amending owner-approved design text (or recording a
sanctioned deviation outside it, the shape the ci.yml rider precedent used),
which is a controller/owner action, not an implementer call
(`proc-latitude-clause-boundary`).

### Options and costs

Both repair candidates were verified in scratch copies, not in the committed
file.

**Option A - quote the value** (recommended)

```yaml
      - name: "Read pinned node version from mise.toml (D85: no mise in release legs)"
```

- Verified: parses under PyYAML **and** Psych.
- The step-name *string* stays byte-identical; the Actions UI label, the D85
  pointer and its punctuation are all preserved exactly. Only YAML quoting
  changes.
- Typography: adds straight double quotes, which the house rule permits (only
  curly quotes are banned).
- No decision content changes anywhere, so no D-entry is needed.
- Cost: the workflow line diverges from the design fence, so **the design fence
  must be amended in the same change** or step 2's diff proof stops being empty.
  That is the sweep duty the change opens, and it is one line on each side.

**Option B - replace the colon inside the parenthetical**

```yaml
      - name: Read pinned node version from mise.toml (D85 - no mise in release legs)
```

- Verified: parses.
- Keeps the file quote-free.
- Cost: changes the step-name text itself, i.e. the human-readable log label and
  the comment form that section 11 pins as verbatim. Same one-line design
  amendment duty.

**Option C - drop the parenthetical, move the pointer into a `#` comment above
the step**

- Cost: largest divergence (changes the step's line count, so the fence diff
  grows beyond one line), and the D85 pointer disappears from the Actions UI
  label where it is most useful.

**Recommendation: Option A, applied to design section 2's fence and to
`release.yml` in one change**, then re-run steps 2 and 3 (fence diff empty
again, `yaml-ok`). It is the minimal edit that preserves every rendered string
and every decision pointer, and it keeps the transcription-fidelity proof
meaningful. If the controller wants the file quote-free, Option B is second, at
the cost of altering the label text.

## Steps executed

### Step 1: create `.github/workflows/release.yml`

Produced by **extracting** the design's section-2 fence, not by retyping it -
extraction is the strictly more faithful method for a verbatim contract, and it
removes transcription risk entirely. 222 lines, ends with a newline.

Fence boundaries established programmatically rather than assumed:

```
$ grep -n '^```' <design> | awk -F: '$1>1230 && $1<1500'
1240:```yaml
1463:```
1482:```json
```

Content is therefore design lines 1241-1462. Pre-write hygiene scan of the
extracted text: 0 lines with trailing whitespace or tabs, 0 non-ASCII
characters, 0 banned typography glyphs (em/en dash, curly quotes, ellipsis,
NBSP, Unicode minus, figure dash, horizontal bar).

### Step 2: transcription-fidelity proof

Three-part, so the empty diff is evidence rather than a tautology.

```
$ sed -n '1241,1462p' <design> > fence-check.yml && diff fence-check.yml .github/workflows/release.yml
DIFF-A-EMPTY (sed line-range extraction == committed file)
```

Independent second extraction, driven by the fence delimiters instead of
hardcoded line numbers (this is what proves the line range above is the right
region, not merely self-consistent):

```
header line 1235, open fence 1240, close fence 1463, content lines 1241..1462
DIFF-B-EMPTY (independent fence-delimited extraction == committed file)
```

Fire-verification of the diff check itself (the passing result is an absence,
so the method was made to produce output once):

```
$ sed -i 's/^name: release$/name: releasf/' perturbed.yml
$ diff fence-sed.yml perturbed.yml
1c1
< name: release
---
> name: releasf
diff exit: 1 (1 = difference detected -> check fires)
```

Restored, re-diffed: empty. Post-commit re-check against the committed blob:

```
$ git show HEAD:.github/workflows/release.yml | diff - fence-sed.yml
committed blob == design fence, byte-identical
```

**The diff was empty.**

### Step 3: parse check - RED (the fork)

Full evidence in the fork section above. `yaml-ok` was not reached, and cannot
be reached against the mandated text.

### Step 4: fire-tests G4-G5 against the committed workflow text

Both script bodies were **extracted from `.github/workflows/release.yml`** with
awk (indentation-driven slice of the step's `run: |` block, dedented by 10),
never retyped from the design. G4's extracted body is 13 lines and contains no
`${{ }}` expression (verified by grep), so it runs wholesale.

**G4** - updater-absence step, scratch tree with a fabricated
`target/release/bundle/`, all foreground:

```
----- G4 case 1: clean tree, 1 bundle file -----
updater-artifact check: 0 hits across 1 bundle output files
exit: 0
----- G4 case 2: planted app.msi.sig -----
target/release/bundle/msi/app.msi.sig
::error::1 updater artifact(s) found - D76 bans updater output
exit: 1
----- G4 case 3a: bundle dir exists but empty -----
::error::positive control failed: bundle output dir is empty
exit: 1
----- G4 case 3b: no bundle dir at all -----
::error::positive control failed: no bundle output dir
exit: 1
----- G4 case 1 re-run (green reachable after the red states) -----
updater-artifact check: 0 hits across 1 bundle output files
exit: 0
```

Matches design section 8's stated outputs exactly, including the summary line's
wording. The design names three cases; I split the positive control into 3a
(empty dir) and 3b (missing dir) because the step has two distinct guards for
them, and both fire with their own message.

**G5** - `pick()`, extracted verbatim (lines 5-15 of the dedented rename-step
body, no `${{ }}` inside the function) into a scratch script with only
`set -euo pipefail` prepended, invoked through the **call-site shape**
`msi="$(pick target/release/bundle/msi/*.msi)"` so the command-substitution
behaviour is exercised too:

```
----- G5 case 1: exactly one match -----
pick: target/release/bundle/msi/app.msi          <- stderr
STDOUT-CAPTURED: target/release/bundle/msi/app.msi
exit: 0
----- G5 case 2: two matches -----
::error::expected exactly one artifact, got: target/release/bundle/msi/app.msi target/release/bundle/msi/other.msi
exit: 1
----- G5 case 3: zero matches (unexpanded glob reaches pick, as at the call site) -----
::error::expected exactly one artifact, got: target/release/bundle/msi/*.msi
exit: 1
----- G5 case 1 re-run (green reachable after the red states) -----
pick: target/release/bundle/msi/app.msi
STDOUT-CAPTURED: target/release/bundle/msi/app.msi
exit: 0
```

Matches design section 8 exactly: log on stderr, path on stdout, exit 0 on one
match; the `::error::` line and exit 1 on zero and on two. Two design
assumptions were confirmed empirically as a side effect: the zero-match case
reaches `pick` as an unexpanded glob (bash default, no nullglob) and is caught
by the `! -e` arm, and `exit 1` inside the command substitution does propagate
to the calling step under `set -e`.

Six observed outputs recorded, plus the two green re-runs.

### Step 5: negative-space check

```
$ grep -n 'tauri-action\|softprops\|rust-cache\|mise-action\|Swatinem\|concurrency' .github/workflows/release.yml
grep exit: 1 (1 = no match, as expected)

$ grep -n '...same pattern...' .github/workflows/ci.yml
40:      - uses: Swatinem/rust-cache@c19371144df3bb44fab255c43d04cbc2ab54d1c4 # v2.9.1
82:      - uses: jdx/mise-action@e6a8b3978addb5a52f2b4cd9d91eafa7f0ab959d # v4.2.0
grep exit: 0 (0 = pattern demonstrably fires)
```

Positive control fires on exactly the two lines the brief predicted. No banned
shape and no concurrency group in release.yml (section 6's deliberate omission).

### Step 6: pin conformance (recount)

```
$ grep -c 'uses:' .github/workflows/release.yml
7
34:      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0    (guard)
85:      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0    (leg)
98:      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
101:      - uses: pnpm/action-setup@0ebf47130e4866e96fce0953f49152a61190b271 # v6.0.9
177:      - uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
190:      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0    (assemble)
191:      - uses: actions/download-artifact@3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c # v8.0.1

$ grep -n 'uses:' .github/workflows/release.yml | grep -v '@[0-9a-f]\{40\} # v'
grep exit: 1 (1 = all pinned + version-commented)
```

7 matches the brief's recount, and the enumeration above is the recount (guard
checkout, leg checkout, setup-node, pnpm/action-setup, upload-artifact, assemble
checkout, download-artifact). Every SHA measured at 40 hex characters. The only
other `@` occurrences in the file are shell array expansions (`"$@"`, `[@]`,
`extra_args[@]`), not action refs - checked, not assumed.

Cross-check against the pin sources: each of the four new SHAs appears twice in
the design (section 1.4 table + section 2), and the checkout SHA appears in
`ci.yml` twice, confirming it is the reused house pin rather than a new one.

Fire-verification of the second grep:

```
$ sed -i '98s|...@8207627... # v7.0.0|...@v7|' .github/workflows/release.yml
$ grep -n 'uses:' ... | grep -v '@[0-9a-f]\{40\} # v'
98:      - uses: actions/setup-node@v7
grep exit: 0 (0 = check fires)
```

Restored; the fence diff was empty again and the grep clean, before staging.

### Step 7: commit

Brief's message and trailer used verbatim, `commit.gpgsign=false`, single file
staged explicitly (`git add .github/workflows/release.yml`; never `git add -A`),
git commands not chained with any non-git segment.

```
944944717927935b42164f754aa27c7e011019a1
release: release.yml - guard (version sync + ci-gate green), four native bundle legs, draft-release assemble with SHA256SUMS (D77/D79/D83-D85/D88-D90); G4/G5 fire-verified
Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
signed: N
```

Committing under a red step 3 was deliberate: the committed text is exactly what
section 11 mandates, the defect is in the design text rather than in the
transcription, and the branch is isolated. The routed one-line repair lands as a
separate commit. The NEEDS_CONTEXT status is the merge gate.

## Self-review

Structural conformance against the brief's two enumerations, read back from a
parse of the quoted variant (`cand-a`, which differs from the committed file
only in the quoting of that one step name - the string value is byte-identical,
so the structure below is the committed structure):

- Name `release`; triggers exactly `push` (`tags: ['v*']`) + `workflow_dispatch`
  with the single input `rehearse-draft-release`, `type: boolean`,
  `default: false`.
- Workflow-level `permissions: {contents: read}`; both policy comment blocks
  present (least-privilege, pinning policy with the two recorded D85
  deviations).
- Jobs `guard`, `bundle`, `assemble`. `guard` on `ubuntu-22.04` with
  `{contents: read, actions: read}`; `bundle` `needs: guard`; `assemble`
  `needs: [guard, bundle]` with `{contents: write}` - `contents: write` on
  exactly one job.
- Guard poll cadence as written: `for i in $(seq 1 90)` with `sleep 30` and the
  45-minute comment (30 s x 90 = 45 min, recomputed).
- Matrix `fail-fast: false`, four `include` entries, recounted against the
  brief's table - all four leg ids, runners and `--bundles` values match
  character-for-character:
  `windows-x86_64`/`windows-2025`/`msi`, `windows-arm64`/`windows-11-arm`/`msi`,
  `macos-arm64`/`macos-15`/`dmg`, `linux-x86_64`/`ubuntu-22.04`/`deb,rpm,appimage`.
- 12 leg steps, in the brief's order: checkout, rustup, Linux apt deps
  (`if: runner.os == 'Linux'`), mise.toml node parse, setup-node,
  pnpm/action-setup, `pnpm install --frozen-lockfile`, CLI build + sidecar
  staging, `tauri build`, updater-absence assert, rename + tar.gz pack,
  upload-artifact.
- upload-artifact `with`: `name: muxsmith-${{ matrix.leg }}`,
  `path: release-assets/*`, `retention-days: 7`, `if-no-files-found: error`.
- Assemble steps: checkout, download-artifact
  (`pattern: muxsmith-*`, `path: assets`, `merge-multiple: true`), SHA256SUMS,
  then the draft step gated
  `if: github.ref_type == 'tag' || inputs.rehearse-draft-release`.
- 8-asset name set: the rename step produces
  `muxsmith-$version-$leg.{msi,dmg,deb,rpm,AppImage}` and
  `muxsmith-$version-$leg.tar.gz` with the version-named staging directory, and
  the assemble job generates `SHA256SUMS` after the rename (D90's ordering).
  Nothing in the file publishes, edits or un-drafts a release; `--verify-tag`
  appears only on the tag arm; the rehearsal name is `rehearsal-${GITHUB_RUN_ID}`;
  the body order is banner -> template -> `generate-notes` API output.

Two load-bearing shell expressions in the transcribed text, run against the
real local files rather than trusted:

```
node parse    -> 26.5.0   (mise.toml says: node = "26.5.0")
version parse -> 0.1.0    (Cargo.toml [workspace.package] version = "0.1.0")
```

Run-time inputs from sibling streams, confirmed absent here by construction and
referenced exactly as the design names them (not created by me):
`scripts/check-version-sync.sh`, `src-tauri/tauri.bundle.conf.json`,
`.github/release/draft-body.md`, `.github/release/rehearsal-banner.md`,
`packaging/linux-tarball-README.txt`. Present and used as-is: `mise.toml`,
`package.json`, `rust-toolchain.toml`, `Cargo.toml`, `LICENSE`.

`ci.yml` untouched: the only staged path is the new workflow, and the tree is
clean after the commit.

## Concerns

1. **The fork above is blocking for merge.** The committed workflow does not
   load. One line, routed decision, recommendation Option A.

2. **`bundle` carries no explicit `permissions:` block** (observation, no change
   made). D83's "Interface changes" line reads "legs: `contents: read`", and
   section 2's YAML gives the legs no per-job block - the workflow-level
   `permissions: contents: read` supplies exactly that value, so the effective
   behaviour matches D83's intent. I did not add a block: section 2 is the
   verbatim source and section 11 pins per-job permissions "as written". Flagged
   in case the controller reads D83 as requiring the block explicitly, in which
   case it is a second (behaviour-neutral) design/section-2 mismatch to route
   together with the fork.

3. **The nine-part gate was not run.** Per the preamble it is a pre-push /
   post-merge controller action; this change adds one CI workflow file and
   touches no Rust or TypeScript path. Flagged so the omission is visible rather
   than assumed.

4. **Steps 2 and 3 must both be re-run after the fork's repair**, and the
   repair's design-side amendment is what keeps step 2 meaningful. Recorded here
   because the repair commit will not be mine.

---

## Completion after amendment A1 (controller resume)

**Ruling received:** the fork was routed, the controller ruled Option A (my
recommendation), the resumed design author applied it as amendment **A1**
(commit `d21a19f8984c645996b42a21052be9e22a524ad4`, "plan-8: design amendment A1
- quote the unparseable section-2 step name"), and the resumed design reviewer
approved the delta. Concern 2 (implicit leg permissions) is
controller-adjudicated: **leave as written**, D83's interface line is satisfied
by the workflow-level default. No change made for it.

The amended design lives on master; this worktree's design copy predates it
(merge-base `aec4cef`). The amended section was therefore read from the main
tree read-only at
`/home/senol/Git/Muxsmith/docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`;
all edits stayed in `.worktrees/plan8-c`.

### The borrowed claim, verified before acting on it

The resume described A1 as one line, string byte-identical. I did not take that
on trust: the amended fence was extracted from the main-tree copy and diffed
against my committed (pre-A1) file.

```
$ python3 <fence-delimiter-driven extraction from the amended main-tree copy>
header 1236, open fence 1241, close fence 1464, content 1242..1463 (222 lines)
$ sed -n '1242,1463p' <amended design> > fence-amended-sed.yml
$ diff fence-amended.yml fence-amended-sed.yml
amended fence: both extraction methods agree

$ diff fence-sed.yml fence-amended.yml        # pre-A1 fence vs amended fence
94c94
<       - name: Read pinned node version from mise.toml (D85: no mise in release legs)
---
>       - name: "Read pinned node version from mise.toml (D85: no mise in release legs)"
```

Exactly one line differs, exactly the Option A quoting, 222 lines before and
after. Nothing else rides along, so no further transcription was owed. Note the
fence boundaries shifted by one line (1241-1462 -> 1242-1463): A1 added a line
in the prose above the fence, which is why the boundaries were re-derived from
the delimiters instead of reused.

### Application

Applied by copying the amended fence over the file (same method as the original
transcription, so the result is byte-identical to the contract by construction
rather than by retyping). Post-application `sed -n '94p'`:

```
      - name: "Read pinned node version from mise.toml (D85: no mise in release legs)"
```

### Step 2 re-run: fidelity proof against the AMENDED fence

```
$ diff fence-amended.yml .github/workflows/release.yml
DIFF-A-EMPTY (fence-delimited extraction == committed file)
$ diff fence-amended-sed.yml .github/workflows/release.yml
DIFF-B-EMPTY (independent sed line-range extraction == committed file)
```

**The diff was empty**, against both independent extractions of the amended
fence. The diff method's ability to fire was demonstrated in the original run
and again incidentally here: the pre-application diff above printed the line-94
delta.

### Step 3 re-run: parse check - GREEN

```
$ python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('yaml-ok')"
yaml-ok
$ ruby -ryaml -e '... YAML.safe_load ...'
ruby: parses
```

The red-green pair for this check is now on the record end to end: the pre-A1
text was observed red under both parsers at line 94 col 59 (original run), the
amended text is green under both. `proc-check-green-state-reachable` and
`proc-verification-step-must-be-falsifiable` are discharged by that observed
pair, not by a claim.

Semantic no-op confirmed rather than assumed - the loaded step name is
byte-identical to the pre-A1 string, so the Actions UI label and the D85 pointer
are unchanged:

```
loaded  : 'Read pinned node version from mise.toml (D85: no mise in release legs)'
expected: 'Read pinned node version from mise.toml (D85: no mise in release legs)'
identical: True
```

### Steps conditioned on the changed file, all re-run

The file changed, so every check that reads it was re-run rather than inherited.

**Step 4 (G4/G5).** Extraction anchors re-verified against the final text (the
`run: |` keys still at lines 121 and 137, the step names still at 119 and 135) -
checked, not assumed from the unchanged line count. The re-extracted bodies
diffed clean against the original extractions, proving A1 touched neither
script:

```
G4 body: unchanged by A1
G5 pick(): unchanged by A1
```

Both fire-tests re-run foreground against bodies extracted from the final
committed text; all ten observed outputs are identical to the original run and
to design section 8:

```
G4 case 1 (1 bundle file)     -> updater-artifact check: 0 hits across 1 bundle output files | exit 0
G4 case 2 (planted .sig)      -> ::error::1 updater artifact(s) found - D76 bans updater output | exit 1
G4 case 3a (empty bundle dir) -> ::error::positive control failed: bundle output dir is empty | exit 1
G4 case 3b (no bundle dir)    -> ::error::positive control failed: no bundle output dir | exit 1
G4 case 1 re-run              -> green again (green state reachable after the reds)

G5 one match   -> stderr "pick: target/.../app.msi", stdout captured the bare path | exit 0
G5 two matches -> ::error::expected exactly one artifact, got: .../app.msi .../other.msi | exit 1
G5 zero match  -> ::error::expected exactly one artifact, got: target/.../msi/*.msi | exit 1
G5 one match re-run -> green again
```

**Step 5 (negative space).** `release.yml` grep exit 1 (no banned shape, no
concurrency group); the ci.yml positive control still hits its 2 lines, so the
pattern demonstrably fires.

**Step 6 (pin conformance).** `uses:` count 7, recount unchanged (A1 touched no
`uses:` line); the unpinned-line grep exits 1. Fire-verified again on the final
text: line 98 temporarily set to `@v7`, the grep emitted
`98:      - uses: actions/setup-node@v7` (exit 0), then restored by re-copying
the amended fence, after which the fence diff was empty, the grep clean and the
parse `yaml-ok` again.

### Commit

Amended rather than extended: the branch was never pushed (no upstream, `git
ls-remote --heads origin plan8-c` empty) and the brief prescribes exactly one
commit with one message, so amending leaves the branch at precisely that
prescribed commit carrying the corrected content. Message and trailer re-used
verbatim, `commit.gpgsign=false`, single file staged explicitly.

```
fe46424d4cf17faf24fdfd0c5bcb902b347d5221
release: release.yml - guard (version sync + ci-gate green), four native bundle legs, draft-release assemble with SHA256SUMS (D77/D79/D83-D85/D88-D90); G4/G5 fire-verified
Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
signed: N
```

Final state, verified from git rather than from the working tree:

```
$ git show --name-only --format='' HEAD
.github/workflows/release.yml                 (the only path in the commit)
$ git log --oneline master..HEAD
fe46424 release: release.yml - ...                (exactly one commit)
$ git show HEAD:.github/workflows/release.yml | diff - fence-amended.yml
byte-identical                                    (committed blob == amended fence)
$ git show HEAD:.github/workflows/release.yml | python3 -c "...yaml.safe_load..."
yaml-ok (committed blob)
$ git status --short
                                                  (clean)
```

`ci.yml` is not in the commit (D83). The branch is 6 commits behind master (A1
plus house harvests); that is the controller's merge concern, not a task change.

### Status

**DONE.** All seven steps green: transcription (verbatim against the amended
fence, empty diff on two independent extractions), parse (`yaml-ok`, second
parser agrees), G4 and G5 (ten observed outputs matching section 8), negative
space (with firing positive control), pin conformance (7/7, fire-verified),
commit (`fe46424`, unsigned, trailer, one file).

### Residual notes for the controller

1. **The nine-part gate is still not run** - unchanged from the original run;
   pre-push / post-merge controller action, and this branch adds one CI workflow
   file touching no Rust or TypeScript path.
2. **Concern 2 stays as adjudicated** (no explicit `permissions:` on `bundle`;
   the workflow-level `contents: read` supplies it). Recorded so the task
   reviewer can still weigh it, per the ruling.
3. **House-ledger datapoint: the interactive-`cp`/`rm` alias class bit again.**
   The standing rule warned about it and I still lost one `cp` and one `rm` to
   the `-i` alias prompt (the `cp` silently left the file un-overwritten, which
   the very next check caught because it re-read the file rather than assuming
   the copy). Fixed by `command cp -f ... < /dev/null` and `rm -f ... </dev/null`.
   Flagged because master already carries two occurrences of this class
   (`54386ec` from T3, `c3cc332` from T5); this is a further one, in T4.
