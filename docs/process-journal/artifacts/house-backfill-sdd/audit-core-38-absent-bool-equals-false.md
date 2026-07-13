# Audit: core-38-absent-bool-equals-false (PROMOTION candidate)

**Cluster:** `core-38` | **kind:** pattern | **status:** settled | **claimed count:** 3 | **promoted:** yes (at 3)
**Statement (audited):** An absent boolean-typed matchable property compares equal to `false` for exact matching (mirrors mkvmerge/Matroska), so `exact:{flag:false}` matches a track lacking the flag; `:true` still does not. Scope is any boolean matchable, not just the four vanity flags.

**Verdict: CONFIRMED** - 3 of 3 occurrences survive. Promotion stands.
**verified_count = 3.**

---

## Counting rule applied

From `cluster-core.md` line 5 (the methodology this cluster was built under):
an occurrence = one distinct cited *artifact/attestation point*. The enumerated
valid types include "an independent review event", "a per-task review verdict",
and "an implementation commit-set" - each a separate occurrence. Explicitly
**kept distinct**: "a fix commit that follows a review is its own touchpoint
(find-vs-fix are two events)." Collapse conditions (same-doc sections;
co-cited commit-set for one work item; one review event cited via both its
verdict file and a journal mention) - none apply here.

The three occurrences map onto three of the enumerated distinct types, none
matching a collapse condition. So the count is real *under the stated methodology*.

## Per-occurrence verification

### Occurrence 1 - `independent review decision #1(b)` (kind: decided, 2026-07-09) - SURVIVES
`docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`
- Line 31, in the "Decisions (Şenol, 2026-07-09):" block: "#1 -> (b): absent
  boolean-typed matchable property compares equal to false for exact matching;
  mirror mkvtoolnix's own treatment." Verbatim-in-substance match to the cluster statement.
- Line 43 (Reviewer 1 finding, tagged `[-> decision #1(b)]`) reproduces the bug
  against real mkvmerge and states the steelman verbatim: "Reference profile 4.1
  dodges it via `not:[exact:{flag:true}]`." This is the exact `steelman` field of the cluster.
- Genuine independent-review event; kind `decided` is precise (it is the recorded Şenol decision). Not fabricated, not misattributed.

### Occurrence 2 - `F4 review (SPEC pass)` (kind: decided, 2026-07-09) - SURVIVES
`docs/process-journal/artifacts/plan-2-fixes-sdd/F4-review.md`
- Title + line 7 "SPEC: pass" / "QUALITY: approved". Independent per-task review verdict.
- Line 20 independently verifies the *scope* claim that distinguishes this cluster:
  "Spec 4.4 states the rule generically for any boolean-typed matchable property
  ... not narrowly for the four named vanity flags." That is exactly the cluster's
  "Scope is any boolean matchable, not just the four vanity flags." So this artifact
  attests not just the topic but the specific scope boundary in the statement.
- Distinct from occurrence 3: a per-task review verdict is an enumerated occurrence
  type separate from the commit it reviews. It reviews HEAD 213e1e9 (line 11) but is
  not the commit nor a mere journal mention of it. Not a duplicate.
- Distinct from occurrence 1: a different review event (SDD per-task verification of
  the fix) at a different lifecycle stage than the retrofit find-review. Not a duplicate.
- Caveat (labeling, not a drop): kind `decided` is loose - a SPEC-pass verdict is a
  *verification* attestation, not the decision itself. Does not meet any drop criterion.

### Occurrence 3 - `commit 213e1e9` (kind: decided, 2026-07-09) - SURVIVES
`git show 213e1e9` - "fix(core): absent boolean flags compare equal to false in exact matching", Thu Jul 9 03:55:12 2026.
- Real implementation commit-set (matcher.rs: `Some(PropType::Boolean) => scalar_eq(want, &PropValue::Bool(false))` in the `exact_matches` fallback; non-boolean absent unchanged). Three tests cover both polarities + non-boolean guard. Message cites "spec 4.4, decision #1".
- Enumerated occurrence type "implementation commit-set". F4-report.md line 9 binds F4 -> this commit; only one commit, cited once. Distinct from occurrence 2 (review-of-fix vs fix). Not a duplicate.
- Caveat (labeling): kind is really "implemented", not "decided". Not a drop criterion.

## Skeptical notes (do not change the verdict, but bound its strength)

- **All three occurrences are the same decision #1(b), same date (2026-07-09),
  same plan-2 / plan-2-fixes pipeline.** This is lifecycle-stage counting
  (decide -> implement -> review), not recurrence across independent plans. The
  cluster header describes the strongest recurrences as "touched across three or
  more plans" (line 11); this one is the minimum-strength promotion: one decision
  documented at three lifecycle stages. Under the methodology's explicit
  find-vs-fix + review-verdict + commit-set enumeration it legitimately reaches 3,
  but any reviewer-found-and-fixed-and-reviewed SDD item mechanically produces 3
  attestation points, so "3" here is the floor the methodology allows, not evidence
  of the pattern resurfacing in unrelated contexts.
- No occurrence is fabricated, misattributed, or a duplicate of another - the only
  three drop criteria. The `evidence` fields being `null` is a records gap, not a
  fault: each ref was independently located and confirmed.

**Conclusion:** count of 3 is real under the stated counting rule; no fabricated
or duplicated recurrence. Promotion to standing house-knowledge stands.
