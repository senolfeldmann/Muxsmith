use xtask::codegen::generate;

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
