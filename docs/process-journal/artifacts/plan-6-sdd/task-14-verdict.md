# Verdict (extracted from the reviewer transcript)

Diff fidelity confirmed: the review-package diff was generated with wider context lines (e.g. `@@ -41,21 @@` vs default `@@ -48,7 @@`); the actual changed `+`/`-` content is byte-identical to `git diff a2252bf..bf46932`, and I read the load-bearing files directly via `git show bf46932:` regardless. All verification complete.

---

### Spec Compliance

- ✅ **SuggestionCard is pure render+emit.** Committed file imports only `ref`, `writeText` (clipboard), and `type Suggestion`. No `loadProfile`/`applySuggestion`/`saveProfile`/`IpcError`. `requestApply()` only `emit("apply", { config_path, edit })`; both fields forwarded verbatim, neither parsed.
- ✅ **BatchView owns the round trip with the right args.** `loadProfile(selectedProfile.value)` and `saveProfile(selectedProfile.value, updated)` both use the profile FILE path; `applySuggestion(doc.profile, payload.config_path, payload.edit as StructuredEdit)` uses `config_path` ONLY as the locator arg.
- ✅ **Busy state follows the file idiom.** `applyingIndex = ref<number|null>` folded into `busy` computed alongside `validating`/`dryRunning`; per-card `aria-busy` via `:applying="applyingIndex === i"`; try/catch/finally mirrors `runDryRun`.
- ✅ **Error path + clear.** catch sets `ipcErrorCode`/`ipcErrorParams` (the existing shared alert at `:373-378`); `finally` clears `applyingIndex`. Prior error cleared at attempt start.
- ✅ **`config_path` is an opaque locator, never a path** end-to-end.
- ✅ **Post-apply does nothing beyond dropping busy** — no auto-validate/dry-run call after `saveProfile`.
- ✅ **DiagnosticsPanel documentation-comment-only** — only a comment block added; `defineProps` unchanged; no template/prop/emit change.
- ✅ **Two bilingual keys in `gui-batch.ftl`.** en: `Apply` / `Apply this fix to the profile and save it.`; de: `Anwenden` / `Diese Korrektur auf das Profil anwenden und speichern.`
- ✅ **D22 comment updated** in SuggestionCard per brief (records apply now reads `edit`; D41 supersedes the dead comment-preservation premise; pairing survives on model-ownership).
- ✅ **No-fix case buttonless with paired control** (present/absent on the identical `name("batch-suggestion-apply")` selector).
- ✅ **e2e: two distinct fixture values, bidirectional echo assertions** against `recorded` IPC calls.
- ✅ **`gui-editor.ftl` 45/45 untouched** (both locales); commit touches exactly the amended Files list, no `EditorView.vue`/`App.vue`/`mocks.ts`.

### Adjudications

**Q1 — Wiring correctness: PASS (all four).** (a) SuggestionCard confirmed IPC-free, pure render+emit. (b) `selectedProfile` drives both load and save; `config_path` reaches only `applySuggestion`'s locator slot. (c) `applyingIndex` folded into `busy`, matching the established try/catch/finally idiom. (d) failure surfaces through the existing `ipcErrorCode` alert and `finally` clears the busy state. The defect class that stopped this task twice is closed.

**Q2 — Hardened echo assertions: PASS, genuine and bidirectional.** The mock records the actual args the frontend passes (`__muxsmithRecordInvoke__`), independent of the scripted return, so the assertions test the frontend's real wiring, not an echo. `PROFILE_PATH = "/profiles/demo.yaml"` and `SUGGESTION_CONFIG_PATH = "tracks[0].match"` are distinct. Traced swaps against the mock:
- load-by-locator → `loadCalls[0].args.path === "tracks[0].match"`, fails `.toBe(PROFILE_PATH)`.
- apply-by-path → `applyArgs.configPath === PROFILE_PATH`, fails both `.toBe(SUGGESTION_CONFIG_PATH)` and `.not.toBe(PROFILE_PATH)`.
- save-by-locator → fails `.toBe(PROFILE_PATH)` and `.not.toBe(SUGGESTION_CONFIG_PATH)`.
Additionally non-vacuous on the model itself: `applyArgs.profile.toEqual(loadedProfile)` and `saveArgs.profile.toEqual(appliedProfile)` with `loadedProfile !== appliedProfile`, proving the model flows load→apply→save. The paired negative renders real text (`suggestion-partition` [overflow, dropped=1] → "1 further resolution group was capped..."), and the positive `toBeVisible()` on the same selector prevents a typo'd negative passing vacuously.

**Q3 — Scope and comments: within grant.** BatchView's header-comment fix ("No profile mutation anywhere in this view (D22...)" → updated) is comment-only, on a file the amended Files list explicitly includes (Modify), and made factually false by this exact task's one mutation path — the same same-root-fact logic the brief mandates for the SuggestionCard D22 comment. Not creep. DiagnosticsPanel change is documentation-only and matches its stated out-of-chain role. `gui-editor.ftl` verified 45/45 untouched, both locales. Note: the en `gui-batch.ftl` header carried the stale "never applied" claim and was fixed; the de header is a generic translation note that never carried that claim, so no parallel fix was owed there — no inconsistency gap.

### Strengths

- The refit genuinely closes the locator-as-path defect at the architectural level (card can't misuse a path it never receives), not just at the test level.
- Test hardening is exemplary: distinct values, explicit `.not.toBe(...)` in both directions, non-vacuous model-threading checks, and a robust `aria-busy` drop as the wait instead of racing the recorded-call assertions.
- Comments are accurate and carry the D41/D22 supersession reasoning correctly.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **Silent no-op if the backend contract is violated.** In the ParseError branch, if `doc.config_diagnostics[0]` is undefined (profile null with no folded diagnostic — a `load_profile` contract violation per D42), the handler returns with no error surfaced (busy still cleared). Defensive and narrow; acceptable as-is, but a fallback code would make the invariant breach visible rather than silent.
- **Non-clicked cards' apply buttons stay visually enabled during an in-flight apply** (only `applyingIndex === i` disables the clicked one). The `busy` guard in `onApplySuggestion` blocks the re-entry, so it is functionally safe and consistent with the existing Run-button idiom; purely a visual-affordance nuance.

### HARVEST
Already captured by the controller in amendment 4, verified present — not owed by this task (the `bf46932` commit is product/test/locale-only, touches no `docs/`):
- `docs/process-conventions.yaml:372` records the T14 topology-premise NEEDS_CONTEXT and the echo-mock semantics-blindness that hid the config_path-as-path misuse.
- The two-distinct-fixture-values echo-mock pattern and the present/absent paired-control (falsifiability occurrence 5) are both referenced from the test's own comments as established house conventions.

### Assessment
**Task quality:** Approved
**Reasoning:** Every binding point of the amended brief is met and independently verified against the committed files; the wiring defect that stopped this task twice is structurally closed and the hardened test would fail on any locator/path swap in either direction. Only cosmetic minors remain.
