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
