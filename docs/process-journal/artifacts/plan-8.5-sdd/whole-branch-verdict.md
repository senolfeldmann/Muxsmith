# Whole-branch verdict: Plan 8.5 (macOS packaging fixes)

Reviewer: independent whole-branch (top tier), wrote none of the package.
Date: 2026-07-28. Every number below is my own measurement on this tree
unless it is explicitly cited to a named prior artifact; every absence
check I ran was fired by its own invocation against a state where it hits.

## VERDICT: READY

Nothing must change before the plan closes. The close actions listed at
the end are the plan's own, plus one live-state item (the unpushed
ROADMAP commit) that the close's push resolves.

---

## 0. Scoping verification (done first)

- Range pinned: `f627105..61dc522`. HEAD measured at review start:
  `61dc5222303795e5ef73789b5c43714d609c69d1`. Working tree clean
  (`git status --porcelain` empty).
- The range contains **nine** commits (enumerated, each `git show --stat`
  read): `ff05ac3` (plan round-1 review fixes), `9460daf` + `5060ef5`
  (Task 1 + fix round), `50e08cd` (Task 2), `29ef17b` (plan amendment),
  `87c1dee` (Task 3), `338e779` (ledger entry), `8ad4392` + `61dc522`
  (ROADMAP flake record + owner ruling). The brief's commit enumeration
  omits `ff05ac3`; it is inside the range and is covered by the plan
  review delta (APPROVED per the tracker), so nothing in the range is
  unreviewed. Noted per `proc-57-briefs-not-ground-truth`, no consequence.
- All **11** changed files (diffstat measured: 213 insertions, 43
  deletions) attribute to exactly one commit each; no file in the range
  belongs to no task/controller action, and Task 4 wrote no repo file
  (no unattributed commit exists - checked by exhaustion of the nine).
- Live-state note: `origin/master` is `8ad4392` (live `git ls-remote`),
  local master one commit ahead (`61dc522`, `docs/ROADMAP.md` only). See
  N1 below.

## 1. Cross-task integration - PASS

- **The shared file (Task 1 + Task 3), `.github/release/draft-body.md`:**
  final state carries Task 1's line-1 wording inside Task 3's joined
  line. My counts on master: `^|` = 9 (table only), `^\| \[` = 0 (fired:
  same pattern on the pre-Task-3 blob `50e08cd:` = 2), `__VERSION__` = 8,
  `^---$` = 1. All four match the plan's Step-3 post-state.
- **The shared log (Task 1 + Task 2), design `## Amendment log`:** A4
  (extended by the fix round with the D75-outline-bullet record) precedes
  A5; both read against the tree they describe: `tauri.conf.json` macOS
  block carries `"signingIdentity": "-"` as its only change in the range,
  `tauri.macos.conf.json` is exactly `{"bundle": {"licenseFile": null}}`
  (both files parse: `python3 -m json.tool` clean).
- **Configuration vs documentation:** the evidence chain closing the gap
  between "config says" and "docs promise" is the strongest available and
  I read it rather than re-ran it: RR3 differential CodeResources 0 -> 1,
  RR4 differential LPic 2 -> 0 (Task-4 reviewer, own instruments, R8
  defect artifact as counterfactual), release.yml's `-c` overlay
  precedence measured by the Task-2 reviewer (both levers -> cleared),
  and the owner's hardware acceptance of O1/O2/O3 on 2026-07-28.
  BUILDING.md's new paragraph, INSTALL.md's intro and Gatekeeper text,
  and the ROADMAP S22 supersession bracket all state the same posture;
  no contradiction found between any pair of surfaces.

## 2. The union against its contracts - PASS

Walked the three ROADMAP "Pre-1.0 release gates" finding entries and the
three kickoff rulings; each has a named implementation on master:

| contract | implementation | acceptance |
|---|---|---|
| BLOCKER "damaged" / ruling 1 | `signingIdentity: "-"` (9460daf); S22 wording sweep incl. ROADMAP bracket, INSTALL.md, D75 note, D86 bullet, A4 (+5060ef5) | RR3 machine half; O1 ACCEPTED 2026-07-28 |
| BLOCKER-adjacent dmg SLA / ruling 2 | `tauri.macos.conf.json` licenseFile null (50e08cd); BUILDING.md; A5; Windows guard RR5 | RR4 machine half; O2 ACCEPTED 2026-07-28 |
| release-body links / ruling 3 | draft-body joins (87c1dee) | RR2 machine half; O3 ACCEPTED 2026-07-28 |
| blocker entry's "Whatever lands, INSTALL.md re-verified" duty | folded into O1 | owner confirmed INSTALL.md matches the walked flow ("ja, passt beides") |

No requirement without an implementation; no implementation without a
requirement (the range's remaining commits are plan/process bookkeeping,
each traceable to a review routing or an owner ruling recorded in the
tracker).

## 3. Surviving latitude - NONE FOUND

Scanned every normative sentence the branch added (plan Global
Constraints rewrite, Task-1 read-list rewrite, branch-rule rewrite,
fallback-A5 block, ROADMAP flake entry, ledger entry statement):

- The two pointer contracts ("EVERY amendment ... at the log's state at
  execution time"; "the `## Amendment log` in full") resolve membership
  by opening one named file - observation, not invention. Settled by the
  plan-amendment verdict; concur.
- The flake entry's trigger set ("this test, or any other fake-binary
  test using that helper") is defined by a greppable property
  (`support::fake_mkvmerge_that_fails_queries` callers), and its
  handgriff is named (systematic-debugging pass; candidate fix carried
  with its claim-class boundary stated).
- O4's "(including any still-standing plan-8-era rehearsal drafts)" is
  resolved by `gh release list`, an observable.
- My stale-enumeration sweep of the ruled class: `git grep -nE
  'A1-A3|A2 and A3|A1-A5|A1, A2 and A3' -- docs/ ':!docs/process-journal*'`
  = **0 hits** on master, fired at BASE `f627105` = **3 hits** (the two
  plan sites + the design status line). The Task-2 ruling's cure is
  complete on the live surface.

## 4. The no-work-needed check - ALL PREMISES REPRODUCED

Each place the package concludes a guard/check/step is unnecessary, I ran
the premise:

- **"No Windows hardware step - the msi surface is untouched by
  construction":** the range's only `src-tauri/tauri.conf.json` hunk is
  the one-line macOS addition (whole-branch diff read); RR5 additionally
  measured the license text present in the shipped msi. Holds.
- **"Task 3 needs no design amendment - 4.2 already tree-authoritative
  per A3":** A3's site list names "4.2 release-body template ...
  superseded by the tree" (design read at the A3 block); A4 restates it.
  Holds.
- **"No new triggers registered - the signing-revisit trigger covers
  ad-hoc's successor":** ROADMAP trigger read: "First external-user
  complaint about unsigned-install hurdles ... -> re-evaluate code
  signing/notarization per OS", and ad-hoc signing leaves the install
  hurdle in place, so the trigger's subject is intact. Holds.
- **"Zero new GitHub Actions; nothing to SHA-pin":** `git diff
  --name-only f627105..61dc522 -- .github/workflows/` empty; control:
  same invocation on `.github/` returns `draft-body.md`. Holds.
- **"No Rust changed in this plan" (flake-entry premise):** `git diff
  --name-only f627105..61dc522 -- '*.rs' 'Cargo.toml' 'Cargo.lock'`
  empty; control: unfiltered file list = 11. Holds.
- **Index-race end state:** `86bfd69` still exists as an object;
  `git diff 86bfd69 87c1dee` is empty on my own run - the transient and
  final trees are identical, nothing was lost in the repair. Holds.

## 5. House conformance by entry id - PASS

Every id the package cites resolves in the four YAML files (each grepped:
`proc-latitude-clause-boundary`, `proc-normative-count-recomputed`,
`proc-03-model-assignment`, `proc-07-verify-against-source`,
`proc-57-briefs-not-ground-truth`,
`gate-includes-cross-target-lint-for-the-unrun-os`, new
`concurrent-writers-need-pathspec-scoped-commits`), and the new ledger
entry passed ledger-lint in ci run 30312472265 (tracker; ci green on
`8ad4392`, which contains it).

The supersession rule (`proc-supersede-never-overwrite`) was applied at
all four sites where old wording was reversed: ROADMAP S22 record keeps
its original words plus a bracketed pointer; D75's ruling paragraph keeps
its text plus a note; the plan-8 plan's frozen fence keeps its bytes plus
a note beside it; the D75 outline bullet - live descriptive prose, not
frozen graded material - was corrected in place with A4 recording the
change, which is the correct boundary between the two treatments. The
model-tier table was followed as declared (cheap/mid/cheap/mid
implementers, mid reviewers, this review top tier).

## 6. Findings by severity

No MAJOR findings. Nothing blocks the close.

- **N1 (NOTE, live state):** local master is one commit ahead of origin
  right now (`61dc522`, ROADMAP-only: the owner's flake ruling). This is
  Task-4 flag 1's state persisting past the run it was flagged for. Not a
  defect of any task - pushes are controller actions - but the close's
  ROADMAP bookkeeping commit must ride a push that carries `61dc522`
  with it, or the owner-ruling record exists only locally. Absorbed by
  the close's normal push; named so it cannot be forgotten.
- **N2 (NOTE, tracker figure):** `progress.md` records "Task 1: report
  DONE (commit 9460daf, 8 files)". Measured: `git show --numstat 9460daf`
  = **7** files, and the Task-1 report itself says "seven files". The 8
  is a controller transcription slip in the tracker - exactly the
  borrowed-number class `proc-normative-count-recomputed`'s first
  occurrence records. The tracker is salvaged scratch; this verdict,
  salvaged beside it, is the correction (same self-correcting-record
  mechanism the run used for Task 3's report figures). No dispatch owed.
- **N3 (INFO, pre-existing prose):** INSTALL.md's intro says "so your OS
  will warn before the first launch", which its own Linux section
  contradicts ("No gatekeeping dialog exists on Linux"). The overclaim
  predates this branch (BASE text made the same claim); user-facing prose
  is owner territory. Record only; a candidate for the owner's next
  wording pass, not for this close.
- **N4 (INFO, amendment wording):** A5 freezes
  `src-tauri/tauri.macos.conf.json` "with exactly the content above",
  rendering the JSON inline; the committed file is the same JSON value
  pretty-printed over five lines (the plan's Step 5 carries the
  byte-exact form). Value-identical, byte-different rendering inside a
  prose record. No consumer reads A5 as a byte source while the plan
  exists; not worth an edit.

## 7. Deferred-item triage (one line each)

1. **Flag 1 cure - zero-commits-ahead dispatch precondition:**
   close-batch line - record in the house files (a ledger entry or an
   occurrence on the dispatch-discipline neighborhood), NOT a plan
   amendment; the plan retires at close and the rule is generic: "a
   `workflow_dispatch --ref master` resolves server-side, so dispatch
   requires `git rev-list --count origin/master..HEAD` = 0, or the ahead
   diff measured and shown to touch no pipeline input."
2. **Flag 2 - R5's presence check converted to a count of legs, silently
   reused as a count of log lines:** close-batch line - a
   `violated-corrected` occurrence on `proc-normative-count-recomputed`
   (the count was computed over the wrong set and asserted from drafting
   intent; the entry's statement already reaches this, the occurrence
   makes it findable). The retired plan is NOT amended - the durable home
   is the house files, and the tracker + Task-4 verdict already carry the
   correct decomposition (4 echoes + 4 outputs). If the check is ever
   re-transcribed, restore plan-8 R5's per-leg presence shape; never
   hardcode 8.
3. **F2 - `timeout 5400` foreground wait exceeds the harness Bash cap;
   stop rule ambiguous under chunking:** close-batch line - one house
   entry (operational fact + rule: waits longer than the cap are
   specified as a TOTAL ELAPSED budget executed as chunked foreground
   invocations; a chunk boundary is not a tripped budget). Retired plan
   not amended, same reasoning as item 2.
4. **F1/F3/F4 (Task 4) + Task-3 M1/M2 + Task-2 two minors (report
   prose):** record only - the verdicts carrying the corrections are
   salvaged beside the reports; the durable record self-corrects.
5. **Controller's "no consumer" claim, false as phrased:** record only -
   already corrected in the tracker by the Task-2 implementer's
   disclosure; nothing further.
6. **O4 - delete the rehearsal draft "REHEARSAL - not a release (run
   30312889098)" (and any plan-8-era stragglers):** route to the owner -
   his by design; the only remaining acceptance-surface item.
7. **Flaky test `dry_run_json_emits_a_document_when_the_language_query_fails`:**
   no change needed - owner ruled fix-at-1.x, ROADMAP carries the
   candidate with its claim-class boundary; the registered trigger (a
   second data point) is the correct next actuator.
8. **RR4's R8-artifact corroboration window (retention expires
   ~2026-08-03):** no change needed - the recorded measurements (plan
   review LPic=2; Task-4 reviewer's independent 2 -> 0 differential)
   stand as the evidence after expiry, exactly as the plan pre-stated.
9. **Task-1 verdict's standing note (INSTALL.md described the expected
   flow before O1 had run):** discharged - the owner's 2026-07-28
   acceptance confirmed INSTALL.md against the walked flow; no action.

## 8. Consolidated fix wave

**Nothing must change before the plan closes.** The close proceeds with
its planned actions: push (carrying `61dc522` per N1), ROADMAP
bookkeeping for the three finding entries + Plan-8.5 section, the
close-batch house-file lines of triage items 1-3, journal/HANDOFF/salvage
per the plan's close section, and O4 with the owner.

## HARVEST

- **H1 (pattern, verified at whole-branch distance):** the
  enumeration-to-pointer cure killed its class, measurably - zero
  membership-enumerations of the amendment log survive on the live docs
  surface (fired 3 at BASE). "Replace the dependency instead of
  scheduling its next maintenance" held over three same-day instances
  and a fourth avoided one (A5's append staled nothing).
- **H2 (pattern):** a serial-execution ruling is a constraint on
  DISPATCH CONCURRENCY, not merely on task ordering. The no-worktree
  decision's stated premise ("strictly serial") was operationally
  breached the moment a mid-plan routing (the plan amendment) ran beside
  Task 3 in the same tree, and the shared-index incident followed at
  once. The ledger entry captures the mechanism; the scheduling half -
  "a routed side-task makes the plan concurrent; re-check the isolation
  decision at that moment, not at kickoff" - is the part a future
  controller can actually act on.
- **H3 (pattern):** the package's strongest evidence was differential
  measurement against a preserved defect artifact (R8 dmg: LPic 2 -> 0,
  CodeResources 0 -> 1, LC_CODE_SIGNATURE present in BOTH). Keeping the
  broken artifact within retention and naming it in the plan as the
  counterfactual made every absence check interpretable. Worth doing
  deliberately on any fix-verification plan: preserve the defect
  artifact FIRST, then write the checks as differentials against it.
- **H4 (brief/convention boundary):** dimension 5 required grading the
  supersession practice against "the rule", but no artifact in the
  ground-truth chain names its id; locating
  `proc-supersede-never-overwrite` took a three-pattern search across
  the YAML files. Cheap improvement: a brief that invokes a house rule
  cites its entry id, same as the plan already does for every other
  rule it binds. No stop was forced anywhere else; the gh-log duty never
  activated (read-only `git ls-remote` sufficed for the remote checks,
  per the plan's own RR6 precedent).
