# Task 20 report — D61 number promotion: `ParamValue` end to end

**Stream H** (`.worktrees/plan7-h`, branch `plan7-h`). **Verdict: DONE_WITH_CONCERNS** (one brief-file-list omission, surfaced below; no design fork).

**Commit:** `900db87971b0703202864daaae297faa2dfec2d3` (`900db87`), 10 files, +115/-35.

---

## What changed

- **`src-tauri/src/error.rs`**: added `pub enum ParamValue { Num(u64), Str(String) }` with `#[serde(untagged)]` and `From<&str>/From<String>/From<usize>` impls (verbatim from brief Step 2); `IpcError.params` field type `HashMap<String, String>` -> `HashMap<String, ParamValue>`; `.with` signature `value: impl Into<String>` -> `impl Into<ParamValue>`; field doc updated to describe the numeric-vs-string split. Two of the four promotion sites (`RuleIndexOutOfRange` `index`/`rules`, `EditChangedNothing` `index`) lost their `.to_string()`.
- **`src-tauri/src/run.rs`**: the fourth promotion site — `get_job_log_in`'s `not_found` closure `.with("index", index.to_string())` -> `.with("index", index)`.
- **`src/ipc.ts`**: `IpcError.params: Record<string, string>` -> `Record<string, string | number>` + doc note. `Diagnostic.params` left `Record<string, string>` (untouched, per spec 5.2 / design correction #4).
- **`src/components/SettingsDialog.vue`, `src/views/BatchView.vue`, `src/views/EditorView.vue`**: the three IpcError params `ref` sites `Record<string, string>` -> `Record<string, string | number>`.
- **`locales/en/gui-common.ftl`, `locales/de/gui-common.ftl`**: `apply-rule-index-out-of-range` gains the CLDR plural selector on `$rules` (en + de, same commit). The only message that gains a selector (`$index` stays a bare identifier).
- **`e2e/mocks.ts`**: `MockResult` reject shape and `rejectWith` param type `Record<string, string>` -> `Record<string, string | number>`.
- **`src-tauri/src/lib.rs`** (NOT in the brief's file list — see Divergence 1): one test's two `IpcError.params` assertions adapted to `ParamValue`; test-scoped `use crate::error::ParamValue;` added.

---

## Per-step

| Step | Outcome |
|---|---|
| 1 Failing Rust test | Done. Added `params_serialize_untagged_numbers_and_strings` verbatim. Fire-verified: fails to compile (`the trait From<usize> is not implemented for String` at the `.with("rules", 2usize)` bound). |
| 2 Implement `ParamValue` | Done, verbatim from brief. |
| 3 Promotion-site set | Done. All four sites (error.rs `index`/`rules`/`index`, run.rs `index`), no others; every other `.with` site stays a string via `From<&str>`/`From<String>`. |
| 4 Rust test + shell suite | Done. Green; fmt/clippy clean. |
| 5 TS side | Done. `ipc.ts` + three ref sites. The five direct render sites + three ref-fed `$t` sites pass params through unchanged (verified each only forwards `.params` to `$t`, no value consumed as a string). |
| 6 Plural selector | Done, en + de, verbatim. |
| 7 e2e mocks sweep | Done. Two type sites changed; sweep found no stringly-numeric IpcError fabrications (only `Diagnostic` fabrications, whose wire stays `Record<string,string>`). Grep fire-verified. |
| 8 Full gate | Done. Both gates green (evidence below). |
| 9 Commit | Done. `900db87`. |

---

## Brief-vs-tree divergences

### 1. `src-tauri/src/lib.rs` touched (not in the brief's file list) — CONCERN

The brief's Files list and Step 9 `git add` name only `error.rs` and `run.rs` on the Rust side. But `lib.rs`'s test `detect_mkvmerge_body_too_old_carries_found_and_minimum` (lines 927-928) asserts directly on `IpcError.params` values:

```
assert_eq!(err.params["minimum"], "86.0");
assert!(err.params["found"].contains("v50.0.0"));
```

The mandated field-type change (`params: HashMap<String, ParamValue>`) makes both fail to compile (`ParamValue` has no `PartialEq<&str>`, no `.contains`). The workspace cannot compile without adapting them, so lib.rs had to be edited and staged. Zero outward effect (test-only, semantics preserved: `minimum` -> `ParamValue::Str("86.0".into())`, `found` -> `matches!(&err.params["found"], ParamValue::Str(s) if s.contains("v50.0.0"))`). I added `lib.rs` to the explicit `git add` list beyond the brief's snippet. No design fork — no outward-behavior option exists.

### 2. Existing `error.rs` test assertions adapted (same compile-forced class)

Roughly a dozen assertions of the form `assert_eq!(err.params["<key>"], "<literal>")` in `error.rs`'s test module break the same way. Adapted to construct the expected `ParamValue` (`ParamValue::Str(...)` for string params; `ParamValue::Num(7)`/`Num(1)`/`Num(0)` for the now-numeric `oob`/`noop` `index`/`rules`). No assertion weakened, deleted, or skipped — each is the same logical check re-typed. The brief's own new test (Step 1) sidesteps this by comparing through `serde_json::to_value`, but the pre-existing tests index the `HashMap` directly and had to change.

**Design choice (surfaced):** I did NOT add ergonomic `PartialEq<&str>`/`PartialEq<u64>` impls to `ParamValue` to keep the old assertions verbatim. That would minimize test churn but expand a `pub` type's API surface beyond the design's exact-contract enum, and the `.contains` assertion in lib.rs needs `matches!` regardless (a `PartialEq` impl doesn't help it). Wrapping keeps zero new surface. Reviewer input welcome if the ergonomic impls are preferred.

### Re-verified anchors (wave 2 shifted lines; located by quoted text)

| Brief anchor | Actual (this worktree) | Quoted text matched |
|---|---|---|
| `error.rs:169` `index` (RuleIndexOutOfRange) | line 169 | `.with("index", index.to_string())` ✓ |
| `error.rs:170` `rules` | line 170 | `.with("rules", rules.to_string())` ✓ |
| `error.rs:174` `index` (EditChangedNothing) | line 174 | `.with("index", index.to_string())` ✓ |
| `run.rs:935` `index` | line 935 | `.with("index", index.to_string())` ✓ |
| `SettingsDialog.vue:16` `errorParams` | line **17** | `const errorParams = ref<Record<string, string>>({});` |
| `BatchView.vue:48` `ipcErrorParams` | line 48 | `const ipcErrorParams = ref<Record<string, string>>({});` |
| `EditorView.vue:121-122` `ipcErrorParams` | line **124** | `const ipcErrorParams = ref<Record<string, string>>({});` |
| `e2e/mocks.ts` reject shape / `rejectWith` | lines 28, 34 | both `Record<string, string>` ✓ |
| `apply-rule-index-out-of-range` en | `gui-common.ftl:66` | single-line, no selector ✓ |
| `apply-rule-index-out-of-range` de | `gui-common.ftl:44` | single-line, no selector ✓ |

Rust promotion-site anchors and BatchView were exact; the two other Vue refs and design-cited lines drifted by 1-3 lines (wave-2 shift). All located by quoted text.

### Confirmed premises

- `ApplyError::{RuleIndexOutOfRange{index,rules}, EditChangedNothing{index,..}}` fields are `usize` (`crates/muxsmith-core/src/planner.rs:1940/1942/1953`); `get_job_log_in`'s `index: usize` (run.rs:925). So `From<usize>` covers all four sites. `UnparsableConfigPath(String)` and `run_id: &str` stay strings via `From<String>`/`From<&str>`.
- `IpcError` is gui-crate-local (`muxsmith_gui_lib`): no `src-tauri/tests/`, no other crate references it. All params assertions live in `error.rs` + `lib.rs` (adapted) and `run.rs` (asserts `.code` only for `job-log-not-found`, no params — unaffected).
- Stream G's check-i18n rule 5 is NOT yet in this worktree. Current `scripts/check-i18n.mjs` check 3 is id-set parity only and explicitly ignores multiline pattern bodies (indented continuation lines register no bogus id). The selector's `[one]`/`*[other]` lines are indented, so they pass the old check. The real-Fluent-parse test (`e2e/catalogs.spec.ts` -> `assertAllCatalogsParseCleanly`) validates the selector syntax and passed.
- All five direct render sites (`FirstRun.vue:94`, `RunHistory.vue:155`/`:241`, `JobsView.vue:249`/`:255`) and the three ref-fed `$t` sites forward `.params` to `$t` unchanged; none consume a params value as a string. Widening is transparent to them (no out-of-scope edits). Confirmed by vue-tsc (`pnpm build`) passing.

---

## Fire-verification evidence

- **Step 1 Rust test** (watch-fail): before implementing `ParamValue`, `cargo test -p muxsmith-gui params_serialize` failed to compile: `the trait bound 'String: From<usize>' is not satisfied ... required by a bound in 'error::IpcError::with'` at `.with("rules", 2usize)`. Expected value in the assertion (`serde_json::json!(3)`, a JSON number) differs from the pre-change environment default (index serialized as the string `"3"`). After implementing: `test error::tests::params_serialize_untagged_numbers_and_strings ... ok`.
- **Step 7 grep** (watch-fire): the sweep `grep -rn "rejectWith\|params:" e2e/*.ts | grep -E "index|rules"` returns only `Diagnostic` fabrications on the real tree. To confirm it fires on the target shape, I appended `// FIRE-VERIFY-TEMP: rejectWith("apply-rule-index-out-of-range", { index: "3" })` to `e2e/smoke.spec.ts`, re-ran the grep — it matched the planted line (`e2e/smoke.spec.ts:1500`) — then restored the file via `git checkout` (residue grep count 0, file diff-clean vs HEAD). The grep catches the exact `{ index: "3" }` fabrication shape it is meant to guard.

---

## Commands and results

### Rust gate (foreground)
- `cargo test -p muxsmith-gui params_serialize` (pre-impl): **compile FAIL** (expected — `From<usize>` missing on `String`).
- `cargo test -p muxsmith-gui params_serialize` (post-impl): **ok. 1 passed; 0 failed** (81 filtered out).
- `cargo fmt --all --check`: after auto-`cargo fmt --all` (reformatted two of my wrapped assertions; `git diff --stat` confirmed only `error.rs`/`lib.rs`/`run.rs` touched), **clean** (exit 0).
- `cargo clippy --workspace --all-targets -- -D warnings`: **exit 0**, 0 warnings/errors.
- `cargo test --workspace`: **exit 0**. gui crate lib tests: `test result: ok. 82 passed; 0 failed`. No failures anywhere in the workspace.

### Frontend gate (foreground)
- `pnpm lint` (eslint): **exit 0**.
- `pnpm build` (`vue-tsc --noEmit && vite build`): **exit 0**, `✓ built`. (vue-tsc is the type gate that catches the `string | number` widening and any null-narrowing regression — clean.)
- `pnpm check:i18n`: **exit 0** — `check-i18n: ok (41 source files scanned, 211 catalog ids, 17 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s))`. The 17 unused warnings are pre-existing (non-fatal). No parity violation from the new selector.
- `pnpm test:e2e` (`tsc -p e2e/tsconfig.json && vite build x2 && playwright test`): **exit 0** — `52 passed`. Includes `e2e/catalogs.spec.ts:12 › all Fluent catalogs parse cleanly` (validates the new selector's Fluent syntax) and `e2e/smoke.spec.ts:307 › diagnostics summary and suggestions-capped pluralize their counts` — both green.

---

## Cross-task constraint check

The null-narrowing rule (a `T | null`-derived IPC field feeding a string-typed sink needs narrowing, only vue-tsc catches it): my change introduces no such sink — it widens `Record<string, string>` to `Record<string, string | number>`; no nullable field is newly routed to a string placeable/arg/prop. `pnpm build` (vue-tsc) passing confirms no regression of this class.

---

## Surfaced items summary

1. **lib.rs added to the commit beyond the brief's file list** — compile-forced by the mandated type change; the brief's Rust-file recon (error.rs + run.rs) missed the lib.rs `IpcError.params` test assertion. (Divergence 1.)
2. **~a dozen existing error.rs test assertions re-typed** to `ParamValue` — compile-forced, semantics preserved, none weakened. (Divergence 2.)
3. **Deliberate choice not to add `PartialEq<&str>`/`PartialEq<u64>` to `ParamValue`** — keeps the pub enum at the design's exact contract; test churn absorbed by wrapping instead. Flagged for reviewer preference.
