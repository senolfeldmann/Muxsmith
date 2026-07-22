# Task 20 verdict — D61 number promotion: `ParamValue` end to end

**Reviewer:** independent (stream H, worktree `.worktrees/plan7-h`, HEAD `900db87`, parent `7c29957`).
**Commit under review:** `900db87` — 10 files, +115/-35 (stat reproduced against the tree).

## Combined verdict: APPROVED

- **Spec compliance: PASS.** The `ParamValue` promotion is complete end to end, every param site and both wire directions; nothing beyond scope except the one compile-forced `lib.rs` test adaptation, which is correct and surfaced.
- **Quality: PASS.** All seven gates green under my own foreground build; no findings at any severity.

No fixes required.

---

## Gate runs (my build, foreground, worktree HEAD 900db87)

| Gate | Result |
|---|---|
| `cargo fmt --all --check` | exit 0 (clean) |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, 0 warnings |
| `cargo test --workspace` | exit 0; gui crate 82 passed; target tests `params_serialize_untagged_numbers_and_strings`, `apply_errors_map_to_distinct_codes`, `too_old_carries_found_and_minimum`, `detect_mkvmerge_body_too_old_carries_found_and_minimum` all ok |
| `pnpm lint` (eslint) | exit 0 |
| `pnpm build` (vue-tsc --noEmit && vite) | exit 0, built — vue-tsc is the type gate for the `string \| number` widening and the null-narrowing class; clean |
| `pnpm check:i18n` | exit 0, ok (211 catalog ids; 17 unused warnings, pre-existing, non-fatal); new selector does not trip check 3 |
| e2e prep (`tsc -p e2e/tsconfig.json`; harness + mount vite builds) | all exit 0 (e2e tsc covers `mocks.ts`'s widened type) |
| filtered e2e `npx playwright test --grep "parse cleanly\|pluralize"` | 2 passed — `catalogs.spec.ts:12 all Fluent catalogs parse cleanly` (guards the new selector's Fluent syntax, every locale) + `smoke.spec.ts:307` pluralize |

Build caches present (target 8.1G, node_modules), so runs were incremental. Tree clean before and after (one probe edit to `lib.rs`, restored byte-identically via `git checkout`; `git status --porcelain` empty).

---

## Fire-verification of the report's claims (all reproduced / confirmed plausible)

1. **Step-1 watch-fail mechanism (report §Fire-verification):** consistent with observed behaviour — reverting the `lib.rs` assertions to the pre-change form produced `error[E0308]` (`&str` into `ParamValue`, no `PartialEq<&str>`) and `error[E0599]` (no `.contains` on `ParamValue`). The `From<usize>`-missing trait failure the report cites for Step 1 is the same mechanism at the `.with(usize)` bound.
2. **Step-7 sweep (report §Fire-verification):** reproduced. `grep -rn "rejectWith\|params:" e2e/*.ts | grep -E "index|rules"` returns four lines, all **Diagnostic** fabrications in `editor-markers.spec.ts` (identifiable by `severity`/`config_path`/`rendered`; matched on the `rules` substring in `config_path`), whose wire stays `Record<string,string>` by design. The only IpcError `rejectWith` in the specs is `rejectWith("mkvmerge-not-found")` (no params). No stringly-numeric IpcError fabrication exists to sweep. The pipe catches the planted `{ index: "3" }` shape and excludes benign lines (fire-checked).

---

## Spec compliance (brief + D61)

- **`ParamValue` enum:** verbatim to the design contract (`design :1184-1186`) — `#[derive(Debug, Clone, PartialEq, Serialize)] #[serde(untagged)] enum { Num(u64), Str(String) }`, plus `From<&str>/From<String>/From<usize>`. Field type `HashMap<String, ParamValue>`, `.with` takes `impl Into<ParamValue>`. Enum + field rustdoc updated (D61/i18n-05 cited).
- **Promotion-site set complete and exact.** Walked every `.with(` in `src-tauri/src/*.rs`. Production promotions: `error.rs:202` (`index`), `:203` (`rules`), `:207` (`index`), `run.rs:935` (`index`) — the four D61 mandates (design anchors `error.rs:169/170/174` drifted down ~33 lines by the enum insertion; `run.rs:935` exact). **No numeric-semantic param missed and none over-promoted:** `detail`, `file`, `found`, `minimum`, `path`, `property`, `run_id`, `code` all correctly stay strings — including `minimum` ("86.0", a version not a count; can't be `u64`) and `code` (the `"signal"` fallback forces string, design :1195-1196).
- **TS side:** `ipc.ts` `params: Record<string, string | number>` + doc note; the three ref pairs (`SettingsDialog.vue`, `BatchView.vue`, `EditorView.vue`) widened. All eight render sites forward the whole params object to `$t(code, params)` unchanged — verified: `FirstRun.vue:94`, `RunHistory.vue:155/:241`, `JobsView.vue:249/:255` (direct), `BatchView.vue:445`, `EditorView.vue:491`, `SettingsDialog.vue:103` (ref-fed). **Zero string operations on any params value anywhere in `src/`** (grep for `.split/.toUpperCase/.slice/…` on params: none), so the widening is transparent; vue-tsc green corroborates.
- **Diagnostic wire untouched** (`EditorView.vue:171` still routes through `diagnosticFluentParams`); core-side prose/type-free invariant intact.
- **Plural selector** on `apply-rule-index-out-of-range` only, en+de, verbatim to the brief; `$index` correctly left a bare identifier. Both locales structurally parallel (`[one]`/`*[other]`), so cross-locale parity holds even though stream G's rule 5 is (confirmed) not yet in this worktree.
- **Beyond-brief-list files:** walked the full diff — **`src-tauri/src/lib.rs` is the only one.** No other beyond-list file. (Report's Divergence 1 is accurate and exhaustive.)

## House dimension

No deviation from recorded Tier-2 conventions.
- **core-37 (prose-free core) / core-124 (error-currency split):** the change is entirely shell-local (`IpcError`, rendered from `gui-common.ftl`); core `Diagnostic` currency and its wire are untouched. The typed `IpcError.params` is exactly the shell-side liberty core-124 draws (a shell error is not a profile/plan Diagnostic). No prose enters core.
- **Pub-surface discipline:** `ParamValue` is `pub` with rustdoc on the enum and both variants, matching the crate's documented-public-items rule.
- **Cross-task null-narrowing constraint:** honored vacuously — the change widens `string` → `string | number`; it routes no `Option`-derived `T | null` field into a string-typed sink. vue-tsc green confirms no regression of the class.

---

## Adjudications

### Q1 — `lib.rs` edited beyond the brief's file list / Step-9 git add

**Verdict: correct compile-forced mechanical adaptation (the `brief-drafts-verified-against-tree` shape), NOT a scope breach owing NEEDS_CONTEXT.**

- **Unavoidable — fire-verified.** Reverting the two `lib.rs` assertions to their pre-change form fails the crate build with `error[E0308]` (`err.params["minimum"] == "86.0"`: no `PartialEq<&str>` on `ParamValue`) and `error[E0599]` (`err.params["found"].contains(...)`: no such method). The workspace cannot compile once the design-mandated `params: HashMap<String, ParamValue>` lands. The `E0599` is decisive for Q3-independence: even with the ergonomic impls, `.contains` has no `PartialEq` route, so `lib.rs` must change regardless of the Q3 decision.
- **Semantics-preserving — verified.** `minimum` and `found` are string params (`RuntimeError::TooOld{found,minimum}` are `String`); the adaptation re-types them as `ParamValue::Str(...)` / a `matches!(… Str(s) if s.contains …)` guard, preserving the exact checks. The adapted test passes green in my `cargo test` run.
- **Not a fork.** NEEDS_CONTEXT routes *decisions*; there is no decision here — one faithful adaptation exists, no outward-behavior option. Stopping to ask would be process theater. Directly analogous to the sanctioned plan-7 T12 (`style.css` mislabeled Modify) and plan-6 T5 (unenumerated derive set) cases: a mandated change ripples past the brief's file list; the implementer adapts test-only, semantics preserved, and surfaces it prominently (Divergence 1 CONCERN + added to the explicit `git add`). Correct.
- **Brief-authoring gap, not an implementer defect:** the controller's Rust-file recon (error.rs + run.rs) missed that `lib.rs` also asserts on `IpcError.params`. Recorded in HARVEST.

### Q2 — the ~dozen re-typed `error.rs` test assertions

**Verdict: each is the same logical check; none weakened, skipped, or deleted. Verified assertion-by-assertion against the diff.**

- String params re-typed to `ParamValue::Str(...)` with the identical value, no logic change: `too_old` (`found`, `minimum`), `non_zero…exit_code` (`detail`; `code`→`Str("2")` — correctly *not* promoted, the string-`code` contract), `non_zero_signal` (`code`→`Str("signal")`), `parse_failure` (`detail`), `identify_json_failure` (`detail`), `settings_errors` (`detail`), `save_errors` (`detail`), `apply_errors` (`path`, `property`), `with_overwrites` (`k`), `with_attaches` (`run_id`).
- Promoted-to-`Num` assertions reflect the *intended* new numeric behaviour, same logical check (value equality): `apply_errors` `oob.index`→`Num(7)`, `oob.rules`→`Num(1)`, `noop.index`→`Num(0)`.
- **Subtle case handled correctly:** `with_attaches_and_overwrites_params` calls `.with("index", "0").with("index", "1")` with **string literals**, so the assertion stays `ParamValue::Str("1")` — the implementer did not blindly convert every `"index"` to `Num`; it matched the actual argument type. Right call.
- `serializes_as_code_and_params` (unchanged) still passes because untagged `Str` serializes identically. The new `params_serialize_untagged_numbers_and_strings` is the brief's verbatim Step-1 test. All green in my run.

### Q3 — deliberately no `PartialEq<&str>`/`PartialEq<u64>` ergonomic impls

**Verdict: right call. Keep as-is. Not a fix now, and not a whole-branch/1.x note either — the impls should not exist.**

- The design fixes the enum's **exact contract** — the derive set is enumerated (`design :1184`) and the document states "the two-variant enum is the exact contract" (`:1234`). Adding `PartialEq<&str>`/`<u64>` would expand a `pub` type's API surface beyond what the design sanctions; inventing that surface is precisely the latitude `proc-latitude-clause-boundary` removes from the keyboard. `proc-04-spec-wins`: the enumerated shape binds.
- It would not even buy what it costs: `lib.rs`'s `found` assertion needs `matches!` regardless (a `PartialEq` impl gives no `.contains`), so the churn the impls would save is partial while the surface cost is permanent.
- If the ergonomics were ever wanted, that is a design amendment (a fresh ADR), not a task-20 change. The implementer correctly flagged it as a preference rather than acting on it.

---

## HARVEST

- **Brief-authoring gap (ledger candidate / calibration):** the brief's Files list and Step-9 `git add` were exhaustive-by-convention but factually incomplete against the tree — `lib.rs`'s `detect_mkvmerge_body_too_old_carries_found_and_minimum` asserts on `IpcError.params` and is compile-forced by the mandated field-type change. Same class as plan-7 T12 (`style.css` mislabeled Modify) and plan-6 T5 (unenumerated derive set): when a task **mandates a type/field change on a shared symbol**, the brief's file recon must grep every assertion/consumer of that symbol across the whole crate, not just the sites it means to edit. Reinforces `brief-drafts-verified-against-tree` and `proc-57-briefs-not-ground-truth` (the file list is a load-bearing premise; verify it against the tree).
- **Over-restriction watch — do-not-over-restrict calibration (correct direction):** the implementer did **not** stop the compile-forced `lib.rs` adaptation as NEEDS_CONTEXT; it proceeded and surfaced. That is the right read — a mandated change's mechanical, semantics-preserving, test-only ripple is in-scope-with-surfacing, not a routed fork. Calibration data alongside plan-7 T14/T16 for the latitude carve-out's do-not-over-restrict edge.
- **Dominant pattern (clean):** a wire-type promotion done as the honest typed enum (design's rejected-alternative was the rotting `NUMERIC_IPC_PARAMS` lockstep table); the shell-local `IpcError` has no core-37 constraint forbidding a typed wire, so the enum is the correct fix and the diff carries no per-site promotion table. No repeated rejections, no defects.
- **No ledger-promotable new convention** emerged; the change instantiates existing settled entries (core-124 shell error currency, brief-drafts-verified-against-tree, proc-latitude-clause-boundary).
