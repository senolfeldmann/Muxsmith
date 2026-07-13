# Idiomacy review, slice F4: src-tauri/ Rust

Scope: all .rs under `src-tauri/` excluding `target/`. Enumerated and read completely:

- `src-tauri/build.rs` (6 lines)
- `src-tauri/src/main.rs` (10)
- `src-tauri/src/error.rs` (251)
- `src-tauri/src/settings.rs` (327)
- `src-tauri/src/lib.rs` (839)
- `src-tauri/src/run.rs` (1713)

Toolchain judged against: Rust 1.96.1, edition 2024, Tauri 2.

## Findings

### F4-1 (yagni): `MkvmergeInfo.meets_minimum` is a dead flag — always `true`, never read

`src-tauri/src/lib.rs:147` (field), computed at line 284 (`meets_minimum: pair >= MIN_SUPPORTED`).

`Mkvmerge::detect` already refuses a too-old candidate outright (D28); a too-old mkvmerge surfaces as `IpcError` `mkvmerge-too-old`, never as an `Ok`. So every `Ok(MkvmergeInfo)` carries `meets_minimum: true` by construction — the field's own doc concedes it is "a defensive re-check of that same fact, not the primary signal the frontend branches on". Verified: no frontend code reads it; the only frontend occurrence is the mirrored type declaration (`src/ipc.ts:40`). An IPC payload field that is provably constant and unconsumed is exactly the dead flag yagni targets.

**Replacement:** delete the field, its doc, the `pair >= MIN_SUPPORTED` line, the `assert!(info.meets_minimum)` lines in three tests, and the mirrored `meets_minimum: boolean;` in `src/ipc.ts`. If the invariant "detect enforces the floor" is worth pinning, the existing `detect_mkvmerge_body_too_old_carries_found_and_minimum` test already does.

lines_cut: 8, deps_cut: 0.

### F4-2 (yagni): `ActiveRun` is a single-field wrapper with one construction site

`src-tauri/src/run.rs:84`.

```rust
pub(crate) struct ActiveRun {
    ctl: Arc<QueueControl>,
}
```

One field, one construction site (`Reservation::commit`), no methods, no delegation; its own doc explains why it deliberately carries nothing else (the run id "is not duplicated here"). The enum variant already names the state — `RunSlot::Running(ActiveRun { ctl })` says nothing that `RunSlot::Running(Arc<QueueControl>)` does not.

**Replacement:** `RunSlot::Running(Arc<QueueControl>)`; match arms become `Some(RunSlot::Running(ctl)) => ctl.cancel_all()` etc. (call sites: `commit`, `abort_and_quit`, `do_cancel_run`, `do_cancel_job`, test helper `running()`).

lines_cut: 13, deps_cut: 0.

### F4-3 (yagni): unused `_state` parameters on `list_runs` and `get_job_log`

`src-tauri/src/run.rs:548` (`list_runs`) and `:557` (`get_job_log`).

Both commands take `_state: State<AppState>` that nothing reads; the doc keeps it "for parity with the other run-lifecycle commands and any future runs-root override". That is a prediction, not a need — a parameter kept for a feature nobody built. A Tauri 2 command with zero state params is fully supported, and re-adding `State` later is a one-line change at exactly the moment the override actually exists.

**Replacement:** drop the parameter from both signatures and delete the two doc paragraphs explaining its unusedness.

Caveat for the merge stage: the doc calls it "part of the given interface"; if the task brief's interface sketch is treated as binding here, this becomes a recorded decision. It is not on the known-non-findings list, so reported.

lines_cut: 6, deps_cut: 0.

### F4-4 (idiom): hand-rolled `match` + explicit `drop` where `?` is the construct

`src-tauri/src/run.rs:453-459`, in `start_run`:

```rust
let outcome = match outcome {
    Ok(outcome) => outcome,
    Err(e) => {
        drop(reservation);
        return Err(e);
    }
};
```

is `let outcome = outcome?;`. The explicit `drop(reservation)` is redundant: the uncommitted `Reservation`'s `Drop` fires on the early return either way, and the statement immediately above (the `spawn_blocking` join-error `map_err(...)?` at line 447-451) already relies on exactly that implicit drop while the reservation is live. Same function, same invariant, two different mechanisms — the `?` form is both the edition-2024 norm and internally consistent.

lines_cut: 6, deps_cut: 0.

### F4-5 (native): redundant Windows string fallback — `Path::ends_with` already matches cross-platform

`src-tauri/src/settings.rs:320-323`, test `settings_path_lives_under_a_muxsmith_subdirectory`:

```rust
path.ends_with("muxsmith/settings.json") || {
    // Windows path separator.
    path.to_string_lossy().ends_with("muxsmith\\settings.json")
}
```

`Path::ends_with` compares whole path components, and on Windows `std::path` recognizes both `/` and `\` as separators (`std::path::is_separator('/')` is `true` there), so the needle `"muxsmith/settings.json"` parses to the components `[muxsmith, settings.json]` on every platform and already matches a `\`-separated Windows path. The lossy-string fallback branch is dead on all platforms — the stdlib does this natively.

**Replacement:** `assert!(path.ends_with("muxsmith/settings.json"));`

lines_cut: 4, deps_cut: 0.

## Considered and not flagged

- `settings.rs` hand-rolled atomic write (temp file + `fs::rename`): `tempfile::NamedTempFile::persist` would be the library route, but `tempfile` is a dev-dependency only; promoting it to a runtime dependency of the release binary to replace ~10 well-tested lines fails the dependencies-are-earned bar. Hand-rolled write-then-rename is an accepted, human-normal pattern here.
- `ftl_message` line-parser instead of fluent-rs in the shell: documented, tested, proportionate (four one-line strings; a full `FluentBundle` would be a new dependency of this crate duplicating the frontend loader).
- `CloseDecision` two-variant enum with one production caller: isomorphic to `bool`, but a named two-variant enum over boolean-blindness is accepted Rust style and the call site reads better; not a violation.
- `default_jobs()` free fn + manual `Default for AppSettings`: required shape for a serde field default of `1`; idiomatic.
- Recorded non-findings honored: fake-mkvmerge helper copies (3, at tracked trigger), `RECENT_PROFILES_CAP` TS/Rust duplication (B11), `MUXSMITH_RUNS_ROOT` debug-only (D26), regex-per-`matches()` (v1.x entry — not in this slice anyway), version pins.
- `Reservation`/`TeardownGuard` RAII, `PlanOutcome::Ready(Box<...>)`, poison-recovering `lock_active`: complexity earned by documented invariants (single-run gate, panic-safe teardown, `large_enum_variant`).
- `BatchView` passing `jobs: settings.value.default_jobs` while the shell's `jobs.unwrap_or(1)` ignores `AppSettings.default_jobs`: checked; the frontend comment records this as deliberate (`BatchView.vue:266-271`). Not a finding.

## Routed (out-of-scope observations, not findings)

- `src-tauri/src/settings.rs:150` (`save`): the doc claims the atomic-rename publish protects against "crash, power loss, kill", but the temp file is never `sync_all()`d before `fs::rename` (nor the directory after). Rename atomicity covers process crash; on power loss with delayed allocation (ext4/btrfs), the renamed `settings.json` can still come up empty or torn. Either add an `File::sync_all` before the rename or scale the doc claim back to process-crash safety. Correctness/durability, routed.

## Verdict

Not clean: 5 findings (3 yagni, 1 idiom, 1 native), all small; 1 routed durability note.
