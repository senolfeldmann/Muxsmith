### Task 5: Capability code generator (xtask)

**Files:**
- Create: `crates/xtask/Cargo.toml`, `crates/xtask/src/main.rs`, `crates/xtask/src/gen.rs`
- Create: `crates/xtask/tests/fixtures/mini-schema.json`
- Modify: `Cargo.toml` (workspace members)
- Test: `crates/xtask/tests/gen.rs`

**Interfaces:**
- Consumes: nothing.
- Produces: `cargo run -p xtask -- gen-capability <schema.json> <out.rs>` writing a Rust source file containing `pub static MATCHABLE_PROPERTIES: &[(&str, PropType)]`. Task 6 commits its output as `crates/muxsmith-core/src/capability/generated.rs`.
- The upstream schema file is an INPUT only. It is never committed and never shipped (spec 9 / decision log).

- [ ] **Step 1: Scaffold the xtask crate**

Add `"crates/xtask"` to workspace `members` in the root `Cargo.toml`.

`crates/xtask/Cargo.toml`:

```toml
[package]
name = "xtask"
version.workspace = true
edition.workspace = true
license.workspace = true
publish = false

[dependencies]
serde_json = "1"
```

- [ ] **Step 2: Create the mini-schema fixture**

`crates/xtask/tests/fixtures/mini-schema.json` (synthetic, mirrors the upstream structure without copying it):

```json
{
  "title": "synthetic mini schema for generator tests",
  "type": "object",
  "properties": {
    "tracks": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "codec": { "type": "string" },
          "id": { "type": "integer" },
          "type": { "type": "string" },
          "properties": {
            "type": "object",
            "properties": {
              "audio_channels": { "type": "integer" },
              "default_track": { "type": "boolean" },
              "display_dimensions": { "type": "string" },
              "forced_track": { "type": "boolean" },
              "language": { "type": "string" },
              "track_name": { "type": "string" }
            }
          }
        }
      }
    }
  }
}
```

- [ ] **Step 3: Write the failing test**

`crates/xtask/tests/gen.rs`:

```rust
use xtask::gen::generate;

#[test]
fn generates_matchable_table_from_schema() {
    let schema = include_str!("fixtures/mini-schema.json");
    let out = generate(schema).unwrap();
    // Track-level fields injected by the generator:
    assert!(out.contains(r#"("type", PropType::String)"#));
    assert!(out.contains(r#"("codec", PropType::String)"#));
    assert!(out.contains(r#"("id", PropType::Integer)"#));
    // Properties from the schema:
    assert!(out.contains(r#"("audio_channels", PropType::Integer)"#));
    assert!(out.contains(r#"("default_track", PropType::Boolean)"#));
    assert!(out.contains(r#"("language", PropType::String)"#));
    // Header marker so humans know not to edit:
    assert!(out.contains("GENERATED FILE"));
}

#[test]
fn rejects_schema_without_track_properties() {
    assert!(generate("{}").is_err());
}
```

Make xtask a lib+bin so the test can import it: add to `crates/xtask/Cargo.toml`:

```toml
[lib]
path = "src/lib.rs"
```

and create `crates/xtask/src/lib.rs`:

```rust
pub mod gen;
```

- [ ] **Step 4: Run test to verify it fails**

Run: `cargo test -p xtask`
Expected: FAIL (`gen` not defined)

- [ ] **Step 5: Implement the generator**

`crates/xtask/src/gen.rs`:

```rust
use serde_json::Value;

/// Extract matchable track property names and types from the mkvmerge
/// identification output schema. Only derived FACTS are emitted; the
/// schema text itself is never redistributed (spec 9).
pub fn generate(schema_json: &str) -> Result<String, String> {
    let schema: Value =
        serde_json::from_str(schema_json).map_err(|e| format!("invalid JSON: {e}"))?;

    let track_props = schema
        .pointer("/properties/tracks/items/properties/properties/properties")
        .and_then(Value::as_object)
        .ok_or("schema has no tracks.items.properties.properties.properties object")?;

    let mut entries: Vec<(String, &'static str)> = vec![
        // Track-level fields outside the nested properties object.
        ("type".into(), "String"),
        ("codec".into(), "String"),
        ("id".into(), "Integer"),
    ];

    for (name, def) in track_props {
        let prop_type = match def.get("type").and_then(Value::as_str) {
            Some("boolean") => "Boolean",
            Some("integer") => "Integer",
            Some("number") => "Float",
            // Strings, unions and anything exotic degrade to String:
            // matching still works, only exact-type checks get looser.
            _ => "String",
        };
        entries.push((name.clone(), prop_type));
    }
    entries.sort();
    entries.dedup_by(|a, b| a.0 == b.0);

    let mut out = String::new();
    out.push_str("// GENERATED FILE - do not edit.\n");
    out.push_str("// Regenerate: cargo run -p xtask -- gen-capability <schema.json> <this file>\n");
    out.push_str("// Source: mkvmerge identification output schema (facts only, not the schema).\n\n");
    out.push_str("use super::PropType;\n\n");
    out.push_str("pub static MATCHABLE_PROPERTIES: &[(&str, PropType)] = &[\n");
    for (name, ty) in &entries {
        out.push_str(&format!("    (\"{name}\", PropType::{ty}),\n"));
    }
    out.push_str("];\n");
    Ok(out)
}
```

`crates/xtask/src/main.rs`:

```rust
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("gen-capability") if args.len() == 4 => {
            let schema = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
                eprintln!("cannot read {}: {e}", args[2]);
                process::exit(1);
            });
            let code = xtask::gen::generate(&schema).unwrap_or_else(|e| {
                eprintln!("generation failed: {e}");
                process::exit(1);
            });
            fs::write(&args[3], code).unwrap_or_else(|e| {
                eprintln!("cannot write {}: {e}", args[3]);
                process::exit(1);
            });
            eprintln!("wrote {}", args[3]);
        }
        _ => {
            eprintln!("usage: cargo run -p xtask -- gen-capability <schema.json> <out.rs>");
            process::exit(2);
        }
    }
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test -p xtask`
Expected: PASS (2 tests)

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(xtask): capability table generator from identification schema"
```

---

