# Audit: core-46-symlink-source-discovery (PROMOTION candidate)

**Cluster kind:** pattern / domain core / status settled
**Claimed:** count 3, promoted, promoted_at 3 (would become a standing convention).
**Verdict: REJECTED** — 1 distinct violated-corrected occurrence survives (< 3). Demote to Tier 1.

## Statement under audit

> A symlink whose target is a regular file must be discovered (classify via `fs::metadata`); do not recurse into directory symlinks (cycle guard); skip broken symlinks silently. Bug I: `walk_files` under `symlink_metadata` dropped symlinked source files with no diagnostic.

The statement itself is technically correct and matches the shipped code (`crates/muxsmith-core/src/discovery.rs`, `walk_files` symlink arm). The audit question is not whether the pattern is *true* — it is whether it *recurred ≥3 independent times*, as a promotion to a standing convention requires. It did not.

## The three cited refs all trace to ONE bug on ONE day

Every ref is a stage in the lifecycle of a single defect: **bug I == task F8**, surfaced 2026-07-09, in one function (`walk_files`), fixed by one implementer episode.

Binding evidence that F8 and bug I are the same thing, not two findings:
- `docs/superpowers/plans/2026-07-09-plan-2-fixes.md:50` — task header reads verbatim **"F8: discovery symlink handling (bug I)"**.
- `docs/process-journal/artifacts/plan-2-fixes-sdd/F8-report.md:1` — **"F8: discovery symlink handling (bug I) - report"**.
- Commit `cb3ae84` subject — **"fix(core): discover symlinked source files (F8, bug I)"**.

Lifecycle:
1. Plan 2 independent review finds bug I (discovery / the violation).
2. `cb3ae84` fixes it (the correction).
3. F8-review reviews the fix → **SPEC pass**, flags a residual test-coverage gap.
4. `608f2b5` adds the missing tests (correction of the coverage gap from step 3).

## Per-occurrence verdict

### Occurrence 1 — "independent review bug I" — **KEEP**
Ref: `docs/process-journal/artifacts/plan-2-review/independent-review-2026-07-09.md`, line 26 and line 80.
- Line 26: "I. Symlinked source files dropped by walk_files (neither is_file nor is_dir) with no diagnostic."
- Line 80: "CONFIRMED discovery.rs:197-222 - symlinked source files dropped (symlink is neither is_file nor is_dir), no diagnostic. [I]"

Real artifact; genuinely supports "symlink source discovery arose here as violated-corrected." This is the canonical origin of the finding. The one surviving distinct occurrence.

### Occurrence 2 — "F8 review (SPEC pass)" — **DROP (misattributed)**
Ref: `docs/process-journal/artifacts/plan-2-fixes-sdd/F8-review.md`.
The F8-review reviews the *fix* `cb3ae84`. Its verdict for the discovery pattern is explicit:
- Line 40: **"Verdict: SPEC pass."** — "All four required behaviors ... are implemented correctly and match the task."
- Line 53: "Verdict: QUALITY changes-needed — **not because anything is broken**, but because one explicitly-specified behavior (broken-symlink skip) has no regression test."

A SPEC-pass review is the *opposite* of a violated-corrected occurrence of the pattern: it confirms the pattern was applied correctly. What it flags is a test-coverage QUALITY gap (test hygiene), a different concern from the symlink-discovery pattern being violated. Counting a validation-plus-coverage-nit as a "violated-corrected" instance of the discovery pattern is a misattribution. Its warning about a *hypothetical future* `.unwrap()` regression (finding 1) is forward-looking risk, not a second past occurrence.

### Occurrence 3 — "commits cb3ae84/608f2b5" — **DROP (duplicate of occurrence 1)**
Refs verified via `git show`:
- `cb3ae84` "fix(core): discover symlinked source files (F8, bug I)" — the **correction** of the exact bug occurrence 1 is the **finding** of.
- `608f2b5` "test(core): cover broken-symlink skip and multi-hop chain in walk_files (F8)" — adds tests only, no production change; it is the correction of the coverage gap from occurrence 2, binding it to the same episode.

A "violated-corrected" occurrence is one instance where the pattern was violated and then fixed. The review that *found* bug I (occ 1) and the commits that *fixed* bug I (occ 3) are the two halves of that single instance, not two instances. Listing both double-counts one event.

## Why this fails the promotion gate

The ≥3-occurrence gate exists to separate a *recurring class* (the same mistake made independently in multiple places or at multiple times) from a *single well-documented bug*. Here all three refs are: one bug (bug I = F8), one day (2026-07-09), one function (`walk_files`), one implementer episode. If "found it + reviewed it + fixed it" counted as three occurrences, every bug in the project would auto-qualify, emptying the gate of meaning. This is one bug with a thorough paper trail, not a recurrence.

## Result

- verified_count: **1** distinct violated-corrected occurrence (occurrence 1).
- Verdict: **REJECTED**. The promotion does not stand; demote `core-46-symlink-source-discovery` to Tier 1.
- Dropped: occ 2 (misattributed — SPEC pass, not a violation of the pattern), occ 3 (duplicate — the correction half of occ 1's single event).
