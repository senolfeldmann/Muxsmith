# Task 2 report: D86 + D82 - bundle configuration, CLI-sidecar overlay, collateral

**Status: DONE_WITH_CONCERNS** (one documentation-accuracy residue outside my scope, section "Concerns")

- Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan8-a`, branch `plan8-a`
- Base: `7e36f96` (Task 1, merged)
- Commit: `c884bd6233355deedf1e89f808619b711f01af1b` - unsigned (`%G?` = `N`), trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, 4 files changed, 48 insertions, 2 deletions
- Tree after the run: clean, no strays
- Sources read before editing: implementer preamble; task-2 brief; D82 (design 592-712), D86 (921-1012), sections 3.1, 3.2, 3.4, 4.4, plus 1.1 for the schema facts the literals rest on - all from the worktree copy

## Amendment A1: verified irrelevant to this task, no stale transcription

The worktree copy of the design is pre-A1 (`git merge-base --is-ancestor d21a19f HEAD` -> NO). A1's diff against my base touches exactly three hunks of the design, none in my scope:

```
$ git diff d21a19f HEAD -- docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md | grep '^@@'
@@ -1,7 +1,6 @@
@@ -1332,7 +1331,7 @@ jobs:
@@ -2024,38 +2023,3 @@ NEEDS_CONTEXT with a decision memo (`proc-latitude-clause-boundary`).
```

Status line, section 2's step-name quoting (Task 4's fork), amendment log. Sections 3.1/3.2/3.4/4.4 and D82/D86 are byte-identical pre- and post-A1, so my transcription source is current.

Overlay filename cross-check (the brief's `src-tauri/tauri.bundle.conf.json` vs every design reference, incl. the section-2 build step at line 1358):

```
$ grep -n 'tauri.bundle.conf.json' docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md
608:1. A committed overlay `src-tauri/tauri.bundle.conf.json` containing
620:   `pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json ...`
705:**Interface changes:** new committed file `src-tauri/tauri.bundle.conf.json`;
788:`pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json --bundles <leg set>`
1358:        run: pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json --bundles "${{ matrix.bundles }}"
1528:### 3.2 `src-tauri/tauri.bundle.conf.json` (D82, new, committed, complete)
1749:overlay (`src-tauri/tauri.bundle.conf.json`); plain `pnpm exec tauri
1759:pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json
```

One name at all eight sites; the shipped file matches it exactly.

## Steps 1-4: what changed

| Step | File | Change |
|---|---|---|
| 1 | `src-tauri/tauri.conf.json` | bundle block rewritten to design 3.1; `build` and `app` untouched; no `version` key (Task 1's D87 state preserved) |
| 2 | `src-tauri/tauri.bundle.conf.json` | new, design 3.2 content |
| 3 | `.gitignore` | one line `src-tauri/binaries/` after `src-tauri/gen/` in the `# JS/Tauri frontend` block |
| 4 | `BUILDING.md` | `### Reproducing a release bundle locally` appended as the last subsection of `## Building and running`, immediately before `## Tooling quirks` |

`createUpdaterArtifacts`, `fileAssociations`, `macOS.dmg`, `linux.appimage` are absent as decided (D76/D86), fire-verified below.

### Verbatim transcription, checked mechanically not by eye

Each block was extracted from the design by line range and diffed against the written artifact:

```
--- 3.1 bundle block: design vs live ---
IDENTICAL (32 lines)
--- 3.2 overlay: design vs live ---
IDENTICAL
--- 4.4 rider: design vs live ---
IDENTICAL (18 lines)
```

These are absence-shaped results (diff silence), so they carry a control: under FT-1's deliberate mutation the same 3.1 comparison fired with a multi-hunk diff (see FT-1). The line counts (32 / 18) also rule out an empty-vs-empty false pass.

### Scope discipline on `tauri.conf.json`

The rewrite was a whole-file `Write`; the resulting diff is a single hunk confined to the bundle block:

```
$ git diff -- src-tauri/tauri.conf.json
@@ -30,13 +30,34 @@
   },
   "bundle": {
     "active": true,
-    "targets": "all",
+    "targets": ["msi", "dmg", "deb", "rpm", "appimage"],
+    "publisher": "Şenol Feldmann",
[...]
+    "linux": {
+      "deb": { "section": "video", "recommends": ["mkvtoolnix"] },
+      "rpm": { "recommends": ["mkvtoolnix"] }
+    }
   }
 }
```

Nothing outside `bundle` is touched. Top-level key set before the change was `$schema`, `productName`, `identifier`, `build`, `app`, `bundle` - exactly what 3.1 shows, so the plan's 2026-07-23 verification still holds.

## Step 5: named checks (all foreground)

The brief's five checks (first block), plus `grep -c` for the "one hit" wording (second block, a separate invocation):

```
json-ok
--- publisher ---
Şenol Feldmann
--- icon count ---
5
--- targets ---
msi,dmg,deb,rpm,appimage
--- gitignore ---
16:src-tauri/binaries/
```

```
--- gitignore hit count ---
1
```

`Ş` is a real U+015E, not a transliteration and not a `\u` escape, in both fields:

```
$ jq -r '.bundle.publisher, .bundle.copyright' src-tauri/tauri.conf.json | hexdump -C | head -4
00000000  c5 9e 65 6e 6f 6c 20 46  65 6c 64 6d 61 6e 6e 0a  |..enol Feldmann.|
00000010  43 6f 70 79 72 69 67 68  74 20 28 63 29 20 32 30  |Copyright (c) 20|
00000020  32 36 20 c5 9e 65 6e 6f  6c 20 46 65 6c 64 6d 61  |26 ..enol Feldma|
00000030  6e 6e 0a                                          |nn.|
```

`.gitignore` line 16 sits between `src-tauri/gen/` (15) and the blank line before the Task-12 comment, i.e. inside the JS/Tauri block as specified.

Task 1's invariant re-checked (my rewrite could have silently reintroduced `version`):

```
$ jq 'has("version")' src-tauri/tauri.conf.json
false
$ bash scripts/check-version-sync.sh
version-sync: OK (0.1.0)
```

`cargo check -p muxsmith-gui`, with `touch src-tauri/build.rs` first so the build script genuinely re-ran (no warm-cache pass claimed as evidence):

Commands run (inside `bash -c`, from the worktree root):

```
touch src-tauri/build.rs
cargo check -p muxsmith-gui > /tmp/cc.log 2>&1; echo "cargo check exit: $?"
grep -E "^(warning|error)" /tmp/cc.log || echo "(no warning/error lines)"
tail -2 /tmp/cc.log
```

Output:

```
cargo check exit: 0
(no warning/error lines)
   Compiling muxsmith-gui v0.1.0 (/home/senol/Git/Muxsmith/.worktrees/plan8-a/src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
```

Not run (controller duty per the preamble, not a task step): the nine-part gate. This task changes packaging config only; the sole consumer of the changed config in the gate is `tauri-build`'s build script, exercised above.

## Fire-verifications

Every absence-shaped assertion in this task was made to produce output once. Backups were taken with `command cp -f`, restores done with `/usr/bin/cp -f` or `git checkout --`, each restore confirmed by sha256 or `git status`.

### FT-1: the omitted-key check can fire

Injected `createUpdaterArtifacts` into the bundle block:

```
-- with createUpdaterArtifacts injected:
{
  "createUpdaterArtifacts": true
}
-- and the design-block diff on the mutated file:
3c3,9
<     "targets": ["msi", "dmg", "deb", "rpm", "appimage"],
---
>     "targets": [
[...]
>     "createUpdaterArtifacts": false
(diff fired as expected)
-- restored:
{
  "createUpdaterArtifacts": false
}
restore verified by hash
```

(The trailing `false` is the `has()` verdict on the restored file, i.e. key absent.) Both the `has()` check and the design-block diff are falsifiable.

### FT-2: `cargo check` clean is meaningful - it can fail on an active sidecar

`externalBin` moved into the BASE config with no staged binary:

```
=== FT-2a: baseline - overlay file present, NOT passed via -c: cargo check clean? ===
   Compiling muxsmith-gui v0.1.0 (/home/senol/Git/Muxsmith/.worktrees/plan8-a/src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s

=== FT-2b: fire - the same externalBin in the BASE config, no staged sidecar ===
  cargo:rustc-env=TAURI_ENV_TARGET_TRIPLE=x86_64-unknown-linux-gnu
  resource path `binaries/muxsmith-x86_64-unknown-linux-gnu` doesn't exist
```

So D82's "the overlay keeps the dev/test surface byte-identical to today" is verified by a check that demonstrably fails when the sidecar requirement is active.

Process note: the FT-2 script aborted before its own restore step because I used bash's `PIPESTATUS` under zsh (`(eval):17: PIPESTATUS[0]: parameter not set`, fatal under `set -u`), leaving the config mutated. Caught immediately, restored with `/usr/bin/cp -f` and confirmed:

```
-- pre-restore state (mutated?):
true
-- post-restore:
false
src-tauri/tauri.conf.json: OK
```

### FT-3: the overlay FILENAME is load-bearing (new empirical evidence for D82)

Design 1.2/D82 argue from source-read that only `tauri.<platform>.conf.json` auto-merges. Confirmed at runtime with the installed tooling - same content, two names:

```
cargo check exit with tauri.linux.conf.json present: 101
  resource path `binaries/muxsmith-x86_64-unknown-linux-gnu` doesn't exist
cargo check exit with only tauri.bundle.conf.json present: 0
```

The chosen name never applies implicitly; a platform-pattern name would break every plain build. Temp file removed, tree clean.

### FT-4: the ignore rule works, with a positive control

Staged a sidecar at the real lookup path plus a deliberately non-ignored probe in the same directory:

```
-- git status --porcelain, src-tauri paths only:
    M src-tauri/tauri.conf.json
   ?? src-tauri/PROBE-NOT-IGNORED
   ?? src-tauri/tauri.bundle.conf.json
-- check-ignore verdicts:
.gitignore:16:src-tauri/binaries/	src-tauri/binaries/muxsmith-x86_64-unknown-linux-gnu
   PROBE not ignored (expected: the control shows untracked files DO surface)
```

The staging dir is ignored by rule `.gitignore:16`, and the control proves `git status` does surface untracked files there, so the absence is the rule working rather than a blind check.

### FT-5 (control, not required): what `cargo check` actually validates

To know whether "config parses" means "field names verified", I injected an unknown bundle field:

```
exit with an unknown bundle field: 101
  unknown field `bogusFieldProbe`, expected one of `active`, `targets`, `createUpdaterArtifacts`, `publisher`, `homepage`, `icon`, `resources`, `copyright`, `license`, `license-file`, `licenseFile`, `category`, `fileAssociations`, `short-description`, `shortDescription`, `long-description`, `longDescription`, `use-local-tools-dir`, `useLocalToolsDir`, `external-bin`, `externalBin`, `windows`, `linux`, `macOS`, `macos`, `iOS`, `ios`, `android`
```

`tauri-build` denies unknown fields, and the error enumerates the accepted set - so the clean `cargo check` independently confirms every field name and type I wrote (`publisher`, `copyright`, `homepage`, `license`, `licenseFile`, `category`, `shortDescription`, `longDescription`, `targets` as an array, plus the `windows.wix.language` / `linux.deb.recommends` / `linux.rpm.recommends` / `macOS.minimumSystemVersion` sub-objects) against the installed tooling, not only against the design's schema read. `licenseFile: "../LICENSE"` resolves to the existing repo-root `LICENSE` (present, 1072 bytes).

## Post-commit re-verification against the committed text

```
committed tauri.conf.json parses
committed overlay parses
Şenol Feldmann
5
msi,dmg,deb,rpm,appimage
false
1
```

(publisher, icon count, targets, `has("version")`, `.gitignore` hit count - read out of `git show HEAD:`, not the worktree.)

sha256 of the committed blobs: `3371d13d...` (`tauri.conf.json`), `780279a2...` (`tauri.bundle.conf.json`).

## Interfaces produced (for Tasks 4 and 6)

- `targets` superset `msi,dmg,deb,rpm,appimage` - the values Task 4's `--bundles` narrows against.
- Overlay path for `-c`: `src-tauri/tauri.bundle.conf.json`; sidecar lookup path the legs must stage: `src-tauri/binaries/muxsmith-<host-triple>[.exe]` (FT-2b/FT-3 show the exact string the build script demands on this host: `binaries/muxsmith-x86_64-unknown-linux-gnu`).
- `src-tauri/binaries/` ignored, so a leg's staging step leaves no dirty tree.
- R8's rehearsal observable is in place: `bundle.publisher` carries U+015E.

## Concerns

1. **Stale neighbouring paragraph in BUILDING.md, deliberately not touched.** Lines 65-68 still read: "Building the desktop bundle itself (`pnpm exec tauri build`) is out of scope for local development and not part of the CI gate yet." My appended subsection documents exactly that local invocation, and Task 4 adds the workflow that builds bundles in CI - so the sentence is wrong on both halves once this plan lands. Design 4.4 is an "addition, complete" and my brief scopes the file to one appended subsection, so editing that paragraph would be unrequested latitude; I did not. It is also not covered elsewhere: `grep -n 'BUILDING' task-*-brief.md implementer-preamble.md` matches only task-2-brief.md (four hits, all mine) and the preamble's gate line. Controller decision - a one-sentence rider in a later task or a plan-close doc item.

2. **`git status` was clean after every fire-test, but three of them ran before the commit.** The pre-commit hashes were re-checked against `sha256sum -c` (all four files OK) and the committed blobs re-verified afterwards, so the commit contains the intended text; noted only because the audit trail spans mutations.

No fork was encountered: every literal was frozen by design section 11 and carried verbatim in the brief, and the one omission-shaped question I checked (the overlay filename) is enumerated identically at all eight design sites.
