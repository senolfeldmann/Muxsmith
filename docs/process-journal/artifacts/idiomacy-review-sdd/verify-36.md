# Verify-36: MkvmergeInfo.meets_minimum dead flag (yagni, F4)

**Verdict: CONFIRMED** (with one amendment to the replacement list)

## Finding under test

`src-tauri/src/lib.rs:147` - `MkvmergeInfo.meets_minimum` is a dead flag: always `true` on every `Ok`, unread by the frontend. Proposed: delete field, doc, computation (line 284), three test asserts, mirrored TS field (`src/ipc.ts:40`).

## Technical verification

1. **"Every Ok carries true by construction" - confirmed, and stronger than claimed.**
   `Mkvmerge::detect` (`crates/muxsmith-core/src/capability/runtime.rs:118-140`) runs `enforce_floor` on every rung; a below-floor candidate returns `Err(RuntimeError::TooOld)` immediately (`runtime.rs:219-227`, `pair < MIN_SUPPORTED` check at :222). `detect_mkvmerge_body` (`lib.rs:278-286`) then calls `mkv.version_pair()`, which on a detect-handle answers from the pair cached by that same floor check (`runtime.rs:174-179`, `cached_version_pair`) - no second spawn, so not even a TOCTOU window exists. `pair >= MIN_SUPPORTED` at `lib.rs:284` is a tautology on the Ok path.

2. **"Its own doc calls it a defensive re-check" - confirmed verbatim.** `lib.rs:135-138`: "never an `Ok` with `meets_minimum: false` -- `Mkvmerge::detect` already refuses a too-old candidate outright (D28), so `meets_minimum` here is a defensive re-check of that same fact, not the primary signal the frontend branches on."

3. **"No frontend code reads it" - confirmed.** Whole-tree grep (excluding docs/, target/, node_modules/): the only frontend occurrences are the mirrored type declaration `src/ipc.ts:40` and a mock **producer** `e2e/smoke.spec.ts:80` (`MKVMERGE_INFO` constant). Nothing consumes the field. The too-old UX path D28 requires is served by the `mkvmerge-too-old` `IpcError` with `found`/`minimum` params (tested at `lib.rs:744-751`), which this finding does not touch.

4. **Refutation gates (b)-(d):** (b) no toolchain-idiom claim is involved - pure dead-code deletion, no doc check applicable; (c) no duplication claim; (d) tag=yagni with concrete construct and concrete replacement named. None fire.

## Amendment to the replacement

The fix list misses one site: **`e2e/smoke.spec.ts:80`** (`meets_minimum: true` in the `MKVMERGE_INFO` mock). Once the field leaves the `MkvmergeInfo` interface, this object literal fails TypeScript excess-property checking, so the line must go in the same change. Two mechanical follow-ons: rename `detect_mkvmerge_body_success_reports_version_and_meets_minimum` (`lib.rs:755`), and in `detect_mkvmerge_body_finds_the_real_mkvmerge_when_available` (`lib.rs:793-794`) the deleted assert was the only use of `info` - drop the binding or assert on `path`/`version` instead. Net cut stays ~8-10 lines; the estimate is fine.

## Decision guard

- **Specs (D1-D35):** grep for `meets_minimum`/`MkvmergeInfo` over `docs/superpowers/specs/*.md` - no hit. D28 (`2026-07-10-plan-5-gui-design-decisions.md:167-181`) mandates the detection ladder and enforcing the minimum "with a clear error"; that requirement lives in the `TooOld` -> `mkvmerge-too-old` error path, which survives the deletion untouched. D28 does not mandate a `meets_minimum` field. No conflict.
- **IDEAS.md / ROADMAP.md:** no hit for the construct. The ROADMAP's whole-codebase idiomacy review entry is the umbrella authorizing this very pass (its yagni axis even names "dead flags" as a target class); `meets_minimum` appears neither in that entry's NAMED INPUTS nor in cosmetic-cleanup group K. Not separately tracked.
- **Transparency note (not a guard source):** the retired Plan-5 plan (`docs/superpowers/plans/2026-07-10-plan-5-gui-run-path.md:214`) and task-7 brief specified the `{path, version, meets_minimum}` shape. Those are completed-plan implementation artifacts, not standing decisions; the shipped code doc itself demotes the field to a non-load-bearing re-check.
