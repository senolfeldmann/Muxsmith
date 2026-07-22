# Task 15 report: D58 curated-domain dropdowns in exact-match value cells

**Verdict: DONE**
**Commit:** `9f4aa8a` (branch `plan7-f`)
**Test summary:** full frontend gate green (52 e2e passed, incl. the 10 new dropdowns cases + Task-14 markers still green); `cargo test -p muxsmith-core --features ts` 15 passed; fmt/clippy/vue-tsc/eslint all clean; CI drift gate clean.

## Files touched (exactly the four the brief names)
- `crates/muxsmith-core/tests/ts_export.rs` (`emit_settables_ts`)
- `src/bindings/settables.ts` (regenerated, committed)
- `src/editor/widgets/PropertyMapWidget.vue`
- `e2e/editor-dropdowns.spec.ts` (new)

## Per-step

**Step 1 - extend the emitter: done.** Added `TYPE_VALUES`/`CODEC_KIND_NAMES` to the `muxsmith_core::capability` import; emitted the two `as const` arrays before `std::fs::write` using the brief's verbatim snippet. Header line changed in the same emitter string to add `// type maps + curated matchable value domains` as a second comment line (kept the `@generated` marker line intact, so `keywords.ts`'s shared first line is untouched). `cargo fmt` reflowed the import to multi-line; no other change.

**Step 2 - regenerate + verify drift-gate coverage: done.** `cargo test -p muxsmith-core --features ts` rewrote `settables.ts`. `git diff --stat` = `1 file changed, 26 insertions(+)`: `TYPE_VALUES` (4 entries: audio, buttons, subtitles, video) and `CODEC_KIND_NAMES` (17 entries: srt, ass, pgs, vobsub, webvtt, aac, ac3, eac3, dts, truehd, flac, opus, mp3, h264, h265, av1, vp9) plus the header line. Emitter output is deterministic (re-run post-commit produces an empty `git diff` against HEAD -> the D44 directory-scoped CI drift gate covers it with no CI edit).

**Step 3 - failing e2e: done.** `e2e/editor-dropdowns.spec.ts` mounts `PropertyMapWidget` in isolation through the wave-3 mount harness (`mountComponent`, feeding `{ spec, path, modelValue }`). The 8-row closed matrix is one `test` each, plus the two write-path behaviours. Cell variant is identified by the value cell's tag (`SELECT` vs `INPUT`) since `data-testid="property-map-value"` is shared across variants. Pre-implementation run (`npx playwright test --grep dropdowns`, after rebuilding the mount bundle over the un-changed widget): **5 failed / 5 passed** - the 5 select-expecting cases (1, 2, 3, the select-write, the re-resolve) failed; the 5 text-cell cases (4-8) passed trivially.

**Step 4 - implement the cell resolution: done.** `PropertyMapWidget.vue`: `ValueCellKind` gains `"select"`; a `DOMAINS` map (`type`->`TYPE_VALUES`, `codec_kind`->`CODEC_KIND_NAMES`) + `domainFor(key)` helper; `cellKindFor(key, value)` (signature widened to take the value) resolves `select` **before** the scalar switch iff all four D58 conditions hold: matchable+scalar widget; `props.path` starts with `tracks[`; `key === "type" || key === "codec_kind"` (byte-exact); `value === "" || domain member`. Template: a `<select data-testid="property-map-value" :value="row.value" @change="onSelectInput">` branch ahead of the checkbox branch (checkbox flips `v-if`->`v-else-if`), rendering the domain array as options plus an empty placeholder only when `row.value === ''`; carries the same `diag-anchored--*`/`aria-invalid`/`aria-labelledby` wiring as the sibling cells. Option labels render raw (`{{ opt }}`), matching the `SelectWidget` house pattern (config vocabulary, never `$t`) - so **no new Fluent message**, and the bilingual-message constraint does not apply.

**Step 5 - run + gates: done.**
- `npx playwright test --grep dropdowns` (mount bundle rebuilt over the implemented widget): **10 passed**.
- `cargo fmt -p muxsmith-core -- --check`: exit 0.
- `cargo clippy -p muxsmith-core --features ts --tests`: clean, no warnings.
- `npx vue-tsc --noEmit`: exit 0, no output.
- `npx eslint .`: exit 0.
- `pnpm build` (dist rebuilt so the full-app specs see the change) then `pnpm test:e2e`: **52 passed, 0 failed** - includes `editor-markers.spec` (Task 14) still green, `editor-tooltips` (mounts every widget), `smoke`, and the 10 dropdowns cases. Note: the marker fixture's rule 0 carries `match.exact = { type: "video" }`, so with this change that cell now renders as a `<select value="video">` in the real app; the marker spec and its axe check still pass, which is end-to-end evidence the select integrates (accessible name via the existing `aria-labelledby`, markers still resolve).
- CI drift gate: re-run emitter post-commit -> empty `git diff`.

**Step 6 - commit: done.** `git add` of exactly the four files; `git -c commit.gpgsign=false commit` with the brief's exact message + `Co-Authored-By` trailer -> `9f4aa8a`. Working tree clean afterwards.

## Fire-verification (each new absence/count assertion watched failing, then restored)
Every deliberate break was applied to `PropertyMapWidget.vue`, the mount bundle rebuilt, the affected case run, observed failing, then reverted:
1. **Presence + option-count** (cases 1/2/3): fired naturally by the Step-3 pre-implementation run (no `<select>` exists -> `cellTag` SELECT != INPUT, `locator("option")` count wrong).
2. **Path gate** dropped (`props.path.startsWith("tracks[")` -> `true`): **case 6** failed (attachment path flipped to SELECT). Restored.
3. **`values === "scalar"` gate** dropped in the select condition (-> `true`): **case 7** failed (substring cell flipped to SELECT). Restored.
4. **Byte-exact key gate** loosened (`key === "type"` -> `key.includes("type")`): **case 4** failed (`raw:type` flipped to SELECT). Restored.
5. **Domain-membership gate** dropped (`value === "" || typeof value === "string"`): **case 5** failed (`type: vido` flipped to SELECT). Restored.
6. **Placeholder guard** forced on (`v-if="row.value === ''"` -> `v-if="true"`): **case 2** failed (option count 5 vs expected 4). Restored.

Final state after all restores: `npx playwright test --grep dropdowns` -> 10 passed; full `pnpm test:e2e` -> 52 passed.

## Brief-vs-tree divergences / surfaced items
- **No design fork; no premise refuted.** Task 14's `path?: string` prop and `rowAnchors` are present in the worktree widget (HEAD `ff49658`) exactly as the brief assumes; consuming `props.path` for the gate needed no adaptation.
- **Header line placement (minor, in-scope):** the brief said "update the generated header line to 'type maps + curated matchable value domains' in the same emitter string". `settables.ts` had no descriptive header, only the `@generated` marker line. Implemented as a second comment line appended in the same `String::from(...)`, leaving the `@generated` marker line unchanged so it stays identical to `keywords.ts`'s first line. Zero outward effect beyond the committed generated file.
- **`cellKindFor` signature widened** from `(key)` to `(key, value)` to evaluate D58 condition 4 (value membership); all template call sites updated. Internal to the component, no outward surface change (structural-conformance grant).
- **Select uses `:value` + `@change`**, not `v-model` - matching this widget's existing per-row `:value`/`@input` convention (the row value lives in a `Record`, not a single model). Verified against the installed Vue 3.5.39 source (`runtime-core.cjs.js` `mountElement`) that children mount before `value` is patched (and `value` is deferred to last), so `:value` on `<select>` selects the matching option correctly on initial render.
