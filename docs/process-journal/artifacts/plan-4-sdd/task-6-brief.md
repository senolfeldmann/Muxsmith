### Task 6: mkvtoolnix in CI [WAVE 1 - independent]

**Files:**
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1:** Add to the `test` job, after checkout and before the cargo steps:

```yaml
      - name: Install mkvtoolnix (gated integration tests)
        if: runner.os == 'Linux'
        run: sudo apt-get update && sudo apt-get install -y mkvtoolnix
```

Linux-only deliberately: branch pushes are Linux-only anyway (minute economy) and the gated tests self-skip elsewhere; macOS/Windows installs are a go-public follow-up. Comment this in the YAML.
- [ ] **Step 2:** Verification is the next push's CI run: the controller confirms the gated tests RAN (not skipped) in the Actions log - note in the report that this is verified post-merge.
- [ ] **Step 3: Commit** - `ci: install mkvtoolnix on Linux so gated tests run (D18)`

---

