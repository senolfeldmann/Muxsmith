# Seed [T6-m1] — attachments/chapters-only EmptyPlan test missing

**Verdict: CONFIRMED** (at HEAD 2f17880)

## Original finding

Task-6 verdict (`docs/process-journal/artifacts/plan-5.5-sdd/task-6-verdict.md`, "Issues (residual)"): "no dedicated attachment/chapters-only zero-track test (predicate provably ignores them; documentation-value only)". Deferred in `whole-branch-verdict.md` line 86 to the idiomacy review / Plan 6 test rider.

## State at HEAD

The `EmptyPlan` predicate (`crates/muxsmith-core/src/planner.rs:1240-1241`, `detect_empty_plans`) computes `has_tracks` from `plan.assignments[*].track_id` and the `keep_unmatched && !primary_track_ids.is_empty()` passthrough only. `plan.attachments` / `plan.chapters` never enter the predicate, so a plan whose only resolved content is attachment or chapters donors still fires the warning — intended per spec 5.2 ("zero output tracks"; attachments/chapters are not tracks), but nothing pins it.

Existing EmptyPlan coverage, none of it this case:

- `empty_plan_warns_when_all_optional_rules_match_nothing` — `crates/muxsmith-core/tests/planner_resolution.rs:2418` (bare zero-match)
- `empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough` — `planner_resolution.rs:2461` (D20 keep path)
- `dry_run_json_surfaces_empty_plan_batch_report` — `crates/muxsmith-cli/tests/dry_run_cli.rs:702` (end-to-end `--json` surfacing)
- suggestions regression-rejection touchpoint — `crates/muxsmith-core/tests/suggestions.rs:951`

No test combines a zero-track-resolution plan with resolved attachments and/or chapters donors. The seed still applies.

## Replacement

Add one test in `planner_resolution.rs`, directly after `empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough` (after line 2479), e.g. `empty_plan_fires_when_only_attachments_and_chapters_resolve`:

- Profile: the same optional non-matching `de` audio rule as the two neighbors, plus an `attachments.rules[0].add` locator (on-disk donor file in the tempdir, pattern of the existing donor-fixture tests around lines 1088/1199) and/or a `chapters.external` `.xml` donor.
- Assert: `plan.attachments.add_files` (and/or `plan.chapters != ChapterSource::Keep`) non-empty — proving donors resolved — AND exactly one `DiagCode::EmptyPlan` warning fires.
- One-line comment tying it to the predicate: attachments/chapters are deliberately not "output tracks" (spec 5.2), so they do not suppress the warning.

Documentation-value only (the predicate provably ignores those fields today); the test pins the boundary against a future refactor of `detect_empty_plans` silently widening `has_tracks`.

## Estimates

- lines_cut: -45 (test addition, net new lines including donor-file setup and comment)
- deps_cut: 0

Tag: `test`
