# Plan 6 execution-plan review verdict

Artifact: `docs/superpowers/plans/2026-07-16-plan-6-profile-editor.md` (1078 lines)
Ground truth: `docs/superpowers/specs/2026-07-15-plan-6-design.md` (2005 lines), the v1 spec, the Tier-2 files, the tree at `0cc15d7`.
Reviewer: independent; did not author the plan.

## VERDICT

**NEEDS FIXES** — one Critical (Task 6 hands the implementer an uncloseable semantic fork: the `StructuredEdit` -> `MatchExpr` seam the plan says is "reuse" does not exist in reusable form), plus two false parallel-stream invariants and a test command that does not run.

Coverage itself is **clean**: every design section maps to a task. The design's own enumerations (the 17-row table, the 43-row widget table, the four keyword guards) are exceptionally accurate and I verified them line by line at source. The defects are in the plan's *seams*, not its scope.

---

## Coverage table

Design section -> implementing task. Verified by walking each section's normative content.

| Design section | Lines | Task(s) | Status |
|---|---|---|---|
| **D41** core writer | 122-300 | Task 2 (writer), Task 1 (SaveError supersession), Task 4 step 6 (spec 8.2), Task 9 step 4 (`editor-save-note`), Task 11 (renders the note) | COVERED |
| **D42** editor IPC surface | 303-407 | Task 8 | COVERED |
| **D43** `apply_suggestion` | 410-495 | Task 6 (core), Task 8 (command), Task 12 (batch-view UI) | COVERED — but see F1 |
| **D44** ts-rs bindings + drift check | 498-682 | Task 5 | COVERED |
| **D45** registry / widgets / never-arm / check-i18n | 685-1099 | Task 9 (registries + catalogs + gate), Task 10 (widgets), Task 11 (view) | COVERED — but see F7, F8 |
| **D46** keyword domains | 1103-1224 | Task 3 | COVERED |
| **D47** schema as user artifact | 1227-1332 | Task 7 | COVERED |
| **D48** canonical save omits defaults | 1336-1722 | Task 4 | COVERED |
| **§2** Fluent catalogs | 1725-1768 | 42 labels + 1 note -> Task 9; `gui-common.ftl` save + apply codes -> Task 8 step 4; `gui-batch.ftl` 2 keys -> Task 12 | COVERED — but see F8 |
| **§3** Spec amendments | 1772-1813 | 8.2 -> Task 4 step 6; 8.1 -> Task 7 step 2; 8.4 -> Task 7 step 3; 4.8/4.9 "no change" -> no task needed | COVERED — but see F3 |
| **§4** mkvtoolnix parity audit (SI-3) | 1817-1850 | **No task, and none required** — the section's own conclusion is that no parity target exists and "the absence of golden-test work in this plan is a recorded conclusion rather than a gap" (`:1844-1846`) | COVERED (by design conclusion) |
| **§5** Gap table | 1854-1863 | Every row's closure column names D41/D42/D43, all of which have tasks | COVERED (transitively) |
| **§6** Out of scope | 1867-1891 | No task by definition — **except** the embedded positive requirement "spec 8.3's tooltip/inline-explanation baseline **still applies to the editor's views**" (`:1878-1880`), which Task 11 names as a binding point but no step implements and no key budget admits | **PARTIAL — see F8** |
| **§7** Triggers 1-7 | 1895-1930 | Plan's "Triggers this plan creates" restates all seven; controller-owned, not a task | COVERED |
| **§7** items 8/9 (resolved) | 1932-1946 | Marked RESOLVED at HEAD, "do not re-issue" | COVERED (no action) |
| **§7** item 10 (`gui-22` collision, OPEN) | 1947-1956 | Plan's "Open, carried into the plan close" section | COVERED |
| **§8** What the implementer must not decide | 1960-2005 | Task 1 step 4 appends the two owner rulings; all 16 existing bullets appear as per-task Binding points (traced individually) | COVERED |

**Reverse check (scope creep): none found.** Task 1 is not in the design (the owner rulings post-date it) but is authorised by them. Task 1 steps 2-3 make two uncommissioned corrections (the dangling "D47's catalog table" cross-reference; naming the two `gui-common.ftl` codes) — both are factually right, both are inside the sentence already being edited, and both follow from ruling 1. Not creep.

---

## Findings

### F1 — CRITICAL. Task 6 (`plan:611`, `plan:662`): the `StructuredEdit` -> `MatchExpr` seam does not exist, and the plan asserts it does

**What the plan says.** `plan:611`: "**Do not write a new applier.** ... the engine's own `with_rule_match` narrowing helper must be reused, not re-implemented". `plan:662` (step 3): "Implement `apply_suggestion` reusing `rule_index_of` and `with_rule_match`." Interface (`plan:604`): `apply_suggestion(profile: &Profile, config_path: &str, edit: &StructuredEdit) -> Result<Profile, ApplyError>`.

**Evidence gathered (opened the tree).**

`crates/muxsmith-core/src/planner.rs:1853`:
```rust
pub fn with_rule_match(profile: &Profile, ri: usize, delta: &MatchExpr) -> Profile {
```
It takes a **`&MatchExpr`**, not a `&StructuredEdit`. The two are bridged by one private function, `crates/muxsmith-core/src/planner.rs:1809`:
```rust
// Builds the MatchExpr delta a candidate edit represents.
fn delta_for(edit: &StructuredEdit, scalar: &Scalar) -> MatchExpr {
    let mut m = MatchExpr::default();
    match edit {
        StructuredEdit::AddExact { property, .. } => {
            let mut map = BTreeMap::new();
            map.insert(property.clone(), scalar.clone());
```
Three facts, each verified at source:

1. **`delta_for` is private** (`fn`, not `pub fn`; contrast `#[doc(hidden)] pub fn with_rule_match` at `:1853` and `#[doc(hidden)] pub fn rule_index_of` at `:2032`). The plan's file list for Task 6 does not mention a visibility change.
2. **For `AddExact`/`AddNotExact` it discards the edit's own `value`** (`{ property, .. }`) and uses the **`scalar` argument** instead.
3. **The `scalar` is not derivable from a `StructuredEdit`.** It comes from batch identification. `planner.rs:2063`:
```rust
fn prop_value_as(v: &PropValue) -> Option<(String, Scalar)> {
    match v {
        PropValue::Bool(b) => Some((b.to_string(), Scalar::Bool(*b))),
        PropValue::Int(i) => Some((i.to_string(), Scalar::Int(*i))),
        PropValue::Str(s) => Some((s.clone(), Scalar::Str(s.clone()))),
        PropValue::Float(_) => None,
    }
}
```
and `planner.rs:1738-1762` shows the engine building the pair and putting only the **display String** into the edit while handing the **typed `Scalar`** to `delta_for`:
```rust
let Some((display, scalar)) = prop_value_as(val) else { continue; };
...
StructuredEdit::AddExact { property: prop.clone(), value: display.clone() },
...
raw.push(Candidate { apply: delta_for(&edit, &scalar), ... });
```

**Why it is Critical.** `apply_suggestion` receives only the `StructuredEdit`, whose `value` is a `String`. To call `with_rule_match` it must produce a `MatchExpr`, which for the two `Exact` variants requires a `Scalar` — and `Scalar` is a four-way sum (`Bool`/`Int`/`Float`/`Str`, `match_expr.rs:19-28`). The implementer must invent String->Scalar reconstruction. The obvious wrong answer (`Scalar::Str(value)`) compiles, passes any test that only uses string-valued properties, and silently produces `exact: { default_track: "true" }` where the engine simulated `exact: { default_track: true }` — an applied edit that is **not** the edit that was simulated. That voids `core-03-suggestion-verified-edit` ("an applied suggestion survives the next dry run", `conventions.yaml:21`), which is the exact invariant `plan:611` invokes to justify the reuse mandate.

D43 forecloses the only escape the tree offers: re-planning inside apply is rejected (`design:486-488`), so apply cannot re-identify the batch to recover the `Scalar`.

This is `proc-latitude-clause-boundary`'s omission form, verbatim: "must the implementer invent something it is not allowed to invent?" Yes. And because `plan:611`/`:662` assert the two named reuse targets are sufficient, the implementer has no signal that a fork exists — they will not return NEEDS_CONTEXT, they will just write the conversion.

**Concrete fix.** Close the seam in the plan. The tree supplies the missing input: `capability::matchable_type(name) -> Option<PropType>` (`capability/mod.rs:40`) recovers the declared type from the property name, so a typed parse is constructible. Either:
- (a) mandate `delta_for` be made `#[doc(hidden)] pub` on the house pattern its two siblings already use, add a `StructuredEdit -> Scalar` step specified against `matchable_type` + `PropType`, and enumerate the per-`PropType` parse; **and** add a test asserting the applied delta equals the engine's simulated delta for a `Bool`- and an `Int`-valued property; or
- (b) route it as a design fork (NEEDS_CONTEXT with a decision memo) before dispatching Task 6.

Do not dispatch Task 6 as written.

### F2 — IMPORTANT. Task 8 steps 2 and 5 (`plan:798`, `plan:813`): `cargo test -p muxsmith` names no package

**Evidence.** Ran it:
```
$ cargo test -p muxsmith --lib error --no-run
error: package ID specification `muxsmith` did not match any packages
```
Workspace members (`cargo metadata --no-deps`): `['muxsmith-core', 'muxsmith-cli', 'xtask', 'muxsmith-gui']`. `src-tauri/Cargo.toml:2` is `name = "muxsmith-gui"`.

The likely source of the error is a bin-vs-package conflation: `crates/muxsmith-cli/Cargo.toml:9` declares `[[bin]] name = "muxsmith"`, which is why `cli_schema.rs` legitimately calls `Command::cargo_bin("muxsmith")`.

Step 2 hedges ("Confirm the crate name from `src-tauri/Cargo.toml` first"), which partly rescues it. **Step 5 does not**: `cargo test -p muxsmith` with "Expected: PASS" is an expectation that cannot be met.

Note the plan gets the *other* package right: `cargo test -p muxsmith-cli --test cli_schema` (Task 3) is correct.

**Fix.** `cargo test -p muxsmith-gui --lib error` (step 2) and `cargo test -p muxsmith-gui` (step 5). Drop the hedge; the name is now verified.

### F3 — IMPORTANT. Wave 1 streams A and C both modify the v1 spec, contradicting the plan's own stream invariant

**Evidence.** `plan:34`: "Three streams, parallel worktrees, **no shared files between them**."

- Task 4 Files (`plan:389`, stream A): `Modify: docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (spec 8.2 amendment). Staged at `plan:484`.
- Task 7 Files (`plan:695`, stream C): `Modify: docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (spec 8.1 + spec 8.4 exception). Staged at `plan:734`.

Opened the spec: `## 8. Surfaces` at `:353`, `### 8.1 CLI` at `:355`, `### 8.2 GUI` at `:369`, `### 8.4 Internationalization architecture` at `:394`. All three amendment sites sit inside one ~40-line section of one file, edited concurrently in two worktrees.

Second-order: both tasks independently run the `proc-04-spec-wins` self-contradiction sweep on that file (Task 4 step 6, Task 7 step 4), each against a tree carrying only its own amendment. The sweep is a whole-document property; neither run observes the merged result.

**Fix.** Give the v1 spec a single owner. Move the spec-8.2 amendment out of Task 4 into Task 7 (stream C is already the docs stream and already amends this file), and have Task 7 run the sweep once against all three amendments. Task 4 then touches only core + its fixture, restoring the stream invariant.

### F4 — IMPORTANT. Wave 3: Task 12 collides with Tasks 10 and 11 on `e2e/smoke.spec.ts`

**Evidence.** `plan:829`: "Task 9 -> Task 10 -> Task 11. **Task 12 needs only Task 8 and may run as its own stream in parallel with 9-11**."

- Task 10 Files (`plan:924`): `Test: e2e/smoke.spec.ts (extend)`; staged `plan:961`.
- Task 11 Files (`plan:973`): `Test: e2e/smoke.spec.ts`; staged `plan:1010`.
- Task 12 Files (`plan:1024`): `Test: e2e/smoke.spec.ts`; staged `plan:1057`.

`wc -l e2e/smoke.spec.ts` -> 574. Task 12 runs in a separate stream, concurrently with 10 and 11, all three appending to the same 574-line file.

**Fix.** Either give Task 12 its own spec file (`e2e/apply-suggestion.spec.ts` — the tree already precedents a per-concern spec: `e2e/catalogs.spec.ts` exists precisely because a concern was split out of a shared module), or sequence Task 12 after Task 11 and drop the parallelism claim.

### F5 — IMPORTANT. Task 8 (`plan:768`): `ProfileDocument`'s two halves are incompatible; no step defines it and no test pins it

**What the plan says.** "`ProfileDocument` is `{ profile: Option<Profile>, diagnostics: Vec<Diagnostic> }`, serialized through the existing `report::json` document machinery so its `diagnostics` array is **byte-identical in shape** to what `validate_profile` already returns (`core-85-report-json-dry`)."

**Evidence.** `validate_profile` -> `validate_profile_body` (`src-tauri/src/lib.rs`):
```rust
fn validate_profile_body(path: &Path) -> serde_json::Value {
    let diags = validate::config_diagnostics_from_file(path);
    report::json::config_only_document(&diags, None, &ShellRenderer)
}
```
`crates/muxsmith-core/src/report/json.rs:78`:
```rust
pub fn config_only_document(
    config_diags: &[Diagnostic],
    mkvmerge_found: Option<bool>,
    renderer: &dyn DiagnosticRenderer,
) -> serde_json::Value {
    let mut doc = serde_json::json!({
        "config_diagnostics": rendered_diags(config_diags, renderer),
        "files": [],
        "batch_diagnostics": [],
        "suggestions": [],
    });
```
and `json.rs:176`:
```rust
pub fn rendered_diags(diags: &[Diagnostic], renderer: &dyn DiagnosticRenderer) -> Vec<serde_json::Value> {
    diags.iter().map(|d| {
        let mut v = serde_json::to_value(d).unwrap();
        v["rendered"] = serde_json::Value::String(renderer.diagnostic(d));
        v
    }).collect()
}
```
So what `validate_profile` returns is: an **untyped `serde_json::Value`**, keyed **`config_diagnostics`** (not `diagnostics`), whose array elements are the `Diagnostic` serde value **plus an injected `"rendered"` string**, alongside `files`/`batch_diagnostics`/`suggestions`. (Confirmed by the shell's own test, which asserts `doc["config_diagnostics"]` and `doc["files"]`.)

A Rust struct field `diagnostics: Vec<Diagnostic>` serialized by derive produces **none** of that: no `rendered`, different key, no envelope. The two descriptions in `plan:768` cannot both hold. Task 8 step 3 says only "then `ProfileDocument` and the four commands to `lib.rs`" — no definition, no shape test. The plan's own testability rule is unmet, and `core-85-report-json-dry` is explicitly invoked, so an implementer guess is a house violation either way.

Compounding: the sibling command `validate_profile_model` is typed `Result<serde_json::Value, IpcError>` (untyped) while `load_profile` is `Result<ProfileDocument, IpcError>` (typed). Both carry diagnostics; the plan does not say whether the editor receives them in one shape or two.

**Fix.** Pin, in Task 8: the exact `ProfileDocument` definition; whether its diagnostics array carries `rendered` (i.e. whether it is built by `rendered_diags`/`config_only_document` or by derive); the field name; and add a step-1 test asserting the shape against `validate_profile`'s output. If the answer is "reuse `config_only_document` and add a `profile` key", say that.

### F6 — IMPORTANT. Task 9: `e2e/catalogs.spec.ts` is a deliverable with no work item — and needs no work

**Evidence.** Task 9 Files (`plan:838`): `Test: e2e/catalogs.spec.ts (extend)`. It is staged at `plan:914`. **No step in Task 9 mentions it.** Steps 1-7 cover the registry proof, the types, the option arrays, the catalogs, the check-i18n scan, the scan proof, and the gate.

Opened the file (12 lines) — it is one test delegating to `assertAllCatalogsParseCleanly`. Opened that (`e2e/i18n-en.ts`):
```js
export function assertAllCatalogsParseCleanly(): void {
  const locales = readdirSync(LOCALES_ROOT, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())...
    const allFiles = readdirSync(dir).filter((f) => f.endsWith(".ftl")).sort();
    const guiAndDiagnostics = allFiles
      .filter((f) => f.startsWith("gui-") || f === "diagnostics.ftl")
```
It **globs**. `gui-editor.ftl` is picked up automatically with zero change to either file. The design says exactly this at `:1728-1731` ("`scripts/check-i18n.mjs` picks up any `locales/<tag>/` directory automatically").

So the plan lists a file to extend, stages it in the commit, gives no instruction — and the correct action is "nothing". An implementer either invents a test (latitude in a normative position) or commits a puzzled no-op.

**Fix.** Delete `e2e/catalogs.spec.ts` from Task 9's Files and from its `git add` line. If the intent was to record that the guard covers the new catalog for free, state that as a no-work note, not a deliverable.

### F7 — IMPORTANT. Task 11 is too large to review as one unit and is specified in one line

**Evidence.** Task 11 creates `src/views/EditorView.vue` carrying, per `plan:970`, "rule grid, drag-reorder, section composition, open/save, the save-surface note", plus (binding points `plan:982`) validation through `validate_profile_model` on every edit and Save disabled while errors exist — and modifies `src/App.vue` (the `View` union, the nav, the mount block) and `src/ipc.ts` (four command signatures).

Its entire implementation step (`plan:998`) is:
> **Step 3: Implement the view, the nav entry, and the ipc signatures**

Sizes measured (`wc -l`): `src/views/BatchView.vue` 449, `src/views/JobsView.vue` 332, `src/App.vue` 119, `src/ipc.ts` 296. EditorView carries more than BatchView does (BatchView has no drag-reorder and no 13-registry section composition), so this task's single step produces the largest view in the repo plus two cross-cutting edits.

House precedent points the other way: `locales/en/gui-batch.ftl:1` reads `## T10: BatchView.vue and its components` — Plan 5 gave one view a whole task, and this plan gives Task 10 the ten widgets alone.

Contrast the plan's own calibration elsewhere: Task 2 has seven detailed steps for a ~60-line module with its test body written out verbatim.

**Fix.** Split Task 11 into three: (a) the rule grid + drag-reorder; (b) section composition + widget dispatch over the 13 registries; (c) open/save + the save-surface note + the nav entry + the `ipc.ts` signatures. Each ends in its own e2e assertion.

### F8 — IMPORTANT. Spec 8.3's tooltip baseline applies to the editor, has no task step, and the pinned 43-key count structurally excludes it

**Evidence — three plan/design statements that cannot all hold.**

1. Design `:1878-1880` (in §6, out-of-scope): "**The spec 8.3 tooltip/inline-explanation baseline still applies to the editor's views** (D22's 'NOT deferred' clause); only the sidebar machinery waits." Task 11 restates it as a binding point at `plan:984`.
2. The v1 spec, `### 8.3 Self-explanation and help mode` (`:381`): "Baseline discoverability: **every non-obvious control carries a tooltip**; views carry small inline explanations where a first-time user would otherwise guess."
3. Plan Global Constraints ruling 2 (`plan:17`): "`gui-editor.ftl` carries **43** keys: 42 registry labels + 1 note". Task 9 binding point (`plan:851`): "`gui-editor.ftl` carries exactly **43** keys". Task 9 (`plan:852`): "Widget facets add **no** keys".

Every tooltip and inline explanation is user-facing prose, so under spec 8.4 each needs a Fluent key. The editor's surface has 42 controls. 43 = 42 labels + 1 note leaves **zero** keys for tooltips. No task step implements 8.3 for the editor.

The implementer of Task 11 hits this at the keyboard: adding one tooltip makes the catalog 44 keys and violates an owner ruling. Per the plan's own Global Constraints (`plan:14`) that is a NEEDS_CONTEXT — but a fork the plan could have closed should not reach the keyboard.

I am **not** re-litigating the 43 ruling; the ruling is about the *save-note* count (one message, not two). What is unclosed is whether the editor's 8.3 baseline is in Plan 6 at all, and if so where its keys live.

**Fix.** Route to the owner as a scope question before dispatch: does Plan 6's editor ship spec 8.3's tooltip baseline (then `gui-editor.ftl` needs a stated tooltip budget on top of 43, and Task 11 needs a step), or does the editor's 8.3 baseline defer to Plan 7 with the sidebar (then design `:1878-1880`'s "still applies" needs amending)?

### F9 — MINOR. Task 6 step 5 (`plan:676`): the "prove the reuse is real, not asserted" command cannot ever fire

**Evidence.** The plan gives:
```bash
grep -n "with_rule_match\|extend(" crates/muxsmith-core/src/planner.rs | sed -n '/apply_suggestion/,$p'
```
Ran it against the tree:
```
$ grep -n "with_rule_match\|extend(" crates/muxsmith-core/src/planner.rs | sed -n '/apply_suggestion/,$p' | wc -l
0
```
Structurally: `grep` emits only lines containing `with_rule_match` or `extend(`. `sed -n '/apply_suggestion/,$p'` needs a line containing `apply_suggestion` to **open** its range. No line can contain both a call to `with_rule_match` and the string `apply_suggestion` (the function's own signature line contains neither pattern). The range never opens; the step prints nothing and "passes" for every possible implementation, including a hand-rolled clobbering applier.

This is the step guarding `core-44` — the Bug C regression the plan calls "the one thing here that a plausible-looking reimplementation gets wrong silently" (`plan:673`).

Second defect in the same step: "A `BTreeMap::extend` anywhere in the applier is the Bug C shape and fails review" is over-broad against the house's own recorded exemption. `planner.rs:1868`, inside `with_rule_match` itself:
```rust
if let Some(add) = &delta.not {
    expr.not.get_or_insert_with(Vec::new).extend(add.clone());
}
```
with the rationale at `planner.rs:1844-1845`: "`not` entries are always additive (appending a not-clause always narrows, never relaxes), so plain `extend` stays correct there."

**Fix.** Scope the grep to the function body, e.g.
```bash
awk '/fn apply_suggestion/,/^}/' crates/muxsmith-core/src/planner.rs | grep -n "with_rule_match\|extend("
```
with the expectation stated as "at least one `with_rule_match` hit; no `extend(` on an `exact`/`substring` map". Then verify the *observed* output rather than the absence of output.

### F10 — MINOR. Task 3 step 1 (`plan:313`): `schema_json()` does not exist, and the fallback instruction contradicts itself

**Evidence.** The plan's test body:
```rust
let schema = schema_json(); // the file's existing helper that runs `muxsmith schema`
```
Opened `crates/muxsmith-cli/tests/cli_schema.rs` in full (21 lines). It contains two tests and **no helper**; it inlines the invocation:
```rust
let out = Command::cargo_bin("muxsmith").unwrap().arg("schema").assert().success().get_output().stdout.clone();
let schema: serde_json::Value = serde_json::from_slice(&out).unwrap();
```
"the file's existing helper that runs `muxsmith schema`" is false.

The plan's fallback (`plan:339`) — "Adapt the helper call to whatever `cli_schema.rs` already uses to obtain the schema JSON - read the file first; do not add a second way to run the command" — then has no resolution: there is nothing to adapt to; duplicating the block is forbidden by the same sentence; extracting a helper is the only legal move, is unbudgeted, and modifies an existing passing test the Files list does not flag as a refactor.

**Fix.** State it: "extract a `schema_json()` helper from `schema_prints_json_schema_and_exits_zero` and re-point that test at it; both tests then share the one invocation."

### F11 — MINOR. Task 6 step 1 (`plan:620-649`): three of four test bodies are empty

**Evidence.** As written:
```rust
#[test]
fn apply_never_clobbers_an_existing_match_key() {
}

#[test]
fn an_unparseable_or_out_of_range_config_path_is_an_error() {
}

#[test]
fn an_unknown_structured_edit_kind_fails_to_deserialize() {
}
```
`apply_narrows_the_named_rule_only` carries comments only. `apply_never_clobbers_an_existing_match_key` is the `core-44` Bug C guard — by the plan's own account the most important test in the task.

Contrast Task 2 step 1 and Task 4 step 1, whose bodies are written out verbatim and are directly runnable. The inconsistency lands on the one task that also carries F1: with empty bodies and no named fixture, nothing forces the implementer to confront the `Scalar` reconstruction.

**Fix.** Write the four bodies against named fixtures in `tests/suggestions.rs`, as Tasks 2 and 4 do. At minimum the no-clobber body, and (per F1) one asserting a `Bool`- or `Int`-valued property round-trips to the engine's own delta.

### F12 — MINOR. Task 4 step 2 (`plan:441-446`) is not a TDD red step and cannot be

**Evidence.** Title: "**Step 2: Run guard 1 to confirm it fails**". Expectation: "FAIL - the fixture does not exist yet, or (once it does) **it passes trivially** because nothing is skipped yet. **Both are expected at this point**".

Before D48's `skip_serializing_if` attributes exist, nothing is omitted, so a save/load round-trip **must** pass. The design records this as already true: `:90-92`, "Canonical round-trip is exact. Serializing the parsed `reference.yaml` and re-parsing yields an equal model (`p == p2` -> `true`). This is D41's correctness floor and **it holds today**." Step 1 creates the fixture, so the "fixture does not exist" arm is closed by the preceding step. Step 2 therefore always passes.

An "Expected" that admits both outcomes cannot be violated and verifies nothing.

**In fairness**, the plan compensates well: step 4 ("Prove guard 1 catches the naive predicate", `plan:454-462`) is the real red proof, it is correctly constructed, and it ends "If it does **not** go red, guard 1 is not testing what D48 says it tests - stop and return NEEDS_CONTEXT". The defect is step 2's label and expectation only, not a missing guard.

**Fix.** Relabel step 2 "Baseline: observe guard 1 green on the unmodified serializer" with the single expectation PASS, and state that step 4 is the red proof.

### F13 — MINOR. Task 9 step 1 (`plan:855-865`) depends on step 2

**Evidence.** "**Step 1: Write the failing registry-completeness proof**", whose body begins:
```bash
# after step 2 exists, delete one entry from outputFields and run:
pnpm build
```
Step 1 cannot be executed first; it requires `src/editor/registries.ts`, which step 2 creates. The step is honest about it, which makes the ordering the defect rather than a hidden trap.

**Fix.** Swap: write the types and registries, then run the deliberate-break proof.

### F14 — MINOR. Global Constraints (`plan:24`): "`#![deny(missing_docs)]` is on" is false for the crate Task 8 works in

**Evidence.** True for core: `crates/muxsmith-core/src/lib.rs:1` is `#![deny(missing_docs)]`. False for the shell — `grep -rn "deny(missing_docs)" src-tauri/src/` returns one hit, and it is a negation. `src-tauri/src/lib.rs:14-17`:
```
//! Not `#![deny(missing_docs)]`: `src-tauri` is a bin-shaped crate (the
//! `[lib]` target exists only so Tauri's mobile entry point can call into
//! it), unlike `muxsmith-core`/`muxsmith-cli`. Public items are still
//! documented.
```
`grep -n "^#!\[" src-tauri/src/lib.rs` -> no crate-level attributes at all.

The constraint is stated globally and binds every task; Task 8 (the shell task) is the one it is wrong about. Effect is benign (writing rustdoc that is not enforced costs nothing, and the tree asks for it anyway), but it is a false tree premise in a normative position, and the "and `cargo doc` gates that the intra-doc links resolve" half will not behave as stated there.

**Fix.** "`#![deny(missing_docs)]` is on in `muxsmith-core` and `muxsmith-cli`; `src-tauri` deliberately opts out (`src-tauri/src/lib.rs:14-17`) but still documents public items."

### F15 — MINOR. Task 1 step 5 (`plan:136-137`): the second grep verifies nothing, before or after the fix

**Evidence.** The plan gives:
```bash
grep -n "Two new Fluent keys\|D47's catalog table" docs/superpowers/specs/2026-07-15-plan-6-design.md
# Expected: no output. Both defects corrected in step 2.
```
Ran it on the current document: **only `:278` matches**, on the first alternative. Then:
```
$ grep -c "D47's catalog table" docs/superpowers/specs/2026-07-15-plan-6-design.md
0
```
The phrase never matches because the design wraps it across a line break — `:278` ends "`...en+de (D47's`" and `:279` is "`catalog table).`" — and `grep` is line-based.

So the grep asserts it has checked two defects while checking one. If step 2 removed "Two new Fluent keys" but left the dangling `(D47's catalog table)` cross-reference that step 2 itself identifies as the second defect, this check would report "no output" and pass.

**Fix.** `grep -n "Two new Fluent keys" ...` and, separately, `grep -n "D47's" ...` (expect: no output).

### F16 — MINOR. The stream-A serialization rationale is false for Task 2; Tasks 2 and 3 are needlessly chained

**Evidence.** `plan:7` and `plan:36`: "stream A is a four-task serial chain because D41/D46/D48/D44 all converge on `crates/muxsmith-core/src/profile/model.rs`" / "**All four converge on** `crates/muxsmith-core/src/profile/model.rs`, so they are a chain, not a fan-out."

Task 2's Files (`plan:157-160`):
- Create: `crates/muxsmith-core/src/profile/save.rs`
- Modify: `crates/muxsmith-core/src/profile/mod.rs`
- Test: `crates/muxsmith-core/tests/profile_save.rs`

**No `model.rs`.** Tasks 3, 4, 5 do touch it; Task 2 does not. Task 2 and Task 3 have fully disjoint file sets (Task 3: `model.rs`, `validate.rs`, `cli_schema.rs`, `validate_semantics.rs`) and could run in parallel.

The real graph is 2 -> 4 (Task 4 extends `tests/profile_save.rs` and consumes `save::to_string`) and 3 -> 5 (Task 5 consumes the `KEYWORDS` constants) — both of which the plan states correctly one sentence later at `plan:36`.

The resulting order is **safe** (over-serialization costs wall-clock, not correctness), so this is Minor. But the stated reason is wrong, and a later re-cut reasoning from it would mis-cut.

**Fix.** "Tasks 3, 4 and 5 converge on `model.rs`; Task 2 is independent of it but precedes Task 4, which consumes its writer. 2 and 3 may run in parallel."

### F17 — MINOR. Task 5 step 8 (`plan:573`): the drift check's hole is *not* closed elsewhere during waves 1-2

**Evidence.** The plan (following design `:624-628`): "That hole is closed elsewhere - a missing `keywords.ts` fails the TypeScript build on the registry's import of it, which `pnpm build` runs on every leg."

The registry is `src/editor/registries.ts`, created by **Task 9, wave 3**. Task 5 merges in **wave 1**. `package.json` -> `"build": "vue-tsc --noEmit && vite build"`; between the two merges nothing imports `keywords.ts`, so no gate closes the hole for two waves.

Impact is low: Task 5 step 6 (`plan:548-552`) does `ls -1 src/bindings/` with "Expected: `keywords.ts` and `profile.ts`" and inspects the output, which covers the first-generation case in-task. But the claim as written is a `proc-no-work-needed-check` shape whose premise does not hold at the point it is made.

**Fix.** Note the wave gap: "the compensating import lands in Task 9 (wave 3); until then step 6's `ls` is the check."

### F18 — MINOR. Task 12 leaves a stale comment asserting the opposite of the new behaviour

**Evidence.** `src/components/SuggestionCard.vue:6-9`, which the plan cites at `plan:1021` as the props evidence:
```
// D22: Plan 5 ships suggestions as show-and-copy only, never applied --
// the profile is never mutated from here. `edit` (the structured,
// machine-applicable form) is deliberately never read; only
// `config_path` and `yaml_fragment` -- the exact text the CLI itself
// prints for `dry-run-suggestion` -- are shown/copied.
```
Task 12 makes the frontend forward `edit` (`plan:1031`: "The frontend **forwards two opaque fields it never interprets** - `config_path` and `edit`"), falsifying "`edit` ... is deliberately never read". Task 12's Files includes `SuggestionCard.vue` but no step mentions updating the comment.

Note the citation itself: `:6-9` is the *comment* documenting the props, not the props declaration. The design makes the same citation and it is approved, so this is not a citation defect — but it means the plan's own evidence line points at the text that Task 12 invalidates.

**Fix.** Add to Task 12 step 3: update the D22 comment to record that apply now reads `edit`, and why (D41 supersedes D22's stated reason).

---

## Verified-clean

Everything below I opened or ran and confirmed. Listed so the controller knows the coverage behind the finding count.

**The no-work-needed check the brief flagged — VERIFIED TRUE.** `plan:898`: "check 2 already counts a key as used when it appears anywhere in `src/` as a quoted literal, single- or double-quoted (`:191-198`, the test at `:193`)". Opened `scripts/check-i18n.mjs`. Lines 191-198 are exactly the loop; line 193 is exactly:
```js
    if (text.includes(`"${id}"`) || text.includes(`'${id}'`)) {
```
Both quote forms counted. `src/jobRowState.ts:44-55` is exactly `jobStateKey`, returning double-quoted `"jobs-state-*"` literals from a switch — the identical shape, and it does pass today. The plan's conclusion that check 2 needs no change is **correct**, as is its claim that only check 1 misses the registry (`CALL_RE` at `:168` matches only `$t(`/`t(` call sites). `LABEL_KEY_RE`'s placement alongside `CALL_RE`, feeding `literalCallIds` and `missing`, is consistent with the script's structure.

**The nine-part gate — exact.** `BUILDING.md:70-95` lists the five-part Rust gate (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, `cargo deny check`) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`, and says "nine parts total". The plan's Global Constraints list matches item for item and in order.

**Design citations spot-checked — all correct** (opened each range):

| Plan cites | Contains what the plan says |
|---|---|
| `:132-133` (superseded signatures) | Yes — both `to_string`/`to_file` `Result<_, Diagnostic>` lines. Matches `process-conventions.yaml:334`'s own citation of the same defect. |
| `:278-279` ("Two new Fluent keys") | Yes, verbatim |
| `:1736` / `:1749` (the 43) | `:1736` = the note row (count 1); `:1749` = "why `gui-editor.ftl` carries **43**". Together they carry the claim. |
| `:1737` (the `gui-common.ftl` row) | Yes, verbatim: `\| D41 \| save-failure \`IpcError\` codes \| \`gui-common.ftl\` \| codes \|` |
| `:1517-1535` (the 17-row table) | Yes — header at `:1517`, **exactly 17 rows** at `:1519-1535` |
| `:1447-1457` (the predicates) | Yes — the four non-generic predicates at `:1453-1456` |
| `:1557-1637`, `:1612-1637` (schemars interaction; the three `{}` fields) | Yes |
| `:1657-1701` (the guard-2 analysis) | Yes |
| `:768-782` (13 structs) | Yes — 13 rows at `:770-782` |
| `:848-894` (43-field widget table) | Yes — **exactly 43 rows** at `:852-894` |
| `:806-819` (`TextSyntax` + `FieldWidget`) | Yes — **exactly 10 variants** at `:810-819` |
| `:694-706`, `:905-909` | Yes |
| `:1076-1091`, `:1082-1091` (check-i18n fix) | Yes |
| `:1156-1161` (the four `KEYWORDS`) | Yes |
| `:70-77` (bare-string measurement), `:1112-1116`, `:1118-1120` (target shape, probe) | Yes |
| `:556-570` (the emitter), `:614-631` (the untracked-file hole) | Yes |
| `:1797-1813` (self-contradiction sweep), `:1895-1930` (**exactly** triggers 1-7; 8/9/10 begin at `:1932`), `:1947-1956` (`gui-22` item 10) | Yes |

**Tree citations spot-checked — all correct:**
- **The D48 17-row table against `model.rs`**: every one of `:30 :37 :40 :43 :47 :82 :100 :105 :201 :211 :256 :273 :285 :291 :306 :361 :364` is a `#[serde(default...)]` line, and the four divergent ones are exactly `:82 default_true`, `:100 FilenameCfg::keep`, `:201 SourceCfg::primary`, `:306 drop_policy`. `model.rs` carries 27 `serde(default` lines total: these 17 plus 10 `skip_serializing_if = "Option::is_none"` — so the design's 17 is precisely the un-skipped set. Nothing missing, nothing spurious.
- `validate.rs` guards `:105` (`SourceCfg::Keyword(k) if k == "primary"`), `:129` (`FilenameCfg ... "keep"`), `:149` (`ChaptersCfg ... "keep" || "drop"`), `:166` (`TitleCfg ... "keep" || "clear"`) — all four exact, and the four hand-typed `allowed` strings the plan names (`"primary"`, `"keep"`, `"keep, drop"`, `"keep, clear"`) follow from them.
- `validate.rs:430-437` = `fn domain_hint` (MAX 8, join `", "`) — so the plan's "our domains are 1 to 2 values, so it is a plain join" and the byte-identical/snapshots-must-not-move claim hold.
- `validate.rs:193` = `pub fn config_diagnostics`; `:20-21` = "touches no filesystem beyond the profile itself"; `:466-472` = the `Some(false)` -> `InvalidKeyword`/`allowed: "true"` rejection; `:460-465` = `LocatorConflict`.
- `planner.rs:201` / `:231` = the derive lines of `StructuredEdit` / `Suggestion` (correct place to add `Deserialize`); `:2032` = `pub fn rule_index_of`; `:1437` and `:1516` = `config_path: format!("tracks[{ri}].match")`.
- `src-tauri/src/lib.rs:440-452` = **exactly** the `invoke_handler` block (`.invoke_handler(` at 440, `])` at 452); `:73-79` = `on_blocking`'s body; `src-tauri/src/error.rs:8-15` carries the quoted IpcError boundary verbatim.
- `src/App.vue:10` = `type View = "batch" | "jobs";`; `:98-104` = **exactly** the v-show-not-v-if rationale comment; `:71-96` = the nav (ends `</nav>` at 96).
- `src/views/BatchView.vue:118` = `report.value = await validateProfile(selectedProfile.value);` — the live consumer is real.
- `capability/mod.rs:55` = `pub static TYPE_VALUES: &[&str]`; `:125-129` = `CODEC_KIND_NAMES`, with "so the two can never drift" verbatim at `:127`.
- Task 1's quoted catalog line is exact: `locales/en/diagnostics.ftl:6` = `parse-error = The profile could not be parsed: { $detail }`.
- Task 8 step 4's claim that every `IpcError` code lives in `gui-common.ftl`: `mkvmerge-spawn-failed` (`:33`), `settings-io-failed` (`:45`), `internal-task-failed` (`:47`). Correct.
- Task 12's "beside `SuggestionCard.vue`'s current copy-button keys": `gui-batch.ftl:54-55` = `batch-suggestion-copy` / `batch-suggestion-copy-tooltip`. Correct.
- Task 1's "`catalog_completeness.rs` matches `DiagCode` exhaustively": `crates/muxsmith-cli/tests/catalog_completeness.rs:35` — "Exhaustive match: a new `DiagCode`...". Correct.
- Tier-2 ids all resolve: `core-124-error-currency-split` (`conventions.yaml:422` — and its statement independently confirms the plan's ruling-1 restatement, including the `ParseError` corollary and the `catalog_completeness.rs` argument), `core-03` (`:21`), `core-44` (`:194`), `core-33` (`:597`), `core-109` (`:741`), `core-83-zero-rule-keep-passthrough` (`product-boundaries.yaml:388` — statement confirms "a zero-rule profile with tracks.unmatched=keep is a LEGAL pure-passthrough remux", exactly as Task 4 characterises it).
- Task 4's core-83 test is sound: `KeepDrop` is `#[serde(rename_all = "lowercase")]` with `#[default] Keep` (`model.rs:181-187`), so it emits `keep`/`drop` and `text.contains("unmatched: keep")` is a valid assertion; `Profile` requires only `profile_version`/`input`/`tracks`, so the inline fixture parses.
- Task 2's test is sound: `Profile` derives `PartialEq` (`model.rs:17`), and `Format`, `from_str`, `from_file` are all `pub` (`load.rs:17`, `:37`, `:56`). `load::from_file:57-62` is exactly the `Some("json") => Format::Json, _ => Format::Yaml` match the plan says to mirror.
- Task 2 step 5's conditional resolves correctly to a no-op: `tempfile = "3.27.0"` is already in `crates/muxsmith-core/Cargo.toml` `[dev-dependencies]`. The plan's "check first, and only add it if it is missing" is the right shape.
- `.gitattributes` = `* text=auto eol=lf` (design's LF claim holds).
- `.cargo/` does not exist — Task 5's "Create `.cargo/config.toml`" is correct.
- `proc-proposed-safeguard-stays` conformance: Task 4's guard-2 binding point (`plan:404`) applies it correctly and cites trigger 2 as the removal path, exactly as the convention prescribes.

**Could not verify:** `ts-rs` `max_stable_version = 12.0.1`. Network is unavailable in this environment; `curl https://crates.io/api/v1/crates/ts-rs` returned nothing. The plan already mandates re-verification in Task 5 step 1 with a NEEDS_CONTEXT on divergence, which is the correct handling under `ci-10-pin-everything` / `proc-07-verify-against-source`. Not a finding.

**Also clean:** no explicit latitude clause anywhere in the plan (swept for "may choose", "either approach", "if a simpler", "at your discretion" — none). Version-pin discipline conforms (`ts-rs = { version = "12.0.1", optional = true }`, caret, matching `schemars = "1.2.1"`; `=`-pins reserved for dev-deps, consistent with `proptest = "=1.11.0"` / `insta = "=1.48.0"`). Commit discipline conforms (unsigned, explicit staging, no `git add -A`). Typography conforms (ASCII hyphens, straight quotes throughout).

---

## HARVEST

Observed dominant patterns and repeated rejections. Reported only — I did not write to the ledger or the convention files.

1. **"Derive the second copy from the first" is now the plan's most-invoked rule, and it has reached count 4+.** It appears as `capability::CODEC_KIND_NAMES` from `CODEC_KINDS` ("so the two can never drift", `capability/mod.rs:127`), D46's four `KEYWORDS` feeding guard + `allowed` + schema, D48's "one function, three mentions, no copies" across `default`/`skip_serializing_if`/`schemars(extend)`, and D44/D45's generated `keywords.ts` chosen **over** a guard test with the explicit reasoning "a guard detects drift; derivation makes it unrepresentable" (`design:950`). The plan's own citation policy (`plan:28`) is the same rule applied to prose: "a second copy of a normative table is a drift surface, and refusing to create one is this design's own most-repeated argument". This looks like a Tier-2 candidate in its own right — something like *derive-over-guard: where a second copy is unavoidable, generate it from the first; a guard is what you build when derivation is unavailable* — with the corollary the design already states, that `schemars(extend)` taking arbitrary expressions removed the last excuse for a literal.

2. **The house has a settled shape for "share an engine internal with a test/consumer": `#[doc(hidden)] pub` + a comment saying why.** Two instances in `planner.rs` alone, both with near-identical rationale prose: `with_rule_match` at `:1849-1852` ("shared verbatim with `prop_planner.rs`'s suggestion-application property so the test exercises the real splice, not a duplicate copy that could silently drift from it") and `rule_index_of` at `:2030` ("the engine itself uses, not a duplicate copy that could silently drift from it"). This is the mechanism F1's fix needs for `delta_for`, and it is already the local pattern — worth noting as an emerging convention since it now has 2 occurrences with matching justifications.

3. **Enumerate-don't-describe is being applied recursively, and it is working.** D45 `:722` records its own first draft failing it ("The first draft of this ADR left these as `...`. That is a latitude clause no keyword scan catches") and cites D48's standard back at itself. The measured payoff is real: I verified the 17-row table, the 43-row widget table and the 13-struct table row by row against the tree and found **zero** errors, which is not the usual outcome. The counter-observation for the ledger: the plan's remaining latitude (F1, F6, F8) is all in **seams between** enumerated sets — the `StructuredEdit`->`MatchExpr` bridge, a file listed with no work, a requirement with no key budget. The enumeration discipline is mature; the interface-between-enumerations discipline is not. That gap looks like the next thing worth a rule.

4. **`proc-no-work-needed-check` earned its keep in this review, in both directions.** Of the four no-work-needed shapes I ran: check 2 (`plan:898`) verified **true** with an exact line match; the `tempfile` conditional verified **true** (already present); Task 5's "closed elsewhere" verified **false for waves 1-2** (F17); and Task 1 step 5's grep verified **vacuous on one of its two halves** (F15). A 50% hit rate on a convention whose whole premise is that this shape is unreliable. Note also that two *verification steps* (F9, F15) were themselves vacuous — the convention currently aims at prose claims ("so we need no X"); the observed failure here is a **command that cannot fail**, which the same rule would catch if extended: *a verification step whose expectation is "no output" is verified by making it produce output once*. Both F9 and F15 would have been caught by that. Candidate refinement, source agent-emergent, count 2 this session.

5. **Repeated rejection worth recording: "reuse X" as a plan directive keeps outrunning what X's signature actually offers.** The design already caught one instance and recorded it as correction #1 (`design:41`): the brief's "hoist `apply_edit_to_first_rule`" was refuted because the helper "takes no `Profile`, mutates nothing". F1 is the *same failure one level down* — the plan's replacement reuse target (`with_rule_match`) also does not take what the caller has. The pattern is: a reuse mandate is written from the function's **name and purpose**, not from its **signature**. A cheap rule would be: *a plan that mandates reuse of a function names its exact signature and shows the call site's arguments coming from the task's own inputs.* That check, applied to `plan:611`, surfaces F1 immediately.

6. **Bin-name vs package-name is a live trap in this workspace and has now bitten once.** `muxsmith-cli` produces a binary named `muxsmith` (`crates/muxsmith-cli/Cargo.toml:9`), and `src-tauri` is package `muxsmith-gui` with lib `muxsmith_gui_lib` and bin `muxsmith-gui`. Four names, three of them close. F2 is the resulting error. Worth a one-line note wherever `cargo test -p` commands get written.

---

## Out of range

Design decisions I would question but which are settled and are **not** findings against the plan. Recorded here only so they are not mistaken for oversights.

1. **`ProfileDocument`'s `diagnostics` vs `validate_profile`'s `config_diagnostics` envelope** (design `:341-349`). F5 is a finding against the *plan* (no definition step, no shape test, two incompatible descriptions in one bullet). But the underlying tension — a typed `ProfileDocument` alongside `validate_profile_model`'s untyped `serde_json::Value`, both carrying diagnostics in what may be two different shapes — originates in the design and may deserve an owner look independent of Plan 6's plan.

2. **D45's 42-of-43 editable decision vs spec 8.3's tooltip baseline** (F8). The design records `(b)` honestly as "A decision, not a derivation" and argues it well. I am not contesting it. What is unaddressed in the design, and inherited by the plan, is that the 43-key catalog and spec 8.3's "every non-obvious control carries a tooltip" cannot both be satisfied in this plan. F8 routes it as a plan-level fork; if the owner reads it as a design gap instead, that is the more natural home.

---

# Round 2 — re-review of the rewritten plan (2026-07-16)

Judge: same reviewer, same standards. Round-1 non-findings and Verified-clean items stay settled and are not re-litigated. This section grades **the delta**: the plan was renumbered (Task 11 split into 11/12/13; old Task 12 became Task 14), and four findings were resolved by controller/owner decisions I judge for **faithful implementation, not correctness** (they are settled): F1 -> ADR D49, F5 -> `config_only_document` + `"profile"` key, F8 -> tooltips ride Plan 7, F4 -> Task 14 sequenced after Task 13.

## VERDICT: APPROVED

All 18 round-1 findings are fixed. D49 is integrated faithfully and I re-verified its load-bearing tree claims independently. Two new imprecisions surfaced, both **Minor and non-blocking** (a test references a fixture constant that does not exist but is trivially extractable; one incidental type name in a Consumes line). Neither is a fork the implementer must invent a decision to close. Dependency graph is acyclic and correct; no parallel wave-1 stream shares a file; the split left no dangling interface and no stale cross-reference.

## Re-walked coverage table (new numbering)

Design section -> implementing task, walked against the rewritten structure. D49 is now a graded section.

| Design section | Task(s) | Status |
|---|---|---|
| **D41** core writer | Task 2; Task 1 (SaveError supersession); Task 7 step 3 (spec 8.2); Task 9 step 4 (`editor-save-note`); Task 13 (renders note) | COVERED |
| **D42** editor IPC surface | Task 8; Task 1 steps 5/7 (`load_profile` envelope supersession) | COVERED |
| **D43** `apply_suggestion` | Task 6 (core, w/ D49); Task 8 (command); Task 14 (batch UI) | COVERED |
| **D44** ts-rs bindings + drift | Task 5 (now the wave-1 join) | COVERED |
| **D45** registry/widgets/never-arm/check-i18n | Task 9 (registries+catalogs+gate), Task 10 (widgets), **Tasks 11+12+13** (view, split) | COVERED |
| **D46** keyword domains | Task 3 | COVERED |
| **D47** schema as user artifact | Task 7 | COVERED |
| **D48** canonical save omits defaults | Task 4 | COVERED |
| **D49** apply seam (NEW) | Task 6 (reshape + applier + 7 guards), Task 5 (ts derive on `StructuredEdit`), Task 8 (shell mapping + 3 catalog codes), Task 1 step 4 (design §2 catalog repoint) | COVERED |
| **§2** Fluent catalogs | 42 labels + note -> Task 9; save+apply codes -> Task 8 step 4; batch keys -> Task 14; §2 row repoints -> Task 1 steps 3/4 | COVERED |
| **§3** Spec amendments (8.1/8.2/8.4) | Task 7 (single owner, one sweep) | COVERED |
| **§4** parity audit (SI-3) | No task, none required (design's own conclusion) | COVERED |
| **§5** gap table | Transitive via D41/D42/D43 tasks | COVERED |
| **§6** out of scope + editor-8.3 baseline | Task 1 step 6 corrects `:1878-1880` (tooltips -> Plan 7); rest is no-task-by-definition | COVERED |
| **§7** triggers 1-7 + D49 removal trigger | Plan's "Triggers" section (7) + D49 addendum | COVERED |
| **§8** what the implementer must not decide | Task 1 step 7 appends all four rulings | COVERED |

**No double-coverage from the split.** Tasks 11/12/13 carry disjoint responsibilities on one file, serial (11 rule grid+reorder; 12 section composition+dispatch; 13 open/save+note+nav+ipc). **No task invents work the design/D49 do not call for.**

## Per-finding disposition (F1-F18)

Every one opened at its cited task/step and checked against the tree or D49.

- **F1 (Critical) — FIXED.** Resolved by **D49** (`docs/superpowers/specs/2026-07-16-plan-6-apply-seam.md`, 1322 lines, approved four-eyes). I read it in full and re-ran its load-bearing tree claims: `planner.rs:10` is `use serde::Serialize;` (D49's "becomes `use serde::{Deserialize, Serialize}`" targets the right line); `Scalar` is imported at `planner.rs:17`; `delta_for` at `:1809` is private (`fn`, not `pub fn`) with the 2-arg `(edit: &StructuredEdit, scalar: &Scalar)` shape D49 says loses its scalar arg; `ts_rs` appears nowhere in the tree. D49's chosen fix (option ii: `StructuredEdit::{AddExact,AddNotExact}` carry `value: Scalar`; `delta_for` drops the scalar arg and stays private; `apply_suggestion` = `rule_index_of` -> bounds check -> `with_rule_match(profile, i, &delta_for(edit))`; a third `ApplyError::EditChangedNothing` variant detects the `core-44` silent no-op by `applied == *profile`) closes exactly the seam I flagged — the display-string-to-`Scalar` reconstruction the old plan forced. Task 6 integrates it faithfully: it cites D49 by line range for every body rather than copying (its stated anti-drift policy), defers **all** ts concerns to Task 5 (binding point at plan `:609`), lands `ApplyError` with exactly three `pub` variants and no `Deserialize`, and lists all seven guard tests against their D49 sections. **I spot-checked every D49 line-range citation in Task 6 and Task 8** (G1 `:895-933`, G2 `:943-980`, G3 `:989-1019`, unparsable `:1034-1045`, rule-index `:1047-1058`, G4 `:1081-1095`, control `:1100-1112`, harness split `:815-857`, `spliced_scalar` `:864-891`, `yaml_scalar` `:1264-1276`, `P_ALREADY_CONSTRAINED` `:1073-1079`, wire shape `:345-383`, `delta_for` `:415-448`, `apply_suggestion` `:465-519`, `ApplyError` `:522-560`, engine call sites `:454-459`, shell mapping `:597-616`, catalog `:627-641`, ts emit `:1201-1218`, removal trigger `:1122-1137`) — **all resolve to what the plan claims they contain.**
- **F2 (Important) — FIXED.** Task 8 steps 2/5 now use `cargo test -p muxsmith-gui`, and step 2 (plan `:950`) adds the explicit note: "The package is `muxsmith-gui` ... the binary named `muxsmith` is the CLI ... do not confuse them." Verified: `cargo metadata` members are `muxsmith-gui`; `src-tauri/Cargo.toml:2` = `name = "muxsmith-gui"`.
- **F3 (Important) — FIXED.** Task 7 (plan `:681`) is now "the single owner of the v1 spec" and carries all three amendments (8.1 step 2, 8.2 step 3, 8.4 step 4) with **one** self-contradiction sweep (step 5). Task 4's file list (plan `:486-490`) no longer includes the v1 spec, and plan `:491` states so explicitly. Streams A/B/C are now file-disjoint (A: core profile + cli tests; B: `planner.rs`+`suggestions.rs`; C: `README.md`+v1 spec).
- **F4 (Important) — FIXED.** Task 14 is wave 4, "Sequenced **after Task 13**" (plan `:1286`), with the note that its only wave-3-shared file is `e2e/smoke.spec.ts` and running after 13 means no concurrent writer. Wave 3 (Tasks 9-13) is serial in one stream E worktree, so the three prior `smoke.spec.ts` extenders never collide either.
- **F5 (Important) — FIXED** (one minor nit, N1 below). Owner resolution implemented exactly: `load_profile` returns `config_only_document(&diags, None, renderer)` + a `"profile"` key, **no bespoke struct**. Task 1 amends all three design sites (`:311`, `:341-349`, `:404`) — I confirmed `:341` still carries the bespoke `{ profile: Option<Profile>, diagnostics: Vec<Diagnostic> }` in the unamended file, so the supersession has a real target. Task 8 adds a `load_profile_body` mirroring `validate_profile_body` (which I confirmed is at `src-tauri/src/lib.rs:174`) and a shape test (`:915-941`) asserting `load_profile`'s `config_diagnostics` is byte-identical to `validate_profile`'s and that `"profile"` is the only added key. The test is functionally sound: both bodies funnel through `config_only_document`/`rendered_diags`, so the envelopes match; on `ParseError` both produce `[ParseError]` and `"profile": null`.
- **F6 (Important) — FIXED.** Task 9 (plan `:1009`) no longer stages or edits `e2e/catalogs.spec.ts`; it states the file "is **not** a deliverable of this task and needs no work ... globs `locales/<tag>/*.ftl` ... do not stage it, do not edit it." I confirmed in round 1 that `assertAllCatalogsParseCleanly` globs the locale dirs.
- **F7 (Important) — FIXED.** Old Task 11 split into three independently-testable, right-sized tasks: **11** (rule grid + drag-reorder, ends in an e2e reorder assertion), **12** (section composition + widget dispatch over the 13 registries, ends in a per-section/widget e2e assertion), **13** (open/save + save-note + validate-on-edit + nav + `ipc.ts`, ends in a save/disable-Save e2e assertion). Each has its own Files/Interfaces/TDD cycle/commit. Interfaces resolve cleanly: 11 produces the scaffold -> 12 consumes it + Task 10's dispatcher + Task 9's registries -> 13 consumes Task 8's commands + Task 12's composed view. Task 10's "Produces, for Task 12" (plan `:1099`) correctly follows the renumber.
- **F8 (Important) — FIXED.** Owner ruled the editor's 8.3 tooltip baseline rides Plan 7. Task 1 step 6 (plan `:161-173`) rewrites design `:1878-1880`; I confirmed `:1879` still reads "still applies to the editor" in the unamended file. `gui-editor.ftl` stays 43 (Task 9 binding `:1022`, Task 13 binding `:1246`). **Verified ROADMAP `:74-84` carries the editor's 42 tooltip keys** with the F8 provenance note ("Raised by the plan-6 plan review (F8)"). Global Constraints ruling 4 (plan `:20`) records it.
- **F9 (Minor) — FIXED.** Task 6 step 6 (plan `:660-668`) rescopes the grep with `awk '/pub fn apply_suggestion/,/^}/' ... | grep -nE "with_rule_match|delta_for|extend\("`. **I ran the awk range against a structural mock of D49's `apply_suggestion` body** and it captured the function and matched `let applied = with_rule_match(profile, index, &delta_for(edit));` — so the check produces output and can fail, unlike the round-1 `sed` range that never opened. The step says "read from the OUTPUT (not from its absence)" and exempts `with_rule_match`'s own `not`-merge `extend` (a different function, outside the range). Real check.
- **F10 (Minor) — FIXED.** Task 3 step 1 (plan `:393-409`) now explicitly extracts a `schema_json()` helper from the existing inlined invocation, gives its full code, and states extract-and-repoint is "the only legal move" under the file's no-duplicate rule. Matches the real `cli_schema.rs` (two tests, no helper).
- **F11 (Minor) — FIXED.** Task 6 step 1 (plan `:611-625`) copies all seven test bodies "verbatim from D49's guard section", and D49 writes every body out in full (G1 `:900-932`, G2 `:947-979`, G3 `:995-1019`, two arg-failure tests, G4, control). No empty bodies remain.
- **F12 (Minor) — FIXED.** Task 4 step 2 (plan `:543-548`) is relabeled "Baseline - observe guard 1 green ... Expected: **PASS**. This is *not* a TDD red step and cannot be one", citing design `:90-92`; step 4 is named "the real red proof."
- **F13 (Minor) — FIXED.** Task 9 step 1 is now "Write the types and the registries"; step 2 is "Prove the registry-completeness proof fires (deliberate break) ... Now that step 1 has created `registries.ts`." Order corrected.
- **F14 (Minor) — FIXED.** Global Constraints (plan `:27`) now reads "`#![deny(missing_docs)]` is on in `muxsmith-core` and `muxsmith-cli` ... `src-tauri` deliberately opts out (`src-tauri/src/lib.rs:14-17` ...)". Matches the tree.
- **F15 (Minor) — FIXED.** Task 1 step 8 (plan `:199-231`) splits the round-1 line-break-spanning grep into single-alternative greps, each with a stated current occurrence. **Verified against the unamended design**: `suggestion-rule-not-found` occurs exactly once (`:1739`), `D47's` exactly once (`:278`), `carries **43**` at `:1749`, the bespoke-struct string at `:341`, "still applies to the editor" at `:1879` — so each "expected: no output" grep would genuinely fail if its edit were skipped. Non-vacuous.
- **F16 (Minor) — FIXED.** Wave-1 rationale (plan `:41`) now reads "Task 2 does **not** touch `model.rs` (it creates `save.rs`) ... Task 2 and Task 3 are mutually independent (disjoint file sets) and may run in parallel, and Task 4 follows both (`2 -> 4` content, `3 -> 4` shared `model.rs`)." Correct.
- **F17 (Minor) — FIXED.** Task 5 step 8 (plan `:821`) now states "**That hole is closed only in wave 3** ... between this merge and Task 9 nothing imports the bindings ... Until then, step 6's `ls` + inspection is the check." Step 6 (plan `:800`) carries the in-task first-generation check.
- **F18 (Minor) — FIXED.** Task 14 step 3 (plan `:1317`) explicitly directs updating the stale D22 comment at `SuggestionCard.vue:6-13` ("it currently states `edit` 'is deliberately never read' ... which Task 14 falsifies"). I confirmed the comment block (lines 5-13) carries exactly those claims.

## New findings (introduced or surfaced by the rewrite)

### N1 — MINOR. Task 8 step 1 (`plan:922`, `:943`) references a fixture constant `LOADABLE_INVALID_PROFILE` that does not exist

**Evidence.** The shape test writes `std::fs::write(&invalid, LOADABLE_INVALID_PROFILE).unwrap(); // the existing fixture`, and the hedge at `plan:943` says "If the module already names its loadable-invalid fixture something other than `LOADABLE_INVALID_PROFILE`, use that constant." I opened `src-tauri/src/lib.rs`:
- `grep -n "LOADABLE_INVALID_PROFILE\|const.*: &str"` -> **no output**. There are no named `&str` fixture constants in the file.
- The existing loadable-invalid test (`validate_profile_body_reports_validate_diagnostics_for_a_loadable_invalid_profile`, `:566`) writes its YAML as an **inline string literal** (`std::fs::write(&path, "profile_version: 1\n...")`), not a named constant.

So the test as written will not compile (`LOADABLE_INVALID_PROFILE` undefined), and the hedge covers "named something else" but not "not a named constant at all." Secondarily, the same test inlines a fresh malformed-YAML string (`"profile_version: 1\ninput: [not-a-map\n"`) for the parse-error half rather than reusing the `:557` load-failure test's fixture (which is a *missing file*, not malformed content) — mildly at odds with the step's "reuse ... do not introduce a parallel one."

**Why Minor, not blocking.** The intent is unambiguous and closeable without inventing a decision: extract the existing inline YAML from the `:566` test into a module-level `const LOADABLE_INVALID_PROFILE: &str = ...` and repoint both tests at it — the exact "reuse, do not duplicate" the step directs. Both parse-error triggers (missing file vs malformed content) yield a `ParseError` and satisfy the assertion, so correctness is unaffected. It is a wording/precision defect, not a fork.

**Fix.** Change `plan:922`/`:943` to say the loadable-invalid fixture is currently an inline literal in the `:566` test and must be **extracted** into a shared `const LOADABLE_INVALID_PROFILE`, both tests repointed; and either name the parse-error input a fresh deliberate constant or drop the "reuse the existing parse-error fixture" phrasing for that half.

### N2 — TRIVIAL (advisory). Task 11 Consumes line (`plan:1145`) names a type `Rule` that ts-rs does not export

**Evidence.** `plan:1145`: "Consumes: Task 5's `profile.ts` types (`Profile`, `Rule`, ...)." The model's rule struct is `TrackRule` (`crates/muxsmith-core/src/profile/model.rs:198`; D45's export table lists `TrackRule`), so `profile.ts` exports `TrackRule`, not `Rule`. This is incidental colour inside a `(..., ...)` illustrative list, not a normative build instruction, so it strands nothing. Note only; correct to `TrackRule` if touched.

## Fix-round-introduced-defect check

Nothing the rewrite changed contradicts a round-1 Verified-clean item. Specifically re-checked:
- **Dependency graph acyclic and correct.** Edge set `1 -> {2,3,6,7}`, `2 -> 4`, `3 -> 4`, `4 -> 5`, `6 -> 5`, `7` independent (plan `:46`). No `5 <-> 6` cycle: Task 6 lands `StructuredEdit` in its final D49 shape **minus** the `#[cfg_attr(feature="ts",...)]` line and the `use ts_rs::TS;` import (plan `:609`, `:638`), and Task 5 adds exactly those (plan `:742`, `:758`, `:788`). Task 5 also adds `TS` to `Scalar` in the same task, so `derive(TS)` on `StructuredEdit` (which contains `Scalar`) resolves. This is the one-way `6 -> 5` edge the coordinator described, and it matches D49's "D49 cannot land before D44." Acyclic.
- **No parallel-stream file collision.** Streams A (Tasks 2/3/4: core profile + cli tests), B (Task 6: `planner.rs` + `suggestions.rs`), C (Task 7: docs) are file-disjoint. Within A, Task 2 (`save.rs`/`mod.rs`/`profile_save.rs`/`Cargo.toml`) and Task 3 (`model.rs`/`validate.rs`/`cli_schema.rs`/`validate_semantics.rs`) are disjoint, so their "may run in parallel" holds; Task 4 shares `model.rs` with 3 and `profile_save.rs` with 2, so it correctly follows both. Task 5 (join) touches `model.rs`/`planner.rs` but only after A and B merge — sequential, no collision.
- **No stale cross-reference from renumbering.** Swept every `Task N` mention: 11/12/13 are the split editor-view tasks, 14 is batch apply, and every dependency/pointer (`plan:7`, `:999`, `:1190`, `:1245`, `:1338`) names the correct new number. No leftover "Task 12" pointing at the old apply-in-batch task.
- **No new vacuous check.** Every "expected: no output" grep in the rewrite (Task 1 step 8's five splits) was verified against the current tree to have a real current occurrence its edit removes; the positive-count greps ("6 or more", "1 or more", "one hit") are real. Task 6 step 6's grep now fires (tested). No check whose result cannot change.
- **No new latitude.** Swept for explicit clauses and unenumerated normative sets; none introduced. D49's asymmetry (`Exact` -> `Scalar`, substring -> `String`) is enumerated and marked "not the implementer's to reconsider"; the 17/42/43/10 counts remain enumerated by their tables.

## HARVEST (delta)

The round-1 HARVEST stands. What the rewrite adds:

1. **The "reuse X" defect class round-1 HARVEST item 5 predicted recurred once more, and the fix pattern is now visible.** Round 1 noted that reuse mandates get written from a function's *name/purpose*, not its *signature* (the F1 seam, itself a repeat of design correction #1). The rewrite's response — D49 — is the model answer: it opens with a "Verified ground truth" section that **quotes the signature verbatim and runs the tree** before deciding, then writes every downstream body out in full. The residual N1 is the same class in miniature at the *test-fixture* level ("the existing fixture `LOADABLE_INVALID_PROFILE`" named as if it exists, when it is an inline literal). Candidate rule, now with two observed levels (function reuse, fixture reuse): *a reuse reference names the exact existing symbol and is checked to exist; if it is inline today, the instruction is "extract," not "use."*

2. **D49 is a worked example of `proc-proposed-safeguard-stays` applied to its own guards.** It ships seven tests, explicitly analyses that G1/G2 may be construction-tautologies post-landing, and instead of dropping them records a named implementation-time experiment (mutate `delta_for`'s arm to `Scalar::Str(scalar_display(value))`; if only G3 fails, G1/G2 are removable). That is the exact "keep the vacuity analysis, re-aim it at a measurable trigger" shape the convention prescribes — a clean second instance to point at.

3. **The split (F7) confirms the round-1 observation that this plan's enumeration discipline is mature but its *seam* discipline was the weak point.** The rewrite closed every seam I flagged (F1 via D49, F5 via a pinned envelope + shape test, F7 via explicit produces/consumes across 11/12/13). The one seam that slipped through (N1, a fixture reference) is again an *interface between* two well-specified things (an existing test's fixture and a new test), not a defect inside either — the same signature the round-1 HARVEST named. The pattern is consistent enough across two rounds to be worth a ledger note.
