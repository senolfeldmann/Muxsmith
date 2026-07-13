# Muxsmith era E3 (2026-07-09) — decision-history reconstruction

Era E3 = Plan 3 "full resolution and command generation (pure layer)". Impl range `62d4956..7d46547` (14 commits). Sources mined: design memo D7-D12 (`specs/2026-07-09-plan-3-design-decisions.md`), plan doc (`plans/2026-07-09-plan-3-resolution-command.md`), progress ledger + 14 verdict files (`artifacts/plan-3-sdd/`), journal entry `2026-07-09 | Plan 3 complete`, git log.

E3 is a pure-core era: domains are core / testing / process / ci only (no cli/gui/i18n/executor code was written — executor items appear only as Plan-4 forward decisions recorded in the memo). One record per occurrence; the same (topic, approach) at several points in the trail is emitted as separate occurrences (decided / reinforced / violated-corrected / deferred).

| # | topic | approach | kind | domain | occ_kind | occ_ref |
|---|-------|----------|------|--------|----------|---------|
| 1 | Plan 3/4/5 split | pure layer before process before GUI | pattern | process | decided | memo D7 |
| 2 | Plan 3/4/5 split | same | pattern | process | reinforced | journal Plan-3 |
| 3 | Persisted per-job logs | defer to Plan 5 | non-decision | executor | deferred | memo D7 Deferrals |
| 4 | Executor spawn abstraction | trait behind spawn, mirror Identify | pattern | executor | decided | memo D7 fwd |
| 5 | --jobs N concurrency runtime | std thread pool, no tokio | restraint | executor | decided | memo D7 fwd |
| 6 | SIGINT cleanup placement | ship in Plan 4, not deferred | pattern | executor | decided | memo D7 fwd |
| 7 | command purity / Plan as IR | Plan->Vec<String>, no Profile, format-neutral | pattern | core | decided | memo D8 |
| 8 | command purity / Plan as IR | same | pattern | core | reinforced | whole-branch |
| 9 | identify parses attachments+chapters | from the -J already fetched | pattern | core | decided | memo D9 |
| 10 | identify parses attachments+chapters | same | pattern | core | reinforced | task-1 verdict |
| 11 | chapters as count, presence only | parsed-but-unused intentional | pattern | core | reinforced | whole-branch M8 |
| 12 | attachment scope | primary-only; donors --no-attachments | pattern | core | decided | memo D10 |
| 13 | reuse existing Plan-2 machinery | locator/template/LanguageIndex/DiagCodes | pattern | core | decided | memo D10 |
| 14 | reuse existing Plan-2 machinery | same | pattern | core | reinforced | whole-branch |
| 15 | settable-language validation site | per-file at application point | pattern | core | decided | plan T5 / D10 |
| 16 | batch-level settable-language check | optional fold, low value | non-decision | core | deferred | whole-branch rec |
| 17 | command argv construction | groups + --track-order, mirror mkvtoolnix-gui | pattern | core | decided | memo D11 |
| 18 | verify vs real mkvmerge v100 | run binary, never from memory | pattern | testing | decided | memo D9/D11, plan |
| 19 | verify vs real mkvmerge v100 | same | pattern | testing | reinforced | whole-branch |
| 20 | add cardinality + zero-match | attaches-all collection-populator, zero=warn | pattern | core | decided | memo D12 / 62d4956 |
| 21 | add cardinality + zero-match | same (two-round debate) | pattern | core | reinforced | journal Plan-3 |
| 22 | add cardinality + primary-only | same | pattern | core | reinforced | task-8 verdict |
| 23 | one match algebra tracks+attachments | Matchable trait | pattern | core | decided | plan Architecture |
| 24 | Matchable for &&Track | blanket impl<M> for &M | pattern | core | decided | task-2 verdict |
| 25 | scope-constraint vs truthfulness fix | accept doc-comment truth fix | pattern | process | decided | task-3 verdict |
| 26 | title Ctx field parity | add source_stem to match validate.rs | pattern | core | decided | task-6 verdict |
| 27 | shared primary_ctx() helper | decline (two instances) | restraint | core | decided | task-6 verdict |
| 28 | chapters external reuse | mirror track-external, no identify | pattern | core | reinforced | task-7 verdict |
| 29 | input-group membership | exclude zero-track donors | pattern | core | violated-corrected | task-9 v1 -> d55f19d -> v2 |
| 30 | donor group ordering | first-Some, unpinned by golden | non-decision | core | deferred | task-9 v2 |
| 31 | empty-group exclusion rationale | correct false "mkvmerge rejects" comment | pattern | core | violated-corrected | whole-branch M2 / 7d46547 |
| 32 | incremental golden tests | each task extends prior golden | pattern | testing | decided | plan / journal Deltas |
| 33 | live mkvmerge acceptance | gated, self-skipping | pattern | testing | decided | plan T12 / task-12 verdict |
| 34 | argv correctness vs v100 | no command.rs correction needed | pattern | core | reinforced | task-12 verdict |
| 35 | live test coverage breadth | richer gated live (attachment+changes) | non-decision | testing | deferred | whole-branch M1 |
| 36 | zero-output-track plan warning | consider planner empty-plan diag | non-decision | core | deferred | whole-branch M3 |
| 37 | FakeIdent+lang() dup | extract tests/support.rs | non-decision | testing | deferred | whole-branch M7 |
| 38 | tempdir leak (mem::forget) | let TempDir drop | non-decision | testing | deferred | whole-branch M7 |
| 39 | attachment-id fixture realism | 1-based ids | non-decision | testing | deferred | whole-branch M5 |
| 40 | eager resolve on discarded plan | leave (pre-finalize design) | non-decision | core | deferred | whole-branch M6 |
| 41 | plan-execution method | subagent-driven-development (SI-1) | pattern | process | decided | journal / progress |
| 42 | model-tiering per role | sonnet impl/review, opus whole-branch | pattern | process | decided | journal Mechanics |
| 43 | per-task gate re-run | controller re-runs, never trust report | pattern | ci | decided | journal Mechanics |
| 44 | per-commit gate | test+fmt+clippy+deny, never skipped | pattern | ci | reinforced | plan Global Constraints |
| 45 | core-crate discipline | prose-free, diags-as-data, no new codes | pattern | core | reinforced | plan Global Constraints |
| 46 | brief self-containment | manually append front-loaded refs | pattern | process | decided | journal Friction |
| 47 | SDD artifact salvage | salvage by name, drop stale reports | pattern | process | decided | journal Friction |

## Detail (evidence + steelman per record)

See structured output for full evidence quotes, statements, and steelman fields. Key debated / corrected occurrences:

- **D12 add cardinality (#20/#21):** two-round debate. Şenol pushed back on the font special-case; resolved by the slot-vs-collection reframe (track/chapters donors fill a unique slot; select/drop/add populate a collection). Adopted: add-attaches-all. Rejected steelman: exactly-one, erroring on 0/>=2 like a track donor.
- **input_groups (#29):** implementer shipped the wrong reading (track-less donors as empty groups); task-9 review called it Important; fixed `d55f19d` + regression test; re-reviewed clean.
- **empty-group rationale (#31):** whole-branch review drove real v100, found the exclusion comment's stated reason empirically false (mkvmerge accepts empty groups); comment corrected `7d46547`, guard kept.
- **Matchable &&Track (#24):** brief's "resolves via type inference" claim was factually wrong; implementer added blanket `impl<M: Matchable> for &M`, reviewer confirmed sound.
- **source_stem (#26):** brief's literal wording would have produced a validated-but-silently-empty title field; implementer added source_stem to the Ctx to match validate.rs.
