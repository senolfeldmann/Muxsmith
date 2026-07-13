# Audit: ci-01-matrix-strategy (PROMOTION candidate)

**Cluster:** `ci-01-matrix-strategy` (kind: pattern, domain: ci)
**Claimed status:** settled, count 3, promoted, promoted_at 3
**Audit verdict:** **CONFIRMED** - 3 of 3 occurrences survive; promotion stands.
**Auditor stance:** adversarial (goal: no fabricated recurrence becomes a standing convention).

## Statement under audit

> While the repo is private, branch pushes run Linux only and the full windows/macos/linux
> matrix runs only on PRs, tags and manual dispatch via a dynamic fromJSON matrix (macOS bills
> 10x, Windows 2x); the trim was always framed as reverting to a static 3-OS matrix on every
> push once the repo goes public, and that revert was enacted at go-public.

Every load-bearing clause of the statement is corroborated by primary artifacts (see per-occurrence checks). The statement is factually accurate against the repo history, not just plausible.

## Per-occurrence verification

### Occurrence 1 - 2026-07-08, kind=`decided` -> SUPPORTED

Ref: `spec 2026-07-08 §10 + journal 2026-07-08 'CI while private' + commit 97ae031`
(evidence note: E0 [3] and E1 [4] deduped - same three artifacts, same decision)

- **commit 97ae031** (`git show`): title "ci: linux-only on branch pushes while private; full matrix on PR/tag/dispatch". Diff to `.github/workflows/ci.yml` replaces the static `os: [ubuntu, windows, macos]` list with the dynamic
  `os: ${{ fromJSON((github.event_name == 'push' && startsWith(github.ref, 'refs/heads/')) && '["ubuntu-latest"]' || '["ubuntu-latest","windows-latest","macos-latest"]') }}`,
  adds `tags: ['v*']` and `workflow_dispatch` triggers, and carries the inline comment "Revert to a static 3-OS list on going public." Directly implements the decision. CONFIRMED.
- **spec §10** (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, "## 10. Testing"): the CI bullet reads "... While the repo is private, branch pushes run Linux only (Actions minute multipliers: Windows 2x, macOS 10x); the full matrix runs on PRs, tags and manual dispatch. Reverts to the full matrix on every push when the repo goes public." The same commit 97ae031 added this sentence (diff touches the spec). CONFIRMED.
- **journal 'CI while private'** (`docs/process-journal.md`, 2026-07-08 session-1 entry, "Decisions and why" section, lines 38-39): "CI while private: dynamic fromJSON matrix, Linux on branch pushes, 3-OS on PR/tags/dispatch (macOS bills 10x); revert on going public." Recorded as a settled decision. CONFIRMED.

All three cited artifacts corroborate ONE decision; the evidence note's E0/E1 dedup is honest (it collapses two evidence items into one occurrence, it does not inflate the count). Kind `decided` is correct: the item sits in the journal's Decisions section and was implemented in the same session.

### Occurrence 2 - 2026-07-08, kind=`deferred` -> SUPPORTED

Ref: `journal 2026-07-08 Pending decisions + handoff plan-1-close`

- **journal Pending decisions** (`docs/process-journal.md`, 2026-07-08 session-1 entry, "Open threads" section, line 103): "Pending decisions: go-public timing + CI matrix revert; Dependabot cadence if enabled." An explicitly open/deferred decision about the CI matrix strategy, tied to go-public timing. CONFIRMED as a deferral.
- **handoff plan-1-close** (`docs/process-journal/artifacts/plan-1-sdd/progress.md`, line 41 - the plan-1 progress/handoff ledger): "First CI run 28931578050: SUCCESS on all 3 OSes (2m44s). Repo currently PRIVATE - Actions burns free-tier minutes with macOS 10x multiplier; matrix-trim or go-public decision pending with Şenol." Independently records the same pending decision at plan-1 close. CONFIRMED.

Kind `deferred` is correct: both artifacts frame the go-public timing / matrix revert as an open item that is Şenol's call.

**Distinctness from occurrence 1 (adversarial check):** Not a duplicate. Occ 1 cites the journal Decisions section + spec + commit and records the *settled mechanism* (linux-only-while-private dynamic matrix). Occ 2 cites the journal Open-threads section + the progress ledger and records the *still-open timing/trigger* (when to go public, hence when to flip back). Different artifacts, different sections, different facet (settled mechanism vs. deferred trigger). The journal itself keeps the two in separate sections, and the progress ledger records the pending item independently. The decided->deferred->decided arc is coherent and each node is in the record. Occ 2 is a genuine, separately-documented occurrence, not a re-description of occ 1.

### Occurrence 3 - 2026-07-10, kind=`decided` -> SUPPORTED

Ref: `commit 226fa06 + journal Plan 5`

- **commit 226fa06** (`git show`): title "ci: static 3-OS matrix on every push (go-public)". Diff to `.github/workflows/ci.yml` replaces the dynamic fromJSON expression with a static `os: ["ubuntu-26.04","windows-2025","macos-15"]`, and rewrites the comment to "Static 3-OS matrix since going public (2026-07-10) ... The private-era dynamic Linux-only trim is retired." Directly enacts the framed revert. CONFIRMED.
- **journal Plan 5** (`docs/process-journal.md`, 2026-07-10 "Plan 5 complete ... + go-public" entry, "Decisions and why", line 384): "Go-public decided mid-close-out when the 3-OS verification cost question came up ('GitHub gives more resources; reversible'). Pulled two gates forward: ConcurrencyTracker doc(hidden), static 3-OS matrix." CONFIRMED. (Corroborated further by the same-day "Plan 5 close addendum" entry documenting the first-ever 3-OS runs on the post-go-public static matrix.)

Kind `decided` is correct: go-public was decided and the static matrix enacted in that session.

## Result

| Occ | Date | Kind | Ref supports kind? | Distinct? | Fabricated/misattributed? |
|-----|------|------|--------------------|-----------|---------------------------|
| 1 | 2026-07-08 | decided | yes | yes | no |
| 2 | 2026-07-08 | deferred | yes | yes (distinct artifacts + facet) | no |
| 3 | 2026-07-10 | decided | yes | yes | no |

- Occurrences dropped: **0**
- verified_count = **3**
- **Verdict: CONFIRMED** (>=3 survive; promotion stands). No occurrence is fabricated, misattributed, or a duplicate; the statement is accurate against primary artifacts (workflow diffs, spec §10, journal, plan-1 progress ledger).
