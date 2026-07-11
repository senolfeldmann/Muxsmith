# Building Muxsmith from source

Single source for build prerequisites and commands until the public README
absorbs this at 1.0.

Muxsmith is a Rust workspace (`crates/muxsmith-core`, `crates/muxsmith-cli`,
`crates/xtask`, `src-tauri`) plus a Vue 3 + TypeScript frontend (`src/`) that
`src-tauri` (crate `muxsmith-gui`) wraps into a Tauri 2 desktop app.

## Prerequisites

### Rust toolchain

Pinned via `rust-toolchain.toml` (currently 1.96.1, with `rustfmt` and
`clippy`); `rustup` on a rustup-managed system installs and switches to it
automatically the first time you run `cargo` in this repo. No manual step.

### System libraries (Tauri's native shell)

Tauri needs a platform webview plus a few native headers to compile
`src-tauri`.

**Fedora:**

```bash
sudo dnf install -y webkit2gtk4.1-devel librsvg2-devel libappindicator-gtk3-devel
```

**Debian/Ubuntu** (same set CI installs):

```bash
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

**Windows:** MSVC build tools (Visual Studio Build Tools or Visual Studio
with the "Desktop development with C++" workload) and the WebView2 runtime
(preinstalled on current Windows 10/11; installable separately otherwise).

**macOS:** Xcode Command Line Tools (`xcode-select --install`). The system
WebKit ships with the OS.

### Node + pnpm

The repo pins Node 26.5.0 and pnpm 11.10.0 via `mise.toml` (root, committed);
`pnpm@11.10.0` is also mirrored in `package.json`'s `packageManager` field.

- **mise users** (recommended, matches CI): `mise install` in the repo root
  picks up `mise.toml` and installs the pinned versions.
- **Without mise:** install Node >= 26 yourself, then `corepack enable` so
  Corepack reads `packageManager` and provisions the exact pnpm version.

Verify: `node --version` (v26.x), `pnpm --version` (11.x),
`pkg-config --modversion webkit2gtk-4.1` (Linux only; prints a version once
the system libraries above are installed).

## Building and running

```bash
pnpm install         # frontend + Tauri JS dependencies (frozen lockfile in CI)
pnpm dev             # Tauri dev window: Vite dev server + hot-reloading webview
pnpm build            # vue-tsc type-check + production frontend build
```

`pnpm build` only builds the frontend bundle (`dist/`); it does not invoke
`cargo tauri build`. Building the desktop bundle itself
(`pnpm exec tauri build`) is out of scope for local development and not
part of the CI gate yet.

### The Rust gate (five parts, run from the repo root, workspace-wide)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo deny check
```

`#![deny(missing_docs)]` gates presence of rustdoc comments; the `cargo doc`
run above is what gates their *correctness* (broken intra-doc links and
other rustdoc warnings), which presence-only enforcement cannot catch.

### Frontend checks

```bash
pnpm lint            # eslint (Vue rules, TypeScript rules, D27 no-raw-text)
pnpm check:i18n       # frontend Fluent catalog completeness gate (spec 8.4)
pnpm test:e2e         # Playwright smoke + axe a11y + i18n completeness (type-checks e2e/, builds the harness, then runs)
```

CI (`.github/workflows/ci.yml`) runs the same five-part Rust gate plus
`pnpm lint`, `pnpm build`, `pnpm check:i18n`, and `pnpm test:e2e` on every
push/PR (nine parts total); `cargo deny check` runs as an
independent job.

## Tooling quirks

- vue-tsc + `withDefaults` with `T | null`-typed props has a known quirk
  (surfaced in Plan 5, task 11; details in the archived task-11 report,
  docs/process-journal/artifacts/plan-5-sdd/). Check there before fighting
  a TS error on a withDefaults call.

## Documentation standard

- Rustdoc states MEANING, not a name echo: what the item is for, its
  contract, its edge cases. `#![deny(missing_docs)]` enforces presence
  only; this line carries the quality bar (agreed Plan 1, previously
  chat-only).

## Deliberately not used

Recorded so they are not re-litigated without their reasons (Plan-1
tooling stock-take, 2026-07-08):

- `just` runner: xtask covers every dev task; a second entry point drifts.
- `sccache`: no compile-time pain at this workspace size.
- `cargo-outdated`: Renovate/Dependabot replaces it once activated.
- Coverage tooling (cargo-llvm-cov): revisit at v1.x planning (ROADMAP).

## Cross-target lint rule

Before the first push that hits a new OS leg in CI: run clippy
cross-target locally (`-D warnings` workspaces need cfg-gated imports for
cfg-gated tests) - two Windows-CI red rounds of exactly this class in
Plan 5.
