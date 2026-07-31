# Task 7 verdict: the user-facing documentation (W5)

Reviewed: `task-7-brief.md` against `task-7-report.md` and `review-78e968f..7b403e8.diff`, commit `7b403e8082f43f238816361f901456d809d8ad9c` on `master`. Every claim below was reproduced at the artifact named beside it; none is taken from the report on trust.

## 1. Spec compliance, requirement by requirement

| # | Requirement (brief) | Verdict | Evidence |
|---|---|---|---|
| S1a | English opening paragraph's entry-set sentence gains creation | MET | `help/en/view-editor.md:3`: "Start a new profile, open a profile file, or reopen one from the recent profiles" |
| S1b | New `## Creating a profile` section, after opening paragraph, before `## Editing the model` | MET | `help/en/view-editor.md:5-11` sits between line 3 and `## Editing the model` at line 13 |
| S1c | Fixed content: New seeds one extension + one empty rule, warned not errored | MET | `src/views/EditorView.vue:457-463` `blankProfile()`: `extensions: ["mkv"]`, `tracks.rules: [{ match: {} }]`; severity confirmed at the real producer, not just the citing comment (see finding I-2 below): `crates/muxsmith-core/src/report/mod.rs:78` `EmptyMatchExpression => "empty-match-expression"`, `e2e/editor-markers.spec.ts:70` `{ code: "empty-match-expression", severity: "warning", config_path: "tracks[0].match", ... }` |
| S1d | Fixed content: nothing written until Save, asks where, writes there after | MET | `src/views/EditorView.vue:536-567` `doSave()`: `if (path === null)` opens `saveDialog`, later saves skip it |
| S1e | Fixed content: one profile at a time, replacing warns while unsaved changes exist | MET | `EditorView.vue:428` (`pickAndOpen`) and `:524` (`createBlank`): both gate on `dirty.value && !(await confirmDialog.value?.ask())` |
| S1f | Fixed content: switching views never touches it | MET | `src/App.vue:253-262`: `v-show`, not `v-if`, on all three views; comment confirms EditorView state "survives a switch to Jobs and back" |
| S1g | Fixed content: quitting with unsaved changes warns | MET | `src-tauri/src/run.rs:610-616` `close_decision()`: `(false, true) => ConfirmDiscard`, `(true, true) => ConfirmAbortAndDiscard`, keyed on `AppState::editor_dirty` |
| S1h | Undo/Redo named with keyboard shortcuts, deliberately the one place they're documented | MET | `help/en/view-editor.md:11`; shortcuts match `EditorView.vue:868-884` (`onEditorKeydown`) exactly; repo-wide sweep for `Ctrl+Z` / `Strg+Z` finds only these two files (see command below) |
| S2 | German topic: same content, same register, same section structure | MET, with one Important defect | Structure and content parity confirmed paragraph-by-paragraph (section 2 below); terminology defect at finding I-1 |
| S3 | `batch-profile-none`, both locales, byte-exact to the brief's fenced text | MET | Python byte comparison, both files, both `True` (section 2 below) |
| S4a | Full gate green, foreground | MET (the subset this task's diff can affect) | Reproduced `pnpm check:i18n` (identical output to the report), `pnpm build`, `pnpm test:e2e` (101 passed, 0 failed) on the unmodified tree; parts 1-6 unaffected by construction since no Rust/TS source changed (`git diff --stat` confirms) |
| S4b | Absence check H1 clean on the two topics, with a firing control | MET | Reproduced verbatim, see section 3 |
| S4c | `git diff --stat` covers exactly the four Files-list files | MET | Reproduced against `78e968f..7b403e8`, identical to the report's table |
| S5 | Commit message and trailer | MET | `git log -1 7b403e8`: subject "help+batch: the editor creates profiles, guards unsaved ones and undoes edits", `Co-Authored-By: Claude Sonnet 5` trailer present |
| MND1 | No new help id / topic file | MET | `git diff --stat` lists no new `.md`; `check:i18n` reports "22 help id(s) x 2 help locale(s)" unchanged from a clean-tree baseline |
| MND2 | Undo shortcuts named in the topic | MET | Same as S1h |
| MND3 | Two fenced `batch-profile-none` values, both locales | MET | Same as S3 |
| MND4 | Batch gains no New control | MET | `command grep -n "editor-action-new\|New profile\|batch-profile-new\|createBlank\|New\b" src/views/BatchView.vue` -> no output |
| MND5 | No other topic edited | MET | `git diff --stat 78e968f..7b403e8` lists only the four Files-list paths |

**All brief requirements MET.** The report's own narrative for S4a contains one claim that does not reproduce (see finding I-1); it does not change the S4a verdict (the gate itself is green), but it is the sharpest instance of exactly the "prose that is plausible and false" failure mode this review was pointed at, so it is filed as the top task-quality finding rather than buried in a footnote.

## 2. Locale lockstep, read paragraph by paragraph

Opening: "Start a new profile, open a profile file, or reopen one" / "Lege ein neues Profil an, öffne eine Profildatei oder eines der zuletzt verwendeten Profile" - match.
§Creating a profile, ¶1 (seed/warning): match, including the added " - das Profil ist damit unvollständig, nicht falsch" mirroring "so the profile is incomplete, not wrong."
§Creating a profile, ¶2 (replace guard, decline, view-switch, quit): match sentence-for-sentence, including the added "declining leaves it untouched" / "lehnst du das ab, bleibt es unverändert" (verified true at `EditorView.vue:428`/`:524`: an early `return` before any state write - not in the brief's fixed-content list but a true, harmless elaboration, within the task's latitude).
§Creating a profile, ¶3 (undo/redo + shortcuts): match, including modifier keys and macOS variants.
§Save semantics, new lead paragraph: match, including the added "every later save... with no dialog" / "jeder weitere Speichervorgang... ohne erneuten Dialog" (same latitude judgment as above, verified true at `doSave()`).

Byte-exact catalog check:
```
$ python3 -c "
brief_en = 'batch-profile-none = No profile selected yet. Choose one below to validate it and start a batch, or create one in the Editor view.'
brief_de = 'batch-profile-none = Noch kein Profil ausgewählt. Wähle unten eines aus, um es zu prüfen und einen Stapel zu starten, oder erstelle eines in der Editor-Ansicht.'
en_line = [l.rstrip() for l in open('locales/en/gui-batch.ftl', encoding='utf-8') if l.startswith('batch-profile-none')][0]
de_line = [l.rstrip() for l in open('locales/de/gui-batch.ftl', encoding='utf-8') if l.startswith('batch-profile-none')][0]
print(en_line == brief_en, de_line == brief_de)
"
True True
```

Typography sweep (em/en-dash, curly quotes, ellipsis, NBSP) over all four changed files: clean, all four.

Undo-shortcut sweep, repo-wide, confirming the "one place" claim:
```
$ command grep -rn "Ctrl+Z\|Strg+Z\|Ctrl+Shift+Z\|Ctrl+Y\|Cmd+Z" --include="*.md" --include="*.vue" --include="*.ts" --include="*.ftl" help/ src/ locales/
help/de/view-editor.md:11:...
help/en/view-editor.md:11:...
```
Exactly the two new lines - true.

## 3. D62 help hygiene, reproduced independently

```
$ grep -nE 'https?://|\||</?[a-zA-Z]' help/en/view-editor.md help/de/view-editor.md
(no output, exit 1)
$ grep -nE 'https?://|\||</?[a-zA-Z]' docs/INSTALL.md | head -3
4:[GitHub releases page](https://github.com/senolfeldmann/Muxsmith/releases).
12:PowerShell on Windows `Get-FileHash <file> -Algorithm SHA256` compared
28:Artifact: `muxsmith-<version>-windows-x86_64.msi` (Intel/AMD) or
```
Clean result on the topics, firing control confirmed on `docs/INSTALL.md` - a real absence, not a pattern that never fires.

```
$ pnpm check:i18n
check-i18n: ok (42 source files scanned, 227 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
$ pnpm build
✓ built in 163ms
$ pnpm test:e2e
101 passed (8.2s)
```
Identical to the report's figures. Ran `pnpm build` before `pnpm test:e2e` per the house build-before-e2e rule, even though the tree was already at the post-commit state, to rule out a stale-bundle false pass.

## 4. Task-quality findings

### Important

**I-1: the report's coverage claim for `batch-profile-none` does not reproduce - the string has never had a test, at all, in this repo's history.**

Both the brief (line 44: "the existing batch scenario asserts this string through `en(id)`, so its assertion follows the catalog automatically") and the report (section 3: "101 of 101 passed... including... the batch-view specs that assert `batch-profile-none` through `en(id)`") assert that an existing e2e test exercises this catalog value. Running the premise instead of weighing it:

```
$ command grep -rn 'en("batch-profile-none"\|expect.*batch-profile-none\|getByText.*batch-profile-none' . 2>/dev/null | grep -v node_modules | grep -v .worktrees
(no output)
```

A broader sweep of every `.ts` file under `e2e/` (excluding the gitignored, vendored `e2e/.generated/` bundle, which only embeds the catalogs' raw text for Fluent-parse checking, not a value assertion) confirms it: the string "batch-profile-none" appears nowhere in any test file, in any form, before or after this task's commit. `e2e/catalogs.spec.ts` (`assertAllCatalogsParseCleanly`) only checks that the `.ftl` files parse; it asserts nothing about any individual value. The paragraph in `src/views/BatchView.vue:347-349` that renders this string (`<p v-if="!selectedProfile">{{ $t("batch-profile-none") }}</p>`) carries no `data-testid` and is targeted by no test.

The shipped strings are still correct (verified byte-exact against the brief, section 2), and the binding is a direct, untransformed `$t(id)` call, so nothing is actually broken. But this is the exact "no-work-needed premise" the review brief named as a standing check, and it fails: a user-visible content change ships in this package with zero test coverage, old or new, because both the brief and the implementer accepted a claimed existing producer without opening the file that would have shown it does not exist. House entry `design-empirical-claims-reproducible` (`docs/process-conventions.yaml:586`, already at count 11 before this instance) names exactly this failure shape: "the review then re-runs instead of re-deriving" - re-run here, and it fails.

**I-2: German prose reuses "Dateiendung" for a concept the corpus already names "Erweiterung" - two different referents, one word, same document.**

`help/de/view-editor.md:7`: "Neues Profil legt ein Profil mit einer Kandidaten-Dateiendung und einer leeren Regel an" describes the seed's `input.extensions: ["mkv"]` value. The established German term for that field, everywhere else in the corpus, is "Erweiterung"/"Erweiterungsliste"/"Erweiterungsfilter":

```
help/de/editor-input-extensions.md:1: # Erweiterungen (Eingabe)
help/de/editor-input-extensions.md:3: Die Erweiterungsliste entscheidet, was überhaupt in den Stapel gelangt...
locales/de/gui-editor.ftl:43: editor-input-extensions = Erweiterungen
```

"Dateiendung" is not free-floating vocabulary either: the *same file*, 18 lines later, uses it for a different concept entirely - the save-file's own suffix determining output format: `help/de/view-editor.md:25`: "Das Format folgt der Dateiendung; ein YAML-Profil bleibt YAML." The English text does not have this problem, because English happens to use the single word "extension" for both concepts (`help/en/editor-input-extensions.md:3` "extension list", `help/en/view-editor.md:25` "the file's extension") - so the locale-lockstep file-set check cannot surface this, and a paragraph-by-paragraph content read is what catches it. A German reader who has read the "Erweiterungen (Eingabe)" topic has no reason to connect "Kandidaten-Dateiendung" back to that field, and two sentences that use the identical German word for two different things in one short topic is imprecise in exactly the register this task's own instruction ("the same content in the register that file already uses") was meant to preserve.

### Minor

**M-1: English paraphrases the New button's label; German quotes it exactly.**

`help/de/view-editor.md:7` opens with "Neues Profil legt ein Profil..." - byte-identical to the button's own catalog value, `locales/de/gui-editor.ftl:155` `editor-action-new = Neues Profil`. The English equivalent, `help/en/view-editor.md:7`, opens with "New starts a profile..." where the button's actual catalog value is `editor-action-new = New profile` (`locales/en/gui-editor.ftl:151`) - two words, not one. Repo-wide, this is also the first instance anywhere in `help/en/*.md` of a control named at a bare sentence-initial word with no bold, code span, or "the X button" framing (existing convention: `` `Add` attaches...`` in `editor-attachments-rules.md`, "The Add button appends..." / "The Remove button deletes..." in `editor-tracks-rules.md`). Not misleading in practice (the visible button text does start with "New"), but it is an avoidable inexactness the German sentence, sitting right beside it, does not share.

**M-2: the severity claim's cited evidence is one level removed from the actual producer.**

Report section 2's table cites `src/views/EditorView.vue:450-453` (a doc comment) as the artifact checked for "the seed's only diagnostic is a WARNING... not an error." Per house entry `a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact` (`docs/decision-ledger.yaml:5728`), a comment that cites behavior established elsewhere is a claim about that elsewhere, checked there, not at the comment paraphrasing it. The underlying claim holds - independently confirmed at the actual producer, `crates/muxsmith-core/src/report/mod.rs:78` (`EmptyMatchExpression => "empty-match-expression"`) and `e2e/editor-markers.spec.ts:70` (`severity: "warning"`, `config_path: "tracks[0].match"`) - so nothing shipped is wrong. But the report's own evidence chain stops one hop short of where this house entry says it must land, for a claim this task's help text states as fact to the end user.

## 5. Latitude taken (not in the brief's fixed-content list, judged acceptable)

- "declining leaves it untouched" (both locales) - true elaboration, verified at `EditorView.vue:428`/`:524`'s early-return-before-write.
- "every later save on that profile writes there directly, with no dialog" (both locales) - true elaboration, verified at `doSave()`'s `path === null` branch.

Both additions are factually accurate, present in both locales in parallel, and fill a gap the terser brief sentence would otherwise leave (stating a warning exists without stating what declining it does). No latitude finding against either.

## Findings summary

- Critical: 0
- Important: 2 (I-1, I-2)
- Minor: 2 (M-1, M-2)

## Harvest (for the controller, not routed by me)

- **The "an existing test already covers X" premise is the sharpest recurring shape of `design-empirical-claims-reproducible` because it looks unfalsifiable from inside the task**: the implementer has no reason to grep for a test they're told already exists, and the brief itself inherited the premise rather than checking it. The generalizable handle: whenever a brief or report says "no new test needed, an existing one asserts this," the reviewer's very first move is to grep for that assertion by the exact string it supposedly checks - not the mechanism, the literal value - before reading anything else in that section. This is the second time in this plan alone a report's confident "checked every fact against the code" self-audit (task-7 section 5) missed a claim about test coverage rather than about shipped behavior; the self-audit pattern that works for "is this fact about the code true" does not automatically extend to "is this fact about the test suite true," and a future brief/report template might ask the two questions separately.
- **A locale can hide a terminology defect that a lockstep, file-set-only check structurally cannot see, and it takes a nearby SAME-LANGUAGE reused word to produce it**: German's I-2 finding exists only because German draws a lexical distinction (Erweiterung vs Dateiendung) that English collapses into one word (extension). A translator reusing an established term from the wrong nearby concept is a distinct failure shape from mistranslation or omission - worth a name if this recurs, something like "borrowed a sibling concept's word because the target language had already spent the precise one elsewhere."
