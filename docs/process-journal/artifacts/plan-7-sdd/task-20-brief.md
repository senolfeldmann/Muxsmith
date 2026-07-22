### Task 20: D61 - number promotion: `ParamValue` end to end

**Stream H** (`.worktrees/plan7-h`), parallel to G. Read D61's number-promotion block.

**Files:**
- Modify: `src-tauri/src/error.rs`, `src-tauri/src/run.rs`
- Modify: `src/ipc.ts`
- Modify: `src/components/SettingsDialog.vue:16`, `src/views/BatchView.vue:48`, `src/views/EditorView.vue` (the params-ref pair - anchors re-verified by text, wave 2 shifted lines)
- Modify: `locales/en/gui-common.ftl`, `locales/de/gui-common.ftl`
- Modify: `e2e/mocks.ts`

**Interfaces:**
- Consumes: nothing from G (file-disjoint).
- Produces: the typed wire `IpcError.params: Record<string, string | number>`.

- [ ] **Step 1: Failing Rust test** (in `error.rs`'s test module): numeric params serialize as JSON numbers, strings as strings:

```rust
    #[test]
    fn params_serialize_untagged_numbers_and_strings() {
        let err = IpcError::new("apply-rule-index-out-of-range")
            .with("index", 3usize)
            .with("rules", 2usize)
            .with("path", "x.yaml");
        let v = serde_json::to_value(&err).unwrap();
        assert_eq!(v["params"]["index"], serde_json::json!(3));
        assert_eq!(v["params"]["path"], serde_json::json!("x.yaml"));
    }
```

Run: `cargo test -p muxsmith-gui params_serialize` (use the shell crate's actual test invocation). Expected: FAIL to compile (`.with` takes `Into<String>`).

- [ ] **Step 2: Implement `ParamValue`.** In `error.rs`:

```rust
/// One `IpcError` param value: a number for numeric-semantic params
/// (`index`, `rules`), a string for everything else. `#[serde(untagged)]`
/// keeps string params serializing exactly as before while numeric params
/// become JSON numbers, so Fluent can apply CLDR plural rules (D61,
/// i18n-05).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ParamValue {
    /// A numeric param, rendered by Fluent as a number.
    Num(u64),
    /// A string param (the default; includes third-party passthrough text).
    Str(String),
}

impl From<&str> for ParamValue {
    fn from(v: &str) -> ParamValue {
        ParamValue::Str(v.to_owned())
    }
}
impl From<String> for ParamValue {
    fn from(v: String) -> ParamValue {
        ParamValue::Str(v)
    }
}
impl From<usize> for ParamValue {
    fn from(v: usize) -> ParamValue {
        ParamValue::Num(v as u64)
    }
}
```

`params` becomes `HashMap<String, ParamValue>`; `.with` takes `value: impl Into<ParamValue>`.

- [ ] **Step 3: The complete promotion-site set** (the four listed lines and no others; every other `.with` site stays a string by its `From<&str>`/`From<String>` impl unchanged): in `error.rs`'s `From<ApplyError>` arm, `.with("index", index.to_string())` -> `.with("index", index)` and `.with("rules", rules.to_string())` -> `.with("rules", rules)` (RuleIndexOutOfRange), `.with("index", index.to_string())` -> `.with("index", index)` (EditChangedNothing); in `run.rs`'s `get_job_log_in` not-found closure, `.with("index", index.to_string())` -> `.with("index", index)`.

- [ ] **Step 4: Run the Rust test + shell suite.** Expected: PASS; clippy/fmt clean.

- [ ] **Step 5: TS side.** `src/ipc.ts`: `params: Record<string, string | number>`. The three ref sites (typed `Record<string, string>` today) become `Record<string, string | number>`: `SettingsDialog.vue` (`errorParams`), `BatchView.vue` (`ipcErrorParams`), `EditorView.vue` (`ipcErrorParams`). All eight render sites (`FirstRun.vue`, `RunHistory.vue` x2, `JobsView.vue` x2, plus the three ref-fed `$t` sites) pass params through unchanged - fluent-vue accepts `string | number`; no per-site promotion table exists for IpcError. The `Diagnostic` wire is untouched.

- [ ] **Step 6: The plural selector - the ONLY message that gains one** (`$index` is an identifier, never plural-selected). en:

```ftl
apply-rule-index-out-of-range = The suggestion could not be applied: no rule at index { $index } (rule count: { $rules ->
        [one] 1 rule
       *[other] { $rules } rules
    }).
```

de:

```ftl
apply-rule-index-out-of-range = Der Vorschlag konnte nicht angewendet werden: keine Regel an Index { $index } (Regelanzahl: { $rules ->
        [one] 1 Regel
       *[other] { $rules } Regeln
    }).
```

(If stream G's rule 5 is already on master at merge time, it guards this pair; land en+de in this same commit regardless.)

- [ ] **Step 7: e2e mocks sweep.** `e2e/mocks.ts`: the `MockResult` reject shape and `rejectWith` take `Record<string, string | number>`. Sweep for fabricated IpcErrors carrying numeric-semantic params as strings:

```bash
grep -rn "rejectWith\|params:" e2e/*.ts | grep -E "index|rules"
# Expected: no stringly numeric fabrications remain (currently none exist -
# fire-verify the grep by temporarily adding
# rejectWith("apply-rule-index-out-of-range", { index: "3" }) to a spec,
# seeing the grep hit, removing it).
```

- [ ] **Step 8: Full frontend gate + full Rust gate.** Expected: green (check-i18n: the selector's variant keys are plural-category keys - rule 5's carve-out admits them once G lands; before G merges, the old check 3 ignores pattern bodies entirely).

- [ ] **Step 9: Commit**

```bash
git add src-tauri/src/error.rs src-tauri/src/run.rs src/ipc.ts src/components/SettingsDialog.vue src/views/BatchView.vue src/views/EditorView.vue locales/en/gui-common.ftl locales/de/gui-common.ftl e2e/mocks.ts
git -c commit.gpgsign=false commit -m "shell: IpcError params typed string|number via untagged ParamValue; four promotion sites; plural selector on apply-rule-index-out-of-range (D61)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Wave 4

