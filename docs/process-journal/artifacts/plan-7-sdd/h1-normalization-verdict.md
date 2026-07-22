# Plan-7 close: h1-scheme normalization - independent review verdict

Independent reviewer, fresh eyes. Scope: commits `49235c1` (h1 changes) and
`a5015b9` (cross-reference fixes), plus the whole 22x2 help tree. Ratified
form (owner ruling S21, per T9 verdict Q1 / T10 verdict Q2): a help topic's
h1 is `<label> (<section>)` - the control's actual registry label in the
locale, followed by its editor section in parentheses; bare-template
exception kept.

Working tree `/home/senol/Git/Muxsmith`, branch master, HEAD `52ade44` (a
docs-only commit sits above `a5015b9`, as anticipated; the two reviewed
commits and the help tree are unaffected).

## Combined verdict: APPROVED

- **Spec-compliance verdict: APPROVED.** All 5 changed topics x 2 locales
  implement the ratified form exactly: label half byte-matches the
  gui-editor catalog registry label; section half follows the conforming
  precedent (en lowercase catalog section, de catalog-cased noun).
- **Quality verdict: APPROVED.** The classification is sound and complete,
  the 8 deferrals are honestly and correctly scoped, and the
  cross-reference fix is complete and internally consistent. One
  pre-existing German cross-reference slip exists in the tree but was not
  introduced, worsened, or made stale by either commit - out of scope for
  the reviewed work, recorded as a HARVEST item.

## Findings by severity

**None blocking. No defect in either reviewed commit.**

- **LOW / OBSERVATION (pre-existing, out of scope).**
  `help/de/editor-track-rule-match-expr.md:5` references "siehe das Thema
  **Exact**" (English label) for a topic whose German title is `Exakt
  (Match)`; the English counterpart correctly says "the Exact topic". This
  line was touched by neither commit, and it never referenced the old
  German title (`Exakter Match`), so the rename did not make it stale - it
  is a latent en-label-in-de-reference slip. The reference still resolves
  loosely (leads with the `Exakt`/`Exact` stem). Flagged for the owner
  pass, not a fix against this work.

## Review dimension 1: the 5 changed h1s (label byte-match + section precedent)

Byte-compared each h1 label-half against its catalog registry label in both
locales (`grep`-extracted catalog value vs h1 label, bracketed-echo
fire-check showing real non-empty strings on both sides). All 18 labelled
topics OK; the 5 changed ones specifically:

| topic | en label / catalog | de label / catalog | en section | de section |
|---|---|---|---|---|
| editor-tracks-unmatched | Unmatched = Unmatched | Nicht zugeordnet = Nicht zugeordnet | (tracks) | (Spuren) |
| editor-tracks-rules | Rules = Rules | Regeln = Regeln | (tracks) | (Spuren) |
| editor-attachments-unmatched | Unmatched = Unmatched | Nicht zugeordnet = Nicht zugeordnet | (attachments) | (Anhänge) |
| editor-attachments-rules | Rules = Rules | Regeln = Regeln | (attachments) | (Anhänge) |
| editor-match-expr-exact | Exact = Exact | Exakt = Exakt | (match) | (Match) |

Section halves verified against precedent: `(tracks)/(Spuren)` from
`editor-profile-tracks = Tracks/Spuren`, `(attachments)/(Anhänge)` from
`editor-profile-attachments = Attachments/Anhänge` - en lowercased, de
catalog-cased, matching the input/output/track-rule precedent exactly.
`(match)/(Match)` for exact derives from the match-expression control label
`editor-track-rule-match-expr = Match/Match` (the containing MatchExpr
struct has no `editor-profile-*` anchor); it uses catalog vocabulary, is
confirmed by the topic body's own "the Match topic describes the full
algebra", and is the one changed topic whose section basis is a control
label rather than a top-level section label - defensible and within the
form, noted under HARVEST.

Commit shape verified: `49235c1` = 10 files, each exactly 1 ins / 1 del (the
h1 line only); bodies untouched. Single-h1 invariant clean (every help file
has exactly one `^# `).

## Review dimension 2: classification audit (re-derived all 22 x 2)

Walked all 22 topics x 2 locales independently. The implementer's 9
conforming / 5 changed / 8 NEEDS_CONTEXT split is **correct**:

- **9 conforming** - each label-half byte-matches its catalog registry
  label and each section-half follows precedent (verified programmatically,
  both locales). No topic classified conforming actually deviates. The
  template pair is correctly the ratified bare exception.
- **5 changed** - each genuinely deviated before (`Unmatched tracks`, `Track
  rules`, `Attachment rules`, `Unmatched attachments`, `Exact match` and the
  de compounds) and now conforms. None was already conforming, so none was
  changed unnecessarily.
- **8 NEEDS_CONTEXT** - none is actually conforming or should-have-changed
  under the ratified form (deferral judgments below).

No misclassification found in either direction.

## Review dimension 2b: deferral judgment per group

- **Group 1 - Locator pair (15 editor-locator-match-to-source, 16
  editor-locator-match-pattern): CORRECT.** Both have real registry labels,
  but the section is genuinely ambiguous, not merely un-attempted: two
  structural anchors (the `editor-external-block-external = External
  locator / Externer Verweis` block vs the abstract Locator struct), a
  German adjective-casing question (`Externer` vs `externer Verweis`), and -
  verified in the T9 verdict HARVEST - an explicit flag that the bare short
  form "Verweis" needs owner ratification. Deferring the section choice to
  the owner follows a recorded open decision; not over-caution.
- **Group 2 - Top-level Profile controls (17 editor-profile-chapters, 18
  editor-profile-title): CORRECT.** These carry real registry labels but are
  themselves top-level profile sections (catalog "## Profile (top-level
  sections)") with no containing parent section to place in parentheses.
  They fall outside the `(section)` form the same way the template does
  (no qualifying section), so deferring the keep-bare-vs-invent-`(profile)`
  choice is right. Not over-caution: the "real label AND unambiguous
  section" test fails on the section leg (there is no section, not an
  ambiguous one).
- **Group 3 - View topics (19 view-batch, 20 view-editor, 21 view-jobs):
  CORRECT.** No registry label, whole views rather than controls-in-a-
  section; neither leg of the form applies. Correctly outside the form.
- **Group 4 - Suggestion card (22 batch-suggestion-card): CORRECT.** No
  registry label, a card component rather than a control in a section.
  Correctly outside the form.

All 8 deferrals CORRECT; none over-cautious.

## Review dimension 3: the 6 cross-reference fixes + seventh-reference sweep

- Each of the 6 fixes verified against the diff and the current tree: each
  prose reference now names the renamed topic's new title in its own locale
  (`Rules (tracks)`, `Unmatched (tracks)`, `Rules (attachments)` and the de
  equivalents). Commit shape: `a5015b9` = 6 files, each 1 ins / 1 del.
- The parenthetical is correctly retained in these references because the
  bare labels (`Rules`, `Unmatched` / `Regeln`, `Nicht zugeordnet`) collide
  across the tracks/attachments sections - the same reason the h1 carries
  it. References that lead with a unique stable label word are correctly
  left bare: "the Match topic" (label `Match` unique), "the Exact topic"
  (label `Exact` unique, no collision) - consistent with the stated
  retain-only-on-collision rule.
- **Fire-check:** the old-title grep
  (`Track rules|Unmatched tracks|Attachment rules|Unmatched attachments|`
  `Spurregeln|Nicht zugeordnete Spuren|Regeln für Anhänge|`
  `Nicht zugeordnete Anhänge`, filtered to reference context) fired on all
  6 stale sites at the pre-fix commit `49235c1` - the pattern is sound.
- **Seventh-reference result:** none. At HEAD the remaining
  "Spurregeln"/"track rules" matches are all common-noun prose ("Anders als
  Spurregeln...", "the same match algebra as track rules...", "Diesen
  Ausweg gibt es nur bei Spurregeln", "die Spurregeln als Tabelle"), not
  title cross-references; correctly untouched. Walked every "topic"/"Thema"
  reference in the tree: all resolve to a current title except the
  pre-existing de "Thema Exact" slip recorded as the LOW observation above,
  which was not made stale by the rename.

## Review dimension 4: gates (my own build, foreground)

| gate | result |
|---|---|
| `pnpm check:i18n` | PASS - `check-i18n: ok (41 source files scanned, 211 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| `playwright test --grep help` (fresh tsc + harness + mount builds) | 9 passed |
| `pnpm test:e2e` (full: tsc + 2 vite builds + playwright) | 53 passed |

All three reproduce the report's claims exactly.

## HARVEST

- **Pre-existing de cross-reference slip (owner-pass / follow-up
  candidate):** `help/de/editor-track-rule-match-expr.md:5` "siehe das Thema
  Exact" should read "Exakt" to match the de topic title `Exakt (Match)`;
  the en counterpart is already correct. An English label word leaked into a
  German prose reference. Cheap to fix in the same pass that normalizes the
  deferred 8.
- **Over-restriction watch on the ratified form's section-derivation
  rule:** the section half is only mechanically determined for topics whose
  section has a clean `editor-profile-*` anchor (input/output/tracks/
  attachments) or a ratified struct name (track rule / Spurregel). The
  exact-match topic already derives its section from a *control* label
  (`editor-track-rule-match-expr`), and the 8 deferrals are exactly the
  topics where "what counts as the section" is not settled. The owner ruling
  that normalizes the remaining 8 should state the section-derivation rule
  explicitly - section = nearest containing editor section; no containing
  section -> bare exception (template, top-level Profile controls, views,
  card); ambiguous section label -> owner ratifies (locator) - so a
  follow-up normalizes without re-deriving.
- **Ledger candidate - the scheme is entirely review-enforced, not gated:**
  confirmed `check:i18n`/D62 does not check h1 text against any scheme (the
  normalization produced zero gate signal). If the `Label (section)` form is
  ratified branch-wide, a cheap machine check - for every topic whose
  help-id maps to a registry label, assert the h1 label-half equals the
  catalog label in that locale - would convert the whole classification from
  a per-session review duty into a gate. The map help-id -> labelKey is
  mechanical for the 18 labelled topics; the exempt classes (bare template,
  top-level Profile, views, card) are a small enumerated allowlist.
