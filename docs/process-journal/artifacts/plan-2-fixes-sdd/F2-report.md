# F2: add two diagnostic codes - report

## What changed

`crates/muxsmith-core/src/report.rs`:
- Added `DiagCode::EmptyMatchList => "empty-match-list"` to the `diag_codes!` macro
  invocation, placed directly after `InvalidPropertyValue` in the config-time
  block, with rustdoc stating the condition (present-but-empty `any`/`not` list,
  spec 4.3) and its always-false/always-true implication.
- Added `DiagCode::UnidentifiableSource => "unidentifiable-source"`, placed
  directly after `AmbiguousExternal` in the planning-time block, with rustdoc
  stating the condition (discovered primary or resolved donor file exists but
  mkvmerge could not identify it, spec 5.2) and that `detail` carries the
  underlying error text.
- Added a new test `f2_codes_are_registered_with_keys` in `mod tests` asserting
  both `.key()` values and both variants' presence in `DiagCode::ALL`.

`locales/en/diagnostics.ftl`:
- Added `empty-match-list = An "any" or "not" list must not be empty; remove it or add at least one sub-expression.`
- Added `unidentifiable-source = A source file exists but could not be identified: { $detail }.`
  (used the `$detail`-only form per task instructions, to avoid referencing an
  unsupplied `$file` arg).

No other files touched. `muxsmith-core` stays prose-free (no new user-facing
strings in Rust; the two new `///` doc comments are rustdoc, not runtime
strings).

## TDD evidence

1. Wrote the test first (`f2_codes_are_registered_with_keys`) referencing the
   not-yet-existing `DiagCode::EmptyMatchList` / `DiagCode::UnidentifiableSource`.
2. Ran it before touching the enum:

   ```
   $ cargo test -p muxsmith-core --lib report::tests::f2_codes_are_registered_with_keys
   error[E0599]: no variant, associated function, or constant named `EmptyMatchList` found for enum `report::DiagCode`
   error[E0599]: no variant, associated function, or constant named `UnidentifiableSource` found for enum `report::DiagCode`
   (4 occurrences total, could not compile)
   ```

   RED confirmed: fails to compile because the variants do not exist yet (not a
   typo in the test - the two names are exactly the ones about to be added).

3. Added the two `diag_codes!` variants and the two `.ftl` lines, reran:

   ```
   $ cargo test -p muxsmith-core --lib report::tests::f2_codes_are_registered_with_keys
   running 1 test
   test report::tests::f2_codes_are_registered_with_keys ... ok
   test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   ```

   GREEN confirmed.

## Full verification commands run

```
$ cargo test --workspace
... (all suites) test result: ok. 0 failed ... (repeated across every crate/suite)
```
Notably `crates/muxsmith-cli/tests/catalog_completeness.rs`:
```
running 1 test
test every_diag_code_has_a_catalog_message ... ok
```
This is the guard specified in the task: it iterates `DiagCode::ALL` and would
fail if `empty-match-list` or `unidentifiable-source` lacked a Fluent message.
It passed, confirming both new `.ftl` entries render (not falling back to the
raw key).

```
$ cargo fmt --all --check
(no output, exit 0)
```

```
$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
(no warnings, exit 0)
```

ASCII check on the diff (`grep -nP '[^\x00-\x7F]'` over the two changed files):
no matches.

## Concerns

None. Scope was small and self-contained; both variants placed exactly where
the task specified (`EmptyMatchList` after `InvalidPropertyValue`,
`UnidentifiableSource` after `AmbiguousExternal`); the `$detail`-only wording
for `unidentifiable-source` was used per the task's explicit "use the last,
`$detail`-only form" instruction, so the message text does not mention which
file failed identification - later fix tasks that emit this diagnostic should
make sure `config_path`/`file` on the `Diagnostic` itself carries that
context, since the rendered message alone will not.
