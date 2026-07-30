# D111: `raw:` compares without type conversion - one comparator, twelve sites, three tests

**Design document for an amendment to Plan 11's Task A3.** Four-role amendment:
this document is authored against the controller's brief
(`.superpowers/sdd/plan-11/amendment-raw-bytewise-design-brief.md`), graded by an
independent reviewer, then folded into
`docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md` by
that plan's own author, with the plan's original reviewer judging the delta. It is
four-role because it re-cuts A3 from a documentation task into a behaviour change
with its own tests.

- **Date:** 2026-07-30. **Tree:** authored at `d5b42c6`; the controller committed
  several changes to `docs/ROADMAP.md` and `docs/decision-ledger.yaml` during authoring
  and the two review rounds, so **every figure in this document was re-verified at
  `e7c109f`** and none moved: R' 8/6, its soundness control 9 with the ROADMAP hit
  still at `:1913` (the new blocks sit after that line), K' 7/6, the `byte` sweep 71,
  the plan's own R 6 and K 9, and the section 13.1 union 103/28. Both files are outside
  the wording surface of section 4.1 by clause 3, which is why commits to them cannot
  move any figure here; the one figure that *would* move if the design itself were
  committed is the plan's own check R, and section 4.1 measures that.
- **Ruling designed to:** owner, 2026-07-30 - *no type casting happens under
  `raw:`*; the SMALL variant (same-type value equality) together with documenting
  it precisely.
- **Supersedes:** the matcher-behaviour half of **D32**'s acceptance case **B-7**
  (`docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md:74`, the matcher
  table's B-7 row: `| B-7 | { new_gain: Float(6.0) } | exact: { raw:new_gain: 6 } |
  yes | int/float cross-compare |`). Everything else in D32 - the opt-in shape, `RawProperty`,
  `RawOnKnownProperty`, `UnknownPropertySkew`, `SchemaDrift`, cases B-1..B-6 and
  B-8..B-11 - is unchanged. D32's dated table is an append-only record and is NOT
  edited; it gains a `superseded by D111` link (trigger T5).
- **D-number collision check, measured:** `D106`-`D110` are reserved by Plan 12
  (`docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md:9`, which
  names `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` as their carrier;
  that file does not exist yet). `D111` is the next free id. If a concurrent
  writer takes it, the controller renumbers at fold-in.
- **Scope:** Task A3 and nothing else. A3's Step 6 (the README example's missing
  `pattern`) and Step 8 (surface-do-not-edit) are untouched. No other Plan 11 task
  is re-cut, no tracker or house-knowledge YAML is edited by this document or by
  the amended task.

---

## 1. The two comparison paths, kept apart

This is the distinction the whole amendment turns on, and conflating it is what
caused real confusion when the finding was first presented.

| path | reached when | comparison | status |
|---|---|---|---|
| typed `exact` | the key has no `raw:` prefix | in the property's own domain: numbers numerically (`6` equals `6.0`), languages canonicalized, closed domains validated | **documented, intended, UNCHANGED** |
| `raw:` opt-in | the key starts with `raw:` | value equality with no type conversion | **changed by this ADR** |

`scalar_eq`'s doc comment already scopes itself to the typed path, verbatim at
`crates/muxsmith-core/src/matcher.rs:199-201`:

```
/// Value equality between a profile `Scalar` and a track `PropValue`, with
/// int/float cross-comparison (spec 4.3, `exact`). Strings compare
/// case-sensitively (language is special-cased before reaching here).
```

The defect was never that the function has cross arms. It was that the `raw:` arm
calls the same function and inherits a coercion that is correct one path over.
**The cross arms stay in `scalar_eq`.** Section 5's first test exists to keep them
there.

---

## 2. Measured basis

Every fact below was re-derived at this tree. Nothing is inherited from the brief,
the plan or the tracker. Commands are pasted; where an authoring figure differed
from mine, section 14 names the delta.

### 2.1 The three brief facts

**F1. `scalar_eq`'s cross arms exist for the typed path.** Verified by reading
`matcher.rs:199-212`. The doc comment is quoted verbatim in section 1. The README
documents the behaviour it enables (`README.md:52`): "numbers numerically (`6`
equals `6.0`)". Tier-2 `core-72-exact-typed-value-equality`
(`docs/conventions.yaml:325-331`) records it as one of the tool's core semantics.
**Do not remove them.**

**F2. Three `scalar_eq` call sites, and what each belongs to.**

```
$ grep -n "scalar_eq" crates/muxsmith-core/src/matcher.rs
103:            Some(have) => scalar_eq(want, &have),
138:            Some(have) => scalar_eq(want, &have),
140:                Some(PropType::Boolean) => scalar_eq(want, &PropValue::Bool(false)),
202:fn scalar_eq(want: &Scalar, have: &PropValue) -> bool {
410:    // value; scalar_eq(Int, Int) holds.
```

- `:103` is the **`raw:` arm** (inside `if let Some(bare) = prop.strip_prefix("raw:")`, which returns before the `match prop` below). The one site this ADR re-points.
- `:138` is the **typed default arm** (the `_ =>` arm of `match prop`, i.e. every known property that is not `language` or `codec_kind`). Unchanged.
- `:140` is the **Boolean false-when-absent shortcut**, in the `None` branch of that same `_ =>` arm, gated on `matchable_type(prop) == Some(PropType::Boolean)`. It belongs to the **typed** path: the `raw:` arm returns at `:102-105` and never reaches it, which is exactly why an absent `raw:` property does not match (case B-6). Unchanged.
- `:202` is the definition, `:410` a comment. So three call sites, not five.

**F3. Both sides are parsed before any comparison, so byte-exactness over the
textual form is unattainable by a comparator change.**

Profile side: `Scalar` is `#[serde(untagged)]` with `Int` declared before `Float`
(`crates/muxsmith-core/src/profile/match_expr.rs:11-31`), and its own doc comment
states the ordering is semantically load-bearing: "only literals with a decimal
point become `Float`".

Reported side: `PropValue::from_json` (`crates/muxsmith-core/src/identify.rs:34-47`)
tries `as_i64()` and falls back to `as_f64()`. **Naming correction:** the brief
calls it `PropValue::from`; the function is `PropValue::from_json`.

Measured at mkvmerge itself, which settles what the reported side can even carry:

```
$ mkvmerge --version
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit

$ mkvmerge -o lum2.mkv --min-luminance 0:400.500 --max-luminance 0:6.00 v.mkv
$ mkvmerge -J lum2.mkv   # JSON tokens, extracted by regex
max_luminance -> JSON token: 6.0
min_luminance -> JSON token: 400.5
```

mkvmerge canonicalizes the decimal text: `6.00` in, `6.0` out; `400.500` in,
`400.5` out. So `6.0` and `6.00` cannot both arrive, and the textual difference is
gone before any comparator exists. **Consequence carried into every replacement in
section 4: no site claims byte-exactness for a numeric comparison, because that
claim was never true and cannot become true.**

### 2.2 Five further measurements the design needs

**M1. Both coercion directions are live today, end to end through the shipped
binary (SI-3), each with a fired negative control.** Probe muxed outside the
repository from the repo's own `tone.wav` seed; `target/debug/muxsmith` is newer
than every tracked `.rs` file (`find crates src-tauri -name '*.rs' -newer
target/debug/muxsmith` returns nothing).

| profile rule | file reports | today | negative control |
|---|---|---|---|
| `exact: { "raw:audio_channels": 1.0 }` | `"audio_channels": 1` (integer token) | **matches**, one `plan.assignments` entry `rule_index 0, track_id 0, track_kind audio` | `2.0` -> `missing-track` |
| `exact: { "raw:max_luminance": 400 }` | `"max_luminance": 400.0` (float token) | **matches**, one entry `rule_index 0, track_id 0, track_kind video` | `401` -> `missing-track` |

Row 1 is the `(Scalar::Float, PropValue::Int)` cross arm, row 2 the
`(Scalar::Int, PropValue::Float)` arm. Both are what this ADR removes from the
`raw:` path. Row 2's reachability is not exotic: mkvmerge writes a `double`
property with a fractional part even when the value is integral (F3), so an
ordinary file reports `400.0` and an ordinary profile writes `400`.

**M2. Schema v20's shape, and the escape hatch's zero members.**

```
$ python3 <schema reader>   # doc/json-schema/mkvmerge-identification-output-schema-v20.json
total track properties: 59
type=number : ['max_luminance', 'min_luminance', 'projection_pose_pitch', 'projection_pose_roll', 'projection_pose_yaw']
counts: number 5 integer 26 string 18 boolean 9 array 1
```

The `array` one is `multiplexed_tracks`, which `PropValue::from_json` drops as
non-scalar. Against the capability model:

```
schema props: 59  model props: 62
in schema, not in model: []
in model, not in schema: ['codec', 'id', 'type']
```

So the model knows all 59 plus three top-level `-J` fields. **Consequence the
design leans on twice:** no profile can reach a `raw:` numeric comparison through
the *sanctioned* route (a property this build's schema lacks), because with this
mkvmerge there is none - which is why both M1 demonstrations had to use a KNOWN
property, the diagnosed-but-legal route. That is also why the change is at its
cheapest now (section 6).

**M3. `language` is string-typed and `codec_kind` is not reported at all.**

```
language      -> {'description': "...ISO 639-2 language code", 'type': 'string'}
language_ietf -> {'description': "...IETF BCP 47/RFC 5646 language tag", 'type': 'string'}
codec_kind    -> ABSENT
```

This is what keeps every `RawOnKnownProperty`-scoped sentence true: the warning's
trigger set is `matches!(bare, "language" | "codec_kind")`
(`crates/muxsmith-core/src/profile/validate.rs:415`), `language` puts a `Str`
against a reported `Str`, and `raw:codec_kind` can never match any track.

**M4. No snapshot, fixture or integration test asserts a numeric `raw:`
comparison.** In this block and in M5's, long lines are elided with `...` for
width; nothing else is altered, and every path and line number is verbatim.

```
$ git grep -n "raw:" -- 'crates/**/tests' 'crates/**/*.snap' 'crates/**/fixtures' 'e2e' 'src' 'src-tauri'
crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_exits_two_and_renders_the_message.snap:5: ...
crates/muxsmith-cli/tests/snapshots/cli_validate__bare_raw_property_renders_german_with_locale_flag.snap:5: ...
e2e/editor-dropdowns.spec.ts:80: test("case 4: a raw:type key keeps its free-text cell ...
e2e/editor-dropdowns.spec.ts:83:  await mountMap(page, EXACT, "tracks[0].match.exact", { "raw:type": "" });
e2e/smoke.spec.ts:329: // ... a bare `raw:` error severity ...
e2e/smoke.spec.ts:346:        config_path: "tracks[0].match.exact.raw:",
src/editor/widgets/PropertyMapWidget.vue:130-131: // ... byte-exact keys `type`/`codec_kind` ...
```

Every hit is the bare-`raw:` error case, a string key, or editor-dropdown
behaviour. In `crates/muxsmith-core/tests/`, the only matcher-relevant `raw:` use
is `planner_resolution.rs:133`, `exact: { raw:new_prop: foo }` - a string.
`validate_semantics.rs` uses `raw:dolby_complexity_index: 3` but never runs the
matcher. **The instrument demonstrably reaches those files:** it returned snapshot
content, and the control `grep -rln "exact" crates/muxsmith-core/tests/` returns
18 files. So the behaviour change touches no snapshot and no e2e expectation, and
the only executable assertion of the coercion in the tree is `matcher.rs`'s
`b7_raw_int_float_cross_compare`.

**M5. What a `raw:` non-match actually looks like after the change.** Ran against
the M1 probes with the value that does not match.

Non-optional rule, human rendering:

```
[info] tracks[0].match.exact.raw:max_luminance: Property "max_luminance" is used with a raw: prefix; it bypasses the capability model and is matched untyped. ...
src2/lum.mkv (identifier: lum.mkv)
[error] tracks[0].match: No track matches this non-optional rule.
[warning] tracks[0].match: Property "max_luminance" was matched untyped through a raw: opt-in ...
```

`--json` for the same run: `missing-track` carries `"params": {}` and the
document's `suggestions` array is `[]`. A control with a KNOWN-property
non-match (`exact: { type: video, language: eng }`) renders the identical bare
error and also `suggestions: []`.

Optional rule, and **two probes are needed here, which is why the second one exists**.
The single-rule probe below is not sufficient for the claim it was first written to
carry: with only one rule, the plan resolves to zero tracks, so a third diagnostic
fires precisely BECAUSE the rule matched nothing, and that is a signal the claim
denies. Both probes were run; both exit codes measured with `$?`.

*(A) single-rule, `optional: true`, `raw:max_luminance: 401` against a reported
`400.0` - shows the confound:*

```
[info] ... raw:max_luminance ... matched untyped ...
src2/lum.mkv (identifier: lum.mkv)
  rule 0 -> track -
[warning] tracks[0].match: Property "max_luminance" was matched untyped ...
[warning] tracks: This plan resolves to zero output tracks; ...
```

**exit 1.** No error, but the zero-tracks warning is a discriminating signal, so this
probe does not support the no-signal claim.

*(B) two rules - rule 0 `exact: { type: video }` matches, rule 1 optional `raw:` does
not - the probe the claim actually needs:*

```
[info] tracks[1].match.exact.raw:max_luminance: Property "max_luminance" is used with a raw: prefix; ...
src2/lum.mkv (identifier: lum.mkv)
  rule 0 -> track 0
  rule 1 -> track -
  output: out/lum.mkv
[warning] tracks[1].match: Property "max_luminance" was matched untyped through a raw: opt-in ...
1 file matched (searched src2, extensions mkv)
```

**exit 1.** Diagnostic codes, from `--json`: `raw-property` (info) and
`unknown-property-skew` (warning), nothing else; no zero-tracks warning;
`suggestions: []`.

**Exit codes across all four cases, measured rather than assumed, because the first
draft of this document asserted exit 0 for the optional case and was wrong:**
`severity_exit` (`crates/muxsmith-cli/src/commands/mod.rs:25`) maps a worst severity
of Error to 2, Warning to 1, else 0, and `unknown-property-skew` is a warning that
fires on any consumed `raw:` property.

| case | worst diagnostic | exit |
|---|---|---|
| `raw:` match SUCCEEDS | skew warning | **1** |
| required rule, `raw:` non-match | `missing-track` error | **2** |
| optional rule, `raw:` non-match, probe (A) | zero-tracks warning | **1** |
| optional rule, `raw:` non-match, probe (B) | skew warning | **1** |

**So the exit code discriminates nothing about the optional case:** 1 is also what a
working `raw:` rule produces. This is decision-relevant for section 6 and it refutes
part of the tracker's recorded reasoning (section 14, P4). It does NOT support the
word "silently": the human rendering always prints `rule N -> track -` and the exit
is never 0.

---

## 3. Decision 1: the comparator

### 3.1 The semantics, exhaustively

The `raw:` arm compares a profile `Scalar` against a reported `PropValue` and
holds only when both carry the same kind. All sixteen pairs, plus absence:

| profile `Scalar` \ reported `PropValue` | `Bool(b)` | `Int(j)` | `Float(g)` | `Str(t)` | absent |
|---|---|---|---|---|---|
| `Bool(a)` | `a == b` | false | false | false | false |
| `Int(i)` | false | `i == j` | false | false | false |
| `Float(f)` | false | false | `f == g` | false | false |
| `Str(s)` | false | false | false | `s == t` | false |

Reading of the four diagonal cells, so nothing is left to inference:

- `Bool`: `bool` equality.
- `Int`: `i64` equality.
- `Float`: **IEEE `f64` equality, unchanged from the existing `(Float, Float)`
  arm.** Two consequences follow from IEEE and neither is introduced here: a
  profile `.nan` never matches anything, and `-0.0` matches a reported `0.0`. The
  same-type rule is about kinds, not about bit patterns, and no site may describe
  it as bit-level or byte-level comparison of numbers.
- `Str`: `String` equality, which in Rust is byte-wise comparison of the UTF-8
  encoding. Case-sensitive, no Unicode normalization, no `language`
  canonicalization (the `raw:` arm runs before that special case). This is the one
  place where "byte-for-byte" is precise, and it is why the retained sentences in
  section 4.4 keep that word.
- **absent**: false for every kind, without consulting `matchable_type`. Unchanged
  (case B-6); the Boolean false-when-absent shortcut stays on the typed path.

**Not governed by this table, stated so the boundary is not inferred:**
`substring` and `regex` under a `raw:` prefix. Both strip the prefix
(`matcher.rs:185-187`) and read the value through `item_str`
(`matcher.rs:192-197`), which yields a value only for `PropValue::Str`. No scalar
comparison happens on those paths, and this ADR changes nothing about them.

### 3.2 Where it lives, and why

**A new private function `scalar_eq_same_type` in `crates/muxsmith-core/src/matcher.rs`, next to `scalar_eq`, with `scalar_eq` expressed in terms of it.**

Rationale, in order:

1. The `raw:` arm reads its own comparator by name at the call site, so the two
   paths are distinguishable where the choice is made. That is precisely the
   information the old code hid.
2. Layering `scalar_eq = scalar_eq_same_type + two cross arms` puts the *entire*
   difference between the paths in one expression. The defect existed because the
   difference was invisible; a structure that states it cannot re-hide it.
3. One enumeration of the same-type pairs. `Scalar` and `PropValue` are parallel
   enums; a future kind (a `Bytes` variant, a decimal) has to be added in exactly
   one place instead of two that can silently disagree.
4. `scalar_eq`'s existing doc comment stays true and gains a pointer, so a later
   reader cannot re-point the `raw:` arm at it by accident.

The verbatim code. **Verified `rustfmt`-clean** by copying it to a scratch file and
running `rustfmt --edition 2024` (workspace edition 2024, toolchain 1.96.1, no
`rustfmt.toml`): zero diff. `cargo fmt --all --check` is a Plan 11 exit bar, and a
fenced form that rustfmt rewrites has cost this project a review round before.

```rust
/// Value equality WITHOUT any type conversion, for the `raw:` opt-in path
/// (spec 4.4, 9.2): a profile `Scalar` equals a reported `PropValue` only when
/// both carry the same kind. Deliberately NO int/float cross arms - `raw:` is
/// the declared untyped path, where Muxsmith takes no type decision on the
/// user's behalf, so `raw:x: 6` does not match a reported `6.0`. Strings
/// compare byte-wise, `language` is not normalized here (the `raw:` arm runs
/// before that special case). The typed `exact` path uses [`scalar_eq`],
/// which is exactly this function plus the two numeric cross arms; a new
/// `Scalar`/`PropValue` kind is added here, not there.
fn scalar_eq_same_type(want: &Scalar, have: &PropValue) -> bool {
    match (want, have) {
        (Scalar::Str(a), PropValue::Str(b)) => a == b,
        (Scalar::Bool(a), PropValue::Bool(b)) => a == b,
        (Scalar::Int(a), PropValue::Int(b)) => a == b,
        (Scalar::Float(a), PropValue::Float(b)) => a == b,
        _ => false,
    }
}

/// Value equality for the TYPED `exact` path (spec 4.3): the same-type
/// equality of [`scalar_eq_same_type`] plus int/float cross-comparison, so a
/// profile `6` matches a reported `6.0` on a known property - documented
/// behaviour (README, spec 4.3) that mkvmerge's own float-typed properties
/// need, since it reports an integral `max_luminance` as `400.0`. Strings
/// compare case-sensitively (`language` is special-cased before reaching
/// here). The `raw:` path deliberately does NOT call this function.
fn scalar_eq(want: &Scalar, have: &PropValue) -> bool {
    scalar_eq_same_type(want, have)
        || match (want, have) {
            (Scalar::Int(a), PropValue::Float(b)) => (*a as f64) == *b,
            (Scalar::Float(a), PropValue::Int(b)) => *a == (*b as f64),
            _ => false,
        }
}
```

**Truth-table identity of the rewritten `scalar_eq`, argued arm by arm because it
is the correctness core:** the original had `Str/Str`, `Bool/Bool`, `Int/Int`,
`Int/Float`, `Float/Float`, `Float/Int`, `_ => false`. The new form evaluates the
four same-kind arms first; when the kinds match but the values differ,
`scalar_eq_same_type` returns false and the second `match` falls through its two
cross arms to `_ => false`, which is what the original arm returned. When the kinds
differ, only a cross arm can fire, exactly as before. Same function, and section 5's
first test pins it from the outside.

`(*a as f64)` is preserved verbatim from the existing code; clippy's
`cast_precision_loss` is pedantic-group and the gate (`-D warnings` on default
groups) already passes with it.

### 3.3 The call site

In `exact_matches`, the `raw:` arm's call becomes:

```rust
            Some(have) => scalar_eq_same_type(want, &have),
```

`:138` and `:140` keep `scalar_eq`.

**Scope of the code change, stated as a bounded claim rather than as an absolute,
because the absolute is falsified two sections later by this document itself:** no
line of NON-TEST code changes anywhere in the workspace outside the comparator pair
of section 3.2 (`scalar_eq_same_type` added, `scalar_eq`'s body and doc comment
rewritten) and this one re-pointed call site at `:103`. Inside the `tests` module of
the same file, four changes DO land and they are enumerated: R-11 (the B-5 comment),
R-12 (the B-7 test), and section 5's T-1 and T-3. Reading the old absolute literally
would put the test changes out of scope, which is the inverse of section 13's own
voiding of the plan's "none may be inside the `tests` module".

### 3.4 What explicitly does not change

`Scalar`, `PropValue`, `PropValue::from_json`, `scalar_fits`,
`raw_opt_in_diagnostic`'s trigger set, the planner's `UnknownPropertySkew`
emission, `matchable_type`, `codec_kind_prefixes`, the profile JSON schema, the TS
bindings, every `DiagCode`, every Fluent key. Section 11 states the interface
consequence.

---

## 4. Decision 2: the wording target at every site

### 4.1 The surface, stated as a rule rather than a list

**A site is in scope iff it is a *live* artifact.** Live means: not history, not an
append-only decision record, not a controller-owned tracker. Concretely, the tree
minus

1. `docs/process-journal*` and `docs/superpowers/plans` (history),
2. **every dated document under `docs/superpowers/specs` except
   `2026-07-08-muxsmith-v1-design.md`** - the v1 spec is the one living
   authoritative document there; the rest are dated ADR carriers, and the doctrine
   makes ADRs append-only with `superseded by` links rather than edits,
3. `docs/decision-ledger.yaml`, `docs/conventions.yaml`,
   `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`,
   `docs/ROADMAP.md` (controller-owned single-writer files; surfaced in section 12,
   never edited by a task),
4. `Cargo.lock` (a lockfile carries no prose claim).

**Why a rule and not the plan's five-path enumeration - measured, not argued.** An
enumeration of dated spec files goes stale the day the next design document lands,
and it already has: this document is
`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`, the plan's Step-7
pathspec excludes only the 07-11, 07-21 and 07-28 specs, and this document quotes the
retired phrases ten times. **Run today the plan's own check R still returns 6, because
`git grep` skips untracked files and this document is untracked. Run with
`--untracked` - which is what committing it makes permanent - it returns 16 lines
across 6 files, ten of them inside this document.** So the plan's check R is not
merely fragile, it breaks at execution time, and an implementer would meet 16 where
the plan promises 6. Clause 2 of the rule above excludes it by construction. This is
the concrete form of trigger T12, and it is why section 13.5 replaces the plan's
pathspec rather than only its expected counts.

**Two invocations, not one, and this is a correctness requirement rather than a
style preference.** Git applies exclusion after inclusion, so adding the v1 spec as a
positive pathspec alongside `':!docs/superpowers/specs'` does NOT re-include it: the
file is silently dropped and the count comes back short. Measured on the
`untyped equality` expression of section 4.2: the one-invocation form returns **1**
and the two-invocation form returns **2**, the missing line being the v1 spec's `:280`
- one of the two sites R-6 repairs. A short count from this shape looks exactly like a
clean result.

Because git pathspec exclusion beats inclusion, the surface is searched with two
invocations and the counts summed:

```bash
# (a) everything except the specs directory
git grep -nE '<expr>' -- . \
  ':!docs/process-journal*' ':!docs/superpowers/plans' ':!docs/superpowers/specs' \
  ':!docs/decision-ledger.yaml' ':!docs/conventions.yaml' \
  ':!docs/process-conventions.yaml' ':!docs/product-boundaries.yaml' \
  ':!docs/ROADMAP.md' ':!Cargo.lock'
# (b) the one living spec
git grep -nE '<expr>' -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

### 4.2 How the site set was derived, and the discriminator that splits it

**Stage 1, subject-first and alternation-free.** The claim class is "a statement
about what the `raw:` comparison does", so the subject is the feature, not a
vocabulary. A line-based grep is structurally blind to hard-wrapped prose (the
`proc-wrapped-prose-quote-grep` shape: `raw:` on one line, `byte-literal equality`
on the next), so the instrument is **block-based**: markdown paragraphs, contiguous
Rust comment runs, and Fluent message groups over the live surface, selecting every
block that mentions `raw` (`\braw\b|raw:|raw-|Raw[A-Z]|raw_`). That is 158+105 =
263 blocks; narrowing to blocks that also carry a comparison-characterizing term
(`equalit|equal\b|equals|byte|literal|untyped|untypis|typisiert|gleichheit|verglich|numerical|numerisch|coerc|cross-compar|verbatim|wörtlich|type check|typ(en)?prüfung|domain`)
leaves **58 candidate blocks**, every one of which was read.

**Stage 2, classification by reading.** Of the 58, most are the word "raw" in
unrelated senses (raw output lines D24, raw text, raw HTML, a raw capture group, a
raw code string, byte sizes, multi-byte UTF-8 slicing). The blind-spot layer - the
105 blocks that mention `raw` with *no* comparison term, and the alternation-free
`byte` sweep's other hits (`src/help/topics.ts` "byte-for-byte the ...",
`lib.rs` "byte-identical by construction", `profile/load.rs` byte sizes,
`planner.rs` multi-byte tails, `identify.rs` `size` in bytes) - was inspected and
contains no claim about the `raw:` comparison.

**The discriminator, and it is checkable rather than a taste sequence:**

- **"untyped" describing the PATH or the BYPASS stays.** It means "the capability
  model is not consulted", which remains exactly true: no existence, type or domain
  check runs on a `raw:` key.
- **"untyped" describing the EQUALITY moves.** "untyped equality" says the equality
  itself ignores type. After this ADR the equality *requires* the kinds to agree,
  so the phrase becomes false. This is a correctness call, not a style call, and it
  is greppable:

  ```bash
  $ git grep -niE 'untyped equality|untypisierte[a-z]* (Wert)?[Gg]leichheit|equality[^.]{0,20}untyped' -- . ':!Cargo.lock'
  ```

  Over the live surface this returns exactly two lines:
  `crates/muxsmith-core/src/report/mod.rs:87` and the v1 spec `:280`. (Everything
  else it finds is journal diffs and the Plan 11 document's own quotation of the
  alternations.)
- **"byte-exact"/"byte-literal"/"byte-genau" as the NAME of the whole `raw:`
  comparison moves.** It asserted two things at once - no coercion (false then) and
  textual identity (unattainable, F3) - and that ambiguity is what produced the
  defect.
- **The same words describing a STRING comparison stay.** For `String` they are
  precise (section 3.1), and there they contrast with *normalization*, not with
  typing, which is the point those sentences are making.

### 4.3 The repair set: 12 sites, each with its exact replacement

**R-1. `README.md:60`** - the `raw:` bullet of the matching-magic list. Register is
the owner's sell-tone, a case-scoped exception to the house writeup voice; the
bullet's frame ("every convenience above switches off") and its tail stay. Replace
exactly

```
byte-exact value equality against that one field, named verbatim.
```

with exactly

```
plain value equality against that one field, named verbatim, and no type conversion either: `6` matches a reported `6`, never a reported `6.0`.
```

**R-2. `crates/muxsmith-core/src/matcher.rs:96-101`** - the `raw:` arm's comment.
Replace exactly (eight-space indent, as in the file)

```
        // `raw:` opt-in (D32, spec 9.2): untyped byte-literal value equality
        // against the property named verbatim. It bypasses the `language`
        // normalization and `codec_kind` alias arms below, and it takes no
        // false-when-absent Boolean shortcut - the bare name's type is unknown
        // to the capability model (`matchable_type` is `None`), so an absent
        // raw: property simply does not match (B-6).
```

with exactly

```
        // `raw:` opt-in (D32, spec 9.2; semantics D111): matched untyped -
        // the capability model is not consulted - and with NO type conversion
        // in the comparison itself: value equality against the property named
        // verbatim, holding only where the profile value and the reported
        // value carry the same kind, so an `Int` scalar does not match a
        // reported `Float` (case B-7, ruled 2026-07-30). The typed `exact`
        // path keeps that coercion in `scalar_eq`; this arm calls
        // `scalar_eq_same_type`. It also bypasses the `language`
        // normalization and `codec_kind` alias arms below, and it takes no
        // false-when-absent Boolean shortcut - the bare name's type is
        // unknown to the capability model (`matchable_type` is `None`), so an
        // absent raw: property simply does not match (B-6).
```

**Why this wording and not the shorter "untyped value equality":** that phrase
attaches "untyped" directly to the equality, which section 4.2 rules false and
section 8 uses as R-6's and R-7's whole ground. The replacement keeps "matched
untyped" in the permitted PATH sense, marks the boundary with a dash, and states
the type rule separately. **The retirement expression of section 4.2 cannot see the
bad form**, in either its flat or its hard-wrapped shape, so it does not protect
the replacements; section 4.6 therefore adds a loosened form of it that runs over
the replacement texts as well, and it was run after this edit rather than before.

**Two neighbouring uses of "untyped" that are retained, named because they sit
within two sites of a phrase being retired and a reader will otherwise take the
boundary as accidental:** `matcher.rs:407`'s section comment "`raw:` opt-in matcher
cases B-5..B-8 (untyped comparison)" and R-11's retained clause "compares untyped
by value". Both read as the PATH sense - the comparison performed on the untyped
path - and neither attaches "untyped" to the word "equality", which is the
construction the discriminator retires. They stay unedited.

**R-3. v1 spec `:176`** - section 4.4's `raw:` opt-in bullet. Replace exactly

```
is matched untyped (byte-literal value equality against the property named verbatim, no `language` normalization or `codec_kind` aliasing, no false-when-absent Boolean shortcut)
```

with exactly

```
is matched untyped (value equality against the property named verbatim with no type conversion: a string matches only a reported string - byte-for-byte, no `language` normalization, no `codec_kind` aliasing - an integer only an integer, a float only a float, a boolean only a boolean, so `exact: { raw:x: 6 }` does not match a reported `6.0` the way `exact` on a known property does; and no false-when-absent Boolean shortcut)
```

**R-4. v1 spec `:421`, FIRST occurrence** - section 9.2's runtime paragraph.
Replace exactly

```
and is matched untyped: byte-literal value equality against the property named verbatim, with no `language` normalization, no `codec_kind` alias expansion, and no false-when-absent Boolean shortcut.
```

with exactly

```
and is matched untyped: value equality against the property named verbatim with no type conversion, so the profile value matches only a reported value of the same kind (a string only a string, compared byte-for-byte; an integer only an integer; a float only a float; a boolean only a boolean) and `raw:x: 6` does not match a reported `6.0`; with no `language` normalization, no `codec_kind` alias expansion, and no false-when-absent Boolean shortcut.
```

**R-5. v1 spec `:146`** - section 4.3's typed-equality statement, which becomes
over-broad the moment 4.4 says the `raw:` path takes no type decision. It is the
authoritative document and the two sentences sit in one continuous normative text
with no heading boundary between them, so the scope is stated rather than implied.
This is a one-sentence append; the existing sentence is the anchor. Replace exactly

```
Use `regex` for byte-literal matching.
```

with exactly

```
Use `regex` for byte-literal matching. A `raw:`-prefixed key is outside all of this: it has no domain, and it compares without type conversion (4.4).
```

**R-6. v1 spec `:280`** - the `RawOnKnownProperty` row of section 7's diagnostics
table (this is decision item 6, section 8). Replace exactly

```
| `RawOnKnownProperty` | warning | `raw:` applied to a model property with special matching semantics (`language`, `codec_kind`), degrading it to byte-literal untyped equality (config-time; 4.4, 9.2) |
```

with exactly

```
| `RawOnKnownProperty` | warning | `raw:` applied to a model property with special matching semantics (`language`, `codec_kind`), degrading it to plain value equality against the property named verbatim, with no normalization and no type conversion (config-time; 4.4, 9.2) |
```

**R-7. `crates/muxsmith-core/src/report/mod.rs:87`** - the `RawOnKnownProperty`
DiagCode doc, the code-side twin of R-6. Replace exactly

```
the prefix degrades it to byte-literal untyped equality, bypassing those semantics
```

with exactly

```
the prefix degrades it to plain value equality against the property named verbatim (no normalization, no type conversion), bypassing those semantics
```

**R-8. `help/en/editor-match-expr-exact.md:23`** - the "The `raw:` bypass"
paragraph. Prose with code spans only: no pipe, no external URL, no raw HTML, all
three of which `pnpm check:i18n` hard-fails on over `help/`. Replace exactly

```
no absent-means-false shortcut - plain byte-for-byte value equality against the property named verbatim.
```

with exactly

```
no absent-means-false shortcut, and no type conversion - a value matches only a reported value of the same kind, so `6` does not match a reported `6.0` here even though it does under typed equality above.
```

**R-9. `help/de/editor-match-expr-exact.md:23`** - "Der `raw:`-Bypass". Reuses the
topic's own established vocabulary from its `## Typisierte Gleichheit` section
("Zahlen vergleichen numerisch", "Typisierte Gleichheit") rather than coining a
fresh translation. Replace exactly

```
kein Fehlend-heißt-false - nur byte-genaue Wertgleichheit gegen die wörtlich benannte Eigenschaft.
```

with exactly

```
kein Fehlend-heißt-false und keine Typumwandlung - ein Wert matcht nur einen gemeldeten Wert derselben Art; `6` matcht hier also kein gemeldetes `6.0`, anders als bei der typisierten Gleichheit oben.
```

**R-10. `crates/muxsmith-core/src/matcher.rs:199-201`** - `scalar_eq`'s doc
comment, plus the new `scalar_eq_same_type` above it. Not false today, but after
this ADR it is the only place a reader learns that two comparators exist and which
path each serves. The verbatim replacement is the fenced block in section 3.2.

**R-11. `crates/muxsmith-core/src/matcher.rs:409-410`** - the B-5 test comment,
which cites a function that will no longer be on the `raw:` path. Neither of Plan
11's alternations can see this line: it contains no member of either. Replace
exactly

```
    // B-5: a raw: unknown property present on the track compares untyped by
    // value; scalar_eq(Int, Int) holds.
```

with exactly

```
    // B-5: a raw: unknown property present on the track compares untyped by
    // value; scalar_eq_same_type(Int, Int) holds.
```

**R-12. `crates/muxsmith-core/src/matcher.rs:444-449`** - the B-7 test, whose
assertion is the behaviour change itself. Full replacement text in section 5.

### 4.4 The retained set, with the reason

**Seven sites keep their wording** (the Plan 11 retain set minus R-6 and R-7). Each
is scoped to `language`/`codec_kind`, both established string-typed by M3, and each
uses "byte-literal" to contrast with *normalization*, which is what it accurately
describes:

| site | wording | why it stays |
|---|---|---|
| v1 spec `:421`, second occurrence | "that `raw:` degrades to byte-literal equality" | scoped to `language`/`codec_kind` in the same clause; after R-4 the paragraph reads as the general rule followed by its string specialization. This discharges the doctrine's self-contradiction sweep for that paragraph. |
| `crates/muxsmith-core/src/profile/validate.rs:408` | "which `raw:` degrades to byte-literal equality" | doc of the funnel whose trigger set is those two properties, in code at `:415` |
| `crates/muxsmith-core/tests/validate_semantics.rs:249` | "degrading the match to byte-literal equality" | the B-4 test comment; subject is the diagnostic, scope is the same two properties |
| `locales/en/diagnostics.ftl:14` | "...bypasses them (language normalization, codec_kind aliasing) and matches byte-literally instead" | "instead" contrasts with normalization; true of a string comparison. **User-visible product text in two locales; a bilingual product-text change riding a semantics fix earns nothing here.** |
| `locales/de/diagnostics.ftl:21` | "...und gleicht stattdessen byte-literal ab" | same, and the pair must move together or not at all |
| `crates/muxsmith-core/src/matcher.rs:452` | "it byte-literally compares against the `language` property alone" | B-8's subject is single-field lookup and no normalization, a different fact that this ADR does not touch |
| `crates/muxsmith-core/src/matcher.rs:466` | "Byte-literal against the `language` field itself still works." | same |

The **b8 test name** `b8_raw_language_is_byte_literal_no_normalization` also stays:
its subject is the no-normalization property, the word is accurate for a string, and
Tier-1 `core-98-raw-language-single-field` plus D32's B-8 row cite the case by
name. Renaming it buys no accuracy and costs cross-references.

**Two further sites are outside the claim class entirely** and are named because a
`byte` vocabulary sweep finds them and a later reader would otherwise think this
amendment missed them. Both re-verified by reading:

- `src/editor/widgets/PropertyMapWidget.vue:130` - "only for the byte-exact keys
  `type`/`codec_kind`" is about which KEY STRINGS get a dropdown, where byte
  equality of the key name is exactly what happens (and a `raw:type` key fails the
  test and keeps its free-text cell). TRUE.
- `e2e/editor-dropdowns.spec.ts:80` - the case-4 test name, same subject. TRUE.

### 4.5 Delta against Plan 11's six-repair / nine-retain split

Both of the plan's expressions reproduce exactly at this tree: check R returns
**6 lines across 5 files**, check K returns **9 lines across 7 files**, member for
member as the plan lists them. So the split was correctly derived for the question
it was asked. The delta comes from the ruling, not from a measurement error:

| | plan | this ADR | delta |
|---|---|---|---|
| repaired | 6 | **12** | +6: two moved from the retain set (R-6, R-7, the "untyped equality" pair) and four that neither alternation can see (R-5, R-10, R-11, R-12) |
| retained | 9 | **7** | -2, the same pair |
| different-claim, named not edited | 2 | 2 | unchanged |
| behaviour change | none | one comparator arm + one call site | new |
| new tests | none ("no new test", on the ground that B-7 already covers it) | **three** (section 5) | the plan's ground inverts: B-7 asserts the behaviour being removed |

**Why the four invisible sites are invisible, which is the finding rather than a
footnote:** the plan's R alternation lists the four retired phrases and its K
alternation the six retained ones. R-5 (`spec:146`), R-10 (`scalar_eq`'s doc),
R-11 (the B-5 comment) and R-12 (the B-7 test) contain no member of either, because
they are falsified by the *semantics* change rather than by carrying the retired
word. A vocabulary sweep cannot find a site that becomes wrong without changing its
vocabulary; only a subject-first sweep can (section 4.2).

The plan's alternation-free `byte` sweep expectation of **100** lines also
reproduces verbatim. Under the surface rule of section 4.1 it is **71** (67 outside
the specs directory plus 4 in the v1 spec). The 29-line difference is fully
reconciled by the dated specs the rule additionally excludes: plan-2 (1), 5.6 (1),
5.7 (4), plan-6 (9), 6-apply-seam (4), plan75 (1), plan8 (9) = 29.

### 4.6 The amendment's verification expressions

These replace Step 7's checks R and K. **Each is fenced below as its two complete
invocations with the pathspec inline, because an expression whose surface lives in
the prose beside it is not executable as written** - the exact shape the plan being
amended spent a round removing from its own instruments (plan line 972: "and this one
read stdin when it was run verbatim"; plan line 948 lists an elided file selector
among five latitude-by-omission occurrences). Sum the two counts. Run them from the
repository root; in `zsh` the pathspec must not be passed as one unquoted string
parameter, which silently matches nothing.

**Check R' - absence of the retired vocabulary.** The alternation is derived by
reading each of the twelve repair sites, and it drops nothing that a retained site
carries (the retained `byte-literally compares` and `Byte-literal against` are
different strings from `matches byte-literally`):

```bash
git grep -nE 'byte-literal value equality|byte-exact value equality|byte-for-byte value equality|byte-genaue Wertgleichheit|byte-literal untyped equality' -- . \
  ':!docs/process-journal*' ':!docs/superpowers/plans' ':!docs/superpowers/specs' \
  ':!docs/decision-ledger.yaml' ':!docs/conventions.yaml' \
  ':!docs/process-conventions.yaml' ':!docs/product-boundaries.yaml' \
  ':!docs/ROADMAP.md' ':!Cargo.lock'
git grep -nE 'byte-literal value equality|byte-exact value equality|byte-for-byte value equality|byte-genaue Wertgleichheit|byte-literal untyped equality' \
  -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

RED on the pre-state, measured: **8 lines across 6 files** - `README.md:60`,
`crates/muxsmith-core/src/matcher.rs:96`,
`crates/muxsmith-core/src/report/mod.rs:87`, `help/de/editor-match-expr-exact.md:23`,
`help/en/editor-match-expr-exact.md:23`, and the v1 spec at `:176`, `:280`, `:421`.
GREEN on the end state: **0**. Reachable green argued member by member: each of the
eight lies inside one of the fenced replacements R-1..R-9, and no replacement text
contains any alternative. (R-5's replacement keeps "Use `regex` for byte-literal
matching" verbatim, which is not an R' member: the alternation requires
`byte-literal value equality`.)

**Soundness control for R', pointed at a target measured to contain a match.** Run
R''s first invocation with the `':!docs/ROADMAP.md'` exclusion dropped: **9** on the pre-state
(measured; the ninth is `docs/ROADMAP.md:1913`) and **1** on the end state, the
survivor being the ROADMAP's own sentence describing this defect ("`raw:` arm call
the comparison an untyped byte-literal value equality"), cited by wording because a
co-writer edits that file. The control's target was verified present before being
prescribed.

**Check K' - invariance of the retained set.**

```bash
git grep -nE 'to byte-literal equality|matches byte-literally|byte-literal ab\.|byte-literally compares|Byte-literal against' -- . \
  ':!docs/process-journal*' ':!docs/superpowers/plans' ':!docs/superpowers/specs' \
  ':!docs/decision-ledger.yaml' ':!docs/conventions.yaml' \
  ':!docs/process-conventions.yaml' ':!docs/product-boundaries.yaml' \
  ':!docs/ROADMAP.md' ':!Cargo.lock'
git grep -nE 'to byte-literal equality|matches byte-literally|byte-literal ab\.|byte-literally compares|Byte-literal against' \
  -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

**7 lines across 6 files on the pre-state AND the end state**, measured:
`matcher.rs:452` and `:466`, `profile/validate.rs:408`,
`tests/validate_semantics.rs:249`, `locales/de/diagnostics.ftl:21`,
`locales/en/diagnostics.ftl:14`, v1 spec `:421`. That is section 4.4's table member
for member. Its fire, because an invariance check that never moves cannot be told
from a broken one: delete the `matches byte-literally` clause in
`locales/en/diagnostics.ftl`, re-run, watch 7 become 6, restore, and prove the
restore with `git diff --exit-code -- locales/`.

**Vocabulary sweep, alternation-free, because an alternation cannot audit itself.**

```bash
# -nE, not -niE, and Cargo.lock excluded: both narrowings are reasoned below
git grep -nE 'byte' -- . \
  ':!docs/process-journal*' ':!docs/superpowers/plans' ':!docs/superpowers/specs' \
  ':!docs/decision-ledger.yaml' ':!docs/conventions.yaml' \
  ':!docs/process-conventions.yaml' ':!docs/product-boundaries.yaml' \
  ':!docs/ROADMAP.md' ':!Cargo.lock'
git grep -nE 'byte' -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

**71** lines on the pre-state, of which **19 distinct lines are classified sites and
52 are noise** - measured by classifying all 71 mechanically, not by adding the
sets up. The 19:

- **9 repair lines** carrying the word: `README.md:60`, `matcher.rs:96`,
  `report/mod.rs:87`, `help/en/editor-match-expr-exact.md:23`,
  `help/de/editor-match-expr-exact.md:23`, v1 spec `:146`, `:176`, `:280`, `:421`.
- **6 retained lines**: `profile/validate.rs:408`, `tests/validate_semantics.rs:249`,
  `locales/en/diagnostics.ftl:14`, `locales/de/diagnostics.ftl:21`,
  `matcher.rs:452`, and `matcher.rs:457` - the b8 test's NAME, which carries
  `byte_literal` in identifier form and which neither R' nor K' can match.
- **2 different-claim lines**: `PropertyMapWidget.vue:130`,
  `e2e/editor-dropdowns.spec.ts:80`.
- **2 lines that are true and about `regex`, not `raw:`**:
  `help/en/editor-match-expr-exact.md:11` ("for byte-literal pattern matching use
  `regex`") and `help/de/editor-match-expr-exact.md:11` ("für byte-genaue Muster
  `regex`"). Unchanged; the
  same clause survives inside R-5's replacement in the spec.

The 52 noise lines are byte arrays, byte sizes, encodings, multi-byte UTF-8 slicing,
and `byte-identical`/`byte-for-byte` about documents, snapshots and rendered HTML;
the report names them by kind, spread over 25 files. **A hit that is a false
unscoped claim about `raw:` and is not one of the 19 is a finding**, because it means
R' has a hole.

Two narrowings, each with its reason: `-nE` rather than `-niE`, because the audit
hunts prose claims and `getByText` in the Playwright specs matches `byte`
case-insensitively (three e2e files alone contribute 157 hits under `-i`); and
`':!Cargo.lock'`, because a lockfile carries no prose claim.

**Where this sweep does not look, measured rather than reasoned.** The
case-sensitive form is blind to a sentence-initial capital: `matcher.rs:466` reads
"`// Byte-literal against the `language` field itself still works.`" and does NOT
appear among the 71, which is why section 4.4 lists **seven** retained sites while
the sweep classifies six. It is caught by K', whose alternation carries
`Byte-literal against` with the capital. So the retained set is covered by K' and
the sweep together, never by the sweep alone. Three further repair sites (R-10,
R-11, R-12) contain no form of the word at all and are invisible to every
vocabulary instrument here; only the subject-first block sweep of section 4.2 and
the tests of section 5 reach them.

**Check R'' - the retirement expression run over the REPLACEMENTS, not only over the
pre-state.** R' is a fixed-phrase alternation and it cannot see a *new* instance of
the construction it retires: "untyped value equality" matches no member of it, flat
or hard-wrapped. So the retirement is checked a second way, with a loosened,
newline-flattening form that sees the construction rather than the phrase. Fenced
whole, because it is not expressible as a `git grep`:

```python
LOOSE = re.compile(
    r'untyped(?:[^.;)]{0,40}?)equalit|equalit(?:[^.;(]{0,40}?)untyped'
    r'|untypisiert\w*(?:[^.;)]{0,40}?)[Gg]leichheit'
    r'|[Gg]leichheit(?:[^.;(]{0,40}?)untypisiert', re.I)
# flatten comment leaders and line breaks first, so a wrapped phrase is visible:
flat = re.sub(r'\n\s*(?:///?!?|#|--)?\s*', ' ', text)
```

**It is a candidate finder, not a verdict**, and the boundary is the discriminator of
section 4.2: a hit is a defect where "untyped" modifies the noun *equality*, and
permitted where it modifies the verb *matched* and the equality is defined after it.
Every hit is read.

Run over the **ten** fenced replacement blocks of section 4.3 (R-1 to R-9 plus R-11;
R-10's and R-12's texts are the fenced Rust blocks of sections 3.2 and 5) plus those
five Rust blocks, measured after the last edit to this document: **zero strict hits**
(no replacement carries an R' member) and **two loose candidates**, both permitted -
R-3's "is matched untyped (value equality against the property named verbatim with no
type conversion..." and R-4's "and is matched untyped: value equality...", where a
bracket and a colon respectively separate the path sense from the equality. The Rust
blocks contain exactly one "untyped", section 3.2's "`raw:` is the declared untyped
path", also the path sense.

**Fired control:** run the same expression against R-2's pre-fix text ("untyped
value\n// equality against...") and it reports 1 - so a zero here is a measurement,
and this check is what caught R-2.

**A pitfall in the extractor itself, recorded because it is the same self-referential
shape and it produced a wrong figure once:** anchoring on the literal string `with
exactly` matches **eleven** places in this document, not ten - the eleventh is this
very paragraph, which *names* the phrase rather than introducing a block. Two anchors
that both give ten, stated by what they ARE rather than by where they sit: the fenced
block whose **preceding line ENDS with** `with exactly`, or the second fenced block
under each **R-n heading** in section 4.3. Do not let the instrument sweep the prose
that describes it. **No line numbers are given on purpose:** Tier-2
`a-document-never-cites-a-line-number-inside-itself` forbids a document citing its own
lines, on the owner's reasoning that an update duty is a rule requiring someone to
notice, and an earlier form of this paragraph proved him right - it listed the ten
numbers "at the time of writing" and every one of them was stale by five within the
same round, because the document grew above them. The content anchor survives every
insertion; the numbers did not.

Also: the Rust block at section 3.3 is a bare match arm, so it is not a standalone item
and `rustfmt` cannot format it in isolation - a formatter error there must not be read
as a clean result.

At execution time this check runs over the five edited product files as well, where
its expected result is zero strict and zero loose.

**A duty the vocabulary sweep does not discharge, stated because R' will go blind
after the repair:** none of the twelve replacement texts contains a member of R's
old alternation, so a future sweep for the retired phrases will return zero
whether or not a new site has invented its own wrong wording. The durable check is
the subject-first block sweep of section 4.2, and the b7/matrix tests of section 5
are what actually pin the behaviour. Do not treat a green R' as evidence about a
future site.

---

## 5. Decision 3: the test set, with the mandatory safeguard

All three tests live in `crates/muxsmith-core/src/matcher.rs`'s `tests` module.
**Verified `rustfmt`-clean** by the same scratch-copy method as section 3.2: zero
diff under `rustfmt --edition 2024`. `MatchExpr` derives `Default`
(`profile/match_expr.rs:52`), `BTreeMap` is already imported in the module, and
`MatchExpr`/`Scalar` arrive through `use super::*`.

**T-1 (safeguard, mandatory). The typed path still coerces int against float.**
Without it, a future change that strips the cross arms from `scalar_eq` passes the
whole suite. **Per the doctrine's proposed-safeguard rule this test is not argued
out during design or planning; it is removed only after being built and measured
redundant.** It uses `max_luminance` deliberately: one of the five float-declared
properties (`capability/generated.rs:47`, `:48`, `:56`, `:57`, `:58`), for which both an `Int` and a `Float`
profile scalar pass `scalar_fits` - measured through the binary, where
`min_luminance: 400` and `min_luminance: 400.0` both print `Profile is valid.`
while `audio_channels: 1.0` prints `Value for "audio_channels" has type float,
expected integer.` So the test represents a reachable production case, not an
artificial one.

```rust
    // SAFEGUARD: the TYPED `exact` path KEEPS its int/float cross-comparison.
    // `scalar_eq`'s two cross arms are correct there (spec 4.3, README: "6
    // equals 6.0") and mkvmerge's five float-typed properties need them, since
    // it reports an integral `max_luminance` as `400.0`. If this test fails,
    // someone stripped the cross arms from `scalar_eq` instead of leaving the
    // no-coercion rule to `scalar_eq_same_type` and the `raw:` path.
    #[test]
    fn typed_exact_still_cross_compares_int_and_float() {
        let reported_float = track("video", &[("max_luminance", PropValue::Float(400.0))]);
        assert!(matches(
            &expr("exact: { max_luminance: 400 }"),
            &reported_float,
            &lang()
        ));
        let reported_int = track("video", &[("max_luminance", PropValue::Int(400))]);
        assert!(matches(
            &expr("exact: { max_luminance: 400.0 }"),
            &reported_int,
            &lang()
        ));
        // Negative control: the cross arms compare values; they do not make
        // any number match any number.
        assert!(!matches(
            &expr("exact: { max_luminance: 401 }"),
            &reported_float,
            &lang()
        ));
    }
```

**A measured PRECONDITION that T-1 and T-2 both depend on, stated because it is a
precondition and not a given.** Both route their float literals through the `expr()`
helper, so `#[serde(untagged)]` decides which `Scalar` variant a literal becomes -
and `Int` is declared before `Float`. **If `400.0` resolved onto `Scalar::Int`, T-1's
second assertion and T-2's two `6.0` assertions would silently become same-kind
comparisons and would pass with the cross arms removed**, which would make T-1 a
formality instead of a safeguard. Measured through the shipped binary rather than
argued from the `Scalar` doc comment, using `Scalar::type_name()`
(`match_expr.rs:38-43`, which returns `"float"` only for `Scalar::Float` and
`"integer"` only for `Scalar::Int`) as the read-out, via a deliberate
`ValueTypeMismatch`:

| profile literal | `muxsmith validate` reports | resolves to |
|---|---|---|
| `audio_channels: 1.0` | `has type float, expected integer` | `Scalar::Float` |
| `audio_channels: 6.0` | `has type float, expected integer` | `Scalar::Float` |
| `audio_channels: 400.0` | `has type float, expected integer` | `Scalar::Float` |
| `track_name: 6` | `has type integer, expected string` | `Scalar::Int` |
| `track_name: 400` | `has type integer, expected string` | `Scalar::Int` |

So a decimal-point literal becomes `Float` and a bare integer literal becomes `Int`,
both directions of the cross pair are genuinely exercised, and T-3 remains the test
that closes the matrix independently of the parser. **The amended Step 1 re-runs these
five `validate` probes**, because they are the precondition of the two tests' claim.

**T-2. B-7 inverted, both directions, each with its same-kind counterpart** so the
test cannot pass because everything stopped matching. This replaces
`b7_raw_int_float_cross_compare` at `matcher.rs:444-449` in name, comment and body.
The `b7_` prefix stays: the case id is the case id, and its expected outcome
changed by ADR.

```rust
    // B-7 (semantics CHANGED 2026-07-30 by owner ruling, ADR D111: no type
    // casting under `raw:`): int/float cross-comparison does NOT happen on the
    // `raw:` path. Both directions are reachable from a real mkvmerge file - it
    // reports an integral `max_luminance` as `400.0` and `audio_channels` as
    // `1` - so both are pinned here, each with its same-kind counterpart so the
    // test cannot pass by matching nothing.
    #[test]
    fn b7_raw_does_not_cross_compare_int_and_float() {
        let reported_float = track("audio", &[("new_gain", PropValue::Float(6.0))]);
        assert!(!matches(
            &expr("exact: { raw:new_gain: 6 }"),
            &reported_float,
            &lang()
        ));
        assert!(matches(
            &expr("exact: { raw:new_gain: 6.0 }"),
            &reported_float,
            &lang()
        ));

        let reported_int = track("audio", &[("new_gain", PropValue::Int(6))]);
        assert!(!matches(
            &expr("exact: { raw:new_gain: 6.0 }"),
            &reported_int,
            &lang()
        ));
        assert!(matches(
            &expr("exact: { raw:new_gain: 6 }"),
            &reported_int,
            &lang()
        ));
    }
```

**T-3. The full type matrix**, because section 3.1's table is an absolute ("only
within one kind") and Tier-1
`verifying-an-absolute-enumerates-the-cases-it-quantifies-over` says an absolute is
checked by walking the cases it quantifies over, not the arm the prose names. The
`Scalar` side is constructed directly rather than parsed from YAML: the test's
subject is the comparator, and routing the four kinds through `#[serde(untagged)]`
would silently make it a parser test as well.

```rust
    // The `raw:` comparator's full type matrix: a profile value equals a
    // reported value only within one kind - no int/float coercion, no stringly
    // comparison, no bool/number equivalence. The four values are chosen so
    // that every diagonal pair is equal and every off-diagonal pair could only
    // match through a conversion, so this matrix IS the same-type rule.
    #[test]
    fn raw_compares_only_within_one_kind() {
        let wants = [
            Scalar::Bool(true),
            Scalar::Int(6),
            Scalar::Float(6.0),
            Scalar::Str("6".into()),
        ];
        let haves = [
            PropValue::Bool(true),
            PropValue::Int(6),
            PropValue::Float(6.0),
            PropValue::Str("6".into()),
        ];
        for (i, want) in wants.iter().enumerate() {
            for (j, have) in haves.iter().enumerate() {
                let t = track("audio", &[("probe", have.clone())]);
                let e = MatchExpr {
                    exact: Some(BTreeMap::from([("raw:probe".to_string(), want.clone())])),
                    ..MatchExpr::default()
                };
                assert_eq!(
                    matches(&e, &t, &lang()),
                    i == j,
                    "raw:probe {want:?} against reported {have:?}"
                );
            }
        }
    }
```

**Tests deliberately NOT added, with the premise run rather than weighed:**

- **No new integration or e2e test.** The premise is that the wiring from profile
  through identify to the matcher is unchanged and already carries the comparator's
  result end to end. That premise was RUN, not asserted: M1's four binary runs
  (two matches, two fired negative controls, through `muxsmith dry-run --json`) are
  the end-to-end evidence, and the amended task re-runs exactly those four with the
  expected outcomes inverted for the two cross cases - **after `cargo build`, since
  no exit bar rebuilds the binary those probes execute** (section 13.5's Step-1
  bullet). An automated e2e would add an mkvmerge dependency for coverage the unit
  tests already give.
- **No snapshot or fixture update.** Premise measured in M4 with a fired control:
  no snapshot, fixture or integration test asserts a numeric `raw:` comparison.
- **No validate-side test.** `raw_opt_in_diagnostic` and `scalar_fits` are
  untouched (section 3.4), and B-1..B-4 in `validate_semantics.rs` keep passing
  unchanged.

**Existing tests that keep passing unchanged:** B-5 (Int/Int, comment updated by
R-11), B-6 (absent, no shortcut), B-8 (`raw:language`), and every other matcher
test. **Task exit bars:** `cargo fmt --all --check`; `cargo clippy --workspace
--all-targets -- -D warnings`; `cargo test -p muxsmith-core` with
`b7_raw_does_not_cross_compare_int_and_float`,
`raw_compares_only_within_one_kind`,
`typed_exact_still_cross_compares_int_and_float` and
`b8_raw_language_is_byte_literal_no_normalization` named from the pasted output;
`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
--document-private-items` (R-10 adds intra-doc links `[scalar_eq]` and
`[scalar_eq_same_type]`, which is what that gate checks); `pnpm check:i18n` for the
two help edits; `cargo test --workspace` for the snapshot invariance M4 predicts.

---

## 6. Decision 4: `RawOnKnownProperty`'s scope does not change

**Decided: the trigger set stays `matches!(bare, "language" | "codec_kind")`. No
DiagCode is added, widened or re-severitied by this amendment.** The implementer
has nothing to choose here.

Rationale. The case at issue is `raw:` on a known NUMERIC property, which gets
`RawProperty` (info) only, and which this ADR gives a newly different outcome. Two
things carry the decision:

1. **The owner's ruling was made with the visible-failure argument in hand**, and
   the argument holds on the diagnostics that fire. Measured in M5 for exactly this
   case: a non-optional rule produces an error-severity `missing-track` plus a
   `raw-property` info and an `unknown-property-skew` warning, the latter two both
   naming the property. The user is told the property was matched untyped and told
   the rule matched nothing.
2. **Widening is a change to an owner-visible diagnostic surface**, which the
   doctrine's routing matrix sends to the governing human, not to a design round.
   Leaving the surface alone needs no ruling; changing it does.

**The argument FOR widening, stated at strength because it is not weak and because
two measurements strengthened it since the ruling:**

- With this mkvmerge the escape hatch has **zero sanctioned members** (M2), so
  *every* reachable `raw:` numeric comparison today is a `raw:` on a known
  property. The case that gets only an info is not a corner; it is the whole
  reachable set.
- M5 measured that the visibility is thinner than the tracker's recorded reasoning
  claims: `missing-track` renders as a bare line with `"params": {}` and the
  document's `suggestions` array is `[]` (also for a known-property control), so no
  near-miss hint and no proposed narrowing appear. And for an **optional** rule there
  is **no error severity at all** - M5 probe (B), the two-rule case: only
  `raw-property` (info) and `unknown-property-skew` (warning), both of which fire on
  any consumed `raw:` property regardless of whether the comparison succeeded, and
  `suggestions: []`. **The exit is 1, not 0** (`severity_exit` maps a warning worst to
  1), **and 1 is also what a successful `raw:` match produces**, so the exit code
  distinguishes a silently non-matching optional `raw:` rule from a working one not at
  all. That is the accurate form of the gap, and it is narrower than the word
  "silently": the human rendering does print `rule 1 -> track -`. The one-rule probe
  (A) must not be used for this claim - its zero-tracks warning is a discriminating
  signal that only exists because the plan resolved no tracks.
- The house already treats a can-never-match configuration as diagnostic-worthy
  rather than silent: `EmptyRawProperty` is an ERROR on precisely that ground, and
  `UnknownProperty` exists because "a mistyped name that would silently
  never-match is the worst failure mode for a declarative batch tool".

**Therefore, escalated as a product question with a recommendation, not
dismissed** (trigger T7): a config-time diagnostic when a `raw:` key names a
property the capability model knows and the profile value's scalar kind cannot
equal that property's declared kind under same-type equality - `raw:audio_channels:
1.0` (float value, integer property) or `raw:max_luminance: 400` (integer value,
float property, where mkvmerge reports `400.0`). It is decidable at config time
from data already in the model, it is the exact shape of the existing
never-match guards, and it turns the one genuinely new footgun into a
validate-time message. **Recommendation: build it, as its own package after this
amendment lands, at warning severity under its own code.** Not in this amendment:
it is a new user-visible surface, so it is the owner's call, and A3 must not grow a
DiagCode. **This is a routed question, not a safeguard argued away** - a later
round may not close it by agreement; it needs an owner answer.

---

## 7. Decision 5: `UnknownPropertySkew`'s text does not change

**Decided: neither Fluent value moves, and no param changes.**

The en message says the property "was matched untyped through a raw: opt-in
(bypassing the capability model)". "Untyped" there describes the PATH - the
capability model was not consulted - which stays exactly true (section 4.2's
discriminator). The message makes no claim about coercion, so the ruling does not
touch it. The warning's purpose, making the untyped match visible, is served
identically before and after; if anything it matters more, since the comparison it
announces can now fail on a kind mismatch.

**Surfaced, pre-existing, and not this amendment's to fix** (trigger T8): "was
matched untyped" reads as though a match occurred, while the planner emits the
warning before the match-count branch (`planner.rs:630`, comment: "so it fires
regardless of whether the rule resolves, matches nothing (still a MissingTrack,
B-11), or is ambiguous"). Measured in M5: the warning appears alongside
`missing-track` for a rule that matched nothing. "was compared untyped" would be
the accurate verb. That is a two-locale product-text change with its own review
cost and it predates this ruling, so it is routed rather than ridden in.

---

## 8. Decision 6: the spec's diagnostics-table row

**Decided: the row is repaired.** Exact replacement in R-6, its code-side twin in
R-7. What moves is "byte-literal untyped equality"; what stays is the row's subject
(the degradation of `language` normalization and `codec_kind` aliasing) and its
severity, its trigger and its section references.

Reason: after R-3 and R-4 the spec's own authoritative statement is that the `raw:`
comparison requires the kinds to agree. A table row in the same document that calls
the resulting comparison "untyped equality" then asserts the opposite of what the
document just settled - a self-contradiction the doctrine requires a spec amendment
to sweep for, and the one phrase in the retain set that a reader cannot reconcile.
It is also the narrowest possible correctness-grounded cut: "untyped equality" is
the phrase the greppable expression in section 4.2 isolates, and it isolates exactly
two lines tree-wide.

The counter-position, recorded so the fold-in does not re-open it: the row is
scoped to two string-typed properties, so nothing in it is false, and repairing a
true sentence is churn (this is Plan 11's position, and it was right for the wording
the plan was going to ship). It loses because the amendment changes what "untyped"
can be attached to, not because the row was ever wrong.

---

## 9. Rejected alternatives, each with its steelman

**A-1. Compare both sides as their retained SOURCE TEXT.** Rejected by the owner,
2026-07-30. **Steelman at strength:** it is the only semantics under which the word
"raw" is fully honest. It makes `6.0` differ from `6.00`, and it takes no type
decision at all - deciding that an unknown property's value *is* a number is itself
an assumption, and this is literally the "no help whatsoever" the ruling asks for.
It removes the last inference from the escape hatch. **Rejected after the cost was
named:** it needs the `raw:` value to keep its YAML source form instead of being
typed, and the reported side to keep mkvmerge's JSON number token, which
`PropValue::from_json` discards - so it reaches into the identify layer, not the
matcher. And it needs its own rule about whether a JSON string's quotes are part of
the literal, which relocates the type question rather than removing it. Not to be
re-proposed. Note that its motivating case is unreachable from mkvmerge anyway
(F3: `6.00` in, `6.0` out) - measured here, where the tracker had it marked as
reasoning.

**A-2. Strip the cross arms from `scalar_eq` itself.** **Steelman:** the smallest
possible diff, one function, no new name, and it makes "no coercion" true
everywhere at once, which is superficially what "no type casting happens under
`raw:`" sounds like. **Rejected:** it breaks documented, intended behaviour of the
typed path - `scalar_eq`'s own doc scopes the cross arms to `exact` (spec 4.3),
README `:52` promises "numbers numerically (`6` equals `6.0`)", Tier-2
`core-72-exact-typed-value-equality` records it as core semantics, and mkvmerge's
five float-typed properties need it because it reports an integral `max_luminance`
as `400.0`. T-1 exists to make this failure loud.

**A-3. A `coerce_numeric: bool` parameter on `scalar_eq`.** **Steelman:** no second
function, no duplicated arm list, and the difference between the paths is visible
in the signature. **Rejected:** a bare boolean at a call site is unreadable without
opening the callee, and the third call site (the Boolean false-when-absent
shortcut) would have to pass a value that means nothing for it. Section 3.2's
layering gets the same single arm list with self-describing call sites.

**A-4. An exhaustive `match` with no `_ => false`, so a new `Scalar`/`PropValue`
kind breaks the build.** **Steelman:** the wildcard is exactly how a future
parallel-enum drift stays silent, and compile-time exhaustiveness is strictly
stronger than the doc comment's instruction in section 3.2. **Rejected for this
amendment:** it means enumerating all sixteen pairs in code for a change whose
scope is one arm plus wording, and `_ => false` is the deliberate existing style of
this function. The drift risk is mitigated by there now being one enumeration
instead of two. Worth its own item if a third comparator ever appears.

**A-5. Retain all nine scoped sites (Plan 11's split, carried over unchanged).**
**Steelman:** every one of them is true as written - M3 establishes that only a
string can reach the comparison for `language`, and `raw:codec_kind` cannot match
at all - so the change is pure churn, it touches two user-visible Fluent strings in
two locales, and Plan 11 argued the specialization reading carefully. **Rejected
for two of the nine only,** on the greppable "untyped equality" boundary of section
4.2; the other seven are retained on exactly this argument.

**A-6. Repair all nine, including both Fluent strings and the b8 test name.**
**Steelman:** one mechanism, one name, everywhere. The retired word is retired
because it means two things at once, and that ambiguity is not smaller in a scoped
sentence; it is merely harder for a reader to notice, which is how the original
defect survived four documents. **Rejected:** it makes a bilingual product-text
change ride a semantics fix, it renames a test that two house-knowledge entries
cite by name, and for a string comparison the word is not ambiguous but precise
(section 3.1). The boundary in section 4.2 keeps the correctness-grounded half and
drops the taste-grounded half.

**A-7. Ship Plan 11's accurate-for-today wording and amend the same sentences
later.** Overruled by the owner, 2026-07-30: Plan 11 is adjusted rather than
shipping wording that documents a behaviour about to be removed. Recorded because
it was the controller's recommendation and was reversed.

**A-8. Widen `RawOnKnownProperty` to every known property.** Steelman and
disposition in section 6.

---

## 10. Parity: SI-3, and the reference tool has a partial model

SI-3 and `proc-06-mkvtoolnix-parity` bind every behavioural question. The brief's
expected outcome was that there is no parity model at all. **That is refuted:
mkvtoolnix has no profile concept, but it does have declarative value-based track
selection in two places, and both compare strictly within a type.** The finding
outranks the brief's paragraph, and it supports the ruling in both directions.

**Source, read at `~/Downloads/mkvtoolnix`.**

*mkvmerge CLI, `parse_arg_tracks` (`src/merge/mkvmerge.cpp:603-627`).* The
`-a/-d/-s/-b/--track-tags` arguments take a comma list whose elements are either a
track ID or a language, and the discrimination is by the literal's own form:

```cpp
    int64_t tid;
    if (mtx::string::parse_number(element, tid)) {
      tracks.add(tid);
      continue;
    }
    auto language = parse_language(element, fmt::format("{0} {1}", opt, s));
    tracks.add(language);
```

The two go into separate typed buckets. `item_selector_c<T>`
(`src/merge/item_selector.h:27-124`) keeps `std::unordered_map<int64_t, T> m_items`
and `std::unordered_map<mtx::bcp47::language_c, T> m_language_items`, and
`selected()` (`:52-66`) tests the numeric item against the numeric map and the
language against the language map. **They never cross.** The language side is
normalized (`best_language_match` -> `language.find_best_match`).

*mkvtoolnix-gui, `Track::setDefaultsMuxThis` (`src/mkvtoolnix-gui/merge/track.cpp:198-223`).*
The GUI auto-selects tracks by two reported properties from its preferences:
`m_enableMuxingTracksByTheseTypes.contains(m_type)` (an enum over a closed domain)
and `m_enableMuxingTracksByTheseLanguages.contains(Q(language.get_iso639_alpha_3_code()))`
(a string list, compared against the track's language canonicalized to its ISO
639-2 alpha-3 code first). Typed, domain-normalized, and the property set is closed
at two compile-time-typed properties.

**Confirmed by running the binary** (`mkvmerge v100.0`), against a probe muxed from
the repo's own `tone.wav` seed with `--language 0:ger`, so it reports
`language: "ger"`, `language_ietf: "de"`, `audio_channels: 1`, at
`identification_format_version: 20`:

| invocation | audio tracks in output | what it shows |
|---|---|---|
| `mkvmerge -o out.mkv -a 0 probe.mkv` | 1 | numeric literal matched against the track ID |
| `mkvmerge -o out.mkv -a 1 probe.mkv` | 0 | no track ID 1; the numeric literal is never tried against the language |
| `mkvmerge -o out.mkv -a ger probe.mkv` | 1 | language literal matched against the reported language |
| `mkvmerge -o out.mkv -a eng probe.mkv` | 0 | discriminating; not a match-everything selector |
| `mkvmerge -o out.mkv -a de probe.mkv` | 1 | language normalization: ISO 639-1 matches a track tagged `ger` |
| `mkvmerge -o out.mkv -a deu probe.mkv` | 1 | 639-2/T also normalizes |
| `mkvmerge -o out.mkv -a de-DE probe.mkv` | 0 | meaningful distinctions preserved |

**Parity reading.** The reference tool's value matching is same-type with no
coercion between its two selector domains, and domain-aware within the language
domain. That corroborates both halves of the design: the typed `exact` path's
canonicalization and the `raw:` path's refusal to convert. What mkvtoolnix has no
counterpart for is an *escape hatch* - a mechanism that matches an arbitrary
reported property by name. Measured:

```
$ grep -riE '\b(preset|presets|profile|profiles)\b' src -l | wc -l
28
$ grep -riE '\b(preset|presets|profile|profiles)\b' src -o | tr A-Z a-z | sort | uniq -c
    117 profile
      1 profiles
```

All 118 are codec profiles (AAC/H.264/VP9); zero occurrences of "preset". **Fired
control** for the GUI-scoped variant that returned zero: `grep -riE '\bmkvmerge\b'
src/mkvtoolnix-gui --include="*.cpp" --include="*.h" -l` returns 16 files, and the
tree there holds 152 `.h`, 144 `.cpp`, 42 `.ui`, so the instrument reaches the
sources it appeared to skip. So the escape hatch's semantics is Muxsmith's own
product decision with no upstream precedent to follow, and the narrower question -
how a value literal is matched against a reported property - **does** have one, and
this ADR follows it.

---

## 11. Interface consequences

**No serialized or user-visible surface moves.** Enumerated so the claim is
checkable rather than asserted: no `DiagCode` added, removed, renamed or
re-severitied; no Fluent key added or renamed, and no message value changed at all -
both `raw-on-known-property` values and both `unknown-property-skew` values are
retained (sections 4.4 and 7), so `locales/` is not in the Files list and
`git diff --exit-code -- locales/` holds; no
`--json` report field, param name or document shape changed; no profile schema,
`Scalar`, `PropValue` or `MatchExpr` change, so the generated TS bindings and
`profile.ts` are untouched; no CLI flag, exit code or IPC command changed; no gate
part, dependency or CI job added.

**One BEHAVIOUR change, and it is user-visible in outcome:** a profile whose
`exact` map carries a `raw:` key with a numeric literal stops matching a reported
number of the other kind. Reachability, measured: only through `raw:` on a KNOWN
property, because the capability model knows all 59 schema-v20 track properties
(M2) - which is the diagnosed-but-legal route, announced by `RawProperty` (info)
and `UnknownPropertySkew` (warning). Pre-1.0, no released profile can depend on it
through the sanctioned route, and the change gets monotonically more expensive as
the pinned schema grows.

**One private API addition inside `muxsmith-core`:** `scalar_eq_same_type`, a
private function in `matcher.rs`. Not `pub`, so it is not part of the crate's
surface; it does appear in `--document-private-items` rustdoc, which is why R-10's
intra-doc links must resolve.

---

## 12. Triggers created, for the controller to mirror into the ROADMAP

This document writes no tracker. Each item below is stated so the controller can
route it without re-deriving it.

| # | trigger | why |
|---|---|---|
| T1 | Tier-1 `core-91-raw-opt-in` (`docs/decision-ledger.yaml:521`) ends "Shipped exactly per the binding B-1..B-11 acceptance table". B-7's expected outcome inverts, so the entry's transitive claim goes false. | the entry names a table this ADR partly supersedes |
| T2 | Tier-1 `core-98-raw-language-single-field` (`:558`) reads "raw:X reads exactly the property literally named X, byte-exact". Its substance (single-field name lookup) is unchanged; the phrase invites the reading this ADR retires. | harmonization candidate, controller's file |
| T3 | Tier-1 `core-97-raw-on-known-property` (`:546`) states the degradation "to byte-literal equality". Substance unchanged; same vocabulary question as R-6/R-7 resolve in the spec and the DiagCode doc. | harmonization candidate |
| T4 | Tier-2 `core-72-exact-typed-value-equality` (`docs/conventions.yaml:331`) is now guarded by a named test. | an occurrence recording that `typed_exact_still_cross_compares_int_and_float` is its check |
| T5 | D32's matcher case table row **B-7**, at `docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md:74` (measured: `grep -n "B-7"` returns `74:`; the row reads `\| B-7 \| { new_gain: Float(6.0) } \| exact: { raw:new_gain: 6 } \| yes \| int/float cross-compare \|`), needs a `superseded by D111` link, not an edit. Cited by content as well as by line, since the line moves if that document is ever reflowed. | append-only ADR record |
| T6 | The ROADMAP entry beginning `"byte-exact" overstates` carries two claims this document measured differently (section 14, P3 and P4), and its own defect description uses the retired phrase (which is why it is R''s soundness control). **Plus a correction, not only a mirroring duty:** the controller has already mirrored this document's first-draft "exit 0 / fails silently" gloss into the ROADMAP. Measured: exit **1** in the optional case and **1** for a successful `raw:` match too, so the exit discriminates nothing; and the single-rule probe behind the gloss carries a zero-tracks warning that contradicts it. The accurate form is in section 6 and section 14 P4. | the entry's disposition is implemented here; the relayed figure is wrong |
| T7 | **Open owner question**, section 6: a config-time diagnostic for a `raw:` key whose scalar kind can never equal a KNOWN property's declared kind. Recommendation: build it as its own package, warning severity, own code. | new user-visible surface, owner's call; routed, not closed |
| T8 | Pre-existing: `unknown-property-skew`'s "was matched untyped" fires even when the rule matched nothing (`planner.rs:630`; measured in M5). "was compared untyped" is accurate. Two locales. | routed, section 7 |
| T9 | Pre-existing: `missing-track` renders bare with `"params": {}` and `suggestions` is `[]`, for a `raw:` non-match and for a known-property control alike (M5). | weakens a stated visibility argument; worth its own look |
| T10 | Pre-existing, re-verified: `codec_kind` is absent from schema v20's 59 track properties, so `raw:codec_kind` can never match while `RawOnKnownProperty` still warns about it. | already surfaced by A3 Step 8; confirmed |
| T11 | Pre-existing: spec `:146` opens "typed value-equality, not raw string equality", which collides verbally with the `raw:` feature name one section later. Repairing it ripples into `core-72`'s statement and the plan-3.5 principle record, both controller-owned. | readability, routed |
| T12 | Any future sweep of the `raw:` claim surface must use the RULE in section 4.1, not an enumerated file list. **Measured, not cautionary:** the plan's own check-R pathspec excludes only the 07-11/07-21/07-28 specs, so once this document is committed that check returns **16 lines across 6 files instead of 6**, ten of them inside this document (`git grep --untracked` reproduces it today; plain `git grep` does not, because untracked files are skipped). | the enumeration went stale on its first addition, and the staleness is executable |

---

## 13. What this amendment overrides in Plan 11

Named explicitly, because a new rule that voids a neighbouring practice has to say
what replaces it, and the plan author folding this in must remove the superseded
clauses rather than layer them.

**The scope unit is the set of ASSERTIONS, not the task body.** An earlier form of
this section listed Task A3's executable clauses and nothing else, which was the same
defect section 4.1 exists to prevent one level up: a file-and-body list where the
thing being changed is a fact stated in many places. Section 4.1 derives the wording
surface from a rule and T12 gives the reason; the same treatment is owed here, because
the plan author consumes this list and an omission leaves a contradicted clause
standing inside an owner-approved plan.

### 13.1 The rule, its expression, and the set it derives

**Rule: every sentence in the plan document that states the A3 site count, the file
count, the retained-set size, the `scalar_eq` shape, that no behaviour changes, that
no test ships, or that names in IDENTIFIER form a symbol this amendment renames or
reshapes.** Derived over the whole plan document, not over Task A3's section, in four
parts unioned and every member read:

```bash
P=docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md
# E1 - names the task or its work item
grep -nE '\bA3\b|\bW3\b' "$P"
# E2 - asserts the sets or carries the retired vocabulary
grep -niE 'repair set|retained set|retained (assertions|sites|scoped)|byte-literal|byte-exact|byte-for-byte|byte-genaue|unscoped-claim|raw:' "$P"
# E3 - asserts a changed FACT in words, whether or not it names A3
grep -niE 'behaviour is unchanged|behaviour stays|no new test|nine|seven files|five files|six (sites|lines|sentences|repair)|scalar_eq' "$P"
# E4 - names a renamed or reshaped symbol in IDENTIFIER form (underscores, no colon)
grep -nE 'b7_raw|b8_raw|byte_literal|scalar_eq_same_type' "$P"
```

Measured at this tree: E1 **39** lines, E2 **69**, E3 **41**, E4 **7**, union **103**
lines. Of those, **28 are VOID** (5 inside Task A3 beyond the clause list of 13.5, 23
at plan level) and **75 are unaffected**.

**E4 exists because a prose vocabulary is blind to an identifier by construction, and
that blindness cost a member.** E2's terms are `raw:` (needs the colon) and
`byte-literal` (needs the hyphen); a Rust test name carries `raw_` and `byte_literal`
with underscores and no colon, so no term in E1, E2 or E3 can reach a line whose only
reference to the amendment is a test name. **Measured: E1 to E3 return 102 lines and
E4 adds exactly one, plan line 626** (Step 7's gate-parts bullet, which requires
`cargo test` green with `b7_raw_int_float_cross_compare` named from the output - a name
R-12 removes). E4's other six members were already covered. This is the same blindness
section 4.6 records for `matcher.rs:457`, where the b8 test's name carries
`byte_literal` in identifier form and neither R' nor K' can match it.

**E4's own alternation is derived from the artifacts, not recalled**, because a
pattern's enumerated set is itself a claim. The plan carries exactly five identifiers
in this family, by count: `scalar_eq` (10 occurrences), `b7_raw_int_float_cross_compare`
(7), `exact_matches` (5), `b8_raw_language_is_byte_literal_no_normalization` (3),
`raw_opt_in_diagnostic` (2). Of those the amendment **renames one**
(`b7_raw_int_float_cross_compare`, by R-12) and **reshapes one** (`scalar_eq`, already
an E3 term); `exact_matches` and `raw_opt_in_diagnostic` are untouched (section 3.4).
`b8_raw` and `byte_literal` name symbols the amendment deliberately RETAINS and are
included anyway, at zero cost: they add no line E4 does not already reach, and they
keep the instrument sound if a later revision renames `b8`. `scalar_eq_same_type` is
absent from the plan today and is included for the same forward reason.

**E3 is not decoration either, and its own blind-spot probe is what proved it.** E1 and
E2 alone return 87 lines and miss 15 that E3 catches, two of them VOID: plan line
**116**, the authoring heading "six sites to repair, nine to leave" (it names neither
A3 nor the vocabulary, and says "sites to repair" rather than "repair set"), and plan
line **986**, "item 3's assertion set is fifteen lines split six and nine". A
token-based union over the task's name and the retired vocabulary is exactly the
instrument that cannot see a sentence stating the fact in its own words - the same
class as the four wording sites of section 4.5 that neither plan alternation could see.
**Both widenings arrived the same way**, one round apart: a member was found by hand,
and the repair was the expression rather than the member.

### 13.2 VOID inside Task A3, beyond its executable clauses

| plan line | assertion | replaced by |
|---|---|---|
| 456 | task title: "`raw:` compares untyped, not byte-exactly, and six sentences learn to say so" | twelve repair sites plus a behaviour change; and "untyped" as the name of the comparison is the construction R-2 stops using |
| 458 | Read first: the ROADMAP entry "including its recorded disposition that **the behaviour stays and only the wording changes**"; `matcher.rs` "for `scalar_eq`'s six arms"; and the two tests "`b7_raw_int_float_cross_compare` and `b8_...`" | the ruling inverts that disposition; `scalar_eq` becomes a pair of functions; `b7` is renamed by R-12 |
| 476 | Step 1: "run `cargo test -p muxsmith-core matcher` naming `b7_raw_int_float_cross_compare` from the output" | that name no longer exists; section 5's three test names replace it |
| 626 | Step 7's gate-parts bullet: `cargo test -p muxsmith-core` green "with `b7_raw_int_float_cross_compare` and `b8_raw_language_is_byte_literal_no_normalization` named from the pasted output" | **section 5's "Task exit bars" is the replacement**, which names `b7_raw_does_not_cross_compare_int_and_float`, `raw_compares_only_within_one_kind`, `typed_exact_still_cross_compares_int_and_float` and `b8_raw_language_is_byte_literal_no_normalization`, plus `cargo test --workspace`, `cargo doc` and the rebuild of 13.5's Step-1 bullet. Without this row the plan would carry two contradictory gate-parts instructions and the stale one is what an implementer reads at the gate. **Derived by E4, not added by hand** |
| 639 | Step 9's commit message: "raw: is untyped value equality, not byte-exact - numbers still compare numerically" | states the inverted wording AND uses the retired construction; the plan author fences the new message |

### 13.3 VOID at plan level, 23 assertions

| plan line | assertion | measured truth after the amendment |
|---|---|---|
| 61 | model-tier row: "six sites across five files ... a nine-member retained set that must be proven unchanged" | 12 sites, six files, seven-member retained set |
| 116 | authoring heading "six sites to repair, nine to leave" | 12 / 7 |
| 118 | "**`scalar_eq` has SIX arms and two of them coerce**", with the six arms pasted | `scalar_eq` becomes `scalar_eq_same_type` (four arms) plus two cross arms; A3's Read-first designates this block as ground truth, so a task reads it |
| 119 | "The behaviour is therefore settled and covered; only the wording is wrong ... it is why Task A3 owes no new test" | the behaviour changes; three tests ship |
| 122 | "The REPAIR set is exactly six lines across five files" | R' returns 8 lines across 6 files |
| 123 | "The RETAINED set is nine lines across seven files" | K' returns 7 lines across 6 files |
| 124, final clause | "4.4 and 9.2 **will state that strings compare byte-for-byte while numbers compare numerically**" | they will state the opposite. The rest of 124 (the measured trigger set, the corrected ground) survives |
| 166 | corrections row 2: "The set of assertions is 15 lines, split 6 to repair and 9 to leave" | 12 / 7 |
| 180 | coverage map W3: "six repair sites across five files, with a nine-member retained set proven unchanged" | 12 / 7 |
| 200 | "A3 spans five files in two natural languages" | six files |
| 233 | W3-a: "The v1 spec's two unscoped `raw:` statements state the numeric behaviour" | four spec sites (R-3, R-4, R-5, R-6), and the inverse numeric behaviour |
| 234 | W3-b: the fenced comment "naming the cross arms and case B-7" | R-2 names `scalar_eq_same_type` and states that no cross-comparison happens |
| 238 | W3-f: "RED ... exactly 6 lines across 5 files. GREEN: 0" | 8 across 6, GREEN 0 |
| 239 | W3-g: "exactly 9 lines across 7 files on BOTH the pre-state and the end state" | 7 across 6 on both |
| 240 | W3-h: "**The behaviour is unchanged**", with `b7_raw_int_float_cross_compare` named from the output | the behaviour changes; that test is renamed and inverted |
| 866 | plan-close ROADMAP disposition "recording the derived set as six repaired and nine retained" | 12 / 7 |
| 867, item (3) | surfacing item "the nine retained `byte-literal` assertions ... two of them user-visible Fluent strings in two locales" | seven; and the two Fluent strings are precisely the ones that STAY, while two others move under R-6/R-7 |
| 878 | deferral row "The nine retained `byte-literal` assertions keep their wording" | seven keep it |
| 946 | coverage tally "**37** acceptance halves ... **W3=10**" | W3 gains the behaviour change and three tests; no row names them yet |
| 948 | latitude paragraph "the six repair sites and nine retained sites of Task A3" | 12 / 7 |
| 950 | absence-check enumeration: "A3 has one absence check (the repair expression, **red 6**) and one invariance check (the retention expression, **9 on both states**) whose fire is a deliberate deletion that **must move the count to 8**" | red 8; 7 on both states; the fire moves 7 to 6. Plus R'' (section 4.6), a third check A3 did not have |
| 954 | "Counts recomputed from their own enumerations": "**6 repair sites and 9 retained sites in A3 (across 5 and 7 files)**" and "**6 arms of `scalar_eq` of which 2 coerce**" | 12 / 7 across 6 and 6 files; and the arm count moves with section 3.2 |
| 986 | brief refutations: "item 3's assertion set is **fifteen lines split six and nine**" | 12 / 7 |

**Two of these are the sharp ones,** and they are why this is a set rather than a
tidy-up: line 240 is an ACCEPTANCE observable asserting "the behaviour is unchanged",
and line 119 is the AUTHORING ground on which "A3 owes no new test" rests. A fold-in
that leaves them standing ships a plan whose acceptance map contradicts its own task.

### 13.4 Named as UNAFFECTED, so the rule is not over-applied

These are inside the union and must NOT be edited. Listing them is part of the
handoff, because a plan author applying 13.3 mechanically would sweep them.

- **Historical or dated records**, protected by the plan's own Global Constraint that
  a dated measurement is not falsified to today: line **934** (the self-review's NEW-2
  entry, "The model-tier table said 'a seven-member retained set' where eleven other
  sites say nine ... It says nine now"). It is a true record of a round-1 fix, and the
  amendment moving the count back to seven for an unrelated reason does not falsify
  it. **Do not renumber it.** Same for lines 169, 901, 903, 916, 934, 966.
- **Measurements that survive as pre-state facts:** line **120** (both coercion
  directions reachable, demonstrated end to end) is exactly what M1 re-measured and
  stays true of the pre-state; only the conclusion it fed (the numeric wording) moves.
  Line **905** (why the retained sites are true) stays true of the seven.
- **Claims the amendment confirms:** lines **125** (the two different-claim sites),
  **126** (spec `:421` in both sets, first occurrence repaired and second retained),
  **880** (`raw:codec_kind` can never match), **156**/**157** (`check:i18n` gates the
  two help edits; no `rustfmt.toml`), **235**-**237** (W3-c/d/e's observables, which
  are "agrees" and survive the replacement text changing), **241**/**242** (the README
  example), **900** (A3 gained a step).
- **Other tasks' test-duty paragraphs**, lines **344** (A1), **439** (A2), **722**
  (A4), **842** (B1): each grounds "no new test" on its own task producing no
  user-visible consequence, which stays true. Only A3's, line 628, is void.
- **Other tasks' expressions and counts**, lines 147, 150, 203, 204, 705, 972.

### 13.5 The executable clauses of Task A3, as before

- **Files (EXHAUSTIVE) becomes six:** the existing five (v1 spec, `matcher.rs`,
  `README.md`, both help topics) plus `crates/muxsmith-core/src/report/mod.rs`.
  `locales/*`, `validate.rs` and `validate_semantics.rs` are NOT added - their
  sites are retained (section 4.4). `matcher.rs` is now modified in **code**, not
  comment-only.
- **"Nine assertions in seven files are deliberately NOT edited"** is replaced by
  section 4.4's seven-site table plus the two different-claim sites.
- **Step 2 (a) and (b)** are replaced by R-3 and R-4, and Step 2 gains R-5 and R-6.
  Step 2's sentence "Section 7's diagnostics-table row for `RawOnKnownProperty` is
  not touched either" is **void**: it is R-6.
- **Step 3** is replaced by R-2 plus the code changes of section 3.2 and 3.3, and
  R-10 and R-11. Its clauses "Not one line of code changes in this file" and the
  comment-only constraint are **void**.
- **Step 4** is replaced by R-1. Its replacement text contained "so `6` still
  matches a reported `6.0`", which the ruling inverts.
- **Step 5 (a) and (b)** are replaced by R-8 and R-9. Same inversion.
- **Step 7's checks R and K** are replaced by section 4.6's R' and K' (8 and 7 lines
  on the pre-state, 0 and 7 on the end state). Its vocabulary-sweep figure of
  **100** stays valid for its own pathspec and becomes **71** under section 4.1's
  rule, reconciled line for line in section 4.5.
- **Step 7's diff-scope check** is **void** in two clauses: `git diff --stat` now
  names six files, and "every changed line must be a comment line, and none may be
  inside the `tests` module" is false in both halves.
- **Step 7's test-duty paragraph** ("this task ships no new test ... a new test
  would duplicate a passing one") is **void**: three tests ship (section 5), and
  `b7_raw_int_float_cross_compare` is the assertion being inverted rather than the
  coverage being relied on.
- **"Must not decide"** loses three clauses (the six/nine split and the rule that
  splits them; that no behaviour changes; that no new test is written) and keeps
  the rest. It gains: the semantics table of section 3.1, the comparator's name and
  location, every fenced replacement in both natural languages, that
  `RawOnKnownProperty`'s trigger set does not change, that no DiagCode or Fluent
  key is added, and that the two `raw-on-known-property` Fluent values are not
  edited.
- **Step 1** gains the two-direction SI-3 reproduction of M1 (four runs, two
  matching and two fired negative controls) with the **post-change expectation
  inverted for the two cross cases**, the five `validate` probes that establish the
  parser precondition of T-1 and T-2 (section 5), and the parity runs of section 10.
  **It also gains a rebuild, and this is not ceremony:** the four post-change probes
  are the only end-to-end proof of the behaviour change and they execute
  `target/debug/muxsmith`, which **none** of the named exit bars rebuilds -
  `cargo test -p muxsmith-core`, `cargo clippy --workspace --all-targets`,
  `cargo fmt --all --check`, `cargo doc` and `pnpm check:i18n` all leave that binary
  untouched, so the four probes would otherwise be run against the pre-change build
  and would show the OLD behaviour while appearing to confirm the new one. Required
  before the post-change runs: `cargo build -p muxsmith-cli`, then the freshness
  check M1 itself used, `find crates src-tauri -name '*.rs' -newer
  target/debug/muxsmith`, which must return nothing.
- **Untouched:** Step 6 (the README example's `pattern`, owner-ruled, corpus delta
  zero), Step 8 (surface-do-not-edit), Step 9's commit shape apart from its file
  list and its message (which is void per 13.2; the plan author fences the
  replacement), and A3's position in the A1 -> A2 -> A3 -> A4 chain.
- **A measurement that reveals a real ordering or coercion defect rather than a
  wording defect remains NEEDS_CONTEXT.** Unchanged.

---

## 14. Brief premises refuted, with the measurement that refuted them

- **P1. "`PropValue::from` tries `as_i64()` then falls back to `as_f64()`."** The
  function is **`PropValue::from_json`** (`crates/muxsmith-core/src/identify.rs:34`).
  The mechanism is exactly as described; only the name was wrong. Consequence: a
  brief-quoting replacement text or comment would have cited a function that does
  not exist.
- **P2. "The honest expected outcome here is that there is no parity model."**
  Refuted. mkvmerge's `parse_arg_tracks` (`src/merge/mkvmerge.cpp:603`) plus
  `item_selector_c` (`src/merge/item_selector.h:27-124`) and the GUI's
  `Track::setDefaultsMuxThis` (`src/mkvtoolnix-gui/merge/track.cpp:198-223`) are
  declarative value-based track selection with strict same-type separation and
  language normalization, confirmed by seven binary runs (section 10). What is
  genuinely absent is a profile/preset concept (118 "profile" hits in `src`, all
  codec profiles; zero "preset"), so there is no precedent for an escape hatch -
  but there is one for the comparison rule, and it agrees with the ruling.
- **P3. The ROADMAP's supporting reasoning that "`6.0` versus `6.00` is probably
  unreachable from mkvmerge's side, because JSON writers emit the shortest
  round-tripping form", explicitly marked there as reasoning and not measured.**
  Now measured, and the conclusion holds while the stated reason does not:
  `--max-luminance 0:6.00` is reported as `6.0` and `--min-luminance 0:400.500` as
  `400.5`, so mkvmerge does canonicalize and `6.00` never arrives. But it does
  **not** emit the shortest round-tripping form - it writes `400.0` and `6.0` where
  `400` and `6` would be shorter and would round-trip. Its rule is "a double is
  written with a fractional part". That correction is load-bearing rather than
  pedantic: mkvmerge's decimal-point habit is exactly what makes the
  `(Scalar::Int, PropValue::Float)` direction reachable from an ordinary file (M1
  row 2), which is why T-2 pins both directions instead of only the one the old
  B-7 covered.
- **P4. The ROADMAP's recorded reasoning that under byte-exactness "a non-match
  makes the rule match zero tracks, which the suggestion engine reports with a
  proposed narrowing, so the failure is visible and repairable".** Partly refuted
  (M5). The failure is visible - `missing-track` at error severity plus two notices
  naming the property - but the suggestion engine produced nothing: `suggestions` is
  `[]` and `missing-track` carries `"params": {}`, for a `raw:` non-match and for a
  known-property control alike. And for an **optional** rule there is no error
  severity at all, only an info and a warning that fire for any `raw:` use. The
  ruling stands on its other grounds, but section 6's decision is written on the
  diagnostics that actually fire, and this is why section 6 escalates a guard
  instead of resting on the narrowing.

  **A claim of this document's own first draft, refuted by its own re-measurement
  and recorded rather than quietly repaired:** that draft said the optional case
  produces "exit 0". Measured with `$?`: **exit 1**, because `severity_exit`
  (`crates/muxsmith-cli/src/commands/mod.rs:25`) maps a warning-severity worst to 1
  and `unknown-property-skew` is a warning. The draft also carried the claim on a
  single-rule probe whose own pasted output contained a third diagnostic
  (`This plan resolves to zero output tracks`) that fires *because* the rule matched
  nothing - so the probe contradicted the claim it was cited for. The two-rule probe
  (M5 (B)) is the one that supports it. What survives is narrower and sharper: no
  error severity, no suggestion, no narrowing, the skew warning firing regardless of
  outcome, and an exit code of 1 that a successful `raw:` match produces too. The
  word "silently" does not survive at all. **The controller had already relayed the
  wrong figure into `docs/ROADMAP.md` and to the owner; correcting that record is
  the controller's, not this document's** - T6 carries the duty.
- **P5. The brief's framing that the nine string-scoped sites concern `language`
  and `codec_kind`.** Accurate, and re-measured: `raw_opt_in_diagnostic`'s trigger
  is `matches!(bare, "language" | "codec_kind")` (`validate.rs:415`), schema v20
  types both `language` and `language_ietf` as `string`, and `codec_kind` is absent
  from the 59. What the framing does not carry, and what section 4.5 adds, is that
  the nine are **two facts, not one**: seven assertions of the
  `RawOnKnownProperty` statement and two of the separate `raw:language`
  single-field statement. The split matters because the first fact's wording is
  what this ADR decides and the second is untouched.
- **P6. Plan 11's authoring measurements, re-run for comparison rather than
  refuted.** Check R returns 6 lines / 5 files, check K returns 9 lines / 7 files,
  and the fenced `byte` sweep returns 100, all exactly as the plan states. The
  divergence in section 4.5 is a consequence of the ruling and of a broader surface
  rule, not of an authoring error.
