# Task 1 report: D87 version-sync guard script; tauri.conf.json stops declaring a version

Stream A, worktree `.worktrees/plan8-a` (branch `plan8-a`). Status: **DONE**.

## Reading done before edits

- `implementer-preamble.md` (Global Constraints) - read in full.
- `task-1-brief.md` - read in full; code carried verbatim, transcription task.
- Design doc `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
  (worktree copy), D87 (lines 1015-1076), section 3.3 (lines 1538-1566),
  section 8 G1-G3 (lines 1848-1861) - all read in full before touching any file.
- Diffed the design's section 3.3 code block (lines 1540-1566, minus the two
  markdown fence lines) against the written `scripts/check-version-sync.sh`:
  `diff` reported only the two fence-line deletions, zero body differences.
  Byte-identical transcription confirmed.

No fork encountered. The brief settled every decision point (exact file
list, exact script content, exact commit message); nothing required a
NEEDS_CONTEXT escalation.

## Steps executed

### Step 1: delete `version` from `src-tauri/tauri.conf.json`

Removed the single line `"version": "0.1.0",`, nothing else touched.

```
$ jq 'has("version")' src-tauri/tauri.conf.json
false
$ jq -r 'keys_unsorted | join(",")' src-tauri/tauri.conf.json
$schema,productName,identifier,build,app,bundle
```

Matches the brief's expected output exactly (pre-change list was the same
plus `"version"`).

### Step 2: create `scripts/check-version-sync.sh`

Written verbatim from design 3.3, then `chmod +x`. File mode confirmed
`755` before commit and preserved as `100755` in the commit (git diffstat
showed `create mode 100755`).

### Step 3: fire-tests G1-G3 (all foreground, break-observe-restore)

**G1** - tag arm red state, then plain-run green state:
```
$ scripts/check-version-sync.sh v9.9.9; echo "EXIT=$?"
version-sync: tag v9.9.9 != v0.1.0
EXIT=1

$ scripts/check-version-sync.sh; echo "EXIT=$?"
version-sync: OK (0.1.0)
EXIT=0
```

**G2** - reintroduced `"version": "0.1.0"` into `tauri.conf.json`, plain run:
```
$ scripts/check-version-sync.sh; echo "EXIT=$?"
version-sync: src-tauri/tauri.conf.json declares 'version'; it must inherit from Cargo.toml (D87)
EXIT=1
```
Reverted (re-deleted the key via edit, per the brief's execution note - the
Step 1 change was not yet committed at this point). Re-ran Step 1's two jq
checks post-revert:
```
$ jq 'has("version")' src-tauri/tauri.conf.json
false
$ jq -r 'keys_unsorted | join(",")' src-tauri/tauri.conf.json
$schema,productName,identifier,build,app,bundle
```

**G3** - set `package.json` version to `0.1.1`, plain run:
```
$ scripts/check-version-sync.sh; echo "EXIT=$?"
version-sync: Cargo.toml (0.1.0) != package.json (0.1.1)
EXIT=1
```
Reverted to `0.1.0`, verified:
```
$ jq -r .version package.json
0.1.0
```
Post-revert sanity (plain run, expect green):
```
$ scripts/check-version-sync.sh; echo "EXIT=$?"
version-sync: OK (0.1.0)
EXIT=0
```

All three fire-tests fired their red state as specified and returned to
green after revert. `git status --short` after G2/G3 revert showed only
the intended Step 1 diff (`M src-tauri/tauri.conf.json`) plus the untracked
new script - `package.json` carried no diff, confirming a clean revert.

### Step 4: config-parse sanity

```
$ cargo check -p muxsmith-gui
   ... (full workspace dependency chain, tauri-build etc.) ...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 22.90s
$ cargo check -p muxsmith-gui 2>&1 | grep -i "warning\|error"
NO WARNINGS OR ERRORS
```
Clean build, zero warnings, zero errors. Confirms `tauri_build::build()`
parses the version-less `tauri.conf.json` without issue.

### Step 5: commit

Explicit staging (no `git add -A`):
```
$ git add src-tauri/tauri.conf.json scripts/check-version-sync.sh
$ git status --short
A  scripts/check-version-sync.sh
M  src-tauri/tauri.conf.json
```
Commit:
```
$ git -c commit.gpgsign=false commit -m "release: version-sync guard script; tauri.conf.json inherits the Cargo workspace version (D87); G1-G3 fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
[plan8-a 7e36f96] release: version-sync guard script; tauri.conf.json inherits the Cargo workspace version (D87); G1-G3 fire-verified
 2 files changed, 25 insertions(+), 1 deletion(-)
 create mode 100755 scripts/check-version-sync.sh
```

Commit hash: **7e36f96**.

Post-commit tree state: `git status --short` empty (clean).

## Self-review

- **Diff scope**: `git diff` on `tauri.conf.json` shows exactly one line
  removed (`"version": "0.1.0",`), nothing else - matches "nothing else"
  in the brief's Files section.
- **Script transcription**: `diff` against the design's section 3.3 code
  block confirmed byte-identical body (only markdown fence lines differ,
  which are not part of the script).
- **Executable bit**: confirmed `755` locally and `100755` in the commit
  tree object.
- **No new dependencies**: script uses only `awk` and `jq`, both already
  named as consumed interfaces in the brief; no cargo/npm manifest touched.
- **ci.yml untouched**: `git status` throughout shows no file outside the
  two named files ever modified.
- **Typography**: grepped both changed/new files for em/en-dash, curly
  quotes, ellipsis, NBSP - none found.
- **Session-relocation ban**: no EnterWorktree/ExitWorktree or equivalent
  called at any point; all commands ran via absolute-path `cd` into the
  worktree inside the Bash tool.
- **Foreground only**: every command ran synchronously (no
  `run_in_background`).
- **Staging**: `git add` named the two files explicitly; no `git add -A`
  anywhere in the session.
- **Commit trailer/signing**: unsigned via `-c commit.gpgsign=false`,
  `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer present
  (verified in the commit output above).
- **gh usage**: none in this task; no `gh-log.md` entry required.

## Concerns

None. No fork was encountered; the brief's transcription task matched the
design doc exactly on comparison, all four verification steps produced
the exact expected outputs, and the commit is clean and scoped to exactly
the two named files.
