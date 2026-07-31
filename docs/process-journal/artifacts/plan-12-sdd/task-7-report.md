# Task 7 report: the user-facing documentation (W5)

**Status: DONE.** Full 11-part gate green on the final tree, `git diff --stat` covers exactly the
four Files-list files, tree otherwise clean, commit `7b403e8082f43f238816361f901456d809d8ad9c`.

## 1. What was built

### Steps 1-2: `help/en/view-editor.md`, `help/de/view-editor.md`

The opening paragraph's entry-set sentence now names creation alongside open/reopen ("Start a new
profile, open a profile file, or reopen one from the recent profiles" / "Lege ein neues Profil an,
öffne eine Profildatei oder eines der zuletzt verwendeten Profile"). A new `## Creating a profile` /
`## Ein Profil erstellen` section was inserted between the opening paragraph and `## Editing the
model` / `## Das Modell bearbeiten`, covering: the New seed (one extension, one empty rule, warned
not erred); the one-profile-at-a-time replace guard (creating another or opening one warns while
unsaved changes exist, declining leaves it untouched); that switching views never touches the held
profile; that quitting with unsaved changes warns too; and that every edit undoes, naming both the
action-row buttons and the keyboard shortcuts (Ctrl+Z / Cmd+Z for Undo, Ctrl+Shift+Z or Ctrl+Y /
Cmd+Shift+Z or Cmd+Y for Redo). `## Save semantics` / `## Speicherverhalten` gained a leading
paragraph on the save-dialog flow: nothing is written until Save, the first save on a pathless
profile opens a dialog asking where to put the file, every later save on that profile writes there
directly with no dialog. No new help id, no new topic file, no other topic touched -- the New button
carries no `data-help-id` (confirmed: `EditorView.vue`'s New button has no such attribute, unlike
the view root's `data-help-id="view-editor"`).

### Step 3: `locales/en/gui-batch.ftl`, `locales/de/gui-batch.ftl`

`batch-profile-none` replaced byte-for-byte with the brief's fenced text in both locales, appending
the "or create one in the Editor view" / "oder erstelle eines in der Editor-Ansicht" clause. No
placeable added to either value.

## 2. Per-behaviour verification: the artifact each documented sentence was checked against

Every sentence below was checked against the shipped surface (Tasks 3-6's committed code), not
against the plan brief's description of it.

| Documented behaviour | Artifact checked |
|---|---|
| New seeds one extension (`mkv`) and one empty rule (`{ match: {} }`) | `src/views/EditorView.vue:457-463` (`blankProfile()`) |
| The seed's only diagnostic is a WARNING on `tracks[0].match`, not an error | `src/views/EditorView.vue:450-453` (doc comment: "exactly one diagnostic, `empty-match-expression` at WARNING severity") |
| Nothing is written until Save; a pathless profile's first save opens a native save dialog | `src/views/EditorView.vue:536-567` (`doSave`, the `path === null` branch calling `saveDialog`) |
| A later save on the same profile writes directly, no dialog | `src/views/EditorView.vue:557` (`if (path === null)` -- false on a second save, dialog branch skipped) |
| The editor holds one profile at a time; replacing it (New or Open) warns while unsaved changes exist | `src/views/EditorView.vue:420-440` (`pickAndOpen`), `:515-534` (`createBlank`), both gated on `dirty.value && !(await confirmDialog.value?.ask())` |
| Declining the warning leaves the current profile untouched | Same two functions: an early `return` on decline, before any state write |
| The confirm dialog's title/message/confirm-button/cancel-button | `locales/en/gui-editor.ftl:157-159` (`editor-discard-title/-message/-confirm`), `locales/en/gui-settings.ftl:15` (`settings-cancel`, reused as the cancel label per `EditorView.vue:906`); German equivalents `locales/de/gui-editor.ftl:161-163`, `locales/de/gui-settings.ftl:16` |
| The dialog mechanism itself (native `<dialog>`, Esc reads as cancel) | `src/components/ConfirmDialog.vue:1-97` |
| Switching views never touches the held profile | `src/App.vue:253-274` (`v-show`, not `v-if`, on all three views; comment: "EditorView's open profile/diagnostics/currentPath state ... survives a switch to Jobs and back") |
| Quitting with unsaved changes warns as well | `src-tauri/src/run.rs:580-620` (`CloseDecision`/`close_decision`: `(false, true) => ConfirmDiscard`, `(true, true) => ConfirmAbortAndDiscard`, both driven by `AppState::editor_dirty`, mirrored from the frontend's `dirty` via `setEditorDirty`, `src/views/EditorView.vue:229-233`) |
| Every edit can be undone; Undo/Redo sit in the action row | `src/views/EditorView.vue:908-940` (`editor-new`/`editor-open`/`editor-undo`/`editor-redo` buttons) |
| The undo/redo keyboard shortcuts (Ctrl/Cmd+Z, Ctrl/Cmd+Shift+Z or +Y) | `src/views/EditorView.vue:868-884` (`onEditorKeydown`: `mod = ctrlKey \|\| metaKey`; `z` w/o shift -> undo; `z` w/ shift, or `y` -> redo) |
| `batch-profile-none`'s exact wording in both locales | `locales/en/gui-batch.ftl:17`, `locales/de/gui-batch.ftl:13` (post-edit, re-read after the `Edit` calls) |
| Batch gains no New control (must-not-decide item, confirmed rather than assumed) | `grep -n "editor-action-new\|New profile\|batch-profile-new" src/views/BatchView.vue` -> no match |
| `view-editor`'s h1 stays bare ("Editor view" / "Editor-Ansicht"), no scheme label needed | `docs/conventions.yaml:1036-1051` (`help-topic-h1-scheme`, closed exemption list: "the three view topics") |

## 3. Step 4: verification

### `pnpm check:i18n` (D62 help gate + cross-locale parity)

```
$ pnpm check:i18n
check-i18n: ok (42 source files scanned, 227 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
```

### `pnpm build` then `pnpm test:e2e` (build-before-e2e house rule, since this task edits `help/`
and a catalog)

```
$ pnpm build
✓ built in 156ms
$ pnpm test:e2e
Running 101 tests using 16 workers
...
101 passed (8.4s)
```

101 of 101 passed, 0 failed, including the help-topic rendering specs (`e2e/help-mode.spec.ts`,
`e2e/help-topics.spec.ts`) ~~and the batch-view specs that assert `batch-profile-none` through
`en(id)`~~ **[WRONG, see fix-round-1 item 4 below: no test file anywhere asserts this string; the
claim was inherited from the brief's own premise and not checked against the test tree before being
repeated here]**.

### Absence check H1 (content hygiene), over the two edited topics

```
$ grep -nE 'https?://|\||</?[a-zA-Z]' help/en/view-editor.md help/de/view-editor.md
$ echo $?
1
```

No output, exit 1 (grep's "no matches" code) -- clean.

### Its fire, over `docs/INSTALL.md`

```
$ grep -nE 'https?://|\||</?[a-zA-Z]' docs/INSTALL.md
4:[GitHub releases page](https://github.com/senolfeldmann/Muxsmith/releases).
...
81:- `muxsmith-<version>-linux-x86_64.deb` - Debian 13+ / Ubuntu 24.04+: `sudo apt install ./muxsmith-<version>-linux-x86_64.deb`
...
$ echo $?
0
```

The same expression hits a URL and a pipe when one is present, so the clean result above on
`view-editor.md` is a real absence, not a pattern that never fires.

### `git diff --stat` -- exactly the four Files-list files

```
$ git diff --stat
 help/de/view-editor.md   | 12 +++++++++++-
 help/en/view-editor.md   | 12 +++++++++++-
 locales/de/gui-batch.ftl |  2 +-
 locales/en/gui-batch.ftl |  2 +-
 4 files changed, 24 insertions(+), 4 deletions(-)
```

(Run before staging; `git status --porcelain` after staging showed the same four paths, tree
otherwise clean.)

### The full 11-part gate, final tree, foreground, in `BUILDING.md`'s order

| # | Command | Exit |
|---|---|---|
| 1 | `cargo fmt --all --check` | 0 |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| 3 | `cargo test --workspace` | 0 (86 passed, 0 failed, `muxsmith-gui` lib; 2 passed codegen; all doc-tests 0/0) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 |
| 5 | `cargo deny check` | 0 (advisories ok, bans ok, licenses ok, sources ok) |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 |
| 7 | `pnpm lint` | 0 |
| 8 | `pnpm build` | 0 |
| 9 | `pnpm check:i18n` | 0 |
| 10 | `pnpm test:e2e` | 0, **101 passed, 0 failed** |
| 11 | `python3 scripts/ledger-lint.py` | 0, "583 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold" |

This task changes no Rust source and no frontend TypeScript/Vue source, so parts 1-6 and 7 exercise
an unmodified tree by construction (still run in full, foreground, per the exit bar); parts 8-11 are
the ones the edited files can actually affect, and all four are green against the post-edit tree.

## 4. Must-not-decide compliance

No new help id, no new topic file (the New button carries no `data-help-id`, verified directly
rather than assumed -- section 2's table). The undo shortcuts are named in the topic, as the one
place the app documents them. The two `batch-profile-none` values match the brief's fenced text
byte-for-byte. Batch gains no New control (confirmed by grep, section 2). No topic other than
`view-editor` was edited (`git diff --stat`, section 3, lists only the four Files-list paths).

## 5. Findings against the brief's own prose (none survived contact with the shipped surface)

The dispatch warned that this plan's own prose has been wrong about its tree before (three counts
contradicting their own enumerations, an ordering comment corrected twice, one acceptance-map row
claiming a structurally impossible test). I checked every fact this task's brief asserts about the
shipped surface against the actual code before writing a sentence that repeats it (section 2's
table). All of it held: the seed shape, the warning-not-error severity, the save-dialog gating on a
null path, the one-profile-at-a-time replace guard and its decline behaviour, the view-switch
persistence (`v-show`), the shell close-warning wiring, the undo/redo shortcut keys, and the
`batch-profile-none` fenced text. No contradiction between the brief and the code was found, so
nothing here is returned as NEEDS_CONTEXT.

**Fix round 1 correction (see section 3 above, marked in place):** this self-audit's own list is
where the missed claim should have been checked and was not. "The `batch-profile-none` fenced text"
in the list above is true (byte-exact, section 2); what was never checked is a *different* claim
made elsewhere in this same report -- that an existing test asserts that string. A fact about the
code and a fact about the test suite are different questions, and this self-audit answered only the
first for every item on the list, including this one. See fix round 1, item 4.

## Fix round 1

Verdict at `task-7-verdict.md`: spec compliance MET on every requirement (all five acceptance
halves, all five must-not-decide items). Quality: 2 Important, 2 Minor. Three items were mine to
fix; one (the missing test coverage for `batch-profile-none`) is the coordinator's, routed to the
whole-branch review's fix wave, and is not touched here beyond the one sentence it authorized.

**Commit `411f220eaee065442025da62916ee5d6d50b6b2e`**, pathspec-scoped to `help/en/view-editor.md`
and `help/de/view-editor.md` alone, unsigned, one trailer.

### Item 1 (Important, I-2): German reused "Dateiendung" for a concept the corpus calls "Erweiterung"

`help/de/view-editor.md:7` called the seed's single `input.extensions` value "eine
Kandidaten-Dateiendung". Eighteen lines later, the same file's pre-existing "Speicherverhalten"
section uses "Dateiendung" for a different referent -- the save-file's own suffix ("Das Format folgt
der Dateiendung"). The corpus's established term for the extensions-list field is "Erweiterung"
(`help/de/editor-input-extensions.md:1,3`: "Erweiterungen (Eingabe)", "Die Erweiterungsliste
entscheidet..."; `locales/de/gui-editor.ftl:43,84`: `editor-input-extensions = Erweiterungen`,
`editor-locator-extensions = Erweiterungen`; `locales/de/diagnostics.ftl:17,50`:
`empty-extensions`/`unknown-extension` messages both say "Erweiterung(sliste)"). Corrected
"Kandidaten-Dateiendung" to "Kandidaten-Erweiterung" -- the minimal fix the finding named (swap the
colliding noun, keep the "candidate" qualifier for content parity with the English "one candidate
extension").

**The rest of the German addition was checked the same way, term by term, against the corpus rather
than assumed clean because only one word was flagged:** ~~[this claim's scope]~~ **[WRONG, see
fix-round-2 item 2 below: this list's terms were all drawn from the "Ein Profil erstellen" section
alone, the paragraph the original I-2 finding sat in. The addition also touches the pre-existing
"## Speicherverhalten" section (a new lead paragraph), and none of that paragraph's content words
were checked here despite this sentence's "the rest of the German addition" framing. Redone in
fix-round-2 with the term list derived from the diff itself, both files, both sections.]**

- "Ansicht" (view) -- matches the corpus throughout (`Editor-Ansicht`, `Stapel-Ansicht`,
  `Jobs-Ansicht`; `help/de/view-batch.md:1`, `help/de/view-jobs.md:1`, and this same file's own h1).
  No change.
- "Warnung"/"Fehler" (warning/error) -- matches the file's own pre-existing usage two sections down
  ("Meldung mit Schweregrad Fehler", "Warnungen und Hinweise blockieren das Speichern nie"). No
  change.
- "Rückgängig"/"Wiederholen" (Undo/Redo) -- byte-identical to the catalog values
  (`locales/de/gui-editor.ftl:144-145`: `editor-action-undo = Rückgängig`,
  `editor-action-redo = Wiederholen`). No change.
- "Anwendung" (app, in "beim Beenden der Anwendung") -- searched the full German corpus
  (`command grep -rn "\bAnwendung\b\|\bApp\b" help/de/*.md locales/de/*.ftl`) and the shell's own
  close-dialog strings (`locales/de/gui-common.ftl:13-21`): none of them name the app as a noun at
  all -- every one uses the bare verb "beenden" ("...abbrechen und beenden?", "Beenden und
  verwerfen?"), and the English catalog does the same ("Discard changes and quit", no object noun
  either). "Anwendung" therefore is not the established corpus term reused for a second, different
  concept (the I-2 defect shape) -- there was no established term to collide with, since this is the
  first sentence anywhere in the help corpus that needs one. Left as is: not the defect class this
  item targets, and inventing a fix beyond what was found would be deciding a naming question the
  review did not raise. Named here as a residual so a future task naming the app again is not
  starting from zero.
- "Aktionsleiste" (action row) and "Tastenkürzel" (keyboard shortcut) -- both searched
  (`command grep -rn "Aktion\|Tasten" help/de/*.md locales/de/*.ftl`) and found nowhere else in the
  German corpus (the only other "Aktion" hit is `jobs-col-actions = Aktionen`, an unrelated table
  column label in the Jobs view). First use, no collision, no change.
- "Strg"/"Umschalt" (Ctrl/Shift) -- standard German keyboard-key names, not project vocabulary; nothing
  to check against a corpus term.

Re-ran the affected checks after the edit (not the full 11-part gate: this task's diff still reaches
only `help/`, per the dispatch's own scoping):

```
$ grep -nE 'https?://|\||</?[a-zA-Z]' help/en/view-editor.md help/de/view-editor.md
$ echo $?
1
$ pnpm check:i18n
check-i18n: ok (42 source files scanned, 227 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
$ pnpm build
✓ built in 161ms
$ pnpm test:e2e
101 passed (8.1s)
```

### Item 2 (Minor, M-1): English paraphrased the New button's label; German quoted it exactly

`help/en/view-editor.md:7` opened with "New starts a profile..." where the button's actual catalog
value is `editor-action-new = New profile` (`locales/en/gui-editor.ftl:151`), two words. The German
sentence beside it already opened with the byte-exact label ("Neues Profil legt..."). Corrected the
English to name the full label, in the corpus's own established convention for referencing a control
by its exact label at a sentence's head -- `help/en/editor-tracks-rules.md:13,15`: "The Add button
appends...", "The Remove button deletes..." -- giving "The New profile button starts a profile with
one candidate extension and one empty rule...". Checked the rest of the English addition against the
same convention: "Undo"/"Redo" match their catalog values exactly
(`locales/en/gui-editor.ftl:140-141`: `editor-action-undo = Undo`, `editor-action-redo = Redo`), and
neither is paraphrased anywhere in the new text. Left German's own button-reference style
unchanged -- the verdict found no defect on that side, and choosing to also reformat it to the "Die
Schaltfläche X" pattern some other topics use would be deciding a style question the review did not
raise, not fixing the one it did.

### Item 3 (Minor, M-2): the report's evidence for the seed's warning severity cited a doc comment, not the producer

Report section 2's table cited `src/views/EditorView.vue:450-453` -- a doc comment describing the
seed's behaviour -- for the claim "the seed's only diagnostic is a WARNING... not an error." Per
house entry `a-comment-citing-a-sibling-artifact-is-verified-at-that-artifact`, a comment that
attributes behaviour to code elsewhere is a claim about that elsewhere, checked there. I did not
simply substitute the verdict's own two citations (`crates/muxsmith-core/src/report/mod.rs:78` and
`e2e/editor-markers.spec.ts:70`) without re-deriving them myself, per the standing rule that a
borrowed measurement is checked before it carries a claim forward. Both reproduce
(`report/mod.rs:78`: `EmptyMatchExpression => "empty-match-expression"`;
`editor-markers.spec.ts:70`: `{ code: "empty-match-expression", severity: "warning",
config_path: "tracks[0].match", ... }`), but neither is the tightest available evidence: the first
only establishes the diagnostic code's string form, and the second is a hand-written fixture for a
marker-rendering test, not a run of the real validator against the seed's own shape. Tracing the
actual mechanism found a closer producer and a Rust unit test built for exactly this claim:

- The mechanism: `crates/muxsmith-core/src/profile/validate.rs:79-92` -- for each rule whose
  `match_expr` is empty (the seed's `{ match: {} }` is exactly this shape) and not already covered by
  the more specific empty-list diagnostic, the validator pushes
  `Diagnostic::warning(DiagCode::EmptyMatchExpression, format!("{base}.match"))`.
- The producer, run fresh: `crates/muxsmith-core/tests/validate_semantics.rs:148-156`,
  `empty_match_expression_is_warning`, which builds a profile from `"  - match: {}"` (the seed's own
  shape) and asserts the resulting diagnostic's severity is `Severity::Warning`:

```
$ cargo test --package muxsmith-core --test validate_semantics empty_match_expression_is_warning -- --exact --nocapture
running 1 test
test empty_match_expression_is_warning ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s
```

The claim itself was never wrong (both the verdict's citations and this closer one agree); only the
report's citation is corrected here, per the dispatch's own framing of this item. No file under
`help/`, `locales/`, or elsewhere changed for this item -- report-only.

### Item 4 (the coordinator's, not touched beyond the one authorized sentence)

The coordinator's own control (a sibling id from the same catalog block returning 23 hits in the
same spec file, confirming the empty result for `batch-profile-none` is real rather than a grep
mistake) is accepted without a second independent re-run -- reproducing a reviewer's control with no
new information to add would be re-running, not re-deriving, and the coordinator's routing already
states this is a plan defect (an acceptance-map row naming a producer that does not exist) rather
than an implementation gap this task's Files list could close. Corrected the one sentence the
coordinator authorized, in section 3 above (marked in place, not silently rewritten, per this
plan's own convention for a superseded claim): the report no longer states that an existing test
asserts `batch-profile-none` through `en(id)`. No test file was added or touched -- the Files list
for this task is exhaustive and names none, and adding one here would breach the plan rather than
satisfy it, exactly as the coordinator's routing says.

## 6. Fix-round verification: the affected subset only

Per the dispatch, the full 11-part gate was not re-run (this round's diff reaches only the two
`help/` files touched, already covered by the checks pasted under item 1 above). The full gate's
last green run against this task's tree is the original report's section 3.

## Fix round 2

Delta verdict at `task-7-delta-verdict.md`: all four fix-round-1 findings addressed, zero new
breakage. Two items this round: one fix (Zielort/Speicherort), one methodology redo (the sweep,
derived from the diff rather than from memory of what was already checked) plus the report
correction it requires. **Commit `87a07e839462c3777dbdb1696783d33d2c0612c3`**, pathspec-scoped to
`help/de/view-editor.md` alone, unsigned, one trailer.

### Item 1: "Zielort" corrected to "Speicherort"

`help/de/view-editor.md:23`'s save-dialog paragraph said "...öffnet der erste Speichervorgang einen
Dialog, der nach dem Zielort fragt...". The corpus's established term for "the location something is
stored at" is "Speicherort", confirmed independently (not taken from the coordinator's citation) at
both its sites:

```
$ command grep -n "Speicherort" locales/de/gui-common.ftl locales/de/gui-jobs.ftl
locales/de/gui-common.ftl:42:settings-dir-unavailable = Der Speicherort der Anwendungseinstellungen konnte auf diesem System nicht ermittelt werden.
locales/de/gui-jobs.ftl:71:job-log-unavailable = Der Speicherort des Lauf-Protokolls konnte auf diesem System nicht ermittelt werden.
```

Corrected to "...der nach dem Speicherort fragt...". Same defect shape as fix-round-1's
Dateiendung/Erweiterung, one severity notch lower (an uncollided new word next to an established
near-synonym, not a word borrowed from a colliding referent), matching the coordinator's own framing.

### Item 2: the sweep redone, term list derived from the diff, both files, both sections

The prior round's sweep claimed "the rest of the German addition was checked... term by term" but
in fact only walked the "Ein Profil erstellen" section's three paragraphs -- the section the I-2
finding sat in. The addition's fourth paragraph (`## Speicherverhalten`'s new lead sentence) was
never touched, and that is exactly where "Zielort" lived. Corrected in place above (section 3, fix
round 1's sweep claim) rather than left standing.

**Derivation method this time:** the content-word list below is read off `git diff 78e968f..7b403e8
-- help/de/view-editor.md help/en/view-editor.md` directly -- the commit that introduced the
addition -- not off the fix-round-1 report's own list of what it said it checked, and not off memory
of this conversation. Function words (articles, pronouns, prepositions, conjunctions, auxiliary
verbs) are excluded; every noun, content verb and content adjective the diff's added lines introduce
is listed. Each was checked against `help/de/*.md` + `locales/de/*.ftl` (German) or `help/en/*.md` +
`locales/en/*.ftl` (English) with `command grep`, and independently reproduced rather than trusted
from the delta verdict where the delta verdict already named a site.

**German** (both new sections: "Ein Profil erstellen"'s three paragraphs, "## Speicherverhalten"'s
new lead paragraph):

| Term | Corpus check | Verdict |
|---|---|---|
| Kandidat(en) | 5+ sites (`editor-input-extensions.md`, `editor-track-rule-source.md`, `editor-input-pattern.md`, `gui-editor.ftl` tooltip) | established, consistent |
| Erweiterung | 4 sites (`editor-input-extensions.md`, `gui-editor.ftl:43,84`, `diagnostics.ftl:17,50`) | established -- fix-round-1's fix, reconfirmed |
| Regel | extensive (`editor-attachments-rules.md`, `editor-track-rule-*.md`, this file's own prior text) | established, consistent |
| Prüfung | `view-batch.md:3`, and this same file's own pre-existing `## Prüfung bei jeder Änderung` heading | established, consistent |
| Warnung / Fehler | `diagnostics.ftl`, this file's own prior text | established, consistent (re-checked) |
| unvollständig | `cli.ftl:48`, `view-jobs.md:22` | established generic sense, consistent |
| anlegen ("Lege...an", "legt...an", "Anlegen") | one hit elsewhere, unrelated concept (`editor-output-filename.md:10`, subdirectories) -- **but** `gui-editor.ftl:156` (`editor-empty`, pre-existing, unedited): "Erstelle eines mit Neues Profil..." is the corpus's own sibling description of this exact affordance and uses "erstellen", not "anlegen" | **finding, not fixed** -- see below |
| erstellen (heading "Ein Profil erstellen") | same `editor-empty` site | consistent with the heading; inconsistent with the body's "anlegen" -- same finding |
| Ersetzen/ersetzt | `gui-editor.ftl:162` (`editor-discard-message`: "wird es ersetzt"), `editor-output-on-collision.md:7` | established, and byte-close to the confirm dialog's own wording |
| Öffnen/öffne | this file's own prior imperative ("Öffne eine Profildatei"), `gui-editor.ftl:153` comment | established, consistent |
| vorhanden(e) | `gui-editor.ftl:156` ("vorhandene Profildatei"), `editor-track-rule-match-expr.md`, `editor-output-*.md` | established, consistent -- near-exact match to the empty-state precedent |
| Änderungen | extensive, and `gui-editor.ftl:162` ("nicht gespeicherte Änderungen") is byte-identical to this addition's own phrase | established, strong match |
| ablehnen/"lehnst du...ab" | one distant hit (`gui-editor.ftl:55`, unrelated: rejecting a collision), otherwise first use | already reviewed and cleared as latitude in the original verdict's section 5 ("declining leaves it untouched" judged true elaboration); not re-opened |
| unverändert | `editor-track-rule-changes.md:3`, `editor-profile-chapters.md:7`, `diagnostics.ftl:15` | established, consistent |
| Wechsel (standalone) | zero other hits (only "Wechselwirkungen", a different compound) | first use, no collision |
| rühren/"rührt...an" | zero other hits | first use, no collision, and independently correct idiomatic German ("etwas nicht anrühren" = leave untouched) |
| Beenden/beenden | `gui-common.ftl:13-21`, six close-dialog strings, all bare-verb, no noun object anywhere | established, consistent (re-checked) |
| Anwendung | zero hits anywhere in the German corpus as a noun for "the app" | first use, no collision -- residual, reported in fix-round-1, not re-opened |
| gewarnt/warnt | zero other hits | first use, no collision |
| Aktionsleiste | zero other hits | first use, no collision (re-checked) |
| Aktionen | `gui-jobs.ftl:24` (`jobs-col-actions = Aktionen`, a different UI context, same general "user-triggered action" sense) | consistent, not a referent collision |
| Tastenkürzel | zero other hits | first use, no collision (re-checked) |
| Strg / Umschalt | standard German keyboard-key names | not project vocabulary, nothing to check |
| Festplatte | `batch-suggestion-card.md:9`, `view-jobs.md:18,22`, `editor-tracks-rules.md:15`, `gui-jobs.ftl:14,15,50`, **and this same file's own line 25** | established, strong match |
| Pfad | 9 files | established, consistent |
| Speichervorgang | zero other hits | first use, no collision |
| Dialog (prose) | zero other hits in `help/de/*.md` or `locales/de/*.ftl` prose | first use, no collision |
| Speicherort ("Zielort" before the fix) | `gui-common.ftl:42`, `gui-jobs.ftl:71` | **defect, fixed** -- item 1 above |

**English** (both new sections, `help/en/view-editor.md`):

| Term | Corpus check | Verdict |
|---|---|---|
| new / "New profile" | `editor-action-new = New profile` | fix-round-1's fix (M-1), reconfirmed |
| Creating/creating/Create | `gui-editor.ftl:152` (`editor-empty`: "Create one with New profile...") | consistent with the heading and "creating another" in the body |
| Start ("Start a new profile") | same `editor-empty` site uses "Create", not "Start"; no other EN site uses "Start" for profile creation | **finding, not fixed** -- the English mirror of the anlegen/erstellen split: this addition uses two different verbs ("Start" in the opening sentence, "Creating"/"create" in the heading and body) for the same action, and the established catalog precedent uses the second, not the first |
| candidate | `editor-input-extensions.md:3,6`, `editor-input-pattern.md:3`, `editor-locator-match-*.md`, `editor-track-rule-source.md:7,8,10` | established, consistent |
| extension | collapses two concepts (extensions-list vs. file suffix) already named and accepted in the original verdict's I-2 finding text | no new issue -- English's single word for both concepts is the reason the German-only defect existed at all |
| empty / rule | extensive (`EmptyMatchExpression` domain vocabulary, `editor-attachments-rules.md`, `editor-track-rule-*.md`) | established, consistent |
| validation | this file's own `## Validate on edit` heading | established, consistent |
| warning / error | `diagnostics.ftl` severities, extensive | established, consistent |
| holds / Replacing / opening / current | generic prose verbs/adjectives, no competing established term | no defect class applies |
| unsaved changes | `gui-editor.ftl:158` (`editor-discard-message`: "has unsaved changes") | established, byte-identical match |
| declining | zero other hits, but already reviewed and cleared as latitude in the original verdict (same item as German "lehnst du ab") | not re-opened |
| untouched | `editor-output-on-collision.md:17` ("stay untouched") | established, consistent |
| Switching / view | `view-batch.md:1`, `view-editor.md:1`, extensive "Editor view"/"Batch view"/"Jobs view" naming | established, consistent |
| Quitting | zero other prose hits, but pairs with the catalog's own bare "quit" verb family (`close-abort-confirm`: "Abort jobs and quit") | consistent verb, no noun-object established either (mirrors German "Beenden") |
| app | zero hits anywhere in EN help prose as a noun for "the app" | first use, no collision -- same class as German "Anwendung", not previously reported for English specifically; recorded here |
| undone / Undo / Redo | `editor-action-undo = Undo`, `editor-action-redo = Redo` | established (re-checked) |
| action row | zero other hits | first use, no collision (re-checked) |
| keyboard shortcuts | zero other hits | first use, no collision |
| written / disk | `batch-suggestion-card.md:9`, `editor-attachments-rules.md:7`, `editor-output-*.md`, `editor-profile-chapters.md:12`, `editor-tracks-rules.md:15`, `view-jobs.md:18,22`, **and this same file's own line 25** | established, strong match |
| path | 14 files | established, consistent |
| dialog (prose) | zero other hits in `help/en/*.md` prose | first use, no collision |
| "asking where to put the file" | the corpus's established noun for this concept is "location" (`gui-common.ftl:63` `settings-dir-unavailable`, `gui-jobs.ftl:68` `job-log-unavailable`, both "...location could not be determined") | **reviewed, not a clear defect** -- phrased as a clause ("where to put") rather than a competing noun, so it does not collide with or displace "location" the way "Zielort" displaced "Speicherort"; flagged here for completeness rather than fixed |

### The two findings this sweep surfaced but did not act on, and why

Both **anlegen/erstellen** (German) and **Start/Creating** (English) are real, on-point instances of
the "corpus's own closer synonym exists" class the delta verdict's harvest section named as distinct
from I-2's collision shape -- in both cases the pre-existing, unedited `editor-empty` catalog string
(the sibling description of the exact same New-button-creates-a-profile affordance) uses one verb,
and this task's new prose uses a different one in at least one of its three/four occurrences. Neither
was fixed this round. The dispatch scoped this round to one named word (item 1) plus the sweep and
its report correction (item 2), closing with "nothing else is in scope"; a verb-choice consistency
question across a whole paragraph is a step beyond a one-word terminology fix, and pushing two more
edits through on my own judgment risks exactly the kind of self-authorized scope creep this plan's
own standing rule (`proc-proposed-safeguard-stays`'s sibling, "no design decision in this plan is
re-opened, softened or improved, without evidence and routing") warns against, in a round explicitly
meant to be the last. Reported here, with full evidence, for the coordinator or the whole-branch
review to route -- the same treatment "Anwendung" got in fix round 1, and for the same reason: a
finding is not a license to fix it without being asked, once the ask has already been scoped.

### Verification

```
$ grep -nE 'https?://|\||</?[a-zA-Z]' help/en/view-editor.md help/de/view-editor.md
$ echo $?
1
$ pnpm check:i18n
check-i18n: ok (42 source files scanned, 227 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
$ pnpm build
✓ built in 155ms
$ pnpm test:e2e
101 passed (8.1s)
```

`git diff --stat` for this round: `help/de/view-editor.md | 2 +-`, one file, one line changed --
matching the one-word fix. The full 11-part gate was not re-run, per the dispatch ("not required for
a two-word change unless your sweep turns up something wider"); the sweep turned up two findings but
zero additional fixes, so the fix diff stays a single word and the narrower re-run stands.

