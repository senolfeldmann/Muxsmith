# Fastfollow: `--json` document on profile-load / list-languages failure

## Bug recap

Two early-exit error paths in both `run` and `dry-run` broke `--json`'s
"stdout carries exactly one JSON document" contract:

1. Profile-load failure (`load::from_file` `Err`): printed a HUMAN-formatted
   diagnostic line to stdout unconditionally, ignoring `json`.
2. `list_languages` failure: `eprintln!`'d to stderr and returned 2 with NO
   document on stdout at all in json mode.

## Fix per site

All four sites now branch on `json` and reuse the existing "config-only"
document builders (`dry_run::config_only_json`, wrapped in `run::run_json_document`
for `run`), mirroring `validate.rs`'s `Err(d) => vec![d]` fold and the
sibling `Mkvmerge::locate()`-failure branch that already existed just below
each site.

- `crates/muxsmith-cli/src/commands/dry_run.rs:38-53` (profile load): json
  mode -> `config_only_json(&[d], renderer)`; human mode unchanged (still the
  bare `renderer.diagnostic(&d)` line).
- `crates/muxsmith-cli/src/commands/dry_run.rs:80-95` (list_languages): json
  mode -> `config_only_json(&config_diags, renderer)`; human mode unchanged
  (still just the stderr `mkvmerge-query-failed` message; the eprintln itself
  is now only reached in the `else` branch, matching how the sibling
  `locate()`-failure branch already suppresses its own eprintln under
  `--json`).
- `crates/muxsmith-cli/src/commands/run.rs:57-77` (profile load): json mode
  -> `run_json_document(dry_run::config_only_json(&[d], renderer), &[], &[])`
  (empty `jobs`, zeroed `summary`); human mode unchanged.
- `crates/muxsmith-cli/src/commands/run.rs:103-121` (list_languages): same
  wrapping as above; human mode unchanged.

No changes to `commands/mod.rs` were needed; both call sites already imported
the pieces they needed (`dry_run` module, `config_only_json`).

## A semantic nuance worth flagging (not blocking)

`config_only_json` always sets `"mkvmerge_found": false`. On the
list-languages-failure path, mkvmerge *was* located (`Mkvmerge::locate()`
succeeded) — only the `--list-languages` query itself failed. Reusing the
existing builder verbatim (per the task's explicit instruction to build on
"the existing json document builders") means that field reads `false` even
though, strictly, mkvmerge was found but broken. I judged this in-scope
because it reuses the existing schema rather than adding a field, and the
field's practical meaning to a JSON consumer ("don't expect a populated
report, and check your mkvmerge situation") still holds. If a sharper
distinction ever matters, that's a schema change (a new field or a
mkvmerge_found enum) and should go through its own review, not ride along
here.

Same builder is reused for the profile-load-failure path (before mkvmerge is
ever even probed), for the same reason: no schema was added, the existing
"config-only" shape was the closest fit, and `mkvmerge_found: false` reads
correctly as "not confirmed found" (we never checked) rather than an
affirmative false claim.

## TDD: RED then GREEN

Wrote 6 new CLI tests total (RED first, confirmed failing, then GREEN after
the fix):

**`crates/muxsmith-cli/tests/dry_run_cli.rs`:**
- `dry_run_json_emits_a_document_on_profile_load_failure` — RED: panicked
  trying to parse `[error] .../nonexistent.yaml : The profile could not be
  parsed: ...` as JSON. GREEN after fix.
- `dry_run_json_emits_a_document_when_the_language_query_fails` — RED: empty
  stdout, `EOF while parsing a value`. GREEN after fix.
- `dry_run_human_mode_still_just_reports_the_language_query_failure_on_stderr`
  — regression lock for the list-languages human-mode path; passed both
  before and after (documents unchanged behavior; no earlier test in this
  file covered this branch's human-mode side).

**`crates/muxsmith-cli/tests/run_cli.rs`:** identical trio, prefixed `run_`
instead of `dry_run_`, plus `run`-specific `jobs`/`summary` assertions and
`asserts_no_job_ran`.

RED run (`cargo test -p muxsmith-cli --test dry_run_cli --test run_cli`,
before the fix):
```
test dry_run_json_emits_a_document_on_profile_load_failure ... FAILED
test dry_run_json_emits_a_document_when_the_language_query_fails ... FAILED
test run_json_emits_a_document_on_profile_load_failure ... FAILED
test run_json_emits_a_document_when_the_language_query_fails ... FAILED
```
(the two human-mode regression tests already passed pre-fix, as expected —
they pin down *unchanged* behavior, not new behavior)

GREEN run (same command, after the fix): all 19 tests across both files
pass (9 in dry_run_cli.rs, 10 in run_cli.rs).

One iteration note: my first cut of the profile-load-failure tests asserted
`!stdout.contains("The profile could not be parsed")` as a "no human line"
check. That's wrong — `config_only_json` legitimately embeds the diagnostic's
rendered text inside the JSON document's `rendered` field, so the substring
correctly appears there. Replaced with `stdout.lines().count() == 1`, which
actually tests what "no separate human line" means (exactly one `println!`
happened).

## How the list_languages arm is covered

Cheaply testable, not just reasoned about: added a `#[cfg(unix)]` helper
(`fake_mkvmerge_that_fails_queries`, duplicated per-file per this codebase's
existing per-file-helper convention) that writes a tiny `#!/bin/sh` script
named `mkvmerge` into a temp dir, `chmod +x`, and points the child process's
`PATH` at that dir alone. The script exits 0 on `--version` (so
`Mkvmerge::locate()` succeeds) and exits 1 on everything else (so
`list_languages()`'s `--list-languages` call fails with `NonZero`) — a
deterministic stand-in for a broken mkvmerge install, no real MKVToolNix
needed, same wiring trick the pre-existing `empty_path_dir`/
`no_mkvmerge_path` tests already use for the sibling `locate()`-failure path.

Gated `#[cfg(unix)]` because a raw shebang script has no direct Windows
equivalent (`Command::new("mkvmerge")` would look for `mkvmerge.exe`/`.cmd`/
`.bat` there instead), and this repo's CI matrix does run Windows on PRs/tags
(`.github/workflows/ci.yml`). This is real, executed coverage on Linux/macOS
CI runners and on this dev machine; it self-excludes on Windows rather than
silently no-op'ing (there's no `if !cfg(unix) { skip }` — the test simply
doesn't exist in a Windows build), which I'm flagging explicitly rather than
letting it pass as "fully covered on every platform."

## Gate results (all foreground)

- `cargo test --workspace`: all green (21+90+49+... suites, 0 failed across
  every crate; full list checked via `grep -E "FAILED|error\[|test result:"`
  against `cargo test --workspace` output — every line reads `... 0 failed`).
- `cargo fmt --all --check`: failed once (my initial run.rs list_languages
  edit's manual line-wrap didn't match rustfmt's own), fixed by running
  `cargo fmt --all`, re-checked clean.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean, no
  warnings.
- `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
- Re-ran `cargo test --workspace` once more after the fmt pass to confirm
  the reformatting didn't change behavior: still all green.

## Scope

Touched exactly: `crates/muxsmith-cli/src/commands/run.rs`,
`crates/muxsmith-cli/src/commands/dry_run.rs`,
`crates/muxsmith-cli/tests/dry_run_cli.rs`,
`crates/muxsmith-cli/tests/run_cli.rs`. `commands/mod.rs` untouched (not
needed). No exit-code changes, no human-output changes, no document-schema
additions beyond folding the load diagnostic into the pre-existing
config-only shape.
