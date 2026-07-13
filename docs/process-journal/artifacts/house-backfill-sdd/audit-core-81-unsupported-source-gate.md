# Audit: core-81-unsupported-source-gate (PROMOTION candidate)

- **Cluster id:** core-81-unsupported-source-gate
- **Kind:** pattern / domain: core
- **Claimed count:** 3 | **promoted:** true | **promoted_at:** 3
- **Verdict:** CONFIRMED (3/3 occurrences survive; promotion stands)
- **Audited:** 2026-07-13, against `/home/senol/Git/Muxsmith`

## Statement under audit

> A source mkvmerge identifies (exit 0) but cannot mux emits one clear
> UnsupportedSource error (distinct code from UnidentifiableSource, different
> remediation) before rule resolution and skips the file, instead of per-rule
> MissingTrack noise. The gate fires on `!container_recognized ||
> !container_supported` only, not `is_identifiable()`; a recognized+supported
> zero-track container stays a per-rule MissingTrack.

## Occurrence-by-occurrence verification

### Occ 1 — decided, "memo D21", 2026-07-09

**Artifact:** `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md`,
section `## D21: clean unsupported-source diagnostic (fixes B17)` (line 162),
body lines 164-189.

The memo is the origin decision for this pattern and carries every load-bearing
element of the statement:

- **The problem** (lines 164-168): a source identified with exit 0 but reported
  as unrecognized/unsupported "falls through to per-rule `MissingTrack` errors.
  The user sees 'rule X: missing track' when the real cause is 'this file is not
  a source mkvmerge can mux.'" -> matches "instead of per-rule MissingTrack noise".
- **Distinct code, different remediation** (lines 176-179): "New code
  `UnsupportedSource`, distinct from `UnidentifiableSource`... Two codes because
  the remediation differs." -> verbatim match for "distinct code from
  UnidentifiableSource, different remediation".
- **Before rule resolution, skip the file** (lines 172-175): "After
  identification, before rule resolution... its plan is skipped." -> matches
  "before rule resolution and skips the file".
- **Predicate + zero-track boundary** (Open mechanic, lines 181-189):
  "Assumption: fire `UnsupportedSource` when `!container_recognized ||
  !container_supported`; a recognized+supported container with zero tracks stays
  a per-rule `MissingTrack` issue." -> matches the gate predicate and the
  zero-track carve-out.

Right file (the plan-3.5 decisions memo), right date (2026-07-09), right kind
(a design decision, i.e. "decided"), right topic. **SURVIVES.**

Caveat (does not affect the verdict, see Notes): the memo's main **Decision**
bullet (line 172) says "gate on `is_identifiable()`", which the **Open mechanic**
assumption (the `!container_recognized || !container_supported` predicate)
supersedes. The predicate refinement is present in the same memo; the two
verdicts below lock it in as "decision #5". So the topic and approach clearly
arose and were decided here.

### Occ 2 — reinforced, "task-6 verdict (three corner tests)", 2026-07-09

**Artifact:** `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/task-6-review-verdict.md`
(reviewer subagent `af2047475ad451eb4`, `final_message_ts 2026-07-09T17:54:48Z`).

Independently reinforces the pattern:

- Spec-compliance row (line 26): "Trigger is `!container_recognized ||
  !container_supported` only, not `is_identifiable()`" -> "`is_identifiable()`
  (identify.rs:144-146) independently checked to additionally test
  `!tracks.is_empty()`, so the distinction is real, not cosmetic".
- Row line 27: "Recognized+supported+zero-tracks stays `MissingTrack`" with a
  dedicated test.
- Row line 28 + Strengths line 35-36: the OR is split into two dedicated tests
  (one per disjunct) plus the third `recognized_supported_zero_tracks_stays_
  missing_track_not_unsupported_source` test asserting both directions. That is
  the **three corner tests** the ref names (two OR false-branches + the
  zero-track confirmatory boundary).

Right date, right artifact, genuine reinforcement (the reviewer re-derived the
`is_identifiable()` `!tracks.is_empty()` distinction from `identify.rs` rather
than echoing the memo). **SURVIVES.**

### Occ 3 — reinforced, "whole-branch verdict (Strengths)", 2026-07-09

**Artifact:** `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/whole-branch-review-verdict.md`
(reviewer subagent `a362a520427c54db7`, `final_message_ts 2026-07-09T18:02:41Z`),
`## Strengths` section (line 22), bullet at line 25.

Verbatim: "**D21 gate placement is exactly right.** `resolve_file`
(planner.rs:365-374) fires `UnsupportedSource` *after* the skew warning and
*before* the rule loop, mirroring the `UnidentifiableSource` early-return's
`FileReport` shape. Crucially it uses the raw `!container_recognized ||
!container_supported` predicate, **not** `is_identifiable()` (identify.rs:144-145,
which also tests `!tracks.is_empty()`). That distinction is the whole point of
D21 decision #5, and the three regression tests (...) lock all three corners
including the zero-track-stays-missing-track boundary."

Right date, correct section (the ref's "(Strengths)" qualifier matches the `##
Strengths` heading exactly), directly supports every clause of the statement.
**SURVIVES.**

## Distinctness / duplication check

Three distinct artifacts, three distinct events, no duplication:

- Occ 1 = the origin decision (design-decisions memo), authored as a spec.
- Occ 2 = a per-task spec+quality review (single reviewer subagent
  `af2047475ad451eb4`, scoped to Task 6 only).
- Occ 3 = a cross-cutting whole-branch review (a different reviewer subagent
  `a362a520427c54db7`, scoped to all six commits `b04c4a2..2b08de4`).

Occ 2 and Occ 3 are not the same review re-counted: different reviewer agents,
different `tool_use_id`, different scope (one task vs. whole branch). The
whole-branch reviewer independently traced the gate placement at
`planner.rs:365-374` and pairwise commit composition, adding verification the
task-6 review did not perform. Genuine independent reinforcement of one pattern,
not an inflated count. All three fall on 2026-07-09 (one SDD session for Plan
3.5), which is expected: decide, then two independent reviews confirm, on the
same day.

## Notes (do not affect the verdict)

1. **Memo internal shorthand vs. implemented predicate.** The D21 **Decision**
   bullet reads "gate on `is_identifiable()`", while the operative refinement
   lives in the **Open mechanic** ("`!container_recognized ||
   !container_supported`... zero tracks stays `MissingTrack`"). The two verdicts
   promote the Open-mechanic predicate to "decision #5", and the shipped code
   (`planner.rs`, and `is_identifiable()` at `identify.rs:144-146` which also
   tests `!tracks.is_empty()`) follows it. The promotion statement's phrasing
   ("not `is_identifiable()`") is therefore correct against the settled decision
   and the code, even though the memo's own Decision-bullet shorthand is looser.
   Worth tightening the memo if it is ever edited; not a fabrication and not
   grounds to drop the occurrence.

## Conclusion

verified_count = **3** (all occurrences survive; none fabricated, misattributed,
or duplicated). >=3 survive -> **CONFIRMED**. The promotion to standing
house-knowledge stands; its recurrence count is real.
