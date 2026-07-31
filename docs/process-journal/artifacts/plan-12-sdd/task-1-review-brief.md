# Task 1 review brief (Plan 12): the normative documents

**Your role.** Independent reviewer. You did not write this change and you have
no stake in the plan. You grade it, you do not fix it: findings go in your
verdict, never into the tree. You commit nothing and edit no product artifact.

**Your ground truth, in this order:** the v1 spec as amended by this task
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`), the task brief, the
plan's `## Decision register` and `## Global Constraints` sections, and the four
house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`,
`docs/process-conventions.yaml`, `docs/decision-ledger.yaml`). The house files
are ground truth alongside the spec; cite entries by id.

## What to read

| What | Path |
|---|---|
| The task's requirements | `.superpowers/sdd/plan-12/task-1-brief.md` |
| The implementer's report | `.superpowers/sdd/plan-12/task-1-report.md` |
| The diff, with commit list and stat | `.superpowers/sdd/plan-12/review-bd3aa34..b381675.diff` |
| The decision content the ADRs render | `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md`, sections `## Decision register` and `## Global Constraints` only |
| The ADR house form this file follows | `docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md` |

Do not read the whole plan file.

## The consolidated requirement set for THIS task

Assembled here rather than left to the plan's own table, because the plan's
table is the artifact under review's own self-report on coverage.

| # | Requirement |
|---|---|
| T1-a | Spec section 8.2's editor item carries the fenced replacement **exactly**: `open/save YAML` becomes `create/open/save YAML`, and the prescribed paragraph is appended after "Inline validation markers from core diagnostics." |
| T1-b | Spec section 8.2's app-settings paragraph carries its fenced replacement exactly, including the three-state language sentence and its `(8.4)` citation |
| T1-c | Section 8.4 is NOT edited, and no spec region outside those two is touched |
| T1-d | `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` exists with an H1 `# Plan 12 decisions` and one section per decision, D106 through D110, in the house form: **Decision**, **Rationale**, **Rejected alternatives**, and **Triggers created** where one exists |
| T1-e | Every rejected alternative in the plan's Decision register appears in the file, **each with its steelman stated at its strongest**. A caricatured rejection is a defect. This includes the ones whose steelman is strong enough to be mistaken for the winning argument |
| T1-f | D108 is recorded as a REVERSAL: it names the owner ruling it reverses (S22, 2026-07-22, undo/redo wholesale in 1.x), the old reasoning, and the new reason. It records D66's no-confirmation-for-Remove premise as CONSUMED, not reopened |
| T1-g | D109 records the superseded controller reading (an unconditional warning independent of save state) as superseded rather than as a live option, and records shipping the shell's dialogs in English with a recorded reason as OVERRULED by the owner rather than as a live tradeoff - named rather than numbered |
| T1-h | D110 records the owner's ruling in the GENERAL form he gave it (German translations always ship in the same change, without exception), not as a decision about one dialog, and states the two residuals it does not close (a non-literal `ftl_message` argument; the CLI's identical unserved-locale gap, surfaced not fixed) |
| T1-i | No line-number citation anywhere, in either direction: not into the spec, not into the plan, not inside the decisions file itself (Tier-2 `a-document-never-cites-a-line-number-inside-itself`, `comments-locate-by-symbol-never-by-line-number`) |
| T1-j | The self-contradiction sweep ran as an enumeration with a fired control: all three expressions run and pasted, every hit classified, no expression returning nothing |
| T1-k | The commit is unsigned, carries exactly one trailer, and is pathspec-scoped to the two files in the Files list |
| T1-l | Typography: ASCII hyphens, straight quotes, no Unicode ellipsis. German orthography inside German text is orthography and is correct, not a glyph tell |

## Review dimensions

1. **Spec compliance** against the table above. Verdict per row is not required, but
   an unmet row must be named.
2. **Quality** of the ADR prose: does each rejected alternative's steelman state
   the losing argument at its strongest? Would a later reader who reconstructs
   the real argument find it already answered, or find a strawman?
3. **`house`**: deviations from a recorded Tier-2 convention. Flag them with the
   entry id.
4. **Latitude, both forms.** An explicit permission ("either approach works"),
   AND the commoner omission form: a set mandated but never enumerated, a list
   ending open, a placeholder. The test is not "does a permission appear?" but
   **"must a later reader invent something they are not allowed to invent?"** Ask
   it of every normative sentence in the amended spec and the new ADRs, since six
   later tasks build against them.
5. **The no-work-needed check.** Wherever a passage concludes that a guard, an
   enumeration or a check is unnecessary, **verify the claim that makes it
   unnecessary. Run it, do not weigh it.**
6. **Harvest.** Report observed dominant patterns and repeated rejections worth
   recording in the house ledger. You surface; you never write to the ledger.

## Re-running the implementer's evidence

The report's three sweep expressions are **claimed measurements**: the
implementer ran them against a tree that exists, so you can reproduce them. Do.

**Build your own instruments.** If you re-run an expression, run it yourself
rather than executing a script or fixture the implementer wrote, and if you need
a scratch file, write it to a path the implementer could not have used. Agents in
one session share a filesystem and converge on the same names; a re-run that
silently executes the implementer's own instrument produces agreement by
construction.

**Where an expression contains an enumerated set** - keywords, alternations,
file extensions - that enumeration is itself a claim, and a fire test does not
cover it: firing against one present member passes while a missing member stays
invisible. Judge the membership against the artifact.

## Adjudication questions - one required verdict each

The implementer returned DONE_WITH_CONCERNS with three items. Each needs an
explicit verdict from you. They are phrased in both directions deliberately: a
concern merely carried can die as noted-without-ruling, so a verdict is required
rather than optional. Nothing here is pre-rated.

**Q1. The two advance-named sweep hits do not appear in any expression's
output.** The task brief's Step 4 names two passages "known in advance and
consistent, so they are not reported as findings": section 8.3's help-mode
Escape sentence, and section 11's non-goals. The implementer reports that
neither passage contains any term of the expression that was evidently meant to
reach it, so neither can appear as a hit at all; it read both passages directly
and judged them non-contradicting, and changed nothing.
Rule on **both** halves:
(a) Is the implementer's factual claim correct - do those passages really
contain none of the expressions' terms? Re-run and see for yourself.
(b) Given that, is the sweep's coverage still adequate for its purpose, which is
detecting a contradiction introduced into section 8.2? Either verdict is
available: that the three expressions reach every region where a contradiction
with the amendment could live, or that they do not and the sweep has a blind
spot the task did not close. If the latter, name the region the expressions
cannot see.

**Q2. Section 8.4's shipping statement does not name the Tauri shell.** It names
the GUI catalogs, the help topics and the CLI's embedded catalogs as the
surfaces that ship in both languages. D110 makes the shell a fourth such
surface. The implementer did not edit 8.4 (it is outside the Files list and the
brief forbids it) and surfaced this instead. Rule: is this a genuine
contradiction between D110 and spec 8.4 that this task should have returned as
NEEDS_CONTEXT, or is 8.4's statement compatible as written and the observation
correctly a matter for later routing?

**Q3. The `Triggers created` slot is omitted where no trigger exists.** The
house form in the plan-5.7 decisions file writes `**Triggers created:** none.`
explicitly even when there is none; this file omits the slot in D106, D108 and
D109. The implementer chose the brief's wording ("where one exists") over the
local pattern, on the ground that an explicit brief enumeration beats the
follow-the-local-pattern grant. Rule: is that the correct precedence, or is the
house form's explicit `none.` the stronger signal here because an absent slot
and an unconsidered slot look identical to a later reader?

## Your verdict

Write it to `.superpowers/sdd/plan-12/task-1-verdict.md` and return a short
summary in your final message.

The verdict carries: an overall **spec compliance** verdict and an overall
**task quality** verdict (both are required; a verdict missing either is not
accepted); findings graded Critical / Important / Minor, each with the artifact
location by symbol or section name, never by line number; your three numbered
adjudication verdicts; your harvest; and anything you could not verify, named as
such with the reason.

Every quotation you make is copied from the artifact, and every number you state
is one you measured. A paraphrase is fine and is marked as one.
