# Inverse dependency sweep (dep dimension) — find-X2

Scope: every DIRECT dependency across the six manifests, judged on (a) earned, (b) healthy (registry-verified 2026-07-12 via crates.io API / npm registry API / GitHub, never from memory), (c) overlapping. Transitive deps out of scope. deny.toml read for context; its known ignores (rust-unic `unic-*`, proc-macro-error, GTK3 bindings, quick-xml build-time) not re-reported.

Manifests enumerated (exhaustive `find`, no others exist):

- `/home/senol/Git/Muxsmith/Cargo.toml` (workspace root — no `[workspace.dependencies]`; all deps are per-crate)
- `/home/senol/Git/Muxsmith/crates/muxsmith-core/Cargo.toml`
- `/home/senol/Git/Muxsmith/crates/muxsmith-cli/Cargo.toml`
- `/home/senol/Git/Muxsmith/crates/xtask/Cargo.toml`
- `/home/senol/Git/Muxsmith/src-tauri/Cargo.toml`
- `/home/senol/Git/Muxsmith/package.json`

**Result: no findings. All direct dependencies are earned, registry-healthy, and non-overlapping.** Two watch-items and two notes below; none rises to an actionable finding.

## Rust — muxsmith-core

| Dep | Version | Earned | Healthy (registry) | Verdict |
|---|---|---|---|---|
| dirs | 6.0.0 | config_dir/data_dir (D26/D27); no stdlib equivalent (std has only home_dir) | latest, 2025-01, 267M dl | clear |
| language-tags | 0.3.2 | full RFC 5646 parse + registry-based `canonicalize()` in matcher.rs (iw→he, pt-Latn-BR→pt-BR) and BCP47 well-formedness gate in capability/runtime.rs | latest, but **last release 2021-05**; repo not archived, no RUSTSEC advisory, 83M dl | **watch-item 1** (below) |
| regex | 1.12.4 | matcher regex support | healthy, 1.13.0 current (caret req covers); recompilation is a known non-finding | clear |
| schemars | 1.2.1 | profile JSON-schema generation (match_expr, model, CLI schema output) | latest, 2026-02 | clear |
| serde / serde_json | 1.0.228 / 1.0.150 | core serialization; mkvmerge -J | latest / healthy | clear |
| serde_path_to_error | 0.1.20 | path-qualified profile-parse diagnostics (profile/load.rs) | latest, 2025-09, dtolnay | clear |
| time (formatting) | 0.3.53 | RFC3339 timestamps (joblog, run.rs); std cannot format dates | latest, 2026-07-01 | clear |
| yaml_serde | 0.10.4 | YAML profile format | latest, 2026-03. **Positively verified as the official serde_yaml successor**: crates.io owner is ingydotnet (YAML co-creator), repo github.com/yaml/yaml-serde under the YAML Organization. Correct pick over archived serde_yaml and the unmaintained serde_yml fork | clear |
| proptest (dev) | =1.11.0 | property tests | max_stable confirmed 1.11.0 (matches in-manifest verification comment) | clear |
| tempfile (dev) | 3.27.0 | test dirs | latest, 2026-03 | clear |

## Rust — muxsmith-cli

| Dep | Version | Earned | Healthy (registry) | Verdict |
|---|---|---|---|---|
| clap (derive) | 4.6.1 | CLI parsing | latest, 2026-04 | clear |
| ctrlc | 3.5.2 | SIGINT handler for graceful job interruption (commands/run.rs); no stdlib signal handling; minimal vs signal-hook | latest, 2026-02 | clear |
| fluent-bundle | 0.16.0 | Fluent (.ftl) i18n is toolchain ground truth; the only serious Rust Fluent impl | max_stable 0.16.0, 2025-05; projectfluent/fluent-rs repo active (release May 2025, ongoing PRs, no unmaintained notice) | clear, **watch-item 2** |
| muxsmith-core | path | workspace | — | clear |
| schemars / serde_json | reuse | schema command / JSON output | latest / healthy | clear |
| sys-locale | 0.3.2 | cross-platform system-locale detection (i18n.rs fallback); env-var reading only works on Unix | latest, 2024-11; 1Password repo, not archived, stable-purpose crate, 24M dl | clear |
| unic-langid | 0.9.6 | forced by fluent-bundle's public API (`LanguageIdentifier`) | latest, 2025-05. **Disambiguation:** this is a projectfluent crate, NOT part of the abandoned rust-unic project whose `unic-*` crates deny.toml ignores (unic-char-range etc.); no advisory against unic-langid | clear |
| assert_cmd (dev) | 2.2.2 | CLI binary tests | latest, 2026-05 | clear |
| fluent-syntax (dev) | 0.12.0 | catalog-completeness test parses .ftl AST | latest, 2025-05, same fluent-rs repo | clear |
| insta (dev, filters) | =1.48.0 | CLI snapshot tests | max_stable confirmed 1.48.0 (matches in-manifest verification comment) | clear |
| regex / tempfile (dev) | reuse | documented in-manifest as reuse | healthy | clear |

## Rust — xtask

| Dep | Version | Earned | Healthy | Verdict |
|---|---|---|---|---|
| serde_json | "1" | codegen reads schema JSON | healthy | clear; note 2 (loose req) |

## Rust — src-tauri (muxsmith-gui)

| Dep | Version | Earned | Healthy (registry) | Verdict |
|---|---|---|---|---|
| tauri-build (build) | 2.6.3 | Tauri 2 scaffold | current stable | clear |
| tauri | 2.11.5 | app shell | latest, 2026-07-01 | clear |
| tauri-plugin-clipboard-manager | 2.3.2 | used (lib.rs init + 2 frontend imports) | latest | clear |
| tauri-plugin-dialog | 2.7.1 | used (lib.rs init, run.rs dialogs, 4 frontend imports) | latest | clear |
| tauri-plugin-fs | 2.5.1 | used (lib.rs init + frontend import) | latest | clear |
| tauri-plugin-os | 2.3.2 | used (lib.rs init + frontend import) | latest | clear |
| dirs | 6.0.0 | reuse; dirs::config_dir over Tauri path API is a documented decision (settings.rs module doc: keeps load/save pure, consistent with core's D26) | latest | clear |
| serde / serde_json / time | reuse | IPC payloads / RFC3339 | latest | clear |
| muxsmith-core | path | workspace | — | clear |
| tempfile (dev) | 3.27.0 | reuse | latest | clear |

## npm — dependencies

| Dep | Version | Earned | Healthy (npm, 2026-07-12) | Verdict |
|---|---|---|---|---|
| @fluent/bundle | 0.19.1 | Fluent runtime in the webview (4 imports) | latest, 2025-04, not deprecated; Mozilla reference JS impl | clear, watch-item 2 |
| @tauri-apps/api | 2.11.1 | IPC/core/event/mocks (5 imports) | latest, 2026-06 | clear |
| @tauri-apps/plugin-* (clipboard-manager 2.3.2, dialog 2.7.1, fs 2.5.1, os 2.3.2) | — | JS side of the four Rust plugins, versions matched pairwise; all imported | all latest, not deprecated | clear |
| fluent-vue | 3.8.2 | the established Fluent-Vue integration (3 imports); wraps @fluent/bundle | latest, 2026-02-16; repo active, Vue 3 supported, no deprecation | clear |
| vue | 3.5.39 | framework | latest, 2026-06 | clear |

## npm — devDependencies

| Dep | Version | Earned | Healthy | Verdict |
|---|---|---|---|---|
| @axe-core/playwright | 4.12.1 | a11y assertions in e2e/smoke.spec.ts | latest, 2026-06 | clear |
| @intlify/eslint-plugin-vue-i18n | 4.5.1 | deliberately used for exactly one runtime-independent rule (`no-raw-text`), rationale documented in eslint.config.js; no fluent-vue-specific ESLint plugin exists | latest, 2026-06 | clear |
| @playwright/test | 1.61.1 | e2e | latest | clear |
| @tauri-apps/cli | 2.11.4 | tauri dev/build | latest | clear |
| @types/node | 26.1.1 | matches node 26.5.0 | latest | clear |
| @vitejs/plugin-vue | 6.0.7 | Vue SFC in Vite | latest | clear |
| eslint | 10.6.0 | lint | 10.7.0 published 2026-07-10; two-day lag, Renovate territory, not a health issue | clear |
| eslint-plugin-vue | 10.9.2 | Vue lint rules (disjoint from the i18n plugin's job — no overlap) | latest | clear |
| typescript | 6.0.3 | — | held under typescript-eslint ceiling: recorded decision (Renovate rider S14), not flagged | clear (known non-finding) |
| typescript-eslint | 8.63.0 | TS lint | latest | clear |
| vite | 8.1.4 | bundler | latest | clear |
| vue-tsc | 3.3.7 | template typecheck | latest | clear |

## Overlap analysis (pairs evaluated, all cleared)

- **language-tags vs unic-langid** — the one real candidate. Not consolidatable: unic-langid is a minimal Unicode Language Identifier (no full BCP47 grammar, no registry, no canonicalization) and is dictated by fluent-bundle's API; language-tags provides the registry-backed `canonicalize()` the matcher's language equality depends on. Distinct jobs in distinct crates.
- **fluent-bundle (Rust) + @fluent/bundle (JS)** — same job in two runtimes, inherent to CLI (native i18n) vs GUI (webview i18n) with one shared .ftl catalog source. Not a duplication that could be cut.
- **dirs vs Tauri's path resolver** — documented decision in settings.rs; core needs dirs regardless (D26).
- **serde_json vs yaml_serde** — different formats, both load-bearing (mkvmerge -J vs profile YAML).
- **eslint-plugin-vue vs @intlify/eslint-plugin-vue-i18n** — disjoint rule sets.
- **ctrlc vs signal-hook** — ctrlc is the minimal fit; no second signal dep present.

## Watch-items (non-findings, keep decisions)

1. **language-tags 0.3.2 is dormant** (last release 2021-05; 4 open issues; its own README points lighter users to oxilangtag). Keep is still correct: no RUSTSEC advisory, frozen-spec domain, 83M downloads, and neither alternative fits — oxilangtag deliberately ships no subtag-registry database (loses `canonicalize()`), ICU4X's locale stack would add a far heavier tree for the same result. One real limitation to be aware of: its IANA subtag-registry snapshot is ~2021, so registry entries added since then won't canonicalize; harmless here because D19 defers final language validation to mkvmerge at mux time. Re-check at the next pre-release sweep; migrate only if an advisory lands or ICU4X becomes justified by other needs.
2. **The Fluent ecosystem moves slowly on both sides** (fluent-bundle 0.16.0 from 2025-05, @fluent/bundle 0.19.1 from 2025-04). Both repos are alive (active PRs/releases, no unmaintained notices) and both are the reference implementations with no successor; Fluent being toolchain ground truth makes this a monitoring point, not an action.

## Notes (out of the dep dimension, recorded for completeness)

1. **yaml_serde positively verified** as the YAML Organization's official serde_yaml continuation (owner ingydotnet, repo yaml/yaml-serde) — worth recording because the serde_yaml successor field is littered with bad forks (serde_yml is itself now unmaintained) and this pick is the right one.
2. **xtask declares `serde_json = "1"`** while every other manifest writes `1.0.150` (crates/xtask/Cargo.toml:9). Same resolved version via the workspace lockfile; purely a version-requirement style inconsistency with the pin-everything doctrine, not an earned/health/overlap issue and therefore not a finding of this sweep.

## Routed incidentals

None — no correctness/security/performance issues spotted during the sweep.
