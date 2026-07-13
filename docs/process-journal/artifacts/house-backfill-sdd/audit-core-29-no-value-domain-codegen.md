# Audit: core-29-no-value-domain-codegen (PROMOTION candidate)

**Verdict: REJECTED** — demote to Tier 1. verified_count = 2 (< 3).

## Cluster claim

> No xtask codegen path for value domains; type/codec_kind domains are curated by
> hand in capability, because the v20 schema types `type` as a plain string and
> only `aac_is_sbr` carries a schema enum - codegen would serve one irrelevant
> field.

kind: restraint. status: settled. claimed count: 3. Promotion to standing
house-knowledge convention.

The steelman in the cluster is: *"Generate value domains from the pinned schema
at build time ... rejected because only one irrelevant field would benefit."*
Keep this steelman in view — it is the pivot of the audit.

## Occurrence-by-occurrence verification

### Occ 1 — `journal Plan 2 (Decisions)` — kind: decided — KEEP

`docs/process-journal.md`, entry "2026-07-09 | Plan 2 written and implemented",
section **Decisions and why**, lines 129-133:

> D2 dropped an xtask codegen path: the v20 identification schema types `type`
> as a plain string (no enum) and only `aac_is_sbr` carries a schema enum, so
> generating value domains for one irrelevant field was an abstraction the scale
> had not earned. `type`/`codec_kind` domains are curated in `capability`
> instead. Grounded in the mkvtoolnix source tree Şenol provided locally.

Verbatim match to the restraint statement, under an explicit "Decisions" heading.
Supports "no-codegen restraint arose here as decided." **Supports.**

### Occ 2 — `memo D2 note` — kind: decided — DROP (misattributed / inverts the artifact)

Ref resolves to `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md`,
section **D2: Value-domain validation**. This memo's D2 note does **not** decide
"no codegen." It decides the **opposite** for the `type` domain — it is the
steelman/rejected alternative, still standing in the artifact:

| Property | Domain source | Checked at |
|---|---|---|
| `type` | **pinned identification schema enum (build-time, via xtask codegen)** | validate |
| `codec_kind` | curated alias table in `capability` | validate |

- Tradeoff line: "the build-time domains come from the pinned schema."
- Spec amendment: "9 (note on **pinned-schema enum extraction**)."

So in the memo only `codec_kind` is curated; `type` is explicitly assigned to
**xtask codegen from the pinned schema**. That is exactly the approach the
restraint later rejected.

Timeline (commit dates, all 2026-07-09) rules out any "the memo also dropped it"
reading:

- `3b71a71` — memo created (proposes xtask codegen for `type`).
- `d4390d7` 00:27 — memo finalized to `Status: FINAL`; the D2 table row and
  tradeoff on `type` are **unchanged** — still xtask codegen. This is the memo's
  last touch (`git log --follow`).
- `7e02f86` 01:01 — curated value domains implemented; codegen dropped (occ 3).
- journal entry (session close) — records the drop as decision D2 (occ 1).

The memo predates the drop and was never amended to reflect it. There is no other
Plan 2 memo carrying a "D2"; the Plan 2 fix pass used decisions #1-#3, not
D-numbered. So "memo D2 note" unambiguously points to the artifact that **adopted**
codegen for `type`.

Counting this artifact as an occurrence of "we decided NOT to codegen" inverts its
content. This is a fabricated recurrence: it recycles the rejected steelman into a
third supporting tally. **Does not support — drop.**

### Occ 3 — `commit 7e02f86` — kind: decided — KEEP

`feat(capability): curated value domains (type, codec_kind) for D2`. Adds
`TYPE_VALUES`, `matchable_domain()`, `CODEC_KIND_NAMES`. Code comment:

> Curated rather than generated: the upstream identification schema (v20) types
> `type` as a plain string with no enum ... Verified against `mkvmerge -J`.

The implementation embodies the restraint and its code comment states the
curated-not-generated rationale. Distinct artifact from occ 1 (code vs. narrative
decision-record); not a duplicate ref. **Supports.**

## Tally

| Occ | Ref | Result |
|---|---|---|
| 1 | journal Plan 2 (Decisions) | KEEP |
| 2 | memo D2 note | DROP — documents the rejected codegen approach, not the restraint |
| 3 | commit 7e02f86 | KEEP |

verified_count = **2** distinct surviving occurrences.

## Verdict

**REJECTED.** Threshold for a standing house-knowledge convention is >= 3 real
recurrences. Only 2 survive (the journal decision-record and the implementing
commit — arguably one decision event on 2026-07-09, recorded in two artifact
kinds). The third tally was manufactured by citing the design memo, which is the
artifact that *adopted* the xtask-codegen path the restraint later overturned —
the exact fabricated-recurrence failure this audit exists to catch.

The underlying restraint is real and correctly captured (the schema truly types
`type` as a plain string; codegen for one `aac_is_sbr` enum is unearned). It is
sound Tier-1 project knowledge. It has not earned promotion to a standing
convention on the strength of its occurrence count. **Demote to Tier 1.**
