# Task 9 Report: Input, Locator, Keyword and Template Validation

## Adaptations applied (per task instructions)

1. **Block newtype variants, not inline struct variants.** `SourceCfg::External(block)` with `block.external`, `FilenameCfg::Template(block)` / `TitleCfg::Template(block)` with `block.template`, `ChaptersCfg::External(block)` with `block.external` — matching the actual `model.rs` shapes (`ExternalBlock`, `TemplateBlock`), not the brief's illustrative struct-variant syntax.
2. **Flattened regex error text.** Added `flatten_regex_error(e: &regex::Error) -> String` (`e.to_string().split_whitespace().collect::<Vec<_>>().join(" ")`), used both at the new `input.pattern` site and at the pre-existing Task 8 condition-regex site (`validate_expr`'s `regex` kind), replacing the old raw `e.to_string()`.
3. **Threaded `&[String]` template_fields.** `validate_attachment_rule` and `validate_locator` take `template_fields: &[String]`; `validate_expr` signature is untouched (doesn't need it), per the task's explicit note.
4. **Dropped the brief's unused `_reserved: bool` parameter** from `validate_locator` — every call site passed `false` and nothing read it; kept as a 4-arg function instead of carrying dead placeholder state.

## TDD Evidence

### Step 1-2: Tests written, verified failing
Created `crates/muxsmith-core/tests/validate_structure.rs` (brief's 10 tests verbatim). Initial run against the untouched Task 8 `validate.rs`:

```
test result: FAILED. 1 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out

9 of 10 failed as expected (`invalid_input_pattern_is_flagged`, `empty_extensions_flagged_for_input_and_locator`, `locator_with_both_match_options_is_conflict`, `match_pattern_with_unknown_field_is_flagged`, `match_pattern_may_not_use_source_stem`, `filename_template_fields_checked_against_pattern_groups`, `filename_template_with_path_separator_is_flagged`, `bad_template_syntax_is_invalid_template`, `unknown_keywords_are_flagged`); `numbered_group_fields_are_accepted` passed incidentally (no UnknownTemplateField ever fired pre-Task-9).

### Step 3: Implementation
Extended `crates/muxsmith-core/src/profile/validate.rs`:
- `input.pattern` regex compile + capture-group -> `template_fields` derivation (`match`, `g1..gN`, named groups).
- `input.extensions` `EmptyExtensions` check.
- Per-track `source` keyword (`primary` only) / external-locator validation.
- `output.filename`, `chapters`, `title` keyword + template/locator validation, with `source_stem` added to `output.filename.template` and `title.template` allowed fields (title is literal mode per spec 4.7/4.9), and the path-separator ban applied to `output.filename.template` only.
- `validate_attachment_rule`'s `rule.add` now runs full locator validation.
- New helpers `validate_locator` (extensions/XOR/match_pattern) and `validate_template` (parse errors -> `InvalidTemplate`/`UnknownTemplateFilter`, unknown fields, path-separator ban).

### Step 4: Focused test run (real output)
```
$ cargo test -p muxsmith-core --test validate_structure --test validate_semantics
     Running tests/validate_semantics.rs (target/debug/deps/validate_semantics-ccce555145043fa5)
running 14 tests
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/validate_structure.rs (target/debug/deps/validate_structure-c4930cb1d57bed76)
running 10 tests
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Full workspace run (real output)
```
$ cargo test --workspace
unittests muxsmith (main.rs):        0 passed; 0 failed
unittests muxsmith_core (lib.rs):    26 passed; 0 failed
tests/profile_load.rs:                6 passed; 0 failed
tests/validate_semantics.rs:         14 passed; 0 failed
tests/validate_structure.rs:         10 passed; 0 failed
unittests xtask (lib.rs):             0 passed; 0 failed
unittests xtask (main.rs):            0 passed; 0 failed
tests/codegen.rs:                     2 passed; 0 failed
Doc-tests muxsmith_core:              0 passed; 0 failed
Doc-tests xtask:                      0 passed; 0 failed
```
58/58 tests pass, 0 failures.

### Step 5: Commit
Committed from `/home/senol/Git/Muxsmith`; see commit hash in repo log (`feat(core): input, locator, keyword and template validation`).

## Self-Review

- Verified `DiagCode` already carried every variant this task needs (`InvalidRegex`, `EmptyExtensions`, `InvalidKeyword`, `LocatorConflict`, `InvalidTemplate`, `UnknownTemplateField`, `UnknownTemplateFilter`, `PathSeparatorInTemplate`) — no `report.rs` change required.
- Confirmed against spec 4.6/4.7/4.9: `match_to_source`/`match_pattern` are mutually exclusive (no "at least one required" rule, so only the XOR conflict is checked, matching the brief); `source_stem` is literal-mode only, so it's added for `output.filename.template` and `title.template` but never for `match_pattern` (regex mode) fields; the path-separator ban applies to `output.filename.template` only, not `title.template` (chapters has no template variant at all — only `keyword | external`).
- `regex::Error`'s multi-line caret-art Display was the reason for flattening `detail`; checked both call sites (`input.pattern`, condition-regex) now flatten and confirmed pre-existing `validate_semantics.rs` tests (which only `contains()` on codes) are unaffected.
- Ran `cargo clippy -p muxsmith-core --all-targets`: one `collapsible_if` warning at the condition-regex site — confirmed via `git stash`/reclippy that this warning predates Task 9 (Task 8 code, untouched structurally), out of scope for this task.
- Ran `cargo fmt` on `validate.rs`/`validate_structure.rs` only; reverted an incidental repo-wide `cargo fmt` pass on unrelated pre-existing files (toolchain rustfmt-version drift, not part of this task's diff).
- `numbered_group_fields_are_accepted` exercises the unnamed-capture-group path (`g1`, `g2` with no named groups) alongside the named-group path exercised by every other test using `BASE`.

## Concerns

None blocking. Reference profile (`tests/fixtures/reference.yaml`) still validates with zero errors post-extension (its external locator uses `match_to_source: true` with no `match_pattern`, non-empty extensions, and `filename: keep` / `chapters: keep` / `title: clear`, so none of the new checks fire against it).

## Files Modified
- Modified: `crates/muxsmith-core/src/profile/validate.rs`
- Created: `crates/muxsmith-core/tests/validate_structure.rs`
