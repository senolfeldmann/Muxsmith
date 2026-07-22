# Task 1 verdict - D64 en-pinned CLI invocation funnel (plan7-a, commit a8c5951)

**VERDICT: APPROVED**

Independent reviewer grade against the AMENDED plan Task 1 (3a095ef) + Global
Constraints, the AMENDED design D64 (3e119d8, the closed two-caller exception
block), Tier-2 house files, and the real tree. Every implementer claim
re-verified foreground in the worktree; every probe restored byte-identically
(cmp-verified), tree clean at exit.

## Summary of verification

| Claim (report) | Re-verified | Result |
|---|---|---|
| 70/70 tests, listed suites | `cargo test -p muxsmith-cli` foreground | 70 passed, 0 failed (cli_schema 3, cli_validate 5, dry_run_cli 11, run_cli 11, run_live 4, lib 32, +4) |
| Zero snapshot churn | post-run scan | no `.snap.new`/`.pending-snap`, git clean |
| Zero compiler warnings | fmt --check; clippy --workspace --all-targets -D warnings; forced recheck of cli test targets | fmt exit 0; clippy exit 0; zero warnings in the test compile; forced recheck clean |
| grep 1 = 1 file | `grep -rln 'cargo_bin("muxsmith")' .../tests/` | exactly one: `tests/support/mod.rs` |
| grep 2 = 2 hits, both cli_schema | `grep -rn muxsmith_bare .../tests/ \| grep -v support/mod.rs` | exactly two: cli_schema.rs:7, :27 |

Both greps fire-verified with my own fire event (probe added -> grep 1 lists a
second file / grep 2 shows a third hit; probe removed via `command cp -f`
restore; `cmp` byte-identical; git clean).

## Dimension 1 - spec compliance: PASS

- **Funnel shape.** `support::muxsmith(args)` builds `Command::cargo_bin("muxsmith")`,
  `cmd.args(args)`, then `cmd.args(["--locale","en"])` - appended AFTER the
  caller's args, per D64 (the flag is per-subcommand, so it must follow the
  subcommand). All converted call sites pass a single ordered slice
  (`support::muxsmith(&["validate", &path, "--json"])` form). Call-site counts
  match the plan exactly: cli_validate 5, dry_run_cli 13, run_cli 11, run_live 6.
- **Two bare callers, cli_schema.rs and nowhere else.** grep 2 = exactly two,
  both in cli_schema.rs (`schema_json()` and `no_args_shows_usage_and_fails`).
  The exception set is closed as ruled.
- **Dead-code treatment exactly as ruled.** Per-item `#[allow(dead_code)]` on
  the four helpers dead in >=1 binary: `insta_settings` (mod.rs:36),
  `insta_settings_with_tmp` (:48), `fake_mkvmerge_that_fails_queries` (:63),
  `muxsmith_bare` (:108). The funnel `muxsmith` (:89) carries NO allow. cli_schema.rs
  (the zero-funnel-site binary) scopes its exemption at the import site:
  `#[allow(dead_code)]` on `mod support;` (cli_schema.rs:1). Signal property
  preserved: a funnel gone dead in any of the four binaries that must use it
  (none carry an import-site allow) still fails the `-D warnings` gate - forced
  recheck confirms zero dead_code under the current shape.
- **Six files staged, nothing extra.** `--numstat` = exactly the six planned
  files. `git add -A` not used; tree clean, no stray staged paths.
- **Complete surface.** `grep -rlnE 'cargo_bin|assert_cmd::Command'` over the
  whole tests dir returns only support/mod.rs; `catalog_completeness.rs` never
  invokes the binary. The plan's six-file enumeration is the entire
  CLI-invoking surface - nothing missed.
- **Import cleanup.** Removed `use assert_cmd::Command;` (cli_validate,
  cli_schema) and `use assert_cmd::cargo::CommandCargoExt;` (dry_run_cli,
  run_cli, run_live) where the funnel conversion made them unused; the three
  files needing `std::process::Command` for real mkvmerge keep that import.

## Dimension 2 - implementer claims: PASS

All five re-verified (table above). No claim overstated. The 70 total is the
crate total; the five listed integration suites sum to 34, all green.

## Dimension 3 - locale-pinning property: PASS (with an honest scope note)

- Full suite re-run under `LANG=de_DE.UTF-8 LC_ALL=de_DE.UTF-8` (de_DE.utf8 is
  installed, so `sys_locale` genuinely reads it on Linux): 70/70, zero churn.
- **Scope note, not a defect:** at this commit Task 2 has not embedded de, so
  the renderer is en-only and a bare invocation under a de host would also
  render en. The pin is therefore correct-but-inert here; its discriminating
  power (a de host flipping unpinned output) activates only once Task 2 embeds
  de. This is exactly the stream-A `1 -> 2` edge rationale (pin the suite
  before de lands). Task 1's own responsibility - laying a correct funnel - is
  fully met.
- Funnel mechanism validated by driving the built binary directly:
  `schema --locale en` -> exit 2 (clap unexpected-arg); bare `--locale en`
  (no subcommand) -> exit 2; `schema` (bare) -> exit 0; bare no-args -> exit 2
  (usage). These are the exact failures that force the two bare callers - the
  closed exception is empirically justified, not assumed. `validate <good>
  --locale en` under a de host renders "Profile is valid." (en), exit 0.

## Dimension 4 - quality: PASS

- **Assertion bodies unchanged.** No `.assert()/.success()/.failure()/.code()/`
  `assert_snapshot!/assert_eq!/.contains()` line was removed; the only removed
  `.env(...)` lines are PATH/RUNS_ROOT calls relocating onto the funnel call,
  all re-added. No verification weakened anywhere (never-weaken line honored).
- **run_args closure arg-ordering (run_live.rs:455-470).** Builds
  `["run", profile, "--source", src, "--output", out]`, conditionally extends
  `["--on-collision", policy]`, then `support::muxsmith(&args)` appends
  `["--locale","en"]` at the very end - the `--on-collision <value>` pairing is
  never split, and `--locale en` trails a valid `run`-subcommand arg list.
  Empirically validated by the passing `live_run_rerun_with_on_collision_skip...`
  test (exercises both `run_args(None)` and `run_args(Some("skip"))`).
- **House conformance (Tier-2).** `cli-multilang-rendering`'s companion
  constraint (`product-boundaries.yaml:449-462`): "every test asserting CLI
  output pins its locale explicitly, so the suite renders identically
  regardless of the host machine's locale." The funnel implements exactly this;
  the two exceptions are locale-moot by construction (no Renderer on either
  path). No conflict with the v1 spec.
- **Commit discipline.** Unsigned (no signature line); author Şenol; trailer
  `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present; subject
  verbatim from plan Step 8; six files explicitly staged.

## Dimension 5 - DONE_WITH_CONCERNS adjudication

- **cp-alias environment note: no repo effect.** The concern is purely the
  sandbox shell aliasing `cp` interactively. The committed diff contains no
  `cp` usage; the fire-verification (implementer's and mine) is ephemeral shell
  work that touches no tracked file. Using `command cp -f` sidesteps the alias;
  my restores were cmp-verified byte-identical and left the tree clean. Not a
  defect in the deliverable. No action.

## Findings

1. **[Informational / no fix] Doc-comment self-reference offsets the
   occurrence count.** `support/mod.rs:88` is a doc line that quotes the literal
   `cargo_bin("muxsmith")` ("... appears nowhere outside this function."), so a
   *line*-level `grep -rn 'cargo_bin("muxsmith")'` inside that file returns three
   lines (doc + funnel + muxsmith_bare), whereas plan Step 6 prose says "it
   contains two occurrences." The binding invariant is per-FILE (`grep -rln` =
   exactly one file), which is what D64 states and what holds. No fix owed; the
   doc line is correct and intentional. Watch only if a future automated check
   ever asserts an occurrence *count* rather than a file count - it would read 3.

No blocking findings. No correctness, security, or maintainability defect.

## HARVEST

- **Pattern (reusable): "no allow on the critical helper; blanket-silence at the
  one zero-usage binary's import site."** When a shared `tests/support` module
  has a helper that most binaries must use but one legitimately cannot, keep the
  helper allow-free (so dead_code stays a live gate in every binary that must use
  it) and put the `#[allow(dead_code)]` on the `mod support;` line of the single
  outlier binary. This preserves the build-time signal that the helper went dead
  where it matters, instead of a definition-site allow that blinds all binaries.
  Mirrors `muxsmith-core/tests/support/mod.rs`'s per-item convention. Candidate
  for the house ledger as the standard shape for shared-test-module dead-code.
- **Over-restriction watch (per `feedback_regel_braucht_ausloeser_und_handgriff`
  / `feedback_keine_inversion_zur_universalregel`): clean.** The exception is an
  enumerated denylist (exactly these two callers; a third reopens D64) with an
  observable, greppable trigger (grep 2), not an unscoped ban. The dead-code
  allow rule likewise enumerates the four covered helpers rather than
  blanket-allowing the module. No wanted usage silently killed. No inversion to
  a universal rule.
- **Count-drift note (`proc-normative-count-recomputed` spirit):** the plan's
  "two occurrences" prose is a count summarizing a set that the intentional doc
  line pushes to three lines. Harmless here (invariant is per-file), but it is
  the classic count-goes-stale surface - a reason the per-FILE `grep -rln`
  framing in D64 is the right invariant to have chosen.
