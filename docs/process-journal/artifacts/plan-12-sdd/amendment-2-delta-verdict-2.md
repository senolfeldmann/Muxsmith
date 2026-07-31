# Amendment 2, fix round 2: delta verdict

**N-1, N-2, N-3: all three ADDRESSED. New breakage: 0.**
**The pair inventory is complete, and I derived the same eleven independently.**
**The amendment is ready to release to its waiting implementer.**

Instruments at `/tmp/.../scratchpad/amd2-r2-paarprobe/`, a path none of my earlier
passes used. Nothing written in the repository except this verdict; no stash,
checkout, reset, restore or clean; Task 5's six uncommitted paths unchanged after
my pass.

---

## 1. Per-finding verdicts

**N-1 (the twins disagreed on which claims were falsified): ADDRESSED.** I
re-extracted both clauses and split them sentence by sentence, as I did when I
raised it. Both now close with the same claim and the same apposition: "both were
true when written and **both** were falsified by amendment 2 - the count by the
case that amendment's repair added to the guarded set, and the closure claim by
the membership criterion that now depends on this decision". The
self-refutation is gone: the clause that opens with "**two** tests move" no longer
ends by saying only the second claim went stale. The falsified original is still
quoted rather than deleted. The remaining differences between the two forms are
the ones I approved in the last round and are deliberate: the plan says "Task 5
Step 4's case 4" and "Step 4b's criterion" where the decisions document says "the
New case" and "the plan's membership criterion for the cases this guard fronts",
which is a spec document correctly refusing a plan-internal ordinal. **The
per-claim breakdown is more than the finding asked for** and is the right
addition: an append-only record's "why" now names which claim failed for which
reason, rather than asserting that both did.

**N-2 (one of two sibling pointers repointed): ADDRESSED.** Task 5's Files list now
reads `src/views/EditorView.vue` ... "- Step 4c, half 2" beside `e2e/smoke.spec.ts`
... "- Step 4c, half 1". Same form, both naming the step that owns them, and
neither pointing at Step 5, which disclaims instructing. The asymmetry is gone.

**N-3 (the case-blind phrase instrument): ADDRESSED, figure confirmed.** See
section 2.

---

## 2. The three-site figure, and what it does to the earlier rounds

**Confirmed: three.** Run case-insensitively over the two normative documents,
`if the owner strikes` returns `decisions.md:87`, `findings.md:371` and
`findings.md:1178` - the third being Task 5 Step 4's Case 4 parenthetical, which
opens its sentence with "If". The case-sensitive form returns two. A fourth hit
exists in `docs/decision-ledger.yaml` and is the house entry recording this very
class, not a target.

**Retroactive effect: nothing rode on the short set, and I checked that two ways
rather than reasoning it.**

*First, inside the surface the rounds searched.* The three sites are exactly the
three sentences fix round 1 acted on. Site 1178 was reached by the sibling phrase
`nothing else moves` and was repaired; the case blindness therefore cost
redundancy, not coverage. The author's own framing is right - the conclusion held
by luck rather than by design - and I would sharpen it only by saying what the
luck consisted of: a three-expression union where one member was defective and two
were not.

*Second, outside it, because the instrument had a second hole the case fix does
not close.* Every round scoped these phrases to `docs/superpowers/plans/` and
`docs/superpowers/specs/`. I ran the corrected phrases over the whole of `docs/`.
Two further sensitivity-shaped sentences exist:

- `docs/ROADMAP.md`: "per the reviewer's confirmation that nothing else in the
  package depends on the outcome"
- `docs/process-journal.md`: "Nothing else in the package depends on it."
  (sentence-initial - the same case class, in a file no sweep covered)

**Neither is a missed target.** Both are about the close-path residual and D109
decision 5's re-read question, which became R42 and D109 decision 9 - not about
decision 2's New guard, which is the only thing amendment 2 moved. The ROADMAP
sentence is live and unaffected; the journal sentence is a dated open-threads
snapshot whose decision has since been ruled. So the ruling stands: **no
conclusion drawn before the correction is disturbed, and no set was one member
short in any way that mattered** - but the surface half of the instrument's
narrowness is still open, and I closed it here rather than leaving it as an
assumption.

---

## 3. The pair comparison and its inventory

**The Pair J repair is sound.** `Decision [0-9]+ carries it` now returns nothing in
either document, exit 1, with my synthetic control returning 1; both documents read
"The standing-guard decision above carries it" at the corresponding site. That is
the right repair rather than renumbering: the two documents legitimately number
D112's decisions differently (plan nine items, ADR seven), so a description is the
only form that is true in both. It also matches the treatment the plan already
chose for cross-references between the documents, and the finding is a real one -
each number was correct locally and the pair was contradictory only to a reader
holding both, which is precisely the class no expression finds.

**The inventory is complete. I derived it independently and got the same eleven.**
Enumerating the ADR-side changes across the four commits that touch it and grouping
them into distinct twinned statements yields: D107 decision 3(f)'s pointer; D107
decision 7's pointer; D109 decision 1's recents-gate reconciliation; D109 decision
2's sensitivity clause; D112's opening ruling paragraph; D112's supersession
paragraph against the plan's record-placement decision; D112 decisions 1 to 6;
D112's standing-guard decision; D112's uncovered paragraph; D112's rejected
alternatives; D112's Triggers created. Eleven, matching row for row.

**I also tested the generator's own blind spot, which the report asserts away
rather than measures.** "The ADR side of each commit names exactly the statements
that have a twin" is only true if no commit changed a plan-register statement while
leaving the ADR alone - such a change would have a twin and be invisible to an
ADR-hunk inventory. There is exactly one candidate commit, `dceca9b` (amendment 2
proper), which touched the plan and not the decisions document. Measured: it has
**zero** hunks inside the plan's Decision register, against 5, 4, 1 and 2 register
hunks for the four commits that do touch the ADR. So the generator misses nothing
here, for a reason that is measurable and was not measured.

**Two asymmetries the inventory does not name, neither of which hides a
divergence.** The list records one ADR/plan asymmetry as deliberate (Triggers
created) and is silent about two more: the decisions document's `**Rationale:**`
paragraph, which the plan's register has no counterpart to and which row G folds in
as supporting material; and the plan's "The number is measured, not assumed"
paragraph plus its decisions 7 and 8, which the ADR correctly does not carry. I
read the ADR's Rationale claim by claim against the plan - the future-funnel
derivation, the `openPath` ordering with its no-intermediate-frame consequence, the
`createBlank` ordering, and the restated parse-error measurement - and every one is
carried by a plan decision or rejected alternative without contradiction. So this
is a presentational shortfall rather than a completeness failure: an inventory
whose purpose is to record which differences are by design should record all three
asymmetries, not one.

---

## 4. Standing checks

Seven added lines. Typography 0 with my synthetic control returning 1; no latitude
clause in either form; requirement rows 43 and acceptance rows 73 unchanged; the
plan's own gate-part audit at one line, its own sentence; Task 5's Files list and
commit pathspec both six and identical; the commit carries the two tracked
documents, unsigned, one trailer.

**Executability against the tree the implementer will resume into**, which is what
release readiness turns on: Step 4c half 2's OLD fence is still present verbatim in
the working-tree `src/views/EditorView.vue` and its NEW fence is not, so that step
runs exactly as written; Step 4c half 1's end-state fence is already satisfied by
the implementer's own edit to `e2e/smoke.spec.ts`; and of Step 4b's three members,
two carry the repair in the tree and `createBlank resets` does not. The remaining
work is exactly two edits, both fenced, both owned by a named step.

---

## 5. Release

**Ready.** Every finding from three review rounds is addressed; this round
introduced no breakage; the criterion derives rather than lists and I applied it
cold to reach the same three members; the repair pattern is a measured property of
the one shared dialog rather than an assumption; both budget comments have a single
owning step with fenced texts checkable against the tree in either the fresh or the
resumed state; and the twin statements now agree in both documents. The three
carried items - the validation-response race, the ROADMAP `currentPath` write-site
correction and the 43-figure sites - are routed by the controller and gate nothing
here, and the author's recorded residual (the reciprocal pointer is prose at both
ends) is the honest answer rather than a defect.

---

## 6. Harvest

**The round validated the method it was invented for, and the method's own
generator is now the thing worth naming.** The pair check found something three
sweeps could not: two sentences each correct in its own document, contradictory
only to a reader holding both. That is a defect class with no textual signature -
"Decision 9" and "Decision 7" share no wrong word - so no expression over the
corpus can reach it, and no amount of widening a sweep would have. The general
form: **where one fact is written twice under two local numbering schemes, the
correct cross-reference is a description, never an ordinal**, and the check that
finds the violation is reading the two texts against each other.

**What still wants a handle is the inventory, not the comparison.** A pair check is
only as complete as the list of pairs it is run over, and this one was generated
from the hunks of one side. That is sound here - I measured that the one
plan-only commit touched nothing twinned - but the soundness is a property of these
five commits, not of the method. The durable version is symmetric: **generate the
pair list from the hunks of BOTH documents, intersected with the region that is
twinned**, so a change made on either side alone still produces a pair to compare.
It costs one more `git show` per commit.

**And a smaller one, on fixing half an instrument.** N-3's repair corrected the
expression's case sensitivity and left its search surface at two directories.
Both are ways for the same instrument to under-return, and only one was named as
the defect. When a search instrument is found wanting, the two questions are
"does the pattern match what I mean?" and "does it look where the thing could be?"
- and a round that answers only the first has fixed the half that happened to be
reported.
