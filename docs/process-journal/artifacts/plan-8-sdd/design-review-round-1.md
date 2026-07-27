# Plan 8 design review, round 1

Artifact: `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(commit d7e79ba, D75-D90). Reviewed against the controller brief
`.superpowers/sdd/plan-8/design-brief.md`, the superseding CLI-distribution
owner ruling (docs/ROADMAP.md Plan-8 block, commit b5678bd), the Tier-2 house
files, the actual tree, and live sources (all lookups re-run independently
2026-07-22/23; nothing believed from the design or from memory).

## VERDICT: NEEDS FIXES

One Major internal inconsistency in the acceptance test (a checklist
observable no workflow step produces, inside a design that freezes the step
list), plus four Minors. Everything load-bearing is correct: every external
claim I re-verified live checks out, all eight rulings appear with unmodified
substance, the latitude scan is clean. The fixes are small and local.

## Re-verification audit (dimension 2) - all load-bearing claims CONFIRMED

Every item below re-checked by my own live lookup, not taken from the design:

- **windows-arm64 msi** (carries the arm64 leg): at tag `tauri-cli-v2.11.4`,
  `bundle/windows/msi/mod.rs` maps `Arch::AArch64 => "arm64"` (lines 221,
  354, 436), feeds candle `-arch`, errors "unsupported architecture"
  otherwise; `WIX_URL` = wix3141rtm/wix314-binaries.zip (WiX 3.14.1);
  `main.wxs` has the `<?elseif $(sys.BUILDARCH)="arm64"?>` ->
  `ProgramFiles64Folder` branch. Supported; no NEEDS_CONTEXT needed. Confirmed.
- **Runner labels** (docs.github.com GitHub-hosted runners reference, fetched
  live): `windows-2025` x64 standard; `windows-11-arm` arm64, the only GA
  windows-arm64 label (`windows-11-vs2026-arm` is public preview; no dated
  variant exists - `windows-2025-arm`: zero hits); `macos-15` arm64 (M1);
  `ubuntu-22.04` still offered standard; `ubuntu-26.04` public preview;
  `macos-15-intel`/`macos-26-intel` the only Intel macOS labels. All four leg
  pins valid. Confirmed.
- **Action SHAs** (gh api releases/latest + commits/tag, live): setup-node
  v7.0.0=8207627..., pnpm/action-setup v6.0.9=0ebf4713..., upload-artifact
  v7.0.1=043fb46d..., download-artifact v8.0.1=3e5f45b2..., tauri-action
  action-v1.0.0=1deb371b..., checkout v7.0.0=9c091bb2... - every SHA matches
  its claimed tag and is the current latest. Confirmed.
- **Cargo version inheritance (D87)**: live schema.tauri.app/config/2,
  `version` description verbatim: "If removed the version number from
  `Cargo.toml` is used." Confirmed.
- **Schema spot-checks**: `deb.recommends`/`rpm.recommends` exist (quoted
  descriptions verbatim); BundleType enum exactly the 7 claimed values, no
  tar.gz; `createUpdaterArtifacts` default false; externalBin sidecar
  pattern; upgradeCode `.x64` derivation text; category list carries `Video`;
  `minimumSystemVersion` default 10.13; DmgConfig = 5 presentation fields
  only; deb has `section`, rpm has none; mainBinaryName defaults to the
  cargo binary. All confirmed.
- **Landing spots at the pinned tag**: msi - `generate_binaries_data` strips
  `-{target-triple}` (code read), `{{#each binaries}}` component loop in
  `main.wxs` beside the main exe; macOS - `app.rs` copies external binaries
  to `Contents/MacOS`; deb/rpm - `usr/bin` incl. external binaries;
  AppImage - `linuxdeploy.rs` reuses `debian::generate_data`; dmg bundler
  builds the .app itself ("generate the .app bundle if needed");
  tauri-build's sidecar-name-collision error string present; deb/rpm
  webkit/gtk hard-Depends auto-injection in `interface/rust.rs` (lines
  1443-1446). All confirmed.
- **WiX PATH inertness**: zero `<Environment` elements in `main.wxs`
  (positive control: 53 Component/Directory hits), `PathEnvVarFeature`
  locale string present but unreferenced by any functional element,
  `WixUI_InstallDir`, `InstallScope="perMachine"`. Confirmed.
- **Local pinned CLI**: `tauri-cli 2.11.4`; `--bundles` "Space or comma
  separated" (comma form of the Linux leg valid); `-c` merge with the
  "different build flavors" sentence verbatim; `--ci` exists;
  `tauri inspect wix-upgrade-code` re-derives exactly
  `9262b417-b687-5ea3-ace1-18b9d51b215f`. Confirmed.
- **gh 2.94.0**: `--draft`, `--verify-tag` (help text verbatim),
  `--notes-file`, `--target`, `run list --commit/--workflow`; help confirms
  drafts stay modifiable/deletable pre-publish. generate-notes REST: "The
  generated release notes are not saved anywhere." Confirmed.
- **Ecosystem docs**: Apple Sequoia Gatekeeper text verbatim (Control-click
  override removed, Settings path) - brief-note 2's correction is
  evidence-backed and leaves the ruling substance untouched;
  aarch64-pc-windows-msvc listed Tier 1 with host tools;
  node-v26.5.0-win-arm64.zip exists; windows-11-arm image ships Rustup
  1.28.2/Rust 1.92/Node 24.12/VS Enterprise 2022; Tauri docs: msi
  Windows-only, pipelines example = tauri-action + releaseDraft on
  ubuntu-22.04, AppImage "oldest base system" guidance naming Ubuntu
  22.04/Debian 12 + libwebkit2gtk-4.1-dev; pnpm/action-setup version
  "Optional when there is a packageManager field". All confirmed.
- **SI-3 spot-checks** (~/Downloads/mkvtoolnix + live downloads page): NSIS
  `File "../*.exe"` (line 163), zero PATH/EnvVar writes (positive control:
  4 SetOutPath hits); macos/build.sh strips the five binaries in
  `MacOS/` (line 480); debian control.erb splits mkvtoolnix/mkvtoolnix-gui,
  `Section: video`; rpm spec `Name: mkvtoolnix` + `%package gui`; downloads
  page: installer + portable 7z, DMGs, AppImage glibc 2.28+; Fedora ships
  the split. Installed `mkvmerge v100.0` re-run. No literal text adopted.
  All confirmed.
- **Tree facts**: tauri.conf.json/Cargo.toml/package.json/mise.toml/
  rust-toolchain.toml exactly as stated; README placeholder at line 99
  verbatim; BUILDING.md has the "Building and running" section the rider
  targets; `.gitignore` lacks `src-tauri/binaries/`; bin names
  `muxsmith`/`muxsmith-gui`; `cli.rs:10` attribute verbatim; ci.yml:
  `v*` tag trigger, workflow-wide `contents: read`, same checkout SHA, same
  apt list, Swatinem cache present (so "ci.yml keeps its cache" is true);
  all cited house convention IDs exist in the Tier-2 files. Confirmed.
- **Fire-verified my own negative checks**: the awk workspace-version parse
  run against the real Cargo.toml prints 0.1.0; the Environment/PATH greps
  carried positive controls as listed above.

## Findings

### Major

**M1. Rehearsal checklist step R5 names an observable no workflow step
produces - and section 11 forbids adding it.**
Location: section 8 R5 ("leg logs: the enumerated globs `bundle/**/*.sig`,
`bundle/**/latest.json` found nothing - and the positive control...") and
D76 ("The rehearsal checklist asserts the built bundle directories contain
no updater artifacts... with its enumerated glob"). The section-2 workflow
has no step that runs these globs (steps: checkout, rustup, apt, node
parse, setup-node, pnpm, install, CLI stage, build, rename+tar, upload) -
so R5's leg-log observable cannot exist. Section 11 pins "the section-2
job/step structure ... as written", so an implementer cannot add the step
without violating the must-not-decide list; and a post-hoc local check is
structurally blind because the workflow artifacts carry only the renamed
release assets, never the bundle output dirs where `.sig`/`latest.json`
would appear. The acceptance test is the plan's proof of the D76 ruling;
as written this item is unexecutable. Fix (small, at source): add the
enumerated glob-check step (globs + positive control, echoing results) to
the bundle job in section 2, and keep R5 pointed at that step's log.

### Minor

**m2. Miscounted, misplaced citation: the stale x64 comment.**
Location: section 1.2, first bullet: "Two stale comments in that file
[`main.wxs`] still say 'only supported platform is Windows x64'". Reality
at tag `tauri-cli-v2.11.4`: exactly ONE such comment, and it sits in
`mod.rs` (fn `build_wix_app_installer`'s doc line: "For now the only
supported platform is Windows x64."), not in `main.wxs`; `main.wxs`
contains no such comment (GitHub code search over the repo: mod.rs only).
The substantive point (stale comment contradicted by the code beneath it)
survives; count and file are wrong. This is the house zitat-und-zahl class.
Fix: "a stale doc comment on `build_wix_app_installer` (mod.rs) still
says...".

**m3. Fabricated-wording quote of gh help in D77's rationale.**
Location: D77 Rationale: "(gh help, immutable-releases section: 'create
releases as drafts first, attach all assets, and then publish')". That
string does not appear in `gh release create --help`. The actual text:
"When using the `create` command to attach assets to a release, separate
API calls are made to create the release as a draft, upload the assets,
and then publish the release." - a description of the command's internal
mechanics, not a phrased recommendation for asset-heavy releases. The
substance (draft-then-publish is gh's native flow; drafts stay
modifiable/deletable) is true and separately confirmed. Fix: replace the
quote with the real sentence or paraphrase without quotation marks.

**m4. `pick()`'s failure diagnostic is swallowed.**
Location: section 2, rename step. `pick` echoes its
"::error::expected exactly one artifact..." message to stdout, but every
call site is a command substitution (`msi="$(pick ...)"`), so on the
failure path the message is captured into the dead variable and never
reaches the log; the leg fails (exit 1 propagates through `set -e`) with
no diagnostic - defeating the count assertion's purpose as the falsifiable
form. Fix: emit the message to stderr (`>&2`).

**m5. D79's wording vs section 2 on the artifact-only dispatch run.**
Location: D79 Decision ("otherwise the dispatch run ends at workflow
artifacts + SHA256SUMS") vs section 2, where on run A the assemble job
generates SHA256SUMS and `cat`s it to the log but uploads it nowhere; R2's
7-file count confirms no persisted SHA256SUMS is expected on run A. The
YAML is internally consistent; the D79 sentence overstates. Fix: either
reword D79 ("...ends at workflow artifacts; SHA256SUMS is generated and
logged but only persisted on the draft path") or upload it - the reword is
the smaller true change.

### Observations (no fix required)

- **o1.** Status line says "Plan 7.5's parallel design owns D65-D74";
  plan-7.5 actually allocated D65-D72. Harmless as a range reservation
  (ROADMAP assigns Plan 8 "D75+"), no collision either way.
- **o2. D87 drift latency.** The guard runs only in release.yml, so
  package.json drift (or a reintroduced tauri.conf `version` key) first
  surfaces after the tag exists; recovery is delete+re-tag. The brief's
  impossibility criterion IS met (no artifact ever ships mismatched), and
  the design's rejection of bump tooling is sound. But the house
  committed-generated-plus-drift-check precedent gates drift pre-tag in
  ci, and the controller has since ruled Plan 8 absorbs the ledger-lint
  CI-wiring rider (ROADMAP, post-design) - that rider is a free carrier
  for the same ~5-line script invocation at plan-authoring time. Option
  for the plan brief, not a design defect.
- **o3.** R6 lists content checks for msi/deb/rpm/AppImage/tar.gz but no
  direct dmg listing; R8's on-hardware run of the documented
  `Contents/MacOS/muxsmith` location covers the dmg claim. Acceptable as
  designed.

## Dimension pass status

1. Brief + ruling compliance: PASS - all seven kickoff rulings + the
   second-round CLI ruling present with unmodified substance (checked
   against the ROADMAP text); D82 steelmans both required rejected
   alternatives (GUI-only installers, PATH option) plus two more; brew
   cask correctly not proposed; Linux one-package divergence recorded with
   the split's steelman; the superseded assumption handled via section-0
   note 1 with commit citation.
2. Registry verification: PASS with two citation defects (m2, m3) - every
   load-bearing claim independently confirmed live (audit above).
3. Latitude scan: PASS - no explicit latitude clause, no omission latitude
   found; every set enumerated, every shipped file verbatim, section 11
   closes the frame. (M1 is the one place where the frozen frame and the
   checklist disagree - an inconsistency, not latitude.)
4. Workflow architecture: PASS - compose-not-conflict verified against the
   actual ci.yml (`v*` trigger runs the full matrix on tag push; the guard
   consumes its verdict via `gh run list --commit`, flags verified on gh
   2.94.0); same-SHA guard implementable as designed (both workflows see
   the same head SHA from the same push event; the dispatch path's SHA has
   its ci run from the branch push); shared job definitions with exactly
   two conditional points, no forked copy; permissions least-privilege per
   house precedent; no mise, every replacement pinned (setup-node/pnpm
   SHAs verified, rustup reads the committed pin, apt unpinned exactly as
   the house ci.yml already does).
5. Bundle config: PASS - overlay `-c` mechanism verified on the installed
   CLI incl. the build-flavors sentence and the auto-merge filenames the
   overlay name avoids; externalBin landing spots byte-checked at the
   pinned tag (INSTALLDIR / Contents/MacOS / usr/bin); recommends native
   in the live schema; tar.gz contents complete (both binaries, LICENSE,
   README naming webkitgtk 4.1 + mkvtoolnix + glibc floor).
6. Version-sync guard: PASS - all three failure paths walked (tag
   mismatch, package.json drift, reintroduced key); the absence assertion
   closes the bypass; G1-G3 fire-tests are break-observe-restore with a
   green-reachable control. See o2 for the latency observation.
7. SI-3 parity + licensing: PASS - spot-checks at the source tree all
   confirmed; facts only, no text adoption; divergences recorded with
   steelmen and the Windows-portable gap routed to trigger T6.
8. No-work-needed checks: PASS - every "unnecessary" premise re-run
   (arm64 support, schema recommends, ci.yml tag trigger + cache, updater
   default-off, rpm section absence).
9. ADR quality: PASS - sixteen entries D75-D90, no numbering collision
   (o1); decision/rationale/steelmanned rejections/interface notes present
   throughout; D81's missing rejected-alternatives slot is proper for a
   pure scope ruling; triggers named and consolidated in section 9 for
   ROADMAP mirroring.
10. Acceptance test: FAIL on M1 (R5 unexecutable as written); all other
    checklist items name real observables, the falsifiability controls are
    genuine (R3 skip-observable, R4 corrupt-one-byte, R7
    positive-control ls-remote), legs and draft path covered.
11. INSTALL.md scope: PASS - per-OS outline complete and consistent with
    the unsigned ruling; the macOS mechanics correction is evidence-backed
    (Apple text verified verbatim); placement argued with two steelmanned
    rejections; the file names its own obsolescence condition.

## HARVEST

- **Dominant pattern, positive:** verified-negative-with-positive-control
  is now reflexive house practice - the design uses it four times (WiX
  Environment grep, NSIS PATH grep, R3, R7) and it survived my re-runs
  every time. Worth naming in the process conventions as the standard form
  for absence claims in design docs, since it made this review's dimension
  8 nearly free.
- **Dominant pattern, rejection side:** four rejections on one shared
  ground - third-party convenience actions/tools vs first-party pinned
  mechanisms (tauri-action, softprops/action-gh-release, cargo-dist,
  mise-action), each with an honest value inventory and a recorded revisit
  condition. This is a coherent house line ("fewer moving third-party
  parts under pin-everything") that could be distilled into a single
  convention entry instead of being re-argued per design.
- **Matrix-closure discipline:** a second rejection family (NSIS, universal
  binary, per-OS portable archives, CLI/GUI package split) all resolve to
  "the ruled matrix is closed; the gap gets a trigger, not scope creep" -
  the trigger registry (9 entries) is doing real work as the pressure
  valve.
- **Citation-precision regression (m2, m3):** both defects are the
  documented zitat-und-zahl class - a count not measured, a quote not
  copied - inside a design whose load-bearing verification is otherwise
  immaculate. The pattern: decorative corroboration gets less rigor than
  load-bearing claims. The existing rule covers reports; design documents
  need it applied with the same force.
- **Over-restriction flag:** section 11's freeze ("the section-2 job/step
  structure ... as written") is what escalates M1 from a trivial gap to a
  blocking inconsistency - a zero-latitude design that freezes a sketch
  must first prove the sketch produces every observable its own acceptance
  test names. A one-line authoring check ("every checklist observable
  names the step that emits it") would have caught it.
- **No over-restriction found in the artifact/config surface**: the
  enumerations (asset set, leg ids, bundle targets, apt list) all match
  what the rulings require - nothing legitimate is banned.

Reviewer: independent design review, round 1. All lookups live
2026-07-22/23; review file is the only write.

---

# Delta review (round 2)

Delta under review: commit 1a602ab on
`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(the decision-ledger.yaml hunk is controller bookkeeping, not graded).
All five round-1 findings applied, none disputed, plus two folded
corrections. Every load-bearing delta claim re-verified by my own runs
and lookups, 2026-07-23.

## VERDICT: APPROVED

## Per-finding disposition

- **M1 - FIXED.** New step "Assert no updater artifacts were produced
  (D76)" sits in every bundle leg between "Build bundles" and the rename
  step (section 2, verified in the current file: lines 1356/1359/1375);
  the frozen section-2 structure therefore produces R5's observable
  itself. D76 and R5 both point at the step; R5's expected summary line
  matches the step's actual output format. Fire-test G4 reproduced
  locally against the committed script body: green
  (`updater-artifact check: 0 hits across 1 bundle output files`,
  exit 0), planted `app.msi.sig` red (`::error::1 updater artifact(s)
  found...`, exit 1), and missing-tree control-red (positive-control
  error, exit 1) - all three byte-identical to the outputs G4 records.
  The non-empty-tree positive control plus G4's planted red together
  close the pattern-fires question.
- **m2 - FIXED, and the correction improves on my round-1 count.** My
  own re-grep of the tag-pinned `mod.rs` with the author's broader
  pattern ("only support") yields exactly the two claimed hits: line 427
  "For now the only supported platform is Windows x64." (doc line above
  `pub fn build_wix_app_installer`) and line 457 "// target only
  supports x64." (inside that function); `main.wxs`: zero. Both quotes
  verbatim. (Round 1's "exactly one" rested on my narrower pattern
  "only supported"; the re-measured claim is the accurate one.)
- **m3 - FIXED.** The replacement quote is character-for-character
  identical to live `gh release create --help` (gh 2.94.0, re-run
  2026-07-23); the overreach ("GitHub's own recommendation") is replaced
  by the accurate "the create command's own native flow"; the
  modifiable/deletable claim is correctly marked as paraphrase and
  matches the help's immutability section.
- **m4 - FIXED.** `pick()` now routes all diagnostics to stderr and logs
  each selection (`pick: <path>`). G5 reproduced locally: one match ->
  `pick:` line on stderr, path intact on captured stdout, exit 0; two
  matches and zero matches -> the `::error::` diagnostic survives the
  command substitution and reaches the terminal, exit 1. Byte-identical
  to G5's recorded outputs. R1 consumes the new `pick:` lines.
- **m5 - FIXED.** D79 now states the actual behavior (SHA256SUMS
  generated and logged by assemble on every run, persisted as a release
  asset only on the draft path). Stale-reference sweep of the whole
  file: no copy of the old sentence, no "Two pre-merge", no orphaned
  `bundle/**` globs, no old gh quote remains ("Five pre-merge local
  fire-tests" and G1-G5 consistent).

## Folded corrections and the checklist-walk claim

- **o1 fold - verified.** Status line now records D65-D74 as Plan 7.5's
  reservation with its design using D65-D72; matches my own count of the
  7.5 spec's `^## D` headings.
- **Ledger-lint text - verified against the ROADMAP.** Sections 7 and 10
  now record the controller ruling (rider task at plan authoring); the
  quoted fragment "not a design amendment (nothing in the plan-8 design
  depends on it)" matches the ROADMAP ledger-hygiene entry verbatim.
- **Checklist walk - verified.** All ten R-observables now name their
  emitting step or command: R1 (guard-step logs, `pick:` stderr lines
  1/1/3, rename step's closing `ls -l` listing 1/1/4 files - both counts
  correct against the workflow mechanics: pick runs 3x on Linux, the
  tar.gz bypasses it), R2 (upload steps), R3 (skipped-step job view),
  R4 (assemble release step, now named), R5 (the new assert step),
  R6/R7/R9 (owner-run commands with R7's positive control), R8/R10
  (owner actions). The author's self-caught R1 fix is correct.

## New findings

None at severity. One nit, no fix required: R5 quotes the step name
without its "(D76)" suffix ("Assert no updater artifacts were produced"
vs the step's full `name:`) - an unambiguous prefix reference.

## HARVEST additions

- The author's checklist walk surfaced a second emitter-less observable
  (R1) beyond the one reported - confirming round 1's harvest point that
  "every checklist observable names the step that emits it" is a
  productive mechanical authoring check, not just a review lens. The
  controller has ledgered it per the commit message.
- Fire-tests that ship with their recorded expected outputs ("already
  run at design time ... exactly these outputs; re-run pre-merge
  against the committed workflow text") made this delta review nearly
  mechanical: G4 and G5 both reproduced byte-identically on first run.
  Worth keeping as the standard form for scripted guards in design docs.

Reviewer: resumed original reviewer, delta round. G4/G5 reproduced
locally; m2 re-grepped at the pinned tag; m3 re-checked against live gh
help; the review file remains the only write.

---

# Delta review (round 3): amendment A1

Delta under review: commit d21a19f (amendment A1 - the section-2 fence's
setup-node step name was an unquoted YAML scalar containing ": ",
execution-surfaced by T4's NEEDS_CONTEXT, controller-ruled Option A).
All verification below re-run by me, foreground, 2026-07-27.

## VERDICT: APPROVED

## Verification

1. **The amended fence parses.** I extracted the section-2 fence from
   the current file myself (222 lines, matching the amendment's stated
   extent) and ran `yaml.safe_load` on PyYAML 6.0.3 (same version the
   author claims): `yaml-ok`. Red-before-green reproduced: the same
   extraction from the pre-fix commit (1a602ab) is rejected with
   "mapping values are not allowed here", line 94, column 59 - the
   amendment's recorded line and column, exactly.
2. **String byte-identical.** `diff` between the two extracted fences:
   exactly one line (94), quoting only. Programmatic check: the
   double-quoted inner string is byte-identical to the pre-fix plain
   scalar, and the YAML-parsed `name` value equals the pre-fix raw
   string - so the Actions UI label and the D85 pointer are unchanged,
   and no D-entry, R-block, or G4/G5 content was touched. Confirmed
   independently: the step name occurs exactly twice in the design (the
   amended fence line and the amendment log's own citation); nothing
   else references it.
3. **Sweep re-run, fire-verified.** The exact grep pattern against the
   amended fence: 0 hits; against the pre-fix fence: exactly 1 hit,
   line 94 - the zero has its positive control. The full PyYAML parse
   moreover subsumes the sweep: an accepting parse proves no illegal
   scalar of ANY shape survives, not only this one.
4. **Amendment log complete and honest.** Defect named with its
   mechanism (plain scalar + ": "), provenance real
   (`.superpowers/sdd/plan-8/task-4-report.md` exists, dated before the
   amendment commit), ruling attributed, verification recorded with
   red-before-green, untouched blocks named with a checkable reason
   (verified in point 2). The status line points at the log.
5. **Nothing else in the diff.** Three hunks: the status-line pointer,
   the one quoted fence line, the appended amendment log. Confirmed.

## HARVEST additions (round 3)

- **Named lesson - a frozen sketch must pass its parser before it is
  frozen.** The design froze a "verbatim-ready" YAML fence (section 11)
  and two review rounds - mine - verified its semantics exhaustively
  (triggers, permissions, expressions, action SHAs, even fire-testing
  two script bodies) without ever feeding the fence to a YAML parser:
  the cheapest deterministic check available, one command, and the only
  one that catches this defect class by construction. The general form
  for the house: **when a design document freezes machine-readable
  content (YAML, JSON, a script), authoring AND review each owe that
  content one load/parse under the real parser** - reading it is not
  checking it. This is the falsifiability duty applied to the artifact
  itself, not just its claims. (Same class, smaller instance: the
  fence's `run:` bodies could get `bash -n` for free; G4/G5 covered two
  of them by execution, the rest only by eye.)
- **The escalation chain held.** The implementer escalated
  NEEDS_CONTEXT instead of silently fixing a frozen artifact
  (`proc-latitude-clause-boundary` doing its job at exactly the
  section-11 freeze this review tightened); controller ruled; author
  amended with proof; delta review reproduced everything on first run.
  The freeze made the defect loud instead of silently patched - the
  over-restriction flag from round 1 cuts the other way here, worth
  recording as the freeze's benefit side.

Reviewer: resumed original reviewer, amendment round. Fence extracted
and parsed both sides, sweep re-run with positive control; the review
file remains the only write.
