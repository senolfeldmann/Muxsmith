### Task 2: D41 - the core profile writer

**Files:**
- Create: `crates/muxsmith-core/src/profile/save.rs`
- Modify: `crates/muxsmith-core/src/profile/mod.rs` (module doc sentence + `pub mod save;`)
- Test: `crates/muxsmith-core/tests/profile_save.rs` (new)

**Interfaces:**
- Consumes: `profile::load::Format` and `profile::model::Profile` (both exist).
- Produces, for Tasks 4 and 8:
  - `muxsmith_core::profile::save::to_string(profile: &Profile, format: Format) -> Result<String, SaveError>`
  - `muxsmith_core::profile::save::to_file(profile: &Profile, path: &Path) -> Result<(), SaveError>`
  - `muxsmith_core::profile::save::SaveError` with variants `Io(String)` and `Serialize(String)`

**Read first:** design D41 (`:122-300`), for the decision and its rejected alternatives, **as amended by Task 1**.

Binding points, because each is a place an implementer would otherwise improvise:
- The writer lives in **core**, not in `src-tauri` and not in the CLI (`core-85-report-json-dry`: neither surface owns document logic).
- `to_file` picks `Format` from the path extension **exactly as `load::from_file` does** (`profile/load.rs:57-62`): `Some("json")` -> `Format::Json`, everything else -> `Format::Yaml`. A `.json` profile saves as JSON and never silently changes format.
- `yaml_serde::to_string` and `serde_json::to_string_pretty` are the writers; no new dependency.
- `SaveError`, not `Diagnostic` (Global Constraints ruling 1).

- [ ] **Step 1: Write the failing round-trip test**

Create `crates/muxsmith-core/tests/profile_save.rs`:

```rust
//! Spec 8.2 / D41: saving writes canonical YAML from the model. These tests
//! pin the correctness floor (a saved profile reloads equal) and the
//! format-from-extension contract; the default-omission behaviour is D48's
//! and is tested in its own guards.

use std::path::Path;

use muxsmith_core::profile::load::{Format, from_file, from_str};
use muxsmith_core::profile::save::{SaveError, to_file, to_string};

const REFERENCE: &str = include_str!("fixtures/reference.yaml");

#[test]
fn canonical_yaml_round_trips_to_an_equal_model() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Yaml).expect("model serializes");
    let p2 = from_str(&text, Format::Yaml).expect("canonical output re-parses");
    assert_eq!(p, p2, "a saved profile must reload to an equal model (D41)");
}

#[test]
fn canonical_json_round_trips_to_an_equal_model() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let text = to_string(&p, Format::Json).expect("model serializes");
    let p2 = from_str(&text, Format::Json).expect("canonical JSON re-parses");
    assert_eq!(p, p2);
}

#[test]
fn to_file_picks_json_from_the_extension_and_never_changes_format() {
    let dir = tempfile::tempdir().unwrap();
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");

    let json_path = dir.path().join("profile.json");
    to_file(&p, &json_path).expect("writes");
    let text = std::fs::read_to_string(&json_path).unwrap();
    assert!(
        text.trim_start().starts_with('{'),
        "a .json path must save as JSON, not YAML: {text}"
    );
    assert_eq!(from_file(&json_path).unwrap(), p);

    let yaml_path = dir.path().join("profile.yaml");
    to_file(&p, &yaml_path).expect("writes");
    let text = std::fs::read_to_string(&yaml_path).unwrap();
    assert!(
        !text.trim_start().starts_with('{'),
        "a .yaml path must save as YAML: {text}"
    );
    assert_eq!(from_file(&yaml_path).unwrap(), p);
}

#[test]
fn an_unwritable_path_is_an_io_error_not_a_panic() {
    let p = from_str(REFERENCE, Format::Yaml).expect("fixture parses");
    let dir = tempfile::tempdir().unwrap();
    let nope = dir.path().join("no-such-dir").join("profile.yaml");
    match to_file(&p, &nope) {
        Err(SaveError::Io(detail)) => assert!(!detail.is_empty(), "detail carries the io message"),
        other => panic!("expected SaveError::Io, got {other:?}"),
    }
}
```

- [ ] **Step 2: Run it to confirm it fails**

```bash
cargo test -p muxsmith-core --test profile_save
```
Expected: FAIL - `unresolved import muxsmith_core::profile::save`.

- [ ] **Step 3: Write the module**

Create `crates/muxsmith-core/src/profile/save.rs`. `SaveError` derives `Debug` (the test's `{other:?}` needs it) and `PartialEq`; it is a plain enum, not a `std::error::Error`, matching `SettingsError`'s shape in the shell. Give every public item rustdoc - `#![deny(missing_docs)]` is on in core. The two `to_string` arms are `yaml_serde::to_string(profile)` and `serde_json::to_string_pretty(profile)`, each mapping its error into `SaveError::Serialize(e.to_string())`; `to_file` selects the format from the extension exactly as `load::from_file:57-62` does, then `fs::write`, mapping into `SaveError::Io(e.to_string())`.

- [ ] **Step 4: Wire the module in**

In `crates/muxsmith-core/src/profile/mod.rs`, add `pub mod save;` in alphabetical position (after `pub mod model;`), and extend the module doc's sentence so `save` is named beside `load` - it currently reads "[`load`] parses YAML/JSON into the serde model".

- [ ] **Step 5: Add the dev-dependency if absent**

The test uses `tempfile`. Check first, and only add it if it is missing:

```bash
grep -n "tempfile" crates/muxsmith-core/Cargo.toml
```
If absent, add it under `[dev-dependencies]` with the exact version already used elsewhere in the workspace (`grep -rn "tempfile" crates/*/Cargo.toml`) - do not invent a version. (At `0cc15d7`, `tempfile = "3.27.0"` is already present, so this is a no-op; the check is here so a re-cut does not assume it.)

- [ ] **Step 6: Run the tests**

```bash
cargo test -p muxsmith-core --test profile_save
```
Expected: PASS, 4 tests.

- [ ] **Step 7: Full gate, then commit**

Run the nine-part gate. Then:

```bash
git add crates/muxsmith-core/src/profile/save.rs crates/muxsmith-core/src/profile/mod.rs crates/muxsmith-core/tests/profile_save.rs crates/muxsmith-core/Cargo.toml
git -c commit.gpgsign=false commit -m "core: profile::save writes canonical YAML/JSON from the model (D41)"
```

---

