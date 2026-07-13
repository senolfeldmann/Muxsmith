# Verify-16: fully-qualified paths in planner_resolution.rs (slice F2a)

**Verdict: CONFIRMED**

## Finding under test

`crates/muxsmith-core/tests/planner_resolution.rs` uses 20 fully-qualified `muxsmith_core::planner::X` / `muxsmith_core::report::X` paths in signatures and assert bodies while sibling items from the same modules are already imported; replacement: extend the existing `use` lines and drop the prefixes.

## (a) Code says what the finding claims - verified at HEAD (2f17880)

- Line 5 imports `use muxsmith_core::planner::{AppliedChange, Batch, RunInputs, plan_batch};`, line 9 imports `use muxsmith_core::report::{DiagCode, Severity};`.
- Line 24 nevertheless spells `muxsmith_core::planner::Batch` in `plan_one`'s return type - with `Batch` already in scope. Stronger than the finding states: the file uses bare `Batch` at lines 221, 1397, 1470, 1710, so the style is inconsistent even within the file itself.
- All cited sites verified by grep: 24, 74, 75, 79, 82, 87, 137 (two paths: `FileReport` + `Diagnostic`), 1551, 1570, 1591, 1614, 1637, 1659, 2043, 2063, 2104, 2212, 2235, 2258, 2285. That is 20 lines / 21 path occurrences; the count discrepancy is line 137 carrying two, immaterial.
- All seven un-imported names exist publicly at the claimed paths: `AttachmentPlan`, `PrimaryAttachments`, `ChapterSource`, `TagFlags`, `TitleAction`, `FileReport` in `src/planner.rs`; `Diagnostic` in `src/report/mod.rs`.
- No name conflicts blocking the import: no bare-name shadowing of any of the seven in the test file; `tests/support/mod.rs` only defines `FakeIdent`. Bare-name grep hits are comments/strings only.

## (b) Replacement is current idiom - verified against official docs (context7, /rust-lang/book, current main)

Rust Book ch. 7.4 ("Bringing Paths into Scope with the use Keyword", "Creating Idiomatic use Paths"): "for structs, enums, and other items, it is standard convention to specify the full path to the item" in the `use` declaration and use the bare name at usage sites; module-qualifying at call sites is the idiom for *functions*, not types. Edition 2024 changes nothing here. The book's stated exception (same-name items from different modules) does not apply - checked above.

Collapse claim spot-checked: no `rustfmt.toml` in the repo, so default `max_width = 100` applies; e.g. lines 1589-1591 become `assert_eq!(plan.title, TitleAction::Set("Show S03".into()));` (~60 cols) and 2210-2212 become `assert_eq!(plan.attachments.primary, PrimaryAttachments::Subset(vec![1]));` (~78 cols) - both single-line. "~8 multiline asserts collapse" is plausible as an approximation (the struct-literal asserts at 72-78/80-86/1657-1663 may stay multiline; the estimate is not load-bearing).

## (c) Duplication with load-bearing difference - n/a

Not a duplication finding.

## (d) yagni without concrete construct - n/a

Tag is `idiom`; concrete construct (the 20 qualified sites) and concrete replacement (extend the two `use` lines, drop prefixes) are both named.

## Decision guard - no hit

- `docs/superpowers/specs/*.md` (D1-D35), `docs/IDEAS.md`, `docs/ROADMAP.md` grepped for the file and for import-style/fully-qualified-path constructs: no match.
- ROADMAP "Cosmetic cleanup, one pass (sweep group K)" enumerates its items (dead `at` param, TracksCfg placement, stale module doc, etc.) - import style in `planner_resolution.rs` is not among them.
- ROADMAP "Test-hygiene collection (docs-tree B-minors)" enumerates B1-B13 - none concern import style.
- Plan docs mention `planner_resolution.rs` only for the `std::mem::forget`/TempDir fix (Plan 4) and test extensions (Plan 3), unrelated.

Neither DECISION_CONFLICT nor TRACKED; the finding stands as an untracked, correct idiomacy cleanup.
