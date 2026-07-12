### Task 13: D6 mechanical completion - partition report, external suggestions, codec/id narrowing, multiset (#5, #12ii, R1 iv+v)

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs:985-1200` (suggestion engine section; :1014-1015 external skip; `diag_signature`)
- Modify: `report/` + `locales/en/diagnostics.ftl` (partition report rendering)
- Test: suggestion-engine tests (`planner_resolution.rs` neighborhood)

- [ ] Step 1 (external, #12ii): failing test: ambiguous external-source rule gets suggestions like a primary rule. Remove the `SourceCfg::External` skip at :1015; the algorithm is source-agnostic (verify batch validation handles donor identify - it does, suggestions re-run dry-run semantics).
- [ ] Step 2 (codec/id, R1 iv): failing test: an ambiguity resolvable only by `codec` or `id` yields a suggestion using that dimension. Add both to the candidate-dimension list.
- [ ] Step 3 (multiset, R1 v): failing test: two identical diagnostics on different tracks do not collapse; change `diag_signature`'s BTreeSet to a sorted Vec or BTreeMap<sig, count>.
- [ ] Step 4 (partition report, #5): when zero suggestions survive batch-wide, emit the spec-§5.3 partition: group files by the per-file suggestion that would have fixed them; render as info diagnostic `suggestion-partition` listing group -> files. Failing test: two files needing different narrowings produce a two-group partition. Cap groups at 5 with a `SuggestionsCapped`-style overflow note (consistent with D6's existing cap philosophy).
- [ ] Full gate; commit per step `feat(suggest): ...`.

