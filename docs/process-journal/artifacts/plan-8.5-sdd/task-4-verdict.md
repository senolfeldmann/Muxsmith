# Task 4 verdict: Rehearsal re-run + the machine halves (RR1-RR6)

**Graded:** the report at `.superpowers/sdd/plan-8.5/task-4-report.md`, the `gh-log.md` Task-4 section (:508-585), and the live state on GitHub. Task 4 wrote no repo file and made no commit, so there is no diff to grade.
**Reviewer:** independent (did not run the task, did not dispatch anything). Model tier: mid (Opus 5), per `proc-03-model-assignment`.
**Verdict: APPROVED.** No major findings. One minor finding in the report's prose (a "verbatim" claim that is not verbatim), three notes, two of which are plan-text defects rather than execution defects.

Ground truth used: the plan's Global Constraints, its "Owner steps (macOS hardware)" section O1-O4, and all of "## Task 4"; the plan-8 design D75/D76/D77/D79/D83/D86/D89 and its section 2 workflow fence; `docs/process-conventions.yaml` (`proc-03-model-assignment`, `proc-07-verify-against-source`, `proc-normative-count-recomputed`, `proc-latitude-clause-boundary`); `docs/ROADMAP.md` "Test flakiness". The report was read first and then verified rather than graded.

**Instrument independence.** Every measurement below was taken in `/tmp/rr-audit-PD2AR2esJvoN`, created with `mktemp -d /tmp/rr-audit-XXXXXXXXXXXX`. The suffix is random and was minted after the implementer's run finished, so no artifact, listing, control copy or scratch file of the implementer's could be reused by accident or by name collision. All release assets and the R8 defect artifact were re-downloaded from GitHub into that directory; nothing was read from the implementer's scratch.

---

## 1. The machine halves, re-measured on my own instrument

Everything the report asserts reproduces. Not one number diverges.

### RR3 - the bundle seal (ruling 1 machine half): CONFIRMED

`7z l` on my own copy of `muxsmith-0.1.0-macos-arm64.dmg` (7 034 828 bytes, sha256 `8d66e2af...43171d`, matching the release's own `SHA256SUMS` line):

| check | my value | report's value |
|---|---|---|
| `grep -c 'CodeResources' listing.txt` | **1** | 1 |
| the sealed path | `Muxsmith/Muxsmith.app/Contents/_CodeSignature/CodeResources`, **2658 bytes** | same path, 2658 bytes |
| `grep -c '_CodeSignature'` | **2** (the directory entry plus the file) | not reported |
| positive control `grep -c 'MacOS/muxsmith'` | **2** (`MacOS/muxsmith`, `MacOS/muxsmith-gui`) | 2 |

**Differential against the R8 defect artifact** (run 30273529210, artifact `muxsmith-macos-arm64`, re-downloaded independently; retention still open on 2026-07-28):

| check | R8 dmg | new dmg |
|---|---|---|
| `CodeResources` | **0** | **1** |
| `_CodeSignature` | **0** | **2** |
| positive control `MacOS/muxsmith` | **2** | **2** |

The control returns the same non-zero value on both artifacts, so the zero on the R8 side is an absence in the artifact, not a failure of the listing to expose bundle paths. My `CodeResources = 0` on the R8 dmg also reproduces the plan-review round-1 record (`plan-review-round-1.md:351`: "dmg LPic=2, listing CodeResources=0") taken on a different day with a different scratch. Two instruments, two dates, identical numbers.

**Two checks beyond the plan, because the presence of a file named `CodeResources` is a weak claim on its own:**

1. *Is the seal real?* I extracted it and read it. It is a `codesign`-shaped plist: `files` (SHA-1 digest of `Resources/icon.icns`), `files2` (a `hash2` SHA-256 for the same resource, and a `cdhash` + `requirement` pair `cdhash H"c713e92b3a1870dd5fa4d66420413f8975ca2882"` for `MacOS/muxsmith`), plus the standard `rules`/`rules2` regex sets (`^Resources/`, `^Resources/.*\.lproj/`, and so on). Not a placeholder, not an empty envelope.
2. *Would a simpler check have worked?* No, and this matters for anyone re-deriving RR3 later. I parsed the Mach-O load commands of `Contents/MacOS/muxsmith-gui` from both dmgs. **Both** carry `LC_CODE_SIGNATURE` with a valid `0xfade0cc0` embedded-signature blob (new: dataoff 14533792, datasize 131856; R8: dataoff 14533792, datasize 113712). The arm64 toolchain emits a linker ad-hoc signature on every binary, so a binary-level signature check does **not** discriminate signed from unsigned here and would have passed on the defect artifact. The bundle-level `_CodeSignature/CodeResources` is the correct and only discriminator at this layer. The plan picked the right check; the size delta (131856 vs 113712) is consistent with `codesign` re-signing the executable with the resource-directory and requirement data added.

### RR4 - no license resource (ruling 2 machine half): CONFIRMED

| check | my value | report's value |
|---|---|---|
| `LC_ALL=C grep -ac LPic <new dmg>` | **0** (grep exit 1) | 0 |
| planted-copy fire, my own copy at my own path | **1** (grep exit 0); sizes 7 034 828 -> 7 034 832 | 1; same sizes |
| **differential: `LC_ALL=C grep -ac LPic <R8 dmg>`** | **2** | 2 |
| `LC_ALL=C grep -ac 'STR#'` | R8 **1**, new **0** | R8 1, new 0 |
| `LC_ALL=C grep -ac 'TEXT'` (mine, beyond the report) | R8 **1**, new **0** | not reported |

My `LPic = 2` on the R8 dmg independently reproduces the plan-review round-1 measurement the plan records as RR4's method-validity evidence. The plant was made on a copy, fired, and removed (`command rm -f`, absence re-checked). The third resource type (`TEXT`, the MacRoman-decoded EULA payload the R8 walk-through's garbling came from) moves 1 -> 0 in the same direction, which the report did not measure and which I add as a further anchor.

So the absence is anchored four ways now: the check fires on a planted copy; it returns 2 on the artifact that actually carried the defect; both companion resource types of the same EULA block go to zero; and the same numbers came out of an earlier, unrelated measurement.

### RR1, RR2, RR5, RR6 - CONFIRMED

| item | my measurement |
|---|---|
| run 30312889098 | `workflow_dispatch`, headSha `8ad4392c...a0a83e`, completed/**success**, **6** jobs (guard + four bundle legs + assemble) all success |
| draft | `isDraft: true`, `published_at: null`, name `REHEARSAL - not a release (run 30312889098)`, created 2026-07-27T23:14:01Z, **8** assets |
| `sha256sum --ignore-missing -c SHA256SUMS` on my own downloads | exit 0, **7** `: OK` lines; `SHA256SUMS` itself is 7 lines and does not list itself; 8 files on disk |
| RR1 falsifiability control, my own | byte flipped at offset 1000 of a *copy* of the deb -> `muxsmith-0.1.0-linux-x86_64.deb: FAILED`, `WARNING: 1 computed checksum did NOT match`, exit 1, the neighbouring rpm still `OK`; control dir removed |
| RR1 D76 per-leg outputs | 4 executed lines: linux-x86_64 **250**, windows-x86_64 **1**, windows-arm64 **1**, macos-arm64 **5** (see section 5 for the count question) |
| RR2 body | 25 lines, 1711 bytes; 4 `^> ` banner lines; `^---$` = 2; the title line matches `^Muxsmith 0.1.0 - builds carry no developer identity; ...` with `#windows`, `#macos`, `#linux` all on that one line (one occurrence each) |
| RR5 msi x86_64 | primary `Permission is hereby granted` = **1**, in-file control `Muxsmith` = **3**; arm64 identical (1 / 3) |
| RR5 corroboration (mine, beyond the plan) | `msiinfo export ... Control` -> `Permission is hereby granted` = **1**, `LicenseAgreementDlg` = **11**, i.e. the text sits on the license dialog surface, not merely somewhere in the string pool |
| RR6 | `git ls-remote origin` -> exactly 2 refs, both `8ad4392`: `HEAD`, `refs/heads/master`. `grep -c rehearsal` = **0**, control `refs/heads/master` = **1**, planted copy -> **1**. `git ls-remote --tags origin` -> **0 lines**. `gh api .../tags` -> **0** |

---

## 2. The controls actually fired

The plan's binding rule is that an absence check is verified by making it produce output once, with the **same invocation**. Checked for every absence claim in the report, not only the three named in the brief. I fired each one myself rather than accepting the report's record of it.

| absence claim | my control | fired? |
|---|---|---|
| RR2 `grep -cE '^\| \[' <body>` = 0 | identical invocation on `git show 87c1dee^:.github/release/draft-body.md` -> **2**; and on the post-Task-3 template -> **0** | yes, and the negative case is bracketed |
| RR2 `grep -c __VERSION__ <body>` = 0 | identical invocation on the committed `.github/release/draft-body.md` -> **8** | yes |
| RR4 `LC_ALL=C grep -ac LPic <dmg>` = 0 | own copy + `printf 'LPic' >>` -> **1**; plus the R8 differential -> **2** | yes, twice over |
| RR6 `grep -c rehearsal <ls-remote>` = 0 | positive control `refs/heads/master` -> **1** on the same file with the same invocation; plus a planted `refs/tags/rehearsal-30312889098` line -> **1** | yes |
| RR3 `CodeResources` >= 1 (presence, self-verifying) | positive control `MacOS/muxsmith` -> **2**, and the R8 differential gives the 0 state | yes |
| RR1 `sha256sum -c` passes (presence) | own byte-flip -> FAILED naming exactly the corrupted file | yes |
| RR5 primary (presence, self-verifying) | in-file control `Muxsmith` -> 3; no fallback stage was entered and none bottomed out at zero | n/a, correctly handled |

I also verified that the committed template blob (`git show 87c1dee:.github/release/draft-body.md`) and the working-tree file are identical, so the `__VERSION__ = 8` control was taken against the same bytes the pipeline consumed.

One instrument test worth recording as a negative result: the API's asset `download_count` (deb 2, everything else 1) looked like corroboration of the report's re-download story, so I tested whether the counter is live by re-downloading the rpm and re-reading it. **It did not move.** The counter therefore carries no evidential weight in either direction and nothing in this verdict rests on it.

---

## 3. No claim beyond the machine half

Clean. I scanned the report and the gh-log Task-4 section for every acceptance-level term (`unidentified developer`, `damaged`, `Gatekeeper`, `rendered`, `dialog`, `quarantine`, `ruling 1|2|3`, `verified`, `accept`).

- Every RR heading that has an owner counterpart names it: RR2 "(machine half; acceptance is O3)", RR3 "(machine half; acceptance is O1)", RR4 "(machine half; acceptance is O2)".
- RR3 closes with "Whether Gatekeeper now says 'unidentified developer' instead of 'damaged' is O1, on hardware. Not claimed here."
- The report carries an explicit "What this task does NOT claim" section stating "rulings 1, 2 and 3 are NOT verified by this report."
- The two hits on `damaged` / `license dialog` in the gh-log and in the RR3 differential paragraph are descriptions of the **old** R8 artifact, i.e. what the owner already observed, not a claim about the new one. Correct.
- The RR3 sentence "the seal is a change introduced by this package, not a property the artifact always had" is a machine claim about a file's presence, backed by the differential. In scope.

The one defect in this area is a claim about wording rather than about the rulings: see finding F1.

---

## 4. The draft survived; nothing published, edited, deleted or tagged

Verified myself, read-only, without touching the draft.

| what | evidence |
|---|---|
| the draft still exists and is still a draft | `gh api repos/senolfeldmann/Muxsmith/releases`: `draft: true`, **`published_at: null`**, id 360752003, 8 assets |
| exactly one release object exists | `gh release list` -> 1 row; the API list -> 1 object |
| no plan-8-era draft was deleted by this task | they were already gone before Task 4 ran: `plan-review-round-1.md:205` records "rehearsal drafts are already deleted (`gh release list` empty; control ...)". The single-release state is therefore not evidence of a Task-4 deletion |
| no tag ref | `git ls-remote origin` = 2 refs, both `8ad4392`; `git ls-remote --tags origin` = 0 lines; `gh api .../tags` = 0 |
| exactly one dispatch | `gh run list --workflow release.yml --limit 10`: the only release run after the Plan-8.5 push is 30312889098. The five earlier release runs are the plan-8-era R8 batch of 2026-07-27 midday (13:56, 14:07 and the three before). `gh run list --limit 20` shows no other run of any workflow after 23:03:44Z |
| **the body was not hand-edited** | strongest available check, and stronger than the plan asked for: I reconstructed the deterministic region locally as `cat .github/release/rehearsal-banner.md` + `sed 's/__VERSION__/0.1.0/g' .github/release/draft-body.md` (exactly the assemble step's composition, `release.yml:215-217`) and diffed it against the live body's first 23 lines: **byte-identical**. The remaining line is GitHub's own generate-notes output (`**Full Changelog**: .../commits/rehearsal-30312889098`). A manual edit anywhere in the templated region would have shown here |
| repo state | `git status --porcelain` empty; `.superpowers/` and `gh-log.md` are both gitignored (`.gitignore:2`, `.gitignore:3`), so "no repo file written" is structurally consistent, not merely asserted |

---

## 5. The two flags

### Flag 1: local master one commit ahead of origin at dispatch. Correctly reasoned; a note, not a defect.

Measured myself:

- `git rev-list --count origin/master..HEAD` = **1**. The commit is `61dc522` "roadmap: flake is a ruled 1.x fix, with its candidate named".
- `git diff --name-only origin/master..HEAD` = **`docs/ROADMAP.md`** only (11 insertions, 3 deletions).
- Run 30312889098's `headSha` = `8ad4392c4094b1426267a3c517b35554eba0a83e`.
- ci run 30312472265: workflow `ci`, event `push`, headSha **the same** `8ad4392...`, completed/success, 5 jobs (ubuntu-26.04, macos-15, windows-2025, deny, **ledger-lint**) all success.
- All four task commits are ancestors of `8ad4392`: `9460daf` (T1), `5060ef5` (T1 review fix), `50e08cd` (T2), `87c1dee` (T3), each confirmed with `git merge-base --is-ancestor`.

**Ruling: the reasoning is right, and this is a note, not a defect.** `gh workflow run --ref master` resolves the ref server-side, so the run's base is whatever `origin/master` pointed at, which is precisely the SHA precondition 3 requires to be ci-green. The plan's preconditions demand a clean tree and pushed task commits; neither demands `HEAD == origin/master`. The ahead commit touches only `docs/ROADMAP.md`, which no leg of the release pipeline reads (the pipeline consumes `Cargo.toml`, `src-tauri/*` and `.github/release/*`). So the rehearsal base is exactly the verified tree, and the report's own characterisation ("controller bookkeeping that is correctly not in play") holds under measurement.

Worth carrying forward, because the flag is right for a reason larger than this run: the safety here is *contingent on which file the ahead commit touched*. The same ahead-state with a commit touching release collateral would have produced artifacts that silently do not match the local tree, with no signal anywhere in the run output. If the controller wants that made non-contingent, the cheap form is a precondition line requiring `git rev-list --count origin/master..HEAD` to be 0 at dispatch, or requiring the diff to be measured and shown to touch no pipeline input. That is a plan-close item, not a Task-4 fix.

### Flag 2: RR1's grep printed 8 where the plan expects 4. Reasoning right; a real defect, but in the plan text.

Reproduced exactly on my own copy of the log:

| invocation | my value | report's value |
|---|---|---|
| the plan's own `grep -c 'updater-artifact check:'` | **8** | 8 |
| `grep -c 'updater-artifact check: 0 hits across \$bundles_found'` | **4** | 4 |
| `grep -cE 'updater-artifact check: 0 hits across[[:space:]]+[0-9]+ bundle output files$'` | **4** | 4 |

The four executed lines and their jobs: `bundle (linux-x86_64, ubuntu-22.04, deb,rpm,appimage)` 250, `bundle (windows-x86_64, windows-2025, msi)` 1, `bundle (windows-arm64, windows-11-arm, msi)` 1, `bundle (macos-arm64, macos-15, dmg)` 5. One per distinct leg, every N greater than zero. The substantive condition RR1 asserts is satisfied.

**I verified the cause at the source rather than accepting the report's explanation.** `.github/workflows/release.yml:134` is `echo "updater-artifact check: 0 hits across $bundles_found bundle output files"`, byte-identical to the design's section-2 fence at `2026-07-22-plan8-packaging-release-design.md:1396`. GitHub Actions echoes a step's `run:` script into the log group, ANSI-prefixed, before executing it, so each leg contributes exactly two matching lines: the unexpanded source and the expanded output. The doubling is a structural property of `gh run view --log`, not of this run, and it would have appeared on any release run ever made.

**Ruling: the implementer's reasoning is correct, the decomposition is complete (4 + 4 = 8, nothing unaccounted), and the handling was right.** No NEEDS_CONTEXT was owed: nothing here is a fork, no design latitude was exercised, and the substantive per-leg condition is directly checkable and was directly checked. Reporting it rather than swallowing it is the behaviour the plan wants.

**It is nonetheless a genuine defect, and it sits in the plan text.** I traced where the "4" came from. Plan 8's R5, the precedent this check was transcribed from, asserts no count at all: "in each of the four leg logs, the step ... printed its summary line (`updater-artifact check: 0 hits across N bundle output files`, N > 0 ...)" (`2026-07-23-plan-8-packaging-release.md:648-654`). That is a per-leg presence check, and it is instrument-independent. Plan 8.5 Task 4 Step 2 converted it into an aggregate `grep -c ... -> 4`. The 4 is a count of **legs** silently reused as a count of **log lines**, asserted from drafting intent and never measured against the instrument. Nearest house entry is `proc-normative-count-recomputed` (a count that summarizes one enumeration must be recomputed from that enumeration); the fit is by analogy rather than a clean instance, since the failure here is that the count was computed over the wrong set, so I flag it for the controller's routing rather than declaring a ledger occurrence myself.

Recommended fix, for the plan close and not for Task 4: restore plan-8's per-leg presence shape rather than patching 4 to 8. A per-leg check keeps working if GitHub ever changes whether `--log` echoes the script source; a hardcoded 8 inherits the same fragility the 4 had.

---

## 6. gh discipline

| rule | result |
|---|---|
| every gh interaction logged with command, effect and manual-UI equivalent | **satisfied.** The Task-4 section (`gh-log.md:508-585`) carries 13 command entries: the precondition re-read of ci run 30312472265, the dispatch, the identity `run list`, the two `run watch` invocations, `run view` + `--json jobs`, `release view`, the two `release download` calls, the body fetch, `run view --log`, the R8 `run download`, and `release list` (noted as run twice). Each has a "Manual equivalent" line. `git ls-remote` correctly carries no entry (read-only git, per the plan's RR6 step) |
| nothing that costs money | **satisfied.** `gh repo view --json visibility` -> **PUBLIC**, so the run consumed free public-repo runner minutes, which is the cost basis the plan's rule names. No paid gh surface was touched: no `gh api` write, no re-run, no artifact re-upload, no Copilot or Actions billing endpoint |
| one dispatch only | **satisfied and independently confirmed.** `gh run list --workflow release.yml` shows exactly one run after the Plan-8.5 push |
| foreground waits with explicit timeouts, never background-run-plus-monitor | **satisfied**, with a plan-text problem underneath it: see finding F2 |
| the section closes with the explicit no-publish/no-edit/no-delete/no-tag statement | present (`gh-log.md:582-585`), and independently true per section 4 |

---

## Findings

### F1 (MINOR, report prose): "restated verbatim from the plan" is not verbatim

The plan's Step 8 instructs that O1-O4 be "restated verbatim from the owner-steps section", and report line 114 claims they were ("restated verbatim from the plan for the close"). I diffed all four items against the plan's `:102-105` after whitespace normalisation. **All four differ.** The concrete drops and substitutions:

- **O1:** "on the Mac, download Task 4's rehearsal dmg" becomes "download the rehearsal dmg"; "Before first launch, confirm the quarantine attribute is present: `xattr ...` prints a value" is reordered to "confirm `xattr ...` prints a value before first launch"; "Optional corroboration on the same hardware:" becomes "Optional corroboration:".
- **O2:** the trailing parenthetical "(Fallback-branch acceptance instead: the license dialog appears and renders "Şenol Feldmann" correctly - `Ş` intact, no bold bleeding into the following word.)" is **absent**.
- **O3:** "the rehearsal draft's rendered body" becomes "the draft's rendered body"; "as one block (no separate link paragraphs)," loses its parenthetical, which is the clause that says what "one block" means.
- **O4:** "(including any still-standing plan-8-era rehearsal drafts)" is replaced by the report's own measured statement.

Substance survives in O1 and O3, and the O4 substitution is an improvement in content. The label is the defect: a claim about wording that was not checked against the wording. Material risk is low because the controller routes O1-O4 from the plan at close, not from this report, and the plan carries the fallback route for a failed O2 independently.

**Fix (either is sufficient):** paste the four items byte-for-byte from the plan, or drop the word "verbatim" and say "restated (condensed) from the plan; the plan is authoritative". Not blocking.

### F2 (NOTE, plan text): the plan prescribes a single foreground wait the harness cannot execute

Task 4 Step 1 says `timeout 5400 gh run watch <id> --exit-status --interval 30`, as one foreground call. The Claude Code Bash tool's maximum timeout is 600 000 ms, so no single foreground invocation can carry a 5400 s budget. The implementer's stated cause is therefore correct, and the two sequential `timeout 540` invocations with the split disclosed in both the report and the gh-log are the only way to satisfy "foreground, explicit timeout, never background-run-plus-monitor" as written. The timing is consistent with the run record: run created 23:03:44Z, draft created 23:14:01Z, i.e. about 10.5 minutes, against the report's "01:03:43 -> 01:14:38 local, about 11 minutes" (CEST = UTC+2).

Two things this leaves open for the plan close, neither chargeable to Task 4:

1. The plan's stop rule, "A tripped timeout stops the task and returns to the controller with the run URL - never silently extended", becomes ambiguous under chunking, because a chunk boundary trips a timeout without the budget being exhausted. It should be restated as a **total elapsed budget** ("stop and return once cumulative wait exceeds N"), so the next implementer neither returns prematurely on a chunk boundary nor treats the chunking as licence to extend indefinitely.
2. The chunked form should be in the plan text. As written, an implementer following the letter either cannot run the step or reaches for backgrounding, which is exactly what the constraint bans.

### F3 (NOTE, report prose): a borrowed precondition claim drifted

The precondition table's gate cell says the flake was "green on the complete re-run, three further green runs". The record it cites (`docs/ROADMAP.md` "Test flakiness", :1183-1184) says "green in isolation, and green in three subsequent `cargo test --workspace` runs". Four greens either way, and the conclusion (gate green) is unaffected, but "in isolation" became "the complete re-run", which is a stronger statement than the source makes. The gate is the controller's evidence and the report correctly labels the row "verified, not redone"; the drift is in the paraphrase, not in the conclusion.

### F4 (INFO, report prose): one backticked log line is whitespace-normalised

RR1's table renders the macOS leg as `` `0 hits across 5` ``. The literal line is `updater-artifact check: 0 hits across        5 bundle output files` (the padding comes from `wc -l` on macOS). The number is right and the report's own regex uses `[[:space:]]+`, so it clearly knew; only the backticked rendering implies a wording it does not have. Noted because the house treats backticks as a claim about the literal text.

### Nothing found in these areas

- No fabricated or unreproducible number anywhere in the report. Every measurable value I could re-take, I re-took, and all matched.
- No claim of O1, O2 or O3, in the report or the gh-log.
- No repo write, commit, tag, publish, edit, delete or second dispatch.
- No fallback chain entered on a tool failure and passed off as green (RR5's primary hit at stage 1; RR3's 7z resolved at stage 1; the `gh api` download fallback was never needed).
- No unenumerated shape absorbed at the keyboard. The one deviation encountered (flag 2) was reported, and the one instrument constraint (F2) was disclosed rather than worked around.

---

## My own RR3 and RR4 numbers, for the record

**RR3.** New dmg: `CodeResources` = **1** at `Muxsmith/Muxsmith.app/Contents/_CodeSignature/CodeResources`, 2658 bytes; `_CodeSignature` = **2**; positive control `MacOS/muxsmith` = **2**. R8 defect dmg: `CodeResources` = **0**, `_CodeSignature` = **0**, same positive control = **2**. Differential: **0 -> 1**. The seal is a genuine `codesign` resource plist (SHA-1 `files`, SHA-256 `files2`, `cdhash` + `requirement` for the main executable, standard `rules`/`rules2`). Both binaries carry a linker-level `LC_CODE_SIGNATURE` (`0xfade0cc0`, 131856 bytes new vs 113712 R8), so the bundle seal is the only discriminator at this layer.

**RR4.** New dmg: `LPic` = **0**, `STR#` = **0**, `TEXT` = **0**. Planted copy of the new dmg: `LPic` = **1** (sizes 7 034 828 -> 7 034 832). R8 defect dmg: `LPic` = **2**, `STR#` = **1**, `TEXT` = **1**. Differential: **2 -> 0** on the primary, with both companion resource types moving the same way.

---

## Verdict

**APPROVED.** The machine halves are real, they reproduce on an instrument the implementer never saw, the controls fire, the draft is untouched and unpublished, the remote carries no tag, exactly one dispatch was made, and the report claims nothing on the owner's side of the line. F1 is a wording fix in a scratch document. F2 and the plan-text half of flag 2 are corrections the plan close should carry so the next transcription of these checks does not inherit them; neither is a Task-4 fix and neither blocks the close.

Owner steps O1-O4 remain the acceptance and are untouched by this verdict. The rehearsal draft `rehearsal-30312889098` is standing, unedited, and byte-verified against the committed templates.
