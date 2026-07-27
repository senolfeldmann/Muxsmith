# Plan-7.5 close fix: implementer report

Brief: `.superpowers/sdd/plan-7.5/close-fix-brief.md`. Executed on `master` in
the main worktree at `/home/senol/Git/Muxsmith`. Base commit `8e2c044`
("salvage: plan-7.5 SDD archive (31 files, plan close)").

**Commit `d5a6470`**, unsigned (`git log -1 --format='%G?'` -> `N`), not pushed
(`master...origin/master [ahead 2]`, the other being the salvage commit).

Both specified edits applied unchanged. Two brief premises did not hold and are
reported below rather than resolved at the keyboard; neither blocked the edits,
and per the brief's own instruction ("report them rather than editing them")
the extra site found was left untouched.

## What changed: four regions, two files

| # | File | Final lines | Change |
|---|------|-------------|--------|
| 1 | `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md` | 807-812 | supersession note after the amendment-1 mandate block |
| 2 | `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md` | 843 | citation prefix re-pointed (`design-review-round-1.md`) |
| 3 | `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md` | 1127 | citation prefix re-pointed (`task-2-verdict.md`) |
| 4 | `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md` | 372 | supersession note after the Task-4 transcription target |

`git diff --stat`: 2 files changed, 11 insertions(+), 2 deletions(-).

Edit 1 changed only the directory prefix; file names, section names and the
surrounding line text are byte-identical (the `:line`-suffix case did not occur
at either site). Edit 2's note is a plain paragraph in the document's own voice,
not a blockquote continuation, so it cannot be misread as part of the quoted
mandate: 3-space-indented and hard-wrapped at ~70 columns in the design (that
file's list-item prose convention), one long line in the plan (that file's
convention). Wording is the brief's text verbatim at both sites, naming the
ruling commit `406e91b` ("help+spec: owner-ruled wording fix (plan-7.5 owner
pass)") as the supersession's cause.

## Verification

**Fire-verified counts (design file).** Each old pattern was run BEFORE the edit
and returned its expected non-zero count, so the post-edit zero is a real
absence and not a malformed pattern. The new patterns need no separate control:
a non-zero hit self-validates the pattern.

| Pattern (`grep -c -F`) | pre-edit | post-edit |
|---|---|---|
| `.superpowers/sdd/plan-7.5/design-review-round-1.md` | 1 (fired) | 0 |
| `.superpowers/sdd/plan-7.5/task-2-verdict.md` | 1 (fired) | 0 |
| `docs/process-journal/artifacts/plan-7.5-sdd/design-review-round-1.md` | 0 | 1 |
| `docs/process-journal/artifacts/plan-7.5-sdd/task-2-verdict.md` | 0 | 1 |

**Quoted mandate blocks byte-unchanged.** Two independent proofs:

1. The whole diff for both files contains exactly two removal lines, and both
   are citation lines far outside either block (design 836 and 1120 pre-edit):

   ```
   -   (`.superpowers/sdd/plan-7.5/design-review-round-1.md` HARVEST),
   -- **Evidence.** `.superpowers/sdd/plan-7.5/task-2-verdict.md` - Q1,
   ```

2. `diff` of the block regions between `HEAD` and the worktree is empty for
   both (design 794-805, plan 359-370). Fire control for that method: the same
   `diff` invocation on the design's re-pointed citation line (HEAD:836 vs
   worktree:843) reported the change, so an empty result means identity rather
   than a broken comparison.

**Typography.** No non-ASCII character occurs in any added line; the detector
was fired against a known non-ASCII control string to confirm it works.

**Salvage targets exist.** Both re-pointed files are tracked in the salvage
directory (`git ls-files docs/process-journal/artifacts/plan-7.5-sdd/`, 31
files), so the new citations resolve for a reader of the public repo.

**Precedent claim checked.** Commit `9d01862` (plan-7) touches exactly one file,
the plan-7 design, with 3 insertions / 3 deletions and preserves a `:60,64`
suffix. The brief's characterisation of the precedent is accurate, and this
change follows it: design re-pointed, plan document's tracker-location mentions
left alone.

## Findings: two brief premises that did not hold

### Finding 1: the enumeration of tracked citation sites is off by one

The brief predicts the tracked-file search for `.superpowers/sdd/plan-7.5`
(excluding the salvage directory) returns the two design sites, the plan's two
tracker-location mentions, the ROADMAP trigger entry and the two handoff
snapshots. That is seven sites. `git grep` returns **eight**. The unlisted one:

```
docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md:413
```

the plan's close-actions "Salvage re-pointing" bullet, which cites
`.superpowers/sdd/plan-7.5/design-review-round-1.md` while registering the very
trigger this change discharges. It is a third mention in the plan document and
not a `progress.md` tracker-location mention, so the brief's phrasing does not
cover it.

**Left untouched**, per the brief's instruction to report rather than edit. That
is also the substantively consistent outcome: this bullet is the same class of
artifact as the ROADMAP trigger entry at `docs/ROADMAP.md:296`, which the brief
does enumerate and does not re-point. Both are historical records of what the
pre-registered trigger said at registration time, not live pointers a reader is
expected to follow. Whether the controller wants either of them annotated is a
question for the close, not for the implementer.

### Finding 2: the diff spans two files, not the three the brief predicts

Verification item 1 says "exactly four changed regions across three files: two
citation prefixes in the design, one supersession note in the design, one in the
plan". The enumeration names two files (design, plan) and yields four regions;
"three files" is inconsistent with it. The v1 spec is explicitly out of scope, so
there is no third file to change. The observed diff matches the enumeration:
four regions, two files. Recorded because it is a count in a verification
clause, not because anything is missing.

## Scope respected

No code, no ROADMAP, no journal, no house-knowledge YAML, nothing under
`.superpowers/` except this report. Staging was explicit (two paths), the commit
unsigned, no push.

**Working tree carried the controller's changes throughout; none were staged.**
`docs/conventions.yaml` and `docs/decision-ledger.yaml` were already modified at
session start. `docs/ROADMAP.md` was clean at session start and appeared as
modified mid-session, carrying plan-7.5 close content (executed/closed header,
five new trigger entries). It was written at 18:16:01 while this task's two
edits landed at 18:17:42 and 18:17:49, so a concurrent writer produced it, not
this task. All three remain unstaged and untouched. Anyone committing them owns
that diff separately.

---

# Addendum: German help cross-reference alignment

Controller addendum received mid-task and authenticated through the task
channel; owner-ruled 2026-07-27, riding this same close. Both refutations above
were accepted and their dispositions recorded: the eighth site stays as-is
because it records what the pre-registered trigger said, and "two files" is
correct.

**Commit `ddb707a`**, unsigned (`%G?` -> `N`), not pushed. Kept as a SEPARATE commit rather
than an amend of `d5a6470`: the two changes answer different rulings on
different surfaces (internal plan-close documentation hygiene vs the shipped
German help text), and `d5a6470`'s message is accurate and complete for its own
scope. Folding them together would produce one message covering two unrelated
surfaces and would cost the traceability from each ruling to its own change.
Both commits are unpushed, so this is still reversible if the controller
prefers a squash.

## What changed: two regions, two files

| File | Final line | Change |
|---|---|---|
| `help/de/view-batch.md` | 16 | `siehe das Thema zur Vorschlagskarte` -> `siehe das Thema Vorschlagskarte` |
| `help/de/view-editor.md` | 17 | same |

`git diff --stat`: 2 files changed, 2 insertions(+), 2 deletions(-) - one line
each. A `--word-diff` shows `[-zur-]` as the only token touched on either line;
nothing was added.

## Verification

**Target title checked before editing, as instructed.** `help/de/batch-suggestion-card.md`
line 1 is `# Vorschlagskarte`, so the replacement text equals the real topic
title.

**Fire-verified counts** (`git grep -c -F`, scope `help/`):

| Pattern | pre-edit | post-edit |
|---|---|---|
| `siehe das Thema zur Vorschlagskarte` | 2 (fired) | 0 |
| `Thema zur` (broader net) | 2 (fired) | 0 |
| `siehe das Thema Vorschlagskarte` | 0 | 2 |

The old pattern returned exactly 2 before the edit, so the post-edit 0 is a real
absence rather than a malformed pattern.

**No collateral damage.** Only the two German help files appear in `git diff`
besides the controller's own three. No double space was introduced
(`Thema  Vorschlagskarte` counts 0, and that pattern was fired against a
synthetic defect string to prove it matches when the defect exists). Both
edited lines still carry their German letters intact (`zählt`, `über`,
`Schlägt`, `Schaltfläche`): the diff's four content lines all still match a
non-ASCII detector.

**House form confirmed on master**, German (13 sites): `siehe das Thema Quelle`,
`... Vorlage`, `... Optional`, `... Exakt`, `... Match`, `... Match-Muster`,
`... Editor-Ansicht`, `... Nicht zugeordnet (Spuren)`, `... Vorschlagskarte`.
Bare topic title, no preposition. The two edited sites now match it.

*Corrected after review:* this list first rendered the eighth title as
`... Nicht zugeordnet`, truncated at its opening parenthesis by the same
defective pattern described under finding 4 below. The real title is
`Nicht zugeordnet (Spuren)`, matching its h1. Only the German display was
affected, not the German count, because that pattern had no required
terminator after the title and so still matched the line.

## Finding 3: the English counterparts are not house-conformant either

The addendum's scope boundary rests on the premise that the English
counterparts "are ALREADY house-conformant and are NOT touched". The
not-touched part was honoured; the premise behind it does not hold, so it is
reported rather than acted on.

English cross-references take the form `see the <Title> topic`, and every other
site capitalizes the title as the topic's own h1 spells it:

```
2  see the Source topic          2  see the Match pattern topic
2  see the Editor view topic     1  see the Template topic
1  see the Optional topic        1  see the Match topic
1  see the Filename topic        1  see the Exact topic
1  see the Unmatched (tracks) topic
```

The two suggestion-card sites read `see the suggestion card topic`, lowercase,
in `help/en/view-batch.md:16` and `help/en/view-editor.md:17` - while the topic's
own h1 at `help/en/batch-suggestion-card.md:1` is `# Suggestion card`. So the
English pair deviates from the same house form the German pair was just aligned
to, by casing instead of by an inserted preposition, and it is the same two
documents and the same cross-reference.

Not edited: the boundary was explicit, the owner ruled only the German sites,
and the English fix is a wording change to shipped text that wants its own
ruling. Flagged because the two deviations are one defect wearing two
localizations, and fixing only the German half leaves the surface
half-aligned - which is exactly the kind of residue this close is clearing.

---

# Addendum 2: English help cross-reference alignment

Second controller addendum, same authenticated channel; owner-ruled 2026-07-27
after the controller independently re-measured finding 3 rather than relaying
it. Finding 3 confirmed and ruled: align the English pair too.

**Commit `eb4608b`**, unsigned (`%G?` -> `N`), not pushed. Third commit, keeping
the surface split argued for in addendum 1 - shipped English help text is its
own surface, distinct from the German help text (`ddb707a`) and from the
internal plan-close documentation (`d5a6470`).

## What changed: two regions, two files

| File | Final line | Change |
|---|---|---|
| `help/en/view-batch.md` | 16 | `see the suggestion card topic` -> `see the Suggestion card topic` |
| `help/en/view-editor.md` | 17 | same |

One character per site. `git diff --numstat`: `1 1` for each file - one line
changed each, nothing else.

## Verification

**Target title checked before editing**, as with the German pair:
`help/en/batch-suggestion-card.md` line 1 is `# Suggestion card` (verified with
`cat -A`, no trailing whitespace), so the replacement equals the real h1.

**Census (corrected after review; see finding 4 for why the first figures were
wrong):**

| Measure (`help/en/`, `see the ... topic`) | pre-edit | post-edit |
|---|---|---|
| total cross-references | 14 | 14 |
| conformant (capitalized title) | 12 | 14 |
| lowercase-initial title | 2 | 0 |

Measured with a parenthesis-tolerant pattern (`grep -ohP 'see the .*?topic'`
and its `[A-Z]` / `[a-z]` variants), pre-edit figures taken against the
`ddb707a` tree and post-edit against `eb4608b`. The two lowercase sites were
and remain the only non-conformant ones, so the corrected census changes no
edit.

**Fire-verified string counts:**

| Pattern | pre-edit | post-edit |
|---|---|---|
| `see the suggestion card topic` | 2 (fired) | 0 |
| `see the Suggestion card topic` | 0 | 2 |

The old pattern returned exactly 2 before the edit, so the post-edit 0 is a real
absence.

**Case-only change proven.** `git diff --word-diff` yields exactly four tokens
across the whole diff: `[-suggestion-]` `{+Suggestion+}` twice. Independently, a
case-insensitive compare of each file against its parent (`diff -i`) reports the
files identical, so nothing but capitalization changed. That check was
fire-controlled: the same `diff -i` invocation against the German `zur` deletion
in `ddb707a` reports a difference, so it does detect real content changes and
its "identical" verdict here is meaningful.

## Checked and dismissed: the fourth `see the [a-z]` hit is not a third site

A broader net (`see the [a-z]` across `help/en/`) returns four lines, two of
which are not instances of the ruled pattern:

- `help/en/batch-suggestion-card.md:17` - "Run the dry run again to see the
  effect" - ordinary prose, not a cross-reference at all.
- `help/en/editor-track-rule-match-expr.md:17` - "see the suggestion card in the
  Batch view" - a cross-reference, but a different construction: it points at
  the UI element in a view, not at a named help topic, and carries no "topic"
  noun. Its German counterpart at `help/de/editor-track-rule-match-expr.md:17`
  reads "siehe die Vorschlagskarte in der Stapel-Ansicht" - the same
  construction, and the only `siehe die ...` form in the German help. The two
  languages are parallel and internally consistent here.

So the ruled pattern has exactly two sites in English, as claimed. Recorded so
the reviewer does not have to re-derive the dismissal.

## Standing state after three commits

`d5a6470` (plan-close documentation), `ddb707a` (German help), `eb4608b`
(English help): all on `master`, all unsigned, none pushed, each staged
explicitly by path. `docs/ROADMAP.md`, `docs/conventions.yaml` and
`docs/decision-ledger.yaml` remain modified in the working tree, unstaged and
untouched - the controller's own uncommitted changes.

The suggestion-card cross-reference is now house-conformant in both languages;
finding 3 is closed.

---

# Addendum 3: review disposition and the tracker-location qualifier

Independent review of `d5a6470` + `ddb707a` + `eb4608b`: NEEDS FIXES, one
MEDIUM, landing on a measurement rather than on the edits. The edit set was
confirmed correct and complete. Both adjudications raised from the
implementer side were ruled in favour, including the decision to report the
English casing deviation instead of editing it.

## Finding 4 (the MEDIUM): the census pattern could not see a parenthesized title

The English census was 14 sites / 12 conformant / 2 lowercase, not 13 / 11 / 2.
The uncounted site is `help/en/editor-tracks-rules.md:7`, "see the Unmatched
(tracks) topic" - fully conformant, and conformant for the ruled reason: the h1
scheme keeps the parenthetical because a bare `Unmatched` would collide with
`help/en/editor-attachments-unmatched.md`, whose h1 is `# Unmatched
(attachments)`. Its German counterpart, `siehe das Thema Nicht zugeordnet
(Spuren)`, carries the same parenthetical.

**Cause, structural rather than arithmetic:** the pattern `see the [^)]*topic`
cannot match a parenthesized title, because the `)` inside the title ends the
negated character class before `topic` is reached. The controller originated
that pattern; this report reproduced it and inherited the blind spot verbatim.

That is the part worth keeping: **re-running someone else's pattern is not an
independent measurement, it is the same measurement twice.** "Reproduced
independently, and it matches exactly" was true and worthless - the agreement
was guaranteed by construction, because both runs asked the same malformed
question. Independence lives in the method, not in who executes it. The
verification discipline this task otherwise applied (fire the pattern, watch it
return the expected non-zero) also cannot catch this class: the defective
pattern *did* fire, at 13, and 13 is a plausible number. A count that fires is
still only as good as the question it encodes.

**Re-measured here, not taken on the controller's word**, with a
parenthesis-tolerant pattern: pre-edit (`ddb707a` tree) 14 / 12 / 2, post-edit
(`eb4608b`) 14 / 14 / 0. Fire control for the diagnosis itself: restricted to
`help/en/editor-tracks-rules.md`, the defective pattern matches 2 lines and the
tolerant one 3, so the drop is reproduced on demand rather than asserted.

Figures corrected in the addendum-2 census table and in the finding-3 listing
above. The German house-form list was corrected too: it had rendered `Nicht
zugeordnet (Spuren)` as `Nicht zugeordnet`, truncated by the same defect. The
German count was unaffected, since that variant of the pattern had no required
terminator after the title and still matched its line.

## Disposition: `eb4608b`'s commit message keeps the stale figure

`eb4608b`'s message says eleven of thirteen. The correct figures are twelve of
fourteen. **Not amended and not rebased:** commits now sit on top of it and
another implementer is committing to this tree concurrently, so rewriting that
history would invalidate work that is not mine. Per the established house
handling for exactly this case, the commit message is immutable and the
correction lives in this report and in the controller-written progress tracker.
A reader arriving at `eb4608b` from `git log` finds the stale count; a reader
arriving here finds the corrected one and this note explaining the divergence.
Nothing the commit actually did was wrong - the two lowercase sites it fixed
were and remain the only non-conformant ones.

## Product edit (review LOW 3): tracker-location claims qualified

**Commit `bd7dba9`**, unsigned (`%G?` -> `N`), not pushed. Own commit, keeping
the surface split: this is the internal plan document, a fourth surface.

The plan states its tracker location twice in the PRESENT tense, and both named
the git-ignored path with no hint that a readable twin exists:

| File | Final line | Site |
|---|---|---|
| `docs/superpowers/plans/2026-07-23-plan-7.5-track-rule-add-remove.md` | 5 | house-deviation header, "The tracker is ..." |
| same | 31 | execution-method section, "Progress lives in ..." |

The earlier leave-bare boundary under-delivered what the house rule for
evidentiary citations asks. Sentence and path are kept exactly - the claim
records where the tracker lived while the plan executed, which remains true -
and each gains the ruled parenthetical:

> (git-ignored during execution; salvaged at the plan close to
> `docs/process-journal/artifacts/plan-7.5-sdd/progress.md`)

**Verification.** The bare form `` `.superpowers/sdd/plan-7.5/progress.md`. ``
counted 2 before the edit and 0 after; the new parenthetical counts 2.
`--numstat` reports `2 2` - two lines changed, nothing else - and a word-diff
shows each changed token as the old bare citation replaced by itself plus the
appended parenthetical, so neither path nor sentence was altered. The salvage
target is tracked (`git ls-files --error-unmatch` on
`docs/process-journal/artifacts/plan-7.5-sdd/progress.md` resolves).

**Third-site sweep, as instructed.** Searching the plan for `progress.md`,
for `.superpowers/sdd/plan-7.5`, and for `tracker|progress lives|progress is`
returns lines 5 and 31 and nothing else in the present tense. Line 415, the
close-actions salvage-re-pointing bullet, is the only other mention: past
tense, recording what a pre-registered trigger said, with its consumption
recorded in the ROADMAP trigger entry. Correctly excluded; two sites, not
three. No third present-tense claim exists.

## Standing state after four commits

`d5a6470` (plan-close documentation), `ddb707a` (German help), `eb4608b`
(English help), `bd7dba9` (plan document): all on `master`, all unsigned, none
pushed, each staged explicitly by path. `docs/ROADMAP.md` and the `docs/*.yaml`
house-knowledge files remain the controller's, modified in the working tree and
untouched here. The concurrent implementer's files (`docs/INSTALL.md`,
`.github/release/draft-body.md`, `packaging/linux-tarball-README.txt`) were
never staged by this task.
