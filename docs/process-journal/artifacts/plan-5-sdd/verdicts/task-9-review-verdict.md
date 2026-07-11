<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-9  (round 1 of 1)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01GFwsiJtECvTY74aqRgH2ev
  agent_id:           a450561ca117101f5
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a450561ca117101f5.jsonl
  dispatch_desc:      Review Task 9 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-10T15:39:32.208Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

- ✅ Single window shell: `<nav>` (Batch | Jobs, `aria-current="page"` via ternary-to-`undefined`) + `<main>`, view switch is `ref<'batch'|'jobs'>`, no router added, placeholders in `src/views/BatchView.vue`/`JobsView.vue` with matching `data-testid`. `App.vue:449-533`.
- ✅ `src/i18n/index.ts` loads `gui-*.ftl` + `diagnostics.ftl` per locale directory via `import.meta.glob({ query: "?raw", ... })`. Verified outside-diff: `locales/en/diagnostics.ftl` exists and is picked up by the glob pattern; `cli.ftl` correctly excluded. Verified the project is on Vite `8.1.4` (pnpm-lock.yaml), for which `{ query: "?raw", import: "default" }` is the correct current glob option shape (not the pre-Vite-5 `{ as: "raw" }`), so the loader is not silently dead code.
- ✅ Locale fallback chain: `main.ts` resolves `settings.locale ?? navigator.language` once, `buildBundles()` builds `[requested, "en"]` deduped, matching "settings -> system -> en per message."
- ✅ Diagnostics render via the same code+params mechanism the CLI's `diagnostics.ftl` uses (spec 8.4): T9 doesn't render core `Diagnostic`s itself (correctly deferred to T10, which owns `ReportDocument.*_diagnostics`), but it wires `diagnostics.ftl` into the same bundle chain gui code uses, so T10/T11's `$t(diagnostic.code, diagnostic.params)` will resolve against the CLI's actual catalog.
- ✅ FirstRun.vue: `detect_mkvmerge` on mount (in `App.vue`, not FirstRun itself, correctly delegated as a prop), full-screen per-OS guidance, manual path picker via `@tauri-apps/plugin-dialog`, `set_settings` then re-detect (`FirstRun.vue:1178-1195`). Confirmed no auto-retry/spin risk: every re-detect is button-triggered, gated by `busy`.
- ✅ SettingsDialog.vue: native `<dialog>`, `label for`/`id` on all three fields, matches brief exactly.
- ✅ `src/ipc.ts`: `IpcError`, `MkvmergeInfo`, `AppSettings` field names match the brief's literal shape exactly; `set_settings`'s single Rust parameter (`settings`) confirmed against the visible `lib.rs` diff context. Rust param-name/casing for the remaining 8 commands (validate_profile, dry_run, start_run, etc.) is not visible in this diff (pre-existing from T6-T8) — camelCasing of `run_id` -> `runId` is correct per Tauri's documented default IPC convention, but full verification of every argument name against pre-existing Rust signatures is outside this diff. ⚠️ (not blocking T9; becomes load-bearing when T10/T11 actually call these).
- ✅ D29 a11y: semantic HTML (`<header>`, `<nav>`, `<main>`, `<dialog>`), Fluent-sourced names throughout, `data-testid` on every structural node claimed in the report, `label for`/`id` on every form field, `aria-live`/`role="alert"`/`aria-busy` used consistently. Sets a solid, consistent template for T10/T11.
- ✅ No hardcoded user-facing strings: scanned the full diff, every `title`/`aria-label` is `:`-bound to `$t(...)`; only non-user-facing string is a `console.warn` dev log.
- ✅ Zero-new-deps constraint: one exception (`@tauri-apps/plugin-os` + `tauri-plugin-os`, both exact `2.3.2`), flagged and independently verified below.

**Deviation 1 (`@tauri-apps/plugin-os`) — verified correct.** Confirmed via `node_modules/@tauri-apps/api/*.d.ts`: no `os.d.ts` exists in `@tauri-apps/api@2.11.1` — the brief's `platform()` from `@tauri-apps/api` is indeed stale. Confirmed via `node_modules/@tauri-apps/plugin-os/dist-js/index.d.ts`: `platform(): Platform` is synchronous (value set at compile time via a global `__TAURI_OS_PLUGIN_INTERNALS__`), matching the diff's un-awaited `platform()` call in `FirstRun.vue:1143`. Confirmed via the Rust crate's `permissions/default.toml` (`~/.cargo/registry/.../tauri-plugin-os-2.3.2`) that `os:default` grants `allow-platform` — capability entry is correct. Confirmed `tauri_plugin_os::init()` signature matches `src/lib.rs:130` of the crate. Pins are exact on both sides. This deviation is real, well-justified, and correctly implemented.

**Deviation 2 (eslint `no-raw-text` attributes option) — verified correct.** Read the installed rule source (`node_modules/@intlify/eslint-plugin-vue-i18n/dist/rules/no-raw-text.js`): `config.attributes = []` when no option given (confirms the "never checked" claim), and `VAttribute` visitor explicitly `return`s early for `node.directive` (confirms `:title=`/`:aria-label=` bindings were always out of the rule's reach regardless of the option). The added shape (`{ "/.*/": ["title", "aria-label", "placeholder", "alt"] }`) matches the rule's JSON schema (`patternProperties` keyed by tag-name-or-regex). Scan of the diff's own new templates found zero static (non-bound) instances of these attributes, consistent with the empirical RED/GREEN claim.

**Deviation 3 (`gen/schemas` unchanged) — confirmed non-issue.** `src-tauri/gen/` is listed in `.gitignore` and untracked (`git check-ignore` confirms). Correctly assessed as expected, not a smell.

**Verified via targeted Rust source check:** every `IpcError` param name referenced in the new Fluent messages (`$found`/`$minimum`/`$detail`) matches `.with("found", ...)`/`.with("minimum", ...)`/`.with("detail", ...)` in `src-tauri/src/error.rs:81-95` exactly — the diagnostics-style rendering is genuinely wired correctly end to end, not just plausible-looking.

### Strengths
- Both flagged deviations are exactly the kind of "verify, don't assume" work the framework wants: independently reproducible from `node_modules`/`~/.cargo/registry` sources, not just asserted in the report.
- `i18n/index.ts`'s glob-by-directory design genuinely eliminates loader edits for T10/T11's own `.ftl` files and for a future locale — verified against the actual Vite version in use, not just claimed.
- Defensive `AppSettings` merge pattern (`{...baseline, <only the form's own fields>}`) in both `SettingsDialog.save()` and `FirstRun.attempt()` correctly avoids clobbering `recent_profiles`/`dir_memory`, and both re-fetch fresh settings rather than trusting a stale cached copy.
- `ReportDocument.files[].plan`/`suggestions` left as `unknown` rather than half-mirrored is the right call — avoids a fake sense of type safety for a shape this task never reads.
- A11y pattern is consistent and complete across all three new views/components, not just present in one and missing in another.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- `src/i18n/index.ts:743-750` — `catalogsForLocale` matches the locale tag by exact string equality against the `locales/<tag>/` directory name. `navigator.language` commonly reports region-qualified BCP-47 tags (`en-US`, `de-DE`). This is harmless today (the unconditional `"en"` fallback entry absorbs any non-match, verified: v1 ships only `locales/en/`), but is a latent gap for the second locale: a system reporting `de-DE` against a future `locales/de/` directory would silently skip the matching catalog and fall through to English, undermining the "pure content addition" claim the loader design otherwise earns. Worth a primary-subtag normalization (`locale.split("-")[0]`) whenever locale #2 lands.
- `SettingsDialog.vue:596-601` and `FirstRun.vue:1166-1171` — the `browse()` file-picker call is not covered by the component's `busy`/`aria-busy` state, unlike every other async action in both components. Inconsistent with the otherwise-thorough busy-state discipline; low impact since native OS file pickers are effectively blocking anyway.

### Assessment
**Task quality:** Approved
**Reasoning:** Every global constraint is met and the two self-flagged deviations both check out under independent verification (package `.d.ts` inspection, Rust crate permission files, param-name cross-check against `error.rs`) rather than resting on the report's say-so; the only findings are two low-impact polish items with no v1 behavioral effect.