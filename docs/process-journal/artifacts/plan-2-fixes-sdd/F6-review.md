# F6 review: output rendered-name invariant + collision semantics

Independent review of commit `b5acada` ("fix(core): planner rendered-name and
collision severity per spec 4.8"). Verified the committed diff is byte-for-byte
the same changed lines as `F6-review-package.txt` (differs only in diff
context width). Ran `cargo test --workspace` (all suites green, 0 failed),
`cargo test -p muxsmith-core --test planner_resolution` (16 passed), and
`cargo clippy --workspace --all-targets -- -D warnings` (clean) myself; results
match the report's claims.

## Verdict

- **SPEC: fail** -- the amended decision #3 (two-planned-always-error) and the
  on-disk skip/overwrite/error severity+plan-drop mapping are correctly and
  completely implemented and tested. But part (a)'s own contract ("a valid
  name still appends `.mkv`") is silently violated for one class of `keep`
  inputs (Finding 1), which is a spec 4.8 violation ("keep": source basename,
  `.mkv` extension *enforced*), introduced by this diff, in the exact function
  under review.
- **QUALITY: changes-needed** -- Finding 1 is a real regression; Finding 2 is
  the test-hygiene gap that let it through unnoticed by the new (or any
  existing) test.

## Findings

### Finding 1 (Important) -- `render_output`'s shared `.mkv`-append step silently truncates "keep" filenames that contain an inner `.mkv`

**File:** `crates/muxsmith-core/src/planner.rs`, lines 442-448 (Keyword arm) and
477-480 (shared append).

Before this diff, the `FilenameCfg::Keyword` ("keep") arm built the name as
`format!("{stem}.mkv")` -- an *unconditional* append, correct because
`file_stem()` already strips exactly the file's last extension, so
`stem + ".mkv"` always reconstructs the original basename (given the file was
discovered because its extension is `mkv`).

This diff correctly unifies "check the invariants before appending" across
both arms, but as a side effect also unifies the *append* step itself: the
Keyword arm's `rendered` is now the bare `file_stem()` value, and the shared
append logic only appends `.mkv` `if !name.to_lowercase().ends_with(".mkv")`.
That conditional is fine for the `Template` arm (where a user's template can
legitimately already end in `.mkv`), but wrong for the Keyword arm: if the
source basename contains an inner `.mkv` immediately before its real
extension (e.g. `Show.S01E01.mkv.mkv`), `file_stem()` yields
`Show.S01E01.mkv` -- which itself ends in `.mkv` -- so the shared check treats
it as "already has the extension" and skips the append, producing
`Show.S01E01.mkv` instead of reconstructing `Show.S01E01.mkv.mkv`.

Verified the exact string transformation in isolation (standalone script,
not touching the repo) mirroring lines 442-480 verbatim:

```
name="Show.S01E01.mkv" stem="Show.S01E01" stem_ends_with_mkv=false
  new_name="Show.S01E01.mkv" old_name="Show.S01E01.mkv" match_original=true
name="Movie.mkv.mkv"   stem="Movie.mkv"   stem_ends_with_mkv=true
  new_name="Movie.mkv"       old_name="Movie.mkv.mkv"    match_original=false
```

**Failure scenario:** a discovered primary named e.g. `Show.S01E01.mkv.mkv`
(a plausible artifact of a prior re-mux, a download tool, or a manual rename
mistake -- it passes `extensions: [mkv]` and any `input.pattern` matching
`S01E01` trivially) under the default `filename: keep` silently produces an
output plan named `Show.S01E01.mkv`, dropping part of the original name, with
**no diagnostic at all** -- the file just gets a different name than "keep"
promises. If a second, genuinely different, primary named `Show.S01E01.mkv`
also happens to be in the same batch, this additionally manufactures an
unintended `OutputCollision` between two originally distinct filenames.

Not a crash, not a guaranteed data-loss path by itself (rated Important, not
Critical), but a silent, in-scope contract violation of exactly the invariant
this task was reviewing.

### Finding 2 (Important, test hygiene) -- no test exercises the positive "name lacking `.mkv` still gets it appended" path, for either arm

The task's own checklist asks: "does render_output... correctly yield
EmptyRenderedName pre-append (**and a valid name still appends .mkv**)?" No
test in `planner_resolution.rs` checks this. Every pre-existing
`Template`-mode fixture in the file already ends its literal template in
`.mkv` (`'Donor.{match}.mkv'`, `'Donor.S01E02.mkv'`, `'Same.mkv'`), and the new
`'.'`-only test is rejected before the append step ever runs (it fails
`EmptyRenderedName` first). `keep_filename_renders_mkv_output` (pre-existing,
unmodified by F6) uses `Show.S01E01.mkv`, whose stem never ends in `.mkv`, so
it can't distinguish the old unconditional-append behavior from the new
conditional one.

This is precisely the gap that let Finding 1 through: nothing in the suite
asserts what final filename `render_output` actually produces when the
pre-append value does not already end in `.mkv`, isolated for the `keep`
(Keyword) arm specifically. A test with a `keep`-mode primary named with an
inner `.mkv` (or even just a `Template` fixture whose literal doesn't end in
`.mkv`, e.g. `template: '{match}'`) asserting the exact output filename would
have caught Finding 1 directly.

### Finding 3 (Minor, test hygiene) -- `EmptyRenderedName`'s empty-string branch is untested

Only the `rendered == "."` arm got a new test
(`empty_rendered_name_when_template_renders_to_dot`). The `rendered.is_empty()`
arm (`template: ''`, or a template consisting only of fields that render
empty) has no test anywhere in the suite, even though D4's own design-decision
rationale (`docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md`,
"D4") explicitly calls out `template: ""` as "the empty case... reachable
today" -- the specific scenario the decision was motivated by. Low risk in
isolation (all three conditions share one `||` expression, so a break in one
likely breaks the others identically), but it's the literal case the spec
amendment names, and it's the one condition of the three left unverified.

## What was verified as correct

- `detect_output_collisions`: two-planned-outputs is now unconditionally
  `Severity::Error` regardless of `on_collision` (`None`, `Error`,
  `Overwrite`, `Skip` all tested in one parametrized test), and both
  colliding plans are dropped (asserted `plan.is_none()` for both files, not
  just the diagnostic). Matches spec 4.8's amended decision #3 exactly.
- On-disk collision severity-to-policy mapping (`Error`->Error/drop,
  `Skip`->Warning/**drop**, `Overwrite`->Info/keep) is each individually
  tested, and each test asserts **both** the diagnostic severity and the
  concrete `plan` `Some`/`None` outcome, not just one or the other -- this is
  exactly what the task asked to check, and bug E (skip not actually dropping
  the plan) is directly falsified by
  `on_disk_collision_under_skip_is_warning_and_drops_plan`.
- `render_output`'s invariant checks themselves (`EmptyRenderedName`,
  `PathSeparatorInRenderedName`) correctly run on the pre-append `rendered`
  value now, fixing bug B (a `.`-template no longer silently becomes
  `..mkv`); the new `.`-template test confirms this for the reachable case.
- `detect_source_overwrites` (F5's batch-wide `SourceOverwrite` pass) is
  untouched by this diff (confirmed by re-reading the function and by the
  diff itself not touching it), and its existing tests
  (`source_overwrite_when_output_equals_donor_path`,
  `source_overwrite_is_batch_wide_not_per_primary`) still pass.
- `plan_batch`'s ordering
  (`detect_source_overwrites -> finalize_plans -> detect_output_collisions ->
  finalize_plans`) is unchanged by the diff (outside any touched hunk) and
  still correctly drops error-severity collision plans via the second
  `finalize_plans` call, while the on-disk-skip case needs (and gets) an
  explicit `f.plan = None` inside `detect_output_collisions` itself because
  its severity is only `Warning`.
- `Diagnostic::with_severity`: correctly implemented, rustdoc'd (satisfies
  `#![deny(missing_docs)]`), unit-tested, and used to replace the prior direct
  mutation of the public `severity` field. The `for_file(&f.source)` vs. the
  old `for_file(&plan.source)` swap is a pure borrow-checker workaround with
  no behavior change, correctly justified by the paths always being equal by
  construction in `resolve_file`.
- `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D
  warnings`, and `cargo test --workspace` all pass cleanly, matching the
  report's claims.
