<!--
Salvaged 2026-07-10 from SDD session transcript; verdict arrived only in context, never materialized as a file.
  review_target:      task-12  (round 2 of 2)
  session_uuid:       62503ddd-59d4-469d-99d2-a9f5d85f25a5
  session_transcript: /home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5.jsonl
  tool_use_id:        toolu_01JeqcWL5z5PYv9MyNd1JzA5
  agent_id:           a97d2e2389a779e21
  subagent_transcript:/home/senol/.claude/projects/-home-senol-agents-peter/62503ddd-59d4-469d-99d2-a9f5d85f25a5/subagents/agent-a97d2e2389a779e21.jsonl
  dispatch_desc:      Review Task 12 (spec + quality)
  agent_internal_round: 2 of 2
  final_message_ts:   2026-07-10T17:22:43.765Z
  continuation_trigger: The coordinator sent a message while you were working: Re-review request for your Task 12 
Body below is byte-faithful to the reviewer subagent's final message for this round, except this comment.
STATUS: NOT COMMITTED until Şenol reviews.
-->

### Spec Compliance (re-review of 945ee96)

- ✅ **Important finding closed, in the path CI executes.** `package.json:13` on disk now reads `"test:e2e": "tsc --noEmit -p e2e/tsconfig.json && vite build ... && playwright test"`, and CI's "Playwright smoke (GUI e2e)" step (`.github/workflows/ci.yml:71-73`, untouched) runs exactly `pnpm test:e2e` — pnpm run-scripts prepend `node_modules/.bin` to PATH, so `tsc` resolves to the already-pinned typescript 6.0.3 (present in devDependencies, no new dep, confirmed `node_modules/.bin/tsc` exists). The `&&` chain means a type failure (exit 2) stops before the harness build or Playwright ever run — the claimed RED-probe ordering is structurally guaranteed, not incidental.
- ✅ **"Plain tsc suffices" claim verified**: nothing under `e2e/` imports a `.vue` file (all e2e imports are `../src/ipc` (pure .ts), `@playwright/test`, `@fluent/bundle`, node builtins), so no vue-tsc needed. I ran `tsc --noEmit -p e2e/tsconfig.json` against this tree myself: exit 0, green.
- ✅ **Comment fix is accurate — and in one respect more accurate than my original finding.** I verified all three claims against the actual catalogs: (a) multiline selector values genuinely exist today (`diagnostics.ftl:17-20` `invalid-template`, `gui-jobs.ftl:26-29` `jobs-row-warning-count` — my original Minor wrongly said the catalogs were all one-liners; my probe grepped for lines ending in `=`, which selector openers ending in `->` don't match), and the parser handles them correctly: the id registers off the first line, and no continuation line can match `^[A-Za-z]...=`. (b) Attributes: a `$t("msg.attr")` literal would be captured by `CALL_RE` and flagged missing since `knownIds` lacks the dotted form — the comment's prediction and its "extend parseCatalogIds then" remediation pointer are both correct. (c) Terms: leading `-` fails the regex and terms are catalog-internal by Fluent semantics — correct as-is.
- ✅ **No regression.** The diff is one script line plus a comment block; no test, mock, fixture, CI, or catalog content changed. The e2e steps remain Linux-only in CI and `test:e2e` is invoked nowhere else.
- ⚠️ RED/GREEN drift-probe run (TS2561 on a seeded `duration_ms -> durationMs` rename, then 3/3 green) — not reproducible from the diff, but the mechanism is now real: that exact rename in `ipc.ts` would fail the `satisfies JobEvent` fixture through this tsc step, so the claim is credible as described.

### Strengths

- The fix is minimal and lands the check in the one place that guarantees both local and CI coverage (inside the script, not as a separate CI-only step that local runs would skip).
- The new parser-constraint comment mirrors `run.rs::ftl_message`'s documented-constraint pattern as requested, and does it with per-construct precision rather than a vague disclaimer.

### Issues

#### Minor (Nice to Have)

- One phrase in the new comment is slightly imprecise: "continuation lines are indented, so they can never register a bogus id" — the closing `}` of a selector block sits at column 0 (`diagnostics.ftl:20`, `gui-jobs.ftl:29`), i.e. not indented. The conclusion still holds for the actual reason (the regex requires a letter at column 0, and Fluent forbids column-0 variant text), so this is cosmetic, not a correctness gap. Not blocking.

### Assessment

**Task quality:** Approved

**Reasoning:** The Important finding is genuinely closed — the type-check now sits first in the exact script CI executes, verified on disk and against the unchanged CI step — and the documentation fix is accurate against the real catalog contents, correcting an imprecision in my own original finding along the way. The tiny diff introduces no regression.