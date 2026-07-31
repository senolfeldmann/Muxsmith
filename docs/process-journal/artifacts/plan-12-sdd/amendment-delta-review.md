# Plan 12 - one-pair amendment delta review (same reviewer)

Scope: the amendment only. The approved plan is not re-reviewed, and nothing I
confirmed in earlier rounds was re-examined except where this diff touches it.

Artifact: `docs/superpowers/plans/2026-07-30-plan-12-qa-round-3-findings.md` at
`9bc06e6`, 1267 lines, diffed against `0325923` (1242 lines) - the version I
confirmed. Nine hunks, +35/-5, plus a ledger entry outside this review's scope.
Instruments, fresh, at
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr12-amendment-independent/`:
the extracted diff, a node comparator over Files lists versus `git add`, and
source-level probes of `src-tauri/src/run.rs`'s test module, the e2e specs and
`playwright.config.ts`.

## Verdict: APPROVED

The amendment is sound and is genuinely one-pair: no task added, removed or re-cut;
one paragraph's disposition changed, one ADR decision appended, one step joined
Task 6, three acceptance rows added. The load-bearing design claim - that the
trigger is a strengthening rather than any change - holds under my own exhaustive
walk of decision 5's table, and it is what keeps a fifth message, a catalog key and
the budget out of the amendment. **Two minor items to fix, neither in the design and
neither introduced by the ruling**, and I do not make approval conditional on them:
one is an incomplete reconciliation with a neighbouring rejected alternative in the
same ADR, the other a pre-existing wrong ordinal that the amendment makes more
likely to mislead. No safeguard removals are recommended. The plan is ready to
execute.

**New findings: 2 minor. Reproductions: all reproduced, none diverged.**

---

## The six dispositions

### 1. The controller's correction, as the plan's own text carries it - CORRECT BUT INCOMPLETE (finding A1)

Decision 9 states it directly: "**This extends decision 5; it does not reopen it.**
Decision 5's four variants already read both facts and give the coinciding case its
own message, and its rejection of 'two prompts in sequence' is about assembling ONE
situation out of two dialogs. This is a different situation - the state CHANGED
between the read and the confirm ... The table is unchanged and is the vocabulary
below." The precedent-sweep entry says the same and the four-variant table is
untouched. I checked the table itself: four rows, unchanged, and decision 9 speaks
in its variants rather than adding one.

**That reasoning is right on the point it addresses, and it addresses one of three
grounds.** Decision 5's rejected alternative reads, in full: "Rejected because it
doubles the cancel paths, nests a second dialog inside the first's async callback in
the shell, and asks the user two questions where one action follows." The third
ground is the situation ground, and the correction disposes of it correctly. The
first two are now incurred by decision 9 itself: its **Where** bullet puts the
re-read "Inside the abort/discard dialog's own callback, on the confirming branch",
which is the nesting the second ground names, and the second prompt's own decline
path is a second cancel path. The alternative's text was not touched by the
amendment, so the ADR will render both, unreconciled. See A1.

### 2. The strengthening trigger - CONFIRMED, tested exhaustively against decision 5's table

I walked all twelve cells myself rather than reading the plan's summary, treating
each variant as the fact set it states: `Close` = {}, `ConfirmAbort` = {abort},
`ConfirmDiscard` = {discard}, `ConfirmAbortAndDiscard` = {abort, discard}. The rule
returns `Some(current)` iff `current` carries a fact `answered` did not.

| answered | current `Close` | `ConfirmAbort` | `ConfirmDiscard` | `ConfirmAbortAndDiscard` |
|---|---|---|---|---|
| `ConfirmAbort` | None | None | **Some(D)** | **Some(A&D)** |
| `ConfirmDiscard` | None | **Some(A)** | None | **Some(A&D)** |
| `ConfirmAbortAndDiscard` | None | None | None | None |

**Every strengthening is covered:** four cells, and they are exactly the four where a
fact appears that the answered dialog did not state. The two mixed cells are the
interesting ones and the rule gets them right - `ConfirmAbort` -> `ConfirmDiscard`
loses the abort fact and gains the discard fact, and it re-prompts, because the user
was never told anything would be discarded.

**Every non-strengthening is genuinely safe to proceed on**, which is the half a
broken design could not see, so I checked each of the eight `None` cells against what
the callback then does. On `None` the callback performs the *answered* variant's
action: `abort_and_quit` for the two run-bearing variants, `app.exit(0)` for
`ConfirmDiscard`. In all eight the action is one the user agreed to, executed over a
state that is a subset of what they were told:

- the three `Close` cells: no facts remain; the user asked to quit and the app quits.
  `abort_and_quit` over an empty slot exits immediately, which is the pre-existing D31
  path its own doc comment already describes ("right here when the run already tore
  down while the dialog was open").
- `A&D` -> `A` and `A&D` -> `D`: one agreed fact evaporated; the remaining action is a
  strict subset of what was agreed.
- the three unchanged cells: nothing to say.

**There is no unsafe weakening structurally available**, and the reason is worth
recording because it is what makes "a weakening never re-prompts" safe rather than
merely convenient: every dialog in this family ends in a quit, the only variation is
whether a run is also aborted, and aborting a run that no longer exists is a no-op.
So no weakening can produce an action the user did not agree to. The claim holds.

### 3. The decline/confirm asymmetry - CONFIRMED

Sound, and its correctness rests on the placement rather than on the wording.
Declining leaves the app exactly as it was because the re-read sits **before**
`abort_and_quit` or `app.exit(0)`, so no quit has been armed - `quit_after_finished`
is set inside `abort_and_quit`, which has not run - and `api.prevent_close()` from
the first pass means the window never closed. The plan states both halves of that
("Placing it before the action is what makes declining cost nothing"). Confirming
being terminal is the right counterpart: the user has now been shown the full fact
set of the current state and agreed to it. The stated ground for the asymmetry - "the
second prompt exists because the user is being told something they had not been told,
so the safe answer to 'no' is to keep the app alive" - puts the irreversible answer
behind the informed one, which is the correct direction.

### 4. The one-re-read bound and its residual - BOUND CORRECT, PREMISE INCOMPLETE (finding A2)

The bound itself is right and the rejection of a counter is right: a counter would
bound a livelock the user can drive by typing during each dialog, which is a
different problem from the one at hand, and "exactly one re-read" is structural.

**Running the premise rather than weighing it, as instructed.** The paragraph argues
"once the combined variant is shown, both facts are stated and no further
strengthening exists", then names one exception (a run starting during a
`ConfirmDiscard` second prompt). But the four `Some` cells produce **three** distinct
second-prompt variants, not one:

- `A&D` (from `A`->`A&D` and `D`->`A&D`): both facts on screen, no further
  strengthening available. The paragraph's sentence covers exactly this case. ✓
- `ConfirmDiscard` (from `A`->`D`): a run starting during it is a further
  strengthening. **Named**, with its exit through `app.exit(0)` on a fresh run
  attributed to pre-existing D31 behaviour. ✓
- `ConfirmAbort` (from `D`->`A`): **not named.** The user edits during this second
  prompt, the editor goes dirty again, and confirming runs `abort_and_quit`, losing
  the new changes. The dialog they just answered said nothing about changes.

So "no further strengthening exists" is asserted over one of three second-prompt
variants and an exception is named for a second. The third path's consequence is
bounded and stateable - the discard fact *was* stated, by the first dialog of the same
sequence, which the user confirmed - so this is not the data-loss class the original
direction-3 defect was, where the fact had never been stated at all. But the
enumeration is incomplete in the same shape, on the same precedent, one level down.
See A2.

### 5. The twelve-cell matrix - CONFIRMED complete, with the `None` cells correctly identified as the load-bearing half

3 answered variants x 4 current variants = 12, and restricting `answered` to the
three dialog-producing variants is right rather than a gap: the function is only
reachable from a confirm callback, and `Close` produces no dialog to confirm. The plan
says so explicitly ("the three that produce a dialog"). By my walk the split is **4
`Some` and 8 `None`**, and the plan requires each cell asserted explicitly rather than
by example, so the split does not need stating.

The `None` cells are named as the report duty and correctly identified as what a
broken rule passes: "Name the cells that must be `None` in the report as well as the
ones that must be `Some`, since the silent-no-prompt cells are the ones a broken rule
would pass." W4-u and W4-v are the two sides of that one table and W4-v carries the
same note. That is the halves rule applied to a truth table, which is the right shape:
a rule that returned `None` unconditionally would pass eight cells and fail four, and a
rule that returned `Some` unconditionally the reverse, so neither side alone is a
producer.

### 6. W4-w's not-machine-verifiable marking - HONEST, verified at the source

I applied the same test as last round's mock constraint: can the existing harness reach
it at all?

- **The Rust unit tests cannot.** The test module in `src-tauri/src/run.rs` constructs
  `AppState` directly - 19 references - and does not touch the runtime: my probe for
  `Window|app_handle|WebviewWindow|tauri::test|mock_builder` inside the test module
  returned three hits, and I checked each rather than counting them, because the
  pattern is ambiguous by construction: all three are the word "Windows" (the operating
  system) inside comments. So **no test constructs a `Window`, an `AppHandle` or a mock
  Tauri runtime.**
- **The e2e suite cannot.** No spec references `close-requested` in any spelling; the
  only hits in the tree are the Tauri JS API's own event-name constant compiled into
  the gitignored `e2e/.generated/` bundles. And `playwright.config.ts`'s `webServer`
  runs `vite preview` - a static server, no Tauri shell - so `on_close_requested` is
  never registered in an e2e run and the OS close event does not exist there.
- **The recorded reason the plan leans on is real:** `close_decision`'s own doc comment
  says it is "factored off the Tauri types so it is unit-testable", which is exactly the
  constraint W4-w inherits.

Making it machine-verifiable would need `tauri::test::mock_builder`, a test runtime the
workspace does not use - new test infrastructure, the same shape as last round's
releasable-mock finding. So the marking is honest rather than convenient, and the
self-review stating one non-machine-verifiable row instead of claiming zero is the
correct correction. The producer named for it is a reviewer check with the reason
attached ("naming the matrix here would be one producer covering the side that cannot
fail"), which is the right disposition for a wiring claim.

---

## New findings

### A1 (MINOR) Two of decision 5's three rejection grounds are now incurred, and decision 9 does not say so

**Location.** D109 decision 9's opening ("This extends decision 5; it does not reopen
it"), against D109's third rejected alternative, which the amendment left untouched.

**What is wrong.** The rejected alternative reads: "Rejected because it **doubles the
cancel paths**, **nests a second dialog inside the first's async callback in the
shell**, and asks the user two questions where one action follows." Decision 9's
reconciliation answers the third ground and is silent on the first two, both of which
its own design now incurs: the **Where** bullet places the re-read inside the first
dialog's callback and shows the second dialog from there, and the second prompt's
decline path is a second cancel path. The ADR will therefore render, in one document, a
rejected alternative whose stated grounds are partly satisfied by an adopted decision,
with no record of who weighed them - and "**Must not decide**" now forbids reopening
the question, so the next reader cannot resolve it either.

**Why it matters beyond tidiness.** Decision 5's grounds were costs, and the owner's
ruling accepts two of them. That is a legitimate outcome and the honest one to record;
what is not honest is a reconciliation that reads as though no ground applied.

**Ruling.** FIX with one clause in decision 9 and one in the rejected alternative:
decision 5's rejection rested on three grounds, the situation ground genuinely does not
reach a state that changed between read and confirm, and the nesting and the second
cancel path are costs the owner's ruling accepts in exchange for closing R23's residual.
Nothing in the design moves.

### A2 (MINOR) The no-further-strengthening premise covers one of three second-prompt variants

**Location.** D109 decision 9's fifth bullet ("It cannot loop, by construction").

**What is wrong.** Detailed in disposition 4 above. Three second-prompt variants are
reachable (`ConfirmAbortAndDiscard`, `ConfirmDiscard`, `ConfirmAbort`); the sentence
reasons about the first, the exception names the second, and the third - a
`ConfirmAbort` second prompt during which the editor goes dirty again - is unaddressed.

**Ruling.** FIX by completing the enumeration in the same bullet: three second-prompt
variants, the combined one admitting no further strengthening, the `ConfirmDiscard` one
admitting a starting run with its D31 exit, and the `ConfirmAbort` one admitting a
re-dirtied editor whose discard was stated by the first dialog of the same sequence and
confirmed. Stating the third does not change the bound and does not add a case to
handle; it makes the paragraph's conclusion true as written.

### A3 (MINOR, pre-existing, in scope because the dispatch asked for it) Task 1's ordinal into D109's rejected alternatives is wrong

**Location.** Task 1, Step 3's D109 slot: "its **third** rejected alternative - shipping
the shell's dialogs in English with a recorded reason - as OVERRULED by the owner".

**What is wrong.** Measured from the file, D109's rejected alternatives in order are:
1 no guards at all; 2 an unconditional warning; **3 two sequential prompts**; 4 shipping
the shell's dialogs in English; 5 handling the close event in the frontend. The named
alternative is the **fourth**. Untouched by this amendment, so the defect predates it and
I missed it in earlier rounds - I am reporting it because it is exactly the
surviving-ordinal class the dispatch asked me to look for, and because the amendment
makes it more likely to mislead: an implementer following "third rejected alternative"
now lands on the sequential-prompts alternative, the one decision 9 is in tension with
(A1), and could read Task 1 as instructing that *it* be recorded as overruled by the
owner.

**Ruling.** FIX by dropping the ordinal - the alternative is named, so the ordinal carries
no information - or by correcting it to fourth. Cheapest and most robust: name it only,
since a later inserted alternative would stale the ordinal again.

---

## Reproductions

| # | Claim | My measurement | Verdict |
|---|---|---|---|
| M1 | 42 requirements, highest `R42` | 42 rows, max `R42` | reproduced |
| M2 | 69 acceptance halves, 10 + 12 + 22 + 23 + 2 | 10, 12, 22, 23, 2; total 69 | reproduced |
| M3 | D109 items 1-9, in order after the renumbering | `1 2 3 4 5 6 7 8 9`, in order; item 9 is the re-read. The diff shows the item **appended** with no removed lines in that hunk, so nothing was renumbered in the decisions list | reproduced |
| M4 | Cross-references survive the change | Checked every `D109 decision N` reference: "decision 2" -> item 2 (New warns, the strike note in Task 5 case 4) ✓; "decision 5" -> item 5 (the four-variant table) ✓; "decisions 4 and 5" in Task 6's Read-first ✓; "decision 9" -> item 9 ✓. **One ordinal into a different D109 list is wrong** - see A3 | reproduced, with one defect found |
| M5 | 9 absence checks | 9 (case-insensitive census, the unit I corrected in round 2) | reproduced |
| M6 | 7 corrections | 7 rows | reproduced |
| M7 | Gate audit 1 | 1 hit, the self-audit sentence | reproduced |
| M8 | Typography 0 | 0 for em/en dash, curly quotes, ellipsis, NBSP, minus, figure dash, horizontal bar | reproduced |
| M9 | Files list equals `git add` for all seven tasks | 2/2, 6/6, 4/4, 6/6, 5/5, **9/9**, 4/4 - all seven match. Task 6 gained Step 2b and no file, so its list is unchanged, which is what "one-pair" predicts | reproduced |
| M10 | The four-variant table is not reopened | unchanged, four rows, and decision 9 speaks in its variants | reproduced |
| M11 | No fifth message, no catalog key, no budget movement | the amendment touches no catalog block and no budget table; Task 6's Files list still carries the same two `gui-common.ftl` entries and the six fenced ids are unchanged | reproduced |
| M12 | The twelve cells are the exhaustive domain | 3 dialog-producing `answered` x 4 `current` = 12; my independent walk gives 4 `Some` / 8 `None`, every strengthening covered and every `None` safe | reproduced |
| M13 | W4-w cannot be machine-verified by the existing harness | no Rust test touches the Tauri runtime (the three apparent hits are the word "Windows" in comments); no e2e spec references close-requested; `playwright.config.ts` serves `vite preview` with no Tauri shell; `close_decision`'s doc records the factoring reason | reproduced |

**Nothing diverged.** No reproduction disagreed with the plan's figures, and the only
measurement that produced a defect (M4) found it in a pre-existing sentence rather than
in the amendment.

## Scope confirmation

The nine hunks are: the R42 row, the Task-6 model-tier ground, the precedent-sweep
disposition, D109 decision 9, the three acceptance rows, Task 6 Step 2b, the matrix
bullet, the **Must not decide** extension, and the two self-review count sentences.
Nothing else in the plan moved. The `raw:` ruling is not in this diff, as stated. I
recommend no safeguard removals; decision 9 adds one testable rule and one honest
non-machine-verifiable row, and both belong in the may-not-argue-away set on the same
ground as the rest.
