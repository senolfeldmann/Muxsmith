# Task 4 delta verdict - fix round 1

Same reviewer as `task-4-verdict.md`. Scope: `.superpowers/sdd/plan-12/
review-3be0d32..06e7a61.diff` (one commit, `e2e/editor-undo-redo.spec.ts`
only) against `task-4-report.md`'s appended "Fix round 1" section. Findings
1, 2, 3, 5 graded; finding 4 not graded (routed to plan close, not the
implementer's). Round-1 approved parts are not re-opened.

All checks re-run independently: fresh rsync of the current committed tree
(excluding `target/`, `.git/`, `dist/`) to
`/tmp/claude-1000/-home-senol-agents-peter/a1386daa-bdbc-4366-b18d-375daf90cf89/scratchpad/muxsmith-copy`,
every mutation followed by an explicit `pnpm build` before the e2e run (per
the newly-ledgered `frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run`),
every restore verified by `diff` against a saved original, never by exit
code. The tracked repository was touched only to build/run read-only
commands and was confirmed unmodified (`git status --short`) after every
round.

## Per-finding verdicts

**Finding 1 - ADDRESSED.** Report-only correction, no test or code owed (the
controller's ruling assigns this property's first producer to Task 5). The
corrected text states the true reach ("no test in this package can observe
`savedSnapshot`'s value at all, in any scenario") rather than a narrower,
differently-wrong one, and the "S1 pins that structure" overclaim is
corrected on its own terms (scoped to the narrow textual property S1
actually checks, not "the mechanism works"). The wrong original sentence is
left standing with the correction beside it, matching this project's
`proc-supersede-never-overwrite` reporting form.

**Finding 2 - ADDRESSED**, with one evidence-integrity defect flagged (does
not change the verdict; see below). I re-derived the "no observation path"
negative myself rather than checking the trace: exhaustively grepped every
consumer of `canUndo`/`canRedo`/`history`/`position` in `EditorView.vue`
(`undo()`/`redo()`, both `!model.value`-gated, and the two `:disabled`
template bindings, both `!model ||`-gated - nothing else reads them
anywhere in the file); read `e2e/mount-entry.ts` in full and confirmed its
only `window`-exposed globals are `__muxsmithMount__`, `__muxsmithModel__`,
`__muxsmithSetProps__`, `__muxsmithEmitted__`, `__muxsmithTopicHtml__`, with
the app instance (`currentApp`) held module-scoped, never exposed; and
independently reproduced the discriminating mutation (`resetHistory`'s
`undefined` branch nulling only `savedSnapshot`, leaving `history`/
`position` standing) with an explicit rebuild in between - the renamed case
still passes 15/15 clean, confirming it genuinely does not (and by this
component's current wiring, cannot) discriminate that defect. **The
substantive conclusion holds.**

**Evidence-integrity finding (moderate, not blocking):** the report's cited
grep for the observation-path negative - `command grep -rn "setupState\|
__vueParentComponent\|getCurrentInstance" e2e/ src/` - does **not** return
nothing. Run verbatim on the current tree it returns 27 lines, all inside
`e2e/.generated/mount-harness.js`. That file is Vue's own bundled runtime
(vendored, gitignored, and already documented elsewhere in this repo -
`eslint.config.js`'s own comment - as "never hand-authored, never linted"),
not test-authored code reaching into internals, so the qualitative
conclusion the report draws from it is still correct on inspection. But the
claim as literally written ("returns nothing") is false, and this is
exactly the negative the coordinator's dispatch named as load-bearing for
the whole fix - a search whose surface was not scoped to exclude a
known-vendored, gitignored directory produced a false absence that happens
to still support the right conclusion, the same shape the ledger already
records for a different plan-9 instance ("the reported colours were
nonetheless correct... the defect is in the evidence as written rather than
in the conclusion").

**Finding 3 - ADDRESSED.** Independently reproduced the report's exact
mutation (`resetHistory(profile)` call deleted from `createBlank`, rebuilt,
full spec-file run): 1 of 15 fails, and it is exactly the new case - the
other 14, including all six mutation-path cases and "open resets", stay
green. I additionally ran a second, more surgical mutation of my own
(`resetHistory`'s profile-provided branch appends instead of replaces,
leaving `position` nonzero) with its own rebuild: this is caught by **both**
the new case and the pre-existing "open resets" case, since both funnels
share `resetHistory` - stronger evidence than the single mutation quoted in
the report. Verified by reading the code, not assuming: `createBlank`
unconditionally ends with `model.value = profile` (a fresh, truthy
`Profile`), with no branch that leaves it falsy, so the new case's
`:disabled="!model || !canUndo"` reads never short-circuit on `!model` -
`canUndo`/`canRedo`'s real values are what the assertions read, confirming
it does not sit behind Finding 2's disjunction trap.

**Finding 5 - ADDRESSED.** Re-measured fresh, independently, without
reusing my round-1 figure or accepting the report's agreement with it:
`git show 85902c7:src/views/EditorView.vue | command grep -nEi
"dirty|isDirty|unsaved|modified"` returns exactly 3 lines (69, 632, 634).
Matches the correction exactly.

## New breakage

**Zero.** `pnpm lint` and `pnpm check:i18n` reproduce clean on the tracked
repository. `git diff --stat 3be0d32 06e7a61` covers exactly the one file
the dispatch names. No other test in `e2e/editor-undo-redo.spec.ts` or
elsewhere was altered by this diff (confirmed: the diff's 3 deletions are
exactly the old test name line and the two-line comment it replaced; every
other line in the file, including the untouched "positive control" comment
and the failed-open test's actual assertions, is byte-unchanged).

## Delivered-state cleanliness

**Confirmed clean.** Per the coordinator's explicit ask, I did not trust a
`pnpm test:e2e` run alone (its `webServer` serves a pre-built `dist/` that
`test:e2e` itself never rebuilds - exactly the mechanism the new ledger
entry `frontend-mutation-evidence-needs-a-rebuild-before-the-e2e-run` names).
On the tracked repository at current HEAD I deleted `dist/` outright,
rebuilt it from scratch (`pnpm build`, exit 0), then ran the full documented
`pnpm test:e2e` command: **94 passed** (93 + the one new case), `git status
--short` empty afterward. Current HEAD (`2e0165f`) sits one commit past the
graded range `06e7a61` - a controller ledger commit touching only
`docs/decision-ledger.yaml` (confirmed via `git show --stat`), not `src/`
or `e2e/`, so this run is equivalent in every way that matters to running
at the graded commit itself.

## Harvest

- The evidence-integrity gap in Finding 2's own re-derivation - a recursive
  grep whose surface included a gitignored, vendored, machine-generated
  bundle (`e2e/.generated/`) it should have excluded, producing a claim
  ("returns nothing") that does not match the command's own output even
  though the qualitative conclusion survives - is a specific instance
  worth a house-knowledge line next to
  `a-search-whose-terms-come-from-memory-produces-a-false-absence`: a
  recursive sweep whose search directory contains a known-vendored,
  gitignored subtree needs that subtree excluded explicitly (or its hits
  read and dismissed by content), not assumed absent because the command
  ran with `command grep`.
- The controller's mid-review ledger commit (`2e0165f`, "a restore is an
  edit too") landed on exactly the mechanism this delta review needed to
  rule on independently (the stale-`dist/` risk) - a fast, well-targeted
  generalization from this task's own fix-round self-report.
