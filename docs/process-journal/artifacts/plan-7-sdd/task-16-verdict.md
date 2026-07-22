# Task 16 verdict - D59 presentation-only 1-based ordinal column

**Reviewer:** independent (fresh eyes), wave-2 last task.
**Commit under review:** `8f6400f` (single commit, range `9f4aa8a..8f6400f`).
**Worktree:** `/home/senol/Git/Muxsmith/.worktrees/plan7-f` (branch `plan7-f`, HEAD `8f6400f`).

## Combined verdict: APPROVED

- **Spec-compliance verdict: PASS.** Every D59 / brief requirement is met literally: presentation-only leading column, no data-model touch, `{{ index + 1 }}` 1-based, re-renders 1..n by array position after drag-reorder, one new attribute-less bilingual key `editor-track-rule-order` (en `Order` / de draft `Reihenfolge`) in the declarative register, smoke.spec.ts extended additively inside the existing rule-grid describe block. Catalog count recomputed independently: en 46 / de 46, matching the commit message and the `editor-generic-action-keys` budget (42 labels + 1 save + 2 actions + 1 ordinal = 46). No data change (verified: `<td>{{ index + 1 }}</td>`, drag handlers untouched, no per-row Fluent key).
- **Quality verdict: PASS.** Full frontend gate re-run green under my own build; the load-bearing fire-verification claim independently reproduced against a broken binding (both the 0-based and the static-column degenerate states fail the assertions).

No blocking or major findings. Nothing requires a fix before merge.

---

## Findings by severity

### Blocking
None.

### Major
None.

### Minor
None.

### Informational
1. **en catalog header comment not extended to mention the ordinal key.** The `## ...` header at `locales/en/gui-editor.ftl:1-7` enumerates "one labelKey per EditableField ... plus the save-surface note (D41)" and asserts "Every label message carries its tooltip". The new key is neither a registry labelKey nor the save note, and it is not a "label message" in the registry sense, so the header stays literally true and is not falsified by this change (it already omitted the pre-existing generic action keys, i.e. it is descriptive-of-primary-contents, not a strict inventory). No sweep-duty edit is owed (consistent with the plan-7 T3 precedent: a header edit is owed only when the change falsifies a header sentence). The implementer correctly left it unchanged and flagged it; recording here in case the controller wants the header to name the ordinal for completeness. Not a defect.
2. **de/en draft strings ride the owner's rendered-surface pass.** `Reihenfolge` / `Order` are the brief-specified drafts; both are declarative nouns fitting the de catalog's declared register. A positional-index column could alternatively read `Nr.`/`Position` (the column shows an ordinal, not a re-orderable sequence field), but the final wording is explicitly the owner's plan-close gate per D59 and the Global Constraint, and the implementer used exactly the brief's draft. No action for the implementer.

---

## Gate-run summary (foreground, my own build)

| Gate | Command | Result |
|---|---|---|
| Type + build | `pnpm build` (`vue-tsc --noEmit && vite build`) | PASS - 165 modules, no type errors |
| i18n | `pnpm check:i18n` | PASS (exit 0) - 41 source files, 211 catalog ids, 17 pre-existing IpcError false-positive warnings (ordinal key NOT among them), de/en parity holds |
| Lint | `eslint src/views/EditorView.vue e2e/smoke.spec.ts` | PASS (exit 0), clean |
| e2e (full) | `pnpm test:e2e` | PASS - **52 passed**, incl. test 42 (the D59 rule-grid test) |

**Independent fire-verification (report claim 3, plus the brief's degenerate-state duty):**
- **0-based off-by-one:** changed `{{ index + 1 }}` -> `{{ index }}`, rebuilt the mount bundle, ran filtered `--grep "drag-reorder swaps the rows"` -> **1 failed**, `Expected: "1" / Received: "0"`. Confirms the assertion pins the 1-based offset.
- **Static column:** changed the cell to literal `<td>1</td>`, rebuilt, ran -> **1 failed** at line 949, `Expected: "2" / Received: "1"`. Confirms the two distinct expected values (`"1"` on row 0, `"2"` on row 1) reject a static/degenerate column, and that the post-reorder assertions are not vacuous.
- **Restored** `{{ index + 1 }}` byte-identically; `git status` clean, `git diff` empty; filtered test green again (1 passed). Verdict file is my only lasting write; probe edits fully reverted.

Report's other fire claims (1: key-absent -> `en()` helper throws; 2: key-present/column-absent -> `columnheader.first()` Expected "Order" Received "Source") are plausible and consistent with the verified helper (`e2e/i18n-en.ts:155-159` throws on a missing id) and template structure. The report followed the falsifiable-verification discipline (broke the binding, watched it fire) - `proc-verification-step-must-be-falsifiable` held.

---

## Adjudications

### Q1 - catalog section placement: PATTERN-CONFORMING (no routing owed)

The new attribute-less key sits in a new trailing `## Rule grid ordinal (D59)` section in both catalogs rather than inside the all-`.tooltip` `## TrackRule` section.

**Judgment: pattern-conforming, correct.** D59 and the brief pin the key name, values, and no-tooltip status but leave the in-file placement open - a structural fork with zero outward effect, covered by the standing house-pattern grant (`latitude-carveout-zero-content-structural-forks`). All four zero-outward-effect conditions hold: `##` lines are Fluent comments/group markers, so placement changes no API/symbol surface (the id resolves regardless of section), no data format, no verification (additive), and nothing user-visible (the section header is not rendered; the rendered value `Order`/`Reihenfolge` is the enumerated key itself, not the placement).

The chosen placement conforms to the established local pattern: `## Save surface (D41)` is the precedent - a single-key, ADR-referenced, attribute-less trailing section for a non-registry-label key; `## Generic list/map actions` is a second attribute-less trailing section. The ordinal key is exactly such a non-registry-label attribute-less key with its own ADR (D59), so its own trailing ADR-referenced section instantiates that pattern. The alternative (inside `## TrackRule`, which the shared `editor-track-rule-` prefix might suggest) would be the **deviation**: it breaks that section's every-label-carries-a-tooltip uniformity and mixes a non-label among registry labels. The implementer chose the conforming home and surfaced it in the report - the correct transparency move. No NEEDS_CONTEXT was owed.

### Q2 - gate scoped to the four frontend gates: SOUND for this diff

The implementer ran only the four frontend gates, arguing zero Rust files touched.

**Judgment: sound, no cargo-side check was owed.** Verified where `gui-editor.ftl` keys are consumed: **no Rust code embeds or enumerates `gui-editor.ftl`.** Comprehensive grep (`--include=*.rs`, whole tree) for `gui-editor` returns zero hits. The only Rust `include_str!` of GUI catalogs is `src-tauri/src/run.rs:543` embedding `gui-common.ftl` (native dialog strings, D31/D55) - not `gui-editor.ftl`; the CLI embeds only `cli.ftl`/`diagnostics.ftl` (`crates/muxsmith-cli/src/i18n.rs:7-10`), and `catalog_completeness.rs` enumerates `cli.ftl`. `gui-editor.ftl` is consumed solely by the frontend Fluent loader (`src/i18n/index.ts:18` glob `../../locales/*/gui-*.ftl`, a Vite build-time import). The new key `editor-track-rule-order` is not an `IpcError` code, so even the D61 presence gate - which lives in `scripts/check-i18n.mjs` (a Node gate, run green above) and scans `src-tauri/**/*.rs` for `IpcError::new("...")` codes - is untouched. The cargo half (fmt/clippy/test/doc/deny) is genuinely unaffected; deferring it to the controller's merge-time duty is correct. The implementer also correctly left the `editor-generic-action-keys` budget revision (45 -> 46) out of the diff - that is the controller's trigger-10 duty per the brief and product-boundaries.yaml, and the entry already reflects 46.

---

## HARVEST

- **Dominant pattern (positive):** clean instance of the zero-outward-effect house-pattern grant working as designed - the implementer resolved a placement fork the design left silent by conforming to the nearest local precedent (`## Save surface (D41)`) and surfaced it, no keyboard-level product decision, no over-reach. Verification discipline held (binding broken and watched to fire before trust).
- **Over-restriction watch:** no stop this boundary forced was unwarranted; the grant correctly covered the catalog-section placement without a needless NEEDS_CONTEXT. Calibration data for the do-not-over-restrict direction: a new trailing ADR-referenced single-key catalog section mirroring an existing one is grant-covered structural conformance, not a routable fork.
- **Ledger candidate (watch, not yet promotion-ripe):** "a new non-registry-label, attribute-less `gui-editor` key gets its own ADR-referenced trailing `##` section, never inside an all-`.tooltip` registry section." Second instance now (D41 save-note first, D59 ordinal second). Currently just an application of the existing house-pattern grant; note it in case a third instance argues for an explicit gui-editor-catalog-structure convention.
- **No repeated rejections** in this single-task diff.
