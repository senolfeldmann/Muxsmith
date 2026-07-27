# Plan-7.5 close fix: salvage re-pointing + superseded-quote disposition

Two mechanical documentation edits at the plan-7.5 close. Both are fully
specified below; nothing here is a judgment call, and no fork is open. If you
believe a premise of this brief is wrong, say so and stop - do not resolve it
at the keyboard.

Repo: `/home/senol/Git/Muxsmith`, work on `master` in the main worktree
(no worktree, no branch). Absolute paths.

## Context you need (one paragraph)

Plan 7.5 added Add/Remove affordances for track rules to Muxsmith's profile
editor. Its execution record lived in the git-IGNORED directory
`.superpowers/sdd/plan-7.5/`; at the plan close that directory was salvaged
into the tracked tree at `docs/process-journal/artifacts/plan-7.5-sdd/`
(commit on master, 31 files). Documents that cited the old git-ignored path
now point at something a reader of the public repo cannot open. Separately,
the owner ruled a wording change to the v1 spec's section 8.2 on 2026-07-27
(commit `406e91b`), which superseded the exact sentence that the plan-7.5
design and plan documents had mandated and quote verbatim.

## Edit 1: re-point the salvaged-artifact citations

File: `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`

Two citation sites, both currently naming the git-ignored path. Locate them
by CONTENT, not by line number (line numbers drift; the numbers below are
from 2026-07-27 and are orientation only):

- around line 836: `` `.superpowers/sdd/plan-7.5/design-review-round-1.md` ``
  in the amendment-2 scoping paragraph (cited for its HARVEST section)
- around line 1120: `` `.superpowers/sdd/plan-7.5/task-2-verdict.md` `` in an
  Evidence bullet

Replace the directory prefix `.superpowers/sdd/plan-7.5/` with
`docs/process-journal/artifacts/plan-7.5-sdd/` at both sites. Nothing else on
those lines changes - not the file names, not the section names, not any
`:line` suffix.

**Scope boundary, deliberate:** the plan document
(`docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md`) also
mentions `.superpowers/sdd/plan-7.5/progress.md` twice, at its house-deviation
header and in its execution-method section. Those are NOT re-pointed: they
state where the tracker lived while the plan executed, which remains true.
This matches the plan-7 precedent exactly (commit `9d01862` re-pointed the
design's three citations and left the plan document alone).

**Verify your surface, do not assume it.** Before editing, run a
content-anchored search for the string `.superpowers/sdd/plan-7.5` across
TRACKED files only (`git grep`), excluding the salvage directory itself
(`docs/process-journal/artifacts/plan-7.5-sdd/`, whose internal
self-references are historical record and stay untouched). If that search
returns sites beyond the two named above plus the plan document's two
tracker-location mentions plus the ROADMAP's own trigger entry plus the two
handoff snapshots, report them rather than editing them - the count above is
the brief's claim and a mismatch is a finding. Note that a plain `grep -r` in
this environment is bound to a function that respects `.gitignore`; use
`git grep` or `command grep`.

## Edit 2: supersession note on the superseded amendment quote

The owner's 2026-07-27 wording ruling changed the v1 spec's 8.2 view-1
sentence (commit `406e91b`). Current spec text, verified at
`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` line 375:

> Add appends an empty rule - incomplete until filled, announced by a
> validation warning - selects it and opens its detail editor; Remove deletes
> the selected rule without confirmation, legal down to zero rules (per 4.5:
> `unmatched: keep` = passthrough, `drop` = NoTrackRules)

Two documents still carry the PRE-ruling wording as a verbatim mandated
block:

- `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`,
  design section 4's amendment-1 block (around lines 802-803): "Add appends
  an empty rule - invalid until filled, announced by validation - ... legal
  down to zero rules per 4.5"
- `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md`,
  the Task 4 transcription target (around lines 367-368), same wording

**Do NOT rewrite either quoted block.** They are the historical mandate, and
the plan's Step-5 transcription check graded the landed text against exactly
those words; rewriting them would erase the fidelity record and make a
reviewed check look like it had graded something else.

Instead add, immediately after each of the two quoted blocks, ONE line in the
document's own voice recording the supersession. Use this text at both sites,
adjusted only for the surrounding markdown (blockquote vs plain paragraph):

> **Superseded 2026-07-27:** the owner's wording ruling (commit `406e91b`)
> replaced "invalid until filled, announced by validation" with "incomplete
> until filled, announced by a validation warning" and expanded the zero-rule
> clause with its 4.5 consequences. The quoted block above is the wording this
> document mandated and Task 4 was graded against; the shipped spec text is
> authoritative.

Nothing else changes. In particular the plan's coverage-map rows that name
the clause by its old wording ("invalid until filled", "legal down to zero
rules per 4.5") stay as they are - they identify which task implemented which
ADR, and the note above covers the wording drift for both.

## Verification you owe

1. `git diff` shows exactly four changed regions across three files: two
   citation prefixes in the design, one supersession note in the design, one
   in the plan.
2. Each of the two old citation strings counts 0 after the edit and each new
   one counts 1 - and you ran each pattern BEFORE the edit and saw it return
   its expected non-zero count, so a zero afterwards is a real absence rather
   than a malformed pattern.
3. The quoted amendment blocks are byte-unchanged: prove it, e.g. by showing
   that `git diff` contains no removal line from inside either block.
4. Report the exact final line numbers of everything you touched.

## Commit

One commit on master, staged explicitly (never `git add -A`), unsigned:

```
git -c commit.gpgsign=false commit -m "plan-7.5 close: re-point salvaged-artifact citations, record the superseded amendment wording

<body>

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>"
```

Do not push. Do not touch `.superpowers/`, the ROADMAP, the journal, the
house-knowledge YAML files, or any code. Write your report to
`.superpowers/sdd/plan-7.5/close-fix-report.md`.
