# House-knowledge clusters - domain `process`

Reconstructed from 100 occurrence records spanning eras E0-E8. Records grouped by identical `(topic, approach)` even when worded differently across eras; occurrences merged, identical `date+ref` deduped, distinct `date+ref` kept as genuine recurrence. `promoted = count >= 3`.

Counting unit follows the `ci`/`cross` siblings: an **occurrence = one distinct (date, decision-event)**. Same-date, same-decision citations (spec + journal + commit for one call) collapse into a single occurrence with a concatenated `ref`; a decision **re-attested** one era later (E0 -> E1, or a handoff restating a plan-1 call) dedups to the earlier occurrence; the pattern **applied again in a new plan/session** is a distinct occurrence, even on the same calendar date as another plan.

Dates verified against the repo (`git show -s --format=%cs`): `61249f9`/`fad067c`/`c402914`/`411087f` = 2026-07-08 (E0/E1); `d4390d7`/`59d24c8`/`7ba90ee`/`c0c0ef7` = 2026-07-09 (E2/E3/E4, Plan-4 memo); `705f735`/`c9bd6b4`/`cad84a3`/`656449c` = 2026-07-10 (E5 complete / E6); `62aaf61` (session-8 README) = 2026-07-11; `b38a46f`/`b535038`/`c38a197`/`f25b02d` = 2026-07-12 (E7 close / E8). Plan-4/Plan-5 whole-branch verdicts land 2026-07-10; Plan-5.5 journal + close-wave = 2026-07-12.

**Nine clusters reach the promotion threshold:** proc-01 SDD (7), proc-02 whole-branch review (4), proc-03 model assignment (3), proc-04 spec-wins (4), proc-05 commit-signing (3), proc-06 mkvtoolnix-parity (4), proc-07 verify-against-source (3), proc-08 parallel-worktrees (4), proc-09 idiomacy-review (3). No count was padded: same-session/same-document citations collapse to one occurrence, the verification-rerun rule was NOT credited its incidental mentions inside SDD records (proc-10 stays at 2 on primary-subject touchpoints only), and the two Plan-2 execution records ([21] deviation + [22] correction) are one violated-corrected occurrence.

---

## proc-01-sdd - Plans executed via subagent-driven-development
- **kind:** pattern | **status:** settled | **count:** 7 | **promoted:** yes (at 3)
- **Statement:** Plans run via SDD under a controller: a fresh implementer subagent per task, an independent task reviewer, fix/re-review waves, and a final whole-branch review; the plan never grades its own work. Adopted at Plan 1, deviated to inline once at Plan 2 (self-verified "125 tests green" but shipped ~11 bugs; a retrofit independent review caught them all before merge), then held for every subsequent plan and folded into the packaged doctrine (section 0).
- **Steelman:** null (the inline-execution counter-case is the Plan-2 occurrence below).
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 "Mechanics" + handoff plan-1-close + Plan 1 REQUIRED SUB-SKILL header (E0[3]/E1[14]) | "~31 dispatches: 13 implementers, 13 task reviews, ~7 fix/re-review waves, 1 docs pass, 1 final whole-branch review." |
| 2026-07-09 | violated-corrected | journal 2026-07-09 Plan 2 Decisions + fix-pass; plan-2-review/independent-review-2026-07-09.md (E2[21]/[22]) | inline execution skipped the SDD apparatus; "125 tests green, shipped ~11 bugs" turned into caught-before-merge by the retrofit independent review. |
| 2026-07-09 | decided | journal 2026-07-09 Plan 3 complete (Scope) + progress.md (E3[27]) | "First Muxsmith plan executed fully via superpowers SDD (SI-1), in contrast to Plan 2's inline execution." |
| 2026-07-09 | reinforced | Plan 3.5 Global Constraints + progress ledger (E4[35]) | "Execute via SDD per HANDOFF SI-1 (fresh implementer + independent reviewer per task); the controller re-runs suites itself." |
| 2026-07-10 | reinforced | Plan 5 waves + journal Plan 5 (E6[50]) | "SDD in 7 waves with 3 parallel worktree waves, sequential merges re-gating each time." |
| 2026-07-12 | decided | Plan 5.5 Global Constraints + progress.md (E7[61]) | "fresh implementer + independent reviewer per task; controller re-runs gates itself; reviewer verdicts are FILES at creation." |
| 2026-07-12 | reinforced | journal session 7 pt2 (E8[82]) | "Superpowers-throughout + parallelize-independent (SI-1) folded into doctrine section 0." |

---

## proc-02-whole-branch-review - Final whole-branch review as a distinct stage
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** A separate final whole-branch review grades the plan's code against its own constraints after every per-task review passes; it repeatedly caught cross-task drift no task-scoped review could see. At close it runs on the strongest model plus a roll-up funnel over reviewer minors.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 "Moments" (E0[4]) | final reviewer failed the plan on three counts; "the plan does not grade its own work" proved the load-bearing process rule. |
| 2026-07-09 | reinforced | journal fix-pass + plan-2-fixes-sdd/FINAL-review.md I1 + commit 59d24c8 (E2[23]) | FINAL whole-branch review (opus) caught the literal-`.mkv` empty-stem output every per-task review missed (F6 checked pre-append, not the final stem). |
| 2026-07-10 | reinforced | whole-branch-review-verdict.md + journal Plan 5 (E6[51]) | caught the `start_run` override cross-task drift; "the single strongest argument this session for the final cross-cutting review." |
| 2026-07-12 | decided | whole-branch-verdict.md + plan T23 (E7[69]) | whole-branch review on the strongest model + roll-up funnel: 37 ledger items -> 3 fix-now / 16 defer / 14 discard / 4 resolved, each with a named vehicle. |

---

## proc-03-model-assignment - Dispatch model chosen by cognitive demand
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Model strength goes where judgment is, cheaper capacity where transcription is: strongest model for the final review and the decision memos, mid-tier for judgment implementers and every task reviewer, cheapest for tasks whose code the plan already carries.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 model-split bullet (E0[6]) | "haiku for transcription implementers, sonnet for judgment implementers and all task reviewers, fable for the final review." |
| 2026-07-09 | decided | journal 2026-07-09 Plan 2 Decisions (E2[19]) | "spend model strength on the DECISIONS (D1-D6) not the transcription"; memo + spec fold-in under Fable, then quota to Opus. |
| 2026-07-09 | decided | journal 2026-07-09 Plan 3 complete metrics (E3[28]) | "sonnet for all implementers and task reviewers, opus for the whole-branch review; controller Opus 4.8; none ran on the controller model." |

---

## proc-04-spec-wins - Spec is the binding source of truth; spec wins on conflict
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** The design spec is the binding contract; on any plan/code/spec conflict the spec wins and the conflict is flagged, not improvised around. Corollaries: normative decisions are folded into the spec (a decision left only in a memo can be silently overridden by stale spec text), a self-contradiction sweep runs after any spec amendment, and code wins only where the spec itself is wrong, after which the spec is amended.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 "What the process caught" + Plan 1 Global Constraints + commits 3c24845, cd3f239/f7afa8d (E0[7]/E1[16]) | spec-wins applied for a template prose fix; `UnknownProperty` name collision -> spec amended, code kept. |
| 2026-07-09 | decided | memo 2026-07-09-plan-2-design-decisions.md header + commit d4390d7 (E2[20]) | normative D1-D5 folded into the v1 spec (4.4/4.8/5.2/5.4/9) so stale spec text (4.3/4.4) can't silently override. |
| 2026-07-12 | decided | Plan 5.5 Global Constraints + T16 + memo D32 (E7[68]) | spec §9.2 amended to the decided semantics + a spec self-contradiction sweep after amending (doctrine §1). |
| 2026-07-12 | reinforced | journal session 7 pt2 (E8[83]) | "every spec-anchored gap -> implement pre-1.0 (spec is a binding contract)." |

---

## proc-05-commit-signing - Agent commits deliberately unsigned
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Agent commits and merges are deliberately left unsigned via `-c commit.gpgsign=false`. Origin was mechanical (GPG blocks agent commits); elevated to policy (SI-4) because a signature is Şenol's authorship claim, applied even when signing would succeed; reinforced after five wave-1 merges were accidentally signed.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 "Friction" (E0[9]) | "GPG signing blocks agent commits; standing workaround `-c commit.gpgsign=false`." |
| 2026-07-09 | decided | journal session-4-close + progress-ledger SI-4 (E5[43]) | "agent commits deliberately UNSIGNED as policy (signature = his authorship claim); gpgsign=false on every commit AND merge." |
| 2026-07-10 | violated-corrected | journal Plan-4-complete (E5[44]) | five wave-1 merges signed (unlocked gpg-agent); signed/unsigned mix spotted on GitHub; left in history, rule reinforced in HANDOFF SI-4 + Peter memory. |

---

## proc-06-mkvtoolnix-parity - mkvtoolnix parity method (SI-3)
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Because mkvtoolnix is interactive-GUI and Muxsmith is declarative-batch, only muxing semantics/output are parity targets, not input-time convenience guesses. Comparable behavior is verified against the real mkvtoolnix binary/source (not memory), and match or deliberate divergence is recorded in the memo. Licensing boundary: reading GPL source for behavior/facts/interfaces is MIT-compatible, literal expression is not, and modeled wordings are recorded as explicit memo decisions.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | Plan 3.5 memo Origin/Grounding + journal (standing method, HANDOFF SI-3) (E4[32]/[33]) | interactive-GUI vs declarative-batch distinction; only muxing semantics/output are parity targets; formalized as SI-3. |
| 2026-07-10 | reinforced | memo D30/D31 (E6[47]) | cites `MainWindow::beforeCloseCheckRunningJobs ... main_window.cpp:492-548` directly as parity evidence. |
| 2026-07-10 | decided | journal session-6 close + HANDOFF SI-3 (E6[56]) | licensing boundary: behavior/facts/interfaces yes, literal expression no; modeled wordings recorded as explicit memo decisions. |
| 2026-07-12 | decided | Plan 5.5 Global Constraints + d32-analysis.md SI-3 + task-6-verdict.md (E7[66]) | "match mkvtoolnix's permissiveness at the data layer, diverge deliberately at the rule layer where Muxsmith has a hazard it does not" (D32, T6). |

---

## proc-07-verify-against-source - Verify tooling/dependency behavior against source, not brief/docs/memory
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** Load-bearing tooling and dependency behavior is confirmed against the crate's actual source or the registry, never from a brief's claim, a docs sentence, or training-data memory.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | reinforced | task-5-review-verdict.md + journal Task 5 (E4[34]) | corrected a brief doc-comment (`xx-YY` canonicalizes to itself, not `None`) by cross-checking the crate's Suppress-Script table. |
| 2026-07-10 | decided | journal Plan 5 (what the process caught) (E6[49]) | eslint version miss -> "registry-verify-everything discipline" (resolve every dep version against the registry). |
| 2026-07-12 | reinforced | journal session-8 close (E8[88]) | docs sentence on `devCsp` implied dev injection, crate source disproved it; CSP `connect-src 'self'` caught against official docs (would break IPC). |

---

## proc-08-parallel-worktrees - Independent streams run as parallel git worktrees
- **kind:** pattern | **status:** settled | **count:** 4 | **promoted:** yes (at 3)
- **Statement:** Genuinely independent task streams run as parallel git worktrees with a full gate per merge, not strictly serial. Origin was a failure (strictly-serial execution of the independent D19/D20/D21 streams was the clearest miss of Plan 3.5; SI-1 rewritten to parallelize-independent). Only truly independent streams parallelize; a real dependency chain stays serial (challenged mid-Plan-4 and held).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | violated-corrected | journal 2026-07-09 Plan 3.5 (Friction/failure) (E4[36]) | "I am waiting for something that could have been faster"; SI-1 rewritten (parallelize-independent). "The clearest miss of the session." |
| 2026-07-09 | decided | journal session-4-close (plan c0c0ef7) (E5[39]) | "wave 1 = five independent streams to run as parallel worktrees - direct consequence of the Plan 3.5 serial criticism, first parallel run for this repo." |
| 2026-07-10 | reinforced | journal Plan-4-complete (E5[40]/[41]) | merges T5->T7->T4->T6->T1, full gate per merge, zero real conflicts; mid-session challenge that parallelism was underused held (genuine dependency chain). |
| 2026-07-12 | decided | progress.md + plan dependency graph (E7[62]) | six disjoint worktree streams merged sequentially with full gate re-run; DiagCode stream merges last so its exhaustive guard forces the others' fixtures. |

---

## proc-09-idiomacy-review - Whole-codebase idiomacy review as a distinct complexity-only gate
- **kind:** pattern | **status:** settled | **count:** 3 | **promoted:** yes (at 3)
- **Statement:** A whole-codebase idiomacy review is a distinct pre-1.0 gate scoped to complexity only (correctness/security/perf routed to a separate list). The rubric is six dimensions (idiom, dup, stdlib, dep, yagni/over-abstraction, native/platform-reinvention) with a ranked one-line-per-finding output contract; executed with many finders + seed-verification + a code-level dedup barrier + one adversarial verifier per finding.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | journal session-8 close (E8[86]) | "New pre-1.0 gate: whole-codebase idiomacy review after Plans 5.5/6." |
| 2026-07-12 | decided | commit b535038 + ponytail-mining.md candidate 2 (E8[91]) | rubric gains yagni + native axes and the one-line-per-finding contract (four -> six dimensions); correctness/security/perf stay out of scope. |
| 2026-07-12 | decided | journal session 10 + idiomacy-review-findings.md header (E8[96]) | executed six-dimension review; 11 finders + 13 seed verifications + code-level dedup barrier + one adversarial verifier per finding. |

---

## proc-10-verify-rerun - Controller independently re-runs every suite / the full gate
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** The controller independently re-runs every test suite / the full gate with per-part accounting rather than trusting subagent-reported counts or piped/tailed output that swallows exit codes. (Also embedded as a component of the SDD loop at Plan 3.5 and Plan 5.5; only the two primary-subject touchpoints are counted here to avoid double-crediting proc-01.)
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 evidence-integrity + verdicts/task-3-review-verdict.md (E0[5]/E1[15]) | haiku implementers mis-totaled workspace test counts in 5 of 13 reports (code correct each time); one synthesized transcript caught -> controller re-runs every suite. |
| 2026-07-12 | violated-corrected | journal session-8 close (E8[87]) | first eight-part gate run piped outputs through `tail`, swallowing exit codes; re-run with accounting; "same defect class as the audited 5/13 mis-totals." |

---

## proc-11-sdd-artifact-gitignore - Archiving SDD artifacts: the .gitignore trap; verify the commit stat
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Archiving `.superpowers/sdd` artifacts is booby-trapped: the copied tooling directory carries its own bare-`*` `.gitignore` that silently excludes everything, and its `task-N-report.md` paths are reused across plans (stale same-named reports overwritten). Read the commit stat line after any archive/copy commit and select artifacts by name.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | violated-corrected | journal 2026-07-08 "Addendum" + commit 411087f (E0[11]) | the copied sdd dir's bare-`*` `.gitignore` silently excluded all 49 artifacts; caught by reading the commit stat line, fixed in 411087f. |
| 2026-07-09 | decided | journal 2026-07-09 Plan 3 complete (Friction) (E3[30]) | every implementer overwrote a stale same-named `task-N-report.md`; salvage selected Plan-3 files by name and dropped the 2-byte `.gitignore` that had nearly lost Plan 1's artifacts. |

---

## proc-12-plan-split - Pure layer banked+reviewed before the process layer before the GUI
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** The build is split so the pure, fully golden-testable layer is banked green and independently whole-branch-reviewed before the riskiest, least-testable process layer, with the GUI last (Plan 3 = resolution+command, Plan 4 = executor+run+queue, Plan 5 = GUI). The same layering rationale slotted Plan 3.5 (pure-layer mkvtoolnix-parity fixes) before Plan 4.
- **Steelman:** A single combined ~20-task plan avoids one extra plan boundary and a second whole-branch review; the memo names this as the accepted tradeoff.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | memo D7 (specs/2026-07-09-plan-3-design-decisions.md) + journal Plan 3 complete (E3[24]/[25]) | "pure layer testable without a running mkvmerge; bank it green and reviewed before process management"; Şenol chose split over one combined plan; GUI slid to Plan 5. |
| 2026-07-09 | decided | Plan 3.5 memo Status/Scope + journal (E4[31]) | "Plan 3.5 is a set of mkvtoolnix-parity fixes to the pure layer, slotted BEFORE Plan 4 ... same rationale as D7." |

---

## proc-13-roadmap-forward-tracker - ROADMAP as the living forward slot; history is not backlog
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Deferred/forward work lives in a living ROADMAP.md, the single forward slot, distinct from the frozen archives (spec = contract, memos = frozen, IDEAS = unbuilt, journal = history). Durable-as-history is not durable-as-backlog; ROADMAP items are discussion anchors, not execution licenses.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | journal session-6 close + commit c9bd6b4 (E6[57]) | "the forward slot was empty and HANDOFF was silently lossy for it." |
| 2026-07-12 | decided | journal session 7 pt2/pt5 (E8[74]) | audit root finding "frozen archives treated as backlog"; doctrine names the history/backlog split explicitly. |

---

## proc-14-plan-precision - Plans authored at maximal precision to keep task seams drift-free
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Plans are authored at maximal precision: verbatim interface code consumed by downstream tasks, and anchor-precise specs (exact files/line anchors, named tests). Double-edged: verbatim implementation text can carry real defects into review scope; an underspecified task is a DONE-blocking question, not a license to guess.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | reinforced | journal Plan-4-complete (E5[42]) | verbatim interfaces kept four task seams drift-free; verbatim implementation text carried two real defects (watcher deadlock, Windows kill mapping) into review scope. |
| 2026-07-12 | decided | Plan 5.5 Style note line 13 (E7[65]) | anchor-precise specs, design-gated packages running TDD; "if a task feels underspecified ... that is a DONE-blocking question, not a license to guess" (Şenol-approved). |

---

## proc-15-deferral-vehicle - Every deferral carries a named vehicle / reactivation trigger
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Every deferral names the vehicle/reactivation trigger that will bring it back, so a deferral cannot silently become permanent.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | journal session 7 pt5 (D18 root cause) (E8[78]) | audit root: a "cleanup pass" deferral target without a vehicle (D18). |
| 2026-07-12 | reinforced | journal session 9 (E8[78]) | "16 deferred with named vehicles." |

---

## proc-16-spec-code-coherence - Shipped diagnostics/severities must match the spec; whole-branch review catches drift
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** Shipped diagnostics, severities, and catalog rows must match the spec; the whole-branch review catches spec-vs-code drift that per-task reviews cannot (each side was locally consistent), corrected in the close wave.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | whole-branch-verdict.md C1 + whole-branch-fix-report.md (E7[70]) | spec recorded `SchemaDrift` as warning against the owner's info ruling (introduced by the T16.5 review's own amendment ce4fae1); "one word in two places." |
| 2026-07-12 | violated-corrected | whole-branch-verdict.md I1 + progress.md f25b02d (E7[71]) | §5.2 catalog missing EmptyPlan/UnknownExtension/WorkerPanicked (I1), plus a fourth (UnsupportedSource) surfaced by the fixer and added via f25b02d. |

---

## proc-17-plan-sequencing - Correct the plan's declared task graph against real overlaps and gates at dispatch
- **kind:** pattern | **status:** settled | **count:** 2 | **promoted:** no
- **Statement:** The plan's declared task graph (disjointness notes, parallel/serial assignments) is corrected against real file overlaps and human gates at dispatch, not trusted blindly.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | progress.md RESEQUENCED + journal Decisions (E7[63]) | the plan's disjointness note was wrong for T9 (file list overlaps stream B); "T9 runs SERIALLY after stream B merges." |
| 2026-07-12 | decided | progress.md + journal Decisions (E7[64]) | "T21 and T22 run PARALLEL ... T22 (EN-pinned snapshots) may merge before T21" so the hard human terminology gate does not block the tail; T19-before-T22 order preserved. |

---

## proc-18-git-authorization - Agent commits/pushes authorized for this repo; global no-commit rule reversed
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Agent git commits and pushes are authorized for this repo specifically, carrying a Co-Authored-By trailer; the global "no unrequested commits" rule was applied mid-session then explicitly reversed for this repo.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | handoff plan-1-close Git section + journal 2026-07-08 friction (E0[10]/E1[17]) | "commits and pushes are AUTHORIZED for this repo"; "Mid-session rule change (no unrequested commits) later explicitly reversed for this repo." |

---

## proc-19-gh-log-audit - gh-log.md audit for every GitHub interaction
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Every GitHub interaction (gh CLI, API call, push) is recorded in a git-ignored operational `gh-log.md`; nothing that incurs cost is run while the repo is private.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 Friction + commit fad067c + handoff plan-1-close GitHub section (E0[8]/E1[18]) | "every interaction (gh, API, push) gets an entry in gh-log.md ... Nothing that costs money"; fad067c "ignore gh-log.md (operational log, not repo content)." |

---

## proc-20-license-mit - Shipped under MIT over Apache-2.0
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Licensed MIT over Apache-2.0: permissive, allows commercialization by anyone including the author, consistent with the Ruby prototype.
- **Steelman:** Apache-2.0 adds an explicit patent grant the MIT text lacks; rejected in favor of MIT's simplicity and prototype consistency.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | spec §2 + §12 + journal 2026-07-08 + Plan 1 Global Constraints + commit 61249f9 (E0[1]/E1[12]) | "MIT | Permissive, commercialization by anyone including the author; consistent with the Ruby prototype." |

---

## proc-21-deny-missing-docs - deny(missing_docs) on public API; blanket private-doc lint rejected
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** All library crate roots carry `#![deny(missing_docs)]` requiring semantics-not-name rustdoc on public items; private items are documented by judgment only.
- **Steelman:** Documenting every private item too would maximize coverage and consistency; rejected because it produces comment-noise restating obvious names.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | journal 2026-07-08 Decisions + rustdoc-backfill-report.md + commit c402914 (E1[13]) | "deny(missing_docs) for public API; private items documented by judgment only (blanket private-doc lint rejected as comment-noise)." |

---

## proc-22-fix-wave-hygiene - Fix waves: one commit per finding, flag-don't-fold adjacent issues
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Fix waves are one commit per finding, TDD where behavior changes, foreground only, scope kept within the files each finding names; adjacent issues are flagged and routed as separate accepted commits, never silently folded in.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | reinforced | final-fix-wave-report.md + whole-branch-review-verdict-round-2.md (E5[45]/[46]) | four findings one commit each, gate green per commit; "the Fix-2 implementer's scope discipline (flagging this arm instead of silently folding it in) and the coordinator routing it as a separate accepted commit is exactly how that should go." |

---

## proc-23-foreground-exec - Build/test commands run foreground, never background+Monitor
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** All cargo/pnpm build and test commands run in the foreground, never background-run + Monitor (a Plan-4 implementer stalled twice on background waits).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | reinforced | plan global constraints (E6[52]) | "a Plan-4 implementer stalled twice on background-run + Monitor waits." |

---

## proc-24-injection-untrusted - Mid-tool-stream scope changes treated as untrusted
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Mid-tool-stream scope changes are treated as untrusted per the framework's injection-handling doctrine; a legitimate scope addendum embedded in a subagent's tool stream was refused (a healthy false positive) and the controller applied the change himself.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | reinforced | final-fix-wave-plan5-report.md (flagged suspicious message) (E6[53]) | "Treated as untrusted content per the framework's injection-handling doctrine and not implemented." |

---

## proc-25-go-public - Repo taken public mid-close-out
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The repo was taken public mid-close-out: reversible, and it unlocks free CI resources for 3-OS verification.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | journal Plan 5 (decisions) (E6[54]) | "Go-public decided mid-close-out ... GitHub gives more resources; reversible." |

---

## proc-26-leak-audit - Pre-public parallel leak audit; persona name/identity/prototype ref stay public
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Before going public, a parallel multi-auditor leak audit runs over all docs (4 auditors / 258 docs, 0 secrets/PII); Şenol ruled the persona name, commit identity, and mkv-batch-tools prototype reference stay public, only a stale repo URL fixed.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | decided | journal session-6 close + commit 705f735 (E6[55]) | "0 secrets, 0 personal-data leaks ... kann ich doch nennen wie ich will." |

---

## proc-27-doc-truthfulness-adjudication - A truthfulness doc-comment fix accepted despite a scope constraint
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A trait doc-comment edit made under a "no trait changes" scope constraint was adjudicated ACCEPT as a truthfulness fix (changes no contract/algebra); reverting would reintroduce a false comment.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | task-3-review-verdict.md + progress.md Task 3 (E3[26]) | "controller adjudicated ACCEPT - it is a correct doc fix, not a contract/algebra change, and reverting would reintroduce a false comment." |

---

## proc-28-brief-self-containment - Front-loaded shared references must be appended to per-task briefs
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A per-task brief is not self-contained when the plan front-loads shared reference blocks above Task 1 (`scripts/task-brief` extracts only the per-task section); the shared references must be appended to affected briefs before dispatch.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | journal Plan 3 complete (Friction) + progress.md (E3[29]) | "briefs for Tasks 4 and 9-12 had to have the reference blocks manually appended before dispatch." |

---

## proc-29-restructure-completeness - A decided restructure is applied completely (thorough separation)
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A decided restructure (e.g. tracks -> block) is applied completely: all consumers, fixtures and inline-test YAML migrated with zero orphaned sites, verified by an independent full-repo re-scan, not just the diff.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | reinforced | task-1-review-verdict.md + whole-branch-review-verdict.md (E4[37]) | "a full-repo sweep returns zero un-migrated sites ... thorough_separation applied correctly." |

---

## proc-30-design-approval-gate - Şenol approves the design memo before implementation
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The design memo is approved by Şenol before any implementation begins.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-09 | decided | journal session-4-close + commit 7ba90ee (E5[38]) | "commit 7ba90ee (Plan 4 memo, Şenol-approved lgtm)"; memo Status FINAL 2026-07-09. |

---

## proc-31-mise-runtimes - Language runtimes via mise, not dnf/corepack/rustup
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Language runtimes are managed via mise, not dnf/corepack/rustup; the controller's dnf+corepack proposal for node was corrected, with the pin in the repo `mise.toml`.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-10 | violated-corrected | journal Plan 5 (friction) + commits cad84a3..656449c (E6[48]) | "Controller initially proposed dnf/corepack for node - Şenol corrected to mise (now a Peter memory + repo mise.toml)." |

---

## proc-32-gitattributes-lf - .gitattributes normalizes text to LF for byte-exact snapshots
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** `.gitattributes` normalizes text to LF (`text=auto eol=lf`) with per-extension `-text` overrides for binary assets, giving LF-stable files for byte-exact snapshot tests.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-1-verdict.md + plan T1 (E7[67]) | "Attribute semantics idiomatic (`* text=auto eol=lf` + per-extension `-text` overrides)." |

---

## proc-33-question-dialog - Send context as a plain message first, question after
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The question-dialog overlay swallowed pre-question context (the owner could not see what he was deciding); context is sent as a plain message first and the question after (or prose answers).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | journal 2026-07-12 Plan 5.5 (Friction and failure) (E7[72]) | "Pattern now: context as a plain message first, question after (or prose answers)." |

---

## proc-34-mechanism-not-appeal - Process rules encoded as mechanisms, not appeals
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Process rules are encoded as mechanisms rather than appeals (appeals get ignored); root-caused from the forensic audit and packaged into the software-dev-process skill.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | journal session 7 pt2 (E8[73]) | "Process doctrine from the audit root causes: mechanism-not-appeal + durable-as-history-is-not-durable-as-backlog." |

---

## proc-35-behavior-as-package - Doctrine ships as a self-contained, liftable, project-scoped skill
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The process doctrine ships as a self-contained, liftable, project-scoped software-dev-process skill; new projects get an adopt-or-not question at kickoff.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | journal session 7 pt2 (E8[75]) | "feedback_superpowers_throughout memory deleted, content integrated as doctrine section 0." |

---

## proc-36-write-at-creation - Persist a decision the moment it is made
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A decision is persisted to the ROADMAP the same turn it is made (write-at-creation), not batched at session end.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | reinforced | journal session 7 pt6 (E8[77]) | "each decision was persisted to ROADMAP in the same turn (write-at-creation practiced live before the doctrine was even tested)." |

---

## proc-37-house-knowledge-tiers - Tier-1 ledger (always-written) + Tier-2 CONVENTIONS (always-loaded), count-3 promotion
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** House-knowledge is recorded deterministically in a Tier-1 decision-ledger (always written, not always loaded) and promoted to Tier-2 CONVENTIONS (always loaded) at recurrence count 3; the controller is the single writer.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | commit b38a46f + doctrine §7 (E8[79]) | decision-ledger.md "always written, not always loaded ... promoted there when its recurrence count reaches 3." |

---

## proc-38-byte-verified-recovery - Recovered artifacts byte-verified verbatim against source transcripts
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Recovered artifacts (handoffs, verdicts) are byte-verified verbatim against the source transcripts rather than paraphrased.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | journal session 7 pt2 (E8[80]) | "byte-verified verbatim (handoffs cross-checked against next-session reads; 78 verdicts against subagent transcripts)." |

---

## proc-39-persistence-scope - Persistence scope decided case-by-case; do not invert one complaint into a universal rule
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A single concrete complaint is not inverted into an absolute always/never rule; persistence scope is decided case-by-case, asking when unclear (black-and-white generalization is a defect).
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | journal session 7 pt5 (E8[81]) | "Şenol's criticism of the controller: inverting a concrete complaint into a universal rule (black-and-white thinking)"; memory convention amended. |

---

## proc-40-readme-voice - README in sell-tone with personality; case-scoped exception to neutral writeup voice
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The README is written in a sell-tone with personality, a WIP banner, and the AI-collaboration story told openly, a deliberate case-scoped exception to the neutral writeup voice rule.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | journal session-8 close (README 62aaf61) (E8[84]) | "a deliberate, case-scoped exception to the neutral writeup voice rule." |

---

## proc-41-idiomacy-directive - Three-part idiomacy directive replaces "minimize runtime dependencies"
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A three-part idiomacy directive (ecosystem-idiom check + reuse-before-writing + dependencies-earned-both-directions) replaces the misinterpretable "minimize runtime dependencies" and governs code written after it.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | decided | journal session-8 close (E8[85]) | "replaces the misinterpretable minimize runtime dependencies." |

---

## proc-42-readme-magic-anchor - Properties carrying matching magic must be listed in the README
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Properties that carry language-like matching magic (normalization, dual-field lookup, absent-boolean-compares-false, curated domains) must be explicitly listed in the README, a standing directive from the B-8 no-magic ruling, recorded as a README content anchor in the ROADMAP.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | memo D32 addendum follow-on directive (Şenol) (E7[58]) | "properties that carry language-like matching magic ... MUST be explicitly listed in the README - recorded as a README content anchor in the ROADMAP." |

---

## proc-43-ride-existing-plan - Ride a decision's implementation on an existing plan as a task, not a standalone run
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** An approved decision's implementation rides an existing plan as a wave task (D35 as Plan 5.5 task 4.5) rather than spinning a standalone run: one SDD apparatus over a separate run.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | plan T4.5 + journal 2026-07-12 Plan 5.5 (E7[59]) | "ride the plan as wave-1 task 4.5, one SDD apparatus over a standalone run." |

---

## proc-44-handoff-snapshot - Every HANDOFF rewrite snapshots the new state in the same turn (SI-5)
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Every HANDOFF rewrite snapshots the new state to `artifacts/handoffs/` in the same turn (SI-5); the rule moved from intro prose to a standing instruction after a session-close state was overwritten without snapshotting.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | journal session-8 addendum (E8[89]) | "the session-8 HANDOFF rewrite overwrote the session-7-close state without snapshotting it - the exact loss class the recovery effort existed to fix." |

---

## proc-45-bulk-agent-sizing - Bulk-agent fan-out: coarser agents, budget-guarded, in-flight cap 30
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Bulk agent jobs use coarser agents that batch many findings per dispatch and size total fan-out to the available budget (cap in-flight at 30), rather than one agent per finding.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | violated-corrected | journal session 10 lesson (E8[97]) | "one agent per finding meant ~73 verifier dispatches ... the fix is not lower concurrency alone but COARSER agents ... Cap in-flight at 30." |

---

## proc-46-bulk-run-resilience - Bulk runs stay limit-resilient: artifact-at-creation, capture-before-verdict, resume-from-cache
- **kind:** pattern | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Bulk runs stay limit-resilient: each agent writes its report at creation, the workflow captures each finding before its verdict, and a resume replays cached results, so a usage-limit wall leaves a clean re-runnable worklist, not data loss.
- **Steelman:** null
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | reinforced | journal session 10 (E8[98]) | "the workflow captured each finding BEFORE its verdict ... the failures were a clean re-runnable worklist, not data loss." |

---

## proc-47-just-rejected - `just` task runner rejected (xtask covers every dev task)
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** No `just` task runner adopted; xtask already covers every dev task and a second entry point drifts.
- **Steelman:** `just` is the ergonomic, discoverable task runner most Rust projects reach for; xtask is more boilerplate.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-08 | decided | BUILDING.md "Deliberately not used" (Plan-1 stock-take) (E0[2]) | "just runner: xtask covers every dev task; a second entry point drifts." |

---

## proc-48-docsurface-delink - Delink rather than grow the public surface for an intra-doc link
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** An intra-doc link to a private item (`on_close_requested`) was satisfied by delinking, not by publicizing it or adding a pub-use re-export: growing the public surface to satisfy a doc link inverts the relationship (docs describe architecture, they do not dictate it).
- **Steelman:** A pub-use re-export would keep the intra-doc link working without exposing the whole `run` module.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | task-12-verdict.md judgment (E7[60]) | "growing public surface to satisfy an intra-doc link inverts the relationship (docs describe architecture, not dictate it)." |

---

## proc-49-eager-memory-rejected - Eager-memory ambience rejected for carrying the process behavior
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Carrying the process doctrine as always-loaded eager persona-memory ambience was rejected by Şenol in favor of the packaged, adopt-or-not skill.
- **Steelman:** Always-loaded memory keeps the behavior present every session without an explicit adopt step.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | journal session 7 pt2 (E8[76]) | "after Şenol rejected eager-memory ambience: behavior-as-package." |

---

## proc-50-ponytail-ladder-rejected - ponytail's seven-rung laziness ladder rejected wholesale
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Importing ponytail's full seven-rung laziness decision ladder wholesale as a new rule block was rejected as redundant with the existing idiomacy directive and scale rule (ours is bidirectional on deps; the inbound ladder isn't).
- **Steelman:** A compact reuse/stdlib/native/dep laziness ladder, vividly phrased, as a single pre-write checklist.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | ponytail-mining.md REJECTED (E8[92]) | "Adopting the ladder wholesale would duplicate our set to look busy." |

---

## proc-51-aggressive-minimalism-rejected - "Shortest diff wins" aggressive-minimalism framing rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** The aggressive-minimalism "laziest thing that works / shortest diff wins" framing and its lite/full/ultra intensity levels were rejected as inverting the correctness > precision > maintainability > simplicity ordering.
- **Steelman:** Aggressively minimizing code reliably cuts over-build; the benchmark shows real wins on over-build-trap features.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | ponytail-mining.md REJECTED (E8[93]) | "our order is correctness > precision > maintainability, then simplicity ... Shortest diff wins as a headline inverts that priority." |

---

## proc-52-cornercut-ledger-rejected - Grepped corner-cut comments + debt ledger rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** Grepped in-code corner-cut markers with a `/ponytail-debt` grep ledger were rejected as redundant with software-dev-process's HITL deferral record.
- **Steelman:** A grepped marker with an explicit upgrade trigger prevents a deliberate corner-cut from silently becoming permanent.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | ponytail-mining.md REJECTED (E8[94]) | "redundant with software-dev-process, which already tracks deferrals/rejections/findings as a HITL record." |

---

## proc-53-shrink-axis-rejected - Shrink / line-golf review axis rejected
- **kind:** restraint | **status:** settled | **count:** 1 | **promoted:** no
- **Statement:** A shrink / line-golf (same-logic-fewer-lines) review dimension was rejected as noise for a pre-1.0 idiomacy pass; value is in idiomacy/reuse/dead-abstraction, not counting lines.
- **Steelman:** Line-count reduction is a cheap, objective signal that pairs naturally with the other cut-oriented axes.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | decided | ponytail-mining.md REJECTED (E8[95]) | "line-golf is noise in a pre-1.0 idiomacy review; the value is in idiomacy/reuse/dead-abstraction, not counting lines." |

---

## proc-54-doc-deliverables-deferred - 1.0 doc formats fixed, authoring deferred to the 1.0 tag
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** 1.0 documentation formats are fixed (single-file EN GUIDE.md at maximal scope + two EN+DE blog posts) but the authoring itself is deferred to the 1.0 tag via three fresh sessions.
- **Blocked on:** external - awaiting the 1.0 tag.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-11 | deferred | journal session-8 close (format interview) (E8[90]) | "two posts EN+DE written at 1.0 ... three fresh authoring sessions." |

---

## proc-55-correctness-gate-deferred - Correctness/security/perf findings routed to a separate review gate
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The 11 correctness/security/perf findings routed out of the idiomacy pass are deferred to a dedicated normal-review gate, deliberately not blended into the complexity hunt (a complexity hunt and a bug hunt want different mindsets).
- **Blocked on:** internal - the dedicated correctness/security/perf review not yet run.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | ROADMAP c38a197 + idiomacy-review-findings.md routed-out section (E8[99]) | "a complexity hunt and a bug hunt want different mindsets and shouldn't be blended." |

---

## proc-56-idiomacy-fix-wave-deferred - Accepted idiomacy findings become their own SDD fix-wave plan
- **kind:** non-decision | **status:** blocked | **count:** 1 | **promoted:** no
- **Statement:** The 70 confirmed idiomacy findings await triage with Şenol; the accepted set becomes its own SDD-executed fix-wave plan (mechanical + low-risk), with the review scratch salvaged at that close.
- **Blocked on:** internal - triage of the 70 confirmed findings with Şenol.
- **Occurrences:**

| date | kind | ref | evidence |
|---|---|---|---|
| 2026-07-12 | deferred | journal session 10 open threads + ROADMAP L220 (E8[100]) | "accepted set becomes the idiomacy fix wave (own plan, SDD execution)." |

---

## Clustering notes (defensibility)

- **Counting unit = decision-event, matching the `ci` sibling** (not per-artifact as `cross-01` split it). Same-session/same-decision citations bundle into one occurrence with a concatenated `ref`; a pattern re-applied in a new plan is a distinct occurrence. This is why proc-01 (SDD) reaches 7: seven distinct plan/session touchpoints, not seven artifacts of one decision.
- **E0 and E1 are reconstruction eras, not separate events.** All E0/E1 pairs describing the same Plan-1 call (license, SDD, spec-wins, verify-rerun, gh-log, git-authorization) dedup to one 2026-07-08 occurrence, with the E1 handoff/commit citations folded into that occurrence's `ref`.
- **Plan-2 inline execution is one occurrence, not two.** Records [21] (the deviation decision) and [22] (the retrofit independent-review correction) are the same violated-corrected episode of proc-01; counting them separately would inflate.
- **proc-10 (verify-rerun) held at 2, not promoted, deliberately.** The rule is genuinely reinforced as a *component* inside the SDD records at Plan 3.5 ([35]) and Plan 5.5 ([61]) ("the controller re-runs suites/gates itself"), but those touchpoints are already counted under proc-01; crediting them again to proc-10 would double-count one record across two promoted clusters. Only the two primary-subject touchpoints (Plan-1 establish, session-8 violated-corrected) are counted.
- **proc-05 (signing) counts 3 genuine dates:** the 2026-07-08 mechanical workaround, the 2026-07-09 SI-4 policy elevation (signature = authorship claim - a *different* decision than the workaround), and the 2026-07-10 violated-corrected episode. Not one decision triple-cited.
- **proc-08 (parallel) counts 4:** the Plan-3.5 failure origin (07-09), the Plan-4 authoring decision (07-09, distinct ref/event), the Plan-4 execution reinforcement (07-10), and the Plan-5.5 six-stream run (07-12). The Plan-4 "was parallelism underused?" challenge ([41]) shares Plan-4-complete's date+ref and is folded in, not counted.
- **proc-06 vs proc-07 kept separate:** proc-06 is the mkvtoolnix-specific parity oracle with its licensing boundary; proc-07 is the general "read the source, not the brief/docs/memory" discipline (dependency crates, registry, official docs). Distinct `(topic, approach)`, not merged to inflate either.
- **proc-16 and proc-17 collapsed the E7 pairs by theme:** [70]+[71] are two spec-drift findings from one whole-branch review (both counted - distinct findings, distinct refs); [63]+[64] are two sequencing corrections. Neither was merged into proc-02/proc-08 to avoid crediting the same record twice.
- **proc-25 (go-public) and proc-26 (leak-audit) split:** distinct `(topic, approach)` - the reversible go/no-go call vs the pre-public audit method and what-stays-public ruling - so neither absorbs the other.
