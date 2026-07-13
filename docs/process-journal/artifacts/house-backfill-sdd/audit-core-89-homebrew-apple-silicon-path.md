# Audit: core-89-homebrew-apple-silicon-path (PROMOTION candidate)

**Auditor verdict: CONFIRMED.** verified_count = 3 distinct occurrences (claimed 3). Promotion stands.

All three cited refs exist, are on-topic, support their stated `kind`, and are genuinely
independent attestation points. None is fabricated, misattributed, or a same-artifact
duplicate of another. Critically, this is *not* a core-22-style single-episode paper trail:
the pattern genuinely recurred across two independent review events that reached
**conflicting dispositions** (defer at T3, reverse-and-fix at whole-branch).

---

## Claim under audit

- **Statement:** `/opt/homebrew/bin/mkvmerge` (Apple-Silicon Homebrew) is a detection
  candidate: a Finder-launched Tauri app does not inherit the shell PATH, so without it
  detection fails for the most common macOS install route. First excluded under the SI-3
  evidence rule (no formula in the mkvtoolnix source tree), then added when the exclusion
  was found to rest on the wrong authority.
- **Cluster (cluster-core.md:221):** three occurrences, `2026-07-10` — deferred (task-3
  verdict, Minor/product-scope-to-T7); violated-corrected (whole-branch verdict triage-14,
  FIX-NOW); violated-corrected (commit 5e76a15).
- **Promotion basis:** count = 3 -> standing convention.

## Occurrence-by-occurrence verification

| # | ref | artifact | on-topic? | supports `kind`? | verdict |
|---|-----|----------|-----------|------------------|---------|
| 1 | task-3 verdict, Minor (product-scoping to T7) | `docs/process-journal/artifacts/plan-5-sdd/verdicts/task-3-review-verdict.md:44-45` | yes | yes — `deferred` | **KEEP** |
| 2 | whole-branch verdict triage-14 (FIX-NOW) | `docs/process-journal/artifacts/plan-5-sdd/verdicts/whole-branch-review-verdict.md:72` | yes | yes — `violated-corrected` | **KEEP** |
| 3 | commit 5e76a15 | `git show 5e76a15` (runtime.rs `platform_candidates()`) | yes | yes — `violated-corrected` | **KEEP** |

### #1 — T3 per-task review verdict (deferred) — CONFIRMED

`task-3-review-verdict.md:44-45`, under **"#### Minor (Nice to Have)"**:

> **Homebrew Apple Silicon path dropped, likely a real-world gap for T7.** `runtime.rs`
> drops `/opt/homebrew/bin/mkvmerge` because no Homebrew formula exists in the `mkvtoolnix`
> checkout itself (it lives in the separate `homebrew-core` repo, outside SI-3's evidentiary
> scope). This is a faithful, transparently-documented application of the "verify against
> packaging/, never from memory" instruction, so it's not a task defect [...] Worth flagging
> forward to whoever scopes T7's GUI-facing candidate list, since it's a product-completeness
> question the mkvtoolnix source tree genuinely cannot answer.

Rated **Minor**, exclusion attributed to the SI-3 evidence rule, disposition = **defer to T7**.
Exactly matches ref and `kind: deferred`. (The `progress.md:29` "T7 carry-forward (from T3
review)" line is the same review event's progress-doc echo — correctly *not* listed as a
separate occurrence, per the ledger's same-review-cited-twice collapse rule.)

### #2 — whole-branch review verdict, triage item 14 (FIX-NOW) — CONFIRMED

`whole-branch-review-verdict.md:72`, "## Triage of the 16 accumulated items", item 14:

> **FIX-NOW** — one line, and the exclusion rests on the wrong authority: mkvtoolnix's
> packaging tree can't testify about Homebrew, whose own documented prefix is `/opt/homebrew`
> on Apple Silicon. The sharpening fact: **GUI apps launched from Finder do not inherit the
> shell PATH** [...] detection fails for the most common macOS install route, while
> `/usr/local/bin` already incidentally covers Intel brew. Cite Homebrew's docs in the comment.

The wrong-authority diagnosis and the Finder-PATH fact are both present here. Disposition =
**FIX-NOW**, reversing #1's defer. Matches ref and `kind: violated-corrected`.

### #3 — commit 5e76a15 (the fix) — CONFIRMED

`fix(core): add Homebrew Apple Silicon prefix to macOS mkvmerge candidates` (2026-07-10
19:48). Adds `/opt/homebrew/bin/mkvmerge` to `platform_candidates()`'s macOS branch, rewrites
the doc comment to verify against Homebrew's own install docs
(`docs.brew.sh/Installation`) instead of the mkvtoolnix packaging tree, and adds the
candidate-list test assertion. It **is** the correction. Matches ref and
`kind: violated-corrected`.

## Independence analysis (the load-bearing question)

Every ref is authentic and on-topic; the only way this promotion fails is if two of the three
collapse into one arising. The sibling audit **core-22 was REJECTED on exactly that ground**
("one event's SDD paper trail counted as three"), so the same test must be applied here — and
it is applied, not waved away.

**Why core-89 is not core-22.** core-22's decisive factual predicate was *"the collision arose
exactly once in the entire project"* — one task, one 2-minute violation-then-fix, with the
review and journal being retrospective records **of that single commit**. core-89 fails that
predicate: the homebrew question genuinely arose in **two independent review events reaching
opposite conclusions**:

- occ #1 — the **per-task T3 review** examined the exclusion and rated it Minor / defer-to-T7.
- occ #2 — the **whole-branch review**, a separate review event (distinct `review_target`,
  `agent_id`, `tool_use_id`, and `final_message_ts` in the salvage headers of the two verdict
  files), independently re-litigated the same question and **reversed** the disposition to
  FIX-NOW on the wrong-authority grounds.

That is textbook genuine recurrence: the concern was raised, given one answer, then raised
again in a later independent review and given a different answer. It is the opposite of
core-22's single-episode padding. These two alone are two distinct arisings.

**Does the fix commit (#3) collapse into #2?** No. Two independent reasons:

1. The ledger's own occurrence-counting rule (cluster-core.md header) explicitly keeps this
   pair distinct: *"a fix commit that follows a review is its own touchpoint (find-vs-fix are
   two events)."* This is not a loophole invoked for this case; it is the uniform methodology
   under which sibling promoted clusters core-12 and core-21 also promote (review find + fix
   commit as two).
2. The collapse cases the header names are *"a single review event cited via both its verdict
   file and a journal mention"* and *"a co-cited commit-set implementing one work item."*
   Neither fits #2 vs #3: they are different artifact classes (a 16-item triage decision vs a
   source change to `runtime.rs`) performing different actions (decide vs implement). core-22's
   collapse was of a review that *reviewed an already-existing fix commit* plus a journal echo;
   here the review **precedes and orders** the fix and carries the independent analytical
   content (wrong-authority + Finder-PATH), while the commit is a separate, later, on-topic
   artifact. No two refs point to the same artifact or the same action.

Even under the strictest "distinct time the pattern arose" reading, occ #1 and occ #2 are
unambiguously two independent arisings; the find-vs-fix rule makes #3 a third. The recurrence
is real.

## Verified negatives

- The two verdicts are **different review events**, not one cited twice: `task-3-review-verdict.md`
  has `review_target: task-3`, `whole-branch-review-verdict.md` has `review_target: whole-branch`,
  with distinct `agent_id`/`tool_use_id`/timestamps.
- The `progress.md:29` carry-forward and the `final-fix-wave-plan5-report.md:147` "Fix 6"
  writeup are **not** separately listed — they are, respectively, an echo of #1 and a report
  of #3, correctly collapsed.
- `git log --grep` on the homebrew/opt/homebrew candidate returns 5e76a15 as the sole fix
  commit — no second correction commit is being double-counted.

## Disposition

- **Verdict:** CONFIRMED — 3 distinct occurrences survive.
- **verified_count:** 3.
- **Action:** promotion stands; core-89 is legitimate house-knowledge (a standing detection-
  candidate convention).
- **Substance also checks out** (not the audit's job, but confirmed in passing): `/opt/homebrew`
  is Homebrew's documented Apple-Silicon prefix, the Finder-does-not-inherit-PATH fact is
  correct, and the SI-3-authority correction (verify a Homebrew path against Homebrew's docs,
  not mkvtoolnix's packaging tree) is sound.
