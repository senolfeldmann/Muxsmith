# Audit: core-12-unknown-keys-are-errors (PROMOTION candidate)

**Cluster:** `core-12-unknown-keys-are-errors` (kind: pattern, domain: core, status: settled)
**Claimed count:** 5 | **promoted:** true | **promoted_at:** 3
**Verdict:** REDUCED (verified_count = 4; >= 3 survive, promotion stands)
**Audited:** 2026-07-13

Statement under audit: an unknown profile key is a config-time hard error, not a
warning; `#[serde(deny_unknown_fields)]` on every profile struct. The attribute
was silently ineffective on inline untagged struct variants until the
TemplateBlock/ExternalBlock newtype fix restored rejection.

The pattern itself is real and richly evidenced (design decision, plan
constraint, a genuine violation caught at review, and a landed correction that
is still in the current tree). The count is inflated by one: occurrence 3
double-counts the Plan 1 document that occurrence 1 already cites.

## Per-occurrence verification

### occ1 - `spec §4 + Plan 1 Global Constraints` (decided) - CONFIRMED
Two distinct artifacts, both genuine:
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` §4 line 47:
  "Unknown keys are errors, not warnings (explicit over silent)." Error-table row
  line 276 (`UnknownProperty | error`) corroborates.
- `docs/superpowers/plans/2026-07-08-plan-1-core-foundations-validate-cli.md`
  Global Constraints line 15: "Unknown keys in profiles are errors, not warnings:
  `#[serde(deny_unknown_fields)]` on every profile struct (spec 4)."

Supports "decided". Stands independently on the spec §4 anchor even after the
Plan-1 overlap with occ3 is removed.

### occ2 - `handoff plan-1-close` (decided) - CONFIRMED
`docs/process-journal/artifacts/handoffs/2026-07-09-plan-1-close.md` line 41:
"untagged config enums use newtype block variants (`FilenameCfg::Template(TemplateBlock)`,
`SourceCfg::External(ExternalBlock)`) because serde ignores `deny_unknown_fields`
on inline struct variants." Distinct artifact; records the settled model
decision. Supports "decided".
Note (not disqualifying): file is dated 2026-07-09, occurrence claims 2026-07-08.
The ref resolves unambiguously and supports the content; one-day metadata slip only.

### occ3 - `commit 1f00aa6` (decided) - DROP (duplicate of occ1)
`git show 1f00aa6` = "docs: add plan 1", a single-file commit (3259 insertions)
that creates `.../plan-1-core-foundations-validate-cli.md` and nothing else.
`git log` on that file confirms 1f00aa6 is its sole creation commit. So this
occurrence is the git-commit view of the exact "Plan 1 Global Constraints" text
that occ1 already cites as its second anchor. The commit introduces no
independent instance of the pattern beyond the Plan-1 content occ1 counts; it is
the document, not a separate recurrence. Dropped as a duplicate of another listed
occurrence.

### occ4 - `task-4 review event` (violated-corrected) - CONFIRMED
`docs/process-journal/artifacts/plan-1-sdd/verdicts/task-4-review-verdict.md`
line 19: "`#[serde(deny_unknown_fields)]` is not effective on the four
untagged-enum struct variants ... parses successfully today, silently dropping
the unrecognized key. This contradicts spec 4.1 ... and the controller's explicit
binding constraint." Matches the occurrence's evidence string verbatim in
substance. This is the detection half of the violated-corrected cycle: a real
violation caught at review, including that the gap was plan-mandated (copied from
the brief's Step 5 sample). Supports "violated-corrected".

### occ5 - `commit b5eaa3d (newtype block variants)` (violated-corrected) - CONFIRMED
`git show b5eaa3d` = "fix(core): enforce unknown-key rejection inside untagged
config blocks". Diff extracts `TemplateBlock`/`ExternalBlock` standalone structs
(each `#[serde(deny_unknown_fields)]`) and rewires `FilenameCfg`, `TitleCfg`,
`SourceCfg`, `ChaptersCfg` as newtype variants; adds regression tests. The
correction half of the cycle. Still present in the current tree
(`crates/muxsmith-core/src/profile/model.rs` carries `TemplateBlock`/`ExternalBlock`
with `deny_unknown_fields`), confirming "settled". Round-2 verdict
(`task-4-review-verdict-round-2.md`) accepts the fix. Distinct from occ4 (fix
commit vs. review verdict, correction vs. detection); not a duplicate.

## Result

| Occurrence | Ref | Kind | Verdict |
|---|---|---|---|
| occ1 | spec §4 + Plan 1 Global Constraints | decided | CONFIRMED |
| occ2 | handoff plan-1-close | decided | CONFIRMED |
| occ3 | commit 1f00aa6 | decided | DROP (duplicate of occ1) |
| occ4 | task-4 review event | violated-corrected | CONFIRMED |
| occ5 | commit b5eaa3d | violated-corrected | CONFIRMED |

**verified_count = 4** (occ1, occ2, occ4, occ5).
**Verdict: REDUCED** - one occurrence dropped as a duplicate, 4 distinct survive
(>= 3), so the promotion to standing house knowledge stands. Recommend recording
count = 4 and removing the commit-1f00aa6 occurrence to keep the count real.
