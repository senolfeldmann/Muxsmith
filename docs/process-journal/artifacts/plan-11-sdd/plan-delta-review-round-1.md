# Plan 11 - delta review of fix round 1

Reviewer: the resumed original plan reviewer (same judge, same standards as
`.superpowers/sdd/plan-11/plan-review-round-1.md`). Settled non-findings are not
re-litigated here.

Artifact: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
at `fac7b50` (165 insertions, 71 deletions against the reviewed `148f19f`).
Requirement set: `.superpowers/sdd/plan-11/plan-brief.md` plus the two owner-ruled
controller amendments (the `deny.toml` boundary lift and the README example step),
per the dispatch. Tree during review: `master` at `9ad9e05`; the co-writer's
ROADMAP work has landed, so no working-tree divergence affected any measurement.
The repository is unmodified by this review apart from this file
(`git status --porcelain` shows only the untracked Plan-12 document, which is the
co-writer's).

Instruments built fresh for this round, all under
`.../scratchpad/pr11-review-independent/round2/`, none of them reused from round 1
and none of them a path the author would pick: `rv_denyfire.py` (builds the
post-Step-4 `deny.toml` from the plan's two fenced insertions and drives three
variants through `cargo deny -c`), `rv_yamlblocks.py` (independent fenced-block
parser and corpus classifier over a wider surface than the plan's),
`cdsrc/` (cargo-deny 0.19.9 downloaded from `static.crates.io` and diffed against
the shared vendored copy), `endstate/` (the five fenced replacements applied to my
own copies to compute the end-state control), `examples/` (the three documented
profiles extracted and validated). The binary used is still my own build from
round 1.

**On the shared-instrument risk the dispatch names.** Three parties now agree on
the cargo-deny mechanism, which is exactly when agreement is worth least. I
therefore did not read the vendored source as evidence: I downloaded the crate
from crates.io to my own path, read the `Default` impl there, and only then diffed
it against `~/.cargo/registry` to establish that the shared copy is untampered.
Both files came back identical, so the shared read was sound - but that is now a
measured conclusion rather than an assumption.

---

## Verdict: NEEDS_FIXES

Both blocking findings are addressed at the root rather than at the sentence, and
I reproduced both with my own instruments: the cargo-deny account is now correct
about the mechanism, the counterfactual, the control and the blast radius, and it
records its own inversion instead of erasing it; the dead alternation control has
been re-pointed at a target I measured to contain a match, and it now
discriminates. Thirteen of the fourteen non-blocking findings are addressed, one of
them with a classification better than the one I proposed. The two owner-ruled
additions are sound: the README corpus derivation reproduces exactly, delta zero
included, and I could not break its discriminator; the `deny.toml` boundary lift is
recorded as a reversal by authority with nothing else taken as newly permitted.

It returns NEEDS_FIXES for a narrow reason and a broader one. Narrow: three
defects, two of them introduced by this round. The widened self-contradiction sweep
in Task A4 now returns six hits while the plan pre-classifies three and declares
that "a hit outside those three is a finding" - so as written it hands the
implementer three false findings in a required step. A new live positional gate
ordinal entered the plan's own Goal sentence, which falsifies the four-kinds
enumeration in its gate-count self-audit and is the very construction Task A1
exists to remove. And the `glib` parent figure was corrected at five of six sites
while the amendment asserts every site. Broader, and the reason I am not simply
listing three more members: my last two rounds of findings differ mainly in scope
inside one family - an expression or an enumeration whose *expected result* came
from recall rather than from the artifact. That is the convergence trigger, and the
right answer is two written clauses plus one mechanical sweep, not a third round of
members. Both are in the closing section.

---

## Per-finding disposition

| # | Round-1 finding (severity) | Disposition |
|---|---|---|
| 1 | cargo-deny mechanism inversion (Blocking) | **ADDRESSED** - root |
| 2 | dead alternation control (Blocking) | **ADDRESSED** - root |
| 3 | `codec_kind` cited to `generated.rs` (Important) | **ADDRESSED** |
| 4 | "twelve direct parents" (Important) | **NOT_ADDRESSED** - five of six sites |
| 5 | `004e1e8^` line 73/74 (Important) | **ADDRESSED** |
| 6 | unscoped Windows claim in the spec fence (Important) | **ADDRESSED** - by cutting; I concur |
| 7 | A2 Step 1's open third case (Important) | **ADDRESSED** |
| 8 | retained-set exhaustiveness (Minor) | **ADDRESSED** - and it corrected my classification |
| 9 | "eight hunks" (Minor) | **ADDRESSED** |
| 10 | "every gtk-rs family crate" unenumerated (Minor) | **ADDRESSED** |
| 11 | unnamed control id in B1's fire (Minor) | **ADDRESSED** |
| 12 | A4 sweep's three-term set (Minor) | **ADDRESSED** - the terms; see new finding N1 for what the widening left behind |
| 13 | A2 test-duty premise (Minor) | **ADDRESSED** |
| 14 | pathspec named three of four house YAMLs (Minor) | **ADDRESSED** |
| 15 | item-2 surfacing duty had no acceptance row (Minor) | **ADDRESSED** |
| 16 | backticks in a quoted Tier-2 clause (Minor) | **ADDRESSED** |

**Tally: 15 ADDRESSED, 1 NOT_ADDRESSED, 0 disputed.**

Notes on the four that needed judgement rather than a check.

**Finding 4 - NOT_ADDRESSED, one site.** Six sites in the document state a `glib`
parent count. Five now say eleven (the authoring heading, the authoring sentence,
acceptance row W1-l, B1 Step 8, the self-review's recomputed-counts list). The
sixth still says **"Twelve direct parents"**: the `glib` row of the
deferred-by-decision table. The amendment section claims the figure was corrected
at "five sites", and five is exactly what was done - the count of *sites corrected*
was right and the count of *sites stating the fact* was one short. That is the same
shape as the original finding, which is why it is a disposition rather than a new
number: a repeated fact's sweep has to be derived from a search, and this one was
derived from a list. The same row also carries "which has not migrated off GTK3"
unattributed, where B1 Step 8 now correctly attributes that claim to `deny.toml`'s
comment; fixing both in one edit is cheapest.

**Finding 6 - cutting is right, and it drops no briefed requirement.** I weighed
the cut rather than accepting it. The brief requires the 8.1 block to state the
shipped surface and (via the plan's own correction 3) the 130 code; it nowhere
requires the spec to explain what an interrupted non-`run` subcommand returns. Both
surviving claims are measured - `130` is produced only in `commands/run.rs`, and
`ctrlc` appears only there - so nothing unverified remains. The project's record
does not lose the shell convention either: D15 already states it *with* its
qualifier ("128 + SIGINT, shell convention"), which is where a convention about
shells belongs. Rescoping was the alternative and it was worse here: stating the
POSIX half honestly requires stating the Windows half, which this machine cannot
measure, and the string is a must-not-decide fence the implementer may not adjust.
The new fence checks out mechanically too: its OLD text still occurs exactly once
in the spec, the NEW text is absent, its longest line is 77 characters, and it
contains none of "shell", "POSIX", "Windows" or "128".

**Finding 8 - addressed with a better classification than mine.** I had filed
`src/editor/widgets/PropertyMapWidget.vue` and `e2e/editor-dropdowns.spec.ts` as
scoped-but-true assertions of the same `raw:` value-comparison fact, missing from
the retained set. The plan reclassifies them as a *different claim* - about which
key strings get a dropdown, not about how values compare - and that reading is
correct. The code the comment describes is `(key === "type" || key === "codec_kind")`
at `PropertyMapWidget.vue:140`: a byte-exact comparison of the key *name*, which is
precisely why `"raw:type"` fails it and keeps its free-text cell. So they do not
belong in the retained set, and the right instrument is the new alternation-free
vocabulary sweep, which is what the plan added. Keeping them named is still
correct, for the reason the plan gives: a later sweep for "byte-exact" would
otherwise find a site this plan declared handled.

**Finding 12 - the terms are fixed; what the widening left behind is new finding N1.**

---

## Reproductions, with measured figures

### Blocking 1 - the cargo-deny mechanism, counterfactual and blast radius: **REPRODUCES in full**

| claim | plan | measured |
|---|---|---|
| `Default` sets `unsound: Scope::Workspace` | asserted | **confirmed** at `src/advisories/cfg.rs:108` of the crate I downloaded myself; the deserializer fallback repeats it at `:284` |
| `unmaintained` defaults to `Scope::All` | asserted | **confirmed**, `:107` and `:283` - the asymmetry is real and is the whole explanation |
| `Scope` is `All \| Workspace \| Transitive \| None` | asserted | **confirmed**, `src/cfg.rs:58-67` |
| the vendored copy is trustworthy | not claimed | **measured**: my `static.crates.io` download (`sha256 24bb0f6e…`) is byte-identical to `~/.cargo/registry/…/cargo-deny-0.19.9/src/{advisories/cfg.rs,cfg.rs}` |
| `unsound = "deny"` proves the key exists | `error[unexpected-value]` | **confirmed**, `expected '["all", "workspace", "transitive", "none"]'` - a value error, not unknown-key |
| run 1, shipped state (scope on + 19th ignore) | exit 0, `advisories ok` | **exit 0, `advisories ok`** |
| run 2, scope on, ignore removed | exit 1, `advisories FAILED`, `error[unsound]: Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter``, `ID: RUSTSEC-2024-0429` | **exit 1, `advisories FAILED`**, and both quoted strings appear verbatim |
| run 3, both removed | exit 0 | **exit 0** |
| blast radius: exactly one advisory, one `error[unsound]`, no other class | asserted | **confirmed, and measured one level finer than the plan states**: at `-L info`, current state = 18 `advisory-ignored`, 18 distinct ids, `0 errors, 0 warnings, 36 notes`; post-Step-4 = 19 `advisory-ignored`, 19 distinct ids, `38 notes`. The id-set difference is exactly `{RUSTSEC-2024-0429}`, and the note-class difference is exactly one `note[advisory-ignored]` plus one `note[unsound]`. **Zero collateral, at the finest granularity the tool offers.** |
| post-conditions after the two insertions | 19 ignore ids, parses as TOML with `advisories.unsound == "all"` | **confirmed** via `tomllib`: `unsound = 'all'`, 19 ids including `RUSTSEC-2024-0429`, `yanked = 'deny'` intact, `advisories` keys exactly `{ignore, unsound, yanked}`, and `licenses`/`bans`/`sources` sections intact |
| the repo's `deny.toml` is untouched | asserted | **confirmed**: `git diff --exit-code -- deny.toml` exits 0 and `git hash-object deny.toml` equals `HEAD:deny.toml` (`ab8bc5df…`) after all three runs |

Two things worth recording beyond the table. My instrument asserted, before
inserting anything, that both of the plan's anchors occur exactly once and that the
line following the scope anchor is the expected comment; both assertions passed, so
**the two fenced insertions are applicable exactly as written**. And the scope
value: `all` and `transitive` are indeed measurably identical on this tree (both
exit 1 with the same single advisory), so the decision really is about future
behaviour, and I concur with `all` on both stated grounds. One posture for both
informational classes is the right call because `unmaintained` already runs at
`all`, which I verified at the source rather than took on trust; and `Transitive`
is documented in the enum itself as "Matches external crates", so `transitive`
would exempt the workspace's own crates - the only ones this project could actually
fix. The plan states it as a decision, not an option, both in the step and in
Must-not-decide.

### Blocking 2 - the replacement control: **REPRODUCES, both figures**

| claim | plan | measured |
|---|---|---|
| pre-state count | 7 | **7** - the six repair sites plus `docs/ROADMAP.md` |
| end-state count | 1 | **1** - computed by applying all five fenced replacements to my own copies: the six repaired sites go to 0, the ROADMAP's one survives (no task edits that file) |
| the surviving hit is cited by wording, not by line number | asserted | **confirmed.** The plan quotes "`raw:` arm call the comparison an untyped byte-literal value equality"; the line reads exactly that. No line number appears, which matters: `docs/ROADMAP.md` gained over 300 lines in `9ad9e05` while this review ran, so a number would already be stale |
| the control now discriminates | implied | **measured.** Mutating one alternative (`byte-literal value equality` -> `byte-literal XXX equality`) drops the ROADMAP hit to zero, so a broken alternation is now visible - which is exactly what the old control could not do |

All five fenced OLD strings still occur exactly once in their targets (my
end-state construction asserts it and would have raised otherwise).

### New material 1 - the README example step: **REPRODUCES, including the zero delta**

The dispatch flagged the zero delta as the figure most worth checking, because it
is also what an under-derived corpus produces. I derived it over a **wider** surface
than the plan's - every tracked `.md` file, not README plus `docs/*.md` plus
`help/*/*.md` plus the v1 spec - with my own fence parser, and tagged the history
trees rather than dropping them so the exclusion could be judged instead of
inherited.

| claim | plan | measured |
|---|---|---|
| fenced `yaml`/`yml` blocks in the live surface | 6 | **6** (39 across all tracked `.md`; 33 of them in `docs/process-journal*` and `docs/superpowers/plans`, i.e. history) |
| standalone profiles / fragments | 3 / 3 | **3 / 3** |
| the three fragments | two rule-list snippets in a retired design document, one GitHub workflow | **confirmed**: `2026-07-15-plan-6-design.md:185` and `:1426` (rule-list snippets), `2026-07-22-plan8-packaging-release-design.md:1263` (a 222-line `name: release` workflow). The two snippets are in one design document and the workflow in another, which the plan's phrasing slightly compresses; the classification is right |
| profiles lacking `pattern` | exactly 1, the README's first example | **exactly 1**, `README.md:28` |
| the defect, at the binary | exit 2, ``input: The profile could not be parsed: input: missing field `pattern` at line 4 column 3``, `1 error, 0 warnings, 0 infos.` | **exit 2**, that error verbatim, that summary line verbatim |
| the other two profiles | exit 0 | **exit 0** - `README.md:79` (with the `passthrough-profile` info notice) and the v1 spec's (`Profile is valid.`) |
| the repair | `Profile is valid.`, exit 0 | **confirmed** by applying the plan's fenced line to my own extracted copy |
| the comment column | the block's three other end-of-line comments begin at column 30 and this one does too | **confirmed**: columns 30, 30, 30, 30 |
| delta against the named site | zero | **zero** |

**I tried to break the discriminator and could not.** My probe was wider than the
plan's: the plan checks for a block declaring `input:` at column 0 without
`profile_version`; I checked `input:`, `tracks:` **and** `output:` at column 0, and
got **0**, so no profile hides behind the discriminator on any of the three root
keys a profile must have. Two further probes of my own: fenced blocks with a
non-yaml info string that nonetheless contain `profile_version` exist only in
history trees, and standalone profiles inside history trees exist (8 of them) but
are correctly excluded as history. Every tracked non-`docs/` `.yaml` fixture
carries `pattern`, as the plan's third probe says.

One correction to my own instrument, recorded because it is the same defect class
this plan keeps hitting: my first pass flagged `README.md:79` as also lacking
`pattern`, because my regex was line-anchored (`^\s*pattern:`) and that example
declares it inside an inline flow mapping, `input: { pattern: '…', extensions: [mkv] }`.
The binary settled it. **My enumerated pattern was wrong and the plan's figure was
right** - which is the argument for running the binary rather than grepping, and the
plan does require the binary.

### New material 2 - the `deny.toml` boundary lift: recorded correctly

The plan records it as a reversal by authority, not as if it had always been so:
"**The `deny.toml` edit is a REVERSAL of an earlier boundary, by authority, and is
recorded as one rather than presented as if it had always been the plan.**" It names
what the boundary was, why it existed (the disposition was undecided), who lifted
it, and keeps the owner's two parts apart - interim ignore now, proper fix at 1.x.
The amendment section repeats the framing at the level of the plan's inputs.

**Nothing in the file's vicinity is taken as newly permitted.** The Files list
scopes the edit to "two named regions only … No other key, no other entry, and no
existing entry reworded or reordered"; the No-other-file paragraph keeps the
`cargo deny` invocation, the gate's part count, `BUILDING.md`, `ci.yml`,
`Cargo.lock`, `model.rs` and the schema out; acceptance row W1-k bounds the diff
rather than merely asserting it non-empty; Step 7 checks each untouched file per
blob against the base and fires the instrument against the two that did move; and
Must-not-decide names the two regions and the scope value. I verified the one
consequence that could have escaped that fence: **`scripts/ledger-lint.py` does not
read `deny.toml`** (its inputs are `BUILDING.md` and the four house YAMLs), so a
config key cannot move the gate-count invariant - the plan's "a config key changes
no command" is right, and `ledger-lint` is still green on the untouched tree with
its documented summary line.

The vehicle the deferred rows point at is real and already written: the ROADMAP's
v1.x entry for `RUSTSEC-2024-0429` carries the full context the owner asked for,
keeps the interim disposition separate from the deferral, states "Eleven direct
parents" consistently with the corrected figure, and ends in an observable trigger
(the gtk-rs generation moving past 0.18 in `Cargo.lock`, with Renovate named as the
mechanism).

### Acceptance rows 11 -> 13, and the split's honesty

Recounted mechanically: **37 rows**, `W1=13, W2=6, W3=10, W4=3, W5=5`, letters
contiguous per work item, no duplicates. Both stated totals say 37 and the stated
split matches row for row.

The W1 split is real, not cosmetic, and **no row is a "the gate is green" row in
disguise**. The five part-(b) rows separate things that fail independently and that
I exercised separately: W1-g is the mechanism at the tool's `Default` impl; W1-h is
the counterfactual (ignore removed, scope on -> fires); W1-i is the control that the
*scope* rather than the ignore does the work (both removed -> silent); W1-j is the
blast radius as a set; W1-k is the shipped green state paired with a bounded diff.
The one row that could have been a bare green row, W1-k, names the trap in its own
text - "a green `cargo deny` is also what an over-broad ignore list produces" - and
carries the bound as the half that matters. My three-way run distinguishes h, i and
k empirically, which is the test of whether the split is real: three different
configs, three different outcomes.

The one-pair amendment scale is defensible and I concur. The task set is unchanged
at A1-A4 plus B1, no task moved stream, and the coverage map still maps five work
items to five tasks. B1's Files list grew from one file to two, which is the closest
this comes to a boundary change - and it grew by an owner ruling that reshaped an
existing part's disposition, not by a re-cut of the task set.

---

## New findings

### N1. Important - the widened A4 sweep returns six hits while the plan pre-classifies three, and declares the other three findings

**Location:** Task A4 Step 4, first bullet.

The finding I raised was that `grep -nE 'xit cod|SIGINT|Ctrl'` could not see the
spec's own cancellation sentence. The term set is now correctly widened to
`'xit cod|SIGINT|Ctrl|[Cc]ancel|signal'`. The expected result was not re-derived
against the new expression. Run verbatim over the v1 spec it returns **six** lines:

- `:318` mkvmerge exit codes - pre-classified, consistent
- `:319` `- Cancellation: kill the mkvmerge process, delete the partial output file.` - pre-classified, consistent
- `:369` the 8.1 bullet this task replaces - pre-classified
- `:347` `| `executor` | process spawn, progress parse, cancellation, job states |`
- `:379` `3. **Job queue**: … cancel per job or batch.`
- `:391` `… (or Esc, except while the settings dialog is open, whose native cancel consumes Esc) exits.`

The last three match on `[Cc]ancel` and are not in the pre-classified list, while
the plan says "**A hit outside those three is a finding**". So the step as written
hands the implementer three false findings in a required verification, and the
honest responses to it - report them, or open a NEEDS_CONTEXT - both cost a
round-trip over content that is correct.

**What resolves it.** Re-run the widened expression, list all six hits with a
verdict each (the three new ones are plainly consistent: two are module/view
descriptions, one is a keyboard-handling sentence), and keep "a hit outside these
six is a finding". The fix is the re-derivation, not a narrower pattern - the
pattern is now right.

### N2. Important - a new live positional gate ordinal entered the plan, and it falsifies the plan's own gate-count audit

**Location:** the Goal paragraph: "repair the `cargo deny` configuration so **gate
part 5** and the GitHub feed agree about the unsound class".

`gate part 5` is new in this round: `git show 148f19f:<plan> | grep -c 'gate part 5'`
returns 0, the current file returns 1. Two consequences.

First, the plan's gate-count self-audit enumerates **four kinds** of hit for the
expression `'[0-9]+ parts|[0-9]+-part|(one|…|twelve)[- ]part|part [0-9]|parts [0-9]'`
and asserts "Four kinds, and every hit falls in one". This hit falls in none of
them: it is not the unrelated task-part sense, not a quotation of `BUILDING.md`,
not a reference to an ordinal as the subject being removed or surfaced, and not a
reference to `BUILDING.md`'s canonical total. The enumeration is now false, in a
self-audit whose whole purpose is that such a claim be checkable.

Second, and more pointedly, this is the construction Task A1 exists to delete. A1's
own stated reasoning applies verbatim: "once the file states a total of eleven parts
a bare `part 6` acquires a second possible referent that only section context
resolves." `cargo deny check` is the fifth command of `BUILDING.md`'s Rust block; a
part added before it rots this sentence exactly as `part 6` rotted. The plan's
Global Constraints say "this plan must not violate what it repairs" - scoped there
to line-number citations, so this is not a literal breach, but it is the same class
in the same document.

**What resolves it.** Name the part instead of numbering it - "so gate part
`cargo deny check` and the GitHub feed agree" - which costs three characters and
leaves the four-kinds audit true. If the number is kept deliberately, the audit
needs a fifth kind with its justification. Note for the controller separately: the
ROADMAP's new v1.x entry carries the same "gate part 5" phrasing, and that file is
the controller's to edit.

### N3. Minor - the no-permanent-guard deferral names its vehicle but not its observable event

**Location:** the deferred-by-decision row "No permanent guard that the `unsound`
scope STAYS on".

Deferring is right, and I want to be clear that I am not arguing the guard away:
the plan proposes no guard here, it *declines* to build one and says why (a
`deny.toml` key assertion would be new gate infrastructure, which the
tests-belong-to-the-package rule explicitly still allows deferring), and the gap is
named rather than hidden. That is the correct treatment.

The row's vehicle is "A ROADMAP line written at the plan close, phrased as an
observable trigger on the gate-coverage question rather than as a remembered
intention". That names *that there will be* a trigger, not what event fires it -
one level short of what the Triggers section requires, and the row is the only
place the requirement is recorded. The failure mode makes this worth pinning: if
someone drops the key, nothing observable happens, which is precisely why the
original defect survived two sessions.

**What resolves it.** Name the event in the row so the close transcribes rather
than invents it. The candidate that actually fires: the same
gtk-rs-generation-moves-past-0.18 event the v1.x entry already carries, since that
is when the ignore entry gets revisited and the scope key is read anyway - which
makes the guard question a rider on an event that is already watched, rather than a
new watch nobody performs.

---

## The convergence question, which I am raising instead of a third round of members

My last two rounds of findings differ mainly in scope inside one family. Round 1
found a control aimed at a presumed match (finding 2), a retention alternation
blind to two live sites (8), and a sweep whose three terms could not see its own
subject (12). Round 2 finds a widened expression whose expected result was carried
over from the narrow one (N1), an enumeration that classifies every hit and was not
re-run after the document changed (N2), and a repeated figure swept by list rather
than by search (finding 4, one site). Same defect, three widths. Per the doctrine's
convergence rule, that means the defect has stopped being the artifact's contents
and is now the rule it was derived against.

The clause I proposed last round - **a control's TARGET is itself a measurement** -
was adopted verbatim and works: it is what fixed finding 2. It is necessary and not
sufficient, because it governs the target and not the result. Two clauses close the
rest, and both have readable triggers:

1. **When you change a measuring expression, re-run it and re-derive its expected
   result from the new output.** A widened pattern's expected-hit set is not the old
   set plus your prediction. Trigger: you edited a regex, an alternation, a
   pathspec or an exclusion list that some sentence states a count or a hit list
   for.
2. **An enumeration that claims to classify every hit in a document is invalidated
   by any edit to that document, so it is re-run after the last edit.** Trigger: the
   document contains the phrase "every hit falls in one" or an equivalent, and you
   are about to add a paragraph.

**My recommendation for the next round is one mechanical sweep rather than three
point repairs**: walk every expression in the plan that has a stated expected
result - A1's two, A2's three, A3's repair/retention/vocabulary/control set, A4's
two sweeps and its `130` count, B1's `grep -c` figures - re-run each against
`fac7b50`, and reconcile the stated figure with the output. That sweep catches N1
and N2 and finding 4 together, and it is the only form of the fix that a third
party could re-derive identically. Three point repairs would leave the next
reviewer's probe deciding what is left.

---

## Harvest for the controller

Surfaced only; I wrote nothing to any house-knowledge file.

1. **The two clauses above** are the ledger-relevant output of this round. The
   first has now cost three instruments across two rounds in one plan, which is a
   recurrence pattern rather than an incident.
2. **The adopted clause worked, and that is worth an occurrence of its own.** "A
   control's target is itself a measurement" went from a reviewer harvest to a plan
   constraint to a fixed control that I then verified discriminates. That is the
   mechanism functioning end to end, and the entry that carries it earns a
   `reinforced` occurrence citing `fac7b50`.
3. **A reviewer's own instrument produced a false positive this round, and the
   binary caught it.** My line-anchored `pattern:` regex missed an inline flow
   mapping and would have reported a second defective README profile. The
   generalizable rule is the one the plan already follows: where a shipped binary
   can answer the question, grep is the hypothesis and the binary is the
   measurement. Worth recording because it cuts against the reviewer, not the
   author.
4. **"Gate part N" is regenerating in live prose while Plan 11 removes it from
   `BUILDING.md`.** Two new instances landed this round, one in the plan (N2) and
   one in the ROADMAP's v1.x entry, both controller-side. The class the ROADMAP's
   "A neighbouring class" paragraph tracks is currently scoped to `BUILDING.md`; on
   this evidence the live-prose form deserves a sentence in whatever disposition
   the plan close writes, or it will keep being re-introduced by the documents that
   discuss the fix.
5. **Positive pattern worth ledgering:** the three-way fire in B1 Step 5 - shipped
   green, ignore removed with scope on, both removed - is the cleanest instance of a
   discriminating instrument I have seen in this project. Two runs would have
   proved nothing; the third is what separates a live scope from a load-bearing
   ignore. Generalizes to any config change that can be silently inert.
6. **The amendment's own accuracy checks out**, which is worth saying because it is
   new normative text: it reports my verdict as "2 blocking, 5 important and 9
   minor", which matches exactly, and correctly records that nothing was disputed
   and nothing was recommended for removal. The plan's other self-audits also
   survive the new text unchanged - each placeholder term still returns exactly one
   line, the gate-count controls still return 4 against `BUILDING.md` and 0 against
   `renovate.jsonc`, the self-citation check still returns exactly its one synthetic
   control, and the typography is clean (verified with a fire-tested check).
