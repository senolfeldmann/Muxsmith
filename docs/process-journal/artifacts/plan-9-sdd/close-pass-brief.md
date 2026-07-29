# Plan-9 close pass - implementer brief

**Role:** fresh implementer for the one product change the Plan-9 close still
owes. It is a package of enumerated text and configuration corrections, each
routed here by a named review finding. Model tier: mid (dispatch model: Opus 5).
Effort: xhigh. An independent reviewer grades your work; the controller re-runs
your claims.

**This is not a feature.** Every item below is a correction to prose, to a
diagnostic message, or to the gate's own definition. Nothing changes program
behaviour. If any item turns out to require a behaviour change, that is a fork:
return it.

## Preamble (binding)

- Never call session-relocation tools. `master`, main worktree,
  `/home/senol/Git/Muxsmith`. Absolute paths, **foreground runs only**.
- You are the only writer in this tree while you run.
- **Read the files, not a commit hash.**
- A bare `cp` here is aliased interactive and blocks on overwrite. Restore with
  `git checkout --`, baseline taken BEFORE any mutation, and prove it.
- The local `grep` is **ugrep 7.5.0**: `\b` plus bounded repetition under `-E`
  silently returns zero. Use `-P` or a script, and fire every absence check.

## Files (EXHAUSTIVE - nine, no others)

- Modify: `src/views/BatchView.vue` (item 1)
- Modify: `crates/muxsmith-cli/tests/dry_run_cli.rs` (item 2)
- Modify: `crates/muxsmith-core/src/identify.rs` (item 3)
- Modify: `e2e/jobsview-reset.spec.ts` (item 4)
- Modify: `src-tauri/src/lib.rs` (item 5)
- Modify: `BUILDING.md` (item 6)
- Modify: `.github/workflows/ci.yml` (item 6)
- Modify: `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md` (item 7)
- Modify: `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md` (item 7)

The house-knowledge YAMLs, `docs/ROADMAP.md` and `HANDOFF.md` are the
controller's and are NOT yours - the count updates they need are mine to make
after your change lands. Say in your report if you find a count there that will
need it.

## The items

### Item 1 - BatchView's else-branch text (Task-5 delta review, routed to the close)

`src/views/BatchView.vue`, the `!doc.profile` branch's else arm. Since Task 5
replaced the positional fetch with a code-keyed `find`, that arm fires on TWO
triggers - an empty `config_diagnostics`, and a non-empty one carrying no
`parse-error` - while its comment and its `console.error` string still name only
the first. The comment must name both triggers; the string must read as "no
parse-error diagnostic" rather than "no diagnostics". Wording is yours within
those semantics. No logic change.

### Item 2 - three overclaiming strings in the CLI parity test file (Task-5 delta LOW-4)

`crates/muxsmith-cli/tests/dry_run_cli.rs`, in
`dry_run_json_sorts_config_diagnostics_errors_first_when_planning_ran`. The
`files`-is-an-array assertion added by Task 5's fix round is a SHAPE guard, not
a builder discriminator: `config_only_document` also emits `files: []`, measured
against the real profile-load-failure document, so neither that assertion nor
its `mkvmerge_found` neighbour identifies a planned batch document. The
assertions stay; three texts change. The Task-5 delta verdict (LOW-4, in
`.superpowers/sdd/plan-9/task-5-verdict.md`) carries the exact replacement -
read it there and apply it; the licence for the pre-existing `mkvmerge_found`
message, which the earlier fix round was forbidden to touch, is carried by this
pass.

### Item 3 - the `IdentifyCache` doc's constructor claim (whole-branch delta finding 3)

`crates/muxsmith-core/src/identify.rs`, the `IdentifyCache` doc. It now reads
"constructed per planning call", which names one construction context as if it
were the definition; two identify surfaces also construct the cache, per
invocation (`crates/muxsmith-cli/src/commands/identify.rs` and
`src-tauri/src/lib.rs`'s identify command). The per-call lifetime semantics hold
at all four sites - that is what the whole-branch fix established and it does
not change. One clause is owed, along the lines of "constructed per call and
dropped with it (per planning call in the pipeline seam, per invocation on the
identify surfaces)". Verify the four construction sites yourself before writing
the clause. House rule this instantiates: `core-docs-name-callers-illustratively-never-exclusively`.

### Item 4 - the spec-local IPC installer's undisclosed divergence (Task-6 LOW-1)

`e2e/jobsview-reset.spec.ts`, the doc comment above the spec-local soft-outcome
IPC installer. It answers a narrower command set than the shared
`installMockIPC` in `e2e/mocks.ts`: it does not set the OS-plugin platform
global, does not forward to the invoke recorder, and does not answer the
settings/file-write family. That is currently harmless and D104 fences the
spec-local composition, so the CODE stays. Add one sentence naming what it
deliberately omits relative to the shared installer and why the omission is safe
today. Verify the omission list against `e2e/mocks.ts` rather than copying it
from this brief.

### Item 5 - two ambiguous intra-doc links (whole-branch HARVEST 4; PREREQUISITE for item 6)

`src-tauri/src/lib.rs`, two doc comments where ```[`run`]``` is ambiguous
between the `run` function and the private `run` module. Repair per site as
```[`mod@run`]``` or ```[`run()`]``` - pick the one that matches what each
sentence actually points at, which means reading both sentences. **These fail
the rustdoc run only under `--document-private-items`, which item 6 adds to the
gate**, so this item must land in the same change or the first gate run under
the new flag goes red.

### Item 6 - the gate's own definition, at BOTH consuming sites

The gate block in `BUILDING.md` and the CI workflow `.github/workflows/ci.yml`:

- `python3 scripts/ledger-lint.py` joins the local gate block as an ELEVENTH
  part. It is already binding as a pre-push duty (Tier-2
  `ledger-lint-runs-before-every-push`); what was waiting is its appearance in
  the list.
- `--document-private-items` joins the `cargo doc` step **at both sites** -
  `BUILDING.md`'s gate block and `ci.yml`'s doc step. A single-site change was
  the recorded defect of the first attempt at this item: it would have left CI
  carrying the blind spot the change exists to remove.
- Any count wording in these two files that says the gate has ten parts is
  recomputed. Do not sweep other files for the count: the ROADMAP, the house
  YAMLs and the closed plans are either mine or dated records.

**Verify the flag's effect rather than assuming it:** run the doc command with
the flag before your item-5 fix (it must fail on exactly the two ambiguities)
and after (it must pass). That is this item's fire.

### Item 7 - D64's snapshot claim, stale in COUNT and in KIND

Four sites, all in retired plan-7 documents, enumerated with their measurements
in `docs/ROADMAP.md`'s "Docs accuracy" section - read that entry first, it is
the specification for this item:

- `docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md` at three places
  (the "11 insta snapshots" claim, the `cli_validate.rs` constructor/snapshot
  parenthetical, and the "covers all 11 insta snapshots" sentence).
- `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md` at one place, the same
  enumeration with its arithmetic spelled out.

**Recount from the tree yourself; do not transcribe the ROADMAP's numbers.**
The ROADMAP records 13 snapshots (`cli_validate` 5, `dry_run_cli` 3, `run_cli`
4, `run_live` 1) as measured at commit `3412fcc`, and the tree has moved since.
Report your own count.

**The KIND half matters more than the count:** after amendment 4 the German
snapshot does not ride the `muxsmith` funnel at all, it rides the
locale-parameterized helper, so a corrected number alone would still assert a
false coverage claim. The coverage sentence is restated too - the ROADMAP
suggests the shape ("every CLI-invoking snapshot test rides a pinned helper, the
en funnel or its locale-parameterized construction site"); verify that shape is
true of the tree before writing it, and say what you measured.

## Standing rules

- **No design latitude**, in either form. A fork found on code contact returns
  as NEEDS_CONTEXT with a decision memo, never resolved at the keyboard.
- Every item's semantics are fixed above; the PROSE is yours except where an
  item points at a verdict that fences it (item 2).
- **No behaviour change anywhere.** If an edit would alter what the program
  does, stop and return it.
- Counts recomputed from their enumerations, never transcribed; every observed
  value pasted from its run.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis.

## Verification (foreground, no subsets)

The gate as your own change redefines it - eleven parts:
`cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`;
`cargo test --workspace`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items`;
`cargo deny check`; `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`;
`pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e`; `python3 scripts/ledger-lint.py`.

Baselines to measure your delta against, all verified at HEAD: `cargo test
--workspace` **39** `test result:` lines, all ok; `pnpm test:e2e` **68 passed**;
`check:i18n` **212 catalog ids**; `ledger-lint` **516 entries**. None of them
should move - your change touches no behaviour. Report what you measure.

## Commit (SI-4, restated because you cannot see the grant)

Commits are **standing-authorized by the owner**; your global never-commit
default does not apply. You commit, you do not push. `git -c commit.gpgsign=false`,
stage each file by name (never `git add -A`), exactly one trailer
`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no `Claude-Session`
line. One commit for the package is correct; the message names the items.

## Report

`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/close-pass-report.md`, same
content as your final message: per-item before/after, the measurements you took
rather than transcribed (item 7's recount, item 3's four construction sites,
item 4's omission list, item 6's flag fire), the pasted eleven-part run,
divergences and judgment calls each named, numbered concerns a reviewer can rule
on yes/no, anything you surface for the controller including counts in my files
that need updating, and the commit hash with `git show --stat`.
