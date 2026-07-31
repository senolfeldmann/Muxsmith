# Task 5 fix round 1 - delta verdict

Scope: the diff `5ba09e3..57bcc41` (one commit, `e2e/smoke.spec.ts` +
`src/components/ConfirmDialog.vue`) against `task-5-report.md` section 10. The
approved parts of the original pass are not re-opened; only what this diff touches was
re-examined. Reviewer instruments reused from the original pass, same scratch
directory outside the repo, one new probe added (`esc-delta.probe.ts`). Every in-tree
mutation below applied, built, run, then reverted and content-verified (`md5sum`, not
`git status`/exit code) before the next. Repo left clean at `HEAD` `57bcc41` throughout.

## Finding A (Esc coverage) - CLOSED, both checks pass

**Does the case actually exercise Esc, not a path that resembles it?** Verified with my
own instrument (`esc-delta.probe.ts`, event-instrumenting the live `confirm-dialog`
element in the exact scenario the shipped case builds): pressing Escape fires the
native `cancel` event then `close` on the dialog itself, in that order, with focus
correctly inside the dialog (`confirm-dialog-confirm`, the default-autofocused
element) when the key is pressed. Also checked the one other Escape consumer in this
codebase, `App.vue`'s `onHelpKeydown` (document-level, capture-phase): it is registered
only inside `watch(helpMode, ...)`'s "on" branch, so on the editor's discard-guard
screen (help mode never entered) it is not attached to the document at all and cannot
interfere. The case exercises the real mechanism.

**Is `@cancel.prevent` the right red state, or a neighbour of the mechanism the case
protects?** The report's own mutation (`@cancel.prevent`, reproduced independently -
build hash `index-DzduOr0v.js` matched the report's paste exactly, and the run
reddened only the new case) tests whether Escape does anything at all. That leaves a
sharper question unasked: does the case also catch the more dangerous failure, Escape
resolving to the WRONG boolean (confirmed, i.e. discard) rather than doing nothing? I
attacked that directly - changed `onClose()` to `settleAsk?.(true)` instead of `false`
- and it reddened three cases: the new Esc case, Case 2 (the cancel button funnels
through the same `onClose`), and downstream Case 3(ii). The case's own assertions
(file-dialog call count, surviving field value, Undo still enabled - not merely
"dialog hidden") are what catch it. Both mutations reverted and content-verified
(`md5sum` back to `a1a0c22e...`, rebuild hash back to `index-CEyJLAxY.js`).
**Verdict: the case protects the actual safe-direction contract, not just dialog
visibility, and is not vulnerable to being satisfied by a neighbour mechanism.**

## Finding B (false citation) - CLOSED, citation verified true

Read `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` directly (not the
report's paste): D109 decision 6 reads "Esc closes it, which reads as cancel: the safe
direction" - word for word, punctuation included, what the shipped comment quotes. File
path, decision number, and quoted text all check out. Per the coordinator's own
pre-check, the citation form (no line number, only file + named decision) is house-
conformant; I only verified the content, which is accurate.

## Finding C (reentrancy gap) - CLOSED, restatement not softened

Compared section 10 item 3 against my own original wording line by line. Every load-
bearing qualifier survives: the mechanism (`settleAsk` unconditionally overwritten,
first caller's promise orphaned forever, second caller's confirm-click answers
instead), the reproduction claim ("against the actual, unmutated shipped bundle"), the
non-reachability claim scoped correctly to *today's* real-input paths (hit-tested
input blocked, no keyboard shortcut bypasses it), and the severity label (Moderate,
unchanged). The deferral is stated with a named, observable trigger ("the arrival of
`ConfirmDialog`'s own anticipated second caller") rather than a vague "later," and the
report explicitly disclaims the soft reading in its own words ("not weakened into
'unreachable, so it's fine'"). No shading found.

## File placement judgment

**Correct.** The new case belongs with Cases 1-6 in `e2e/smoke.spec.ts`'s "discard
guards" describe, not with the three repaired cases in `e2e/editor-undo-redo.spec.ts`.
Reasoning: Esc-cancel is `ConfirmDialog`'s own behavioral contract (Step 1), the same
class of thing Cases 1-6 already cover (guard fires or doesn't, confirm or cancel
outcome) - not an undo/redo history assertion with the guard as an incidental hurdle,
which is what the three `editor-undo-redo.spec.ts` repairs are. Checked the describe
block's shared scope for anything that could make the new case pass for the wrong
reason: it holds only `const`/`function` fixtures (`PATH_A`, `profileA`, `cleanReport`,
`loadedDoc`, `gotoEditor`) - no `beforeEach`/`beforeAll`, no shared mutable state
between tests. The new case calls `installTauriMocks` with its own command set and its
own `recorded` array, exactly like every sibling; it inherits nothing that isn't also
available to, and independently scoped for, Case 1/2. It reuses Case 2's own scenario
shape (open, edit, open again while dirty) and asserts the identical three outcomes
Case 2 asserts, swapping only the cancel channel - the right sibling to sit next to,
and it is placed directly after Case 2 in the diff.

## New breakage from this fix diff

**0.** Full `e2e/smoke.spec.ts` + `e2e/editor-undo-redo.spec.ts` (61 tests) reran green
on the restored, unmutated tree. No new typography violations, no new line-number
citations, in the diff's added lines.

## Harvest

- **A discriminating mutation can be true and still be the coarse one.** The fix
  report's `@cancel.prevent` attack genuinely discriminates (proved the case can go
  red) but only exercises "does Esc do anything," not "does Esc resolve the SAFE
  value specifically" - the sharper, more realistic regression shape for a cancel/
  confirm surface. Where a case's name is about a *direction* (safe vs. dangerous, not
  merely present vs. absent), the fire-test earns its claim only by attacking the
  direction, not just the presence.
- **A document-level, capture-phase key listener registered conditionally
  (`App.vue`'s `onHelpKeydown`, live only in help mode) is easy to mistake for a
  standing global handler when auditing "what else reacts to Escape."** Worth checking
  the registration's own guard, not just the handler's body, before ruling a keyboard
  path clear of interference.

Delta verdict written to `.superpowers/sdd/plan-12/task-5-delta-verdict.md`.
