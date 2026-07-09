# Task 12 report: reference-example golden + live mkvmerge acceptance

## What was done

Created `crates/muxsmith-core/tests/command_integration.rs` with two tests, plus
two new fixture files it depends on:

- `crates/muxsmith-core/tests/fixtures/identify/reference-primary.json` (9
  tracks: video, audio en/de, subtitle en forced/plain/SDH, subtitle de
  forced/plain/SDH)
- `crates/muxsmith-core/tests/fixtures/identify/reference-donor.json` (1
  subtitle track, the external Turkish donor)

No production code changed. `command.rs` and `tests/command.rs` (Task 9-11
goldens) are untouched: the existing argv contract needed no correction (see
"Live mkvmerge verification" below).

### 1. Pure golden: `reference_example_end_to_end`

**Variant chosen: (b), the preferred one** - drives `plan_batch` with a fake
`Identify`, not a literal `Plan`. And the full spec 4.1 example, not a
representative subset.

While reading the repo I found `crates/muxsmith-core/tests/fixtures/reference.yaml`
already checked in (from Plan 1 Task 4): the spec 4.1 example completed with
the German subtitle trio the spec doc elides as "omitted for brevity", already
consumed by `validate_semantics.rs`/`profile_load.rs`. This is exactly the "full
9-rule" fixture the task brief anticipated, so I reused it verbatim rather than
inventing a subset - it is a stronger lock (the actual reference example, not
an approximation) and there was no reason to duplicate or shrink it.

The fixture's 10 track rules (video, audio en, audio de, subtitle en x3,
subtitle de x3, external Turkish subtitle) resolve one-to-one against the
crafted `reference-primary.json`/`reference-donor.json` tracks: each fixture
track carries exactly the properties (`forced_track`, `flag_hearing_impaired`
presence, `track_name`) needed to satisfy exactly one rule's `exact`/`not`/`any`
combination, so the batch resolves with zero diagnostics and a full 10-entry
`Plan.assignments`. The test asserts that (10 assignments, all `track_id:
Some`) before asserting the argv, so a resolution regression fails loudly and
separately from an argv regression.

The full `command::command(&plan)` argv is then asserted against a hand-built
`expected: Vec<String>`, with the source/output/donor paths spliced in from the
actually-resolved `Plan` (real tempdir paths, not guessable string literals)
and every other token a literal from the canonical contract. This exercises:
multi-group (primary + external donor group), per-track property options
(`--default-track-flag`, `--forced-display-flag`-class flags, `--track-name`,
`--language`, alphabetical-by-property ordering, boolean `1`/`0` encoding),
and non-default title (`clear`) and tags (`global: drop`) - the "at least one
of attachments/chapters/tags/title non-default" bar, satisfied by title+tags
since the reference example's `attachments`/`chapters` are both `keep`-shaped
(spec default).

### 2. Live acceptance: `live_mkvmerge_accepts_planned_command`

Gated on `Mkvmerge::locate()`, self-skips with `eprintln!` + `return` exactly
like `identify_live.rs`/`mkvmerge_runtime.rs`. Per the task's minimal-fixture
suggestion: writes a one-line `.srt` to a tempdir, muxes it alone
(`mkvmerge -q -o source.mkv seed.srt`) into a one-track source MKV (no
audio/video codec needed), then drives `plan_batch` with a **real**
`LiveIdentifier` (real `IdentifyCache` + the located `Mkvmerge`, real
`--list-languages`) against a minimal profile selecting the subtitle track.
`command(&plan)` is spawned directly against `m.path()`, and the test asserts
exit 0, `plan.output.exists()`, and a real `identify_json` re-identification
(1 track, kind `subtitles`).

This is a stronger integration than "gated but synthetic": both identification
and muxing go through the real binary, so the whole pipeline (discovery ->
identify -> plan -> command -> mkvmerge) is exercised, not just the argv
generation in isolation.

## Live mkvmerge verification (before writing the golden's expected argv)

D11 and the task brief both flag the multi-input `( file )` grouping syntax as
the one genuinely uncertain piece of the canonical contract. Before writing
the golden's expected `Vec<String>`, I checked `man mkvmerge` and ran the
binary directly (mkvmerge v100.0, installed via linuxbrew):

- `man mkvmerge` (section "File splitting, linking, appending and
  concatenation"): `( file1 file2 )` is documented as **concatenating**
  multiple files into one logical segment, explicitly "cannot be used if each
  file contains its own set of headers... usually the case with stand-alone
  files like AVI or MP4" (MKV likewise self-contained). A single-file group is
  the doc's own example of the *degenerate* case, stated to be equivalent to
  prefixing the file with `=` (disables VOB-sibling-file auto-detection) - a
  no-op for a non-VOB source.
- Built two real one-track MKVs (audio-only, subtitle-only, muxed from the
  committed `tests/fixtures/seeds/{tone.wav,sub.srt}`) and ran mkvmerge by
  hand with exactly `command.rs`'s shape:
  `mkvmerge -o out.mkv --audio-tracks 0 --no-video --no-subtitles --no-buttons '(' primary.mkv ')' --no-audio --no-video --subtitle-tracks 0 --no-buttons '(' donor.mkv ')'`
  -> exit 0, re-identified output has both tracks correctly combined
  (`{id:0,type:audio}`, `{id:1,type:subtitles}`).
- Extended the manual check to the full contract shape used by the golden:
  `--title ""`, `--no-global-tags` per group, `--no-attachments` on the donor
  group, `--track-name`/`--language` per-track options, `--track-order` -> exit
  0, re-identified title is absent (empty title correctly cleared), track
  names/languages correctly applied, order correct.
- Verified boolean flag encoding separately: `--default-track-flag 0:1
  --forced-display-flag 0:0 --hearing-impaired-flag 0:1` -> exit 0,
  re-identified flags are exactly `{true, false, true}`. `1`/`0` (not
  `true`/`false`) is the correct encoding, confirming `command.rs`'s
  `value_str`.

**Finding: no correction needed.** `command.rs`'s existing argv (Tasks 9-11) is
accepted by mkvmerge v100 exactly as emitted; the `( file )` wrapping around a
single file per group is technically an unnecessary application of the
concatenation syntax (a plain filename with no wrapping would do the same
thing), but it is harmless for MKV/SRT sources per the manual page's own
stated equivalence, and empirically verified correct. This finding is recorded
as a code comment in the new test file's module doc comment (citing the man
page section and the manual command run), rather than as a `command.rs`
change, since there was nothing to change.

## TDD RED/GREEN

The pure golden's expected argv was derived by hand from the canonical
contract (task brief appendix 2) *after* the manual mkvmerge verification
above, so the first `cargo test` run of `reference_example_end_to_end` passed
immediately - there was no implementation bug to chase, and my derivation
matched on the first attempt. To confirm the assertion actually has
discriminating power (not vacuously true), I mutated one expected token
(`"5:English SDH"` -> `"5:English SDH!!MUTATED"`), reran, confirmed a clean
`assertion left == right failed` with the two full argv vectors printed and
only that one token differing, then reverted. This stands in for the RED phase
the brief describes: RED would have been the natural outcome had my
hand-derivation (or `command.rs`) been wrong; the mutation check demonstrates
the test would have caught it.

```
test reference_example_end_to_end ... ok
test live_mkvmerge_accepts_planned_command ... ok
```

Live test's actual local result: **PASS**, real mkvmerge v100.0 spawned twice
(fixture build + planned command), exit 0 both times, output file created and
re-identified with 1 subtitle track.

## Full gate (before commit)

- `cargo test --workspace`: all green (203 tests total across both crates +
  xtask; up from the 164 recorded at the previous HEAD, +2 new integration
  tests +0 elsewhere - the fixtures/JSON changes are data-only).
- `cargo fmt --all --check`: one formatting fixup applied to the new test file
  (import-list wrapping, a multi-line `fs::write` call collapsed), then clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

## Files changed

- `crates/muxsmith-core/tests/command_integration.rs` (new)
- `crates/muxsmith-core/tests/fixtures/identify/reference-primary.json` (new)
- `crates/muxsmith-core/tests/fixtures/identify/reference-donor.json` (new)

## Self-review

- Re-read `command.rs`'s doc comment and the canonical-contract appendix
  line-by-line against my hand-derived `expected` vector before running the
  test, specifically the alphabetical per-track-property ordering
  (`default_track` < `track_name`, `flag_hearing_impaired` < `track_name`,
  `language` < `track_name`) and the fixed category order (video, audio,
  subtitles, buttons) - all confirmed correct on the first run.
- Confirmed `Türkçe` (from the checked-in `reference.yaml`'s literal
  `changes.track_name`) is preserved verbatim rather than transliterated in the
  Rust assertion: the golden is reproducing the actual spec text end to end,
  so substituting an ASCII stand-in would misrepresent what the reference
  example actually produces.
- Checked the resolved-track fixture design for accidental cross-matches
  (e.g. the SDH track's `flag_hearing_impaired: true` must trip the *plain*
  rule's `not` clause so it does not also match that rule) by tracing each of
  the 9 tracks against all 9 non-external match expressions, not just its
  intended target rule.
- Confirmed no `OverlappingRules`/`AmbiguousRule`/`MissingTrack` diagnostics
  leak through: `plan.assignments.len() == 10` and every `track_id.is_some()`
  is asserted before the argv assertion, so a resolution failure surfaces as
  its own distinct assertion failure rather than a confusing argv mismatch.
- Ran the full workspace suite (not just the new test file) and all four gate
  commands myself rather than trusting the in-progress edits.

## Concerns

None blocking. Two minor observations, not actioned because out of Task 12's
scope:

- The `( file )` wrapping in `command.rs` is verified-correct but is,
  strictly, an unnecessary application of mkvmerge's concatenation syntax
  (equivalent to no wrapping at all, per the man page, for any non-VOB single
  file). Removing it would produce identical behavior with a simpler argv.
  Leaving as-is: it is what Tasks 9-11 already locked and it works; changing
  it now would be a cosmetic argv simplification outside this task's mandate
  (verify acceptance, fix only if rejected).
- The live test's fixture is deliberately minimal (one subtitle track, no
  audio/video, no attachments/chapters/tags/title changes) per the task's own
  "cheaply" framing. It does not exercise the multi-group donor path or
  per-track property flags against real mkvmerge the way the manual
  verification above did ad hoc; that manual verification is recorded in the
  code comment but is not itself a repeatable automated test. If broader live
  coverage is wanted later, the manual commands in this report are a ready
  template (Plan 4/5 follow-up territory, not blocking here).

## Note on a stale artifact found in .superpowers/sdd/

`task-12-report.md` at this path already existed before this task, containing
a Plan 1 Task 12 report ("Fluent catalogs, renderer, and `validate`
subcommand"). It had already been salvaged to
`docs/process-journal/artifacts/plan-1-sdd/task-12-report.md` (confirmed
identical) but the original was never deleted from `.superpowers/sdd/`, so this
Plan 3 report overwrote it in place. Flagging in case the salvage/cleanup step
for other stale `.superpowers/sdd/` files from earlier plans was likewise
skipped; not investigated further here (out of this task's scope).

## Whole-branch-review minor fixes

Three small, worth-it findings from the whole-branch review of Plan 3, applied
in one follow-up commit. Scope was strictly these three; no other review item
(richer live test, fixture ids, `FakeIdent` dedup, tempdir leak) was touched.

### Fix 1: comment accuracy (`command.rs`, `input_groups`)

The comment justifying excluding track-less donor input groups claimed
mkvmerge "may reject" an empty input group. Empirically false: mkvmerge v100
accepts an input group with all tracks excluded (verified during Task 12's
live acceptance work). The exclusion behavior itself was already correct;
only the stated reason was wrong. Rewritten to give the real rationale: a
donor contributing no kept track is pointless dead weight in the command
(its attachments are dropped anyway per D10), so it should not be opened as
an input at all.

Before:
```rust
// track) contributes no group: opening it would render an empty group
// (`--no-video --no-audio --no-subtitles --no-buttons ( <source> )`) that
// mkvmerge may reject.
```
After:
```rust
// track) contributes no group: it would carry no kept track into the output
// (and its attachments are dropped regardless, per D10), so opening it as an
// input at all would just be dead weight in the command.
```

### Fix 2: test gap for non-string `changes.language` (`planner_resolution.rs`)

`resolve_changes` validates a `changes.language` value with
`matches!(value, Scalar::Str(s) if lang.normalize(s).is_some())` - so a
non-string `Scalar` (bool, int, float) also fails validation and produces
`InvalidPropertyValue`, not just a recognized-but-invalid string like `"zzz"`
(the only case the existing `invalid_changes_language_is_plan_time_invalid_property_value`
test covered). Added
`changes_language_non_string_value_is_invalid_property_value`, built the same
way as its neighbor: a profile whose rule matches a track and sets
`changes: { language: true }` (deserializes to `Scalar::Bool(true)` via
`from_str`/serde_yaml), driven through the same `plan_one` harness
(fake `Identify` + `plan_batch`), asserting `fr.plan.is_none()` and an
`InvalidPropertyValue` diagnostic at `tracks[0].changes.language`.

Note: the test harness calls `from_str` + `plan_batch` directly, bypassing
`profile::validate` (the CLI-level config-time schema check that would
otherwise flag a bool `language` value as `ValueTypeMismatch` before planning
ever runs). The existing `"zzz"` test bypasses `validate` the same way, so
this test is consistent with the harness's established scope: it isolates
`resolve_changes`'s own plan-time validation.

TDD: wrote the test first and ran it in isolation before touching any other
gate.

```
$ cargo test -p muxsmith-core --test planner_resolution changes_language_non_string_value_is_invalid_property_value

running 1 test
test changes_language_non_string_value_is_invalid_property_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Passed immediately, no production-code change needed - confirming the
brief-required behavior already exists in `resolve_changes` and only lacked a
test.

### Fix 3: readability (`command.rs`, `value_str`)

The `Scalar::Bool(b)` arm read `if *b { "1" } else { "0" }.to_string()`.
Correct (`.to_string()` binds to the whole if/else, per Rust's expression
grammar) but reads at a glance like it applies only to the `else` arm.
Wrapped the conditional in parens for clarity; behavior unchanged:

```rust
Scalar::Bool(b) => (if *b { "1" } else { "0" }).to_string(),
```

### Full gate (after all three fixes)

- `cargo test --workspace`: all green, 204 tests total (up from 203 at HEAD
  0dcb116 - the one new planner test; +0 elsewhere).
- `cargo fmt --all --check`: clean, no diff.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.

### Files changed

- `crates/muxsmith-core/src/command.rs` (comment rewrite + paren wrap)
- `crates/muxsmith-core/tests/planner_resolution.rs` (new test)
