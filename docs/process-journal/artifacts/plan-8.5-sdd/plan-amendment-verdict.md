# Plan 8.5 mid-run amendment verdict (commit 29ef17b)

Same reviewer as round 1 and the delta; scope is the amendment only.
Graded: `29ef17bd6dbb2f0b859d56bf296fb12cfe0384ca` ("plan: amendment-log
references become pointers, not enumerations"), one file
(`docs/superpowers/plans/2026-07-27-plan-8.5-macos-packaging-fixes.md`),
2 insertions / 2 deletions, two hunks (verified `git show --stat` and the
full diff). Diffed against `29ef17b` and its parent `50e08cd` per the
routing note; the `86bfd69` index incident is ledgered elsewhere
(`338e779`) and not re-litigated here. Working tree clean at start and
end.

## STATUS: APPROVED

## 1. Exactly two sites, both took the pointer - confirmed by measurement

At the parent `50e08cd`: `git grep -n 'A1-A3'` on the plan returns
exactly **2** hits (:17 Global Constraints, :113 Task 1 read-list);
`D75-D90` returns exactly **1** hit, on the same :17 line. At HEAD:
`A1-A3` count **0**, `D75-D90` count **0**. The Task-2 reviewer's "exactly
two live sites, both in the plan" holds, both took the pointer, and no
third stale membership enumeration remains in the plan.

## 2. Site 1 (Global Constraints) reads as a rule, and a stronger one than it replaced

The replacement is stated as a bolded rule with a binding verb and a
scope: "its D-entries plus EVERY amendment in its `## Amendment log` bind
this plan, **at the log's state at execution time, not at
plan-authoring**". Determinability: an implementer landing on it opens
the one named file; membership resolves by observation, not judgment -
measured at HEAD, the design has exactly **16** `^## D` sections
(D75..D90; 90-75+1 = 16) and **5** log entries (A1-A5 at :2065, :2096,
:2113, :2154, :2175), the log being one terminal section as the rule's
path implies. The precedence chain around it is untouched (v1 spec above,
house YAMLs alongside, the contradiction-carries-an-amendment sentence
retained). It is not weaker than the enumeration: it is *stronger on
exactly the axis that failed* - the old line was silent about the log
growing mid-run, which is how it went stale while the plan was executing
under A5. This is also not a `proc-latitude-clause-boundary` violation:
that rule bars sets an implementer must *invent* or *derive by judgment*
(the D49 shape); membership here is read off an authoritative artifact at
a named location and named time. The cited precedent is real: the
design's own status line (design :3-4) already carries the pointer form
("every amendment is recorded in the `## Amendment log` section at the
end"), verified at HEAD. No finding.

## 3. The extension (dropping "D75-D90"): outcome right, rationale slightly over-broad - no finding

The author's stated ground ("the ruling's rationale covers any
membership list of a growing governing set") over-generalizes on one
point: the D-set is **not** growing - this house lands design changes as
A-entries, the D-numbering was fixed at approval, and the design's own
header pins the numbering block (D75 start, D65-D74 reserved elsewhere).
So the D-range was not at the same staleness risk. But the operative
claim - "naming the file loses nothing" - is true, measured: "its
D-entries" resolves to exactly D75-D90 today (16 headings, nothing else
matches `^## D`), so the binding set is identical under both readings.
What the range carried beyond that: self-containedness without opening
the file (operationally nil - the constraint requires the file open to
execute against) and a bound excluding a hypothetical future D91 (which,
arriving only via a routed design change, SHOULD bind an in-flight plan -
the same semantics the ruling just adopted for A-entries; a uniform
execution-time contract for both halves is more coherent than a frozen
half and a live half). The extension was reasoned, recorded, and holds.

## 4. Site 2 (Task 1 read-list): correct, and the stronger form for that reader

"the `## Amendment log` in full (never a fixed A-range: the log grows,
and this very task appends to it in Step 5)" - all three supporting
claims check: the same read-list already sends the reader into the design
(D75, D86), the log is one terminal section (measured, :2065-end), and
Task 1 is itself an appender (Step 5(c) appends A4 - executed as
`9460daf`, correctly numbered and formatted, which is what "in full" buys
an appender: knowledge of the current tail). The site is historical now
that Task 1 is executed, but the plan is the standing record and the cure
keeps the line true for every future read. Correct.

## 5. Collateral: nothing broken

Swept every remaining `amendment log` / A-reference in the plan at HEAD:
the coverage-map row (:84, names A4/A5/A3 as member citations - still
true), the 1->2 dependency edge (:93), the append instructions (:246,
:354, :457), the Task 2 interface (:297 "Consumes: A4") - all remain
true; none named the old wording. No plan count consumed the old
enumeration (the round-1 recount register carried no A-set count). One
observation, no finding: Task 2's read-list (:288) still names "A2, A3" -
that is a content read-list (the fence rules its A5 bookkeeping needed),
not a membership enumeration of the binding set, and the task is already
executed (`50e08cd`); it is not the ruled class and was correctly left
alone.

## HARVEST

- **H8.** The cure distinguishes two reference kinds the ruling's class
  analysis needs: a *membership enumeration* of a growing set (goes
  stale by construction; pointer it) versus a *member citation* for
  content (stays true forever; leave it). Site :288 vs site :17 is the
  clean pair. A repointing sweep that flattened both kinds would have
  destroyed information; this one cut exactly the first kind.
- **H9.** A pointer replacing an enumeration should carry a temporal
  clause ("at the log's state at execution time") - the enumeration's
  real defect was not the list but the unstated snapshot time, and a
  pointer without the clause would re-import the same ambiguity one
  level up.
