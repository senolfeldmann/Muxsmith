# Task 18 report - D61 IpcError presence gate in check-i18n

**Verdict: DONE**

Worktree `/home/senol/Git/Muxsmith/.worktrees/plan7-g` (branch `plan7-g`).
Base HEAD before work: `a838a32` (T17). Commit: **`45f0e31c97e29aefd7bb1e8e39c75aeb6806b4e9`**.
One file changed: `scripts/check-i18n.mjs` (54 insertions, 11 deletions). `error.rs` left clean.

## What changed

1. **D61 scan block** (brief Step 1, carried code, verbatim): line-based Rust
   scan of `src-tauri/src/**/*.rs`, each file taken up to its first
   `#[cfg(test)]`, extracting every `IpcError::new("code")` into `ipcErrorCodes`
   (`code -> file:line`). Any code absent from `knownIds` (gui-* + diagnostics
   en catalogs) produces an `ipcErrors` entry naming the Rust site.
2. **Feed into check 2** (Step 1): `...ipcErrorCodes.keys()` added to the
   `usedIds` union - this closes the documented false-positive residual.
3. **Hard-fail wiring**: `ipcErrors` printed under its own header and added to
   the final exit gate (`missing && parity && tooltip && ipc`, all `=== 0`).
4. **ok-line** (Step 3): extended with `${ipcErrorCodes.size} IpcError code(s)
   gated`.
5. **Residual comment retired** (Step 2): the check-2 "Known residual false
   positive" prose (the shell-code exemption list) replaced with a sentence
   recording that D61 now extracts the codes from `src-tauri/src` and both
   hard-gates them for presence and counts them as used in check 2.
6. **Header charter count corrected** (review-routed M1, see below).

## Recomputed counts (from the final file, not transcribed)

- **IpcError codes gated: 19** - matches the brief's expected 19 exactly, **no
  delta**. Split: 14 `gui-common.ftl`, 5 `gui-jobs.ftl` (matches design
  correction #6). All 19 resolve to a catalog message; `ipcErrors` is empty.
- **Independent hard-fail checks: 4** (this is the number the corrected header
  states). Recomputed by reading the final exit gate, which ANDs exactly four
  `.length === 0` arrays: `missing` (check 1, literal-id resolution),
  `parityErrors` (check 3, cross-locale parity), `tooltipErrors` (D55 rule 3,
  editor-tooltip completeness, added by T17), `ipcErrors` (D61, this task).
- **Total independent checks: 5** = the 4 hard-fail + 1 warning-only pass
  (`unused`, check 2, never affects exit).

### Header wording nuance (surfaced)

The original opening "Three independent checks over the same catalog/source
scan:" counted the three *numbered* items 1/2/3 - of which item 2 is
warning-only, so the original "Three" was a total-checks count, not a
hard-fail count. The controller's directive was to recompute "the number of
independent hard-fail checks" (= 4) and correct the header sentence. A bare
one-word swap ("Three" -> "Four") would have been internally inconsistent
because the numbered list still mixes hard-fail and warning items, and because
T17's tooltip check and this task's ipc check are not top-level-numbered (T17's
precedent: fold new D55 rules into the existing structure rather than renumber).

I therefore reworded the opening to state both facts without losing
information: "Four independent hard-failure checks (exit 1), plus one
warning-only pass (check 2). Checks 1-3 below run over the catalog/source scan;
the other two hard-failure checks are D55 rule 3's editor-tooltip completeness
and D61's IpcError presence gate over src-tauri/src, each documented at its own
code block." This keeps the numbered list intact (no renumber - out of the
enumerated M1 scope), names where the two non-numbered hard-fail checks live,
and notes D61 reads a different source than the catalog/source scan. Both
numbers (4 hard-fail, 5 total) are reported here so the reviewer sees the full
basis.

## Brief-vs-tree adaptations

- **Carried scan code: no adaptation needed.** Verified against the T17 tree
  before use: all four functions the block uses (`readFileSync`, `readdirSync`,
  `join`, `relative`) are already imported; `knownIds` is a `Map` (`.has(code)`
  works); `ROOT` is defined. No name collisions (`ipcErrorCodes`, `ipcErrors`,
  `SRC_TAURI`, `IPC_ERROR_RE` all fresh - grep returned nothing).
- **Placement decision (mine):** the brief gives the scan code but not its
  insertion point. `ipcErrorCodes` must exist before the `usedIds` union
  consumes its keys, so I placed the scan block plus the `ipcErrors` print
  immediately before `const usedIds`, right after the `missing` print block
  (grouping the two "no catalog message" hard fails). The scan runs early; the
  gate/ok-line consume it later.
- **Exit gate reformat:** the final `if` grew from 3 to 4 ANDed conditions;
  past 80 cols it is wrapped one condition per line (`&&` line-trailing), which
  matches the file's style and passes eslint.

## Fire-verification (Step 3, both directions)

Bogus code inserted as a non-test line in `src-tauri/src/error.rs` (a comment
line above the `use` block, line 17; the scanner is textual and only the JS
gate runs, so zero compilation risk, trivially reversible):

- **RED:** `pnpm check:i18n` -> exit **1**, output:
  `IpcError code "plan7-bogus-code" (src-tauri/src/error.rs:17) has no message
  in the en GUI catalogs`. Names the Rust site. ✓
- **RESTORE + GREEN:** line removed, `git diff src-tauri/src/error.rs` empty,
  `pnpm check:i18n` -> exit **0**. ✓

### Residual-closure verification (negative made to fire)

To prove the residual closure is real and not vacuous, I temporarily removed
`...ipcErrorCodes.keys()` from `usedIds` and re-ran: **17 unused warnings
resurfaced** - exactly the shell codes reached only via `$t(err.code, ...)`
(`apply-*`, `identify-failed`, `internal-task-failed`, `invalid-run-id`,
`job-log-*`, `mkvmerge-query-failed`, `mkvmerge-spawn-failed`, `no-active-run`,
`profile-save-*`, `run-already-active`, `settings-*`). Restored -> 0 unused.
(17, not 19: `mkvmerge-not-found` and `mkvmerge-too-old` already appear as
literals elsewhere in `src/`, so they were never part of the residual.) This
confirms the feed is what removes the warnings.

## Commands run

| Command | Result |
|---|---|
| `node probe-d61.mjs` (independent extraction replica) | 19 codes, 14 gui-common / 5 gui-jobs, 0 unresolved |
| `pnpm check:i18n` (green baseline, post-edit) | exit 0, "19 IpcError code(s) gated, 0 unused warning(s)" |
| `pnpm check:i18n` (bogus code present) | exit 1, names `src-tauri/src/error.rs:17` |
| `git diff --stat src-tauri/src/error.rs` (after restore) | empty (clean) |
| `pnpm check:i18n` (restored) | exit 0 |
| `pnpm check:i18n` (keys feed removed) | exit 0, **17 unused warning(s)** |
| `pnpm check:i18n` (keys feed restored, final) | exit 0, "19 IpcError code(s) gated, 0 unused warning(s)" |
| `pnpm exec eslint scripts/check-i18n.mjs` | exit 0 (clean) |
| `git add scripts/check-i18n.mjs` + `git -c commit.gpgsign=false commit` | commit `45f0e31` |

## Surfaced items

- Header wording nuance (above): reworded rather than one-word-swapped, to stay
  consistent; both counts reported.
- Placement of the scan block is my decision (brief left it open); no design
  content, additive only.
- `1 other locale` is already present (`locales/de/`), so check 3 parity runs
  non-trivially and passes.
- No design fork encountered; no Rust/wire/catalog changes (D61's
  number-promotion half belongs to other tasks - this task modifies only the
  script).
