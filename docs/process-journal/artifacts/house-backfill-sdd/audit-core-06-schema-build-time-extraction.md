# Audit: core-06-schema-build-time-extraction (PROMOTION candidate)

**Cluster:** `core-06-schema-build-time-extraction` (kind: pattern, domain: core, status: settled, promoted: true)
**Statement under audit:** Matchable property names/types are generated into the capability model at build time by an xtask into a committed `generated.rs`; only derived facts ship, the identification schema itself is never redistributed, and there is no `build.rs` network dependency.
**Claimed count:** 4 | **Verified count:** 4 | **Verdict:** CONFIRMED

Every cited ref exists, is a distinct authoritative artifact, and supports the topic arising there as `decided`. Nothing fabricated, nothing misattributed, no wholesale duplicate. Promotion to standing house-knowledge stands. Caveats below do not reach the DROP threshold but should be recorded honestly.

---

## Per-occurrence verification

### Occurrence 1 — spec §2 rows 8-9 + §9 (kind: decided) — SURVIVES
Artifact: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (committed by 61249f9).

- **§2 Decision log, row 8 "Identification schema":** "Build-time data extraction, never redistributed | Property names/types are generated into the capability model at build time; sidesteps schema licensing and runtime fetching entirely." Direct hit on the whole statement (build-time generation, facts-only, not redistributed, no runtime fetch).
- **§9 Capability model and version skew, item 1:** "Build time: matchable property names and types are extracted from the pinned upstream identification output schema into generated Rust code ... The schema file itself is not redistributed; only facts derived from it ship." Direct hit.
- Nuance: the citation says "rows 8-9". Row 8 (Identification schema) is the on-point row; row 9 is "DRY strategy" (one core crate, diagnostics as data), only tangentially related. The load-bearing support is row 8 + §9, both present. Does not affect survival.

### Occurrence 2 — journal Plan 1 (kind: decided) — SURVIVES
Artifact: `docs/process-journal.md`, entry "2026-07-08 | Spec + Plan 1 complete, repo live | session 1", "Decisions and why" bullet (lines 30-32):

> "mkvmerge identification schema: build-time fact extraction via xtask with committed generated.rs; schema never vendored (licensing sidestep), no build.rs network dependency. Runtime skew handled as untyped-match warning."

Near-verbatim match to the full statement, and the **only** ref that literally states the "no build.rs network dependency" clause. Strongest single supporting artifact.

### Occurrence 3 — commits 830dc47 / 4750abb / 61249f9 "(impl set)" (kind: decided) — SURVIVES
Verified the actual code, not just messages:

- **830dc47** `feat(xtask): capability table generator from identification schema` — `crates/xtask/src/gen.rs::generate()` extracts matchable track property names/types from the identification schema JSON; header comment: "Only derived FACTS are emitted; the schema text itself is never redistributed (spec 9)." Emits a "GENERATED FILE - do not edit" `generated.rs`. `main.rs` reads a **local** `schema.json` path and writes the out file via an explicit `cargo run -p xtask -- gen-capability` invocation — no network, not a `build.rs`.
- **4750abb** `feat(core): capability model with generated matchable table and curated settable set` — commits the actual `crates/muxsmith-core/src/capability/generated.rs` ("Generated from identification schema v20") plus `capability/mod.rs`. This is the committed generated artifact the statement describes.

Both genuinely implement the pattern. Survives.

- **Misattribution nuance (recorded, not fatal):** the third commit in the "impl set", **61249f9**, is `docs: add Muxsmith v1 design spec` — it is the spec-doc commit, i.e. the *source artifact of Occurrence 1*, not implementation. Bundling it under "(impl set)" is loose labeling, and it means Occurrence 3 partially overlaps Occurrence 1's artifact. It does **not** trigger DROP: Occurrence 3 retains independent substance from the two real impl commits (830dc47, 4750abb), so it is not a wholesale duplicate of Occurrence 1. The occurrence is a single unit; one mislabeled commit inside a three-commit set does not sink the unit, but the count of "3 commits" overstates the implementation to 2.

### Occurrence 4 — handoff plan-1-close (kind: decided) — SURVIVES
Artifact: `docs/process-journal/artifacts/handoffs/2026-07-09-plan-1-close.md`, "Decisions made (and why)" bullet:

> "Capability model: matchable properties generated at build time from the mkvmerge identification schema (v20) by `crates/xtask` into committed `generated.rs`; schema never vendored. Runtime version skew -> `UnknownPropertySkew` warning, untyped matching."

Direct hit on build-time / xtask / committed generated.rs / never-vendored. Survives.

- Minor: the handoff file is dated 2026-07-09 (written at session close, 2026-07-08T22:01Z per its provenance comment); occurrence date 2026-07-08 tracks the decision/session date. No effect on support.

---

## Corroborating tree facts

- Only `build.rs` in the repo is `src-tauri/build.rs` = standard `tauri_build::build()` scaffold, added in Plan 5 (fe3d2d5) for GUI codegen, unrelated to the schema. Confirms the "no `build.rs` network dependency" clause holds tree-wide: schema extraction is a manual xtask over a local file, never a build script.
- `capability/mod.rs::matchable_type()` reads `generated::MATCHABLE_PROPERTIES` (the committed table); the schema JSON itself is not present in the shipped crate.

## Skeptic's caveat (does not change the verdict)

All four occurrences are the **same-date (2026-07-08), same-session (session 1)** decision recorded in four artifact types (spec / journal / code / handoff), all `kind: decided`. The count of 4 reflects *documentation breadth of one decision*, not four independent recurrences across the project's lifetime. Under the audit's DROP criteria (fabricated / misattributed / duplicate-of-another-ref), each ref is a genuine, independently-verifiable artifact and none is a wholesale duplicate, so all four survive and the >=3 promotion bar is cleared. But if the promotion policy intends "count" to mean *independent recurrences over time* rather than *corroborating records of one settled decision*, this cluster is a single well-documented decision, not a repeatedly-reconfirmed pattern. The statement is nonetheless correct, settled, and load-bearing house-knowledge; promotion is sound on the merits regardless of how the count is read.
