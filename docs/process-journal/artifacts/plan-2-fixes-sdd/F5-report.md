# F5: two planner-resolution fixes - report

## Status

DONE

## What changed

`crates/muxsmith-core/src/planner.rs`, `resolve_file`:

### (a) SourceOverwrite includes external donor paths

- Added `donor_paths: Vec<PathBuf>` alongside `claims`, declared before the
  per-rule loop.
- In the `SourceCfg::External` match arm's `hits.len() == 1` branch, push the
  resolved `donor` onto `donor_paths` immediately after binding it (before the
  `DonorIsPrimary` check and before calling `id.identify`), so a donor is
  recorded whether or not its identification later succeeds.
- The `SourceOverwrite` check now reads
  `primary_paths.contains(out) || donor_paths.contains(out)`, replacing
  `primary_paths.contains(out) || out == &primary.path`. The removed clause
  was redundant: `primary_paths` (built in `plan_core` from every primary in
  the batch) always contains `primary.path` itself, so membership already
  covers it.
- The `0` (no hits) and `n >= 2` (ambiguous) branches do not push to
  `donor_paths`: no single file was resolved in either case (the ambiguous
  branch already forces an `AmbiguousExternal` error, which alone drops the
  plan, so there is nothing this check would additionally protect there).

### (b) Identification failure -> UnidentifiableSource

- Primary identify failure: `Err(_)` -> `Err(e)`; diagnostic code
  `DiagCode::MissingTrack` -> `DiagCode::UnidentifiableSource`; `detail` param
  changed from the static string `"file could not be identified"` to
  `format!("{e:?}")` (the real `IdentifyError`, which derives `Debug`).
  `config_path` ("input") and `for_file` unchanged.
- Donor identify failure (inside the `hits.len() == 1` branch):
  `Err(_)` -> `Err(e)`; diagnostic code `DiagCode::MissingExternal` ->
  `DiagCode::UnidentifiableSource`; `detail` changed from the static string
  `"donor could not be identified"` to `format!("{e:?}")`. `config_path`
  (`"{base}.source.external"`) and `for_file` unchanged. This branch was
  already unconditional on `rule.optional` before this change (unlike the
  `hits.len() == 0` branch, which does gate on `!rule.optional`), so the
  "hard error regardless of optional" requirement needed no additional
  guard -- confirmed by the new `optional: true` test below, which still
  yields no plan.

No other files touched. `DiagCode::UnidentifiableSource` and its Fluent
message (`unidentifiable-source = A source file exists but could not be
identified: { $detail }.`) already existed from F2; this task is the first to
actually emit the code.

## Test-first

Added to `crates/muxsmith-core/tests/planner_resolution.rs`, three new tests
(two required by the task, one additional for symmetry between the primary
and donor identify-failure paths):

1. `unidentifiable_primary_yields_unidentifiable_source_not_missing_track`:
   `FakeIdent` with an empty fixture map (every `identify` call errors)
   against a primary file. Asserts `plan.is_none()`, a present
   `UnidentifiableSource` diagnostic with a non-empty `detail` param, and the
   *absence* of `MissingTrack`.
2. `unidentifiable_donor_yields_unidentifiable_source_not_missing_external`:
   an external-source rule (`match_to_source: true`, `optional: true`) whose
   locator resolves exactly one donor file (`Donor.S01E01.srt`), but the
   `FakeIdent` fixture map has no entry for it, so donor identification
   fails. Asserts `plan.is_none()` (proving `optional` does not suppress this
   error), a present `UnidentifiableSource`, and the absence of
   `MissingExternal`.
3. `source_overwrite_when_output_equals_donor_path`: an external-source rule
   whose locator points at an absolute sibling directory (`donors/`, outside
   the scanned source tree, so the donor file is never also discovered as a
   second primary) containing `Donor.S01E01.mkv`; `output.filename` is a
   template (`Donor.{match}.mkv`) and `run.output` is set to that same
   `donors/` directory, so the rendered output path is byte-for-byte the
   donor's resolved path. Asserts `plan.is_none()` and a present
   `SourceOverwrite`.

   The sibling-directory construction was necessary to get a *clean* signal:
   an in-tree donor with the same extension as `input.extensions` gets
   independently discovered as a second primary by `discovery::scan_primaries`,
   which would make `primary_paths.contains(out)` true too and no longer
   isolate the donor-path code path being tested.

Confirmed RED before implementing:

```
$ cargo test -p muxsmith-core --test planner_resolution
...
test unidentifiable_primary_yields_unidentifiable_source_not_missing_track ... FAILED
  panicked: expected UnidentifiableSource, got: [Diagnostic { code: MissingTrack, ... }]
test unidentifiable_donor_yields_unidentifiable_source_not_missing_external ... FAILED
  diags: [Diagnostic { code: MissingExternal, ... }]
test source_overwrite_when_output_equals_donor_path ... FAILED
  diags: [Diagnostic { code: OutputCollision, ... }]   <- SourceOverwrite absent
test result: FAILED. 7 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

The third failure's diagnostic list contains only `OutputCollision` (severity
error, since the donor file legitimately pre-exists on disk at the rendered
output path) and no `SourceOverwrite` -- confirming the plan-is-none outcome
in the unfixed code was incidental (an unrelated on-disk collision, not the
donor-overwrite protection under test), and that the assertion correctly
targets the missing code specifically rather than just `plan.is_none()`.

After implementing (a) and (b):

```
$ cargo test -p muxsmith-core --test planner_resolution
running 10 tests
... all ok
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

GREEN confirmed. `OutputCollision` still fires alongside `SourceOverwrite` in
test 3 (both are independently true facts about the same rendered path: it
already exists on disk *because* it is the donor, and it is a hard
source-overwrite regardless of collision policy per spec 4.8); the test does
not assert exclusivity, only that `SourceOverwrite` is present, which is the
actual acceptance criterion.

## Full verification commands run

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed (planner_resolution: 10 passed)
```

```
$ cargo fmt --all --check
(no output, exit 0)
```

```
$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
(no warnings, exit 0)
```

```
$ cargo build --workspace
(clean, exit 0)
```

ASCII check on both changed files (`grep -nP '[^\x00-\x7F]'`): no matches.

No new `pub` items were introduced (no new Display impl was needed --
`IdentifyError` already derives `Debug`, and `format!("{e:?}")` was terse
enough per the task's own suggestion), so `#![deny(missing_docs)]` required no
new doc comments.

## Concerns

None blocking. One judgment call, noted for the final whole-branch review:
the ambiguous-external branch (`hits.len() >= 2`) intentionally does not
contribute to `donor_paths`, since no single donor is resolved in that case
and the branch already forces `AmbiguousExternal` (error), which alone drops
the plan. If a future change ever makes ambiguous-external non-fatal, this
would need revisiting, but that is out of scope here.

## Addendum: fix pass on the independent review (F5-review.md)

The independent review (`F5-review.md`) found the above DONE state
incomplete on one Critical point and flagged one Minor point; both are now
fixed.

### Critical #1: `SourceOverwrite` was per-primary, not batch-wide

The review's own harness proved it: primary A's donor could equal primary
B's rendered output, and B's plan survived (only an `Info`-severity
`OutputCollision`, no `SourceOverwrite`) because `donor_paths` was a local
`Vec` reset on every `resolve_file` call, populated only from the *current*
primary's own resolved donors.

Fix, in `crates/muxsmith-core/src/planner.rs`:

- Removed `donor_paths` and the per-primary `SourceOverwrite` check from
  `resolve_file` entirely. `Assignment.source` already records the same
  information per resolved rule (donor path for a resolved external rule,
  primary path otherwise), so the local `Vec` was redundant once the check
  moved batch-wide.
- Added `detect_source_overwrites(files, primary_paths)`: builds one
  `BTreeSet<PathBuf>` of every primary path plus every `assignment.source`
  from every file's (still-`Some`) plan, then pushes a `SourceOverwrite`
  error onto any file whose rendered `output` is in that set.
- Wired it into `plan_core` *before* the first `finalize_plans` call (it
  needs every file's assignments still present, and the newly-added error
  must be there in time for `finalize_plans` to drop the plan) - ahead of
  the existing `finalize_plans -> detect_output_collisions -> finalize_plans`
  sequence, which is otherwise unchanged.

Test-first (`crates/muxsmith-core/tests/planner_resolution.rs`,
`source_overwrite_is_batch_wide_not_per_primary`): two primaries, A and B,
sharing one profile. B's external rule (`match_to_source: true`) resolves a
real on-disk donor whose name embeds B's own identifier; A's identifier
never matches that donor, so A's own rule evaluation never touches it (A's
old per-primary `donor_paths` would have stayed empty). `output.filename` is
a fixed literal template (no `{match}` field), so *every* primary in the
batch, A included, renders to that exact donor path; `run.output` points at
the donor's directory and `on_collision: overwrite` is set to reproduce the
review's exact "survives as an Info `OutputCollision`" symptom. Confirmed
RED against the pre-fix code:

```
diags: [Diagnostic { code: OutputCollision, severity: Info, config_path: "output",
  file: Some(".../src/Show.S01E01.mkv"),
  params: {"path": ".../donors/Donor.S01E02.mkv"}, suggestion_ref: None }]
```

(exactly the symptom the review predicted: plan survives, only an `Info`
`OutputCollision`, no `SourceOverwrite`). GREEN after the fix, alongside all
10 pre-existing tests in the file (including the single-primary
`source_overwrite_when_output_equals_donor_path`, unaffected by moving the
check batch-wide).

### Minor #4: `detail` embedded raw `Debug` output in user-facing text

Added `impl std::fmt::Display for IdentifyError` in
`crates/muxsmith-core/src/identify.rs`: a terse phrase per variant
(`"mkvmerge failed: <err>"` for the three `RuntimeError` sub-variants,
`"invalid identification JSON: <err>"`, `"cannot read file: <err>"`), no
Rust enum/struct syntax. Both call sites in `planner.rs` that build the
`detail` param (`format!("{e:?}")`, at the primary-identify and
donor-identify failure branches) now use `format!("{e}")`. No rustdoc
needed on the impl itself (trait impls for a pre-existing std trait are
exempt from `#![deny(missing_docs)]`); added a unit test
(`identify::tests::display_is_a_terse_phrase_not_the_debug_dump`) pinning
the exact rendered string for each variant, including the `NonZero { code,
stderr }` case.

Important #2 (the plan-doc-vs-spec contradiction on donor-identify-failure
and `optional`) and Minor #3 (the report's incorrect GREEN-state narrative
for the old test 3) were left as-is: both are out of scope for this task
(no code or test behavior to fix; #2 is a documentation-reconciliation
question for the plan doc, #3 is a correction to a *prior* report's prose,
not to shipped code).

### Verification

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed
    muxsmith-core lib: 63 passed (was 62, +1 Display test)
    planner_resolution: 11 passed (was 10, +1 batch-wide SourceOverwrite test)

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
(no warnings, exit 0)

$ grep -nP '[^\x00-\x7F]' crates/muxsmith-core/src/planner.rs \
    crates/muxsmith-core/src/identify.rs \
    crates/muxsmith-core/tests/planner_resolution.rs
(no matches - ASCII-clean)
```

### Concerns

None blocking. Both fixes are scoped exactly to the two review findings
handed to this task; the review's Important #2 and Minor #3 points are
noted above as knowingly not addressed here.
