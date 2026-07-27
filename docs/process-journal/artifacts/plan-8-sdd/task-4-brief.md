### Task 4: D83/D84/D85/D77/D79/D88/D89/D90 - release.yml

**Stream C** (`.worktrees/plan8-c`). Read design section 2 in full (the verbatim source), D77, D79, D83, D84, D85, D88, D89, D90, section 6, section 8 (G4/G5), and section 11. Model tier: mid.

**Files:**
- Create: `.github/workflows/release.yml`

**Interfaces:**
- Consumes (at run time, not at authoring - the referenced files land via streams A/B and are NOT present in this worktree; their absence here is expected and is not a defect): `scripts/check-version-sync.sh` (Task 1), `src-tauri/tauri.bundle.conf.json` (Task 2), `.github/release/draft-body.md` + `rehearsal-banner.md` (Task 3), `packaging/linux-tarball-README.txt` (Task 3), `mise.toml` (node version parse), `package.json` (`packageManager` for pnpm/action-setup), `rust-toolchain.toml`.
- Produces: the complete release pipeline - triggers `push.tags: ['v*']` + `workflow_dispatch` with the boolean `rehearse-draft-release` input; guard -> 4-leg bundle matrix -> assemble; the operator-facing contracts (input name, `rehearsal-<run_id>` draft naming, workflow-artifact names `muxsmith-<leg>`, retention 7).

The two enumerations transcribed from the design (the reference every step check below recounts against):

**The four legs (D85):**

```
| leg id | runner | rationale for the image |
|---|---|---|
| `windows-x86_64` | `windows-2025` | house pin (ci.yml test matrix) |
| `windows-arm64` | `windows-11-arm` | the only GA windows-arm64 label (section 1.3); native build (Tier-1 host tools) beats cross-compiling |
| `macos-arm64` | `macos-15` | house pin; arm64 (M1) verified |
| `linux-x86_64` | `ubuntu-22.04` | **compat floor**: Tauri's AppImage guidance mandates the oldest base providing webkitgtk 4.1 and names Ubuntu 22.04; Tauri's own CI example builds on it; artifacts built here run on every >= 22.04-era glibc (mkvtoolnix's AppImage targets glibc 2.28+ in the same spirit) |
```

Per-leg `--bundles` (D84/section 2 matrix): `msi` / `msi` / `dmg` / `deb,rpm,appimage`.

**The 8-asset name set (D89):**

```
1. `muxsmith-X.Y.Z-windows-x86_64.msi`
2. `muxsmith-X.Y.Z-windows-arm64.msi`
3. `muxsmith-X.Y.Z-macos-arm64.dmg`
4. `muxsmith-X.Y.Z-linux-x86_64.deb`
5. `muxsmith-X.Y.Z-linux-x86_64.rpm`
6. `muxsmith-X.Y.Z-linux-x86_64.AppImage`
7. `muxsmith-X.Y.Z-linux-x86_64.tar.gz`
8. `SHA256SUMS`
```

- [ ] **Step 1: Create `.github/workflows/release.yml`** by transcribing design section 2's YAML fence exactly - name, both triggers, workflow-level `permissions: contents: read`, the two policy comment blocks, the guard job (version-sync arm conditional on `github.ref_type == 'tag'`, then the gate-green poll: 30 s x 90 rounds, 45-minute fail-safe), the four-entry `include` matrix with `fail-fast: false`, every leg step in order (checkout, rustup, Linux apt deps, mise.toml node parse, setup-node, pnpm/action-setup, `pnpm install --frozen-lockfile`, CLI build + sidecar staging, `tauri build`, updater-absence assert, rename + tar.gz pack, upload-artifact with `retention-days: 7` and `if-no-files-found: error`), and the assemble job (checkout, download-artifact with `merge-multiple: true`, SHA256SUMS, conditional draft creation with the body composition rehearsal-banner -> template -> generated notes). Guard semantics, poll cadence and per-job permissions exactly as written (section 11); action SHAs and version comments exactly per section 1.4's table plus the house checkout pin.

- [ ] **Step 2: Transcription-fidelity proof**: extract the design's section-2 YAML fence with sed into a scratch file, `diff` it against the committed `.github/workflows/release.yml`, and state in the task report that the diff was empty. A non-empty diff is a defect; the design text wins (section 11).

- [ ] **Step 3: Parse check**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('yaml-ok')"
# Expected: yaml-ok (PyYAML present locally, measured 6.0.3 at plan-authoring)
```

- [ ] **Step 4: Fire-tests G4-G5 against the committed workflow text** (design section 8, transcribed verbatim):

```
- **G4**: the updater-absence step's script body (section 2) against a
  scratch tree: clean tree with one bundle file -> summary line
  `updater-artifact check: 0 hits across 1 bundle output files`, exit 0;
  planted `app.msi.sig` -> `::error::1 updater artifact(s) found`,
  exit 1; missing/empty bundle dir -> positive-control error, exit 1.
  (Already run at design time, 2026-07-23, exactly these outputs;
  re-run pre-merge against the committed workflow text.)
- **G5**: the rename step's `pick()` against a scratch dir: one match ->
  `pick: <path>` on stderr, path on stdout, exit 0; zero matches and two
  matches -> `::error::expected exactly one artifact, got: ...` on
  stderr, exit 1. (Already run at design time, 2026-07-23, exactly these
  outputs; re-run pre-merge against the committed workflow text.)
```

Execution notes: "against the committed workflow text" means the script bodies are EXTRACTED from `.github/workflows/release.yml` (slice the step's `run: |` block with awk/sed and dedent), never retyped from the design. G4's body is pure bash (no `${{ }}` expressions) and runs wholesale in a scratch dir carrying a fabricated `target/release/bundle/` tree. For G5, extract the `pick()` function definition into a scratch script and invoke it with one, zero and two matching files. All runs foreground; record the six observed outputs in the task report.

- [ ] **Step 5: Negative-space check** (section 6/section 11; absence check, fire-verified with a positive control):

```bash
grep -n 'tauri-action\|softprops\|rust-cache\|mise-action\|Swatinem\|concurrency' .github/workflows/release.yml
# Expected: no output (none of the banned shapes; no concurrency group).
# Positive control (proves the pattern fires): the same grep against
# .github/workflows/ci.yml hits its mise-action and Swatinem/rust-cache lines.
```

- [ ] **Step 6: Pin conformance** (recount):

```bash
grep -c 'uses:' .github/workflows/release.yml
# Expected: 7 (guard checkout, leg checkout, setup-node, pnpm/action-setup,
# upload-artifact, assemble checkout, download-artifact - recomputed from
# section 2; every one a 40-hex SHA with a version comment)
grep -n 'uses:' .github/workflows/release.yml | grep -v '@[0-9a-f]\{40\} # v'
# Expected: no output (every uses line SHA-pinned + commented). Fire-verify:
# temporarily change one SHA-pin line to @v7 in the working copy, see the
# second grep emit it, restore.
```

- [ ] **Step 7: Commit**

```bash
git add .github/workflows/release.yml
git -c commit.gpgsign=false commit -m "release: release.yml - guard (version sync + ci-gate green), four native bundle legs, draft-release assemble with SHA256SUMS (D77/D79/D83-D85/D88-D90); G4/G5 fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

