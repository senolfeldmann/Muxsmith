# Task 8 review verdict: help-topic content (view-batch, view-jobs, view-editor, batch-suggestion-card, en+de)

> **Round 2 (fix review, commit 77f0525): APPROVED.** Disposition at the end of this file; the round-1 body below is preserved unchanged.

Reviewer: independent SDD task reviewer (fresh session). Commit under review: d76145f on plan7-c (`/home/senol/Git/Muxsmith/.worktrees/plan7-c`); worktree diverges from master by exactly the 8 help files, so the code beside them is the real tree. Ground truth read: plan Task 8 + shared Tasks-8-10 rules (plan lines 790-820), D51/D54, spec 8.3 (v1 design :382-393), Tier-2 house files (docs/conventions.yaml, docs/product-boundaries.yaml), and the code cited below.

## VERDICT: NEEDS FIXES

One Major content-accuracy defect (finding 1) in both locales of batch-suggestion-card; everything else passes. The fix is two sentences across two files plus the Minor in the same files.

## Findings

### 1. MAJOR - batch-suggestion-card misdescribes the YAML fragment as the profile's future content (both locales)

`help/en/batch-suggestion-card.md:3`: "shows the proposed rule fragment as YAML, exactly the text that would land in the profile". `help/de/batch-suggestion-card.md:3`: "zeigt das vorgeschlagene Regel-Fragment als YAML, exakt der Text, der im Profil landen würde".

Both are false, verified against core:

- `yaml_fragment` (`crates/muxsmith-core/src/planner.rs:2173-2177`) renders `"# tracks[{ri}] - add:\n"` + the serialized **MatchExpr delta** - a comment header plus the *additional constraint*, not a rule and not file content. The comment line never lands anywhere (canonical save writes no comments, `profile/save.rs:1-7`).
- Apply merges that delta into the rule's **existing** match with insert-if-absent semantics (`with_rule_match`, `planner.rs:1864-1884`); the file then gets the canonical serialization of the *merged* match. Unless the rule's match was empty, the fragment body does not appear verbatim either.
- The house ledger's own characterization (`docs/conventions.yaml:216`, bug D entry): the fragment is "the YAML the CLI prints verbatim" - a display/clipboard artifact, not profile text.

User impact: the topic teaches Copy users to treat the fragment as paste-ready profile content / a complete replacement, when it is a delta to merge (the fragment's own `add:` comment says so - the help contradicts the artifact it explains). Fix direction: "shows the match constraint the suggestion would add to that rule, as YAML - the same text the CLI prints for this suggestion." Exact wording free (rides the owner pass), the factual claim must change now.

### 2. MINOR - "converges on a resolved profile" overstates core-33 (both locales)

`help/en/batch-suggestion-card.md:13`: "Applying suggestions repeatedly therefore converges on a resolved profile instead of oscillating." `help/de/batch-suggestion-card.md:13`: "läuft deshalb auf ein aufgelöstes Profil zu, statt zu pendeln."

core-33 (`docs/process-journal/artifacts/house-backfill-sdd/cluster-core.md:316`) guarantees **termination of iterated apply** ("narrow-only guarantees convergence (iterated apply terminates)") - not arrival at a resolved profile. Conflicts in the no-single-fix partition get a `SuggestionPartition` info diagnostic and **no** suggestion (`planner.rs:1596-1670`; spec 5.3/D6), and a delta whose key the rule already constrains errors with `EditChangedNothing` (`planner.rs:1908-1915`). The terminal state of repeated apply can still carry conflicts requiring manual edits. "Instead of oscillating" is exactly right; "on a resolved profile" promises more than the guarantee. Fix: keep the monotone-narrowing/termination claim, drop the resolved-profile promise.

### 3. NIT - de names the live-log pane "Live-Protokoll" against the catalog's "Live-Job-Ausgabe"

`help/de/view-jobs.md:17` heading "## Live-Protokoll"; the pane's only catalog name is `jobs-log-region-label = Live-Job-Ausgabe` (`locales/de/gui-jobs.ftl`). Not blocking (the pane has no visible title; "Protokoll" is the catalog's standing word for log elsewhere in the same file), but it is a surface-naming divergence - routed to the owner pass via HARVEST.

## Content accuracy - everything else verified against the tree

- **Cancel semantics** (re-verified myself): batch cancel = stop dequeuing + kill every in-flight job via registered Killers; never-dequeued specs get Cancelled outcomes (`executor/queue.rs:160-230`). Per-job cancel = synchronous kill if in flight, caught at dequeue by the pre-spawn check if still queued; finished outcomes untouched. Topic text matches exactly, including "skipped when its turn comes" and "finished jobs keep their result".
- **No-auto-refresh** (re-verified myself): `BatchView.vue:194-206` documents it explicitly - no post-apply validation or re-plan, auto-refresh a deferred ROADMAP candidate; core-03 = applied edit survives the *next* dry run. Topic's "report not refreshed / change survives the next dry run" is correct in both locales.
- **Apply round trip**: load fresh from disk, apply, save immediately, no confirmation step (`BatchView.vue:207-256`). Canonical-save consequences per D41 (`profile/save.rs`: comment/order/format loss by design, defaults omitted via `skip_serializing_if` in `profile/model.rs`, format picked by extension - .json stays JSON, else YAML). All match.
- **Suggestion origins**: exactly `AmbiguousRule` + `OverlappingRules` (`planner.rs:1366-1460`); config-path form `tracks[{ri}].match` matches the topic's example; card shows path + fragment, Copy -> clipboard (`SuggestionCard.vue`).
- **view-batch**: validate-on-select (`selectProfile` -> `runValidate`); dir memory per profile with restore; empty-field = profile's own directory (matches `batch-source-hint`/`batch-output-hint` verbatim); resolution table one-per-file, rows in rule order (`planner.rs:546` enumerate), `-` placeholder, `id (kind)` label, no-plan message + diagnostics (`ResolutionTable.vue`); severity counts across config+batch+files; the four Run-gate reasons and reason-naming tooltip (`runDisabledReason`, `BatchView.vue:291-307`); Run plans internally, dry run not required (spec 5.5, comment :284-290).
- **view-jobs**: queued -> running -> 4 terminal states; en topic's "done, done with warnings, failed, cancelled" and de's "fertig, fertig mit Warnungen, ..." match the state chips verbatim (`gui-jobs.ftl` en/de); header finished/total, summary ok/warning/failed/cancelled; live log DOM-capped (5000) with per-job filter while the full per-job log is persisted (`JobsView.vue:52-55`, `LiveLog.vue`); history from disk with start time, view/copy/save per job (`RunHistory.vue`); joblog-note semantics (unavailable -> run absent from history, incomplete -> job records missing) match the two catalog messages. "This view takes over the moment one starts": `App.vue:38-39` switches `activeView` to jobs on run dispatch.
- **view-editor**: model-not-text editing; grid summary columns = the catalog's source/match/optional/changes labels; rule order = output track order (D54 row 14); drag reorder (`onDrop`) and row-select detail panel (Task 13b); shallow-watch background revalidation of the whole model on every edit with stale-response guard; save gate = error-severity diagnostics only, matching the code's own "one sanctioned frontend affordance" doc (`EditorView.vue:140-145`); section list (metadata, input, output, attachments, chapters, tags, title) matches `editor-profile-*` labels in both locales; Apply-saves-the-same-way cross-reference correct.

## Rule conformance

- **File set**: exactly the 8 files of D54/D51's 4 ids x 2 locales; pair listing shows every name at count 2; nothing else under `help/`. Commit touches only these 8.
- **h1 opener naming the surface**: all 8 (Batch view/Stapel-Ansicht, Jobs view/Jobs-Ansicht, Editor view/Editor-Ansicht, Suggestion card/Vorschlagskarte).
- **Markdown subset** (controller-ratified provisional): headings, paragraphs, lists, bold, inline code only; no fences, tables, raw HTML, or autolinks (grep + full read).
- **No URLs** (design-derived, D62 check 4): grep clean; cross-topic references are prose ("see the suggestion card topic") as required.
- **Length band 1-3 kB** (controller-ratified provisional): en 1415/1934/1520/1462, de 1619/2221/1796/1782 bytes - all in band.
- **D54 justification coverage**: complete per id (workflow+table+diagnostics; lifecycle+cancel+history/export; model+save+validate-on-edit; apply-saves-to-disk+core-33+no-auto-refresh).
- **en+de land together per topic**: same commit.

## de quality

Independent prose, not literal translation (reordered syntax, idiomatic constructions: "erledigt den ganzen Weg mit einem Klick", "Bearbeitet wird das aus der Datei geparste Modell"); du-imperative register matching the catalogs' message register; config keywords literal (`optional`, "Match" as the catalog's established loanword); straight ASCII quotes per the catalog convention; real umlauts/ß throughout, zero banned glyphs (checked, fire-verified). Terminology catalog-anchored where a catalog term exists: Stapel-Ansicht and Jobs-Ansicht are literally attested in catalog prose (`jobs-no-run`, `batch-run-tooltip-run-active`); Probelauf, Meldungen, Starten, Aufgelöste Spur, Stapel abbrechen, Lauf-Verlauf, In Warteschlange, and all four state chips match the catalogs verbatim. The de defect content is shared with en (findings 1-2), i.e. translation-independent.

## house dimension

No deviation from recorded Tier-2 conventions; nothing in `conventions.yaml`/`product-boundaries.yaml` constrains help prose beyond what the plan transcribed. The topics' claims align with the ledger entries they lean on (core-33, core-03, the bug-D fragment entry - which finding 1 shows the en/de intro contradicting). New patterns flagged for harvest below.

## Q1 adjudication: the three coined de surface names

**Ruling: acceptable as drafts riding the owner surface pass - do not rework now.** Condition: the coinages are enumerated as explicit owner-pass input (HARVEST below), so they are ruled on, not silently ratified.

For reworking now: help topics are rendered surfaces; a name coined here can drift from whatever the owner pass settles, and catalog-derivable phrasing would avoid coining at all. Against: (a) D54 itself routes topic wording through the owner's plan-close rendered-surface pass (latitude-carveout occurrence 2026-07-16), and the commit message declares exactly that - this is the sanctioned channel for these names; (b) no conflicting catalog term exists to violate - neither locale's catalog names the card, the table, or the editor view as a surface, so there is no inconsistency today, only forward risk the pass exists to absorb; (c) two of the three are pattern-derivable anyway: Editor-Ansicht = `nav-editor` ("Editor") + "-Ansicht" with both siblings (Stapel-Ansicht, Jobs-Ansicht) literally attested in catalog prose; Vorschlagskarte composes the catalog's "Vorschlag" with the en id's own "card"; (d) Auflösungstabelle mirrors D54's own en term "resolution table" (design-precedented) and the catalog's "Aufgelöste Spur" - the derivable alternative ("die Tabelle der aufgelösten Spuren") is worse prose for a heading that must name the thing repeatedly. Reworking now would trade a natural compound for clunkier phrasing to avoid a review the names get anyway.

## Q2 adjudication: the version-control recommendation in view-editor

**Ruling: in scope - keep.** (Wording rides the owner pass like all topic prose.)

For stripping: D54's justification column is "the topic's required content" and names mechanics only; the standing surface note (`editor-save-note`) is deliberately declarative, not advisory; help that accumulates advice bloats and stales. For keeping: spec 8.3 defines the topic depth as "what it does, **when to use it**, interactions with other settings" - user-directed guidance is part of the charter, not an overreach; the shared content rule says topics *cover* the D54 line (a floor), not that they are limited to it; and the sentence is the minimal operational consequence of the very semantics D54 *requires* the topic to explain - D41 makes comment loss by-design and unpreventable, so honest documentation of a deliberately lossy operation includes what a user who cares about the loss should do. It makes no product claim that can rot, names no external tool specifically enough to stale, and contains no URL. The out-of-scope reading would leave the topic stating "your comments will be destroyed" while withholding the one-line mitigation - worse help, no gain.

## Fire-verification of absence checks

All three of my absence checks were fired against a known-present control (scratchpad file, worktree untouched): the URL grep (`http://\|https://`), the banned-glyph grep (PCRE class incl. em/en-dash, curly quotes, ellipsis, nbsp), and the raw-HTML grep (`<[a-zA-Z]`) each produced the expected hit on the control and clean output on `help/`.

## HARVEST

- **Pattern (positive, for Tasks 9/10 review)**: the implementer authored against code-comment and ledger anchors (BatchView's apply doc, save.rs rustdoc, core-33/core-03, catalog hint texts) - most claims trace to a written source, which is why only the two fragment/convergence sentences (the ones *extrapolating past* their source) are wrong. Reviewing Tasks 9/10, concentrate on sentences that go beyond the cited anchor.
- **de terminology for the owner pass** (drafts, per Q1): `Vorschlagskarte`, `Auflösungstabelle`, `Editor-Ansicht` (coined, no catalog precedent; derivations above); `Live-Protokoll` as pane name vs catalog `Live-Job-Ausgabe` (finding 3); view-editor de calls the rule grid "Regel-Tabelle"/"Tabelle" - the catalog names the field only "Regeln" (`editor-tracks-rules`), so the grid-as-surface name is also a de coinage; en "resolution table" is design-precedented (D54 wording), so only the de side needs a ruling.
- **Over-restriction watch (provisional content rules)**: no quality loss observed from either provisional rule. The 1-3 kB band left headroom (files landed 1.4-2.2 kB with full D54 coverage plus useful extra sections - Run-gate, live log); the markdown subset forced nothing awkward - the one place a table might have tempted (resolution-table explanation) reads fine as prose. No veto-relevant friction to report.
- **Observation for the fix loop**: findings 1 and 2 live in the same file pair (`batch-suggestion-card.md` en+de, lines 3 and 13); one fix pass over two files closes the verdict.

---

## Round 2: fix review of commit 77f0525 (same reviewer)

Delta reviewed: `git show 77f0525` - exactly the batch-suggestion-card pair, 2 lines per file; d76145f untouched.

### FINAL VERDICT: APPROVED

**Finding 1 - VERIFIED FIXED.** The rewritten intro (en:3 / de:3) now states: the fragment is the match constraint the suggestion would *add*, the same text the CLI prints, with a leading comment line marking it as an addition; a preview of the addition, not the rule's future content; Apply *merges* it into the rule's existing match. Every clause re-verified on the artifact: `yaml_fragment` serializes the MatchExpr delta behind a `# tracks[N] - add:` comment header (`planner.rs:2173-2177`); `with_rule_match` merges insert-if-absent into the existing match (`planner.rs:1864-1884`); and the "same text the CLI prints" claim is empirically true - `crates/muxsmith-cli/src/commands/mod.rs:125-126` prints the `dry-run-suggestion` header then `s.yaml_fragment` verbatim, comment line included. The de counterpart is a faithful, independently phrased equivalent ("Match-Bedingung ... hinzufügen würde", "Vorschau dieser Ergänzung, nicht der künftige Inhalt der Regel", "ergänzt den bestehenden Match ... um die Bedingung") with no added or lost claim.

**Finding 2 - VERIFIED FIXED.** The convergence sentence (en:13 / de:13) now claims termination only ("guaranteed to terminate instead of oscillating" / "endet deshalb garantiert, statt zu pendeln") - exactly core-33's guarantee - and the new residual clause ("conflicts the report offers no suggestion for remain and need a manual edit" / de equivalent) matches the no-single-fix partition behavior (`SuggestionPartition`, `planner.rs:1596-1670`): such conflicts get no suggestion and require manual editing. No overpromise remains.

**No regression.** Only the two paragraphs changed; h1 openers and section structure untouched; markdown subset holds (new text is plain prose, no new constructs); URL and banned-glyph greps clean on the pair (patterns fire-verified in round 1 this session); sizes 1709 (en) / 1983 (de) bytes - still inside the 1-3 kB band; locales consistent with each other.

**Finding 3 (nit) - correctly untouched.** The commit does not touch view-jobs; the "Live-Protokoll" naming stays owner-pass-routed per the HARVEST list, as intended.

**Implementer's flag on the Copy sentence - CONFIRMED CONSISTENT.** The unchanged Copy sentence ("puts the YAML fragment on the clipboard so you can paste it into the profile yourself" / "damit du es selbst in das Profil einfügen kannst") is now framed by the corrected intro: the fragment is established as an addition to merge, and the fragment's own comment line says `add:`, so "paste it yourself" reads as manually incorporating the addition - no false claim. The owner pass may still polish "paste" toward "add/merge it into the rule" for extra precision; that is wording, not a defect.
