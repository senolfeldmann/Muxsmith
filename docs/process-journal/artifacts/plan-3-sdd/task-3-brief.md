### Task 3: attachment matching (`Attachment: Matchable`)

**Files:**
- Modify: `crates/muxsmith-core/src/matcher.rs` (impl + tests) and `crates/muxsmith-core/src/identify.rs` (nothing new; `Attachment::get` from Task 1).

**Interfaces:**
- Consumes: `Matchable` (Task 2), `Attachment::get` (Task 1).
- Produces: `impl Matchable for Attachment`, so `matcher::matches(expr, attachment, lang)` evaluates attachment rules with the same algebra (spec 4.9).

- [ ] **Step 1: Write the failing test.**

```rust
#[test]
fn attachment_matching_uses_the_same_algebra() {
    use crate::identify::Attachment;
    let font = Attachment {
        id: 1, file_name: "Roboto.ttf".into(), size: 100,
        content_type: Some("font/ttf".into()), description: None, uid: None,
    };
    assert!(matches(&expr("substring: { file_name: robot }"), &font, &lang()));
    assert!(matches(&expr("exact: { content_type: font/ttf }"), &font, &lang()));
    assert!(matches(
        &expr("any:\n  - substring: { file_name: .ttf }\n  - substring: { file_name: .otf }"),
        &font, &lang()));
    assert!(!matches(&expr("exact: { description: whatever }"), &font, &lang()));
    assert!(!matches(&expr("substring: { content_type: pdf }"), &font, &lang()));
}
```

- [ ] **Step 2: Run, verify fail.** `cargo test -p muxsmith-core attachment_matching` -> FAIL (`Attachment` does not implement `Matchable`).
- [ ] **Step 3: Implement.** `impl Matchable for Attachment { fn get(&self, prop: &str) -> Option<PropValue> { Attachment::get(self, prop) } }`.
- [ ] **Step 4: Run, verify pass.** `cargo test -p muxsmith-core attachment_matching` -> PASS.
- [ ] **Step 5: Gate + commit.** `feat(matcher): match attachment rules via Matchable`.

---

