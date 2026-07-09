### Task 1: `identify` parses attachments and chapters

**Files:**
- Modify: `crates/muxsmith-core/src/identify.rs`
- Test: same file's `#[cfg(test)] mod tests` (add cases) plus `crates/muxsmith-core/tests/identify_live.rs` if a live case fits; unit tests suffice here.

**Interfaces:**
- Consumes: existing `PropValue::from_json`, `Identification::from_json` (serde_json `Value`).
- Produces: `pub struct Attachment { pub id: u64, pub file_name: String, pub size: u64, pub content_type: Option<String>, pub description: Option<String>, pub uid: Option<u64> }` with `pub fn get(&self, name: &str) -> Option<PropValue>`; `Identification` gains `pub attachments: Vec<Attachment>` and `pub chapters: u64` (total entry count, summed over the `-J` `chapters` array's `num_entries`).

- [ ] **Step 1: Write failing tests.** In `identify.rs` tests, add fixture JSON and assertions:

```rust
#[test]
fn parses_attachments_with_optional_fields() {
    let json = r#"{
      "file_name": "e.mkv",
      "identification_format_version": 20,
      "container": { "recognized": true, "supported": true },
      "tracks": [],
      "attachments": [
        { "id": 1, "file_name": "font.ttf", "size": 1234,
          "content_type": "application/x-truetype-font",
          "description": "Main font", "properties": { "uid": 99 } },
        { "id": 2, "file_name": "cover.jpg", "size": 5678, "properties": {} }
      ],
      "chapters": [ { "num_entries": 12 } ]
    }"#;
    let id = Identification::from_json(json).unwrap();
    assert_eq!(id.attachments.len(), 2);
    let a = &id.attachments[0];
    assert_eq!(a.id, 1);
    assert_eq!(a.file_name, "font.ttf");
    assert_eq!(a.size, 1234);
    assert_eq!(a.content_type.as_deref(), Some("application/x-truetype-font"));
    assert_eq!(a.description.as_deref(), Some("Main font"));
    assert_eq!(a.uid, Some(99));
    assert_eq!(id.attachments[1].content_type, None);
    assert_eq!(id.attachments[1].description, None);
    assert_eq!(id.attachments[1].uid, None);
    assert_eq!(id.chapters, 12);
}

#[test]
fn absent_attachments_and_chapters_default_empty() {
    let json = r#"{ "file_name": "e.mkv", "identification_format_version": 20,
      "container": { "recognized": true, "supported": true }, "tracks": [] }"#;
    let id = Identification::from_json(json).unwrap();
    assert!(id.attachments.is_empty());
    assert_eq!(id.chapters, 0);
}

#[test]
fn attachment_get_exposes_match_properties() {
    let json = r#"{ "file_name": "e.mkv", "identification_format_version": 20,
      "container": { "recognized": true, "supported": true }, "tracks": [],
      "attachments": [ { "id": 3, "file_name": "f.otf", "size": 10,
        "content_type": "font/otf", "properties": {} } ] }"#;
    let a = &Identification::from_json(json).unwrap().attachments[0];
    assert_eq!(a.get("file_name"), Some(PropValue::Str("f.otf".into())));
    assert_eq!(a.get("content_type"), Some(PropValue::Str("font/otf".into())));
    assert_eq!(a.get("description"), None);
    assert_eq!(a.get("id"), Some(PropValue::Int(3)));
    assert_eq!(a.get("size"), Some(PropValue::Int(10)));
    assert_eq!(a.get("nope"), None);
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core identify` -> FAIL (`Attachment` undefined, no `attachments`/`chapters` field).

- [ ] **Step 3: Implement.** Add the `Attachment` struct with a doc comment on each field, a `parse_attachment(v: &Value) -> Option<Attachment>` mirroring `parse_track` (required: `id`, `file_name`, `size`; optional: `content_type`, `description`; `uid` nested under `properties.uid`), the `get` method (map `file_name`/`content_type`/`description` -> `Str`, `id`/`size` -> `Int`, else `None`), add `attachments` and `chapters` to `Identification` and to `from_json`:

```rust
let attachments = v.get("attachments").and_then(Value::as_array)
    .map(|arr| arr.iter().filter_map(parse_attachment).collect())
    .unwrap_or_default();
let chapters = v.get("chapters").and_then(Value::as_array)
    .map(|arr| arr.iter()
        .filter_map(|c| c.get("num_entries").and_then(Value::as_u64))
        .sum())
    .unwrap_or(0);
```

`parse_attachment`:

```rust
fn parse_attachment(v: &Value) -> Option<Attachment> {
    let id = v.get("id").and_then(Value::as_u64)?;
    let file_name = v.get("file_name").and_then(Value::as_str)?.to_string();
    let size = v.get("size").and_then(Value::as_u64)?;
    let content_type = v.get("content_type").and_then(Value::as_str).map(str::to_string);
    let description = v.get("description").and_then(Value::as_str).map(str::to_string);
    let uid = v.get("properties").and_then(|p| p.get("uid")).and_then(Value::as_u64);
    Some(Attachment { id, file_name, size, content_type, description, uid })
}
```

Update every `Identification { ... }` literal in the codebase (tests, fixtures) to include the two new fields; `cargo build` will point them out.

- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core identify` -> PASS.
- [ ] **Step 5: Gate + commit.** Run the full gate. `git -c commit.gpgsign=false commit` with message `feat(identify): parse attachments and chapters from -J`.

---

