# T1 verdict: ci.yml workflow-level permissions (Stream A, verdict item 9)

Independent review of commit `7c75f00a4f587ae07fd5f1b6b1b78e0a20579d60` on
`plan57-a` (worktree `.worktrees/plan57-a`). Reviewed against the plan
(Task 1 + standing constraints), adjudication verdict item 9, the Tier-2
house files (`docs/conventions.yaml`, `docs/process-conventions.yaml`),
and the diff itself; every implementer-report claim re-verified on disk.

## VERDICT: APPROVED

One minor house-style finding (comment wrap), routed to the fix wave or
the T5 funnel at the controller's discretion; no substance, security, or
claim-accuracy defect. Both surfaced deviations adjudicated below.

## Checks performed (all independently re-run, not taken from the report)

1. **Block is workflow-level, valid, `contents: read` only - PASS.**
   `git show 7c75f00`: exactly 1 file, 4 insertions, ci.yml only.
   PyYAML parse (re-run by this reviewer): top-level keys
   `['name', True, 'permissions', 'jobs']`; workflow-level
   `{'contents': 'read'}`; job-level permissions `None` for both `test`
   and `deny` (so nothing overrides the workflow block). `grep -rn
   permissions .github/workflows/` -> single hit at ci.yml:10. Placement
   matches the adjudication's "directly under `on:`" (after the `on:`
   block, before the pre-existing pinning-policy comment). The `True`
   top-level key is YAML 1.1's bare `on:` - pre-existing, unrelated.

2. **Least-privilege audit - HOLDS.** Walked every step of both jobs
   against the current ci.yml in the worktree:
   - `actions/checkout@9c091bb` (x2): clones this repo with
     `GITHUB_TOKEN`; `contents: read` is exactly the documented minimal
     scope. The `tags: ['v*']` trigger changes nothing - this workflow
     creates no releases (plan-6 packaging is a future separate workflow,
     per adjudication item 9).
   - `Swatinem/rust-cache@c193711` (v2.9.1): talks to the Actions cache
     service via the runner's `ACTIONS_RUNTIME_TOKEN`, which the
     `permissions:` key does not govern; save/restore unaffected. It does
     not delete caches via the REST API (which would need
     `actions: write`). Implementer's token-semantics claim is correct.
   - `jdx/mise-action@e6a8b39` (v4.2.0): default `github.token` only for
     public GitHub API reads (release download / rate limit); no write
     scope involved.
   - `EmbarkStudios/cargo-deny-action@bb137d7`: cargo-deny fetches the
     RustSec advisory DB anonymously; no token scopes.
   - All rustup / apt / choco / brew / pnpm / cargo / Playwright steps:
     no `GITHUB_TOKEN` use at all. The skip-marker step's `::error::` is
     a workflow log command, not a Checks-API write.
   - Nothing pushes, comments, uploads release artifacts, or publishes.
   `contents: read` (all other scopes implicitly `none`) suffices
   workflow-wide. This also re-answers the plan's standing reviewer
   question ("do rust-cache / mise-action / all steps need more?") - no,
   confirming the adjudication.

3. **Nothing else changed - PASS.** Diff is the 4 inserted lines only.
   Branch topology: `plan57-a` = master HEAD (`cd5e917`, the plan commit)
   + exactly this one commit; merge-base = master HEAD. Worktree clean
   (`git status --porcelain` empty).

4. **Comment claim content - ACCURATE.**
   - "repo default verified read on 2026-07-14": matches adjudication
     item 9 (`gh api .../actions/permissions/workflow` ->
     `default_workflow_permissions: "read"`) and the logged call in
     `gh-log.md:328` dated 2026-07-14. Verified, not asserted.
   - Out-of-band-drift rationale: matches item 9's open head (b).
   - "satisfies OSSF Scorecard": precision nit only - it satisfies
     Scorecard's *Token-Permissions check*, not Scorecard wholesale.
     The plan's own Task 1 text uses the same shorthand, so the comment
     transcribes the ratified wording. Accept; no change requested.

5. **Commit hygiene per standing constraints - PASS.** Unsigned
   (`%G?` = N, per proc-05 / SI-4), Co-Authored-By trailer present,
   message scoped and accurate; report states file-explicit staging and
   the clean worktree corroborates.

6. **Implementer report vs disk - NO DISCREPANCIES.** All verification
   claims reproduced (YAML parse, structural asserts, grep positions,
   diff stat). Report claim "no local workflow linter available"
   confirmed: none of actionlint / zizmor / action-validator on PATH.

## Findings (ranked)

**F1 (minor, house style): rationale comment exceeds the file's comment
wrap.** ci.yml:9 is 170 chars; every other workflow-level policy comment
in the file wraps at <=78 (next-longest workflow-level comment lines: 78,
78, 77; the lone pre-existing 121-char line at :110 sits *inside* a run
script, a different context). The general house doctrine (match the
dominant local pattern) applies even though no Tier-2 entry records a
wrap width. Fix: re-wrap to 2-3 lines at ~75 cols, content unchanged.
Zero-risk, comment-only. Non-blocking; fix wave or T5 funnel.

No other findings. ci-10 (pins) untouched and respected; ci-01 (matrix)
untouched; ci-08 (gated tests) untouched - all verified in the diff, not
just asserted.

## Deviation adjudications

**(a) Narrowed per-task verification vs ci-06 - LEGITIMATE PER PLAN;
tension real; recording recommended at close (controller action, not a
Stream-A fix).**
Steelman for strict ci-06 first: the entry says the nine-part gate runs
"before every commit, never skipped", the plan's own Standing constraints
restate it, and the 2026-07-13 occurrence shows skipping is exactly how
gate discipline erodes. Against that: (i) the plan's Task 1 Verify clause
*explicitly* narrows T1 to workflow-lint (none available) + YAML
well-formedness with the on-runner proof at T5 - the specific,
owner-ratified instruction controls over the plan's general constraint;
(ii) the diff has zero Rust/frontend surface, so the nine-part gate on
this commit would re-test the already-gated base (`cd5e917`), yielding no
evidence about this change - the *relevant* verification is precisely
what ran, plus the T5 CI run which exercises this very file on-runner;
(iii) the plan-5.6 T1 precedent (ci-06 occurrence 2026-07-13) fixes the
mechanism: disclose -> reviewer flags -> controller rules, which the
implementer followed to the letter. Ruling: no violation, nothing to fix
in the worktree. But this is now the second disclosed narrowing event,
and ci-06's text carries no docs/CI-only-diff boundary - per the repo's
own restriction-scoping doctrine, the controller (single ledger writer)
should record an occurrence on ci-06 at T5 (kind: decided, sanctioned
narrowing for diffs with no gate-covered surface, ref plan 5.7 T1), or
amend the statement with that explicit exemption. Also note the standing
"full gate re-run after every merge" still applies at merge time and is
untouched by this ruling.

**(b) Comment exceeding ~75-col wrapping - FIX-WORTHY (minor,
non-blocking).** The plan said "two lines plus comment" without fixing
the comment's shape; the one-line form came from the controller brief, so
the implementer transcribed faithfully - the deviation is the brief's,
not the implementer's. Still, the file's dominant pattern is unambiguous
and this becomes the longest line in the file by 49 chars, in the repo
whose supply-chain posture is meant to be a showpiece at 1.0. Re-wrap
(F1). Accepting as-is would also be defensible on pure cost grounds;
fix-worthy wins because the fix is free and the file is uniform today.

## Harvest (house dimension, per Tier-2 reviewer brief)

- **Dominant pattern, candidate for the ledger (nature: operational,
  domain: ci): ci.yml policy comments carry decision provenance** - date
  plus decision/plan/verdict ref - in-file, next to the mechanism they
  govern ("Pinning policy (2026-07-10)", "Static 3-OS matrix since going
  public (2026-07-10)", "Version-pin decision (registries checked
  2026-07-11, Şenol's standing preference...)", "Plan 5.5 Task 12
  (#18b)", and now "verdict item 9 ... verified ... 2026-07-14"). The new
  comment conforms. That is at least 5 independent occurrences of
  auditable-provenance-in-ci.yml; nothing in the Tier-2 files records it
  yet.
- **Repeated pattern reinforcing an existing entry:** deviations
  disclosed, never silently resolved - plan-5.6 T1 precedent explicitly
  cited and re-enacted here (two surfaced deviations, both non-silent).
  Reinforcement occurrence candidate for ci-06 (and consistent with
  proc-01's disclosure culture).
- **Repeated-rejection shape worth watching:** "run the full gate on a
  commit the gate cannot observe" has now been rejected twice in favor of
  fit-for-purpose verification (5.6 T1 retro-verify, 5.7 T1 narrowing).
  If it recurs a third time, that is the agent-emergent promotion
  threshold for an explicit gate-scope rule instead of repeated one-off
  rulings.

## Summary

APPROVED. 1 minor finding (F1 comment re-wrap, non-blocking). Deviation
(a): legitimate per plan, record ci-06 occurrence at T5. Deviation (b):
fix-worthy minor, brief-induced. Least-privilege audit independently
confirmed; comment claims verified against gh-log.md and the
adjudication; diff minimal and clean.
