# Task 17 independent review verdict - check-i18n D55 extensions (rules 1-5)

**Reviewer:** independent (fresh eyes), Opus 4.8.
**Under review:** commit `a838a32` on branch `plan7-g`, single file `scripts/check-i18n.mjs` (+197/-20).
**Ground truth:** brief for rules 1-4 + 5(a); the **round-7 amended** D55 block
(`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:679-858`) as the sole
authority for rule 5's select-structure part.

## Combined verdict: APPROVED

- **Spec compliance: PASS.** Rules 1-4 + 5(a) match the brief exactly; rule 5's
  select-structure part matches the amended en-vs-de parity semantics exactly; no
  absolute per-select assertion survives; line-based charter respected; rules 4-5 run
  over all `.ftl` including `cli.ftl`; the reported residual blind spot matches the
  amendment neither wider nor narrower.
- **Quality: PASS.** All four JS/frontend gates green under my own build; one RED
  fire-verification per rule class reproduced independently and restored byte-identically;
  the three previously-false-RED messages confirmed GREEN and, via a positive control,
  confirmed **non-vacuously** green (the gate genuinely compares them).

One **Minor** house/charter finding (non-blocking), below.

---

## Findings by severity

### Blocking
None.

### Important
None.

### Minor

**M1 (house / charter accuracy - understated coverage).** The header opening
(`:3`) still reads *"Three independent checks over the same catalog/source scan"*
and the numbered list enumerates only checks 1/2/3. But rule 3 (editor tooltip
completeness) is now a **fourth independent hard-failure check**: it has its own
top-level error block (`check-i18n: editor labelKeys without a .tooltip attribute
in the en catalog`, `:470`), its own accumulator (`tooltipErrors`), and its own
exit-gate contribution (`missing.length === 0 && parityErrors.length === 0 &&
tooltipErrors.length === 0`, `:483`) - it is not a sub-part of check 1 (id
resolution), check 2 (unused warning), or check 3 (cross-locale parity; rule 3 is
en-only completeness, orthogonal to parity). Rule 2 was correctly folded into
check 1's description and rules 4-5 into check 3's, because those *are* extensions
of those scans; rule 3 is the one new check-class that the opening enumeration does
not name. The task's brief (Step 5) directed updating only the ATTRIBUTES bullet
and check-3's description, which the implementer did faithfully; the "Three" count
was outside that literal scope. But the recorded house convention
`proc-normative-count-recomputed` (trigger 2: *adding a member to an enumerated set
-> grep every numeral/count-word describing that set*) makes the opening count a
sweep target the moment a fourth check-class lands. **Fix:** reword the opening to
"Four independent checks" (or drop the count) and add rule 3 as an enumerated item.
Non-blocking: the check is correct and fully verified; this is documentation
accuracy only.

*(Note for calibration: this is not an overstatement anywhere - the rule-5 header
and the `patternStructure` inline comment are candid about the flat model's limits
and correctly scope absolute validity to the e2e parse guard. The only defect is the
stale check-count in the opening line.)*

---

## Spec-compliance detail

### Rules 1-4 + 5(a) against the brief (binding as written)

| rule | requirement | implementation | status |
|---|---|---|---|
| 1 | `parseCatalogIds` -> `parseCatalog`: `Map<id,{value, attrs:Map<name,body>}>`; wrapper kept; terms unregistered | `:140-172`, byte-for-byte the brief's Step-1 code; value/attr body folding present | PASS |
| 2 | `TA_CALL_RE = /(?<![\w$])\$?ta\(\s*(['"])([^'"]*)\1/g` beside `CALL_RE`; same add-to-`literalCallIds` / push-to-`missing` | `:214`, `:243-249`; member access after the call left dynamic | PASS |
| 3 | every `LABEL_KEY_RE` id carries a `.tooltip` in en - hard fail naming the id | `:227` (`labelKeyIds`), `:316-328`; `.some()` over all en catalogs | PASS |
| 4 | attribute-NAME set == en's - hard fail on missing AND extra | `:442-448` sorted-array join inequality catches both directions | PASS |
| 5(a) | `$name` placeable set equal, variants included | `:332` `PLACEABLE_RE` over the whole body (incl. variant lines), `:367-369` | PASS |
| scope | rules 4-5 run wherever check 3 runs (ALL `.ftl` incl. `cli.ftl`) | `referenceCatalogFiles = listCatalogFiles(LOCALES_EN)` (all `.ftl`), wired into the locale loop; empirically confirmed via `cli.ftl`'s `validate-summary` | PASS |
| charter | line-based, no Fluent parser, no new dependency | `parseCatalog`/`patternStructure` are line-split + regex; imports still only `node:fs`/`node:path` | PASS |

### Rule 5 select-structure against the AMENDED design (sole authority)

- **(b) select-count parity** (`a.selects.length !== b.selects.length`, `:370`) - en-reference, early-returns on mismatch. Matches amendment (b).
- **(c) selector-variable parity** (`sa.selector !== sb.selector`, `:376`). Matches amendment (c).
- **(d) variant-key parity with CLDR/numeric carve-out** (`:379-400`): non-carve-out keys get full equality (`:398`); carve-out (all keys plural-category or `/^\d+$/`) is the **parity** form `(sa.keys.length === 0) !== (sb.keys.length === 0) || sa.defaults !== sb.defaults` (`:392-395`). Matches amendment (d) exactly.
- **No absolute per-select assertion survives.** Grep confirms: no `defaults !== 1`, no `needs >=1 variant`, no `exactly one *-default` *as an assertion* (the only textual match is `:386`, the comment explaining that absolute validity is **delegated** to `assertAllCatalogsParseCleanly` - correct, not a leak).
- **Refuted premise removed.** The brief's `patternStructure` comment "this tree's catalogs have no nested selects" is gone; `:338-348` replaces it with an accurate flat-derivation + parity rationale naming the three real counterexamples.
- **Residual blind spot matches the amendment.** Report completion (`:324-330`): "a variant moved between two sibling selects with total keys and `*`-default count unchanged". Amendment (`:807-809`): "a variant moved between sibling selects with totals unchanged." Same set, neither wider nor narrower.

---

## Quality: gate runs (my own build, foreground)

| gate | command | result |
|---|---|---|
| check:i18n | `node scripts/check-i18n.mjs` | **exit 0** - `ok (41 source files, 211 catalog ids, 17 unused warning(s), 1 other locale checked against 7 en catalogs)` |
| lint | `pnpm lint` (eslint) | **exit 0** |
| build | `pnpm build` (vue-tsc --noEmit && vite build) | **exit 0** - built `dist/` (322.80 kB) |
| e2e | `pnpm test:e2e` (tsc + 2 harness builds + playwright) | **exit 0, 52 passed** - incl. `catalogs.spec.ts`, `editor-tooltips.spec.ts`, the diagnostics/suggestions pluralization test |

The report's four-part completion table is fully corroborated.

### Fire-verification: one RED per rule class (independently reproduced)

Method: synthetic drift on the real tree, gate run, `git checkout --` restore,
**sha256 re-verified against the pre-break baseline** (RESTORE-OK for every file).
Final `git status --short` clean, gate green (exit 0).

| rule class | break | captured RED (verbatim) |
|---|---|---|
| 2 (`$ta` id) | `$ta("zzz-no-such-id")` into `src/i18n/index.ts` | `src/i18n/index.ts:96  "zzz-no-such-id"` (exit 1) |
| 3 (tooltip completeness) | en `editor-profile-meta` `.tooltip`->`.zooltip` | `labelKey "editor-profile-meta" has no .tooltip attribute in the en catalog` (exit 1) |
| 4 (attr-set parity) | de `editor-profile-meta` `.tooltip`->`.zooltip` | `id "editor-profile-meta" attribute set differs from en ({tooltip} vs {zooltip})` (exit 1) |
| 5a (placeable set) | de `identify-failed` `{ $detail }`->`{ $detailx }` | `"identify-failed" value: placeable set differs from en ({detail} vs {detailx})` (exit 1) |
| 5b (select count) | de `jobs-row-warning-count` select -> plain string | `"jobs-row-warning-count" value: select-expression count differs from en (1 vs 0)` (exit 1) |
| 5c (selector var) | de `jobs-row-warning-count` `{ $count ->`->`{ $qty ->` | `"jobs-row-warning-count" value: select 0 selector differs ($count vs $qty)` (exit 1) |
| 5d (plural parity) | de `jobs-row-warning-count` `*[other]`->`[other]` | `select 0 plural variant presence / *-default count differs from en (2/1 vs 2/0)` (exit 1) |
| 5-else (non-plural keys) | de `invalid-template` `[unclosed-brace]`->`[unclosed-bracex]` | `select 0 variant keys differ from en ([unclosed-brace,empty-field] vs [unclosed-bracex,empty-field])` (exit 1) |

### Three previously-false-RED messages: GREEN, and non-vacuously so

On the real tree the gate emits **no** parity line for `validate-summary`,
`batch-diagnostics-summary`, or `suggestion-partition`, and exits 0.

To prove that GREEN is not vacuous (the messages are genuinely in-scope and
compared, not absent/skipped), I ran a **positive control** on `validate-summary`
(`cli.ftl`, the column-0 sibling-reopener shape):
- de `*[other]`->`[other]` on one arm -> RED `"validate-summary" value: select 0
  plural variant presence / *-default count differs from en (6/3 vs 6/2)`. This
  confirms the flat model derives en `validate-summary` as **one** select with 6
  keys / 3 defaults (exactly the report's mechanism analysis), and the amended
  carve-out compares de against it.
- de placeable rename -> RED `placeable set differs from en ({errors,warnings,infos}
  vs {errors,errorsx,warnings,infos})`.
- Restored byte-identically (sha256 match); gate green again.

So the three pass because en/de are structurally parallel under the flat model, not
because the check is inert - the round-7 amendment's central empirical claim holds.

---

## Numbered adjudications

### Q1 - gate scoping to the four JS/frontend parts: **SOUND**

The no-mechanism claim is **verified**, not weighed:
- `scripts/` is **not** a cargo workspace member. `Cargo.toml` members are
  `crates/muxsmith-core`, `crates/muxsmith-cli`, `crates/xtask`, `src-tauri`.
- The **only** `build.rs` is `src-tauri/build.rs`, which is stock
  `fn main() { tauri_build::build() }` - no glob of `scripts/`, no reference to
  the file.
- `scripts/` is in **no** tsconfig include, so `vue-tsc --noEmit` and `vite build`
  never typecheck or bundle `check-i18n.mjs` (and `pnpm build` ran green anyway).
- The only `check-i18n` references in Rust/TS are **prose comments**
  (`crates/muxsmith-cli/src/i18n.rs:254`) and a **separate** e2e file
  (`e2e/i18n-en.ts`) that *re-implements* `MESSAGE_ID_RE` in its own code - not an
  import of the script. Editing the script cannot change their behavior.
- Not a generated binding.

There is no structural path by which a `scripts/check-i18n.mjs` edit reaches Rust
compilation/lint/test/doc. Running the four JS/frontend-touching parts plus the
structural argument for the five cargo parts is the proportionate verification for
THIS diff. The full nine-part gate is the controller's merge responsibility
(`ci-06-per-commit-gate`); the deviation was surfaced and scoped correctly.

### Q2 - implementation follows the amendment, not the stale brief: **CONFIRMED**

At every point the amendment and the stale brief diverge, the code follows the
amendment, and nothing from the stale absolute semantics leaked into code, messages,
or the header:
- **Carve-out branch (code):** parity form `(sa.keys.length === 0) !== (sb.keys.length
  === 0) || sa.defaults !== sb.defaults` (`:392-395`), not the stale
  `sb.keys.length === 0 || sb.defaults !== 1`.
- **Error message:** `plural variant presence / *-default count differs from en`
  (`:396`), not the stale `needs >=1 variant and exactly one *-default` (grep: 0 hits).
- **`patternStructure` comment:** the refuted "no nested selects" premise is gone
  (grep: 0 hits), replaced by the flat-derivation/parity rationale.
- **Header:** check-3 block describes an "en-reference parity check ... absolute
  Fluent validity is delegated to e2e assertAllCatalogsParseCleanly" (`:63-69`); the
  stale "No Fluent attributes exist in any catalog today ... attribute-level parity
  is not needed yet" sentences were removed.
- Rules 5(b)/(c) were en-vs-de parity by construction in the brief already and are
  unchanged; no stale-absolute residue there either.

---

## HARVEST

- **Dominant pattern (positive):** the round-7 fork resolution (Option B: flat
  parser kept, rule 5 select-structure reduced to en-vs-de parity) is implemented
  completely and cleanly - the amendment's semantics, message, comment, header, and
  delegation note all landed consistently in one commit. The governing story
  (execution refuting a design premise, `proc-57-briefs-not-ground-truth` /
  `proc-check-green-state-reachable`) is already ledgered at their 2026-07-22
  occurrences.
- **Over-restriction watch:** none forced. No grant/latitude boundary blocked
  anything here; nothing to flag in the do-not-over-restrict direction.
- **Ledger candidate (minor):** M1 is a concrete instance of
  `proc-normative-count-recomputed` **trigger 2** (add a member to an enumerated set
  -> recompute the summarizing count) landing in a **source-file header comment**
  rather than a design/plan/spec doc. The recorded occurrences are all in normative
  docs; this shows the same trigger reaches code headers. Worth a one-line note that
  the count-recompute duty binds on the header charter of a gate script too.
- **Reusable method (review-side):** proving a GREEN check non-vacuous by injecting a
  real drift into the very message the amendment claims now passes (positive control
  on `validate-summary`) is the review-time complement of
  `proc-check-green-state-reachable` / `proc-verification-step-must-be-falsifiable` -
  a GREEN "these three now pass" claim is only evidence once you have watched the
  same messages go RED under a genuine drift.
