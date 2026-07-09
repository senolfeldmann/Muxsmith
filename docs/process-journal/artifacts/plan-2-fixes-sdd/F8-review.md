# F8 review: symlink discovery in `walk_files`

Independent review of `crates/muxsmith-core/src/discovery.rs` change (commit `cb3ae84`, "fix(core): discover symlinked source files (F8, bug I)"). Read-only: diff package, report, `discovery.rs` in full, `planner.rs`/`identify.rs` call sites; ran `cargo test`/`clippy`/`fmt` myself rather than trusting the report's transcript.

## Code under review

```rust
if meta.is_dir() {
    if recursive {
        stack.push(path);
    }
} else if meta.is_file() {
    out.push(path);
} else if meta.file_type().is_symlink() {
    let Ok(target_meta) = std::fs::metadata(&path) else {
        continue; // broken symlink; skip silently
    };
    if target_meta.is_file() {
        out.push(path);
    }
    // A directory target is never recursed into (cycle guard).
}
```

`meta` comes from `symlink_metadata`, so `is_dir()`/`is_file()` reflect the link itself and are both false for any symlink entry (real dirs/files, unaffected, still hit the first two arms exactly as before). Only symlink entries reach the third arm.

## 1. SPEC / correctness

- **File-target symlink included.** `target_meta.is_file()` -> `out.push(path)`, pushing the symlink's own path (not the resolved target), same convention as a real file. Verified with the `discovers_symlinked_primary_file` test and confirmed it goes through the full `scan_primaries` pattern-match/identifier path, not a bare existence check.
- **Dir-target symlink not recursed.** The dir-target case is a structural no-op: there is no `stack.push` anywhere in the symlink arm, so a directory-target symlink can never enter the traversal stack regardless of `recursive`. This is a stronger guarantee than a runtime check (e.g. a visited-set) — no code path exists that would recurse into it. Cycle-safe by construction, including A-symlink-to-B/B-symlink-to-A style patterns between two directory symlinks, since neither ever gets pushed.
- **Broken symlink skipped silently, no panic.** `std::fs::metadata(&path)` on a dangling target returns `Err`, caught by the `let-else` -> `continue`. No `.unwrap()`/`.expect()` anywhere in the new arm. Matches the existing convention used for unreadable directories in the same function. Logically sound, but see Test Hygiene below: **this path has zero test coverage.**
- **Symlink-to-symlink chain.** `std::fs::metadata` is `stat(2)`, which resolves an arbitrary link chain to its final target in one call (kernel-bounded, `ELOOP` on excess/cyclic chains -> `Err` -> falls into the same broken-symlink skip branch). So: a chain ending in a file is included, a chain ending in a directory is not recursed (same check, since only the *final* resolved type is inspected), and a chain that cycles is an `Err`, landing in the skip branch rather than hanging. No issue; this is standard, well-defined OS behavior, not something the implementation had to build itself.
- **Determinism.** `dir_entries.sort()` before per-entry processing and `out.sort()` on the accumulated result are both unchanged; the symlink arm only affects *what* goes into `out`, not the sort. Output stays deterministic.
- **Regression check on real files/dirs.** The `is_dir()`/`is_file()` arms are byte-for-byte unchanged from before the fix; all four pre-existing tests (`scans_primaries_and_extracts_named_groups`, `duplicate_identifier_is_warned`, `multiple_identifier_matches_uses_first`, `non_recursive_scan_skips_subdirs`) still pass.
- **Callers (`scan_primaries`, `resolve_locator`).** Both consume `walk_files` unchanged; neither has special-cased symlinks, so both transparently gained the fix. Traced downstream consumers of the returned `PathBuf`s:
  - `planner.rs::resolve_file` passes `primary.path` / donor paths from `resolve_locator` into `Identify::identify`, and does a `primary_paths.contains(&donor)` literal-path-equality check (`DonorIsPrimary`). Both are pre-existing, path-string-based, not canonicalization-based, so behavior for a symlink path is identical in kind to behavior for a real-file path — no new dedup or double-counting risk introduced by allowing symlink paths into these lists.
  - `identify.rs::IdentifyCache` keys its cache on the literal `PathBuf` and calls `std::fs::metadata(path)` for the `(mtime, size)` cache key, which already follows symlinks (same call semantics as the new discovery arm) and was already reachable pre-fix for symlinks used directly as CLI/config-specified paths elsewhere; `mkv.identify_json(path)` shells out to `mkvmerge` with that path, which the OS resolves transparently regardless of whether it's a symlink. No behavior change needed or found here.
  - No regression found in either call site.

**Verdict: SPEC pass.** All four required behaviors (include file-target, don't recurse dir-target, skip broken silently, no output-determinism regression) are implemented correctly and match the task.

## 2. Test hygiene

Two new `#[cfg(unix)]` tests, both real (not vacuous): they call `std::os::unix::fs::symlink` to create an actual symlink on the test's tempdir, then assert through the full `scan_primaries` path (identifier extraction, primary count), not just a raw `walk_files` existence check.

- `discovers_symlinked_primary_file`: real target file in one tempdir, symlink to it (matching name) in a second, scanned tempdir. Asserts exactly one primary, `path == link` (the symlink's own path, not the target's), and correct identifier extraction. Confirmed genuinely RED before the fix per the report's transcript (`diags: []`, 0 primaries) and confirmed independently GREEN now (`cargo test -p muxsmith-core --lib discovery`: 7/7 pass).
- `symlinked_directory_is_not_recursed_into`: real directory with a matching file inside, symlinked into the scanned (recursive) source dir, plus a directly-matching file in the source dir itself. Asserts only the direct file's identifier is found, i.e. the file inside the symlinked directory never surfaces. Correctly exercises the cycle-guard arm of the new code. Note (disclosed accurately in the report): this specific test already passed *pre-fix* too, because the old code followed no symlinks of any kind — it became a meaningful regression guard for the *new* code's dir-symlink branch specifically only after the fix landed, not evidence the fix changed this case's outcome. Not vacuous, just not RED/GREEN-informative on its own; the report is transparent about this and it doesn't weaken the test's value going forward.

**Gap found: broken/dangling symlink is untested.** The task spec explicitly calls for "skip broken symlinks silently," and the implementation adds a dedicated `let-else` branch for exactly this case, but no test creates a dangling symlink (e.g. `symlink(dir.join("does-not-exist"), link)`) and asserts it's silently skipped (zero primaries, zero diagnostics, no panic). This is the one explicitly-specified behavior in the task with no automated coverage at all — see Important finding below.

**Minor gap:** no test for a multi-hop symlink chain (symlink -> symlink -> file, or -> directory). Low risk since the behavior falls out of `stat(2)` semantics rather than custom logic, but it is untested and was called out explicitly in the review brief.

**Verdict: QUALITY changes-needed** — not because anything is broken, but because one explicitly-specified behavior (broken-symlink skip) has no regression test, which is exactly the kind of gap a later refactor (e.g. someone "simplifying" the `let-else` to an `.expect()` because "a symlink we just matched on can't fail to stat") would sail through undetected.

## Verification performed independently

- `cargo test -p muxsmith-core --lib discovery`: 7/7 pass (matches report).
- `cargo test --workspace`: all suites pass, 0 failed.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `cargo fmt --all --check`: clean.
- Read `discovery.rs` in full (not just the diff) and the two downstream call sites (`planner.rs`, `identify.rs`) to check for symlink-unaware assumptions; found none.

## Findings

1. **Important** — `crates/muxsmith-core/src/discovery.rs`, the broken-symlink branch (`walk_files`, the `let Ok(target_meta) = std::fs::metadata(&path) else { continue; }` line): explicitly specified behavior ("skip broken symlinks silently") has zero test coverage. Scenario: a future edit replaces the `let-else` with something that assumes the stat can't fail (since the entry already passed `is_symlink()`), e.g. `.unwrap()`; a source directory containing one dangling symlink then panics the whole discovery pass in production, and the test suite stays green because nothing ever exercises a dangling symlink.
2. **Minor** — same function: no test for a multi-hop symlink chain (symlink -> symlink -> file/dir). Behavior is correct by `stat(2)` semantics and reasoned about correctly in the report, but is unverified by any test.
