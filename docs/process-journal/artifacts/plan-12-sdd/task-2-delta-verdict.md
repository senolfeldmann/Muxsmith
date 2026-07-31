# Task 2 delta verdict (Plan 12): fix rounds 1 and 2

**Reviewer:** the author of `task-2-verdict.md`. My own standards apply, and
settled non-findings are not reopened. Read-only on the repo except this file.
Scope: the report-side corrections of round 1, and commit `e778dda` (round 2,
committed source). Tree at review time: `10c819c`, clean, 72 e2e cases green.

| Finding | Verdict |
|---|---|
| **I1** (the split's recorded reason was inverted) | **ADDRESSED** |
| **M1** (mutation A's discrimination overstated) | **ADDRESSED**, and the widening is correct |
| **The source-comment correction** (`e778dda`) | **ADDRESSED** |

No new breakage found. One Minor observation, no action owed.

---

## 1. I1 - ADDRESSED

Corrections 1 and 3 in `task-2-report.md` state the corrected consequence, leave
each wrong sentence standing beside it, and carry the run that produced the
correction in section 9. The correction is right, and I did not take it on the
report's word: I re-measured the merged single-page form against the tree the
comment now ships on (`e778dda`), with a fresh probe under a gitignored path,
nothing inherited from either round's instruments:

```
RVD-MERGED {"set_settings assertion (null persisted)":true,"persisted":["en",null],"German heading comes back":false,"html lang":"en"}
```

That is the same result my own pre-fix probe produced at `ea39d88`, and the same
one section 9.3 reports, value for value. The prescribed merged form is an
always-red case: its persisted assertion holds, both rendering assertions do
not. Never a false green.

Correction 1's further conclusion - "**this was never an instance of the
assertion-below-a-fallback class**" - is also right, and it matters more than
the sentence it replaces. That class is a mutation upstream of a fallback never
reaching the assertion, so the test passes in the state it forbids. What
actually happened is a fixture that cannot reach the state the assertion needs.
Different shape, different lesson.

**Correction 3's third bullet is a catch I did not make**, and it is correct:
"the split is not a workaround for a fixture limitation - it is what restored
the assertion's power" is backwards. The split *is* a re-cut around a fixture
limitation, and that is its sufficient justification. Recorded because a fix
round finding something the reviewer missed is the mechanism working, not a
defect in it.

## 2. M1 - ADDRESSED, and the widening is CORRECT: two cases, not one

**My verdict named one case; the right number is two.** I adopt the widening,
and I measured it independently rather than accepting it.

Method: my own probe file, my own path, no fixture or script from either round
executed. It replicates the two locator-death cases with the single change of an
`en()` Save locator, so the locator death is removed and only the shipped
assertions remain. It carries its own control asserting the *mutated* state
directly, because two greens are exactly what a run against a stale bundle would
also produce.

On the **unmutated** tree, all three fail - the control at `lang=de
deHeadingCount=1` (the app follows the system language, so the mutant is not
present), the two case-probes at the `en("settings-save")` locator, because the
interface is German. That is the instrument's fired negative.

Under **mutation A** (`effectiveLocale` returns `saved ?? "en"`), rebuilt:

```
RVD-CONTROL lang=en deHeadingCount=0
  ✓  CONTROL: the mutant is live (system-locale following is broken) (113ms)
RVD-CASE2 all shipped assertions PASSED under mutation A
  ✓  case 'saving without touching the language': ASSERTIONS ONLY (193ms)
RVD-CASE3 all shipped assertions PASSED under mutation A
  ✓  case 'leaving the system language': ASSERTIONS ONLY (200ms)
  3 passed (559ms)
```

Both cases' full assertion sets pass with the mutant demonstrably live in the
bundle. *Leaving the system language* asserts a persisted `"en"`, the English
heading present, the German heading absent and `lang === "en"` - every one of
which holds on an app that was English from the start, which is precisely what
mutation A produces. So it has no assertion that can tell a correct seam from a
broken one, exactly as round 1 measured.

**Mutation A's reach is two of the four new cases** (*first run* and *returning
to the system language*), not three as my verdict had it and not four as the
report's original table read. My verdict's M1 stands as written rather than
being edited into agreement; this section is its correction, in the same style
the report used for its own.

The rest of M1's substance is undisturbed: the two seam-carrying cases are
killed genuinely below the `[requested, en]` fallback, and the D56 case is
spared correctly.

## 3. The source-comment correction (`e778dda`) - ADDRESSED

### 3.1 Is the new comment TRUE?

Yes, on both checkable claims, each verified against its own source rather than
against the report.

**The fixture claim** - stateless mock, `set_settings` does not feed
`get_settings`, `open()` re-reads the baseline, so a second save inside one page
compares `null` against `null` and the guard stays false. Verified at the source
in my original review and unchanged since: `mocks.ts`'s `nextResult` returns
`q.length > 1 ? q.shift() : q[0]`, so a single-entry queue repeats forever;
`SettingsDialog.vue`'s `open()` reassigns `baseline = await getSettings()`;
`save()` guards on the raw nullable comparison and reassigns `baseline` only
afterwards.

**The red/green claim** - "its `set_settings` assertion still passes (`null` is
persisted), while both rendering assertions fail - the German heading never
comes back and `<html lang>` stays `en`". Measured on `e778dda` in section 1
above: persisted `["en",null]`, German heading back `false`, `html lang` `en`.
All three clauses hold, including the asymmetry between the persisted half and
the rendering half.

**Comment hygiene.** The diff is comment-only - verified with a filter that
strips comment lines and prints anything else, fired first against a synthetic
code line to prove it would have shown one. No AI-tell glyph in the added lines
(scan fired against a synthetic em-dash). No line-number citation. The
neighbouring pointer, "see EN_OVERRIDE_SETTINGS for why one page cannot carry
both", stays true after the rewrite. The citation of ruling 1 stays useful,
since the ruling now carries its own marked correction.

**Minor observation, no action owed.** The clause "cannot be made green inside
one page without a reload or a second fixture state" enumerates an escape set,
and the route the ruling itself rejected - making `e2e/mocks.ts` stateful - is
not named in it. It is not wrong: a stateful store does put the fixture into a
second state, and a counted multi-entry `get_settings` queue is a second fixture
state on existing infrastructure. Worth noting only because correction 1 in the
report words the same set differently ("a reload or new test infrastructure"),
and of the two the comment's wording is if anything the more accurate, since the
counted-queue route needs no new infrastructure. Named so the divergence is not
discovered later as a contradiction.

### 3.2 Is one site really the whole sweep?

**Yes, for the tracked surface.** I did not re-run round 2's patterns as my
check - a sweep re-run with its own patterns agrees by construction. I swept on
two axes chosen to be independent of it.

**Axis 1, locality.** Any restatement of this claim has to sit near its subject.
`git grep -nEi "EN_OVERRIDE|merged case|merged form|single-case form|split the
case|case 3|D106" -- .` returns 93 hits across the tracked tree; I read every
one that could plausibly carry the reason. The tracked artifacts that discuss
this split at all are exactly three: `e2e/locale-switch.spec.ts` (corrected), the
plan document (which predates the split and which ruling 1 deliberately leaves
unedited), and `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` (D106's
ADR, authored before the split, containing no split content). Neither of the
latter two carries the false consequence.

**Axis 2, a disjoint vocabulary.** Round 2's four patterns and mine share only
"measuring nothing". Mine:
`shipped green|false green|vacuous|green while|pass(es|ed)? while|passing while|without measuring|prov(es|ing) nothing|measur(ing|es|ed) nothing|measure nothing|would be green|reads green`.
Scoped to `e2e/ src/ docs/superpowers/ docs/*.yaml docs/*.md *.md`, then filtered
to lines that also carry this episode's subject, it returns five rows: three
about unrelated subjects (plan-7's `applyLocale` remount, plan-9's `--locale`
repeat, plan-12's shell-locale sync non-vacuity) and one that is the corrected
comment's own negation. No additional site. **The filter was fired against the
pre-fix wording** (`e2e/locale-switch.spec.ts:167: * measuring nothing on the
live path`), which it matches - so its empty result on the rest is an absence,
not a malformed instrument.

**The surface itself, walked rather than assumed:**

- *Tracked, non-ignored:* covered by both axes above.
- *Untracked, non-ignored:* none. `git status --porcelain` was empty at every
  point of this review, so `git grep`'s tracked-only reach is total reach here -
  which is round 2's own argument, and it holds.
- *Ignored (`.superpowers/`):* swept in round 1 with `command grep`, which is the
  right instrument; the wrapper's `.gitignore`-honouring behaviour is now a
  Tier-2 entry (`2235d7e`). I verified the disposition of each site it found:
  the ruling carries a marked correction, `progress.md:171` now carries one too,
  and the verdict's three occurrences are the finding quoting the claim, which
  is correct usage.
- *Generated/build output:* checked explicitly, since a stale copy hiding in an
  ignored artifact is the one place neither `git grep` nor a source read looks.
  Neither `dist/` nor `e2e/.generated/` contains the string or the spec; the
  spec file is never bundled.
- *Commits landing after the sweep:* `2235d7e` and `10c819c` both touch ledger
  YAML only, and neither restates the claim (checked; the grep was fired against
  a token those commits do contain, so its empty result is real).

**Could the claim be phrased in words none of round 2's four patterns carry?**
In principle yes - "shipped green", "false green" and "vacuous" are all absent
from their set, and all three are natural phrasings. That is why I swept on
them. They return nothing in the tracked tree. The sweep's surface is whole.

### 3.3 The gate

Re-run by me on the current tree with the exit status captured directly rather
than through the broken array, per the disclosure:

| Part | Exit | Output |
|---|---|---|
| `pnpm lint` | 0 | silent |
| `pnpm check:i18n` | 0 | byte-identical to the report's section 10.4 line |
| `python3 scripts/ledger-lint.py` | 0 | `568 entries across 4 files ... all invariants hold` |
| `cargo fmt --all --check` | 0 | silent |
| `pnpm build` + `npx playwright test` | 0 | `72 passed` |

The `568` figure still matches section 10.4 even though two ledger commits
landed after `e778dda`, because both appended occurrences to existing entries
rather than adding entries - checked, so the unchanged count is not a stale
paste.

The zsh disclosure reproduces: `zsh -c 'true | true; ...'` prints
`PIPESTATUS[0]=[] pipestatus[1]=[0]`. Every exit reading in this delta review
was taken with `$?` directly.

**Not re-run:** `cargo clippy` (both targets), `cargo test --workspace`,
`cargo doc`, `cargo deny`. The commit is a comment inside a TypeScript test
file; it touches no Rust and cannot move them. Named rather than implied.

**Commit metadata:** `e778dda`, unsigned (`%G?` = `N`), exactly one
`Co-Authored-By` trailer, one file, 7 insertions and 2 deletions, comment only.
The subject line - "the split's reason is an always-red case, not a vacuous
green one" - is itself accurate.

## 4. New breakage from either round

**None found.** What I checked, and what came back:

- **A corrected sentence now wrong in a different way.** Corrections 1, 2 and 3
  were read against the measurements they cite. All three hold. Correction 3's
  three bullets are each right, including the one I had missed.
- **A control that does not discriminate.** Round 1's controls were re-derived
  rather than trusted. Its restoration evidence is unusually strong and I
  reproduced two independent parts of it: `sha256sum src/i18n/index.ts` gives
  `d7c2ab8e0e52d4578a3120a68798aef0c63485ad6887c5c27c9508a7e7bdc649`, the same
  hash section 9.5 pastes, and my own build produced `index-CjJCKxvO.js`
  restored against `index-vFA2Btem.js` while mutated - the same two bundle names
  sections 9.2, 9.4 and 9.5 use as their restoration witness. Independent
  corroboration that those runs happened as described.
- **A pasted output that does not match the command above it.** Everything I
  could reproduce, reproduces: `4 failed, 68 passed` under mutation A; `72
  passed` at baseline; the four death sites and their two kinds; round 2's own
  four patterns returning exactly what section 10.1 says they return, including
  the fourth returning only unrelated matter (`broken_override` Rust test names
  and "opposite direction" prose).
- **The line numbers in section 10.4.** The five cases of the edited file sit at
  104, 195, 221, 249 and 275 after the comment grew by five lines - measured,
  matches.

## 5. Standing residuals, unchanged by this delta

- My verdict's **M2** (the soundness control silently constrains the seam's doc
  comment) and **M3** (the `test.use` isolation claim is true but unwitnessed)
  are untouched by either round and remain as recorded. Neither was in scope.
- `progress.md`'s NEEDS_CONTEXT entry keeps the original wrong sentence with a
  marked correction beside it. That is the controller's disposition of section
  9.7 item 2 and it is the right one for an append-only log; noted so it is not
  re-found as an open site.
- The harvest candidate from my verdict stands and is now better evidenced: **a
  negative claim about a test's failure mode is a measurement, and it inverts
  easily.** This episode has the inversion propagating from a ruling into a
  report and from there into committed source, and being caught only by running
  the prescribed form. The trigger is readable - you are about to write that a
  test *would have passed* while measuring nothing, about a form you did not
  execute.
