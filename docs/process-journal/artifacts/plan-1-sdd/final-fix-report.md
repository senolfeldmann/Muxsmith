# Final-review fixes: gate output

## cargo fmt --all --check
(no output; clean)

## cargo clippy --workspace --all-targets -- -D warnings
```
    Checking muxsmith-core v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-core)
    Checking muxsmith-cli v0.1.0 (/home/senol/Git/Muxsmith/crates/muxsmith-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
```

## cargo test --workspace (tail, verbatim)
```
     Running unittests src/lib.rs (target/debug/deps/muxsmith_core-8f44f4eef6de7678)

running 34 tests
test capability::tests::attachment_properties_are_defined ... ok
test capability::tests::codec_kind_is_virtual_matchable ... ok
test capability::tests::matchable_types_from_generated_table ... ok
test capability::tests::settable_maps_to_mkvmerge_options ... ok
test profile::match_expr::tests::empty_expression_reports_empty ... ok
test profile::lint::tests::rules_with_any_are_skipped ... ok
test profile::lint::tests::external_source_rules_are_skipped ... ok
test profile::lint::tests::disjoint_exact_values_are_not_flagged ... ok
test profile::lint::tests::rules_with_substring_are_skipped ... ok
test profile::lint::tests::rules_with_negations_or_regex_are_skipped ... ok
test profile::lint::tests::identical_exact_rules_are_provable_overlap ... ok
test profile::lint::tests::reversed_direction_overlap_is_flagged ... ok
test profile::lint::tests::subset_conditions_are_provable_overlap ... ok
test profile::match_expr::tests::scalar_type_names ... ok
test profile::match_expr::tests::rejects_unknown_keys ... ok
test profile::match_expr::tests::parses_flat_expression ... ok
test report::tests::all_keys_match_serde_encoding ... ok
test report::tests::all_keys_are_unique ... ok
test profile::match_expr::tests::parses_nested_any_and_not ... ok
test report::tests::diagnostic_builder_sets_fields ... ok
test report::tests::diag_code_keys_are_kebab_case ... ok
test report::tests::severity_orders_info_warning_error ... ok
test report::tests::diagnostic_serializes_with_snake_case_severity_and_kebab_code ... ok
test report::tests::worst_severity_picks_error_over_warning ... ok
test template::tests::case_sensitive_flag_controls_inline_i ... ok
test template::tests::double_braces_are_literal ... ok
test template::tests::empty_name_with_filter_is_empty_field ... ok
test template::tests::int_filter_keeps_single_zero ... ok
test template::tests::int_filter_on_missing_field_renders_empty ... ok
test template::tests::parses_fields_and_filters ... ok
test template::tests::rejects_unknown_filter_and_unclosed_brace ... ok
test template::tests::renders_literal_with_filters ... ok
test template::tests::regex_mode_escapes_interpolated_values ... ok
test template::tests::regex_mode_matches_spec_examples ... ok

test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/profile_load.rs (target/debug/deps/profile_load-68c41be2ddd6b29e)

running 6 tests
test unknown_key_inside_filename_template_is_rejected ... ok
test unknown_key_is_parse_error_with_path ... ok
test defaults_apply_when_sections_absent ... ok
test unknown_key_inside_source_external_is_rejected ... ok
test json_profile_parses_identically_to_yaml ... ok
test reference_profile_parses ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/validate_semantics.rs (target/debug/deps/validate_semantics-8890190446697264)

running 14 tests
test attachment_match_uses_attachment_property_set ... ok
test empty_tracks_list_is_rejected ... ok
test attachment_rule_must_have_exactly_one_action ... ok
test empty_match_expression_is_warning ... ok
test exact_value_type_mismatch_is_flagged ... ok
test change_value_type_mismatch_is_flagged ... ok
test nested_any_and_not_are_validated_recursively ... ok
test substring_on_boolean_property_is_flagged ... ok
test invalid_condition_regex_is_flagged ... ok
test unknown_change_property_is_flagged ... ok
test wrong_profile_version_is_rejected ... ok
test unknown_match_property_is_flagged_with_path ... ok
test reference_profile_validates_clean ... ok
test integer_accepted_for_float_property_but_not_reverse ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/validate_structure.rs (target/debug/deps/validate_structure-e2abfd407894ca68)

running 15 tests
test invalid_input_pattern_is_flagged ... ok
test empty_extensions_flagged_for_input_and_locator ... ok
test match_pattern_may_not_use_source_stem ... ok
test locator_with_both_match_options_is_conflict ... ok
test match_to_source_false_with_pattern_is_not_conflict ... ok
test filename_template_with_path_separator_is_flagged ... ok
test unknown_template_filter_carries_name ... ok
test match_pattern_with_unknown_field_is_flagged ... ok
test numbered_group_fields_are_accepted ... ok
test match_to_source_false_is_rejected ... ok
test empty_locator_extensions_flagged ... ok
test filename_keyword_misuse_flagged ... ok
test bad_template_syntax_is_invalid_template ... ok
test filename_template_fields_checked_against_pattern_groups ... ok
test unknown_keywords_are_flagged ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/lib.rs (target/debug/deps/xtask-bd271c62cee407fb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/xtask-68ecf8efda7363d1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/codegen.rs (target/debug/deps/codegen-81943094a8bcf0d4)

running 2 tests
test rejects_schema_without_track_properties ... ok
test generates_matchable_table_from_schema ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests muxsmith_cli

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests muxsmith_core

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests xtask

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Total: 81 tests passed across all suites (76 baseline + 5 new: `unknown_template_filter_carries_name`,
`match_to_source_false_is_rejected`, `match_to_source_false_with_pattern_is_not_conflict`,
`empty_locator_extensions_flagged`, `filename_keyword_misuse_flagged`).
