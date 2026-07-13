# Audit: proc-07-verify-against-source (PROMOTION candidate)

**Cluster:** `proc-07-verify-against-source` (kind `pattern`, domain `process`)
**Claim:** count 3, promoted at count 3, status `settled`.
**Statement under audit:** Load-bearing tooling and dependency behavior is
confirmed against the crate's actual source or the registry, never from a brief's
claim, a docs sentence, or training-data memory.

**Method:** each cited occurrence opened in its named primary artifact (verdict
file / journal entry) and checked against "this (topic, approach) arose here as
{occ.kind}". Drop if fabricated, misattributed, or a duplicate of another listed
occurrence.

**Verdict: CONFIRMED** - all 3 occurrences survive. Each is a genuinely distinct
recurrence (different date, plan/session, and tooling), each fully supported by a
real primary artifact. The count of 3 is real; promotion stands.

**One systematic metadata defect, non-disqualifying (see "Broken index pointers"
below):** every `EX[NN]` parenthetical in the refs points to the wrong record (or,
for E8, a nonexistent one). But each ref is a *composite* - a named primary
artifact plus the reconstruction-index pointer - and in all three cases the **named
primary artifact** (a verdict file or a journal "what the process caught" section)
supports the occurrence verbatim. The substance is real; only the secondary index
is wrong and should be corrected.

---

## Occurrence 1 - reinforced, 2026-07-09, "task-5-review-verdict.md + journal Task 5 (E4[34])" -> SURVIVES

**Artifact:** `docs/process-journal/artifacts/plan-3.5-sdd/verdicts/task-5-review-verdict.md`
(Plan-3.5 Task 5 spec+quality verdict).

**Verbatim support** (the verdict's compliance table and Strengths):
> Doc-comment correction (`xx-YY` canonicalizes unchanged, not `None`) | Confirmed
> accurate. The **only** `Err` return in `canonicalize()` is
> `MultipleExtendedLanguageSubtags` at `lib.rs:525`; every registry lookup ...
> falls back to `.unwrap_or(original)` when a subtag isn't in the table ... The
> brief's own draft ("well-formed-but-invalid tags ... return `None`") was wrong;
> the correction is warranted, not a deviation.

and

> The `canonical_tag` doc-comment correction is a genuine improvement: it's backed
> by reading the crate's actual control flow ... not an assumption.

The verdict cross-checked the correction against the real crate source at
`~/.cargo/registry/.../language-tags-0.3.2/src/lib.rs`, including the
Suppress-Script table (`LANGUAGES_SUPPRESS_SCRIPT`, lib.rs:551-558, the 134-entry
table). This is a textbook instance of the pattern: a **brief's doc-comment claim**
(`xx-YY -> None`) corrected against the **crate's actual source**. Kind `reinforced`
fits (the pattern was applied and independently re-verified here). Not fabricated,
not a duplicate.

*Index nit:* the parenthetical `E4[34]` is wrong. `find-E4.md` record 34 is the
UnsupportedSource gate predicate; the Task-5 doc-comment correction is record **41**
("Dependency behavior :: verify against the crate's actual source/probes, not the
brief's assumption"), and find-E4.md's own closing note confirms "Task 5's
doc-comment correction (record 41)". Should read `E4[41]`. Does not affect the
outcome - the named verdict file is authoritative and supports the occurrence.

## Occurrence 2 - decided, 2026-07-10, "journal Plan 5 (what the process caught) (E6[49])" -> SURVIVES

**Artifact:** `docs/process-journal.md`, Plan-5 entry, section **"What the process
caught"** (line 389).

**Verbatim support (line 389):**
> eslint pinned to a 2-year-stale 9.9.1 from training-data memory, everything else
> registry-current (T4 review). Origin: implementer habit; led to
> registry-verify-everything discipline.

Directly supports the topic and the `decided` kind (the registry-verify-everything
discipline was born here). This is the **registry / not-training-data-memory** arm
of the statement: a dependency version typed from stale memory instead of resolved
against the registry, with the fix generalized into "resolve every dependency
version against the registry, not memory" (find-E6 record 58). Corroborated by
`plan-5-sdd/verdicts/task-4-review-verdict.md` (Important #1), which find-E6 record
57 cites as the source of the eslint catch.

Genuinely **distinct** from occurrence 1: different date (07-10 vs 07-09), different
plan (5 vs 3.5), different ecosystem (npm registry vs a Rust crate's source), and a
different failure mode (stale version from memory vs a wrong behavior claim in a
brief). Not a duplicate.

*Index nit:* `E6[49]` is wrong. `find-E6.md` occurrence 49 is "Log export/copy from
history - parity gap closed in Plan 5" (gui). The eslint/registry material is
occurrences **57** (eslint stale-from-memory) and **58** (Registry-verify-everything
discipline, whose `occ_ref` is exactly "journal Plan 5 (what the process caught)"
and whose evidence is "led to registry-verify-everything discipline"). Should read
`E6[58]`. The named journal section is authoritative and supports the occurrence.

## Occurrence 3 - reinforced, 2026-07-12, "journal session-8 close (E8[88])" -> SURVIVES

**Artifact:** `docs/process-journal.md`, session-8 close entry, section **"What the
process caught"** (lines 545-548). (The journal dates this entry 2026-07-11; the
cluster lists 2026-07-12 - a one-day attribution difference within the same
session-8-close event, see note below. Immaterial to distinctness.)

**Verbatim support (lines 545-548):**
> - Source-over-docs: the docs sentence on devCsp implied dev injection; crate
>   source disproved it for devUrl setups.
> - The CSP surface scan (Explore agent) derived connect-src 'self', which would
>   have broken IPC; caught against the official docs (ipc: + http://ipc.localhost).

Both halves of the occurrence's evidence are present verbatim. The devCsp half is a
**docs sentence** disproved by reading the **tauri 2.11.5 crate source** (also
recorded in the D34 decision, lines 517-520, and find-E8 occurrence 3). The
connect-src half is a derived assumption checked against the **official docs**
before it could break IPC. Both are the pattern exactly. Kind `reinforced` fits.

Genuinely **distinct** from occurrences 1 and 2: different session (8), different
tooling (Tauri CSP / IPC), different sources consulted (Tauri source + official
Tauri docs). Not a duplicate.

*Index nit (most severe of the three):* `E8[88]` **does not exist** -
`find-E8.md` is numbered continuously 1-53 (max occurrence 53). The devCsp/source-
over-docs content is `find-E8.md` occurrence **3** ("Production CSP form :: a
`devCsp` block ... source-verified vs tauri 2.11.5 ... the block would be dead
config"), with the memo trail in the D34 section. Should read `E8[3]` (plus the D34
memo). The named journal section is authoritative and supports the occurrence.

---

## Broken index pointers (systematic, cross-cutting)

All three `EX[NN]` parentheticals are wrong, and the error is systematic rather than
per-occurrence:

| Occurrence | Cited | Actual record | Note |
|---|---|---|---|
| 1 | `E4[34]` | `E4[41]` | 34 = UnsupportedSource gate; find-E4's own note names record 41 |
| 2 | `E6[49]` | `E6[58]` (precursor 57) | 49 = log-export parity gap (gui) |
| 3 | `E8[88]` | `E8[3]` + D34 memo | E8 has no occurrence 88 (max is 53) |

The `find-EX.md` files each carry **independent** per-file numbering (E1->32,
E4->47, E5->92, E6->73, E7->105, E8->53), so `EX[NN]` is not a global index and the
brackets cannot be resolved by any single offset. This looks like an index written
against a different (earlier or renumbered) state of the find files, not
per-occurrence fabrication - because in every case the **named primary artifact** in
the same ref (`task-5-review-verdict.md`, "journal Plan 5 (what the process caught)",
"journal session-8 close") resolves correctly and supports the occurrence. Recommend
correcting the three brackets to `E4[41]`, `E6[58]`, `E8[3]` so the standing
convention's citations are navigable.

## Why the count is real (the audit's whole point)

The three occurrences are three genuinely independent recurrences across the arc:

| Event | Source consulted | Refuted claim source |
|---|---|---|
| Plan-3.5 T5: `xx-YY` canonicalization | `language-tags` crate source (Suppress-Script table) | a **brief's** doc-comment |
| Plan-5 T4: eslint version currency | npm **registry** | **training-data memory** |
| Session-8: Tauri CSP (devCsp / connect-src) | Tauri crate source + **official docs** | a **docs sentence** / a derived assumption |

Each covers a different arm of the statement (source vs registry vs docs; brief vs
memory vs docs sentence), different tooling, and a different date/plan/session. None
is double-booked (the failure mode `core-42` was rejected for). The evidentiary
basis for promotion - three distinct recurrences - is met.

## Bottom line

verified_count = 3. All three cited occurrences survive as distinct, real
recurrences, each anchored to a primary verdict/journal artifact that supports it
verbatim. 3 >= 3 -> **CONFIRMED**: promotion stands.

**Caveat, not a demotion:** the three `EX[NN]` reconstruction-index pointers are all
wrong (`E4[34]->[41]`, `E6[49]->[58]`, `E8[88]->[3]`, the last nonexistent) and
should be corrected for navigability, but they do not undermine the count because
each ref's named primary artifact is authoritative and holds.
