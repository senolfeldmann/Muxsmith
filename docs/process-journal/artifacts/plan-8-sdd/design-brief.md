# Plan 8 design brief (controller-authored)

You author the DESIGN DOCUMENT for Plan 8: the packaging / release
pipeline. Four-eyes: an independent reviewer grades your document against
this brief before the governing human sees it. You write exactly ONE
file: `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`.
Do NOT commit (the controller commits); touch nothing else in the tree.

## Read first

1. docs/ROADMAP.md - the "Plan 8" anchor incl. the S22 KICKOFF block (the
   owner rulings, restated below), the established-state paragraph, the
   two new triggers (signing revisit, Intel-dmg request), and the v1.x
   entries "Remove mise from CI" (stays post-1.0; do not absorb it, but
   do not deepen the mise dependency either - new release legs choose
   their runtime setup with recorded rationale) and "deb/rpm hard
   Depends" (1.x; 1.0 ships Recommends).
2. Spec docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md - the CI/
   packaging sentence in §10 and the stack row in §2.
3. .github/workflows/ci.yml - the existing `v*` tag trigger (today it
   drives only the test matrix; the ROADMAP records it as a KEPT scaffold
   that Plan 8 consumes), runner-image pins, SHA-pinned actions,
   permissions block: these are the house supply-chain precedents.
4. src-tauri/tauri.conf.json (bundle block today: targets "all", icons,
   no per-OS config, no publisher/category; identifier
   io.github.senolfeldmann.muxsmith), root Cargo.toml (workspace version
   0.1.0, crates inherit), package.json (independent 0.1.0), BUILDING.md
   (nine-part gate), mise.toml.
5. Tier-2 house files: docs/process-conventions.yaml (pin-everything,
   model/process rules), docs/conventions.yaml, docs/product-boundaries.yaml.
6. docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md as the
   house STRUCTURAL TEMPLATE (sections, decision-log form, triggers,
   test plan).

## Settled owner rulings (2026-07-22 S22; binding, not re-litigable; each becomes a D-entry)

1. UNSIGNED artifacts on all three OS at 1.0. The design specifies WHERE
   the per-OS install-hurdle documentation lands (README placeholder(1.0)
   riders and/or guide) and its content outline (Windows SmartScreen,
   macOS Gatekeeper right-click-open / quarantine, Linux none). Signing
   revisit is a registered ROADMAP trigger - not design scope.
2. NO auto-updater at 1.0 (v1.x); the bundle config must not require
   updater artifacts or keys.
3. A `v*` tag builds all bundles and attaches them to a DRAFT GitHub
   release; the owner reviews and publishes manually. Never auto-publish.
4. Artifact matrix: Windows x64 msi AND Windows arm64 msi; macOS arm64
   dmg ONLY (no x64 leg; Intel request = registered trigger); Linux x64
   deb + rpm + AppImage + a portable tar.gz ("just runs" archive).
   VERIFY against current authoritative docs (registry-verify, cite
   source + date): Tauri 2 bundling support for windows-arm64 msi and
   the GitHub-hosted runner labels for windows arm64 / macos arm64 /
   linux x64. If windows-arm64 msi is NOT cleanly supported, propose the
   closest supported artifact WITH the evidence and flag it
   NEEDS_CONTEXT - do not silently substitute.
5. Pipeline verification runs via workflow_dispatch: build all legs as
   workflow artifacts, plus a rehearsal input that exercises draft-release
   creation. NEVER a test tag.
6. deb/rpm declare mkvtoolnix as Recommends at 1.0 (hard Depends is a
   recorded v1.x item); AppImage and tar.gz document the runtime
   requirement instead.
7. Plan 8 builds the pipeline; tagging 1.0 is NOT in scope.

## Controller assumption pending owner confirmation (design on this basis; the section must be cleanly amendable if the owner rules otherwise)

At 1.0 the Windows/macOS installers ship the GUI app ONLY (today's
bundle state: no externalBin/sidecar); the CLI ships via the Linux
tar.gz (which carries BOTH binaries) and via `cargo install` from
source. Standalone win/mac CLI artifacts are NOT built; a user request
for them is a trigger candidate. Design the tar.gz contents accordingly
and keep the CLI-distribution decision in its own D-entry so an owner
reversal amends one section, not the matrix.

## The design must RESOLVE

- Workflow architecture: extend ci.yml vs a separate release workflow
  (the `v*` trigger exists in ci.yml today and is recorded as the
  scaffold Plan 8 consumes); interaction between release builds and the
  test matrix (does the release path require the gate green on the same
  SHA, run tests itself, or both) - decide with rationale + steelman.
- Per-leg runner selection with pinned images, matching the house's
  existing pin discipline; every new action SHA-pinned (pin-everything).
- Build tooling per leg: the official tauri-apps/tauri-action vs direct
  `tauri build` steps - an IDIOMACY CHECK duty: verify what the Tauri
  ecosystem currently does from live docs, assess the action's health/
  pinning implications, decide with steelman.
- tauri.conf.json bundle configuration: per-OS blocks, productName/
  publisher/category metadata, msi specifics, dmg specifics, deb/rpm
  metadata incl. the Recommends mechanism (verify the current Tauri 2
  config schema supports it; if not, the leg needs a post-processing
  step - design it), AppImage specifics. Identifier stays
  io.github.senolfeldmann.muxsmith.
- The tar.gz leg: Tauri has no tar.gz bundler for the GUI - design the
  packing step (contents: muxsmith CLI binary + GUI binary + license +
  a short README naming the system requirements incl. webkitgtk and
  mkvtoolnix; naming scheme consistent across all artifacts, e.g.
  muxsmith-<version>-<os>-<arch>.<ext> - decide and record).
- Version sync: Cargo workspace version is the Rust source of truth
  today; tauri.conf.json and package.json restate it independently.
  Decide the mechanism (config inheritance if Tauri 2 supports omitting
  the version / reading it from Cargo - VERIFY live; else a drift-check
  in the gate per the house's committed-generated-plus-drift-check
  pattern, or a sync script). The failure the mechanism must make
  impossible: a tag whose artifacts self-report a different version.
- Checksums: whether the release attaches a SHA256SUMS file (recommend
  and decide; supply-chain hygiene precedent in the house).
- Draft-release body: generated notes vs owner-authored (draft anyway;
  decide the generated baseline).
- Artifact naming, retention of workflow artifacts, and what the
  workflow_dispatch rehearsal produces vs the tag path (shared job
  definitions, not a forked copy - reuse-before-writing applies to
  workflow YAML too).
- CI cost: public repo Actions are free; still record the leg count and
  any deliberate restraint (e.g. no per-push bundle builds).

## Method duties

- SI-3 mkvtoolnix parity where meaningful: mkvtoolnix ships a Windows
  installer AND a portable archive, a macOS dmg, and distro packages -
  direct precedent for the portable-archive artifact; cite what you
  verify (its download page / source tree at ~/Downloads/mkvtoolnix),
  never copy text. Licensing boundary applies.
- EVERY version, runner label, action SHA, and schema claim
  registry-verified live (WebFetch/context7/gh api), cited with source
  and date - never from training memory.
- Behavioral claims anchor-bound; citations per
  code-comment-line-citations-drift (symbol anchors preferred).
- ADR slots complete: decision, rationale, rejected alternatives EACH
  with its steelman, triggers created (named for ROADMAP mirroring),
  interface/wire-format notes (artifact names ARE an interface).
- D-numbering: D75 upward (Plan 7.5's parallel design owns D65-D74).
- Proposed safeguards stay until built and measured redundant.

## Output structure (per the plan-7 template)

Context; decision log D75+; design sections; workflow/config sketches
(illustrative, verbatim-ready where cheap); test/verification plan (the
workflow_dispatch rehearsal is the plan's acceptance test - specify its
checklist); triggers; open items - which must contain NO unresolved fork
beyond the explicitly marked owner-pending CLI-distribution assumption.

Constraints: read-only except your one output file; no git; never call
EnterWorktree/ExitWorktree or any session-relocation tool; absolute
paths; anything you run, run foreground; network reads (docs pages,
registries) are expected and fine.

Final message: at most 3 lines + the document path + any NEEDS_CONTEXT.
