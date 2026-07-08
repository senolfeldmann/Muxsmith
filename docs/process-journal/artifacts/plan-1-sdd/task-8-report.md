# Task 8 Report: Semantic Validation of Match Expressions and Changes

## TDD Evidence

### Step 1-2: Tests Written and Verified Failing
Created `crates/muxsmith-core/tests/validate_semantics.rs` with 14 test cases covering:
- Profile version validation
- Empty tracks rejection
- Track rule validation (match expressions and changes)
- Attachment rule validation
- Property type checking (exact, substring, regex)
- Type coercion rules (Integer→Float ok, Float→Integer not ok)
- Regex validation
- Recursive validation of nested any/not expressions

**Initial run failed as expected:**
```
error[E0432]: unresolved import `muxsmith_core::profile::validate`
```

### Step 3: Implementation Complete
Created `crates/muxsmith-core/src/profile/validate.rs` with:
- `validate()` public function as the entry point
- `validate_expr()` for recursive match expression validation
- `validate_changes()` for BTreeMap<String, Scalar> property validation
- `validate_attachment_rule()` for attachment-specific rule shape
- `scalar_fits()` type checking with proper Int→Float coercion
- Proper config path construction (e.g., `tracks[0].match.exact.language`)
- Support for three action types in attachment rules (select, drop, add)

Modified `crates/muxsmith-core/src/profile/mod.rs` to export the new module.

### Step 4: Test Pass Verification
**Focused test (validate_semantics):**
```
running 14 tests
test empty_match_expression_is_warning ... ok
test empty_tracks_list_is_rejected ... ok
test change_value_type_mismatch_is_flagged ... ok
test exact_value_type_mismatch_is_flagged ... ok
test nested_any_and_not_are_validated_recursively ... ok
test integer_accepted_for_float_property_but_not_reverse ... ok
test attachment_match_uses_attachment_property_set ... ok
test attachment_rule_must_have_exactly_one_action ... ok
test substring_on_boolean_property_is_flagged ... ok
test unknown_match_property_is_flagged_with_path ... ok
test unknown_change_property_is_flagged ... ok
test wrong_profile_version_is_rejected ... ok
test invalid_condition_regex_is_flagged ... ok
test reference_profile_validates_clean ... ok

test result: ok. 14 passed; 0 failed
```

**Full workspace test:** All 30 tests pass (14 semantic + 6 load + 8 existing + 2 codegen).

### Step 5: Commit
Commit hash: `6827894`
All files staged and committed with co-author trailer.

## Self-Review

**Coverage:**
- ✓ All 14 test cases pass
- ✓ Reference profile validates clean (no errors)
- ✓ Path construction tested with `tracks[0].match.exact.colour_depth`, `tracks[0].match.any[0].exact.nonexistent_prop`
- ✓ Attachment rules tested with select/drop conflict detection

**Implementation Quality:**
- ✓ Type checking: Integer accepts Float types, Float rejects Integer types (correct)
- ✓ Regex validation: uses `regex::Regex::new()` crate dependency (already available)
- ✓ Attachment property set: uses `capability::ATTACHMENT_PROPERTIES` iterator
- ✓ Track properties: uses `capability::matchable_type()` lookup
- ✓ Settable properties: uses `capability::settable()` for changes validation
- ✓ Recursive validation: `any` and `not` arrays properly traversed

**Semantic Correctness:**
- ✓ Empty tracks flagged as error (DiagCode::NoTrackRules)
- ✓ Empty match expressions flagged as warning (DiagCode::EmptyMatchExpression)
- ✓ Attachment rule shape enforces exactly 1 action (select XOR drop XOR add)
- ✓ Attachment properties validated against ATTACHMENT_PROPERTIES, not track properties
- ✓ Config paths include array indices and object keys for precise error location

**Notes:**
- Task 9 will extend this file with input/locator/template validation (marked with comment)
- `rule.add` in attachment rules is not validated (locator validation deferred to Task 9)
- No integration with CLI yet; Task 12 will wire this into the command handler

## Concerns
None. All tests pass; implementation matches brief exactly.

## Files Modified
- Created: `crates/muxsmith-core/src/profile/validate.rs` (177 lines)
- Created: `crates/muxsmith-core/tests/validate_semantics.rs` (139 lines)
- Modified: `crates/muxsmith-core/src/profile/mod.rs` (added `pub mod validate;`)
