<!-- Snapshot of HANDOFF.md at the session-28 close, final state: Plan 10 closed, the Linux release base on ubuntu-24.04 and proven by a draft build the owner ran on Fedora, Renovate running. The HANDOFF itself is git-ignored and superseded in place; SI-5 requires this snapshot in the same turn as the rewrite. -->

# Handoff

> Self-contained context transfer. A fresh session should be able to resume from this file alone. Lifecycle and publication rules: SI-5.

**Date:** 2026-07-29 (session 28 close, Plan 10 closed, Linux release base moved)
**Active agent:** Peter (~/agents/peter)
**Working directory / repo:** ~/Git/Muxsmith (github.com/senolfeldmann/Muxsmith, public, master tracks origin/master)

## STANDING INSTRUCTIONS - reproduce this ENTIRE section verbatim in every future HANDOFF, including this reproduction rule itself

### SI-1. Process package

This project runs the **software-dev-process** package as its standing
method: the skill (auto-triggers at controller moments) plus the doctrine
at `~/agents/peter/prompts/software-dev-process.md`. The doctrine is the
single source for the execution spine (Superpowers workflow, SDD never
inline in the controller, parallel worktree streams), write-at-creation
rules, the gates (session start/close, plan close, milestone, pre-execution),
controller duties, and - since session 10 - **house-knowledge management
(doctrine section 7)**: the Tier-2 convention files + Tier-1 ledger, the
source x nature promotion matrix, the escalation flow. Do NOT restate
doctrine content here - read it. Şenol's ruling 2026-07-11: this binding is
project-scoped; new projects get an adopt-or-not question at kickoff.

### SI-2. Process journal

Journal duty per `docs/process-journal/PROMPT.md` (git-tracked, read it each
time - it mandates the salvage pass incl. reviewer-verdict files and a
HANDOFF snapshot at EVERY plan close). Entries at every plan completion and
session close.

### SI-3. mkvtoolnix parity audit in all planning and decision-making

When authoring plans or ADRs, or resolving ANY behavioral question, compare
against mkvtoolnix-gui / mkvmerge wherever meaningful. Load-bearing
distinction: mkvtoolnix is INTERACTIVE (pre-fills guesses the user reviews),
Muxsmith is DECLARATIVE BATCH (the profile is the spec). Muxing semantics and
output are parity targets; input-time convenience guesses are NOT
(docs/IDEAS.md 1-2). Method: classify match / justified divergence / genuine
gap; read the source at ~/Downloads/mkvtoolnix (cite file:line); confirm
mkvmerge behavior by running the binary (v100.0), never from memory; surface
gaps and divergences for Şenol; record divergences in the memo. Licensing
boundary (mkvtoolnix GPL, Muxsmith MIT): behavior, facts and interfaces are
fair game; literal code or text passages are never taken; deliberately
modeled wording is recorded as an explicit ADR decision.

### SI-4. Git commits and pushes are STANDING-authorized for this repo

Şenol's grant (2026-07-09, "persist indefinitely"): commits AND pushes on
~/Git/Muxsmith are authorized standing; never re-request. Agent commits are
deliberately UNSIGNED as policy (a GPG signature is Şenol's authorship claim):
`git -c commit.gpgsign=false` on every agent commit and merge. Trailer per
convention; log every push in gh-log.md (git-ignored). Permission mechanics
are solved: Şenol added the git allow-rules to the agent-side permission
file himself; they match ONLY pure git command shapes (a cd-into-repo plus
git, or git -C). ANY non-git segment chained into the compound voids the
match and the command falls to a permission classifier that sees only the
global never-push rule and denies. Keep git commands pure; do bookkeeping
separately; a denial of a compound is a denial of the SHAPE, not the
action - re-shape and retry before treating the push as blocked (a day's
push was lost to this once; for push specifically the `git -C` shape is the
one that passes - confirmed deliberate 2026-07-15). The agent cannot edit
its own permission file; any rule change is Şenol's edit. Never
`git add -A` (untracked artifacts); stage explicitly. The harness's
security monitor may falsely flag authorized subagent commits (this grant
is invisible to it): verify the commit's content, name the false alarm,
never revert because of the flag alone. **A dispatch that expects a
subagent to commit RESTATES this grant in the dispatch text** - the
subagent inherits a global never-commit default and cannot see a grant
that lives only here (ledger `dispatch-restates-the-standing-commit-grant`).
**Trailer set, owner-ruled 2026-07-28** (Tier-2 `agent-commit-trailer-set`):
exactly one trailer, `Co-Authored-By: Claude <model> <noreply@anthropic.com>`,
no `Claude-Session` line; the model name is canonical with no context-window
suffix; and the string is DERIVED from the dispatch's model parameter, never
written as a literal in a plan or brief. **Two writers in one working tree
share one git INDEX**, so staging your own paths does not isolate them - a
bare `git commit` takes everything staged. Use pathspec-scoped commits
(`git commit -- <paths>`) or give the second writer its own worktree
(`concurrent-writers-need-pathspec-scoped-commits`).

### SI-5. HANDOFF lifecycle: snapshot every state, publication-grade always

HANDOFF.md is git-ignored and superseded in place, so any state not
snapshotted dies with its overwrite. Rule: whenever HANDOFF.md is rewritten
(plan close, session close, mid-session supersede), snapshot the NEW state in
the same turn to `docs/process-journal/artifacts/handoffs/<date>-<label>.md`
and commit it. Because snapshots are committed to the public repo, the HANDOFF
is written publication-grade at ALL times: nothing enters this file that could
not go public - no secrets, no personal or private context, no names or paths
beyond the project's approved-public set.

(SI-1 through SI-5 are carried forward by the reproduction rule in this
section's heading.)

## Objective

Muxsmith v1: rule-based bulk MKV muxing tool (Rust core + CLI + Tauri 2/Vue 3
GUI, MIT, public). **Next milestone: 1.0.** Plans 1 through 10 are closed.
**No further product plan is scheduled.** What stands between here and the tag
is listed under "Next steps", and the first item is not a plan.

## The gate that changes what "done" means

**Owner ruling 2026-07-29, Tier-2 `owner-manual-qa-gates-the-1-0-release`: no
1.0 release is cut before Şenol has personally run a manual QA and bug-hunting
pass on his own hardware.** Its output is first-class scope input in three
shapes he named: real bugs; behaviour he dislikes even where it matches the
spec; and v1.x items he decides belong in 1.0 after all.

Round 1 covered the install paths only - three OSes installed and launched, the
documented steps and SHA commands confirmed, one finding (now documented). **The
product itself is still untested by him**: a real dry-run and run over his own
library, the profile editor including rule add and remove, suggestion apply, the
jobs view during a live batch with a mid-run cancel, run history, the locale
switch, help mode. **Until that pass has run, 1.0 scope is unknown by
construction, and no completeness claim about 1.0 may be made** - however short
the remaining list looks. His timing call: the full pass comes once the next
plan is implemented, which is now.

A build for it exists as a rehearsal draft on `a5b63ba` (`workflow_dispatch` on
release.yml with the draft flag; never a tag, never published). A fresh draft
build off current master would be more useful, since Plan 10 changed
`docs/INSTALL.md` and the README.

## Constraints and conventions

The SIs above; the doctrine (SI-1). The v1 spec is authoritative over designs
and plans on conflict.

- **The gate is what `BUILDING.md` enumerates** - foreground, no subsets, before
  any push. Since Plan 10 Task 1 the file STATES its own total behind a
  `gate-total` marker and `scripts/ledger-lint.py` checks that statement against
  the commands its three `gate-block`-marked blocks enumerate. Anything deriving
  a count still says "per BUILDING.md" and must agree with that file. Two
  boundaries of that check are recorded as ROADMAP triggers: a FOURTH marked
  gate block would be invisible to it, and a command wrapped with a trailing `|`
  or `&&` is not modelled (backslash continuations are, and it refuses on them).
- **Model tiering, owner ruling 2026-07-28**: the top model serves ONE role, the
  plan-close whole-branch review and its delta re-reviews. Everything else -
  design and plan four-eyes rounds, decision documents, task implementers, task
  reviewers, fix dispatches, recon - runs the mid tier; plan-carried
  transcription runs the cheap tier. Every dispatch names its model explicitly.
- **House-knowledge YAML is edited by targeted text replacement only**, never
  through a serializer round-trip, and never by a script anchored on a repeated
  key pair - anchor on the entry's `- id:` line. 545 entries at this close.
  **No task edits these files** - the controller is the single writer. Run
  `ledger-lint` after every batch edit and before every commit.
- **A comment never locates code by line number** (owner ruling 2026-07-29,
  Tier-2 `comments-locate-by-symbol-never-by-line-number`): name the symbol.
  Naming the file stays fine. Plan 10 Task 5 swept the corpus; see the open
  question below about what the ruling's scope covers.
- **A document never cites a line number inside ITSELF** (owner ruling
  2026-07-29, Tier-2 `a-document-never-cites-a-line-number-inside-itself`).
- **Two writers in one tree share one index** (SI-4). This close violated it
  four times and got away with it; the rule is `git commit -- <paths>`, and a
  reviewer's byte-identity proof is per file against blobs, never a clean
  `git status`.
- Subagents never call session-relocation tools; worktrees are plain
  directories.

## Current state (verified)

- **master at the session-28 close head - the commit that added this file's own
  snapshot - and everything is pushed.** Re-derive rather than trusting this
  line: `git log --oneline -1`, `git status`, `git rev-list origin/master..master`. The eleven-part gate
  ran green before every push in this session, each part's exit code captured
  separately. CI is green on the pushed heads; **one run in this session was
  RED** and it is worth knowing about: on the docs-only commit `ad4746d` the
  Windows leg's choco mkvmerge install did not produce the binary at the path it
  asserts, while both neighbouring commits were green on all five jobs and the
  same log carried npm registry retries. Recorded in the ROADMAP as a second,
  distinct flake class, because that step is what makes "3-OS green" mean
  live-binary tests on three of three.
- **Plan 10 is EXECUTED AND CLOSED.** Five serial tasks in one tree, each with a
  fresh implementer and an independent reviewer; one task fix round; a
  whole-branch review on the top tier returning READY_WITH_MINORS; one close fix
  wave whose three findings were all verdicted ADDRESSED by the resumed
  reviewer. Archive: `docs/process-journal/artifacts/plan-10-sdd/` (32 files,
  count verified in the commit and in the index, re-salvaged after the close fix
  wave).
- **What landed:** the gate-count invariant in `BUILDING.md` +
  `scripts/ledger-lint.py`; the two D102 preserved-order producers, selected by a
  four-mutation measurement rather than assumed; `renovate.jsonc`;
  the user-facing documentation pass over `README.md` and `docs/INSTALL.md`; the
  comment line-citation sweep (24 lines across 16 files, 21 comments).
- **Two vulnerability alerts are still OPEN** with their own ruled one-task
  vehicle in the ROADMAP's pre-1.0 gates, unscheduled against Plan 10 on purpose.
  The `cargo deny` / GitHub disagreement under them is still unmeasured, and
  until it is, neither mechanism may be quoted as coverage.
- **House knowledge is at 548 entries**, up from 531 at the session start. One
  entry promoted to Tier 2 on its third occurrence
  (`a-search-whose-terms-come-from-memory-produces-a-false-absence`, whose
  statement now carries the split that promoted it: a measuring expression has
  two enumerations, what it READS and what it MATCHES).
- **Renovate is RUNNING** since 2026-07-29: dependency-dashboard issue #2 exists.
  Its activation trigger fired and was deliberately re-deferred to a sharper
  observable, because the cadence is the 1st to 3rd of the month and no
  dependency PR exists yet to obsolete a `deny.toml` RUSTSEC ignore or carry the
  TypeScript-7 bump.
- **The draft rehearsal build the owner tested is run `30491217194`** on
  `fd78bfc`: four bundle legs plus assemble green, seven artifacts and
  SHA256SUMS, never a tag, never published.

## The Linux release base moved, after the plan close

**Owner ruling 2026-07-29, recorded as Tier-2
`linux-artifacts-carry-the-build-base-glibc-floor`: the Linux release artifacts
build on `ubuntu-24.04`, the test matrix stays on `ubuntu-26.04`.** Forced by
GitHub retiring the Ubuntu-22 runner images (deprecation 2026-09-17, unsupported
2027-04-17), which fired a registered ROADMAP trigger nobody had noticed until
an owner question made it worth verifying.

**The consequence he accepted is product reach, not a version number:** the
minimum glibc rises 2.35 -> 2.39, so Ubuntu 22.04 LTS and Debian 12 can no
longer run any Linux artifact, the AppImage included. Building on 26.04 was
rejected because its 2.43 floor would drop Debian 13 too. Separately measured:
the rpm hard-requires `libwebkit2gtk-4.1.so.0`, which stock RHEL 10 repositories
do not carry - EPEL does - so RHEL support is qualified in both artifact tables.

**PROVEN 2026-07-29, and this closes the base move's one open verification.**
Nothing local could show it - no gate part reads `release.yml` - so a draft
rehearsal build was dispatched (`workflow_dispatch` with the draft flag, run
`30491217194` on `fd78bfc`): all four bundle legs plus assemble green, including
the AppImage step, which was the named risk as the only step depending on host
library layout rather than package names. **The owner then installed and ran that
build on Fedora and reports it working.**

**Two standing consequences for any future text:** every row or sentence telling
a reader which systems an artifact runs on states the requirement rather than a
distribution family standing in for it; and the texts that state the floor are
found by grepping the tree, never from an enumeration - two enumerations of them
went stale within one afternoon.

## Next steps (priority order)

1. **The owner's full product QA pass - the only thing that can close 1.0
   scope.** His timing condition ("once the next plan is implemented") is met,
   and the build he needs exists and works: run `30491217194` on `fd78bfc`,
   seven artifacts plus SHA256SUMS, a draft that is not a tag and is not
   published. **Round 2 is partly done** - he installed and ran it on Fedora and
   reports it working, which also proved the new 24.04 base. What is still
   untested by him: the same build on Windows and macOS, and the product's
   feature surface, which the QA-gate section above enumerates (a real dry-run
   and run over his own library, the profile editor including rule add and
   remove, suggestion apply, the jobs view during a live batch with a mid-run
   cancel, run history, the locale switch, help mode). A later build off a newer
   head would need a fresh `workflow_dispatch` with the draft flag.
2. **The vulnerability vehicle** (ROADMAP, pre-1.0 gates): its own one-task
   plan, and the part that outranks both alerts is the disagreement, not the
   bumps. Bump `postcss` past 8.5.17 through the lockfile; **MEASURE** why
   `cargo deny check` is green on this tree while GitHub reports a Rust advisory,
   rather than restating the informational-class hypothesis; **INVESTIGATE**
   `glib` only, and if it cannot move independently of Tauri's tree, say so and
   give it its own vehicle. Until that measurement exists, neither `cargo deny`
   nor the GitHub feed may be quoted as coverage.
3. **Renovate: nothing is owed by anyone, one thing is worth watching.** It is
   running - dependency-dashboard issue #2 exists since 2026-07-29, after the
   owner forced a run from the hosted portal. Its activation trigger fired and
   was deliberately re-deferred to a sharper observable, because the config's
   cadence is the 1st to 3rd of the month: **when its first dependency PRs land**
   (expected 2026-08-01 to 08-03; security updates bypass the schedule), walk the
   18 commented RUSTSEC ignores in `deny.toml` and drop the ones those PRs make
   obsolete, and take the TypeScript-7 bump when the typescript-eslint ceiling
   allows. Cosmetic residue, the owner's: the inert `renovate/configure` branch
   from the closed onboarding PR is still on the remote; the agent's granted git
   shapes do not include branch deletion.
4. **Four routed items waiting on a package that touches their file**, all with
   vehicles in the ROADMAP's "Docs accuracy" section: the one surviving
   line-number citation in `.github/workflows/ci.yml` (now IN scope, since the
   owner widened the ruling to CI and configuration comments on 2026-07-29); the
   `raw:` "byte-exact" wording, which is wrong in the spec, the matcher's comment
   and the README at once because `scalar_eq` compares Int against Float; the v1
   spec's section 8.1 synopsis omitting `validate`'s flags; and `BUILDING.md`'s
   three positional gate ordinals plus its one over-80 prose line.
5. **Archive duty:** session 28 is archived by the NEXT session; session 27 was
   archived at the start of this one.

## Open questions / risks

- **All three owner questions from this session are RULED**, so nothing waits on
  him: the comment line-number ruling now reaches CI and configuration comments;
  a growth-prone README figure loses its number rather than gaining a
  maintenance duty, which he applied to both figures in that paragraph; and the
  Linux release base is 24.04 with tests on 26.04. The runner-image exclusion in
  `renovate.jsonc` was surfaced for his confirmation and he did not object while
  ruling on the base move that motivates it; it stays as shipped, recorded on
  `ci-04-dependabot-cadence`.
- **The comment-citation class is closed WITHIN its corpus selector, not
  tree-wide.** Any sentence saying Plan 10 closed that class names the selector -
  source files in six extensions - or it is false. One member survives in
  `.github/workflows/ci.yml` and is routed.
- **A shipped `deb` declares no `libc6` dependency**, so on a system below the
  glibc floor `apt install` succeeds and the binary fails later with nothing
  having warned the user. `docs/INSTALL.md` is the only channel that warns, which
  is why its floor statement sits before the runtime-requirements list rather
  than inside it. Do not move it back.
- **The reach-claim checker exists but is deliberately not in the gate**
  (ROADMAP candidate section): it parses prose, which is how such a check becomes
  permanently red on correct content. Reconsider if a third artifact table
  appears - the README's `placeholder(1.0)` mandates one at the tag.
- **The controller's own error class stayed the most frequent, again**, and again
  every instance was caught by something else - twice by the owner directly.
  Ledgered: four commits into a tree with a live product writer using `git add`
  plus a bare commit rather than the pathspec-scoped form; a review brief
  demanding byte-identity against a commit the controller then moved; one verdict
  harvest mined after the next dispatch instead of before it; one occurrence
  written from recall that a measurement corrected; a stale plan acceptance row
  copied forward as current state (the Renovate activation, which the owner
  corrected); and a red CI run dismissed as superseded without being read.
- **The scoping lesson this session paid for three times, now a house rule:** when
  a change moves a FACT that several texts assert, the scope unit is the set of
  assertions, not a file list - derive it by grepping the tree for the fact, and
  put the RULE in the brief rather than the member list.
- Framework-side follow-ups are tracked agent-side.
