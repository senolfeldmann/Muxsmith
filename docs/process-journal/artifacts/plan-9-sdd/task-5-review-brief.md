# Task 5 review brief - Plan 9

**Role:** independent reviewer of Plan 9, Task 5 (central errors-first sort:
`severity_sorted` hoisted from the CLI into core and applied to
`config_diagnostics` in both JSON builders; `BatchView.vue` fetches the
parse-error diagnostic by code; the enumerated parse-failure apply e2e
scenario; spec S-7). You did not write this code. Model tier: mid (dispatch
model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus
the same content as your final message.

## Preamble

- No session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a hash.** The task is one commit, `e134fdc`; the house
  commit `44a2010` landed just before it. Grade the current tree.
- **Independent instruments** at
  `/tmp/claude-1000/-home-senol-agents-peter/f3f59563-e804-4657-853b-2a25af50ea15/scratchpad/t5rev-independent/`
  (create it). Never re-run an instrument the implementer wrote, never a shared
  default path, and never a path its report names. Any absence check needs its
  own fire.
- The local `grep` is **ugrep 7.5.0**: `\b` plus bounded repetition under `-E`
  returns zero silently. Use `-P` or a script, and fire every absence check
  before believing it.
- If you mutate anything, restore non-interactively (`git checkout --`) with a
  baseline taken first and prove the restoration. A bare `cp` here is aliased
  interactive and can hang with the tree still mutated.

## Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (this task amends
   S-7).
2. The Plan-9 design: **D102** in full (site, stability contract, scope
   boundary, consumers sweep) and **D103** in full, including the amendment-1
   producer paragraph that enumerates the e2e scenario down to its fixture and
   both assertions; section 0 note 2; design section 5's sort/fetch bullets and
   two-scenarios bullet; the `## Amendment log` at its current state.
3. The plan's **Task 5** (Files list, Steps 1-8, "Must not decide").
4. The four house-knowledge YAMLs; cite ids, re-verify any `:line`.

The implementer's brief and its report (`task-5-report.md`) are evidence, not
ground truth.

## Dimensions

1. **Contract compliance where the design fences text character-for-character**:
   D102's doc comment on the hoisted function, D103's `find` line, the Step-3
   profile YAML, the Step-5 scenario's document fields and both assertions, and
   the S-7 sentence as the design's section-3 fence writes it. Build your own
   comparison rather than reading the report's.
2. **The hoist's semantics, not just its text.** The CLI definition is gone and
   a re-export stands in its place; satisfy yourself that every pre-existing
   call site (human rendering in `commands/mod.rs`, `dry_run.rs`, `validate.rs`,
   `run.rs`) behaves identically, and that the sort reaches exactly
   `config_diagnostics` in `batch_document` and `config_only_document` and
   nothing else - D102's scope boundary keeps per-file `diagnostics` and
   `batch_diagnostics` in collection order.
3. **The unenumerated third Rust test is the centre of this review.** The
   implementer measured a gap (with `batch_document`'s sort removed the whole
   workspace stayed green), applied the owner's four-condition precedence rule
   and BUILT a producer the plan does not list. Re-measure that gap yourself
   with your own mutation and your own restore proof, then rule on both halves:
   did the gap exist as claimed, and does the test actually close it (fire it
   against the same mutation)? Adjudication question 1.
4. **The e2e scenario's colour claims.** The plan states it passes on today's
   tree and carries no red-today claim; its discriminating power is against a
   defective code-keyed rewrite. Verify both directions with your own harness:
   green on the pre-edit positional fetch, red under a mutated predicate. A
   scenario that stays green under every mutation is a finding.
5. **Latitude, both forms, including the inverse** - did the implementer
   resolve at the keyboard something that should have returned? Its nine named
   divergences and six numbered concerns are where to look hardest; several are
   composed prose in files the design fences only partially.
6. **House dimension**: Tier-2 conformance, in particular
   `latitude-carveout-zero-content-structural-forks` **as amended today**
   (the file-vs-within-file boundary, and the new import case: adding a symbol
   import the task's own enumerated addition requires, inside a listed file,
   where the addition would not compile without it - the implementer added
   `use std::cmp::Reverse;` in core and deleted it in the CLI);
   `tests-ship-with-the-feature-never-after` and its execution-time four
   conditions; `an-import-removal-sweeps-the-doc-links-that-named-the-symbol`
   (the CLI lost a `use` line); `proc-normative-count-recomputed` including its
   callers'-docs facet.
7. **The no-work-needed check**, standing and unusually load-bearing here: this
   report concludes several times that something needs no work. Run each
   premise, do not weigh it - the GUI display-order consequence dismissed as
   "vacuous with respect to this diff" because e2e fixtures never pass through
   core; D102's scope boundary left unguarded as "preserved behaviour, not a
   consequence this diff creates"; the parity test's assertion pair called
   "self-guarding against vacuity"; the omitted `mkvmerge_found` key called the
   faithful mirror of what core emits on that path; and the claim that no plan
   or design line states a count of this task's tests.
8. **Verification quality**: re-run the full Step-7 bar yourself
   (`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
   `cargo test --workspace`; `pnpm lint`; `pnpm build`; `pnpm test:e2e`) and
   recompute every aggregate the report states. Two specifically: it reports
   **34 test binaries** where Task 4's report reported 39, and it reports
   **64 e2e passing** against a pre-existing 63. Both may be right; neither is
   taken on trust.

## Adjudication questions (one explicit verdict each, not pre-rated)

1. **The unenumerated third Rust test**
   (`dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`).
   Was building it correct under the owner's four-condition execution-time
   precedence, or should the measured `batch_document` gap have returned as
   NEEDS_CONTEXT? Rule on the conditions individually as well as on the
   outcome.
2. **The re-export's position and its doc comment.** It sits in the import
   region with three composed lines of rustdoc, rather than bare at the deleted
   function's old position. The design specifies "zero wrapper code" and says
   nothing about a doc comment. Correct, or a deviation to remove?
3. **The composed doc paragraphs on both builders in `json.rs`**, stating that
   `config_diagnostics` is sorted and why the per-file arrays are not. D102 asks
   the non-uniformity be recorded so it reads as a decision; the implementer
   chose the code as the site. In scope, or documentation the design did not
   sanction?
4. **The parity test's assertion set**: exactly the plan's two enumerated
   assertions, with no exit-code pin, the exit status travelling in the
   JSON-parse panic context. Sufficient, or does the missing exit assertion
   leave a real failure mode silent?
5. **The e2e fixture omitting `mkvmerge_found`.** The design enumerates the
   other fields and not this one; the TS type has it optional; the sibling
   fixtures in that describe all set it. Faithful mirror of core's output, or a
   fixture that diverges from its neighbours without a stated reason?
6. **BatchView's else-branch text left untouched.** After the code-keyed
   `find`, that branch fires for strictly more envelopes than before, so its
   comment and its `console.error` string now describe only one of its
   triggers. The implementer left both, citing the plan's "the existing
   else-branch `console.error` stays" and amendment 3's precedent that a text
   falsified by a change is a design matter. Leave as is, or route a
   correction?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/task-5-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with file:line, the evidence you ran and the exact
required change; the six adjudications; an evidence appendix naming your
instrument paths; and a HARVEST including what Tasks 6 and 7 must carry and any
observation worth a ledger entry.
