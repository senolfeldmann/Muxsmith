# Plan 8 design review brief (round 1)

Independent reviewer, fresh eyes; you did not author the design. Artifact
under review: `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(commit d7e79ba, D75-D90). Ground truth: the controller brief
`.superpowers/sdd/plan-8/design-brief.md` PLUS its superseding controller
addendum (the CLI-distribution owner ruling, recorded in docs/ROADMAP.md
Plan-8 kickoff block "CLI-distribution ruling"), the spec, the Tier-2
house files, the ACTUAL TREE, and for external claims the LIVE
authoritative sources. The design's claims are verified against reality
and by your own lookups, never believed.

## Dimensions

1. **Brief + ruling compliance**: every "must RESOLVE" item resolved;
   the seven kickoff rulings AND the second-round CLI ruling (installers
   bundle the CLI, no add-to-PATH anywhere, Linux one-package, tar.gz
   both binaries, brew cask NOT proposed) appear with unmodified
   substance. The rejected alternatives D82 must steelman: GUI-only
   installers and the PATH-option variant.
2. **Registry-verification audit**: the design claims every runner
   label, action SHA, and schema fact was live-verified 2026-07-22 with
   citations. Re-verify the LOAD-BEARING ones yourself (live lookup, not
   memory): the windows-arm64 msi support claim (bundler source at the
   pinned tauri-cli version; this claim carries the whole arm64 leg),
   the four runner-image labels, every action SHA the design pins (does
   the SHA exist and match the claimed tag?), the Cargo-inheritance
   version claim of D87 (does the pinned Tauri CLI actually support
   tauri.conf.json without a version key, reading from Cargo.toml?).
   Spot-check the remainder. A citation that does not say what the
   design says it says is a finding.
3. **Latitude scan, BOTH forms**: explicit permissions and omission
   latitude (unenumerated sets in normative positions: leg lists,
   artifact lists, checklist items ending open, "etc."). The
   plan-author test: must anyone downstream invent something they are
   not licensed to invent?
4. **Workflow architecture soundness (D83/D79/D85)**: the release.yml /
   ci.yml split against the ACTUAL ci.yml (does the existing `v*` tag
   trigger on ci.yml conflict or compose with the new workflow - both
   triggering on the same tag push; is the same-SHA gate-green guard
   implementable as designed?); the workflow_dispatch rehearsal path
   shares job definitions with the tag path (reuse, not a forked copy)
   as the brief demands; permissions blocks least-privilege per house
   precedent; no mise on release legs per the brief, with the setup it
   proposes instead actually pinned.
5. **Bundle config (D86/D82/D88)**: the tauri.conf.json changes against
   the current schema (verify the overlay/build-flavor mechanism the
   design uses for externalBin exists as described); externalBin
   mechanics (target-triple naming, where the CLI lands per OS -
   install dir on Windows, Contents/MacOS on macOS - byte-check against
   the cited Tauri docs); deb/rpm Recommends mechanism as designed;
   tar.gz packing step contents complete (both binaries, LICENSE,
   README with webkitgtk + mkvtoolnix requirements).
6. **Version-sync guard (D87)**: the guard script design - does it make
   the failure impossible as the brief demands (a tag whose artifacts
   self-report a different version)? Walk the failure paths: tag !=
   Cargo version, package.json drift, tauri.conf drift. The house
   drift-check precedent (committed-generated-plus-drift-check) is the
   pattern to compare against.
7. **SI-3 parity + licensing**: parity claims spot-checked at
   ~/Downloads/mkvtoolnix; no literal text adoption; the Linux
   one-package divergence recorded with the split's steelman.
8. **No-work-needed check**: every passage concluding something is
   unnecessary (e.g. "no substitute artifact needed", "ci.yml not
   modified", cache omission on release legs) - run the premise.
9. **ADR quality**: all sixteen entries complete (decision, rationale,
   steelmanned rejected alternatives, triggers named for ROADMAP
   mirroring, interface notes - artifact names ARE an interface);
   numbering D75-D90, no collision with D65-D74.
10. **Acceptance test**: the rehearsal checklist is the plan's
    acceptance test - is it executable as written, does every check
    have a fire-verified guard as claimed (re-run the claim where
    cheap), and does it cover every leg and the draft-release path?
11. **INSTALL.md scope (D75)**: the per-OS install-hurdle outline
    complete (SmartScreen, Gatekeeper right-click/quarantine, Linux
    none) and consistent with the unsigned ruling; placement choice
    (new docs/INSTALL.md vs README) argued with steelman.

## Output

Write `.superpowers/sdd/plan-8/design-review-round-1.md`: verdict
APPROVED or NEEDS FIXES; findings by severity, each with location and
what to change; a HARVEST section (dominant patterns, repeated
rejections, over-restriction flags). Final message: verdict word + at
most three lines + the file path.

## Constraints

Read-only on the tree except your verdict file; no git writes; never
call EnterWorktree/ExitWorktree or any session-relocation tool; absolute
paths; anything you run, run foreground; live web/registry lookups are
expected and required for dimension 2.
