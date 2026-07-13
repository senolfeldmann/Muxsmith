# Audit: core-119-invalidpropertyvalue-allowed (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 3, promoted: true
**Statement under audit:** planner.rs emitted `InvalidPropertyValue` without the template-required `$allowed` on the `changes.language` path, rendering a literal `{$allowed}` to users; found by the T10 fixture-fidelity spot-check (per-DiagCode fixtures are structurally blind to single-site divergence), routed to T9(ix), fixed with a site-specific no-`{$` regression test.

**Verdict: REJECTED** — one of three occurrences is a duplicate; verified_count = 2 (< 3). Demote to Tier 1.

The underlying defect is real, correctly diagnosed, and cleanly fixed — none of the refs is fabricated and none is misattributed to the wrong topic. But the three listed occurrences are one defect's paper trail across the discovery→routing→fix workflow on a single day at a single site, and the middle ref (plan T9(ix)) is by its own text a transcription of the T10 finding, not an independent occurrence. Strip the duplicate and 2 distinct substantive artifacts remain, below the promotion bar of 3.

---

## The single defect (verified against source)

One defect, one site, one date:

- `crates/muxsmith-core/src/planner.rs`, `resolve_changes` (brief said `600-605`, actual `709-717`): emitted `InvalidPropertyValue` with only `property`+`value`; the FTL template's `Allowed values include: {$allowed}.` branch needs `$allowed`, which this site omitted. The sibling emitter `walk_exact_languages` (`planner.rs:388-396`, verdict/report also cite `:317`) set the full param set, which is exactly why the per-DiagCode leak guard stayed green.
- Fix (task-9 report item (ix), commit `408593e`): added `.with("allowed", "a valid ISO 639/BCP-47 language code")` plus regression test `invalid_changes_language_diagnostic_renders_without_placeholder_leak`, which renders THAT emitter's diagnostic through the CLI `Renderer` and asserts no `{$` substring. Before the fix it rendered the literal `Allowed values include: {$allowed}.`

All three cited refs point at this one defect on 2026-07-11.

---

## Per-occurrence verification

### Occurrence 1 — "task-10 verdict (fixture-fidelity spot-check)" (2026-07-11, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-5.5-sdd/task-10-verdict.md`

- Lines 12-21 record the discovery verbatim: *"fixture-fidelity spot-check across emitters found planner.rs:600-605 emits InvalidPropertyValue with only property+value while the template requires $allowed - the plan-time invalid-changes.language path renders a literal {$allowed} to users. Guard is green anyway: per-DiagCode fixtures are structurally blind to single-site divergence when sibling sites ... set the full param set."*
- Line 24: *"Production fix routed to plan T9 item (ix)."*
- This is the genuine discovery point: a real, distinct, substantive review artifact where the defect first surfaced. NOT fabricated, NOT misattributed, NOT a duplicate. Kind "violated-corrected" is loose (the correction is only *routed* here, not performed), but the topic unambiguously and substantively arose here. SURVIVES.

### Occurrence 2 — "task-9 verdict (ix)" (2026-07-11, kind: violated-corrected) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-5.5-sdd/task-9-verdict.md`

- Line 18-20, item (ix): *"both emitter sites identical, site-specific no-{$ regression drives the real emitter, guard note updated."* Independent review verdict approving the executed fix.
- Distinct from Occurrence 1: that artifact *found* the leak (in the T10 guard work); this one *verifies the fix landed* (regression test drives the real emitter, sites reconciled). Different lifecycle stage, different substantive content, different file. By the same standard the sibling audit core-40 used to keep a verification review as its own occurrence, this SURVIVES.

### Occurrence 3 — "plan T9(ix)" (2026-07-11, kind: violated-corrected) — DROPPED (duplicate of Occurrence 1)

Ref: `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md`, line 272

- The item text opens: *"(ix) (added 2026-07-11, **T10-review finding**) planner.rs:600-605 emits InvalidPropertyValue with only property+value - the template also requires $allowed, so the plan-time invalid-changes.language path renders a literal {$allowed} to the user. Fix: set allowed (mirror the sibling site planner.rs:317). Regression test: ... assert no {$ substring ..."*
- By its own label this item **is** the T10-review finding (Occurrence 1), transcribed into the plan as the routing target. The defect does not *arise* independently in the plan — it is imported verbatim from the T10 verdict, with the fix direction appended. As an *occurrence of the topic*, it records nothing that Occurrence 1 (the discovery) and Occurrence 2 (the fix review) do not already carry: it is the connective routing note between them, not a third independent event.
- This is the audit's textbook "duplicate of another listed occurrence." DROP.

---

## Why this differs from the sibling audit core-40 (which CONFIRMED a lifecycle at 3)

core-40 kept a decision → implementation-commit → review lifecycle at 3 because each ref was a **genuinely distinct work product with independent substance** (a Şenol decision; real code + 171 lines of new tests; a review that ran the suite and found a *new* regression). None self-cited another as its source.

core-119 has only **two** such independent substantive artifacts — the discovery review (Occ 1) and the fix review (Occ 2). The third listed ref (plan T9(ix)) is a routing note that explicitly names itself "T10-review finding," i.e. a pointer back to Occ 1, carrying no independent occurrence of the defect and no independent verification. There is also no separate implementation-commit occurrence in the list (commit `408593e` / task-9-report is folded inside Occ 2's verdict, not counted on its own). So the real distinct-artifact count is 2, and one of the three listed entries is a strict duplicate.

Applied consistently with core-40's own drop test ("fabricated / misattributed-topic / duplicate-of-another"), core-40 had zero qualifying drops; core-119 has one.

---

## Nature-of-count note

Beyond the duplicate: all three entries share one date (2026-07-11), one site (`planner.rs` `resolve_changes`), one root cause, and one find-and-fix cycle. Each is typed "violated-corrected," but there was exactly ONE such cycle — Occ 1 captures its "found" half and Occ 2 its "corrected" half. This is a single incident, not a pattern recurring across independent contexts. Even under the most generous lifecycle-splitting reading (discovery and fix as two occurrences) the count is 2 — a house-knowledge convention should not be minted from a single, well-handled bug.

**verified_count = 2 -> REJECTED (< 3). Demote to Tier 1.**
