# T4 verdict: NonUtf8Path guard at plan finalize (Stream D, D37)

- **Reviewer:** independent (did not implement); adversarial review, ground truth read in plan order (ADR D37 → plan Task 4 → routed-items verdict item 2 → spec 5.2/4.8/6/9.2 → Tier-2 house files → T4-report)
- **Object:** commits `1cf10f9` (impl) + `6a056eb` (spec) on branch `plan57-d`, worktree `/home/senol/Git/Muxsmith/.worktrees/plan57-d`; branch delta verified to be exactly these two commits / 7 files, worktree clean
- **Verdict: APPROVED** (2 non-blocking minors, 3 observations, 3 harvest candidates)

## 1. Path-role inventory completeness (core risk) — VERIFIED COMPLETE

Independently re-derived the argv-bound path set from `command.rs` and hunted for counterexamples.

**All four `display().to_string()` sites in `command.rs` confirmed** (grep over the worktree): `:100` `--output`, `:116` `--chapters`, `:121` `--attach-file`, `:136` group source. No fifth site exists.

**Guard set equivalence proven against `command::input_groups` (command.rs:75-83):** `input_groups` = `plan.source` (unconditionally, group 0) + every distinct `a.source` with `a.track_id.is_some()`. The guard (`planner.rs:1185-1217`) covers `plan.output`, `ChapterSource::External`, each `attachments.add_files`, `plan.source`, and each assignment source with `track_id.is_some() && a.source != plan.source`. A `track_id: Some` assignment sourced at `plan.source` is group 0 — covered by the `primary` entry. Set equality is exact. The `track_id: None` exclusion is correct (such a source contributes no input group; guarding it would drop valid plans) and the coupling comment cross-references `input_groups`' membership condition. Completeness is anchored structurally: the guard enumerates the same `Plan` fields `command` itself reads, so a path can only escape both together.

**Counterexample hunt — every candidate cleared:**

- *Non-path argv elements:* `--title` `Set(s)` is UTF-8 `String` from profile text; track/attachment ids numeric; per-track property values are `Scalar` from profile YAML (UTF-8 by parse); `--track-order` numeric. Nothing else in `command.rs` is path-derived.
- *Template stem injection (the subtle vector):* `render_ctx` (planner.rs:853-859) and `render_output`'s keep branch (:874-879) use strict `to_str()`, **not** `to_string_lossy()` — a non-UTF-8 stem cannot smuggle real-UTF-8 U+FFFD into `plan.output` under the guard's radar. A non-UTF-8-stem primary either fails as `EmptyRenderedName` (keep mode, stem drops to `""`) or is caught via the `primary` role, since `plan.source` itself is then non-UTF-8. No leak.
- *Profile-level output dir / runs-root:* flow into `plan.output` via `output_dir.join(name)` (planner.rs:931) → caught by the `output` role (test 1 uses exactly this vector, and my E2E run below reproduces it through the real CLI).
- *Attachment/chapters donors under a non-UTF-8 subdirectory* (verdict item 2 residual case 1): land in `plan.attachments.add_files` / `ChapterSource::External` → `attachment`/`chapters` roles; this pass is their only guard (never identified), as the pass comment records.
- *Fake-donor edge (`AmbiguousExternal` placeholder):* placeholder assignment is sourced at the primary path and the diagnostic is unconditionally Error — that plan never survives (S11 guard, planner.rs:1132-1139). No unguarded candidate donor reaches argv.
- *mkvmerge binary path:* `LiveSpawner` (executor/spawn.rs:76) passes the `PathBuf` natively to `Command::new` — no lossy conversion, correctly out of D37 scope.
- *Frontends:* CLI `commands/run.rs:175` and GUI `src-tauri/src/run.rs:309` both build `argv: command(p)` verbatim; the spawner prepends only `--gui-mode`. No frontend adds path-derived argv.
- *All other `display()`/`to_string_lossy` sites in core:* joblog.rs:284 (joblog record), job.rs:233 (test-only), planner.rs diagnostic params / `diag_signature` keys — none are argv. Clear.

**No counterexample found.**

## 2. Placement — VERIFIED

`plan_core` (planner.rs:290-296): `detect_source_overwrites` → `detect_non_utf8_paths` → `finalize_plans` (first drop) → `detect_output_collisions` → `finalize_plans` → `detect_empty_plans`. A poisoned plan is nulled by the first `finalize_plans` before `detect_output_collisions` counts outputs (it iterates `f.plan` only, :1224-1228): a dropped file can neither fabricate a two-planned-outputs collision nor shield a sibling. Ordering after `detect_source_overwrites` is behaviorally inert (both passes only stack error diagnostics; neither drops).

Suggestion partitions: `NonUtf8Path` is not an ambiguity code and seeds no candidates; the suggestion simulation re-runs `plan_core`, where the poisoned file reproduces the identical diagnostic on both sides of `diag_signature`, so no false regression/resolution. `detect_empty_plans` skips plan-less files — no `EmptyPlan` stacked on a NonUtf8Path drop, matching the 5.2 `EmptyPlan` row ("survived every finalize pass").

## 3. Semantics — VERIFIED (gates re-run by reviewer, foreground)

- Error severity via `Diagnostic::error` (:1208); dropped by `finalize_plans` on `Severity::Error`; no job built (both frontends build jobs only from surviving plans; test 1 asserts `plan.is_none()`).
- Once per offending file: BTreeSet dedup by path within a plan, first role in argv order wins — matches D37's "once per offending file" (per offending path, per FileReport via `for_file`, consistent with `SourceOverwrite`).
- Per-file isolation proven by test 2 (`non_utf8_primary_is_dropped_while_clean_sibling_plans`): clean sibling plans normally, zero contamination.

## 4. Wire format — VERIFIED, including empirical render of every arm

- JSON shape per D37: `code` (serde kebab `non-utf8-path`, pinned by `all_keys_match_serde_encoding`, passing) + params `path` (lossy rendering; exact U+FFFD string asserted in test 1) + `role`. Plain by-value `Diagnostic`, no boxing, no new fields, no side channel (conventions.yaml uniform-error-currency upheld).
- **Selector arms cover the emitted role set exactly in both languages.** Emitted = {output, chapters, attachment, primary, donor}; en and de both declare `[output] [chapters] [attachment] [donor] *[primary]`. Because no gate ever parses the de catalog with a real Fluent parser (see observation O2), I rendered both catalogs empirically via fluent-bundle 0.16 (the locked CLI version): **all five roles render their correct arm in both languages, zero format errors; an unknown role falls to `*[primary]`** ("Die Quelldatei…"/"The source file…") — visible-but-generic, the `unsupported-source` precedent exactly. Acceptable: nothing in the emitted set falls through today, and the fallback degrades loudly-enough (a rendered message naming the path), never a panic or a blank. See O3 for the residual gate gap.
- Fixture arm exercises the non-default `donor` branch with a U+FFFD-carrying path — the stronger branch choice.

## 5. Spec — VERIFIED

- The 5.2 row (spec:271) is accurate against the implementation: role list, "once per offending file", "exactly the paths `command` renders", plan dropped, D37 ref. No overclaim.
- Contradiction sweep re-run spot-wise, implementer's conclusions confirmed: `EmptyPlan` row (spec:266, "survived every finalize pass… local or cross-file") absorbs the new pass unchanged; 9.2's "untyped path is opt-in" is a code-path claim about `raw:` properties; section 6 (spec:313) "`command` is a pure function `Plan -> Vec<String>`" stays true — the guard is what keeps it infallible; 4.8's "Two invariants are checked on the RENDERED name" stays accurate (NonUtf8Path checks whole paths; the rendered name is UTF-8 by construction via strict `to_str()`).

## 6. House dimension — VERIFIED

- core-37 (prose-free core): code + structured params only; all prose in the ftl catalogs. ✓
- core-31 pattern-clone: invariant guarded at the enforcement point regardless of upstream narrowing; rationale in the pass comment; test 2 drives the exact bypass (injected `Identify`) that motivates it. ✓
- German catalog: real umlauts, register matches neighbors ("Dateianhang-Regel" cluster). ✓ (one wording nit, M2)

## Gates re-run (all foreground, by reviewer)

| Gate | Result |
|---|---|
| `cargo test -p muxsmith-core` | ok, all suites 0 failed (incl. `planner_non_utf8_path`: 2 passed) |
| `cargo test -p muxsmith-core --test planner_non_utf8_path` (explicit) | 2 passed, 0 failed |
| `cargo test -p muxsmith-cli` (catalog completeness + leak check + fixture arm) | ok, all suites 0 failed |
| `cargo test --workspace` | 37x "test result: ok", 0 failures (matches implementer's claim) |
| `cargo fmt --all --check` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0 |
| `node scripts/check-i18n.mjs` | ok (12 pre-existing gui-* unused warnings, unrelated) |
| E2E (reviewer-built): real CLI `dry-run` with non-UTF-8 `--output` override | `[error] output: The output path …/out�/… is not valid UTF-8…`, exit 2, no plan |
| Empirical Fluent render (fluent-bundle 0.16, scratch harness) | en+de parse; all 5 roles render correct arms; unknown role → `*[primary]` |

## Findings (ranked)

**Blockers: none.**

- **M1 (minor, stale comment):** `detect_output_collisions`' doc comment (planner.rs:1220-1221) still reads "Only surviving plans (post the SourceOverwrite finalize pass) are considered" — the first finalize pass now also incorporates NonUtf8Path drops. The `plan_core` sequence comment was updated; this one was missed. Cosmetic; fold into any next planner.rs touch or the Task-5 roll-up funnel.
- **M2 (nit, de wording):** `[attachment] Die angehängte Datei` — the add-file has not been attached (the plan just failed); "Die anzuhängende Datei" is strictly correct. Register otherwise matches neighbors; meaning unambiguous. Not worth a fix cycle alone; bundle if the catalog is touched again.

**Observations (no action for T4):**

- **O1:** `--locale de` on the CLI renders English — by design (v1 ships English only on the CLI; `i18n.rs` embeds en catalogs, spec 8.4). Recorded so the E2E evidence above is not misread as a defect.
- **O2 (pre-existing gate gap):** no gate parses `locales/de/*.ftl` with a real Fluent parser — `check-i18n` is line-based by documented constraint, the CLI embeds en only, the GUI loads de at runtime. A de syntax error would first surface in the running GUI. Cleared empirically for this entry; see harvest H1.
- **O3 (precedent-consistent):** Fluent select never errors on an unknown selector value; combined with the single-fixture-per-code leak check, a future sixth role added without an ftl arm would not be caught by any gate (it would render as "source file"). The fixture-args comment documents the value set as the tripwire — same posture as `unsupported-source`. See harvest H3.

## Harvest (candidates for the Task-5 ledger, doctrine §7)

- **H1 (gate gap):** de catalogs are never Fluent-parsed/rendered by any gate. Cheapest closure: make the CLI leak-check iterate both locale trees through `FluentResource::try_new` + a render pass (the en machinery exists; de would piggyback), or a dedicated de-render test. First empirical de parse of a new entry happened in this review.
- **H2 (pattern, reusable):** completeness anchor for consumer-guarding invariants: enumerate from the same struct fields the consumer (`command`) reads and cross-reference the consumer's membership condition (`input_groups`) in a comment on both sides. The guard cannot silently drift from the consumer without one of the two comments lying. Worked here; candidate house pattern.
- **H3 (rule of thumb):** when adding a value to a select-keyed Fluent message's domain (role/kind), the ftl arms in every locale must land in the same diff; no existing gate catches a missing arm (select falls to the default silently). The value-set comment at the fixture arm is the current tripwire.

## Evidence summary

Two commits, 7 files, worktree clean, branch delta exactly the reviewed change. Every claim in T4-report checked on disk; none found false. The one claim the implementer could not have proven from the gates alone (de selector correctness) was proven here empirically.
