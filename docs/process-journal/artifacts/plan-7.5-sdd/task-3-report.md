# Task 3 report: D71 topic content - `editor-tracks-rules`, en + de

- Stream B, worktree `/home/senol/Git/Muxsmith/.worktrees/plan75-b`, branch `plan75-b`
- Commit: `29952b9be07d1b83fb658f30c7e27e61d1af276b` (unsigned, `%G? = N`; trailer present)
- Status: DONE_WITH_CONCERNS (both concerns below are flagged wording/scope notes for the owner pass, neither blocks the task)
- Base at start: `1d82179` ("house: plan-7.5 plan delta APPROVED ...")

## Inputs read before editing

- `implementer-preamble.md` (Global Constraints), `task-3-brief.md`
- D71 in full, including both rejected help-id routes and the rejected second-host-element route (design `2026-07-22-plan75-track-rule-add-remove-design.md`, D71 begins at :556)
- The anchors behind the claim enumeration, read in full so no sentence drifts past them: D65 (:217), D66 (:269), D67 (:329), D69 (:428); D41 save-note surface via the catalog value `editor-save-note` (`locales/en/gui-editor.ftl:134`, `locales/de/gui-editor.ftl:138`) and the Editor view topic's "Save semantics" section
- `content-claims-anchor-bound` (`docs/decision-ledger.yaml:3810`), `help-topic-h1-scheme` (`docs/conventions.yaml:1033`), `code-comment-line-citations-drift` (`docs/conventions.yaml:1012`), `core-83-zero-rule-keep-passthrough` (`docs/product-boundaries.yaml:389`), `editor-generic-action-keys` (`docs/product-boundaries.yaml:404`)
- Both current topic files in full, plus the sibling topics for register: `help/{en,de}/view-editor.md`, `help/{en,de}/batch-suggestion-card.md`, `help/{en,de}/editor-attachments-rules.md`, `help/de/view-batch.md`, `help/de/view-jobs.md`
- `scripts/check-i18n.mjs` help gate (D62 checks 1-6) to know what the structural gate actually enforces over `help/`: topic existence both directions, locale lockstep, orphans, external-URL ban, pipe/table ban, raw-HTML ban. No paragraph- or section-parity constraint exists, so the two locales' added paragraph counts are a register choice, not a gate.

## What landed

`help/en/editor-tracks-rules.md`, section "Editing a rule" - two paragraphs appended after the existing one:

> The Add button appends a new empty rule at the end of the list, selects it, and opens its detail panel. A warning flags that new rule until you fill in its match expression.
>
> The Remove button deletes the selected rule; it stays unavailable until a row is selected. Removing asks no confirmation - like every other change in the editor it touches the model only, and the file on disk changes when you save (see the Editor topic).

`help/en/editor-tracks-rules.md`, section "When the list may be empty" - one clause appended to the existing paragraph:

> Removing the last rule is allowed; the empty list it leaves behind is exactly the state described here.

`help/de/editor-tracks-rules.md`, "Eine Regel bearbeiten":

> Die Schaltfläche Hinzufügen hängt eine neue, leere Regel am Ende der Liste an, wählt sie aus und öffnet ihr Detailpanel. Eine Warnung markiert diese neue Regel, bis du ihren Match-Ausdruck ausgefüllt hast.
>
> Die Schaltfläche Entfernen löscht die ausgewählte Regel; sie bleibt gesperrt, solange keine Zeile ausgewählt ist. Entfernen verlangt keine Bestätigung - wie jede andere Änderung im Editor betrifft es nur das Modell, und die Datei auf der Festplatte ändert sich erst, wenn du speicherst (siehe das Editor-Thema).

`help/de/editor-tracks-rules.md`, "Wann die Liste leer sein darf":

> Auch die letzte Regel darf entfernt werden; die dann leere Liste ist genau der hier beschriebene Zustand.

The de text is authored in the file's own register, not a transliteration of the en: du-imperative and the file's local terminology (`Raster`/`Rasterzeile`, `Detailpanel`, `Match-Ausdruck`), `gesperrt` for the unavailable state (the term `help/de/view-editor.md` already uses for the save gate), `Bestätigung` for the confirmation notion (the term `help/de/batch-suggestion-card.md` uses). Sentence structure differs from the en on purpose (a `wie jede andere Änderung` subordinate clause where the en runs a coordinated main clause).

## Claim-to-anchor map (each sentence traced; the reviewer grades the extrapolations)

| Sentence | Claim | Anchor |
| - | - | - |
| Add ... appends at the end, selects it, opens its detail panel | enumeration bullet 1, first half | D67 (ruling 3) |
| A warning flags that new rule until you fill in its match expression | enumeration bullet 1, warning clause | D65 (empty skeleton, warning severity, empirical emission) |
| Remove deletes the selected rule; unavailable until a row is selected | enumeration bullet 1, second half | D66 (ruling 2, `:disabled="selectedIndex === null"`) |
| asks no confirmation; model-only until you save | enumeration bullet 1, tail | D66 + D41 save-note surface (`editor-save-note`, and the Editor view topic's save section) |
| Removing the last rule is allowed; the empty list is the state described here | enumeration bullet 2 | D69 (ruling 5) + `core-83-zero-rule-keep-passthrough` |

Button labels in prose are byte-equal to the catalog values (`editor-action-add` = `Add` / `Hinzufügen`, `editor-action-remove` = `Remove` / `Entfernen`; verified at `locales/en/gui-editor.ftl:138-139`, `locales/de/gui-editor.ftl:142-143`). The phrasing form "The Add button ..." / "Die Schaltfläche Hinzufügen ..." copies the house form already in the tree for a button referenced mid-prose (`help/en/view-editor.md:17` "The Apply button ...", `help/de/view-editor.md:17` "Die Schaltfläche Anwenden ..."); the bold-label form is reserved in-house for a label that leads its own bullet or paragraph, which is not the shape of this section.

### Deliberately NOT claimed

True per the design but outside D71's closed enumeration, so absent from the topic by discipline (`content-claims-anchor-bound`: the defect locus is the plausible completion past the anchor):

- selection clears after removal / the detail panel closes (D66)
- button placement relative to the grid (D70, D72)
- the diagnostic's identity and config path (`EmptyMatchExpression` at `tracks[i].match`) and what its rendered text says (D65)
- Save disabling at zero rules under `drop`, and the info notice under `keep` (D69)
- keyboard reachability, keyboard shortcuts (D70), tooltip absence (D72), any undo/redo prospect (D66 explicitly builds nothing toward it)

No severity word beyond "a warning" / "eine Warnung" appears.

## Check evidence

### Step 3a - no h1 changed, fire-verified

```
$ cd /home/senol/Git/Muxsmith/.worktrees/plan75-b && git diff help/ | grep -E '^[+-]# ' ; echo "exit=$?"
exit=1                      # no output: no h1 line changed in either locale
```

Fire-verification (h1 temporarily changed to `# FIREVERIFY Rules (tracks)`):

```
$ git diff help/ | grep -E '^[+-]# ' ; echo "exit=$?"
-# Rules (tracks)
+# FIREVERIFY Rules (tracks)
exit=0
```

Reverted, re-run: `exit=1`, no output. The empty result is therefore a measured absence, not a malformed pattern. `help-topic-h1-scheme` unaffected (content-only edit).

### Step 3b - `pnpm check:i18n`, fire-verified

```
$ pnpm check:i18n
check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).
```

Fire-verification (`https://example.com` temporarily appended in the en topic body):

```
$ pnpm check:i18n
check-i18n: help-topic gate violations (D62):
  help/en/editor-tracks-rules.md: contains an external URL (banned, D62 check 4)
[ELIFECYCLE] Command failed with exit code 1.
```

Restored, re-run: `check-i18n: ok ...`, exit 0. The D62 content gate demonstrably reaches the edited file.

### Step 4 - frontend gate, foreground

```
$ cd /home/senol/Git/Muxsmith/.worktrees/plan75-b && pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e
$ eslint .                                   # clean
$ vue-tsc --noEmit && vite build             # built in 167ms
$ node scripts/check-i18n.mjs                # check-i18n: ok (as above)
  53 passed (2.6s)                           # playwright, chromium
```

Included in the green run: `e2e/help-mode.spec.ts` (annotation/hover/pin/suppression specs, which derive expected sidebar HTML from these same topic files at runtime), `e2e/editor-markers.spec.ts`, `e2e/editor-tooltips.spec.ts`. No spec required an edit; content edits cannot desynchronize them, exactly as the brief predicted.

The five cargo parts of the nine-part gate were not run: this task touches no Rust and the plan's full-gate duty sits at the merge, not at the task commit. Flagged for the controller, not skipped silently.

### Typography, fire-controlled

```
$ grep -nP '[\x{2010}-\x{2015}\x{2018}\x{2019}\x{201C}\x{201D}\x{2026}\x{00A0}\x{2212}]' help/en/editor-tracks-rules.md help/de/editor-tracks-rules.md ; echo "exit=$?"
exit=1                      # no typographic dash, smart quote, ellipsis or NBSP
$ printf 'a \xe2\x80\x94 b\n' | grep -nP '[...same class...]' ; echo "exit=$?"
1:a — b
exit=0                      # positive control: the pattern does fire
```

German orthography intact in the de additions (`Schaltfläche`, `wählt`, `öffnet`, `Änderung`, `ausgewählte`); no ß occurs in the added text. ASCII hyphen-with-spaces as the file's existing pause marker.

### Zero-surface confirmation

```
$ git diff HEAD~1 --stat
 help/de/editor-tracks-rules.md | 6 +++++-
 help/en/editor-tracks-rules.md | 6 +++++-
 2 files changed, 10 insertions(+), 2 deletions(-)
$ git status --porcelain help/          # empty: no new topic file, nothing untracked
$ git -C /home/senol/Git/Muxsmith status --porcelain help/   # empty: main tree untouched
```

Two files, content-only: no help-id, no `data-help-id`, no topic file, no catalog id or Fluent attribute, no registry, component, Rust, `DiagCode`, eslint or dependency change. The 46-id `gui-editor.ftl` budget is untouched by construction (no catalog file in the diff).

Isolation: every command ran with an explicit `cd` into the worktree (or `git -C` for the read-only main-tree check). No `EnterWorktree`/`ExitWorktree` or any session-relocation tool was called; no background runs; staging was explicit, never `git add -A`. The report path `.superpowers/` is gitignored (`.gitignore:2`), so writing it does not modify the tracked main tree.

## Self-review

- **Claim fidelity.** Both enumeration bullets are carried; nothing beyond them. The one place the wording goes past a bare transcription is the model-only-until-save clause, which restates D41's own sentence ("an unsaved model mutation touches no file") and the Editor view topic's existing save section rather than adding a new behavioral claim.
- **Structure.** h1s untouched (fire-verified), section headings untouched, no list/table introduced, no backticks added (the additions name no config values).
- **Register.** en declarative with the file's second-person guidance; de du-imperative with the file's own vocabulary. Neither locale is a word-for-word mirror of the other.
- **Bilingual duty.** One commit carries both locales.
- **Counts.** This task changes no enumerated set, so no normative count anywhere consumes it (`proc-normative-count-recomputed` has no work here); the 22-help-id x 2-locale figure in the check output is unchanged from the pre-edit run.

## Concerns (minor, for the owner's rendered-surface pass)

1. **Cross-reference wording to the Editor view topic.** I used the form already in the tree in both locales for that target - "see the Editor topic" / "siehe das Editor-Thema" (`help/en/batch-suggestion-card.md:9`, `help/de/batch-suggestion-card.md:9`). The alternative, arguably more conformant to `help-topic-h1-scheme`'s label-leading rule, would be "see the Editor view topic" / "siehe das Thema Editor-Ansicht", matching the target h1s ("Editor view" / "Editor-Ansicht") and this file's own reference phrasing ("see the Optional topic" / "siehe das Thema Optional"). I chose in-tree precedent over composing a new form; flagging it rather than deciding it, since it is a wording question and a two-site consistency question at once (flipping it here would leave the suggestion-card topic on the old form). Not a fork in the design sense - no D71 claim depends on it.
2. **The Add/Remove code does not exist in this worktree** (stream A builds it), as the dispatch states. Every claim was verified against D71's enumeration and its anchor decisions, not against landed code. The empirical anchors (D65's single warning-severity emission, D66's disable condition, D67's auto-select) are the design's own verified-at-authoring findings; if stream A's landed implementation diverges from them, these sentences inherit the divergence. The natural place to catch that is the plan-close rendered-surface pass, where both streams are merged and the surface is looked at.
