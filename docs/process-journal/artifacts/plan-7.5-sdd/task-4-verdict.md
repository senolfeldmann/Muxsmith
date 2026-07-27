# Task 4 verdict: the v1-spec amendments (design section 4)

Reviewer: independent task reviewer, mid tier. Commit `70282fd`, base `a0ed95e`.
Read-only review; no git writes.

**Spec compliance: PASS**
**Task quality: PASS**

---

## Method

Ground truth read directly, not via the report: design section 4
(`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md:788-848`)
and the v1 spec's surrounding text
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`).

**Content-anchored extraction** (no line numbers on either side):

- Design amendment 1 = the blockquote following the bare `with` line inside
  `## 4. Spec amendments proposed`, `> ` prefixes stripped, lines joined.
- Design amendment 2 = the blockquote following the line containing
  `Insert after the ... EmptyMatchList ... row:`, same normalization.
- Landed amendment 1 = the regex span `track-rule grid.*?detail editor per rule,`
  on the spec's unique `**Profile editor**` line (assert-unique: exactly 1 match).
- Landed amendment 2 = the spec's unique line starting `` | `EmptyMatchExpression` ``
  (assert-unique: exactly 1 match).

Compared under whitespace collapse. Result is stronger than the brief's
`diff -wB` requirement: **exact string equality**, not merely a zero exit.

| Amendment | design chars | landed chars | equal |
|---|---|---|---|
| 1 (spec 8.2 view 1) | 312 | 312 | yes |
| 2 (spec 5.2 row) | 328 | 328 | yes |

**Diff package verified faithful before use:** the package body is byte-identical
to `git diff -U10 a0ed95e..70282fd` (checked against `-U3`/`-U5`/`-U10`; only
`-U10` matches). It is not a hand-assembled render.

---

## Spec compliance

1. **Byte-faithfulness (both amendments): PASS.** Exact equality per the table
   above. Every clause of the design's wording is present; nothing added.

2. **The 8.2 splice: PASS.** Spec `:375` is one line and reads coherently end to
   end: the parenthetical opens at `(order, source, ...` and closes with `)`
   immediately before `, detail editor per rule, panels for attachments/...`.
   The rest of the item is verbatim untouched (D41/D48 save semantics, "Inline
   validation markers from core diagnostics") - confirmed on the diff, which
   changes exactly one line here.

3. **Row placement and table syntax: PASS.** The new row sits at `:284`,
   immediately after `EmptyMatchList` (`:283`) and before `UnknownPropertySkew`
   (`:285`), exactly as the design directs. 4 pipes / 3 cells, matching the
   table's `| Code | Severity | Condition |` header (`:257-258`); no unescaped
   pipe inside the description; line terminates with `|`. Severity cell reads
   `warning`.

4. **Amendment 2 scoping honored: PASS.** `git diff --stat` = 1 file, 2
   insertions, 1 deletion - i.e. the one new row plus the one-line 8.2 replace,
   nothing else. Exactly one 5.2 row added; no other row touched; no attempt to
   complete the table's 17 un-rowed `diag_codes!` members.

5. **Glyph and whitespace hygiene: PASS.** Both changed lines are pure ASCII: no
   em/en-dash, no curly quotes, no ellipsis, no NBSP, no tab, no trailing
   whitespace.

---

## Task quality

### The plan-7 T21 truncation class did not repeat

Exact string equality on both blocks (above) is a stronger check than the
brief's Step 5 asks for. The two clauses the brief singled out as truncation
bait are present verbatim:

- amendment 1's `- invalid until filled, announced by validation -`
- amendment 2's full suppression clause, through `... for the same node`

### The report's Step 1 red run re-fired independently

Not taken on the report's word. Re-derived from `git show a0ed95e:<spec>`:

- pre-edit `:374` carries the old fragment `... drag to reorder), detail editor per rule, ...` (present, single line)
- pre-edit `:283` is the `EmptyMatchList` error row (present, single line)
- pre-edit `grep -c EmptyMatchExpression` = **0** (the falsifying absence is genuine)
- positive control: pre-edit `EmptyMatchList` hits `141` and `283` - exactly the brief's stated control

So Step 4's post-edit checks are real evidence, not a grep that could never have fired.

### The tree carries what the amendments assert

**Amendment 1 (affordance, merged stream A `e36885f`)** - `src/views/EditorView.vue`:

- "Add appends an empty rule": `addRule()` at `:451-459` builds `[...rules.value, { match: {} }]` - append at the end, empty skeleton (D65).
- "selects it and opens its detail editor": `:458` sets `selectedIndex` to the new last index; the detail section at `:652` is `v-if="selectedRule"`, so it opens reactively.
- "Remove deletes the selected rule without confirmation": `removeSelectedRule()` at `:467-476` splices immediately; no dialog anywhere in the file.
- "legal down to zero rules": no floor, no guard; the Remove button (`:641-648`) is gated only on `selectedIndex === null`.

**Amendment 2 (diagnostic)** - severity, condition and suppression all verified against the code, not against the design's assertion about the code:

- Severity: `crates/muxsmith-core/src/profile/validate.rs:88-91` pushes `Diagnostic::warning(DiagCode::EmptyMatchExpression, ...)`. The row's `warning` is correct.
- "no conditions at all (no exact/substring/regex/any/not)": `MatchExpr::is_empty()` (`crates/muxsmith-core/src/profile/match_expr.rs:83-89`) tests exactly those five keys - the row's enumeration is complete and exact.
- "it would match every track of its source": corroborated by `match_expr.rs:81-82` and by the shipped catalog text (`locales/en/diagnostics.ftl:9`, `locales/de/diagnostics.ftl:16`).
- "config-time": emitted inside `validate::validate`, the config-time validator.
- Suppression clause: `empty_list_here` at `validate.rs:85-87` is `any`/`not` `is_some_and(|v| v.is_empty())` - present-but-empty, top-level, exactly as the row words it; guarded at `:87`. `EmptyMatchList` does fire for the same node (`validate.rs:353`, `:364`). Regression test at `crates/muxsmith-core/tests/validate_hardening.rs:67-72`.
- Catalog wiring: `crates/muxsmith-core/src/report/mod.rs:77` (`empty-match-expression`), both locale catalogs, `crates/muxsmith-cli/tests/catalog_completeness.rs:59`.

### Adjacent-sentence sweep, tables included (`proc-spec-sweep-covers-tables`)

All four spec tables enumerated and checked (`:18-30` decision log, `:156-167`
settable-property map, `:257-289` diagnostics, `:336-346` module map), plus the
prose neighbours of both sites.

No contradiction found. Notable results:

- **5.2 `PassthroughProfile` (`:267`)** - "emitted at validate time so an accidental delete-all-rules edit stays visible" - **corroborates** the new 8.2 text (Remove without confirmation, down to zero) rather than conflicting with it.
- **5.2 `AmbiguousRule` (`:259`)** - "rule matches >= 2 tracks of its source" - complementary: the plan-time consequence of the config-time warning the new row describes. No conflict.
- **4.3 (`:141`)** - "A present-but-empty `any` or `not` list is a config-time error (`EmptyMatchList`)" - consistent with the new row's suppression clause.
- **5.4 static lint** - speaks only of provable overlaps; disjoint from the new row (matches the design's own sweep).
- **§11 non-goals** - carries no add/remove claim; confirmation dialogs and undo appear nowhere in the spec.
- **8.3 baseline** ("every non-obvious control carries a tooltip") - no conflict: the amendment claims nothing about tooltips, and D72 records the obviousness premise as verified.
- Decision-log, property-map and module tables carry nothing about editor affordances or diagnostic codes.
- No stale copy of the replaced fragment survives anywhere outside the plan's and the design's own "replace X with Y" quotations, which are the record of the change and must keep the old text.

---

## Findings by severity

**Blocking:** none.
**Major:** none.
**Minor:** none against this task.

**Observations (2).** Both concern the owner-approved *wording* of design section
4, which Task 4 was mandated to transcribe byte-faithfully. Deviating from it
would itself have been the defect, so neither is a Task-4 finding. Both are
routed to HARVEST.

- **O1 - 8.2's "legal down to zero rules per 4.5" compresses a qualification
  4.5 spells out.** Spec 4.5 reads "Empty rules under `drop` remain a
  `NoTrackRules` error"; `drop` is the default. D69 makes the intended reading
  explicit - the *Remove operation* has no floor, the resulting *state's*
  validity is `keep` -> `PassthroughProfile` info / `drop` -> `NoTrackRules`
  error. The `per 4.5` citation carries that for a reader who follows it, so
  this is a compression, not a contradiction. Worth noting that the stream-B
  help topic is already more precise than the spec here
  (`help/en/editor-tracks-rules.md:23`: "An empty rule list is legal only under
  `Unmatched: keep` ... Removing the last rule is allowed").

- **O2 - 8.2's "invalid until filled" describes warning severity, not a save
  block.** D65 records this explicitly ("Invalid-until-filled means guided, not
  save-blocked"; `saveDisabled` gates on error severity only). The spec sentence
  carries the word without the qualifier, so a reader of 8.2 alone could infer
  Save is blocked on a fresh rule. The design flagged the same nuance itself
  ("One precision worth recording (not a correction)").

---

## HARVEST

1. **Spec-wording clarification candidate (owner call, not a defect):** 8.2's two
   compressions, O1 and O2 above. Both are one-clause fixes if the owner wants
   them ("the Remove operation is legal down to zero rules; the resulting
   state's validity per 4.5"; "invalid until filled - a validation warning, not
   a save block"). Distinct from plan-close trigger 4, which fires on a *core*
   change to `EmptyMatchExpression`'s severity; this is a standing readability
   gap in the spec text as it now stands.

2. **Process signal, positive:** the brief's Step 1 was written so its red run is
   a real fire event with a named positive control, and Step 5 named the exact
   clauses most likely to be dropped. Re-deriving both from git took under a
   minute and left nothing to take on trust. That shape (pre-edit absence +
   positive control + named truncation-bait clauses) is what made this task
   independently checkable at near-zero cost; worth keeping as the default for
   verbatim-transcription briefs.

3. **Reviewer method note:** content-anchored extraction with a uniqueness
   assertion on both sides (exactly one `**Profile editor**` line, exactly one
   `` | `EmptyMatchExpression` `` line) turns a "the text matches" claim into a
   check that fails loudly if a second copy ever appears. Cheaper and stronger
   than a line-number diff, which goes stale on the next insertion.
