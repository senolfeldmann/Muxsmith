# Task 8 report: `muxsmith run` subcommand

## What was implemented

- `crates/muxsmith-cli/src/cli.rs`: `Cmd::Run` clap variant, exactly the field list the brief's Step 1
  specifies (`profile`, `--source`, `--output`, `--on-collision` reusing `CollisionArg`, `--jobs`
  default 1, `--fail-fast`, `--json`, `--locale`). Every field carries a doc comment (`#![deny(missing_docs)]`
  applies to `Cmd`'s variant fields since `Cmd` is `pub`; the brief's own snippet only commented `jobs`/
  `fail_fast`, so the rest were filled in mirroring `DryRun`'s sibling fields).
- `crates/muxsmith-cli/src/main.rs`: dispatch arm for `Cmd::Run`, mapping `on_collision` through
  `CollisionArg::policy` exactly like `DryRun`'s arm.
- `crates/muxsmith-cli/src/commands/run.rs` (new): the full flow.
  - Re-plans from scratch through `plan_batch` (spec 5.5 level 3), identical to `dry_run.rs` through that
    point, then builds `Vec<JobSpec>` from `batch.files` via `filter_map(|f| f.plan.as_ref())` + `command(p)`
    exactly as the brief's Step 2 snippet specifies.
  - Prints the planning report in dry-run's human format FIRST (via the now-shared `print_batch_human`),
    before touching the queue.
  - Empty `specs` (nothing plans cleanly) folds and returns via `diag_exit_code`, never touching the queue
    - verified live (see TDD evidence below).
  - Runs the queue via `LiveSpawner { mkvmerge: mkv.path().into() }` on a `std::thread::scope`d worker thread
    while the calling thread drains the `mpsc::Receiver` concurrently, rendering milestone lines as jobs
    progress (not only after the whole batch finishes). The queue thread owns the sole `Sender` (moved in via
    a `move` closure), so it drops when that thread's closure returns, ending the receiver loop
    deterministically - no explicit `drop` needed on the caller's side.
  - `MilestoneState`: per-job-index tracking of the highest 25/50/75 threshold already printed. A `Progress`
    event renders one line per **newly crossed** threshold, in ascending order (so a single coarse jump, e.g.
    straight from 0% to 90%, still surfaces all three milestone lines rather than silently skipping them);
    the printed `$percent` is the threshold itself, not the raw reported value.
  - Terminal-state lines (`run-job-ok/-warning/-failed/-cancelled`) and the final `run-summary` line, folded
    via `job_exit_code` (2 any Failed / 1 any Warning / else 0) and combined with `diag_exit_code` via
    `std::cmp::max`.
  - Cancel-flag check (`if cancel.load(...) { return 130; }`) sits **after** the summary print, matching the
    exact ordering Task 10's own brief snippet implies (`/* summary already printed */`), so Task 10 can drop
    its two-line guard in without restructuring anything here. The flag is a plain `Arc<AtomicBool>` that
    never flips in this task's scope; the branch is structurally present but currently unreachable (confirmed
    by manual runs below always exiting 0/1/2, never 130).
- `crates/muxsmith-cli/src/commands/mod.rs`: extracted `all_diags`, `diag_exit_code` (was `dry_run::exit_code`),
  and `print_batch_human` out of `dry_run.rs` into shared `pub(crate)` functions, per the brief's explicit
  invitation ("extract shared pieces into `commands/mod.rs` if cleaner - reviewer judges"). `dry_run.rs` now
  calls these instead of defining its own copies; its behavior and its own tests are unchanged (verified: all
  `dry_run_cli.rs` tests still pass).
- `locales/en/cli.ftl`: the 8 new keys, copied verbatim from the brief (`run-job-start`, `-progress`,
  `-notice`, `-ok`, `-warning`, `-failed`, `-cancelled`, `run-summary`).
- `crates/muxsmith-cli/tests/run_cli.rs` (new): 3 CLI-level tests for the planning-failure paths (see below).

## What was explicitly left for Tasks 9/10/11

- **Task 9 (`--json` final document):** `--json` mode in `run.rs` currently just suppresses all human lines
  (planning report AND milestone/summary lines) and returns the same numeric exit code; it does **not** build
  the `{config_diagnostics, files, batch_diagnostics, suggestions, jobs, summary}` document the brief's Task 9
  section specifies. Two `TODO(Task 9)` comments mark the exact spots (the mkvmerge-not-found early return,
  and the empty-specs early return) where that document still needs to be emitted. NDJSON streaming
  (`--json-events`) was not touched at all, per the plan's explicit v1.x deferral.
- **Task 10 (SIGINT):** no `ctrlc` dependency, no signal handler installed. `cancel` is constructed locally in
  `run::run` as `Arc::new(AtomicBool::new(false))` and never flipped; the fold logic that honors it (`if
  cancel.load(...) { return 130; }`, placed after the summary print) is already in place and was written to
  match the exact snippet in Task 10's own brief section, so Task 10 should only need to insert its handler
  installation before the queue starts, plus swap the local `AtomicBool::new(false)` for a shared one it can
  set from the signal handler.
- **Task 11 (gated e2e):** no `run_live.rs`. The full execute path (a real mux, exit 0 from actual `Ok`
  outcomes, `--jobs N` concurrency, `--on-collision skip` rerun semantics) is that task's job. I did do manual
  (non-committed, non-gate) smoke runs of the real binary to sanity-check the implementation end-to-end - see
  below - but no e2e test file was added.

## Design decisions not fully pinned down by the brief

- **1-based job index in rendered lines** (`[1/3]` not `[0/3]`): the brief's Fluent key text uses `{ $index }`
  without specifying the base. I chose 1-based as the more human-natural convention for a progress display.
  Low-risk, easily flipped in review if the reviewer wants 0-based to match `JobEvent`'s own indexing.
- **`Progress` events render the crossed *threshold* value, not the raw reported percent** (e.g. a jump from
  0% to 33% still prints "... 25%", not "... 33%"). This is what makes "print at 25/50/75 threshold crossings"
  a deterministic, testable pure function; the alternative (print the raw percent, gated by threshold
  crossing) is also defensible but less predictable for the milestone lines' purpose as fixed checkpoints.
- **A `Progress` event that jumps past several thresholds in one step renders one line per threshold crossed**,
  in ascending order, rather than only the highest one reached. Chosen so a long mux with sparse `--gui-mode`
  progress reports (or a fake/scripted spawner in a future test) never silently skips a milestone the user
  would otherwise expect to see.
- **Failed job with no exit code** (spawn failure, or theoretically a kill outside the cancel flag) renders
  `"... failed (exit n/a)"` rather than omitting the parenthetical or crashing on `Option::unwrap`.
- **Duration formatting**: `duration_ms as f64 / 1000.0` at one decimal place (`"{:.1}"`, e.g. `"1.5s"`). No
  existing precedent for seconds-formatting was found elsewhere in the codebase; this is a fresh, minimal
  choice.
- **`run-job-warning`'s `$count` has no pluralization** ("1 warnings" is grammatically off but matches the
  brief's fixed template literally; no Fluent plural-form branching was introduced since the brief locked the
  exact text).

None of these are requirements-shaping (they're rendering minutiae within an otherwise fully-specified flow),
so I made the call rather than escalating; flagged here for the reviewer to override if any reading differs
from what was intended.

## What was tested and results

`cargo test --workspace`: **249 tests, 0 failed** across the whole workspace (25 test-result blocks, all
`0 failed`), including:
- 18 `muxsmith-cli` lib unit tests (13 new, in `commands::run::tests`; 5 pre-existing `i18n` tests unchanged).
- 3 new `run_cli.rs` integration tests, all passing.
- 6 pre-existing `dry_run_cli.rs` tests, still passing after the `all_diags`/`diag_exit_code`/
  `print_batch_human` extraction into `commands/mod.rs`.
- 1 `catalog_completeness` test, still passing (it only iterates `DiagCode::ALL`, not `cli.ftl`'s general
  message keys, so it does not directly gate the 8 new `run-*` keys - the brief's Step 6 note "same guard as
  always" already anticipated this; the keys were added because the renderer's `msg()` falls back to the raw
  id on a miss, and every rendered line in the milestone tests asserts on the actual rendered text, not the
  raw key, which would fail if any key were missing or misspelled).

### Milestone unit tests (`commands::run::tests`, 13 tests)

Covers: only-3-lines-at-25/50/75 across a realistic percent sequence; a single event that jumps past all
three thresholds at once (each still renders, in order, and a further 100% event renders nothing more);
independent tracking per job index; no re-print on a repeated or regressing percent; `Started` renders 1-based
`[index/total]` + output + "start"; `Warning`/`Error` both render `run-job-notice` with their text;
`Finished` for all four `JobState`s (`Ok` with duration, `Warning` with count+duration, `Failed` with exit
code, `Failed` with `exit_code: None` falling back to "n/a", `Cancelled` with neither); `job_exit_code`'s
worst-of fold across Ok/Warning/Failed/Cancelled combinations; `render_summary`'s exact count line.

### CLI planning-error tests (`run_cli.rs`, 3 tests)

- `missing_profile_file_exits_two_before_any_planning`: `load::from_file` failure, the very first branch -
  exits 2, no job line in stdout. Fully hermetic (no mkvmerge dependency).
- `bad_regex_profile_exits_two_without_executing_a_job`: an uncompilable `input.pattern` regex leaves
  `discovery::scan_primaries` returning zero primaries (confirmed by reading `discovery.rs`:60-64, which
  explicitly returns `(Vec::new(), diags)` on a regex-compile error, relying on `validate` having already
  reported it) -> `batch.files` is empty -> `specs` is empty -> the empty-specs early return fires, folding
  `config_diags`'s `invalid-regex` (error-severity) to exit 2, without ever building a `JobSpec` or calling
  `run_queue`. Not gated on `have_mkvmerge()`: traced both branches (mkvmerge present vs. missing) and both
  print the same config diagnostic and exit 2 without a job line, so the assertions hold in either
  environment.
- `bad_regex_profile_with_missing_mkvmerge_exits_two_without_executing_a_job`: same profile, mkvmerge forced
  missing via a `PATH` override to an empty temp dir, hermetic regardless of the test machine's mkvmerge
  situation - exercises the *other* early-return branch specifically (confirmed distinct from the above by
  the RED run below, where breaking the empty-specs branch only failed the mkvmerge-present variant, not this
  one).

## TDD evidence

### RED 1 - milestone thresholding

Command: temporarily removed the `self.last_milestone[*index] < threshold` guard (and the corresponding
update) from `MilestoneState::render`'s `Progress` arm, then ran
`cargo test -p muxsmith-cli --lib commands::run::tests -- --nocapture`.

Result: 3 of 13 tests failed as expected -
`a_jump_past_several_thresholds_renders_each_one_in_order` (kept re-emitting all 3 lines for every event
instead of nothing on the second call), `repeated_or_regressing_percent_does_not_reprint` (`second.is_empty()`
failed - a repeated 25% re-printed), and `progress_prints_only_at_25_50_75_crossings` (18 lines instead of 3 -
every threshold re-fired on every subsequent percent >= it). This is exactly the defect class the brief's
"track last-milestone per index" requirement guards against, so the tests catch it precisely.

Reverted; re-ran the same command clean (0 failed).

### RED 2 - planning-error path (empty specs never executes)

Command: temporarily replaced `return diag_exit_code(&config_diags, &batch);` in the empty-specs branch with
a hardcoded `return 0;`, then ran `cargo test -p muxsmith-cli --test run_cli -- --nocapture`.

Result: `bad_regex_profile_exits_two_without_executing_a_job` failed (`left: Some(0), right: Some(2)`), while
`bad_regex_profile_with_missing_mkvmerge_exits_two_without_executing_a_job` and
`missing_profile_file_exits_two_before_any_planning` still passed - confirming the first test specifically
exercises the empty-specs fold path (distinct from the other two, which exercise the load-failure and
mkvmerge-missing branches respectively) and that the fold's correctness is what the test actually pins down.

Reverted; re-ran clean (3 passed).

### GREEN - full gate

```
cargo test --workspace     -> 25/25 test-result blocks "ok", 0 failed
cargo fmt --all --check    -> clean (after one `cargo fmt --all` pass to apply rustfmt's own
                               multi-line-call formatting to run.rs, which I had written manually)
cargo clippy --workspace --all-targets -- -D warnings  -> clean, 0 warnings
cargo deny check           -> advisories ok, bans ok, licenses ok, sources ok
```

## Manual verification (not part of the gate, sanity only)

Built two real fixture MKVs via mkvmerge, ran the actual `muxsmith run` binary against them:

```
$ muxsmith run p.yaml --source $DIR --output $DIR/out
<dry-run-shaped planning report for both files>
[1/2] .../out/Show.S01E01.mkv ... start
[1/2] .../out/Show.S01E01.mkv ... 25%
[1/2] .../out/Show.S01E01.mkv ... 50%
[1/2] .../out/Show.S01E01.mkv ... 75%
[1/2] .../out/Show.S01E01.mkv ... ok (0.0s)
[2/2] .../out/Show.S01E02.mkv ... start
[2/2] .../out/Show.S01E02.mkv ... 25%
[2/2] .../out/Show.S01E02.mkv ... 50%
[2/2] .../out/Show.S01E02.mkv ... 75%
[2/2] .../out/Show.S01E02.mkv ... ok (0.0s)
2 ok, 0 warning, 0 failed, 0 cancelled
EXIT=0
```
Both output files existed on disk afterward. Also checked `--json` (suppresses all human lines, prints
nothing yet as documented, exit 0, mux still runs and produces correct output) and `--jobs 2` (runs fine).

## Files changed

- `crates/muxsmith-cli/src/cli.rs` - `Cmd::Run` variant.
- `crates/muxsmith-cli/src/main.rs` - dispatch arm.
- `crates/muxsmith-cli/src/commands/run.rs` - new: the whole `run` flow, `MilestoneState`, exit folding, tests.
- `crates/muxsmith-cli/src/commands/mod.rs` - new shared `all_diags`/`diag_exit_code`/`print_batch_human`.
- `crates/muxsmith-cli/src/commands/dry_run.rs` - now calls the shared helpers instead of defining its own;
  dropped its now-unused `Severity` import.
- `locales/en/cli.ftl` - 8 new `run-*` keys.
- `crates/muxsmith-cli/tests/run_cli.rs` - new: 3 planning-failure CLI tests.

## Self-review findings

- **Completeness:** every brief step (1-6) implemented; all 8 Fluent keys present, verbatim-diffed against
  the brief's snippet (see `grep -n "^run-" locales/en/cli.ftl` in this session - byte-for-byte match).
- **Quality:** doc comments added throughout matching the crate's existing convention (module-level `//!`,
  function/struct-level `///`); `#[allow(clippy::too_many_arguments)]` added and justified (8 params, over
  clippy's default threshold of 7 - matches the brief's own 8-parameter `run::run` signature, not something I
  chose to bloat).
- **Discipline:** no NDJSON, no `--json` final document, no `ctrlc`/SIGINT handling - all confirmed absent by
  grep and by the TODO markers left in place. The scoped-thread draining design was my own call within the
  brief's explicit instruction ("draining events on the main thread"); no plan/spec deviation.
- **Testing:** milestone unit tests cover the threshold edge cases explicitly (multi-threshold jump, repeat,
  regression, per-index independence) beyond the bare minimum; planning-error CLI test present and confirmed
  via RED/GREEN to catch the exact regression class it's meant to guard; output pristine (no stray debug
  prints, no leftover TODO-marker code paths beyond the two documented `TODO(Task 9)` comments).

## Issues or concerns

None blocking. The two open judgment calls worth a reviewer's explicit sign-off are the 1-based index
convention and the "print the threshold value, not the raw percent" choice for `run-job-progress` - both are
cheap to flip if the reviewer's reading of the brief differs.

## Fluent pluralization fix (Şenol-approved amendment)

Şenol approved (2026-07-10) amending the locked `run-job-warning` text flagged in the "Self-review findings"
section above ("1 warnings" is grammatically off): pluralize the warning count via a Fluent plural selector
instead of the fixed `{ $count } warnings` template.

### What changed

- `locales/en/cli.ftl`: `run-job-warning`'s count phrase is now a Fluent select expression:
  `{ $count -> [one] 1 warning *[other] { $count } warnings }`, embedded mid-pattern (parenthetical), following
  the multiline variant-indent style already used by `invalid-template` in `diagnostics.ftl` (4-space `[key]`,
  3-space `*[key]` so the `[` aligns, closing `}` at column 0, pattern continues on that line).
- `crates/muxsmith-cli/src/commands/run.rs`: the `JobState::Warning` arm now calls the new
  `renderer.msg_with_count(...)` instead of `renderer.msg(...)`, passing `outcome.warnings.len()` as a `usize`
  directly instead of pre-stringifying it via `.to_string()`.
- `crates/muxsmith-cli/src/i18n.rs`: added `Renderer::msg_with_count(id, args, count_key, count: usize)`,
  additive and non-breaking. It builds `FluentArgs` exactly like `msg` for the base `args`, then also
  `fargs.set(count_key, count)` with `count` as a bare `usize` (relies on `fluent_bundle`'s
  `From<usize> for FluentValue` impl, which produces `FluentValue::Number`, not `FluentValue::String`).
  `msg`'s and `msg_with_count`'s shared tail (message/pattern lookup, `format_pattern`, missing-id fallback)
  was factored into a private `render(&self, id, fargs: FluentArgs)` so neither duplicates that logic. `msg`'s
  public signature and behavior are unchanged; every other existing call site (`validate.rs`, `mod.rs`,
  `identify.rs`, `i18n.rs`'s own `diagnostic()`) is untouched.
- **Scope note:** the task's stated file scope was `cli.ftl`, `run.rs`, and this memo line. Touching
  `i18n.rs` was not in that list but proved unavoidable: `Renderer::msg`'s signature is hard-typed
  `args: &[(&str, &str)]`, so there is no way to route a real `usize` through it without changing a type
  somewhere in `i18n.rs` - "convert the call site to pass a numeric type" (as the task specified) cannot be
  satisfied by a `run.rs`-only edit in a statically typed language. Kept the change additive (new method,
  shared private helper) so no other call site's behavior or types were affected. Flagging this deviation
  explicitly rather than silently deciding it was in-scope.

### Why $count needed to change type (verification, not assumption)

Traced `fluent-bundle` 0.16.0's selector-matching code (`src/resolver/expression.rs` `Select` arm calls
`key.matches(&selector, scope)`, `src/types/mod.rs` `FluentValue::matches`): a variant key literal like `one`
parses as `FluentValue::String("one")`; it only resolves to a CLDR plural category (via `intl_pluralrules`)
in the `(FluentValue::String(a), FluentValue::Number(b))` match arm. When the runtime selector is itself a
`FluentValue::String` (e.g. `"1"`, what `count.to_string().as_str()` produced through the old `msg()` call),
the match falls to `(String, String)` plain equality (`"one" == "1"` -> false) and then the catch-all
`_ => false`, so `[one]` never matches and the pattern always falls through to `*[other]` regardless of the
count's value - confirming the task brief's warning exactly. `FluentArgs::set` is generic over
`V: Into<FluentValue<'args>>`, and `fluent-bundle` provides `From<usize> for FluentNumber`/`FluentValue`
(`src/types/number.rs`, `from_num!` macro covering `usize` among others), so passing the raw `usize` through
`msg_with_count` produces a `FluentValue::Number` and the selector resolves correctly. No isolate-mark
(FSI/PDI) concern applies here: the renderer already sets `use_isolating(false)` (`i18n.rs` line 31,
pre-existing), consistent with every other rendered line in this file.

### RED

Added `finished_warning_with_exactly_one_warning_renders_singular` to `commands::run::tests` (1-warning
outcome), asserting the rendered line contains `"(1 warning,"` and does not contain `"1 warnings"`. Ran
`cargo test --package muxsmith-cli --lib commands::run::tests::finished_warning_with_exactly_one_warning_renders_singular`
against the pre-fix code (ftl unchanged, call site still using `renderer.msg` with a stringified count):

```
thread '...finished_warning_with_exactly_one_warning_renders_singular' panicked:
expected singular '1 warning' in: [1/1] a.mkv ... warning (1 warnings, 2.0s)
test result: FAILED. 0 passed; 1 failed
```

Confirms the bug reproduces exactly as described, and that the assertion is precise enough to catch it
(`"(1 warning,"` is not a substring of `"(1 warnings,"` since `warning` is immediately followed by `,` in the
singular form vs. `s` in the plural form).

No other existing test asserted the literal `"1 warnings"` text (grepped the repo); the only other 1-warning
outcome in the suite, `run_json_document_maps_outcomes_to_indexed_job_entries_and_counts_the_summary`, asserts
the JSON `warnings` array shape, not rendered human text, so it needed no change.

### GREEN

After applying the ftl change and the `msg_with_count` call-site conversion, the same focused test and the
full `commands::run::tests` module (16 tests, including the pre-existing `finished_warning_renders_count_and_duration`
with 2 warnings, still asserting `"2 warnings"`) and `i18n::tests` (5 tests, unchanged) all pass:

```
cargo test --package muxsmith-cli --lib commands::run::tests -> 16 passed; 0 failed
cargo test --package muxsmith-cli --lib i18n::tests          -> 5 passed; 0 failed
```

### Full gate

```
cargo test --workspace                                  -> 25/25 test-result blocks ok, 0 failed
cargo fmt --all --check                                  -> clean
cargo clippy --workspace --all-targets -- -D warnings     -> clean, 0 warnings
cargo deny check                                          -> advisories ok, bans ok, licenses ok, sources ok
```

### Memo

Added one bullet to the D15 section of `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`:
"Amendment 2026-07-10 (Şenol): run-job-warning pluralizes the warning count via a Fluent plural selector
(plan's locked text rendered '1 warnings')."

### Commit

One commit, `fix(cli): pluralize run-job-warning count via Fluent plural selector`, covering all four files
(`locales/en/cli.ftl`, `crates/muxsmith-cli/src/commands/run.rs`, `crates/muxsmith-cli/src/i18n.rs`, the
design-decisions memo). Not pushed. The pre-existing untracked `HANDOFF.md` in the worktree was left alone
(out of scope, unrelated to this task).
