# Task 4 report: v1-spec amendments (design section 4)

Status: DONE. Commit `70282fd4c5539ca0a41665ea29ede9afec6f186e` on master.

## Context read

- Global Constraints (`implementer-preamble.md`) read in full before any edit.
- `task-4-brief.md` read in full.
- Design section 4 (`docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md:789-850`) read in full, including the scoping paragraph (amendment 2) and the self-contradiction sweep at the section's end. The design's quoted amendment text matches the brief's quoted text verbatim (the brief's own note that it is "line-wrapped for this plan" checks out against the design's line-wrapped original at :801-805 and :814-819).
- Checked whether the design's post-brief amendments (case-9 witness, dates) touch section 4: they do not. `git log --oneline -5` on master shows the two streams already merged (`e36885f` stream A, `33be397` stream B) plus `341c411` (trigger-1 house update) and `ae24589` (case-9 witness / T2 findings, which touches the e2e test file, not the design doc's section 4). Section 4's amendment blocks read identical to the brief's transcription. No NEEDS_CONTEXT triggered.

## Step 1: anchor verification (fire-verified)

Ran against `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` before any edit:

- Line 374: `1. **Profile editor**: track-rule grid (order, source, match summary, changes, optional; drag to reorder), detail editor per rule, ...` - fragment present, single line. Matches brief.
- Line 283: `| \`EmptyMatchList\` | error | a present-but-empty \`any\` or \`not\` list (config-time; 4.3) |` - single line. Matches brief.
- `grep -n "EmptyMatchExpression" <spec>` before the edit: no output (exit 1). This is the red run / fire event for Step 4's absence check.
- Positive control: `grep -n "EmptyMatchList" <spec>` before the edit hit exactly `141:` and `283:`, matching the brief's stated positive control exactly.

## Step 2: Amendment 1 applied

Exact-substring replace on the single line at (pre-edit) :374, spec 8.2 view 1 first sentence. Old fragment:

> track-rule grid (order, source, match summary, changes, optional; drag to reorder), detail editor per rule,

New fragment (landed, still one line - now :375 after the earlier table-row insertion shifted it by one):

> track-rule grid (order, source, match summary, changes, optional; drag to reorder; Add appends an empty rule - invalid until filled, announced by validation - selects it and opens its detail editor; Remove deletes the selected rule without confirmation, legal down to zero rules per 4.5), detail editor per rule,

Rest of the item (save semantics, inline markers) untouched - confirmed by `git diff` (see below).

## Step 3: Amendment 2 applied

Inserted one new table row immediately after the `EmptyMatchList` row (spec 5.2), as a single line:

> | `EmptyMatchExpression` | warning | a rule's `match` expression has no conditions at all (no exact/substring/regex/any/not): it would match every track of its source (config-time; suppressed when the emptiness is a present-but-empty top-level `any`/`not` list, which already raises its own `EmptyMatchList` for the same node) |

Exactly one row added; no other table row touched.

## Step 4: verification checks (all foreground, all matched expected)

```
$ grep -c "EmptyMatchExpression" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
1
$ grep -n "drag to reorder), detail editor per rule" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
(no output, exit 1)
$ grep -n "Add appends an empty rule" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
375:1. **Profile editor**: track-rule grid (... Add appends an empty rule - invalid until filled, announced by validation - selects it and opens its detail editor; Remove deletes the selected rule without confirmation, legal down to zero rules per 4.5), detail editor per rule, ...
```

All three match the brief's "Expected" lines exactly (count 1, empty old-fragment grep, exactly one hit on the new clause).

## Step 5: unabbreviated-transcription check - ran, result clean

Diffed both landed blocks against design section 4's amendment text with `diff -wB` (whitespace/line-wrap-insensitive) over normalized single-line extracts:

- Amendment 1: design's quoted replacement block (design :801-805, `sed` + strip `> ` prefixes) vs. the landed parenthetical extracted from spec :375 (`grep -oP` between "drag to reorder; " and "), detail editor per rule"). `diff -wB` exit code: **0** (identical). The parenthetical "- invalid until filled, announced by validation -" is present verbatim in the landed text (confirmed by direct inspection of the diff input, not just the zero exit).
- Amendment 2: design's quoted table row (design :814-819, prefixes stripped, joined) vs. the landed row (spec :284, whole line). `diff -wB` exit code: **0** (identical). The full suppression clause ("suppressed when the emptiness is a present-but-empty top-level `any`/`not` list, which already raises its own `EmptyMatchList` for the same node") is present verbatim.

Both checks ran; both diffs came back clean (no differences beyond whitespace/line-wrap). No truncation, no dropped clause - the plan-7 T21 defect class did not repeat here.

## Additional checks run

- `git diff` on the target file (pre-commit): exactly two hunks, matching the two amendments, nothing else touched.
- Typography sweep on the diff for the banned tell-glyphs (em/en-dash, curly quotes, ellipsis, NBSP): none found. Only plain ASCII hyphens (`-`) used, matching both the design's own text and the preamble's typography rule.
- Scoping re-check: amendment 2 adds exactly one row (`grep -c` = 1); did not touch any of the other 16 un-rowed `diag_codes!` members or attempt to "complete" the table (out of scope per the brief and design section 4's scoping paragraph).

## Step 6: commit

Staged explicitly (`git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, confirmed via `git status --short` that only this one file was staged), committed unsigned with the required trailer:

```
commit 70282fd4c5539ca0a41665ea29ede9afec6f186e
spec: 8.2 names the rule grid's Add/Remove affordance; 5.2 gains the missing EmptyMatchExpression row (plan-7.5 design section 4, amendments 1-2)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
```

Working tree clean after commit. Not pushed.

## Test commands

Task brief carries no cargo/pnpm/e2e run-commands (docs-only task; `grep -i "pnpm\|playwright\|cargo" task-4-brief.md` returns nothing) - the "use `pnpm exec playwright test --grep`" instruction for the pnpm-separator form did not apply to this task. The nine-part gate (BUILDING.md) is a plan-close/whole-branch obligation per the preamble, not a per-task step this brief names; it is not run here.

## Concerns

None. Both anchors, both amendments, and both checks landed exactly as specified; the transcription-fidelity diff came back clean on both blocks.
