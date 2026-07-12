# Task 16.5 reviewer verdict (model: sonnet, 2026-07-12)

Diff: 2ae62dd..a86eecb, fix ce4fae1 on plan55-t165

## Adjudications
1. Primaries-only CORRECT (not merely defensible): donors identify only
   after their primary succeeded (structural); the schema version is a
   build constant of the one local mkvmerge binary, so donor-newer-while-
   primaries-pinned is impossible; the ruling targeted the granularity
   (per-file noise), not the population. Extending to donors would be
   over-engineering for an unreachable case.
2. config_path "input" reasonable - and better precedented than the
   report claimed (SourceOverwrite is exactly the batch-wide no-field
   class, bucket "output").

## Spec Compliance
✅ own DiagCode SchemaDrift info; params found_version(max)+pinned; once
per batch (single site outside the loop, Option-guarded); raw: hint
bilingual, T21 register verified against the corrections commit; fixture
lockstep (exhaustive macro match); TDD triad with genuine RED. Named risk
(d) confirmed clean BY CONSTRUCTION: simulations reuse the same primaries
+ cached idents, SchemaDrift signature/count byte-identical baseline-vs-
sim, multiset guard cancels.

## Fix wave (ce4fae1, controller-verified: cargo doc clean, 116 lib tests)
Spec-lockstep convention restored: §5.2 catalog row + §9 runtime prose;
identify.rs format_version doc names both consumers.

## Minor (recorded)
"Schema-Version" shorthand mid-message (consistent en+de, cosmetic).

## Assessment
Spec compliance ✅. Task quality: Approved (after the doc-lockstep fix).
