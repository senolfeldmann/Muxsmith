# Task 4 report: Scaffold src-tauri + Vue frontend + toolchain + CI

Status: **DONE** (all gate commands green locally; push + CI verification explicitly out of scope per controller instructions, see "Pending" below).

Commit: `fe3d2d5` "feat(gui): scaffold Tauri 2 shell + Vue 3 frontend + CI toolchain" on branch `plan5-t4` in worktree `/home/senol/Git/Muxsmith/.worktrees/plan5-t4`.

## What was implemented

### Rust: `muxsmith-gui` crate (`src-tauri/`)

- `src-tauri/Cargo.toml`: package `muxsmith-gui`, `[lib]` target `muxsmith_gui_lib` (staticlib/cdylib/rlib, the standard Tauri bin+lib split) plus `[[bin]] muxsmith-gui`. Deps: `tauri = "2.11.5"`, `tauri-plugin-dialog = "2.7.1"`, `tauri-plugin-clipboard-manager = "2.3.2"`, `serde`/`serde_json` pinned to the exact versions already used elsewhere in the workspace (avoids gratuitous duplicate-version entries). `build-dependencies`: `tauri-build = "2.6.3"`.
- `src-tauri/build.rs`: `tauri_build::build()`.
- `src-tauri/src/lib.rs`: `pub fn run()` builds the `tauri::Builder`, registers both plugins, launches. Documented (not `#![deny(missing_docs)]`, per your instruction that src-tauri is bin-shaped).
- `src-tauri/src/main.rs`: `fn main() { muxsmith_gui_lib::run(); }`, Windows console-suppression attribute.
- `src-tauri/tauri.conf.json`: identifier `io.github.senolfeldmann.muxsmith`, `devUrl` `http://localhost:5173`, `frontendDist` `../dist`, `beforeDevCommand`/`beforeBuildCommand` = `pnpm dev`/`pnpm build`.
- `src-tauri/capabilities/default.json`: minimal permission set (`core:default`, `dialog:allow-open`, `clipboard-manager:allow-write-text`) rather than each plugin's full `:default` bundle - least privilege for a placeholder GUI with no implemented screens yet; extend as features land.
- `src-tauri/icons/`: generated via `pnpm exec tauri icon <placeholder-1024.png> -o src-tauri/icons` (the Tauri CLI's own generator, not ImageMagick - ImageMagick here has no ICNS encoder). The CLI also emitted iOS/Android/Windows-Store variants; I deleted everything except the 5 files `tauri.conf.json`'s `bundle.icon` actually references (`32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.icns`, `icon.ico`) - this project targets desktop only, no mobile/Store scope exists anywhere in the plan. Placeholder artwork (solid color + circle); a real app icon is a separate, later concern.
- Root `Cargo.toml`: added `"src-tauri"` to workspace members.

### Frontend (repo root, matching `frontendDist: "../dist"`)

- `package.json`: `packageManager: "pnpm@11.10.0"`, scripts `dev`/`build`/`preview`/`lint`. All deps exact-pinned (`.npmrc` has `save-exact=true`).
- `vite.config.ts`, `tsconfig.json`, `src/env.d.ts` (needed for `vite/client` types, notably the `?raw` FTL import), `index.html`.
- `src/main.ts`: loads `locales/en/gui-common.ftl` via Vite `?raw`, builds a `FluentBundle`, installs `fluent-vue`.
- `src/App.vue`: `<script setup lang="ts">` placeholder; template renders `{{ $t("app-title") }}` only - no literal text.
- `locales/en/gui-common.ftl`: `app-title = Muxsmith` (new catalog alongside the existing `cli.ftl`/`diagnostics.ftl`).
- `pnpm-workspace.yaml`: auto-created by `pnpm approve-builds vue-demi` (fluent-vue's `vue-demi` dependency has a postinstall script; pnpm 10+ blocks install scripts by default and records approvals here). Not in the brief's literal file list but a necessary, legitimate byproduct - committed so CI's `--frozen-lockfile` install doesn't need an interactive approval prompt.

### Scoping call: where "no hardcoded strings" does and doesn't reach

D27/the i18n-pipe requirement is about the Vue template layer the ESLint rule can see. Three places still carry a literal `"Muxsmith"`, deliberately: `tauri.conf.json`'s `productName`/window `title` (native OS chrome, not rendered through Vue) and `index.html`'s `<title>` (browser/webview tab, same reasoning). None of these pass through Fluent anywhere in the real world (Tauri doesn't localize window titles via the web content pipeline), and none are reachable by the ESLint Vue-template rule. Flagging this explicitly since it's an open dimension the brief didn't spell out.

### Step 2: the empirical lint-gate proof (mandatory per brief)

Added `<p>This is a raw literal string probe for D27.</p>` to `App.vue`, ran `pnpm lint`:

```
/home/.../src/App.vue
  9:8  error  raw text 'This is a raw literal string probe for D27.' is used  @intlify/vue-i18n/no-raw-text

✖ 1 problem (1 error, 0 warnings)
```

**`@intlify/eslint-plugin-vue-i18n`'s `no-raw-text` rule works standalone, confirmed empirically.** No custom fallback rule was needed. `eslint.config.js` registers only that one rule from the plugin (not its `recommended`/`base` presets, which pull in message-catalog rules expecting `settings['vue-i18n'].localeDir` - we use Fluent, not vue-i18n catalogs, so those would misfire). Probe string removed after confirming.

One real bug surfaced and fixed during this step: `typescript-eslint`'s `configs.recommended` sets `languageOptions.parser` **globally** (no `files` restriction) in its `base` config. My first ordering put the Vue-parser override before `...tseslint.configs.recommended`, so tseslint's global block clobbered `vue-eslint-parser` for `.vue` files afterward, and `App.vue`'s `<script setup lang="ts">` failed to parse ("'>' expected", ESLint trying to parse the SFC's opening tag as if it were bare TypeScript). Fixed by reordering: `tseslint.configs.recommended` first, then `eslint-plugin-vue`'s config, then the Vue-specific parser override last (flat config's last-matching-block-wins semantics require this order).

### CI (`.github/workflows/ci.yml`)

Added, before the cargo steps and after the existing mkvtoolnix step: a `runner.os == 'Linux'`-gated apt install of Tauri's build headers (`libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev` - verified byte-for-byte against the current `v2.tauri.app/start/prerequisites/` page via WebFetch, no changes needed), then `jdx/mise-action@e6a8b3978addb5a52f2b4cd9d91eafa7f0ab959d` (verified this is genuinely the v4.2.0 release commit via `gh api repos/jdx/mise-action/git/refs/tags/v4.2.0`) with `cache: true`, then `pnpm install --frozen-lockfile`. After the existing three cargo steps: `pnpm lint`, `pnpm build`. mise-action runs unconditionally (all three OSes need node+pnpm for the frontend steps); only the apt step is Linux-gated.

### `cargo deny check` (Step 4) - the big one

Tauri's dependency tree needed real work here, more than "add a couple of licenses":

**Licenses** - added to the `allow` list, each because `cargo deny check` actually reported it missing: `BSD-3-Clause` (alloc-no-stdlib, via brotli), `Apache-2.0 WITH LLVM-exception` (target-lexicon), `BSL-1.0` (clipboard-win, via arboard on Windows), `Zlib` (foldhash), `MPL-2.0` (cssparser, via Tauri's webview/CSS tooling). **Flagging MPL-2.0 specifically**: it's weak/file-level copyleft, not purely permissive like the rest of the allow-list. Standard and safe to allow as an unmodified dependency (doesn't affect Muxsmith's own MIT license), but it's a different category from the "MIT/Apache/BSD/Zlib/ISC" family your brief anticipated, so worth your explicit awareness rather than a silent addition.

**Advisories** - 18 RUSTSEC IDs added to `ignore`, none discretionary, each with a documented reason in `deny.toml`:
- 5 IDs: the `unic-*` crate family (open-i18n/rust-unic), unmaintained, "no safe upgrade available" per the advisories themselves. Pulled via `urlpattern -> tauri-utils`.
- 11 IDs: the gtk-rs GTK3 bindings family (`gtk`, `gdk`, `atk`, `gdkx11`, etc. + `proc-macro-error` via `glib-macros`) - archived upstream in favor of gtk4-rs. **This is a structural fact about Tauri 2 on Linux today**: its windowing backend (`tao`/`wry`) still depends on gtk-rs 0.18 (GTK3), which the gtk-rs project itself has deprecated. No fix exists at the application level; Tauri would have to migrate to GTK4 bindings. Worth knowing this is real, ongoing supply-chain debt inherited from the framework choice, not something Task 4 introduced or can resolve.
- 2 IDs: quick-xml 0.39.4 DoS advisories (quadratic parse time, unbounded namespace-declaration allocation on untrusted XML). I verified via `cargo tree -i` that this vulnerable version is reachable *only* through `wayland-scanner`, a `(proc-macro)` build-time codegen tool that parses the static, trusted protocol XML bundled inside `wayland-protocols`/`wayland-protocols-wlr` - it never touches untrusted input and isn't linked into the shipped binary. A second, already-patched quick-xml 0.41.0 (via `plist`) is the version actually reachable at runtime. Ignoring these two is a reasoned call based on that reachability analysis, not a blanket suppression - flagging it so you can sanity-check the reasoning rather than just trusting the count.

Re-run `cargo deny check` periodically (comment in `deny.toml` says so); an ID should come out once its crate drops out of `Cargo.lock` on an upstream fix, not linger.

### Toolchain version note: TypeScript 7 vs 6

npm's `typescript@latest` today is `7.0.2` (the native/Go-rewrite line). I tried it first; `pnpm install` failed on a real peer-dependency conflict - `typescript-eslint@8.63.0` (and its whole `@typescript-eslint/*` family) declares `typescript: ">=4.8.4 <6.1.0"`, i.e. the TS tooling ecosystem hasn't caught up to TS 7 yet. Downgraded to `typescript@6.0.3` (latest 6.x, satisfies both `typescript-eslint`'s ceiling and `vue-tsc`'s `>=5.0.0` floor). Confirmed clean: `pnpm install` reports no peer issues, `pnpm build` (`vue-tsc --noEmit && vite build`) passes. This is a deliberate downgrade from npm's bare "latest", not an oversight - noting it since the repo's pin-everything policy treats version choices as decisions, not defaults.

### `BUILDING.md` (Step 5)

Root-level, per-OS prerequisites (Fedora `dnf` line copied verbatim from the T0 commit `c822a17`'s Step 1; Debian/Ubuntu `apt` line matches the CI step; Windows MSVC+WebView2; macOS Xcode CLT), Rust toolchain note (rust-toolchain.toml, 1.96.1), Node/pnpm via mise or corepack, and the full command set. One deliberate discrepancy from a literal reading of the brief: it asks BUILDING.md to document `pnpm lint`/`test:e2e` as "the command set." `test:e2e` (Playwright) isn't implemented in Task 4 - the plan doc (`docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md`) lists Playwright in the overall Plan 5 tech stack, not Task 4's own file list. I documented `pnpm test:e2e` in BUILDING.md with an explicit "not yet implemented (later task)" note rather than either fabricating the script or silently dropping the brief's requested line.

## Gate results (all run in the foreground, in the specified order)

| Command | Result |
|---|---|
| `cargo test --workspace` | pass (all existing suites unchanged; new `muxsmith_gui`/`muxsmith_gui_lib` unit-test binaries: 0 tests, expected for a placeholder) |
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo deny check` | pass (`advisories ok, bans ok, licenses ok, sources ok`; `bans`' `multiple-versions` stays at its existing `warn` policy, unchanged, several duplicate `toml`/`toml_edit` versions in the tree from Tauri's own deps, non-fatal) |
| `pnpm lint` | pass |
| `pnpm build` | pass (`vue-tsc --noEmit && vite build`, 37 modules, ~78 KB bundle) |

## Files changed

New: `.npmrc`, `BUILDING.md`, `eslint.config.js`, `index.html`, `locales/en/gui-common.ftl`, `package.json`, `pnpm-lock.yaml`, `pnpm-workspace.yaml`, `src-tauri/{Cargo.toml,build.rs,capabilities/default.json,icons/*,src/lib.rs,src/main.rs,tauri.conf.json}`, `src/{App.vue,env.d.ts,main.ts}`, `tsconfig.json`, `vite.config.ts`.

Modified: `.github/workflows/ci.yml`, `.gitignore`, `Cargo.lock`, `Cargo.toml`, `deny.toml`.

## Self-review findings (fixed before commit)

1. **eslint.config.js block-order bug** (typescript-eslint's global parser override clobbering the Vue parser) - caught by actually running Step 2's probe, not just reading the config. Fixed, documented in a comment.
2. **Icon over-generation** - the Tauri CLI's icon generator produces iOS/Android/Windows-Store assets beyond this project's desktop-only scope; pruned to exactly what `tauri.conf.json` references.
3. **Comment referencing an ephemeral path** - `eslint.config.js` originally pointed a reader at `task-4-report.md` inside `.superpowers/`, which is entirely gitignored; a dangling reference for anyone who clones the repo fresh. Rewrote the comment to be self-contained.
4. **`zsh`'s `path` special variable** - purely a local research-tooling gotcha (assigning a shell variable named `path` silently clobbers `$PATH` in zsh), not a repo issue; noting only because it cost real time during crates.io version lookups.

## Pending (explicitly out of scope per controller instructions)

- **Push branch `plan5-t4` and verify the CI Linux job passes** (brief's Step 6, second half). Not done - the controller handles pushes. Everything that can be verified locally (the full six-command gate) is green; the only thing untested is the actual GitHub Actions environment (runner-provided `apt`/mise-action network behavior, GitHub's cache backend). Local `cargo build -p muxsmith-gui` already confirms the webkit2gtk-4.1 headers and full plugin stack compile end to end on this machine's already-matching dependency versions (2.52.4), which is the same version CI's apt install will pull on `ubuntu-26.04`.

---

# Fix report (review round 1, commit `46c7874`)

Commit: `46c7874` "fix(gui): review fixes - eslint 9.39.4, vue-demi approval comment, deny.toml grouping" on `plan5-t4`, on top of `fe3d2d5`.

## 1. IMPORTANT: eslint repin (9.9.1 -> 9.39.4)

- Resolved against the registry, not memory: `mise exec -- pnpm view eslint version` -> `10.6.0` (true latest is a 10.x major); latest stable 9.x from `pnpm view eslint versions` -> **9.39.4**. Pinned exactly, per the adjudicated rule (stay on latest 9.x when latest is a major).
- Peer ranges confirmed via `pnpm view <pkg> peerDependencies`:
  - `eslint-plugin-vue@10.9.2`: `^8.57.0 || ^9.0.0 || ^10.0.0` - accepts 9.39.4
  - `@intlify/eslint-plugin-vue-i18n@4.5.1`: `^8.0.0 || ^9.0.0-0 || ^10.0.0` - accepts 9.39.4
  - `typescript-eslint@8.63.0`: `^8.57.0 || ^9.0.0 || ^10.0.0` - accepts 9.39.4
- **Correction to the review's anticipated rationale**: all three peers also accept `^10.0.0`, so "peer ceilings" is NOT what keeps us off eslint 10. The binding reason is the brief's interface spec (`eslint@9`) plus the coordinator directive; a 10.x major bump is available and would be a separate deliberate decision. Stated as such in the commit message.
- `pnpm-lock.yaml` updated via `mise exec -- pnpm install`; `pnpm peers check` reports "No peer dependency issues found".

## 2. MINOR: pnpm-workspace.yaml comment

Added the explanatory header: vue-demi (fluent-vue dependency) needs its postinstall to select the Vue-2/Vue-3 build; pnpm blocks install scripts by default and CI's `--frozen-lockfile` install cannot approve interactively, hence the recorded approval.

## 3. MINOR: deny.toml grouping

`RUSTSEC-2024-0370` (proc-macro-error) moved out from under the "gtk-rs GTK3 bindings archived upstream" header into its own commented entry: unmaintained in its own right (its own advisory), reached as glib-macros' proc-macro dependency via the same Tauri Linux stack.

## Commands run and outputs (all foreground)

| Command | Result |
|---|---|
| `mise exec -- pnpm install` | pass; lockfile updated (`- eslint 9.9.1` / `+ eslint 9.39.4`) |
| `mise exec -- pnpm peers check` | "No peer dependency issues found" |
| `mise exec -- pnpm lint` | pass |
| `mise exec -- pnpm build` | pass (vue-tsc + vite, 37 modules, identical bundle) |
| `cargo deny check` | exit 0 (re-run because deny.toml was touched, comment-only edit) |
| deny.toml TOML parse / pnpm-workspace.yaml YAML parse | both OK |
| D27 probe re-proof under eslint 9.39.4 | raw-text probe fails lint with `@intlify/vue-i18n/no-raw-text`, removed, lint green again |

Cargo gates (`test`/`fmt`/`clippy`) not re-run: no Rust source, manifest, or lockfile-affecting change in this commit (deny.toml edit is comment-only and covered by the `cargo deny check` re-run above).

---

# Fix report (review round 2, commit `63fdfc4`)

Controller decision: eslint@9 in the brief was a plan-text default, not a constraint; policy is newest major when nothing blocks it. Bumped.

Commit: `63fdfc4` "build(gui): bump eslint to 10.6.0 (newest major, controller decision)" on `plan5-t4`, on top of `46c7874`. Files: `package.json` (exact pin `10.6.0`), `pnpm-lock.yaml`.

- Registry re-verified before pinning: `mise exec -- pnpm view eslint version` -> `10.6.0`.
- Running binary confirmed: `pnpm exec eslint --version` -> `v10.6.0`.

## Commands and outputs (all foreground)

| Command | Result |
|---|---|
| `mise exec -- pnpm install` | pass; lockfile updated (`- eslint 9.39.4` / `+ eslint 10.6.0`), no peer warnings |
| `mise exec -- pnpm peers check` | "No peer dependency issues found" |
| `mise exec -- pnpm lint` | pass |
| `mise exec -- pnpm build` | pass (vue-tsc + vite, 37 modules, identical bundle) |
| D27 probe under eslint 10 | raw template literal -> lint FAILS, exit 1, exactly one error: `@intlify/vue-i18n/no-raw-text` ("raw text 'probe raw text eslint10' is used"); probe removed -> lint green |

No breakage from the major: the intlify `no-raw-text` rule and the flat-config setup (tseslint-first block ordering, Vue parser override) behave identically under 10.6.0. No revert needed. Cargo gates untouched (JS-only change).
