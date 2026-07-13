# Task 10 report: known_extensions required method (T5-m2)

## Commit

`198fad3` `refactor(core): known_extensions is a required Identify method (T5-m2)`

## Impl inventory (every `Identify` impl found, via `grep -rn "impl.*Identify\b" --include="*.rs" .`)

| Impl | Location | Change |
|---|---|---|
| `LiveIdentifier<'_>` | `crates/muxsmith-core/src/identify.rs:397` | none - already overrode `known_extensions` (delegates to `IdentifyCache::known_extensions`), verified not assumed |
| `FakeIdentWithExtensions` | `crates/muxsmith-core/tests/planner_resolution.rs:1685` | none - already overrode (`self.known_extensions.clone()`), verified not assumed |
| `FakeIdent` | `crates/muxsmith-core/tests/support/mod.rs:20` | added explicit `fn known_extensions(&mut self) -> Option<Vec<String>> { None }` |
| `OneIdent` | `crates/muxsmith-cli/tests/catalog_completeness.rs:368` | added explicit `fn known_extensions(&mut self) -> Option<Vec<String>> { None }` |

No fifth impl surfaced anywhere in the workspace (core, cli, gui, xtask) - `cargo build --workspace --all-targets` compiled clean on the first attempt after the trait's default body was dropped, confirming the inventory above is complete; the compiler would have hard-failed any impl missing an override.

Trait declaration (`crates/muxsmith-core/src/identify.rs`) changed from a defaulted method with a body to a bare signature ending in `;`; the stale doc sentence ("Defaulted here so existing `Identify` fakes need no change to keep compiling.") is removed, the rest of the doc comment (contract + `None` meaning) kept as-is.

## Gate results (nine parts, BUILDING.md, foreground, `mkvmerge` on PATH via `/home/linuxbrew/.linuxbrew/bin/mkvmerge`)

| # | Command | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | clean |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| 3 | `cargo test --workspace` | all green across every test binary (core, cli, gui-lib, xtask, doctests); 0 failed |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean |
| 5 | `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok |
| 6 | `pnpm lint` | clean |
| 7 | `pnpm build` | vue-tsc + vite build clean |
| 8 | `pnpm check:i18n` | ok (12 pre-existing unused-key warnings, unrelated to this change) |
| 9 | `pnpm test:e2e` | 7/7 passed |

Frontend legs (6-9) are unaffected by this Rust-only change; run anyway because BUILDING.md's gate is nine parts, not conditional on which files changed.

## Diff shape vs. binding constraints

- Zero behavior change confirmed: every impl returns exactly what it returned before (`None` for the two fakes, unchanged delegation for the two live/extension-aware impls).
- No new `DiagCode`, no signature change beyond `fn known_extensions(&mut self) -> Option<Vec<String>>;` losing its body.
- `git diff` before commit: 3 files, 10 insertions / 5 deletions - trait decl (-5/+2 net, mostly the trimmed doc line), two 4-line explicit-`None` additions.

## Surfaced patterns / deviations for the house ledger

`docs/decision-ledger.yaml:714` entry `core-117-known-extensions-make-required` (`kind: non-decision`, `status: blocked`, `blocked_on: "idiomacy review (internal)"`) is exactly the deferral this task closes. Recommend the controller update it on the next ledger pass:
- `status: blocked` -> `settled`
- `blocked_on: "idiomacy review (internal)"` -> `null`
- append occurrence `{date: "2026-07-13", kind: decided, ref: "task-10 implementation, commit 198fad3"}`

Not edited directly here, per this plan's established pattern (task-1/2/3/4/6/7/9 reports surface ledger candidates in this section rather than writing `decision-ledger.yaml` from the implementer seat; the controller/verdict step harvests them).

No other deviation found: doc comment content beyond the trimmed sentence, house pattern for trait-required-vs-defaulted methods, and the two named test-double files matched the brief exactly.
