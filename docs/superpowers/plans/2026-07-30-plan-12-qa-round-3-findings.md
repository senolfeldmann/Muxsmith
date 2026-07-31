# Plan 12: the owner QA round-3 findings, the unsaved-profile guards, and editor undo/redo

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **House deviation from the skill text:** progress NEVER enters this document. No box in this file is ever ticked; the checkbox syntax is structure, not a tracking surface. The tracker is `.superpowers/sdd/plan-12/progress.md`.
>
> **Execution starts only on the owner's plan approval** (standing gate; the same rule the Plan-8.5, Plan-9, Plan-10 and Plan-11 headers carry).
>
> **There is no design document for this package, deliberately and on the controller's recorded scoping decision.** Every fork is a surface or seam decision decidable against the tree; no wire format and no public interface changes shape. The decisions live in this plan's own **Decision register** below, the five ADRs are authored in Task 1's diff, and the plan review grades both. A later reader looking for `docs/superpowers/specs/...plan12-design...` will not find one, and none is missing. The normative residue does land as a file: `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`, carrying D106-D110 from Task 1 and D112 from amendment 1 (the owner's failed-load ruling of 2026-07-31, which arrived after Task 1 had closed).

**Goal:** land the owner's QA round-3 findings plus the two items his rulings during plan authoring added, in five work items: the settings language control gains a third "system language" state so the effective locale and the displayed value agree and the choice stays reversible (W1); a New action creates a blank profile in the editor, which today cannot be reached at all without hand-authoring a YAML file (W2); editor undo/redo across every model mutation, which also supplies the save-state signal the next item needs (W3); the discard guards that warn before unsaved editor content is replaced or the app quits (W4); and the user-facing documentation that describes what the four shipped (W5).

**Where this package sits, because it changes what "done" means.** This package does **not** close 1.0 scope and no sentence in it may be read as though it did. Tier-2 `owner-manual-qa-gates-the-1-0-release` is a standing precondition on the tag: the owner's manual pass is **currently STOPPED** on W2 and resumes once a build carrying it exists, further rounds are expected, and its output is first-class scope input in the three shapes he named (real bugs; behaviour he dislikes even where it matches the spec; v1.x items he decides belong in 1.0 after all). 1.0 scope is therefore unknown by construction while this plan runs. **No task in this plan bumps a version, prepares or creates a tag, or edits a release body**, and the plan close does not propose the tag.

**Architecture:** seven strictly serial tasks on `master` in the main worktree - no branches, no worktrees (ruling and reasoning in the sequencing section). Each task commits with an explicit pathspec. One full gate run, foreground, before the single push at the plan close; each task additionally runs the gate green before its own commit (the gate's binding site is pre-push; this plan has no merges). No task edits any house-knowledge YAML; ledger and ROADMAP writes are controller close actions.

**Tech Stack:** Rust workspace (toolchain pinned via `rust-toolchain.toml`), Tauri 2 / Vue 3 / TypeScript frontend, Playwright e2e with the in-repo mock+mount harness (`e2e/`), Fluent catalogs under `locales/`, markdown help topics under `help/`. **No new dependency of any kind, cargo or npm; no new gate part, CI job or runtime dependency; and no new Tauri capability permission.** Two of those three are measured rather than assumed: the capability claim rests on the dialog-permission finding below, and the no-dependency claim rests on D110's decision that the shell is TOLD its locale rather than resolving one, which is what keeps `sys-locale` out of `src-tauri`. The two tooling files this package edits gain content inside an existing check rather than a new one: `scripts/check-i18n.mjs` gains an allowlist entry, not a new check part, and `eslint.config.js` gains one rule entry inside its existing per-`.vue` rules block (amendment 1, D112's standing-guard decision), which `pnpm lint` already runs - so the no-new-gate-part claim holds for both.

## Scope

**In scope, five work items.** W1 and W2 are the owner's QA round-3 findings, both OWNER-RULED. W3 and W4 arrived as owner rulings during this plan's authoring and are recorded here as work items rather than as amendments, because the plan had not been approved when they landed. W5 is the documentation half of W2-W4.

**Explicitly OUT of scope, stated here rather than left to be inferred:**

- **Deriving a profile from a file the user selects whose structure is read out.** The owner ruled it a pre-1.0 item, and it gets its own package with its own design round, because it needs new core mapping machinery and a product-boundary ADR. **It must not be started, prepared for with speculative seams, or partially built here.** No task adds a code path, a parameter, a catalog key or a test for it. Two **information duties** about it are discharged in the decision register (D107-i, D108-i) and nothing else about it enters this plan.
- **A bundled template or example profile.** Not ruled in; recorded in the ROADMAP as staying unbuilt so it is not revived by implication.
- **A confirmation dialog for rule removal.** D66 (plan-7.5 design) removed it on the recorded ground that undo/redo, not a dialog, is the durable answer to accidental destruction. W3 builds that answer, which consumes the premise rather than reopening it. Spec 8.2's "Remove deletes the selected rule without confirmation" is **not weakened**, and no task adds such a dialog. The discard-guard family (W4) does not conflict with it: W4's triggers are whole-model replacement and app exit, never a single rule mutation, and D109 records the boundary explicitly.
- **Shell strings this package does not add.** Localizing the shell IS in scope for this package's own strings, by an owner ruling of 2026-07-30 that is general rather than about one dialog: **German translations always ship in the same change, without exception**, so "English with a recorded reason" is not an available disposition for any user-visible string this package introduces (D110). What stays out is the pre-existing surface: the four `close-abort-*` strings become localized as a side effect of the mechanism, because they go through the same lookup, but **no other pre-existing shell or config string is swept in** - `tauri.conf.json`'s window title and bundle strings are not locale-aware and this package adds none. Anything found there is SURFACED for routing, not fixed here.

## Consolidated requirement set

Five controller addenda accumulated on one brief, so the full set is restated here as the single contract. The reviewer's coverage walk runs over this list and over the acceptance map's halves.

| # | Requirement | Source | Implemented by |
|---|---|---|---|
| R1 | The settings language control gains a third option representing "no override" (system language), preselected on first run, so the effective locale and the displayed value agree and the choice stays reversible | ROADMAP round-3 finding 1, owner-ruled shape A | Task 2 |
| R2 | The sentinel a `<select>` uses for "system", and its mapping in BOTH directions | brief decision 1 | D106, Task 2 |
| R3 | Where the "absent means system locale" resolution rule lives: a shared seam or a deliberate duplication, decided with reasons | brief decision 2 | D106, Task 2 |
| R4 | What the control DISPLAYS while "system" is selected, including whether it names the resolved language | brief decision 3 | D106, Task 2 |
| R5 | The live-switch path when the user switches TO system | brief decision 4 | D106, Task 2 |
| R6 | The exact catalog strings for W1 in `locales/en/` AND `locales/de/` | brief decision 5 | Task 2 |
| R7 | The existing e2e assertion expecting the locale control to hold `en`: corrected or replaced, stated | brief decision 6 | Task 2 (refuted premise; see corrections) |
| R8 | The new test's mechanism for presenting a non-English system locale | brief decision 7 | D106, Task 2 |
| R9 | Whether `e2e/mocks.ts`'s default `locale` value changes | brief decision 8 | D106, Task 2 |
| R10 | The spec amendment for W1 | brief decision 9 | Task 1 |
| R11 | An ADR for W1, because D56 declared the settings write out of scope by name | brief decision 10 | Task 1 (D106) |
| R12 | A New action creates an empty profile in the editor | ROADMAP round-3 finding 2, owner-ruled shape A | Task 3 |
| R13 | The seed, MEASURED against the validator rather than reasoned about | brief decision 11 | D107, authoring section, Task 3 |
| R14 | The `currentPath` decoupling: what replaces each duty and what the validation watcher gates on instead | brief decision 12 | D107, Task 3 |
| R15 | Save with no path yet: a save dialog on Save, or a distinct Save-as | brief decision 13 | D107, Task 3 |
| R16 | Where New is offered; whether the editor's empty state is repaired; whether Batch's empty state also offers New | brief decision 15 | D107, Tasks 3 and 7 |
| R17 | The exact catalog strings for W2, both locales | brief decision 16 | Task 3 |
| R18 | Whether the in-app help topic changes | brief decision 17 | Task 7 |
| R19 | The spec amendment for W2 | brief decision 18 | Task 1 |
| R20 | An ADR for W2, because a recorded design position is reversed by an owner ruling | brief decision 19 | Task 1 (D107) |
| R21 | Losing an unsaved profile: opening another profile WARNS when unsaved changes exist | owner ruling, addendum 3 | D109, Task 5 |
| R22 | Switching tabs must not affect the editor's content at all, and therefore warns not at all | owner ruling, addendum 3 | D109, Task 5 (assertion only; already satisfied by `v-show`) |
| R23 | Closing the app WARNS when unsaved changes exist | owner ruling, addendum 3 | D109, Task 6 |
| R24 | The guard's shape: the confirmation precedes the destructive action, and its text names what is being overwritten | owner ruling, addendum 2 | D109, Tasks 5 and 6 |
| R25 | The precedence between the two reasons to block a close (a run in progress, unsaved changes), and whether the user sees one prompt or two when both hold | controller addendum 3, measurement 2 | D109, Task 6 |
| R26 | Every path that mutates the profile model, enumerated from the tree with the expression stated, and coverage per path | controller addendum 3 safeguard | authoring section, D108, Task 4 |
| R27 | Editor undo/redo across every editor mutation: field edits, rule add/remove including the unconfirmed delete, drag-reorder, list/map widget mutations | owner ruling, addendum 4; ROADMAP v1.x entry as the requirement set | D108, Task 4 |
| R28 | The save-state gate is DERIVED from the undo history, not maintained as a second boolean | controller addendum 4 | D108, D109, Task 4 |
| R29 | Undo granularity | addendum 4 decision 1 | D108, Task 4 |
| R30 | Whether saving clears the history or marks a position in it | addendum 4 decision 2 | D108, Task 4 |
| R31 | A depth limit, or none, with the memory consequence stated | addendum 4 decision 3 | D108, Task 4 |
| R32 | Keyboard shortcuts, and whether they follow platform convention per OS or one binding everywhere | addendum 4 decision 4 | D108, Task 4 |
| R33 | Whether undo/redo is also surfaced as visible controls, and where, with catalog keys in both locales | addendum 4 decision 5 | D108, Task 4 |
| R34 | What happens to the history when a different profile is opened | addendum 4 decision 6 | D108, Task 4 |
| R35 | Whether the derivation package's population would be ONE undoable step (information duty) | addendum 4 decision 7 | D108-i |
| R36 | Whether the chosen mechanism carries a populated unsaved profile (information duty) | addendum 1 | D107-i |
| R37 | The S22 reversal recorded as a reversal, with the old reasoning named | addendum 4 | Task 1 (D108) |
| R38 | SI-3 parity comparison for undo granularity against mkvtoolnix-gui, classified | addendum 4 | authoring section, D108 |
| R39 | Every user-visible string this package adds ships in German in the same change, the shell's included; no string is deferred or accepted in English | owner ruling, addendum 5 | D110, Tasks 2-7 |
| R40 | The package's user-visible string surfaces, enumerated from the tree with the expression stated, and each confirmed to ship in both locales in the task that introduces it | addendum 5 item 2 | authoring section, D110 |
| R41 | The gate's blind spot on shell strings, with the check that closes it proposed as part of this package and its residual coverage stated | addendum 5 item 3 | D110, Task 6 |
| R42 | The close decision is re-read after the user confirms, and a second prompt fires when the state changed - closing the residual the precedent sweep found, without reopening decision 5's four-variant table | owner ruling 2026-07-30 (one-pair amendment) | D109 decision 9, Task 6 |
| R43 | After a load that fails to parse, the editor keeps the "Selected profile" line and the parse error and hides the empty-state paragraph and the recents section; the two gates carry a condition stated once rather than inheriting a term a later task redefines | owner ruling 2026-07-31 (amendment 1) | D112, Task 4 |

## Global Constraints

- **Ground truth and precedence:** the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative on conflict, **except where Task 1 amends it under the owner's rulings**, which is the point of R10 and R19. Below it: `.superpowers/sdd/plan-12/plan-brief.md` and its five controller addenda; `docs/ROADMAP.md`'s "OWNER QA PASS, round 3" entry in the Pre-1.0 release gates section, in full, including both derivation-package rulings and the v1.x "Editor undo/redo, all operations" entry. The four house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`) are ground truth alongside them; cite entries by id. The ones that bind this package hardest: `editor-generic-action-keys` (the editor catalog budget is a hard boundary), `gui-closed-domain-dropdowns`, `core-83-zero-rule-keep-passthrough`, `gui-table-caption`, `comments-locate-by-symbol-never-by-line-number`, `tests-ship-with-the-feature-never-after`, `proc-04-spec-wins`, `proc-06-mkvtoolnix-parity`, `testing-si3-run-binary`, `proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`, `proc-proposed-safeguard-stays`, `proc-normative-count-recomputed`, `a-document-never-cites-a-line-number-inside-itself`, `a-search-whose-terms-come-from-memory-produces-a-false-absence`, `proc-sweep-surface-completeness`.
- **The gate as `BUILDING.md` enumerates it**, run foreground, no subsets, **before any push**. This constraint names the FILE and states no count deliberately. Per-task verification below names the subset each task must run green before committing; that subset is a task exit bar, not a gate substitute.
- **No design decision is re-opened, softened, or "improved".** A contradiction discovered on code contact is refuted with evidence or returned as NEEDS_CONTEXT, never silently absorbed.
- **Every fork in this plan is closed.** No task brief, verdict or fix-round dispatch may carry a design-latitude clause in either form: an explicit permission, or an omission - an unenumerated set in a normative position, a list ending open, a "one per X" with no X list, a step that requires inventing a name, a string, a key or a file that is not written down somewhere the implementer can read. A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** (options, costs against the named invariants, a recommendation) and is routed by the controller, never resolved at the keyboard (`proc-latitude-clause-boundary`).
- **No task edits any house-knowledge YAML** and no task edits `docs/ROADMAP.md` or `docs/process-journal.md`. A task that finds something ledger-worthy SURFACES it in its report; ROADMAP dispositions are close actions. **The editor catalog budget revision this package needs is stated in the decision register and is part of what the owner approves when he approves this plan** (the same route the 45 -> 46 revision took through the plan-7 design); the Tier-2 statement update is a controller close action.
- **A proposed safeguard stays** (`proc-proposed-safeguard-stays`): a guard, test, enumeration or check this plan proposes is removed only after it is built and MEASURED redundant, never argued away during authoring or review. This binds the mutation-path enumeration and its per-path coverage in particular.
- **Every test this package's own behaviour makes observable ships in the same task** (`tests-ship-with-the-feature-never-after`). The recorded exemption is narrow and does not apply here: new test INFRASTRUCTURE may be deferred, a scenario the existing infrastructure can already express may not. A describe-level `test.use({ locale })` is existing infrastructure (measured in the authoring section). **No task writes "coverage follows in a later plan" about behaviour this plan introduces**, and the scope argument may not be used to remove a test - it proves too much, since it would remove the test of every feature.
- **Absence-shaped verification steps carry three things**: the expression, the PRE-STATE run that makes it fire with an exact expected non-zero result, and the END-STATE run with its expected zero (`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`). Where the expression contains an enumerated set, that enumeration is derived from the artifact, never from recall, and is fired against a known-present member (`a-search-whose-terms-come-from-memory-produces-a-false-absence`).
- **Where the path from mutation to assertion crosses a FALLBACK, the red state must defeat the fallback rather than disturb its input.** A locale chain, a default branch, a retry, an `unwrap_or`: each supplies a plausible value for every mutation upstream of it, so an assertion made downstream is green under all of them. The handle: assert the single step without the chain, and pin one concrete value on the far side rather than a non-empty-and-not-the-key result, because a fallback's output is also non-empty and also not the key. **The trigger is readable in the artifact** - does a fallback clause sit between the thing mutated and the thing asserted? - and this plan carries three instances where it applies: the shell parity test (D110 decision 4, which an earlier draft of this plan got wrong in exactly this way), the frontend locale tests (whose de-only literals are what defeats `buildBundles`'s `[requested, en]` chain, stated at W1's producers rather than left to luck), and the e2e mock's own `get_settings` fallback (which a scenario-supplied `locale: null` would silently mask if the assertions did not read a de-only string).
- **Evidence lines carry pasted output** (`design-empirical-claims-reproducible`): every observed value in a task report is pasted from the run that produced it, never recalled, and never attributed to a command that was not the one run.
- **SI-3, the mkvtoolnix parity duty** (`proc-06-mkvtoolnix-parity`, `testing-si3-run-binary`): behavioural questions compare against mkvtoolnix-gui / mkvmerge by reading the source at `~/Downloads/mkvtoolnix` and running the binary, never from memory. The load-bearing distinction on record is interactive-versus-declarative-batch. The parity findings this plan needs are measured in the authoring section and are re-verifiable there; a task that disagrees with one returns it.
- **A borrowed precedent carries the CONDITIONS that made it correct, and synchrony is the one most easily dropped.** Copying a shape out of a reference implementation copies its guarantees only where its preconditions still hold; the commonest silent loss is a pair of adjacent statements becoming a pair separated by an `await`, which turns "read the current state" from a description of what was just written into a race. **The trigger is readable:** the precedent's two statements are adjacent and yours are not. **The handle:** name the condition in the same clause that cites the precedent, and where the condition is gone, carry the value across the gap explicitly instead of re-reading it. This plan's own sweep for the pattern is in the authoring section; it found the instance that produced this rule and two more that are answered rather than defective.
- **A comment never locates code by line number** (name the symbol; naming the file is fine) and **a document never cites a line number inside itself**. Both are owner-ruled Tier-2 entries and both bind every string, comment and document this plan prescribes, including `assert!` messages and Fluent values.
- **Two writers in one working tree share one git index**, so every commit in this plan is pathspec-scoped (`git commit -- <paths>`), and no second writer is dispatched while a task is live (`a-serial-ruling-binds-dispatch-concurrency-too`, `concurrent-writers-need-pathspec-scoped-commits`).
- **This plan hardcodes no model name and no commit-trailer string.** Commits and pushes on this repo are standing-authorized by the owner; agent commits are deliberately unsigned - `git -c commit.gpgsign=false commit ...` - with exactly one trailer, `Co-Authored-By: Claude <model> <noreply@anthropic.com>`, where `<model>` is the canonical model name **derived from that dispatch's explicit model parameter, never written as a literal here** (`agent-commit-trailer-set`; SI-4, restated in every dispatch that expects a commit). Stage files explicitly, **never `git add -A`**; the single push gets a `gh-log.md` entry.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, in this plan, in every code comment and in every string it prescribes. German orthography in Fluent catalogs and help topics is orthography, not an AI-tell glyph, and is copied exactly where it appears.
- **Implementer preamble, verbatim in every dispatch:** subagents never call session-relocation tools (EnterWorktree/ExitWorktree or any equivalent); absolute paths; foreground runs only; work on `master` in the main worktree.
- **How each task's `Read first` list is derived**, three questions, first answer settles it. (1) Does a step require the file to be OPENED to derive something this plan does not already state - an anchor, a symbol, a current string, a contract? If not, it is not a read-input; a file the task only WRITES INTO is a target and its content is enumerated in the Files list. (2) If yes: can this plan NAME the file in advance, or is it a member of a set a prescribed MEASUREMENT computes at execution time? Nameable -> `Read first`; measurement-computed -> not, because the measuring step IS its definition. (3) Is it a universal input already bound by these Global Constraints? `BUILDING.md` is named once by the gate clause and every task runs that gate, so no task repeats it for that role.

## Execution method (binding)

Subagent-driven development (`superpowers:subagent-driven-development`): a fresh implementer subagent per task, an independent reviewer per task grading against this plan, the plan brief with its five addenda, the ROADMAP entries they cite and the spec, and a whole-branch review at the plan close before the close actions. Progress lives in `.superpowers/sdd/plan-12/progress.md`.

## Model tiers (`proc-03-model-assignment`)

Every task reviewer runs the mid tier; the whole-branch review at the plan close runs the top tier, which is the ONLY role the top tier serves; the controller loop runs mid. The controller sets the model parameter explicitly at every dispatch - an omitted parameter inherits the session default, which is not an assignment. **No task in this plan qualifies for the cheap tier:** even Task 1, whose spec and ADR content is supplied here, must run a self-contradiction sweep over the spec and render five ADRs with honest rejected alternatives, which is judgment rather than transcription.

| task | tier | ground |
|---|---|---|
| 1 (normative documents: spec + D106-D110) | mid | two fenced spec replacements plus five ADRs whose slots this plan supplies; the self-contradiction sweep is an enumeration with a fired control |
| 2 (W1: the locale control) | mid | a shared seam extracted across three files, a sentinel mapped in both directions, and a new test whose locale mechanism is prescribed but whose fixtures are composed |
| 3 (W2: blank profile creation) | mid | the `currentPath` duty split across six call sites, a save-dialog flow conforming to a documented capture pattern, and seven new tests |
| 4 (W3: undo/redo + the derived save-state) | mid | a history over the existing single mutation funnel, a coalescing boundary rule, keyboard handling, and per-mutation-path coverage over an enumerated set |
| 5 (W4a: the frontend discard guards) | mid | a reusable confirm component plus two guarded call sites with a prescribed ordering, and an invariant assertion |
| 6 (W4b: the shell close guard, localized) | mid | a four-state decision function in Rust with its unit matrix, a second pure rule over the same enum with an exhaustive twelve-cell matrix, two new commands, a locale table plus a locale-aware lookup, a source-derived parity test split three ways with one prescribed red state each, and six catalog strings in two locales the shell's line parser constrains |
| 7 (W5: user-facing documentation) | mid | prose in two locales against a shipped surface, under the D62 help gate |

## Authoring-time verification (2026-07-30, source tree at `148f19f`)

Every value below is pasted from a run made at plan authoring. `git status --porcelain` at authoring showed exactly ` M docs/ROADMAP.md` (a parallel writer's in-flight edit), and `git diff --stat 2c04ac4 148f19f` shows one added plan document, so **no source file, catalog, help topic or spec changed between the reconnaissance the brief distils and these measurements**.

### The two measurements the brief demanded

**1. The validator's actual output on the candidate seeds.** Measured twice on two instruments. First through the CLI, which reaches `validate::config_diagnostics` by the same funnel the GUI command uses (`crates/muxsmith-cli/src/commands/validate.rs` calls `config_diagnostics_from_file`, which loads then calls `config_diagnostics`; `src-tauri/src/lib.rs`'s `validate_profile_model_body` calls `config_diagnostics` directly):

```
$ ./target/debug/muxsmith validate <seed> --json   # per seed, exit code pasted
S1 schema minimum      {"diagnostics":[{"code":"empty-extensions",...,"severity":"error"},
                                       {"code":"no-track-rules",...,"severity":"error"}]}   exit=2
S2 + one empty rule    {"diagnostics":[{"code":"empty-extensions",...,"severity":"error"},
                                       {"code":"empty-match-expression","config_path":"tracks[0].match",...,"severity":"warning"}]} exit=2
S3 pattern "" + ["mkv"] + one empty rule
                       {"diagnostics":[{"code":"empty-match-expression",...,"severity":"warning"}]}  exit=1
S4 pattern ".*" + ["mkv"] + one empty rule
                       {"diagnostics":[{"code":"empty-match-expression",...,"severity":"warning"}]}  exit=1
S5 passthrough (unmatched keep, no rules)
                       {"diagnostics":[{"code":"passthrough-profile",...,"severity":"info"}]}        exit=0
```

Second on the **model** path rather than the file path, because the seed is an in-memory object and the GUI command deserializes it from JSON exactly as this probe does. A throwaway crate outside the repo (path dependency on `muxsmith-core`, nothing in the repo touched) fed each seed as the JSON the IPC wire carries, called `validate::config_diagnostics`, and printed both the diagnostics and what `save::to_string` would write:

```
########## S1 schema minimum
--- json in: {"profile_version":1,"input":{"pattern":"","extensions":[]},"tracks":{"rules":[]}}
--- config_diagnostics (2 total):
    Error EmptyExtensions at input.extensions
    Error NoTrackRules at tracks.rules
########## S2 schema minimum + one empty rule
--- config_diagnostics (2 total):
    Error EmptyExtensions at input.extensions
    Warning EmptyMatchExpression at tracks[0].match
########## S3 empty pattern + one extension + one empty rule
--- config_diagnostics (1 total):
    Warning EmptyMatchExpression at tracks[0].match
########## S4 .* pattern + one extension + one empty rule
--- json in: {"profile_version":1,"input":{"pattern":".*","extensions":["mkv"]},"tracks":{"rules":[{"match":{}}]}}
--- config_diagnostics (1 total):
    Warning EmptyMatchExpression at tracks[0].match
--- save::to_string (yaml):
    |profile_version: 1
    |input:
    |  pattern: .*
    |  extensions:
    |  - mkv
    |tracks:
    |  rules:
    |  - match: {}
--- yaml round-trip equals model: true
########## S5 passthrough: unmatched keep, no rules
--- config_diagnostics (1 total):
    Info PassthroughProfile at tracks.rules
```

**What this decides, and it narrows the recommendation on record rather than confirming it.** The ROADMAP's measurement of the schema-minimum seed reproduces exactly (two errors, empty extensions and no track rules). But the recommendation "a seed following the existing empty-rule-plus-warning idiom" is **necessary and not sufficient**: S2 is that idiom applied to a schema-minimum base and it still carries `empty-extensions` at error severity, so Save is still dead on first use. A non-empty `input.extensions` is **forced by the validator**, which is why this measurement was owed. S4 is the chosen seed (D107). Its runtime companion is measured too: `mkvmerge --list-types` on this machine reports `Matroska audio/video files [mk3d mka mks mkv]` under `mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit`, so `mkv` raises no `UnknownExtension` at plan time either.

**2. What `currentPath` in `EditorView.vue` is really load-bearing for.** Expression and full output:

```
$ grep -rn "currentPath" src/ e2e/ --exclude-dir=.generated
src/views/EditorView.vue:50:// validate state (`currentPath`, `diagnostics`, the ipc-error pair)
src/views/EditorView.vue:57:// `currentPath` (only Open's own IPC round trip ever sets it): a bare-
src/views/EditorView.vue:119:const currentPath = ref<string | null>(null);
src/views/EditorView.vue:185:  () => !model.value || !currentPath.value || hasErrors.value || saving.value || opening.value,
src/views/EditorView.vue:191:// whenever the held model changes. Gated on `currentPath` -- see the doc
src/views/EditorView.vue:202:  if (!currentPath.value || !value) {
src/views/EditorView.vue:231:    currentPath.value = path;
src/views/EditorView.vue:267:  if (saveDisabled.value || !model.value || !currentPath.value) {
src/views/EditorView.vue:273:    await saveProfile(currentPath.value, model.value);
src/views/EditorView.vue:493:    <p v-if="currentPath">
src/views/EditorView.vue:494:      {{ $t("batch-profile-current", { path: currentPath }) }}
src/views/EditorView.vue:498:      v-if="!currentPath && recents.length"
src/App.vue:237:             mid-run, and EditorView's open profile/diagnostics/currentPath
e2e/editor-rule-add-remove.spec.ts:8: *   `currentPath`, which a bare mount never sets -- so these cases exercise
```

**Six load-bearing duties, one write site, and two live documents that describe them.** By symbol rather than by line: `currentPath` is written in exactly one place, `openPath`; it is read by (a) `saveDisabled`, as a precondition on Save; (b) the `watch(model)` validation gate; (c) `doSave`'s own re-guard; (d) `doSave`'s save target, the only duty that genuinely needs a path; (e) the template's open-path line, through `batch-profile-current`; (f) the template's recents-section gate. `src/App.vue`'s comment mentions it descriptively and stays true. `e2e/editor-rule-add-remove.spec.ts`'s header states the validation gate as the reason its bare-mount cases need no IPC mock, and **that sentence is falsified by this package**, so the file joins Task 4's Files list as a named region.

**The ROADMAP's framing is one duty off, and the difference matters.** It records `currentPath` as doubling as "where to save" and as "may I edit and validate at all". Measured: **editing is gated on `model`, not on `currentPath`** - the whole editing surface sits inside `<template v-if="model">`, and every widget path writes through it regardless of any path. What `currentPath` actually gates is validation, the Save button, the path display and the recents affordance. So the decoupling is a four-way split, not a two-way one, and D107 enumerates a replacement per duty.

### Measurements that decide the rest

- **The dirty state does not exist to be read.** `grep -nEi "dirty|isDirty|unsaved|modified" src/views/EditorView.vue` returns nothing; the same expression over `~/Downloads/mkvtoolnix/src/mkvtoolnix-gui/merge/tab.cpp` returns its `hasBeenModified`/`savedState` lines, so the pattern demonstrably matches a change-tracking implementation when one is present. Addendum 3's measurement 3 is confirmed.
- **The mutation-path enumeration, derived from the tree.** Expression and full output:

  ```
  $ grep -nE '^\s*model\.value = ' src/views/EditorView.vue
  233:    model.value = doc.profile ?? undefined;
  309:  model.value = { ...(model.value ?? ({} as Profile)), [key]: value } as Profile;
  316:  model.value = {
  407:  model.value = { ...model.value, tracks: { ...model.value.tracks, rules: next } };
  424:  model.value = {
  454:  model.value = {
  471:  model.value = {
  ```

  Seven assignments, named by their enclosing symbol: `openPath` (a load, not an edit), `setFieldValue`, `setTracksUnmatched`, `setRuleValue`, `onDrop`, `addRule`, `removeSelectedRule`. **The mutation set is therefore those six functions**, and every widget-level write (`ListWidget`, `PropertyMapWidget`, `KeywordOrBlockWidget`, `StringListWidget`, `SectionWidget`, `OptionalFlagWidget` each assign their own `defineModel` prop) reaches the profile only through one of them by emitting upward. That is what makes a single funnel possible at all.

  **The blind spot this expression has, checked rather than assumed** (`proc-sweep-surface-completeness`): it sees only whole-value assignments, so an in-place mutation would be invisible. Second expression, aimed exactly there: `grep -nE 'model\.value\.[A-Za-z_]+ *=|model\.value\.[A-Za-z_.]*\.(push|splice|pop|shift|unshift|sort|reverse)\(' src/views/EditorView.vue` returns nothing, **exit 1**. Its fire, against a synthetic file containing `model.value.input = y;` and `model.value.tracks.rules.push(x);`, matches both lines and exits 0, so the empty result is a real absence rather than a broken pattern. Second blind spot: an external writer through the `defineModel` prop. `grep -n "<EditorView" -A2 src/App.vue` returns `<EditorView v-show="activeView === 'editor'" />` with no `v-model`, so App.vue never writes the editor's model.
- **Tab switching already preserves editor content, and a test already asserts it.** `src/App.vue` mounts all three views with `v-show` and its comment says so explicitly ("EditorView's open profile/diagnostics/currentPath state (Task 13) survives a switch to Jobs and back"). `e2e/smoke.spec.ts` carries the case "the editor tab stays mounted across a switch to Jobs and back (v-show, not v-if)", which fills the pattern field, navigates to Jobs and back, and re-asserts the field value and the open path. Addendum 3's measurement 1 is confirmed: **R22 needs no mechanism**, and what the package owes is the unsaved-state half of that invariant.
- **A window-close handler already exists, and its decision function is already unit-testable.** `src-tauri/src/run.rs` carries `on_close_requested` (matching `WindowEvent::CloseRequested`, calling `api.prevent_close()`), the two-variant `CloseDecision` enum, and `close_decision(state)`, which today reads only the run slot through `lock_active`. Its existing unit tests are `close_decision_lets_an_idle_window_close_normally` and `close_decision_confirms_while_planning_and_while_running`. `AppState` (in `src-tauri/src/lib.rs`) already carries an `AtomicBool` field (`quit_after_finished`), so a second flag follows an existing shape. Addendum 3's measurement 2 is confirmed, and R25 is a real decision this plan settles (D109).
- **The shell's dialog strings are English-only by construction, which the owner's ruling now forbids.** `run.rs` reads them through `ftl_message`, a single-line lookup over `include_str!("../../locales/en/gui-common.ftl")` - the one and only catalog read in `src-tauri` (surface derivation below); `locales/de/gui-common.ftl`'s own header says the `close-abort-*` strings "are not yet shown to a de user; kept single-line and translated for parity and a later shell i18n". The Rust test `close_abort_strings_resolve_from_the_ftl_catalog` enumerates the four keys and pins `close-abort-title` to `Abort running jobs`; a companion test pins that `ftl_message` never prefix-matches. Consequences for W4: the keys must stay single-line and column-0 in **both** locales, that test's enumeration is a named region Task 6 extends, and the lookup itself becomes locale-aware (D110).
- **The house pattern for a locale-aware embedded catalog, which D110 conforms to rather than reinventing.** `crates/muxsmith-cli/src/i18n.rs` carries `const LOCALES: &[(&str, &str, &str)]` - one row per locale, `(primary subtag, cli.ftl, diagnostics.ftl)`, each `include_str!`ed - and `Renderer::new(locale: Option<&str>)` resolves `--locale`, then `sys_locale::get_locale()`, then `en`, collapses the tag to its primary subtag, and builds the per-message fallback chain `[requested, en]` deduplicated. Its own doc records why the table is hand-written: "`include_str!` is compile-time and has no glob form - the accepted asymmetry with the frontend's zero-code glob". `sys-locale` is a dependency of `muxsmith-cli` only, not of `src-tauri`.
- **THE STRING-SURFACE SET, and it takes TWO expressions because each is blind to what the other sees.** The pair is published the way Task 4's mutation enumeration publishes its own, and neither alone produces the set.

  **E1, the loader-call form**: `git ls-files -- '*.rs' '*.ts' '*.mjs' '*.vue' | grep -vE '^e2e/|tests?/' | xargs grep -nE '(include_str!|import\.meta\.glob|readFileSync|join)\(.*(locales|help)'`. Output, nine lines in four files: `crates/muxsmith-cli/src/i18n.rs` at four sites (`include_str!` of the en and de `{diagnostics,cli}.ftl`) - the **CLI surface**; `scripts/check-i18n.mjs` at three (`join(ROOT,"locales")`, `join(ROOT,"help")`, a `readFileSync`) - the **gate**, not a surface; `src-tauri/src/run.rs` at one (`include_str!` of `en/gui-common.ftl`) - the **shell surface**; `src/help/topics.ts` at one (`import.meta.glob("../../help/*/*.md"`) - the **help surface**.

  **E1's blind spot, and it is where a real surface hides:** the pattern requires the path on the same line as the opening paren, so a call whose argument array wraps to the next line is invisible - which is exactly the frontend's own loader. **E2, the path-literal form, aimed there**: the same file set with `grep -nE '"[^"]*(locales|help)/'`. It returns `src/i18n/index.ts:18`, the wrapped argument array `["../../locales/*/gui-*.ftl", "../../locales/*/diagnostics.ftl"]` - the **frontend surface**, the largest one in the package - plus the four CLI sites, the shell site, the help site, two error-message templates and two import paths in `check-i18n.mjs` and `App.vue`/`HelpSidebar.vue`, and one doc comment, all classified by reading. **E2 is blind where E1 is not:** `join(ROOT, "locales")` has no trailing slash, so E2 misses the gate's own three sites.

  **Controls, and what each does and does not prove.** E1 over `src/views/EditorView.vue` returns 0 and E2's pattern matches a synthetic `"../../locales/x.ftl"` line, so both discriminate a loading site from an ordinary one. **Neither control proves completeness** - a fired control shows the pattern works, not that the search surface was whole - which is why the set is the UNION of two differently-shaped expressions plus one member neither can reach: `tauri.conf.json`'s window title and bundle strings, which are values rather than paths and are not locale-aware at all.

  **Five shipped surfaces, therefore:** frontend catalogs (glob, all locales), help topics (glob, all locales), CLI catalogs (a hand-written two-row table), the shell (one file, en only before this package), and `tauri.conf.json`'s non-localized chrome. **This package adds strings to exactly three** - frontend catalogs (W1, W2, W3, W4a), the shell (W4b), help topics (W5) - and none to the CLI catalogs or to `tauri.conf.json`, which is checkable against the seven Files lists rather than asserted.
- **What the gate actually covers, which refines the blind-spot diagnosis rather than confirming it.** `check-i18n.mjs`'s cross-locale parity check runs over `referenceCatalogFiles`, which is **every** `.ftl` in `locales/en/` (its own comment: "Scope here is deliberately ALL `.ftl` files in locales/en/, not `catalogFiles`"), so `gui-common.ftl` is included and **the de shell strings' EXISTENCE is already gated**. The blind spot is therefore not a missing German string; it is that the shell **never reads any locale but en**, so those de values are shipped dead - which is exactly how an English-only shell dialog passed a green i18n gate. Two further facts the same file yields: its unused-id check carries an explicit allowlist `RUST_ONLY_IDS` naming the four `close-abort-*` keys as "consumed by src-tauri's own `include_str!` lookup in run.rs, never by the frontend", so **this package's six new shell ids must join that enumeration** or be reported as unused (warning-only, but it is an enumeration this change falsifies); and that check is warning-only while the parity check is a hard failure.
- **The frontend already holds exactly one authoritative locale value.** `applyLocale` in `src/i18n/fluent.ts` sets the `shallowRef` `currentLocale` and is called from precisely two places, `main.ts`'s bootstrap (before mount) and `SettingsDialog.save()`'s live switch. Its only current consumer is `HelpSidebar.vue`, which passes it to `topicHtml`. So a second consumer costs one watcher and no new resolution rule.
- **The frontend confirm route, and a permission assumption refuted before it was made.** `@tauri-apps/plugin-dialog@2.7.1`'s `confirm(message, options)` does **not** invoke a `confirm` command: its implementation routes through `messageCommand`, i.e. `invoke('plugin:dialog|message', ...)`, and returns `result === okLabel`. So the permission it needs is `dialog:allow-message`, not the `dialog:allow-confirm` the schema also offers, and a mocked test would have to return the exact ok-label STRING rather than a boolean. `src-tauri/capabilities/default.json` grants `dialog:allow-open`, `dialog:allow-save`, `clipboard-manager:allow-write-text`, `os:default`, `fs:allow-write-text-file`, `core:default`, `core:event:default` - not `dialog:allow-message`. **D109 chooses the in-app `<dialog>` route instead, so no capability changes and the ROADMAP's "no new capability" cost statement stays true.**
- **The save-dialog capability is granted and its house pattern is documented.** `dialog:allow-save` is in the capability file; `src/components/RunHistory.vue`'s `saveLog` carries the capture-state-before-the-dialog-gap pattern in so many words ("Captured before the dialog gap: the native save dialog can stay open indefinitely, and the user may select a different job meanwhile"), builds a `defaultPath` from ids, and sources its filter name from Fluent. W2's save flow conforms to it.
- **Catalog inventory, and two in-tree counts that are ALREADY stale.** `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/<loc>/<file>` returns, identically for en and de: `gui-editor.ftl` **46**, `gui-settings.ftl` **8**, `gui-common.ftl` **38**. The 46 decomposes exactly as Tier-2 `editor-generic-action-keys` records it: 42 registry `labelKey` ids (`grep -oE 'labelKey:\s*"[a-z][a-z0-9-]*"' src/editor/registries.ts | sort -u | wc -l` -> 42), plus `editor-save-note`, plus `editor-action-add`/`editor-action-remove`, plus `editor-track-rule-order` (D59, the 45 -> 46 revision). **Two live comments still say 45**, in `src/views/EditorView.vue` and in `e2e/smoke.spec.ts`; both are files this package edits, and both are named regions in their tasks' Files lists. No test asserts the count.
- **The existing locale assertions - TWO of them - and the brief's premise about them refuted.** `grep -rn "toHaveValue" e2e/*.ts` returns two hits on the locale control, both inside `test.describe("german locale")`'s case "selecting German in the settings dialog saves it, and it renders the German catalog on the next start": `await expect(localeSelect).toHaveValue("en")` before the save, and `await expect(reloadedLocaleSelect).toHaveValue("de")` after the reload. The first runs under `e2e/mocks.ts`'s default `get_settings` (the scenario mocks only `detect_mkvmerge`), which returns `locale: "en"` - a concrete string; the second runs against the case's own `DE_SETTINGS` mock, whose `locale` is `"de"`. **Their disposition is identical and neither changes:** under shape A a stored `"en"` still displays as `"en"` and a stored `"de"` still displays as `"de"`, so both are blind to the null case rather than wrong about their own, and the new test is what covers the state they cannot see.
- **A describe-level locale override works and does not disturb its siblings**, measured rather than reasoned. Playwright 1.61.1's own type documentation for the option says "Locale will affect `navigator.language` value". A probe outside the repo (its own config, `use: { locale: "en-US" }`, three tests) printed:

  ```
  BASELINE navigator.language = en-US
  OVERRIDE navigator.language = de-DE      # inside test.describe(...) { test.use({ locale: "de-DE" }) }
  SIBLING  navigator.language = en-US      # a later describe with no override
  3 passed
  ```

  So D29's deliberate English pinning survives, and no config change is needed.
- **`focusout` bubbles to an ancestor, and Playwright's `fill()` dispatches both `input` and `change`.** Same probe harness:

  ```
  PROGRAMMATIC: ["root:focusout","root:focusout"]     # focus moved a->b inside the subtree, then out of it
  PLAYWRIGHT FILL: ["fill:input","fill:change","fill:root-focusout"]
  ```

  A `blur` listener on the same ancestor logged nothing, confirming `blur` does not bubble while `focusout` does. This is the mechanism D108's coalescing boundary uses, and it tells a test writer that closing a coalescing window needs an explicit focus move.
- **Text widgets commit per keystroke.** `src/editor/widgets/TextWidget.vue` binds `v-model="model"` on both its `<input>` and its `<textarea>`, so Vue's default `input`-event binding makes every keystroke a model write. This is the fact R29 turns on.
- **mkvtoolnix parity, three findings, each measured.**
  - **New is the multiplexer menu's FIRST entry, before Open and Save, on Ctrl+N, and the empty state offers both paths.** `src/mkvtoolnix-gui/forms/main_window/main_window.ui` adds `actionMergeNew` before `actionMergeOpen`, `actionMergeSave` and `actionMergeSaveAs`; the action carries `<string>&amp;New</string>` and `<string>Ctrl+N</string>`; `merge/tool.cpp` connects it to `Tool::appendNewTab`, which creates a fresh tab in memory. `forms/merge/tool.ui` carries the empty-state pair `newFileButton` ("&New") and `openFileButton` ("&Open settings") under the text "No multiplex job has been opened yet." and an instruction naming the menu, the buttons and drag-and-drop.
  - **Save falls back to Save-as when no filename exists.** `merge/tab.cpp`'s `Tab::onSaveConfig` begins `if (p.config.m_configFileName.isEmpty()) { onSaveConfigAs(); return; }`, and `onSaveConfigAs` opens `Util::getSaveFileName`. This is the parity precedent D107 adopts for R15.
  - **There is NO undo/redo anywhere in mkvtoolnix-gui.** `grep -rn -i "undo" src/mkvtoolnix-gui/merge/` returns nothing (exit 1) while `grep -rli "multiplex" src/mkvtoolnix-gui/merge/` returns 7 files, so the surface is real and the pattern discriminates. `grep -rn "QUndoStack|QUndoCommand|QUndoGroup" src/` returns nothing (exit 1) while `grep -rl "QMessageBox" src/mkvtoolnix-gui/` returns 15 files, so the Qt-class pattern also discriminates. The only `edit-undo` uses are ICONS on `actionHeaderEditorReload` and `actionChapterEditorReload` (re-read from file) and the header editor's per-value Reset button. Its answer to accidental destruction is instead the modified-tab close warning in `Tool::closeTab`, gated on `hasBeenModified()` and on the `m_warnBeforeClosingModifiedTabs` preference (whose Muxsmith counterpart is parked as v1.x in `gui-17`). **Classification: a genuine gap in the reference tool for W3, so granularity has no parity model and D108 decides it on its own reasoning; and a direct parity match for W4's save-state gate, whose `currentState() != savedState` shape D108 adopts.**
- **THE BORROWED-PRECEDENT SWEEP, and its result is not empty.** Expression, derived from what this plan actually cites rather than from a list of what a precedent can be called: `grep -nE '(Tab::|Tool::|actionMerge|hasBeenModified|savedState|currentState|setDefaults|saveLog|newFileButton|openFileButton)'` over the plan, whose alternation is the set of reference-implementation symbols the plan names anywhere. **Fired control:** the same expression over a synthetic line carrying `Tab::onSaveConfig` matches, so an empty result would be a real absence. Five borrowed precedents sit in normative positions, each classified by one question - is the original's correctness underwritten by synchrony that the translation does not have?
  - **`Tab::onSaveConfig`'s `savedState = currentState()` after the write** (D108 decision 3). **YES, and this is the instance that produced the rule above.** Fixed by marking the captured profile; Task 4 Step 1c carries both the expression and the condition.
  - **`Tab::onSaveConfig` delegating to `onSaveConfigAs` when the filename is empty** (D107 decision 5). **YES on the condition, already answered.** `Util::getSaveFileName` is a blocking modal, so the reference flow has no gap at all; the translation's `await saveDialog(...)` opens one. The compensation is already fenced and is the reason it is fenced: Task 3 captures `profile` and the needs-path flag before the first await, conforming to `RunHistory.saveLog`'s in-repo pattern, which is itself already async and therefore imports no synchrony to lose.
  - **`Tool::closeTab`'s warn-before-closing-a-modified-tab** (D109). **Condition is modality, not synchrony, and it holds on one surface by construction and not on the other.** The in-app `<dialog>` is inert by specification while `showModal()` is up, so the model cannot move between reading `dirty` and receiving the answer; the Open and New guards are therefore closed. The shell's OS dialog does leave the webview live, so a read-then-ask gap exists there, and it has **three** directions rather than the two that are benign:
    1. computed dirty, stays dirty - the dialog said so, nothing is lost silently;
    2. computed clean with no run - the window closes in the same event, leaving no window to edit in;
    3. **computed clean WITH a run active** - `close_decision` returns `ConfirmAbort`, the abort dialog goes up through its callback and speaks only about running jobs, the user edits the profile while it is up so `set_editor_dirty(true)` lands, and confirming runs `abort_and_quit` **without re-reading the decision**. The app exits and the unsaved changes are gone with no dialog having mentioned them.

    **Direction 3 was a residual and is now RULED.** Its shape is the pre-existing D31 flow, but its consequence was new: before this package there was no editor state the close was required to protect, and R23 is exactly the requirement it defeats - in the data-loss direction the owner's save-state gating exists to avoid. The two closing costs went to the owner as a decision rather than as a defect discovered later, and **he chose to re-read the decision after the confirm and prompt again when the state changed**; the alternative, widening the abort dialog's text to mention changes that may not exist, was not taken. **The design is D109 decision 9**, which extends decision 5 rather than reopening it: the four-variant table stays exactly as it is and becomes the vocabulary the re-read speaks in.
  - **`hasBeenModified() { return currentState() != savedState; }`** (D108 decision 4). **NO.** A pure comparison in the reference and a pure `computed` here; nothing is read across a gap.
  - **`actionMergeNew` before `actionMergeOpen`, and the empty state naming both paths** (D107 decisions 6 and 7). **NO.** Static UI structure with no timing content.
- **A snapshot is small, and the method is named because the figure moves with it.** A history entry is `JSON.stringify` of the model the frontend holds, which arrived over the wire as serde's own output with every default omitted (D48). Measured in that form, compact, through `muxsmith-core`'s own `load` plus `serde_json::to_string`: the chosen seed is **101 bytes**, and the README's four-rule example - with `pattern: ".*"` supplied so it loads at all - is **419 bytes**. For comparison, and to keep the units apart, compact JSON of that YAML block as written is 492 bytes and 507 with the pattern line added. **An earlier draft of this plan said 430, and the cause was a truncated fixture** - the extracting `sed` range dropped the example's last line - not a different serialization; recorded because the wrong figure came from an instrument that did not do what its description said, the same class as the surface-set finding above. This is the memory statement R31 owes: 100 entries of a profile this size is under 50 KB, and a pathological 20 KB profile still bounds the history at 2 MB.
- **Decision numbering.** The highest D-number in use across `docs/superpowers/specs/*.md`, `docs/ROADMAP.md` and the four house YAML files is **D105**; `grep -rn "\bD10[6-9]\b"` over the repo returns nothing. This package takes **D106, D107, D108, D109, D110**; the same expression widened to `\bD1(0[6-9]|1[0-9])\b` also returns nothing, and its fired control is `\bD10[0-5]\b`, which matches in `docs/ROADMAP.md`. The README's "How this got built" paragraph carries no decision figure after Plan 10, so nothing outside the ADR file consumes the series.
- **The apply-suggestion path writes the profile file behind the editor's back.** `src/views/BatchView.vue` loads the profile, calls `applySuggestion`, then `saveProfile(selectedProfile.value, updated)`. So a suggestion applied in Batch rewrites a file the editor may be holding with unsaved changes. This is pre-existing and unchanged by this package; it is SURFACED, not fixed (close actions).

### Corrections to the brief and to the ROADMAP found at plan authoring (`proc-57-briefs-not-ground-truth`)

None of the seven changes a ruling; three change what a task does, and one is a correction of this plan's own draft.

| # | Statement | Reality |
|---|---|---|
| 1 | Brief decision 12 and the ROADMAP: "`currentPath` doubles as 'where to save' and as 'may I edit and validate at all'" | **Editing is gated on `model`, not on `currentPath`.** The editing surface sits inside `<template v-if="model">`. `currentPath` gates validation, the Save button, the path display and the recents affordance - four duties, not two - and D107 replaces each one by name. The item is still the actual work of W2; the split is just wider than recorded. |
| 2 | Brief decision 6: "the existing e2e assertion ... asserts the defect as correct behaviour" | **It is blind, not wrong, and it stays unchanged.** Its scenario mocks a concrete stored `locale: "en"`, and under shape A a stored `"en"` still displays as English. The ROADMAP's own body concedes exactly this ("It passes legitimately under its own mock and would pass identically under `locale: null`"), which the headline sentence overstates. The blindness is what the new test closes; correcting or replacing the assertion would change a true statement. |
| 3 | ROADMAP: the recommended seed follows "the existing empty-rule-plus-warning idiom" | **Necessary, not sufficient.** Measured: that idiom on a schema-minimum base still carries `empty-extensions` at error severity, so Save is still dead. A non-empty `input.extensions` is forced by the validator. The measurement wins and the seed is S4. |
| 4 | Reasonable assumption about the frontend confirm route: the dialog plugin's `confirm` needs `dialog:allow-confirm` | **It needs `dialog:allow-message`**, because the JS `confirm` is a wrapper over `invoke('plugin:dialog|message', ...)` compared against the ok label. Refuted before it was relied on; D109 avoids the question by using the in-app `<dialog>` house pattern, so no permission changes at all. |
| 5 | Tier-2 `editor-generic-action-keys` records the editor catalog budget as 46 | **True in the ledger and stale in the tree's own comments.** `gui-editor.ftl` carries 46 ids, and two live comments (`src/views/EditorView.vue`, `e2e/smoke.spec.ts`) still say 45, having missed the D59 revision. Both are inside files this package edits and are repaired as named regions. |
| 6 | This plan's own first draft, and the controller's framing of it: the i18n gate "cannot see the shell", which is how an English-only string became shippable | **Half right, and the half that is wrong changes what the safeguard must check.** The parity check DOES cover `gui-common.ftl` for every locale, so a missing German shell string is already a hard gate failure. What no check covers is that the shell READS only en, so the German values ship dead. A safeguard aimed at key existence would therefore have been green before and after, testing nothing; the one this plan proposes exercises the shell's own lookup per locale. Recorded as a self-correction because the first draft accepted the English-only shell with a recorded reason, which the owner then rejected as a general rule. |
| 7 | ROADMAP round-3 finding 2: "The documented first-run story is hand-authoring: the README's ... copy-pasteable example" | **That example does not load.** `./target/debug/muxsmith validate` on the README's first YAML block returns `[error] input: The profile could not be parsed: input: missing field 'pattern' at line 4 column 3`, exit 2: `Input.pattern` has no serde default and the block omits it. The README's second YAML block validates clean (exit 0, one `passthrough-profile` info). This strengthens the finding rather than changing it: the documented alternative to a New action is itself broken. **Its disposition is already RULED and is not this plan's to route** - `docs/ROADMAP.md` at this plan's own baseline carries it in the Docs-accuracy section with the owner's ruling attached (a serde default for `Input::pattern` was rejected as magic; the example gains the explicit line) and its vehicle named as Plan 11's fix round. This plan therefore cites the ruled entry and does nothing else with it; the measurement above stays as an independent confirmation of a record that predates it, which is also why the seed's own `pattern` is written out rather than defaulted. |

## Decision register

The forks the brief and its addenda name, each settled. This is the coverage ground truth the plan reviewer walks alongside the requirement table. Task 1 renders D106-D110 into the decisions file; the rejected alternatives and their steelmen below are the ADR content, not a summary of it. **D112 arrived after Task 1 had closed** (owner ruling 2026-07-31, amendment 1) and is rendered into that same file by the amendment rather than by a task, so the file carries D106-D110 and D112.

### D106: the settings language control is three-state, with the empty string as the "no override" sentinel (W1, R2-R9, R11)

**Decision.**

1. **Sentinel: the empty string**, held in a named module constant `SYSTEM_LOCALE = ""` in `SettingsDialog.vue`. Mapping, both directions and both fenced in Task 2: load is `form.locale = baseline.locale ?? SYSTEM_LOCALE`; save is `locale: form.locale === SYSTEM_LOCALE ? null : form.locale`. **Ground: it is the house pattern for "no override" on the sibling nullable field in the same component** - `mkvmerge_path` already loads as `baseline.mkvmerge_path ?? ""` and saves as `form.mkvmergePath.trim() === "" ? null : ...`. A second reason: the empty string cannot be a BCP-47 tag, so it can never collide with a real locale value.
2. **The resolution rule gets a shared seam**, `effectiveLocale(saved: string | null): string` exported from `src/i18n/index.ts`, the module that already owns locale negotiation (`primarySubtag`, `buildBundles`). `src/main.ts`'s `resolveLocale` and `SettingsDialog.vue`'s live switch are its two callers. **Ground: the defect being repaired IS one nullable field read with two different fallbacks in two files that nothing reconciles.** A deliberate duplication would re-create it with a new constant.
3. **The control displays a plain "System language" and does not name the resolved language.** Ground: naming it needs either `Intl.DisplayNames` (whose own output needs a locale argument, producing a mixed-language label) or a catalog key per language, which exists only for the two shipped locales - so the option label would break precisely on the systems the rejected alternative already failed on (neither German nor English). The resolved language is observable anyway, because the whole interface renders in it, and the hint is amended to say what the option does.
4. **The live switch resolves through the seam:** `if (next.locale !== baseline.locale) applyLocale(effectiveLocale(next.locale))`. The comparison stays on the raw nullable saved values, so a `null` -> `"en"` or `"de"` -> `null` transition fires and a settings save that leaves the language alone does not. The existing `next.locale !== null` type narrowing and its comment disappear, because `effectiveLocale` returns `string`.
5. **The new test presents a non-English system locale with a describe-level `test.use({ locale: "de-DE" })`** in `e2e/locale-switch.spec.ts`, reusing that file's own `de()`/`en()` helpers. Measured: it changes `navigator.language` for that describe only and leaves siblings on the config's `en-US`, so plan-5 D29's deliberate English pinning is untouched and `playwright.config.ts` is not edited.
6. **`e2e/mocks.ts`'s default `locale: "en"` does not change.** Ground: every scenario that does not mock `get_settings` inherits it, so flipping it to `null` is a suite-wide fixture-semantics change (from "an explicit English override" to "no override"), and it would silently invalidate the one existing assertion on the control. The new test supplies `locale: null` in its own scenario, which is the additive route.
7. **Both existing locale-control assertions stay unchanged** - `toHaveValue("en")` before the save and `toHaveValue("de")` after the reload - on correction 2's measurement and their shared disposition: each asserts the display of a concrete stored value, which shape A does not change.
8. **An out-of-band stored locale outside {`null`, `en`, `de`} is not handled.** The UI can no longer produce one: the option set is exactly those three and save writes only those. A value from a hand-edited settings file renders as an unmatched select, and the shipped fallback chain still renders the interface correctly (`buildBundles` falls through to `en`). No task adds handling, a migration or a test for it.

**Rejected alternatives.**

- **Initialise the control from the effective locale and leave the option set at two.** Steelman, at strength: one line, no catalog key, no shared seam, no test infrastructure, and it removes the visible disagreement the owner actually reported - which is the whole of what he saw. Rejected because it is defective on any system whose locale is neither German nor English (the control would hold a value absent from its own option list), and because the first Save would still lock the user out of system-following permanently, which is the part of the finding that is a spec defect rather than a display mismatch. Recorded as having lost in the ROADMAP already; carried here so the ADR is self-contained.
- **Duplicate the resolution rule in the dialog rather than extracting a seam.** Steelman: four words of code, no new export, no cross-module coupling, and a reader of the dialog sees the rule in place instead of chasing an import - which is exactly the "explicit over magic" preference the house holds elsewhere. Rejected because two copies of this rule diverging is the defect under repair, and the divergence was invisible for two plans.
- **The sentinel `"system"` instead of `""`.** Steelman: a self-describing token that reads unambiguously in the DOM and in a settings dump, cannot be confused with "the user cleared a field", and is immune to a future `<option>` written without an explicit `value` attribute silently aliasing the sentinel. Rejected on the house pattern (the sibling nullable field in the same component already uses `""`) and because `"system"` occupies the same value space as a language subtag while `""` cannot. The hazard its steelman names is answered rather than dismissed: all three options carry explicit values, and the sentinel is a named constant rather than a bare literal.
- **Name the resolved language in the option label ("System language (English)").** Steelman: it closes the gap the shipped hint claims - that the control says which language the interface uses - without making the user infer anything, and the information is genuinely useful on a machine whose locale the user has not thought about. Rejected on decision 3's reasoning; the hint carries the explanation instead, which is where the false claim actually sits.

### D107: New creates a blank profile, and `currentPath` keeps only its path duties (W2, R13-R16)

**Decision.**

1. **The seed is S4**, produced fresh per call by a module-level factory in `EditorView.vue` (a fresh object per call, never a shared constant, matching the immutable-rebuild discipline of every other write in the file): `{ profile_version: 1, input: { pattern: ".*", extensions: ["mkv"] }, tracks: { rules: [{ match: {} }] } }`. Measured consequence: exactly one diagnostic, `empty-match-expression` at **warning** severity on `tracks[0].match`, so Save is enabled and the profile is incomplete-and-announced exactly as spec 8.2's Add idiom prescribes one level down. **The one prefilled value that is not structural, `extensions: ["mkv"]`, is forced by the validator, not chosen for convenience**, and the README's own example profile uses the same list. `pattern: ".*"` over `""`: both are diagnostic-free (measured), and `.*` makes the identifier the whole basename rather than the empty string for every file, so the seed is immediately usable in a dry run.
2. **This is not a guess.** The README's "Muxsmith deliberately does not do: guess" boundary is about inferring intent from a file - "No language-from-filename, no auto-title" - and its stated reason is a guess applied unattended across hundreds of files with no review step. The seed is a constant scaffold the user sees in a form before anything is written, and its rule is the one the owner already blessed for Add.
3. **`currentPath` keeps exactly its path duties; the other duties move**, one replacement per measured duty: (a) `saveDisabled` drops `!currentPath.value` and becomes `!model.value || hasErrors.value || saving.value || opening.value`; (b) the `watch(model)` validation gate moves to `sessionActive`; (c) `doSave`'s re-guard drops `currentPath` and gains the save-dialog branch; (d) the save target stays `currentPath`, the one duty that needs a path; (e) the path line keeps `v-if="currentPath"` and gains a `v-else-if="sessionActive"` branch naming the unsaved state; (f) the recents-section gate moves from `!currentPath` to `!model`, so the recents affordance disappears once the editor holds anything. **(f) is SUPERSEDED IN PART BY D112** (owner ruling 2026-07-31): `!model` is what Task 3 built and shipped, and D112 replaces it with the two-term pre-session condition, which keeps this clause's own consequence (the affordance disappears once the editor holds anything) and adds the failed-load state to it. The clause stands as the record of the shipped state; the live gate is D112's.
4. **`sessionActive`** means: a profile entered the editor through one of its own funnels, `openPath` or `createBlank`, and therefore false in the bare mount-harness case where a spec injects `modelValue` and installs no IPC mock. **It lands in two steps, and the split is deliberate rather than an inconsistency:** Task 3 introduces it as a plain ref set by both funnels, and Task 4 converts it to a `computed` over the `savedSnapshot` ref its history owns, because the two facts are established at exactly the same two moments. Giving Task 3 the snapshot machinery would put undo state in the task whose review has no reason to look at it. **That property is a safeguard bought deliberately in plan 6 and it is preserved, not removed:** dropping the gate entirely would make every bare-mount spec fire `validate_profile_model` with no IPC bridge at all.
5. **Save with no path opens the save dialog; there is no separate Save-as action.** Parity: mkvtoolnix's own `Tab::onSaveConfig` delegates to `onSaveConfigAs` when the config filename is empty (measured). The flow conforms to `RunHistory.saveLog`'s documented capture-state-before-the-dialog-gap pattern: the model and the needs-a-path fact are captured before the await; `saving` is set before the dialog so the button is disabled while it is open; a cancelled dialog returns without writing; on success `currentPath` is set and, **only when the path was newly established**, `rememberRecentProfile` runs so the just-created profile is reachable from both views' recents. `defaultPath: "profile.yaml"`, filter name reused from `batch-profile-filter-name` (the same key the open dialog already uses).
6. **New is offered in the editor's action row, rendered immediately before Open** (parity: `actionMergeNew` precedes `actionMergeOpen`, and the reference tool's empty state offers both). Both buttons stay visible in every state, so the empty state does not duplicate them.
7. **The editor's empty state is repaired, and the Diagnostics heading stops rendering over nothing.** A new paragraph, shown when the editor holds no model, names both entry paths. **The paragraph's gate is SUPERSEDED BY D112** (owner ruling 2026-07-31): "when the editor holds no model" is what Task 3 shipped, and D112 narrows it to the pre-session state, so a failed load no longer renders it. The diagnostics half of this decision is untouched and the sentence below on the failed-load case is the reason: that state still explains itself. The diagnostics `<section>` is gated on `diagnostics.length`, which removes the bare heading in **every** state rather than only pre-session, and keeps the failed-load case visible (a load that returns `profile: null` still carries its parse diagnostic). No "no diagnostics" line is added: `DiagnosticsPanel`'s own documented position is that an empty list means there is nothing to say at this spot, and the Save button's enabled state is the editor's all-clear signal.
8. **Batch's empty state does NOT gain a New button.** Instead `batch-profile-none`'s text names the Editor view as the place to create one (Task 7). Ground: the editor is where profiles are authored, a second create entry point in another view means a nav switch plus a create for the same action, and spec 8.3 asks views to carry small inline explanations where a first-time user would otherwise guess - which a sentence does and a button does not.
9. **Selection after New:** `createBlank` selects the seeded rule (index 0) so the detail panel opens on the one field the warning is about, mirroring D67's behaviour for Add.

**Rejected alternatives.**

- **A schema-minimum seed.** Steelman: it is the honest empty profile, it invents nothing at all, every field the user must fill is visibly unfilled, and it cannot be accused of guessing. Rejected on the measurement: two error-severity diagnostics, so the user's first sight of the feature is a dead Save button.
- **The empty-rule idiom on a schema-minimum base.** Steelman: it is literally the idiom spec 8.2 already blesses, one level up, and it is the recommendation on record in the ROADMAP. Rejected on the measurement: `empty-extensions` survives at error severity, so Save is still dead. This is the alternative the brief invited the measurement to overrule.
- **A passthrough seed (`unmatched: keep`, zero rules).** Steelman: measured zero errors AND zero warnings, a legal profile by `core-83-zero-rule-keep-passthrough`, no invented rule at all, and the user reaches rules through the existing Add button. Rejected because it silently declares a pure-passthrough intent the user did not choose (that mode is a deliberate, documented choice, not a default), its info notice reads as confirmation rather than as an instruction to fill something in, and it leaves the rule grid empty where the owner's approved idiom is an incomplete-and-warned rule.
- **A distinct Save-as action beside Save.** Steelman: mkvtoolnix ships one, it makes "write this profile to a different file" reachable for an already-saved profile, and it keeps Save unsurprising by never opening a dialog. Rejected for this package: the gap it closes is not the finding, it costs a control plus catalog keys against a hard budget, and the reference tool's own Save already falls back to Save-as when there is no filename - which is the behaviour adopted. Recorded as a candidate for later disposition.
- **A "no diagnostics" line in the editor's diagnostics section.** Steelman: BatchView provides its own zero-case statement rather than leaving the panel silent, so the caller-provides-the-zero-case pattern is the house one, and positive confirmation after fixing an error is worth a key. Rejected because gating the section on content removes the defect in every state at zero catalog cost, and the panel's own doc argues the same position.

**D107-i, information duty (addendum 1).** **The mechanism carries a populated unsaved profile as-is.** The state W2 introduces is "the editor holds a model, `currentPath` is null, `sessionActive` is true", and nothing in it depends on the model being the blank seed: a third funnel that assigns a derived profile and establishes the same baseline lands in exactly that state, Save opens the save dialog for it, and the discard guards cover it. **No seam, parameter or extension point is added for that package**, and no task references it. The one consequence worth naming rather than discovering later: the guards' text and behaviour are written for "the editor's content", not for "a blank seed", so they read correctly for a populated profile without change.

### D108: editor undo/redo over the single mutation funnel, with the save state derived from its history (W3, R26-R34, R37, R38)

**This is a REVERSAL of the owner's own S22 ruling of 2026-07-22** (plan-7.5 kickoff), which put undo/redo wholesale in 1.x. The old reasoning, recorded so the reversal is legible: at 1.0 the explicit-save model bounds the loss, and undo/redo - not a confirmation dialog - is the durable answer to accidental destruction. **His new reason:** change tracking is being built anyway for the discard guards, so the feature's cost has already been paid. The second half of the old reasoning is not reversed but consumed: D66's no-confirmation-for-Remove stands, and this is the answer it named.

**Decision.**

1. **The history is driven by the ONE existing mutation funnel, `watch(model)`**, which every profile write already passes through (measured: seven whole-value assignments, no in-place mutation, no external writer). A history entry is the serialized model (`JSON.stringify`). State: `history: string[]`, `position: number`, `savedSnapshot: string | null`. **The push rule is a comparison, not a flag:** on every model change, push only when the serialized model differs from `history[position]`. That makes an undo-driven assignment a no-op for the history by construction (an undo sets the model to `history[position - 1]` and moves `position` first, so the watcher's comparison matches), removes any need for an applying-history latch, and incidentally dedupes a widget that re-emits an identical value. A push while `position` is not the last index truncates the redo tail, which is the standard behaviour.
2. **Granularity: one entry per editing burst**, where a burst ends at a **focus change inside the editor** or at a **discrete grid operation**. Implementation: a `coalesce` flag; the watcher replaces the entry at `position` while it is set and appends otherwise; a `@focusout` listener on the editor's root section clears it (measured: `focusout` bubbles to an ancestor, `blur` does not); and `addRule`, `removeSelectedRule` and `onDrop` clear it before mutating, so two consecutive clicks of the same button are two steps even though focus never moved. **This is the middle option the owner named ("per committed field edit") reached without touching a single widget or changing the validate-on-edit cadence** - text widgets keep their per-keystroke `v-model` binding (measured) and validation still runs on every write, as spec 7 requires. **The failure direction is stated because it is the reason this design is acceptable:** if a discrete funnel ever forgot its reset, the worst case is a coarser undo step (two operations merged), never a lost mutation and never a stale save state, because those come from the funnel and not from the flag.
3. **Saving MARKS the profile it WROTE, and does not clear the history.** A successful save sets `savedSnapshot` to the serialization of the profile `doSave` captured before its awaits - not to the current history entry, because the editing surface stays live across the save dialog and the write, so the two can differ and only one of them is on disk. Undo and redo remain available across a save. This is what makes decision 4 possible at all, and Task 4 Step 1c carries the expression plus the reason the synchronous parity precedent does not license the live read.
4. **The save state is DERIVED, and there is no second mechanism:** `dirty = computed(() => savedSnapshot.value !== null && history.value[position.value] !== savedSnapshot.value)`. It is a value comparison rather than an index comparison, which makes it immune to depth-cap truncation and gives the parity semantics (mkvtoolnix's `currentState() != savedState`): editing and then manually restoring the identical content is not dirty. **Its failure direction is annoyance, never data loss** - a spurious dirty warns where nothing was at risk, a hand-set boolean would silently fail to warn where something was. **That claim holds only because decision 3 marks the written profile rather than the live one**, and it is the reason that expression is fenced rather than described: a mark taken from the live history inverts this direction exactly, since it would report clean over content the file does not hold.
5. **Depth limit: 100 entries.** On a push past the cap the oldest entry is dropped and `position` decremented. Memory consequence, measured rather than asserted, in the form a history entry actually takes (compact JSON of the serde-normalized model, defaults omitted per D48 - see the authoring section, which also records why an earlier figure here was wrong): the chosen seed is 101 bytes and the README's four-rule example 419, so 100 entries of a realistic profile is well under 50 KB, and a pathological 20 KB profile still bounds the history at 2 MB. `savedSnapshot` is a value, so truncation cannot corrupt the dirty computation; the only consequence of dropping the saved state out of the history is that undo cannot walk back that far.
6. **Keyboard: one binding set for all three platforms, accepting both modifiers.** Undo is `(ctrlKey || metaKey) && !shiftKey && key === "z"`; redo is `(ctrlKey || metaKey) && shiftKey && key === "z"` or `(ctrlKey || metaKey) && key === "y"`. **No per-OS branch**, which satisfies both the macOS Cmd convention and the Windows/Linux Ctrl convention with one implementation and no platform-specific code (conventions.md's portability preference). The handler is a `@keydown` on the editor's root section rather than a document listener, so it cannot fire while another view is active and needs no lifecycle teardown; the consequence, stated: the shortcut requires focus inside the editor, and the visible buttons cover the rest. **The handler does not intercept a text-entry control** (`INPUT` of a text-ish type, or `TEXTAREA`), so the browser's native character-level undo keeps working while typing; model-level undo takes over once focus leaves. Help mode is unaffected: its capture-phase `keydown` handles only Escape and Enter/Space on a help target, and the recorded ruling keeps keyboard channels live there.
7. **Undo and redo are ALSO visible controls**: two buttons in the editor's action row after New and Open, disabled when their stack end is reached, labelled by two new keys in the `editor-action-*` family. Ground: spec 8.3 requires the UI to be usable without external documentation, and a keyboard-only feature is undiscoverable. They carry no `title` attribute, matching every other button in this view (the house pattern here is no tooltips; the pre-existing divergence from BatchView is surfaced, not fixed).
8. **Opening or creating a profile RESETS the history** to a single entry holding the loaded or created state, with `savedSnapshot` set to it, so undo can never reach across profiles and a freshly opened profile is not dirty. Applying any history entry clears `selectedIndex`, the same rule `onDrop` and `removeSelectedRule` already follow, because a selection maps to an identity rather than a position.
9. **A load that returns no profile CLEARS the history, and this is decided here rather than left to the funnel.** `openPath` assigns `model.value = doc.profile ?? undefined`, and the `undefined` branch is a real, tested state: a parse failure resolves at the IPC level and carries its diagnostic. In that branch the history becomes **empty** and `savedSnapshot` becomes `null`, which means `sessionActive` is false, `dirty` is false, and `canUndo`/`canRedo` are false - so no guard fires over a failed open and undo cannot resurrect the previous profile into a path that just failed to parse. The diagnostics panel still renders the parse error, because D107 gates that section on `diagnostics.length` rather than on the session. **The alternative the implementer must not invent** - leaving the previous history standing - is what makes the next decision necessary, and both are stated because either alone would leave the reachable defect open.
10. **Undo and redo are gated on `model`, in the functions and not only in the buttons.** The action row sits OUTSIDE the `<template v-if="model">` wrapper, so the two buttons are reachable when the editor holds nothing, and the keyboard handler is on the editor's root section, so it fires there too. Both `undo()` and `redo()` therefore return early unless `model.value` is set, and the buttons' `:disabled` carries the same term. Decision 9 makes the reachable path unreachable; **this guard stays anyway** (`proc-proposed-safeguard-stays`), because "currently unreachable" is precisely the claim a later change falsifies, and the cost is one term in two conditions.

**Rejected alternatives.**

- **Per-keystroke granularity.** Steelman: it is the simplest possible rule, it needs no boundary concept at all, it can never merge two operations the user thinks of as separate, and character-level undo is what a text editor does. Rejected because undo would walk a typed regex one character at a time, a 100-entry cap would be consumed by one field, and the reference-tool comparison offers no support for it (there is no undo there to imitate).
- **Commit-on-change binding (`v-model.lazy`) in the text widgets.** Steelman: it makes the model write itself the commit boundary, so the history needs no coalescing rule whatsoever, and it removes validation flicker while typing; Playwright's `fill()` dispatches `change` as well as `input` (measured), so the existing suite would survive it. Rejected because it changes a shipped, spec-7 behaviour (validate on every profile edit) for every text field in the editor as a side effect of an undo feature, and it touches several widgets where the chosen rule touches none.
- **A time-based coalescing window.** Steelman: it merges bursts regardless of focus behaviour and needs no DOM event at all. Rejected because it makes the granularity untestable without controlling the clock, and because it merges two deliberate operations that happen to fall inside the window.
- **A hand-set dirty boolean, with undo/redo built separately.** Steelman: each mechanism is then simple in isolation, and the boolean is trivially cheap. Rejected on the controller's recorded reasoning and independently on the failure direction: a boolean a mutation path forgets fails silently toward data loss, while a history a mutation path misses also breaks undo for that operation, which a test and a user both see.
- **Unlimited history depth.** Steelman: a profile snapshot is tiny (measured), so a cap solves a problem nobody has, and dropping entries is exactly what a user reaching for undo does not want. Rejected because unbounded growth is unbounded by construction and the memory statement would then have no number in it; 100 entries is far past any editing session's realistic undo reach.

**D108-i, information duty (addendum 4 decision 7).** **The derivation package's population would be ONE undoable step, if and only if it assigns the whole profile in one write** - which is what the push rule makes true: one whole-value assignment produces one history entry, and the coalescing flag would be cleared by the focus change that selecting a container necessarily involves. **Nothing is built for it here** and no task references it.

### D109: the discard guards - the confirmation precedes the destructive action, and one prompt covers a coinciding close (W4, R21-R25)

**Decision.** All three guards are gated on the derived save state (D108 decision 4). None fires on a clean editor.

1. **Opening another profile over unsaved changes WARNS, before the file dialog.** The guard sits in `pickAndOpen`, ahead of the open dialog, so the owner's ordering holds: confirm, then file dialog, then replace. Declining returns without touching anything; **cancelling the file dialog after confirming also leaves the model intact**, because nothing is discarded until a load succeeds. The guard is NOT in `openPath` (the shared funnel), which runs after the dialog. The recents-click path needs no guard because the recents section is gated on the pre-session condition (D107 decision 3f as amended by D112), so it cannot be reached while the editor holds anything - and that unreachability is asserted by a test rather than argued, so a later change to that gate cannot silently open the hole. **D112 only narrows that gate**: its `!model` term alone already carries this unreachability, and the second term subtracts the failed-load state, where the editor holds nothing either.
2. **New over unsaved changes warns the same way.** **This is the plan author's decision, not the owner's ruling**, and it is named as such: his ruling enumerated opening a profile, switching tabs and closing the app, and New did not exist when he named them. Ground: New discards exactly as much as Open, the confirm mechanism exists and gains one call site, and leaving the package's own new button as the one unguarded discard path would be a data-loss hole in the very feature that introduces the state. **Sensitivity, stated so it can be struck cleanly:** if the owner strikes it, the only change is that `createBlank` is called unconditionally and one test moves from asserting the confirm to asserting its absence; nothing else in the package depends on it.
3. **Switching tabs warns not at all, and nothing is built.** The requirement ("must not affect the editor's content at all") is already satisfied by App.vue's `v-show` mounting, and an existing test asserts field value and open path across a switch. **What this package adds is one assertion**: the same round trip with unsaved changes, asserting that the content, the dirty state and the undo history all survive and that no confirmation appears.
4. **Closing the app WARNS when unsaved changes exist**, through the handler that already exists rather than a new mechanism. `AppState` gains `editor_dirty: AtomicBool`; a new command `set_editor_dirty` writes it; the frontend calls it from a watcher on the derived dirty state. The call is tolerant like every other background bookkeeping write in this codebase (a failure is logged, not surfaced); the consequence, named rather than discovered: a failed sync leaves the shell's flag stale, so a warning can be missed. The alternative - surfacing an error dialog from a bookkeeping write - is worse, and the window is one IPC call wide.
5. **R25, the precedence: ONE prompt, always, and a third message for the coinciding case.** `CloseDecision` becomes four variants and `close_decision` reads both facts:

   | run slot occupied | editor dirty | decision | dialog | confirming does |
   |---|---|---|---|---|
   | no | no | `Close` | none | closes normally |
   | yes | no | `ConfirmAbort` | the existing abort dialog | aborts the batch and quits (unchanged) |
   | no | yes | `ConfirmDiscard` | the new discard dialog | quits, discarding the unsaved profile |
   | yes | yes | `ConfirmAbortAndDiscard` | a dialog naming both facts | aborts the batch and quits |

   Two prompts in sequence is rejected below. The combined case gets its own message rather than a composed one, because assembling prose from two localized fragments is precisely what the i18n architecture forbids.
6. **The confirm surface for the two in-app guards is an in-app `<dialog>`**, a small component with title/message/label props and an exposed `ask(): Promise<boolean>`, mounted once by `EditorView`. Ground: it is the house pattern for a frontend modal (D27's `SettingsDialog` uses a native `<dialog>` with `showModal()`), it is assertable as rendered DOM with its real Fluent text and scannable by axe, and it needs **no capability change** - where the dialog plugin's `confirm` would need `dialog:allow-message` and would only be observable at the wire (correction 4). Esc closes it, which reads as cancel: the safe direction. The component's props are the minimum a second caller needs, which is also what makes it callable by a later package without anything being added for one.
7. **The guard text names what is being overwritten**, per the owner's recorded shape, and offers exactly two choices: discard and continue, or cancel. There is no "save first" third button.
8. **The guard family does not reach a single rule mutation.** Its triggers are whole-model replacement and app exit. Spec 8.2's "Remove deletes the selected rule without confirmation" is untouched, and D66's premise is satisfied by D108 rather than reopened.
9. **The close decision is RE-READ once after the user confirms, and a second prompt fires only when the state now carries a fact the answered dialog did not state** (owner ruling 2026-07-30, closing the residual the precedent sweep found). **This extends decision 5; it does not reopen it.** Decision 5's four variants already read both facts and give the coinciding case its own message, and its rejection of "two prompts in sequence" is about assembling ONE situation out of two dialogs. This is a different situation - the state CHANGED between the read and the confirm - so the same one-prompt-per-state principle applies to a state the machine currently reads only once. The table is unchanged and is the vocabulary below. **What the reconciliation does and does not settle, stated because that rejection rested on THREE grounds and this decision incurs two of them:** the situation ground - two questions where one action follows - genuinely does not reach a changed state, since no single message could have been right for a fact that did not exist at read time; the other two, **nesting a second dialog inside the first's async callback and doubling the cancel paths, are costs the owner's ruling accepts** in exchange for closing R23's residual. They are acceptable here for reasons the bullets below carry rather than by assertion: the nesting is exactly one level deep and terminates, and the second cancel path is the mechanism that makes an informed decline possible at all.
   - **Where.** Inside the abort/discard dialog's own callback, on the confirming branch, **before** `abort_and_quit` or `app.exit(0)` is called. The callback already holds the `AppHandle` it needs (`abort_and_quit(&app.state::<AppState>(), ...)` reads state there today), so the re-read is `close_decision(&app.state::<AppState>())` with no new plumbing. Placing it before the action is what makes declining cost nothing: no quit has been armed yet, and `api.prevent_close()` from the first pass means the window never closed.
   - **What triggers the second prompt.** Not any change - a **strengthening**. The dialog the user answered stated a fact set (run-abort, discard, or both); the re-read names its own variant, and the second prompt fires exactly when that variant carries a fact the answered dialog did not state. A weakening never re-prompts: the user saved while the dialog was up, or the run finished, so everything they agreed to still holds or has become less severe. `Close` on the re-read is the extreme weakening and proceeds silently to the quit the user asked for.
   - **Which dialog, and NO fifth message.** The re-read lands in one of the four variants, and the second prompt is that variant's own dialog: `ConfirmAbortAndDiscard`'s combined message when a run was the stated fact and the editor became dirty, `ConfirmDiscard`'s when the run ended and the editor became dirty. `Close` has no dialog and needs none. **No message is added, so no catalog key and no budget change** - the message set is owner-visible and inventing a fifth entry in it would have been a finding to surface rather than a decision to take here.
   - **Confirming and declining at the second prompt.** Confirming performs the action the re-read's variant prescribes in decision 5's table - `abort_and_quit` for the two run-bearing variants, `app.exit(0)` for `ConfirmDiscard` - and is **terminal**. Declining returns to the running app: the window stays open, nothing is aborted, no quit is armed, and the user is back where they were. That asymmetry is deliberate; the second prompt exists because the user is being told something they had not been told, so the safe answer to "no" is to keep the app alive.
   - **It cannot loop, by construction: there is exactly ONE re-read.** The second prompt's answer is final and no third read happens. The bound is structural rather than a counter, because the alternative is a livelock the user can trigger by typing during each dialog. **The residual this leaves, walked over all THREE second-prompt variants rather than argued from one**, because the four `Some` cells produce three distinct dialogs and a "therefore no further case exists" conclusion has to enumerate as many members as its own table does - which is the very defect this entry descends from, one level down:
     - **`ConfirmAbortAndDiscard`** (reached from either single-fact answer): both facts are on screen, so no further strengthening exists at all and the bound needs no argument.
     - **`ConfirmDiscard`** (the answered dialog was the abort one; the run ended and the editor went dirty): a run started during it is a third strengthening. Confirming exits through `app.exit(0)` on a fresh run, which is the pre-existing D31 behaviour for a run started after the decision and not a class this package introduces.
     - **`ConfirmAbort`** (the answered dialog was the discard one; the user saved and a run started): the editor going dirty AGAIN during it is a third strengthening, and confirming aborts and quits with that content unsaved. **This is not the class the original finding named**, and the difference is the whole reason the bound holds: the discard fact was stated by the first dialog of this same sequence and the user confirmed it, so nothing is lost that no dialog mentioned - which was the defect - as against a fact stated once and then acted on twice.

     Stating the `ConfirmAbort` case changes neither the bound nor the design; it makes the paragraph's conclusion true as written.
   - **The rule is a pure function, so it is testable the way `close_decision` already is.** `reconfirm_decision(answered: CloseDecision, current: CloseDecision) -> Option<CloseDecision>` returns `Some(current)` when `current` carries a fact `answered` did not state and `None` otherwise, factored off the Tauri types for exactly the reason the existing decision function was.

**Rejected alternatives.**

- **No guards at all** (the position this plan would have taken before the owner's ruling, recorded because the reasoning is on the record and a later reader will reconstruct it). Steelman, at strength: the same loss already exists and is accepted for an opened-and-edited profile, so a guard only on the new path would be the lone deviation; the app's close handler consults only the run slot today, so the close half needs a new frontend-to-shell channel; and `gui-17` already parks the reference tool's own warn-before-discard preference as a v1.x candidate. **Overruled by the owner**, who gated the whole family on save state instead - which answers the deviation objection, because the guard now covers the opened profile and the created one identically.
- **An unconditional warning, independent of save state** (the ROADMAP's controller reading 2). Steelman: it fires where nothing is at risk, which is the cheaper error, and it needs no change tracking at all - so the whole of D108's history would be optional. **Superseded by the owner's ruling of 2026-07-30**, which gates every warning in this family on unsaved changes.
- **Two sequential prompts when a run and unsaved changes coincide.** Steelman: no new message is needed, each prompt says one true thing, and the existing abort dialog stays byte-identical. Rejected because it doubles the cancel paths, nests a second dialog inside the first's async callback in the shell, and asks the user two questions where one action follows. **This rejection stands for its own case and is not superseded by decision 9**, whose situation is a different one: here both facts are true at the moment of the read, so one combined message is the right answer and the third ground is decisive. Decision 9 addresses a state that CHANGED between the read and the confirm, where no single message could have been right, and it therefore **accepts the first two grounds as costs** - the nesting and the extra cancel path - for the reasons recorded there. The grounds are kept rather than trimmed, because a rejected alternative keeps its steelman and a later reader has to be able to see which cost was paid and where.
- **Shipping the shell's dialogs in English with a recorded reason** (this plan's own first position, recorded because the reasoning is on the record and the reversal is the point). Steelman, at strength: the shell reads one embedded English catalog through a deliberately minimal line-lookup, the de values already exist for parity, and localizing it is a mechanism rather than a string, so it looks like a separate item with its own vehicle. **OVERRULED by the owner, 2026-07-30, and as a general rule rather than a judgment about this dialog: German translations always ship in the same change, without exception.** The steelman's own first clause is what made the position wrong - a German user meeting an English quit dialog is the same class of defect W1 exists to fix, one surface down. D110 carries the decision.
- **Handling the close event in the frontend** via the window plugin's own close-requested listener. Steelman: the frontend already holds the dirty state and the localized catalog, so the dialog would be localized and testable in the DOM. Rejected because it would put two independent handlers on one OS event whose composition this plan cannot verify, and it would inverse an existing, unit-tested shell decision that already owns this event.

### D110: the shell renders in the locale the frontend applied, and the shell's strings come under a parity check (W4b, R39-R41)

**Owner ruling, 2026-07-30, general and without exception: German translations always ship in the same change.** So no string this package adds is English-only, and "accepted with a recorded reason" is not an available disposition for any of them.

**Decision.**

1. **The shell's lookup becomes locale-aware, conforming to D63's shape rather than inventing one.** `run.rs` gains a `LOCALES: &[(&str, &str)]` table - one row per locale, `(primary subtag, gui-common.ftl)`, each `include_str!`ed - exactly the hand-written table shape `crates/muxsmith-cli/src/i18n.rs` uses and for the same recorded reason (`include_str!` is compile-time and has no glob form). `ftl_message` takes the requested locale, collapses it to its primary subtag, and walks the per-message chain `[requested, en]`: the requested row's line lookup, then the en row's, then the raw key. That is D63's fallback chain in the shell's line-lookup idiom rather than a `FluentBundle`, because the reason the line lookup exists - the shell consumes only simple one-line messages and a Fluent stack in the shell would duplicate the frontend's loader - is unchanged by adding a second locale.
2. **The shell does NOT resolve the locale; it is told the one the frontend applied, and that is the authoritative resolution.** `AppState` gains `dialog_locale: Mutex<String>`, defaulted to `"en"`; a new command `set_shell_locale` writes it; `App.vue` pushes `currentLocale` through it with `watch(..., { immediate: true })`, so the value lands at mount and again on every live switch. **Why not let the shell resolve its own** (the obvious alternative, and the one that would have needed `sys-locale` in `src-tauri`): D106's whole finding was ONE nullable field read with TWO fallbacks in two places that nothing reconciled, and an independent shell resolution would be a THIRD implementation of "absent means the system locale" - in another language, against another system-locale source. `effectiveLocale` stays the single resolution rule in the product and the shell consumes its result.
3. **The two resolutions therefore cannot disagree, and that is the point of decision 2 rather than a happy accident.** Had the shell resolved independently, they could have: the frontend's fallback is `navigator.language` (what the webview reports) and Rust's would have been `sys_locale::get_locale()` (what the OS reports), and those differ on a machine whose webview UI language and OS locale are set apart - precisely the no-stored-override case that produced finding 1. A shell dialog in one language over a UI in another is the defect one level down from the one being fixed. **The residual window, named rather than hidden:** a dialog that fires before the frontend has ever pushed renders en. It is not reachable in practice, though not impossible in principle - the window is shown by the shell rather than by the webview's bootstrap, so a close request in the first frames is conceivable - and a failed push leaves the previous value - the same tolerance class as the save-state sync, in the same direction (a stale language, never a missing dialog).
4. **The shell's strings come under a parity check, derived rather than hand-listed, and its assertions are made BELOW the fallback.** The lookup is therefore split so it can be tested at all: a row-level `lookup_in(catalog, key) -> Option<&str>` carrying the line-parse rules, and `ftl_message(key, locale)` composing `[requested, en]` over it. **That seam exists because of the check, and the reason is the finding that produced it:** an assertion made through the composed chain cannot fail, since the en fallback supplies a non-empty non-key value for every mutation upstream of it. Three parts, each naming the red state it defeats:
   - **(a) directory versus table.** Read `locales/` at test time; every locale directory must have a row in the shell's table. Defeats the missing-row mutation, i.e. a locale that ships in `locales/` and `help/` but is silently unserved by the shell.
   - **(b) every key present in every row, asserted on `lookup_in` alone.** Derive the shell-consumed key set from `run.rs`'s own source (`include_str!` of the module plus a regex over `ftl_message("...")` call sites), then for every table row assert `lookup_in(row_catalog, key)` is `Some` and non-empty - **never through `ftl_message`**. Defeats the deleted-de-key mutation, which the chained form resolves to English.
   - **(c) one pinned German value.** `ftl_message("close-abort-title", "de")` must equal `Laufende Jobs abbrechen`, mirroring the existing test's pinned en wording. Defeats the de-row-aimed-at-the-en-catalog mutation, which (b) alone cannot see because the en catalog does contain every key. **This part is also the producing check for D110's user-visible half** - that a German user reads German - which no other check in this package covers. **What (c) is NOT, named so it is not read as more than it is:** it pins a pre-existing key and therefore proves the shell reads the de catalog at all - the mechanism D110 needs - and it does not detect a NEW German value accidentally copied from its English source. No check in this repo does that for any catalog (the frontend gate compares ids, not value distinctness), so the depth is house-consistent rather than a gap this package opens.

   **The general handle, stated because this plan got it wrong once and the class has three instances in this project:** where the mechanism under test contains a fallback - a locale chain, a default branch, a retry, an `unwrap_or` - a red state mutated upstream of the fallback never reaches the assertion. The red state must DEFEAT the fallback, not merely disturb its input: assert the single step without the chain, and pin one concrete value on the far side rather than a non-empty non-key result, because a fallback's output is also non-empty and also not the key. **The trigger is readable in the artifact** (does the path between mutation and assertion cross a fallback clause?), which is why it is a handle rather than a warning.

   **What it leaves uncovered, stated rather than implied:** a non-literal `ftl_message` argument would be invisible to the regex (the same limitation the frontend's literal scan documents for `$t(dynamicKey)`; every current call site is a literal, measured); and the CLI's own `LOCALES` table has the identical unserved-locale gap, which this package SURFACES rather than fixes, because the CLI is not a surface it touches.
5. **`RUST_ONLY_IDS` in `scripts/check-i18n.mjs` gains this package's six shell ids.** It is an enumeration of shell-consumed keys that this change falsifies; leaving it stale would report them as unused frontend ids. One named region, not a licence to edit the script otherwise.

**Rejected alternatives.**

- **Let the shell resolve independently** (stored setting, then `sys_locale::get_locale()`, then en - a literal transcription of D63's own cascade). Steelman, at strength: it is the closest possible conformance to the house pattern the ruling itself points at; it needs no command, no state field and no frontend wiring, so the shell keeps working even if the frontend never speaks; and `sys-locale` is already in the workspace's tree, so the dependency is free in practice. Rejected because it creates a third implementation of the rule whose duplication is the finding under repair in W1, and because its two fallbacks can disagree exactly where the user has no stored override - the case the QA finding came from.
- **Keep `ftl_message` en-only and pass the whole rendered string down from the frontend** (the shell would receive title, message and labels rather than a locale). Steelman: the frontend already has the catalog, the Fluent stack and the locale, so the shell would need no table, no lookup change and no parity check at all - the strings would be localized by construction. Rejected because it inverts the shell's prose-free posture: dialog text would arrive as data from the webview, so the shell could no longer show a dialog when the frontend is wedged, and the catalog would stop being the single source the shell reads. It also trades one small table for a wider IPC surface carrying user-visible prose.
- **A check written in `check-i18n.mjs` instead of Rust.** Steelman: it keeps every i18n check in one place, which is the recorded position that killed a separate `check-help.mjs` (D62). Rejected because the thing under test is the shell's own lookup, and a JS check would have to re-implement that lookup to test it - a second copy of the mechanism, which is the defect class this package exists to remove.

### D112: the two pre-session surfaces carry an explicit condition, and a failed load is not the pre-session state (amendment 1, R43)

**Owner ruling, 2026-07-31, option A of the question Task 3's review parked.** After a profile fails to load the editor renders the "Selected profile" line and the parse error, and renders neither the empty-state paragraph nor the recents section; those two show only before anything has been opened or created at all. The loss he accepted with it, named rather than discovered later: in that one state the recents shortcut is not offered, so re-picking a profile costs a click through Open, which is in the action row in every state. **The option itself is his and is not re-argued here** - what this decision settles is the gate condition the ruling's own cost clause demanded ("these two gates need an explicit definition rather than inheriting one, or the contradiction returns two tasks later").

**The number is measured, not assumed.** `grep -rhoE '\bD[0-9]{1,4}\b' docs/` sorted numerically ends at **D111** (`docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`), and `grep -rnE '\bD1(1[2-9]|[2-9][0-9])\b' docs/` returns nothing, exit 1, with a fired control: the same expression against a synthetic line carrying `D112` matches it. Widened to the whole tracked tree (`git grep`), still nothing. **D112 is therefore the next free id**, and D111 is not this plan's, which is why the series skips no number of its own.

**Decision.**

1. **The condition is one named `computed` in `EditorView.vue`, `nothingOpenedOrCreated`, and both surfaces read it**: `!model.value && currentPath.value === null`. **No RENDER gate in the view - no `v-if`, no `v-else-if` - reads a bare `!model` afterwards**, which is precisely the reach of the standing lint rule this entry adds below and of absence check P1 that runs beside it. The `:disabled` bindings Step 4 prescribes for Undo and Redo (D108 decision 10) and `saveDisabled`'s own `!model.value` (D107 decision 3a) keep their `!model` term and are deliberately outside that reach: they gate an action on whether there is content, a different question from whether anything has been opened or created.
2. **Two terms, because two facts have to be absent at once.** `model` carries "the editor holds something"; `currentPath` carries "a file has been bound to the editor". A load that resolves but fails to parse leaves the second set and clears the first - `openPath` sets `currentPath` and then assigns `doc.profile ?? undefined` - which is exactly what separates that state from the pre-session one. Nothing else can produce the combination: `model` is cleared in that branch alone, and the only site that clears `currentPath` (`createBlank`) assigns a seed in the same synchronous body.
3. **The condition is NOT `!sessionActive`,** and the reason is this task's own change: Task 4 turns `sessionActive` into a `computed` over `savedSnapshot`, and D108 decision 9 nulls `savedSnapshot` on a failed load, so `!sessionActive` is TRUE in precisely the state these surfaces must stay hidden in.
4. **No third state flag is added.** The two existing refs already determine the fact, and a flag would have to be remembered by every future funnel that fills the editor, where the derived condition covers such a funnel by construction.
5. **D107 decision 3's duty split is preserved, not reopened.** `currentPath` is read here for the one question it answers - which file is the editor bound to - and not as a stand-in for "may I edit" or "is there content". The `!model` term still carries the has-content half, so a created-and-unsaved profile hides both surfaces with no path in existence, exactly as D107 decision 3f requires.
6. **No new catalog string, and no other gate moves.** The editing surface (`v-if="model"`), the path line, the unsaved line and the diagnostics section keep their conditions, because the ruling keeps the path line and the parse error rendering in the state it changes. The editor catalog budget arithmetic is untouched by this amendment.
7. **The record is a new decision, not an edit of D107.** `proc-supersede-never-overwrite` (`docs/decision-ledger.yaml`) governs: "the superseded entry keeps its statement and rationale with a supersession pointer, the successor entry carries the live rule plus the loser's steelman". D107 decisions 3f and 7 describe what Task 3 built, measured and committed; rewriting them would delete the shipped state from the record and leave a dated owner ruling with no dated entry. Both keep their text and gain a pointer.
8. **It is built in Task 4** (placement argued at the sequencing section and in that task's own sections), and its producers ship with it.
9. **The single-definition property gets a STANDING guard on an existing gate part.** `eslint.config.js`'s per-`.vue` rules block gains one `vue/no-restricted-syntax` entry whose selector matches a `v-if` or `v-else-if` expression containing a unary `!` over `model`, so `pnpm lint` - already a gate part, so **no gate part is added** - turns red on a re-inlined render gate and names the file and line. Measured before prescribing, not reasoned about: `eslint-plugin-vue` is a direct devDependency at the version `package.json` pins, `vue/no-restricted-syntax` ships in it, and the rule is red on the two gates as Task 3 left them and green on Task 4's end state **with the `:disabled` bindings present**, so it does not over-match them. The runs are in Task 4 Step 4c.

**What this leaves uncovered, stated rather than implied.** Two instruments watch the property and they cover different things. The standing lint rule is the durable one: it reads the parsed template, so spacing, term order and a differently-spelled duplicate cannot slip past it, and it fires by itself on every gate run. What it does not see is a render gate that reaches the same question through another NAME - a second computed recomputing `!model && currentPath === null` under a different identifier would satisfy both instruments while duplicating the definition. No check in this repo detects that class for any derived value, so the depth is house-consistent rather than a gap this amendment opens, and the catch for it is the diff review. Absence check P1 is narrower still (one exact spelling of the attribute) and is kept as the cheap demonstration that the pre-state was red, not as the property's guard.

**Rejected alternatives.**

- **Gate both surfaces on `!sessionActive`.** Steelman, at strength: it is the term this plan already coined for "a profile entered the editor through one of its own funnels", it is one term instead of two, and read against the tree as Task 3 left it it is exactly right - `sessionActive` is a ref today, set by both funnels and never cleared, so it is true after a failed open and both surfaces would hide correctly. Rejected because this very task redefines it: after the conversion to a `computed` over `savedSnapshot`, a failed load makes it false, and the gate inverts in the one state it was written for. That is the contradiction the ruling's cost clause predicted, and it would have landed two tasks after the sentence that caused it.
- **A dedicated ref set by both funnels and never cleared** (`hasOpenedOrCreated`). Steelman: it names the fact in one word instead of deriving it from two, no later change to `model` or `currentPath` can perturb it, and unlike `sessionActive` nothing else would consume it, so no other task could redefine it out from under this gate. Rejected because it is a third state flag carrying information the two existing refs already determine, and because it has to be REMEMBERED: any future funnel that puts content into the editor must set it, and the one that forgets leaves the empty state rendering over a populated editor. The derived condition needs no such memory - a funnel that fills the editor writes `model`, one that binds a file writes `currentPath` - which is the same reasoning D108 decision 4 applies to the save state, one surface down. D107-i's derivation-package funnel is the concrete case: it lands at "model set, path null" and both surfaces hide with nothing added for it.
- **Clear `currentPath` on a failed load and leave both gates on `!model`.** Steelman: one assignment changes instead of two gates, no new term is introduced at all, and "the editor is bound to a file" arguably ought to mean the file actually parsed. Rejected because it deletes the only place the failing file is named, which is the measurement that decided the owner's option: the rendered error is `parse-error = The profile could not be parsed: { $detail }` and carries a detail, not a path. This alternative reverses the ruling while appearing to implement it.
- **Leave the single-definition property to the diff review** (this amendment's own first position, recorded because the reversal is the point and because the reasoning that produced it is the reusable part). Steelman, at strength: a render gate is two lines of template a reviewer reads anyway, the property is structural rather than behavioural, and no check part watches template shape - so a guard looks like it must be a NEW gate part, which this plan's Tech Stack constraint forbids outright. **The steelman's last clause is what made the position wrong, and only running it showed that:** `pnpm lint` is already a gate part, `eslint-plugin-vue` is already a direct devDependency, `vue/no-restricted-syntax` ships in the installed version, and `eslint.config.js` already carries a per-`.vue` rules block, so the guard costs one rule entry and no new gate part. Decision 9 carries it. **The general handle, because the class is cheap to check and was not:** a no-work-needed conclusion whose enabling premise is a claim about what the toolchain can or cannot do is settled by invoking the toolchain, never by reasoning about it - every input to that premise (`package.json`, the config file, the installed plugin's rule list) is on disk.

### Catalog budget, stated as an owner-visible cost

The editor catalog budget is a Tier-2 hard boundary against prose growth (`editor-generic-action-keys`), currently **46** ids, and its two prior revisions were owner rulings or owner-approved design decisions. This package needs **eight** new `gui-editor.ftl` ids, so the budget revises **46 -> 54**, itemized so any single one can be struck at plan approval:

| key | task | why it cannot be reused or omitted |
|---|---|---|
| `editor-action-new` | 3 | the New button's label; a control cannot ship without one and D27 forbids a literal |
| `editor-empty` | 3 | the empty state's explanatory text, the "no explanatory text" half of the recorded finding |
| `editor-unsaved` | 3 | distinguishes "a new profile, not yet saved" from "nothing open"; no existing key says it |
| `editor-discard-title` | 5 | the confirm dialog's title |
| `editor-discard-message` | 5 | the confirm dialog's body, which must name what is overwritten (owner's shape) |
| `editor-discard-confirm` | 5 | the confirm button; the cancel button reuses `settings-cancel`, the same cross-view reuse the editor's Save already does with `settings-save` |
| `editor-action-undo` | 4 | the Undo button |
| `editor-action-redo` | 4 | the Redo button |

**The other two catalogs, itemized in the same table so the strike-one-at-approval option covers the whole package rather than its frontend half** - the shell's strings are user-visible strings under D110 and are not a separate class:

| key | catalog | task | why it cannot be reused or omitted |
|---|---|---|---|
| `settings-locale-option-system` | `gui-settings.ftl` (8 -> 9) | 2 | the third option's label; the whole of W1 is that this option exists |
| `close-discard-title` | `gui-common.ftl` (38 -> 44) | 6 | the discard-only close dialog's title |
| `close-discard-message` | `gui-common.ftl` | 6 | its body, which must name what is lost |
| `close-discard-confirm` | `gui-common.ftl` | 6 | its confirm label; the cancel label reuses the existing `close-abort-dismiss` |
| `close-abort-discard-title` | `gui-common.ftl` | 6 | the coinciding case's title |
| `close-abort-discard-message` | `gui-common.ftl` | 6 | its body, which names both facts; composing it from the two single-fact messages is what the i18n architecture forbids |
| `close-abort-discard-confirm` | `gui-common.ftl` | 6 | its confirm label, which names both actions |

`gui-batch.ftl` is unchanged in count (`batch-profile-none` reworded), and `settings-locale-label`'s `.hint` is reworded in place. **Fifteen new ids across three catalogs, thirty catalog lines across both locales, and every one of the thirty lands in the same task as the code that renders it** (R39). Two values change without a new id (`batch-profile-none`, the locale hint), both in both locales, both in one task.

## Work-item coverage map

The walk the plan reviewer repeats. A row missing here is a defect.

| Work item | Implemented by |
|---|---|
| W1. The locale control | Task 2 (code, catalog, tests); Task 1 (spec 8.2 app-settings sentence, D106) |
| W2. Blank profile creation | Task 3 (code, catalog, tests); Task 1 (spec 8.2 editor item, D107); Task 4 (the pre-session gate condition and its tests, D112 - amendment 1) |
| W3. Undo/redo and the derived save state | Task 4 (code, catalog, tests); Task 1 (spec 8.2 editor item, D108) |
| W4. The discard guards | Task 5 (frontend half), Task 6 (shell half); Task 1 (spec 8.2 editor item, D109) |
| W5. User-facing documentation | Task 7 (help topic both locales, `batch-profile-none` both locales) |

## Sequencing, dependency graph, and the no-worktree ruling

**Strictly serial: Task 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7.**

**The file overlap, stated before the ordering argument leans on it, and derived by reading the seven Files lists rather than from recall.** `src/views/EditorView.vue` is written by Tasks 3, 4, 5 and 6; `e2e/smoke.spec.ts` by Tasks 3, 4, 5 and 6; `locales/{en,de}/gui-editor.ftl` by Tasks 3, 4 and 5; `e2e/editor-undo-redo.spec.ts` by Tasks 4 and 5 (amendment 1); the v1 spec by Task 1 alone. That overlap is the reason the tasks are serial and the reason all spec amendments are consolidated into one task rather than three. **Two of those figures were understated in this plan's own first form and are corrected here rather than left standing**: both `EditorView.vue` and `e2e/smoke.spec.ts` are written by four tasks, not three and two - Task 6 lists both (one watcher on `dirty`, and the wire assertions for its two syncs). The correction changes no ordering argument, since serial execution was already the conclusion; it removes an enumeration a later reader would have trusted.

What orders the seven:

- **1 first, and it is the reason the spec amendments are not spread across the implementation tasks.** Three work items amend the same spec paragraph. Three serial edits to one paragraph would mean two of them work against text this plan cannot fence at authoring time, and each would owe its own self-contradiction sweep. One task, one fenced replacement per region, one sweep - and every later implementer reads a spec that already describes the target it is building (`proc-04-spec-wins`).
- **2 before 3 only by convenience.** The locale work shares no file with any other task; it runs early because it is the smaller of the two original findings and its review is independent of everything else.
- **3 before 4, a hard edge.** Task 4's history baseline is established by `openPath` **and** `createBlank`, and `createBlank` does not exist until Task 3. Task 4 also owns the `savedSnapshot` ref that Task 3's `sessionActive` computes over, so Task 3 introduces `sessionActive` as a ref and Task 4 converts it to a computed over `savedSnapshot` - stated here explicitly so neither task invents the other's half. (The alternative, giving Task 3 the snapshot machinery it does not yet need, would put undo state in the task whose review has no reason to look at it.)
- **4 before 5 and 6, a hard edge.** Both guards are gated on the save state Task 4 derives. Building either first would mean building the boolean the doctrine and the controller both rejected. **Amendment 1 adds a second edge in the same direction:** Task 4 settles the pre-session gate condition (D112), and D109 decision 1's argument for leaving `openPath` unguarded consumes that gate - so the consumer runs after the task that fixes it, never in the same task as the fix.
- **5 before 6, a soft edge with a real reason.** Both implement D109; the frontend half establishes the dirty signal's shape in the view, and the shell half consumes it through a new command. Reversed, Task 6 would wire a command against a signal that does not exist yet.
- **The owner's translation ruling did NOT re-cut the tasks, and here is what it moved instead.** Localizing the shell's dialogs is work on strings Task 6 already introduces, so it landed inside Task 6 rather than as a task of its own: that task gained the locale table, the locale-aware lookup, a second command, the source-derived parity test and the allowlist entry, and its Files list gained `src/App.vue` (one watcher) and `scripts/check-i18n.mjs` (one named region). Nothing moved between tasks and no task was added, removed or re-cut.
- **7 last.** It documents what shipped, in the register the help topics already use, and its sentences name behaviour that only exists after Task 6.
- **Parallelism is unavailable even in principle.** Three tasks write the same view file and three write the same catalogs; one tree means one index and one working state.

**No worktrees, as a ruling.** The doctrine's handle (`proc-08-parallel-worktrees`) is a comparison, not a count: a stream costs a worktree setup, a merge, a full gate run on the merged state and the controller choreography around both, so it earns its place only when the task's own work exceeds that. Here the comparison does not even get to be close: every task after the first writes the same view or the same catalogs as another, so two streams would have to serialise anyway, and each stream would run the full gate as `BUILDING.md` enumerates it a second time on its merge. Serial in one tree wins, the way plans 8.5, 9 and 10 ruled for their own reasons. **The serial ruling binds the CONTROLLER's dispatch concurrency too** (`a-serial-ruling-binds-dispatch-concurrency-too`): no second writer is dispatched while a task is live, and every commit is pathspec-scoped regardless (`concurrent-writers-need-pathspec-scoped-commits`).

Commits: each task commits on `master` with explicit pathspecs (block per task). One push, at the close, after the full gate.

## Acceptance map

Every work item, walked in its HALVES, to the task implementing it and the named producer of each half. "MV" = machine-verifiable by a command or test named here.

**Three observables are covered without a row of their own, named here so the asymmetry does not read as a gap.** W2's three new editor ids and W4a's three discard ids have no catalog-parity row where W1 and W3 do (W1-j, W3-t), because the same hard gate every task runs covers them and their values are additionally asserted through rendered text (W2-e, W4-a); "New renders immediately before Open", the SI-3-derived ordering, has no assertion because DOM order is the one property the parity precedent fixes and no test in this suite asserts sibling order; and the two reworded values (`batch-profile-none`, the locale hint) ride the assertions that already read them through `en(id)`.

| # | Observable half | Producer | MV |
|---|---|---|---|
| W1-a | On a system whose locale is German, with nothing stored, the interface renders German | Task 2, `locale-switch.spec.ts`'s new describe under `test.use({ locale: "de-DE" })`, asserting a **de-only** literal heading - a string absent from the en catalog, which is what makes the assertion defeat `buildBundles`'s `[requested, en]` fallback instead of passing on it | yes |
| W1-b | ...and the control shows the system option | same test, `toHaveValue("")` on `select#settings-locale` plus the option's rendered de label | yes |
| W1-c | Saving without touching the language stores NO override | same file, the recorded `set_settings` argument carries `locale: null` | yes |
| W1-d | Choosing English stores `"en"` | Task 2, the round-trip test's first leg, recorded `set_settings` args | yes |
| W1-e | ...and the interface switches to English live, `<html lang>` following | same test, the de-only string gone plus `documentElement.lang` | yes |
| W1-f | Choosing System again stores `null` | same test, second leg, recorded `set_settings` args | yes |
| W1-g | ...and the interface returns to the system language live | same test, the de-only string back plus `documentElement.lang` | yes |
| W1-h | An explicitly stored locale still displays as itself | the **two** EXISTING `smoke.spec.ts` assertions (`"en"` before the save, `"de"` after the reload), both unchanged and both named in Task 2's report with their shared measurement | yes |
| W1-i | The resolution rule exists in exactly one place | Task 2 absence check L1 with its fire and its control | yes |
| W1-j | The new option's strings exist in both locales and parse | `pnpm check:i18n` (hard cross-locale parity) plus `pnpm lint` (D27 no-raw-text), both inside the gate | yes |
| W2-a | New puts an editable profile in the editor | Task 3, `smoke.spec.ts`'s new describe: the rule grid renders one row and the pattern field is present | yes |
| W2-b | ...and it is validated although no file path exists | same test, the recorded `validate_profile_model` invocation | yes |
| W2-c | The seed's diagnostic is a warning, rendered in the panel | Task 3, the seed test asserting the `empty-match-expression` line through the en catalog | yes |
| W2-d | ...and Save is therefore enabled | same test, `editor-save` enabled | yes |
| W2-e | Before anything is opened or created, the editor explains both entry paths | Task 3, the empty-state test asserting the paragraph | yes |
| W2-f | ...and renders no Diagnostics heading over nothing | same test, absence check E1 with its in-test fire (the heading appears after New) | yes |
| W2-g | Save with no path asks where to write | Task 3, the recorded `plugin:dialog|save` invocation | yes |
| W2-h | ...and writes the seed to the picked path | same test, the recorded `save_profile` args (path and profile both asserted) | yes |
| W2-i | ...after which the editor shows that path | same test, the `batch-profile-current` line | yes |
| W2-j | ...and the profile is reachable from recents | same test, the recorded `set_settings` carrying the path first in `recent_profiles` | yes |
| W2-k | A cancelled save dialog writes nothing and leaves the profile unsaved | Task 3, absence check E2 with its fire (`plugin:dialog|save` recorded, `save_profile` not) | yes |
| W2-l | The seed choice is reproducible against the validator | Task 3 Step 1, the validator re-run pasted in the report | yes |
| W2-m | After a load that fails to parse, the editor still names the failing file and still renders the parse error | Task 4 Step 6's D112 case, leg 3's two POSITIVE assertions - the `batch-profile-current` line carrying the failing path, and the rendered parse-error line. They are what make the state under test identifiable as the failed load rather than as any state in which the editor happens to show nothing | yes |
| W2-n | ...and renders neither the empty-state paragraph nor the recents section | same case, absence check P2's zero in leg 3 (`editor-empty` and `editor-recents` both count 0), whose fire is leg 1 | yes |
| W2-o | Before anything is opened or created, both of those surfaces DO render | same case, leg 1 (`editor-empty` visible, `editor-recents` count 1). **This is the half an implementation that hides them in every state would pass without**, which is why it is a row of its own and not only P2's fire | yes |
| W2-p | No render gate in the editor asks `!model` directly; the pre-session state is asked through the one named condition | **Task 4 Step 4c's `vue/no-restricted-syntax` rule**, standing inside `pnpm lint`: red on the two gates as Task 3 left them, green on the end state with the `:disabled` bindings present. Plus Step 7's absence check P1 as the one-shot demonstration of the same pre-state. **The row is scoped to what those two measure**: a second computed that recomputes the same expression under a different NAME satisfies both, and D112 names that as uncovered rather than letting this row imply otherwise | yes |
| W3-a | A top-level field edit is one undo step | Task 4, mutation-path case 1 (`setFieldValue`) | yes |
| W3-b | A `tracks.unmatched` change is one undo step | Task 4, mutation-path case 2 (`setTracksUnmatched`) | yes |
| W3-c | A per-rule detail edit is one undo step | Task 4, mutation-path case 3 (`setRuleValue`) | yes |
| W3-d | A drag-reorder is one undo step | Task 4, mutation-path case 4 (`onDrop`) | yes |
| W3-e | Add is one undo step | Task 4, mutation-path case 5 (`addRule`) | yes |
| W3-f | Remove is one undo step | Task 4, mutation-path case 6 (`removeSelectedRule`) | yes |
| W3-g | Undo restores the previous state in the view | Task 4, the undo/redo test asserting the rendered control values | yes |
| W3-h | ...and in the model the shell would receive | same test, `readModel` on the mount-harness case | yes |
| W3-i | Redo re-applies an undone step | same test | yes |
| W3-j | A new edit after an undo drops the redo tail | same test, Redo disabled | yes |
| W3-k | Typing inside one field coalesces into one step | Task 4, the granularity test, first half | yes |
| W3-l | ...and a focus change starts a new step | same test, second half (the two halves are the boundary rule's positive and negative) | yes |
| W3-m | Two consecutive presses of the same grid button are two steps | same test, third half | yes |
| W3-n | The buttons report the ends of the history | Task 4, disabled-state assertions at both ends | yes |
| W3-o | Both modifier keys and both redo spellings work | Task 4, the keyboard test over the enumerated combinations | yes |
| W3-p | The shortcut leaves a text-entry control's native undo alone | Task 4, absence check U1 with its in-test fire (the same keys undo outside a field) | yes |
| W3-q | Saving marks a position rather than clearing the history | Task 4, undo still available after a save. **This half passes whether or not the saved position moves**, which is why it is not the whole observable | yes |
| W3-q2 | ...and the editor is no longer in the unsaved state after that save | **Task 5**, the guard test's after-a-save leg: Open immediately after a successful save shows NO confirm, where the same click before the save does. This is `dirty`'s first observable proxy: Task 4 introduces the value but nothing user-facing reads it until the guards exist, and a test-only surface to observe it earlier would be a mechanism the product does not have | yes |
| W3-r | Opening another profile resets the history | Task 4, Undo disabled after an open | yes |
| W3-s | The depth cap holds | Task 4, the cap test driving past 100 entries | yes |
| W3-t | The two new labels exist in both locales | `pnpm check:i18n` inside the gate | yes |
| W3-u | A load that returns no profile leaves no history behind, and undo cannot reach the previous profile from that state | Task 4 Step 6's failed-open case, whose control is the Undo-enabled state earlier in the same test | yes |
| W4-a | With unsaved changes, Open warns BEFORE the file dialog | Task 5, absence check G1: `plugin:dialog|open` not recorded while the confirm is open; its fire is the same counter after confirming | yes |
| W4-b | Confirming discards and proceeds to the file dialog | Task 5, the guard test's confirm leg | yes |
| W4-c | Cancelling keeps the profile and never opens the file dialog | Task 5, the cancel leg with the same absence check | yes |
| W4-d | With no unsaved changes, Open goes straight to the file dialog | Task 5, absence check G2 (no confirm rendered) with its fire in the dirty leg | yes |
| W4-e | New over unsaved changes warns the same way | Task 5, the New leg | yes |
| W4-f | The recents affordance is unreachable while the editor holds a profile | Task 5, the invariant assertion (count 0) with its fire (count 1 in the empty state) | yes |
| W4-g | A tab switch preserves content, dirty state and history, and warns not at all | Task 5, the extended round-trip test | yes |
| W4-h | The shell learns the save state as it changes | Task 6, the recorded `set_editor_dirty` calls, true then false across a save | yes |
| W4-i | An idle, clean window closes without a prompt | Task 6, Rust unit case 1 of the four-state matrix | yes |
| W4-j | A run with no unsaved changes asks about the run only | Task 6, unit case 2 | yes |
| W4-k | Unsaved changes with no run ask about the changes only | Task 6, unit case 3 | yes |
| W4-l | Both together produce ONE prompt naming both | Task 6, unit case 4 | yes |
| W4-m | Confirming a discard-only close quits; cancelling does not | Task 6, the dialog-callback unit coverage | yes |
| W4-n | The six new shell strings resolve from the catalog and stay single-line | Task 6, the extended `close_abort_strings_resolve...` enumeration | yes |
| W4-u | A close the user already confirmed prompts AGAIN when the state now carries a fact the answered dialog did not state | Task 6 Step 5's `reconfirm_decision` matrix, the strengthening cells asserting `Some(v)` with the exact variant | yes |
| W4-v | ...and does NOT prompt again when the state weakened or did not move | same matrix, the weakening and unchanged cells asserting `None`. **This is the side a broken rule passes**, which is why the cells are named in the report rather than counted | yes |
| W4-w | The re-read is actually wired into the confirming branch, ahead of the abort | **reviewer check, not machine-verifiable**: `on_close_requested`'s dialog callback needs the Tauri runtime, which is the recorded reason `close_decision` was factored out in the first place. The matrix proves the rule, never that it is called, and saying so is the honest form - naming the matrix here would be one producer covering the side that cannot fail | no, by nature |
| W4-o | The shell is told the locale the frontend applied, at startup | Task 6 Step 6, the recorded `set_shell_locale` call after `page.goto`, asserted against the concrete expected tag rather than against "whatever was applied" | yes |
| W4-p | ...and again when the user switches the language | same test, a second recorded call asserted equal to `"de"` (the live half; the startup half alone would pass on a shell that never hears a change) | yes |
| W4-q | Every shell-consumed key is present in every shipped locale's own catalog | Task 6 Step 5 part (b), asserted on `lookup_in` for each row directly, never through the `[requested, en]` chain; red state: one de key deleted | yes |
| W4-r | A locale that ships in `locales/` but has no shell row is a failure, not a silent English fallback | Task 6 Step 5 part (a); red state: the `de` row deleted from the table | yes |
| W4-t | **A German user reads a German dialog** - the user-visible half of D110, and the one an en-fallback assertion cannot see | Task 6 Step 5 part (c), pinning `ftl_message("close-abort-title", "de")` to its German value, the mirror of the existing pinned en wording; red state: the `de` row pointed at the en catalog | yes |
| W4-s | The six new shell ids are not reported as unused frontend ids | Task 6 Step 6b, the `pnpm check:i18n` runs before and after the allowlist edit, both pasted | yes |
| W5-a | The help topic names creation, the save-as flow and the discard behaviour, in both locales | Task 7, `pnpm check:i18n`'s D62 gate plus the reviewer reading both topics against the shipped surface | yes (existence and hygiene) |
| W5-b | Batch's empty state names the editor as the create path, in both locales | Task 7, the rendered string asserted through `en(id)` in the existing batch scenario | yes |

---

## Task 1: the normative documents - two spec amendments and D106-D110

Read first: this plan's Decision register in full; `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` sections 8.2, 8.3, 8.4 and 11; one existing decisions file (`docs/superpowers/specs/2026-07-14-plan-5.7-decisions.md`) for the ADR house form; Tier-2 `proc-04-spec-wins`, `a-document-never-cites-a-line-number-inside-itself`. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (two regions only: the tail of section 8.2's numbered item 1, and section 8.2's app-settings paragraph)
- Create: `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`

Section 8.4 is deliberately NOT edited: its locale sentence is already true, and the control's surface belongs to 8.2, which is where it was missing.

**Interfaces:**
- Consumes: nothing.
- Produces: the amended spec every later task reads as ground truth, and the five decision records (D106-D110).

- [ ] **Step 1: section 8.2's editor item.** Replace exactly the sentence

```
detail editor per rule, panels for attachments/chapters/tags/title, open/save YAML, recent profiles.
```

  with exactly

```
detail editor per rule, panels for attachments/chapters/tags/title, create/open/save YAML, recent profiles.
```

  and append, after the item's existing final sentence "Inline validation markers from core diagnostics.", exactly:

```
New creates a blank profile in the editor and touches no file: the seed carries the format version, one candidate extension and one empty track rule, so it is incomplete-until-filled and announced by a validation warning exactly as Add's empty rule is, never by an error that would disable Save. A profile created this way has no path yet; Save opens a save dialog and the picked path becomes the profile's path from then on. Undo and redo cover every model mutation - field edits, rule add and remove including the unconfirmed delete, drag-reorder, and every list or map widget mutation - at one step per editing burst, where a burst ends at a focus change or a grid operation; saving marks a position in that history rather than clearing it, and the history is what the editor derives "has unsaved changes" from. The editor holds at most one profile. Replacing it - by creating another, or by opening one - warns first and only while unsaved changes exist, naming what would be overwritten; switching views never touches it; and closing the app with unsaved changes warns as well, in one prompt that also covers a running batch when both hold.
```

- [ ] **Step 2: section 8.2's app-settings paragraph.** Replace exactly

```
App settings (not profile data): mkvmerge path override, default parallelism. Stored in the platform config directory.
```

  with exactly

```
App settings (not profile data): mkvmerge path override, default parallelism, interface language. Stored in the platform config directory. The language control is three-state - follow the system language, English, German - where following the system IS the absence of a stored override, is preselected until the user chooses otherwise, and stays reachable afterwards, so saving without touching the control stores no override (8.4).
```

- [ ] **Step 3: the decisions file.** Create `docs/superpowers/specs/2026-07-30-plan-12-decisions.md` with an H1 `# Plan 12 decisions` and one section per decision, in the house form measured from the plan-5.7 file: `## D106: <title>` then the bold slots **Decision**, **Rationale**, **Rejected alternatives** (each alternative with its steelman stated at its strongest, not as a strawman), and, where one exists, **Triggers created**. Content comes from this plan's Decision register: D106 the locale control, D107 blank profile creation, D108 undo/redo and the derived save state, D109 the discard guards, D110 the shell's localized dialogs and the parity check over them. Four fixed properties of the file:
  - **D108 is recorded as a REVERSAL**, naming the owner ruling it reverses (S22, 2026-07-22, undo/redo wholesale in 1.x), the old reasoning (at 1.0 the explicit-save model bounds the loss, and undo/redo rather than a confirmation dialog is the durable answer to accidental destruction), and the new reason (change tracking is being built anyway). It also records that D66's no-confirmation-for-Remove premise is CONSUMED, not reopened.
  - **D109 records the superseded controller reading** (an unconditional warning independent of save state) as superseded by the owner's save-state gate, not as an open option, and its rejected alternative **shipping the shell's dialogs in English with a recorded reason** - named rather than numbered, since an ordinal into that list stales the moment one is inserted - as OVERRULED by the owner rather than as a live tradeoff.
  - **D110 records the ruling in the general form the owner gave it** (German translations always ship in the same change, without exception), not as a decision about one dialog, and states the residual it does not close (a non-literal `ftl_message` argument; the CLI's identical unserved-locale gap, surfaced not fixed).
  - **Every rejected alternative in the register appears**, including the ones whose steelman is strong enough to be mistaken for the winning argument. A caricatured rejection is a defect here.
  - No line-number citations, in either direction: not into the spec, not into this plan, not inside the file itself. Name sections, symbols and decisions.

- [ ] **Step 4: the self-contradiction sweep, as an enumeration with a fired control** (`proc-04-spec-wins`'s corollary). Run, and paste, all three:
  - `grep -nEi 'locale|language' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
  - `grep -nEi 'undo|redo|unsaved|discard|confirm' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
  - `grep -nEi 'create|new profile|open/save' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

  Read every hit and classify it as consistent or contradicting; a contradiction is a finding to report, never a silent second edit. **Fired control for the sweep, because an empty or thin result must be distinguishable from a broken pattern:** the `locale|language` expression must return section 8.4's own locale bullets and section 8.2's amended paragraph, and the `create|new profile|open/save` one must return the amended editor item - **named by what they search for rather than by position**, since an ordinal into a bullet list stales the moment one is inserted. If any expression returns nothing at all, it is malformed and the step is not done.
  - Two hits are known in advance and are consistent, named so they are not reported as findings: section 8.3's help-mode Escape sentence (a different keyboard channel), and section 11's non-goals, which name neither creation nor undo.

- [ ] **Step 5: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green (this task changes only documents, so every part is behaviour-preserving by construction; a failure is a real finding -> NEEDS_CONTEXT). `git diff --stat` covers exactly the two files in the Files list.

- [ ] **Step 6: commit.**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md docs/superpowers/specs/2026-07-30-plan-12-decisions.md
git -c commit.gpgsign=false commit -m "spec+adr: the editor creates, undoes and guards its unsaved profile; app settings carry a three-state language" -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md docs/superpowers/specs/2026-07-30-plan-12-decisions.md
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the two fenced spec replacements; that section 8.4 is not edited; the five decision numbers (D106-D110, the next free numbers, measured); which alternatives are recorded as rejected and that each carries a steelman; that the sweep is an enumeration with a fired control rather than a reading; that no file outside the Files list is touched.

---

## Task 2: the settings language control becomes three-state (W1)

Read first: this plan's D106 in full; `docs/ROADMAP.md`'s "OWNER QA PASS, round 3" finding 1 through its ruling and its known-cost block; `src/main.ts`; `src/i18n/index.ts`; `src/components/SettingsDialog.vue`; `locales/en/gui-settings.ftl` and `locales/de/gui-settings.ftl`; `e2e/locale-switch.spec.ts` in full (its `de()`/`buildDeBundle` helpers are reused); `e2e/mocks.ts`'s `installMockIPC` for the `get_settings` fallback and the second-registration mechanism; the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src/i18n/index.ts` (add the exported `effectiveLocale` seam; nothing else)
- Modify: `src/main.ts` (`resolveLocale` routes through the seam; its doc comment names it)
- Modify: `src/components/SettingsDialog.vue` (the sentinel constant, the form init, the save mapping, the live switch and its comment, the third `<option>`)
- Modify: `locales/en/gui-settings.ftl` (the new option id and the reworded `.hint`)
- Modify: `locales/de/gui-settings.ftl` (the same two)
- Modify: `e2e/locale-switch.spec.ts` (a new describe with three cases; the file's header doc gains the new subject)

`playwright.config.ts` and `e2e/mocks.ts` are NOT edited (D106 decisions 5 and 6). `e2e/smoke.spec.ts` is NOT edited by this task.

**Interfaces:**
- Consumes: nothing.
- Produces: `effectiveLocale`, whose second consumer is the dialog.

- [ ] **Step 1: the seam.** Add to `src/i18n/index.ts`, exported, with a doc comment that states the rule and names both callers by symbol:

```ts
export function effectiveLocale(saved: string | null): string {
  return saved ?? navigator.language;
}
```

- [ ] **Step 2: `src/main.ts`.** `resolveLocale` becomes `effectiveLocale((await getSettings()).locale)` in the try and `effectiveLocale(null)` in the catch, with the import added. The existing doc comment keeps both facts it carries (why the locale resolves before mount, and why a `get_settings` failure is not a startup blocker) and gains the seam's name in place of the inline `navigator.language` description.

- [ ] **Step 3: `src/components/SettingsDialog.vue`.** Four edits:
  - A module-level constant with a comment stating why the empty string is the sentinel and naming the sibling field that already uses it: `const SYSTEM_LOCALE = "";`
  - `form`'s initial `locale` becomes `SYSTEM_LOCALE`, and `open()`'s init becomes `form.locale = baseline.locale ?? SYSTEM_LOCALE;`
  - `save()`'s `next` carries `locale: form.locale === SYSTEM_LOCALE ? null : form.locale,` and the live switch becomes exactly

```ts
    if (next.locale !== baseline.locale) {
      applyLocale(effectiveLocale(next.locale));
    }
```

    The existing comment's `!== null` narrowing explanation is replaced by one naming the seam; the D56 live-switch rationale and the v-show-keeps-state sentence stay.
  - The `<select>` gains, as its FIRST option, `<option :value="SYSTEM_LOCALE">{{ $t("settings-locale-option-system") }}</option>`; the `en` and `de` options keep their explicit values and their order.

- [ ] **Step 4: the catalogs, both locales, fenced.** In `locales/en/gui-settings.ftl` replace exactly

```
settings-locale-label = Language
    .hint = Which language the Muxsmith interface uses.
```

  with exactly

```
settings-locale-label = Language
    .hint = Which language the Muxsmith interface uses. System language follows your operating system and falls back to English where a translation is missing.
settings-locale-option-system = System language
```

  and in `locales/de/gui-settings.ftl` replace exactly

```
settings-locale-label = Sprache
    .hint = In welcher Sprache Muxsmith seine Oberfläche anzeigt.
```

  with exactly

```
settings-locale-label = Sprache
    .hint = In welcher Sprache Muxsmith seine Oberfläche anzeigt. Systemsprache folgt der Sprache deines Betriebssystems und fällt auf Englisch zurück, wo eine Übersetzung fehlt.
settings-locale-option-system = Systemsprache
```

  Both files keep `settings-locale-option-en`/`-de` unchanged and in place. Neither value carries a placeable, so the pattern-structure parity check (D55 rule 5) is satisfied by construction, and the attribute-name sets stay equal.

- [ ] **Step 5: the tests, in `e2e/locale-switch.spec.ts`.** A new `test.describe("system-locale default (D106)")` carrying `test.use({ locale: "de-DE" })` as its first statement, three cases. Every interaction string comes from `en()`. **Every asserted German string must be one whose German value DIFFERS from its English value** - either a literal that exists only in de, or `de(id)` for an id whose two values differ - because `buildBundles` negotiates `[requested, en]` per message, so an assertion on an id whose values are identical passes even when the interface fell back to English entirely. That is the frontend instance of the fallback handle in the Global Constraints, and the set it excludes is measured rather than guessed: **15 gui-* ids carry identical en/de values, and both language option labels (`settings-locale-option-en` = `English`, `settings-locale-option-de` = `Deutsch`) are among them**, in the very dialog these cases drive. **The method decides the figure, so it is stated in full: full multi-line value comparison, attributes excluded, over messages that carry a value of their own.** Two coarser readings of the same tree give two other numbers and both reconcile under that one rule - counting value-less messages as well gives 16 (the single extra member is `batch-recents-select`, which carries only a `.tooltip`), and comparing first lines only gives 18 (the two further members are the selector messages whose `{ $n ->` opening matches while their German branches differ). The three cases below are already clear of that set - they assert the batch heading (`Batch` / `Stapel`) and the new `settings-locale-option-system` (`System language` / `Systemsprache`) - so this scopes the permission to its safe set rather than changing any prescribed assertion.
  - **Case 1, first run.** Mock `get_settings` with `locale: null` (and `detect_mkvmerge` as the file already does). `page.goto("/")`. Assert (a) the batch heading renders its de value, and `documentElement.lang` is `"de"`; (b) open settings and `select#settings-locale` has value `""`, and the selected option's text equals the de `settings-locale-option-system`.
  - **Case 2, saving without touching the language.** Same scenario. Open settings, change `settings-default-jobs` to a different number, save. Assert exactly one `set_settings` call and that its `settings.locale` is `null`. **This is the defect's core - the first Save creating an override the user never requested - and it is the persisted half of W1-c.**
  - **Case 3, the round trip.** Same scenario. Open settings, select `"en"`, save: assert the recorded `set_settings.locale === "en"`, that the de heading is gone and the en heading present, and `documentElement.lang === "en"`. Then open settings again, select the system option, save: assert the recorded `set_settings.locale === null`, the de heading back, and `documentElement.lang === "de"`. No reload anywhere in this case: the live path is what it measures.
  - The file's header doc gains a paragraph naming the new subject and stating why the describe-level locale override is safe (it does not disturb the suite-wide English pinning of plan-5 D29, and the config is untouched).

- [ ] **Step 6: verification.**
  - **Absence check L1, the single resolution rule.** `grep -rn "navigator.language" src/ | grep -v "src/i18n/index.ts"`. **RED, run FIRST on the pre-state: exactly 2 lines**, both in `src/main.ts` - `resolveLocale`'s try branch and its catch branch, which are the two places the rule is written today. **GREEN on the end state: 0.** **Soundness control, because an empty grep and a broken grep look identical:** the same expression WITHOUT the filter must return exactly **2** lines from `src/i18n/index.ts` on the end state - the pre-existing occurrence in `primarySubtag`'s doc comment plus Step 1's `return saved ?? navigator.language;` - proving the pattern still matches where the token survives. **Both figures are measured on the plan's baseline commit, and the comment occurrence is the reason the control is 2 rather than 1**; an implementer whose recount disagrees returns NEEDS_CONTEXT with both runs pasted rather than adjusting the fence. **Reachable green state, argued member by member:** both pre-state occurrences sit inside `resolveLocale`, which Step 2 rewrites whole, and the replacement contains no such token.
  - The full gate as `BUILDING.md` enumerates it, foreground, green. `pnpm check:i18n` is the hard cross-locale parity gate for the new id, `pnpm lint` the D27 no-raw-text check for the new option.
  - `git diff --stat` covers exactly the six files in the Files list; anything else is a defect signal -> NEEDS_CONTEXT.
  - Report **both** existing `smoke.spec.ts` locale-control assertions by name - `toHaveValue("en")` on `localeSelect` and `toHaveValue("de")` on `reloadedLocaleSelect`, both inside the German-locale describe's settings-save case - together with the measurement showing why each stays valid: the first runs under the mock default's concrete `"en"`, the second under the case's own `DE_SETTINGS`, and shape A changes the display of neither.

- [ ] **Step 7: commit.**

```bash
git add src/i18n/index.ts src/main.ts src/components/SettingsDialog.vue locales/en/gui-settings.ftl locales/de/gui-settings.ftl e2e/locale-switch.spec.ts
git -c commit.gpgsign=false commit -m "settings: a third system-language option, so the effective locale and the shown value agree and the override stays removable" -- src/i18n/index.ts src/main.ts src/components/SettingsDialog.vue locales/en/gui-settings.ftl locales/de/gui-settings.ftl e2e/locale-switch.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the sentinel and its two mappings; the seam's name, signature and home; that the option label does not name the resolved language; the four fenced catalog strings; that `playwright.config.ts` and `e2e/mocks.ts` are untouched; that both existing locale-control assertions stay (the `"en"` one and the `"de"` one); that an out-of-band stored locale gets no handling.

---

## Task 3: New creates a blank profile, and `currentPath` keeps only its path duties (W2)

Read first: this plan's D107 in full and the authoring section's seed measurement; `docs/ROADMAP.md`'s round-3 finding 2 through its rulings and its measured-cost block; `src/views/EditorView.vue` in full; `src/components/RunHistory.vue`'s `saveLog` (the capture-before-the-dialog-gap pattern this task conforms to); `src/components/DiagnosticsPanel.vue`'s doc comment (why the panel carries no empty state); `src/views/BatchView.vue`'s profile/recents block (the empty-state house pattern); `src/recentProfiles.ts`; `src/ipc.ts`'s `saveProfile`/`loadProfile`/`validateProfileModel`; `e2e/smoke.spec.ts`'s two editor describes (the fixtures, the a11y helper and the recents scenario this task's tests sit beside); the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src/views/EditorView.vue` (the seed factory, `sessionActive`, `createBlank`, the four `currentPath` duty replacements, `doSave`'s dialog branch, the template's New button, unsaved line, empty state, recents gate and diagnostics gate, **and the two doc-comment regions this task's own change falsifies**: the Task-13 block's `currentPath` gate explanation and the stale `gui-editor.ftl stays 45` count)
- Modify: `locales/en/gui-editor.ftl` (three new ids under a new section comment)
- Modify: `locales/de/gui-editor.ftl` (the same three)
- Modify: `e2e/smoke.spec.ts` (a new describe with the tests below, **and the stale `catalog budget is 45` comment**)

**Why the two comment regions are in scope rather than fenced off.** Both are references this task's own edit falsifies inside files it already modifies: the validation gate moves off `currentPath`, and the editor catalog grows. Widening the code while leaving its own description wrong is the defect class `comments-locate-by-symbol-never-by-line-number` and `proc-normative-count-recomputed` exist to prevent. **Both are NAMED regions**, so the Files enumeration stays exhaustive and the within-file qualifier still bites everywhere else (`latitude-carveout-zero-content-structural-forks`).

**Interfaces:**
- Consumes: nothing.
- Produces: `createBlank` and `sessionActive`, which Task 4 converts to a computed over its own snapshot ref.

- [ ] **Step 1: re-measure the seed before writing it.** Write the four candidate seeds and run the validator on each, exactly as the authoring section did, and paste the output. **If the measurement disagrees with the authoring result - if the chosen seed produces any error-severity diagnostic - stop and return NEEDS_CONTEXT with both runs pasted.** The seed is chosen by measurement, and a tree that has moved is a finding, not a licence to adjust the fence.

- [ ] **Step 2: `EditorView.vue`'s script.** Six edits, each fixed here.
  - The seed factory, at module level, with a doc comment stating why it is a function (a fresh object per call, matching the immutable-rebuild discipline of every write in this view) and why `extensions` carries a value at all (the validator: an empty list is an error, so a bare seed would greet the user with a disabled Save):

```ts
function blankProfile(): Profile {
  return {
    profile_version: 1,
    input: { pattern: ".*", extensions: ["mkv"] },
    tracks: { rules: [{ match: {} }] },
  };
}
```

  - `const sessionActive = ref(false);` with a doc comment stating its duty (a profile entered the editor through one of its own funnels, `openPath` or `createBlank`) and why it exists rather than reusing `currentPath` (a created profile has no path, and the bare mount-harness case must keep firing no IPC).
  - `saveDisabled` drops `!currentPath.value`.
  - The `watch(model)` gate becomes `if (!sessionActive.value || !value) { return; }`, and the comment above it names `sessionActive` instead of `currentPath`, keeping its other two facts (why a shallow watch suffices, and what `validationGeneration` is for).
  - `openPath` sets `sessionActive.value = true;` beside `currentPath.value = path;`.
  - `createBlank`:

```ts
function createBlank(): void {
  if (opening.value || saving.value) {
    return;
  }
  ipcErrorCode.value = null;
  currentPath.value = null;
  diagnostics.value = [];
  sessionActive.value = true;
  model.value = blankProfile();
  selectedIndex.value = 0;
}
```

    The order is load-bearing and is commented as such: `sessionActive` is set before the model, so the watcher that fires on the assignment validates the seed instead of returning early; `diagnostics` is cleared first so a previous profile's findings never render against the new model; and index 0 is selected so the detail panel opens on the one field the warning names, mirroring Add's own behaviour (D67).

- [ ] **Step 3: `doSave`, with the dialog branch.** Replace the function body with exactly this shape, whose capture-before-the-await discipline mirrors `RunHistory.saveLog` and is commented as doing so:

```ts
async function doSave(): Promise<void> {
  if (saveDisabled.value || !model.value) {
    return;
  }
  // Captured before the dialog gap: the native save dialog can stay open
  // indefinitely, and the model may change underneath it -- what gets
  // written must be the profile that was in the editor when Save was
  // clicked (same rule as the job-log export in `RunHistory`).
  const profile = model.value;
  let path = currentPath.value;
  const needsPath = path === null;
  saving.value = true;
  ipcErrorCode.value = null;
  try {
    if (needsPath) {
      const picked = await saveDialog({
        defaultPath: "profile.yaml",
        filters: [{ name: fluent.$t("batch-profile-filter-name"), extensions: ["yaml", "yml"] }],
      });
      if (typeof picked !== "string") {
        return;
      }
      path = picked;
    }
    await saveProfile(path, profile);
    currentPath.value = path;
    if (needsPath) {
      // A profile that just acquired a path is exactly what the recents
      // memory is for; an already-pathed save leaves it alone, as before.
      const persisted = await rememberRecentProfile(path);
      if (persisted) {
        recents.value = persisted.recent_profiles;
      }
    }
  } catch (e) {
    const err = e as IpcError;
    ipcErrorCode.value = err.code;
    ipcErrorParams.value = err.params;
  } finally {
    saving.value = false;
  }
}
```

  The import becomes `import { open as openDialog, save as saveDialog } from "@tauri-apps/plugin-dialog";`. `dialog:allow-save` is already granted, so no capability file changes (authoring section).

- [ ] **Step 4: `EditorView.vue`'s template.** Five edits:
  - A New button immediately BEFORE the existing Open button (parity: New precedes Open in the reference tool's menu), `type="button"`, `data-testid="editor-new"`, `:disabled="opening || saving"`, `@click="createBlank"`, label `{{ $t("editor-action-new") }}`. No `title` attribute: no button in this view carries one.
  - After the existing `<p v-if="currentPath">` path line, `<p v-else-if="sessionActive" data-testid="editor-unsaved">{{ $t("editor-unsaved") }}</p>`.
  - An empty-state paragraph, `v-if="!model"`, `data-testid="editor-empty"`, rendering `{{ $t("editor-empty") }}`, placed after the path/unsaved lines and before the recents section.
  - The recents section's gate changes from `!currentPath && recents.length` to `!model && recents.length`.
  - The diagnostics `<section>` gains `v-if="diagnostics.length"`. Its heading, id and `DiagnosticsPanel` mount are otherwise untouched, and `DiagnosticsPanel` itself is NOT edited.

- [ ] **Step 5: the two falsified comment regions.**
  - The Task-13 doc block's sentence explaining that the validate-on-edit watcher is gated on `currentPath` because only Open sets it is rewritten to name `sessionActive` and to state both funnels that set it. The block's other content is preserved verbatim.
  - The sentence stating `gui-editor.ftl stays 45` is corrected to the recomputed number. **Recompute it from the file rather than copying this plan:** `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl` after Step 6, which must equal 49 (46 measured at authoring plus this task's three). The same correction applies to the `catalog budget is 45` comment in `e2e/smoke.spec.ts`, whose decomposition becomes 42 labels + 1 save-surface note + 2 generic action keys + 1 rule-grid ordinal + this task's 3. **If the recount disagrees with 49, that is a finding: return NEEDS_CONTEXT with both numbers pasted.** Task 4 raises both numbers again by two and Task 5 by three; each task recomputes rather than predicting.

- [ ] **Step 6: the catalogs, both locales, fenced.** Append to `locales/en/gui-editor.ftl`, after its existing generic-action section, exactly:

```
## Profile creation and the pre-session state

editor-action-new = New profile
editor-empty = No profile open. Create one with New profile, or choose an existing profile file.
editor-unsaved = New profile, not saved yet.
```

  and to `locales/de/gui-editor.ftl`, in the same position, exactly:

```
## Profilerstellung und der Zustand vor dem Öffnen

editor-action-new = Neues Profil
editor-empty = Kein Profil geöffnet. Erstelle eines mit Neues Profil oder wähle eine vorhandene Profildatei aus.
editor-unsaved = Neues Profil, noch nicht gespeichert.
```

  None of the six values carries a placeable or an attribute, so attribute-name and pattern-structure parity hold by construction. None is a registry `labelKey`, so D55 rule 3's tooltip duty does not reach them.

- [ ] **Step 7: the tests, a new describe in `e2e/smoke.spec.ts`**, placed after the recents describe, using that file's existing `MKVMERGE_INFO`, `en`/`name` helpers and `assertNoSeriousA11yViolations`. Fixtures: a `warnReport` carrying the measured seed diagnostic (`empty-match-expression`, warning, `tracks[0].match`) and a `cleanReport`; a `PICKED_PATH` distinct from every other path literal in the file, so an identity assertion cannot pass on a shared value.
  - **Case 1, New creates and validates.** Nav to the editor, click `editor-new`. Assert: one `editor-rule-row`; the pattern field carries `.*`; `editor-unsaved` visible and no `batch-profile-current` line; `editor-empty` gone; and the recorded `validate_profile_model` invocation exists, carrying the seed's `input.extensions` - the wire half of the decoupling, since no path exists.
  - **Case 2, the seed is warned, not blocked.** Same flow with `warnReport`: the diagnostics panel lists the `empty-match-expression` line through the en catalog, and `editor-save` is enabled.
  - **Case 3, the pre-session empty state.** Nav to the editor and assert nothing else: `editor-empty` visible, and **absence check E1**, `section[aria-labelledby="editor-diagnostics-heading"]` has count 0. **Its fire is in the same test:** click `editor-new` with `warnReport` mocked and the same locator must have count 1. Run `assertNoSeriousA11yViolations` on both states.
  - **Case 4, Save with no path.** New, then Save with `plugin:dialog|save` mocked to return `PICKED_PATH`. Assert the recorded `plugin:dialog|save` call; the recorded `save_profile` call with `path === PICKED_PATH` and a profile whose `tracks.rules` has length 1; then the `batch-profile-current` line for `PICKED_PATH`; then a recorded `set_settings` whose `recent_profiles[0]` is `PICKED_PATH`.
  - **Case 5, the cancelled dialog.** Same, with `plugin:dialog|save` returning `null`. **Absence check E2:** no `save_profile` call. **Its fire is the recorded `plugin:dialog|save` call in the same test** (the flow ran) plus case 4's non-zero counter for the same command.
  - **Case 6, an already-pathed save is unchanged.** Open a profile, edit, Save: no `plugin:dialog|save` call at all, and `save_profile` carries the opened path. This is the regression guard for the branch, and its fire is case 4.

- [ ] **Step 8: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green. Every pre-existing test passes unchanged; in particular the three `editor-save` assertions in `e2e/editor-rule-add-remove.spec.ts` all run after an Open (measured at authoring), so removing `currentPath` from `saveDisabled` cannot change them, and any change in a pre-existing test's behaviour is a defect signal -> NEEDS_CONTEXT. `git diff --stat` covers exactly the four files in the Files list.

- [ ] **Step 9: commit.**

```bash
git add src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "editor: New creates a blank profile, and the path stops gating validation and Save" -- src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the seed (measured, not chosen); which duty each replacement covers; that Save opens the dialog and no Save-as action is added; that New renders before Open; that the diagnostics section is gated on content rather than on the session; that no "no diagnostics" line is added; that `DiagnosticsPanel.vue` is not edited; that Batch gains no New button; the six fenced catalog values; that the recents memory is fed only when a path is newly established.

---

## Task 4: undo/redo over the mutation funnel, and the save state derived from it (W3)

Read first: this plan's D108 in full and the authoring section's mutation-path enumeration with both expressions; **this plan's D112 in full** (amendment 1, the owner's failed-load ruling of 2026-07-31); `docs/ROADMAP.md`'s v1.x entry "Editor undo/redo, all operations" (the requirement set); `src/views/EditorView.vue` as Task 3 left it, in full; `src/editor/widgets/TextWidget.vue` (the per-keystroke binding this task's granularity rule works around); `e2e/editor-rule-add-remove.spec.ts` in full (its bare-mount cases, its header doc, and the Add/Remove interactions the mutation-path cases reuse); **`e2e/smoke.spec.ts`'s recents describe** (its `settingsWith` helper, which is where the `AppSettings` fixture shape D112's case needs is written out, and its `editor-recent-profile` locator, the only one of the recents surface's two testids that any spec currently uses - **`editor-recents` appears nowhere under `e2e/`** and is read from `src/views/EditorView.vue`'s template, where both are defined); `eslint.config.js` (its per-`.vue` rules block, which Step 4c extends, and the comment above the existing rule, which is the form the new one follows); `e2e/mount.ts` (`mountComponent`, `readModel`); the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src/views/EditorView.vue` (the history state, the push rule inside the existing watcher, the coalescing boundary, undo/redo functions, the keyboard handler, `sessionActive` becoming a computed, the two funnels' baseline calls **and the load-bearing-order comment above them**, the two buttons, **`doSave`'s post-write region** - the one line that marks the written profile, and the only part of `doSave` this task touches - **and, per amendment 1, the `nothingOpenedOrCreated` computed plus the two template gates that read it**: the `editor-empty` paragraph's and the `editor-recents` section's)
- Modify: `locales/en/gui-editor.ftl` (two new ids)
- Modify: `locales/de/gui-editor.ftl` (the same two)
- Modify: `e2e/editor-rule-add-remove.spec.ts` (**the header doc sentence this package falsified**: the bare-mount cases' reason is `sessionActive`, not `currentPath`)
- Create: `e2e/editor-undo-redo.spec.ts` (the mutation-path table, the undo/redo cases, **and amendment 1's pre-session/failed-load case**)
- Modify: `e2e/smoke.spec.ts` (**the catalog-budget comment only**, recomputed)
- Modify: `eslint.config.js` (**the one `vue/no-restricted-syntax` entry of Step 4c, inside the EXISTING per-`.vue` rules block, and nothing else in the file** - amendment 1)

**What amendment 1 moved in this list, stated so it is checkable rather than asserted.** The gate condition and its two gates live in `src/views/EditorView.vue` and its case lives in `e2e/editor-undo-redo.spec.ts`; both were already members, and D112 adds no catalog string, so neither `.ftl` moves. **`eslint.config.js` is the one addition**, and it is therefore also added to the commit block below. It is a named region: the rules block gains one entry, no other rule and no other part of the config changes, and **no gate part is added** - `pnpm lint` already runs `eslint .`.

**Interfaces:**
- Consumes: Task 3's `createBlank` and `sessionActive`.
- Produces: `dirty`, which Tasks 5 and 6 gate their guards on, and `savedSnapshot`, which `sessionActive` now derives from.

- [ ] **Step 1: the history state and the push rule.** Add to the script, with doc comments carrying the reasoning D108 records:
  - `const history = ref<string[]>([]);`, `const position = ref(-1);`, `const savedSnapshot = ref<string | null>(null);`, `let coalesce = false;`
  - `sessionActive` changes from a ref to `const sessionActive = computed(() => savedSnapshot.value !== null);`, and Task 3's two assignment sites drop their `sessionActive.value = true` in favour of establishing the baseline (below). The doc comment states that the two facts are established at the same two moments by construction.
  - `const dirty = computed(() => savedSnapshot.value !== null && history.value[position.value] !== savedSnapshot.value);`
  - `const canUndo = computed(() => position.value > 0);` and `const canRedo = computed(() => position.value < history.value.length - 1);`
  - A `resetHistory(profile: Profile | undefined)` helper used by `openPath` and `createBlank`, with **both branches fenced** (D108 decisions 8 and 9): given a profile, history becomes a single entry holding `JSON.stringify(profile)`, `position` 0, `savedSnapshot` that same string, `coalesce` false; given `undefined` - the failed-load branch, where `doc.profile` is null and the diagnostic carries the parse error - history becomes empty, `position` `-1`, `savedSnapshot` `null`, `coalesce` false. `openPath` calls it with `doc.profile ?? undefined` on the same value it assigns to the model, so the two can never disagree.
    **The call site's POSITION is load-bearing and is inherited rather than invented**: `resetHistory` takes the place of the `sessionActive.value = true` assignment Task 3 put **before** the model assignment, and it must stay before it for the two reasons that made that order load-bearing there and one more of its own - `sessionActive` (now derived from `savedSnapshot`) must already be true when the watcher fires on the assignment, or the loaded profile is never validated; and `history[0]` must already equal the serialized model, or the push rule sees a difference and appends a second entry, so a freshly opened profile would start one step deep and dirty. **Task 3's comment naming that order as load-bearing is updated in this task to name `resetHistory`** rather than the assignment it replaces, since the requirement outlives the statement that carried it.
  - Inside the EXISTING `watch(model)`, before the validation round trip, the push rule:

```ts
  const serialized = JSON.stringify(value);
  if (serialized !== history.value[position.value]) {
    if (coalesce) {
      history.value = [...history.value.slice(0, position.value), serialized];
    } else {
      history.value = [...history.value.slice(0, position.value + 1), serialized];
      position.value = history.value.length - 1;
      if (history.value.length > HISTORY_DEPTH) {
        history.value = history.value.slice(1);
        position.value -= 1;
      }
      coalesce = true;
    }
  }
```

    with `const HISTORY_DEPTH = 100;` and a comment stating why the rule is a comparison rather than a flag (an undo-driven assignment equals the entry at the new position, so it cannot push; no applying-history latch exists to get wrong), and what the depth costs (the measured snapshot sizes).

- [ ] **Step 1c: mark the profile that was WRITTEN, in `doSave`.** Immediately after `await saveProfile(path, profile)` resolves - **inside the existing `try`, so a failed save leaves the state dirty** - add exactly:

```ts
    savedSnapshot.value = JSON.stringify(profile);
```

  **The value is the captured `profile`, never `history.value[position.value]`, and the difference is the whole point of the line.** Two awaits sit between Task 3's capture (`const profile = model.value;`) and this point - the save dialog on the needs-path branch, and the write itself - and `saving.value = true` disables only `editor-save`, `editor-new` and `editor-open`, not the widgets, so the editing surface stays live through both. Marking the live position would therefore record a state that was never written whenever the model moved inside either window: `dirty` would go false while the file holds the older profile, and **every guard in the D109 family would disarm over unsaved changes** - the data-loss direction, where D108 decision 4 promises annoyance. Marking the captured value instead makes the property structural: when nothing moved, `profile` is the same object the last push serialized, so the string equals `history[position]` and `dirty` is false as intended; when it moved, the two differ and `dirty` stays true.

  **Why the parity precedent does not license the live read**, stated because this plan cites that precedent and an earlier draft over-read it: in `mkvtoolnix-gui` the sequence `updateConfigFromControlValues(); p.config.save(); p.savedState = currentState();` is **fully synchronous**, so there `currentState()` IS what was written. The borrowed shape carries that condition with it, and a flow with an `await` between the capture and the mark does not meet it. Comment the line accordingly: it names the written value, and names synchrony as the condition the precedent's own form depends on.

  **Nothing else in `doSave` changes**: Task 3 owns its dialog branch, its capture discipline and its recents write, and this task adds one line after the write. Omitting the line entirely is the opposite defect and is just as visible: `dirty` would never return to false and every guard would fire on a profile that was just saved, which is the disposition the owner overruled when he gated the family on save state.

- [ ] **Step 2: the coalescing boundary.** `addRule`, `removeSelectedRule` and `onDrop` each set `coalesce = false;` as their first statement, with one shared comment stating why focus alone is not enough (two consecutive clicks of the same button never move focus). The editor's root `<section>` gains `@focusout="coalesce = false"`, with a comment recording the measured fact that `focusout` bubbles where `blur` does not.

- [ ] **Step 3: undo, redo, and the keyboard.** Two functions applying a history entry - moving `position`, assigning `model.value = JSON.parse(history.value[position.value]) as Profile`, clearing `selectedIndex` (a selection maps to an identity, not a position, the rule `onDrop` already follows) and setting `coalesce = false` - each guarded by `canUndo`/`canRedo` **and by `model.value` being set** (D108 decision 10: the action row and the keyboard handler both sit outside the `v-if="model"` wrapper, so neither may apply an entry while the editor holds nothing). Then a `@keydown` handler on the editor's root section, whose condition set is fixed here and whose comment states the no-per-OS-branch reasoning:
  - ignore the event entirely when its target is a text-entry control - `TEXTAREA`, or `INPUT` whose `type` is one of `text`, `search`, `url`, `tel`, `password`, `email`, `number` - so the browser's native character-level undo keeps working while typing;
  - undo on `(ctrlKey || metaKey) && !shiftKey && key.toLowerCase() === "z"`;
  - redo on `(ctrlKey || metaKey) && shiftKey && key.toLowerCase() === "z"`, or `(ctrlKey || metaKey) && key.toLowerCase() === "y"`;
  - `preventDefault()` only on a handled combination.

- [ ] **Step 4: the two buttons and their catalog ids.** In the action row after New and Open: `data-testid="editor-undo"` / `editor-redo`, `:disabled="!model || !canUndo"` / `:disabled="!model || !canRedo"` (the model term per D108 decision 10, since this row renders outside the editing surface's own gate), labels `{{ $t("editor-action-undo") }}` / `{{ $t("editor-action-redo") }}`, no `title`. Append to `locales/en/gui-editor.ftl`'s generic-action section exactly

```
editor-action-undo = Undo
editor-action-redo = Redo
```

  and to `locales/de/gui-editor.ftl` exactly

```
editor-action-undo = Rückgängig
editor-action-redo = Wiederholen
```

- [ ] **Step 4b (amendment 1): the pre-session gate condition, defined once (D112).** Two edits, both fenced, and nothing else about the template moves.
  - In the script, immediately below `sessionActive`'s new computed form from Step 1:

```ts
// D112 (owner ruling 2026-07-31): the pre-session state -- nothing has
// been opened or created at all -- and the ONE definition both surfaces
// that may appear only in that state read. Two terms, because two facts
// have to be absent at once: `model` carries "the editor holds
// something", `currentPath` carries "a file has been bound to the
// editor". A load that resolves but fails to parse leaves the second set
// and clears the first (`openPath` sets `currentPath`, then assigns
// `doc.profile ?? undefined`), and that state is the one the ruling is
// about: the path line names the failing file and the panel carries the
// parse error, so a second sentence saying no profile is open, plus a
// recents list offering a fresh start, contradict what is already on
// screen.
//
// NOT `sessionActive`: this task derives it from `savedSnapshot`, which
// the failed-load branch nulls (D108 decision 9), so `!sessionActive` is
// TRUE in exactly the state these two surfaces must stay hidden in. NOT
// `!model` alone: that is the gate Task 3 shipped, and it is what renders
// both surfaces over a failed load today.
const nothingOpenedOrCreated = computed(
  () => !model.value && currentPath.value === null,
);
```

  - In the template, the two gates Task 3 wrote become the two below, and no other gate is touched:

```
      v-if="nothingOpenedOrCreated"
```

    on the `editor-empty` paragraph (Task 3 wrote `v-if="!model"`), and

```
      v-if="nothingOpenedOrCreated && recents.length"
```

    on the `editor-recents` section (Task 3 wrote `v-if="!model && recents.length"`).

  **What stays exactly as it is, because the ruling keeps it rendering in the state it changes:** `<template v-if="model">` on the editing surface, `v-if="currentPath"` on the path line with its `v-else-if="sessionActive"` unsaved branch, and `v-if="diagnostics.length"` on the diagnostics section. **Task 3's own steps are not rewritten** - they record what that task built, measured and committed, and the gates above name the state they start from so the sequence stays legible (`proc-supersede-never-overwrite`).

- [ ] **Step 4c (amendment 1): the standing guard for the property Step 4b establishes (D112's standing-guard decision).** Add exactly this entry to `eslint.config.js`, as the FIRST rule in the existing `**/*.vue` rules block, immediately above `@intlify/vue-i18n/no-raw-text`. Nothing else in that file changes, and **no gate part is added**: `pnpm lint` already runs `eslint .`.

```js
      // D112 (owner ruling 2026-07-31): the pre-session state is ONE named
      // computed, `nothingOpenedOrCreated`, and a render gate that asks
      // `!model` directly is the defect that decision exists to remove --
      // that expression is also true after a load that failed to parse,
      // where the editor must NOT offer its pre-session surfaces. Scoped by
      // directive name to `v-if`/`v-else-if`, so the `:disabled="!model ||
      // !canUndo"` bindings D108 decision 10 requires stay legal: those gate
      // an ACTION on whether there is content, not a RENDER on whether
      // anything was ever opened or created.
      "vue/no-restricted-syntax": [
        "error",
        {
          selector:
            "VAttribute[directive=true][key.name.name=/^(if|else-if)$/] UnaryExpression[operator='!'] > Identifier[name='model']",
          message:
            "A render gate must not read `!model` directly: the pre-session state is `nothingOpenedOrCreated` (D112).",
        },
      ],
```

  **Falsifiability, in the form this plan requires - the expression, a red state with its exact expected non-zero result, and the green end state - with every figure measured at amendment time against the tree as Task 3 left it** (the runs are pasted in the amendment report; re-derive rather than trust them):
  - **The instrument exists before the rule is written.** `eslint-plugin-vue` is a direct devDependency pinned in `package.json` (**10.9.2** at amendment time, read from the installed package's own manifest), and that version ships `vue/no-restricted-syntax`. If either is false in the tree this task meets, that is a finding: NEEDS_CONTEXT with the run pasted, not a rule adjusted at the keyboard.
  - **RED, the pre-state: exactly 2 errors of `vue/no-restricted-syntax`** in `src/views/EditorView.vue`, one on the `editor-empty` paragraph's gate and one on the `editor-recents` section's. **The two gates are the fence, not their line numbers**, which move as this task's other steps add script above them. If Step 4b is already applied when this step runs, restore those two gates for the red run and re-apply them: the red state is the tree as Task 3 left it.
  - **GREEN, the end state: 0**, with Step 4's two `:disabled="!model || !canUndo"` / `:disabled="!model || !canRedo"` bindings PRESENT in the file. That is the over-match control and it is not optional: a selector that also caught those would make this rule and D108 decision 10 mutually unsatisfiable, and the green run is what proves it does not.
  - **The selector's own enumerated set is fired member by member**, because a set inside an instrument is a claim like any other: `v-if` fires in the red run above, and `v-else-if` is fired separately by pointing the existing `v-else-if` at `!model` once and watching the rule report it. A regex branch that never fires is a branch that was never tested.

- [ ] **Step 5: the mutation-path coverage, as an enumerated table.** In `e2e/editor-undo-redo.spec.ts`, a table with one entry per mutation path measured at authoring, each generating its own `test()` on a fresh page. **The set is the six functions the authoring expression returned, and it is closed:** `setFieldValue`, `setTracksUnmatched`, `setRuleValue`, `onDrop`, `addRule`, `removeSelectedRule`. Each case, on the served app with mocked IPC: open a profile whose fixture reaches that path, perform the mutation through the real control, then assert (a) Undo is enabled, (b) one Undo restores the pre-mutation rendering, (c) Redo re-applies it. **Per path, not one test for the mechanism**, and the entry names the function it covers so a reviewer can map the table to the expression's output.
  - **Re-derive the set before writing the table**, with both authoring expressions pasted into the report: the whole-value expression and the in-place-mutation expression with its own fired control. **A seventh mutation path is a finding: NEEDS_CONTEXT, not a seventh row invented at the keyboard.**

- [ ] **Step 6: the remaining cases in the same file.**
  - **Granularity, three halves in one flow.** Type several characters into the pattern field: one Undo restores the field's pre-typing value in full (the burst is one step). Then type, move focus to another control, type again: two Undos are needed, one per burst. Then click Add twice: two Undos are needed. Playwright's `fill()` dispatches one input event and its own `change`, so a test that needs two bursts moves focus explicitly between them (measured).
  - **Truncation:** undo once, then edit; Redo is disabled.
  - **Save marks rather than clears:** open, edit, Save (mocked), then Undo is still enabled and one Undo restores the pre-edit state.
  - **Open resets:** with history built, open another profile; Undo and Redo are both disabled.
  - **A failed open clears rather than keeps** (D108 decisions 9 and 10): with history built and Undo enabled, open a second path whose mocked `load_profile` returns a document with `profile: null` and a parse diagnostic. Assert, in this order: the diagnostic renders (the section is gated on content, so a failed open still explains itself); Undo and Redo are both disabled; and the editing surface is gone. **Its own control is the state before the failed open**, where Undo was enabled in the same test - so a test that passes because Undo is never enabled anywhere cannot be mistaken for this one passing.
  - **The depth cap:** drive more than `HISTORY_DEPTH` distinct discrete mutations (Add repeated), then assert that Undo cannot reach the original state - the count is derived from the constant, not hardcoded twice.
  - **Absence check U1, the text-entry exemption.** With focus in the pattern field, the undo combination must NOT change the model (`readModel` on a mount-harness case, or the rendered rule count on the served app). **Its fire is in the same test:** the identical combination with focus on a button DOES undo. Two runs of one exemption, so a branch that swallows everything cannot pass.
  - **The mount-harness property is preserved and asserted by its own file:** `e2e/editor-rule-add-remove.spec.ts`'s bare-mount cases must pass unchanged, and its header doc is corrected to name `sessionActive` as the reason.
  - **Amendment 1's case: a failed load hides both pre-session surfaces, and the pre-session state still shows them** (D112). ONE test, three legs in one flow, so each leg is a control for the others. Its scenario mocks exactly five commands: `detect_mkvmerge` with `MKVMERGE_INFO`; `get_settings` resolving with a settings object whose `recent_profiles` carries the path leg 2 opens (the `settingsWith` shape `e2e/smoke.spec.ts`'s recents describe writes out); `plugin:dialog|open` queued as `[<that path>, <the failing path>]`; `load_profile` queued as `[<a document carrying a profile>, <a document whose profile is null and whose config_diagnostics carries one parse-error entry>]`; and `validate_profile_model` resolving with a diagnostic-free report. `set_settings` is deliberately NOT mocked - `e2e/mocks.ts`'s own fallback answers it, which is the fixture shape the shipped recents cases already use. **The model is never edited in this flow**, so `dirty` stays false and the discard guard Task 5 later adds to `pickAndOpen` cannot change what this test does.
    - **Leg 1, before any click - this is absence check P2's FIRE:** `editor-empty` is visible and `editor-recents` has count **1**.
    - **Leg 2, after the successful open:** `editor-empty` and `editor-recents` both count **0**. This is the gate Task 3 shipped, asserted so that leg 3 cannot be read as covering it.
    - **Leg 3, after the failing open - P2's zero:** `editor-empty` and `editor-recents` both count **0**, AND the open-path line renders the failing path (`batch-profile-current`), AND the parse-error diagnostic renders.
    - **Why leg 3's zero on the recents section is not vacuous:** that gate is a conjunction, so a zero could mean either term is false. `recents` is non-empty throughout this flow - the `get_settings` mock seeds one path, and each successful open writes another through `rememberRecentProfile` - so the term that is false in leg 3 is the gate, and leg 1's count of 1 is that same list measured through the same locator before the flow starts.

- [ ] **Step 7: verification.**
  - **Absence check S1, the saved position is never marked from LIVE state, by either route.** `grep -nE 'savedSnapshot\.value *= *(history|JSON\.stringify\(model)' src/views/EditorView.vue`, expected **0** on the end state. **The alternation has two members and both are fired**, each against its own synthetic line - `savedSnapshot.value = history.value[position.value];` and `savedSnapshot.value = JSON.stringify(model.value);` - because firing one member leaves the other unproven, the same rule this plan applies to its own gate audit's per-alternative fires. The second member is not decoration: it is the **likelier** re-break, since the fixed line is `JSON.stringify(profile)` and a simplifier reaching for the model finds `model.value` in scope right there, so the wrong version differs from the right one by one word. **This is not the widening D1 declined:** a second exact expression adds no prose surface and cannot produce triage, where a broader *name* pattern would. **Its pre-state is empty by construction** (neither line exists yet), so the two fires are the only thing that makes the zero mean anything. **What it does not cover, because a grep cannot follow a binding:** the same defect written through a local const (`const live = history.value[position.value]; savedSnapshot.value = live;`) escapes both members. **This check exists because the defect it catches was introduced by a fix round of this very plan**, which is where a structural property gets quietly re-broken: the assignment reads correct in isolation and is wrong only in the presence of the two awaits above it.
  - **The behavioural gap check, and why it is not prescribed here - a deliberate, argued gap rather than an omission.** A test that moves the model INSIDE one of the two awaits would be the direct producer, and the existing harness cannot express it: `e2e/mocks.ts` resolves a queued value immediately and its scenario crosses a `page.addInitScript` serialization boundary ("it must not close over anything from this module's scope beyond the `scenario` argument"), so there is no way to hold a mocked `plugin:dialog|save` open until the test releases it; a Playwright action issued after the click races the microtask that resolves it, and this project has an owner call against flakiness. A releasable mock response is **new test infrastructure**, which is the one exemption `tests-ship-with-the-feature-never-after` names, so it is surfaced for controller routing rather than written here as "coverage follows later". What ships instead is stronger than a comment and weaker than that test: the fix makes the property structural, S1 pins the structure, and the report states the residual.
  - **Absence check D1, no second save-state mechanism** (R28, and it costs one line because its red state is already measured). `grep -nEi "dirty|isDirty|unsaved|modified" src/views/EditorView.vue`. **RED, the pre-state, already measured on the baseline: 0 lines**, with the control that the same pattern returns `hasBeenModified`/`savedState` against `~/Downloads/mkvtoolnix/src/mkvtoolnix-gui/merge/tab.cpp`, so an empty result is a real absence. **GREEN, the end state: exactly the derived members and nothing else** - the `dirty` computed and its doc comment, and no assignment of the form `dirty.value =` or `<name>Dirty = ref(`. The end-state expression is therefore `grep -nE "dirty\.value *=|(isDirty|unsavedChanges|modified) *= *ref\(" src/views/EditorView.vue`, whose expected result is **0**, fired once against a synthetic line carrying `dirty.value = true` to prove it matches an assignment when one exists. **A second boolean introduced in a later fix round is what this catches**, which is the whole content of R28. **The two alternatives are not equally strong, and the asymmetry is stated rather than widened:** the first is structural and catches any reassignment of the derived value whatever it is called; the second is three plausible names, so a rival flag called something else escapes it. Not widened, because a broader name pattern would match ordinary prose and turn the check into triage; recorded so a later reader does not read D1 as exhaustive over rival mechanisms.
  - **Absence check P1 (amendment 1), no surface still carries a bare `!model` gate** - D112's "defined once", pinned structurally. `grep -nE 'v-if="!model' src/views/EditorView.vue`. **RED, the pre-state, measured on the tree as Task 3 left it: exactly 2 lines**, the `editor-empty` paragraph's gate and the `editor-recents` section's, which are the two Step 4b replaces. **GREEN, the end state: 0.** **That pre-state run IS the fire** - the same expression on the same file returning a non-zero result - so the zero afterwards cannot be a pattern that matches nothing anywhere. **An implementer whose pre-state recount disagrees with 2 returns NEEDS_CONTEXT with both runs pasted** rather than adjusting the fence. **What it does not cover, recorded so it is not read as more than it is:** it matches the exact spelling `v-if="!model`, so a gate written with a space after the bang, or one whose terms are ordered the other way (`v-if="currentPath === null && !model"`), escapes it. **Those are exactly the cases Step 4c's lint rule catches**, because that rule reads the parsed template rather than the file's characters, and it is the standing guard where P1 is the one-shot demonstration that the pre-state was red. P1 is kept beside it rather than replaced by it (`proc-proposed-safeguard-stays`): it is one line, its pre-state is already measured, and a grep and a lint rule fail for different reasons.
  - **The lint rule's green end state is not a separate run:** `pnpm lint` is inside the gate below, and Step 4c's rule is an error-severity rule in it, so the gate's own green run is the rule's green run. Paste Step 4c's red run and this green one side by side.
  - The full gate as `BUILDING.md` enumerates it, foreground, green. The `gui-editor.ftl` recount after this task must be 51; recompute it and correct the two comments (`src/views/EditorView.vue` was corrected by Task 3 and is corrected again here; `e2e/smoke.spec.ts`'s budget comment likewise). A disagreement is a finding -> NEEDS_CONTEXT. **NOT DISCHARGED IN FULL, recorded rather than rewritten (amendment 2):** Task 4 corrected `e2e/smoke.spec.ts` and left `src/views/EditorView.vue`'s sentence at its Task-3 figure, and both its review and its delta review graded this step MET. The step stands as what was required; the correction lands in Task 5 Step 4c on Task 5's own authority, since Task 5's three ids falsify that sentence again. `git diff --stat` covers exactly the seven files in the Files list (six before amendment 1 added `eslint.config.js`).

- [ ] **Step 8: commit.**

```bash
git add src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/editor-rule-add-remove.spec.ts e2e/editor-undo-redo.spec.ts e2e/smoke.spec.ts eslint.config.js
git -c commit.gpgsign=false commit -m "editor: undo/redo over the one mutation funnel, and the unsaved state derived from its history" -- src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/editor-rule-add-remove.spec.ts e2e/editor-undo-redo.spec.ts e2e/smoke.spec.ts eslint.config.js
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the push rule's comparison form; the coalescing boundary and its three explicit resets; the depth constant; the keyboard condition set including the text-entry exemption and its enumerated input types; that the save state is derived and no second flag exists; that saving marks a position; that opening resets; the two fenced labels; that the mutation set is the six the expression returned and a seventh is a finding; **and, from amendment 1**: the pre-session condition's two terms and its fenced expression; that it is one named computed both gates read rather than two inline conditions; that it is NOT derived from `sessionActive` and adds no third state flag; that the path line, the unsaved line, the diagnostics section and the editing surface keep their own gates; that the failed-load state adds no catalog string; that Task 3's shipped steps are not rewritten; that the standing guard is one `vue/no-restricted-syntax` entry in the existing per-`.vue` rules block and adds no gate part, no dependency and no second config file; the selector and the message, both fenced; that it is scoped to `v-if`/`v-else-if` so the `:disabled` bindings stay legal; and that absence check P1 stays beside it rather than being replaced by it.

---

## Task 5: the discard guards in the editor (W4a)

Read first: this plan's D109 in full; **this plan's D112** (amendment 1), whose pre-session condition is the gate this task's step 2 argument and its case 5 both consume; `docs/ROADMAP.md`'s round-3 finding 2 second owner ruling (the guard's shape and its ordering) and the sharpened first one; `src/views/EditorView.vue` as Task 4 left it; `src/components/SettingsDialog.vue` (the native-`<dialog>` house pattern, its `defineExpose` shape and its Esc note); `e2e/smoke.spec.ts`'s existing view-switch case (which this task extends); the amended spec section 8.2. Model tier: mid.

**Files (EXHAUSTIVE):**
- Create: `src/components/ConfirmDialog.vue`
- Modify: `src/views/EditorView.vue` (the dialog mount, the two guarded call sites, **and the stale catalog-budget sentence in the file's own header doc block** - amendment 2, Step 5)
- Modify: `locales/en/gui-editor.ftl` (three new ids)
- Modify: `locales/de/gui-editor.ftl` (the same three)
- Modify: `e2e/smoke.spec.ts` (the guard cases; the extended view-switch case; **the catalog-budget comment**, recomputed)
- Modify: `e2e/editor-undo-redo.spec.ts` (**the three named cases this task's own guard fronts with a confirm**, and only those: "open resets", "createBlank resets" and the failed-open case; Step 4b derives the set from a stated criterion, so a reviewer can reproduce it rather than trust the list. No other case in that file is touched - amendment 1, set corrected by amendment 2)

**Why the header-comment region is in scope rather than fenced off, and what it records.** This task adds three ids to `gui-editor.ftl`, which falsifies that file's own sentence about how many the catalog carries - the same ground Task 3's Files list gives for the two comment regions it repaired, and what `proc-normative-count-recomputed` exists to prevent. **The miss it also closes is on the record rather than quietly repaired:** Task 4's Step 7 named `src/views/EditorView.vue` and `e2e/smoke.spec.ts` together and required both budget comments recomputed; Task 4 corrected only the second, and its review and its delta review both graded that step MET, so the sentence has been stale across two packages. Task 5 does not inherit Task 4's authority for that - **its own authority is that its own three ids falsify the sentence again**, which is why the correction belongs here and not in a sweep with no owner.

**Interfaces:**
- Consumes: Task 4's `dirty`.
- Produces: `ConfirmDialog`, whose props are the minimum a second caller needs.

- [ ] **Step 1: `ConfirmDialog.vue`.** A native `<dialog data-testid="confirm-dialog">` with `showModal()`, mirroring `SettingsDialog.vue`'s imperative pattern: props `title`, `message`, `confirmLabel`, `cancelLabel` (all strings, so the caller resolves its own Fluent text and the no-raw-text rule is satisfied by binding), `defineExpose({ ask })` returning `Promise<boolean>`, resolved `true` by the confirm button and `false` by the cancel button, by `close` and by Esc (the native cancel, which reads as "do not discard" - the safe direction, and the same Esc semantics the settings dialog documents). The confirm button carries `data-testid="confirm-dialog-confirm"`, the cancel button `confirm-dialog-cancel`. A doc comment states that the component exists rather than an inline dialog so a second caller can reuse it, and that its props are the minimum for that.

- [ ] **Step 2: the two guarded call sites.** `EditorView` mounts one `ConfirmDialog` with the three discard strings and `settings-cancel` as the cancel label (the same cross-view reuse the editor's Save already makes of `settings-save`). Then:
  - `pickAndOpen` begins, after its existing busy guard, with `if (dirty.value && !(await confirmEl.value?.ask())) { return; }` - **before** the open dialog, so the ordering is confirm, then file dialog, then replace. A cancelled file dialog after a confirmed discard leaves the model untouched, because nothing is discarded until a load succeeds.
  - `createBlank` gains the same guard. Because `createBlank` is synchronous today, it becomes `async` and its click handler awaits it; nothing else about it changes.
  - `openPath` gains NO guard: it runs after the dialog, and the recents-click path that also reaches it is unreachable while the editor holds a model (the recents section is gated on the pre-session condition Task 4 defines under D112, whose `!model` term alone carries that unreachability).

- [ ] **Step 3: the catalog, both locales, fenced.** Append to `locales/en/gui-editor.ftl` exactly:

```
## Discard confirmation (D109)

editor-discard-title = Unsaved changes
editor-discard-message = The profile in the editor has unsaved changes. Continuing replaces it and the changes are lost.
editor-discard-confirm = Discard changes
```

  and to `locales/de/gui-editor.ftl` exactly:

```
## Verwerfen-Bestätigung (D109)

editor-discard-title = Nicht gespeicherte Änderungen
editor-discard-message = Das Profil im Editor hat nicht gespeicherte Änderungen. Wenn du fortfährst, wird es ersetzt und die Änderungen sind verloren.
editor-discard-confirm = Änderungen verwerfen
```

- [ ] **Step 4: the tests, in `e2e/smoke.spec.ts`'s new guard describe.**
  - **Case 1, Open over unsaved changes, confirmed.** Open a profile, edit a field, click Open. **Absence check G1:** no `plugin:dialog|open` call recorded yet, while `confirm-dialog` is visible with its en message. **Its fire is the same counter after clicking confirm**, which must be non-zero, and the second profile must then be in the editor.
  - **Case 2, Open over unsaved changes, cancelled.** Same up to the dialog, then cancel: `plugin:dialog|open` still not recorded, the edited value still in the field, the dirty state still true (Undo still enabled).
  - **Case 3, Open with no unsaved changes, in both of its reachable shapes.** (i) Open a profile and click Open again without editing: **absence check G2**, `confirm-dialog` count 0, and `plugin:dialog|open` recorded immediately. (ii) **The after-a-save shape, which is W3-q2's producer:** open, edit (the confirm now appears on an Open click - assert it, then cancel), Save successfully, then click Open again and assert the confirm does NOT appear. **The two clicks in one test are each other's control**: the first proves the guard can fire in this scenario, the second proves the save cleared the state. Without leg (ii) a `savedSnapshot` frozen at the load baseline ships silently, because every other assertion in this package passes with it frozen. **Its fire is leg (ii)'s own first click**, not case 1's.
  - **Case 4, New over unsaved changes.** Edit, click New, confirm: the seed replaces the edited profile. Cancel: it does not. (If the owner strikes D109 decision 2, this case inverts to asserting no dialog; nothing else moves.)
  - **Case 5, the recents affordance is unreachable while a profile is held.** With a profile open, `editor-recents` count 0; **its fire** is count 1 (or the seeded recent button's presence) in the pre-session state of the same test.
  - **Case 6, the view-switch invariant, extended rather than duplicated.** The existing case "the editor tab stays mounted across a switch to Jobs and back" gains: the field is edited before the switch, and after the round trip the edited value, the enabled Undo button and the absence of any `confirm-dialog` are all asserted. This is R22's assertion and the reason nothing was built for it.
  - Run `assertNoSeriousA11yViolations` with the confirm dialog open.

- [ ] **Step 4b (amendment 1, criterion corrected by amendment 2): repair the cases in `e2e/editor-undo-redo.spec.ts` that this task's own guard fronts.**

  **The criterion, and it derives from what Step 2 guards rather than from one of the ways in.** Step 2 puts the confirm on **two functions**, `pickAndOpen` and `createBlank`, and on neither `openPath` nor anything else. A case in that file is a member of the affected set **iff it activates a control bound to one of those two functions at a moment when `dirty` is true**. Three facts make that decidable by reading, with no judgement left over:
  - **The controls are exactly two**, read from `EditorView.vue`'s template rather than assumed: `editor-open` is bound to `pickAndOpen` and `editor-new` to `createBlank`. `editor-recent-profile` is bound to `openPath`, which this task deliberately leaves unguarded, so a recents click is never a member - and it is unreachable while the editor holds a model anyway (D112's condition).
  - **`dirty` is true at a click iff a model mutation lies between that click and the most recent baseline before it.** A baseline is established by a successful open, a create, or a successful save (D108 decisions 3 and 8); a mutation is any of the six funnel functions D108 decision 1 enumerates, reached in a test through a field edit, a rule Add or Remove, or a drag-reorder.
  - **The derivation is therefore mechanical:** list every activation of those two controls in the file, and for each read backwards to the nearest baseline and ask whether a mutation lies between. **A case with no such activation is not a member however dirty it gets**, and a case that activates a guarded control from a clean baseline is not a member either.

  **Why the criterion is stated this way rather than as "builds history, then opens again":** that earlier phrasing was scoped to one of the two entry points, so it structurally could not see a case that replaces the editor's content through New. It was correct for the file as it stood and was falsified by a later ruling that added exactly such a case to Task 4. **A criterion narrower than the mandate it serves regenerates the defect on the next addition** (`a-normative-claim-is-scoped-down-to-its-producers-reach`, one level up), which is why the rule above is written against the guarded functions and not against a control.

  **The set, re-derived from that criterion against the file as it stands: THREE cases.**
  - **"open resets: opening a second profile clears both Undo and Redo"** - its first Open runs from a clean baseline and is not a member; its second Open follows a field edit and is.
  - **"createBlank resets: New after edited history clears both Undo and Redo"** - a field edit, then New. **This is the member the earlier criterion could not see**, and it entered the file through Task 4's own fix round after amendment 1 was written.
  - **"a failed open ..."** (the failed-open case D108 decisions 9 and 10 own) - its first Open is clean and is not a member; its second, after a field edit, is.

  **Every other case in the file fails the criterion, and the reason is stated per case rather than by exclusion:** the shared open helper every test starts with runs before any model exists, so its Open is clean; the six mutation-path cases mutate after that single open and activate no guarded control again; granularity, truncation, the depth cap and U1 activate no guarded control at all; "save marks rather than clears" edits and then saves, which re-establishes the baseline, and activates no guarded control afterwards; and amendment 1's D112 three-leg case activates Open twice but never mutates the model, so both clicks run from a baseline and `dirty` is false at each.

  - **The repair, in each of the three:** between the activation of the guarded control and the assertions that follow it, assert that `confirm-dialog` is visible, then click `confirm-dialog-confirm`.
  - **The added member takes the IDENTICAL repair, and that is a measurement rather than an assumption** (amendment 2, B-2). Step 2 mounts **one** `ConfirmDialog` in `EditorView`, and both guarded functions await that same instance's `ask()`, so the dialog that appears carries the same `confirm-dialog` and `confirm-dialog-confirm` testids whichever control opened it. The one difference between the entry points does **not** reach the repair: after a confirmed Open the flow continues into the file dialog, which those cases already mock, while after a confirmed New `createBlank` completes with no further IPC - so the New case needs **no additional mock, no additional wait and no different locator**. If the implementer finds that it does, that is NEEDS_CONTEXT, not a locator invented at the keyboard.
  - **No existing assertion in any of the three is removed, weakened, reordered or reworded, and no case changes what it is about.** Each keeps testing exactly what it was written to test, over an editor that is genuinely dirty - which is the state a real user reaches, and the reason the alternative repair (re-establishing a baseline before the guarded click so the confirm never fires) is rejected: it would swap the case's own subject to dodge a mechanism. `proc-proposed-safeguard-stays` binds here too - the assertions these cases already carry are the safeguard.
  - **Task 4 does not pre-adapt them.** `ConfirmDialog` does not exist until this task's Step 1, so those clicks would fail there. The task that introduces the guard repairs the cases its guard changes, which is the same shape Task 3 and Task 4 already use for the comment regions their own edits falsify.
  - **This produces no new observable and the acceptance map gains no row:** W4-a to W4-c already grade the guard on the Open path and W4-e on the New path. The repair keeps three existing producers alive rather than producing anything new.

- [ ] **Step 4c (amendment 2): the stale catalog-budget sentence in `EditorView.vue`'s header doc block.** Recompute the count from the file first - `grep -cE '^[A-Za-z][A-Za-z0-9_-]*\s*=' locales/en/gui-editor.ftl` after Step 3, which must equal **54**; **a disagreement is a finding -> NEEDS_CONTEXT with both numbers pasted**, not a fence adjusted at the keyboard. Then replace exactly

```
// packages did add to it: `gui-editor.ftl` carries 49 ids today, three of
// them this view's own New affordance (`editor-action-new`,
// `editor-empty`, `editor-unsaved`, D107). The Open button, the
```

  with exactly

```
// packages did add to it: `gui-editor.ftl` carries 54 ids today, eight
// of them this view's own affordances: profile creation
// (`editor-action-new`, `editor-empty`, `editor-unsaved`, D107),
// undo/redo (`editor-action-undo`, `editor-action-redo`, D108) and
// the discard confirmation (`editor-discard-title`,
// `editor-discard-message`, `editor-discard-confirm`,
// D109). The Open button, the
```

  Nothing else in that doc block changes. The eight decompose as this package's own additions to this view (3 + 2 + 3), and 46 + 8 = 54 against the count the authoring section measured before the package started.

- [ ] **Step 5: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green - **and the three repaired cases in `e2e/editor-undo-redo.spec.ts` are part of it**, so a repair that missed one shows up as a red gate here rather than at the push. **The `gui-editor.ftl` recount must be 54, and the budget comments it governs are exactly two, both of which this task corrects**: `e2e/smoke.spec.ts`'s (Step 4) and `src/views/EditorView.vue`'s header sentence (Step 4c). Naming both closes the disagreement between this step and the Files list that a Task-5 implementer met on code contact. `git diff --stat` covers exactly the six files in the Files list.

- [ ] **Step 6: commit.**

```bash
git add src/components/ConfirmDialog.vue src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts e2e/editor-undo-redo.spec.ts
git -c commit.gpgsign=false commit -m "editor: confirm before unsaved changes are replaced, ahead of the file dialog" -- src/components/ConfirmDialog.vue src/views/EditorView.vue locales/en/gui-editor.ftl locales/de/gui-editor.ftl e2e/smoke.spec.ts e2e/editor-undo-redo.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** that the surface is an in-app `<dialog>` rather than a native dialog (and that therefore no capability changes); that the guard sits in `pickAndOpen` and `createBlank` and not in `openPath`; that a cancelled file dialog discards nothing; the three fenced strings and the reuse of `settings-cancel`; that there is no third "save first" button; that the view-switch case is extended rather than duplicated; that no rule-removal confirmation is added anywhere; **and, from amendment 1 as amendment 2 corrects it**: the membership criterion in Step 4b and the three cases it yields; that the repair is a visibility assertion plus a confirm click and never a removed, weakened, reordered or reworded assertion; that the New case takes the identical repair rather than a different one; that no case there is rewritten to avoid the guard instead; that the file is otherwise untouched; and, from amendment 2, that the header-comment correction lands in this task, its fenced replacement text, and that the count is recomputed from the catalog rather than copied from this plan.

---

## Task 6: the shell learns the save state, and one close prompt covers both reasons (W4b)

Read first: this plan's D109 decisions 4 and 5 and **D110 in full**; `src-tauri/src/run.rs`'s `CloseDecision`, `close_decision`, `on_close_requested`, `abort_and_quit`, `ftl_message` and the two dialog-string tests, plus its existing `close_decision` unit cases; `src-tauri/src/lib.rs`'s `AppState`, its `Default` impl and the single `invoke_handler` registration; **`crates/muxsmith-cli/src/i18n.rs`'s `LOCALES` table and `Renderer::new`**, the house pattern D110 conforms to; `locales/en/gui-common.ftl`'s `close-abort-*` block and `locales/de/gui-common.ftl`'s header note about them; **`scripts/check-i18n.mjs`'s `RUST_ONLY_IDS` block and the comment above it**; `src/i18n/fluent.ts` (`currentLocale`, and the two places `applyLocale` is called from); `src/ipc.ts` (the wrapper shape this task follows); `src/App.vue`'s script setup; `src/views/EditorView.vue` as Task 5 left it. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `src-tauri/src/lib.rs` (the two `AppState` fields and their defaults, the `set_editor_dirty` and `set_shell_locale` commands, their handler registrations)
- Modify: `src-tauri/src/run.rs` (the locale table and the locale-aware `ftl_message`, the four-variant `CloseDecision`, `close_decision`, `on_close_requested`'s dialog selection and its confirm actions, the extended dialog-string test, the new per-locale parity test, the new decision-matrix tests)
- Modify: `locales/en/gui-common.ftl` (six new single-line ids)
- Modify: `locales/de/gui-common.ftl` (the same six)
- Modify: `src/ipc.ts` (a `setEditorDirty` and a `setShellLocale` wrapper)
- Modify: `src/views/EditorView.vue` (one watcher on `dirty`)
- Modify: `src/App.vue` (one watcher on `currentLocale`, pushing it to the shell)
- Modify: `scripts/check-i18n.mjs` (**the `RUST_ONLY_IDS` allowlist only** - the enumeration of shell-consumed ids this task's six new keys falsify; no other change to the script)
- Modify: `e2e/smoke.spec.ts` (the wire assertions for both syncs)

**Interfaces:**
- Consumes: Task 4's `dirty`; `currentLocale` from `src/i18n/fluent.ts`.
- Produces: `set_editor_dirty`, `set_shell_locale`, the four-state close decision, and a locale-aware shell lookup.

- [ ] **Step 1: the shell state and the two commands.** `AppState` gains `editor_dirty: AtomicBool` (defaulted false) and `dialog_locale: Mutex<String>` (defaulted `"en"`), each with a doc comment stating that it mirrors a frontend value, that the frontend is its only writer, and what a failed sync costs (a stale flag, or a stale dialog language - never a missing dialog). Add `#[tauri::command] fn set_editor_dirty(dirty: bool, state: State<AppState>)` and `#[tauri::command] fn set_shell_locale(locale: String, state: State<AppState>)`, both registered in the one `invoke_handler` list. **The shell does not resolve a locale of its own and `sys-locale` is NOT added to `src-tauri`** (D110 decision 2): `effectiveLocale` stays the product's single resolution rule.

- [ ] **Step 1b: the locale-aware lookup, split into a row step and a chain step.** Add `const DE_GUI_COMMON: &str = include_str!("../../locales/de/gui-common.ftl");` beside the existing en constant and a `LOCALES: &[(&str, &str)]` table carrying `("en", GUI_COMMON_FTL)` and `("de", DE_GUI_COMMON)`, in the shape `crates/muxsmith-cli/src/i18n.rs` uses and with a doc comment naming that file as the pattern it follows and the reason the table is hand-written. Then two functions, and the split is prescribed rather than left to taste:
  - `fn lookup_in(catalog: &'static str, key: &str) -> Option<&'static str>` carries the line parse - column-0 `key = value`, trimmed, never prefix-matching - and returns `None` when the catalog has no such line. This is the existing `ftl_message` body with its `unwrap_or` removed.
  - `fn ftl_message(key: &'static str, locale: &str) -> &'static str` collapses `locale` to its primary subtag (everything before the first `-`, lowercased), then walks `[requested, en]` over `lookup_in`, then returns `key`.

  **The split is what makes the parity test possible**, and the doc comment on `lookup_in` says so in one clause: an assertion made through the chain is green under every mutation upstream of the en fallback, so the check calls the row step directly (Step 5). The existing single-line, column-0 and never-prefix-match properties are preserved exactly, and the chain function's doc comment keeps its recorded reason for not being a Fluent parser, extended with the second locale. Call sites read the locale from `AppState`.

- [ ] **Step 2: the decision matrix.** `CloseDecision` gains `ConfirmDiscard` and `ConfirmAbortAndDiscard`, each documented. `close_decision` reads both facts and returns exactly the four-row table D109 decision 5 fixes. `on_close_requested` selects the title/message/confirm triple per variant, keeps `close-abort-dismiss` as the cancel label for all three confirming variants, and on confirmation runs `abort_and_quit` for the two run-bearing variants and exactly `app.exit(0)` for `ConfirmDiscard`, no run existing there to abort. **The code is fenced rather than left to the implementer**: the neighbouring site passes one through a closure (`abort_and_quit(&app.state::<AppState>(), |code| app.exit(code))`), so an unwritten literal would be an invented value. The unchanged `Close` path still returns before `prevent_close`.

- [ ] **Step 2b: the re-read on the confirming branch (D109 decision 9).** Add the pure rule beside `close_decision`, factored off the Tauri types for the same recorded reason:

```rust
/// Whether a confirmed close still needs asking about (D109 decision 9).
/// `Some(current)` when the state now carries a fact the dialog the user
/// answered did not state, `None` when nothing was added - the state
/// weakened, or did not move. One re-read only: the caller acts on the
/// answer to the prompt this returns and never reads again.
fn reconfirm_decision(answered: CloseDecision, current: CloseDecision) -> Option<CloseDecision>
```

  Its body compares the two facts each variant stands for - run-abort and discard - and returns `Some(current)` exactly when `current` names one the `answered` variant did not. Then, in the dialog callback's confirming branch and **before** `abort_and_quit` or `app.exit(0)`: re-read `close_decision(&app.state::<AppState>())`, pass it through `reconfirm_decision`, and on `Some(v)` show `v`'s own dialog - the same construction as the first pass, with `v`'s title, message and confirm label - whose callback is **terminal**: confirming performs `v`'s action from decision 5's table, declining returns without arming anything and leaves the window open. On `None`, proceed exactly as today. **No fifth message is added and no catalog key changes**; if the implementer concludes one is needed, that is NEEDS_CONTEXT, because the message set is owner-visible.

- [ ] **Step 3: the catalog, both locales, fenced, single-line by the shell's own parser constraint.** Append to `locales/en/gui-common.ftl` after the `close-abort-*` block exactly:

```
close-discard-title = Unsaved changes
close-discard-message = The profile in the editor has unsaved changes. Quit and lose them?
close-discard-confirm = Discard changes and quit
close-abort-discard-title = Running jobs and unsaved changes
close-abort-discard-message = A job is running and the profile in the editor has unsaved changes. Abort all running jobs, discard the changes and quit?
close-abort-discard-confirm = Abort jobs, discard changes and quit
```

  and to `locales/de/gui-common.ftl` in the same position exactly:

```
close-discard-title = Nicht gespeicherte Änderungen
close-discard-message = Das Profil im Editor hat nicht gespeicherte Änderungen. Beenden und verwerfen?
close-discard-confirm = Änderungen verwerfen und beenden
close-abort-discard-title = Laufende Jobs und nicht gespeicherte Änderungen
close-abort-discard-message = Derzeit läuft ein Job und das Profil im Editor hat nicht gespeicherte Änderungen. Alle laufenden Jobs abbrechen, die Änderungen verwerfen und beenden?
close-abort-discard-confirm = Jobs abbrechen, Änderungen verwerfen und beenden
```

  **Both locales' values are read**, through the locale-aware lookup Step 1b builds, so a German user reads the German text; Step 5's part (c) pins one of them. Two properties are unchanged and bind both files equally: every value stays **single-line and column-0**, because the shell's parser is a line lookup and not a Fluent parser, and no value carries an attribute. The de catalog header's note that the `close-abort-*` strings "are not yet shown to a de user ... for parity and a later shell i18n" is **consumed by this task, not extended**: those four strings become readable by the same change, and the header's forward-looking clause is a close-action disposition rather than a standing limitation.

- [ ] **Step 4: the two frontend syncs.** `src/ipc.ts` gains `setEditorDirty(dirty: boolean)` and `setShellLocale(locale: string)`, both documented beside their siblings. `EditorView` gains `watch(dirty, (value) => { void setEditorDirty(value).catch(() => { /* background bookkeeping */ }); });`. `App.vue` gains `watch(currentLocale, (locale) => { void setShellLocale(locale).catch(() => { /* background bookkeeping */ }); }, { immediate: true });` - **`immediate` is load-bearing and is commented as such**: `main.ts` applies the locale before the app mounts, so without it the shell would hold `"en"` until the user changed the language. Both watchers carry the tolerance comment and its named consequence, mirroring the view's existing tolerance for its recents write. `App.vue` is otherwise untouched.

- [ ] **Step 5: the Rust tests, three groups.**
  - Extend `close_abort_strings_resolve_from_the_ftl_catalog`'s key enumeration with the six new ids (the enumeration is the point of that test, so it is a named region), keeping its pinned reference-wording assertion and passing `"en"` explicitly now that the lookup takes a locale.
  - Add four `close_decision` cases, one per matrix row, each constructing the state it names: idle-and-clean, run-and-clean (the existing case covers this row and is extended rather than duplicated), dirty-and-idle, dirty-and-running. Assert the exact variant per row.
  - **Add the `reconfirm_decision` matrix, exhaustively rather than by example.** Three `answered` variants (the three that produce a dialog) against all four `current` variants, twelve cells, each asserting `Some(v)` or `None` explicitly. **Exhaustive because a pure function over a four-value enum admits it**, and because the two halves of the observable are opposite cells of the same table: the strengthening cells are the "second prompt appears" side and the weakening and unchanged cells are the "it does not appear" side, so a table that skipped either would cover one side of a two-sided consequence. Name the cells that must be `None` in the report as well as the ones that must be `Some`, since the silent-no-prompt cells are the ones a broken rule would pass.
  - **Add the shell parity test (D110 decision 4), derived rather than hand-listed, split three ways so its assertions sit BELOW the `[requested, en]` chain.** (a) Read the `locales/` directory at test time under `env!("CARGO_MANIFEST_DIR")` and assert every locale directory has a row in the shell's `LOCALES` table. (b) Derive the shell's consumed key set from `include_str!("run.rs")` with a regex over `ftl_message("...")` literals, then for **every row** assert `lookup_in(row_catalog, key)` is `Some` and non-empty - **called on the row directly, never through `ftl_message`**. (c) Assert `ftl_message("close-abort-title", "de") == "Laufende Jobs abbrechen"`, the German mirror of the existing test's pinned en wording.
    **Each of the three prescribed red states names the part that must fail AND the parts that must not**, so a mutation cannot be satisfied by the wrong assertion, and where one mutation trips a second assertion as well, that is stated rather than left to inference:
    - Point the `de` row at the en catalog: **(c) fails**; (a) and (b) pass, because the row exists and the en catalog holds every key.
    - Delete **`close-discard-title`** from `locales/de/gui-common.ftl` - **the key is named because the choice matters**: it is one this package adds, so the mutation exercises the new surface, and it is deliberately NOT the key (c) pins, so **(b) fails** while (a) and (c) pass. Deleting the pinned key instead would fail (c) as well and falsify the stated must-not-fail half.
    - Delete the `de` row from the table: **(a) fails, and (c) fails with it** - the chain then finds no requested row and falls through to en, so the pinned German value is not returned. (b) passes, because it iterates only the rows that exist. **Two failures from one mutation is a property of the design, not a defect**, and it is written here so an implementer meeting two does not read the second as a surprise.

    Run all three, paste all three failures, restore, then paste the green run. **A red state that produces no failure is a defect in the test, not in the mutation**, and returns as NEEDS_CONTEXT.
    **Why the lookup is split at all** (Step 1b): an assertion made through the composed chain is green under every mutation upstream of the en fallback, which is what an earlier draft of this plan got wrong. **The derivation must not be replaced by a literal list** - a hand-written list is the blind spot this test exists to remove, and a key added to the shell later would not join it. Two residuals go in the report, not into the test: a non-literal `ftl_message` argument would be invisible (every current call site is a literal), and `crates/muxsmith-cli/src/i18n.rs` has the identical unserved-locale gap, which this task SURFACES and does not fix.

- [ ] **Step 6: the wire tests, both syncs.** In `e2e/smoke.spec.ts`: after an edit, a recorded `set_editor_dirty` call carrying `true`, and after a successful Save one carrying `false` (both halves, because a flag that only ever sets is worse than none); and the shell-locale sync, **both halves asserted against concrete values rather than against "whatever was applied"**: a recorded `set_shell_locale` call at startup whose argument equals **`"en"`** - determined, not open, because `smoke.spec.ts`'s scenarios take `get_settings` from the mock default, which returns `locale: "en"`, so `effectiveLocale("en")` is `"en"` - plus a second call whose argument equals `"de"` after the settings dialog switches the language. The live half is what makes the pair non-vacuous: a shell told once and never again passes the startup half and fails the user.

- [ ] **Step 6b: the allowlist.** Add the six new ids to `RUST_ONLY_IDS` in `scripts/check-i18n.mjs`, beside the four `close-abort-*` keys already there, and extend that block's comment to say the set is shell-consumed rather than naming D31 alone. Nothing else in the script changes. **Verification for this step is the run itself:** `pnpm check:i18n` must report no unused-id warning for any of the six, and the pre-state run before the allowlist edit must report exactly those six - paste both.

- [ ] **Step 7: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green - `cargo test --workspace` covers the three Rust test groups, `pnpm check:i18n` covers cross-locale parity for the six new ids plus the allowlist, and the cross-target clippy part covers the two new commands on Windows without linking. `git diff --stat` covers exactly the nine files in the Files list.

- [ ] **Step 8: commit.**

```bash
git add src-tauri/src/lib.rs src-tauri/src/run.rs locales/en/gui-common.ftl locales/de/gui-common.ftl src/ipc.ts src/views/EditorView.vue src/App.vue scripts/check-i18n.mjs e2e/smoke.spec.ts
git -c commit.gpgsign=false commit -m "shell: closing with unsaved editor changes confirms, in one prompt and in the language the UI is using" -- src-tauri/src/lib.rs src-tauri/src/run.rs locales/en/gui-common.ftl locales/de/gui-common.ftl src/ipc.ts src/views/EditorView.vue src/App.vue scripts/check-i18n.mjs e2e/smoke.spec.ts
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the four-row matrix and that it yields ONE prompt; **that the re-read happens once, on the confirming branch, before the action, and that a strengthening is its only trigger**; that the second prompt is an existing variant's dialog and no fifth message is added; that declining the second prompt returns to the app without arming a quit; that the combined case gets its own message rather than composed prose; that the discard-only confirmation exits rather than aborting anything; the six fenced strings and their single-line form in both locales; **that the shell is told the locale rather than resolving one, and that `sys-locale` is therefore not added**; that the locale table follows the CLI's shape; that the parity test derives its key set from the source rather than listing it; that the `check-i18n.mjs` edit is the allowlist and nothing else; that the CLI's identical unserved-locale gap is surfaced and not fixed; that both frontend syncs are tolerant; that the dialog-string test's enumeration is extended.

---

## Task 7: the user-facing documentation (W5)

Read first: `help/en/view-editor.md` and `help/de/view-editor.md` in full; `locales/en/gui-batch.ftl` and `locales/de/gui-batch.ftl`'s profile block; `scripts/check-i18n.mjs`'s D62 block (the six help conditions this task's prose must satisfy: referenced-to-file, orphans, locale lockstep, the external-URL ban, the pipe ban, the raw-HTML ban); Tier-2 `help-topic-h1-scheme`; the shipped behaviour as Tasks 3 to 6 committed it. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `help/en/view-editor.md`
- Modify: `help/de/view-editor.md`
- Modify: `locales/en/gui-batch.ftl` (the `batch-profile-none` value only)
- Modify: `locales/de/gui-batch.ftl` (the same)

No new help id and no new topic file: the New button carries no `data-help-id`, matching every other button in the editor, so the D62 referenced-to-file check gains nothing to satisfy.

**Interfaces:**
- Consumes: Tasks 3 to 6's shipped surface.

- [ ] **Step 1: the English topic.** Its opening paragraph's entry-set sentence, which today names only open and reopen, gains creation. Add a `## Creating a profile` section after the opening paragraph and before `## Editing the model`, and extend `## Save semantics` with the save-dialog flow. The content is fixed: New starts a profile with one candidate extension and one empty rule, which validation announces as a warning, so it is incomplete rather than wrong; nothing is written until Save, which asks where to put the file and writes there from then on; the editor holds one profile at a time, and replacing it - creating another or opening one - warns first while unsaved changes exist; switching views never touches it; quitting with unsaved changes warns as well; and every edit can be undone, with Undo and Redo in the action row or their keyboard shortcuts. **Naming the undo shortcuts in the topic is deliberate**: it is the one place the app documents them.

- [ ] **Step 2: the German topic**, the same content in the register that file already uses (infinitive imperatives, `Meldungen` for diagnostics, `Stapel` for batch), with the same section structure, because the locale lockstep check compares the file SET and a reader compares the content.

- [ ] **Step 3: `batch-profile-none`, both locales, fenced.** In `locales/en/gui-batch.ftl` replace exactly

```
batch-profile-none = No profile selected yet. Choose one below to validate it and start a batch.
```

  with exactly

```
batch-profile-none = No profile selected yet. Choose one below to validate it and start a batch, or create one in the Editor view.
```

  and in `locales/de/gui-batch.ftl` replace exactly

```
batch-profile-none = Noch kein Profil ausgewählt. Wähle unten eines aus, um es zu prüfen und einen Stapel zu starten.
```

  with exactly

```
batch-profile-none = Noch kein Profil ausgewählt. Wähle unten eines aus, um es zu prüfen und einen Stapel zu starten, oder erstelle eines in der Editor-Ansicht.
```

  Neither value gains a placeable, and the existing batch scenario asserts this string through `en(id)`, so its assertion follows the catalog automatically.

- [ ] **Step 4: verification.** The full gate as `BUILDING.md` enumerates it, foreground, green - `pnpm check:i18n` carries the D62 help gate and the cross-locale parity, `pnpm test:e2e` the help-topic rendering and the batch assertion. **Absence check H1, the content-hygiene half, run on the two edited topics:** `grep -nE 'https?://|\||</?[a-zA-Z]' help/en/view-editor.md help/de/view-editor.md` must return nothing outside inline code spans. **Its fire:** the same expression over `docs/INSTALL.md` returns matches, so the pattern demonstrably hits a URL and a pipe when one is present. `git diff --stat` covers exactly the four files in the Files list.

- [ ] **Step 5: commit.**

```bash
git add help/en/view-editor.md help/de/view-editor.md locales/en/gui-batch.ftl locales/de/gui-batch.ftl
git -c commit.gpgsign=false commit -m "help+batch: the editor creates profiles, guards unsaved ones and undoes edits" -- help/en/view-editor.md help/de/view-editor.md locales/en/gui-batch.ftl locales/de/gui-batch.ftl
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** that no new help id or topic file is created; that the undo shortcuts are named in the topic; the two fenced `batch-profile-none` values; that Batch gains no New control; that no other topic is edited.

---

## Plan close (controller actions, not tasks)

- **Entry condition:** Tasks 1-7 committed, working tree clean, and **the gate as `BUILDING.md` enumerates it green, foreground, no subsets** (the pre-push site). Then the single push (SI-4; `gh-log.md` entry) and the push-triggered CI run green on the head SHA.
- **Whole-branch review** by an independent reviewer on the **top tier** (`proc-03-model-assignment`), against this plan, the plan brief with its five addenda, the ROADMAP entries they cite and the amended spec - before any further close action.
- **Verdict-harvest mining** into the ledger, per the verdict-arrival duty rather than deferred to here. Known inputs already: the editor catalog budget revision 46 -> 54, which updates Tier-2 `editor-generic-action-keys`'s statement the way the 45 -> 46 revision did; the two stale in-tree budget comments, which are a recurrence of the count-drift class rather than a new one; **the owner's translation ruling itself, which is general and therefore belongs in the house record rather than only in this plan's D110** (German translations always ship in the same change, without exception - a Tier-2 candidate on its face, source human, so count 1); the de catalog header's "a later shell i18n" note, now consumed rather than open; **the CLI's `LOCALES` table carrying the identical unserved-locale gap the shell's new parity test closes**, surfaced by Task 6 and deliberately not fixed; the apply-suggestion path in BatchView writing a profile file the editor may hold unsaved; and **amendment 1's own residue** - the owner's failed-load ruling of 2026-07-31, and D112's supersession of two D107 clauses, which is this package's one `proc-supersede-never-overwrite` instance and was recorded as a pointer plus a successor rather than as an edit; **and the standing lint rule D112 adds**, a template-shape guard on an existing gate part, which is a new check class in this house and arrived by refuting a no-work-needed premise rather than by design. **Amendment 2 adds two more:** a normative enumeration of this plan's own, correct when drawn and falsified by a later ruling that added a member the enumeration's criterion could not see - the recurrence `a-normative-claim-is-scoped-down-to-its-producers-reach` names, one level up, and the repair was to rewrite the criterion against the mandate rather than to raise the count; and a Task-4 step requirement half-discharged while two independent reviews graded it MET, which is a review-instrument finding rather than an implementer one, since both graded the step and neither opened the file the step named. **The README's broken first example is deliberately NOT on this list**: its ROADMAP entry, its owner ruling and its vehicle (Plan 11's fix round) all predate this plan, so a second disposition from this close would duplicate a ruled one.
- **Blocked-pool sweep** per the standing duty. `gui-17` (the abort-confirmation-suppression setting parked as v1.x) is the entry this package walks closest to and its condition has not cleared: this plan adds no preference for suppressing any warning.
- **ROADMAP dispositions**, one per item this package closes or moves: the "OWNER QA PASS, round 3" entry's two findings both close, while **the entry itself does NOT close** - the QA gate is a standing precondition on the tag, the owner's pass is resuming rather than finished, and further rounds are expected; the derivation-package paragraphs stay OPEN and gain the two information-duty answers this plan states (the mechanism carries a populated unsaved profile; a single whole-profile assignment is one undo step); the v1.x "Editor undo/redo, all operations" entry closes as built pre-1.0 with its S22 reversal recorded; **the trigger "An accidental-rule-deletion report arrives -> route to the v1.x undo/redo entry" needs consuming**, because this package builds its target; and the corrections table's seven items are recorded against the entries that carried them.
- **SDD salvage** of `.superpowers/sdd/plan-12/` per the standing salvage rule, with its `diff -r` re-check and the salvaged file COUNT verified against the commit rather than asserted.
- **Journal + HANDOFF snapshot** per the standing duty.
- **The completion statement this close is allowed to make, and the one it is not.** This package's completion is NOT 1.0 completeness. `owner-manual-qa-gates-the-1-0-release` binds: no completeness claim about 1.0 may be made until the owner's manual pass has run to its end and produced its findings, however short the ROADMAP's open list looks afterwards. What this close may say is narrower and is the point of the package: **the pass that was STOPPED can resume**, because the precondition it stopped on now exists in a build.

## Self-review (writing-plans skill duty, run at authoring)

**Coverage.** All five work items appear in the work-item coverage map with a named task, all 43 consolidated requirements appear in the requirement table with a named implementer, and all 73 acceptance halves appear in the acceptance map with a named producer. Producer-less observables: zero. Non-machine-verifiable rows: one, W4-w, whose producer is a reviewer check because the close callback needs the Tauri runtime and the pure-rule matrix would otherwise stand in for the wiring it cannot see; plus one qualified row (W5-a claims the D62 gate for existence and hygiene and names the reviewer's reading for the prose, rather than claiming a machine check for register). **Halves, walked deliberately rather than by row count:** W1 takes ten rows - seven for the first-run state and the three transitions, each split into its rendered and its persisted side, plus the unchanged existing assertion, the single-rule absence check and the catalog gate - because a locale change is exactly the two-sided consequence the halves rule was written for; W2 into wire and rendered per step, with the save flow split across dialog, write, display and recents because the four fail independently, plus amendment 1's four - the failed-load state's kept surfaces, its hidden ones, the pre-session state where the same two must still appear, and the single-definition check - because a producer that asserted only the hidden side would pass against code that hid them everywhere; W3 into one row per enumerated mutation path, which is the safeguard's own shape; W4 into the frontend guard's ordering, its two answers, its absence case, the invariant assertion, the shell's four-state matrix as four rows because a single row naming "the close decision" would satisfy the map while covering one state, and the localization into six more - the locale reaching the shell at startup and again on a live switch as two rows, since a shell that is told once and never again passes the first and fails the user, and the German-rendering half as its own row because it is the only one an English fallback cannot satisfy; and the ruled re-read into three, one per side of its observable plus one for the wiring, because the pure rule's matrix proves what the function decides and never that the callback calls it - the split exists so that the side which cannot fail is not the whole row.

**Latitude.** All nineteen decisions the brief enumerates are settled in the Decision register - eighteen of them one-to-one in the requirement table, and its decision 14 spread across R21 to R25 because the owner settled it himself in four separate rulings - as is every decision the five addenda added, and each task carries a **Must not decide** list naming what it may not reopen. Swept for the omission form specifically: every catalog value is fenced byte-for-byte in both locales; every `data-testid` is written out; the seed is fenced and its measurement prescribed; the keyboard condition set enumerates its input types rather than saying "a text field"; the mutation set is the six functions an expression returned, with a seventh declared a finding; the close matrix is a four-row table rather than a precedence sentence; the spec replacements are fenced on both sides; the ADR content is supplied rather than described. Searched for placeholders (TBD, TODO, "appropriate", "similar to", "and so on", "etc"): none. **The one place an implementer must produce prose this plan does not fence is Task 7's help-topic content**, and it is bounded by a closed fact list plus a named register, which is the same treatment plan 10 gave its README pass.

**Counts, recomputed from their own enumerations at authoring rather than recalled, and again at amendment 1 for the three the amendment moves.** 7 tasks (counted from the task headings), 5 work items, 43 requirements (counted from the requirement table's rows, highest `R43`), 73 acceptance halves (counted from the acceptance map's rows: 10 + 16 + 22 + 23 + 2, the per-work-item split re-counted rather than derived from the total), 7 brief/ROADMAP corrections, 6 decision records in the decisions file (D106-D110 from Task 1, D112 from amendment 1), 6 mutation paths, 5 candidate seeds measured, 15 new catalog ids across 3 catalogs (8 editor, 1 settings, 6 common), 30 new catalog lines across both locales, 4 close-decision states, 100 history entries. The editor catalog budget's arithmetic is stated three times as a running total (49 after Task 3, 51 after Task 4, 54 after Task 5) and each task recomputes it from the file rather than trusting the plan, because the number is exactly the kind that goes stale between tasks.

**The gate's part count, audited - and the audit found one, in this document, and it was removed.** Expression, its alternation derived by reading what this plan actually writes about the gate rather than from recall of the forms a gate count can take: `grep -nE '[0-9]+ parts|[0-9]+-part|(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)[ -]part'`. **Fired per alternative, each alone, because a compound control proves only that one member matches:** `11 parts` -> 1, `an eleven-part gate` -> 1, `a 6-part block` -> 1; negative control `the gate runs in order` -> 0. Run over this file as first written, it returned **two** lines: the no-worktree ruling, which said "the eleven-part gate", and **this sentence**, which matches because it quotes its own search expression. The ruling was rewritten to name `BUILDING.md` and no number. Every later re-run has found the same thing: a fix round adds prose about parts, the expression matches it, and the sentence is reworded. It returned **two** after the owner's translation ruling, **four** after the review's first fix round, **two** after the second, and **two** again after amendment 1's fix round, each time with every new match a false positive created by the fixes themselves. The last one is the clearest instance of the prediction this paragraph makes: amendment 1's fix round wrote "in this plan's three parts" about an absence check's falsifiability, the expression matched it on the very next run, and the sentence was reworded to name the three things instead of counting them. Every one was removed by rewording the matching sentence rather than by narrowing the pattern, because a pattern narrowed to dodge a false positive is a pattern that can miss a real hit; the noise is the price of an expression that cannot be talked out of a match. The current run returns **one** line, this sentence. That last one is not padding: a document auditing itself is inside its own search space, and an audit that forgets the auditor is the same defect one turn later. **The first hit is the whole argument for running this audit rather than asserting it** - the ban was in this plan's own Global Constraints, the sentence violating it was written anyway, and only the expression found it. Every remaining gate reference names `BUILDING.md` and states no number.

**Absence-shaped checks, enumerated rather than tallied, each with its fire and its reachable green state.** L1 (the single resolution rule) fires on its pre-state run - **two lines, one per branch of `resolveLocale`, the figure being stated once at Task 2 Step 6 and taken from there rather than restated here** - and carries a soundness control that must return the surviving occurrences. D1 (no second save-state mechanism) has a pre-state zero with the mkvtoolnix comparison as its control and a distinct end-state expression fired against a synthetic assignment. S1 (the saved position is never marked from the live history) is fired against a synthetic assignment too, and its pre-state is empty by construction, so that fire is the only thing making its zero mean anything. E1 (no Diagnostics heading pre-session) and E2 (a cancelled dialog writes nothing) each fire inside their own test on the neighbouring state. U1 (the text-entry exemption) is run twice in one test, once where it must pass through and once where it must fire, because a single run cannot distinguish an exemption that works from one that swallows everything. G1 (the file dialog waits for the confirmation) fires on the same counter after confirming; G2 (no confirm when clean) fires on case 1's visible dialog. H1 (help-topic hygiene) fires against a document that carries a URL and a pipe. **Amendment 1 adds two.** P1 (no render gate spelled `v-if="!model`) has a pre-state measured on the tree as Task 3 left it, exactly two lines, and that non-zero run is its own fire; P2 (a failed load hides both pre-session surfaces) fires in the first leg of its own test, where the same two locators must resolve, and carries two positive assertions in the leg that must be zero so the state under test is identifiable rather than merely empty. **P1 is the one-shot demonstration and not the property's guard**: the standing guard is Step 4c's lint rule, which runs inside `pnpm lint` on every gate run, reads the parsed template rather than the file's characters, and carries its own red state (the two shipped gates), its own green state (the end state with the `:disabled` bindings present, the over-match control) and a per-member fire of its selector's enumerated directive set. Task 1's sweep expressions carry a stated must-return rather than a zero, because a sweep that returns nothing is malformed. Task 4's mutation enumeration carries its own second expression aimed at the first one's blind spot, and that second expression was fired against a synthetic positive at authoring. **Every one of these fires is prescribed for the implementer against a deliverable that does not exist yet; the figures in the Authoring-time verification section are the ones a reviewer can reproduce today.** The two are kept visibly apart on purpose.

**What round 1 of the plan review changed, recorded because two of its findings were defects in this document's own instruments rather than in the plan's design.** Seven majors, six minors and two nits, all fixed: the shell parity test's two red states were both green because the assertion ran through the very fallback the mutation was upstream of, which is the third instance of that class in this project and is now answered by a handle in the Global Constraints rather than by a patched red state; `savedSnapshot` was never written on a successful save, so every discard guard would have stayed armed after the first save while the only producer for that half passed regardless; the failed-open branch of the history was undecided and Undo was reachable with no model; the shell's German-rendering half had no producer at all; L1's two figures were both off by one; the string-surface expression could not produce the row the plan attributed to it, because the frontend's loader wraps its argument to the next line, so the set is now the union of two differently-shaped expressions with the blind spot stated; a 430-byte figure came from a truncated fixture; two locale-control assertions were counted as one; and a ruled disposition was about to be duplicated. **Not changed, because the review found them sound:** the shell locale route and its rejected alternatives, the nineteen brief decisions and their implementers, the seed measurement, the task cut, the sequencing and the close.

**What round 2 changed, and its centre was a defect this plan's own round-1 repair introduced.** Thirteen of the fifteen round-1 findings were graded addressed, two addressed-with-concern, none unaddressed; the reviewer withdrew half of its own F4 prescription after judging the Task-5 placement of the save-state check better than what it had asked for. The blocker it found was **Step 1c marking the live history position instead of the profile `doSave` captured and wrote** - F4 with the sign flipped, and the worse sign: two awaits sit between capture and mark, the editing surface stays live across both, and a model change inside either window would have marked the editor clean over content the file does not hold, disarming every guard. That is the data-loss direction where D108 decision 4 claims annoyance, and the borrowed precedent that appeared to license it (`Tab::onSaveConfig`) is licensed there only by being fully synchronous. **The general rule that fell out of it is now a Global Constraint** - a borrowed precedent carries the conditions that made it correct, synchrony being the one most easily dropped - and this plan ran that sweep over itself, finding one more precedent whose condition is genuinely gone (the Save-as fallback, already compensated by the capture) and one whose guarantee is modality rather than synchrony (the discard confirms). Six minors followed: two under-specified red states, the restated-count class recurring in the very paragraph that enumerates the absence checks, a `de()` permission wider than its safe set, one half of an acceptance row whose step had not caught up, and an ordering requirement that had ridden a sentence this plan replaced.

**Refutations.** Seven, in the corrections table with pasted evidence: `currentPath` does not gate editing; the existing locale assertion is blind rather than wrong; the recommended seed is necessary but insufficient; the dialog plugin's `confirm` needs `dialog:allow-message` rather than `dialog:allow-confirm`; the editor catalog budget is 46 in the ledger and 45 in two live comments; the README's own copy-pasteable example profile does not load; and **this plan's own first draft was wrong about the gate's blind spot** - the parity check does cover the shell's catalog, so a German shell string's existence was already gated and the German values were shipping dead, which means a safeguard aimed at key existence would have been green before and after. Two of the seven were controller premises, one was an assumption this plan would otherwise have relied on, one is a defect in a document nobody was looking at, and one is a correction of this document.

**The string surfaces, enumerated from the tree rather than from recall** (R40, `a-search-whose-terms-come-from-memory-produces-a-false-absence`): the expression and its full output are in the authoring section, with a fired control. Five surfaces exist - frontend catalogs, help topics, CLI catalogs, the shell, and `tauri.conf.json`'s non-localized chrome - and this package adds strings to exactly three. Walked per surface: **frontend catalogs** gain 9 ids and 2 reworded values, every one of them written in both locales in the task that renders it (Tasks 2, 3, 4, 5, 7); **the shell** gains 6 ids in both locales in Task 6, which is also the task that makes the German half readable at all; **help topics** gain their new sections in both locales in Task 7. **CLI catalogs and `tauri.conf.json` gain nothing**, which is a claim about this package's own diff and is checkable against the seven Files lists: no task lists `locales/*/cli.ftl` or `src-tauri/tauri.conf.json`. Nothing is deferred to a later locale pass, and no string in this package ships in one language (R39).

**Safeguards proposed here that later rounds may not argue away** (`proc-proposed-safeguard-stays`): the per-mutation-path coverage table; the second expression aimed at the first's blind spot; the fires on all eleven absence checks (L1, D1, S1, E1, E2, U1, G1, G2, H1, P1, P2); the U1 double run; **amendment 1's leg 1 and leg 2**, which keep the pre-session and the shipped-`!model` behaviours asserted beside the state D112 changes, so a gate that hid the two surfaces everywhere could not pass; **amendment 1's standing lint rule (Step 4c) with its red state, its over-match control and its per-member selector fire**, which is not to be simplified into P1 during a fix round - the grep is the demonstration, the rule is the guard, and they fail for different reasons; **the confirm-visibility assertion Step 4b adds to each of the three repaired Task-4 cases**, which is what keeps those cases graded rather than merely passing, **and Step 4b's membership criterion itself**, which is what lets a reviewer re-derive that set instead of trusting it - the earlier count was falsified by one later ruling, and a criterion scoped to the guarded functions is what stops the next one from doing it again; the four-row close matrix as four unit cases rather than one; **the shell's source-derived per-locale parity test split three ways, each part with the red state it defeats named, and its derivation-not-a-list requirement**; **the split of the shell lookup into a row step and a chain step, which exists so that test can assert below the fallback**; **the model gate on undo and redo, which decision 9 makes currently unreachable and which stays for exactly that reason**; **absence check S1, which pins the one structural property whose violation this plan already committed once**; **the pre-and-post `check:i18n` pair on the allowlist edit**; the after-a-save leg of Task 5's case 3, which is the only check in the package that fails if the saved position is never marked; the seed re-measurement in Task 3 Step 1; and the recents-unreachability assertion that converts a piece of reasoning into a check. Each is removable only after it is built and measured redundant. The parity test in particular is not to be simplified into a hand-written key list during a fix round: the hand-written list is the blind spot it exists to remove, and a list that is correct today goes stale on the next shell string without anyone noticing.
