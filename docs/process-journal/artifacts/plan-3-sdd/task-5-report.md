# Task 5 report: resolve settable `changes` per assignment + plan-time language validation

(Plan 3, Task 5. Supersedes a stale report previously at this path from an earlier plan's differently-numbered Task 5, the xtask capability generator -- unrelated work, already committed at `e78847d` and prior.)

## What was done

In `crates/muxsmith-core/src/planner.rs`:

- Added a new private helper `resolve_changes(rule: &TrackRule, base: &str, primary_path: &Path, lang: &LanguageIndex, diags: &mut Vec<Diagnostic>) -> Vec<AppliedChange>`. It:
  - Returns `Vec::new()` when `rule.changes` is `None`.
  - Otherwise iterates the rule's `changes: BTreeMap<String, Scalar>` in key order (already property-name ascending, since `BTreeMap` iterates key-ascending) and builds one `AppliedChange { property, value }` per entry.
  - For the entry whose property is `"language"`, validates the value: must be `Scalar::Str(s)` where `lang.normalize(s).is_some()`. Any other case (non-string value, or a string `lang.normalize` doesn't recognize) pushes `Diagnostic::error(DiagCode::InvalidPropertyValue, format!("{base}.changes.language")).for_file(primary_path).with("property", "language").with("value", scalar_display(value))`. The `AppliedChange` is still built and returned in this case (it lands in `assignments`, but the file's plan gets dropped anyway since an error diagnostic forces `plan: None` via the existing `finalize_plans` pass).
  - All other settable properties (`sub_charset`, `track_name`, boolean flags, etc.) are carried through unchecked, per the brief: `validate.rs`'s `validate_changes` already checks type/known-ness at config time; this is a deliberate, file-scoped defense-in-depth pass for `language` only, distinct from the batch-wide `walk_exact_languages` walk over `match.exact.language`.
- Added a small private helper `scalar_display(value: &Scalar) -> String` (match over the four `Scalar` variants, `to_string()`/`clone()`) to render the diagnostic's `value` param as a plain string, since no `Display` impl exists for `Scalar` yet and this is the first per-value diagnostic that needs to handle a non-`Str` case.
- Wired `resolve_changes` into the `1 =>` (single-match) arm of `resolve_file`'s per-rule loop: `let changes = resolve_changes(rule, &base, &primary.path, lang, &mut diagnostics);`, then `changes` (was `vec![]`) on the constructed `Assignment`.
- Added `TrackRule` to the `crate::profile::model::{...}` import list (needed for the helper's parameter type).
- Left every other `assignments.push(...)` construction (`0 =>`, `n =>`, external-source `0`/err/`n` branches) at `changes: vec![]` unchanged, per scope: nothing to apply on a rule that didn't resolve to a real track.

In `crates/muxsmith-core/tests/planner_resolution.rs`:

- Extended the shared `lang()` fixture with a Turkish row (`["Turkish", "tur", "tur", "tr"]`) so a valid non-English/German settable-language value has something to normalize against, distinct from the match-language ("en") already used in the existing fixtures.
- Added `changes_resolve_to_applied_changes_in_property_order`: a rule matching the fixture's `en` audio track with `changes: { language: tr, track_name: X }` yields `plan.assignments[0].changes == vec![AppliedChange { property: "language", value: Scalar::Str("tr") }, AppliedChange { property: "track_name", value: Scalar::Str("X") }]` (property order confirms the BTreeMap-order claim).
- Added `invalid_changes_language_is_plan_time_invalid_property_value`: the same rule with `changes: { language: zzz }` yields `fr.plan.is_none()` and a diagnostic with `code == DiagCode::InvalidPropertyValue && config_path == "tracks[0].changes.language"`.
- Added imports: `muxsmith_core::planner::AppliedChange`, `muxsmith_core::profile::match_expr::Scalar`.

## TDD RED

```
cargo test -p muxsmith-core --test planner_resolution changes
...
test changes_resolve_to_applied_changes_in_property_order ... FAILED
  left: []
 right: [AppliedChange { property: "language", value: Str("tr") }, AppliedChange { property: "track_name", value: Str("X") }]
test invalid_changes_language_is_plan_time_invalid_property_value ... FAILED
  assertion failed: fr.plan.is_none()
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 24 filtered out
```

Both failed as expected before implementation (empty `changes`, no diagnostic emitted -> plan still `Some`).

## TDD GREEN

```
cargo test -p muxsmith-core --test planner_resolution
test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Full gate

```
cargo test --workspace        -> all suites ok (0 failures across every crate/test binary)
cargo fmt --all --check       -> exit 0 (one round of `cargo fmt --all` needed to normalize
                                  line-wrapping of the new call site and a test assertion; re-checked clean)
cargo clippy --workspace --all-targets -- -D warnings  -> clean, no warnings
cargo deny check              -> advisories ok, bans ok, licenses ok, sources ok
```

## Files changed

- `crates/muxsmith-core/src/planner.rs`
- `crates/muxsmith-core/tests/planner_resolution.rs`

## Self-review

- Confirmed `resolve_changes` is only reached from the matched (`1 =>`) branch, so `changes` stays `vec![]` on every unmatched/ambiguous/missing-external/ambiguous-external path, per scope.
- Confirmed the property-order test asserts exact `Vec<AppliedChange>` equality (not a subset check), and the order is guaranteed by `BTreeMap<String, Scalar>` iteration order in `rule.changes`, not by YAML key order -- verified `changes` is typed as `BTreeMap` (not e.g. an order-preserving map) in `profile/model.rs`. `"language" < "track_name"` lexicographically anyway, so this also isn't accidentally passing on YAML order.
- Confirmed the non-`Str` defensive branch is reachable in principle: the planner test harness (`plan_one` -> `plan_batch`) never runs `profile::validate::validate`, unlike the real CLI (`dry_run.rs`, which always runs config-time validate first and would already flag a non-string `changes.language` as `ValueTypeMismatch`). Did not add a dedicated test for the non-`Str` case specifically -- the brief's own example only exercises the invalid-string case, and the check (`matches!(value, Scalar::Str(s) if lang.normalize(s).is_some())`) is a single boolean expression whose non-`Str` branch is trivially symmetric with the invalid-string branch (both just evaluate to `false`).
- Grepped `changes:` across `planner.rs` to confirm no other `Assignment { .. }` construction was missed; all four remaining `vec![]` sites are the intended no-op ones (missing external, unidentifiable donor, ambiguous external, no-match, ambiguous-match).
- `#![deny(missing_docs)]` not implicated: both new functions (`resolve_changes`, `scalar_display`) are private, and the crate compiled/tested clean under the full gate, which would have caught a missing-docs violation on any pub item.
- Re-ran the full gate a second time after a wording tweak to the `scalar_display` comment (no logic change) to confirm nothing regressed.

## Concerns

- None blocking. One deliberate scope note carried over from the brief: an invalid `changes.language` on a rule that never matched a track (unmatched optional rule, ambiguous rule, etc.) goes uncaught by this per-file validation, since `resolve_changes` is only invoked in the matched branch. This mirrors the brief's own stated scope choice (the paragraph under Step 3) and is explicitly deferred, not an oversight.
