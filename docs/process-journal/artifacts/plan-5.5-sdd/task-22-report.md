### Task 22 report: insta snapshots for CLI rendering (#2)

**Branch:** `plan55-t22` (worktree `.worktrees/t22`, based on post-T19/T20 master)

## 1. Redaction filter set (Step 1, load-bearing)

Two shared filters in `crates/muxsmith-cli/tests/support/mod.rs::insta_settings()`
(the `filters` cargo feature, plain-string regex redaction -- the right tool
for CLI stdout/stderr snapshots; `redactions` is for structured JSON/YAML
snapshots and does not apply here):

| Filter | Pattern | Why |
|---|---|---|
| mkvmerge version banner | `mkvmerge v\d+(?:\.\d+){1,3}[^\n]*` -> `mkvmerge v[VERSION]` | Defensive. Grepped the whole CLI crate: no current call site echoes `Mkvmerge::version()`'s raw `--version` output into rendered CLI text (the `mkvmerge-not-found`/`mkvmerge-query-failed` messages are fixed strings with no version param). But CI genuinely runs divergent mkvmerge builds per leg (Plan 5.5 Task 2: apt ships 97.0-1build1 on Linux, choco/brew ship 100.0.0 on Windows/macOS), and spec 10 explicitly accepts third-party error text (regex/serde/I-O) as a `detail` param -- a future `RuntimeError::TooOld` (carries the raw `--version` line, see `capability/runtime.rs:48`) surfacing through that path would silently diverge snapshots across legs without this filter already in place. |
| job duration | `\d+\.\d+s\b` -> `[N.Ns]` | `run-job-ok`/`-warning`/`-failed`'s `{ $seconds }` is always `format!("{:.1}", duration_ms as f64 / 1000.0)` (`commands/run.rs:452`) -- genuinely nondeterministic wall-clock time. Not exercised by the final snapshot set (see 4iv below) but load-bearing for any future job-milestone snapshot. |

A third, **per-test** filter in `insta_settings_with_tmp(path)`: an exact,
`regex::escape`d literal match on that test's own `TempDir` path ->
`[TMPDIR]`. Deliberately not a shared "looks like an absolute path" regex:
each temp root is only known to its own call site, and a generic path-shaped
pattern risks also swallowing unrelated content (e.g. a `config_path` value)
and masking a real bug. Verified stable: ran the four affected tests twice
(fresh random tempdir each run, `--test-threads=1`), `diff -r` on the two
`tests/snapshots/` trees was empty -- the redaction produces byte-identical
output regardless of the actual OS-assigned path.

Not filtered: config_paths (`tracks[0].match`, `input.pattern`) -- these are
schema-relative identifiers echoed by the `diagnostic-line` template, not
filesystem paths or translatable wording, and are stable across machines.

## 2. insta version (registry-verified)

`insta = { version = "=1.48.0", features = ["filters"] }` -- crates.io API
(`GET /api/v1/crates/insta`, 2026-07-12) reports `max_stable_version:
"1.48.0"`. `cargo-insta` 1.48.0 was already installed locally (`cargo insta
--version`), matching exactly. `regex = "1.12.4"` added as a second
dev-dependency (identical to the version muxsmith-core already pins;
resolves to the same tree entry, not a new crate) for `regex::escape` in the
tempdir filter.

Idiomacy verified against docs.rs/insta.rs, not assumed from training data:
`Settings::add_filter` requires the `filters` feature (not a default);
`Settings::bind(closure)` is the correct non-macro API for a `Settings`
built by a helper function (`with_settings!` wants filters written inline as
a literal `vec![...]`, awkward for a per-test dynamic tempdir path); the
default `INSTA_UPDATE=auto` resolves via `insta::utils::is_ci()`, confirmed
by reading `insta` 1.48.0's own source (`src/utils.rs`, via `gh api
repos/mitsuhiko/insta/contents/...`): checks `CI` env var first (any
non-empty, non-"0"/"false" value = CI), falls back to `TF_BUILD` (Azure
DevOps) if `CI` is unset.

## 3. Converted vs. kept inventory

Grepped every `contains(` across `crates/muxsmith-cli/tests/*.rs` (32 sites,
6 files) and classified each: **convert** if it pins human-authored prose
that a copy edit would break; **keep** if it checks exit codes, counts, JSON
key presence, a structural identifier (config_path, program name), or an
*absence* (which cannot be meaningfully snapshotted).

**Converted to `assert_snapshot!` (11 tests, 4 files):**

| File | Test | What |
|---|---|---|
| cli_validate.rs | `valid_profile_exits_zero_with_ok_message` | stdout: "Profile is valid." |
| cli_validate.rs | `invalid_profile_exits_two_and_renders_messages` | stdout: 3 combined diagnostics (was 3 separate `.and()` fragments) |
| cli_validate.rs | `warnings_only_exits_one` | stdout: overlap warning |
| run_cli.rs | `bad_regex_profile_exits_two_without_executing_a_job` | stdout, tempdir-redacted (see 4i) |
| run_cli.rs | `bad_regex_profile_with_missing_mkvmerge_exits_two_without_executing_a_job` | stdout |
| run_cli.rs | `run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent` | stdout, tempdir-redacted (was 3 separate substring checks) |
| run_cli.rs | `run_human_mode_surfaces_config_diagnostics_on_a_language_query_failure` | stderr: "Querying mkvmerge failed." |
| dry_run_cli.rs | `dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent` | stdout, tempdir-redacted |
| dry_run_cli.rs | `dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing` | stdout (replaces a loose `A \|\| to_lowercase().contains("regex")` fallback with an exact match) |
| dry_run_cli.rs | `dry_run_human_mode_surfaces_config_diagnostics_on_a_language_query_failure` | stderr: "Querying mkvmerge failed." |
| run_live.rs | `live_run_muxes_two_sources_and_reports_exit_zero` | one extracted line, not full stdout (see 4iv) |

**Kept as plain asserts (documented inline at each site):**

- `run_cli.rs:32,192`, `dry_run_cli.rs:627` (was 632) -- absence checks
  (`!stdout.contains("... start")`, `!stdout.contains(" ok, ")`,
  `!stdout.contains("Querying mkvmerge")`): behavioral invariants ("no job
  ran" / "--json suppresses human lines"), not a wording pin -- there is
  nothing to snapshot for an absence.
- `run_cli.rs:118,610`, `dry_run_cli.rs:501,685` -- `stderr.contains("mkvmerge")`
  and `stdout.contains("tracks[0].match")`: structural identifiers (program
  name, schema-relative config_path), not translatable prose.
- `cli_schema.rs:15-16` -- `text.contains("profile_version"/"tracks")`: JSON
  schema key presence, the brief's own "key presence in JSON" stay-bucket.
- `catalog_completeness.rs` (4 sites) -- `contains("{$")`/`.contains(id)`:
  the catalog-integrity mechanism itself (unresolved-placeholder detection,
  set membership), not app-output wording; converting these would test the
  test harness, not the CLI.

## 4. Notable judgment calls

**i. Caught a real bug via the "review before accept" mandate.**
`bad_regex_profile_exits_two_without_executing_a_job`'s first `.snap.new`
contained an unredacted absolute path: `0 files matched (searched
/tmp/.tmp0YYOjv, extensions mkv)`. Root cause: this test is *not* gated on
`have_mkvmerge()` (deliberate, per its own doc comment -- the old loose
`contains("Invalid regular expression")` assertion was written to survive
either a "mkvmerge present" or "mkvmerge absent" test machine). With
mkvmerge present (true of this dev machine and every CI leg since Plan 5.5
Task 2), `run` still reaches `plan_batch`/`print_batch_human` despite the
config-time regex error, so stdout also carries the dry-run-summary line
with this test's own real tmp path. Fixed by wrapping this one test's
snapshot in `insta_settings_with_tmp(dir.path())` too. Re-ran twice with
fresh random paths and diffed the two `tests/snapshots/` trees byte-for-byte
identical (see section 1) before accepting -- this would have been an
immediate CI failure (different runner, different tmp path) had it gone in
unredacted. Noted in the test's own comment for the next reader: this test's
stdout shape was already, by design, machine-dependent (present vs. absent
mkvmerge); the snapshot fixes on the "present" shape only, which is now the
shape every real execution context (this machine, all 3 CI legs) produces.
The "absent" shape is separately and deterministically covered by the sibling
test right below it (forces mkvmerge missing via an empty `PATH`).

**ii. `invalid_profile_exits_two_and_renders_messages`** used to be three
`.and()`-chained substring predicates; now one snapshot capturing the full
3-diagnostic block (2 errors + 1 warning + the `validate-summary` count
line) -- strictly more coverage of the actual rendered order/formatting,
not just presence of three fragments.

**iii. `dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing`**
had a loose `A || B` fallback (`contains("Invalid regular expression") ||
to_lowercase().contains("regex")`) -- exactly the kind of hedge a
wording-coupled test grows over time to survive edits without anyone
tightening it back up. The snapshot is now exact, which is the point.

**iv. `run_live.rs` scope-narrowed deliberately.** Did not snapshot this
gated E2E test's full stdout, only the extracted run-summary line. The
milestone lines above it (`run-job-start`/`-progress`/`-ok`) carry real
elapsed seconds (needs the duration filter) and, for a near-instant
single-subtitle-track fixture mux, a nondeterministic *subset* of the 25/50/
75% progress thresholds mkvmerge's coarse-grained progress reporting
happens to cross -- not safe to pin even with redaction, since the set of
lines itself (not just their content) can vary run to run. The extracted
line has no dynamic content at all (`"{ok} ok, {warning} warning, {failed}
failed, {cancelled} cancelled"`, all pre-resolved integers), found by its
distinctive shape rather than assumed to be the last stdout line (a
`run-joblog-written` line can follow it, since a real runs-root resolves in
this environment).

## 5. Snapshot content review (Step 2)

Ran `cargo insta test` (not `cargo test`) so all snapshot tests are
collected rather than stopping at the first failure; every `.snap.new` was
`cat`'d and read in full before running `cargo insta accept` (transcript
below is the actual reviewed content, byte-for-byte what got committed):

```
cli_validate__valid_profile_exits_zero_with_ok_message.snap:
Profile is valid.

cli_validate__invalid_profile_exits_two_and_renders_messages.snap:
[error] input.pattern: Invalid regular expression: regex parse error: ([
 ^
error: unclosed character class
[error] tracks[2].match.substring.forced_track: Property "forced_track" has type boolean; substring conditions require a string property.
[warning] tracks[1]: Rules tracks[0] and tracks[1] provably overlap: every track matching one also matches the other. Add a distinguishing condition to one of them.
2 errors, 1 warning, 0 infos.

cli_validate__warnings_only_exits_one.snap:
[warning] tracks[1]: Rules tracks[0] and tracks[1] provably overlap: ...
0 errors, 1 warning, 0 infos.

dry_run_cli__dry_run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent.snap:
0 files matched (searched [TMPDIR], extensions mkv)

dry_run_cli__dry_run_human_mode_surfaces_config_diagnostics_on_a_language_query_failure.snap:
Querying mkvmerge failed.

dry_run_cli__dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing.snap:
[error] input.pattern: Invalid regular expression: regex parse error: S(?<s>\d{2}E(?<e>\d{2}) ^ error: unclosed group

run_cli__bad_regex_profile_exits_two_without_executing_a_job.snap:
[error] input.pattern: Invalid regular expression: ... unclosed group
0 files matched (searched [TMPDIR], extensions mkv)

run_cli__bad_regex_profile_with_missing_mkvmerge_exits_two_without_executing_a_job.snap:
[error] input.pattern: Invalid regular expression: ... unclosed group

run_cli__run_human_mode_speaks_on_an_empty_source_dir_instead_of_staying_silent.snap:
0 files matched (searched [TMPDIR], extensions mkv)

run_cli__run_human_mode_surfaces_config_diagnostics_on_a_language_query_failure.snap:
Querying mkvmerge failed.

run_live__live_run_muxes_two_sources_and_reports_exit_zero.snap:
2 ok, 0 warning, 0 failed, 0 cancelled
```

Every value matches its diagnostic's real emitter (config_path, message
text, counts) and its Fluent template exactly; every `[TMPDIR]` placeholder
sits exactly where the real temp root used to be checked; no leaked
absolute path, mkvmerge version, or duration in the final accepted set.
`cargo insta accept` promoted all 11 `.snap.new` -> `.snap`; confirmed zero
`.snap.new` remain (`find tests/snapshots -name '*.snap.new'` empty).

## 6. CI-strictness check (Step 3)

`ci.yml`'s test job ends with plain `cargo test --workspace` (no
`INSTA_UPDATE` set anywhere in the workflow, no `cargo-insta` binary
needed). This is sufficient and correct without any `ci.yml` change:

- GitHub Actions sets `CI=true` in every job by default (platform default,
  not something this repo's workflow opts into).
- insta's default `INSTA_UPDATE=auto` resolves to `SnapshotUpdate::Auto` ->
  `is_ci()` true -> `SnapshotUpdateBehavior::NoUpdate` (confirmed by reading
  `insta` 1.48.0's own `env.rs`/`utils.rs` source, section 2 above).
- `assert_snapshot!` under `NoUpdate` panics on a mismatch exactly like any
  other assert -- `cargo test` (not `cargo insta test`, which force-passes
  to collect every new snapshot for review) fails the build normally.

**Empirically verified**, not just read from source: deliberately corrupted
one accepted snapshot's content, ran `CI=true cargo test -p muxsmith-cli
--test cli_validate` -- failed hard with a diff, zero `.snap.new` written,
exit nonzero, "Stopped on the first failure" (no force-pass). Restored the
original content, reran, clean pass. Also confirmed plain `cargo test -p
muxsmith-cli` (dev machine, no `CI` var, `INSTA_UPDATE` unset) passes clean
against the real accepted snapshots.

## 7. Gate results (nine parts, per `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md:18`)

`pnpm install --frozen-lockfile` run once first (`node_modules` was absent
in this fresh worktree).

| # | Command | Result |
|---|---|---|
| 1 | `cargo test --workspace` | 36 test-result blocks, all `ok`, 0 failed |
| 2 | `cargo fmt --all --check` | clean |
| 3 | `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| 4 | `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` (32 pre-existing duplicate-version warnings, unrelated to `insta`/`regex`, exit 0) |
| 5 | `pnpm lint` | clean |
| 6 | `pnpm build` | built |
| 7 | `pnpm check:i18n` | ok (12 pre-existing GUI-catalog unused-key warnings, unrelated) |
| 8 | `pnpm test:e2e` | 4/4 Playwright smoke tests passed |
| 9 | `cargo doc --workspace --no-deps` | generated clean |

## 8. Self-review

- Every conversion target traced to a real emitter/template, not guessed;
  content reviewed against the actual rendered text before acceptance, per
  section 5.
- Redaction correctness verified empirically (double-run diff), not just
  asserted.
- CI-strictness verified empirically (forced failure + restore), not just
  read from docs.
- The one real defect the process caught (section 4i) is exactly the
  scenario the brief's Step 1 warned about; fixed before commit, not left
  as a known issue.
- `.gitattributes`' LF policy (Task 1) needed no per-snapshot override --
  confirmed all 11 `.snap` files are LF-only (`cat -A`, no `^M`).
- Did not touch `cli_schema.rs` or `catalog_completeness.rs`: reviewed and
  classified (section 3), correctly out of scope, not overlooked.

## 9. Concerns / residual risk

- `run_cli__bad_regex_profile_exits_two_without_executing_a_job.snap` and
  `dry_run_cli__dry_run_human_surfaces_config_diagnostics_when_mkvmerge_missing.snap`
  both embed the `regex` crate's own parse-error wording verbatim (spec 10's
  accepted third-party-text exception). A `regex` crate version bump that
  changes its error formatting will fail these two snapshots -- correctly
  so (Cargo.lock pins the exact version already, so this only fires on a
  deliberate `cargo update`, at which point `cargo insta review` is the
  right next step, not a silent break).
- `bad_regex_profile_exits_two_without_executing_a_job` (section 4i) no
  longer tolerates a test machine with mkvmerge absent from `PATH` in the
  way the original loose assertion did (its own doc comment describes that
  as the original intent). In today's reality (Task 2 installs mkvmerge on
  every CI leg unconditionally) this is moot everywhere the suite actually
  runs; flagged rather than silently narrowed, in case a future CI change
  reintroduces a leg without mkvmerge.
- Mkvmerge-version and duration filters are currently unexercised by any
  committed snapshot (no CLI-rendered text embeds either today, confirmed by
  grep) -- present per the brief's explicit mandate and for the next
  snapshot that touches job-progress or a version-skew message, not because
  a current leak was found.

## T22 Review Cleanup (2026-07-12)

Two follow-up fixes from code-review pass:

1. **Removed dead `predicates` dev-dependency** from `crates/muxsmith-cli/Cargo.toml`:
   - Grepped to verify no direct usage in code (only a doc-comment mention in cli_validate.rs).
   - `predicates` is still transitively pulled by `assert_cmd`, so no functional change.
   - Cleaned up `Cargo.lock` (`cargo build -p muxsmith-cli --tests`).

2. **Reconciled contradictory comments** in `crates/muxsmith-cli/tests/run_cli.rs` around `bad_regex_profile_exits_two_without_executing_a_job`:
   - Old header claimed "machine-independence" regardless of mkvmerge situation.
   - Inline comment correctly described actual behavior: mkvmerge-present-shaped.
   - Merged into one accurate header; removed redundant inline comment.

**Verification:**
- `cargo test -p muxsmith-cli`: all 29 tests pass
- `cargo fmt --all --check`: clean
- `cargo deny check`: pass (license set unchanged, no new advisories)

**Commit:** `5a1bd8f` (unsigned)
