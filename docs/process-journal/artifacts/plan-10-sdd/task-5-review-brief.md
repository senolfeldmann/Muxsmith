# Task 5 review brief - Plan 10

**Role:** independent reviewer of Plan 10, Task 5 (W2: the comment
line-citation sweep - every source comment locating code by a line number is
rewritten to name the symbol, across sixteen files). You did not write this.
Model tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Preamble (binding)

- Never call session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `1a23283`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t5rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never a shared
  default path.
- If you mutate a tracked file, baseline first, restore non-interactively
  (`git checkout --` or `command cp -f`; a bare `cp` is aliased interactive
  here), and prove the restoration.
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty. Capture exit codes
  directly.
- **Prove tree identity per FILE against blobs**, not by a clean `git status`:
  `git hash-object <file>` against `git rev-parse 1a23283:<file>`, plus
  `git diff --name-status 1a23283` to name anything that moved. A controller may
  commit process artifacts under `docs/` while you work; that is not your
  finding, but you should be able to say so precisely.

## Ground truth, in precedence order

1. The Tier-2 entry **`comments-locate-by-symbol-never-by-line-number`** in
   `docs/conventions.yaml`, in full, including its handle and its **SCOPE
   BOUNDARY** sentence. This is the transformation's contract and the owner's
   ruling.
2. The plan,
   `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md`: Global
   Constraints; the Authoring-time verification section's corpus block (both
   expressions with their pasted results); **Task 5** in full - Files list with
   its A/B markers, the "Why expression B's four sites are IN" note, Steps 1-5,
   "Must not decide"; acceptance rows **W2-a, W2-b, W2-c**.
3. `docs/ROADMAP.md`'s "Docs accuracy" stale-citation entry with its RULED block.
4. The cited code itself - each rewritten comment's target - which is the only
   way to check the hardest property: that the named symbol is the one the
   comment MEANS.
5. The four house-knowledge YAML files; cite entries by id.

The implementer's brief (`task-5-brief.md`) and report (`task-5-report.md`) are
**evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/review-44f1c8e..1a23283.diff`
- one commit, sixteen files, 51 insertions / 40 deletions.

## What the controller already measured (reproduce independently, do not skip)

Expression A returns 0 on the end state and 20 against the pre-state commit;
expression B returns 0 on the end state and 4 against an extracted pre-state
tree. So both absence checks have a fired control from a second pair of hands.
Your dimension 1 is not to re-confirm that, but to check what those two
expressions cannot see.

## Dimensions

1. **Did the sweep MISS a member the two expressions cannot see?** This is the
   highest-value question in the review, because the corpus was defined by two
   patterns and a pattern is a claim. Look for line-number locators in forms
   neither expression matches: a spelled-out "line 412", an `L412` form, a
   `#L412` GitHub-style suffix, a citation split across a line break so that
   neither half matches, a line number inside a doc-comment code fence, and any
   locator in a file type outside the selector (`.toml`, `.ftl`, `.json`,
   `.yaml`, `.mjs` config, shell scripts under `scripts/`). Derive your search
   set from the artifacts rather than from the plan's list, and fire every
   pattern you use against a known-present case.
2. **Per site, does the named symbol point at what the comment MEANS?** Open
   each rewritten comment's target and check. The plan warns that several
   citations were already stale, so naming the symbol from the CITED LINE would
   name the wrong symbol - the implementer reports nine of twenty-four were
   measurably stale at HEAD. Verify a sample of those staleness claims yourself
   at the commits involved, and check every rewrite where staleness was claimed
   as the reason the symbol differs from what the old number pointed at.
3. **Comment text only.** No line of code, markup or test logic changed. Prove
   it structurally rather than by reading: every hunk must fall inside a
   comment. Name your method.
4. **The three README-citing sites in `crates/muxsmith-cli/tests/run_live.rs`.**
   Task 4 changed the README under them. Two must name the README's own anchor
   for the passthrough recipe rather than a span; the third carried a QUOTATION
   (`per README.md:91 "every command takes --json"`) that Task 4's correction
   falsified, and the controller's constraint was that the quotation be DROPPED,
   not re-anchored. Verify the committed text against today's README - including
   that the anchor it names actually exists in the file, and that the inlined
   recipe literal still matches the README's recipe byte for byte.
5. **The four expression-B sites.** Their spans are deleted and the surviving
   text must be the identifier the comment itself already supplied (D48, D44,
   D45), with nothing invented. One of them is adjudication question 1.
6. **The two `run.rs` files** - `crates/muxsmith-cli/src/commands/run.rs` and
   `src-tauri/src/run.rs`. The disambiguation is a stated deliverable: check
   that every surviving file reference is unambiguous in the repo, and that the
   comment that previously pointed at the wrong one now points at the right one.
7. **Scope boundary.** No file under `docs/` is edited (the boundary separates
   by the artifact DOING the citing, not by the artifact cited). Verify.
8. **House dimension:** `comments-locate-by-symbol-never-by-line-number`
   itself - does every rewritten comment satisfy its handle, and does any
   rewrite introduce a NEW volatile locator? Plus
   `latitude-carveout-zero-content-structural-forks`, typography (ASCII hyphens,
   straight quotes, no Unicode ellipsis), and the package-wide rule that a claim
   about the tree is a count.
9. **The no-work-needed check.** Wherever the report concludes a site needed no
   further change or that a form cannot occur, run the premise.
10. **Verification quality.** Re-run the full gate as `BUILDING.md` enumerates
    it, foreground, and recompute the report's aggregates (24 matched lines, 21
    comments, 16 files, nine stale citations).

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **`src/editor/registries.ts`.** The plan prescribes `design \`:889-936\`` ->
   `D45`, but D45 is already named in the same sentence, so applying the mapping
   literally would produce "in D45 (D45)". The implementer dropped the
   parenthetical instead. Is that the faithful application of a mapping whose
   purpose is the identifier rather than the parenthesis, or a deviation from a
   prescribed replacement that should have returned as NEEDS_CONTEXT?
2. **`run_live.rs`'s recipe anchor.** The rewrite anchors the byte-exact YAML
   literal by the README's heading rather than by its fence. Does a heading
   anchor still let a future maintainer find the exact block the literal must
   match, or does the byte-exactness requirement need the fence named?
3. **Two rewrites that name a LOCATION rather than a symbol**, each for a stated
   reason: `identify.rs`'s "(its own doc comment)", where the symbol is already
   the sentence's subject, and `run_live.rs`'s use of a full repo path to
   separate the two `run.rs` files. Does the convention's handle cover both, or
   is either a residue of the class the sweep exists to remove?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-5-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with `file:line`, evidence run, exact required change;
the three adjudications; an evidence appendix naming instrument paths and
commands; and a **HARVEST**. Two harvest inputs are already on the table and
want your judgment: the implementer's finding that `src-tauri/src/lib.rs`'s
citation was stale in the very commit that INTRODUCED it (never once pointing at
its target in a committed tree), and its observation that the Tier-2 entry's own
occurrence records the expression-A corpus (20/13) where the swept union is
24/16.

Your final message carries the same in short form.
