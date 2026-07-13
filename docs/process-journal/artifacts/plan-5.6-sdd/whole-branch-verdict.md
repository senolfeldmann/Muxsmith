# Whole-branch review verdict: Plan 5.6 (0b3149a..9e84e02, 33 commits)

Reviewer: whole-branch close-out (T13). Ground truth: plan Global Constraints + coverage ledger, spec 2026-07-08, ADR D36, Tier-2 house files (product-boundaries / conventions / process-conventions). Full diff read end-to-end; focused repo greps for the load-bearing claims; no suites re-run (nine-part gate controller-verified green on this HEAD).

## Wave-wide verdict

**Zero-behavior-change claim: HOLDS**, with one annotation the two-delta enumeration undercounts.

Verified per surface class:

- **Rendered CLI text**: byte-identical. The `dry-run-summary` -> `batch-summary` rename changes only the Fluent key (en+de message bodies untouched); `msg` delegating to `msg_with_counts` with empty counts builds identical FluentArgs; `severity_sorted` reuse in `validate` preserves the stable error-first order (same `sort_by_key(Reverse)` on the same insertion order); the `validate_expr` if/else-if/else consolidation preserves diagnostic order in all four branches (property-level diag first, InvalidRegex last, per property, exactly as before).
- **JSON documents**: identical except sanctioned delta (1) - `claimants` appears on `overlapping-rules` diagnostics only (`skip_serializing_if` keeps every other code's JSON unchanged; field appended after `suggestion_ref`, matching D36's wire note). `validate --json` cannot surface it (OverlappingRules is plan-time); its inline render was replaced by `rendered_diags`, whose implementation is verbatim-identical.
- **Exit codes**: `severity_exit` mapping (Error=2, Warning=1, else 0 incl. None/Info) identical to both replaced folds.
- **Latent-only changes**: diag_signature tuple key (sanctioned delta 2, '|'-collision closed; the map is count-lookup only, ordering irrelevant; T2 report records the routed-item closure). Template::parse Peekable rewrite preserves the char-offset `pos` contract on both error variants. `run_id_timestamp` via `PrimitiveDateTime::parse` against the same descriptor that formats: 16-char prefix + trailing-literal check rejects the same shapes (sign-prefixed years fail the final "Z" literal; out-of-range fields fail component parse), joblog rejection tests unchanged and green.
- **Composition checks**: CODEC_KIND_NAMES order preserved (the deleted sync test proved it pre-wave, derivation fixes it by construction); `matchable_domain`'s `Some(&CODEC_KIND_NAMES)` works via transitive deref coercion at 'static. T5's `rendered_diags` reuse composes with T2's claimants (config diags never carry them). T11's `config_diagnostics` preserves validate-then-lint order at both former sites. T9's marker const covers all 21 sites; only the intended CI grep literal remains, with the lockstep comment. T12's comment-level changes cannot affect `MESSAGE_ID_RE`, `parseCatalogIds`, or run.rs's line parser (none match '#'-prefixed lines); e2e real-parse guard now runs as a named test, still CI-red on failure.
- **GUI behavior**: `defineModel` replaces the ref+watch+emit forwarder with identical net state (the dropped `immediate: true` initial emit only re-set an already-false ref); `useTemplateRef` is type-level; `defaultAppSettings` export is field-identical to both deleted copies; `resolveLocale(): Promise<string>` handles the only theoretical non-string source (hand-edited `"locale": ""`) identically to the old filter (buildBundle("") -> null -> en fallback).

**Annotation**: there is a third interface delta the two-item claim does not name: `MkvmergeInfo` loses `meets_minimum` (T6, plan-sanctioned). It is the shell<->frontend IPC payload, shipped in lockstep in one binary; the grep step confirmed no consumer read it (ipc.ts mirror only), the e2e mock was updated in the same stream, and every `Ok` clears the floor by construction. No observable behavior change, but it is a wire-shaped change and belongs in the wave's delta enumeration, not under "zero".

CI semantics (T8) are the one surface that cannot be proven this session; see below.

## b99847c review

**Approved.** One-hunk, test-only, correct.

- The conflict is real and exactly as described: T1 (stream A) turned `CODEC_KIND_NAMES` into `LazyLock<Vec<&'static str>>` on the premise that prop_matcher's `.to_vec()` compiles unchanged; T3 (stream B) removed all `select(CONST.to_vec())` tails. Merged, `select(CODEC_KIND_NAMES)` has no `Into<Cow<'static, [T]>>` from `&LazyLock`. Each stream was gate-green in isolation; only the post-merge gate could catch it - which it did.
- `.as_slice()` is the right fix: static LazyLock derefs to `&'static Vec`, `as_slice()` yields `&'static [&'static str]`, `Cow::Borrowed` - no per-call allocation, preserving T3's intent, same value distribution and shrinking as before.
- Grep confirms this is the only select-over-LazyLock site; no sibling latent conflict. Commit message accurately scoped. Nothing masked.

## Cross-task findings

#### Critical
None.

#### Important
None.

#### Minor (new ones only)
1. **ci.yml toolchain step, Windows failure masking**: the step became a two-command `run: |` block. On windows-2025 the default shell is pwsh, which does not stop on a failed native command mid-script; if `rustup toolchain install` failed but `rustup component add` exited 0, the step would pass. Not a silent-green risk (the cargo steps would fail loudly later), but diagnosability regresses vs the old single-command step. One-line fix: `shell: bash` on the step (Actions bash runs `-e`). Rider candidate for the fix wave.
2. **cli validate.rs `collect()` is now a hollow shell**: T5 reshaped it, T11 reduced its body to a single `config_diagnostics_from_file` call with one caller - a composition neither task reviewer saw whole. Inline at the call site. Cosmetic; rider candidate.
3. **`validate_extension_list` latent case-fold widening**: old code lowered the profile extension and assumed `known` (mkvmerge `--list-types`) is lowercase; `eq_ignore_ascii_case` now folds both sides. Unobservable with real mkvmerge (lowercase output), strictly more robust; same latent class as the sanctioned diag_signature fix but unnamed. Record only.
4. **ADR filename pattern deviation**: `2026-07-13-plan-5.6-decisions.md` vs the six siblings' `*-design-decisions.md`. Plan-specified (the plan names this exact path), so not implementer drift; renaming now costs reference churn in committed docs. Accept; optional `git mv` + reference update if Şenol wants the glob-consistency.

## Funnel triage

1. **report/mod.rs: no direct test on `claimants`** - **MUST-FIX**. D36 is the wave's only sanctioned wire change and nothing pins its JSON shape: grep shows no test or snapshot touching overlapping-rules JSON at all. Engine consumption covers population only transitively (TC-A would fail on an empty vec), but nothing guards the serialization contract (`claimants` present on overlapping-rules, absent elsewhere via `skip_serializing_if`, indices matching the rendered `rules` param). The house pattern pins wire surfaces with goldens (testing-command-golden, core-85); a ~15-line unit test on `with_claimants` + `serde_json::to_value` closes it.
2. **cli validate.rs double Vec allocation** - ACCEPT. Scope-forced by `rendered_diags(&[Diagnostic])`; a validate run carries dozens of diagnostics at most, and borrow-genericizing a core function for a CLI micro-allocation inverts the cost.
3. **src-tauri run.rs `let outcome = ...; let outcome = outcome?;`** - ACCEPT, fold as fix-wave rider (pure one-line collapse to `.await?` with zero risk; the explicit-drop removal that motivated the two-step shape already landed).
4. **on_blocking unqualified in lib.rs vs `crate::on_blocking` in run.rs** - ACCEPT. One call site in run.rs; a `use` item for one call is a wash. Rider if the file is touched anyway.
5. **detect_mkvmerge_body_finds_the_real_mkvmerge_when_available panic-guard only** - ACCEPT. `.expect("detect")` still asserts exactly what the name claims (the detect ladder finds and version-parses real mkvmerge); the removed assert became tautological once every `Ok` clears the floor by construction.
6. **src/i18n empty-string locale** - ACCEPT. No producing path, and even a hand-edited `"locale": ""` degrades identically to the old filter: `buildBundle("")` returns null and the chain falls back to en.
7. **check-i18n.mjs recursive readdirSync returns dir entries** - ACCEPT. Only a directory literally named `*.ts`/`*.vue` under repo-controlled src/ would EISDIR, and the failure is loud (script crash), not a silent gate bypass.
8. **ci.yml component list stated twice (ci.yml + rust-toolchain.toml)** - ACCEPT. Drift fails loud (a component missing from CI breaks its own fmt/clippy step; a component added to the toolchain file but unused in CI costs nothing); the rustup #4216 comment already anchors why the duplication exists. Optional sync-nudge comment as rider.
9. **locales/de/cli.ftl header dropped article** - REFUTED, no action. The current header reads "The en catalog is the / source of truth" across the line wrap; the article is present (same in de/gui-jobs.ftl). Record as refuted in the roll-up.

## House dimension

No violations. Specifically checked:

- **conventions.yaml**: core-37 (prose-free core) actively reinforced by D36 - structured params over display-string re-parse is the same principle; add a `reinforced` occurrence (ref: ADR D36 / commit 89f346b). testing-support-helpers reinforced: `fake_mkvmerge_that_fails_queries` hoisted to tests/support, retiring exactly the run_cli.rs:L498 duplication the entry's last occurrence names; add occurrence (ref: commit 0e8d048). core-85 respected (rendered_diags made pub and reused rather than re-implemented). No new dependencies (time's `parsing` is a feature on an existing pinned dep - "dependencies are earned" upheld).
- **process-conventions.yaml**: proc-01/proc-02 followed (this review is the proc-02 instance; b99847c as an inline controller fix is the sanctioned post-merge-gate reconciliation, now independently reviewed). proc-07 exercised by T8's four VERIFY-FIRST items (all confirmed, none refuted). ci-10 pins intact (no action/version un-pinned; components come from the pinned toolchain). ci-08 gate strengthened, not weakened (single-source marker + lockstep CI comment). proc-05 respected (agent commits unsigned, trailers present).
- **product-boundaries.yaml**: no scope touched; pure refactor wave.
- **Ledger harvest candidates**: (a) new Tier-1 pattern candidate, agent-emergent/technical-code: "a literal that an external gate greps for has exactly one source of truth (const) and the gate carries a lockstep comment naming it" (MKVMERGE_SKIP_MARKER, ci.yml:105); (b) the two reinforcement occurrences above; (c) proc-09 gets its executed-wave occurrence at plan close.
- **Deviations observed, both plan-specified (not implementer drift)**: ADR filename pattern (above); de/gui-batch header at `##` while sibling de headers moved to `###` (the plan classifies gui-batch's block as a message-block section comment, defensible under the Fluent comment spec).

## Assessment

**Branch: NEEDS FIX WAVE**

Complete must-fix list:
1. Direct test pinning the D36 wire contract: `with_claimants` populates `claimants` + the rendered `rules` param from one slice; serialized JSON carries `claimants` on overlapping-rules and omits it on every other code (funnel item 1).

Optional riders for the same wave (zero-risk one-liners, not blocking on their own): collapse `outcome?` in run.rs start_run; `shell: bash` on the ci.yml toolchain step; inline cli validate.rs `collect()`; `use crate::on_blocking` in run.rs.

**Reasoning:** Every surface the zero-behavior-change claim covers verifies as byte-equivalent or plan-sanctioned, including the cross-stream compositions no task review could see (T1xT3 reconciled correctly by b99847c; T5xT2, T6xT7, T11xT5 all sound); the one unprovable surface (CI on-runner) is soundly reasoned locally with the "first push shows 3 green legs" open item already recorded. The single gap standing between this branch and done is that the wave's only wire-format change ships untested at the wire, which the house's own golden-pinning pattern requires closed; the fix is one small test, and cheap riders can travel with it.

---

# Final verdict: fix wave a5d506b (parent 9e84e02, verified)

Commit inspected directly (git show matches the review diff; exactly the four in-scope files; unsigned, trailers present; tree clean; gate re-ran green on this HEAD, controller-verified).

- **MUST-FIX (D36 wire-contract test): RESOLVED.** `with_claimants_populates_structural_field_and_json_from_one_slice` pins all three legs: (1) single-slice co-derivation - one `with_claimants(&[0, 2])` call asserted against both the structural field and the rendered `"tracks[0], tracks[2]"` param (non-contiguous indices, catching reindexing bugs); (2) `json["claimants"] == [0, 2]` on the claimant-carrying diagnostic; (3) absence proven the strong way - `as_object().get("claimants").is_none()` fails on a serialized `null`, so it pins genuine key elision by `skip_serializing_if`, not a weak `is_null()`. Location is house-consistent: report/mod.rs's existing test module, directly beside `diagnostic_serializes_with_snake_case_severity_and_kebab_code`, same build-serialize-assert idiom - the sibling wire-shape suite, not a new home.
- **R1 (start_run `outcome?` collapse + `use crate::on_blocking`): RESOLVED.** One-line `.await?`; the `?` early-return still fires the Reservation Drop exactly as before. The use-item folds in the qualification-consistency rider too - both riders closed in one edit, as nominated.
- **R2 (ci.yml `shell: bash`): RESOLVED.** Single line on the toolchain step; Actions bash runs `-eo pipefail`, closing the pwsh partial-failure-masking window on Windows; Linux/macOS semantics unchanged. Still covered by the standing "first push shows 3 green legs" open item, as is the whole workflow.
- **R3 (inline `collect()`): RESOLVED.** Wrapper deleted, `config_diagnostics_from_file` inlined at the single call site; no behavior change.

**New findings in the fix commit:** none. Nothing rode along beyond the mandated items; all hunks are within the nominated scope, no rendered text, JSON shape, exit code, or CI semantics change beyond the stated Windows-diagnosability fix.

**Branch: READY**
