# Task 7 delta verdict: fix round 1 (`c570b4c..411f220`)

Same reviewer, delta only. Base `c570b4c` is the controller's own ledger commit; nothing in that
commit is graded. The graded diff is `411f220`, two lines across `help/en/view-editor.md` and
`help/de/view-editor.md`; the report's appended "Fix round 1" section (`task-7-report.md:168-307`)
is graded against it and, for item 3 and item 4, against artifacts the diff itself does not touch.

## Per-finding verdicts

**I-1 (missing `batch-profile-none` test coverage) - addressed, as scoped.** Not this implementer's
to build (coordinator's routing to the whole-branch review's fix wave, confirmed correct: the gap is
an acceptance-map row naming a producer that does not exist, a plan defect, not a Files-list gap this
task could close). What was owed here - marking the report's false sentence in place - is done:
`task-7-report.md:76-80` now reads the original claim struck through
(`~~and the batch-view specs that assert `batch-profile-none` through `en(id)`~~`) followed by
**`[WRONG, see fix-round-1 item 4 below: no test file anywhere asserts this string; the claim was
inherited from the brief's own premise and not checked against the test tree before being repeated
here]`**, plus a matching correction appended to section 5's self-audit (`task-7-report.md:161-166`).
Marked in place, not deleted, not left standing. No test file touched (`git diff --stat` for `411f220`
lists only the two `help/` files); correctly so, per the coordinator's own framing.

**I-2 (German "Dateiendung"/"Erweiterung" collision) - addressed, with a concern about the sweep's
completeness, not about the fix.** The flagged substitution is correct and independently confirmed at
every cited site: `locales/de/gui-editor.ftl:43` (`editor-input-extensions = Erweiterungen`), `:84`
(`editor-locator-extensions = Erweiterungen`), `locales/de/diagnostics.ftl:17`
(`empty-extensions = Die Erweiterungsliste darf nicht leer sein.`), `:50` (`unknown-extension = Die
Erweiterung "..." ist keine der von mkvmerge unterstützten Erweiterungen...`). "Erweiterung" is
genuinely the corpus's established term for the `input.extensions` concept at four independent sites,
not merely a word the corpus happens to use somewhere else. See the completeness finding below - this
does not change the "addressed" verdict on the substitution itself, but it does mean the fix report's
accompanying rigor claim overstates what was actually checked.

**M-1 (English "New" paraphrase) - addressed.** `help/en/view-editor.md:7` now reads "The New profile
button starts a profile...", containing the catalog value `editor-action-new = New profile`
(`locales/en/gui-editor.ftl:151`) byte-for-byte. The claimed convention is real, not invented: verified
directly at `help/en/editor-tracks-rules.md:13` ("The Add button appends a new empty rule...") and
`:15` ("The Remove button deletes the selected rule..."), both matching the cited lines exactly.

**M-2 (report citation swapped for a doc comment) - addressed.** The new citations are real and assert
what the claim needs, checked directly rather than trusted: `crates/muxsmith-core/src/profile/validate.rs:79`
(`if rule.match_expr.is_empty()`) through the `Diagnostic::warning(DiagCode::EmptyMatchExpression, ...)`
push inside it, and `crates/muxsmith-core/tests/validate_semantics.rs:148-156`
(`empty_match_expression_is_warning`, built from `profile("  - match: {}")` - the seed's own shape -
asserting `d.severity == Severity::Warning`). Reproduced independently, not copied from the report's
paste:

```
$ cargo test --package muxsmith-core --test validate_semantics empty_match_expression_is_warning -- --exact --nocapture
test empty_match_expression_is_warning ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 23 filtered out; finished in 0.00s
```

Byte-identical to the report's pasted result. Not a citation-for-citation swap of the same defect
class: the new citation is measurably tighter than the verdict's own two (a real mechanism plus a
unit test purpose-built for this exact claim, versus a diagnostic-code string table and a
hand-written marker fixture), and the report says so rather than claiming parity it didn't check.

## The German term sweep's completeness

**Incomplete, and the gap is exactly the shape the coordinator named.** The report's sweep
(`task-7-report.md:192-220`) lists six terms/term-groups it checked beyond the flagged word - Ansicht,
Warnung/Fehler, Rückgängig/Wiederholen, Anwendung, Aktionsleiste/Tastenkürzel, Strg/Umschalt - and
every one of them comes from the three "Ein Profil erstellen" paragraphs, i.e. the section the
original finding was located in. Deriving the German addition's content words from the diff itself
(the original task-7 diff, `78e968f..7b403e8`, not the fix report's own list) rather than from what
the report chose to name: the addition also includes the `## Speicherverhalten` lead paragraph -
"Bis zum Speichern wird nichts auf die Festplatte geschrieben. Hat das Profil noch keinen Pfad, öffnet
der erste Speichervorgang einen Dialog, der nach dem Zielort fragt; jeder weitere Speichervorgang für
dieses Profil schreibt danach direkt dorthin, ohne erneuten Dialog." - and not one content word from
that paragraph (Festplatte, Pfad, Speichervorgang, Dialog, Zielort) appears anywhere in the sweep's
list, despite the sweep's own framing ("the rest of the German addition was checked... term by term")
claiming exactly that coverage.

Checking the five omitted terms myself against the corpus (`command grep -rn` over `help/de/*.md` and
`locales/de/*.ftl`):

- **Festplatte**: matches this same file's own pre-existing, untouched text 18 lines below
  ("...der Datei auf der Festplatte bleiben nicht erhalten") - consistent, no defect.
- **Pfad**: matches established corpus usage throughout (`editor-locator-path = Pfad`, six further
  hits across `help/de/editor-*.md` and `locales/de/*.ftl`) - consistent, no defect.
- **Speichervorgang**: zero hits anywhere else in `help/de/*.md` or `locales/de/*.ftl` - first use, no
  collision, same class as the sweep's own "Anwendung"/"Aktionsleiste" verdicts, just never actually
  run.
- **Dialog**: zero hits anywhere else in `help/de/*.md` or `locales/de/*.ftl` prose - first use, no
  collision.
- **Zielort ("nach dem Zielort fragt") - a real, if lower-severity, instance of the same defect
  class the sweep exists to catch.** The corpus already has an established term for "the location
  something is stored at": `locales/de/gui-common.ftl:42`
  (`settings-dir-unavailable = Der Speicherort der Anwendungseinstellungen konnte...`) and
  `locales/de/gui-jobs.ftl:71` (`job-log-unavailable = Der Speicherort des Lauf-Protokolls konnte...`),
  both using "Speicherort." "Zielort" is a different, uncollided word for a closely related concept
  (where a save dialog will write the file), not a wrong-referent collision like "Dateiendung" was -
  a reasonable native speaker could defend it as the dialog's *destination* versus a *settled*
  storage location - so I am not raising it to the severity of I-2 itself. But it is exactly the kind
  of candidate a genuine term-by-term sweep exists to surface, and this sweep did not reach it because
  it never looked at the paragraph the word is in.

Net judgment: the sweep that shipped is real work, correctly executed on the paragraphs it covered,
and its "first use, no collision" calls (Anwendung, Aktionsleiste, Tastenkürzel) are independently
correct. But its own framing claims a scope ("the German addition") wider than what it actually
walked, and the untouched remainder is not trivial padding - it contains the one place in this whole
task where the corpus's own established vocabulary offers a closer term than the one shipped. This is
the "term list from memory of what you wrote, not from the artifact" failure this project has hit
before, one level lower in severity than the original I-2 catch: it did not hide a wrong-referent
collision this time, but it was never positioned to catch one in the paragraph it skipped, and finding
one there was a matter of which paragraph got read, not diligence within the paragraph that was.

## New breakage introduced by this fix diff

**None.** Both changed lines were re-verified in place: typography clean (no em/en-dash, curly
quotes, ellipsis, NBSP in either file), D62 absence check H1 clean with the standing fire control
unaffected, `pnpm check:i18n` unchanged (`22 help id(s) x 2 help locale(s)`, same as both prior runs),
`pnpm build` then the help-topic and New-profile e2e specs (18 tests, including
`e2e/help-mode.spec.ts:286`'s byte-level `topicMarkup("view-editor")` comparison against the live
file, which is what actually exercises the new "The New profile button..." prose) all green against
the freshly built bundle. Commit `411f220`, pathspec-scoped to the two `help/` files, matches
`git diff --stat`.

## Findings summary (delta)

- I-1: addressed (scope respected)
- I-2: addressed, sweep-completeness concern noted above (harvest, not a new open finding)
- M-1: addressed
- M-2: addressed
- New Critical/Important breakage in this diff: 0

## Harvest

- **A "checked X term by term against the corpus" claim needs the same treatment as any other
  enumeration claim: derive the term list from the diff, never from the section where the original
  finding lived.** This sweep anchored on the paragraph the reviewer's finding pointed at and never
  widened to the rest of the same task's addition, even though the fix report's own sentence claimed
  the wider scope. The mechanical tell for a future reviewer: when a fix report says "swept the rest
  of a multi-paragraph addition," count the paragraphs the addition actually touched (here: the diff
  that introduced the content, not the diff that fixed one word in it) and confirm the swept-term list
  draws from all of them, not just the one the finding named.
- **"An uncollided word" and "the corpus's own closer synonym" are different findings worth naming
  separately.** Zielort/Speicherort is not the Dateiendung shape (a word borrowed from a colliding
  concept nearby) - it is a new word where a corpus-established synonym for essentially the same
  concept already exists elsewhere. Both are real, but the second is milder and more common
  (translation-into-a-house-vocabulary always has near-synonym options); worth its own name if it
  recurs, distinct from I-2's collision shape, so a future finding doesn't over- or under-state it by
  reaching for the nearest existing label.
