# Task 6 report: Zero-track plan warning (#6)

Worktree: `.worktrees/stream-b`, branch `plan55-stream-b`.
Commit: `c09875e feat(planner): warn when a plan resolves to zero tracks (#6)`.

## What was implemented

A rendered plan (spec 5.1: no error-severity diagnostic for the file) whose
track assignments are all unmatched now raises a new warning-severity
`DiagCode::EmptyPlan`. Previously this case (a valid profile, e.g. one whose
only rules are `optional` and the file simply has nothing to offer them)
muxed a valid-but-empty MKV with exit 0 and no diagnostic at all - the
ROADMAP-tracked pre-1.0 gate this task closes.

`resolve_file` (`planner.rs`) now computes, right before building `Plan`
(after all per-rule assignments, chapters, attachments, and output-render
diagnostics are known):

```rust
let has_tracks = assignments.iter().any(|a| a.track_id.is_some())
    || (keep_unmatched && !ident.tracks.is_empty());
```

and, only when the plan will actually render (`plan.is_some()`) and no
error-severity diagnostic already dooms it to `None` in `finalize_plans`,
pushes `Diagnostic::warning(DiagCode::EmptyPlan, "tracks").for_file(...)`.

### D20 semantics (binding design note from the brief)

Under `tracks.unmatched: keep`, the primary's own tracks always pass
through untouched regardless of rule matches - D20's own reasoning is
"keep = match to what is already there," i.e. a kept-but-unmatched primary
track already counts as *matched*. Consequently `has_tracks` treats a
non-empty `keep`-mode primary (`ident.tracks` non-empty) as a track
presence in its own right, independent of whether any rule fired. This
means `EmptyPlan` cannot fire on a `keep`-mode plan whenever the primary
itself carries at least one track (the only way it *could* fire under
`keep` is a primary with literally zero tracks, which is a degenerate case
outside this task's scope - a zero-track, recognized+supported container
stays `MissingTrack` per D21, an existing separate diagnostic).
`empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough`
(`planner_resolution.rs`) locks this in with a comment tying it back to
D20, using the *same* zero-rule-match profile as the warning test, with
only `unmatched: keep` added, to isolate the one variable that flips the
outcome.

### Guard against redundant noise on already-errored files

`resolve_file` builds `Plan` (locally `Some`) even for a file that already
carries a per-rule error (`MissingTrack`, `AmbiguousRule`,
`OverlappingRules`, `MissingExternal`, `AmbiguousExternal`,
`UnidentifiableSource` on a donor, ...); `plan_core`'s later
`finalize_plans` pass is what actually nulls `f.plan` for any file with an
error-severity diagnostic. Checking `plan.is_some()` alone at the point
`resolve_file` returns is therefore not sufficient to mean "will render" -
so the push is additionally gated on
`!diagnostics.iter().any(|d| d.severity == Severity::Error)`, evaluated
after every local diagnostic source (rule loop, output render, chapters,
attachments) has already contributed. Without this guard,
`missing_track_when_no_match_and_not_optional`-shaped cases (a single
non-optional rule matching nothing) would get a spurious `EmptyPlan`
warning attached to a file whose plan is about to be dropped entirely -
true but useless information, and the existing test suite's `.any(code ==
...)`-style assertions wouldn't have caught it.

## TDD evidence

RED (`DiagCode::EmptyPlan` + Fluent message added as scaffolding; planner
logic not yet wired):

```
$ cargo test -p muxsmith-core --test planner_resolution empty_plan -- --nocapture
running 2 tests
thread 'empty_plan_warns_when_all_optional_rules_match_nothing' panicked at
crates/muxsmith-core/tests/planner_resolution.rs:1609:5:
assertion `left == right` failed: diags: []
  left: 0
 right: 1
test empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough ... ok
test empty_plan_warns_when_all_optional_rules_match_nothing ... FAILED
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 52 filtered out
```

(The keep-mode test trivially passes either way pre-implementation, since
it asserts *absence* of the diagnostic; the positive test is the one that
proves the warning is wired.)

GREEN (implementation added):

```
$ cargo test -p muxsmith-core --test planner_resolution empty_plan -- --nocapture
running 2 tests
test empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough ... ok
test empty_plan_warns_when_all_optional_rules_match_nothing ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out
```

Full `planner_resolution` suite: 54 passed, 0 failed (no regressions from
the guard's interaction with existing `MissingTrack`/`AmbiguousRule`-shaped
tests). `catalog_completeness` re-checked explicitly: passes (proves the
new `empty-plan` Fluent key exists and is wired).

Two tests added to `crates/muxsmith-core/tests/planner_resolution.rs`,
appended after the last attachment test:

- `empty_plan_warns_when_all_optional_rules_match_nothing`: brief's Step 1
  scenario (a single `optional` rule matching nothing against the `SERIES`
  fixture); asserts the plan still renders unchanged (one assignment,
  `track_id: None`) plus exactly one warning-severity `EmptyPlan` attached
  to the file.
- `empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough`: same
  profile plus `tracks.unmatched: keep`; asserts `plan.keep_unmatched`,
  non-empty `plan.primary_track_ids`, and no `EmptyPlan` diagnostic - the
  D20 case the brief asked to be noted in the test.

## SI-3 parity note (for the memo)

**mkvtoolnix-gui has no general zero-selected-tracks warning at all.**
Checked once, live-source, at `~/Downloads/mkvtoolnix`:
`Tab::checkIfMissingAudioTrackIsOK`
(`src/mkvtoolnix-gui/merge/tab.cpp:489-522`) is the closest thing that
exists, and it is narrower on three axes: audio-only (never checks
video/subtitle tracks), opt-out via a setting defaulting to
`IfAudioTrackPresent` (`settings.cpp:593`, so it stays silent unless the
source actually contains an audio track that got deselected), and a
"continue anyway" confirmation dialog rather than a report-level
diagnostic. `Tab::ensureAtLeastOneTrackEnabledMaybe`
(`tab.cpp:523-563`) only auto-sets the container enabled-flag
(`--track-enabled-flag`) on tracks *already* selected for muxing and never
blocks; `Tab::isReadyForMerging` (`tab.cpp:419-441`) checks only the
destination filename. `Tab::addToJobQueue` (`tab.cpp:565-616`) calls all
three but none of them inspect "how many tracks total are selected across
every file" - so a video-only or subtitle-only source with every track
manually deselected sails straight through to job creation with zero
feedback, and mkvmerge then exits 0 and writes the track-less file exactly
as it does from the CLI (already verified live in Plan 3). **Divergence,
not parity**: Muxsmith's unconditional per-file `EmptyPlan` warning is
strictly broader than mkvtoolnix-gui's narrow, opt-out, audio-only
heads-up, and fires as a first-class report entry rather than a modal the
user must click through.

## Files changed

- `crates/muxsmith-core/src/planner.rs`: `resolve_file` computes
  `has_tracks` and pushes `EmptyPlan` under the D20/error-guard conditions
  above; the pre-existing `keep_unmatched` computation was hoisted into a
  local `let` (previously inline in the `Plan` struct literal) since both
  the new check and the struct field now need it.
- `crates/muxsmith-core/src/report/mod.rs`: new `DiagCode::EmptyPlan`
  ("empty-plan"), placed after `UnsupportedSource` (the last per-file
  primary-source-shape code before the cross-file collision codes begin).
- `locales/en/diagnostics.ftl`: `empty-plan` message.
- `crates/muxsmith-core/tests/planner_resolution.rs`: two new tests, as
  above.

No CLI/GUI production files touched. Batch-report visibility (the brief's
"Interfaces" line) needed no new plumbing: `print_batch_human`
(`crates/muxsmith-cli/src/commands/mod.rs`) already iterates every
`FileReport.diagnostics` unconditionally, `batch_document`
(`crates/muxsmith-core/src/report/json.rs`) already serializes the same
vector into each file's JSON `diagnostics` array, and `diag_exit_code`
already folds every diagnostic's severity (via `all_diags`) into the
dry-run/run exit code - so pushing into the existing `diagnostics: Vec`
was sufficient to satisfy "appears in per-file diagnostics AND is visible
in the batch report/summary counts" by construction, matching how every
other existing warning (`DonorIsPrimary`, `UnknownExtension`,
`DuplicateIdentifier`, ...) already flows. Verified there is no separate
diagnostic-severity aggregate for dry-run/run to also update: `validate`'s
`validate-summary` counter line is config-time-only (never reaches
planning, so it cannot see this code); `run`'s `run-summary`/`summary`
JSON block counts `JobState` (mkvmerge's own exit code per job), an
orthogonal, execution-time concept this warning correctly leaves alone.

## Gate results (from worktree root, all green)

- `cargo fmt --all --check`: clean (after one `cargo fmt --all` to apply
  formatting the initial edit didn't match - line-wrap on the `if` guard).
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo test --workspace`: all passing (core 54/54 in
  `planner_resolution` plus every other suite; 0 failures anywhere).
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `pnpm lint`: clean.
- `pnpm check:i18n`: `ok (16 source files scanned, 173 catalog ids, 12
  unused warning(s))` - the 12 warnings are the same pre-existing
  `gui-*.ftl` keys Task 5 already noted as unrelated; catalog id count is
  +1 for the new `empty-plan` key.
- `pnpm build`: `vue-tsc --noEmit && vite build` clean.
- `pnpm test:e2e`: `3 passed` (Playwright smoke + a11y + i18n-completeness
  harness).
- `cargo test -p muxsmith-cli --test catalog_completeness`: explicitly
  re-checked, passes.

`node_modules` already present in the worktree (from Task 5); no
`pnpm install` needed this time.

## Self-review findings

1. **The error-severity guard was not explicit in the brief and is the one
   design decision I made unilaterally.** The brief says the warning
   "fires when a rendered plan has zero track assignments," which I read
   as requiring the plan to actually survive into the final `Batch`, not
   merely be `Some` at the point `resolve_file` returns (the two differ
   because `finalize_plans` runs later, cross-file). Verified this reading
   against the existing `missing_track_when_no_match_and_not_optional` and
   `ambiguous_rule_when_two_tracks_match` tests: without the guard, both
   would gain a spurious second diagnostic on a file whose plan is `None`
   in the final batch. Confident this is correct, not just defensible.
2. **Cross-file drops (`SourceOverwrite`, `OutputCollision`-as-error) are
   deliberately NOT guarded against.** Those run after `resolve_file`
   returns, in separate batch-wide passes in `plan_core`, so a file that
   locally resolves to zero tracks and *also* later gets its plan dropped
   for an unrelated collision reason will carry both an `EmptyPlan`
   warning and the collision error. Judged consistent with how the
   existing diagnostics model already behaves (any warning can coexist
   with a later, unrelated error on the same file; nothing today special-
   cases this), and the collision error already dominates the exit code,
   so the warning is inert noise at worst, not a wrong signal. Flagging
   for awareness rather than treating as a defect.
3. Confirmed no existing test asserts an exact `diagnostics.len()` or
   equality on the full diagnostics vector anywhere in the core test
   suite (`grep` came back empty), so there was no risk of silently
   breaking an unrelated test via the new diagnostic's presence.
4. Placed the new `DiagCode` variant after `UnsupportedSource` rather than
   at the end of the enum (mirroring where Task 5 placed
   `UnknownExtension`, next to its thematic neighbor) - it is the last
   code in the "primary source shape" cluster before the cross-file
   collision codes begin, which is where a "the plan as a whole came out
   empty" check conceptually belongs.

## Concerns

- C1 (T10's exhaustive param-fixture guard) will need a fixture entry for
  `DiagCode::EmptyPlan` (no params beyond the implicit `file`); not
  addressed here per the ledger (T10's job at merge time), consistent with
  how Task 5 left `UnknownExtension`'s fixture for the same reason.
- None outstanding beyond the two self-review items above, both already
  judged and neither blocking.

## Fix wave (post-finalize relocation + batch-report test)

Addresses two Important findings from the T6 review, on top of `c09875e`
(unchanged). Commit: `a60e9a0 fix(planner): EmptyPlan decided post-finalize,
batch-report regression test (T6 review)`.

### Finding 1: relocated the EmptyPlan check

Self-review item 2 above ("cross-file drops are deliberately NOT guarded
against") turned out to be the actual defect, not just a flagged tradeoff:
`resolve_file`'s ad-hoc `!diagnostics.iter().any(|d| d.severity ==
Severity::Error)` guard only ever saw *local* diagnostics, so a file that
resolved to zero tracks locally and then lost its plan entirely to a
cross-file pass (`detect_source_overwrites` or `detect_output_collisions`,
both of which run after `resolve_file`) kept a stale `EmptyPlan` warning
whose "mkvmerge will still write a valid but track-less MKV" framing was by
then false - the file produces no output at all.

Moved:

- `has_tracks` computation and the `EmptyPlan` push, both removed from
  `resolve_file` entirely (`planner.rs`). The local `has_tracks` `let` and
  the `if plan.is_some() && !has_tracks && !diagnostics.iter().any(...
  Severity::Error)` block are gone; `resolve_file` no longer scans its own
  diagnostics for this purpose at all.
- New `plan_core`-private pass `detect_empty_plans(&mut [FileReport])`,
  called last, after both `finalize_plans` calls:
  `detect_source_overwrites` -> `finalize_plans` -> `detect_output_collisions`
  -> `finalize_plans` -> `detect_empty_plans`. It iterates `files`, skips
  any `f.plan.is_none()`, and recomputes has-tracks straight from the
  surviving `Plan`'s public fields:
  `plan.assignments.iter().any(|a| a.track_id.is_some()) ||
  (plan.keep_unmatched && !plan.primary_track_ids.is_empty())` - the same
  D20 semantics as before, just read off `Plan` instead of local
  `resolve_file` state.

Removed guard: the ad-hoc `Severity::Error` scan is gone outright, not
relocated. `f.plan.is_some()` post both finalize passes already means "no
error-severity diagnostic, local or cross-file, doomed this file" (that is
exactly what `finalize_plans` enforces), so the separate scan was redundant
once the check moved past both finalize calls.

Reworded the `EmptyPlan` doc comment in `report/mod.rs`: "no error-severity
diagnostic on the file" -> "survived every finalize pass (no error-severity
diagnostic on the file, local or cross-file)", so the parenthetical is true
for the diagnostic's full lifetime, matching the new mechanism rather than
the old resolve_file-local one.

Both existing `planner_resolution.rs` tests
(`empty_plan_warns_when_all_optional_rules_match_nothing`,
`empty_plan_does_not_fire_under_keep_unmatched_primary_passthrough`) needed
**no changes at all**, assertions or plumbing: both go through `plan_batch`
-> `plan_core`, assert via `.filter`/`.any` over the file's final
`diagnostics` vec, and are indifferent to which pass within `plan_core`
appended the diagnostic.

### Finding 2: batch-report regression test

Added `dry_run_json_surfaces_empty_plan_batch_report` to
`crates/muxsmith-cli/tests/dry_run_cli.rs`, alongside the other `--json`
diagnostics tests (`dry_run_json_diagnostics_all_carry_rendered_text`,
`dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing`). Builds a
real one-track MKV via mkvmerge (`tone.wav`, tagged `--language 0:eng`),
writes a profile with a single `optional` rule requiring `language: de`
(grammatically valid so it never trips the unrelated `InvalidPropertyValue`
batch diagnostic; never matches the fixture's `eng` track), runs `dry-run
--json`, and asserts:

- exit code is exactly `1` (worst diagnostic present is a warning, not an
  error - dry-run's exit mirrors mkvmerge: 0/1/2),
- `files` has exactly one entry with a non-null `plan` (the plan renders,
  just empty),
- `files[0].diagnostics` contains an entry with `code == "empty-plan"`.

TDD: written and run before checking in; passed on the first run (`cargo
test -p muxsmith-cli --test dry_run_cli
dry_run_json_surfaces_empty_plan_batch_report`, `1 passed`), since the
underlying feature (`c09875e`) already exists and this test is exercising
existing end-to-end behavior the review found lacked coverage, not driving
new production code.

### Gate results (from worktree root, all green)

- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo test --workspace`: all passing, no failures anywhere (core +
  cli + gui + xtask suites); `dry_run_cli` 10/10 including the new test.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- `pnpm lint`: clean.
- `pnpm build`: `vue-tsc --noEmit && vite build` clean.
- `pnpm check:i18n`: `ok (16 source files scanned, 173 catalog ids, 12
  unused warning(s))` - same pre-existing warnings as before, unrelated.
- `pnpm test:e2e`: `3 passed`.

`node_modules` already present; no `pnpm install` needed.
