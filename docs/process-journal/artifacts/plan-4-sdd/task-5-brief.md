### Task 5: `tests/support` consolidation + tempdir-leak fix [WAVE 1 - independent]

**Files:**
- Create: `crates/muxsmith-core/tests/support/mod.rs`
- Modify: `crates/muxsmith-core/tests/planner_resolution.rs`, `suggestions.rs`, `command_integration.rs` (use the shared helpers; fix `std::mem::forget`)

**Interfaces:**
- Produces: `support::{FakeIdent, lang}` - `FakeIdent` exactly as today (`by_name: HashMap<String, Identification>` + `Identify` impl, from `command_integration.rs:78-90`), `lang()` returning the 3-row en/de/tr `LanguageIndex`.

- [ ] **Step 1:** Create `tests/support/mod.rs` (a `tests/` SUBDIRECTORY module - not compiled as its own test crate) with the two helpers, `#[allow(dead_code)]` on items where a consumer file uses only part. Each of the three test files: `mod support;` + `use support::{FakeIdent, lang};`, deleting the local copies.
- [ ] **Step 2:** Fix the 15 `std::mem::forget` sites (planner_resolution.rs:59,401,453,592,662,825,888,1153,1193,1235,1376,1416,1455; suggestions.rs:56,152): change the owning helpers (e.g. `plan_one`) to RETURN the `TempDir` alongside their value (callers bind `let (batch, _dir) = ...`), so directories are cleaned on drop instead of leaked. Mechanical; the compiler finds every caller.
- [ ] **Step 3:** Full gate green (this task is pure test refactor; zero behavior change - the suite itself is the spec).
- [ ] **Step 4: Commit** - `test: shared support module (FakeIdent, lang) and tempdir-leak fix (D18)`

---

