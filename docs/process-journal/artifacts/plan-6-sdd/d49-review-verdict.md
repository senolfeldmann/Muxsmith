# Independent review verdict: D49, the `apply_suggestion` seam

Artifact: `docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`
Tree state: HEAD `36bfee5` (D49 cites `41c4cb7`; the two intervening commits are
ROADMAP/ledger text only, so every code citation re-runs unchanged).
Reviewer: independent, no stake in the document.

---

## VERDICT

**NEEDS FIXES** - the decision (option ii) is right and its measurements are
exceptionally sound, but D49's own prescribed test-fix reconstructs the wrong
`Scalar` variant for a Boolean property (the exact error the ADR exists to
prevent), its mandated catalog prose is ungrammatical in the case its own test
constructs, and its "only two failures" claim omits a silent no-op.

Severity counts: **Critical 0 | Important 4 | Minor 3**

Nothing here overturns option (ii). Findings 1-3 are defects an implementer would
carry into the tree; finding 4 is an argument that will be re-litigated.

---

## Findings

### 1. (Important) The prescribed fix for test site `:722` carries a display String where the property is Boolean

**Where:** D49, "Interface changes" -> *Three comparison sites*:

> `:325`'s `value == "Chapter 1: Intro"` becomes `value == &Scalar::Str("Chapter 1: Intro".to_string())`, and `:722`/`:890` take the same shape against whichever literal they already name.

**What is wrong.** "The same shape" is `&Scalar::Str(<literal>.to_string())`. That
is correct for two of the three sites and wrong for the third:

| site | guard | property | declared type | correct post-D49 literal |
|---|---|---|---|---|
| `:325` | `property == "track_name"` | `track_name` | `PropType::String` (`generated.rs:66`) | `&Scalar::Str("Chapter 1: Intro".to_string())` ✅ as written |
| `:722` | `property == "forced_track"` | `forced_track` | **`PropType::Boolean`** (`generated.rs:41`) | **`&Scalar::Bool(true)`** ❌ D49 says `Scalar::Str("true")` |
| `:890` | `property == "language"` | `language` | `PropType::String` (`generated.rs:43`) | `&Scalar::Str("eng".to_string())` ✅ |

`tests/suggestions.rs:722` reads `StructuredEdit::AddNotExact { property, value } if property == "forced_track" && value == "true"`. Post-D49 the engine emits
`Scalar::Bool(true)` there, never `Scalar::Str("true")`. Following D49 literally
produces an assertion that **compiles** (`Scalar: PartialEq`) and fails at
runtime with the misleading message the test already carries
(`"expected AddNotExact{{forced_track,true}} on the optional rule"`).

This is the document's own thesis turned on itself: D49's "Why this and not
reconstruction" section states that the display String is a lossy projection
because "`Bool(true)` and `Str("true")` both project to `"true"`" - and then
reconstructs `Str("true")` from the display string `"true"` for a Boolean
property.

**Evidence gathered.**
- `sed -n '715,730p' crates/muxsmith-core/tests/suggestions.rs` -> the guard is `property == "forced_track" && value == "true"`.
- `sed -n '883,895p'` -> `:890` is `property == "language" && value == "eng"`.
- `sed -n '318,332p'` -> `:325` is `property == "track_name" && value == "Chapter 1: Intro"`.
- `grep -n '("forced_track"\|("language"\|("track_name"' crates/muxsmith-core/src/capability/generated.rs` -> `41: ("forced_track", PropType::Boolean)`, `43: ("language", PropType::String)`, `66: ("track_name", PropType::String)`.
- Probe run (real crate, `plan_batch` against `P_AMBIGUOUS` + `SERIES`): the engine's `forced_track` candidate carries `PropValue::Bool` via `prop_value_as`, i.e. `Scalar::Bool` post-D49.

**Additional hazard at `:890`.** `:890` is a **negative** assertion
(`assert!(!batch.suggestions.iter().any(...))`). A mis-typed literal there never
matches, so `!any` is trivially true and the test **passes vacuously** - it would
not announce the error the way `:722` does. It happens to be correct as written,
but the blanket "take the same shape" instruction is exactly the mechanism that
would silently hollow it out.

**Fix.** Replace the sentence with the per-site enumeration (it is three sites;
write them):
- `:325` -> `value == &Scalar::Str("Chapter 1: Intro".to_string())`
- `:722` -> `value == &Scalar::Bool(true)`
- `:890` -> `value == &Scalar::Str("eng".to_string())`

and note that `:890` is a negative assertion whose mis-typing is silent.

---

### 2. (Important) `apply-rule-index-out-of-range` is ungrammatical in the singular - the case D49's own test constructs - and the house's plural machinery cannot reach it

**Where:** D49, "The catalog entries":

```
apply-rule-index-out-of-range = The suggestion could not be applied: rule { $index } does not exist; the profile has { $rules } rules.
apply-rule-index-out-of-range = Der Vorschlag konnte nicht angewendet werden: Regel { $index } existiert nicht; das Profil hat { $rules } Regeln.
```

**What is wrong.** Two compounding problems.

*(a) The singular renders wrong, and it is the tested case.* Neither line uses a
CLDR plural selector, so `rules == 1` renders "the profile has **1 rules**." /
"das Profil hat **1 Regeln**." D49's own guard
`apply_rejects_a_rule_index_past_the_end` uses `P_AMBIGUOUS`, annotated by D49
itself as `// exactly 1 rule`, and asserts
`ApplyError::RuleIndexOutOfRange { index: 7, rules: 1 }`. The singular is not a
hypothetical edge; it is the only case the ADR exercises.

*(b) A plural selector would not work if added.* IpcError messages render through
`$t(err.code, err.params)` with `params: Record<string, string>` - raw strings.
`@fluent/bundle` only resolves `[one]`/`*[other]` against a JS **number**, so a
selector on `$rules` would always fall through to `*[other]`. The house has a
number-promotion mechanism for exactly this (`src/diagnosticFluentParams.ts`'s
`NUMERIC_DIAGNOSTIC_PARAMS`, mirrored Rust-side by
`muxsmith_cli::i18n::numeric_diagnostic_params`), but it is keyed by **diagnostic
code** and applied only at `DiagnosticsPanel.vue:34`. No IpcError render site
passes through it. So D49's message class cannot currently pluralize at all -
a real gap the ADR neither uses nor records.

The house demonstrably cares: `[one]` selectors exist in `locales/en/cli.ftl`,
`locales/en/gui-batch.ftl`, `locales/en/gui-jobs.ftl` (+ de), and `e2e/smoke.spec.ts`
has a dedicated test *"diagnostics summary and suggestions-capped pluralize their
counts (1 error singular, 2 warnings plural, 1 dropped suggestion singular)"*.

**Evidence gathered.**
- `grep -rn "\[one\]" locales/` -> selectors in `cli.ftl:3,6,9,24,32`, `gui-jobs.ftl:27` (de `:30`), `gui-batch.ftl:32,35,38`; **none in `gui-common.ftl`**.
- `grep -rn '\$t(.*\.code' src/ --include="*.vue"` -> `FirstRun.vue:94`, `RunHistory.vue:155`, `RunHistory.vue:241`, `JobsView.vue:246`, `JobsView.vue:252` all call `$t(<err>.code, <err>.params)` directly; only `DiagnosticsPanel.vue:34` wraps with `diagnosticFluentParams(...)`.
- `src/ipc.ts:16-19` -> `IpcError { code: string; params: Record<string, string> }`.
- `src-tauri/src/error.rs:30-43` -> `pub params: HashMap<String, String>`.
- `cat src/diagnosticFluentParams.ts` -> `NUMERIC_DIAGNOSTIC_PARAMS` holds exactly `suggestions-capped` and `suggestion-partition`; the docstring states the string-argument fall-through explicitly.

**Fix.** Cheapest: rephrase both lines to avoid the plural agreement, e.g.
`...; the profile's rule count is { $rules }.` /
`...; die Regelanzahl des Profils ist { $rules }.` Otherwise extend the
number-promotion mechanism to IpcError params and record that extension as part
of D49. Either way, name the constraint so the implementer does not add a
selector that silently degrades.

---

### 3. (Important) "These are the only two failures the function has" omits the silent no-op that `core-44`'s `or_insert` creates

**Where:** D49, "`ApplyError`":

> Exactly two variants. These are the only two failures the function has: `rule_index_of` returning `None`, and an in-range parse that overruns the rule list. `with_rule_match` cannot fail (it indexes a checked index and merges maps), and `delta_for` is total.

**What is wrong.** "merges maps" elides the semantic `core-44` exists for.
`with_rule_match` (`planner.rs:1856-1861`) merges with
`map.entry(k.clone()).or_insert_with(|| v.clone())`. If the target rule
**already** constrains that property, the delta is silently dropped and
`apply_suggestion` returns `Ok(<unchanged profile>)`. That is a third outcome:
neither an error nor a change.

The engine is protected from it; the apply path is not. `core-44` says so in as
many words: *"a clobbering candidate becomes a no-op then rejected by the
acceptance sim"*. The engine's sim is `resolves_without_regression`
(`planner.rs:1878+`), which rejects a candidate that fails to clear the
ambiguity. `apply_suggestion` runs no sim - D43 forbids the re-plan - and
D43 routes the result through `validate_profile_model`, which **cannot see a
no-op**: the unchanged profile is still perfectly valid, so no diagnostic fires.
The user clicks Apply and nothing happens, silently.

This is not exotic. D43 explicitly anticipates the drift ("the user's model may
have moved since"), and Plan 6 **is** the profile-editor plan, so "suggestion
computed against P, applied to an edited P'" is the normal path, not the corner.

None of the five guards covers it: G1, G2 and G3 all apply to a freshly-parsed
fixture whose rule does not yet carry the property, so `or_insert` always
inserts.

**Evidence gathered.**
- `sed -n '1853,1900p' crates/muxsmith-core/src/planner.rs` -> `or_insert_with` on both the `exact` and `substring` merge loops; `not` uses `extend` (additive, unaffected).
- `core-44-suggestion-no-clobber` statement (via `yaml.safe_load` of `docs/conventions.yaml`), count 3, settled: "a clobbering candidate becomes a no-op then rejected by the acceptance sim".
- D43 (`2026-07-15-plan-6-design.md:470-474`): "the user's model may have moved since, and the honest way to find out is the same validation every other edit gets."

**Scope note.** I am not re-opening D43's no-re-plan ruling. The claim under
review is D49's own exhaustiveness assertion, which is checkable and incomplete.

**Fix.** Enumerate the outcome and rule it - either (a) accept it explicitly
("an apply against a drifted model may no-op; Plan 7 owns surfacing it") so the
document does not assert an exhaustiveness it does not have, or (b) return the
information (a `bool`/`Option<Profile>` "changed" signal, or a third `ApplyError`)
and guard it. At minimum, drop "these are the only two failures" or qualify it to
"the only two *error* variants; a drifted model additionally yields a silent
no-op, accepted because X".

---

### 4. (Important) The rejection of option (i) overstates its case: the Float divergence is behaviorally inert, and the one real break is a case option (i) handles correctly

**Where:** D49, "Rejected: (i) reconstruct the typed `Scalar`..." -> *Why it loses*:

> Reconstructing `min_luminance` as `Float(400.0)` diverges from the simulated `Int(400)` - measured `!=` - which is exactly the `core-03` violation the amendment exists to prevent, reintroduced by the mechanism meant to prevent it.

**What is wrong.** The `!=` is real (I reproduced it). The escalation to
"`core-03` violation" is not. `core-03` is a **behavioural** guarantee - "an
applied suggestion survives the next dry run". `scalar_eq` (`matcher.rs:202-212`)
carries cross-type arms:

```rust
(Scalar::Int(a), PropValue::Float(b)) => (*a as f64) == *b,
(Scalar::Float(a), PropValue::Int(b)) => *a == (*b as f64),
```

so `Scalar::Float(400.0)` **matches** a `PropValue::Int(400)` track. I ran it
against the real matcher: **it matches (`true`)**. The rule still matches, the
ambiguity still resolves, no `MissingTrack`, and `scalar_fits(Float, PropType::Float)`
passes validation. The divergence is a type-identity and rendering difference
(`400.0` vs `400`), not a survival failure.

Worse for the argument: the genuine `core-03` break - `Scalar::Str("true")`
matching nothing - is a case option (i) would get **right**, because
`matchable_type("forced_track") = Boolean` drives the parse straight to
`Bool(true)`. So the one failure D49 measures as decisive is the one failure
option (i) does not have.

D49's §1.6 claim itself - "`matchable_type` is not a function onto `Scalar`" - is
**sound and verified**: `PropType::Float` genuinely admits two variants, so no
total parse exists. That is a legitimate reason to reject (i). The honest case
against (i) is the one D49 already makes in its final paragraph (a new parse
function, a new failure mode, a new inversion guard, two copies instead of one,
plus a cosmetic rendering drift on 5 properties). That case wins on its own.
The overstated sentence is what gets the rejection re-litigated the moment
someone reconstructs the real argument - which is precisely why it matters.

**Evidence gathered.**
- `sed -n '414,426p' crates/muxsmith-core/src/profile/validate.rs` -> `scalar_fits` at `:416-425`, admitting `(Int, Float)` and `(Float, Float)`. D49's citation is exact.
- `grep -n "fn scalar_eq" -A 20 crates/muxsmith-core/src/matcher.rs` -> six arms + `_ => false`, including the two cross-type arms above.
- Probe against the real `muxsmith_core::matcher::matches`, track with `min_luminance: 400` (-> `PropValue::Int(400)`):
  ```
  track min_luminance PropValue = Some(Int(400))
  Scalar::Int(400)     matches track? true
  Scalar::Float(400.0) matches track? true
  --- Bool property ---
  Scalar::Bool(true)   matches? true
  Scalar::Str("true")  matches? false
  ```

**Fix.** Downgrade the claim to what it is: reconstruction diverges in *type
identity* and *YAML rendering* for the 5 Float-declared properties, not in match
behaviour (`scalar_eq` absorbs Int/Float). Let the rejection rest on the
no-total-parse fact (§1.6, sound) plus the cost paragraph (sound). Delete the
"exactly the core-03 violation" sentence.

---

### 5. (Minor) The `ts_rs::TS` import is mandated by omission

**Where:** D49, "The wire shape" mandates
`#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]`
on `StructuredEdit`, and "Interface changes" re-states it as an explicit binding.

**What is wrong.** `derive(TS)` requires `TS` in scope. D49 is otherwise
exhaustive about imports - it names `planner.rs:10` becoming
`use serde::{Deserialize, Serialize};` and confirms `Scalar` is already imported
at `:17` - but never names the `ts_rs` import. There is no precedent to copy:
**no `ts_rs`, `derive(TS)` or `feature = "ts"` exists anywhere in the tree today**
(D44 has not landed; `crates/muxsmith-core/Cargo.toml` has no `[features]`
section and no ts-rs dependency).

The wrong choice is not free. A plain `use ts_rs::TS;` is an **unused import**
when the `ts` feature is off, which fails the house's
`cargo clippy --workspace --all-targets -- -D warnings` gate
(`.github/workflows/ci.yml:86`). The correct form is
`#[cfg(feature = "ts")] use ts_rs::TS;`. The ledger records this exact failure
class twice - `decision-ledger.yaml:2339` ("cfg-gate imports/helpers for cfg-gated
tests: Windows legs went red twice on -D warnings from unix-only imports") and
`:2577`.

Related sequencing note: D49's derive also requires `Scalar: TS`, which only
exists once D44 lands. That dependency is satisfiable (D44 covers `Scalar` -
`2026-07-15-plan-6-design.md:540` measures its emission) but unstated, so a plan
that orders D43/D49 before D44 will not compile.

**Fix.** Name the import and its gate (`#[cfg(feature = "ts")] use ts_rs::TS;`),
and state that the derive lands with or after D44.

---

### 6. (Minor) `report/json.rs:56` is attributed to the CLI crate; the module lives in muxsmith-core

**Where:** D49 §1.7 table row "CLI `--json` report (`report/json.rs:56`)", repeated
in "Interface changes" ("the CLI's `dry-run --json` report (`report/json.rs:56` ...)").

**What is wrong.** The file is `crates/muxsmith-core/src/report/json.rs`. There is
no `crates/muxsmith-cli/src/report/`. The line number is right - `:56` is
`"suggestions": batch.suggestions,` - and the report *is* the CLI's output, so the
substance holds. But the unqualified path sits in a table one row below a
crate-qualified `muxsmith-cli/src/commands/mod.rs:126`, so it reads as
muxsmith-cli and sends the implementer to a path that does not exist.

**Evidence gathered.** `find . -name "json.rs" -not -path ./target/*` -> exactly one
hit, `./crates/muxsmith-core/src/report/json.rs`. `sed -n '45,65p'` on it -> `:56` is
`"suggestions": batch.suggestions,`.

**Fix.** Qualify as `crates/muxsmith-core/src/report/json.rs:56`.

---

### 7. (Minor) The grep backing "no test asserts it" cannot see unquoted TS object keys

**Where:** D49 §1.7 ("No test asserts the JSON shape of `edit`: `grep -rn '"edit"'`
across `*.rs`, `*.json`, `*.ts` (excluding `target/`) returns nothing") and again
in "Interface changes" as the evidence for "No test asserts it".

**What is wrong.** The conclusion is **true** - I re-ran the grep and every wider
search I could construct, and nothing asserts a suggestion's `edit.value`. But the
pattern that establishes it is unsound: `'"edit"'` matches only a *quoted* key, and
a TypeScript object literal writes it unquoted. `e2e/smoke.spec.ts:177` contains
`edit: null` inside the mocked dry-run report - a site the cited grep cannot match
and D49 does not name among its consumers.

That site is inert (`src/ipc.ts:132` types `edit: unknown`, and `null` is assignable
to `unknown`, so `pnpm build`'s type-check sees nothing and the mock neither asserts
the shape nor breaks). So D49's conclusion survives - but by luck of the fixture's
content, not by the method, and the same grep would have missed a mock that *did*
carry a real value.

**Evidence gathered.**
- `grep -rn '"edit"' --include="*.rs" --include="*.json" --include="*.ts" .` (excluding target/node_modules) -> no matches, reproduced.
- `grep -rn "edit:" --include="*.ts" --include="*.vue" .` -> `e2e/smoke.spec.ts:177: edit: null`, `src/ipc.ts:132: edit: unknown`.
- `grep -rn "edit" crates/muxsmith-cli/tests/snapshots/` -> nothing; no insta snapshot contains a suggestion.
- CLI `--json` tests touch suggestions only as `assert_eq!(report["suggestions"].as_array().unwrap().len(), 0)` (`dry_run_cli.rs:320`, `:559`, `:617`) - length, never shape.

**Fix.** Restate the evidence with a pattern covering unquoted keys, and list
`e2e/smoke.spec.ts:177` as a third, inert consumer with the reason it is inert.

---

## Measurements re-run

Every empirical claim I could reach was re-run against the tree. **All reproduced.**
D49's measurement discipline is the strongest part of the document.

| # | D49's claim | Method | Result |
|---|---|---|---|
| 1 | 62 matchable properties | `grep -c '^    ("' generated.rs` | **62** ✅ |
| 2 | 9 Boolean / 5 Float / 27 Integer / 21 String | `grep -o 'PropType::[A-Za-z]*' \| sort \| uniq -c` | **9/5/27/21**, sums to 62 ✅ |
| 3 | The 9 Boolean names | `grep -n 'PropType::Boolean'` | exact match, all 9 ✅ |
| 4 | The 5 Float names | `grep -n 'PropType::Float'` | exact match, all 5 ✅ |
| 5 | `codec`(String,:18), `id`(Integer,:42), `type`(String,:67) | `grep -n` | all three exact ✅ |
| 6 | `prop_value_as` maps `Float -> None` | read `planner.rs:2063-2070` | verbatim ✅ |
| 7 | Reachable set closed at `{Bool,Int,Str}` | total match over `PropValue`'s 4 variants | ✅ (see caveat below) |
| 8 | Only one `AddExact`/`AddNotExact` construction site | `grep -rn "AddExact\|AddNotExact" crates/ src-tauri/` | **only `planner.rs:1744`/`:1751`** ✅ no other site |
| 9 | `delta_for` has 2 call sites (`:1762`, `:1791`) | `grep -rn "delta_for"` | exactly 2 + def ✅ |
| 10 | `with_rule_match` called at `:1400,:1470,:1571,:1646` + `prop_planner.rs:310` ("all five") | `grep -rn "with_rule_match"` | exactly those 5 ✅ |
| 11 | `scalar_eq` has no `(Str,Bool)`/`(Str,Int)` arm; "six arms plus the fallthrough" | read `matcher.rs:202-212` | **exactly 6 arms + `_ => false`** ✅ |
| 12 | `Scalar::Str("true")` matches nothing | real matcher probe | `false` ✅ |
| 13 | No production code reads `.value` on the two exact variants | full grep + read every consumer | ✅ confirmed |
| 14 | `grep -rn '"edit"'` returns nothing | re-run | ✅ returns nothing (but see Finding 7) |
| 15 | 9 grep matches, `:509`/`:514` bind `..`, 7 value-binding sites `:97,:100,:203,:206,:325,:722,:890` | `grep -n` + `sed` each line | **exact** ✅ |
| 16 | §1.9 P_AMBIGUOUS -> `flag_hearing_impaired:true`, `forced_track:false`, `forced_track:true`; wire `{"value":"true"}` | probe crate, real `plan_batch` | **byte-identical**, incl. JSON ✅ |
| 17 | §1.9 P_SUBS_BY_LANGUAGE -> `codec:SubRip/SRT`, `codec:SubStationAlpha/ASS`, `id:1` | probe crate | **byte-identical** ✅ |
| 18 | §1.6 `from_json(400)=Int(400)`, `from_json(400.0)=Float(400.0)`, `matchable_type(min_luminance)=Float`, `Int(400)!=Float(400.0)` | probe crate | **all four** ✅ |
| 19 | §1.6 yaml `Float(400.0)="400.0\n"`, `Int(400)="400\n"` | probe crate | **exact** ✅ |
| 20 | §1.8 YAML laundering table (3 rows) | probe crate, real loader | **all three rows exact** ✅ |
| 21 | §1.8 `apply_edit_to_first_rule` is a YAML string template | read `tests/suggestions.rs:95-114` | ✅ confirmed |
| 22 | Serde round-trip **8/8** on the proposed internally-tagged + untagged shape | probe crate, exact proposed shape | **8/8** ✅ |
| 23 | `true`->Bool, `"true"`->Str, `3`->Int, `3.0`->Float, `"3"`->Str | probe crate | **all five** ✅ |
| 24 | ts-rs 12.0.1 emits `Scalar = boolean \| number \| number \| string` and the 4-arm `StructuredEdit` | **ran ts-rs 12.0.1 from local cargo cache** with `TS_RS_LARGE_INT="number"` | **byte-for-byte identical**, incl. the duplicate `number` ✅ |
| 25 | `planner.rs` is one file, 2204 lines, no `planner/` dir | `wc -l`, `ls` | **2204** ✅ |
| 26 | `with_rule_match`/`rule_index_of` already `#[doc(hidden)] pub` | read `:1852-1853`, `:2031-2032` | ✅ |
| 27 | `scalar_fits` at `validate.rs:416-425` admits `(Int,Float)` + `(Float,Float)` | read | **exact lines** ✅ |
| 28 | `IpcError::with` takes `impl Into<String>` (`error.rs:60`) | read | ✅ (so the `to_string()` is genuinely required) |
| 29 | `From<SettingsError>` precedent at `error.rs:123-132` | read | **exact** ✅ |
| 30 | `commands/mod.rs:126` prints only `yaml_fragment` | read | ✅ |
| 31 | `src/ipc.ts:132` types `edit: unknown` | read | ✅ |
| 32 | D44 never names `StructuredEdit`; its "20 model types" are unenumerated | `grep -n "StructuredEdit"` over the design doc; D44 spans `:498-684` | ✅ both true |
| 33 | `plan` is a hand-rolled duplicate of `plan_multi` (`:24-41` vs `:117-136`) | read both | ✅ **exact line ranges**, semantically identical |
| 34 | G1 non-vacuity: `checked` reaches 3 | probe | **checked = 3** ✅ |
| 35 | G2 non-vacuity: `checked` reaches 1 | probe | **checked = 1** ✅ |
| 36 | G3 fails loudly on the wrong answer | probe: applied `Scalar::Str(display)` and re-planned | **`MissingTrack` fires** for all 3 Bool edits and for `id` ✅ |
| 37 | JS int/float trap premise (`Float(3.0)` serializes as `3.0`) | probe | `json(Float(3.0)) = 3.0` ✅ premise real, and unreachable via the engine ✅ |

**Caveats on measurements that reproduced with a nuance:**

- **#7 (reachable set closed at three).** Reproduced *for engine-produced edits*.
  It is closed by `prop_value_as`'s total match, and I found **no other
  construction site**, so the claim holds as stated. Note the set is not closed on
  the *inbound* wire: `StructuredEdit` gains `Deserialize` carrying a 4-variant
  `Scalar`, so a `Scalar::Float` is representable from the shell. D49 is aware and
  answers this under rejected alternative (iii); the two statements are consistent
  because the frontend echoes rather than constructs. Flagged only so a later
  reader does not mistake "the engine cannot emit Float" for "Float cannot arrive".
- **#36 (G3 falsifiability).** The wrong-answer probe fires `MissingTrack` for the
  Bool edits and `id`, but **not** for the two `codec` edits - correctly, since
  `Scalar::Str` *is* the right type for a String property. G3 is genuinely
  falsifiable; it is simply the Bool/Int rows that carry it. D49's removal-trigger
  prediction ("If G1, G2 and G3 all fail, they are load-bearing") holds.

**Could not verify:** nothing. I expected ts-rs to be unreachable (no network),
but `ts-rs-12.0.1.crate` and `ts-rs-macros-12.0.1.crate` were present in the local
cargo registry cache, so claim #24 was executed rather than assumed.

**Method note.** All probes ran from a scratch crate outside the repo
(`/tmp/.../d49probe`) depending on `muxsmith-core` by path. The repo tree was not
modified; `git status` shows only the untracked D49 document.

---

## Verified-clean

- **Scope collision with Plan 9: none.** Plan 9's item is a **production** hoist -
  "Hoist the four-copy planning pipeline into a shared `plan_pipeline()` core fn"
  (`ROADMAP.md:114-119`), the CLI/src-tauri duplication, with "the seam INTERFACE
  is this plan's design question (ledger `core-121-planner-seam-and-hoist`)".
  D49's `plan_model` is a **test-local helper** in
  `crates/muxsmith-core/tests/suggestions.rs`, invisible outside that test binary.
  Different layer, different artifact, no pre-emption of Plan 9's design question.
  The naming proximity (`plan_model` / `plan_pipeline`) is the only collision and
  it is cosmetic.
- **"Removes an existing duplicate rather than adding one": true.** `plan`
  (`:24-41`) and `plan_multi` (`:117-136`) are the same body; `plan` writes one
  hardcoded `Show.S01E01.mkv`/`SERIES` pair where `plan_multi` loops. Routing
  `plan` through `plan_multi` is a strict deduplication. D49's line ranges are
  exact.
- **`core-124-error-currency-split`: conformant, and D49 is following a recorded
  pattern rather than inventing one.** The entry names the
  "SettingsError/SaveError/**ApplyError** shape" explicitly and routes operational
  failures to an IpcError code in `gui-common.ftl`. D49's `ApplyError` is a plain
  enum with structured params, no prose, no `Deserialize`, mapped in
  `src-tauri/src/error.rs` - exactly the shape. (Prose defect in the catalog text
  itself: Finding 2.)
- **`core-37-prose-free-core` (count 11): untouched.** `StructuredEdit` gaining
  `Deserialize` does not make `Suggestion` or `DiagCode` deserializable (they are
  separate types with their own derives; `Suggestion` merely *contains* a
  `StructuredEdit`). `DiagCode` stays `Serialize`-only at `report/mod.rs`. The
  `Scalar` wire change opens no door here: `Scalar` is a value type in
  `MatchExpr.exact`, carries no prose, and is already a bidirectional wire type on
  the profile itself (`profile/model.rs` derives both halves). `ApplyError` carries
  a path and two integers - no authored English.
- **`core-90-doc-hidden-pub-test-access` (count 4): correctly not reached.** D49's
  §1.10.3 is right that `with_rule_match` (`:1852-1853`) and `rule_index_of`
  (`:2031-2032`) are *already* `#[doc(hidden)] pub`, and that `apply_suggestion`,
  living in the same 2204-line `planner.rs`, needs neither. `delta_for` stays
  private and is reachable in-module. The open "does it extend to production
  reuse?" question genuinely does not arise. This also correctly retires the plan
  review's "delta_for must be made reachable" concern as a cost of nothing.
- **`core-33-suggestion-narrow-only`: preserved.** `delta_for` emits only
  `exact`/`substring`/`not` additions; `with_rule_match` touches one rule index and
  reorders nothing. Verified by reading both.
- **`core-72-exact-typed-value-equality` (count 3): this is the entry D49 serves.**
  "exact compares each property in its own domain" is exactly what carrying the
  typed `Scalar` preserves end-to-end. The decision is well-aimed at a settled,
  thrice-reinforced house semantic.
- **`proc-proposed-safeguard-stays` (count 1): exemplary compliance.** D49 argues
  none of its five guards out during design. Its "Removal trigger" section does
  precisely what the entry prescribes - keeps the vacuity analysis, re-aims it at
  implementation time, and names a concrete experiment ("change `delta_for`'s
  `AddExact` arm to `Scalar::Str(scalar_display(value))` and run the suite"). I ran
  that experiment's substance (measurement #36) and its prediction holds.
- **`proc-latitude-clause-boundary` (count 5): D49 closes D44's omission
  correctly.** D44 genuinely leaves "the 20 model types" unenumerated in a
  normative position and never names `StructuredEdit` (verified: D44 spans
  `:498-684`, no `StructuredEdit` in range). D49 identifying this and binding the
  derive explicitly is the right move, and is the entry applied as intended. D49
  re-opens the same shape only once, minorly (Finding 5, the unnamed import).
- **Guards G1/G2 fixtures already exist**, as claimed: `P_AMBIGUOUS`
  (`suggestions.rs:17`), `CODEC_ID_ONLY` (`:464`), `P_SUBS_BY_LANGUAGE` (`:481`).
  The sibling `ambiguity_resolvable_only_by_codec_or_id_yields_those_dimensions`
  (`:491`) does assert `has_id` (`:514`) as D49 says.
- **The `AddSubstring`/`AddNotSubstring` asymmetry is correct and correctly
  defended.** `MatchExpr.substring` is `Option<BTreeMap<String, String>>` and
  `MatchExpr.exact` is `Option<BTreeMap<String, Scalar>>`; each variant carrying its
  target map's type is right, and D49 is right to fence it off from the implementer.
- **Wire-ripple consumer hunt: D49's enumeration is complete** (modulo the inert
  e2e mock, Finding 7). I searched insta snapshots, e2e mocks/specs, the Vue tree,
  CLI tests, and the JSON report. No snapshot, fixture, assertion or type-check
  sees a suggestion `value`. `pnpm build` cannot see the change
  (`edit: unknown`). The "observable output change ... no test asserts it" premise
  runs clean.
- **The JS int/float trap analysis is honest.** The premise is real (`Float(3.0)`
  serializes as `3.0`, verified), and the "cannot fire" rests on the same total
  match §1.4 rests on - which I verified has no other construction site. D49
  correctly names where the guard would become owed (the day `prop_value_as` gets a
  `Float` arm) rather than pretending the hazard does not exist. This is a
  no-work-needed conclusion that survives its own premise being run.
- **Rejected alternative (iii) (`EditValue{Bool,Int,Str}`) is honestly steelmanned
  and soundly rejected.** The steelman states the unrepresentable-state argument at
  full strength; the rebuttal (an `EditValue -> Scalar` conversion is the same
  reconstruction site with a narrower input, plus a duplicated `#[serde(untagged)]`
  ordering) is correct - `MatchExpr.exact` does hold `Scalar`, so the conversion is
  unavoidable.
- **Rejected alternative "widen `with_rule_match`" is honestly steelmanned and
  soundly rejected.** "Would break all five" is exact: 4 production sites hold a
  `cand.apply` `MatchExpr`, and `prop_planner.rs:310` passes a `doc.match_expr` with
  no edit behind it. Verified by reading all five.
- **§1.10's corrections to the plan review are accurate**, including the
  single-file-module point (#1) and the `#[doc(hidden)] pub` omission (#3).
- **§1.5's `codec_kind` claim is correct**: it resolves in `matchable_type`
  (hardcoded `String`) but is never written into a `Track`'s properties - the only
  reads are `matcher.rs:120-124`'s alias resolution. It cannot become a candidate.

---

## HARVEST

Patterns and repeated rejections observed. Reported only; I have written nothing
to the ledger or the convention files.

1. **The dominant pattern is a good one, and it should be named: D49 is the
   highest-fidelity design document I have graded in this repo.** 37 empirical
   claims, every single one reproduced, several to the byte (the ts-rs output, the
   §1.9 JSON, the YAML laundering table, the 9/5/27/21 split, seven test-site line
   numbers). The house's "measure, don't assert" discipline is fully internalized
   here. **Every defect I found is in prose the document wrote *around* its
   measurements, never in a measurement.** That is a strong signal about where
   review effort pays: the measured core needed no correction; the connective
   argument needed four.

2. **Repeated rejection: the "no-work-needed" shape keeps being the right call
   here.** D49 makes the move three times (the `core-90` question is not reached;
   the JS trap cannot fire; no test asserts the wire) and **all three survive
   running the premise**. Two of the three, though, rest on evidence that is
   narrower than the claim (Finding 7's grep; Finding 5's absent precedent). The
   generalizable lesson is not "distrust no-work-needed" - `proc-no-work-needed-check`
   already handles that - but: **when the conclusion is "nothing to do", the
   *method* still has to be sound, because nobody re-runs a negative.** A grep that
   returns nothing looks identical whether it was the right grep or the wrong one.

3. **A recurring near-miss: a document that correctly diagnoses a type confusion
   can still commit it in its own prose.** Finding 1 is D49 reconstructing
   `Scalar::Str("true")` from a display string for a Boolean property - in the very
   paragraph fixing the tests for an ADR whose thesis is "`Bool(true)` and
   `Str("true")` both project to `"true"`, you cannot invert that." This is the
   third instance of this shape I can see recorded in this repo's own files
   (`feedback_regel_braucht_ausloeser_und_handgriff`'s cited 2026-07-15 episode: an
   agent diagnosed a failure pattern in writing and produced its third instance in
   the same document; `process-conventions.yaml:334`: a second omission instance in
   the same approved design document, past four review rounds). **Awareness of a
   defect class demonstrably does not confer immunity to it inside the same
   artifact.** If a rule is wanted, the only shape that works is a mechanical one
   with a readable trigger - e.g. *"a design document that changes a type's variant
   must state the per-site literal for every site it touches, never 'the same shape
   as above'"* - because "be careful about type confusion" is exactly the
   noticing-based rule that fails.

4. **Repeated rejection worth recording: "the same shape as X" and "whichever
   literal they already name" are latitude by omission, even when the set is tiny
   and mechanical.** D49 enumerates 4 template sites and then collapses 3
   comparison sites into a pattern - and the collapse is where the defect landed.
   The set was **three**. The document that writes out four `.ftl` lines "so none
   is invented" declined to write out three literals. `proc-latitude-clause-boundary`
   (count 5) names the test exactly ("must the implementer invent something it is
   not allowed to invent?"); this is a case where the answer is subtle - the
   implementer does not *invent* the literal, they *derive* it - and derivation is
   where the type error hides. **Candidate sharpening: an unenumerated set in a
   normative position is latitude even when its members are derivable, because a
   derivable member is exactly what a wrong derivation produces.**

5. **A structural observation, not a defect: `catalog_completeness.rs` gates
   `DiagCode -> cli.ftl` exhaustively, but nothing gates `IpcError` codes ->
   `gui-common.ftl`.** `check-i18n.mjs` explicitly downgrades the IpcError case to a
   warning ("Known residual false positive, accepted because this half is a
   warning"), and `core-124`'s "every new code forces new bilingual prose" is
   therefore enforced by convention for IPC codes, not by a gate. D49 complies
   voluntarily and completely (four lines, both locales). Findings 2's plural gap is
   the visible cost of that asymmetry: a `DiagCode` with a plural param has a wired
   promotion path; an `IpcError` with one does not. Routing note: this is a
   standing-structure observation about the house's gates, above D49's pay grade,
   and it is the owner's call whether it is worth a gate.

---

## Out of range

Neither item affects the verdict; both are D43/D44 territory, recorded so they are
not lost.

1. **D44 leaves "the 20 model types" unenumerated** in a normative position
   (`2026-07-15-plan-6-design.md:509`, `:548`), which is a
   `proc-latitude-clause-boundary` omission in an ADR that is already approved and
   settled. D49 detected it and worked around it correctly for its own type. Any
   other type reaching that wire has the same hole, and the implementer of D44 will
   face it. Not D49's to fix.
2. **D43's ruling that `apply_suggestion` neither validates nor re-plans** is what
   makes Finding 3's silent no-op unobservable. I am not challenging the ruling -
   it is settled and its rationale (no full re-identification of the batch to
   recompute a held suggestion) is sound. Finding 3 targets only D49's own
   exhaustiveness claim on top of it. If the owner would rather close the no-op
   than document it, that decision reaches back into D43's shape and should be
   routed as such rather than folded into D49.

---
---

# Round 2 — 2026-07-16 — judging the delta

Same judge, same standards. Round-1 non-findings stay settled and were not
re-litigated; what follows grades only what changed. Tree state: HEAD `36bfee5`,
unchanged since round 1, so every round-1 measurement still stands without re-run.

## VERDICT

**APPROVED**, with two Minor prose corrections to apply before implementation
(both fully specified below — nothing to invent, no third review round warranted).

All seven round-1 findings are **fixed**, four of them better than the minimum the
finding asked for. The owner-ruled third exit is correctly designed, correctly
scoped, and its measurements reproduce byte-for-byte. Two new Minor defects
entered in the fix round; neither touches the design, the decision, the guards, or
any measurement. Both are single-line prose corrections.

Severity counts (new this round): **Critical 0 | Important 0 | Minor 2.**

## Per-finding disposition

| # | Round-1 finding | Disposition |
|---|---|---|
| F1 | `:722` literal wrong | **Fixed** (exceeds ask) |
| F2 | plural in `apply-rule-index-out-of-range` | **Fixed** |
| F3 | "only two failures" omits silent no-op | **Fixed** (owner-ruled; exceeds ask) |
| F4 | option (i) rejection overstated | **Fixed** (exceeds ask) |
| F5 | `ts_rs::TS` import by omission | **Fixed** (exceeds ask) |
| F6 | `report/json.rs` crate attribution | **Fixed** |
| F7 | grep method unsound | **Fixed** |

**F1 — fixed, and the fix is right where a second wrong derivation would have
hidden.** The prose instruction is gone; `:1289-1293` is now a table with each
property's declared type named. I checked all three literals individually against
`generated.rs`, which is the only way this finding could be closed:

| site | property | D49 says | tree says | correct? |
|---|---|---|---|---|
| `:325` | `track_name` | `String` (`generated.rs:66`) -> `&Scalar::Str("Chapter 1: Intro".to_string())` | `66: ("track_name", PropType::String)` | ✅ |
| `:722` | `forced_track` | **`Boolean`** (`generated.rs:41`) -> `&Scalar::Bool(true)` | `41: ("forced_track", PropType::Boolean)` | ✅ |
| `:890` | `language` | `String` (`generated.rs:43`) -> `&Scalar::Str("eng".to_string())` | `43: ("language", PropType::String)` | ✅ |

All three now individually correct. The document additionally states the trap
rather than just avoiding it (`:1284-1287`: "They are **not** the same shape, and
deriving them from the old string is exactly the mistake this ADR exists to
prevent"), records that the wrong `:722` literal **compiles** and fails only at
runtime, and records `:890`'s vacuity hazard with the range cited — I re-read
`:887-893` and it is exactly the `!...any(...)` negative assertion claimed. That
last point is the one I raised as a secondary hazard; it was picked up in full.

**F2 — fixed.** `(rule count: { $rules })` / `(Regelanzahl: { $rules })`. No
counted noun, so no agreement to get wrong, in either locale. `apply-edit-changed-nothing`
likewise uses "rule { $index }" / "Regel { $index }" — singular by construction.
No selector added, no promotion mechanism touched, and `:648-659` records *why* a
selector must not be added (params reach Fluent as strings at every IpcError call
site; `diagnosticFluentParams` is keyed by diagnostic code) with the structural gap
routed to ROADMAP rather than silently absorbed. That is the right disposal.

Judged as prose, both locales: the German is grammatical (`schränkt ... bereits
ein` splits the separable verb correctly; `Regelanzahl` is a well-formed compound),
matches the existing `gui-common.ftl` de register (`Der Vorschlag konnte nicht
angewendet werden: ...` mirrors `Die Anwendungseinstellungen konnten nicht gelesen
oder geschrieben werden: { $detail }`), and honours the de catalog header's own
rule that "placeables and selector structure mirror" the en. The labelled-value
parenthetical is faintly terse but it is a structured-data error message and the en
does the same, so the locales stay parallel. No du-imperative is owed — these are
declaratives, not instructions. The `P_ALREADY`/`P_AMBIGUOUS` prose slip is
corrected: `:647` now reads "it loads `P_AMBIGUOUS`, annotated `// exactly 1 rule`",
which matches the test body at `:1047`.

**F3 — fixed, and the design is sound.** I verified the reasoning and the counts
myself rather than accepting the report:

- **`Profile` derives `PartialEq`** — `profile/model.rs:17` reads
  `#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]`. The
  comparison needs no new impl, as claimed. ✅
- **The measurement block (`:570-575`) reproduces byte-for-byte.** Ran the real
  `with_rule_match` against D49's verbatim `P_ALREADY_CONSTRAINED`:
  ```
  no-op case (forced_track already true)   applied == profile ? true
      rule match after: Some({"forced_track": Bool(true), "type": Str("subtitles")})
  control (unconstrained default_track)    applied == profile ? false
  not-append (AddNotExact)                 applied == profile ? false
  ```
  Identical to the document, including the `rule match after` line. ✅
- **The not-variant reasoning is correct, and I tried to break it.** D49 claims the
  two `not`-appending variants can never reach the exit because `extend` is always
  additive. The adversarial case is an `AddNotExact` onto an rule that *already*
  carries an identical not-clause — if `extend` deduplicated, that would no-op. It
  does not: **not-list len before/after: 1 / 2**, `applied == profile ? false`. The
  duplicate is appended, so the model always changes. `AddNotSubstring`: `false`
  likewise. The claim holds where I attacked it. ✅
- **The scoping is right, not merely lucky.** D49 excludes *only* the two `not`
  variants — it does **not** claim `AddSubstring` is safe. I checked whether that
  exclusion is exactly right: `AddSubstring` onto a rule with an existing
  `substring.track_name` yields `applied == profile ? true`. So `AddSubstring`
  **can** reach the exit, and D49 correctly did not exclude it. A sloppier version
  of this fix would have said "only the exact variant can no-op" and been wrong. ✅
- **The tree's own comment independently backs the reasoning.** I suspected
  `:1845-1848` was a mis-citation (it sits *before* `with_rule_match`, so I expected
  it could not be `resolves_without_regression`'s comment). I was wrong: it reads
  *"which `resolves_without_regression` correctly rejects for not resolving the
  ambiguity. `not` entries are always additive (appending a not-clause always
  narrows, never relaxes), so plain `extend` stays correct there."* The citation is
  exact, and that comment corroborates both halves of D49's argument. ✅
- **Counts.** `Exactly three variants ... exactly three exits` (`:562`) ✅;
  `ApplyError` "with its three variants" (`:1142`) ✅; "Seven tests" (`:801`) ✅ —
  G1, G2, G3, three `ApplyError` tests, one control = 7, and all seven bodies are
  specified; "none of these seven" (`:1122`) ✅. One stale count found: see N1.
- **The control test is the right call and was added unprompted.** Its stated
  rationale ("Without this, G4 would pass against an `apply_suggestion` that
  returned `EditChangedNothing` unconditionally") is exactly correct, and my probe
  confirms it discriminates (`default_track` unconstrained -> `false` -> `Ok`).
  A guard that cannot fail is worse than none; so is a guard whose complement
  cannot fail. Adding it without being asked is the behaviour the house wants.

**F4 — fixed, and the rejection is now *stronger*, not weaker.** The "exactly the
core-03 violation" sentence is gone. `:716-725` now states the scope precisely:
"a break in **delta identity and rendering, not in matching**", cites
`matcher.rs:207`, `:209` for the `(Int, Float)`/`(Float, Int)` arms — I re-read
both; `:207` is `(Scalar::Int(a), PropValue::Float(b))` and `:209` is
`(Scalar::Float(a), PropValue::Int(b))`, exact — and concedes outright that the one
case which *does* break matching is one option (i) gets right.

Judged on the coordinator's question — does the rejection still stand after losing
its load-bearing sentence? **Yes, and it stands better.** It now rests on two
independent legs, both of which I verified in round 1: (a) §1.6, the mechanism
option (i) proposes does not exist as a function, because `PropType::Float` admits
two `Scalar` variants per the house's own `scalar_fits` (`validate.rs:416-425`) —
so the parse must *guess*, and the guess silently changes what the user's profile
says (`400.0` where the engine rendered `400`; measured); and (b) cost — a new parse
function, a new failure mode, a new inversion guard, versus option (ii) which
deletes the projection instead of inverting it. Neither leg depended on the cut
sentence. The document is now un-re-litigable on this point, which was the whole
purpose of the finding: the sentence that would have collapsed under one command is
gone, and its replacement states its own limit.

**F5 — fixed, thoroughly.** `:392-409` mandates
`#[cfg(feature = "ts")] use ts_rs::TS;`, states the `cfg` is "**not** optional",
gives the clippy consequence with the gate cited (`.github/workflows/ci.yml:86`),
and adds an explicit **"D49 cannot land before D44"** with the reason (needs the
`ts` feature, the optional dep, the `[features]` section, and `Scalar: TS`). That
sequencing note is more than the finding asked for and closes the coordination gap
I raised as a secondary. Its supporting grep claim re-runs clean: nothing matching
`ts_rs|ts-rs` in any `*.toml`/`*.rs` outside `target/`. (The command as written in
the document globs under zsh because `--include=*.toml` is unquoted; quoting it
reproduces the claimed result. Not worth a finding — the claim is true and the
document is not a shell script.)

**F6 — fixed.** Both sites now name it correctly: `:230` reads "`dry-run --json`
report, built in **core** (`crates/muxsmith-core/src/report/json.rs:56`), consumed
by the CLI", and `:1162-1167` says "The document is built **in core** ... the CLI is
its consumer, not its author."

**F7 — fixed, method and conclusion both.** Re-ran both greps myself:
- `grep -rn "edit" --include="*.ts" --include="*.vue" --include="*.json" .` minus
  `node_modules`, filtered to `edit`-as-a-key -> **2 hits**: `e2e/smoke.spec.ts:177`
  (`edit: null,`) and `src/ipc.ts:132` (`edit: unknown;`). Exactly as claimed.
- `grep -rn '\["edit"\]\|"edit"' --include="*.rs" crates/ src-tauri/` -> **0 hits**.
  Exactly as claimed.

`:1169-1183` keeps the conclusion, names the superseded grep as unable to have
found the hits, and both TS hits are correctly explained as inert. The author's
reported diagnostic path (its `-E` retry returned nothing while `sed` showed the
hit, so it distrusted *both* and rebuilt the pattern) is the right instinct and
matches what I found independently. Worth noting: the fixed method is one that
*could* have failed and did return results — which is the property a negative
claim needs.

## Ruling: the `index` + `property` params on `EditChangedNothing`

**Earned. Keep them.** This was the author's addition, not the ruling's, and it was
surfaced rather than hidden — which is itself the correct handling. On the merits:

1. **The ruling's intent requires them.** Şenol ruled the no-op is *detected*, not
   merely documented. A bare code renders "The suggestion changed nothing." — that
   detects and informs nobody; the user cannot act on it. Naming the blocking
   constraint ("rule 0 already constrains forced_track") is what converts detection
   into something the user can resolve. Params are what make the ruling mean
   anything.
2. **Nothing is invented.** `index` is already in hand (the checked `rule_index_of`
   result, two lines up); `property` comes from `edit_key`, a total match over a
   closed four-variant grammar. Both are facts the function already holds.
   `proc-latitude-clause-boundary`'s test — "must the implementer invent something
   it is not allowed to invent?" — answers no.
3. **`core-37` is not touched.** A property name is a structured param, not prose.
   This is exactly the house's own `unknown-property` / `value-type-mismatch`
   shape, which interpolate `$property` for the same purpose.
4. **The params reach the catalog correctly** — `IpcError::with` takes
   `impl Into<String>` (`error.rs:60`), hence `index.to_string()` and `property`
   passed through; the `From<ApplyError>` arm at `:609-613` matches the
   `From<SettingsError>` precedent exactly. One defect in the *rendering*, not the
   params: see N2.
5. **`property` is accurate for every variant that can reach the exit** — the
   coordinator's specific question. I tested it rather than reasoned it:
   `AddSubstring` onto a rule already carrying `substring.track_name` no-ops, and
   `edit_key` returns `"track_name"`, which **is** the key that blocked it
   (`delta_for` inserts literally `"track_name"` into `m.substring`,
   `planner.rs:1824`/`:1829`). The message is true for that case. The two `not`
   variants never reach the exit, so their `edit_key` arm is unreachable in
   practice — dead but harmless, and keeping `edit_key` total is right, not padding.

**One imprecision I accept rather than flag.** `track_name` is *also* a matchable
property (`generated.rs:66`), so an `AddExact { property: "track_name" }` blocked on
`exact.track_name` and an `AddSubstring` blocked on `substring.track_name` both
render "already constrains track_name" — the message names the key, not the arm. In
the editor the rule is on screen, so the user can see which arm; naming the arm
would cost a third param for little gain. Acceptable as designed. Recording it only
so a future reader does not mistake it for an oversight.

## New findings

### N1. (Minor) The catalog's own count did not follow the third variant through

**Where:** `:624-625`.

> `core-124` records that every new code forces new bilingual prose. **Two codes ×
> two locales = four lines**, written out here so none is invented:

**What is wrong.** The block immediately below it now contains **three** codes and
**six** lines. The sentence is the pre-F3 count, left behind when the third variant
was added directly underneath it.

**Evidence.** `awk 'NR>=627 && NR<=641 && /^apply-/' ... | wc -l` -> **6**. The en
block (`:629-633`) has `apply-unparsable-config-path`, `apply-rule-index-out-of-range`,
`apply-edit-changed-nothing`; the de block (`:637-641`) has the same three. Three
codes × two locales = six lines.

I swept every other numeric claim in the document (`grep -n "Two codes\|four lines\|Seven tests\|these seven\|three variants\|three exits\|Exactly ...\|7 sites\|9 matches"`): **this is the only stale one.** "Seven tests", "three variants", "three exits", "Exactly four edits", "7 sites", "9 matches", "two places" all check out.

**Second instance of the same shape, same cause:** `:1026`'s heading reads
"### Three `ApplyError` tests" but the section contains **two** test bodies; the
third (`apply_rejects_an_edit_the_no_clobber_merge_drops`) lives under its own
"### G4" heading at `:1080`. The claim at `:1028` ("`ApplyError`'s three variants
are each reachable and each asserted") is **true** — all three are asserted — and
`:801`'s "Seven tests" accounts correctly. Only the section heading over-promises
its own contents. Trivial, but it is the same "the third variant propagated into
the substance but not into the surrounding prose" pattern.

**Fix.** `:624-625` -> "Three codes × two locales = six lines". Optionally retitle
`:1026` to "Two of the three `ApplyError` tests (the third is G4, below)", or move
G4's test body under it.

### N2. (Minor) `{ $property }` is interpolated bare; every house message quotes it — 16 of 16

**Where:** `:632` (en) and `:640` (de).

```
apply-edit-changed-nothing = The suggestion changed nothing: rule { $index } already constrains { $property }.
apply-edit-changed-nothing = Der Vorschlag hat nichts geändert: Regel { $index } schränkt { $property } bereits ein.
```

**What is wrong.** The house wraps an interpolated property name in straight double
quotes, without exception. Counted, not estimated:

```
quoted:   16
total:    16
```

Every occurrence of `{ $property }` across `locales/en/*.ftl` and `locales/de/*.ftl`
is `"{ $property }"` — `unknown-property`, `raw-property`, `raw-on-known-property`,
`not-string-property`, `value-type-mismatch`, `unknown-settable-property`,
`unknown-property-skew`, `invalid-property-value`, in both locales.

D49's new line is the only bare one, and it is **internally inconsistent with its
own sibling**: `apply-unparsable-config-path` (`:630`, `:638`) quotes `"{ $path }"`
one line above. `conventions.yaml`'s framing applies directly — a lone deviation is
the outlier, not a new style.

This is not only cosmetic. Unquoted, a config identifier dissolves into the running
prose, and worst in German: *"Regel 0 schränkt forced_track bereits ein."* gives the
reader no boundary for the identifier. That is precisely why the house quotes it
16/16.

**Reaches the tree.** D49 instructs that these lines are "written out here so none
is invented", so an implementer copies them verbatim and ships the deviation.

**Fix.** `"{ $property }"` in both locales:
```
apply-edit-changed-nothing = The suggestion changed nothing: rule { $index } already constrains "{ $property }".
apply-edit-changed-nothing = Der Vorschlag hat nichts geändert: Regel { $index } schränkt "{ $property }" bereits ein.
```

## Nothing regressed against the round-1 clean list

Checked each item I cleared in round 1 against what the delta touched:

- **`core-124`** — the third variant follows the same shape (plain enum, structured
  params, no prose, no `Deserialize`, mapped in `src-tauri/src/error.rs`). The entry
  names the "SettingsError/SaveError/**ApplyError** shape" and this still is one.
  Strengthened, not regressed: the new variant is the clearest case yet of an
  operational failure that is not a `Diagnostic`. ✅
- **`core-37`** — untouched. `ApplyError` gains a variant carrying a `usize` and a
  property name; no authored prose in core, no new `Deserialize`, `DiagCode` still
  `Serialize`-only. `edit_key` returns a `&str` that is either a caller-supplied
  property name or the literal `"track_name"` — a config key, not prose. ✅
- **`core-44`** — not merely intact but now *surfaced*. D49 previously let
  `or_insert` drop a delta silently; it now detects it and cites the semantic. The
  merge itself is called unchanged. ✅
- **`core-33`** — unchanged; `apply_suggestion` still narrows one rule index and
  the new exit only *rejects*, never widens. Erroring on a no-op cannot relax
  anything. ✅
- **`core-72`** — unchanged; the F1 table now makes the typed-equality semantic
  explicit at each test site. ✅
- **Plan 9 non-collision** — the delta added `P_ALREADY_CONSTRAINED` and two tests
  to `tests/suggestions.rs`; `plan_model` is unchanged from round 1. Still a
  test-local helper, still no contact with Plan 9's production `plan_pipeline()`
  hoist. ✅
- **The (iii) and widen rejections** — textually unchanged (`:743-790`), still
  sound. ✅
- **`proc-proposed-safeguard-stays`** — the removal trigger was updated to "none of
  these seven" and still argues nothing out during design, still names the
  implementation-time experiment. ✅

One interaction worth naming as *improved*: my round-1 "Out of range" item said
closing the no-op would reach back into D43's shape and should be routed rather
than folded in. It **was** routed — to Şenol, who ruled — and the resolution
respects D43's boundary exactly: no re-plan, no batch, no validation, just one
model comparison the function already had the operands for. That is the routing
`proc-latitude-clause-boundary` prescribes, executed properly.

## HARVEST — updated by the delta

Round 1's harvest items 1-5 stand. The delta sharpens two and adds one.

1. **Item 3 gains its clearest instance yet, and it is now a *fix-round* instance.**
   Round 1 recorded: a document that correctly diagnoses a defect class still
   commits it in its own prose. The delta shows the *counting* version of the same
   thing: the author added a third `ApplyError` variant and propagated it correctly
   into the enum, the shell mapping, the catalog block, the tests, the test count
   ("Seven"), the variant count ("three"), and the exit count ("three exits") — and
   missed it in the one sentence sitting directly above the block that grew (N1),
   plus the heading directly above the tests that grew. **The substance propagated;
   the prose around it did not.** This is now three rounds of the same pattern
   (round-1 F1-F4 were all prose-around-measurements; N1/N2 are too). The
   generalizable form: *when a change adds a member to an enumerated set, the counts
   and headings describing that set are the highest-risk text in the document, and
   they are exactly the text nobody re-reads because it is not where the work
   happened.* A mechanical trigger exists for it — "if you added a member to a set,
   grep the document for every numeral and count-word describing that set" — which
   satisfies the trigger-and-handle test rather than asking anyone to *notice*.

2. **Item 2 is confirmed from the other side.** Round 1 said: nobody re-runs a
   negative, so the *method* must be sound even when the conclusion is "nothing to
   do". The author's reported F7 path is the healthy version — its retry disagreed
   with `sed`, and rather than trusting either it diagnosed its own pattern as
   broken. That is the correct response to two instruments disagreeing, and it is
   worth recording as the positive exemplar next to the negative one.

3. **New: a fix that concedes ground is stronger than one that holds it.** F4 asked
   the author to *weaken* its own decisive claim. The rewritten rejection explicitly
   states its limit ("delta identity and rendering, not in matching") and concedes
   that the rejected option gets the Bool set right — and the rejection is now
   harder to overturn than before, because the sentence that would have collapsed
   under one command is gone. Recording this because the instinct under review
   pressure is to defend the strongest-sounding claim; the delta demonstrates that
   *scoping* a claim to what was measured is what makes it survive. This is the same
   asymmetry `proc-proposed-safeguard-stays` names from the other direction: an
   overstated argument costs a re-litigation later, an accurately-scoped one costs
   a sentence now.

4. **Not a pattern, an observation worth one line:** the author added a control test
   nobody asked for, on the reasoning that G4 alone would pass against an
   unconditional thrower. That is a reviewer's instinct applied to its own work, and
   it is the difference between a guard and a guard that means something. The house
   already has `proc-proposed-safeguard-stays` for *keeping* proposed guards; there
   is no entry for *complementing* a guard with its control. Whether that deserves
   one is the owner's call — I only note that it was done unprompted and that it was
   correct.
