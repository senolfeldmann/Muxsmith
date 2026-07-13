# Audit: exec-44-runlog-14day-autoprune (PROMOTION candidate)

**Kind:** pattern (executor domain) · **Claimed count:** 3 · **promoted:** true
**Verdict:** CONFIRMED (3/3 occurrences survive; promotion stands)
**Audited:** 2026-07-13

## Cluster statement under audit

Run-log pruning was left out of v1 at D26 (location documented, prune facility
parked), then reversed at D35 to an automatic 14-day fixed prune with no v1
config, implemented in core executor/joblog so CLI and GUI both inherit; age is
decided by the parsed run-id name only (never mtime), pruning best-effort and
symlink-safe with every IO error ignored. Parity MATCH with mkvtoolnix
removeOldJobs=true/14 days.

## Per-occurrence verification

### Occurrence 1 - memo D26 (2026-07-10, kind=deferred) - CONFIRMED

- **Ref opened:** `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md`, D26 section (lines 106-132).
- **Cited evidence:** "No pruning in v1: the location is documented; a prune facility is a v1.x candidate."
- **Actual text (line 115-116):** "No pruning in v1: the location is documented; a `prune` facility is a v1.x candidate."
- **Verdict:** Verbatim match. D26 genuinely defers pruning: it fixes the log location (`<platform-data-dir>/muxsmith/runs/<run-id>/`) and explicitly parks the prune facility as a v1.x candidate. Corroborated a second time in the same memo at D30/SI-3 (line 234-236): "no automatic 'remove completed jobs after N days' setting in v1 ... pruning is v1.x (D26)." This is a real `deferred` event, not fabricated. **Survives.**

### Occurrence 2 - plan T4.5 / task-4.5-verdict.md (kind=decided) - CONFIRMED

- **Refs opened:** `.superpowers/sdd/plan-5.5/task-4.5-verdict.md` and `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` (Task 4.5, lines 136-150).
- **Cited evidence:** "Age is decided by the PARSED NAME ONLY, never mtime ... every io error during pruning is IGNORED."
- **Actual text:**
  - Verdict line 11-13: "age by parsed name ONLY (no mtime calls); symlink-safe via DirEntry::file_type (symlink_metadata semantics) with a real cfg(unix) test; every io error ignored with the rustdoc why".
  - Plan Step 2 (line 148): "... whose name parses, and whose timestamp is older than `now - 14d`. Best-effort: every io error during pruning is IGNORED".
- **Verdict:** The cited evidence is a faithful synthesis of the two artifacts named in the ref (the "every io error during pruning is IGNORED" fragment is verbatim from the plan Step 2; the "parsed name ONLY, never mtime" fragment is verbatim from the verdict/"no mtime calls"). This is a genuinely **distinct** decision from D35, not a duplicate: D35 explicitly left the age source open ("the age source (run-id timestamp vs dir mtime) are design details of the implementing task"), and T4.5 is where that open dimension was resolved to name-only. Reviewer Assessment: Approved. **Survives.**

### Occurrence 3 - memo D35 (2026-07-11, kind=decided) - CONFIRMED

- **Ref opened:** `docs/superpowers/specs/2026-07-11-pre-1.0-design-decisions.md`, D35 section (lines 69-107).
- **Cited evidence:** "for this tool class the log is needed right away or not at all - its value decays in days"; parity MATCH with mkvtoolnix removeOldJobs=true/14 days.
- **Actual text:**
  - Rationale (line 81-82): "Şenol: for this tool class the log is needed right away or not at all - its value decays in days, and 14 days is a sane window."
  - Parity (line 97-99): "MATCH with mkvtoolnix-gui's default behavior: `m_removeOldJobs = true`, `m_removeOldJobsDays = 14` (settings.cpp:625-626 ...)."
- **Verdict:** Verbatim match on both the rationale quote and the parity claim. D35 is the reversal decision: automatic prune, 14 days, fixed, no v1 config, core layer (executor/joblog) so CLI and GUI both inherit, overruling the keep-forever+explicit-prune recommendation. **Distinct** from occurrence 2 (the decision-with-rationale vs the implementation-detail resolution). **Survives.**

## Cross-checks (beyond the three refs)

The cluster statement is not merely paper-consistent; it matches the shipped code and git history:

- **Source:** `crates/muxsmith-core/src/executor/joblog.rs` - `RUN_LOG_RETENTION = time::Duration::days(14)` citing D35 + IDEAS #7 (line 97-99); `prune_stale_runs` decides age "by the directory NAME alone, via run_id_timestamp, never by filesystem mtime" (line 108-112); symlink exclusion via `DirEntry::file_type()` (line 114-117); "Every I/O error ... is deliberately IGNORED" with the rustdoc why (line 119-124); called from `RunLogger::create` (line 218) so both surfaces inherit.
- **Git history:** `2ca5ddd docs: D35 run-log auto-prune decision (14 days, fixed, mkvtoolnix parity)`; `e8e85d9 docs: ... D35 rides as wave-1 Task 4.5 (vehicle decision)`; `3511efe feat(executor): auto-prune run logs older than 14 days (D35)`. Three distinct commits mirror the three occurrences.
- **ROADMAP** (line 135): "Run-log auto-prune implementation (D35): DONE 2026-07-12 (Plan 5.5 Task 4.5, merged d18f1b7)."

## Notes / caveats (non-disqualifying)

- **Date-field drift.** The cluster labels occurrences 2 and 3 as `2026-07-12`, but the underlying artifacts are dated `2026-07-11` (D35 memo header; verdict header "model: sonnet, 2026-07-11"; plan "added 2026-07-11"). The `2026-07-12` date matches the merge/DONE (commit d18f1b7, ROADMAP). This is metadata noise on the `date` field, not a misattribution of topic or `kind`, and does not affect distinctness or authenticity of any occurrence. No occurrence dropped on this basis.
- **No fabricated recurrence.** The three occurrences are genuinely distinct artifacts recording distinct events (deferral -> reversal-decision -> implementation-detail resolution), each supported verbatim or by faithful synthesis of the named ref. No occurrence is a duplicate, misattribution, or invention.

## Result

- Occurrence 1 (D26 deferred): **survives**
- Occurrence 2 (T4.5 verdict decided): **survives**
- Occurrence 3 (D35 decided): **survives**

**verified_count = 3.** Threshold for promotion (>=3 distinct real occurrences) met.
**Verdict: CONFIRMED** - the promotion stands as a standing house-knowledge convention.
