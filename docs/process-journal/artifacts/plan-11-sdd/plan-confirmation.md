# Plan 11 - reviewer confirmation of the round-4 fix round

Reviewer: the resumed original plan reviewer (sixth pass). This is a **narrow
confirmation pass**, not a review: scope is finding 1, the four members the
author's attached sweep found, the hardening clause, finding 2, and a ruling on
the remaining hits. Nothing I confirmed in `plan-delta-review-round-4.md` is
re-examined - not the 27 VOID members, the 21 UNAFFECTED, the ten cross-arm
sites, T-1's mandatory-ness, the four byte-identical tasks, or any figure.

Artifact: the working-tree state of
`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
(925 lines, uncommitted), against `6a564bc` (923 lines), the version I graded.
Diff: 8 insertions, 6 deletions - seven changed content lines. Instruments under
`.../round6/`.

**The controller was right to ask for this pass.** My round-4 signal was a
pre-commitment to approve on a one-clause deletion. What landed is five sentence
repairs, a hardening clause and a scope correction, four of the five inside the
block A3's Read-first designates as ground truth. That is more than my condition
covered, and the highest-stakes text in the plan should not ride in on a promise
made about something smaller.

---

## Verdict: APPROVED

**Plainly, for the gate: I approve the plan at this state. Execution may begin on
this signal plus the owner's already-given approval.** All five repairs are
correct, the hardening clause says what I ruled rather than a softened version,
finding 2 is corrected in the artifact and not only in the report, no further
compression happened, and my own wider re-derivation of the remaining hits finds
**zero** assertions of a retired fact in the plan's own voice. I recommend no
safeguard removals.

---

## The five dispositions

**1. Finding 1 - CONFIRMED FIXED.** The bullet now closes at "The measurement is
stronger than the tracker's 'deliberate-looking'." - exactly where I said it
should end - and the front of the sentence keeps the VOID marker, the owner's
date, the D111 §5 pointer and the surviving measurement, all of which I said not
to touch. `grep -c 'owes no new test'` returns **1**, at L875, and that survivor
is the amendment record my round-4 table already ruled legitimate. **I
fire-tested the near-zero rather than trusting it:** appending a synthetic
occurrence to a scratch copy takes the count to 2, so the 1 is a measurement and
not a broken pattern.

**2. L118's tail - CONFIRMED FIXED, and I agree it was the strongest instance.**
It closed as a bare present-tense assertion of precisely what D111 inverts, in
the ground-truth block: "So the comparison is byte-exact for strings and NUMERIC
for numbers, and 'byte-exact' is false exactly where a number is involved." It
now reads "On the pre-state the `raw:` arm of `exact_matches` **called** it
directly, which is what **made** the comparison numeric for numbers and
'byte-exact' false wherever a number **was** involved. **After D111 that arm
calls `scalar_eq_same_type` and numbers no longer compare across kinds on the
`raw:` path**; the sentence above describes the defect this task removes, not the
behaviour it ships." Past-tensed, closed with what ships, and self-disclaiming.
Had this been found in round 4 I would have raised it at the same severity as
finding 1.

**3. L124's mechanism - CONFIRMED FIXED.** The call-site sentence now names both
comparators with their states: "hands the reported `PropValue` straight to the
comparator - `scalar_eq` on the pre-state, `scalar_eq_same_type` after D111
re-points the call site." The bullet's closing conclusion carries a
**PRE-AMENDMENT conclusion, VOID:** marker and states the inverse plus R-6's
move. I searched the whole line for a residual present-tense call-site claim and
found none - the only apparent second occurrence is my own regex splitting on the
period inside `item.get(bare)`.

**4. L891's counts consumer - CONFIRMED FIXED.** "now 37 after this round's
additions" is now "40 after this round's and the D111 fold-in's additions",
matching the 40 I recounted in round 4 and the split stated two clauses earlier.
This one is worth naming for what it was rather than for its size: a stale
consumer figure inside the counts list the author had just edited, which is the
consumer-versus-source shape one level further in than the round-3 instance.

**5. L123's splice - CONFIRMED FIXED, both halves.** `pre-state record., and`
is now `pre-state record, and`, and the present-tense scope claim "every one of
them **is** scoped to `language`/`codec_kind`" is now "**was** scoped". I swept
for other doubled-punctuation splices of that shape and found none.

---

## Ruling on the remaining hits

I re-derived the set my own way rather than checking the author's six. My
expression targets the inverted facts in the plan's own voice - present-tense
copulas with the retired predicates, the numeric-comparison claims, the
`6.0`-matching examples, the retired figures, and `SIX arms` - and it is
deliberately wider than a marked-span check, because the whole class turned on
sentence TAILS rather than marked spans.

**It returns 15 hits, and all 15 pass the test.** More than the six reported,
because the net is wider; no disagreement.

| kind | count | members |
|---|---|---|
| Past-tensed record or explicitly named inversion | 7 | L118 ("**had** SIX arms ... the PRE-state"), L122 ("**was** six lines ... D111 makes it twelve"), L123 ("**was** nine lines ... D111 makes it seven"), L124 (**VOID** marker + "They state the opposite"), L463 ("the ruling **INVERTS** its recorded disposition"), L787 ("**was INVERTED** by the owner"), L875 (the amendment record) |
| 13.4-protected UNAFFECTED test-duty paragraphs | 2 | L347 (A1), L763 (B1) - each still grounded on its own task producing no user-visible consequence |
| Different subject, not in the class | 4 | L226 and L776 (B1's `Cargo.lock`/`package.json` unchanged), L147 (`BUILDING.md`'s nine over-80 lines), and the corrections-table continuation at L166, whose row carries its supersession and which I ruled a dated record in round 4 |
| True present-tense statement about something else | 2 | L881 (the README step "is unchanged in substance" - finding 2's own correction), and the L166 row's figure with its stated supersession |

**Zero assertions of a retired fact in the plan's own voice. The class is
closed.** Applying the distinction to tails rather than to marked spans is what
closed it, and that is the transferable part.

---

## The hardening clause

**CONFIRMED, and it is what I ruled rather than a softened version.** At L562 it
states that D111's fences are the **SINGLE SOURCE** of the twelve replacement
strings and that transcribing them into the plan is **deliberately NOT wanted**,
and it carries every element I gave: both preconditions (each site names its own
`R-n`; each `R-n` is defined under its own heading in section 4.3, so the lookup
is deterministic rather than a search), the asymmetric-failure ground in the form
I ruled it ("**nothing in this task compares the applied text against D111**", so
a drifted duplicate leaves every check green while the wrong sentence lands in
the v1 spec, the README and two shipped help topics - silent, green and
user-visible), the contrast that a paraphrase fails loudly by showing up as text
matching no fence, the explicit "Do not paste the fences in 'for the
implementer's convenience'", and the note that this is the class the project has
paid for four times.

**On placement: adequate, and I would not move it.** It sits as its own bolded
paragraph immediately above A3's "Must not decide", inside the task whose fences
it governs. A fix round that wanted to paste the fences in would be editing A3's
Steps 2 to 5 and would read the task to its close; Must not decide is the list an
agent scans before changing anything normative, and this sits directly on top of
it. Being *inside* that enumeration would be marginally more prominent but could
not carry the reasoning, and the reasoning is what makes the rule survive a
disagreement. The two halves are also covered separately: Must not decide already
forbids composing the strings ("every fenced replacement R-1 to R-12, in both
natural languages"), and this paragraph forbids duplicating them.

---

## Finding 2 and compression

**Finding 2 - CONFIRMED corrected in the artifact.** The "Deliberately
unchanged" line now states that all four other tasks are byte-identical with zero
changed lines each, and separately that A3's README-example step is unchanged in
substance and listed as untouched by D111 13.5 but is **not** byte-identical -
condensed from 22 non-blank lines to 18 - with the reason given ("a scope report
is what a reviewer is invited to trust instead of re-deriving"), what survives
enumerated, and the three relocated claims named with their new homes. That is
the correction, in the place a later reader will look.

**No further compression - CONFIRMED by measurement, not by reading.** Comparing
per-section non-blank line counts between the version I graded and this one:
**zero sections shrank.** Total non-blank goes 658 to 659, the single increase
being A3 at 77 to 78 for the hardening clause. Meta-text share is flat at 23%.
The line also records that the close-time compression recommendation stands and
that no condensation is done here, which is the disposition I asked for.

---

## One thing new, and it is against my own instrument

My first diff filter for this pass, `^[+-][^+-]`, silently missed the four
changed lines in the ground-truth block, reporting 5 changed lines against a true
14. The cause is that a deleted markdown bullet reads as `-- **The coercion...`,
so the second character is the very class my pattern excluded to skip `+++`/`---`
headers. I caught it only because `git diff --numstat` disagreed with my count.

That is the same defect class this review has spent five rounds on - an
expression whose own exclusion set hides a member - and it is worth a harvest
line because every agent that diffs markdown will meet it: **filter diff headers
by the three-character prefix (`+++ `/`--- `), never by "the second character is
not a sign", or bullets and `--flag` lines vanish.** The cheap guard is the one
that caught it here: cross-check any hand-rolled diff filter against
`--numstat` before believing its count.

Nothing in the artifact is affected.

---

## Harvest for the controller

1. **The tail rule, which is the transferable output of this round.** A
   void-marked passage is repaired to its final period, and the distinction that
   decides each hit is *record versus assertion in the document's own voice*
   applied to sentence tails, not to marked spans. Four of the five repairs here
   were tails on lines whose fronts had already been marked VOID.
2. **A markdown-aware diff filter.** See above; the failure is silent and
   produces an undercount that looks like a small, safe change.
3. **The single-source ruling now has its clause and its placement precedent**:
   the reasoning lives in a paragraph above Must-not-decide, and the enumeration
   forbids composition. Both halves are needed - one bans inventing the string,
   the other bans copying it.
