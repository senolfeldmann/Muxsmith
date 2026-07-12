### Task 5: Wire `--list-types` extension validation (#3)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (batch-validation walk, next to `validate_language_values` - locate `fn validate_language_values` and mirror its integration)
- Modify: `crates/muxsmith-core/src/capability/runtime.rs` (consume `list_types()`; it exists and is tested)
- Modify: `crates/muxsmith-core/src/profile/model.rs:73-75` (doc comment currently claims this validation exists - make it true, adjust wording to actual behavior)
- Modify: `crates/muxsmith-core/src/report/mod.rs` (new DiagCode `UnknownExtension`, warning severity), `locales/en/diagnostics.ftl` (message with `$extension` + `$known` params)
- Test: planner batch-validation tests (mirror the language-validation test layout)

**Interfaces:** produces DiagCode::UnknownExtension; T10's param-guard fixture must include it (T10 runs in parallel - its completeness guard will FAIL on a missing fixture, which is the guard working; coordinate via cross-task constraint in the ledger).

- [ ] Step 1: Failing test: profile with `extensions: [mkv, mp4a]` against a runtime whose list-types yields mkv/mp4/avi...; assert one `UnknownExtension` warning for `mp4a` naming the extension, batch continues.
- [ ] Step 2: Implement: once per batch (not per file), degrade-with-warning when mkvmerge is absent - same pattern as language validation (walkthrough #3 decided consistency with the existing walk).
- [ ] Step 3: Fix the model.rs doc comment to describe the now-real behavior. Full gate; commit `feat(planner): validate profile extensions against mkvmerge --list-types (#3)`.

