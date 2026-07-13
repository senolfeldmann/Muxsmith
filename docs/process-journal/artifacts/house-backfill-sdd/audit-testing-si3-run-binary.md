# Audit: `testing-si3-run-binary` (PROMOTION candidate)

- **Cluster id:** `testing-si3-run-binary`
- **Kind / domain:** pattern / testing
- **Claimed count:** 5 (promoted, `promoted_at: 3`)
- **Verdict:** **CONFIRMED** — 5 of 5 occurrences survive; promotion stands.
- **verified_count:** 5

## Statement under audit

> mkvmerge behavior (flag spellings, argv semantics, `-J` shape, cross-file donor+keep ordering) is confirmed by running the installed real binary (v100), never asserted from memory; golden tests then lock the exact strings and the `-J` shape is pinned to identification schema v20. Standing SI-3 house rule, re-issued as a per-plan global constraint and independently upheld by implementers and whole-branch reviewers.

## Per-occurrence verification

Each cited ref was opened in `/home/senol/Git/Muxsmith` and checked for whether the (topic, approach) actually arose there as the stated `kind`.

### occ1 — `decided` — plan Global Constraints + memo D9/D11 (Plan 3) — **SURVIVES**

- `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md:17`: "**Confirm mkvmerge behavior by running the binary (v100 installed), never from memory.** The `-J` shape is pinned to identification schema v20 (`...mkvmerge-identification-output-schema-v20.json`)." Verbatim match to the evidence, and it sits in the plan's **Global Constraints** block (line 11) that every task inherits.
- Memo backing in `docs/superpowers/specs/2026-07-09-plan-3-design-decisions.md`: D9 (line 96-97) "Confirmed against the installed binary, not memory (repo rule)"; D11 (line 162-163) argv "pinned during implementation against mkvmerge v100 via golden tests ... and by running the binary, not from memory."
- Genuine `decided` event: the rule is issued as a per-plan global constraint plus two design-decision memos. Not fabricated, not misattributed.

### occ2 — `reinforced` — whole-branch-review-verdict.md (Plan 3, Strengths) — **SURVIVES**

- `docs/process-journal/artifacts/plan-3-sdd/verdicts/whole-branch-review-verdict.md:26`: "I verified all **26 flag spellings** exist in v100, and ran a rich argv end-to-end ... exit 0."
- Line 27: "I confirmed the `-J` `id` is identical to mkvmerge's `--attachments` selector id ... **This is the one place a numbering mismatch would have silently selected the wrong files; it is correct.**"
- Reviewer independently drove real v100 beyond the goldens (line 22: "Where the golden tests only assert strings, I drove real mkvmerge v100 to close the gaps"). Matches evidence verbatim, including the silent-wrong-file risk framing. Genuine `reinforced` event.

### occ3 — `decided` — plan Global Constraints (Plan 3.5) — **SURVIVES**

- `docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md:18`: "mkvmerge is external (v100, identification schema v20). Confirm mkvmerge behavior by RUNNING the binary, never from memory (gated tests self-skip when mkvmerge is absent)." Verbatim match including the gated-self-skip clause.
- Distinct from occ1: a separate plan document at a separate plan boundary, re-issuing the constraint for the Plan 3.5 cycle (consistent with the statement's "re-issued as a per-plan global constraint"). Genuine `decided` event, not a copy-artifact of occ1.

### occ4 — `reinforced` — whole-branch-review-verdict.md (Plan 3.5, header + Important #1) — **SURVIVES**

- `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/whole-branch-review-verdict.md` header (salvaged, byte-faithful per the file's own provenance comment): "I verified the one load-bearing runtime claim (donor+keep ordering) against the real mkvmerge v100 rather than trusting the single-file test." Verbatim match.
- Important #1 elaborates: reviewer ran `mkvmerge -o out.mkv ( primary ... ) ( donor ... ) --track-order 1:0` against v100 to establish the cross-file order `[DONOR, PRIM0, PRIM1]`. Real-binary verification of the cross-file donor+keep ordering named in the statement. Genuine `reinforced` event, distinct branch/review from occ2.

### occ5 — `reinforced` — journal 2026-07-09 (Moments) + task-3/task-7 verdicts — **SURVIVES**

- `docs/process-journal.md:261` (under the Plan 3.5 session-4 **Moments**, header line 258): "Tasks 3 and 7 implementers both hand-ran mkvmerge before encoding the order assertion, unprompted (SI-3 holding)." Verbatim.
- Corroborated by the task verdicts (implementer-level, distinct from the reviewer-level occurrences):
  - `plan-3.5-sdd/verdicts/task-3-review-verdict.md:39`: "Manual empirical repro in the report (raw `mkvmerge --track-order 0:1`) matches the Rust test's encoded expectation exactly — the assumption was confirmed against the binary, not written from memory, satisfying SI-3." (also line 27: observed BRAVO/ALPHA/CHARLIE against mkvmerge v100.)
  - `plan-3.5-sdd/verdicts/task-7-review-verdict.md:40`: "The live test rewrite is a genuine two-file (primary + separate donor) cross-mux, **hand-verified against real mkvmerge before being encoded as a Rust assertion (SI-3 discipline)**."
- Genuine `reinforced` event. The two task verdicts named in the ref both independently support the journal Moment.

## Distinctness / duplication check

All five map to separate authored artifacts at separate process points:

- occ1 vs occ3: two different plan documents (Plan 3, Plan 3.5), each issuing the constraint for its own cycle — not one text counted twice.
- occ2 vs occ4: two different whole-branch reviews on two different branches (`3283552..` era vs `b04c4a2..2b08de4`), each an independent reviewer act driving real v100.
- occ5: implementer-level upholding (hand-ran before the assertion), distinct from the reviewer-level acts in occ2/occ4; journal Moment + two corroborating task verdicts.

No occurrence is fabricated, misattributed, or a duplicate of another listed occurrence.

## Cross-check of the cluster's headline claims

- "26 flag spellings exist in v100" -> occ2 line 26. ✅
- "`-J` attachment id equals the `--attachments` selector id — the one silent-wrong-file risk" -> occ2 line 27. ✅
- "drove real v100 beyond the goldens" -> occ2 (line 22) + occ4 (Important #1). ✅
- "hand-ran mkvmerge before encoding order assertions, unprompted" -> occ5 (journal 261 + task-3/task-7 verdicts). ✅
- "`-J` shape pinned to identification schema v20" -> occ1 (plan line 17, D9). ✅

## Conclusion

**CONFIRMED.** 5/5 occurrences verified against their cited artifacts; verified_count = 5 (>= 3). The recurrence is real, not fabricated: the SI-3 "run the binary, never from memory" discipline was decided as a per-plan global constraint (occ1, occ3) and independently upheld by both whole-branch reviewers (occ2, occ4) and task implementers (occ5). Promotion to a standing house convention stands.
