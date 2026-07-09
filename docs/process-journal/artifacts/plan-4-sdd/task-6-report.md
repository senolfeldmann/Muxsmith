# Task 6 Report: mkvtoolnix in CI

## Summary

Successfully added mkvtoolnix installation step to the `test` job in `.github/workflows/ci.yml`. The step runs on Linux only, after checkout and before cargo steps, with explanatory comment as specified.

## Changes Made

**File:** `.github/workflows/ci.yml`

**Location:** After `Swatinem/rust-cache@v2` step (line 24), before `cargo fmt --all --check` step.

**Added:**
```yaml
      # Linux-only: branch pushes are Linux-only anyway (minute economy);
      # gated integration tests self-skip on other platforms.
      - name: Install mkvtoolnix (gated integration tests)
        if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y mkvtoolnix
```

## Validation Results

- **YAML Parsing:** PASSED - `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` succeeded
- **cargo test --workspace:** PASSED - 2 passed in codegen tests; unit tests in all crates passed
- **cargo fmt --all --check:** PASSED
- **cargo clippy --workspace --all-targets -- -D warnings:** PASSED
- **cargo deny check:** PASSED - advisories ok, bans ok, licenses ok, sources ok

## Commit

```
5aefde1 ci: install mkvtoolnix on Linux so gated tests run (D18)
```

Files changed: 1 (`.github/workflows/ci.yml`)
Insertions: 5
Commit message: Per brief specification with co-author trailer

## Post-Merge Verification

Verification of gated test execution in CI is confirmed post-merge: the controller will review the next push's Actions log to confirm gated tests RAN (not skipped) in the Linux test job. This task installs the dependency; actual test execution confirmation is out-of-scope here.

## Concerns

None. YAML is valid, all gate commands pass, commit is formatted correctly and pushed to branch `plan4-t6` only.
