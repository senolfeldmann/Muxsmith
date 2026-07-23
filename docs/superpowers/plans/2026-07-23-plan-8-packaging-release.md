# Plan 8: packaging / release pipeline

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **House deviation from the skill text:** progress NEVER enters this document. No box in this file is ever ticked; the checkbox syntax is structure, not a tracking surface. The tracker is `.superpowers/sdd/plan-8/progress.md`.

**Goal:** implement ADRs D75-D90 (`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`): the packaging/release pipeline - a new `release.yml` (guard job with version-sync and ci-gate-green checks, four native bundle legs, draft-release assemble with SHA256SUMS), the bundle-config rewrite plus the CLI-sidecar overlay, the version-sync guard script, the install/runtime documentation and release collateral, the hand-packed tar.gz leg, unified artifact naming - plus the controller-ruled RIDER (ledger-lint CI wiring bundled with its per-entry duplicate-key extension), closed by the workflow_dispatch REHEARSAL that executes the design's R1-R10 acceptance checklist.

**Architecture:** two waves. Wave 1 runs four parallel worktree streams: stream A (`.worktrees/plan8-a`) is the config chain, serial (Task 1 D87 version sync, then Task 2 D86+D82 bundle config + overlay - same file `src-tauri/tauri.conf.json`); stream B (`.worktrees/plan8-b`) is the verbatim collateral (Task 3: `docs/INSTALL.md`, release-body templates, tar.gz README, README placeholder rider); stream C (`.worktrees/plan8-c`) is `release.yml` itself (Task 4, one file, one task); stream D (`.worktrees/plan8-d`) is the rider (Task 5: ledger-lint duplicate-key extension + ci.yml wiring). All four streams are file-disjoint (enumeration in the dependency section). Wave 2 is Task 6, the rehearsal: it runs on master, writes no repo file, and starts only after every wave-1 merge is gated AND master is pushed - `workflow_dispatch` resolves `release.yml` on the default branch only (controller pushes; standing authorization SI-4). Merge sequentially, nine-part gate after every merge.

**Tech Stack:** GitHub Actions (`release.yml`, new; SHA-pinned actions per design section 1.4), Tauri 2 bundler driven by the repo-pinned `@tauri-apps/cli` 2.11.4 via `pnpm exec` (lockfile), bundled WiX 3.14.1, rustup-pinned Rust 1.96.1 (`rust-toolchain.toml`), `actions/setup-node` + `pnpm/action-setup` (mise.toml/package.json stay the version sources; no mise on release legs, D85), bash+jq+awk guard script, `gh` CLI (runner-image tool; 2.94.0 local), coreutils `sha256sum`, Python 3 + PyYAML 6.0.3 (ledger-lint; CI venv, rider task).

## Global Constraints

- **The design document is the contract**: `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` (D75-D90, owner-approved 2026-07-23 after a one-round four-eyes fix loop; the section-8 R1-R10 rehearsal checklist is this plan's acceptance test). The v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative above it on conflict; flag conflicts, do not improvise. Do not reopen settled decisions or re-derive their rationales.
- **Every fork in this plan is closed.** No task brief, verdict or fix-round dispatch may add a design-latitude clause, in either form: an explicit permission or an omission (an unenumerated set in a normative position). A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** (options, costs against the named invariants, a recommendation) and is routed by the controller before it is resolved. It is never decided at the keyboard. Design section 11 ("What the implementer must not decide") binds every task below.
- **ci.yml scope (D83, section 11 first bullet - carried in its ruled scope):** no design-scoped task modifies `.github/workflows/ci.yml`, at all. The single sanctioned exception is Task 5's rider job - a controller ruling recorded OUTSIDE the design (ROADMAP "Ledger hygiene", 2026-07-22 S22; the design's own section 7 last bullet and section 10 record the rider as "not a design amendment"). The rider is strictly additive: it appends one self-contained job and changes no existing line of ci.yml (verified by a task step). Task 5 carries the full scope adjudication.
- **Tier-2 files are ground truth alongside spec + design.** Entries that bind this plan: `deps-first-party-pinned-over-convenience` (first-party mechanism with repo-pinned versions over convenience wrappers - the ground D84/D77/D85/D88 rejections stand on, and Task 5's setup choice), `design-empirical-claims-reproducible`, `proc-normative-count-recomputed` (every count below is recomputed from its enumeration; a task that changes a set re-recounts), `design-acceptance-observables-have-producers` (every R-observable's emitter is named; Task 6 evaluates at the named emitters), `proc-latitude-clause-boundary`, `proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`, `proc-03-model-assignment` (the model-tier section below), `ci-10-pin-everything`, `code-comment-line-citations-drift`. Cite entries by id; re-verify any `:line` you attach.
- **SI-4 push rules bind**: commits AND pushes on this repo are standing-authorized; agent commits and merges are deliberately unsigned (`git -c commit.gpgsign=false`) with the repo trailer; every push is logged in `gh-log.md`; git commands stay pure (no non-git segment chained into the compound). Pushes are controller actions between/after merges, never implementer actions.
- **Nine-part gate green before any push and after every merge**, per BUILDING.md, run foreground, no subsets: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo deny check`, `pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`. Note: this plan changes packaging config and CI, not product code paths - the gate still runs in full, no subsets.
- **No new runtime or product dependency of any kind** (npm or cargo). CI-side additions are bounded and enumerated: `release.yml` introduces exactly the four pinned actions of design section 1.4 (`actions/setup-node`, `pnpm/action-setup`, `actions/upload-artifact`, `actions/download-artifact`) plus the house `actions/checkout` pin reused; the rider adds a pinned `PyYAML==6.0.3` install inside a throwaway CI venv (registry-verified latest 2026-07-23; matches the local install ledger-lint already requires). Nothing else.
- **Any new GitHub Action is SHA-pinned to its release commit with a version comment** (house precedent, ci.yml policy block; the design's section-1.4 table is the authoritative pin source). No action beyond the enumerated set may be introduced (design section 11: notably no tauri-action, no softprops, no cache action on release legs, no mise-action).
- **No task creates a `v*` tag, publishes, edits or un-drafts any release, or resolves a README `placeholder(1.0)` comment** (D81; D75's rider edits one comment's text, not its placeholder status). The rehearsal drafts are deleted by the OWNER at plan close (R10), not by any task.
- **Commits unsigned** (`git -c commit.gpgsign=false commit ...`), trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`, stage files explicitly - **never `git add -A`**.
- **Typography**: ASCII hyphens, straight quotes, no Unicode ellipsis, in all docs, code comments, YAML and messages. Exception, decided in D86 and carried verbatim: the config VALUES `"Şenol Feldmann"` / `"Copyright (c) 2026 Şenol Feldmann"` keep the `Ş` (correct orthography is not an AI-tell glyph; the publisher-rendering fallback fires only on R8's observable failure and changes exactly the `publisher` field to `"Senol Feldmann"` - an owner close-item, never an implementer call).
- **Foreground runs only in implementer briefs**: no background-run-plus-monitor inside subagents. Task 6's gh watches/polls run foreground with explicit timeouts.
- **gh usage rules**: every gh interaction on this repo gets a `gh-log.md` entry (command, effect, manual-UI equivalent); nothing that costs money (standard runners on this public repo are free, design section 1.3/6); `--help` runs are local and need no entry.
- **Counts are recomputed from their enumerations** (`proc-normative-count-recomputed`): every count in this plan was recomputed from the table or list it summarizes at plan-authoring (2026-07-23); a task that changes a set re-recounts and updates the consuming line in the same change.
- **A verification step whose expected result is an absence is fire-verified once** (`proc-verification-step-must-be-falsifiable`): break the thing deliberately, watch the check fire, restore. The design's G1-G5 fire-tests are re-run pre-merge against the committed text as section 8 mandates (G1-G3 in Task 1, G4-G5 in Task 4); every other absence-shaped step below carries its own fire-verification.
- **Implementer preamble, verbatim in every dispatch**: subagents never call session-relocation tools (EnterWorktree/ExitWorktree or any equivalent), worktrees are plain directories, absolute paths.

## Execution method (binding)

Subagent-driven development (`superpowers:subagent-driven-development`): a **fresh implementer subagent per task**, an **independent reviewer per task** grading against this plan and the design, and a **whole-branch review at the plan close** before the close actions. Progress lives in `.superpowers/sdd/plan-8/progress.md`. This statement is binding per the pre-execution gate; `feedback_use_specified_execution_method` applies.

## Model tiers (proc-03-model-assignment)

Mid tier is the default (judgment implementation); cheap tier only where this plan itself carries the code verbatim, verified transcription-complete against the design at plan-authoring. Every task reviewer runs the mid tier. The controller sets the concrete model parameters explicitly at every dispatch (an omitted parameter is not an assignment); the top model never runs a subagent.

| task | implementer tier | ground |
|---|---|---|
| 1 (D87) | cheap | the plan carries `scripts/check-version-sync.sh` verbatim (design 3.3, verified complete incl. shebang) and G1-G3 are fully scripted procedures |
| 2 (D86+D82) | cheap | the plan carries the full `tauri.conf.json` end state (3.1; top-level key set verified against the live file), the overlay (3.2), the `.gitignore` line (3.4) and the BUILDING.md subsection (4.4) verbatim |
| 3 (collateral) | mid | transcription in nature, but the plan cites design section 4 instead of duplicating ~150 lines of owner-pass-bound prose (citation rule below); the cheap-tier carry-condition is deliberately not met |
| 4 (release.yml) | mid | G4/G5 scratch-tree fire-tests, extraction of committed script bodies, and workflow verification exceed transcription |
| 5 (rider) | mid | new Python logic (duplicate-key loader) + CI job authoring + scope record |
| 6 (rehearsal) | mid | live gh operations and R1-R10 evaluation judgment |

## How this plan cites the design

The design is the settled, measured contract every reviewer grades against. Where it already states the implementation exactly, this plan cites it by section rather than copying it - a second copy of a normative block is a drift surface. **Transcribed here without abbreviation** (the design's hardest-won enumerations, plus the blocks the cheap tier's carry-condition requires, named per the plan brief): the R1-R10 rehearsal checklist (section 8 -> Task 6), the G1-G5 fire-tests (section 8 -> G1-G3 in Task 1, G4-G5 in Task 4), the four-leg matrix (D85 -> Task 4), the 8-asset name set (D89 -> Task 4), the guard script (3.3 -> Task 1), and the config surface (3.1, 3.2, 3.4, 4.4 -> Task 2). Every transcribed block was diffed against the design text at plan-authoring (2026-07-23) and matched byte-for-byte (the plan-7 T21 truncation defect is ledgered; the mechanical diff is the guard against repeating it). Section 2 (the release.yml YAML) is deliberately NOT copied into this plan: Task 4's implementer transcribes it from the design directly and diffs the committed file against the design's fence as a task step - one normative copy on disk, one in the design. Line references into tree files are avoided; the one measured line (README.md:99, measured 2026-07-23) is re-verified by content in Task 3.

## Dependency graph and stream cut

Full edge set (a -> b means b needs a merged or earlier in the same worktree):

- Stream A: `1 -> 2` (same file `src-tauri/tauri.conf.json`; Task 2 transcribes the design-3.1 end state, which presumes Task 1's version-key deletion).
- Tasks 3, 4, 5 have no incoming edges: three mutually file-disjoint streams, branched from the same master state as stream A.
- Wave 2: `{1, 2, 3, 4, 5} -> 6`, every edge load-bearing:
  - `1 -> 6`: the guard job runs `scripts/check-version-sync.sh`; R9 asserts the version self-report D87 establishes.
  - `2 -> 6`: the bundle legs read the rewritten bundle block, the overlay and the staged sidecar; R6 checks the resulting package metadata and content.
  - `3 -> 6`: the assemble job reads `.github/release/draft-body.md` + `rehearsal-banner.md`; the Linux leg packs `packaging/linux-tarball-README.txt`; R4/R5 evaluate them.
  - `4 -> 6`: `release.yml` is the dispatched workflow; `workflow_dispatch` resolves it on the default branch only, so it must be merged AND pushed before any dispatch.
  - `5 -> 6`: the guard's gate-green check consumes the ci.yml run conclusion on the rehearsal SHA, which after the rider includes the `ledger-lint` job - a red ledger mechanically blocks the rehearsal (intended consequence, recorded in Task 5).

File-disjointness of the wave-1 cut (the checkable claim; recomputed at plan-authoring): stream A touches `src-tauri/tauri.conf.json`, `scripts/check-version-sync.sh`, `src-tauri/tauri.bundle.conf.json`, `.gitignore`, `BUILDING.md`; stream B touches `docs/INSTALL.md`, `.github/release/draft-body.md`, `.github/release/rehearsal-banner.md`, `packaging/linux-tarball-README.txt`, `README.md`; stream C touches `.github/workflows/release.yml`; stream D touches `.github/workflows/ci.yml`, `scripts/ledger-lint.py`. Pairwise disjoint. Task 6 writes no repo file (`gh-log.md` is git-ignored).

Candidate seams the brief asked to be decided, decided:

- **Version-sync guard + tauri.conf version-key removal: ONE task** (Task 1). The script's green state (G1) is unreachable while the key exists; splitting would leave an intermediate state that cannot be gated.
- **Bundle-config (D86) + externalBin overlay (D82): ONE task** (Task 2), serialized after Task 1 in stream A - both edit `src-tauri/tauri.conf.json`'s surface, and 3.1's transcription target presumes the key deletion. The D82 collateral (`.gitignore` line, BUILDING.md subsection) rides here because it documents exactly this task's mechanism.
- **INSTALL.md (D75): rides the collateral task** (Task 3) together with the other section-4 verbatim files (4.2 templates, 4.3 tar.gz README, 4.5 README rider): all new files plus one comment edit, file-disjoint from every other stream, one transcription discipline.
- **release.yml (D83/D85/D77/D79): ONE task** (Task 4), not an intra-file serial chain: section 2 carries the complete file, section 11 freezes it, and cutting one frozen artifact across several implementers would review fragments against a whole. The tar.gz packing step (D88) and naming/SHA256SUMS (D89/D90) live inside it, per the brief.
- **The RIDER is its own task** (Task 5): file-disjoint from everything (ci.yml + the lint script), its own ruling chain (ROADMAP, not the design).
- **The REHEARSAL is its own task** (Task 6), wave 2, after merge + push - the sequencing constraint is structural, not a preference.

Merge order: A, then B, then C, then D (the streams are file-disjoint, so any order is safe; this fixed order is determinism, not dependency). Full nine-part gate after every merge. After D merges and gates green: master sanity runs, both foreground - `scripts/check-version-sync.sh` (expect exit 0) and `python3 scripts/ledger-lint.py` (expect exit 0; the red states were fire-verified in-task) - then the controller pushes master (SI-4; gh-log entry), waits foreground for the push-triggered ci run to complete green **including the new `ledger-lint` job** (`timeout 2700 gh run watch <run-id> --exit-status --interval 30`; this observation is the rider's in-CI green-reachable evidence), and only then dispatches wave 2.

## Design-section coverage map

Every D-entry and design section -> the task(s) implementing or consuming it:

- D75 -> Task 3 (INSTALL.md 4.1 + README rider 4.5); consumed by Task 4's draft-body links; R8 (owner, plan close).
- D76 -> Task 2 (no `createUpdaterArtifacts`, no updater config - absence preserved by 3.1's transcription), Task 4 (updater-absence step + G4), Task 6 (R5).
- D77 -> Task 4 (assemble job, `--draft --verify-tag`, body composition via `generate-notes`), Task 3 (draft-body.md), Task 6 (R4/R5).
- D78 -> Task 4 (four-leg matrix, seven artifacts), Task 6 (R2).
- D79 -> Task 4 (two triggers, `rehearse-draft-release` input, shared jobs, `rehearsal-<run_id>` naming, `--target`/no `--verify-tag` on the rehearsal path), Task 6 (runs A and B), plan close (R10).
- D80 -> Task 2 (deb/rpm `recommends`), Task 3 (INSTALL.md Linux section + tar.gz README requirements), Task 6 (R6).
- D81 -> no implementing task by design: carried as the global constraint banning tags/publishing/placeholder resolution; Task 6 ends at the rehearsal; the first real tag is out of scope.
- D82 -> Task 2 (overlay 3.2, `.gitignore` 3.4, BUILDING.md 4.4), Task 4 (CLI build + sidecar staging step), Task 3 (INSTALL.md CLI/PATH sections), Task 6 (R6).
- D83 -> Task 4 (guard job: version-sync arm + gate-green poll, per-job permissions), Task 6 (R1); its ci.yml scoping is adjudicated in Task 5.
- D84 -> Task 4 (`pnpm exec tauri build --ci -c ... --bundles <leg set>`; no tauri-action).
- D85 -> Task 4 (runner pins, toolchain steps without mise, no cache; leg matrix transcribed there).
- D86 -> Task 2 (bundle block per 3.1 incl. upgradeCode, publisher, category, minimumSystemVersion, wix.language), Task 6/close (R8 publisher rendering; fallback protocol is an owner close item).
- D87 -> Task 1 (version-key deletion, guard script 3.3, G1-G3), Task 4 (guard wiring), Task 6 (R9).
- D88 -> Task 4 (tar.gz packing step), Task 3 (tarball README 4.3), Task 6 (R6).
- D89 -> Task 4 (rename step with `pick()`, workflow artifact names, `retention-days: 7`; asset set transcribed there), Task 6 (R2).
- D90 -> Task 4 (SHA256SUMS generation in assemble), Task 6 (R4).
- Section 0 (brief corrections) + section 1 (verified ground truth): read-context for every task; no implementation - do not re-verify what section 1 already pins, and do not contradict it.
- Section 2 -> Task 4 (the verbatim source). Section 3.1 -> Task 2; 3.2 -> Task 2; 3.3 -> Task 1; 3.4 -> Task 2.
- Section 4.1 -> Task 3; 4.2 -> Task 3; 4.3 -> Task 3; 4.4 -> Task 2; 4.5 -> Task 3.
- Section 5 (mkvtoolnix parity audit) -> no code; informs close trigger 6 (windows-portable gap).
- Section 6 (CI cost and restraint) -> Task 4's negative space: no concurrency group, no per-push/per-PR bundling, no extra triggers - verified by Task 4's banned-shape grep.
- Section 7 (deliberately out of scope) -> plan close (nothing below implements any listed item; the ledger-lint bullet is Task 5's ruling record, executed as the rider).
- Section 8 -> Task 1 (G1-G3), Task 4 (G4-G5), Task 6 (runs A/B, R1-R10).
- Section 9 (triggers 1-9) -> plan close (mirror into the ROADMAP).
- Section 10 (open items: none) -> nothing to route.
- Section 11 -> binds every task (global constraints).
- RIDER (outside the design; ROADMAP "Ledger hygiene" rulings 2026-07-22 S22) -> Task 5.

---

## Wave 1

Four parallel worktrees. No task lands on master first; every stream branches from the same master state.

---

### Task 1: D87 - version sync: the guard script, and tauri.conf.json stops declaring a version

**Stream A** (`.worktrees/plan8-a`). Read D87 in full, design 3.3, and section 8's G1-G3. Model tier: cheap (the code is carried below, verbatim).

**Files:**
- Modify: `src-tauri/tauri.conf.json` (delete the top-level `version` key; nothing else)
- Create: `scripts/check-version-sync.sh` (executable)

**Interfaces:**
- Consumes: `Cargo.toml` `[workspace.package] version` (0.1.0 today), `package.json` `version`, `jq` + `awk` (local and runner-image tools).
- Produces: the guard contract release.yml's guard job calls (Task 4): plain run = consistency check, `v<X.Y.Z>` argument = consistency + tag equality; exit 0/1.

- [ ] **Step 1: Delete the `version` key from `src-tauri/tauri.conf.json`**

Remove the top-level `"version": "0.1.0",` line, nothing else. Verify:

```bash
jq 'has("version")' src-tauri/tauri.conf.json
# Expected: false
jq -r 'keys_unsorted | join(",")' src-tauri/tauri.conf.json
# Expected: $schema,productName,identifier,build,app,bundle
# (measured pre-change 2026-07-23: the same list plus "version")
```

The bundler then reads the version from `src-tauri/Cargo.toml`, which inherits `[workspace.package] version` - the schema fallback is verified in design section 1.1; do not re-verify it.

- [ ] **Step 2: Create `scripts/check-version-sync.sh`** with exactly this content (design 3.3, transcribed verbatim; diffed against the design at plan-authoring), then `chmod +x scripts/check-version-sync.sh`:

```bash
#!/usr/bin/env bash
# Version-sync guard (Plan 8, D87). Usage:
#   scripts/check-version-sync.sh          # consistency only
#   scripts/check-version-sync.sh vX.Y.Z   # consistency + tag equality
# Asserts: Cargo workspace version == package.json version;
# tauri.conf.json declares NO version (it inherits Cargo's);
# with an argument: the tag is exactly v<version>.
set -euo pipefail

fail() { echo "version-sync: $*" >&2; exit 1; }

cargo_v="$(awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml)"
[ -n "$cargo_v" ] || fail "could not parse [workspace.package] version from Cargo.toml"

pkg_v="$(jq -r .version package.json)"
[ "$cargo_v" = "$pkg_v" ] || fail "Cargo.toml ($cargo_v) != package.json ($pkg_v)"

tauri_has_v="$(jq 'has("version")' src-tauri/tauri.conf.json)"
[ "$tauri_has_v" = "false" ] || fail "src-tauri/tauri.conf.json declares 'version'; it must inherit from Cargo.toml (D87)"

if [ "$#" -ge 1 ]; then
  [ "$1" = "v$cargo_v" ] || fail "tag $1 != v$cargo_v"
fi

echo "version-sync: OK ($cargo_v)"
```

- [ ] **Step 3: Fire-tests G1-G3** (design section 8, transcribed verbatim; run each foreground, record the outputs in the task report):

```
- **G1**: `scripts/check-version-sync.sh v9.9.9` on the clean tree ->
  must exit 1 (tag arm red state); `scripts/check-version-sync.sh` ->
  exit 0 (green reachable, `proc-check-green-state-reachable`).
- **G2**: temporarily add `"version": "0.1.0"` back to
  `tauri.conf.json` -> plain run must exit 1 (absence assertion fires);
  revert.
- **G3**: temporarily set `package.json` version to `0.1.1` -> plain run
  must exit 1 (equality arm fires); revert.
```

Execution notes: G2's "revert" means re-deleting the key by edit (Step 1's change is not yet committed, so `git checkout` would also revert it) - re-run Step 1's two jq checks after the revert. G3's revert restores `"version": "0.1.0"` in `package.json`; verify with `jq -r .version package.json`.

- [ ] **Step 4: Config-parse sanity without the version key**

Run: `cargo check -p muxsmith-gui`
Expected: clean - src-tauri's build script (`tauri_build::build()`) parses `tauri.conf.json` on every compile, so this is the cheapest full-parse proof that deleting the key breaks nothing.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/tauri.conf.json scripts/check-version-sync.sh
git -c commit.gpgsign=false commit -m "release: version-sync guard script; tauri.conf.json inherits the Cargo workspace version (D87); G1-G3 fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

### Task 2: D86 + D82 - the bundle configuration, the CLI-sidecar overlay, and their collateral

**Stream A**, after Task 1. Read D86 and D82 in full, design 3.1, 3.2, 3.4, 4.4. Model tier: cheap (all four artifacts carried below, verbatim).

**Files:**
- Modify: `src-tauri/tauri.conf.json` (the bundle block; nothing outside it)
- Create: `src-tauri/tauri.bundle.conf.json`
- Modify: `.gitignore` (one line)
- Modify: `BUILDING.md` (one appended subsection)

**Interfaces:**
- Consumes: Task 1's version-free config; the five existing icons under `src-tauri/icons/`; `LICENSE` at the repo root.
- Produces: the bundle surface Task 4's legs build against (`--bundles` values narrow 3.1's `targets` superset); the overlay filename `src-tauri/tauri.bundle.conf.json` release builds pass via `-c` (D84); the staging dir ignore rule.

- [ ] **Step 1: Rewrite `src-tauri/tauri.conf.json` to the design-3.1 end state.** The full file after the change (design 3.1 verbatim; `build` and `app` sections unchanged and elided there AND here - leave them exactly as they are; the live file's top-level key set was verified 2026-07-23 to contain nothing beyond what 3.1 shows):

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Muxsmith",
  "identifier": "io.github.senolfeldmann.muxsmith",
  "build": { "...": "unchanged" },
  "app": { "...": "unchanged" },
  "bundle": {
    "active": true,
    "targets": ["msi", "dmg", "deb", "rpm", "appimage"],
    "publisher": "Şenol Feldmann",
    "copyright": "Copyright (c) 2026 Şenol Feldmann",
    "homepage": "https://github.com/senolfeldmann/Muxsmith",
    "license": "MIT",
    "licenseFile": "../LICENSE",
    "category": "Video",
    "shortDescription": "Rule-based bulk MKV muxing tool",
    "longDescription": "Declare how your MKVs should look. Muxsmith forges the whole library into shape - one profile, hundreds of files, zero clicking.",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "wix": {
        "upgradeCode": "9262b417-b687-5ea3-ace1-18b9d51b215f",
        "language": ["en-US"]
      }
    },
    "macOS": {
      "minimumSystemVersion": "11.0"
    },
    "linux": {
      "deb": { "section": "video", "recommends": ["mkvtoolnix"] },
      "rpm": { "recommends": ["mkvtoolnix"] }
    }
  }
}
```

Every literal is frozen by design section 11 (upgradeCode GUID, publisher spelling with `Ş`, category `Video`, section `video`, minimumSystemVersion `11.0`, the en-US language list). `createUpdaterArtifacts`, `fileAssociations`, `macOS.dmg` and `linux.appimage` stay ABSENT (D76/D86 - absence is the decision, not an oversight). A JSON file cannot carry the upgradeCode warning as a comment; it lives in D86 and the BUILDING.md subsection (Step 4).

- [ ] **Step 2: Create `src-tauri/tauri.bundle.conf.json`** containing exactly (design 3.2):

```json
{
  "bundle": {
    "externalBin": ["binaries/muxsmith"]
  }
}
```

The filename deliberately avoids the auto-merged `tauri.<platform>.conf.json` patterns, so it never applies implicitly; only release builds pass it via `-c` (D82/D84).

- [ ] **Step 3: Add the `.gitignore` line** (design 3.4): one added line, `src-tauri/binaries/`, in the `# JS/Tauri frontend` block (below `src-tauri/gen/`).

- [ ] **Step 4: Append the BUILDING.md subsection** (design 4.4, verbatim) under the `## Building and running` section - insert it at the end of that section, immediately before the `## Tooling quirks` heading:

````markdown
### Reproducing a release bundle locally

Release bundles add the CLI as a bundled sidecar via a build-flavor
overlay (`src-tauri/tauri.bundle.conf.json`); plain `pnpm exec tauri
build` deliberately omits it so dev/test builds need no staging step.
To reproduce what CI ships:

```bash
cargo build --release -p muxsmith-cli
triple="$(rustc -vV | sed -n 's/^host: //p')"
mkdir -p src-tauri/binaries
cp "target/release/muxsmith$( [ "$(uname -o 2>/dev/null)" = Msys ] && echo .exe )" \
   "src-tauri/binaries/muxsmith-$triple$( [ "$(uname -o 2>/dev/null)" = Msys ] && echo .exe )"
pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json
```

Do not change `bundle.windows.wix.upgradeCode` in `tauri.conf.json`:
it is Muxsmith's permanent MSI upgrade identity (design D86).
````

- [ ] **Step 5: Verify**

```bash
python3 -m json.tool src-tauri/tauri.conf.json >/dev/null && python3 -m json.tool src-tauri/tauri.bundle.conf.json >/dev/null && echo json-ok
# Expected: json-ok
jq -r .bundle.publisher src-tauri/tauri.conf.json
# Expected: Şenol Feldmann   (the Ş intact - D86's deliberate orthography)
jq '.bundle.icon | length' src-tauri/tauri.conf.json
# Expected: 5
jq -r '.bundle.targets | join(",")' src-tauri/tauri.conf.json
# Expected: msi,dmg,deb,rpm,appimage
grep -n 'src-tauri/binaries/' .gitignore
# Expected: one hit, inside the JS/Tauri block
```

Run: `cargo check -p muxsmith-gui`
Expected: clean (the build script parses the rewritten config; the overlay is NOT read by normal builds - that absence is D82's point and is exercised only at the rehearsal).

- [ ] **Step 6: Commit**

```bash
git add src-tauri/tauri.conf.json src-tauri/tauri.bundle.conf.json .gitignore BUILDING.md
git -c commit.gpgsign=false commit -m "release: bundle metadata + pinned upgradeCode (D86); CLI sidecar via build-flavor overlay, staging dir ignored, local repro documented (D82)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

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

### Task 4: D83/D84/D85/D77/D79/D88/D89/D90 - release.yml

**Stream C** (`.worktrees/plan8-c`). Read design section 2 in full (the verbatim source), D77, D79, D83, D84, D85, D88, D89, D90, section 6, section 8 (G4/G5), and section 11. Model tier: mid.

**Files:**
- Create: `.github/workflows/release.yml`

**Interfaces:**
- Consumes (at run time, not at authoring - the referenced files land via streams A/B and are NOT present in this worktree; their absence here is expected and is not a defect): `scripts/check-version-sync.sh` (Task 1), `src-tauri/tauri.bundle.conf.json` (Task 2), `.github/release/draft-body.md` + `rehearsal-banner.md` (Task 3), `packaging/linux-tarball-README.txt` (Task 3), `mise.toml` (node version parse), `package.json` (`packageManager` for pnpm/action-setup), `rust-toolchain.toml`.
- Produces: the complete release pipeline - triggers `push.tags: ['v*']` + `workflow_dispatch` with the boolean `rehearse-draft-release` input; guard -> 4-leg bundle matrix -> assemble; the operator-facing contracts (input name, `rehearsal-<run_id>` draft naming, workflow-artifact names `muxsmith-<leg>`, retention 7).

The two enumerations transcribed from the design (the reference every step check below recounts against):

**The four legs (D85):**

```
| leg id | runner | rationale for the image |
|---|---|---|
| `windows-x86_64` | `windows-2025` | house pin (ci.yml test matrix) |
| `windows-arm64` | `windows-11-arm` | the only GA windows-arm64 label (section 1.3); native build (Tier-1 host tools) beats cross-compiling |
| `macos-arm64` | `macos-15` | house pin; arm64 (M1) verified |
| `linux-x86_64` | `ubuntu-22.04` | **compat floor**: Tauri's AppImage guidance mandates the oldest base providing webkitgtk 4.1 and names Ubuntu 22.04; Tauri's own CI example builds on it; artifacts built here run on every >= 22.04-era glibc (mkvtoolnix's AppImage targets glibc 2.28+ in the same spirit) |
```

Per-leg `--bundles` (D84/section 2 matrix): `msi` / `msi` / `dmg` / `deb,rpm,appimage`.

**The 8-asset name set (D89):**

```
1. `muxsmith-X.Y.Z-windows-x86_64.msi`
2. `muxsmith-X.Y.Z-windows-arm64.msi`
3. `muxsmith-X.Y.Z-macos-arm64.dmg`
4. `muxsmith-X.Y.Z-linux-x86_64.deb`
5. `muxsmith-X.Y.Z-linux-x86_64.rpm`
6. `muxsmith-X.Y.Z-linux-x86_64.AppImage`
7. `muxsmith-X.Y.Z-linux-x86_64.tar.gz`
8. `SHA256SUMS`
```

- [ ] **Step 1: Create `.github/workflows/release.yml`** by transcribing design section 2's YAML fence exactly - name, both triggers, workflow-level `permissions: contents: read`, the two policy comment blocks, the guard job (version-sync arm conditional on `github.ref_type == 'tag'`, then the gate-green poll: 30 s x 90 rounds, 45-minute fail-safe), the four-entry `include` matrix with `fail-fast: false`, every leg step in order (checkout, rustup, Linux apt deps, mise.toml node parse, setup-node, pnpm/action-setup, `pnpm install --frozen-lockfile`, CLI build + sidecar staging, `tauri build`, updater-absence assert, rename + tar.gz pack, upload-artifact with `retention-days: 7` and `if-no-files-found: error`), and the assemble job (checkout, download-artifact with `merge-multiple: true`, SHA256SUMS, conditional draft creation with the body composition rehearsal-banner -> template -> generated notes). Guard semantics, poll cadence and per-job permissions exactly as written (section 11); action SHAs and version comments exactly per section 1.4's table plus the house checkout pin.

- [ ] **Step 2: Transcription-fidelity proof**: extract the design's section-2 YAML fence with sed into a scratch file, `diff` it against the committed `.github/workflows/release.yml`, and state in the task report that the diff was empty. A non-empty diff is a defect; the design text wins (section 11).

- [ ] **Step 3: Parse check**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('yaml-ok')"
# Expected: yaml-ok (PyYAML present locally, measured 6.0.3 at plan-authoring)
```

- [ ] **Step 4: Fire-tests G4-G5 against the committed workflow text** (design section 8, transcribed verbatim):

```
- **G4**: the updater-absence step's script body (section 2) against a
  scratch tree: clean tree with one bundle file -> summary line
  `updater-artifact check: 0 hits across 1 bundle output files`, exit 0;
  planted `app.msi.sig` -> `::error::1 updater artifact(s) found`,
  exit 1; missing/empty bundle dir -> positive-control error, exit 1.
  (Already run at design time, 2026-07-23, exactly these outputs;
  re-run pre-merge against the committed workflow text.)
- **G5**: the rename step's `pick()` against a scratch dir: one match ->
  `pick: <path>` on stderr, path on stdout, exit 0; zero matches and two
  matches -> `::error::expected exactly one artifact, got: ...` on
  stderr, exit 1. (Already run at design time, 2026-07-23, exactly these
  outputs; re-run pre-merge against the committed workflow text.)
```

Execution notes: "against the committed workflow text" means the script bodies are EXTRACTED from `.github/workflows/release.yml` (slice the step's `run: |` block with awk/sed and dedent), never retyped from the design. G4's body is pure bash (no `${{ }}` expressions) and runs wholesale in a scratch dir carrying a fabricated `target/release/bundle/` tree. For G5, extract the `pick()` function definition into a scratch script and invoke it with one, zero and two matching files. All runs foreground; record the six observed outputs in the task report.

- [ ] **Step 5: Negative-space check** (section 6/section 11; absence check, fire-verified with a positive control):

```bash
grep -n 'tauri-action\|softprops\|rust-cache\|mise-action\|Swatinem\|concurrency' .github/workflows/release.yml
# Expected: no output (none of the banned shapes; no concurrency group).
# Positive control (proves the pattern fires): the same grep against
# .github/workflows/ci.yml hits its mise-action and Swatinem/rust-cache lines.
```

- [ ] **Step 6: Pin conformance** (recount):

```bash
grep -c 'uses:' .github/workflows/release.yml
# Expected: 7 (guard checkout, leg checkout, setup-node, pnpm/action-setup,
# upload-artifact, assemble checkout, download-artifact - recomputed from
# section 2; every one a 40-hex SHA with a version comment)
grep -n 'uses:' .github/workflows/release.yml | grep -v '@[0-9a-f]\{40\} # v'
# Expected: no output (every uses line SHA-pinned + commented). Fire-verify:
# temporarily change one SHA-pin line to @v7 in the working copy, see the
# second grep emit it, restore.
```

- [ ] **Step 7: Commit**

```bash
git add .github/workflows/release.yml
git -c commit.gpgsign=false commit -m "release: release.yml - guard (version sync + ci-gate green), four native bundle legs, draft-release assemble with SHA256SUMS (D77/D79/D83-D85/D88-D90); G4/G5 fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

### Task 5: RIDER - ledger-lint duplicate-key extension + CI wiring

**Stream D** (`.worktrees/plan8-d`). Read the two ROADMAP "Ledger hygiene" rider rulings (2026-07-22 S22), `scripts/ledger-lint.py` in full (including its docstring's house-pattern note and its now-consumed CI-wiring deferral sentence), and D83's decision + rationale. Model tier: mid.

**Controller ruling (binding, recorded in the ROADMAP, not the design):** wire `scripts/ledger-lint.py` into CI AND extend it with the per-entry duplicate-key check - both deferral triggers fired 2026-07-22 (S22); one task.

**Scope adjudication (recorded here so no reviewer re-derives it):** D83 decides the RELEASE pipeline's placement - a separate `release.yml`, ci.yml untouched by the release path - and section 11 restates that for the design's own implementers ("ci.yml is not modified, at all"). The rider is not release-pipeline work and not design scope: the design itself records it as outside ("not a design amendment; nothing in the plan-8 design depends on it" - section 7 last bullet, section 10, and the ROADMAP ruling's own wording). Resolution: no collision - the rider adds one self-contained, additive job to ci.yml and changes no existing line (Step 5 verifies exactly that), while every design-scoped task remains bound by section 11's ban. A separate lint-workflow file was weighed and rejected: it would duplicate trigger/checkout boilerplate, fragment the gate surface, and forfeit the free coupling that makes a red ledger block a release - D83's gate-green check consumes ci.yml run CONCLUSIONS, so a `ledger-lint` job inside ci.yml gates the release path at zero extra wiring (this consequence is intended and is the `5 -> 6` dependency edge). The deny job is the house precedent for exactly this shape: a cheap, independent hygiene job inside ci.yml.

**Files:**
- Modify: `scripts/ledger-lint.py`
- Modify: `.github/workflows/ci.yml` (additive only: one appended job)

**Interfaces:**
- Consumes: the four house YAML files (`docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`, `docs/decision-ledger.yaml`), PyYAML (local install measured 6.0.3; CI venv pins the same).
- Produces: ledger-lint check 6 (duplicate keys) and the ci.yml `ledger-lint` job every future push/PR runs.

- [ ] **Step 1: Extend `scripts/ledger-lint.py` with check 6 - duplicate keys.** The contract (closed; the code shape within these bounds is the implementer's):

- **Detection mechanism**: a `yaml.SafeLoader` subclass that detects duplicate keys during mapping construction (PyYAML's documented `construct_mapping` extension point), used for the whole-file load. NOT a regex or line heuristic - the script's own docstring records the real-parser principle, and a linter that must be trusted keeps it.
- **Scope**: any mapping anywhere in any of the four files (top-level entry mappings and every nested mapping). This is a superset of the ROADMAP's "per-entry" requirement and is strictly simpler than entry-scoping the loader.
- **Violation format**: one `FAIL` line per duplicate, naming the file, the key, and both 1-based line numbers from the YAML node marks (e.g. `FAIL docs/process-conventions.yaml: duplicate key 'steelman' (lines 61 and 63)`); aggregated with the existing violations and the existing summary/exit semantics (exit 0 clean, 1 on any violation).
- **The observed S21 defect shape must be caught**: a doubled `steelman:` line inside one entry (Step 2's fixture reproduces it).
- **Docstring maintenance in the same edit** (the edit is the trigger; sweep the file): the numbered check list gains item 6, and the stale deferral sentence at the docstring's end ("CI wiring is a separate step ... rides the next CI-touching plan, per the ROADMAP") is replaced by one line stating the wiring exists (ci.yml `ledger-lint` job, Plan 8 rider). Verify the sweep: `grep -n "CI wiring" scripts/ledger-lint.py` afterwards shows only the new sentence.
- **Checks 1-5 unchanged** in behavior and output.

- [ ] **Step 2: Fire-test the extension** (foreground; record outputs in the task report):

```bash
# RED (new check): duplicate the steelman: line inside one entry of
# docs/process-conventions.yaml (the observed S21 shape), then:
python3 scripts/ledger-lint.py
# Expected: FAIL line naming the file, key 'steelman' and both line numbers; exit 1.
git checkout -- docs/process-conventions.yaml

# CONTROL (old checks still live after the loader swap): bump one entry's
# count field by 1 in docs/conventions.yaml, then:
python3 scripts/ledger-lint.py
# Expected: the existing "count is N but has M occurrences" FAIL; exit 1.
git checkout -- docs/conventions.yaml

# GREEN (reachable):
python3 scripts/ledger-lint.py
# Expected: "ledger-lint: <N> entries across 4 files, all invariants hold"; exit 0.
```

- [ ] **Step 3: Append the `ledger-lint` job to `.github/workflows/ci.yml`** - exactly this block, added after the `deny` job; no existing line is touched:

```yaml
  ledger-lint:
    # House-knowledge structural integrity (scripts/ledger-lint.py):
    # count==occurrences, refs present, blocked/tier fields, duplicate
    # ids, per-entry duplicate keys. Rider on Plan 8 by controller
    # ruling (ROADMAP "Ledger hygiene", 2026-07-22 S22) - additive job
    # only; D83's "ci.yml is not modified" is scoped to the release
    # pipeline and holds. PyYAML pinned; the interpreter floats with
    # the runner image (same recorded shape as brew above).
    runs-on: ubuntu-26.04
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
      - name: ledger-lint (house YAML invariants)
        run: |
          python3 -m venv "$RUNNER_TEMP/ledger-lint-venv"
          "$RUNNER_TEMP/ledger-lint-venv/bin/pip" install PyYAML==6.0.3
          "$RUNNER_TEMP/ledger-lint-venv/bin/python" scripts/ledger-lint.py
```

Pin rationale, recorded: PyYAML==6.0.3 is the registry-latest (pypi.org, verified 2026-07-23) and matches the local install the script already runs under. The venv is the first-party mechanism (`deps-first-party-pinned-over-convenience`); a pinned `actions/setup-python` was weighed and rejected - it adds an action pin whose only value here is an interpreter the image already ships, and the stdlib venv sidesteps PEP-668 system-python concerns without it. The runner label matches the file's existing jobs (ubuntu-26.04); no new runner family.

- [ ] **Step 4: Run the job's exact step commands locally** (green proof of the step, foreground):

```bash
RUNNER_TEMP="$(mktemp -d)" bash -c '
  python3 -m venv "$RUNNER_TEMP/ledger-lint-venv"
  "$RUNNER_TEMP/ledger-lint-venv/bin/pip" install PyYAML==6.0.3
  "$RUNNER_TEMP/ledger-lint-venv/bin/python" scripts/ledger-lint.py
'
# Expected: pinned install succeeds; "all invariants hold"; exit 0.
```

The in-CI red state is deliberately NOT exercised: ci.yml triggers only on master pushes, PRs and dispatch, so a red run would require pushing a broken ledger - the red is proven locally (Step 2) plus the platform's core semantic that a nonzero step exit fails the job; the in-CI GREEN is observed after the merge-order push (merge-order section above) as the wiring's green-reachable evidence.

- [ ] **Step 5: Verify the additive-only property** (the D83-compat observable):

```bash
git diff master -- .github/workflows/ci.yml | grep -c '^-[^-]'
# Expected: 0 (no removed/changed existing line; the diff is pure addition).
# Fire-verify: temporarily edit one existing ci.yml line in the working
# copy, see the count go nonzero, restore.
```

- [ ] **Step 6: Commit**

```bash
git add scripts/ledger-lint.py .github/workflows/ci.yml
git -c commit.gpgsign=false commit -m "ci: ledger-lint job (Plan 8 rider, S22 ruling) + per-entry duplicate-key check via SafeLoader subclass; fixture fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Wave 2

One task, on master, after every wave-1 stream is merged, gated, and pushed. It writes no repo file.

---

### Task 6: the REHEARSAL - two workflow_dispatch runs, R1-R10

Read design section 8 in full, D79, D83, and the merge-order section above. Model tier: mid.

**Sequencing constraint (structural):** `workflow_dispatch` requires `release.yml` on the default branch, so this task runs AFTER the workflow lands on master and is pushed (controller pushes; standing authorization SI-4). Preconditions, verified before the first dispatch, in order:

1. All five wave-1 tasks merged, nine-part gate green on master, master sanity runs green (merge-order section), master pushed.
2. The push-triggered ci run on the head SHA completed green including the `ledger-lint` job: `gh run list --workflow ci.yml --commit "$(git rev-parse HEAD)" --json status,conclusion` shows completed/success (else the release guard will poll against a red or absent gate). If the run is still in flight, wait foreground: `timeout 2700 gh run watch <run-id> --exit-status --interval 30`.
3. Baseline for R3: record `gh release list` output (expected today: empty - the repo has no releases).

**gh usage rules bind throughout**: every gh interaction on this repo gets a `gh-log.md` entry - command, effect, manual-UI equivalent (format per the file's header); nothing that costs money; public-repo Actions on standard runners are free (two dispatch runs = 12 jobs total, recomputed: 6 per run - guard, 4 legs, assemble; design section 6).

**Wall-clock note:** four native legs incl. `windows-11-arm` (4-CPU), cold builds by design (no cache, D85). Every wait is a foreground `gh run watch` or a bounded poll with an explicit timeout - **never background-run-plus-monitor**. Ceilings: run-id resolution 10 x 15 s; each `gh run watch` wrapped in `timeout 5400` (90 min); a tripped timeout stops the task and returns to the controller with the run URL and partial evaluation - it is never silently extended.

**The acceptance checklist (design section 8, transcribed verbatim - this is the plan's acceptance test; evaluate each observable at its named emitter):**

```
- **R1** (run A): guard passes both steps; the gate-green step's log
  names the found ci run. All four legs green; the rename step's log
  carries one `pick: <path>` line per selected artifact (1 per Windows
  leg, 1 on macOS, 3 on Linux - the function logs every selection to
  stderr) and its closing `ls -l` lists exactly the renamed files
  (1 / 1 / 4 incl. the tar.gz).
- **R2** (run A): four workflow artifacts named `muxsmith-<leg>` exist
  with retention 7; downloaded together they contain exactly the 7 files
  of D89 with the scheme-conformant names (count check: 7).
- **R3** (run A): **no release was created** (the assemble release step
  shows as skipped in the run's job view - the skip is the observable,
  not an absence-grep; `gh release list` unchanged as corroboration).
- **R4** (run B): draft `rehearsal-<run_id>` exists (created by the
  assemble job's release step), `isDraft: true`, with exactly 8 assets
  (7 + SHA256SUMS). Download all; `sha256sum -c SHA256SUMS` passes.
  Falsifiability control once: corrupt one downloaded byte, watch `-c`
  fail, redownload.
- **R5** (run B): body = rehearsal banner + template with the version
  substituted + generated notes. And in each of the four leg logs, the
  step "Assert no updater artifacts were produced" printed its summary
  line (`updater-artifact check: 0 hits across N bundle output files`,
  N > 0 - the N is the step's built-in positive control that it saw the
  bundle tree; the step's red and control-red states are the G4
  fire-test).
- **R6** (run B or A, artifact-content checks on a Linux machine):
  `dpkg-deb -I muxsmith-*.deb` shows `Recommends: mkvtoolnix` and
  `Depends:` containing `libwebkit2gtk-4.1-0` and `libgtk-3-0`;
  `dpkg-deb -c` lists `./usr/bin/muxsmith` and `./usr/bin/muxsmith-gui`;
  `rpm -qp --recommends muxsmith-*.rpm` prints `mkvtoolnix`;
  `./muxsmith-*.AppImage --appimage-extract` yields
  `squashfs-root/usr/bin/muxsmith`; the tar.gz lists the D88 four-file
  layout under its version-named directory; `msiextract` (msitools) on
  each msi lists `muxsmith.exe` and `muxsmith-gui.exe`.
- **R7** (run B): **no tag ref was created**:
  `git ls-remote origin | grep -c rehearsal` is 0 while the same
  `ls-remote` output demonstrably lists `refs/heads/master` (the
  positive control proving the listing works; the repo currently has no
  tags, so master is the known-present ref).
- **R8** (owner, on real hardware, once per OS): msi installs on a
  Windows machine after the SmartScreen "Run anyway" flow exactly as
  `docs/INSTALL.md` describes; Programs-and-Features shows publisher
  "Şenol Feldmann" unmangled (D86's fallback fires only if this fails);
  dmg opens on an Apple-Silicon Mac via the Settings > Open Anyway flow
  as documented; `muxsmith` runs from the documented per-OS CLI
  locations. These three walk-throughs double as the screenshot source
  for any future doc polish.
- **R9** (run B): version self-report - `./muxsmith --version` from the
  tar.gz prints the Cargo workspace version (D87's artifact-level
  criterion; the guard already pinned tag == that version).
- **R10** (owner): delete the rehearsal draft(s) after inspection.
```

- [ ] **Step 1: Dispatch run A** (artifact path; input defaults to false):

```bash
gh workflow run release.yml --ref master
```

gh-log the dispatch. Resolve the run id by bounded poll (10 tries x 15 s foreground sleep): `gh run list --workflow release.yml --limit 1 --json databaseId,status,event,createdAt` until a `workflow_dispatch` run created after the dispatch timestamp appears.

- [ ] **Step 2: Watch run A foreground**: `timeout 5400 gh run watch <idA> --exit-status --interval 30` (flags verified against gh 2.94.0 `--help` at plan-authoring). Expected: all 6 jobs green.

- [ ] **Step 3: Evaluate R1-R3.** Emitter mapping: R1 - `gh run view <idA> --log` (guard log's gate-green line; per-leg `pick:` stderr lines: recount 1/1/3 against the leg matrix; the rename step's closing `ls -l` listings: 1/1/4 files). R2 - `gh api "repos/senolfeldmann/Muxsmith/actions/runs/<idA>/artifacts" --jq '.artifacts[] | {name, expires_at}'` (four names `muxsmith-<leg id>`; `expires_at` ~7 days out), then `gh run download <idA> --dir <scratch>/runA` and count + name-check the 7 files against the transcribed D89 set (the version token is the current Cargo workspace version - recompute from `Cargo.toml`, today 0.1.0). R3 - `gh run view <idA> --json jobs` shows the release-creation step `skipped` in the assemble job (the skip IS the observable); `gh release list` equals the Step-0 baseline. gh-log every command.

- [ ] **Step 4: Dispatch run B** (rehearsal path):

```bash
gh workflow run release.yml --ref master -f rehearse-draft-release=true
```

Resolve the id as in Step 1; watch as in Step 2 (`timeout 5400`, foreground). Expected: all 6 jobs green, the assemble release step RUNS this time.

- [ ] **Step 5: Evaluate R4-R5.** R4 - `gh release view "rehearsal-<idB>" --json isDraft,name,assets` (`isDraft: true`; asset count exactly 8); download all assets to `<scratch>/runB` - primary `gh release download "rehearsal-<idB>" --dir <scratch>/runB`; if the draft's tagless state defeats the tag-name lookup, fall back to `gh api repos/senolfeldmann/Muxsmith/releases --jq '...'` to get the release id and fetch each asset via `gh api` with `Accept: application/octet-stream` (both shapes gh-log'd; no third variant). Then `sha256sum -c SHA256SUMS` in that directory passes; run the falsifiability control once (corrupt one byte, `-c` fails naming the file, redownload). R5 - `gh release view "rehearsal-<idB>" --json body`: banner first, then the template with the version substituted (no `__VERSION__` survivor - `grep -c __VERSION__` on the body: 0; fire-verify the grep against the committed template file, which has 8), then the generated-notes section; and `gh run view <idB> --log | grep "updater-artifact check:"` yields exactly 4 lines (one per leg), each `N > 0`.

- [ ] **Step 6: Evaluate R6** on run B's downloaded assets. Tool gate first: `command -v dpkg-deb rpm msiextract sha256sum tar`. Measured on the execution machine (Fedora) at plan-authoring: `rpm`, `sha256sum`, `tar` present; **`dpkg-deb` and `msiextract` ABSENT**. If still absent, STOP and request the owner-authorized install through the controller (`sudo dnf install dpkg msitools` - a system change on Şenol's machine is his call, standing rule), then run every R6 check as transcribed. Skipping any R6 check is a defect, not an option.

- [ ] **Step 7: Evaluate R7**: `git ls-remote origin > <scratch>/ls-remote.out`; `grep -c rehearsal <scratch>/ls-remote.out` -> 0 AND `grep -c 'refs/heads/master' <scratch>/ls-remote.out` -> >= 1 (the transcribed positive control; read-only git needs no gh-log entry).

- [ ] **Step 8: Evaluate R9**: extract run B's tar.gz, run `./muxsmith --version` from the extracted version-named directory; output equals the `Cargo.toml` workspace version (awk-parse it; do not hardcode).

- [ ] **Step 9: Route R8 and R10 to the plan close.** Both are owner steps (real hardware; draft deletion). The task report lists them as pending-owner with the D86 publisher-fallback protocol note (fires only on R8's observable failure; changes exactly the `publisher` field to `"Senol Feldmann"`). The task does NOT delete the rehearsal drafts - the draft is the rehearsal's deliverable for the owner's inspection (D79).

- [ ] **Step 10: Report.** R1-R7 + R9 verdicts with the observed values, the two run URLs, every gh-log entry written, scratch cleanup done (downloaded assets removed; the drafts remain for the owner).

---

## Plan close (controller actions, not tasks)

- **Whole-branch review** by the resumed independent reviewer against the design, before any close action (house standing).
- **Owner steps R8 + R10** (pre-registered): the three real-hardware walk-throughs (Windows msi incl. Programs-and-Features publisher rendering - the ONLY trigger for D86's pre-decided fallback, `publisher` -> `"Senol Feldmann"`, no other field; Apple-Silicon dmg via the Settings flow; per-OS CLI locations), then deletion of the rehearsal draft(s). The owner publishes nothing in this plan (D81).
- **Owner rendered-surface pass** over the user-facing collateral wording: `docs/INSTALL.md` (pre-registered by the brief), the draft-body and rehearsal-banner templates, `packaging/linux-tarball-README.txt`, the BUILDING.md subsection, the README rider comment. Structure and facts are the design's; wording is the owner's.
- **Triggers to mirror into the ROADMAP** (design section 9, recomputed: 9): 1 (ubuntu-22.04 retirement -> move the Linux leg to 24.04 + raise the documented floors in the same change), 2 (dated windows-arm64 label appears -> pin it, closing D85's recorded deviation), 3 (tauri/@tauri-apps/cli bump -> re-verify the four pinned bundler facts before the next release), 4 (signing revisit fires -> shrink INSTALL.md per its embedded comment; evaluate artifact attestations first - EXTENDS the existing registered signing trigger), 5 (Intel-dmg request -> `macos-x86_64` leg on `macos-15-intel`, extend D89's set + D77's table - EXTENDS the existing registered Intel trigger), 6 (windows-portable request -> the section-5 7z parity gap becomes a v1.x candidate), 7 (German installer UI request -> reopen D86's `wix.language`), 8 (a tar.gz bundler lands / cargo-dist earns its keep -> revisit D88), 9 (runner-image gh breaks a release-ops invocation -> pin gh by direct versioned download).
- **ROADMAP "Ledger hygiene" bookkeeping**: record the rider as executed (CI wiring + duplicate-key extension DONE via Task 5) on both entries.
- **sdd-scratch citations riding the salvage** (pre-registered; checked 2026-07-23): the design itself contains NO `.superpowers/sdd` citation (grep for `sdd`/`superpowers`/`review-round` over the design: zero hits, positive control against the plan brief fired). What does point into the plan-8 scratch: two decision-ledger occurrence refs naming `design-review-round-1.md` (`design-acceptance-observables-have-producers`, occurrence 1; `proc-quote-verbatim-or-paraphrase`, occurrence 1). Per the ruled round-8 house pattern (ROADMAP Triggers precedent: citations move WITH the salvage in the same change), re-point exactly these two refs to the salvaged artifact path when the plan-8 sdd salvage runs.
- **Journal + HANDOFF snapshot** per SI-2 at the plan close (standing duty; listed for completeness, not new).
