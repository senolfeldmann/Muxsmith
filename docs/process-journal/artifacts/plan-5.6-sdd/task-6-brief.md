### Task 6: src-tauri shell (Stream E, first)

**Files:**
- Modify: `src-tauri/src/run.rs`, `src-tauri/src/lib.rs`, `src-tauri/src/settings.rs`, `src/ipc.ts` (one mirrored line)

**Interfaces:** none outward; IPC command signatures lose an unused State param (list_runs, get_job_log) - Tauri 2 supports zero-state commands; frontend invoke sites unchanged (no args passed today - implementer verifies with a grep for both command names in src/).

- [ ] `run.rs:84` **yagni** - delete single-field wrapper ActiveRun; `RunSlot::Running(Arc<QueueControl>)` directly; match arms at commit, abort_and_quit, do_cancel_run, do_cancel_job, and the running() test helper become `Some(RunSlot::Running(ctl)) => ctl.cancel_all()` etc.
- [ ] `lib.rs:296-301, :312-331, :337-349, :367-376` + `run.rs:447-451` **dup** - one `async fn on_blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T, IpcError> + Send + 'static) -> Result<T, IpcError>` in src-tauri (error.rs or lib.rs); the five spawn_blocking + map_err(internal-task-failed) wrappers become closure bodies.
- [ ] `lib.rs:147` **yagni** - FIRST `grep -rn meets_minimum src/` (expected: only the ipc.ts:40 mirror; any real reader = raise, don't delete). Then delete the field, its doc, the `pair >= MIN_SUPPORTED` computation (:284), the assert!(info.meets_minimum) in three tests, and the ipc.ts mirror line. (Mkvmerge::detect already refuses too-old candidates; every Ok carries true by construction.)
- [ ] `run.rs:548, :557` **yagni** - drop the unused `_state: State<AppState>` from list_runs and get_job_log plus the two doc paragraphs about its unusedness (re-adding State later is one line).
- [ ] `run.rs:453` **idiom** - `let outcome = outcome?;` - the explicit `drop(reservation)` in the Err arm is redundant (Drop fires on early return, exactly as the map_err(...)? two lines above already relies on).
- [ ] `settings.rs:320` **native** - `assert!(path.ends_with("muxsmith/settings.json"));` - Path::ends_with matches whole components and std::path recognizes '/' on Windows; the to_string_lossy backslash fallback is dead on all platforms.
- [ ] `run.rs:91-95` **doc (seed T4-m1)** - rewrite lock_active's "Recovery is sound because ..." sentence to: "Recovery is sound because the slot value is only ever replaced wholesale by a single, non-panicking assignment (*slot = Some(...)/*slot = None); the remaining critical sections (the cancel/abort/close arms) only read the slot and call into its contents, never mutating it in place -- so even a panic while the guard is held (e.g. inside cancel_all) cannot leave a half-applied Option<RunSlot>."
- [ ] Full gate; commits `refactor(shell): ...`.

