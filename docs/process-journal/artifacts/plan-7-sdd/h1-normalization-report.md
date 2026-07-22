# Plan-7 close: h1-scheme normalization report

Implementer sweep over the 22x2 bilingual help topics. Ratified form (T9
verdict Q1, owner-adopted): a topic h1 is `<label> (<section>)` - the
control's actual registry label in that locale, followed by its surface in
parentheses. The bare-template exception is kept (T9 Q1). Label half read
from the gui-editor catalogs (`locales/{en,de}/gui-editor.ftl`); section
half from the conforming topics' precedent.

**Verdict: DONE_WITH_CONCERNS.** 5 of the 13 deviating topics normalized
(unambiguous against precedent); 8 deferred as NEEDS_CONTEXT (section name
has no precedent anchor / no registry label). Concern 1 (stale
cross-references) was RESOLVED on coordinator resume - see the
cross-reference fix section. Concern 2 (the 8-topic deferral) remains open
for an owner ruling.

- **Commit (h1 normalization):** `49235c1fa8bc59cb4351b97ca85369a0abadd64e`
  (branch master), 10 files, 10 insertions / 10 deletions, exactly the h1
  line per file.
- **Commit (cross-reference fix, follow-up):**
  `a5015b965094e1aa2f0bf0924fdabf5a2faabc67` (branch master), 6 files, 6
  insertions / 6 deletions, one reference line each.
- **Counts (topics):** 22 total = 9 conforming + 5 changed + 8 NEEDS_CONTEXT.
- **Counts (files):** 44 total = 18 conforming + 10 changed + 16 NEEDS_CONTEXT.

## Classification (all 22 topics, both locales)

Section-name precedent from the conforming set: host editor section,
lowercased in en, noun-form (catalog casing) in de. Input ->
`(input)`/`(Eingabe)`, Output -> `(output)`/`(Ausgabe)`, TrackRule ->
`(track rule)`/`(Spurregel)`.

| # | topic (help-id) | host / labelKey | en h1 | de h1 | class |
|---|---|---|---|---|---|
| 1 | editor-input-pattern | Input | `Pattern (input)` | `Muster (Eingabe)` | conforming |
| 2 | editor-input-extensions | Input | `Extensions (input)` | `Erweiterungen (Eingabe)` | conforming |
| 3 | editor-output-filename | OutputCfg | `Filename (output)` | `Dateiname (Ausgabe)` | conforming |
| 4 | editor-output-on-collision | OutputCfg | `On collision (output)` | `Bei Kollision (Ausgabe)` | conforming |
| 5 | editor-template-block-template | TemplateBlock | `Template` | `Vorlage` | conforming (ratified bare exception, T9 Q1) |
| 6 | editor-track-rule-source | TrackRule | `Source (track rule)` | `Quelle (Spurregel)` | conforming |
| 7 | editor-track-rule-match-expr | TrackRule | `Match (track rule)` | `Match (Spurregel)` | conforming |
| 8 | editor-track-rule-optional | TrackRule | `Optional (track rule)` | `Optional (Spurregel)` | conforming |
| 9 | editor-track-rule-changes | TrackRule | `Changes (track rule)` | `Änderungen (Spurregel)` | conforming |
| 10 | editor-tracks-unmatched | TracksCfg | `Unmatched tracks` -> `Unmatched (tracks)` | `Nicht zugeordnete Spuren` -> `Nicht zugeordnet (Spuren)` | **changed** |
| 11 | editor-tracks-rules | TracksCfg | `Track rules` -> `Rules (tracks)` | `Spurregeln` -> `Regeln (Spuren)` | **changed** |
| 12 | editor-attachments-unmatched | AttachmentsCfg | `Unmatched attachments` -> `Unmatched (attachments)` | `Nicht zugeordnete Anhänge` -> `Nicht zugeordnet (Anhänge)` | **changed** |
| 13 | editor-attachments-rules | AttachmentsCfg | `Attachment rules` -> `Rules (attachments)` | `Regeln für Anhänge` -> `Regeln (Anhänge)` | **changed** |
| 14 | editor-match-expr-exact | MatchExpr | `Exact match` -> `Exact (match)` | `Exakter Match` -> `Exakt (Match)` | **changed** |
| 15 | editor-locator-match-to-source | Locator | `Match to source` | `Match zur Quelle` | NEEDS_CONTEXT |
| 16 | editor-locator-match-pattern | Locator | `Match pattern` | `Match-Muster` | NEEDS_CONTEXT |
| 17 | editor-profile-chapters | Profile (top-level) | `Chapters` | `Kapitel` | NEEDS_CONTEXT |
| 18 | editor-profile-title | Profile (top-level) | `Title` | `Titel` | NEEDS_CONTEXT |
| 19 | view-batch | view (no registry label) | `Batch view` | `Stapel-Ansicht` | NEEDS_CONTEXT |
| 20 | view-editor | view (no registry label) | `Editor view` | `Editor-Ansicht` | NEEDS_CONTEXT |
| 21 | view-jobs | view (no registry label) | `Jobs view` | `Jobs-Ansicht` | NEEDS_CONTEXT |
| 22 | batch-suggestion-card | card (no registry label) | `Suggestion card` | `Vorschlagskarte` | NEEDS_CONTEXT |

## Changes made (per-change before -> after)

Label from the gui-editor catalog, section from the host's editor section.
Topics 10-13 are exactly the label-collision case the ratified form was
designed for (T9 Q1: `unmatched` and `rules` labels collide across Tracks
and Attachments; the parenthetical disambiguates).

| help-id | locale | before | after | label source | section source |
|---|---|---|---|---|---|
| editor-tracks-unmatched | en | `# Unmatched tracks` | `# Unmatched (tracks)` | `editor-tracks-unmatched` = "Unmatched" | `editor-profile-tracks` = "Tracks" |
| editor-tracks-unmatched | de | `# Nicht zugeordnete Spuren` | `# Nicht zugeordnet (Spuren)` | = "Nicht zugeordnet" | = "Spuren" |
| editor-tracks-rules | en | `# Track rules` | `# Rules (tracks)` | `editor-tracks-rules` = "Rules" | "Tracks" |
| editor-tracks-rules | de | `# Spurregeln` | `# Regeln (Spuren)` | = "Regeln" | = "Spuren" |
| editor-attachments-unmatched | en | `# Unmatched attachments` | `# Unmatched (attachments)` | `editor-attachments-unmatched` = "Unmatched" | `editor-profile-attachments` = "Attachments" |
| editor-attachments-unmatched | de | `# Nicht zugeordnete Anhänge` | `# Nicht zugeordnet (Anhänge)` | = "Nicht zugeordnet" | = "Anhänge" |
| editor-attachments-rules | en | `# Attachment rules` | `# Rules (attachments)` | `editor-attachments-rules` = "Rules" | "Attachments" |
| editor-attachments-rules | de | `# Regeln für Anhänge` | `# Regeln (Anhänge)` | = "Regeln" | = "Anhänge" |
| editor-match-expr-exact | en | `# Exact match` | `# Exact (match)` | `editor-match-expr-exact` = "Exact" | match expression, labeled "Match" (`editor-track-rule-match-expr`); confirmed by the topic's own "the Match topic describes the full algebra" and the prior compound "Exact match" |
| editor-match-expr-exact | de | `# Exakter Match` | `# Exakt (Match)` | = "Exakt" | = "Match" |

Bodies untouched (`git diff` shows only the h1 line per file). Single-h1
invariant preserved (one `^# ` per file, verified).

## NEEDS_CONTEXT (8 topics; not changed) - decision memo

Each has a genuine open decision the precedent does not close. Normalizing
them would require inventing a section name (preamble: an unenumerated set
you must invent is a fork -> return, do not decide). My recommendation is
given per group for a fast owner ruling.

1. **Locator pair (15 editor-locator-match-to-source, 16
   editor-locator-match-pattern).** Section = the (external) locator. The
   catalog names it `editor-external-block-external` = "External locator" /
   "Externer Verweis", but no conforming topic anchors the lowercase/short
   form, and the T9 harvest explicitly flags the bare de short form
   "Verweis" as needing owner ratification. Two plausible sections per
   locale: en `(external locator)` vs `(locator)`; de `(Externer Verweis)`
   vs `(externer Verweis)` vs `(Verweis)`. **Recommendation:** full catalog
   form, en `(external locator)` / de `(externer Verweis)` -> e.g.
   `Match to source (external locator)` / `Match zur Quelle (externer Verweis)`.

2. **Top-level Profile controls (17 editor-profile-chapters, 18
   editor-profile-title).** These are leaf controls directly under Profile
   with no containing sub-section - they render as their own top-level form
   sections (per view-editor). There is no parent section to name in
   parentheses. Options: keep bare (extend the template top-level
   exception) vs invent `(profile)`/`(Profil)`. **Recommendation:** keep
   bare as a top-level exception, same rationale class as the template.

3. **View topics (19 view-batch, 20 view-editor, 21 view-jobs).** Whole
   views, not controls-in-a-section; `data-help-id` on the view root, no
   registry label. The "label (section)" control form does not apply (no
   control, no containing section). **Recommendation:** exempt class -
   leave `Batch view` / `Editor view` / `Jobs view` and the de
   `*-Ansicht` forms as-is (this is the T8 "view names" scheme; it reads
   naturally and has no section to add).

4. **Suggestion card (22 batch-suggestion-card).** `data-help-id`, no
   registry label (explicit STOP trigger), a card component rather than a
   control in a section. **Recommendation:** exempt with the views, or
   coin a section only if the owner wants uniformity.

## Concerns

1. **Stale cross-references created by the 5 renames - RESOLVED (see the
   cross-reference fix section below).** Originally deferred to a
   "cross-reference sweep" owner-pass item; the coordinator ruled that
   vehicle does not exist, so the six references were fixed in this same
   task (commit `a5015b9`). The renames had made these prose
   title-references stale - they named a title that changed:
   - en `help/en/editor-tracks-unmatched.md:15` "See the **Track rules**
     topic" -> now `Rules (tracks)`
   - en `help/en/editor-tracks-rules.md:7` "see the **Unmatched tracks**
     topic" -> now `Unmatched (tracks)`
   - en `help/en/editor-attachments-unmatched.md:15` "See the **Attachment
     rules** topic" -> now `Rules (attachments)`
   - de `help/de/editor-tracks-unmatched.md:15` "das Thema **Spurregeln**"
     -> now `Regeln (Spuren)`
   - de `help/de/editor-tracks-rules.md:7` "das Thema **Nicht zugeordnete
     Spuren**" -> now `Nicht zugeordnet (Spuren)`
   - de `help/de/editor-attachments-unmatched.md:15` "das Thema **Regeln
     für Anhänge**" -> now `Regeln (Anhänge)`
   References that survive untouched: "the Match topic"/"das Thema Match"
   and "the Exact topic"/"das Thema Exakt" both lead with a stable label
   word (the label part of the new title is unchanged), so they still
   resolve correctly. The D62 gate does not check prose cross-references,
   so none of this blocked CI.

2. **8-of-13 deferral is large.** It is the honest cut: only the four
   collision-case topics plus exact-match have a section name the
   conforming precedent actually anchors. If the owner rules on the three
   groups in the memo above (recommendations given), a fast follow-up
   normalizes the remaining 8 without a re-analysis.

## Gate / test evidence (all foreground)

- **`pnpm check:i18n`** (D62 gate, checks 1-6 over help/): PASS. Output:
  `check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError
  code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1
  other locale(s) checked for parity against 7 en/ catalog(s)).` The h1
  edits introduce no `|`, URL, or raw-HTML tag (checks 4-6); the gate does
  not check h1 text against any scheme.
- **`playwright test --grep help`** (fresh harness+mount builds first):
  **9 passed** (help-topics, help-mode, locale-switch help cases).
- **`pnpm test:e2e`** (full suite, tsc + 2 vite builds + playwright):
  **53 passed.**

## Cross-reference fix (follow-up, commit `a5015b9`)

Coordinator ruling on resume: concern 1's deferral had no real vehicle (no
separately-scheduled cross-reference sweep exists), so the six stale
references are fixed in this same task. Scope: exactly the six enumerated
sites; each prose reference now names the renamed topic's NEW h1 title in
its own locale; the 8 NEEDS_CONTEXT topics and all other body text are
untouched. The parenthetical form is kept in the reference text because the
bare labels (`Rules`, `Unmatched`) collide across sections - the same
reason the h1 carries it.

| file:line | locale | before | after |
|---|---|---|---|
| help/en/editor-tracks-unmatched.md:15 | en | `See the Track rules topic.` | `See the Rules (tracks) topic.` |
| help/en/editor-tracks-rules.md:7 | en | `see the Unmatched tracks topic.` | `see the Unmatched (tracks) topic.` |
| help/en/editor-attachments-unmatched.md:15 | en | `See the Attachment rules topic.` | `See the Rules (attachments) topic.` |
| help/de/editor-tracks-unmatched.md:15 | de | `Siehe das Thema Spurregeln.` | `Siehe das Thema Regeln (Spuren).` |
| help/de/editor-tracks-rules.md:7 | de | `siehe das Thema Nicht zugeordnete Spuren.` | `siehe das Thema Nicht zugeordnet (Spuren).` |
| help/de/editor-attachments-unmatched.md:15 | de | `Siehe das Thema Regeln für Anhänge.` | `Siehe das Thema Regeln (Anhänge).` |

6 files, 6 insertions / 6 deletions (one reference line each); no `|`, URL,
or raw-HTML introduced. Post-fix survival grep over `help/` for the old
topic-reference phrases: clean.

**Gate / test evidence (foreground, re-run after the fix):**
- `pnpm check:i18n`: PASS (same `check-i18n: ok ...` line as above).
- `playwright test --grep help` (fresh harness+mount builds): **9 passed.**
- `pnpm test:e2e` (full suite): **53 passed.**

## Notes

- Did not touch `docs/superpowers/specs/` (the concurrently-edited files);
  scope was help topic files only.
- No EnterWorktree/ExitWorktree or session-relocation tool used; absolute
  paths throughout; branch master, HEAD was b686c19 at start.
