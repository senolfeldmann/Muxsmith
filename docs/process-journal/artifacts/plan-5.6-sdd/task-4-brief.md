### Task 4: Core integration tests (Stream C)

**Files:**
- Modify: `crates/muxsmith-core/tests/command.rs`, `tests/command_integration.rs`, `tests/planner_resolution.rs`, `tests/profile_load.rs`, `tests/suggestions.rs`, `tests/validate_structure.rs`, `tests/executor_no_hang_live.rs`

**Interfaces:** none; pure test-side. One NEW test (T6-m1).

- [ ] `tests/command.rs:48` **idiom** - all 10 golden argv sites (48, 398, 467, 565, 605, 635, 664, 702, 749, 798) compare directly: `assert_eq!(muxsmith_core::command::command(&plan), ["--output", ...])` - std's `impl PartialEq<[B; N]> for Vec<A>` makes the `.map(String::from).collect()` tails unnecessary (verified compiling+passing on 1.96.1).
- [ ] `tests/command_integration.rs:121` **idiom** - same conversion boilerplate, one site; mixed `&'static str` / `&String` elements LUB-coerce to `[&str; N]` (verified on 1.96.1).
- [ ] `tests/planner_resolution.rs:24` etc. **idiom** - extend the existing `use muxsmith_core::planner::{...}` / `use muxsmith_core::report::{...}` with the missing names; drop the 20 fully-qualified paths (74-87, 137, 1551, 1570, 1591, 1614, 1637, 1659, 2043, 2063, 2104, 2212, 2235, 2258).
- [ ] `tests/planner_resolution.rs:619` **idiom** - `d.params.get("detail").is_some_and(|s| !s.is_empty())` (line 741 is the in-file precedent).
- [ ] `tests/planner_resolution.rs:~2479` **test (seed T6-m1)** - NEW test `empty_plan_fires_when_only_attachments_and_chapters_resolve` after the keep-mode test: same optional non-matching de-audio rule as the two neighboring EmptyPlan tests, plus an attachments.rules[0].add on-disk donor (donor-fixture pattern from lines 1088/1199); assert donors resolved (plan.attachments.add_files non-empty) AND exactly one warning-severity DiagCode::EmptyPlan; one-line comment: attachments/chapters are not output tracks per spec 5.2, so they do not suppress the warning. Run it first to confirm it passes against current detect_empty_plans (it pins a boundary, not a bug).
- [ ] `tests/profile_load.rs:131-158` **idiom** - the last two tests use the line-1-5 imports (from_str, Format::Yaml, KeepDrop::...); qualified expressions collapse.
- [ ] `tests/suggestions.rs:194` **yagni** - delete single-caller wrapper no_clobber_batch; bind the two-entry file array once (`let files = [("Show.S01E01.mkv", AMBIGUOUS_FOO), ("Show.S01E02.mkv", GUARDED_FOO)];`) and pass to both plan_multi calls (TC-C at line 874 is the in-file pattern).
- [ ] `tests/suggestions.rs:591` **yagni** - inline partition_diags' filter chain at its one caller (line 627): `batch.batch_diagnostics.iter().filter(|d| d.code == DiagCode::SuggestionPartition && d.params.get("kind").map(String::as_str) == Some("group")).collect()`. overlap_diags (four callers) stays.
- [ ] `tests/suggestions.rs:24, :591, :638, :667` **idiom** - add Diagnostic, Severity to the report import; short names at the four sites.
- [ ] `tests/validate_structure.rs:123` **yagni** - `for snippet in ["chapters: discard\n", "title: wipe\n"]` - drop the dead tuple element.
- [ ] `tests/executor_no_hang_live.rs:30` **dup (seed T3-m1 = the tracked cfg(unix) twin, counted once)** - delete line 30's `#[cfg(unix)]`; the file-level `#![cfg(unix)]` (line 20) already gates the binary.
- [ ] Full gate; commit `refactor(tests): core integration-test idiom cleanup + EmptyPlan donor boundary test`.

