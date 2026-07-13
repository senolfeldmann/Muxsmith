# Adversarial promotion audit: core-87-version-floor

**Cluster:** `core-87-version-floor` (`pattern`, `core`, `settled`)
**Statement:** The mkvmerge version floor `MIN_SUPPORTED = (86,0)` was derived empirically from the identification-schema v19->v20 diff traced to NEWS.md v86.0, with the evidence in the const doc; independently re-derived by the reviewer.
**Claimed:** count 3, promoted (Tier 1 -> Tier 2 at `promoted_at: 3`).
**Audit question:** Are the three cited occurrences real, distinct, and supporting, so the recurrence count that promoted this to an always-loaded standing convention is genuine?

**Verdict: REJECTED. verified_count = 2. Demote to Tier 1.**

The topic is factually impeccable and well-evidenced. It fails promotion on *count integrity*: the third occurrence is not an independent recurrence, and it is counted here in a way its own task-siblings are not. Threshold 3 exists (per `docs/decision-ledger.md`) "to avoid overfitting one-offs"; this is a one-off.

---

## Occurrence-by-occurrence

### Occ 1 — "Plan T3 step 1" (`decided`, 2026-07-10) - SURVIVES

Real artifact. Plan-5 task-3 brief, Step 1 ("Empirically fix the floor (SI-3)"), present at both `.superpowers/sdd/task-3-brief.md` and `docs/process-journal/artifacts/plan-5-sdd/task-3-brief.md`:

> Step 1: Empirically fix the floor (SI-3). The capability table is generated from identification schema v20; find which mkvtoolnix release introduced format version 20 ... `MIN_SUPPORTED` = that release. Record the evidence in the const's doc comment.

This is where the approach (derive the floor empirically from the schema-v20 release, evidence in the const doc) was specified. Supports "topic arose here as `decided`." Genuine, distinct.

### Occ 2 — "task-3 verdict (inductive step confirmed)" (`decided`, 2026-07-10) - SURVIVES

Real artifact, and the strongest of the three. Located at `docs/process-journal/artifacts/plan-5-sdd/verdicts/task-3-review-verdict.md` (outside `.superpowers/`; a byte-faithful salvage of the reviewer subagent's final message, session `62503ddd-...`). The reviewer independently re-derived the entire SI-3 chain against `~/Downloads/mkvtoolnix` rather than trusting the report:

- `src/merge/id_result.h:37` -> `ID_JSON_FORMAT_VERSION = 20` confirmed;
- direct `diff` of schema v19 vs v20 (five enumerated `tag_*` props -> open `patternProperties`);
- `NEWS.md` "Version 86.0" entry (line 499) matches that change; no schema-affecting entry in v83/84/85 between the v82.0 explicit "bumped to 19" (line 633) and v86.0 -- "the inductive step ('v86.0 is therefore the release that moved 19->20') holds."

A genuinely independent reinforcement event, by a different agent, distinct from the plan. This is the "independently re-derived by the reviewer" clause of the statement. Supports the topic. Genuine, distinct.

### Occ 3 — "commit c7ef52a" (`decided`, 2026-07-10) - DROPPED

The artifact is real: c7ef52a is the merge of `85cdc62 feat(core): mkvmerge detection ladder + minimum version floor (D28)`, which lands `pub const MIN_SUPPORTED: (u64, u64) = (86, 0)` with the evidence doc comment. But it is **not an independent occurrence**:

1. **It is the materialization of Occ 1, not a new event.** The plan step (Occ 1) tasked the implementer to "derive the floor and record the evidence in the const doc"; the commit is that same task done, by the same implementer, in the same task cycle. Plan-says-derive-X and commit-lands-derived-X are the two ends of *one* action, not two independent instances. This is the "duplicate of another listed occurrence" drop criterion in substance.

2. **The statement itself enumerates only two derivations.** "derived empirically ... independently re-derived by the reviewer" = implementer (Occ 1) + reviewer (Occ 2). There is no third independent derivation for the commit to represent; it merely records the first.

3. **Decisive tell - selective, outcome-driven counting.** c7ef52a / 85cdc62 is a single multi-pattern merge that landed *three* task-3 patterns together (its message spells out all three): the detection ladder (`core-86`), the version floor (`core-87`), and the platform-candidate paths (`core-88`). The two siblings do **not** count this commit:

   | cluster | occurrences | counts c7ef52a? | count | promoted |
   |---|---|---|---|---|
   | core-86 detection-ladder | memo D28 + task-3 verdict | no | 2 | no |
   | core-88 platform-paths | Plan T3 step 2 + task-3 verdict | no | 2 | no |
   | **core-87 version-floor** | Plan T3 step 1 + task-3 verdict + **commit c7ef52a** | **yes** | **3** | **yes** |

   The identical merge commit is admitted as an occurrence for exactly the one pattern where it flips count 2 -> 3 and clears the promotion bar, and excluded from its two task-siblings landed by that same commit. There is no principled basis for the asymmetry; it is count-padding to reach the threshold, which is precisely what this audit exists to catch.

Dropped as a non-independent duplicate (git materialization) of Occ 1, counted inconsistently relative to sibling patterns.

---

## Result

- Distinct surviving occurrences: **2** (Occ 1 plan step; Occ 2 reviewer re-derivation).
- Threshold for Tier-2 promotion (`docs/decision-ledger.md`): count reaching **3**.
- `verified_count = 2 < 3` -> **REJECTED: demote `core-87` from CONVENTIONS.md (Tier 2) back to the decision-ledger (Tier 1)**, at count 2, alongside its task-3 siblings `core-86` and `core-88`.

This is a count-integrity finding only. The floor value `(86, 0)` and its derivation are correct, cross-checked against the mkvtoolnix source in the const doc, the report, and the reviewer verdict. Nothing here disputes the fact; the fact simply has not recurred across enough independent contexts to earn an always-loaded standing convention. It belongs in Tier 1, where a genuine third occurrence in a later plan would legitimately promote it.
