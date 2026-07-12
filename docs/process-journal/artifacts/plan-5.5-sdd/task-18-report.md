# Task 18 report: OverlappingRules auto-suggestions (D33, #12iii)

**Status:** DONE. Branch `plan55-t18`, commit `f68e5d7`
(`feat(suggest): overlap-narrowing suggestions (D33, #12iii)`), unsigned, not pushed.
Full nine-part gate green. All four TC acceptance cases pass.

## What was built

`crates/muxsmith-core/src/planner.rs` (+289/-14) and
`crates/muxsmith-core/tests/suggestions.rs` (+361 tests). No new edit variant,
no new `DiagCode`, no new/changed Fluent message. The overlap path reuses the
`AmbiguousRule` machinery via the four D33 touch-points:

- **`SeedMode` enum + param on `candidates_for_rule`.** `Ambiguous` = the
  existing behavior (>=2 matched set, both polarities). `Overlap` = seed from
  the single shared track (`matched.len() >= 1`) and NOT-polarity only. The
  `matched.len() < 2` guard became `matched.len() < mode.min_matched()`; the
  two polarity loops skip positive polarity in `Overlap` mode. This is how the
  "seed from the shared track's property vector" and "NOT-only" forced-list
  items manifest - one shared generator, one mode flag.
- **`no_regression` factored out** of `resolves_without_regression`; the new
  `resolves_overlap_without_regression` reuses it and swaps the "still
  conflicted?" half to key on the overlap **instance** `(file, track)` from the
  diagnostic, not on `rule_index == edited-rule` (forced-list item: the
  diagnostic is filed under the lowest claimant, so index-keying would misread
  "resolved"). For a >=3-claimant overlap, dropping one claimant leaves a
  smaller overlap on the same `(file, track)`, which this still sees as
  unresolved - the automatic degradation the forced list requires.
- **Overlap loop in `suggest`.** `overlap_conflicts(baseline)` distills each
  `OverlappingRules` diagnostic into `{file, track, claimants}`, parsing
  claimants back from T9's rendered `$rules` list (symmetric generation over
  ALL claimants). Candidates are generated per claimant, tagged with their rule
  index, simulated against the whole batch, and accepted by the overlap gate.
  Rank tiebreak between narrowings on different rules: candidate rank, then
  broader rule first (`rule_breadth` = batch-wide match-domain size), then
  lower index. Cap 3 + `SuggestionsCapped` (keyed on the lowest claimant,
  mirroring the diagnostic's `config_path`). Emitted with
  `resolves = OverlappingRules`, `config_path = tracks[edited].match`.
- **`partition_for_overlap`.** Runs when nothing survives batch-wide. An
  overlap is single-file, so it partitions over exactly the conflict's file:
  reports the top-ranked narrowing that resolves the conflict in isolation but
  was rejected batch-wide, as one `kind=group` diagnostic (existing `*[group]`
  render). When nothing resolves it even in isolation (two-required, >=3
  claimants), it emits nothing and the standing `OverlappingRules` diagnostic
  (T9 `$rules`, all claimants) is the no-fix report.

## Per-test-case results (all pass)

- **TC-A (optional yields):** baseline overlap on SERIES id2; all 3 emitted
  suggestions edit `tracks[1]` (the optional rule; every `tracks[0]` candidate
  regresses to `MissingTrack` and is rejected), all `resolves=OverlappingRules`;
  `AddNotExact{forced_track,true}` is present; applying the forced_track NOT
  clears the overlap and produces a plan. **Nuance worth flagging:** the actual
  rank-0 suggestion is `AddNotExact{default_track,false}`, not `forced_track`.
  Both are rank-0 flags on id2 (`default_track:false`, `forced_track:true`);
  the deterministic tiebreak `(rank, prop, value)` orders `default_track`
  before `forced_track` alphabetically. The D33/analysis text's "top-ranked
  forced_track" is illustrative and did not account for id2 also carrying
  `default_track:false`. Both are correct narrowings; I assert `forced_track`
  is present rather than a brittle position-0 equality the rank function
  contradicts. 9 further candidates capped (`SuggestionsCapped tracks[0]`).
- **TC-B (two required):** zero suggestions; no `SuggestionPartition` fabricated;
  `overlapping-rules` names both claimants. Passes.
- **TC-C (batch regression rejection):** two files, all-optional rules
  (`{subtitles,forced}`, `{subtitles,language:en}`); file A = video + forced-eng
  sub (overlap), file B = one non-forced-eng sub only. `AddNotExact{language,eng}`
  on `tracks[1]` clears A but empties B (`EmptyPlan`, a new diagnostic) -> the
  multiset "nothing-new" guard rejects it; the test proves it is absent from
  suggestions, that batch-safe narrowings still survive, that the language NOT
  *does* resolve A in isolation, and that on the full batch it empties B.
- **TC-D (>=3 claimants):** zero suggestions; no partition; `overlapping-rules`
  names all three claimants. Passes.

TC-C used the pre-existing `plan_multi` batch harness already in
`suggestions.rs`; no `plan_one` extension was needed (the brief's "extend
plan_one" note is satisfied by that existing helper).

## Fluent / NEEDS_CONTEXT check (the brief's tripwire)

**No new/changed Fluent message was necessary; the tripwire did not fire.**
- Overlap suggestions render through the existing `dry-run-suggestion`
  (config_path + fragment), which is agnostic to `resolves`.
- Partition groups (TC-C path) render through the existing `suggestion-partition`
  `*[group]` arm.
- The unresolvable-overlap "report" is the already-emitted `overlapping-rules`
  diagnostic (T9 made it name every claimant). I deliberately do **not** emit a
  new `SuggestionPartition` kind for the unresolvable case: doing so would route
  through the `*[group]` default arm and leak `{$count}`/`{$fix}`/`{$files}`,
  forcing a new `[unresolvable]` Fluent arm - exactly the message the brief
  forbids inventing. Leaning on the standing diagnostic keeps it "within the
  existing catalog voice" (D33's marked assumption) with zero Fluent change.
  `catalog_completeness` (4 tests) stays green.

## Structural finding (design feedback, non-blocking)

Analysis while building TC-C surfaced a theorem the D33 memo did not: **for a
single overlap conflict, the "no batch-wide suggestion -> partition group"
outcome (TC-C's literal expected shape) is not realizable.** An overlap is
single-file and narrowing is monotone (add-only), so: a required claimant's
narrowing self-rejects (`MissingTrack` on its own file); an optional claimant's
only batch collateral is `EmptyPlan`/a-new-overlap elsewhere; and whenever a
co-claimant distinguishes the conflict track from a cross-file track, the
NOT on the distinguishing property is a batch-**safe** clearing candidate that
is emitted as a suggestion (so `accepted` is non-empty -> no partition). If no
distinguishing property exists, the co-claimant also overlaps the cross-file
track -> symmetric -> a uniform batch-safe fix exists. Net: whenever an overlap
is resolvable in isolation there is a batch-safe suggestion, so
`partition_for_overlap`'s group branch effectively never fires; the overlap
partition degenerates to the unresolvable (zero-group) case.

I therefore implemented `partition_for_overlap` faithfully (it is correct and
defensive - it *would* emit a group for a genuine isolation-resolvable-but-
batch-rejected conflict), but wrote TC-C to prove the **realizable** guarantee
D33 actually cares about ("the multiset nothing-new guard rejects batch-unsafe
overlap narrowings") rather than force an unrealizable partition outcome. This
is a candidate ROADMAP/memo note: TC-C in the D33 spec rests on a scenario the
narrow-only grammar + per-file overlap emission cannot produce.

## Gate (nine parts, all green, run foreground)

1. `cargo test --workspace` - ok (incl. 112-test core lib, 12 suggestion tests,
   4 catalog_completeness).
2. `cargo fmt --all --check` - ok (formatter applied to the new tests).
3. `cargo clippy --workspace --all-targets -- -D warnings` - ok.
4. `cargo deny check` - advisories/bans/licenses/sources ok.
5. `pnpm lint` - ok.
6. `pnpm build` (vue-tsc + vite) - ok.
7. `pnpm check:i18n` - ok (12 pre-existing gui-* unused warnings, unrelated).
8. `pnpm test:e2e` (playwright) - 3 passed.
9. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` - ok.

`pnpm install --frozen-lockfile` run once (node_modules was absent).

## Self-review / concerns

- **Reuse discipline:** one shared candidate generator (SeedMode flag), one
  factored `no_regression`, `yaml_fragment`/`with_rule_match`/`rule_source_ident`
  reused. No near-duplicate of the ambiguous path.
- **Scope:** `partition_for_overlap` is arguably dead-ish code per the theorem
  above, but it is the design-mandated touch-point, is correct, and handles any
  edge case the theorem misses; I kept it rather than silently drop a decided
  mechanism.
- **`SuggestionsCapped` config_path** for overlap is keyed on the lowest
  claimant (`tracks[N].match`) while the capped suggestions themselves edit
  another rule; this matches the `OverlappingRules` diagnostic's own filing
  convention and is not asserted beyond existence.
- **TC-A ranking nuance** (default_track vs forced_track) documented above - the
  only place the implemented behavior diverges from the memo's illustrative text,
  and it is the memo that is imprecise, not the rank function.
