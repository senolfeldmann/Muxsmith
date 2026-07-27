# Whole-branch review brief: Plan 8 (packaging / release pipeline)

You are the independent whole-branch reviewer closing Plan 8. You did not
write any of this code and you grade it against its contracts, not against
what its authors reported. Every task in this plan already passed an
independent task review; your job is the whole those reviews could not see -
cross-task integration, the union's consistency with spec/design/tree, and
the disposition of everything the run deliberately deferred to this moment.

Repo: `/home/senol/Git/Muxsmith` (public, master tracks origin/master).
Work on master in the main worktree. Use absolute paths.

## What Plan 8 is (one paragraph, so you need no other context)

Muxsmith is a rule-based bulk MKV muxing tool (Rust core + CLI + Tauri 2/Vue 3
GUI, MIT, pre-1.0). Plan 8 built the packaging and release pipeline: a new
`.github/workflows/release.yml` that on a `v*` tag (and on
`workflow_dispatch` for rehearsals) runs a guard job, four native bundle legs
(Windows x64 + arm64 msi, macOS arm64 dmg, Linux x64 deb/rpm/AppImage plus a
hand-packed portable tar.gz) and an assemble job that attaches the eight
artifacts plus a `SHA256SUMS` file to a **draft** GitHub release the owner
publishes manually. Around it: a version-sync guard script (one version
source, the Cargo workspace), the Tauri bundle-config rewrite with a CLI
sidecar overlay, install/runtime documentation and release collateral, and a
controller-ruled rider (the `ledger-lint` CI job plus a per-entry
duplicate-key check in the linter). The plan closed its execution with a
`workflow_dispatch` REHEARSAL that executed the design's R1-R10 acceptance
checklist; that rehearsal is green.

## Ground truth, in priority order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - the v1 spec,
   authoritative above everything below on conflict.
2. `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` -
   the plan-8 design (ADRs D75-D90), owner-approved 2026-07-23 after a
   four-eyes fix loop, **plus mid-run amendment A1** (commit `d21a19f`,
   four-eyes delta APPROVED): the contract for this plan. Its section 8 is
   the R1-R10 acceptance checklist and the G1-G5 fire-tests; its section 11
   enumerates what the implementer must not decide.
3. `docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md` - the
   execution plan (tasks 1-6, stream cut, model tiers, coverage map).
4. The house-knowledge files, ground truth alongside the above:
   `docs/product-boundaries.yaml` (product scope), `docs/conventions.yaml`
   (code style), `docs/process-conventions.yaml` (method/CI), and the
   Tier-1 `docs/decision-ledger.yaml`. Cite entries by `id`; re-verify any
   `:line` you attach to a citation.

## The diff you are grading

`.superpowers/sdd/plan-8/whole-branch-8.diff` - `git diff aec4cef..7302e1b`
restricted to the plan-8 pathset. **The range is pinned to the SHA `7302e1b`,
not to `HEAD`**: master advances during your review with process-artifact
commits (the parallel plan-7.5 close: its SDD salvage, journal entries,
house-knowledge YAML edits). None of those touches your pathset. Use
`7302e1b` in every range you compute; if you find a plan-8 pathset file
changed after it, that is a finding.

```
.github/release/ .github/workflows/ci.yml .github/workflows/release.yml
.gitignore BUILDING.md README.md docs/INSTALL.md packaging/
scripts/check-version-sync.sh scripts/ledger-lint.py
src-tauri/tauri.conf.json src-tauri/tauri.bundle.conf.json src-tauri/wix/
src-tauri/src/lib.rs crates/muxsmith-core/tests/joblog.rs
```

**Regenerate that diff yourself and confirm it is byte-identical** before
trusting the file. The scoping is path-based because a second plan (7.5,
track-rule add/remove in the profile editor) executed in parallel on the same
master; its pathset (`src/views/EditorView.vue`, `e2e/editor-rule-*.spec.ts`,
`e2e/help-mode.spec.ts`, `help/**`, the 7.5 design and the v1 spec's 8.2/5.2
amendments) is disjoint and is NOT yours. **Verify that disjointness claim
rather than accepting it**: run `git log aec4cef..7302e1b --name-only`, and if
any product file in range falls outside both pathsets, review it here and say
so.

Two files in your pathset are *not* plan-8 tasks but landed inside the plan-8
execution window as controller-dispatched fixes for latent pre-session
defects, each already independently reviewed once:

- `crates/muxsmith-core/tests/joblog.rs` (`c06b8dd`) - a calendar-bomb test
  fixture (hardcoded 2026-07-10 stamps aged past the 14-day run-log prune,
  turning the nine-part gate red on an unrelated merge). Verdict:
  `joblog-datebomb-fix-verdict.md`. Also already re-reviewed by the plan-7.5
  whole-branch reviewer. Only report a NEW finding here.
- `src-tauri/src/lib.rs` (`f4f932e`) - a Windows-only clippy red
  (unused-import behind a missing `cfg(unix)` gate) that had been failing CI
  on Windows unobserved for five runs. Verdict:
  `windows-clippy-fix-verdict.md`.

## Evidence that already exists - read it, do not re-run it

All under `.superpowers/sdd/plan-8/`:

- `progress.md` - the run's tracker. It carries every deferred item verbatim;
  the adjudication questions below are drawn from it.
- `task-{1..6}-brief.md`, `task-{1..5}-report.md`, `task-{1..5}-verdict.md`.
- `task-6-report.md` - the rehearsal. Two parts: the first attempt (BLOCKED,
  both Windows legs died in WiX `light.exe`) and, from the `# RE-RUN` heading
  on, the full green rehearsal (runs `30272619000` and `30273529210`, R1-R10).
- `wix-fix-report.md` / `wix-fix-verdict.md` - the Windows blocker's
  diagnosis and fix (`07c0255`: a WiX localization file with code page 1254,
  so the publisher string keeps its correct orthography). Note the recorded
  CORRECTION to that report in `progress.md`.
- `gate-logs/` (11 files) - the nine-part gate runs after each merge and the
  CI watch logs.

**Do not dispatch any GitHub workflow, do not create, edit, publish, delete
or un-draft any release, do not tag, do not push, do not commit.** The draft
release `rehearsal-30273529210` is deliberately preserved: it is the owner's
input for acceptance item R8 and he deletes it at the plan close (R10). Any
`gh` command you run must be read-only (`gh run view`, `gh run list`,
`gh release view`), and every one of them gets an appended entry in
`gh-log.md` (git-ignored) with the command, its effect, and the manual web-UI
equivalent.

## Review dimensions

Run all of these. For each finding give: severity (**Blocker** = the plan
cannot close / **Important** = fix before close / **Minor** = record and
route), the file and the evidence, and what specifically to change. You
propose text; you do not edit product files.

1. **Cross-task integration.** The five wave-1 tasks were built in four
   parallel worktrees and merged sequentially. Do the parts agree on master
   where they touch: the version source (`Cargo.toml` workspace version ->
   `tauri.conf.json` inheritance -> guard script -> the release job's
   artifact names -> the documented names in `docs/INSTALL.md` and the
   tar.gz README -> what the rehearsal actually produced)? Does every
   consumer contract named in the plan's dependency graph hold on the merged
   state rather than per-branch?
2. **Design/spec/tree three-way.** Walk the design's ADRs D75-D90 plus
   amendment A1 and confirm each is implemented as decided, or that the
   divergence is recorded. Same for the v1 spec's section 10 (packaging) -
   flag anything the shipped pipeline contradicts.
3. **The acceptance set itself.** The rehearsal reports R1-R10 green at their
   named emitters. Sample-verify: pick the observables that are checkable
   from the tree or from read-only run/release queries and confirm the report
   is telling the truth (`design-acceptance-observables-have-producers`).
   State explicitly what is NOT verifiable outside a real Windows/macOS
   install, rather than implying coverage you do not have.
4. **Pinning and supply chain.** Every action in `release.yml` and in the
   ci.yml rider job SHA-pinned with a version comment, and the pin set
   exactly the enumerated one (`ci-10-pin-everything`, design section 1.4;
   no tauri-action, no softprops, no cache action on release legs, no
   mise-action). Verify the SHAs resolve to the claimed versions.
5. **ci.yml additivity.** The rider was ruled strictly additive - one new
   self-contained job, no existing line changed. Verify it on the diff.
6. **Local runnability.** Run what can be run here, foreground:
   `bash scripts/check-version-sync.sh` (expect exit 0),
   `python3 scripts/ledger-lint.py` (expect exit 0), and a YAML parse of
   `release.yml` and `ci.yml`. For any check whose passing result is an
   ABSENCE, fire-verify it once (break it deliberately, watch it fire,
   restore) rather than trusting the empty result -
   `proc-verification-step-must-be-falsifiable`. Restore with alias-proof
   non-interactive forms (`command cp -f`, `command rm -f`) and verify the
   restoration against a pre-mutation backup -
   `proc-noninteractive-file-ops-in-agents`.
7. **`house` dimension.** Flag every deviation from a recorded convention in
   the four house-knowledge files, by entry id.
8. **Surviving design latitude.** The plan and design were written under an
   absolute latitude ban (`proc-latitude-clause-boundary`). Check the shipped
   artifacts and the plan/design text for latitude in BOTH forms: an explicit
   permission ("the implementer may choose", "either approach works") and the
   commoner form, an **omission** - a mandated set that is never enumerated,
   a list ending in "...", a "one per X" with no X list. The test is not
   "does a permission appear" but "must the implementer invent something it
   is not allowed to invent".
9. **The no-work-needed check** (`proc-no-work-needed-check`). Wherever a
   passage in the design, plan, a report or a verdict concludes that a guard,
   an enumeration or a check is unnecessary ("so we need no X", "X cannot
   happen here", "the work already exists"), **run the premise that makes it
   unnecessary**. Do not weigh it.
10. **Documentation truthfulness.** `docs/INSTALL.md`, `BUILDING.md`,
    `README.md`, the tar.gz README and the draft-body template describe a
    pipeline that now exists. Check each factual claim against the tree and
    the rehearsal evidence. Anchor text checks on CONTENT, not on line
    positions (`proc-wrapped-prose-quote-grep`), and remember that a firing
    positive control proves your PATTERN, never that your SEARCH SURFACE was
    complete (`proc-sweep-surface-completeness`).

Tooling trap, real in this environment: the interactive shell has `cp`
aliased to `cp -i` and `grep` bound to a function that respects
`.gitignore` - so a plain `grep -r` silently skips `.superpowers/` (git-
ignored) and returns a false empty. Use `command grep` when sweeping there.

## Adjudication questions - one explicit verdict per item, all thirteen

These are the items the run deliberately deferred to this review. For each:
state what it actually is (verify it yourself; several are borrowed claims
from reports), then rule **fix-before-close** (goes into the whole-branch fix
wave) / **close-batch one-liner** (a doc or bookkeeping line the controller
routes at the plan close) / **route to owner** / **record only** / **no
change needed**. Phrased neutrally on purpose - none is pre-rated, and
"no change needed" is a real answer for any of them. Where you rule a text
change, give the exact replacement text so no implementer has to invent it.

1. **BUILDING.md, two stale sites.** (a) Around lines 65-68, the claim that
   `tauri build` is "out of scope for local development"; the run recorded
   that only that half goes wrong once the plan's BUILDING.md subsection 4.4
   and `release.yml` land, while the neighbouring "not part of the CI gate"
   stays true (release.yml is not the gate). (b) Around lines 92-96, the
   CI-jobs enumeration names `cargo deny check` as THE independent job, which
   the rider's `ledger-lint` job makes stale. Verify both by content, quote
   the current text, and give the replacement.
2. **A tenth gate part: cross-target Windows clippy.** The owner APPROVED
   adding it after the Windows-only clippy red went unobserved for five CI
   runs; the run recorded that the BUILDING.md change lands in this fix wave.
   Determine and state precisely: the exact command, whether it needs a
   prerequisite (`rustup target add x86_64-pc-windows-msvc`) and whether that
   prerequisite belongs in the documented gate, whether it is actually
   runnable on a Linux dev machine (try it - a cross-target *clippy* is not a
   cross-target *build*), and the exact BUILDING.md text. If it turns out not
   to be runnable as assumed, say so plainly - that is a finding, not a
   failure.
3. **`scripts/ledger-lint.py`: `yaml.ReaderError` escapes the parse `try` as
   a raw traceback** (pre-existing on master, ledgered as
   `inline-wrapper-keeps-try-scope`). Reproduce it. Then rule: in scope for
   this fix wave because the plan touched the file, or deliberately left.
4. **R1 observable wording.** The design's R1 and the emitter disagree: the
   gate-green echo names a SHA where the observable text says run id (already
   ledgered). Locate both sites, decide which one is wrong, and rule whether
   the correction is a doc one-liner or a `release.yml` change - weighing
   that a workflow change costs another GitHub round-trip to verify.
5. **R6 deb-payload path form.** dpkg 1.23.7 emits payload paths as
   `usr/bin/...` without a leading `./`; a design/doc site states otherwise.
   Find every affected site and give the one-liner.
6. **D86's publisher fallback, superseded on the merits.** D86 provided that
   if the publisher string's `Ş` failed to render (acceptance item R8), the
   `publisher` field alone would fall back to ASCII "Senol Feldmann". The WiX
   diagnosis established that the ASCII fallback would NOT have fixed the
   build (three sinks carry the character, the LICENSE text among them) and
   that the code page is the actual instrument. **Verify that claim from the
   wix-fix evidence yourself** - it is the load-bearing one here - then
   propose the ADR bookkeeping line that records the supersession.
7. **Stale `"language": ["en-US"]` sites.** The WiX fix introduced
   `src-tauri/wix/locale-en-US.wxl`, which the pre-fix text does not know.
   Recorded candidate sites: design around :958/:1012/:1511/:2007, plan
   around :248/:262, and plan :718 whose trigger-7 premise is stale. Line
   numbers drift - locate by content, report what you actually find (more or
   fewer sites is a finding), and give each replacement. The change is
   owner-authorized.
8. **Bundler version citation sweep.** The pinned tauri bundler is 2.9.4;
   2.11.4 is the `@tauri-apps/cli` version. Establish both numbers from the
   lockfiles yourself, then sweep every citation site and list the
   miscitations with corrections. Name your search surface explicitly.
9. **The `ansicpg1252` cosmetic residual.** Upstream `tauri-bundler`
   hardcodes `ansicpg1252` in the license RTF header, so how the `Ş` renders
   in the Windows installer's license dialog on a real install is unverified.
   Rule: does this owe a tracker entry (a ROADMAP trigger with a nameable
   observable event, or a v1.x candidate), or is the record in the wix-fix
   report sufficient? Note that "record it in a frozen report" is history,
   not a backlog.
10. **Task 1 observation i3: no acceptance observable reads a
    BUNDLER-produced version.** R9 reads the CLI's own `--version` (clap).
    Rule whether that is a real gap in the acceptance set, whether R6's
    package-metadata inspection already covers it, and if it is a gap,
    whether it owes an acceptance item or a trigger.
11. **Joblog comment note.** Four sites pass stale timestamps as the `create`
    function's `run_id` ARGUMENT, which is safe only via an implicit
    prune-before-leaf invariant (task report concern 3). Rule: comment note
    owed now, or recorded for the next touch - and if recorded, in which
    concrete vehicle.
12. **D75 cross-reference defect** (pre-existing LOW from the task-3 review):
    a "section 1" cross-ref that should point at section 0:51. Verify and
    give the corrected text.
13. **The nine wording items routed to the owner's rendered-surface pass**
    (enumerated in `task-3-verdict.md`). Confirm all nine are still open and
    still correctly owner-routed rather than silently landed, and restate
    them compactly enough that the controller can put the list to the owner
    without opening the verdict file.

## Output contract

Write your verdict to `.superpowers/sdd/plan-8/whole-branch-verdict.md`.
That file and scratchpad files are the ONLY writes you make: no git
operations, no product-file edits, no `.superpowers` edits other than that
file (`gh-log.md` appends excepted, as required above).

Structure:

- `# Whole-branch verdict: Plan 8 (packaging / release pipeline), range
  aec4cef..7302e1b path-scoped`
- **`## VERDICT: READY`** or **`## VERDICT: NEEDS FIXES`** - READY means
  nothing blocks the plan close.
- `## 0. Scoping verification` - the disjointness check and the diff
  regeneration, done first.
- Then your dimension findings, numbered, severity-tagged.
- `## Adjudication` - all thirteen questions, each with its explicit verdict
  and, where a change is ruled, the exact replacement text.
- `## Fix wave` - the consolidated list of what must change before the close,
  in the order an implementer would apply it, so the controller can dispatch
  it as one task. If nothing must change, say so in those words.
- `## HARVEST` - patterns you observed that the house should record: dominant
  practices worth a convention, repeated rejections, and anything you judge
  the process got wrong. Include, explicitly, any place where a brief or a
  convention boundary FORCED a stop that you judge it should have covered -
  that is a wanted finding, not second-guessing.

Rules that bind you as they bind every agent here: quote only what you have
opened (a quotation is a claim about wording), and measure every number you
report rather than estimating it. If you find a fork this brief left open,
return it as a question in the verdict - do not resolve it at the keyboard.
No session-relocation tools (no `EnterWorktree`/`ExitWorktree` or
equivalent). Every command run in the foreground.
