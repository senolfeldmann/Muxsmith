<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-11  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01MfdpPV6Ewjer9JHyD7rN85
  agent_id:           a87fd421a659bffed
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a87fd421a659bffed.jsonl
  dispatch_desc:      Review Task 11 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T22:54:57.671Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance

- File scope: exactly one new file, `crates/muxsmith-cli/tests/run_live.rs` (234 lines), nothing else touched. Matches the "ONLY" constraint.
- Gated locate-or-skip: both tests call `have_mkvmerge()` (identical `mkvmerge --version` probe as `run_cli.rs`/`dry_run_cli.rs`) as the first statement, `eprintln!` + `return` on failure, before any `tempdir()`/`Command::cargo_bin` call. Confirmed clean skip.
- Case 1: builds two SRT-only source MKVs via real mkvmerge, minimal profile (subtitles rule), invokes `run --source ... --output ...`, asserts exit 0, both outputs exist, both `-J`-identify (`container.recognized == true`, `container.type == "Matroska"`), stdout contains `"2 ok, 0 warning, 0 failed, 0 cancelled"`. All present, matches brief.
- Case 2: rerun with `--on-collision skip`, asserts exit 1, outputs "untouched" via byte-content + backdated-mtime double check. Present, matches brief plus goes beyond it (byte-content check is an addition, not a deviation).
- Typography: `grep -nP '[^\x00-\x7F]' run_live.rs` -> no matches. ASCII-only confirmed (module doc uses `" -- "` ASCII double-hyphen, not em-dash).
- `env!("CARGO_BIN_EXE_muxsmith")` requirement: implementer used `Command::cargo_bin("muxsmith")` via `assert_cmd::cargo::CommandCargoExt` instead of the literal `env!(...)` macro. ⚠️ Technically not the literal mechanism named in the brief/global constraints, but it is the exact same helper `run_cli.rs` (Task 8/9, already-reviewed prior task) uses for the identical purpose (locate and run the actual binary) - functionally equivalent and consistent with established project convention. Not flagging as a deviation given the precedent, but noting the letter-vs-spirit gap since the brief named the macro specifically.

### Named-risk findings

1. **Non-vacuity mechanism** - verified in diff, lines 215-249: `content1`/`content2` read via `fs::read` before `backdate_mtime`; `backdate_mtime` (154-164) sets mtime via `File::set_modified`, then re-reads via `fs::metadata(path).unwrap().modified().unwrap()` and returns that as the reference (`stale1`/`stale2`). Post-rerun assertions compare against `stale1`/`stale2` (the re-read value), not against a freshly-computed `SystemTime::now() - 1h`. Content comparison (`assert_eq!(fs::read(&out1).unwrap(), content1, ...)`) is present alongside the mtime comparison, independently. Matches the report's claim exactly.
2. **Skip-path correctness** - both tests: `have_mkvmerge()` check is the literal first line of the test body, `eprintln!` + `return` before any `tempfile::tempdir()`, `Command::cargo_bin`, or other assert_cmd/Command construction. Clean skip confirmed for both.
3. **Summary-line assertion** - checked `locales/en/cli.ftl:24`: `run-summary = { $ok } ok, { $warning } warning, { $failed } failed, { $cancelled } cancelled`. Test's literal `"2 ok, 0 warning, 0 failed, 0 cancelled"` is the exact rendered i18n string for 2 successful jobs, not a brittle guess. Correct.
4. **Case 2 exit-code semantics** - traced `crates/muxsmith-core/src/planner.rs:916-955` (`detect_output_collisions`): a pre-existing on-disk collision under `CollisionPolicy::Skip` gets `Severity::Warning` and `f.plan = None` explicitly nulled (both files hit this, since both outputs already exist on disk from the setup run). Traced `crates/muxsmith-cli/src/commands/run.rs:145-170`: `specs` is built by filter-mapping only files with `Some(plan)`, so both files drop out, `specs.is_empty()` is true, and the function returns `diag_exit_code(&config_diags, &batch)` **before** `run_queue` is ever spawned - architecturally guaranteed no job starts, not just empirically observed. `diag_exit_code` (`commands/mod.rs:33-39`) folds `all_diags` (which chains per-file diagnostics, confirmed at `mod.rs:19-27`) to `Severity::Warning -> 1`. Test asserts `second.status.code() == Some(1)` - correct value, correctly reasoned. "No job started" is pinned via `!stdout.contains(" ok, ")`, which is sound given the code path returns before any job/summary printing is reachable.

### Strengths

- All four named-risk claims in the report verified true against actual source, not just plausible-sounding.
- Test fixtures, CLI invocation shape (`args(["run"]).arg(&profile).args(["--source"])...`), `muxsmith()`/`have_mkvmerge()` helper duplication, and the `output.filename` keep-mode comment are all byte-for-byte consistent with `run_cli.rs`'s established style - confirmed the project already duplicates these helpers per test file (`cli_validate.rs`, `dry_run_cli.rs`, `run_cli.rs` each redefine them; no `tests/common/mod.rs` exists), so this is convention-following, not new duplication debt.
- `--on-collision skip` value mapping traced through `cli.rs`'s `CollisionArg` (clap kebab-case default) to `CollisionPolicy::Skip` - correct flag/value.
- Meta-test methodology described in the report (same-bytes rewrite probe catching on the mtime assertion specifically) is a genuinely strong verification method for a "did the negative assertion actually assert" concern; test structure in the diff is consistent with that having been done (mtime check is the last, independent assertion after content check).
- Hermetic: each test owns an independent `tempfile::tempdir()`; no shared mutable state, no `PATH` mutation, no cross-test ordering dependency.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)

- `crates/muxsmith-cli/tests/run_live.rs:230-233`: the "no job started" check only asserts `!stdout.contains(" ok, ")`. `run_cli.rs:28-33` has a slightly stronger existing helper `asserts_no_job_ran` that also checks `!stdout.contains("... start")`. Not reusable directly (each `tests/*.rs` file is its own crate, no shared `common` module in this project), and functionally the `run.rs:155` early-return makes a stray "start" line unreachable anyway - so this is defense-in-depth the test forgoes, not a real gap today. Would only matter if a future refactor moved the specs-empty check after some interim print.
- `build_source_mkv`'s `name` parameter does double duty as both the `.srt` sidecar stem (`"{name}.srt"` -> e.g. `Show.S01E01.mkv.srt`) and the full output filename (`dir.join(name)`). Functions correctly (file_stem strips only the trailing `.mkv`), but the double-`.mkv` intermediate artifact name is a bit confusing on a re-read; a plain episode-stem parameter with `.mkv`/`.srt` extensions appended inside the helper would read cleaner.
- `env!("CARGO_BIN_EXE_muxsmith")` named explicitly in the brief/global constraints vs. the `assert_cmd::cargo::CommandCargoExt::cargo_bin` helper actually used (same as prior tasks' precedent) - worth a one-line note in the commit/report acknowledging the substitution, since the brief named a specific mechanism.

### Assessment

**Task quality:** Approved
**Reasoning:** Every spec-mandated case and every named risk was traced to source and confirmed correct (collision severity, exit-code fold, i18n summary string, non-vacuous backdate ordering); no critical or important issues found, only a couple of nice-to-have polish items.