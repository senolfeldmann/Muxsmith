# Task 4 review brief - Plan 10

**Role:** independent reviewer of Plan 10, Task 4 (W5: the user-facing
documentation pass - `README.md` against the shipped CLI surface, the
exact-typed-matching paragraph plus its four-item matching-magic list, the two
counts in the "How this got built" paragraph, and `docs/INSTALL.md`'s
unsigned-package warning). You did not write this. Model tier: mid (dispatch
model: Opus 5). Effort: xhigh.

**You commit nothing and edit no product file.** Output: a verdict file plus the
same content as your final message.

## Preamble (binding)

- Never call session-relocation tools; absolute paths; **foreground runs only**.
- **Read the files, not a commit hash.** The tree is at `e657263`.
- **Independent instruments** under
  `/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t4rev-independent/`
  (create it). Never re-run an instrument the implementer wrote; never a shared
  default path. **Derive the CLI surface from the BINARY yourself** - build or
  locate `target/debug/muxsmith` and capture its help output with your own
  commands - rather than reading the report's pasted table.
- If you mutate anything, baseline first, restore non-interactively, prove it.
  A bare `cp` is aliased interactive here.
- This shell is **zsh**: `${PIPESTATUS[0]}` is empty. Capture exit codes
  directly.
- The tree must be byte-identical to `e657263` when you finish. Prove it.

## Ground truth, in precedence order

1. **The shipped binary and the code**, for every claim about the CLI surface:
   `crates/muxsmith-cli/src/cli.rs` (`Cli`'s doc comment),
   `crates/muxsmith-cli/src/commands/mod.rs` (`severity_exit`),
   `crates/muxsmith-cli/src/commands/run.rs` (`job_exit_code`).
2. The v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`),
   sections 4 and 5, for the matching claims - **note that the controller has
   already recorded one place where the spec itself is stale (section 8.1's
   synopsis omits `validate`'s flags), so where spec and binary disagree about
   the SURFACE, the binary is what the README documents.**
3. `crates/muxsmith-core/src/matcher.rs` (`exact_matches`, `lang_eq`,
   `canonical_tag`), `capability/runtime.rs` and `capability/mod.rs`
   (`LanguageIndex::normalize`, `matchable_type`, `matchable_domain`),
   `profile/validate.rs` (`validate_expr`) - the code half of the anchor
   verification.
4. `docs/ROADMAP.md`: the **README entry** with its owner split and its
   **Content anchors** block (the four-item list originates there); the
   **OWNER QA PASS, round 1** entry, which carries the Fedora warning string
   verbatim; the **"Artifact signing: firm 1.x"** entry.
5. The plan: Global Constraints, the Authoring-time verification section's
   README block, **Task 4** in full (Files list, Steps 1-8, "Must not decide"),
   acceptance rows **W5-a through W5-f**, and **Amendments 1 and 2**.
6. The four house-knowledge YAML files; cite entries by id.

The implementer's brief and report are **evidence, not ground truth**.

## The diff

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/review-f2e9d75..e657263.diff`

## Dimensions

1. **Every corrected claim, re-derived from the binary.** Walk the README's CLI
   reference flag by flag against your own help capture: the `--json` and
   `--locale` blanket claims (false for `schema`), `--on-collision`'s value
   domain, `--locale`'s documented default, and the exit-code sentence's `130`.
   Build your own divergence table; the report's is a claim.
2. **Did the pass MISS a divergence?** The task corrected what the plan
   enumerated plus whatever Step 1's table surfaced. Walk every subcommand and
   every flag the binary lists, and name anything the README still gets wrong or
   still omits in a way that misleads. A prior reviewer warned this section is
   "easy to lose".
3. **The exact-typed-matching paragraph and the four anchor items**, each
   against BOTH the spec section and the core symbol: language's ISO/BCP-47
   normalization and dual-field lookup; absent boolean flags comparing equal to
   `false` under `exact`; the curated closed domains for `type` and
   `codec_kind` with out-of-domain as a config-time error; and `raw:`'s
   contrast (byte-exact, single-field, no normalization, no aliasing, no
   false-when-absent). A claim the code does not support is a Critical finding
   if it was written anyway.
4. **The two counts, re-measured with your own commands**, including the
   verdict count's three boundary checks and the frozen-unit
   counter-measurement. **The trap is that the frozen unit reproduces the
   README's OLD figure and looks like confirmation**, so state which unit each
   number in the README's sentence names and whether a later reader could
   re-derive it from the sentence as written. Check the decision series claim
   the same way: is what the sentence asserts a count, a reach, or a range, and
   is that assertion true?
5. **`docs/INSTALL.md`, both named regions.** The warning string against the
   ROADMAP's verbatim record (grep, do not eyeball); the attribution to `dnf`
   rather than the `rpm` binary; the scope (no claim about the deb path); the
   register against the macOS Gatekeeper detour; and whether the note leaves the
   Linux section's existing "No gatekeeping dialog exists on Linux" sentence
   true. Then the file-top HTML comment's extended enumeration - it is
   hard-wrapped across three lines, so locate it by `code signing lands` on its
   own line, not by the sentence.
6. **What must NOT have changed:** the four `placeholder(1.0)` comments, the
   work-in-progress banner, and the passthrough recipe's YAML. Verify the recipe
   is byte-identical to its pre-task state and that
   `crates/muxsmith-cli/tests/run_live.rs`'s inlined copy still matches it.
7. **No new `file:line` citation anywhere in what this task wrote** - Task 5
   sweeps that class next, and this package must not create a member while
   repairing one. Run both of Task 5's corpus expressions (fenced in the plan's
   Task 5 Step 1) against the current tree with fired controls and state the
   corpus size.
8. **House dimension:** `owner-manual-qa-gates-the-1-0-release` (why the QA
   finding is documented rather than fixed), `proc-wrapped-prose-quote-grep`,
   `latitude-carveout-zero-content-structural-forks` (the two named-region
   repairs), the README's sell-tone register exception, and typography (ASCII
   hyphens, straight quotes, no Unicode ellipsis) in both edited files.
9. **The no-work-needed check.** Wherever the report concludes a claim needed no
   correction, or that something is already covered, run the premise.
10. **Verification quality.** Re-run the full gate as `BUILDING.md` enumerates
    it and recompute the aggregates the report states (12 rows examined, 5
    divergent; 104 headings / 103 numbers / D105; 219 versus the frozen 78).

## Adjudication questions (one explicit verdict each, phrased in both directions, not pre-rated)

1. **Where `--on-collision`'s domain landed.** The plan says the domain is
   enumerated "where the flag is introduced"; the implementer wrote it into the
   `dry-run` subsection's prose rather than into the two synopsis headings. Is
   that the right reading of the plan's phrase, or does the reader who meets the
   bare `<policy>` placeholder in a synopsis still lack what the correction owed
   them?
2. **Exit code 130's attribution.** The implementer attached `130` to `run`,
   having measured that `run` is its only producer, rather than stating it
   command-neutrally the way `cli.rs`'s doc comment words the shared contract.
   Which is true of the shipped binary, and which serves a README reader better?
   Say whether the committed form could mislead someone scripting `dry-run`.
3. **The anchor list's lead-in.** W5-c's phrasing is "the four magic properties
   are explicitly listed"; the committed lead-in says "three places ... and one
   where it deliberately does less", because the plan's own fourth item is a
   CONTRAST (`raw:` does no magic) rather than a fourth magic property. Is the
   committed framing more accurate than the acceptance row's phrasing, or does
   it under-deliver the row?

## Verdict

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-10/task-4-verdict.md`:
verdict (APPROVED / APPROVED_WITH_MINORS / NEEDS_FIXES); numbered
severity-tagged findings with `file:line`, evidence run, exact required change;
the three adjudications; an evidence appendix naming instrument paths and
commands; and a **HARVEST** (patterns, repeated rejections, and specifically
**what Task 5 must carry** - it edits comments in `run_live.rs` that quote
README text this task changed).

Your final message carries the same in short form.
