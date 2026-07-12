### Task 3: Robust event-stream reads - no truncation, no hang (#9)

**Files:**
- Modify: `crates/muxsmith-core/src/executor/spawn.rs:101-107` (`LiveJob::next_line`)
- Test: `crates/muxsmith-core/src/executor/spawn.rs` unit tests + a live gated test

**Interfaces:** `RunningJob::next_line(&mut self) -> Option<String>` signature unchanged; behavior change only.

Current verified code:

```rust
fn next_line(&mut self) -> Option<String> {
    let mut line = String::new();
    match self.reader.read_line(&mut line) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(line.trim_end().to_string()),
    }
}
```

- [ ] Step 1: Failing test: feed a reader whose stream contains `b"ok line\n\xFF\xFE broken\nafter\n"` through a `LiveJob`-shaped harness (extract the read logic into a testable `fn read_next_line<R: BufRead>(r: &mut R) -> Option<String>` if needed - keep `next_line` a thin delegate). Assert the iterator yields `"ok line"`, then a lossy-decoded line containing U+FFFD, then `"after"`, then `None`. Current code: fails (stream ends at the broken line).
- [ ] Step 2: Implement with `read_until(b'\n')` + `String::from_utf8_lossy`, continue after decode-degraded lines; only `Ok(0)` (true EOF) ends the stream. Read errors other than UTF-8 cannot occur through `read_until` on a pipe except real I/O errors - treat a real `Err` from `read_until` as EOF but keep it distinguishable in a comment (it no longer swallows decode issues).
- [ ] Step 3: No-hang regression (gated, live): a scripted fake child (or the fake-mkvmerge helper) that writes an invalid-UTF-8 line followed by >64KiB of further output, then exits; assert `run_job` completes with the full tail captured (pins the job.rs post-None wait() path - the audit verified spawn.rs holds the pipe open during wait, job.rs loop assumption must be pinned by this test).
- [ ] Step 4: Full gate; commit `fix(executor): survive non-UTF-8 output lines - no truncation, no pipe-full hang (#9)`.

