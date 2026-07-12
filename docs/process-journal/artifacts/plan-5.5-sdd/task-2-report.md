# Task 2 report: mkvmerge on all three CI legs + version pin decision

## What was implemented (Steps 1-3 of the brief, `.github/workflows/ci.yml`)

**Step 1 - per-OS install steps.** Replaced the single `if: runner.os == 'Linux'`
install step with three named steps, one per OS, each gated by its own
`if: runner.os == '...'`, matching the existing style used for
`Install Tauri Linux build dependencies`.

**Step 2 - version-pin decision.** Checked the current versions directly
against each registry (WebFetch/WebSearch, 2026-07-11):

| Manager | Source checked | Version found | Pinned? |
|---|---|---|---|
| apt | `packages.ubuntu.com/resolute/mkvtoolnix` | `97.0-1build1` | yes, exact: `mkvtoolnix=97.0-1build1` |
| choco | `community.chocolatey.org/packages/mkvtoolnix` | `100.0.0` | yes, exact: `--version=100.0.0` |
| brew | `formulae.brew.sh/api/formula/mkvtoolnix.json` | `100.0` (`versioned_formulae: []`) | no - floats |

Decision: majors diverge (97 apt vs 100 choco/brew) -> pin per-manager per
Şenol's standing preference, which the brief's own decision rule ("pin if
majors diverge") also selects. apt and choco both support an install-time
exact-version selector and are pinned. Homebrew's core tap keeps no
versioned `mkvtoolnix` formula and no install-time version selector
(`brew pin` only locks an *already-installed* version against a later
`brew upgrade`, it does not let you choose an older version at install
time) - the one manager where an exact pin is not idiomatically possible,
so it floats on latest stable. All of this is documented in a ci.yml
comment replacing the old backlog note, and in the commit message.

**Important corrected fact vs. the task brief:** the brief's outer context
stated "runners are ubuntu-24.04 / windows-2025 / macos-15" and told me to
check "the Ubuntu 24.04 (noble) package page." The actual `ci.yml` matrix
(confirmed by reading the file, not from the outer summary) runs
`ubuntu-26.04` ("Resolute Raccoon", GA per GitHub as of this session; the
top-of-file comment calling it "a preview image" is now stale but out of
this task's scope to fix). I used the **actual** matrix OS (26.04/resolute)
as the authoritative source for the apt version lookup instead of the
stale 24.04 instruction - noble's mkvtoolnix version would have been the
wrong input for a pin decision that has to hold on the runner that's
actually used. Noted here per "report contradictions instead of guessing"
since it doesn't block the task, but the correction should be visible.

**Step 3 - skip-marker assertion.** Added a new step
`Assert no gated tests silently skip (mkvmerge missing)`, positioned right
after the existing `cargo test --workspace` step, running unconditionally
on every leg (no OS guard - all three legs now install mkvtoolnix, so zero
skips is the expectation everywhere, not just win/mac):

```yaml
- name: Assert no gated tests silently skip (mkvmerge missing)
  shell: bash
  run: |
    cargo test --workspace -- --nocapture --test-threads=1 2>&1 | tee gated-test-output.log
    count="$(grep -c 'mkvmerge not found; skipping' gated-test-output.log || true)"
    echo "Skip-marker occurrences: $count"
    if [ "$count" -ne 0 ]; then
      echo "::error::$count gated integration test(s) silently skipped (mkvmerge not found on PATH) - expected 0 now that mkvtoolnix is installed on every leg."
      exit 1
    fi
```

### How it works, and why it cannot false-negative

- **The exact marker.** Grepped the repo for the literal eprintln text used
  by every gated test (`crates/muxsmith-core/tests/{mkvmerge_runtime,identify_live,
  command_integration,executor_live}.rs`, `crates/muxsmith-cli/tests/{dry_run_cli,
  run_live,run_cli}.rs`, `src-tauri/src/lib.rs`): `eprintln!("mkvmerge not found;
  skipping");`. One correction to the outer task's "facts": this string is not
  centralized in `crates/muxsmith-core/tests/support/mod.rs` (that file only holds
  `FakeIdent` and the `lang()` helper) - it's inlined at each gated-test call site.
  The exact wording given in the task ("mkvmerge not found; skipping") was
  correct and is what I grep for; only the claimed location was wrong. Doesn't
  change the implementation - grepping combined test output catches every call
  site regardless of which file it lives in.
- **`--nocapture` is load-bearing.** `cargo test` captures stdout/stderr per test
  and only replays it for *failing* tests; the gated tests intentionally `return`
  early (still counted as passing) after the `eprintln!`, so without
  `--nocapture` the marker is invisible even though the skip happened. Verified
  this is exactly the mechanism described in the task brief.
- **`--test-threads=1` closes an interleaving loophole.** With `--nocapture`,
  every test thread writes directly and concurrently to the real stdout/stderr
  (Rust's own docs note nocapture output interleaves across threads). A
  workspace-wide run has many threads; without serializing, a marker line could
  in principle be split/interleaved with another thread's output and evade the
  grep. Single-threading removes that risk entirely rather than relying on
  `eprintln!`'s per-call write being atomic under contention.
- **Cannot false-negative on either failure mode:**
  - *mkvmerge genuinely missing* (install regression on a leg): the gated tests
    still run (they don't panic, they self-skip-and-pass) but emit the marker,
    `grep -c` returns > 0, the step explicitly checks `count -ne 0` and exits 1
    with a `::error::` annotation.
  - *`cargo test` itself fails* (a real test regression, unrelated to mkvmerge):
    `shell: bash` on GitHub Actions defaults to `-e -o pipefail` (verified via
    GitHub's own ADR: Windows resolves to
    `Git\bin\bash.EXE --noprofile --norc -e -o pipefail {0}`, same flags
    non-Windows). `pipefail` makes the `cargo test | tee ...` pipeline's exit
    status reflect `cargo test`'s failure even though `tee` is the last command
    in the pipe, and `-e` then aborts the step immediately - no need for manual
    exit-code plumbing, and unlike capturing into a shell variable first, `tee`
    still streams the full failure output to the CI log before aborting.
  - The `|| true` after `grep -c` exists only to stop `-e` from tripping on
    grep's own "no match" exit status (1) in the *expected*, healthy case
    (count = 0) - it does not suppress any actual failure path; I locally
    simulated all three cases (zero-match, non-zero-match, upstream pipeline
    failure) under `bash -e -o pipefail` and confirmed each behaves as
    designed (see Gate/verification section).
- **Runs on every leg, not just win/mac.** The brief's acceptance text names
  win/mac explicitly, but the outer task context flagged that Linux now also
  installs mkvtoolnix so zero is expected there too. Rather than special-case
  the assertion to skip Linux, it runs identically on all three legs - simpler,
  and it also catches a regression where Linux's own install step breaks.
- **Why the workspace is rerun in full rather than filtering to just the gated
  test binaries:** the gated tests are scattered across 7 files with no
  consistent naming/tag that a `--test <name>` filter could target reliably
  (and one set lives as inline unit tests in `src-tauri/src/lib.rs`, which
  `--test` can't select at all). A workspace-wide rerun costs one extra
  (relink-only, not rebuild - `--nocapture`/`--test-threads` are runtime flags)
  test pass but automatically covers any gated test added in the future without
  needing the filter list maintained in lockstep - the same kind of blind spot
  this task exists to close.

## Gate results (all 8 parts, run locally from repo root; this machine has a
real mkvmerge on PATH, so the gated tests exercised the live path here too)

| Check | Result |
|---|---|
| `cargo test --workspace` | ok - 78+ tests passed (workspace total across crates) |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| `pnpm lint` | exit 0 |
| `pnpm build` | exit 0 (vue-tsc + vite build succeeded) |
| `pnpm check:i18n` | ok (171 catalog ids, pre-existing unused-key warnings only, non-fatal) |
| `pnpm test:e2e` | 3/3 Playwright smoke tests passed |

`ci.yml` itself can't be executed locally (no `act`/`actionlint` installed;
per the task instructions this wasn't assumed and wasn't installed). Instead:
validated with `python3 -c "import yaml; yaml.safe_load(...)"` - parses
cleanly, and I inspected the parsed step list to confirm the `if:`/`shell:`/
`run:` fields came out exactly as intended (no accidental YAML block-scalar
or indentation mistakes). Additionally hand-simulated the new step's bash
logic under `bash -e -o pipefail` (the exact flags GitHub Actions'
`shell: bash` uses on every OS, confirmed via the `actions/runner` ADR) for
three scenarios - zero-match (pass), non-zero-match (correctly exits 1),
and an upstream pipeline failure (correctly exits 1, no output swallowed) -
all behaved as designed.

## Files changed

- `.github/workflows/ci.yml` - only file touched.

## Commit

`374005a` - "ci: install mkvmerge on every leg + version-pin decision (Plan
5.5 Task 2, #14)", unsigned (`commit.gpgsign=false`), `ci.yml` staged
explicitly (no `git add -A`).

## Self-review findings

- Re-read the diff after committing: no stray debug artifacts, no scope
  creep into `docs/ROADMAP.md` or other files (out of scope per the
  controller's instructions - Steps 4-5 are the controller's).
- Confirmed `choco`/`brew` are runner-preinstalled per the brief's framing
  (no new `uses:` action added, so the SHA-pinning discipline for actions
  doesn't apply here - only `run:` steps were added).
- Double-checked apt pin syntax (`pkg=version`, no space) and choco pin
  syntax (`--version=X`) against each tool's standard CLI convention.
- One residual operational risk, disclosed in both the ci.yml comment and
  the commit message rather than hidden: the apt pin is an *exact build
  string* (`97.0-1build1`). Ubuntu can bump the build suffix on a
  no-source-change rebuild (e.g. a shared-library ABI bump), which would
  make this exact pin start failing with "version not found" until bumped.
  This is the same maintenance shape as the existing SHA-pin policy at the
  top of the file (needs an occasional manual or Renovate-assisted bump);
  it is not something Renovate/Dependabot will catch automatically today
  since it's a plain string inside a shell `run:` line, not a manifest
  field - flagging this as a known gap rather than silently accepting it.

## Concerns for the controller

- The skip-marker assertion step reruns `cargo test --workspace` a second
  time (single-threaded, `--nocapture`) after the existing `cargo test
  --workspace` step. This roughly doubles Rust test wall-clock time per CI
  run (compilation itself is not repeated - only flags differ, so cargo's
  up-to-date check skips rebuilding). Judged this an acceptable, correctness-
  first tradeoff given the brief's explicit "cannot false-negative"
  requirement and the project's current size; flagging in case Şenol wants
  a cheaper mechanism later (e.g. a compile-time cfg/feature that tags gated
  tests for `--test`-level filtering).
- The apt exact-build pin is the most maintenance-prone of the three (see
  above) - worth keeping an eye on if a future CI run reports "version not
  found" for mkvtoolnix on the Linux leg.
- Did not touch `docs/ROADMAP.md` or push, per the explicit scope boundary
  for this task (Steps 4-5 are the controller's).

## Fix wave 1 (Windows GITHUB_PATH)

**Problem:** CI run 29165166725 caught 18 gated tests silently skipping on Windows. Cause: `choco install` writes the machine PATH registry, but a running GitHub Actions job never re-reads it; `mkvmerge.exe` installed to `C:\Program Files\MKVToolNix` remained unreachable in subsequent steps.

**Changes:** `.github/workflows/ci.yml`, Windows install step only (lines 51-60):
- Added explicit `shell: pwsh` declaration
- Added existence check: `Test-Path 'C:\Program Files\MKVToolNix\mkvmerge.exe'` with hard failure if missing (Write-Error + exit 1)
- Added idiomatic PATH propagation: `Add-Content -Path $env:GITHUB_PATH -Value 'C:\Program Files\MKVToolNix'` for all downstream steps

**Validation:** `python3 -c "import yaml; yaml.safe_load(...)"` — YAML parses successfully; pwsh block syntax verified against GitHub Actions runner spec.

**Commit:** `19deec3` — "ci(fix): propagate MKVToolNix dir to GITHUB_PATH on Windows leg"

## Fix wave 2 (Windows set_modified)

**Problem:** CI run 29165393732 failed on Windows with `PermissionDenied (os error 5)` in `live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched`. Root cause: `crates/muxsmith-cli/tests/run_live.rs` line 149, `backdate_mtime` helper opened files read-only with `fs::File::open(path)`, then called `file.set_modified(target)`. Windows requires FILE_WRITE_ATTRIBUTES access to modify file times; Unix futimens accepts read-only handles.

**Changes:** `crates/muxsmith-cli/tests/run_live.rs`, lines 148-150:
- Changed `fs::File::open(path)` to `fs::OpenOptions::new().write(true).open(path)`
- Added one-line comment documenting the Windows constraint

**Repo scan:** Grep across `crates/` and `src-tauri/` found only one `set_modified` call site (the one fixed). No other sites required repair.

**Validation:**
- `cargo test -p muxsmith-cli --test run_live`: both tests passed (live_run_muxes_two_sources_and_reports_exit_zero ✓, live_run_rerun_with_on_collision_skip_exits_one_and_leaves_outputs_untouched ✓)
- `cargo fmt --all --check`: exit 0
- `cargo clippy --workspace --all-targets -- -D warnings`: exit 0

**Commit:** `24ac702` — "test(fix): open writable handle for set_modified - Windows PermissionDenied (run 29165393732)"
