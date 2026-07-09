# F8: discovery symlink handling (bug I) - report

## Status

DONE

## What changed

`crates/muxsmith-core/src/discovery.rs` (`walk_files` only):

- Added a third arm to the per-entry match, alongside the existing
  `meta.is_dir()` / `meta.is_file()` (both queried via `symlink_metadata`,
  which does not follow links -- so a symlink entry hits neither and fell
  through silently before this fix). The new arm triggers on
  `meta.file_type().is_symlink()` and resolves the target's type via
  `std::fs::metadata(&path)`, which does follow the link:
  - Target metadata `Ok` and `is_file()`: push the entry's own path (the
    symlink path, not the resolved target) onto `out`, same as a real file.
  - Target metadata `Ok` and `is_dir()`: no-op. Directory symlinks are never
    recursed into regardless of `recursive`, by construction (there is no
    `stack.push` in this arm) -- the task's explicit cycle guard.
  - Target metadata `Err` (broken symlink, missing/unreadable target):
    `continue` inside the `let-else`, i.e. skipped silently, matching the
    task's specified behavior and the existing convention for unreadable
    directories in the same function.
- Updated the function's doc comment to state the new symlink-target
  resolution and cycle-guard behavior (previously said "Symlinks are not
  followed").
- No signature change, no new `pub` items, no new diagnostic. `scan_primaries`
  and `resolve_locator` both consume `walk_files` unchanged and pick up the
  fix automatically: a symlinked primary now reaches the existing
  extension/pattern-match logic, so it is discovered exactly like a real file,
  with the same `IgnoredFile`/`MultipleIdentifierMatches`/`DuplicateIdentifier`
  diagnostics applying to it as to any other candidate.

## Test-first

Added to `crates/muxsmith-core/src/discovery.rs`'s existing `#[cfg(test)]
mod tests`, both `#[cfg(unix)]` (use `std::os::unix::fs::symlink`; this
project runs on Linux/macOS):

1. **`discovers_symlinked_primary_file`**: a real target `Show.S01E05.mkv`
   in one `tempdir` (outside the scanned tree entirely), a symlink to it
   named identically inside a second `tempdir` (the scanned source dir).
   Asserts `scan_primaries` returns exactly one `PrimaryFile`, that its
   `path` equals the symlink's own path (not the target's), and that its
   identifier is extracted correctly (`S01E05`) -- i.e. the symlinked file
   goes through the normal pattern-matching path, not just a bare "is it
   found" check.
2. **`symlinked_directory_is_not_recursed_into`**: a real directory in one
   `tempdir` containing `E09.mkv`, a symlink to that directory placed inside
   a second `tempdir` (the recursive-scanned source dir) alongside a direct
   `E01.mkv`. Asserts exactly one primary is found (`E01`) -- `E09` inside the
   linked directory must not surface, which would happen if the directory
   symlink were followed.

Confirmed RED before implementing (tests added, `walk_files` unchanged):

```
$ cargo test -p muxsmith-core --lib discovery
running 7 tests
test discovery::tests::resolve_locator_matches_by_identifier ... ok
test discovery::tests::multiple_identifier_matches_uses_first ... ok
test discovery::tests::symlinked_directory_is_not_recursed_into ... ok
test discovery::tests::discovers_symlinked_primary_file ... FAILED
test discovery::tests::non_recursive_scan_skips_subdirs ... ok
test discovery::tests::duplicate_identifier_is_warned ... ok
test discovery::tests::scans_primaries_and_extracts_named_groups ... ok

failures:
---- discovery::tests::discovers_symlinked_primary_file stdout ----
thread '...' panicked: assertion `left == right` failed: diags: []
  left: 0
 right: 1
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 59 filtered out
```

Exactly the predicted symptom: the symlinked file is dropped with zero
diagnostics (`diags: []`), not merely mis-sorted or mis-classified.
`symlinked_directory_is_not_recursed_into` already passed pre-fix, since the
pre-fix code follows no symlinks at all (a stricter-than-required but
harmless pre-fix behavior for that one case); it stays green post-fix as a
regression guard for the cycle guard specifically.

After implementing:

```
$ cargo test -p muxsmith-core --lib discovery
running 7 tests
test discovery::tests::resolve_locator_matches_by_identifier ... ok
test discovery::tests::multiple_identifier_matches_uses_first ... ok
test discovery::tests::symlinked_directory_is_not_recursed_into ... ok
test discovery::tests::non_recursive_scan_skips_subdirs ... ok
test discovery::tests::discovers_symlinked_primary_file ... ok
test discovery::tests::duplicate_identifier_is_warned ... ok
test discovery::tests::scans_primaries_and_extracts_named_groups ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 59 filtered out
```

GREEN confirmed.

## Full verification commands run

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed
    muxsmith-core lib: 66 passed (was 64, +2 F8 tests)

$ cargo fmt --all --check
Diff in .../discovery.rs (one over-long line in the new
symlinked_directory_is_not_recursed_into test)
$ cargo fmt --all
$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)
```

ASCII check (`grep -nP '[^\x00-\x7F]' crates/muxsmith-core/src/discovery.rs`):
no matches.

`#![deny(missing_docs)]`: no new `pub` items were added (the fix is entirely
inside the private `walk_files`; its doc comment was updated, not newly
required). Nothing else to satisfy.

`prose-free core`: no new strings, diagnostics, or locale keys; the fix is
pure control flow plus a private-function doc comment.

## Concerns

None blocking.

- The fix resolves the symlink's target type via a second `stat` call
  (`std::fs::metadata`) only for entries that are actually symlinks (a small
  minority in the common case of a real media library); no extra syscall
  cost for the non-symlink fast path.
- A symlink whose target is itself a symlink chain is resolved transitively
  by `std::fs::metadata` per its normal (follow-until-final-target)
  semantics; a chain ending in a directory is still not recursed into (same
  guard, since the final resolved type is what is checked), and a chain
  containing a cycle is an `Err` from `metadata` (OS-level ELOOP), landing in
  the existing broken-symlink skip branch -- no infinite loop possible
  either at the immediate-child level (guarded explicitly) or through a
  deeper symlink-to-symlink cycle (guarded by the OS and surfaced as a
  metadata error).
- Did not add a diagnostic for "symlink found and included" or "directory
  symlink skipped": the task specifies silent inclusion (like a real file)
  and silent skip (like an unreadable directory), respectively, so no new
  `DiagCode` was warranted per the task's literal wording ("no diagnostic").

## Addendum: review-flagged test-coverage gaps closed

The independent review (`F8-review.md`) passed SPEC but flagged QUALITY
changes-needed: two explicitly-specified `walk_files` behaviors had zero test
coverage (broken-symlink skip: Important; multi-hop chain: Minor).
`walk_files` itself was confirmed correct by the review and left unchanged
here; only tests were added, to
`crates/muxsmith-core/src/discovery.rs`'s existing `#[cfg(test)] mod tests`,
both `#[cfg(unix)]`:

1. **`broken_symlink_is_skipped_silently`**: a dangling symlink
   (`Show.S01E06.mkv`, pattern-matching name) pointing at a nonexistent
   target, alone in the scanned source dir. Asserts zero primaries and zero
   diagnostics -- silent skip, no panic, no `IgnoredFile` or any other
   diagnostic. Locks in the `let Ok(target_meta) = std::fs::metadata(&path)
   else { continue }` branch: a future edit that swaps this for `.unwrap()`
   would panic this test instead of shipping a production regression.
2. **`multi_hop_symlink_chain_is_discovered`**: a three-tempdir chain --
   real file in `target_dir`, an intermediate symlink `hop.mkv` in `link_dir`
   pointing at it, and the scanned source dir's `Show.S01E07.mkv` (the only
   pattern-matching name in the chain) pointing at `hop.mkv`. Asserts exactly
   one primary, `path` equal to the source dir's own link (not either
   resolved hop), and correct identifier extraction -- confirms
   `fs::metadata` resolves the full chain in one call, as the review reasoned
   from `stat(2)` semantics but had not exercised.

Both pass:

```
$ cargo test -p muxsmith-core --lib discovery
running 9 tests
test discovery::tests::multiple_identifier_matches_uses_first ... ok
test discovery::tests::non_recursive_scan_skips_subdirs ... ok
test discovery::tests::broken_symlink_is_skipped_silently ... ok
test discovery::tests::symlinked_directory_is_not_recursed_into ... ok
test discovery::tests::duplicate_identifier_is_warned ... ok
test discovery::tests::resolve_locator_matches_by_identifier ... ok
test discovery::tests::discovers_symlinked_primary_file ... ok
test discovery::tests::multi_hop_symlink_chain_is_discovered ... ok
test discovery::tests::scans_primaries_and_extracts_named_groups ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 59 filtered out
```

Full gate re-run:

```
$ cargo test --workspace
    muxsmith-core lib: 68 passed (was 66, +2 tests); every other suite: ok, 0 failed

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)
```

`walk_files` was not touched: no defect was found, so no behavior change was
made per the task's constraint.
