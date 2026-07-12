# Task 13 report: D6 mechanical completion

Status: **DONE_WITH_CONCERNS**. Four steps, four commits, full gate green. One
substantive design note on the step-4 partition grouping key (brief vs D6),
surfaced below rather than reconciled silently, plus a smaller note that step 1
needed more than the brief predicted.

Branch `plan55-stream-e`, worktree `/home/senol/Git/Muxsmith/.worktrees/stream-e`.

## Commits (one per step, `feat(suggest): ...`, unsigned)

| Step | Commit | Subject |
|---|---|---|
| 1 | `31c2135` | external-source rules get suggestions (#12ii) |
| 2 | `b1f611c` | codec and id become narrowing dimensions (R1 iv) |
| 3 | `2ddc964` | diagnostic signature is a multiset (R1 v) |
| 4 | `0ddd945` | no-single-fix partition report (#5) |

## Files changed

- `crates/muxsmith-core/src/planner.rs` — `rule_source_ident` helper; codec/id
  candidate dims; multiset `diag_signature` + count-based
  `resolves_without_regression`; `PARTITION_GROUP_CAP` + `partition_for_rule`;
  `#[derive(Clone)]` on `Candidate`; new `#[cfg(test)] mod tests`; `suggest`
  rustdoc updated.
- `crates/muxsmith-core/src/report/mod.rs` — new `DiagCode::SuggestionPartition`.
- `crates/muxsmith-core/tests/suggestions.rs` — one integration test per step 1,
  2, 4.
- `locales/en/diagnostics.ftl` — `suggestion-partition` message (kind selector,
  EN-only per C2).
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` — §5.2 catalog row
  for the new code (see concern 4).

## Per-step: what and TDD evidence

Focused runs while iterating; full gate once at the end. Every step: failing
test committed-then-implemented in the same commit.

### Step 1 — external-source rules get suggestions (#12ii)

Removed the `SourceCfg::External` skip and made candidate generation
source-aware. The brief's "just remove the skip" is necessary but **not
sufficient**: `candidates_for_rule` identified `primary.path` unconditionally,
so for an external rule it drew discriminators from the primary's tracks, not
the donor's (where the ambiguity lives). Added `rule_source_ident(rule, primary,
id)` mirroring `resolve_file`'s source resolution (keyword -> primary; external
-> the single located+identified donor, `None` on zero/ambiguous/unidentifiable),
and candidate generation now reads that.

- RED: `ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule`
  ```
  cargo test -p muxsmith-core --test suggestions ambiguous_external_source_rule_gets_suggestions_like_a_primary_rule
  ... panicked ... an external-source rule must get suggestions like a primary one
  test result: FAILED. 0 passed; 1 failed
  ```
  (First assertion — donor conflict is AmbiguousRule — passed; the empty
  suggestions assertion failed, confirming the skip *and* the wrong-file
  candidate source were both gaps.)
- GREEN: `test result: ok. 6 passed` (full suggestions suite).

### Step 2 — codec and id are narrowing dimensions (R1 iv)

`candidates_for_rule` iterated only the nested `properties` map plus the `type`
pseudo-prop. Pushed `codec` and `id` (top-level `Track` fields, both in the
capability model) onto the candidate-property list alongside `type`.

- RED: `ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions`
  (fixture: two subtitle tracks identical except `codec` and `id`; `codec_id`
  deliberately equal so only codec/id discriminate)
  ```
  ... panicked ... expected both a codec-based and an id-based suggestion, got []
  test result: FAILED. 0 passed; 1 failed
  ```
- GREEN: `test result: ok. 7 passed`.

### Step 3 — diagnostic signature is a multiset (R1 v)

`diag_signature` collapsed diagnostics sharing `(code, config_path, file)` into
a `BTreeSet`, so a second copy of a pre-existing diagnostic read as no
regression. Now `BTreeMap<sig, count>`; `resolves_without_regression` compares
counts (`sim_count <= base_count` per signature) — D6 acceptance criterion (b)'s
"compare multisets before/after".

- RED (new `#[cfg(test)] mod tests` in planner.rs, testing the private
  `diag_signature`/`resolves_without_regression`):
  `duplicate_signature_diagnostic_is_a_regression_not_a_collapse`
  ```
  cargo test -p muxsmith-core --lib planner::tests::duplicate_signature_diagnostic_is_a_regression_not_a_collapse
  ... panicked ... a newly duplicated diagnostic must count as a regression
  test result: FAILED. 0 passed; 1 failed
  ```
- GREEN: `test planner::tests::duplicate_signature_diagnostic_is_a_regression_not_a_collapse ... ok`.

Test location note: a **unit** test (private fns), not an end-to-end acceptance
scenario. This is deliberate — see concern 2.

### Step 4 — no-single-fix partition report (#5)

When no candidate resolves a conflicted rule batch-wide (`accepted.is_empty()`),
`partition_for_rule` groups the affected files by the per-file refinement that
resolves each in isolation (top-ranked candidate whose single-file re-plan
passes `resolves_without_regression`). One `SuggestionPartition` info
diagnostic per group (`kind=group`, params `fix` = the rendered YAML fragment,
`files` = sorted display paths, `count`), deterministic (BTreeMap key order),
capped at `PARTITION_GROUP_CAP = 5` with a `kind=overflow` note carrying
`dropped` — mirroring `SuggestionsCapped`'s never-silent cap.

- RED: `no_single_fix_produces_a_two_group_partition` (file A: two subs separable
  only by `forced_track`; file B: two subs separable only by `language`, with
  **disjoint** subtitle ids 3/4 vs 1/2 so no single `id` narrowing resolves both
  -> genuine no-single-fix)
  ```
  ... panicked ... expected a two-group partition, got []
  left: 0  right: 2
  test result: FAILED. 0 passed; 1 failed
  ```
  (The `batch.suggestions.is_empty()` assertion passed already, confirming the
  batch-wide engine produces nothing here; only the partition was missing.)
- GREEN: `test result: ok. 8 passed`.

New `DiagCode::SuggestionPartition` + its EN Fluent message were added *with* the
test (catalog contract, keeps `every_diag_code_has_a_catalog_message` green), so
the RED is a clean assertion failure, not a missing-symbol compile error; the
emitting logic is the GREEN delta.

## Gate results (full, from worktree root, before the step-4 commit)

`pnpm install --frozen-lockfile` run once (node_modules was absent).

| Gate | Result |
|---|---|
| `cargo fmt --all --check` | clean (after one `cargo fmt --all`) |
| `cargo clippy --workspace --all-targets -- -D warnings` | CLEAN |
| `cargo test --workspace` | all pass (core 105 lib + 8 suggestions + 49 planner_resolution + rest; cli incl. catalog_completeness) |
| `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| `pnpm lint` | clean |
| `pnpm build` | ok |
| `pnpm check:i18n` | ok (12 pre-existing gui-* unused-key warnings, unrelated) |
| `pnpm test:e2e` | 3 passed |

## Self-review findings / concerns

1. **Partition grouping key: brief vs D6 (the flagged §5.3 tension).** The brief
   directed "group files by the per-file suggestion that would have fixed them";
   D6 step 6 says "partition by conflict signature (the property-vector multiset
   of the conflicting tracks)". I implemented the brief's per-file-suggestion
   key. This satisfies §5.3's top-level requirement ("list the files requiring
   different resolutions") and honors D6's "reuses the discriminator diff from
   step 2" (the per-file fix *is* a discriminator candidate), but it is **not**
   D6's literal grouping key. Consequences: my grouping is **coarser** (two files
   whose top-ranked fix is identical merge, even if their full property vectors
   differ — D6 would separate them) and **more actionable** (each group is
   labeled with an applicable refinement, not an opaque property-vector
   multiset). No §5.3 contradiction, but a conscious divergence from D6's text —
   surfacing per the task instruction rather than reconciling silently. If the
   controller wants D6-literal signature grouping, the key changes from
   `yaml_fragment(top candidate)` to a serialized conflicting-track property
   multiset; the rest of the machinery is unaffected.

2. **Multiset (step 3) is not triggerable through the acceptance path in v1.**
   Within v1's narrow-only single-rule edit grammar, a narrowing candidate
   cannot introduce a duplicate-signature regression: narrowing rule `ri`
   reduces its own overlaps and matches, `AmbiguousRule`/`MissingTrack` are
   unique per (rule, file), and other rules are untouched. So the fix is a
   correctness-completeness alignment with D6 criterion (b), tested directly on
   `resolves_without_regression` (unit) rather than end-to-end. It becomes
   load-bearing the moment the edit grammar grows (OverlappingRules suggestions,
   multi-rule edits).

3. **`id` makes the no-single-fix case rarer.** Because `id` is unique per track,
   an `id` narrowing almost always resolves a single file, and resolves
   *batch-wide* whenever the target track ids align across files — a
   semantically poor but valid "fix" that pre-empts the partition. The partition
   therefore fires only when no candidate (id included) works batch-wide; the
   step-4 fixture forces this with disjoint subtitle ids. This is faithful to D6
   (id is a legitimate, low-ranked discriminator) but worth the controller
   knowing: the partition is a genuine last resort. Relatedly, a "no per-file
   fix" file cannot occur (id always provides one), so no affected file is
   silently dropped from the partition; the code still skips a fixless file
   defensively.

4. **Spec §5.2 catalog row added (beyond the brief's stated file set).** The
   `DiagCode` rustdoc states "every variant corresponds to exactly one row of
   the spec 5.2 catalog table". Adding `SuggestionPartition` without a row would
   break that documented invariant, so I added the row. This is a spec-doc edit
   the brief did not name (it named `report/` + `diagnostics.ftl`); called out
   for visibility. No test enforces spec-table parity, so it is consistency, not
   a gate.

5. **Overflow note reuses the `suggestion-partition` key with a `kind` selector**
   (`group`/`overflow`) rather than a second `DiagCode` — matching the
   `invalid-template` selector precedent and C1's singular "your new DiagCode".
   `SuggestionsCapped` was not reused for the group cap: its message hardcodes
   "capped at 3" and its meaning is per-rule candidate truncation, not
   group-count capping.

6. **Cross-task awareness.** C1: the new `DiagCode::SuggestionPartition` will need
   its param fixture from the exhaustive-guard task at merge (params: `kind`,
   and per-kind `fix`/`files`/`count` or `dropped`). C2: the new Fluent
   message(s) are EN-only as required. C2's "any overflow note key" resolved to
   a `kind=overflow` variant of the same key, not a separate key.
