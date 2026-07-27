# Plan 7.5 design brief (controller-authored)

You author the DESIGN DOCUMENT for Plan 7.5: track-rule add/remove in the
profile editor. Four-eyes: an independent reviewer grades your document
against this brief before the governing human sees it. You write exactly
ONE file: `docs/superpowers/specs/2026-07-22-plan75-track-rule-add-remove-design.md`.
Do NOT commit (the controller commits); touch nothing else in the tree.

## Read first (in this order)

1. docs/ROADMAP.md - the "Plan 7.5" anchor incl. the S22 KICKOFF block
   (the owner rulings, restated below) and the v1.x undo/redo entry.
2. Spec docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md §8.2
   (grid, detail panel), §8.3 (tooltips, help mode incl. the
   pointer-scoped suppression), §5.2 (diagnostics), §4.5 (track rules).
3. docs/superpowers/specs/2026-07-21-plan7-help-i18n-design.md - the house
   STRUCTURAL TEMPLATE (section layout, decision-log form, triggers,
   spec-amendment style, test plan).
4. Tier-2 house files: docs/product-boundaries.yaml (editor-generic-
   action-keys incl. the 46-key budget, gui-closed-domain-dropdowns,
   help-mode-suppression-pointer-scope, core-83), docs/conventions.yaml
   (help-topic-h1-scheme, code-comment-line-citations-drift,
   content-claims anchor rules), docs/process-conventions.yaml.
5. The editor implementation: src/ (EditorView.vue, the rule grid, the
   detail panel, ListWidget and its add/remove, the FieldSpec registries),
   generated bindings (settables.ts and siblings), and the core side
   (validate.rs) for what an empty rule actually emits.
6. docs/process-journal/artifacts/plan-7-sdd/task-10-verdict.md finding 1
   (the surfacing artifact: help topics fabricated Add/Remove buttons that
   did not exist - your design makes them exist).

## Settled owner rulings (2026-07-22 S22; binding, not re-litigable; each becomes a D-entry recording the ruling with its rationale)

1. A fresh rule is an EMPTY SKELETON, invalid-until-filled; the existing
   diagnostics/inline-marker plumbing guides the user. No prefilled
   guesses (the declarative-batch boundary against input-time magic).
2. Remove has NO confirmation dialog (house ListWidget precedent;
   explicit save bounds the loss; undo/redo is deliberately v1.x -
   your design builds NOTHING toward it, but may note interactions).
3. Add APPENDS at the end; the new rule is auto-selected and its detail
   panel opens. Reordering stays the existing drag-reorder.
4. The buttons render the generic `editor-action-add`/`editor-action-remove`
   keys - ZERO new label keys. Any key-budget change needs an explicit
   ADR and an occurrence per editor-generic-action-keys; default is none.
5. No last-rule protection: deleting down to zero rules is legal state
   handled by existing semantics (core-83 passthrough with
   `unmatched: keep`, NoTrackRules diagnostic with `drop`). Verify what
   the editor surfaces for the zero-rule state and describe it.

## The design must RESOLVE (no fork may survive into planning; a fork that needs an owner call is NEEDS_CONTEXT in your final message, not a clause in the document)

- Exact UI: button placement/order on the bespoke grid per the ListWidget
  precedent; the remove button's disabled state (no selection?); keyboard
  reachability per the house's existing patterns.
- The concrete empty-skeleton TrackRule value: which fields, from the
  generated types; what validate.rs emits for it; verified empirically
  (run it or cite the code path with SYMBOL anchors, not bare line spans).
- Help-id strategy for the affordance: gui-helpid-equals-labelkey ties
  help-ids to labelKeys, and the generic action keys collide as anchors.
  Resolve (e.g. extend the existing editor-tracks-rules section topic
  content vs dedicated help-ids with new en+de topic files and the D62
  lockstep ripple) with rationale AND the rejected option's steelman.
  Any new/changed topic h1 follows conventions.yaml help-topic-h1-scheme.
- Tooltips for the two buttons: spec 8.3 baseline says every non-obvious
  control carries one - either specify the .tooltip attributes (recall:
  tooltips are attributes, zero new ids) or argue the buttons are obvious;
  if you conclude "not needed", the no-work-needed rule applies: verify
  the premise, in this case against spec 8.3's own wording.
- Detail-panel behavior for the fresh rule (panel state, focus target).
- Help-mode interaction: the new buttons under the pointer-scoped
  suppression (they are activation controls in the content area - what
  happens in help mode; conform to help-mode-suppression-pointer-scope).
- Diagnostics behavior on the fresh invalid rule: what fires, where the
  markers land (registries/config_path mapping) - verified, not assumed.
- e2e coverage plan: which spec files, which fixtures, the D62/D55 gate
  ripple enumerated (every new help-id -> two topic files, check-i18n
  consequences).
- The spec-8.2 amendment: exact, verbatim-ready wording.
- Serialization/API surface: confirm add/remove is pure frontend model
  mutation + existing canonical save (expectation: no IPC/wire change;
  if you find otherwise, that is a memo-worthy interface finding).

## Method duties

- SI-3 mkvtoolnix parity: classify explicitly (match / justified
  divergence / genuine gap) wherever meaningful; mkvtoolnix-gui has no
  declarative rule editor - if you rely on that, state it from the source
  at ~/Downloads/mkvtoolnix (cite file) or from running mkvtoolnix-gui
  semantics you can verify; the interactive-vs-declarative distinction is
  the load-bearing frame. Licensing boundary: behavior/facts fair game,
  never literal text/code.
- Every behavioral claim anchor-bound; file:line citations follow
  code-comment-line-citations-drift (prefer symbol anchors; verify at the
  CURRENT tree).
- ADR slots complete: decision, rationale, rejected alternatives EACH
  with its steelman, triggers created (name them for ROADMAP mirroring),
  interface changes (expected: none).
- D-numbering: D65 upward. Plan 8's parallel design owns D75 upward - if
  you need D75 or higher, STOP and report instead of colliding.
- Any version/tooling claim: registry-verified live, never from training
  memory.
- Proposed safeguards (guards, gates, enumerations) are not argued away
  in-design; they stay until built and measured redundant.

## Output structure (per the plan-7 template)

Context; decision log D65+; design sections; spec-amendment block; test
plan; triggers; open items - which must contain NO unresolved fork.

Constraints: read-only except your one output file; no git; never call
EnterWorktree/ExitWorktree or any session-relocation tool; absolute
paths; anything you run, run foreground.

Final message: at most 3 lines + the document path + any NEEDS_CONTEXT.
