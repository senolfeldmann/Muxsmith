# Plan 6 scoped design amendment: the `apply_suggestion` seam (2026-07-16)

Amends D43 (`2026-07-15-plan-6-design.md:410-495`). D43 binds `apply_suggestion`
to reuse the engine's narrowing helper rather than reimplement it. An
independent review found the seam D43 assumes does not exist: the helper takes a
`&MatchExpr`, the bridge to `StructuredEdit` is private, and that bridge reads a
typed `Scalar` the applier has no way to obtain. This document establishes the
ground truth by running the tree, corrects the review where it is wrong, and
settles the seam as **D49**.

D48 (`2026-07-15-plan-6-design.md:1336`) is the highest ADR in
`docs/superpowers/specs/` (`grep -rn "^## D" docs/superpowers/specs/`, sorted on
the number). This is D49.

---

## 1. Verified ground truth

Every claim below was produced by running the tree at commit `41c4cb7`, not by
reading the review. Commands are named so each line can be re-run.

### 1.1 `StructuredEdit` as it exists today

`crates/muxsmith-core/src/planner.rs:199-228`, quoted verbatim from the file:

```rust
/// The closed grammar of suggestion edits (spec 5.3, D6); only ever narrows a
/// rule. Populated by the engine (see `suggest`).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StructuredEdit {
    /// Add `property: value` to the rule's `exact` map.
    AddExact {
        /// The matchable property to constrain.
        property: String,
        /// The value to require.
        value: String,
    },
    /// Add `{ exact: { property: value } }` to the rule's `not` list.
    AddNotExact {
        /// The matchable property to exclude on.
        property: String,
        /// The value to exclude.
        value: String,
    },
    /// Add `track_name: value` to the rule's `substring` map.
    AddSubstring {
        /// The `track_name` substring to require.
        value: String,
    },
    /// Add `{ substring: { track_name: value } }` to the rule's `not` list.
    AddNotSubstring {
        /// The `track_name` substring to exclude.
        value: String,
    },
}
```

Field types, complete: `AddExact { property: String, value: String }`,
`AddNotExact { property: String, value: String }`, `AddSubstring { value: String }`,
`AddNotSubstring { value: String }`. Derives: `Debug, Clone, PartialEq, Serialize`.
**No `Deserialize` today**; D43 rules it gains one. The derive sits on
`planner.rs:201`, which is the line D43 cites - D43's citation is correct.

### 1.2 `Scalar` already derives `Deserialize`

`crates/muxsmith-core/src/profile/match_expr.rs:17-28`:

```rust
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Scalar {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}
```

`Deserialize` is present. `#[serde(untagged)]`, and the declaration order is
documented as load-bearing for numbers.

### 1.3 The engine holds the typed `Scalar` when it builds the edit

`planner.rs:1738-1766`, verbatim:

```rust
                let Some((display, scalar)) = prop_value_as(val) else {
                    continue;
                };
                for (polarity, edit) in [
                    (
                        0u8,
                        StructuredEdit::AddExact {
                            property: prop.clone(),
                            value: display.clone(),
                        },
                    ),
                    (
                        1u8,
                        StructuredEdit::AddNotExact {
                            property: prop.clone(),
                            value: display.clone(),
                        },
                    ),
                ] {
                    if !mode.keeps_polarity(polarity) {
                        continue;
                    }
                    if seen.insert((prop.clone(), display.clone(), polarity)) {
                        raw.push(Candidate {
                            apply: delta_for(&edit, &scalar),
                            rank: (rank_of(prop, polarity), prop.clone(), display.clone()),
                            edit,
                        });
                    }
                }
```

Yes: `scalar` is in scope and live at the exact moment `StructuredEdit` is
constructed. The engine chooses to put `display` in the edit and pass `scalar`
past it. The typed value is not unavailable; it is discarded.

`prop_value_as` (`planner.rs:2063-2070`):

```rust
fn prop_value_as(v: &PropValue) -> Option<(String, Scalar)> {
    match v {
        PropValue::Bool(b) => Some((b.to_string(), Scalar::Bool(*b))),
        PropValue::Int(i) => Some((i.to_string(), Scalar::Int(*i))),
        PropValue::Str(s) => Some((s.clone(), Scalar::Str(s.clone()))),
        PropValue::Float(_) => None,
    }
}
```

`delta_for` (`planner.rs:1809-1838`) destructures `{ property, .. }` on the
`AddExact`/`AddNotExact` arms and builds the delta from its `scalar` argument;
the `AddSubstring`/`AddNotSubstring` arms use `edit.value` directly.

### 1.4 The reachable value set is `{Bool, Int, Str}`; `Float` is unreachable

`prop_value_as` is a total match over `PropValue`'s four variants and maps
`Float` to `None`. A `PropValue::Float` therefore never becomes a candidate at
all. **The set of `Scalar` variants that can appear in an engine-produced
`AddExact`/`AddNotExact` is exactly three: `Scalar::Bool`, `Scalar::Int`,
`Scalar::Str`.** This is the enumeration `proc-latitude-clause-boundary` demands,
and it is closed by a total match, not by convention.

### 1.5 What `matchable_type` resolves, and the set it covers

`capability::matchable_type` is at `capability/mod.rs:40` (the review's citation
is correct). It returns `Option<PropType>`, where `PropType` has four variants
(`capability/mod.rs:23-32`): `String`, `Boolean`, `Integer`, `Float`.

Its domain is `generated::MATCHABLE_PROPERTIES` plus the virtual `codec_kind`.
Counted, not estimated:

- `grep -c '^    ("' crates/muxsmith-core/src/capability/generated.rs` -> **62** entries.
- `grep -o 'PropType::[A-Za-z]*' crates/muxsmith-core/src/capability/generated.rs | sort | uniq -c` -> **9 Boolean, 5 Float, 27 Integer, 21 String** (9+5+27+21 = 62).
- Plus `codec_kind` (hardcoded `String` in `matchable_type`) = **63 names** resolve.

The 9 Boolean names: `default_track`, `enabled_track`, `flag_commentary`,
`flag_hearing_impaired`, `flag_original`, `flag_text_descriptions`,
`flag_visual_impaired`, `forced_track`, `text_subtitles`.

The 5 Float names: `max_luminance`, `min_luminance`, `projection_pose_pitch`,
`projection_pose_roll`, `projection_pose_yaw`.

The three pseudo-props the engine pushes (`planner.rs:1731-1733`) are all in the
generated table: `codec` (String, `:18`), `id` (Integer, `:42`), `type`
(String, `:67`). `codec_kind` resolves in `matchable_type` but is never a
candidate property: it is a curated alias resolved at match time from `codec_id`
(`matcher.rs:120-124`) and is never inserted into a `Track`'s `properties`
(no `codec_kind` write exists outside `capability/mod.rs` and the matcher).

### 1.6 `matchable_type` is not a function onto `Scalar` (the decisive fact)

`profile/validate.rs:416-425`, verbatim:

```rust
fn scalar_fits(value: &Scalar, t: PropType) -> bool {
    matches!(
        (value, t),
        (Scalar::Str(_), PropType::String)
            | (Scalar::Bool(_), PropType::Boolean)
            | (Scalar::Int(_), PropType::Integer)
            | (Scalar::Int(_), PropType::Float)
            | (Scalar::Float(_), PropType::Float)
    )
}
```

`PropType::Float` admits **both** `Scalar::Int` and `Scalar::Float`. The house's
own validator says so. A `PropType` therefore does not determine a `Scalar`
variant, and `matchable_type` cannot drive a parse that must produce one.

Worse, in the only case that is reachable, `matchable_type` gives the **wrong**
answer. `prop_value_as` dispatches on the runtime `PropValue`, not the declared
type, so a Float-declared property reaches an `AddExact` only when mkvmerge emits
an integral literal, and then the scalar is `Int`. Measured against the real
crate:

```
from_json(400)      = Some(Int(400))
from_json(400.0)    = Some(Float(400.0))     // -> prop_value_as -> None, dropped
matchable_type(min_luminance ) = Some(Float)
Int(400) == Float(400.0)? false
```

So for `min_luminance`, the engine simulates `Scalar::Int(400)` while
`matchable_type` says `Float`. Reconstruction would yield `Scalar::Float(400.0)`,
which is `!=` the simulated delta and renders as `400.0` rather than `400`
(measured: `yaml_serde::to_string(&Scalar::Float(400.0))` = `"400.0\n"`,
`Scalar::Int(400)` = `"400\n"`).

### 1.7 What consumes `StructuredEdit.value` today

Complete list, from `grep -rn "StructuredEdit" --include="*.rs" crates/` plus the
TS tree:

| consumer | reads `value` on `AddExact`/`AddNotExact`? |
|---|---|
| `delta_for` (`planner.rs:1812`, `:1817`) | **No** - destructures `{ property, .. }`, discards it |
| `delta_for` (`planner.rs:1824`, `:1829`, substring arms) | Yes, as `String` (unaffected) |
| engine `seen` dedup (`planner.rs:1761`) | No - uses the local `display`, not the edit |
| engine `rank` (`planner.rs:1763`) | No - uses the local `display`, not the edit |
| `yaml_fragment` (`planner.rs:2088`) | No - takes `&MatchExpr` (`cand.apply`), never the edit |
| CLI human output (`muxsmith-cli/src/commands/mod.rs:126`) | No - prints `s.yaml_fragment` only |
| `dry-run --json` report, built in **core** (`crates/muxsmith-core/src/report/json.rs:56`), consumed by the CLI | Serializes the whole `Suggestion`, so `edit.value` is on that wire |
| Frontend (`src/ipc.ts:132`) | No - typed `edit: unknown`; D22 confines the GUI to `yaml_fragment` |
| `tests/suggestions.rs` | Yes, 7 sites (below) |

**No production code reads `value` on `AddExact`/`AddNotExact`.** Today it is a
pure wire/display artifact with no reader.

Test sites binding `value` on those two variants, counted from
`grep -n "StructuredEdit::AddExact\|StructuredEdit::AddNotExact" tests/suggestions.rs`
(9 matches, of which `:509` and `:514` bind `..` and are unaffected): **7 sites** -
`:97`, `:100`, `:203`, `:206`, `:325`, `:722`, `:890`.

No test asserts the JSON shape of `edit`. Established by two greps, both in a
form that could have failed (a `'"edit"'` search cannot match an unquoted
TypeScript key and would have missed the real hits):
`grep -rn "edit" --include="*.ts" --include="*.vue" --include="*.json" .` outside
`node_modules`, filtered to `edit`-as-a-key, returns **2** hits - `src/ipc.ts:132`
(`edit: unknown;`) and `e2e/smoke.spec.ts:177` (`edit: null,`), both inert against
a `value` type change; and `grep -rn '\["edit"\]\|"edit"' --include="*.rs" crates/ src-tauri/`
returns **0**. See "Interface changes" for why each TS hit is inert.

### 1.8 The existing `core-03` test cannot catch a type error

`tests/suggestions.rs:68-91` (`every_suggestion_survives_the_next_dry_run`)
applies through `apply_edit_to_first_rule` (`:95-114`), which is a **YAML string
template**: it formats `{property}: {value}` into profile text and re-parses it.
YAML's own scalar inference then re-types the value. Measured against the real
loader:

```
exact: { type: subtitles, default_track: true }    -> {"default_track": Bool(true),  "type": Str("subtitles")}
exact: { type: subtitles, id: 3 }                  -> {"id": Int(3),                 "type": Str("subtitles")}
exact: { type: subtitles, min_luminance: 400 }     -> {"min_luminance": Int(400),    "type": Str("subtitles")}
```

The string path launders the type. `apply_suggestion` mutates the model in
memory and has no YAML round-trip, so it has no laundering step. **The existing
core-03 test provides zero coverage for the model-level applier.** This is
precisely why the wrong answer "passes string-only tests".

### 1.9 What the engine actually emits (real run, both fixtures)

Running `plan_batch` against the committed fixtures through the real crate:

```
##### P_AMBIGUOUS + fixtures/identify/series-s01e01.json  (3 suggestions)
  edit = AddExact { property: "flag_hearing_impaired", value: "true" }
  json = {"kind":"add_exact","property":"flag_hearing_impaired","value":"true"}
  edit = AddExact { property: "forced_track", value: "false" }
  json = {"kind":"add_exact","property":"forced_track","value":"false"}
  edit = AddExact { property: "forced_track", value: "true" }
  json = {"kind":"add_exact","property":"forced_track","value":"true"}

##### P_SUBS_BY_LANGUAGE + CODEC_ID_ONLY  (3 suggestions)
  edit = AddExact { property: "codec", value: "SubRip/SRT" }
  edit = AddExact { property: "codec", value: "SubStationAlpha/ASS" }
  edit = AddExact { property: "id", value: "1" }
```

The current wire for a Boolean property is literally `"value":"true"`. Both
fixtures the guard needs already exist in `tests/suggestions.rs`
(`P_AMBIGUOUS:17`, `CODEC_ID_ONLY:464`, `P_SUBS_BY_LANGUAGE:481`).

### 1.10 Corrections to the review's claims

The review is right on every citation it makes. Three of its framings are wrong
or moot, and one omission matters.

1. **"Would need `delta_for` reachable from the apply path" - moot for both
   options.** `crates/muxsmith-core/src/planner.rs` is a single **file** module
   (2204 lines; there is no `planner/` directory). D43 places `apply_suggestion`
   at `planner::apply_suggestion`, i.e. **in that same module**. A private
   `delta_for` is already reachable from it. No visibility change is needed by
   any option on the table. The review presented this as a cost of option (i);
   it is a cost of nothing.

2. **`matchable_type` cannot drive the parse.** The review offers it as "the
   house's authority on a matchable property's type" that "could drive the
   parse". It is the authority on the *declared* type, but the declared type does
   not determine the `Scalar` variant (§1.6): `PropType::Float` admits both
   `Scalar::Int` and `Scalar::Float` by the house's own `scalar_fits`. Option
   (i)'s stated mechanism does not exist as a function.

3. **`with_rule_match` is already `#[doc(hidden)] pub`** (`planner.rs:1852-1853`),
   as is `rule_index_of` (`planner.rs:2031-2032`). The review does not say this.
   It means `core-90`'s "does it extend to production reuse?" question **does not
   arise**: `apply_suggestion` reaches both from inside the same module by
   ordinary private visibility, and needs neither the `pub` nor the `#[doc(hidden)]`.
   The existing annotations stay exactly as they are, for the reason their own
   comments give (cross-crate access from `tests/prop_planner.rs`).

4. **The review implies `Scalar::Str(value)` is merely a formatting slip.** It is
   worse than that: `matcher.rs:202-212`'s `scalar_eq` has no `(Str, Bool)` or
   `(Str, Int)` arm and falls through to `_ => false`, so a stringified value
   makes the narrowed rule match **nothing**, surfacing as `MissingTrack` rather
   than as a wrong-looking value. Verified by reading the function; there are
   exactly six arms plus the fallthrough.

Everything else the review asserts - `with_rule_match`'s signature,
`delta_for`'s privacy and shape, the discarded `value`, `prop_value_as`'s pair,
the engine's split at `:1738-1762`, D43 forbidding a re-plan, and
`core-72`'s typed-equality semantics - is confirmed verbatim.

---

## D49: `StructuredEdit` carries the typed `Scalar`; `delta_for` loses its scalar argument

**Decision.** Option (ii). The edit carries exactly what the engine simulated, so
nothing is reconstructed and nothing can diverge. `apply_suggestion` splices
through the engine's own `delta_for` + `with_rule_match`, unchanged in behaviour.

### The wire shape

`planner.rs:199-227` becomes exactly this:

```rust
/// The closed grammar of suggestion edits (spec 5.3, D6); only ever narrows a
/// rule. Populated by the engine (see `suggest`) and accepted back from the
/// shell to apply (D43, D49).
///
/// `AddExact`/`AddNotExact` carry a typed [`Scalar`], not a display string:
/// `exact` compares each property in its own domain (`core-72`), so the
/// variant is a core semantic. The engine holds the typed value when it builds
/// the edit; carrying it is what makes the applied delta identical to the
/// simulated one (`core-03`, D49).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StructuredEdit {
    /// Add `property: value` to the rule's `exact` map.
    AddExact {
        /// The matchable property to constrain.
        property: String,
        /// The value to require, in the type the engine identified.
        value: Scalar,
    },
    /// Add `{ exact: { property: value } }` to the rule's `not` list.
    AddNotExact {
        /// The matchable property to exclude on.
        property: String,
        /// The value to exclude, in the type the engine identified.
        value: Scalar,
    },
    /// Add `track_name: value` to the rule's `substring` map.
    AddSubstring {
        /// The `track_name` substring to require.
        value: String,
    },
    /// Add `{ substring: { track_name: value } }` to the rule's `not` list.
    AddNotSubstring {
        /// The `track_name` substring to exclude.
        value: String,
    },
}
```

`AddSubstring`/`AddNotSubstring` keep `value: String`. This asymmetry is not an
oversight and is not the implementer's to reconsider: `MatchExpr.substring` is
`Option<BTreeMap<String, String>>` (`match_expr.rs:61`) while `MatchExpr.exact` is
`Option<BTreeMap<String, Scalar>>` (`match_expr.rs:56`). Each variant carries the
type its target map holds.

Two import changes in `planner.rs`, both mandatory:

- `:10` becomes `use serde::{Deserialize, Serialize};`.
- A new `#[cfg(feature = "ts")] use ts_rs::TS;`. The `cfg` is **not** optional
  and there is no precedent to copy: `ts_rs` appears nowhere in the tree today
  (`grep -rn 'ts_rs\|ts-rs' --include=*.toml --include=*.rs` returns nothing
  outside `target/`), and `crates/muxsmith-core/Cargo.toml` has no `[features]`
  section at all, both because D44 is unlanded. A bare `use ts_rs::TS;` would be
  an unused import on a default build and fail
  `cargo clippy --workspace --all-targets -- -D warnings` (`.github/workflows/ci.yml:86`).

`Scalar` is already imported at `planner.rs:17`.

**D49 cannot land before D44.** The `#[cfg_attr(feature = "ts", derive(TS), ...)]`
above needs the `ts` feature, the optional `ts-rs` dependency, and the
`[features]` section to exist, and `StructuredEdit`'s derive needs `Scalar: TS` -
which D44 provides. Landing D49 first would either not compile under
`--features ts` or silently ship a `StructuredEdit` the binding does not export.

### `delta_for`

Loses its second parameter and stays private (`planner.rs:1809`):

```rust
// Builds the MatchExpr delta a candidate edit represents. Total: the edit
// carries its own typed value (D49), so there is nothing to reconstruct.
fn delta_for(edit: &StructuredEdit) -> MatchExpr {
    let mut m = MatchExpr::default();
    match edit {
        StructuredEdit::AddExact { property, value } => {
            let mut map = BTreeMap::new();
            map.insert(property.clone(), value.clone());
            m.exact = Some(map);
        }
        StructuredEdit::AddNotExact { property, value } => {
            let mut inner = MatchExpr::default();
            let mut map = BTreeMap::new();
            map.insert(property.clone(), value.clone());
            inner.exact = Some(map);
            m.not = Some(vec![inner]);
        }
        StructuredEdit::AddSubstring { value } => {
            let mut map = BTreeMap::new();
            map.insert("track_name".to_string(), value.clone());
            m.substring = Some(map);
        }
        StructuredEdit::AddNotSubstring { value } => {
            let mut inner = MatchExpr::default();
            let mut map = BTreeMap::new();
            map.insert("track_name".to_string(), value.clone());
            inner.substring = Some(map);
            m.not = Some(vec![inner]);
        }
    }
    m
}
```

**Visibility: unchanged (private).** `apply_suggestion` is in the same module.

### The engine call sites

Exactly four edits, all in `planner.rs`:

1. `:1746` - `value: display.clone()` becomes `value: scalar.clone()` (the `AddExact` arm).
2. `:1753` - `value: display.clone()` becomes `value: scalar.clone()` (the `AddNotExact` arm).
3. `:1762` - `apply: delta_for(&edit, &scalar)` becomes `apply: delta_for(&edit)`.
4. `:1791` - `apply: delta_for(&edit, &Scalar::Str(tok.to_string()))` becomes `apply: delta_for(&edit)`; the synthetic `Scalar::Str` disappears.

`prop_value_as` keeps its `(String, Scalar)` return shape unchanged. Both halves
stay live: `scalar` now goes into the edit, `display` continues to key the `seen`
dedup set (`:1761`) and the `rank` tuple (`:1763`). Neither is redundant.

### `apply_suggestion`

New, `pub`, in `planner.rs`:

```rust
/// Applies a structured suggestion edit to the rule named by `config_path`,
/// returning a new profile. Narrow-only (`core-33`): splices through the
/// engine's own `delta_for` + `with_rule_match`, so the applied delta is
/// identical to the one the engine simulated (`core-03`) and the no-clobber
/// semantics of `core-44` apply unchanged. The caller validates the result
/// through the normal `validate_profile_model` path (D43); this function does
/// not validate and does not re-plan.
pub fn apply_suggestion(
    profile: &Profile,
    config_path: &str,
    edit: &StructuredEdit,
) -> Result<Profile, ApplyError> {
    let index = rule_index_of(config_path)
        .ok_or_else(|| ApplyError::UnparsableConfigPath(config_path.to_string()))?;
    let rules = profile.tracks.rules.len();
    if index >= rules {
        return Err(ApplyError::RuleIndexOutOfRange { index, rules });
    }
    let applied = with_rule_match(profile, index, &delta_for(edit));
    // core-44's or_insert merge silently drops a delta whose key the rule
    // already constrains. Noticing that nothing happened is not a re-plan and
    // not a validation (D43): it is one comparison of the model against itself.
    if applied == *profile {
        return Err(ApplyError::EditChangedNothing {
            index,
            property: edit_key(edit).to_string(),
        });
    }
    Ok(applied)
}
```

`Profile` derives `PartialEq` (`profile/model.rs:17`), so the comparison needs no
new impl.

`edit_key` is a new private helper in `planner.rs`, total over the four variants:

```rust
// The match key an edit targets: the named property for the two exact variants,
// the fixed `track_name` key for the two substring ones (delta_for,
// planner.rs:1824, :1829).
fn edit_key(edit: &StructuredEdit) -> &str {
    match edit {
        StructuredEdit::AddExact { property, .. }
        | StructuredEdit::AddNotExact { property, .. } => property,
        StructuredEdit::AddSubstring { .. } | StructuredEdit::AddNotSubstring { .. } => {
            "track_name"
        }
    }
}
```

### `ApplyError`

New, `pub`, in `planner.rs`, per `core-124` (an operational failure of a core
routine, never a `Diagnostic`: the profile's content is not wrong, the caller's
argument is - a frontend bug). Shaped after the `SettingsError` precedent
(`src-tauri/src/settings.rs:100-108`): a plain enum carrying structured params,
no prose, no `Deserialize`.

```rust
/// An operational failure of [`apply_suggestion`] (`core-124`): the edit did not
/// reach the profile. Never a `Diagnostic` - the profile's content is not the
/// problem. The shell maps each variant to an `IpcError`.
#[derive(Debug, Clone, PartialEq)]
pub enum ApplyError {
    /// `config_path` is not of the `tracks[<N>].match` form `rule_index_of`
    /// parses; carries the offending path verbatim.
    UnparsableConfigPath(String),
    /// The parsed index is past the end of `tracks.rules`.
    RuleIndexOutOfRange {
        /// The index parsed out of `config_path`.
        index: usize,
        /// The number of rules the profile actually has.
        rules: usize,
    },
    /// The edit changed nothing: `with_rule_match`'s `or_insert` merge
    /// (`core-44`, never clobber) dropped the delta because rule `index`
    /// already constrains `property`. The engine's acceptance simulation
    /// rejects such a candidate before it is ever shown; apply has no
    /// simulation, so a suggestion computed against a since-edited model
    /// arrives here instead, and `Ok(unchanged)` would report success for a
    /// no-op.
    EditChangedNothing {
        /// The rule the edit targeted.
        index: usize,
        /// The match key that was already constrained.
        property: String,
    },
}
```

**Exactly three variants, because `apply_suggestion` has exactly three exits.**
An earlier draft claimed two and was wrong. The third is not a parse failure but
a silent success: `with_rule_match` merges `exact`/`substring` with
`or_insert_with` (`planner.rs:1856-1861`), so when the rule already constrains the
key the delta is dropped and the returned profile equals the input. Measured
against the real helper, on a rule that already carries `forced_track: true`
given a delta of `forced_track: false`:

```
no-op case: applied == profile ? true
  rule match after: Some({"forced_track": Bool(true), "type": Str("subtitles")})
control (unconstrained property): applied == profile ? false
not-append (AddNotExact):          applied == profile ? false
```

The engine never ships such a candidate - `resolves_without_regression` rejects
it for not resolving the ambiguity (`planner.rs:1845-1848`'s own comment says so).
Apply has no acceptance simulation, and Plan 6 is the **editor** plan, so a
suggestion computed against a model the user has since edited is the normal path,
not an edge case.

The two `not`-appending variants can never reach this exit: `with_rule_match`
merges `not` with `extend` (always additive), and `delta_for` always emits a
non-empty `not` vector for them - the third measurement above confirms it. The
detection is a whole-model comparison rather than a per-variant rule, so it stays
correct without enumerating which variants can trigger it.

`with_rule_match` itself cannot fail (it indexes a checked index and merges maps),
and `delta_for` is total.

### The shell mapping

New, in `src-tauri/src/error.rs`, matching the `From<SettingsError>` precedent
(`error.rs:123-132`) exactly:

```rust
impl From<ApplyError> for IpcError {
    fn from(e: ApplyError) -> IpcError {
        match e {
            ApplyError::UnparsableConfigPath(path) => {
                IpcError::new("apply-unparsable-config-path").with("path", path)
            }
            ApplyError::RuleIndexOutOfRange { index, rules } => {
                IpcError::new("apply-rule-index-out-of-range")
                    .with("index", index.to_string())
                    .with("rules", rules.to_string())
            }
            ApplyError::EditChangedNothing { index, property } => {
                IpcError::new("apply-edit-changed-nothing")
                    .with("index", index.to_string())
                    .with("property", property)
            }
        }
    }
}
```

`IpcError::with` takes `impl Into<String>` for both key and value
(`error.rs:60`), hence the `to_string()` on the two numbers.

### The catalog entries

`core-124` records that every new code forces new bilingual prose. Three codes x
two locales = six lines, written out here so none is invented:

`locales/en/gui-common.ftl`:

```
apply-unparsable-config-path = The suggestion could not be applied: "{ $path }" does not name a rule.
apply-rule-index-out-of-range = The suggestion could not be applied: no rule at index { $index } (rule count: { $rules }).
apply-edit-changed-nothing = The suggestion changed nothing: rule { $index } already constrains "{ $property }".
```

`locales/de/gui-common.ftl`:

```
apply-unparsable-config-path = Der Vorschlag konnte nicht angewendet werden: "{ $path }" benennt keine Regel.
apply-rule-index-out-of-range = Der Vorschlag konnte nicht angewendet werden: keine Regel an Index { $index } (Regelanzahl: { $rules }).
apply-edit-changed-nothing = Der Vorschlag hat nichts geändert: Regel { $index } schränkt "{ $property }" bereits ein.
```

**`apply-rule-index-out-of-range` states the count as a labelled value, never as a
counted noun.** An earlier draft read "the profile has { $rules } rules" /
"das Profil hat { $rules } Regeln", which renders "1 rules" / "1 Regeln" in
exactly the case `apply_rejects_a_rule_index_past_the_end` constructs
(it loads `P_AMBIGUOUS`, annotated `// exactly 1 rule`, and asserts `rules: 1`).
A Fluent plural
selector cannot fix that here and must not be added: `IpcError.params` reaches
Fluent as `Record<string, string>` at every call site
(`RunHistory.vue:155`, `:241`, `JobsView.vue:246`, `:252`, `FirstRun.vue:94`),
which pass `error.params` straight into `$t`. The one number-promoting path,
`diagnosticFluentParams` (`DiagnosticsPanel.vue:34`), is keyed by **diagnostic**
code (`NUMERIC_DIAGNOSTIC_PARAMS`, `src/diagnosticFluentParams.ts:14`), so an
`IpcError` code never reaches it and `[one]` would always fall through to
`*[other]`. `(rule count: { $rules })` sidesteps agreement in both locales and
needs no mechanism. The structural gap behind this (nothing gates `IpcError`
codes against `gui-common.ftl`, and `IpcError` params are not number-promoted)
is a ROADMAP item with its own trigger and is out of D49's scope.

### Why this and not reconstruction

The house's recorded preference where the same knowledge must exist twice is to
**derive the second copy from the first so the two can never drift**
(`capability/mod.rs:125-129`, applied in D45 and D48). Option (i) invokes that
preference and inverts it. The display `String` is a **lossy projection** of the
`Scalar` (`prop_value_as` measured: `Bool(true)` and `Str("true")` both project to
`"true"`; `Int(400)` and `Float(400.0)` both project to values the declared type
cannot disambiguate, per §1.6). You cannot derive the original from a lossy
projection; you can only guess, and §1.6 shows the guess is wrong for the
Float-declared properties in the only case that is reachable.

Read correctly, the preference **selects option (ii)**: keep the rich value (the
`Scalar`) as the single copy, and derive the lossy one (the display string) from
it where a display is wanted. The tree already has that derivation -
`scalar_display` (`planner.rs:846-853`) - and D49 leaves it in place. After D49
there is **one** copy of the value on the wire, so the drift question does not
arise and no guard is needed for it.

---

## Rejected alternatives

### Rejected: (i) reconstruct the typed `Scalar` from the display String at apply time

**Steelman, at its strongest.** The wire is a contract, and D43 already settled
its shape; a scoped amendment about a *core-internal* seam should not reach out
and change a published type. Reconstruction keeps `StructuredEdit` exactly as
D43 froze it: the ts-rs binding still emits `value: string`, the CLI's
`--json` report keeps byte-identical output, and `Suggestion`'s serialization is
untouched, so nothing downstream re-tests. It also keeps the *typing decision*
where the type knowledge lives: core alone decides what `default_track: "true"`
means, and the frontend cannot express a type at all, so it cannot express a
wrong one. That is a real integrity property, and it is the same instinct that
made D43 refuse `Deserialize` on `DiagCode`. And it invokes the house's own
recorded preference: rather than shipping a second copy of the value's type on
the wire (where a frontend echo could corrupt it), derive it from the one
authority that already exists, `capability::matchable_type` - the file the
house generates from the identification schema precisely so this knowledge has a
single source. Two copies can drift; a derivation cannot.

**Why it loses.** The derivation it proposes is not available.
`matchable_type` returns `PropType`, and `PropType` does not determine a
`Scalar` variant: `scalar_fits` (`validate.rs:416-425`) admits both
`(Scalar::Int, PropType::Float)` and `(Scalar::Float, PropType::Float)`. For
the 5 Float-declared properties the reconstruction has no correct answer to
give, and the answer it would give is provably the wrong one: `prop_value_as`
dispatches on the runtime `PropValue` and drops `PropValue::Float`, so a
Float-declared property reaches an `AddExact` **only** carrying `Scalar::Int`
(measured: `from_json(400) = Int(400)`, `from_json(400.0) = Float(400.0)` which
is then dropped). Reconstructing `min_luminance` as `Float(400.0)` therefore
diverges from the simulated `Int(400)` - measured `!=` - so the applied edit is
not the edit that was simulated, and the YAML it renders reads `400.0` where the
engine rendered `400`.

The scope of that divergence is stated precisely, because overstating it would
put the whole rejection one command away from being re-litigated: it is a break
in **delta identity and rendering, not in matching**. `scalar_eq` has
`(Int, Float)` and `(Float, Int)` arms (`matcher.rs:207`, `:209`), so
`Scalar::Float(400.0)` still matches an `Int(400)` track and such a suggestion
would survive the next dry run. The reconstruction's failure is that it silently
changes what the user's profile says, not that it stops matching. Symmetrically,
the one case that *does* break matching - `Scalar::Str("true")` against a Boolean
property - is a case option (i) gets **right**, since `matchable_type("forced_track")`
is `Boolean`. Option (i) loses on the Float set and on cost, not on the Bool set.

The integrity half of the steelman is answered without paying for it. The
grammar stays closed by type: `#[serde(tag = "kind")]` rejects an unknown tag at
the boundary, `Scalar` is a closed four-variant untagged enum, and a wrong-typed
value that survives the wire is caught downstream by `scalar_fits` as a
`ValueTypeMismatch` on the validation pass D43 already routes every applied
result through. The frontend does not construct edits in any case: it echoes back
the opaque `edit` it received (D43's own rationale; `src/ipc.ts:132` types it
`unknown`), and the echo is measured type-preserving (§ guard, and the 8/8
round-trip below).

Finally, the cost the steelman weighs is smaller than it appears and the benefit
is negative: reconstruction needs a new parse function, a new failure mode
(unparsable display string), and a new guard proving the parse inverts
`prop_value_as` for all three reachable variants. Option (ii) needs none of
those, because it deletes the projection instead of inverting it.

### Rejected: (iii) a narrower `EditValue { Bool, Int, Str }` enum instead of `Scalar`

**Steelman, at its strongest.** §1.4 proves the reachable set is exactly three
variants, and `Scalar` has four. Using `Scalar` therefore puts a value on the
wire (`Scalar::Float`) that the engine can never produce, which is
unrepresentable-state territory: a type that admits states its producer cannot
reach invites a consumer to handle them, or worse, to construct them. A purpose-
built `EditValue` makes the illegal state unrepresentable, documents the closed
set in the type system rather than in a comment, and would make the guard for the
Float case unnecessary by construction. The house is explicit that a closed
grammar is a real integrity property (`core-33`, and D43's defence of the four-
variant enum).

**Why it loses.** It creates precisely the second copy the house's derivation
preference exists to prevent. `MatchExpr.exact` holds `Scalar`
(`match_expr.rs:54`), so an `EditValue` must be converted to a `Scalar` inside
`delta_for` - a hand-written mapping that is a second, drifting statement of what
a scalar is, and the conversion function is exactly the reconstruction site
option (i) lost on, merely with a narrower input. It would also need its own
`#[serde(untagged)]` ordering (the subtle, load-bearing part of `Scalar`, per
`match_expr.rs:9-16`), duplicating the one piece of this design that is easy to
get wrong. The gain is against a threat that does not exist: a `Scalar::Float`
arriving from the shell is not a correctness hole - if the property is
Integer-typed, `scalar_fits` rejects it as `ValueTypeMismatch`; if it is
Float-typed, `Scalar::Float` is a legitimate narrowing the user could have typed
into the YAML by hand. Unrepresentability buys nothing here and costs a duplicate
type.

### Rejected: widen `with_rule_match` to take a `&StructuredEdit`

**Steelman, at its strongest.** D43's binding instruction is "the engine's own
narrowing helper must be reused". `with_rule_match` *is* that helper by name, and
it takes a `&MatchExpr`, so apply must build a `MatchExpr` first - which is the
whole seam problem. If `with_rule_match` took the edit, there would be one entry
point, one call for both engine and apply, no private bridge to route around, and
`core-44`'s no-clobber semantics would sit behind a single door that nobody can
walk past. Fewer doors is fewer ways to get it wrong.

**Why it loses.** It conflates two responsibilities that the engine needs
separately. `with_rule_match` is called at four sites (`planner.rs:1400`, `:1470`,
`:1571`, `:1646`), each of which already holds a `MatchExpr` (`cand.apply`) and
none of which holds only an edit; `tests/prop_planner.rs:310` calls it with a
`doc.match_expr` that has no `StructuredEdit` behind it at all. Narrowing the
parameter to `&StructuredEdit` would break all five, and widening it to accept
either means a second entry point regardless. The seam D43 wants already exists
correctly: `delta_for` translates edit to delta, `with_rule_match` splices a
delta. `apply_suggestion` calls both, in the same module, exactly as the engine
does. That *is* the reuse D43 asked for; the composition is the door.

---

## The guard

`core-03` must be mechanically enforced, not asserted. §1.8 proves the existing
`every_suggestion_survives_the_next_dry_run` cannot do it: it applies through a
YAML string template, and YAML re-parse re-types the value, so the test passes
whatever the model-level applier does.

Seven tests, all in `crates/muxsmith-core/tests/suggestions.rs`: G1, G2 and G3
below, three `ApplyError` tests (one per variant), and one control proving the
no-op detection does not fire when the edit lands. G1, G2 and G3 run on fixtures
that already exist in that file; G4 and its control add one fixture
(`P_ALREADY_CONSTRAINED`), given in full below. §1.9 measured that the existing
fixtures really do produce the needed suggestions, so none of the guards can pass
vacuously - and each asserts its own non-vacuity explicitly.

### Harness change the guard needs

`plan_multi` (`tests/suggestions.rs:117-136`) takes profile **YAML**. G3 must
re-plan an applied **model**, with no YAML round-trip - that round-trip is the
laundering step. Split it, keeping both existing call shapes working:

```rust
// Plans an already-built model. The model-level entry point: G3 must re-plan
// what `apply_suggestion` returned WITHOUT a YAML round-trip, since YAML
// re-parse re-types scalars and would launder exactly the defect under test.
fn plan_model(profile: &Profile, files: &[(&str, &str)]) -> (Batch, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let mut by_name = HashMap::new();
    for (name, json) in files {
        std::fs::write(dir.path().join(name), b"x").unwrap();
        by_name.insert(
            (*name).to_string(),
            Identification::from_json(json).unwrap(),
        );
    }
    let run = RunInputs {
        source: dir.path().to_path_buf(),
        output: Some(dir.path().join("out")),
        on_collision: None,
    };
    let mut ident = FakeIdent { by_name };
    let batch = plan_batch(profile, &run, &mut ident, &lang());
    (batch, dir)
}

fn plan_multi(profile_yaml: &str, files: &[(&str, &str)]) -> (Batch, tempfile::TempDir) {
    plan_model(&from_str(profile_yaml, Format::Yaml).unwrap(), files)
}

fn plan(profile_yaml: &str) -> (Batch, tempfile::TempDir) {
    plan_multi(profile_yaml, &[("Show.S01E01.mkv", SERIES)])
}
```

`plan_model` is today's `plan_multi` body with the `from_str` line lifted out to
the caller and `plan_batch(&profile, ...)` becoming `plan_batch(profile, ...)`;
nothing else moves.

`plan` and `plan_multi` keep their present signatures and behaviour; `plan` is
today a hand-rolled duplicate of `plan_multi` with the single SERIES file
(compare `:24-41` against `:117-136`), so this removes a duplicate rather than
adding one. Add `use muxsmith_core::profile::model::Profile;` and
`use muxsmith_core::planner::{ApplyError, apply_suggestion};` to the file's
imports.

G1 and G2 both read back the scalar apply spliced. The two variants land in
different arms of the rule's `MatchExpr` - `AddExact` in `exact`, `AddNotExact`
in the first `not` entry's `exact` - so both guards share one accessor, defined
once in `tests/suggestions.rs`:

```rust
// The scalar `apply_suggestion` spliced for `property`, read out of whichever
// arm the edit's variant targets: AddExact -> `exact`, AddNotExact -> the first
// `not` entry's `exact` (delta_for's two exact-bearing arms, planner.rs:1812,
// :1817). Returns None if the key is absent, which is itself a guard failure.
fn spliced_scalar<'a>(
    applied: &'a Profile,
    edit: &StructuredEdit,
    property: &str,
) -> Option<&'a Scalar> {
    let m = &applied.tracks.rules[0].match_expr;
    match edit {
        StructuredEdit::AddExact { .. } => m.exact.as_ref()?.get(property),
        StructuredEdit::AddNotExact { .. } => m
            .not
            .as_ref()?
            .first()?
            .exact
            .as_ref()?
            .get(property),
        StructuredEdit::AddSubstring { .. } | StructuredEdit::AddNotSubstring { .. } => None,
    }
}
```

The two substring variants return `None` rather than being unreachable: G1 and G2
filter to `AddExact`/`AddNotExact` before calling, so a `None` from those arms
would surface as the `assert_eq!` failure it is, not as a panic.

### G1: the Bool case

```rust
// core-03, type dimension: the delta apply splices must be the delta the engine
// simulated, for a Boolean property. Guards the D49 seam: a display-string edit
// (`Scalar::Str("true")`) would make `scalar_eq` fall through to `false`
// (matcher.rs:202-212 has no (Str, Bool) arm) and the rule would match nothing.
#[test]
fn apply_splices_the_simulated_scalar_for_a_bool_property() {
    let (batch, _dir) = plan(P_AMBIGUOUS);
    let profile = from_str(P_AMBIGUOUS, Format::Yaml).unwrap();

    let mut checked = 0;
    for s in &batch.suggestions {
        let (property, value) = match &s.edit {
            StructuredEdit::AddExact { property, value }
            | StructuredEdit::AddNotExact { property, value } => (property, value),
            _ => continue,
        };
        if !matches!(property.as_str(), "forced_track" | "flag_hearing_impaired") {
            continue;
        }
        assert!(
            matches!(value, Scalar::Bool(_)),
            "engine emitted {value:?} for the Boolean property {property}; expected Scalar::Bool"
        );
        let applied = apply_suggestion(&profile, &s.config_path, &s.edit).unwrap();
        let spliced = spliced_scalar(&applied, &s.edit, property);
        assert_eq!(
            spliced,
            Some(value),
            "apply spliced {spliced:?} for {property}; the engine simulated {value:?}"
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "no Boolean-property suggestion in the fixture; this guard would pass vacuously"
    );
}
```

Measured non-vacuity: the fixture yields `flag_hearing_impaired: true`,
`forced_track: false`, `forced_track: true` (§1.9), so `checked` reaches 3.

### G2: the Int case

Same structure as G1, on the fixture that produces an Integer discriminator.
Written out rather than described, so nothing is reconstructed:

```rust
// core-03, type dimension, Integer property. `id` is PropType::Integer
// (generated.rs:42); a `Scalar::Str("1")` would fall through `scalar_eq` exactly
// as the Bool case does.
#[test]
fn apply_splices_the_simulated_scalar_for_an_int_property() {
    let (batch, _dir) = plan_multi(P_SUBS_BY_LANGUAGE, &[("Show.S01E01.mkv", CODEC_ID_ONLY)]);
    let profile = from_str(P_SUBS_BY_LANGUAGE, Format::Yaml).unwrap();

    let mut checked = 0;
    for s in &batch.suggestions {
        let (property, value) = match &s.edit {
            StructuredEdit::AddExact { property, value }
            | StructuredEdit::AddNotExact { property, value } => (property, value),
            _ => continue,
        };
        if property != "id" {
            continue;
        }
        assert!(
            matches!(value, Scalar::Int(_)),
            "engine emitted {value:?} for the Integer property id; expected Scalar::Int"
        );
        let applied = apply_suggestion(&profile, &s.config_path, &s.edit).unwrap();
        let spliced = spliced_scalar(&applied, &s.edit, property);
        assert_eq!(
            spliced,
            Some(value),
            "apply spliced {spliced:?} for id; the engine simulated {value:?}"
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "no id suggestion in the fixture; this guard would pass vacuously"
    );
}
```

Measured non-vacuity: the fixture yields `AddExact { property: "id", value: "1" }`
(§1.9), and the sibling `ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions`
(`tests/suggestions.rs:491`) already asserts on `has_id` (`:511`, `:515-523`) that an
`id` suggestion is present on this fixture, so `checked` reaches 1.

### G3: the behavioural core-03 loop, at the model level

```rust
// core-03 proper ("an applied suggestion survives the next dry run"), applied
// through the real `apply_suggestion` and re-planned as a MODEL. The sibling
// `every_suggestion_survives_the_next_dry_run` re-plans through YAML text,
// whose re-parse re-types scalars and therefore cannot observe a type defect in
// the applier (D49 section 1.8). This one has no YAML in the loop.
#[test]
fn every_applied_suggestion_survives_the_next_dry_run_at_the_model_level() {
    for (profile_yaml, files, resolved) in [
        (P_AMBIGUOUS, &[("Show.S01E01.mkv", SERIES)][..], DiagCode::AmbiguousRule),
        (P_SUBS_BY_LANGUAGE, &[("Show.S01E01.mkv", CODEC_ID_ONLY)][..], DiagCode::AmbiguousRule),
    ] {
        let (batch, _dir) = plan_multi(profile_yaml, files);
        let profile = from_str(profile_yaml, Format::Yaml).unwrap();
        assert!(!batch.suggestions.is_empty(), "fixture produced no suggestions");
        for s in &batch.suggestions {
            let applied = apply_suggestion(&profile, &s.config_path, &s.edit).unwrap();
            let (re, _d) = plan_model(&applied, files);
            assert!(
                !re.files[0].diagnostics.iter().any(|d| d.code == resolved),
                "applied suggestion {:?} did not resolve {resolved:?}",
                s.edit
            );
            assert!(
                !re.files[0].diagnostics.iter().any(|d| d.code == DiagCode::MissingTrack),
                "applied suggestion {:?} over-narrowed into MissingTrack",
                s.edit
            );
        }
    }
}
```

G3 is the one that fails loudly on the wrong answer: `Scalar::Str("true")` in
`exact` makes `scalar_eq` return `false` for every track, the rule matches
nothing, and `MissingTrack` fires.

### The two argument-failure `ApplyError` tests

`ApplyError`'s three variants are each reachable and each asserted; none is
inferred. The two argument failures are below; the third variant
(`EditChangedNothing`) is asserted by G4, since its setup is a fixture rather
than a bad argument.

```rust
#[test]
fn apply_rejects_an_unparsable_config_path() {
    let profile = from_str(P_AMBIGUOUS, Format::Yaml).unwrap();
    let edit = StructuredEdit::AddExact {
        property: "forced_track".to_string(),
        value: Scalar::Bool(true),
    };
    assert_eq!(
        apply_suggestion(&profile, "not-a-rule-path", &edit),
        Err(ApplyError::UnparsableConfigPath("not-a-rule-path".to_string()))
    );
}

#[test]
fn apply_rejects_a_rule_index_past_the_end() {
    let profile = from_str(P_AMBIGUOUS, Format::Yaml).unwrap();  // exactly 1 rule
    let edit = StructuredEdit::AddExact {
        property: "forced_track".to_string(),
        value: Scalar::Bool(true),
    };
    assert_eq!(
        apply_suggestion(&profile, "tracks[7].match", &edit),
        Err(ApplyError::RuleIndexOutOfRange { index: 7, rules: 1 })
    );
}
```

### G4: the silent no-op is detected, not returned as `Ok`

The third exit. A profile that already constrains the property, plus an edit that
would widen it: `core-44`'s `or_insert` keeps the existing constraint, the delta
is dropped, and the model is unchanged. Pre-ruling this returned `Ok(unchanged)`
and reported success for a no-op.

```rust
// The rule already constrains `forced_track: true`. An AddExact carrying
// `false` would widen it, so core-44's or_insert drops the delta and the model
// comes back unchanged. apply_suggestion must NOTICE that (D49) rather than
// return Ok. No re-plan, no batch, no validation - one model comparison.
const P_ALREADY_CONSTRAINED: &str = r#"
profile_version: 1
input: { pattern: 'S(?<s>\d{2})E(?<e>\d{2})', extensions: [mkv] }
tracks:
  rules:
    - match: { exact: { type: subtitles, forced_track: true } }
"#;

#[test]
fn apply_rejects_an_edit_the_no_clobber_merge_drops() {
    let profile = from_str(P_ALREADY_CONSTRAINED, Format::Yaml).unwrap();
    let edit = StructuredEdit::AddExact {
        property: "forced_track".to_string(),
        value: Scalar::Bool(false),
    };
    assert_eq!(
        apply_suggestion(&profile, "tracks[0].match", &edit),
        Err(ApplyError::EditChangedNothing {
            index: 0,
            property: "forced_track".to_string(),
        })
    );
}

// Control: the detection must not fire when the edit really lands. Without
// this, G4 would pass against an `apply_suggestion` that returned
// EditChangedNothing unconditionally.
#[test]
fn apply_returns_ok_when_the_edit_reaches_the_model() {
    let profile = from_str(P_ALREADY_CONSTRAINED, Format::Yaml).unwrap();
    let edit = StructuredEdit::AddExact {
        property: "default_track".to_string(),   // not yet constrained
        value: Scalar::Bool(true),
    };
    let applied = apply_suggestion(&profile, "tracks[0].match", &edit).unwrap();
    assert_eq!(
        applied.tracks.rules[0].match_expr.exact.as_ref().unwrap().get("default_track"),
        Some(&Scalar::Bool(true))
    );
}
```

Measured on the real `with_rule_match` before specifying these: the no-op case
yields `applied == profile` **true** (rule still reads
`{"forced_track": Bool(true), "type": Str("subtitles")}` after the `false` delta),
the unconstrained-property control yields **false**, and an `AddNotExact` append
yields **false**. G4 and its control therefore both discriminate, and neither can
pass vacuously.

### Removal trigger

`proc-proposed-safeguard-stays`: none of these seven is argued out during design.
The vacuity analysis is recorded here instead, aimed at implementation time.

Under D49, G1 and G2's *first* assertion (the engine emits a typed scalar) and
their *second* (apply splices that same scalar) are true by construction, because
`delta_for` reads the edit's own field and there is no other value to read. The
honest candidate-for-removal claim is therefore: "G1/G2 test the compiler." That
claim is measurable exactly one phase later and not before - the named experiment
is: after D49 lands, change `delta_for`'s `AddExact` arm to
`map.insert(property.clone(), Scalar::Str(scalar_display(value)))` and run the
suite. If G1, G2 and G3 all fail, they are load-bearing and stay. If only G3
fails, G1/G2 are candidates for removal as localizers. They stay until that run
happens, because the construction-level identity they pin is one refactor from
being untrue, and the refactor that unpins it will not announce itself.

---

## Interface changes

**Becomes public:** `planner::apply_suggestion`, `planner::ApplyError` (with its
three variants). Nothing else; `edit_key` is private. `with_rule_match` (`planner.rs:1853`) and
`rule_index_of` (`planner.rs:2032`) are **already** `#[doc(hidden)] pub` and are
not touched; `delta_for` (`planner.rs:1809`) **stays private**. `core-90`'s open
question (whether `#[doc(hidden)] pub` extends to production reuse) is not
reached by this design and stays open: `apply_suggestion` lives in `planner.rs`
alongside all three and uses ordinary in-module visibility.

**Changes on the wire:** `StructuredEdit::AddExact.value` and
`StructuredEdit::AddNotExact.value` change from `String` to `Scalar`. Concretely,
for the measured fixture suggestion (§1.9):

```
before: {"kind":"add_exact","property":"forced_track","value":"true"}
after:  {"kind":"add_exact","property":"forced_track","value":true}
```

This wire is consumed at two places, both of which change observably:

1. The Tauri IPC `apply_suggestion` command (D43) - the new direction, so there is
   no prior shape to break.
2. The `dry-run --json` report. The document is built **in core**
   (`crates/muxsmith-core/src/report/json.rs:56` serializes `batch.suggestions`
   whole, so `suggestions[].edit.value` is in it); the CLI is its consumer, not
   its author. This is an observable output change for anyone parsing that
   report. The CLI's own human rendering is unaffected
   (`muxsmith-cli/src/commands/mod.rs:126` prints only `s.yaml_fragment`).

   **No test asserts `edit`'s shape.** The method matters here, because nobody
   re-runs a negative and a grep that returns nothing looks identical whether it
   was the right grep or the wrong one. An earlier draft rested this on
   `grep -rn '"edit"'`, which **cannot** match an unquoted TypeScript object key
   and therefore could not have found the two real hits. What was actually run,
   in a form that could have failed and did return results:

   - `grep -rn "edit" --include="*.ts" --include="*.vue" --include="*.json" .` (excluding `node_modules`), filtered to `edit`-as-a-key -> **2 hits**: `src/ipc.ts:132` (`edit: unknown;`, the type declaration) and `e2e/smoke.spec.ts:177` (`edit: null,`).
   - `grep -rn '\["edit"\]\|"edit"' --include="*.rs" crates/ src-tauri/` -> **0 hits**.

   Both TS hits are inert against this change. `ipc.ts:132` types the field
   `unknown` and reads nothing out of it. `e2e/smoke.spec.ts:177` is a mock IPC
   response whose `edit` is `null`: it never populates `value`, so no `value`
   type can break it. The conclusion stands; the grep that established it does
   not.

`Suggestion` and `DiagCode` do **not** gain `Deserialize`; D43's ruling stands
and `core-37` is untouched. `StructuredEdit` gains `Deserialize` exactly as D43
ruled. `Scalar` needs no change: it already derives `Deserialize`
(`match_expr.rs:17`).

**Serde round-trip, measured** on the exact proposed shape (internally-tagged
outer enum, untagged `Scalar` inner - a combination worth measuring rather than
assuming, since serde buffers internally-tagged content through `Content`):
8 of 8 cases round-trip identically (`Bool`, `Int`, a Float-typed property
carrying `Int`, `Str`, a `Str` whose text is `"true"`, `Float`, an `AddNotExact`,
an `AddSubstring`). The wire keeps the distinctions that matter:
`value: true` -> `Bool(true)` and `value: "true"` -> `Str("true")` deserialize
distinctly; `3`, `3.0` and `"3"` land on `Int`, `Float` and `Str` respectively.

**What a ts-rs binding emits.** D44 already puts `Scalar` on the wire - confirmed:
D44 measures `Scalar` as `boolean | number | number | string` and pins
`TS_RS_LARGE_INT = "number"` specifically to protect `Scalar::Int(i64)` from
becoming `bigint`. D44 does **not** name `StructuredEdit` anywhere, and its
"20 model types" are not enumerated in the document, so D49 binds it explicitly:
`StructuredEdit` carries
`#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]`,
the same attribute D44 puts on the model types, targeting the same single
`profile.ts`.

Measured with ts-rs 12.0.1 and `TS_RS_LARGE_INT = "number"` on the exact proposed
shape, the generated output is:

```ts
export type Scalar = boolean | number | number | string;

export type StructuredEdit = { "kind": "add_exact", property: string, value: Scalar, } | { "kind": "add_not_exact", property: string, value: Scalar, } | { "kind": "add_substring", value: string, } | { "kind": "add_not_substring", value: string, };
```

(The duplicate `number` in `Scalar` is D44's already-recorded cosmetic artifact of
`Int` and `Float` both mapping to `number`.)

**Frontend:** no change required. `src/ipc.ts:132` types `edit: unknown` and D22
confines the GUI to show-and-copy on `yaml_fragment`; the frontend forwards the
edit back opaquely (D43's stated rationale). Measured through
`JSON.stringify(JSON.parse(wire))` in node, the echo is byte-identical for all
three reachable variants and keeps the distinction that matters:

```
OK  {"kind":"add_exact","property":"forced_track","value":true}      -> unchanged
OK  {"kind":"add_exact","property":"id","value":1}                   -> unchanged
OK  {"kind":"add_exact","property":"codec","value":"SubRip/SRT"}     -> unchanged
OK  {"kind":"add_exact","property":"track_name","value":"true"}      -> unchanged
```

One latent trap, recorded because it is invisible and would otherwise be
rediscovered the hard way: JavaScript has no int/float distinction, so an
integral JS number always stringifies without a decimal point. A
`Scalar::Float(3.0)` would serialize as `3.0`, parse to the JS number `3`, and
echo back as `3`, which Rust then deserializes as `Scalar::Int(3)` - a silent
type change across the echo. **This cannot fire under D49** and needs no guard:
`prop_value_as` is a total match over `PropValue`'s four variants and maps
`Float` to `None` (`planner.rs:2068`), so the engine never puts a `Scalar::Float`
in an `AddExact`/`AddNotExact` (§1.4). The claim that makes the guard unnecessary
is that total match, and it is the same one §1.4 rests on. The trap becomes real
the day someone gives `prop_value_as` a `Float` arm; that change, not this ADR,
is where the guard would be owed.

Whether `ipc.ts`'s hand-written `Suggestion` interface is later replaced by the
generated `profile.ts` type is D44's question, not this one.

**Tests:** the 7 sites binding `value` on `AddExact`/`AddNotExact`
(`tests/suggestions.rs:97`, `:100`, `:203`, `:206`, `:325`, `:722`, `:890`) stop
compiling and are updated. They split into two kinds, and both fixes are given
here rather than left to the implementer.

*Four template sites* (`:97`, `:100`, `:203`, `:206`) interpolate `{value}` into a
YAML string template and now hold a `Scalar` rather than a `String`.
`scalar_display` (`planner.rs:846-853`) is private to `planner.rs` and is **not**
made public for this - a test-only rendering does not earn a production API
widening. `tests/suggestions.rs` gets this local helper instead, and the four
sites interpolate `yaml_scalar(value)` in place of `value`:

```rust
// The value as the engine's own `display` string rendered it before D49 moved
// the typed Scalar into the edit; mirrors planner.rs's private `scalar_display`.
// Keeps these fixtures byte-identical to what they produced pre-D49.
fn yaml_scalar(v: &Scalar) -> String {
    match v {
        Scalar::Bool(b) => b.to_string(),
        Scalar::Int(i) => i.to_string(),
        Scalar::Float(f) => f.to_string(),
        Scalar::Str(s) => s.clone(),
    }
}
```

This is byte-identical to today's output, not merely equivalent: today's
`value: String` **is** `prop_value_as`'s `display`, whose three reachable arms
(`b.to_string()`, `i.to_string()`, `s.clone()`, `planner.rs:2065-2067`) are the
same three expressions. The four fixtures therefore keep asserting on exactly the
YAML they assert on now.

*Three comparison sites* (`:325`, `:722`, `:890`) compare `value` against a `&str`
and now compare against a `Scalar`. All three literals are written out, one per
site, each with its property's declared type named. They are **not** the same
shape, and deriving them from the old string is exactly the mistake this ADR
exists to prevent: `Bool(true)` and `Str("true")` both project to `"true"`, so a
derivation reproduces the projection rather than inverting it.

| site | property | declared type | old | new |
|---|---|---|---|---|
| `:325` | `track_name` | `String` (`generated.rs:66`) | `value == "Chapter 1: Intro"` | `value == &Scalar::Str("Chapter 1: Intro".to_string())` |
| `:722` | `forced_track` | **`Boolean`** (`generated.rs:41`) | `value == "true"` | `value == &Scalar::Bool(true)` |
| `:890` | `language` | `String` (`generated.rs:43`) | `value == "eng"` | `value == &Scalar::Str("eng".to_string())` |

`:722` is the one that does not follow `:325`. `Scalar` derives `PartialEq`, so
`value == &Scalar::Str("true".to_string())` there **compiles** and fails only at
runtime, because post-D49 the engine emits `Scalar::Bool(true)` for that Boolean
property.

`:890` is additionally a **negative** assertion
(`!batch.suggestions.iter().any(...)`, `:887-893`): a wrong literal there makes
the `any` match nothing, the negation hold, and the test pass **vacuously**. It
must be `Scalar::Str("eng".to_string())` because `language` is String-typed, and
the reason it is right cannot be inferred from the test going green.

`:509` and `:514` bind `..` and are unaffected. `tests/prop_planner.rs` does not
name `StructuredEdit` and is unaffected; its `with_rule_match` call
(`prop_planner.rs:310`) passes a `MatchExpr` directly and is untouched by the
`delta_for` signature change.

**Not changed by this ADR:** `core-33` (apply still narrows the conflicted rule's
match only - `apply_suggestion` calls `with_rule_match` with a single rule index
and never touches another rule, reorders nothing, and relaxes nothing, since
`delta_for` emits only `exact`/`substring`/`not` additions); `core-44` (the
`or_insert` semantics live in `with_rule_match`, which is called unchanged);
`core-72` (typed equality is now preserved end-to-end rather than broken at the
apply seam); D6's closed grammar (still four variants, still `#[serde(tag)]`);
D43's argument shape (`profile`, `config_path`, `edit`), its refusal of a
re-plan, and its routing of the applied result through the normal validation
pass.
