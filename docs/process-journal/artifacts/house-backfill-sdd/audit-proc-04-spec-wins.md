# Adversarial promotion audit: proc-04-spec-wins

- **Cluster:** `proc-04-spec-wins` (pattern, process, status=settled)
- **Claimed count:** 4 · **Promoted:** yes (`promoted_at: 3`)
- **Verdict:** **CONFIRMED** — 4 of 4 occurrences survive (>= 3 required). Promotion stands.
- **verified_count:** 4

Each occurrence was opened at its cited ref in `/home/senol/Git/Muxsmith` (journal
entry, memo, plan, task brief, `git show`) and checked for whether the artifact
actually supports that the spec-wins topic arose there as the stated `kind`. No
occurrence was fabricated, misattributed to the wrong artifact, or a duplicate of
another. One date-metadata error noted (occ 4), which is not a drop reason.

---

## Occurrence 1 — 2026-07-08 · `decided` · SURVIVES

**Ref:** journal 2026-07-08 "What the process caught" + Plan 1 Global Constraints + commits 3c24845, cd3f239/f7afa8d
**Claimed evidence:** spec-wins applied for a template prose fix; UnknownProperty name collision -> spec amended, code kept.

- `docs/process-journal.md:49-53` — "Template-error params carried English prose
  out of core, violating the plan's own exit criterion (final review; **spec-wins
  rule applied**)." and "UnknownProperty name collision between spec table and code
  (final review) -> **spec amended, code kept**." Both claims verbatim in the artifact.
- Plan 1 Global Constraints `2026-07-08-plan-1-core-foundations-validate-cli.md:13`
  — "Spec is authoritative... **On conflict, the spec wins; flag the conflict instead
  of improvising**." This is the pattern statement's exact wording, present as a
  standing plan constraint at project start.
- Commit `3c24845` "fix: final-review findings - **prose-free template errors**,
  match_to_source semantics, hardened tests" — the code-fixed-to-match-spec (spec-wins) side.
- Commits `cd3f239` ("split UnknownProperty ... record v1 i18n exceptions") and
  `f7afa8d` ("UnknownProperty covers match conditions only") — both touch only the
  spec file, confirming the "spec amended, code kept" corollary (code-wins only where
  the spec is wrong, then the spec is amended).

Verdict: fully supported as a `decided` event. Both corollaries (spec-wins on prose;
code-kept-then-spec-amended on the name collision) are grounded in named artifacts.

## Occurrence 2 — 2026-07-09 · `decided` · SURVIVES

**Ref:** memo 2026-07-09-plan-2-design-decisions.md header + commit d4390d7
**Claimed evidence:** normative D1-D5 folded into the v1 spec (4.4/4.8/5.2/5.4/9) so stale spec text can't silently override.

- Memo header `2026-07-09-plan-2-design-decisions.md:3-7` — "The normative rules
  (D1-D5) are **folded into the v1 design spec (sections 4.4, 4.8, 5.2, 5.4, 9)**;
  this memo remains the decision record... **On any spec/memo conflict the spec wins
  per repo convention**." Section list matches the claim exactly.
- Commit `d4390d7` "spec: fold final Plan 2 decisions into the v1 design spec" —
  body: "Amends 4.4 ..., 4.8 ..., 5.2 ..., 5.4 ..., 9 ...", "**Prevents the
  authoritative spec from silently overriding the decisions** (4.3+4.4 previously
  implied codec_kind under substring; a bad enum value previously degraded to
  MissingTrack)." Diff touches the spec + memo only.

Verdict: fully supported as a `decided` event. This is the "decision-in-a-memo can be
silently overridden by stale spec text -> fold it into the spec" corollary, verbatim.

## Occurrence 3 — 2026-07-12 · `decided` · SURVIVES

**Ref:** Plan 5.5 Global Constraints + T16 + memo D32
**Claimed evidence:** spec §9.2 amended to the decided semantics + a spec self-contradiction sweep after amending (doctrine §1).

- Plan 5.5 Global Constraints `2026-07-11-plan-5.5-pre-1.0-hardening.md:20` — "**Spec
  is authoritative over this plan on conflict**." (spec-wins doctrine as a standing constraint).
- T16 brief `.superpowers/sdd/plan-5.5/task-16-brief.md:7` (= plan line 363) —
  "Implement; **spec §9.2 gets amended to match the decided semantics exactly; spec
  self-contradiction sweep after amending (doctrine §1)**." Matches the claimed evidence word-for-word.
- Memo D32 `2026-07-11-plan-5.5-design-decisions.md:18-19` — "**Spec §9.2 is amended
  by T16 to exactly this semantics (self-contradiction sweep after)**."
- Execution confirmed, not just planned: commits `27c8b79` "feat(core): reachable
  forward-compat path for unknown properties (D32, #4)" and merge `2ec98d2`; the
  amended §9.2 semantics are now live in the v1 spec (`raw:` opt-in at
  `2026-07-08-muxsmith-v1-design.md:176,282,404`). D32 addendum dated "2026-07-12,
  T16 review" confirms the 07-12 date.
- "doctrine §1" is real: Plan 1 / Plan 5.5 both carry the "spec wins, flag not
  improvise" constraint that doctrine §1 codifies.

Verdict: fully supported as a `decided` event, and actually executed (spec file
changed), not merely a planned intention. Distinct artifact + event from occ 4.

## Occurrence 4 — 2026-07-12 · `reinforced` · SURVIVES (with a date-metadata caveat)

**Ref:** journal session 7 pt2
**Claimed evidence:** every spec-anchored gap -> implement pre-1.0 (spec is a binding contract).

- `docs/process-journal.md:443` (session 7 entry) — "Pattern: **every spec-anchored
  gap -> implement pre-1.0 (spec is a binding contract)**". The claimed evidence is
  present verbatim, including the "spec is a binding contract" phrase that is the
  core of proc-04's statement.

Caveat (not a drop reason): the session-7 journal entry is headed **2026-07-11**
(scope "session 2026-07-10/11"), whereas the occurrence records the date as
**2026-07-12**. This is an off-by-one date-metadata error, not a fabrication or
misattribution — the ref ("journal session 7") points to the correct artifact and
the content genuinely supports the claim. The facet is slightly broader than a
plan/code/spec conflict (spec *promises* are obligations to honor pre-1.0), but it is
explicitly grounded in "spec is a binding contract," so it legitimately *reinforces*
proc-04 rather than a neighboring pattern. Distinct from occ 3 (different artifact:
walkthrough journal vs Plan 5.5/T16/D32; different event: 07-11 walkthrough decision
vs 07-12 spec amendment).

---

## Distinctness check (no self-duplication)

| # | Date | Kind | Artifact | Event |
|---|------|------|----------|-------|
| 1 | 07-08 | decided | Plan 1 journal + commits 3c24845 / cd3f239 / f7afa8d | template prose spec-wins + UnknownProperty spec-amend |
| 2 | 07-09 | decided | Plan 2 memo header + commit d4390d7 | D1-D5 folded into spec |
| 3 | 07-12 | decided | Plan 5.5 GC + T16 + D32 + commits 27c8b79 / 2ec98d2 | §9.2 amended + self-contradiction sweep |
| 4 | 07-11* | reinforced | session-7 journal | spec-anchored gaps -> pre-1.0 (binding contract) |

Four distinct dates/artifacts/events. No two occurrences collapse into one.
(*occ 4 recorded as 07-12; artifact dated 07-11.)

## Conclusion

**CONFIRMED.** All four cited occurrences are real, grounded in named artifacts, and
distinct. The recurrence is genuine — this is not a fabricated count. proc-04-spec-wins
qualifies as a standing house convention; promotion stands. verified_count = 4.
