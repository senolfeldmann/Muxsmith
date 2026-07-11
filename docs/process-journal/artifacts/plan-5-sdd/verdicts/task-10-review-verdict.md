<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-10  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01ThJ59N1o8wkSP7k9X9DAD3
  agent_id:           a89c4c25db8bd12b1
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a89c4c25db8bd12b1.jsonl
  dispatch_desc:      Review Task 10 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T16:15:33.526Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Profile picker (dialog plugin, `.yaml`/`.yml` filter) + MRU recents list; pick → `validate_profile` → diagnostics rendered via Fluent with severity + text and a `role="status"` summary line (`src/views/BatchView.vue` lines ~603-629, 862-877).
- ✅ Source/output pickers prefilled from `dir_memory[profile]` on selection and persisted back on both picker-selection and blur-commit `@change` (`selectProfile`, `persistDir`, `onDirInputChange`).
- ✅ Dry-run trigger, disabled while any IPC call is in flight, `aria-busy` during (`batch-dry-run` button).
- ✅ `batch_document` render: per-file `<table>` with `<caption>`/`<th scope="col">`, rule → resolved track rows, config+batch+per-file diagnostics, suggestions as cards with YAML fragment + copy-only button. D22 confirmed clean: no write/save call against any profile path anywhere in the diff; `Suggestion.edit` is typed `unknown` and never dereferenced.
- ✅ Run button emits `start-run` with `RunRequest`, disabled with a Fluent tooltip on no-report/errors/mkvmerge-missing.
- ✅ App.vue wiring exactly as specified: `pendingRun` ref, `onStartRun` switches to `jobs`, `:pending-run`/`@consumed` wired on `JobsView`, cleared on consume. `JobsView.vue` confirmed byte-for-byte untouched.
- ✅ ipc.ts param-name claim independently verified: no `#[tauri::command(rename_all = ...)]` anywhere in `lib.rs`/`run.rs`; `get_job_log(run_id, index)` (run.rs:398) is invoked as `invoke("get_job_log", { runId, index })` (ipc.ts:282), relying on Tauri's default conversion, exactly as claimed.
- ✅ FTL claim independently re-verified (not just trusted): parsed `gui-common.ftl` + `gui-batch.ftl` + `diagnostics.ftl` with `@fluent/bundle` directly and formatted all 35 `gui-batch.ftl` messages with representative args — zero parse/format errors. Cross-checked every `$t(...)` key referenced across the four new/changed `.vue` files against the catalog: no missing keys, `browse-button(-tooltip)` and `severity-*` correctly reused rather than duplicated.
- ✅ `batch_document` field-shape claim verified against source: `PlanAssignment`/`FilePlan`/`Suggestion` in `ipc.ts` match `planner::Assignment`/`Plan`/`Suggestion` (`crates/muxsmith-core/src/planner.rs:42-215`) field-for-field, correctly narrowed to only what the view renders.
- ✅ Zero new deps confirmed: `@tauri-apps/plugin-clipboard-manager`/`plugin-dialog` already in `package.json`/`Cargo.toml`, plugins already registered in `lib.rs`, capabilities (`dialog:allow-open`, `clipboard-manager:allow-write-text`) already granted — none of this touched by the diff.
- ✅ ESLint `no-raw-text` workaround claim verified legitimate: the rule (`eslint.config.js:54-63`) and its `title`/`aria-label`/`placeholder`/`alt` attribute coverage are real; both cited workarounds (composing "severity: message" as one Fluent call instead of concatenation, moving the resolved-track ternary into a `<script setup>` function) genuinely relocate non-prose data (a CLI-parity ASCII placeholder, passthrough mkvmerge vocabulary) rather than smuggling real user-facing copy past the lint.
- ⚠️ No live GUI smoke run (acknowledged carry-forward, T12's job per the brief) — cannot independently confirm the dialog/clipboard round-trip actually renders correctly in a live webview, only that types/wiring/catalogs are consistent.

### Strengths
- Every outside-diff risk named for scrutiny came back clean on independent re-verification rather than just trusting the report (Rust command signatures, `report/json.rs` shapes, FTL parse/render, catalog glob loader, eslint rule existence, plugin/capability wiring).
- `dir_memory` writes correctly spread the existing per-profile object and the outer map, touching only `dir_memory[profile][kind]`, never another profile's entry or `mkvmerge_path`/`default_jobs`/`locale` (`updateSettings`/`persistDir`).
- Blur-commit-before-switch sequencing for the directory inputs is correct: a native `blur`/`change` fires (and is handled) before a recents-button click's own handler runs, so switching profiles reliably persists the outgoing profile's typed value before loading the new one's `dir_memory`.
- Run-gate logic (`runDisabledReason`) correctly threads the `mkvmerge_found: Option<bool>` tri-state documented in `report/json.rs` (undefined on profile-load failure falls through to the `hasErrors` check instead of being misread as "mkvmerge missing").
- a11y is consistent with T9 precedent, not just internally self-consistent: `role="alert"` for IPC errors matches `SettingsDialog.vue:101`, `aria-busy` on in-flight actions matches `FirstRun.vue`, `data-testid` granularity meets-or-exceeds the existing baseline (`SettingsDialog.vue` has exactly one `data-testid`; this view has one per interactive control).

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
- **Client-side recents list is never capped, diverging from the server's own MRU contract.** `RECENT_PROFILES_CAP = 10` (`src-tauri/src/settings.rs:27`) is enforced only inside `save()` at write time (`settings.rs:139-140`); `get_settings`/`set_settings` themselves don't guarantee a capped return. `BatchView.vue`'s `updateSettings` helper does `const next = mutate(current); await setSettings(next); settings.value = next;` — it assigns the **pre-truncation** `next` object (with the full, uncapped `recent_profiles` array) directly to the reactive `settings.value`, which is what the "Recent profiles" list renders from. In a single running session, picking more than 10 distinct profiles grows the visibly rendered recents list past the intended MRU bound (it only self-corrects on the next `getSettings()`, e.g. after an app restart). This is a real, demonstrable divergence from the D27 MRU cap, in exactly the axis flagged for scrutiny ("MRU/dir_memory update correctness"). Fix: either truncate client-side to the same constant after every `updateSettings` write, or re-read `getSettings()` after `setSettings()` instead of trusting the local `next`.

#### Minor (Nice to Have)
- `resolvedTrackLabel` (`ResolutionTable.vue`) glues `track_id`/`track_kind` with hardcoded `" ("`/`")"` punctuation outside Fluent. The justification (CLI-parity, passthrough mkvmerge vocabulary) is reasonable and documented, but it is punctuation composed in app code rather than routed through a Fluent pattern; worth revisiting if Plan 6's editor grid needs locale-aware formatting here.
- Background settings writes (`rememberRecentProfile`, `persistDir`) fail silently to `console.warn` with no user-facing indication that a recents/dir-memory preference didn't persist. Explicitly justified as non-blocking bookkeeping in the code comments, and reasonable for this severity of failure, but a user gets no signal their directory memory silently didn't save.
- `JobsView.vue` (untouched, as required) has no `defineProps`/`defineEmits`, so `:pending-run`/`@consumed` currently fall through as an inert non-primitive attribute and a listener for an event nobody dispatches. Expected placeholder limbo pending T11, not a defect in this diff, but worth a note for whoever picks up T11 first.

### Assessment
**Task quality:** Approved
**Reasoning:** Every named outside-diff risk (ipc.ts param names, FTL parse/render + missing-key check, `report::json`/`planner` field-shape fidelity, D22 discipline, eslint workaround legitimacy) checked out on independent verification, and spec/a11y/App.vue-wiring compliance is complete. One real but narrow-blast-radius bug (uncapped client-side recents list, self-heals on restart) should be fixed but does not block merge-quality judgment of this task in isolation.