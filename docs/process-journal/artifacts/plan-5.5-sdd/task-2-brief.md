### Task 2: mkvmerge on all three CI legs + version pin decision (#14, FIRST substantive task by decision)

**Files:**
- Modify: `.github/workflows/ci.yml:33-37` (the `if: runner.os == 'Linux'` guard block)

**Interfaces:** produces live-mkvmerge CI on windows-2025/macos-15; T11/T23 rely on gated tests actually running there.

- [ ] Step 1: Replace the Linux-only install step with three per-OS steps (keep SHA-pinning discipline; choco and brew are runner-preinstalled):

```yaml
      - name: Install mkvtoolnix (gated integration tests, Linux)
        if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y mkvtoolnix
      - name: Install mkvtoolnix (gated integration tests, Windows)
        if: runner.os == 'Windows'
        run: choco install mkvtoolnix -y
      - name: Install mkvtoolnix (gated integration tests, macOS)
        if: runner.os == 'macOS'
        run: brew install mkvtoolnix
```

- [ ] Step 2: Version pin decision (ROADMAP rider): check what versions apt/choco/brew currently deliver (`mkvmerge --version` echo step in CI). Record the decision in the commit message: pin per-manager if the majors diverge, float if aligned - Şenol's standing preference is pin, so default to `choco install mkvtoolnix --version=X` and brew formula pin unless impossible; document either way in a ci.yml comment replacing the old backlog note (:33-34).
- [ ] Step 3: Add a step after tests on every leg: `cargo test -- --list 2>/dev/null | grep -c gated || true` is NOT reliable; instead assert the skip count: run the gated suite with `--nocapture` and grep CI logs for the self-skip marker string (read `tests/support/mod.rs` for the exact skip message first). Acceptance: the marker count on Windows and macOS legs is ZERO (walkthrough #14: "otherwise silent skipping is traded for silent skipping").
- [ ] Step 4: Push, verify all three legs green WITH live tests; paste the three job URLs into the PR/commit body. gh-log entry.
- [ ] Step 5: This closes the fired go-public trigger: remove the consumed trigger line from ROADMAP pre-1.0 (mark "done 2026-07-<date>, run <id>").

