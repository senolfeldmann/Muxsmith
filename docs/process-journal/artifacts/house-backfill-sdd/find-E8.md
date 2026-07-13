# E8 (2026-07-11/12) - process/meta era: decision-history reconstruction

Era scope: process/meta decisions, the pin-everything doctrine, the idiomacy
findings, the house-convention refute (Vue props form), plus the two non-code
design gates (D34 CSP, D35 retention). Sources: memo
`docs/superpowers/specs/2026-07-11-pre-1.0-design-decisions.md`; journal
sessions 7/8/9-close/10; `idiomacy-review-findings.md`; `ponytail-mining.md`;
seeded `docs/CONVENTIONS.md` + `docs/decision-ledger.md` (validated against the
trail - decision-ledger is an empty Tier-1 scaffold, all 8 CONVENTIONS entries
trace to real trail decisions).

One record per (topic, approach) OCCURRENCE. Recurrences (same topic/approach at
several trail points) are emitted as separate records, not pre-merged.

---

## Memo D34 - production CSP (session 8)

1. **Production CSP form :: strict explicit-directive block** - pattern/decided/gui
   - `app.security.csp` = `default-src 'none'` + explicit `script-src/style-src/img-src/connect-src/base-uri/form-action`; no devCsp, no dangerousDisableAssetCspModification.
   - occ_ref: memo D34. evidence: "The strict `default-src 'none'` base makes every future surface addition ... a conscious CSP edit instead of a silent allowance - explicit over magic."

2. **Production CSP form :: docs-idiom `'self'` baseline (string form)** - restraint/decided/gui
   - occ_ref: memo D34 rejected-alternatives. steelman: "defensible and common in well-maintained Tauri apps." why rejected: "silently allows bundle-origin loads for every unenumerated directive (media/font/worker/manifest)."

3. **Production CSP form :: a `devCsp` block** - restraint/decided/gui
   - occ_ref: memo D34 (+ journal session-8 close). steelman: initially proposed to cover dev. why rejected: source-verified vs tauri 2.11.5 that with `devUrl` set neither csp nor devCsp reaches the dev page - "the block would be dead config." (Withdrawn same day.)

4. **Production CSP form :: `csp: null` (create-tauri-app scaffold default, status quo)** - restraint/decided/gui
   - occ_ref: memo D34. steelman: the shipped status quo since Plan 5 T4. why rejected: "no blast-radius cap if an injection path ever appears (a future v-html, a dependency regression)."

## Memo D35 - run-log retention (session 8)

5. **Run-log retention :: automatic 14-day pruning, fixed in v1 (no config)** - pattern/decided/executor
   - Implemented in core (executor/joblog) so CLI+GUI both inherit; Şenol overruled the presented recommendation. occ_ref: memo D35. evidence: "for this tool class the log is needed right away or not at all - its value decays in days, and 14 days is a sane window." Parity MATCH with mkvtoolnix defaults (removeOldJobs=true, 14 days).

6. **Run-log retention :: keep-forever + explicit `prune` facility** - restraint/decided/executor
   - occ_ref: memo D35 (Peter's recommendation, overruled). steelman: "batch run logs as audit artifacts, deletion as an explicit act." why rejected: "the audit-value premise does not hold for this tool class in Şenol's judgment; unbounded history clutter is the worse default."

7. **Run-log retention :: configurable prune (disable / change days) in v1** - non-decision/deferred/executor
   - blocked_on: internal - option surface not earned; parked to IDEAS #7 pending real demand. occ_ref: memo D35. evidence: "configuration surface not earned yet; deferred to IDEAS #7."

8. **Run-log retention :: keep-forever plain (no prune, status quo)** - restraint/decided/executor
   - occ_ref: memo D35. steelman: simplest, zero mechanism. why rejected: "unbounded growth, cleanup nobody performs."

9. **Run-log pruning :: mkvtoolnix immediate-removal on completion/exit** - restraint/decided/executor
   - occ_ref: memo D35 parity note. steelman: mkvtoolnix-gui ships these additional removal policies. why rejected: "they would delete the post-mortem record spec 6 mandates persisting." (Only the 14-day default is adopted, not the immediate-removal policies.)

## Process doctrine (session 7, forensic audit)

10. **Process-rule form :: mechanism-not-appeal** - pattern/decided/process
    - Rules that are appeals get ignored; encode them as mechanisms. Root-caused from the audit. occ_ref: journal session 7 pt2 ("Process doctrine from the audit root causes: mechanism-not-appeal ..."). Packaged as software-dev-process skill.

11. **Forward-tracking :: durable-as-history-is-not-durable-as-backlog (living ROADMAP vs frozen archives)** - pattern/decided/process
    - occ_ref: journal session 7 pt2 + pt5. evidence: audit root finding "frozen archives treated as backlog"; fix = a living forward-tracker distinct from history.

12. **Doctrine delivery :: behavior-as-package (self-contained, liftable, PROJECT-scoped binding; adopt-or-not at kickoff)** - pattern/decided/process
    - occ_ref: journal session 7 pt2. evidence: "behavior-as-package ... new projects get an adopt-or-not question at kickoff. feedback_superpowers_throughout memory deleted, content integrated as doctrine section 0."

13. **Doctrine delivery :: eager-memory ambience** - restraint/decided/process
    - occ_ref: journal session 7 pt2. steelman: always-loaded persona-memory ambience keeps the behavior present without a package. why rejected: Şenol "rejected eager-memory ambience" in favor of the self-contained liftable package.

14. **Decision persistence :: write-at-creation (persist to ROADMAP the same turn the call is made)** - pattern/reinforced/process
    - occ_ref: journal session 7 pt6. evidence: "each decision was persisted to ROADMAP in the same turn (write-at-creation practiced live before the doctrine was even tested)."

15. **Deferrals :: must carry a named vehicle / reactivation trigger** - pattern/decided/process
    - occ_ref: journal session 7 pt5 (audit root: "'cleanup pass' deferral target without vehicle (D18)"). Reinforced session 9 ("16 deferred with named vehicles") and CONVENTIONS non-decisions (blocked-on named).

16. **House-knowledge recording :: Tier-1 ledger (always-written) + Tier-2 CONVENTIONS (always-loaded), count-3 promotion, controller sole writer** - pattern/decided/process
    - occ_ref: commit b38a46f + software-dev-process doctrine §7 + decision-ledger.md/CONVENTIONS.md headers. evidence: "an item is promoted here at count 3"; "the controller is the single writer".

17. **Recovery/verification :: byte-verified verbatim recovery (handoffs + verdicts cross-checked against transcripts)** - pattern/decided/process
    - occ_ref: journal session 7 pt2. evidence: "byte-verified verbatim (handoffs cross-checked against next-session reads; 78 verdicts against subagent transcripts)."

18. **Persistence scope :: case-by-case, ask when unclear (do NOT invert one concrete complaint into a universal rule)** - non-decision-corrected/violated-corrected/process
    - occ_ref: journal session 7 pt5. evidence: "Şenol's criticism of the controller: inverting a concrete complaint into a universal rule (black-and-white thinking). Correction: persistence scope is a case-by-case decision, ask when unclear." (agents-framework memory convention amended.)

19. **Execution method :: Superpowers-throughout + parallelize-independent (SI-1) folded in as doctrine section 0** - pattern/reinforced/process
    - Origin session 4 (Plan 3.5 serial-execution criticism); E8 integrates it into the packaged doctrine. occ_ref: journal session 7 pt2 ("content integrated as doctrine section 0").

20. **Spec status :: spec is a binding contract; every spec-anchored gap implemented pre-1.0** - pattern/reinforced/process
    - occ_ref: journal session 7 pt2. evidence: "every spec-anchored gap -> implement pre-1.0 (spec is a binding contract)." (Re-affirms the session-1 spec-wins rule; also the argument behind the idiomacy four-copy-pipeline hoist.)

## Session 8 (non-code pre-1.0 gates)

21. **README voice :: sell-tone with personality + WIP banner + AI-collab story told openly (case-scoped exception to the neutral writeup voice)** - pattern/decided/process
    - occ_ref: journal session-8 close (README shipped 62aaf61). evidence: "a deliberate, case-scoped exception to the neutral writeup voice rule."

22. **Dependency rule :: idiomacy directive (ecosystem-idiom check + reuse-before-writing + dependencies-earned-both-directions), replacing "minimize runtime dependencies"** - pattern/decided/process
    - occ_ref: journal session-8 close. evidence: "replaces the misinterpretable 'minimize runtime dependencies'." (framework-side, governs code written after it; feeds the Muxsmith idiomacy gate.)

23. **Pre-1.0 gate :: whole-codebase idiomacy review scheduled after Plans 5.5/6** - pattern/decided/process
    - occ_ref: journal session-8 close ("New pre-1.0 gate: whole-codebase idiomacy review after Plans 5.5/6"). Recurs at #28 (rubric) and #35 (execution).

24. **Verification discipline :: controller re-runs the full gate with per-part accounting; never trust piped/tailed output or a booked claim** - pattern/violated-corrected/process
    - occ_ref: journal session-8 close. evidence: "first eight-part gate run piped outputs through tail, swallowing exit codes ... Re-run with accounting: 32 binaries, 370 tests, all green. Same defect class as the audited 5/13 mis-totals." (Also session 7: cross-check refuted two false "meanwhile resolved" claims; SI-4's "mechanically solved" claim was false on disk.)

25. **Fact-checking discipline :: verify against crate/source, not just docs** - pattern/reinforced/process
    - occ_ref: journal session-8 close. evidence: "the docs sentence on devCsp implied dev injection; crate source disproved it for devUrl setups"; the CSP surface scan's `connect-src 'self'` (would break IPC) caught against official docs.

26. **HANDOFF snapshotting :: every HANDOFF rewrite snapshots the new state to artifacts/handoffs/ in the same turn (SI-5)** - pattern/violated-corrected/process
    - occ_ref: journal session-8 addendum. evidence: "the session-8 HANDOFF rewrite overwrote the session-7-close state without snapshotting it - the exact loss class the recovery effort existed to fix." Countermeasure SI-5 added; the rule moved from intro prose into the SI.

27. **1.0 documentation deliverables :: GUIDE.md single EN file (maximal scope) + two EN+DE blog posts, authored fresh at 1.0** - non-decision/deferred/process
    - blocked_on: external - awaiting the 1.0 tag; "three fresh authoring sessions." occ_ref: journal session-8 close (format interview with Şenol).

## Session 9 close - ponytail mining adoptions/rejections

28. **Idiomacy-review rubric :: add `yagni` + `native` axes and ponytail's one-line-per-finding output contract (four dimensions -> six)** - pattern/decided/process
    - occ_ref: commit b535038 + ponytail-mining.md candidate 2. evidence: "The review has thus six dimensions instead of four plus an one-line-per-finding contract; correctness/security/performance remain explicitly outside this pass."

29. **Shared idiomacy directive :: native-platform-before-dependency clause** - pattern/decided/cross
    - occ_ref: ponytail-mining.md candidate 1 (into `_shared/conventions.md`). evidence: A "covers stdlib<->library but never names the native-platform layer (`<input type=date>` over a lib, DB constraint over app code). Distinct axis. Adopt."

30. **Shared coding rule :: comprehension-gate on minimalism ("trace the real flow ... before choosing the simplest mechanism; the smallest change in the wrong place is a second bug")** - pattern/decided/cross
    - occ_ref: ponytail-mining.md candidate 3. "Best single idea in the whole system"; generalizes the diagnosis-scoped verify_topology to the building side.

31. **ponytail rule set :: adopt the seven-rung decision ladder wholesale as a rule block** - restraint/decided/process
    - occ_ref: ponytail-mining.md REJECTED. steelman: a compact reuse/stdlib/native/dep laziness ladder. why rejected: "reuse/stdlib/dependency/no-premature-abstraction are already forced by A+B, more completely ... Adopting the ladder wholesale would duplicate our set to look busy."

32. **Minimalism framing :: "laziest thing that works" / aggressive-minimalism persona / lite-full-ultra intensity levels** - restraint/decided/process
    - occ_ref: ponytail-mining.md REJECTED. steelman: "shortest diff wins" produces less code. why rejected: "our order is correctness > precision > maintainability, then simplicity ... 'Shortest diff wins' as a headline inverts that priority."

33. **Deferral tracking :: `ponytail:` corner-cut comments + /ponytail-debt grep ledger** - restraint/decided/process
    - occ_ref: ponytail-mining.md REJECTED. steelman: a deferral cannot silently become permanent (grepped markers with upgrade triggers). why rejected: "redundant with software-dev-process, which already tracks deferrals/rejections/findings as a HITL record (better fit than grepping code comments)."

34. **Review tag :: `shrink` / line-golf axis** - restraint/decided/process
    - occ_ref: ponytail-mining.md REJECTED. steelman: same logic in fewer lines. why rejected: "line-golf is noise in a pre-1.0 idiomacy review; the value is in idiomacy/reuse/dead-abstraction, not counting lines."

## Session 10 - idiomacy review executed

35. **Idiomacy-review design :: six dimensions (idiom/dup/stdlib/dep/yagni/native), complexity-only scope, correctness/security/perf routed out; 11 finders + 13 seed-verifies + code-level dedup barrier + one adversarial verifier per finding** - pattern/decided/process
    - occ_ref: journal session 10 + idiomacy-review-findings.md header. evidence: "Six dimensions ... correctness/security/perf explicitly out of scope and routed to a separate list."

36. **Bulk-agent fan-out :: coarser agents (batch many findings per verifier), sized to budget with budget-guarding, in-flight cap 30** - non-decision-corrected/violated-corrected/process
    - occ_ref: journal session 10 ("Lesson (Şenol's correction, recorded as a rule)"). evidence: "one agent per finding meant ~73 verifier dispatches ... the fix is not lower concurrency alone but COARSER agents ... Cap in-flight at 30." Two distinct limit classes (per-model Fable quota vs rolling session limit).

37. **Bulk-run resilience :: artifact-at-creation + capture-finding-before-verdict + resume-from-cache (nothing lost across two usage-limit walls)** - pattern/reinforced/process
    - occ_ref: journal session 10. evidence: "every completed agent had already written its report to disk (artifact-at-creation), and the workflow captured each finding BEFORE its verdict ... the failures were a clean re-runnable worklist, not data loss."

## CONVENTIONS.md seeds (validated + their trail occurrences)

38. **One GUI report document shape (`config_only`/`batch`/`run_document` via report::json, one frontend render path)** - pattern/reinforced/gui
    - Origin Plan 5 T2. occ_ref: CONVENTIONS.md Patterns (b38a46f) + idiomacy finding run.rs:L249 ("report::json documents were already hoisted for this reason (Plan 5 T2), orchestration was left behind").

39. **Diagnostics through the catalog (construct Diagnostic/DiagCode, render via Fluent; never format a diagnostic string inline)** - pattern/reinforced/core
    - Origin Plan 1/2. occ_ref: CONVENTIONS.md Patterns (b38a46f).

40. **Error-first ordering everywhere (Reverse(severity), shared `severity_sorted`/one fold)** - pattern/reinforced/cli
    - Origin Plan 1. occ_ref: CONVENTIONS.md Patterns (b38a46f) + idiomacy finding validate.rs:L19 (a re-implementation of the shared sort/exit-fold flagged as a deviation to correct).

41. **Vue props: `defineProps` + `props.x` (all prop-taking SFCs conform; a lone deviation is the outlier)** - pattern/reinforced/gui
    - Origin Plan 5. occ_ref: CONVENTIONS.md Patterns (b38a46f).

42. **Vue props form :: reactive-props destructure** - restraint/violated-corrected/gui  [THE house-convention refute]
    - occ_ref: idiomacy-review-findings.md Refuted, BatchView.vue:L53. A finder proposed switching `withDefaults(defineProps<...>(), ...)` to reactive-props destructure (toolchain supports it, Vue 3.5.39); verification REFUTED it. steelman: "terser, 3.5-supported." why killed: it would make the component the lone outlier vs its siblings - internal consistency wins over an individually-idiomatic deviation.

43. **Vue props form :: reactive-props destructure** - restraint/reinforced/gui
    - occ_ref: CONVENTIONS.md Restraints (b38a46f) - the refute materialized into a standing restraint. evidence: "The steelman for destructure (terser, 3.5-supported) lost to internal consistency. (Idiomacy review 2026-07-12, refuted finding.)"

44. **Shared test helpers via `tests/support/mod.rs` (cross-file helpers in the support submodule; same-crate duplication is a defect)** - pattern/reinforced/testing
    - Origin Plan 1-4. occ_ref: CONVENTIONS.md Patterns (b38a46f) + idiomacy finding run_cli.rs:L498 (same-crate verbatim `fake_mkvmerge_that_fails_queries` dup flagged, with an in-code soft counter-preference - Şenol to decide).

45. **Bilingual Fluent (new/changed user-facing messages land in en+de in the same change; parity gate enforces it)** - pattern/reinforced/i18n
    - Origin Plan 5.5 (T19/T20 cross-locale parity gate). occ_ref: CONVENTIONS.md Patterns (b38a46f).

46. **Pin everything (exact version pins for toolchain+JS deps, SHA-pinned GitHub Actions; a floating version is a defect)** - pattern/reinforced/ci
    - Origin Plan 5 (Şenol's pin-everything doctrine, session 5/6). occ_ref: CONVENTIONS.md Patterns (b38a46f). Reinforced by idiomacy findings (rustup-show non-install idiom; Cargo resolver "2" vs edition-2024 default "3"; deny.toml dead version field) and contrasted by the routed-out mise-action floating-binary finding.

47. **No mkvtoolnix input-convenience guesses (filename-derived lang/flags, auto-title, unique-name suffixing, sequence auto-append NOT emulated; declarative-batch, the profile is the spec)** - restraint/reinforced/core
    - Origin Plan 3.5 audit / SI-3 (session 4). occ_ref: CONVENTIONS.md Restraints (b38a46f) + docs/IDEAS.md 1-4. steelman: mkvtoolnix pre-fills these guesses for the user to review. why not: Muxsmith is declarative-batch with no per-file review.

48. **mise is a dev tool, not a CI tool (CI must not fetch a floating mise binary at run time)** - restraint/decided/ci
    - occ_ref: CONVENTIONS.md Restraints (b38a46f) + idiomacy-review-findings.md routed-out ci.yml:L73 (jdx/mise-action without a version pin downloads latest at run time - contradicts pin-everything). steelman: the mise-action is convenient and currently green. why constrained: violates pin-everything; removal is tracked.

49. **mise-in-CI removal (drop the floating mise binary from CI)** - non-decision/deferred/ci
    - blocked_on: internal - deferred post-1.0 (mise-in-CI kept until release stabilizes). occ_ref: CONVENTIONS.md Restraints note + ROADMAP c38a197 ("post-1.0 mise-out-of-CI"); supersedes the routed pre-1.0 item.

50. **Injectable-planner-seam interface (S4/S5/S6)** - non-decision/deferred/core
    - blocked_on: internal - Plan 6 profile-editor design (a shared `plan_pipeline()` IS the seam; the four-copy pipeline hoist is folded into Plan 6). occ_ref: CONVENTIONS.md Non-decisions (b38a46f) + ROADMAP L37 ("never-decided injectable-planner-seam interface question").

51. **Four-copy planning pipeline hoist (one core `plan_pipeline`) :: idiomacy-wave vs fold into Plan 6** - non-decision/deferred/core
    - blocked_on: internal - Şenol to decide idiomacy-fix-wave vs Plan 6 (same seam as #50). occ_ref: idiomacy-review-findings.md run.rs:L249 (biggest cut, ~100 lines across cli dry_run.rs/run.rs + src-tauri lib.rs/run.rs; "spec 5.5/7 mandate the copies stay behaviorally identical, which is the argument FOR one shared implementation").

## ROADMAP idiomacy triage (c38a197)

52. **Routed-out correctness/security/perf items :: a separate normal-review gate (deliberately not blended into the complexity hunt)** - non-decision/deferred/process
    - blocked_on: internal - the dedicated correctness/security/perf review not yet run (release-blocking triage). occ_ref: ROADMAP c38a197 ("Correctness/security/perf review of the idiomacy pass's 11 routed-out items ... deliberately not blended") + idiomacy-review-findings.md routed-out section. evidence: "a complexity hunt and a bug hunt want different mindsets and shouldn't be blended."

53. **Idiomacy fix wave :: accepted findings become their own SDD plan (mechanical + low-risk)** - non-decision/deferred/process
    - blocked_on: internal - triage of the 70 confirmed findings with Şenol; salvage `.superpowers/sdd/idiomacy-review/` at that close. occ_ref: journal session 10 open threads + ROADMAP L220. evidence: "accepted set becomes the idiomacy fix wave (own plan, SDD execution)."
