# Plan 12 whole-branch fix report

**Status: DONE.** All eight blocking findings (I-1, I-7, I-2, I-3, I-4, I-5, I-6,
M-4) plus O-1 are fixed. Full eleven-part gate green in the final tree (pasted
at the end). Six commits, each pathspec-scoped:

| Commit | Scope |
|---|---|
| `991ea7c` | I-4: `src-tauri/src/run.rs` |
| `4bfca22` | M-4: `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` |
| `b8b4250` | I-6: `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md` |
| `cbe3895` | I-5: `help/de/view-editor.md`, `help/en/view-editor.md` |
| `9c860d3` | O-1: `eslint.config.js` |
| `d2b622a` | I-1, I-7, I-2, I-3: `e2e/mocks.ts`, `e2e/global.d.ts`, `e2e/smoke.spec.ts`, `e2e/editor-undo-redo.spec.ts` |

Every mutation below was applied in-place, run, then restored via `command cp`
from a scratchpad backup and verified identical by `md5sum` (never by exit
code alone), with `pnpm build` re-run before every Playwright run that
followed an edit or a restore, per the verdict's own environment notes.

---

## I-1: `batch-profile-none` had no producer

**Change:** `e2e/smoke.spec.ts`, the existing "dry-run document renders..."
scenario (`batch view: dry run`). Added an assertion that the empty-state
paragraph is visible before a profile is picked, and hidden once
`batch-profile-current` takes over.

**Mutation (inversion, not removal):** `src/views/BatchView.vue`, swapped
`v-if="!selectedProfile"` / `v-else` to `v-if="selectedProfile"` / `v-else`
(the two paragraphs trade places instead of one being deleted).

```
$ pnpm build   # (exit 0, omitted)
$ npx playwright test e2e/smoke.spec.ts -g "dry-run document renders"
  ✘  1 [chromium] › ... dry-run document renders the resolution table...
    Error: expect(locator).toBeVisible() failed
    Locator: getByTestId('view-batch').getByText('No profile selected yet. ...')
    Expected: visible
    Error: element(s) not found
  1 failed
```

Restored (`command cp` + `md5sum` match), rebuilt, re-ran:

```
$ npx playwright test e2e/smoke.spec.ts -g "dry-run document renders"
  ✓  1 [chromium] › ... dry-run document renders the resolution table...
  1 passed (934ms)
```

---

## I-7: `settings-locale-label.hint` had no producer, and `en(id)` cannot read it

**Change:** `e2e/smoke.spec.ts`, the existing "selecting German in the
settings dialog saves it..." scenario. Added
`expect(dialog.locator("#settings-locale-hint")).toHaveText(enAttr("settings-locale-label", "hint"))`
right after the existing `name("settings-locale-label")` assertion (which
reads the LABEL, never the hint).

**Mutation:** `src/components/SettingsDialog.vue`, changed the hint
paragraph's binding from `$ta("settings-locale-label").hint` to
`$ta("settings-default-jobs-label").hint` (a real, plausible neighbouring
value, not a blank).

```
$ pnpm build   # (exit 0, omitted)
$ npx playwright test e2e/smoke.spec.ts -g "selecting German in the settings dialog saves it"
  ✘  1 [chromium] › ... selecting German in the settings dialog saves it...
    Error: expect(locator).toHaveText(expected) failed
    Locator:  getByTestId('settings-dialog').locator('#settings-locale-hint')
    Expected: "Which language the Muxsmith interface uses. System language follows your operating system and falls back to English where a translation is missing."
    Received: "How many mkvmerge processes to run at the same time during a run. 1 runs jobs one after another."
  1 failed
```

Restored, rebuilt, re-ran:

```
$ npx playwright test e2e/smoke.spec.ts -g "selecting German in the settings dialog saves it"
  ✓  1 [chromium] › ... selecting German in the settings dialog saves it...
  1 passed (655ms)
```

---

## I-2: only bare `Control+z` was ever pressed, and only inside U1

**Change:** `e2e/editor-undo-redo.spec.ts`, new test `I-2: every documented
modifier key and redo spelling drives undo/redo`. Opens a profile, adds three
rules (4 rows total), then drives all six combinations
`help/{en,de}/view-editor.md` documents against the rule count: `Control+z`,
`Meta+z` (undo), `Control+y`, `Meta+y`, `Control+Shift+z`, `Meta+Shift+z`
(redo).

**Mutation A (drop the Meta modifier entirely):** `src/views/EditorView.vue`,
`onEditorKeydown`: `const mod = event.ctrlKey || event.metaKey;` ->
`const mod = event.ctrlKey;`

```
$ pnpm build   # (exit 0, omitted)
$ npx playwright test e2e/editor-undo-redo.spec.ts -g "I-2:"
  ✘  1 [chromium] › ... I-2: every documented modifier key and redo spelling...
    Error: expect(locator).toHaveCount(expected) failed
    Expected: 2
    Received: 3
      671 |     await expect(rows).toHaveCount(2);
  1 failed
```

Restored, rebuilt.

**Mutation B (invert which key triggers which action):**
`onEditorKeydown`'s two branch bodies swapped (`undo()`/`redo()` traded
places between the `z`-without-shift branch and the `shift-z`-or-`y` branch).

```
$ pnpm build   # (exit 0, omitted)
$ npx playwright test e2e/editor-undo-redo.spec.ts -g "I-2:"
  ✘  1 [chromium] › ... I-2: every documented modifier key and redo spelling...
    Error: expect(locator).toHaveCount(expected) failed
    Expected: 3
    Received: 4
      669 |     await expect(rows).toHaveCount(3);
  1 failed
```

Restored (`md5sum` match against the pre-mutation copy), rebuilt, re-ran the
whole file:

```
$ npx playwright test e2e/editor-undo-redo.spec.ts
  17 passed (7.4s)
```

(U1 and every other case in the file still pass -- the fix touches the same
function U1 exercises.)

---

## I-3: the save-marking property was covered in presence, uncovered in direction

**Change:** `e2e/editor-undo-redo.spec.ts`, new test `I-3: an edit made while
Save is in flight is not marked saved -- the dirty guard still fires on the
next Open`. Requires holding one mocked `save_profile` response open while a
real edit lands inside the window -- a same-tick mock resolution is too
narrow for a driven Playwright action to land inside, and the suite has no
`waitForTimeout`-style mechanism to fall back on (deliberately, per its own
established pattern). Added to `e2e/mocks.ts`:

- `gatedWith(value, gate)`: a `MockResult` variant that does not resolve until
  released.
- `releaseGate(page, gate)`: resolves it from the test.
- `global.d.ts` gains the ambient `window.__muxsmithReleaseGate__` type.

The test: opens a profile, clicks Save (captures the pre-edit profile,
pattern `.*`, and blocks on the gate), edits the pattern to
`"edited-during-save"` **while the write is still pending**, releases the
gate, asserts the recorded `save_profile` call's argument still carries the
pre-edit pattern (`.*`), then clicks Open and asserts the discard-confirm
dialog appears (i.e. the editor still reads dirty, because
`"edited-during-save"` was never written).

**Mutation (the exact one the whole-branch verdict named, and previously
invisible: "101 passed" under it before this fix):**
`src/views/EditorView.vue`, `doSave`:
`savedSnapshot.value = JSON.stringify(profile);` ->
`savedSnapshot.value = history.value[position.value];`

```
$ pnpm build   # (exit 0, omitted)
$ npx playwright test e2e/editor-undo-redo.spec.ts -g "I-3:"
  ✘  1 [chromium] › ... I-3: an edit made while Save is in flight is not marked saved...
    Error: expect(locator).toBeVisible() failed
    Locator:  getByTestId('view-editor').getByTestId('confirm-dialog')
    Expected: visible
    Received: hidden
      413 |     await expect(editor.getByTestId("confirm-dialog")).toBeVisible();
  1 failed
```

Restored (`md5sum` match), rebuilt, re-ran the whole file: `17 passed (7.4s)`
(pasted under I-2 above, same run).

---

## I-4: the `CloseDecision` -> dialog-strings and -> action mappings had no producer

**Change:** `src-tauri/src/run.rs`. Extracted two pure functions:

- `close_dialog_strings(decision, locale) -> Option<(&str, &str, &str)>`,
  factored out of `show_close_dialog`'s inline match. The literal
  `ftl_message("close-abort-title", locale)`-style calls were **moved, not
  rewritten to read a key variable** -- they stay textually literal, so
  `every_row_carries_every_key_the_shell_source_literally_looks_up`'s
  `include_str!`-based scan (which does not care which function nests a
  call) still finds every one of them. An earlier draft of this fix returned
  raw key names instead and routed `ftl_message` through a variable at the
  call site in `show_close_dialog`; that would have silently dropped eight of
  the ten close-* keys out of that scan's derived set, the same class of
  defect (a fallback/derivation blind spot) this whole review exists to
  catch. Rejected before it was shipped, not after.
- `close_action(decision) -> CloseAction` (`None` / `ExitDiscard` /
  `AbortAndExit`), factored out of `confirm_close`'s inline match the same
  way.

Two new unit tests, `close_dialog_strings_map_each_decision_to_its_own_wording`
and `close_action_maps_each_decision_to_its_own_effect`, each asserting all
four `CloseDecision` rows against pinned, concrete values (not "non-empty").

**Mutation 1 (the review's own, reproduced):** pointed `ConfirmDiscard` at the
`close-abort-*` keys instead of `close-discard-*`:

```
$ cargo test -p muxsmith-gui --lib run::tests::close_dialog_strings
FAILED
thread '...' panicked at src-tauri/src/run.rs:1509:9:
assertion `left == right` failed
  left: Some(("Abort running jobs", "There is currently a job running. ...", "Abort jobs and quit"))
 right: Some(("Unsaved changes", "The profile in the editor has unsaved changes. ...", "Discard changes and quit"))
```

Restored (`md5sum` match against a pristine backup).

**Mutation 2 (the action mapping's own equivalent swap):** swapped
`ConfirmDiscard`/the run-active arm between `ExitDiscard` and `AbortAndExit`:

```
$ cargo test -p muxsmith-gui --lib run::tests::close_action
FAILED
thread '...' panicked at src-tauri/src/run.rs:1530:9:
assertion `left == right` failed
  left: AbortAndExit
 right: ExitDiscard
```

Restored (`md5sum` match), re-ran:

```
$ cargo test -p muxsmith-gui --lib
test result: ok. 88 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## I-5: the terminology repair, at all five sites (3 German, 2 English)

**Change:** `help/de/view-editor.md` (`:3` "Lege ... an" -> "Erstelle ...",
`:7` "legt ... an" -> "erstellt", `:9` "Anlegen" -> "Erstellen") and
`help/en/view-editor.md` (`:3` "Start a new profile" -> "Create a new
profile", `:7` "starts a profile" -> "creates a profile"), matching the
corpus term already used at each file's own heading (`erstellen`/`create`,
`locales/{de,en}/gui-editor.ftl`). No spec anywhere asserted the old literal
wording (`command grep` for the old strings across `e2e/` returned nothing),
so no test needed updating. `pnpm check:i18n` and the full e2e run both stayed
green (below).

---

## I-6: the falsified mechanism paragraph in the plan75 design doc

**Change:**
`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md:99-102`.
Added the in-place supersession marker (the same form this plan already uses
on D107 decision 3(f)), citing D107 decisions 3(a)/3(b) and the current,
re-verified line numbers in `EditorView.vue` (`330-332` for `saveDisabled`,
`349` for the `watch(model)` gate -- both grepped fresh, not carried over from
the verdict). The paragraph itself is untouched, per the house rule that a
superseded clause stands as the record of what shipped at the time.

The verdict also flagged a second, weaker instance
(`2026-07-21-plan7-help-i18n-design.md:245-252`) and gave its own reasoned
recommendation that it owes nothing (a dated recon inventory citing line
numbers as of a point in time, not a standing mechanism claim -- the same
class the ROADMAP/journal 43-figures were already ruled to owe nothing). I
re-read that paragraph and concur: it is written as a snapshot
("`run.rs::ftl_message` (`:539-560`)..."), not as ground truth asserted to
hold today. Left untouched.

---

## M-4: the ADR's "every current call site is a literal" claim

**Change:**
`docs/superpowers/specs/2026-07-30-plan-12-decisions.md:141`. Replaced the
false clause with a statement of the three non-literal call sites that exist
today (re-measured fresh, not copied from the verdict, since my own I-4 edit
shifted line numbers in `run.rs`):

```
$ command grep -nE 'ftl_message\([a-z_]' src-tauri/src/run.rs
565:fn ftl_message(key: &'static str, locale: &str) -> &'static str {
1727:            let value = ftl_message(key, "en");
1751:        assert_eq!(ftl_message(unknown_key, "en"), unknown_key);
1755:            ftl_message(prefix_of_a_real_key, "en"),
1788:    /// (`ftl_message(key, ...)` over a variable) has no `"` directly after
```

Three real non-literal call sites (`:1727`, `:1751`, `:1755`; the other two
hits are the function definition and a doc comment, not calls), all inside
`#[cfg(test)]`, matching the verdict's own count and nature (one pre-existing
enumeration loop, two probes this same package added). Corrected the ADR
sentence to name this rather than claim it away.

---

## O-1: `v-show` outside the D112 lint rule

**Change:** `eslint.config.js`, `vue/no-restricted-syntax`'s selector:
`/^(if|else-if)$/` -> `/^(if|else-if|show)$/`. Verified no existing
`v-show="!model"` usage would newly trip the widened rule
(`command grep -rn "v-show=" src/**/*.vue` -> only `activeView === '...'`
bindings in `App.vue`, none matching `!model`), so this is a pure widening
with no fallout on the current tree.

**Mutation (fire the rule against the gap it used to miss):**
`src/views/EditorView.vue`, temporarily changed
`v-if="nothingOpenedOrCreated"` to `v-show="!model"` on the `editor-empty`
paragraph.

```
$ npx eslint .
/home/senol/Git/Muxsmith/src/views/EditorView.vue
  951:16  error  A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112)  vue/no-restricted-syntax
✖ 1 problem (1 error, 0 warnings)
```

Restored (`md5sum` match), re-ran:

```
$ npx eslint .
$ echo $?
0
```

---

## Full gate, final state, all six commits landed, working tree clean

```
$ git status --short
(empty)

$ git diff --stat 5cabf32..HEAD
 ...26-07-22-plan75-track-rule-add-remove-design.md |   7 +
 .../specs/2026-07-30-plan-12-decisions.md          |   2 +-
 e2e/editor-undo-redo.spec.ts                       | 106 ++++++++++++-
 e2e/global.d.ts                                    |   5 +
 e2e/mocks.ts                                       |  44 +++++-
 e2e/smoke.spec.ts                                  |  23 +++
 eslint.config.js                                   |  13 +-
 help/de/view-editor.md                             |   6 +-
 help/en/view-editor.md                             |   4 +-
 src-tauri/src/run.rs                               | 172 +++++++++++++++++----
 10 files changed, 341 insertions(+), 41 deletions(-)
```

Rust gate:

```
$ cargo fmt --all --check
(exit 0)

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
(exit 0)

$ cargo test --workspace
... (every suite) test result: ok. 0 failed ...
muxsmith-gui lib: test result: ok. 88 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files
(exit 0)

$ cargo deny check
advisories ok, bans ok, licenses ok, sources ok
(exit 0)

$ cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
(exit 0, two pre-existing "GNU compiler is not supported for this target" advisory warnings, unrelated to this wave)
```

Frontend gate:

```
$ pnpm lint
$ eslint .
(exit 0)

$ pnpm build
✓ built in 155ms
(exit 0)

$ pnpm check:i18n
check-i18n: ok (42 source files scanned, 227 catalog ids, 19 IpcError code(s) gated,
22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked
for parity against 7 en/ catalog(s)).
(exit 0)

$ pnpm test:e2e
... 103 passed (8.2s)
(exit 0; was 101 before this wave -- I-2 and I-3 are each one new `test()`, I-1
and I-7 extended existing tests)
```

House-knowledge gate:

```
$ python3 scripts/ledger-lint.py
ledger-lint: 585 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
(exit 0)
```

Every command above is the literal command run in this session (`command
grep`/`command cp` where the trap applies), exit codes captured per command,
never through a pipeline.
