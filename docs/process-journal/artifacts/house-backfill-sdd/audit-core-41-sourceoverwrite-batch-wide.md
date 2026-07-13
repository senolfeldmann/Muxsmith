# Audit: core-41-sourceoverwrite-batch-wide (PROMOTION candidate)

**Cluster:** `core-41-sourceoverwrite-batch-wide` (kind: pattern, domain: core, status: settled)
**Claimed count:** 5 | **promoted:** true (promoted_at: 3)
**Statement:** Collect all input paths batch-wide (primaries + every resolved track/attachment/chapters donor) before rendering and fire SourceOverwrite if any rendered output equals any of them; capture is independent of render success, so donors of render-failed files are protected too. The protection set was repeatedly found incomplete and progressively closed by construction.

**Verdict: CONFIRMED** — verified_count = 5 (all occurrences survive; none fabricated, misattributed, or duplicated). Promotion to a standing convention stands. Count is real.

---

## Per-occurrence verification

### Occ 1 — `violated-corrected`, 2026-07-09, ref: "F5 review (per-primary -> batch-wide, commit 6f475b3)" — SURVIVES

Two artifacts corroborate this jointly, as the ref cites both.

`docs/process-journal/artifacts/plan-2-fixes-sdd/F5-report.md`, addendum "Critical #1: `SourceOverwrite` was per-primary, not batch-wide" (lines 164-217): the independent F5-review's own harness proved that primary A's donor could equal primary B's rendered output and B's plan survived (only an `Info`-severity `OutputCollision`, no `SourceOverwrite`) "because `donor_paths` was a local `Vec` reset on every `resolve_file` call, populated only from the *current* primary's own resolved donors." The fix removed the per-primary check and added the batch-wide `detect_source_overwrites(files, primary_paths)` wired into `plan_core` before the first `finalize_plans`.

`git show 6f475b3` (Thu Jul 9 2026, *"fix(core): SourceOverwrite batch-wide, Display for IdentifyError"*): the commit message states verbatim that the per-primary check was replaced with a batch-wide `detect_source_overwrites` pass "gathering every primary path plus every assignment.source across all files before anything drops a plan."

The occurrence evidence ("donor_paths scoped per-primary, not batch-wide; a different primary can silently overwrite a donor another primary reads from") matches both artifacts exactly. A defect found by review and corrected = `violated-corrected`. **Confirmed.**

### Occ 2 — `deferred`, 2026-07-09, ref: "FINAL review M2 (render-failed-donor gap accepted for v1)" — SURVIVES

`docs/process-journal/artifacts/plan-2-fixes-sdd/FINAL-review.md`, item M2 (lines 101-118): "F5: a donor referenced only by a render-failed file escapes the batch-wide SourceOverwrite set." It explains that `detect_source_overwrites` gathers inputs only from files whose `plan` is `Some`; a file whose `render_output` returned `None` has `plan == None` and its donor assignments are discarded in `resolve_file`, so a donor referenced only by such a file is unprotected. Classified **MINOR** ("Extremely narrow"); the review verdict (lines 18-23) is `changes-needed` for the F6 regression only — M2 is not raised to changes-needed, i.e. accepted as a narrow gap rather than fixed in the pass. That is a `deferred` occurrence. **Confirmed.**

### Occ 3 — `decided`, 2026-07-11, ref: "task-7 (render-failed donors protected)" — SURVIVES

`docs/process-journal/artifacts/plan-5.5-sdd/task-7-report.md` ("SourceOverwrite completeness + S11 guard comment", commit `0456f72`): closes exactly the M2 gap ("Plan-2 FINAL review finding M2; the only audit finding with data-loss potential, per docs/ROADMAP.md"). `resolve_file` now returns `(FileReport, Vec<PathBuf>)`; the second element captures every `Assignment.source` "right after the assignments loop and before `assignments` moves into `Plan` — so it exists regardless of whether `render_output` later succeeds." A TDD test (`source_overwrite_protects_donor_of_render_failed_file`) exercises the three-way constellation. The deferred M2 gap is decided and closed here for the track-donor kind = `decided`. **Confirmed.** (Distinct from Occ 1: Occ 1 fixed cross-primary scoping; Occ 3 fixed render-failure capture. Distinct from Occ 2: Occ 2 defers M2, Occ 3 closes it — a legitimate defer-then-close pair, two events.)

### Occ 4 — `reinforced`, 2026-07-11, ref: "task-7.5 (attachment donors)" — SURVIVES

`docs/process-journal/artifacts/plan-5.5-sdd/task-7.5-report.md` ("SourceOverwrite protection for attachment donors"): extends T7's existing `resolved_sources` mechanism (not a parallel one) by chaining `attachments.add_files` into the capture, at the same pre-render point. New test `source_overwrite_protects_attachment_donor_of_render_failed_file`. Same invariant broadened to a second donor kind = `reinforced`. The report's Concerns section also flags the still-open chapters gap, which becomes Occ 5. **Confirmed.**

### Occ 5 — `reinforced`, 2026-07-11, ref: "task-7.6 (chapters donors, class closed by exhaustive match)" — SURVIVES

`docs/process-journal/artifacts/plan-5.5-sdd/task-7.6-report.md` ("SourceOverwrite protection for chapters donors — #7 class closure", commit `99b2e34`): chains `ChapterSource::External` into `resolved_sources` via an **exhaustive** `match` (`External`/`Keep`/`Drop`), so a future `ChapterSource` variant fails to compile until chained. A completeness comment records that `model.rs` has exactly two `Locator` field sites (`ExternalBlock.external`, `AttachmentRule.add`) feeding the three donor kinds — closing the class by construction. The combined verdict `docs/process-journal/artifacts/plan-5.5-sdd/task-7.5-7.6-verdict.md` independently confirms the exhaustive match and the two-field-site closure. Same invariant broadened to the third and final donor kind = `reinforced`. **Confirmed.**

---

## Count reconciliation

All five occurrences are distinct real events, corroborated by their own artifacts, with distinct dates, commits, and content:

1. **Per-primary → batch-wide** scoping fix (F5 review + commit `6f475b3`, 07-09).
2. **Render-failed-donor gap deferred** as narrow Minor M2 (Plan-2 FINAL review, 07-09).
3. **M2 closed for track donors** (task-7 / `0456f72`, 07-11).
4. **Attachment donors** added to the protection set (task-7.5 / `ca238dc`, 07-11).
5. **Chapters donors** added; class closed by construction (task-7.6 / `99b2e34`, 07-11).

No two are the same event: Occ 1 and Occ 3 fix different sub-defects of the invariant (cross-primary scoping vs render-failure capture); Occ 2→Occ 3 is a genuine defer-then-close pair; Occ 3/4/5 cover three distinct donor kinds (track / attachment / chapters) via three distinct commits. This is a real, progressively-hardened invariant — exactly the recurrence pattern a house-knowledge promotion should reflect, not a fabricated count.

The statement's mechanism claims also hold against the code artifacts: batch-wide union of primaries + all resolved donor sources; SourceOverwrite fired on any rendered output landing in that set; capture taken before `output.map(...)` moves data into `Plan`, hence independent of render success; and "closed by construction" is literally realized by the exhaustive match plus the two-`Locator`-field-site completeness comment.

**verified_count = 5.** 5 ≥ 3 → **CONFIRMED.** The promotion of `core-41-sourceoverwrite-batch-wide` to a standing convention stands on a real count.
