# Whole-branch review brief: Plan 8.5 (macOS packaging fixes)

You are the independent whole-branch reviewer closing Plan 8.5. You wrote none
of it. Every task already passed its own independent review; your job is the
whole those reviews could not see.

Repo: `/home/senol/Git/Muxsmith` (public, MIT, pre-1.0). Work on master in the
main worktree, absolute paths.

## What this package is, in one paragraph

Muxsmith is a rule-based bulk MKV muxing tool (Rust core + CLI + Tauri 2/Vue 3
GUI). Plan 8 shipped its packaging and release pipeline and passed every
machine-checkable acceptance item. Two items were reserved for the governing
human on real hardware, and that walk-through - the first human execution of
the documented install path - found three defects, two of them 1.0 blockers:
the macOS app did not launch at all (Gatekeeper called it damaged, because the
bundle carried no code-signature seal even though its binaries carry the arm64
linker's ad-hoc signature), the dmg showed a pre-mount license dialog whose
text garbled at the publisher's non-ASCII character, and the release body's
three OS links rendered as three paragraphs. Plan 8.5 fixes exactly those
three, under two owner rulings: ad-hoc sign the bundle (one config line, no
Apple account, which is NOT the code signing deferred to 1.x), and remove the
dmg licence entirely rather than repair its encoding - with a pre-decided
tiebreaker that if removing it for macOS while keeping Windows needed
contortions, the rendering should be fixed instead. It did not: a platform
config can clear an inherited key, established at the pinned CLI's source and
reproduced empirically three times by three different agents.

## Range and ground truth

Range: `f627105..HEAD`. Pin it to the SHA you measure at start and say which.
The task commits are `9460daf`, `5060ef5` (Task 1 + its fix round), `50e08cd`
(Task 2), `87c1dee` (Task 3), plus the plan amendment `29ef17b` and
controller-side ROADMAP/ledger commits. Task 4 wrote no repo file.

Ground truth, in priority order: the v1 spec
`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`; the packaging
design `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md`
(D75-D90, amendments A1-A5); the plan
`docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md`; the
ROADMAP's Plan 8.5 anchor and the three finding entries under "Pre-1.0 release
gates"; and the four `docs/*.yaml` house-knowledge files, cited by entry id.

Prior evidence, to read rather than re-run: `.superpowers/sdd/plan-8.5/`
carries `progress.md` (the tracker, which records every routing and ruling),
the four task reports and verdicts, the two delta verdicts, the plan review
and its delta, and the plan-amendment verdict.

## What is already settled - do not re-litigate

- The owner ACCEPTED all three rulings on real Apple-Silicon hardware and on
  the rendered draft (2026-07-28): the installer shows the
  unidentified-developer path instead of "damaged", the dmg mounts with no
  licence dialog, the rendered body carries the three links on one line, and
  `docs/INSTALL.md`'s macOS section matches the flow he walked. Only O4 - his
  deletion of the rehearsal draft - remains.
- Task 4's reviewer established that BOTH the fixed and the defective GUI
  binaries carry a valid `LC_CODE_SIGNATURE` blob, because the arm64 linker
  always ad-hoc signs the binary, so the bundle seal is the only discriminator
  at that layer. That is settled evidence, not an open question.

## Dimensions

1. **Cross-task integration.** Four tasks, serial, on one tree. Do the parts
   agree on master where they touch - the config, the two documents Task 1 and
   Task 3 both edited, the design's amendment log that Tasks 1 and 2 both
   appended to? Does the shipped configuration actually produce what the
   documentation now promises?
2. **The union against its contracts.** Walk the three ROADMAP finding entries
   and the plan's three rulings; name what implements each. A requirement with
   no implementation is a finding.
3. **Surviving latitude, in both forms** - an explicit permission, and the
   commoner one, an unenumerated set in a normative position.
4. **The no-work-needed check.** Wherever any artifact concludes a guard,
   enumeration or check is unnecessary, run the premise that makes it so. Do
   not weigh it.
5. **House conformance** by entry id, including the rule that a frozen
   transcription target is never rewritten but gets a supersession note - this
   package applied it in several places.
6. **The deferred list, triaged.** The tracker records items the run routed
   rather than fixed. For each, rule: fix-before-close, close-batch line,
   route to the owner, record only, or no change needed. Two are already
   argued in Task 4's verdict (the ahead-of-origin dispatch precondition, and
   the log-line count that was converted from a per-leg presence check and
   asserted from intent) - rule on the recommended handling, including whether
   a retired plan document is worth amending at all or whether the durable
   home is the house-knowledge files.
7. **Anything the package broke** that no single task review could see.

## Output

Write `.superpowers/sdd/plan-8.5/whole-branch-verdict.md`:
`## VERDICT: READY` or `## VERDICT: NEEDS FIXES`, a scoping verification done
first, findings by severity with evidence, the deferred-item triage, a
consolidated fix wave if anything must change before the close (say so in
those words if nothing must), and a `## HARVEST` section - patterns worth
recording, and any place a brief or a convention boundary forced a stop you
judge it should have covered.

That file plus scratchpad files are your only writes. No git write operations,
no product edits, no session-relocation tools, no workflow dispatch, and do
not touch the draft release - it is the owner's pending deletion. Read-only
`gh` queries are fine and each gets a `gh-log.md` entry. Foreground commands.
`command grep` or `git grep` for the git-ignored `.superpowers/` tree. Build
any harness at a path no earlier agent could have written. Quote only what you
open; measure every number.
