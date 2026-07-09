# F9: two rendering fixes (bugs J and G) - report

## Status

DONE

## What changed

### (a) diagnostic renderer must attribute the file (bug J)

`crates/muxsmith-cli/src/i18n.rs` (`Renderer::diagnostic` only):

- `Renderer::diagnostic` now branches on `d.file`:
  - `Some(file)`: renders via a new `diagnostic-line-file` Fluent message,
    passing `file` (via `to_string_lossy()`) as an extra `$file` arg
    alongside the existing `$severity` / `$config_path` / `$message`.
  - `None`: unchanged, still renders via the existing `diagnostic-line`
    message with the same three args as before.
- `locales/en/cli.ftl`: added
  `diagnostic-line-file = [{ $severity }] { $file } { $config_path }: { $message }`
  next to the existing `diagnostic-line = [{ $severity }] { $config_path }: { $message }`,
  which is untouched.
- No call-site changes needed: `validate.rs` and `dry_run.rs` (text and JSON
  paths) all go through the single `Renderer::diagnostic` method, so every
  batch-level diagnostic that carries a file (`IgnoredFile`,
  `MultipleIdentifierMatches`, `DuplicateIdentifier`, `UnknownPropertySkew`,
  `UnidentifiableSource`, all set via `.for_file(...)`) now prints its own
  file inline instead of reading as attached to whatever file was printed
  just above it.

### (b) unknown-property-skew message references an unsupplied param (bug G)

`locales/en/diagnostics.ftl`:

- Old: `unknown-property-skew = Property "{ $property }" is unknown to this
  Muxsmith build but reported by the local mkvmerge; it is matched untyped.`
  -- references `$property`, which the only emitter
  (`crates/muxsmith-core/src/planner.rs:268`,
  `Diagnostic::warning(DiagCode::UnknownPropertySkew, "input").for_file(&primary.path).with("version", ...)`)
  never supplies, so it rendered the literal `{$property}` to the user.
- New: `unknown-property-skew = This file was identified by a newer mkvmerge
  format (version { $version }) than this build pins; unknown track
  properties are matched untyped.` -- references only `$version`, the param
  the emitter actually supplies. Emitter in `planner.rs` was not touched, per
  the task constraint.

## Test-first

Added to `crates/muxsmith-cli/src/i18n.rs`'s existing `#[cfg(test)] mod
tests`, building `Diagnostic` via `muxsmith_core::report::Diagnostic`'s
public builders (`::info`, `.for_file(...)`):

1. **`diagnostic_with_file_includes_the_file_path`**: `Diagnostic::info(DiagCode::IgnoredFile,
   "input").for_file("some/path.mkv")`, rendered via `Renderer::diagnostic`.
   Asserts the output contains `"some/path.mkv"`.
2. **`diagnostic_without_file_omits_it_and_still_renders`**: same diagnostic
   without `.for_file(...)`. Asserts the output does NOT contain
   `"some/path.mkv"`, and still contains `"[info]"` and the config path
   `"input"` -- i.e. the `None` path renders correctly and unchanged.
3. **`unknown_property_skew_uses_only_the_supplied_version_param`**: calls
   `renderer.msg("unknown-property-skew", &[("version", "42")])`. Asserts the
   output contains `"42"` and does NOT contain the literal `"{$property}"` or
   `"$property"`.

Confirmed RED before implementing (tests added, `i18n.rs`/`.ftl` files
unchanged):

```
$ cargo test -p muxsmith-cli --lib i18n
running 5 tests
test i18n::tests::unknown_message_id_falls_back_to_raw_id ... ok
test i18n::tests::diagnostic_without_file_omits_it_and_still_renders ... ok
test i18n::tests::invalid_locale_falls_back_to_en_and_renders ... ok
test i18n::tests::unknown_property_skew_uses_only_the_supplied_version_param ... FAILED
test i18n::tests::diagnostic_with_file_includes_the_file_path ... FAILED

failures:
---- i18n::tests::unknown_property_skew_uses_only_the_supplied_version_param stdout ----
panicked: expected version in: Property "{$property}" is unknown to this
Muxsmith build but reported by the local mkvmerge; it is matched untyped.

---- i18n::tests::diagnostic_with_file_includes_the_file_path stdout ----
panicked: expected file path in: [info] input: File matches the extension
list but not the input pattern.

test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Exactly the predicted symptoms: the file-carrying diagnostic renders with no
file at all, and the skew message leaks the unresolved `{$property}`
placeholder.

After implementing both fixes:

```
$ cargo test -p muxsmith-cli --lib i18n
running 5 tests
test i18n::tests::unknown_message_id_falls_back_to_raw_id ... ok
test i18n::tests::diagnostic_without_file_omits_it_and_still_renders ... ok
test i18n::tests::invalid_locale_falls_back_to_en_and_renders ... ok
test i18n::tests::unknown_property_skew_uses_only_the_supplied_version_param ... ok
test i18n::tests::diagnostic_with_file_includes_the_file_path ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

GREEN confirmed.

## Full verification commands run

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed
    (no FAILED, no error[ lines in the full run's output)

$ cargo fmt --all --check
(no output, exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)

$ cargo test -p muxsmith-cli --test catalog_completeness
running 1 test
test every_diag_code_has_a_catalog_message ... ok
```

ASCII check (`grep -nP '[^\x00-\x7F]' locales/en/cli.ftl locales/en/diagnostics.ftl crates/muxsmith-cli/src/i18n.rs`):
no matches.

## Concerns

None blocking.

- `diagnostic-line-file` puts `$file` between the severity bracket and the
  config path (`[{ $severity }] { $file } { $config_path }: { $message }`),
  matching the task's suggested wording exactly. The file string is whatever
  `PathBuf::to_string_lossy()` produces (platform-native separators; lossy
  UTF-8 substitution for non-UTF-8 paths on Unix), consistent with how paths
  are already surfaced elsewhere in the CLI (e.g. `identify-failed`'s
  `$file`).
- The emitter in `planner.rs` was intentionally left untouched per the task;
  only the catalog message was reworded to match its actual param contract.
  No other `DiagCode` catalog entry references a param its only emitter does
  not supply (not re-audited exhaustively here since only bug G was in
  scope, but the fix pattern -- match the message to the emitter's actual
  `.with(...)` calls -- generalizes if a similar skew is found elsewhere).
