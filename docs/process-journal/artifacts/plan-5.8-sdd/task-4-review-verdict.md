# Task 4 review verdict: plan-JSON serialization fix (ADR D40)

Independent reviewer; did not implement. Worktree `/home/senol/Git/Muxsmith/.worktrees/plan58-c`, branch `plan58-c`, commit `de9ec50` on merged master `34e5624`. Everything below was re-run or source-read by me on the worktree, not taken from the implementer report.

## Independent verification

| Check | Result |
|---|---|
| Three enums are struct variants | PASS - `ChapterSource::External{path}`, `TitleAction::Set{title}`, `PrimaryAttachments::Subset{ids}` (planner.rs:89/113/130); `#[serde(tag="kind", rename_all="snake_case")]` retained |
| Every construction/match site updated | PASS - grep of all `TitleAction::`/`ChapterSource::`/`PrimaryAttachments::` sites: every `Set`/`External`/`Subset` uses `{ }` syntax; zero tuple-style remnants (one match is a comment). `command_integration.rs` touches only unchanged variants (`KeepAll`/`Keep`), correctly untouched |
| No TS/frontend reference | PASS - grep over `*.ts/tsx/vue/svelte` for the three types AND the literal strings `"Set"/"External"/"Subset"` in `src/`: zero hits |
| Round-trip tests cover EVERY variant | PASS - 3 tests, all 9 variants (Keep/Clear/Set, Keep/Drop/External, KeepAll/DropAll/Subset), asserting the snake_case tagged shapes |
| No other same-class landmine in core | PASS (beyond brief) - the only other `#[serde(tag=...)]` enums in core, `StructuredEdit` (planner.rs:202) and `JobEvent` (queue.rs:21), are struct-variant-only. The fix is complete for the class, not just the instance |
| README recipe driven byte-identically | PASS - programmatic byte-compare of README.md:72-77 vs the test's inline literal: identical. Coupling comment present (run_live.rs:280-292), states the inline-not-read rationale and the update obligation |
| e2e drives dry-run --json + run | PASS - dry-run --json: exit 0, JSON parses, asserts `plan.title == {"kind":"set","title":"S01E01"}`; run: exit 0, output exists, `mkvmerge -J` title `S01E01`, persistence asserted (exactly one run dir + `summary.json` parses + plan title matches) |
| All 4 gated live tests actually ran | PASS - `cargo test -p muxsmith-cli --test run_live` foreground: 4 passed, 0 filtered/ignored, real mkvmerge v100.0 on PATH, zero skip markers |
| Pre-fix failure evidence genuine | PASS (corroborated) - matches the whole-branch-verdict's own independent reproduction (report/json.rs:44, `TitleAction::Set`); the three enum panics carry the exact serde internally-tagged error strings ("containing a string"/"...a sequence"). Not re-run against a reverted tree (the no-modify constraint), which the independent Finding-1 reproduction already covers |
| cargo test --workspace | PASS - green across every crate incl. doc-tests (core lib 119, cli, gui/src-tauri 78, xtask); no FAILED/error/panic |
| fmt / clippy -D warnings / doc -D warnings / deny | PASS - all exit 0, zero warnings; deny "advisories ok, bans ok, licenses ok, sources ok" |
| D40 slot completeness (R1) | PASS - all 7 slots present (Decision incl. the R3 option recorded / Rationale / Rejected alternatives (5) / Interface-wire-format / Spec amendments / Triggers created=none / Consistency note SI-3 "not meaningful"); modeled on the D38/D39 layout; spec-amendment zero-hit grep re-verified by me (all 6 patterns, 0 hits) |
| Commit hygiene | PASS - unsigned (`%G? = N`), trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` present, explicit staging (7 files == brief inventory), one commit |

## The central adjudication: the R3 deviation

The brief preferred `Result`-returning builders propagating to the callers' error surfaces; the implementer kept the infallible signature and added a private `plan_value` helper that falls back to `serde_json::Value::Null` on serialization error, rationale in D40.

**(a) Is the fallback equally safe, or does it swallow a future bug silently?**

Split, and the split matters. On the dimension that actually motivated Finding 1 - "a completed successful mux must never be reported as a failure" - the fallback is equally safe, and D40's argument is factually correct. I verified the load-bearing ordering: `run.rs` collects the mux outcomes at `handle.join()` (run.rs:271) *before* `batch_document` is ever called (run.rs:275). A `Result` propagated to a non-zero exit would therefore turn an already-completed mux into a reported failure, which is Finding 1 in miniature. The null-fallback preserves the invariant; and it is idiomatic for this codebase - run.rs already degrades a failed run-log write to an `eprintln` warning with the exit code unchanged (run.rs:299-302), the exec-19 "best-effort report/log degradation" pattern.

On the *other* dimension - "detect a future regression of the same class" - the fallback is **not** equally safe: `unwrap_or(Null)` silently emits `"plan": null` for a file that actually has a plan, in both `--json` and the persisted `summary.json`, with no signal anywhere in a release build. D40 presents the degrade as pure benefit and does not name that cost. It is a genuine (if low-probability) silent-swallow.

**(b) Is there a strictly better in-scope middle (fallback PLUS a loud signal)?**

Partially, and D40 does not address the cheapest form. A one-line `debug_assert!` in `plan_value`'s error branch would make the future-regression case fail loudly in every debug build (all tests/CI run debug) while still degrading in release - scream in dev, degrade in prod. It emits no wire prose and no user-facing text, so it does **not** violate core-37 (prose-free-core), and needs no signature change. D40's rejected-alternatives slot rejects only the *`eprintln`* form of a loud signal, on the core-37 grounds that report/json.rs "produces codes and params, never text" - a correct objection to a user-facing string, but one that simply does not apply to `debug_assert!`. That form is unenumerated.

The heavier "production signal" forms are legitimately out of proportion, and D40's cost reasoning covers them by extension: a structured in-report diagnostic would need a new `DiagCode` + a bilingual catalog entry (the exact cost D40 rightly rejects for `Result`-propagation) plus a restructure of the pure `plan_value` helper to reach the diagnostics vector; and the house's own best-effort precedent (exec-19, run.rs:299-302) surfaces via existing structured channels or silently ignores benign cases, never via a fresh catalog message. So production-silence is defensible; the missed middle is specifically the dev-time `debug_assert` tripwire.

**(c) Does D40 honestly represent the preferred option it overrode?**

Yes. The `Result`-propagation entry names it as "the brief's own PREFERRED default for R3" and engages it on the merits with two honest arguments, both of which I verified: the cost claim (signature ripples into both binaries - confirmed, `batch_document` is called from cli `dry_run.rs`/`run.rs` and src-tauri `lib.rs`/`run.rs`, 6 sites; plus a new bilingual catalog message), and the Finding-1-failure-mode reintroduction (confirmed via the run.rs ordering above). No strawman.

**Ruling: the deviation STANDS.** The `Result`-rejection is sound, well-reasoned, house-consistent, and on the load-bearing invariant arguably better than the brief's stated preference - which is exactly what the latitude clause anticipated, and the recording obligation in D40 is met. One Minor, non-blocking finding attaches (below): the fallback is silent where a one-line `debug_assert!` would make its own stated raison d'etre - "make that future mistake degrade instead of crash" - loud in dev/CI at zero cost, and D40's alternatives analysis does not consider it. Owner's call; not a fix that gates approval.

## Findings

### Minor - the `plan_value` fallback degrades silently; no dev-time tripwire

- **Evidence:** `crates/muxsmith-core/src/report/json.rs:156-160`. `serde_json::to_value(p).unwrap_or(serde_json::Value::Null)` substitutes `null` on serialization error with no signal in any build. The path is unreachable today (all variants are struct variants, pinned by the new round-trip tests), so this bites only on a future regression - but that future regression is precisely what R3 exists to catch, and it would ship emitting `null` plans into persisted run documents unnoticed.
- **Suggested fix (optional):** add `debug_assert!(false, "plan serialization failed; a Plan-adjacent enum reintroduced a non-map newtype variant under #[serde(tag=...)]: {p:?}")` (or `debug_assert!(v.is_ok(), ...)` on the result) inside the error branch, keeping the release-build `null` degrade. Dev-loud, prod-degrade; no core-37 impact, no signature change. Its marginal value is bounded by whether a test drives the bad variant through `batch_document` - the new README e2e already does so for `TitleAction`, and this generalizes that guard to any variant. A one-line note in D40's rejected-alternatives (why the dev-tripwire form was or was not taken) would close the (b)-gap.
- Not Important because: the path is unreachable today; the per-variant round-trip test is the present primary guard; `null` is already a legitimate wire value for `f.plan`; and production-silence is correctly reasoned and house-consistent.

No Critical or Important findings. The D40 placement before "## Deliberately out of scope" (rather than strictly last) is a defensible grouping judgment, correctly self-flagged in the report; not a finding.

## Verdict (a): spec compliance

**PASS.** R1 (D40, all 7 slots, spec-amendment grep re-verified, SI-3 not-meaningful note present), R2 (three struct variants, every site updated, consumer inventory grep confirmed), R3 (no `unwrap`/`expect` on plan/Batch data in report/json.rs; the pre-existing `Diagnostic` unwrap is structurally infallible and correctly out of scope; the latitude was honestly exercised and recorded), R4 (per-variant round-trip tests for all nine variants + the README-recipe regression e2e through dry-run --json and run), R5 (house gate clean, ASCII-only, unsigned single commit with the trailer) are all met. The one deviation is inside the brief's explicit delegation and satisfies the recording obligation.

## Verdict (b): code quality

**PASS** with one Minor. The fix is complete for the defect class (the only two other tagged enums in core are already struct-variant-only), idiomatic, and house-conform (core-37, core-85, the exec-19 best-effort-degradation philosophy). The Minor is the optional dev-time tripwire on the silent fallback.

## Harvest (candidates for the convention ledger)

- **Reinforces the whole-branch-verdict's technical candidate** ("any `#[serde(tag=...)]` enum on a wire surface either uses struct variants exclusively or gets a per-variant serialization round-trip test"). D40 instantiates it, and this review confirms core now satisfies it wholesale (three enums fixed; `StructuredEdit`/`JobEvent` already compliant). Count -> 2, agent-emergent, technical-code; promote at 3.
- **Reinforces the paste-runnable-doc candidate** ("a doc recipe is verified by driving every command its prose tells the user to run next"). The new e2e drives dry-run --json AND run byte-identically from the README recipe, exactly the gap the whole-branch pass identified. Count -> 2, agent-emergent, process.
- **New candidate:** when core substitutes a best-effort fallback on a can-not-happen serialization/encoding path, pair the release-build degrade with a dev-build tripwire (`debug_assert!`), so a future regression fails in CI rather than silently emitting the fallback into a persisted artifact. Separates dev-loud from prod-degrade; complements, does not replace, the structured error channel used where a renderer is in reach (exec-19). Count 1, agent-emergent, technical-code.

## Final

APPROVED

## Re-review round 1 (commit ff9140f)

Delta reviewed against de9ec50; everything below re-verified by me on the worktree.

- **Finding resolved as intended.** `plan_value`'s error branch (report/json.rs:156-175) now carries `debug_assert!(false, ...)` naming the serde error, the defect class ("a Plan-adjacent enum has reintroduced a non-map newtype variant under #[serde(tag = \"kind\")]") and the D40 context, then degrades to `null`. Exactly the dev-loud/prod-degrade middle the Minor asked for; the rewrite from the `map/unwrap_or` chain to a `match` is behavior-identical. The updated doc comment correctly describes both halves.
- **Assert cannot fire on the current suite.** Re-ran `cargo test -p muxsmith-core` (debug build, assert active): all green, zero firings; `cargo test -p muxsmith-cli --test run_live` 4/4 through the real debug binary's `batch_document` path with a live `Set` plan. Additionally verified the one non-regression route into the branch - a non-UTF8 `PathBuf` inside a plan, which serde_json refuses to serialize - is guarded upstream by `detect_non_utf8_paths` (plans with such paths are dropped to `None`, pinned by `planner_non_utf8_path.rs`, 2 tests passing), so no current input can reach the assert. This also means the fallback is slightly less theoretical than D40's "future regression only" framing suggests (the upstream guard, not the enum shapes alone, keeps paths safe) - which strengthens, not weakens, the case for the assert; no doc change required.
- **D40 sentence accurate.** The added sentence in the rejected-alternatives eprintln bullet (decisions file :379-383) records the adopted `debug_assert!` middle ground, its dev/CI-loud rationale, and that the release-build null degradation stands unchanged. "Compiled out in release" is the standard, correct description of `debug_assert!` semantics. This closes the (b)-gap named in the adjudication.
- **Nothing else changed.** `git diff de9ec50..ff9140f` touches exactly 2 files (report/json.rs +16/-4, decisions.md +5): the helper, its doc comment, and the one D40 sentence. No enum, test, or other file touched. Worktree clean.
- **Gates.** `cargo fmt --all --check` exit 0; `cargo clippy --workspace --all-targets -- -D warnings` exit 0, zero warnings (notably `assertions_on_constants` does not fire on the messaged `debug_assert!(false, ...)`). Commit unsigned (`%G? = N`), trailer present, two files staged as claimed.

**Re-review verdict: APPROVED.** No open findings remain.
