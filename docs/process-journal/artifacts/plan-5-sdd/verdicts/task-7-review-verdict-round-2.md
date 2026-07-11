<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 2 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01KCutA9uJptrXWExYKa55c9
  agent_id:           a9f8dab8becf1bfef
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a9f8dab8becf1bfef.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-10T14:29:51.055Z
  continuation_trigger: The coordinator sent a message while you were working: Re-review request for your Task 7 f
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance (fix round)

- ✅ **Finding #1 closed at the root cause.** `Mkvmerge` gains a private `cached_version_pair`; `enforce_floor` is the only writer and sets it strictly on its `Ok` path (`crates/muxsmith-core/src/capability/runtime.rs:219`): `m.version()?` and `parse_version_pair(&raw)?` propagate errors before any cache write, and `TooOld` returns `Err` carrying no handle at all — so no error path can ever produce a cached handle, and no stale/wrong pair can exist. `at()` (`runtime.rs:84`) and `locate()` (`runtime.rs:97`) construct with `None`, so their per-call spawn behavior is byte-for-byte unchanged, and the core test proves it (`at()` handle bumps the counter 1→2). Grep confirms the only production `version_pair()` call site is `detect_mkvmerge_body` (`src-tauri/src/lib.rs:256`) — the intended consumer; the CLI uses `locate()` handles exclusively, so zero behavior drift there. Snapshot semantics ("re-probe via `at(path)`") documented on the field, `version_pair`, and `enforce_floor`.
- ✅ **Finding #2 closed on all three commands.** `dry_run`, `identify`, `detect_mkvmerge` each clone `settings_path` and call `load_settings_from(settings_path.as_deref())` *inside* the `spawn_blocking` closure (diff lines 433-436, 460-462, 489-491); no filesystem I/O remains on the async command path. `AppState::load_settings` delegates to the new free helper, so there is one settings-read code path. The `Result` plumbing is correct: `dry_run`'s closure now returns `Result<Value, IpcError>` and the trailing `?` flattens the `JoinError`-mapped outer layer, matching `identify`/`detect_mkvmerge`'s existing shape — observable error codes unchanged. `get_settings`/`set_settings` deliberately stay sync, consistent with the original documented rationale.
- ✅ **Finding #3 closed with the right assertions.** `dry_run_body_query_failure_after_successful_detect_sets_mkvmerge_found_true` uses the existing `fake_mkvmerge` with a floor-clearing version (v123.4.5 ≥ 86.0, so `detect` succeeds; every non-`--version` invocation exits 1, so `list_languages` fails) — precisely the branch at `lib.rs:181`. It asserts `mkvmerge_found == true` plus the empty config-only document shape, i.e. the facts that distinguish this branch from both the `Some(false)` and load-failure branches.
- ✅ Minor #5 also closed: three direct mapping tests for `NonZero{Some(2)}` (detail + `code: "2"`), `NonZero{None}` (`code: "signal"`), and `Parse` — every `RuntimeError` arm now has direct coverage.
- ✅ Report counts now reconcile with the diff: 5 new GUI tests (2 in `lib.rs`, 3 in `error.rs`) on top of the actual prior 28 = 33; 1 new core test = 7 in `mkvmerge_runtime.rs`. The correction note honestly owns the earlier inflation.
- ✅ Known residual (`detect(None)` PATH rung double-spawns via `locate()` + `enforce_floor`) is real, correctly scoped as pre-existing, controller-accepted for whole-branch review; `locate()` untouched per instruction.

### Strengths

- The spawn-count test methodology is the right instrument: a counter-file fake proves the *absence* of a subprocess spawn, which no assertion on return values could; the claimed RED (2) → GREEN (1) is exactly what the pre-fix code would produce, so the claim is internally consistent with the first diff.
- Two-layer pinning: the core test pins the caching contract, the shell test (`detect_mkvmerge_body_spawns_version_exactly_once`) pins the command-body composition that actually regressed — the second is not redundant with the first.
- Cache-by-construction design (set only where the parse already happened, immutable thereafter) needs no invalidation logic and cannot go stale within a handle's lifetime; the one genuine caveat (binary replaced on disk mid-session) is documented with its escape hatch.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. All three prior Importants verified closed against the diff, not the report.

#### Minor (Nice to Have)

1. `counting_fake_mkvmerge` + `spawn_count` are now duplicated verbatim between `crates/muxsmith-core/tests/mkvmerge_runtime.rs` and `src-tauri/src/lib.rs` tests — a second instance of the already-accepted `fake_mkvmerge` mirror convention. At three mirrored helpers, a shared test-support crate starts to earn itself; one for the whole-branch review's consideration, not this task.
2. Asymmetry: `load_settings_from` exists as a free helper but `save_settings` still inlines its own path resolution. Currently harmless (`set_settings` is sync, no closure needs it); worth folding the day a save ever moves into a blocking task.

### Assessment

**Task quality:** Approved

**Reasoning:** All three Importants are verifiably closed in the diff itself — the cache is provably set only after a successful floor check with `at()`/`locate()` behavior unchanged and no other `version_pair` consumers to drift, the settings read sits inside the `spawn_blocking` closure on all three commands with an unchanged error contract, and the new tests assert the load-bearing facts (spawn counts, `mkvmerge_found: true`, all `RuntimeError` mapping arms). Remaining items are pre-existing, controller-accepted, or cosmetic.