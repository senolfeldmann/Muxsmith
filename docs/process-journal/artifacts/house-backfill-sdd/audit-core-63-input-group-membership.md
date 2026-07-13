# Audit: core-63-input-group-membership (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 4, promoted: true (at 3)
**Statement under audit:** `input_groups` = primary always (group 0) + donor sources with at least one `track_id=Some` assignment; exclude track-less donors. The first impl unconditionally included every distinct `assignment.source` (misreading the canonical reference's primary-only carve-out). Separately, the exclusion-comment's `mkvmerge may reject empty groups` rationale was empirically false against v100 and corrected while keeping the guard.

**Verdict: CONFIRMED** — all 4 occurrences survive; verified_count = 4. Promotion to standing house-knowledge stands.

Every ref is a real, checkable artifact, on-topic, with the `violated-corrected` kind accurate for each. Nothing fabricated, nothing misattributed, no strict duplicate under the house's own applied standard (see the core-40 precedent invoked below). One honest caveat on the *nature* of the count is recorded at the end; it does not meet the drop bar.

The statement bundles two distinct-but-adjacent findings on the same code region, both on 2026-07-09:
- **Part A (the behavioral rule):** exclude track-less donor sources from `input_groups`. Occurrences 1-2.
- **Part B (a doc-accuracy correction):** the comment justifying the exclusion cited a false reason (`mkvmerge may reject empty groups`); corrected while keeping the guard. Occurrences 3-4.

---

## Per-occurrence verification

### Occurrence 1 — "task-9 verdict (membership fix + test)" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-3-sdd/verdicts/task-9-review-verdict.md` (round 1 of 2, final_message_ts `2026-07-09T12:17:43Z`).

- The **detection** artifact. Important issue #1 (lines 36-52) states the defect verbatim: `input_groups` "unconditionally includes every distinct `assignment.source`, including a donor whose only assignment(s) have `track_id: None`", rendering `--no-video --no-audio --no-subtitles --no-buttons ( <donor> )`.
- The "misreading of the canonical reference's primary-only carve-out" claim in the statement is this verdict's own argument (line 37): the reference's "primary is always group 0 even if it contributes no tracks" carve-out is only meaningful if non-primary groups exclude zero-track sources. Exactly Part A.
- Prescribes the fix as the `a.track_id.is_some()` guard (lines 40-51), which is what d55f19d then implemented.
- Salvaged from the SDD session transcript with full provenance (session uuid, tool_use_id, agent_id, timestamp). Real, on-topic. Kind `violated-corrected` fits: the Task 9 impl violated the rule, the review caught it.

### Occurrence 2 — "commit d55f19d" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `git show d55f19d` — `fix(command): exclude unmatched donor sources from input groups`, 2026-07-09 14:20:21 +0200 (12:20 UTC).

- The **implementation** of what Occurrence 1 detected. Diff adds the `a.track_id.is_some() &&` guard to `input_groups` in `crates/muxsmith-core/src/command.rs` and 46 lines of tests in `tests/command.rs` (the "+ test" in the ref label).
- Round-2 verdict (`task-9-review-verdict-round-2.md`, 12:23 UTC) independently confirms the fix and traces the regression test `unmatched_donor_rule_opens_no_input_group` by hand against the pre-fix algorithm (would push `/m/e.tr.srt` with `track_id: None`, fail the `!argv.contains` assertion) — so the test genuinely pins the bug.
- Real, on-topic (Part A). Distinct artifact from Occurrence 1 (code+test vs review finding).

### Occurrence 3 — "whole-branch verdict Minor 2 (false rationale)" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-3-sdd/verdicts/whole-branch-review-verdict.md`, Minor #2 (lines 44-45), final_message_ts `2026-07-09T13:27:30Z`.

- The **detection** for Part B. Minor 2 verbatim: "`command.rs` `input_groups` rationale is empirically false. The comment ... says an empty input group is one 'that mkvmerge may reject.' mkvmerge v100 accepts empty groups (I ran `--no-video --no-audio --no-subtitles --no-buttons ( f ) ...` and a fully zero-track single-file mux: both exit 0). The guard ... is still the *right* behavior ... but the stated reason is wrong."
- This is the statement's second sentence (Part B) literally. Not misattributed. The reviewer ran live mkvmerge v100 to empirically disprove the rationale — substantive, independent work, a different finding from Occurrence 1 (which was about *whether* to exclude, not *why the comment's reason is wrong*).
- Different reviewer agent (`a976df077071a93f9`) and later timestamp than Occurrence 1's. Not a duplicate. Kind `violated-corrected` fits: false comment shipped, review caught it.

### Occurrence 4 — "commit 7d46547" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `git show 7d46547` — `chore(command,planner): whole-branch review minors (comment, test, readability)`, 2026-07-09 15:33:28 +0200 (13:33 UTC).

- The **implementation** for Part B. Diff replaces the `mkvmerge may reject` comment with the real rationale ("it would carry no kept track into the output ... so opening it as an input at all would just be dead weight"), keeping the guard unchanged. Exactly "corrected while keeping the guard."
- Commit message names the finding explicitly: "mkvmerge does not actually reject an empty input group; the real reason is a track-less donor contributes nothing worth opening as an input."
- Real, on-topic (Part B). Distinct artifact from Occurrence 3 (commit vs review finding).

---

## Drop-criteria test (fabricated / misattributed-topic / strict-duplicate)

- **Fabricated:** none. All four refs open and check out; the two verdicts carry full transcript provenance, the two commits are in the branch with the exact diffs the statement describes.
- **Misattributed-topic:** none. All four concern `input_groups` membership; occ 1-2 are the statement's Part A, occ 3-4 its Part B (which the statement bundles explicitly, not a stretch).
- **Strict-duplicate:** none, under the house's own applied standard. The peer audit `audit-core-40-output-collision-planned-twice.md` explicitly keeps a *review* and the *commit it reviews* as distinct occurrences ("Real implementation artifact, distinct from the review that verifies it"), and confirmed that cluster despite all three being one decision pipelined. core-63 is *stronger* than core-40 on two counts: (a) it spans two genuinely distinct incidents (membership rule + comment rationale), not one; (b) the `violated-corrected` kind is *accurate* for the commits (the code was truly violated then corrected), where core-40 had to record two imprecise `decided` labels for what were really a review and an implementation.

Applying a stricter collapse here than the house applied to its peers would be inconsistency, not rigor. All four survive.

---

## Corroboration outside the four refs

- Round-2 re-review (`task-9-review-verdict-round-2.md`) independently verifies the Part A fix is correct, the `group_index` `.expect(...)` invariant holds by construction, and the regression test provably fails pre-fix.
- The current comment in `crates/muxsmith-core/src/command.rs` still carries the corrected (Part B) rationale, so the fix stuck.

---

## Honest caveat on the *nature* of the count (recorded, below the drop bar)

count: 4 is **two same-day detection->fix pairs**, not four independent recurrences:
- Incident A (membership rule): task-9 verdict detects (12:17 UTC) -> d55f19d fixes (12:20 UTC).
- Incident B (false comment rationale): whole-branch verdict detects (13:27 UTC) -> 7d46547 fixes (13:33 UTC).

Both incidents landed on 2026-07-09, each as a review-finding immediately followed by its fix commit. And only Incident A concerns the actual behavioral **rule** being promoted; Incident B is a documentation-accuracy correction of the *comment* that justifies the rule (the guard's behavior was never in doubt there). A future consumer should read "count: 4" as "one membership rule caught+fixed, plus a false rationale in its comment caught+fixed, same day," not "this pattern kept independently cropping up in four contexts."

This is legitimate for a **settled** pattern, and matches how core-40 was treated: the evidence that the rule is real and load-bearing (independent review detection + code + regression test + live-v100 validation + a second review confirming the fix) is exactly what a standing convention needs, and none of it is fabricated. Under the audit's drop criteria (fabricated / misattributed-topic / duplicate-of-another) none of the four qualifies, so the count is not reduced.

**verified_count = 4 -> CONFIRMED.**
