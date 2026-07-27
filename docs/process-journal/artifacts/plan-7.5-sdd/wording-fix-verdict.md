# plan-7.5 owner-pass wording fix - independent review verdict

**APPROVED**

Date: 2026-07-27
Reviewer: independent (fresh context, read-only except this file)
Subject: commit `406e91b` ("help+spec: owner-ruled wording fix (plan-7.5 owner pass)")
Package: `.superpowers/sdd/plan-7.5/review-a6548ed..406e91b.diff`
Report under review: `.superpowers/sdd/plan-7.5/wording-fix-report.md`
Remit: fidelity to the OWNER-RULED text and non-breakage. Not wording taste.

---

## 1. Package fidelity and diff scope

- `a6548ed` is `406e91b^` (both resolve to `a6548ed89a0584ab997fb7124f263bf54537b275`).
- The package's `## Diff` section is **byte-identical** to `git diff -U10 a6548ed 406e91b`.
  (A naive compare against plain `git diff` reports spurious differences: the
  generator writes -U10, `git diff` defaults to -U3. Only context width differs;
  every `+`/`-` line matches.)
- Scope: 5 files, 7 insertions, 7 deletions, 7 changed lines. **Nothing else in the diff.**
  No heading, no code, no locale catalog, no test, no config.

## 2. String ledger, tree-wide

Whitespace-tolerant `grep -aoE` patterns over all 1174 tracked files, excluding
`.superpowers/sdd/` (the review ledger legitimately quotes pre-edit text, so it
can never read 0 and is not a content surface).

| | old (expect 0) | new | expected |
|---|---|---|---|
| de cross-ref `das Editor-Thema` -> `das Thema Editor-Ansicht` | 0 | 2 | 2 |
| en cross-ref `the Editor topic` -> `the Editor view topic` | 0 | 2 | 2 |
| de Remove subject `; sie bleibt gesperrt, solange` -> `; die Schaltfläche bleibt gesperrt, solange` | 0 | 1 | 1 |
| en Remove subject `; it stays unavailable until` -> `; the button stays unavailable until` | 0 | 1 | 1 |
| en Add warning `A warning flags that new rule until` -> `A warning in its detail panel flags the new rule until` | 0 | 1 | 1 |
| de Add warning `Eine Warnung markiert diese neue Regel, bis` -> `Eine Warnung im Detailpanel markiert die neue Regel, bis` | 0 | 1 | 1 |
| spec `invalid until filled, announced by validation` -> `incomplete until filled, announced by a validation warning` | 0 | 1 | 1 |
| spec `legal down to zero rules per 4.5` -> `legal down to zero rules (per 4.5:` | 0 | 1 | 1 |

All eight new strings land in exactly the expected files (4 help files + the spec)
and nowhere else. All seven ruled replacements are present.

**Fire-check (the zeros are real absences, not malformed patterns).** Each of the
10 (file, pattern) pairs was run against its pre-edit blob at `a6548ed` and its
post-edit worktree state: **pre = 1, post = 0 in all 10 cases.** A deliberately
non-matching control pattern returned 0 on the same input, so the counter is not
stuck at 1. A second, line-wrap-proof pass (whole file normalized with
`tr '\n' ' ' | tr -s '[:space:]' ' '`, then `grep -oF` for each of the 8 old
strings across all 5 files) found no residual. The report's own fire-check claim
is therefore independently reproduced, not borrowed.

**Residual old-string hits (4), all correctly untouched.** These are frozen
historical artifacts, not live surfaces:

- `docs/process-journal/artifacts/plan-7-sdd/review-0fea107..cc0e6d7.diff` -
  archived plan-7 review diff (`das Editor-Thema`, `the Editor topic`). A committed
  record of what the tree said then; rewriting it would falsify the record.
- `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md` -
  the plan doc (`invalid until filled, announced by validation`,
  `legal down to zero rules per 4.5`), which quoted the old spec text as the
  byte-faithful transcription mandate. All four tasks complete; plan closing.
  See HARVEST H1.

## 3. h1s untouched

- `git diff -U0 a6548ed 406e91b` over the whole pathset yields **no `^[+-]#` line**:
  no heading added or removed anywhere.
- Stronger check: every `^#` line is byte-identical pre/post in each of the five
  files (4, 5, 4, 5, 31 heading lines respectively).
- Current help h1s: `# Vorschlagskarte`, `# Regeln (Spuren)`, `# Suggestion card`,
  `# Rules (tracks)`.

## 4. Gates, re-run in the correct order (foreground, build first)

| gate | result |
|---|---|
| `pnpm build` | **green** (`vue-tsc --noEmit && vite build`), bundle `dist/assets/index-zXViCkWE.js` |
| `pnpm test:e2e` | **62 passed**, 0 failed |
| `pnpm check:i18n` | **ok** - 41 source files, 211 catalog ids, 19 IpcError codes gated, 22 help ids x 2 help locales, 0 unused warnings, parity ok against 7 `en/` catalogs |

The build produced the *same* bundle hash as the `dist/` already on disk (14:48),
confirming the implementer did rebuild after the edits; the pre-run `dist/` carried
the new wording (2 hits) and the old wording 0 times.

**The stale-dist finding verified at the mechanism, not taken on report.**
`package.json`'s `test:e2e` is
`tsc --noEmit -p e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && vite build --config e2e/vite.mount.config.ts && playwright test`:
it builds only the two e2e harness bundles, never `dist/`. `playwright.config.ts:30`
serves `vite preview --host 127.0.0.1 --port 4173 --strictPort`, and :8-9 carry the
precondition the report quoted, verbatim: `` Precondition: `dist/` must already exist ``
/ `` (`pnpm build`), which the per-commit gate runs before `pnpm test:e2e`. ``
Already ledgered as `e2e-preview-needs-fresh-dist` (tier 1, `docs/decision-ledger.yaml:4309`).
The build-first order is the correct gate form; a standalone `test:e2e` after any
`help/**.md` edit is not a valid gate.

## 5. The doubled closing paren at the NoTrackRules clause - NOT flagged

Syntactically coherent nested prose parens. Paren walk over `spec:375`:

- offset 39 `(` -> depth 1, the pre-existing `track-rule grid (order, source, ...)` parenthetical
- offset 315 `(` -> depth 2, the ruled gloss `(per 4.5: ...)`
- offset 379 `)` closes the gloss (depth 1)
- offset 380 `)` closes the grid parenthetical (depth 0)

Final depth 0, max nesting 2, backtick count even (4). The sentence resumes
correctly after the `))` with `, detail editor per rule, panels for ...`, which is
the next item in the Profile-editor component list - the grammar of the enclosing
enumeration is intact. Dense, but unambiguous and not garbled. Per the remit
(flag only if it actually garbles the sentence): **no flag.**

## 6. Factual accuracy of the two sharpened claims

Both ruled edits *add* assertions rather than only rephrasing, so each was checked
against its anchor. Both are correct, and each moves the text toward measured truth:

- **"announced by a validation warning"** - D65
  (`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`)
  records the config-time diagnostic as `EmptyMatchExpression` at **warning**
  severity at the measured code level, and the design's own preamble states that
  "invalid-until-filled" is warning, not error. The prior "invalid ... announced by
  validation" was the looser text; the ruling corrects it.
- **"in its detail panel" / "im Detailpanel"** - D65: "The marker lands at the Match
  section legend inside the auto-opened detail panel (D67) ... the grid row marker
  (bare `` `tracks[{i}]` ``) deliberately does not fire". The new location claim is
  accurate and resolves T3 LOW-2 precisely: the old locationless sentence invited
  the grid-row reading that D65 deliberately excludes.
- **"per 4.5: `unmatched: keep` = passthrough, `drop` = NoTrackRules"** - faithful to
  spec:180: "`rules` may be empty when `unmatched: keep`: that is a legal
  pure-passthrough remux ... Empty rules under `drop` remain a `NoTrackRules` error."

The one ruled string recorded verbatim in the ruling line (`progress.md:29`),
`incomplete until filled, announced by a validation warning`, byte-matches the
spec exactly (count 1).

## 7. de register, house form, typography

- **Register.** `die Schaltfläche bleibt gesperrt, solange keine Zeile ausgewählt ist`
  matches the established house pattern verbatim in shape: `view-batch.md:20`
  "Starten bleibt gesperrt, solange ...", `view-editor.md:11` "Speichern ist
  gesperrt, solange ...". Terminology consistent (`gesperrt`, not `deaktiviert`).
  du-form consistent with the rest of the corpus.
- **Cross-reference form.** `siehe das Thema Editor-Ansicht` conforms to the dominant
  de form `siehe das Thema <Label>` and byte-matches the target h1 `# Editor-Ansicht`
  (`help/de/view-editor.md`). en `the Editor view topic` byte-matches `# Editor view`.
  The stated rationale (name the topic as it is actually titled) holds in both locales.
- **Typography.** 0 hits across all 7 added lines for em/en dash, figure dash,
  horizontal bar, U+2212, smart quotes, ellipsis, NBSP. Positive control: the same
  pattern against a fixture seeding each forbidden glyph fired on all 8 bad lines and
  correctly skipped the clean hyphen/straight-quote/three-dot line. No trailing
  whitespace, no CR. German diacritics intact (`Schaltfläche`, `ausgewählt`,
  `Bestätigung`, `Änderung`); no transliterated forms.

Two observations, both wording taste and therefore outside the remit, recorded only
so nobody mistakes them for oversights: (a) the de/en pair differs in determiner -
en "in **its** detail panel" (possessive) vs de "im Detailpanel" (anaphoric definite
after "öffnet ihr Detailpanel"); the German definite is the idiomatic choice here and
"in ihrem Detailpanel" would be heavier. (b) Both locales now repeat a noun within or
across adjacent sentences ("Die Schaltfläche Entfernen ... die Schaltfläche bleibt
gesperrt"; "opens its detail panel. A warning in its detail panel"). That repetition
*is* the ruled disambiguation - naming the subject was the point.

## 8. Non-breakage of the surrounding state

- Follow-up commit `32525dc` touches only `docs/decision-ledger.yaml` and
  `docs/process-conventions.yaml` - **no overlap** with the five reviewed files.
- Worktree is identical to `406e91b` for all five files; `git status --porcelain` clean.
- No locale-catalog or help-id parity break (`check:i18n` parity ok); the edits are
  pure Markdown body text, no help id, anchor, or heading involved.

---

## HARVEST

- **H1 - the plan doc's quoted mandate now diverges from the spec it mandated.**
  `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md` still quotes
  `invalid until filled, announced by validation` and `legal down to zero rules per 4.5`
  as the byte-faithful transcription target. Correct to leave untouched (it records
  what was mandated), but it is now the only live-ish document carrying the superseded
  text. Worth one line of disposition at plan close, so a future sweep does not read
  it as an unfinished replacement.
- **H2 - the de cross-ref outlier family is only partially normalized.** The ruling
  covered the 4 Editor-topic sites. The sibling outlier `das Thema zur Vorschlagskarte`
  remains at 2 sites (`help/de/view-editor.md`, `help/de/view-batch.md`) where en uses
  the bare-label form `see the suggestion card topic` (2 sites). `task-3-verdict:84`
  named this as part of the same tree-wide de outlier family. Not in the seven, not a
  defect here; a residual owner-pass candidate.
- **H3 - companion to the already-ledgered `grep -I` exclusion: a *provenance*
  exclusion.** A tree-wide "old string at 0" check can never pass while the SDD ledger,
  archived review diffs, and the mandating plan doc legitimately quote pre-edit text.
  The sound form of the check scopes to content surfaces (`git ls-files | grep -v
  '^\.superpowers/sdd/'`, and treat `docs/process-journal/artifacts/**/*.diff` and the
  plan docs as frozen), then verifies each zero by fire-checking the pattern against
  the pre-edit blob at BASE. Without that scoping the reviewer either reports false
  positives or, worse, loosens the pattern until it reads 0.
- **H4 - review-package diffs are written at -U10.** Comparing a package against
  plain `git diff` reports differences that are only context width. Compare at
  `-U10`, or compare the `+`/`-` lines alone. Cheap trap, easy to misread as tampering.

---

**Verdict: APPROVED.** Seven ruled replacements, five files, verbatim, nothing else.
Old strings gone from every content surface with the zeros fire-checked against the
pre-edit blobs; new strings at exactly the expected counts; h1s and all other headings
byte-identical; `pnpm build` -> `pnpm test:e2e` (62 passed) -> `pnpm check:i18n` all
green in the correct order; the doubled paren is well-formed nested prose and does not
garble the sentence; de register, house cross-reference form, and typography conform.
The two added factual assertions (warning severity, detail-panel location) are both
confirmed against D65. No fixes required.
