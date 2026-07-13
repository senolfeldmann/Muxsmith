# verify-17: FQ-path inconsistency in tests/suggestions.rs

**Finding:** `crates/muxsmith-core/tests/suggestions.rs` — `Batch` imported (L6) but written `muxsmith_core::planner::Batch` in `plan`'s signature (L24); `muxsmith_core::report::Diagnostic` (L591, L667) and `muxsmith_core::report::Severity::Info` (L638) fully qualified while `DiagCode` from the same module is imported (L9). Tag: idiom, slice F2a.

**Verdict: CONFIRMED**

## (a) Code says what the finding claims — yes, at all four sites

- L6: `use muxsmith_core::planner::{Batch, RunInputs, StructuredEdit, plan_batch};`
- L9: `use muxsmith_core::report::DiagCode;`
- L24: `fn plan(profile_yaml: &str) -> (muxsmith_core::planner::Batch, tempfile::TempDir) {`
- L591: `fn partition_diags(batch: &Batch) -> Vec<&muxsmith_core::report::Diagnostic> {`
- L638: `&& d.severity == muxsmith_core::report::Severity::Info),);`
- L667: `fn overlap_diags(batch: &Batch) -> Vec<&muxsmith_core::report::Diagnostic> {`

The inconsistency is even sharper than stated: L591/L667 use short `Batch` and the full crate path for `Diagnostic` **in the same signature**, and L24 fully qualifies a type the file already imports and uses short elsewhere.

## (b) Replacement is current idiom — yes

Checked via context7 (`/rust-lang/rust`, rust-analyzer style guide + rustc-dev-guide import conventions), not training memory. Convention: frequently used types are imported directly into the local namespace; where a prefix is kept for clarity it is the **module** prefix (`report::Diagnostic`), not the full crate path, and never a mixed state where one sibling (`DiagCode`) is imported and another (`Diagnostic`, `Severity`) from the same module is fully qualified. The proposed fix (`use muxsmith_core::report::{DiagCode, Diagnostic, Severity};` + short names at the four sites) matches what the file already does for `DiagCode` and `Batch`. Edition 2024 changes nothing here.

Viability: no name conflicts — `Diagnostic` and `Severity` appear nowhere else in the file, and the `support` module defines neither.

## (c) Duplication with load-bearing difference — n/a

Not a duplication finding.

## (d) yagni completeness — n/a

Tag is `idiom`, not `yagni`; concrete construct and replacement are named regardless.

## Decision guard — no conflict, not separately tracked

- Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `suggestions.rs`, "fully qualified", "qualified path": no hits.
- ROADMAP "Whole-codebase idiomacy review" is the initiative this finding is produced **under** (slice F2a), not a separate tracker for it; its NAMED INPUTS list does not contain this item.
- ROADMAP group K (cosmetic cleanup) enumerates other files (load.rs, model.rs, command_integration.rs, planner.rs); suggestions.rs import style is not among them.
- Test-hygiene collection (B1-B11): no coverage of this construct.

## Notes

`lines_cut: 1` is plausible (the L637-638 wrap likely collapses once `Severity::Info` is short); either way line count is not a refutation ground.
