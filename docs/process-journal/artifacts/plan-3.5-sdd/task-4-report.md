# Task 4 report: BCP-47 language validation (D19, Plan 3.5)

(This file previously held a stale report from Plan 3's differently-scoped
Task 4 — `Plan`/`Assignment` resolution-field defaults; superseded here.)

## Implemented

1. `crates/muxsmith-core/Cargo.toml`: added `language-tags = "0.3.2"` to `[dependencies]`. `cargo build -p muxsmith-core` updated `Cargo.lock` (one new package, `language-tags 0.3.2`, zero transitive dependencies of its own).
2. `crates/muxsmith-core/src/capability/runtime.rs`: added `LanguageIndex::is_valid_value(&self, token: &str) -> bool`, doc comment verbatim from the brief:
   ```rust
   pub fn is_valid_value(&self, token: &str) -> bool {
       self.normalize(token).is_some() || language_tags::LanguageTag::parse(token).is_ok()
   }
   ```
3. `crates/muxsmith-core/src/planner.rs`: swapped both predicates, located by function name (line numbers in the brief predate Tasks 1-2/D20, which shifted the file):
   - `walk_exact_languages`: `&& lang.normalize(v).is_none()` -> `&& !lang.is_valid_value(v)`.
   - `resolve_changes`: `matches!(value, Scalar::Str(s) if lang.normalize(s).is_some())` -> `matches!(value, Scalar::Str(s) if lang.is_valid_value(s))`.
   - `validate_language_values`'s caller already iterates `profile.tracks.rules` (Task 1's `{ unmatched, rules }` shape); nothing further to adjust.

## TDD RED/GREEN evidence

Added `changes_language_pt_br_regional_tag_is_not_invalid_property_value` to `crates/muxsmith-core/tests/planner_resolution.rs`: a profile with `changes: { language: pt-BR }` on a matched track, asserting `fr.plan.is_some()` and no `InvalidPropertyValue` diagnostic anywhere in `fr.diagnostics`.

To get a clean RED/GREEN pair I temporarily `git stash`-ed only the `planner.rs` predicate swap (keeping the new dependency, the `is_valid_value` method, and the new/edited tests in place), ran the test, then restored the swap:

- **RED** (old `lang.normalize(s).is_some()` predicate still in place): `cargo test -p muxsmith-core --test planner_resolution pt_br -- --nocapture` -> `FAILED`, diagnostic `InvalidPropertyValue { property: "language", value: "pt-BR" }` at `tracks[0].changes.language`.
- **GREEN** (predicate swapped back in via `git stash pop`): same command -> `ok`, 1 passed.

## Regression-test conflict found and fixed (the one deviation from the brief's literal step list)

D19 is explicit: well-formedness only, no registry `validate()` ("a grammatically valid but nonexistent tag is accepted here and left for mkvmerge to reject at mux time"). I probed `language-tags 0.3.2` directly (throwaway crate, not committed) before trusting this: RFC 5646's primary-language subtag grammar (`2*3ALPHA` for the ISO-639 shortest form, `4ALPHA` reserved, `5*8ALPHA` registered) accepts **any** 2-8 letter alphabetic string as well-formed, whether or not it is a real ISO code:

```
"zz"  -> Ok("zz")     "zzz" -> Ok("zzz")   "xx"     -> Ok("xx")
"nolang" -> Ok("nolang")    "notalanguage" (12 chars) -> Err(SubtagTooLong)
"123" -> Err(InvalidLanguage)   "a" -> Err(InvalidLanguage)
```

Two pre-existing regression tests used exactly such strings as "obviously bogus language" fixtures:
- `bad_language_value_is_batch_invalid_property_value` used `language: zz` (the `walk_exact_languages` / match-time path).
- `invalid_changes_language_is_plan_time_invalid_property_value` used `language: zzz` (the `resolve_changes` / changes-time path).

Both "zz" and "zzz" are syntactically well-formed BCP-47 primary-language subtags, so after the predicate swap they would silently start passing plan-time validation — these two tests would flip from a meaningful assertion to a false pass. Satisfying "existing ISO-code validation/matching tests must stay green" as a *real* pass (not just "the suite doesn't crash") required fixing the fixtures, not just the production code.

Fix: changed both fixtures to `language: notalanguage` (12 letters: fails the BCP-47 length bound *and* the ISO lookup). Verified with planner.rs stashed/un-stashed that both tests pass identically before and after the predicate swap — i.e. the fix preserves each test's original intent (a bogus language value is still rejected) rather than papering over the new permissiveness. Also updated the one in-code comment that named "zzz" explicitly, so the file stays internally consistent.

This is the one place I exercised judgment beyond the brief's literal file list (which named only test *additions*, not edits to the two pre-existing tests); flagging it per the task's request to report concerns and judgment calls.

## cargo deny

No `deny.toml` change. `cargo deny check` -> `advisories ok, bans ok, licenses ok, sources ok` on the first run after adding the dependency. `language-tags` is MIT/Apache-2.0 dual-licensed (both already allowed) and pulls in no transitive dependencies.

## Gate (final run before commit, all four green)

```
cargo fmt --all --check                                    -> OK
cargo clippy --workspace --all-targets -- -D warnings       -> OK (no warnings)
cargo test --workspace                                      -> OK, 0 failed anywhere; planner_resolution.rs: 46 passed (was 45; net +1 after the new test)
cargo deny check                                             -> advisories ok, bans ok, licenses ok, sources ok
```

## Files changed

- `Cargo.lock`
- `crates/muxsmith-core/Cargo.toml`
- `crates/muxsmith-core/src/capability/runtime.rs`
- `crates/muxsmith-core/src/planner.rs`
- `crates/muxsmith-core/tests/planner_resolution.rs`

## Self-review

- Doc comment on `is_valid_value` matches the brief verbatim; `#![deny(missing_docs)]` (crate-wide, `src/lib.rs:1`) is satisfied — clippy's build would otherwise have errored, and it didn't.
- No blanket `deny.toml` change; none was needed, matched the brief's expectation.
- Typography: ASCII punctuation only in all new/changed content.
- Both call sites located and confirmed by function name (`walk_exact_languages`, `resolve_changes`), not the brief's stale absolute line numbers, per the task's instruction.
- Commit is a single, atomic commit including `Cargo.lock`, gpgsign disabled, correct trailers.

## Concerns

- The "zz"/"zzz" -> "notalanguage" fixture fix (above) is the only judgment call beyond the brief's literal steps. Confidence is high (empirically probed the actual crate rather than assumed), but it does touch two tests the brief didn't name for editing.
- `is_valid_value` accepts any well-formed-but-fictitious BCP-47 tag (e.g. `xx-YY`, `qq`) at plan time by design (D19) — intended scope cut (mkvmerge is the final authority at mux time), not a gap, but worth keeping in mind if a later task revisits language-validation strictness.
- Two untracked files present before this task started (`HANDOFF.md`, `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`) are unrelated to Task 4; left untouched and unstaged.
