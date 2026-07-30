# Amendment 5 author brief - Plan 11

**Role:** author of Amendment 5 to Plan 11. You write the amendment INTO the plan
document; an independent reviewer judges the delta afterwards. Model tier: mid
(dispatch model: Opus 5). Effort: xhigh.

**Scale, decided by the controller and not yours to change: this is a ONE-PAIR
amendment.** No task is added, removed or re-cut. The task set stays A1-A4 plus
B1, every task keeps its boundaries, and B1's work is unchanged - only the
JUSTIFICATION of one thing it defers changes. Every edit you make is a fenced
string, a figure, a caveat, a control, or a deferral row inside an existing task.

**Where this sits.** Plan 11 is under EXECUTION, not in planning. A1 is committed
and reviewed (`a0d5d3e`), B1 is committed and reviewed (`c422999`), A2 is running
right now, A3 and A4 have not started. Six defects in the plan itself surfaced at
execution - four found by implementers who refused to correct them at the
keyboard, and two more by the reviewers. **Two of them shipped into a commit and
a fix round will follow yours to repair them**, so your fenced replacement text
is what that fix round applies.

## Preamble (binding)

- **Work in the MAIN worktree, `/home/senol/Git/Muxsmith`, on `master`.** The
  plan document lives there and is not part of either stream's branch. Do NOT
  enter, read or write `/home/senol/Git/muxsmith-plan11-a` (a task is writing
  there right now) or `/home/senol/Git/muxsmith-plan11-b`.
- **You edit exactly one file:**
  `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`.
  No product file, no `deny.toml`, no house-knowledge YAML, no ROADMAP. The
  controller and a later fix round own those.
- **A second writer shares this working tree's git index.** Commit with an
  explicit pathspec: `git commit -- <path>`, never a bare `git commit`.
- Never call session-relocation tools. Absolute paths, foreground runs only.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, no
  em-dash. Anything you write into a fence that ships into a source comment is
  also width-constrained by the file it lands in.
- **Read the files, not a commit hash.** The plan has been amended four times
  already; read what is there now.

## What to read first

1. The plan itself, in full enough to place your edits:
   `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`.
   Its four existing amendment sections show the house form for what you are
   writing.
2. `.superpowers/sdd/plan-11/task-a1-verdict.md` - findings 4 and 5, and
   adjudication 1, which supplies exact replacement text for the control.
3. `.superpowers/sdd/plan-11/task-b1-verdict.md` - findings 1, 2, 3, 4 and 5,
   and adjudications 1 and 2. **Adjudication 1(b) supplies the replacement
   wording for the shipped `deny.toml` comment, width- and glyph-checked.** Read
   its reasoning for dropping the count rather than correcting it.
4. `.superpowers/sdd/plan-11/task-a1-report.md` and `task-b1-report.md` for the
   implementers' own measurements.
5. `.superpowers/sdd/plan-11/progress.md` - the controller's tracker, whose
   surfaced-items list carries each defect with its reproduction.

## The six defects this amendment repairs

Each is already measured. **You do not re-derive them from scratch, but you DO
verify any figure you write against its source** - the plan's own history is that
a fix round introduced new wrong figures three times.

1. **Task A1 Step 3's soundness control for absence check O cannot fire.** It
   names a retired plan document as the known-present case for the digit-form
   expression; that file carries only spelled ordinals. The A1 reviewer supplied
   exact replacement text naming a document it measured to contain two digit-form
   matches, and its reasoning for not using the plan-11 document (whose count
   moves on every amendment) is part of the fix. Reproduced independently by the
   controller: digit form 0 in the named file, spelled form 3.
2. **The authoring section's "one live consumer" claim is understated by one.**
   `docs/ROADMAP.md` carries a fenced forward-looking replacement text citing the
   removed ordinal, beside the Tier-2 entry the plan already names. The controller
   has since recorded that rider as fired-and-re-deferred; the plan's claim about
   the SIZE of the live-consumer set is what you correct.
3. **The fenced `deny.toml` comment asserts a wrong count.** It says `unmaintained`
   "(default `all`) reported its 18". Measured on the pre-state, independently by
   the implementer, the reviewer and the controller: `advisory-ignored` 18,
   `unmaintained` 16, `vulnerability` 2. The 18 is the ignore-ENTRY count.
4. **The same fenced comment asserts wrong tool semantics, and this one nobody
   caught until the review.** It says the `workspace` scope "excludes every
   external crate". `Scope::Workspace` keys on DIRECT DEPENDENTS, so an external
   crate that a workspace member depends on directly IS in scope; the reviewer
   proved it with a throwaway crate depending on `glib` directly, which fires
   under `unsound = "workspace"`. Take the reviewer's replacement wording; the
   mechanism the comment explains survives and is strengthened, but the sentence
   as fenced is false.
5. **The permanent-guard deferral's premise is refuted.** The deferred-by-decision
   row grounds itself on "a lint asserting a `deny.toml` key would be new gate
   infrastructure". It would not: `unused-ignored-advisory = "deny"` is one key in
   the same table, and the reviewer measured it turning the dropped-key regression
   into a hard gate failure. It also measured that the loss is not silent at
   defaults - cargo-deny emits `warning[advisory-not-detected]` naming the exact
   ignore line. **The row must stop resting on a refuted cost.** What replaces it
   is NOT a decision to add the key: the reviewer explicitly recommends against
   adding it blind, because it also reddens the gate when an ignored advisory
   legitimately disappears upstream. **The controller has PARKED it as a one-key
   owner decision with the measurement attached**, and the row records that
   routing. B1's shipped work does not change.
6. **The `glib` "on normal edges" caveat is wrong in three ways**, and the plan
   states it in several places. The direction is inverted (`glib` depends on
   `glib-macros`, not the reverse); `-e normal` does not exclude proc-macro edges
   at all (cargo has a separate `no-proc-macro` kind, and the reviewer
   demonstrated a proc-macro parent surviving `-e normal` on another crate); and
   `-e build` / `-e dev` return nothing for glib under a fired control. **The
   eleven-parent figure does not move** - eleven is the complete direct-consumer
   set - but the caveat explaining what the figure excludes is false and there is
   no excluded consumer at all.

Two smaller items, from the B1 verdict's minors, to fold in while you are there:

7. **Task B1 Step 5's instruction `git diff --exit-code -- deny.toml` "clean" is
   unperformable as written** at the point it appears: the file legitimately
   differs there, because Step 4 has already edited it. What the step means is
   that the VARIANT runs did not mutate the repository's own file. State the
   check it actually needs.
8. **The local gate and CI run different cargo-deny versions** - 0.19.9 locally,
   0.19.8 in CI (the action's Dockerfile pins it). The reviewer measured
   `src/advisories/cfg.rs` byte-identical between the two, so nothing breaks here,
   but the plan leans on a config key without ever naming the skew. Record it
   where the plan discusses the tool.

## The rule that governs your own writing, because this plan's history is the argument for it

**Every figure you write is re-derived from a run, and every fenced string you
write is checked against the artifact it lands in.** Three previous fix rounds on
this plan introduced fresh wrong figures while repairing old ones, and the
document's own self-review records the method that finally worked: execute the
document's own text rather than a retyped equivalent, and visit every site that
RESTATES a figure, not only the site that computes it. Defects 3, 4 and 6 each
appear at more than one site. **Sweep for each of them by searching, not by
working from a list**, and say in the amendment how you established each set.

**Do not compress or condense anything.** The plan's reviewer has already ruled
that its meta-text is compressed at the plan close, not mid-execution.

## Deliverable

- A new **`## Amendment 5`** section in the plan's house form, dated 2026-07-30,
  stating its routing (this brief, `task-a1-verdict.md`, `task-b1-verdict.md`),
  its scale and why it is one-pair, what moved and why, and what was deliberately
  left alone.
- The actual edits at every site each defect touches, including the acceptance
  map, the authoring section, the corrections table and the self-review where
  they restate a corrected figure.
- **The plan's own counts recomputed** where your edits change a set.
- A commit on `master`, pathspec-scoped to the plan document only, unsigned
  (`git -c commit.gpgsign=false commit -- <path>`), trailer
  `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>` - exactly one trailer,
  no `Claude-Session` line. Do not push.

## Report contract

Write your full report to
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-11/amendment-5-report.md`: every
site you changed with its reason, how you derived each set, every figure you
re-ran with its output, and anything you found and did NOT change.

Return to the controller only: status, the commit SHA, a one-line summary of what
moved, and concerns.
