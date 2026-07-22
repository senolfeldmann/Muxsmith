# Task 2 verdict - D63 CLI multilingual rendering (plan7-a, commit d6d20ad)

**VERDICT: APPROVED**

Independent reviewer grade against plan Task 2 (`2026-07-21-plan-7-help-i18n.md`
:192-377) + Global Constraints, design D63 (round-5 committed) + amendment 3
(section 6), the Tier-2 `cli-multilang-rendering` boundary entry
(`product-boundaries.yaml:449-462`), and the real tree with installed crates.
Every implementer claim re-verified foreground in the worktree; the one probe
that touched a file was restored byte-identically (cmp + sha256), tree clean at
exit. Implementer verdict DONE_WITH_CONCERNS; the single concern (Step 4 grep
substitution) is adjudicated in Q1 and upheld.

## Summary of verification

| Claim | Re-verified | Result |
|---|---|---|
| Four enumerated tests green | `cargo test -p muxsmith-cli --lib i18n` | 16/16 lib tests pass incl. `de_request_renders_de_message`, `message_missing_in_requested_locale_falls_back_per_message`, `region_qualified_de_resolves_de_row`, `unknown_tag_renders_en_chain` |
| Full CLI suite green, zero snapshot churn | `cargo test -p muxsmith-cli` | 9 test binaries all `ok`; no `.snap.new`; `git status` clean |
| Single-file diff | `git diff --stat a8c5951 d6d20ad` | only `crates/muxsmith-cli/src/i18n.rs` (+105 -27) |
| No new dependency | Cargo.toml/Cargo.lock unchanged in diff; `sys-locale`/`unic-langid` already present | confirmed |
| `Renderer::new` signature unchanged | `pub fn new(locale: Option<&str>) -> Renderer` | unchanged |
| fmt / clippy / doc clean | `cargo fmt --all --check`; `cargo clippy -p muxsmith-cli --all-targets -D warnings`; `RUSTDOCFLAGS="-D warnings" cargo doc -p muxsmith-cli --no-deps` | all exit 0, zero warnings |
| Commit discipline | `git show`; `git log -1 --format=%G?` | one commit, only i18n.rs staged, message matches plan Step 7 verbatim + trailer, unsigned (`N`) |

## Dimension 1 - spec / boundary compliance: PASS

- **Embed table** (`i18n.rs:16-19`): exactly two locales x two catalogs,
  `[("en", EN_CLI, EN_DIAGNOSTICS), ("de", DE_CLI, DE_DIAGNOSTICS)]`; the de pair
  is `locales/de/{cli,diagnostics}.ftl` (both exist, parity-gated). Matches D63
  and the plan Step 3 block character-for-character.
- **Resolution order** (`new`, :40-43): `locale.map(...).or_else(sys_locale::get_locale).unwrap_or_else("en")`
  = explicit `--locale` > system locale > en. Binding order per the boundary entry.
- **Chain `[primary-subtag, en]` deduplicated** (:46-48): `vec![primary, "en"]`
  then `Vec::dedup()`. Adjacent dedup is correct and sufficient here: the vec is
  always length 2, so the only duplicate producible is `["en","en"]` (request en)
  -> `["en"]`; `de` -> `["de","en"]` (2 bundles); unknown `fr` -> `["fr","en"]`,
  the fr row is absent so the loop skips it (:51-56) and one en bundle results.
  So "one bundle when en or unknown, two when de" holds, and the inline comment
  is accurate.
- **Per-message fallback de -> en -> raw id** (`render`, :108-124): walks
  `self.bundles` in order, returns the first bundle whose message has a value
  pattern; exhausted -> `id.to_string()`. Verbatim the boundary entry's chain.
- **Primary-subtag collapse** (:46): `langid.language.as_str()` (the Rust mirror
  of the frontend `primarySubtag`); `de-DE`/`de-AT` collapse to the `de` row.
- No new dependency, single-file diff, signature unchanged: all confirmed above.

## Dimension 2 - the D63 rejected-alternative property: PASS (probed)

The two-bundle chain exists so an en fallback message pluralizes under **EN** CLDR
rules, not the requested locale's. The implementation delivers this structurally:
each bundle is `FluentBundle::new(vec![row_langid])` where `row_langid =
row_tag.parse()` (the **row's own** tag, :57-59), not the requested langid; and
`render` formats each message with the **owning** bundle (:117-119). The rejected
single-bundle alternative would have carried one requested-langid set with de
overriding en.

Because only en/de are embedded and they share identical one/other categories,
the divergence is not observable through `Renderer::new` today (a `ru` request
collapses to no row -> en-only chain). To prove the guarantee is non-vacuous I
built a falsifiable probe (temporary test, restored byte-identically): a chain
`[ru-bundle-without-msg, en-bundle-with-plural-msg]` where the message has
`[one]/[few]/[many]/*[other]` variants.

- Property arm: `msg_with_counts("m", n=2)` -> **"OTHER"** (en fallback formats
  under EN rules; n=2 is `other` in English). The rejected single-`ru`-bundle
  alternative would print `"FEW"` here.
- Control arm (fire-verification): same message **owned** by a `ru`-langid bundle
  -> **"FEW"** (n=2 is `few` in Russian). This proves per-bundle langid actually
  changes CLDR selection, so the property arm's "OTHER" is a meaningful pass, not
  a check that cannot fail.

Both assertions passed. The mechanism is correct and its guarantee has teeth.

## Dimension 3 - behavioral verification (run against the real binary): PASS

Four enumerated tests + full suite re-run above. Real-binary probes on
`muxsmith validate <good.yaml>` (built `target/debug/muxsmith`), each with a
control:

| Invocation | Output | Expected |
|---|---|---|
| `--locale de` | `Das Profil ist gültig.` | German (de catalog) |
| `--locale de-DE` | `Das Profil ist gültig.` | collapses to de |
| `--locale fr` (unknown) | `Profile is valid.` | en chain |
| `--locale en` (control) | `Profile is valid.` | English |
| `LANG=de_DE.UTF-8`, no `--locale` | `Das Profil ist gültig.` | system-locale picks de (LIVE) |
| `LANG=en_US.UTF-8`, no `--locale` (control) | `Profile is valid.` | system-locale picks en |

The last pair is the key result: system-locale rendering is now **live behavior**
on Linux (`sys-locale` reads `LANG`), and flipping `LANG` flips the output - so
the control fire-verifies the de result is a genuine host-locale pickup, not a
fixed default. The full CLI suite nonetheless stays deterministic (green, zero
snapshot churn) **precisely because** T1's D64 funnel appends `--locale en` to
every output-asserting invocation; the de embed going live did not break a single
test. The de catalogs also parse at runtime (the binary constructed the de bundle
under `--locale de` without hitting the `expect("embedded catalog must parse")`
panic, exercising both de resources).

## Dimension 4 - quality: PASS

- **Synthetic-bundle test mechanism sound.** `bundle_from` (:257-263) builds a
  single-locale bundle from inline FTL through the private `bundles` field, so
  `message_missing_in_requested_locale_falls_back_per_message` can exercise an
  en-only id (`only-en`) that the real catalogs forbid (check-i18n cross-locale
  id parity). Correct rationale, minimal, and the only test that reaches for the
  private field - the three behavioral tests use the real `Renderer::new` path.
- **Rustdoc claims now true (amendment 3).** Module doc (:21-26) and `new` doc
  (:32-38) describe the D63 reality: embeds en+de, per-message fallback chain
  (requested -> en -> raw id), resolution `--locale` > system > en, per-bundle
  langid for CLDR. Whole-crate grep for a stale claim (`ships English content
  only|ships English only|English content only`) over `crates/muxsmith-cli/src/`
  is clean (exit 1). The **only** two "English" hits in the crate src are the
  Step 1 fixture strings (`only-en = English only` and its assertion, :274/:279),
  both inside `#[cfg(test)]` - test data, not renderer rustdoc. See Q1 for the
  fixture-string collision this creates with the plan's literal grep.
- **House conformance.** Mirrors the frontend `buildBundles` mental model as D63
  directs; `set_use_isolating(false)` preserved for grep-able output; ASCII
  typography in all new comments/docs; `#![deny(missing_docs)]` satisfied (doc
  gate clean). `msg`/`msg_with_counts`/`diagnostic*`/numeric-param promotion left
  untouched above the lookup, exactly as the plan scoped.
- **Commit discipline.** Single commit, only i18n.rs staged, message = plan Step 7
  verbatim, `Co-Authored-By` trailer present, unsigned.

## Dimension 5 - Q1 adjudication: substitution UPHELD (within scope)

**The deviation.** Plan Step 4's literal verification command is
`grep -n "English content only\|English only" crates/muxsmith-cli/src/i18n.rs`,
expected empty. On the correctly-rewritten tree it returns **2 hits** - but both
are Step 1's own fixture strings (`only-en = English only`, :274/:279), not
rustdoc survivors. The implementer substituted a sharper pattern
`ships English content only|ships English only`, verified no-match on the tree,
and fire-verified it against the pre-Task-2 HEAD.

Re-verified here (fire-verified both directions):
- Plan literal grep on current tree: **2 hits**, both the Step 1 fixtures (RED
  that can never clear - the collision).
- Implementer's sharper pattern on current tree: **no match** (exit 1).
- Implementer's sharper pattern against `HEAD~1` (a8c5951, pre-Task-2): prints
  the **two old doc lines** (:12 module doc "v1 ships English content only", :19
  `new` doc "v1 ships English only"). RED state reachable -> the empty result is
  meaningful. Both real survivors are caught; the fixture is excluded.

**Argument it should have round-tripped (NEEDS_CONTEXT).** Global Constraint 16
closes every fork and forbids keyboard resolution; the plan is the contract and
it gave a *literal* command. A strict reading says any deviation from plan text -
even a fix - is a keyboard choice, and the honest move on finding a defective
plan step is to surface it and let the controller re-issue, because a verification
command is part of the evidence standard by which the task is proven to land.

**Argument it was within scope (a verification-method fix).** Global Constraint 16
and design section 9 govern **design latitude** - scope, mechanics, user-visible
behavior, unenumerated normative sets. The substitution touches **none** of these:
the rustdoc end-state is byte-for-byte what the plan mandated, and nothing about
the embed table, resolution order, fallback chain, or output changed. Step 4's
stated *intent* is "verify no survivor" (`# Expected: no output`); the literal
grep is a defective instrument for that intent, broken by a self-collision with
the plan's **own** Step 1 fixture - not an external surprise. The replacement
serves the stated intent exactly, is **stronger not weaker** (excludes the
false-positive fixture, still catches both real survivors, fire-verified), and
honors the falsifiability constraint (GC 29). Crucially, the implementer did
**not** silently swallow the defect: DONE_WITH_CONCERNS surfaced it, so the
plan-internal defect **did** round-trip to review for the harvest - which is the
correct handling, not a bypass.

**Ruling: within scope, upheld.** The decisive line is design-latitude vs
verification-instrumentation. No design decision was made; a plan-internal
self-check whose green state was unreachable (it collided with the plan's own
fixture) was re-instrumented to serve its own stated intent, stronger and
fire-verified both ways, and the defect was escalated rather than buried. That is
exactly the boundary between a fix an implementer may make at the keyboard and a
fork it may not. Minor refinement only: the concern note would ideally cite
`proc-check-green-state-reachable` by name so the class is legible to the
controller - polish, not a defect.

## HARVEST

1. **New occurrence of `proc-check-green-state-reachable`** (decision-ledger
   :3820, Tier-1 process pattern). Plan-7 Task 2 Step 4's absence grep
   (`English content only\|English only`) is permanently RED no matter how
   correctly the rustdoc rewrite lands, because it over-matches Step 1's **own**
   fixture strings (`only-en = English only`, the assertion, both mandated by the
   plan at Step 1). New flavor vs the round-1 occurrence (Task 5's grep hitting
   preserved DOM ids + a gitignored generated file): here the over-match target
   is the **plan's own earlier-step test fixture** - a self-collision internal to
   a single task. The class is confirmed again and the "construct the intended
   END state and show the expected-pass is reachable before the check enters the
   plan" handle would have caught it (running the grep against a tree that
   *already* has Step 1's fixtures shows 2 hits with the rustdoc correct).
   Suggested occurrence text: *"plan-7 Task 2 Step 4: absence grep
   `English content only|English only` collides with Step 1's own fixture strings
   (`only-en = English only`), unreachable green; implementer substituted
   `ships English content only|ships English only`, fire-verified both directions,
   and surfaced it as DONE_WITH_CONCERNS. Self-collision-with-own-fixture flavor."*

2. **Over-restriction watch (the substituted pattern).** The sharper pattern
   `ships English content only|ships English only` is tuned to the two known
   survivors' exact wording (both old lines contained "ships English ... only").
   It is correct as a **one-shot completion check** (the two real targets both
   match; fixture excluded). It must **not** be promoted into a standing/reusable
   "no English-only claims" guard as-is: as a persistent check it would under-fire
   on a differently-worded future survivor (e.g. "renders English only",
   "English-only mechanism"). This is the mirror risk of the green-state-reachable
   defect - trading an unclearable false-positive for a possible false-negative -
   and is acceptable here only because the check is a throwaway verifying two
   enumerated known lines are gone, not a lint. Record as a watch, not a defect.

3. **Process note (not a defect): the concern channel worked as designed.** The
   green-state-reachable defect was caught by the implementer, fixed without a
   design decision, and escalated via DONE_WITH_CONCERNS so it reached the harvest
   - the intended path for a plan-internal defect discovered on code contact. No
   ledger action beyond occurrence 1; recorded so the controller sees the
   escalation functioning rather than a silent swallow.

---

**Method notes.** All test/gate/probe runs foreground. The langid probe
(dimension 2) was appended to `i18n.rs`, run, then restored via `command cp -f`
from a pre-probe backup; `cmp` byte-identical and sha256 match confirmed, `git
status` clean at exit. No writes to the tree survive; this verdict file is the
only persisted output.
