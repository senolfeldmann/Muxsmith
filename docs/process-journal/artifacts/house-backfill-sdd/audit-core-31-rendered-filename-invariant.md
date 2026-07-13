# Audit: core-31-rendered-filename-invariant (PROMOTION candidate)

**Verdict: CONFIRMED** — 4/4 occurrences survive; promotion to standing house-knowledge stands.

Adversarial audit of the promotion candidate `core-31-rendered-filename-invariant`
(kind `pattern`, domain `core`, status `settled`, claimed count 4, `promoted: true`).
Each cited ref was opened in `/home/senol/Git/Muxsmith` and checked against the test
"does this artifact support that (rendered-filename invariant, re-check + two new
per-file codes) arose here as `{occ.kind}`?". Drop criteria applied: fabricated,
misattributed, or duplicate of another listed occurrence.

## Statement under audit

The planner re-checks the rendered output filename independently of the config-time
template-text check (no separators, non-empty stem, not `.`/`..`); new per-file errors
`PathSeparatorInRenderedName` + `EmptyRenderedName`. The empty-stem case
(template `''` -> `.mkv` -> hidden file) slipped through the fix pass because the check
ran on the pre-append value; whole-branch review caught what per-task reviews missed.

## Per-occurrence verification

| # | ref | kind | result |
|---|-----|------|--------|
| 1 | memo D4 | decided | SURVIVES |
| 2 | commit af76a3a | decided | SURVIVES |
| 3 | FINAL review I1 | violated-corrected | SURVIVES |
| 4 | commit 59d24c8 | violated-corrected | SURVIVES |

### 1. memo D4 — `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md` §D4 — SURVIVES

"D4: Rendered-filename invariant in the planner." Decides the planner re-checks the
RENDERED output filename after interpolation, *independently of* the config-time
`PathSeparatorInTemplate` check on template text, and introduces the two new per-file,
error-severity diagnostics `PathSeparatorInRenderedName` and `EmptyRenderedName`
(empty stem -> hidden `.mkv`, plus `.`/`..`). Carries the steelman verbatim ("Guard the
invariant, not the induction proof over field sources").

Provenance check (guards against a post-hoc backfilled "decision"): the memo was
committed 2026-07-09 00:23-00:27 (`3b71a71` pre-decide, `d4390d7` fold), i.e. ~30 min
*before* the implementation commit af76a3a (01:01). Genuine forward-looking decision,
not reconstructed after the fact. `decided` attribution correct.

### 2. commit af76a3a — SURVIVES

`feat(core): add Plan 2 diagnostic codes (D1/D2/D4) with messages`, real ancestor of
HEAD. Diff genuinely *adds* (not merely touches) the two variants:
`+ PathSeparatorInRenderedName => "path-separator-in-rendered-name"`,
`+ EmptyRenderedName => "empty-rendered-name"` in `report.rs`, plus their Fluent
messages and catalog-completeness assertions. This is the codification stage of D4 —
a distinct artifact (commit/code) from occ.1 (spec prose), adding genuinely new state
(the codes now exist in the catalog). `decided` attribution correct.

### 3. FINAL review I1 — `docs/process-journal/artifacts/plan-2-fixes-sdd/FINAL-review.md` §I1 — SURVIVES

"F6: template rendering to `.mkv` produces a hidden empty-stem output instead of
`EmptyRenderedName` (regression)." The review header states it is the whole-branch,
independent, read-only review "for cross-cutting correctness the per-task reviews could
not see" — matching the statement's "whole-branch review caught what per-task reviews
missed" exactly. I1 pinpoints that `render_output` tested the raw *pre-append* `rendered`
value, so `.mkv` passed and produced the hidden file. This is the detection half of the
violation. `violated-corrected` attribution correct.

### 4. commit 59d24c8 — SURVIVES

`fix(core): catch .mkv-literal empty-stem output (I1)`, real ancestor of HEAD. Diff
moves the emptiness/dot check off the pre-append `rendered` value onto the post-strip
stem (`let stem = strip_mkv_suffix(&name); if stem.is_empty() || stem == "." ...`),
"which subsumes the old pre-append check." The correction half of the same violation,
a distinct artifact (code fix) from occ.3 (review doc). `violated-corrected` attribution
correct.

## Skeptic's caveat (recorded, not count-affecting)

The four occurrences collapse to **two underlying events documented at two lifecycle
stages each**:

- Event A (decided): memo D4 decision (occ.1) -> af76a3a codification (occ.2)
- Event B (violated-corrected): I1 detection (occ.3) -> 59d24c8 fix (occ.4)

A strict "distinct-event" reading could argue the cluster rests on 2 events, not 4
independent recurrences. This does **not** trigger a drop under the audit's duplicate
criterion: a droppable duplicate is two refs pointing at the same artifact/event with
nothing new. Here each pair is decision-record vs implementation-commit and
detection-doc vs fix-commit — distinct artifacts, distinct timestamps, each adding real
content (the codes actually get created; the bug actually gets fixed). All four are real,
non-fabricated, and correctly attributed to the topic.

Even under the harshest collapse to 2 events, the pattern is genuinely *settled*
(decided, implemented, regressed-and-fixed via an invariant-specific whole-branch review)
and the statement is fully substantiated by real branch artifacts. Promotion stands
either way.

## Result

- verified_count = **4** (surviving distinct occurrences)
- Threshold: >=3 survive -> **CONFIRMED**, promotion holds.
- No occurrence dropped.
