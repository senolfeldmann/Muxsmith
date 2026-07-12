### Task 4.5: D35 run-log auto-prune, 14 days fixed (added 2026-07-11 at the execution go)

Vehicle decision Şenol 2026-07-11 (reserved at the Plan 5.5 authoring, decided at the go): rides this plan as a wave-1 stream-A task instead of a standalone run. Decision D35 itself (14 days, fixed, no v1 config, parity MATCH with mkvtoolnix-gui's remove-old-jobs default) lives in docs/superpowers/specs/2026-07-11-pre-1.0-design-decisions.md; configurability is parked as IDEAS #7.

**Files:**
- Modify: `crates/muxsmith-core/src/executor/joblog.rs` (prune logic in/next to `RunLogger::create` at :127; new pub run-id-timestamp parser)
- Modify: `src-tauri/src/run.rs:916` (`started_at_from_run_id` becomes a thin delegate to the new core parser - reuse-before-writing; its output format and `None`-for-foreign-names contract unchanged)
- Test: `crates/muxsmith-core/tests/joblog.rs`

**Interfaces:** new `pub fn run_id_timestamp(name: &str) -> Option<OffsetDateTime>` in joblog.rs (parses the fixed `YYYYMMDD-HHMMSSZ` 16-byte prefix, tolerating the collision `-N` suffix, exactly the semantics of the current shell parser); new `pub fn prune_stale_runs(runs_root: &Path, now: SystemTime)` called best-effort by `RunLogger::create` before creating the new leaf. No signature change on `create` (both CLI run.rs:317 and shell run.rs:310 callers stay untouched).

- [ ] Step 1: Failing test: `runs_root` seeded with dirs `20200101-000000Z` (stale), `20200101-000000Z-2` (stale, collision-suffixed), a fresh dir named via `make_run_id(now)`, a non-run dir `keep-me`, and a plain file `notes.txt`; after `RunLogger::create`, assert both stale dirs are gone and everything else (fresh, `keep-me`, `notes.txt`, the new run dir) survives. Age is decided by the PARSED NAME ONLY, never mtime (a name that does not parse is a directory core did not create - same philosophy as the existing shell parser doc).
- [ ] Step 2: Implement: retention constant 14 days with a comment citing D35 (fixed, no config in v1; IDEAS #7 parks configurability). Prune iterates `read_dir(runs_root)`, removes via `remove_dir_all` only entries that are directories (not symlinks - check `file_type()`), whose name parses, and whose timestamp is older than `now - 14d`. Best-effort: every io error during pruning is IGNORED, with a rustdoc line stating why that is deliberate (pruning is hygiene; it must never fail or delay a run - the worst case of a failed prune is old logs surviving).
- [ ] Step 3: Move the parse logic to core, delegate the shell's `started_at_from_run_id` to it; the shell's existing tests keep passing unchanged (that is the reuse acceptance criterion).
- [ ] Step 4: Full gate; commit `feat(executor): auto-prune run logs older than 14 days (D35)`.

