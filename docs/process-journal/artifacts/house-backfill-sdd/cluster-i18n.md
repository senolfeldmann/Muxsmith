# House-knowledge clusters - domain `i18n`

Reconstructed from 30 occurrence records spanning eras E0-E8. Records were grouped by identical `(topic, approach)`; occurrences merged, identical `date+ref` deduped, distinct `date+ref` kept as genuine recurrence. `promoted = count >= 3`.

Dates verified against the repo (`git show -s --format=%cs`): `61249f9`/`a671949`/`3c24845` = 2026-07-08; `2e0dc00` = 2026-07-09; `79f0447` = 2026-07-10; `362db2d`/`c229400`/`0d19dc4`/`b38a46f` = 2026-07-12. Task-verdict artifacts without a commit are dated by plan cycle (plan-3/4 = 07-10, plan-5 = 07-11, plan-5.5 = 07-12), consistent with the sibling `cluster-cross.md` anchors.

Two clusters reach the promotion threshold: **prose-free core** (count 5) and **Fluent CLDR plural selectors** (count 3). Everything else is a single- or double-touchpoint decision. Three non-decisions remain blocked (deferred fixes). No count was padded - see the clustering notes at the bottom for the two judgment calls (record 5 double-cited, records 4+7 deduped).

---

## i18n-01-fluent-choice - Fluent as the one i18n system (single catalog, CLI + GUI)
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Fluent is the i18n library, chosen because it is the one system with first-class Rust (`fluent-rs`) AND JS (`@fluent/bundle`) implementations, so a single catalog under `locales/` serves the CLI and the future GUI and message templates exist exactly once.
- **Steelman:** gettext/ICU are more established but lack a matched Rust+JS pair, which would force two divergent catalogs.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §2 row 10 + §8.4 + journal 2026-07-08 bullet 6 | "Fluent chosen because it is the one system with first-class Rust AND JS implementations, so one catalog serves CLI and future GUI." |
| 2026-07-08 | decided | spec §2 'Localization' + §8.4; Plan 1 Global Constraints; commits 61249f9 / a671949 | E1 re-attestation as Plan 1 Global Constraint + first implementation. |

---

## i18n-02-prose-free-core - Core emits codes + params only, never user-facing prose
- **kind:** pattern | **status:** settled | **count:** 5 | **promoted:** yes (at 3)
- **Statement:** `muxsmith-core` emits diagnostic codes + structured params only, never user-facing prose; all labels, messages and hints live in shared Fluent catalogs (and per-locale markdown) rendered at presentation time. A new `DiagCode` without a matching `.ftl` message fails the catalog-completeness test. (Open residual breach tracked as `i18n-17`.)
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec 2026-07-08 §5.2 + §8.4 + journal 2026-07-08 bullet 6 tail | "Core emits code+params only; this rule later forced a real fix." |
| 2026-07-08 | decided | spec §2 + §8.4; Plan 1 Global Constraints; commits 61249f9 / a671949 | "No hardcoded user-facing strings in any layer; core emits diagnostic code + params only." |
| 2026-07-08 | violated-corrected | journal 2026-07-08 final review + whole-branch-review-verdict.md (Important #1), round-2 confirmation; fix commit 3c24845 | "Template-error params carried English prose out of core (format!(\"unknown filter: ...\")), failing the plan's own exit criterion and spec 8.4; spec won, restructured to code-like kind/name tokens rendered via a Fluent selector." |
| 2026-07-10 | reinforced | plan Global Constraints + Task 6 (UnsupportedSource) | "A new DiagCode without a matching .ftl message fails catalog_completeness.rs; Task 6 added the unsupported-source Fluent message." |
| 2026-07-10 | reinforced | plan Global Constraints + task-3-review-verdict.md | "A DiagCode without a Fluent message fails catalog_completeness; JobEvents carry pass-through mkvmerge text, not core-authored UI prose." |

---

## i18n-03-content-scope-en-v1 - Mechanism complete day one, English-only content in v1
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** The i18n mechanism ships complete from day one, but v1 ships English catalogs and help topics only; adding a locale is content work, not a refactor. No target locale was committed for v1; German was later added at plan T21 (`i18n-11`), validating the "content work, not a refactor" claim.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | deferred | spec 2026-07-08 §2 row 10 + §11 | "i18n-ready from day one; English-only content ships in v1." (no target locale committed for v1) |
| 2026-07-08 | decided | spec §2 'Localization' + §11 non-goal; commit 61249f9 | "Adding a locale is content work, not a refactor." |

---

## i18n-04-param-agreement - A message references only params the emitter sets
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A Fluent message must reference only params the emitter actually sets, or the placeable renders literally. Bug G: the `UnknownPropertySkew` message referenced `{$property}` but the emitter set only `$version`, so it rendered a literal `{$property}`; the catalog guard only checked message existence, not param wiring. (Deeper: `validate` hard-rejects unknown props at config time, so skew's untyped-forward-matching path is unreachable in v1 - a known limitation.)
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | independent-review-2026-07-09.md bug G + F9; commit 2e0dc00 | "UnknownPropertySkew message references {$property}, emitter only sets version -> renders literal {$property}." |

---

## i18n-05-plural-selectors - Count-dependent messages pluralize via Fluent CLDR selectors
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Count-dependent user-facing messages pluralize via Fluent CLDR plural selectors (`{ $n -> [one] ... *[other] ... }`), not an `error(s)`/`(s)` provisional. First applied to `run-job-warning`, later extended to five keys replacing the `error(s)` provisional, with singular+plural renderer assertions.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | deferred | task-8-review-verdict.md | "The defect is in the brief's mandated text, not the code ... the human decides whether to amend the spec." |
| 2026-07-10 | decided | memo D15 amendment 2026-07-10 + commit 79f0447 | "run-job-warning pluralizes the warning count via a Fluent plural selector (plan's locked text rendered '1 warnings')." |
| 2026-07-12 | decided | task-19-verdict.md / plan T19 | "five keys on CLDR [one]/*[other]; zero (s) patterns remain." |

---

## i18n-06-plural-numeric-args - Plural args must be numeric FluentValues; Rust/TS promotion mirror pinned
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Plural-selecting Fluent args must be numeric `FluentValue`s, not strings, or the `[one]` variant never matches. The mirrored Rust/TS lists of which params get numeric-promoted are a real cross-language contract that `check:i18n` does not cover, pinned by a Rust-side test that names the TS mirror (TS tightened to `Number.isInteger && >=0`).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | journal 2026-07-10 Plan-4-complete + commit 79f0447 (i18n.rs msg_with_count) | "$count reached Fluent as a string, so [one] could never match; caught only because the dispatch mandated verifying the arg type." |
| 2026-07-12 | decided | task-19-verdict.md (b); fix 0d19dc4 | "a REAL unenforced cross-language contract ... pin test ADDED (Rust side, names the TS mirror)." |

---

## i18n-07-plurality-render-boundary - Plurality resolved at the render boundary, not the wire
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Plurality is resolved at the render boundary (promotion), not carried through a typed-params wire change - the wire change would touch dozens of call sites plus JSON schema and IPC types for only two codes' benefit.
- **Steelman:** A typed-params wire change would carry plurality end-to-end in the type system rather than reconstructing it at render.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-19-verdict.md (c) | "vs. a typed-params wire change (dozens of call sites + JSON schema + IPC type for two codes' benefit)." |

---

## i18n-08-no-raw-text-lint - No bare template text (D27), enforced by @intlify no-raw-text
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** CI enforces no bare template text (including aria strings) via the `@intlify` no-raw-text lint, verified at plan time to fire without vue-i18n as the runtime (custom check as fallback). Lint workarounds may relocate only passthrough/ASCII non-prose data, never real user-facing copy.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | memo D27 / plan T4 step2 | "the exact rule is verified at plan time - it must fire on bare template text without requiring vue-i18n as the runtime library." |
| 2026-07-11 | reinforced | task-9-review-verdict.md / task-10-review-verdict.md | "both cited workarounds genuinely relocate non-prose data rather than smuggling real user-facing copy past the lint." |

---

## i18n-09-usage-completeness-gate - check-i18n.mjs hard-fails on missing t() ids
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A CI script (`check-i18n.mjs`) parses ftl ids and scans src for `t()` ids, hard-failing (exit 1) on any `t()` id missing from the catalogs; unused keys warn only. Exemption comments are supported.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | plan T12 / task-12-review-verdict.md | "hard-fail on missing literal ids, exemption comments present." |

---

## i18n-10-cross-locale-parity - check:i18n cross-locale key parity, hardened with real-parse guard
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** `check:i18n` enforces cross-locale key parity with EN as the fixed reference, extended to include `cli.ftl` (which has no UI chrome to mask a silent English fallback). A regex-based parity check has a blind spot - malformed multiline indentation can pass parity while the real Fluent parser rejects it - closed by an e2e real-Fluent-parse guard covering every catalog of every locale.
- **Steelman:** null
- **Occurrences:** (record 22 fixes a blind spot in record 21's own mechanism -> same parity concern, kept as two distinct touchpoints)

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-20-verdict.md / plan T20 | "Extending check 3 ... is right - cli.ftl has no UI chrome to mask a silent English fallback." |
| 2026-07-12 | violated-corrected | task-20-verdict.md m1 / task-21-verdict.md; commit c229400 | "c229400: e2e real-Fluent-parse guard covers every catalog of every locale incl. cli.ftl." |

---

## i18n-11-german-locale - Six catalogs translated to German, terminology anchored to de.po, Şenol gate
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** All six catalogs translated to German, terminology anchored to mkvtoolnix's `de.po` facts (single domain terms only, zero sentence overlap verified across 12 distinctive sentences) behind a hard Şenol terminology gate before merge; gate passed with three corrections (Starten, Meldungen, Verweis).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-21-verdict.md / plan T21 | "zero sentence overlap with mkvtoolnix de.po (12 distinctive sentences grepped; anchor terms genuinely present)." |
| 2026-07-12 | reinforced | journal 2026-07-12 Plan 5.5 / progress.md; commit 362db2d | "terminology approved with 3 corrections (Starten, Meldungen, Verweis - applied 362db2d)." |

---

## i18n-12-de-placeable-parity-deferred - Placeable/selector-structure drift in de not machine-checked
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** Placeable-name and selector-structure drift in the German catalogs is not machine-checked (`check:i18n` enforces id parity only); a `check:i18n` extension to cover placeable/selector structure was deferred, no actual drift found at close.
- **Blocked on:** future check:i18n extension (internal).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | task-21-verdict.md residual / whole-branch-verdict.md M1 | "placeable-name and selector-structure drift in de is not machine-checked." |

---

## i18n-13-cli-key-rename-deferred - dry-run-summary -> batch-summary rename deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The `dry-run-summary` cli.ftl key serves both run and dry-run (a latent catalog-skimming trap); renaming it to `batch-summary` was deferred (touches en+de+allowlist+2 snapshots).
- **Blocked on:** idiomacy review (internal).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | task-8-verdict.md m2 / whole-branch funnel T8-m2 | "rename to batch-summary (touches en+de+allowlist+2 snapshots)." |

---

## i18n-14-locale-picker-endonyms - Locale picker labels are endonyms
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Locale picker labels are endonyms (English/Deutsch identical in both catalogs) per universal picker convention; T21's language-name precedent was an accidental byproduct.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-21.5-verdict.md | "Endonym labels: English/Deutsch identical in both catalogs (universal picker convention)." |

---

## i18n-15-settings-hint-evergreen - Settings locale hint is evergreen, no language enumeration
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The settings locale hint is evergreen - no language enumeration, tightened to two plain sentences - because enumerating locales re-stales the hint at every new locale (it was re-staling at locale #3).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-21.5-verdict.md | "Evergreen hint: no language enumeration (was re-staling at locale #3)." |

---

## i18n-16-bilingual-cutoff - New messages EN-only in waves 1-2, bilingual (en+de) from T19
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** New Fluent messages are EN-only during waves 1-2 and land bilingual (en+de) from T19 onward (constraint C2); thereafter any new or changed user-facing message lands in both locales in the same change, enforced by the cross-locale parity gate.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | progress.md C2 / plan Global Constraints | "new Fluent messages in Waves 1-2 are EN-only until T19; from T19 on bilingual (en + de)." |
| 2026-07-12 | reinforced | CONVENTIONS.md Patterns (b38a46f) | "seeded from Plan 5.5 (T19/T20 cross-locale parity gate)." |

---

## i18n-17-mixed-language-allowed-deferred - Core emits English via `allowed` param, catalog fix deferred
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** Core emits English prose through the `allowed` param at two sites, rendering a mixed-language diagnostic in de mode on the marquee language property - a residual breach of prose-free core (`i18n-02`); the catalog-side fix (a kind selector) was too large for the branch close and deferred to pre-1.0 polish.
- **Blocked on:** pre-1.0 polish / Plan 6 (internal).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | whole-branch-verdict.md I2 | "a mixed-language diagnostic on the marquee language property undercuts it ... DEFER with a ROADMAP pre-1.0-polish line." |

---

## Clustering notes (defensibility)

- **Record 5 (E1 combined Global Constraint) is cited in two clusters** - `i18n-01-fluent-choice` and `i18n-02-prose-free-core`. The E1 record ratifies *both* the Fluent library choice and the prose-free-core constraint in one Plan 1 artifact, so it is a genuine occurrence of each. Neither cluster's promotion status turns on it: `i18n-01` is 2 with or without a split, `i18n-02` is already 5. No count crosses a threshold because of the double-cite.
- **Records 4 and 7 are the same event, deduped to one occurrence** in `i18n-02`. Both describe the template-error prose leak caught at the final review and fixed in commit `3c24845` (record 4 is the E0 journal framing, record 7 the E1 whole-branch-verdict framing). Identical fix commit -> one occurrence, not two.
- **Pluralization is split into three honest clusters, not merged into one inflated count:** `i18n-05` (the CLDR-selector pattern: run-job-warning deferred->decided, then five keys), `i18n-06` (the numeric-`FluentValue` arg constraint + Rust/TS promotion mirror), `i18n-07` (the render-boundary restraint). Records 11 and 12 share commit `79f0447` but are different events (the pluralization decision vs the `$count`-is-a-string bug), so they sit in different clusters rather than double-counting the commit.
- **`i18n-10` merges records 21 and 22** (same cross-locale parity mechanism: 22 closes a regex-vs-real-parser blind spot in 21). `i18n-09` (usage completeness: src `t()` id not in catalog) is kept separate - a distinct failure mode from cross-locale parity, despite living in the same `check:i18n` tooling family.
- **No cluster is `contested`.** Every deferral that returned was resolved (`i18n-05` run-job-warning; `i18n-03`/`i18n-11` EN-only-then-German). The three open items (`i18n-12`, `i18n-13`, `i18n-17`) are `blocked` non-decisions, not contested recurrences.
- **`i18n-17` is a live residual breach of `i18n-02`** but is tracked as its own deferred non-decision rather than folded in, so the prose-free-core cluster stays `settled` (the principle holds and was reinforced) while the open exception stays visibly `blocked`.
