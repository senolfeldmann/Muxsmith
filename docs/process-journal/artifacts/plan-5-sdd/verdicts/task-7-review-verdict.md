<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-7  (round 1 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01KCutA9uJptrXWExYKa55c9
  agent_id:           a9f8dab8becf1bfef
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a9f8dab8becf1bfef.jsonl
  dispatch_desc:      Review Task 7 (spec + quality)
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-10T14:13:28.407Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

- ✅ IPC surface matches the brief exactly: `validate_profile`, `dry_run`, `identify`, `detect_mkvmerge`, `get_settings`, `set_settings`, all registered in `invoke_handler` (`src-tauri/src/lib.rs:661-668`), all returning `Result<_, IpcError>`.
- ✅ `IpcError { code, params }` is prose-free; the only text passthrough is third-party `detail` (spec 8.4's accepted exception) — verified across every `From` impl in `error.rs`.
- ✅ `AppSettings` shape (`mkvmerge_path`, `default_jobs=1`, `locale`, `recent_profiles` MRU-cap-10, `dir_memory` keyed by profile path) matches D27's actual text (checked `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md:134-166`, which — despite living under a section titled "Frontend stack" — is the authoritative source for this exact settings shape); never touches the profile YAML anywhere in the diff.
- ✅ `detect_mkvmerge` → `MkvmergeInfo{path,version,meets_minimum}` via `Mkvmerge::detect` + `version_pair >= MIN_SUPPORTED`; `TooOld` carries distinct `found`/`minimum` params and a distinct code from `NotFound` (`error.rs:162-165`, tested).
- ✅ Judgment call 1 (`ShellRenderer` echoing the code key): checked spec 8.4 directly (`2026-07-08-muxsmith-v1-design.md:383-390`) — the frontend owns its own `@fluent/bundle` catalog and re-renders from `code`/`params`; the CLI/GUI "same structures" requirement is about shape, not content. Not misleading: `code`/`params` are already present verbatim on the same diagnostic object, so a consumer that mistakenly reads `rendered` gets the code string again, not garbage.
- ✅ Judgment call 2 (override consistency): every mkvmerge-touching command (`dry_run`, `identify`, `detect_mkvmerge`) uniformly resolves via `Mkvmerge::detect(mkvmerge_override)`; `validate_profile` correctly never touches mkvmerge. No path bypasses the override.
- ✅ Judgment call 5 (`Mkvmerge::detect`'s override branch bypassing the `Spawn`→`NotFound` remap): verified directly in `crates/muxsmith-core/src/capability/runtime.rs:101-122` — `detect()`'s override arm calls `enforce_floor` directly, never through `locate()`'s remap. The report's claim is a discovered fact, not an assumption.
- ✅ Judgment call 3 (async+`spawn_blocking` deviation for `detect_mkvmerge`): confirmed via Tauri v2 docs (context7) — "Commands without the `async` keyword run on the main thread unless explicitly defined with `#[tauri::command(async)]`." The deviation is real, correctly reasoned, and documented.
- ⚠️ "Long-running commands async via `spawn_blocking` (webview never blocks)": only partially true. See Important #2 below — `state.load_settings()?` (synchronous `fs::read_to_string`) runs directly in the async command body of `dry_run`/`identify`/`detect_mkvmerge`, outside `spawn_blocking`.
- ⚠️ Gate claim "56 muxsmith-gui tests incl. 4 gated-live against real mkvmerge": the diff itself contains exactly 3 gated-live call sites (`grep -c real_mkvmerge_available()` → 3, at lines 506/565/628), not 4. See Minor #4.
- ✅ ASCII punctuation: grepped the diff for em/en-dash, smart quotes, ellipsis — none found.

### Strengths

- Command bodies are genuinely Tauri-free (`&Path`/`Option<&Path>`/owned types only), independently unit-tested, satisfying the brief's testability requirement literally.
- `dry_run_body` is a faithful, line-for-line mirror of `crates/muxsmith-cli/src/commands/dry_run.rs`'s orchestration (compared directly): same load → config-validate → detect → `list_languages` → `plan_batch` → document sequence, same three fold branches. The one substitution (`detect()` for `locate()`) and the hardcoded `on_collision: None` (which safely falls back to `profile.output.on_collision` per `RunInputs`'s own doc, not a silent behavior loss) are both correctly scoped to this task.
- No stray `unwrap`/`expect`/`panic!` in production code — the only one is the pre-existing, documented Tauri-launch panic; every other occurrence is confined to `#[cfg(test)]`.
- Error-mapping design (`RuntimeError`/`IdentifyError`/`SettingsError` → `IpcError`) is clean and correctly reasons about the shared `mkvmerge-query-failed` code for both `NonZero` and `Parse`.

### Issues

#### Critical (Must Fix)
None found.

#### Important (Should Fix)

1. **Redundant mkvmerge subprocess spawn in `detect_mkvmerge_body`** (`lib.rs:537-545`). `Mkvmerge::detect()` already runs `enforce_floor`, which spawns `mkvmerge --version` once and validates it against `MIN_SUPPORTED` (`runtime.rs:183-193`, whose doc explicitly states "the version query is never run twice for the same candidate"). `detect_mkvmerge_body` then calls `mkv.version_pair()?` again, spawning `--version` a *second* time to recover the pair `enforce_floor` already computed and discarded. This doubles subprocess-spawn cost on a path the implementer's own doc says runs "on every GUI startup." **Plan-mandated**: the brief's interface sketch literally specifies `Mkvmerge::detect(...) + version_pair >= MIN_SUPPORTED`, so the implementer followed instructions; the real gap is a missing core API (`detect`/`enforce_floor` should surface the pair it already parsed). Worth a follow-up core change, not a rework of this task.

2. **Blocking file I/O runs directly in async command bodies, outside `spawn_blocking`.** `dry_run`, `identify`, and `detect_mkvmerge` (`lib.rs:574`, `592`, `619`) all call `state.load_settings()?` — a synchronous `fs::read_to_string` — before handing off to `spawn_blocking`. Tauri's own docs explicitly warn against exactly this pattern (splashscreen guide: avoid blocking calls in async command bodies, they can stall the runtime). The absolute risk here is small (a tiny local JSON file), but it directly contradicts this task's own stated rationale for using `spawn_blocking` at all, and is inconsistent with `get_settings`/`set_settings`'s explicit, documented choice to accept blocking I/O only because they're plain sync commands. Fix: fold the settings read into the same `spawn_blocking` closure, or document why it's deliberately excluded.

3. **`dry_run_body`'s "mkvmerge found but `list_languages` fails" branch (`Some(true)`, `lib.rs:181`, the documented "broken installation" case) has zero test coverage.** The exact fixture needed already exists and is used one branch over: `fake_mkvmerge(dir, "<valid version>")` answers `--version` successfully (passing `enforce_floor`) and fails every other invocation, which is precisely what's needed to hit `list_languages()` failure. No test in the diff asserts `doc["mkvmerge_found"] == true` anywhere. The report's claim of "20 unit/integration tests covering every command body's branches" is not accurate as a result — this is a real, cheaply closable gap, not a hypothetical one.

#### Minor (Nice to Have)

4. **Report's self-reported test counts don't match the diff.** Claims "4 tests gated on the real installed mkvmerge" (actual: 3, `grep -c real_mkvmerge_available()`); claims `settings.rs: 6 + error.rs: 8 + lib.rs: 20` = 34 tests, but actual `#[test]` counts are `6 + 7 + 15` = 28 (which *does* match the Gate section's separately-stated aggregate "28 new tests," so only the per-file breakdown is inflated). Doesn't change functional correctness, but directly ties into finding #3 — the coverage-completeness claim it makes turned out to be the specific one that's wrong.
5. `RuntimeError::NonZero`/`RuntimeError::Parse` (both → `mkvmerge-query-failed`) have no test coverage anywhere, direct or indirect. Lower priority than #3: these are largely unreachable through the currently-wired command bodies except via #3's own untested path, and even there `dry_run_body` discards the specific code by design.
6. `settings::save` (`settings.rs:1106-1115`) uses plain `fs::write`, not write-to-temp-then-rename; a crash mid-write can leave a torn `settings.json`, which `load()` would then report as `settings-parse-failed` with no visible in-task recovery path. This matches the codebase's existing precedent (`executor::joblog.rs` uses the identical non-atomic pattern), so it's inherited, not a new regression — but both are worth hardening together in a later pass.
7. `SettingsError::Parse` (`settings.rs`) is reused for both JSON deserialization failures and the essentially-unreachable `serde_json::to_vec_pretty` serialization failure in `save()`. Harmless naming overload given `AppSettings`'s plain-data shape can't realistically fail to serialize.

### Assessment

**Task quality:** Needs fixes

**Reasoning:** No correctness bugs and the core design decisions (override consistency, orchestration mirroring, async/`spawn_blocking` split, `ShellRenderer`'s no-prose contract) are sound and independently verified against the spec, core source, and Tauri docs. But a documented-as-significant branch (`dry_run_body`'s "found but broken" path) ships untested despite trivially available fixtures, contradicting the report's own coverage claim, and settings I/O partially bypasses the `spawn_blocking` discipline the rest of the task otherwise follows carefully.