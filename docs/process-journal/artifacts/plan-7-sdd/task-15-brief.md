### Task 15: D58 - curated-domain dropdowns in exact-match value cells

Read D58 in full and `gui-closed-domain-dropdowns` (`product-boundaries.yaml:420`).

**Files:**
- Modify: `crates/muxsmith-core/tests/ts_export.rs` (`emit_settables_ts`)
- Modify: `src/bindings/settables.ts` (regenerated, committed, never hand-edited)
- Modify: `src/editor/widgets/PropertyMapWidget.vue`
- Test: extend `e2e/editor-markers.spec.ts`? No - new `e2e/editor-dropdowns.spec.ts`

**Interfaces:**
- Consumes: `capability::TYPE_VALUES` (4) / `capability::CODEC_KIND_NAMES` (17, derived from `CODEC_KINDS`), Task 14's `path` prop.
- Produces: `TYPE_VALUES`, `CODEC_KIND_NAMES` consts in `settables.ts`.

- [ ] **Step 1: Extend the emitter.** In `emit_settables_ts` (`ts_export.rs:52`, the existing `out.push_str` style), before the final `std::fs::write`, emit the two domain arrays sourced from the capability constants (never hand-written in TS, D46; add `TYPE_VALUES`/`CODEC_KIND_NAMES` to the file's existing `muxsmith_core::capability` imports):

```rust
    out.push_str("export const TYPE_VALUES = [\n");
    for v in TYPE_VALUES {
        out.push_str(&format!("  \"{v}\",\n"));
    }
    out.push_str("] as const;\n");

    out.push_str("export const CODEC_KIND_NAMES = [\n");
    for v in CODEC_KIND_NAMES.iter() {
        out.push_str(&format!("  \"{v}\",\n"));
    }
    out.push_str("] as const;\n");
```

Update the generated header line to "type maps + curated matchable value domains" in the same emitter string.

- [ ] **Step 2: Regenerate + verify drift-gate coverage.**

Run: `cargo test -p muxsmith-core --features ts`
Expected: `settables.ts` rewritten with the two new consts (4 and 17 entries - recount in the diff). `git diff --stat src/bindings/settables.ts` shows the change; commit it as generated output.

- [ ] **Step 3: Write the failing e2e.** `e2e/editor-dropdowns.spec.ts` (mount-harness over `PropertyMapWidget` with `matchExprFields.exact`'s spec, or the full editor app - use the harness): the closed condition matrix, each a test case:

| case | widget | path | key | value | expected cell |
|---|---|---|---|---|---|
| 1 | exact (matchable+scalar) | `tracks[0].match.exact` | `type` | `""` | `<select>` with 4 domain options + one empty placeholder |
| 2 | same | same | `type` | `audio` | `<select>`, `audio` selected |
| 3 | same | same | `codec_kind` | `""` | `<select>` with 17 options + placeholder |
| 4 | same | same | `raw:type` | `""` | text input (byte equality fails - `raw:` bypass preserved) |
| 5 | same | same | `type` | `vido` | text input with `vido` intact (out-of-domain never eaten) |
| 6 | same | `attachments.rules[0].select` | `type` | `""` | text input (path gate: not a track context) |
| 7 | substring (matchable+string) | `tracks[0].match.substring` | `type` | `""` | text input (outside the decree's boundary) |
| 8 | changes (settable+scalar) | `tracks[0].changes` | `language` | `""` | unchanged existing typed cell (settables carry no `type`/`codec_kind`) |

Plus: selecting `video` in case 1 writes the string value exactly as the text cell does (assert the emitted model update), and after correcting case 5's value to `video` via the text input, the cell re-resolves to a `<select>`.

Run: `pnpm test:e2e -- --grep "dropdowns"`. Expected: FAIL.

- [ ] **Step 4: Implement the cell resolution.** In `PropertyMapWidget.vue`: extend the kind union and resolver - `select` resolved BEFORE the scalar-type switch, iff all four hold: (1) `props.spec.widget.properties === "matchable" && props.spec.widget.values === "scalar"`; (2) `props.path?.startsWith("tracks[")` (the D57 path prop - keeps the dropdown out of the attachment context where the type table is wrong, ground-truth flaw note); (3) `key === "type" || key === "codec_kind"` (byte equality); (4) `value === ""` or the value is a member of the key's domain (`TYPE_VALUES` / `CODEC_KIND_NAMES`). Template: the `select` cell renders `<select>` with the domain array as options plus, when the current value is `""`, one empty placeholder option; writing goes through the same update path as the text cell.

- [ ] **Step 5: Run the spec, then fmt/clippy and the full frontend gate.** Expected: PASS / green (the CI drift gate re-runs the emitter and finds no diff).

- [ ] **Step 6: Commit**

```bash
git add crates/muxsmith-core/tests/ts_export.rs src/bindings/settables.ts src/editor/widgets/PropertyMapWidget.vue e2e/editor-dropdowns.spec.ts
git -c commit.gpgsign=false commit -m "editor: type/codec_kind dropdowns in exact-match track cells, domains emitted beside the type maps (D58)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

