# Task 6 report: the REHEARSAL (plan-8 wave 2, acceptance test)

**Status: BLOCKED.**

Run A failed. Both Windows legs die in `Build bundles (D84)` at WiX
`light.exe`; `assemble` is therefore skipped wholesale, which removes the
draft-release path that R4/R5 are defined on. Run B was **not dispatched**
(decision memo below - the controller routes it). Every observable that
run A's three green jobs can still produce was evaluated at its named
emitter; nothing was retried.

- **Run A**: id `30263340264`, event `workflow_dispatch`,
  `rehearse-draft-release` at its default `false`,
  https://github.com/senolfeldmann/Muxsmith/actions/runs/30263340264
  -> conclusion **failure**.
- **Run B**: not dispatched. Rationale in "Decision memo 1".
- **Fully passed: 2 of 10** (R7, R9). 2 failed (R1, R2), 1 inconclusive at
  its named emitter (R3), 1 not evaluable (R4), 2 partial (R5, R6), 2
  owner-pending (R8, R10 - R10 is a no-op this run: no draft exists).

Dispatch SHA `dbd0dc38fd374c66f6288dd2857de8e51a8e2638` (= `origin/master`,
verified by `git rev-parse HEAD origin/master`), workspace version
`0.1.0` (awk-parsed from `Cargo.toml`, never hardcoded).

---

## Preconditions (all three verified before the dispatch)

1. **Wave-1 merged, gate green, master pushed.** `git rev-parse HEAD
   origin/master` returns the same SHA twice; `git status --porcelain`
   empty; `git cat-file -e origin/master:.github/workflows/release.yml`
   exits 0, so `workflow_dispatch` had a default-branch workflow to
   dispatch.
2. **ci gate green on the head SHA.**

   ```
   $ gh run list --workflow ci.yml --commit dbd0dc38fd374c66f6288dd2857de8e51a8e2638 --json databaseId,status,conclusion,createdAt
   [{"conclusion":"success","createdAt":"2026-07-27T11:15:37Z","databaseId":30261258685,"status":"completed"}]
   ```

3. **R3 baseline.** `gh release list` -> empty output, exit 0. The repo had
   zero releases before the rehearsal.

Recovery note (the three 529-overload kills before this run): `gh run list
--workflow=release.yml --limit 5 --json databaseId,status,conclusion,createdAt`
returned `[]`, so no earlier dispatch existed and nothing was re-dispatched
over a live run. Backfilled into `gh-log.md`.

---

## Verdict table

| R | Verdict | Named emitter | Evidence summary |
|---|---------|---------------|------------------|
| R1 | **FAIL** | guard log; per-leg rename-step stderr + closing `ls -l` | guard both steps green; 4 of the expected 6 `pick:` lines (macOS 1, Linux 3, **Windows 0+0**); `ls -l` 1 / 4 on the two green legs, absent on both Windows legs |
| R2 | **FAIL** | `gh api .../artifacts` + `gh run download` | **2 of 4** artifacts, **5 of 7** files. Names conformant, retention exactly 7 days on both |
| R3 | **INCONCLUSIVE at the emitter** | assemble job view | the assemble **job** was skipped with an EMPTY steps array, so the step-level skip that R3 names never materialized; corroboration passes (`gh release list` still empty) |
| R4 | **NOT EVALUABLE** | assemble release step | no draft exists - assemble never ran |
| R5 | **PARTIAL** | 4 leg logs + release body | 2 of 4 `updater-artifact check:` lines, both with N > 0; body half not evaluable (no draft) |
| R6 | **PARTIAL (3 pass, 2 blocked)** | downloaded assets | rpm PASS, AppImage PASS, tar.gz PASS; **deb blocked** (`dpkg-deb` absent), **msi blocked twice** (no msi artifact AND `msiextract` absent) |
| R7 | **PASS** | `git ls-remote origin` | 0 `rehearsal` hits, 1 `refs/heads/master` hit (positive control fired), 0 `refs/tags/` hits |
| R8 | **PENDING OWNER** | real hardware | dmg artifact exists and is inspectable; **no msi artifact exists**, so the Windows walk-through has no input yet |
| R9 | **PASS** | tar.gz binary | `muxsmith 0.1.0` == awk-parsed `0.1.0` |
| R10 | **N/A this run** | owner | no draft was created, so there is nothing to delete. Ownership of the deletion is a scope conflict - Decision memo 2 |

---

## R1 - guard PASS, legs 2 of 4

**Guard, step 1** (`Version sync (D87; tag arm only on the tag path)`), run A
full log line 126:

```
guard	Version sync (D87; tag arm only on the tag path)	2026-07-27T11:48:35.5402310Z version-sync: OK (0.1.0)
```

**Guard, step 2** (`Require the ci gate green on this SHA (D83)`), line 150:

```
guard	Require the ci gate green on this SHA (D83)	2026-07-27T11:48:36.0333715Z ci gate green for dbd0dc38fd374c66f6288dd2857de8e51a8e2638
```

Both guard steps pass. **Sub-finding (observable/emitter wording
mismatch, not a workflow defect):** R1 asks that the gate-green step's log
"names the found ci run". The emitter's only success echo is `echo "ci gate
green for $GITHUB_SHA"` - it names the **SHA**, never the run id or number;
the script never extracts one from the `gh run list --json
status,conclusion` result it queries. The observable is satisfiable only
under the reading "names the SHA the green run was found for". Recorded for
the design's `design-acceptance-observables-have-producers` bookkeeping.

**`pick:` lines - recount against the leg matrix.** Every `pick:` line in
the whole run (grep `'^bundle.*Z pick: '`, which excludes the echoed script
body from the `##[group]` header):

```
bundle (macos-arm64, macos-15, dmg)	...	2026-07-27T11:52:03.9309010Z pick: target/release/bundle/dmg/Muxsmith_0.1.0_aarch64.dmg
bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)	...	2026-07-27T11:55:36.5587639Z pick: target/release/bundle/deb/Muxsmith_0.1.0_amd64.deb
bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)	...	2026-07-27T11:55:36.5593409Z pick: target/release/bundle/rpm/Muxsmith-0.1.0-1.x86_64.rpm
bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)	...	2026-07-27T11:55:36.5599021Z pick: target/release/bundle/appimage/Muxsmith_0.1.0_amd64.AppImage
```

Per-leg count, recomputed from that enumeration:

```
      3 bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)
      1 bundle (macos-arm64, macos-15, dmg)
```

Expected 1/1/1/3 = 6; observed 0/0/1/3 = **4**. The two Windows legs
produced none because they died in the preceding step. Where the function
did run, it behaved exactly as designed: one `pick:` per selection, on
stderr, and the bundler's native names it globs around
(`Muxsmith_0.1.0_aarch64.dmg`, `Muxsmith-0.1.0-1.x86_64.rpm`) confirm D89's
premise that hardcoding them would have been wrong.

**Closing `ls -l`, macOS leg** (1 file, matches):

```
total 13752
-rw-r--r--@ 1 runner  staff  7039809 Jul 27 11:52 muxsmith-0.1.0-macos-arm64.dmg
```

**Closing `ls -l`, Linux leg** (4 files incl. the tar.gz, matches):

```
total 106056
-rwxr-xr-x 1 runner runner 84896248 Jul 27 11:55 muxsmith-0.1.0-linux-x86_64.AppImage
-rw-r--r-- 1 runner runner  7963766 Jul 27 11:55 muxsmith-0.1.0-linux-x86_64.deb
-rw-r--r-- 1 runner runner  7964715 Jul 27 11:55 muxsmith-0.1.0-linux-x86_64.rpm
-rw-r--r-- 1 runner runner  7769321 Jul 27 11:55 muxsmith-0.1.0-linux-x86_64.tar.gz
```

Corroborated by the upload action's own line counts: `there will be 1 file
uploaded` (macOS) and `there will be 4 files uploaded` (Linux).

### The Windows failure (the R1 blocker)

Both legs fail identically, on two different machines and two
architectures. windows-arm64 (`windows-11-arm`), log lines 524-526:

```
bundle (windows-arm64, windows-11-arm, msi)	Build bundles (D84)	2026-07-27T11:56:04.6693032Z      Running light to produce C:\a\Muxsmith\Muxsmith\target\release\bundle\msi\Muxsmith_0.1.0_arm64_en-US.msi
bundle (windows-arm64, windows-11-arm, msi)	Build bundles (D84)	2026-07-27T11:56:08.6574655Z failed to bundle project: `failed to run C:\Users\runneradmin\AppData\Local\tauri\WixTools314\light.exe`
bundle (windows-arm64, windows-11-arm, msi)	Build bundles (D84)	2026-07-27T11:56:08.6584408Z        Error failed to bundle project: `failed to run C:\Users\runneradmin\AppData\Local\tauri\WixTools314\light.exe`
```

windows-x86_64 (`windows-2025`), lines 1051-1053, same shape with
`...\bundle\msi\Muxsmith_0.1.0_x64_en-US.msi`.

Everything up to the WiX bind step is green, which narrows the blast radius
usefully:

```
bundle (windows-x86_64, windows-2025, msi)	Build bundles (D84)	2026-07-27T11:57:04.5776666Z     Finished `release` profile [optimized] target(s) in 6m 06s
bundle (windows-x86_64, windows-2025, msi)	Build bundles (D84)	2026-07-27T11:57:04.6256608Z        Built application at: D:\a\Muxsmith\Muxsmith\target\release\muxsmith-gui.exe
bundle (windows-x86_64, windows-2025, msi)	Build bundles (D84)	2026-07-27T11:57:04.6821484Z         Info Verifying wix package
bundle (windows-x86_64, windows-2025, msi)	Build bundles (D84)	2026-07-27T11:57:05.7529170Z         Info Target: x64
bundle (windows-x86_64, windows-2025, msi)	Build bundles (D84)	2026-07-27T11:57:19.5589704Z      Running light to produce D:\a\Muxsmith\Muxsmith\target\release\bundle\msi\Muxsmith_0.1.0_x64_en-US.msi
```

The arm64 leg's counterpart prints `Info Target: arm64` and `Running candle
for "...\target\release\wix\arm64\main.wxs"`, so design section 1.2's
verified arm64 arch mapping and the `$(sys.BUILDARCH)="arm64"` template
branch both hold. Cargo compiled, the sidecar staged, WiX downloaded and
hash-validated, candle compiled the `.wxs`. Only `light.exe` (the binder)
fails.

**tauri-bundler discards light's stderr at this version.** The failed-step
log ends at the `failed to run ...light.exe` line; grep over the whole
1054-line failed log for `LGHT[0-9]`, `CNDL[0-9]` and `error [A-Z]+[0-9]+`
returns no WiX diagnostic. So the actual WiX error code is **not in the
run's evidence** and no fix should be built on a guess about it.

**Leading hypothesis, explicitly UNVERIFIED, offered as the diagnostic
path and not as a finding.** Exactly two config values in the whole bundle
surface contain a byte outside ASCII, and both are WiX-bound:

```
$ grep -nP '[^\x00-\x7F]' src-tauri/tauri.conf.json src-tauri/tauri.bundle.conf.json
src-tauri/tauri.conf.json:34:    "publisher": "Şenol Feldmann",
src-tauri/tauri.conf.json:35:    "copyright": "Copyright (c) 2026 Şenol Feldmann",
```

`Ş` is U+015E, which has no Windows-1252 representation (it lives in
Windows-1254 / ISO-8859-9). WiX 3's binder writes the MSI database in the
culture's code page - `wix.language: ["en-US"]` selects 1252 - and WiX
documents a light-time error class for exactly this: a string containing
characters unavailable in the database code page, fixed either by changing
the characters or by setting a `Codepage` attribute
([WiX code pages](https://documentation.help/WiX-3.10.1/codepage.html),
[LGHT0311 thread](https://wix-users.narkive.com/1yDvG3UO/getting-error-lght0311),
[FireGiant code pages](https://docs.firegiant.com/wix/tools/codepage/)).
That the failure is identical on x64 and arm64 points at a config cause
over an environment one. **Confirming step (controller's to route, not
mine):** one dispatch with `tauri build --verbose` (or any variant that
surfaces light's stderr) to read the real error code before touching
anything.

**Why this matters beyond a build fix - Decision memo 3.** D86 pre-decided
a publisher fallback (`publisher` -> `"Senol Feldmann"`) and gated it on
**R8's** observable, a *rendering* failure in Programs-and-Features. If the
code-page hypothesis holds, the same character fails at *build* time, R8 is
never reached, and the pre-decided fallback's trigger never fires; the
`copyright` field carries the same character, so a publisher-only change
may also be insufficient. Reconciling that is a design decision, not an
implementer call.

---

## R2 - FAIL (2 of 4 artifacts, 5 of 7 files)

```
$ gh api "repos/senolfeldmann/Muxsmith/actions/runs/30263340264/artifacts" --jq '.total_count, (.artifacts[] | {name, expires_at, created_at, size_in_bytes, expired})'
2
{"created_at":"2026-07-27T11:55:41Z","expired":false,"expires_at":"2026-08-03T11:55:37Z","name":"muxsmith-linux-x86_64","size_in_bytes":107701118}
{"created_at":"2026-07-27T11:52:06Z","expired":false,"expires_at":"2026-08-03T11:52:04Z","name":"muxsmith-macos-arm64","size_in_bytes":6996184}
```

What passes: both names match `muxsmith-<leg id>` with D85's leg ids, and
`retention-days: 7` is confirmed exactly - `created_at` 2026-07-27,
`expires_at` 2026-08-03 on both, i.e. 7 days. The workflow's own echoed
inputs corroborate (`retention-days: 7`, `if-no-files-found: error`).

What fails: `total_count` is **2**, expected 4.

**Artifact inventory vs the D89 naming scheme.** `gh run download
30263340264 --dir <scratch>/runA` -> 5 files:

```
runA/muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.AppImage
runA/muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.deb
runA/muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.rpm
runA/muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.tar.gz
runA/muxsmith-macos-arm64/muxsmith-0.1.0-macos-arm64.dmg
```

Against D89's 7 pre-SHA256SUMS names, with the version token recomputed
from `Cargo.toml` (`0.1.0`):

| # | D89 name | Present |
|---|----------|---------|
| 1 | `muxsmith-0.1.0-windows-x86_64.msi` | **MISSING** (leg red) |
| 2 | `muxsmith-0.1.0-windows-arm64.msi` | **MISSING** (leg red) |
| 3 | `muxsmith-0.1.0-macos-arm64.dmg` | yes |
| 4 | `muxsmith-0.1.0-linux-x86_64.deb` | yes |
| 5 | `muxsmith-0.1.0-linux-x86_64.rpm` | yes |
| 6 | `muxsmith-0.1.0-linux-x86_64.AppImage` | yes |
| 7 | `muxsmith-0.1.0-linux-x86_64.tar.gz` | yes |

Count check: **5**, expected 7. Every file that exists is
character-for-character scheme-conformant, including the capitalized
`.AppImage`. The rename-and-glob machinery of D89 is therefore proven for
5 of 7 artifacts; only the two the red legs never produced are missing.

**D90 SHA256SUMS content check: NOT PERFORMED.** `SHA256SUMS` is generated
by the assemble job, which was skipped, so no such file exists in this run.
No substitute was computed - a locally generated checksum file over 5 of 7
files would prove nothing about D90's emitter.

---

## R3 - INCONCLUSIVE at its named emitter; corroboration passes

R3's design text is explicit that the *step-level skip* is the observable
and an absence-grep is not acceptable. That emitter did not exist in run A:

```
$ gh api "repos/senolfeldmann/Muxsmith/actions/runs/30263340264/jobs" --jq '.jobs[] | select(.name=="assemble") | {name, conclusion, steps: [.steps[] | {name, conclusion}]}'
{"conclusion":"skipped","name":"assemble","steps":[]}
```

The **job** was skipped because `needs: [guard, bundle]` went red, so the
`Create draft release (tag path) or rehearsal draft (D77/D79)` step never
materialized and has no `skipped` conclusion of its own to read. This is
the *right* outcome for safety but the *wrong* mechanism for the
observable: R3 is meant to prove that the step's `if: github.ref_type ==
'tag' || inputs.rehearse-draft-release` guard suppresses release creation
on the artifact path. That specific claim remains untested - a green run A
is its only possible producer.

Corroboration, which does pass: `gh release list` -> empty output, exit 0,
identical to the Step-0 baseline. No release was created.

---

## R4 - NOT EVALUABLE

No draft exists. `assemble` never ran, so no `rehearsal-<run_id>` release
was created, no 8-asset set exists, and the `sha256sum -c SHA256SUMS`
round-trip plus its one-time byte-corruption falsifiability control have no
subject. Nothing was faked or substituted.

---

## R5 - PARTIAL (2 of 4 updater-check lines; body half not evaluable)

The updater-absence half, at its named emitter (the four leg logs). Both
lines that exist, verbatim:

```
bundle (macos-arm64, macos-15, dmg)	Assert no updater artifacts were produced (D76)	2026-07-27T11:52:03.8649600Z updater-artifact check: 0 hits across        5 bundle output files
bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)	Assert no updater artifacts were produced (D76)	2026-07-27T11:55:36.5366007Z updater-artifact check: 0 hits across 250 bundle output files
```

Count: **2** of the required 4. Both carry N > 0 (5 and 250), so the step's
built-in positive control - proof it actually saw a populated bundle tree
rather than passing on an empty one - fired on both green legs. The Windows
legs failed in `Build bundles (D84)`, the step immediately before this one,
so the check never ran there. (The `wc -l` padding in the macOS value is
BSD `wc` output, cosmetic.)

The body half (banner first, then the template with the version
substituted, then generated notes; `grep -c __VERSION__` on the body -> 0,
fire-verified against the committed template) is **not evaluable**: no
release body exists. The template-side half of that fire-verification was
confirmed - `grep -c '__VERSION__' .github/release/draft-body.md` -> **8**,
matching the known-8 control the brief names, so the grep pattern is proven
to fire on a positive case; only the body it should return 0 against is
missing.

---

## R6 - PARTIAL: 3 checks pass, 2 blocked

Tool gate first, as the brief mandates:

```
dpkg-deb: ABSENT      (dpkg-deb -I <deb> -> "command not found", exit 127)
rpm:      /usr/bin/rpm            (RPM version 6.0.1)
msiextract: ABSENT    (msiextract --version -> "command not found", exit 127)
sha256sum: /usr/bin/sha256sum
tar:      /usr/bin/tar
```

Unchanged from the plan-authoring measurement. Per the brief this is a STOP
on the affected checks pending the owner-authorized
`sudo dnf install dpkg msitools` routed through the controller - no system
change was made and no substitute tool was used. (For the controller's
information only, not applied: `ar`, `7z` and `bsdtar` are present and
could read a `.deb`'s control member, but swapping the transcribed tool is
a latitude decision, not mine.)

**rpm - PASS.**

```
$ rpm -qp --recommends muxsmith-0.1.0-linux-x86_64.rpm
mkvtoolnix
```

Both binaries are in the payload, and `--requires` corroborates design
1.2's recorded auto-injection (rpm expresses these as sonames, which is why
the design routes the `libwebkit2gtk-4.1-0` / `libgtk-3-0` *package-name*
check to the deb):

```
$ rpm -qpl muxsmith-0.1.0-linux-x86_64.rpm | grep -E '/usr/bin/'
/usr/bin/muxsmith
/usr/bin/muxsmith-gui
$ rpm -qp --requires muxsmith-0.1.0-linux-x86_64.rpm
libwebkit2gtk-4.1.so.0()(64bit)
libgtk-3.so.0()(64bit)
rpmlib(CompressedFileNames) = 3.0.4-1
rpmlib(FileDigests) = 4.6.0-1
rpmlib(PayloadFilesHavePrefix) = 4.0-1
```

**AppImage - PASS.**

```
$ chmod +x muxsmith-0.1.0-linux-x86_64.AppImage
$ ./muxsmith-0.1.0-linux-x86_64.AppImage --appimage-extract      # exit 0
$ ls -l squashfs-root/usr/bin/
-rwxr-xr-x. 1 senol senol  6380680 Jul 27 13:54 muxsmith
-rwxr-xr-x. 1 senol senol 18349232 Jul 27 13:54 muxsmith-gui
```

`squashfs-root/usr/bin/muxsmith` present, as the design requires. Minor
factual correction to the brief's mechanical note: the file arrived from
`gh run download` **already executable** (`-rwxr-xr-x` before any
`chmod`) - the upload/download round-trip preserved the mode the Linux
leg's own `ls -l` shows. The `chmod +x` was run anyway and is harmless;
INSTALL.md's instruction to users stays correct for other download paths.

**tar.gz - PASS (D88 four-file layout under the version-named directory).**

```
$ tar tzvf muxsmith-0.1.0-linux-x86_64.tar.gz
drwxr-xr-x runner/runner     0 2026-07-27 13:55 muxsmith-0.1.0-linux-x86_64/
-rwxr-xr-x runner/runner 18330416 2026-07-27 13:55 muxsmith-0.1.0-linux-x86_64/muxsmith-gui
-rw-r--r-- runner/runner     1072 2026-07-27 13:55 muxsmith-0.1.0-linux-x86_64/LICENSE
-rw-r--r-- runner/runner     1088 2026-07-27 13:55 muxsmith-0.1.0-linux-x86_64/README.txt
-rwxr-xr-x runner/runner  6374008 2026-07-27 13:55 muxsmith-0.1.0-linux-x86_64/muxsmith
```

5 entries = 1 directory + **4** files, recounted from that listing:
`muxsmith`, `muxsmith-gui`, `LICENSE`, `README.txt`. Directory name equals
the archive's basename.

**deb - BLOCKED** (`dpkg-deb -I` / `-c`, i.e. the `Recommends: mkvtoolnix`,
the `Depends:` containing `libwebkit2gtk-4.1-0` and `libgtk-3-0`, and the
`./usr/bin/muxsmith` + `./usr/bin/muxsmith-gui` payload check). The
artifact is downloaded and available; only the tool is missing.

**msi - BLOCKED twice over**: no msi artifact was produced (both legs red)
*and* `msiextract` is absent.

---

## R7 - PASS

```
$ git ls-remote origin > <scratch>/ls-remote.out    # read-only git, no gh-log entry owed
$ cat <scratch>/ls-remote.out
dbd0dc38fd374c66f6288dd2857de8e51a8e2638	HEAD
dbd0dc38fd374c66f6288dd2857de8e51a8e2638	refs/heads/master
$ grep -c rehearsal <scratch>/ls-remote.out
0
$ grep -c 'refs/heads/master' <scratch>/ls-remote.out
1
$ grep -c 'refs/tags/' <scratch>/ls-remote.out
0
```

The `0` is not a bare absence: the same output demonstrably lists
`refs/heads/master`, so the listing and the grep both work - the
transcribed positive control fired. The repo still carries zero tag refs.

**Scope caveat, stated honestly:** R7 is written against run B. Evaluated
here after run A, it proves the current remote state carries no
`rehearsal` ref and no tag at all. The run-B-specific instance (that the
rehearsal path's `gh release create ... --target "$GITHUB_SHA"` creates no
tag ref) is untested, because that code path never ran.

---

## R8 - PENDING OWNER, with an upstream blocker

Owner steps on real hardware, unchanged in substance: msi install after the
SmartScreen "Run anyway" flow exactly as `docs/INSTALL.md` describes plus
the Programs-and-Features publisher rendering; dmg via the Settings > Open
Anyway flow on Apple Silicon; `muxsmith` from the documented per-OS CLI
locations.

- The **dmg exists** (`muxsmith-0.1.0-macos-arm64.dmg`, 7,039,809 bytes)
  and its half of R8 is materially possible today from run A's artifact.
- The **msi half has no input**: neither msi was built.
- **D86 publisher-fallback protocol**, carried as required: it fires only
  on R8's observable failure and changes exactly the `publisher` field to
  `"Senol Feldmann"`. See Decision memo 3 - the build failure may have
  pre-empted that trigger's premise.

---

## R9 - PASS

```
$ tar xzf muxsmith-0.1.0-linux-x86_64.tar.gz -C <scratch>/tgz
$ cd <scratch>/tgz/muxsmith-0.1.0-linux-x86_64 && ./muxsmith --version
muxsmith 0.1.0
$ awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml
0.1.0
```

The artifact's self-reported version equals the awk-parsed workspace
version. D87's artifact-level criterion holds on the tar.gz leg.

---

## R10 - N/A this run

No draft was created, so there is nothing to delete and no cleanup was
performed. Ownership of the deletion is a live scope conflict, below.

---

## Decision memos (controller routes; none resolved at the keyboard)

### Memo 1: run B was not dispatched

**Fact established by run A:** when the `bundle` matrix ends red,
`assemble` (`needs: [guard, bundle]`) is skipped wholesale - observed, with
an empty steps array. `fail-fast: false` lets the green legs finish but
does not change the matrix job's own conclusion.

**Consequence:** a run B on this SHA cannot produce a draft, so R4, R5's
body half, R6's msi checks and R7's run-B instance are unreachable by it.
Its only yield would be a second identical Windows failure - and run A
already failed twice independently, on `windows-2025` and `windows-11-arm`,
which is the determinism evidence a second run would have bought.

**Why not dispatched:** the task's evaluation discipline forbids a silent
retry and asks for evidence plus a report. Dispatching a run whose designed
observables are structurally unreachable is not evaluation; deferring costs
nothing (public-repo Actions are free, and the run is one command once the
Windows leg is fixed).

**Options for the controller.** (a) Fix the WiX failure, then dispatch A
and B fresh - recommended: it restores the acceptance test's premise, and
R1/R2/R3 need a green run A anyway, so the current run A must be re-run
regardless. (b) Dispatch B now for a determinism data point only - cheap in
money, ~15 min wall clock, yields no new R-observable. (c) Rule that a
partial rehearsal suffices - contradicts section 8, which makes R1-R10 the
plan's acceptance test.

### Memo 2: who deletes the rehearsal draft (scope conflict)

The dispatch message that started this task states the draft "is DELETED
after evaluation" by the implementer. Three binding sources say the
opposite, and they are the ones the dispatch itself pointed at first:

- `implementer-preamble.md` line 11: "The rehearsal drafts are deleted by
  the OWNER at plan close (R10), not by any task."
- `task-6-brief.md` Step 9: "The task does NOT delete the rehearsal
  drafts - the draft is the rehearsal's deliverable for the owner's
  inspection (D79)."
- design section 8, R10: "(owner): delete the rehearsal draft(s) after
  inspection."

Moot for this run (no draft exists), so nothing was done and nothing was
lost either way. It needs an explicit ruling before the run that does
produce a draft: the asymmetry favours not deleting (a deleted draft
removes R8's Windows/macOS inspection surface and is not recoverable
without another full run, whereas an undeleted draft is one command away
from gone). Flagged rather than decided, per the closed-fork rule.

### Memo 3: D86's publisher fallback may have the wrong trigger

Covered in full under R1's Windows section. In one line: D86 gates the
`Ş` -> `S` fallback on R8's *rendering* observable, but if the code-page
hypothesis holds the same character breaks the *build*, R8 is never
reached, and `copyright` carries the character too. Needs a design-level
answer once a verbose run has produced light.exe's actual error.

---

## Method notes

- **Foreground only.** No background run, no monitor. The `gh run watch`
  was wrapped in `timeout 570` and re-entered a second time because this
  harness caps a single Bash call at 10 minutes; the brief's `timeout 5400`
  cannot be expressed in one call here. The second call returned
  `Run release (30263340264) has already completed with 'failure'` (exit
  1), and every subsequent read was a bounded `gh run view` / `gh api`
  poll, never a re-dispatch.
- **No repo file was written** other than this report and the `gh-log.md`
  entries (that file is git-ignored). No commit, no push, no tag, no
  release, no `ci.yml` touch, no config edit.
- **gh-log.md**: 11 entries appended for this task, covering the backfilled
  pre-dispatch state check, the two precondition reads, the dispatch, the
  id resolution, the watch, the four run reads, the two API reads, the
  download, the post-run release list, and the run-B non-dispatch decision.
  `git ls-remote` needed none (read-only git); `gh --version` was a local
  version probe of the same class as `--help`.
- **Counts recomputed, not carried**: 4 `pick:` lines, 1 / 4 `ls -l`
  entries, 2 artifacts, 5 of 7 files, 2 of 4 updater lines, 4 tar.gz files,
  8 `__VERSION__` occurrences in the template - each recomputed from the
  enumeration quoted above it.
- **Quotes**: every transcript block above is copied from the run log,
  command output, or a file, never reconstructed. The one place a mechanism
  is asserted without artifact evidence (the WiX code-page hypothesis) is
  labelled as unverified in its own sentence.
- **Scratch cleanup**: downloaded assets, the extracted AppImage
  (`squashfs-root`) and the extracted tar.gz were removed after evaluation;
  the log extracts were kept in the session scratchpad, outside the repo.

---
---

# RE-RUN (post-WiX-fix): the FULL rehearsal, R1-R10

**Status: DONE.** Everything above this line describes the first attempt on
`dbd0dc3`, which was BLOCKED by the WiX failure. This section supersedes it:
the full acceptance test ran on `c999090` with both Windows legs green.

- **Run A** (artifact path, `rehearse-draft-release` default `false`):
  id **30272619000** -> conclusion **success**, all 6 jobs green.
  https://github.com/senolfeldmann/Muxsmith/actions/runs/30272619000
- **Run B** (rehearsal path, `rehearse-draft-release=true`):
  id **30273529210** -> conclusion **success**, all 6 jobs green, draft
  `rehearsal-30273529210` created.
  https://github.com/senolfeldmann/Muxsmith/actions/runs/30273529210
- **R1-R10: 8 of 8 machine-verifiable observables PASS.** R8 and R10 are
  owner steps by design and are pending-owner (not failures).
- Both runs on head SHA `c9990908749fcf5bc720ab7fce4335aed8a48315`
  (`= origin/master`), workspace version `0.1.0` (awk-parsed).
- **The draft is NOT deleted** (controller ruling this dispatch; design R10;
  preamble line 11). Nothing published, un-drafted, edited, or tagged.

## What changed since the first attempt

Master carries the fix, `07c0255` + `c999090`, which I read before
dispatching: `bundle.windows.wix.language` moved from `["en-US"]` to
`{ "en-US": { "localePath": "wix/locale-en-US.wxl" } }`, and the new
`src-tauri/wix/locale-en-US.wxl` sets `WixLocalization/@Codepage="1254"`
plus `<String Id="TauriCodepage">1254</String>`. **My U+015E hypothesis was
confirmed and corrected in one respect that matters:** the sink set was
`publisher` (main.wxs Manufacturer, 8 hits) **plus the LICENSE text WiX
inlines into WixUIExtension's LicenseAgreementDlg** (1 hit) - `copyright`
never reaches WiX at all. So the D86 publisher-only fallback I flagged as
possibly insufficient (memo 3) provably was insufficient: the fix commit
records that ASCII-izing publisher and copyright still failed on the
LICENSE text. No fallback was applied; `Ş` survives on every platform.
**Memo 3 is therefore resolved on the merits, not deferred** - the D86
fallback was never the right instrument for this failure.

Preconditions re-verified before dispatch: `HEAD == origin/master ==
c999090`, clean tree, and the ci gate green - the push-triggered ci run
`30272193732` was still `in_progress`, so it was waited out foreground
(`timeout 570 gh run watch 30272193732 --exit-status --interval 30`, exit 0)
rather than letting the release guard poll an unfinished gate. Baseline
re-established at **zero releases including drafts**, via the API endpoint
that shows drafts to a push-authorized token, not merely by empty
`gh release list` output:

```
$ gh api repos/senolfeldmann/Muxsmith/releases --jq 'length, (.[] | {name, draft, tag_name})'
0
```

R6's tool gate is now clean: `dpkg-deb` = `/usr/bin/dpkg-deb` (Debian
1.23.7), `msiextract` = `/usr/bin/msiextract` (0.106.58-a155), `msiinfo`
also present; `rpm` 6.0.1, `sha256sum`, `tar` as before.

## Re-run verdict table

| R | Verdict | Emitter that produced it |
|---|---------|--------------------------|
| R1 | **PASS** | run A guard log; 6 `pick:` lines at 1/1/1/3; four closing `ls -l` at 1/1/1/4 |
| R2 | **PASS** | 4 artifacts, retention 7; 7 downloaded files, all D89-conformant |
| R3 | **PASS at its named emitter** | run A assemble job ran; release step conclusion `skipped`. Cross-run positive control: the same step is `success` in run B |
| R4 | **PASS** | draft `rehearsal-30273529210`, `isDraft: true`, 8 assets, `sha256sum -c` exit 0, control fired and restored |
| R5 | **PASS** | body = banner + substituted template + generated notes, 0 `__VERSION__` survivors (grep fire-verified at 8 on the template); 4 of 4 `updater-artifact check:` lines, each N > 0 |
| R6 | **PASS, all 5 checks** | deb, rpm, AppImage, tar.gz, both msi |
| R7 | **PASS** | `git ls-remote origin` after run B: 0 `rehearsal`, master present, 0 `refs/tags/`; two positive controls |
| R8 | **PENDING OWNER** | real hardware; inputs now exist for all three walk-throughs |
| R9 | **PASS** | `muxsmith 0.1.0` from both runs' tar.gz == awk-parsed `0.1.0` |
| R10 | **PENDING OWNER** | draft deliberately preserved |

## R1 - PASS

Guard, both steps, from run A's log:

```
guard	Version sync (D87; tag arm only on the tag path)	2026-07-27T13:56:10.8022078Z version-sync: OK (0.1.0)
guard	Require the ci gate green on this SHA (D83)	2026-07-27T13:56:11.6150890Z ci gate green for c9990908749fcf5bc720ab7fce4335aed8a48315
```

Run B's guard is identical in shape (`version-sync: OK (0.1.0)`;
`ci gate green for c9990908749fcf5bc720ab7fce4335aed8a48315`), so the
guard's green path is reproduced twice. The R1 sub-finding from the first
attempt stands unchanged and is design-side, not workflow-side: the step
echoes only `$GITHUB_SHA`, so its log names the **SHA**, never a run id;
"names the found ci run" is satisfiable only under that reading.

All `pick:` lines in run A (grep `'^bundle.*Z pick: '`, which excludes the
echoed script body):

```
bundle (linux-x86_64, ...)	...	2026-07-27T14:04:21.6059964Z pick: target/release/bundle/deb/Muxsmith_0.1.0_amd64.deb
bundle (linux-x86_64, ...)	...	2026-07-27T14:04:21.6065964Z pick: target/release/bundle/rpm/Muxsmith-0.1.0-1.x86_64.rpm
bundle (linux-x86_64, ...)	...	2026-07-27T14:04:21.6072473Z pick: target/release/bundle/appimage/Muxsmith_0.1.0_amd64.AppImage
bundle (windows-arm64, windows-11-arm, msi)	...	2026-07-27T14:03:22.2028045Z pick: target/release/bundle/msi/Muxsmith_0.1.0_arm64_en-US.msi
bundle (macos-arm64, macos-15, dmg)	...	2026-07-27T13:59:41.2371420Z pick: target/release/bundle/dmg/Muxsmith_0.1.0_aarch64.dmg
bundle (windows-x86_64, windows-2025, msi)	...	2026-07-27T14:06:22.6039749Z pick: target/release/bundle/msi/Muxsmith_0.1.0_x64_en-US.msi
```

Per-leg count, recomputed from that enumeration - **1 / 1 / 1 / 3, total 6**,
exactly R1's requirement:

```
      3 bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)
      1 bundle (macos-arm64, macos-15, dmg)
      1 bundle (windows-arm64, windows-11-arm, msi)
      1 bundle (windows-x86_64, windows-2025, msi)
```

Run B reproduces the same 1/1/1/3 distribution. Note the msi native names
still end `_en-US.msi`: the language key stayed `en-US`, so D89's asset
names are unaffected by the fix, as the controller stated and the rename
output below confirms.

Closing `ls -l` per leg - **1 / 1 / 1 / 4 incl. the tar.gz**:

```
=== windows-x86_64 ===
total 6640
-rw-r--r-- 1 runneradmin 197121 6799360 Jul 27 14:06 muxsmith-0.1.0-windows-x86_64.msi
=== windows-arm64 ===
total 6196
-rw-r--r-- 1 runneradmin 197121 6344704 Jul 27 14:03 muxsmith-0.1.0-windows-arm64.msi
=== macos-arm64 ===
total 13752
-rw-r--r--@ 1 runner  staff  7039815 Jul 27 13:59 muxsmith-0.1.0-macos-arm64.dmg
=== linux-x86_64 ===
total 106052
-rwxr-xr-x 1 runner runner 84892152 Jul 27 14:04 muxsmith-0.1.0-linux-x86_64.AppImage
-rw-r--r-- 1 runner runner  7963728 Jul 27 14:04 muxsmith-0.1.0-linux-x86_64.deb
-rw-r--r-- 1 runner runner  7964746 Jul 27 14:04 muxsmith-0.1.0-linux-x86_64.rpm
-rw-r--r-- 1 runner runner  7769436 Jul 27 14:04 muxsmith-0.1.0-linux-x86_64.tar.gz
```

## R2 - PASS

```
$ gh api "repos/senolfeldmann/Muxsmith/actions/runs/30272619000/artifacts" --jq '.total_count, (.artifacts[] | {name, created_at, expires_at, size_in_bytes})'
4
{"created_at":"2026-07-27T14:06:23Z","expires_at":"2026-08-03T14:06:22Z","name":"muxsmith-windows-x86_64","size_in_bytes":6569608}
{"created_at":"2026-07-27T14:04:27Z","expires_at":"2026-08-03T14:04:22Z","name":"muxsmith-linux-x86_64","size_in_bytes":107696994}
{"created_at":"2026-07-27T14:03:23Z","expires_at":"2026-08-03T14:03:22Z","name":"muxsmith-windows-arm64","size_in_bytes":6129667}
{"created_at":"2026-07-27T13:59:43Z","expires_at":"2026-08-03T13:59:41Z","name":"muxsmith-macos-arm64","size_in_bytes":6996440}
```

`total_count` **4**; all four names are `muxsmith-<leg id>` over D85's leg
ids; `expires_at` is 2026-08-03 against `created_at` 2026-07-27 on every
one, i.e. `retention-days: 7` exactly.

`gh run download 30272619000 --dir <scratch>/runA` -> **7** files:

```
muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.AppImage
muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.deb
muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.rpm
muxsmith-linux-x86_64/muxsmith-0.1.0-linux-x86_64.tar.gz
muxsmith-macos-arm64/muxsmith-0.1.0-macos-arm64.dmg
muxsmith-windows-arm64/muxsmith-0.1.0-windows-arm64.msi
muxsmith-windows-x86_64/muxsmith-0.1.0-windows-x86_64.msi
```

Against D89 with the version token recomputed from `Cargo.toml` (`0.1.0`),
all 7 present and character-for-character conformant, `.AppImage`
capitalization included:

| # | D89 name | Present |
|---|----------|---------|
| 1 | `muxsmith-0.1.0-windows-x86_64.msi` | yes |
| 2 | `muxsmith-0.1.0-windows-arm64.msi` | yes |
| 3 | `muxsmith-0.1.0-macos-arm64.dmg` | yes |
| 4 | `muxsmith-0.1.0-linux-x86_64.deb` | yes |
| 5 | `muxsmith-0.1.0-linux-x86_64.rpm` | yes |
| 6 | `muxsmith-0.1.0-linux-x86_64.AppImage` | yes |
| 7 | `muxsmith-0.1.0-linux-x86_64.tar.gz` | yes |

Count check: **7**.

## R3 - PASS at its named emitter

Run A's assemble job ran and the release step reports its own `skipped`
conclusion, which is precisely the positive observation the design demands
in place of an absence-grep:

```
$ gh api "repos/senolfeldmann/Muxsmith/actions/runs/30272619000/jobs" --jq '.jobs[] | select(.name=="assemble") | {name, conclusion, steps: [.steps[] | {name, conclusion}]}'
{"conclusion":"success","name":"assemble","steps":[{"conclusion":"success","name":"Set up job"},{"conclusion":"success","name":"Run actions/checkout@..."},{"conclusion":"success","name":"Run actions/download-artifact@..."},{"conclusion":"success","name":"Generate SHA256SUMS (D90)"},{"conclusion":"skipped","name":"Create draft release (tag path) or rehearsal draft (D77/D79)"},{"conclusion":"success","name":"Post Run actions/checkout@..."},{"conclusion":"success","name":"Complete job"}]}
```

**Cross-run positive control** (stronger than the design asked for): the
same step in run B reports `{"conclusion":"success","name":"Create draft
release (tag path) or rehearsal draft (D77/D79)"}`. So run A's `skipped` is
produced by the step's `if: github.ref_type == 'tag' ||
inputs.rehearse-draft-release` evaluating false, not by a permanently dead
step - the two runs differ in exactly the one input that gates it.

Corroboration: `gh release list` empty and `gh api .../releases --jq length`
-> `0` after run A, identical to baseline.

**D90 content check on run A's SHA256SUMS step** (the step ran even though
the release step did not, so its output is readable):

```
a4bff6c889c051cf258536007157a0fccd8e26a275f5964d6e80c50c969b3084  muxsmith-0.1.0-linux-x86_64.AppImage
bf5fa73b8119aae55e04075be43f4f63d53e3cc579e206bd714301ff8a21ae7a  muxsmith-0.1.0-linux-x86_64.deb
e2b22b3ba90d3ddc985fe645f3c8ad146c9ddf4e79c2ae4e0545d0ee156244c9  muxsmith-0.1.0-linux-x86_64.rpm
8ecd1982db8f5c6affd04da3e752dc5939a0cb124585e8caa7d6135623ff2457  muxsmith-0.1.0-linux-x86_64.tar.gz
4f53d130477a90ae0d3bef4fe6ddc248e035ca22f2f65c6f9c401ce393a9ec10  muxsmith-0.1.0-macos-arm64.dmg
aef98f73c45d616bd60bb3109e586b4b5689f5103f026c6bf14c38ae362dd685  muxsmith-0.1.0-windows-arm64.msi
7a23f7f04d149861a844e71c592c69de59c93339d504fa54f2a24ffbf1ed1ada  muxsmith-0.1.0-windows-x86_64.msi
```

7 lines, filenames exactly D89's, default `sha256sum` two-space format, and
no self-entry - the glob expands before the redirect creates the file, as
D90 requires ("generated after the rename, never before").

## R4 - PASS

```
$ gh release view "rehearsal-30273529210" --json isDraft,name,tagName,targetCommitish,assets --jq '{isDraft, name, tagName, targetCommitish, assetCount: (.assets|length), assets: [.assets[].name]}'
{"assetCount":8,"assets":["muxsmith-0.1.0-linux-x86_64.AppImage","muxsmith-0.1.0-linux-x86_64.deb","muxsmith-0.1.0-linux-x86_64.rpm","muxsmith-0.1.0-linux-x86_64.tar.gz","muxsmith-0.1.0-macos-arm64.dmg","muxsmith-0.1.0-windows-arm64.msi","muxsmith-0.1.0-windows-x86_64.msi","SHA256SUMS"],"isDraft":true,"name":"REHEARSAL - not a release (run 30273529210)","tagName":"rehearsal-30273529210","targetCommitish":"c9990908749fcf5bc720ab7fce4335aed8a48315"}
```

`isDraft: true`; asset count exactly **8** = D89's 7 + `SHA256SUMS`; name
carries the REHEARSAL marker; `tagName` is the rehearsal name and
`targetCommitish` the dispatch SHA (the `--target` arm, no tag).

Download - the tagless draft resolved on the **primary** form, so the
brief's `gh api` fallback was not needed and no third variant was invented:

```
$ gh release download "rehearsal-30273529210" --dir <scratch>/runB      # exit 0, 8 files
$ sha256sum -c SHA256SUMS
muxsmith-0.1.0-linux-x86_64.AppImage: OK
muxsmith-0.1.0-linux-x86_64.deb: OK
muxsmith-0.1.0-linux-x86_64.rpm: OK
muxsmith-0.1.0-linux-x86_64.tar.gz: OK
muxsmith-0.1.0-macos-arm64.dmg: OK
muxsmith-0.1.0-windows-arm64.msi: OK
muxsmith-0.1.0-windows-x86_64.msi: OK
check-exit=0
```

**Falsifiability control, run once.** One byte of the arm64 msi flipped at
offset 4096:

```
$ xxd -s 4096 -l 1 muxsmith-0.1.0-windows-arm64.msi
00001000: fd                                       .
$ printf '\xAA' | dd of=muxsmith-0.1.0-windows-arm64.msi bs=1 seek=4096 count=1 conv=notrunc
$ xxd -s 4096 -l 1 muxsmith-0.1.0-windows-arm64.msi
00001000: aa                                       .
$ sha256sum -c SHA256SUMS
muxsmith-0.1.0-linux-x86_64.AppImage: OK
muxsmith-0.1.0-linux-x86_64.deb: OK
muxsmith-0.1.0-linux-x86_64.rpm: OK
muxsmith-0.1.0-linux-x86_64.tar.gz: OK
muxsmith-0.1.0-macos-arm64.dmg: OK
muxsmith-0.1.0-windows-arm64.msi: FAILED
muxsmith-0.1.0-windows-x86_64.msi: OK
sha256sum: WARNING: 1 computed checksum did NOT match
check-exit=1
```

The check named exactly the corrupted file, left the other six OK, and
exited 1 - so the earlier all-OK result is a real measurement, not a check
that cannot fail. Restored by redownloading that one asset (`--pattern ...
--clobber`); byte back to `fd`, `-c` exit 0 again.

## R5 - PASS

Body, verbatim from `gh release view "rehearsal-30273529210" --json body`,
in the mandated composition order - banner, then template, then generated
notes:

```
> **REHEARSAL DRAFT - not a release.** Created by a workflow_dispatch
> rehearsal run to exercise draft-release assembly (D79). No git tag
> exists for this draft. Inspect body, assets and checksums, then
> delete this draft.

---
Muxsmith 0.1.0 - unsigned builds; read the install note for your OS
before first launch: [Windows](...INSTALL.md#windows)
| [macOS](...INSTALL.md#macos)
| [Linux](...INSTALL.md#linux)

**Runtime requirement:** Muxsmith drives `mkvmerge` from
[MKVToolNix](https://mkvtoolnix.download/). The deb/rpm packages declare
it as a recommended dependency; on Windows/macOS install it yourself
(details in the install notes).

| Artifact | For |
|---|---|
| `muxsmith-0.1.0-windows-x86_64.msi` | Windows 10/11, Intel/AMD |
| `muxsmith-0.1.0-windows-arm64.msi` | Windows 11 on ARM |
| `muxsmith-0.1.0-macos-arm64.dmg` | macOS 11+, Apple Silicon |
| `muxsmith-0.1.0-linux-x86_64.deb` | Debian/Ubuntu |
| `muxsmith-0.1.0-linux-x86_64.rpm` | Fedora & co. |
| `muxsmith-0.1.0-linux-x86_64.AppImage` | any Linux distro |
| `muxsmith-0.1.0-linux-x86_64.tar.gz` | portable, CLI + GUI |

Verify downloads: put `SHA256SUMS` beside the files and run
`sha256sum -c SHA256SUMS`.

---
**Full Changelog**: https://github.com/senolfeldmann/Muxsmith/commits/rehearsal-30273529210
```

(The INSTALL.md URLs are shown abbreviated here only to keep the block
narrow; in the body they are the full `blob/master/docs/INSTALL.md#...`
links.) Every `__VERSION__` token is substituted - the version appears as
`0.1.0` in the lead line and in all seven table rows:

```
$ grep -c '__VERSION__' <body>              -> 0
$ grep -c '__VERSION__' .github/release/draft-body.md   -> 8
```

The `0` is fire-verified: the identical pattern returns **8** against the
committed template, so it demonstrably fires on a positive case. The
generated-notes section is the `Full Changelog` line - thin because the
repo has no prior release to diff against, which is expected, and it proves
the `generate-notes` API endpoint was used (the link targets the rehearsal
name, not a tag).

Updater-absence half, all four leg logs of run B:

```
bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)	Assert no updater artifacts were produced (D76)	2026-07-27T14:14:33.0785630Z updater-artifact check: 0 hits across 250 bundle output files
bundle (windows-x86_64, windows-2025, msi)	Assert no updater artifacts were produced (D76)	2026-07-27T14:16:00.4517732Z updater-artifact check: 0 hits across 1 bundle output files
bundle (macos-arm64, macos-15, dmg)	Assert no updater artifacts were produced (D76)	2026-07-27T14:13:23.6354440Z updater-artifact check: 0 hits across        5 bundle output files
bundle (windows-arm64, windows-11-arm, msi)	Assert no updater artifacts were produced (D76)	2026-07-27T14:14:35.7653040Z updater-artifact check: 0 hits across 1 bundle output files
```

Count: **4**, one per leg. Every N > 0 (250, 1, 5, 1), so the step's
built-in positive control - proof it saw a populated bundle tree rather
than passing vacuously - fired on all four. Run A's four lines are
identical in shape (250 / 1 / 5 / 1). The `wc -l` padding in the macOS
value is BSD `wc`, cosmetic.

## R6 - PASS, all five checks

Run on run A's downloaded artifacts (the brief allows run A or B); the
tar.gz check was repeated on run B's asset with an identical listing.

**deb**, `dpkg-deb -I` - `Recommends: mkvtoolnix` and both required
`Depends:` package names present:

```
 Package: muxsmith
 Version: 0.1.0
 Architecture: amd64
 Installed-Size: 24188
 Maintainer: Şenol Feldmann
 Section: video
 Priority: optional
 Homepage: https://github.com/senolfeldmann/Muxsmith
 Depends: libwebkit2gtk-4.1-0, libgtk-3-0
 Recommends: mkvtoolnix
 Description: Rule-based bulk MKV muxing tool
```

`Maintainer: Şenol Feldmann` decodes correctly, incidentally confirming the
code-page problem was WiX-specific and never touched the Linux packages.

**deb payload**, `dpkg-deb -c`. **A false-empty was caught here and is
worth recording**: the design's literal `./usr/bin/muxsmith` does not
appear in dpkg 1.23.7's output, which emits paths without the `./` prefix,
so a naive grep for the design's string returns nothing. Verified rather
than trusted:

```
$ dpkg-deb -c ...deb | grep -cF './usr/bin/'
0
$ dpkg-deb -c ...deb | awk '{print $NF}' | grep -E '^(\./)?usr/bin/'
usr/bin/muxsmith-gui
usr/bin/muxsmith
$ dpkg-deb -c ...deb | awk '{print $NF}' | grep -cE '^(\./)?usr/bin/mkvmerge'   # negative control
0
```

Both binaries are in the payload; the pattern is proven to discriminate
(it finds the two that exist and not a path that does not). The `./`
difference is a **cosmetic mismatch in the design's R6 text against
dpkg >= 1.22-era output**, not an artifact defect - flagged as a one-line
doc item, nothing more.

**rpm**:

```
$ rpm -qp --recommends muxsmith-0.1.0-linux-x86_64.rpm
mkvtoolnix
```

**AppImage**:

```
$ chmod +x muxsmith-0.1.0-linux-x86_64.AppImage
$ ./muxsmith-0.1.0-linux-x86_64.AppImage --appimage-extract     # exit 0
$ ls -l squashfs-root/usr/bin/
-rwxr-xr-x. 1 senol senol  6380680 Jul 27 16:02 muxsmith
-rwxr-xr-x. 1 senol senol 18349232 Jul 27 16:02 muxsmith-gui
```

`squashfs-root/usr/bin/muxsmith` present.

**tar.gz** - D88's four-file layout under the version-named directory:

```
drwxr-xr-x runner/runner     0 2026-07-27 16:04 muxsmith-0.1.0-linux-x86_64/
-rwxr-xr-x runner/runner 18330264 2026-07-27 16:04 muxsmith-0.1.0-linux-x86_64/muxsmith-gui
-rw-r--r-- runner/runner     1072 2026-07-27 16:04 muxsmith-0.1.0-linux-x86_64/LICENSE
-rw-r--r-- runner/runner     1088 2026-07-27 16:04 muxsmith-0.1.0-linux-x86_64/README.txt
-rwxr-xr-x runner/runner  6374008 2026-07-27 16:04 muxsmith-0.1.0-linux-x86_64/muxsmith
```

5 entries = 1 directory + **4** files, recounted: `muxsmith`,
`muxsmith-gui`, `LICENSE`, `README.txt`.

**msi, both legs** - `msiextract` on each lists both executables:

```
=== windows-x86_64 ===          exit 0
PFiles/Muxsmith/muxsmith.exe
PFiles/Muxsmith/muxsmith-gui.exe
=== windows-arm64 ===           exit 0
PFiles/Muxsmith/muxsmith.exe
PFiles/Muxsmith/muxsmith-gui.exe
```

`muxsmith.exe` is the D82 sidecar CLI with its target-triple suffix
stripped, beside the GUI, exactly as design 1.2 recorded.

**Bonus check on the fix, not an R-observable** (`msiinfo`, now installed) -
the publisher survives the 1254 code page in the Property table of both
msi, which is the data R8's Programs-and-Features reading depends on:

```
$ msiinfo export <msi> Property | grep -iE 'Manufacturer|ProductName|ProductVersion'
Manufacturer	Şenol Feldmann
ProductName	Muxsmith
ProductVersion	0.1.0
```

`msiinfo suminfo` shows `Author: <0xDE>enol Feldmann` (the single 1254 byte,
rendered as a replacement char by a UTF-8 terminal - not mojibake in the
file), `Template: x64;0` / `Arm64;0`, and
`Application: Windows Installer XML Toolset (3.14.1.8722)`, matching design
1.2's pinned WiX 3.14.1. This corroborates the controller's verification
claim independently; it does **not** replace R8, which still needs the real
Programs-and-Features rendering on Windows.

## R7 - PASS (now the real run-B instance)

Taken **after** run B created the draft, which is what makes it meaningful:

```
$ git ls-remote origin
c9990908749fcf5bc720ab7fce4335aed8a48315	HEAD
c9990908749fcf5bc720ab7fce4335aed8a48315	refs/heads/master
$ grep -c rehearsal <out>          -> 0
$ grep -c 'refs/heads/master' <out> -> 1
$ grep -c 'refs/tags/' <out>        -> 0
```

Two positive controls, so the `0` is a measurement: the transcribed one
(the same output demonstrably lists `refs/heads/master`), plus a second -
the identical `grep -c rehearsal` returns **1** when a synthetic
`deadbeef refs/tags/rehearsal-30273529210` line is appended to the same
data, proving the pattern would find exactly the ref it is looking for.
A draft release with a `tagName` therefore creates no tag ref, as D79
requires.

## R8 - PENDING OWNER (inputs now exist)

All three walk-through inputs are available for the first time:
`muxsmith-0.1.0-windows-x86_64.msi`, `muxsmith-0.1.0-windows-arm64.msi`,
`muxsmith-0.1.0-macos-arm64.dmg` - on the preserved draft
`rehearsal-30273529210` and as run-A/B workflow artifacts until
2026-08-03. Steps unchanged: msi install after the SmartScreen "Run anyway"
flow per `docs/INSTALL.md` plus the Programs-and-Features publisher
rendering; dmg via Settings > Open Anyway on Apple Silicon; `muxsmith` from
the documented per-OS CLI locations.

**D86 publisher-fallback protocol, superseded on the merits.** The fallback
was pre-decided as the answer to a publisher-rendering failure. The fix
commit records that ASCII-izing publisher (and copyright) does **not** fix
the WiX build, because the LICENSE text carries the same character - so if
R8's publisher rendering ever came out mangled, the pre-decided
`publisher -> "Senol Feldmann"` edit is not the instrument; the code page
is. Recorded for the plan close as a design bookkeeping item; no action
taken here.

## R9 - PASS

```
$ tar xzf muxsmith-0.1.0-linux-x86_64.tar.gz -C <scratch> && cd muxsmith-0.1.0-linux-x86_64
$ ./muxsmith --version
muxsmith 0.1.0
$ awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml
0.1.0
```

Verified on **both** runs' tar.gz (run A's artifact and run B's release
asset), identical output. D87's artifact-level criterion holds.

## R10 - PENDING OWNER; draft preserved

`rehearsal-30273529210` is left in place, `isDraft: true`, 8 assets, no tag.
The controller's ruling this dispatch settles the conflict that memo 2
raised in favour of the plan text: the owner inspects for R8, then deletes
at plan close. Nothing was published, edited, or un-drafted.

## Method notes (re-run)

- **Foreground only, no background-run-plus-monitor.** Every wait was a
  `timeout 570 gh run watch <id> --exit-status --interval 30` re-entered as
  needed (this harness caps one Bash call at 10 min, so the brief's single
  `timeout 5400` cannot be expressed in one call). One watch of run A
  dropped with `error connecting to api.github.com` - a transient client
  fault, not a run failure; state was re-established with
  `gh run view --json status,conclusion` (`in_progress`) and the watch
  re-entered. **Nothing was ever re-dispatched.**
- **Order kept**: run A completed before run B was dispatched. Run A's
  evaluation was done while run B built, which changes no ordering.
- **No repo file written** other than this report and `gh-log.md` (both
  git-ignored; `git status --porcelain` empty throughout). No commit, no
  push, no tag, no `ci.yml` touch, no config edit, no release published or
  deleted.
- **Absence results fire-verified, not trusted**: the `__VERSION__` grep
  (0 on the body, 8 on the template), the checksum check (byte flipped,
  named failure observed, restored), the `rehearsal` ref grep (two positive
  controls), the deb payload grep (a genuine false-empty caught and
  corrected with a negative control added).
- **Counts recomputed** from the enumerations quoted above them: 6 `pick:`
  lines at 1/1/1/3, `ls -l` at 1/1/1/4, 4 artifacts, 7 downloaded files,
  7 SHA256SUMS lines, 8 draft assets, 4 updater lines, 4 tar.gz files,
  8 `__VERSION__` occurrences in the template, 2 refs from ls-remote.
- **Quotes**: every transcript block is copied from a run log, command
  output, or a file.
- **gh-log.md**: 12 further entries for the re-run, including the
  `--clobber` failure, the transient watch drop, and an explicit entry
  recording that the draft is deliberately not deleted.
- **Scratch cleanup**: all downloaded assets and extractions removed; the
  draft and both runs' workflow artifacts remain (the owner's R8 inputs).
