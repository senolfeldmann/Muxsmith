# Plan-8 whole-branch fix wave

The whole-branch reviewer returned **NEEDS FIXES** for Plan 8 (the packaging
and release pipeline). Nothing it found questions the shipped pipeline: the
rehearsal is genuinely green, the release workflow is byte-identical to its
design contract, every action pin resolves. What blocks the plan close is the
documentation debt the run itself deferred to this review, plus one small
shipped-code regression. You are fixing exactly that.

Repo: `/home/senol/Git/Muxsmith`, work on `master` in the main worktree. No
branch, no worktree, no session-relocation tools. Absolute paths.

## Your contracts, in order

1. **`.superpowers/sdd/plan-8/whole-branch-verdict.md`** - the reviewer's
   verdict. Its `## Fix wave` section is your work list and its
   `## Adjudication` section carries the **exact replacement text** for every
   edit, already content-verified against the tree. Read both fully before
   editing. Where the verdict gives replacement text, use it verbatim; do not
   improve it.
2. `docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md` - the
   design (ADRs D75-D90 plus amendment A1), the contract the pipeline was
   built against. You are adding amendment **A2** to it.
3. The four house-knowledge files (`docs/product-boundaries.yaml`,
   `docs/conventions.yaml`, `docs/process-conventions.yaml`,
   `docs/decision-ledger.yaml`) are ground truth alongside them.

Line numbers anywhere in those documents are orientation only. **Locate every
edit by content**; a `:line` that does not match its quoted text is a finding
to report, not a target to guess at.

## Scope: fix-wave items A, B, D of the verdict's list

Item C is your verification (below). Item E is controller work and is NOT
yours - do not touch the ROADMAP, the process journal, `progress.md`, or any
`docs/*.yaml` house-knowledge file.

**A. `scripts/ledger-lint.py`** - two edits.

1. The `ReaderError` escape. The loader construction sits outside the parse
   `try`, so a file with an illegal control character raises an uncaught
   traceback instead of the linter's own "does not parse" failure line. The
   reviewer established this is a REGRESSION introduced by commit `92c62f1`,
   not a pre-existing condition: `git show aec4cef:scripts/ledger-lint.py`
   wraps `yaml.safe_load` inside the `try`, and `yaml.reader.ReaderError` is
   a `yaml.YAMLError` subclass, so the pre-plan-8 script caught it cleanly.
   Apply the fix exactly as written in `task-5-verdict.md`'s m1 snippet
   (`loader = None` before the try; construction inside it;
   `finally: if loader is not None: loader.dispose()`).
2. The docstring's CI trigger claim (around :30-31): "on every push and pull
   request" -> "on every master push, `v*` tag and pull request". This is
   task-5 verdict minor m3, which the task close routed nowhere and which
   therefore shipped unfixed; verify against `ci.yml`'s actual trigger block
   before you write the replacement.

**B. `BUILDING.md`** - four edits, all with verbatim text in verdict
adjudications 1 and 2.

1. The `:65-68` stale claim (adjudication 1a text). Note the boundary the run
   recorded: only the "out of scope for local development" half is wrong; the
   neighbouring "not part of the CI gate" stays TRUE, because `release.yml`
   is not the gate.
2. The `:92-95` CI-jobs enumeration (adjudication 1b text), stale since the
   `ledger-lint` job landed.
3. The tenth gate part, owner-approved: the prerequisites line, the heading's
   five->six part count, the new clippy line in the fence, and the rationale
   paragraph - all four pieces exactly as adjudication 2 (i) and (ii) give
   them.
4. Delete the `## Cross-target lint rule` section (around :140-145).
   **Controller ruling: the deletion stands, no veto.** Its rule becomes gate
   part 6 and its rationale is carried into (ii)'s paragraph, so nothing is
   lost; keeping both texts would state one rule twice with weaker force.

**D. The design document** - six edits, texts in adjudications 4, 5, 6, 7, 10
and 12.

1. Amendment-log entry **A2** (adjudication 6 text): the D86 publisher-ASCII
   fallback is superseded on the merits. The reviewer verified the underlying
   claim itself from the diagnosis run's ASCII A/B transcript - the fallback
   would not have fixed the build, because the code page is the instrument
   and three sinks carry the character.
2. The stale WiX language sites (adjudication 7 table): design `:958`,
   `:1511`, `:2007`, and `:1966`. Note what the reviewer corrected in the
   recorded list: `:1012` is NOT stale and gets no edit, and `:1966` is a real
   stale site that no list had recorded. Apply the four, skip `:1012`.
3. The three fallback-clause touches (`:941-943`, `:1921`, `:2022-2024`) ->
   "(superseded, A2)" per adjudication 6.
4. R1 observable wording (adjudication 4) plus the R1 addendum line
   (adjudication 10).
5. R6 dpkg payload path form (adjudication 5).
6. The D75 cross-reference "section 1" -> "section 0, note 2"
   (adjudication 12).

## The one fork, ruled - do not re-open it

The reviewer returned an explicit fork: the plan document
(`docs/superpowers/plans/2026-07-23-plan-8-packaging-release.md`) carries
frozen copies of the pre-fix WiX language list at `:248` (a config fence) and
`:262` (a frozen-list item). Sync them, or leave them frozen?

**Controller ruling: they stay frozen, and each gets a supersession line.**
The rule, applied uniformly to every plan-copy site: a plan is a retired
contract and its transcribed blocks are the wording an implementer was
graded against - rewriting them destroys that record. The drift is carried by
a note instead. This is the same rule applied hours earlier at the plan-7.5
close (commit `d5a6470`), so the house now has one pattern, not two.

Concretely, two additions, no rewrites:

- After the `:248` fence, add one line:
  `**Superseded 2026-07-27:** the shipped configuration is the en-US language MAP with its locale file (`wix/locale-en-US.wxl`), per design amendment A2; the fence above is the wording this plan mandated before the WiX code-page fix.`
- At `:262`, append to the frozen-list item: ` (superseded by design amendment A2)`.

Nothing else in the plan document changes. If you find a third frozen
plan-copy site carrying the pre-fix list, report it - two is the measured
claim and a mismatch is a finding.

Second fork, also ruled so it does not reach you as a choice: the tenth gate
part's Windows target is a **documented prerequisite** in BUILDING.md, NOT a
`targets = [...]` key in `rust-toolchain.toml`. Reason: the gate is the
pre-push gate its runner executes, while a `targets` key would make every
contributor's first build download a Windows std they never need. Do not add
the key.

## Verification you owe (verdict item C)

The nine-part gate is not owed for a docs-plus-linter change. Run, foreground:

1. `python3 scripts/ledger-lint.py` - expect exit 0. Recount the entry total
   it prints rather than quoting the verdict's number.
2. The three linter fire tests: a planted control character must now produce
   the linter's own `FAIL ...: does not parse (...)` line plus summary plus
   exit 1 (not a traceback); a planted duplicate key must still fire; the
   green state must be reachable after restoring. Restore with alias-proof
   forms (`command cp -f`) and verify each restoration against a pre-mutation
   backup, then `git status` clean.
3. `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`
   once, to confirm the newly documented gate part is green and the
   documented prerequisite is what the machine actually needs. If the target
   is not installed, install it (`rustup target add x86_64-pc-windows-msvc`)
   and say so in your report - that is a toolchain addition on this machine
   and it must be reported, not silently done.
4. For every edit whose correctness rests on an absence (a string gone, a
   section deleted), run the pattern BEFORE the edit and watch it return its
   expected non-zero count, so the later zero is a real absence rather than a
   malformed pattern. Name the search surface you swept.

## Commits

Three commits on master, staged explicitly (never `git add -A`), unsigned
(`git -c commit.gpgsign=false`), trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`:

1. `scripts/ledger-lint.py` (item A)
2. `BUILDING.md` (item B)
3. the design document + the plan document's two supersession lines (item D +
   the ruled fork)

Do not push. Do not create, edit, publish or delete any GitHub release - the
draft `rehearsal-30273529210` is the owner's pending inspection input and must
survive untouched. Do not dispatch any workflow. Every `gh` command (there
should be none) would need a `gh-log.md` entry.

Write your report to `.superpowers/sdd/plan-8/fix-wave-report.md`. If any
premise in this brief or in the verdict fails against the tree, stop and
report it - refuting a premise with evidence is a valid completion, and the
last three dispatches in this project each found one.
