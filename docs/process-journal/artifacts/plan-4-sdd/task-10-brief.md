### Task 10: SIGINT - ctrlc, kill in-flight, exit 130

**Files:**
- Modify: `crates/muxsmith-cli/Cargo.toml` (add `ctrlc = "3"`), `commands/run.rs` (install handler, wire the cancel flag, exit 130)
- Test: unit-level only (the cancel path through the queue is already covered by T3's tests; a real SIGINT e2e is not cheaply automatable - note this in the report)

- [ ] **Step 1:** In `run.rs` before `run_queue`: `let cancel = Arc::new(AtomicBool::new(false));` + 

```rust
    // Single-level SIGINT (D16): first Ctrl-C requests graceful cancel
    // (queue kills in-flight, partials deleted, summary printed, exit 130);
    // a second Ctrl-C during cleanup force-exits immediately.
    let handler_cancel = Arc::clone(&cancel);
    let _ = ctrlc::set_handler(move || {
        if handler_cancel.swap(true, Ordering::SeqCst) {
            std::process::exit(130);
        }
    });
```

After the queue returns: `if cancel.load(Ordering::SeqCst) { /* summary already printed */ return 130; }`.
- [ ] **Step 2:** `cargo deny check` green with the new dep (`ctrlc` is MIT/Apache-2.0; if a transitive dep trips a license not yet allowed, add THAT license individually with a comment - never blanket).
- [ ] **Step 3:** Full gate. **Step 4: Commit** - `feat(cli): SIGINT cancels the batch via ctrlc, exit 130 (D16)`

---

