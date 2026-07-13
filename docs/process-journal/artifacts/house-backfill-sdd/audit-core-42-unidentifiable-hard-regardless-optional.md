# Audit: core-42-unidentifiable-hard-regardless-optional (PROMOTION candidate)

**Cluster:** `core-42-unidentifiable-hard-regardless-optional` (kind `pattern`, domain `core`)
**Claim:** count 3, promoted at count 3, status `settled`.
**Statement under audit:** A present-but-unidentifiable source is a hard
`UnidentifiableSource` error regardless of `rule.optional` (optional means "zero
matches acceptable", not "a broken file is acceptable"); spec 5.2 makes it
unconditional. The fix-plan draft that said donor-identify-failure should respect
optional was superseded and the plan text reconciled to shipped behavior.

**Method:** each cited occurrence opened in its source artifact and checked against
"this (topic, approach) arose here as {occ.kind}". Drop if fabricated, misattributed,
or a duplicate of another listed occurrence.

**Verdict: REJECTED** - only 2 distinct occurrences survive (occurrences 1 and 3 are
the same reconciliation event captured twice). The statement is *substantively true*,
but the recurrence *count is not real*: it was padded by double-booking one commit as
both "the plan text" and "the commit that wrote that plan text." Demote to Tier 1.

---

## Occurrence 1 - decided, 2026-07-09, "fix plan F5 text (superseding draft)" -> DROP (duplicate of occurrence 3)

**Artifact:** `docs/superpowers/plans/2026-07-09-plan-2-fixes.md:34` (F5 task,
identify-failure bullet), in its current, post-reconciliation state.

**Verbatim support (line 34):**
> A present-but-unidentifiable file is a HARD error regardless of `rule.optional`
> (optional means "zero matches is acceptable", not "a broken file is acceptable";
> spec 5.2 UnidentifiableSource is unconditional error) - decided in the F5 dispatch,
> superseding an earlier draft line here that said it should respect optional.

The text supports the topic exactly. **But this wording did not exist as an
independent artifact.** `git log -L 34,34` on the file shows the line reached this
state through exactly one commit - **953c5cd** - and no other:

- `847b476` (plan created): the line said the **opposite** - "Donor-identify-failure
  must respect `rule.optional` consistently with the zero-hits branch."
- `953c5cd` (the only subsequent edit to this line): rewrote it to the hard-error
  wording quoted above.

So "fix plan F5 text (superseding draft)" is not a second, independent occurrence -
it **is** the output of commit 953c5cd (occurrence 3). Same file, same line, same
2026-07-09 reconciliation, same `decided` kind. Before that commit the plan text
contradicted the topic; the topic entered the plan text solely *via* 953c5cd. Two
refs pointing at one event = a duplicate. Dropped, keeping the commit (occurrence 3)
as the canonical dated record.

## Occurrence 2 - decided, 2026-07-09, "F5 review Important #2" -> SURVIVES

**Artifact:** `docs/process-journal/artifacts/plan-2-fixes-sdd/F5-review.md`, section
**Important #2** (lines 99-130): "Delivered donor-identify-failure behavior
contradicts the plan doc's own F5 instruction, unreconciled."

**Verbatim support (lines 120-128):**
> The master spec catalog (`2026-07-08-muxsmith-v1-design.md` 5.2) backs the shipped
> behavior: `MissingTrack`/`MissingExternal` explicitly say "non-optional" in their
> condition text, `UnidentifiableSource`'s row does not mention `optional` at all,
> and 4.8 talks about hard errors "regardless of policy" in the same spirit. So the
> delivered behavior is very likely the actually-intended one - but the plan doc ...
> was never corrected to match ... a future pass that takes the plan doc's F5 line at
> face value could "fix" this back to optional-gated and silently reintroduce a spec
> violation.

Genuine, distinct artifact (a review file, written before the reconciliation commit;
the review is what surfaced the unreconciled contradiction and is presumably what
prompted 953c5cd). Supports the topic directly, including the exact spec-5.2 backing
the statement cites. Not fabricated, not misattributed, not a duplicate of any other
listed occurrence.

*Kind nit (non-blocking):* the listed kind is `decided`, but Important #2 is a review
**finding/recommendation** that flags the contradiction as *unreconciled* rather than
enacting a decision. It substantively endorses the shipped behavior as spec-correct
("very likely the actually-intended one"), so it legitimately counts as an occurrence
of the topic - but its more accurate kind is `review`/`reinforced`, not `decided`.
This does not change the outcome; even fully credited, it is only the second distinct
occurrence.

## Occurrence 3 - decided, 2026-07-09, "commit 953c5cd" -> SURVIVES

**Artifact:** commit `953c5cd` - "docs: reconcile F5 plan text with the shipped
optional-independent behavior."

**Verbatim support (the commit's entire diff):** a one-line change to
`docs/superpowers/plans/2026-07-09-plan-2-fixes.md:34`, replacing "Donor-identify-
failure must respect `rule.optional` ..." with the hard-error wording quoted under
occurrence 1. `git show --stat`: `1 file changed, 1 insertion(+), 1 deletion(-)`.

Supports the topic and the `decided` kind (this is the act that reconciled plan to
shipped behavior). Kept as the canonical representative of the reconciliation event.

---

## Distinctness check (the crux)

Two distinct events, not three:

| Event | Listed as | Distinct? |
|---|---|---|
| F5 review surfaces the unreconciled contradiction, endorses spec-5.2 hard-error reading | occ 2 (`F5-review.md` Important #2) | yes |
| Plan F5 line reconciled to hard-error wording | occ 1 (the resulting plan text) **and** occ 3 (commit 953c5cd, which produced that text) | **one event, double-booked** |

`git log -L 34,34:docs/superpowers/plans/2026-07-09-plan-2-fixes.md` proves occ 1's
wording exists only as commit 953c5cd's output - there is no independent "superseding
draft" artifact apart from that commit. Occurrences 1 and 3 collapse into a single
recurrence. verified_count = **2**.

## Why this matters (the audit's whole point)

A genuine third occurrence of this topic **does exist in the repo but was not cited**:
`docs/process-journal/artifacts/plan-2-fixes-sdd/F5-report.md` (lines 42-48, 62-69) is
where the F5 *dispatch* actually decided the hard-error behavior and locked it with a
dedicated test (`unidentifiable_donor_yields_unidentifiable_source_not_missing_external`,
asserting `optional: true` still yields no plan). Had the cluster cited that as the
third occurrence, the arc would be three genuinely distinct artifacts - dispatch
decision (F5-report) -> review confirmation (F5-review Important #2) -> plan-text
reconciliation (commit 953c5cd) - and the count would be real.

Instead the occurrence list padded the count by recording the single reconciliation
twice (plan text + its commit). That is exactly the fabricated-recurrence failure mode
this audit exists to catch: the *statement* is correct and spec-backed, but the
promotion's evidentiary basis (three independent recurrences) is not met by the refs
as listed.

## Bottom line

verified_count = 2. Occurrence 2 (F5 review) and the reconciliation event
(occurrences 1 == 3) are the only distinct recurrences among the cited refs;
occurrence 1 is a duplicate of occurrence 3. 2 < 3 -> **REJECTED**: demote to Tier 1.

**Not a demotion of the claim's truth** - the behavior is correct and spec-5.2-backed.
It can be re-promoted cleanly by relisting the third occurrence as the uncited
`F5-report.md` (F5 dispatch decision + test), which is a genuinely distinct artifact,
instead of double-counting commit 953c5cd's plan-text edit.
