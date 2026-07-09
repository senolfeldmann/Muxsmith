# Plan 2 fix pass - SDD progress ledger

Plan: docs/superpowers/plans/2026-07-09-plan-2-fixes.md
Base commit (branch point for this pass): 847b476
Execution: subagent-driven-development (fresh implementer + independent reviewer per task). On master (authorized).

## Tasks
- [x] F2: new diagnostic codes (EmptyMatchList, UnidentifiableSource)
- [x] F1: dry-run runs validate + rendered JSON (implement + independent review + fix)
- [x] F3: validate rejects empty any/not (a67879e; controller-verified, final review for independent pass)
- [x] F4: matcher absent-boolean-is-false (213e1e9, independent review clean)
- [x] F5: planner SourceOverwrite (batch-wide) + UnidentifiableSource (independent review found Critical per-primary scoping, fixed 6f475b3)
- [x] F6: planner output + collision semantics (b5acada + fix 550ba59; review found keep-name .mkv regression)
- [x] F7: suggestion engine no-clobber + valid YAML + cap log
- [x] F8: discovery symlink handling (cb3ae84 + tests 608f2b5, independent review SPEC pass)
- [x] F9: renderer file attribution + skew message (2e0dc00, controller-verified)

## Log
(append one line per completed task: Task Fx: complete (commits base7..head7, review clean))
Task F2: complete (commit d9422b3, mechanical + catalog-guard-verified; scrutiny deferred to final whole-branch review)
Task F1: complete (commits d9422b3..09d7244, independent review found + fixed a real spec-5.5 gap: mkvmerge-not-found path dropped config diagnostics)
  Residual (Minor, for final whole-branch review): the mkvmerge-query-failed path (list_languages fails) has the same defect - config diags dropped - left out of F1 scope. `mkvmerge_found` JSON key only on the not-found path. Human dry-run output not severity-sorted unlike validate.
Task F4: complete (commit 213e1e9, independent review SPEC pass/QUALITY approved)
Task F7: complete (TDD, controller-run, not yet independently reviewed): with_rule_match insert-only for exact/substring (bug C); yaml_fragment now serializes the real MatchExpr delta via yaml_serde instead of hand-formatting (bug D); cap-3 truncation logged via new DiagCode::SuggestionsCapped in batch_diagnostics (D6). New DiagCode required touching report.rs + locales/en/diagnostics.ftl + spec 5.2 table, beyond the plan doc's planner.rs-only file list.
