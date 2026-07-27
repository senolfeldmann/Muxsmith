# Plan-7.5 close fix: delta judgement

## VERDICT: APPROVED

Delta only. Same judge, same standards; settled non-findings from
`close-fix-verdict.md` are not reopened. MEDIUM 1 is discharged, LOW 3 is
discharged, and the two remaining notes below are observational and block
nothing.

**Scope of this pass:** report addendum 3 plus commit `bd7dba9`, against the
three coordinator items. Everything is re-measured; nothing is taken from the
report or from the coordinator message. Graded set pinned as before to
`ddb707a` / `eb4608b` for the census and to `bd7dba9` for the product edit.
`ecab53a`, `bcb67f3`, `6d81738` and `14735b8` are excluded per Q1 and the
coordinator's confirmation; `14735b8` is read only as evidence for item 2, not
graded.

---

## Item 1: the census correction. VERIFIED, nothing outstanding.

**Corrected figures reproduce exactly.** Measured with a parenthesis-tolerant
pattern, pinned to the two revisions the report names:

| revision | total | conformant | lowercase |
|---|---|---|---|
| `ddb707a` (pre-edit) | 14 | 12 | 2 |
| `eb4608b` (post-edit) | 14 | 14 | 0 |

These are the report's corrected numbers, arrived at independently. The
finding-3 listing now sums correctly too: the nine title lines total 12, plus
the 2 lowercase sites gives 14.

**The diagnosis fire control is real, not asserted.** Restricted to
`help/en/editor-tracks-rules.md` at `eb4608b`, the defective pattern
`see the [^)]*topic` matches **2** lines and the tolerant `see the .*?topic`
matches **3**. The one-line drop is produced on demand, in a scope small enough
to inspect by eye. This is the right shape: the report did not merely accept my
count, it reproduced the *mechanism* that generated the wrong one. That is the
form `proc-verification-step-must-be-falsifiable` asks for applied to a
diagnosis rather than to a check.

**The second instance is genuine, and neither of us caught it.** The German
house-form list had rendered the eighth title as `... Nicht zugeordnet`,
truncated by the same character class. Verified: the real h1 at
`help/de/editor-tracks-unmatched.md:1` is `# Nicht zugeordnet (Spuren)`, the
corrected list now carries it, and the corrected list's nine distinct titles
against 13 sites matches my own German census exactly.

**Its "count unaffected" claim is measured true, not waved through.** I ran
three pattern variants over `help/de/` at `eb4608b`:

| variant | hits |
|---|---|
| `siehe das Thema [^)]*` (defective) | 13 |
| `siehe das Thema .*?` (tolerant) | 13 |
| `siehe das Thema` (literal) | 13 |

The stated reason holds structurally: the German pattern has no required
terminator after the title, so the parenthesis truncates the *display* without
losing the *line*. The English pattern required `topic` after the negated class
and therefore lost the line entirely. Same character class, two different
failure modes, and the report names the difference correctly.

Finding the second instance is the part that earns the approval. The correction
I demanded was one number; the report went back through every artifact the
pattern had touched and found a place I had not looked.

## Item 2: the commit-message disposition. SUFFICIENT, with one report-side note.

**Ruling: sufficient.** A reader who lands on `eb4608b` first has a one-step
public path to the correction, and it is not the one the report names.

`docs/process-conventions.yaml:617` is a **tracked** file carrying a new
`violated-corrected` occurrence under `proc-sweep-surface-completeness` (added
by the controller in `14735b8`). It names the wrong figures, the right ones, the
cause, and the commit:

> "the English help cross-reference census was reported as 13 sites / 11
> conformant by four consecutive layers (controller measurement, implementer
> report, eb4608b commit message, review brief); the tree holds 14 / 12. The
> pattern see the [^)]*topic cannot match a parenthesized title, so
> help/en/editor-tracks-rules.md:7 was invisible"

Decisively: `git grep eb4608b` over the tracked tree returns **exactly one hit**,
that occurrence. So the commit hash itself is the index term, and a reader
starting at the stale message reaches the correction by the most obvious search
they would run. Not amending was the right call for the stated reasons (three
commits sit on top, two writers were live), and the record is reachable, so the
immutability argument costs nothing.

**LOW 6 (report-side, no edit required): the disposition paragraph understates
its own record.** Report line 374 says the correction "lives in this report and
in the controller-written progress tracker". Both are git-ignored, confirmed:
`git check-ignore -v` resolves `.superpowers/sdd/plan-7.5/close-fix-report.md`
and `.superpowers/sdd/plan-7.5/progress.md` to `.gitignore:2`. Neither is
reachable by a public-repo reader. The paragraph omits the one record that *is*
reachable and that actually answers the question the coordinator asked. The
disposition is right; its stated justification points at the two records that
cannot carry it.

**Observation, structural, not a defect of this delta.** The salvaged archive
copy `docs/process-journal/artifacts/plan-7.5-sdd/progress.md` is 30 lines; the
live tracker is 32. The two added lines are the close-fix account and the
controller's rulings on my open questions, both written after the salvage commit
`8e2c044` (18:13). Grepping the salvaged copy for the correction returns 0. So
the public archive of the plan-7.5 record is frozen at a point before this close
finished, and the four commits and one correction the close then produced are
absent from it. The durable residue survives in the ledger occurrence, which is
why this is an observation and not a finding. See H7.

## Item 3: commit `bd7dba9`. VERIFIED, nothing outstanding.

**The edit is insertion-only at token level.** `git diff --word-diff=porcelain`
yields exactly four tokens across the commit: on each of the two lines, the old
`` `.superpowers/sdd/plan-7.5/progress.md`. `` replaced by itself plus the
appended parenthetical. Sentence, surrounding text and the original path are
byte-identical; only the terminal period moves outside the new parenthesis.
`--numstat` is `2 2`, one file.

**Counts reproduce.** Scoped to the plan document: the bare form counts 2 at
`bd7dba9^` and 0 at `bd7dba9`; the new parenthetical counts 2. The salvage twin
`docs/process-journal/artifacts/plan-7.5-sdd/progress.md` resolves under
`git ls-files --error-unmatch`.

**It satisfies all three elements the ruling invokes.**
`code-comment-line-citations-drift` asks an evidentiary record for the original
verbatim, a scoping qualifier naming the staling event, and a pointer to the live
twin. The landed text delivers each: path kept, "git-ignored during execution;
salvaged at the plan close" as the qualifier, and the tracked twin as the
pointer. Keeping the present-tense "The tracker is" is correct and not a residue
of my LOW 3: rewriting it to past tense would have altered the record the ruling
said to preserve, and the parenthetical already resolves what a public reader
needs to know.

**The third-site sweep holds under an independent, broader pattern.** I did not
re-run the report's `tracker|progress lives|progress is`. Over the plan document
at HEAD:

- case-insensitive `progress|tracker`: **2** hits, lines 5 and 31.
- literal `.superpowers`: **3** hits, lines 5, 31 and 415.

So there is no third present-tense tracker-location claim, and line 415 is the
only other mention. Line 415 is untouched, which matches the ruling's stated
scope (the two present-tense claims only, not the past-tense trigger record).

**Hygiene.** Unsigned (`%G?` = `N`), unpushed (`origin/master` still `7302e1b`),
one path staged. Across all four implementer commits the path set is seven files
and contains nothing controller-side and nothing from the parallel plan-8 wave:
no `docs/ROADMAP.md`, no `docs/*.yaml`, no `docs/INSTALL.md`,
`.github/release/draft-body.md` or `packaging/linux-tarball-README.txt`.
Typography: 2 added lines, 0 hits for em/en dash, curly quotes, ellipsis, NBSP,
Unicode minus; control fired.

---

## Outstanding

Nothing blocking. One low report-side note (item 2's disposition paragraph naming
only the two git-ignored records) and one structural observation (the salvaged
archive predates the close's own output). Neither needs an edit before the close.

---

## HARVEST (delta)

**H7. A plan's public archive is frozen at the salvage, but the close keeps
producing.** Plan 7.5 salvaged at 18:13 and then produced four commits, a review,
a correction and two controller rulings, none of which are in the archived
`progress.md`. The salvage is not the last act of a close, it is roughly the
midpoint. Two workable handles, both cheap: run the salvage as the *last* close
action after the review loop settles, or accept the freeze and require that
anything the close produces afterwards lands in a tracked file (the ledger
occurrence did that here, by luck of the harvest rather than by rule). Trigger is
readable: you are dispatching a salvage while a review of that plan's close is
still open.

**H8. When a record is called "sufficient", name the reachable copy.** The
disposition paragraph justified not amending by pointing at two records that a
public reader cannot open, while the one tracked record that carries the
correction went unmentioned. The argument was right and its evidence was the
wrong two files. Handle: when you claim a correction is recorded, state which
copy is reachable from where the stale claim lives, and check the reachability
(`git check-ignore`, `git grep <hash>`) rather than assuming a file in the task
folder counts as a record.

**H9. Correcting a pattern defect means re-walking every artifact the pattern
touched, not just the number that was challenged.** The review named one wrong
count; the correction found a second, silent instance in the German list where
the same character class had truncated a displayed title without changing any
number. A count defect announces itself; a display defect from the same pattern
does not. Handle: when a sweep pattern is found broken, list every output that
pattern produced and re-derive each, including the ones nobody questioned.
Reinforces `proc-sweep-surface-completeness` from the remediation side.
