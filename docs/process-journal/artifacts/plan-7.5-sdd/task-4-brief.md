### Task 4: the v1-spec amendments (design section 4)

On master, after streams A and B merge - docs only; this task is the single owner of the v1 spec file in this plan. Amendment 1 may not land before Task 1's code (the practical join point covers it); amendment 2's asserted core semantics already exist in the tree. Implementer: **cheap tier** (this plan carries the text verbatim; the work is anchor-verify, apply, diff-check); reviewer: **mid tier**. Read design section 4 in full, including its scoping paragraph and self-contradiction sweep (already run design-side; not re-derived here).

**Files:**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

- [ ] **Step 1: Verify both anchors by quoted text** (measured at plan-authoring 2026-07-23): the view-1 sentence fragment `track-rule grid (order, source, match summary, changes, optional; drag to reorder), detail editor per rule,` (spec :374, a single long line) and the diagnostics-table row starting `| \`EmptyMatchList\` | error |` (spec :283, a single line). Also confirm the pre-edit absence that makes Step 4's checks falsifiable: `grep -n "EmptyMatchExpression" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` prints nothing before the edit (this red run is the fire event; positive control: the same grep for `EmptyMatchList` hits :141 and :283).

- [ ] **Step 2: Amendment 1 - spec 8.2, view 1 ("Profile editor"), first sentence.** Replace

> track-rule grid (order, source, match summary, changes, optional;
> drag to reorder), detail editor per rule,

with

> track-rule grid (order, source, match summary, changes, optional;
> drag to reorder; Add appends an empty rule - invalid until filled,
> announced by validation - selects it and opens its detail editor;
> Remove deletes the selected rule without confirmation, legal down
> to zero rules per 4.5), detail editor per rule,

The wording above is the design's verbatim text, line-wrapped for this plan; in the spec the sentence is ONE line - replace the exact substring in place, preserving the single-line layout. The rest of the item (save semantics, inline markers) is unchanged.

- [ ] **Step 3: Amendment 2 - spec 5.2, diagnostics table.** Insert after the `EmptyMatchList` row, as ONE table line in the file:

> | `EmptyMatchExpression` | warning | a rule's `match` expression
> has no conditions at all (no exact/substring/regex/any/not): it
> would match every track of its source (config-time; suppressed
> when the emptiness is a present-but-empty top-level `any`/`not`
> list, which already raises its own `EmptyMatchList` for the same
> node) |

(The suppression clause transcribes `validate.rs`'s own comment and guard - the `empty_list_here` check - verified against the code by the design.) Scoping, binding on the task and its review: this amendment adds exactly ONE row. The 5.2 table is not exhaustive (17 of `diag_codes!`'s 47 members have no row, measured design-side 2026-07-22); completing it is the round-1 review's controller watch item, **outside this plan's scope** - adding any second row is a defect, not diligence.

- [ ] **Step 4: Verify, fire-verified via Step 1's red runs**

```bash
S=docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
grep -c "EmptyMatchExpression" $S
# Expected: 1 (the new row; Step 1 established 0 before the edit).
grep -n "drag to reorder), detail editor per rule" $S
# Expected: no output - the old fragment is gone (Step 1's anchor
# verification was this grep's firing run, at :374).
grep -n "Add appends an empty rule" $S
# Expected: exactly one hit, inside the 8.2 view-1 item.
```

- [ ] **Step 5: The unabbreviated-transcription check** (the ledgered plan-7 T21 truncation defect, not repeated): diff the two landed blocks against design section 4's amendment text (whitespace/line-wrap-insensitive, e.g. `diff -wB` over normalized extracts, or a word-by-word read-through of both). Every clause of the design's wording must be present - in particular amendment 1's parenthetical "- invalid until filled, announced by validation -" and amendment 2's full suppression clause. State in the task report that the check ran and its result.

- [ ] **Step 6: Commit**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "spec: 8.2 names the rule grid's Add/Remove affordance; 5.2 gains the missing EmptyMatchExpression row (plan-7.5 design section 4, amendments 1-2)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Plan close (controller actions, not tasks)

- **Whole-branch review** by an independent reviewer against the design, before any close action (house standing; top-tier model per `proc-03`).
- **Owner rendered-surface pass** over the changed user-facing wording (pre-registered here so the close does not improvise): the new sentences in `help/en/editor-tracks-rules.md` + `help/de/editor-tracks-rules.md` - the complete set; this plan changes no catalog value and no other user-facing string.
- **Salvage re-pointing** (pre-registered; the ROADMAP Triggers entry registered 2026-07-22/23): when the plan-7.5 SDD salvage runs at this close, re-point the design's citation of `.superpowers/sdd/plan-7.5/design-review-round-1.md` (its amendment-2 scoping paragraph) to the salvaged artifact path **in the same change as the salvage**, per the ruled round-8 house pattern.
- **Design triggers routed** (design section 6, all six):
  - Trigger 1 is **CONSUMED at this close**: extend `editor-generic-action-keys`' statement to record the rule grid as the third render site of the generic pair (occurrence ref: the owner-approved design), no budget change (D68: zero new ids).
  - Trigger 2 is resolved by Task 4 landing amendment 2; if the owner declines it at the close instead, register the `EmptyMatchExpression` spec-5.2 gap as a one-liner - it must not stay unrecorded.
  - Triggers 3-6 mirror into the ROADMAP as standing triggers: site-specific wording/tooltips on the generic keys reopens the shared-key owner question (3); a core change to `EmptyMatchExpression`'s severity or the skeleton's emission set re-verifies D65 and case 6's Save-enabled pin (4); an accidental-rule-deletion report routes to the v1.x undo/redo entry, not a confirmation dialog (5); help-annotating the grid buttons is a D54 id/host-set owner change reopening D71 (6).
