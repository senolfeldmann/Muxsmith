<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-12  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01JeqcWL5z5PYv9MyNd1JzA5
  agent_id:           a97d2e2389a779e21
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a97d2e2389a779e21.jsonl
  dispatch_desc:      Review Task 12 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T17:15:23.715Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

- ✅ **Scenario (a)** first-run gate: detect-fail -> guidance -> manual-path recovery. Verified against `FirstRun.vue`/`App.vue` line-by-line; two `detect_mkvmerge` calls and the `set_settings` persisted path are asserted from real recorded invocations, not a UI flag.
- ✅ **Scenario (b)** dry-run: resolution table (`<caption>`-derived accessible name), composed diagnostic line, suggestion card, clipboard copy. Verified `DiagnosticsPanel.vue`'s exact `$t("batch-diagnostic-line", {severity, message})` composition and `plugin-clipboard-manager`'s real `write_text(text)` invoke shape against `node_modules` source — both match the test's assertions exactly.
- ✅ **Scenario (c)** live run: scripted `job-event`s progress rows/log, per-row cancel invokes `cancel_job({index})`, `run-finished` announces via `aria-live="polite"` (`JobsView.vue:281-282`). `<progress>`'s `value` JS property is asserted mid-run, before `finished`, genuinely exercising the live path (`JobRow.vue`'s `progressValue` computed).
- ✅ Mock fidelity vs `crates/muxsmith-core/src/executor/queue.rs::JobEvent` and `src-tauri/src/run.rs`: independently re-verified every fixture shape (`JobEvent`'s four snake_case-tagged variants, `JobOutcome`, `RunFinishedEvent` = `RunDocument` + `joblog_status`, `StartedRun`) against the Rust source and `src/ipc.ts` — no drift anywhere I checked.
- ✅ Pre-registration listener ordering: `JobsView.vue`'s `ensureListeners()` is awaited before `startRun`, and `mockIPC`'s `shouldMockEvents` internals (`node_modules/@tauri-apps/api/mocks.js`) resolve listeners from a live `Map` at emit-time, not a snapshot — the mock's ordering guarantee matches the real bridge's for the tested paths.
- ✅ `@axe-core/playwright`, fail serious/critical, one scan per of the three view states.
- ✅ `check-i18n.mjs`: hard-fail on missing literal ids, exemption comments present for `diagnostics.ftl` and the four D31 `close-abort-*` ids, unused-keys warn-only.
- ✅ CI: three Linux-only steps (`if: runner.os == 'Linux'`), correct order (i18n gate -> browser install -> smoke), matches existing guard style; no new `uses:` so nothing needed SHA-pinning.
- ✅ Dev deps exact-pinned (`1.61.1`/`4.12.1`/`26.1.1`, no `^`/`~`).
- ✅ Locators `getByRole` first with `data-testid` fallback only where no distinct role exists (`view-batch`, `job-row`, `job-progress`, `live-log`, `jobs-run-summary`) — verified against the actual component markup, all testids exist and are used consistently.
- ✅ Locale pinned `en` at both the Playwright (`use.locale: "en-US"`) and app-settings (mocked `get_settings.locale: "en"`) layers.
- ⚠️ Gate "ran green" (cargo/pnpm suite) — not re-run per instruction; the one command I ran myself (`tsc --noEmit -p e2e/tsconfig.json`) is a narrow read-only check, not the gate.

### Strengths

- Mock/fixture fidelity is exceptional and independently reproducible, not just asserted: `@tauri-apps/plugin-os`'s `platform()` bypassing `invoke()` entirely (confirmed in shipped source, `dist-js/index.js:34-35`) is a genuinely subtle Tauri detail the mock correctly special-cases via `installMockIPC`'s direct `window.__TAURI_OS_PLUGIN_INTERNALS__` write.
- Every load-bearing assertion checks a **recorded IPC call**, not a UI echo: `cancelJob`'s `{index}` arg (`src/ipc.ts:274`), `setSettings`'s `{settings}` arg (`src/ipc.ts:238`), clipboard's exact `text` arg — all cross-checked against the real wrapper functions' actual invoke signatures.
- Scenario (c)'s Run-button enablement is driven by real `BatchView.vue` reactive state (`runDisabled` computed off an actual `validateProfile` response), not forced into an enabled state by the test.
- `vite preview --host 127.0.0.1` fix is a sound, portable determinism improvement with no CI downside; the `name()` exact-match helper derives its expected text from the live catalog rather than a hardcoded string, so it stays resilient to legitimate copy changes while correctly rejecting genuine substring ambiguity ("Run" vs "Dry run").

### Issues

#### Important (Should Fix)

- **The report's central drift-detection claim is not actually enforced anywhere in the gate.** `task-12-report.md` states the `satisfies JobEvent`/`RunFinishedEvent` fixture annotations mean "a future contract drift in `ipc.ts` would fail the smoke's own type-check, not just silently pass." This is false as wired: `e2e/tsconfig.json` exists but nothing invokes it. `pnpm test:e2e` (`package.json:11`) is `vite build --config e2e/vite.harness.config.ts && playwright test` — no `tsc` call. `pnpm build`'s `vue-tsc --noEmit` is scoped to `src/**` only (root `tsconfig.json:14`, excludes `e2e/`). `pnpm lint` uses `tseslint.configs.recommended` (`eslint.config.js:37`), the non-type-aware preset with no `parserOptions.project` — it cannot catch a `satisfies` violation. Playwright's own test transform is Babel-based strip-types (confirmed in `node_modules/.pnpm/playwright@1.61.1/.../lib/transform/babelBundle.js`), never a type checker. I confirmed this concretely by running `node_modules/.bin/tsc --noEmit -p e2e/tsconfig.json` myself: it exits 0 today (fixtures are in fact well-typed), which proves the check is trivially available and simply not wired anywhere. Fix is one line (add a `tsc --noEmit -p e2e/tsconfig.json` step to `test:e2e` or CI). Doesn't affect current correctness — I independently verified every fixture against the Rust sources by hand — but the specific safety mechanism the report claims protects "mock fidelity," the brief's own load-bearing property, is currently inert.

#### Minor (Nice to Have)

- `scripts/check-i18n.mjs`'s catalog parser (`MESSAGE_ID_RE = /^([A-Za-z][A-Za-z0-9_-]*)\s*=/`, line 1409) is a bare per-line regex with no handling for Fluent multiline values, attributes (`.attr =`), or terms (`-term =`). Current catalogs are simple one-liners (verified: none of `locales/en/*.ftl` use these constructs) so it works correctly today, but unlike `run.rs::ftl_message()` — which documents this exact constraint explicitly and pins it with a test (`close_abort_strings_resolve_from_the_ftl_catalog`) — the script's header comment doesn't flag the limitation, so a future catalog entry using an attribute or continuation line would silently mis-parse with nothing to catch the drift.
- The 11 warn-only i18n false positives (shell `IpcError` codes reached only via `$t(err.code, ...)`) are outside the brief's literal exemption scope but harmless (warning-only, zero effect on exit code); the implementer's documented deferral is reasonable and fine to leave for follow-up.

### Assessment

**Task quality:** Needs fixes

**Reasoning:** Functionally the smoke suite is excellent — mock/fixture fidelity against the Rust IPC contract holds up under independent, line-by-line verification, and every meaningful assertion checks real recorded behavior rather than a mock echo. The one Important finding is narrow and cheap (wire the already-passing `e2e/tsconfig.json` type-check into an actual script/CI step) but it directly contradicts a explicit, load-bearing claim in the implementer's own report, so it should be closed before signing this off as fully matching what was reported.