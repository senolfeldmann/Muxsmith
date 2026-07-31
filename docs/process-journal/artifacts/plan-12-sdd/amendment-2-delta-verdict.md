# Amendment 2, fix round 1: delta verdict

**All five findings ADDRESSED. B-5 re-graded MET.**
**New breakage: 1 Important, 2 Minor.**

The Important is in the pair I was asked to check as a pair: the rewritten
sensitivity clause says two different things in the two documents, and the
decisions-document form contradicts its own opening sentence.

Instruments at `/tmp/.../scratchpad/amd2-delta-pruefung/`, a path none of my
earlier passes used. Nothing in the repository was written except this verdict;
no stash, checkout, reset, restore or clean was run, and the same six paths of
Task 5's paused work are unchanged after my pass.

---

## 1. Per-finding verdicts

**I-1 (the falsified sensitivity clause): ADDRESSED, and it introduced N-1 below.**
Both documents now state the correct operative content - striking D109 decision 2
moves two tests (`e2e/smoke.spec.ts`'s New case and `createBlank resets` in
`e2e/editor-undo-redo.spec.ts`) plus a third dependent that is not a test, the
membership criterion's guarded pair - and Task 5 Step 4's Case 4 parenthetical no
longer says "nothing else moves". Measured post-fix over
`docs/superpowers/plans/` and `docs/superpowers/specs/`: `the only change is`
returns nothing at all, `nothing else moves` returns only the out-of-family
plan-6 note, `nothing else in the package depends` returns one hit per document,
each inside the sentence that records its own staleness. The falsified original
stays legible: both forms quote it as "one test and 'nothing else in the package
depends on it'". The pair is not identical where it matters - see N-1.

**I-2 (the sibling budget comment left without a live owner): ADDRESSED, and the
fence form is right for the situation.** I checked both halves the controller
named.

*The fence form.* Step 4c half 1 fences an end state rather than an
old-to-new replacement, and I verified that against both tree states rather than
against the argument: the fenced three lines are present **verbatim** in the
working-tree `e2e/smoke.spec.ts` (the resumed state, already at 54) and **absent**
from the same file at HEAD (still at 51). So a resumed implementer finds the
condition already met and checkable, and a fresh run finds it unmet and must
reach it. A replacement fence keyed on the old figure would have matched neither
the resumed tree nor a re-run after Task 5's Step 4 touched the file. The choice
is correct and the reasoning is verifiable, not merely plausible.

*The competing-owner half.* Step 5 now reads "This step asserts that they were
corrected; **Step 4c is the only place that instructs it**". That is explicit
enough that an implementer cannot read Step 5 as a second owner: it names the
owner and disclaims itself in the same sentence. Inside Task 5 the references
resolve one way only - Step 4c's heading, Step 5's two pointers to it, and the
`Must not decide` entry "Step 4c is their single owner". The remaining crack is
the Files-list pointer for the other file, N-2.

**M-1 (the criterion's unnamed staleness boundary): ADDRESSED.** Step 4b gains a
closure clause and the pointer is genuinely reciprocal. Both ends name the other:
Step 4b names "D109 decision 2's sensitivity clause"; the plan-side clause names
"Step 4b's criterion"; the ADR-side clause names "the plan's membership criterion
for the cases this guard fronts" - descriptively rather than by step ordinal,
which is right for a spec document and consistent with this plan's own recorded
rule against ordinals that stale. **The closure condition names the real event,
not a paraphrase:** the criterion is closed over the set of functions Step 2
guards, D109 decision 1 is the owner's ruling and carries no strike route, and
decision 2 is the one the plan itself flags as the author's with a strike clause
attached - so "the one documented event" is accurate rather than rhetorical. It
is also stated as "if the guarded pair ever **changes**", which covers an addition
to the pair and not only the documented removal. The author's own concern 4 (prose
cannot guard its own truth) is the honest residual and I agree with it.

**M-2 (B-2's wrong ground): ADDRESSED, and the corrected ground is right at the
code.** The plan now says `validate_profile_model` **does** fire after a confirmed
New and that both commands are already in the shared helper's mock set, so the
operative word is *additional*. I verified the mechanism rather than the sentence:
`createBlank` calls `resetHistory(profile)` before `model.value = profile`, so
`savedSnapshot` is set and `sessionActive` is true when `watch(model)` flushes,
and the watcher therefore runs its `validateProfileModel` call; `openProfile`'s
mock set carries `validate_profile_model` alongside `plugin:dialog|open`. The
plan also now warns that an implementer reading the old ground could raise a
question against a call that is already covered, which is the residual the
correction owed.

**M-3 (the imprecise per-case exclusion): ADDRESSED.** "activate no guarded
control at all" became "activate no guarded control **beyond that shared helper's
own Open**, which is the clean activation classified in the clause before this
one". The two sentences now agree, and the helper's single activation is
classified exactly once.

**B-5 (the sweep): re-graded MET.** I ran the three phrases myself rather than
grading the account of them, and then asked the question that decides it.

The round ran six shapes, not one: the three function-shaped phrases, a
consequence-counting shape (16 hits), a term-derived shape (18 hits), the counts,
the gate-part audit and typography - and it **read** the hits rather than tallying
them, disposing of each out-of-family sense by name. My own recomputation agrees
on every figure: 43 requirement rows, 73 acceptance rows, Task 5's Files list and
commit pathspec both six and identical, the gate-part audit at one line (its own
sentence), typography zero over the 25 added lines with my synthetic control
returning 1, no latitude clause in either form, no self line-number citation.

**Is it complete or merely wider?** Wider, genuinely - F2 and F3 are
discovery-capable shapes that did not exist last round, and F2 is the one that
would have caught last round's misses without being told where to look. It is not
complete, and the two defects that escaped it show why in a way that argues *for*
the MET rather than against it: **neither is expressible as a search.** N-1 is two
sentences the round itself wrote that disagree with each other, which no sweep for
falsified statements can surface, because both are new. N-2 is a pair of
cross-references where one was updated and the sibling was not, which carries none
of the words any phrase or term expression could key on. The shape that finds both
is a pairing check - "for every thing I touched that has a twin, diff the twin" -
not an enumeration. B-5 asked for the sweep and the sweep was run properly and
honestly, including two disclosed instrument defects of the author's own. Holding
it open for a class no expression can express would be moving the bar; the two
defects are findings in their own right instead.

---

## 2. New breakage

### Important

**N-1. The two rewritten sensitivity clauses do not say the same thing, and the
decisions-document form contradicts its own opening.** Both were written by this
diff. Their operative content matches and is correct. Their closing sentence does
not:

> **plan:** The original clause said one test and "nothing else in the package
> depends on it"; both were true when written and **both were falsified by
> amendment 2** ...
>
> **decisions document:** The clause originally said one test and "nothing else in
> the package depends on it"; both were true when written, and **the second is the
> claim that went stale** ...

The plan is right. The original made two claims - that one test moves, and that
nothing else depends on the decision - and amendment 2 falsified both: it added
`createBlank resets` to Task 5's repaired set, which is why the same ADR sentence
opens by saying **two** tests move. So the ADR's closing clause is not only
divergent from its twin, it is refuted two clauses earlier by itself.

*Steelman, because there is one:* you could argue the count merely tracked the
package while the closure claim was the load-bearing promise, so "the second is
the claim that went stale" is a judgement about which failure mattered. That
reading does not survive the divergence - the plan says "both", the ADR says "the
second", and a reader with one document has no way to know the other disagrees.

*Why it is Important rather than Minor.* It lands in the append-only decisions
document, where the whole purpose of the restated clause is
`proc-supersede-never-overwrite`'s "the reversal stays reconstructible from the
files alone - who ruled what, when, and why the first ruling lost". An inaccurate
"why", self-refuted in its own sentence, is the one defect that record cannot
carry, and correcting it later costs a further restatement rather than an edit.
The fix is one clause.

### Minor

**N-2. The fix round repointed one of the two Files-list entries and left its
sibling.** Task 5's Files list now reads:

- `e2e/smoke.spec.ts` ... **the catalog-budget comment, recomputed - Step 4c, half 1**
- `src/views/EditorView.vue` ... **and the stale catalog-budget sentence in the file's own header doc block** - amendment 2, **Step 5**

The instruction for the EditorView sentence lives in Step 4c half 2. Step 5 is
the step that now explicitly disclaims instructing anything. The pointer predates
this diff (amendment 2 wrote it), so it is not new breakage in the strict sense -
what is new is the asymmetry, created by updating the neighbour and not this one,
in the round whose entire lesson is that an edit walks to its neighbours. It
resolves in one hop, because Step 5 forwards to Step 4c by name, which is why it
is Minor rather than a reopening of I-2.

**N-3. One member of the three-phrase instrument is case-blind, and that was not
noted.** `grep 'if the owner strikes'` as run returns two sites; run
case-insensitively it returns three - the third is Task 5 Step 4's Case 4
parenthetical, which begins the sentence with "If". The union with `nothing else
moves` covered it, so the round's conclusion is unaffected and the reported "three
hits inside this plan family" is correct as a union. But a phrase-shaped
instrument aimed at prose has to survive sentence-initial capitalisation, and this
one does not; that it was carried by a sibling expression is luck rather than
design.

---

## 3. Standing dimensions

Typography 0 over the 25 added lines (synthetic control returns 1); no latitude in
either form (control fires); no self line-number citation; counts unchanged at 43
and 73; Task 5's Files list and commit pathspec both six and identical; the plan's
own gate-part audit at one line, its own sentence; commit is the two tracked
documents, unsigned, one trailer; the six uncommitted paths of Task 5's work
untouched before and after. The ADR edit is a marked restatement that quotes the
falsified claim rather than deleting it, which is the house form.

I also swept for anything else this round's edits falsified outside Task 5: Task
4's own Files entry for `e2e/smoke.spec.ts` ("the catalog-budget comment only",
which takes it to 51 before Task 5 takes it to 54) stays true; the plan close's
"two stale in-tree budget comments" stays true; the safeguards paragraph's "each
of the three repaired Task-4 cases" and "Step 4b's membership criterion itself"
stay true. Nothing else moved.

---

## 4. Harvest

**The round closed a class and then produced its two remaining instances by hand.**
The lesson the previous verdict harvested was "an edit to a set walks to its
neighbours". This round walked to the neighbours it was told about and, in the
same commit, wrote two sentences that disagree with each other (N-1) and updated
one of two sibling pointers (N-2). Both are the same shape one level in: **not a
neighbour that was missed, but a twin that was touched and not compared.**

**The handle, and it is mechanical rather than a matter of care.** A sweep answers
"what else says this?" A pair check answers "do the two things I just wrote still
say the same thing?" They are different operations and only the first has ever
been run in this plan family. The trigger is readable at the keyboard: **you are
editing a statement that exists in two documents, or repointing one of two
sibling references.** The handle is to diff the two artifacts against each other
afterwards - literally, sentence by sentence - rather than to run an expression
over both. Every clause-in-two-documents edit in this package (D107's supersession
pointers, D109 decision 2, D112's decision 1) has been a candidate for this and
none has had it.

**Second item, smaller: step ordinals repeat across tasks in this plan and are now
being used as cross-references.** "Step 4b" and "Step 4c" each name two different
things depending on which task you are in, and D109's sensitivity clause - which
sits in the Decision register, far from either task - now points at "Step 4b's
criterion". It resolves, because only one Step 4b has a criterion, but it resolves
by content rather than by address. The decisions document's own form of the same
pointer avoids the ordinal entirely and is the better pattern; the plan's own
recorded rule against ordinals into a decision list applies to step ordinals for
exactly the same reason.

**Carried forward, unchanged and still unrouted:** the validation-response race,
the ROADMAP `currentPath` write-site correction, and the two stale 43-figure sites
in the editor widget files - of which `StringListWidget.vue`'s is the one worth a
vehicle, since its stale premise sits under a design decision rather than beside
a count.
