# Plan-9 close pass - review brief

**Role:** independent reviewer of the Plan-9 close pass, a package of seven
enumerated corrections across nine files: prose, diagnostic messages, and the
gate's own definition. You did not write it. Model tier: mid (dispatch model:
Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

**What makes this package unusual.** Nothing here changes behaviour, so the test
suite cannot tell you whether it is right - every item is a claim about the tree
that has to be checked against the tree. Two items also change the gate that
checks everything else, which means a defect in them is invisible until the next
push. Grade accordingly: for each item, the question is not "does it compile"
but "is the new sentence TRUE, and is it true of the code as it is now".

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.** The pass is one commit, `9dc3a4d`, and it is
  HEAD.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/closerev-independent/`
  (create it). Never re-run an instrument another agent wrote, never a shared
  default path, never a path the report names. Any absence check needs its own
  fire; the local `grep` is **ugrep 7.5.0**, where `\b` plus bounded repetition
  under `-E` returns zero silently - use `-P` or a script.
- The e2e suite runs against BUILT bundles: `pnpm test:e2e` regenerates them, a
  bare playwright invocation does not.
- If you mutate anything, restore non-interactively (`git checkout --`, never a
  bare `cp` - it is aliased interactive here) with a baseline taken first, and
  prove the restore. Leave the tree clean at HEAD.

## Ground truth, in precedence order

1. The tree itself. Every item is a statement about it.
2. The findings that routed each item, at their sources:
   `.superpowers/sdd/plan-9/task-5-verdict.md` (LOW-4, item 2),
   `.superpowers/sdd/plan-9/whole-branch-review-verdict.md` (delta finding 3,
   item 3; HARVEST, item 5), `.superpowers/sdd/plan-9/task-6-verdict.md`
   (LOW-1, item 4), `docs/ROADMAP.md`'s Plan-9 close-action blocks and its
   "Docs accuracy" section (items 1, 6, 7).
3. The implementer's brief, `.superpowers/sdd/plan-9/close-pass-brief.md`.
4. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

The implementer's report (`close-pass-report.md`) is evidence, not ground truth.

## Dimensions

1. **Truth of every new sentence.** Item by item, check the claim against the
   code it describes, with your own measurement. Specifically: item 1's two
   trigger shapes against the `find` and the branch; item 3's construction-site
   enumeration; item 4's omission list against `e2e/mocks.ts` AND its safety
   argument against the consumers it names; item 7's snapshot count and, more
   importantly, its coverage claim in KIND.
2. **The gate change, both sites, and its blast radius.** `BUILDING.md` and
   `.github/workflows/ci.yml` must carry the same doc invocation. Verify the
   flag is on the doc step in both, that ledger-lint appears in the local gate
   block, and that nothing else in either file now contradicts the change.
   Then ask the question a single-site check would miss: is there any OTHER
   consumer of the gate's definition in the repo that now disagrees with it?
3. **Item 5 is the prerequisite for item 6 and deserves its own pass.** The two
   link repairs must point at what their sentences actually mean - one at the
   module, one at the function. Read both sentences and rule on each choice
   independently; a link that resolves but points at the wrong item is worse
   than one that fails.
4. **The seven numbered concerns.** Each needs an explicit verdict, below.
   Concern 1 is the sharpest: a fenced text was applied verbatim and the
   implementer measured it to be factually wrong.
5. **Latitude, both forms.** The brief fixed each item's semantics and left the
   prose free except where an item pointed at a fenced text. Check for prose
   that exceeded its semantics, and for the inverse - an item stopped short of
   what its finding required.
6. **House dimension**: Tier-2 conformance, in particular
   `core-docs-name-callers-illustratively-never-exclusively` (item 3 is its
   instance) and `ledger-lint-runs-before-every-push` (item 6 lands what that
   entry deferred).
7. **The no-work-needed check**, standing: run every premise the report uses to
   conclude something needs no work - notably item 4's "safe today" argument,
   item 6's "neither file states a gate total", and item 7's decision to leave
   the neighbouring stale numbers as dated records.
8. **Verification quality**: re-run the gate as the change now defines it,
   eleven parts, and recompute every aggregate rather than quoting it (39 test
   result lines, 68 e2e, 212 catalog ids, 516 ledger entries, 13 snapshots).

## Adjudication questions (one explicit verdict each, not pre-rated)

1. **The fenced LOW-4 comment is in the tree verbatim and says the
   profile-load-failure shape "carries neither key".** The implementer measured
   that document and it carries `files: []`; only `mkvmerge_found` is absent.
   Verify that measurement yourself, then rule: does the fenced text stand
   because it was fenced, or is a narrowing edit owed - and if owed, on whose
   licence?
2. **Item 3's clause names three production construction sites.** The finding
   said four. Measure the sites yourself and rule on which number is right and
   whether the clause is complete.
3. **The module doc in the same file still carries the exclusive form**
   ("constructed per planning call"). The finding scoped itself to the type's
   doc; the implementer left the module doc and asked. Rides this pass, or a
   separate vehicle?
4. **`BUILDING.md`'s own enumeration lists ten commands after this change, not
   eleven, because `pnpm build` is documented elsewhere in the file.** Every
   count outside the file counts it as a gate part. Add it to the frontend
   block, or leave the discrepancy?
5. **A dated provenance comment in `ci.yml` still calls rustdoc "the ninth gate
   part".** Left as history. Correct, or re-word?
6. **Item 7 annotated the dated plan-7 enumerations rather than overwriting
   them, and left neighbouring numbers in the same enumerations stale** because
   those enumerations declare their own measurement date. Correct reading of
   what a dated record is, or an under-correction?
7. **Item 4's disclosure is one long sentence.** Fine, or does it need
   splitting?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/close-pass-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with file:line, the evidence you ran and the exact
required change; the seven adjudications; an evidence appendix naming your
instrument paths; and a HARVEST including anything the controller must carry
into its own files (the report already lists count statements that this change
made stale - verify that list rather than trusting it, and say what it missed).
