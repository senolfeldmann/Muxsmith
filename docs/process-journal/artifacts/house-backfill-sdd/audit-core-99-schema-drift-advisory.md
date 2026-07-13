# Adversarial audit: core-99-schema-drift-advisory (PROMOTION candidate)

- **Cluster:** `core-99-schema-drift-advisory` (kind pattern, domain core, status settled)
- **Claimed:** count 3, promoted at 3
- **Audit date:** 2026-07-13
- **Verdict:** CONFIRMED (3 distinct occurrences survive; promotion stands)

## Method

Each of the three cited occurrences was opened at its authoritative artifact in
`/home/senol/Git/Muxsmith` and checked against the criterion "this (topic,
approach) arose here as `{occ.kind}`". Duplicate / fabricated / misattributed
occurrences would be dropped; `verified_count` = surviving distinct
occurrences.

The topic is a single, coherent thread: the newer-mkvmerge-schema advisory,
carried through its defer -> decide -> ship lifecycle.

## Occurrence-by-occurrence

### Occ 1 - 2026-07-11 `deferred` (task-16 verdict, per-file warner dropped, open question)

**Artifact:** `.superpowers/sdd/plan-5.5/task-16-verdict.md`

**Supports the claim.** The verdict body states verbatim: "The old
planner.rs:445 emission was a per-file SCHEMA-DRIFT ADVISORY (fires on every
newer-schema file ... 50 identical warnings on a 50-file batch)"; "D32's 'was
dead code' premise was FALSE"; and "VERDICT: open-question-for-Şenol, narrowly
scoped: should a schema-drift advisory (once per batch, own diagnostic)
survive, or is dropping it acceptable?"

This matches the cluster statement's three load-bearing claims for this stage:
per-file warner removed, false "dead code" premise, and the deferred open
question of whether any advisory survives. Kind `deferred` (open question routed
to Şenol) is exactly right. Corroborated by `progress.md:207-212` (T16 COMPLETE,
"TWO Şenol ratifications PENDING (memo D32 addendum...): schema-drift advisory
drop", and "Review also validated the implementer's stale-premise catch (live
:445 emitter, memo said dead code)").

**Minor date discrepancy (not disqualifying):** the cluster dates this
2026-07-11, but the verdict file self-heads "model: opus, 2026-07-12" and its
mtime is Jul 12 01:40. The underlying question was in fact raised in the D32
design round (`d32-analysis.md`, mtime Jul 11 22:14) on 2026-07-11 and
adjudicated in the T16 verdict just past midnight. So 2026-07-11 is defensible
if anchored to the design round, ~2h off if anchored to the verdict file. This
is a labeling nuance across midnight, not a fabrication or misattribution; the
ref unambiguously supports the occurrence. **SURVIVES.**

### Occ 2 - 2026-07-12 `decided` (memo D32 addendum RESOLVED / plan T16.5)

**Artifacts:** `.superpowers/sdd/plan-5.5/task-16.5-brief.md` +
`.superpowers/sdd/plan-5.5/progress.md:241-245`

**Supports the claim.** The compound ref has two components, both real:

- *memo D32 addendum RESOLVED:* the ratification item was recorded post-T16 as
  a pending "memo D32 addendum" (`progress.md:208`), then resolved at the Şenol
  gate: "ŞENOL GATE PASSED (2026-07-12): ... skew notice REBUILT once-per-batch
  (T16.5)" (`progress.md:241-245`).
- *plan T16.5:* `task-16.5-brief.md` header "Task 16.5: schema-drift batch
  notice (added 2026-07-12, Şenol decision at the D32-addendum gate)", body:
  "rebuild it as its OWN diagnostic, ONCE PER BATCH (not per file, the old
  mis-scoping), info severity, message includes the raw: discovery hint".

This is the decision event proper - Şenol ruling the advisory should survive and
specifying its rebuilt shape. Kind `decided` is correct. Distinct in time,
artifact, and kind from Occ 1 (the deferral) and Occ 3 (the shipping review).
**SURVIVES.**

### Occ 3 - 2026-07-12 `reinforced` (task-16.5 verdict, shipped)

**Artifact:** `.superpowers/sdd/plan-5.5/task-16.5-verdict.md`

**Supports the claim.** Spec-compliance section: "own DiagCode SchemaDrift info;
params found_version(max)+pinned; once per batch (single site outside the loop,
Option-guarded); raw: hint bilingual ...". Assessment: "Approved (after the
doc-lockstep fix)." Corroborated by `progress.md:250-252` (T16.5 COMPLETE,
a86eecb + doc fix ce4fae1, MERGED). This is the implementation-review event
confirming the decision was executed as specified - kind `reinforced`
(shipped). It also sources every technical detail in the cluster statement's
final clause (single emission site outside the loop, `found_version(max)` +
`pinned` params, Option-guarded). **SURVIVES.**

## Duplication / fabrication check

- **No fabrication:** all three refs resolve to real, on-disk artifacts whose
  text supports the claimed topic and kind.
- **No misattribution:** each artifact is about exactly this schema-drift
  advisory, not a neighboring diagnostic. (Note core-100-schemadrift-primaries-only
  correctly forks off Occ 3's *adjudication 1* as a separate cluster; core-99
  does not double-count that sub-ruling.)
- **No mutual duplication:** the three are distinct pipeline stages (defer ->
  decide -> ship) with distinct artifacts, timestamps, and kinds. None restates
  another.

## Honest caveat (does not change the verdict)

The three occurrences are the *lifecycle stages of one decision*
(open-question -> Şenol ruling -> shipped review), not three independent
re-derivations of the pattern in different contexts. A stricter "genuine
recurrence" reading might weight lifecycle-of-one-decision below
independent-rediscovery. But the promotion rubric here counts distinct,
supported, non-duplicate occurrences, and under that rubric all three stand.
The pattern is also genuinely settled house knowledge (own DiagCode, once per
batch, info severity, raw: hint) and matches the shipped code as reviewed, so
promotion to a standing convention is substantively warranted, not merely
count-passing.

## Result

- Occ 1: SURVIVES (minor cross-midnight date label noted)
- Occ 2: SURVIVES
- Occ 3: SURVIVES
- **verified_count = 3**
- **Verdict: CONFIRMED** - >= 3 distinct occurrences survive; promotion stands.
