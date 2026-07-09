# F7: suggestion engine - no-clobber, valid YAML, cap logging - report

## Status

DONE

## What changed

`crates/muxsmith-core/src/planner.rs`:

### (a) `with_rule_match`: insert-only-if-absent for `exact`/`substring` (bug C)

- `exact` and `substring` deltas now merge via `map.entry(k.clone()).or_insert_with(|| v.clone())`
  instead of `BTreeMap::extend`. `extend` calls `insert` unconditionally and
  overwrites an existing key; a candidate targeting a property/value already
  constrained by the rule (e.g. `AddSubstring` on `track_name` when the rule
  already has `substring: { track_name: X }`) silently replaced the
  constraint instead of adding to it -- a suggestion may only narrow a match,
  never relax it (D6), and an overwrite is a relaxation whenever the new
  value is not a superset-preserving refinement of the old one.
- Under the fix, a candidate whose key already exists becomes a genuine no-op
  for that key: the rule is unchanged, the ambiguity it was meant to resolve
  remains, and `resolves_without_regression` correctly rejects it (never
  emitted). This matches the task's specified behavior exactly; the
  acceptance simulation needed no change.
- `not` entries are untouched (`extend` on the list): appending a not-clause
  is always additive/narrowing regardless of what is already in the list, so
  no collision can occur there by construction.

### (b) `yaml_fragment`: serialize the real delta via `yaml_serde` (bug D)

- Signature changed from `fn yaml_fragment(ri: usize, edit: &StructuredEdit)`
  to `fn yaml_fragment(ri: usize, delta: &MatchExpr)`. The caller now passes
  `&cand.apply` (the actual typed `MatchExpr` delta already used to drive the
  simulation) instead of re-deriving a delta from the `StructuredEdit`'s
  `String`-typed `value` field. This also preserves the original `Scalar`
  type (bool/int/string) of the edited value, which the `StructuredEdit`
  enum's `String` fields do not carry, e.g. a `forced_track: true` edit now
  serializes as `true`, not `"true"`.
- New private struct `MatchFragment<'a> { #[serde(rename = "match")] expr:
  &'a MatchExpr }`, `#[derive(Serialize)]`. The field is not literally named
  `match` (a reserved word); the wire name is set via `rename`.
- The body is now `yaml_serde::to_string(&MatchFragment { expr: delta
  }).expect(...)`, replacing four hand-formatted `format!` arms that
  interpolated a raw string into `{ property: value }` with no quoting.
  `yaml_serde` (the already-used serde-yaml fork) decides quoting per
  `Scalar` variant, so any value -- including one containing `:`, `,`, `{`,
  or `}` -- round-trips correctly. Manually verified output for a
  colon-bearing value:
  ```
  # tracks[0] - add:
  match:
    exact:
      track_name: 'Chapter 1: Intro'
  ```
  still human-readable, matching the spec's "exact YAML fragment" framing.
- `.expect("a MatchExpr delta always serializes to YAML")`: the delta is
  always a small tree of plain owned data built by `delta_for`, so
  serialization cannot fail in practice; `.expect` with a stated invariant
  matches the codebase's existing convention (`identify.rs`, `discovery.rs`)
  over a silent `unwrap_or_else` that would swallow a real bug.

### (c) cap-3 truncation logged via a new diagnostic (D6)

**Mechanism chosen:** a new `DiagCode::SuggestionsCapped` (info severity,
`config_path: "tracks[{ri}].match"`, param `dropped`), pushed into
`Batch.batch_diagnostics`.

Rationale for this over the alternatives:
- A new field on `Batch` (e.g. `Vec<SuggestionCap>`) would need its own
  rendering support wired into the CLI (and, later, GUI) before it becomes
  "visible in the report" at all -- the D6 requirement. `Diagnostic` is
  already rendered generically for both human (`Renderer::diagnostic`, which
  loops over `batch.batch_diagnostics` and calls `renderer.diagnostic(d)`)
  and `--json` output, with zero CLI changes needed. Reusing it is the
  actually-least-invasive path to real visibility, not just the
  smallest-diff path.
- `report.rs`'s own doc comment states the invariant "every variant
  corresponds to exactly one row of the spec 5.2 catalog table"; I kept that
  invariant intact by adding the corresponding row to
  `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` section 5.2,
  alongside the new `DiagCode` variant, its rustdoc, and its Fluent message
  in `locales/en/diagnostics.ftl` (required by the `catalog_completeness`
  CI guard in `muxsmith-cli/tests/catalog_completeness.rs`).
- `suggest`'s return type changed from `Vec<Suggestion>` to
  `(Vec<Suggestion>, Vec<Diagnostic>)`; `plan_batch` now destructures both
  and extends `batch.batch_diagnostics` with the cap diagnostics. `suggest`
  cannot push directly into the final `Batch` because it only receives an
  immutable `baseline: &Batch` (the pre-suggestion snapshot used to compute
  `base_sig`), not the batch being built.
- Per conflicted rule: `total_accepted = accepted.len()` is captured before
  `accepted.truncate(3)`; if `total_accepted - accepted.len() > 0`, one
  `Diagnostic::info(DiagCode::SuggestionsCapped, ...)` is pushed with
  `dropped` set to that count. No diagnostic when the accepted count is
  already <= 3 (nothing was actually dropped).

`crates/muxsmith-core/src/report.rs`:

- Added `SuggestionsCapped => "suggestions-capped"` to the `diag_codes!`
  macro invocation, in the planning-time group, after
  `UnknownPropertySkew`, with rustdoc per the established one-doc-per-variant
  convention. `DiagCode::ALL`, and every test that iterates it
  (`all_keys_are_unique`, `all_keys_match_serde_encoding`), pick it up
  automatically.

`locales/en/diagnostics.ftl`:

- Added `suggestions-capped = { $dropped } further suggestion(s) for this
  rule were capped at 3 and not shown.`

`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`:

- Added the `SuggestionsCapped` row to the spec 5.2 diagnostics table, to
  keep the code/table correspondence `report.rs` documents as an invariant.

## Test-first

Added to `crates/muxsmith-core/tests/suggestions.rs` (new imports: `serde::Deserialize`,
`muxsmith_core::planner::Batch`, `muxsmith_core::profile::match_expr::{MatchExpr, Scalar}`):

1. **`with_rule_match_never_widens_an_existing_substring_constraint`** (a).
   New helper `plan_multi` (plans an arbitrary named-file set, generalizing
   the existing single-file `plan()`). Two-file fixture:
   - File 1 (`AMBIGUOUS_FOO`): two subtitle tracks, both containing "Foo" in
     `track_name` and otherwise identical -- ambiguous under
     `match: { exact: { type: subtitles }, substring: { track_name: Foo } }`,
     and (since candidate generation only draws from files whose *current*
     matched set has >= 2 tracks) the only source of discriminator
     candidates.
   - File 2 (`GUARDED_FOO`): already unambiguous under the *original* rule
     (only its "Foo Only" track matches); its second track ("Director
     Extended") is a decoy sharing no "Foo" but sharing the token "Director"
     with file 1's ambiguous track. This is the fixture that actually
     exposes the bug: a clobbering `with_rule_match` applying
     `AddSubstring{value:"Director"}` resolves file 1's ambiguity (good) but
     silently redirects file 2 from its correct track (id 1) to the decoy
     (id 2) -- with *zero* new diagnostics anywhere, since file 2 still
     matches exactly one track either way. The pre-existing "no new
     diagnostic" acceptance check cannot catch this on its own; only
     insert-only semantics prevents it.
   - Assertions: file 1 is ambiguous; suggestions are non-empty; no emitted
     suggestion is `AddSubstring{value:"Director"}` (the specific clobber
     vector); for *every* emitted suggestion, reapplying it and re-planning
     leaves file 1 unambiguous AND file 2 resolved to track id 1 (never the
     decoy, id 2).
   - New helper `apply_edit_to_no_clobber_rule`, mirroring the existing
     `apply_edit_to_first_rule` pattern but against `P_NO_CLOBBER`'s
     pre-existing `exact`/`substring` constraints; its `AddSubstring` arm
     models the literal effect of a key overwrite (`substring: { track_name:
     {value} }`, replacing "Foo" wholesale) since that is what the buggy
     code's `extend` actually produced internally, and what the test's
     re-plan step needs to reproduce externally when this branch is
     exercised by a not-yet-rejected candidate during the RED run.

2. **`yaml_fragment_round_trips_a_value_containing_a_colon`** (b). New
   single-file fixture (`P_COLON_AMBIGUOUS` + `COLON_TRACK_NAMES`): two
   subtitle tracks differing only in `track_name` ("Chapter 1: Intro" /
   "Extras"), ambiguous under a bare `exact: { type: subtitles }` rule. Since
   `track_name` is a matchable `String` property (`capability::mod.rs`), the
   full value -- not just a whitespace-split token -- becomes an
   `AddExact{property:"track_name", value:"Chapter 1: Intro"}` candidate,
   which resolves the ambiguity on its own and is accepted (rank puts it
   among the top 3 kept). Finds that suggestion, parses its `yaml_fragment`
   with `yaml_serde::from_str` into a local `MatchFragmentDoc { #[serde(rename
   = "match")] match_expr: MatchExpr }`, and asserts the parsed
   `exact["track_name"]` is `Scalar::Str("Chapter 1: Intro")`.

3. **`suggestion_cap_truncation_is_logged_not_silent`** (c). Reuses the
   already-existing `P_AMBIGUOUS`/`SERIES` fixture (no new fixture needed):
   its two conflicting subtitle tracks ("English forced" / "English SDH")
   differ across `forced_track`, `flag_hearing_impaired`, and two
   discriminating `track_name` tokens ("forced"/"SDH"), which alone yield 10
   accepted candidates before ranking/truncation -- well over the cap of 3.
   Asserts `batch.batch_diagnostics` contains a `SuggestionsCapped`
   diagnostic at `config_path == "tracks[0].match"` whose `dropped` param
   parses as a number > 0. Deliberately does not assert the exact count (10
   accepted here), to stay robust to unrelated changes in candidate
   generation elsewhere; the requirement is presence and non-silence, which
   the task text asks for explicitly.

Confirmed RED before implementing (compiling against the new `DiagCode`
variant added first, as scaffolding, with no emission logic yet):

```
$ cargo test -p muxsmith-core --test suggestions
running 5 tests
test yaml_fragment_round_trips_a_value_containing_a_colon ... FAILED
  panicked: yaml_fragment must be valid, parseable YAML: Error { kind: PARSER,
  problem: "did not find expected ',' or '}'", ... }
test with_rule_match_never_widens_an_existing_substring_constraint ... FAILED
  panicked: a candidate overwriting the existing track_name substring must
  never be accepted: found [AddNotExact{...}, AddNotExact{...},
  AddSubstring { value: "Director" }]
test ambiguous_rule_gets_a_validated_suggestion ... ok
test suggestion_cap_truncation_is_logged_not_silent ... FAILED
  panicked: expected a logged suggestion cap for tracks[0]'s conflict
test every_suggestion_survives_the_next_dry_run ... ok
test result: FAILED. 2 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
```

All three failures are exactly the predicted symptoms: a genuine YAML parse
error on the colon-bearing fragment (not a cosmetic issue), the specific
clobber candidate (`AddSubstring{value:"Director"}`) present in the emitted
suggestions, and no cap diagnostic at all (the variant existed but nothing
emitted it yet).

After implementing (a), (b), (c):

```
$ cargo test -p muxsmith-core --test suggestions
running 5 tests
test yaml_fragment_round_trips_a_value_containing_a_colon ... ok
test ambiguous_rule_gets_a_validated_suggestion ... ok
test suggestion_cap_truncation_is_logged_not_silent ... ok
test every_suggestion_survives_the_next_dry_run ... ok
test with_rule_match_never_widens_an_existing_substring_constraint ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

GREEN confirmed. Manually verified the rendered fragment for the colon case
is still human-readable (via a throwaway `--nocapture` print, removed before
commit):

```
# tracks[0] - add:
match:
  exact:
    track_name: 'Chapter 1: Intro'
```

## Full verification commands run

```
$ cargo test --workspace
... every suite: test result: ok, 0 failed
    muxsmith-core lib: 64 passed (unchanged)
    suggestions: 5 passed (was 2, +3 F7 tests)
    catalog_completeness (muxsmith-cli): 1 passed (SuggestionsCapped's
      Fluent entry keeps this green)

$ cargo fmt --all --check
(no output, exit 0; one intermediate `cargo fmt --all` run was needed for
 the new test file's multi-line macro-arg wrapping, then re-verified clean)

$ cargo clippy --workspace --all-targets -- -D warnings
    Checking muxsmith-core v0.1.0 (...)
    Checking muxsmith-cli v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
(no warnings, exit 0)
```

ASCII check (`grep -nP '[^\x00-\x7F]'`) on all changed source/test/locale
files: no matches. The one non-ASCII grep hit in the touched spec doc
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is pre-existing
content (a `Türkçe` example value and box-drawing characters in an unrelated
tree diagram), not introduced by the added `SuggestionsCapped` table row.

`#![deny(missing_docs)]`: the only new `pub`-visible item is the
`DiagCode::SuggestionsCapped` enum variant, rustdoc'd inline in the
`diag_codes!` macro invocation, consistent with every other variant.
`MatchFragment` (the new YAML-serialization wrapper struct) is
module-private, not `pub`, so it needs no rustdoc.

## Concerns

None blocking.

- `suggest`'s return type changing from `Vec<Suggestion>` to `(Vec<Suggestion>,
  Vec<Diagnostic>)` is a private-function signature change (not part of the
  crate's public API), so it carries no compatibility concern; the only
  caller is `plan_batch` in the same module, updated in lockstep.
- The `apply_edit_to_no_clobber_rule` test helper's `AddSubstring` arm
  deliberately encodes "replace the substring map's `track_name` key
  wholesale" -- the literal effect of the pre-fix bug's `extend`-based
  overwrite -- rather than modeling the fixed insert-only behavior. This is
  intentional: the helper's job in this test is to externally reproduce
  whatever the *actual* internal simulation would have produced for a given
  edit, for both the RED run (where this branch is genuinely exercised, since
  the buggy code emits `AddSubstring{value:"Director"}`) and the GREEN run
  (where it is dead code for this specific fixture, since that candidate is
  never accepted post-fix, but stays available/correct for any future
  fixture that does exercise it).
- Did not add a `dropped`/count assertion pinned to the exact figure (10
  accepted, 7 dropped) for the `P_AMBIGUOUS` fixture in test (c); asserted
  presence and non-zero instead, matching the task's literal requirement and
  staying robust to incidental changes in candidate generation elsewhere in
  the engine (e.g. a future property added to the SERIES fixture's tracks).
