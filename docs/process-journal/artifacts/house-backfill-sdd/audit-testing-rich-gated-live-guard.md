# Audit: `testing-rich-gated-live-guard` (PROMOTION candidate)

**Cluster kind:** pattern · testing domain
**Claimed count:** 3 · promoted (at 3)
**Verdict: CONFIRMED** — 3 of 3 occurrences survive; promotion stands.

The statement: the Plan-3 one-off manual v100 validation is converted into a
standing gated live guard (real attachment via `--attach-file` /
`--attachment-mime-type text/plain`, SI-3 probed; rename/change a track, keep
the attachment, re-identify via `-J` asserting `track_name==Renamed`,
`default_track true`, attachment `file_name` preserved). Deferred in Plan 3,
scheduled (D18), implemented in Plan 4.

This is a clean **deferral → decision → implementation** thread across two eras.
Each occurrence is a distinct artifact, distinct role, and (2 of 3) distinct date.

---

## Occurrence 1 — 2026-07-09 · deferred · CONFIRMED

**Ref:** `docs/process-journal/artifacts/plan-3-sdd/verdicts/whole-branch-review-verdict.md` (Plan 3, Minor 1) + journal open threads

- **Minor 1 (line 43), byte-faithful match:** "a future argv refactor could
  regress the shape and the golden test would just be updated to the new
  (wrong) string with nobody re-running the binary. Fix: add one gated live
  case with an attachment + a couple of `changes` so CI-with-mkvmerge guards the
  real surface, not just string equality." The cluster's evidence string is a
  verbatim slice of this.
- **Reinforced by the verdict's Recommendations (line 63):** "Extend the gated
  live test (Minor 1) — the single most valuable follow-up, since it converts my
  one-off manual v100 validation into a standing regression guard." This is
  exactly the "one-off manual v100 validation → standing guard" framing the
  cluster statement uses.
- **Secondary ref verified:** `docs/process-journal.md` line 231 (Plan-3
  handoff open threads): "Deferred minors: richer gated live test (attachment +
  changes) - highest value". The "+ journal open threads" citation is real.
- **Role check:** genuinely a *deferral* — filed under "Minor (Nice to Have)"
  and listed as a follow-up recommendation, not fixed in Plan 3. Correct
  `occ_kind = deferred`.

## Occurrence 2 — 2026-07-09 · decided · CONFIRMED

**Ref:** `memo D18` → `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md` §"D18: Plan 3 follow-up triage" (lines 138-144)

- **Byte-faithful match (lines 142-144):** "Richer gated live test (attachment +
  `changes`): converts the one-off manual mkvmerge-v100 validation from the Plan
  3 review into a standing guard, in the same gated tier the executor tests
  extend." Identical to the cluster's evidence string.
- **Role check:** genuinely a *decision* — D18 triages the Plan-3 follow-up
  explicitly "Into Plan 4". Correct `occ_kind = decided`. Date consistent with
  the cluster's own era note ("Plan-4 design-memo D18 decisions dated
  2026-07-09").
- **Not a duplicate of Occ 1:** distinct artifact (design memo vs. review
  verdict), distinct role (scheduling decision vs. review deferral).

## Occurrence 3 — 2026-07-10 · reinforced · CONFIRMED

**Ref:** `docs/process-journal/artifacts/plan-4-sdd/verdicts/task-7-review-verdict.md`

- **Match:** the Plan-4 task-7 verdict reviews the *implementation* of exactly
  this test. Spec-Compliance section: "Primary built via real mkvmerge with SRT
  + `--attach-file` and explicit `--attachment-mime-type text/plain`" (line 20);
  "SI-3 probe against the real binary documented" (line 25); all three
  assertions present — `track_name == "Renamed"`, `default_track == true`,
  attachment present with the original `file_name "note.txt"` (line 23);
  re-identified via `-J` (line 22). Matches the cluster's evidence string in full.
- **Role check:** genuinely a *reinforcement* — the deferred/decided pattern is
  now realized in code and reviewed as spec-compliant. Correct
  `occ_kind = reinforced`.
- **Attribution guard (the trap):** there are *two* `task-7-review-verdict.md`
  files. The Plan-**3** one (2026-07-09) is about the keep+donor track-order
  deterministic guard (fix aa75025) and belongs to a *different* cluster
  (`testing-keep-donor-deterministic-guard`); journal line 248 is that one. The
  cluster correctly cites the Plan-**4** verdict (2026-07-10), whose subject is
  the attachment+changes gated live test. No misattribution, no cross-cluster
  double-count.

---

## Independent corroboration (beyond the cited refs)

The "implemented in Plan 4" claim is not fabricated: the test physically exists
in the tree at `crates/muxsmith-core/tests/command_integration.rs`:

- SI-3 probe comment (lines 440-448) documenting the `--attach-file` +
  `--attachment-mime-type text/plain` behavior confirmed against v100.
- `ATTACHMENT_PROFILE` (line 449) with `changes: { track_name: Renamed,
  default_track: true }`.
- Fixture build via real mkvmerge with `--attachment-mime-type text/plain
  --attach-file` (line 478).
- `assert_eq!(track_name(track), "Renamed")` (line 530).

So the guard is a real standing test, not a described-but-never-landed artifact.

## Distinctness / count discipline

| # | date | role | artifact | distinct? |
|---|---|---|---|---|
| 1 | 2026-07-09 | deferred | Plan-3 whole-branch verdict, Minor 1 (+ journal) | yes |
| 2 | 2026-07-09 | decided | memo D18 (Plan-4 design decisions) | yes |
| 3 | 2026-07-10 | reinforced | Plan-4 task-7 verdict + landed test | yes |

Three distinct artifacts, three distinct roles, two distinct dates. No
fabrication, no misattribution, no duplicate. The recurrence signal is real.

**verified_count = 3 → CONFIRMED. The promotion to a standing house convention
stands.**
