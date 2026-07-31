## Task 6: the shell learns the save state, and one close prompt covers both reasons (W4b)

Read first: this plan's D109 decisions 4 and 5 and **D110 in full**; `src-tauri/src/run.rs`'s `CloseDecision`, `close_decision`, `on_close_requested`, `abort_and_quit`, `ftl_message` and the two dialog-string tests, plus its existing `close_decision` unit cases; `src-tauri/src/lib.rs`'s `AppState`, its `Default` impl and the single `invoke_handler` registration; **`crates/muxsmith-cli/src/i18n.rs`'s `LOCALES` table and `Renderer::new`**, the house pattern D110 conforms to; `locales/en/gui-common.ftl`'s `close-abort-*` block and `locales/de/gui-common.ftl`'s header note about them; **`scripts/check-i18n.mjs`'s `RUST_ONLY_IDS` block and the comment above it**; `src/i18n/fluent.ts` (`currentLocale`, and the two places `applyLocale` is called from); `src/ipc.ts` (the wrapper shape this task follows); `src/App.vue`'s script setup; `src/views/EditorView.vue` as Task 5 left it. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src-tauri/src/lib.rs` (the two `AppState` fields and their defaults, the `set_editor_dirty` and `set_shell_locale` commands, their handler registrations)
- Modify: `src-tauri/src/run.rs` (the locale table and the locale-aware `ftl_message`, the four-variant `CloseDecision`, `close_decision`, `on_close_requested`'s dialog selection and its confirm actions, the extended dialog-string test, the new per-locale parity test, the new decision-matrix tests)
- Modify: `locales/en/gui-common.ftl` (six new single-line ids)
- Modify: `locales/de/gui-common.ftl` (the same six)
- Modify: `src/ipc.ts` (a `setEditorDirty` and a `setShellLocale` wrapper)
- Modify: `src/views/EditorView.vue` (one watcher on `dirty`)
- Modify: `src/App.vue` (one watcher on `currentLocale`, pushing it to the shell)
- Modify: `scripts/check-i18n.mjs` (**the `RUST_ONLY_IDS` allowlist only** - the enumeration of shell-consumed ids this task's six new keys falsify; no other change to the script)
- Modify: `e2e/smoke.spec.ts` (the wire assertions for both syncs)

**Interfaces:**
- Consumes: Task 4's `dirty`; `currentLocale` from `src/i18n/fluent.ts`.
- Produces: `set_editor_dirty`, `set_shell_locale`, the four-state close decision, and a locale-aware shell lookup.

- [ ] **Step 1: the shell state and the two commands.** `AppState` gains `editor_dirty: AtomicBool` (defaulted false) and `dialog_locale: Mutex<String>` (defaulted `"en"`), each with a doc comment stating that it mirrors a frontend value, that the frontend is its only writer, and what a failed sync costs (a stale flag, or a stale dialog language - never a missing dialog). Add `#[tauri::command] fn set_editor_dirty(dirty: bool, state: State<AppState>)` and `#[tauri::command] fn set_shell_locale(locale: String, state: State<AppState>)`, both registered in the one `invoke_handler` list. **The shell does not resolve a locale of its own and `sys-locale` is NOT added to `src-tauri`** (D110 decision 2): `effectiveLocale` stays the product's single resolution rule.

- [ ] **Step 1b: the locale-aware lookup, split into a row step and a chain step.** Add `const DE_GUI_COMMON: &str = include_str!("../../locales/de/gui-common.ftl");` beside the existing en constant and a `LOCALES: &[(&str, &str)]` table carrying `("en", GUI_COMMON_FTL)` and `("de", DE_GUI_COMMON)`, in the shape `crates/muxsmith-cli/src/i18n.rs` uses and with a doc comment naming that file as the pattern it follows and the reason the table is hand-written. Then two functions, and the split is prescribed rather than left to taste:
  - `fn lookup_in(catalog: &'static str, key: &str) -> Option<&'static str>` carries the line parse - column-0 `key = value`, trimmed, never prefix-matching - and returns `None` when the catalog has no such line. This is the existing `ftl_message` body with its `unwrap_or` removed.
  - `fn ftl_message(key: &'static str, locale: &str) -> &'static str` collapses `locale` to its primary subtag (everything before the first `-`, lowercased), then walks `[requested, en]` over `lookup_in`, then returns `key`.

  **The split is what makes the parity test possible**, and the doc comment on `lookup_in` says so in one clause: an assertion made through the chain is green under every mutation upstream of the en fallback, so the check calls the row step directly (Step 5). The existing single-line, column-0 and never-prefix-match properties are preserved exactly, and the chain function's doc comment keeps its recorded reason for not being a Fluent parser, extended with the second locale. Call sites read the locale from `AppState`.

- [ ] **Step 2: the decision matrix.** `CloseDecision` gains `ConfirmDiscard` and `ConfirmAbortAndDiscard`, each documented. `close_decision` reads both facts and returns exactly the four-row table D109 decision 5 fixes. `on_close_requested` selects the title/message/confirm triple per variant, keeps `close-abort-dismiss` as the cancel label for all three confirming variants, and on confirmation runs `abort_and_quit` for the two run-bearing variants and exactly `app.exit(0)` for `ConfirmDiscard`, no run existing there to abort. **The code is fenced rather than left to the implementer**: the neighbouring site passes one through a closure (`abort_and_quit(&app.state::<AppState>(), |code| app.exit(code))`), so an unwritten literal would be an invented value. The unchanged `Close` path still returns before `prevent_close`.

- [ ] **Step 2b: the re-read on the confirming branch (D109 decision 9).** Add the pure rule beside `close_decision`, factored off the Tauri types for the same recorded reason:

```rust
/// Whether a confirmed close still needs asking about (D109 decision 9).
/// `Some(current)` when the state now carries a fact the dialog the user
/// answered did not state, `None` when nothing was added - the state
/// weakened, or did not move. One re-read only: the caller acts on the
/// answer to the prompt this returns and never reads again.
fn reconfirm_decision(answered: CloseDecision, current: CloseDecision) -> Option<CloseDecision>
```

  Its body compares the two facts each variant stands for - run-abort and discard - and returns `Some(current)` exactly when `current` names one the `answered` variant did not. Then, in the dialog callback's confirming branch and **before** `abort_and_quit` or `app.exit(0)`: re-read `close_decision(&app.state::<AppState>())`, pass it through `reconfirm_decision`, and on `Some(v)` show `v`'s own dialog - the same construction as the first pass, with `v`'s title, message and confirm label - whose callback is **terminal**: confirming performs `v`'s action from decision 5's table, declining returns without arming anything and leaves the window open. On `None`, proceed exactly as today. **No fifth message is added and no catalog key changes**; if the implementer concludes one is needed, that is NEEDS_CONTEXT, because the message set is owner-visible.

- [ ] **Step 3: the catalog, both locales, fenced, single-line by the shell's own parser constraint.** Append to `locales/en/gui-common.ftl` after the `close-abort-*` block exactly:

```
close-discard-title = Unsaved changes
close-discard-message = The profile in the editor has unsaved changes. Quit and lose them?
close-discard-confirm = Discard changes and quit
close-abort-discard-title = Running jobs and unsaved changes
close-abort-discard-message = A job is running and the profile in the editor has unsaved changes. Abort all running jobs, discard the changes and quit?
close-abort-discard-confirm = Abort jobs, discard changes and quit
```

  and to `locales/de/gui-common.ftl` in the same position exactly:

```
close-discard-title = Nicht gespeicherte Änderungen
close-discard-message = Das Profil im Editor hat nicht gespeicherte Änderungen. Beenden und verwerfen?
close-discard-confirm = Änderungen verwerfen und beenden
close-abort-discard-title = Laufende Jobs und nicht gespeicherte Änderungen
close-abort-discard-message = Derzeit läuft ein Job und das Profil im Editor hat nicht gespeicherte Änderungen. Alle laufenden Jobs abbrechen, die Änderungen verwerfen und beenden?
close-abort-discard-confirm = Jobs abbrechen, Änderungen verwerfen und beenden
```

  **Both locales' values are read**, through the locale-aware lookup Step 1b builds, so a German user reads the German text; Step 5's part (c) pins one of them. Two properties are unchanged and bind both files equally: every value stays **single-line and column-0**, because the shell's parser is a line lookup and not a Fluent parser, and no value carries an attribute. The de catalog header's note that the `close-abort-*` strings "are not yet shown to a de user ... for parity and a later shell i18n" is **consumed by this task, not extended**: those four strings become readable by the same change, and the header's forward-looking clause is a close-action disposition rather than a standing limitation.

- [ ] **Step 4: the two frontend syncs.** `src/ipc.ts` gains `setEditorDirty(dirty: boolean)` and `setShellLocale(locale: string)`, both documented beside their siblings. `EditorView` gains `watch(dirty, (value) => { void setEditorDirty(value).catch(() => { /* background bookkeeping */ }); });`. `App.vue` gains `watch(currentLocale, (locale) => { void setShellLocale(locale).catch(() => { /* background bookkeeping */ }); }, { immediate: true });` - **`immediate` is load-bearing and is commented as such**: `main.ts` applies the locale before the app mounts, so without it the shell would hold `"en"` until the user changed the language. Both watchers carry the tolerance comment and its named consequence, mirroring the view's existing tolerance for its recents write. `App.vue` is otherwise untouched.

- [ ] **Step 5: the Rust tests, three groups.**
  - Extend `close_abort_strings_resolve_from_the_ftl_catalog`'s key enumeration with the six new ids (the enumeration is the point of that test, so it is a named region), keeping its pinned reference-wording assertion and passing `"en"` explicitly now that the lookup takes a locale.
  - Add four `close_decision` cases, one per matrix row, each constructing the state it names: idle-and-clean, run-and-clean (the existing case covers this row and is extended rather than duplicated), dirty-and-idle, dirty-and-running. Assert the exact variant per row.
  - **Add the `reconfirm_decision` matrix, exhaustively rather than by example.** Three `answered` variants (the three that produce a dialog) against all four `current` variants, twelve cells, each asserting `Some(v)` or `None` explicitly. **Exhaustive because a pure function over a four-value enum admits it**, and because the two halves of the observable are opposite cells of the same table: the strengthening cells are the "second prompt appears" side and the weakening and unchanged cells are the "it does not appear" side, so a table that skipped either would cover one side of a two-sided consequence. Name the cells that must be `None` in the report as well as the ones that must be `Some`, since the silent-no-prompt cells are the ones a broken rule would pass.
  - **Add the shell parity test (D110 decision 4), derived rather than hand-listed, split three ways so its assertions sit BELOW the `[requested, en]` chain.** (a) Read the `locales/` directory at test time under `env!("CARGO_MANIFEST_DIR")` and assert every locale directory has a row in the shell's `LOCALES` table. (b) Derive the shell's consumed key set from `include_str!("run.rs")` with a regex over `ftl_message("...")` literals, then for **every row** assert `lookup_in(row_catalog, key)` is `Some` and non-empty - **called on the row directly, never through `ftl_message`**. (c) Assert `ftl_message("close-abort-title", "de") == "Laufende Jobs abbrechen"`, the German mirror of the existing test's pinned en wording.
    **Each of the three prescribed red states names the part that must fail AND the parts that must not**, so a mutation cannot be satisfied by the wrong assertion, and where one mutation trips a second assertion as well, that is stated rather than left to inference:
    - Point the `de` row at the en catalog: **(c) fails**; (a) and (b) pass, because the row exists and the en catalog holds every key.
    - Delete **`close-discard-title`** from `locales/de/gui-common.ftl` - **the key is named because the choice matters**: it is one this package adds, so the mutation exercises the new surface, and it is deliberately NOT the key (c) pins, so **(b) fails** while (a) and (c) pass. Deleting the pinned key instead would fail (c) as well and falsify the stated must-not-fail half.
    - Delete the `de` row from the table: **(a) fails, and (c) fails with it** - the chain then finds no requested row and falls through to en, so the pinned German value is not returned. (b) passes, because it iterates only the rows that exist. **Two failures from one mutation is a property of the design, not a defect**, and it is written here so an implementer meeting two does not read the second as a surprise.

    Run all three, paste all three failures, restore, then paste the green run. **A red state that produces no failure is a defect in the test, not in the mutation**, and returns as NEEDS_CONTEXT.
    **Why the lookup is split at all** (Step 1b): an assertion made through the composed chain is green under every mutation upstream of the en fallback, which is what an earlier draft of this plan got wrong. **The derivation must not be replaced by a literal list** - a hand-written list is the blind spot this test exists to remove, and a key added to the shell later would not join it. Two residuals go in the report, not into the test: a non-literal `ftl_message` argument would be invisible (every current call site is a literal), and `crates/muxsmith-cli/src/i18n.rs` has the identical unserved-locale gap, which this task SURFACES and does not fix.

- [ ] **Step 6: the wire tests, both syncs.** In `e2e/smoke.spec.ts`: after an edit, a recorded `set_editor_dirty` call carrying `true`, and after a successful Save one carrying `false` (both halves, because a flag that only ever sets is worse than none); and the shell-locale sync, **both halves asserted against concrete values rather than against "whatever was applied"**: a recorded `set_shell_locale` call at startup whose argument equals **`"en"`** - determined, not open, because `smoke.spec.ts`'s scenarios take `get_settings` from the mock default, which returns `locale: "en"`, so `effectiveLocale("en")` is `"en"` - plus a second call whose argument equals `"de"` after the settings dialog switches the language. The live half is what makes the pair non-vacuous: a shell told once and never again passes the startup half and fails the user.

- [ ] **Step 6b: the allowlist.** Add the six new ids to `RUST_ONLY_IDS` in `scripts/check-i18n.mjs`, beside the four `close-abort-*` keys already there, and extend that block's comment to say the set is shell-consumed rather than naming D31 alone. Nothing else in the script changes. **Verification for this step is the run itself:** `pnpm check:i18n` must report no unused-id warning for any of the six, and the pre-state run before the allowlist edit must report exactly those six - paste both.

- [ ] **Step 7: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green - `cargo test --workspace` covers the three Rust test groups, `pnpm check:i18n` covers cross-locale parity for the six new ids plus the allowlist, and the cross-target clippy part covers the two new commands on Windows without linking. `git diff --stat` covers exactly the nine files in the Files list.

- [ ] **Step 8: commit.**

```bash
git add src-tauri/src/lib.rs src-tauri/src/run.rs locales/en/gui-common.ftl locales/de/gui-common.ftl src/ipc.ts src/views/EditorView.vue src/App.vue scripts/check-i18n.mjs e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "shell: closing with unsaved editor changes confirms, in one prompt and in the language the UI is using" -- src-tauri/src/lib.rs src-tauri/src/run.rs locales/en/gui-common.ftl locales/de/gui-common.ftl src/ipc.ts src/views/EditorView.vue src/App.vue scripts/check-i18n.mjs e2e/smoke.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the four-row matrix and that it yields ONE prompt; **that the re-read happens once, on the confirming branch, before the action, and that a strengthening is its only trigger**; that the second prompt is an existing variant's dialog and no fifth message is added; that declining the second prompt returns to the app without arming a quit; that the combined case gets its own message rather than composed prose; that the discard-only confirmation exits rather than aborting anything; the six fenced strings and their single-line form in both locales; **that the shell is told the locale rather than resolving one, and that `sys-locale` is therefore not added**; that the locale table follows the CLI's shape; that the parity test derives its key set from the source rather than listing it; that the `check-i18n.mjs` edit is the allowlist and nothing else; that the CLI's identical unserved-locale gap is surfaced and not fixed; that both frontend syncs are tolerant; that the dialog-string test's enumeration is extended.

---

