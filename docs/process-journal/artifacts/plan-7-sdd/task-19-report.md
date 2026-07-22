# Task 19 report - D62 help-topic gate

**Verdict:** DONE_WITH_CONCERNS (2 concerns, both surfaced below - neither is a blocker; the gate is green on the real tree and every check is fire-verified both directions).

**Commit:** `a653d39` on branch `plan7-g` (parent `45f0e31`).
**File:** `scripts/check-i18n.mjs` (+130 / -6). ESLint clean (`eslint.config.js`); prettier not installed in this tree.

---

## What changed

Added D62's help-topic gate to `scripts/check-i18n.mjs` as a fifth hard-fail
array (`helpErrors`), folded into the existing exit condition and ok-line.

- **Header comment widened**: "Four independent hard-failure checks" ->
  "Five", "the other two" -> "the other three", now naming D62's
  help-topic-tree gate. This is the conventions.md sweep of the count/
  enumeration that depends on the hard-fail set I extended.
- **Referenced help-id set** built from three literal scans over the
  already-loaded `fileTexts` plus `src/help/state.ts`:
  `helpId:` registry literals (a), `data-help-id="..."` template literals
  (b), `VIEW_TOPICS` `view-*` values (c). (c) redundant-with-(b) by design.
- **Six hard-fail conditions** feed `helpErrors`:
  1. referenced -> file, per locale (missing file names id, locale, site)
  2. file -> referenced (orphans)
  3. `help/` vs `locales/` locale-set lockstep
  4. external-URL ban
  5. table/pipe ban (ZERO-PIPE)
  6. raw-HTML ban (inline code spans exempt)
- **ok-line** gains `22 help id(s) x 2 help locale(s)` (the brief's stated
  expected `22 x 2`).

---

## CONCERN 1 - scope: checks 5-6 exceed the brief's four, which the design calls "exhaustive"

The task-19 brief (Step 1) sketches **four** checks and the ok-line note
"expected 22 x 2". Design section D62 (main-repo
`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:1304-1367`,
and line **1885**: "the four D62 checks are the exhaustive extension set")
likewise enumerates exactly four: referenced->file, orphans, lockstep,
external-URL ban. Neither the brief nor the design contains a table check
or a raw-HTML check.

The run's **two cross-task controller constraints** handed to me name "the
ZERO-PIPE table check" and "the raw-HTML check" as binding, with mechanism
requirements (zero-pipe over pipe-pair; exempt inline code spans, or the
two pattern-topic files go red). The framing was explicit: these "override
any conflicting mechanism the brief might sketch; surface it if they do
conflict."

I treated this as a controller-resolved extension, not a
fork-to-stop-on: the controller enumerated both checks and their
mechanisms (nothing for me to invent), the constraints are additive with
zero outward effect (gate-only, no API/data-format change, verification
only strengthened), and both pass green on the real tree - the controller's
own tree-specific detail ("`(?<season>...)` in both pattern topics ... two
legitimate files would go red without the exemption") proves the checks
were designed against this exact help content. So I implemented all six and
surface the conflict here.

**Reconciliation the controller must do:** the design's line-1885
"exhaustive four" and the brief's four-check sketch are now stale relative
to the six-check gate. If the intended D62 really is four checks only,
reverting checks 5-6 is a clean 2-block deletion (the `// 4-6.` content
loop drops back to the brief's check-4-only form). My read is that the
controller's binding constraints are authoritative and the doc needs the
update, not the code.

## CONCERN 2 - premise refutation: the brief's `data-help-id` regex over-captures on the real tree

The brief's `DATA_HELP_ID_RE = /data-help-id="([^"]+)"/g`, run verbatim
against the tree, captures **24** referenced ids, not the 22 the brief
itself expects - two are garbage:

- `${id}` from `src/App.vue:124` - the runtime lookup
  `querySelector(\`[data-help-id="${id}"]\`)`.
- `spec.helpId` from `src/editor/widgets/FieldWidgetDispatcher.vue:73` -
  the dynamic Vue bind `:data-help-id="spec.helpId"`.

Both make check 1 (referenced->file) **permanently red on legitimate
content** (no `help/*/${id}.md`, no `help/*/spec.helpId.md`) - the exact
defect class the run flags ("permanently red on legitimate content is the
defect class this plan has already hit twice").

**Minimal adaptation (surfaced):** constrain the captured value to the
help-id grammar `[a-z][a-z0-9-]*` on both literal regexes. This drops the
two garbage captures (both start with / contain non-grammar characters),
keeps every real id, and yields exactly the brief's expected **22**.
`VIEW_TOPIC_RE` already carried the same `view-[a-z-]*` grammar posture, so
this makes the three id-scans uniform. `HELP_ID_PROP_RE` had no false
positive on the current tree (it was clean at `[^'"]*`); I constrained it
too for consistency/robustness, not out of a demonstrated defect - if the
reviewer wants strict-minimal, reverting only `HELP_ID_PROP_RE` to
`[^'"]*` is safe and loses nothing on this tree.

### Minor note - table check does not exempt code spans (by constraint)

Per the constraints the ZERO-PIPE table check bans **any** `|`, including
inside code spans; only the raw-HTML check got the code-span exemption. The
two pattern topics use no regex alternation today (zero pipes tree-wide), so
green. A future topic needing `(mkv|mka)` in a code span would go red and
have to rephrase. This is faithful to the stated constraint (the controller
distinguished the two checks deliberately); flagged only so the asymmetry is
a known decision, not an oversight.

---

## Brief steps

- **Step 1 (implement four checks):** done, with the two adaptations above
  (regex grammar constraint; +checks 5-6 per controller constraints). Header
  comment widened as instructed.
- **Step 2 (fire-verify):** done for all six, both directions (below).
- **Step 3 (full gate + commit):** done. Gate green, committed `a653d39`
  with explicit `git add`, `commit.gpgsign=false`, exact Co-Authored-By
  trailer. Kept the brief's exact subject line; added a body paragraph
  recording the controller-directed checks 5-6 and the regex adaptation so
  `git log` is honest about full scope.

TDD/verification order: this gate has no fixture self-test (decision-ledger
`testing-check-i18n-self-test` explicitly deferred it; e2e covers only the
parser-blindness half), so the canonical method here - as in T17/T18 - is
fire-verify: break -> watch red, restore -> watch green. Done per check.

---

## Fire-verification evidence (all foreground, tree clean after each)

Baseline before/after any break: `node scripts/check-i18n.mjs` -> exit 0,
`... 22 help id(s) x 2 help locale(s) ...`.

| Check | Break | Broken result | Restored |
|---|---|---|---|
| 1 referenced->file | `data-help-id="plan7-bogus"` in JobsView | exit 1; `help id "plan7-bogus" (referenced at src/views/JobsView.vue:337) has no help/de/plan7-bogus.md` + `.../en/...` | exit 0 |
| 2 orphans | create `help/en/orphan.md` | exit 1; `help/en/orphan.md: orphan topic (no helpId/data-help-id/VIEW_TOPICS reference)` | exit 0 |
| 3 lockstep | `mkdir help/fr` + one file | exit 1; `help/ locales [de,en,fr] != locales/ [de,en] (lockstep, D62)` | exit 0 |
| 4 external-URL | `https://example.com` in view-batch.md | exit 1; `help/en/view-batch.md: contains an external URL (banned, D62 check 4)` | exit 0 |
| 5 table/ZERO-PIPE | `col a \| col b` (single pipe, headerless) | exit 1; `help/en/view-batch.md:22: contains a table/pipe character (banned, D62 check 5)` | exit 0 |
| 6 raw-HTML | see discrimination below | exit 1 | exit 0 |

**Check 6 exemption discrimination (the load-bearing part):**
- Angle brackets **inside** a code span (`` `<span-in-code>` ``) -> exit **0**,
  no raw-HTML error (exemption holds; matches the real `(?<season>...)` case).
- Same angle brackets **outside** a code span (`<span-raw>`) -> exit **1**;
  `help/en/view-batch.md:22: contains raw HTML (banned, D62 check 6; inline code spans exempt)`.
- The real tree is green with `help/en|de/editor-input-pattern.md` carrying
  `(?<season>\d{2})` / `S(?<season>\d{2})E(?<episode>\d{2})` in code spans -
  green direction on real content, confirming the constraint's warning
  ("two legitimate files would go red without the exemption") is satisfied.

`git status --porcelain` after the full fire-verify sweep: only
` M scripts/check-i18n.mjs` (no residual test artifacts; the interactive
`rm` alias was bypassed with `command rm -f`).

## Commands run (key ones)

- `node scripts/check-i18n.mjs` - baseline green (pre and post), and after each restore.
- Probe of brief regexes vs tree -> 24 ids incl. `${id}`, `spec.helpId` (concern 2).
- Probe of all six checks with grammar-fixed regexes -> 22 ids, `helpErrors` = 0.
- Per-check break/run/restore (table above), each foreground.
- `npx --no-install eslint scripts/check-i18n.mjs` -> clean.
- `git add scripts/check-i18n.mjs` + `git -c commit.gpgsign=false commit ...` -> `a653d39`.

## Surfaced items (for reviewer / controller)

1. **Checks 5-6 exceed the brief's four and the design's "exhaustive four"
   (line 1885).** Built per binding controller constraints; design doc +
   brief need reconciliation (concern 1).
2. **Brief's `data-help-id` regex over-captures 2 garbage ids on the real
   tree** (`${id}`, `spec.helpId`), which would be permanently-red. Fixed by
   grammar-constraining the capture; also applied to `helpId:` for
   consistency (revertable) (concern 2).
3. **Table check bans pipes even inside code spans** (per ZERO-PIPE
   constraint; only raw-HTML got the code-span exemption) - deliberate,
   green today, flagged as a known asymmetry.
4. **No new imports, no API/data-format change, verification only
   strengthened** - the structural-conformance grant covers the additive
   extension.
