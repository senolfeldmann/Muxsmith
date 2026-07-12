# D33 analysis: OverlappingRules auto-narrowing suggestions

Design round for Plan 5.5 Task 17. No code. This document is the ANALYSIS;
Şenol decides; the memo is written after his decision. Task 18 implements
whatever D33 selects. Line references are to `crates/muxsmith-core/src/planner.rs`
on current master (`a999cb5`, T13 merged).

## Problem

The D6 suggestion engine (`suggest`, planner.rs:991ff) auto-generates and
validates narrowing refinements for `AmbiguousRule`. For `OverlappingRules`
(two or more rules each claiming the same single track) it produces nothing:
D6's algorithm sketch (`docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md`
§"D6", step 1) says "an `OverlappingRules` group generates candidates for each
of the two rules independently (narrowing either can resolve it)", but step 2's
discriminator generation is written for the ambiguous case (diff the property
vectors *within* a rule's matched set of >=2 tracks). For overlap there is no
>=2 set to diff: each claimant matched exactly one track. D33 fixes what "a good
overlap-narrowing proposal" is, or establishes that in the common case there
isn't one.

## Current engine mechanics (post-T13)

**Emission site (planner.rs:385, 493-536).** `claims: BTreeMap<(source, track_id),
Vec<rule_index>>` is populated **only** in the exactly-one-match arm (line
493-498, the `1 =>` case). The zero-match arm (`MissingTrack`) and the >=2-match
arm (`AmbiguousRule`, line 508) never push to `claims`. At line 526 any
`(source, tid)` claimed by >=2 rules emits one `OverlappingRules` error whose
`config_path` is `tracks[{rules[0]}]` (the earliest claimant by rule index; the
`Vec` is push-ordered, so ascending `ri`), with params `rule_a=tracks[rules[0]]`,
`rule_b=tracks[rules[1]]`, `track=tid`. **Only `rules[0]` and `rules[1]` are
named even when >=3 rules claim the track** (a known roadmap polish item; overlap
suggestions need all claimants, so it converges here).

**Structural consequence, load-bearing for all of D33:** every rule that
contributes to an overlap matched **exactly one** track — the shared one. This
is not incidental; it is enforced by the emission site (`claims` receives only
the `1 =>` arm).

**Candidate generation (`candidates_for_rule`, planner.rs:1177).** For a rule
`ri`, over every primary: take the rule's source identification
(`rule_source_ident`, keyword=primary / external=the single donor, T13),
collect the tracks the rule **ambiguously matches**, and bail if `matched.len()
< 2` (line 1198). For each matched track it seeds discriminator candidates from
the track's property vector — `properties` plus the T13 pseudo-props `type`,
`codec`, `id` (line 1210-1221) — gated by `capability::matchable_type`, in **both
polarities** (`AddExact`/`AddNotExact`; for `track_name`, additionally
`AddSubstring`/`AddNotSubstring` per whitespace token). `seen` dedups
`(prop, value, polarity)`.

**Rank (`rank_of`/`rank_substring`, planner.rs:1405-1422).** typed flags (0) <
language (1) < other exact (2), doubled + polarity (positive before negation);
`track_name` substring last (6+polarity). Sort ascending, cap 3, `SuggestionsCapped`
info records the drop.

**Simulate + accept (`suggest` loop, planner.rs:1019-1024; `resolves_without_regression`,
planner.rs:1356).** Apply the delta via `with_rule_match` (insert-only-if-absent
for exact/substring — a narrowing may never overwrite/relax; `not` always
additive), re-run `plan_core` over the whole batch, accept iff **(a)** rule `ri`
carries no `AmbiguousRule` anywhere in the sim, and **(b)** the diagnostic
*multiset* (`diag_signature`, keyed `code|config_path|file`, count-valued per
R1 v) does not grow for any signature. (b) is code-agnostic: a new `MissingTrack`,
a new `AmbiguousRule`, or a **new `OverlappingRules`** all disqualify a candidate
already. (a) is hard-wired to `AmbiguousRule` and is the piece that needs
generalization.

**No-single-fix (`partition_for_rule`, planner.rs:1080).** When nothing survives
batch-wide: for each affected file, find the top-ranked candidate that resolves
it *in isolation*, group files by that fragment, emit one `SuggestionPartition`
info per group (cap 5 + overflow note). `affected` is keyed on `AmbiguousRule`
for `ri` (line 1098). Reuses the same candidate list.

**Static counterpart.** `ProvableOverlap` (config-time lint, report/mod.rs:109)
flags one exact-only rule's conditions subsuming another's. That is a
compile-time subsumption warning; `OverlappingRules` here is the runtime fact
(both rules landed on the same identified track). D33 concerns only the runtime
suggestion path.

## The decisive structural fact: narrow-only cannot redirect

The v1 edit grammar (`StructuredEdit`, planner.rs:170-202) is **add-constraint-only**:
`AddExact`, `AddNotExact`, `AddSubstring`, `AddNotSubstring`. Top-level matching
is conjunctive (`exact` AND `substring` AND `regex` AND `any` AND `not`), so
adding any clause is **monotone-restrictive**: it can only shrink a rule's
matched set, never grow or redirect it.

Combine that with "every overlap claimant matched exactly one track": narrowing
a claimant rule can only take its match on the contested source from `{T}` to
`{T}` (a no-op that does not resolve) or to `{}` (empty). **It can never move the
rule onto a different track.** Redirection is impossible under v1's grammar.

Therefore a single narrowing resolves an overlap **iff it shrinks one claimant's
match on the contested source to empty without introducing a regression**, which
happens only when:

- **the yielding rule is `optional`** (empty match = satisfied, no `MissingTrack`), or
- **the yielding rule matches legitimately on *other* files in the batch**, and the
  added constraint excludes only the contested track, not the legitimate ones.

The common case — **two required rules colliding on one track in a single file**
— is **unresolvable by any v1 suggestion**: narrowing either to empty produces a
`MissingTrack`, which (b) rejects. The honest output there is the explicit
no-fix report, not a suggestion.

This sharply narrows D33: "WHAT to narrow with" is largely determined, and "WHICH
rule" is determined by *feasibility* (optionality / batch structure), not by a
precedence heuristic.

## Selection-policy steelmen (WHICH rule gets narrowed)

### Policy 1 - later-by-list-order (narrow `rules[1]`, keep `rules[0]`)
**For:** cheapest and fully deterministic; matches a "first rule wins, the later
one is the intruder" mental model; the diagnostic already files `config_path`
under `rules[0]`, so "edit the other one" is a one-liner.
**Against:** Muxsmith's resolution model has **no rule precedence** — rules are
independent under strict uniqueness, which is *why* an overlap is an error rather
than a silent shadow (spec 2, 5.2). Baking "later yields" invents a precedence
the model rejects. Worse, it is often wrong: the *earlier* rule is frequently the
accidental over-broad one (`type: video` written before a specific codec rule).
And it is orthogonal to feasibility: if `rules[1]` is the required rule and
`rules[0]` optional, narrowing `rules[1]` regresses and yields nothing while a
valid fix on `rules[0]` exists.

### Policy 2 - broader-by-match-domain-size (narrow the more permissive rule)
**For:** the strongest *intent* heuristic — the rule matching more tracks
across the batch is the accidental over-claimer; the specific rule is the
deliberate one (`{type: video}` vs `{codec_id: V_MPEG4/ISO/AVC}` → narrow the
`type` rule). Aligns the suggestion with what the user probably meant.
**Against:** "broader" is ambiguous to measure — over this file, this source's
track set, or the whole batch? counting matched tracks (all overlap claimants
tie at 1 on the contested source) or counting constraints/domain? On the
contested file the two claimants match the identical single track, so the naive
"matched-count" measure is always a tie and needs a fallback. It is still a
guess at intent, and like Policy 1 it can pick the *required* rule and produce a
regressing, rejected candidate when the *optional* rule was the only feasible
one.

### Policy 3 - narrowing-survives-`resolves_without_regression` (symmetric, both rules)
Do not pre-select. Generate candidates for **both** (all) claimants, simulate
each, emit every narrowing that clears the overlap with no regression, ranked;
the user picks. **For:** this is exactly D6 step 1's "narrowing either can
resolve it" and exactly how the `AmbiguousRule` path already works (generate
liberally in both polarities, let simulate+accept filter, rank deterministically)
— consistency with the shipped engine is itself an argument. Given the
structural fact above, **feasibility itself selects the rule**: only the optional
/ batch-legitimate rule's narrowing survives; guessing is unnecessary and
strictly dominated (Policies 1/2 can only ever pick a rule that *also* survives
acceptance, or a rule that regresses and is discarded — never better than letting
acceptance decide). **Against:** can surface a "narrow `rules[0]`" option the
user finds counterintuitive when they think of `rules[1]` as the intruder; more
candidates to simulate (cost is a non-issue, D6 complexity note); and when
narrowings on two *different* rules both survive (e.g. both optional), ranking
across rules needs a rule-identity tiebreak for deterministic ordering.

## Narrowing-dimension analysis (WHAT to propose)

Reuse the **dimension vocabulary and both-polarity, acceptance-filtered
machinery** of `candidates_for_rule` — do **not** invent an "overlap-specific
dimension" mechanism. Two changes to the seed, both forced by the structural
fact:

1. **Seed source: the shared track's own property vector**, not a >=2 matched
   set (which is empty for overlap). The `matched.len() < 2` guard (line 1198)
   must be bypassed on the overlap path; the contested track is taken from the
   diagnostic's `track` param + `file`, and its properties (incl. T13 pseudo-props
   `type`/`codec`/`id` and `track_name` tokens) seed the candidates.

2. **Polarity: NOT-only is the meaningful set.** Because narrowing can only zero
   the claim, the useful edit is one that **excludes** the shared track:
   `AddNotExact{p, v}` / `AddNotSubstring{tok}` for a property value the shared
   track *has*. `AddExact` toward a value the shared track *lacks* also zeroes the
   match, but expresses "not T" obliquely and can be a no-op when it duplicates an
   existing key (insert-only-if-absent, `with_rule_match`); the NOT form is the
   honest expression of "this rule should not take that track". **Recommendation:
   generate NOT-polarity candidates from the shared track's props for the overlap
   path; drop positive `AddExact`/`AddSubstring` seeding there** (they cannot
   redirect and only add near-tie noise to the ranked list). This is a genuine
   departure from the ambiguous path, justified by the monotone-narrowing fact.

The brief's alternative — "overlap-specific dimensions: the properties on which
the two rules' *matched track sets* differ" — only yields signal in a multi-file
batch where the two rules diverge on some files; on the single contested file
they match the identical single track, so the intra-file difference set is empty.
It is a strict subset of what "NOT-from-the-shared-track's-props, acceptance-
filtered across the batch" already covers. Reject it as a separate mechanism.

## Acceptance-criterion interaction

Four precise touch-points; only the first is a real logic change.

1. **Generalize the "still conflicted?" gate (`resolves_without_regression`,
   line 1361-1368).** Today it checks "no `AmbiguousRule` for `ri`". For overlap
   the gate must be "**the targeted overlap instance is gone**" — keyed on the
   conflict identity `(rule_a, rule_b, track, file)` from the diagnostic's params,
   **not** on `rule_index == edited-rule`. Reason: the overlap diagnostic is filed
   under `config_path = tracks[rules[0]]`; if we edit `rules[1]` and key the gate
   on the edited index, it would spuriously read "resolved" while the diagnostic
   still sits under `rules[0]`. Cleanest formulation: pass the target conflict
   signature in and require its count in the sim to drop to zero.

2. **The "nothing new anywhere" half (line 1369-1371) needs no change.** The
   multiset containment already rejects a new `MissingTrack` (over-shrink of a
   required rule), a new `AmbiguousRule`, and a **new `OverlappingRules`** (if
   narrowing one rule causes another rule-pair to newly collide — a batch
   phenomenon; impossible single-file since narrowing only zeroes). This is the
   whole reason (b) was made code-agnostic and multiset-valued in T13; it pays
   off directly here.

3. **>=3 claimants degrade to the partition path, correctly and automatically.**
   To clear a 3-rule overlap on `T`, at least two claimants must drop `T`. A
   single narrowing leaves a 2-rule overlap → an `OverlappingRules` diagnostic
   remains → gate (1) rejects → no candidate survives → `partition_for_rule`
   fires. v1 does **not** attempt combined multi-rule edits (combinatorial; D6
   says narrow one rule). This is why naming all claimants in the diagnostic
   (roadmap polish (i)) belongs in the same block: the partition/no-fix report for
   a >=3 overlap must list every claimant, or the user cannot act.

4. **Generalize `partition_for_rule` to the overlap conflict unit.** Two edits:
   the `affected` predicate keys on `OverlappingRules` for the conflict instead
   of `AmbiguousRule` (line 1098), and the candidate seed is the overlap seed set.
   Partition **per overlap-conflict-instance `(rule_a, rule_b, track)`**, not per
   rule — that is the unit the diagnostic reports and a fix resolves; carry
   `rule_b`/`track` params on the `SuggestionPartition` output mirroring the
   `OverlappingRules` diagnostic. For the common two-required-single-file case the
   partition has **zero resolving groups** — the report must then say
   *explicitly* "no narrowing resolves this without dropping a required track;
   make one rule `optional`, or make the two rules' intents disjoint" (none of
   which is expressible in the narrow-only grammar). This is a small addition to
   the partition renderer: an empty-groups / "unresolvable overlap" branch.

## Recommendation + tradeoff

**Recommendation:** **Policy 3 (symmetric generate-for-all-claimants, acceptance-
filtered), with a NOT-polarity seed drawn from the shared track's property vector,
and Policies 2-then-1 used only as the deterministic *rank tiebreak* between
surviving narrowings on different rules** (broader rule first, then lower rule
index). Reuse `candidates_for_rule`/`resolves_without_regression`/
`partition_for_rule` with the four touch-points above; add no new edit variant
and no new selection mechanism.

**Rationale (one line):** feasibility (optionality / batch structure), not a
precedence guess, determines which rule can be narrowed, so letting the existing
acceptance criterion select — exactly as the `AmbiguousRule` path already does —
is both the most correct and the most consistent choice.

**Tradeoff (one line):** the common case (two required rules, one file) yields
*no* suggestion and only an explicit no-fix report — honest, but a user expecting
a one-click fix for every overlap will find overlap "less helpful" than ambiguity,
which is a genuine and unavoidable property of the narrow-only v1 grammar, not a
gap to paper over.

## Acceptance test cases (for the recommended policy)

Harness: `plan_one(profile_yaml, file_name, ident_json)` from
`crates/muxsmith-core/tests/planner_resolution.rs`; fixture `SERIES` =
`fixtures/identify/series-s01e01.json` (video id0 `V_MPEG4/ISO/AVC`; audio id1
eng "English"; subtitles id2 `forced_track:true` "English forced"; subtitles id3
`flag_hearing_impaired:true` "English SDH"; both subs codec_id `S_TEXT/UTF8`).
Batch cases need a two-file harness (extend `plan_one`); they feed the property
test.

### TC-A - optional yielding rule → single NOT-narrowing emitted (the resolvable case)
```yaml
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles, forced_track: true } }            # R0 (required) → id2 only
    - match: { exact: { codec_id: 'S_TEXT/UTF8', forced_track: true } }    # R1 (optional) → id2 only
      optional: true
```
Baseline: `OverlappingRules(rule_a=tracks[0], rule_b=tracks[1], track=2)`, plan
dropped. Expected suggestions: one or more, **all editing `tracks[1]`** (the
optional rule); every candidate editing `tracks[0]` is rejected (it regresses to
`MissingTrack` — R0 has no other target). Top-ranked (flag rank 0):
`AddNotExact{property: forced_track, value: "true"}` on `tracks[1]`, fragment
`# tracks[1] - add:` / `match: { not: [ { exact: { forced_track: true } } ] }`.
Post-application: R1 matches nothing → optional satisfied; R0 claims id2 alone;
overlap gone; no new diagnostic; **plan produced**. Proves: symmetric generation,
acceptance auto-selects the optional rule, NOT-polarity seed from the shared
track.

### TC-B - two required rules, single file → NO suggestion, explicit no-fix report (the common case)
```yaml
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }                       # R0 (required) → id0 only
    - match: { exact: { codec_id: 'V_MPEG4/ISO/AVC' } }       # R1 (required) → id0 only
```
(the existing `overlapping_rules_when_two_rules_claim_one_track` fixture.)
Baseline: `OverlappingRules(rule_a=tracks[0], rule_b=tracks[1], track=0)`.
Expected: **zero `Suggestion`s**; a `SuggestionPartition` with **zero resolving
groups** → the "unresolvable overlap" branch, stating no narrowing resolves
without dropping a required track (remediation: make one `optional` or disjoin
intents). Proves: the engine never emits a regressing suggestion, and the
honest-failure path fires for the common case.

### TC-C - regression rejection: NOT that also drops a legitimate match (batch)
Two files. File A = `SERIES`. File B = a fixture whose only subtitles track is
`forced_track:true`, `language: jpn`, and which R1 (optional) is *meant* to keep.
Rules as TC-A. The candidate `AddNotExact{language, "eng"}` clears the overlap on
A (drops R1's id2) but on B drops R1's legitimate jpn... (choose values so the
narrowing that clears A also kills a wanted B match) → new `MissingTrack`/empty
on B → (b) rejects → falls to `partition_for_rule`, which groups A under the
language-agnostic NOT (e.g. `forced_track`) that resolves A in isolation. Proves:
multiset "nothing-new" guard rejects batch-unsafe narrowings; partition reuses
the per-file resolving candidate. (Batch harness; also the property-test seed.)

### TC-D - >=3 claimants → single narrowing insufficient → partition names all claimants
```yaml
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: video } }                       # R0 → id0
    - match: { exact: { codec_id: 'V_MPEG4/ISO/AVC' } }       # R1 → id0
    - match: { exact: { default_track: true, type: video } }  # R2 → id0
```
Baseline: `OverlappingRules` on track 0 with **three** claimants (0,1,2).
Expected: no single narrowing survives (removing one leaves a 2-rule overlap →
gate (1) rejects); `SuggestionPartition`/no-fix report that **lists all three
claimants** (ties to roadmap polish (i)). Proves: graceful degradation and the
all-claimants requirement.

## Open taste calls for the human

Technically determined (not taste; state in the memo as forced):
- Seed candidates from the shared track's property vector, not a >=2 set (else
  zero candidates).
- Gate acceptance on the target overlap *instance* disappearing, not on
  `AmbiguousRule` / edited-rule index.
- >=3 claimants → single narrowing cannot resolve → partition/no-fix.
- The common two-required-single-file overlap has no v1 suggestion; the report is
  the deliverable there.

Genuinely Şenol's call (underdetermined by correctness):

1. **Symmetric vs precedence-restricted selection.** My rec: symmetric (emit
   narrowings for whichever claimant survives). Alternative: assert a rule
   precedence (later/broader yields) and only ever suggest editing that one. This
   is a product-philosophy call — does Muxsmith stay precedence-free (current
   model) or introduce "the later/broader rule is the one to fix"? Recommend
   symmetric, but it is his to decide whether that is "too many suggestions".

2. **Rank tiebreak between surviving narrowings on *different* rules.** broader-
   rule-first vs later-rule-first vs plain rule-index order. Pure ordering; all
   deterministic. My rec: broader-first, then rule-index. His taste.

3. **NOT-only vs also emit positive `AddExact` for overlap.** My rec: NOT-only
   (positive can't redirect, only adds near-tie noise). If he prefers the overlap
   path to mirror the ambiguous path's both-polarity generation for uniformity,
   that is a defensible taste choice (harmless, filtered by acceptance, costs
   ranking clarity).

4. **Narrowing an optional rule to *empty* as an accepted "fix".** TC-A's
   suggestion effectively *disables* R1 on that file (it now matches nothing).
   That is a valid resolution but silently removes the rule's effect. Taste:
   accept it as a first-class suggestion (my rec — it is the only feasible fix and
   the user sees the exact edit), or down-rank / annotate it as "this disables the
   rule here" so it does not read as a free win.

5. **Empty-groups partition wording.** How loud/prescriptive the "unresolvable
   overlap" report is — bare "no narrowing resolves this" vs an actionable hint
   listing the out-of-grammar remedies (make optional / disjoin / reorder).
   Presentation taste; recommend the actionable form since the user otherwise has
   no next step.

---
Analysis file: `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-5.5/d33-analysis.md`
