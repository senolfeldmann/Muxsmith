# Amendment 5 review brief - Plan 9

**Role:** independent reviewer of Plan 9's amendment 5, both halves - the design
rider under D104 and the plan's Task-6 qualifier. You did not write them. Model
tier: mid (dispatch model: Opus 5). Effort: xhigh.

**You commit nothing and edit no file.** Output: a verdict file plus the same
content as your final message.

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.** The amendment is commit `1e0dbd8`, two
  documents, and it is the tree's HEAD.
- **Task 6 is mid-flight and its work sits uncommitted** in this same tree: six
  modified files under `e2e/` plus the untracked `e2e/jobsview-reset.spec.ts`.
  Do not touch, stage, revert or run anything that writes to them. You are
  read-only in any case.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/a5rev-independent/`
  (create it). Never re-run an instrument another agent wrote, never a shared
  default path, never a path the author's report names. Any absence check needs
  its own fire; the local `grep` is **ugrep 7.5.0**, where `\b` plus bounded
  repetition under `-E` returns zero silently - use `-P` or a script.

## What this amendment is

Task 6 returned NEEDS_CONTEXT on a blocking fork: D104's item 2 fixes an
assertion - `cancel-batch` is disabled after a fresh dispatch is rejected - whose
target control cannot be rendered in that scenario's end state, because the
control's own `v-if` names `runActive` among its disjuncts and a fresh rejection
leaves all three false. The behaviour D104 asserts is confirmed; its stated
observable is unsatisfiable. The controller ruled, without escalating, that item
2's second assertion asserts the control's ABSENCE paired with the positive
bearer (`jobs-empty` visible, the `v-else` of the same condition). This amendment
writes that ruling into the design and qualifies the one plan sentence that
restated the old vehicle. The code edit rides Task 6's fix round and is not part
of this amendment.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (the v1 spec).
2. The Plan-9 design as it now stands: **D104 in full, including the new
   amendment-5 rider**, and the `## Amendment log`.
3. The plan's **Task 6** and its amendment log.
4. `src/views/JobsView.vue` - the tree is the arbiter of every render-condition
   and line claim the rider makes.
5. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

The author's brief (`amendment-5-brief.md`), its report (`amendment-5-report.md`)
and Task 6's report are evidence, not ground truth.

## Dimensions

1. **Is the recorded reason true?** Re-derive every anchor and condition the
   rider states, against `src/views/JobsView.vue` at the tree, with your own
   instrument. The load-bearing chain: the control's render condition, the
   fresh-dispatch reset, the catch arm's `runActive = false`, and that
   `ensureJobsLength` is unreachable on rejection. A rider that records a true
   ruling for a false reason is a finding.
2. **Is the replacement assertion right, and is the pairing argument sound?**
   The claim is that `jobs-empty` visible is logically equivalent to
   `runActive === false` given `jobs` empty and `finishedSummary` null, and that
   `toHaveCount(0)` alone would pass against a view that never mounted. Check
   both halves at the template. If the pairing has a hole - a state where
   `jobs-empty` is visible and `runActive` is true, or vice versa - that is a
   BLOCKING finding, because the amendment would then fence a weaker assertion
   than the one it replaces.
3. **Completeness of the sweep.** The amendment's own thesis is that a stated
   vehicle became false. Every other place that restates it must be qualified or
   consciously left. Run your own sweep over both documents AND over the tree,
   and check the disposition of each hit. One is already known: the header doc
   comment of the uncommitted `e2e/jobsview-reset.spec.ts` (line 39), which the
   author routed to Task 6's fix round rather than editing. Rule on whether that
   routing is correct and recorded where the fix round will actually see it.
   Anything you find that nobody has dispositioned is a finding.
4. **The "Unchanged by this amendment" enumeration** - is it the right set? An
   enumeration in a normative position is exactly the latitude-by-omission shape:
   check what it leaves out as hard as what it names.
5. **Latitude, both forms**, including the inverse: did the author resolve at the
   keyboard something that should have returned? Its five named judgment calls
   are where to look hardest, in particular the two pointer qualifiers added
   inside D104 beyond the rider.
6. **House dimension**: Tier-2 conformance;
   `latitude-carveout-zero-content-structural-forks` (does the amendment stay
   inside its enumerated scope);
   `concurrent-writers-need-pathspec-scoped-commits` (the author committed with
   a pathspec while another agent's work sat in the tree - verify the commit
   contains exactly the two documents and that the seven `e2e/` entries are
   intact); `proc-normative-count-recomputed` for any count the amendment states,
   including the amendment/round numbering.
7. **The no-work-needed check**, standing: run every premise the report or the
   rider uses to conclude something needs no work, no qualification or no
   re-measurement. Two are named by the author itself - that the
   `not.toBeEnabled()` measurement need not be re-run because Task 6 ran it, and
   that section 5's count-level references carry no falsified statement.
8. **Typography and form**: ASCII hyphens, straight quotes, no Unicode ellipsis
   on the added lines; the rider follows the file's existing rider shape and the
   log entries follow their siblings' form.

## Adjudication questions (one explicit verdict each, not pre-rated)

The author raised six. Rule on each in both directions.

1. Are the two pointer qualifiers inside D104 - on item 2's numbered line and on
   the `runActive`-not-a-prop paragraph - in scope, or should the rider have
   stood alone?
2. Is omitting a design-commit hash from the plan's amendment entry acceptable
   for a single-commit amendment, given amendments 1 and 3 cite one?
3. Is "CONTROLLER-RULED AMENDMENT 5" the right label in a log whose four
   previous entries are owner-ruled?
4. Is attributing the `not.toBeEnabled()` measurement to Task 6 sufficient, or
   must the rider carry a measurement its own author re-ran?
5. Should the plan half have carried an explicit fix-round instruction for
   `e2e/jobsview-reset.spec.ts` (the assertion at `:176` and the header comment
   at `:39`), or is the rider-as-fence plus the routing clause the correct split?
6. Does the "Unchanged by this amendment" close enumerate the right set, in
   particular naming item 2's scenario and first assertion as unchanged while its
   second assertion changes vehicle?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/amendment-5-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with file:line, the evidence you ran and the exact
required change; the six adjudications; an evidence appendix naming your
instrument paths; and a HARVEST including anything the Task-6 fix round must
carry and any observation worth a ledger entry.
