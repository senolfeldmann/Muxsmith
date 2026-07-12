# Task 14 report: proptest for the correctness core (#1)

Status: **DONE**

Delivered the spec-§10 property-based test suite for the correctness core
(match algebra, language normalization, planner determinism + suggestion
survival), deferred since Plan 2. No production code touched; only a dev-dep
pin and three new test files.

## proptest version

- Pinned exact: `proptest = "=1.11.0"` in
  `crates/muxsmith-core/Cargo.toml` `[dev-dependencies]`.
- Source of truth: crates.io registry API
  (`https://crates.io/api/v1/crates/proptest`), fetched 2026-07-11.
  `max_stable_version` = `newest_version` = **1.11.0** (published 2026-03-24).
  Not from training memory.
- Resolves and builds under the pinned toolchain (rustc 1.96.1); locked at
  1.11.0 in `Cargo.lock`.
- `cargo deny check` clean with the new tree: advisories ok, bans ok,
  licenses ok, sources ok. proptest's transitive deps (rand 0.9, ppv-lite86,
  rusty-fork, unarray, ...) are all MIT/Apache-permissive; no new advisory or
  license exemption needed.

## Files changed

- `crates/muxsmith-core/Cargo.toml` - added the pinned dev-dep (2 lines +
  comment).
- `crates/muxsmith-core/tests/prop_matcher.rs` (new) - 7 properties.
- `crates/muxsmith-core/tests/prop_language.rs` (new) - 8 properties.
- `crates/muxsmith-core/tests/prop_planner.rs` (new) - 3 properties.
- `Cargo.lock` - proptest + transitive deps.

All three test files reuse the existing `tests/support/` module
(`FakeIdent`, `lang()`); no changes to shared test support or production.

Default `PROPTEST_CASES` (256) throughout, so CI seeding is deterministic.
No `proptest-regressions/` fell out (every property passed on the first run;
no shrink was ever triggered).

## Per-property notes

### prop_matcher.rs (match algebra, spec 4.3/4.4)

Strategy shape: `arb_expr()` is a `prop_recursive` (depth 3) generator of
**validated-by-construction** `MatchExpr` - exact/substring/regex leaves over
real matchable properties with type-correct values, closed-domain values for
`type`/`codec_kind`, `codec_kind` only under `exact`, regex patterns drawn
from a curated compiling set (so InvalidRegex is impossible), and non-empty
`any`/`not` lists. String values come from `free_string()`: weighted 3:1
toward a shared pool (overlaps with track values, so matches are
non-vacuous) vs. genuinely arbitrary `any::<String>()` (arbitrary UTF-8).
`arb_track()` builds tracks over the same vocabulary. Index: the shared
`lang()` (eng/ger/tur). 256 cases each.

- `not_not_is_identity` - `matches(not(not(e))) == matches(e)`.
- `any_singleton_equals_inner` - `matches(any([e])) == matches(e)`.
- `any_is_order_insensitive` - reversal and rotation of an `any` list agree.
- `any_is_disjunction` - `matches(any(v)) == v.iter().any(matches)`.
- `not_is_nor` - `matches(not(v)) == !v.iter().any(matches)`.
- `matcher_is_total_and_deterministic` - arbitrary validated expr + arbitrary
  track (arbitrary UTF-8 in values/patterns): matching never panics and is a
  pure function (two calls agree). This is the "no panic on arbitrary UTF-8"
  property; invalid regex is excluded by construction.
- `generated_exprs_are_validation_clean` - the standing contract behind the
  others: every generated expr, wrapped in a minimal profile and run through
  `validate()`, produces no error-severity diagnostic under `tracks[0].match`.
  Makes "invalid regex is impossible post-validation" a proven fact, not an
  assumption.

### prop_language.rs (normalization + canonical equality, spec 4.4)

Strategy shape: `arb_rows()` mints `LanguageIndex` rows with **globally
disjoint** synthetic codes (base-26 alpha, `id*3 / id*3+1 / id*3+2`), keyed
off a `btree_set` of distinct ids - mirroring the real
`mkvmerge --list-languages` invariant that each ISO code appears once (a
shared code across two canonical groups is the only thing that would break
idempotence, and that input never occurs upstream). 639-2/639-1 cells are
deterministically blanked per id for coverage of partial rows.
`arb_wellformed_tag()` builds RFC-5646-shaped `lang[-Script][-REGION]` tags.
`arb_lang_token()` mixes real ISO codes, well-formed tags, and arbitrary
UTF-8.

- `normalize_is_idempotent` - a token's canonical key is a fixed point
  (`normalize(normalize(x)) == normalize(x)`).
- `normalize_is_case_insensitive` - ASCII-case flips never change the key.
- `codes_in_a_row_share_one_canonical` - the 639-1/2/3 spellings of one row
  all normalize to a single canonical key (bare-ISO forms agree).
- `valid_value_contains_normalizable` - `is_valid_value` is a superset of
  `normalize`.
- `wellformed_tags_are_valid_values` - well-formed BCP-47 tags are accepted
  even against an empty index (the tag-grammar path, D19).
- `queries_are_total_on_arbitrary_input` - no panic on arbitrary UTF-8.
- `language_equality_is_reflexive` - via the matcher's `exact: { language }`
  path: a track's own language value always matches itself.
- `language_equality_is_symmetric` - swapping profile/track operands never
  changes the verdict. Exercises the real `lang_eq` including the BCP-47
  canonicalization fallback (private in `matcher.rs`, reached here through the
  public matcher API) over ISO codes, tags, and arbitrary strings.

### prop_planner.rs (determinism, rendered names, D6)

Harness: `run_plan` writes dummy files into a throwaway source dir and calls
`plan_batch` through `FakeIdent`; identifications built directly as structs.

- `plan_is_byte_identical_across_runs` (determinism, spec 5.5) - one source
  dir, two fresh identifier caches, generated profile (1-2 rules) over
  generated files (1-4 primaries, 1-4 random tracks each): the two serialized
  `Batch`es are byte-identical. Catches any HashMap-ordering leak into output.
- `produced_plan_names_are_well_formed` (D4, spec 4.8) - profile with a
  `type: video` optional rule over files each carrying exactly one video
  track (so a plan is always produced), filename `keep` / `{match}` /
  `{s}-{e}`: every produced plan's output filename has no path separator,
  ends in `.mkv` over a non-empty (`.`/`..`-free) stem, and sits directly in
  the output directory (`parent() == out_dir` - the "no separator injection"
  check, since an injected separator would push the file into a subdirectory).
- `accepted_suggestion_survives_replan` (D6, spec 5.3) - the flagship.
  `arb_ambiguous_ident()` generates one video + two subtitle tracks of
  **distinct** languages, so `exact: { type: subtitles }` is guaranteed
  ambiguous and a resolving discriminator is guaranteed to exist (random
  track_name/forced spread otherwise). For every emitted suggestion: parse its
  own `yaml_fragment` (what the CLI prints / GUI applies), splice it into the
  rule with insert-only semantics (mirrors `with_rule_match`), re-plan, and
  assert (a) the rule's `AmbiguousRule` is gone everywhere and (b) no
  diagnostic signature absent from the pre-edit baseline appears
  (basename-scoped signature, so the two throwaway dirs are comparable). Two
  `prop_assume` guards keep it non-vacuous (batch must actually be ambiguous
  and actually carry suggestions); the generator satisfies both essentially
  always.

## Real-bug findings

None. All 18 properties pass at 256 cases. No `#[ignore]`, no
`DONE_WITH_CONCERNS`.

## Gate results (from worktree root, per BUILDING.md)

| Step | Result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | pass |
| `cargo test --workspace` | pass (18 new prop tests + all existing; 0 failures) |
| `cargo deny check` | pass (advisories/bans/licenses/sources ok) |
| `pnpm lint` | pass |
| `pnpm build` | pass |
| `pnpm check:i18n` | pass (ok; pre-existing unused-id warnings only) |
| `pnpm test:e2e` | pass (3/3) |

`pnpm install --frozen-lockfile` was run once first (node_modules absent).

## Self-review

- **Do the properties test real behavior?** Yes. The algebra laws
  (not-not, any/not disjunction/NOR) and the D6 survival check exercise the
  actual `matcher::matches` and `planner::plan_batch`/`suggest` codepaths, not
  restatements. `language_equality_is_symmetric` reaches the private `lang_eq`
  BCP-47 canonicalizer through the public matcher API, so the canonicalization
  fallback is genuinely covered.
- **Are strategies meaningful, not vacuous?** The shared-pool weighting makes
  matcher leaves hit and miss in a real mix (not all-false disjunctions); the
  D6 generator guarantees ambiguity + a resolving suggestion so its two
  `prop_assume`s almost never reject; determinism operates on non-trivial
  batches (real diagnostics, sometimes suggestions).
- **Validated-by-construction contract** is itself asserted
  (`generated_exprs_are_validation_clean`), so the "invalid regex impossible
  post-validation" premise is checked, not assumed.

## Scope notes / concerns

- D6 uses a single primary per case. The multi-file no-regression path (bug C,
  a suggestion that resolves one file but silently redirects another) is
  already covered by the targeted `tests/suggestions.rs` fixture; this
  proptest generalizes the single-file resolve-without-regression loop across
  random track-property combinations. A multi-file random generator was judged
  not to add proportional coverage over the existing fixture. Non-blocking.
- The suggestion delta is applied via the emitted `yaml_fragment` (the
  user-facing artifact) rather than reconstructing typed scalars from the
  `StructuredEdit` string, so the property tests "the suggestion as shown,
  applied" - the strongest reading of D6.
