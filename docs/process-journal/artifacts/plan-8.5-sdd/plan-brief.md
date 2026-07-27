# Brief: author the Plan 8.5 execution plan (macOS packaging fixes)

You are authoring an execution plan. You are not implementing it and not
designing it - both design questions are already ruled by the project's
owner and are quoted below as binding. Your output is one plan document that
a fresh implementer can execute task by task without inventing anything.

Repo: `/home/senol/Git/Muxsmith` (public, Rust core + CLI + Tauri 2/Vue 3
GUI, MIT, pre-1.0). Work on `master` in the main worktree, absolute paths.

## Where this comes from

Plan 8 built the packaging and release pipeline and closed with a green
rehearsal: a `workflow_dispatch` run that builds all four native legs and
assembles a draft GitHub release. Every machine-checkable acceptance item
passed. Two items were reserved for the owner on real hardware - install the
artifacts and inspect the draft - and that walk-through, the first human
execution of the documented install path, found three defects no machine
check could see. Two are 1.0 blockers.

Read `docs/ROADMAP.md`, section **"Plan 8.5: macOS packaging fixes"** for the
kickoff rulings, and the three finding entries under **"Pre-1.0 release
gates"** for the measurements behind them. Those entries are your
requirements; this brief adds the shape, not new content.

## The three items

**1. The macOS app does not launch at all.** Gatekeeper reports it as
damaged. Established: both Mach-O binaries in `Contents/MacOS` carry the
arm64 linker's embedded ad-hoc signature, the bundle has no
`_CodeSignature/CodeResources` seal, and `bundle.macOS` configures no
`signingIdentity`. Confirmed on the owner's Mac: removing the quarantine
attribute makes the app launch, so the app is sound and the packaging state
is the defect.

**Owner ruling: ad-hoc signing.** `bundle.macOS.signingIdentity` takes
Tauri's documented pseudo-identity `"-"`. Verify that mechanism at the
vendor's current documentation yourself before writing the task - I read it
too, and a plan should not rest on a controller's reading.

Two things the plan must carry, not leave implicit:

- **The acceptance criterion is an observable dialog, not a config diff.** A
  freshly built, quarantined bundle must produce the "unidentified
  developer" prompt where it now produces "damaged", so that the flow
  `docs/INSTALL.md` documents is the flow that actually occurs. That
  observation needs macOS hardware and is therefore an OWNER step - plan it
  as one, explicitly, the way Plan 8 planned R8. Do not invent a machine
  substitute and do not let a task claim the fix is verified without it. A
  bundle-side check that CAN run in CI or locally (does the built bundle now
  contain `_CodeSignature/CodeResources`) is worth having as the machine
  half; name it as the machine half, never as the acceptance.
- **The S22 ruling's wording changes in the same change.** That ruling says
  the 1.0 artifacts are unsigned on all three systems. Ad-hoc signing does
  not reopen it in substance - no Apple account, no certificate, no
  notarization - but it makes that sentence untrue as written. Find every
  site that states it (design documents, ROADMAP, INSTALL.md, the release
  collateral - sweep, and name the surface you swept) and correct them in
  the same package. A frozen transcription target is not rewritten; it gets
  a supersession note, per the rule this project applied twice at the last
  two plan closes.

**2. The dmg's pre-mount license must go.** The owner: mounting a dmg and
being met with a license dialog is odd under MIT. Preferred route: drop it
for macOS only, keeping the Windows license dialog, which the same
walk-through confirmed renders correctly.

**The owner's tiebreaker is binding and pre-decided, so no implementer has
to weigh it:** if dropping it for macOS while keeping it on Windows turns
out to need contortions, do NOT build the contortion - fix the rendering on
the macOS side instead. KISS decides, not completeness.

What is established: `bundle.licenseFile` is `../LICENSE`, global, with no
per-platform variant anywhere in the config schema (checked at the vendor
reference: no DmgConfig/WixConfig/NsisConfig/Deb/Rpm/AppImage section
carries a license property). The documented lever is a platform-specific
config file that overrides the bundle section - the same overlay mechanism
this project already uses for the CLI sidecar under D82.

**The plan's first task on this item is therefore an EXPERIMENT, and its
result picks the branch**: can an overlay config CLEAR an inherited key, or
only set one? Determine it empirically. If it cannot be determined without
macOS hardware, say so in the plan and route the decision to the owner
rather than guessing - but check first whether the config merge alone can be
observed on this machine, since merging is a CLI concern and only the
bundling needs macOS. Write the plan so BOTH branches are executable
without a second planning round: the clear-it branch and the fix-the-
rendering branch, each with its own task and acceptance.

For the fix-the-rendering branch, one constraint that is not negotiable: the
LICENSE text is a legal document. Its content, including the spelling of the
copyright holder's name, does not change. Only its encoding or its
presentation may.

**3. The release body's OS links break into three paragraphs.** Confirmed on
the rendered draft. `.github/release/draft-body.md` lines 2-4 begin with a
`|` continuation that GitHub renders as separate blocks. Join them onto one
line. The same treatment covers the two other wrapped regions in that file -
check them, do not assume.

## Constraints the plan inherits

- **The design and spec are ground truth**, with the v1 spec authoritative
  on conflict. The plan-8 design document
  (`docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`,
  D75-D90 plus amendments A1-A3) governs the packaging surface this plan
  touches; where this package contradicts a D-number, the plan says so
  explicitly and carries the amendment as a task rather than diverging
  silently.
- **The four house-knowledge files** (`docs/product-boundaries.yaml`,
  `docs/conventions.yaml`, `docs/process-conventions.yaml`,
  `docs/decision-ledger.yaml`) are ground truth alongside them. Cite entries
  by id; re-verify any `:line` you attach.
- **Every fork must be closed.** No task, verdict or fix dispatch may carry a
  design-latitude clause, in either form: an explicit permission, or the
  commoner one, an omission - an unenumerated set in a normative position. A
  fork discovered on code contact returns as NEEDS_CONTEXT with a decision
  memo and is routed by the controller, never resolved at the keyboard.
- **Gate**: ten parts per BUILDING.md, foreground, no subsets, before any
  push and after every merge.
- **Pins**: no new runtime or product dependency; any new GitHub Action
  SHA-pinned with a version comment.
- **SI-4 (restate it in every dispatch that expects a commit):** commits and
  pushes on this repo are standing-authorized by the owner; agent commits are
  deliberately unsigned (`git -c commit.gpgsign=false`) with the repo
  trailer; stage explicitly, never `git add -A`; every push is logged in
  `gh-log.md`. A subagent inherits a global never-commit default and cannot
  see this grant unless the dispatch carries it.
- **No task creates a tag, publishes or deletes a release.** The rehearsal
  runs by `workflow_dispatch`; its draft is deleted by the owner.
- **Model tiers** per `proc-03-model-assignment`, named per task in the plan,
  with the controller setting the parameter explicitly at dispatch.
- **Verification steps whose expected result is an absence are
  fire-verified**: break it, watch the check fire, restore with alias-proof
  non-interactive forms and verify the restoration against a backup.

## Shape

Small package. Do not inflate it: the owner asked for KISS in the ruling
itself, and a plan that turns three defects into a programme would be
answering a different request. Give it the smallest task cut that keeps each
task independently reviewable, a dependency graph honest about what can run
in parallel, an explicit statement of which steps need macOS hardware and
therefore the owner, and a plan-close section listing the close actions
(including the rehearsal re-run that proves the pipeline still assembles
after the config change).

Write the plan to
`docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md`,
following the structure of the two plans already in that directory. Commit
it (unsigned, explicit staging, trailer). Do not push.

Report anything in this brief you found to be wrong against the tree. Five
briefs in the last session carried a defect each, and every one was caught
by the agent receiving it rather than by me.
