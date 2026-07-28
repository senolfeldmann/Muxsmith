# Plan 9 design review brief (round 1)

Independent reviewer, fresh eyes; you did not author this design. Artifact
under review:
`/home/senol/Git/Muxsmith/docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
(commit `cd8a27a`, D91-D105, 1397 lines).

**Its section 0 is under review too.** The author refuted several premises of
the controller brief and corrected two recon claims. The controller
re-verified two of those at the source and accepted them (BatchView detects
by `!doc.profile`, not by the index read; the mount harness passes props
statically so a second `pendingRun` is undeliverable without a hook) and
recorded the resulting scope call in the ROADMAP anchor. Check the REST of
section 0 the same way, and check that each accepted correction actually
propagated into the decision that rests on it - a correction recorded in
section 0 but not applied in its D-section is a finding.

Ground truth, in this order of authority: the v1 spec
(`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`), the controller
brief `.superpowers/sdd/plan-9/design-brief.md`, the owner rulings in the
ROADMAP Plan 9 anchor (commit `90bc3ae`), the Tier-2 house files
(`docs/conventions.yaml`, `docs/process-conventions.yaml`,
`docs/product-boundaries.yaml`) and the Tier-1 `docs/decision-ledger.yaml`,
and **the actual tree**. The design's claims are verified against reality and
by your own lookups, never believed.

The owner's eight rulings (brief section 3) are NOT under review. Grade how
the design mechanizes them; do not re-litigate whether they are right. The
same holds for the two OUT items: their exclusion is ruled.

## Dimensions

1. **Fork coverage.** The brief enumerates NINETEEN forks the design must
   close. Walk them one by one and name the section that closes each. A fork
   with no decision is a finding; so is a decision that closes it with a
   choice ("either shape works"). This is the review's primary dimension: the
   characteristic design defect is a missing decision, not a wrong one, and
   nothing downstream catches it.
2. **Recon open-question coverage.** `.superpowers/sdd/plan-9/recon-inventory.md`
   records OPEN QUESTIONS per input (sections 1.4, 2.3, 3.4, 4.5, 5.4, 6.4,
   7.4, 8.5, 10.4). Every one is either closed by a decision in this design or
   explicitly declared out of scope WITH a reason. Silence on one is a
   finding.
3. **Behavior preservation across the hoist - the dimension this plan lives
   or dies on.** The recon classifies seven DELIBERATE divergences (D-1 to
   D-7) among the four pipeline copies. For each: does the design keep it, as
   a parameter or as a documented per-surface difference, and would the
   designed `plan_pipeline` actually reproduce today's observable behavior on
   all four surfaces? Check D-2 hardest (`mkvmerge_found: Some(false)` means
   two different things in the same wire field today); a single meaning chosen
   without saying which surface's behavior changes is a finding. Also verify
   the six accidental differences (A-1 to A-6) are genuinely accidental as
   claimed, by reading the sites.
4. **Empirical and anchor verification, with your OWN instruments.** The
   design must cite commands and outputs for its behavioral claims. Re-verify
   the load-bearing ones YOURSELF, and build any harness you need at a scratch
   path you invent in this pass - never re-run a script, fixture or copy the
   author wrote, and never at the obvious shared path both of you would
   default to. Executing the author's instrument produces agreement by
   construction. Specifically re-verify: the measured spans the design relies
   on (the four copies, `run_batch`, `resolve_runs_root`); that
   `resolve_runs_root` has exactly the three GUI call sites the design names
   and no test consumer; that the `worker-panicked` catalog key exists in both
   locales and is looked up nowhere else; the empty-`raw:` behavior at
   validate, at match time and at planning (construct a profile and run the
   binaries); and that `BatchView`'s index-0 read is the only POSITIONAL
   consumer of `config_diagnostics` (corrected 2026-07-28 from
   "order-dependent", which was too narrow and is the round-1 reviewer's own
   brief-boundary finding: EditorView and BatchView also RENDER the array in
   order, so their display changes under the sort even though nothing indexes
   it). Any negative result you report must be
   fire-verified: make the identical check produce output once against a
   known-present case, and say so.
5. **Latitude scan, both forms.** Explicit permissions ("the implementer may
   choose", "either approach", "if a simpler equivalent exists") AND omission
   latitude: an unenumerated set in a normative position, a list ending open,
   a placeholder, a "one per X" without the X list, a mandated key set whose
   keys are never listed. The test is "must the implementer invent something
   it is not allowed to invent?", applied to every normative sentence, not to
   the vocabulary.
6. **User-visible strings and keys are complete.** The new DiagCode's variant
   name, catalog key, both locales' Fluent text verbatim, its spec 5.2
   severity row, its `catalog_completeness` fixture row. Same for any new key
   the GUI job row needs. A missing string is omission latitude with a user
   consequence.
7. **House dimension.** Deviations from a recorded Tier-2 convention or from
   the dominant local pattern, cited by entry id. Include the ledger entries
   the rulings touched (`core-121-planner-seam-and-hoist`,
   `exec-36-core-stderr-logging`, `exec-37-panicked-msg-catalog`,
   `cli-08-config-diags-json-ordering`, `exec-43-runsroot-debug-gated`,
   `empty-bare-raw-property-rejected-at-validate`): does the design contradict
   a statement or a steelman recorded there?
8. **No-work-needed check.** Every passage concluding that a guard, an
   enumeration, a test or a check is unnecessary ("so we need no X", "X cannot
   happen here", "the work already exists", "this is covered by Y") - RUN the
   premise, do not weigh it. This shape has produced false claims in this
   project's design rounds before, including in a document that had just
   conceded the pattern.
9. **Safeguard survival.** No guard, test or enumeration that the brief, the
   recon or an earlier round proposed may be argued away in this document. A
   design-phase removal rests on agreement rather than measurement by
   construction, so flag any dropped safeguard as a finding regardless of how
   good the argument reads.
10. **ADR quality.** Every `Dn` complete: decision, rationale, rejected
    alternatives each with a real steelman (not a strawman), triggers created
    and stated as `<observable event> -> <action>` for ROADMAP mirroring,
    interface and wire-format changes called out, `superseded by` links where
    a prior D is displaced. Numbering starts at D91 with no collision; verify
    the highest D in use yourself.
11. **Spec amendments.** Every spec section the design changes is named with
    exact replacement text, and the amendment does not contradict a
    neighbouring spec section (this has happened here: 4.9 contradicted 4.5
    two plans after the ADR that changed it). Check §5.2, §5.5/7, §6/7 in
    particular, and sweep for collateral contradictions yourself.
12. **Acceptance observables have producers.** For each ruled item, the design
    states what is observably true when it is built and which emitter produces
    that observable. An acceptance item whose observable has no named producer
    is not acceptance.
13. **SI-3 parity.** The brief names exactly three comparisons that are owed
    (per-job failure reporting against mkvtoolnix-gui's job queue; diagnostic
    ordering in mkvmerge's own output; and an explicit statement that nothing
    else has an analogue). Spot-check the parity claims at
    `~/Downloads/mkvtoolnix` and by running the installed `mkvmerge v100.0`.
    No literal code or text adoption; a deliberately modeled wording must be
    recorded as an explicit decision.
14. **Scope discipline.** No work on the two OUT items (no Vitest, no
    `tauri::test`/`mock_builder`, no `src-tauri/tests/`, no IpcError funnel
    beyond the ruled mount-glob widening), no new dependency, no
    product-boundary change, no plan/task structure, no release action.

## Output

Write `/home/senol/Git/Muxsmith/.superpowers/sdd/plan-9/design-review-round-1.md`:

- Verdict: `APPROVED` or `NEEDS FIXES`.
- Findings by severity (Critical / Important / Minor), each with its location
  in the document, what is wrong, and what to change. For an empirical
  finding, include the command and output that establishes it.
- A `## HARVEST` section: dominant patterns you observed, repeated rejections,
  and any place where a rule or the brief's own boundary forced a stop you
  judge it should have covered (an over-restriction finding is wanted, not
  second-guessing).

Final message: the verdict word, at most three lines of summary, and the file
path.

## Constraints

Read-only on the tree except your own verdict file. No git commands at all.
Never call a session-relocation tool (EnterWorktree/ExitWorktree or
equivalent); scratch space is a plain directory under /tmp that you name.
Absolute paths everywhere. Anything you run, run in the foreground. Building
and running your own probes against the repo is expected for dimension 4.
