### Task 7: Shell IPC - read-only commands + settings (D23, D27, D28)

**Files:**
- Create: `src-tauri/src/settings.rs`, `src-tauri/src/error.rs`
- Modify: `src-tauri/src/lib.rs` (commands + state + invoke_handler)
- Test: `src-tauri/src/settings.rs` unit tests (tempdir config dir); command fns factored so the core-calling body is testable without a Tauri runtime

**Interfaces:**
- Produces (all returns `Result<_, IpcError>`; `IpcError { code: String, params: HashMap<String, String> }` - codes only, frontend renders via Fluent):

```rust
#[tauri::command] async fn validate_profile(path: String) -> ...serde_json::Value   // report::json::config_only_document
#[tauri::command] async fn dry_run(profile: String, source: Option<String>, output: Option<String>) -> ...Value  // batch_document; runs on spawn_blocking
#[tauri::command] async fn identify(file: String) -> ...Value
#[tauri::command] fn detect_mkvmerge(state: State<AppState>) -> ...MkvmergeInfo     // { path: String, version: String, meets_minimum: bool } via Mkvmerge::detect(settings override) + version_pair >= MIN_SUPPORTED
#[tauri::command] fn get_settings(state) -> AppSettings
#[tauri::command] fn set_settings(state, s: AppSettings) -> ...()
```

- `AppSettings` (serde, JSON at `app_config_dir()/settings.json`): `mkvmerge_path: Option<String>`, `default_jobs: usize` (1), `locale: Option<String>`, `recent_profiles: Vec<String>` (cap 10, MRU), `dir_memory: HashMap<String, DirMemory { source: Option<String>, output: Option<String> }>` keyed by profile path (D27: never written into the user's YAML).
- Consumed by: T9 (first-run + settings dialog), T10 (validate/dry_run/recents/dir_memory).

- [ ] **Step 1: TDD settings round-trip** (write/read/missing-file defaults/MRU cap) against a tempdir.
- [ ] **Step 2:** Implement commands as thin wrappers; no prose, no logic beyond argument plumbing and settings I/O. Register in `invoke_handler`; grant `dialog` + `clipboard-manager` + `event` permissions in `capabilities/default.json`.
- [ ] **Step 3:** Full gate + pnpm build green. **Commit** `feat(gui): read-only IPC commands + app settings persistence`

---

