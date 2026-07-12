### Task 14: proptest for the correctness core (#1)

**Files:**
- Modify: `crates/muxsmith-core/Cargo.toml` (dev-dep `proptest`, exact-pin per toolchain doctrine; registry-verify the current version, do not trust memory)
- Create: `crates/muxsmith-core/tests/prop_matcher.rs`, `prop_language.rs`, `prop_planner.rs`

Properties (from the walkthrough decision + D6):
- [ ] Match algebra: `not(not(e)) == e` over arbitrary track/expr; `any([e]) == e`; `any` is order-insensitive; no panic on arbitrary UTF-8 in values/patterns (invalid regex must be impossible post-validation - property runs on validated exprs).
- [ ] Language: `normalize(normalize(x)) == normalize(x)`; canonical equality symmetric; bare-ISO and tagged forms agree with the language-index rows (strategy over the index's own entries + random well-formed tags).
- [ ] Planner: same input -> byte-identical plan (determinism); rendered-name invariants (D4: no separator injection, template output stable); D6 property: an accepted suggestion, applied to the profile, survives the next dry-run without new diagnostics (strategy: generate small ambiguous profiles, take the engine's own suggestion, re-plan).
- [ ] Deterministic seeds in CI (`PROPTEST_CASES` default fine; check in `proptest-regressions/` if any fall out).
- [ ] Full gate; commit `test(core): property-based tests for matcher, language, planner (spec §10, #1)`.

