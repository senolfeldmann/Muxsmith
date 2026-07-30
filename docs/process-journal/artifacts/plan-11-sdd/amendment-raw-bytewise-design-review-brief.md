# Plan 11 amendment, design half - controller brief for the REVIEWER

You grade a design document authored by a different agent. You did not write it,
you owe it no deference, and your verdict gates it: only after you approve does
the plan's own author fold it into Plan 11, with that plan's original reviewer
judging the delta. This is the design half of a four-role amendment.

## Artifacts

- Under review: `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`
  (ADR D111).
- Its requirement set: `.superpowers/sdd/plan-11/amendment-raw-bytewise-design-brief.md`.
  Read it; the design refutes four of its premises and you must check each.
- Context: `docs/ROADMAP.md`'s "Docs accuracy" entry on `"byte-exact"`, which
  carries the owner ruling and its refinement; the approved plan
  `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`,
  task A3; the v1 spec sections 4.3, 4.4, 9.2 and the diagnostics table.
- Doctrine: `~/agents/peter/prompts/software-dev-process.md`. Section 1's ADR slot
  requirements, section 3's latitude ban in both forms and its proposed-safeguard
  rule bind the artifact; section 4's independent-instruments rule and its
  claimed-versus-prescribed distinction bind you.

## The safety property that outranks everything else here

The design rewrites `scalar_eq` as `scalar_eq_same_type(..) || <the two cross
arms>` and re-points only the `raw:` call site. **Verify that this preserves the
typed `exact` path EXACTLY** - same result for every input pair, no reordering
effect, no short-circuit change that alters behaviour. If the typed path moves at
all, that is blocking: it is documented behaviour of the whole matcher, and the
owner's ruling is scoped to `raw:` alone. Build the check yourself; do not read it
off the diff.

Second: the design claims `substring`/`regex` under `raw:` are unaffected because
they route through `item_str` and never reach a scalar comparison. Confirm.

## The four refuted premises, each of which you re-measure

1. **`PropValue::from` is really `PropValue::from_json`.** Trivial to confirm and
   worth confirming, because the controller's brief would have put a nonexistent
   function name into a comment.
2. **"There is no parity model" is refuted.** The design finds that mkvtoolnix has
   no profile or preset concept but DOES have declarative value-based track
   selection which is strictly same-type, citing `parse_arg_tracks` and
   `item_selector_c`, and reports seven binary runs (`-a 0`/`-a ger` select,
   `-a 1`/`-a eng` do not, `-a de`/`-a deu` normalize onto a `ger`-tagged track,
   `-a de-DE` does not). **Reproduce the runs with your own probe file**, and judge
   whether the analogy genuinely corroborates the ruling or is being stretched:
   mkvmerge's track selector is a CLI argument parser, not a profile matcher, and
   the interactive-versus-declarative distinction SI-3 turns on cuts both ways.
3. **mkvmerge does NOT emit the shortest round-tripping form.** Measured:
   `--max-luminance 0:6.00` reports `6.0` (so the `6.0`/`6.00` conclusion holds)
   but `400` is written back as `400.0`. **This is load-bearing**, because that
   habit is what makes the `(Int, Float)` direction reachable from an ordinary
   file, and the design pins both directions in its tests on the strength of it.
   Reproduce it, and check the test set actually covers what the measurement
   implies.
4. **"The suggestion engine reports a proposed narrowing" is partly refuted.**
   Measured: `suggestions` is `[]` and `missing-track` carries empty params both
   for a `raw:` non-match and for a known-property control, and an **optional**
   rule produces no error at all, exit 0. Reproduce this. It matters beyond the
   design: it is one of the four arguments on which the owner's ruling was
   recommended, so the controller owes him a correction, and your reproduction
   decides how strongly it is worded.

## The six decisions, and where I want scrutiny rather than a tick

- **The semantics table.** Sixteen pairs plus absent. Check exhaustiveness against
  the actual `Scalar` and `PropValue` variant sets rather than against the table's
  own axes, and check the float row: IEEE equality means `NaN` never matches
  itself. Decide whether that needs stating.
- **Wording: 12 repair / 7 retain / 2 different-claim** against the plan's 6/9.
  The surface is stated as a RULE rather than a file list, deliberately excluding
  the design's own document. Re-derive it your own way - the design used a
  block-based subject-first pass because a line grep is blind to hard-wrapped
  prose - and report your delta. The **retain** half is where a wrong call ships a
  false statement, so check each retained line is genuinely true under the new
  semantics, not merely conveniently exempt.
- **Tests: three, including the mandatory safeguard** proving the typed path still
  cross-compares, on `max_luminance` with a negative control, plus B-7 inverted
  both directions and a 16-pair matrix. Grade these as DESIGNS against their
  specification: does each prescribed red state exercise the anchor? And apply the
  question that found two defects in the sibling plan: **would this producer still
  pass if the mechanism it covers were broken?**
- **Item 4, `RawOnKnownProperty` unchanged, with the widening ESCALATED to the
  owner** with a recommendation to build a config-time never-match guard in its own
  package. Judge whether that is an honest escalation or a safeguard argued away
  under an escalation's clothing. The proposed-safeguard rule says a guard the
  design proposes stays; the question is whether this one was ever proposed as part
  of THIS package.
- **Item 5, `UnknownPropertySkew` unchanged**, on the ground that "matched untyped"
  describes the path, which stays true - and a pre-existing defect (it fires even
  when nothing matched) surfaced rather than ridden in. This is a
  "therefore no change needed" conclusion: **run the premise, do not weigh it.**
- **Item 6**, the diagnostics-table row and its code-side twin.

## Section 13, and it is the reason the plan half can proceed

The design enumerates every Plan 11 task A3 clause the amendment voids: the Files
list, both Step-7 checks, the diff-scope check, the no-new-test ground, three
must-not-decide clauses. **The plan author consumes that list**, so an omission
there silently leaves a contradicted clause in an approved plan. Check it for
completeness against task A3 yourself rather than against the list.

Section 12's twelve triggers and the `superseded by D111` link owed to D32's B-7
row are the controller's to route; confirm they are complete enough to route from,
and flag anything the design should have surfaced and did not.

## Standing

Own instruments at your own path:
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/d111-review-independent/`.
Never at the path the author used. Recommend no safeguard removals. Where any
passage concludes something is unnecessary, run the premise.

Read-only with respect to the repository apart from your verdict file: no edits to
the design, product files, the plan, trackers or house-knowledge YAML, and nothing
staged, committed or pushed. No worktree, no session-relocation tool.

Verdict to `.superpowers/sdd/plan-11/amendment-raw-bytewise-design-review.md`, a
file before it is an answer. APPROVED / NEEDS_FIXES / BLOCKED with one paragraph of
reasoning; numbered findings each with severity, location, defect, evidence and
what resolves it; your reproductions with measured figures beside the author's; a
harvest section. **If you approve, say so plainly** - an unambiguous gate signal is
what the plan half waits on.

Your final response: verdict, finding count by severity, which of the four
refutations reproduced, and whether the typed path is provably unmoved.
