# Task 8 report: resolve attachments (rules + unmatched + external adds)

Note: this file previously held a report for a different, earlier "Task 8"
(semantic validation of match expressions/changes, from a prior plan's
numbering). That content is stale and has been replaced below with the
report for Plan 3 Task 8 (attachment resolution), the task this brief
(`task-8-brief.md`) actually describes.

## What was done

Implemented `resolve_attachments` in `crates/muxsmith-core/src/planner.rs` and wired it into `resolve_file`, replacing the Task-4 default literal `AttachmentPlan { primary: KeepAll, add_files: vec![] }`.

**Existing attachments (select/drop/unmatched):** for each of the primary's attachments (`ident.attachments`, never a donor's, per D10), walks `profile.attachments.rules` in order. The first rule with a matching `select` keeps it, the first with a matching `drop` drops it (`add` rules skipped in this pass); no match falls back to `unmatched` (`Keep`/`Drop`). The kept id set reduces to the most compact `PrimaryAttachments`: `KeepAll` if nothing was filtered, `DropAll` if everything was, `Subset(sorted ids)` otherwise.

**Adds (D12):** for each rule with `add: Some(locator)`, calls `discovery::resolve_locator` and extends `add_files` with *all* hits (a locator is a query, not a unique slot-filler). A rule whose locator matches zero files pushes a **warning** (not error) `Diagnostic` with code `MissingExternal` at `attachments.rules[{i}].add`, `for_file`-tagged to the primary; this never nulls the plan since only `Severity::Error` does that (`finalize_plans`). After all rules run, `add_files` is deduped by path via a `BTreeSet` `retain`, preserving first-seen order.

Also updated the `MissingExternal` doc comment in `crates/muxsmith-core/src/report.rs` from "(track rule or chapters)" to "(track rule, chapters, or attachment add)".

## Files changed

- `crates/muxsmith-core/src/planner.rs`: added `resolve_attachments` (private helper), added `Attachment` to the `crate::identify` import, wired the call into `resolve_file`, replaced the default `AttachmentPlan` literal in the `Plan { .. }` construction.
- `crates/muxsmith-core/src/report.rs`: one doc-comment line on `DiagCode::MissingExternal`.
- `crates/muxsmith-core/tests/planner_resolution.rs`: added `WITH_ATTACHMENTS` fixture const and 7 new tests (below).
- `crates/muxsmith-core/tests/fixtures/identify/with-attachments.json` (new): one video track + three attachments (id0 `a.ttf`, id1 `b.otf`, id2 `cover.jpg`), the fixture the brief's test cases specify.

## TDD

**RED** (`cargo test -p muxsmith-core --test planner_resolution attachment`), before implementation (default stub still `KeepAll`/`[]`):

```
running 7 tests
test attachment_no_rules_and_unmatched_keep_resolves_to_keep_all ... ok
test attachment_select_rule_keeps_matched_and_unmatched_drop_removes_rest ... FAILED
test attachment_add_two_rules_matching_same_file_is_deduped ... FAILED
test attachment_add_locator_attaches_all_matching_files ... FAILED
test attachment_no_rules_and_unmatched_drop_resolves_to_drop_all ... FAILED
test attachment_drop_rule_covers_one_and_unmatched_keep_keeps_the_rest ... FAILED
test attachment_add_locator_zero_matches_yields_missing_external_warning_and_plan_survives ... FAILED

test result: FAILED. 1 passed; 6 failed
```

(The "no rules + unmatched keep" case passed trivially against the Task-4 default, as expected; the other six exercise real behavior and failed as intended.)

**GREEN**, after implementation:

```
running 7 tests
test attachment_no_rules_and_unmatched_drop_resolves_to_drop_all ... ok
test attachment_select_rule_keeps_matched_and_unmatched_drop_removes_rest ... ok
test attachment_drop_rule_covers_one_and_unmatched_keep_keeps_the_rest ... ok
test attachment_add_locator_attaches_all_matching_files ... ok
test attachment_add_locator_zero_matches_yields_missing_external_warning_and_plan_survives ... ok
test attachment_no_rules_and_unmatched_keep_resolves_to_keep_all ... ok
test attachment_add_two_rules_matching_same_file_is_deduped ... ok

test result: ok. 7 passed; 0 failed
```

Full workspace (`cargo test --workspace`): all green, `planner_resolution.rs` at 44 tests (37 pre-existing + 7 new).

## Gate (all before commit)

- `cargo test --workspace`: pass (every test binary green, no FAILED anywhere).
- `cargo fmt --all --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass, zero warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `cargo build --workspace`: pass (confirms no `#![deny(missing_docs)]` violation; `resolve_attachments` is private so it needs no doc comment for the lint, but I gave it one anyway since it explains the D10/D12 reasoning inline).

## Self-review

- Verified the `KeepAll` reduction (`kept.len() == primary_attachments.len()`) correctly falls through to `KeepAll` in the zero-attachments case too (0 == 0), matching the pre-existing Task-4 default test (`plan_and_assignment_carry_resolution_field_defaults`, which uses the empty-attachments `SERIES` fixture and still expects `KeepAll`) - confirmed still passing.
- Checked `Attachment: Matchable` is a direct (non-blanket) impl, so iterating `primary_attachments: &[Attachment]` with a plain `for att in ...` gives `att: &Attachment` and calls `matcher::matches(select, att, lang)` without needing the double-reference blanket impl that `Track` filtering relies on elsewhere in the file.
- Checked `resolve_locator`'s `walk_files` sorts directory entries, so a single locator's hits are already in path order; verified with the "attaches all matching files" test (`a.ttf` before `b.ttf` despite being written to disk in reverse order).
- Confirmed dedup via `BTreeSet::insert` inside `Vec::retain` preserves first-seen order and only strips later duplicates, matching "two rules matching one file attach it once" without reordering.
- Confirmed the `MissingExternal` warning path does not null the plan: `finalize_plans` only drops plans with an `Error`-severity diagnostic, and this one is constructed via `Diagnostic::warning`. Test asserts both `fr.plan.is_some()` and `d.severity == Severity::Warning`.
- Checked typography: no non-ASCII characters anywhere in the diff or the new fixture file (`grep -P '[^\x00-\x7F]'` clean).
- Confirmed no other code path in the repo assumes a fixed severity for `DiagCode::MissingExternal` (grepped all call sites); severity is chosen per-callsite via `Diagnostic::error`/`::warning`, consistent with existing precedent (`OutputCollision` already varies its severity by policy).
- Did not touch chapters, tags, title, tracks, or collision/overwrite logic; diff is scoped to attachments plus the one doc-comment line the brief named.

## Concerns

None. Scope stayed inside attachments; the zero-match add case is a warning as required and does not suppress the plan. Flagging the stale-report mixup above only as an FYI to whoever runs Plan 3's next task - the `.superpowers/sdd/` numbering appears to have been reused across two different plans, so filenames alone are not a reliable cross-plan reference.
