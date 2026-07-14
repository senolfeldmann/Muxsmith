# T2 report: settings.rs fsync before atomic rename (Stream B, verdict item 6)

- **Worktree/branch:** `/home/senol/Git/Muxsmith/.worktrees/plan57-b` (`plan57-b`)
- **File touched:** `src-tauri/src/settings.rs` only
- **Status:** DONE - all gates green, committed `9143866`

## The exact diff

```diff
diff --git a/src-tauri/src/settings.rs b/src-tauri/src/settings.rs
index 85896fb..7dabe41 100644
--- a/src-tauri/src/settings.rs
+++ b/src-tauri/src/settings.rs
@@ -18,7 +18,7 @@

 use std::collections::HashMap;
 use std::fs;
-use std::io;
+use std::io::{self, Write};
 use std::path::{Path, PathBuf};

 use serde::{Deserialize, Serialize};
@@ -142,6 +142,13 @@ pub fn load(path: &Path) -> Result<AppSettings, SettingsError> {
 /// publishes it onto `path` -- same-filesystem rename is atomic on Linux,
 /// macOS, and Windows, so any reader of `path` always sees either the
 /// previous complete file or the new complete one, never a partial write.
+/// The temp file is flushed to disk ([`fs::File::sync_all`]) before the
+/// rename: rename atomicity alone only covers process death, while under
+/// delayed allocation (e.g. ext4, btrfs) the rename can reach the journal
+/// before the temp file's data blocks reach the disk, turning a power loss
+/// into exactly the torn/empty file this path exists to prevent. With the
+/// fsync, a power cut yields the previous complete file (rename lost) or
+/// the new complete one -- never a torn one.
 /// `fs::rename` is the ONLY thing that ever touches the final name. On a
 /// failed rename the temp file is removed rather than left behind (its
 /// error is deliberately swallowed: the publish failure is what gets
@@ -166,7 +173,17 @@ pub fn save(path: &Path, settings: &AppSettings) -> Result<(), SettingsError> {
         .unwrap_or_else(|| "settings".to_string());
     let tmp_path = parent.join(format!(".{file_name}.tmp-{}", std::process::id()));

-    fs::write(&tmp_path, &bytes).map_err(|e| SettingsError::Io(e.to_string()))?;
+    let mut tmp_file = fs::File::create(&tmp_path).map_err(|e| SettingsError::Io(e.to_string()))?;
+    tmp_file
+        .write_all(&bytes)
+        .map_err(|e| SettingsError::Io(e.to_string()))?;
+    tmp_file
+        .sync_all()
+        .map_err(|e| SettingsError::Io(e.to_string()))?;
+    // Drop closes the temp file before the publish rename, matching the
+    // open-write-close behavior of the fs::write this replaced instead of
+    // leaning on platform-specific rename-while-open semantics.
+    drop(tmp_file);
     fs::rename(&tmp_path, path).map_err(|e| {
         let _ = fs::remove_file(&tmp_path);
         SettingsError::Io(e.to_string())
```

## Why the error mapping matches the file's existing style

Every fallible step in `save()`/`load()` already maps its `io::Error` via
`.map_err(|e| SettingsError::Io(e.to_string()))` (create_dir_all at :153,
the former fs::write, fs::rename at :170, fs::read_to_string in `load`);
`SettingsError::Io(String)` is documented as carrying "the underlying OS
error text" and nothing else (the `detail` prose-passthrough exception,
core-37's shell-side analogue). All three new steps (`File::create`,
`write_all`, `sync_all`) use that identical closure - no new variant, no
composed prose, no `From` impl the file does not otherwise use.

Other conformance points:

- `fs::File::create` (module-prefixed) matches the file's existing
  `fs::write`/`fs::rename`/`fs::read_to_string` call style; only the
  `Write` trait needed an import, folded into the existing
  `use std::io;` line as `use std::io::{self, Write};`.
- Explicit `drop(tmp_file)` before the rename preserves exact behavioral
  parity with `fs::write` (which opens, writes, closes before returning):
  the handle is provably closed before the publish rename, instead of
  relying on Rust std's Windows share-mode defaults (FILE_SHARE_DELETE)
  to permit rename-while-open. Comment states the meaning, per
  BUILDING.md's "rustdoc/comments state MEANING" bar.
- Rustdoc: the durability claim ("crash, power loss, kill",
  "previous complete or new complete, never partial") is kept verbatim
  and NOT weakened; one paragraph was added documenting the fsync step
  and why it is load-bearing (delayed-allocation torn-file mode), since
  the doc describes the publish mechanism step by step and would
  otherwise describe an incomplete mechanism. `[`fs::File::sync_all`]`
  is an intra-doc link, gated by the `cargo doc -D warnings` part.

## Scope discipline (verdict item 6 boundaries)

- NO directory fsync (outside the doc's claim: a power-lost rename
  legitimately yields the *previous* complete file). Not added.
- NO joblog changes (separately tracked v1.x). Not touched.
- No other file touched; tests unchanged (task requires existing tests
  to pass unchanged).
- Considered and deliberately NOT added (parity + scope): temp-file
  cleanup on a failed `write_all`/`sync_all`. The pre-change code had the
  identical behavior on a failed `fs::write` (temp may remain; only the
  failed *rename* cleans up), and the verdict's minimal fix names no
  cleanup. Flagging for the reviewer rather than silently expanding scope.

## Gates (all FOREGROUND, run from the worktree root; run pre-commit, re-run post-commit at HEAD 9143866)

All five BUILDING.md Rust gate parts, workspace-wide:

```
== cargo test -p muxsmith-gui ==
test result: ok. 78 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s   (lib, incl. all 7 settings tests)
test result: ok. 0 passed; ...   (main.rs unit target)
test result: ok. 0 passed; ...   (doc-tests muxsmith_gui_lib)

== cargo fmt --all --check ==
clean (exit 0)

== cargo clippy --workspace --all-targets -- -D warnings ==
Finished `dev` profile [unoptimized + debuginfo] target(s)   (exit 0, no warnings)

== RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps ==
Generated .../target/doc/muxsmith_cli/index.html and 5 other files   (exit 0;
gates the new [`fs::File::sync_all`] intra-doc link)

== cargo deny check ==
advisories ok, bans ok, licenses ok, sources ok   (exit 0)

== cargo test --workspace ==
36 of 36 "test result:" lines are "ok", 0 non-ok, exit 0
(incl. core planner/executor/CLI suites and the mkvmerge-gated live tiers)
```

Existing settings tests pass unchanged (none were modified), including the
two atomic-publish regression guards
(`save_leaves_no_temp_file_behind_after_a_successful_write`,
`save_cleans_up_its_temp_file_when_the_publish_rename_fails`).

## Commit

- **Hash:** `91438667c743a5f352170c38a3ccdf0d983da54a` (short `9143866`) on
  branch `plan57-b`, worktree `/home/senol/Git/Muxsmith/.worktrees/plan57-b`
- **Message:** `settings: fsync temp file before atomic rename (durability claim now holds)`
- **Staging:** explicit `git add src-tauri/src/settings.rs` only (never `-A`);
  1 file changed, 19 insertions(+), 2 deletions(-); working tree clean after.
- **Unsigned** via `-c commit.gpgsign=false` (proc-05).

## Deviations surfaced

1. **Commit trailers:** the task's literal commit command carried only the
   `Co-Authored-By` trailer; the repo's actual commit style (checked against
   the last 3 commits on master/plan57-b) also carries a `Claude-Session:`
   line, so both trailers were included - a superset of the specified
   command, conforming to house pattern.
2. **Temp-file cleanup on failed write/sync deliberately not added** (see
   Scope discipline above): behavior parity with the replaced `fs::write`,
   and outside the verdict's minimal fix. Reviewer may weigh in.
3. **cargo deny was run** although the change touches no dependencies -
   ci-06 says all five parts before every commit, never skipped; running it
   was cheaper than justifying a skip. No deviation from ci-06.

No deviations from docs/conventions.yaml; the relevant entries
(core-37 analogue: error carries only OS error text; BUILDING.md rustdoc
meaning bar) are addressed in the conformance section above.
