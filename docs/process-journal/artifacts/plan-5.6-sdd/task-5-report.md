# Task 5 report: CLI crate (Stream D)

Plan 5.6, pre-1.0 idiomacy fix wave. Worktree
`/home/senol/Git/Muxsmith/.worktrees/plan-5.6-d`, branch `plan-5.6-d`.

## Status: DONE

## Commits

- `0e8d048` `refactor(cli): mechanical idiomacy fixes (plan 5.6 task 5, stream D)`
  (items 1-6)
- `c877e4f` `i18n(cli): rename dry-run-summary to batch-summary (T8-m2)` (item 7)

Both commits are unsigned (`git -c commit.gpgsign=false commit`), explicitly
staged file-by-file (no `git add -A`), not pushed.

## Per-item results

1. **`commands/mod.rs:32` yagni** - `all_diags` inlined into its only caller
   `diag_exit_code`; the ordering-rationale sentence moved onto
   `diag_exit_code`'s doc comment. Verified `all_diags` had exactly one
   caller before inlining (grepped the crate; the only other `all_diags`
   hits were an unrelated local variable of the same name in
   `dry_run_cli.rs`).
2. **`commands/validate.rs:19-34` dup** - three sub-fixes:
   - `validate::run` now calls `commands::severity_sorted` instead of an
     inline `sort_by_key(Reverse(...))`. Since `severity_sorted` returns
     `Vec<&Diagnostic>` (borrows) but `report::json::rendered_diags` needs
     an owned `&[Diagnostic]` (verified: JSON output order is asserted by
     `cli_validate.rs`'s `json_output_is_machine_readable` test, "sorted
     output leads with an error"), the sorted borrows are cloned once into
     the owned `diagnostics: Vec<Diagnostic>` used by both output branches.
     `Diagnostic` already derives `Clone`; the list is small
     (config-diagnostic count) and validate is not a hot path, so this is
     not a behavior-relevant cost.
   - `report::json::rendered_diags` made `pub` (the one core visibility
     change in scope) and called directly, dropping the inline
     `to_value` + `v["rendered"]` map.
   - Extracted `pub(crate) fn severity_exit(Option<Severity>) -> i32` in
     `commands/mod.rs`, shared by `diag_exit_code` and `validate::run`,
     dropping the duplicated fold.
3. **`i18n.rs:45` dup** - `Renderer::msg` now delegates to
   `msg_with_counts(id, args, &[])` instead of duplicating the
   `FluentArgs`-building loop.
4. **`tests/cli_validate.rs:3` yagni** - deleted the dead `mod support;`
   (grepped: `cli_validate.rs` calls no `support::` helper) and removed
   both `#[allow(dead_code)]` from `tests/support/mod.rs`; the remaining
   consumers (`run_cli.rs`, `dry_run_cli.rs`) use
   `insta_settings_with_tmp` (which calls `insta_settings` internally), so
   neither function is dead code in either surviving test binary.
5. **`tests/run_cli.rs:498` + `tests/dry_run_cli.rs:576` dup** - confirmed
   byte-identical function bodies (differing only in doc comments),
   hoisted `fake_mkvmerge_that_fails_queries` into `tests/support/mod.rs`
   as `#[cfg(unix)] pub fn` with a merged doc comment; both files now call
   `support::fake_mkvmerge_that_fails_queries()`. Deleted run_cli's
   "kept local per this file's existing per-file-helper convention" note
   (obsolete once shared). Matches the `testing-support-helpers` house
   pattern (docs/conventions.yaml) and its 3x-duplication trigger.
6. **`tests/catalog_completeness.rs:453` yagni** - `fixture_args` and
   `allowlisted_cli_key_args` now return `Vec<(&'static str, &'static
   str)>` directly (every arm was already string-only); `render_and_find_leaks`
   takes that shape and passes pairs straight to `renderer.msg`;
   `string_pairs` and the `FluentValue` import are gone.
7. **Seed T8-m2 rename** - `dry-run-summary` -> `batch-summary` at all nine
   occurrences (re-verified via grep before editing): `locales/en/cli.ftl:23`
   + `locales/de/cli.ftl:28` (message text untouched, id only, same
   commit), `commands/mod.rs:129` (call site) + two test fn renames
   (`batch_summary_renders_the_singular_form_for_one_matched_file` /
   `..._plural_form_for_two_or_more_matched_files`),
   `catalog_completeness.rs` allowlist entry + fixture arm, one comment
   each in `run_cli.rs`/`dry_run_cli.rs`. Confirmed via grep that no
   `.snap` fixture carries the key name (they capture rendered prose, e.g.
   "1 file matched (searched ...)"). Deleted the resolved
   "dry-run-summary -> batch-summary rename (T8-m2)" clause from
   `docs/ROADMAP.md`'s NAMED INPUTS list (a comma/semicolon-separated
   prose enumeration shared by all six wave-1 streams; removed only this
   task's clause, left every other stream's pending item untouched).

## Gate results (nine parts, run twice: once per commit)

Both times, in the foreground, no background-run-plus-monitor:

1. `cargo fmt --all --check` - clean (one pass required `cargo fmt --all`
   first, to reflow two `vec![...]` arms in `catalog_completeness.rs` past
   rustfmt's line-length break point; re-checked clean after).
2. `cargo clippy --workspace --all-targets -- -D warnings` - clean, no
   warnings.
3. `cargo test --workspace` - all green both times (no failures, no
   ignored). Between the two commits the CLI's own suite briefly showed 2
   failures during a deliberate mid-flight, uncommitted state (see
   "Commit-splitting mechanics" below); never in a committed or gate-run
   state.
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - clean.
5. `cargo deny check` - exit 0, "advisories ok, bans ok, licenses ok,
   sources ok".
6. `pnpm lint` - clean (exit 0).
7. `pnpm build` - clean (vue-tsc + vite build succeed).
8. `pnpm check:i18n` - "ok" both times; the second run's en/de parity
   check is the closest thing to an automated confirmation that the
   `batch-summary` rename landed bilingually in the same commit (it
   parses `locales/en/cli.ftl` and `locales/de/cli.ftl` and checks key-set
   parity, cli.ftl deliberately included per its own header comment).
9. `pnpm test:e2e` - 6/6 Playwright specs pass both times.

## Commit-splitting mechanics (not itself a finding, noted for transparency)

The brief's two commit subjects (`refactor(cli): ...` /
`i18n(cli): rename ...`) implied items 1-6 and item 7 should land in
separate commits, but four files carry hunks from both groups
(`commands/mod.rs`, `catalog_completeness.rs`, `run_cli.rs`,
`dry_run_cli.rs`). Since all edits were applied to the working tree before
any commit existed this session, the split was done by: temporarily
reverting only the seven `batch-summary` occurrences in those four mixed
files back to `dry-run-summary`, staging and committing the refactor
group, then reapplying the seven-occurrence rename and re-running the
full nine-part gate before the second commit. The transient mixed
working-tree state in between (visible only in this session's tool log,
never committed) briefly failed 2 of the CLI's own tests, the expected
effect of a `.rs` file referencing a Fluent key the already-renamed
catalog no longer defines. Each commit is independently gate-clean:
verified by adding a temporary detached worktree at `0e8d048` (commit 1
alone) and running `cargo test -p muxsmith-cli -p muxsmith-core` there -
all green, confirming commit 1's `.rs` files and (untouched-by-that-commit)
`.ftl` files agree on the old key name - then removing the temporary
worktree. Commit 2 (`c877e4f`, current `HEAD`) is the state the full
nine-part gate ran against directly.

## Files changed (both commits combined)

- `crates/muxsmith-cli/src/commands/mod.rs`
- `crates/muxsmith-cli/src/commands/validate.rs`
- `crates/muxsmith-cli/src/i18n.rs`
- `crates/muxsmith-cli/tests/catalog_completeness.rs`
- `crates/muxsmith-cli/tests/cli_validate.rs`
- `crates/muxsmith-cli/tests/dry_run_cli.rs`
- `crates/muxsmith-cli/tests/run_cli.rs`
- `crates/muxsmith-cli/tests/support/mod.rs`
- `crates/muxsmith-core/src/report/json.rs`
- `docs/ROADMAP.md`
- `locales/de/cli.ftl`
- `locales/en/cli.ftl`

Matches the brief's exclusive file list exactly (verified via
`git diff --stat` against the pre-task commit); nothing in `planner.rs`,
`report/mod.rs`, `src-tauri`, the frontend, or the catalog header comments
was touched.

## Self-review

- **Completeness**: all 7 brief items implemented; nothing deferred or
  skipped.
- **Quality**: `severity_exit` and `severity_sorted` are the two shared
  primitives now used by both `diag_exit_code` and `validate::run`,
  matching the plan's intent of one fold/one sort, not two. The
  `fixture_args`/`allowlisted_cli_key_args` conversion to
  `Vec<(&'static str, &'static str)>` is a straight mechanical rewrite,
  match-arm-for-match-arm, preserving every fixture value verbatim;
  diffed old vs. new content line-by-line during construction to avoid
  transcription errors across ~60 match arms.
  the one point requiring a judgment call (not a pure mechanical
  transcription) is `validate.rs`'s `severity_sorted(...).into_iter().cloned().collect()`:
  `rendered_diags`'s signature (`&[Diagnostic]`, owned) can't accept
  `severity_sorted`'s `Vec<&Diagnostic>` output directly, and the JSON
  output's error-first order is test-asserted, so a clone was the
  least-invasive bridge without changing either function's signature.
  Flagging this since it's the one spot where "reuse X" and "call Y"
  don't compose for free.
- **Discipline**: nine-part gate run in full twice (once per commit),
  foreground only; explicit `git add <files...>` both times, no
  `git add -A`; commits unsigned via `-c commit.gpgsign=false`; nothing
  pushed.
- **Test output**: pristine both gate runs - no warnings from
  fmt/clippy/doc, no cargo-deny advisories, no lint errors, no failed or
  ignored tests in `cargo test --workspace` or `pnpm test:e2e`.

## Surfaced patterns / deviations (not silently resolved)

- **House pattern confirmed, not new**: item 5's hoist is a direct
  instance of the already-codified `testing-support-helpers` pattern
  (docs/conventions.yaml, "Flagged at 3x duplication... duplicating a
  helper within a crate is a defect"); no new pattern introduced, just
  applied.
- **`docs/ROADMAP.md` shared-paragraph risk**: the NAMED INPUTS line this
  task edited is one prose paragraph enumerating deferred items for all
  six wave-1 streams (T2-m1, T8-m2, T5-m2, T14-m1, ...). Each stream
  presumably edits the same paragraph to remove its own resolved clause;
  since these are six parallel worktrees, merging all six back to a
  shared branch/PR will very likely produce a text conflict on this one
  paragraph even though each stream's actual code changes are disjoint.
  Worth a heads-up to whoever integrates the six streams; not something
  this task could resolve unilaterally without touching another stream's
  scope.
- **`Diagnostic: Clone` used for the first time in the CLI's `validate`
  path** (see "Quality" above) to bridge `severity_sorted`'s borrow-based
  return type into `rendered_diags`'s owned-slice signature. Not a new
  dependency or pattern, just noting it as the one non-mechanical
  micro-decision in an otherwise verbatim-transcription task.
