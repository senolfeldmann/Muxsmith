### Task 10: Catalog param-drift guard + full-key coverage (#16)

**Files:**
- Modify: `crates/muxsmith-cli/tests/catalog_completeness.rs:8-16`
- Create: param fixture table in the same test file

- [ ] Step 1: Failing state by construction: build a `fn fixture_args(code: DiagCode) -> FluentArgs` table covering every DiagCode with the params its emitter actually sets (source: grep each `Diag::new`/emitter site); the test renders every message with its fixture and asserts the output contains no `{$` substring. Missing table entry = compile error via exhaustive match (the guard grows with the enum).
- [ ] Step 2: Second test: enumerate ALL keys in `locales/en/cli.ftl` (parse the ftl at test time) and assert each key is either a DiagCode message, in an explicit allowlist of directly-rendered keys (the 8 `run-*` keys and future ones - the allowlist entries each render with their fixture too), or the test fails naming the orphan key.
- [ ] Step 3: Full gate; commit `test(i18n): param-drift guard + full cli.ftl key coverage (#16)`.

