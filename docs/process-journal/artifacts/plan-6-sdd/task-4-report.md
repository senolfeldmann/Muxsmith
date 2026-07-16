# Task 4 report: D48 - a canonical save omits default-valued fields

Stream A, worktree `.worktrees/plan6-a`, branch `plan6-a`. Commit `443ce80`.

## What was implemented

`crates/muxsmith-core/src/profile/model.rs`: the 17-row table at design
`:1517-1535` applied row by row. Every row carries three attributes naming
the same function:

- `#[serde(default[= "fn"], skip_serializing_if = "pred")]`
- `#[schemars(extend("default" = serde_json::to_value(fn()).unwrap()))]`

13 rows use the generic `is_default<T: Default + PartialEq>`. Four bespoke
predicates were added, each next to the default-producing function it
names:

- `is_default_true` (next to `default_true()`) for `Input.recursive`.
- `is_keep_filename` (next to `impl FilenameCfg`) for `OutputCfg.filename`.
- `is_primary` (next to `impl SourceCfg`) for `TrackRule.source`.
- `is_drop_policy` (next to `drop_policy()`) for `TracksCfg.unmatched`, the
  core-83 hazard row.

`crates/muxsmith-core/tests/fixtures/all-non-default.yaml`: new fixture,
every one of the 17 fields set to a non-default value. `TrackRule.source`
is set to `external` (covering `TrackRule.source`, `TrackRule.optional`,
`Locator.recursive`, `Locator.case_sensitive` in one rule).

`crates/muxsmith-core/tests/profile_save.rs`: extended with guard 1 (two
tests, verbatim from the brief) and guard 2 (one table test).

## Baseline PASS (step 2, observed against the unmodified serializer)

Before touching `model.rs`, I wrote the fixture and guard-1 tests, then
`git stash`-ed the (already-drafted, at that point unstaged) `model.rs`
edit to run the tests against the pristine derive behaviour, per the
brief's requirement that step 2 be observed on the unmodified serializer
rather than assumed:

```
$ cargo test -p muxsmith-core --test profile_save all_non_default
test all_non_default_fields_survive_the_round_trip ... ok

$ cargo test -p muxsmith-core --test profile_save core83
test the_core83_passthrough_profile_survives_a_save ... ok
```

Both PASS, as the design predicts (`:90-92`): before `skip_serializing_if`
exists, nothing is omitted, so the round trip holds trivially. Then
`git stash pop` restored the `model.rs` edit and work continued.

## Step 4: the deliberate-break RED proof

Changed `TracksCfg.unmatched`'s `skip_serializing_if` from `is_drop_policy`
to the generic `is_default` (leaving the `#[serde(default = "drop_policy")]`
and the `extend` annotation untouched, so only the predicate went naive):

```
running 6 tests
test the_core83_passthrough_profile_survives_a_save ... FAILED
test all_non_default_fields_survive_the_round_trip ... FAILED
test an_unwritable_path_is_an_io_error_not_a_panic ... ok
test canonical_json_round_trips_to_an_equal_model ... ok
test canonical_yaml_round_trips_to_an_equal_model ... ok
test to_file_picks_json_from_the_extension_and_never_changes_format ... ok

failures:

---- the_core83_passthrough_profile_survives_a_save stdout ----
thread 'the_core83_passthrough_profile_survives_a_save' panicked at crates/muxsmith-core/tests/profile_save.rs:83:5:
tracks.unmatched defaults to DROP, so `keep` is not a default and must be written: profile_version: 1
input:
  pattern: E(\d+)
  extensions:
  - mkv
tracks:
  rules: []

---- all_non_default_fields_survive_the_round_trip stdout ----
thread 'all_non_default_fields_survive_the_round_trip' panicked at crates/muxsmith-core/tests/profile_save.rs:72:5:
assertion `left == right` failed: a non-default value must never be omitted (D48 guard 1)
  left: Profile { ..., tracks: TracksCfg { unmatched: Keep, ... }, ... }
 right: Profile { ..., tracks: TracksCfg { unmatched: Drop, ... }, ... }

test result: FAILED. 4 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Exactly the predicted failure mode: `unmatched: keep` silently omitted,
reloads as `Drop`. Reverted (`is_default` -> `is_drop_policy`), reran:
all 6 tests green again. Guard 1 is proven to test what D48 says it tests.

## Guard 2

Implemented as a table test in `profile_save.rs`
(`schema_defaults_match_the_serde_defaults`), house shape per
`capability/mod.rs`'s `settable_maps_to_mkvmerge_options`: a literal
17-row `expected` table (`[(Option<&str> defs_type, &str field, Value
default); 17]`), checked length-first against an independent count of
every `default` key actually present anywhere in the generated schema
(`Profile`'s own `properties` plus every `$defs` type's `properties`),
then row by row via `serde_json::Value::pointer`.

The expected values are **hand-written literals**, deliberately not
derived by calling the same default-producing functions the `extend`
annotations call - the design's own guard-2 analysis (`:1706-1723`) notes
that a call-derived comparison degenerates to `to_value(F()) ==
to_value(F())`, a tautology. A literal table is the independent truth an
`extend` annotation can still drift from.

Beyond the brief's mandate, I additionally proved guard 2 itself can go
red (cheap, and the brief's own "if you believe it cannot fail, that
belief is the trigger for the design trigger, not for deleting the test"
made me want the evidence rather than the belief): temporarily changed
`TracksCfg.unmatched`'s `extend` expression from `drop_policy()` to
`KeepDrop::default()` (the wrong function - same type, so it still
compiles):

```
thread 'schema_defaults_match_the_serde_defaults' panicked:
assertion `left == right` failed: schema default mismatch for Some("TracksCfg").unmatched
  left: String("keep")
 right: String("drop")
```

Reverted, reran: green. This is not part of the brief's required steps
(step 4's red proof is scoped to guard 1); I did it because it was a
30-second check that materially raises confidence in guard 2's own
soundness, consistent with the house rule that a check whose passing
result is an absence needs to be shown to fire once.

Final guard-2 run (unmodified): 7/7 tests green (see full-suite output
below).

## Additional confirmation (not requested, cheap, and load-bearing)

Printed the canonical save of `reference.yaml` via a throwaway example
(written, run, deleted - never committed): **112 lines**, matching D48's
own measured table (`:1383-1387`) exactly. `output:`, `attachments:`,
`chapters:` vanish entirely; `tags:` survives with only `global: drop`;
`title: clear` survives - all as the design predicts.

Also diffed the JSON Schema before/after the 17 attribute pairs: only 3
lines differ (`Profile.output`, `Profile.attachments`, `Profile.tags`
collapsing from their materialized object to `{}`), confirming the other
14 of 17 restore byte-for-byte as D48 claims (`:1653-1678`).

Also wrote a throwaway verification example (written, run, deleted) that
parses `all-non-default.yaml` and asserts each of the 17 fields against
its actual serde default (not a guessed one - caught my own first-draft
bug where I compared `TracksCfg.unmatched` against the wrong constant).
Confirmed: `ALL_NON_DEFAULT = true`, count = 17.

## Gate (nine parts, foreground)

All from `.worktrees/plan6-a` after `cargo fmt --all` (two lines needed
reformatting: the `filename` field's now-multi-attribute serde line, and
one `assert_eq!` call in the new guard-1 test - both mechanical, no
content change).

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | pass (after `cargo fmt --all`) |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | pass, 0 warnings |
| 3 | `cargo test --workspace` | pass, all suites green (479 tests summed across crates, 0 failed) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | pass, 0 warnings |
| 5 | `cargo deny check` | pass: "advisories ok, bans ok, licenses ok, sources ok" (exit 0; pre-existing duplicate-version notes in the tauri tree, unrelated to this change) |
| 6 | `pnpm lint` | pass, 0 findings |
| 7 | `pnpm build` | pass, `vue-tsc --noEmit && vite build` clean |
| 8 | `pnpm check:i18n` | pass ("ok", 12 pre-existing unused-catalog-key warnings, none touched by this task) |
| 9 | `pnpm test:e2e` | pass, 7/7 Playwright tests green |

`crates/muxsmith-core --test profile_save` final run: **7 passed, 0
failed** (the 4 pre-existing D41 tests + guard 1's 2 tests + guard 2's 1
test).

`git diff --exit-code crates/muxsmith-cli/tests/snapshots/`: exit 0,
unmoved. Per the cross-task correction, this is run because the brief
mandates it, not trusted as evidence of round-trip fidelity - that
evidence is guard 1, the step-4 red proof, and guard 2 above.

## Files changed

- `crates/muxsmith-core/src/profile/model.rs` - 17 field attribute pairs,
  1 generic + 4 bespoke predicates, all with rustdoc.
- `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` - new,
  guard-1 fixture.
- `crates/muxsmith-core/tests/profile_save.rs` - guard 1 (2 tests) + guard
  2 (1 test) + `count_defaults` helper, extending Task 2's file.

## Self-review

- **Every row of the 17 has all three attributes naming one function**:
  verified by an `awk` extraction pairing each `#[serde(default...)]` /
  `#[schemars(extend(...))]` / field line across the whole file - all 17
  pairs present, each `extend`'s inner call textually matches the
  `serde(default = "...")` name (or the implicit `Default::default()` for
  plain `#[serde(default)]` rows), and no row is missing either half.
- **The fixture genuinely sets all 17 to non-defaults**: verified
  programmatically (not by inspection) via a throwaway example comparing
  each parsed field against its real serde default; caught and fixed one
  bug in my own checker along the way (compared `TracksCfg.unmatched`
  against the wrong `KeepDrop` variant on the first pass).
- **Guard 2 covers all 17 rows**: the `expected` array is typed
  `[(...); 17]`, a compile-time count; the runtime `schema_default_count`
  cross-check additionally confirms the live schema carries exactly 17
  `default` keys total, not 16 (an untagged extend) or 18 (a stray one).
  Both the row-values and the guard's own ability to fail were exercised
  by deliberate mutation.
- **Test output pristine**: no `#[allow(...)]`, no `.unwrap()`-panics
  left dangling outside test assertions, no dead-code warnings in the
  final committed state (the transient `is_drop_policy` unused-fn warning
  during the step-4 break was expected and gone on revert, confirmed by
  the clean clippy run afterward).
- **No design latitude was exercised.** The 17-row table and the
  mechanism section fully specified every predicate, every `extend`
  expression, and the three struct-valued fields' `{}` collapse; I found
  no fork requiring a NEEDS_CONTEXT escalation.

## Concerns

None blocking. Two notes for the record:

1. I performed the model.rs edit (step 3) before writing the baseline-PASS
   observation (step 2) chronologically in my own workflow, then used
   `git stash` to retroactively produce a true unmodified-serializer
   baseline before re-applying the edit. The *evidence* asked for (guard 1
   green against the pristine derive) was genuinely observed, not assumed
   - the stash/pop just reordered when I looked at it relative to when I
   typed it.
2. Two throwaway diagnostic examples
   (`crates/muxsmith-core/examples/print_save.rs` and
   `verify_fixture.rs`) were written, run, and deleted during
   verification; neither is present in the final tree or the commit.
