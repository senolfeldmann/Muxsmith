### Task 10: known_extensions required method (Wave 2, seed T5-m2)

**Files:**
- Modify: `crates/muxsmith-core/src/identify.rs:383`, `crates/muxsmith-core/tests/support/mod.rs:20` (FakeIdent), `crates/muxsmith-cli/tests/catalog_completeness.rs:467` (OneIdent)

- [ ] Make Identify::known_extensions a required method (declaration only, drop the `{ None }` default body - a capability stub whose absence silently no-ops production validation must not default); add explicit `fn known_extensions(&mut self) -> Option<Vec<String>> { None }` to FakeIdent and OneIdent; trim the stale doc sentence "Defaulted here so existing Identify fakes need no change to keep compiling." LiveIdentifier and FakeIdentWithExtensions already override.
- [ ] Full gate; commit `refactor(core): known_extensions is a required Identify method (T5-m2)`.

