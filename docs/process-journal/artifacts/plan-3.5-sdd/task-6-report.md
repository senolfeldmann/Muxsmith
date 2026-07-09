# Task 6 Report: `UnsupportedSource` diagnostic (D21)

## Implemented

1. **`crates/muxsmith-core/src/report.rs`**: added `UnsupportedSource => "unsupported-source"` to the planning-time section of `diag_codes!`, immediately after `UnidentifiableSource`, with the exact doc comment from the brief (distinguishes from `UnidentifiableSource`, notes the zero-track exclusion).
2. **`locales/en/diagnostics.ftl`**: added `unsupported-source = mkvmerge identified this file but its container is not a supported muxing source.` right after `unidentifiable-source` (before `suggestions-capped`).
3. **`crates/muxsmith-core/src/planner.rs`**, in `resolve_file`: inserted the gate after the `UnknownPropertySkew` format-version-skew block and before `let mut assignments = Vec::new();`:
   ```rust
   if !ident.container_recognized || !ident.container_supported {
       diagnostics
           .push(Diagnostic::error(DiagCode::UnsupportedSource, "input").for_file(&primary.path));
       return FileReport {
           source: primary.path.clone(),
           identifier: primary.identifier.whole.clone(),
           plan: None,
           diagnostics,
       };
   }
   ```
   (The `diagnostics.push(...)` line shape differs slightly from the brief's multi-line form purely because `cargo fmt` collapsed it; semantics identical, mirrors the `UnidentifiableSource` early return's `FileReport` field set exactly.)
4. **`crates/muxsmith-core/tests/planner_resolution.rs`**: added three tests after `unidentifiable_donor_yields_unidentifiable_source_not_missing_external`, using the existing `plan_one(profile_yaml, file_name, ident_json)` helper and `P_VIDEO_AUDIO` profile:
   - `unrecognized_container_yields_unsupported_source_not_missing_track` (`recognized: false, supported: true`)
   - `unsupported_container_yields_unsupported_source_not_missing_track` (`recognized: true, supported: false`)
   - `recognized_supported_zero_tracks_stays_missing_track_not_unsupported_source` (`recognized: true, supported: true`, `tracks: []` -> confirms decision #5: NO `UnsupportedSource`, plain `MissingTrack` fires instead)

   Split the OR condition into two separate tests (one per false-branch) rather than one combined test, since the gate's whole point is that `container_recognized` and `container_supported` are independently sufficient; a single test picking one wouldn't demonstrate the `||`.

## JSON field names confirmed

From `Identification::from_json` in `crates/muxsmith-core/src/identify.rs:150-197`, cross-checked against the existing unit test `unrecognized_container_is_not_identifiable` (identify.rs:424-431) and fixture `crates/muxsmith-core/tests/fixtures/identify/series-s01e01.json`:

```json
{ "container": { "recognized": false, "supported": false }, "file_name": "...", "identification_format_version": 20, "tracks": [] }
```

`container.recognized` -> `Identification.container_recognized`, `container.supported` -> `Identification.container_supported`. A missing `tracks` key defaults to empty via `unwrap_or_default()`, so `"tracks": []` is not strictly required but was included for clarity in the new tests. Used `identification_format_version: 20` (matches `PINNED_IDENTIFICATION_FORMAT_VERSION` in `capability/mod.rs:13`) to keep the new tests from also tripping the unrelated `UnknownPropertySkew` warning.

## TDD

- **RED**: ran `cargo test -p muxsmith-core --test planner_resolution unsupported_source` before touching `report.rs`/`planner.rs` -> compile error, `no variant ... named 'UnsupportedSource' found for enum 'DiagCode'` (3 occurrences, one per new test referencing the not-yet-existing variant).
- **GREEN**: after adding the `DiagCode` variant, the Fluent message, and the planner gate -> all 3 new tests pass (`cargo test -p muxsmith-core --test planner_resolution unsupported_source` and `... recognized_supported_zero_tracks`), confirmed individually and together.

## Files changed

- `crates/muxsmith-core/src/report.rs`
- `crates/muxsmith-core/src/planner.rs`
- `locales/en/diagnostics.ftl`
- `crates/muxsmith-core/tests/planner_resolution.rs`

## Gate results (all four, post-implementation)

- `cargo test --workspace`: all green (planner_resolution now 49 tests, up from 46; catalog_completeness, validate_semantics, validate_structure, codegen, doctests all pass; zero failures).
- `cargo fmt --all --check`: failed once on first pass (rustfmt collapsed the `diagnostics.push(...)` call onto two lines differently than the brief's literal snippet); ran `cargo fmt --all` to apply, then `--check` passed clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Self-review

- **Trigger correctness**: gate uses `!ident.container_recognized || !ident.container_supported` exactly as specified, not `is_identifiable()`. Verified by the third test: recognized=true, supported=true, tracks=[] does NOT produce `UnsupportedSource`; it falls through to the per-rule loop, `matched.len() == 0` for both non-optional rules in `P_VIDEO_AUDIO`, producing `MissingTrack` as before. This is the exact decision-#5 distinction the task turns on, and it is now test-covered, not just reasoned about.
- **`FileReport` shape**: matches the `UnidentifiableSource` early return's four fields exactly (`source`, `identifier`, `plan: None`, `diagnostics`); no extra fields, no missing ones.
- **Diagnostic construction**: mirrors `Diagnostic::error(DiagCode::UnidentifiableSource, "input").for_file(&primary.path)`'s pattern (`config_path: "input"`, `.for_file(...)`), with no extraneous `.with(...)` params, matching the brief (no `detail` param needed here since the condition is a plain boolean gate, not an external error string).
- **`#![deny(missing_docs)]`**: the new variant carries a doc comment (`///`), same as every other `diag_codes!` entry; clippy ran clean (no missing-docs warning).
- **Catalog completeness**: `catalog_completeness.rs` passed, confirming the new `DiagCode` has its Fluent message wired.
- **Typography**: ASCII-only in all touched code/text (no dashes, smart quotes, or ellipsis introduced).
- **Placement**: gate sits after the skew check and before `let mut assignments = Vec::new();`, per the brief; does not interfere with the skew warning (both can co-fire on the same file: a container can be simultaneously unrecognized and reported via a newer identification format version; no test exercises that specific combination since it's outside D21's scope, and the two diagnostics are independent pushes to the same `diagnostics` vec).

## Concerns

- None blocking. One minor formatting note already covered above: the brief's literal 3-line `diagnostics.push(...)` snippet reformats to 2 lines under this project's `rustfmt` defaults; ran `cargo fmt --all` rather than hand-fighting the formatter, consistent with "the gate is the source of truth for style, not the brief's inline snippet."
- Did not add a test for the *donor* path (external-source `SourceCfg::External` branch) hitting an unsupported container, since D21 and the plan's self-review scope this gate to the **primary** file only ("donor-side `UnsupportedSource` gate (primary-only per D21)" is explicitly listed as deferred/out of scope). The donor identify-error path already has its own coverage via `UnidentifiableSource` and is unchanged by this task.
- Pre-existing unrelated content in this same report path (`task-6-report.md`, an older "resolve title and tags" report from an earlier plan's Task 6) was overwritten, per the brief's explicit instruction to write the Task 6 report to this exact path; the old content was stale (belonged to a different plan's task numbering) and is presumably already reflected wherever that earlier plan's work landed.
