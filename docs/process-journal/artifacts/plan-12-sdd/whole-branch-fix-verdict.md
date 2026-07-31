# Plan 12 whole-branch fix wave: scoped re-review verdict

Package: `review-5cabf32..d2b622a.diff`, six commits, ten files. Base `5cabf32` is a
controller ledger commit touching only `docs/decision-ledger.yaml`, so no controller
work falls inside the graded range (verified: `git show --stat --oneline 5cabf32`).

**Merge verdict: READY.** New breakage introduced by this diff: **0 Critical, 0
Important, 3 Minor** (all in the new shared test helper's contract, all recorded as
harvest, none opening another wave).

Every mutation below is **mine, not the fixer's**. Where the fixer used a cell or a
spelling, I deliberately used a different one. Every in-tree mutation was restored with
`command cp` from a pristine copy outside the repository and verified BY CONTENT with
`md5sum`, `pnpm build` re-run between every edit and every end-to-end run including the
restores, exit codes captured per command and never through a pipeline.

---

## Per-finding verdicts

**I-1 (`batch-profile-none` had no producer) - ADDRESSED.**
The producer landed in the place acceptance row W5-b names: `smoke.spec.ts`'s existing
`batch view: dry run` scenario, visible before the pick and hidden after it.
My mutation (an inversion, not the fixer's `v-if` swap): point the paragraph at a
different REAL id, `$t("batch-profile-none")` -> `$t("batch-recents-empty")`, so it
still renders non-empty prose. `npx playwright test e2e/smoke.spec.ts` -> **1 failed, 45
passed**, the failure being that scenario. Discriminates.
**Measured scope, stated rather than assumed:** I reverted this plan's own rewording in
BOTH catalogs (dropping ", or create one in the Editor view." / ", oder erstelle eines
in der Editor-Ansicht.") -> `check:i18n` exit 0 and **103 passed**. The producer pins
the render path and the id, not the value, because both sides of the assertion read the
same catalog through `en(id)`. That is the house depth this repo states for itself in
D110's own residual paragraph ("the frontend gate compares ids, not value distinctness")
and it is exactly what W5-b asks for, so it is compliance, not a shortfall - but it is
worth knowing that this producer would not have caught the rewording being reverted.

**I-7 (the reworded locale hint had no producer) - ADDRESSED.**
`smoke.spec.ts` now asserts `#settings-locale-hint` against
`enAttr("settings-locale-label", "hint")`, placed while the UI is still English so the
en catalog is the right source.
My mutation: bind the hint to a different real attribute,
`$ta("settings-locale-label").hint` -> `$ta("settings-mkvmerge-path-label").hint` (the
fixer used `settings-default-jobs-label`). -> **1 failed, 45 passed**. Discriminates.
Same `enAttr` scope caveat as I-1, same house depth.

**I-2 (no keyboard test over the enumerated combinations) - ADDRESSED.**
One new case drives all six documented combinations against the rule count.
Two mutations of mine, one removal and one inversion, neither the fixer's:
- **G1, removal of one spelling only:** `(key === "z" && event.shiftKey) || key === "y"`
  -> `(key === "z" && event.shiftKey)`. -> **1 failed, 16 passed.**
- **G2, an inversion:** `if (key === "z" && !event.shiftKey)` -> `if (key === "z")`, so
  Shift+Z falls into undo instead of ever reaching redo. -> **1 failed, 16 passed**, the
  failure being the new case by name.
Both times only the new case reddens, so it discriminates without collateral.

**I-3 (the save-marking direction) - ADDRESSED, and this is the strongest of the five.**
Three experiments:
- **E1, the review's own inversion:** `savedSnapshot.value = JSON.stringify(profile)` ->
  `savedSnapshot.value = history.value[position.value]`. Before this wave that mutation
  left **101 passed**. Now: **1 failed, 102 passed**, and the one failure is the new
  I-3 case.
- **E2, a second spelling of the same defect** (to check the case is keyed to the
  mechanism and not to one token): `savedSnapshot.value = JSON.stringify(model.value)`,
  i.e. re-reading the live model at the far end instead of the captured one. -> **1
  failed, 102 passed.** Same case.
- **E3, the decisive one - is the new infrastructure load-bearing or decoration?** I
  neutralised `gatedWith` inside `mocks.ts` so it resolves immediately like
  `resolveWith`, AND kept E1's inversion in place. Result: **1 passed.** With the gate
  removed the inversion is invisible again, exactly as it was before this wave. So the
  held-open response is not scaffolding around the test - it *is* the test's power, and
  without it this producer would be one that cannot fail in the direction it exists for.

**I-4 (the two mappings past `close_decision`) - ADDRESSED.**
Two pure functions extracted, `close_dialog_strings` and `close_action`, each with a
four-row unit test pinning concrete values rather than "non-empty".
Two mutations of mine, both on cells the fixer did **not** use:
- **F1:** point `ConfirmAbortAndDiscard` at the plain `close-discard-*` wording, so the
  combined dialog silently drops the running-jobs half. -> `cargo test` **87 passed, 1
  failed**, `close_dialog_strings_map_each_decision_to_its_own_wording`.
- **F2:** map `ConfirmAbortAndDiscard` to `ExitDiscard`, so a confirmed close with a run
  in flight quits without aborting it. -> **87 passed, 1 failed**,
  `close_action_maps_each_decision_to_its_own_effect`.
Both are the dangerous direction and both are now caught.

**I-5 (the create-affordance terminology) - ADDRESSED at all five sites.**
`help/de/view-editor.md:3,7,9` now read `Erstelle` / `erstellt` / `Erstellen`;
`help/en/view-editor.md:3,7` read `Create` / `creates`. Corpus sweep:
`command grep -rn "nlege\|Anlegen\|Lege .* an\|legt .* an" help/de/` returns exactly one
line, `help/de/editor-output-filename.md:10`, about the filename not creating
subdirectories - a different referent that mirrors the corpus diagnostic itself
(`path-separator-in-rendered-name` de: "Muxsmith legt niemals Unterverzeichnisse an"),
correctly untouched.
`command grep -rniE "start(s)? a (new )?profile" help/en/` returns nothing. The English
collision with the corpus's own "start a run / start a batch" is gone with it.

**I-6 (the falsified mechanism paragraph) - ADDRESSED.**
The in-place supersession marker is present in the house form, the original paragraph
left standing as the record. **Both cited line numbers verified at the artifact rather
than accepted:** `EditorView.vue:330-332` is exactly `saveDisabled`'s definition and
`:349` is exactly `if (!sessionActive.value || !value) {`. The fixer re-grepped instead
of copying my numbers forward, and they are right. (`EditorView.vue` is byte-identical
to its pre-wave state, `md5 404b1f7e...`, so the citations could not have drifted under
the wave either.)

**M-4 (the ADR's "every current call site is a literal" claim) - ADDRESSED.**
The false clause is replaced by a statement of what is actually there. I re-measured the
replacement's own claim on today's `run.rs` (post-extraction, so line numbers moved):
`command grep -nE 'ftl_message\([a-z_]' src-tauri/src/run.rs` returns five lines, of
which `:565` is the definition and `:1788` a doc comment; the three real non-literal
call sites are `:1727`, `:1751`, `:1755`, all after `#[cfg(test)]` at `:1090`, with
`:1727` inside `close_abort_strings_resolve_from_the_ftl_catalog` (`:1704`) and
`:1751`/`:1755` inside `ftl_message_falls_back_to_the_key_and_never_prefix_matches`
(`:1742`). Exactly what the corrected sentence says, function names included. The new
sentence also carries no line numbers, which is what makes it survive the extraction it
shipped beside.

**O-1 (`v-show` outside the D112 render-gate rule) - ADDRESSED.**
Fired in both directions on the widened selector: `v-show="!model"` on the `editor-empty`
paragraph now produces the D112 message and `npx eslint .` exits 1 (before the widening
the same probe exited 0 with zero findings, measured in the first verdict); the shipped
tree lints clean; and the carve-out the widening must not disturb is intact -
`:disabled="!model || !canUndo"` and `:disabled="!model || !canRedo"` are present at
`EditorView.vue:928` and `:936` and legal, because the selector matches on the attribute
NAME and `disabled` is not in the widened list.

---

## The three rulings

### 1. The new shared test infrastructure (`gatedWith` / `releaseGate`) - SOUND

Graded as infrastructure, not as a passing test.

**Is it load-bearing?** Yes, and this is measured rather than argued: E3 above. Neutralise
the gate and the finding's own inversion goes invisible again. The mechanism earns its
place.

**Is the gate deterministic?** Yes. There is no timer anywhere in it. The pending promise
settles only through `releaseGate` -> `page.evaluate` -> `window.__muxsmithReleaseGate__`
-> the stored resolver. Nothing races. Its own doc's supporting claim also checks out:
`command grep -rn "waitForTimeout\|setTimeout\|sleep(" e2e/ --exclude-dir=.generated`
returns two lines, one being that doc comment itself and one a `test.setTimeout(90_000)`
budget on the depth-cap case - so the suite genuinely had no timing-based wait to fall
back on, and the fixer did not invent that constraint to justify the mechanism.
Empirically: **three consecutive full runs, 103 passed each, 8.1-8.2s**, no flake in the
one timing-sensitive case.

**Can it hang or leak between cases?** Not between cases. `gateResolvers` is created
inside `installMockIPC`, which Playwright re-evaluates fresh on every navigation, and
each test gets its own `page`; there is no cross-test channel. Within a case, an
unreleased gate leaves the awaited `invoke()` pending until the Playwright per-test
timeout - a named failed test, not a hung suite. Bounded.

**Does it change behaviour for a scenario that does not use it?** No. The `resolve` path
is now `return result.value;`, byte-equivalent to the old ternary's else-branch, and
`reject` is untouched; the only unconditional addition is assigning
`window.__muxsmithReleaseGate__`. Every pre-existing case still passes and none moved.

**Is the shared file's contract still honest?** Mostly, with one real gap and two
unreached hazards - all three Minor, all three harvest:
- **The contract now has an unnamed exception.** `MockScenario.commands`'s own doc says
  "An unmocked command throws in the page instead of silently hanging the app's
  `invoke()` promise forever." `gatedWith` deliberately introduces exactly that hang,
  bounded only by the test timeout. That is the point of the mechanism, but the file's
  stated no-hang principle now has an exception it does not mention.
- **Same-gate-name reuse is undocumented.** `nextResult` repeats the last queue entry
  once the queue is exhausted, so a second call to a gated command would register a
  second promise under the same gate name and overwrite the first resolver in the Map,
  orphaning it. Unreached today: the only gate user asserts exactly one `save_profile`
  call (`command grep -rn "gatedWith\|releaseGate" e2e/ --exclude-dir=.generated`
  returns four lines, all in one test). One clause - "one outstanding call per gate
  name" - would close it.
- **Layering is undocumented for gates.** `installMockIPC` is explicitly re-registerable
  (the german-locale case does it); a second registration builds a fresh `gateResolvers`
  and overwrites the global, so a gate pending from the first becomes unreleasable.
  Near-moot, since a re-registration only takes effect after a navigation, which tears
  down pending promises anyway.

**One correction to the framing in your dispatch:** this is *not* the infrastructure an
earlier task deferred. Task 2's fork 2 deferred a **stateful settings store** in
`e2e/mocks.ts`, for save-then-reopen without a reload (progress.md's close-action
tracker). A held-open response is a different mechanism and does not discharge or
pre-empt that item, which stays on the tracker. Building new test infrastructure here is
also not a rule breach: `tests-ship-with-the-feature-never-after`'s own narrow exemption
reserves deferral for harness/runner/framework work, it does not forbid adding it.

### 2. The I-4 shape deviation - the fixer is RIGHT and my verdict's suggestion was WRONG

Measured, not weighed. I built the shape I had proposed - keys returned, `ftl_message`
called over a variable at the call site - on one arm (`ConfirmDiscard`) and ran it:

- My proposed shape, everything else untouched: **88 passed.** The change is invisible.
- My proposed shape **plus the red state D110 decision 4 prescribes for part (b)**
  (delete `close-discard-title` from `locales/de/gui-common.ftl`): **88 passed.** The
  guard no longer fires.
- **Control, shipped shape, same deleted key:** `cargo test` **87 passed, 1 failed**,
  `every_row_carries_every_key_the_shell_source_literally_looks_up`, message
  `locale "de" has no non-empty value for key "close-discard-title" (shell-consumed, per
  a literal ftl_message call in run.rs)`.

So the fixer's measurement is exactly right, and the consequence is worse than "eight
keys leave a set": my shape would have silently disabled an existing guard's own
prescribed red state, which is the derivation-blind-spot class this whole review exists
to catch. The alternative is genuinely worse, and the fixer rejected it before shipping
rather than after being challenged, with the ground stated.
**And the extraction still does what the finding required:** F1 and F2 above both
discriminate, on cells the fixer never used. Keeping the lookups inside the extracted
function costs nothing for testability, because the function is pure in `(decision,
locale)` either way.

### 3. The I-6 sibling left alone - CONFIRMED, and measured rather than concurred

You are right that two agents agreeing is not evidence, so I looked for something
checkable. The deciding property is whether
`docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md:245-252` is a point-in-time
recon inventory or a standing mechanism claim. That section cites eight line ranges. I
resolved every one of them against today's tree:

| Cited | Claimed to be | What is actually there today |
|---|---|---|
| `error.rs:44` | `pub params: HashMap<..>` | a `ParamValue` doc comment |
| `error.rs:169-170,174` | numeric params | (shifted) |
| `run.rs:935` | an `index` param | `run_id,` |
| `check-i18n.mjs:42-50` | IpcError residual | the "used" definition prose |
| `check-i18n.mjs:247-297` | cross-locale parity | file reading |
| `check-i18n.mjs:108-127` | line-based id parser | path constants (parser is at `:152`/`:162`) |
| `run.rs:539-560` | `ftl_message` | `lookup_in`'s doc (`ftl_message` is at `:565`) |
| `eslint.config.js:61-68` | `no-raw-text` | the D112 rule comment (`no-raw-text` at `:8`/`:11`) |

**Every citation in that section is stale, and they went stale across several plans, not
this one.** That is structural evidence of a snapshot rather than a maintained claim, and
it is the property that separates it from the plan-7.5 paragraph, which carries no line
citations at all and reads as a standing description of how the mechanism works. Same
class as the ROADMAP and journal 43-figures already ruled to owe nothing. **Leaving it is
correct.**

One residual I name rather than reopen: that paragraph also carries a clause that is
normative rather than snapshot - "those four must stay single-line, attribute-free
messages" - and "four" is now ten. It is not load-bearing, because the constraint is
stated live and correctly in three other places (`locales/en/gui-common.ftl:5`,
`locales/de/gui-common.ftl:8`, `run.rs:553`) and enforced mechanically by `RUST_ONLY_IDS`
plus `every_row_carries_every_key_the_shell_source_literally_looks_up`. Harvest, not work.

---

## The two things a wave like this most often gets wrong

**Was any existing assertion weakened, reworded, deleted or skipped? NO - measured with a
firing control.**
I extracted every removed line in the delta across `e2e src src-tauri locales help
scripts eslint.config.js` and searched it for assertion tokens
(`expect(|assert_eq!|assert!|toBe|toHave|toBeVisible|toBeHidden|toBeEnabled|toBeDisabled`):
**zero matches.** Control: the same pattern over the ADDED lines returns **25**, so the
pattern works. The complete deletion set is one import line (widened), three `mocks.ts`
lines (the union type and the ternary, both widened, `resolve`/`reject` behaviour
preserved), four `eslint.config.js` lines (comment and selector, widened), five help
prose sentences (the terminology repair), and `run.rs` signature/doc/match-arm lines
consumed by the extraction. Nothing else.
No `test.skip`, `test.only`, `.fixme`, `describe.skip`, `#[ignore]` or timeout change
anywhere in the delta. Test counts moved in one direction only: Rust **86 -> 88**, e2e
**101 -> 103**.

**Do the acceptance-map rows my walk found false now match what the branch contains? YES,
and no row was edited to make that true.**
The plan document is untouched by the range (`git log 5cabf32..d2b622a -- <plan>` is
empty) and untouched by the base commit. So the rows were left alone and two of them
became true by construction:
- **W5-b** ("the rendered string asserted through `en(id)` in the existing batch
  scenario") -> **now TRUE**: the producer is in that exact scenario,
  `smoke.spec.ts:211` and `:217`.
- **W3-o** ("Task 4, the keyboard test over the enumerated combinations") -> **now TRUE**:
  `editor-undo-redo.spec.ts:644`, Task 4's own file, all six combinations.
- **W4-m** -> still false, unchanged, as ruled: a close-time text correction, the
  observable riding the 1.x GUI-test-harness item.
- **W3-h** ("`readModel` on the mount-harness case") -> **still false**, unchanged:
  `command grep -c "readModel" e2e/editor-undo-redo.spec.ts` returns **0**. This was my
  non-blocking Minor and it stays a close-time row correction. Saying so plainly, since
  there is no second wave: **the row still names a producer that is not there, and the
  close must correct the row's text.** Its observable is materially covered by the
  rendered assertions, so nothing is untested; only the map is wrong.

---

## Delivered state

| Check | Result |
|---|---|
| `git status --short` | empty |
| `md5sum` of all eight files I mutated vs pristine | identical |
| `pnpm build` | 0 |
| `npx playwright test` x3 | 103 passed, 103 passed, 103 passed |
| `cargo test --workspace` | 0 (`muxsmith-gui` lib 88 passed) |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 |
| `cargo fmt --all --check` | 0 |
| `npx eslint .` | 0 |
| `node scripts/check-i18n.mjs` | 0 |
| `npx tsc --noEmit -p e2e/tsconfig.json` | 0 |
| `python3 scripts/ledger-lint.py` | 0, 585 entries, invariants hold |

**Typography: clean**, over the added lines of both the delta and the whole branch
`bd3aa34..d2b622a`, with a detector built from explicit code points and **self-tested**
(a probe string containing all eleven glyphs is reported as containing all eleven before
the scan runs).

---

## Harvest

**H-6. A producer written to close a coverage finding needs its own load-bearing test,
and the test is: neutralise the new mechanism and re-run the finding's mutation.**
I-3 is the case. The obvious check - "does the new test go red under the inversion?" -
passes, and passes for a case that would still have been useless. The check that
actually establishes the producer is E3: neutralise the gate to an immediate resolve,
keep the inversion, and see the case turn green again. That distinguishes "the test
detects the defect" from "the test's new infrastructure is what detects it", and only
the second answers whether the infrastructure earned its place. Suggested statement:
*where a producer needs new infrastructure to reach its state, grade the infrastructure
by disabling it and re-running the mutation the producer exists for; if the mutation
stays caught, the infrastructure was decoration.*

**H-7. Two producers for the same finding class can honestly sit at different depths, and
the depth is set by what the surrounding suite can already do.** This wave shipped
`en(id)`-relative assertions for I-1 and I-7 (a catalog reword is invisible, measured)
and literal-value assertions for I-4 (a reword reddens). Both are right: the e2e suite's
established pattern is `en(id)`, and `run.rs` already had a pinned-wording assertion to
extend. The defect would be shipping the weaker depth *silently*. Suggested clause on
`a-normative-claim-is-scoped-down-to-its-producers-reach`: *a new producer states which
mutations it does not see, in the same breath as the finding it closes - "asserted
through the catalog helper" and "pinned to a literal" are different guarantees and the
row that names one must not read as the other.*

**H-8. A reviewer's proposed fix shape is a borrowed claim like any other, and the fixer
who measures it is doing the review's job.** My verdict proposed the key-returning shape
for I-4 on architectural grounds and never ran it. Run, it silently disables an existing
guard's prescribed red state. The fixer measured, rejected it before shipping, and said
so. Suggested statement: *a fix dispatch may carry a suggested shape, never a mandated
one; the implementer owes the measurement and a deviation with a measured ground is
compliance, not a deviation to be graded down.*

**H-9. My own typography instrument failed the exact test I apply to others.** The
detector's enumerated bad-glyph set was written as literal characters; one of them
normalised to a plain space, so the check matched every line in the diff and reported a
false positive on the first pass of this re-review - and, in the first verdict, the same
detector reported "clean" without my ever having confirmed it could fire. Rebuilt from
code points with a self-test that prints all eleven glyph names before scanning; both
runs then came back genuinely clean. This is the enumerated-set-inside-the-instrument
trap from the house rules, met on the reviewing side rather than the authoring side.
Suggested clause: *a detector whose pattern contains an enumerated set of literal
characters prints its own self-test before its result - the set is a claim, and a
literal-character set is exactly the kind that normalises silently between the author's
keyboard and the file.*

---

*Same reviewer as the whole-branch verdict. Every figure measured at the artifact named
beside it; every mutation mine, restored and verified by content.*
