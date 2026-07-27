# Plan 8 design: packaging / release pipeline

Status: DRAFT 2026-07-22, fix round 1 applied 2026-07-23; amendment log at
the end (A1, 2026-07-23). Numbering starts
at **D75** per the ROADMAP Plan-8 kickoff block; D65-D74 is Plan 7.5's
parallel reservation (its design uses D65-D72 - counted from its spec's
`^## D` headings; no collision either way). Last pre-existing ADR is D64
(`2026-07-21-plan7-help-i18n-design.md`).

Scope per the ROADMAP Plan-8 anchor and the S22 kickoff rulings
(`docs/ROADMAP.md`, Plan-8 section, commits 85c0da6 + b5678bd): the seven
S22 kickoff rulings plus the S22 second-round CLI-distribution ruling are
binding and not re-litigated here; each is a D-entry that designs its
mechanics. The remaining D-entries close every fork the brief requires the
design to resolve (workflow architecture, build tooling, runners, bundle
config, version sync, tar.gz leg, naming, checksums, draft body,
rehearsal). **Every fork in this document is closed.** No design-latitude
clause appears in it, in either form (explicit permission or omission);
every set an implementer needs is enumerated, every file content that ships
is written out verbatim.

Grounding: v1 design spec §2 (stack row: "Mature, small bundles, best
packaging") and §10 (CI sentence: "packaging artifacts (msi, dmg, deb, rpm,
AppImage) on release tags"); Tier-2 `docs/process-conventions.yaml`
(`ci-10-pin-everything`, `ci-13-packaging-deferred`, `ci-01-matrix-strategy`,
`proc-latitude-clause-boundary`, `proc-proposed-safeguard-stays`,
`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`),
`docs/conventions.yaml` (`code-comment-line-citations-drift`,
`core-derive-dont-restate`), `docs/product-boundaries.yaml`
(`cross-06-no-bundling-v1`, `cross-01-stack`); `.github/workflows/ci.yml`
(house supply-chain precedents); mkvtoolnix source at
`~/Downloads/mkvtoolnix` and installed `mkvmerge v100.0` (run 2026-07-22).
Every version, runner label, action SHA and schema claim below was
registry-verified live on 2026-07-22 (WebFetch / `gh api` / `curl` of
schema.tauri.app and raw.githubusercontent.com at the installed tool's tag /
local pinned-CLI runs), never from training memory
(`proc-07-verify-against-source`). External-repo code is cited by symbol
anchor plus tag, not bare line numbers, per
`code-comment-line-citations-drift`.

---

## 0. Notes and corrections to the brief

Each checked against the tree or a live source before anything was built on
it (`proc-57-briefs-not-ground-truth`).

| # | Brief statement | Reality |
|---|---|---|
| 1 | "Controller assumption pending owner confirmation" on CLI distribution (GUI-only win/mac installers, CLI via tar.gz + `cargo install`), to be designed as an amendable D-entry | **Superseded mid-authoring by owner ruling** (2026-07-22 S22 second round, controller addendum; verified against `docs/ROADMAP.md` Plan-8 anchor, commit b5678bd "roadmap: plan-8 CLI-distribution ruling + brew-cask v1.x entry"): msi and dmg bundle the CLI **alongside** the GUI, no add-to-PATH installer option, Linux stays one package per format with both binaries, tar.gz carries both, Homebrew Cask is a v1.x ROADMAP entry and not design scope. Designed as settled D82, not as an amendable section. The formerly sanctioned pending item is therefore closed; this document has **zero** owner-pending forks. |
| 2 | Ruling F1 content outline: "macOS Gatekeeper right-click-open / quarantine" | The right-click/Control-click override **no longer exists on current macOS**. Apple (developer.apple.com/news/?id=saqachfa, fetched 2026-07-22): "In macOS Sequoia, users will no longer be able to Control-click to override Gatekeeper when opening software that isn't signed correctly or notarized. They'll need to visit System Settings > Privacy & Security to review security information for software before allowing it to run." D75's documented content leads with the Settings path, keeps Control-click marked as pre-macOS-15 only, and adds the `xattr` quarantine removal as the terminal alternative. This corrects the outline's mechanics with evidence; the ruling itself (unsigned at 1.0, documentation ships) is untouched. |
| 3 | "the `v*` trigger exists in ci.yml today and ... drives only the test matrix" | Confirmed: `ci.yml` `on.push.tags: ['v*']` with no bundling step anywhere (no `tauri build` in any workflow); recorded as KEPT scaffold in the ROADMAP idiomacy-review triage. |
| 4 | tauri.conf.json "bundle block today: targets `all`, icons, no per-OS config, no publisher/category" | Confirmed (`src-tauri/tauri.conf.json` bundle block: `active`, `targets: "all"`, five icons, nothing else). Also confirmed: `version: "0.1.0"` declared there independently, `Cargo.toml` `[workspace.package] version = "0.1.0"` with all crates on `version.workspace = true`, `package.json` `"version": "0.1.0"`. |
| 5 | "windows arm64 msi ... If NOT cleanly supported, propose the closest supported artifact ... and flag NEEDS_CONTEXT" | **Cleanly supported; no NEEDS_CONTEXT needed.** Evidence in section 1 (WiX bundler arch map incl. `arm64`, WiX 3.14.1, template `BUILDARCH=arm64` branch). |

---

## 1. Verified ground truth (all sources fetched/run 2026-07-22)

### 1.1 Tauri 2 config schema (`https://schema.tauri.app/config/2`, the URL `tauri.conf.json` itself pins as `$schema`)

- **`version` fallback exists**: the top-level `version` property's schema
  description: "App version. ... **If removed the version number from
  `Cargo.toml` is used.**" This is D87's mechanism.
- **`bundle.linux.deb.recommends` and `bundle.linux.rpm.recommends` both
  exist** ("The list of deb dependencies your application recommends" /
  "The list of RPM dependencies your application recommends"). Ruling 6 is
  implementable natively; **no post-processing step is needed** and none is
  designed.
- **`BundleType` enum is exactly** `deb`, `rpm`, `appimage`, `msi`, `nsis`,
  `app`, `dmg` (7 values). There is **no tar.gz bundler**, confirming the
  brief; D88 designs the packing step.
- `bundle` carries `category` (closed string list incl. `Video` and
  `Utility`), `copyright`, `publisher`, `homepage`, `license`/`licenseFile`,
  `shortDescription`, `longDescription`, `createUpdaterArtifacts`
  ("Produce updaters and their signatures or not"), `externalBin`, `targets`
  (`"all"` | array | single).
- **`externalBin`** resolves paths by the sidecar pattern
  "binary-name{-target-triple}{.system-extension}" (schema description
  quotes the exact lookup examples). D82's mechanism.
- **`bundle.windows.wix.upgradeCode`**: "must stay the same across all of
  your updates ... By default, tauri generates this code by generating a
  Uuid v5 using the string `<productName>.exe.app.x64` in the DNS
  namespace ... It is recommended that you set this value in your tauri
  config file." Note the derivation string is the literal `.x64` regardless
  of build arch, so the default is already arch-independent; D86 pins it.
- `bundle.windows.wix.language` (installer UI languages), `wix.version`
  (MSI version override) exist; `bundle.macOS.minimumSystemVersion`
  "Defaults to `10.13`"; `bundle.macOS.dmg` carries only presentation
  fields (`appPosition`, `applicationFolderPosition`, `background`,
  `windowPosition`, `windowSize`).
- **`mainBinaryName`**: "By default, Tauri uses the output binary from
  `cargo`" - i.e. without this option the bundled main binary keeps its
  cargo name. Our GUI crate's `[[bin]]` is `muxsmith-gui`
  (`src-tauri/Cargo.toml`), so bundles carry `muxsmith-gui(.exe)` as the
  main executable and the CLI sidecar `muxsmith(.exe)` **cannot collide
  with it**, not even on case-insensitive filesystems. (Had the two names
  matched, `tauri-build`'s `copy_binaries` errors explicitly: "Cannot
  define a sidecar with the same name as the Cargo package name" -
  `crates/tauri-build/src/lib.rs`, fn `copy_binaries`, tag
  `tauri-cli-v2.11.4`.)

### 1.2 tauri-bundler / tauri-cli source at the installed version (tag `tauri-cli-v2.11.4` on github.com/tauri-apps/tauri; local CLI reports `tauri-cli 2.11.4`, run 2026-07-22)

- **Windows arm64 msi is supported.** `bundle/windows/msi/mod.rs`, fn
  `build_wix_app_installer`: the arch match maps `Arch::AArch64 => "arm64"`
  (alongside `x64`/`x86`; anything else errors "unsupported architecture"),
  and the same mapping feeds candle's `-arch` argument (fn `run_candle`).
  The bundled WiX is **v3.14.1** (`WIX_URL` constant:
  `wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip`).
  The WiX template (`main.wxs`) has an explicit
  `<?elseif $(sys.BUILDARCH)="arm64"?>` branch mapping to
  `ProgramFiles64Folder`. Two stale comments in **`mod.rs`** (not the
  template) still contradict the arm64-handling code beneath them: the
  doc line above `pub fn build_wix_app_installer`, "For now the only
  supported platform is Windows x64.", and the later
  "// target only supports x64." inside that function - both re-copied
  verbatim from the tag-pinned file 2026-07-23 (grep "only support" over
  it: exactly these two hits; `main.wxs` has none). Corroborating docs (v2.tauri.app/distribute/windows-installer,
  fetched 2026-07-22): ".msi installers can only be created on Windows",
  ARM64 build instructions via `--target aarch64-pc-windows-msvc`; the
  page's x86-emulation caveat concerns the NSIS **installer stub**, not
  MSI.
- **msi content and options**: `main.wxs` installs `perMachine`
  (`InstallScope="perMachine"`) to `ProgramFiles64Folder\<productName>`
  with `WixUI_InstallDir` (install-dir dialog only, **no feature-selection
  UI**). External binaries are emitted as components in `INSTALLDIR`
  beside the main exe (`generate_binaries_data` strips the target-triple
  suffix; template loop `{{#each binaries}}`). The template contains a
  Feature titled "PATH Environment Variable" (`PathEnvVarFeature` locale
  string) but **no `<Environment>` element anywhere in `main.wxs`** (grep
  with positive control: `Path=`/`SetOutPath`-class patterns fire, no
  Environment element hit), and `WixUI_InstallDir` exposes no feature
  chooser - so the stock msi has **no functional add-to-PATH option**,
  which is exactly what the D82 ruling requires. A plain-text
  `licenseFile` is auto-converted to RTF (fn around the
  `license.ends_with(".rtf")` branch).
- **macOS**: `bundle/macos/app.rs` copies external binaries into
  `Contents/MacOS` with the triple suffix stripped
  (`settings.copy_binaries`, `Settings::copy_binaries` in `settings.rs`)
  and the app's own binaries by name (`copy_binaries_to_bundle`);
  `CFBundleExecutable` = main binary name. The **dmg bundler builds the
  .app itself** ("generate the .app bundle if needed",
  `bundle/macos/dmg/mod.rs`, fn `bundle_project`), so a `targets` list
  without `app` still produces a correct dmg.
- **deb/rpm**: both place app binaries **and** external binaries in
  `/usr/bin` (`bundle/linux/debian.rs` fn `generate_data`;
  `bundle/linux/rpm.rs` "Add external binaries" block). The deb control
  file writes `Recommends:` from config (fn `generate_control_file`).
  **tauri-cli auto-injects the system Depends**: `interface/rust.rs`
  pushes `libwebkit2gtk-4.1-0` and `libgtk-3-0` into deb Depends and
  `libwebkit2gtk-4.1.so.0()(64bit)` / `libgtk-3.so.0()(64bit)` into rpm
  Requires - our config adds only the mkvtoolnix Recommends on top.
- **AppImage**: `bundle/linux/appimage/linuxdeploy.rs` reuses
  `debian::generate_data` for the AppDir, so the external CLI binary rides
  along at `usr/bin/` inside the image; webkit is bundled into the image
  (webkit2gtk-4.1 injected-bundle handling in the same file); output name
  is `{productName}_{version}_{arch}.AppImage`.
- **Local pinned CLI runs** (`./node_modules/.bin/tauri`, 2.11.4):
  `tauri build --help` documents `-b, --bundles [<BUNDLES>...]` and
  `-c, --config <CONFIG>` ("JSON strings or paths to JSON, JSON5 or TOML
  files to merge with the default configuration file ... use this for more
  specific use cases such as different build flavors") - D82's overlay
  mechanism and D84's per-leg `--bundles` selection.
  `tauri inspect wix-upgrade-code` printed:
  `Default WiX Upgrade Code, derived from Muxsmith: 9262b417-b687-5ea3-ace1-18b9d51b215f`.

### 1.3 GitHub-hosted runners (docs.github.com "GitHub-hosted runners" reference, fetched 2026-07-22)

Standard runner labels relevant here, architecture as listed by GitHub:

| label | arch | status |
|---|---|---|
| `windows-2025` | x64 | standard (house test-matrix pin) |
| `windows-11-arm` | arm64 | standard, GA (4-CPU on public repos; the only GA windows-arm64 label - no dated variant exists) |
| `macos-15` | arm64 (M1) | standard (house test-matrix pin) |
| `macos-15-intel` / `macos-26-intel` | x64 | the only remaining Intel macOS labels (relevant to the registered Intel-dmg trigger, not to this design) |
| `ubuntu-22.04` | x64 | standard, still offered |
| `ubuntu-24.04` | x64 | standard |
| `ubuntu-26.04` | x64 | **public preview** (matches the ci.yml comment) |

Standard runners on public repos are free (house premise already recorded
in the ci.yml matrix comment since go-public). The `windows-11-arm` image
inventory (actions/partner-runner-images `images/arm-windows-11-image.md`,
fetched 2026-07-22; repo carries a notice that image management moves into
actions/runner-images and the partner repo archives June 2026): Rustup
1.28.2 + Rust 1.92 preinstalled (rustup then installs our pinned 1.96.1
from `rust-toolchain.toml`), Visual Studio Enterprise 2022 (MSVC arm64),
Node 24, Chocolatey. Rust `aarch64-pc-windows-msvc` is **Tier 1 with host
tools** (doc.rust-lang.org/rustc/platform-support.html, fetched
2026-07-22). Node 26.5.0 ships `win-arm64` archives
(nodejs.org/dist/v26.5.0/ lists `node-v26.5.0-win-arm64.zip`).

### 1.4 Actions and their pins (`gh api .../releases/latest` + `.../commits/<tag>`, 2026-07-22)

| action | latest release | release commit SHA (the pin) |
|---|---|---|
| `actions/setup-node` | v7.0.0 (2026-07-14) | `820762786026740c76f36085b0efc47a31fe5020` |
| `pnpm/action-setup` | v6.0.9 (2026-06-15) | `0ebf47130e4866e96fce0953f49152a61190b271` |
| `actions/upload-artifact` | v7.0.1 (2026-04-10) | `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` |
| `actions/download-artifact` | v8.0.1 (2026-03-11) | `3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c` |
| `tauri-apps/tauri-action` | action-v1.0.0 (2026-06-29) | `1deb371b0cd8bd54025b384f1cd735e725c4060f` (recorded for D84's rejection analysis; not used) |
| `actions/checkout` | (house pin reused) | `9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0` # v7.0.0, already in ci.yml |

`pnpm/action-setup` with `version` omitted reads the `packageManager`
field from `package.json` (its README: "Optional when there is a
`packageManager` field in the `package.json`"), which pins pnpm 11.10.0
from the existing single source. tauri-action health, for the record:
repo pushed 2026-07-20, 1587 stars, not archived, v1.0.0 shipped
2026-06-29 - the rejection in D84 is on fit, not on health.

### 1.5 Release-operations mechanics (gh 2.94.0 local `--help` runs + GitHub REST docs, 2026-07-22)

- `gh release create` supports `--draft`, `--verify-tag` ("Abort in case
  the git tag doesn't already exist in the remote repository"),
  `--notes-file`, `--target <branch-or-SHA>`; its immutable-releases note
  confirms drafts stay modifiable/deletable. `gh run list` supports
  `-c, --commit SHA` and `-w, --workflow` filters (D83's gate check).
- REST `POST /repos/{owner}/{repo}/releases/generate-notes` returns
  generated markdown and "the generated release notes are not saved
  anywhere" - a pure computation endpoint (docs.github.com REST releases
  page). D77 composes the draft body with it.
- **A draft release does not create the tag ref**; the tag materializes at
  publish (GitHub community discussion #24690; the REST docs do not state
  it crisply, so the rehearsal checklist verifies it empirically with a
  positive control - section 8, step R7).
- SmartScreen dialog wording used by D75/`docs/INSTALL.md` - heading
  "Windows protected your PC", link "More info", button "Run anyway" -
  verified against current descriptions (learn.microsoft.com Q&A and
  multiple KB corroborations, searched 2026-07-22).
- Tauri's own CI guidance (v2.tauri.app/distribute/pipelines/github,
  fetched 2026-07-22) recommends tauri-action with `releaseDraft: true`
  and builds its Linux leg on **`ubuntu-22.04`** - the ecosystem idiom
  D85 follows for the compat floor even while D84 rejects the action
  itself. The AppImage guide (v2.tauri.app/distribute/appimage) is
  explicit: "you must build your Tauri application using the oldest base
  system you intend to support", naming Ubuntu 22.04/Debian 12 as the
  baselines providing `libwebkit2gtk-4.1-dev`.

### 1.6 mkvtoolnix (SI-3 sources: mkvtoolnix.download/downloads.html fetched 2026-07-22; source tree `~/Downloads/mkvtoolnix` at v100)

Full audit in section 5. Facts used by D-entries: Windows ships installer
(`mkvtoolnix-64-bit-<v>-setup.exe`) **and** portable 7z archives; macOS
ships DMGs; Linux ships distro repos plus a "distribution-agnostic
AppImage" (glibc 2.28+). The NSIS installer ships **every** tool
(`packaging/windows/installer/mkvtoolnix.nsi`: `File "../*.exe"`) and
performs **no PATH manipulation** (grep for `EnvVar`/`Environment`/PATH
writes: nothing; positive control: the same grep form hits `SetOutPath`
staging lines). The dmg carries all five binaries in `Contents/MacOS`
(`packaging/macos/build.sh`: `strip ${dmgcnt}/MacOS/mkv{merge,info,extract,propedit,toolnix-gui}`;
its README text even instructs running "mkvmerge and mkvpropedit from
./MKVToolNix-<v>.app/Contents/MacOS/"). Debian packaging splits CLI and
GUI (`packaging/debian/control.erb`: `Package: mkvtoolnix` +
`Package: mkvtoolnix-gui`, `Section: video`), and the rpm spec does the
same (`Name: mkvtoolnix` + `%package gui`); Fedora ships the split as
`mkvtoolnix` / `mkvtoolnix-gui` (packages.fedoraproject.org, fetched
2026-07-22). Licensing boundary: only facts were taken from the
mkvtoolnix tree, no text or asset is copied.

### 1.7 Local tree facts

- Binaries: CLI `[[bin]] name = "muxsmith"`
  (`crates/muxsmith-cli/Cargo.toml`), GUI `[[bin]] name = "muxsmith-gui"`
  (`src-tauri/Cargo.toml`); CLI self-reports its version
  (`crates/muxsmith-cli/src/cli.rs:10`:
  `#[command(name = "muxsmith", version, about)]`, clap wires
  `--version` to the workspace version) - D87's artifact-level check.
- `README.md` carries four `placeholder(1.0)` comments; the release one
  reads `<!-- placeholder(1.0): release artifacts per OS (msi/dmg/deb/rpm/AppImage) once the packaging pipeline lands -->`
  (D75's landing hook).
- `.gitignore` has no entry for a sidecar staging dir (D82 adds one);
  `LICENSE` (MIT) sits at the repo root; all five icons exist under
  `src-tauri/icons/`.
- `mise.toml` pins node 26.5.0 + pnpm 11.10.0; `package.json`
  `packageManager: pnpm@11.10.0`.

---

## D75: Unsigned artifacts at 1.0; install-hurdle documentation lives in `docs/INSTALL.md`, linked from release notes and (at tag) from the README placeholder

**Owner ruling (S22, binding, not re-litigated):** all three OS ship
unsigned at 1.0; signing revisit is the already-registered ROADMAP trigger
("First external-user complaint about unsigned-install hurdles ...") and
not design scope.

**Decision (the design part: WHERE and WHAT).** A new committed file
`docs/INSTALL.md` is the single home of the per-OS install documentation.
Consumers: (a) the draft-release body template links its per-OS anchors
(D77); (b) at the 1.0 tag, the README `placeholder(1.0)` release-artifacts
comment resolves into an Install section that lists the artifact per OS
and links `docs/INSTALL.md` - Plan 8 updates the placeholder comment's
text to name that consumption (a rider edit, not a resolution; resolution
stays an at-tag item per the ROADMAP README entry); (c) the tar.gz README
(D88) restates only the Linux runtime requirements and points to the same
file. Content: section 4.1 carries the full verbatim file. Outline per the
ruling, with the macOS mechanics corrected per brief-note 2:

- **Windows**: SmartScreen interception ("Windows protected your PC" ->
  "More info" -> "Run anyway"), unknown-publisher framing, per-machine
  install to `C:\Program Files\Muxsmith` (verified template default),
  manual PATH step for the CLI.
- **macOS**: unsigned and not notarized; macOS 15+: first open is blocked,
  then System Settings > Privacy & Security > "Open Anyway" (Apple
  citation in section 1); macOS 11-14: Control-click -> Open; alternative
  for terminal users: `xattr -d com.apple.quarantine`; CLI location inside
  the app bundle and the manual PATH step.
- **Linux**: no gatekeeping hurdle (ruling text: "Linux none"); AppImage
  `chmod +x`; mkvtoolnix runtime installation per distro; deb/rpm already
  place both binaries on PATH.

**Rationale.** One file, three consumers: the release notes need a stable
link target that exists before the first tag; the README placeholder
system already reserves the at-tag landing spot; duplicating the text
into the release body wholesale would fork it (say-once). `docs/` is where
the repo keeps user-facing standing documents (BUILDING.md precedent:
"single source ... until the public README absorbs this at 1.0").

**Rejected: README-only (write the section into README now).** Steelman:
one fewer file; the README is where users look first; the placeholder
exists precisely for this content. Rejected: the README describes the
current WIP state and its placeholder system resolves **at tag** (ROADMAP:
"Remaining at the 1.0 tag: resolve the four placeholder(1.0) comments");
landing install instructions for artifacts that do not exist yet would
make the README lie for the whole pre-tag window, and release notes would
have no stable anchor until then.

**Rejected: release-notes-only (no committed file).** Steelman: the notes
travel with the artifacts, and the owner edits drafts anyway. Rejected:
per-release duplication drifts (every release re-states the same hurdles),
and the README placeholder would have nothing to link at tag time.

**Interface changes:** new file `docs/INSTALL.md`; one comment-text edit
to `README.md` (the placeholder rider); release-body template links
(D77). Trigger interaction: when the registered signing trigger fires and
signing lands, `docs/INSTALL.md` shrinks per OS - recorded there as an
HTML comment so the file names its own obsolescence condition.

---

## D76: No auto-updater at 1.0; the bundle config must not require updater artifacts or keys

**Owner ruling (S22, binding).** v1.x item.

**Decision.** `bundle.createUpdaterArtifacts` stays **absent** from
`tauri.conf.json` (schema default `false`, verified; the field's only
purpose is "Produce updaters and their signatures or not"). No
`plugins.updater` config, no `TAURI_SIGNING_PRIVATE_KEY` secret exists
anywhere in the workflow, and D77's release job attaches exactly D89's
enumerated asset set - no `.sig`, no `latest.json`. A dedicated step in
every bundle leg ("Assert no updater artifacts were produced", section
2) fails the leg if the bundle output tree contains a file matching the
two enumerated patterns `*.sig` / `latest.json`, with a built-in
positive control (it first asserts the tree exists and is non-empty);
rehearsal checklist step R5 reads that step's log, and its red and
control-red states are fire-verified pre-merge (section 8, G4).

**Rationale.** The ruling; and structurally, updater keys are exactly the
kind of long-lived secret the unsigned-at-1.0 posture avoids holding.

**Rejected: pre-provisioning updater config behind a flag** ("it is
cheap to add the config now, disabled"). Steelman: the v1.x updater then
lands as a one-line flip. Rejected: dead config is a standing lie
(`yagni` dimension of the idiomacy review); the v1.x updater work will
need its own design (key custody, endpoints) anyway, and a disabled block
would still tempt every future config edit to "just enable it".

**Interface changes:** none (absence is the interface).

---

## D77: A `v*` tag builds all bundles and attaches them to a DRAFT release; the owner publishes manually; the draft body is a fixed template plus GitHub-generated notes

**Owner ruling (S22, binding):** never auto-publish; owner reviews and
publishes.

**Decision.** The `release.yml` assemble job (section 2) creates the
release with `gh release create "$TAG" --draft --verify-tag` - `--draft`
implements the ruling, `--verify-tag` makes accidental tag creation from
the default branch impossible (flag semantics verified, section 1.5).
Nothing in the workflow publishes, edits, or un-drafts a release; there
is no `gh release edit --draft=false` anywhere. Publishing is a human
click in the GitHub UI after the owner's review.

The draft body = the fixed template of section 4.2 (install-note digest
linking `docs/INSTALL.md` anchors, runtime-requirement note, artifact
table, checksum-verification line) with the version substituted, followed
by GitHub's generated notes fetched via
`gh api "repos/$GITHUB_REPOSITORY/releases/generate-notes" -f tag_name=...`
and appended (endpoint verified side-effect-free, section 1.5). The
generated part contributes the commit/changelog digest ("Full Changelog"
compare link) without any hand-rolled git-log formatting; the owner
rewrites prose freely in the draft.

**Rationale.** `gh` is first-party, preinstalled on every GitHub-hosted
runner, and draft-first is the create command's own native flow when
assets are attached - its help: "When using the `create` command to
attach assets to a release, separate API calls are made to create the
release as a draft, upload the assets, and then publish the release."
(re-copied verbatim from `gh release create --help`, gh 2.94.0,
2026-07-23); the same help's immutable-releases section confirms drafts
stay modifiable and deletable until publish (paraphrase). Composing the body
workflow-side (template + API call) instead of via `--generate-notes`
keeps one deterministic mechanism: the help text documents prepending
only for `--notes`, not `--notes-file`, and a two-flag interaction we
cannot verify locally has no place in a release path (the API-call
composition is verifiable and rehearsed).

**Rejected: `softprops/action-gh-release`.** Steelman: the most-used
release action, declarative asset globs, idempotent re-runs, SHA-pinnable.
Rejected: a third-party action with its own release cadence and pin
maintenance, replacing three `gh` commands against a first-party tool that
ships on the runner image; pin-everything prefers fewer moving third-party
parts, and D84 already rejects the sibling convenience action on the same
grounds.

**Rejected: owner-authored body only (empty generated baseline).**
Steelman: the owner rewrites the body anyway; generated notes on a
repo with direct pushes (no PR flow) are thin. Rejected: the ruling's
review model wants the draft to arrive reviewable, not empty; the
compare link and commit list cost nothing and the template's fixed part
carries the load-bearing content (install notes, checksums).

**Interface changes:** release body structure (section 4.2) becomes the
user-visible release-notes contract; `permissions: contents: write` on
exactly one job (assemble), matching the house least-privilege precedent
(ci.yml holds `contents: read` workflow-wide).

---

## D78: The artifact matrix, verified: windows x64 msi + windows arm64 msi + macOS arm64 dmg + Linux x64 deb/rpm/AppImage/tar.gz

**Owner ruling (S22, binding):** exactly this matrix; no macOS x64 leg
(Intel request is the registered ROADMAP trigger); no NSIS; no other
formats.

**Decision.** Seven artifacts per release, built on four legs (D85), named
per D89. The load-bearing verification the ruling demanded:

- **windows-arm64 msi**: supported by the installed toolchain - bundler
  arch map `Arch::AArch64 => "arm64"` feeding candle `-arch arm64`, WiX
  3.14.1, template `BUILDARCH=arm64` branch (section 1.2, all at tag
  `tauri-cli-v2.11.4`). Built natively on the GA `windows-11-arm` runner
  (section 1.3), Rust target Tier 1 with host tools. **No substitute
  artifact is needed.**
- **Runner labels** for all four legs exist as standard GitHub-hosted
  runners (section 1.3 table).
- deb/rpm/AppImage/tar.gz: Linux x64 only, per the ruling; the AppImage
  precedent (mkvtoolnix ships one, glibc 2.28+) and the tar.gz design are
  D88/D85.

**Rationale.** The ruling. The one deliberate asymmetry worth recording:
Windows gets two arches while macOS gets one because macOS arm64 covers
every Mac Apple still sells and the Intel remainder has a registered
trigger, whereas Windows-on-ARM devices cannot run the x64 msi at all
(the x64 emulator runs apps, but an x64 **perMachine msi** installs a
permanently-emulated app on hardware we can build natively for at zero
marginal cost).

**Rejected: NSIS instead of (or beside) msi.** Steelman: Tauri's NSIS
target is newer-generation, supports more compression and per-user
installs, and mkvtoolnix itself uses NSIS. Rejected: the ruling names msi;
msi is the Windows enterprise-deployable format (GPO/Intune), and one
installer format per OS keeps the matrix at the ruled seven artifacts.
The mkvtoolnix NSIS precedent transfers at the level that matters (ship
every tool in one installer - D82), not at the format level.

**Rejected: universal macOS binary (`universal-apple-darwin`).**
Steelman: one dmg serves both arches; Tauri supports the target natively.
Rejected: it contradicts the ruling (macOS arm64 ONLY, Intel via
trigger), doubles binary size for every arm64 user, and requires both
Rust std targets on the runner for zero ruled benefit.

**Interface changes:** the artifact set (7 files + SHA256SUMS = 8 release
assets) is the public release contract; enumerated with names in D89.

---

## D79: Pipeline verification via `workflow_dispatch` with a `rehearse-draft-release` input; never a test tag

**Owner ruling (S22, binding):** verification runs as workflow_dispatch
building all legs as workflow artifacts, plus a rehearsal input that
exercises draft-release creation; a test tag is banned.

**Decision.** `release.yml` has exactly two triggers: `push.tags: ['v*']`
and `workflow_dispatch` with one boolean input `rehearse-draft-release`
(default `false`). **Shared job definitions, no forked copy** - the same
guard, the same four build legs and the same assemble job run on both
triggers (reuse-before-writing applied to workflow YAML, as the brief
directs); the only conditional points are:

1. guard: the tag-equality arm of the version check runs only on the tag
   path (`github.ref_type == 'tag'`);
2. assemble: the release-creation step runs when
   `github.ref_type == 'tag'` **or** `inputs.rehearse-draft-release`;
   otherwise the dispatch run ends at workflow artifacts - the assemble
   job still generates and logs SHA256SUMS, but it is persisted (as a
   release asset) only on the draft path.

The rehearsal draft is named `rehearsal-<run_id>` (deliberately **not**
matching `v*`, so no tooling that keys on the tag scheme can confuse it),
created with `--target "$GITHUB_SHA"` and **without** `--verify-tag` (the
tag deliberately does not exist; a draft creates no ref - section 1.5,
plus the empirical check in section 8). Its body is the real template
prefixed by the rehearsal warning block (section 4.2). The owner inspects
and then deletes the rehearsal draft (checklist step; drafts are
deletable, verified).

**Rationale.** The ruling bans the one obvious alternative (test tag) for
a good reason: a `v*` test tag would trip the real tag path, the ci.yml
tag trigger, and any future consumer of the tag namespace. The
shared-jobs shape means the rehearsal exercises byte-identical build
logic; the only untested lines on the tag path are the two conditionals
above, which the first real tag exercises under the owner's eyes (draft,
so still no publish risk).

**Rejected: a separate rehearsal workflow file.** Steelman: zero
conditionals in release.yml; the rehearsal cannot possibly touch the real
path. Rejected: it IS a forked copy - the brief names that exact shape as
the thing to avoid; two files drift, and the rehearsal then verifies the
copy, not the pipeline.

**Rejected: rehearsal auto-deletes its draft.** Steelman: no manual
cleanup, no lingering drafts. Rejected: the draft IS the rehearsal's
deliverable - the owner reviews its body, asset list and checksums; a
self-deleting artifact verifies nothing anyone saw.

**Interface changes:** the dispatch input name `rehearse-draft-release`
and the `rehearsal-<run_id>` draft-name scheme are operator-facing
contract; recorded in section 8's checklist.

---

## D80: deb and rpm declare `mkvtoolnix` as Recommends; AppImage and tar.gz document the runtime requirement

**Owner ruling (S22, binding):** Recommends at 1.0; hard Depends is the
recorded v1.x entry.

**Decision.** `bundle.linux.deb.recommends: ["mkvtoolnix"]` and
`bundle.linux.rpm.recommends: ["mkvtoolnix"]` in `tauri.conf.json` - both
fields verified in the live schema (section 1.1), so **no post-processing
step exists or is designed** (the brief's contingency is moot). The
package name `mkvtoolnix` is verified on both sides: Debian/Ubuntu (the
house ci.yml installs `mkvtoolnix=97.0-1build1` via apt on every Linux
run) and Fedora (packages.fedoraproject.org, section 1.6). AppImage and
tar.gz carry no dependency metadata by nature; their documentation duty
lands in `docs/INSTALL.md`'s Linux section and the tar.gz README
(section 4.3), both naming mkvtoolnix/mkvmerge and how to install it.
The rehearsal verifies the built packages' metadata
(`dpkg-deb -I`: `Recommends: mkvtoolnix` plus the auto-injected
`Depends: libwebkit2gtk-4.1-0, libgtk-3-0`; `rpm -qp --recommends`) -
section 8, step R6.

**Rationale.** The ruling, plus the verified fact that Tauri auto-injects
the webkit/gtk hard Depends (section 1.2): the app genuinely cannot start
without those, while it CAN start without mkvmerge (first-run detection
and the absent-mkvmerge guidance already ship - the ruling's recorded
rationale). The v1.x hard-Depends promotion is a two-token change
(`recommends` -> `depends` merge), noted in the v1.x ROADMAP entry's
context.

**Rejected: `Suggests` instead of `Recommends`.** Steelman: Muxsmith
"works" as a profile editor without mkvmerge, and Suggests avoids
apt's default auto-install of Recommends for users who only want to edit
profiles. Rejected: muxing IS the product (spec §1); a default install
that cannot mux serves nobody, and apt's Recommends-by-default is exactly
the behavior the owner chose short of a hard dependency.

**Interface changes:** deb/rpm metadata is user-visible package contract;
enumerated strings above are exhaustive (no other weak dependencies).

---

## D81: Plan 8 builds the pipeline; tagging 1.0 is out of scope

**Owner ruling (S22, binding).**

**Decision.** No task of this plan creates a `v*` tag, publishes any
release, or resolves the README `placeholder(1.0)` comments (D75's rider
edit changes a comment's text, not its placeholder status). The plan's
end state: `release.yml` merged and rehearsed via workflow_dispatch
(section 8 checklist green), config and docs landed, ROADMAP triggers
mirrored. The first real tag is a separate owner decision with its own
gate (doctrine §3 milestone consult of the Triggers list - among them the
mkvmerge-version-bump and signing entries).

**Rationale.** The ruling; and the rehearsal path exists precisely so the
pipeline is proven without the tag.

**Interface changes:** none.

---

## D82: msi and dmg bundle the CLI beside the GUI via `externalBin` in a build-flavor overlay config; Linux packages carry both binaries; no add-to-PATH option anywhere

**Owner ruling (S22 second round, binding; ROADMAP commit b5678bd):**
msi and dmg ship the CLI binary alongside the GUI (mkvtoolnix parity:
`File "../*.exe"` in its NSIS script, all binaries in its dmg's
`Contents/MacOS` - both verified in section 1.6); NO add-to-PATH
installer option (mkvtoolnix has none either - verified negative with
positive control, section 1.6); Linux stays ONE package per format with
both binaries (mkvtoolnix's deb/rpm CLI/GUI split is a recorded
deliberate divergence); tar.gz carries both; Homebrew Cask is a v1.x
ROADMAP entry, not design scope.

**Decision - mechanism.** The CLI ships as a Tauri **external binary**
(sidecar used purely as a packaging vehicle; nothing spawns it, so no
shell-plugin capability is added):

1. A committed overlay `src-tauri/tauri.bundle.conf.json` containing
   exactly:

   ```json
   {
     "bundle": {
       "externalBin": ["binaries/muxsmith"]
     }
   }
   ```

   applied only by release builds via
   `pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json ...`
   (the `-c` merge is the CLI's documented build-flavor mechanism,
   section 1.2). The name deliberately avoids the auto-merged
   `tauri.<platform>.conf.json` filenames, so it never applies
   implicitly.
2. Each build leg first runs
   `cargo build --release -p muxsmith-cli`, then stages the binary at the
   sidecar lookup path:
   `src-tauri/binaries/muxsmith-<host-triple><.exe on windows>` (triple
   from `rustc -vV`; all four legs build natively, so host triple ==
   target triple everywhere). `src-tauri/binaries/` is added to
   `.gitignore` (build artifact, never committed).
3. Landing spots (all verified against the bundler at the installed tag,
   section 1.2): msi installs `muxsmith.exe` into `INSTALLDIR` beside
   `muxsmith-gui.exe`; the .app inside the dmg carries
   `Contents/MacOS/muxsmith` beside `Contents/MacOS/muxsmith-gui`
   (exactly mkvtoolnix's dmg layout); deb/rpm install `/usr/bin/muxsmith`
   and `/usr/bin/muxsmith-gui`; the AppImage carries `usr/bin/muxsmith`
   internally (a structural rider of the shared deb data dir - inert but
   harmless; the supported Linux CLI channels remain deb, rpm and
   tar.gz). No name collision exists anywhere because the GUI binary is
   `muxsmith-gui`, not `Muxsmith` (section 1.1, mainBinaryName fact).
4. PATH: no installer option, matching the ruling and the verified
   inertness of the stock WiX template's PATH pseudo-feature
   (section 1.2). The manual PATH step per OS is documented in
   `docs/INSTALL.md` (section 4.1): Windows - add the install folder to
   the user PATH; macOS - `sudo ln -s` the bundle binary into
   `/usr/local/bin` (or add the `Contents/MacOS` dir to PATH); Linux
   deb/rpm - none needed (`/usr/bin`); tar.gz - run from the extraction
   dir or link it.

**Why the overlay instead of putting `externalBin` in the base config:**
`tauri-build` processes `externalBin` on **every** `src-tauri` compile
(fn `copy_binaries`, section 1.1) - in the base config it would make
every `cargo test --workspace`, every `pnpm dev` and every CI test leg
depend on a pre-staged sidecar file that only release builds need.
The overlay keeps the dev/test surface byte-identical to today
(config absent -> the whole block is skipped) and scopes the new build
requirement to the four release legs plus the documented local
reproduction (BUILDING.md gets a short "Reproducing a release bundle
locally" subsection naming the two commands of steps 1-2; content in
section 4.4).

**Rejected: GUI-only win/mac installers (the brief's superseded
assumption).** Steelman: today's bundle state needs zero new mechanism;
`cargo install` serves CLI users on every OS; a second binary doubles
installer size for GUI-only users; and the sidecar staging step is a new
failure mode in every release build. Rejected by owner ruling, on
mkvtoolnix parity: the direct precedent ships every tool in one
installer, and a Windows/macOS CLI user without a Rust toolchain
otherwise has NO supported channel at all (the tar.gz is Linux-only) -
the assumption's "trigger candidate" answer would have shipped 1.0 with
that hole.

**Rejected: add-to-PATH installer option.** Steelman: a CLI you cannot
call from the shell without manual PATH surgery is half-shipped, and
Windows users in particular rarely edit PATH; WiX could do it with one
`<Environment>` element in a custom template. Rejected by owner ruling,
mkvtoolnix parity again (none of its installers touch PATH); PATH writes
are the single most complained-about installer side effect, a custom WiX
template forks Tauri's maintained template forever (every tauri-cli bump
would need a template re-diff), and the documented manual step serves the
minority who want the CLI on PATH without imposing on everyone else.

**Rejected: mkvtoolnix's Linux CLI/GUI package split (deb/rpm
`muxsmith` + `muxsmith-gui` packages).** Steelman: it is the distro-idiomatic
shape - headless servers install the CLI without pulling webkit/gtk;
mkvtoolnix, and most CLI+GUI suites, package exactly this way; our single
deb hard-Depends on `libwebkit2gtk-4.1-0` (auto-injected, section 1.2)
forces the whole webview stack onto CLI-only machines. Rejected by owner
ruling (recorded divergence): Tauri's bundler produces one package per
invocation and a second CLI-only package would be a hand-rolled parallel
packaging pipeline (exactly the bulk the tar.gz already covers for
webkit-averse users at zero packaging cost); at Muxsmith's scale one
package is the honest size of the product, and the split can be revisited
if distro users ever ask (that request is observable).

**Rejected: `bundle.resources` as the packaging vehicle.** Steelman:
resources support subdirectories, avoiding any collision concern by
construction. Rejected: resources land in `Contents/Resources` (macOS) /
resource dirs, are not the platform's executable location, give the msi
no component GUID handling for an exe, and `externalBin` is the
purpose-built field whose landing spots are exactly the ruled ones
(verified, section 1.2).

**Interface changes:** new committed file `src-tauri/tauri.bundle.conf.json`;
`.gitignore` gains `src-tauri/binaries/`; BUILDING.md gains the local
reproduction subsection; the installed-file set per artifact (step 3
above) is user-visible contract, restated in `docs/INSTALL.md`.
Not in scope, per the same ruling: Homebrew Cask (v1.x ROADMAP entry -
noted here once so this design is not read as having rejected it on the
merits).

---

## D83: A separate `release.yml`; ci.yml is not modified; the guard job requires the ci gate green on the same SHA before any leg builds

**Decision.** The pipeline is a new workflow file
`.github/workflows/release.yml` (section 2). `ci.yml` is **not touched**:
its `v*` tag trigger keeps running the full test matrix + deny job on the
tagged SHA - that run IS how the KEPT scaffold is consumed (the scaffold
guarantees a gate verdict exists for every tag; Plan 8 adds the consumer
of that verdict). `release.yml`'s first job (`guard`) does two things,
in order:

1. **Version sync** (D87's script).
2. **Gate-green check**: polls
   `gh run list --workflow ci.yml --commit "$GITHUB_SHA" --json status,conclusion`
   (flags verified, section 1.5) until at least one ci.yml run for this
   exact SHA reports `completed`/`success`; fails the workflow if a
   completed run concludes red and none succeeds, or on a 45-minute
   timeout (the 3-OS matrix completes well inside that; the timeout is
   the fail-safe against a never-started run). All four build legs
   `needs: guard`; the assemble job `needs: [guard, bundle]`. A tag on a
   red or untested SHA therefore produces **no bundles and no draft**,
   mechanically.

**Rationale.** Separation is earned by three structural differences, not
taste: (a) permissions - ci.yml is `contents: read` workflow-wide (house
least-privilege precedent, its own comment cites the verified repo
default); the release path needs `contents: write` on exactly one job,
which in a merged file would either elevate the shared workflow surface
or scatter per-job overrides through the test matrix; (b) trigger
semantics - release adds a `workflow_dispatch` **input**; ci.yml's
existing plain dispatch (used for re-runs) would gain a
release-rehearsal input it must ignore on every test dispatch; (c)
runners - the release legs deliberately pin different images than the
test matrix (D85), and a merged matrix would carry per-job `if:`
conditions on every leg. The gate-green check makes the release path
depend on the gate verdict instead of duplicating the gate.

**Rejected: extend ci.yml with conditional release jobs.** Steelman: one
file, one trigger set, the `v*` trigger already lives there, shared
checkout/toolchain steps amortize, and `needs:` inside one workflow is a
by-construction same-SHA guarantee with zero polling. Rejected: the three
structural differences above; plus blast radius - every release-leg edit
would risk the gate file that every push depends on, and the sanctioned
scaffold reading ("Plan 8 consumes the trigger") is satisfied more
precisely by consuming the run it produces than by absorbing the file.

**Rejected: release re-runs the tests itself (or ci.yml becomes a
reusable `workflow_call` that release invokes).** Steelman: hermetic -
no cross-workflow coupling, no poll loop, no race; reusable workflows are
the idiomatic same-repo composition mechanism. Rejected: it duplicates a
3-OS matrix run that the kept `v*` trigger already performs on the same
SHA (double cost, zero signal - same code, same jobs), and the
workflow_call conversion would modify ci.yml structurally, which this
plan otherwise leaves untouched (smallest-change principle, and the v1.x
"remove mise from CI" entry already owns the next ci.yml restructuring).
The poll's race window (tag pushed while ci runs) is handled by waiting,
not failing.

**Rejected: rely on the draft-review human gate alone (no mechanical
check).** Steelman: the owner reviews every draft anyway and would see a
red gate. Rejected: "be careful at review time" is exactly the shape the
house replaces with a handle (ledger-lint precedent); the check is ~20
lines against a documented API.

**Interface changes:** new file `.github/workflows/release.yml`;
`guard` job contract (fails = no bundles); `permissions:` blocks per
job (guard: `contents: read, actions: read`; legs: `contents: read`;
assemble: `contents: write`).

---

## D84: Build tooling: direct `pnpm exec tauri build` with the repo-pinned CLI; tauri-action rejected

**Decision.** Every leg builds with the project's own pinned toolchain:
`pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json --bundles <leg set>`
(`@tauri-apps/cli` 2.11.4 from the frozen lockfile; flags verified on the
installed CLI, section 1.2). Per-leg `--bundles`: `msi` (both Windows
legs), `dmg` (macOS; the dmg bundler builds the .app itself - verified,
section 1.2), `deb rpm appimage` (Linux). Release creation and asset
upload are D77's `gh` steps. No `tauri-action`.

**Idiomacy check, discharged.** The ecosystem default IS
`tauri-apps/tauri-action` (Tauri's own pipeline docs recommend it,
section 1.5; health verified good, section 1.4). It is rejected on fit,
with its value inventory walked: (a) it runs `tauri build` - which we run
directly with the exact lockfile-pinned CLI version instead of the
action's resolution; (b) it creates the release and uploads assets - but
against D89's unified naming scheme its native asset names
(`Muxsmith_0.1.0_x64_en-US.msi` etc.) would need a rename pass anyway,
at which point `gh release create` with the renamed files is fewer moving
parts, and D79's rehearsal draft (`rehearsal-<run_id>`, no tag) does not
match its tagName-template model; (c) it wires updater artifacts and
signing - both banned at 1.0 (D76, D75). What remains is a third-party
action wrapping step (a) with less version control than the lockfile
already gives us.

**Rejected: tauri-action.** Steelman: the official, documented,
SHA-pinnable path (v1.0.0, active, 2026-06-29); it handles per-platform
artifact discovery, release creation, draft mode and future updater
support in ~10 lines of YAML, and every Tauri CI example in the wild
uses it - deviating IS the unidiomatic move that needs justification.
Rejected: the three-point inventory above - each of its jobs is either
already owned by a house mechanism (lockfile-pinned CLI, gh, D89 rename)
or banned at 1.0 (updater, signing); adopting it would mean fighting its
naming and release model from day one while still pinning and tracking a
new third-party dependency. The rejection is recorded as fit-based so a
future updater-era revisit (when its value inventory changes) is honest.

**Rejected: `cargo tauri build` via a globally installed tauri-cli.**
Steelman: no node needed on... no - node is needed for the frontend
build regardless. Rejected outright: it would introduce a second,
unpinned CLI version beside the lockfile-pinned one (`ci-10-pin-everything`).

**Interface changes:** none beyond the workflow file; the pinned CLI
version continues to come from `pnpm-lock.yaml`.

---

## D85: Four native build legs with pinned runners; toolchain setup without mise; no build cache on release legs

**Decision.** One `bundle` job with a four-entry `include` matrix
(`fail-fast: false` so all leg failures surface in one run):

| leg id | runner | rationale for the image |
|---|---|---|
| `windows-x86_64` | `windows-2025` | house pin (ci.yml test matrix) |
| `windows-arm64` | `windows-11-arm` | the only GA windows-arm64 label (section 1.3); native build (Tier-1 host tools) beats cross-compiling |
| `macos-arm64` | `macos-15` | house pin; arm64 (M1) verified |
| `linux-x86_64` | `ubuntu-22.04` | **compat floor**: Tauri's AppImage guidance mandates the oldest base providing webkitgtk 4.1 and names Ubuntu 22.04; Tauri's own CI example builds on it; artifacts built here run on every >= 22.04-era glibc (mkvtoolnix's AppImage targets glibc 2.28+ in the same spirit) |

The Linux release leg deliberately diverges from the test matrix's
`ubuntu-26.04` (preview) pin: **test on the newest, build releases on the
oldest supported**. The divergence and its reason are recorded as a
comment in release.yml (same pattern as ci.yml's brew-float comment).
`windows-11-arm` is an undated label; a dated variant does not exist
(section 1.3), so the pin-everything deviation is forced and recorded in
the same comment block - the same sanctioned shape as brew's
no-version-selector float in ci.yml.

Toolchain setup per leg, honoring the ROADMAP's directive that new
release legs must not deepen the mise dependency and choose their runtime
setup with recorded rationale:

- **Rust**: `rustup toolchain install` reading `rust-toolchain.toml`
  (pinned 1.96.1) - the existing house mechanism, no action. No
  rustfmt/clippy needed (release legs do not lint; the gate already ran,
  D83).
- **Node**: `actions/setup-node@8207627...` (# v7.0.0) with
  `node-version` fed from a one-line parse of `mise.toml`
  (`sed -n 's/^node = "\(.*\)"$/\1/p' mise.toml`) - mise.toml stays the
  single version source without running mise; setup-node has no native
  mise.toml support, and duplicating the version literal into the
  workflow would create drift the parse avoids.
- **pnpm**: `pnpm/action-setup@0ebf4713...` (# v6.0.9) with no version
  input - it reads `packageManager` from package.json (verified,
  section 1.4), the second existing single source. `corepack` is not
  relied on (its bundling status in current Node lines is exactly the
  kind of moving target a release path should not stand on).
- **Linux system deps**: the same apt package list ci.yml installs for
  Tauri builds (`libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`),
  on 22.04 (the AppImage docs name it as providing exactly these).
- **No mkvtoolnix on release legs** - nothing tests at build time; the
  gate covered that (D83).
- **No `Swatinem/rust-cache` and no other cache on release legs.**

**Rationale for no cache.** Release artifacts should be built from clean
state: caches are a supply-chain surface (a poisoned cache entry flows
into shipped binaries, and cache scoping rules are a moving target) and a
reproducibility variable; build time on free public-repo runners is the
cheap side of that trade. ci.yml keeps its cache - test builds are not
shipped.

**Rejected: cross-compile the arm64 msi from `windows-2025`.** Steelman:
one fewer runner family; the house already pins windows-2025; WiX takes
`-arch arm64` regardless of host. Rejected: native arm64 runners are GA
and free for public repos with rustup/VS preinstalled (section 1.3), a
native leg keeps all four legs structurally identical (host triple ==
target triple, no `--target` special case, no cross-linker risk), and
the Tauri docs' arm64 guidance is written for exactly this native path.

**Rejected: `mise-action` on release legs (as ci.yml does today).**
Steelman: maximum consistency with the test matrix; one version source
by construction. Rejected: the ROADMAP directive says do not deepen the
mise CI dependency (its removal is a recorded v1.x item precisely because
the action fetches a floating mise binary at run time - the
pin-everything violation the routed supply-chain finding named); the
parse+setup-node mechanism keeps the single source without the floating
binary. The v1.x item itself stays post-1.0 and is NOT absorbed here -
ci.yml is untouched (D83).

**Rejected: `ubuntu-24.04` or `ubuntu-26.04` for the Linux release
leg.** Steelman: newer toolchains, longer runway before image
retirement, and the test matrix already runs 26.04 so 22.04 adds a
third Ubuntu flavor to reason about. Rejected: every glibc/webkit symbol
version the artifacts link against becomes the users' minimum; the
authoritative Tauri guidance and the ecosystem's own CI example both
build on the oldest supported base, and a release artifact that cannot
run on a 2022 LTS is a real cost against zero benefit (nothing in the
build needs a newer base). Trigger T2 covers the image's eventual
retirement.

**Interface changes:** the four leg ids (`windows-x86_64`,
`windows-arm64`, `macos-arm64`, `linux-x86_64`) are D89's os-arch name
tokens and the workflow-artifact names - an interface, enumerated there.

---

## D86: `tauri.conf.json` bundle configuration: explicit targets, full metadata, pinned upgradeCode, minimal per-OS blocks

**Decision.** The bundle block is rewritten as (full file diff in
section 3.1; identifier `io.github.senolfeldmann.muxsmith` stays, per
the brief):

- `targets`: `["msi", "dmg", "deb", "rpm", "appimage"]` - the explicit
  shipped set replacing `"all"` (which would also build nsis and app as
  loose targets). Per-leg narrowing at build time via `--bundles` (D84);
  the config list is the authoritative superset (explicit over magic;
  anyone running a bare `tauri build` locally gets exactly the shipped
  formats for their OS).
- `publisher`: `"Şenol Feldmann"`; `copyright`:
  `"Copyright (c) 2026 Şenol Feldmann"`; `homepage`:
  `"https://github.com/senolfeldmann/Muxsmith"`; `license`: `"MIT"`;
  `licenseFile`: `"../LICENSE"` (WiX auto-converts the plain-text file
  to RTF - verified, section 1.2; deb/rpm embed it as-is).
  The non-ASCII publisher is deliberate (correct orthography); the
  rehearsal explicitly verifies its rendering in the msi's
  Programs-and-Features entry (step R8), and the pre-decided fallback if
  the msi mangles it is the ASCII transliteration `"Senol Feldmann"` in
  `publisher` only - no other field, no other trigger.
- `category`: `"Video"` (closed schema list, verified; the product is a
  video-container tool; mkvtoolnix's own desktop metadata sits in the
  video group).
- `shortDescription`: `"Rule-based bulk MKV muxing tool"`;
  `longDescription`: `"Declare how your MKVs should look. Muxsmith forges the whole library into shape - one profile, hundreds of files, zero clicking."`
  (the README's own tagline, reused not re-invented).
- `windows.wix.upgradeCode`:
  `"9262b417-b687-5ea3-ace1-18b9d51b215f"` - the value the installed
  CLI's `tauri inspect wix-upgrade-code` derives today (run 2026-07-22,
  section 1.2), pinned so a future `productName` change can never
  silently change upgrade identity (the schema's own recommendation).
  Both arches share it, which matches Tauri's default derivation (the
  literal `.x64` string regardless of arch) and means an arch-switching
  user gets a clean upgrade instead of two side-by-side installs.
  `windows.wix.language`: `["en-US"]`, explicit - the installer UI
  language set; the app itself is bilingual at runtime; a `de-DE`
  installer UI would double the msi count against the ruled two-artifact
  Windows matrix, so it is deliberately not configured (a de installer
  UI request is observable and can ride the signing revisit).
- `macOS.minimumSystemVersion`: `"11.0"` - the schema default 10.13 is
  unsatisfiable on an arm64-only artifact (no Intel leg exists, D78);
  11.0 is the arm64 hardware floor, so `LSMinimumSystemVersion` stops
  lying.
- `macOS.dmg`: **not configured** - Tauri's default dmg window layout
  ships at 1.0. This is the presentation-token carve-out's territory
  (`latitude-carveout-presentation-tokens`), and the visual-polish pass
  is the recorded v1.x "schick machen" item; stating the non-configuration
  here closes the fork explicitly rather than leaving it as omission.
- `linux.deb`: `{"section": "video", "recommends": ["mkvtoolnix"]}`
  (Debian section verified against mkvtoolnix's own `Section: video`,
  section 1.6); `linux.rpm`: `{"recommends": ["mkvtoolnix"]}` (rpm has
  no section field in the schema; its `Group` era is long deprecated).
- `linux.appimage`: **not configured** - `bundleMediaFramework` stays
  default-false (Muxsmith renders no media; gstreamer would add tens of
  MB for nothing), no extra `files`.
- `createUpdaterArtifacts`: absent (D76). `fileAssociations`: **not
  configured** - Muxsmith opens profiles and directories, not .mkv files
  double-clicked from a file manager; associating .mkv would hijack a
  container format every media player competes for. Recorded as a
  deliberate non-decision; a user request reopens it as a product
  question, not an implementation nicety.
- `version`: **removed** (D87).

**Rationale.** Each value above is either reuse of an existing repo
string (tagline, license, repository URL), a verified schema enum member,
or carries its inline justification. The block stays minimal: every
per-OS field not listed here is deliberately default.

**Rejected: keep `targets: "all"` and rely solely on `--bundles`.**
Steelman: smaller diff; the workflow controls what actually builds
anyway. Rejected: "all" includes nsis and standalone app - formats the
matrix ruling excludes; config should state the shipped truth
(`core-derive-dont-restate`'s spirit: one authoritative statement), and a
local `tauri build` without flags should not produce artifacts the
project does not ship.

**Rejected: per-arch upgradeCodes for the two msi artifacts.** Steelman:
Windows Installer treats same-UpgradeCode packages as one product family,
and MS guidance historically leaned per-arch codes so x86/x64 could
coexist. Rejected: coexistence of an x64 and an arm64 Muxsmith on one
machine is an anti-goal (the arm64 msi exists precisely so arm users do
not run x64), Tauri's default already shares the code across arches
(verified derivation string), and two codes would double the pinned
config surface for a scenario we do not want to support.

**Interface changes:** installed-app metadata (publisher, category,
copyright) is user-visible; the upgradeCode is a permanent wire-format
commitment (must never change again - recorded as a comment beside the
value); `wix.language` fixes the msi UI language contract at en-US.

---

## D87: Version sync: `tauri.conf.json` stops declaring a version (Cargo inheritance); a guard script makes a mismatched tag impossible

**Decision.** Three moves:

1. **Delete the `version` key from `tauri.conf.json`.** The bundler then
   reads the version from `src-tauri/Cargo.toml`, which inherits
   `[workspace.package] version` (schema fallback verified verbatim,
   section 1.1). Rust source of truth stays the workspace version, now
   with one restatement fewer.
2. **`package.json` keeps its `version` field but it is guard-enforced.**
   The field is npm-required-ish boilerplate on a private package and
   feeds nothing user-visible; deleting it would be a gratuitous
   deviation from the npm ecosystem shape (idiomacy), so it stays and the
   guard pins it to the workspace version.
3. **New `scripts/check-version-sync.sh`** (verbatim in section 3.3),
   run by release.yml's guard job on every tag and dispatch (D83):
   - parses the workspace version from `Cargo.toml`
     (`[workspace.package]` block-scoped awk, no toolchain needed),
     `package.json` (`jq -r .version`), and asserts equality;
   - asserts `tauri.conf.json` **has no** `version` key
     (`jq 'has("version")'` == false) - so the inheritance can never be
     silently bypassed by a reintroduced literal;
   - on the tag path additionally asserts `tag == "v" + version`.

   The script is bash+jq only, runnable locally; its red states are
   fire-verified in the test plan (section 8, G1-G3) per
   `proc-verification-step-must-be-falsifiable`.

**The failure this makes impossible** (the brief's named criterion): a
tag whose artifacts self-report a different version. After move 1 every
artifact's self-reported version IS the workspace version (bundle
metadata from Cargo via the verified fallback; the CLI's `--version` from
the same workspace field via clap - section 1.7), and the guard refuses
to build any tag not equal to it. Artifact-level confirmation rides the
rehearsal (step R9: `muxsmith --version` from the tar.gz equals the
Cargo version; bundle filenames carry it).

**Rejected: sync script that rewrites the three files (bump tooling).**
Steelman: one command bumps everywhere; no guard needed if writing is
centralized. Rejected: a writer is a bigger mechanism than the problem -
versions change once per release by hand today; the house pattern for
restatement risk is committed-source-plus-drift-check
(settables.ts precedent, ledger count-integrity precedent), and a check
that fails loudly beats tooling that must be remembered to be used
(the exact failure mode the ROADMAP's ledger-lint entry documents).

**Rejected: keep the tauri.conf.json version and add it to the guard.**
Steelman: explicit beats inherited (house lean); the schema itself says
"It's recommended to manage the app versioning in the Tauri config".
Rejected: the schema's recommendation targets projects whose source of
truth IS the Tauri config; ours is the Cargo workspace (settled, spec
§2 stack row - the Rust core is the product's root), and an inherited
value that cannot drift beats a restated value plus a check that it did
not (`core-derive-dont-restate`: derive, don't restate). The absence
assertion in the guard keeps the inheritance itself enforced.

**Interface changes:** `tauri.conf.json` loses a key (behavior-neutral,
verified fallback); new script file `scripts/check-version-sync.sh`
(house scripts/ convention beside `ledger-lint.py`); guard job wiring
(D83).

---

## D88: The tar.gz leg: a hand-packed archive with both binaries, LICENSE and a README

**Owner rulings folded in:** the tar.gz is the Linux "just runs" archive
(S22 kickoff) and carries BOTH binaries (S22 second round).

**Decision.** The Linux leg packs, after `tauri build`:

```
muxsmith-<version>-linux-x86_64/
├── muxsmith          # target/release/muxsmith (CLI)
├── muxsmith-gui      # target/release/muxsmith-gui (GUI)
├── LICENSE           # repo root LICENSE, verbatim
└── README.txt        # packaging/linux-tarball-README.txt, verbatim
```

then `tar czf muxsmith-<version>-linux-x86_64.tar.gz <dir>` (the
directory prefix inside the archive, so extraction never splats four
files into cwd). The README content is committed at
`packaging/linux-tarball-README.txt` (new top-level `packaging/` dir -
release-channel collateral is neither a Tauri artifact under `src-tauri/`
nor CI logic under `.github/`; full verbatim content in section 4.3). It
names: what the two binaries are, the runtime requirements (mkvtoolnix/
mkvmerge on PATH; webkitgtk 4.1 + gtk3 for the GUI; the glibc floor
implied by the 22.04 build base), how to run each, the PATH hint, and the
`docs/INSTALL.md` / repo links. English only at 1.0 (the file is static
collateral; the 1.0 doc set's language policy - EN GUIDE, EN+DE blogs -
is a settled ROADMAP item, and a `de` twin can ride any later locale
pass).

The binaries are the exact files the same build already produced for the
deb/rpm/AppImage (no re-compile, no strip pass - byte-identical to the
packaged ones; whatever the cargo release profile ships is what every
Linux channel ships).

**Rationale.** Tauri has no tar.gz bundler (BundleType enum verified,
section 1.1), so a packing step is the only path; mkvtoolnix's portable
archive is the direct SI-3 precedent for shipping a "just runs" archive
beside installers. tar.gz over tar.xz/zip: the ruling's own words name
tar.gz, and it is the Linux-native default.

**Rejected: `cargo-dist` (or similar) for the archive leg.** Steelman:
purpose-built release archiver, checksums and installers included,
well-maintained. Rejected: a whole release-orchestration framework to
produce one tar of two already-built binaries is the dependency the
earned-dependency rule rejects; it would also fight the Tauri bundler for
ownership of everything else. Trigger T8 records the revisit condition.

**Rejected: per-OS portable archives (win zip, mac tar.gz).** Steelman:
mkvtoolnix ships a Windows portable; symmetry is tidy. Rejected: the
ruled matrix is closed (D78); win/mac CLI users are served by D82's
bundled CLI. The Windows-portable parity gap is recorded in section 5
with a trigger candidate, not smuggled into scope.

**Interface changes:** archive layout above is user-visible contract;
new committed file `packaging/linux-tarball-README.txt`; new `packaging/`
directory.

---

## D89: Unified artifact naming `muxsmith-<version>-<os>-<arch>.<ext>`; workflow-artifact names and retention

**Decision.** Every release asset is renamed post-build to the scheme
`muxsmith-<version>-<os>-<arch>.<ext>`, lowercase product name, `<os>` in
{`windows`, `macos`, `linux`}, `<arch>` in {`x86_64`, `arm64`}, `<ext>`
keeping its canonical case (`.AppImage` capitalized as the format
mandates). The complete asset set of a release `vX.Y.Z` (8 assets):

1. `muxsmith-X.Y.Z-windows-x86_64.msi`
2. `muxsmith-X.Y.Z-windows-arm64.msi`
3. `muxsmith-X.Y.Z-macos-arm64.dmg`
4. `muxsmith-X.Y.Z-linux-x86_64.deb`
5. `muxsmith-X.Y.Z-linux-x86_64.rpm`
6. `muxsmith-X.Y.Z-linux-x86_64.AppImage`
7. `muxsmith-X.Y.Z-linux-x86_64.tar.gz`
8. `SHA256SUMS`

The rename step in each leg globs the bundler's per-format output dir
(`target/release/bundle/<format>/` under the workspace target dir, since
`src-tauri` is a workspace member) and **asserts exactly one
match per expected artifact** before renaming (a zero or multiple match
fails the leg - the count assertion is the falsifiable form; the
bundler's native names like `Muxsmith_0.1.0_x64_en-US.msi` or
`muxsmith-gui_0.1.0_amd64.deb` vary per format, which is exactly why the
glob-and-assert beats hardcoding them). deb/rpm consumers lose nothing by
the rename: dpkg/rpm read package metadata, not filenames, and the
distro-conventional filename matters only inside repositories, which
GitHub release assets are not.

Workflow artifacts: one per leg, named `muxsmith-<leg id>` (leg ids =
D85's four), each containing that leg's renamed artifacts;
`retention-days: 7` (bundles are reproducible from the tag; the release
assets are the durable copy; 90-day default would hoard gigabytes of
rehearsal output). Upload via `actions/upload-artifact` pinned per
section 1.4; assemble downloads with `actions/download-artifact`
(same table).

**Rationale.** One grep-able scheme across seven artifacts beats four
per-format conventions; the version-in-name is the user-facing half of
D87 (a downloaded file names what it contains); os-arch tokens match the
leg ids so workflow logs, artifact names and asset names all speak one
vocabulary.

**Rejected: keep bundler-native names.** Steelman: zero rename logic;
deb/rpm names follow distro convention (`_amd64`, `.x86_64`), which
package-savvy users expect; tauri-action users ship these names all over
GitHub. Rejected: three naming dialects (WiX's `x64_en-US`, deb's
`amd64`, rpm's `x86_64`) plus a capitalized product prefix on some and
not others is a support-tax scheme; the release page is the product's
shop window and the unified names state os/arch in one vocabulary.
The metadata-not-filename fact removes the only technical objection.

**Interface changes:** the 8-asset name set IS an interface (users,
scripts, the SHA256SUMS contents, D77's body template all reference it);
`<os>-<arch>` tokens are bound to D85's leg ids; changing the scheme
later is a breaking change for any user automation and gets a D-entry,
not a drive-by.

---

## D90: The release attaches one `SHA256SUMS` file

**Decision.** The assemble job, after downloading all four legs'
artifacts into one directory, runs `sha256sum <the 7 files> > SHA256SUMS`
(coreutils on the ubuntu-22.04 assemble runner) and attaches it as the
8th asset. The draft body's template names the verification command
(`sha256sum -c SHA256SUMS` after downloading files next to it). The
rehearsal verifies round-trip (step R4: download rehearsal assets,
`sha256sum -c` passes; falsifiability control: flip one byte locally and
watch `-c` fail once).

**Rationale.** Supply-chain hygiene precedent in the house
(pin-everything, deny job, SHA-pinned actions): unsigned artifacts (D75)
make an integrity channel MORE valuable, not less - a checksum file is
the cheapest honest statement "this is what CI built" available without
keys. One aggregate file over per-artifact `.sha256` files: one download,
one command, standard tooling.

**Rejected: GPG-sign the checksums (or artifacts).** Steelman:
checksums on the same server as the artifacts authenticate nothing
against a compromised release; a detached signature would. Rejected: key
custody is exactly the burden the unsigned-at-1.0 ruling declined;
sigstore/attestations are the modern answer and belong to the signing
revisit trigger, not to 1.0.

**Rejected: GitHub artifact attestations (`actions/attest-build-provenance`).**
Steelman: keyless provenance, first-party, free. Rejected for 1.0 scope:
it adds an action pin + `id-token: write`/`attestations: write`
permissions and a verification story users need `gh` for; a strong
candidate for the signing-revisit trigger where it is hereby recorded as
the first option to evaluate.

**Interface changes:** asset #8 and its format (`sha256sum` default
format, filenames exactly as in D89 - which is why the file is generated
after the rename, never before).

---

## 2. The release workflow (verbatim-ready sketch)

`.github/workflows/release.yml`. Every action SHA below is from the
verified table (section 1.4); comments carry the decision pointers.

```yaml
name: release
on:
  push:
    tags: ['v*']
  workflow_dispatch:
    inputs:
      rehearse-draft-release:
        description: >-
          Also rehearse draft-release assembly: creates a clearly marked
          DRAFT named rehearsal-<run_id> (never a tag, never published).
        type: boolean
        default: false

# Least privilege (house precedent, ci.yml): read-only default;
# the assemble job alone elevates to contents: write (D77/D83).
permissions:
  contents: read

# Pinning policy: actions pinned to release commit SHAs, runner images
# pinned to explicit versions (ci.yml policy comment is the authority).
# Two recorded deviations (D85): windows-11-arm has no dated label
# (the only GA windows-arm64 image); ubuntu-22.04 here deliberately
# diverges from the test matrix's ubuntu-26.04 - release artifacts are
# built on the oldest supported base (Tauri AppImage guidance) while
# tests run on the newest.

jobs:
  guard:
    runs-on: ubuntu-22.04
    permissions:
      contents: read
      actions: read   # gh run list (gate-green check)
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
      - name: Version sync (D87; tag arm only on the tag path)
        run: |
          if [ "${GITHUB_REF_TYPE}" = "tag" ]; then
            scripts/check-version-sync.sh "${GITHUB_REF_NAME}"
          else
            scripts/check-version-sync.sh
          fi
      - name: Require the ci gate green on this SHA (D83)
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          # Wait up to 45 min for a completed ci.yml run on this exact
          # SHA; succeed on the first success, fail when all completed
          # runs are red or on timeout.
          for i in $(seq 1 90); do
            runs="$(gh run list --workflow ci.yml --commit "$GITHUB_SHA" \
              --json status,conclusion \
              --jq '[.[] | {status, conclusion}]')"
            if [ "$(jq 'map(select(.status == "completed" and .conclusion == "success")) | length' <<<"$runs")" -gt 0 ]; then
              echo "ci gate green for $GITHUB_SHA"; exit 0
            fi
            total="$(jq 'length' <<<"$runs")"
            done_ct="$(jq 'map(select(.status == "completed")) | length' <<<"$runs")"
            if [ "$total" -gt 0 ] && [ "$done_ct" = "$total" ]; then
              echo "::error::ci gate red for $GITHUB_SHA"; exit 1
            fi
            echo "waiting for ci ($done_ct/$total completed)"; sleep 30
          done
          echo "::error::timed out waiting for a ci run on $GITHUB_SHA"; exit 1

  bundle:
    needs: guard
    strategy:
      fail-fast: false
      matrix:
        include:
          - leg: windows-x86_64
            os: windows-2025
            bundles: msi
          - leg: windows-arm64
            os: windows-11-arm
            bundles: msi
          - leg: macos-arm64
            os: macos-15
            bundles: dmg
          - leg: linux-x86_64
            os: ubuntu-22.04
            bundles: deb,rpm,appimage
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
      - name: Install pinned Rust toolchain
        shell: bash
        run: rustup toolchain install
      - name: Install Tauri Linux build dependencies
        if: runner.os == 'Linux'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
      - name: "Read pinned node version from mise.toml (D85: no mise in release legs)"
        id: node
        shell: bash
        run: echo "version=$(sed -n 's/^node = "\(.*\)"$/\1/p' mise.toml)" >> "$GITHUB_OUTPUT"
      - uses: actions/setup-node@820762786026740c76f36085b0efc47a31fe5020 # v7.0.0
        with:
          node-version: ${{ steps.node.outputs.version }}
      - uses: pnpm/action-setup@0ebf47130e4866e96fce0953f49152a61190b271 # v6.0.9
        # no version input: reads packageManager from package.json
      - name: Install frontend dependencies
        run: pnpm install --frozen-lockfile
      - name: Build CLI and stage it as the bundle sidecar (D82)
        shell: bash
        run: |
          cargo build --release -p muxsmith-cli
          triple="$(rustc -vV | sed -n 's/^host: //p')"
          mkdir -p src-tauri/binaries
          if [ "${RUNNER_OS}" = "Windows" ]; then
            cp target/release/muxsmith.exe "src-tauri/binaries/muxsmith-${triple}.exe"
          else
            cp target/release/muxsmith "src-tauri/binaries/muxsmith-${triple}"
          fi
      - name: Build bundles (D84)
        shell: bash
        run: pnpm exec tauri build --ci -c src-tauri/tauri.bundle.conf.json --bundles "${{ matrix.bundles }}"
      - name: Assert no updater artifacts were produced (D76)
        shell: bash
        run: |
          set -euo pipefail
          # Positive control first: the bundle output tree must exist and
          # be non-empty, else the absence check below proves nothing.
          [ -d target/release/bundle ] || { echo "::error::positive control failed: no bundle output dir" >&2; exit 1; }
          bundles_found="$(find target/release/bundle -type f | wc -l)"
          [ "$bundles_found" -gt 0 ] || { echo "::error::positive control failed: bundle output dir is empty" >&2; exit 1; }
          find target/release/bundle -type f \( -name '*.sig' -o -name 'latest.json' \) -print >&2
          updater_hits="$(find target/release/bundle -type f \( -name '*.sig' -o -name 'latest.json' \) | wc -l)"
          if [ "$updater_hits" -ne 0 ]; then
            echo "::error::$updater_hits updater artifact(s) found - D76 bans updater output" >&2
            exit 1
          fi
          echo "updater-artifact check: 0 hits across $bundles_found bundle output files"
      - name: Rename artifacts to the release scheme; pack tar.gz on Linux (D88/D89)
        shell: bash
        run: |
          set -euo pipefail
          version="$(awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml)"
          leg="${{ matrix.leg }}"
          out="release-assets"; mkdir -p "$out"
          pick() { # pick <glob...> -> asserts exactly one existing match;
                   # path on stdout (the return value), log on stderr so it
                   # survives the command substitution at the call sites
            local matches=("$@")
            if [ "${#matches[@]}" -ne 1 ] || [ ! -e "${matches[0]}" ]; then
              echo "::error::expected exactly one artifact, got: ${matches[*]}" >&2
              exit 1
            fi
            echo "pick: ${matches[0]}" >&2
            echo "${matches[0]}"
          }
          # Bundle output sits under the WORKSPACE target dir (src-tauri is
          # a workspace member; the repo root target/ is cargo's out dir).
          case "$leg" in
            windows-*)
              msi="$(pick target/release/bundle/msi/*.msi)"
              cp "$msi" "$out/muxsmith-$version-$leg.msi" ;;
            macos-arm64)
              dmg="$(pick target/release/bundle/dmg/*.dmg)"
              cp "$dmg" "$out/muxsmith-$version-$leg.dmg" ;;
            linux-x86_64)
              deb="$(pick target/release/bundle/deb/*.deb)"
              rpm="$(pick target/release/bundle/rpm/*.rpm)"
              ai="$(pick target/release/bundle/appimage/*.AppImage)"
              cp "$deb" "$out/muxsmith-$version-$leg.deb"
              cp "$rpm" "$out/muxsmith-$version-$leg.rpm"
              cp "$ai"  "$out/muxsmith-$version-$leg.AppImage"
              stage="muxsmith-$version-$leg"
              mkdir "$stage"
              cp target/release/muxsmith target/release/muxsmith-gui "$stage/"
              cp LICENSE "$stage/LICENSE"
              cp packaging/linux-tarball-README.txt "$stage/README.txt"
              tar czf "$out/$stage.tar.gz" "$stage" ;;
          esac
          ls -l "$out"
      - uses: actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a # v7.0.1
        with:
          name: muxsmith-${{ matrix.leg }}
          path: release-assets/*
          retention-days: 7
          if-no-files-found: error

  assemble:
    needs: [guard, bundle]
    runs-on: ubuntu-22.04
    permissions:
      contents: write   # draft-release creation + asset upload (D77)
    steps:
      - uses: actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0 # v7.0.0
      - uses: actions/download-artifact@3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c # v8.0.1
        with:
          pattern: muxsmith-*
          path: assets
          merge-multiple: true
      - name: Generate SHA256SUMS (D90)
        run: |
          cd assets
          sha256sum * > SHA256SUMS
          cat SHA256SUMS
      - name: Create draft release (tag path) or rehearsal draft (D77/D79)
        if: github.ref_type == 'tag' || inputs.rehearse-draft-release
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          set -euo pipefail
          version="$(awk '/^\[workspace.package\]/{f=1;next} /^\[/{f=0} f && /^version = /{gsub(/version = |"/,""); print; exit}' Cargo.toml)"
          if [ "${GITHUB_REF_TYPE}" = "tag" ]; then
            relname="${GITHUB_REF_NAME}"; title="Muxsmith ${version}"
            extra_args=(--verify-tag)
          else
            relname="rehearsal-${GITHUB_RUN_ID}"
            title="REHEARSAL - not a release (run ${GITHUB_RUN_ID})"
            extra_args=(--target "${GITHUB_SHA}")
            cat .github/release/rehearsal-banner.md > body.md
          fi
          sed "s/__VERSION__/${version}/g" .github/release/draft-body.md >> body.md
          gh api "repos/${GITHUB_REPOSITORY}/releases/generate-notes" \
            -f tag_name="${relname}" -f target_commitish="${GITHUB_SHA}" \
            --jq .body >> body.md
          gh release create "${relname}" --draft "${extra_args[@]}" \
            --title "${title}" --notes-file body.md assets/*
```

Notes for the implementer, all decided above: the `body.md` composition
order is rehearsal-banner (rehearsal only) -> template -> generated
notes; the two template files live under `.github/release/` (workflow
collateral, unlike the shipped tar.gz README which is product collateral
under `packaging/` - D88); `assets/*` uploads exactly the 8 files
because `merge-multiple: true` flattens the four legs' artifacts plus
the generated SHA256SUMS and nothing else is in the directory.

---

## 3. Config and script changes

### 3.1 `src-tauri/tauri.conf.json` - the new bundle surface (D86/D87/D76)

The file after the change (non-bundle sections unchanged and elided;
`version` key deleted from the top level):

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

A JSON file cannot carry the upgradeCode's never-change-this warning as a
comment; the warning lives in this D86 and in the BUILDING.md subsection
(4.4), which is the file a config-editing human reads.

### 3.2 `src-tauri/tauri.bundle.conf.json` (D82, new, committed, complete)

```json
{
  "bundle": {
    "externalBin": ["binaries/muxsmith"]
  }
}
```

### 3.3 `scripts/check-version-sync.sh` (D87, new, complete)

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

### 3.4 `.gitignore` (D82)

One added line in the JS/Tauri block: `src-tauri/binaries/`.

---

## 4. Documentation artifacts (verbatim)

### 4.1 `docs/INSTALL.md` (D75/D82, new, complete)

````markdown
# Installing Muxsmith

Release downloads live on the
[GitHub releases page](https://github.com/senolfeldmann/Muxsmith/releases).
All 1.0-era builds are **unsigned**: your OS will warn before the first
launch. The sections below show the one-time step per OS. Verify
downloads against the release's `SHA256SUMS`
(`sha256sum -c SHA256SUMS` with the files beside it).

Every install ships two programs: **Muxsmith** (the GUI) and
**`muxsmith`** (the command-line tool).

<!-- When code signing lands (registered ROADMAP trigger), the
     SmartScreen and Gatekeeper sections below shrink to the signed-app
     reality; keep the CLI/PATH halves. -->

## Windows

Artifact: `muxsmith-<version>-windows-x86_64.msi` (Intel/AMD) or
`muxsmith-<version>-windows-arm64.msi` (Windows on ARM, e.g. Snapdragon
laptops). The installer is per-machine and installs to
`C:\Program Files\Muxsmith`.

**SmartScreen:** because the installer is unsigned, Windows shows
"Windows protected your PC" with no run button. Click **More info**,
then **Run anyway**. The publisher shows as unknown; that is expected
for an unsigned build.

**CLI:** `muxsmith.exe` is installed next to the app in
`C:\Program Files\Muxsmith`. The installer does **not** modify PATH.
To call `muxsmith` from any terminal, add that folder to your user PATH:
Settings > System > About > Advanced system settings > Environment
Variables > select `Path` > Edit > New >
`C:\Program Files\Muxsmith` > OK, then open a new terminal.

## macOS

Artifact: `muxsmith-<version>-macos-arm64.dmg` (Apple Silicon,
macOS 11+). There is currently no Intel build.

Open the dmg and drag **Muxsmith** to Applications.

**Gatekeeper:** the app is unsigned and not notarized.

- **macOS 15 (Sequoia) and newer:** double-click Muxsmith once; macOS
  blocks it. Open **System Settings > Privacy & Security**, scroll to
  the security section, and click **Open Anyway** next to the Muxsmith
  entry, then confirm. This is needed once.
- **macOS 11-14:** Control-click (or right-click) Muxsmith.app and
  choose **Open**, then **Open** again in the dialog. Once is enough.
- **Terminal alternative** (any version):
  `xattr -d com.apple.quarantine /Applications/Muxsmith.app`

**CLI:** the command-line tool is inside the app bundle at
`/Applications/Muxsmith.app/Contents/MacOS/muxsmith`. Nothing touches
PATH. To call it as `muxsmith`, link it once:

```sh
sudo ln -s /Applications/Muxsmith.app/Contents/MacOS/muxsmith /usr/local/bin/muxsmith
```

## Linux

Artifacts (x86_64):

- `muxsmith-<version>-linux-x86_64.deb` - Debian/Ubuntu: `sudo apt install ./muxsmith-<version>-linux-x86_64.deb`
- `muxsmith-<version>-linux-x86_64.rpm` - Fedora & co.: `sudo dnf install ./muxsmith-<version>-linux-x86_64.rpm`
- `muxsmith-<version>-linux-x86_64.AppImage` - any distro: `chmod +x` the file, then run it
- `muxsmith-<version>-linux-x86_64.tar.gz` - portable archive with both binaries; see its
  `README.txt`

No gatekeeping dialog exists on Linux. deb/rpm install both `muxsmith`
and `muxsmith-gui` to `/usr/bin` (already on PATH). The deb/rpm packages
declare **mkvtoolnix** as a recommended dependency; the AppImage and
tar.gz do not manage dependencies, so install the runtime requirements
yourself:

- **mkvtoolnix** (provides `mkvmerge`; required for every mux/dry run):
  `sudo apt install mkvtoolnix` / `sudo dnf install mkvtoolnix`
- **GUI only, deb/rpm/tar.gz:** webkitgtk 4.1 and gtk3
  (`libwebkit2gtk-4.1-0` on Debian/Ubuntu, `webkit2gtk4.1` on Fedora) -
  the deb/rpm declare these as hard dependencies; the AppImage bundles
  them; for the tar.gz install them via your package manager.
````

The Windows/macOS artifact tables above are the release-notes template's
link targets (`#windows`, `#macos`, `#linux` anchors).

### 4.2 Release-body templates (D77/D79, new, complete)

`.github/release/draft-body.md`:

```markdown
Muxsmith __VERSION__ - unsigned builds; read the install note for your OS
before first launch: [Windows](https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md#windows)
| [macOS](https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md#macos)
| [Linux](https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md#linux)

**Runtime requirement:** Muxsmith drives `mkvmerge` from
[MKVToolNix](https://mkvtoolnix.download/). The deb/rpm packages declare
it as a recommended dependency; on Windows/macOS install it yourself
(details in the install notes).

| Artifact | For |
|---|---|
| `muxsmith-__VERSION__-windows-x86_64.msi` | Windows 10/11, Intel/AMD |
| `muxsmith-__VERSION__-windows-arm64.msi` | Windows 11 on ARM |
| `muxsmith-__VERSION__-macos-arm64.dmg` | macOS 11+, Apple Silicon |
| `muxsmith-__VERSION__-linux-x86_64.deb` | Debian/Ubuntu |
| `muxsmith-__VERSION__-linux-x86_64.rpm` | Fedora & co. |
| `muxsmith-__VERSION__-linux-x86_64.AppImage` | any Linux distro |
| `muxsmith-__VERSION__-linux-x86_64.tar.gz` | portable, CLI + GUI |

Verify downloads: put `SHA256SUMS` beside the files and run
`sha256sum -c SHA256SUMS`.

---
```

`.github/release/rehearsal-banner.md`:

```markdown
> **REHEARSAL DRAFT - not a release.** Created by a workflow_dispatch
> rehearsal run to exercise draft-release assembly (D79). No git tag
> exists for this draft. Inspect body, assets and checksums, then
> delete this draft.

---
```

### 4.3 `packaging/linux-tarball-README.txt` (D88, new, complete)

```text
Muxsmith - portable Linux archive (x86_64)
==========================================

Contents:
  muxsmith       command-line tool
  muxsmith-gui   desktop app (Tauri/GTK)
  LICENSE        MIT license
  README.txt     this file

This archive "just runs": no installation step. Run the tools from this
directory (./muxsmith --help, ./muxsmith-gui) or put the directory on
your PATH / symlink the binaries into ~/.local/bin.

Requirements
------------
- mkvtoolnix (provides mkvmerge) - required for every dry run and mux:
    Debian/Ubuntu:  sudo apt install mkvtoolnix
    Fedora:         sudo dnf install mkvtoolnix
- For muxsmith-gui: webkitgtk 4.1 and gtk3
    Debian/Ubuntu:  libwebkit2gtk-4.1-0 (and libgtk-3-0)
    Fedora:         webkit2gtk4.1
- glibc from Ubuntu 22.04 (2022) or newer; any current distribution
  qualifies. If this archive does not run on your system, the AppImage
  from the same release bundles its dependencies.

Docs and source: https://github.com/senolfeldmann/Muxsmith
Install notes:   https://github.com/senolfeldmann/Muxsmith/blob/master/docs/INSTALL.md
```

### 4.4 `BUILDING.md` rider (D82/D86, addition, complete)

Appended as a subsection under "Building and running":

```markdown
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
```

### 4.5 `README.md` placeholder rider (D75, comment-text edit, complete)

Line 99's comment becomes:

```markdown
<!-- placeholder(1.0): Install section - artifact table per OS (msi x2 /
     dmg / deb / rpm / AppImage / tar.gz, naming per Plan-8 D89) linking
     docs/INSTALL.md, which already carries the per-OS unsigned-install
     steps; drop the WIP banner in the same pass -->
```

---

## 5. mkvtoolnix parity audit (SI-3)

Sources: section 1.6. Format families side by side:

| channel | mkvtoolnix (v100) | Muxsmith 1.0 | disposition |
|---|---|---|---|
| Windows installer | NSIS setup.exe, x64 + x86, ships every tool (`File "../*.exe"`), no PATH write | msi, x64 + arm64, ships GUI + CLI (D82), no PATH write | parity in substance; format differs deliberately (D78: msi over NSIS); mkvtoolnix has no arm64 build - Muxsmith exceeds here |
| Windows portable | 7z archive (64/32-bit) | none (CLI rides the msi; tar.gz is Linux-only) | recorded gap, trigger T6 |
| macOS | dmg, all five binaries in `Contents/MacOS`, manual invocation documented | dmg, both binaries in `Contents/MacOS`, manual PATH step documented (D82/D75) | direct parity, incl. the no-PATH stance |
| Linux distro packages | apt/dnf repos, split `mkvtoolnix` / `mkvtoolnix-gui`, `Section: video` | GitHub-released deb + rpm, ONE package with both binaries, `section: video`, `Recommends: mkvtoolnix` | split is a recorded deliberate divergence (owner ruling, steelmanned in D82); no hosted repo at 1.0 (out of scope; nothing registered) |
| AppImage | distribution-agnostic, glibc 2.28+ | AppImage from the 22.04 floor (D85) | direct parity; precedent for the compat-floor build |
| Store channels | Microsoft Store, Chocolatey, MacPorts, Homebrew (CLI-only formula) | none at 1.0; Homebrew Cask is the recorded v1.x ROADMAP entry | out of design scope by owner ruling |
| Checksums/signing | GPG-signed distro repos | SHA256SUMS (D90), unsigned (D75) | gap tied to the registered signing trigger |

Licensing boundary: facts only; no mkvtoolnix text, script, or asset is
copied into any Muxsmith artifact.

---

## 6. CI cost and deliberate restraint

Per tag or dispatch run: **6 jobs** - guard, 4 bundle legs, assemble.
All on free public-repo standard runners, including `windows-11-arm`
(section 1.3). Bundle builds run **only** on `v*` tags and manual
dispatch - deliberately no per-push and no per-PR bundling (the nightly-
artifact idea stays unproposed: nothing consumes it, and the test matrix
already covers compilation on every push). ci.yml's own tag-triggered
test run is not duplicated (D83). No concurrency group: tags and
dispatches are rare, owner-driven events; a canceling group would add
config for a collision that operationally cannot occur (two simultaneous
releases), and a queued duplicate run is harmless (drafts are idempotent
to recreate and the rehearsal names carry the run id).

---

## 7. Deliberately out of scope

Each with its reason and, where one exists, its registered revisit hook:

- **Code signing / notarization** - owner ruling F1; registered ROADMAP
  trigger ("First external-user complaint about unsigned-install
  hurdles..."). D90 records artifact attestations as that trigger's
  first candidate.
- **Auto-updater** - owner ruling (v1.x); D76.
- **macOS x64 / universal** - registered Intel-request trigger; D78.
  The `macos-15-intel` label exists when it fires (section 1.3).
- **Tagging 1.0, publishing anything** - ruling 7 / D81.
- **Homebrew Cask** - v1.x ROADMAP entry by owner ruling (section 0
  note 1); not proposed here.
- **Hosted apt/dnf repositories, Store/Chocolatey channels** - no ruling
  requests them; recorded as observed mkvtoolnix channels in section 5
  only.
- **deb/rpm hard Depends on mkvtoolnix** - recorded v1.x entry; D80
  notes the two-token change site.
- **Removing mise from ci.yml** - stays the recorded post-1.0 item; this
  plan's release legs simply never adopt mise (D85), which neither
  absorbs nor deepens it. The ROADMAP trigger "the v1.x mise-out-of-CI
  structural work starts" is NOT fired by this plan.
- **ledger-lint CI wiring** - the deferral trigger fired with this plan
  (release.yml makes Plan 8 the next CI-touching plan; surfaced via this
  design's delivery). Controller ruling, since recorded in the ROADMAP
  ledger-hygiene entries (2026-07-22 S22): Plan 8 absorbs the wiring as
  a rider task, bundled with the duplicate-key extension; the rider
  enters the plan brief at plan authoring and is explicitly "not a
  design amendment (nothing in the plan-8 design depends on it)"
  (ROADMAP wording, re-read 2026-07-23).

---

## 8. Test / verification plan: the workflow_dispatch rehearsal is the acceptance test

Five pre-merge local fire-tests (falsifiability duty,
`proc-verification-step-must-be-falsifiable`; each is
break-observe-restore):

- **G1**: `scripts/check-version-sync.sh v9.9.9` on the clean tree ->
  must exit 1 (tag arm red state); `scripts/check-version-sync.sh` ->
  exit 0 (green reachable, `proc-check-green-state-reachable`).
- **G2**: temporarily add `"version": "0.1.0"` back to
  `tauri.conf.json` -> plain run must exit 1 (absence assertion fires);
  revert.
- **G3**: temporarily set `package.json` version to `0.1.1` -> plain run
  must exit 1 (equality arm fires); revert.
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

Then the two dispatch runs, in order. **Run A** (`rehearse-draft-release:
false`) proves the artifact path; **Run B** (`true`) proves draft
assembly. Checklist (every step names its observable):

- **R1** (run A): guard passes both steps; the gate-green step's log
  names the found ci run. All four legs green; the rename step's log
  carries one `pick: <path>` line per selected artifact (1 per Windows
  leg, 1 on macOS, 3 on Linux - the function logs every selection to
  stderr) and its closing `ls -l` lists exactly the renamed files
  (1 / 1 / 4 incl. the tar.gz).
- **R2** (run A): four workflow artifacts named `muxsmith-<leg>` exist
  with retention 7; downloaded together they contain exactly the 7 files
  of D89 with the scheme-conformant names (count check: 7).
- **R3** (run A): **no release was created** (the assemble release step
  shows as skipped in the run's job view - the skip is the observable,
  not an absence-grep; `gh release list` unchanged as corroboration).
- **R4** (run B): draft `rehearsal-<run_id>` exists (created by the
  assemble job's release step), `isDraft: true`, with exactly 8 assets
  (7 + SHA256SUMS). Download all; `sha256sum -c SHA256SUMS` passes.
  Falsifiability control once: corrupt one downloaded byte, watch `-c`
  fail, redownload.
- **R5** (run B): body = rehearsal banner + template with the version
  substituted + generated notes. And in each of the four leg logs, the
  step "Assert no updater artifacts were produced" printed its summary
  line (`updater-artifact check: 0 hits across N bundle output files`,
  N > 0 - the N is the step's built-in positive control that it saw the
  bundle tree; the step's red and control-red states are the G4
  fire-test).
- **R6** (run B or A, artifact-content checks on a Linux machine):
  `dpkg-deb -I muxsmith-*.deb` shows `Recommends: mkvtoolnix` and
  `Depends:` containing `libwebkit2gtk-4.1-0` and `libgtk-3-0`;
  `dpkg-deb -c` lists `./usr/bin/muxsmith` and `./usr/bin/muxsmith-gui`;
  `rpm -qp --recommends muxsmith-*.rpm` prints `mkvtoolnix`;
  `./muxsmith-*.AppImage --appimage-extract` yields
  `squashfs-root/usr/bin/muxsmith`; the tar.gz lists the D88 four-file
  layout under its version-named directory; `msiextract` (msitools) on
  each msi lists `muxsmith.exe` and `muxsmith-gui.exe`.
- **R7** (run B): **no tag ref was created**:
  `git ls-remote origin | grep -c rehearsal` is 0 while the same
  `ls-remote` output demonstrably lists `refs/heads/master` (the
  positive control proving the listing works; the repo currently has no
  tags, so master is the known-present ref).
- **R8** (owner, on real hardware, once per OS): msi installs on a
  Windows machine after the SmartScreen "Run anyway" flow exactly as
  `docs/INSTALL.md` describes; Programs-and-Features shows publisher
  "Şenol Feldmann" unmangled (D86's fallback fires only if this fails);
  dmg opens on an Apple-Silicon Mac via the Settings > Open Anyway flow
  as documented; `muxsmith` runs from the documented per-OS CLI
  locations. These three walk-throughs double as the screenshot source
  for any future doc polish.
- **R9** (run B): version self-report - `./muxsmith --version` from the
  tar.gz prints the Cargo workspace version (D87's artifact-level
  criterion; the guard already pinned tag == that version).
- **R10** (owner): delete the rehearsal draft(s) after inspection.

The first real tag (outside this plan, D81) re-exercises the tag-only
arms under the draft's safety: `--verify-tag`, the guard's tag-equality
check, and the real release name.

---

## 9. Triggers created (for the controller to mirror into the ROADMAP)

1. **`ubuntu-22.04` runner deprecation/retirement announced** -> move
   the Linux release leg (D85) to `ubuntu-24.04`, record the raised
   glibc/webkit floor in `docs/INSTALL.md` and the tar.gz README's
   requirement line in the same change.
2. **A dated windows-arm64 runner label appears** (currently only the
   undated `windows-11-arm` is GA) -> pin it, closing D85's recorded
   pin-everything deviation.
3. **`@tauri-apps/cli` / `tauri` major or minor bump lands (Renovate
   era or manual)** -> re-verify the four bundler facts this design
   pins before the next release: externalBin landing spots (msi
   INSTALLDIR, `Contents/MacOS`, `/usr/bin`), the wix template's
   PATH-feature inertness, the config `version` -> Cargo fallback, and
   `--bundles` value coverage. (Section 1.2 records them at
   tauri-cli-v2.11.4.)
4. **The signing revisit fires (existing ROADMAP trigger)** -> beside
   per-OS signing config: shrink `docs/INSTALL.md` per its embedded
   comment, and evaluate GitHub artifact attestations first (D90's
   record).
5. **The Intel-dmg request fires (existing ROADMAP trigger)** -> add a
   `macos-x86_64` leg on `macos-15-intel` (label verified to exist,
   section 1.3), extend D89's asset set + D77's body table, and decide
   universal-vs-second-dmg then (D78 records why universal lost at
   arm64-only scale).
6. **A user asks for a portable Windows build** -> the mkvtoolnix
   Windows-7z parity gap (section 5) becomes a v1.x candidate: a
   `windows-x86_64.zip` from the existing msi leg's binaries, D88-style.
7. **A user asks for a German installer UI** -> reopen D86's
   `wix.language` single-language decision (the mechanism is a config
   list; the cost is more msi artifacts or a transform decision, which
   is why it waits for a request).
8. **A tar.gz-equivalent bundler lands in Tauri or cargo-dist earns its
   keep for the archive leg** -> revisit D88's hand-packed step.
9. **The gh CLI on runner images breaks a release-ops invocation**
   (it floats with the pinned runner image - the one unpinned tool in
   the path) -> pin gh by direct versioned download in release.yml,
   same shape as every other pin.

---

## 10. Open items

**None.** The one sanctioned pending item the brief carried (the
CLI-distribution assumption) was ruled by the owner mid-authoring
(section 0, note 1) and is designed as settled D82. The ledger-lint
scope question was ruled by the controller (section 7): the wiring rides
Plan 8's plan as a rider task, outside this design; no artifact in this
document depends on it.

---

## 11. What the implementer must not decide

Every fork is closed above; a fork discovered on code contact returns as
NEEDS_CONTEXT with a decision memo (`proc-latitude-clause-boundary`).

- ci.yml is not modified, at all (D83).
- The workflow is exactly one new file `release.yml` with the section-2
  job/step structure; guard semantics, poll cadence (30 s, 90 rounds)
  and per-job permissions as written.
- Runner labels, action SHAs and their comment forms: section 1.4's
  table and section 2's YAML, verbatim; no additional action may be
  introduced (notably: no tauri-action, no softprops, no cache action
  on release legs, no mise-action).
- Per-leg `--bundles` values, the overlay filename
  `src-tauri/tauri.bundle.conf.json` and its exact content, the sidecar
  staging path pattern, and the `.gitignore` line (D82/3.2/3.4).
- The full bundle block of section 3.1 including every literal
  (upgradeCode GUID, publisher spelling with `Ş`, category `Video`,
  section `video`, minimumSystemVersion `11.0`, en-US language list);
  the `version` key is deleted, not moved.
- `scripts/check-version-sync.sh` as written in 3.3; its three red
  states are fire-verified per section 8 G1-G3 before merge.
- The 8-asset name set of D89 character-for-character; the rename step's
  exactly-one-match assertion; retention 7 days.
- tar.gz layout and the four verbatim documentation artifacts of
  section 4 (INSTALL.md, draft-body.md, rehearsal-banner.md,
  linux-tarball-README.txt) plus the BUILDING.md subsection and the
  README placeholder-rider comment text - content changes are owner
  changes.
- Draft-release mechanics: `--draft` always; `--verify-tag` on the tag
  path only; rehearsal name `rehearsal-<run_id>`; body composition
  order; generated notes via the `generate-notes` API endpoint, not
  `--generate-notes`.
- The publisher-rendering fallback (D86) fires only on R8's observable
  failure, and changes exactly the `publisher` field to
  `"Senol Feldmann"`.
- No step publishes, edits or un-drafts a release; no test tag is ever
  pushed (D77/D79/D81).

---

## Amendment log

**A1 (2026-07-23, controller-ruled internal technical fork).** Task 4's
verbatim transcription surfaced that section 2's fence was not loadable
YAML: the step name
`Read pinned node version from mise.toml (D85: no mise in release legs)`
stood as a plain (unquoted) scalar containing `": "`, which YAML forbids -
PyYAML 6.0.3 and Psych both reject at that line (implementer memo:
`.superpowers/sdd/plan-8/task-4-report.md`), so section 11's
verbatim-transcription mandate and the plan's parse check could not both
hold. **Ruling: Option A** - the scalar is double-quoted; the step-name
string, its Actions UI label and the D85 pointer stay byte-identical, only
YAML quoting changes. One fence line changed; no decision content moved,
so no D-entry; the G4/G5 fire-tests and every R-block observable are
untouched (none quotes the YAML line, only the name string, which is
unchanged). Verification, run against the amended fence (extracted as
fence lines 1-222 = the region between the section-2 code-fence markers):

```
$ python3 -c "import yaml; yaml.safe_load(open('fence-post.yml')); print('yaml-ok')"
yaml-ok                        (PyYAML 6.0.3)

$ grep -cnP '^\s*(-\s+)?[a-z_-]+:\s+[^"'"'"'|>&*\[{].*:\s' fence-post.yml
0                              (no other unquoted ": " scalar remains)
```

The zero is fire-verified: the identical grep against the pre-fix fence
returned exactly one hit, fence line 94 (the defective step name), and the
pre-fix PyYAML run rejected at that same line/column ("mapping values are
not allowed here", line 94, column 59) - check red before green, delta
`diff` between the two extracts shows the single quoted line and nothing
else.
