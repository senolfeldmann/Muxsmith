# Task 8 Report: Empty-batch summary line, always-print (#8)

## What was implemented

`print_batch_human` (shared verbatim by `dry-run` and `run`, spec 5.5) now
always prints a trailing batch-summary line, unconditionally on the number
of files: `"{count} files matched (searched {root}, extensions {extensions})"`.
On a zero-file batch this is the previously-missing signal (human mode used
to print literally nothing and exit 0, a silent success channel while
`--json` already emitted a zeroed document). Info level, exit code
unchanged (still the worst-of diagnostic fold via `diag_exit_code`).

Confirmed by reading the code that no such summary line existed at all
before this change, for either the empty or non-empty case (not "only
printed when non-empty" as a shallower reading of the brief might suggest);
the ROADMAP.md "Empty-batch human output (D15 gap)" entry (lines 258-266)
carries the exact literal decision text quoted in the task brief and
confirms this is new functionality, not a widened existing conditional.

### Design decisions (both grounded in codebase precedent, not guessed)

- **Fluent key `dry-run-summary`**, not `run-empty-batch`: the shared
  function's existing message family (`dry-run-file`, `dry-run-assignment`,
  `dry-run-output`, `dry-run-suggestion`) is already prefixed `dry-run-`
  despite being printed by both `dry-run` and `run` (the function's own doc
  comment calls it "dry-run's human format"), so the new key follows that
  established naming convention rather than the brief's example name.
- **Footer placement** (printed after per-file lines, batch diagnostics,
  and suggestions), not a header. Mirrors the closest sibling precedent in
  the codebase: `validate.rs`'s `validate-summary`, which is also a
  trailing summary line following the same "human mode must always speak"
  requirement for a sibling command.
- **No plural selector** (`"1 files matched"` reads grammatically odd but
  is literal): matches the decision text's exact quoted wording ("0 files
  matched") and the codebase's own precedent of not plural-selecting count
  lines in Wave 1 (`run-summary` has the identical pattern, e.g. "1
  warning" only via a `[one]` selector there but bare elsewhere). Plan 5.5
  Task 19 ("plural selectors") is the designated place to retrofit CLDR
  selectors across the catalog; doing it piecemeal here would be
  inconsistent with that wave boundary.

## Files changed

- `crates/muxsmith-cli/src/commands/mod.rs`: `print_batch_human` gains
  `root: &Path, extensions: &[String]` parameters; prints the new
  `dry-run-summary` line unconditionally at the end of the function; doc
  comment updated.
- `crates/muxsmith-cli/src/commands/run.rs:153-158`: call site passes
  `&run_inputs.source, &profile.input.extensions`.
- `crates/muxsmith-cli/src/commands/dry_run.rs:122`: call site passes
  `&run.source, &profile.input.extensions`.
- `locales/en/cli.ftl`: new EN-only key
  `dry-run-summary = { $count } files matched (searched { $root }, extensions { $extensions })`
  (cross-task constraint C2: EN-only, a later task translates; no
  `locales/de/*.ftl` exists yet in this repo at all, so no parity gap was
  introduced).
- `crates/muxsmith-cli/tests/run_cli.rs`: new test
  `run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent`.
- `crates/muxsmith-cli/tests/dry_run_cli.rs`: new test
  `dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent`.

## TDD evidence

**RED** (both tests, before the implementation change):

```
$ cargo test -p muxsmith-cli --test run_cli run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent -- --nocapture
thread 'run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent' panicked at crates/muxsmith-cli/tests/run_cli.rs:361:5:
expected the zero-count batch summary line, got stdout:
test result: FAILED. 0 passed; 1 failed

$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent -- --nocapture
thread 'dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent' panicked at crates/muxsmith-cli/tests/dry_run_cli.rs:240:5:
expected the zero-count batch summary line, got stdout:
test result: FAILED. 0 passed; 1 failed
```

**GREEN** (after implementing `print_batch_human` + call-site changes +
catalog entry):

```
$ cargo test -p muxsmith-cli --test run_cli run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent -- --nocapture
test run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent ... ok

$ cargo test -p muxsmith-cli --test dry_run_cli dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent -- --nocapture
test dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent ... ok
```

**Manual verification of the non-empty case** (real mkvmerge, one fixture
file, `dry-run --source <dir> --output <dir>/out`):

```
/tmp/tmp.EeAm7flqm9/Show.S01E01.mkv (identifier: S01E01)
rule 0 -> track 0
output: /tmp/tmp.EeAm7flqm9/out/Show.S01E01.mkv
1 files matched (searched /tmp/tmp.EeAm7flqm9, extensions mkv)
EXIT=0
```

Confirms the same code path handles both the empty and non-empty case
uniformly (no special-casing), and that `count`/`root`/`extensions` render
correctly end to end.

## Gate results (full gate, from worktree root, all foreground)

- `cargo fmt --all --check`: clean (exit 0)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, zero
  warnings
- `cargo test --workspace`: all suites `ok`, zero failures (includes
  `run_cli.rs` 11/11 and `dry_run_cli.rs` 10/10, both gated on
  `have_mkvmerge()`; mkvmerge v100.0 was present on this machine so the new
  tests ran for real, not self-skipped)
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`
  (exit 0)
- `pnpm install --frozen-lockfile`: ran once (node_modules was missing in
  this worktree), lockfile unchanged
- `pnpm lint`: clean
- `pnpm build`: clean (`vue-tsc --noEmit && vite build`)
- `pnpm check:i18n`: `ok` (exit 0); pre-existing 12 "unused" warnings in
  `gui-*.ftl` are unrelated to this change (`cli.ftl` is excluded from this
  scan by design, confirmed by reading `scripts/check-i18n.mjs`)
- `pnpm test:e2e`: 3/3 Playwright specs pass

## Self-review findings

- Confirmed `print_batch_human` is `pub(crate)` with exactly two call
  sites (`run.rs`, `dry_run.rs`); no GUI/`src-tauri` consumer exists, so the
  signature change has no other blast radius.
- Confirmed the JSON path is untouched: `batch_document`/`run_document` in
  `crates/muxsmith-core/src/report/json.rs` were not modified; `--json`
  already emitted the zeroed `summary` document per the brief.
- Confirmed the new print call sits inside the existing `if !json { ... }`
  block in both commands, before any early return, so it fires on every
  human-mode invocation that reaches planning (including the
  `specs.is_empty()` early-return path in `run.rs`, which happens after
  this print already ran).
- Confirmed no other test in the workspace depended on `print_batch_human`'s
  previous silence on an empty batch or asserted an exact stdout line count
  for the non-empty human path (`cargo test --workspace` is the proof; no
  test needed adjustment).
- Confirmed no `locales/de/*.ftl` exists yet anywhere in the repo, so the
  EN-only key introduces no de-catalog parity gap for a later task to
  discover.

## Concerns

- None blocking. One minor, deliberate, and explicitly deferred: the
  message is not plural-correct for `count == 1` ("1 files matched"). This
  is intentional (see "Design decisions" above) and matches the literal
  decision text plus the codebase's existing Wave-1 non-pluralized
  precedent (`run-summary`); Task 19 is the designated place to retrofit
  CLDR plural selectors across the catalog, including this key.
- Task 10 (catalog param-drift guard + full-key coverage) will need a
  fixture entry for `dry-run-summary`'s three params (`count`, `root`,
  `extensions`) and must add it to the allowlist of directly-rendered keys;
  nothing required of this task beyond naming the key sensibly, per the
  brief's own note, but flagging it here so Task 10's implementer isn't
  surprised by an unlisted key.
