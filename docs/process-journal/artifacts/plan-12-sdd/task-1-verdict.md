# Task 1 verdict (Plan 12): the normative documents

Independent review of commit `b3816750c1842cdd2db26065adbc958c928a889f`
(`bd3aa34..b381675`), graded against `.superpowers/sdd/plan-12/task-1-review-brief.md`.

**Spec compliance: MET.** All twelve rows of the brief's consolidated requirement
set are satisfied, each verified against the artifact with an instrument built
for this review.

**Task quality: APPROVED WITH FINDINGS.** The two shipped artifacts are correct,
byte-faithful to their fences, and complete against the register. Every finding
below is either a defect in the report's *evidence* rather than in the artifact,
or an item inherited verbatim from the plan's Decision register that Task 1 had
no authority to change. No finding requires a fix round on the artifacts. Two
Important, six Minor, zero Critical.

---

## Method note

Every instrument below was written for this review, under
`/tmp/.../scratchpad/rvw-t1-peter/`, never by executing anything the implementer
wrote. Where an expression contains an enumerated set, the membership was derived
from the artifact and each member fired individually against a probe I built,
because a control fired against one present member passes while a missing member
stays invisible (`a-search-whose-terms-come-from-memory-produces-a-false-absence`).

---

## Spec compliance, row by row

| # | Verdict | How it was checked |
|---|---|---|
| T1-a | met | Both fences extracted from the brief programmatically and counted in the spec: OLD editor sentence 0 occurrences, NEW 1, appended paragraph 1. The paragraph follows `Inline validation markers from core diagnostics.` joined by a single space; the item ends at the paragraph and `2. **Batch view**` follows. |
| T1-b | met | OLD app-settings paragraph 0 occurrences, NEW 1, followed by a blank line and `First-run and startup:`. |
| T1-c | met | `git show --numstat`: 2 insertions / 2 deletions in the spec, both inside 8.2 (the editor item and the app-settings paragraph). Section 8.4 byte-unchanged. Commit touches exactly the two files in the Files list. |
| T1-d | met | H1 `# Plan 12 decisions`; five `## D1nn:` sections, D106-D110, each carrying `**Decision:**`, `**Rationale:**`, `**Rejected alternatives:**`, and `**Triggers created:**` in D107 and D110. |
| T1-e | met | See below - all 22 alternatives present, titles identical, steelmen transcribed essentially verbatim. |
| T1-f | met | D108's section opens, before `**Decision:**`, with `**This is a REVERSAL of the owner's own S22 ruling of 2026-07-22**`, the plan-7.5 kickoff, undo/redo wholesale in 1.x; the old reasoning; the new reason (change tracking is being built anyway); and D66's premise as `**CONSUMED**`, "It is not reopened here." |
| T1-g | met | The superseded controller reading closes "Recorded as superseded, not as an open option." The English-dialogs alternative is titled without an ordinal and closes "This is recorded as overruled, not as a live tradeoff; D110 carries the decision." Both sentences are additions the ADR makes over the register - the required ones. |
| T1-h | met | D110 opens with the general ruling and states explicitly that the general form is deliberate ("it is not a decision about the quit dialog"). Both residuals appear under `**What the parity check leaves uncovered, stated rather than implied.**`, and the CLI gap also as a trigger. |
| T1-i | met | Six line-number shapes swept (`<file>.<ext>:N`, `line[s] N`, `LNNNN`, bare `:N`, `row N`, `at NNN`): zero hits. Two fire tests: the same expression against `2026-07-14-plan-5.7-decisions.md` returns its `runtime.rs:206`; a six-line probe exercising every alternation member returns 6/6. |
| T1-j | met, with the reservation under Q1(b) | All three expressions re-run. E1 returns 38 lines, the identical line set the report pastes; E2 and E3 return exactly one line each (382). Both named fired controls present. No expression empty. |
| T1-k | met | `git log -1 --format='%G?'` returns `N` (unsigned). Exactly one trailer, `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`; commit body is one line. Numstat covers the two Files-list paths and nothing else. |
| T1-l | met | Thirteen-member denylist (U+2010-U+2015, U+2018/2019/201C/201D, U+2026, U+00A0, U+2212) counted over both touched files: clean. Each member fired individually against a probe: 13/13. The ADR file contains no non-ASCII character at all; the spec's non-ASCII are pre-existing and legitimate (`Türkçe` in the reference example, box-drawing glyphs in the architecture tree). |

### T1-e in detail

Register and ADR blocks were extracted independently and their bullets compared
title by title:

```
D106: register=4 adr=4      D107: register=5 adr=5      D108: register=5 adr=5
D109: register=5 adr=5      D110: register=3 adr=3      TOTAL 22 = 22
```

Every title is byte-identical after normalising `*` and backticks, and in the
same order. Control against my own instrument's blind spot: the extractor flags
any non-bold-led bullet, and the raw blocks were dumped and checked for
non-bullet content - zero non-bullet non-empty lines in either document, in all
ten blocks, so the bullet enumeration is the whole set.

Body text: a per-bullet sequence diff shows the transcription is faithful. The
only substantive deltas are three, all correct:

- D107's Save-as bullet drops "Recorded as a candidate for later disposition."
  from the bullet - relocated into `**Triggers created:**`, which is where it belongs.
- D109's superseded-reading bullet gains "Recorded as superseded, not as an open
  option." and de-ordinalises "(the ROADMAP's controller reading 2)" into "the
  controller's reading recorded in the ROADMAP".
- D109's English-dialogs bullet gains "This is recorded as overruled, not as a
  live tradeoff;".

A set-difference over every backticked identifier in each decision section found
no technical content in the ADR that is absent from the register. The four
identifiers that differ are all rewordings, not inventions - notably
`tracks[0].match` becoming "the seeded rule's `match`", which removes an index
ordinal and loses nothing, there being exactly one seeded rule.

---

## Findings

### Important

**I-1. Two absence-shaped verifications are narrated rather than pasted.**
Location: report, Step 3, the ordinal-sweep paragraph under fixed property 2, and
the `**Typography.**` paragraph.

The report describes the ordinal instrument as "an expression pairing every
ordinal word against `rejected|alternative|bullet|option|item` in both
directions" and reports "It returned two hits" - without the expression and
without its output. The typography scan is described as "a character-class scan
covering the whole denylist (U+2010-U+2015, curly single and double quotes,
U+2026, U+00A0, U+2212)" and its pasted evidence is the number `13` with a note.
Neither can be re-run by a reviewer; both can only be re-derived, which is
precisely what `design-empirical-claims-reproducible` forbids - its statement
names the exact shape: "Narrating a check instead of pasting it is where this
entry's violations cluster." Plan 12's Global Constraints restate it
("Evidence lines carry pasted output").

Consequence: none on the artifacts. I re-derived both from scratch and both
conclusions hold (typography clean, 13/13 fire; no list ordinal). The defect is
that the report's conclusions survive while their evidence does not travel - the
pure form the ledger's 2026-07-30 occurrence of this entry describes.

Contrast worth recording: the three expressions the *brief* fenced were pasted
with full output, faithfully (I reproduced E1's 38-line set exactly). The
verifications the implementer chose himself are where the pasting discipline
lapsed. See harvest H3.

**I-2. Step 4's bolded claim about the two advance-named hits contradicts Step 4's
own Expression 1 classification.**
Location: report, Step 4, "The two advance-named hits".

The report states, in bold: "**Neither appeared in the output of any of the three
expressions**". Measured over lines 442-458 of the spec (section 11), the
`locale|language` expression returns two lines - the `Locales beyond English and
German` non-goal bullet and its continuation. Those two lines are in the pasted
Expression 1 output, and the report's own classification list for Expression 1
carries the entry "Section 11's non-goal `Locales beyond English and German`:
**consistent**." The same holds one step weaker for section 8.3, whose
help-mechanics sentence appears in Expression 1's output, though its Escape
sentence does not.

The narrow claims in the same sentence are both true and I confirmed them
(section 8.3's Escape sentence: 0 hits for `undo|redo|unsaved|discard|confirm`;
section 11: 0 hits for `create|new profile|open/save`). It is the unqualified
generalisation that is wrong, and it is wrong inside the step whose purpose is a
self-contradiction sweep. It also makes the report's surfaced item 1 half
spurious: the brief's advance-naming of section 11 *was* satisfied, by
Expression 1.

### Minor

**M-1. "The one place an item ordinal survives" is false.**
Location: report, Step 3, the "No line-number citations" paragraph's final clause.

Measured over the ADR file: 39 `decision <n>` references on 11 distinct lines,
including one inside a rejected-alternatives bullet - D106's fourth alternative
closes "Rejected on decision 3's reasoning" - which is structurally the same
shape as the "decision 5's table" reference the report names as the sole
survivor.

The artifact is not defective here. The house form numbers its decisions and its
`**Rationale:**` slot must point at them, so within-record ordinals are
unavoidable, and the register does the same. What is defective is a recalled
observation presented as a sweep result, and the reason it survived is the
instrument: an expression enumerating ordinal *words* cannot see the numeric
form, and firing it against `docs/decision-ledger.yaml` proves the word pattern
valid, not the enumeration complete
(`a-search-whose-terms-come-from-memory-produces-a-false-absence`: "A measuring
expression carries TWO enumerations and both are claims").

**M-2. Two counts sit in normative position in the ADR without their enumeration.**
Location: `2026-07-30-plan-12-decisions.md`, D110 decision 5 ("this package's six
shell ids") and D108 decision 1 ("measured: seven whole-value assignments").

Neither forces a later reader to invent anything - Task 6's Files list and its
fenced catalog blocks carry all six ids by name, and Task 4's `Read first` names
the plan's mutation-path enumeration - so this is not a latitude clause under the
brief's test. It is flagged because the ADR is the durable artifact and the plan
is retired: a count whose set lives only in a plan document goes stale silently
(`proc-normative-count-recomputed`). I did verify the seven: `grep -c 'model\.value\s*='`
over `EditorView.vue` returns 8, one of which is a comment quoting the pattern -
so the claim is correct, and my own first instrument had the blind spot the check
exists to expose.

**M-3. D108's "hand-set dirty boolean" steelman is the weakest of the 22 and omits
its strongest available argument.**
Location: `2026-07-30-plan-12-decisions.md`, D108, rejected alternatives.

As written the steelman is "each mechanism is then simple in isolation, and the
boolean is trivially cheap." The argument it does not make: D108 decision 1
serialises the whole model with `JSON.stringify` on every model change, and
decision 2 deliberately keeps text widgets on their per-keystroke `v-model`
binding - so the chosen design pays a full-model serialisation and string
comparison per keystroke, which a boolean does not. D108's Rationale addresses
memory (decision 5, with measured entry sizes) and never CPU or input latency,
and no other alternative raises it either.

Under T1-e's bar - "each with its steelman stated at its strongest ... A
caricatured rejection is a defect" - this one is understated. Inherited verbatim
from the register, which Task 1 must not re-open, so it is not an authorship
defect. Surfaced for the controller, since the cost is real and currently
unaddressed anywhere in the package.

**M-4. The amended spec's "in one prompt" carries no per-state qualifier, while
D109 decision 9 mandates a conditional second prompt.**
Location: spec 8.2, editor item, final sentence; `2026-07-30-plan-12-decisions.md`,
D109 decision 9.

The spec sentence reads "closing the app with unsaved changes warns as well, in
one prompt that also covers a running batch when both hold." D109 decision 9
mandates a re-read after the confirm and a second prompt when the state
strengthened. The reconciliation - one prompt *per state*, not one prompt per
close - exists in D109's own prose and in the ROADMAP's controller correction
("B does not overrule decision 5, it extends its own principle of one prompt per
state"), and nowhere in the spec sentence that Task 6 reads as ground truth under
`proc-04-spec-wins`.

This is inside the fenced replacement Task 1 must not decide, and Task 6's Step 2b
names decision 9 explicitly, so nothing is at risk today. Routed to the controller
because the spec is the document that outlives both.

**M-5. The spec's seed enumeration omits `input.pattern`, which D107's fenced seed
carries.**
Location: spec 8.2, "the seed carries the format version, one candidate extension
and one empty track rule"; D107 decision 1.

D107's seed is `{ profile_version: 1, input: { pattern: ".*", extensions: ["mkv"] },
tracks: { rules: [{ match: {} }] } }`, and its Rationale justifies `pattern: ".*"`
over `""` deliberately. The spec's three-element summary does not cover it. A
reader taking the spec's enumeration as exhaustive would seed without a pattern.
Fenced text, not Task 1's to decide; recorded so the divergence is visible.

**M-6. D110's "every current call site is a literal, measured" is false as stated.**
Location: `2026-07-30-plan-12-decisions.md`, D110, "What the parity check leaves
uncovered".

`src-tauri/src/run.rs` contains `let value = ftl_message(key);` inside its
`#[cfg(test)]` module (test module opens well above it), iterating a literal
four-key array. That is a non-literal `ftl_message` argument in the source today.

Consequence is nil: all four keys it exercises also appear at literal production
call sites, so part (b)'s derived key set is unaffected. It matters only because
Task 6 builds that regex against exactly this sentence and will meet a
counter-example to it. Inherited verbatim from the register.

---

## The no-work-needed checks

Every passage concluding that a guard, enumeration or check is unnecessary was
run rather than weighed. All held.

| Claim | Result |
|---|---|
| D106 d8: no handling/migration/test for an out-of-band locale, because "`buildBundles` falls through to English" | Holds. `buildBundles` maps `[locale, "en"]` through `primarySubtag` and builds both. |
| D106 d1: the `""` sentinel is the house pattern of the sibling nullable field | Holds, exactly as quoted: `form.mkvmergePath = baseline.mkvmerge_path ?? "";` and `mkvmerge_path: form.mkvmergePath.trim() === "" ? null : ...`. |
| D107 d2: the README's guess boundary is about inferring intent from a file | Holds. README: "No language-from-filename, no auto-title", reason given as an unattended batch applying a wrong guess 400 times. |
| D107 d1: the README's example uses the same extension list | Holds, `extensions: [mkv]` at two sites. |
| D107 d5: mkvtoolnix's `Tab::onSaveConfig` delegates to `onSaveConfigAs` on an empty filename | Holds, verified in `~/Downloads/mkvtoolnix/src/mkvtoolnix-gui/merge/tab.cpp`: `if (p.config.m_configFileName.isEmpty()) { onSaveConfigAs(); return; }`. |
| D108 d1: seven whole-value assignments, no in-place mutation, no external writer | Holds (see M-2). |
| D108 d6: help mode's capture-phase handler takes only Escape and Enter/Space on a help target | Holds. `onHelpKeydown` has exactly those two branches. |
| D109 d1: the recents-click path needs no guard, the section being gated | Holds today as `v-if="!currentPath && recents.length"`; D107 3f moves it to `!model`, as the register states. |
| D109 d3: nothing is built for tab switching, because `v-show` mounting plus an existing test already cover it | Holds. `App.vue` mounts all three views with `v-show` under a comment saying so; `e2e/smoke.spec.ts` carries "the editor tab stays mounted across a switch to Jobs and back (v-show, not v-if)", asserting both the field value and the open path after the round trip. |
| D109 d6: the native `<dialog>` + `showModal()` house pattern (D27) | Holds in `SettingsDialog.vue`. |
| D110 d1: the CLI's hand-written `include_str!` table is the shape being followed | Holds. `crates/muxsmith-cli/src/i18n.rs` carries `const LOCALES: &[(&str, &str, &str)]` over four `include_str!` constants. Arity differs (the shell reads one catalog, the CLI two); the shape is the same. |
| D110 4(c): the pinned German value | Holds byte-exact: `locales/de/gui-common.ftl` carries `close-abort-title = Laufende Jobs abbrechen`. |
| D110, steelman: "`sys-locale` is already in the workspace's tree" | Holds. `crates/muxsmith-cli/Cargo.toml`: `sys-locale = "0.3.2"`, one lockfile entry. |
| D110 4(a): `locales/` versus the table | The directory set is exactly `de`, `en`. |

## Latitude, both forms

Explicit-permission form: an eighteen-member expression (`either approach`,
`whichever`, `as appropriate`, `to taste`, `TBD`, `etc.`, `such as`, `e.g.`, and
eleven more) returns zero hits over the ADR file; the same expression fired
18/18 against a probe carrying every member, so the empty result is a measurement
rather than a malformed pattern.

Omission form, judged by reading every normative sentence of both artifacts
against "must a later reader invent something they are not allowed to invent?":
no. The two counts under M-2 are the only unenumerated sets, and both are closed
downstream by documents their consumers read. The spec's coarser formulations
("every list or map widget mutation", "a grid operation", "one candidate
extension") are each closed by a fenced or enumerated ADR decision - the widget
set mechanically, by D108's single mutation funnel; the grid operations by
D108 decision 2's three named functions; the extension by D107's fenced seed.

## D-number collision sweep

Independently re-measured. Assignment-shaped occurrences (heading or leading bold)
of D106-D110 exist in exactly two files: the plan's Decision register and this
task's ADR file. Every other occurrence in the tracked tree is a reservation
reference or a forward citation - including
`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`, which states
"**D-number collision check, measured:** `D106`-`D110` are reserved by Plan 12".
The same assignment pattern fired against D111 returns Plan 11's three sites, so
the pattern is valid. Highest D-number in `docs/` is 111. No collision.

---

## Adjudication verdicts

### Q1(a) - the factual claim: PARTLY CORRECT.

The two narrow claims are correct and reproduce. Over spec lines 390-411
(section 8.3), `undo|redo|unsaved|discard|confirm` returns 0; over lines 442-458
(section 11), `create|new profile|open/save` returns 0. Section 8.3's help-mode
Escape sentence matches none of the three expressions at all.

The bolded generalisation is incorrect. Section 11's non-goals return 2 lines
under `locale|language`, they are in the pasted Expression 1 output, and the
report classifies them there. Section 8.3 likewise surfaces under Expression 1
via its help-mechanics sentence. Graded as I-2.

### Q1(b) - the sweep's coverage: NOT ADEQUATE. The sweep has a blind spot.

The three expressions carry the vocabulary of locale/language, of
undo/redo/unsaved/discard/confirm, and of create/new-profile/`open/save`. The
amendment additionally asserts things about *saving*, *closing the app*,
*keyboard* bindings, *switching views*, warning-versus-error *severity*, and a
running *batch* - and none of those words appears in any expression. Measured
regions the sweep cannot see, each carrying a live constraint on the amendment:

1. **Section 7's frontend bullet**, "Frontend-side logic is limited to UX
   affordances (e.g. disabling Save while errors exist)". This is the sentence
   the amendment's "never by an error that would disable Save" must not
   contradict, and it is the single most load-bearing pre-existing constraint on
   the new text. `save` matches no expression; `create|new profile|open/save`
   matches only the literal compound, and `save` appears on just two lines of the
   whole spec.
2. **Section 8.3's help-mode suppression bullet**, "keyboard and text-entry
   channels (typing into fields, select changes via keyboard) stay deliberately
   live" - the keyboard channel D108's undo/redo binding shares. No expression
   contains `keyboard`, `key`, `Esc` or `shortcut`. This is also the mechanical
   reason the brief's own advance-named 8.3 hit could never materialise: the
   brief expected the sweep to reach a keyboard passage with no keyboard term in
   any pattern.
3. **Section 8.3's view-switch clause**, "the active view is switched, in which
   case the hover state resets too", against the amendment's "switching views
   never touches it". `switch` matches no expression.
4. **The batch/cancel/exit region** (8.1's exit codes including 130 for a
   cancelled batch, section 6's cancellation bullet, 8.2's job-queue item),
   against "one prompt that also covers a running batch when both hold". None of
   `batch`, `cancel`, `abort`, `close`, `quit` is in any expression - and `batch`
   alone occurs on 22 lines.

Structurally there is a fifth, and it is the one that actually cost something:
**the sweep's surface is the spec file only.** Task 1 produced a matched pair of
normative documents in one commit, and the sweep looks at one of them. The single
genuine tension I found (M-4, "in one prompt" against D109 decision 9) lives
across the pair, where no spec-internal expression could ever reach it.

Stated honestly, and it matters for the grade: I read every one of those regions
and all are in fact consistent with the amendment, so the blind spot cost no
defect this round. `proc-sweep-surface-completeness` names exactly this
distinction - "A firing positive control proves a sweep PATTERN is valid, never
that its SEARCH SURFACE is complete" - and the fired controls the brief
prescribed proved the patterns, which was all they could do. The finding is
against the instrument's design, which is a controller matter (the expressions
were prescribed, not chosen), not against the implementer's execution of it.

### Q2 - section 8.4 and the shell: COMPATIBLE AS WRITTEN. Correctly a routing matter, not NEEDS_CONTEXT.

8.4's statement reads: "v1 ships English and German content on both surfaces -
GUI catalogs and help topics, and the CLI's embedded catalogs
(`cli-multilang-rendering`, D63)". The **two surfaces** are the GUI and the CLI.
The Tauri shell is the GUI's own native layer and its dialog strings are GUI
user-visible strings, so D110 does not add a fourth *surface* - it brings a part
of the GUI surface into the conformance the statement already demands. Three
independent supports:

- 8.4's first bullet forbids hardcoded user-facing strings "in any layer" and
  its accepted-exception list does not name shell dialog text, so the shell's
  present en-only rendering is a defect *against* 8.4 that D110 repairs.
- 8.4's one-catalog-source-of-truth bullet is satisfied, not strained: D110 has
  the shell read `gui-common.ftl` out of the same `locales/` tree rather than
  introducing a second catalog.
- 8.4's locale-selection bullet ("system locale with manual override in app
  settings ... falls back to English per message") is exactly the semantics D110
  decision 2 and D106 implement.

There is therefore nothing to refute and nothing to return. The implementer's
disposition - surface it, do not edit 8.4, do not raise NEEDS_CONTEXT - is
correct, and DONE_WITH_CONCERNS is the right status for it.

**Rider, and it is the part that needs routing.** The clause after the dash is an
enumeration of *embed sites*, and after Task 6 there will be three: the
frontend's `@fluent/bundle` load, the CLI's `LOCALES` table, and the shell's
`LOCALES` table. The enumeration goes stale when Task 6 lands, not now
(`proc-normative-count-recomputed`). Two notes for whoever routes it:

- D110's own part (a) does **not** cover this. It fires on a new locale
  *directory* without a table row, never on a new *embed site* without a mention
  in 8.4.
- The only trigger for it today is prose in a task report, which is the class of
  trigger that dies. Recommend attaching it to Task 7 (the documentation task,
  which is already editing user-facing prose) or to the plan close, rather than
  leaving it in a report.

### Q3 - the `Triggers created` omission: THE IMPLEMENTER'S PRECEDENCE IS CORRECT. No change.

Two independent grounds, the second stronger than the one the implementer gave.

**(i) The brief's own wording.** Step 3 enumerates the slots and qualifies the
last: "and, **where one exists**, `Triggers created`". An explicit enumeration in
the brief beats a follow-the-local-pattern grant, which is what the implementer
argued and which stands on its own.

**(ii) `**Triggers created:** none.` is not the house form - it is a three-file
habit, and the brief's exemplar is a one-section sample.** Measured over the
eleven `## D`-sectioned decision files in `docs/superpowers/specs/`:

```
file                                     D-secs  Dec  Rat  Rej  Trig
2026-07-09-plan-2-design-decisions.md         6    5    0    0     0
2026-07-09-plan-3-design-decisions.md         6    6    0    0     0
2026-07-09-plan-3.5-design-decisions.md       3    3    0    0     0
2026-07-09-plan-4-design-decisions.md         6    5    0    0     0
2026-07-10-plan-5-gui-design-decisions.md    10   10    0    0     0
2026-07-11-plan-5.5-design-decisions.md       3    0    2    2     0
2026-07-11-pre-1.0-design-decisions.md        2    2    2    2     2
2026-07-13-plan-5.6-decisions.md              1    0    1    1     0
2026-07-14-plan-5.7-decisions.md              1    1    1    1     1
2026-07-14-plan-5.8-decisions.md              3    3    3    3     3
2026-07-30-plan-12-decisions.md               5    5    5    5     2

TOTAL D-sections: 46; Triggers-created slots: 8
excluding plan-12: 41 sections, 6 slots
```

Thirty-five of the 41 pre-existing decision sections omit the slot entirely. The
files that write it every time are plan-5.7 (one section), plan-5.8 (three) and
pre-1.0 (two). The brief named plan-5.7 as "the ADR house form this file
follows", and plan-5.7 contains exactly one decision - so the exemplar cannot
distinguish "the form always writes the slot" from "this one decision had a
trigger to write". Derived from the corpus rather than the exemplar, omitting the
slot when nothing exists is the dominant house behaviour by a wide margin, and
this file's four-slot core (Decision / Rationale / Rejected alternatives, plus
Triggers where one exists) matches the most recent form the corpus uses.

The concern behind the question - "an absent slot and an unconsidered slot look
identical to a later reader" - is a real one in general. Here it is already
answered: the report names the omission, the rule it followed and the pattern it
diverged from, as its surfaced item 3. Nothing further is owed.

---

## Harvest

Surfaced for the controller; nothing written to the ledger.

**H1. A single named exemplar is not a house form; the corpus is.** The brief
pointed at one file as "the ADR house form this file follows", and that file
carries one decision. Measuring the corpus reversed the reading (Q3). Candidate
handle, reinforcing `proc-sweep-surface-completeness` one level up: when a brief
or a review cites one exemplar as the house form, the *surface* of that claim is
the corpus of comparable artifacts, not the cited file - and the check is a count
across the corpus, which is cheap.

**H2. A self-contradiction sweep over one document cannot see the document it was
written against.** Task 1 produced a matched normative pair in one commit and
swept one member of it. The only real tension in the change (M-4) lives across
the pair. Candidate handle: where a task ships two normative documents in one
change, the sweep's surface is both, and the cross-document pass is the one that
finds the tensions the intra-document pass is structurally blind to. Reinforces
`proc-sweep-surface-completeness`.

**H3. The pasting discipline decays exactly where the brief stops prescribing.**
The three expressions the brief fenced were pasted with full output and reproduce
byte-exact. The four verifications the implementer chose himself vary from fully
reproducible (the line-number pattern, with its fire test pasted) to narrated
only (the ordinal sweep). Both instances of I-1 are in the self-chosen set.
Candidate refinement to `design-empirical-claims-reproducible`: the trigger is
not "you are writing an absence claim" alone but "you are writing an absence
claim the brief did not ask you for" - a prescribed check inherits its evidence
form from the dispatch, a volunteered one has to supply its own, and that is
where the entry's violations concentrate.

**H4. A repeated rejection ground, three instances in one plan.** D106's
"Duplicate the resolution rule in the dialog rather than extracting a seam",
D110's "Let the shell resolve independently" and D110's "A check written in
`check-i18n.mjs` instead of Rust" are all rejected on one ground: *it would be a
second (or third) implementation of a rule that already exists; the seam is the
answer*. Three instances of one ground inside a single package is a candidate
house pattern. Whether an existing entry already covers it is a check for the
controller before anything is promoted.

---

## What I could not verify, and why

- **Ten of the eleven gate parts.** I re-ran only `python3 scripts/ledger-lint.py`,
  which returned `ledger-lint: 566 entries across 4 files plus BUILDING.md's gate
  enumeration, all invariants hold`, exit 0 - byte-identical to the line the
  report pastes, and the part that also enforces the gate-count invariant the
  report's "11 parts" claim rests on. The other ten I did not re-run: the change
  touches only `docs/superpowers/specs/`, no other gate part reads that path, and
  the working tree is clean at `b381675`. Recorded as not re-verified rather than
  as verified.
- **Whether the commit trailer's model name matches the dispatch's model
  parameter.** SI-4 requires the name be derived from that parameter; the
  parameter is not an artifact in the tree. I verified what is checkable: the
  commit is unsigned (`%G?` = `N`), carries exactly one trailer, and its body is
  one line.
- **D107's seed measurement** ("exactly one diagnostic, `empty-match-expression`
  at warning severity"; "`pattern: \".*\"` and `\"\"` are both diagnostic-free").
  Reproducing it requires running the validator against the seed. It is an
  authoring-section measurement that Task 3 will exercise directly, not a Task 1
  deliverable.
- **Two browser-runtime measurements carried into D108** (`focusout` bubbles
  where `blur` does not; Playwright's `fill()` dispatches `change` as well as
  `input`). Both are standard and neither is load-bearing for Task 1's output;
  not re-run.
