# Task 18 verdict - D61 IpcError presence gate in check-i18n.mjs

**Reviewer:** independent (fresh eyes). **Worktree:** `/home/senol/Git/Muxsmith/.worktrees/plan7-g`, branch `plan7-g`, HEAD `45f0e31`.
**Combined verdict: APPROVED.**
**Spec compliance: PASS. Quality: PASS.**

Single commit `45f0e31`, one file (`scripts/check-i18n.mjs`, +54/-11), `error.rs` clean, commit unsigned (`%G? = N`, proc-05). All probe edits restored byte-identically (git-blob-hash confirmed); the only lasting writes are this verdict.

---

## Findings by severity

### Blocking
None.

### Major
None.

### Minor
None.

### Nit (no action required, recorded for completeness)
- **N1 - retired-comment prose is a mild overgeneralization.** The new check-2 comment (lines 51-58) states "Shell IpcError codes ... are reached only via a generic `$t(err.code, err.params)` pattern and never spelled out literally in src/." Two of the 19 codes (`mkvmerge-not-found`, `mkvmerge-too-old`) *are* spelled literally in `src/views/FirstRun.vue:39,41`. This does not rise to a finding: (a) the original comment carried the identical framing; (b) every code the comment names as an *example* (`mkvmerge-spawn-failed`, `run-already-active`) is genuinely generic-only; (c) it is descriptive class-prose, not an exhaustive per-code assertion; (d) zero behavioral consequence - those two are counted as used via `literalAnywhereIds` independent of the D61 feed. Left as-is is fine.

---

## 1. Spec compliance - PASS

Checked the committed block against the brief's carried code and the D61 design block (design `:1205-1300`).

- **Carried scan code verbatim.** File lines 280-304 match the brief's Step-1 fenced code byte-for-byte (`SRC_TAURI`, `IPC_ERROR_RE`, the recursive `readdirSync`/`.rs` filter, the `#[cfg(test)]` cutoff, the `code -> file:line` Map, `first-site-wins` via `!ipcErrorCodes.has`, the sorted `[...ipcErrorCodes].sort()` gate loop, the hard-fail message string). No adaptation was needed and the report's "no adaptation" claim is correct - all four fs/path helpers were already imported, `knownIds` is a `Map`, `ROOT` defined, and the four new identifiers are collision-free.
- **usedIds feed** (`...ipcErrorCodes.keys()`, line 318) present; **hard-fail join** (`ipcErrors.length === 0`, line 524) present as the fourth ANDed exit condition; **ok-line** extended with `${ipcErrorCodes.size} IpcError code(s) gated`.
- **Residual-comment retirement (Step 2): accurate to actual behavior.** The old "Known residual false positive, accepted because this half is a warning" block plus its enumerated exemption list is gone; the replacement records that D61 extracts every `IpcError::new("code")` from `src-tauri/src`, hard-gates presence, and feeds `usedIds` so the codes count as used. This matches what the check now does (see quality section). Brief `:42-50` region correctly located despite line drift.
- **Scope: number-promotion correctly EXCLUDED.** D61's design carries a Rust/wire/catalog "number promotion" half; the report correctly scopes it to other tasks and touches only the script. `error.rs` left clean - verified.
- **19-code expectation - independently recounted from the tree, not transcribed: exactly 19.**
  - Production `IpcError::new` sites (before each file's cutoff): error.rs 12 unique, run.rs 5 unique, lib.rs 2 unique = **19**. Matches design correction #6 ("12 error.rs / 5 run.rs / 2 lib.rs").
  - Catalog resolution (each code `grep -E '^code[[:space:]]*='` against `locales/en/*.ftl`): **14 gui-common / 5 gui-jobs, 0 missing.** Matches the report and correction #6. (`mkvmerge-not-found`, `mkvmerge-query-failed`, `identify-failed` additionally exist in `cli.ftl`, but `knownIds` is gui-*+diagnostics-scoped and all 19 resolve to a gui-* id, so no dedup concern - design nuance confirmed.)
- **Cutoff premise verified structurally.** Every `.rs` in `src-tauri/src` has **exactly one** `#[cfg(test)]` (error.rs:181/333, lib.rs:588/1014, run.rs:955/1685, settings.rs:193/342; main.rs none), each introduces a single `mod tests {` that runs to EOF, and no production code re-emerges after it (only-top-level-item-after-cutoff check confirmed `mod tests {` in every case). No mid-file test module exists that would silently truncate the scan. The post-cutoff `IpcError::new` sites (error.rs 305-328, incl. the test-only `"x"` and duplicate production codes) are correctly all inside the test module.
- **Gate coverage complete.** Every `IpcError` code originates from `IpcError::new("literal")`: the only `IpcError { }` struct literal is inside `fn new` itself, and no `::new(` call takes a non-literal first argument (`grep 'IpcError::new\([^"]'` empty). The regex captures 100% of code origins in this tree.

## 2. Quality - PASS (all gates run under my own build, foreground)

| Gate | Result |
|---|---|
| `pnpm check:i18n` | **exit 0** - "19 IpcError code(s) gated, 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)" |
| `pnpm lint` (eslint) | **exit 0** |
| `pnpm build` (vue-tsc --noEmit && vite build) | **exit 0** - 165 modules, built clean |
| `pnpm test:e2e` (tsc + 2 vite builds + playwright) | **exit 0 - 52 passed** |

(Note: the diff touches only the standalone `scripts/check-i18n.mjs`, which neither the app build nor e2e import, so build/e2e are independent of this change; run anyway per directive and confirm the branch is green.)

**Fire-verification reproduced (both directions).** Inserted `// IpcError::new("plan7-bogus-code")` on a non-test production line (error.rs:2, textual scanner, zero compile risk) -> `pnpm check:i18n` **exit 1** naming `src-tauri/src/error.rs:2`. Restored via `git checkout` -> blob hash identical to `HEAD:src-tauri/src/error.rs` -> `pnpm check:i18n` **exit 0**.

**Residual-closure claim reproduced (negative made to fire).** Removed `...ipcErrorCodes.keys()` from `usedIds` -> **17 unused warnings resurfaced** (exactly the generic-only shell codes: apply-*, identify-failed, internal-task-failed, invalid-run-id, job-log-*, mkvmerge-query/spawn-failed, no-active-run, profile-save-*, run-already-active, settings-*). Restored byte-identically -> 0 unused. The report's "17 not 19" is confirmed: `mkvmerge-not-found`/`mkvmerge-too-old` are the two omitted, caught as literals in `FirstRun.vue:39,41` via `literalAnywhereIds`.

**Ran the premise the report concluded needs no run: the cutoff is load-bearing.** Forced `cut = -1` (cutoff disabled) -> the test-only `IpcError::new("x")` (error.rs:305) surfaces and the gate goes **exit 1**. The shipped cutoff correctly excludes it, gating exactly 19. This also means the green 19-count is itself a standing self-check on the cutoff: a broken cutoff would yield 20 codes and a hard failure. Restored byte-identically.

## 3. House dimension - no Tier-2 deviations

- Follows the file's established **line-based literal-scan** house pattern (`CALL_RE`/`LABEL_KEY_RE` precedents; not a Rust parser), consistent with the D61 design's own rationale.
- **proc-normative-count-recomputed** applied correctly to the M1 fix (count recomputed from the final enumeration, not transcribed) - see Q1.
- No new dependencies (reuses `node:fs`/`node:path` already imported); **proc-05** satisfied (commit unsigned).
- **latitude-carveout-zero-content-structural-forks** covers the two implementer judgment calls: the scan-block *placement* (zero outward effect - no symbol surface, no data format, no verification weakening, nothing user-visible; additive within the file) and the Step-2 comment repair (a code-comment edit repairing prose the change itself falsified - a sweep duty, grant-covered per the plan-7 T3 precedent that catalog/code header comments fail the user-visible test). Both correctly surfaced in the report.

## 4. Adjudication

**Q1 - does the reworded taxonomy accurately state the file's actual check inventory?**

**Recounted both numbers from the final file myself:**
- **Hard-failure checks = 4**, read off the exit gate (lines 520-524), which ANDs exactly four `.length === 0` arrays: `missing` (check 1, literal-id resolution), `parityErrors` (check 3, cross-locale parity + D55 rules 4/5), `tooltipErrors` (D55 rule 3, editor-tooltip completeness, T17), `ipcErrors` (D61, this task).
- **Total distinct checks = 5** = those 4 hard-fail + 1 warning-only (`unused`, check 2, a `console.warn` never in the exit gate).

**Judgment: the reworded sentence states the actual inventory accurately; no misdescription.**
- "Four independent hard-failure checks (exit 1), plus one warning-only pass (check 2)" = 4 + 1, both correct.
- "Checks 1-3 below run over the catalog/source scan" - accurate: all three numbered items operate over `src/` + the en catalog; none reads `src-tauri/src`.
- "the other two hard-failure checks are D55 rule 3's editor-tooltip completeness and D61's IpcError presence gate over src-tauri/src, each documented at its own code block" - accurate: of the 4 hard-fails, 2 sit in the numbered 1-3 (checks 1 and 3) and the other 2 are tooltip + ipc; both have their own `// --- ...` block comments (lines 353, 280); D61 is correctly noted as scanning a *different* source (`src-tauri/src`).
- The numbered list's own labels stay consistent (check 1 "HARD FAILURE", check 2 "WARNING ONLY", check 3 "HARD FAILURE").

**A bare "Three" -> "Four" swap would have been wrong on two counts**, which is why the rewording (not scope creep) is the correct fix: (a) it would misstate the total as 4 when there are 5 distinct checks; (b) it would leave "over the same catalog/source scan" asserting that all four run over the catalog/source scan, which is false for D61 (src-tauri/src). The M1 directive ("recompute and correct the count") is honored faithfully - the extra descriptive change is forced by the count correction, not beyond it.

## 5. HARVEST

- **Dominant pattern:** a plan-carried transcription task executed faithfully; the single judgment surface (the review-routed M1 header fix) was handled well - reworded over word-swapped, both recomputed numbers reported in the report for the reviewer's basis. Clean no-divergence instance of `brief-drafts-verified-against-tree` (carried code verified against the tree, no adaptation needed).
- **Repeated rejections:** none - no fix rounds; APPROVED first pass.
- **Over-restriction watch:** no stop was forced that should have been covered. The latitude grant correctly covered the placement decision and the comment-repair sweep, consistent with the recorded plan-7 T3 catalog-header-comment precedent. No calibration concern either direction.
- **Ledger candidates:** no new entry. This task is a reinforcement instance of two existing entries, offered for the controller to record an occurrence if desired:
  - **proc-normative-count-recomputed** - the M1 fix is a *source-file charter* count recomputed from the final enumeration (trigger-2 shape: a count made stale by an addition, recomputed when the file settled). Directly continues the 2026-07-22 occurrence that routed M1 here.
  - **brief-drafts-verified-against-tree** - a clean instance where the carried literal needed no adaptation, verified before use. (No new entry warranted; both are settled at count >= 3.)
- **Process observation worth keeping:** the shipped gate's green 19-count doubles as a standing self-check on the `#[cfg(test)]` cutoff - if the cutoff ever breaks, the test-only `IpcError::new("x")` in error.rs surfaces as a 20th code and fails CI. The cutoff is not silently trusted; it is load-bearing and continuously exercised by the real tree.
