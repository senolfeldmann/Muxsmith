# Task 12 report: catalogs + spec polish (Plan 5.6, Wave 2)

Commit: `9e84e02` `docs(i18n): Fluent comment levels + honest header claims; spec §5.2 WorkerPanicked cell` (master, unsigned, not pushed). 10 files, 63 insertions, 54 deletions. Comment lines and one spec table cell only.

## Item 1: Fluent comment levels (projectfluent.org comment spec)

File-scope headers `#` -> `###` (resource comments; previously attached to the adjacent first message as its translator note):

| File | Change |
|---|---|
| `locales/de/cli.ftl` | 5-line header `#` -> `###` (also reworded per item 2) |
| `locales/de/diagnostics.ftl` | 5-line header `#` -> `###` (also item 2) |
| `locales/de/gui-common.ftl` | 7-line header `#` -> `###` (also item 2) |
| `locales/de/gui-settings.ftl` | 3-line header `#` -> `###` (wording unchanged — already accurate) |
| `locales/en/gui-settings.ftl` | 2-line header `#` -> `###` |

Message-block section comments `#` -> `##` (group comments):

| File | Blocks bumped |
|---|---|
| `locales/en/gui-common.ftl` | 4 blocks: D31 close-confirmation, T9 app-shell navigation, T9 IPC error codes, T9 first-run detection |
| `locales/en/gui-batch.ftl` | 1 block: T10 file header (a section comment separated from the first message by a blank line, so `##` not `###`) |
| `locales/de/gui-batch.ftl` | 1 block: de header, same structure as en (blank line before first message), `#` -> `##` (also item 2) |

Exception carve-outs honored:

- `locales/en/gui-jobs.ftl` + `locales/de/gui-jobs.ftl` section comments verified already `##` (5 sections each) — not re-edited. Only de/gui-jobs.ftl's *header* wording changed (item 2, stays `#` because it sits blank-line-separated... see note below), en/gui-jobs.ftl completely untouched (not in the commit).
- `locales/en/gui-common.ftl` `identify-failed` 8-line translator note stays `#` (genuine single-message note, attaches to `identify-failed`).
- `locales/en/gui-batch.ftl` D23 note on `batch-run-tooltip-run-active` stays `#` (single-message note).
- de/gui-jobs.ftl and de/cli.ftl-style headers: de/gui-jobs.ftl's header was left at `#` level per the brief's file list ("headers only where listed" — gui-jobs listed for the header *claim* fix, not the level fix; its header is followed by a blank line then a `##` section, and the brief's level-fix list names only the five files where the `#` block attaches to the first message).

## Item 2: five overclaiming de headers scoped to keys (seed M1)

All five now read (embedded in each file's existing header prose): "keys mirror it (id parity enforced by scripts/check-i18n.mjs); placeables and selector structure mirror it by convention (reviewed manually, not machine-checked)".

- `locales/de/cli.ftl` (header lines 1-7 after rewrap)
- `locales/de/diagnostics.ftl`
- `locales/de/gui-batch.ftl`
- `locales/de/gui-jobs.ftl`
- `locales/de/gui-common.ftl`

`locales/de/gui-settings.ftl` claims only keys and its wording stays (level bump only). Surrounding terminology notes (Spur/Stapel/Lauf vocabularies, du-Imperativ, ASCII quotes, close-abort NOTE) preserved verbatim, rewrapped.

## Item 3: spec §5.2 WorkerPanicked Severity cell

`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:286`: Severity cell `info` -> `n/a (job-error token, not a rendered diagnostic)`. Condition column unchanged.

## Self-check: comment-only diff

`git diff -- locales/ | grep -E '^[+-]' | grep -v '^(\+\+\+|---)' | grep -vE '^[+-]#'` returned **empty**: every changed line in every .ftl file is a comment line. The spec diff is exactly the one WorkerPanicked table row. **No message id, no message value, no rendered output changed anywhere.** Additionally verified the two consumers the plan names: `MESSAGE_ID_RE` in scripts/check-i18n.mjs matches `^([A-Za-z]...)=` (a `#` line can never match), and `src-tauri/src/run.rs::ftl_message` strips the exact key prefix then `=` (comment lines never match a key prefix) — both comment-level-agnostic, as the idiomacy review claimed.

## Gate (nine parts, all run foreground before the commit, all green)

| # | Check | Result |
|---|---|---|
| 1 | `cargo fmt --all --check` | clean |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | clean |
| 3 | `cargo test --workspace` | all pass (incl. run.rs ftl_message pin tests, catalog_completeness) |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | clean |
| 5 | `cargo deny check` | advisories/bans/licenses/sources ok |
| 6 | `pnpm lint` | clean |
| 7 | `pnpm check:i18n` | exit 0; 179 catalog ids, de parity ok; 12 unused warnings = the exact known-residual IpcError false positives documented in the script header |
| 8 | `pnpm build` (vue-tsc + vite) | clean |
| 9 | `pnpm test:e2e` | 7/7 pass, incl. `catalogs.spec.ts` "all Fluent catalogs parse cleanly" and the two de-locale tests (the real-parse confirmation target) |

## Conventions

Checked against docs/product-boundaries.yaml, docs/conventions.yaml, docs/process-conventions.yaml: docs-only change, no boundary or convention touched. A "future sweep" note originally flagged here claimed en/cli.ftl and en/diagnostics.ftl carry the same `#`-header idiom issue — **retracted, see T12b below**: the claim was written from an unverified assumption and is false.

## T12b follow-up: en/cli.ftl + en/diagnostics.ftl (controller-directed completion check)

Directive: apply the same `#` -> `###` resource-comment fix to `locales/en/cli.ftl` and `locales/en/diagnostics.ftl`, but only if the T12 finding holds on inspection.

Finding on inspection: **it does not hold — the T12 concern was a reporting error, retracted.**

- `locales/en/cli.ftl`: contains **no comment lines at all** (`grep -n '^#'` matches nothing); the file starts directly at `validate-ok = ...`.
- `locales/en/diagnostics.ftl`: likewise **no comment lines at all**; starts directly at `severity-error = error`.

There is no header to misattach or bump in either file. The T12 note claiming otherwise was extrapolated from the de counterparts (which did have such headers) without reading the en originals. Both files left byte-identical; **no change, no commit** for T12b.

Gate for T12b: not applicable — the working tree is unchanged from commit `9e84e02`, whose full nine-part gate (including `pnpm check:i18n` and `pnpm test:e2e`) ran green as recorded above; there is no new state to gate. The Rust gate parts remain untouched by .ftl comments in any case.
