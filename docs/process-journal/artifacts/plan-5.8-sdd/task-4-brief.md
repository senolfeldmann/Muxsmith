# Task 4 brief: plan-JSON serialization fix (ADR D40) - struct variants + unwrap hardening + regression e2e

In-flight addition to Plan 5.8, routed from the whole-branch review (finding 1, owner go 2026-07-15). Read the finding first: `.superpowers/sdd/plan-5.8/whole-branch-verdict.md` (main checkout; section "Finding 1") - it carries the evidence, root cause, and reproduction.

## The defect (verified, reproduced)

Three plan enums in `crates/muxsmith-core/src/planner.rs` are `#[serde(tag = "kind")]` with non-map newtype payloads: `TitleAction::Set(String)` (:110-118), `ChapterSource::External(PathBuf)` (:90-97), `PrimaryAttachments::Subset(Vec<u64>)` (:125-132). serde's internally-tagged representation cannot serialize these at runtime; `report/json.rs:44` (the `json!` embedding of `f.plan` in the batch document) panics. `run` builds the run document unconditionally (`run.rs:274-275`), so ANY run whose plan instantiates one of the three variants muxes successfully and then dies with exit 101; `dry-run --json` likewise. Reproduction: the README passthrough recipe (README.md:69-78, with the `title:` template) against any two-track mkv.

## Requirements

### R1 - ADR D40, authored by you

Append `## D40: ...` to `docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md`, modeled on D37-D39's slot layout (Decision / Rationale / Rejected alternatives / Interface-wire-format change / Spec amendments / Triggers created / Consistency note). Content requirements:
- Decision: the three enums become struct variants - `Set { title: String }`, `External { path: PathBuf }`, `Subset { ids: Vec<u64> }` - keeping `#[serde(tag = "kind")]`; plus the hardening decision for R3 (record the option you implement).
- Rationale: internally-tagged serde cannot represent non-map newtype payloads (runtime error, not compile error); no working consumer of the old shape can exist because it never serialized successfully; pre-1.0, shape choice free.
- Rejected alternatives, each with why: adjacently-tagged `#[serde(tag = "kind", content = "value")]` (keeps newtype variants but produces a less self-describing JSON shape and diverges from the codebase's existing internally-tagged style); README edit / removing the recipe's title line (the recipe is core-83-mandated documentation and correct as a promise); leaving the unwrap (root-cause principle: harden the class, not the instance).
- Interface/wire-format change: the plan JSON shape for title/chapters/attachments elements; state explicitly that no compatibility concern exists (old shape never serialized).
- Spec amendments: grep the v1 spec for any prose that spells the plan-JSON field shapes (`TitleAction`, `"kind"` variants); name required edits or record "none required - the spec does not spell these shapes" after checking.
- Consistency note (SI-3): record explicitly that an mkvtoolnix comparison is not meaningful here (internal report/plan serialization, no mkvtoolnix counterpart) - per the SI-3 rule "wherever meaningful".
- Triggers created: none expected; state it.

### R2 - the shape fix

- Convert the three variants to struct variants in `crates/muxsmith-core/src/planner.rs`; update every construction and match site (inventory: `planner.rs`, `crates/muxsmith-core/src/command.rs`, plus tests; no TS file references these shapes - verify with a grep and record it).
- Field names: `Set { title }`, `External { path }`, `Subset { ids }`.

### R3 - unwrap hardening (owner-approved rider)

`report/json.rs`'s document builders must not be able to panic on plan serialization. Design the minimal house-conform error path: preferred option is switching the affected builder(s) to return `Result` and propagating to the existing error-reporting surface of the callers (`run.rs`, CLI output paths); if you find a materially simpler equally-safe alternative, implement it and record the choice in D40. No `unwrap()`/`expect()` on serializing user-derived plan data remains in report/json.rs.

### R4 - tests

1. Round-trip serialization unit test per variant of all three enums (all variants, not only the three fixed ones) - the DiagCode `all_keys_match_serde_encoding` idiom is the model; place it near the enums or in the report tests, your judgment.
2. Regression e2e in `crates/muxsmith-cli/tests/run_live.rs` (same have_mkvmerge gate + helpers as the existing tests): the README passthrough recipe VERBATIM (read it from README.md at test time or inline it byte-identically - inline is fine, note the coupling in a comment) driven through `dry-run --json` (exit 0, valid JSON document parses) and `run` (exit 0, output exists, mkvmerge -J shows the templated title "S01E01", run document/log persisted - assert what the existing run-log tests assert about persistence). This pins both the bug and the recipe's paste-runnability.
3. Existing suites keep passing.

### R5 - process constraints

- Worktree: /home/senol/Git/Muxsmith/.worktrees/plan58-c (branch plan58-c, based on the merged master state 34e5624). Do not push.
- House conventions bind (docs/conventions.yaml, docs/process-conventions.yaml); surface new patterns/deviations in the report, never silently resolve.
- TDD order: write the failing round-trip test + failing e2e first (they fail on the old shapes with the exact panic), then fix, then green. Foreground runs, real output in the report.
- Full house pre-commit gate before committing (fmt, clippy -D warnings, cargo test --workspace, RUSTDOCFLAGS doc, deny; plus pnpm lint/build/check:i18n/test:e2e if you touch anything outside crates/ - you should not).
- Typography ASCII-only in all prose; English.
- One commit, or two if you prefer ADR+code separation; unsigned (`git -c commit.gpgsign=false`), explicit staging, trailer `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.

## Report contract

Write your report to `.superpowers/sdd/plan-5.8/task-4-report.md` (main checkout): per requirement R1-R5 how it is met, test commands + real output (including the pre-fix failure evidence), the consumer-inventory grep result, anything surfaced. End report and final message with exactly one status: DONE / DONE_WITH_CONCERNS / NEEDS_CONTEXT / BLOCKED, plus commit hash(es) and a one-line test summary.
