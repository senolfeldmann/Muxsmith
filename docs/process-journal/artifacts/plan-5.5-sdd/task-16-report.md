# Task 16 report: reachable forward-compat path for unknown properties (D32, #4)

Shape B (`raw:` opt-in) implemented against the D32 acceptance cases B-1..B-11.
Worktree `.worktrees/t16`, branch `plan55-t16`. TDD: all B-cases written as
failing tests first (B-1 as a passing regression pin), then implemented to green.

## Correction to the brief's "implementation facts" (verified against post-T9 code)

The brief and `d32-analysis.md` both state "UnknownPropertySkew is currently dead
code emitted nowhere; the work is a validate.rs decision + plan-time emission."
That is **stale against post-T9 master.** There is one live production emission:
`planner.rs:445-451` emitted `UnknownPropertySkew` **per file** when
`ident.format_version > PINNED`, with a `version` param and `config_path: "input"`.
(The two other references, `commands/mod.rs:183` and `i18n.rs:209`, are
`#[test]`-only, not production.)

Consequence: the decided semantics did not "revive dead code"; they **repurpose a
live per-file emission** into a per-consumed-`raw:`-property emission. I removed the
old per-file trigger. This is required by the decided semantics, not a scope
addition:
- B-10 fires the skew at `found_version == pinned` (20 == 20), which the old
  `> PINNED` gate never did.
- Under shape B the untyped path is opt-in, so a newer-schema file with **no**
  `raw:` property must emit **no** skew (all referenced names are typed/pinned) -
  the exact opposite of the old unconditional per-file warning.

Verified: `grep` for every `UnknownPropertySkew` / `format_version` reference; the
only production emitter was the one removed. Full-workspace test suite (green)
confirms no pre-existing test depended on the old per-file behavior.

## Per-test-case results (B-1..B-11 all green)

| # | Case | Result |
|---|---|---|
| B-1 | `exact:{langauge:de}` -> `UnknownProperty` error, `tracks[0].match.exact.langauge`, unchanged | PASS (regression pin; bare typo still hard-rejects) |
| B-2 | `exact:{raw:dolby_complexity_index:3}` -> `RawProperty` info; no UnknownProperty/ValueTypeMismatch | PASS (config_path keeps literal `raw:` key; `property` param stripped) |
| B-3 | `substring:{raw:new_text_field:foo}` -> `RawProperty` info; no UnknownProperty/NotStringProperty | PASS |
| B-4 | `exact:{raw:language:de}` / `raw:codec_kind` -> `RawOnKnownProperty` warning | PASS (both; no CodecKindExactOnly on the raw form) |
| B-5 | track `{dolby_complexity_index:Int(3)}` + `raw:dolby_complexity_index:3` -> match | PASS |
| B-6 | `{}` + `raw:new_flag:true` -> no match (no false-when-absent for unknown type) | PASS |
| B-7 | `{new_gain:Float(6.0)}` + `raw:new_gain:6` -> match (int/float cross-compare) | PASS |
| B-8 | `{language:"ger",language_ietf:"de"}` + `raw:language:de` -> no match | PASS (see semantics note) |
| B-9 | fmt 21, prop present, non-optional -> `UnknownPropertySkew{new_prop,21,20}` + plan | PASS |
| B-10 | fmt 20, prop present -> `UnknownPropertySkew{new_prop,20,20}` + plan | PASS |
| B-11 | fmt 21, prop absent everywhere -> `UnknownPropertySkew` **and** `MissingTrack`, no plan | PASS |

### B-8 semantics note (analysis prose was internally inconsistent)

The `d32-analysis.md` B-8 row says both "byte-literal equality against the
`language` value" **and** "matches `de` here only via `language_ietf`'s literal
`de`" - contradictory, since `raw:language` looks up the property named verbatim
(`language` = "ger"), not `language_ietf`. I implemented the crisp, natural
meaning of `raw:`: **byte-literal equality against the single property named,
no dual-field lookup, no normalization.** So `raw:language:de` vs a track whose
`language` is "ger" -> **no match** (matches the analysis row's leading "**no**"
and the memo's "only via the literal `de`"). The test also pins the positive
contrast (`raw:language:ger` matches; normal `language:de` still normalizes).

## Design decisions on dimensions the acceptance table left open

- **RawProperty vs RawOnKnownProperty are mutually exclusive.** The D32 prose reads
  "emits `RawProperty` ... and, on a known-special property, `RawOnKnownProperty`",
  which could mean "both fire for `language`/`codec_kind`." The acceptance table
  (B-4) lists **only** `RawOnKnownProperty`, and the brief says implement the cases
  "verbatim." I emit `RawOnKnownProperty` alone for `language`/`codec_kind` (the
  more specific, non-redundant diagnostic). Assumption - flag if both were intended.
- **"Known-special" set = `{language, codec_kind}`,** hardcoded, matching the
  decision's explicit enumeration and exactly the two arms `matcher::exact_matches`
  special-cases. A `raw:` on a **known-but-not-special** property (e.g.
  `raw:forced_track`) emits `RawProperty` (info), same as an unknown raw property -
  it announces the untyped bypass. Not in the acceptance cases; assumption.
- **UnknownPropertySkew message is non-branching** (3 params: `property`,
  `found_version`, `pinned`), surfacing both version numbers rather than adding a
  derived `newer` selector param. The decision names only `found_version`/`pinned`;
  a reader distinguishes newer-vs-same from the values. Fluent cannot compare
  numbers, so a branch would require a 4th derived param the decision does not name.
- **Skew emission granularity:** one warning per `(file, rule, distinct raw
  property name)`, deduped per rule via a BTreeSet (also fixes order); emitted
  after source resolution, before the match-count branch, so it fires regardless of
  match/miss/ambiguity (B-11). `config_path = tracks[i].match`. For an external
  source the `found_version` is the **donor's** schema (the file whose tracks are
  matched untyped); failed external resolution `continue`s before emission (no
  untyped match attempted).

## Model layer / serde

No model change needed. `MatchExpr.{exact,substring,regex}` are
`BTreeMap<String, _>`; `#[serde(deny_unknown_fields)]` is at the struct level, not
the inner maps, so an arbitrary `raw:`-prefixed key survives as the map key string.
YAML parses `raw:dolby_complexity_index: 3` as key `raw:dolby_complexity_index`
(a colon not followed by a space stays in the plain scalar); confirmed end-to-end
by B-2/B-3 (validate) and B-5..B-8 (matcher parses via `yaml_serde`). Reported as
in-scope-checked, no change required.

## Spec amendment + self-contradiction sweep

Amended `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`:
- **§9 item 2 (the "9.2"):** rewritten to the decided semantics verbatim in intent
  - config-time hard-reject unless `raw:`, untyped byte-literal match, config-time
  `RawProperty`/`RawOnKnownProperty`, plan-time `UnknownPropertySkew` per consumed
  `raw:` property with `found_version`/`pinned`, pinned-schema bump as the typed
  alternative. Removed "become matchable as untyped values ... instead of failing".
- **§5.2 diagnostics table:** `UnknownPropertySkew` row rewritten to the raw:/params
  semantics; added `RawProperty` (info) and `RawOnKnownProperty` (warning) rows;
  `UnknownProperty` row notes the `raw:` opt-out.
- **§4.3:** the substring/regex "non-string property is a type error" bullet now
  carries the `raw:` exemption.
- **§4.4:** added a "`raw:` opt-in" bullet (rationale, syntax, untyped semantics,
  the two config-time codes + plan-time skew, `changes`-not-accepted, YAML parsing).
- **§5.4 static lint:** "unknown properties" now "(unless `raw:`-opted)".

**Sweep - what I checked, and the finding:**
- §4.3 (match algebra), §4.4 (property model), §5.2 (diag table), §5.4 (static
  lint), §5.5 (operation-level superset), §9.1 (build-time), §9.2 (runtime): grep
  for `instead of failing | become matchable | newer schema | version skew |
  untyped | schema skew | forward comp` after amending returns **only the amended
  passages** - no residual old "automatic-on-skew" wording anywhere.
- §5.5 superset invariant **upheld:** validate emits the config-time raw codes;
  dry-run/run additionally emit the plan-time skew - a strict superset, never an
  inversion. This is exactly why shape B (a static in-profile marker) was the only
  viable disambiguator.
- §9.1 ("upgrading the pinned schema is a normal PR") stays valid and is now
  referenced from §9.2 as the typed alternative to `raw:`.

Also corrected two **code** doc comments that still described the old per-file
trigger (doctrine §1 / BUILDING.md "rustdoc states meaning"):
`capability/mod.rs` `PINNED_IDENTIFICATION_FORMAT_VERSION` and `identify.rs`
`Identification.format_version`.

## Files changed

Core: `report/mod.rs` (2 DiagCodes + skew doc), `profile/validate.rs` (raw: strip +
`raw_opt_in_diagnostic`), `matcher.rs` (raw: strip in exact/substring/regex +
`strip_raw`), `planner.rs` (removed per-file emission; `emit_raw_property_skew` +
`collect_raw_props` in the rule loop), `capability/mod.rs` + `identify.rs` (doc).
CLI: `catalog_completeness.rs` (fixtures for 2 new codes + skew params),
`i18n.rs` + `commands/mod.rs` (updated the two skew test sites off the old `version`
param). Catalog: `locales/en/diagnostics.ftl` (EN-only: `raw-property`,
`raw-on-known-property`, rewritten `unknown-property-skew`). Spec + tests as above.

Fixture-guard lockstep: the exhaustive `fixture_args` match forced the two new
DiagCode fixtures in the same commit; `catalog_completeness` (4 tests) green.

## Gate (nine parts, all green)

1. `cargo test --workspace` - all suites pass (B-1..B-11 green; no regressions).
2. `cargo fmt --check` - clean (applied `cargo fmt`).
3. `cargo clippy --workspace --all-targets -D warnings` - clean.
4. `cargo deny check` - advisories/bans/licenses/sources ok.
5. `pnpm lint` - clean.
6. `pnpm build` - ok.
7. `pnpm check:i18n` - ok (new diagnostics.ftl ids are dynamic-only, allowlisted;
   the 12 "unused" warnings are the pre-existing shell IpcError codes, not mine).
8. `pnpm test:e2e` - 3 passed.
9. `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` - clean.

`pnpm install --frozen-lockfile` run once (node_modules was absent).

## Self-review

- GUI needs no code change: the frontend globs `locales/*/diagnostics.ftl`
  (`src/i18n/index.ts`) and renders diagnostics by `code`, so the new messages are
  picked up automatically; e2e green confirms.
- Suggestion-engine re-simulation is unaffected: the skew signature appears
  identically in baseline and simulations (discriminators add typed keys, never
  `raw:` keys), so `resolves_without_regression`'s multiset containment holds.
- `changes` deliberately unchanged: `raw:` is a match-side opt-in only; a
  `changes:{raw:...}` key stays `UnknownSettableProperty` (documented in §4.4).

## Concerns / follow-ups

- **Brief-vs-code factual drift (above):** the brief author believed the skew was
  dead code. My removal of the live per-file emission is a small behavior change to
  a shipped diagnostic (a newer-schema file with no `raw:` property no longer warns).
  Correct under D32, but worth an explicit nod in review since it deviates from the
  brief's stated premise.
- **Two marked assumptions** (RawProperty/RawOnKnownProperty exclusivity for
  language/codec_kind; RawProperty for known-non-special raw props) resolve
  dimensions the acceptance table left open. Both follow the acceptance-table-verbatim
  reading; flag if the D32 prose's "and" intended both codes for special names.
- **B-8 analysis prose** was self-contradictory; I implemented the crisp
  byte-literal-single-property reading. If the intent was dual-field byte-literal
  (matching `language_ietf` too), B-8 flips to a match and the matcher needs the
  language/language_ietf pair minus normalization - not what "raw: opts out of the
  language special-casing" naturally means. Recommend confirming the crisp reading.
