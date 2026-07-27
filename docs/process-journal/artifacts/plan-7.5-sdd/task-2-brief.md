### Task 2: D71 help-mode conformance - the additive case 9 in `e2e/help-mode.spec.ts`

**Stream A**, after Task 1. Implementer: **mid tier**; reviewer: **mid tier**. Read D71's help-mode block, design section 5 case 9, and `help-mode-suppression-pointer-scope` (`docs/product-boundaries.yaml:463`).

**Files:**
- Modify: `e2e/help-mode.spec.ts` (additive: exactly ONE test appended)

**Interfaces:** consumes Task 1's buttons, the file's own helpers (`topicMarkup`/`normalizeInPage`, `installTauriMocks`/`resolveWith`), the I1 sibling's fixture shape. Produces nothing downstream.

Zero new production code: the buttons are unannotated activation controls and the shipped capture-phase delegation covers them by construction (D71); adding a button-side help-mode condition is a defect, not diligence (design section 8).

- [ ] **Step 1: Re-verify the describe-block anchors by quoted text** (Task 1 shifted nothing in this file, but the rule is text-anchors within a serial stream): the file's three describes are `"help mode (D52)"`, `"help mode annotations (D54)"` and `"help mode drag suppression (I1)"` (measured at plan-authoring: :95/:186/:395). The new test is **appended inside the `"help mode (D52)"` block** - the activation-suppression family, per the design.

- [ ] **Step 2: Write the test** - title: `"the rule-grid Add button mutates outside help mode; both activation channels are suppressed inside it"` - following the I1 sibling's in-test-counterpart shape (real app + `installTauriMocks`, its own opened-profile fixture, mutation control and suppression assertion in the SAME test and harness). Fixture: a one-rule profile (`exact: { type: "video" }`); mocks as in the I1 sibling (`detect_mkvmerge`, `"plugin:dialog|open"`, `load_profile`, `validate_profile_model`). Sequence and assertions (design case 9, complete):

  - Open the profile (`nav-editor`, `editor-open`), confirm the starting row count.
  - **Outside help mode (the non-vacuity controls, both channels)**: click `editor-rule-add` -> row count +1; focus `editor-rule-add` and press Enter -> row count +1 again. Both channels demonstrably mutate.
  - **Toggle help mode on** (`help-toggle`, sidebar visible): click `editor-rule-add` -> row count unchanged AND the sidebar renders the `view-editor` topic (compare `innerHTML` against `normalizeInPage(page, topicMarkup("view-editor"))`, the D52 block's own mechanism) - the click was suppressed and pinned the fallthrough id; focus `editor-rule-add`, press Enter -> row count unchanged (the D52 keydown interception).
  - **Add, not Remove, carries the assertions deliberately** (transcribe this rationale as a comment): Remove is disabled without a selection, so a suppression check against it could pass vacuously.

- [ ] **Step 3: Run the new test, then the whole file**

Run: `pnpm test:e2e -- --grep "both activation channels are suppressed"` - expected PASS (the mutation controls prove non-vacuity; the suppression halves pass by construction, which is exactly what "conformance by construction" asserts). Then `pnpm test:e2e -- help-mode` (the full file) - expected: all pre-existing cases green.

- [ ] **Step 4: Additive-only check, fire-verified**

```bash
cd /home/senol/Git/Muxsmith/.worktrees/plan75-a && git diff --numstat e2e/help-mode.spec.ts
# Expected: deletions column 0 - the change is purely additive; no existing
# assertion was touched. Fire-verify: temporarily delete one existing line,
# re-run, confirm a non-zero deletion count, restore, confirm 0 again.
```

- [ ] **Step 5: Frontend gate, foreground**

Run: `pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e`
Expected: green.

- [ ] **Step 6: Commit**

```bash
git add e2e/help-mode.spec.ts
git -c commit.gpgsign=false commit -m "e2e: help mode suppresses both activation channels of the rule-grid Add button, with in-test mutation controls (D71, case 9)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Stream B

`.worktrees/plan75-b`, branched from the same master state as stream A. One task.

---

