<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-10  (round 2 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01ThJ59N1o8wkSP7k9X9DAD3
  agent_id:           a89c4c25db8bd12b1
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a89c4c25db8bd12b1.jsonl
  dispatch_desc:      Review Task 10 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-10T16:26:51.606Z
  continuation_trigger: The coordinator sent a message while you were working: Re-review request for your Task 10 
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance (re-review of 638eda2)
- ✅ **Recents cap fix is correct and complete.** `RECENT_PROFILES_CAP = 10` (BatchView.vue:75) matches the Rust constant (`src-tauri/src/settings.rs:29`, verified still `10`), with a symbol-reference comment tying them. The `.slice(0, RECENT_PROFILES_CAP)` sits **inside the mutation closure**, so `updateSettings`' `settings.value = next` now always assigns the capped list — the exact fix I asked for. Grep confirms `rememberRecentProfile` (BatchView.vue:81) is the **only** frontend site that grows `recent_profiles`; `persistDir` never touches it, and `SettingsDialog`'s save path (SettingsDialog.vue:39,73) only passes the re-fetched, already-persisted (therefore server-capped) list through unmodified. The load path (`onMounted` → `getSettings`) returns what `save()` truncated, so `settings.value` is capped on every path.
- ✅ **v-show conversion is complete.** Full read of App.vue: both views carry `v-show` (lines 99, 103), no lingering `v-if`/`v-else` on either. Both single-root SFCs (`<section>` roots, BatchView.vue:265 / JobsView.vue:9), so `v-show`'s inline `display:none` actually attaches — the silent multi-root failure mode does not apply.
- ✅ **First-run gate intact.** Both views live inside the `<template v-else>` of the `checking` / `blockedError` v-if/v-else-if chain (App.vue:49-60); pre-detection, neither view (nor nav) renders at all. The gate itself is untouched by the diff.
- ✅ **pending-run prop flow unbroken.** `:pending-run="pendingRun"` / `@consumed="pendingRun = null"` unchanged; `onStartRun` still stores-then-switches. Noted per the coordinator: with both views persistently mounted, the payload now arrives at JobsView as a **prop update on an already-mounted component**, not at mount — T11's side owns watching for that (its re-review covers it); not a T10 defect.
- ✅ `aria-current` on the nav buttons keyed on `activeView` (App.vue:68, 76) present and unaffected.

### Strengths
- The cap is applied at the single point that matters (inside the mutation, before both persist and local assignment), so persisted state and rendered state cannot diverge — cleaner than a post-hoc client truncate after the write.
- The App.vue comment (lines 91-97) states the actual rationale for `v-show` (JobsView's future onMounted-registered run listeners surviving tab switches mid-run) instead of a generic "performance" hand-wave, and explicitly delimits it against the first-run gate.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- **The v-show change removes an incidental settings refresh.** With `v-if`, BatchView remounted on every tab round-trip and its `onMounted` re-fetched settings; now `settings.value` refreshes only at app start and on this view's own writes. Consequence: `settings.value.default_jobs` (passed as `jobs` in the run payload) can be stale if the user changes default parallelism in SettingsDialog after the last profile pick and then clicks Run without re-picking. The staleness window existed before on the no-tab-switch path (SettingsDialog is a sibling; a tab switch was never a guaranteed refresh), so this is a widening of a pre-existing gap, not a new defect — but a `getSettings()` re-read in `emitStartRun`, or T11 resolving `jobs: null` against fresh settings, would close it cleanly.
- Cross-language constant duplication (`RECENT_PROFILES_CAP` in TS and Rust) can drift silently; the mirror-comment is the only guard. Acceptable at this scale — exposing the constant over IPC for one number would be unearned machinery — just worth knowing it exists.

### Assessment
**Task quality:** Approved
**Reasoning:** The Important finding is closed exactly as specified (cap inside the mutation, all growth paths covered, constants aligned), and the controller-assigned v-show conversion is complete with the first-run gate and pending-run wiring verified intact. The only residue is a minor, pre-existing settings-staleness window on `default_jobs`, best closed on the T11 side.