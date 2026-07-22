# Task 9 review verdict: APPROVED

Reviewer: independent SDD task reviewer, Plan 7 Task 9 (editor control topics 1-9, D54 rows 1-9, en+de, 18 files).
Commit reviewed: 34a5aa7 on plan7-d (worktree clean at review time, `git status` empty).
Ground truth used: plan Task 9 + shared Tasks-8-10 rules (2026-07-21-plan-7-help-i18n.md:790-827), design D51/D54 rows 1-9, v1 spec 4.2-4.9 + 5.1-5.5, conventions.yaml (core-33), REAL tree (validate.rs, planner.rs, discovery.rs, capability/mod.rs, capability/runtime.rs, registries.ts, locales/de/*.ftl).

## Dimension 1: content accuracy (load-bearing claims)

Every hard behavioral claim verified against spec and code directly. No inaccuracy found.

- **Match algebra** (match-expr topic vs spec 4.3): five-part conjunction, AND within a part, `any` = at least one, `not` = none, arbitrary nesting, `EmptyMatchList` on present-but-empty lists, typed `exact` (`de` == `ger`), `substring` case-insensitive containment, `regex` as written with `(?i)` - all exact. Uniqueness contract (`MissingTrack` with near-miss hint / `AmbiguousRule` / `OverlappingRules`) matches spec 5.2 wording.
- **External locator** (source topic vs spec 4.6 + code): two-stage selection (locator selects files, match selects one track) and both-stage uniqueness (`AmbiguousExternal` / `AmbiguousRule`) confirmed in spec and planner.rs:550-575. `match_to_source` = sugar for `{match}` applied as an unanchored regex search on the donor basename (discovery.rs:143-166, `re.is_match(name)`), so "requires the primary's identifier in the donor's name" is correct. Mutual exclusivity = `LocatorConflict` (validate.rs). Locator `recursive` default false, own `extensions` list, path relative-to-primary-dir-or-absolute: all confirmed. `DonorIsPrimary` warning semantics match 5.2.
- **on_collision scope** (on-collision + filename topics vs spec 4.8/5.2): policy governs only pre-existing on-disk files; `SourceOverwrite` always hard error under every policy, explicitly including donors; two-planned-outputs `OutputCollision` always error, policy-independent, with the "no policy could define which plan wins" rationale. Per-policy severities (error/no-plan, skip/warning, overwrite/info) exact. `PathSeparatorInRenderedName` / `EmptyRenderedName` on the rendered name, all platforms: exact.
- **Required vs optional** (optional topic vs spec 4.5 + planner.rs:545-680): optional covers exactly the zero-match case; planner emits `MissingTrack`/`MissingExternal` only under `!rule.optional`, emits nothing on optional zero-match ("with no diagnostic" is code-true), emits `AmbiguousRule` unconditionally (uniqueness not relaxed), and calls `resolve_changes` only in the matched-exactly-one branch ("changes apply only when a track actually matched" is code-true). `SuggestionPartition` grouping claim matches 5.2/5.3.
- **Curated settable set** (changes topic vs capability/mod.rs:81-100 + validate.rs): the topic's 10 properties with their types are exactly the `SETTABLE` table (3 strings, 7 booleans, grouped correctly). `UnknownSettableProperty` at validate.rs:379, config-time. `raw:` accepted on the match side only, never in changes (spec 4.4; `validate_changes` has no raw path). Narrow-only guarantee = core-33 (conventions.yaml:597, closed StructuredEdit enum, narrows match only): the topic's "refinements to the rule's Match, never to its changes" states it correctly. Bonus: settable `language` accepting ISO 639-2 and BCP-47 verified against `LanguageIndex.is_valid_value` (runtime.rs:398-401, normalize OR BCP-47 grammar parse; plan-time `InvalidPropertyValue`, planner.rs:826-843); `sub_charset` lenient pass-through confirmed (planner comment: "other settables are carried through unchecked").
- **UnknownTemplateField at validate time**: implementer's citation re-verified - validate.rs:520, inside `validate_template`, called from the config-time static pass; the template topic's "a validation error, caught before any file is touched" is exact (spec 5.5 level 1 touches no filesystem beyond the profile). `{source_stem}` literal-mode-only additionally enforced for `match_pattern` (validate.rs:475 comment + empty `template_fields` path).
- Smaller claims spot-verified: input pattern unanchored basename search / first match (4.2), `IgnoredFile` info, `MultipleIdentifierMatches` info, `DuplicateIdentifier` warning with attract-same-externals consequence (5.2), extensions case-insensitive + `UnknownExtension` still-used-for-matching (4.2/5.2), input `recursive` default true, Recursive adjacency ("next to this field": registries.ts:104-108, pattern/extensions/recursive in order), template fields and filters (4.7) including `:int`/`:pad2`/`:pad3` examples, `(?<season>...)` syntax valid for the Rust regex crate.

## Dimension 2: rule conformance

- **h1 opener**: all 18 files open with an h1 naming the surface. 16 use `<label> (<section>)`; the template pair uses bare "Template"/"Vorlage" (see Q1 and finding 3).
- **Markdown subset**: clean. No code fences, no tables (zero `|` characters in the whole tree, which also excludes headerless GFM tables), no markdown links/autolinks, no URLs, no raw HTML. The only `<` characters sit inside inline code spans (`(?<season>\d{2})`), which marked treats as code, not HTML - legal under the subset.
- **Size band**: all 18 within 1-3 kB. en 1577-1837 bytes, de 1765-2060 bytes.
- **File set**: exactly D54 rows 1-9 x 2 locales; `ls help/en help/de | sort | uniq -c` shows each of the 9 ids exactly twice; commit 34a5aa7 touches exactly those 18 files, nothing else; no Task 10 id present anywhere in the worktree's `help/`.
- **Commit**: message and trailer exactly as plan Step 3 specifies.

## Dimension 3: de quality

- **Register**: du-imperative throughout ("Trage ... ein", "Erfasse", "Benenne", "Nutze", "Lass", "Behebe", "Beachte", "prüfst du"), per the plan's Tasks-8-10 rule (the more specific authority over gui-editor.ftl's declarative-label clause, which governs catalog labels only). Config keywords kept literal (`keep`, `error`, `skip`, `overwrite`, `primary`, `match_to_source`, `true`/`false`).
- **Independence**: genuinely authored de prose, not an en echo - parallel structure but native phrasing, consistently ~10% longer, with idiomatic recastings ("sich einspielen", "per Definition veraltet", "laut scheitern lassen").
- **Catalog terminology**: h1 labels match locales/de/gui-editor.ftl exactly (Muster, Erweiterungen, Dateiname, Bei Kollision, Vorlage, Quelle, Match, Optional, Änderungen). Domain terms consistent with the de catalogs: Stapel, Probelauf, Spur/Spurregel, Kennung, externer Verweis, Spender-Datei (diagnostics.ftl:44), Primärquelle, Vorlage, setzbare Eigenschaft, Ausgabe.
- **Orthography**: correct umlauts/ß throughout; no typographic AI-tell glyphs (em/en-dash, curly quotes, ellipsis, NBSP - grep-verified, pattern fire-verified).

## Dimension 4: house

- Cross-topic references are prose ("see the Source topic" / "siehe das Thema Quelle"), never links - D62 check 4 posture, uniformly applied (11 en + 11 de reference sites).
- Diagnostics named by exact code in backticks, matching DiagCode identifiers and the catalog's message ids.
- Scope discipline: rows 1-9 covered per their justification lines without bleeding into Task 10's topics - `raw:`/typed-domain detail deferred to the Exact topic, locator pairing detail to the two locator topics, each with a pointer. Matches D54's partition intent.

## Findings

1. **(info)** extensions topic: "Every entry is validated at runtime against the local mkvmerge's `--list-types` output" is unconditional; spec 5.2 skips `UnknownExtension` when the capability query is unavailable. Depth-appropriate simplification for user-facing help (a mkvmerge-less run degrades wholesale anyway); at most an owner-pass wording candidate. No fix required.
2. **(info)** match-expr topic states `substring`/`regex` are "string properties only" without the `raw:` exemption (spec 4.3). The bypass is the Exact topic's assigned content per D54 row 10, and the topic points there. Consistent with the design's partition; no fix required.
3. **(info)** h1 internal exception: the template pair's bare "Template"/"Vorlage" vs the other eight `<label> (<section>)`. Justified - TemplateBlock.template serves both output filename and title, so any single section qualifier would be wrong. Feeds Q1.

No blocking or major findings.

## Q1 adjudication: h1 form variation across streams

**Ruling: acceptable per-stream variation riding the owner's wording pass; not a Task 9 defect; flag to the whole-branch review for alignment.**

For treating it as a defect: the 22 topics render in one sidebar; a user hopping topics sees three title conventions, and cross-referencing prose ("see the Match topic") reads cleanest when titles follow one scheme; consistency across a single rendered surface is normally one artifact's duty. Against: the shared rule deliberately says only "an h1 naming the surface" - all three forms satisfy it; the streams are mutually independent by design; and D54 explicitly routes topic content wording through the owner's plan-close rendered-surface pass, which is the designated alignment point for exactly this class of wording variance. Charging Task 9 with a cross-stream property no single stream can see (streams C/E live in other worktrees) would make the fix loop the wrong instrument.

Recommendation for the alignment: adopt Task 9's `<label> (<section>)` form as the branch convention. Registry labels collide across sections (`extensions` and `recursive` exist on both Input and Locator; `unmatched` and `rules` on both Tracks and Attachments - Task 10's territory), so an unqualified label cannot disambiguate; the parenthetical does it uniformly where Task 10's ad-hoc surface names do it case by case. Keep the bare-template exception (one block, two host surfaces).

## Q2 adjudication: replaced combined grep + content cleanliness

**Ruling: process-correct, and the committed content is clean under both corrected checks (re-run independently).**

The combined fence+table pattern silently missing a planted table is precisely the malformed-absence-check failure mode the house conventions name: an empty grep result is identical whether the content is clean or the pattern is wrong, so each defect class needs its own check, fire-verified once. Splitting and separately fire-verifying is the prescribed remedy, not an improvisation. I did not reuse the reported malformed pattern; I re-ran independent equivalents: (a) fence check `grep -rn '\`\`\`' help/` - clean, exit 1; (b) table check `grep -rnE '^\s*\|' help/` - clean, exit 1; plus (c) a strictly stronger zero-pipe check `grep -rn '|' help/` - clean, exit 1 - which also catches headerless GFM tables the leading-pipe form misses. All three patterns fire-verified against a planted control file (scratchpad, outside the worktree) containing a fence, a leading-pipe table, and a headerless pipe table: every pattern produced hits on the control (fence 2, leading-pipe 2, any-pipe 4), plus URL/HTML/mdlink controls for the other absence checks (1 hit each).

## HARVEST

- **Pattern for Task 19 / D62 gate**: for the help tree the zero-pipe grep strictly dominates the leading-pipe table check - topics have no legitimate `|` at all, and `^|` misses headerless GFM tables. Worth adopting as the gate's table check.
- **Gate hazard found via my own false positive**: a naive raw-HTML check (`<[a-zA-Z/]`) fires on `(?<season>...)` inside inline code spans in both pattern topics - content that is legal under the subset (marked does not parse code spans as HTML). Task 19's markdown-subset check must exempt inline code spans (or anchor on block-level/line-start HTML), else two legitimate files go red on day one.
- **de terminology for the owner pass** (coinages with no catalog anchor yet; keep consistent across streams): "Beinahe-Treffer" (near-misses - diagnostics.ftl has no `.hint` attributes yet, so no conflict today, but reuse this term when hints land), "Nur-Einschränken-Garantie" (narrow-only, core-33 - Task 8's batch-suggestion-card topic must name the same guarantee; cross-stream sync to check at whole-branch review), "Ein-Klick-Übernahme" (one-click apply - same cross-check vs stream C), "Spender-Zuordnung" (donor pairing), bare "Verweis" as running short form of the catalog's "Externer Verweis" (acceptable, but the owner should ratify the short form).
- **Over-restriction watch on the provisional rules**: no chafing observed. The 1-3 kB band held with headroom on all 18 files (max 2060 bytes, 67% of ceiling); the markdown subset never forced an awkward workaround - lists, emphasis, and inline code carried everything the D54 briefs required. Nothing in this task argues for loosening the provisional rules; the veto window can close on them unchanged as far as this stream is concerned.
