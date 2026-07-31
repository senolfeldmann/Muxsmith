# Task 3 delta verdict (Plan 12): fix round 1, commit `6904e4a`

Same reviewer as `task-3-verdict.md`, so the standards are the ones applied
there and settled non-findings are not reopened. Every number below is one I
measured on this machine; the tree was clean before and after each run, and each
mutation was followed by `pnpm build` **and** `pnpm test:e2e` before anything was
believed.

**Delta verdict: APPROVED, with one new Important finding that the fix itself
introduced.** Seven of seven dispatched and self-initiated findings are
addressed. The new finding is a one-sentence comment edit and does not put the
commit in question.

| Finding | Verdict |
|---|---|
| I-1, the false load-bearing-order claim | **ADDRESSED** (see D-1 below) |
| I-3, D107 decision 9 has no producer | **ADDRESSED** |
| M-1, the dialog's arguments are unread | **ADDRESSED** |
| M-6, the duplicated fixture name | **ADDRESSED** |
| M-3, the misattributed tier-row quote | **ADDRESSED** |
| M-4, the reflow artifact | **ADDRESSED** |
| M-5, the missing surfacing item | **ADDRESSED** |
| I-2 | out of scope by routing; both gates verified unchanged, not graded |

Baseline on the committed state: `pnpm build` exit **0**; `pnpm lint` exit **0**;
`pnpm check:i18n` exit **0**; `pnpm test:e2e` exit **0**, **79 passed** (was 78;
case 7 is the addition); `ledger-lint` exit **0**.

---

## New finding

### D-1 (Important). The replacement claim is sufficient but not necessary, and the comment's two paragraphs point in opposite directions under measurement

**Symbol:** `createBlank`'s doc comment, `src/views/EditorView.vue`.

The corrected comment says two things:

```
// ... The order below is kept for readability (clear, then establish,
// then assign) and nothing depends on it.
//
// What IS load-bearing is that the gate and the model are established in the
// SAME synchronous block. Measured: inserting an `await` between
// `model.value = ...` and `sessionActive.value = true` fails three cases ...
```

The measured half reproduces exactly. The generalization built on it does not.
Three mutations, each applied to the committed tree, rebuilt and run:

| Order in the body | `await` position | Result |
|---|---|---|
| `model`, then gate (**reversed**) | between them | **3 failed** — cases 1, 2, 3 |
| gate, then `model` (**as shipped**) | between them | **79 passed** |
| gate, then `model` (**as shipped**) | after both | **79 passed** |

The first row is the implementer's mutation C, reproduced: cases 1, 2 and 3 fail,
the queued watcher reads a still-false gate and the seed is never validated. The
second and third rows are the counterfactual the claim implies but the round did
not run. **"The gate and the model are established in the SAME synchronous
block" is not the constraint** — in the shipped order the function can be split
by an `await` anywhere and stay green. There is no synchronicity requirement at
all while the gate precedes the model.

What is actually load-bearing is the thing the paragraph above declares
inconsequential: **the relative order.** `sessionActive` must already be true
when the watcher's queued callback runs, and putting the gate first guarantees
that unconditionally, `await` or no `await`. The reversed order guarantees it
only while nothing yields in between.

So the two paragraphs combine into a trap in exactly the function Task 5 is about
to make async. A Task 5 implementer, licensed by "nothing depends on it",
reorders the two writes while adding the await the guard needs, and lands in row
one. The old comment over-constrained harmlessly; this one under-constrains the
move that matters while over-constraining one that does not.

**Handle**, one sentence, replacing the third paragraph: *the gate must be true
before the queued watcher runs, so `sessionActive` is written before
`model.value` and never after an `await` that follows it; keeping the gate first
is what makes this funnel safe to make async.* The second paragraph's "nothing
depends on it" then needs narrowing to the two writes it actually measured
(`diagnostics` position, and the swap **in a synchronous body**).

**Class, stated plainly because it is the same one the previous round produced:**
a measurement answers its own question and not the broader one erected on top of
it. Mutation C established that one configuration breaks; it could not establish
what property the working configurations share, and the comment asserts that
property as measured. The round already had the material to bound it — the
order-swap it ran as mutation A — and did not combine the two.

---

## The four questions

### 1. The new claim

**Verified rather than accepted, and it does not hold as stated** — see D-1. The
cited measurement is reproducible (3 failed, cases 1-3, on the reversed order
with an `await` between); the constraint it is offered as evidence for is
falsified by two counterfactuals I ran (both green). I-1 itself is still
**ADDRESSED**: the original false claim is gone, is explicitly named as false so
a later reader cannot reconstruct it from the code's shape, and the mechanism
(`flush: "pre"`, the callback queued rather than run at the assignment) is stated
correctly.

### 2. Case 7 and its failing evidence

**Sound on all three counts.**

- **It fails without the line.** Deleting `selectedIndex.value = 0;`: `pnpm
  build` exit 0, `pnpm test:e2e` exit 1, **exactly one failure**, case 7
  (`element(s) not found` on `editor-rule-detail`), 78 passed. That also
  re-confirms the previous round's measurement that nothing else in the
  repository observes that line.
- **It pins the index, not merely a selection.** Setting `selectedIndex.value =
  1` (out of range against a one-rule seed) also fails case 7. The
  `aria-labelledby="editor-rule-row-0"` assertion carries the index; the
  `aria-current` assertion carries the row side. The comment's own qualifier
  ("With one rule in the seed these pin index 0 from both ends") is the honest
  form — with one rule the second assertion cannot discriminate between two valid
  indices, and the comment says so rather than claiming more.
- **It touches no prescribed case's assertion list.** Case 7 is a new `test(...)`
  appended after case 6 inside the same describe; the diff adds no line inside
  cases 1-6 except M-6's rename. It reuses this describe's own `gotoEditor`
  helper and `warnReport` fixture and the pre-existing `editor-rule-detail`
  testid, so the four conditions of the precedence clause hold as the report
  states them.

### 3. The dialog-argument assertions

**Both fires discriminate, and a third member I fired myself also discriminates.**

- `defaultPath: "profile.yaml"` -> `"not-a-profile.txt"`: case 4 fails at the
  `defaultPath` assertion (`Expected: "profile.yaml" / Received:
  "not-a-profile.txt"`), 1 failed / 78 passed.
- the filter's `extensions: ["yaml", "yml"]` -> `["yaml"]`: case 4 fails at the
  `filters` assertion, 1 failed / 78 passed.
- **the filter's NAME**, which the report did not fire: pointing
  `fluent.$t("batch-profile-filter-name")` at a different catalog key also fails
  case 4 at the `filters` assertion. The `toEqual` over the whole array covers
  both members of the filter object, so the pair-fire reasoning holds one level
  deeper than the report claimed it.

The two assertions are correctly split rather than merged: the `defaultPath`
mutation leaves the `filters` assertion green and vice versa, so neither is
riding the other. The filter name is read through `en(...)` rather than a
duplicated literal, so a catalog rewording cannot desynchronize the assertion
from what the dialog shows.

### 4. New breakage, and the partial-state risk from the mid-round discard

**None found.** I checked the committed blob rather than the report's account of
it, which is the right instinct given that `git checkout --` on a file carrying
both a mutation and the round's own fixes discarded work mid-round.

- **Both re-applied edits are present and complete in `6904e4a`.** I-1's
  rewritten comment is there in full (all three paragraphs, including the "an
  earlier version of this comment claimed it was" sentence). M-4's reflow is
  there and the identifier `batch-profile-pick`/`batch-profile-current`/
  `batch-profile-filter-name` is whole on one line.
- **The fix changed no executable line of `EditorView.vue`.** Stripping comment
  lines from the file at `2cc0650` and at `6904e4a` gives byte-identical text;
  of the 39 changed lines in that file, **0** are non-comment. So the discard and
  re-apply could not have left a half-applied executable state, and the
  `createBlank` body is bit-for-bit what the previous round's verdict graded.
- **I-2's two gates are untouched**, verified with my own grep:
  `v-if="!model"` on the empty-state paragraph and `v-if="!model &&
  recents.length"` on the recents section both still read exactly what the plan
  fenced. Not graded, per the routing.
- **M-6's rename is complete and safe.** Inside the new describe the old name
  survives only in the comment explaining the rename; the recents describe's own
  `OPENED_PATH` is correctly left alone. Both fixture values still occur exactly
  once each in the file and remain distinct from every other path literal, so
  `echo-mock-distinct-fixture-values` still holds.
- **Typography clean.** No em-dash, en-dash, smart quote, Unicode ellipsis or
  non-breaking space on any added line of the fix diff. The pattern was fired
  against a known-present case first, so the empty result is a measured absence
  rather than a broken expression.
- Lint, type-check, i18n parity, ledger-lint and the full e2e suite are green on
  the committed state, and the case count moved 78 -> 79 exactly as the one
  added test predicts.

**One cosmetic residue of M-4, named for completeness and not as a finding:** the
reflow stops where the paragraph re-joins untouched text, so the line `// has two
precedents:` now runs about a third of its neighbours' width. The defect I named
(a mid-identifier break) is gone; the paragraph simply carries its ragged line one
position later.

---

## Not reopened

`M-2` (the recents gate has no producer here) and `M-7` (the `createBlank`
re-guard is untested) were recorded as a carry-forward and a consistency note
respectively, and the round's reasons for leaving them are the ones I gave.
`M-2`'s deferral gains a second ground the report names correctly: I-2's routing
may change what that gate should be, and building a producer now would pin a gate
the owner is about to rule on.

---

# Scoped delta 2: D-1 only, commit `6ca7685`

**D-1: ADDRESSED. Clean — no new breakage, and nothing else reopened.**

## Is the new sentence true, and is it the whole truth?

**True, and this time it does not outrun its evidence.** Every number the comment
states reproduces against my own runs: gate first with an `await` between gate
and model, **79 passed**; gate first with the `await` after both, **79 passed**;
model first with an `await` between them, **3 failed**, and the three are cases 1,
2 and 3 of the New describe. The claim it builds on them is now the one the
measurements actually license — the relative order, scoped by "it becomes
load-bearing exactly when the synchronicity above stops holding", which is the
precise boundary between version 2's two paragraphs and the reason they
contradicted each other.

**On "await-proof", which is the sentence with the most reach: it holds for every
placement, and I measured the one the round did not.** The comment aims the
reader at a specific future shape — "the discard guard (D109) makes it `async`
and puts an `await` in front of the seed" — and its three configurations do not
include that placement. So I ran it, plus the last remaining slot:

| `await` placement, gate before model | Result |
|---|---|
| in front of the seed (the D109 shape) | **79 passed** |
| immediately before the gate | **79 passed** |
| between gate and model (delta 1) | 79 passed |
| after the model (delta 1) | 79 passed |

Four of four slots green, against the one failing configuration that needs the
order reversed first. The reasoning under it is airtight independently of the
count: an `await` cannot reorder two statements, so the gate write always
precedes the model write in program order, and the callback that write queues can
only run at or after it. "Await-proof" is therefore not a generalization from the
measured cases — it is a property of the ordering, and the measurements now cover
every placement it quantifies over.

The rest of the comment checks out sentence by sentence: the `flush: "pre"`
paragraph is correctly narrowed to "while the body stays fully synchronous", the
D109 pointer is the right ADR (the discard guards, W4/R21-R25) and outlives the
task numbering as intended, and the two-wrong-directions record is accurate and
is the part that stops a later reader reconstructing either dead claim from the
code's shape.

*Forward note, explicitly not a finding and not measured:* making this funnel
`async` also makes concurrent entry possible, and the existing
`if (opening.value || saving.value)` guard does not cover a second `createBlank`.
Low stakes (the second seed would overwrite the first), out of scope here, and
named only because Task 5 is the task that opens it.

## Comment-only, from the blob

Confirmed from the committed object, not the report. Stripping comment lines,
`src/views/EditorView.vue` is byte-identical across `6904e4a -> 6ca7685` **and**
across `2cc0650 -> 6ca7685`, so the executable text this task shipped has not
moved since the original review. Of the 52 changed lines in the commit's own diff
for that file, **0** are non-comment, and the commit touches exactly one file.
Nothing in `createBlank`'s body, the template or the catalogs moved.

Gate on the committed state: `pnpm build` **0**, `pnpm lint` **0**,
`pnpm check:i18n` **0**, `pnpm test:e2e` **0** with **79 passed**, `ledger-lint`
**0**. No em-dash, en-dash, smart quote, Unicode ellipsis or non-breaking space on
any added line (pattern fired against a known-present case first). Tree clean at
`6ca7685` before and after every run.
