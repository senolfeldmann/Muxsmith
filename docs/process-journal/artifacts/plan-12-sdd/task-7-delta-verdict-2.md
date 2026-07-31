# Task 7 delta verdict: fix round 2 (`43c1a44..87a07e8`) - closing round

Same reviewer, delta only. Base `43c1a44` is the controller's ledger commit; nothing in it is
graded. The graded diff is `87a07e8`, one line in `help/de/view-editor.md`. The report's appended
"Fix round 2" section (`task-7-report.md:314-452`) is graded against it, and against the corpus for
the sweep and the two withheld findings.

## Verdict on the two items

**The word (Zielort -> Speicherort): addressed, confirmed correct.** `help/de/view-editor.md:23` now
reads "...öffnet der erste Speichervorgang einen Dialog, der nach dem Speicherort fragt..."
(`git diff --stat 43c1a44..87a07e8`: one file, one line, matching the commit). "Speicherort"
re-verified independently at both its established sites, not taken from either the coordinator's or
the report's citation:

```
$ command grep -n "Speicherort" locales/de/gui-common.ftl locales/de/gui-jobs.ftl
locales/de/gui-common.ftl:42:settings-dir-unavailable = Der Speicherort der Anwendungseinstellungen konnte auf diesem System nicht ermittelt werden.
locales/de/gui-jobs.ftl:71:job-log-unavailable = Der Speicherort des Lauf-Protokolls konnte auf diesem System nicht ermittelt werden.
```

**The sweep: substantively adequate and closes the completeness gap round 1 left open, but its own
"every noun, content verb and content adjective is listed" claim still slightly overstates what the
table shows.** Verdict below is a set comparison, not a re-reading of the report's conclusion.

## Set comparison: my derivation vs. the report's table

Derived independently from `git diff 78e968f..7b403e8 -- help/de/view-editor.md help/en/view-editor.md`
(the original addition, not the report's list, not memory of this conversation), reading only the
`+` lines and the net-new clauses inside changed lines, excluding articles/pronouns/prepositions/
conjunctions/auxiliaries the same way the report's stated method does.

**Structural gap from round 1 is closed.** Round 1's sweep table contained zero terms from the
`## Speicherverhalten` lead paragraph. This round's table has nine rows sourced from exactly that
paragraph (Festplatte, Pfad, Speichervorgang, Dialog, Speicherort/Zielort, plus the English mirror:
disk, path, dialog, "asking where to put the file") - the paragraph is no longer invisible to the
sweep, which is the specific defect this round exists to close.

**Residual gap, found by independently deriving the full word list and diffing it against the
report's table:** the table omits several genuine content words the diff introduces - German
*hält* ("Der Editor hält jeweils ein Profil"), *aktuell* ("das aktuelle Profil"), *falsch* ("nicht
falsch"), *fragt*, *schreibt/geschrieben* ("write"); English *flags*, *sit*, *respond*, *leaves*,
*touches*, *opens*, *writes*. None of these are function words by the report's own exclusion rule
(articles/pronouns/prepositions/conjunctions/auxiliary verbs); all are content verbs or adjectives
the diff adds and the table does not name.

I checked every one of these myself rather than accept the omission as harmless by assumption:

```
$ command grep -rn "\bhält\b" help/de/*.md locales/de/*.ftl
help/de/view-editor.md:9: ...  (only the line under review)
$ command grep -rn "\baktuell" help/de/*.md locales/de/*.ftl
help/de/view-batch.md:20: ... "dem aktuellen Profil und den aktuellen Verzeichnissen" ...
help/de/view-editor.md:9, :19: ... (:19 is this same file's own PRE-EXISTING, untouched text)
$ command grep -n "\bcurrent\b" help/en/*.md locales/en/gui-batch.ftl
help/en/view-batch.md:20: ... "the current profile and directories" ...
help/en/view-editor.md:9, :19 (:19 pre-existing, untouched)
$ command grep -n "\bflags\b" help/en/*.md
help/en/editor-tracks-rules.md:13: "A warning in its detail panel flags the new rule..."
help/en/view-editor.md:7 (the new text, same construction)
$ command grep -n "\bsit\b\|\brespond" help/en/*.md
help/en/view-editor.md:11 (only the line under review, both words)
```

Result: **zero further defects.** *hält*, *falsch*, *fragt*, *schreibt*, *sit*, *respond*, *leaves*,
*touches*, *opens* are genuine first uses with no established corpus alternative to collide with -
the same "first use, no collision" class the report's table correctly applied to Anwendung,
Aktionsleiste and the rest. *flags* is not merely uncollided, it is a positive match to an existing,
pre-task, unedited precedent (`editor-tracks-rules.md:13`, identical construction: "a warning...
flags..."), which the table's omission cost nothing but the chance to cite one more confirming
site. *aktuell*/*current* looked like the most promising candidate for a hidden synonym defect - a
plausible parallel to `batch-profile-current = Ausgewähltes Profil`/`Selected profile` - but resolves
clean: "aktuell"/"current" is independently established, in exactly this generic "presently active"
sense, at `view-batch.md:20` in both locales and at this same file's own pre-existing (task-7-untouched)
"Validate on edit" sentence (`view-editor.md:19`). It names a different, correctly-matched concept
("the state right now") from `batch-profile-current`'s "which profile is selected in the Batch picker"
- not a second word for the same referent, so not the defect class at all.

**Judgment:** this is not round 1's failure recurring one notch narrower. Round 1's gap was
structural - a whole paragraph never read, so nothing in it could have been caught regardless of
content. Round 2's gap is a handful of generic verbs/adjectives omitted from an otherwise-thorough,
both-files-both-sections table; every omission I traced independently resolved clean, and one
(*flags*) would have strengthened the table's own case had it been included. The sweep is wide
enough to close the task; its "every... is listed" framing is the one place still slightly ahead of
what the table shows, worth a harvest line, not a blocking finding.

## Ruling on the two withheld findings

**Classification confirmed for both**, against the sites named, independently re-derived:

- **anlegen/erstellen (German).** `locales/de/gui-editor.ftl:156`, `editor-empty` (pre-existing,
  untouched by any round of this task): "Kein Profil geöffnet. **Erstelle** eines mit Neues Profil
  oder wähle eine vorhandene Profildatei aus." This is the corpus's own sibling description of the
  identical affordance (the New button creates a blank profile) and it uses "erstellen." The new
  heading matches it ("Ein Profil **erstellen**"), but the body prose describes the same action twice
  with "anlegen" instead (`view-editor.md:7` "legt ... an"; `:9` "durch Anlegen eines neuen"). Repo-wide
  sweep for "anlegen"/"Anlegen"/"legt...an" outside this file finds one hit, in an unrelated concept
  (`editor-output-filename.md:10`, subdirectory creation) - confirming "anlegen" is not itself an
  established term for THIS concept anywhere else either; the only established word for this specific
  affordance is "erstellen," at `editor-empty`. Classification: **confirmed** - synonym direction, not
  a collision (no competing referent for "anlegen" itself), and correctly distinguished from I-2's
  shape.
- **Start/Creating (English).** `locales/en/gui-editor.ftl:152`, `editor-empty` (same site, same
  status): "No profile open. **Create** one with New profile, or choose an existing profile file."
  The new heading matches ("**Creating** a profile"), and the body's second paragraph matches
  ("creating another"), but the opening sentence ("**Start** a new profile...") and the first body
  sentence's verb ("New **starts** a profile...") use a different verb for the identical action.
  Repo-wide sweep for "Start"/"starts" against profile-creation elsewhere in `help/en/*.md` finds
  nothing - "Start" is not an established alternative name for this affordance anywhere else either.
  Classification: **confirmed**, same shape as the German instance, correctly its mirror rather than
  a separate finding.

**No further instance of the synonym direction found.** Beyond the two named, I specifically
stress-tested the candidates most likely to hide a third instance - *aktuell*/*current* against
*Ausgewählt*/*Selected* (resolved clean, different referent, see above), *Speichern*/*schreiben*
against the pre-existing *neu schreiben* / *zurückschreiben* wording two sentences below (resolved
clean: "write" is the unavoidable generic verb for "data reaches disk," and the pre-existing
"rewrites"/"written back" describe a different, complementary claim - the full-canonical-rewrite
mechanism - not a competing name for the same fact), and every "first use, no collision" row in both
the report's table and my own extension of it (above). None produced a third instance. **The wave's
list of two is complete**, as far as this task's addition goes.

## New breakage introduced by this fix diff

**None.** Re-verified in place: typography clean (re-ran the sweep after catching and fixing a bug in
my own first pass - my denylist's "nbsp" entry was accidentally a plain space rather than U+00A0,
which flagged 412/404 false hits; corrected and re-run came back clean on both files, consistent with
every prior round), D62 absence check H1 clean (`grep -nE 'https?://|\||</?[a-zA-Z]'`, exit 1, no
output), `pnpm check:i18n` unchanged (`22 help id(s) x 2 help locale(s)`), `pnpm build` then 18 e2e
tests covering help-topic rendering and every New-profile scenario, all green, including
`e2e/help-mode.spec.ts:286`'s byte-level `topicMarkup("view-editor")` comparison against the live
file (the one test that actually renders this paragraph's changed word). `git diff --stat` and the
commit message match the report exactly.

## Findings summary (this delta)

- The word: addressed
- The sweep: adequate to close (harvest note on its completeness framing, not a blocking finding)
- New Critical/Important breakage in this diff: 0
- Withheld findings (anlegen/erstellen, Start/Creating): classification confirmed for both, correctly
  routed to the whole-branch review rather than fixed here; no third instance found

## Task 7: closed.

Every requirement in the original brief is MET (`task-7-verdict.md`); all four round-1 findings
addressed (`task-7-delta-verdict.md`); this round's one-word fix is correct and its widened sweep,
while not a literal every-word audit, independently re-derives to zero further defects. The two
synonym-direction findings and the pre-existing `batch-profile-none` coverage gap are real, correctly
NOT fixed under this task's scope, and correctly routed onward - none of them is shipped-content
breakage, all three are already the whole-branch review's stated work.

## Harvest

- **A "checked every noun/verb/adjective" claim is itself an enumeration claim and needs the same
  discipline as any other:** re-derive the actual word list from the artifact (here, the diff) rather
  than accept the claim's own table as the full set. This round's table is far better than round 1's
  (both files, both sections, real corpus citations per row) but still under-lists by a handful of
  generic verbs - harmless this time because none collided, but "harmless this time" is exactly the
  property that makes the next occurrence of this gap dangerous, since nothing about the method
  distinguishes a safe omission from an unsafe one in advance.
- **A promising synonym candidate (aktuell/current vs. Ausgewählt/Selected) is worth naming even when
  it resolves clean**, because the resolution itself is informative: two terms can look like the same
  referent from vocabulary alone and turn out to name genuinely different concepts once you check
  what each site is actually describing. The `a-new-term-is-checked-against-the-corpus-in-both-
  directions` rule should be read as "check whether an established synonym exists for the same
  referent," not merely "check whether the near-synonym word appears elsewhere" - the second, cheaper
  test would have false-flagged this one.
