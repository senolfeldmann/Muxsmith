# F6: output rendered-name invariant + collision semantics - report

## Status

DONE

## What changed

`crates/muxsmith-core/src/planner.rs`:

### (a) `render_output`: EmptyRenderedName now tests the pre-append value (bug B)

- Renamed the local `name` binding to `rendered` and stopped appending
  `.mkv` before the invariant checks. `FilenameCfg::Keyword` now yields just
  the source file stem (no `format!("{stem}.mkv")`); `FilenameCfg::Template`
  yields the raw `render_literal` output (the "if not ends with .mkv, push
  it" step was removed from this branch).
- `PathSeparatorInRenderedName` now checks `rendered` (unaffected in
  practice: `.mkv` never introduces a separator, so this check's outcome
  was already correct before, only its input variable renamed for
  consistency with the fix below).
- `EmptyRenderedName` now checks `rendered.is_empty() || rendered == "." ||
  rendered == ".."` -- the pre-append value. The old `stem_only` variable
  (computed by stripping a `.mkv`/`.MKV` suffix off the post-append `name`)
  is gone; it was the workaround that made the direct `name == "."` /
  `name == ".."` arms dead code, since `name` always already ended in
  `.mkv` by the time they ran.
- `.mkv` is now appended once, after both checks pass, into a `name`
  rebound from the validated `rendered` value, and only that final `name`
  is joined onto `output_dir`.

Before this fix, a template rendering to `"."` reached the checks as
`"..mkv"` (`.` plus the appended `.mkv`): `stem_only` (`.mkv` stripped) was
`"."`, non-empty, and `name` (`"..mkv"`) matched neither `"."` nor `".."`,
so no diagnostic fired and the plan silently produced a hidden/malformed
output file. Now the check runs on `"."` itself, before any append.

### (b) `detect_output_collisions`: collision semantics per amended spec 4.8 (decision #3, bug E)

- Two plans rendering to the same path: previously `Skip` policy downgraded
  this to a `Warning` (and, since `finalize_plans` only drops `Error`
  severity, the plan survived) while every other policy stayed `Error`.
  Now it is unconditionally `Severity::Error` regardless of `policy` --
  decision #3: the batch is internally inconsistent and neither `skip` nor
  `overwrite` can pick a winner between two plans claiming one path.
- A pre-existing on-disk file at the output path (not a batch input;
  `SourceOverwrite`, handled by `detect_source_overwrites`, covers that
  case) keeps following `on_collision`: `Error` -> `Error`, `Skip` ->
  `Warning`, `Overwrite` -> `Info`. This mapping was already correct before
  this task; what was missing (bug E) was that `Skip` never dropped the
  plan, because `Warning` severity is invisible to `finalize_plans` (which
  only nulls a plan on `Error`). The function now explicitly sets
  `f.plan = None` in that one case (`!planned_twice && policy ==
  CollisionPolicy::Skip`), so "skip" actually means the output is not
  produced rather than being a silently-ignored warning with the plan
  intact.
- The two-branches-of-`if`/`match` chain that previously special-cased
  `Skip` inside the `planned_twice` arm is gone; `planned_twice` now maps
  unconditionally to `Severity::Error`, and the on-disk `match policy {
  ... }` arm is unchanged from before.
- Restructured the borrow to extract `out` (the output `PathBuf`) via
  `f.plan.as_ref().map(|p| p.output.clone())` up front, instead of holding
  `&f.plan` across the loop body: the new code needs to both push a
  diagnostic onto `f.diagnostics` and potentially null `f.plan` in the same
  iteration, which a live borrow of `f.plan` would not allow. `for_file`
  now takes `&f.source` instead of the former `&plan.source` binding (the
  two are always equal: both are set from `primary.path.clone()` in
  `resolve_file`).

`crates/muxsmith-core/src/report.rs`:

### (c) `Diagnostic::with_severity` builder

- Added `pub fn with_severity(mut self, severity: Severity) -> Self`,
  rustdoc'd, placed after `for_file` in the builder chain.
- `detect_output_collisions` no longer constructs `let mut d =
  Diagnostic::info(...); ...; d.severity = severity;`; it now chains
  `.with_severity(severity)` onto the builder, so severity is set through
  the same builder chain as `with`/`for_file` rather than by reaching into
  the public field.
- Added a unit test, `with_severity_overrides_constructor_severity`, next
  to the existing `diagnostic_builder_sets_fields` test in `report.rs`'s
  `tests` module.

## Test-first

Added to `crates/muxsmith-core/tests/planner_resolution.rs`:

1. `empty_rendered_name_when_template_renders_to_dot`: `output.filename.template: '.'`
   against a single primary. Asserts `plan.is_none()` and a present
   `EmptyRenderedName`.
2. `two_planned_outputs_to_same_path_are_always_output_collision_error`: two
   primaries (`Show.S01E01.mkv`, `Show.S01E02.mkv`) under a fixed-literal
   `output.filename.template` (no `{match}` field, so both render to the
   same `Same.mkv` in a sibling `out/` directory), looped over
   `on_collision` in `[None, Error, Overwrite, Skip]`. Asserts for every
   policy: both files' plans are `None`, both carry an `OutputCollision`
   diagnostic, and its severity is `Error`.

   New helper `plan_two_same_output(policy) -> Batch`, and `Batch` added to
   the test file's `muxsmith_core::planner` import (needed as the helper's
   return type once it stopped being an inline single-file `plan_one`
   call). Source and output directories are siblings (`root/src`,
   `root/out`) rather than the pre-existing `plan_one` helper's nested
   `source/out` layout: `input.recursive` defaults to `true` (spec 4.2),
   scoped to `run.source`, so a file written into a nested output dir
   would get rediscovered as a second, unwanted primary. `plan_one`'s
   existing tests never write anything into their nested `out/`, so this
   was latent, not previously triggered.
3. `on_disk_collision_under_skip_is_warning_and_drops_plan`,
   `on_disk_collision_under_overwrite_is_info_and_keeps_plan`,
   `on_disk_collision_under_error_is_error_and_drops_plan`: new helper
   `plan_one_with_existing_output(policy) -> Batch`, same sibling
   `src`/`out` layout, pre-creates a file at the exact rendered ("keep")
   output path before planning. Assert, per policy: `Skip` -> `Warning`
   severity and `plan.is_none()` (bug E: the plan-drop, not just the
   severity, is the assertion that would have failed against the old
   code); `Overwrite` -> `Info` and `plan.is_some()`; `Error` -> `Error`
   and `plan.is_none()`.

Confirmed RED before implementing:

```
$ cargo test -p muxsmith-core --test planner_resolution
running 16 tests
...
test empty_rendered_name_when_template_renders_to_dot ... FAILED
  diags: []
test on_disk_collision_under_skip_is_warning_and_drops_plan ... FAILED
  diags: [Diagnostic { code: OutputCollision, severity: Warning, ... }]
  (plan was Some, not None -- bug E)
test two_planned_outputs_to_same_path_are_always_output_collision_error ... FAILED
  policy Some(Skip): diags: [Diagnostic { code: OutputCollision, severity: Warning, ... }]
test result: FAILED. 13 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

Exactly the three predicted symptoms: the dead-code empty-name check let
`.` through, `Skip` on an on-disk collision left the plan intact under
only a `Warning`, and `Skip` on a two-planned-outputs collision was
likewise a `Warning` rather than the unconditional `Error` decision #3
calls for.

After implementing (a), (b), (c):

```
$ cargo test -p muxsmith-core --test planner_resolution
running 16 tests
... all ok
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

GREEN confirmed.

## Full verification commands run

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed
    muxsmith-core lib: 64 passed (was 63, +1 with_severity test)
    planner_resolution: 16 passed (was 11, +5 F6 tests)

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)
```

ASCII check on all three changed files (`grep -nP '[^\x00-\x7F]'`): no
matches.

`#![deny(missing_docs)]`: the only new `pub` item is
`Diagnostic::with_severity`, rustdoc'd.

## Concerns

None blocking. Two judgment calls, noted for the whole-branch review:

- The task text says "for `skip`, ALSO DROP the plan (set it to None)" for
  the on-disk case specifically; decision #3 already makes the
  two-planned-outputs case unconditionally `Error`, which `finalize_plans`
  drops on its own, so no separate explicit null was needed for that
  branch (verified by the RED/GREEN transition in test 2, which never
  depended on an explicit null in the `planned_twice` arm).
- `for_file(&f.source)` replaces the old `for_file(&plan.source)`. Both are
  always equal (`resolve_file` sets `Plan.source` and `FileReport.source`
  from the same `primary.path.clone()`), so this is a pure borrow-checker
  workaround, not a behavior change; left as a one-line note here rather
  than a code comment, since the invariant is already documented on the
  `Plan`/`FileReport` struct fields themselves.

## Addendum: fix pass on the independent review (F6-review.md)

The independent review found one Important finding (the keep-name path
regressed) and two, both fixed.

### Important #1: keep-name `.mkv` handling regressed

Unifying `FilenameCfg::Keyword` with `FilenameCfg::Template` under one
conditional `.mkv`-append made the keep arm share the template arm's "append
only if not already ending in `.mkv`" check. keep is defined (spec 4.8) as
`file_stem + ".mkv"` unconditionally: the source's own extension carries no
meaning to preserve, even when the stem itself already ends in something
that looks like `.mkv` (a double extension). The shared conditional instead
truncated one `.mkv` off in that case.

Fix, in `render_output` (`crates/muxsmith-core/src/planner.rs`): kept the D4
invariant checks (`PathSeparatorInRenderedName`, `EmptyRenderedName`) on the
unified pre-append `rendered` value, but replaced the single shared
`if !rendered.ends_with(".mkv") { push }` step with a second match on
`profile.output.filename`: `Keyword` -> `format!("{rendered}.mkv")`
unconditionally; `Template` -> `rendered` unchanged if it already ends in
`.mkv` (case-insensitively), else `.mkv` appended. The D4 checks and the
collision/`SourceOverwrite` logic downstream are untouched.

### Important #2 (test hygiene): the valid-name-append path was untested

Added to `crates/muxsmith-core/tests/planner_resolution.rs`:

1. `keep_filename_on_mp4_source_replaces_extension_with_mkv`: a keep-name
   `.mp4` source; asserts the output is `Show.S01E01.mkv`. Passes under both
   the buggy and fixed code (file_stem already strips `.mp4`, so the shared
   conditional's `ends_with(".mkv")` check was false either way) -- kept per
   the task's explicit request as a locked-in regression guard, not a RED
   case.
2. `keep_filename_does_not_apply_the_templates_conditional_append`: the
   actual RED case. Source `Show.S01E01.mkv.mkv` (a keep-name source whose
   stem, `Show.S01E01.mkv`, already ends in `.mkv`). Confirmed RED against
   the pre-fix code: `plan.output.file_name()` was `"Show.S01E01.mkv"` (one
   `.mkv` truncated by the shared conditional) instead of the expected
   `"Show.S01E01.mkv.mkv"` (unconditional `stem + ".mkv"`). GREEN after the
   fix.
3. `template_filename_appends_mkv_when_missing`: `template: 'Custom'` ->
   `Custom.mkv`.
4. `template_filename_already_ending_in_mkv_is_not_doubled`: `template:
   'Custom.mkv'` -> stays `Custom.mkv`, not `Custom.mkv.mkv`.

   Both template tests exercise the arm whose behavior did not change
   across the fix (the conditional append was always correct for
   `Template`; only `Keyword` lost its unconditional append), so neither is
   a RED case either -- both pass before and after, added as coverage per
   the task's explicit request since the append path had no direct test
   before this task.

### Minor #3: empty-string `EmptyRenderedName` untested

Added `empty_rendered_name_when_template_renders_to_empty_string`:
`filename: { template: '' }` next to the existing dot-rendering test.
`Template::parse("")` succeeds (zero segments) and `render_literal` returns
`""`, so `rendered.is_empty()` fires `EmptyRenderedName` with `plan: None`,
same code path as the `"."` case, just a different one of the three
rejected values. Passes under both the buggy and fixed code (this path was
never touched by the regression); added purely to close the coverage gap
the task named.

### Verification

```
$ cargo test -p muxsmith-core --test planner_resolution
running 21 tests (was 16, +5)
... all ok
test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --workspace
... every suite: test result: ok, 0 failed

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)

$ grep -nP '[^\x00-\x7F]' crates/muxsmith-core/src/planner.rs \
    crates/muxsmith-core/tests/planner_resolution.rs
(no matches - ASCII-clean)
```

No new `pub` items were introduced by this addendum, so `#![deny(missing_docs)]`
required no new doc comments.

### Concerns

None blocking. Same scoping note as the F5 addendum: this pass is scoped
exactly to the three review findings handed to it.

## Addendum: fix pass on the final whole-branch review (FINAL-review.md, finding I1)

The final review found one Important regression in this task's own (a) fix:
`render_output`'s `EmptyRenderedName` check tested the raw pre-append
`rendered` value, so a template rendering to exactly `.mkv` (already
carrying the extension, empty stem) passed the pre-append check (non-empty,
not `.`/`..`) and then the template arm left it unchanged (already ends in
`.mkv`), producing the hidden, empty-stem output file `.mkv` with exit 0 --
the invariant this task was supposed to close, reopened by its own fix.

### Fix

`crates/muxsmith-core/src/planner.rs`, `render_output`:

- Removed the pre-append `rendered.is_empty() || rendered == "." ||
  rendered == ".."` check entirely. It is now fully subsumed: appending or
  keeping `.mkv` and then stripping it back off, per the final-stem check
  below, catches every value the pre-append check caught (rendered `""` ->
  final name `.mkv` -> stem `""`; rendered `"."` -> final name `..mkv` ->
  stem `"."`; rendered `".."` -> final name `...mkv` -> stem `".."`) plus
  the case it missed (rendered `.mkv"` itself, or any empty-field-then-
  literal-`.mkv"` template, both of which the template arm leaves
  unchanged before the check now runs). Two checks doing overlapping work
  is unneeded complexity now that the final-stem check is a strict
  superset.
- `PathSeparatorInRenderedName` is unchanged: still checks pre-append
  `rendered`, since `.mkv` never itself introduces a separator.
- After computing the final `name` (post keep/template `.mkv` handling,
  same as before), added the final-stem check: a new private helper
  `strip_mkv_suffix(name: &str) -> &str` strips a trailing `.mkv`
  case-insensitively (guarded by `str::is_char_boundary` before slicing, so
  a non-matching multi-byte tail is never sliced into -- `.mkv` is ASCII,
  so a real match is always at a safe boundary, but the guard makes that
  safe rather than merely true by construction). If the resulting stem is
  empty, `.`, or `..`, `EmptyRenderedName` fires and the function returns
  `None`, same as before.
- The keep arm's unconditional append and the template arm's
  already-ends-in-`.mkv` conditional (this task's earlier (a)/addendum
  fix) are untouched; the new check runs after both, on their shared
  output `name`.

### Test-first

Added to `crates/muxsmith-core/tests/planner_resolution.rs`:

1. `empty_rendered_name_when_template_renders_to_literal_mkv`:
   `output.filename.template: '.mkv'`. Confirmed RED against the pre-fix
   code (`diags: []`, plan present); GREEN after the fix.
2. `empty_rendered_name_when_template_field_renders_empty_before_literal_mkv`:
   the same failure mode reached a different way -- `input.pattern:
   'S(?<s>\d{2})E(?<e>\d{2})(?<x>Q)?'` (an optional named group, `x`, that
   exists in the pattern but never participates in this fixture's match)
   with `output.filename.template: '{x}.mkv'`. `x` is a valid template
   field (profile validation derives allowed fields from the regex's
   capture names statically, not from a specific match), but `Ctx` never
   binds it for this file, so it interpolates as `""`, rendering `{x}.mkv`
   to `.mkv"` -- byte-identical to test 1's direct literal, reached through
   the field-interpolation path instead. Confirmed RED (`diags: []`, plan
   present); GREEN after the fix.

Both assert `plan.is_none()` and a present `EmptyRenderedName` diagnostic,
matching the existing `empty_rendered_name_when_template_renders_to_dot`/
`..._to_empty_string` test style.

RED:

```
$ cargo test -p muxsmith-core --test planner_resolution
running 23 tests
...
test empty_rendered_name_when_template_renders_to_literal_mkv ... FAILED
  diags: []
test empty_rendered_name_when_template_field_renders_empty_before_literal_mkv ... FAILED
  diags: []
test result: FAILED. 21 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

GREEN after the fix:

```
$ cargo test -p muxsmith-core --test planner_resolution
running 23 tests
... all ok
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

All prior F6 tests (keep/`.mkv`/`.mp4` sources, template append, template
already-`.mkv` not doubled, the two dot/empty-string `EmptyRenderedName`
cases, both collision suites) stayed green throughout; none needed changes.

### Verification

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)

$ grep -nP '[^\x00-\x7F]' crates/muxsmith-core/src/planner.rs \
    crates/muxsmith-core/tests/planner_resolution.rs
(no matches - ASCII-clean)
```

No new `pub` items were introduced (`strip_mkv_suffix` is a private
helper), so `#![deny(missing_docs)]` required no new doc comments.

### Concerns

None blocking. `strip_mkv_suffix`'s `is_char_boundary` guard is defensive
rather than load-bearing under the current call sites: every `name` this
function sees is either `rendered` with `.mkv` freshly appended via
`format!`, or `rendered` itself already ending in some ASCII case-variant
of `.mkv` per the `ends_with` check just above it, so the boundary is
always valid by construction today. Kept the explicit guard anyway so the
helper stays correct if a future caller ever passes it an arbitrary
string, rather than relying on an invariant enforced only by its current
call sites.
