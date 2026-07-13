# Audit: core-59-matchable-trait (PROMOTION candidate)

**Cluster kind:** pattern (domain: core), status: settled, claimed count: 3, promoted: true (at 3)
**Statement under audit:** The matcher is genericized over a `Matchable` trait implemented for both `Track` and `Attachment` so one match algebra evaluates both. The plan brief's "resolves via type inference" regression-guard claim was factually wrong (`Iterator::filter` hands `&&Track`); solved with a blanket `impl<M: Matchable> Matchable for &M`, verified sound and right-sized.

**Verdict: CONFIRMED** — all 3 occurrences survive; verified_count = 3. Promotion to standing house-knowledge stands.

Nothing fabricated, nothing misattributed to the wrong topic, no strict duplicate-of-another-ref. Both halves of the statement (the `Matchable` architecture decision AND the `&&Track` blanket-impl correction) are backed by real artifacts. One honest caveat on the *nature* of the count is recorded at the end; it does not meet the drop bar (same disposition as the sibling core-40 audit).

---

## Per-occurrence verification

### Occurrence 1 — "Plan 3 Architecture + Task 2/3" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/superpowers/plans/2026-07-09-plan-3-resolution-command.md`

- **Architecture (line 7):** "Generalize the `matcher` over a `Matchable` trait so the same match algebra evaluates track rules and attachment rules." Verbatim origin of the pattern's first half (one match algebra over `Matchable` for both item kinds).
- **Task 2 (lines 250-288):** "generalize the matcher over a `Matchable` trait" — produces `pub trait Matchable { fn get(&self, prop: &str) -> Option<PropValue>; }` impl'd for `Track`, `matches` widened `&Track` -> `&M`.
- **Task 3 (lines 292-324):** "attachment matching (`Attachment: Matchable`)" — `impl Matchable for Attachment` so `matches` evaluates attachment rules "with the same algebra." This is the "implemented for both `Track` and `Attachment`" clause, concretely planned.
- **Corroborates the "factually wrong" clause:** Task 2 Step 4 (line 287) contains the exact claim the statement calls out — "planner calls `matcher::matches` with `&Track`, still resolves via type inference." So the brief genuinely made the regression-guard claim that later proved wrong. Not the auditor's inference; it is in the plan text.
- This is the genuine origin decision of the pattern. Kind "decided" is exact.

### Occurrence 2 — "journal Plan 3, &&Track unification" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/process-journal.md`, section **"2026-07-09 | Plan 3 complete (pure layer: resolution + command)"** (lines 202-231), bullet at line 214.

- Correctly attributed to the **Plan 3** entry (section spans 202-231; Plan 3.5 starts at 233). Not misfiled to a neighboring plan.
- Line 214 verbatim: "Task 2 `&&Track` unification (implementer-caught during impl): planner's `.iter().filter(|t| ...)` hands `&&Track`, so `M` unified to `&Track`; solved with a blanket `impl<M: Matchable> Matchable for &M`, no planner change. The brief's stated regression-guard ('resolves via type inference') was factually wrong."
- This is the pattern statement's second half almost word-for-word (the `&&Track` mechanism, the blanket `&M` fix, the wrong brief claim). Real, distinct artifact.
- **Kind caveat (does not trigger drop):** the journal is a retrospective session record, not the point where the fix was decided. "decided" is loose; the decision to adopt the blanket impl happened during implementation (occ 3's context). The topic genuinely and materially arose here as a recorded design outcome, so it is kept, not dropped.
- **Overlap note (see final caveat):** this occurrence describes the same sub-event as Occurrence 3.

### Occurrence 3 — "task-2 verdict, blanket &M impl" (2026-07-09, kind: decided) — SURVIVES

Ref: `docs/process-journal/artifacts/plan-3-sdd/verdicts/task-2-review-verdict.md` (salvaged from the SDD session transcript; byte-faithful to the reviewer subagent's final message, `final_message_ts: 2026-07-09T10:54:03Z`).

- Distinct artifact from occ 2: authored by the independent review subagent, not the controller's journal. Substantive independent work, not a copy — the reviewer read `planner.rs:355-358`/`:704-707` to confirm the double-reference is real, ran `cargo check --workspace --all-targets` on the `8a2defc` tree, and traced runtime deref termination.
- Directly backs the statement's second half:
  - Line 24-28: named-risk check on `impl<M: Matchable> Matchable for &M` "confirmed sound" (clean compile, no coherence conflict, no infinite trait/runtime recursion).
  - Line 25: "Verified the double-reference claim is real, not invented ... `Iterator::filter`'s `FnMut(&Self::Item)` hands the closure `t: &&Track`" — the `&&Track` mechanism.
  - Line 28: "It is the correct minimal generalization" — the "right-sized" clause verbatim in substance.
  - Line 33: "the brief's regression-guard claim ('resolves via type inference') was factually wrong" — the "factually wrong" clause.
- Topic unambiguously arose here in a real, distinct artifact.
- **Kind caveat (does not trigger drop):** this is a *verification review* adjudicating the implementer's deviation as accepted, not a fresh decision. "decided" is imprecise; "reviewed"/"confirmed" would be exact. Kept because the topic substantively arose here.

---

## Corroboration outside the three refs

The pattern is encoded in shipped source, not just prose. `crates/muxsmith-core/src/matcher.rs` carries the `Matchable` trait, `impl Matchable for Track`, the blanket `impl<M: Matchable> Matchable for &M`, and `impl Matchable for Attachment` (Task 3), with `matches`/`exact_matches`/`item_str` generic over `M: Matchable`. The task-2 verdict's line references (matcher.rs:18-41) and the plan's Task 2/3 specs match this. The rule is settled, coded, and independently reviewed — the profile a standing convention should have.

---

## Honest caveat on the *nature* of the count (recorded, below the drop bar)

The three occurrences resolve to **two distinct facets on one date (2026-07-09):**

- **Facet A — the architecture decision** (occ 1): genericize the matcher over `Matchable` for both `Track` and `Attachment`, one algebra.
- **Facet B — the `&&Track` blanket-impl correction** (occ 2 AND occ 3): the implementer-caught deviation and its `impl<M: Matchable> Matchable for &M` fix.

Occurrences 2 and 3 document the *same* sub-event (Facet B) in two different artifacts — the independent review verdict (occ 3, the originating adjudication) and the controller's retrospective journal (occ 2, the record of it). They are not a strict duplicate under the audit's drop criteria: different files, different authors, independent existence, and this cluster file counts artifact-threaded lifecycles this way throughout (e.g. core-40: decision + commit + review of the same decision; core-119: two verdicts + plan for one bug). The sibling core-40 audit kept all three of a single decision's lifecycle artifacts on exactly this reasoning; consistency demands the same here.

So "count: 3" should be read by a future consumer as "one architecture decision plus one implementation-caught fix, the fix evidenced in both the review and the journal" — not "this issue recurred independently three times." Two of the three "decided" kind labels are imprecise (occ 2 is a *record*, occ 3 is a *review*). None of that meets the drop bar: nothing is fabricated, nothing is misattributed to the wrong topic, and no ref duplicates another ref's artifact.

**verified_count = 3 -> CONFIRMED.**
