# Seed 10 — [whole-branch M2] overlap_conflicts re-parses claimants from rendered param string

**Verdict: CONFIRMED** (at HEAD 2f17880)

## Current state on disk

- **Producer** — `crates/muxsmith-core/src/planner.rs:689-698` (`plan` per-file diagnostics): the structured claimant indices (`rules: &Vec<usize>` from the `claims` map) are rendered into the display param `rules` as `"tracks[0], tracks[1], ..."` via `format!("tracks[{r}]")` + `join(", ")`, then discarded.
- **Consumer** — `crates/muxsmith-core/src/planner.rs:1869-1890`, `overlap_conflicts()`: recovers the indices by `d.params.get("rules").map(|r| r.split(',').filter_map(rule_index_of).collect())` (lines 1878-1882), i.e. core re-parses its own display-formatted param. `rule_index_of` (line 1971) is a substring scan for `tracks[N]`.
- `Diagnostic.params` is `BTreeMap<String, String>` documented as *renderer template interpolation values* (`report/mod.rs:194-197`), so this is a round-trip through a human-facing format. The parallel AmbiguousRule path parses `config_path` instead, which is a defined machine identifier — not the same smell.
- ROADMAP.md:186-187 records exactly this item as a deferred named input to the idiomacy review ("overlap_conflicts re-parses claimants from the rendered param string (whole-branch M2)"). No commit since the whole-branch verdict touched either site.

## Failure mode

No type-level link between producer and consumer: a change to the `$rules` display format (separator, wording, localization) silently yields empty `claimants`, killing symmetric overlap-suggestion generation (D33) with no compile error and no diagnostic.

## Recommended fix

Tag: **idiom**. Carry the claimants structurally:

1. Add to `Diagnostic` (`report/mod.rs`): `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub claimants: Vec<usize>` — report JSON unchanged for every other diagnostic code.
2. At the production site (planner.rs:693-696), set both the display param and the structured field from the same `rules` slice (one builder, e.g. `.with_claimants(rules)`, so they cannot diverge).
3. In `overlap_conflicts` (planner.rs:1878-1882), replace the split/re-parse with `claimants: d.claimants.clone()`; the `rules` param stays purely presentational. `rule_index_of` remains for the `config_path` call sites.

Estimates: lines_cut ~0 net (removes the 5-line parse, adds field + doc + builder of similar size), deps_cut 0. Spec touch-point: the report schema (§5.2/8.4) gains one optional field; worth a one-line spec note.
