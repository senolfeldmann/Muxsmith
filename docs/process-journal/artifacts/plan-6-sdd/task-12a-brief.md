### Task 12a: the settable-property-type binding (new, 2026-07-16 owner-rulings routing)

**Stream E** (`.worktrees/plan6-e`), sequenced after Task 11 and before Task 12. It is the vehicle for owner Ruling 2's type source (Amendment 2): the typed value cells need a settable-property -> scalar-type map on the frontend, and hand-mirroring `capability::SETTABLE` in TypeScript is banned (the exact D44/D46 drift class). Per this plan's trigger 4 ("a second Muxsmith artifact needs TypeScript types -> extend the `ts` feature's export set rather than hand-mirroring again"), Task 5's ts-rs emitter is extended to emit one more committed binding. It is a separate step from Task 12 because it is a core-crate + generated-artifact + CI-drift concern (Task 5's layer, not the frontend's), and its output must be **tracked before** Task 12 imports it (`git-diff-proof-needs-tracked-target`, and the TypeScript build resolves the import only once the file exists).

**Files:**
- Modify: `crates/muxsmith-core/src/capability/mod.rs` (a documented public `matchable_properties()` accessor over the private `generated` table)
- Modify: `crates/muxsmith-core/tests/ts_export.rs` (a second value emitter beside `emit_keywords_ts`, emitting both property-type tables)
- Create (committed, generated): `src/bindings/settables.ts`

**Interfaces:**
- Consumes: `muxsmith_core::capability::{SETTABLE, PropType, matchable_type}` (`pub`, `crates/muxsmith-core/src/capability/mod.rs:22,72,40`) and the new `matchable_properties()` accessor this task adds.
- Produces, for Task 12: `src/bindings/settables.ts` exporting `SETTABLE_TYPES` (settable-name -> scalar-type-tag), `MATCHABLE_TYPES` (matchable-name -> scalar-type-tag), and the shared `PropScalarType` union type - one committed binding file carrying both tables (owner ruling: one file total).

**Read first:** design D44 (`:498-682`) for the emitter mechanism and the committed-binding + CI-drift contract; Task 5's `emit_keywords_ts` (`crates/muxsmith-core/tests/ts_export.rs`) for the exact emit-a-value-artifact shape this extends (ts-rs exports types, not values, so a value map needs a hand-written emitter); the house rule `git-diff-proof-needs-tracked-target` (`docs/decision-ledger.yaml`) for Step 3's staging precondition.

Binding points:
- **The type sources are `capability::SETTABLE` and the matchable table; hand-mirroring either list in TypeScript is banned** (the exact D44/D46 drift class that D46's generated keyword arrays and D44's generated bindings exist to prevent). `SETTABLE` is `&[(&str, PropType, &str)]` (10 rows). The matchable table is `generated::MATCHABLE_PROPERTIES` (`&[(&str, PropType)]`, 62 rows) **plus** the `codec_kind` virtual (`matchable_type("codec_kind") == String`, `mod.rs:41-43`) = **63** matchable props.
- **`generated` is a private module** (`mod.rs:9`, no `pub`), so `MATCHABLE_PROPERTIES` is not reachable from a `tests/` binary. This task adds a small documented public accessor `pub fn matchable_properties() -> &'static [(&'static str, PropType)] { generated::MATCHABLE_PROPERTIES }` to `capability/mod.rs`, keeping `generated` private while exposing the table - consistent with `SETTABLE`/`ATTACHMENT_PROPERTIES` already being `pub` and with the `settable()`/`matchable_type()` accessor style. The emitter reads it and appends `("codec_kind", matchable_type("codec_kind"))` so the virtual's tag is derived from its source arm, never re-asserted (the can-never-drift house principle).
- **`PropType -> tag` is an exhaustive `match`, so a new `PropType` variant fails the Rust build** (the house never-arm at the Rust layer, the CODEC-derivation shape D46 uses): `String -> "string"`, `Boolean -> "boolean"`, `Integer -> "integer"`, `Float -> "float"`. All four tags occur in `MATCHABLE_TYPES` (63 props: 22 String incl `codec_kind`, 9 Boolean, 27 Integer, 5 Float, tree-verified); `SETTABLE_TYPES` holds only `String`/`Boolean` today. The shared `PropScalarType` union names all four, so Task 12's widget switch is total over `PropType`'s whole domain (a later Integer settable, or a schema-regenerated new type, needs no widget change).
- **No `.github/workflows/ci.yml` change.** The Task-5 drift step (`ci.yml:134-138`) already runs `cargo test -p muxsmith-core --features ts` then `git diff --exit-code src/bindings/`, which covers `settables.ts` the moment it is committed. The untracked-first-generation hole D44 records still applies until then; Step 3 closes it in-task by staging before proving, and the commit closes it for CI.

- [ ] **Step 1: Add the accessor, then extend the emitter**

First, in `crates/muxsmith-core/src/capability/mod.rs`, add the documented public accessor `matchable_properties()` (binding point above) beside `matchable_type`; `generated` stays private. Then in `crates/muxsmith-core/tests/ts_export.rs`, add `use muxsmith_core::capability::{PropType, SETTABLE, matchable_type, matchable_properties};`, a private `fn scalar_tag(t: PropType) -> &'static str` with the exhaustive four-arm match above, and a second `#[test] fn emit_settables_ts()` mirroring `emit_keywords_ts`'s structure: read `TS_RS_EXPORT_DIR`, write a `// @generated by ...` header, then `export type PropScalarType = "boolean" | "integer" | "float" | "string";`, then two `export const ... = {` ... `} as const satisfies Record<string, PropScalarType>;` blocks - `SETTABLE_TYPES` (one `"<name>": "<scalar_tag>",` line per `SETTABLE` triple) and `MATCHABLE_TYPES` (one line per `matchable_properties()` row in its declaration order, then a final `"codec_kind": "<scalar_tag(matchable_type("codec_kind"))>"` line). Quote the keys, so a non-identifier property name stays valid TS. Write to `Path::new(&dir).join("settables.ts")` (one file, both tables).

- [ ] **Step 2: Generate and inspect (the first-generation check)**

```bash
cargo test -p muxsmith-core --features ts
ls -1 src/bindings/
```
Expected: `keywords.ts`, `profile.ts`, `settables.ts`. Open `settables.ts` and confirm from its content (numbers tree-verified against `generated.rs`): `SETTABLE_TYPES` has **10** rows (one per `SETTABLE`), with `forced_track`/`default_track` -> `"boolean"` and `language`/`track_name`/`sub_charset` -> `"string"`; `MATCHABLE_TYPES` has **63** rows - 22 `"string"` (incl. `codec_kind`), 9 `"boolean"`, 27 `"integer"`, 5 `"float"` - with the spot-checks `codec_kind` -> `"string"`, `forced_track`/`default_track` -> `"boolean"`, `audio_channels` -> `"integer"`, `min_luminance`/`max_luminance` -> `"float"`; and `PropScalarType` names all four tags. Because nothing imports the file until Task 12, this inspection **is** the first-generation correctness check (the CI drift step cannot see an untracked file - D44's recorded hole).

For the type-count spot-check you can cross-check against the tree directly: `for t in String Boolean Integer Float; do printf "%s " "$t"; grep -cE "PropType::$t\b" crates/muxsmith-core/src/capability/generated.rs; done` yields 21/9/27/5 (String is 22 in `MATCHABLE_TYPES` after the `codec_kind` virtual is appended).

- [ ] **Step 3: Stage, then prove the drift check catches drift**

Stage first: the proof is a `git diff --exit-code`, and against an **untracked** path it exits 0 regardless of content, silently inverting the expected red (`git-diff-proof-needs-tracked-target`).

```bash
git add src/bindings/settables.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"     # Expected: exit=0 (index == working tree)
printf '\n// drift\n' >> src/bindings/settables.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"     # Expected: exit=1 (the gate fires)
git checkout -- src/bindings/settables.ts
cargo test -p muxsmith-core --features ts               # regenerate a clean tree
git diff --exit-code src/bindings/ ; echo "exit=$?"     # Expected: exit=0
```
Read the fire from the OUTPUT (`exit=1` on the appended file), not from its absence, and record it in the report.

- [ ] **Step 4: Full gate, then commit**

Run the nine-part gate (the default `cargo test --workspace` does not run the `ts` feature; `pnpm build` typechecks the new `settables.ts` standalone even though Task 12 is its first importer). Then:

```bash
git add crates/muxsmith-core/src/capability/mod.rs crates/muxsmith-core/tests/ts_export.rs src/bindings/settables.ts
git -c commit.gpgsign=false commit -m "core: emit the settable + matchable property-type bindings for the editor's typed value cells (D44 trigger 4, owner rulings 2026-07-16)"
```

---

