# Verdict (extracted from the reviewer transcript at write-time)

Now I have everything needed. Compiling the review.

### Spec Compliance

- ✅ **Files.** Exactly `src/editor/fieldSpec.ts`, `src/editor/registries.ts`, `locales/en/gui-editor.ftl`, `locales/de/gui-editor.ftl` created; `scripts/check-i18n.mjs` modified. `e2e/catalogs.spec.ts` untouched and absent from the commit (`git show --name-only 57cc117` lists exactly the 5 briefed files).
- ✅ **13 registries, total coverage, no subset.** Verified `Record<keyof T, FieldSpec>` on all 13: `profileFields`, `metaFields`, `inputFields`, `outputFields`, `templateBlockFields`, `externalBlockFields`, `trackRuleFields`, `locatorFields`, `attachmentsFields`, `tracksFields`, `attachmentRuleFields`, `tagsFields`, `matchExprFields` (`src/editor/registries.ts:449-592`).
- ✅ **43-row table walked field-by-field against `registries.ts`.** Every one of the 43 rows in D45's table matches: same field, same widget kind, same widget parameters (keywords array, block/of/item registry name, `optional`, `reorderable`, `syntax`, `properties`/`values`). No missing field, no extra field, no widget-kind mismatch. `profile_version` correctly uses `FixedField` (`fixed: true`, source-comment `why`), not `Omit<Profile, "profile_version">` and not a `FieldWidget` variant.
- ✅ **`FieldWidget` 10 variants, closed, `fixed` excluded.** `src/editor/fieldSpec.ts:359-371` matches the design union verbatim: `text`, `bool`, `optionalFlag`, `select`, `keywordOrBlock`, `directoryPath`, `stringList`, `propertyMap`, `list`, `section` — exactly 10, `fixed` correctly kept out as the other half of `FieldSpec`.
- ✅ **Three likely-wrong widget choices, checked individually.** `optionalFlag` used only once, for `Locator.match_to_source` (`registries.ts:536`) — not a `select`, matching the "off-state is absence" ruling. `TextSyntax` all four values present and correctly assigned: `plain` (`Meta.name`/`description`), `regex` (`Input.pattern`), `templateLiteral` (`TemplateBlock.template`), `templateRegex` (`Locator.match_pattern`). `propertyMap.properties` correctly split `matchable` (`MatchExpr.exact/substring/regex`) vs `settable` (`TrackRule.changes`), not collapsed to one value.
- ✅ **`reorderable`.** `true` on `tracksFields.rules`/`attachmentsFields.rules` (`registries.ts:548-554`), `false` on `matchExprFields.any`/`.not` (`registries.ts:590-591`), with an inline comment naming the semantic reason — matches design exactly.
- ✅ **Keyword arrays imported, not hand-written.** `registries.ts:409-414` imports `CHAPTERS_KEYWORDS`, `FILENAME_KEYWORDS`, `SOURCE_KEYWORDS`, `TITLE_KEYWORDS` from `../bindings/keywords`; no local array literal for any of the four (confirmed by reading the four use sites, all bare identifiers).
- ✅ **`COLLISION_POLICIES`/`KEEP_DROP` completeness guard.** `registries.ts:432-445` matches the design's `as const satisfies readonly X[]` + `Exclude<...> extends never` shape exactly (identifier names vary slightly from the design's illustrative names, immaterial).
- ✅ **43 keys per catalog, independently recounted.** I manually counted both diff hunks (not the implementer's grep output): en = 43 (42 labels + `editor-save-note`), de = 43, same key set. No tooltip keys, no widget-facet keys present in either file.
- ✅ **`editor-save-note` en text is character-for-character the brief's fixed text.** Diff line 210 matches the brief verbatim.
- ✅ **`LABEL_KEY_RE` matches the design's regex exactly** (`/labelKey:\s*(['"])([^'"]*)\1/g` — confirmed against design text at the true location, `docs/superpowers/specs/2026-07-15-plan-6-design.md:1124`; the brief's cited `:1082-1091` is stale by ~35 lines, a brief/design drift not attributable to the implementer) and feeds the identical `literalCallIds`/`missing` path as `CALL_RE` (`scripts/check-i18n.mjs:279-285`). Check 2 and check 3 code paths are untouched — read the full file; only the doc comment and the new regex+loop were added.
- ✅ **Falsifiability proofs.** Both reported outputs are structurally consistent with the design's own worked TS2741 example and with the actual code (`match_expr` reasoning independently re-derived below); not re-run per instructions, no reason found to doubt them.
- ⚠️ **Nine-part gate results** (cargo fmt/clippy/test/doc/deny, pnpm lint/build/check:i18n/test:e2e) taken on the implementer's word — not re-run, per review scope.
- ✅ **Typography.** Independently grepped all 5 changed files for the AI-tell Unicode ranges (dashes, curly quotes, ellipsis, NBSP): clean.
- ✅ **Commit.** Unsigned (`%G?` = `N`), message matches the brief's exact text, exactly the 5 briefed files staged, nothing else.

### Adjudications

**Q1:** (a) **Confirmed.** `src/bindings/profile.ts:373` (`export type TrackRule = { source?: SourceCfg, match: MatchExpr, optional?: boolean, changes?: ... }`) carries `match`, not `match_expr`; the Rust source (`crates/muxsmith-core/src/profile/model.rs:283`, `#[serde(rename = "match")] pub match_expr: MatchExpr`) confirms why. A `match_expr:` key in `trackRuleFields` would trigger both a missing-property error on `match` (TS2741) and an excess-property error on `match_expr` against `Record<keyof TrackRule, FieldSpec>` — it cannot compile. (b) **`match` was the only type-correct resolution.** This is squarely the `brief-drafts-verified-against-tree` house pattern (`docs/process-conventions.yaml:355-372`): a literal draft element (the design table's Rust-identifier naming, itself correct for prose readability) diverges from the generated tree artifact; the implementer verified against the tree, adapted (used `match`), and surfaced it in the report as a disclosed wrinkle rather than escalating as NEEDS_CONTEXT or silently transcribing. Correctly routed, not a fork that needed owner adjudication — there was no second type-correct option to choose between.

**Q2:** (a) **Conforms mechanically.** No curly quotes in either `gui-editor.ftl` (grepped, clean, matching the straight-ASCII-quotes rule stated in `locales/de/diagnostics.ftl`'s header). The five established terms the de header claims (`Spur`, `Regel`, `Vorlage`, `Eigenschaft`, `Match-Ausdruck`) are all independently attested in `diagnostics.ftl`/`gui-batch.ftl`/`cli.ftl` before this task, not invented. Declarative register on the save note matches the "statement, not command" rationale; the header block itself follows the same "en catalog is source of truth, parity enforced by check-i18n" boilerplate every de catalog uses. (b) **Within mandate.** The brief's Step 4 explicitly requires 42 labelKey texts per locale with a specific fixed exception (the save note); authoring short field-label prose is the deliverable, not a scope excursion. The implementer correctly flagged the sub-case with genuinely no house precedent (first UI surface naming `Locator`/`MatchExpr`/`TrackRule` fields directly) as a pending native-speaker QA item rather than silently presenting invented vocabulary as settled — appropriate given the controller has already flagged that pass as owed.

### Strengths

- The 43-row table walk found zero discrepancies against the widget table — full row-by-row fidelity, including the three fields the brief specifically warned were easy to get wrong.
- The `match`/`match_expr` divergence was caught, correctly resolved, and disclosed with a source comment at the exact site (`registries.ts:510-515`) rather than left for a future reader to rediscover.
- `check-i18n.mjs` change is genuinely minimal and additive: same regex shape, same loop structure, same target sets as `CALL_RE`, no new parsing infrastructure introduced.
- Catalog header documents its own terminology choices (loanword vs. calque decisions) inline, in the same style diagnostics.ftl's de header already uses — makes a future translator's job checkable rather than guessable.
- The judgment calls in the report's "Concerns" section were pre-identified and flagged for a targeted follow-up (native-German label QA) rather than presented as settled, and the labelKey naming scheme was disclosed as a convention rather than silently applied.

### Issues

#### Critical (Must Fix)
None.

#### Important (Should Fix)
None.

#### Minor (Nice to Have)
- `locales/de/gui-editor.ftl:1` uses `##` for its top-of-file header while its own source `locales/en/gui-editor.ftl:1` uses `#`. Every other de/en catalog pair in the tree keeps the hash count matched to its own en source (`gui-jobs.ftl` `#`/`#`, `gui-batch.ftl` `##`/`##`, `gui-settings.ftl` `###`/`###`) — this pair breaks that (admittedly small-n) self-consistency. Cosmetic only; Fluent does not parse comment depth and check-i18n does not either.

### HARVEST

- A second confirmed occurrence of `brief-drafts-verified-against-tree` in this same plan-6 wave (design's `match_expr` naming vs. the generated `match` binding key), cleanly disclosed and correctly adjudicated — reinforces the existing entry, no new pattern.
- The task brief's own cross-references to the design document are stale relative to the design's actual current line numbers (`:1082-1091` cited for the check-i18n fix, actually at `:1117-1139`; similarly `:694-706`/`:806-819` for the type definitions did not land where cited). Content matched in every case, so this cost nothing here, but it is the same class of drift `proc-04-spec-wins`/the design's own self-contradiction-sweep guards against, one level up: a brief's line citations into a design doc can drift when the design is amended after the brief is cut, and nothing currently re-verifies a brief's citations post-amendment. Worth a citation-freshness check if this pattern recurs on a task where content, not just line numbers, has drifted.

### Assessment
**Task quality:** Approved
**Reasoning:** Every spec-compliance item verified against the committed tree checks out; the one type-correct divergence from the design table was caught, resolved correctly, and disclosed per house pattern; the only finding is a single cosmetic hash-count inconsistency with no functional effect.
