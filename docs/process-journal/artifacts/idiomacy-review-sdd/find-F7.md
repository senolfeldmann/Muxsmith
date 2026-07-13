# Idiomacy review - slice F7 (build/config/toolchain files + CI)

Scope: Cargo.toml (root), crates/muxsmith-core/Cargo.toml, crates/muxsmith-cli/Cargo.toml,
crates/xtask/Cargo.toml, src-tauri/Cargo.toml, deny.toml, rust-toolchain.toml, mise.toml,
package.json, pnpm-workspace.yaml, tsconfig.json, vite.config.ts, eslint.config.js,
src-tauri/tauri.conf.json, .github/workflows/ci.yml (only workflow file).
All files read completely. Idiom claims verified against current official docs
(cargo book, rustup blog/inside-rust 2026-07-03, cargo-deny book, pnpm 11 docs,
typescript-eslint docs, eslint-plugin-vue user guide, action.yml of the two actions).

## Findings

### F7-1 [idiom] ci.yml:30 - `rustup show` as toolchain installer is the pre-1.28 idiom, now a no-op installer

The step "Install pinned Rust toolchain" runs `rustup show`. Since rustup 1.28.0
(2025-03) `rustup show` no longer installs the active toolchain as a side effect;
GitHub runners ship rustup >= 1.28. The job still works only because 1.28.1
re-enabled implicit installation *on proxy invocations* (the first `cargo`/`rustc`
call installs the pinned 1.96.1), i.e. the named install step installs nothing and
the real install happens invisibly inside Swatinem/rust-cache's or `cargo fmt`'s
first rustc invocation. The rustup team explicitly recommends against `rustup show`
for installation (inside-rust post, 2026-07-03, "plans for the 1.30 release cycle")
and 1.30 tightens implicit installation further.

Replacement: `run: rustup toolchain install` (no arguments; reads
rust-toolchain.toml including the rustfmt/clippy components). The step comment's
"rustup on the runner auto-installs it" should die with it.
lines_cut: 0.

### F7-2 [idiom] Cargo.toml:2 - `resolver = "2"` on an all-edition-2024 workspace; the edition's default is `"3"`

Every member inherits `edition = "2024"` via `[workspace.package]`. Cargo's
documented default resolver for edition 2024 is `"3"` (MSRV-aware
`incompatible-rust-versions = "fallback"`; requires Rust >= 1.84, satisfied by the
pinned 1.96.1). A virtual workspace must state the resolver explicitly - having the
key is right - but the stated value is the edition-2021 default, so the workspace
root fights the edition the workspace itself pins. Behavior-neutral here (Cargo.lock
committed, deps pinned), which is exactly why the divergence carries no benefit.

Replacement: `resolver = "3"`.
lines_cut: 0.

### F7-3 [idiom] eslint.config.js:14 - `tseslint.config()` is deprecated by the pinned typescript-eslint

typescript-eslint deprecated the `config()` helper in favor of ESLint core's
`defineConfig` (typescript-eslint issue #10935; the packages/typescript-eslint doc
page states the deprecation; getting-started shows `defineConfig` from
`eslint/config`). Pinned typescript-eslint 8.63.0 is well past the deprecation;
pinned ESLint 10.6.0 ships `defineConfig`. Drop-in here: none of the composed
configs hit the two documented behavior differences (files-intersection on extends,
config typing).

Replacement: `import { defineConfig } from "eslint/config"; export default defineConfig(...)`.
(eslint-plugin-vue's own user guide still shows `ts.config(...)`, but the authority
for the helper is its own package, which deprecates it.)
lines_cut: 0.

### F7-4 [idiom] crates/xtask/Cargo.toml:9 - `serde_json = "1"` floats; only unpinned dependency requirement in the workspace

Every other dependency requirement in all four member manifests is a full
`major.minor.patch` pin per the repo's own exact-pin doctrine (rust-toolchain.toml
comment, ci.yml pinning-policy block); xtask alone says `"1"`, which accepts any
1.x. It is also the same crate pinned as `1.0.150` in the other three manifests,
kept in sync there by comments (muxsmith-cli's regex note tracks cross-manifest
consistency by hand). Cargo's native mechanism for a shared pinned dep is
`[workspace.dependencies]` + `serde_json.workspace = true`.

Replacement: minimum fix `serde_json = "1.0.150"`; the Cargo-idiomatic fix is
workspace dependency inheritance for the shared deps (serde, serde_json, dirs,
time, tempfile, regex, schemars). The inheritance aspect straddles the excluded
cross-file-duplication dimension - flagged here only because workspace inheritance
is Cargo's own convention for it; merge stage should dedupe against the dup sweep.
lines_cut: 0.

### F7-5 [yagni] ci.yml:5 - `tags: ['v*']` push trigger has no consumer

ci.yml is the only workflow; no job or step conditions on `startsWith(github.ref,
'refs/tags/')` and there is no release/bundle job. A `v*` tag push re-runs exactly
the CI that already ran for the branch push of the same commit. Dead trigger until
a release pipeline exists.

Replacement: delete the line (re-add together with the future release job).
lines_cut: 1.

### F7-6 [yagni] deny.toml:6,49 - `version = 2` in `[advisories]` and `[licenses]` is a dead key

Current cargo-deny docs (both cfg pages) state the `version` field "is no longer
used"; version-2 semantics are the only behavior, and the old version-1 fields now
error outright. cargo-deny-action v2.0.20 ships a cargo-deny far past the 0.16
cutover. Both keys are inert config.

Replacement: delete both `version = 2` lines.
lines_cut: 2.

### F7-7 [yagni] crates/xtask/Cargo.toml:11-12 - `[lib] path = "src/lib.rs"` restates Cargo's auto-discovery default

`src/lib.rs` is exactly where Cargo auto-discovers the lib target; the table adds
nothing (unlike muxsmith-cli's `[[bin]]`, where `name`/`path` are load-bearing to
rename the binary, or src-tauri's `[lib]`, which sets `crate-type`).

Replacement: delete the `[lib]` table (and its preceding blank line).
lines_cut: 3.

### F7-8 [yagni] ci.yml:137-138 - `with: command: check` restates cargo-deny-action's default

The action's action.yml defaults `command` to `check`.

Replacement: delete the `with:` block, leaving the bare `uses:`.
lines_cut: 2.

### F7-9 [yagni] ci.yml:74-75 - `with: cache: true` restates mise-action's default

jdx/mise-action defaults `cache` to `true`.

Replacement: delete the `with:` block.
lines_cut: 2.

## Routed (not findings; correctness/security/performance lane)

1. **ci.yml (workflow level) - no `permissions:` block** [security]. The
   GITHUB_TOKEN gets the repository default (potentially write-all). GitHub's
   hardening guidance and OSSF Scorecard expect explicit least privilege; for this
   workflow `permissions: contents: read` at top level suffices. Notably the one
   gap in an otherwise thorough supply-chain posture (SHA pins, pinned runners,
   cargo-deny sources gate).
2. **ci.yml:73 - mise-action downloads the *latest* mise binary at runtime**
   [security/supply-chain]. No `version:` input is set; the SHA pin covers the
   action's JS, not the floating mise release it fetches and executes. Against the
   repo's own pin-everything doctrine, this is an unpinned executable on every run.
3. **ci.yml:80+104 - the full workspace test suite runs twice per matrix leg**
   [performance]. Line 80 runs it parallel, line 104 again single-threaded with
   `--nocapture` for the skip-marker grep. The second invocation alone would gate
   both test failures (exit code propagates through `tee` only with pipefail -
   GitHub's `shell: bash` default includes `-o pipefail`, so it does) and silent
   skips; dropping the line-80 run roughly halves Rust test time on all three OS
   legs.

## Checked and clean (deliberate non-findings)

- **pnpm-workspace.yaml `allowBuilds: vue-demi: true`** - exactly the pnpm 11
  convention (v11 replaced `onlyBuiltDependencies` et al. with the `allowBuilds`
  map); the file doubles as pnpm's settings carrier per pnpm 10+ docs, `packages:`
  not required.
- **vite.config.ts** - matches Tauri 2's documented Vite contract (fixed port,
  strictPort, src-tauri ignore); Vite-idiomatic otherwise.
- **tauri.conf.json** - canonical Tauri 2 shape: `$schema`, reverse-DNS identifier,
  documented CSP object form incl. `ipc: http://ipc.localhost`, standard build/
  bundle blocks.
- **tsconfig.json** - Vite-idiomatic (`moduleResolution: "Bundler"`, `noEmit`,
  `types: ["vite/client"]`). create-vue's project-references split
  (tsconfig.app/node) was considered and rejected as scale-inappropriate for one
  app tsconfig; vite.config.ts type-checks fine in the single config since it uses
  no Node globals.
- **eslint.config.js structure** - global-ignores object, tseslint-before-vue
  ordering (correct for flat-config parser precedence, well commented),
  `parserOptions.parser = tseslint.parser` for SFCs, and cherry-picking only
  `no-raw-text` from the intlify plugin are all per the respective plugins' docs.
  `pluginVue.configs["flat/recommended"]` is still the documented v10 name.
- **muxsmith-cli `[[bin]] name/path`** - load-bearing (binary named differently
  from the package); idiomatic.
- **src-tauri `crate-type = ["staticlib", "cdylib", "rlib"]`** - Tauri 2 template
  convention; kept.
- **rust-toolchain.toml, mise.toml** - minimal, idiomatic.
- Recorded decisions honored: exact pins / SHA pins, TS 6.0.3 ceiling, proptest/
  insta exact pins, choco/brew mkvtoolnix pin asymmetry (documented in ci.yml).
- **package.json `packageManager` + mise.toml pnpm pin** - two version sources for
  pnpm (ci.yml comment claims mise.toml is the single source). Cross-file
  duplication -> dup sweep's lane, noted here only for the merge stage.

Totals: 9 findings (4 idiom, 5 yagni), lines_cut 10, deps_cut 0. Not clean.
