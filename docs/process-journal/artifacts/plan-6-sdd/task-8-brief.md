### Task 8: D42 - the editor's IPC surface

**Files:**
- Modify: `src-tauri/src/lib.rs` (3 new commands + `apply_suggestion` command + a `load_profile_body`; `invoke_handler` at `:440-452`)
- Modify: `src-tauri/src/error.rs` (`From<SaveError>`, `From<ApplyError>`)
- Modify: `locales/en/gui-common.ftl`, `locales/de/gui-common.ftl`
- Test: `src-tauri/src/lib.rs` unit tests, `src-tauri/src/error.rs` unit tests

**Interfaces:**
- Consumes: Task 2's `profile::save::{to_file, SaveError}`; Task 6's `planner::{apply_suggestion, ApplyError, StructuredEdit}`; the existing `profile::validate::{config_diagnostics, config_diagnostics_from_file}` (`profile/validate.rs:193`) and `report::json::config_only_document`.
- Produces, for Tasks 9-14: four commands and the `load_profile` document shape.

| command | signature |
|---|---|
| `load_profile` | `async fn load_profile(path: String) -> Result<serde_json::Value, IpcError>` |
| `save_profile` | `async fn save_profile(path: String, profile: Profile) -> Result<(), IpcError>` |
| `validate_profile_model` | `async fn validate_profile_model(profile: Profile) -> Result<serde_json::Value, IpcError>` |
| `apply_suggestion` | `async fn apply_suggestion(profile: Profile, config_path: String, edit: StructuredEdit) -> Result<Profile, IpcError>` |

**Read first:** design D42 (`:303-407`, **as amended by Task 1** - `load_profile` returns the `config_only_document` envelope plus a `"profile"` key, no bespoke struct), D43's command paragraph (`:417-421`), and D49 §"The shell mapping" (`:592-620`) and §"The catalog entries" (`:622-659`).

Binding points:
- **`load_profile` returns no bespoke struct.** Add a `load_profile_body(path: &Path) -> serde_json::Value`, mirroring the existing `validate_profile_body` (`src-tauri/src/lib.rs:174-177`): on `load::from_file(path)` -> `Ok(profile)`, build `report::json::config_only_document(&validate::config_diagnostics(&profile), None, &ShellRenderer)` and inject `doc["profile"] = serde_json::to_value(&profile)`; on `Err(d)` (a `ParseError`), build `report::json::config_only_document(&[d], None, &ShellRenderer)` and set `doc["profile"] = serde_json::Value::Null`. Its `config_diagnostics` array is then byte-identical in shape to `validate_profile`'s (both go through `config_only_document`/`rendered_diags`), with the added `"profile"` key the only difference (owner decision, `core-85`).
- **`validate_profile(path)` is kept, not changed, not retargeted, not renamed, not removed.** It has a live consumer at `src/views/BatchView.vue:118`, and the batch view has no model to send. The two commands are not redundant: one validates a file the user picked by path, the other validates a model the user is editing. Both funnel into the same `config_diagnostics`, so no logic is duplicated (spec 7).
- **`validate_profile_model` returns `serde_json::Value`, not a typed struct**, and is byte-identical in envelope to `validate_profile`/`load_profile` (all three go through `config_only_document`). It wraps `validate::config_diagnostics(&profile)` directly (no disk).
- All four are `async` on `on_blocking` (`:73-79`), but for **two different reasons**, and the distinction is the thing an implementer gets wrong by pattern-matching. `load_profile`/`save_profile` touch the disk. **`validate_profile_model` does not touch the disk at all** - `config_diagnostics` is pure (`validate.rs:20-21`). It is on `on_blocking` because it is CPU-bound work on every keystroke: it compiles every regex and parses every template, and Tauri 2 runs a non-`async` command on the main thread, so a plain `fn` would stall the webview on each edit. "Touches the disk" is not the criterion; "could stall the webview" is - which is why `get_settings`/`set_settings` are deliberately non-async despite real file I/O.
- The `Err` case stays what it is everywhere else in this file: the blocking task itself panicking. **Expected failures are diagnostics in the document, not `Err`.** The `Err(IpcError)` path exists only for `save_profile` and `apply_suggestion`, whose `SaveError`/`ApplyError` are genuine operational failures.

- [ ] **Step 1: Write the failing error-mapping and load-profile-shape tests**

First, in `src-tauri/src/error.rs`'s test module, mirroring the existing `settings_errors_map_to_distinct_codes` shape. The `ApplyError` variants and `IpcError` codes are D49's (`:530-559`, `:597-616`); Task 1 already repointed the design's section-2 catalog row to the same three:

```rust
#[test]
fn save_errors_map_to_distinct_codes() {
    let io: IpcError = SaveError::Io("permission denied".into()).into();
    let ser: IpcError = SaveError::Serialize("bad float".into()).into();
    assert_eq!(io.code, "profile-save-io-failed");
    assert_eq!(io.params["detail"], "permission denied");
    assert_eq!(ser.code, "profile-save-failed");
    assert_ne!(io.code, ser.code);
}

#[test]
fn apply_errors_map_to_distinct_codes() {
    let unparsable: IpcError = ApplyError::UnparsableConfigPath("not-a-rule-path".into()).into();
    let oob: IpcError = ApplyError::RuleIndexOutOfRange { index: 7, rules: 1 }.into();
    let noop: IpcError =
        ApplyError::EditChangedNothing { index: 0, property: "forced_track".into() }.into();

    assert_eq!(unparsable.code, "apply-unparsable-config-path");
    assert_eq!(unparsable.params["path"], "not-a-rule-path");
    assert_eq!(oob.code, "apply-rule-index-out-of-range");
    assert_eq!(oob.params["index"], "7");
    assert_eq!(oob.params["rules"], "1");
    assert_eq!(noop.code, "apply-edit-changed-nothing");
    assert_eq!(noop.params["index"], "0");
    assert_eq!(noop.params["property"], "forced_track");

    assert_ne!(unparsable.code, oob.code);
    assert_ne!(oob.code, noop.code);
    assert_ne!(unparsable.code, noop.code);
}
```

Then, in `src-tauri/src/lib.rs`'s test module, beside the existing `validate_profile_body_*` tests (`:554-589`), pin the `load_profile` document shape against `validate_profile`'s output (F5 / `core-85`). Both fixtures already exist in that module and must be **reused, not duplicated** (`testing-support-helpers`):

- The loadable-invalid profile is currently an **inline literal** inside `validate_profile_body_reports_validate_diagnostics_for_a_loadable_invalid_profile` (`lib.rs:569-572`: the zero-rules profile that yields `no-track-rules`). Extract it into a module-level `const LOADABLE_INVALID_PROFILE: &str = ...` and repoint **both** that existing test and the new shape test at it - so the two share one fixture instead of forking it.
- The parse/load-error case is the existing **missing-file** fixture: `validate_profile_body_reports_load_failure_with_no_mkvmerge_key` (`lib.rs:557-563`) passes a path to a file that was never written (`dir.path().join("missing.yaml")`). Reuse that same missing-file shape for the error half rather than inlining a fresh malformed-YAML string.

```rust
// Extracted from lib.rs:569-572 (was inline); the loadable-invalid test above
// is repointed here too.
const LOADABLE_INVALID_PROFILE: &str =
    "profile_version: 1\ninput: { pattern: '.*', extensions: [mkv] }\ntracks:\n  rules: []\n";

#[test]
fn load_profile_body_matches_validate_profile_diagnostics_and_adds_the_model() {
    let dir = tempfile::tempdir().unwrap();

    // A loadable but invalid profile: load_profile's diagnostics envelope is
    // byte-identical to validate_profile's, plus the parsed model under "profile".
    let invalid = dir.path().join("p.yaml");
    std::fs::write(&invalid, LOADABLE_INVALID_PROFILE).unwrap();
    let loaded = load_profile_body(&invalid);
    let validated = validate_profile_body(&invalid);
    assert_eq!(
        loaded["config_diagnostics"], validated["config_diagnostics"],
        "load_profile's config_diagnostics must be shape-identical to validate_profile's (core-85)"
    );
    assert_eq!(loaded["files"], validated["files"], "same envelope, not a second shape");
    assert!(!loaded["profile"].is_null(), "a loadable profile is present under \"profile\"");
    assert!(validated.get("profile").is_none(), "validate_profile carries no model");

    // A load failure (missing file, the same fixture shape as lib.rs:557-563):
    // "profile" is null and the single diagnostic still matches validate's.
    let missing = dir.path().join("missing.yaml"); // never written
    let loaded_missing = load_profile_body(&missing);
    let validated_missing = validate_profile_body(&missing);
    assert_eq!(loaded_missing["config_diagnostics"], validated_missing["config_diagnostics"]);
    assert!(loaded_missing["profile"].is_null(), "a load failure yields profile: null");
}
```

- [ ] **Step 2: Run to confirm they fail**

```bash
cargo test -p muxsmith-gui --lib
```
Expected: FAIL - the `From` impls and `load_profile_body` do not exist. (The package is `muxsmith-gui`, `src-tauri/Cargo.toml:2`; the binary named `muxsmith` is the CLI, `crates/muxsmith-cli/Cargo.toml:9` - do not confuse them.)

- [ ] **Step 3: Implement the mappings and the commands**

Add `From<SaveError>` (mapping `Io` -> `profile-save-io-failed`, `Serialize` -> `profile-save-failed`, each with a `detail` param) and `From<ApplyError>` (verbatim from D49 §"The shell mapping", `:597-616`) to `error.rs`. Then add `load_profile_body`, the four commands, and register each in the `invoke_handler` (`:440-452`). `apply_suggestion` calls core's `planner::apply_suggestion(&profile, &config_path, &edit).map_err(Into::into)`.

- [ ] **Step 4: Add the catalog entries, bilingual**

Five new `IpcError` codes across `locales/{en,de}/gui-common.ftl` (every `IpcError` code in this tree lives there today - `mkvmerge-spawn-failed`, `settings-io-failed`, `internal-task-failed` - so this follows the existing split). The three apply codes are verbatim from D49 §"The catalog entries" (`:627-641`); the two save codes match the register of the existing `{ $detail }` codes:

`locales/en/gui-common.ftl`:
```
profile-save-io-failed = The profile could not be written: { $detail }
profile-save-failed = The profile could not be serialized for saving: { $detail }
apply-unparsable-config-path = The suggestion could not be applied: "{ $path }" does not name a rule.
apply-rule-index-out-of-range = The suggestion could not be applied: no rule at index { $index } (rule count: { $rules }).
apply-edit-changed-nothing = The suggestion changed nothing: rule { $index } already constrains "{ $property }".
```

`locales/de/gui-common.ftl`:
```
profile-save-io-failed = Das Profil konnte nicht geschrieben werden: { $detail }
profile-save-failed = Das Profil konnte für das Speichern nicht serialisiert werden: { $detail }
apply-unparsable-config-path = Der Vorschlag konnte nicht angewendet werden: "{ $path }" benennt keine Regel.
apply-rule-index-out-of-range = Der Vorschlag konnte nicht angewendet werden: keine Regel an Index { $index } (Regelanzahl: { $rules }).
apply-edit-changed-nothing = Der Vorschlag hat nichts geändert: Regel { $index } schränkt "{ $property }" bereits ein.
```

`apply-rule-index-out-of-range` states the count as a labelled value `(rule count: { $rules })` / `(Regelanzahl: { $rules })`, never as a counted noun - D49 `:643-659` records why a Fluent plural selector cannot be used here and must not be added.

- [ ] **Step 5: Run the tests**

```bash
cargo test -p muxsmith-gui
pnpm check:i18n
```
Expected: PASS; `check:i18n` green (check 3 enforces en/de parity on the five new keys).

- [ ] **Step 6: Full gate, then commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/error.rs locales/en/gui-common.ftl locales/de/gui-common.ftl
git -c commit.gpgsign=false commit -m "shell: load/save/validate-model/apply commands and their error codes (D42, D43, D49)"
```

---

## Wave 3

Task 8 merges to master, gate green. Then the frontend, serial within stream E (`.worktrees/plan6-e`): Task 9 -> Task 10 -> Task 11 -> Task 12 -> Task 13. Task 14 is **wave 4**, sequenced after Task 13 (F4: not parallel with 9-13).

