# Task 4 report: Rehearsal re-run + the machine halves (RR1-RR6)

**Run:** https://github.com/senolfeldmann/Muxsmith/actions/runs/30312889098
**Conclusion:** completed / **success**, all 6 jobs green.
**Draft left standing for the owner:** `REHEARSAL - not a release (run 30312889098)`, tag name `rehearsal-30312889098` (the *release object's* tag field; no git tag ref exists - see RR6).
**Repo files written:** none. `gh-log.md` updated (git-ignored, not repo content). No commit, no tag, no publish, no release edit, no release deletion.

**Verdict: all six machine halves green.** No NEEDS_CONTEXT return. One plan-text discrepancy (RR1's expected count, see below) is reported rather than swallowed; it is a documentation artifact, not an artifact defect.

## Preconditions (verified, not redone)

| # | Precondition | Command | Observed |
|---|---|---|---|
| 1 | Tasks 1-3 committed | `git log --oneline 8ad4392` | `9460daf` (T1), `5060ef5` (T1 review fix), `50e08cd` (T2), `87c1dee` (T3) all ancestors of the pushed head |
| 1 | Tree clean | `git status --porcelain` | empty |
| 1 | Ten-part gate green | ROADMAP "Test flakiness" section | present: one flake on the first `cargo test --workspace`, green on the complete re-run, three further green runs |
| 2 | master pushed | `git rev-parse origin/master` | `8ad4392c4094b1426267a3c517b35554eba0a83e` |
| 3 | ci green incl. ledger-lint | `gh run view 30312472265 --json ...` + `--json jobs` | `ci`, event `push`, headSha `8ad4392`, completed/**success**; 5 jobs (ubuntu-26.04, macos-15, windows-2025, deny, **ledger-lint**) all success |

**Observation worth the controller's eye (not a blocker):** local `master` was **one commit ahead** of `origin/master` at dispatch time - `61dc522` "roadmap: flake is a ruled 1.x fix, with its candidate named". Measured: `git diff --name-only origin/master..HEAD` -> `docs/ROADMAP.md` only. The dispatch resolves `--ref master` on the remote, and the run's `headSha` came back `8ad4392` - i.e. exactly the SHA that ci run 30312472265 proved green. So the unpushed commit is controller bookkeeping that is correctly not in play; no task artifact and no release collateral is affected. Flagged only so nobody later reads "master was ahead" as an unverified rehearsal base.

## Step 1: dispatch and watch

- `gh workflow run release.yml --ref master -f rehearse-draft-release=true` -> run **30312889098** (the dispatch printed the run URL directly; the plan's bounded run-id poll was therefore unnecessary, and `gh run list --workflow release.yml --limit 1` confirmed the identity: event `workflow_dispatch`, branch `master`, headSha `8ad4392`).
- **One dispatch run only**, as the plan's closed fork requires. No second run, no re-dispatch.
- Watch was foreground with an explicit timeout, taken in two sequential `timeout 540 gh run watch 30312889098 --exit-status --interval 30` invocations because the harness caps a single foreground call below the plan's 5400 s budget. First returned exit 124 (cap reached, run still building), second returned **exit 0** on completion. Wall clock 01:03:43 -> 01:14:38 local, about 11 minutes, far inside the 5400 s budget. Nothing was backgrounded and no timeout was silently extended.
- Jobs (6/6 success): `guard`; `bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)`; `bundle (windows-x86_64, windows-2025, msi)`; `bundle (windows-arm64, windows-11-arm, msi)`; `bundle (macos-arm64, macos-15, dmg)`; `assemble`.

Version token recomputed from the tree, never hardcoded: `V="$(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml)"` -> **0.1.0**, confirmed to be the `[workspace.package]` value (Cargo.toml:5-6).

## RR1: the pipeline still assembles - GREEN

| Element | Expected | Observed |
|---|---|---|
| Draft exists, `isDraft` | true | `true`, name `REHEARSAL - not a release (run 30312889098)`, created 2026-07-27T23:14:01Z |
| Asset count | exactly 8 | **8**: the 7 artifacts + `SHA256SUMS` |
| Download | primary `gh release download --dir <scratch>` | exit 0, all 8 files present; the `gh api` fallback was **not** needed (no second variant used) |
| `sha256sum -c SHA256SUMS` | passes | exit 0, **7** `: OK` lines (SHA256SUMS does not list itself) |
| Falsifiability control | `-c` fails naming the file | one byte flipped at offset 1000 of the deb (`dd ... conv=notrunc`) -> `muxsmith-0.1.0-linux-x86_64.deb: FAILED`, `WARNING: 1 computed checksum did NOT match`, exit 1, **all six other files still OK**; re-downloaded that one asset -> clean verify restored, exit 0 |
| D76 updater assertion per leg | `0 hits across N`, N > 0, in each of the four legs | 4 executed lines, one per distinct bundle leg: linux-x86_64 `0 hits across 250`, windows-x86_64 `0 hits across 1`, windows-arm64 `0 hits across 1`, macos-arm64 `0 hits across 5` |

**Discrepancy against the plan text, reported not swallowed.** The plan expects `grep -c "updater-artifact check:"` on `gh run view <id> --log` to print **4**. It printed **8**. Decomposed with two disjoint invocations:

- `grep -c 'updater-artifact check: 0 hits across \$bundles_found'` -> **4**. These are the runner's echo of the step's *script source* (the log group header prints the shell source, so `$bundles_found` appears unexpanded, followed by the ANSI reset sequence).
- `grep -cE 'updater-artifact check: 0 hits across[[:space:]]+[0-9]+ bundle output files$'` -> **4**. These are the executed outputs, and `cut -f1 | sort -u` on them returns exactly the four distinct bundle-leg job names listed in the table above.

4 + 4 = 8, fully accounted for. The substantive condition RR1 asserts (each of the four legs asserting zero updater artifacts across a non-zero file count) is verified per leg by direct inspection. I did not treat this as NEEDS_CONTEXT: the check is evaluable, its cause is measured rather than assumed, and no unenumerated tool shape or exhausted fallback chain is involved. It is worth a plan-text fix if this check is ever re-transcribed.

## RR2: the draft body - GREEN (machine half; acceptance is O3)

Body fetched with `gh release view rehearsal-30312889098 --json body --jq '.body'` (25 lines, 1711 bytes).

| Check | Expected | Observed |
|---|---|---|
| Banner block first | banner then `---` | 4 `^> ` lines, first line `> **REHEARSAL DRAFT - not a release.** ...`, then `---` |
| Title line with the Task-1 wording AND all three anchors on that one line | 1 | **1** (single regex requiring `^Muxsmith 0.1.0 - builds carry no developer identity; read the install note for your OS before first launch:` followed by `#windows`, `#macos`, `#linux` in order on the same line) |
| `INSTALL.md#` anchors | 3, all on one line | `grep -o` gives exactly one each of `#windows`, `#macos`, `#linux`; `grep -c` (line count) = 1, i.e. all three sit on a single line |
| `grep -cE '^\| \['` | 0 | **0**. **Fired:** the identical invocation against the pre-Task-3 template (`git show 87c1dee^:.github/release/draft-body.md`) -> **2** |
| `grep -c __VERSION__` | 0 | **0**. **Fired:** the identical invocation against the committed template `.github/release/draft-body.md` -> **8** |
| Generated notes after the `---` | present | `^---$` count 2 (post-banner and pre-notes); `**Full Changelog**: https://github.com/senolfeldmann/Muxsmith/commits/rehearsal-30312889098` present as the final line |

Both absence results are live: each was produced by an invocation that returned a hit against a known-present control on this machine, not merely by the plan's record of Task 3's pre-count.

## RR3: the bundle is sealed - GREEN (machine half; acceptance is O1)

`7z l muxsmith-0.1.0-macos-arm64.dmg` succeeded in **one stage** (the plan's primary form; the dmg listed as `Type = Dmg` with bundle paths directly, so the two-stage inner-HFS form was not needed and no third shape occurred).

| Check | Expected | Observed |
|---|---|---|
| `grep -c 'CodeResources' listing.txt` | >= 1 | **1**: `Muxsmith/Muxsmith.app/Contents/_CodeSignature/CodeResources`, 2658 bytes |
| positive control `grep -c 'MacOS/muxsmith'` | >= 1 | **2** (`MacOS/muxsmith`, `MacOS/muxsmith-gui`) - the listing does expose bundle paths |

**Differential control, beyond the plan's requirement:** the same two invocations run against the **R8 defect artifact** (run 30273529210's workflow artifact `muxsmith-macos-arm64`, i.e. the pre-signing dmg whose "damaged" refusal the owner walked through) give `CodeResources` = **0** and `_CodeSignature` = **0** with the same positive control `MacOS/muxsmith` = **2**. So the seal is a change introduced by this package, not a property the artifact always had.

**This is the machine half only.** Whether Gatekeeper now says "unidentified developer" instead of "damaged" is O1, on hardware. Not claimed here.

## RR4: no license resources in the dmg - GREEN (machine half; acceptance is O2)

| Check | Expected (clear branch) | Observed |
|---|---|---|
| `LC_ALL=C grep -ac LPic <dmg>` | 0 | **0** (grep exit 1) |
| **Planted-copy fire** (mandatory) | 1 | `command cp -f` the dmg, `printf 'LPic' >>` it, identical invocation -> **1** (grep exit 0); sizes 7034828 vs 7034832 confirm the plant; `command rm -f` removed the control, verified gone |
| Corroboration on the real R8 defect dmg | 2 (the plan's recorded plan-review measurement) | **2** - reproduced exactly, via `gh run download 30273529210 -n muxsmith-macos-arm64` while its retention lasts (expiry around 2026-08-03) |
| Extra: `STR#`, the other EULA resource type | - | R8 dmg **1**, new dmg **0** |

The absence is therefore triple-anchored: the check fires on a planted copy, it returns 2 on the artifact that actually had the defect, and the second resource type of the same EULA block moves 1 -> 0 in the same direction.

## RR5: the msi still carries its license text - GREEN

On `muxsmith-0.1.0-windows-x86_64.msi` (the plan's named target):

| Check | Expected | Observed |
|---|---|---|
| primary `LC_ALL=C grep -ac "Permission is hereby granted"` | >= 1 | **1** (exit 0) |
| in-file positive control `LC_ALL=C grep -ac "Muxsmith"` | >= 1 | **3** (exit 0) |

Primary passed at the first stage, so the `msiinfo export Control` / `msidump` fallback chain was **not entered** - no stage of it bottomed out at zero, and no check was passed on tool failure. Extra data point outside the plan's named check: the arm64 msi gives the same pair (1 and 3).

## RR6: no tag ref was created - GREEN

`git ls-remote origin` (read-only git; no gh-log entry owed) returns exactly two refs, both at `8ad4392`: `HEAD` and `refs/heads/master`.

| Check | Expected | Observed |
|---|---|---|
| `grep -c rehearsal` on the ls-remote output | 0 | **0** |
| positive control `grep -c 'refs/heads/master'` | >= 1 | **1** |
| **Planted-copy fire** (added, beyond the plan's transcribed shape) | 1 | a copy with a fake `refs/tags/rehearsal-30312889098` line appended -> **1** |
| `git ls-remote --tags origin` | no tags | **0 lines** |

The draft's `tagName` field reads `rehearsal-30312889098`, which is the release object's tag *name*, not a ref: the remote carries no tag ref at all.

## What this task does NOT claim

Per the plan's owner-steps section and the dispatch contract: **rulings 1, 2 and 3 are NOT verified by this report.** RR3, RR4 and RR2 are machine halves. The acceptance is the owner's, on Apple-Silicon hardware, restated verbatim from the plan for the close:

- **O1 (ruling 1 ACCEPTANCE):** download the rehearsal dmg **in a browser** (the browser sets `com.apple.quarantine`; `gh release download`/`curl` do NOT); confirm `xattr -p com.apple.quarantine /Applications/Muxsmith.app` prints a value before first launch - if it prints nothing the test cannot fire, re-download via browser. Install, double-click: EXPECTED the "unidentified developer"-class refusal with System Settings > Privacy & Security > **Open Anyway**, i.e. the flow `docs/INSTALL.md` documents; NOT "damaged". Complete Open Anyway once; the app launches. In the same pass, re-verify INSTALL.md's macOS section sentence-by-sentence against the observed flow (the ROADMAP blocker entry's "Whatever lands ..." duty, located by that quote); wording drift returns as an owner edit. Optional corroboration: `codesign --verify --deep --strict /Applications/Muxsmith.app` exits 0 and `codesign -dvv /Applications/Muxsmith.app 2>&1 | grep Signature=adhoc` hits.
- **O2 (ruling 2 ACCEPTANCE, clear branch):** opening the same dmg presents **no pre-mount license dialog** - straight to the drag-to-Applications window.
- **O3 (ruling 3 ACCEPTANCE):** the draft's **rendered** body on github.com shows the title plus the three OS links as one block, the runtime-requirement paragraph as one block, the checksum paragraph as one block, and the artifact table intact.
- **O4:** delete the rehearsal draft(s) after inspection. Measured for the owner: `gh release list` returns **exactly one** release object, this draft. No plan-8-era rehearsal draft is still standing, so O4 has a single item.

## Housekeeping

- **gh-log.md:** one section appended, `## 2026-07-28 (session 23) - Plan 8.5 Task 4: rehearsal re-run (RR1-RR6)`, with an entry per gh interaction (command, effect, manual web-UI equivalent): precondition re-verification, the dispatch, the identity check, the two watch invocations, run view + jobs, release view, the two release downloads, the body fetch, the run log, the R8 artifact download, and the release list. It closes with the explicit statement that nothing was published, edited, deleted or tagged.
- **Scratch:** downloaded assets (all 8 rehearsal artifacts plus the R8 dmg) removed after the checks; the small text outputs (listings, body, run log, ls-remote) were kept only for the duration of this report. The scratch lived outside the repo, so the repo tree was never touched by the downloads.
- **The draft REMAINS**, deliberately - it is the owner's inspection deliverable for O1-O3.
- **Repo state unchanged by this task:** `git status --porcelain` empty; HEAD still `61dc522`; `origin/master` still `8ad4392`.
