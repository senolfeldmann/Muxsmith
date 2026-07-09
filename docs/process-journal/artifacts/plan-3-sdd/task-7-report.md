# Task 7 report: resolve chapters (keep / drop / external locator)

## What was done

Added `resolve_chapters` to `crates/muxsmith-core/src/planner.rs` and wired it
into `resolve_file`'s `Plan { .. }` construction, replacing the
`ChapterSource::Keep` literal left by Task 4.

`resolve_chapters(profile, primary, primary_dir, diags) -> ChapterSource`:

- `ChaptersCfg::Keyword("drop")` -> `Drop`.
- `ChaptersCfg::Keyword(_)` (i.e. `"keep"`, and defensively any other value,
  since `validate.rs` already rejects anything but `keep`/`drop` at config
  time) -> `Keep`.
- `ChaptersCfg::External(block)`: `discovery::resolve_locator(&block.external,
  primary_dir, &primary.identifier)`, then on `hits.len()`:
  - `1` -> `External(path)`.
  - `0` -> push `Diagnostic::error(MissingExternal, "chapters.external").for_file(&primary.path)`,
    return `Keep` as an unused placeholder (the error already forces
    `plan: None` in the existing `finalize_plans` pass).
  - `n` -> push `Diagnostic::error(AmbiguousExternal, "chapters.external").for_file(&primary.path).with("count", n.to_string())`,
    return `Keep` as the same placeholder.

No changes to `DiagCode`, `report.rs`, or `validate.rs`: the doc comments for
`MissingExternal`/`AmbiguousExternal` already say "track rule or chapters"
(from Plan 2), and `validate.rs` already fully validates the `chapters` key
(keyword shape and the external locator) at config time -- confirmed by
reading `validate::validate` lines ~123-137 before starting.

A chapters file is never passed through `Identify`: only the locator is
resolved, matching the brief's explicit note that a chapters file is
XML/simple-format text for `--chapters`, not an mkvmerge source with its own
tracks.

## TDD

**RED** -- added 5 tests to `crates/muxsmith-core/tests/planner_resolution.rs`
(appended at file end, following the existing task-tagged-comment
convention) before touching `planner.rs`:

- `chapters_drop_keyword_resolves_to_drop`
- `chapters_keep_keyword_resolves_to_keep`
- `chapters_external_one_match_resolves_to_external_path`
- `chapters_external_zero_matches_yields_missing_external_and_no_plan`
- `chapters_external_two_matches_yields_ambiguous_external`

Ran `cargo test -p muxsmith-core --test planner_resolution chapters`:

```
running 5 tests
test chapters_drop_keyword_resolves_to_drop ... FAILED
test chapters_external_two_matches_yields_ambiguous_external ... FAILED
test chapters_external_zero_matches_yields_missing_external_and_no_plan ... FAILED
test chapters_keep_keyword_resolves_to_keep ... ok
test chapters_external_one_match_resolves_to_external_path ... FAILED
test result: FAILED. 1 passed; 4 failed; 0 ignored
```

(The `keep` test passed trivially since `Keep` was already the Task-4
literal default; the other four failed as expected -- wrong variant or an
absent diagnostic -- confirming the tests actually exercise the unimplemented
behavior.)

**GREEN** -- after implementing `resolve_chapters` and wiring it in:

```
running 5 tests
test chapters_drop_keyword_resolves_to_drop ... ok
test chapters_keep_keyword_resolves_to_keep ... ok
test chapters_external_zero_matches_yields_missing_external_and_no_plan ... ok
test chapters_external_two_matches_yields_ambiguous_external ... ok
test chapters_external_one_match_resolves_to_external_path ... ok
test result: ok. 5 passed; 0 failed
```

## Full gate

- `cargo test --workspace`: all green (planner_resolution: 37 passed,
  including the 5 new; no regressions elsewhere).
- `cargo fmt --all --check`: clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: advisories/bans/licenses/sources all ok.

## Files changed

- `crates/muxsmith-core/src/planner.rs`: `use` import for `ChaptersCfg`;
  `resolve_chapters` helper; wired into `resolve_file`.
- `crates/muxsmith-core/tests/planner_resolution.rs`: 5 new tests.

## Self-review

- Diff matches the brief's spec literally: same match arms, same config_path
  string (`"chapters.external"`, not `tracks[..]`-scoped), same placeholder
  pattern, same reuse of `discovery::resolve_locator` and the primary_dir
  already computed in `resolve_file`.
- Checked `git diff` for non-ASCII bytes (`grep -P '[^\x00-\x7F]'`): none.
- `resolve_chapters` is private (`fn`, not `pub fn`), so `#![deny(missing_docs)]`
  does not require a `///` doc; it carries a `//` explanatory comment anyway,
  consistent with the file's existing style for private helpers
  (`resolve_title`, `resolve_tags`).
- Verified the `n == 2` ambiguous test actually produces 2 distinct hits: the
  external-locator regex match is a substring search on the rendered
  `{match}` pattern (`Template::render_regex_pattern` -> `regex::Regex::is_match`,
  not anchored), so `Show.S01E01.xml` and `Show.S01E01.alt.xml` both contain
  the identifier substring and both hit under a plain `match_to_source: true`
  locator -- no need for a hand-rolled `match_pattern` in that test.
- Did not touch `crates/muxsmith-core/src/discovery.rs`, `report.rs`,
  `validate.rs`, or attachments (Task 8 scope) -- confirmed via `git diff
  --stat` that only `planner.rs` and the test file changed.
- Left the untracked `HANDOFF.md` at the repo root alone; out of scope for
  this task.

## Concerns

None. Scope, interfaces, and test shape matched the brief exactly; no
ambiguity encountered that needed escalation. Note: this report file
previously held stale content from an unrelated earlier "Task 7: Template
Engine" plan; it has been overwritten with this task's report.
