# Plan 12 pre-execution re-validation report

Recon only. No product artifact was authored, no plan/spec/source/catalog/test/document
was edited, nothing was committed. This file is the whole deliverable.

---

## 0. Headline

**No blockers.** Every Class A member is present exactly once in its named target file
with its stated anchor intact. Every Class B figure that rests on the source tree
reproduces exactly. **Three Class B figures MOVED**, all three in the documentation
layer and none of them reached by a task step:

- **B-27, decision numbering** — the plan's "highest D-number in use is **D105**, and
  `\bD10[6-9]\b` returns nothing" is falsified at HEAD: Plan 11 landed **D111**, and
  `D106`/`D109`/`D110` now appear as *references to Plan 12's own reservation*. No
  competing allocation: D106-D110 remain unassigned.
- **B-29, correction 7 (the README's first example)** — at the authoring baseline it
  failed to load (exit 2, `missing field 'pattern'`); at HEAD it validates clean
  (exit 0). Plan 11's fix round, which the plan names as its vehicle, has discharged.
- **B-30, the ROADMAP "OWNER QA PASS, round 3" entry** — it did **not exist in any
  commit** at the stated baseline `148f19f`; it lived only in the dirty working tree
  the plan's own authoring note records (` M docs/ROADMAP.md`). It is now committed
  (`9ad9e05`), so every citation of it resolves against a committed corpus for the
  first time. This is a strengthening, not a regression.

The structural reason the rest is clean is itself measured (§5, B-38): **the delta's
intersection with the entire Plan-12 target surface is two files, and neither is a
Plan-12 target.**

---

## 1. HEAD measured at, and the delta

```
$ git rev-parse HEAD
bd3aa34179ab689c7b681df90b4d01dde823da37

$ git rev-parse --abbrev-ref HEAD
master

$ git status --porcelain
(empty - clean working tree)
```

`git diff --name-only 148f19f..HEAD` returns **72 files**. Grouped, because the grouping
is what the verification leans on:

| group | files |
|---|---|
| Rust source | `crates/muxsmith-core/src/matcher.rs`, `crates/muxsmith-core/src/report/mod.rs`, `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` |
| build / CI / deps | `.github/workflows/ci.yml`, `BUILDING.md`, `deny.toml`, `pnpm-lock.yaml` |
| user-facing docs | `README.md`, `help/en/editor-match-expr-exact.md`, `help/de/editor-match-expr-exact.md` |
| spec | `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md` (new) |
| plans | `docs/superpowers/plans/2026-07-30-plan-11-...md`, `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` |
| process layer | `docs/ROADMAP.md`, `docs/decision-ledger.yaml`, `docs/process-conventions.yaml`, `docs/process-journal.md`, and 54 files under `docs/process-journal/artifacts/`  (3+4+3+2+2+4+54 = 72) |

**Not in the delta, and this is load-bearing:** `locales/`, `src/`, `e2e/`, `scripts/`,
`src-tauri/`, `crates/muxsmith-cli/`, `help/*/view-editor.md`.

The plan document itself changed in the delta (four commits: `e5cb799`, `0325923`,
`9bc06e6`, `d5b42c6`) — its own authoring rounds. The version measured here is the one
at HEAD, which is the one an implementer will read.

---

## 2. Derivation method

### 2.1 Class A

**E-A1, the replace-verb expression.**

```
$ grep -nEi 'replace|replacement|replacing' docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md
```

35 hits; seven carry the imperative form `Replace exactly` / `replace exactly`
(plan lines 588, 606, 690, 705, 800, 1200, 1212). Six of the seven fence an OLD region;
the seventh (line 800, `doSave`'s body) supplies only a NEW shape with no OLD text, so
it has no locatable string and is not a Class A member.

**E-A1's blind spot, stated:** it keys on the verb. It cannot see (a) an *anchor* quoted
inline rather than fenced, (b) an append-position anchor ("after its existing
generic-action section"), or (c) a current sentence a task must *amend* rather than
*replace* — which is exactly where this plan puts its comment repairs and its
region-extension instructions.

**E-A2, aimed at that blind spot**, over the task sections only:

```
$ awk 'NR>=574' <plan> | grep -nE '(existing|today (says|names)|stale|falsif|corrected to|rewritten|reworded|in the same position|after its existing|after the .* block|beside)'
```

Its yield supplied A7-A30: the two stale `45` comments, the de-catalog header note, the
named Rust test region, the `RUST_ONLY_IDS` allowlist, the existing e2e case title and
locale assertions, the `currentPath` code fragments, the help-topic headings, and the
kept catalog keys.

**How the characters were obtained.** Every needle is **extracted programmatically from
the plan file** by line range (fenced blocks) or by a regex over the plan's own line
(inline literals), then counted with `str.count` against the target file's bytes. No
pattern was retyped. The harness is
`<scratchpad>/classa.py`; each needle is printed verbatim in §3.2 as
extracted.

**Line breaks.** A fenced OLD block spanning several lines is searched as one exact
multi-line substring, joined with `\n` exactly as it sits in the plan (A3, A4). Three
inline-quoted members (A7, A8, A9) quote text that the target file *wraps across a
comment continuation* (`\n// `, `\n### `); those were re-run under an explicitly stated
normalization (join `\n<indent><comment-marker>` to one space, strip backticks, collapse
whitespace, and for A8 additionally case-fold) and each then resolves to exactly one
occurrence. Both the strict and the normalized figure are reported.

### 2.2 Class B

**E-B1, the expression form:**

```
$ grep -nE 'grep |wc -l|git ls-files|--json|--list-types|\$ ' <plan>
```

22 hits, covering every figure the plan states together with the command that produced it.

**E-B1's blind spot:** a measured value stated *without* its expression on the same line —
byte sizes, tool versions, pasted probe transcripts, symbol enumerations, the catalog
budget figure, the identical-en/de-value counts.

**E-B2, aimed there:**

```
$ grep -nE '\*\*[0-9]+\*\*|[0-9]+ bytes|exit=?[0-9]|Measured|measured rather than|returns (nothing|exactly|its|7|0)|v[0-9]+\.[0-9]+' <plan> | grep -vE 'grep |wc -l|git ls-files'
```

It returns the members E-B1 cannot reach: the seed transcripts (plan lines 129-137), the
`currentPath` duty finding (198), the E1/E2 controls (229), the Playwright probe (238),
the snapshot byte sizes (272), the seed measurement in the corrections table (284), the
depth-limit memory statement (348), the catalog budget (434), and the identical-value
figures (722).

The union of E-B1 and E-B2 is the Class B set below. It reaches outside the
`Authoring-time verification` section by construction — members B-33 through B-40 come
from task steps, the acceptance map and the self-review section.

---

## 3. Class A

### 3.1 Table

Occurrence counts are over the **whole target file**, at HEAD.

| # | member | target file | occ | verdict |
|---|---|---|---:|---|
| A1 | fenced OLD, spec 8.2 editor item (Task 1 Step 1) | `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` | **1** | unchanged |
| A1-anchor | quoted append anchor, the item's existing final sentence | same | **1** | unchanged — same line 382, inside `### 8.2 GUI`, and it *is* the item's final sentence |
| A1-NEW | the fenced NEW text (control: must be absent) | same | **0** | unchanged (replacement not pre-applied) |
| A2 | fenced OLD, spec 8.2 app-settings paragraph (Task 1 Step 2) | same | **1** | unchanged — line 386, inside `### 8.2 GUI` |
| A2-NEW | the fenced NEW text (control) | same | **0** | unchanged |
| A3 | fenced OLD, 2 lines (Task 2 Step 4) | `locales/en/gui-settings.ftl` | **1** | unchanged |
| A4 | fenced OLD, 2 lines (Task 2 Step 4) | `locales/de/gui-settings.ftl` | **1** | unchanged |
| A5 | fenced OLD, `batch-profile-none` (Task 7 Step 3) | `locales/en/gui-batch.ftl` | **1** | unchanged |
| A6 | fenced OLD, `batch-profile-none` (Task 7 Step 3) | `locales/de/gui-batch.ftl` | **1** | unchanged |
| A7 | quoted stale comment `gui-editor.ftl stays 45` (Task 3 Step 5) | `src/views/EditorView.vue` | 0 strict / **1** normalized | unchanged — see §3.3 |
| A8 | quoted stale comment `catalog budget is 45` (Task 3 Step 5) | `e2e/smoke.spec.ts` | 0 strict / **1** normalized | unchanged — see §3.3 |
| A9 | quoted de-catalog header note (Task 6 Step 3 consumes it) | `locales/de/gui-common.ftl` | 0 strict / **1** normalized | unchanged — see §3.3 |
| A10 | named Rust test to extend, `close_abort_strings_resolve_from_the_ftl_catalog` | `src-tauri/src/run.rs` | **1** | unchanged (definition at line 1322) |
| A11 | its pinned en value `Abort running jobs` | `src-tauri/src/run.rs` | **1** | unchanged |
| A12 | existing e2e case title Task 5 case 6 extends | `e2e/smoke.spec.ts` | **1** | unchanged (line 1388) |
| A13 | existing locale assertion `toHaveValue("en")` | `e2e/smoke.spec.ts` | **1** | unchanged (line 826, inside the `german locale` describe, 732-850) |
| A14 | existing locale assertion `toHaveValue("de")` | `e2e/smoke.spec.ts` | **1** | unchanged (line 848, same describe) |
| A15 | allowlist region `RUST_ONLY_IDS` | `scripts/check-i18n.mjs` | 2 | unchanged — definition (117) + use (323); a symbol, not a replace target |
| A16 | existing en catalog const `GUI_COMMON_FTL` | `src-tauri/src/run.rs` | 3 | unchanged — declaration (519) + uses |
| A17 | existing cancel key `close-abort-dismiss` | `src-tauri/src/run.rs` | 2 | unchanged |
| A18 | the neighbouring closure call the plan fences as the pattern | `src-tauri/src/run.rs` | **1** | unchanged |
| A19 | `!currentPath.value`, the term `saveDisabled` drops | `src/views/EditorView.vue` | 3 | unchanged — and 3 is *correct*: lines 185/202/267, exactly duties (a), (b) and (c) of D107 decision 3, each named by its enclosing symbol |
| A20 | recents gate old form `!currentPath && recents.length` | `src/views/EditorView.vue` | **1** | unchanged |
| A21 | path-line anchor `<p v-if="currentPath">` | `src/views/EditorView.vue` | **1** | unchanged |
| A22 | existing dialog import `open as openDialog` | `src/views/EditorView.vue` | **1** | unchanged |
| A23 | `settings-cancel` probed against `gui-editor.ftl` | `locales/en/gui-editor.ftl` | 0 | **probe error, mine** — the key's home is `gui-settings.ftl`; corrected as A23b. Not a finding about the tree. |
| A23b | `settings-cancel`, correct home | `locales/en/gui-settings.ftl` | **1** | unchanged |
| A24 | reused filter key `batch-profile-filter-name` | `locales/en/gui-batch.ftl` | **1** | unchanged |
| A25 | help anchor `## Editing the model` | `help/en/view-editor.md` | **1** | unchanged |
| A26 | help anchor `## Save semantics` | `help/en/view-editor.md` | **1** | unchanged |
| A27 | kept option key `settings-locale-option-en` | `locales/en/gui-settings.ftl` | **1** | unchanged |
| A28 | same, de | `locales/de/gui-settings.ftl` | **1** | unchanged |
| A29 | `close-discard-title` (Task 6 red state; must be ABSENT pre-build) | `locales/de/gui-common.ftl` | **0** | unchanged — 0 is the correct pre-state; fired (§3.4) |
| A30 | falsified header-doc sentence fragment | `e2e/editor-rule-add-remove.spec.ts` | **1** | unchanged (line 8) |

**34 rows, counted from the table above: 31 Class A members, plus 2 absence controls
(A1-NEW, A2-NEW) and 1 corrected probe error of mine (A23, superseded by A23b).
0 blockers.**

### 3.2 The needles as extracted (verbatim from the plan document)

```
A1        'detail editor per rule, panels for attachments/chapters/tags/title, open/save YAML, recent profiles.'
A1-anchor 'Inline validation markers from core diagnostics.'
A1-NEW    'detail editor per rule, panels for attachments/chapters/tags/title, create/open/save YAML, recent profiles.'
A2        'App settings (not profile data): mkvmerge path override, default parallelism. Stored in the platform config directory.'
A3        'settings-locale-label = Language\n    .hint = Which language the Muxsmith interface uses.'
A4        'settings-locale-label = Sprache\n    .hint = In welcher Sprache Muxsmith seine Oberfläche anzeigt.'
A5        'batch-profile-none = No profile selected yet. Choose one below to validate it and start a batch.'
A6        'batch-profile-none = Noch kein Profil ausgewählt. Wähle unten eines aus, um es zu prüfen und einen Stapel zu starten.'
A7        'gui-editor.ftl stays 45'
A8        'catalog budget is 45'
A9        'are not yet shown to a de user; kept single-line and translated for parity and a later shell i18n'
A10       'close_abort_strings_resolve_from_the_ftl_catalog'
A11       'Abort running jobs'
A12       'the editor tab stays mounted across a switch to Jobs and back'
A13       'toHaveValue("en")'
A14       'toHaveValue("de")'
A15       'RUST_ONLY_IDS'
A16       'GUI_COMMON_FTL'
A17       'close-abort-dismiss'
A18       'abort_and_quit(&app.state::<AppState>(), |code| app.exit(code))'
A19       '!currentPath.value'
A20       '!currentPath && recents.length'
A21       '<p v-if="currentPath">'
A22       'open as openDialog'
A23/A23b  'settings-cancel'
A24       'batch-profile-filter-name'
A25       '## Editing the model'
A26       '## Save semantics'
A27/A28   'settings-locale-option-en'
A29       'close-discard-title'
A30       'which a bare mount never sets'
```

### 3.3 The three wrapped quotes, resolved

All three exist; the plan's quote is a *reading* of text the file wraps, not a byte-exact
fence, so the strict count of 0 is an artifact of the line break and not an absence.

```
A7  src/views/EditorView.vue:65
      // carries none (`gui-editor.ftl` stays 45, the brief's own Files list
    normalized occurrences: 1   (plan's quote lacks the source's backticks)

A8  e2e/smoke.spec.ts:867-868
      // ... the AttachmentRule fields they are the registry labels for. Catalog
      // budget is 45 (42 labels + 1 save-surface note + 2 generic action keys).
    normalized occurrences: 1   (wraps mid-phrase; source capitalizes "Catalog")

A9  locales/de/gui-common.ftl:7-8
      ### close-abort-* strings below are not yet shown to a de user; kept
      ### single-line and translated for parity and a later shell i18n. The
    normalized occurrences: 1
```

Uniqueness probes (strict, on the unwrapped fragment):

```
$ grep -rn "stays 45" src/
src/views/EditorView.vue:65:// carries none (`gui-editor.ftl` stays 45, the brief's own Files list

$ grep -rniE "budget is 45" e2e/
e2e/smoke.spec.ts:868:// budget is 45 (42 labels + 1 save-surface note + 2 generic action keys).
```

Each is the only occurrence in its file and in its directory.

### 3.4 Absence-shaped Class A checks, with their fires

**A1-NEW / A2-NEW (the fenced NEW spec text must not already be present).** Both 0. The
fire is the paired OLD member measured through the identical code path against the same
file, returning 1 — so the harness demonstrably finds a present string in that file.

**A29 and the whole new-key set.**

```
$ grep -rn "close-discard\|close-abort-discard\|editor-discard\|editor-action-new\|editor-action-undo\|editor-action-redo\|editor-empty\|editor-unsaved\|settings-locale-option-system" locales/ src/ e2e/ src-tauri/ scripts/ help/ | wc -l
0

# FIRE, same shape against a known-present key:
$ grep -rn "close-abort-title" locales/ src-tauri/ scripts/ | wc -l
6
```

**Enumeration note (the fire test does not cover set membership).** The pattern above
carries an enumerated set of nine key stems. That set was **derived from the plan's own
catalog-budget tables**, not from recall: the two tables were parsed programmatically
(§4, B-37) and yield exactly 8 editor ids + 1 settings id + 6 common ids = 15 new ids,
whose nine distinct stems are the alternation used. Membership is therefore current
against the plan at HEAD.

**Create-targets absent, as the plan requires:**

```
$ ls src/components/ConfirmDialog.vue
ls: cannot access 'src/components/ConfirmDialog.vue': No such file or directory
$ ls e2e/editor-undo-redo.spec.ts
ls: cannot access 'e2e/editor-undo-redo.spec.ts': No such file or directory
$ ls docs/superpowers/specs/2026-07-30-plan-12-decisions.md
ls: cannot access '...': No such file or directory
```

### 3.5 Anchor observations (measurements, not judgements)

- **The spec's 8.2 region is untouched by the delta.** The spec changed in the delta
  (14 insertions, 9 deletions), but its hunks land at lines 146, 176, 280, 361-372 and
  426 — i.e. **§4.3, §4.4, §5.2, §8.1 and §9 item 2**. None is inside §8.2 (line 378 to
  390). So the "different sections, therefore no conflict" premise is now measured
  rather than expected. Sidenote for the record: the brief states Plan 11 amended
  "4.3, 4.4, 7, 8.1 and 9.2"; measured, there is **no §7 edit** and there **is** a §5.2
  edit (the `RawOnKnownProperty` diagnostics-table row). The conclusion is unaffected.
- **The gui-editor append anchor is not the file tail.** Task 3 Step 6 says "Append to
  `locales/en/gui-editor.ftl`, after its existing generic-action section". That section
  header exists exactly once per locale file (`## Generic list/map actions`, en line 136 /
  de line 140) — but it is followed by a further section, `## Rule grid ordinal (D59)`,
  which is the file's last. So "append after the generic-action section" and "append at
  the file tail" are two different placements here. Recorded as a measurement; routing is
  the controller's.
- **The German help topic uses translated headings.** Task 7 Step 2 asks for "the same
  section structure". Measured: `help/en/view-editor.md` carries `## Editing the model`,
  `## Validate on edit`, `## Save semantics`; `help/de/view-editor.md` carries
  `## Das Modell bearbeiten`, `## Prüfung bei jeder Änderung`, `## Speicherverhalten`.
  Structure matches; the heading *strings* are localized, so an implementer looking for
  the English anchors in the de file will not find them.

---

## 4. Class B

Verdicts: `reproduces` = current value equals the authoring value.
`MOVED` = it does not.

| # | figure | expression as run | authoring value | current value | verdict |
|---|---|---|---|---|---|
| B-1 | validator on the five candidate seeds, CLI path | `./target/debug/muxsmith validate <seed> --json` per seed | S1 2 errors exit 2; S2 error+warning exit 2; S3 warning exit 1; S4 warning exit 1; S5 info exit 0 | identical (transcript §4.1) | **reproduces** |
| B-2 | the model-path probe (throwaway crate outside the repo) | not re-run | see plan | — | **not re-run, deliberately** — the brief conditions it on the CLI figures moving or being ambiguous; they did neither (§6) |
| B-3 | `currentPath` sweep | `grep -rn "currentPath" src/ e2e/ --exclude-dir=.generated` | 13 lines, listed with line numbers | identical, same line numbers | **reproduces** |
| B-4 | dirty-family absence in the editor | `grep -nEi "dirty\|isDirty\|unsaved\|modified" src/views/EditorView.vue` | nothing | nothing, exit 1 | **reproduces** (fired, §4.2) |
| B-5 | mutation-path enumeration | `grep -nE '^\s*model\.value = ' src/views/EditorView.vue` | 7 lines at 233,309,316,407,424,454,471 | identical | **reproduces** |
| B-6 | in-place-mutation blind-spot expression | the plan's second expression | nothing, exit 1 | nothing, exit 1 | **reproduces** (fired, §4.2) |
| B-7 | no external writer through the prop | `grep -n "<EditorView" -A2 src/App.vue` | `<EditorView v-show="activeView === 'editor'" />`, no `v-model` | identical (line 254) | **reproduces** |
| B-8 | tab-switch invariant already asserted | case title in `e2e/smoke.spec.ts` | present | present, line 1388 | **reproduces** |
| B-9 | close handler + decision function already exist | symbol grep over `src-tauri/src/run.rs` | `on_close_requested`, two-variant `CloseDecision`, `close_decision` reading only `lock_active`, tests `close_decision_lets_an_idle_window_close_normally` / `close_decision_confirms_while_planning_and_while_running` | all present (543/545/548/554/574/1214/1226) | **reproduces** |
| B-10 | `AppState` already carries an `AtomicBool` | grep `src-tauri/src/lib.rs` | `quit_after_finished: AtomicBool` | present (107, default at 115) | **reproduces** |
| B-11 | shell reads exactly one catalog, en only | `grep -rn "include_str!" src-tauri/src/` | one site | one site: `run.rs:519` | **reproduces** |
| B-12 | the shell test's key enumeration and pinned wording | read `run.rs` test body | 4 `close-abort-*` keys; `close-abort-title` pinned to `Abort running jobs`; companion prefix-match test | identical (1322-1348) | **reproduces** |
| B-13 | CLI i18n house pattern | grep `crates/muxsmith-cli/src/i18n.rs` | `const LOCALES: &[(&str,&str,&str)]`, `Renderer::new(locale: Option<&str>)`, `sys_locale::get_locale()` fallback | identical (16/39/42) | **reproduces** |
| B-14 | `sys-locale` is a CLI-only dependency | `grep -rn "sys-locale" */Cargo.toml` | `muxsmith-cli` only | `crates/muxsmith-cli/Cargo.toml:19` only | **reproduces** |
| B-15 | E1, loader-call surface | the plan's `git ls-files … xargs grep` | 9 lines in 4 files | 9 lines in 4 files, identical | **reproduces** |
| B-16 | E1's control over `EditorView.vue` | same expression, one file | 0 | 0, exit 1 | **reproduces** |
| B-17 | E2, path-literal surface | the plan's second expression | `src/i18n/index.ts:18` plus CLI/shell/help/check-i18n/import/doc lines | 14 lines, including `src/i18n/index.ts:18` | **reproduces** — see the note in §4.3 |
| B-18 | gate parity scope covers `gui-common.ftl` | read `scripts/check-i18n.mjs` | `referenceCatalogFiles` = ALL `.ftl` in `locales/en/`, comment quoted | identical (340-344) | **reproduces** |
| B-19 | `RUST_ONLY_IDS` enumerates the four shell keys | read `scripts/check-i18n.mjs` | four `close-abort-*` keys under a D31 comment | identical (114-122) | **reproduces** |
| B-20 | `applyLocale` has exactly two callers | `grep -rn "applyLocale" src/` | `main.ts` bootstrap + `SettingsDialog.save()` | `src/main.ts:25`, `src/components/SettingsDialog.vue:72` | **reproduces** |
| B-21 | the dialog plugin's `confirm` routes through `message` | read `node_modules/@tauri-apps/plugin-dialog/dist-js/index.js` | `messageCommand` -> `invoke('plugin:dialog\|message')`, compared to ok label | identical (117-118, 170-172) | **reproduces** |
| B-22 | plugin-dialog version | `package.json` | 2.7.1 | 2.7.1 | **reproduces** |
| B-23 | granted capabilities | `src-tauri/capabilities/default.json` | the 7 named, no `dialog:allow-message` | identical set of 7 | **reproduces** |
| B-24 | `RunHistory.saveLog` capture pattern | grep `src/components/RunHistory.vue` | the "Captured before the dialog gap" comment, `defaultPath`, Fluent filter name | present (105, 119, 120) | **reproduces** |
| B-25 | catalog inventory | `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/<loc>/<file>` | gui-editor **46**, gui-settings **8**, gui-common **38**, identically en/de | 46 / 8 / 38, identically en/de | **reproduces** |
| B-26 | registry `labelKey` ids | `grep -oE 'labelKey:\s*"[a-z][a-z0-9-]*"' src/editor/registries.ts \| sort -u \| wc -l` | 42 | 42 | **reproduces** |
| B-27 | **decision numbering** | `grep -rE '\bD10[6-9]\b'` and the widened form over the named sources | highest in use **D105**; both expressions return nothing | highest in use **D111**; both expressions return hits | **MOVED** — §4.4 |
| B-28 | the two stale in-tree `45` comments | grep `src/`, `e2e/` | both present, ledger says 46 | both present; Tier-2 `editor-generic-action-keys` still states 46 | **reproduces** |
| B-29 | **README's first example does not load** | `./target/debug/muxsmith validate <block 1>` | error `missing field 'pattern'`, exit 2 | `Profile is valid.`, exit 0 | **MOVED** — §4.5 |
| B-30 | **the ROADMAP round-3 entry** | `git grep -n "OWNER QA PASS, round 3" 148f19f -- docs/ROADMAP.md` | cited as ground truth | absent at 148f19f; present at HEAD (line 1116) | **MOVED** — §4.6 |
| B-31 | the two existing locale-control assertions | `grep -rn "toHaveValue" e2e/*.ts` | two on the locale control, `"en"` before the save, `"de"` after the reload, inside `test.describe("german locale")` | identical (826, 848; describe 732-850) | **reproduces** — see §4.3 note on the expression's phrasing |
| B-32 | identical en/de gui-* values | full multi-line comparison, attributes excluded | **15**; 16 counting value-less messages; 18 comparing first lines only; both language option labels in the set | 15 / 16 / 18; extra member of the 16 is `batch-recents-select`; the two further members of the 18 are `batch-diagnostics-summary` and `jobs-row-warning-count`; `settings-locale-option-en` = `English`/`English` and `-de` = `Deutsch`/`Deutsch` | **reproduces** |
| B-33 | L1 pre-state and its soundness control | `grep -rn "navigator.language" src/ \| grep -v "src/i18n/index.ts"`, then unfiltered | pre-state exactly **2**, both `src/main.ts`; control's surviving `index.ts` occurrences 2 on the end state (1 pre-existing comment + 1 new line) | pre-state 2 (`main.ts:17`, `main.ts:19`); unfiltered adds exactly 1 `index.ts` line (the `primarySubtag` doc comment, line 33) | **reproduces** |
| B-34 | Task 1's three sweep expressions must return non-empty | the three greps over the spec | must not be zero | 37 / 1 / 1 | **reproduces** |
| B-35 | H1's fire against `docs/INSTALL.md` | `grep -cE 'https?://\|\|\|</?[a-zA-Z]' docs/INSTALL.md` | matches | 9 | **reproduces**; the two target topics return 0 today, which is the check's own end-state value |
| B-36 | the gate part-count self-audit | the plan's own alternation over the plan | 1 line (the auditing sentence itself) | 1 line, same sentence | **reproduces** |
| B-37 | self-review counts | recounted from the document | 7 tasks; 42 requirements (highest R42); 69 acceptance halves = 10+12+22+23+2; 5 work items; 5 ADRs D106-D110; 7 corrections; 15 new ids = 8 editor + 1 settings + 6 common; 30 lines | 7 / 42 (highest R42) / 69 = 10+12+22+23+2 / 5 / 5 / 7 / 8+1+6=15 / 30 | **reproduces** |
| B-38 | the Plan-12 target surface in the delta | `git diff --name-only 148f19f..HEAD -- help/ locales/ src/ e2e/ scripts/ src-tauri/` | (not an authoring figure; measured here) | exactly 2 files, both `editor-match-expr-exact.md`, neither a Plan-12 target | new measurement, §5 |
| B-39 | `e2e/mocks.ts` default `get_settings` locale | grep `e2e/mocks.ts` | `locale: "en"` | `e2e/mocks.ts:121: locale: "en",` | **reproduces** |
| B-40 | the three `editor-save` assertions all run after an Open | grep + context read of `e2e/editor-rule-add-remove.spec.ts` | all three post-Open | 268/310/353, each preceded by an `editor-open` click (230/300/342) | **reproduces** |
| B-41 | `TextWidget` commits per keystroke | grep `src/editor/widgets/TextWidget.vue` | `v-model` on both `<input>` and `<textarea>` | lines 35 and 43 | **reproduces** |
| B-42 | BatchView writes the profile behind the editor's back | grep `src/views/BatchView.vue` | `applySuggestion` then `saveProfile` | lines 245 and 250 | **reproduces** |
| B-43 | Playwright version | `package.json` | 1.61.1 | 1.61.1 | **reproduces** |
| B-44 | mkvmerge runtime companion | `mkvmerge --version`, `--list-types` | `mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit`; `Matroska audio/video files [mk3d mka mks mkv]` | identical | **reproduces** |
| B-45 | mkvtoolnix parity findings (3) | version + mtime check on `~/Downloads/mkvtoolnix` | measured at authoring | MKVToolNix **100.0**; newest file in the tree dated **2026-07-05**; nothing modified since 2026-07-25 | **reproduces** — source greps not re-run, per the brief |
| B-46 | snapshot byte sizes | not re-run as a fresh probe | seed **101 bytes**, README four-rule example **419 bytes** | seed figure corroborated at **101** from the plan's own pasted S4 JSON; serialization path not in the delta | **corroborated, not re-run** — §6 |
| B-47 | the plan's 16 house-knowledge ids all exist | id lookup in the four house YAMLs, ids derived from the plan's own sentence | all cited as ground truth | 16/16 present | **reproduces** (fired, §4.2) |
| B-48 | Tier-2 `editor-generic-action-keys` states the budget as 46 | read `docs/product-boundaries.yaml` | 46 | statement carries "REVISED AGAIN 45 -> 46 by the plan-7 design (D59)" | **reproduces** |
| B-49 | the ROADMAP v1.x undo/redo entry is the requirement set | `git diff` over `docs/ROADMAP.md` | cited in Task 4's Read-first | the entry text is **retained unchanged**, wrapped by a new "PULLED INTO PRE-1.0" paragraph | **reproduces** |

### 4.1 Validator seed transcript (B-1), pasted

```
########## S1
{"diagnostics":[{"code":"empty-extensions","config_path":"input.extensions","params":{},"rendered":"[error] input.extensions: The extensions list must not be empty.","severity":"error"},{"code":"no-track-rules","config_path":"tracks.rules","params":{},"rendered":"[error] tracks.rules: The profile defines no track rules; add at least one rule, or set tracks.unmatched: keep for a pure passthrough remux.","severity":"error"}]}
exit=2
########## S2
{"diagnostics":[{"code":"empty-extensions","config_path":"input.extensions","params":{},"rendered":"[error] input.extensions: The extensions list must not be empty.","severity":"error"},{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"rendered":"[warning] tracks[0].match: This match expression is empty and would match every track.","severity":"warning"}]}
exit=2
########## S3
{"diagnostics":[{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"rendered":"[warning] tracks[0].match: This match expression is empty and would match every track.","severity":"warning"}]}
exit=1
########## S4
{"diagnostics":[{"code":"empty-match-expression","config_path":"tracks[0].match","params":{},"rendered":"[warning] tracks[0].match: This match expression is empty and would match every track.","severity":"warning"}]}
exit=1
########## S5
{"diagnostics":[{"code":"passthrough-profile","config_path":"tracks.rules","params":{},"rendered":"[info] tracks.rules: This profile defines no track rules and tracks.unmatched is keep: a pure passthrough remux; every primary track is copied unchanged. If this is not intended, add track rules.","severity":"info"}]}
exit=0
```

`cargo build -p muxsmith-cli` reported `Finished dev profile ... in 0.12s` before the run,
so the binary is current against HEAD — which matters, because `matcher.rs` and
`report/mod.rs` are both in the delta and are the one plausible mechanism by which these
figures could have moved. They did not: S4 is still the only non-passthrough seed whose
worst diagnostic is a warning, so **D107's seed choice stands on a re-measured basis.**

### 4.2 Absence-shaped Class B checks, with their fires

**B-4 / B-6, and the plan's D1 pre-state.**

```
$ grep -nEi "dirty|isDirty|unsaved|modified" src/views/EditorView.vue
exit=1                                    # empty

# CONTROL (the plan's own): same expression, known-present case
$ grep -nEi "dirty|isDirty|unsaved|modified" ~/Downloads/mkvtoolnix/src/mkvtoolnix-gui/merge/tab.cpp
552:  auto modified = false;
557:      modified                            = true;
560:  if (modified)
654:Tab::hasBeenModified() {
control-exit=0
```

```
$ grep -nE 'model\.value\.[A-Za-z_]+ *=|model\.value\.[A-Za-z_.]*\.(push|splice|pop|shift|unshift|sort|reverse)\(' src/views/EditorView.vue
exit=1                                    # empty

# FIRE against a synthetic file carrying both forms
$ grep -nE '<same expression>' fire_inplace.txt
1:model.value.input = y;
2:model.value.tracks.rules.push(x);
exit=0
```

Both members of the alternation fire — the assignment form on line 1, the method form
on line 2 — so the empty result is a real absence in both halves, not one half masking
the other.

**B-45, the reference source unchanged.**

```
$ find ~/Downloads/mkvtoolnix -type f -newermt "2026-07-25"
(nothing)

# The negative is corroborated by a positive listing over the same tree,
# so "find sees nothing" is distinguishable from "find sees nothing here":
$ find ~/Downloads/mkvtoolnix -type f -printf '%T@ %TY-%Tm-%Td %p\n' | sort -rn | head -3
1783263182 2026-07-05 /home/senol/Downloads/mkvtoolnix/.tx/config
1783263182 2026-07-05 /home/senol/Downloads/mkvtoolnix/tools/development/update_translations.rb
1783263182 2026-07-05 /home/senol/Downloads/mkvtoolnix/tools/development/timestamp.pl

$ grep -m1 AC_INIT ~/Downloads/mkvtoolnix/configure.ac
AC_INIT([MKVToolNix],[100.0],...)
```

**B-47, the id-existence check.** Its own enumerated set is the risk, so the 16 ids were
**extracted from the plan's Ground-truth sentence by regex**, not typed from memory:

```
ids derived from the plan's own sentence: 16
  editor-generic-action-keys                          OK product-boundaries.yaml
  gui-closed-domain-dropdowns                         OK product-boundaries.yaml
  core-83-zero-rule-keep-passthrough                  OK product-boundaries.yaml
  gui-table-caption                                   OK conventions.yaml
  comments-locate-by-symbol-never-by-line-number      OK conventions.yaml
  tests-ship-with-the-feature-never-after             OK process-conventions.yaml
  proc-04-spec-wins                                   OK process-conventions.yaml
  proc-06-mkvtoolnix-parity                           OK process-conventions.yaml
  testing-si3-run-binary                              OK process-conventions.yaml
  proc-verification-step-must-be-falsifiable          OK process-conventions.yaml
  proc-check-green-state-reachable                    OK process-conventions.yaml
  proc-proposed-safeguard-stays                       OK process-conventions.yaml
  proc-normative-count-recomputed                     OK process-conventions.yaml
  a-document-never-cites-a-line-number-inside-itself  OK process-conventions.yaml
  a-search-whose-terms-come-from-memory-produces-a-false-absence  OK process-conventions.yaml
  proc-sweep-surface-completeness                     OK process-conventions.yaml
MISSING: none

# FIRE: the same lookup must be able to say NOT FOUND
  editor-genericccc-action-keys-XX-bogus              NOT FOUND
```

**B-34's sweeps** are the inverse shape — the plan requires them to return *non-empty*,
and they do (37 / 1 / 1), so no fire is owed; a zero would itself have been the failure.

### 4.3 Two expression-phrasing notes (neither is a move)

- **B-31.** The plan writes: "`grep -rn "toHaveValue" e2e/*.ts` returns two hits on the
  locale control". Run literally, that expression returns **19** lines across two files;
  **2** of them are the locale-control assertions. The load-bearing fact (two assertions,
  `"en"` before the save and `"de"` after the reload, both inside the German-locale
  describe) reproduces exactly. The looseness is in the sentence, not the figure, and it
  predates the delta.
- **B-17.** E2 returns **14** lines at HEAD (measured with `| wc -l`, not counted by eye).
  The plan's prose decomposition accounts for **12**: `src/i18n/index.ts:18` (1) + four CLI
  sites (4) + the shell site (1) + the help site (1) + "two error-message templates" in
  `check-i18n.mjs` (2, = lines 472 and 475) + "two import paths" in `App.vue`/
  `HelpSidebar.vue` (2) + "one doc comment" (1, = `src/i18n/index.ts:35`). **Two lines are
  unaccounted**, both further `check-i18n.mjs` strings the pattern legitimately matches:
  line 571 (`referencedHelpIds.set(m[1], "src/help/state.ts (VIEW_TOPICS)")`) and line 591
  (the `has no help/${locale}/${id}.md` error template). None of E2's files is in the
  delta, so this is an authoring-time decomposition slip, **not** a movement. Recorded
  because the plan cites that decomposition as the evidence for its five-surface
  conclusion; the conclusion itself is unaffected — both extra lines sit in
  `check-i18n.mjs`, which the plan already classifies as "the gate, not a surface".

### 4.4 B-27, decision numbering: the measurement

```
$ git grep -lE '\bD111\b' 148f19f -- docs/
exit=1                                    # absent at the authoring baseline
# FIRE, same command shape for a number known present there:
$ git grep -lE '\bD105\b' 148f19f -- docs/
148f19f:docs/ROADMAP.md
148f19f:docs/decision-ledger.yaml
... (32 files in total)
control-exit=0

# highest D-number across the plan's named sources
#   (docs/superpowers/specs/*.md, docs/ROADMAP.md, the four house YAMLs)
at 148f19f : ... 101 102 103 104 105
at HEAD    : ... 104 105 106 109 110 111

# the plan's two stated absence expressions, at HEAD, over those named sources
$ grep -rnE '\bD1(0[6-9]|10)\b' docs/superpowers/specs/*.md docs/ROADMAP.md docs/*.yaml
docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md:31:- **D-number collision check, measured:** `D106`-`D110` are reserved by Plan 12
docs/decision-ledger.yaml:5440:  ... ref: "plan-12 amendment delta review A3 ..."
docs/ROADMAP.md:1354:  ... It was presented as contradicting D109 decision 5. ...
```

**What moved and what did not.** The *figure* moved (D105 -> D111) and both stated
absence expressions now return hits, so neither can be re-run as written and get its
stated zero. The *decision* is untouched: all three hits are references to Plan 12's own
reservation, and Plan 11's design doc records taking D111 explicitly *because* D106-D110
were reserved for Plan 12. **D106-D110 carry no competing allocation at HEAD.**

Two consequences a controller may want to look at, stated as measurement rather than
routing: Task 1's `Must not decide` list names "the five decision numbers (D106-D110,
the next free numbers, measured)" — they are still free, but no longer the *next* free
numbers; and the plan's authoring-section sentence asserting both greps return nothing is
now false as written.

### 4.5 B-29, correction 7: the measurement

```
# README block 1, at HEAD
$ ./target/debug/muxsmith validate <README block 1 at HEAD>
Profile is valid.
exit=0

# README block 1, at the authoring baseline
$ git show 148f19f:README.md -> block 1
$ ./target/debug/muxsmith validate <README block 1 at 148f19f>
[error] ... input: The profile could not be parsed: input: missing field `pattern` at line 4 column 3
1 error, 0 warnings, 0 infos.
exit=2

# README block 2, both revisions: unchanged
[info] tracks.rules: ... a pure passthrough remux ...
0 errors, 0 warnings, 1 info.
exit=0
```

The delta added `pattern: '.*'` to the README's first block — the exact repair the
ROADMAP's ruling prescribed, through the vehicle the plan names (Plan 11's fix round).
The plan's corrections-table row 7 sentence "That example does not load" is therefore
**no longer true at HEAD**, and the plan-close note that the item is "deliberately NOT on
this list ... its vehicle (Plan 11's fix round) all predate this plan" now describes a
discharged vehicle rather than a pending one.

Side effect worth recording: the README block at HEAD differs from the plan's own
simulated fix only by a trailing YAML comment on the `pattern` line, and comments do not
survive `load` + re-serialization — so B-46's 419-byte figure for the four-rule example
is not disturbed by the change.

### 4.6 B-30, the ROADMAP round-3 entry: the measurement

```
$ git grep -n "OWNER QA" 148f19f -- docs/ROADMAP.md
148f19f:docs/ROADMAP.md:946:- **OWNER QA GATE, ruled 2026-07-29 ...
148f19f:docs/ROADMAP.md:967:- **OWNER QA PASS, round 1 (2026-07-29): ...
148f19f:docs/ROADMAP.md:998:- **OWNER QA PASS, round 2 (2026-07-29): ...
# round 3 absent.
# FIRE: a heading known present at that commit
$ git grep -c "Pre-1.0" 148f19f -- docs/ROADMAP.md
148f19f:docs/ROADMAP.md:2

$ grep -n "OWNER QA PASS, round 3" docs/ROADMAP.md      # at HEAD
1116:- **OWNER QA PASS, round 3 (2026-07-30): the first pass over the PRODUCT surface,

$ git log --oneline -S"OWNER QA PASS, round 3" -- docs/ROADMAP.md | tail -1
9ad9e05 roadmap: owner QA round 3, its five rulings, and the cargo-deny question answered
```

The plan's own authoring note explains it: `git status --porcelain` at authoring showed
` M docs/ROADMAP.md`, a parallel writer's in-flight edit. The entry the plan cites as
precedence-bearing ground truth, and that Tasks 2, 3 and 5 carry in their `Read first`
lists, was therefore **uncommitted at the stated baseline** and is committed now. Verified
present at HEAD with both findings, finding 1's "RULED 2026-07-30 by the owner: shape A"
line (1175), and the v1.x undo/redo entry retained unchanged under a new
"PULLED INTO PRE-1.0" wrapper.

---

## 5. Why the source-tree half is uniformly clean

```
$ git diff --name-only 148f19f..HEAD -- help/ locales/ src/ e2e/ scripts/ src-tauri/
help/de/editor-match-expr-exact.md
help/en/editor-match-expr-exact.md
```

Two files, and neither is a Plan-12 target: Task 7 edits `help/{en,de}/view-editor.md`.
So **not one file that any Class A string or source-tree Class B figure lives in changed
in the delta.** The delta's source changes are confined to `crates/muxsmith-core`
(`matcher.rs`, `report/mod.rs`, one fixture) plus build/CI/deps and the documentation and
process layer.

That is the mechanism behind the result, and it is also the answer to the brief's sharper
worry: the Plan-11 class of failure (a fenced placement instruction that became
unperformable against the state a later step held) has no purchase here, because no later
step touched the state.

---

## 6. What was NOT verified, and why

1. **The out-of-repo model-path probe (B-2).** Not re-run. The brief conditions it on the
   CLI figures having moved or being ambiguous; they reproduced exactly, including exit
   codes, so the condition is not met.
2. **The snapshot byte sizes, 101 and 419 (B-46).** Not re-run as a fresh probe — the
   instrument is a throwaway crate outside the repo, gated by the same condition as (1).
   Two independent corroborations instead: the 101-byte figure is exactly the byte length
   of the S4 JSON the plan's own probe pasted (`len(...) == 101`, computed from the plan
   file), and `git diff --stat 148f19f..HEAD -- crates/` shows the delta touched only
   `matcher.rs`, `report/mod.rs` and one fixture — not the config types or `save`, which
   is where serialization lives. A stronger statement than "unchanged" is not available
   without rebuilding the probe.
3. **The three mkvtoolnix source greps (B-45).** Not re-run, per the brief's explicit
   instruction, on the strength of the version + mtime check (MKVToolNix 100.0, nothing in
   the tree touched since 2026-07-05).
4. **The gate.** Not run. It is not a Class A or Class B member and the brief does not ask
   for it. So this report says nothing about whether `BUILDING.md`'s gate is green at HEAD;
   each task runs it as its own exit bar.
5. **The e2e and Rust suites.** Same reason. Test *presence*, *titles* and *assertion
   text* were verified statically; no test was executed.
6. **A29's expected-absent status is a pre-state, not a proof of buildability.** It shows
   `close-discard-title` is not yet in the de catalog, which is what Task 6 Step 5's red
   state needs; it says nothing about whether the mutation will fail the intended
   assertion, which only exists after Task 6 builds it.
7. **A23 was a probe error of mine**, not a tree finding: I aimed `settings-cancel` at
   `gui-editor.ftl` when its home is `gui-settings.ftl`. Corrected as A23b and left in the
   table rather than deleted, so the row count is honest about what was run.

---

## 7. Blockers

**None.**

For the record, the three MOVED figures were each checked against the definition of a
blocker in the brief (a Class A member at zero or multiple occurrences, so that the task
carrying it cannot execute as written) and none qualifies: B-27 and B-29 are figures in
the authoring-verification and corrections sections that no task step re-runs as a gate,
and B-30 is a citation that resolves *better* at HEAD than at the stated baseline. Task 3
Step 1 re-measures the seed and Task 2 Step 6 re-measures L1; both reproduce, so neither
returns NEEDS_CONTEXT on arrival.

Routing of the three MOVED figures — whether the plan's authoring section is amended, and
whether the corrections table's row 7 and the plan close's note about it are restated — is
the controller's, not this report's.
