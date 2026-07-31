# SDD ledger — plan: docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md

Workspace: `.superpowers/sdd/plan-12/` (house path, named by the plan itself;
the skill script's `<plan-basename>` default is deliberately not used - every
prior plan's scratch uses the short name and this plan's Execution-method
section fixes it).

Execution: seven strictly serial tasks on `master` in the main worktree, no
branches, no worktrees (plan's own ruling, owner-approved). The serial ruling
binds controller dispatch concurrency too: no second writer while a task is
live. Every commit pathspec-scoped.

## Pre-execution (controller, before Task 1)

- Session-start gate run. HEAD at session start `bd3aa34`, tree clean, nothing
  unpushed, no open PRs (so the Renovate first-dependency-PR trigger has not
  fired; expected 2026-08-01 to 08-03).
- Standing transcript duty discharged for the previous session, verified on
  disk by the controller (checksum identity, manifest row shape).
- Pre-flight conflict scan over all seven tasks: CLEAN. The two places a review
  rubric could read a mandate as a defect both carry their rationale in the
  plan - D108 decision 10's guard on a path decision 9 makes unreachable
  (`proc-proposed-safeguard-stays`), and Task 6's parity test deriving its key
  set from `include_str!("run.rs")` rather than a hand-written list. Neither is
  a plan contradiction; a reviewer finding on either is adjudicated against the
  plan text.
- Controller obligation from the previous session (Plan 12 authored and
  measured at `148f19f`, before Plans 11 and 11.5 landed): re-validation
  dispatched, brief at `pre-execution-revalidation-brief.md`.

## Controller items carried through this run

- **Task 4 Step 7 routes new test infrastructure.** The behavioural producer
  for the save-marking property (a test that moves the model inside one of
  `doSave`'s two await windows) needs a releasable mock response, which is new
  test infrastructure and therefore the narrow exemption
  `tests-ship-with-the-feature-never-after` names. The plan surfaces it for
  controller routing rather than writing "coverage follows later". Route it at
  Task 4's completion; it is not a licence to let the task ship without S1 and
  the report's residual statement.

## Pre-execution re-validation: CLEAR (report `pre-execution-revalidation-report.md`)

Measured at `bd3aa34`. 31 Class A members (fenced OLD strings and quoted
regions) all present exactly once in their named target file; 49 Class B
figures re-run, 3 moved, none of the three reached by a task step.

Controller spot-verification of the two load-bearing claims, run independently
of the recon agent:
- Task 1's two fenced spec-8.2 OLD strings: 1 occurrence each, with the
  post-amendment variant returning 0 as the discriminating control.
- `git diff --name-only 148f19f..HEAD -- help/ locales/ src/ e2e/ scripts/
  src-tauri/ Cargo.toml package.json` returns two files, both
  `editor-match-expr-exact.md`, neither a Plan-12 target. No source file this
  plan touches has moved since its authoring measurement.

The three moved figures, and their dispositions:
1. **D-numbering.** The plan states the highest D-number in use is D105 and
   that `\bD10[6-9]\b` returns nothing. Both are now false: Plan 11 landed
   **D111**, and D106/D109/D110 appear as references. Measured at HEAD: every
   hit is a RESERVATION reference, not an assignment - Plan 11's own design
   doc records "D106-D110 are reserved by Plan 12" and took D111 accordingly.
   **D106-D110 remain free and are assigned by Task 1 as the plan prescribes.**
   Carried into the Task 1 dispatch so the implementer does not read D111 as a
   collision or renumber.
2. **Correction 7** (the README's first YAML block did not load). Plan 11's fix
   round - the vehicle the plan itself names - has discharged; it validates
   clean now. No task acts on it.
3. **The ROADMAP round-3 entry** was an uncommitted working-tree edit at the
   plan's baseline and is committed now (`9ad9e05`). A strengthening: the
   tasks that cite it now cite committed text.

Also measured by the recon and worth carrying: the HANDOFF's premise about
which spec sections Plan 11 amended was slightly off (no section 7 edit; there
is a section 5.2 one). The conclusion is unaffected - none of Plan 11's hunks
land in section 8.2, the only region Plan 12 amends.

## Task log

Task 1: implementer DONE_WITH_CONCERNS, commit `b381675` (BASE `bd3aa34`).
Controller verified on disk rather than from the report: commit unsigned
(`%G?` = N), exactly one trailer `Co-Authored-By: Claude Opus 5`, exactly the
two files of the Files list, tree clean, one unpushed commit (correct - the
single push is a close action). Both fenced spec-8.2 replacements landed
verbatim; the decisions file carries D106-D110 with Decision / Rationale /
Rejected alternatives / information duties.
Task 1: three concerns routed to the reviewer as numbered adjudication
questions rather than resolved by the controller - (1) the brief's two
advance-named sweep hits are unreachable by the expressions that were meant to
reach them, so the question is whether the sweep's coverage is adequate;
(2) spec 8.4's both-languages statement does not name the Tauri shell, which
D110 makes a fourth such surface; (3) the `Triggers created:` slot is omitted
where none exists, against the plan-5.7 house form's explicit `none.`.
Task 1: review dispatched, mid tier.
Task 1: review returned. **Spec compliance MET** (all twelve rows checked
independently, both fences byte-exact, 8.4 byte-unchanged). **Task quality
APPROVED WITH FINDINGS**: 0 Critical, 2 Important, 6 Minor. Adjudications -
Q1(a) partly correct (the narrow claims hold, the generalisation does not),
Q1(b) NOT ADEQUATE with the blind spot named, Q2 compatible as written and
correctly a routing matter, Q3 implementer's precedence correct on corpus
evidence.
Task 1: verdict harvest MINED at verdict arrival, not deferred to the close
(commit `f083bc2`, ledger-lint green at 568 entries). H3 -> occurrence on
`design-empirical-claims-reproducible` (7 -> 8); H2 + Q1(b) -> occurrence on
`proc-sweep-surface-completeness` (8 -> 9); H1 -> new Tier-1 entry
`a-cited-exemplar-is-not-the-house-form-the-corpus-is`; H4 -> new Tier-1 entry
`a-second-implementation-of-an-existing-rule-is-the-standing-rejection-ground`,
opened at count 1 rather than 3 because the three instances are one authoring
pass surfaced by one review.
Task 1: fix round 1/5 dispatched (fresh implementer, mid tier - the findings
are written out to the line, so resumption would replay a large transcript to
change three passages). Scope: I-1, I-2 and M-1, all report-evidence defects
in `task-1-report.md`. **No tracked file changes in this round** and the
commit stands.

Task 1: fix round 1/5 returned. Report-only, no tracked file touched, HEAD
still `f083bc2`. Both Important findings and M-1 corrected in place with the
originals left verbatim and marked; four re-runs pasted, each with a fired
control. Two facts worth carrying: the fixer swept for the affected FACT rather
than the verdict's site list and found two further sites restating I-2's false
claim; and its own first instrument had the same defect class the round is
about (an awk pass that skipped bold-slot prose, reporting 8 where grep
reported 39), caught only because two instruments disagreed. It also reports a
subtotal discrepancy with the verdict's own M-1 figure and recorded it as
measured instead of reshaping it to agree.
Task 1: delta re-review dispatched to the RESUMED original reviewer (doctrine:
delta judgment is always the same judge), with the subtotal discrepancy, the
completeness of the widened correction sweep, and the deliberate presence of
the thirteen denylist characters in the report's control blocks all put to it
explicitly.
Controller measurement for that last point, so the close does not trip over
it: **no gate part scans for the typography denylist.** The expression over
`scripts/`, `.github/workflows/` and `BUILDING.md` is empty, and its control
fires eleven times against the report itself, so the empty result is a real
absence rather than a broken pattern. Salvaging this report at the close cannot
redden the gate.

Task 1: delta re-review returned - **I-1, I-2 and M-1 all ADDRESSED**. The
reviewer executed every pasted command by copying the characters out of the
document rather than retyping them, and diffed each against the output beneath
it; all reproduced. On the subtotal dispute it ran a third measurement with a
differently-implemented instrument and **ruled against its own verdict**: 3
references on 2 lines, the fixer's figure. It also ruled the widened correction
sweep complete (exactly three sites assert the fact, each now marked; no
fourth) and the deliberate glyph presence acceptable with one rider, recorded
as a ledger occurrence.
Task 1: **complete** (commits `bd3aa34..b381675` for the artifacts,
review clean after one fix round; `f083bc2` carries the harvest). One new
deferred minor from the delta round: **N-1**, the status paragraph's
correction calls both further defects "the same class (a check narrated instead
of pasted)", which fits I-1 exactly and M-1 only partly - M-1's root is an
incomplete enumeration inside the instrument, a different entry, which the
round's own C-2 states correctly. Nothing downstream turns on it.

Task 2: dispatched, mid tier, BASE `0c001ee`. Controller re-measured both of
the task's prescribed L1 figures at the current head before dispatch and both
reproduce the plan's baseline: the RED pre-state is exactly 2 lines, both in
`src/main.ts`, and `src/i18n/index.ts` carries 1 occurrence today, so the
control's end-state figure of 2 is reached by Step 1's own added line. Pasted
into the dispatch so the implementer meets a measured premise rather than an
inherited one.

Task 2: returned **NEEDS_CONTEXT** after Steps 1-4 complete and green. Two
forks, both measured, neither resolved at the keyboard - the behaviour the
latitude ban asks for. (1) The brief mandates interaction strings from `en()`,
but the new cases click Save while the interface renders German, so the locator
cannot find the control. (2) Case 3's second leg cannot switch live: `open()`
re-reads the baseline from a mock whose exhausted queue repeats `locale: null`,
so both sides of the guard are `null` and `applyLocale` never fires - the
persisted half asserts correctly, which is what makes it dangerous, since the
case would have shipped GREEN while measuring nothing on the live path.
  **[CORRECTED 2026-07-31: the last clause is wrong and is left standing so the
  wrong version is not reconstructed. The case goes always-RED, not green -
  measured twice independently. See the Task 2 review entry below.]**
Task 2: both ruled by the controller (internal test mechanics, nothing
user-visible, so not the owner's) and written to
`task-2-ruling-1.md` rather than into a message channel the reviewer cannot
see. Locators follow the RENDERING locale; the assertion rule is explicitly
untouched. Case 3 splits into its two directions, direction 2 from its own
stored-`"en"` scenario, preserving every prescribed assertion with no reload
and no `mocks.ts` edit. Making the mock stateful was rejected as new test
infrastructure and became a close-action tracker item instead.
Controller re-measured all three load-bearing mechanism claims at the source
before ruling: `open()`'s baseline reassignment, `mocks.ts`'s own documented
last-entry-repeats behaviour, and the catalog values (`settings-save` differs,
the two language-option labels are identical in both). The implementer's
analysis held on every point.
Task 2: implementer resumed with the ruling; it keeps Steps 1-4 in its working
tree, so no other writer may be dispatched until it returns.

Task 2: implementer **DONE** after the ruling, commit `ea39d88`, six files.
Controller verified on disk: unsigned, one trailer, exactly the Files-list
paths, tree clean. Full eleven-part gate green including the Rust block it had
deliberately skipped mid-flight (507 Rust tests, 72 e2e).
Task 2: the implementer did not treat green as evidence - it fired three
mutations against the shipped mechanisms and reports that each killed exactly
the cases depending on it. One of them vindicates ruling 2 empirically:
removing the live `applyLocale` reddens "returning to the system language",
which is precisely the assertion the brief's single-case form could not make
fail. The split restored an assertion's power rather than working around a
fixture.
Task 2: review dispatched, mid tier, with the mutation evidence named as the
highest-value thing to check - a mutation that fails to discriminate looks
identical to one that does, and this feature sits on a per-message locale
fallback chain, the exact structure that lets a red state pass while looking
real.

Task 2: review returned. **Spec compliance PASS** (all thirteen requirements
verified independently), **task quality PASS, strong**. 0 Critical, 1
Important, 3 Minor. All three adjudications answered: the split preserves every
prescribed assertion and adds three the single case could not reach; the
locator/assertion boundary is respected, with the sets derived from the file
rather than from the report; and the lint-warning observation affects no
coverage claim this task makes, fired in both directions.
Task 2: the reviewer also re-ran all three mutations with its OWN instruments
and added a fourth of its own, which found that the two assertions carrying
T2-a are not redundant - a point in the implementation's favour rather than a
finding.
Task 2: **I1 is a defect in the CONTROLLER's own ruling**, and it is the one
worth carrying. The ruling said the prescribed single-case form would have
"shipped green while measuring nothing" - a false-green, and an instance of the
assertion-below-a-fallback class this plan guards hardest. The reviewer
rebuilt the prescribed case against the committed implementation and measured
it: **always-RED, not false-green.** Leg 2's persisted assertion passes and its
two rendering assertions are false against correct code. The implementer's own
mid-flight gate line said so - 69 passed, 2 failed, the two new prescribed
cases among them - and the controller read past it. The mechanism was right and
the consequence invented; the ruling stands unchanged because the split is
correct either way, and the corrected reason is the stronger argument for it.
Corrected in `task-2-ruling-1.md` as a marked block with the wrong version left
standing.
Task 2: fix round 1/5 dispatched (fresh implementer, mid tier) for I1 and M1,
report-side only. The shipped code is untouched and `ea39d88` stands.

Task 2: fix round 1 returned. It corrected the inverted consequence at three
sites in the report, left each wrong sentence standing beside its correction,
and went **one case further than the verdict** on M1 - under mutation A, both
of the two locator-timeout cases pass in full once the Save locator is switched
to `en()`, so neither has any assertion that discriminates the seam, where the
verdict had named only one.
Task 2: it also surfaced two things outside its scope, both verified by the
controller at the source. **(1) The inversion is in COMMITTED SOURCE**:
`e2e/locale-switch.spec.ts`'s `EN_OVERRIDE_SETTINGS` doc comment says the
prescribed shape "would pass while measuring nothing", and cites the controller
ruling that said it - so the error propagated out of a process document into
the tree, where it outlives every scratch artifact. Fix round 2 dispatched for
it (product artifact, so a dispatched implementer and a commit, not a
controller edit). **(2) A harness trap with reach beyond this package**: `grep`
here is a shell function honouring `.gitignore`, so a rooted recursive sweep
cannot see `.superpowers/` at all and returns a false empty. The agent's first
repo-wide sweep came back clean for exactly that reason, and it caught it
itself.
Task 2: that trap was already a Tier-1 entry at count 2
(`gitignored-paths-need-command-grep`), and this is its **third strict-fit
event**, so the promotion rule fires: agent-emergent + operational promotes at
3. Moved into the always-checked layer with `tier: 2`, `promoted_at: 3`,
`count: 3` (commit `2235d7e`, ledger-lint green, entry total unchanged at 568
because the entry moved rather than being created). The controller reproduced
the trap independently before recording it - a rooted sweep returns only the
tracked hit, `command grep` returns the ignored ones, and an explicit path
fires as the control.

Task 2: delta re-review returned - **I1, M1 and the source-comment correction
all ADDRESSED**, no new breakage. The reviewer re-measured the merged form on
the shipping tree rather than accepting the fix's account of it, verified the
new comment's two checkable claims (fixture at the source, red/green by
running), and satisfied itself the one-site sweep was whole on two axes chosen
to be independent of the fix round's own four patterns - including firing its
filter against the PRE-FIX wording, which it matches. On M1 it adopted the
widening and measured it: under mutation A both locator-timeout cases pass in
full once the Save locator is switched, so neither discriminates the seam.
Task 2: **complete** (commits `0c001ee..ea39d88` for the feature, `e778dda` for
the comment correction; two fix rounds, review clean). One deferred minor: the
corrected comment's escape clause words its set as "without a reload or a
second fixture state" where fix round 1 words the same idea as "new test
infrastructure" - not wrong, since a stateful store IS a second fixture state,
but the two phrasings differ and the divergence is recorded rather than left to
be found as a contradiction.
Task 3: dispatched, mid tier, BASE `10c819c`. Controller re-measured three of
its premises first: both editor catalogs carry 46 ids today (so the recount
target of 49 is 46 + three), and both stale "45" comments exist and were
located - the second one, in `e2e/smoke.spec.ts`, does NOT contain the words my
first filter looked for, which is the memory-derived-enumeration trap this
project has a Tier-2 entry for, caught only because I noticed an expected site
missing. The recon's seed re-measurement was passed on as corroboration
explicitly NOT replacing the task's own Step 1.

Task 3: returned **NEEDS_CONTEXT** with everything built and measured except
the commit. **The plan's fenced `doSave` body does not type-check** - the
aliased `const needsPath = path === null` gives no narrowing because `path` is
reassigned in the dialog branch, so `path` stays `string | null` at both call
sites. Two fixed statements collided: "replace the body with exactly this
shape" against "the gate green before any push". Routed, not resolved; the
implementer left the tree in the failing state deliberately so the fork stayed
unresolved on disk.
Task 3: controller REPRODUCED the failure before ruling - `pnpm build` exit 2,
both TS2345 errors at the named lines, exit code captured directly. Ruled
option A (one token, `if (path === null)` at the first guard only) in
`task-3-ruling-1.md`, with both alternatives steelmanned and rejected, and a
site comment required so a later simplifier cannot unify the two conditions and
re-break the build. No plan amendment owed: the block's purpose - the
capture-before-the-await discipline and the branch structure - is intact.
Routing named explicitly as the controller's rather than the owner's: nothing
product-visible, behaviour identical under every option, and the plan's own
precedence already ranks a green gate above a fenced block's byte-identity.
Task 3: **the controller walked into a trap it had recorded an hour earlier** -
read a build's exit status through a pipeline and got the tail's `0` while the
build had failed with `2`. Caught because the error text was visible in the
output. Re-measured cleanly. This is the second live instance of
`bash-isms-run-under-bash` in one session, now at count 2.
Two more plan defects surfaced by Task 3 and ruled in the same file: the
brief says "four candidate seeds" where its own source enumerates FIVE (the
implementer ran five, a superset), and the model-tier table says "seven new
tests" where Step 7 enumerates SIX (the enumeration governs). Both are the
count-diverging-from-its-own-enumeration class.
**Cross-task constraint for Tasks 4 and 5:** Step 6's catalog placement clause
("append after its existing generic-action section") is under-determined once a
further section sits after that one. Exactly one placement satisfied both
clauses in Task 3, but Tasks 4 and 5 append under the same wording with yet
another section in between. **Their dispatches carry an explicit placement**
rather than inheriting the ambiguity.

Task 3: implementer **DONE** after the ruling, commit `2cc0650`, four files.
Controller verified on disk: unsigned, one trailer, exactly the Files-list
paths, tree clean; the diff carries the ruled deviation and nothing wider - the
first guard is `if (path === null)`, `const needsPath` and the second
`if (needsPath)` are intact, and the site comment states why the two are not
unified, located by symbol. Recount independently confirmed at 49 ids in both
locales. Full eleven-part gate green (507 Rust tests, 78 Playwright), exit codes
captured directly per command rather than through a pipeline.
Task 3: review dispatched, mid tier, with the tests named as the thing to
attack - six cases and two absence checks ship here and they will be read as the
coverage of the feature that resumes the owner's QA pass, so the question per
case is whether it can FAIL, not whether it passes.

Task 3: review returned. **Spec compliance APPROVED**, **task quality APPROVED
WITH REQUIRED FIXES**: 0 Critical, 3 Important, 7 Minor. The reviewer built and
ran 17 mutations, and answered the "can each new test fail?" question per case
with a mutation each.
Task 3: **I-2 is the owner's** and is parked with a costed memo
(`owner-decision-failed-load-empty-state.md`): after a profile fails to parse,
the editor shows "Selected profile: <path>" and "no profile open" at the same
time, and the recents list - hidden after any open before this task - returns.
Three options, each with a different user-visible loss; the deciding
measurement is that the rendered parse error carries a detail and NOT the file
path, so the path line is the only place the failing file is named. Not
blocking: Task 5 touches the same surface and is the natural vehicle.
Task 3: fix round 1 (resumed implementer) closed I-1 and I-3 plus four minors.
It found a real constraint under the false one and, while doing so, **destroyed
its own work with a revert-by-file** and caught it via `git status`; both
lessons are in the ledger.
Task 3: **the controller nearly handed the reviewer a false finding.** The first
review package was built over the task-boundary range, which contained the
controller's own ledger commit, so `docs/decision-ledger.yaml` appeared in the
task's file list - a file no task may edit. Caught by reading the package's file
list before dispatch; rebuilt from the fix commit's parent. Ledger entry
`a-controller-commit-between-task-commits-pollutes-a-range-review-package`.
Task 3: delta re-review found **D-1, a new Important defect introduced BY the
fix** - the corrected comment replaced a false ordering claim with a false
synchronicity claim, built on one mutation that varied two things at once. The
reviewer ran the counterfactuals the round had not: in the shipped order an
`await` may sit anywhere and the suite stays green, so the relative ORDER is
what is load-bearing, which the same paragraph called inconsequential. Live
trap, because Task 5 makes that function async and "order does not matter"
licenses precisely the failing configuration. Ledger:
`a-mutation-answers-its-own-question-never-the-constraint-built-on-it`.
Task 3: fix round 2 (fresh implementer) corrected it with all three
configurations plus a baseline control pasted, named the async change by its ADR
rather than by a task number so the pointer outlives the plan numbering, and
recorded in the comment that it has been wrong twice in opposite directions.
Commit `6ca7685`, comment-only, full gate green. Scoped delta sent to the
resumed reviewer.

Task 3: scoped delta returned **D-1 ADDRESSED, clean**. The reviewer ran two
FURTHER await placements beyond the three the comment states - including the
D109 shape the comment aims its reader at - both green, so four of four
placements are measured against the single failing configuration; and it noted
that the conclusion holds independently of the count, since an `await` cannot
reorder two statements, which makes await-proofness a property of the ordering
rather than a generalization from cases. Comment-only confirmed from the blob:
stripping comments, `EditorView.vue` is byte-identical across both the fix
commit and the whole task.
Task 3: **complete** (commits `10c819c..2cc0650` for the feature, `6904e4a` and
`6ca7685` for the two fix rounds; review clean). Forward note carried into the
HANDOFF, flagged by the reviewer as not a finding and not measured: making this
funnel async also makes concurrent entry possible, and the existing busy guard
does not cover a second call into it - Task 5's ground.

## Session 31 closed here

Tasks 1-3 of 7 done. Session-close gate run: journal entry appended and
committed (`902ea36`), HANDOFF rewritten and snapshotted per SI-5 (`223a14c`),
deferral sweep done (every item below has a named vehicle), no salvage owed
because no plan boundary passed. Nothing pushed - the single push is a
plan-close action.

### Close actions accumulated during execution (for the plan close)

- **ROADMAP trigger to write:** when a test needs save-then-reopen without a
  reload, build a stateful settings store into `e2e/mocks.ts`. Observable
  event, so it is a trigger and not a wish. Surfaced by Task 2's fork 2.
- **Ledger harvest candidate:** a prescribed control's FIGURE can silently
  encode a constraint nobody stated - Task 2's soundness control expects 2
  occurrences, which forbids the new doc comment from containing the token,
  derivable only from the control's own enumeration.

## Session 32 opened here (2026-07-31)

Session-start gate run. HEAD `411122f`, tree clean, 17 commits ahead of
`origin/master` (the single push is a plan-close action), the ROADMAP's Triggers
section read in full. **No trigger has fired since the last session:** the one
live candidate is Renovate's first dependency PRs, and `gh pr list --state open`
returns `[]` against a control (`--state all --limit 3`) that returns the closed
Renovate configuration PR, so the empty result is a real absence rather than a
broken command. Expected window remains 2026-08-01 to 08-03.

**The governing human is absent for the rest of this session** (stated at the
opening). Standing consequence: a question that is genuinely his gets a decision
memo written to the plan scratch and the run continues on everything not
depending on it; the run does not stop at the first fork.

**Owner ruling 1 received and recorded**: the parked failed-load empty-state
decision is ruled **option A**. Written to
`owner-ruling-1-failed-load-empty-state.md` at receipt, with the chosen option
quoted verbatim from the memo; the memo's status line supersedes itself in place
rather than being deleted.

Amendment 1 dispatched instead of a controller constraint, because the ruling
changes user-visible behaviour and needs a gate condition defined in the plan,
which is plan content the controller does not author. **One pair** (no task
added, removed or re-cut), **fresh rather than resumed** (both original roles are
in earlier sessions and cannot be resumed across the boundary; recorded as a
deviation, not a shortcut), **top tier for both roles** (the plan's tier table is
scoped to its seven implementation tasks; an amendment is a design-and-plan
four-eyes round, which the doctrine puts on the strongest model). Brief at
`amendment-1-brief.md`, BASE `411122f`.

Amendment 1: author returned DONE_WITH_CONCERNS, commit `bf857ed`. Controller
verified on disk rather than from the report: unsigned (`%G?` = N), exactly one
trailer, exactly the two tracked documents (the plan and the decisions file),
tree clean. The record landed as a new decision **D112** with supersession
pointers into two D107 clauses rather than edits to them, and the work was placed
in **Task 4**, not Task 5 as the memo had guessed.

Amendment 1: review returned. **Requirement compliance MET on all ten**, each
reproduced with the reviewer's own instruments built outside the repository.
**Quality CHANGES REQUIRED**: 0 Critical, 2 Important, 6 Minor, 3 pre-existing
observations. Both Important findings are the same family - a claim wider than
the check that guards it, and a no-work-needed conclusion whose premise the
reviewer refuted by running it (a standing lint guard for the amendment's own
declared residual costs one line on an already-installed plugin).

Amendment 1: **verdict harvest MINED at verdict arrival**, not deferred to the
close (commit `47a52ca`, `ledger-lint` green, house knowledge 571 -> 574). Two
occurrences onto existing Tier-2 entries - the tooling premise onto
`proc-no-work-needed-check` (3 -> 4), the mixed-fence report defect onto
`design-empirical-claims-reproducible` (9 -> 10) - and three new Tier-1 entries:
`a-sentence-that-characterises-an-artifact-is-a-second-claim`,
`a-normative-claim-is-scoped-down-to-its-producers-reach`, and
`a-boundary-that-forbids-an-edit-names-where-the-finding-goes-instead`. The last
is a defect in the controller's own brief, surfaced by the reviewer under the
over-restriction clause the brief itself asks reviewers to run.

Amendment 1: fix round 1 dispatched to the RESUMED author (a four-eyes artifact
in a fix loop is finished by its own author), carrying I-1, I-2, M-1, M-2, M-4,
M-5, a confirmation duty on M-3 and M-6, and **one controller-authorized scope
extension**: the Task-4 cases that build history and then click Open will meet
Task 5's confirm dialog, and Task 5's exhaustive Files list forbids it from
repairing the file it breaks. Folded into this amendment rather than raised as a
second one - plan content, no task added, removed or re-cut, the author holds the
context, and leaving it costs a certain NEEDS_CONTEXT at Task 5's dispatch.
Controller ruling inside I-2, so the author is not deciding it: the lint guard
lands in Task 4 rather than as a close action, on
`tests-ship-with-the-feature-never-after` and `proc-proposed-safeguard-stays`. The
reviewer's measurement behind it is borrowed and the author re-derives it before
prescribing anything from it.

Amendment 1: fix round 1 returned, commit `7c80c40`. Controller verified on disk:
unsigned, one trailer, exactly the two tracked documents, tree clean. **Note for
the delta package's base:** the controller's own ledger commit `47a52ca` sits
between the round-1 head and this fix, so the scoped package was built from
`47a52ca` rather than from `bf857ed` - the trap
`a-controller-commit-between-task-commits-pollutes-a-range-review-package`
records, caught this time before the dispatch rather than after.
The author re-derived the borrowed lint measurement with its own probe, enumerated
the scope extension's affected set from the plan's step text and landed on the
reviewer's two rather than its own three, and swept three further corrections its
edits forced. Delta re-review dispatched to the RESUMED original reviewer, with
the two new-content items flagged as needing full judgment rather than an
ADDRESSED verdict.

Amendment 1: delta re-review returned - **all eight findings ADDRESSED**, and both
new-content items judged SOUND on their own merits rather than as fixes. The
reviewer executed the plan's fenced lint rule by extracting its characters and
running them inside a copy of the repo's real config, so the fenced text is
measured to be the text that works; it walked the scope extension's criterion over
every Task-4 case independently and landed on the same two; it re-derived all three
sweep corrections; and it ran one scope check the plan asserts and never measured
(the rule is file-scoped in its figures but bound to the repo-wide lint run, and
produces those two hits and nothing else across all 24 tracked `.vue` files).
**New breakage: 0 Critical, 0 Important, 4 Minor.**
Amendment 1: delta harvest mined at arrival (commit `85902c7`, `ledger-lint` green,
entry total unchanged at 574 because an occurrence was appended rather than an
entry created): the reach defect survived one notch narrower - the prescribed
rule's configuration enumerates two render directives and the prose about it names
the wider category, leaving a third outside. Occurrence on
`a-normative-claim-is-scoped-down-to-its-producers-reach`, 1 -> 2.
Amendment 1: **complete** (commits `bf857ed` and `7c80c40`; one fix round, delta
clean). One pair, fresh rather than resumed, both roles top tier.

Task 4: dispatched, mid tier, BASE `85902c7`. The dispatch carried three things
the extracted brief cannot know: the shipped `createBlank` comment as the source
of the measured ordering constraint (rather than the plan's prose, which is wrong
about it), an **explicit catalog placement** for the two new ids - immediately
after `editor-action-remove` inside the `## Generic list/map actions` section of
both catalogs, since a later section now exists and the plan's wording alone no
longer picks a unique spot - and the validation-response race as a named RISK
with an instruction to return NEEDS_CONTEXT if the failed-open case proves flaky,
never to work around it.

Task 4: implementer DONE_WITH_CONCERNS, commit `1092eb7`, seven files. Controller
verified on disk: unsigned, one trailer, exactly the Files-list paths, tree clean.
Full eleven-part gate green (507 Rust tests, 93 Playwright, 14 of them new).
**Two of its four concerns did not reproduce for me as reported**, and both went
to the reviewer with my own run pasted rather than as an assertion: the broad
pre-state expression the plan calls "already measured: 0 lines" returns **3** on
the task's actual base commit where the implementer reported 2, and the
keyboard-resolved lint decision was verified to have the precedent it cites.

Task 4: review returned. **Spec compliance MET** - every fenced value extracted
programmatically and matched byte for byte. **Task quality: 5 findings**, 1 high,
2 moderate, 1 low-moderate, 1 low, none blocking the code. The reviewer built its
own mutations rather than re-running the implementer's.
Task 4: the highest finding is a coverage claim, not a code defect: deleting the
whole save-marking line leaves the build and all 93 cases green, because the value
it feeds has no consumer inside this task. The plan already assigns that property's
first producer to Task 5, so no test is owed here - what was wrong is the residual
paragraph, which named only the hardest uncovered scenario and let the easy one
read as covered.
Task 4: the second is real coverage work - the failed-open case asserts through a
button whose disabled binding is a disjunction, so the no-model term satisfies it
and a mutation that leaves the history uncleared still passes.
Task 4: verdict harvest MINED at arrival (commit `3be0d32`, `ledger-lint` green,
575 entries). One **PROMOTION to tier 2** on its third strict-fit event -
`a-normative-claim-is-scoped-down-to-its-producers-reach`, agent-emergent plus
process, `promoted_at: 3` - the entry opened by this session's amendment review,
earned its second occurrence in that review's delta, and its third here, cited
against itself by the reviewer. Plus one new tier-1 entry,
`a-disabled-assertion-over-a-disjunction-proves-only-its-weakest-term`.
Task 4: fix round 1 dispatched to the RESUMED implementer (the work needs the held
context of the spec file and the harness, not a written-out line edit). Two
controller rulings travelled with it: on finding 2, defeat the fallback if an
observation path exists and otherwise scope the test's name and claim down to what
it covers, never weaken an assertion; on finding 3, the owner-ruled
`tests-ship-with-the-feature-never-after` governs over the step list's silence, so
the missing case for behaviour this task introduces is added here.

Task 4: fix round 1 returned, commit `06e7a61`, one file (`e2e/editor-undo-redo.spec.ts`
only; the view is byte-identical to the task commit). Controller verified on disk:
unsigned, one trailer, tree clean. On finding 2 the implementer tried the stronger
route first, reports measuring that no observation path exists outside the two
model-dominated bindings, and only then took the licensed scope-down; on finding 3
it added the missing funnel case and verified rather than assumed that the case is
not behind the same disjunction; on finding 5 it re-measured instead of adopting my
figure and landed on 3 independently.
Task 4: the implementer self-reported a process trap - a build from deliberately
broken source left a stale `dist/`, and the restore that followed did not rebuild,
so a later full-suite run measured the broken bundle. **Already a ledger entry**
(`frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`), so an occurrence
was appended rather than a duplicate entry opened, 1 -> 2, with the sharpening this
instance adds: the entry reads as a false-GREEN rule and the same mechanism produces
a false RED, because a restore is an edit too. Mechanism re-verified by the
controller at `playwright.config.ts` rather than taken from the report (commit
`2e0165f`).
Task 4: scoped delta sent to the RESUMED original reviewer, with the fix's own
load-bearing negative named as the thing to attack - the licence for the scope-down
was conditional on there being no observation path, so if one exists the finding is
not addressed - plus a required confirmation that the delivered state builds and
passes from its own committed source rather than from a leftover artifact.

Task 4: delta re-review returned - **findings 1, 2, 3 and 5 all ADDRESSED, zero new
breakage**. The reviewer re-derived finding 2's load-bearing negative independently
rather than checking the trace it was given, and confirmed the delivered state by
deleting `dist/` outright, rebuilding from the committed source and running the full
suite (94/94) - the one check that makes a green run mean anything after a session
of mutation testing.
Task 4: one new moderate, non-blocking: the fix report's evidence for that negative
pastes a `command grep` invocation and reports it returns nothing, where run
verbatim it returns 27 lines inside the gitignored vendored bundle. The plain
`grep` form does return nothing, so the reported result belongs to a different
command than the one written down. Conclusion survives, evidence does not
reproduce. Harvested as an occurrence on `design-empirical-claims-reproducible`
(10 -> 11, commit `3f48008`) rather than a new entry, since that statement already
forbids exactly this form.
Task 4: **complete** (commits `85902c7..1092eb7` for the feature, `06e7a61` for the
fix round; one fix round, delta clean). One deferred minor below.

Task 5: dispatched, mid tier, BASE `3f48008`. The dispatch named the highest-value
item the brief cannot know: Case 3 leg (ii) is the FIRST and ONLY producer anywhere
in this plan for the save-marking property, since Task 4's reviewer measured that
deleting the whole mark line leaves everything green - so the dispatch required a
deliberate break of the marking mechanism and a pasted red run, rather than a green
leg. It also carried the explicit catalog placement (a new section at the end of
both files, since Task 4's ids went inside the existing generic-action section),
the async-`createBlank` concurrency note as ground rather than as work, and the
three environment traps including the build-before-e2e rule.

Task 5: returned **NEEDS_CONTEXT** with everything built and 10 of 11 gate parts
green, the commit deliberately withheld because the gate is not. It found a genuine
collision between two of the plan's own statements and did not resolve it: Step 2
guards `createBlank` with no exceptions, while Step 4b's repair set is fixed at two
cases under a criterion scoped to a second **Open** click - and a third case in the
same file clicks **New** while dirty, so it now hangs on an unanswered confirm. The
implementer derived that from the tree, left the third case untouched, reproduced
the failure in isolation and wrote a costed memo. Exactly the behaviour the latitude
ban asks for, and the fourth task in a row to do it.
Task 5: it also ran the leg-(ii) mutation as instructed and reports the marking
break reddening **only** that leg, with the restore rebuilt and verified by asset
hash - the discrimination the whole property depends on.
Task 5: **the root cause is the controller's.** Amendment 1 drew that enumeration
correctly; the controller then ruled a third case into Task 4's fix round and did
not revisit the enumeration that counted the set. Recorded as the writing half of
the new ledger entry `a-normative-sentence-naming-a-set-is-discharged-member-by-member`
(commit `7915e5d`, 576 entries, lint green).
Task 5: a second, independent defect surfaced by the same implementer and verified
by the controller at the artifact: `src/views/EditorView.vue`'s catalog-budget
comment still reads 49. **Task 4's Step 7 named that file and `e2e/smoke.spec.ts`
together; only the second was corrected**, and Task 4's review and delta re-review
both graded spec compliance MET. That is the grading half of the same ledger entry.
Task 5: routed as **amendment 2** rather than as a controller ruling over fenced
plan text - one pair, the resumed author and, after it, the resumed reviewer. The
brief asks for the CRITERION to be repaired rather than the count, so the next
addition does not regenerate the defect, and folds the stale-comment ownership
question into the same vehicle. The dispatch carries a sharpened boundary: Task 5's
uncommitted work is in the tree, so no stash, no checkout, no reset, no path outside
`docs/`, pathspec-scoped commit only.

Amendment 2: author returned DONE_WITH_CONCERNS, commit `dceca9b`, the plan document
only. Controller verified on disk: unsigned, one trailer, one tracked file, **and
Task 5's uncommitted work still present and unstaged** - the pathspec discipline
held under the one-tree, two-writer condition the doctrine warns about. Step 4b's
membership rule now derives from the two functions Task 5 guards rather than from
one entry control, which yields three cases; the added member takes the identical
repair; the stale budget comment is placed into Task 5 as a fenced step, with Task
4's half-discharged step marked rather than rewritten.
Amendment 2: the author reported two instrument defects of its own rather than
papering over them - a search pattern that missed the very site the brief named,
because the comment writes the filename in backticks and the pattern demanded a
bare space, and an `&&` chain in which a zero-match `grep -c` short-circuited the
rest, so a later check never ran and its silence read as a pass. Both re-run
soundly. Harvest at the verdict's arrival.
Amendment 2: **its concern 1 is itself under-enumerated, which is the third instance
of this session's newest rule inside one hour.** It reports one further site
carrying a stale catalog-key figure; my own wider search returns four sites carrying
a 43-figure claim, two in editor widget files and two in `src/editor/registries.ts`,
and they do not all count the same set - the widget comments speak of catalog keys,
the registry comments of a field table. **No claim that any of them is wrong**:
what is owed is a per-site measurement against the set each figure actually counts.
Out of every Files list in this plan, so it is routed as a close item and the
reviewer's independent count decides what goes into it.
Amendment 2: review dispatched to the RESUMED reviewer, graded as a new amendment
rather than as a fix round, with the criterion-versus-list question named as the
thing to attack and the same read-only, do-not-touch-the-dirty-tree constraint.

Amendment 2: review returned. **B-1, B-2, B-3, B-4 and B-6 MET; B-5, the sweep, NOT
MET.** The reviewer ruled explicitly on the question the brief named: the criterion
**derives** rather than lists - applied cold it lands on the same three members,
keys on the mandate rather than on a control, catches keyboard activation by
construction and correctly excludes a recents click. It also re-enumerated the
activation sites with a wider search than the author's and found the same eight.
**2 Important, 3 Minor**, both Importants consequences of the missed sweep: two
sentences describing the set's sensitivity without naming a member were falsified
and no term-derived sweep could see them; and the rewrite of one step deleted the
only live instruction for the SIBLING file's budget comment - the same ambiguity
the amendment was raised to remove, reproduced on the other half of the pair.
Amendment 2: harvest mined at arrival (commits `aab03dc` + `e0c606a`, lint green,
577 entries). One new entry,
`an-edit-to-a-set-walks-to-its-neighbours-not-only-to-its-enumerations`, carrying
both Importants as its two occurrences. **Deliberately NOT appended to the
neighbouring entry from an hour earlier**, which would have taken it to three and
promoted it into the always-checked layer on a stretched reading: that entry is
about enumerations, this one about sentences with no shared vocabulary, and the
statement-fit gate says open a narrow entry rather than walk an existing one toward
its threshold.
Amendment 2: fix round 1 dispatched to the resumed author, carrying both Importants,
the three Minors, and one controller ruling rather than a routing - the criterion
must state what it depends on and the event that would shrink the guarded set must
point back at it, because leaving that open means writing the same finding again
next round.

Amendment 2: fix round 1 returned, commit `d4c8401`, two documents. Controller
verified on disk: unsigned, one trailer, **Task 5's six uncommitted paths intact
before and after**. The author ran the reviewer's three sensitivity phrases rather
than only its own term-derived expressions, rewrote the falsified clause in BOTH
documents with the original quoted and marked, gave the sibling budget comment a
live end-state fence and a single named owner, and closed the controller ruling
with a reciprocal pointer. It flags in its own concern 4 that the pointer is prose
at both ends and that nothing turns red if a future edit updates only one side -
the honest answer, since a prose statement cannot guard its own truth, and it is
named rather than covered.
Amendment 2: scoped delta sent to the resumed reviewer, with the pair-not-half
question named for each finding and one instruction that decides the B-5 re-grade:
a sweep that finds what the last review named is not evidence that nothing else was
falsified by this round's own edits.

**Convergence check, run deliberately rather than after a third round** (doctrine
§4): the amendment loop widened once - a claim wider than its check, then a sweep
blind to neighbours - and the response was to repair the CRITERION rather than to
route more members, which the reviewer then confirmed derives the set cold. That is
the prescribed answer to the convergence question, taken before the second widening
rather than after. If this delta returns another sweep miss one level wider, the
defect is the written sweep rule and the next dispatch asks what that rule is
MISSING as a single clause, not for more sites.

Amendment 2: delta re-review returned - **all five findings ADDRESSED and B-5
re-graded MET**. The reviewer checked the fence form against BOTH tree states rather
than against the author's argument (the fenced lines present verbatim in the
resumed working tree at 54, absent at HEAD at 51, so a replacement fence would have
matched neither), and judged the sweep "wider, not complete" while ruling that
holding B-5 open would move the bar, because the two escapees are a different class
no expression can express.
Amendment 2: **1 new Important and 2 Minors introduced BY the fix, and they are one
shape the reviewer named as a single missing clause rather than as more sites.** The
round that closed the missed-neighbour class produced two instances of its twin by
hand: the rewritten clause disagrees between the plan and the decisions document
about which claims were falsified, with the ADR form self-refuting; and one of two
sibling pointers was repointed while the other still points at a step that now
disclaims instructing. A sweep answers "what else says this"; only a pair
comparison answers "do the two things I just wrote still agree".
Amendment 2: harvest mined at arrival (commit `74b4efe`, lint green, 578 entries):
new entry `a-statement-living-in-two-documents-is-diffed-against-its-twin-not-swept`,
opened at count 1 with both instances in one occurrence, per the house precedent
that several instances surfaced by one review of one authoring pass are one event.
Amendment 2: fix round 2 dispatched, scoped to the three, with the METHOD stated as
the substance - compare each pair side by side, and then do the same for every other
statement this amendment put into both documents, reported as its own step rather
than as part of a sweep. **The convergence question is answered and not re-asked:**
the reviewer supplied the missing clause itself, which is the outcome doctrine §4
prescribes, so this round routes a rule and not a longer list.

Amendment 2: fix round 2 returned DONE, commit `9d95c5e`, five changed lines.
Controller verified on disk: unsigned, one trailer, two documents, Task 5's six
uncommitted paths unchanged through staging and commit.
**The pair-check rule paid for itself in the round that introduced it.** Two facts
worth carrying beyond this plan: the corrected case-insensitive expression returns
**three** sites where the previous round's union had carried the third by luck; and
the side-by-side comparison across eleven statement pairs - inventory derived from
the author's own commit hunks rather than from recall - found an eleventh defect no
sweep had produced, a rejected alternative citing a decision by NUMBER where each
number is correct inside its own document and the pair contradicts itself to anyone
holding both. Both now name the decision by description.
Amendment 2: final delta dispatched, with two questions that are not fix grading -
whether anything earlier rode on the one-member-short expression, and whether an
eleven-pair inventory derived from four commits' hunks is COMPLETE, since
completeness of the pair inventory is the very property this round exists to
establish.

Amendment 2: final delta returned - **N-1, N-2, N-3 all ADDRESSED, 0 new breakage,
released**. Two answers beyond fix grading. The three-site figure has **no
retroactive effect**, checked two ways: inside the searched surface all three sites
are the sentences the previous round repaired, so the blindness cost redundancy and
not coverage; and because a case fix does not close the instrument's other half, the
reviewer ran the corrected phrases over all of `docs/` and found two further
sensitivity-shaped sentences, both about a different decision, one live-but-unaffected
and one a dated snapshot. The **pair inventory is complete**, and the reviewer
measured the generator's blind spot rather than accepting the report's assertion: an
inventory built from one document's hunks cannot see a change made only on the other
side, and the one candidate commit has zero hunks in the twinned region.
Amendment 2: **complete** (commits `dceca9b`, `d4c8401`, `9d95c5e`; two fix rounds,
delta clean). Harvest mined at arrival (commit `9ee3b9e`, lint green, 579 entries):
the pair-check entry gained its measured weak point in the statement itself plus a
`reinforced` occurrence, and one new entry,
`a-cross-reference-between-independently-numbered-documents-is-a-description`.

Task 5: released. The brief was **regenerated** rather than left stale - the copy the
implementer was dispatched with has a two-case Step 4b and no Step 4c, and a resumed
agent reads the path it was given. Ruling relayed with the cause named as the
controller's, not as a plan defect the implementer should have caught. The reviewer
had already verified executability against the exact tree being resumed into: the
remaining work is two fenced edits plus one case repair, each under a named step.

Task 5: implementer **DONE** after the amendment, commit `0b00262`, six files.
Controller verified on disk: unsigned, one trailer, exactly the Files-list paths,
tree clean, full eleven-part gate green (100 e2e cases). **Both budget comments
checked individually by the controller at their own sites** - 54 in each - rather
than by asking whether the step ran, which is the exact failure Task 4's two review
passes made.
Task 5: review returned. **Spec compliance MET**, requirement by requirement, with
both halves of the two-site step independently checked and the catalog count
re-derived. **0 Critical, 2 Moderate, 1 Low.** The reviewer reproduced the
acceptance-critical mutation itself: breaking the save-marking mechanism reddens leg
3(ii) and nothing else, which is the first time in this plan that property has had a
producer at all.
Task 5: **the reentrancy finding is deferred with a trigger, not dropped, and that is
a controller decision.** The confirm component's `ask()` has no reentrancy guard; the
reviewer reproduced the theft against the real bundle and also established that no
real user input reaches it, because the native modal blocks real clicks and only a
script-level call bypasses hit-testing. Building a guard is product behaviour this
plan does not prescribe, so it would be latitude. Event: the arrival of a SECOND
caller of that component - which its own doc comment names as its reason to exist,
so the trigger is written into the artifact rather than into someone's memory. Goes
on the tracker at the plan close.
Task 5: verdict harvest MINED at arrival (commit `5ba09e3`, lint green, 581
entries). Two new tier-1 entries:
`a-concurrency-note-asks-whether-the-function-defends-itself-not-whether-callers-are-safe`
(the general form of the reentrancy finding - a reachability answer expires when a
second caller appears) and
`a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact` (a shipped comment
cites a sibling as documenting something it does not; the premise came from the
brief's own Read-first line, so it was inherited rather than invented).
Task 5: fix round 1 dispatched to the resumed implementer - the Esc coverage gap and
the false citation - with item 3 explicitly withheld as the controller's and the
instruction to restate it as an accepted deferred residual rather than to soften it.

Task 5: fix round 1 returned, commit `57bcc41`, two files. Controller verified on
disk: unsigned, one trailer, tree clean. The Esc case was attacked by preventing the
dialog's native cancel event and reports reddening only itself; the false citation now
names a decision in the plan's own decisions document. **Controller check at the
artifact rather than at the report:** the shipped comment carries the file without a
line number, so the citation form conforms to the house rule - the line number the
implementer put in its message to me is not in the code.
Task 5: scoped delta sent to the resumed reviewer, with three questions beyond fix
grading - whether the chosen mutation breaks the mechanism the case protects or a
neighbour of it, whether the re-citation says what the comment claims (repeating the
defect one document over would look identical), and whether the deferred reentrancy
finding was restated at full strength rather than shaded into harmlessness, since a
deferral that rewrites its own finding is how a real defect disappears.

Task 5: delta re-review returned - **all three findings CLOSED, 0 new breakage**.
The reviewer did not settle for the fix's own mutation: where the fix proved its Esc
case by suppressing the dialog's native cancel event, which shows only that Escape
does something, the reviewer attacked the DIRECTION - the close handler resolving as
a confirm rather than a cancel - and that reddened the new case plus two downstream
ones. It also read the re-cited decision itself and found the quoted sentence word
for word, and ruled the deferred reentrancy finding restated at full strength with
every load-bearing qualifier intact.
Task 5: harvest mined at arrival (commit `ed1a635`, lint green, 582 entries): new
entry `a-fire-test-on-a-two-direction-surface-attacks-the-direction-not-the-presence`.
Task 5: **complete** (commits `9ee3b9e..0b00262` for the feature, `57bcc41` for the
fix round; one NEEDS_CONTEXT that produced amendment 2, one fix round, delta clean).
One deferred item with a trigger, below.

### Deferred with a trigger, Task 5 (write to the ROADMAP at the plan close)

- **The confirm component's `ask()` has no reentrancy guard.** Reproduced by the
  reviewer against the real shipped bundle: a second, non-hit-tested entry silently
  steals the confirmation from the first caller. Unreachable by any real user input
  today, verified down to browser hit-testing - the native modal blocks real clicks
  and only a script-level call bypasses it. **Event: a second caller of that
  component is added.** The trigger is written into the artifact rather than into
  anyone's memory, because the component's own doc comment states it exists so a
  second caller can reuse it. Building the guard now would have been product
  behaviour this plan does not prescribe.

Task 6: dispatched, mid tier, BASE `ed1a635`. The dispatch carried Task 1's deferred
minor M-6 VERBATIM as a cross-task constraint (the non-literal call site in the shell
file's own test module, which this task's key-set derivation was bound to meet), the
enumeration-governs rule, and the four environment traps, since this is the first
task whose prescribed red states mutate TRACKED files. No catalog-placement ruling was
needed or given: unlike the editor catalogs, the shell catalog's block is followed
immediately by a section heading, so the plan's own "after the close-abort block"
fence picks a unique spot - checked before deciding not to rule.

Task 6: implementer **DONE**, commit `a47fc19`, nine files. Controller verified on
disk: unsigned, exactly one trailer, exactly the Files-list paths, tree clean, nothing
pushed. Full eleven-part gate green.
**The harness security monitor flagged the subagent's commit**, which is the known
false alarm SI-4 records: the owner's standing commit grant for this repository is
invisible to that monitor. Handled per SI-4 - the commit's content and scope were
verified on disk, the false alarm is named, nothing was reverted.

Task 6: review returned. **Spec compliance MET** across all eleven steps, every
fenced block byte-checked, and all three prescribed red states reproduced from a clean
out-of-repo copy byte-matching the report's pasted failures. **0 Critical, 2 Moderate,
2 Low.**
Task 6: **the sharpest finding is a PLAN defect, not the implementer's.** The
acceptance row asserting that confirming a discard-only close quits and cancelling
does not is marked machine-verified, and no such test exists or can exist - the row
directly below it states in its own text why that surface cannot be unit-tested, for
the same mechanism and the same reason. Routed as a close-time text correction rather
than a third amendment: the row is descriptive, it changes no work, and the plan's own
family has the precedent of routing text corrections to the close. The observable
itself rides the owner's existing 1.x GUI-test-harness item, which is where Tauri
mock-app infrastructure belongs by his 2026-07-28 ruling. **Carried explicitly into
the whole-branch review brief**, so the acceptance walk is not misled by a row I now
know to be wrong.
Task 6: the second Moderate is a false claim in shipped source - a failure-cost comment
asserting a failed sync is never shown where nothing is at risk, which is false in one
direction for a symmetric retry-less flag. Routed to the implementer.
Task 6: **both of the implementer's disclosed test-content changes were measured
against the latitude boundary and land on OPPOSITE sides of it** - rebinding probe
keys from literals to locals is inside the grant, changing an existing case's typed
fixture value is outside it and should have returned as a fork. Both repairs correct,
both disclosed, so the code stands; recorded as the house's standing example pair
because the two are easy to conflate at the keyboard.
Task 6: verdict harvest MINED at arrival (commit `011fb96`, lint green, 583 entries).
An occurrence onto the now-tier-2 `a-normative-claim-is-scoped-down-to-its-producers-reach`
carrying a cheap new tell - where one row in a map explains its own non-verifiability,
walk every row sharing its mechanism - an occurrence onto
`latitude-carveout-zero-content-structural-forks` (14 -> 15) carrying the example pair,
and one new entry,
`a-failure-cost-comment-does-not-inherit-its-neighbours-guarantee`.
Task 6: fix round 1 dispatched to the resumed implementer for the two items that are
its own, with items 3 and 4 explicitly withheld as the controller's.

Task 6: delta re-review returned - **both routed findings ADDRESSED, 0 new breakage**,
and the reviewer re-derived both failure directions from the mechanism rather than
reading the corrected sentence for plausibility. It also confirmed the fix did NOT
over-correct onto the sibling field, whose absolute rests on a different mechanism
and genuinely holds. Harvest: that inverse failure is now a clause in
`a-failure-cost-comment-does-not-inherit-its-neighbours-guarantee` with a reinforcing
occurrence (commit `78e968f`).
Task 6: **complete** (commits `ed1a635..a47fc19` for the feature, `3caa87f` for the fix
round; one fix round, delta clean).

Task 7: dispatched, mid tier, BASE `78e968f`. The dispatch named the one thing the
brief could not: this task documents shipped behaviour, so every sentence is checked
at the SURFACE rather than against the plan's description of it - and this plan's own
prose has been wrong about its own tree repeatedly. Also carried the help-and-catalog
rebuild trap, which is exactly this task's file set.
Task 7: implementer **DONE**, commit `7b403e8`, four files, full eleven-part gate green.
Task 7: review returned. **Spec compliance MET** on every requirement, both catalog
replacements byte-compared, the help-hygiene absence check reproduced with its fire
control. **2 Important, 2 Minor.**
Task 7: **the first Important is the SECOND acceptance row in this plan naming a
producer that does not exist.** The row says the reworded batch empty-state string is
asserted in the existing batch scenario; that id appears in zero test files.
**Controller reproduced it with a control before routing** - a sibling id from the
same catalog block returns 23 hits in the same spec file, so the empty result is a
real absence and not a broken pattern. Routed into the **whole-branch review's fix
wave**, which is not bound by a task's exhaustive Files list, rather than into a third
amendment or a breach of Task 7's scope.
Task 7: the second Important is a German terminology defect the English side is
structurally unable to show - the new German sentence reuses a word the same file uses
eighteen lines later for a different concept, where the corpus term is another word,
and English uses one word for both concepts. Routed to the implementer.
Task 7: verdict harvest MINED at arrival (commit `c570b4c`, lint green, 584 entries).
An occurrence onto the tier-2 `a-normative-claim-is-scoped-down-to-its-producers-reach`
(4 -> 5, its fifth in this plan alone) and one new entry,
`an-acceptance-row-naming-an-existing-producer-is-verified-by-finding-it`, which names
the asymmetry both instances share: a row naming a producer the plan will BUILD is
self-correcting, a row naming an EXISTING one is believed because it describes the
past and nothing in the run ever opens it.
Task 7: fix round 1 dispatched, three items, with the acceptance-row gap explicitly
withheld and one sentence owed to correct where the report repeats the false claim.

Task 7: fix round 1 returned, commit `411f220`, two lines. Delta re-review returned
**all four findings addressed, 0 new breakage** - the corpus term confirmed at four
independent sites, the English label byte-exact against the catalog with its naming
convention verified real in a sibling topic, and the swapped citation's unit test
reproduced byte for byte rather than accepted.
Task 7: **the delta found one more, and it is about a SURFACE rather than a pattern.**
The fix round reported checking its whole German addition term by term; the reviewer
derived the term list from the DIFF instead of from that list and found an entire
second paragraph of the same addition never looked at - which is where a second real
collision sits, a word for which the corpus already has an established term used in
two shipped catalogs. Four of that paragraph's five content words are clean. Occurrence
on `proc-sweep-surface-completeness` (9 -> 10, commit `43c1a44`): a firing control
proves a sweep's pattern valid, never that its search surface is complete, and this
sweep was anchored on the previous finding's location rather than on the extent of the
change.
Task 7: fix round 2 dispatched, resumed implementer - the one word, and the sweep
redone with its term list DERIVED from the diff, with the previous round's claim
corrected in place because as written it asserts something the run did not establish.

Task 7: fix round 2 returned, commit `87a07e8`, one line. Its widened sweep - term
list derived from its own original diff across both help files - surfaced **two
further terminology findings of a DIFFERENT class**: not one word meaning two things
(collision), but the corpus already having its own word for the thing (synonym), a
German verb and its English mirror both naming an affordance the shipped catalog
already names otherwise. Reported, deliberately not fixed, scope respected.
Task 7: **the convergence rule fired and was applied rather than noted.** Three rounds
had returned the same shape one class at a time - a collision, a second collision in
an unswept paragraph, then two synonyms. Doctrine §4 says two consecutive rounds
returning the same finding one level wider mean the defect is the unwritten rule, not
the artifact, and that the dispatch question changes: state what the rule is MISSING
as a single clause rather than list further members. Done, as
`a-new-term-is-checked-against-the-corpus-in-both-directions` (commit `50ae53f`, 585
entries): a new term owes a search for the WORD (collision) and a search for the
CONCEPT (synonym), and the second is the one that survives review because nothing
contradicts and each sentence reads fine alone. **The two concrete instances go to the
whole-branch fix wave as work**, so the rule is written AND the members are fixed -
neither substitutes for the other.
Task 7: final delta sent, asking the reviewer to compare derived term SETS rather than
grade the conclusion, to confirm or refute the synonym classification at the corpus,
and to say whether any further synonym instance exists that neither of us has named -
that last answer is what decides whether the wave's list is complete.

Task 7: final delta returned - **both items closed, 0 new breakage, task CLOSED.** The
reviewer derived the content-word list itself and diffed SETS rather than grading the
conclusion: the table omits a handful of generic verbs, every one of which it then
checked against the corpus itself, all clean, one of them positively matching an
existing precedent the table simply did not cite. Its ruling on the sweep is the one
that matters - round 1's gap was structural (a whole paragraph never read) and that is
closed; what remains is a slightly overstated framing, not a blind spot.
Task 7: the synonym classification is **confirmed at the corpus**, and the reviewer
stress-tested the most promising candidate for a third instance and found it resolves
clean - different referent, not a synonym. **The wave's list of two is complete**,
which was the question that decided it.
Task 7: **complete** (commits `78e968f..7b403e8` for the documentation, `411f220` and
`87a07e8` for two fix rounds; two fix rounds, delta clean).

**All seven tasks of Plan 12 are closed.**

## Whole-branch review (2026-07-31)

Dispatched on the top tier over `bd3aa34..50ae53f`, 48 commits, with the brief naming
the three things only it could do and stating that 13 of those commits touch product
files so process commits are not read as task work.

**Verdict: NEEDS_FIXES. 0 Critical, 7 Important, 7 Minor, eight blocking items.**
Nothing on the branch was broken; what was missing was coverage in the direction that
loses data, plus four normative statements that no longer described the tree.
**Five of the eight blocking items were MISSING PRODUCERS** the acceptance map claimed
existed. Two of the four false rows were already known and routed by the controller;
the reviewer found two more, and both of those name producers the plan would BUILD.

**It refuted a ledger entry the controller had written six hours earlier.** That entry
claimed the asymmetry runs between rows naming an EXISTING producer and rows naming
one the plan would build, the second kind being self-correcting. Measured false: the
four false rows split evenly across it. Statement rewritten to the reviewer's rule -
a row is protected by the STEP LIST that names its producer, whatever tense it is
written in - with the refutation recorded as a dated occurrence and the original
wording left in git history (commit `5cabf32`).
Two direction failures on this branch shared one shape, which is the review's sharpest
harvest: **where a decision is factored out of an untestable caller for testability,
the mapping from that decision to its effect is a SECOND decision and has to move out
with it.** Testing the decision and leaving the mapping behind is how a dirty editor
with no run could be asked about aborting jobs and lose the profile unmentioned.
Its acceptance walk covered **73 rows, 4 with a gap**, and it opened every row naming
an existing producer and ran its mechanism. It also found all four claims of the map's
pre-table prose note defective.
Of 20 deferred and parked items it ruled **18 stand, 2 need work**, and it re-derived
rather than borrowed on the ones that mattered - including confirming the validation
race is genuinely pre-existing at this branch's base rather than widened by it.

Fix wave: ONE implementer, all eight blocking items plus the recommended lint pull-in,
six pathspec-scoped commits `991ea7c`, `4bfca22`, `b8b4250`, `cbe3895`, `9c860d3`,
`d2b622a`. Controller verified all six on disk: unsigned, exactly one trailer each,
tree clean. 103 e2e cases (two new), 88 Rust unit tests in the GUI crate (two new),
full eleven-part gate green.
**The wave built the held-open mock the plan had deferred as new test infrastructure**,
because the save-in-flight producer could not be written without it - so Task 4's
recorded residual is closed by construction rather than carried.
Scoped re-review dispatched to the resumed whole-branch reviewer, with three rulings
demanded rather than passes: on the new shared test infrastructure as infrastructure,
on the fixer's measured deviation from the verdict's own suggested shape, and on the
sibling document two agents concurred owes nothing - a concurrence not being a
measurement.

## Plan close (2026-07-31, session 32)

**Whole-branch fix re-review: READY.** 0 Critical, 0 Important, 3 Minor, all three in
the new test helper's own contract and all unreached. Every one of the nine findings
re-verified with the reviewer's OWN mutations rather than the fixer's, and the two
checks a wave most often fails came back clean: zero assertion tokens in the complete
removed-line set with a firing control, no skips or timeout changes, counts moved up
only (Rust 86 -> 88, e2e 101 -> 103); and the plan document is untouched by both the
range and its base, so no acceptance row was edited to match a producer that is not
there. Gate on the delivered state green, e2e run three times.

**Three things from that re-review that correct the record rather than the code:**

1. **The controller's own claim was wrong and is corrected here.** This session's
   ledger said the fix wave "built the held-open mock the plan had deferred as new
   test infrastructure", closing Task 4's residual by construction. The reviewer
   refuted it: Task 2's fork 2 deferred a **stateful settings store**, which is a
   different thing and **stays on the tracker untouched**. What the wave built is a
   gated, deterministically released IPC response, which no entry had deferred. Both
   the earlier line and this correction stand, dated.
2. **The reviewer ruled against its own remedy, measured.** Its verdict had proposed a
   specific extraction shape for the close-dialog mapping; the fixer deviated with a
   measurement, and the reviewer then BUILT its own proposal and ran an existing
   guard's prescribed red state under it - 88 passed, where the shipped shape gives 87
   passed and 1 failed with the expected message. Its own shape would have silently
   disabled that guard, because moving the message lookups out of the function removes
   them from a source-literal scan another test derives its key set from. Ledger:
   `a-remedy-preference-in-a-verdict-is-measured-or-stated-without-the-comparative`
   (1 -> 2).
3. **The reviewer's own typography instrument had been broken**, and it said so
   unprompted: its bad-glyph set was held as literal characters, one had normalised to
   a plain space, and in the FIRST whole-branch verdict that detector reported the
   branch clean without ever having been shown able to fire. Rebuilt from code points
   with a self-test proving all eleven glyphs detectable; both passes then came back
   genuinely clean. New ledger entry
   `a-glyph-denylist-is-built-from-code-points-and-self-tested-before-it-scans`
   (commit `8512e77`, 586 entries).

### Roll-up funnel - every deferred, parked and minor item classified

No item ends silently in the frozen archive. Twenty items went to the whole-branch
review for triage; it ruled 18 stand and 2 need work, and both of those were fixed in
the wave. Classification:

- **fixed in the wave** (7): the batch empty-state producer (I-1), the settings
  locale-hint producer (I-7), the keyboard-spelling producer (I-2), the save-marking
  direction producer (I-3), the close-decision-to-effect mapping producers (I-4), the
  create-affordance terminology at all five sites (I-5), the plan-7.5 design doc's two
  falsified mechanism statements (I-6). Plus the ADR's call-site-literal sentence and
  the lint rule's `v-show` widening.
- **promoted to the tracker with a written line** (3): the confirm component's missing
  reentrancy guard, as a ROADMAP trigger on the arrival of a second caller; the
  validation-response race, as a fifth member of Plan 13; the stale Tier-2 catalog
  budget, recomputed at this close rather than patched by this package's delta.
- **recorded, not tracked** (8): the four scratch-artifact minors (Task 1 N-1, Task 2's
  escape-clause wording divergence, Task 4's evidence gap, Task 7's sweep-table
  framing); D108's CPU steelman gap, re-derived by the reviewer rather than borrowed;
  the two `registries.ts` figures, MEASURED to hold; and the plan-7 design document's
  sibling paragraph, which the reviewer established is a dated snapshot by resolving
  all eight of its line citations as stale.
- **discarded with reason** (0).
- **carried to the ROADMAP as its own items** (2): `SelectWidget.vue`'s stale catalog
  figure as a docs-accuracy correction, and `StringListWidget.vue`'s dead design
  premise as its own vehicle rather than a reworded sentence.

**Two acceptance rows stay false in a document that becomes history at this close, and
that is a deliberate controller call rather than an oversight.** W4-m claims unit
coverage for a surface that needs the Tauri runtime; W3-h names a helper that returns
zero hits in the file its row points at. Neither is a coverage gap - W4-m's observable
rides the owner's 1.x GUI-test-harness item and W3-h's is materially covered by the
rendered assertions, both established by measurement. Correcting the rows would mean
editing plan content, which the controller does not author, and dispatching a
four-eyes round for two descriptive lines in a retiring document is more machinery than
the correction is worth. **The knowledge is preserved where it will be read**: here, in
the journal entry, and in the general rule the plan produced.

### Blocked-pool sweep (all four house-knowledge files)

Seventeen `status: blocked` entries walked, one by one, against the audit question:
has the `blocked_on` condition cleared, or has the deferred work landed in the tree
regardless of it? **None has fired and none is stale.** Fourteen are gated on v1.x
planning, a shipped 1.0, or external product demand, none of which this plan touches.
The three internal ones were checked against what this plan actually did:
`testing-cli-helper-dedup` waits on the next CLI test file and this plan added none;
`testing-comment-fixture-convention` waits on a second use of that fixture style and
this plan added none; `testing-check-i18n-self-test` waits on v1.x test-hardening, and
although this plan did edit that script's allowlist, editing a script is not the phase
its condition names. **One observation worth carrying rather than a status change:**
this session produced a live argument for that last entry - a reviewer's own detector
reported a clean tree while being structurally unable to fire - which is evidence for
the entry when its phase arrives, not a condition that clears it.

### Deferred minor, Task 7 (for the whole-branch triage)

- The fix-round-2 sweep table is framed as covering every content word and lists a
  subset; the omissions are low-risk generic verbs, all independently checked clean by
  the reviewer. Sixth instance in this plan of the claim-wider-than-its-producer shape,
  recorded here rather than as a sixth ledger occurrence, because the entry is already
  tier 2 and always loaded and its behaviour does not change at six.

### Required fix for the whole-branch wave (not deferrable)

- **The reworded batch empty-state string ships with no producer.** The acceptance map
  claims it is asserted through the catalog helper in the existing batch scenario; the
  id appears in zero test files, controller-reproduced with a control returning 23 hits
  for a sibling id in the same spec file. User-visible, reworded by this plan, and
  expressible with the existing harness, so `tests-ship-with-the-feature-never-after`
  binds and the narrow infrastructure exemption does not reach it. Withheld from Task 7
  because that task's exhaustive Files list carries no test file; routed to the
  whole-branch fix wave, which is not bound by a task's Files list.

**A stale Tier-2 entry the reviewer surfaced, and the plan already owns it.**
`docs/product-boundaries.yaml`'s `editor-generic-action-keys` records the catalog
budget's history as 43 -> 45 -> 46; the count was already 51 before Task 5 and is 54
now. The reviewer read it as orphaned; it is not - the plan's own Global Constraints
name the Tier-2 statement update as a controller close action. What the review adds
is that the entry went stale one package EARLIER than the close would have assumed,
so the close recomputes rather than applying this package's delta to 46.

### The 43-figure sites, enumerated and dispositioned (2026-07-31)

Raised by the amendment author as one further stale site, widened by the controller
to four, enumerated and MEASURED per site by the reviewer. Two sets, not one, which
is why a single verdict over "the 43 figures" would have been wrong:

- `src/editor/registries.ts`, two statements counting registry FIELD SPECS ("42 of
  the 43 fields", "the 43-row table"). **Both HOLD**, measured: 42 label keys plus
  exactly one fixed field across 13 registries.
- `src/editor/widgets/SelectWidget.vue`, counting CATALOG KEYS. **Stale on either
  reading** - the catalog is 54 today and the registry label keys are 42. Its basis
  was real when written.
- `src/editor/widgets/StringListWidget.vue`, counting catalog keys. **Stale and
  wrong on its substance**: it says no generic add/remove chrome exists in the
  catalog, and both of those keys have existed since an earlier package, so the
  premise under that widget's design choice is gone.

All four sit outside every Files list in this plan and outside every fence in it.
**Close-item disposition:** the third is a comment correction for the ROADMAP's
docs-accuracy section; the fourth is not a correction but a design premise that has
expired, so it takes a vehicle of its own rather than a reworded sentence. The
ROADMAP and journal 43-figures are dated and explicitly scoped to their package,
and owe nothing.

### Deferred minor, Task 4 (for the whole-branch review's triage)

- **The evidence gap above.** The fix report's negative is true and independently
  re-derived, but the command it cites does not produce the result it reports. The
  report was not corrected in a further round, because the finding is about a
  citation in a scratch artifact and the conclusion it supports is confirmed;
  recorded here so the whole-branch triage can decide whether the report is worth
  a correction pass before salvage.

### Deferred minors, amendment 1 (for the whole-branch review's triage)

- **O-1** `v-show` is the one render directive outside the prescribed lint rule and
  the prose describing the rule does not say so. The harvest entry above records
  the general shape; this is its concrete instance in a shipping plan.
- **O-2** the report's in-place corrections leave the wrong sentence standing, as
  the house form asks, but carry no pointer at it, so a reader meeting the wrong
  sentence first has nothing telling them a correction follows.
- **O-3** the scope extension's rejected alternative is stated with an accurate
  mechanism claim but without its steelman; its real advantage, that it would have
  kept the two tasks' Files lists disjoint, is the argument omitted.
- **O-4** the author's "eight ordinal cross-references" figure is unverifiable,
  because no such ordinal existed at the round-1 head. The end state was verified
  (zero ordinals into D112 across both documents and the ROADMAP, fired control)
  and the divergence that motivated the renames is real (the plan numbers D112 with
  nine items, the ADR with seven).

### A product defect found by review, verified by the controller, routed (2026-07-31)

**The validation response can erase the parse error after a failed open.**
`watch(model)` returns early on the no-model branch **without incrementing
`validationGeneration`**, so a `validate_profile_model` response still in flight
from the previously opened profile resolves with its generation still current and
overwrites `diagnostics.value` with its own list. The parse diagnostic that
`openPath` had just written is gone, and the user is left in the failed-open state
with nothing explaining it. `openPath`'s `opening` flag does not cover the window,
because the watcher's await outlives that function's `finally`.

Surfaced by the amendment reviewer as a pre-existing observation. **Reproduced by
the controller at the source rather than taken on the report**: the early return
and the missing increment, the generation comparison that then still matches, and
the assignment that overwrites. Task 4 does not change any of it - its own failed
-open branch sits above the same early return.

**Vehicle: a fifth member of Plan 13**, written into the ROADMAP at this plan's
close (ROADMAP dispositions are close actions). The precedent is exact: Plan 13
already took a pre-existing defect routed out of a plan whose fence excluded it,
on the recorded ground that its floor is not its content. Not folded into Plan 12
as a further amendment, because it is a product change no task in this plan
contracts for, and not escalated, because no escalation criterion is met - nothing
recorded collides, no two documented judgments diverge, and the routing pattern is
already the house's.

**Carried into Task 4's dispatch as a named risk, not as work:** if the prescribed
failed-open case proves flaky, this race is the known cause and the case returns as
NEEDS_CONTEXT rather than being patched at the keyboard.

Draft text for the ROADMAP entry, so the close does not have to re-derive it:

> **A fifth member, routed here from Plan 12's amendment-1 review (2026-07-31).**
> `src/views/EditorView.vue`'s `watch(model)` returns early on the no-model branch
> without incrementing `validationGeneration`, so a validation response in flight
> from the previous profile still matches its generation and overwrites the parse
> diagnostic a failed open just wrote. The user then sees the failed-open state
> with no explanation of it, which is precisely the state the owner's option-A
> ruling shapes. Two-line class of fix (increment before the early return, or
> increment unconditionally); the choice and its test are the implementer's to
> propose and the review's to grade. Reproduced at the source by the controller,
> not only reported.

### Deferred minors, Task 1 (for the whole-branch review's triage)

- **M-2** two counts in the ADR without their enumeration ("six shell ids",
  "seven whole-value assignments"). Both verified correct; both closed
  downstream (Task 6's Files list names the six; Task 4's Read-first names the
  enumeration). Flagged because the ADR outlives the plan.
- **M-3** D108's hand-set-dirty-boolean steelman omits its strongest argument:
  the chosen design serialises the whole model per keystroke while text widgets
  stay on per-keystroke binding, so it pays a full `JSON.stringify` where a
  boolean would not; D108's rationale addresses memory and never CPU.
  **Controller ruling, recorded so it is not re-litigated:** the cost is real
  but dominated by an existing accepted cost - the same watcher already fires
  the validation IPC round trip on every model write, which is orders of
  magnitude more expensive than a local serialisation of a profile measured at
  101 bytes for the seed and 419 for the README's four-rule example. No
  criterion for owner escalation is met (no recorded statements collide, no two
  documented judgments diverge, no invariant trade without a ranking). Not a
  fix; recorded as a steelman gap in a durable artifact.
- **M-4** the amended spec's "in one prompt" carries no per-state qualifier
  while D109 decision 9 mandates a conditional second prompt. Inside fenced
  text Task 1 must not decide; Task 6 Step 2b names decision 9 explicitly, so
  nothing is at risk during execution. Roll-up item at the plan close, because
  the spec outlives both.
- **M-5** the spec's seed enumeration omits `input.pattern`, which D107's
  fenced seed carries and justifies. Same disposition as M-4.
- **M-6** D110's "every current call site is a literal, measured" is falsified
  by `ftl_message(key)` inside `run.rs`'s own `#[cfg(test)]` module.
  Consequence nil for the derived key set (all four keys it exercises also
  appear at literal production call sites). **Travels verbatim into Task 6's
  dispatch as a cross-task constraint**, so its implementer meets the
  counter-example already knowing it, instead of spending a NEEDS_CONTEXT round
  trip on it.

