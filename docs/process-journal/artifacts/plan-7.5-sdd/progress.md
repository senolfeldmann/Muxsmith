# SDD ledger — plan: docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md

BASE (master at execution start): 1d82179
Stream A worktree: .worktrees/plan75-a (branch plan75-a)
Stream B worktree: .worktrees/plan75-b (branch plan75-b)

Task 1: dispatched (stream A implementer, mid tier, BASE 1d82179)
Task 3: dispatched (stream B implementer, mid tier, BASE 1d82179)
Task 3: report DONE_WITH_CONCERNS (commit 29952b9); review dispatching
Task 1: report DONE_WITH_CONCERNS (commit fc9e9a4); pnpm-grep pattern promoted tier-2; review dispatching
Task 3: minor (deferred): pronoun-antecedent ambiguity, Remove sentence, both locales (owner pass)
Task 3: minor (deferred): warning-claim wording vs D65 marker non-firing (whole-branch triage)
Task 3: complete (commits 1d82179..29952b9, review clean - both verdicts APPROVED, Q1-Q3 adjudicated, 2 deferred minors)
Task 1: complete (commits 1d82179..fc9e9a4, review clean - both verdicts APPROVED, Q1-Q3 adjudicated for the implementer)
Merge-time controller action (design trigger 1): update editor-generic-action-keys statement in place - the rule grid is the third generic-action render site - when stream A lands on master.
Task 2: report DONE_WITH_CONCERNS (commit 92ba1e7); review dispatching
Task 2: fix round 1 opens after design amendment - review findings: M1 committed comment overclaims (one-sentence fix), M2 keydown mechanism unguarded repo-wide (event-level witness needs design amendment; routed to resumed design author). Q1: case correct-as-landed at contract level.
Task 2: fix round 1/5 dispatched (M1 comment + M2 amended case-9 witness; amendment 89782cd+e525813 four-eyes approved)
Task 2: fix round 1/5 applied (commit ae24589; both fire-tests show witness-only failure); scoped re-review dispatching
Task 2: fix round 1/5 (2 addressed, 0 open - M1 comment, M2 witness; re-review reproduced the acceptance fire-test incl. whole-suite inversion 1-failed/61-passed; commits 92ba1e7..ae24589)
Task 2: complete (commits fc9e9a4..ae24589, review clean after fix round 1)
Merging stream A (plan75-a: T1+T2) into master; trigger-1 ledger action + full gate follow.
Stream A merged (e36885f) + trigger-1 consumed (341c411); gate GREEN. Merging stream B.
Stream B merged (33be397); gate GREEN. Dispatching Task 4 (spec amendments, cheap tier, on master).
Task 4: report DONE (commit 70282fd); review dispatched (fact line follows dispatch result)
Task 4: complete (commit 70282fd, review clean - PASS/PASS; two wording observations to owner pass)
All 4 tasks complete. Whole-branch review dispatching (path-scoped diff: the two plans interleave on master, the 7.5 pathset is disjoint).
Whole-branch verdict: READY (whole-branch-verdict.md). Close actions: owner pass, salvage re-pointing, triggers 3-6 mirroring. Plan close next (or next session per budget).
OWNER RULINGS (2026-07-27, in-session): wording items 1-3+5 APPROVED as recommended (4 cross-ref sites to title-naming form; button-subject disambiguation; detail-panel naming; spec "incomplete until filled, announced by a validation warning" + zero-rules consequence clause); item 4 (tooltip) approved AS LANDED, no change. Executing now as post-plan fix dispatch (owner call: this session, lean vehicle).
Owner-pass wording fix: complete (commit 406e91b, review APPROVED - fidelity byte-exact, both sharpened claims measured MORE correct vs D65). Close items added: (H1) plan doc quotes the now-superseded spec amendment text - one-line supersession disposition at plan close; (H2) residual owner-pass candidate: de "das Thema zur Vorschlagskarte" (2 sites) vs title-naming form.
