<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-10  (round 1 of 1)
  session_uuid:       f6ee0efc-4c8f-4f64-9e20-94324fe759ca
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca.jsonl
  tool_use_id:        toolu_01AD4kPnhNXdTLrFvJng2XfB
  agent_id:           acfbbf1343143bd92
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/f6ee0efc-4c8f-4f64-9e20-94324fe759ca/subagents/agent-acfbbf1343143bd92.jsonl
  dispatch_desc:      Review Task 10 (spec + quality)
  agent_internal_round: 1 of 1
  final_message_ts:   2026-07-09T22:42:46.581Z
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance
- ✅ Spec compliant. `crates/muxsmith-cli/Cargo.toml:220` adds `ctrlc = "3"` and nothing else; `deny.toml` untouched (diff stat lists only `Cargo.lock`, `Cargo.toml`, `run.rs`); the handler code in `run.rs:189-197` is a verbatim copy of the plan-mandated block from `task-10-brief.md:9-19`; the post-queue `if cancel.load(...) { return 130; }` at `run.rs:239-241` is unchanged from the pre-existing (dead) code, now live; testing scope honored (no test files touched, matching the brief's unit-level-only / e2e-out-of-scope instruction).
- No ⚠️ items — every claim in the report was independently verifiable from the diff plus the surrounding file.

### Named-risk findings

**1. Handler placement and flag identity — verified, correct.**
`run.rs:187` creates `cancel`; the handler is installed at `run.rs:189-197`, cloning it into `handler_cancel` (`run.rs:192`), strictly before `std::thread::scope` at `run.rs:207` which is where `run_queue` actually starts (spawned at `run.rs:209`). Inside that scope, `queue_cancel = Arc::clone(&cancel)` (`run.rs:208`) is unchanged from Task 8 — same underlying `Arc<AtomicBool>` allocation as `handler_cancel`. The post-queue check at `run.rs:239` reads `cancel` itself, the un-cloned original binding. All three references (`handler_cancel`, `queue_cancel`, `cancel`) are clones of one Arc; no flag-identity fork.

**2. Double-Ctrl-C semantic and signal-safety — verified, correct.**
`AtomicBool::swap` returns the pre-swap value. First SIGINT: previous value `false` → `if false` → no exit, flag now `true` (graceful cancel armed). Second SIGINT: previous value `true` → `if true` → `std::process::exit(130)` (force-exit). Matches spec exactly.
On blocking/signal-safety: read the vendored `ctrlc-3.5.2` source (`~/.cargo/registry/src/.../ctrlc-3.5.2/src/platform/unix/mod.rs`, `src/lib.rs:129-146`). The raw OS handler (`extern "C" fn os_handler`) only calls `sem_post()` — async-signal-safe. `set_handler` spawns a dedicated `"ctrl-c"` background thread that blocks on the semaphore and invokes the user closure there, outside raw signal-handler context. So the atomic swap and `std::process::exit` in this closure run on an ordinary thread; there is no async-signal-safety concern to flag.

**3. `let _ = ctrlc::set_handler(...)` error swallowing — acceptable, documented consequence.**
Traced `ctrlc`'s `init_and_set_handler` (`src/lib.rs:112-126`): the two realistic failure modes are (a) `Error::MultipleHandlers` if `set_handler` is called twice in one process (guarded by an `INIT` atomic+lock), and (b) `Error::System` if the OS fails to spawn the `"ctrl-c"` thread (resource exhaustion). `set_handler` (not `try_set_handler`) passes `overwrite = true`, so a pre-existing custom SIGINT handler doesn't produce `EEXIST` either. The report's claim that (a) cannot occur here — `run()` is called exactly once per process, integration tests exercise the compiled binary as a subprocess rather than calling `run()` in-process — checks out from the code structure (`main.rs` calls `run()` once; CLI tests use `assert_cmd`). Consequence of a swallowed failure: SIGINT reverts to OS default disposition (immediate termination, no cleanup, no 130) instead of crashing or misbehaving — a benign degradation, and this is the brief's literal mandated code, not an implementer choice.

**4. `--json` mode print-before-130 ordering — verified, correct.**
`run.rs:226-237`: both branches (`println!("{}", render_summary(...))` for human mode, `println!("{}", run_json_document(...))` for `--json`) execute unconditionally before the cancel check at `run.rs:239`. Confirmed by reading the file directly (sanctioned out-of-diff check) since this code predates this diff (Task 8/9) and this task didn't touch it — matches the report's self-review claim.

### Strengths
- Handler code, wiring point, and post-queue check are exact matches to the brief's mandated text — no scope creep, no reinterpretation.
- `Cargo.lock`/license diligence in the report (`ctrlc`'s full cross-platform transitive tree checked against `deny.toml`'s existing `MIT`/`Apache-2.0`/`Unicode-3.0` allowlist) matches what's actually in the diff: no `deny.toml` edit, and every new lockfile entry (`nix`, `cfg_aliases`, `block2`, `dispatch2`, `objc2`, `objc2-encode`) is MIT or a multi-license resolvable to MIT/Apache-2.0.
- Stale comments correctly retired: the old "plain, never-flipped flag" doc-comment sentence (`run.rs:33-39` region) and the old "Task 10 wires SIGINT onto this" inline comment are both replaced with accurate D16 descriptions, keeping the function-level doc comment truthful rather than leaving it to rot.
- Typography check: no non-ASCII characters (no em-dashes, smart quotes) in either changed file.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. The one plan-mandated risk item (named risk 3, `let _ = ctrlc::set_handler(...)`) is literal brief text (`task-10-brief.md:14`), not an implementer deviation, and its failure consequence is benign as traced above — not classifying it as a defect.

#### Minor (Nice to Have)
- `crates/muxsmith-cli/Cargo.toml:220`: `ctrlc = "3"` pins only the major version, while every sibling dependency in the same file pins a full patch version (`clap = "4.6.1"`, `schemars = "1.2.1"`, `serde_json = "1.0.150"`, `sys-locale = "0.3.2"`, `unic-langid = "0.9.6"`). This is a direct copy of the brief's exact instruction (`task-10-brief.md:7`: "add `ctrlc = \"3\"`"), so not a defect to fix against the brief — but it's a local style inconsistency worth a one-line note if/when someone next touches this file, since the pattern elsewhere in the workspace is full pinning.

### Assessment
**Task quality:** Approved
**Reasoning:** Handler wiring, flag identity, double-Ctrl-C semantics, error-swallowing consequence, and json-mode print ordering all verified directly against the code and (for the signal-safety claim) the vendored `ctrlc` source, with no deviation from the brief and no scope creep into `deny.toml` or test files.