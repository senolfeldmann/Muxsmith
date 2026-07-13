# Task 4 report: core integration tests (Stream C)

Branch: `plan-5.6-c`, worktree `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-c`.
Commit: `486effe` `refactor(tests): core integration-test idiom cleanup + EmptyPlan donor boundary test`.

Scope respected: only the seven owned files under `crates/muxsmith-core/tests/`
touched (`command.rs`, `command_integration.rs`, `planner_resolution.rs`,
`profile_load.rs`, `suggestions.rs`, `validate_structure.rs`,
`executor_no_hang_live.rs`). No `src/` file, no `prop_*.rs` file touched.
Zero intended behavior change; every pre-existing assertion still asserts
exactly the same expected values (verified per-item below and by the full
gate).

## Items implemented

1. **`command.rs` (10 golden argv sites, lines 48/398/467/565/605/635/664/702/749/798).**
   Collapsed `vec![...].into_iter().map(String::from).collect::<Vec<_>>()` to
   a direct array literal `[...]` at all 10 sites, relying on std's
   `Vec<T>: PartialEq<[U; N]>` (`String: PartialEq<&str>` closes it). Same
   expected values, unchanged.
2. **`command_integration.rs:121`.** Same collapse for the one site; the
   array mixes `&'static str` literals and `&String` (`&output_disp` /
   `&primary_disp` / `&donor_disp`), which LUB-coerce to `&str` in array-literal
   position, so `Vec<String>: PartialEq<[&str; N]>` applies. Removed the
   intermediate `expected` binding; `assert_eq!` now compares `command(plan)`
   against the array directly.
3. **`planner_resolution.rs` fully-qualified paths.** Extended
   `use muxsmith_core::planner::{...}` with `AttachmentPlan, ChapterSource,
   FileReport, PrimaryAttachments, TagFlags, TitleAction` and
   `use muxsmith_core::report::{...}` with `Diagnostic`; collapsed every
   qualified reference to the short name. See "Surfaced deviations" below for
   a line-count reconciliation note (line 24 vs. line 2285).
4. **`planner_resolution.rs:619` (`is_some_and`).** `!d.params.get("detail").unwrap_or(&String::new()).is_empty()` →
   `d.params.get("detail").is_some_and(|s| !s.is_empty())`, matching the
   in-file precedent at (then-)line 741.
5. **New test, seed T6-m1** — `empty_plan_fires_when_only_attachments_and_chapters_resolve`,
   inserted directly after `empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough`.
   Same zero-rule-match profile as the two neighboring `EmptyPlan` tests
   (one `optional: true` audio/de rule matching nothing, default
   `tracks.unmatched: drop`), plus a real on-disk `attachments.rules[0].add`
   donor (`donors/Font.ttf`, mirroring the `donors`-subdirectory fixture
   pattern used by `source_overwrite_protects_attachment_donor_of_render_failed_file`
   / `..._chapters_donor_...`). Asserts: `plan.attachments.add_files` is
   non-empty (the donor genuinely resolved), `plan.chapters ==
   ChapterSource::Keep` (the default, trivially "resolved"), and exactly one
   `Severity::Warning` `DiagCode::EmptyPlan` diagnostic still fires. One-line
   rationale is in the test's own doc comment: attachments/chapters are not
   output tracks per spec 5.2, so `detect_empty_plans` (which only inspects
   `plan.assignments` / `primary_track_ids`, confirmed by reading
   `src/planner.rs`) never sees them.

   **First-run evidence** (isolated run, before touching anything else in the
   file afterward):
   ```
   $ cargo test -p muxsmith-core --test planner_resolution empty_plan_fires_when_only_attachments_and_chapters_resolve -- --nocapture
   running 1 test
   test empty_plan_fires_when_only_attachments_and_chapters_resolve ... ok
   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 69 filtered out
   ```
   Passed on first run, as expected for a boundary pin (no RED phase — this
   documents current, correct behavior against future widening, not a bug
   fix).
6. **`profile_load.rs:131-158`.** Both `tracks_block_parses_and_unmatched_defaults_to_drop`
   and `tracks_unmatched_keep_parses` now use the line 1-5 imports
   (`from_str`, `Format::Yaml`, `KeepDrop::{Drop,Keep}`) instead of the fully
   qualified `muxsmith_core::profile::load::...` / `muxsmith_core::profile::model::...` forms.
7. **`suggestions.rs:194` (yagni).** Deleted the single-caller `no_clobber_batch()`
   wrapper; bound `let files = [("Show.S01E01.mkv", AMBIGUOUS_FOO), ("Show.S01E02.mkv", GUARDED_FOO)];`
   once in `with_rule_match_never_widens_an_existing_substring_constraint`
   and passed `&files` to both of that test's `plan_multi` calls (the
   wrapper's implicit call, now inlined, and the per-suggestion loop's call,
   which previously duplicated the same literal array) — the TC-C pattern at
   (then-)line 874.
8. **`suggestions.rs:591` (yagni).** Deleted `partition_diags`; inlined its
   filter chain at its one caller (`no_single_fix_produces_a_two_group_partition`)
   as a single combined predicate: `batch.batch_diagnostics.iter().filter(|d| d.code == DiagCode::SuggestionPartition && d.params.get("kind").map(String::as_str) == Some("group"))`.
   `overlap_diags` (four callers) left untouched.
9. **`suggestions.rs:24, 591→now inline, 638, 667` (idiom).** Added
   `Diagnostic, Severity` to the `report` import; collapsed the three
   remaining fully-qualified sites (`plan`'s return type at line 24 —
   `muxsmith_core::planner::Batch`, already covered by the existing
   `planner` import; the `Severity::Info` comparison; `overlap_diags`'s
   return type). The fourth original site (`partition_diags`'s own
   `Vec<&muxsmith_core::report::Diagnostic>` signature) disappeared together
   with the function in item 8.
10. **`validate_structure.rs:123` (yagni).** `for (snippet, _section) in [("chapters: discard\n", "chapters"), ("title: wipe\n", "title")]` →
    `for snippet in ["chapters: discard\n", "title: wipe\n"]`, dropping the
    dead tuple element.
11. **`executor_no_hang_live.rs:30` (dup).** Deleted the redundant
    `#[cfg(unix)]` on the file's one test; the file-level `#![cfg(unix)]`
    (line 20) already gates the whole binary.

## Full nine-part gate (run once, clean, before the commit above)

1. `cargo fmt --all --check` — clean (one intermediate run found new
   reflow opportunities the mechanical collapses created — e.g. an
   `assert_eq!` that now fits one line — fixed with `cargo fmt --all`,
   then re-verified clean).
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean.
3. `cargo test --workspace` — clean, all green (every crate's suite passed;
   `muxsmith-core`'s test binaries alone: `command` 15/15,
   `command_integration` 4/4, `planner_resolution` 70/70,
   `profile_load` 8/8, `suggestions` 12/12, `validate_structure` 15/15,
   `executor_no_hang_live` 1/1).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` — clean.
5. `cargo deny check` — `advisories ok, bans ok, licenses ok, sources ok`.
6. `pnpm lint` — clean.
7. `pnpm build` — clean (`vue-tsc --noEmit && vite build`).
8. `pnpm check:i18n` — `ok` (12 pre-existing unused-key warnings, unrelated
   to this task, unchanged by it).
9. `pnpm test:e2e` — 6/6 Playwright specs passed.

## Self-review

- **Completeness:** all 11 content items plus the gate/commit step done;
  diff limited to the seven owned files (`git diff --stat` confirms).
- **Assertions unweakened:** every collapsed golden-argv array carries the
  identical string sequence as before (spot-checked via diff; confirmed by
  the still-passing tests, which compare argv byte-for-byte). The
  `is_some_and` rewrite is logically identical to the prior
  `unwrap_or(&String::new()).is_empty()` (both mean "present and
  non-empty"). No test was deleted, weakened, or had its assertion count
  reduced — only helper-function indirection and qualification changed.
  The new test is additive.
- **Discipline:** unsigned commit (`git log -1 --format=%G?` → `N`),
  explicit `git add` of the seven files (no `-A`/`.`), no push, gate run in
  the foreground throughout (no background+monitor pattern).
- **Pristine test output:** no leftover `dbg!`/stray prints; the executor
  no-hang test's script output is the pre-existing mkvmerge-stub chatter,
  unrelated to this task.

## Surfaced for the house ledger (not silently resolved)

1. **`planner_resolution.rs` fully-qualified-path anchor list vs. its own
   stated count.** The brief's line list (`74-87, 137, 1551, 1570, 1591,
   1614, 1637, 1659, 2043, 2063, 2104, 2212, 2235, 2258`) sums to 19
   occurrences by literal enumeration, but the brief's own prose says "the
   20 fully-qualified paths." A full-file scan found 21 non-import
   occurrences total; excluding line 24 (`muxsmith_core::planner::Batch`
   in `plan_one`'s return-type annotation — already covered by the existing
   bare `Batch` import, a different flavor of redundancy than the rest) and
   including line 2285 (a `PrimaryAttachments::Subset` occurrence in the
   same test cluster as the listed 2212/2235/2258, evidently omitted from
   the list by a transcription gap) is the only combination that reconciles
   to exactly 20. I collapsed both line 24 and line 2285 along with the
   listed set, on the reasoning that the aggregate count is more likely
   correct than an enumerated list's completeness and that both are the
   identical mechanical fix as their siblings — flagging here rather than
   silently deciding, per the "re-verify each anchor" instruction.
2. **`suggestions.rs` item 16's line list mixes two different qualified
   names under one description.** "add Diagnostic, Severity to the report
   import; short names at the four sites [24, 591, 638, 667]" literally
   describes only the `report::{Diagnostic,Severity}` sites (591/638/667 at
   brief-time numbering), but line 24 is actually
   `muxsmith_core::planner::Batch` (already covered by the existing
   `planner` import — no import extension needed for it). Implemented as
   specified (all four sites collapsed); flagged since the item's own prose
   doesn't literally cover line 24's case, though its explicit line list
   unambiguously does.
3. **No new house pattern established.** Both items above are anchor-list
   bookkeeping corrections within an already-mechanical, brief-specified fix
   class (redundant fully-qualified paths where a bare import already
   covers the name) — not a new design decision, convention, or deviation
   from `docs/conventions.yaml` / `docs/process-conventions.yaml`. Nothing
   here rises to a ledger entry beyond this note.
