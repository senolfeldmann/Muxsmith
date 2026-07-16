### Task 5: D44 - ts-rs bindings, committed, with a CI drift check (the wave-1 join)

Runs **after streams A and B merge to master**. It is the join of the model.rs chain (Task 4, carrying Task 3's `KEYWORDS`) and the `StructuredEdit` chain (Task 6), because it owns **all** ts concerns: the `ts` feature, the derives on the 20 model types, and the derive on `StructuredEdit` in `planner.rs`.

**Files:**
- Modify: `crates/muxsmith-core/Cargo.toml` (`ts-rs` optional dep + `ts` feature)
- Create: `.cargo/config.toml` (`[env]` block)
- Modify: `crates/muxsmith-core/src/profile/model.rs`, `crates/muxsmith-core/src/profile/match_expr.rs` (cfg_attr TS derives on the 20 model types)
- Modify: `crates/muxsmith-core/src/planner.rs` (the `ts` import + the cfg_attr TS derive on `StructuredEdit` - the one ts concern Task 6 deferred)
- Create: `crates/muxsmith-core/tests/ts_export.rs` (the export test + the keywords emitter)
- Create (committed, generated): `src/bindings/profile.ts`, `src/bindings/keywords.ts`
- Modify: `.github/workflows/ci.yml` (new Linux-leg drift step)

**Interfaces:**
- Consumes: Task 3's four `KEYWORDS` constants; Task 6's final `StructuredEdit` shape.
- Produces, for Tasks 9-13: `src/bindings/profile.ts` (the 20 model types **plus** `StructuredEdit`, 21 types total) and `src/bindings/keywords.ts` (`FILENAME_KEYWORDS`, `SOURCE_KEYWORDS`, `CHAPTERS_KEYWORDS`, `TITLE_KEYWORDS` as `as const` arrays).

**Read first:** design D44 (`:498-682`) in full - it carries the `[env]` block, the emitter, the drift step, and the measured evidence for each - and D49 §"Interface changes" > "What a ts-rs binding emits" (`:1201-1218`), which pins `StructuredEdit`'s generated shape.

Binding points:
- Generation is `cargo test -p muxsmith-core --features ts`, **not** an xtask, and **the reason is feature unification, not taste**: xtask would need `muxsmith-core = { features = ["ts"] }`, and Cargo unifies features across workspace members within one invocation, so `cargo build --workspace` would enable `ts` for every consumer of core and put `ts-rs` into the shipped cli and src-tauri builds. `-p muxsmith-core --features ts` cannot leak that way.
- Bindings are **committed, not built** (`core-06-schema-build-time-extraction` already mandates a committed generated artifact).
- `TS_RS_LARGE_INT = "number"` is **mandatory**: without it `Scalar::Int(i64)` maps to `bigint`, which does not survive `JSON.stringify` and would break the IPC wire at the model's most-used point.
- `export_to = "profile.ts"` without a trailing slash names a **file**, so all 21 types land in one file with no cross-imports.
- **`StructuredEdit`'s ts derive is added here, in `planner.rs`** (Task 6 deferred it): add `#[cfg(feature = "ts")] use ts_rs::TS;` and the `#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]` on `StructuredEdit`, per D49 §"The wire shape". The `cfg` on the import is not optional: a bare `use ts_rs::TS;` is an unused import on a default build and fails `cargo clippy -- -D warnings`.

- [ ] **Step 1: Add the dependency and feature**

In `crates/muxsmith-core/Cargo.toml`: `ts-rs = { version = "12.0.1", optional = true }` and a `ts = ["dep:ts-rs"]` feature. Before writing the version, re-verify it against the registry rather than trusting this plan (`proc-07-verify-against-source`):

```bash
curl -s https://crates.io/api/v1/crates/ts-rs | python3 -c "import json,sys; print(json.load(sys.stdin)['crate']['max_stable_version'])"
```
Expected: `12.0.1`. If it differs, that is a NEEDS_CONTEXT, not a silent bump - `ci-10-pin-everything` binds.

- [ ] **Step 2: Prove the default build stays clean**

The whole isolation claim rests on this. Measure it now, before the derives exist, and again in step 7:

```bash
cargo tree -p muxsmith-core | grep -c ts-rs
```
Expected: `0`.

- [ ] **Step 3: Create `.cargo/config.toml`**

```toml
[env]
TS_RS_EXPORT_DIR = { value = "src/bindings", relative = true }
TS_RS_LARGE_INT = "number"
```

- [ ] **Step 4: Add the derives**

`#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]` on each of the 20 model types - the 13 structs in D45's table (`:768-782`) plus the 7 enums (`FilenameCfg`, `SourceCfg`, `ChaptersCfg`, `TitleCfg`, `CollisionPolicy`, `KeepDrop`, `Scalar`) - **and** on `StructuredEdit` in `planner.rs` (the 21st type, D49), together with the `#[cfg(feature = "ts")] use ts_rs::TS;` import there. That set is the whole model reachable from `Profile` plus the one wire type the shell accepts back, with no residue.

- [ ] **Step 5: Write the export test and the keywords emitter**

Create `crates/muxsmith-core/tests/ts_export.rs`. ts-rs's `#[ts(export)]` writes `profile.ts` as a test side effect; the emitter for `keywords.ts` is ~12 lines and the design gives it verbatim at `:556-570`. It reads `TS_RS_EXPORT_DIR` so both artifacts share one destination. `ts-rs` exports **types, not values**, which is why the four keyword arrays need an emitter at all.

- [ ] **Step 6: Generate and commit the bindings**

```bash
cargo test -p muxsmith-core --features ts
ls -1 src/bindings/
```
Expected: `keywords.ts` and `profile.ts`. Inspect `profile.ts` and confirm two things: `Scalar` emits `boolean | number | number | string` (the duplicate `number` is cosmetic and expected - a `bigint` anywhere means `TS_RS_LARGE_INT` is not reaching the build), and `StructuredEdit` emits with `value: Scalar` on its two `add_exact`/`add_not_exact` members exactly as D49 measured (`:1214-1218`). Because the drift-check hole (a never-committed first-generation file) is not closed by the CI step until wave 3 (see step 8), this `ls` plus inspection **is** the first-generation check in-task.

- [ ] **Step 7: Re-prove the isolation**

```bash
cargo tree -p muxsmith-core | grep -c ts-rs
```
Expected: still `0` - the derives are behind the feature.

- [ ] **Step 8: Add the CI drift step**

In `.github/workflows/ci.yml`, on the **Linux leg only** (matching the existing `check:i18n` and Playwright gating):

```yaml
      - name: TS bindings are not stale
        if: runner.os == 'Linux'
        run: |
          cargo test -p muxsmith-core --features ts
          git diff --exit-code src/bindings/
```

Add a comment recording the step's one known hole, so it is not rediscovered as a bug: `git diff --exit-code` does not see a **new untracked** file (measured, D44 `:614-631`), so the gate catches a *stale* committed artifact from the first commit onward but cannot catch a never-committed first-generation one. **That hole is closed only in wave 3**, when Task 9's `src/editor/registries.ts` imports `keywords.ts` and a missing file fails the TypeScript build (`pnpm build` = `vue-tsc --noEmit && vite build`); between this merge and Task 9 nothing imports the bindings, so no gate closes it for two waves. Until then, step 6's `ls` + inspection is the check. `git status --porcelain` would close it directly and is deliberately **not** adopted: it would also fire on unrelated untracked files and turn every CI leg into a working-tree cleanliness assertion.

- [ ] **Step 9: Prove the drift check actually catches drift**

```bash
printf '\n// drift\n' >> src/bindings/keywords.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"
# Expected: exit=1  (the gate fires)
git checkout src/bindings/keywords.ts
git diff --exit-code src/bindings/ ; echo "exit=$?"
# Expected: exit=0
```

- [ ] **Step 10: Full gate, then commit**

```bash
git add crates/muxsmith-core/Cargo.toml Cargo.lock .cargo/config.toml crates/muxsmith-core/src/profile/model.rs crates/muxsmith-core/src/profile/match_expr.rs crates/muxsmith-core/src/planner.rs crates/muxsmith-core/tests/ts_export.rs src/bindings/profile.ts src/bindings/keywords.ts .github/workflows/ci.yml
git -c commit.gpgsign=false commit -m "core: ts-rs generates the wire types (model + StructuredEdit) behind a ts feature, committed + CI drift-checked (D44, D49)"
```

---

## Wave 2

Streams A, B and C merge to master and the join (Task 5) lands, gate green after each merge. Then:

