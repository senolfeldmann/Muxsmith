<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      whole-branch  (round 1 of 2)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_015wPAmACsHJKzjUUUkHYEFX
  agent_id:           a44acd0735a4896d9
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a44acd0735a4896d9.jsonl
  dispatch_desc:      Whole-branch review Plan 4
  agent_internal_round: 1 of 2
  final_message_ts:   2026-07-09T23:05:33.697Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

# Final whole-branch review: Plan 4 (7aec492..93d1a6b)

Gate independently re-verified on HEAD with real mkvmerge v100: **264 tests pass, 0 gated skips, `fmt --check` / `clippy -D warnings` / `deny check` all clean.**

### Strengths

- **The T1→T2→T3→T8 seam chain is genuinely clean.** Each layer consumes the previous one's interface verbatim; `RegisteringSpawner` (queue.rs:374-386) is the standout - it gets killers into the registry without widening `run_job`'s signature or adding a callback parameter. No contract drift, no redundant defensive re-checks at the boundaries.
- **`stop` vs `cancel` as separate flags.** Fail-fast never masquerades as cancellation, so the 130 override can only come from a real SIGINT. This is the kind of distinction that usually erodes across tasks; it held through T3, T8, and T10.
- **Deterministic concurrency tests.** `RendezvousSpawner` (Barrier), the condvar `Gate`, and `ScriptByIndexSpawner` (index keyed via argv, not call order) eliminate every sleep-based timing bet. The `recv_timeout` comment ("hang-to-failure converter, not a race window") shows the reasoning was deliberate.
- **The two fastfollow fixes came with symmetric regression tests in both `dry_run_cli.rs` and `run_cli.rs`, plus human-mode pin-down tests** so the fix provably didn't leak into the untouched path.
- **`run_live.rs`'s non-vacuous "untouched" assertion** (backdate mtime, re-read the stored value, byte-equality on top) is how this check should always be written and almost never is.
- **Plan defect caught and fixed in implementation:** the plan's pinned watcher shape (exit only on cancel) would deadlock `thread::scope` on every non-cancelled batch. The `done` flag fixes it and the watcher comment documents it. This is an issue with the **plan**, resolved correctly.
- Doc comments are truthful after the fixes (spawn-failure no-delete rationale, drain-before-wait mutex discipline, one-shot-sweep semantics), and Fluent keys have zero drift: all 8 new `run-*` keys used, params match the ftl exactly, `msg_with_count` is documented for exactly the mechanism (`FluentValue::Number` selector) that forced it.

### Triage of the 13 ledgered findings

1. **v1.x backlog** - the skip branch asserts severity; the default branch's exit-2 is adequate coverage for a policy default.
2. **v1.x backlog** - failure mode needs `remove_file` to fail on a just-closed file (permissions/AV); fix direction exists (push a line into `outcome.errors`, which already carries the spawn-error string).
3. **wontfix** - grammar caps at 100; milestones only compare `>=`; no observable harm. Clamp opportunistically if the parser is ever touched.
4. **v1.x backlog** - surfacing is indirectly covered (queue's `outcomes_index_aligned`, run.rs notice-line tests); one missing `exists()` assertion.
5. **v1.x backlog** - plan-mandated one-shot sweep, bounded to one job per worker, recorded honestly. Cheap narrowing available (see Recommendations).
6. **v1.x backlog** - a worker panic implies a bug in `run_job`, which is panic-free by construction today; acceptable for v1.
7. **wontfix** - test-helper-only fragility.
8. **v1.x backlog** - add the JobEvent serde golden test when Plan 5's consumer lands; that is when the shape becomes load-bearing.
9. **wontfix** - the renderer unit tests assert rendered fragments, so a missing `run-*` key fails as a raw-id render; that guard is adequate.
10. **fix before push** - escalated: the fastfollow widened this from "imprecise" to "false" (see Important 2).
11. **wontfix** - fmt accepted it; zero value in a dedicated change.
12. **v1.x backlog** - Cargo.lock pins 3.5.2; the sibling convention is style, fold in whenever Cargo.toml is next touched.
13. **wontfix** - cosmetic; the existing assertions are sufficient.

### Issues

#### Critical (Must Fix)

None.

#### Important (Should Fix)

1. **Windows: a killed job maps to `Warning` and keeps the partial - D17 breaks on the platform D16 was explicitly designed for.**
   `/home/senol/Git/Muxsmith/crates/muxsmith-core/src/executor/spawn.rs:98` (`LiveJob::wait`) + `/home/senol/Git/Muxsmith/crates/muxsmith-core/src/executor/job.rs:126-131`. `std::process::Child::kill` on Windows is `TerminateProcess(handle, 1)`, and Windows `ExitStatus::code()` is always `Some`. So Killer-killed job → `wait()` = `Some(1)` → `JobState::Warning` → **partial output kept**, `Cancelled` never reached. That is precisely the `on_collision: skip` silent-corruption footgun D17 names as its sharpest argument. The `RunningJob::wait` doc contract ("`None` when killed") - which `FakeJob` encodes and every T2/T3 test builds on - is only satisfied by the live impl on Unix. **Fix:** share an `Arc<AtomicBool>` killed flag between `LiveJob` and its Killer (killer sets it before `kill()`); `wait()` returns `None` when set - exactly `FakeJob`'s existing mechanism, unit-testable cross-platform. Not blocking this Linux-only push; must land before the 3-OS matrix / go-public, since Windows cancellation is an explicit Plan-4 deliverable (D16).

2. **`mkvmerge_found: false` is now asserted on a path that never checked mkvmerge.**
   `/home/senol/Git/Muxsmith/crates/muxsmith-cli/src/commands/dry_run.rs:48` and `/home/senol/Git/Muxsmith/crates/muxsmith-cli/src/commands/run.rs:71`: the fastfollow reuses `config_only_json` for profile-load failure, where the mkvmerge lookup never ran - the document now states a fact that was never established (a JSON consumer distinguishing "install MKVToolNix" from "fix your profile path" is actively misled). Three paths emit `false`; only `locate()` failure is accurate (ledger #10 covers the found-but-broken middle case). **Fix before push** (branch is still local, shape not yet externally consumed, gets strictly more expensive later): emit the field only on the locate-failure path, or replace with an honest tri-state / `planning_ran: false`. No existing test asserts the field on the load-failure path, so the fix is friction-free.

#### Minor (Nice to Have)

- `/home/senol/Git/Muxsmith/crates/muxsmith-cli/src/cli.rs:12-13` - the `Cli::command` doc still says "0 clean / 1 warnings / 2 errors (spec 8.1)"; T10 added 130. Cross-task doc drift; one line.
- `/home/senol/Git/Muxsmith/crates/muxsmith-core/src/executor/queue.rs:87` - `workers = opts.jobs.max(1)` is not capped by `specs.len()`; `--jobs 100000` over 2 files spawns 100000 threads (thread-spawn failure panics inside the scope). `.min(specs.len().max(1))`.
- `/home/senol/Git/Muxsmith/crates/muxsmith-core/src/executor/spawn.rs:94` - `read_line` `Err` (non-UTF-8, e.g. a non-UTF-8 filename echoed in a warning) is treated as EOF, silently ending the event stream mid-job. Outcome state stays correct via exit code, but remaining warnings/progress are lost. `read_until` + `from_utf8_lossy` is the robust form.
- Empty-batch asymmetry: `run --json` prints a zeroed `summary`; human mode on a clean empty run (exit 0) prints nothing at all - a silent success. **Plan/spec-level gap** (D15 does not specify empty-batch human output), not an implementation error; a "0 ok, ..." or nothing-to-do line is worth deciding in v1.x.
- CLI test-helper duplication: `have_mkvmerge`/`muxsmith()`/`fake_mkvmerge_that_fails_queries` now exist 2-3x across `dry_run_cli.rs`/`run_cli.rs`/`run_live.rs` - the exact pattern T5 consolidated in core. Documented as deliberate; apply the T5 principle to the CLI crate when the next test file appears.
- API-surface inconsistency: `identify`'s fake is test-local (`tests/support`), while `FakeSpawner` **and** `ConcurrencyTracker` are public lib API. `FakeSpawner` is plan-mandated and defensible (Plan 5 shell tests); `ConcurrencyTracker` is pure test instrumentation in the public surface - `#[doc(hidden)]` or relocation before go-public.
- `jobs[].index` in the JSON document indexes the queue, not `files`; correlating a job to its file requires joining on the output path. One sentence in `run_json_document`'s doc (or a `source` field later) prevents consumer misreads in Plan 5.

### Recommendations

1. Fold the **Important 2** fix into the branch before pushing (15 minutes, no test churn).
2. Land **Important 1** as the first fast-follow, with a unit test asserting killer-then-`wait()` returns `None` on the live impl; hard gate before the full OS matrix runs.
3. Cheap narrowing of ledger #5: have `run_job` check `cancel` immediately before `spawn` and return `Cancelled` without spawning - closes most of the register-after-sweep window without touching the one-shot watcher design.
4. Plan 5 opening move: JobEvent serde golden test (#8) the moment the Tauri shell consumes the stream.
5. After push, confirm in the Actions log that the gated tests ran rather than skipped (T6's verification, already in the plan close-out).

### Assessment

**Ready to merge/push?** Yes - with the Important 2 one-liner recommended pre-push while the branch is still local.

**Reasoning:** The full gate is independently green with the gated tier running live, the D15 exit contract and D17 delete-partial semantics verify end-to-end across human/json/error/SIGINT paths on the platforms this push exercises, and the cross-task seams show no drift - the implementation even caught and fixed a latent deadlock in the plan's pinned watcher shape. The two Important findings are narrow (a Windows-only kill-mapping defect on a path CI does not yet exercise, and a semantically false JSON field on a freshly added error path); neither undermines the branch on Linux/macOS, but Important 2 is cheapest to fix now and Important 1 must precede any Windows claim.