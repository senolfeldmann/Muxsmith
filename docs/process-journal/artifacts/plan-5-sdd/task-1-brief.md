### Task 1: Raw output events (D24) + JobEvent serde golden test

**Files:**
- Modify: `crates/muxsmith-core/src/executor/job.rs` (line loop + `JobProgress`), `crates/muxsmith-core/src/executor/queue.rs` (`JobEvent` + worker `on_progress` mapping)
- Test: `crates/muxsmith-core/tests/executor_events.rs` (new)

**Interfaces:**
- Produces: `JobProgress::OutputLine(String)`; `JobEvent::Output { index: usize, line: String }` serializing as `{"event":"output","index":0,"line":"..."}`. Semantics: every line mkvmerge writes that is NOT a `#GUI#progress` tick is emitted verbatim (tags included); tagged warning/error lines ADDITIONALLY keep their existing tag-stripped `WarningLine`/`ErrorLine` emission. Consumed by T6 (persistence) and T8/T11 (live log).

- [ ] **Step 1: Failing golden test.** In `executor_events.rs`, one test asserting exact `serde_json::to_string` output for ALL `JobEvent` variants including the new one (the GUI now consumes the stream; the wire shape is contract):

```rust
let ev = JobEvent::Output { index, line: "#GUI#warning hello".into() };
assert_eq!(serde_json::to_string(&ev).unwrap(),
    r#"{"event":"output","index":0,"line":"#GUI#warning hello"}"#);
```

Plus `started`/`progress`/`warning`/`error`/`finished` (finished embeds the full `JobOutcome` object: `state`, `exit_code`, `warnings`, `errors`, `duration_ms`). Run: `cargo test -p muxsmith-core --test executor_events` - FAILS (variant missing).

- [ ] **Step 2: Failing behavior test** (same file, `FakeSpawner::script`): a script with a progress tick, a plain line, and a tagged warning yields `OutputLine` for the plain AND the tagged line (verbatim), `WarningLine` for the tagged one, NO `OutputLine` for the tick.
- [ ] **Step 3: Implement**: add the variants; in `run_job`'s line loop emit `OutputLine(line.clone())` for every non-tick line before the existing tag handling; map in the worker's `on_progress` closure (queue.rs:146-155) to `JobEvent::Output`.
- [ ] **Step 4: Full gate green.** Existing CLI milestone rendering must ignore `Output` events (check `MilestoneState::render` handles the new variant via a wildcard or explicit no-op arm).
- [ ] **Step 5: Commit** `feat(core): raw output-line job events (D24) + JobEvent wire golden test`

---

