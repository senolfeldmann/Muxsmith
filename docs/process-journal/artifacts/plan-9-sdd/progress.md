# Plan 9 - progress tracker

The plan document carries no progress (house deviation stated in its header);
this file is the tracker. Controller-written. A task's state is its row here,
never a ticked box in the plan.

| # | Task | State | Commits | Verdict |
|---|---|---|---|---|
| 1 | Planner seam `pipeline::plan_pipeline` + four call-site migrations (D91-D95, S-4/S-8) | DONE | `9bbe53d`, doc fix round `fed55be` | `task-1-verdict.md` APPROVED; three findings closed by the delta |
| 2 | `run_batch` hoist into core; src-tauri runs-root seam deleted (D96, D97) | DONE | `9b2843f` | `task-2-verdict.md` APPROVED_WITH_MINORS; no code change required, residue below |
| 3 | Worker-panic payload end to end (D98-D100, S-1/S-2) **plus the amendment-3 rustdoc restatement (Step 2)** | DONE | `9e5e112`, fix round `4e73739` | `task-3-verdict.md` NEEDS_FIXES (1 BLOCKING, 1 MEDIUM, 1 LOW) -> fix round -> delta APPROVED |
| 4 | `EmptyRawProperty` + Run-gate e2e scenario (D101, S-1/S-3/S-5/S-6) | DONE | `d768657` + `3412fcc` (the amendment-4 fix round) | `task-4-verdict.md` APPROVED_WITH_MINORS; no code change required, all minors are report/tracker items routed by the controller |
| 5 | Central severity sort + BatchView code-keyed fetch (D102, D103, S-7) | DONE | `e134fdc` + fix round `17505d8` | `task-5-verdict.md` APPROVED_WITH_MINORS -> fix round -> delta APPROVED; the delta added LOW-4, routed to the close |
| 6 | D23 e2e tests + mount-harness hook + `name()` hoist (D104) | DONE | `a2c1028` (one commit; the task returned NEEDS_CONTEXT mid-run and committed after the amendment-5 ruling) | `task-6-verdict.md` APPROVED_WITH_MINORS; LOW-1 routed to the close, four INFO items recorded |
| 7 | D49 G1/G2 removal experiment (D105) | MEASURED, anomaly branch | none by design (no file, no commit; tree byte-identical) | `task-7-verdict.md` APPROVED_WITH_MINORS; two report-local minors, one MEDIUM against the DESIGN, and one open fork for the close |

## Amendments

- **Amendment 1** (owner rulings, session 24): tests ship with the feature; no
  GUI identification session cache. Design and plan each amended by their own
  author and delta-reviewed by their own reviewer.
- **Amendment 2** (owner ruling, session 24): the e2e `name()` helper hoist,
  plan-side only (Task 6 Step 3).
- **Amendment 3** (owner ruling 2026-07-28, session 25, mid-execution after
  Task 2): `run_batch`'s moved rustdoc is restated for its core home. Design
  half `08621cb`, delta review APPROVED with no findings
  (`amendment-3-verdict.md`); plan half `36d8538` (Task 3 gains Step 2, its
  steps renumbered to eleven) plus fix round `63fc5b2` (two line edits: the
  ruling re-attributed to its owning ledger entry, Task 3's must-not list
  swept), delta APPROVED. **Amendment 3 is CLOSED on both halves.** The
  replacement doc comment lives ONLY in D96's amendment-3 rider; the plan
  points at it.
  Authors and reviewers were fresh, not resumed: the session-24 agents cannot
  be resumed across a session boundary.

- **Amendment 5** (CONTROLLER-ruled 2026-07-28, mid-execution inside Task 6;
  the first amendment of this plan not ruled by the owner, and the log labels
  it as such). D104's item 2 fixed an assertion - `cancel-batch` is disabled
  after a fresh dispatch is rejected - whose target control cannot render in
  that scenario: its own `v-if` names `runActive` among three disjuncts and the
  fresh branch clears the other two before the invoke. The adjudicated
  behaviour is confirmed, measured by mutating the catch arm; only the vehicle
  was impossible. Ruled: item 2's second assertion asserts the control's
  ABSENCE paired with the positive bearer (`jobs-empty` visible, the `v-else`
  of the same condition). Both halves in one commit `1e0dbd8`, one author, one
  reviewer, verdict APPROVED_WITH_MINORS. Contested-criteria audit for the solo
  ruling is in the design's Round-5 log entry and was re-run independently by
  the reviewer.
- Amendment-5 review residue with no artifact change: LOW-3 (the rider says a
  paragraph is "likewise unchanged" whose text the same commit qualified - the
  delta record is complete, the wording is loose) and INFO-1 (the pairing's
  recorded reason is true but weaker than the one that carries it in situ).
  Both are rider-wording items to fold in only if that rider is revised anyway.

## Task-2 residue routed by the controller (no code change)

- MEDIUM-2, the fixture-value mutation decided at the keyboard instead of
  returned: recorded as a route violation on
  `latitude-carveout-zero-content-structural-forks`. The committed code stands
  - it follows the destination file's unbroken pattern, and the reviewer
  measured the mutation unobserved by any assertion.
- LOW-1/2/3, report arithmetic (an undercounted comment rewrite, a
  minority-form count off by one, "22 body lines" double-counting the
  signature): no artifact change; recorded so the plan close does not
  re-derive them.
- Harvest mined into the house files in `851ada1`, `d7fd277`, `5b38d59`.

## Carried into Task 3 (from the Task-2 verdict and amendment 3)

- The four silently-discarded executor failures (`job.rs` create_dir_all,
  `joblog.rs` remove_dir_all, `spawn.rs` kill, `spawn.rs` wait) STAY discarded.
- Anchors shifted by `9b2843f`, so the plan's authoring numbers are stale:
  `fn recover_panicked_worker` at `queue.rs:424`, the `eprintln!` at `:441`,
  `worker_panic_is_reported_as_failed_not_cancelled` at `:783`. Locate by
  content.
- The Task-2 implementer's claim that Task 3's `JobOutcome` sweep must also
  cover the two moved tests was refuted by its reviewer: neither test mentions
  `JobOutcome`.
- Task 3 removes imports, so the import-removal doc-link sweep applies
  (ledger `an-import-removal-sweeps-the-doc-links-that-named-the-symbol`).

## Owner rulings, session 25 (2026-07-28)

- **Amendment 3** (rustdoc restatement as a design change, riding Task 3) -
  closed, both halves approved.
- **File-vs-within-file boundary** adopted into
  `latitude-carveout-zero-content-structural-forks`.
- **Precedence, a plan's pinned-test enumeration vs Tier-2
  `tests-ship-with-the-feature-never-after`: A plus C.** At EXECUTION time the
  Tier-2 rule wins narrowly - the implementer BUILDS the missing producer when
  all four hold (additive; existing infrastructure; the consequence is created
  by this package's own diff; named in the report for the reviewer to rule on);
  outside those four the enumeration binds and the fork returns. Companion at
  the PLAN REVIEW: the coverage walk runs over each acceptance observable's
  HALVES to a named producing test, which is where Task 3's gap actually came
  from. Recorded in the Tier-2 entry; the plan-review half is doctrine
  (agent-side), not project data.
- **Amendment 4** (Task 4's German subprocess test): option B with the
  delegation sharpening - a locale-parameterized PINNED helper in
  `crates/muxsmith-cli/tests/support/mod.rs`, with the existing funnel
  delegating to it. Plan halves `ba69c36` + fix round `4e5daa6`, both
  approved; implemented in `3412fcc`. The controller's "exactly one
  `cargo_bin` call site" wording was refuted by the plan author and corrected
  to the FILE-level form D64 actually carries (the bare helper keeps its own
  call inside its closed exception).
- **Model tiering, owner ruling 2026-07-28 (S25):** the top model now serves
  ONE role, the plan-close whole-branch review and its deltas. Design/plan
  four-eyes rounds and decision documents run the mid tier. Two cost levers
  ride agent-side: a small amendment gets one author and one reviewer for both
  halves unless it adds, removes or re-cuts a TASK; a tiny implementer-side
  fix round goes to a fresh agent, never displacing the resumption rules for
  delta reviews or for a four-eyes artifact's own author.

## Task-5 residue routed by the controller

- **LOW-1 and LOW-2 are report-count defects, no artifact change.** The report
  states "34 test binaries"; the measured house unit is 39 `test result:` lines
  = 35 test binaries + 4 doc-test targets, which is what Tasks 3 and 4 also
  recorded. And it says the `have_mkvmerge()` gate has five neighbours in
  `dry_run_cli.rs`; there were six at the parent commit, seven now. Recorded
  here so the plan close does not re-derive them, same routing as the Task-2
  report-arithmetic minors.
- **The `--document-private-items` finding (HARVEST 4) is a reinforcement, not
  a new item.** The ROADMAP's "Gate: rustdoc does not link-check private items"
  entry already records both ambiguity errors at `src-tauri/src/lib.rs:54`
  and `:87`, their `mod@run` / `run()` repair, and the requirement that the
  flag's two consuming sites and the fix land in the same pass. The review
  reproduced it independently; no vehicle was missing.
- **HARVEST 6 got its vehicle in the ROADMAP** (D102's unguarded scope
  boundary, measured total). Disposition at the close is the owner's.
- Harvest items 7 to 11 mined into the house files at the verdict-arrival gate:
  two occurrences (`tests-ship-with-the-feature-never-after`,
  `latitude-carveout-zero-content-structural-forks`) and three new Tier-1
  entries (a composed doc claim about a call-site set; an e2e fixture verified
  against its emitter rather than its neighbours; frontend mutation evidence
  needs a rebuild before the e2e run). The reviewer proposed folding the first
  into the import-removal sweep entry; it was opened narrow instead, because
  statement-fit does not carry a claim that no longer HOLDS under an entry
  about a link that no longer RESOLVES.

## Task-5 delta round (fix `17505d8`, delta APPROVED)

- MEDIUM-1 and LOW-3 both close. The reviewer re-classified all nine
  `severity_sorted` call sites with a fresh instrument to check the new
  appositive, and fired the second half of the fix's compound assertion
  (`is_array`) that the fix implementer's own mutation had not reached.
- **LOW-4 is new and the reviewer recorded it as its own defect**: its
  first-pass required change preferred one remedy on a rationale it had not
  run. The added `files` assertion is a shape guard, not a builder
  discriminator - the real profile-load-failure document satisfies it, measured
  - so three strings now overclaim. The test stays; the texts go to the close
  (ROADMAP), including the licence for the pre-existing message the fix brief
  had put off limits.
- Delta harvest mined: an occurrence on
  `proc-verification-step-must-be-falsifiable` (the per-assertion fire duty
  reaches inside a compound predicate) and two new Tier-1 entries (a remedy
  preference in a verdict is measured or stated without the comparative; a
  fired check still needs a reachable failing input).
- Baselines Task 6 measures its delta against, re-verified by the reviewer at
  `17505d8`: `cargo test --workspace` 39 `test result:` lines all ok, 0 skip
  markers; `pnpm test:e2e` 64 passed; `ls e2e/*.spec.ts` 9.

## Task-6 review residue

- **The four ruled tests are discriminators, measured.** The reviewer built four
  targeted source mutations and each reddened exactly one test and nothing else
  in the 68-test suite: the pre-D23-fix implementation for tests 2 and 3, the
  literal post-resolve reading the plan-5 round-2 verdict rejected for test 1,
  and an unwired D100 for test 4. That is the acceptance evidence for the D23
  item's coverage half and is stronger than "the tests pass" - cite it when the
  ROADMAP D23 item is marked resolved.
- A fifth mutation settled a premise nobody had measured: the spec's page-side
  handler genuinely reproduces the Rust command's emit-before-resolve ordering,
  observed from the frontend side rather than inferred from the Rust doc
  comments.
- **LOW-1 routed to the close** (the spec-local IPC installer's undisclosed
  divergence from the shared one), with its ROADMAP entry.
- INFO items with no artifact change: the header comment's "same gating
  condition" is true under the `v-if` reading and loose under the disabled-binding
  reading, and it echoes the rider's own wording; a third `.generated/` path
  declaration now exists and got a ROADMAP trigger keyed on DECLARATION SITES.
- **Controller defect, recorded rather than smoothed over:** my review brief
  said three commits landed while Task 6 was mid-flight. Two did. The reviewer
  measured it; occurrence on `proc-57-briefs-not-ground-truth`. It fed the
  commit-decision adjudication's risk weighing, so it was not inert.
- Harvest mined at the verdict-arrival gate: occurrences on the absence-idiom
  entry (its rename clause is now measured, not argued) and on the
  returning-task entry, whose statement gained a FOURTH consideration - a fenced
  commit message must stay true of what is committed, so a subset that omits the
  artifact the message names is uncommittable regardless of the other three.

## Task-7 result and the fork it leaves for the close

- **Measured: G1 green, G2 red, G3 green** - the anomaly branch of D105's
  decision rule, matching the design's own worked example of one. No removal in
  any direction; all three guards stay. The reviewer reproduced every cell
  independently and the mutated tree's blob hash matched the task's, so the two
  runs were the same tree content, not merely the same diff text.
- **Why the experiment could not answer its question, measured rather than
  argued.** The fenced mutation site, `delta_for`, has three call sites in core:
  two build the candidate's simulated apply, one is the applier. The engine
  re-validates its own candidates, so every degraded `AddExact` candidate was
  dropped and replaced by its NOT-polarity twin before any guard saw it. G2 went
  red through its anti-vacuity assertion, never reaching its type comparison;
  G1 stayed green because its counter counts both polarities and still read
  three. The reviewer then applied the same defect class at the APPLIER site
  only, in an isolated crate copy, and all three guards went red through their
  own assertions - the pattern D105 hypothesized, under a mutation D105 did not
  fence.
- **Open fork for the close (owner-facing).** D105 fixes statement text for the
  two clean branches and none for the anomaly, while its step 4 mandates that
  the anomaly be recorded. So `core-d49-g1g2-experiment` needs a
  controller-composed, strictly factual statement, marked in the entry as not
  one of D105's fixed texts - and the owner decides whether the experiment is
  re-fenced at the applier site or the D49 question stays open with the guards
  standing.
- ROADMAP consequences for the close: the consumed D49 trigger line names only
  the two clean branches and needs wording for an outcome its own text does not
  anticipate; "Plan-9 design trigger 4" stays registered and NOT fired, but its
  condition is now unreachable without a re-fenced experiment, so it is a
  trigger waiting on a protocol nobody has scheduled.
- Report-local minors for the roll-up funnel: an off-by-one `:line` citation
  (the `#[test]` attribute line instead of the function line) and a mechanism
  INFERENCE filed under "verified at the source" - correct, but its site
  (`planner.rs:1406-1413`, where the engine re-validates candidates) went
  uncited.
- Tree hygiene routed to the ROADMAP: a stale cross-file line citation in
  `suggestions.rs:1015`, to be fixed by whichever task next owns that file.
- Harvest mined: an occurrence on `proc-proposed-safeguard-stays` (its scenario
  ran to completion for the first time, and the rule held) plus two new Tier-1
  entries - a mutation experiment mutates a site only the guard's subject reads,
  and an anti-vacuity counter counts the dimension the guard is about.

## Plan close (2026-07-29)

- **Ten-part gate green foreground, no subsets**, plus `ledger-lint`, before the
  push of `9143075..23136b6`. CI observed in progress on that head.
- **Whole-branch review, top tier: NEEDS_FIXES, then READY after the fix wave.**
  Its finding is the one no task review could reach: spec amendment S-8 removed
  the GUI session cache, and two rustdoc passages in `identify.rs` still
  described it, both citing the section that now says the opposite. Task 1's fix
  round had swept for the literal string `per-session` and closed a third site;
  these two say the same thing in other words. Fix `96dbcf6`; the delta review
  re-measured the sweep with a wrap-insensitive method (its own line-based pass
  would have missed the module doc, which wraps mid-phrase) and confirmed no
  fourth site.
- **The fix round's out-of-scope sweep found a third artifact class**: Tier-2
  `core-20-ondisk-cache` asserted the same ruled-out lifetime in its statement
  AND its `blocked_on`. Corrected in `e255d40`. Code, module docs, always-loaded
  convention - three classes, one stale premise.
- **Controller defect, second of the session:** my ledger occurrence had
  restated that literal-phrase measurement as a claim about the class. Qualified
  in `b40db26`.
- **Delta LOW carried forward, not fixed:** "constructed per planning call"
  reads as an exclusive constructor claim while two identify surfaces also build
  the cache. The semantics hold at all four sites; the wording is a one-clause
  sharpening for the close's text-corrections pass, and it was seeded by the
  reviewer's own suggested wording, which it says so itself.
- **Promotion sweep done** (`b36296e`): all five owner-ruled entries promoted
  into their nature files, three technical-code and two product-scope, each
  block moved verbatim except tier and `promoted_at`. Two carried pre-landing
  tense and were rewritten - an always-loaded entry saying "the eprintln goes
  away" or naming "today's `n/a` exit code" would bind future tasks to a state
  this branch already changed.
- **`core-121` settled** in the same commit: the seam is built, its interface is
  the thing the entry left open, and the measured duplication it recorded is
  gone.
- **`core-d49-g1g2-experiment` written** with a controller-composed statement
  that says so in its own text, because D105 fixed wording for two clean
  branches and none for the anomaly it also mandated be recorded.
- **Blocked-pool sweep** (`1dc85f0`): 18 entries re-read, none settled, one
  premise-stale (`testing-cli-helper-dedup` claimed no shared CLI test-support
  module; one has existed since `aba7f4f`), corrected in place with the work
  itself measured as still undone.
- **ROADMAP bookkeeping** (`922bec9`): anchor marked executed and closed, IN
  items named against their commits, both D49 trigger lines dispositioned.

## Post-authoring ground-truth edits (controller notes)

- 2026-07-28 05:06: added `source: human` + `nature: technical-code` to `exec-43-runsroot-debug-gated` in `docs/decision-ledger.yaml` (commit 2155c1d), resolving the NEEDS_CONTEXT the plan author routed. The plan document authored at 05:04 therefore carries a corrections-table row and a close-action rider whose evidence line no longer reproduces; the routing is SATISFIED, not open. Ledger anchors cited by the plan shifted by two lines (`gui-d23-reset-gating-form` :4535 -> :4537).
- 2026-07-28 (session 25): this tracker existed with only the note above - no
  task rows for Tasks 1 and 2, though both had run. Surfaced by the
  amendment-3 plan author; filled in here. The plan's header names this file
  as the tracker, so an empty tracker is a controller defect, not a house
  deviation.
