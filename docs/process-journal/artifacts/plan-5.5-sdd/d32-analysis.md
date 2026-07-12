# D32 analysis: UnknownPropertySkew mitigation

Task 15 design round, Plan 5.5. No code. Analysis feeds Şenol's decision; the
`D32` memo section in `docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md`
is written after he decides. Task 16 implements the decided shape and amends spec §9.2.

## Problem

Spec §9.2 promises that when the local mkvmerge reports a newer identification
schema than the build pinned, properties unknown to the built-in model become
matchable as untyped values with an `UnknownPropertySkew` warning, "instead of
failing". But `validate.rs` hard-rejects any unknown property name at config
time, before planning ever runs. The promised forward-compat path is therefore
unreachable: no profile referencing a newer-schema property survives validation
to reach the runtime where the skew would be detected and downgraded.

The hard-reject is not a bug in isolation. It exists on purpose: for a
declarative batch tool, a typo'd property name that silently never matches is
the worst failure mode. `exact: { langauge: de }` (transposed) must not quietly
resolve to "no track has property `langauge`" and drop a track. So the design
tension is real: typo protection (config-time hard-reject) versus forward
compatibility (runtime untyped matching). D32 decides how to hold both.

The information-theoretic core of the problem: **at config time Muxsmith cannot
tell a typo from a genuine newer-schema property.** `validate` (level 1) touches
no filesystem and never invokes mkvmerge (spec 5.5), so it has neither the local
`identification_format_version` nor the `-J` track output. The only signal that
distinguishes "typo" from "real new property" is runtime (`-J` reports a newer
`format_version` and the property is actually present on a track), and that
signal exists only at plan time. Any mitigation must decide *what stands in for
that missing signal at config time*: nothing (accept all, shape C), a static
in-profile marker (shape B), or "keep rejecting, just explain better" (shape A).

## Spec §9.2 exact promise

Section 9, item 2 (the "9.2" the plan cites), verbatim:

> 2. **Runtime**: the local mkvmerge is queried for version, supported file types and languages. `mkvmerge -J` output carries `identification_format_version`; if it is newer than the pinned one, unknown track properties become matchable as untyped values with an `UnknownPropertySkew` warning instead of failing (forward compatibility without lying about type safety).

And the diagnostic table (5.2) row it references:

> | `UnknownPropertySkew` | warning | property unknown to the built-in model but present in a newer identification schema version (9.2) |

Two load-bearing phrases: "become matchable as untyped values" (the mechanism)
and "without lying about type safety" (the constraint: untyped is honest,
silent-wrong is not). Whatever D32 decides, Task 16 amends this text to match.

## Current code reality

The blocker is exactly one call path; the runtime machinery it feeds is already
built.

- **The hard-reject.** `validate.rs:224` (`match prop_type(prop) { None => diags.push(unknown_property(&p, prop)) ... }`) inside `validate_expr`, emitting via `unknown_property` at `validate.rs:344-346` (`DiagCode::UnknownProperty`, error severity). The same `None` arm fires for `substring`/`regex` at `validate.rs:277`. `prop_type` resolves through `capability::matchable_type` (`capability/mod.rs:36`), a lookup into the build-generated `MATCHABLE_PROPERTIES` table (`capability/generated.rs:7`, generated from schema v20) plus the virtual `codec_kind`. `None` = "not in the capability model" = the `UnknownProperty` condition.
- **The pinned version and the skew hook already exist.** `PINNED_IDENTIFICATION_FORMAT_VERSION = 20` (`capability/mod.rs:13`); `Identification.format_version` is parsed from `-J` (`identify.rs:187-190`) and its doc comment (`identify.rs:122-124`) already says it is "compared against PINNED... to detect schema skew (spec 9.2)". `DiagCode::UnknownPropertySkew` exists in the table. Nothing currently emits it: it is a dead warning today.
- **The matcher already matches untyped, with no code change needed.** `parse_track` (`identify.rs:200-222`) stores *every* scalar property from the `-J` `properties` object into `Track.properties` (`BTreeMap<String, PropValue>`), regardless of whether the capability model knows the name; non-scalars (arrays/objects/null) are dropped (`PropValue::from_json`, `identify.rs:32-45`). `Track::get` (`identify.rs:69-76`) returns whatever is in that map. In the matcher, `exact_matches` (`matcher.rs:94-133`) for any property not special-cased (`language`, `codec_kind`) does `item.get(prop)` then `scalar_eq` (`matcher.rs:184-194`, with int/float cross-comparison). So an unknown property already resolves to a correct untyped equality *if it is present on the track*. The only special case: an **absent** property whose name is a known Boolean gets the Matroska false-when-absent shortcut (`matcher.rs:127-130`, via `matchable_type`); an absent *unknown* property returns `None` there and does not match, which is the correct honest behavior (we cannot assume false-when-absent for a name we do not know).

Consequence for scoping Task 16: the forward-compat path is almost entirely a
`validate.rs` decision plus a plan-time warning emission. The matcher's untyped
comparison is a no-op to build (it already works); at most it needs to learn to
strip an opt-in marker (shape B) or nothing at all (shapes A/C). This keeps the
implementation surface small and argues against any shape that demands new
matcher typing machinery.

**One architectural constraint pre-filters the design space.** Spec 5.5: "dry-run
is a strict superset of validate, never a subset." Every diagnostic `validate`
(level 1) emits, `dry-run` (level 2) must also emit. This forbids the tempting
"gate on the runtime version" hybrid, where an unknown property is an *error* at
`validate` (no mkvmerge, assume typo) but a *warning* at `dry-run` (mkvmerge
present, `format_version` newer). That inverts the superset relation (dry-run
would suppress a validate error) and is rejected on that ground alone. The
disambiguator therefore has to be **stable across levels**: either "always
reject" (A), a static in-profile marker readable at every level (B), or "always
downgrade" (C). This is the single most decisive filter in the analysis, and it
is why the "obvious" version-gated approach is not one of the three shapes.

## Shape A steelman: did-you-mean suggestion on `UnknownProperty`

Keep the config-time hard-reject exactly as is; enrich the diagnostic with a
`suggestion` param naming the nearest capability property (edit distance over
`MATCHABLE_PROPERTIES`), so `langauge` yields "did you mean `language`?".

**Honors typo protection: maximally.** This is the pure typo-protection play. It
does nothing except make the already-correct rejection more actionable. Zero risk
of a typo ever matching untyped, because nothing ever matches untyped. The
strongest shape on the axis the hard-reject exists to defend.

**Forward-compat delivery: none, by design.** This shape does not make §9.2
reachable; it concedes that §9.2's automatic-untyped promise was itself the
mistake and removes it. Forward compatibility is then delivered out-of-band: a
user on a newer mkvmerge who needs a new property waits for a Muxsmith build with
a bumped pinned schema, which spec §9.1 already frames as "a normal PR". The
argument that this is acceptable: the population of users who (i) have upgraded
mkvmerge past Muxsmith's pinned schema, (ii) need to *match* on a property added
in that gap, and (iii) cannot wait one release cycle is small, because matching
is dominated by long-stable properties (`type`, `language`, `codec_id`, the
flags) and brand-new schema properties are rare and exotic (new HDR/projection
metadata). Under that read, the runtime untyped path is speculative generality
(YAGNI) and the pinned-schema-bump is the honest, type-safe forward-compat story.

**Config readability: best.** No new syntax. Profiles stay exactly as documented;
the only change is better error text.

**Implementation surface: smallest.** `validate.rs` `unknown_property` gains a
suggestion param (a small edit-distance scan of `MATCHABLE_PROPERTIES`, capped);
one Fluent hint variant. `matcher.rs` untouched. `UnknownPropertySkew` becomes
provably dead and is deleted (with its C1 fixture obligation) or repurposed; §9.2
is amended to drop the automatic-untyped-matching sentence.

**Version skew, operationally:** a user upgrades mkvtoolnix; a profile matching a
newly added property is rejected at `validate` with "unknown property `X`". The
user has no in-tool escape; the fix is "wait for a Muxsmith release that pins the
newer schema" (or self-build). For the common upgrade (newer mkvmerge, but the
profile only touches stable properties) there is no skew symptom at all, because
those properties are still in the pinned model.

**Where it violates the philosophy:** it resolves the tension by amputating one
side. It abandons a promise the approved spec made, and it couples the user's
ability to use a property to Muxsmith's release cadence rather than to their own
mkvtoolnix install. Defensible as a scope cut, but it is a cut, not a
reconciliation.

## Shape B steelman: explicit opt-in marker (`raw:` prefix)

A `raw:`-prefixed property name inside `exact`/`substring`/`regex` bypasses the
capability existence/type/domain checks and matches untyped; a plain (unprefixed)
name still hard-rejects. Example:

```yaml
match:
  exact:
    type: audio
    raw:dolby_complexity_index: 3
```

The disambiguator is *user intent expressed in the profile text*: the user
asserts "this name is deliberate, not a slip." That marker is static and present
at every operation level, so it satisfies the 5.5 superset invariant without a
version-gated split.

**Honors typo protection: preserved for the whole plain namespace.** A bare typo
`langauge: de` still hard-rejects exactly as today. The only way to reach untyped
matching is to type a six-character prefix, which no one does by accident. Typo
protection is not weakened; it is made explicitly waivable, per-property, by an
act that is hard to perform accidentally. This matches the persona's "explicit
over magic" and "config over convention when the convention is not widely
understood": the escape valve is visible in the profile, greppable, and local to
the one property it applies to (a typo in a *known* property elsewhere in the same
rule is still caught).

**Forward-compat delivery: full, and honestly typed.** §9.2's intent ("matchable
as untyped values... without lying about type safety") is delivered: `raw:`
matching is explicitly untyped, and the untyped-ness is declared, not inferred.
`UnknownPropertySkew` gets a real reason to fire (plan time, when a `raw:`
property is consumed and `format_version > PINNED`), so the dead warning comes
alive with accurate semantics. §9.2 is amended from "automatic on skew" to
"opt-in via `raw:`, acknowledged by `UnknownPropertySkew` at plan time".

**Config readability: a modest wart.** A `raw:` prefix is one more thing to learn,
and `raw:foo: true` is slightly odd to read (though it parses cleanly as YAML: a
colon not followed by a space stays inside the plain scalar key, so no quoting is
needed). The cost is bounded and self-documenting: the prefix *is* the
explanation. A nested-block alternative (`exact: { raw: { foo: 3 } }`) reads
marginally cleaner but costs a model change and splits one property map into two;
the inline prefix is the smaller surface.

**Implementation surface: moderate, still `validate.rs`-centric.** `validate.rs`:
in `validate_expr`, strip a leading `raw:`; if present, skip the
existence/type/domain checks and (recommended) emit a config-time `RawProperty`
info diagnostic so `validate` alone signals the bypass. `matcher.rs`: strip
`raw:` before `item.get`; because `matchable_type` returns `None` for the
stripped unknown name, the false-when-absent Boolean shortcut correctly does not
fire (absent untyped property does not match). `planner`: emit
`UnknownPropertySkew` when a `raw:` property participates and `format_version >
PINNED`. New DiagCode `RawProperty` (info) carries a C1 exhaustive-fixture
obligation (progress.md C1); the minimal variant omits `RawProperty` and relies
only on the plan-time `UnknownPropertySkew`, trading validate-time honesty for
one fewer code. No new matcher *typing* machinery, because untyped comparison
already exists.

**Version skew, operationally:** a power user on Arch/Homebrew running mkvmerge
100 against a Muxsmith build pinned at schema 20 needs to match a property added
after 20. They add `raw:new_hdr_field: ...` and it works today, without waiting
for a Muxsmith release; `dry-run` prints an `UnknownPropertySkew` warning naming
the property and the version gap, so the untyped match is visible, not silent.
A user who fat-fingers `raw:` onto a typo gets the honest downstream signal
(`MissingTrack` for a non-optional rule, because nothing has that property) plus
the skew warning; they opted out of protection knowingly.

**Where it violates the philosophy:** it does not, much. The residual costs are
the syntax-surface tax (one prefix, one info code) and one genuine footgun worth
a guard: `raw:` on a property that *does* have special semantics (`raw:language`
bypasses the ISO-639/BCP-47 normalization and `raw:codec_kind` bypasses the alias
expansion, degrading them to byte-literal equality). That should warn
(`RawOnKnownProperty`) rather than silently mislead.

## Shape C steelman: warning-only downgrade

Unknown property name becomes a warning (not an error) at every level plus an
untyped match attempt. This is §9.2's literal promise applied unconditionally,
with no opt-in and no version gate.

**Honors typo protection: it does not, at config time.** A typo `langauge: de`
passes `validate` with only a warning. This is the shape most directly at odds
with the hard-reject's reason for existing. The steelman is that the typo is not
fully silent: because no track carries a `langauge` property, the rule matches
zero tracks, and for a **non-optional** rule that surfaces as `MissingTrack`
(error) at `dry-run`, whose hint lists near-misses (5.2). So a typo in a required
rule still fails the batch, one level later and with a worse message than "did you
mean `language`".

**But the holes are real and are exactly the silent-drop cases.** For an
`optional: true` rule, a typo'd condition silently matches nothing and the track
is silently dropped, no diagnostic. Worse, a typo inside a `not:` sub-expression
inverts: a condition that can never be true makes the `not:` always satisfied, so
the rule matches tracks it should have excluded, silently and wrongly. These are
the precise "declarative rule silently does the wrong thing" failures the
hard-reject was built to prevent, and shape C reintroduces them wholesale. The
`UnknownPropertySkew`/downgraded warning is emitted, but a warning among many in a
bulk run is easy to miss, and nothing forces the user to reconcile it.

**Forward-compat delivery: full and automatic.** No opt-in, no waiting. Every
unknown property just works untyped. This is the shape's one clear win, and it is
the cheapest path to a literally-§9.2-compliant tool.

**Config readability: unchanged.** No syntax at all.

**Implementation surface: small.** `validate.rs`: change the `UnknownProperty`
severity from error to warning (or emit `UnknownPropertySkew` in its place); no
opt-in parsing. `matcher.rs`: unchanged (untyped already works). Optionally emit
`UnknownPropertySkew` at plan time when `format_version > PINNED` to preserve the
version semantics; but note the config-time warning is unconditional, so on an
*older-or-equal* mkvmerge a typo is downgraded too, with no version excuse.

**Version skew, operationally:** the newer-mkvmerge user's new property just
matches, no ceremony. The cost lands on everyone else: every profile author now
runs without the config-time safety net, and the failure surface for ordinary
typos moves from "immediate, precise, config-time error" to "maybe a
`MissingTrack` later, maybe a silent drop, maybe a silent inverted `not:`". For a
tool whose entire value proposition is unattended correctness over a batch, that
is the wrong trade.

**Where it violates the philosophy:** it resolves the tension by amputating the
*other* side from shape A. It honors forward-compat maximally and typo protection
minimally, and it does so unconditionally rather than only under real skew.

## mkvtoolnix parity finding (SI-3)

Behavior read from the GPL source at `~/Downloads/mkvtoolnix` (facts and
locations only, no code copied).

- **Unknown track properties: preserved untyped, never rejected (shape-C-like
  permissiveness).** The GUI assigns the entire `-J` `properties` object wholesale
  into a generic `QVariantMap m_properties` (member declared
  `src/mkvtoolnix-gui/merge/track.h:24`; assigned at
  `src/mkvtoolnix-gui/util/file_identifier.cpp:310`,
  `track->m_properties = obj.value("properties").toMap();`). Its typed widgets
  then *read* known keys out of that map by name (`Track::setDefaultsBasics`,
  `src/mkvtoolnix-gui/merge/track.cpp:277ff`) without removing them. Unknown keys
  stay in the map, inert: no widget, no command-line path. An older GUI on a newer
  mkvmerge degrades gracefully, using the keys it knows and carrying the rest.
- **`identification_format_version`: ignored entirely.** The string does not occur
  anywhere under `src/mkvtoolnix-gui/`. `FileIdentifier::parseOutput`
  (`src/mkvtoolnix-gui/util/file_identifier.cpp:152`) reads only `container`,
  `tracks`, `attachments`, `chapters`, `global_tags`, `track_tags`; the version
  integer is present in the parsed map but never compared, warned on, or errored
  on.
- **Producer-side schema version:** `ID_JSON_FORMAT_VERSION = 20`, defined at
  `src/merge/id_result.h:37` (the `MTX_IDENTIFICATION_FORMAT_VERSION` spelling in
  the brief does not exist; the real symbol is `ID_JSON_FORMAT_VERSION`). This is
  the mkvmerge-side emitter constant; the GUI does not reference it.

**Why the parity does not transfer as a licence to be permissive.** mkvtoolnix-gui
has *no facility to match, filter, or select tracks on arbitrary property names*.
It exposes a fixed set of typed widgets, each hard-wired to one known key. There
is no declarative rule language, so there is no surface on which a mistyped
property name can silently never-match. mkvtoolnix can therefore afford to
"preserve and ignore" unknown properties precisely because the typo failure mode
does not exist in its model. Muxsmith's rule language is exactly the surface that
creates that failure mode. So SI-3 says "preserve the unknown, do not reject the
container" (which Muxsmith already does at the `identify` layer:
`parse_track` keeps every scalar), but it gives *no* cover for letting an unknown
name flow untyped into a *match rule* without an explicit act. That distinction is
the argument for opt-in (B) over automatic (C): match mkvtoolnix's permissiveness
at the identify/data layer (already done), diverge deliberately at the rule layer
where Muxsmith has a hazard mkvtoolnix does not. This is the same "deliberate
divergence, because a batch rule tool must say what mkvtoolnix can leave unsaid"
pattern already recorded for `EmptyPlan` (progress.md SI-3 memo, T6).

## Recommendation + tradeoff

**Recommend shape B (explicit `raw:` opt-in).** It is the only shape that holds
both goals at once: plain names keep the config-time hard-reject that protects
against the worst failure mode, while a deliberate, greppable, per-property marker
delivers honestly-typed forward compatibility that works on the user's own
mkvtoolnix without waiting for a Muxsmith release, and it revives
`UnknownPropertySkew` with accurate semantics. It aligns with the "explicit over
magic" spine of the project and with the SI-3 reading (permissive at the data
layer, deliberately gated at the rule layer).

**One-line rationale:** a static in-profile opt-in is the only disambiguator that
survives the 5.5 level-superset invariant while keeping typo protection intact and
making §9.2 reachable and honest.

**One-line tradeoff:** it adds one syntax marker and one info diagnostic to the
config surface (plus a `RawOnKnownProperty` guard for the `language`/`codec_kind`
footgun), where shape A would add nothing but drop the §9.2 promise outright.

**Marked as Şenol's call (product-shaped, Medium authority):** whether
forward-compatibility-without-a-release is worth any config-surface cost at all is
a judgment about Muxsmith's audience, not a correctness question. If he reads the
newer-mkvmerge-power-user scenario as YAGNI for this tool, shape A (did-you-mean +
formally drop §9.2's automatic promise) is the correct, cleaner downgrade, and its
deltas are below. Shape C is not recommended under either reading: it pays the
full typo-protection cost unconditionally, including on older mkvmerge where no
skew excuse exists.

## Acceptance test cases for the recommended shape (B)

Semantics Task 16 implements and §9.2 is amended to. `PINNED = 20`.
DiagCodes: existing `UnknownProperty` (error), existing `UnknownPropertySkew`
(warning), new `RawProperty` (info, config-time), new `RawOnKnownProperty`
(warning, config-time). The last two carry C1 exhaustive-fixture obligations; the
minimal variant (see note after the tables) drops `RawProperty`.

### B-1..B-3: config-time (`validate`), typo protection preserved

| # | Profile fragment | Expected diagnostics |
|---|---|---|
| B-1 | `exact: { langauge: de }` | `[UnknownProperty { config_path: "tracks[0].match.exact.langauge", property: "langauge" }]` (error), unchanged from today. Rule rejected at config time; nothing reaches the matcher. |
| B-2 | `exact: { raw:dolby_complexity_index: 3 }` | `[RawProperty { config_path: "tracks[0].match.exact.raw:dolby_complexity_index", property: "dolby_complexity_index" }]` (info). No `UnknownProperty`. No `ValueTypeMismatch` (integer value accepted untyped). |
| B-3 | `substring: { raw:new_text_field: foo }` | `[RawProperty { ... property: "new_text_field" }]` (info). No `UnknownProperty`, no `NotStringProperty` (untyped, assumed string-capable for substring). |

### B-4: config-time footgun guard (`raw:` on a property with special semantics)

| # | Profile fragment | Expected diagnostics |
|---|---|---|
| B-4 | `exact: { raw:language: de }` | `[RawOnKnownProperty { property: "language" }]` (warning). Accepted, but flags that `raw:` bypasses ISO-639/BCP-47 normalization; the match degrades to byte-literal equality (see B-8). Same expected for `raw:codec_kind`. |

### B-5..B-8: matcher behavior (untyped comparison)

Given a track and an `exact` rule; expected match result. `lang` index as in the
matcher unit tests.

| # | Track properties | Rule | Matches? | Why |
|---|---|---|---|---|
| B-5 | `{ dolby_complexity_index: Int(3) }` | `exact: { raw:dolby_complexity_index: 3 }` | yes | `scalar_eq(Int(3), Int(3))` |
| B-6 | `{}` (property absent) | `exact: { raw:new_flag: true }` | **no** | absent + unknown type -> no false-when-absent shortcut (`matchable_type` is `None`); contrast a known Boolean flag, which would match `false` when absent |
| B-7 | `{ new_gain: Float(6.0) }` | `exact: { raw:new_gain: 6 }` | yes | int/float cross-compare (`scalar_eq` Int vs Float) |
| B-8 | `{ language: Str("ger"), language_ietf: Str("de") }` | `exact: { raw:language: de }` | **no** for `ger`-only, matches only literal `de` | `raw:` opts out of language normalization: byte-literal equality against the `language` value; matches `de` here only via `language_ietf`'s literal `de`. Documents the B-4 footgun. |

### B-9..B-11: plan-time skew warning and the "opted-in but wrong" path

| # | `format_version` | Track has property? | Rule (`optional`?) | Expected plan-time diagnostics | Plan produced? |
|---|---|---|---|---|---|
| B-9 | 21 (> PINNED) | yes | `exact: { raw:new_prop: X }`, non-optional | `[UnknownPropertySkew { property: "new_prop", found_version: 21, pinned: 20 }]` (warning); rule resolves to the track | yes |
| B-10 | 20 (== PINNED) | yes | same, non-optional | `[UnknownPropertySkew { property: "new_prop", found_version: 20, pinned: 20 }]` (warning) - the match is untyped regardless of version; params let the message distinguish "newer schema" from "same-version untyped" | yes |
| B-11 | 21 | **no** (absent on all tracks) | same, non-optional | `[UnknownPropertySkew { ... }]` (warning) **and** `MissingTrack` (error) | **no** - the `raw:` path does not suppress the genuine "nothing matched"; opting into raw on a name nothing carries still fails a required rule |

Design note carried into the memo: `UnknownPropertySkew` fires per `raw:`
property *consumed at plan time*, with `found_version`/`pinned` params, so one
code covers both "genuine newer-schema skew" (B-9) and "untyped match on
same-version" (B-10); the message template branches on the params. §9.2 is
amended to: unknown property names are rejected at config time unless marked
`raw:`; a `raw:` property matches untyped and is acknowledged by
`UnknownPropertySkew` at plan time, whose params report whether the runtime schema
is newer than pinned.

**Minimal variant (fewer DiagCodes):** drop `RawProperty` (B-2/B-3 then validate
clean with no diagnostic) and fold B-4's guard into a single `RawOnKnownProperty`.
Trades away the config-time "you are using an unchecked property" signal (so
`validate` alone no longer announces the bypass) for two fewer codes and two fewer
C1 fixtures. Recommend keeping `RawProperty`: the whole point of B over C is that
the escape valve is *visible*, and a silent bypass at `validate` undercuts that.

## Test-case deltas if the human picks another shape

### If shape A (did-you-mean + drop the §9.2 automatic promise)

- Remove B-2..B-11 (no `raw:` syntax, no untyped rule path, no plan-time skew
  warning).
- **A-1:** `exact: { langauge: de }` -> `UnknownProperty` (error) **with** a
  `suggestion: "language"` param (nearest `MATCHABLE_PROPERTIES` name by capped
  edit distance, e.g. <= 2). Fluent hint renders "unknown property `langauge`; did
  you mean `language`?".
- **A-2:** `exact: { xzptlk: 1 }` (no near neighbor) -> `UnknownProperty` (error),
  **no** `suggestion` param; hint omits the "did you mean" clause.
- **A-3 (regression pin):** every currently-passing `UnknownProperty` test still
  errors; the only change is the optional added param.
- **Spec/DiagCode cleanup:** §9.2 amended to remove "become matchable as untyped
  values... instead of failing"; forward-compat restated as "bump the pinned
  schema (normal PR, §9.1)". `UnknownPropertySkew` becomes dead -> remove it and
  its C1 fixture, or repurpose; note the removal in the spec self-contradiction
  sweep (doctrine §1) so 5.2's table row goes too.

### If shape C (warning-only downgrade)

- Remove B-2..B-4 (no opt-in syntax) and the `RawProperty`/`RawOnKnownProperty`
  codes. Matcher tests B-5..B-8 survive but with the property name unprefixed
  (`exact: { dolby_complexity_index: 3 }`), since every unknown name now flows
  untyped.
- **C-1 (downgrade):** `exact: { langauge: de }` -> config-time **warning** (either
  `UnknownProperty` re-severitied to warning, or `UnknownPropertySkew` emitted in
  its place), **no** error. At `dry-run` the rule matches 0 tracks -> `MissingTrack`
  (error) for a non-optional rule. Test asserts the typo still ultimately fails a
  required rule, one level later.
- **C-2 (the optional hole - must be an explicit test, it is the cost):**
  `optional: true` rule with `exact: { langauge: de }` -> config-time warning only;
  at plan time the rule silently matches nothing and the track is dropped with
  **no error**. Test documents the silent-drop.
- **C-3 (the `not:` inversion hole):** `not: [ exact: { langauge: de } ]` -> the
  typo'd inner condition is never true, so `not:` is always satisfied and the rule
  matches tracks it should exclude, silently. Test documents the semantic
  inversion.
- **C-4 (version-agnostic downgrade):** on `format_version` 20 (== PINNED), an
  unknown name is still downgraded to a warning - there is no skew, yet typo
  protection is gone. Test pins that the downgrade is unconditional (the cost lands
  even absent real skew), which is the core argument against C.
- **Matcher:** unchanged (untyped already works); only `validate.rs` severity
  changes plus an optional plan-time `UnknownPropertySkew` when `format_version >
  PINNED`.
