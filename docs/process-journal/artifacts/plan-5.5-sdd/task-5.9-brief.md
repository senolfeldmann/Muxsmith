### Task 5.9: Locator extension validation - spec §4.6 completion (added 2026-07-11, T5-review routing)

Spec §4.6:198 mandates `extensions: [string] # validated against mkvmerge --list-types` for external-source LOCATORS too; T5 delivered the input.extensions half (walkthrough #3's scope). Stream B, after T7 (planner/validate/model region).

**Files:**
- Modify: `crates/muxsmith-core/src/planner.rs` (extend the T5 batch walk to locator extensions - recursive walk over track rules / chapters / attachments `add` entries, mirror `walk_exact_languages`' traversal)
- Modify: `crates/muxsmith-core/src/profile/model.rs:254-256` (the "validated ... like `input.extensions`" doc claim is actively false until this task lands; make it true)
- Test: planner batch-validation tests (locator with unknown extension in a track rule, a chapters entry, an attachments entry; batch continues, one UnknownExtension warning each, dedup semantics same as T5's walk)

- [ ] Failing tests first (three locator positions), implement the recursive walk reusing T5's `validate_extension_values` core, make the model.rs:254 doc true. Full gate; commit `feat(planner): validate locator extensions against --list-types (spec §4.6)`.

