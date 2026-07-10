### Task 3: mkvmerge detection ladder + version floor (D28)

**Files:**
- Modify: `crates/muxsmith-core/src/capability/runtime.rs`
- Test: `crates/muxsmith-core/tests/mkvmerge_runtime.rs` (append)

**Interfaces:**
- Produces: `Mkvmerge::detect(override_path: Option<&Path>) -> Result<Mkvmerge, RuntimeError>` (ladder: override -> PATH via `locate()` -> platform candidates, each probed with `--version`); `fn platform_candidates() -> Vec<PathBuf>` (private, but its list unit-tested via a seam); `Mkvmerge::version_pair(&self) -> Result<(u64, u64), RuntimeError>` parsing `"mkvmerge vNN.N.N ..."`; `pub const MIN_SUPPORTED: (u64, u64)`; `RuntimeError::TooOld { found: String, minimum: String }` (new variant). Consumed by T7's `detect_mkvmerge` command.

- [ ] **Step 1: Empirically fix the floor (SI-3).** The capability table is generated from identification schema v20; find which mkvtoolnix release introduced format version 20: `grep -rn "identification_format_version" ~/Downloads/mkvtoolnix/NEWS.md | head` (and the src if NEWS is ambiguous). `MIN_SUPPORTED` = that release. Record the evidence in the const's doc comment.
- [ ] **Step 2: Fix the candidate list from authority, not memory.** Windows: `%ProgramFiles%\MkvToolNix\mkvmerge.exe` (+ `(x86)`); macOS: `/Applications/MKVToolNix-*.app/Contents/MacOS/mkvmerge` (glob), `/opt/homebrew/bin/mkvmerge`, `/usr/local/bin/mkvmerge`; Linux: `/usr/bin/mkvmerge`, `/usr/local/bin/mkvmerge`, `/var/lib/flatpak/exports/bin/org.bunkus.mkvtoolnix-gui`? - VERIFY each against mkvtoolnix's own packaging (windows installer NSIS script + macOS bundle layout in ~/Downloads/mkvtoolnix/packaging/) and drop any location the packaging does not actually use. Cite file paths in the doc comment.
- [ ] **Step 3: TDD.** Failing tests: `version_pair` parses `"mkvmerge v100.0.0 ('Message') 64-bit"` -> `(100, 0)`; ladder prefers override over PATH (probe a tempdir fake script on Unix; gate Windows-only aspects); `TooOld` surfaces found+minimum. Implement minimal. Gated live test: `detect(None)` finds the real v100 and `version_pair() >= MIN_SUPPORTED`.
- [ ] **Step 4: Full gate green. Commit** `feat(core): mkvmerge detection ladder + minimum version floor (D28)`

---

