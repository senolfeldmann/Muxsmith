# Task 3 report: property tests (Stream B, after T2)

**Status: DONE**

Worktree `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-b` (branch `plan-5.6-b`).
All 8 checkbox items implemented, mechanically, with zero intended behavior
change except the deliberate T14-m1 `prop_assume!` -> `prop_assert!`
strengthening the plan itself mandates. Nine-part gate green before the
(single) commit.

## Commit

- `cc49337` refactor(tests): property-test idiom cleanup (T14 seeds +
  select/BTreeMap sweeps)

Unsigned (`git -c commit.gpgsign=false`), explicit staging (the four named
files only, no `git add -A`), no push. One commit for the whole task: all
four files are mechanically related (the dup item's `#[doc(hidden)] pub`
markers in `planner.rs` only exist to let `prop_planner.rs` call the real
engine function) and there was no benefit to artificially splitting them.

Note: the first commit attempt omitted the mandated
`Co-Authored-By`/`Claude-Session` trailer lines; caught immediately and fixed
via `git commit --amend` before anything was pushed or shared (final SHA
`cc49337`, tree/diff unchanged, message only).

## Per-item implementation

Anchors re-verified against the code before each edit; all line numbers below
are pre-edit (matched by function name per the brief's warning, though in
practice the three test files' line numbers had not shifted from the brief -
only `planner.rs`'s had, from the preceding planner task's commits).

1. **`prop_planner.rs`:42,51,106,176,245,281 idiom** - the six single/multi-key
   `let mut map = BTreeMap::new(); map.insert(...)` builder chains
   (`exact_one`, `substring_one`, `video_track`, `arb_track`'s closure,
   `arb_nonvideo_track`'s closure, `arb_ambiguous_ident`'s `sub` closure) become
   `BTreeMap::from([(k, v), ...])` literals inlined directly into the
   `MatchExpr`/`Track` struct literal, dropping the mutable local entirely.
2. **`prop_planner.rs`:144,156 idiom** - `select(TYPE_VALUES.to_vec())` ->
   `select(TYPE_VALUES)` at both sites (`arb_plan_expr`, `arb_track`). Verified
   against the vendored proptest 1.11.0 source
   (`~/.cargo/registry/src/index.crates.io-*/proptest-1.11.0/src/sample.rs:156`):
   `pub fn select<T: Clone + fmt::Debug + 'static>(values: impl Into<Cow<'static,
   [T]>>) -> Select<T>`. `TYPE_VALUES` is `pub static TYPE_VALUES: &[&str]`
   (`capability/mod.rs:53`), i.e. `&'static [&'static str]`, which converts to
   `Cow<'static, [T]>` via std's blanket `From<&'a [T]> for Cow<'a, [T]>`
   (`T: Clone`) - no behavior change, drops an unneeded clone.
3. **`prop_planner.rs`:450-454 idiom (seed T14-m1)** - both `prop_assume!`
   guards in `accepted_suggestion_survives_replan` become `prop_assert!` with
   the condition-specific messages from the seed
   (`docs/process-journal/artifacts/idiomacy-review-sdd/seed-4.md`), verbatim;
   the stale "Meaningfulness guard" comment replaced with the seed's
   by-construction-invariant wording. This is the plan's one deliberate
   strengthening, not a behavior change to production code.
4. **`prop_planner.rs`:315-337,360-364 dup (seed T14-m3)** - `planner.rs`'s
   `with_rule_match` (was :1773, now :1778) and `rule_index_of` (was :1948, now
   :1957) marked `#[doc(hidden)] pub` (crate is `publish = false`, confirmed in
   `Cargo.toml`). `prop_planner.rs`'s `apply_suggestion` shrunk from a 22-line
   duplicate of the insert-only splice to a 2-line delegate:
   `with_rule_match(profile, ri, &doc.match_expr)`; the local `rule_index_of`
   copy deleted, both call sites (`s.config_path`, `d.config_path`) now resolve
   through the newly-imported `planner::rule_index_of`. `diag_sig` kept as the
   deliberate documented tuple-vs-string variant (its comment already states
   this; brief said keep, not touch). Splice semantics (never widens an
   existing constraint) stay independently covered by
   `suggestions.rs::with_rule_match_never_widens_an_existing_substring_constraint`,
   confirmed still present and passing.
5. **`prop_matcher.rs`:83-107 stdlib** - `exact_one`/`substring_one`/
   `regex_one`'s one-entry maps become `BTreeMap::from([(prop.to_string(),
   val)])` inlined into the `MatchExpr` struct literal, same pattern as item 1.
6. **`prop_matcher.rs`:179 stdlib** - `arb_track`'s
   `let mut properties = BTreeMap::new(); for (k, v) in entries { ... }` loop
   becomes `properties: entries.into_iter().collect()` (`FromIterator`).
   Verified not a behavior change: `BTreeMap`'s `FromIterator`/`Extend` impl
   inserts in iterator order and a later `insert` for a duplicate key
   overwrites the earlier value, identical to the loop's `properties.insert(k,
   v)` semantics (relevant since `arb_prop_entry`'s `prop_vec(..., 0..6)` can
   draw the same property key twice).
7. **`prop_matcher.rs`:130 etc idiom** - all 14 `select(CONST.to_vec())` sites
   (`STRING_POOL`, `STRING_PROPS`x3, `TYPE_VALUES`, `CODEC_KIND_NAMES`,
   `BOOL_PROPS`x2, `INT_PROPS`, `REGEXES`, `STRING_PROPS_TRACK`,
   `INT_PROPS_TRACK`, `TYPE_VALUES`, `CODEC_POOL`) drop `.to_vec()`; count
   verified by grep before and after (14 -> 0 occurrences of `.to_vec()` in the
   file). The `select(vec!["eng", ...])` literal sites in `prop_planner.rs`/
   `prop_language.rs` are untouched, per the brief - no `.to_vec()` there to
   drop, and they build a fresh `Vec` with no static backing slice to reuse.
8. **`prop_language.rs`:128-130,139-141 stdlib** - `track_with_language`'s and
   `language_expr`'s one-entry maps become `BTreeMap::from([...])` inlined into
   the `Track`/`MatchExpr` struct literal, same pattern as items 1 and 5.

## Gate results (nine parts, per BUILDING.md)

Run in the foreground, full run before the commit.

1. `cargo fmt --all --check` - OK (one `cargo fmt --all` auto-reflow needed
   first, on a single line in `prop_matcher.rs`'s `arb_expr`; re-checked clean)
2. `cargo clippy --workspace --all-targets -- -D warnings` - OK, no warnings
3. `cargo test --workspace` - OK: every suite green, 0 failed, 0 ignored;
   `prop_language` 8/8, `prop_matcher` 7/7, `prop_planner` 3/3 (including
   `accepted_suggestion_survives_replan` under the new `prop_assert!`s)
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - OK; confirms
   `#[doc(hidden)]` exempts the two newly-`pub` planner functions from
   `#![deny(missing_docs)]` (no rustdoc comment was added to either, by design
   - they stay `//` implementation comments)
5. `cargo deny check` - advisories ok, bans ok, licenses ok, sources ok
6. `pnpm lint` - OK
7. `pnpm build` - OK (vue-tsc + vite build)
8. `pnpm check:i18n` - ok (179 catalog ids; pre-existing 12 unused-key warnings,
   unrelated to this task, no frontend files touched)
9. `pnpm test:e2e` - 6 passed

## Self-review

- **Completeness**: all 8 checkbox items done; grep confirms zero residual
  `.to_vec()` in `prop_matcher.rs` and zero residual local `rule_index_of` in
  `prop_planner.rs`.
- **Oracles unweakened**: the D6 property now asserts (not assumes) its two
  preconditions - strictly stronger, per the brief's explicit mandate. Every
  other property test's assertions are untouched; only construction code
  (builders, generators) changed shape, never behavior. The apply_suggestion
  shrink calls the *real* engine function instead of a parallel copy, so the
  test now exercises actual production code instead of a maintained-in-parallel
  mirror - a strict improvement in fidelity, not a loss.
- **Discipline**: no file outside the four named touched; only the two named
  `#[doc(hidden)] pub` markers added to `planner.rs`, no other visibility or
  logic changes there; unsigned; explicit staging; no push.
- **Pristine output**: no warnings, no skips, no ignored, across all nine gate
  parts.

## Surfaced patterns / deviations for the house ledger

- **No new pattern or deviation.** This task is a pure application of two
  already-settled house conventions: `testing-support-helpers` (shared
  cross-file test code goes through a real, single source of truth rather than
  a parallel copy - here extended from the `tests/support/mod.rs` module case
  to a `#[doc(hidden)] pub` crate-internal reuse, since `with_rule_match`/
  `rule_index_of` are planner-internal, not test-helper code) and the
  idiomacy-review rubric's `stdlib`/`idiom`/`dup` dimensions (`proc-09`). No
  convention-file edit was needed.
- **One process note, not a deviation**: the brief's line-number anchors for
  the three test files matched the current code exactly (only `planner.rs`'s
  had shifted, as the brief itself flagged); re-verifying by function name
  before editing cost nothing extra here but would have caught drift had it
  existed.

## Files changed

- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-b/crates/muxsmith-core/src/planner.rs`
  (two `#[doc(hidden)] pub` markers only)
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-b/crates/muxsmith-core/tests/prop_planner.rs`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-b/crates/muxsmith-core/tests/prop_matcher.rs`
- `/home/senol/Git/Muxsmith/.worktrees/plan-5.6-b/crates/muxsmith-core/tests/prop_language.rs`
