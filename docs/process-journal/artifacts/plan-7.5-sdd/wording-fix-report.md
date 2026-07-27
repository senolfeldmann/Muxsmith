# plan-7.5 owner-pass wording fix - implementer report

Date: 2026-07-27
Tree: `/home/senol/Git/Muxsmith` (MAIN, `master`); `.worktrees/` untouched.
Mandate: seven OWNER-RULED verbatim edits (2026-07-27), no judgment calls.

## Edits applied (7/7, all verbatim)

| # | File | Line | Change |
|---|------|------|--------|
| 1 | `help/de/batch-suggestion-card.md` | 9 | `(siehe das Editor-Thema)` -> `(siehe das Thema Editor-Ansicht)` |
| 2a | `help/de/editor-tracks-rules.md` | 15 | `(siehe das Editor-Thema)` -> `(siehe das Thema Editor-Ansicht)` |
| 2b | `help/de/editor-tracks-rules.md` | 15 | `; sie bleibt gesperrt, solange` -> `; die Schaltfläche bleibt gesperrt, solange` |
| 3 | `help/en/batch-suggestion-card.md` | 9 | `(see the Editor topic)` -> `(see the Editor view topic)` |
| 4a | `help/en/editor-tracks-rules.md` | 15 | `(see the Editor topic)` -> `(see the Editor view topic)` |
| 4b | `help/en/editor-tracks-rules.md` | 15 | `; it stays unavailable until` -> `; the button stays unavailable until` |
| 5 | `help/en/editor-tracks-rules.md` | 13 | `A warning flags that new rule until` -> `A warning in its detail panel flags the new rule until` |
| 6 | `help/de/editor-tracks-rules.md` | 13 | `Eine Warnung markiert diese neue Regel, bis` -> `Eine Warnung im Detailpanel markiert die neue Regel, bis` |
| 7 | `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` | 375 | 8.2 view-1: `invalid until filled, announced by validation` -> `incomplete until filled, announced by a validation warning`; `legal down to zero rules per 4.5` -> `legal down to zero rules (per 4.5: `` `unmatched: keep` `` = passthrough, `` `drop` `` = NoTrackRules)` |

Five files, seven changed lines, no other content touched.

## Verification

**String ledger.** Each of the 10 old strings counted exactly 1 before its
edit and exactly 0 after; each of the 10 new strings counts exactly 1.
Fire-check: the pre-edit run of every pattern returned 1, so each pattern is
proven to match real content and the post-edit zero is a real absence, not a
malformed pattern.

**h1s untouched.** All four help h1s unchanged (`# Vorschlagskarte`,
`# Regeln (Spuren)`, `# Suggestion card`, `# Rules (tracks)`); `git diff -U0`
over `help/` yields no `^[+-]# ` line.

**Typography scan.** The 7 added diff lines scanned for em/en dash, figure
dash, horizontal bar, Unicode minus, smart quotes, ellipsis, NBSP: 0 hits.
Positive control: the same pattern against a fixture carrying each forbidden
glyph fired on all 4 bad lines and correctly did not fire on the clean
hyphen line. German diacritics intact (`Schaltfläche`), no transliterated
forms.

**Checks.**
- `pnpm check:i18n`: ok - 41 source files, 211 catalog ids, 19 IpcError codes gated, 22 help ids x 2 help locales, 0 unused warnings, parity ok.
- `pnpm test:e2e`: **62 passed**, 0 failed (expected count 62).

## One finding worth recording: e2e needs `pnpm build` first

The first `pnpm test:e2e` run came back **4 failed / 58 passed**. The failures
were all `help-mode.spec.ts` sidebar-markup compares, with the *new* wording
as expected and the *old* wording as received.

Cause is not the edits. `playwright.config.ts` drives `vite preview` over
`dist/`, and its own comment states the precondition: "`dist/` must already
exist (`pnpm build`), which the per-commit gate runs before `pnpm test:e2e`".
`test:e2e` builds only the two e2e harness bundles, never `dist/`. The
`dist/` on disk was from 12:58, pre-edit, and carried the old wording twice.
The spec's expected side reads `help/en/*.md` from disk at test time (fresh),
so any help-text edit desynchronizes a stale `dist/` and fails exactly these
4 tests.

Ran `pnpm build` (documented precondition; `dist/` is gitignored, so no repo
effect), confirmed `dist/` then carried the new wording twice and the old
zero times, and re-ran: 62/62 green.

Note for whoever runs these checks next: **`pnpm test:e2e` alone is not a
valid gate after any `help/**.md` or i18n-catalog edit.** `pnpm build` must
precede it, or 4 help-mode tests fail spuriously.

A repo-wide `grep -rIl` for the old wording had initially reported the stale
`dist/` bundle as clean; `-I` classified the minified bundle as binary. With
`grep -a` it showed 2 hits. Use `-a` when sweeping build output.

## Scope notes

- Item 7 deliberately sharpens the plan-7.5-design-transcribed amendment text
  on owner authority. The design's section-4 text stays historical; the spec
  is the living document.
- The topic-name edits stay inside D71's claim enumeration; the panel naming
  was pre-cleared by the whole-branch triage.

## Commit

`406e91bb9c413fa60e9cdf72e54d3d2479586fb3` on `master` - unsigned
(`git -c commit.gpgsign=false`, verified `%G?` = `N`), exactly the five files
staged explicitly (5 files changed, 7 insertions, 7 deletions), repo trailer
`Co-Authored-By: Claude Fable 5` per the repo's own convention (this repo's
log carries no `Claude-Session` line). Working tree clean. **Not pushed**
(1 commit ahead of `origin/master`).

Amended once: the first commit message transliterated `Schaltfläche` to
`Schaltflaeche` in a quoted German fragment, against the typography
convention, which binds commit messages too. Fixed before handover; the
pre-amend hash `3232d38` is dead.
