### Task 3: D71 topic content - `editor-tracks-rules`, en + de

**Stream B** (`.worktrees/plan75-b`). Implementer: **mid tier**; reviewer: **mid tier**. Read D71 in full (including the two rejected help-id routes - they are the reason this content lands in the EXISTING topic), `content-claims-anchor-bound` (`docs/decision-ledger.yaml:3810`), `help-topic-h1-scheme` (`docs/conventions.yaml:1033`), and both current topic files in full (the register the new sentences must match).

**Files:**
- Modify: `help/en/editor-tracks-rules.md`
- Modify: `help/de/editor-tracks-rules.md`

**Interfaces:** consumes nothing from stream A (the claims are closed below); produces the topic content the owner's rendered-surface pass finalizes at the plan close.

This is the T10-finding-1 restoration: the fabricated Add/Remove sentence that review deleted returns **as truth**, with every claim anchor-bound. This is authored content, not boilerplate - real prose in each locale's register (the de body's du-imperative style), never a transliteration of the en text and never a placeholder. The defect locus named by `content-claims-anchor-bound`: sentences that extrapolate PAST their anchor; the reviewer grades the extrapolations.

**The claim enumeration (D71, transcribed - the sentences carry exactly these claims, nothing more):**

- "Editing a rule" section gains the affordance mechanics: Add appends
  a new empty rule at the end, selects it and opens the detail panel;
  the empty rule is announced by a warning until its match expression
  is filled (anchor: D65/D67, the empirical emission). Remove deletes
  the SELECTED rule, is unavailable until a row is selected, and asks
  no confirmation - saving is what makes changes permanent (anchor:
  D66, D41 save-note surface).
- "When the list may be empty" gains one clause: removing the last rule
  is allowed and lands in exactly the legality described there (anchor:
  D69/core-83).
- h1s are NOT touched (content-only edit), so `help-topic-h1-scheme` is
  unaffected; the edited files must still pass the D62 content bans
  (no URLs, no pipes, no raw HTML - the new sentences are plain prose).

The de sections carrying the edits are the same two by heading: "Eine Regel bearbeiten" and "Wann die Liste leer sein darf" (measured at plan-authoring: en :9/:17, de :9/:17).

- [ ] **Step 1: Author the en additions** in the two named sections, carrying exactly the enumerated claims. Do not invent behavior beyond the anchors (no keyboard-shortcut claims, no undo promises, no severity claims beyond "a warning").

- [ ] **Step 2: Author the de additions** - same claims, real de prose in the existing body's register, de orthography (umlauts, ß).

- [ ] **Step 3: Structure verification, fire-verified.**

```bash
cd /home/senol/Git/Muxsmith/.worktrees/plan75-b && git diff help/ | grep -E '^[+-]# '
# Expected: no output - no h1 line changed in either locale.
# Fire-verify: temporarily edit the en h1, re-run, confirm the +/- pair
# appears, revert, confirm empty.
pnpm check:i18n
# Expected: green - the D62 content bans (external-URL, pipe/table,
# raw-HTML) pass over the edited files. Fire-verify once: temporarily add
# `https://example.com` to the en topic body, run, confirm the red exit
# naming the file, restore, confirm green.
```

- [ ] **Step 4: Frontend gate, foreground**

Run: `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e`
Expected: green - the help-mode/annotation specs derive their expected sidebar HTML from these same files at runtime, so content edits cannot desynchronize them; a structural mistake fails the markdown/help gates instead.

- [ ] **Step 5: Commit (both locales, one commit - the bilingual duty)**

```bash
git add help/en/editor-tracks-rules.md help/de/editor-tracks-rules.md
git -c commit.gpgsign=false commit -m "help: editor-tracks-rules documents Add/Remove truthfully - skeleton append + warning guidance, selection-scoped unconfirmed remove, last-rule legality (D71; wording rides the owner surface pass)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Final task on master

