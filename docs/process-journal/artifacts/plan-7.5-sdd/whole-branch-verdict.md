# Whole-branch verdict: Plan 7.5 (track-rule add/remove), range 1d82179..HEAD path-scoped

Reviewer: independent whole-branch reviewer, top tier. Read-only except this
file, one transient fire-verification mutation (restored, proven below), and
scratchpad artifacts. No git writes, no session-relocation tool, every run
foreground on master (`/home/senol/Git/Muxsmith`).

## VERDICT: READY

Nothing blocks the plan close. The four merged tasks form a coherent whole,
the spec/design/tree three-way agrees at every seam checked, the D62/D55
invariants hold on the union with full-domain evidence, the shipped suite is
healthy (full frontend gate re-run green here, 62 passed), and every deferred
item triages to closed, harvested, or the pre-registered owner pass - none to
a pre-close fix. The close actions still owed are enumerated at the end.

---

## 0. Scoping verification (done first, as dispatched)

- `git log 1d82179..HEAD --name-only`, filtered to non-docs/non-.worktrees:
  the 7.5 pathset is exactly the package's seven files; plan-8's remainder
  (`src-tauri/tauri.conf.json`, `src-tauri/tauri.bundle.conf.json`,
  `scripts/check-version-sync.sh`, `.gitignore`, `BUILDING.md`, plan-8 docs)
  is disjoint as announced.
- **One in-range product file outside both sets:**
  `crates/muxsmith-core/tests/joblog.rs` (`c06b8dd`), a controller-side
  gate-red fix (latent calendar-bomb fixture, red on the pre-merge base,
  ledgered as `test-fixture-dates-outside-retention-windows`,
  `docs/decision-ledger.yaml:4237`). Not a 7.5 task file, but it landed inside
  the 7.5 execution window with no task-level reviewer, so I reviewed its full
  diff here: fixture run-id now derives from `make_run_id(SystemTime::now())`,
  behavior under test unchanged, the two deliberate aging tests correctly keep
  absolute stamps. `cargo test -p muxsmith-core --test joblog` re-run: 12
  passed. Sound; no finding.
- **The diff package is faithful:** I regenerated
  `git diff 1d82179..HEAD -- <the seven paths>` independently;
  byte-identical to `whole-branch-75.diff`.
- **Both merges are clean carries:** first-parent diffs of `e36885f` and
  `33be397` name exactly their stream files; no conflict-resolution delta.

## 1. Cross-task integration (the whole the task reviews could not see)

The four parts assert one consistent contract everywhere they overlap:

- **Skeleton/warning story** - T1's `addRule` appends `{ match: {} }`; case 6
  pins warning severity at `tracks[1].match` with Save enabled; T3's topic
  sentence claims "a warning ... until you fill in its match expression";
  T4's 5.2 row says `warning`; core (`validate.rs:79-92`) emits exactly that,
  suppression clause included. Four artifacts, one truth.
- **Remove story** - `:disabled="selectedIndex === null"` (code) = "stays
  unavailable until a row is selected" (topic) = "deletes the selected rule
  without confirmation" (spec 8.2) = D66.
- **Zero-rule story** - cases 5/7/8 (no floor, drop-error/keep-info) = topic
  "Removing the last rule is allowed" = spec "legal down to zero rules per
  4.5" = D69/core-83.
- **Witness story** - the amended design's case-9 block, the landed
  `probeEnterKeydown` helper and comments, and D71's widened keydown bullet
  agree; the T2 re-review proved the helper byte-identical to the design's
  fenced block.
- **Design section coverage against the landed union:** every row of the
  plan's coverage map is discharged - D65-D72 landed as mapped, section 4 =
  T4 (both amendments, exact equality re-confirmed by grep at HEAD: one
  `EmptyMatchExpression` row, the 8.2 sentence present verbatim), section 5 =
  9 cases in 2 files (8 + 1 counted in the gate output; suite 53 -> 62),
  trigger 1 consumed, trigger 2 resolved by amendment 2 landing.

## 2. Spec / design / tree three-way seams

Checked at HEAD, not borrowed: spec 8.2 sentence == design amendment-1 text;
spec 5.2 row == design amendment-2 text == `validate.rs` severity + guard ==
`match_expr.rs::is_empty`'s five keys; the amendment's discrimination premise
"`onHelpKeydown` is the ONLY keydown listener in `src/`" re-grepped true
(the register/unregister pair at `src/App.vue:105`/`:111`, capture phase).
No contradiction found anywhere spec, design, and tree overlap.

## 3. D62/D55 invariants on the union - by iteration domain

- `git diff 1d82179..HEAD --name-status -- locales/`: empty (whole tree, not
  named files). `-- help/`: exactly `M` on the two topic files, no A/D.
- `data-help-id` grep over all of `src/` at base and at HEAD: identical
  seven-member set.
- `editor-action-add`/`-remove` exist at base (`1d82179:locales/en/
  gui-editor.ftl:138-139`) - the buttons consume, not create. Id counts
  46/46 at HEAD.
- `check-i18n.mjs` enumerations are filesystem-derived (`readdirSync` over
  `HELP_ROOT`, `LOCALES_ROOT`, `SRC` recursive), so its unchanged "211
  catalog ids, 22 help id(s) x 2 help locale(s)" is full-domain evidence,
  not a spot output. 22 files per help locale confirmed on disk.

## 4. Suite health (gate re-run, foreground, this session)

`pnpm lint && pnpm build && pnpm check:i18n && pnpm test:e2e` -> exit 0,
**62 passed (2.7s)**, all nine 7.5 cases green by name in the output.

**Witness probe fire-verified at the shipped file** (closes the T2
re-review's deferred item 4, which had been accepted on argument): flipping
only the probe's `cancelable: true` to `false` (line 123) turns case 9 red at
the inside-phase witness - `e2e/help-mode.spec.ts:305`, `Expected: true /
Received: false` - exactly the loud failure the amendment's mutual-control
argument predicted. Restored via `command cp` from a pre-mutation backup;
md5 identical (`82e584f98ada33fc782291104e7c9a6d`), `git status --porcelain`
empty, `git diff --exit-code` 0.

## 5. House conformance of the union (Tier-2 sweep)

- `help-topic-h1-scheme`: h1s untouched by the plan and conformant
  (`Rules (tracks)` / `Regeln (Spuren)` = byte-equal catalog label +
  containing-section parenthetical); the one prose cross-reference added
  (`see the Editor topic`) uses a unique label leading bare, per the scheme.
- `editor-generic-action-keys`: statement records the third render site,
  occurrence 2026-07-27 (trigger 1 consumed, `product-boundaries.yaml:404`).
- `code-comment-line-citations-drift`: no `file:line` in any added product
  line (union grep, positive control fired); the T3-harvested second trigger
  is in the statement.
- `e2e-filter-invokes-playwright-directly`: tier 2 at
  `docs/conventions.yaml:1050` - T2's L3 label discrepancy is reconciled.
- `redundant-layers-need-mechanism-witness` resolves at HEAD
  (`decision-ledger.yaml:4251`) - the T2 re-review's dangling-anchor note
  self-healed at merge as predicted.
- Typography: no AI-tell glyph in any added union line (class grep with
  positive control); de topic carries proper umlauts/ß orthography.
- Commit hygiene spot-checked by the task reviewers per commit; nothing in
  the union contradicts them.

## 6. Findings by severity

**Critical: none. Major: none. Minor: none.**

Informational:

- **I-1 - the scoping instruction under-specified the remainder.** The
  dispatch named plan-8's files as the expected disjoint remainder;
  `c06b8dd` (joblog fixture fix) was in-range, product-code, and in neither
  set. Resolved here by direct review (section 0). Process note only; see
  HARVEST.
- **I-2 - a second intention-as-fact ledger line, already class-named.**
  `progress.md` line 10 recorded "pnpm-grep pattern promoted tier-2" while
  the tree still said tier 1; the promotion landed later (`a61fead`). Same
  class as `proc-ledger-records-facts-not-intentions` (count 1, plan-8
  occurrence). Candidate second occurrence - controller's call, no product
  effect, label since reconciled.

## 7. Triage: deferred minors and observations, disposition per item

| # | Item (source) | Disposition |
|---|---|---|
| 1 | T3 LOW-1: pronoun antecedent in the Remove sentence, both locales | **Deferred, owner pass** (pre-registered close action covers exactly these files). Fix candidates already drafted in the T3 verdict. Semantically self-correcting, not blocking. |
| 2 | T3 LOW-2: "A warning flags that new rule" vs the marker landing in the detail panel, not the row (routed to this triage) | **Deferred, owner pass - not a pre-close fix.** The sentence asserts no location and D71's licensed claim ("announced by a warning") is true: the panel auto-opens on Add, so the marker is visible at the moment described, plus the never-filtered panel row. The ambiguity is inherited verbatim from the design's own enumeration; a keyboard fix now would bypass the plan's mandated wording authority. Put the panel-naming candidate (stays inside D65) before the owner. |
| 3 | T3 Q1: de cross-reference normalization, three sites tree-wide | **Deferred, owner-pass agenda item** (touches files outside the plan set; a single-site flip would manufacture inconsistency). |
| 4 | T3 LOW-3: report quote misattributed to D41 | **Closed via harvest**: the second trigger is in `code-comment-line-citations-drift`'s statement. Report artifact, no product effect. |
| 5 | T3 Q2 residual: sentences inherit any stream-A divergence | **Resolved by this review**: the three-way check (section 1) found no divergence between landed code and the claims. |
| 6 | T2 L1: brief carried the defeated filter form | **Closed**: recurrence guard landed (tier-2 promotion, `conventions.yaml:1050`); plan artifact only. |
| 7 | T2 L2: fire-verification scoped to the filtered case | **Closed as harvest** (ledgered `1af9540`); the lesson was applied in the fix-round re-review. |
| 8 | T2 L3: progress.md tier label vs tree | **Closed**: reconciled at HEAD (tier 2). Residue noted as I-2. |
| 9 | T2 re-review 3: ruling id dangling on the branch | **Closed**: resolves at HEAD (`decision-ledger.yaml:4251`). |
| 10 | T2 re-review 4: `cancelable: true` not flipped | **Closed empirically this review** (section 4 fire-test). |
| 11 | T2 nit: positional "below" cross-reference | Stays as landed (nit; target verified still below). |
| 12 | T1 I1: brief line-ref drift (4110 vs 4111) | **Closed**: plan artifact, no consumer. |
| 13 | T1 I2: trigger 1 due | **Closed**: consumed at `341c411`, verified at the entry. |
| 14 | T1 I4: third local `name()` helper copy | Stays recorded as a future shared-helper candidate; deliberately not a finding (house pattern). |
| 15 | T4 O1/O2: spec 8.2 compressions ("per 4.5" zero-rule legality; "invalid until filled" vs warning severity) | **Deferred, owner call at close** - carry alongside the owner pass as the T4 HARVEST's one-clause candidates. Byte-faithful transcription was mandated; both compressions carry their resolving citations; the design flagged O2 itself. Not v1.x-routed: they are spec-wording, actionable any time the owner wants. |

## 8. Close actions still owed (controller, after this verdict)

1. **Owner rendered-surface pass** over the two topic files (pre-registered),
   with agenda items 1, 2, 3 from the table and the O1/O2 spec-wording
   candidates (item 15) put before the owner in the same sitting.
2. **Salvage re-pointing**: the design cites
   `.superpowers/sdd/plan-7.5/design-review-round-1.md` (amendment-2 scoping);
   re-point in the same change as the salvage (ROADMAP trigger already
   registered).
3. **Triggers 3-6 mirrored into the ROADMAP** as standing triggers - not yet
   present in the Triggers section (checked); trigger 5's route target (the
   v1.x undo/redo entry) exists.

## HARVEST

- **A path-scoped whole-branch package needs a remainder manifest.** When two
  plans interleave, the dispatch should enumerate ALL in-range non-docs
  product paths and attribute each (this plan / other plan / neither), so the
  reviewer's scoping check is set subtraction instead of archaeology. The
  `c06b8dd` gap was benign but cost the attribution work, and a controller-
  side fix commit is exactly the commit with no task reviewer by
  construction - name its review owner (whole-branch by default) in
  progress.md when it lands.
- **Sweep task verdicts' "verification limits" at whole-branch.** Two of
  them (branch-lagged ledger id, unflipped `cancelable`) had become checkable
  after merge; one grep and one two-minute fire-test converted them from
  accepted-on-argument to measured. Cheap, and it is exactly the residue the
  task tier could not reach.
- **The interleaved-plans model held.** Disjoint pathsets, per-merge full
  gates, and path-scoped whole-branch review composed without a single
  cross-plan conflict; the one shared surface (docs/house files) was
  append-only and conflict-free. Precedent worth citing when the next
  parallel-plan cut is argued.
- **Candidate occurrence** for `proc-ledger-records-facts-not-intentions`
  (I-2): progress.md line 10's tier-2 label preceded the promotion commit.
  Controller's call; count would go 1 -> 2.
