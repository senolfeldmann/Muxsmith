# Task 3 report: New creates a blank profile, and `currentPath` keeps only its path duties (W2)

**Status: DONE.** Commit **`2cc0650`**, four files, unsigned, one trailer. The full 11-part
gate is green on the committed state.

**One deviation from the plan's fenced text, and it is authorised in writing.** Step 3's
fenced `doSave` body does not type-check. I returned it as NEEDS_CONTEXT rather than patching
it; the controller reproduced the failure independently and ruled:

> `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-12/task-3-ruling-1.md`
> -- controller ruling 1, option A: **inside the `try`, the FIRST guard becomes
> `if (path === null)`**, everything else in the fenced block byte-identical, one comment owed
> at the site. The ruling sits below the plan and above this brief for that one deviation and
> is silent on everything else; no plan amendment is owed, and the reviewer reads both.

**A reviewer walking the brief against the diff will find exactly one token that differs from
the fenced block. That token is the ruling's, not mine.** Nothing else in the diff departs
from the brief.

Read the fork section first (it is the reasoning the ruling rests on), then the step-by-step.

---

## The fork: the fenced `doSave` block does not type-check

**Routed, ruled, closed.** Raised as NEEDS_CONTEXT; reproduced independently by the controller
on the tree I left deliberately unresolved; decided in `task-3-ruling-1.md` as option A. The
diagnosis and options below are kept because the ruling rests on them and a reviewer grading
the one-token difference needs the reasoning, not just the verdict.

### What was observed

Step 3 prescribes the `doSave` body "exactly this shape". Applied verbatim, `pnpm build`
(the gate's `vue-tsc --noEmit` part) fails. Pasted from the run against the tree as it
currently stands:

```
$ pnpm build
$ vue-tsc --noEmit && vite build
src/views/EditorView.vue(354,23): error TS2345: Argument of type 'string | null' is not assignable to parameter of type 'string'.
  Type 'null' is not assignable to type 'string'.
src/views/EditorView.vue(359,53): error TS2345: Argument of type 'string | null' is not assignable to parameter of type 'string'.
  Type 'null' is not assignable to type 'string'.
[ELIFECYCLE] Command failed with exit code 2.
```

The two sites are `await saveProfile(path, profile)` and `await rememberRecentProfile(path)`.

### Why, mechanically

`let path = currentPath.value;` has declared type `string | null`. TypeScript's control-flow
analysis of an aliased condition (`const needsPath = path === null;` used as `if (needsPath)`)
narrows the aliased variable **only when that variable is a `const`, a `readonly` property, or
an un-modified `let`/parameter**. `path` is a `let` that IS reassigned (`path = picked;`), so
the alias carries no narrowing. Consequently the then-branch ends with `path: string`, the
implicit else-branch keeps `path: string | null`, and the join after the `if` is
`string | null` -- which neither `saveProfile(path: string, ...)` nor
`rememberRecentProfile(path: string)` accepts.

This is not a mock, a fixture or an environment artifact: it is the compiler's reading of the
prescribed text, and it reproduces deterministically.

### The collision, stated as two written statements

- The plan/brief Step 3: replace the `doSave` body with **exactly this shape** (and D107
  decision 5 fixes the behaviour it encodes).
- Global Constraints: **the gate as `BUILDING.md` enumerates it**, run foreground, no subsets,
  green -- and `pnpm build` (vue-tsc) is one of its four frontend parts.

Both are festgeschrieben; they cannot both be satisfied. That is the escalation criterion
"two fixed statements collide", so this is routed rather than decided at the keyboard.

### Options, with costs

**Option A (recommended): change one token, `if (needsPath)` -> `if (path === null)`.**

```ts
    if (path === null) {          // was: if (needsPath)
      const picked = await saveDialog({ ... });
      if (typeof picked !== "string") { return; }
      path = picked;
    }
    await saveProfile(path, profile);
```

- The direct `path === null` test narrows `path` in both branches: `string` at the end of the
  then-branch (assigned from `picked`, which is `string` past the `typeof` guard) and `string`
  in the else-branch (the failed null test). The join is `string`.
- `needsPath` survives untouched and still gates the recents write, so D107 decision 5's "only
  when the path was newly established" is encoded exactly as prescribed.
- **Behaviour is provably identical**: `path` is not written between `const needsPath = path
  === null;` and the `if`, so the two expressions evaluate the same boolean at the same moment.
- Cost: the fenced block is no longer byte-identical to the plan. One token.
- Measured: with this applied, the **entire 11-part gate is green** (pastes in section "Step 8"
  below), including every pre-existing test unchanged.

**Option B: `await saveProfile(path as string, profile)` plus the same cast at
`rememberRecentProfile`.**

- Keeps the `if (needsPath)` line byte-identical.
- Cost: two casts that assert away a genuine nullable the compiler is right about. If a future
  edit ever reorders the branch, the cast silences the compiler instead of catching it. Also
  two deviations rather than one.

**Option C: introduce a separate `const targetPath: string` after the branch.**

- Cost: the largest structural deviation of the three, and it duplicates `path`'s role.

**Option D: amend the plan's fenced block to Option A's text and re-dispatch.**

- Correct if the controller wants the plan document and the code to stay byte-identical, which
  is the property the fencing exists to protect. Cost: a plan edit plus a fix round; the code
  change is the same one token either way.

**Recommendation: Option A**, with the plan's fenced block amended to match (i.e. A and D
together) if byte-identity between plan and code is a property the controller wants preserved
for the reviewer. Option A alone is sufficient for correctness; the pairing is about the
document.

### What was ruled, and what landed

**Option A, adopted.** The ruling rejects B (two `as string` casts: a cast suppresses the
checker where the guard satisfies it, and would leave two assertions a later change to the
branch structure could silently falsify) and C (a second `const`: a new name for a value that
already has one, in a block whose whole point is a discipline about which value is used
where). It declines D: the fenced block's purpose is the capture-before-the-await discipline
and the branch structure, both preserved intact, so a four-eyes amendment would spend two
rounds making a document match code its own reviewer is already grading against the ruling.

Landed in `doSave`, and nothing else in the block moved:

- The **first** guard inside the `try` is now `if (path === null)`.
- `const needsPath = path === null;` is untouched, and so is the **second** `if (needsPath)`
  that gates the recents write. That site must keep asking the original question: after the
  dialog branch runs, `path === null` is false, and D107 decision 5 gates the recents write on
  the path having been **newly** established. The constant is what carries that question
  across the branch, which is why the plan introduced it.
- **A comment at the site**, added on the ruling's explicit instruction, stating the narrowing
  reason and why the two conditions are deliberately not unified -- located by symbol, never
  by line number (`comments-locate-by-symbol-never-by-line-number`). Without it the two
  conditions read as redundant side by side and a later simplifier would collapse them back
  and re-break the build.

Behaviour is identical by construction, not by measurement: between `const needsPath = path
=== null;` and the first guard the block writes only `saving.value` and `ipcErrorCode.value`,
so `path` is unwritten and the two conditions are equal at that point.

---

## Step 1: re-measure the seed before writing it

Run in full, on both instruments the authoring section used. **The measurement agrees with the
authoring result: the chosen seed S4 produces no error-severity diagnostic.**

### A note on the seed count, surfaced rather than resolved

Step 1 says "the four candidate seeds"; the authoring section it points at (`## Authoring-time
verification`, "The two measurements the brief demanded") enumerates **five** (S1 schema
minimum, S2 schema-minimum + one empty rule, S3 empty pattern + one extension + one empty rule,
S4 `.*` + one extension + one empty rule, S5 passthrough). D107's rejected-alternatives list
covers S1, S2 and S5, and both S3 and S4 are named as diagnostic-free with S4 chosen. The
qualifier "exactly as the authoring section did" settles the execution: I ran all five, which
is a superset of any four and cannot under-satisfy the step. Not treated as a fork, since no
reading of the step is left open by running everything it points at. Surfaced because the
number is wrong in a normative step and Tasks 4-7 inherit the same brief style.

### Instrument 1: the CLI, `./target/debug/muxsmith validate <seed> --json`

`crates/muxsmith-cli/src/commands/validate.rs` calls `validate::config_diagnostics_from_file`,
reaching `config_diagnostics` by the same funnel `src-tauri/src/lib.rs`'s
`validate_profile_model_body` uses. Binary rebuilt at the current head first
(`cargo build -p muxsmith-cli` -> `Finished dev profile ... in 0.13s`). Exit codes captured
with `$?` directly, not through `PIPESTATUS` (zsh).

```
########## s1
profile_version: 1
input:
  pattern: ""
  extensions: []
tracks:
  rules: []
--- output:
{"diagnostics":[{"code":"empty-extensions","config_path":"input.extensions","params":{},"rendered":"[error] input.extensions: The extensions list must not be empty.","severity":"error"},{"code":"no-track-rules","config_path":"tracks.rules","params":{},"rendered":"[error] tracks.rules: The profile defines no track rules; add at least one rule, or set tracks.unmatched: keep for a pure passthrough remux.","severity":"error"}]}
exit=2
########## s2
profile_version: 1
input:
  pattern: ""
  extensions: []
tracks:
  rules:
  - match: {}
--- output:
{"diagnostics":[{"code":"empty-extensions","config_path":"input.extensions","params":{},"rendered":"[error] input.extensions: The extensions list must not be empty.","severity":"error"},{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"rendered":"[warning] tracks[0].match: This match expression is empty and would match every track.","severity":"warning"}]}
exit=2
########## s3
profile_version: 1
input:
  pattern: ""
  extensions:
  - mkv
tracks:
  rules:
  - match: {}
--- output:
{"diagnostics":[{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"rendered":"[warning] tracks[0].match: This match expression is empty and would match every track.","severity":"warning"}]}
exit=1
########## s4
profile_version: 1
input:
  pattern: .*
  extensions:
  - mkv
tracks:
  rules:
  - match: {}
--- output:
{"diagnostics":[{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"rendered":"[warning] tracks[0].match: This match expression is empty and would match every track.","severity":"warning"}]}
exit=1
########## s5
profile_version: 1
input:
  pattern: .*
  extensions:
  - mkv
tracks:
  unmatched: keep
  rules: []
--- output:
{"diagnostics":[{"code":"passthrough-profile","config_path":"tracks.rules","params":{},"rendered":"[info] tracks.rules: This profile defines no track rules and tracks.unmatched is keep: a pure passthrough remux; every primary track is copied unchanged. If this is not intended, add track rules.","severity":"info"}]}
exit=0
```

### Instrument 2: the model path, JSON in -> `validate::config_diagnostics`

Because the seed is an in-memory object and the GUI command deserializes it from the JSON the
IPC wire carries, exactly as this probe does. A throwaway crate **outside the repo** (own
`[workspace]`, own target dir, path dependency on `muxsmith-core`; `git status --porcelain`
confirmed empty before and after). Full output:

```
########## S1 schema minimum
--- json in: {"profile_version":1,"input":{"pattern":"","extensions":[]},"tracks":{"rules":[]}}
--- config_diagnostics (2 total):
    Error EmptyExtensions at input.extensions
    Error NoTrackRules at tracks.rules
--- save::to_string (yaml):
    |profile_version: 1
    |input:
    |  pattern: ''
    |  extensions: []
    |tracks:
    |  rules: []
--- yaml round-trip equals model: true
########## S2 schema minimum + one empty rule
--- json in: {"profile_version":1,"input":{"pattern":"","extensions":[]},"tracks":{"rules":[{"match":{}}]}}
--- config_diagnostics (2 total):
    Error EmptyExtensions at input.extensions
    Warning EmptyMatchExpression at tracks[0].match
--- save::to_string (yaml):
    |profile_version: 1
    |input:
    |  pattern: ''
    |  extensions: []
    |tracks:
    |  rules:
    |  - match: {}
--- yaml round-trip equals model: true
########## S3 empty pattern + one extension + one empty rule
--- json in: {"profile_version":1,"input":{"pattern":"","extensions":["mkv"]},"tracks":{"rules":[{"match":{}}]}}
--- config_diagnostics (1 total):
    Warning EmptyMatchExpression at tracks[0].match
--- save::to_string (yaml):
    |profile_version: 1
    |input:
    |  pattern: ''
    |  extensions:
    |  - mkv
    |tracks:
    |  rules:
    |  - match: {}
--- yaml round-trip equals model: true
########## S4 .* pattern + one extension + one empty rule
--- json in: {"profile_version":1,"input":{"pattern":".*","extensions":["mkv"]},"tracks":{"rules":[{"match":{}}]}}
--- config_diagnostics (1 total):
    Warning EmptyMatchExpression at tracks[0].match
--- save::to_string (yaml):
    |profile_version: 1
    |input:
    |  pattern: .*
    |  extensions:
    |  - mkv
    |tracks:
    |  rules:
    |  - match: {}
--- yaml round-trip equals model: true
########## S5 passthrough: unmatched keep, no rules
--- json in: {"profile_version":1,"input":{"pattern":".*","extensions":["mkv"]},"tracks":{"unmatched":"keep","rules":[]}}
--- config_diagnostics (1 total):
    Info PassthroughProfile at tracks.rules
--- save::to_string (yaml):
    |profile_version: 1
    |input:
    |  pattern: .*
    |  extensions:
    |  - mkv
    |tracks:
    |  unmatched: keep
    |  rules: []
--- yaml round-trip equals model: true
```

### Verdict

Every one of the five reproduces the plan's authoring output on both instruments, exit codes
included. **S4 carries exactly one diagnostic, `empty-match-expression`, WARNING severity, at
`tracks[0].match`** -- no error-severity diagnostic, so Save is alive on first use. No
NEEDS_CONTEXT from this step; the seed is written as fenced.

## Step 2: `EditorView.vue`'s script, six edits

All six applied as prescribed.

1. **`blankProfile()`**, module level, placed immediately before `createBlank`. Doc comment
   states why it is a function (a fresh object per call, the immutable-rebuild discipline of
   every write in the view) and why `extensions` carries a value (forced by the validator: an
   empty list is `empty-extensions` at error severity, so a bare seed would ship a dead Save
   button), plus the `.*`-over-`""` reason from D107 decision 1.
2. **`sessionActive`**, a plain `ref(false)` beside `currentPath`. Doc comment states its duty
   (a profile entered through one of the view's own funnels, `openPath` or `createBlank`) and
   why it is neither `currentPath` (a created profile has no path) nor `model` (the bare
   mount-harness case must keep firing no IPC -- a plan-6 safeguard preserved, not removed).
   **No snapshot machinery and no `computed`**, per the Task 4 boundary.
3. **`saveDisabled`** drops `!currentPath.value`. Its existing doc comment names only the
   error-severity rule and stays true, so it is untouched.
4. **The `watch(model)` gate** is now `if (!sessionActive.value || !value)`, and the comment
   above it names `sessionActive` and both funnels while keeping its other two facts (why a
   shallow watch suffices, what `validationGeneration` is for).
5. **`openPath`** sets `sessionActive.value = true;` beside `currentPath.value = path;`.
6. **`createBlank`**, byte-identical to the fenced block, **synchronous** per the Task 5
   boundary. The comment above it states the load-bearing order: `diagnostics` cleared before
   the model is replaced; `sessionActive` set before `model` so the watcher that fires on that
   assignment validates the seed instead of returning early; index 0 selected so the detail
   panel opens on the field the warning names (D67).

## Step 3: `doSave` with the dialog branch

Body replaced with the fenced shape; import changed to
`import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";`. No
capability file touched (`dialog:allow-save` already granted).

**This is where the fork sat** -- see "The fork" above for the diagnosis, the options and the
ruling that closed it. As committed, the block carries exactly one token that differs from the
plan's fenced text (`if (path === null)` as the first guard), **on the authority of
`task-3-ruling-1.md`**. Everything else in the block is verbatim: the
capture-before-the-dialog-gap comment and its `RunHistory` citation, `saving` set before the
dialog, the cancelled-dialog early return, `currentPath` set on success, and the recents write
still gated on `needsPath`. Its behaviour is measured in Step 7's cases 4-6.

## Step 4: `EditorView.vue`'s template, five edits

- New button immediately **before** the Open button: `type="button"`,
  `data-testid="editor-new"`, `:disabled="opening || saving"`, `@click="createBlank"`, label
  `{{ $t("editor-action-new") }}`, no `title` attribute.
- `<p v-else-if="sessionActive" data-testid="editor-unsaved">` after the existing
  `<p v-if="currentPath">` path line.
- `<p v-if="!model" data-testid="editor-empty">`, after the path/unsaved lines and before the
  recents section.
- The recents gate moved from `!currentPath && recents.length` to `!model && recents.length`.
- The diagnostics `<section>` gained `v-if="diagnostics.length"`. Its heading, id and
  `DiagnosticsPanel` mount are otherwise untouched; `DiagnosticsPanel.vue` is **not** edited.

## Step 5: the two falsified comment regions

**Region 1, the Task-13 doc block's `currentPath` gate explanation.** The sentence now reads
that the watcher is gated on `sessionActive`, "which only this view's own two funnels set
(`openPath` on a completed load, `createBlank` on a fresh seed -- D107)". The block's other
content is preserved. One adjacent clause changed with it: "never click Open" became "never
click Open or New", because the sentence's job is to explain why a bare mount never reaches
the gate and there are now two funnels to exclude, not one. Flagged here because it is
adjacent to, rather than inside, the gate clause.

**Region 2, the stale count in the same block.** `gui-editor.ftl stays 45` is gone. The
sentence now records that Task 13 added no keys, that later packages did, and that the catalog
carries **49** ids today, naming the three this view's New affordance needs.

**Region 3, `e2e/smoke.spec.ts`'s budget comment.** `budget is 45 (42 labels + 1 save-surface
note + 2 generic action keys)` became `budget is 49 (42 labels + 1 save-surface note + 2
generic action keys + 1 rule-grid ordinal + 3 profile-creation keys)`. The rule-grid ordinal
term was missing from the decomposition in the file, as the dispatch stated; it is restored
here.

### The recount, re-run on the FINAL COMMITTED state

Not the number measured before the ruling's token landed: re-run against the committed blobs
themselves, plus the working tree, which `git status --porcelain` reports empty so the two are
the same content.

```
$ git show HEAD:locales/en/gui-editor.ftl | command grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*='
49
$ git show HEAD:locales/de/gui-editor.ftl | command grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*='
49
$ command grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl
49
$ command grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/de/gui-editor.ftl
49
```

49 in both locales, matching the expected 46 + 3 and the decomposition 42 + 1 + 2 + 1 + 3.

**The counting expression carries a fired control**, because a count is only as good as the
pattern producing it and a pattern that silently misses a member still returns a plausible
number. The same expression, run against the committed blobs and filtered to this task's three
ids, resolves all three in both locales:

```
$ git show HEAD:locales/en/gui-editor.ftl | command grep -E '^[A-Za-z][A-Za-z0-9_-]*\s*=' | command grep -E '^editor-(action-new|empty|unsaved) '
editor-action-new = New profile
editor-empty = No profile open. Create one with New profile, or choose an existing profile file.
editor-unsaved = New profile, not saved yet.
$ git show HEAD:locales/de/gui-editor.ftl | command grep -E '^[A-Za-z][A-Za-z0-9_-]*\s*=' | command grep -E '^editor-(action-new|empty|unsaved) '
editor-action-new = Neues Profil
editor-empty = Kein Profil geöffnet. Erstelle eines mit Neues Profil oder wähle eine vorhandene Profildatei aus.
editor-unsaved = Neues Profil, noch nicht gespeichert.
```

That paste also confirms the German umlauts survived into the commit intact.

The expression itself is derived from the artifact rather than composed from memory: it is
`check-i18n.mjs`'s own `MESSAGE_ID_RE` (column-0 `id =` lines only), which `e2e/i18n-en.ts`
states it mirrors exactly. `command grep` throughout rather than the shell's `grep` function,
which honours `.gitignore` and can return a false empty on a rooted sweep. No NEEDS_CONTEXT
from this step.

## Step 6: the catalogs, both locales

Appended at end of file in both, which is the only placement satisfying **both** clauses of
the step ("Append to ..." and "after its existing generic-action section"): the generic-action
section is no longer last (a rule-grid-ordinal section follows it, which is exactly why the
corrected decomposition in Step 5 has that term), so appending at EOF is an append AND is
after the generic-action section, while inserting immediately after that section would not be
an append. One placement satisfies both clauses, so this is a reading, not a fork.

Values copied exactly as fenced, umlauts included (`Öffnen`, `geöffnet`, `wähle`). Verified by
`pnpm check:i18n`:

```
$ pnpm check:i18n
check-i18n: ok (41 source files scanned, 216 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
```

## Step 7: the tests

A new describe, `editor view: New creates a blank profile (plan 12 W2, D107)`, placed after the
recents describe, using the file's existing `MKVMERGE_INFO`, `en`/`name`, `visibleText` and
`assertNoSeriousA11yViolations`.

**Fixtures.** `warnReport` carries the measured seed diagnostic (`empty-match-expression`,
warning, `tracks[0].match`); `cleanReport` is its empty counterpart; `PICKED_PATH =
"/profiles/created-by-new.yaml"` is distinct from every path literal in the file, verified by
`command grep -oE '"/[A-Za-z0-9./_-]+"' e2e/smoke.spec.ts | sort -u` before choosing it.

**Two fixtures the step does not enumerate, and why composing them is not a fork.** Case 6
("Open a profile, edit, Save") cannot run without a path to open and a `LoadProfileDocument`
to return, and the step's fixture list names neither -- so that list is demonstrably a floor
rather than a closed set. Composed: `ALREADY_PATHED_PATH = "/profiles/already-pathed.yaml"`
(a second literal distinct from `PICKED_PATH` and from every other path in the file, so "Save
wrote the OPENED path and never opened a dialog" cannot be satisfied by the dialog's own
return value) and `openedDoc`/`openedProfile`, a pathed profile with one non-empty rule,
deliberately not the seed.

**Corrected in fix round 1 (M-3): an earlier draft of this paragraph borrowed a qualifier
from the wrong table row.** It read "the model-tier ground for this task is 'seven new tests'
whose fixtures are composed (the same framing Task 2 ran under)". Task 3's tier row reads
"... and seven new tests"; "whose fixtures are composed" belongs to **Task 2's** row ("a new
test whose locale mechanism is prescribed but whose fixtures are composed"). The conclusion is
unaffected because it never needed that support: case 6's own prescribed text cannot execute
without an opened-profile fixture, which is by itself sufficient to show the fixture list is a
floor. The borrowed clause is removed rather than repaired.

**A count discrepancy, surfaced.** The model-tier table sizes this task at "seven new tests";
Step 7 enumerates **six** cases. I built the six enumerated ones -- the step is the normative
text, the table is sizing rationale. Same class as the four-versus-five seed count above.

### The six cases, and what each asserts

1. **New creates and validates.** One `editor-rule-row`; the pattern field holds `.*`;
   `editor-unsaved` visible; the open-path line absent; `editor-empty` gone; and the recorded
   `validate_profile_model` invocation carries the seed's `input.extensions` (`["mkv"]`) and
   one rule -- the wire half of the decoupling, since no path exists and this call could not
   have happened under the old `currentPath` gate.
2. **The seed is warned, not blocked.** With `warnReport`, the diagnostics panel renders the
   `empty-match-expression` line through the en catalog (`batch-diagnostic-line` composed from
   `severity-warning` and the diagnostic message, never a hand-typed literal), and
   `editor-save` is enabled.
3. **The pre-session empty state**, plus absence check E1 and its fire (below). Two
   `assertNoSeriousA11yViolations` scans, one per state.
4. **Save with no path.** The recorded `plugin:dialog|save` call; the recorded `save_profile`
   with `path === PICKED_PATH` and a profile whose `tracks.rules` has length 1; the
   `batch-profile-current` line for `PICKED_PATH`; a recorded `set_settings` whose
   `recent_profiles[0]` is `PICKED_PATH`.
5. **The cancelled dialog**, plus absence check E2 and its fire (below).
6. **An already-pathed save is unchanged.** Open, edit, Save: `save_profile` carries
   `OPENED_PATH` and the edited pattern; **no** `plugin:dialog|save` call at all, even though a
   different path was scripted for it and would have been written had the branch opened; and
   exactly one `set_settings` on record -- the OPEN's -- which guards "the recents memory is
   fed only when a path is newly established".

### Absence check E1: the diagnostics section does not render over nothing

- **Expression:** `section[aria-labelledby="editor-diagnostics-heading"]`, held in one
  `DIAGNOSTICS_SECTION` const so the zero-assertion and its fire cannot drift apart.
- **End state (zero):** in case 3, after navigating to the editor and before any funnel runs,
  the locator has count 0. Before this task the same state rendered a "Diagnostics" heading
  over an empty panel -- literally nothing, with no explanatory text, which is round-3 finding
  2's own contributing observation.
- **Fire (non-zero), in the same test:** clicking `editor-new` with `warnReport` mocked makes
  the same locator resolve to count **1**. So the zero is a measured absence, not a selector
  that matches nothing anywhere.

### Absence check E2: a cancelled save dialog writes nothing

- **Expression:** `recorded.filter((r) => r.cmd === "save_profile")`, expected length 0.
- **Fire, in the same test:** the recorded `plugin:dialog|save` call, polled to exactly 1
  before the absence is asserted -- so the flow demonstrably ran and reached the dialog, and
  the zero cannot mean "Save was never attempted". Reinforced by an end-state assertion that
  distinguishes cancel from success: `editor-unsaved` still visible and the open-path line
  still absent, which a successful write would have swapped. Case 4's non-zero `save_profile`
  counter is the cross-test fire for the same command name.

### A third absence, with its own derived term and fire

`PROFILE_LINE_PREFIX` (the "no open-path line" assertions in cases 1 and 5) is **derived from
the catalog at runtime** -- `visibleText(en("batch-profile-current", { path: "" })).trim()`,
Fluent's isolate marks stripped -- rather than typed from memory of what the message says, so
the absence cannot pass because the search term drifted from the catalog. Its fire is case 4,
where the same locator's count is asserted greater than zero once the profile acquires a path.

## Step 8: verification

**The full gate as `BUILDING.md` enumerates it, all 11 parts, foreground, no subsets, on the
state that was then committed.** Re-run after the ruling's token landed, so no figure here is
carried over from the earlier probe run. Every exit code captured with `$?` directly on the
command itself, never through a pipeline -- under zsh the array is `pipestatus` and 1-indexed,
so `${PIPESTATUS[0]}` reads blank and would have reported a false success:

```
1 cargo fmt --all --check exit=0
2 cargo clippy exit=0
3 cargo test --workspace exit=0
4 cargo doc exit=0
5 cargo deny check exit=0
6 cargo clippy windows exit=0
7 pnpm lint exit=0
8 pnpm build exit=0
9 pnpm check:i18n exit=0
10 pnpm test:e2e exit=0
11 ledger-lint exit=0
```

Six Rust, four frontend, one house-knowledge -- the split `BUILDING.md`'s own gate-total
sentence states, which `scripts/ledger-lint.py` cross-checks against the three marked gate
blocks:

```
$ python3 scripts/ledger-lint.py
ledger-lint: 568 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

Rust tests, summed from the run's own `test result:` lines rather than by hand (an earlier
hand-sum of the same log came out 505; the arithmetic is the machine's for that reason):

```
$ command grep -E "^test result: ok\." f3.log | sed -E 's/^test result: ok\. ([0-9]+) passed.*/\1/' | paste -sd+ | bc
507
$ command grep -cE "^test result: ok\." f3.log
39
```

507 Rust tests pass across 39 suites.

Frontend parts, output pasted from the runs above:

```
$ pnpm lint          -> exit 0, no output
$ pnpm build         -> vue-tsc clean; "✓ built in 154ms"
$ pnpm check:i18n    -> check-i18n: ok (41 source files scanned, 216 catalog ids, ...) [full line in Step 6]
$ pnpm test:e2e      -> "78 passed (3.0s)"
```

**Every pre-existing test passes unchanged.** In particular the `editor-save` assertions in
`e2e/editor-rule-add-remove.spec.ts` and the three in the Task-13 open/save describe are green
without modification, confirming the authoring measurement that removing `currentPath` from
`saveDisabled` cannot change them (all of them run after an Open). No pre-existing test's
behaviour changed, so no defect signal from that direction.

The six new tests, pasted from the same run:

```
  ✓  73 [chromium] › e2e/smoke.spec.ts:1714:3 › editor view: New creates a blank profile (plan 12 W2, D107) › New seeds the editor with the blank profile and validates it with no path in existence (225ms)
  ✓  74 [chromium] › e2e/smoke.spec.ts:1753:3 › editor view: New creates a blank profile (plan 12 W2, D107) › the seeded rule is warned, not blocked: the diagnostic renders and Save stays enabled (211ms)
  ✓  75 [chromium] › e2e/smoke.spec.ts:1779:3 › editor view: New creates a blank profile (plan 12 W2, D107) › the pre-session state names both entry paths, and the diagnostics section does not render over nothing (830ms)
  ✓  76 [chromium] › e2e/smoke.spec.ts:1808:3 › editor view: New creates a blank profile (plan 12 W2, D107) › Save on a pathless profile opens the save dialog, writes the picked path, and remembers it (233ms)
  ✓  77 [chromium] › e2e/smoke.spec.ts:1854:3 › editor view: New creates a blank profile (plan 12 W2, D107) › a cancelled save dialog writes nothing (239ms)
  ✓  78 [chromium] › e2e/smoke.spec.ts:1885:3 › editor view: New creates a blank profile (plan 12 W2, D107) › an already-pathed save is unchanged: no dialog, and the opened path is what gets written (212ms)
```

The commit covers exactly the four files in the Files list, no more:

```
$ git show --stat --format="" HEAD
 e2e/smoke.spec.ts         | 306 +++++++++++++++++++++++++++++++++++++++++++++-
 locales/de/gui-editor.ftl |   6 +
 locales/en/gui-editor.ftl |   6 +
 src/views/EditorView.vue  | 148 +++++++++++++++++++---
 4 files changed, 451 insertions(+), 15 deletions(-)
```

## Step 9: commit

```
$ git log -1 --format="%H%n%s%n---body---%n%b"
2cc065077ca3597e90d0f81c9cfc9a5ad7523bcd
editor: New creates a blank profile, and the path stops gating validation and Save
---body---
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git log -1 --format="%G?"
N
```

Subject verbatim from Step 9. Staged explicitly by path and committed pathspec-scoped
(`git commit ... -- <the four paths>`), never `git add -A`, because two writers in one working
tree share one index. Unsigned via `git -c commit.gpgsign=false`, confirmed by `%G?` reporting
`N`. **Exactly one trailer**, per SI-4 and the dispatch's literal string.

`git status --porcelain` is empty afterwards, so the committed state, the state the gate ran
against and the state the recount was taken from are all the same content.

**Not pushed.** The dispatch authorises the commit and withholds the push.

---

## Surfaced, and their dispositions

All four were raised before being acted on; all four are disposed of in
`task-3-ruling-1.md`, which is the authority for the first and the record for the rest.

1. **The fork itself** (section "The fork"): the fenced `doSave` block does not type-check.
   Returned as NEEDS_CONTEXT with options, costs and a recommendation; reproduced by the
   controller; **ruled option A**, one token plus the comment at the site. Routing rather than
   resolving was confirmed correct: the two colliding statements were both inside the plan, so
   the plan's own precedence settled it and nothing product-visible was at stake.
2. **"the four candidate seeds"** in Step 1 versus the five the authoring section enumerates
   and the five D107 reasons about. Ran all five. **Ruled correct handling** -- a superset
   cannot under-satisfy -- and the brief's count is the thing that is wrong against its own
   source. Recorded as a count-against-its-own-enumeration instance.
3. **"seven new tests"** in the model-tier table versus the six cases Step 7 enumerates. Built
   the six. **Ruled: the enumeration governs the tier table.** Same class as item 2.
4. **The catalog append position** was under-determined ("Append ... after its existing
   generic-action section", which is no longer the last section). Exactly one placement
   satisfies both clauses here, so reading it was correct rather than routing it. **Ruled a
   real forward problem:** Tasks 4 and 5 append to the same catalogs under the same wording
   with yet another section now in between, so their dispatches carry an explicit placement
   instead of inheriting the ambiguity. Carried as a cross-task constraint, not left to the
   next implementer.
5. **`editor-generic-action-keys` (Tier 2, `docs/product-boundaries.yaml`) now states a number
   this diff falsified.** Added in fix round 1; the reviewer found it missing (M-5) and was
   right. Its statement reads "The editor catalog budget is deliberately REVISED 43 -> 45 (42
   labels + 1 save note + 2 action keys) by the same ruling, and REVISED AGAIN 45 -> 46 by the
   plan-7 design (D59)", against a catalog that now carries **49**. This is a surfacing item,
   not an edit miss: the plan makes the Tier-2 statement update a controller close action and
   forbids a task from editing the house-knowledge YAML. Flagged now rather than at the plan
   close because **Tasks 4 and 5 raise the number twice more** (by two and by three), so the
   entry will be stale against three successive values if it waits.
6. **Nothing else ledger-worthy** was found. No house-knowledge YAML, `ROADMAP.md`
   or `docs/process-journal.md` was touched. `DiagnosticsPanel.vue` was not edited. No
   capability file was changed. No new pattern was introduced that deviates from the house
   ones; the save flow follows `RunHistory.saveLog`'s capture-before-the-gap discipline, the
   recents write reuses `rememberRecentProfile` unchanged, and the tests follow the file's own
   catalog-derived-assertion convention throughout.

---

# Fix round 1

**Status: DONE.** Commit **`6904e4a`**, two files, unsigned, one trailer, not pushed. Full
11-part gate green. Verdict: `task-3-verdict.md` (spec compliance APPROVED; task quality
APPROVED WITH REQUIRED FIXES, 0 Critical, 3 Important, 7 Minor).

Fixed: **I-1**, **I-3**, **M-1**, **M-6** (dispatched), plus **M-3**, **M-4**, **M-5**
(report- and comment-level accuracy defects the reviewer proved; each named below).
**Not touched: I-2**, routed to the owner. Both its gates still read exactly what the plan
fenced -- verified before committing:

```
$ command grep -n 'v-if="!model"' src/views/EditorView.vue
625:      v-if="!model"
$ command grep -n 'v-if="!model && recents.length"' src/views/EditorView.vue
632:      v-if="!model && recents.length"
```

**No executable line of `createBlank` changed in this round.** The function body is
byte-identical to the previous commit; only the comment above it moved:

```
$ diff <(git show HEAD~1:src/views/EditorView.vue | sed -n '/^function createBlank/,/^}/p') \
       <(sed -n '/^function createBlank/,/^}/p' src/views/EditorView.vue)
(no output)
```

## I-1: the load-bearing-order claim, re-measured rather than deleted

The reviewer's two measurements reproduce. Each mutation was applied, built and run, then
reverted with a rebuild before the next.

**Mutation A -- swap `sessionActive.value = true;` and `model.value = blankProfile();`**

```
A pnpm build exit=0
A pnpm test:e2e exit=0
  78 passed (3.0s)
```

**Mutation B -- move `diagnostics.value = [];` after the model assignment**

```
B pnpm build exit=0
B pnpm test:e2e exit=0
  78 passed (3.0s)
```

Both halves of the old comment are false. Cause, as the reviewer diagnosed: `watch(model, ...)`
runs at Vue's default `flush: "pre"`, so the callback is queued rather than run at the
assignment and observes every write in this synchronous body whatever order they were made in;
and no render happens between two synchronous ref writes, so the `diagnostics` clear cannot be
outrun by a paint either.

**But an ordering constraint DOES survive, and it is the one the next tasks can break.** The
brief asked me to pin it with a mutation if it existed. It does:

**Mutation C -- make `createBlank` async and put `await Promise.resolve();` between
`model.value = blankProfile();` and `sessionActive.value = true;`** (the shape Task 5
introduces when it makes this funnel async):

```
C pnpm build exit=0
C pnpm test:e2e exit=1
  3 failed
    [chromium] > ... > New seeds the editor with the blank profile and validates it with no path in existence
    [chromium] > ... > the seeded rule is warned, not blocked: the diagnostic renders and Save stays enabled
    [chromium] > ... > the pre-session state names both entry paths, and the diagnostics section does not render over nothing
  75 passed (7.5s)
```

So the true constraint is not the relative order of the writes but that **the gate and the
model are established in the SAME synchronous block**: an `await` after the model assignment
lets the queued watcher run at that microtask boundary, read a still-false `sessionActive` and
return early, and the seed is never validated.

The rewritten comment states exactly that: the order inside the body is not load-bearing and
is kept for readability; the same-synchronous-block property is; and it names the mutation
that shows it. It also says plainly that an earlier version of the comment claimed the
opposite, so a later reader does not reconstruct the false timing model from the code's shape.
The third clause (index 0 and the detail panel) was true and is kept, now with its producer.

**Plan-side residual, surfaced not edited:** the false claim originates in the plan's Step 2
prose ("The order is load-bearing and is commented as such"), and the reviewer notes the
correction is owed there too. Tasks 4 and 5 both position call sites inside this funnel, so
their dispatches inherit the same premise. No task may edit the plan document, so this is
routed rather than fixed.

## I-3: D107 decision 9 now has a producer

Added **case 7**, `New selects the seeded rule, so the detail panel opens on the field the
warning names`, as its own test rather than an extra assertion inside a prescribed case, so no
existing case's enumerated assertion list is touched.

It asserts more than panel presence: the panel labels itself by the selected row's own id
(`aria-labelledby="editor-rule-row-0"`) and the row's select button carries
`aria-current="true"`, so the seeded rule is pinned as the selected one from both ends rather
than "some rule got selected".

**Failing without the line (mutation D -- delete `selectedIndex.value = 0;`):**

```
D pnpm build exit=0
D pnpm test:e2e exit=1
  ✘  79 [chromium] › ... › New selects the seeded rule, so the detail panel opens on the field the warning names (5.2s)
    Error: expect(locator).toBeVisible() failed
    Locator: getByTestId('view-editor').getByTestId('editor-rule-detail')
    Expected: visible
    Timeout: 5000ms
    Error: element(s) not found
  1 failed
  78 passed (7.2s)
```

Exactly one test fails, and it is the new one -- which also re-confirms the reviewer's measured
starting point that nothing else in the repository observes that line.

**The four conditions of the owner-ruled precedence clause in
`tests-ship-with-the-feature-never-after`, named here so the reviewer can rule on the
addition:** the test is ADDITIVE (no existing assertion, fixture value or helper touched); it
runs on EXISTING infrastructure (the `editor-rule-detail` testid the T13b describe in the same
file already asserts, plus this describe's own `gotoEditor` helper and `warnReport` fixture);
the consequence is one THIS diff creates (`selectedIndex.value = 0` is a line this task added);
and it is named in this report. Outside those four the plan's enumeration would still bind and
the fork would return.

## M-1: the save dialog's own arguments are now read

Case 4 asserted only that `plugin:dialog|save` happened. It now reads the arguments the call
carries -- both fixed by D107 decision 5 -- with the filter name taken from the en catalog
rather than duplicated as a literal:

```ts
expect(dialogOptions.defaultPath).toBe("profile.yaml");
expect(dialogOptions.filters).toEqual([
  { name: en("batch-profile-filter-name"), extensions: ["yaml", "yml"] },
]);
```

**Two assertions, so two fires.** A single mutation would have covered only one of them, and a
fire against one member of a pair says nothing about the other:

**Mutation E -- `defaultPath: "profile.yaml"` -> `"not-a-profile.txt"`** (the reviewer measured
this exact mutation leaving the suite green at `2cc0650`):

```
E pnpm test:e2e exit=1
  ✘  76 [chromium] › ... › Save on a pathless profile opens the save dialog, writes the picked path, and remembers it
    Error: expect(received).toBe(expected) // Object.is equality
    Expected: "profile.yaml"
    Received: "not-a-profile.txt"
  1 failed / 78 passed
```

**Mutation F -- the save filter's `extensions: ["yaml", "yml"]` -> `["yaml"]`:**

```
F pnpm test:e2e exit=1
  ✘  76 [chromium] › ... › Save on a pathless profile opens the save dialog, writes the picked path, and remembers it
  1 failed / 78 passed
```

The argument shape was measured, not assumed: `@tauri-apps/plugin-dialog`'s `save()` invokes
`plugin:dialog|save` with `{ options }`, so the recorded args carry the options object one
level down.

## M-6: the duplicated fixture name

This describe's `OPENED_PATH` is renamed to **`ALREADY_PATHED_PATH`** (value unchanged,
`/profiles/already-pathed.yaml`). The recents describe's own `OPENED_PATH` is left alone: the
collision was in the name this task introduced, and renaming a pre-existing fixture would be
gratuitous churn in a describe this task does not own. The new name says what the fixture is
for. The only remaining occurrence of the old name inside this describe is the comment
explaining why the rename exists.

## Also fixed, from the Minor list, because each was a proved accuracy defect

- **M-3, a misquote in this report.** The passage in Step 7 attributed "whose fixtures are
  composed" to Task 3's tier row; it belongs to Task 2's. Corrected in place, with the
  correction shown rather than silently overwritten, and the borrowed clause removed rather
  than repaired -- the conclusion never needed it, since case 6's own prescribed text cannot
  execute without an opened-profile fixture.
- **M-4, the reflow artifact my own rewrite left** in the Task-13 doc block: the line ending
  `` `batch-profile-pick`/`batch-profile- `` ran half the width of its neighbours and broke the
  identifier mid-word. The paragraph is reflowed; the identifier is now whole on one line.
- **M-5, a missing item in the surfacing list.** `editor-generic-action-keys` (Tier 2,
  `docs/product-boundaries.yaml`) states a catalog budget of 46 against a catalog that now
  carries 49. Added as item 5 of the surfacing list above, with the reason it is flagged now
  rather than at the plan close: Tasks 4 and 5 raise the number twice more.

## Deliberately not fixed

- **I-2** -- routed to the owner. Neither gate touched (verified above).
- **M-2** (the recents-gate change has no producer in this task) -- the reviewer records it as
  a carry-forward, the acceptance map assigns W4-f to Task 5, and I-2's routing may change what
  that gate should even be. Building a producer now risks pinning a gate the owner is about to
  rule on.
- **M-7** (`createBlank`'s `if (opening.value || saving.value)` re-guard is untested) -- the
  reviewer records it as consistency with `pickAndOpen`'s existing untested guard rather than a
  new gap. Not additive-and-uncontested in the way I-3 was, and the button's own `:disabled`
  binding is the covered half.

## Gate, on the committed state

Full 11 parts as `BUILDING.md` enumerates them, foreground, no subsets, exit codes captured
with `$?` directly on each command:

```
1 cargo fmt --all --check exit=0
2 cargo clippy exit=0
3 cargo test --workspace exit=0
4 cargo doc exit=0
5 cargo deny check exit=0
6 cargo clippy windows exit=0
7 pnpm lint exit=0
8 pnpm build exit=0
9 pnpm check:i18n exit=0
10 pnpm test:e2e exit=0
11 ledger-lint exit=0
```

```
507 Rust tests across 39 suites (summed from the run's own `test result:` lines)
79 passed (3.1s)                                  [was 78; case 7 is the addition]
check-i18n: ok (41 source files scanned, 216 catalog ids, ...)
ledger-lint: 568 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

**Every revert in this round was followed by `pnpm build` before anything was believed**, per
the reviewer's harvest that `pnpm test:e2e` does not rebuild `dist/` and would otherwise report
the previous tree's bundle against a clean `git status`. That discipline caught a real error
here: `git checkout -- src/views/EditorView.vue` after mutation F also discarded this round's
own I-1 and M-4 edits in the same file. `git status` showed only `e2e/smoke.spec.ts` modified,
which is what exposed it; both edits were re-applied and re-verified by content before the
gate ran. **Handle worth carrying: a revert-by-file is only safe while that file carries
nothing but the mutation** -- once a fix round edits the same file it is mutating, the revert
needs to be a targeted inverse edit, or the fix has to be re-applied and re-checked afterwards.

## Commit

```
$ git log -1 --format="%H%n%s%n---body---%n%b"
6904e4a0e30eb30c56a22350fa3997f4c505fedd
editor: correct createBlank's ordering comment to what is measured, and cover the seeded selection
---body---
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git log -1 --format="%G?"
N
```

Staged explicitly by path, committed pathspec-scoped, never `git add -A`, unsigned, exactly
one trailer, not pushed. `git status --porcelain` empty afterwards.

---

# Fix round 3 (comment-only): the createBlank ordering constraint, measured

**Scope:** one sentence of one doc comment. No executable line changed; verified by
filtering the diff for any changed line that is not a comment (the filter was fired
against a known code line first, so its empty result is evidence rather than an
untested pattern):

```
$ git diff -U0 src/views/EditorView.vue | grep -E "^[+-]" | grep -vE "^(\+\+\+|---)" \
    | grep -vE "^[+-]\s*//"
(no output)

$ printf '+  sessionActive.value = true;\n' | grep -E "^[+-]" | grep -vE "^(\+\+\+|---)" \
    | grep -vE "^[+-]\s*//"
+  sessionActive.value = true;          <- the filter fires
```

## Why the comment was wrong a second time

Version 1 claimed the statement order inside the body was load-bearing. A reviewer
measured it and it was false: swapping `sessionActive`/`model` and moving the
`diagnostics` clear each left the suite green.

Version 2 corrected that but over-corrected: it declared the order inconsequential and
pinned a *synchronicity* constraint instead ("the gate and the model are established in
the SAME synchronous block"), on the strength of a single measurement -- reversed order
plus an `await` between the two fails three cases. That measurement is real. The
constraint inferred from it is not, because the counterfactual was never run: the
failure is not caused by the `await`, it is caused by the reversal, which the `await`
merely makes observable.

This is a live hazard rather than a cosmetic one. **Task 5 (D109, the discard guards)
makes `createBlank` `async` and puts an `await` in front of the seed** -- verified in the
plan, not assumed: *"Because `createBlank` is synchronous today, it becomes `async` and
its click handler awaits it; nothing else about it changes."* An implementer reading
"nothing depends on the order" is licensed to reorder, and reorder-plus-`await` is exactly
the failing configuration. The comment was disarming the guard it should have been.

## The three configurations, re-measured in this round

Every run: `pnpm build` first, then `pnpm test:e2e`, because `test:e2e` does not rebuild
`dist/` and would otherwise report the previous tree's bundle. Exit codes captured with
`$?` directly per command, never through a pipeline.

**Baseline, unmutated tree at `8ce50e4`** (the control the two green configurations are
read against -- "79 passed" says nothing without it):

```
BUILD exit=0
E2E   exit=0
  79 passed (3.0s)
```

**Configuration A -- shipped order (gate first), `await` between gate and model:**

```js
  sessionActive.value = true;
  await Promise.resolve();
  model.value = blankProfile();
```

```
A BUILD exit=0
A E2E   exit=0
  79 passed (3.0s)
```

**Configuration B -- shipped order, `await` after both:**

```js
  sessionActive.value = true;
  model.value = blankProfile();
  await Promise.resolve();
```

```
B BUILD exit=0
B E2E   exit=0
  79 passed (3.0s)
```

**Configuration C -- reversed order (model first), `await` between:**

```js
  model.value = blankProfile();
  await Promise.resolve();
  sessionActive.value = true;
```

```
C BUILD exit=0
C E2E   exit=1
  ✘  73 [chromium] › e2e/smoke.spec.ts:1716:3 › editor view: New creates a blank profile (plan 12 W2, D107) › New seeds the editor with the blank profile and validates it with no path in existence (5.1s)
  ✘  74 [chromium] › e2e/smoke.spec.ts:1755:3 › editor view: New creates a blank profile (plan 12 W2, D107) › the seeded rule is warned, not blocked: the diagnostic renders and Save stays enabled (5.2s)
  ✘  75 [chromium] › e2e/smoke.spec.ts:1781:3 › editor view: New creates a blank profile (plan 12 W2, D107) › the pre-session state names both entry paths, and the diagnostics section does not render over nothing (5.5s)
    Error: expect(locator).toBeVisible() failed
    Error: expect(locator).toHaveCount(expected) failed
  3 failed
  76 passed (7.6s)
[ELIFECYCLE] Command failed with exit code 1.
```

All three reproduce the reviewer's numbers exactly. Each mutation required making the
function `async`/`Promise<void>`; that alone is not what breaks C, since A and B carry
the same signature change and stay green.

**Conclusion.** There is no synchronicity requirement in the shipped order. What is
load-bearing is the relative order: with `sessionActive` set before `model`, the gate is
already true at the moment the `model` write queues the `flush: "pre"` watcher callback,
so wherever that callback later flushes it cannot observe a false gate -- the function is
await-proof regardless of where an `await` lands. The reverse order is what breaks once
an `await` separates the two: the watcher runs at that microtask boundary, reads a
still-false gate, returns early, and the seed is never validated.

## Restoration

The previous round destroyed its own fix edits with `git checkout -- src/views/EditorView.vue`
after a mutation. This round mutated the same file it was about to edit, so no
revert-by-file was used at any point: each configuration was reached by a **targeted
inverse edit** of the previous one, and the restoration to the shipped body was likewise a
targeted inverse edit, verified by content before anything was believed:

```
$ git status --porcelain
(empty)
$ git diff
(empty)
```

Then rebuilt, because a clean source tree does not imply a clean bundle:

```
RESTORE BUILD exit=0
RESTORE E2E   exit=0
  79 passed (3.0s)
```

Only after that green restoration was the comment edit applied. The comment edit is
therefore the *only* delta in the committed diff, which `git show --stat` confirms.

## What the comment now says

Retained from version 2 because both are correct and load-bearing for the reader: the
`flush: "pre"` mechanism (why order does not matter while the body is synchronous), and
the explicit note that earlier versions claimed otherwise -- now naming **both** wrong
versions and their opposite directions, so neither false model can be reconstructed from
the code's shape. Replaced: the synchronicity claim, by the ordering claim, with the
consequence for the imminent async version stated in the same breath.

The corrected claim, verbatim:

```
// What IS load-bearing is the RELATIVE ORDER of those two, and it becomes
// load-bearing exactly when the synchronicity above stops holding:
// `sessionActive` is established BEFORE `model`. Measured, all three
// configurations -- gate first with an `await` between gate and model: 79
// passed. Gate first with the `await` after both: 79 passed. Model first
// with an `await` between them: 3 failed (the first three cases of the New
// describe in `e2e/smoke.spec.ts`). Setting the gate first makes this
// function await-proof: at the moment `model` is written the gate is
// already true, so the callback that write queues cannot observe anything
// else, wherever it later flushes. The reverse order breaks as soon as an
// `await` lands between the two -- the watcher runs at that microtask
// boundary, reads a still-false gate, returns early, and the seed is never
// validated.
//
// Which is the case this funnel is walking into: the discard guard (D109)
// makes it `async` and puts an `await` in front of the seed. Keep the gate
// above the model assignment there. This comment has been wrong about this
// function twice, in opposite directions -- first that every statement's
// order was load-bearing, then that none of it was and that the constraint
// was gate and model sharing one synchronous block -- so do not reconstruct
// either claim from the code's shape.
```

Located by symbol (`createBlank`), never by line number. Existing shape and voice kept:
same `--` dashes, same CAPS emphasis, same `Measured:` evidence prefix, and no comment
line exceeds the file's existing 78-character maximum (checked, with the check fired at a
lower threshold first to prove it reads the comment lines at all).

## Gate, on the committed state

Full 11 parts as `BUILDING.md` enumerates them, foreground, no subsets, exit codes
captured with `$?` directly on each command:

```
1  cargo fmt --all --check         exit=0
2  cargo clippy                    exit=0
3  cargo test --workspace          exit=0
4  cargo doc                       exit=0
5  cargo deny check                exit=0
6  cargo clippy windows            exit=0
7  pnpm lint                       exit=0
8  pnpm build                      exit=0
9  pnpm check:i18n                 exit=0
10 pnpm test:e2e                   exit=0
11 ledger-lint                     exit=0
```

```
507 Rust tests across 39 suites (summed from the run's own `test result:` lines)
79 passed (3.0s)
check-i18n: ok (41 source files scanned, 216 catalog ids, ...)
ledger-lint: 570 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

## Commit

```
$ git log -1 --format="%H%n%s%n---body---%n%b"
6ca7685f0ff82464298aa0d8c0d7256ca466adff
editor: the createBlank comment pins the ordering constraint that is actually load-bearing
---body---
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git log -1 --format="%G?"
N
```

Staged explicitly by path, committed pathspec-scoped, never `git add -A`, unsigned,
exactly one trailer, not pushed. `git status --porcelain` empty afterwards; the commit
touches `src/views/EditorView.vue` alone.
