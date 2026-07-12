# Task 5.9 report: locator extension validation (spec §4.6)

## What was implemented

Extended T5's batch-wide `input.extensions` vs. `mkvmerge --list-types`
check to every locator position a profile can declare, per spec §4.6:

- `crates/muxsmith-core/src/planner.rs`: `validate_extension_values` now
  also walks `profile.tracks.rules[i].source` (when `SourceCfg::External`),
  `profile.chapters` (when `ChaptersCfg::External`), and
  `profile.attachments.rules[i].add`. The per-item loop T5 already had was
  extracted into a new `validate_extension_list(known, path_prefix,
  extensions, diags)` helper, called once for `input.extensions` and once
  per locator hit, so the actual comparison logic exists in exactly one
  place. `config_path` values are `tracks[i].source.external.extensions[j]`,
  `chapters.external.extensions[j]`, `attachments.rules[i].add.extensions[j]`
  — the `.extensions[j]` suffix appended onto the same base path strings
  `MissingExternal`/`AmbiguousExternal` already use for these positions
  elsewhere in `planner.rs`, and that `validate_locator`'s three call sites
  in `profile/validate.rs` (the established "every locator position" walk
  in this codebase, used for `EmptyExtensions`/`LocatorConflict`) already
  use. Cross-checked: those three call sites are the only places a
  `Locator` appears in the profile model, so nothing is missed.
- `crates/muxsmith-core/src/report/mod.rs`: `UnknownExtension`'s doc
  comment updated to name all four extensions-list positions instead of
  only `profile.input.extensions`, since the diagnostic now fires for any
  of them.
- `crates/muxsmith-core/src/profile/model.rs:257-259`: no text edit. The
  `Locator.extensions` doc ("validated against `mkvmerge --list-types`
  like `input.extensions`") was already worded correctly — it was written
  aspirationally in an early rustdoc pass (`git log -L` on those lines
  shows it predates both T5 and this task) and was false only because the
  behavior didn't exist yet. It is true now; there was nothing incorrect
  in the text to change.

## Dedup behavior (as requested)

**No dedup by extension value**, in either direction, consistent with what
T5 already did for `input.extensions`: T5 never deduped two identical
strings appearing twice in `input.extensions` (each index got its own
diagnostic at its own `config_path`), and this task keeps that. The same
unknown extension repeated across `input.extensions` and a locator (or
across two locators) produces one `UnknownExtension` warning per
occurrence, each at its own `config_path`. Verified by the new test
`unknown_extension_repeated_across_input_and_locator_is_not_deduped`
(`mp4a` in both `input.extensions[1]` and
`tracks[1].source.external.extensions[0]` → 2 diagnostics, not 1).
Confirmed no downstream dedup exists either (checked `report/json.rs` and
the CLI diagnostic rendering path — plain iteration/chaining, no
dedup-by-code+params anywhere).

## TDD evidence

RED (four new tests failing, two pre-existing extension tests still
passing, confirming the harness/imports/paths were right before touching
implementation):

```
test unknown_extension_is_batch_warning_naming_the_extension ... ok
test unknown_extension_check_degrades_when_runtime_unavailable ... ok
test unknown_extension_repeated_across_input_and_locator_is_not_deduped ... FAILED
test unknown_extension_in_attachments_add_locator_is_batch_warning ... FAILED
test unknown_extension_in_track_rule_locator_is_batch_warning ... FAILED
test unknown_extension_in_chapters_locator_is_batch_warning ... FAILED
test result: FAILED. 2 passed; 4 failed; 0 ignored
```

GREEN (after implementing `validate_extension_list` + the three-site
walk):

```
test unknown_extension_is_batch_warning_naming_the_extension ... ok
test unknown_extension_check_degrades_when_runtime_unavailable ... ok
test unknown_extension_in_attachments_add_locator_is_batch_warning ... ok
test unknown_extension_repeated_across_input_and_locator_is_not_deduped ... ok
test unknown_extension_in_track_rule_locator_is_batch_warning ... ok
test unknown_extension_in_chapters_locator_is_batch_warning ... ok
test result: ok. 6 passed; 0 failed
```

The three positive-case tests each isolate the `UnknownExtension`
diagnostic from the position's own resolution error: the track-rule test
uses `optional: true` (suppresses `MissingExternal` on zero hits); the
chapters test provides a real matching `.xml` donor file (chapters has no
`optional` escape — zero hits is always a hard `MissingExternal` error);
the attachments test needs nothing extra since a zero-hit `add` is already
a warning, not an error (spec 4.9). Each test also asserts
`batch.files[0].plan.is_some()` to confirm the batch genuinely continues
past the warning.

## Files changed

- `crates/muxsmith-core/src/planner.rs` — extended walk + extracted helper
- `crates/muxsmith-core/src/report/mod.rs` — `UnknownExtension` doc comment
- `crates/muxsmith-core/tests/planner_resolution.rs` — 4 new tests

No change to `crates/muxsmith-core/src/profile/model.rs` (see above: the
doc claim there was already accurate text, just previously unfulfilled by
behavior).

## Gate results (foreground, all from `.worktrees/stream-b`)

1. `cargo fmt --all --check` — clean
2. `cargo clippy --workspace --all-targets -- -D warnings` — clean
3. `cargo test --workspace` — all pass (core: 59 planner_resolution tests
   including the 6 extension tests, plus every other crate's suite)
4. `cargo deny check` — advisories ok, bans ok, licenses ok, sources ok
5. `pnpm lint` — clean
6. `pnpm check:i18n` — ok (16 files scanned, 173 catalog ids; 12
   pre-existing unused-key warnings, unrelated to this diff)
7. `pnpm build` — clean (vue-tsc + vite build)
8. `pnpm test:e2e` — 3/3 Playwright smoke tests pass
9. `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace` — fails,
   but only on the pre-existing known failures outside this task's scope:
   `capability/runtime.rs:110` (`platform_candidates` private-link),
   `executor/joblog.rs:124` (`JobAccumulator` private-link),
   `executor/queue.rs:73` (`worker_count` private-link),
   `muxsmith-cli/src/i18n.rs:53,59` (`msg` broken-link). Grepped the full
   doc output for `planner.rs`/`report/mod.rs`: zero hits. This diff adds
   no new doc warnings.

## Self-review

Dispatched an independent read-only review subagent against the diff
(cross-checking locator coverage against `validate.rs`'s
`validate_locator` call sites, `config_path` convention consistency,
dedup-pipeline search, test-quality check by reading
`resolve_file`/`resolve_chapters`/attachment-`add` handling, and rustdoc
quality). Result: all 6 points PASS, no bugs or gaps found. Full findings
folded into this report's earlier sections rather than duplicated here.

## Concerns

None. The change is a mechanical, low-risk extension of an existing,
already-reviewed check (T5) onto three additional call sites that share
one established path-naming precedent already used elsewhere in this
codebase (`validate_locator` in `validate.rs`), with no new abstractions,
no schema/model changes, and no new dependencies.
