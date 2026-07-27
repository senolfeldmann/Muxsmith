# Task 1 report: Ad-hoc signing + the S22 wording sweep (A4)

**Commits:** `9460daf14089d6d61af8fed859bd5f1aaedf46f7` (original Task 1) and `5060ef5b8dae58d1a8e649abad00f377f871c953` (review round 1 fix, see below) on `master` (main worktree, no branch/worktree used).

## Edits applied (old -> new, per file)

1. `src-tauri/tauri.conf.json`: macOS block gained `"signingIdentity": "-"` (Step 1).
2. `docs/INSTALL.md:5-6` (intro paragraph): "All 1.0-era builds are **unsigned**: your OS will warn ..." -> "No 1.0-era build carries a developer identity - the Windows installers are unsigned, the macOS app is ad-hoc signed (no Apple certificate, no notarization) - so your OS will warn ..." (Step 3).
3. `docs/INSTALL.md:52` (Gatekeeper line, shifted from :50 by the intro edit's +2 net lines, measured: 2 removed/4 added): "**Gatekeeper:** the app is unsigned and not notarized." -> "**Gatekeeper:** the app is ad-hoc signed - no Apple developer identity, not notarized - so macOS treats it as coming from an unidentified developer." (Step 3).
4. `.github/release/draft-body.md:1`: "Muxsmith __VERSION__ - unsigned builds; ..." -> "Muxsmith __VERSION__ - builds carry no developer identity; ..." (Step 3).
5. `docs/ROADMAP.md` S22 kickoff record (located by content, not line number): inserted the `[superseded in part 2026-07-27, Plan 8.5 ruling 1: ...]` bracket into the "UNSIGNED artifacts on all three OS at 1.0 (...)" sentence (Step 3).
6. `README.md` placeholder-rider comment (located by content): "the per-OS unsigned-install" -> "the per-OS install-hurdle" (Step 3). `placeholder(1.0)` count re-measured at 4, unchanged.
7. `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md`: inserted the "Superseded 2026-07-27 (Plan 8.5 Task 1)" note directly after the frozen README-comment fence in Task 3 Step 4 (Step 4).
8. `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`: three insertions (Step 5) - (a) D75 supersession-note paragraph after the S22 owner-ruling paragraph; (b) new `macOS.signingIdentity` bullet in D86's decision list, directly after the `macOS.minimumSystemVersion` bullet; (c) A4 amendment-log entry appended after A3 (A3 was the last entry, file measured at 2133 lines pre-edit via `git show <parent>:<path> | wc -l`, not 2150 as originally reported here).

All eight edits matched the plan's quoted "old" text exactly on first read (no content-anchor mismatch); no fork was hit, nothing routed to NEEDS_CONTEXT.

## Step 2 (red-state fire-verification for the config change)

- Green run: `./node_modules/.bin/tauri inspect wix-upgrade-code` exit 0, printed `9262b417-b687-5ea3-ace1-18b9d51b215f`.
- Backed up `src-tauri/tauri.conf.json` to `/tmp/tauri.conf.json.bak` (`command cp -f`).
- Fired red state: set `"signingIdentity": 123`, re-ran -> exit 1, error `` `"tauri.conf.json"` error on `bundle > macOS > signingIdentity`: 123 is not of types "null", "string" `` - matches the plan's predicted exact wording.
- Restored (`command cp -f`), re-ran green -> exit 0 again, and `git diff src-tauri/tauri.conf.json` showed exactly the Step-1 one-line addition (plus the necessary trailing-comma change on the preceding line).

## The three closing checks (Step 6), each fire-verified by its own invocation

**(a)** `git grep -nE 'unsigned on all three|All 1.0-era builds are \*\*unsigned\*\*|unsigned builds|the app is unsigned' -- README.md docs/INSTALL.md docs/ROADMAP.md .github/release/`
- Pre-edit fire: **4 hits** (`.github/release/draft-body.md:1`, `docs/INSTALL.md:5`, `docs/INSTALL.md:50`, `docs/ROADMAP.md:547`) - matches the plan's stated pre-edit count exactly.
- Post-edit: **1 hit** - `docs/ROADMAP.md:550` (`  to the "unsigned on all three OS at 1.0" wording and wants an explicit`), the ROADMAP blocker entry's own quoted-wording line. Line number differs from the plan's cited `:547` because this run's S22-record edit added 3 net lines above it (measured from the commit hunk: 1 line removed, 4 lines added); the plan itself flags ROADMAP line numbers as the authoring-day snapshot only and directs matching by content, which this does.

**(b)** `command grep -c 'per-OS unsigned-install' README.md`
- Pre-edit fire: **1**.
- Post-edit: **0**.
- Confirmed the two legitimately-kept sites of the same string still stand untouched: `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md:358` (frozen fence, now followed by the Step-4 supersession note) and `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:1797` (design 4.5 fence, covered by A4).

**(c)** `command grep -c 'superseded in part 2026-07-27, Plan 8.5 ruling 1' docs/ROADMAP.md`
- Pre-edit: **0**.
- Post-edit: **1**.

All three checks passed on the first post-edit run; no re-fix cycle was needed.

## Sweep classification confirmation (Step 4, pre-edit, informational - not a Step-6 fire-verification)

**Correction (review round 1):** the original version of this section wrongly labeled a seven-path list as "the five Step-3 sites" and drew a completeness conclusion that did not follow from it. Restated precisely below.

`git grep -niE "unsigned|not signed|notariz"` over the whole tracked tree returned **302** hits pre-edit. Within that output, the actual **five Step-3 sites** (`docs/INSTALL.md:5`, `docs/INSTALL.md:50`, `.github/release/draft-body.md:1`, `docs/ROADMAP.md:181`, `README.md:101`) were present, plus two of the plan's separately-classified **KEPT** sites (`docs/INSTALL.md:31`, `docs/INSTALL.md:34`, the Windows-stays-unsigned bullets) - seven paths total, two different classes, conflated in the original wording. This only confirms Step 3's own five sites were findable pre-edit; it says nothing about the Design-doc row of the classification table ("D75 ruling text + its INSTALL outline bullet"), which this broad grep does not scope to and which I never separately verified. That is exactly the row that turned out incomplete: Step 5 amended the D75 ruling-text paragraph but never touched the outline bullet ("unsigned and not notarized") at (pre-fix) line 312, which restated the superseded wording as live decision prose, contradicting the corrected `docs/INSTALL.md:52` within the same commit. Found by the reviewer, not by this check.

## Fix round (review round 1, one major)

Reviewer finding: `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:312` (D75's macOS outline bullet, live prose, not inside any fence - confirmed via `grep -n '^```'` over the surrounding region, no fence markers between lines 250-340) still read "unsigned and not notarized" after commit `9460daf`, contradicting `docs/INSTALL.md:52` and falsifying the D75 supersession note's own claim ("Every site restating the old wording was corrected in the same change") by exactly that one site. Step 4's classification table had named this site but Step 5's three edits never covered it - the miss was that Step 4 should have returned NEEDS_CONTEXT instead of leaving a named-but-unedited site.

Fix applied (commit `5060ef5`, staged explicitly, same file only):
- Corrected the `:312` bullet's opening clause from "unsigned and not notarized" to the same substance as `docs/INSTALL.md:52` ("ad-hoc signed - no Apple developer identity, not notarized - so macOS treats it as coming from an unidentified developer"), rest of the bullet unchanged.
- Extended A4's amendment-log entry with a clause naming this site and recording that it was missed at the first pass and is now corrected in place (live prose, not a frozen fence, so the tree is directly authoritative - no supersession-by-note needed, unlike the section 3.1/4.5 fences).
- Checked A4 and section 11's frozen-literal list for a stated numeral or count over the affected-sites set that `proc-normative-count-recomputed` would require updating: found none (the "fence bookkeeping" enumeration lists sections 3.1/4.1/4.2/4.5 by name, never by number, and section 11's literal list is likewise unnumbered prose). The one totality claim in scope - D75's supersession note, "Every site restating the old wording was corrected in the same change" - needed no wording change; it is satisfied by the underlying fact once the `:312` fix landed, verified below.
- Re-ran all three Step 6 closing checks post-fix: (a) 1 hit (unchanged), (b) 0 (unchanged), (c) 1 (unchanged) - the fix touches neither check's scoped strings.
- Confirmed by fence-membership that the fix sites (`:312` and the amendment-log insertion around line 2160) sit outside any fence, and that the one other place still containing "unsigned and not notarized" (`docs/superpowers/specs/...design.md:1647`) sits inside the section 4.1 verbatim-transcription fence, already covered by A3/A4 as superseded-by-the-tree - correctly left untouched, unlike `:312`.

## Premises refuted

None in the original Task 1 pass. In the fix round: none - the reviewer's finding and all four figure corrections were confirmed by direct measurement against the commits (see corrections above and in the edit list).

## Post-commit state

Two commits: `9460daf` (original Task 1, seven files) and `5060ef5` (fix round, one file: `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`). `git status --porcelain` empty; `git diff --stat` empty after each commit. No `git add -A` used at either commit.
