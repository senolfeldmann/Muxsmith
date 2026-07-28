# Amendment 5 - independent review verdict

**Verdict: APPROVED_WITH_MINORS.**

The ruling is true, the replacement assertion is right and strictly stronger
than a count alone, the pairing has no hole, the sweep over both documents is
complete, the commit is correctly isolated from Task 6's live work, and every
count and anchor recomputes. One MEDIUM finding concerns what the fix round is
told to do, not what the amendment wrote: the normative record enumerates one
site where the fix round must touch two. It is dischargeable in the fix brief
without editing either document, which is why this is not NEEDS_FIXES.

---

## 1. Findings

### MEDIUM-1: the fix-round routing enumerates one site where there are two

**Where:** design amendment log, Round 5 closing clause
(`docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md:1857-1858`):
"The code edit rides Task 6's fix round, in the spec file that task already
creates." Plan amendment 5
(`docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md:479-484`)
names no site at all.

**Evidence.** My own tree-wide sweep (instrument in the appendix, fire control
included) finds exactly three live restatements of the falsified vehicle. Two
were qualified by this amendment (design `:1233`, design `:1259`, plus the
plan's `:392`). The third is `e2e/jobsview-reset.spec.ts:36-40`, the header doc
comment: "it keeps the internal transitions assertable through the cancel-batch
button's own disabled state instead of through a value the test itself
supplied." That sentence is now false for item 2 in exactly the way the two
qualified sentences were. The assertion itself sits at `:176`. So the fix round
has two edits, and the singular "the code edit" names one.

Per `proc-latitude-clause-boundary` (`docs/process-conventions.yaml:326`), an
unenumerated set in a normative position is latitude, and this one is a set of
two.

**The routing itself is CORRECT and I rule it so:** the amendment must not edit
a co-writer's uncommitted file, and it did not. The defect is that the
enumeration lives only in the author's report section 5, which is evidence, not
a standing artifact the fix round reads.

**Required change (either discharges it; the first is sufficient):**

1. The Task-6 fix-round brief enumerates both sites: replace the assertion at
   `e2e/jobsview-reset.spec.ts:176` with the rider's two-line pair, and give the
   header doc comment at `:36-40` the same items-1-and-3 scope qualifier the two
   document sentences got, pointing at D104's amendment-5 rider rather than
   restating it.
2. Optionally, one clause in the plan's amendment-5 entry naming both sites, so
   the record survives the fix brief.

### LOW-1: a claimed-unavailable ground truth is in the repo, one grep away

**Where:** `amendment-5-report.md:136-139` - "the doctrine text itself is not in
this repo and I did not audit the analysis."

**Evidence.** The four observable contested criteria are enumerated verbatim
inside `proc-latitude-clause-boundary`'s own statement
(`docs/process-conventions.yaml:326`): recorded-statement collision, post-memo
judgment divergence, invariant trade without recorded ranking, ledger recurrence
while contested. The plan-6 line the author cited as mere corroboration
(`docs/superpowers/specs/2026-07-15-plan-6-design.md:1999-2000`) says so in its
own sentence: "A recorded-statement collision is one of the four observable
contested criteria in `proc-latitude-clause-boundary`."

**I ran the audit the report declined to run**, and the solo ruling holds on all
four (see adjudication 3). No amendment change required; the finding is that the
self-limitation was unnecessary and the premise for it was not measured.

### LOW-2: the "cannot re-run it" premise is false as stated; the claim it excuses is true

**Where:** `amendment-5-report.md:131-133` - "Re-running it would mean mutating
Task 6's uncommitted spec, which the preamble forbids", justifying that the
`not.toBeEnabled()` fact is carried on Task 6's attribution.

**Evidence, run by me, read-only, against the pinned dependency**
(`node_modules/.pnpm/playwright-core@1.61.1/node_modules/playwright-core/lib/coreBundle.js`,
`_expectCore`): for a missing element the short-circuit list is enumerated and
closed - `to.be.hidden` (positive), `to.be.visible` (isNot), `to.be.detached`
(positive), `to.be.attached` (isNot), `to.be.in.viewport` (isNot), plus the
document-level `to.have.title` / `to.have.url` / `to.match.aria`. Everything
else falls through to `return { matches: options.isNot, missingReceived: true }`,
which the polling loop scores as a mismatch (`if (matches === options.isNot)`)
and surfaces on timeout as `errorMessage = "element(s) not found"`.
Probe with its own fire: `to.be.enabled` **absent** from that region,
`to.be.disabled` **absent**, while the controls `to.be.visible`,
`to.be.hidden`, `to.be.attached` are **present**.

So the rider's sentence is TRUE, and it is now verified by an instrument that is
not Task 6's: `not.toBeEnabled()` cannot pass on a detached element, while
`.not.toBeVisible()` can, and that asymmetry is by enumeration, not by accident.
The same read confirms the other half: `to.have.count` takes the `isArray`
branch and evaluates against an empty element list, which is exactly why
`toHaveCount(0)` alone would pass against a view that never mounted.

The finding is the reasoning, not the fact: a read-only verification path
existed at zero risk to the co-writer's tree and was not taken before the cost
claim was written.

### LOW-3: "likewise unchanged" said of a paragraph the same commit edited

**Where:** design `:1335-1337` - "The `runActive`-not-a-prop rationale above is
likewise unchanged."

That paragraph's rationale is indeed unchanged, but its text gained a scope
qualifier in this very commit (design `:1259-1260`). The Round-5 log entry does
record that edit explicitly, so the delta record is complete and nothing is
hidden; the rider's wording is simply loose about substance versus text. No
change required unless the rider is revised anyway.

### INFO-1: the pairing's recorded reason is true but weaker than the one that carries it

The rider justifies pairing with "`toHaveCount(0)` alone would also pass against
a view that never mounted at all". True. But item 2's first assertion
(`jobs.getByRole("alert")` has the localized `run-already-active` text,
`e2e/jobsview-reset.spec.ts:175`) already proves the view mounted, so that is
not what the pairing buys in situ. What `jobs-empty` visible actually adds is
proof that the `v-else` BRANCH IS ACTIVE, hence that all three disjuncts are
false - which also survives someone deleting or renaming the `cancel-batch`
testid, a mutation the count alone would silently accept. Worth the sharper
sentence if the rider is ever touched.

### INFO-2: plan Step 4's NEEDS_CONTEXT clause states an equivalence this very episode falsified

Plan `:398`: "if any of the three ordering tests fails against the unmodified
views, **that contradicts the adjudicated premise** and returns as
NEEDS_CONTEXT". Task 6's failure did not contradict the premise; it exposed an
unsatisfiable observable, and the premise came out confirmed. The amendment
declares the clause unchanged and still binding, which is right about its
operative half (return it, never fix it at the keyboard) and is why the routing
worked. Recording it as a ledger observation is the proportionate response, not
a document edit.

### INFO-3: progress.md is behind

`.superpowers/sdd/plan-9/progress.md` records amendments 1-4 (the fourth at
`:79`) but not 5, and Task 6's row (`:14`) still reads NOT STARTED with no
NEEDS_CONTEXT note. Controller bookkeeping, outside the author's brief-scoped
two documents; named here so the close does not re-derive it.

---

## 2. What I verified and found sound

**Dimension 1, is the recorded reason true - YES, every link re-derived.** My
own instrument (`anchors.py`, appendix) checks 17 claims plus one deliberately
wrong control; 17 matched, the control mismatched. `cancel-batch` `:263`,
`:disabled="!runActive"` `:264`, the `v-if` `:258` reading
`jobs.length > 0 || runActive || finishedSummary`, `startingFresh =
!runActive.value` `:176`, the fresh reset `:177-182` (jobs, logLines,
finishedSummary, then `runActive = true`), `ensureJobsLength(started.total_jobs)`
`:191`, the catch arm `:193-196`, `v-else` `:327`, `jobs-empty` `:328`. Control
flow confirmed structurally rather than by eye: `try` opens `:184`, `await
startRun({` `:185`, `catch` `:192`, so `:191` sits after the await and a
rejection skips it. The complete set of writers to `runActive` is four sites
(`:50` defineModel default false, `:119` onRunFinished, `:181`, `:195`), and to
`jobs`/`finishedSummary` five, none of which fires in item 2's scenario after
the catch. End state: jobs `[]`, finishedSummary `null`, runActive `false`, all
three disjuncts false, button not rendered, `v-else` active.

**Dimension 2, replacement and pairing - SOUND, no hole.** `v-if`/`v-else` are
mutually exclusive by construction, so `jobs-empty` visible implies
`runActive === false` UNCONDITIONALLY (the rider's two givens are not even
needed for that direction); the converse needs jobs empty and finishedSummary
null, which item 2's scenario establishes and which the rider states. There is
no state with `jobs-empty` visible and `runActive` true, and none with
`runActive` false, jobs empty, finishedSummary null and `jobs-empty` hidden. Two
practical checks beyond the logic: `jobs-no-run` is a non-empty string in both
locales (`locales/en/gui-jobs.ftl:13`, `locales/de/gui-jobs.ftl:16`) and the
mount harness serves `page.setContent('<!doctype html><div id="mount"></div>')`
with no stylesheet, so nothing can render the placeholder zero-box and
`toBeVisible()` is not a flake risk. The fenced snippet's locator variable
`jobs` matches the spec's own local (`e2e/jobsview-reset.spec.ts:174`), so it
pastes in unadapted. Ordering is safe: the alert assertion above it already
proves the catch arm ran, and `startError` is set in the same synchronous arm as
`runActive = false`, so `toHaveCount(0)` cannot pass on the pre-dispatch state.

**Dimension 3, completeness of the sweep - COMPLETE except the known third
site.** My sweep covered the whole tree, not the two documents. Live surfaces
and their disposition: design `:1230` (item 1, "cancel-batch is disabled") stays
TRUE - item 1's soft outcome leaves `finishedSummary` set, block rendered;
design `:1242` (item 3, "stays enabled") stays TRUE - the second dispatch is not
fresh; design `:1233` and `:1259` QUALIFIED; plan `:392` QUALIFIED; plan `:397`
("the D104 assertions target `data-testid` selectors ... no fourth `name()`
consumer") stays TRUE under the replacement, since both new assertions are
testid-based with no localized accessible-name match; `docs/ROADMAP.md:677`
carries the same claim and stays TRUE for the same reason; design section 5
`:1594` and acceptance observable 7 `:1688` are count- and file-level only, so
the author's "no falsified statement" reading is confirmed by my own read;
`e2e/jobsview-reset.spec.ts:164` (item 1) and `:216` (item 3) stay correct.
`task-6-brief.md` restates nothing (its only hit, `:68`, is a testid-existence
pointer that stays true). Undispositioned: none, beyond MEDIUM-1's `:39`.

**Dimension 4, the "Unchanged" enumeration - RIGHT SET.** I enumerated D104's
decided content independently: the "No code fix" ruling with its two premises;
the harness enumeration (glob/`resolvePath` fence, `spec.props` ref and
`__muxsmithSetProps__`, `mount.ts` untouched); the spec composition; the four
numbered items; the not-duplicated orderings 1 and 5; the
`runActive`-not-a-prop paragraph; the ledger entry. Every one is either named in
the close or addressed in the rider body (the `runActive` paragraph, see LOW-3).
Nothing is silently left out, and the closing "everything else this entry
decides" is a closed set that survives the walk.

**Dimension 6, house - CONFORMANT.**
`concurrent-writers-need-pathspec-scoped-commits`: `git show --name-only HEAD`
returns exactly the two documents, `git diff --cached --name-only` is empty (so
nothing of the co-writer's is staged), and `git status --porcelain -- e2e/`
still shows the seven entries (six ` M`, one `??`), unstaged as they were.
`latitude-carveout-zero-content-structural-forks`: nothing structural was
resolved at the keyboard; the two pointer qualifiers are ruled in scope below.
`proc-normative-count-recomputed`, all recomputed by my instrument, not read
from the text: three disjuncts (the condition splits into exactly three on
`||`); four numbered D104 items, so "the four assertions" still holds; design
amendment-log rounds `['1','2','3','4','5']`; plan amendment entries
`['1','2','3','4','5']`; and the arithmetic behind "coincide at 5 by arithmetic,
not by rule" checks out - rounds 1 and 2 are review rounds, round 3 = amendment
1, round 4 = amendment 3, round 5 = amendment 5, while plan amendments 2 and 4
are plan-only, so 2 + 3 = 5 on one side and 2 + 3 = 5 on the other.

**Dimension 8, typography and form - CLEAN.** My checker over all 123 added
lines of `HEAD` finds zero occurrences of em dash, en dash, figure dash,
horizontal bar, Unicode minus, U+2010/U+2011, smart quotes, ellipsis or NBSP;
the same checker against a control string containing an em dash, smart quotes
and an ellipsis trips four times, so the zero is a real zero. The rider follows
D96's amendment-3 rider shape exactly: bold ruling sentence naming what changed
and what did not, executed-as-ordered narrative with the routing reference,
anchors declared re-verified at the tree with the date, substance in bold-lead
paragraphs, `**Unchanged by this amendment:**` close. The Round-5 log entry
matches its siblings' form (round number, date, ruling authority, mid-execution
context, routing, "one ruling, nothing else touched", then the bulleted ruling).

---

## 3. The six adjudications

**1. The two pointer qualifiers inside D104: IN SCOPE. The rider should not have
stood alone.**

The brief's "Change nothing else in either document" collides with its own
design-half instruction to write the rider "in the shape the file already uses
for amendment riders (read amendment 3's rider under D96 and the `## Amendment
log` entries first, and follow that form - this brief does not restate it)".
That form, recorded in the file itself at Round 4 `:1819-1821`, is precisely:
"the entry's opening sentence and section 5's D96 bullet now point at the rider
so 'moves as-is' cannot be read as covering the rustdoc going forward". The
brief incorporated the practice by reference and then ordered the identical
operation on the plan's twin sentence. The design's `runActive` paragraph is the
ORIGIN of which the plan sentence is the copy; qualifying the copy and leaving
the origin is the dangling-reference defect one level up. Both edits are
strictly pointer-shaped, add zero decision content, and are two parenthetical
deletions to revert. Flagging it as judgment call 1 rather than burying it was
the correct handling of a genuinely conflicting brief.

**2. Omitting the design-commit hash: ACCEPTABLE, and in fact forced.**

Amendments 1 and 3 cite one because their halves were separate commits by
separate authors (amendment 3 had two briefs). Amendment 5 was dispatched as one
author for both halves under the recorded model-tiering ruling
(`progress.md:87-89`: one author and one reviewer for both halves unless the
amendment adds, removes or re-cuts a TASK), and its brief ordered "One commit
for both halves is correct here". A commit cannot cite its own hash. The entry
says so explicitly and designates the design's Round-5 entry as the authoritative
delta record, which is the same role the cited hashes play in amendments 1 and 3.
Nothing is lost: `git log` locates the commit from either document.

**3. "CONTROLLER-RULED AMENDMENT 5" in a log of four owner-ruled entries: RIGHT
LABEL.**

Two questions hide in this one. Was the solo ruling permitted, and does the label
belong in the log? On the first, I ran the audit the author declined to run,
against `proc-latitude-clause-boundary`'s own enumeration. Fork nature: internal
technical - no product scope and no user-visible behaviour changes, only the
observable through which a test reads an already-adjudicated transition. That
routes to the controller. Solo requires none of the four contested criteria to
hold: (a) recorded-statement collision - none; a recorded observable collided
with the TREE, which is ground truth, not a second recorded statement, and the
statements consulted (`tests-ship-with-the-feature-never-after`,
`proc-proposed-safeguard-stays`, D104's "No code fix") all point the same way;
(b) post-memo judgment divergence - none; Task 6's memo recommended option A at
`task-6-report.md:354-360` and the controller ruled option A; (c) invariant trade
without recorded ranking - none; option A pays nothing, it keeps the no-code-fix
ruling, the assertion, and the asserted behaviour; (d) ledger recurrence while
contested - no entry for this class exists yet (the author surfaces it as a
candidate) and nothing is contested. Solo holds.

On the label: amendment 4 was owner-ruled on the same surface class ("substance
right, stated form impossible"), and the difference is real rather than
arbitrary - amendment 4 traded a third bare `cargo_bin` caller against a new
locale-parameterized helper touching D64's recorded pinning decision and a Files
list, which is an invariant trade wanting a ranking. Marking the difference in
the log is exactly right; hiding it under a uniform "OWNER-RULED" would
misrepresent who decided, and a reader auditing the tier decision needs to see
the claim in order to check it.

**4. Attributing the `not.toBeEnabled()` measurement to Task 6: SUFFICIENT for
the rider, though the stated reason for not re-running was wrong.**

A design document that attributes a measurement to the run that produced it, by
name and section, is doing the right thing - it is a citation, not a borrowed
claim dressed as its own. Requiring the amendment author to re-run a Playwright
suite mid-flight in a co-writer's tree would be worse practice than citing. But
the report's justification ("re-running would mean mutating Task 6's uncommitted
spec") is not the only path and was not tested: the mechanism was verifiable
read-only from the pinned dependency, which is what I did (LOW-2). Net: the rider
text stands as written; the fact is now independently confirmed, so no
re-measurement obligation survives this review.

**5. The plan half carrying no explicit fix-round instruction: THE SPLIT IS
CORRECT, THE ENUMERATION IS NOT.**

Rider-as-fence plus a routing clause is the right shape and matches amendment
3's rustdoc precedent exactly (one home for the contract, the plan points at it,
the code edit rides the task that owns the file). Restating the two assertions in
the plan would create the second home the fence exists to prevent. So the answer
to the question as posed is: no, the plan should not carry the replacement text.
What the record must carry is the SET of sites, and it names one of two - see
MEDIUM-1. Discharge that in the fix brief, not by duplicating the fence.

**6. The "Unchanged by this amendment" close: RIGHT SET, and the item-2 split is
correctly cut.**

Naming item 2's scenario and first assertion as unchanged while its second
assertion changes vehicle is precisely right, and both halves are verified:
the scenario (`rejectWith("run-already-active")` on a fresh dispatch) is
untouched, and the first assertion passes today
(`task-6-report.md:233-234`, my own read of the spec at `:175`). Items 1, 3 and 4
in full is correct - item 4 carries no cancel-batch assertion at all, which is
also why the `runActive` paragraph's qualifier names items 1 and 3 rather than 1,
3 and 4. My independent walk of D104's decided content found nothing the
enumeration silently omits (see dimension 4). The only wrinkle is LOW-3's
wording.

---

## 4. Evidence appendix

All instruments are mine, written for this review, under
`/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/a5rev-independent/`.
Nothing another agent wrote was re-run, no shared default path was used, and
every absence check below carries its own fire. All runs foreground; no file in
the repo was written, staged or reverted by me.

| Instrument | What it establishes | Fire |
|---|---|---|
| `anchors.py` | re-derives all 17 rider anchors and the control flow from `src/views/JobsView.vue`, plus the complete writer sets for `runActive`, `jobs`, `finishedSummary` | one deliberately wrong claim (`jobs-empty` expected at `:263`) must and does report MISMATCH; 17 others OK |
| `sweep.py` | tree-wide sweep for `cancel-batch`, `jobs-empty` and the disabled-state vehicle, every file outside `.git`/`node_modules`/build dirs | a fourth pattern that cannot occur returns 0 while the three real ones return 379/38/407 |
| `typo_counts.py` | 13 AI-tell glyphs over the 123 added lines of `HEAD`; recomputes the disjunct count, D104's item count, both amendment-log numberings, and the rider-shape parity with D96 | the same checker over a control string with em dash, smart quotes and ellipsis trips 4 times |
| ad hoc `python3` over `playwright-core@1.61.1/lib/coreBundle.js` | the enumerated missing-element short-circuit list in `_expectCore` and the `matches === options.isNot` polling branch that emits "element(s) not found" | `to.be.enabled` and `to.be.disabled` absent from the region while the controls `to.be.visible`, `to.be.hidden`, `to.be.attached` are present |
| `git show --name-only HEAD`, `git diff --cached --name-only`, `git status --porcelain -- e2e/` | commit contains exactly the two documents; index empty; the seven `e2e/` entries intact and unstaged | the staged list being empty is corroborated by the same seven entries showing worktree-only status codes |

House entries read at the source and confirmed to say what the amendment uses
them for: `proc-latitude-clause-boundary`
(`docs/process-conventions.yaml:326`), `latitude-carveout-zero-content-structural-forks`
(`:348`), `proc-proposed-safeguard-stays` (`:497`),
`proc-normative-count-recomputed` (`:512`),
`tests-ship-with-the-feature-never-after` (`:668`),
`concurrent-writers-need-pathspec-scoped-commits`
(`docs/decision-ledger.yaml:4440`), `gui-d23-reset-gating-form` (`:4539`).

---

## 5. HARVEST

**The Task-6 fix round MUST carry (MEDIUM-1):**

1. `e2e/jobsview-reset.spec.ts:176` - replace
   `await expect(jobs.getByTestId("cancel-batch")).toBeDisabled();` with the
   rider's pair, verbatim:
   ```ts
   await expect(jobs.getByTestId("cancel-batch")).toHaveCount(0);
   await expect(jobs.getByTestId("jobs-empty")).toBeVisible();
   ```
2. `e2e/jobsview-reset.spec.ts:36-40` - the header doc comment's
   "keeps the internal transitions assertable through the cancel-batch button's
   own disabled state instead of through a value the test itself supplied" gets
   the same items-1-and-3 scope qualifier, pointing at D104's amendment-5 rider
   and not restating it. Items 1 and 3 keep the disabled-state sentence; item 2's
   vehicle lives in the rider.
3. Nothing else in that file changes; `:164` (item 1) and `:216` (item 3) are
   correct as written and stay.

**Controller bookkeeping (INFO-3):** `progress.md` needs the Amendment-5 entry
beside amendment 4 at `:79` and a Task-6 row that reflects NEEDS_CONTEXT plus
the amendment, rather than NOT STARTED.

**Ledger-worthy observations:**

- **Playwright absence idiom, now verified from source rather than from one
  run.** The negated state matchers do not pass on a detached element:
  playwright-core's `_expectCore` short-circuits a missing element only for an
  ENUMERATED list (`to.be.hidden`, `to.be.visible` under `not`, `to.be.detached`,
  `to.be.attached` under `not`, `to.be.in.viewport` under `not`, plus
  title/url/aria), and everything else, `to.be.enabled` and `to.be.disabled`
  included, returns `missingReceived` and keeps polling to a timeout reported as
  "element(s) not found". The house idiom for asserting a control is gone is
  therefore `toHaveCount(0)` PAIRED with a positive bearer, because
  `to.have.count` takes the array branch and an empty page satisfies it. The
  pairing's real strength is that the bearer proves the intended branch is
  active, not merely that something mounted.
- **Second instance in one plan of "design substance right, stated form cannot
  execute"** (amendment 4's impossible pinned invocation, amendment 5's
  bearerless assertion), both caught on code contact by the executing agent and
  neither by design review. The readable check the class suggests: an assertion
  about a control's STATE presupposes the control renders, so a scenario that
  RESETS state must have its render chain walked at design time, not only its
  state transition. The design's own wording carried the tell both times - here,
  a "disabled AGAIN" whose earlier state the entry never established, and which
  is itself buttonless.
- **A red test does not entail a contradicted premise** (INFO-2). Plan Step 4's
  clause equates the two; this episode is a counterexample where the premise came
  out confirmed and the observable was the defect. The operative half of such a
  clause (return it, never fix it at the keyboard) is what did the work and
  should be what carries the weight in future plans; the causal gloss should say
  "contradicts the premise OR shows the stated observable unsatisfiable", since
  the two route identically but conclude differently.
- **A cost claim that excuses a verification is itself a claim to measure**
  (LOW-1, LOW-2, same shape twice in one report): "the doctrine text is not in
  this repo" was refuted by one grep of `process-conventions.yaml`, and "re-running
  it would mean mutating the co-writer's spec" was refuted by a read-only look at
  the pinned dependency. Both self-limitations were written before the cheap path
  was tried.
