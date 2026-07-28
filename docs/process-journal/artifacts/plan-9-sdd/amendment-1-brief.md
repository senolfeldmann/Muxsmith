# Plan 9 amendment 1 brief (owner rulings, 2026-07-28, after design and plan approval)

Controller-authored routing for two owner rulings that arrived AFTER both the
design and the plan were four-eyes approved and after the design was
owner-approved. Execution has not started; no task has been dispatched. The
amendment therefore runs through the same pairs that authored the artifacts:
the design's author amends the design and its original reviewer judges the
delta, then the plan's author amends the plan and its original reviewer judges
that delta.

## Ruling A: a feature's tests ship with the feature, never after it

**Owner, verbatim in substance:** "der e2e test, den will ich nicht nachgereicht
haben, wir schreiben keine Features und reichen tests nach", clarified minutes
later with "also der e2e test sollte auch jetzt kommen".

This overturns a CONTROLLER decision, not an owner one. During the design's fix
round I routed the two producer-less acceptance observables as "restate
honestly, add no e2e tests", on the ground that new scenarios sit outside the
ruled scope. The owner's standard is the opposite and it governs: this plan
creates two user-visible consequences, so this plan tests them.

**In scope now, both of them:**

1. **The GUI Run-gate consequence of D101's new error severity.** A profile
   carrying a bare `raw:` becomes an error-severity config diagnostic, and
   `BatchView`'s `hasErrors` (`src/views/BatchView.vue:282`) then disables the
   Run button with the `tooltip-errors` reason (`:303-304`). No e2e feeds
   BatchView an error-severity config diagnostic today. Add the scenario:
   assert `data-testid="batch-run"` (`:511`) is disabled and its tooltip
   attribute carries the errors reason.
2. **The branch D103 edits.** No e2e resolves `load_profile` with a
   parse-failure document. Add the scenario: a `profile: null` document whose
   `config_diagnostics` carries the single `parse-error` diagnostic, asserting
   the alert line surfaces that diagnostic's code - which is exactly what
   D103's code-keyed fetch must keep doing.

**The boundary that does NOT move, and why it is not in tension with the
above:** the owner's earlier ruling cut new test INFRASTRUCTURE (Vitest, a
`tauri::test`/`mock_builder` harness, a `src-tauri/tests/` tree) to 1.x. Both
scenarios above need none of it. The existing Playwright plus mock-IPC harness
already does exactly this shape: `e2e/smoke.spec.ts` mocks `load_profile`,
`validate_profile` and `dry_run` with `resolveWith(<document>)` (`:204-208`)
and already feeds non-empty `config_diagnostics` arrays (`:271`, `:384`).
Scenarios are in scope; infrastructure is not. State that distinction
explicitly in both documents, because it is the boundary a later reader will
otherwise misread in one direction or the other.

Consequence for the design: acceptance observables 5 and 6 gain real
producers, and the two "no producer, rides the v1.x harness item" paragraphs
in D101 and D103 are replaced by the named tests. The v1.x GUI-test-harness
ROADMAP item stays as it is - it covers `start_run`'s untested composition,
which these scenarios do not touch.

## Ruling B: the GUI identification session cache is out

**Owner:** the cache feature is unnecessary and reads as overengineering.

D93 currently does two separable things: it makes the seam take a caller-owned
`IdentifyCache` (`&mut`), and it gives the GUI one cache per app session in
`AppState` so a dry-run followed by a run reuses identifications. **Both go.**
The seam constructs its own `IdentifyCache` per call, which is exactly what all
four copies do today; `LiveIdentifier` keeps owning its cache and does not
become borrowing; the two test-site adaptations D93 listed
(`crates/muxsmith-core/tests/command_integration.rs:231` and `:493`) are not
touched; `AppState` gains no field.

**The consequence the design must handle, not hide.** D93 conformed the GUI to
spec 5.5, which reads: "Identification cache: in-memory per session, keyed on
path + mtime + size, **shared between dry-run and run**. On-disk cache is a
future candidate." (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:311`).
With the sharing removed, that sentence is false for the GUI, where a dry-run
and a run are separate calls. The spec is authoritative, so it does not get
left contradicted: the design amends the sentence to describe what the product
does - per-call in the GUI, per-process in the CLI - and records the amendment
in its spec-amendments section with the exact replacement text, like every
other S-entry.

**Recorded so it is not re-litigated as an oversight:** the cost the owner
accepted is that a GUI dry-run followed by a run identifies every file twice,
i.e. one `mkvmerge -J` process spawn per file per command rather than per
session. The ADR keeps the session cache as its rejected alternative with that
steelman.

## What this amendment does NOT change

- No other D-entry, no other acceptance observable, no task boundary beyond
  what these two rulings force.
- The serial sequencing ruling and the no-worktree ruling stand.
- The infrastructure boundary of ruling A above.
- Nothing about the two OUT items of the original kickoff (the 1.x GUI test
  harness for `start_run`'s composition, the IpcError funnel).

## Order of work

1. Design author (resumed) amends the design: D93 rewritten, its spec
   amendment added, D101 and D103's coverage paragraphs replaced by the named
   producers, acceptance observables 5 and 6 updated, amendment log entry.
2. Design reviewer (resumed) judges that delta only.
3. Plan author (resumed) amends the plan: Task 1 loses the cache work, the two
   e2e scenarios land in tasks, the acceptance map is corrected, the coverage
   map follows the design, amendment note.
4. Plan reviewer (resumed) judges that delta only.
5. Owner approves the amended plan; execution starts.
