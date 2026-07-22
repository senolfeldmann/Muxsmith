# Design review - round 7 (delta review of the round-7 amendment, commit 1fa2972)

Scope: the single-file design amendment rewriting D55 rule 5's select-structure
part from absolute assertions to en-reference parity (T17 NEEDS_CONTEXT,
controller ruling Option B). Settled content outside the amendment was not
re-litigated. Reviewed against: the amendment diff
(`.superpowers/sdd/plan-7/design-amendment-round-7.diff`), the current master
state of `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`, the T17
decision memo (`.superpowers/sdd/plan-7/task-17-report.md`), and the live tree
(`e2e/i18n-en.ts`, `e2e/catalogs.spec.ts`, `locales/`).

## Verdict

**NEEDS FIXES**

One Major finding: the delegation citation `e2e/i18n-en.ts:118-136` is stale on
the tree the amendment was committed to; the function sits at 125-143.
Everything else - encoding fidelity, latitude, sweep, record quality - passes.

---

## Findings by severity

### MAJOR

**F1 - Stale line citation in the amended rule text (design doc line 800).**
The rule delegates absolute Fluent validity to "`assertAllCatalogsParseCleanly`
(`e2e/i18n-en.ts:118-136`, run by `e2e/catalogs.spec.ts:12`)". On the current
tree (HEAD = 1fa2972, clean working tree, file last touched by wave-2 commit
3fab82f) the function spans **lines 125-143**; lines 118-136 start inside
`buildEnBundle` (117-123) and cut off at 136 - excluding lines 139-141, the
standalone `cli.ftl` parse that is precisely the second of the "two bundle
groupings" the same sentence asserts. Provenance verified: at the design's
creation commit f7e1e83 the function did occupy 118-136 (correction #5's
citation, correct when written); the wave-2 merge (d194588 D53 + 3fab82f D55
fold, merged as 7c29957) shifted the file by +7 lines, and the round-7 author
copied the round-1 range into the new block without re-verifying it - inside a
sentence labeled "(controller-verified)". The label is true for the function's
existence, runner, and two-groupings behavior (all re-verified here) and false
for the coordinates. Fix: `e2e/i18n-en.ts:118-136` -> `e2e/i18n-en.ts:125-143`
at design doc line 800.

### MINOR

**F2 - Correction #5 carries the same stale range (design doc line 57).**
Pre-existing staleness (went stale at the wave-2 merge, outside this
amendment's semantic delta), but it is the source F1 was copied from, and the
amendment's sweep explicitly walked correction #5. The fix commit should sweep
it to 125-143 in the same edit; leaving it makes the next copy repeat F1.

### NOTE (no action required)

**N1 - "a select missing its default is a parse error caught there":**
mechanically, `@fluent/bundle`'s `addResource` reports only id collisions (per
`i18n-en.ts`'s own module doc); the missing-default case is caught by
`parseOrThrow`'s dropped-ids layer (line 103-113: the real parser produced no
value and no attributes for a scanned id). The effect claim - the guard catches
it as a hard failure - holds; the wording is tolerable shorthand.

---

## Dimension results

### 1. Faithful encoding - PASS except F1

- (b)/(c)/(d) encode exactly the ruling: select-count parity against en,
  selector-variable parity against en, variant-key equality with the CLDR/
  numeric carve-out reduced to parity (variant presence empty-vs-non-empty +
  `*`-default count vs en). Matches the memo's Option B expression
  (`(sa.keys.length === 0) !== (sb.keys.length === 0) || sa.defaults !== sb.defaults`)
  at the semantic level with nothing left to invent.
- Absolutes removed: "No absolute per-select assertion remains in this rule" is
  true of the rule text; a document-wide grep for "exactly one `*`-default" /
  "at least one variant" finds survivors only inside the delegation sentence
  (correctly re-attributed to the parse guard) and the quoted refuted premise
  in the amendment block.
- Delegation named inline: function name correct, `e2e/catalogs.spec.ts:12`
  correct (verified: that line runs `assertAllCatalogsParseCleanly()`), the
  "every locale x file across the app's two bundle groupings" claim correct
  (verified: gui-* + diagnostics.ftl combined bundle, cli.ftl standalone; all
  7 files x 2 locales covered, no orphan `.ftl` category). The line-range
  citation fails (F1).
- Charter: the rule is phrased as "a pure en-vs-de parity check on the flat
  line-based derivation"; the D55 header's "the parser stays line-based per
  its own charter" (line 763-764) stands unchanged. Within charter.

### 2. Latitude scan - PASS

- Permission-form: none. "Option A/B/C" appear only as historical record of
  rejected/accepted resolutions; no "may choose"/"either works" anywhere in
  the amended text. The doc-header claim "no design-latitude clause in either
  form" survives the amendment.
- Omission-form: the normative enumerations are closed - (b)-(d) are the
  complete select-structure part; the fire-tested drift-class list (placeable
  rename, selector-var rename, select add/drop, non-plural variant-key drift,
  default-count drift) matches the memo's five exactly, no "etc."; the
  dependent-sentence sweep list enumerates concrete anchors. The blind-spot
  "e.g." is illustration inside a descriptive acceptance record whose class is
  exactly defined by complement ("any drift that changes the flat token stream
  still surfaces"); not a mandated check with a hidden target list. The
  numeric-literals "`[0]`, `[1]`, ..." wording is pre-amendment settled
  content, unchanged by the diff, not re-litigated.
- Implementability: the T17 worktree carries the implemented 5b-5d code; the
  amended semantics plus the memo's exact expression (reachable via the
  amendment's authority citation) pin the completion change fully. Nothing to
  invent.

### 3. Sweep completeness - PASS

Independent document walk (grep patterns: rule 5 variants, carve-out,
`*`-default, select-structure, selector-structure, patternStructure, i18n-12,
absolute-assertion remnants, plus a full section-structure pass) found exactly
the author's dependent set and nothing more. Two additional mentions checked
and confirmed non-dependent: line 775 ("coverage comes from rules 3-5" - a
pointer, semantics-independent) and line 244 (section-1 ground truth "the e2e
all-locales real-parse guard exists" - consistent with, indeed supporting, the
delegation). Sections 2, 5, 7 and the doc header carry no rule-5-dependent
sentence. No missed dependent found.

### 4. Record quality - PASS

- Refuted premise present and sourced: "this tree's catalogs have no nested
  selects" (the brief's `patternStructure` comment), with the dependent
  absolute branch named.
- Three counterexamples with mechanisms, cross-checked against the memo AND
  the tree: `validate-summary` (`locales/en+de/cli.ftl`, column-0 reopeners
  `}, { $warnings ->` / `}, { $infos ->` at cli.ftl:5,8 en / :12,15 de),
  `batch-diagnostics-summary` (`gui-batch.ftl:35,38`), `suggestion-partition`
  (`diagnostics.ftl:68-76`, nested `$kind` -> `$dropped`/`$count`). All match
  the described shapes.
- Authority chain complete: T17 memo path + controller ruling, Option B,
  counterexamples controller-verified.
- Rejected options with honest reasons: A (nesting-aware parser fights the
  line-based charter), C (allowlist would exempt the D39 `$kind` case - the
  entry's own motivation). Matches the memo's cost analysis without
  sanitizing.
- Residual blind spot recorded as what parity can NOT catch:
  flat-structure-invariant drift, with a concrete example. Present and honest.

### 5. No-work-needed premises - PASS (ran, not weighed)

Each verified-unaffected claim re-run against the actual text:

- Correction #5's sentence (line 57): "only the placeable-set and
  selector-structure parity extension" - parity-worded, holds. (Its embedded
  citation is F2.)
- D61 (lines 1280-1281): "D55's parity rule 5 guards the new selector
  cross-locale" - holds; the `$rules` CLDR selector is guarded by
  select-count/selector-var parity plus the parity carve-out.
- D63 gate-coverage bullet (1406-1412): quoted accurately in the amendment;
  already names the real-parse delegation layer - holds.
- D63 rejected-single-bundle note (1428): "the class D55 rule 5's carve-out
  already anticipates" - the carve-out stands under parity semantics; a `ru`
  catalog with `few`/`many` still passes (presence and default-count parity
  are category-set-independent) - holds.
- Section 6 amendments 1-6: read in full; none touch rule 5 - holds.
- Section 8 trigger 2 (1813-1816): "verify D55 rule 5's category carve-out
  passes its catalogs" - coherent, indeed more robust, under parity - holds.
  Trigger 9 (1837-1841): "remaining live content - placeable/selector parity"
  - parity-worded - holds.
- Section 9 "the five D55 rules" (1883): rule amended, not added/removed;
  count holds. The changed carve-out bullet (1886-1891) carries the parity
  semantic and the delegation pointer, matching the rule text.
- Delegation premise: function exists, is exported, is run by
  `catalogs.spec.ts:12`, covers all locale dirs x all 7 `.ftl` files in the
  two groupings; the missing-default case is caught via the dropped-ids layer
  (N1). Ran and confirmed - except the line coordinates (F1).

---

## HARVEST

- **Pattern (ledger candidate): a copied `file:line` citation is a borrowed
  claim and goes stale at every merge touching the cited file.** F1's exact
  mechanism: a round-1 citation, correct at f7e1e83, copied into a round-7
  amendment across the wave-2 merge boundary that had shifted the file +7
  lines - inside a "(controller-verified)" sentence. Greppable trigger: typing
  `:NN` or `:NN-NN` after a filename into a design/amendment block obliges a
  re-verification against the current tree at write time (the same discipline
  the zitat-und-zahl rule applies to quotes and numbers in reports).
- **Pattern (ledger candidate): scope verification labels.** "controller-
  verified" attached to a sentence verified the function's existence and
  behavior but not its cited coordinates; an unscoped label transfers trust to
  every clause it touches. Either scope the label ("behavior
  controller-verified") or verify every load-bearing clause under it.
- **Pattern (positive, keep): the amendment's sweep format** - each
  verified-unaffected site listed with a short quoted anchor - made the
  independent re-walk cheap and every quote checked out accurate. Worth
  keeping as the house format for dependent-sentence sweeps.
- **Over-restriction watch:** "No absolute per-select assertion remains in
  this rule" scopes rule 5 only. Watch that it is not later read as forbidding
  check-i18n from ever carrying an absolute assertion anywhere (rule 3's
  tooltip-completeness and D62's checks are absolute by design); the sentence
  is a statement about rule 5's select-structure part, not a script-wide ban.

---

Reviewer: independent round-7 delta reviewer, 2026-07-22. No design content
was changed; this file is the review's only write.

---

## Delta verdict (fix round, 2026-07-22, commit baa8ee2)

**APPROVED**

Re-verified independently, not taken from the author's report:

- **New span correct on the current tree**: `export function
  assertAllCatalogsParseCleanly` at `e2e/i18n-en.ts:125`, closing brace at
  `:143`; the file is unchanged since wave-2 commit 3fab82f, so the span the
  amendment now cites is the span that holds.
- **Both sites fixed**: design doc line 800 (the rule-5 delegation sentence,
  F1) and line 57 (correction #5, F2) both read `e2e/i18n-en.ts:125-143`,
  function name kept beside the span.
- **Zero stale occurrences**: grep for `118-136` over the design doc returns
  nothing; grep for `125-143` returns exactly the two fixed sites.
- **Commit scope confirmed**: `baa8ee2` touches only
  `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md`, 2 insertions
  / 2 deletions, and the diff is exactly the two citation substitutions -
  no other design text changed.
- **Prior non-findings stand**: the design file's only change since the
  round-7 review is baa8ee2; the intervening commit c898a96 is a ledger
  entry, not the design. Dimensions 2-5 (latitude, sweep, record quality,
  no-work-needed premises) are therefore untouched and their PASS results
  carry over; N1 remains a note, no action.

F1 and F2 are closed. The round-7 amendment is approved.

Reviewer: resumed round-7 delta reviewer, 2026-07-22.

---

## Round-8 delta verdict (2026-07-22, commit 0a1db52: D62 four -> six checks)

**NEEDS FIXES**

One Minor finding; every dimension otherwise passes. Scope: the D62
amendment only; settled content not re-litigated.

### Findings

**M1 (Minor) - source-attribution drift in check 5's rationale (design doc,
check-5 text): "which the T9 content review proved a pipe-pair pattern
misses".** The T9 verdict (`task-09-verdict.md:60,64`) empirically proved the
**leading-pipe** form (`^\s*\|`) misses headerless GFM tables (planted-control
fire-test: any-pipe 4 hits, leading-pipe 2, missing the headerless table) and
harvested "the zero-pipe grep strictly dominates the leading-pipe table
check". "Pipe-pair" appears nowhere in T9; it entered in the controller's
constraint note (`controller-notes.md:138`, "beats the pipe-pair pattern") and
the amendment hardened that paraphrase into a proof claim about the T9
artifact. Substantively harmless (a pipe-pair pattern fails the same
single-pipe case by construction; zero-pipe is ruled IN regardless), but the
durable record now misdescribes what a named source proved - the exact class
the ledger's citation-drift entry was just widened for. Fix: name the
fire-tested pattern ("a leading-pipe pattern") or attribute the pipe-pair
framing to the controller constraint, not to T9's proof.

**N2 (Note, no action) - the empty-state note is an idealized model,
inherited, unreachable.** The round-8 addition ("checks 2 and 4-6 iterate the
(empty) topic-file set and are vacuously green, so the count stays two") is
internally consistent with the note's settled model and its own added claim is
true under it. For the record: the implementation (`a653d39`) derives
`helpLocales` via `readdirSync(HELP_ROOT)`, so an absent `help/` crashes
(ENOENT) rather than reporting reds, and with an empty `help/` present check 1
iterates zero locales and cannot fire either - the settled "two checks
hard-fail" model idealizes both. Moot in practice: gate and tree landed
together in `a653d39`, the state is unreachable, and a crash is equally
not-green, so the note's forcing-order conclusion holds under every reading.
Settled content; not a delta defect.

### Dimension results

1. **Faithful encoding - PASS.** Check 5 as written (any `|`, code spans
   included, hard fail) matches the implementation (`check-i18n.mjs:625`,
   unconditional `line.includes("|")`) and the constraint's zero-pipe letter;
   check 6 (angle-bracket markup outside inline code spans) matches
   `:617,628` (backtick-span strip, then `/<\/?[a-zA-Z]/`) and the
   constraint's mandated exemption. The 5-vs-6 asymmetry is recorded as
   deliberate, attributed to the constraints' letter, WITH its accepted
   consequence (a future `(mkv|mka)` code-span alternation goes red and is
   rephrased, not exempted) - matching T19's minor note exactly.
2. **Latitude scan - PASS.** No permission-form latitude; the enumeration is
   closed at six, the heading's three content bans are exactly checks 4-6,
   the consequence example illustrates a fully defined class, and the
   implementation the text describes exists committed (`a653d39`), so the
   T19-completion path invents nothing.
3. **Sweep completeness - PASS.** Independent walk (greps: all four / four
   D62 / exhaustive / broad `\bfour\b` / "external-URL ban" / all `D62`
   mentions) found exactly the author's changed set (heading, all-six count,
   empty-state note, section 9 line 1950) plus the author's
   verified-unaffected set; every remaining `four` in the document is
   non-D62 (close-abort keys, eslint attrs, D58/D61/D63 counts). No missed
   dependent.
4. **Record quality - PASS except M1.** Provenance chain complete and
   verified: constraints postdate the 2026-07-21 approval (routed from the
   T9/T10 execution-time content reviews - T9 verdict HARVEST lines 64-65
   carry both mechanisms; controller-notes.md carries the verbatim binding
   constraint; T19 report concern 1 surfaces the collision; `a653d39`
   implements and fire-verifies all six both directions). Counts recomputed:
   six enumerated items, "all six", section 9's "six D62 checks", heading's
   three bans - all consistent.
5. **No-work-needed premises - PASS.** Ran, not weighed: both pattern-topic
   files exist on `plan7-g` (`help/en+de/editor-input-pattern.md:8,16`) with
   `(?<season>\d{2})` / `S(?<season>\d{2})E(?<episode>\d{2})` inside code
   spans, and the naive pattern would fire on them (`<s` matches
   `/<\/?[a-zA-Z]/`); pipe-freeness of `help/` re-verified myself - literal
   `|` grep over the tree returns 0 with a positive control (a neighbouring
   pattern over the same files returns hits), independently of T9's and
   T10's own documented planted-control runs.

**Round-7 non-findings stand:** the diff touches only D62 and section 9's
D62 bullet; rule 5's text and both `e2e/i18n-en.ts:125-143` citations (lines
57, 800) are untouched, and no `118-136` occurrence reappeared.

### HARVEST (round-8 additions)

- **A paraphrase in an authority chain re-verifies against the origin
  artifact at each hop.** M1's mechanism: artifact ("leading-pipe") ->
  controller note ("pipe-pair") -> amendment ("T9 proved pipe-pair"), each
  hop plausible, the composition false. When encoding a routed constraint
  that itself cites a source, the encoder checks the source, not just the
  routing - 3rd instance of the durable-artifact citation-drift class.
- **Positive pattern, keep:** the amendment recording an accepted
  consequence ("goes red and is rephrased, not exempted") alongside the
  deliberate asymmetry is exactly what keeps a future reader from "fixing"
  the asymmetry as a bug.

Reviewer: resumed round-7/8 delta reviewer, 2026-07-22.

---

## Round-8 fix delta verdict (2026-07-22, commit 19c0a6c: M1 correction)

**APPROVED**

Re-verified independently:

- **Corrected clause matches the T9 artifact at the letter**: the clause's
  three claims each sit at the cited lines on the current tree -
  `task-09-verdict.md:60` carries the fire-verification of the leading-pipe
  pattern verbatim as `^\s*\|` (planted-control run: any-pipe 4 hits,
  leading-pipe 2, the headerless table escaping it) and `:64` carries the
  dominance harvest including "topics have no legitimate `|` at all". The
  pattern string in the design clause is T9's own, character-exact.
- **"pipe-pair" grep-absent** from the design document (exit 1, zero hits);
  the amendment block carries no repeat of the retracted attribution.
- **Commit scope confirmed**: 19c0a6c touches only the check-5 rationale
  clause (+6/-4, single hunk, design file only).
- **All prior non-findings stand**: both `e2e/i18n-en.ts:125-143` citations
  (lines 57, 800) intact, rule 5 untouched, D62's checks/asymmetry/
  empty-state text and both amendment blocks otherwise unchanged. N2
  remains a note, no action.

M1 is closed. The round-8 amendment is approved.

### Scratch-path adjudication (coordinator's open question)

The corrected clause cites `.superpowers/sdd/plan-7/task-09-verdict.md:60,64`
- a git-ignored location (`.gitignore:2`, confirmed via `git check-ignore`)
that the plan close will salvage into the journal artifacts dir. The
committed document carries three such citations: line 820 (T17 memo, round
7), 1348 (T9 verdict, this fix), 1383 (T19 report, round 8).

**Adjudication: correct as written today; not a present finding; it creates
a binding plan-close sweep obligation.**

- Citing the future salvaged location instead would be a citation false at
  commit time - exactly the round-7 F1 class, and worse than scheduled
  staleness, because nobody can verify it before the salvage happens. The
  house standard these rounds enforced is "verified on the current tree";
  the current-tree address is the only one that satisfies it.
- The path being git-ignored means the three citations are unresolvable from
  git history alone (fresh clone); the amendment blocks tolerate this
  because they quote the load-bearing evidence inline (counterexamples,
  mechanisms, harvest wording), so the citations are corroboration, not the
  sole carrier. That is the right construction and it holds here.
- The salvage move is the edit; the references are its finite, greppable
  dependents (`grep -n '\.superpowers/sdd' docs/` = exactly the three
  sites). Per the standing dependency-sweep rule, the same change that
  moves the artifacts re-points the three citations to the salvaged
  addresses - line refs (`:60,64`, only site 1348) survive a plain move
  unchanged, so the sweep is a path substitution. Recommend the controller
  mirror this as a trigger in the section-8 style (trigger 9's "corrects
  the entry in place when consuming this design" is the exact shape).

Reviewer: resumed round-7/8 delta reviewer, 2026-07-22.

---

## Plan-close batch delta verdict (2026-07-22, commit 7897bc4: four one-liners, two files)

**NEEDS FIXES**

The four items themselves are correctly executed and all of the delta's own
citations verify against the current tree; the M4 fix's sweep is incomplete -
its own premise event ("D63 rewrote `muxsmith-cli/src/i18n.rs`") staled three
more citations in the same document, one of them the same fact M4 re-pointed.

### Findings

**F1 (Major) - design line 1483: `i18n.rs:79-83` cites the raw-id fallback
M4 just re-pointed, at its pre-rewrite address.** D63's per-message-fallback
bullet ("a message missing everywhere falls back to the raw id exactly as
today (`i18n.rs:79-83`)") cites the same fact D51's M4 fix corrected to
`Renderer::render` `:121-123` in this very commit. Current `:79-83` holds
`msg`'s body end and `msg_with_counts`'s rustdoc. Fixing one site of a
same-fact pair while the one-command class walk (`grep -n "i18n.rs"` over the
design) shows the sibling is the exact dependency-sweep failure the Tier-2
citation-drift rule names. Fix: re-point to `:121-123` (or the `render` walk,
`:108-123`).

**F2 (Minor) - design line 1478: `set_use_isolating(false)` cited at
`i18n.rs:30-31`; actual `:62`** (impl; a second copy in the test module at
`:260`). Current `:30-31` is a blank line plus `impl Renderer {`.

**F3 (Minor) - design line ~1494: the `zz-ZZ-invalid` test case cited at
`:212`; actual `:248`** (`mod tests` opens at `:237`). Current `:212` sits in
the `DiagnosticRenderer` impl.

**Verified still-holding, no action** (state them so the fix scope is
exact): `i18n.rs:21-27` (line 1470) - the span now holds the Renderer struct
rustdoc, which states the `--locale` > system > en binding order the sentence
claims; `i18n.rs:7-8` (line 1516) - still `include_str!` embed constants
(the table now spans `:7-10`; under-inclusive, not wrong). Lines 83, 1749,
1829 are dated historical records (section-1 snapshot, amendment-3 target,
E1 ruling) and correctly left as history.

**N3 (Note, owner-surface-pass routing, not a batch defect):** the shipped
`editor-output-directory` tooltip ("Default directory the muxed files are
written to. The output directory picked in the Batch view overrides it per
run.") does not verbalize the empty -> source-directory fallback the
corrected D54 cell describes; whether "tooltip covers it" needs the fallback
stated is a final-strings question for the owner's rendered-surface pass
(section 9's last bullet owns it).

### Dimension results

1. **Delta citations - PASS.** Every span the batch wrote verifies on the
   current tree, read by me: `Renderer::msg` contract rustdoc at
   `i18n.rs:73-78` (rustdoc 73-77 + signature 78); fallback body inside
   `Renderer::render` (fn opens `:108`) at `:121-123` (comment pair +
   `id.to_string()`); `planner.rs:282-286` is the `output_dir` chain
   character-exact, with `unwrap_or_else(|| run.source.clone());` at `:286`;
   the corrected parenthetical matches the chain's semantics (run-level
   override, then profile directory, then source dir). The
   `help-mode-suppression-pointer-scope` entry exists in
   `product-boundaries.yaml` (`:463-476`, tier 2, source human, S21/Option 2
   occurrence recorded).
2. **M6 provenance - PASS.** D52's equivalence sentence ("One capture-phase
   `mouseover` + one `focusin` listener ... Hover and keyboard focus are
   equivalent triggers (a11y)") pre-existed the batch - the design did
   mandate it; the new spec 8.3 sentence is subordinated ("recorded at plan
   close from the shipped, design-mandated behavior"), and section 9's
   "hover and focusin are equivalent (D52)" is consistent. Accurate record,
   right direction of authority.
3. **Pointer-scope letter + E3 history - PASS.** All five sites (spec 8.3
   bullet, design header E3 parenthetical, D52 suppression bullet, amendment
   6(c), section 7's appended scope-refinement) state the entry's letter:
   click activation + drag-reorder suppressed, capture-phase; keyboard and
   text-entry live, deliberately; accidental-mutation class closed. "Both at
   capture phase" is true on the tree (D52's click listener is
   capture-phase; `App.vue:104` registers `dragstart` with `true`, the I1
   fix from 13e138c). Section 7's historical E3 opening ("RULED: global
   suppression with the enumerated allowlist") is intact with the refinement
   layered after it, E1-style - no rewritten history.
4. **Sweep completeness - FAIL (F1-F3).** My walk: "global suppression"
   survives only in the preserved historical record and the amendment
   block's quote; "All other activation", `:41-46`, and "profile's own
   directory" are gone from normative positions (the latter two survive only
   inside the amendment block's own change record, which is correct); the
   spec carries no other suppression sentence; D54 counts untouched. But the
   `i18n.rs` class walk the M4 fix owed (its own premise names the rewrite)
   was not done - three stale spans remain (findings above), one same-fact
   with the M4 fix itself.
5. **Latitude - PASS.** No permission-form latitude; the suppressed-channel
   enumeration is closed (two members, phases stated), the live-channel
   examples are the authority entry's own wording, the allowlist is
   unchanged and still marked exhaustive, and the D54 cell edit introduces
   description, not choice.

**Prior non-findings stand**: rule 5, D62's six checks, and both
`e2e/i18n-en.ts:125-143` citations are untouched by this commit; no
`118-136` or "pipe-pair" reappeared (grep-confirmed in the sweep above).

### HARVEST (batch-review additions)

- **A citation fix names its staling event; the event defines the sweep
  set.** M4's own text says "after D63 rewrote the file" - that sentence is
  the instruction to grep the document for every other citation into that
  file before closing the item. A fix that corrects one member of a
  greppable class it itself names is the F1 mechanism. Ledger candidate as a
  sharpening of the Tier-2 citation-drift entry: the trigger is writing the
  words "after X rewrote/moved/renamed Y" into a fix record.
- **Positive pattern, keep:** layered rulings (historical E3 opening intact,
  dated scope-refinement appended, every current-scope site citing the
  product-boundaries entry) - the E1-reversal format reused exactly as
  intended; and the amendment block quoting the old wording it replaced
  keeps the change auditable without git archaeology.

Reviewer: resumed round-7/8 + plan-close delta reviewer, 2026-07-22.

---

## Class-sweep delta verdict (2026-07-22, commit 52ade44: F1-F3 + i18n.rs citation inventory)

**APPROVED**

Re-verified independently, every claim run against the tree:

- **The three corrected spans are exact.** F1: the D63 fallback bullet now
  cites `Renderer::render` `:121-123` (comment pair + `id.to_string()`,
  inside `render`, fn opens `:108`). F2: `set_use_isolating(false)` at
  `:60-62` is precisely comment-plus-call (`:60-61` the "plain grep-able
  text" comment, `:62` the call) - the design's "grep-able output" gloss now
  matches the code comment's own words. F3: `:248` holds
  `Renderer::new(Some("zz-ZZ-invalid!"))` inside
  `invalid_locale_falls_back_to_en_and_renders` (`:247`) - test name exact.
- **The re-measured D64 count is correct.** `mod tests` opens `:237` and the
  file ends `:518` (518 lines total; the module runs to EOF). My own count:
  `Renderer::new(Some(` = 14 file-wide and 14 within `:237+` (all in the
  module), `Renderer::new(None)` = 0 - and the Some-variant's 14 hits are
  the positive control proving the None-grep's empty result trustworthy.
  Section 9's separate "four D63 renderer unit tests" count is untouched
  and remains consistent (it counts D63's enumerated new tests, not the
  module's constructors).
- **Re-confirmed span spot-check**: `:21-27` re-verified (struct rustdoc
  `21-26` carries the `--locale` > system > en order, `pub struct` at 27);
  `:7-8` still the `include_str!` embed constants. Amendment 3's added
  landed locations verified: struct doc `:21-26`, `Renderer::new` doc
  `:32-38` (`pub fn new` at `:39`).
- **Historical-marking spot-check (section 1 + E1 record)**: original
  evidence preserved verbatim in all three marked sites (spans `:7-8`,
  `:32-37`, `:12`, `:19` unchanged; qualifiers appended, nothing rewritten),
  and the qualifiers are accurate: at the design-creation commit `f7e1e83`,
  `i18n.rs:7-8` holds the two en-only `include_str!` constants and `:32-37`
  the loop loading exactly `[EN_DIAGNOSTICS, EN_CLI]` - the pre-D63 layout
  the paragraphs describe. Each marking names the staling event
  ("design-time, pre-D63") and section 1's additionally points at where
  current spans live ("current spans are cited in D63"), which disambiguates
  the pre-existing "today" wording to design-day.
- **Commit scope confirmed**: design file only (+38/-9); every hunk is a
  citation re-point, a historical marking, the D64 re-measure, or the dated
  addendum recording the class sweep. No settled prose altered beyond the
  citations and their qualifiers. All prior non-findings stand (rule 5, D62,
  the `125-143` citations - untouched by the diff).

F1-F3 are closed; the owed class sweep is done and recorded.

### Precedent ruling: historical marking vs re-pointing for design-time citations

**Ruling: historical marking is the correct instrument for evidentiary
citations, and re-pointing them would be a defect - the citation-drift
convention applies to a citation according to the sentence it serves, not
uniformly.**

- A **live descriptive claim** (D63's "the fallback body ... de -> en -> raw
  id") asserts the present tree; its citation must track the tree, drift is
  a defect, re-pointing is the fix. That is the rule rounds 7-8 and this
  batch enforced.
- An **evidentiary record** (section 1's ground-truth snapshot, section 7's
  E1 fork evidence, amendment 3's description of the text it replaced)
  asserts a past state that justified a recorded decision. Its citation is
  evidence OF that state; re-pointing it at today's tree would detach the
  evidence from the reasoning it supports - falsifying the record, the same
  defect class as rewriting the E3 ruling would have been. Drift is repaired
  with a scoping qualifier, never with new numbers.
- A valid historical marking has three parts, all present in 52ade44:
  (a) the original span preserved verbatim; (b) a qualifier naming the tree
  or the staling event ("design-time, pre-D63 file"); (c) where a live twin
  of the fact exists, a pointer to it, so no reader mistakes the marked
  citation for a current one.
- The discriminator is readable off the sentence, not a judgment call: does
  it describe what the system IS (live - re-point), or record what the
  author SAW when deciding (evidence - mark)? Amendment 3 is the boundary
  case done right: its target spans are evidence (marked), and the landed
  result is a new live claim (cited fresh at `:21-26`/`:32-38`).

Recommend the controller fold this two-class rule into the Tier-2
citation-drift entry's statement.

Reviewer: resumed round-7/8 + plan-close delta reviewer, 2026-07-22.
