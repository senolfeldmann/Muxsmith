# Plan 2 design decisions

Status: FINAL 2026-07-09. Şenol confirmed no additional context is coming, so
these decisions are accepted, not provisional. The normative rules (D1-D5) are
folded into the v1 design spec (sections 4.4, 4.8, 5.2, 5.4, 9); this memo
remains the decision record (rationale) and the home of the D6 suggestion-engine
algorithm. On any spec/memo conflict the spec wins per repo convention.

Grounding: v1 design spec (authoritative), Plan 1 code as committed at 33d6587
(`capability/mod.rs`, `profile/model.rs`, `template.rs`, `report.rs`).

## D1: `codec_kind` is `exact`-only

**Decision.** `codec_kind` may appear only under `exact`. Under `substring` or
`regex` it is a config-time error: new diagnostic `CodecKindExactOnly`
(`codec-kind-exact-only`, error, validate-time).

- Rationale: `codec_kind` is a curated alias over `codec_id` prefix sets; a
  pattern over the alias token is ill-defined (against the token? the prefix
  set? the underlying `codec_id`?). Pattern matching over real data is already
  available via `codec_id`.
- Tradeoff: one more diagnostic code; users who wrote `substring: {codec_kind: ...}`
  get an error instead of silent never-match, which is the point.
- Why a new code: `NotStringProperty` would lie (`codec_kind` IS String-typed
  in the capability model); the message must say "use exact, or match
  codec_id".
- Spec amendment: 4.4 conveniences (`codec_kind` entry), 5.2 table.

## D2: Value-domain validation (`type: vdieo` must not pass)

**Decision.** New diagnostic `InvalidPropertyValue` (`invalid-property-value`,
error, params: property, value, allowed/hint). Emitted for `exact` condition
values outside a closed domain. One code, two emission times, complete
classification of value-domained properties:

| Property | Domain source | Checked at |
|---|---|---|
| `type` | pinned identification schema enum (build-time, via xtask codegen) | validate (config-time) |
| `codec_kind` | curated alias table in `capability` | validate (config-time) |
| `language` | `mkvmerge --list-languages` (runtime) | plan time (dry-run/run), once per config path, not per file |
| `input.extensions` / locator `extensions` | `mkvmerge --list-types` (runtime) | already specced (4.2), unchanged |
| booleans / numerics | type system | already covered by `ValueTypeMismatch`, unchanged |
| `sub_charset` | open-ended (iconv) | deliberately lenient per 4.4, unchanged |

- Rationale: a typo'd enum value today surfaces as `MissingTrack` on every
  file with a misleading hint; strict-explicitness doctrine says config bugs
  are config-time errors.
- Tradeoff: version skew. validate has no mkvmerge access (5.5), so the
  build-time domains come from the pinned schema; a genuinely new enum value
  from a newer mkvmerge would be rejected at validate time with no v1 escape
  hatch. Accepted: track-type enums are extremely stable, and the skew story
  (9.2) covers unknown properties, not enum values.
- Scope limit: `exact` only. substring/regex values that can provably never
  match a domain value are subsumption-lint territory, out of v1 scope.
- Spec amendment: 5.2 table, 5.4 static lint list, 9 (note on pinned-schema
  enum extraction).

## D3: Unify locator/output path types on `PathBuf`

**Decision.** `Locator.path` changes from `String` to `PathBuf`, matching
`output.directory`.

- Rationale: both are filesystem paths; `PathBuf` is the honest type and makes
  planner joins (`primary_dir.join(&locator.path)`) natural. serde/schemars
  treat `PathBuf` as string, so profile format and JSON Schema are unchanged.
- Tradeoff: none at runtime; purely a model-consistency fix. Portability note
  for docs: profiles should use forward slashes; Windows APIs accept them.
- Spec amendment: none (4.6 already says "string" at format level, which stays
  true).

## D4: Rendered-filename invariant in the planner

**Decision.** The planner re-checks the RENDERED output filename (after
interpolation), independently of the config-time `PathSeparatorInTemplate`
check on template text. Two new per-file, error-severity diagnostics:

- `PathSeparatorInRenderedName` (`path-separator-in-rendered-name`): rendered
  name contains `/` or `\`, checked on all platforms (a profile must fail
  identically everywhere).
- `EmptyRenderedName` (`empty-rendered-name`): rendered stem is empty (the
  ".mkv appended if missing" rule would otherwise silently produce the hidden
  file `.mkv`) or the rendered name is `.` / `..`.

- Rationale: today all template fields are basename-derived, so separators
  cannot occur; the check is a cheap invariant that becomes load-bearing the
  day a field source beyond basenames is added. Guard the invariant, not the
  induction proof over field sources. The empty case is reachable today via
  `template: ""` or a template consisting only of fields that render empty.
- Tradeoff: two more catalog entries; both conditions are distinct user-facing
  situations, so separate codes beat a reason-param multiplexed message
  (Fluent catalog stays one message per code).
- Spec amendment: 4.8, 5.2 table.

## D5: `Template::parse` brace semantics, locked

**Decision.** Lock current behavior with tests, no code change intended:
`{{` and `}}` are escapes; a lone `}` is a literal; a lone `{` with no closing
`}` is `InvalidTemplate`. If the lone-`}` test reveals different current
behavior, make it literal (forgiving where no ambiguity exists: a lone `}`
cannot start a field; strict where ambiguity exists: an unclosed `{` is a
user error).

## D6: Suggestion engine design

Contract recap (spec 5.3, unchanged): for `AmbiguousRule` and
`OverlappingRules`, emit only refinements that resolve every instance
batch-wide and introduce no new diagnostic anywhere; refinements are
structured edits, not prose.

### Data flow

```
(Profile, BatchIdentification, Report) -> Vec<Suggestion>
```

Pure post-pass over the planner's report, using the same cached
identification data. No I/O.

```
Suggestion := {
  diagnostic_ref,          # which conflict instance group this resolves
  config_path,             # the rule being edited
  edit: StructuredEdit,    # closed enum, see below
  yaml_fragment: String,   # rendered once, deterministically, by core
}

StructuredEdit :=            # closed grammar, v1
  | AddExactCondition     { property, value }          # into the rule's exact map
  | AddNotExactCondition  { property, value }          # into not: [ { exact: ... } ]
  | AddSubstringCondition { property: track_name, value }
  | AddNotSubstring       { property: track_name, value }
```

The edit grammar is deliberately closed: GUI one-click apply and YAML
fragment rendering stay trivial, and simulation covers the entire candidate
space by construction. Suggestions only ever narrow the conflicted rule's
match expression; they never reorder rules, never touch other rules, never
relax anything (v1 scope).

### Algorithm

1. **Group.** Collect conflict instances by rule `config_path` across the
   whole batch: for `AmbiguousRule` on rule R, the per-file sets of matched
   tracks; for `OverlappingRules` on rules (R1, R2), the per-file contested
   tracks. An `OverlappingRules` group generates candidates for each of the
   two rules independently (narrowing either can resolve it).
2. **Generate.** For each group, diff the matchable property vectors of the
   conflicting tracks (from cached identification). A property p with value v
   is a discriminator candidate if in EVERY affected file it splits the
   conflict set (at least one track has it, at least one does not). Each
   discriminator yields candidates in both polarities (AddExact / AddNotExact;
   for `track_name`, additionally substring candidates from name tokens),
   because the engine cannot know which track the user intends: for a
   two-track ambiguity, candidates selecting each side are both emitted.
   The candidate set is bounded by (properties present on conflicting tracks)
   x (their observed values) x 2 polarities.
3. **Simulate.** For each candidate: apply the edit to a cloned profile and
   re-run the REAL planning pass (same code path as dry-run, no parallel
   implementation) against the cached identification of the entire batch.
4. **Accept.** Keep a candidate iff (a) every instance of the group's conflict
   is gone, and (b) the diagnostic set introduces nothing new anywhere:
   compare (code, config_path, file) multisets before/after; pre-existing
   unrelated diagnostics remain allowed, any addition disqualifies.
   This IS the "an applied suggestion survives the next dry run" invariant.
5. **Rank and cap.** Deterministic preference order among accepted candidates:
   typed flags (forced_track, flag_*) > language > codec-derived > track_name
   substring; positive exact before not-conditions at equal property rank;
   ties broken by property name, then value (stable order, reproducible
   output). Emit at most 3 per conflict group, log the cap in the report if
   more were accepted (no silent truncation).
6. **No-single-fix report.** If no candidate is accepted: partition the
   affected files by their conflict signature (the property-vector multiset of
   the conflicting tracks) and report the partition explicitly ("these 3 files
   need one resolution, these 2 another"), per spec 5.3's explicit-failure
   requirement. The partition reuses the discriminator diff from step 2.

### Properties worth stating

- **Determinism:** same profile + same identification cache -> byte-identical
  suggestion list. No randomness, stable sorts only.
- **Complexity:** O(candidates x batch-replan). Replanning is pure in-memory
  matcher evaluation over cached identification; for realistic batches
  (thousands of files, tens of candidates) this is milliseconds, no
  optimization work in v1. If it ever hurts, restrict re-simulation to
  affected rules; do not build that now.
- **Convergence:** applying an accepted suggestion strictly narrows one rule
  and (by acceptance criterion b) creates no new conflicts, so iterated
  apply-and-rerun terminates; no oscillation is possible in v1's
  narrow-only edit grammar.

### Testing (feeds Plan 2 tasks)

- Property test for the acceptance invariant: for arbitrary generated
  batches/profiles with injected ambiguities, every emitted suggestion, when
  applied, yields a re-plan with the conflict gone and no new diagnostics
  ("survives the next dry run" as an executable property).
- Unit: discriminator generation (splits in every file, both polarities),
  ranking order, cap logging, no-single-fix partition.
- Golden: YAML fragment rendering for each StructuredEdit variant.
