# Owner surface-pass inputs (de wording + coined names), collected at verdict arrival

Running list for the plan-close owner pass; each item cites its source.

## Post-approval addendum 2 (h1 review, 2026-07-22)

- Pre-existing de slip (LOW, not a rename regression):
  help/de/editor-track-rule-match-expr.md:5 says "das Thema Exact" while
  that topic's own de title is "Exakt (Match)" - one-word wording fix at
  the owner's next pass.
- The 8 h1-deferred topics await the owner's section-derivation ruling:
  (a) locator pair - section name (external locator vs locator) + the de
  short form "Verweis" (T9 harvest flagged it for ratification);
  (b) chapters/title - top-level leaves, recommend staying bare;
  (c) views + suggestion card - no registry label, recommend exempt.
  The h1 reviewer's advice: state the derivation rule explicitly so the
  deferred 8 normalize without re-deriving.

## Post-approval addendum (close-batch delta review N3, 2026-07-22)

Collected AFTER the owner's wholesale approval of the list below: the
editor-output-directory tooltip (en+de) does not verbalize the
empty-to-source-dir fallback the corrected D54 cell now describes -
wording precision idea for the owner's next rendered-surface look, not
a defect (the help topic documents the behavior correctly).

## From T3 (tooltip attributes; implementer report)
- "Capture-Gruppen" (loanword), "Stapel-Ansicht", "Spender-Datei"/"Spender-Zuordnung"
  (extends the diagnostics anchor), "übersteuert" (editor-output-directory),
  the ASS-font consequence sentence in editor-attachments-unmatched.
- All en tooltip wording rides the pass too (plan rule).

## From T8 (view topics; task-08-verdict.md)
- Coined de names, ruled acceptable-as-draft (review Q1): "Vorschlagskarte",
  "Auflösungstabelle", "Editor-Ansicht"; also "Regel-Tabelle" for the grid
  (harvest).
- Nit: de view-jobs heading "Live-Protokoll" vs catalog
  jobs-log-region-label = "Live-Job-Ausgabe" (finding 3).

## From T9 (implementer report)
- h1 form "<label> (<section>)" e.g. "# Muster (Eingabe)" - cross-stream
  h1-form variation flagged (T9 vs T10 vs T8 forms differ; whole-branch
  review aligns or the pass rules).

## From T9 review (task-09-verdict.md)
- de coinages needing cross-stream sync: "Beinahe-Treffer",
  "Nur-Einschränken-Garantie", "Ein-Klick-Übernahme" (vs stream C's
  terms), bare "Verweis" short form.
- h1-form alignment recommendation for the whole-branch review: adopt
  T9's "<label> (<section>)" form (registry labels collide across
  sections; unqualified labels cannot disambiguate).
- Reviewer note: provisional content rules chafed nowhere; nothing
  argues against closing the veto window unchanged.

## From T3 review (task-03-verdict.md)
- Q5 additions: "Richtlinie"; "gemuxt/Muxen" vs diagnostics'
  "Multiplex"; basename->"Dateiname" flattening; "erfasste Felder" vs
  "Capture-Gruppen/Vorlagenfelder" inconsistency.
- Finding 1: editor-output-directory tooltip misses the empty-case
  clause; true behavior is SOURCE-DIR fallback (planner.rs:285), not
  D54's "profile's own directory" (both locales).
- Finding 2: template tooltip "source basename" -> spec 4.7's
  {source_stem} (extensionless), both locales.
- T8 fix note: the en Copy sentence ("paste it into the profile
  yourself") now reads as pasting the CONSTRAINT under the corrected
  fragment description - consistent, but review the pair together
  (batch-suggestion-card, both locales).

## From T10 review (task-10-verdict.md)
- Finding 4: en prose "Case sensitive" vs catalog label "Case-sensitive".
- Cross-stream sweep items: h1 scheme split (T9 "<label> (<section>)" vs
  T10 compounds vs T8 view names); one cross-reference sweep over all 22
  topics; DiagCode-vs-prose stream divergence.
