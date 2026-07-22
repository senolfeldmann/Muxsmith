# Plan 7 plan review, round 2 (delta review)

Artifact: `docs/superpowers/plans/2026-07-21-plan-7-help-i18n.md` (1994 lines, uncommitted)
Reviewer: same reviewer as round 1; delta judged against round-1 standards. Settled non-findings not re-litigated; every fix verified on the artifact and against the tree, not from the author's report.

## VERDICT: APPROVED

All six findings verified fixed; both notes landed sound; no regression found in the delta; no new finding-level defect. Three notes recorded below, none blocking.

---

## Per-finding disposition

### Finding 1 (MAJOR, Task 5 Step 3 completion check) - VERIFIED FIXED

I executed the redesigned command against today's tree:

- **30 hits**, and the per-name distribution matches the plan's classification exactly: `batch-run-tooltip` 5 (the `runTooltip` computed at `BatchView.vue:307` + the four `runDisabledReason` state-id returns), `browse-button-tooltip` 2 (`SettingsDialog.vue:109`, `FirstRun.vue:113`), `batch-browse-dir-tooltip` 2 (`BatchView.vue:403,426`), the remaining 21 names 1 hit each. Every hit sits inside the 8 files Step 2 rewrites; every hit is a `$t`/`:title`/`runDisabledReason` render-position usage a step replaces - so the green state is reachable, member-by-member, as claimed.
- **The `-v` filter's complement is exactly the six named lines**: I ran the inverse (`grep -E 'aria-describedby="|id="'` over the same 24-name matches) and got precisely `SettingsDialog.vue:102,104,124,126,136,145` - the three hint names, once as `id=` and once as `aria-describedby=` each. Nothing else is dropped today.
- **Filter-scope judgment (the question put to me)**: the filter's true match surface is any line containing the substring `id="` - which includes `data-testid="`/`data-help-id="` spellings the caveat does not name. That broader class changes nothing: any masked line is by construction a line combining a render usage with *some* attribute containing `id="`, and the stated bound (one attribute per template line) covers the broader class identically. I probed `src/` for lines combining an `id`/`aria-describedby` attribute with a `:title`/`$t(` usage: none exist. With the whole-branch review seeing the final diff as the named backstop, the caveat's bound is sound. The understated spelling set is recorded as note (a) below, not a finding.
- **Red fire-verification retained**: the step keeps "run the same command before Step 2" as the fire event, and my execution of the scoped command (30 hits, non-empty) is exactly that event - it also demonstrates the `-v` filter does not swallow the red state.
- `--exclude-dir=.generated` removes the gitignored build artifact from scope (verified in round 1 that it was the ninth hit file; it no longer appears). Both exclusions are stated in the step as controller-authorized with their reasons - no silent narrowing.
- The cited `proc-check-green-state-reachable` exists: `docs/decision-ledger.yaml:3820`, Tier-1, minted 2026-07-21 from the round-1 harvest. Valid reference, correct content.

### Finding 2 (MODERATE, Task 11 `loadHarness`) - VERIFIED FIXED

- The Files block now Modifies `e2e/mount.ts` and its parenthetical states today's reality exactly (verified against the file: exports are `mountComponent`/`readModel`/`readEmitted`, bootstrap inline in `mountComponent`).
- Step 4's extraction is behavior-identical: `loadHarness(page)` is verbatim the two bootstrap lines of today's `mountComponent` (`page.setContent('<!doctype html><div id="mount"></div>')` + `addScriptTag({ path: MOUNT_HARNESS_PATH })` - compared against `mount.ts:22-23`), `mountComponent` re-points to it, no second copy.
- Signature consistency: `loadHarness(page: Page): Promise<void>` matches the Step 2 call site `await loadHarness(page)`; the import comment now reads "added by this task's Step 4"; the commit staging includes `e2e/mount.ts`.

### Finding 3 (MINOR, Task 1 call-site counts) - VERIFIED FIXED

Recomputed: 5 (`cli_validate.rs`), 11 (`run_cli.rs`), 6 (`run_live.rs`) call sites excluding each `fn muxsmith()` definition line - the plan now states these numbers with the exclusion rule spelled out, in both the Files block and the body. Matches my round-1 measurement exactly; `dry_run_cli.rs` 13 and `cli_schema.rs` 2 unchanged and still correct.

### Finding 4 (MINOR, Task 5 Step 4 27-vs-28) - VERIFIED FIXED

The prose now reads "gui-batch 27" with explicit at-this-point qualifiers for both moving counts ("gui-common reaches its final 38 only after Task 12 adds 2; gui-batch its final 28 only after Task 6 adds `batch-resolved-track`"), consistent with the command's expected `27/27`. The end-state table (final 28) is untouched and still correct.

### Finding 5 (MINOR, commit trailer) - VERIFIED FIXED

Recounted: exactly **20** occurrences of `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` - the Global Constraints line plus all 19 bash commit snippets (Tasks 9/10 commit in prose and inherit the constraint), each as a second `-m`. Zero placeholder forms survive (`your model name` / `<model`: no hits). The controller-caught intermediate placeholder violation is gone from the artifact.

### Finding 6 (MINOR, stale design line count) - VERIFIED FIXED

The number is dropped, with the reason stated in place ("a line count is deliberately not stated here; it goes stale on every amendment"). Right fix - a recount would have gone stale on the next amendment again.

### Round-1 notes

- **Note 1 (Task 16 test location)**: FIXED - pinned to `e2e/smoke.spec.ts`'s "editor view: rule grid + drag-reorder" describe block, which is the real and only grid-column assertion site (verified at `smoke.spec.ts:926`). The "(or ... match at dispatch)" hedge is gone.
- **Note 2 (Task 14 per-row mechanics)**: FIXED - Step 5 now states the mechanism: per-row anchors inject the map once at component level and compute per row from `${props.path}.${rowKey}` / `${props.path}[${i}]` / the row index. Sound. See note (c) on one phrase.
- **Note 3 (speculative smoke.spec/i18n-en Modify entries)**: deliberately unchanged, conditioned steps - accepted as settled.

## Regression sweep of the delta

Checked the delta's blast radius against round-1-verified anchors: the catalog end-state table unchanged (gui-batch row intact), Task 5's "re-point all 26 render sites" commit message still consistent (20 `:title` + 6 hint = 26), Task 1 Steps 6-8 intact, Task 21's replace-from texts untouched, dependency graph and stream cut untouched, the D55/D57 transcriptions untouched. The Tasks 8-10 change is purely additive marking: the three plan-added rules are now labeled "controller-ratified acceptance criteria (provisional - the governing human holds an open veto window on them)" and explicitly separated from the design-derived rules, with the 1-3 kB band's provenance distinguished ("normative band here; D51 states it only as a cost estimate") - exactly the separation the round-1 watch asked for. No regression found.

## New findings in the delta

None at finding level. Three notes, none blocking:

- (a) Task 5 Step 3's caveat names `id="`/`aria-describedby="` as the masked-attribute spellings; the pattern also matches `data-testid="`/`data-help-id="` lines. Same mitigation bound (one attribute per template line), zero occurrences today - cosmetic understatement only.
- (b) Task 11 Step 2's expected-FAIL will manifest as an import/type failure (`loadHarness` and `topics.ts` do not exist until Steps 3-4), not an assertion failure. Task 2 Step 2 names compile-error-as-failing-state explicitly; here it is implicit. FAIL is still FAIL; a copying implementer will not be misled.
- (c) Task 14's new sentence says `useDiagAnchor`'s getter-closure shape lets "one component instantiate it per row" - for dynamic `v-for` rows a per-row composable call is not valid Vue; the sentence's own parenthetical alternative ("inline the same lookup against the injected map") is the correct mechanism and is what the dynamic-row cases will use. The e2e fixture table pins the observable behavior either way.

## HARVEST

- **The fix round held the line on scope**: every change is traceable to a round-1 finding or note; nothing else moved (verified by anchor spot-checks, not trust). The one intermediate defect - the `<your model name>` placeholder fix-of-a-fix - was caught controller-side before this round and does not survive in the artifact.
- **The green-state demonstration is the transferable artifact of this round**: the member-by-member classification (30 hits, per-name counts, survivor enumeration, filter complement) is exactly the shape `proc-check-green-state-reachable` now prescribes, and my independent execution reproduced every number. Worth citing as the reference instance when the entry is next applied.
- **Over-restriction watch, closed for this plan**: the Tasks 8-10 content rules are now marked provisional with a named owner veto window - the correct routing (bind now, ratify explicitly) rather than silent bindingness or silent deletion. Nothing else in the delta adds restriction beyond the design.
- **Residual pattern to keep an eye on in execution**: substring-based `grep -v` filters age poorly as templates evolve (note (a)); if the completion command is ever reused for a future migration, the filter's match surface should be re-derived, not copied.

## Whole-plan justification

Round 1 found the plan complete and faithful on coverage, latitude, streams, and design fidelity, failing only on execution-surface defects. The fix round repaired all six precisely: the one unsatisfiable check is now demonstrably satisfiable (I re-derived its green state member-by-member against the tree and reproduced the author's classification exactly, including the filter's six-line complement), the one phantom interface is now a real, behavior-identical extraction with consistent signatures, and the count/trailer/marking corrections all recompute clean. The delta introduced no regression and no new defect above note level. The plan is fit for execution as specified; the three notes ride to the whole-branch review, none gating.
