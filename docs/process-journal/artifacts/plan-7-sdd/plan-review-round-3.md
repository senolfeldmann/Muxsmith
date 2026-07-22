# Plan 7 plan review, round 3 (delta review: Task 1 / round-5-amended D64)

Artifact: `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md` (2026 lines, uncommitted)
Reviewer: same reviewer as rounds 1-2. Scope: Task 1 and its Global-Constraints D64 bullet only; everything else settled by round 2. Ground truth: the committed round-5 design (D64 read at `3e119d8`'s tree state), the main tree, and the `plan7-a` worktree. Written as a separate round file (matching this folder's per-round convention) rather than appended to round 2.

## VERDICT: APPROVED (round-3 delta)

All three review points verified on the artifact and tree; both measured dead-code claims independently reproduced in the worktree; no regression; no new finding-level defect. One note, non-blocking.

---

## Per-point disposition

### 1. Task 1 matches the amended D64 - VERIFIED

- **Surface enumeration**: Task 1's body carries the amended shape verbatim in substance - `cli_schema.rs` "2 sites, both via the bare helper - zero funnel sites in this file", with the refutation rationale (argument-less `Schema` unit variant, exit 2 on `--locale`, no `Renderer` on either path, English-only `description` fields per spec 8.4/D47). Matches the committed D64's two-exception block point for point. Premise facts re-verified at the tree: `Schema` is a bare unit variant in `cli.rs` (the four `locale: Option<String>` args sit on Validate/DryRun/Identify/Run at :31/:55/:66/:93); `main.rs`'s `Schema` arm prints `schema_for!(Profile)` and constructs no `Renderer`.
- **Step 4's diff**: `schema_json()` becomes `support::muxsmith_bare().arg("schema")...` - compared against the real `cli_schema.rs:5-15` body: a faithful minimal transformation (same chain, only the constructor swapped); `no_args_shows_usage_and_fails` unchanged from round 1's already-bare form. The import-site `#[allow(dead_code)] mod support;` is in the same step with its reason.
- **Global Constraints bullet**: now "the two enumerated `support::muxsmith_bare()` callers (`no_args_shows_usage_and_fails` and `cli_schema.rs`'s shared `schema_json()`, closed per the round-5-amended D64: exactly these two, a third caller reopens D64)". Correct and complete.
- **Step 6 recomputed from the amended code shape**: the post-conversion `cli_schema.rs` contains exactly one `support::muxsmith_bare()` occurrence in `schema_json` and one in `no_args_shows_usage_and_fails` - the grep's "exactly TWO hits, both in cli_schema.rs" expectation is correct; the fire-verification now expects a THIRD hit on the deliberate break. The first grep's one-file invariant and its two-occurrences note remain true (funnel + bare both in `support/mod.rs`, verified in the worktree at :86/:101).

### 2. dead_code rule - VERIFIED, both measurements reproduced

Probes run in `.worktrees/plan7-a`. The worktree's in-progress state is the pre-amendment shape (funneled `schema_json`, no allows anywhere) - i.e. the state that produced the NEEDS_CONTEXT. Handling: probe B ran read-only on that state; probe A reconstructed the ruled end state on `cli_schema.rs` only, via backup + `cmp`-verified byte-identical restore. Final `git status` identical to initial (same six modified files, restored file `cmp`-clean); only `target/` was written.

- **Probe B (cli_validate flags exactly the four, funnel not flagged)**: `cargo clippy -p muxsmith-cli --test cli_validate` on the allow-free support module emits exactly four never-used warnings - `insta_settings` (:36), `insta_settings_with_tmp` (:47), `fake_mkvmerge_that_fails_queries` (:62), `muxsmith_bare` (:101) - and none for the funnel `muxsmith`. Reproduces the plan's measured claim exactly.
- **Bonus per-binary sweep (the plan's parenthetical)**: `run_live` flags the same four; `dry_run_cli` and `run_cli` flag only `muxsmith_bare` - "(it and run_live use only the funnel; dry_run_cli/run_cli leave only muxsmith_bare dead)" is exact. The union of dead sets across the four funnel-using binaries is precisely the four allowed helpers; the funnel is live in all four, so the allow-free funnel preserves the signal property the ruling protects.
- **Probe A (cli_schema zero dead warnings under the ruled shape)**: with `#[allow(dead_code)] mod support;` plus the bare `schema_json` applied, `cargo clippy -p muxsmith-cli --test cli_schema` emits zero dead-code warnings (grep-zero validated by probe B's four-hit run of the same pattern), and all three `cli_schema` tests pass.
- **Both sides of the refuted premise measured**: the restored (funneled) worktree state fails both schema tests (`schema_prints_json_schema_and_exits_zero`, `keyword_domains_project_as_closed_enums_not_bare_strings` FAILED; `no_args` ok) - the exact red the Task-1 implementer reported; the ruled bare shape goes green. The amendment is not just internally consistent, it is the measured fix.

### 3. Consistency - VERIFIED

- Step 1's dead-code paragraph and Step 4's diff cross-reference each other coherently ("Step 4's diff" / "the ruled import-site allow ... the definition-site allow the ruling forbids"); no contradiction between "funnel NEVER carries an allow" and the import-site mechanism.
- The four allows are stated as four and are four (three existing helpers verified at their worktree lines + `muxsmith_bare`).
- The 11-snapshot claim (3+3+4+1+0) is unchanged and remains correct per round 1's recount; `cli_schema.rs` contributes zero snapshots, consistent with its locale-independence rationale.
- **Stale-phrasing sweep clean**: grep over the whole plan for one-caller/single-caller/"routes through the funnel"/"pinned regardless"/"locale-independent JSON Schema" phrasings - the only match is the Global Constraints line itself, hit by the benign `funnel.*schema_json` alternation inside the correct two-caller sentence. Presence control: 11 `muxsmith_bare` occurrences, two-caller wording present at all four expected sites (constraint bullet, Interfaces, Step 1 rustdoc, dead-code paragraph).

## New findings in the delta

None at finding level. One note, non-blocking:

- (a) Step 1's preamble still says "Append to the existing file (which keeps its insta helpers unchanged)" - literally false under the same step's dead-code rule, which adds one `#[allow(dead_code)]` attribute line to each of the three insta helpers. The dead-code paragraph reconciles it explicitly ("additive attribute lines on the three existing helpers; their bodies and docs unchanged"), and the governing instruction is unambiguous and adjacent, so this is a wording tension, not a defect. Worth one word ("unchanged" -> "otherwise unchanged") if the file is touched again; not gating.

## HARVEST

- **The reopening traversal worked as designed**: a closed one-caller exception met a refuting tree fact, returned as NEEDS_CONTEXT instead of a keyboard fix, was re-ruled into a closed two-caller set with the reopening clause intact (third caller reopens), and the design was amended and committed before the plan was. `proc-closed-exception-shape`'s first live traversal is a clean instance.
- **Measured-claims discipline held under dispute**: the controller-ruled-then-disputed-then-re-ruled dead_code treatment landed in the plan with both parts carrying named measurements, and both reproduced exactly on independent re-execution - including the per-binary parenthetical I probed beyond the ask. The plan's habit of stating measurements with their method ("per-target clippy in the plan-7-a worktree") is what made one-command reproduction possible; keep it.
- **Worktree-safe probing pattern for reviewers**: pre-amendment in-progress state turned out to be an asset - it let me measure the red side of the refuted premise for free before reconstructing the green side. Backup + `cmp`-verified restore + final `git status` diff is the complete safety envelope; note that an interactive `cp` alias in the shell can hang a compound command on overwrite (use `command cp -f`), which cost one timeout this round.

## Whole-delta justification

The amended Task 1 is a faithful transcription of the committed round-5 D64: the two-caller exception is enumerated, closed, and correctly wired through the Files block, Interfaces, Step 1's rustdoc, Step 4's diffs, Step 6's recomputed grep expectations, and the Global Constraints bullet, with zero stale one-caller or schema-through-the-funnel phrasing surviving anywhere in the plan. The controller-ruled dead-code treatment is folded in with explicit provenance and two measured claims, both of which I reproduced independently in the worktree - plus the red counterfactual the amendment exists to fix. The delta stays inside D64's "test-support surface only; no product code" boundary. Approved; the single wording note rides along without gating.
