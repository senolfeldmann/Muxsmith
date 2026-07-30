# Task B1 review brief - Plan 11

**Role:** independent reviewer of Plan 11, Task B1 (W1: the two open dependency
alerts - a `postcss` lockfile bump, a `cargo deny` configuration repair applying
the owner's interim ruling, and a `glib` investigation that produces a finding
rather than a fix). You did not write this change. Model tier: mid (dispatch
model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Your output is a verdict file
plus the same content in short form as your final message.

## Preamble (binding)

- Never call session-relocation tools (EnterWorktree/ExitWorktree or any
  equivalent). Absolute paths, **foreground runs only**.
- **The work sits in a worktree, not on `master`:**
  `/home/senol/Git/muxsmith-plan11-b`, branch `plan-11-stream-b`, head `c422999`,
  base `5378264`. A concurrent stream is running in
  `/home/senol/Git/muxsmith-plan11-a`; do not read or touch it.
- **Read the files, not a commit hash.** The tree is at `c422999`; the controller
  has made no product edit since.
- **Independent instruments.** Build every harness you use - config copies,
  scripts, scratch trees - under
  `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/b1rev-independent/`
  (create it). **Never re-run an instrument the implementer wrote, and never use
  a shared default path.** The three-way `cargo deny` fire is exactly the place
  this matters: write your own config copies from your own reading of
  `deny.toml`, at your own paths.
- The repository's own `deny.toml` is never mutated to produce a variant; drive
  variants with `cargo deny check advisories -c <your path>`. Prove
  `git diff --exit-code -- deny.toml` clean afterwards.
- **`gh` is off limits to you.** Every `gh` call against this owner's repositories
  owes a `gh-log.md` entry, and that file lives in the main worktree you are not
  writing to. The controller ran the alert feed itself at this session's start
  and both open alerts reproduce (`GHSA-r28c-9q8g-f849` postcss,
  `GHSA-wrw7-89jp-8q8g` glib); treat that as the controller's measurement, and
  verify the advisory's own content against the local RustSec database instead.
- The tree must be byte-identical to `c422999` when you finish. Prove it.
- **Do not re-run the full gate.** The implementer ran all eleven parts green in
  this worktree and the controller re-runs the whole gate on each merged state.
  Re-run only what your own findings require.

## Ground truth, in precedence order

1. The plan,
   `/home/senol/Git/muxsmith-plan11-b/docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`:
   its **Global Constraints**, the **Authoring-time verification** section's four
   stream-B blocks in full, **Task B1** in full (Files list, the "No other file
   is written" paragraph, the boundary-reversal paragraph, Steps 1 through 11,
   "Must not decide"), and acceptance rows **W1-a through W1-m**, which are the
   thirteen halves this task must produce.
2. `.superpowers/sdd/plan-11/plan-brief.md`, item 1.
3. `docs/ROADMAP.md`: the "TWO OPEN VULNERABILITY ALERTS" entry with its RULED
   block, and the v1.x `glib` unsoundness entry the shipped comment must agree
   with.
4. The four house-knowledge YAML files as ground truth alongside them; cite
   entries by id.

The implementer's brief (`.superpowers/sdd/plan-11/task-b1-brief.md`) and its
report (`.superpowers/sdd/plan-11/task-b1-report.md`) are **evidence, not ground
truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/review-5378264..c422999.diff`
carries the commit list, the stat summary and the full diff with context. Read
it in one call rather than re-deriving the range with git commands.

## Dimensions

1. **Contract compliance, character for character.** The two `deny.toml`
   insertions against the plan's fences, byte for byte from your own extraction -
   including their placement (immediately after `yanked = "deny"` and before the
   `# All entries below are transitive` comment; the ignore entry immediately
   after the quick-xml line). The commit message and trailer shape.
2. **Bounded diff, not merely non-empty.** `git diff -U0 -- deny.toml`: no
   existing ignore id reworded, reordered or removed, no other key touched. The
   lockfile diff: exactly the package set `{postcss, nanoid}` and nothing else,
   with `nanoid` justified as postcss's own dependency.
3. **The three-way fire, rebuilt by you.** Shipped state green; the scope live
   (ignore entry removed) failing with `ID: RUSTSEC-2024-0429`; and the control
   with BOTH the scope key and the ignore removed exiting 0. A single green run
   proves none of this, which is why the plan prescribes three. Report each exit
   code and the discriminating output line.
4. **The blast radius as a SET.** Under the new scope, is the fired set exactly
   `{RUSTSEC-2024-0429}` with no other error or warning class? Set-difference it
   yourself against the pre-existing ignore list rather than eyeballing a total.
5. **Untouched things, with the instrument fired.** `Cargo.lock`, `package.json`,
   the `cargo deny` invocation in `BUILDING.md` and `.github/workflows/ci.yml`,
   the profile model and schema. An `--exit-code` check reporting no change looks
   identical to one aimed at a path that cannot change, so fire it on the two
   files that DID move.
6. **The `postcss` requirement, not the landing version.** The requirement is
   `>= 8.5.18`; the landing version is deliberately unfenced. Verify the four
   lockfile sites agree with each other and that `pnpm install --frozen-lockfile`
   succeeds on the result.
7. **Latitude, both forms**, including the inverse: did the implementer decide at
   the keyboard something that should have returned as NEEDS_CONTEXT? Conversely,
   did it return or omit something the plan had already settled? Its two numbered
   findings are where to look hardest.
8. **House dimension.** Tier-2 conformance by id, in particular
   `ci-04-dependabot-cadence`, `ci-10-pin-everything`,
   `proc-07-verify-against-source`, `proc-no-work-needed-check`,
   `proc-verification-step-must-be-falsifiable`, `proc-normative-count-recomputed`.
9. **The no-work-needed check.** Wherever the report or the shipped comment
   concludes that something is unnecessary, already covered, or cannot happen -
   run the premise that makes it so.

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **A wrong count inside a SHIPPED comment.** The plan's fenced `deny.toml`
   comment says `unmaintained` "(default `all`) reported its 18". The implementer
   measured 16 `note[unmaintained]` plus 2 `note[vulnerability]` on the pre-state,
   and applied the fence verbatim as the plan requires. The controller reproduced
   the measurement independently on `master`: `note[advisory-ignored]` 18,
   `note[unmaintained]` 16, `note[vulnerability]` 2 - so 18 is the ignore-entry
   count and not the unmaintained count, and the sentence conflates two sets.
   **Two questions, both explicit: (a) does the mechanism the comment explains
   survive the correction, or does the corrected breakdown change the account?
   (b) what exactly should the sentence say?** Give the replacement wording you
   would ship; the controller routes it as a plan amendment to the plan's author
   and original reviewer, and does not patch it in. The implementer's sharpening
   is on the table and is yours to accept or reject: it reports that in
   cargo-deny 0.19.9 the vulnerability class has NO scope key at all and
   therefore reaches transitive crates unconditionally, while `unsound` defaults
   to `Workspace` - verify that at the tool's own source before you rely on it.
2. **The inverted twelfth consumer.** The plan's authoring section states that
   `glib-macros 0.18.5` consumes `glib` over a proc-macro edge that `-e normal`
   excludes, and calls it the excluded twelfth consumer. The implementer measured
   the opposite direction; the controller reproduced it
   (`cargo tree -i glib-macros@0.18.5 -e normal --depth 1` shows `glib` as the
   dependent, and `glib`'s own dependency list contains `glib-macros`).
   **Does this change the eleven-parent figure or any decision resting on it, and
   what is the correct statement of the excluded-edge caveat - or is there no
   excluded consumer at all?**
3. **The unrun alert feed.** The implementer deliberately did not re-run
   `gh api` on the alert feed, on the ground that a `gh` call owes a `gh-log.md`
   entry in a worktree it was forbidden to touch, and verified the GHSA alias
   against the public advisory instead. **Was that the right call, and is
   acceptance row W1-e satisfied by the substitute plus the controller's own
   session-start run, or does the row require a measurement this task did not
   make?**
4. **What the change buys.** Step 6 requires four statements in specific terms,
   including that the advisory is IGNORED and not FIXED, and that the GitHub
   alert stays open and is not dismissed. **Are all four present and accurate in
   the report, and does any sentence anywhere in the diff or the report read as a
   resolution claim it is not entitled to make?**
5. **The permanent-guard gap.** The task deliberately ships no permanent guard
   that the `unsound` key stays set, and the plan routes that to a ROADMAP trigger
   rather than to new lint infrastructure. **Is the three-way fire adequate
   coverage for what this task changes, or does a config key whose loss would be
   silent owe more than an execution-time demonstration?** Note the boundary the
   house rule draws: new test INFRASTRUCTURE may be deferred, a scenario the
   existing infrastructure can already express may not.

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/task-b1-verdict.md`:

- Verdict: APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES.
- Numbered, severity-tagged findings (Critical / Important / Minor), each with
  `file:line`, the evidence you ran, and the exact required change.
- The five adjudications, one explicit verdict each, with the replacement wording
  question 1 asks for.
- An evidence appendix naming your instrument paths and the commands you ran.
- A **HARVEST** section: observed dominant patterns, repeated rejections, and
  anything the rest of this plan must carry. The controller is the single writer
  into the house-knowledge files; you surface, you do not write.

Your final message carries the same content in short form: verdict, the findings
as one-liners with severity, the five adjudications, and the verdict file path.
