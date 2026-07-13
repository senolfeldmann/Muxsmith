# Idiomacy review - slice F2a (core planner/suggestions/property tests)

Slice: `crates/muxsmith-core/tests/planner_resolution.rs` (2480 lines), `crates/muxsmith-core/tests/suggestions.rs` (1005 lines), `crates/muxsmith-core/tests/prop_planner.rs` (483 lines). All three read completely at HEAD. Dimensions hunted: idiom, stdlib, yagni, native. Toolchain ground truth: Rust 1.96.1 / edition 2024, proptest =1.11.0, insta =1.48.0 (insta is unused in this slice; that is correct - these are behavioral diag-code assertions, not snapshot material).

Overall verdict: healthy test code. Test structure (one behavior per `#[test]`, decision/spec references in comments, `(batch, _dir)` tempdir-keepalive pattern, `prop_assume!` meaningfulness guards, deliberate insert-only suggestion-application mirror) is idiomatic for the pinned toolchain. Findings are all small polish items; nothing structural.

## Findings

### F2a-1 [idiom] planner_resolution.rs:24 - fully-qualified paths for types whose sibling imports already exist

20 non-`use` occurrences of `muxsmith_core::planner::X` / `muxsmith_core::report::X` in expression/signature position, while the file already imports sibling items from those exact modules (`use muxsmith_core::planner::{AppliedChange, Batch, RunInputs, plan_batch};`, `use muxsmith_core::report::{DiagCode, Severity};`). The inconsistency is sharpest at line 24: `fn plan_one(...) -> (muxsmith_core::planner::Batch, tempfile::TempDir)` spells out `Batch` fully even though `Batch` is in the use list five lines up. Affected names: `Batch` (24), `FileReport`/`Diagnostic` (137), `AttachmentPlan`, `PrimaryAttachments` (74-75, 2212, 2235, 2258, 2285), `ChapterSource` (79, 2043, 2063, 2104), `TagFlags` (82, 1659), `TitleAction` (87, 1551, 1570, 1591, 1614, 1637).

**Replacement:** extend the two existing `use` lines with `AttachmentPlan, ChapterSource, FileReport, PrimaryAttachments, TagFlags, TitleAction` and `Diagnostic`; drop the prefixes at all 20 sites. Roughly eight currently-multiline `assert_eq!` calls collapse to one line each under rustfmt.

- lines_cut: 15, deps_cut: 0

### F2a-2 [idiom] suggestions.rs:24 - same fully-qualified-path inconsistency

`Batch` is imported (line 6) yet written as `muxsmith_core::planner::Batch` in `plan`'s signature (24); `muxsmith_core::report::Diagnostic` (591, 667) and `muxsmith_core::report::Severity::Info` (638) are fully qualified while `DiagCode` from the same module is imported.

**Replacement:** add `Diagnostic, Severity` to the `use muxsmith_core::report::{...}` line, use `Batch` in `plan`'s signature, shorten the four sites.

- lines_cut: 1, deps_cut: 0

### F2a-3 [idiom] prop_planner.rs:144 - `select(TYPE_VALUES.to_vec())` allocates for nothing

`proptest::sample::select` in the pinned 1.11.0 takes `impl Into<Cow<'static, [T]>>` (verified in the vendored source, `proptest-1.11.0/src/sample.rs:156`), and `TYPE_VALUES` is `pub static TYPE_VALUES: &[&str]` (`src/capability/mod.rs:53`), i.e. already `&'static [&str]`, which converts via `Cow::Borrowed`. The `.to_vec()` at lines 144 and 156 is a copy of training-data habit, not the API's idiom. The `select(vec![...])` literal sites are fine - proptest's own docs use that form when no static slice exists.

**Replacement:** `select(TYPE_VALUES)` at both sites.

- lines_cut: 0, deps_cut: 0

### F2a-4 [idiom] prop_planner.rs:42 - mut-map-plus-insert for fixed literal BTreeMaps

Six builder sites construct fixed-key `BTreeMap`s via `let mut map = BTreeMap::new(); map.insert(...); ...` (lines 42, 51, 106, 176, 245, 281). For a fixed literal key set, `BTreeMap::from([(k, v), ...])` is the modern stdlib constructor (stable since 1.56, standard in edition-2024 code) and removes the `mut` binding and the statement-per-entry shape.

**Replacement:** e.g. `exact_one` becomes
```rust
fn exact_one(prop: &str, val: Scalar) -> MatchExpr {
    MatchExpr {
        exact: Some(BTreeMap::from([(prop.to_string(), val)])),
        ..Default::default()
    }
}
```
Same transform at the other five sites (the multi-entry `properties` maps in `video_track`, `arb_track`, `arb_nonvideo_track`, `arb_ambiguous_ident`'s `sub`).

- lines_cut: 8, deps_cut: 0

### F2a-5 [yagni] suggestions.rs:194 - `no_clobber_batch()` single-caller pure-delegate wrapper

`fn no_clobber_batch()` (194-202) only forwards to `plan_multi(P_NO_CLOBBER, &[...])` and has exactly one caller (233). Its supposed value - naming the file set - is undercut by the same test restating the identical two-entry file array inline ten lines later (266-271) for the re-plan.

**Replacement:** delete the wrapper; bind the array once in the test (`let files = [("Show.S01E01.mkv", AMBIGUOUS_FOO), ("Show.S01E02.mkv", GUARDED_FOO)];`) and pass it to both `plan_multi` calls - which also removes the inline restatement. (TC-C at line 874 already uses exactly this `let files = [...]` pattern; this makes the file self-consistent.)

- lines_cut: 5, deps_cut: 0

### F2a-6 [yagni] suggestions.rs:591 - `partition_diags()` single-caller helper; siblings inline the same filter

`fn partition_diags` (591-597) has one caller (627). The neighboring overlap tests check the same `SuggestionPartition` code inline (`batch.batch_diagnostics.iter().any(|d| d.code == DiagCode::SuggestionPartition)` at 806-810 and 989-993), so the helper is not even the file's established idiom for this check. Contrast `overlap_diags` (667), which has four callers and rightly stays.

**Replacement:** inline the filter chain at 627:
```rust
let groups: Vec<_> = batch
    .batch_diagnostics
    .iter()
    .filter(|d| d.code == DiagCode::SuggestionPartition
        && d.params.get("kind").map(String::as_str) == Some("group"))
    .collect();
```

- lines_cut: 5, deps_cut: 0

### F2a-7 [idiom] planner_resolution.rs:619 - `unwrap_or(&String::new()).is_empty()` instead of `is_some_and`

`!d.params.get("detail").unwrap_or(&String::new()).is_empty()` allocates a `String` just to test emptiness and buries the actual predicate ("present and non-empty") under a double negation. The same file already uses the idiomatic form at line 741 (`unsupported.params.get("donor").is_some_and(|d| ...)`).

**Replacement:** `d.params.get("detail").is_some_and(|s| !s.is_empty())`.

- lines_cut: 0, deps_cut: 0

## Routed (out of scope for this hunt, not correctness)

- suggestions.rs:325 - the section-header comment `// --- (b) yaml_fragment must emit valid, round-trippable YAML (bug D) ---` is a verbatim duplicate of line 291 (copy-paste artifact); delete one. Editorial only.

## Explicit non-findings checked

- insta =1.48.0 unused in slice: correct choice; these are targeted diag-code/param assertions, snapshots would be strictly worse.
- `Vec<Strategy>` returned from `prop_flat_map` closures (prop_planner.rs 193, 199, 262): documented proptest behavior (`Strategy` is implemented for `Vec<S>`), idiomatic.
- `use proptest::collection::vec as prop_vec;`: standard rename to avoid shadowing.
- `run_plan` (prop_planner.rs:121) drops its `TempDir` while returning `out_dir` as a bare `PathBuf`: deliberate and safe - planning completes inside the function and callers only do path comparisons (`plan.output.parent()`), never fs access; `diag_sig`'s basename scoping exists precisely because each run gets a fresh throwaway dir.
- `FakeIdentWithExtensions` (planner_resolution.rs:1690): not a delegate-only wrapper - it adds independently controlled `known_extensions`, four consumer tests, justified test double.
- Hand-formatted YAML in `apply_edit_to_first_rule` / `apply_edit_to_no_clobber_rule` vs parsed application in prop_planner's `apply_suggestion`: intentional, each mirrors a different consumer path and says so in comments.
- Fake-mkvmerge helper copies, RECENT_PROFILES_CAP, regex recompilation: known non-findings per the brief, not re-flagged.
