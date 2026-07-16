# Task 9 report: D45 - the registry data layer, catalogs, and the i18n gate

**Branch:** plan6-e, worktree `/home/senol/Git/Muxsmith/.worktrees/plan6-e`
**Commit:** `57cc117` "gui: the field registry, its catalogs, and the label-key i18n gate (D45)"

## What was implemented

**`src/editor/fieldSpec.ts`** (types only, no runtime dependencies): `EditableField`/`FixedField`/`FieldSpec` per design `:737-741`, `TextSyntax` (4 values), `RegistryName` (13-member union, one per struct in the design's 13-struct table -- "profile" included by symmetry even though nothing currently points at it, since the design states the union has 13 members, not 12), and `FieldWidget` (10 closed variants, per `:848-860`).

**`src/editor/registries.ts`**: `COLLISION_POLICIES`/`KEEP_DROP` option arrays with the `satisfies` + `Exclude<...> extends never` completeness guard (design `:946-950`), then the 13 registries in the design table's order, filled row-by-row from the 43-row widget table (`:891-936`). All 42 `EditableField` entries plus the sole `FixedField` (`Profile.profile_version`). The four keyword arrays (`FILENAME_KEYWORDS`/`SOURCE_KEYWORDS`/`CHAPTERS_KEYWORDS`/`TITLE_KEYWORDS`) are imported from `src/bindings/keywords.ts`, never hand-written.

**`locales/en/gui-editor.ftl`** and **`locales/de/gui-editor.ftl`**: 43 keys each (42 field labels + `editor-save-note`), grouped by struct with `##` section comments matching the house catalog convention (`gui-jobs.ftl`'s per-component-area style). German register follows the established house terminology (`Spur`/`Regel`/`Vorlage`/`Eigenschaft`/`Match-Ausdruck` from `diagnostics.ftl`/`gui-batch.ftl`), declarative (not imperative) for the save note since the English source is a statement, not a command.

**`scripts/check-i18n.mjs`**: added `LABEL_KEY_RE = /labelKey:\s*(['"])([^'"]*)\1/g` alongside `CALL_RE`, scanned inside the same per-line loop over `src/**/*.{vue,ts}`, matches added to `literalCallIds` and pushed to `missing` when unknown -- identical treatment to `CALL_RE` matches. Also extended the file's top-of-file doc comment (check 1's description) to name this new coverage, since it would otherwise go stale the moment the scan landed.

## A wrinkle caught against the generated types (not a design defect)

The design's widget table (`:891-936`) names TrackRule's second field `match_expr` (the Rust identifier, used in prose because `match` collides with the reserved word). The **generated** TS binding (`src/bindings/profile.ts`), however, exports it as `match` -- ts-rs respects the Rust struct's `#[serde(rename = "match")]` (confirmed at `crates/muxsmith-core/src/profile/model.rs:283`) when projecting the type. `Record<keyof TrackRule, FieldSpec>` therefore requires the object key `match`, not `match_expr`; using `match_expr` would fail to compile (`match` missing, `match_expr` not a key of `TrackRule`). This isn't a table error -- the table is correctly naming the Rust field for readability -- so it needed no NEEDS_CONTEXT escalation, just the correct object-literal key. Recorded as a comment directly above `trackRuleFields` in `registries.ts`; the field's `labelKey` stays `editor-track-rule-match-expr` for catalog-side clarity.

## Falsifiability proof 1: registry-completeness break (Step 2)

Deleted the `on_collision` entry from `outputFields`, ran `pnpm build`:

```
src/editor/registries.ts(110,14): error TS2741: Property 'on_collision' is missing in type '{ directory: { labelKey: string; widget: { kind: "directoryPath"; optional: true; }; }; filename: { labelKey: string; widget: { kind: "keywordOrBlock"; keywords: readonly ["keep"]; block: "templateBlock"; }; }; }' but required in type 'Record<keyof OutputCfg, FieldSpec>'.
[ELIFECYCLE] Command failed with exit code 2.
```

Matches the brief's expected TS2741 shape exactly, naming the missing key. Restored the entry, `pnpm build` returned to a clean `vite build` (built in 122ms, no errors).

## Falsifiability proof 2: label-key gate break (Step 6)

Baseline `pnpm check:i18n`: exit 0, "ok (21 source files scanned, 229 catalog ids, 18 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s))."

Changed `outputFields.on_collision.labelKey` from `"editor-output-on-collision"` to `"editor-does-not-exist"`, ran `pnpm check:i18n`:

```
check-i18n: literal t()/$t() ids missing from the en catalog:
  src/editor/registries.ts:116  "editor-does-not-exist"
...
[ELIFECYCLE] Command failed with exit code 1.
```

Fires correctly, naming the file, line, and the bogus id. Reverted; `pnpm check:i18n` returned to the identical baseline output (exit 0, same "18 unused warning(s)" count -- `editor-output-on-collision` moved from the unused-warning list back to "detected" once the correct labelKey was restored).

## 43-key counts

```
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl
43
$ grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/de/gui-editor.ftl
43
```

Cross-checked further: en/de key sets are set-identical (`diff` of the sorted id lists, empty), and the 42 non-note en keys are set-identical to the 42 `labelKey: "..."` string literals extracted from `registries.ts` (`diff`, empty) -- so every registry label resolves to a real catalog id and no catalog id is orphaned.

## Gate results (nine parts, foreground, no subsets)

1. `cargo fmt --all --check` -- PASS.
2. `cargo clippy --workspace --all-targets -- -D warnings` -- PASS, zero warnings (no Rust files touched by this task; ran anyway per "no subsets").
3. `cargo test --workspace` -- PASS, all crates green (0 failed across every test binary).
4. `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` -- PASS.
5. `cargo deny check` -- PASS ("advisories ok, bans ok, licenses ok, sources ok").
6. `pnpm lint` (eslint) -- PASS, no output.
7. `pnpm build` (vue-tsc + vite build) -- PASS.
8. `pnpm check:i18n` -- PASS (exit 0), same 18-warning baseline as above (`editor-save-note` is the only gui-editor.ftl key in the unused list, expected: nothing renders it yet, that's Task 13's job).
9. `pnpm test:e2e` (tsc + vite build + Playwright) -- PASS, 7/7, including `e2e/catalogs.spec.ts` picking up the new `gui-editor.ftl` in both locales automatically (its glob-based `assertAllCatalogsParseCleanly`), confirming the brief's "for free" claim empirically rather than by assertion.

`node_modules` was missing in this fresh worktree; ran `pnpm install` first (used the existing lockfile, 222 packages, no resolution changes).

## Files changed

- `src/editor/fieldSpec.ts` (new): `EditableField`, `FixedField`, `FieldSpec`, `TextSyntax`, `RegistryName`, `FieldWidget`.
- `src/editor/registries.ts` (new): `COLLISION_POLICIES`, `KEEP_DROP` with completeness guards, and the 13 registries (`profileFields`, `metaFields`, `inputFields`, `outputFields`, `templateBlockFields`, `externalBlockFields`, `trackRuleFields`, `locatorFields`, `attachmentsFields`, `tracksFields`, `attachmentRuleFields`, `tagsFields`, `matchExprFields`).
- `locales/en/gui-editor.ftl` (new): 43 keys.
- `locales/de/gui-editor.ftl` (new): 43 keys.
- `scripts/check-i18n.mjs`: `LABEL_KEY_RE` scan added alongside `CALL_RE` (21 lines), header comment extended.

Committed staged explicitly (`git add src/editor/fieldSpec.ts src/editor/registries.ts locales/en/gui-editor.ftl locales/de/gui-editor.ftl scripts/check-i18n.mjs`), no `git add -A`. `e2e/catalogs.spec.ts` was not staged or edited, per the brief. `git status` after commit: clean working tree.

## Self-review

- 13/13 registries present, each typed `Record<keyof <Struct>, FieldSpec>` against the generated `src/bindings/profile.ts` types -- the compiler itself is the completeness proof (build is green); confirmed by reading the full file.
- 43/43 keys per catalog (counted both by `grep -c` and by the registry-labelKey cross-diff above).
- Four keyword arrays imported, not hand-written: `grep -n "KEYWORDS" src/editor/registries.ts` shows only the `import { CHAPTERS_KEYWORDS, FILENAME_KEYWORDS, SOURCE_KEYWORDS, TITLE_KEYWORDS } from "../bindings/keywords"` line and their four use sites; no local array declarations for any of the four.
- Both falsifiability proofs recorded verbatim above, each with an explicit restore-and-reconfirm-green step.
- `reorderable` set per the design's semantic rule: `true` for `tracks.rules`/`attachments.rules`, `false` for `matchExpr.any`/`matchExpr.not`.
- `optionalFlag` used only for `Locator.match_to_source` (not a tri-state `select`); `TextSyntax` carries all four values across the fields that need them (`regex` for `Input.pattern`, `templateLiteral` for `TemplateBlock.template`, `templateRegex` for `Locator.match_pattern`, `plain` for `Meta.name`/`Meta.description`); `propertyMap.properties` is `matchable` for `MatchExpr.exact`/`substring`/`regex` and `settable` for `TrackRule.changes`, matching the design's evidence-settled choices.
- Test output pristine across all nine gate parts; no warnings, no skipped tests.
- Typography: grepped all five changed/new files for the AI-tell glyph set (em/en-dash, minus sign, curly quotes, ellipsis, NBSP) -- clean; verified the grep pattern itself fires against a known-present em-dash control before trusting the clean result.

## Concerns

None blocking. Two judgment calls made where the design left implementation-level naming open (not a design fork -- these are non-normative, don't touch any table row, widget kind, or type shape):

1. **`labelKey` string naming convention.** The design gives exactly one worked example (`editor-output-*`); I extended the pattern to `editor-<registry-context>-<field>` for all 42 (e.g. `editor-track-rule-match-expr`, `editor-match-expr-any`). Fully enumerable and cross-checked against the catalog (see counts above), so any reviewer can verify it mechanically.
2. **English label wording and German translation choices** for the 42 short field-label nouns (e.g. "External locator" for `ExternalBlock.external`, "Match zur Quelle" for `match_to_source`) -- no house precedent existed for several of these (this is the first UI surface naming `Locator`/`MatchExpr`/`TrackRule` fields directly), so I anchored on established terminology already in `diagnostics.ftl`/`gui-batch.ftl`/`gui-common.ftl` (`Spur`, `Regel`, `Vorlage`, `Eigenschaft`, `Match-Ausdruck`) rather than inventing fresh vocabulary. A native-German reviewer pass on the 42 short labels would be cheap and is worth doing before Tasks 10-13 render them, since short noun labels are exactly the kind of thing that reads fine in isolation but wants a second pair of eyes once seen in the actual panel layout.
