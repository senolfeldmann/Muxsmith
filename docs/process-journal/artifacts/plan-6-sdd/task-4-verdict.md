# Verdict (extracted from the reviewer transcript at write-time)

I have everything needed. All load-bearing facts verified against source, the design table, and the house files. Writing the review.

---

### Spec Compliance

- ✅ **All 17 rows present, each with three attributes naming one function.** Walked D48's table (`:1558-1576`) row by row against `model.rs`. Every row carries `#[serde(default…, skip_serializing_if = …)]` + `#[schemars(extend("default" = to_value(F()).unwrap()))]`, and all three name the same `F` per row. Concretely: the 5 `Profile`-level struct fields (`output`/`attachments`/`chapters`/`tags`/`title`, model.rs:59-87) and the 8 other generic rows use `is_default` + `to_value(<Type>::default())`; the 4 divergent rows use `default_true`/`FilenameCfg::keep`/`SourceCfg::primary`/`drop_policy` consistently across all three attributes.
- ✅ **The two silent-hazard rows use their bespoke predicates, not the generic.** `TracksCfg.unmatched` → `is_drop_policy` (model.rs:416, calling `drop_policy()`); `Input.recursive` → `is_default_true` (model.rs:122, calling `default_true()`). Verified the necessity from source: `KeepDrop::default()` = `Keep` (model.rs:245-251, `#[default] Keep`), so `tracks.unmatched`'s `drop` default genuinely differs from `T::default()` and a generic predicate would silently omit `unmatched: keep`. Symmetrically confirmed `attachments.unmatched`, `tags.global`, `tags.track` correctly stay on `is_default` because their default *is* `Keep`.
- ✅ **The two compiler-caught divergent rows correctly use bespoke predicates.** `FilenameCfg` (model.rs:188-190) and `SourceCfg` (model.rs:287-289) derive lines omit `Default`, confirming `is_default<T: Default>` cannot instantiate (`E0277`); `is_keep_filename` and `is_primary` are the correct carve-outs.
- ✅ **Struct-valued fields accepted as `{}`, not patched.** Guard 2's expected table (profile_save.rs) lists `output`/`attachments`/`tags` → `json!({})`; `chapters`/`title` → `json!("keep")` (enums serialize their keyword default). Matches D48 `:1653-1671` exactly.
- ✅ **Guard 1 present with the brief's bodies.** `all_non_default_fields_survive_the_round_trip` and `the_core83_passthrough_profile_survives_a_save` are substantively verbatim; the only delta is rustfmt wrapping the first test's `assert_eq!` across three lines (report discloses this). The `core-83` test is byte-identical to the brief.
- ✅ **Guard 1 fixture sets all 17 to non-default.** Cross-referenced each fixture row against its default: `recursive:false`(≠true), `filename:template`(≠keep), `on_collision:skip`(≠error), `tracks.unmatched:keep`(≠drop), `source:external`(≠primary), Locator `recursive:true`/`case_sensitive:true`(≠false), `optional:true`(≠false), `attachments.unmatched:drop`(≠keep), `attachments.rules:[one]`(≠[]), `chapters:drop`, `tags.global:drop`, `tags.track:drop`, `title:clear`. The 5 `Profile`-level struct rows are non-default transitively (each block carries a non-default child). All 17 covered.
- ✅ **Guard 2 present in the house shape, all 17 rows.** `schema_defaults_match_the_serde_defaults` (profile_save.rs): `[…; 17]` compile-time length, length-check-first (against an independent live count), then row-by-row via `Value::pointer` — mirroring `capability/mod.rs:177` `settable_maps_to_mkvmerge_options` (const EXPECTED, `assert_eq!(len)`, then loop).
- ✅ **No post-processing in save.rs; v1 spec untouched.** Commit touches only `model.rs`, the fixture, and `profile_save.rs` (confirmed). Omission is purely derive-side.
- ✅ **Typography ASCII-clean** across all three changed files (no em/en-dash, smart quotes, ellipsis, NBSP).

### Adjudications

**Q1 — valid observation.** The baseline is evidence that nothing is omitted *pre-D48* (design `:90-92`, "it holds today"); its subject is the serializer's behavior, and `git stash` physically reverted `model.rs` to its pristine (no-skip) state on disk before the run, so the compiler compiled and the test exercised exactly the unmodified serializer. Drafting-before-stashing is a workflow-ordering detail with no bearing on the state under test — the stash produces the pristine tree regardless of when the edit was typed. The pathspec-scoped stash (only `model.rs`, leaving the new guard-1 tests and fixture in place) is what made the observation possible at all; a bare `git stash` would have stashed the tests too. This is ⚠️ not independently re-verifiable from the final commit (stash/pop leaves no trace), but the reported PASS output is consistent with what the design proves must hold by construction, so the baseline's real value — that the fixture parses and the tests are wired — is genuinely demonstrated. Not compromised.

**Q2(a) — within remit, and correctly *not* a removal decision.** The mutation experiment was run-and-reverted, never committed; the deliverable is unchanged. It is a verification action of exactly the kind house doctrine mandates (`conventions.md`: a check whose passing result is an absence is trusted only after it is made to fire once; process-conventions `:335` treats vacuous/negative assertions as a recurring, named defect). The design's trigger-2 (`:1730-1735`) reserves the *removal* of guard 2 as "its own decision with evidence." The implementer ran the experiment, found guard 2 fires, and therefore kept it — they did not arrogate the removal decision. That is the diligent read, not scope creep.

**Q2(b) — the observation settles trigger 2 as "guard 2 stays," but reject the "design phase was wrong" framing.** I verified the mechanism against the diff: guard 2's expected side is **hand-written literals** (`json!("drop")`, `json!(true)`, …), *not* `to_value(F())`. Mutating `TracksCfg.unmatched`'s `extend` from `drop_policy()` (→`"drop"`) to `KeepDrop::default()` (→`"keep"`, since `KeepDrop::default()` = `Keep`, confirmed at model.rs:245-251) changes the schema's `$defs/TracksCfg/properties/unmatched/default` to `"keep"`, which then diverges from the literal `"drop"` → red (`left: "keep", right: "drop"`, exactly the reported panic). So the guard is genuinely falsifiable and stays.

But the design was **not wrong** about the tautology. Its analysis (`:1706-1710`) was construction-specific: it targeted a guard whose expected side is *derived from the same function the extend calls* (`to_value(F()) == to_value(F())`). The implementer escaped that verdict by building a *different* instrument — independent literals — which is precisely the form the house doctrine prescribes (`process-conventions:335`: "size and derivability are not exemptions" for writing literals out; `:380`: the guard was restored with "the tautology analysis re-aimed onto a falsifiable trigger"). The guard stays because it was built in the non-tautological form the doctrine wanted, not because the tautology reasoning was refuted. "For good" holds for this construction; it would revert to a removal candidate only if someone rewrote the expected side into the derived form. The implementer's own report framing gets this right ("a literal table is the independent truth an extend annotation can still drift from") and does not overclaim.

### Strengths

- The one-function-across-three-attributes mechanism is applied uniformly and correctly across all 17 rows, including the delicate `KeepDrop`-typed rows where the same type carries opposite defaults in two structs — the exact trap D48 warned about, navigated cleanly.
- Guard 2's independent-count cross-check (`count_defaults` over `Profile.properties` + every `$defs` type = 17) catches both a missing `extend` (subset) and a stray one (superset), not just per-row values — stronger than the brief required.
- Both guards' ability-to-fail was exercised (step-4 red proof for guard 1; the extra guard-2 mutation), aligning with the house's acute sensitivity to vacuous/negative checks.
- Every bespoke predicate carries rustdoc naming the exact failure mode it prevents (`E0277` vs. silent inversion/destruction), and the fixture annotates each row inline with its default (`# default drop`), making the non-default claim auditable at a glance.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None. The one plan-mandated known limitation — the step-6 snapshot check is vacuous for value-level defects on these paths — is acknowledged in the report and explicitly *not* relied upon; the load-bearing proofs (guard 1, the step-4 red, guard 2) all exist and were exercised.

#### Minor (Nice to Have)
- Self-review bullet "No design latitude was exercised" (report:214-217) slightly understates reality: the derived-vs-literal choice for guard 2 was a genuine fork that the design itself contested. The implementer resolved it in the doctrine-correct direction *and* documented the reasoning prominently in both the report and the test's rustdoc, so this is a framing quibble on the self-characterization, not a code defect.
- Guard 2 uses `let expected` rather than the house `const EXPECTED` (capability/mod.rs:179). This is forced — `serde_json::Value` is not const-constructible — so it is the only possible form, noted only for completeness.

### HARVEST

- **Convention candidate (generalize D48's mechanism):** For any serde+schemars field whose serde default is a *named function* rather than `Default::default()`, its `skip_serializing_if` predicate and its `schemars(extend("default"=…))` must both call that same function; a generic `is_default` on such a field is a silent-data-loss defect (compiles when the types line up, omits a non-default value that reloads inverted/destroyed). This is D48 stated as a reusable rule, not a one-off.
- **Reinforcement of `process-conventions:335` (write literals, don't derive):** A guard whose expected side is derived from the same source it checks is vacuous; independent literals are what make it falsifiable. Observed again here as the deciding factor for guard 2's soundness — a second live instance of the "size/derivability are not exemptions" ruling.
- **Practice observed (fire-the-negative-once):** Both new guards had their red state produced deliberately before being trusted (step-4 break; guard-2 `extend` mutation). Consistent with the eager house value on absence/negative checks; worth noting as an established, repeated implementer practice rather than a per-task ask.
- **Readable-fixture pattern:** Annotating each row of a "set-everything-non-default" fixture with its default inline (`# default drop`) makes the fixture self-auditing. Candidate convention for guard/round-trip fixtures.

### Assessment

**Task quality:** Approved

**Reasoning:** All 17 rows are complete and internally consistent, the two silent-hazard predicates are correct against the actual `KeepDrop`/`Default` facts I verified in source, both guards ship in falsifiable form with their failure modes exercised, and no post-processing or v1-spec touch leaked in. No Critical or Important findings; the two Minors are cosmetic.
