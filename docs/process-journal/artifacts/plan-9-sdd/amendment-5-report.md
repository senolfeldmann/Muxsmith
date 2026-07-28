# Amendment 5 - author report (design half + plan half)

Both halves written and committed in one commit, `1e0dbd8`. No code, no test,
no house-knowledge YAML touched. Task 6's uncommitted work is untouched and
still uncommitted.

## 1. What each half now says

### Design half - `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`

**The amendment-5 rider at D104's end** (after the ledger-entry paragraph,
before D105), in the file's own rider shape: a bold ruling sentence, then
bold-lead paragraphs, then an "Unchanged by this amendment" close - the shape
D96's amendment-3 rider uses.

1. *The stated vehicle has no bearer in item 2's scenario.* `cancel-batch`
   (`src/views/JobsView.vue:263`, bound `:disabled="!runActive"` at `:264`)
   inside `<template v-if="jobs.length > 0 || runActive || finishedSummary">`
   (`:258`); fresh dispatch is `startingFresh = !runActive.value` (`:176`) and
   the fresh branch resets `jobs`, `logLines`, `finishedSummary` before the
   invoke (`:177-182`); on rejection the catch arm sets `startError` and
   `runActive = false` (`:193-196`) while `ensureJobsLength` (`:191`) is never
   reached. All three disjuncts false in every end state, so "disabled" has
   nothing to be true of. Includes the tell: the entry's own "disabled *again*"
   names a mount state that is also buttonless.
2. *The adjudicated behavior is confirmed, not contradicted.* The absence IS
   the consequence of `runActive` going back to false, measured by mutating the
   catch arm (button appears, 1 element where 0 was expected). The defect is
   the vehicle, not the substance - same class as the plan's amendment 4.
3. *The replacement*, as a fenced `ts` block, verbatim from the brief, with
   item 2's first assertion explicitly unchanged; then *the pairing is the
   assertion, not the count alone*: `jobs-empty` (`:328`, the `v-else` at
   `:327` of the same condition) is visible exactly when all disjuncts are
   false, so with `jobs` empty and `finishedSummary` null its visibility is
   logically equivalent to `runActive === false`; `toHaveCount(0)` alone would
   also pass against a view that never mounted.
4. *No wording-preserving fix exists, and that was measured.*
   `expect(locator).not.toBeEnabled()` does not pass on a detached element -
   Task 6 ran it, "element(s) not found", times out exactly like the positive
   form. Both forms require the element to resolve.
5. *Items 1 and 3 are untouched* - item 1's soft outcome leaves
   `finishedSummary` set, item 3's second dispatch is not fresh, so both render
   the block and keep the disabled-state vehicle (both green in the same run);
   the `runActive`-not-a-prop rationale is unchanged.

Plus **"Unchanged by this amendment"**: "No code fix", the harness
enumeration, the spec composition, item 2's scenario and first assertion, items
1/3/4 in full, the not-duplicated orderings 1 and 5, the ledger entry.

**Two pointer qualifiers inside D104** (judgment call, concern 1 below): item
2's numbered line now ends "- the second assertion's vehicle is the amendment-5
rider's, at this entry's end", and the `runActive`-not-a-prop paragraph's
closing clause now reads "... disabled state (items 1 and 3; item 2's vehicle
is the amendment-5 rider's)". Both point; neither restates.

**Amendment log: Round 5**, labelled CONTROLLER-RULED AMENDMENT 5, routing to
the task-6 report's section 4, with the numbering note (Round 5 = amendment 5
by arithmetic, not by rule: rounds 1-2 were review rounds, amendments 2 and 4
were plan-only), why it was ruled rather than escalated, and the one-ruling
delta: what the rider carries, which two sentences now point at it, what stays
unchanged, and that the code edit rides Task 6's fix round.

### Plan half - `docs/superpowers/plans/2026-07-28-plan-9-core-hoists-planner-seam.md`

**Task 6 Step 2's closing sentence** now reads: "... the internal transitions
are asserted through the cancel-batch button's disabled state, as D104 fixes -
for items 1 and 3; item 2's second assertion takes the vehicle D104's
amendment-5 rider fixes." The original clause is byte-identical up to the
appended qualifier; the qualifier points and does not restate, so the two
replacement assertions have exactly one home (the rider).

**`## Amendment 5`** appended to the plan's amendment log in the shape of the
previous four: routing line, the explicit contrast with amendment 4 (this one
DOES amend the design, hence design Round 5), the ruling in one paragraph, then
three "what moved" bullets - the Step-2 qualifier and why it points rather than
restates; a consumer sweep naming each surface with its verdict (Files list
unchanged, coverage-map D104 row unchanged, Step 4's verification bar and
NEEDS_CONTEXT clause unchanged and still binding, must-not-decide's "the four
assertions" unchanged, Step 5's `git add`/commit line untouched); and the
deliberate no-renumbering/no-new-step note with its reason.

## 2. Premises verified, and how

Every line anchor in the brief was re-derived by `grep -n` against
`src/views/JobsView.vue` at the current tree, not by arithmetic on a range
read. All correct, none refuted:

| Brief's claim | Measured |
|---|---|
| `cancel-batch` at `:263` | `data-testid="cancel-batch"` at 263 |
| render condition at `:258` | `<template v-if="jobs.length > 0 \|\| runActive \|\| finishedSummary">` at 258 |
| fresh-branch reset at `:177-182` | `if (startingFresh) {` 177, `jobs.value = []` 178, `logLines.value = []` 179, `finishedSummary.value = null` 180, `runActive.value = true` 181, `}` 182 |
| catch arm at `:193-196` | `startError.value = e as IpcError;` 193, `if (startingFresh) {` 194, `runActive.value = false;` 195, `}` 196 |
| `jobs-empty` at `:328` | `data-testid="jobs-empty"` at 328, its `v-else` at 327 |

Additional anchors I derived and used in the rider: `startingFresh =
!runActive.value` at `:176`, `:disabled="!runActive"` at `:264`,
`ensureJobsLength(started.total_jobs)` at `:191` (inside the `try`, after the
`await`, so a rejection skips it).

Other premises checked:

- **D104 opens "No code fix"** - confirmed at the entry's decision paragraph.
- **`tests-ship-with-the-feature-never-after` and `proc-proposed-safeguard-stays`
  exist and say what the exclusion needs** - `docs/process-conventions.yaml`
  `:668` and `:497`; the first names "recording the gap honestly" as not a
  resolution, the second permits removal only after a guard is built and
  MEASURED redundant (here it was built and measured NOT redundant: it detects
  the flag). Both citations are apt as used.
- **The fresh reset is unconditional before the invoke**, so option D
  (re-shaping the scenario) is impossible - confirmed at `:176-182`.
- **Items 1, 3, 4 are green and only item 2's second assertion failed** -
  task-6 report 3.4 and the pasted failure at 4.1 (`176 |` is the failing line;
  the tree's `e2e/jobsview-reset.spec.ts` still carries `toBeDisabled()` at
  line 176, so the fix round has work to do there).
- **The logs' numbering** - design rounds 1, 2 (reviews), 3 (= amendment 1), 4
  (= amendment 3); plan amendments 1-4. Recounted from the headers, not
  imported; hence Round 5 = amendment 5.
- **Consumer sweep for the disabled-state claim**, both documents grepped for
  `cancel-batch|jobs-empty|disabled`: in the design only D104 item 2 (`:1233`)
  and the `runActive`-not-a-prop sentence (`:1257`, pre-edit numbering) restate
  it; section 5's "the four assertions of D104" and the acceptance observable
  are count- and file-level only, so they carry no falsified statement. In the
  plan only Step 2's sentence. One more instance lives outside both documents,
  in Task 6's uncommitted spec - surfaced in section 5, not edited.

Nothing in the brief was refuted.

**Not independently verified, and marked as such:**

- The `not.toBeEnabled()` measurement is Task 6's; the rider attributes it
  ("Task 6 ran it") rather than claiming it as mine. Re-running it would mean
  mutating Task 6's uncommitted spec, which the preamble forbids.
- "No contested criterion holds" is the controller's own tier analysis; the
  design log records it as such ("its tier analysis found no contested
  criterion"). The four-criteria framing is corroborated on record
  (`docs/superpowers/specs/2026-07-15-plan-6-design.md:1999`, "one of the four
  observable contested criteria"), but the doctrine text itself is not in this
  repo and I did not audit the analysis.

## 3. Divergences and judgment calls

1. **Two pointer qualifiers added inside D104** beyond the rider itself (item
   2's numbered line, the `runActive`-not-a-prop paragraph). The brief says
   "Change nothing else in either document"; I read that enumeration as
   plan-side (it continues "Task 6's Files list, its other steps, ..."), and
   applied the file's own recorded precedent: amendment 3 gave D96's opening
   sentence and section 5's D96 bullet exactly this treatment "so 'moves as-is'
   cannot be read as covering the rustdoc going forward", and the brief itself
   orders the identical operation on the plan's twin sentence. Leaving the
   design's own twin unqualified while qualifying the plan's would be the
   dangling-reference defect one level up. Strictly pointer-shaped; reverting
   is two parenthetical deletions.
2. **No design-commit hash in the plan's amendment entry.** Amendments 1 and 3
   cite the amended design commit; here both halves land in one commit, so the
   hash cannot be cited from inside its own content. The entry says so
   explicitly and names the design's Round-5 entry as the authoritative delta
   record.
3. **CONTROLLER-RULED, not OWNER-RULED**, in both logs - every prior entry says
   owner-ruled. Per the brief this is the controller's call, not an escalation,
   and the label carries that distinction rather than hiding it.
4. **The plan half does not tell the fix round what to edit in the spec.** The
   rider is the fence (one home, no drift); the design log's routing clause
   records that the code edit rides Task 6's fix round. Same split amendment 3
   used for the rustdoc fence.
5. **The rider says "the same class as the plan's amendment 4"** - a
   cross-reference from design to plan. Justified: it is the precedent that
   makes "substance right, stated form impossible" a recognized category here,
   and the task-6 report already invokes it.

## 4. Numbered concerns a reviewer can rule on yes/no

1. Are the two pointer qualifiers inside D104 (judgment call 1) in scope, or
   should the rider have stood alone with item 2's line and the
   `runActive`-not-a-prop sentence left unqualified?
2. Is omitting a design-commit hash from the plan's amendment entry acceptable
   for a single-commit amendment, given amendments 1 and 3 cite one?
3. Is "CONTROLLER-RULED AMENDMENT 5" the right label in a log whose four
   previous entries are owner-ruled?
4. Is attributing the `not.toBeEnabled()` measurement to Task 6 ("Task 6 ran
   it") sufficient, or must the design rider carry a measurement the amendment
   author re-ran?
5. Should the plan half have carried an explicit fix-round instruction for
   `e2e/jobsview-reset.spec.ts` (line 176 and the file's header comment), or is
   the rider-as-fence plus the routing clause the correct split?
6. Does the rider's "Unchanged by this amendment" close enumerate the right
   set, in particular naming item 2's scenario and first assertion as unchanged
   while its second assertion changes vehicle?

## 5. Surfaced for the controller (I edited no house-knowledge YAML)

- **A third instance of the now-partially-false claim sits in Task 6's
  uncommitted spec.** `e2e/jobsview-reset.spec.ts:39`, the file's header doc
  comment: "through the cancel-batch button's own disabled state instead of
  through ...". Same class as the two sentences the amendment qualified, but it
  is Task 6's file, so the fix round should carry it alongside the assertion at
  `:176`. I did not touch it.
- **Ledger candidate (process, likely an occurrence on an existing entry):**
  second instance in one plan of *design substance right, stated form cannot
  execute* - amendment 4's impossible pinned invocation, amendment 5's
  bearerless assertion. Both were caught only on code contact by the executing
  agent, neither by design review. The readable check the class suggests: an
  assertion about a control's STATE presupposes the control renders, so a
  scenario that resets state needs its `v-if`/render chain walked at design
  time, not only its state transition.
- **Ledger candidate (gui/e2e tooling fact, measured by Task 6):** Playwright's
  negated `not.toBeEnabled()` / `not.toBeDisabled()` do NOT pass on a detached
  element - they report "element(s) not found" and time out like the positive
  form. The absence idiom is `toHaveCount(0)` paired with a positive bearer
  (here `jobs-empty` visible), because the count alone also passes against a
  view that never mounted.
- **The design's own wording carried the tell** ("disabled *again*", naming a
  mount state that is also buttonless). Worth recording as the shape a design
  reviewer can read: an "again" that refers to a state the entry never
  establishes.

## 6. Commit

```
$ git show --stat HEAD
1e0dbd8926d2872d5021cfb55af86c7c6a8270ad
Şenol Feldmann <senol.feldmann@gmail.com>

design+plan: amendment 5 - D104 item 2's second assertion asserts the control's absence

 .../2026-07-28-plan-9-core-hoists-planner-seam.md  |  10 +-
 ...-07-28-plan9-core-hoists-planner-seam-design.md | 116 ++++++++++++++++++++-
 2 files changed, 123 insertions(+), 3 deletions(-)

$ git log -1 --format="%(trailers)"
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

Exactly two files, both mine. One trailer, no `Claude-Session` line, committed
with `git -c commit.gpgsign=false commit -F <msg> -- <two pathspecs>` (never
`git add`, so the shared index was never used).

`python3 scripts/ledger-lint.py` after the edits: "510 entries across 4 files,
all invariants hold" (docs-only push gate; I changed no YAML, run as a sanity
check).

## 7. Proof Task 6's work is untouched and still uncommitted

```
$ git status --porcelain
 M e2e/editor-markers.spec.ts
 M e2e/editor-rule-add-remove.spec.ts
 M e2e/global.d.ts
 M e2e/i18n-en.ts
 M e2e/mount-entry.ts
 M e2e/smoke.spec.ts
?? e2e/jobsview-reset.spec.ts
```

Seven entries, byte-for-byte the same list as before my first edit (six
modified, one untracked). `git diff --stat -- e2e/` after the commit still
shows all six modifications live in the working tree (40 insertions, 34
deletions across the six files), and `e2e/jobsview-reset.spec.ts` is still
untracked on disk (10737 bytes).

Independent evidence that I never wrote to them: every `e2e/` mtime is
`2026-07-28 23:09:41` or earlier, while my two document writes are 23:24:13 and
23:25:18 (commit at 23:26). I issued no write, `cp`, `mv` or `git add` against
any path outside my two documents; the only shell writes in this run were to my
scratchpad (the commit message) and to this report.

## 8. Typography check

`git diff -U0 -- docs/superpowers | grep "^+" | grep -P
'[\x{2010}-\x{2015}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{2212}\x{00A0}]'`
returned nothing on the added lines. Absence-shaped, so it was fired first: the
same pattern against a control string containing an em dash and smart quotes
matched line 1 and ignored the clean line 2.
