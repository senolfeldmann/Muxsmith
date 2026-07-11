<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-1  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01VyC5WkVuop6TF4ZBKxvjLk
  agent_id:           a5614aef8852f153e
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-a5614aef8852f153e.jsonl
  dispatch_desc:      Review Task 1 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T20:13:57.393Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

Both named checks clear: `Mkvmerge::locate() -> Result<Mkvmerge, RuntimeError>` (runtime.rs:48) and `path(&self) -> &Path` (runtime.rs:60) exist exactly as the gated test uses them, so the workspace compiles and the "77 passed" claim is credible; and `executor_live.rs`'s skip block mirrors `command_integration.rs:213-230` verbatim (identical `fn mkvmerge() -> Option<Mkvmerge>` helper, identical `let Some(m) = ... else { eprintln!("mkvmerge not found; skipping"); return; }`, identical SRT fixture).

### Spec Compliance
- ✅ Spec compliant. Every mandated interface matches the brief verbatim:
  - `SpawnError` derives + shape (spawn.rs:46-47), `Killer` alias (spawn.rs:52), `Spawn::spawn` signature (spawn.rs:59), `RunningJob: Send` + three method signatures (spawn.rs:63-69), `LiveSpawner { pub mkvmerge: PathBuf }` (spawn.rs:91-94), `FakeSpawner` fields (spawn.rs:146-150), `FakeSpawner::script`/`spawned` signatures (spawn.rs:155,164). Consumed types `executor::spawn::{Spawn, RunningJob, Killer, LiveSpawner, FakeSpawner, SpawnError}` are all exported.
  - `LiveSpawner::spawn` body matches the brief's mandated code exactly, including `.arg("--gui-mode").args(argv)` order and `stderr(Stdio::null())` (spawn.rs:98-103).
  - Both mandated fake tests present (spawn.rs:216-238). `fake_spawner_scripts_lines_and_exit` is semantically identical to the brief; only the final `assert_eq!` is line-wrapped by rustfmt (no logic change). `fake_killer_ends_stream_and_wait_returns_none` is byte-for-byte the brief text.
  - `pub mod executor;` added in the alphabetical slot (lib.rs:255); `executor/mod.rs` doc + `pub mod spawn;` verbatim (mod.rs:20-24).
  - gui-mode grammar in the doc comment (spawn.rs:75-90) encodes the report's OBSERVED v100 lines: progress `#GUI#progress NN%` final always `100%`, `#GUI#warning '<file>': <message>` (exit 1), `#GUI#error <message>` no leading filename (exit 2). Not assumed.
  - gated test asserts observed reality (executor_live.rs:311-320): `exit == Some(0)`, at least one `#GUI#progress ` line, and the *last* progress line (via `rfind`, correctly skipping trailing "Multiplexing took..." lines) equals `#GUI#progress 100%`.
  - `#![deny(missing_docs)]` satisfied: every pub item is documented, verified by the crate building under the existing deny (SpawnError's tuple field is exempt from missing_docs; LiveSpawner's named pub field is documented at spawn.rs:92).
  - Typography ASCII-only in the committed code (doc comments use `-`, `'`, `:`; no em-dash/smart-quote/ellipsis).
- ⚠️ Nothing left unverifiable from the diff.

### Strengths
- RED evidence is genuine, not merely literal: referencing an undeclared `FakeSpawner` from a compiled-in test produces `E0433 cannot find type` — the test provably cannot pass without the implementation. Stronger than the brief's literal "module absent" (which yields a false-green "0 tests matched"). The judgment call is sound.
- The Send/Sync split is correctly designed for the downstream multi-threaded queue: `Box<dyn RunningJob>` is `Send` (moves into one worker thread), `Killer = Arc<dyn Fn()+Send+Sync>` is shareable to a separate cancellation thread. Both concrete jobs are `Send` (FakeJob: Vec/usize/Option/Arc<AtomicBool>; LiveJob: ChildStdout/Arc<Mutex<Child>>).
- `next_line()` reads the piped stdout handle directly and does **not** touch the child mutex, so the blocking read never blocks the kill path — the phase you would actually cancel during leaves the lock free.
- FakeJob honors the pinned contract exactly: killed flag short-circuits `next_line` to `None` (spawn.rs:192) AND `wait` to `None` (spawn.rs:200-204); `spawn` records `argv.to_vec()` (spawn.rs:171). `killed: Arc<AtomicBool>` cloned into the killer closure (spawn.rs:206-208).
- gated grammar assertion uses `rfind` on progress lines rather than `lines.last()`, so trailing non-progress stdout ("The cue entries...", "Multiplexing took...") does not defeat the `100%` check. More correct than a naive last-line assertion.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. (The two items below are plan-mandated verbatim code; recording them as latent constraints for the queue task, not as defects that block this task.)

#### Minor (Nice to Have)
- **`LiveJob::wait()` holds the child mutex across the blocking `waitpid` (spawn.rs:128-135).** Answering the concurrency prompts directly: (a) *Can a blocking call hold the mutex while a killer tries to take it?* Yes — while a thread sits in `wait()` on a still-running process, a concurrent `Killer` blocks on `child.lock()` until the process exits on its own, defeating cancellation. (b) *Process closes stdout but keeps running?* `next_line()` returns `None` at EOF, the worker proceeds to `wait()`, which then blocks holding the lock; a kill during that window stalls. For mkvmerge the window is negligible because it emits its trailing lines ("Multiplexing took...") to stdout and closes stdout only at near-exit, so the realistic cancellation path (kill during the `next_line()` blocking read, where the lock is free) works. This is the brief's verbatim code, so it is not a defect to fix here — but it imposes a constraint the queue task must respect: **drain `next_line()` to EOF before calling `wait()`; never call `wait()` on a live process from one thread while relying on `Killer` from another.** Worth surfacing to the controller so Task 2/3 does not build a stall into the cancellation path.
- **(c) Kill is best-effort and idempotent as documented (spawn.rs:136-141) — confirmed.** The `Result` is discarded (`let _ =`), so best-effort holds; and because the killer shares the *same* `Child` behind the mutex, once `wait()` has reaped it, std's `Child::kill` sees `status.is_some()` and returns without signalling — so repeated kills and post-wait kills are safe no-ops with no PID-reuse hazard. This is exactly why `Arc<Mutex<Child>>` (not a raw pid) is the right shared handle. No action needed; noting because the prompt asked.
- **`read_line` error is treated as clean EOF (spawn.rs:123-124: `Ok(0) | Err(_) => None`).** A mid-stream read error (e.g. a non-UTF-8 byte in a warning line's filename) silently ends the line stream as if the process closed stdout, dropping subsequent lines and the real EOF. gui-mode progress/warning/error output is ASCII in the observed v100, so the practical risk is low, and the brief mandates this exact match arm. Latent robustness gap only; acceptable for this seam.
- Trait `Spawn` carries no `Send + Sync` supertrait (brief-conformant). Both concrete spawners are `Send + Sync` (LiveSpawner = PathBuf; FakeSpawner = Mutex), so the queue task can use `Arc<dyn Spawn + Send + Sync>` at the use site. No change here; flagging so the controller expects to add the marker bound downstream rather than in this crate.

### Assessment
**Task quality:** Approved
**Reasoning:** All mandated interfaces, both fake tests, the observed gui-mode grammar, and the gated live assertion match the brief verbatim and encode real v100 behavior; the RED evidence is genuine and the external API the gated test depends on is confirmed to exist. The only findings are plan-mandated latent concurrency/robustness gaps that do not compromise this task's contract but should travel to the queue task as usage constraints (drain stdout before `wait()`; add `Send + Sync` at the use site).