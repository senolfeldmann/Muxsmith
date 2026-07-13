# Seed [T8-m2]: dry-run-summary -> batch-summary rename

**Verdict: CONFIRMED** (tag: idiom)

## Finding

The Fluent key `dry-run-summary` is still named for the dry-run path although the summary line it renders prints for every batch: `batch_human_report` (crates/muxsmith-cli/src/commands/mod.rs:82) emits it unconditionally, and its caller `print_batch_human` (mod.rs:65) is, per its own doc comment (mod.rs:63-64), "Shared verbatim by `dry-run` and `run`: spec 5.5 requires `run` to re-plan and print exactly this report before executing." The name asserts a scope the key no longer has.

## Occurrences at HEAD

| Location | Role |
|---|---|
| crates/muxsmith-cli/src/commands/mod.rs:129 | call site (anchor; inside shared `batch_human_report`) |
| locales/en/cli.ftl:23 | key definition (en) |
| locales/de/cli.ftl:28 | key definition (de) |
| crates/muxsmith-cli/tests/catalog_completeness.rs:246 | `ALLOWLISTED_CLI_KEYS` entry |
| crates/muxsmith-cli/tests/catalog_completeness.rs:313 | fixture-args match arm |
| crates/muxsmith-cli/src/commands/mod.rs:217, 236 | unit-test fn names `dry_run_summary_renders_...` |
| crates/muxsmith-cli/tests/run_cli.rs:358 | comment naming the key |
| crates/muxsmith-cli/tests/dry_run_cli.rs:243 | comment naming the key |
| docs/ROADMAP.md:181 | deferral record for this very rename |

## Replacement

Rename the key to `batch-summary` at all nine locations above: both `.ftl` definitions, the call site, the allowlist entry and its fixture arm, the two test comments, the two unit-test fn names (`batch_summary_renders_...`), and drop the resolved deferral line from docs/ROADMAP.md.

**Snapshot correction:** the whole-branch verdict's deferral note ("touches en+de+allowlist+2 snapshots") over-counted. No `.snap` file contains the key name (verified by grep); snapshots capture rendered output, and the rename changes only the key, not the message value ("N files matched ..."). Snapshots are untouched.

## Estimates

- lines_cut: 1 (the ROADMAP deferral line; everything else is in-place rename)
- deps_cut: 0

## Why CONFIRMED

Key still defined and used at HEAD; docs/ROADMAP.md:181 still lists "dry-run-summary -> batch-summary rename (T8-m2)" as pending idiomacy-review work. Nothing in the salvaged plan-5.5 material (progress.md:64, whole-branch-verdict.md:69) or the repo indicates it was applied.
