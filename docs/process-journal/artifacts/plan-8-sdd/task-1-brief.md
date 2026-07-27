### Task 1: D87 - version sync: the guard script, and tauri.conf.json stops declaring a version

**Stream A** (`.worktrees/plan8-a`). Read D87 in full, design 3.3, and section 8's G1-G3. Model tier: cheap (the code is carried below, verbatim).

**Files:**
- Modify: `src-tauri/tauri.conf.json` (delete the top-level `version` key; nothing else)
- Create: `scripts/check-version-sync.sh` (executable)

**Interfaces:**
- Consumes: `Cargo.toml` `[workspace.package] version` (0.1.0 today), `package.json` `version`, `jq` + `awk` (local and runner-image tools).
- Produces: the guard contract release.yml's guard job calls (Task 4): plain run = consistency check, `v<X.Y.Z>` argument = consistency + tag equality; exit 0/1.

- [ ] **Step 1: Delete the `version` key from `src-tauri/tauri.conf.json`**

Remove the top-level `"version": "0.1.0",` line, nothing else. Verify:

```bash
jq 'has("version")' src-tauri/tauri.conf.json
# Expected: false
jq -r 'keys_unsorted | join(",")' src-tauri/tauri.conf.json
# Expected: $schema,productName,identifier,build,app,bundle
# (measured pre-change 2026-07-23: the same list plus "version")
```

The bundler then reads the version from `src-tauri/Cargo.toml`, which inherits `[workspace.package] version` - the schema fallback is verified in design section 1.1; do not re-verify it.

- [ ] **Step 2: Create `scripts/check-version-sync.sh`** with exactly this content (design 3.3, transcribed verbatim; diffed against the design at plan-authoring), then `chmod +x scripts/check-version-sync.sh`:

```bash
#!/usr/bin/env bash
# Version-sync guard (Plan 8, D87). Usage:
#   scripts/check-version-sync.sh          # consistency only
#   scripts/check-version-sync.sh vX.Y.Z   # consistency + tag equality
# Asserts: Cargo workspace version == package.json version;
# tauri.conf.json declares NO version (it inherits Cargo's);
# with an argument: the tag is exactly v<version>.
set -euo pipefail

fail() { echo "version-sync: $*" >&2; exit 1; }

cargo_v="$(awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml)"
[ -n "$cargo_v" ] || fail "could not parse [workspace.package] version from Cargo.toml"

pkg_v="$(jq -r .version package.json)"
[ "$cargo_v" = "$pkg_v" ] || fail "Cargo.toml ($cargo_v) != package.json ($pkg_v)"

tauri_has_v="$(jq 'has("version")' src-tauri/tauri.conf.json)"
[ "$tauri_has_v" = "false" ] || fail "src-tauri/tauri.conf.json declares 'version'; it must inherit from Cargo.toml (D87)"

if [ "$#" -ge 1 ]; then
  [ "$1" = "v$cargo_v" ] || fail "tag $1 != v$cargo_v"
fi

echo "version-sync: OK ($cargo_v)"
```

- [ ] **Step 3: Fire-tests G1-G3** (design section 8, transcribed verbatim; run each foreground, record the outputs in the task report):

```
- **G1**: `scripts/check-version-sync.sh v9.9.9` on the clean tree ->
  must exit 1 (tag arm red state); `scripts/check-version-sync.sh` ->
  exit 0 (green reachable, `proc-check-green-state-reachable`).
- **G2**: temporarily add `"version": "0.1.0"` back to
  `tauri.conf.json` -> plain run must exit 1 (absence assertion fires);
  revert.
- **G3**: temporarily set `package.json` version to `0.1.1` -> plain run
  must exit 1 (equality arm fires); revert.
```

Execution notes: G2's "revert" means re-deleting the key by edit (Step 1's change is not yet committed, so `git checkout` would also revert it) - re-run Step 1's two jq checks after the revert. G3's revert restores `"version": "0.1.0"` in `package.json`; verify with `jq -r .version package.json`.

- [ ] **Step 4: Config-parse sanity without the version key**

Run: `cargo check -p muxsmith-gui`
Expected: clean - src-tauri's build script (`tauri_build::build()`) parses `tauri.conf.json` on every compile, so this is the cheapest full-parse proof that deleting the key breaks nothing.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/tauri.conf.json scripts/check-version-sync.sh
git -c commit.gpgsign=false commit -m "release: version-sync guard script; tauri.conf.json inherits the Cargo workspace version (D87); G1-G3 fire-verified" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

