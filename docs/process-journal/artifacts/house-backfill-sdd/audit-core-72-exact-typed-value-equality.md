# Audit: core-72-exact-typed-value-equality (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 3, promoted: true (at 3)
**Statement under audit:** `exact` compares each property in its own domain (numbers numerically, languages canonicalized so `de==ger`, `pt-Latn-BR==pt-BR`), preserving meaningful distinctions; `regex` is the byte-literal escape hatch. One of the tool's core semantics; surfaced in spec 4.3 at 1.0.

**Verdict: CONFIRMED** — all 3 occurrences survive; verified_count = 3. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no strict duplicate. The principle is real, deliberately decided by Şenol, encoded verbatim into the authoritative spec, and independently reinforced by a whole-branch review that singled it out as load-bearing. The statement is fully faithful to the artifacts (every clause — own-domain compare, `6`==`6.0`, `de`==`ger`, `pt-Latn-BR`==`pt-BR`, `pt-BR`!=`pt-PT`, `regex` as literal escape hatch, "core semantics", "spec 4.3 at 1.0" — is present in the cited refs). One honest caveat on the *nature* of the count is recorded at the end; it does not meet the drop bar.

All three occurrences are dated 2026-07-09.

---

## Per-occurrence verification

### Occurrence 1 — "memo D19 Principle" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md`, D19, lines 78-85.

- Verbatim, under an explicit **"Principle (README/guide-worthy, Şenol 2026-07-09)"** header:
  `Muxsmith's exact operator is typed value-equality, not raw string equality: each property is compared in its own domain. Numbers compare numerically (6==6.0); languages compare as languages, ISO spellings and BCP-47 tags reduced to canonical form (de==ger, pt-Latn-BR==pt-BR) while meaningful distinctions survive (pt-BR!=pt-PT). Byte-literal matching remains available via regex. This is one of the tool's core semantics; surface it in the public docs at 1.0. Task 5 writes it into spec 4.3.`
- This is the origin decision, attributed to Şenol on the date, and it carries **every** clause of the statement — including the two that a paraphrase would be most likely to invent (`6`==`6.0` and the `regex` escape hatch). Kind "decided" is exact.
- Rock solid.

### Occurrence 2 — "spec 4.3 (Task 5 step 6)" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` §4.3, line 146 (the authoritative spec), written there by plan Task 5 step 6.

- Spec §4.3 line 146, verbatim:
  `exact is typed value-equality, not raw string equality: each property is compared in its own domain. Numbers compare numerically (6 == 6.0); languages compare as languages, with ISO 639 spellings and BCP-47 tags reduced to canonical form (de == ger, pt-Latn-BR == pt-BR) while meaningful distinctions are preserved (pt-BR != pt-PT, zh-Hans != zh-Hant). Use regex for byte-literal matching.`
- The "Task 5 step 6" anchor checks out: `docs/superpowers/plans/2026-07-09-plan-3.5-mkvtoolnix-parity.md` Task 5 **Step 6** (lines 509-513) is titled *"State the principle in the spec"* and quotes exactly the paragraph that landed in §4.3. Task 5's file list (line 432) names `…muxsmith-v1-design.md section 4.3 (the exact principle)`, and the commit message (line 526-527) documents it.
- Distinct artifact from Occ 1: the **authoritative spec** (which the memo itself declares wins on conflict), not the decision log. The topic is now the settled, standing specification of the tool's match semantics.
- **Kind caveat (does not trigger drop):** this is the *encoding* of Occ 1's decision into the spec, on the same date, per Occ 1's own instruction ("Task 5 writes it into spec 4.3"). "decided" is loose — "encoded" / "implemented" would be exact — but the topic materially arose here in a distinct, load-bearing artifact, so it is kept, not dropped.

### Occurrence 3 — "whole-branch verdict (load-bearing flag)" (2026-07-09, kind: reinforced) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/whole-branch-review-verdict.md`, Recommendations, line 62 (Plan 3.5 whole-branch review, target b04c4a2..2b08de4, reviewer subagent session ffc7c915, final message 2026-07-09T18:02:41Z).

- Verbatim:
  `The "exact is typed value-equality, not string equality" principle now stated in spec 4.3 is genuinely load-bearing and easy to lose; the self-review already flags it for the 1.0 README pass. Keep that flag alive — a user who assumes exact is strcmp will be surprised by de == ger and pt-Latn-BR == pt-BR in the opposite direction from the surprises above.`
- This is the "load-bearing-and-easy-to-lose flag" the ref names, exactly. Kind "reinforced" is precise: an **independent** reviewer, doing the cross-cutting whole-branch view, singled out the already-stated principle as important to preserve and told the team to keep the 1.0-README flag alive. It decides nothing new and encodes nothing new — it reinforces.
- Genuinely distinct occurrence: distinct artifact (a review verdict), distinct author (an independent reviewer subagent, not Şenol and not the implementer), distinct role (reinforcement). This is the one occurrence that is *not* part of the decision-to-spec pipeline, and it is what lifts the cluster from "one decision" to "a decision an outside reviewer independently judged load-bearing."

---

## Corroboration outside the three refs

The principle is not orphaned in the three cited spots; it is threaded through the process record consistently, which is what a settled core semantic should look like:

- `docs/process-journal.md` line 241 (Plan 3.5 journal): `Surfaced "exact = typed value-equality, not string equality" -> spec 4.3, README-flagged.`
- The matcher implementation encodes the match-side half (`crates/muxsmith-core/src/matcher.rs` `lang_eq`, canonical-form compare with raw fallback), with tests `lang_eq_canonical_forms_match` (the discriminator) and `lang_eq_preserves_meaningful_distinctions` (the forward guard) — both quoted in the plan Task 5 and confirmed present by the whole-branch verdict's triage (items 4-5).
- The whole-branch verdict independently walked `exact: { language: pt-BR }` against a `por` + `language_ietf: pt-BR` track and confirmed the canonical-match behavior (Strengths, line 26).

## Honest caveat on the *nature* of the count (recorded, below the drop bar)

Occurrences 1 and 2 are one decision realized on a single date: the decision (Occ 1, memo D19) explicitly orders the spec encoding (Occ 2, "Task 5 writes it into spec 4.3"), and Occ 2 is that encoding. Occ 2's kind label "decided" is imprecise for what is really an *encoding* of an already-made decision. So "count: 3" should be read as "one decision, encoded into the authoritative spec, then independently reinforced by review," not "this issue kept cropping up in three unrelated contexts."

That said, this cluster is a **stronger** promotion case than a pure decision-lifecycle triple: Occurrence 3 is genuine independent reinforcement — a separate reviewer, on the whole-branch pass, flagged the principle as load-bearing and easy to lose on its own judgment. That is exactly the signal a standing convention wants: a deliberate Şenol decision, encoded in the spec that governs the product's core match semantics, and independently affirmed as worth preserving. Under the audit's drop criteria (fabricated / misattributed-topic / duplicate-of-another) none of the three qualifies, so the count is not reduced.

**verified_count = 3 -> CONFIRMED.**
