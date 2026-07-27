# Plan 8.5 plan review, round 1

Reviewer: independent (did not author the plan). Graded artifact:
`docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md` at
commit `c2514e7be0ae7287adc41160fb0a71d140f39776` (= HEAD; working tree
clean at review start and at review end, verified `git status --porcelain`
both times). Ground truth read: v1 spec (precedence only - no conflict
arose), plan-8 design D75-D90 + A1-A3, ROADMAP Plan-8.5 anchor (:206-248)
and the three finding entries (:464-527), the controller brief
(`.superpowers/sdd/plan-8.5/plan-brief.md`), the four house YAML files (by
entry id). Every number below was measured by me in this session; every
quote was opened.

## STATUS: NEEDS FIXES

One Major (a post-sweep absence check that does not satisfy the plan's own
fire-verification constraint and is structurally blind to two of the five
sites it guards), plus four Minors. The load-bearing experiment, all three
vendor-source claims, all transcriptions, and all recomputed counts are
confirmed - the fixes are surgical, no re-plan.

---

## The settled branch: both legs re-verified independently (dimension 3)

**Leg 1 (source), CONFIRMED.** I fetched the pinned tag's files myself via
`gh api` (`tauri-apps/tauri` at `tauri-cli-v2.11.4`, into a
reviewer-unique scratch dir), not the plan's citations:

- `crates/tauri-cli/src/helpers/config.rs`: line 6 `use json_patch::merge;`;
  in `fn load_config` the platform file from
  `tauri_utils::config::parse::read_platform(target, tauri_dir)` is applied
  with `merge(&mut config, &platform_config)` - RFC 7396, null deletes.
  `-c` overlays are combined with `merge_patches` (doc comment, verbatim:
  "Same as [`json_patch::merge`] but doesn't delete the key when the
  patch's value is `null`") and the combined patch is then applied with the
  same deleting `merge(&mut config, &merge_config)`. Both levers clear,
  exactly as the plan states. Schema validation
  (`config_schema_validator().iter_errors(&config)`) runs after both
  merges, on the post-merge document; `exit(1)` on error when not
  reloading - the plan's Step-2 red-state shape matches the source
  (`error on \`{path}\`` with `" > "`-joined path).
- `crates/tauri-utils/src/config/parse.rs:54-56`: `Target::MacOS =>
  "tauri.macos.conf.json"`, Windows/Linux their own files - the macOS
  platform file is structurally invisible to Windows/Linux builds, which is
  the premise of the plan's no-Windows-hardware section. Holds.

**Leg 2 (empirical), REPRODUCED with my own instrument.** I could not
recreate `src-tauri/tauri.linux.conf.json` (reviewer write constraint:
one file only), so I used the second lever the plan itself claims and the
source proves equivalent at the final merge: a reviewer-authored
null-carrying overlay at a scratch path the implementer could not have
written, passed via `-c`. Same pinned binary (`tauri-cli 2.11.4`, matches
`package.json:30` exact pin):

- Baseline `./node_modules/.bin/tauri bundle -d --bundles deb`:
  control file shows ` Recommends: mkvtoolnix`, count 1 (the
  known-present control that fires the absence check).
- With `-c <scratch>/plan85-null-probe.json` containing
  `{"bundle":{"licenseFile":null,"linux":{"deb":{"recommends":null}}}}`:
  exit 0, no validation error, **Recommends count 0**,
  ` Depends: libwebkit2gtk-4.1-0, libgtk-3-0` intact (positive control).
- Cleanup: `target/debug/bundle` removed, `git status --porcelain` empty.

The CLEAR branch is correctly settled. The author's platform-file variant
is additionally covered by leg 1 (identical deleting merge call) and by the
CLI's own `-c` help text, which documents the platform-file auto-merge
(verified in this session's `tauri bundle --help` output).

**Second claim (dmg license mechanism), CONFIRMED at the pinned source.**
`crates/tauri-bundler/src/bundle/macos/dmg/mod.rs:161-169`: `--eula` is
passed only when `settings.license_file()` is `Some`; `settings.rs`
`license_file()` returns `bundle_settings.license_file.clone()` with **no
fallback**; `crates/tauri-cli/src/interface/rust.rs:1696` maps
`config.license_file` directly. Clearing the key removes the whole EULA
resource block - defect class gone, not repaired, exactly the ROADMAP
entry's analysis. `bundle_dmg` script: `file -b` result matched against
`'Rich Text Format data'*` -> `'RTF '` else `'TEXT'`; udifrez + template as
described; `FILESYSTEM="HFS+"` default. All as the plan states.

**Branch machinery checks:** the fallback branch is executable without a
second planning round (verbatim RTF, config, BUILDING paragraph, commit
message; switch triggers enumerated at Step 4 and in the fallback header;
"any other outcome -> NEEDS_CONTEXT, no third branch"). The branch
condition is observable (a printed 0 or 1 from an enumerated grep). The
tiebreaker is quoted as the owner's ruling (Global Constraints bullet 2,
ROADMAP anchor wording), not re-derived, and Task 2 Step 4 implements it
mechanically. One residual: the fallback A5 text is not verbatim (Minor 4).

**Fallback RTF verified mechanically (my instrument):** extracted the
plan's RTF block (plan :396-419) to scratch; `file -b` reports exactly
`Rich Text Format data, version 1, ANSI, code page 1252` (the plan's
claimed detection result, and the prefix the dmg script keys on); applying
the plan's stated transformation (drop 2 header lines, drop final `}`,
strip `\par`, `\u350?` -> `Ş`) yields 21 lines **identical line-for-line**
to `LICENSE` (21 lines; `Ş` = U+015E = decimal 350, correct RTF escape;
`c5 9e` at LICENSE:3 hexdump-confirmed). The legal constraint is satisfied
by the carried artifact itself.

## The three brief-premise refutations (dimension 5): all three CORRECT, none overshoots

| # | Ruling |
|---|--------|
| 1 | **Correct.** `cat -A .github/release/draft-body.md`: line 2 begins `before first launch:`; only lines 3-4 begin `| `. `grep -cE '^\| \['` = 2, as the plan uses. The brief/ROADMAP "lines 2-4" was wrong; the defect and fix are unchanged. Replacement right. |
| 2 | **Correct.** D82's recorded rationale exists verbatim in the design ("The name deliberately avoids the auto-merged `tauri.<platform>.conf.json` filenames, so it never applies implicitly"). At source, both mechanisms flow through the same deleting `json_patch::merge` into the config - "same merge machinery, different application mechanism" is exactly the true statement, and A5's scope note (avoid-rationale is sidecar-specific: `externalBin` is compile-time-processed, `licenseFile` has no compile-time consumer) correctly prevents a false D82 contradiction. No overshoot. |
| 3 | **Correct.** At the pinned tag: `debian.rs` contains zero functional `license` references (only two SPDX header comments; my case-insensitive grep's method control fired on `rpm.rs`); `rpm.rs` consumes only `settings.license()` - the string - at :61 into the `PackageBuilder` License tag; `license_file` appears in neither. D86's "deb/rpm embed it as-is" is false at source. Correctly marked not load-bearing (my probe deb built fine with `licenseFile: null`). No overshoot. |

## Findings

### MAJOR 1 - Task 1 Step 6: the post-sweep absence check is not fire-verified by its own invocation and cannot see two of the five sites

Evidence, all measured on the pre-edit tree:

- Step 4 (plan :208) claims: "run it PRE-edit and confirm it hits the five
  Step-3 sites - that firing is the fire-verification of the post-edit
  check in Step 6". But Step 4's run is `git grep -niE "unsigned|not
  signed|notariz"` over the tracked tree, while Step 6 (plan :263) is a
  **different invocation**: different patterns, case-sensitive, different
  pathspec. Firing grep A is not a fire-verification of grep B; the house
  constraint (Global Constraints, "break the thing deliberately, watch
  **the check** fire") binds on the invocation that asserts the absence.
- Step 6's own grep, run pre-edit, returns exactly **4** hits
  (`draft-body.md:1`, `INSTALL.md:5`, `INSTALL.md:50`, `ROADMAP.md:486`) -
  not 5. The two sites it can never see: the ROADMAP:181 S22 record (old
  text `UNSIGNED artifacts on all three OS at` matches no pattern -
  case-sensitive `unsigned on all three` misses on case AND on the word
  `artifacts`) and README.md:101 (`per-OS unsigned-install` matches no
  pattern). A cheap-tier implementer who skips either Step-3 edit still
  gets Step 6's "exactly one hit" and reports green - the check passes on
  the defect it exists to catch.

Fix (concrete, two additions and one relabel in Step 6):

1. Relabel the fire: "fire-verified by running THIS grep pre-edit:
   expected exactly 4 hits - draft-body:1, INSTALL:5, INSTALL:50,
   ROADMAP:486" (measured values above).
2. Add the two missing post-edit checks with their own fires:
   `command grep -c 'per-OS unsigned-install' README.md` -> 0 (pre-edit: 1,
   which is the fire; note the same string legitimately remains in the
   plan-8 plan fence and design 4.5 fence, so the check must stay scoped to
   README.md), and presence check
   `command grep -c 'superseded in part 2026-07-27, Plan 8.5 ruling 1' docs/ROADMAP.md`
   -> 1 (presence checks self-verify).

### MINOR 1 - three stale/wrong ROADMAP line anchors for the INSTALL-re-verify duty

The "Whatever lands, `docs/INSTALL.md`'s macOS section is re-verified
against the real flow afterwards" duty sits at **ROADMAP:489-490**
(measured `grep -n 'Whatever lands'`). The plan cites it as `:483-484`
(coverage map, plan :77) and `:483` (O1, plan :102; plan close, plan :544).
Lines 483-484 are different sentences of the same entry. The quoted text is
right; the anchors are wrong, and Task 1's own S22 edit will shift the true
line by +5 (3 old lines -> 8 new). Fix: cite `:489` with a drift note, or
anchor by quote only. (This is the exact defect class the brief's
dimension 5 names.)

### MINOR 2 - sweep kept-list misdescribes ROADMAP:168

Plan :213 keeps `ROADMAP :168` as "(store-channel context)". Line 168 is
the Plan-8 section preamble: "Carries its own constraint set (code
signing, notarization) that no other [plan shares]". Keeping it is right
(still-true historical framing); the label is wrong - the store-channel
site is :926 (Homebrew-Cask entry), which the plan lists separately and
correctly. Fix: relabel :168 ("Plan-8 section preamble, constraint-set
framing").

### MINOR 3 - the sweep classification leaves two hit classes unenumerated

I ran the Step-4 sweep (`git grep -niE "unsigned|not signed|notariz"`,
tracked tree, journal excluded). Beyond the plan's classified sites, it
returns: (a) ten SI-4 commit-signing lines in the historical plan
preambles (plan-5 :18, plan-5.5 :19, plan-5.6 :20, plan-5.7 :47,
plan-5.8 :16, plan-6 :25, plan-7 :24, plan-7.5 :22, plan-8 :19 and :24) -
covered only implicitly by the general "commit-signing hits excluded by
subject" clause, while the out-of-surface bullet names decision-ledger and
process-conventions specifically and not these; and (b) the plan-8.5 plan
file itself (~25 hits: quoted old/new pairs and this very surface
definition), which falls in **no** named bucket. Under
`proc-latitude-clause-boundary` (an unenumerated set in a normative
position) the implementer must judge these hits alone. Fix: add both
classes to the classification ("historical plan preambles: commit-signing
subject, excluded" and "this plan file: quotes old wording as edit
targets, excluded").

### MINOR 4 - the fallback-branch A5 variant is described, not carried

Plan :372 specifies the fallback A5 by transformation ("write the A5
variant that records the SET-to-RTF route, the Step-2 contradiction ...
verbatim, and the same D82 scope note and D86 correction; the fence
bookkeeping adds `packaging/macos-license.rtf` to the frozen set") while
every other fallback artifact - RTF, config, BUILDING paragraph, commit
message - is verbatim. `proc-latitude-clause-boundary` says derivability
is not an exemption, and the fallback branch runs WITHOUT controller
routing (Step 4 sends a printed `1` straight to the fallback), so the
composed amendment would reach the tree unrouted. Content points are all
enumerated, hence Minor, not Major. Fix: write the ~15-line fallback A5
out verbatim in the fallback section.

### MINOR 5 - RR4's method validity rested on a conditional corroboration; I closed it by measurement, record it

RR4's mandatory fire (appending the literal bytes `LPic` to a copy) proves
grep mechanics, not that a license-carrying dmg exposes `LPic` in
plaintext; the corroboration that proves THAT is conditional ("if a
plan-8-era rehearsal dmg is still downloadable"). Measured state: the
rehearsal drafts are already deleted (`gh release list` empty; control
fired against tauri-apps/tauri, which lists releases), but run
30273529210's workflow artifacts are unexpired, and on the real R8 dmg
(`muxsmith-0.1.0-macos-arm64.dmg`, the artifact whose license dialog the
owner saw): `LC_ALL=C grep -ac LPic` = **2**. The method is valid.
Retention is 7 days (D89), so the corroborating positive expires around
2026-08-03. Fix: record the authoring-time-equivalent measurement (LPic=2
on run 30273529210's dmg, this review) in RR4, and name the artifact-path
corroboration as the primary form while retention lasts.

Also validated on the same real artifacts, for the record: RR3's listing
method - `7z l` on the R8 dmg lists bundle paths in one stage
(`MacOS/muxsmith` hits: 2 - both binaries) with `CodeResources` count 0 on
the defect artifact (the delta RR3 asserts post-fix); RR5's primary -
`LC_ALL=C grep -ac 'Permission is hereby granted'` = 1 on the R8 msi with
control `Muxsmith` = 3. No plan change needed for these two.

### Observations (no fix required)

- BUILDING.md insertion anchors (Task 2 Step 6): "the existing first
  paragraph (ending `... staging step.`)" actually continues with "To
  reproduce what CI ships:" on the next line of the same markdown
  paragraph. Both anchor strings are unique and adjacent; the insertion is
  executable and splits the paragraph as evidently intended. Cosmetic.
- `tauri inspect wix-upgrade-code` does not accept `-c` (checked): Task 1
  Step 2's edit+restore red-state procedure is the only available shape.
  Green state verified live: exit 0, GUID `9262b417-b687-5ea3-ace1-18b9d51b215f`
  printed, matching the config and the plan.
- Remote master is at `b1eb231` - the plan commit and the two ROADMAP
  commits are unpushed, as the authoring brief required ("Do not push");
  commit is unsigned (`%G?` = N) with the repo trailer. Task 4
  precondition 2 carries the push.
- Vendor doc re-fetched (v2.tauri.app/distribute/sign/macos): the plan's
  quoted sentence ("Ad-hoc code signing does not prevent MacOS from
  requiring users to whitelist the installation in their Privacy &
  Security settings") appears verbatim; pseudo-identity `-`, config path
  `bundle > macOS > signingIdentity`, `APPLE_SIGNING_IDENTITY` all as
  stated. Schema: `signingIdentity` typed `["string","null"]`;
  `license`/`licenseFile` appear exactly once each, in the global bundle
  object; no per-format license key (all checked in the pinned CLI's
  bundled `config.schema.json`).

## Coverage table (dimension 1)

| Requirement (source) | Implementing task/step | Acceptance | Verdict |
|---|---|---|---|
| Ruling 1: `signingIdentity: "-"` (ROADMAP :216-217) | T1 S1-S2 | RR3 machine half; **O1** dialog | covered |
| Ruling 1: S22 wording updated in same change (:219-221) | T1 S3-S5 (5 live sites + frozen fence note + A4) | T1 S6 check | covered; M1, m2, m3 on the check/classification |
| Ruling 1 acceptance: quarantined build shows "unidentified developer" (:221-223) | owner step | **O1** (browser-download quarantine guard incl.) | covered, honestly owner-marked |
| INSTALL.md re-verified against real flow (:489-490) | folded into O1 | O1 | covered; m1 anchors wrong |
| Ruling 2: license drop macOS-only, Windows dialog kept (:224-231) | T2 S5 (platform file) | RR4 + RR5 machine halves; **O2** | covered |
| Ruling 2 tiebreaker pre-decided, not re-weighed (:227-231) | T2 S4 branch rule (mechanical) | n/a | covered |
| Ruling 2 experiment first, result picks branch (:232-240) | T2 S1-S4, settled at authoring, re-run in-task | branch rule | covered; both legs re-verified by this review |
| Both branches executable, no second planning round (brief) | T2 S5-S9 + fallback section | O2 both forms | covered; m4 (fallback A5 not verbatim) |
| LICENSE legal constraint (brief/ruling) | fallback constraint + content-identity check | diff step | covered; RTF verified content-identical here |
| Ruling 3: join OS links; two other regions checked not assumed (:241-243) | T3 (3 regions measured: 1-4, 6-9, 21-26; table 11-19 kept; banner checked clean) | RR2 machine half; **O3** | covered; end-state byte-verified by reconstruction |
| Rehearsal re-run proves pipeline assembles (anchor + brief) | T4 S1-S2 (RR1) | plan-close gate | covered |
| No tag/publish/delete; owner deletes drafts | Global Constraints; **O4** | n/a | covered |
| Design contradictions carried as amendments | A4 (T1 S5), A5 (T2 S7); T3 rides A3 (verified in the log: 4.2 IS tree-superseded) | n/a | covered |
| Gate/pins/SI-4/model tiers/fire-verification (brief) | Global Constraints + per-task | n/a | covered; tiers conform to proc-03's owner-bound text |
| ROADMAP bookkeeping | plan close | n/a | covered |
| Brief-defect reporting duty | corrections table (3 entries) | n/a | covered, all three rulings above |

## No-work-needed premises, each run (standing check)

| Claim | Premise run | Result |
|---|---|---|
| "Task 3 needs no amendment - 4.2 already tree-authoritative per A3" | read A3 | TRUE: A3 lists 4.2 among the superseded-by-tree sites; banner explicitly NOT superseded ("fence diffs clean"), matching T3's banner claim |
| "No Windows hardware step - msi untouched by construction" | parse.rs platform selection + RR5 + ROADMAP CLOSED entry (owner: Windows "correct throughout") | TRUE |
| "Nothing to SHA-pin, no action added" | task file lists touch no workflow | TRUE |
| "Banner has no wrapped region of this shape" | read banner (4-line blockquote + `---`) | TRUE |
| "licenseFile has no compile-time consumer" | bundler-only consumption verified at source | TRUE |
| "R1/R2/R3/R6/R7/R9 not re-checked - unchanged surfaces" | R-ids matched against design section 8; change surface is macOS-scoped config + draft-body (RR2 covers) + RR6 transcribes R7 | TRUE |
| "Tiebreaker never fires - one 3-line file, zero workflow changes" | the experiment (re-run by me) | TRUE |
| "No new triggers - signing-revisit covers ad-hoc's successor" | ROADMAP :375-378, :438-440 | TRUE |
| "6 jobs, 8 assets, 4 leg logs" | release.yml: guard + 4-leg matrix + assemble = 6; 7 D89 artifacts + SHA256SUMS = 8; `updater-artifact check:` line at :134 emitted per leg | TRUE |

## House conformance (dimension 6, by id)

`proc-03-model-assignment`: tiers match the owner-bound statement (top =
Fable 5 only whole-branch review here; reviewers/controller/judgment mid =
Opus 5; transcription tasks cheap; explicit parameter duty restated).
`proc-07-verify-against-source` / `proc-57-briefs-not-ground-truth`:
the authoring-time verification section is the instantiation; premises
re-verified here at source. `proc-latitude-clause-boundary`: honored
except m3/m4 residuals. `proc-normative-count-recomputed`: every count I
recomputed holds - ten-part gate = 6 (BUILDING "Rust gate" incl.
cross-target part 6 per `gate-includes-cross-target-lint-for-the-unrun-os`)
+ 4 frontend; five live sites; placeholder(1.0) = 4; draft-body 11 / 2 /
8 / 1 pre and 9 / 0 / 8 / 1 post; RR1-RR6; O1-O4; 3 corrections.
Frozen-transcription practice: T1 S4's supersession note matches the
plan-8 plan's existing note shape (:262) and the A3/ledger supersession
principle; S22 ruling text edited in place is the ROADMAP (a live doc),
with the frozen plan-8 fence getting the note - correct split. SI-4
restated per the ledger's dispatch rule; gh rules per the plan-8 precedent;
typography clean (ASCII, `Ş` exception carried).

## Scale (dimension 7)

Four serial tasks, no worktrees, one gate site, one dispatch run: matches
the KISS ruling; the no-worktree rationale is argued and correct at this
size. The document's length is authoring-time evidence and the mandated
executable fallback branch, not programme inflation. No task is too thin
to review independently. No finding.

## HARVEST

- **H1 (brief/convention boundary that forced a workaround - wanted
  finding).** The reviewer write-constraint ("one file, no product-file
  writes") collides with "re-verify the empirical leg yourself" when the
  author's instrument was a repo-tree file (`src-tauri/tauri.linux.conf.json`).
  I resolved it via the `-c` lever (source-proven to share the deleting
  merge) plus git-ignored build output and a clean-tree proof. Future
  review briefs for experiment-carrying plans should either pre-authorize
  ephemeral experiment writes with a restore proof, or explicitly bless the
  equivalent-lever form.
- **H2.** A fire-verification must be the SAME invocation as the absence
  check it verifies; a neighbouring grep firing is agreement theater (M1's
  generalization; same lineage as the wording-fix H3 "zero needs named
  exclusions plus a fire-check against BASE").
- **H3.** When an RR check expects an absence on a NEW artifact, validate
  the method on the real DEFECT artifact at authoring while it exists
  (retention windows close): one measured positive (LPic=2 on the R8 dmg)
  converts a conditional corroboration into recorded evidence at near-zero
  cost.
- **H4.** `gh release list` returning empty was treated as "drafts
  deleted" only after the same invocation returned rows against a repo
  known to have releases - the empty-result rule applied to a hosted-API
  surface, where an auth/scope failure also prints nothing.
- **H5 (pattern worth keeping).** The plan's "authoring-time verification +
  tasks re-run the cheap confirmations" split held up under full
  independent re-verification: all vendor claims, all counts, both
  experiment legs, the RTF, and the reconstructed end-state file were
  reproducible exactly. The four-eyes cost went into the checks named
  above, not into re-deriving the route.

## Review evidence trail (commands whose results carry findings)

All foreground, absolute paths from `/home/senol/Git/Muxsmith`. Sweep:
`git grep -niE "unsigned|not signed|notariz"` (tracked tree). Step-6
pre-edit: `git grep -nE 'unsigned on all three|All 1.0-era builds are
\*\*unsigned\*\*|unsigned builds|the app is unsigned' -- README.md
docs/INSTALL.md docs/ROADMAP.md .github/release/` -> 4 hits (listed in
M1). Experiment: baseline + `-c` probe as described (scratch instrument
under `scratchpad/plan85-review-peer-check-*`). Artifacts:
`gh run download 30273529210 -n muxsmith-macos-arm64 -n
muxsmith-windows-x86_64` to scratch; dmg LPic=2, listing CodeResources=0,
MacOS/muxsmith=2; msi Permission=1, Muxsmith=3. gh usage in this review
was read-only (release list x2, run list, artifact list/download, source
fetches from tauri-apps/tauri); recorded here because the reviewer cannot
write the git-ignored `gh-log.md` under the one-file constraint.
