# Routed-items review verdict (correctness / security / perf)

- **Date:** 2026-07-14
- **Base commit:** `b96f27b106b4c24a6db463be0a84b0cd4b41016f` (master; all of Plan 5.6 `0b3149a..a5d506b` and the Windows lint fix `a783a63` merged)
- **Scope:** the 11 items in "## Routed out of scope" of
  `/home/senol/Nextcloud/Projekte/project-muxsmith/idiomacy-review-findings.md` (planner.rs:L1965 through ci.yml:L104),
  adjudicated against current master at the actual (shifted) sites.
- **Reviewer stance:** bug-hunt (correctness, security, durability, performance). Nothing was edited or committed; recommendations conform to `docs/conventions.yaml`, `docs/product-boundaries.yaml`, `docs/process-conventions.yaml`.

Severity scale for STILL-OPEN items: release-blocker / should-fix-pre-1.0 / can-wait-v1.x.

---

## 1. planner.rs:L1965 — '|'-joined signature key can collide

**Verdict: FIXED-BY 403e573** (plan-5.6 T2, "refactor(planner): idiom/dup/yagni/stdlib cleanups").

`diag_signature` now returns `BTreeMap<(String, String, String), usize>` (planner.rs:1934) — a structural tuple key. The comment at 1927-1930 explicitly records the closure: "a `config_path` or file that itself contains the former `|` separator can no longer collide two distinct diagnostics into one signature." Multiset semantics (R1 v) preserved; the regression-vs-collapse property is pinned by `duplicate_signature_diagnostic_is_a_regression_not_a_collapse` (planner.rs:2042).

Interaction check: the related display-re-parse concern (claimants) was separately closed by ADR D36 (structural `claimants` field, commit 89f346b, wire contract pinned in a5d506b) — that was a Seeds item, not one of the 11, but confirms the whole "re-parse rendered params" class is retired.

## 2. command.rs:L100 — non-UTF-8 paths enter argv lossily via `Path::display()`

**Verdict: STILL-OPEN.** Sites on master: `command.rs:100` (`--output`), `:116` (`--chapters`), `:121` (`--attach-file`), `:136` (group source). All four use `display().to_string()` (U+FFFD substitution). `pub fn command(plan: &Plan) -> Vec<String>` (command.rs:55) is infallible, so there is no error channel at the conversion site.

The asymmetry claim verified: the identify path rejects non-UTF-8 full paths (`capability/runtime.rs:206`, `RuntimeError::Parse("non-UTF-8 path cannot be passed to mkvmerge")`).

**Exposure is narrower than the finding implies** — two existing guards shrink it:
- Discovery silently skips any file whose *file name* is non-UTF-8 (`discovery.rs:63-66` primaries, `:159-162` locator hits).
- Primaries and track donors are identified, so a non-UTF-8 *full path* (e.g. non-UTF-8 source-root or directory component) fails identification cleanly before planning.

Residual reachable cases:
1. **Attachment/chapters donors** are never identified; a donor with a clean file name under a non-UTF-8-named *subdirectory* (recursive locator walk) reaches `command()` and produces a corrupted `--attach-file`/`--chapters` argument → late, misleading mkvmerge job failure instead of a plan-time diagnostic.
2. **Non-UTF-8 `--output-dir` CLI override** with a clean source dir: plans render, `--output` argv gets the lossy path; mkvmerge fails to open, or (if a lossy-named dir exists) writes to a different directory than the one all core checks validated.
3. **Check/execute divergence:** `detect_source_overwrites` compares true `PathBuf`s (`planner.rs:1142-1146`) while execution uses the lossy string — the exec-02 hard boundary ("never writes over its inputs") is enforced on a different byte string than the one handed to mkvmerge. The overwriting corner needs an input literally named with U+FFFD at the right position, so it is contrived, but the invariant's enforcement and execution operating on different representations is a genuine correctness defect in the product's hardest guarantee.

**Severity: should-fix-pre-1.0.** Grounding scenario: (3) — a hard, non-configurable product boundary (product-boundaries exec-02) whose check and whose execution disagree on the path identity; plus (1), realistic for media collections with legacy-encoded (Latin-1/CP1251) directory names on Linux, which today die as a confusing mkvmerge "file not found" at run time instead of a clean plan-time error.

**Minimal fix (house-conform):** plan-time guard, not a `command()` signature change. In the planner's finalize pass, validate every argv-bound path of a finalized plan (`plan.output`, chapters path, each `attachments.add`, each input-group source) with `path.to_str().is_none()` → emit one per-file **error** `Diagnostic` (new `DiagCode`, e.g. `NonUtf8Path`, params: lossy `path` + role) and drop the plan, exactly like the sibling rendered-filename invariant (core-31). Keeps `Diagnostic` as the uniform by-value error currency and core prose-free (core-37: code + params only); `command()` stays infallible because non-UTF-8 can no longer reach it. Catalog keys en+de + spec 5.2 row + one C1 fixture ride along.

## 3. tests/suggestions.rs:L325 — duplicated section-header comment

**Verdict: STILL-OPEN** (shifted to lines 279 and 313; byte-identical `// --- (b) yaml_fragment must emit valid, round-trippable YAML (bug D) ---`). Plan 5.6 did not touch it.

**Severity: can-wait-v1.x.** Editorial only; zero behavioral surface. Justification: the only failure scenario is a reader momentarily misreading section structure.

**Minimal fix:** delete line 313 (the copy directly above the first test; line 279 correctly heads the section including its fixtures). Fold into the ROADMAP "Cosmetic cleanup, one pass (sweep group K)" v1.x item rather than a dedicated commit.

## 4. tests/identify_live.rs:L20 — `make_sample` spawns bare-PATH `mkvmerge`, not the located handle

**Verdict: STILL-OPEN** (site now `identify_live.rs:20`, `Command::new("mkvmerge")` inside `make_sample`; the gate at :36-38 uses `Mkvmerge::locate()`).

Re-checked the finding's premise against current runtime code: `Mkvmerge::locate()` (capability/runtime.rs:94-104) is still PATH-only, so fixture spawn and gated handle remain behaviorally identical **today**. But the hypothetical the finding hedged on ("if locate() ever gains a non-PATH source") has moved closer: `detect()` (runtime.rs:118-140) now carries the platform-candidates ladder incl. `/opt/homebrew/bin` (core-89). The day any live suite's gate switches from `locate()` to `detect()` — the natural "improvement" — a machine with mkvmerge only at a platform candidate passes the gate and then **panics** in `make_sample` (`expect("spawn mkvmerge to build fixture")`) instead of skipping.

**Severity: can-wait-v1.x.** Justification: no reachable failure on master (gate and fixture agree on PATH); the failure needs a future refactor as its trigger.

**Minimal fix:** pass the located handle: `fn make_sample(m: &Mkvmerge, dir: &Path)` using `Command::new(m.path())`, matching the sibling live suites (`command_integration.rs`, `executor_live.rs` already use `m.path()`). Observable trigger for pulling it forward: any change that makes a test gate use `detect()`.

## 5. cli/commands/run.rs:L220 — `let _ = ctrlc::set_handler(...)` swallows registration failure

**Verdict: STILL-OPEN** (site confirmed at run.rs:220 on master, unchanged by 5.6; the D16 single-level SIGINT contract is documented at :216-218).

If registration fails, the run silently loses the D16 cleanup semantics: no in-flight-job kill via the queue, no partial-output deletion, no summary, no deliberate exit-130 path. (Terminal Ctrl-C still SIGINTs the whole foreground process group, so mkvmerge children die anyway — the loss is the *cleanup*, chiefly stale partial outputs, not a runaway mux.) `ctrlc::set_handler` fails only on double registration (impossible here: one registration per CLI process) or an OS-level signal-registration failure — vanishingly rare on the three target OSes.

**Severity: can-wait-v1.x.** Justification: the failure scenario requires an OS-level sigaction/SetConsoleCtrlHandler failure that practically does not occur; the degradation when it does is a leftover partial file, not data loss (partials are new outputs; inputs are never touched per exec-02).

**Minimal fix (house-conform):** on `Err`, render a warning instead of discarding — but through the Fluent catalog, not a bare `eprintln!`: the CLI's established shape for exactly this degradation class is `run-joblog-unavailable` (run.rs:332, `create_logger` doc at :311-314). Add `run-signal-handler-unavailable` to `locales/en/cli.ftl` + `locales/de/cli.ftl`, render via `renderer.msg(...)` on `Err`, register the key in `tests/catalog_completeness.rs`. A bare `eprintln!` (the finding's proposal) would be the lone non-Fluent user-facing string in the CLI — a house violation (core-37's surface-side counterpart; every existing CLI warning goes through `renderer.msg`). Bundles naturally with item 8 (same two catalog files).

## 6. src-tauri/settings.rs:L150 — durability claim vs missing fsync before rename  *(release-relevant head b)*

**Verdict: STILL-OPEN** (site now settings.rs:136-174). The rustdoc still claims the atomic publish covers "crash, power loss, kill" (:137-138) and promises "any reader of `path` always sees either the previous complete file or the new complete one, never a partial write" (:143-144). The body is `fs::write(&tmp_path, ...)` + `fs::rename` (:169-173) with **no `sync_all` anywhere in the file** — verified by grep. The ROADMAP v1.x note "settings half was hardened in the Plan-5 fix wave" refers to the atomic rename itself; the fsync gap was never addressed and no supersession is recorded.

The claim is false exactly as the finding states: rename atomicity covers process death; on power loss with delayed allocation (ext4 `data=ordered`, btrfs) the rename can commit to the journal before the temp file's data blocks are on disk, so `settings.json` can come up zero-length or torn. ext4's `auto_da_alloc` rename-over-existing heuristic mitigates but does not guarantee, and does not bind other filesystems. Consequence chain verified in current code: torn file → `load` returns `SettingsError::Parse` (settings.rs:126) → mapped to IPC `settings-parse-failed` (error.rs:127-129) on **every** settings read/write command (lib.rs:125-128) — a persistent error state until the user manually deletes the file. That is precisely the "recovery flow loops on forever" scenario the atomic-publish doc (:138-140) claims to have closed.

**Severity: should-fix-pre-1.0.** Justification: a documented durability contract that the implementation does not deliver, whose failure mode is a persistently erroring GUI settings surface after a power cut; the fix is ~5 lines with no design questions. Not a release-blocker: the data at risk is a recent-profiles list and defaults, not user media, and the vulnerable window is milliseconds per save.

**Minimal fix:** replace `fs::write(&tmp_path, &bytes)` with `File::create` + `write_all` + `sync_all()` before the rename (all mapped to `SettingsError::Io`). With the temp file fsynced, "previous complete or new complete" holds across power loss too (a lost rename yields the *previous* file — within the stated contract); a directory fsync for rename *durability* is not required by the doc's claim and can stay out. Alternatively (zero-risk floor): scale the doc back to process-crash safety. Recommend the fsync — it makes the existing doc true instead of weakening it. Keep scope here: joblog atomic writes are separately tracked (ROADMAP v1.x) and must not be pulled in.

## 7. src/diagnosticFluentParams.ts:L26 — comment overclaims usize-parity strictness

**Verdict: STILL-OPEN** (comment now at ts:25-26: "Mirrors the Rust side's `parse::<usize>()` strictness: rejects negative numbers, floats, empty strings, and scientific notation"; code at :46-50 still `Number(raw)` + `Number.isInteger(n) && n >= 0`).

Re-verified the semantics: negatives and floats **are** genuinely rejected (left as string), and whitespace-only is explicitly handled (:43-45); but `Number("1e3")` → 1000, `Number("0x10")` → 16, and `Number(" 42 ")` → 42 all pass and get promoted, so the "scientific notation" clause of the comment is false and hex/padded forms are silently accepted. Behaviorally harmless on master: every value arriving here is canonical Rust `usize::to_string()` output.

**Severity: can-wait-v1.x.** Justification: no reachable misbehavior — the only producer emits canonical digits; the defect is a documentation lie that would mislead the next maintainer touching wire formats.

**Minimal fix:** gate with `/^\d+$/.test(raw)` before `Number(raw)` (one line; makes the documented contract real and exactly matches canonical `usize::to_string()` output), or — cheaper still — reword the comment to what the code does. Prefer the regex: it converts a false comment into a true invariant.

## 8. locales/en/cli.ftl:L20 — intended 2-space indent is stripped by the Fluent parser

**Verdict: STILL-OPEN** (en/cli.ftl:20-21 `dry-run-assignment` / `dry-run-output`, de/cli.ftl:26-27 unchanged with the same two post-`=` spaces; the 5.6 `dry-run-summary` → `batch-summary` rename (c877e4f) did not touch these keys).

**Empirically re-verified against the actual runtime parser**, not just the ebnf: vendored `fluent-syntax 0.12.0` (the version in Cargo.lock), `parser/core.rs::get_message` calls `expect_byte(b'=')` then `get_pattern()`, whose first statement is `self.skip_blank_inline()` (parser/pattern.rs:49) — the spaces are consumed before the pattern starts. Rendered CLI dry-run output is therefore flush-left today; the visual indent in the source is dead text. Blast radius of fixing: **no snapshot pins these lines** (grepped every `.snap` in the repo for rendered assignment/output text — the three dry-run snaps cover empty-dir/config-diag cases only), and `catalog_completeness.rs` checks renderability, not leading whitespace.

**Severity: should-fix-pre-1.0.** Justification: rendered human-output cosmetics are cheapest to change before the 1.0 output freeze; post-1.0 the same 4-line change perturbs users' eyeballed/scraped output. The current output is not wrong, but the source encodes an intent (visual hierarchy under `dry-run-file`) that silently does not ship — either the intent or the dead spaces should go before the format is frozen.

**Minimal fix:** decision first (Şenol's call — presentation preference, not a technical question): if the indent is wanted, `dry-run-assignment = {" "}{" "}rule { $rule } -> track { $track }` (or a single `{"  "}` literal placeable) at en:20-21 + de:26-27; if not, delete the dead spaces (zero behavior change). Four catalog lines either way, no code, no snapshot updates.

## 9. .github/workflows/ci.yml:L1 — no workflow-level `permissions:` block  *(release-relevant head a)*

**Verdict: STILL-OPEN, severity downgraded by repo-level evidence.** No `permissions:` key anywhere in ci.yml (grep over the current 139-line file). However, the finding's "GITHUB_TOKEN gets the repository default (potentially write-all)" was checked against the authoritative source: `gh api repos/senolfeldmann/Muxsmith/actions/permissions/workflow` returns `{"default_workflow_permissions":"read","can_approve_pull_request_reviews":false}` — this repo's token is **read-only today**, not write-all. (Call logged to gh-log.md per house rule.)

What remains open: (a) the read default is still broader than needed (read on *all* scopes vs `contents: read` only); (b) it is mutable out-of-band state — one settings-page click, or a future org policy, silently re-broadens every workflow run, and nothing in the repo would show it; (c) OSSF Scorecard's Token-Permissions check scores the explicit top-level block, relevant for a public 1.0 repo whose supply-chain posture is otherwise a showpiece (SHA-pinned actions, pinned runners, cargo-deny job). Neither job needs anything beyond `contents: read`: checkout, rust-cache (cache API needs no token scopes), mise-action (public release download), cargo-deny-action.

**Severity: should-fix-pre-1.0.** Justification: two lines close the one gap in an otherwise deliberate least-privilege story before the tag draws attention to the repo; the concrete scenario is a compromised third-party action exfiltrating/abusing a token whose breadth the repo file no longer controls after an out-of-band settings change.

**Minimal fix:** workflow-level

```yaml
permissions:
  contents: read
```

directly under `on:` in ci.yml. No per-job overrides needed. (Plan-6 packaging will need `contents: write` on its release workflow — job-level, in that future workflow, not here.)

## 10. .github/workflows/ci.yml:L73 — mise-action fetches a floating mise binary  *(release-relevant head c)*

**Verdict: STILL-OPEN — SUPERSEDED / TRACKED; not re-litigated per the recorded owner decision.**

State on master re-verified: `jdx/mise-action@e6a8b39... # v4.2.0` (ci.yml:76), SHA-pinned action JS, **no `version:` input** — the 5.6 T8 commit (a0a0b4c) only dropped a restated-default `cache: true`; there was never a mise-binary pin. `mise.toml` pins node 26.5.0 + pnpm 11.10.0 (the tools mise installs), but the mise binary itself still floats to latest at every CI run — the ci-10-pin-everything tension is real and unchanged.

Supersession verified as recorded: ROADMAP "v1.x candidates" → **"Remove mise from CI (post-1.0, Şenol 2026-07-12)"** (docs/ROADMAP.md:300-305) explicitly "Supersedes the routed pre-1.0 supply-chain finding (the SHA pin covers the action JS, not the mise binary it downloads). Structural, hence post-1.0, not a pre-tag patch." The pre-1.0 gate text (ROADMAP:263-265) cross-references the same supersession. Human-sourced decision; per the review brief this is adjudicated as tracked, not re-argued.

**Severity: can-wait-v1.x (by owner decision).** One factual note for the record, not a recommendation against the decision: until the structural removal lands, every CI run executes an unpinned latest-mise binary; if that exposure window ever bothers anyone pre-tag, `with: version:` on the action is a one-line interim pin that the recorded decision neither adopts nor explicitly forecloses. Şenol's call; the tracked path needs nothing from this review.

## 11. .github/workflows/ci.yml:L104 — full workspace test suite runs twice per matrix leg

**Verdict: STILL-OPEN** (sites now ci.yml:81 `cargo test --workspace` and :105 `cargo test --workspace -- --nocapture --test-threads=1 2>&1 | tee ...` inside the skip-marker gate). Plan 5.6 changed neither.

The finding's technical basis re-verified against current GitHub docs (workflow-syntax reference, fetched 2026-07-14): an explicit `shell: bash` step runs `bash --noprofile --norc -eo pipefail {0}` — pipefail **is** active on the :102-112 step (it declares `shell: bash`), so a test failure survives the `| tee` and the :105 run alone gates both real failures and silent skips. Dropping :81 is sound for the gate.

One tradeoff the finding does not weigh: :81 is the only run exercising the suite under parallel test threads; :105 is serialized. Dropping :81 loses the concurrent execution mode (the executor/queue tests are exactly the ones whose races parallelism can surface). The recorded rationale for `--test-threads=1` (ci.yml:96-98, marker-line interleaving) is likely over-cautious — Rust's `eprintln!` takes the stderr lock per call, so a single-call marker line cannot tear mid-line within a process, and cargo runs test binaries sequentially — meaning a *single parallel* `--nocapture` run could gate everything and keep the concurrency coverage. But that contradicts a recorded in-file rationale (ci-08 territory), so it needs its own small verification (one deliberate CI run inspecting marker integrity) before adoption; it is not claimable from this review alone.

**Severity: can-wait-v1.x.** Justification: pure wall-clock/latency — Actions minutes are free on the public repo (ci-01 context); the double run wastes the cheaper (parallel) of the two runs per leg, so the saving is real but well under the finding's "roughly halves". No correctness or gating defect.

**Fix options, ranked:** (1) v1.x, verify-then-adopt the single parallel `--nocapture | tee` run (best: one run, keeps parallel coverage; requires falsifying the :96-98 interleaving rationale empirically and amending that comment); (2) drop :81 as the finding proposes (safe now, loses parallel-mode coverage); (3) keep both (status quo, defensible as belt-and-braces). Observable trigger for (1): next time ci.yml is touched for any other reason.

---

## Summary table

| # | Item (source order) | Verdict | Severity |
|---|---------------------|---------|----------|
| 1 | planner.rs `'\|'`-joined signature key | FIXED-BY 403e573 (plan-5.6 T2) | — |
| 2 | command.rs non-UTF-8 argv lossiness | STILL-OPEN | should-fix-pre-1.0 |
| 3 | suggestions.rs duplicate section comment | STILL-OPEN | can-wait-v1.x |
| 4 | identify_live.rs bare-PATH fixture spawn | STILL-OPEN | can-wait-v1.x |
| 5 | run.rs swallowed ctrlc registration | STILL-OPEN | can-wait-v1.x |
| 6 | settings.rs fsync-before-rename durability | STILL-OPEN | should-fix-pre-1.0 |
| 7 | diagnosticFluentParams.ts strictness comment | STILL-OPEN | can-wait-v1.x |
| 8 | cli.ftl stripped dry-run indent | STILL-OPEN | should-fix-pre-1.0 |
| 9 | ci.yml missing `permissions:` block | STILL-OPEN (premise downgraded: repo default verified `read`, not write-all) | should-fix-pre-1.0 |
| 10 | ci.yml floating mise binary | STILL-OPEN — SUPERSEDED/TRACKED (ROADMAP v1.x "Remove mise from CI") | can-wait-v1.x (owner decision) |
| 11 | ci.yml double test run per leg | STILL-OPEN | can-wait-v1.x |

Counts: 1 FIXED, 10 STILL-OPEN (of which 1 superseded/tracked, 4 should-fix-pre-1.0, 6 can-wait). 0 REFUTED, 0 PARTIALLY-RESOLVED, 0 release-blockers.

## Triage proposal

### (i) Release-blockers

None. Nothing in the routed list threatens user data, muxed output correctness on the happy path, or the security of the published repo as it stands today (token default verified read-only).

### (ii) One small pre-1.0 fix plan (4 items, ~1 short SDD task or one supervised session)

Ordered by leverage; items 9+6 are the ROADMAP's named heads, 2 is the only real correctness item, 8 rides the output freeze:

1. **ci.yml `permissions: contents: read`** (item 9) — 2 lines, no test surface. Verify: one green CI run (rust-cache and mise-action need no broader scope).
2. **settings.rs fsync before rename** (item 6) — swap `fs::write` for `File::create`+`write_all`+`sync_all`, ~5 lines; existing atomic-publish tests keep passing; the rustdoc becomes true as written. Do not expand into joblog (separately tracked v1.x).
3. **Non-UTF-8 argv guard at plan finalize** (item 2) — the one item with real design surface: new error `DiagCode` (e.g. `NonUtf8Path`, lossy path + role params), emitted per file in the finalize pass for any argv-bound path failing `to_str()`, plan dropped; catalog en+de, spec 5.2 row, C1 fixture, one Unix-only test with an `OsStr::from_bytes` path. Pattern-clone of core-31's rendered-filename invariant.
4. **cli.ftl dry-run indent** (item 8) — after Şenol's one-word call (indent: yes/no). If yes: `{" "}` placeables en:20-21/de:26-27; if no: delete the dead spaces. Optionally bundle item 5 (ctrlc warning via new `run-signal-handler-unavailable` key) here since the same two catalog files and catalog_completeness registration are already open — it is cheap in this bundle and not worth its own task later.

### (iii) v1.x deferrals, each with an observable trigger

- **Item 10 (mise binary)** — already tracked: ROADMAP v1.x "Remove mise from CI". Trigger: exists (post-1.0 structural work). Nothing to add.
- **Item 11 (double test run)** — fold into the next deliberate ci.yml edit. Trigger: any ci.yml change; then run the one-off verification of the single-parallel-run variant (falsify the :96-98 interleaving rationale empirically) before dropping either run.
- **Item 5 (ctrlc warning)** — if not bundled into (ii).4: trigger is the next cli.ftl/catalog-touching change.
- **Item 4 (bare-PATH fixture spawn)** — trigger: any change that moves a live-test gate from `Mkvmerge::locate()` to `detect()` (which now carries non-PATH candidates); at that moment this becomes a real panic-instead-of-skip bug and must land in the same diff.
- **Item 7 (`/^\d+$/` gate)** — trigger: any change to diagnostic wire params or `NUMERIC_DIAGNOSTIC_PARAMS`; until then the false comment clause is the only defect.
- **Item 3 (duplicate comment)** — fold into ROADMAP's existing "Cosmetic cleanup, one pass (sweep group K)".
