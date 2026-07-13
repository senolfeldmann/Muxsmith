# Audit: core-80-keep-mode-track-order (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 4, promoted: true (promoted_at: 3)
**Statement under audit:** Under `keep`, `--track-order` lists all primary tracks first in source order (`0:id`), donors trailing (`g:id` in rule order). The initial assumption (only matched tracks listed, mkvmerge appending kept-unmatched after) placed a donor-only-keep profile's added track ahead of the primary's video/audio; the whole-branch review flagged this as a usability trap and it was reversed to option B (kept-unmatched primary tracks count as matched). Both variants live-verified against v100.

**Verdict: CONFIRMED** — all 4 occurrences survive; verified_count = 4. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no strict duplicate. This is a genuine assumption -> flag -> reversal -> rebuild arc, each stage resting on a distinct artifact, all on 2026-07-09, all live-verified against mkvmerge v100. One ref-label imprecision and one caveat on the *nature* of the count are recorded at the end; neither meets the drop bar.

---

## Per-occurrence verification

### Occurrence 1 — "memo D20 / Plan 3 Task 3 (assumption A)" (2026-07-09, kind: decided) — SURVIVES

Refs: `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md` (D20) + `docs/process-journal/artifacts/plan-3.5-sdd/task-3-brief.md` / `task-3-report.md`.

- The memo D20 **originally** held assumption A. Proof from the reversal commit's deletions (`git show c1d5614`): the removed text reads *"**Assumption: unmatched-kept tracks keep source-relative order, placed after the explicitly-ordered matched tracks**; revise per the empirical mkvmerge result."* That is assumption A verbatim, and it was the memo's recorded decision before the reversal.
- Plan 3.5 **Task 3** locked assumption A with a gated live guard: task-3-brief — *"Confirms the D20 assumption (memo open mechanic #3, per SI-3): under keep, tracks kept but absent from --track-order land in source-relative order after the ordered ones."* task-3-report empirically reproduced it against mkvmerge **v100** (`BRAVO, ALPHA, CHARLIE` = ordered-first then unlisted in source order). This is the "assumption A, live-verified against v100" half of the statement.
- Kind "decided" is exact for the memo (assumption A was the decided position); Task 3 is the confirmation guard.
- **Ref-label imprecision (does not trigger drop):** the ref says "Plan 3 Task 3"; the correct plan is **Plan 3.5** Task 3. A distinct *Plan 3* Task 3 existed on a different topic ("attachment matching") — task-3-report.md explicitly notes it "overwrites a stale task-3-report.md left over from Plan 3's Task 3." The memo D20 anchor and the assumption-A content are unambiguous, so the topic is correctly attributed; only the plan number is mislabeled by 0.5.

### Occurrence 2 — "whole-branch verdict Important #1 (donor-first trap)" (2026-07-09, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/whole-branch-review-verdict.md`, Issues -> Important **#1**.

- The verdict flags exactly the donor-first trap: *"Donor+`keep` track ordering is surprising for the marquee use case ... a profile that writes only a donor rule under `keep` puts the added subtitle **first**, ahead of the primary's video/audio."*
- Verified against the binary in-verdict: `mkvmerge ... ( primary[PRIM0,PRIM1] ) --subtitle-tracks 0 ( donor[DONOR] ) --track-order 1:0 => [DONOR, PRIM0, PRIM1]` against mkvmerge **v100** — the steelman's exact `[DONOR,PRIM0,PRIM1]` native rendering.
- Called a "genuine usability trap on the one workflow `keep` exists to enable," explicitly "not a correctness defect and not a merge blocker." Kind "violated-corrected" is a fair label: assumption A as-shipped was flagged here as wrong-for-the-use-case and this is the detection point that drove the correction.
- Distinct artifact (a cross-cutting review), distinct from occ 1's memo/task and occ 3's reversal. Solid.

### Occurrence 3 — "journal + memo D20-B (reversal, commit c1d5614)" (2026-07-09, kind: decided) — SURVIVES

Refs: `git show c1d5614` + `docs/process-journal.md` (reversal entry).

- Commit c1d5614 ("docs: Plan 3.5 memo updates ... D20-B keep ordering") inserts the reversal into the memo: *"**Track order under `keep` (RESOLVED B, Şenol 2026-07-09; reverses the earlier assumption).** `--track-order` lists ALL primary tracks first, in the primary's source order (id ascending), then donor tracks in profile-rule order."* Message body: *"D20 records decision B (donors trail; 'keep = match what is already there') reversing assumption #3."*
- Journal corroborates the decision as Şenol's: *"D20 keep TRACK ORDER reversed at close-out ... Şenol called B (donors trail): 'keep = match what's already there' makes kept-unmatched primary tracks count as matched, so --track-order lists them (invariant holds), primary first. Task 7 built it."*
- Kind "decided" is exact — this is the reversal decision. Distinct artifact from occ 4 (the decision + memo, versus its implementation + verdict).

### Occurrence 4 — "task-7 verdict (option B built, commits 51567d7/aa75025)" (2026-07-09, kind: reinforced) — SURVIVES

Refs: `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/task-7-review-verdict.md` + `docs/process-journal/artifacts/plan-3.5-sdd/task-7-report.md` + `git show 51567d7`/`aa75025`.

- Task-7 verdict: **PASS**, confirming option B was built to the memo: *"`push_track_order_keep` (command.rs:76-90) is a direct, literal transcription of the D20-B decision text (verified against ...plan-3.5-design-decisions.md:127-136)"*; live test `live_keep_donor_trails_primary` asserts `["PA","PB","DONOR"]` against real mkvmerge — the "option B live-verified against v100" half of the statement.
- Commit 51567d7 ("feat(command,planner): keep-mode --track-order lists primary before donors (D20 revision B)") implements it: Plan gains `primary_track_ids`; `push_track_order` emits every primary track first then donor assignments; *"confirms the emitted order against mkvmerge v100 (SI-3)."*
- Commit aa75025 ("test(command): fast unconditional guard for keep-mode donor-trails-primary track order") adds a deterministic unit test asserting the exact `"0:0,0:1,1:0"` order (verified to fail against a scratch donor-first/dropped-primary implementation) — the regression guard that locks the settled rule.
- Kind "reinforced" is apt: option B implemented, spec-transcribed, independently reviewed PASS, and guarded by both a live and a deterministic test. Distinct artifacts (code + verdict) from occ 3 (decision + memo).

---

## Distinctness / duplicate check

Four distinct artifacts, four distinct roles in one lifecycle:

| Occ | Artifact | Role |
|-----|----------|------|
| 1 | memo D20 (pre-reversal) + Plan 3.5 Task 3 live guard | assumption A decided + locked |
| 2 | whole-branch-review-verdict.md, Important #1 | flagged as usability trap |
| 3 | commit c1d5614 (memo D20-B) + journal | reversal decided |
| 4 | task-7 report/verdict + commits 51567d7/aa75025 | option B built + guarded |

No occurrence duplicates another's artifact. The three commits (51567d7, aa75025, c1d5614) were git-committed in one end-of-session batch (20:27–20:45), but that is an SDD gate-close artifact, not evidence of a merged event: the reversal *decision* (occ 3) and the *build + review* (occ 4) are genuinely separate work items with separate artifacts, and occ 1/2 predate both.

## Caveat on the *nature* of the count (recorded, below the drop bar)

All four occurrences are the **same single design question (keep-mode track order) carried through one lifecycle on one date (2026-07-09)**: assumption A decided+guarded (occ 1) -> flagged as a trap (occ 2) -> reversed to B (occ 3) -> B built+guarded (occ 4). This is one decision reversed and rebuilt, not the pattern independently recurring in four unrelated contexts.

It clears the bar more cleanly than a pure decide->implement->review pipeline would, because occ 2 is a real *violation-correction* (an independently-reviewed usability flag that changed the design), not just a restatement. The kind labels are accurate here (decided / violated-corrected / decided / reinforced), and every stage is live-verified against mkvmerge v100 — exactly the profile a standing convention should carry. A future consumer should read "count: 4" as "one design decision, flagged and reversed under review, then rebuilt and guarded," not "this issue kept cropping up across the codebase."

Under the audit's drop criteria (fabricated / misattributed-topic / duplicate-of-another) none of the four qualifies. The only defect found is the occ-1 ref mislabeling "Plan 3.5" as "Plan 3," which does not affect topic attribution.

**verified_count = 4 -> CONFIRMED.**
