# Task F1: dry-run config-time validation + `--json` rendered messages

## What changed

`crates/muxsmith-cli/src/commands/dry_run.rs` (CLI-layer only, `muxsmith-core` untouched):

1. **Bug A fix** - `run()` now calls `muxsmith_core::profile::validate::validate(&profile)` and
   `muxsmith_core::profile::lint::provable_overlaps(&profile)` immediately after loading the
   profile, before the `Mkvmerge::locate()` lookup (validate needs no filesystem access beyond
   the profile; planning does). The resulting `config_diags: Vec<Diagnostic>` is threaded through:
   - `exit_code()` now takes `(config_diags, batch)` and computes the worst severity across
     config diagnostics, `batch.batch_diagnostics`, and every file's `diagnostics` (`all_diags()`
     grew a third chained iterator).
   - Human output prints each config diagnostic (via `renderer.diagnostic`) before the existing
     per-file/batch/suggestions output.
   - Planning (`plan_batch`) still runs unconditionally afterwards; no fail-fast, per the task
     brief (a config error does not suppress planning diagnostics, matching how `MissingTrack`
     from an unresolvable rule and the config-time `UnknownProperty` that caused it can now both
     surface in one report).

2. **Bug F fix** - `--json` no longer does `serde_json::to_string(&batch)` (raw, unrendered). A
   new `batch_json()` builds the report object by hand, field-for-field matching `Batch`'s
   existing shape (`files`/`batch_diagnostics`/`suggestions`) plus a new top-level
   `config_diagnostics` array, running every diagnostic list (config, batch-level, and each
   file's per-file list) through a new `rendered_diags()` helper that mirrors `validate.rs`'s
   `v["rendered"] = renderer.diagnostic(d)` pattern.

### Design decisions / assumptions (per "use your judgment" in the brief)

- **`config_diagnostics` is a distinct top-level JSON array**, not merged into
  `batch_diagnostics`. Rationale: the task phrasing keeps "config diagnostics" and "batch
  diagnostics (per-file + batch-level)" as separate pools for the exit-code computation, and
  `muxsmith-core`'s `Batch` type has no slot for config-time diagnostics (out of scope to change
  it) - a separate field is more legible for scripts than silently folding a config-time concept
  into a struct field documented as "runtime config checks and cross-file facts" in core's own
  rustdoc.
- **mkvmerge-not-found path is unchanged**: `config_diags` is computed before the
  `Mkvmerge::locate()` check but is *not* printed on that early-return path; the existing
  `eprintln!("mkvmerge-not-found"); return 2;` behavior is preserved verbatim. The brief allowed
  either choice ("you may still surface config diagnostics; use your judgment"); I kept the
  narrower change since that path has no test coverage in this environment (mkvmerge is
  installed) and printing config diagnostics there would need a JSON shape decision with nothing
  to verify it against. Flagging this as the one open dimension rather than silently picking it.

## TDD evidence

### Red (before implementation)

Added two tests to `crates/muxsmith-cli/tests/dry_run_cli.rs`, ran against the unmodified
`dry_run.rs`:

```
running 3 tests
test dry_run_surfaces_config_time_invalid_regex ... FAILED
test dry_run_plans_a_single_file ... ok
test dry_run_json_diagnostics_all_carry_rendered_text ... FAILED

---- dry_run_surfaces_config_time_invalid_regex stdout ----
thread 'dry_run_surfaces_config_time_invalid_regex' panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:104:5:
assertion `left == right` failed: stdout: {"files":[],"batch_diagnostics":[],"suggestions":[]}
, stderr:
  left: Some(0)
 right: Some(2)

---- dry_run_json_diagnostics_all_carry_rendered_text stdout ----
thread 'dry_run_json_diagnostics_all_carry_rendered_text' panicked: called `Option::unwrap()` on a `None` value

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```

This is exactly bug A as described: a broken-regex profile produced an **empty report and exit
0** (`{"files":[],"batch_diagnostics":[],"suggestions":[]}`, exit `Some(0)`), silent success on a
broken profile.

### Green (after implementation)

```
running 3 tests
test dry_run_surfaces_config_time_invalid_regex ... ok
test dry_run_plans_a_single_file ... ok
test dry_run_json_diagnostics_all_carry_rendered_text ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

Manual confirmation of the same broken-regex profile, human mode:

```
$ muxsmith dry-run bad.yaml --source $DIR
[error] input.pattern: Invalid regular expression: regex parse error: S(?<s>\d{2}E(?<e>\d{2}) ^ error: unclosed group
EXIT=2
```

Manual confirmation of `--json` on a profile mixing all three diagnostic categories (config-time
`UnknownProperty`, batch-level `IgnoredFile`, per-file `MissingTrack` x2):

```json
{
    "batch_diagnostics": [
        { "code": "ignored-file", "rendered": "[info] input.pattern: File matches the extension list but not the input pattern.", ... }
    ],
    "config_diagnostics": [
        { "code": "unknown-property", "rendered": "[error] tracks[2].match.exact.bogus_property: Unknown property \"bogus_property\". It is not part of the mkvmerge identification model.", ... }
    ],
    "files": [
        { "diagnostics": [
            { "code": "missing-track", "rendered": "[error] tracks[1].match: No track matches this non-optional rule.", ... },
            { "code": "missing-track", "rendered": "[error] tracks[2].match: No track matches this non-optional rule.", ... }
          ], "identifier": "S01E01", "plan": null, ... }
    ],
    "suggestions": []
}
```

Every diagnostic object across all three categories carries a non-empty `rendered` string.

## Tests added

`crates/muxsmith-cli/tests/dry_run_cli.rs` (both mkvmerge-gated, following the existing
`have_mkvmerge()` self-skip style; mkvmerge is installed in this environment so both actually
ran, not skipped):

- `dry_run_surfaces_config_time_invalid_regex`: a profile with an unbalanced-paren
  `input.pattern` regex must produce a `config_diagnostics` array containing an `invalid-regex`
  entry, and the process must exit 2 (not the prior empty-report/exit-0 behavior).
- `dry_run_json_diagnostics_all_carry_rendered_text`: a profile engineered to produce all three
  diagnostic categories in one run (config-time `UnknownProperty` via a bogus match property,
  batch-level `IgnoredFile` via a non-matching extra file, per-file `MissingTrack` via an
  unsatisfiable video rule) asserts every diagnostic object, across `config_diagnostics`,
  `batch_diagnostics`, and every file's `diagnostics`, has a non-empty `rendered` string field.

## Commands run

- `cargo test -p muxsmith-cli --test dry_run_cli` - red, then green (shown above).
- `cargo test --workspace` - all green (every crate, unit + integration + doc tests), no
  regressions in the pre-existing suite (`cli_validate`, `catalog_completeness`,
  `validate_structure`, `validate_semantics`, `validate_hardening`, `suggestions`, `codegen`,
  planner/matcher/discovery unit tests, etc.).
- `cargo fmt --all --check` - clean (one pass of `cargo fmt --all` was needed first to wrap two
  lines rustfmt considered too long; both files re-verified clean after).
- `cargo clippy --workspace --all-targets -- -D warnings` - clean, no warnings.

## Concerns

- The mkvmerge-not-found early-return path does not surface config diagnostics (see "Design
  decisions" above); this is an explicit, judgment-call scope narrowing, not an oversight. No
  test exercises that path since mkvmerge is present on this machine and the task's tests are
  mkvmerge-gated by design; if a future environment without mkvmerge needs config diagnostics
  surfaced there too, that is a small follow-up, not a hidden gap in this fix.
- `config_diagnostics` as a new top-level JSON key is a shape change to dry-run's `--json`
  output (previously the raw serialized `Batch`, now a bespoke envelope). Any external consumer
  of the old raw shape (files/batch_diagnostics/suggestions only, no rendered fields) would need
  to adjust; this is inherent to bug F's fix (spec 5.2 requires the rendered field) and was called
  for by the task, not incidental.

## Follow-up: independent review findings, fixed

An independent review of the above flagged two issues. Both are now fixed in
`crates/muxsmith-cli/src/commands/dry_run.rs` / `crates/muxsmith-cli/tests/dry_run_cli.rs`.

### Finding 1 (Important) - config diagnostics dropped on the mkvmerge-not-found path

Confirmed the review's diagnosis exactly: `config_diags` was computed but the
`Mkvmerge::locate()` `Err` branch did `eprintln!(mkvmerge-not-found); return 2;` without ever
looking at `config_diags`, and in `--json` mode that branch printed plain text to stderr, so
`--json` produced *no* JSON on stdout at all (an empty regex-broken profile with mkvmerge hidden
gave `stdout: ""`, `stderr: "mkvmerge was not found..."`, exit 2 - not a JSON document). That
directly violates spec 5.5 ("dry-run is a strict superset of validate, never a subset"): a
regex-broken profile still reports `invalid-regex` from `validate`, but previously dry-run
reported nothing but the mkvmerge failure once mkvmerge was absent.

**Fix** (`dry_run.rs`, the `Mkvmerge::locate()` `Err` arm):
- Human mode: print every `config_diags` entry via `renderer.diagnostic()` (identical to the
  normal-path rendering), *then* the existing `mkvmerge-not-found` message to stderr. Exit stays 2.
- `--json` mode: emit a new, valid JSON document via a new `config_only_json()` helper instead of
  calling `eprintln!` with plain text:
  ```json
  {
    "config_diagnostics": [ { ...same shape as the normal path, incl. "rendered"... } ],
    "files": [],
    "batch_diagnostics": [],
    "suggestions": [],
    "mkvmerge_found": false
  }
  ```
  `files`/`batch_diagnostics`/`suggestions` are empty because planning never ran (mkvmerge is
  required for identification); `mkvmerge_found: false` gives JSON consumers a way to distinguish
  "mkvmerge missing" from any other error-severity dry-run report, since exit code 2 alone does
  not. Reused the existing `rendered_diags()` helper, so the config diagnostic shape (including
  `rendered`) is byte-for-byte identical to the normal path's `config_diagnostics` entries.
- Corrected the `run()` doc comment: it previously said unconditionally "folds both diagnostic
  sets into one report; the exit code reflects the worst severity across all of them", which
  overstates this path (no batch diagnostics exist to fold in, and the exit code is the
  mkvmerge-not-found failure outright, not a severity computation). Added an explicit exception
  clause.
- The `mkvmerge-query-failed` path (`list_languages()` failing after `locate()` succeeds) has the
  literal same defect (config diagnostics dropped, plain text under `--json`) but was **not**
  touched: the finding's text scopes the fix to `Mkvmerge::locate()` specifically, and fixing an
  unscoped second path risked doing more than was asked. Flagging it here as a candidate
  follow-up, not a hidden gap.

**Test coverage for Finding 1**: contrary to the original report's assumption ("hard to exercise
where mkvmerge IS installed"), the branch turned out to be reliably testable without faking
anything: `Mkvmerge::locate()` calls `Command::new("mkvmerge")` unqualified, which resolves via
the *child process's* `PATH`. Overriding just `PATH` (via `assert_cmd`'s `.env("PATH", ...)`) to a
fresh empty temp directory when spawning the `muxsmith` binary under test forces a real OS-level
"executable not found" spawn error, which `Mkvmerge::locate()` genuinely maps to
`RuntimeError::NotFound` - the real, unmodified code path, not a stub. Verified by hand first
(`env -i PATH=/tmp/emptybin ./target/debug/muxsmith dry-run ...`) before writing it as a test.
Added two tests, neither gated by `have_mkvmerge()` (they simulate absence regardless of whether
the test machine actually has mkvmerge installed):
- `dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing`: asserts exit 2, a valid JSON
  document, `config_diagnostics` containing `invalid-regex` with non-empty `rendered`,
  `files`/`batch_diagnostics`/`suggestions` all empty, `mkvmerge_found == false`.
- `dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing`: asserts exit 2, the rendered
  config diagnostic text present on stdout, the mkvmerge-not-found message present on stderr.

### Finding 2 (Minor) - test asserted aggregate, not per-category

`dry_run_json_diagnostics_all_carry_rendered_text` asserted `all_diags.len() >= 3` and non-empty
`rendered` on every entry, but never asserted `config_diagnostics` specifically was non-empty - a
regression that zeroed out `config_diagnostics` while batch/per-file counts still summed to >= 3
would have passed silently.

**Fix**: added
`assert!(!report["config_diagnostics"].as_array().unwrap().is_empty(), ...)` to that test,
directly after the existing aggregate-count assertion. This assertion already passes against the
current (already-fixed) `dry_run.rs`; its value is purely as a regression guard against a future
change that reintroduces the aggregate-only blind spot.

### Commands run (this follow-up)

Red (before the Finding-1 fix, tests added first per TDD):
```
$ cargo test -p muxsmith-cli --test dry_run_cli
running 5 tests
test dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing ... FAILED
test dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing ... FAILED
test dry_run_surfaces_config_time_invalid_regex ... ok
test dry_run_plans_a_single_file ... ok
test dry_run_json_diagnostics_all_carry_rendered_text ... ok
test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

---- dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing ----
json report: EOF while parsing a value at line 1 column 0, stderr: mkvmerge was not found on PATH...
---- dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing ----
expected the config-time diagnostic in stdout, got stdout: , stderr: mkvmerge was not found on PATH...
```

Green (after the fix):
```
$ cargo test -p muxsmith-cli --test dry_run_cli
running 5 tests
test dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing ... ok
test dry_run_json_surfaces_config_diagnostics_when_mkvmerge_missing ... ok
test dry_run_surfaces_config_time_invalid_regex ... ok
test dry_run_json_diagnostics_all_carry_rendered_text ... ok
test dry_run_plans_a_single_file ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

Full workspace, formatting, and lints:
```
$ cargo test --workspace
... (every crate, unit + integration + doc tests) ...
test result: ok. <all suites 0 failed>
```
No `FAILED`, `error[`, or `panicked` anywhere in the full `cargo test --workspace` output (grepped
to confirm).

```
$ cargo fmt --all --check
(clean, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
(clean, exit 0)
```

Manual confirmation of both output modes with mkvmerge forced absent via `PATH` override, same
broken-regex profile as the automated tests:
```
$ env -i PATH=/tmp/emptybin ./target/debug/muxsmith dry-run bad.yaml --source $DIR --json
{"batch_diagnostics":[],"config_diagnostics":[{"code":"invalid-regex","config_path":"input.pattern",
"params":{"detail":"regex parse error: ... unclosed group"},
"rendered":"[error] input.pattern: Invalid regular expression: regex parse error: ... unclosed group",
"severity":"error"}],"files":[],"mkvmerge_found":false,"suggestions":[]}
EXIT=2

$ env -i PATH=/tmp/emptybin ./target/debug/muxsmith dry-run bad.yaml --source $DIR
[error] input.pattern: Invalid regular expression: regex parse error: ... unclosed group
EXIT=2
stderr: mkvmerge was not found on PATH. Install MKVToolNix or set the mkvmerge path.
```

### Concerns (this follow-up)

- `mkvmerge-query-failed` (the `list_languages()` failure path) has the identical structural
  defect as Finding 1 but is out of the finding's stated scope; left unfixed, flagged as a
  candidate follow-up.
- `mkvmerge_found` is a new top-level JSON key added only to the not-found-path report, not to the
  normal-path `batch_json()` output. Consumers checking for its presence/absence rather than its
  value on the success path would need to know it is only emitted on the failure path; this was a
  deliberate minimal-diff choice (adding it unconditionally to both paths was not requested and
  would touch working code beyond the finding's scope).
