### Task 2: D86 + D82 - the bundle configuration, the CLI-sidecar overlay, and their collateral

**Stream A**, after Task 1. Read D86 and D82 in full, design 3.1, 3.2, 3.4, 4.4. Model tier: cheap (all four artifacts carried below, verbatim).

**Files:**
- Modify: `src-tauri/tauri.conf.json` (the bundle block; nothing outside it)
- Create: `src-tauri/tauri.bundle.conf.json`
- Modify: `.gitignore` (one line)
- Modify: `BUILDING.md` (one appended subsection)

**Interfaces:**
- Consumes: Task 1's version-free config; the five existing icons under `src-tauri/icons/`; `LICENSE` at the repo root.
- Produces: the bundle surface Task 4's legs build against (`--bundles` values narrow 3.1's `targets` superset); the overlay filename `src-tauri/tauri.bundle.conf.json` release builds pass via `-c` (D84); the staging dir ignore rule.

- [ ] **Step 1: Rewrite `src-tauri/tauri.conf.json` to the design-3.1 end state.** The full file after the change (design 3.1 verbatim; `build` and `app` sections unchanged and elided there AND here - leave them exactly as they are; the live file's top-level key set was verified 2026-07-23 to contain nothing beyond what 3.1 shows):

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Muxsmith",
  "identifier": "io.github.senolfeldmann.muxsmith",
  "build": { "...": "unchanged" },
  "app": { "...": "unchanged" },
  "bundle": {
    "active": true,
    "targets": ["msi", "dmg", "deb", "rpm", "appimage"],
    "publisher": "Şenol Feldmann",
    "copyright": "Copyright (c) 2026 Şenol Feldmann",
    "homepage": "https://github.com/senolfeldmann/Muxsmith",
    "license": "MIT",
    "licenseFile": "../LICENSE",
    "category": "Video",
    "shortDescription": "Rule-based bulk MKV muxing tool",
    "longDescription": "Declare how your MKVs should look. Muxsmith forges the whole library into shape - one profile, hundreds of files, zero clicking.",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "wix": {
        "upgradeCode": "9262b417-b687-5ea3-ace1-18b9d51b215f",
        "language": ["en-US"]
      }
    },
    "macOS": {
      "minimumSystemVersion": "11.0"
    },
    "linux": {
      "deb": { "section": "video", "recommends": ["mkvtoolnix"] },
      "rpm": { "recommends": ["mkvtoolnix"] }
    }
  }
}
```

Every literal is frozen by design section 11 (upgradeCode GUID, publisher spelling with `Ş`, category `Video`, section `video`, minimumSystemVersion `11.0`, the en-US language list). `createUpdaterArtifacts`, `fileAssociations`, `macOS.dmg` and `linux.appimage` stay ABSENT (D76/D86 - absence is the decision, not an oversight). A JSON file cannot carry the upgradeCode warning as a comment; it lives in D86 and the BUILDING.md subsection (Step 4).

- [ ] **Step 2: Create `src-tauri/tauri.bundle.conf.json`** containing exactly (design 3.2):

```json
{
  "bundle": {
    "externalBin": ["binaries/muxsmith"]
  }
}
```

The filename deliberately avoids the auto-merged `tauri.<platform>.conf.json` patterns, so it never applies implicitly; only release builds pass it via `-c` (D82/D84).

- [ ] **Step 3: Add the `.gitignore` line** (design 3.4): one added line, `src-tauri/binaries/`, in the `# JS/Tauri frontend` block (below `src-tauri/gen/`).

- [ ] **Step 4: Append the BUILDING.md subsection** (design 4.4, verbatim) under the `## Building and running` section - insert it at the end of that section, immediately before the `## Tooling quirks` heading:

````markdown
### Reproducing a release bundle locally

Release bundles add the CLI as a bundled sidecar via a build-flavor
overlay (`src-tauri/tauri.bundle.conf.json`); plain `pnpm exec tauri
build` deliberately omits it so dev/test builds need no staging step.
To reproduce what CI ships:

```bash
cargo build --release -p muxsmith-cli
triple="$(rustc -vV | sed -n 's/^host: //p')"
mkdir -p src-tauri/binaries
cp "target/release/muxsmith$( [ "$(uname -o 2>/dev/null)" = Msys ] && echo .exe )" \
   "src-tauri/binaries/muxsmith-$triple$( [ "$(uname -o 2>/dev/null)" = Msys ] && echo .exe )"
pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json
```

Do not change `bundle.windows.wix.upgradeCode` in `tauri.conf.json`:
it is Muxsmith's permanent MSI upgrade identity (design D86).
````

- [ ] **Step 5: Verify**

```bash
python3 -m json.tool src-tauri/tauri.conf.json >/dev/null && python3 -m json.tool src-tauri/tauri.bundle.conf.json >/dev/null && echo json-ok
# Expected: json-ok
jq -r .bundle.publisher src-tauri/tauri.conf.json
# Expected: Şenol Feldmann   (the Ş intact - D86's deliberate orthography)
jq '.bundle.icon | length' src-tauri/tauri.conf.json
# Expected: 5
jq -r '.bundle.targets | join(",")' src-tauri/tauri.conf.json
# Expected: msi,dmg,deb,rpm,appimage
grep -n 'src-tauri/binaries/' .gitignore
# Expected: one hit, inside the JS/Tauri block
```

Run: `cargo check -p muxsmith-gui`
Expected: clean (the build script parses the rewritten config; the overlay is NOT read by normal builds - that absence is D82's point and is exercised only at the rehearsal).

- [ ] **Step 6: Commit**

```bash
git add src-tauri/tauri.conf.json src-tauri/tauri.bundle.conf.json .gitignore BUILDING.md
git -c commit.gpgsign=false commit -m "release: bundle metadata + pinned upgradeCode (D86); CLI sidecar via build-flavor overlay, staging dir ignored, local repro documented (D82)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

