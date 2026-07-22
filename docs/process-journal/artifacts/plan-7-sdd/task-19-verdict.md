# Task 19 verdict - D62 help-topic content gate (independent review)

**Combined verdict: APPROVED.**
**Spec-compliance: PASS** - all six checks of the amended D62 implemented per its letter, incl. the check-5-vs-6 code-span asymmetry and the lockstep/en-reference structure; 22 help id(s) x 2 help locale(s) independently recomputed from the tree.
**Quality: PASS** - `pnpm lint` exit 0, `pnpm build` exit 0, `pnpm test:e2e` 52 passed exit 0, gate exit 0; every one of the six checks fire-verified RED in scratch and restored byte-identically; tree git-clean after.

Reviewed against the AMENDED D62 (main repo `docs/.../2026-07-21-plan7-help-i18n-design.md:1304-1416`, round-8 amendment folding checks 5-6), not the stale worktree copy or the four-check brief. HEAD `a653d39` on `plan7-g`, single commit, `scripts/check-i18n.mjs` +130/-6.

---

## Findings by severity

### Blockers
None.

### High / Medium
None.

### Low
- **L1 - grammar constraint introduces a narrow CI blind spot (evidenced).** Constraining the capture to `[a-z][a-z0-9-]*` (both `HELP_ID_PROP_RE` and `DATA_HELP_ID_RE`) means a *malformed*, non-kebab id literal (uppercase/underscore/leading-digit/dot) is silently **not** captured, so check 1 never fires on it. Fire-proved: a scratch `helpId: "Editor_Bad"` literal in a scanned `.ts` leaves the gate GREEN (exit 0), whereas the brief's `[^'"]*` would have captured it and gone RED on the missing file. A *kebab-but-bogus* id (`editor-bogus-xyz`) still goes RED, so the common typo class stays caught. This is the accepted cost of the necessary over-capture fix (see Q1); runtime degrades **visibly** (D51 raw-id fallback renders the literal id), not silently, so severity is low. Watch item, not a fix gate.
- **L2 (info) - `RAW_HTML_RE` per-line inline-span strip is inline-only.** The strip `line.replace(/`[^`]*`/g, "")` exempts inline code spans but not fenced (```` ``` ````) blocks, and `/<\/?[a-zA-Z]/` would false-positive prose like `a<b`. Both are consistent with the design letter ("outside an inline code span"); neither occurs in the current tree (no fenced blocks; every `<` sits inside a single-backtick span). Watch item.
- **L3 (info) - ok-line wording.** Emits `... help id(s) x N help locale(s)` where the brief string said `... locale(s)`. Trivial, arguably clearer (disambiguates from catalog locales). Not a defect.

---

## Numbered adjudications

### Q1 - the `DATA_HELP_ID_RE` grammar constraint: correct minimal adaptation? cannot under-capture?
**Correct minimal adaptation; cannot under-capture on the closed set.** The brief's `/data-help-id="([^"]+)"/g` + `helpId:\s*...([^'"]*)` over-captures two garbage ids on the real tree - `${id}` (`App.vue:124` querySelector template literal) and `spec.helpId` (`FieldWidgetDispatcher.vue:73` dynamic Vue bind) - making check 1 permanently RED on legitimate content. Constraining the capture to the help-id grammar drops exactly those two (`$` fails `[a-z]`; `spec` is followed by `.` not `"`) and nothing else.

**Under-capture: none.** Walked the closed set of every referenced id (18 `helpId:` registry literals + 5 static `data-help-id` literals + 3 `VIEW_TOPICS` values, 22 unique): all are lowercase kebab starting with a letter, every one fully conforms to `[a-z][a-z0-9-]*` (and `view-[a-z-]+` for the view scan). Verified three independent ways: (a) manual walk of the closed set; (b) an independent recomputation script replicating the three regexes -> exactly the 22 expected ids, the brief-unconstrained form -> 24 with the two named extras; (c) both gate directions GREEN = referenced-set and file-set form a bijection of size 22. The residual is L1 (malformed *illegitimate* literals silently skipped), which is a non-catch of a defect, not an under-capture of a legitimate id.

### Q2 - also constraining `HELP_ID_PROP_RE` "for consistency": within grant, or gratuitous?
**Within the justified-consistency / structural-conformance grant; keep, with one caveat.** `HELP_ID_PROP_RE` was clean on the current tree (my probe: it captured the same 18 ids either way; `helpId: string` in `topics.ts` is not captured because it is unquoted). Constraining it gives all three id-scans one uniform grammar posture (`VIEW_TOPIC_RE` already carried it) - the house "match the sibling pattern" - it is revertable and loses nothing on this tree. **Caveat:** it is not purely "verification strengthened" as the report frames it (surfaced item 4) - it extends the same L1 blind spot to `helpId:` property literals. Net acceptable: the asymmetry of constraining two scans and not the third would itself be a wart, so uniform treatment is the cleaner call. Not worth a fix cycle; the report correctly flagged it as revertable.

### Q3 - the check-5 pipes-everywhere asymmetry: matches the letter? any current proximity to the accepted consequence?
**Matches the design letter exactly; one proximity cluster, no redesign warranted.** Check 5 is `if (line.includes("|"))` - bans any pipe, no code-span stripping. Check 6 strips inline code spans (`line.replace(/`[^`]*`/g, "")`) before `/<\/?[a-zA-Z]/`. Fire-verified both arms: a pipe **inside** a code span (`` `(mkv|mka)` ``) still goes RED (check 5); angle brackets **inside** a code span (`` `<span>` ``) stay GREEN (check 6 exemption); the same HTML **outside** a code span goes RED. Zero pipes tree-wide today (grep confirmed).

**Proximity report:** the regex / match-algebra topics sit closest to the accepted consequence. `editor-track-rule-match-expr.md`, `editor-locator-match-pattern.md` and `editor-match-expr-exact.md` discuss alternation/OR and codec-kind alternatives where a pipe would be the natural code-span notation (`(mkv|mka)`, `(srt|ass|pgs)`, `(?i)(sdh|forced)`); the authors routed around the ban by spelling out the prose words **"OR"** and **"alternation"** and using comma-separated separate code spans instead of a single alternation span. Green today; a future regex-alternation *example* placed in a code span would trip check 5 and require rephrasing, which is precisely the design's stated accepted consequence ("rephrased, not exempted"). No change recommended.

---

## Gate-run summary (own build, foreground)

| Gate | Result |
|---|---|
| `node scripts/check-i18n.mjs` | exit 0 - `... 22 help id(s) x 2 help locale(s) ...` |
| independent id recompute (probe) | constrained = exactly 22; brief-unconstrained = 24 (extras `${id}`, `spec.helpId`) |
| `pnpm lint` | exit 0 |
| `pnpm build` (`vue-tsc --noEmit && vite build`) | exit 0, 165 modules |
| `pnpm test:e2e` (tsc + 2 vite builds + playwright) | **52 passed**, exit 0 |
| help tree | 22 topics x {en,de}; zero pipes; every `<` is a `(?<...>)` span in an inline code span; no fenced blocks |

**Fire-verification (each RED reproduced, restored byte-identically, `git status` clean after):**

| Check | Break | RED line observed |
|---|---|---|
| 1 referenced->file | removed `help/en/view-batch.md` | `help id "view-batch" (referenced at src/views/BatchView.vue:334) has no help/en/view-batch.md` |
| 2 orphan | created `help/en/zzz-orphan.md` | `help/en/zzz-orphan.md: orphan topic ...` |
| 3 lockstep | `mkdir help/fr` + file | `help/ locales [de,en,fr] != locales/ [de,en] (lockstep, D62)` |
| 4 external-URL | `https://example.com` | `... contains an external URL (banned, D62 check 4)` |
| 5 pipe (outside span) | `col a \| col b` | `...:N: contains a table/pipe character (banned, D62 check 5)` |
| 5 pipe (inside span) - asymmetry | `` `(mkv\|mka)` `` | still RED - check 5 bans pipes in code spans |
| 6 raw-HTML (outside span) | `<div>raw</div>` | `... contains raw HTML (banned, D62 check 6; inline code spans exempt)` |
| 6 raw-HTML (inside span) - exemption | `` `<span>` `` | GREEN - exemption holds |
| L1 blind spot | `helpId: "Editor_Bad"` | GREEN (silently skipped); `helpId: "editor-bogus-xyz"` -> RED |

All scratch edits were tracked-file restores via `git checkout --` or created-file deletion; both worktree and main tree left clean apart from this verdict file.

---

## HARVEST

- **Dominant pattern / ledger candidate.** Literal-scan gate regexes for kebab ids must constrain the capture to the id grammar (`[a-z][a-z0-9-]*`), never `[^'"]+` / `[^"]+`: a naive `data-help-id="..."` scan over `src/` inevitably over-captures dynamic Vue binds (`:data-help-id="spec.helpId"`) and querySelector template literals (`[data-help-id="${id}"]`), producing permanent-RED on legitimate content. This is the same brief-vs-real-tree defect class the plan hit earlier; grammar-constraining the capture is the clean, house-idiomatic (D45 `LABEL_KEY_RE` precedent) fix.
- **Over-restriction watch (2 items, both benign today).** (a) The grammar constraint silently skips malformed non-kebab id literals - a CI blind spot bounded by a visible runtime raw-id fallback; worth a ledger note if a stricter "every `helpId:`/`data-help-id` literal is either a valid id or a known dynamic shape" guarantee is ever wanted. (b) `RAW_HTML_RE`'s inline-span-only strip does not exempt fenced code blocks and can false-positive `a<b` prose - fine under the design's inline-only letter, revisit only if a topic needs a fenced HTML example.
- **Repeated rejection reaffirmed (implementation conforms).** One i18n gate, one CI step (D62 rejected `check-help.mjs`); literal-scan house pattern; content-hygiene safeguards not argued back out (`proc-proposed-safeguard-stays`). The header comment correctly widens its scope and its "Five hard-failure checks" count is right at the hard-fail-array granularity (missing / parity / tooltip / ipc / help), with the inner "six conditions feed helpErrors" documented at the block.
- **Ledger candidate (content rule).** The check-5/check-6 code-span asymmetry (pipes banned everywhere incl. inline spans; raw HTML exempt in inline spans) is a deliberate, now-documented markdown-subset content rule whose accepted consequence is that regex-alternation topics rephrase (prose "OR"/"alternation", comma-separated code spans) rather than write `(a|b)` in a code span. Current authors already comply; the rule and its consequence are worth pinning so a future topic author does not fight the gate.
