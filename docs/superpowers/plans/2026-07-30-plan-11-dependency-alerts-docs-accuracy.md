# Plan 11: the two dependency alerts and four routed documentation defects

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **House deviation from the skill text:** progress NEVER enters this document. No box in this file is ever ticked; the checkbox syntax is structure, not a tracking surface. The tracker is `.superpowers/sdd/plan-11/progress.md`.
>
> **Execution starts only on the owner's plan approval** (standing gate; same rule the Plan-8.5, Plan-9 and Plan-10 headers carry).
>
> **There is no design document for this package, deliberately and with the controller's recorded decision.** Every fork here is wording-level or mechanism-level and decidable against the tree; no interface, wire format or architecture is at stake. The coverage ground truth for this plan is therefore `.superpowers/sdd/plan-11/plan-brief.md` together with the ROADMAP entries it cites, not a design file. A later reader looking for `docs/superpowers/specs/...plan11...` will not find one, and none is missing.

**Goal:** land two independent bodies of pre-1.0 residue. **Stream B** owns the two open dependency-vulnerability alerts as ONE task with three parts (bump `postcss` through the lockfile; MEASURE why `cargo deny check` is green while GitHub reports a Rust advisory; establish whether `glib` can move independently of Tauri's tree). **Stream A** owns four documentation defects that earlier plans each correctly left un-fixed because they sat outside every task's Files list: `BUILDING.md`'s three positional gate ordinals plus its one over-80 prose line; the line-number citations that survive outside the source-file selector Plan 10's sweep used; the `raw:` comparison wording that overstates itself for numeric scalars; and the v1 spec's section 8.1 CLI synopsis, which underclaims the shipped flag surface.

**Where this package sits, and what it is NOT.** This is **not** a 1.0 completeness statement and no sentence in it may be read as one. Tier-2 `owner-manual-qa-gates-the-1-0-release` is unsatisfied: the owner's manual product pass runs in parallel with this plan, its round 3 stopped after two findings, and its output is first-class scope input in three shapes he named (real bugs; behaviour he dislikes even where it matches the spec; v1.x items he decides belong in 1.0 after all). 1.0 scope is therefore unknown by construction while this plan runs. **No task in this plan bumps a version, prepares or creates a tag, or edits a release body**, and the plan close does not propose the tag.

**Architecture:** two streams, each in its own git worktree, merged sequentially into `master` with a **full gate run on each merged state**.

- **Stream A - documentation accuracy.** Four strictly serial tasks (A1 -> A2 -> A3 -> A4) in one worktree. They are documentation and configuration edits; the doctrine's parallelism boundary (`proc-08-parallel-worktrees`: a stream earns its overhead only when its own work exceeds one gate run plus one merge) puts them in one tree rather than one tree each, and two of them amend the same file.
- **Stream B - the dependency alerts.** One task (B1) with three parts, per the owner's ruling of 2026-07-29 recorded in the ROADMAP's pre-1.0 gates section. It clears the parallelism boundary on its own - it installs npm dependencies, compiles the Rust workspace and runs suites - and it is streamed separately for a second reason: **a transitive lockfile bump can turn the four frontend gate parts red for reasons that have nothing to do with stream A**, and a shared tree would block stream A behind that.

The stream split is fixed by the controller brief and is not reopened here. The task cut WITHIN each stream is this plan's and is argued in the sequencing section.

**Owner-ruling note the cut respects.** The dependency work was ruled "its OWN one-task vehicle, not a Plan-10 rider", with the recorded reason that Plan 10 was a contract already under execution and a package reopened for every incoming finding stops being a contract. This plan carries it from the start as its own stream, which satisfies that reason. It does not become a rider on the documentation work: no stream-A task touches `pnpm-lock.yaml`, `Cargo.lock`, `package.json` or `deny.toml`, and B1 touches no file any stream-A task lists.

**Tech Stack:** Rust workspace (toolchain pinned via `rust-toolchain.toml`, currently 1.96.1), Tauri 2 / Vue 3 / TypeScript frontend, pnpm 11.10.0 and Node 26.5.0 pinned via `mise.toml`, Playwright e2e with the in-repo mock+mount harness (`e2e/`), Fluent catalogs under `locales/`, help topics under `help/`, `scripts/ledger-lint.py` (Python 3 + PyYAML) as the house docs-invariant checker, `cargo-deny` 0.19.9 as the advisory/licence gate part. **No new dependency of any kind, cargo or npm, and no new gate part, CI job or runtime dependency.** B1's `postcss` move is a lockfile resolution change, not a dependency addition.

## Global Constraints

- **Ground truth and precedence:** the v1 spec (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`) is authoritative on conflict (`proc-04-spec-wins`). Below it: the plan brief (`.superpowers/sdd/plan-11/plan-brief.md`); `docs/ROADMAP.md` for the "TWO OPEN VULNERABILITY ALERTS" entry in the Pre-1.0 release gates section including its RULED block, the three "Docs accuracy" entries this plan closes, and the "A neighbouring class" paragraph in the "Gate-count derivation has no check" section; `BUILDING.md` for the gate, verbatim, and as Task A1's own subject. The four house-knowledge files (`docs/product-boundaries.yaml`, `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/decision-ledger.yaml`) are review ground truth alongside the spec; cite entries by id. The ones that bind hardest: `comments-locate-by-symbol-never-by-line-number`, `a-document-never-cites-a-line-number-inside-itself`, `code-comment-line-citations-drift`, `a-search-whose-terms-come-from-memory-produces-a-false-absence`, `proc-sweep-surface-completeness`, `testing-si3-run-binary`, `proc-06-mkvtoolnix-parity`, `tests-ship-with-the-feature-never-after`, `ledger-lint-runs-before-every-push`, `owner-manual-qa-gates-the-1-0-release`.
- **The gate as `BUILDING.md` enumerates it**, run foreground, no subsets, **before any push**. This constraint deliberately names the FILE and states no count: Task A1 edits that file, and a plan that hardcoded a number here would fork the contract it executes against. Per-task verification below names the **exit-bar subset** each task must run green before committing; that subset is a task exit bar, **not** a gate substitute. The mandatory full runs are named in the sequencing section (one per worktree before its merge, one per merged state).
- **No design decision is re-opened, softened, or "improved".** A contradiction discovered on code contact is refuted with evidence or returned, never silently absorbed.
- **Every fork in this plan is closed** (`proc-latitude-clause-boundary`). No task brief, verdict or fix-round dispatch may carry a design-latitude clause in either form: an explicit permission, or an omission - an unenumerated set in a normative position, a list ending open, a "one per X" with no X list, a step that requires inventing a name, a string, a wording or a file that is not written down somewhere the implementer can read. A fork discovered on code contact returns as **NEEDS_CONTEXT with a decision memo** (options, costs against the named invariants, a recommendation) and is routed by the controller, never resolved at the keyboard.
- **No task edits any house-knowledge YAML** (`docs/decision-ledger.yaml`, `docs/conventions.yaml`, `docs/process-conventions.yaml`, `docs/product-boundaries.yaml`). The controller is their single writer. **No task edits `docs/ROADMAP.md` or `docs/process-journal.md` either**; ROADMAP dispositions are close actions. A task that finds something ledger-worthy or tracker-worthy SURFACES it in its report; the "Surfaced for the controller" list in the plan close carries the ones already known at authoring.
- **No task edits a retired plan document or an archived process artifact.** A retired plan and a superseded design document are history: their gate counts, their line spans and their wordings were true when written, and rewriting them to today's tree would falsify their own record. This is the same principle the ROADMAP's MEASURED block establishes for stale gate counts, and it is why the four historical `byte-literal` sites named in Task A3's OUT list are not swept.
- **No task creates a tag, bumps a version, publishes or edits a release, resolves a README `placeholder(1.0)` comment, or removes the README work-in-progress banner.**
- **SI-3, the mkvtoolnix parity duty** (`testing-si3-run-binary`, `proc-06-mkvtoolnix-parity`): a behavioural question is settled against the real mkvtoolnix source at `~/Downloads/mkvtoolnix` and by running the installed binary, never from memory. Task A3 is where this bites, and it is not waived on the grounds that only wording changes: the wording asserts a comparison behaviour, and what mkvmerge actually reports decides whether the assertion is true. The authoring section discharges it once; A3 re-runs the two commands named there.
- **A comment never locates code by line number** (`comments-locate-by-symbol-never-by-line-number`, owner-ruled, **widened by owner ruling of 2026-07-29 session 28 to reach CI and configuration comments**) and **a document never cites a line number inside itself** (`a-document-never-cites-a-line-number-inside-itself`, owner-ruled). Tasks A1 and A2 are in that family, so **this plan must not violate what it repairs**: every citation in this document points at ANOTHER file at the named commit `2c04ac4`, which the first entry's scope boundary permits for a process artifact, and no citation in this document points into this document - self-references name the container (a task, a step, a table row), never a number.
- **Two writers in one working tree share one git index.** A task that commits while another writer is live uses pathspec-scoped commits (`git commit -- <paths>`; `concurrent-writers-need-pathspec-scoped-commits`). Worktrees isolate by construction, so within a stream this is belt-and-braces; it is mandatory for anything the controller runs on `master` while a stream is live.
- **Tests belong to the feature package** (`tests-ship-with-the-feature-never-after`). Weighed per task, in each task's verification step, with the reason stated. This is not a blanket exemption and no task defers a scenario the existing test infrastructure can already express.
- **Verification steps whose expected result is an absence are fire-verified AND have a reachable green state** (`proc-verification-step-must-be-falsifiable`, `proc-check-green-state-reachable`): the check is run on the PRE-state where it must hit, with an exact expected non-zero count, and then on the intended end state, with its expected zero. Every absence-shaped acceptance item below carries both halves. An item that carried only the end state would be incomplete.
- **A proposed safeguard stays** (`proc-proposed-safeguard-stays`): a guard, test, enumeration or check this plan proposes is removed only after it is built and MEASURED redundant, never argued away during authoring or review.
- **Counts are recomputed from their enumerations** (`proc-normative-count-recomputed`): every count in this plan was recomputed from its own list at plan-authoring on 2026-07-30; a task that changes a set re-recounts and updates the consuming line in the same change.
- **Evidence lines carry pasted output** (`design-empirical-claims-reproducible`): every observed value in a task report is pasted from the run that produced it, never recalled, and never attributed to a command that was not the one run.
- **A measuring expression's own enumerations are claims** (`a-search-whose-terms-come-from-memory-produces-a-false-absence`): both the file selector and the pattern alternation are derived from the artifact, not from recall, and each is fired against a known-present case before an empty result is reported. This plan's own first pass at Task A3's corpus used the alternation `byte[- ]?(exact|literal|identical|wise)` and could not match `byte-for-byte` or `byte-genaue`; the corrected expression is in the authoring section and the defect is correction 5.
- **Typography:** ASCII hyphens, straight quotes, no Unicode ellipsis, in this plan, in every code comment, and in every string it prescribes. German orthography in Fluent catalogs and `help/de` topics is orthography, not an AI-tell glyph, and is copied exactly where it appears.
- **SI-4 (restate in every dispatch that expects a commit; `dispatch-restates-the-standing-commit-grant`):** commits and pushes on this repo are standing-authorized by the owner; agent commits are deliberately unsigned - `git -c commit.gpgsign=false commit ...` - with exactly one trailer, `Co-Authored-By: Claude <model> <noreply@anthropic.com>`, where `<model>` is the canonical model name **derived from that dispatch's explicit model parameter, never written as a literal in this plan or a task brief** (`agent-commit-trailer-set`; no `Claude-Session` line, no context-window suffix). Stage files explicitly, **never `git add -A`**.
- **Implementer preamble, verbatim in every dispatch:** subagents never call session-relocation tools (EnterWorktree/ExitWorktree or any equivalent); absolute paths; foreground runs only (`proc-noninteractive-file-ops-in-agents`); work in the worktree the stream names, never on `master`.
- **How each task's `Read first` list is derived**, the same three-question test Plan 10 established, applied here rather than restated in prose: (1) does a step require the file to be OPENED to derive something this plan does not already state - an anchor, a symbol, a current string, a contract? If not, it is not a read-input. (2) If yes, can this plan NAME the file in advance, or is it a member of a set a prescribed MEASUREMENT computes at execution time? Nameable -> `Read first`; measurement-computed -> not, because the measuring step IS its definition. (3) Is it a universal input already bound by these Global Constraints? `BUILDING.md` is named once, by the gate clause; a task that needs it for its own reason names it anyway, as A1 does, where the file is the subject being rewritten.

## Execution method (binding)

Subagent-driven development (`superpowers:subagent-driven-development`, `proc-01-sdd`): a fresh implementer subagent per task, an independent reviewer per task grading against this plan, the plan brief, the ROADMAP entries it cites and the spec, and a whole-branch review at the plan close before the close actions. Progress lives in `.superpowers/sdd/plan-11/progress.md`. **The skill is LOADED at the pre-execution gate, never reproduced from memory.** A deviation from this method (inline instead of SDD, serial instead of the two planned streams, skipping a review stage) is raised and approved BEFORE task 1, never decided implementation-side.

## Model tiers (`proc-03-model-assignment`)

Every task implementer and every task reviewer runs the **mid** tier; the whole-branch review at the plan close runs the **top** tier, which is the ONLY role the top tier serves; the controller loop runs mid. The controller sets the model parameter explicitly at every dispatch - an omitted parameter inherits the session default, which is not an assignment. **No task in this plan qualifies for the cheap tier:** the cheap tier is reserved for work this plan carries verbatim. Task A1 comes closest, since every replacement string is fenced here character for character, but its four fires and its re-measurement of the ordinal set are judgment, not transcription.

| task | tier | ground |
|---|---|---|
| A1 (W5: `BUILDING.md` ordinals + reflow) | mid | fenced replacements, but four fires against a gate part and a re-measurement that decides whether the fence still applies |
| A2 (W2: the two surviving line citations) | mid | two sites in two different comment syntaxes, each needing the cited target opened to name the right symbol |
| A3 (W3: the `raw:` comparison wording) | mid | six sites across five files in two languages, an SI-3 re-run, and a seven-member retained set that must be proven unchanged |
| A4 (W4: the spec's 8.1 CLI synopsis) | mid | prose against a surface re-derived from the shipped binary, plus a spec self-contradiction sweep |
| B1 (W1: the two dependency alerts) | mid | a lockfile move whose landing version is execution-time-dependent, plus a measurement whose mechanism must be established at the tool's own source |

## Authoring-time verification (2026-07-30, at `2c04ac4`)

**Every value in this section is a CLAIMED measurement: it was run at plan-authoring against the tree at `master` head `2c04ac4` and a reviewer can reproduce it now.** It is deliberately kept apart from the fires and checks the tasks PRESCRIBE, which run later against deliverables that do not yet exist and which no reviewer can reproduce today - those live in the task steps and in the acceptance map's "prescribed" column. Working-tree state at authoring: `git status --porcelain` printed one line, `` M docs/ROADMAP.md``, a co-writer's in-flight change; nothing in this section reads that file's working copy.

### The two alerts, re-verified at the source

`gh api repos/senolfeldmann/Muxsmith/dependabot/alerts`, open alerts only, pasted fields:

- `{"ghsa":"GHSA-r28c-9q8g-f849","manifest":"pnpm-lock.yaml","patched":"8.5.18","pkg":"postcss","sev":"high","vuln_range":"<= 8.5.17"}`
- `{"ghsa":"GHSA-wrw7-89jp-8q8g","manifest":"Cargo.lock","patched":"0.20.0","pkg":"glib","sev":"medium","vuln_range":">= 0.15.0, < 0.20.0"}`

So the `postcss` requirement is **`>= 8.5.18`**, which is what "past 8.5.17" means, and the brief's figure is confirmed rather than approximated.

### `postcss`: the lockfile move is available and its mechanism is measured

- `grep -nE '^ *postcss@|postcss: ' pnpm-lock.yaml` returns four lines, all `8.5.16`; `pnpm why postcss` ends with `Found 1 version of postcss`. Its two parents are `@vue/compiler-sfc@3.5.39` and `vite@8.1.4`.
- The parents' declared ranges, from the registry: `npm view @vue/compiler-sfc@3.5.39 dependencies.postcss` -> `^8.5.15`; `npm view vite@8.1.4 dependencies.postcss` -> `^8.5.16`. Both are caret ranges over `8.5.x`, so **no transitive parent constrains `postcss` below the patched version** and a lockfile-level update can move it. `npm view postcss dist-tags` -> `{ latest: '8.5.25' }`.
- **The mechanism was verified in a scratch copy, not assumed.** `package.json` and `pnpm-lock.yaml` were copied to a scratch directory and `pnpm update postcss --ignore-scripts` was run there under pnpm 11.10.0 (`pnpm update --help`: `--depth <number> ... Infinity is default`, which is what reaches a transitive dependency). Result: `postcss@8.5.16` -> `postcss@8.5.24` at all four lockfile sites, `package.json` byte-identical to the repo's (`diff` printed nothing), and the lockfile diff is **eight hunks covering exactly two packages: `postcss` and `nanoid` (3.3.15 -> 3.3.16, postcss's own dependency)**. Nothing else moved.
- Two consequences the task steps carry rather than assume. **The landing version is execution-time-dependent** (the scratch run landed on `8.5.24` while the registry's `latest` reads `8.5.25`), so this plan states the REQUIREMENT `>= 8.5.18` and the task pastes what it observes; a fenced version number would be a fresh wrong number. And **the diff is not `postcss`-only**, so a check demanding that would fail on correct work.

### `cargo deny`: the disagreement is not a database gap

- `deny.toml` carries **18** `RUSTSEC-` ignore entries (`grep -cE '^\s*"RUSTSEC-' deny.toml` -> `18`), and none of them is a glib id. The brief's figure of 18 is confirmed.
- `cargo deny --version` -> `cargo-deny 0.19.9`. `cargo deny check advisories` -> `advisories ok`; `cargo deny check advisories --show-stats` -> `advisories ok: 0 errors, 0 warnings, 36 notes`.
- **RustSec carries the glib advisory, and it is the SAME advisory GitHub reports.** `~/.cargo/advisory-dbs/advisory-db-*/crates/glib/RUSTSEC-2024-0429.md` exists and its front matter reads `informational = "unsound"`, `aliases = ["GHSA-wrw7-89jp-8q8g"]`, `patched = [">=0.20.0"]`, affected functions `glib::VariantStrIter::{next,nth,last,next_back,nth_back}` over `>=0.15.0,<0.20.0`. The alias is byte-identical to the GHSA id in the alert above, so **the two mechanisms are looking at one advisory and the gap is not database coverage.**
- **`RUSTSEC-2024-0429` is not merely un-failed, it is not surfaced at any severity.** `cargo deny -L info check advisories` mentions `RUSTSEC` on 54 lines and mentions `RUSTSEC-2024-0429` on **zero**, while the 18 ignored ids each produce a `note[advisory-ignored]`. So the advisory is neither an error, nor a warning, nor an ignored note. This **refines the hypothesis on record**: it is not that the configuration does not FAIL on the unsound class, it is that this cargo-deny version does not evaluate that class here at all. What is NOT measured, and is B1's to establish at cargo-deny's own source or current documentation, is which configuration keys 0.19.9 accepts for informational advisory classes and whether any of them could surface this one.

### `glib`: twelve parents, all one generation

`cargo tree -i glib@0.18.5 -e normal --depth 1` lists glib 0.18.5 and **twelve** direct parents: `atk 0.18.2`, `cairo-rs 0.18.5`, `gdk 0.18.2`, `gdk-pixbuf 0.18.5`, `gdkx11 0.18.2`, `gio 0.18.4`, `gtk 0.18.2`, `javascriptcore-rs 1.1.2`, `pango 0.18.3`, `soup3 0.5.0`, `webkit2gtk 2.0.2`. `grep -c '^name = "glib"' Cargo.lock` -> `1`, so there is one glib in the tree. Every gtk-rs crate in the lock is at `0.18.x` (`atk 0.18.2`, `cairo-rs 0.18.5`, `gdk 0.18.2`, `gdk-pixbuf 0.18.5`, `gio 0.18.4`, `gtk 0.18.2`, `pango 0.18.3`); nothing `0.20+` exists. The full reverse tree bottoms out at `tauri 2.11.5` through `muda`, `tao`, `tauri-runtime`, `tauri-runtime-wry`, `webkit2gtk` and `wry`. This is the GTK3 generation `deny.toml`'s own comment records as archived upstream in favour of gtk4-rs, which Tauri 2's tao/wry have not migrated off.

### Item 2's corpus: TWO surviving members, not one

Both expressions were run over **all tracked files outside `docs/`**, which is a wider search surface than Plan 10's six-extension source selector, because the owner's session-28 widening puts CI and configuration comments in scope. The cited-extension alternation was derived from the tree rather than recalled: `git ls-files | sed -n 's/.*\.\([A-Za-z0-9_]*\)$/\1/p' | sort -u | paste -sd'|'` yields `css|diff|ftl|gitattributes|gitignore|html|icns|ico|js|json|jsonc|lock|log|md|mjs|npmrc|png|py|rs|sh|snap|srt|toml|ts|txt|vue|wav|wxl|yaml|yml`.

- **Expression A, filename citations.** `git ls-files | grep -v '^docs/' | xargs grep -nE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+"` with `$EXT` the alternation above. **One line:** `.github/workflows/ci.yml:90:      # correctness, so a broken intra-doc link (queue.rs:73, linking a`. **Fired control:** the same alternation over `docs/*.yaml` returns 11, 20, 10 and 1 matches in `conventions.yaml`, `decision-ledger.yaml`, `process-conventions.yaml` and `product-boundaries.yaml`, so the pattern demonstrably matches a filename-plus-line citation when one is present.
- **Expression B, bare line spans**, run per file so the `file:line:` prefix cannot pollute the match, and written out in full because the authoring section's promise is that a reviewer can re-run it today: `git ls-files | grep -v '^docs/' | grep -vE '\.(png|ico|icns|wav|snap|lock)$' | while read -r f; do grep -nE '(^|[[:space:]`,(])[:][0-9]+' "$f" | sed "s|^|$f:|"; done`. **One line:** `crates/muxsmith-core/tests/fixtures/all-non-default.yaml:2:# :1517-1535) set to a value that is NOT its default. A predicate that`. **Fired control:** the same expression against `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md` returns matches, so it sees a bare span when one is present.
- **Blind-spot probe, because both expressions require a colon.** The prose form `line NNN` / `lines NNN` over the same file set returns two lines, both in `e2e/smoke.spec.ts`, and both are test DATA (`"mkvmerge output line 1"`), not citations. No citation uses the prose form.
- **The `ci.yml` citation is stale, verified at its target.** `queue.rs` resolves to exactly one tracked file, `crates/muxsmith-core/src/executor/queue.rs`; its line 73 at `2c04ac4` is `pub struct QueueOpts {`. At the parent of the commit that wrote the comment (`004e1e8`, `ci: cargo doc -D warnings as ninth gate part (#18b)`) line 73 was the first line of the `jobs` field's doc comment and line 74 carried the broken link `(see [`worker_count`])`. **So the comment means `QueueOpts::jobs`'s doc comment and the private `worker_count` helper it then linked**, and today that doc reads `(see the private `worker_count` helper)` - a code span, not a link, so the comment's claim is a historical record and the ruling gives historical statements no exception.
- **The fixture citation is the same bare-span form Plan 10 already ruled IN**, and its repair has a committed precedent: `crates/muxsmith-core/tests/profile_save.rs` now reads `defaulted fields (design D48)` where it once carried `design `:1517-1535``. The fixture's comment already names D48 in its own first token, so the repair is a token deletion exactly as it was there.

### Item 3's corpus: six sites to repair, nine to leave, and the code that decides which

- **`scalar_eq` has SIX arms and two of them coerce.** `crates/muxsmith-core/src/matcher.rs`, function `scalar_eq`, pasted: `(Scalar::Str(a), PropValue::Str(b)) => a == b`, `(Scalar::Bool(a), PropValue::Bool(b)) => a == b`, `(Scalar::Int(a), PropValue::Int(b)) => a == b`, `(Scalar::Int(a), PropValue::Float(b)) => (*a as f64) == *b`, `(Scalar::Float(a), PropValue::Float(b)) => a == b`, `(Scalar::Float(a), PropValue::Int(b)) => *a == (*b as f64)`, `_ => false`. The `raw:` arm of `exact_matches` calls it directly. So the comparison is byte-exact for strings and NUMERIC for numbers, and "byte-exact" is false exactly where a number is involved.
- **The coercion is a deliberate, already-tested design case.** `matcher.rs` carries `b7_raw_int_float_cross_compare`, whose body is `let t = track("audio", &[("new_gain", PropValue::Float(6.0))]); assert!(matches(&expr("exact: { raw:new_gain: 6 }"), &t, &lang()));`, and the plan-5.5 design document's case table records `B-7 | { new_gain: Float(6.0) } | exact: { raw:new_gain: 6 } | yes | int/float cross-compare`. **The behaviour is therefore settled and covered; only the wording is wrong.** This is stronger than the tracker's "deliberate-looking" and it is why Task A3 owes no new test.
- **Both coercion directions are reachable, and one needs no exotic file. Demonstrated end to end through the shipped binary (SI-3).** `Scalar` is `#[serde(untagged)]` with `Int` before `Float`, so only a literal with a decimal point deserializes as `Float`; `PropValue::from_json` makes an integral JSON number `Int` and a non-integral one `Float`. A probe file was muxed with the installed `mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit` from the repo's own `tone.wav` seed; its `mkvmerge -J` output reports `identification_format_version: 20` (matching `PINNED_IDENTIFICATION_FORMAT_VERSION: u64 = 20`) and `"audio_channels": 1`. A profile whose only rule is `match: { exact: { "raw:audio_channels": 1.0 } }` run through `muxsmith dry-run --json` produced `plan.assignments` containing `{"rule_index":0,"track_id":0,"track_kind":"audio"}` - **the `Float(1.0)` scalar matched the reported `Int(1)`**. **Fired negative control:** the same profile with `2.0` produced `missing-track`, so the matcher discriminates and the match above is not a vacuous pass.
- **The other direction is real too, from mkvtoolnix's own schema.** `~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json` declares five track properties as `type: number` rather than `integer`: `max_luminance`, `min_luminance`, `projection_pose_pitch`, `projection_pose_roll`, `projection_pose_yaw`; all five are `double` in the mkvtoolnix source. So an `Int` scalar against a reported `Float` is available as well.
- **The REPAIR set is exactly six lines across five files.** Expression, whose alternation was derived by reading each site rather than recalled: `git grep -nE 'byte-literal value equality|byte-exact value equality|byte-for-byte value equality|byte-genaue Wertgleichheit'` over live artifacts (excluding the process journal, retired plans, the three superseded design documents, the four house YAML files and `docs/ROADMAP.md`). Total **6**: `README.md:60`, `crates/muxsmith-core/src/matcher.rs:96`, the v1 spec at `:176` (section 4.4) and `:421` (section 9.2), `help/de/editor-match-expr-exact.md:23`, `help/en/editor-match-expr-exact.md:23`.
- **The RETAINED set is nine lines across seven files**, and every one of them is scoped to `language`/`codec_kind`: `git grep -nE 'byte-literal untyped equality|to byte-literal equality|matches byte-literally|byte-literal ab\.|byte-literally compares|Byte-literal against'` over the same surface returns `crates/muxsmith-core/src/matcher.rs:452` and `:466`, `crates/muxsmith-core/src/profile/validate.rs:408`, `crates/muxsmith-core/src/report/mod.rs:87`, `crates/muxsmith-core/tests/validate_semantics.rs:249`, the v1 spec at `:280` and `:421`, `locales/de/diagnostics.ftl:21`, `locales/en/diagnostics.ftl:14`.
- **Why the retained set is true as written, measured rather than argued.** `raw_opt_in_diagnostic` in `crates/muxsmith-core/src/profile/validate.rs` gates `RawOnKnownProperty` on `matches!(bare, "language" | "codec_kind")` - exactly two properties, in code - and `crates/muxsmith-core/src/capability/generated.rs` declares both as string-typed. `scalar_eq`'s `_ => false` arm means a non-`Str` scalar against a `Str` property never matches, so no value that can reach the comparison for those two properties is coerced, and "byte-literal" holds. **The spec therefore does not contradict itself after Task A3**: 4.4 and 9.2 will state that strings compare byte-for-byte while numbers compare numerically, and the diagnostics-table row's narrower claim about two string-typed properties is a specialization of that, not a second mechanism.
- **The v1 spec line at `:421` is in BOTH sets** - it carries the unscoped claim once and the `RawOnKnownProperty` claim once. Task A3 changes the first occurrence and leaves the second, which is why its acceptance runs a repair check AND a retention check rather than one file-level assertion. For the same reason `matcher.rs` is in both sets and a file-level byte-identity check on it is impossible.

### Item 4's corpus: the spec's 8.1 block is stale in four of its five lines

Captured from the shipped binary (`target/debug/muxsmith`, `muxsmith 0.1.0`, newer than every tracked `.rs` file, verified with `find crates src-tauri -name '*.rs' -newer target/debug/muxsmith` returning nothing):

| subcommand | v1 spec 8.1 states | the binary's `--help` lists | delta |
|---|---|---|---|
| `validate` | no flags | `--json`, `--locale` | both omitted |
| `dry-run` | `--source`, `--output`, `--json` | `--source`, `--output`, `--on-collision`, `--json`, `--locale` | `--on-collision`, `--locale` omitted |
| `run` | `--source`, `--output`, `--jobs`, `--fail-fast`, `--json` | `--source`, `--output`, `--on-collision`, `--jobs`, `--fail-fast`, `--json`, `--locale` | `--on-collision`, `--locale` omitted |
| `identify` | `--json` | `--json`, `--locale` | `--locale` omitted |
| `schema` | no flags | none beyond `-h/--help` | **correct** |

- **The exit-code line in the same block is stale too.** `grep -c '130' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` -> `0`. The spec's only process-exit-code statement is the 8.1 bullet `- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors.` Meanwhile `crates/muxsmith-cli/src/cli.rs` documents the contract as `0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)` - **a citation reaching past its source**, since spec 8.1 does not carry 130. `README.md:193`, which Plan 10 corrected, does carry it. So Plan 10 left the authoritative document underclaiming relative to a document it outranks.
- **Who can actually produce 130, measured.** `std::process::exit(130)` and `return 130;` both live in `crates/muxsmith-cli/src/commands/run.rs`, whose own comment records "this is the one registration in the process" for its `ctrlc::set_handler`; `grep -rn "ctrlc" crates/muxsmith-cli/src/` returns hits in that file only. So `run` is the one subcommand that returns 130 from Muxsmith's own code. D16, defined in `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`, states "the process exits 130", which is what makes `cli.rs`'s D16 half correct and its spec-8.1 half wrong.
- **The README is already right and needs no edit.** `README.md:125` reads "Four of them - `validate`, `dry-run`, `identify`, `run` - take `--json` ... and `--locale` ... `muxsmith schema` takes neither", and `:143`/`:153` carry `--on-collision` on `dry-run` and `run`. The assertion set for item 4 is therefore the spec alone among live artifacts.

### Item 5's corpus: three ordinals and one long line, all reproduced

- **The ordinals.** The tracker's own expression, re-run at `2c04ac4`: `grep -nE 'part [0-9]|parts [0-9]' BUILDING.md` returns exactly three lines - `:102` `The cross-target clippy run (part 6) type-checks the workspace for Windows`, `:134` `CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all`, `:135` `three OS legs (its Windows leg covers natively what part 6 cross-checks`. **Fired control:** the same expression over `README.md` returns `0`, and over `BUILDING.md` returns 3, so it discriminates. `:134` and `:135` are consecutive lines of one hard-wrapped paragraph, which is the shape `proc-wrapped-prose-quote-grep` names and the reason a by-paragraph reading saw two ordinals where there are three.
- **The long line.** A fence-aware line-length pass over `BUILDING.md` finds exactly **one** non-fenced line over 80 characters: `:138`, **86** characters, `and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)`. Nine lines in the file exceed 80 in total; the other eight are all inside fenced blocks. `:138` sits in the SAME paragraph as `:134` and `:135`, which is exactly why the reflow and the ordinal rewrite must land in one edit.
- **No markdown or line-length linter exists** to enforce the 80-column norm: the repo has no `.editorconfig`, no markdownlint config and no prettier config, and no gate part reads `BUILDING.md` for line width. The norm is the file's own, so the acceptance check is a measured line-length assertion rather than a lint invocation.
- **The gate part that DOES read the file is green and stays the boundary.** `python3 scripts/ledger-lint.py` prints `ledger-lint: 548 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` and exits 0. The Rust gate block's six commands in order are `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items`; `cargo deny check`; `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings`. **So "parts 1-4" names fmt, clippy, test and doc, and "part 6" names the cross-target clippy run**, which is what Task A1's replacement text substitutes.
- **A spelled-ordinal sweep, because the tracker's expression cannot see one.** The expression, written out because an alternation abbreviated with an ellipsis is an unenumerated set a re-runner would have to invent: `\b(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth)[- ](gate|part)|\b(gate|part) (one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\b|as the [a-z]+ gate part`, run over all tracked non-`docs/` text files (excluding `png ico icns wav snap lock`). It returns exactly one gate-related hit: `.github/workflows/ci.yml:88`, `# Plan 5.5 Task 12 (#18b): rustdoc correctness as the ninth gate part.` **Fired control:** the same family of terms over `docs/superpowers/plans` hits `2026-07-11-plan-5.5-pre-1.0-hardening.md` three times. **That hit is a measured NON-defect, recorded so nobody renumbers it:** it is a dated provenance statement about what Plan 5.5 Task 12 added, corroborated by that retired plan's own task heading, and the ROADMAP's MEASURED block establishes that such a record is not falsified to today's count.
- **The one live consumer of the "part 6" wording is a Tier-2 statement.** `gate-includes-cross-target-lint-for-the-unrun-os` in `docs/process-conventions.yaml` reads "documented as gate part 6 in `BUILDING.md`". Task A1 falsifies it. **The plan may not edit that file** (Global Constraints), so it is surfaced in the plan close for the controller, who is its single writer.

### Existing gate and CI facts the tasks lean on

- `package.json` scripts, pasted: `lint` -> `eslint .`, `build` -> `vue-tsc --noEmit && vite build`, `check:i18n` -> `node scripts/check-i18n.mjs`, `test:e2e` -> `tsc --noEmit -p e2e/tsconfig.json && vite build --config e2e/vite.harness.config.ts && vite build --config e2e/vite.mount.config.ts && playwright test`.
- **`pnpm check:i18n` gates the `help/` tree, which is what makes Task A3's two help edits machine-checked.** `scripts/check-i18n.mjs` implements D62's six hard-fail conditions over `help/`: referenced-to-file per locale, file-to-referenced orphans, `help/` versus `locales/` locale lockstep, an external-URL ban, a table/pipe ban, and a raw-HTML ban with code spans exempt. A3's edits add prose with code spans only, no pipe, no URL, no HTML.
- **No `rustfmt.toml` exists**, so rustfmt runs on defaults; `wrap_comments` is off by default, which is why A3's fenced comment is reproduced as written rather than as a formatter would rewrap it. The fenced comment is still kept inside the surrounding file's own width so `cargo fmt --all --check` has nothing to say about it either way.

## Corrections to the brief found at plan-authoring (`proc-57-briefs-not-ground-truth`)

None of the six changes a ruling. Four change a set a task acts on.

| # | Brief statement | Reality |
|---|---|---|
| 1 | Item 2 is "`.github/workflows/ci.yml`'s surviving line-number citation" (one site) | **TWO sites survive outside Plan 10's selector, not one.** Run over every tracked non-`docs/` file with a tree-derived extension alternation, expression A returns the `ci.yml` hit and expression B returns `crates/muxsmith-core/tests/fixtures/all-non-default.yaml:2`, a `#` comment carrying the bare span `:1517-1535` into the design document. It is in scope on the same widened ruling and by the same controller ruling Plan 10 applied to the identical form: a bare span with no filename is the WORSE form of what the owner banned, not an exempted one. Task A2 owns both. |
| 2 | Item 3's wording is wrong "in three places at once" (spec, `matcher.rs`, README) | **The set of assertions is 15 lines, split 6 to repair and 9 to leave.** The repair set adds `help/en/editor-match-expr-exact.md` and `help/de/editor-match-expr-exact.md`, both shipped user-facing help, and it counts the v1 spec's two unscoped occurrences separately because they sit in different sections. The nine retained lines are all scoped to `language`/`codec_kind`, whose trigger set is `matches!(bare, "language" \| "codec_kind")` in code and whose two members are string-typed, so "byte-literal" is true of them and repairing them would rewrite true sentences. Both sets, both expressions and the code that decides the split are in the authoring section. |
| 3 | Item 4 is "the v1 spec's section 8.1 synopsis omits `validate`'s flags" | **Four of the block's five synopsis lines are stale, and the block's exit-code bullet is stale too.** `--locale` is missing from all four flag-bearing subcommands, `--on-collision` from `dry-run` and `run`, `--json` from `validate`; only `muxsmith schema` is correct. Separately `130` appears zero times in the whole v1 spec while `crates/muxsmith-cli/src/cli.rs` cites "spec 8.1" for it, and `README.md`, which the spec outranks, carries it after Plan 10. Same defect direction as the item the brief names - the authoritative document underclaiming a shipped surface - inside the same block, and the spec-amendment self-contradiction sweep the brief invokes is what makes it this item's business rather than a new vehicle's. |
| 4 | Item 1(b): the hypothesis is that "this configuration may not fail on that class" | **Refined by measurement, and the refinement changes what the disposition question is.** RustSec does carry the advisory (`RUSTSEC-2024-0429`, `informational = "unsound"`) and its `aliases` field names the exact GHSA GitHub reports, so the two mechanisms see one advisory. But `cargo deny -L info check advisories` mentions that id on zero lines while mentioning `RUSTSEC` on 54 - it is not an error, not a warning, and not an ignored note. So the class is not evaluated at all rather than evaluated and tolerated, and "make the check fail on that class" may not be an available configuration at 0.19.9. B1 establishes which keys the version actually accepts; that part is deliberately not pre-empted here. |
| 5 | (the brief's method warning about a pattern's own enumerated set) | **This plan walked into that defect and records it rather than only citing it.** Its first pass at item 3's corpus used `byte[- ]?(exact\|literal\|identical\|wise)`, which cannot match `byte-for-byte` or `byte-genaue` and therefore missed both help topics; they surfaced only because a second, differently-derived expression over `raw:` mentions found them. The corrected alternation in the authoring section was built by reading each site's actual words. |
| 6 | Item 5: three ordinal sites and one over-80 line | **Reproduced exactly, and the surface has one more member the tracker's expression cannot see.** `grep -nE 'part [0-9]\|parts [0-9]' BUILDING.md` returns `:102`, `:134`, `:135`, and the fence-aware length pass returns `:138` at 86 characters as the only non-fenced line over 80. The addition is that a SPELLED ordinal exists in a live file (`ci.yml`, "the ninth gate part") which that expression cannot match; it is measured to be a dated provenance record, so it is a non-defect, and it is recorded here so a later sweep does not "repair" it and so the tracker's expression is not mistaken for a complete instrument. |

## Work-item coverage map

The walk the plan reviewer repeats. A row missing here is a defect.

| Brief work item | Implemented by |
|---|---|
| W1. The two open dependency alerts, three parts in one task | Task B1 (part a: the `postcss` lockfile move; part b: the `cargo deny` measurement, no fix; part c: the `glib` investigation, no fix) |
| W2. `.github/workflows/ci.yml`'s surviving line-number citation | Task A2, **widened by correction 1 to the two-member set the derivation returns** |
| W3. "byte-exact" overstates what `raw:` does for numeric scalars | Task A3, **widened by correction 2 to six repair sites across five files, with a nine-member retained set proven unchanged** |
| W4. The v1 spec's section 8.1 synopsis omits `validate`'s flags | Task A4, **widened by correction 3 to the whole 8.1 synopsis block and its exit-code bullet** |
| W5. `BUILDING.md`'s three positional gate ordinals plus its one over-80 prose line | Task A1, both in ONE edit |

## Sequencing, the two streams, and the merge order

**Streams run concurrently; tasks inside stream A are strictly serial: A1 -> A2 -> A3 -> A4.**

### Worktrees, merges and gate runs

- **Stream A** works in a worktree the controller creates at `../muxsmith-plan11-a` on a branch `plan-11-stream-a` off `master`. **Stream B** works in a worktree at `../muxsmith-plan11-b` on a branch `plan-11-stream-b` off `master`. The controller creates both; no implementer creates, enters or leaves a worktree, and no implementer calls a session-relocation tool.
- **Each stream runs the full gate as `BUILDING.md` enumerates it, foreground, in its own worktree, green, before it reports done.** A fresh worktree carries neither `target/` nor `node_modules/`, so its first gate run needs `pnpm install` and a cold cargo build; that cost is the price of the split and is not re-argued.
- **Merge order: stream A first, then stream B.** Ground: stream B is the stream whose gate result can move for reasons outside anybody's diff - a transitive lockfile bump reaches `pnpm lint`, `pnpm build`, `pnpm check:i18n` and `pnpm test:e2e` - so merging it second means that if the merged state goes red, the newly merged half is the only candidate and the bisect is free. Merging it first would leave stream A's merge gate carrying two possible causes.
- **The controller runs the full gate on `master` after each merge** (`proc-08-parallel-worktrees`), so two post-merge runs. A merge that needs a manual edit in a product file is dispatched, never resolved by the controller (`proc-01-sdd` bright line).
- **One push, at the plan close, after the final post-merge gate is green**, with its `gh-log.md` entry.
- Both streams' Files lists are **pairwise disjoint** and so are their write-sets: no stream-A task reads or writes `pnpm-lock.yaml`, `package.json`, `Cargo.lock` or `deny.toml`, and B1 writes none of the eight files stream A lists. The one place the two streams touch the same subject is prose: A1's replacement paragraph in `BUILDING.md` names `cargo deny check` as a CI job, and B1 must not change that arrangement - which it does not, because B1 changes neither `deny.toml` nor the invocation.

### Why stream A is cut into four tasks and ordered this way

- **Four tasks, not one.** The four work items are four unrelated fact sets with disjoint acceptance checks and disjoint files, and a single task would make its review a four-subject grading with one verdict. Because the doctrine's boundary already put all four in ONE worktree, the extra granularity costs four commits and four exit-bar subsets, not four full gate runs.
- **Four tasks, not two.** Merging A3 and A4 into one "spec amendment" task was considered and rejected: A3 spans five files in two natural languages and its own retained-set invariance, A4 is spec-only plus a surfacing duty, and the two facts share nothing but the file they live in. Serial ordering already gives them the non-concurrency the brief requires.
- **A1 first**, because it is the only stream-A task that edits a file a gate part parses. Putting it first buys three later independent executions of `scripts/ledger-lint.py` against the edited `BUILDING.md` on a real tree at zero extra cost, and surfaces a broken invariant at A1's own review rather than at the stream gate. This is the same reasoning Plan 10 used to put its gate-tooling task first.
- **A2 second.** It touches `.github/workflows/ci.yml` and one test fixture, which nothing else in either stream reads or writes. It is placed before the two spec tasks only so that the stream's two independent files are done while the spec is still untouched, which keeps any spec-related NEEDS_CONTEXT from stalling unrelated work.
- **A3 before A4, the one hard edge.** Both amend `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`. A4 owes a spec self-contradiction sweep (doctrine section 1: a spec amendment sweeps the spec for self-contradictions before commit), and a sweep is only worth running over the FINAL text, so it runs after A3's amendment has landed. Running A4 first would make its sweep true of a spec A3 is about to change.
- **Parallelism inside stream A is unavailable in principle, not merely unattractive.** A3 and A4 write the same file; A1's edit changes what every later gate run reads. One tree means one index and one working state, and the serial ruling **binds the controller's dispatch concurrency too** (`a-serial-ruling-binds-dispatch-concurrency-too`): no second writer is dispatched into a stream's worktree while one of its tasks is live.

Commits: each of the five tasks commits on its stream's branch with explicit pathspecs, using the fenced `git add` and `git commit` blocks in its own commit step.

## Acceptance map

Every work item, walked in its HALVES, to the task implementing it and the named producer of each half. "MV" = machine-verifiable by a command or test named here. The **prescribed** column separates evidence a reviewer can reproduce today from a fire an implementer will perform later against a deliverable that does not yet exist: `authoring` means the value is in the authoring section and is reproducible now; `task` means the producer runs at execution time and the reviewer grades the fire's DESIGN against its specification rather than re-running it.

| # | Observable half | Producer | MV | evidence |
|---|---|---|---|---|
| W1-a | `pnpm-lock.yaml` resolves `postcss` at `>= 8.5.18` | B1 Step 2: `pnpm why postcss` and the four lockfile sites, pasted with the observed version. The requirement is the inequality; the landing version is not fenced, because the authoring probe landed on `8.5.24` while the registry's `latest` read `8.5.25` | yes | task (authoring: the move is available, both parent ranges measured) |
| W1-b | `package.json` is byte-identical and carries no `pnpm.overrides` key | B1 Step 2's separate check: `git diff --exit-code -- package.json` plus `grep -c '"overrides"' package.json`. **A separate row from W1-a because the mechanism ruling is exactly that the lockfile moves and the manifest does not** - one producer named for "postcss was bumped" would cover the lockfile side only. **RED, and it costs nothing because the tree supplies it:** the SAME instrument on the SAME tree must exit **1** for `pnpm-lock.yaml`, which did move, while exiting **0** for `package.json`. An `--exit-code` check reporting no change is otherwise indistinguishable from one aimed at a path that cannot change | yes | task |
| W1-c | The lockfile diff covers only `postcss` and `nanoid` | B1 Step 2: the full `git diff -- pnpm-lock.yaml`, pasted, with every changed package named. `nanoid` is in scope because it is postcss's own dependency and the authoring probe measured it moving 3.3.15 -> 3.3.16; a third package appearing is a finding, not a pass | yes | task (authoring: eight hunks, two packages) |
| W1-d | The four frontend gate parts stay green on the bumped lockfile | B1 Step 3: `pnpm lint`, `pnpm build`, `pnpm check:i18n`, `pnpm test:e2e`, each run foreground and each result pasted. **Four named commands rather than "the frontend gate", because a lockfile bump can move exactly one of them** | yes | task |
| W1-e | GitHub's alert and RustSec's record are the SAME advisory, not two | B1 Step 4: the advisory file's `aliases` field beside the alert's `ghsa` field, both pasted | yes | authoring (`GHSA-wrw7-89jp-8q8g` on both sides) |
| W1-f | The advisory's own class is established at RustSec's record | B1 Step 4: `informational = "unsound"` quoted from the local advisory-db file, with the file's path | yes | authoring |
| W1-g | This project's effective `cargo deny` handling of that class is established at the tool's own source | B1 Step 5: which configuration keys `cargo-deny 0.19.9` accepts for informational advisory classes, read at the tool's documentation or source and cited, plus what `deny.toml` sets. **A separate row from W1-f because one is the advisory's property and the other is the tool's, and a report naming only the first restates the hypothesis** | yes | task (authoring: the advisory is not surfaced at any severity, which is the observation the mechanism must explain) |
| W1-h | A demonstration reproduces the green result and the reported advisory TOGETHER | B1 Step 6: one pasted transcript containing `cargo deny check advisories` exiting 0, the same run at `-L info` with `RUSTSEC-2024-0429` absent from it, a count of the `RUSTSEC` ids that ARE present, and the advisory file proving the id exists locally. **The absence half needs its own fired control, which is the id count: an empty grep for one id looks identical whether the id is absent or the pattern is broken** | yes | task |
| W1-i | `deny.toml` and the `cargo deny` invocation are unchanged | B1 Step 7: `git hash-object deny.toml` against `git rev-parse <base>:deny.toml`, and `git diff --exit-code -- deny.toml BUILDING.md .github/workflows/ci.yml`. Named per file rather than asserted from a clean `git status`, which in a repository with a concurrent writer proves only that nothing is uncommitted right now. **RED, from the same tree:** the blob comparison must DIFFER for `pnpm-lock.yaml` (whose hash-object cannot equal the base blob after Step 2) and MATCH for these three, so the instrument is shown to discriminate rather than to agree by construction | yes | task |
| W1-j | Whether `glib` can move independently of Tauri's tree is ANSWERED | B1 Step 8: `cargo tree -i glib@0.18.5 -e normal --depth 1` and the gtk-rs version tally, pasted, with the verdict stated in the brief's own terms - an upgrade project rather than a bump - and named as an acceptable, expected completion | yes | authoring (twelve parents, all one generation) |
| W1-k | `Cargo.lock` is unchanged | B1 Step 8: `git diff --exit-code -- Cargo.lock`, sharing W1-i's fired control (the same command exits 1 for `pnpm-lock.yaml`). Separate row from W1-i because part (c) is the part a well-meaning implementer would be tempted to "just fix", and because a `cargo` invocation can rewrite the lockfile as a side effect of resolution without anyone intending it | yes | task |
| W2-a | No filename-plus-line citation survives in a tracked non-`docs/` file | A2 Step 4, absence check A. **RED: the pre-state run returns exactly 1 line** (`.github/workflows/ci.yml`). **GREEN: 0** | yes | task (authoring: the pre-state count and its fired control) |
| W2-b | No bare `:<line>` span survives in a tracked non-`docs/` file | A2 Step 4, absence check B. **RED: the pre-state run returns exactly 1 line** (`crates/muxsmith-core/tests/fixtures/all-non-default.yaml`). **GREEN: 0.** Separate row because expression A cannot see this form, which is exactly how this member escaped Plan 10's corpus | yes | task (authoring: the pre-state count and its fired control) |
| W2-c | Each rewritten comment still points at what it meant | A2 Step 2: per site, the target opened and the named symbol quoted from it in the report | yes | authoring for both sites (`QueueOpts::jobs` and its then-linked `worker_count`; `D48`, already named in the fixture's own text) |
| W2-d | The edited workflow is still valid YAML | A2 Step 4: `python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0. **A row of its own because no gate part parses `.github/workflows/ci.yml`** - the local gate would stay green over a syntactically broken workflow | yes | task |
| W2-e | The edit is comment text only, in both files | A2 Step 4: `git diff -U0` over the two files, every changed line shown to begin with a comment marker (`#` in both). Separate from W2-d: valid YAML and comment-only are independent properties, and the fixture is consumed by a test whose data must not move | yes | task |
| W3-a | The v1 spec's two unscoped `raw:` statements state the numeric behaviour | A3 Step 2: the two fenced replacements applied, and the repair expression returning 0 over the spec | yes | task (authoring: the two sites and their exact current text) |
| W3-b | `matcher.rs`'s `raw:` arm comment agrees with `scalar_eq` | A3 Step 3: the fenced comment applied, naming the cross arms and case B-7; `cargo fmt --all --check` and `cargo clippy --workspace --all-targets -- -D warnings` green | yes | task |
| W3-c | The README's `raw:` bullet agrees | A3 Step 4: the fenced replacement applied; the repair expression returning 0 over `README.md` | yes | task |
| W3-d | `help/en`'s `raw:` bypass section agrees | A3 Step 5: the fenced replacement applied; `pnpm check:i18n` green | yes | task |
| W3-e | `help/de`'s `raw:` bypass section agrees | A3 Step 5: the fenced German replacement applied; `pnpm check:i18n` green. **A separate row from W3-d because a wording repair silently staying English in the other locale is the characteristic failure here, and the i18n gate checks key and topic completeness rather than whether two topics say the same thing** | yes | task |
| W3-f | The whole unscoped-claim set is gone | A3 Step 6, absence check R. **RED: the pre-state run of the repair expression returns exactly 6 lines across 5 files. GREEN: 0** | yes | task (authoring: 6, with the alternation derived per site) |
| W3-g | The nine retained scoped sites are untouched | A3 Step 6, invariance check K. **The retention expression returns exactly 9 lines across 7 files on BOTH the pre-state and the end state.** Its fire is a deliberate deletion of one retained site, which must drop the count to 8, then restored - because an invariance check that never moves is indistinguishable from a broken one | yes | task (authoring: 9) |
| W3-h | The behaviour is unchanged | A3 Step 6: `cargo test -p muxsmith-core` green with `b7_raw_int_float_cross_compare` and `b8_raw_language_is_byte_literal_no_normalization` named from the pasted output, plus the SI-3 re-run of the two authoring commands reproducing the match and its negative control | yes | task (authoring: the end-to-end demonstration and its fired control) |
| W4-a | The spec's 8.1 synopsis block matches the shipped binary for all five subcommands | A4 Step 2: the fenced block applied, and the divergence table re-derived in Step 1 from `<sub> --help` for every subcommand the binary lists, pasted per subcommand | yes | task (authoring: the five-row table) |
| W4-b | The spec's exit-code bullet carries 130 and says who can reach it | A4 Step 3: the fenced replacement applied; `grep -c '130'` over the spec returning a non-zero count. **A separate row from W4-a because the two facts fail independently: one is a flag surface, the other an exit-code contract, and they are only neighbours** | yes | task (authoring: `130` absent from the spec, produced only in `commands/run.rs`) |
| W4-c | The amendment introduces no spec self-contradiction | A4 Step 4: the sweep, run as an enumeration - every other place in the spec that states a CLI flag, a subcommand synopsis or an exit code, each listed with its verdict. **Both sweeps named in Step 4 (the exit-code-sentence sweep and the double-dash flag sweep) are run and pasted; the flag sweep's fired control is that it must return the amended 8.1 block itself** | yes | task (authoring: the spec's only two exit-code sentences) |
| W5-a | No positional gate ordinal remains in `BUILDING.md` | A1 Step 3, absence check O. **RED: the pre-state run returns exactly 3 lines (`:102`, `:134`, `:135`). GREEN: 0** | yes | task (authoring: 3, with a fired control) |
| W5-b | No non-fenced prose line in `BUILDING.md` exceeds 80 characters | A1 Step 3, absence check L. **RED: the pre-state run returns exactly 1 line, at 86 characters. GREEN: 0.** A separate row from W5-a because the two are different properties of the same paragraph and each can be fixed while the other regresses | yes | task (authoring: 1 non-fenced over-80 line out of 9 over-80 lines total) |
| W5-c | `scripts/ledger-lint.py`'s gate-count invariant still holds on the edited file | A1 Step 3: `python3 scripts/ledger-lint.py` exits 0 and prints its summary line. **Fired: the check is made to fire once on the edited file** by changing the canonical total, watching it exit 1, and restoring - so a green run is not mistaken for a check that no longer looks at the file | yes | task (authoring: the green baseline and its summary line) |
| W5-d | The three marked gate blocks, their markers and the canonical sentence are byte-identical | A1 Step 3: `git diff -U0 -- BUILDING.md` showing every changed line, none of them a marker line, a fence line, a line inside a fence, or the canonical gate-total sentence. Separate row from W5-c: the invariant can hold while a block was edited in a way that keeps the arithmetic. **RED, supplied by the neighbouring fire at no cost:** while the `ledger-lint` fire has the canonical sentence mutated, this same diff command must SHOW that sentence as changed; it comes back clean only after the restore. Paste it in both states | yes | task |
| W5-e | The Tier-2 statement that cites "gate part 6" is surfaced, not silently orphaned | A1 Step 4: the report names `gate-includes-cross-target-lint-for-the-unrun-os` in `docs/process-conventions.yaml` and quotes the falsified clause. **Not machine-verifiable as a repair, because the plan may not edit that file**; the row claims the surfacing, and the repair is a controller close action | yes (the surfacing) | authoring (the entry and its clause) |

## Stream A worktree

The controller creates it before dispatching A1:

```bash
git worktree add -b plan-11-stream-a ../muxsmith-plan11-a master
```

Every stream-A task works there, with absolute paths. No implementer runs `git worktree`.

---

## Task A1: `BUILDING.md` loses its three positional gate ordinals and its one long prose line, in one edit (W5)

Read first: the plan brief's item 5; `docs/ROADMAP.md`'s "Gate-count derivation has no check" section in full, including its MEASURED block, its NARROWED FORM block, its DONE block and the "A neighbouring class" paragraph that routes this item; **`BUILDING.md` in full**, because it is the subject being rewritten and because the four commands the replacement text names must be read out of its own Rust gate block rather than taken from this plan; `scripts/ledger-lint.py` in full, because Step 3's fire has to know which lines the check reads; Tier-2 `ledger-lint-runs-before-every-push`, `proc-wrapped-prose-quote-grep` (the hard-wrapped paragraph), `gate-includes-cross-target-lint-for-the-unrun-os` (the statement Step 4 surfaces), `proc-verification-step-must-be-falsifiable` and `proc-check-green-state-reachable`. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `BUILDING.md` - two regions only: the cross-target clippy paragraph's opening line, and the CI paragraph in the House-knowledge check section. Nothing else in the file, and in particular no marker line, no fence, no line inside a fence, and not the canonical gate-total sentence.

**Interfaces:**
- Consumes: nothing.
- Produces: a `BUILDING.md` with no positional gate ordinal, which is what makes the Tier-2 repair in the plan close a rewrite of one clause rather than a redesign.

- [ ] **Step 1: re-measure before editing.** Run both expressions and paste both with their full output.

```bash
grep -nE 'part [0-9]|parts [0-9]' BUILDING.md
```

```bash
python3 - <<'EOF'
lines = open('BUILDING.md').read().split('\n')
fence = False
for i, l in enumerate(lines, 1):
    if l.startswith('```'):
        fence = not fence
        continue
    if fence:
        continue
    if len(l) > 80:
        print(f'{i}: len={len(l)}  {l!r}')
EOF
```

  The authoring runs returned **three ordinal lines** (`:102`, `:134`, `:135`) and **one over-80 non-fenced line** (`:138`, 86 characters). **If a re-measurement returns a different set, that set is the ground truth and the report says so, and the fenced replacements below are re-checked against the text actually present before either is applied.** A hit this plan does not fence returns as NEEDS_CONTEXT rather than being rewritten at the keyboard.

- [ ] **Step 2: the two edits, both fenced, applied together.** They are one edit in substance: the paragraph that carries the long line also carries two of the three ordinals, so a reflow moves them and an ordinal rewrite reflows them.

  (a) Replace exactly the line

```
The cross-target clippy run (part 6) type-checks the workspace for Windows
```

  with exactly

```
The cross-target clippy run type-checks the workspace for Windows
```

  Nothing else in that paragraph changes. The parenthetical is a pure deletion: the sentence already names the run, so the position added nothing, and once the file states a total of eleven parts a bare "part 6" acquires a second possible referent that only section context resolves.

  (b) Replace exactly the six-line paragraph

```
CI (`.github/workflows/ci.yml`) runs Rust-gate parts 1-4 natively on all
three OS legs (its Windows leg covers natively what part 6 cross-checks
from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`, and
`pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants, Plan-8 rider)
run as independent jobs.
```

  with exactly

```
CI (`.github/workflows/ci.yml`) runs the Rust block's `cargo fmt`,
`cargo clippy`, `cargo test` and `cargo doc` commands natively on all three
OS legs (its Windows leg covers natively what the cross-target clippy run
cross-checks from Linux) plus `pnpm lint`, `pnpm build`, `pnpm check:i18n`,
and `pnpm test:e2e` on every master push, `v*` tag and PR; `cargo deny check`
and `scripts/ledger-lint.py` (house-knowledge and gate-count invariants,
Plan-8 rider) run as independent jobs.
```

  Three things about this replacement, stated so none of them is a judgment call. **The four commands are named, not numbered**, and they are the first four of the Rust gate block in its own order - `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` - shortened to their subcommand in the replacement text because the paragraph is prose about CI rather than a command listing; **if the implementer's own reading of the Rust block finds a different first four, that is NEEDS_CONTEXT with both readings pasted, not a licence to adjust the fence.** The longest line is 77 characters, so the over-80 line is gone as a consequence of the rewrite rather than by a separate reflow. And no backtick span is broken across a line end, because a code span split over a hard wrap renders inconsistently.

- [ ] **Step 3: verification, four checks, all outputs pasted.**
  - **Absence check O, positional ordinals.** `grep -nE 'part [0-9]|parts [0-9]' BUILDING.md`. **Its own fire is Step 1's pre-state run, which returned exactly three lines**; on the end state it must return nothing. **Soundness control, because an empty grep and a broken grep look identical:** the same expression over `docs/superpowers/plans/2026-07-11-plan-5.5-pre-1.0-hardening.md` returns matches, so the pattern demonstrably hits a positional gate ordinal when one is present. **Reachable green state, argued member by member:** all three matched lines sit inside the two regions Step 2 rewrites, and neither replacement contains the token `part` followed by a digit.
  - **Absence check L, line length.** The fence-aware script from Step 1. **Its own fire is Step 1's pre-state run, which returned exactly one line at 86 characters**; on the end state it must print nothing. **Soundness control:** run the same script with the threshold lowered to 60, which must print many lines, proving the script measures and reports rather than silently printing nothing. **Reachable green state:** the only over-80 non-fenced line is inside the paragraph Step 2(b) replaces, and the replacement's longest line is 77.
  - **The gate part that reads this file, made to fire.** `python3 scripts/ledger-lint.py` must exit 0 on the end state and print its summary line. Then, as a fire: change `11 parts` to `12 parts` in the canonical gate-total sentence, run again, watch it exit 1 naming the total mismatch, and **restore**. Paste both runs. Without this, a green run cannot be told apart from a check that no longer looks at the file at all. The restore is proven in the diff check below.
  - **Diff scope, and the same diff fired.** `git diff -U0 -- BUILDING.md`, pasted in full. Every changed line must lie in one of Step 2's two regions. **No marker line - `<!-- gate-total; checked by scripts/ledger-lint.py -->` and the three `<!-- gate-block: rust; ... -->`, `<!-- gate-block: frontend; ... -->`, `<!-- gate-block: house; ... -->` lines, each ending `checked by scripts/ledger-lint.py -->`, all four named rather than abbreviated - no fence line, no line inside a fence, and not the canonical gate-total sentence may appear as changed** - if one does, either the fire was not restored or the edit went wide, and both are defects. **Fired at no extra cost, using the fire above:** while the canonical sentence is still mutated for the `ledger-lint` fire, run this same diff command and paste it; it must SHOW that sentence as changed. Then run it again after the restore, where it must not. A clean diff that was never seen dirty cannot be told apart from a diff aimed at the wrong pathspec. `git diff --stat` must name exactly one file.
  - **Test duty, weighed** (`tests-ship-with-the-feature-never-after`): this task produces no user-visible consequence - it changes prose in a build document - so it ships no new test, and the reason is that there is no behaviour to observe, not that the scope is tight. The behaviour that IS observable near it, the gate-count invariant, already has its check in `scripts/ledger-lint.py`, which this task exercises and fires rather than extends.

- [ ] **Step 4: surface, do not edit.** The report names, with the quoted clause and no edit:
  - `docs/process-conventions.yaml`'s Tier-2 entry `gate-includes-cross-target-lint-for-the-unrun-os`, whose statement contains "documented as gate part 6 in `BUILDING.md`". This task's edit falsifies that clause. The plan may not edit house-knowledge YAML; the controller is its single writer.
  - `docs/ROADMAP.md`'s "A neighbouring class" paragraph in the "Gate-count derivation has no check" section, which enumerates the three ordinal sites this task removes and cites the 86-character line. Its disposition is a controller close action.
  - `.github/workflows/ci.yml`'s comment "rustdoc correctness as the ninth gate part", found by a spelled-ordinal sweep the tracker's own expression cannot match. **It is reported as a measured NON-defect and is NOT edited**: it is a dated provenance statement about what Plan 5.5 Task 12 added, corroborated by that retired plan's own task heading, and the ROADMAP's MEASURED block establishes that such a record is not renumbered to today's count. It is named so that a later sweep does not "repair" it.

- [ ] **Step 5: commit.**

```bash
git add BUILDING.md
git -c commit.gpgsign=false commit -m "docs: BUILDING.md names the gate commands instead of numbering them, and its CI paragraph fits 80 columns" -- BUILDING.md
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the two fenced replacements; that the reflow and the ordinal rewrite land together; that the four commands are named rather than numbered; that no marker, fence, fenced line or canonical sentence is touched; that the three surfaced items are surfaced rather than edited; that `ci.yml`'s spelled ordinal is a record and stays.

---

## Task A2: the two line-number citations that survive outside Plan 10's source selector (W2)

Read first: the plan brief's item 2; **Tier-2 `comments-locate-by-symbol-never-by-line-number` in full**, including its handle, its SCOPE BOUNDARY sentence and its "WIDENED BY OWNER RULING 2026-07-29 (session 28)" clause, which is the governing text for this task and which reaches CI and configuration comments; Tier-2 `code-comment-line-citations-drift` for the two citation classes; `docs/ROADMAP.md`'s "Docs accuracy" first entry in full, including its "OPEN OWNER QUESTION" paragraph, **which the Tier-2 statement above supersedes and which the controller repairs, not this task**; `.github/workflows/ci.yml`'s `cargo doc` step and its leading comment block; **`crates/muxsmith-core/src/executor/queue.rs`**, because Step 2 names a symbol out of it and that cannot be done without opening it; `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` and **`crates/muxsmith-core/tests/profile_save.rs`**, whose committed `(design D48)` form is the precedent this task follows for the fixture's bare span; the authoring section's corpus measurement for this item. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `.github/workflows/ci.yml` (the `cargo doc` step's leading comment block, comment text only - no `runs-on`, no pin, no `run:` line, no `name:`, no other job or step)
- Modify: `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` (the file-top `#` comment block, comment text only - not one line of profile data)

**Interfaces:**
- Consumes: nothing.
- Produces: the tree-wide closure of the class, which the plan close's ROADMAP disposition may then state - **and may state only with its own surface named**, because "tree-wide" here means every tracked file outside `docs/`, where process artifacts legitimately keep citing a line at a named commit.

- [ ] **Step 1: RE-MEASURE both expressions before editing anything, and paste both with their full output.** The cited-extension alternation is derived from the tree in the first command rather than typed from this plan, because a pattern's own enumeration is a claim (`a-search-whose-terms-come-from-memory-produces-a-false-absence`).

  **Expression A, filename citations:**

```bash
EXT=$(git ls-files | sed -n 's/.*\.\([A-Za-z0-9_]*\)$/\1/p' | sort -u | paste -sd'|')
echo "EXT=$EXT"
git ls-files | grep -v '^docs/' | xargs grep -nE "[A-Za-z0-9_./-]+\.($EXT):[0-9]+"
```

  **Expression B, bare line spans.** Run PER FILE and strip the `file:line:` prefix before any second filter: the one-pipeline form's own prefix matches the filename pattern, so a naive filter excludes every line and reports a clean tree that is not clean.

```bash
git ls-files | grep -v '^docs/' | grep -vE '\.(png|ico|icns|wav|snap|lock)$' \
  | while read -r f; do grep -nE '(^|[[:space:]`,(])[:][0-9]+' "$f" | sed "s|^|$f:|"; done
```

  The authoring runs returned **one line each**: `.github/workflows/ci.yml` under A, `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` under B. **If a re-measurement returns a different set, that set is the ground truth and the report says so**; a hit in a file this task's Files list does not name, or a hit outside a comment, returns as NEEDS_CONTEXT rather than being edited, the second because this task may not touch data or code and such a hit would make the green state unreachable inside its own constraints.

- [ ] **Step 2: rewrite both sites by the ruling's own handle** - "replace the number with the symbol the line sits in; where no symbol names it, name the nearest one plus what you mean". Naming the FILE stays normal and wanted. **Historical statements get no exception**; the distinction between a live pointer and a historical record disappears rather than being judged.

  (a) **`.github/workflows/ci.yml`.** Replace exactly the two lines

```
      # correctness, so a broken intra-doc link (queue.rs:73, linking a
      # private item) rotted silently since Plan 4 until this task. All
```

  with exactly

```
      # correctness, so a broken intra-doc link (in `QueueOpts::jobs`'s doc
      # comment in crates/muxsmith-core/src/executor/queue.rs, which then
      # linked the private `worker_count` helper) rotted silently since
      # Plan 4 until this task. All
```

  **Why that symbol, verified at the target rather than at the cited line.** The cited line 73 today is `pub struct QueueOpts {`, so naming the symbol the LINE now holds would name the wrong thing. At the parent of the commit that wrote this comment, line 73 was the first line of the `jobs` field's doc comment and the line after it carried the broken link `(see [`worker_count`])`. So the comment means that field's doc and that helper. **Verify both by opening the file and by reading the citing commit's parent** before writing the replacement; the authoring section pastes what those two reads returned.

  (b) **`crates/muxsmith-core/tests/fixtures/all-non-default.yaml`.** Replace exactly the two lines

```
# D48 guard 1 fixture: every one of the 17 defaulted fields (design
# :1517-1535) set to a value that is NOT its default. A predicate that
```

  with exactly

```
# D48 guard 1 fixture: every one of the 17 defaulted fields (design D48)
# set to a value that is NOT its default. A predicate that
```

  **Nothing is invented: the surviving identifier is one the comment itself already supplies** in its own first token, and the form is byte-for-byte the one Plan 10 committed for the identical citation in `crates/muxsmith-core/tests/profile_save.rs`, which now reads `defaulted fields (design D48)`. **The count 17 is NOT re-measured and NOT changed by this task**: it is a different fact with its own consumers in `profile_save.rs`'s two guard doc comments, and a task that repaired a line citation while silently moving a count would be doing two jobs under one review.

- [ ] **Step 3: the scope boundary, from the convention entry itself.** This task touches comments in tracked files **outside `docs/`**. It does NOT touch dated evidence citations in PROCESS artifacts - review verdicts, journal entries, ledger occurrence refs, tracker measurements - which legitimately cite `<file>:<line>` at a named commit, because there the moment is part of the claim. **No file under `docs/` is edited by this task**, and that is the boundary's whole content: it runs by the artifact DOING the citing, not by the artifact cited, which is why the fixture's citation INTO a design document is swept rather than exempted.

- [ ] **Step 4: verification, five checks, all outputs pasted.**
  - **Absence check A.** Expression A from Step 1 on the end state must return nothing. **Its fire is Step 1's own pre-state run, which returned exactly 1 line.** **Soundness control:** the same alternation over `docs/*.yaml` returns matches in all four house files, so the pattern hits a filename-plus-line citation when one is present. **Reachable green state:** the single matched line sits inside the comment Step 2(a) rewrites, and the replacement contains no filename followed by a colon and digits.
  - **Absence check B.** Expression B from Step 1 on the end state must return nothing. **Its fire is Step 1's own pre-state run, which returned exactly 1 line.** **Soundness control:** the same expression against `docs/superpowers/plans/2026-07-29-plan-10-pre-1.0-package.md` returns matches. **Reachable green state:** the single matched line sits inside the comment Step 2(b) rewrites, and the replacement contains no bare colon-and-digits.
  - **The blind spot both expressions share, checked rather than assumed** (`proc-sweep-surface-completeness`, `feedback-shaped duty in `a-search-whose-terms-come-from-memory-produces-a-false-absence`): both patterns require a colon, so neither can see a prose locator. Run `git ls-files | grep -v '^docs/' | grep -vE '\.(png|ico|icns|wav|snap|lock)$' | xargs grep -nEi '\blines? [0-9]+'` and classify every hit. The authoring run returned two, both test DATA in `e2e/smoke.spec.ts` (`"mkvmerge output line 1"`), neither a citation. **A hit that IS a citation returns as NEEDS_CONTEXT**, because it is a member of the class outside this task's fenced Files list.
  - **The workflow is still valid YAML.** `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` must exit 0. **This is a check of its own because no gate part parses `.github/workflows/ci.yml`**: the whole local gate stays green over a syntactically broken workflow, and the break would surface only on the next push.
  - **Comment-only, in both files.** `git diff -U0 -- .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml`, pasted in full; every added and removed line must begin, after leading whitespace, with `#`. Then the exit-bar subset: `cargo test --workspace` green, which is what proves the fixture's DATA did not move - it is consumed by `profile_save.rs`'s D48 guard 1. `git diff --stat` must name exactly two files.
  - **Test duty, weighed:** no new test. This task changes comment text in a workflow and in a test fixture and produces no user-visible consequence; the fixture's data, which does have observable consequences, is asserted unchanged by the existing `cargo test --workspace` run above rather than by a new test. The one property that IS newly observable - that the workflow still parses - gets the explicit check above rather than a test, because the repository has no test harness that loads workflow YAML and adding one would be new test infrastructure.

- [ ] **Step 5: surface, do not edit.** The report names `docs/ROADMAP.md`'s "Docs accuracy" first entry and both of the things this task makes stale in it: its "OPEN OWNER QUESTION" paragraph, which the owner answered in session 28 and whose answer is recorded in the Tier-2 statement's WIDENED clause, and its claim that exactly ONE member survives, where the derivation returns two. Both are controller close actions.

- [ ] **Step 6: commit.**

```bash
git add .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml
git -c commit.gpgsign=false commit -m "comments: locate code by symbol in the CI workflow and the D48 fixture, closing the class outside docs/" -- .github/workflows/ci.yml crates/muxsmith-core/tests/fixtures/all-non-default.yaml
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the corpus (measured with both prescribed expressions, not chosen); that the fixture's bare span is IN scope, which is settled here on the widened ruling and on Plan 10's controller ruling for the identical form; the two fenced replacements; that the symbol is derived from the citing commit's parent rather than from the cited line as it stands today; that the count 17 is neither re-measured nor changed; that no file under `docs/` and no line of code or data changes.

---

## Task A3: `raw:` compares untyped, not byte-exactly, and six sentences learn to say so (W3)

Read first: the plan brief's item 3; `docs/ROADMAP.md`'s "Docs accuracy" entry beginning '"byte-exact" overstates', including its recorded disposition that **the behaviour stays and only the wording changes**; **`crates/muxsmith-core/src/matcher.rs`**, for `scalar_eq`'s six arms, `exact_matches`'s `raw:` branch, and the two tests `b7_raw_int_float_cross_compare` and `b8_raw_language_is_byte_literal_no_normalization`; **`crates/muxsmith-core/src/profile/validate.rs`'s `raw_opt_in_diagnostic`**, whose `matches!(bare, "language" | "codec_kind")` is the measured trigger set that decides which sites are repaired and which are retained; `crates/muxsmith-core/src/capability/generated.rs` for those two properties' declared types; the v1 spec's sections 4.4, 7 (the diagnostics table) and 9.2; `README.md`'s matching-magic section; `help/en/editor-match-expr-exact.md` and `help/de/editor-match-expr-exact.md` in full, because each replacement must match its own topic's established vocabulary; Tier-2 `core-72-exact-typed-value-equality`, `core-92-raw-shape-a-rejected`, `testing-si3-run-binary`, `proc-06-mkvtoolnix-parity`, `tests-ship-with-the-feature-never-after`; the authoring section's item-3 block in full. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (section 4.4's `raw:` bullet and section 9.2's runtime paragraph - **one occurrence each; the second `byte-literal` occurrence in the 9.2 paragraph is RETAINED**, and no other line of the spec changes)
- Modify: `crates/muxsmith-core/src/matcher.rs` (the `raw:` branch's comment inside `exact_matches` - **comment text only; the `b8` test's comments and its name are RETAINED**)
- Modify: `README.md` (the `raw:` bullet in the matching-magic list - one phrase)
- Modify: `help/en/editor-match-expr-exact.md` (the "The `raw:` bypass" section's one paragraph)
- Modify: `help/de/editor-match-expr-exact.md` (the "Der `raw:`-Bypass" section's one paragraph)

**Nine assertions in seven files are deliberately NOT edited**, and the list is exhaustive so the invariance check below has something to check: `crates/muxsmith-core/src/matcher.rs` (the `b8` test's two comment lines and the test's own name), `crates/muxsmith-core/src/profile/validate.rs`, `crates/muxsmith-core/src/report/mod.rs`, `crates/muxsmith-core/tests/validate_semantics.rs`, the v1 spec's diagnostics-table row for `RawOnKnownProperty` and the second occurrence in its 9.2 paragraph, `locales/en/diagnostics.ftl` and `locales/de/diagnostics.ftl`. **Every one of them is scoped to `language`/`codec_kind`, both string-typed, so "byte-literal" is TRUE of them** - `scalar_eq`'s `_ => false` arm means no non-string scalar reaches a comparison with a string property, so no value that can match is coerced. Repairing a true sentence is not this item's disposition, and the two Fluent strings are user-visible product text besides.

**Interfaces:**
- Consumes: nothing.
- Produces: the spec wording every other site follows, which is why the spec is amended in the same task rather than ahead of it.

- [ ] **Step 1: discharge SI-3 and the code half, before writing a word.** Paste all of it.
  - `mkvmerge --version`. Read `~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json` and list the track properties declared `type: number` rather than `integer`. The authoring run found five: `max_luminance`, `min_luminance`, `projection_pose_pitch`, `projection_pose_roll`, `projection_pose_yaw`.
  - Reproduce the end-to-end demonstration and its negative control with the shipped binary, in a scratch directory outside the repository. Mux a probe from `crates/muxsmith-core/tests/fixtures/seeds/tone.wav`, confirm from `mkvmerge -J` that it reports `"audio_channels"` as an integral number, then run `muxsmith dry-run --json` twice against a one-rule profile: once with `match: { exact: { "raw:audio_channels": <the reported value>.0 } }`, which must produce a `plan.assignments` entry naming `rule_index` 0, and once with a value that does not match, which must produce `missing-track`. **Both runs are required: the matching run alone cannot distinguish a coercion from a matcher that matches everything.**
  - Paste `scalar_eq`'s six arms and the `raw:` branch of `exact_matches` from the tree, and run `cargo test -p muxsmith-core matcher` naming `b7_raw_int_float_cross_compare` from the output. **If any of this fails to reproduce, that is NEEDS_CONTEXT with the pasted result**, because the wording below is derived from it.

- [ ] **Step 2: the v1 spec, two replacements, one occurrence each.**

  (a) In section 4.4's `raw:` opt-in bullet, replace exactly

```
is matched untyped (byte-literal value equality against the property named verbatim, no `language` normalization or `codec_kind` aliasing, no false-when-absent Boolean shortcut)
```

  with exactly

```
is matched untyped (value equality against the property named verbatim, with strings compared byte-for-byte and numbers compared numerically, so `exact: { raw:x: 6 }` matches a reported `6.0`; no `language` normalization or `codec_kind` aliasing, no false-when-absent Boolean shortcut)
```

  (b) In section 9.2's runtime paragraph, replace exactly

```
and is matched untyped: byte-literal value equality against the property named verbatim, with no `language` normalization, no `codec_kind` alias expansion, and no false-when-absent Boolean shortcut.
```

  with exactly

```
and is matched untyped: value equality against the property named verbatim, with strings compared byte-for-byte and numbers compared numerically (`raw:x: 6` matches a reported `6.0`), no `language` normalization, no `codec_kind` alias expansion, and no false-when-absent Boolean shortcut.
```

  **The later sentence in that same paragraph, which ends "that `raw:` degrades to byte-literal equality", is NOT touched.** It is scoped to `language` and `codec_kind` and is true of them, and after (a) and (b) it reads as the specialization it is rather than as a second mechanism - which is what keeps this amendment out of a self-contradiction. Section 7's diagnostics-table row for `RawOnKnownProperty` is not touched either, for the same reason.

- [ ] **Step 3: `crates/muxsmith-core/src/matcher.rs`, comment text only.** Replace exactly

```
        // `raw:` opt-in (D32, spec 9.2): untyped byte-literal value equality
        // against the property named verbatim. It bypasses the `language`
```

  with exactly

```
        // `raw:` opt-in (D32, spec 9.2): untyped value equality against the
        // property named verbatim - `scalar_eq` compares strings
        // byte-for-byte but numbers numerically, so an `Int` scalar matches
        // a reported `Float` of the same value (case B-7). It bypasses the
        // `language`
```

  The remaining lines of that comment are unchanged. **Not one line of code changes in this file**, and the `b8` test keeps its name and its two comment lines: its subject is `raw:language`, a string property, where byte-literal is exactly right.

- [ ] **Step 4: `README.md`, one phrase, in the README's own register.** Replace exactly

```
byte-exact value equality against that one field, named verbatim.
```

  with exactly

```
plain value equality against that one field, named verbatim - the same plain comparison as everywhere else, so `6` still matches a reported `6.0`.
```

  **Register: the README is written in the owner's sell-tone**, a case-scoped exception to the house writeup voice recorded on its ROADMAP entry, and the replacement reuses the paragraph's own established phrase "a plain value comparison" rather than introducing a new one. The bullet's frame - "every convenience above switches off" - stays, because "above" scopes to the three bullets that precede it and numeric comparison is not one of them; only the false claim inside it moves.

- [ ] **Step 5: the two help topics, one paragraph each, both in the same commit.**

  (a) In `help/en/editor-match-expr-exact.md`, replace exactly

```
no absent-means-false shortcut - plain byte-for-byte value equality against the property named verbatim.
```

  with exactly

```
no absent-means-false shortcut - plain value equality against the property named verbatim, with strings compared byte-for-byte and numbers still compared numerically, so `6` equals a reported `6.0`.
```

  (b) In `help/de/editor-match-expr-exact.md`, replace exactly

```
kein Fehlend-heißt-false - nur byte-genaue Wertgleichheit gegen die wörtlich benannte Eigenschaft.
```

  with exactly

```
kein Fehlend-heißt-false - nur einfache Wertgleichheit gegen die wörtlich benannte Eigenschaft, wobei Zeichenketten byte-genau und Zahlen weiterhin numerisch verglichen werden, `6` also gleich einem gemeldeten `6.0` ist.
```

  **The German is not a fresh translation: it reuses the same topic's own "Zahlen vergleichen numerisch" vocabulary from its Typed-equality section**, so the two locales stay in one voice. Both replacements are prose with code spans only - **no pipe character, no external URL, no raw HTML** - because `pnpm check:i18n` hard-fails on all three over `help/`.

- [ ] **Step 6: verification, five checks, all outputs pasted.**
  - **Absence check R, the repair set.** The expression, with its alternation derived by reading each of the six sites rather than typed from memory:

```bash
git grep -nE 'byte-literal value equality|byte-exact value equality|byte-for-byte value equality|byte-genaue Wertgleichheit' -- . \
  ':!docs/process-journal*' ':!docs/superpowers/plans' \
  ':!docs/superpowers/specs/2026-07-11*' ':!docs/superpowers/specs/2026-07-21*' ':!docs/superpowers/specs/2026-07-28*' \
  ':!docs/decision-ledger.yaml' ':!docs/conventions.yaml' ':!docs/process-conventions.yaml' ':!docs/ROADMAP.md'
```

    **RED, run FIRST on the pre-state: exactly 6 lines across 5 files** - `README.md`, `crates/muxsmith-core/src/matcher.rs`, the v1 spec twice, `help/de/editor-match-expr-exact.md`, `help/en/editor-match-expr-exact.md`. **GREEN on the end state: 0.** **Reachable green state, argued member by member:** each of the six lies inside one of the five fenced replacements above, and no replacement text contains any of the four alternatives. **Soundness control for the alternation itself:** run the same expression WITHOUT the `':!docs/superpowers/specs/2026-07-11*'` exclusion, which must return the plan-5.5 design document's B-4 row, proving the pattern still matches a live occurrence of one alternative when the surface admits it.
  - **Invariance check K, the retained set.**

```bash
git grep -nE 'byte-literal untyped equality|to byte-literal equality|matches byte-literally|byte-literal ab\.|byte-literally compares|Byte-literal against' -- . \
  ':!docs/process-journal*' ':!docs/superpowers/plans' \
  ':!docs/superpowers/specs/2026-07-11*' ':!docs/superpowers/specs/2026-07-21*' ':!docs/superpowers/specs/2026-07-28*' \
  ':!docs/decision-ledger.yaml' ':!docs/conventions.yaml' ':!docs/process-conventions.yaml' ':!docs/ROADMAP.md'
```

    **Exactly 9 lines across 7 files on the pre-state AND on the end state**, matching the deliberately-not-edited list above member for member. **Its fire, because an invariance check that never moves cannot be told apart from a broken one:** delete one retained site - the `matches byte-literally` clause in `locales/en/diagnostics.ftl` - run again, watch the count fall to 8, and **restore**; paste both runs. The restore is proven by the diff check below.
  - **The gate parts that read these files.** `cargo fmt --all --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test -p muxsmith-core`, with `b7_raw_int_float_cross_compare` and `b8_raw_language_is_byte_literal_no_normalization` named from the pasted output; `pnpm check:i18n`, which is what machine-checks the two help edits. All green, foreground.
  - **Diff scope.** `git diff --stat` must name exactly the five files in the Files list. `git diff -U0 -- crates/muxsmith-core/src/matcher.rs`, pasted in full: every changed line must be a comment line, and none may be inside the `tests` module. `git diff --exit-code -- locales/` must exit 0, which is where the K-check's fire would show if it had not been restored.
  - **Test duty, weighed, with the reason measured rather than asserted:** this task ships **no new test**, because it changes no behaviour and the behaviour its wording describes is **already covered** - `b7_raw_int_float_cross_compare` asserts the exact Int-against-Float coercion the new wording states, and it existed before this plan. The claim that makes the new test unnecessary is therefore run rather than weighed: the test is named from a pasted green run in this step, and the plan-5.5 design document's case table records B-7 as an intended case. A new test would duplicate a passing one.

- [ ] **Step 7: surface, do not edit.** The report names, without editing:
  - The nine retained assertions, as a harmonization candidate for the controller: they are true as written, and whether the house wants one vocabulary for `raw:` across scoped and unscoped statements is a decision above this task, with two of the nine being user-visible Fluent strings in two locales.
  - `docs/decision-ledger.yaml` and `docs/ROADMAP.md` each carry a statement of the same `raw:` fact in the controller's own files; the plan may not edit them.
  - **A measured observation, pre-existing and not this plan's to change:** `codec_kind` is absent from mkvmerge's own identification schema v20 track properties (59 properties, `codec_kind` not among them), so `raw:codec_kind` reads a property mkvmerge never reports and can never match a track, while `RawOnKnownProperty` still warns about it. Recorded because it was found while measuring this item's trigger set, not because this task touches it.

- [ ] **Step 8: commit.**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md crates/muxsmith-core/src/matcher.rs README.md help/en/editor-match-expr-exact.md help/de/editor-match-expr-exact.md
git -c commit.gpgsign=false commit -m "spec+docs: raw: is untyped value equality, not byte-exact - numbers still compare numerically" -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md crates/muxsmith-core/src/matcher.rs README.md help/en/editor-match-expr-exact.md help/de/editor-match-expr-exact.md
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** the six repair sites and the nine retained ones, nor the rule that splits them (an unscoped claim is false and moves; a claim scoped to `language`/`codec_kind` is true and stays); every fenced replacement, in both natural languages; that no behaviour changes and no `deny.toml`-style configuration is touched; that no new test is written, on the measured ground that `b7_raw_int_float_cross_compare` already covers the behaviour; that the two Fluent catalogs are not edited. **A measurement that reveals a real ordering or coercion DEFECT rather than a wording defect is a finding, not a wording to soften: NEEDS_CONTEXT.**

---

## Task A4: the v1 spec's section 8.1 states the shipped CLI surface (W4)

Read first: the plan brief's item 4; `docs/ROADMAP.md`'s "Docs accuracy" entry beginning "The v1 spec's section 8.1 synopsis"; **the v1 spec's section 8 in full**, because the sweep in Step 4 runs over it and because the fenced block replaces part of it; **`README.md`'s "Using the CLI" section**, whose post-Plan-10 text is the consistency target and which this task does NOT edit; **`crates/muxsmith-cli/src/cli.rs`**, for the `Cli` doc comment's exit-code contract and the `Cmd` subcommand list; **`crates/muxsmith-cli/src/commands/run.rs`**, for the `ctrlc` registration and the two sites that produce 130; **`crates/muxsmith-cli/src/commands/mod.rs`**, for `severity_exit`; `docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md`'s D16, the decision the new bullet cites; Tier-2 `proc-04-spec-wins`, `code-comment-line-citations-drift`; the authoring section's item-4 block in full. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (section 8.1's fenced synopsis block and its exit-code bullet - nothing else in the file, and in particular not the sections Task A3 amended)

**Interfaces:**
- Consumes: Task A3's committed spec, so Step 4's self-contradiction sweep runs over the final text rather than over a text about to change.
- Produces: a spec 8.1 that `crates/muxsmith-cli/src/cli.rs`'s "(spec 8.1, D16)" citation actually lands on.

- [ ] **Step 1: re-derive the surface from the shipped binary, not from the source or from this plan.** Build or locate `target/debug/muxsmith`; confirm it is not stale with `find crates src-tauri -name '*.rs' -newer target/debug/muxsmith`, which must print nothing (rebuild if it does not). Capture `muxsmith --help`, `muxsmith --version`, and `<sub> --help` for **every** subcommand `--help` lists. Paste all of it, and build the divergence table (spec says / binary says / verdict) at FLAG granularity including value names and possible-value sets. **Every correction below was measured at plan-authoring and must be REPRODUCED here before it is written; a correction the re-derivation does not reproduce is a finding, not a silent drop.** Exit codes are not in the help text: derive them from `cli.rs`'s `Cli` doc comment, `severity_exit` in `commands/mod.rs`, and `job_exit_code` plus the two literal `130` sites in `commands/run.rs`, and name each symbol with its file in the report.

- [ ] **Step 2: replace the synopsis block.** Replace exactly

```
muxsmith validate <profile>
muxsmith dry-run  <profile> [--source DIR] [--output DIR] [--json]
muxsmith run      <profile> [--source DIR] [--output DIR] [--jobs N] [--fail-fast] [--json]
muxsmith identify <file> [--json]
muxsmith schema                      # print the profile JSON Schema
```

  with exactly

```
muxsmith validate <profile> [--json] [--locale LOCALE]
muxsmith dry-run  <profile> [--source DIR] [--output DIR]
                            [--on-collision POLICY] [--json] [--locale LOCALE]
muxsmith run      <profile> [--source DIR] [--output DIR]
                            [--on-collision POLICY] [--jobs N] [--fail-fast]
                            [--json] [--locale LOCALE]
muxsmith identify <file> [--json] [--locale LOCALE]
muxsmith schema                      # print the profile JSON Schema
```

  Three fixed properties of this block. **The flags are enumerated per subcommand rather than summarized in a bullet**, because a blanket "every subcommand takes `--json`" is precisely the false claim Plan 10 removed from the README, and `muxsmith schema` is the counterexample that makes it false. **The order inside each line is the binary's own option order**, which is what makes the block re-derivable from `--help` by the next reader. **The continuation indent aligns under `<profile>`**, the standard synopsis wrap, so no line exceeds 80 characters. The `schema` line is byte-identical to the original, including its comment and its column alignment, because it was already correct.

- [ ] **Step 3: replace the exit-code bullet.** Replace exactly

```
- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors.
```

  with exactly

```
- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors, plus 130 for a
  cancelled batch (D16). Only `run` can reach 130: it is the one subcommand
  that installs a SIGINT handler, so an interrupted `validate`, `dry-run`,
  `identify` or `schema` dies by signal and the shell reports 130 by its own
  128-plus-signal convention rather than by anything Muxsmith returns.
```

  **Why this bullet is in this task rather than in a vehicle of its own**, stated because the tracker entry names only the synopsis: `130` appears zero times in the whole v1 spec while `crates/muxsmith-cli/src/cli.rs` cites "spec 8.1" for it and `README.md`, which the spec outranks, states it after Plan 10. That is the same defect direction as the synopsis - the authoritative document underclaiming a shipped surface - inside the same fenced-block-plus-bullets region, and the spec-amendment self-contradiction sweep this item owes is what would find it anyway. **The mechanism sentence is measured, not inferred:** `grep -rn "ctrlc" crates/muxsmith-cli/src/` returns hits only in `commands/run.rs`, whose own comment records "this is the one registration in the process".

- [ ] **Step 4: the spec self-contradiction sweep, run as an enumeration rather than as a reading.** A spec amendment sweeps the spec for self-contradictions before commit (doctrine section 1), and this item is itself one such contradiction. Paste every command and its full output, and give every hit a verdict.
  - Every other exit-code sentence: `grep -nE 'xit cod|SIGINT|Ctrl' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`. The authoring run returned two lines: section 6's mkvmerge-job exit codes (a different subject - the child process's code, not Muxsmith's) and the 8.1 bullet this task replaces. **A third hit is a finding.**
  - Every other statement of a CLI flag or subcommand surface: `grep -nE -- '--[a-z][a-z-]+' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, classified per hit as consistent with the amended block or as a finding. **Fired control for that expression: it must return the amended 8.1 block's own lines**, so an empty or short result is visibly wrong rather than quietly clean.
  - `grep -c '130' docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` must now be non-zero; the authoring run on the pre-state returned `0`, which is this check's red state.
  - A cross-document consistency read against `README.md`'s "Using the CLI" section, reported rather than acted on: the README is already correct on the flag surface and on 130, so **this task edits no file but the spec**. A divergence found here is a finding.

- [ ] **Step 5: verification.**
  - The four Step-4 checks above, all pasted.
  - **A no-permanent-checker decision, recorded rather than left implicit:** this task proposes no gate check comparing the spec's synopsis against `--help`. The claim that makes it unnecessary is a recorded house decision, not an argument invented here, and it is citable rather than weighed: the one instrument of that shape this project built - the reach-claim checker over `docs/INSTALL.md` prose - was **deliberately not promoted** into `scripts/ledger-lint.py`, on the reviewer's recommendation and the controller's agreement, because it parses PROSE, which `proc-check-green-state-reachable` names as the way such a check becomes permanently red on correct content. The record is the ROADMAP's "Reach-claim checker" section. **If the implementer's reading of that section does not support this, that is a finding: NEEDS_CONTEXT**, not a checker built at the keyboard.
  - Exit-bar subset: `python3 scripts/ledger-lint.py` green (the spec is not one of its four files, so this proves only that nothing else broke) and `git diff --stat` naming exactly one file. No Rust or frontend part can observe a spec edit, and saying so is the honest form rather than running them for appearances - the stream's full gate run covers the tree.
  - **Test duty, weighed:** no new test. This task changes a specification document and no behaviour; the surface it documents already ships and is exercised by the existing `cli_validate` and `dry_run_cli` suites. The one thing worth locking - that the spec keeps matching the binary - is exactly the prose-parsing checker the bullet above declines on a recorded ground.

- [ ] **Step 6: surface, do not edit.** The report names `crates/muxsmith-cli/src/cli.rs`'s `Cli` doc comment, "every command shares the exit-code contract 0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)". **It is hard-wrapped across two `///` lines, so it does not grep as one string** (`proc-wrapped-prose-quote-grep`): locate it by `shares the exit-code contract` on its own line, not by the whole sentence. **This task makes its citation land** - spec 8.1 now carries 130 - but the measurement also shows "every command" to be over-broad as to who can PRODUCE 130, since only `run` installs the handler. **It is not edited here:** it is a source comment about a different half of the fact, its rewording needs a decision about how to describe the signal-death case that the owner has not seen, and Plan 10's precedent for an adjacent claim a task did not falsify is to surface it. Vehicle in the deferred-by-decision note.

- [ ] **Step 7: commit.**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "spec: 8.1 states the shipped CLI flag surface and the 130 cancellation code" -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

**Must not decide:** that the surface is re-derived from the binary rather than from the source or from this plan; the two fenced replacements, including the continuation-indent form and the per-subcommand enumeration; that the exit-code bullet is IN this task; that the sweep is an enumeration with a fired control rather than a reading; that no permanent checker is built, on the recorded ground cited in Step 5; that `README.md` and `cli.rs` are surfaced rather than edited.

---

## Stream B worktree

The controller creates it before dispatching B1, concurrently with stream A's:

```bash
git worktree add -b plan-11-stream-b ../muxsmith-plan11-b master
```

---

## Task B1: the two open dependency alerts - one bump, one measurement, one investigation (W1)

Read first: the plan brief's item 1 in full; **`docs/ROADMAP.md`'s "TWO OPEN VULNERABILITY ALERTS" entry in the Pre-1.0 release gates section, in full**, including its exposure analysis and its RULED block, which is the owner's ruling on the vehicle and on the three parts; the "Dependabot/Renovate activation" entry in the same section for the two riders (the `deny.toml` RUSTSEC pruning and the held TypeScript bump) and for why the trigger has not fired; **`deny.toml` in full**, because Step 5 reads its effective configuration and because Step 7 proves it unchanged; **`BUILDING.md`'s Rust gate block**, because `cargo deny check` is one of its commands and this task must not change the invocation; `package.json` and `pnpm-lock.yaml`, because parts of Step 2 are assertions about them; Tier-2 `ci-04-dependabot-cadence`, `ci-10-pin-everything`, `gate-includes-cross-target-lint-for-the-unrun-os`, `proc-07-verify-against-source`, `proc-no-work-needed-check`; the authoring section's stream-B blocks in full. Model tier: mid.

**Files (EXHAUSTIVE):**
- Modify: `pnpm-lock.yaml` (part a; the resolution of `postcss` and of its own dependency `nanoid`, written by `pnpm update`, never by hand)

**No other file is written.** In particular: `package.json` is NOT edited and gains no `pnpm.overrides` entry and no direct dependency; `deny.toml` is NOT edited; the `cargo deny` invocation in `BUILDING.md` and in `.github/workflows/ci.yml` is NOT edited; `Cargo.lock` is NOT edited. Parts (b) and (c) produce a report, not a diff, and that is their completion.

**Interfaces:**
- Consumes: nothing.
- Produces: the lockfile the merged state's gate runs against, and the measurement whose disposition belongs to the controller and, where it would move gate coverage, to the owner.

- [ ] **Step 1: record the starting state.** Paste: `pnpm --version` and `node --version`; `git rev-parse HEAD` for this worktree's base; `grep -nE '^ *postcss@|postcss: ' pnpm-lock.yaml`; `pnpm why postcss`; `cargo deny --version`; `grep -n -A2 '^name = "glib"' Cargo.lock`. Then `pnpm install --frozen-lockfile`, so the worktree has `node_modules` and so the pre-state lockfile is proven self-consistent before anything moves.

- [ ] **Step 2: part (a), bump `postcss` through the lockfile.** The prescribed mechanism, and the reason it is this one:

```bash
pnpm update postcss
```

  **This is a lockfile-level update of a transitive dependency, which is what the owner's ruling calls for** - the tracker's words are "through the lockfile ... a lockfile decision rather than a pinned-dependency one". It is **not** a `pnpm.overrides` entry in `package.json` and **not** a direct dependency addition. Reason to state in the report: this project's toolchain doctrine pins direct dependencies exactly (`ci-10-pin-everything`), and adding an override for a build-time transitive package changes what the manifest asserts about the dependency graph. `pnpm update`'s `--depth` default is `Infinity`, which is what reaches a transitive dependency; no `--latest` is used, because both parents' declared ranges already admit the patched version and `--latest` would ignore ranges the project has not been asked to ignore.

  Then verify, pasting each result:
  - `pnpm why postcss` and `grep -nE '^ *postcss@|postcss: ' pnpm-lock.yaml`. **The requirement is `>= 8.5.18`**, which is the alert's own `first_patched_version`. **The landing version is not fenced in this plan and must not be**: the authoring probe landed on `8.5.24` while the registry's `latest` read `8.5.25`, so a fenced number would be a fresh wrong number. Paste what you observe.
  - `git diff --exit-code -- package.json` must exit 0, and `grep -c '"overrides"' package.json` must return 0. **These are separate assertions from the lockfile one**: the whole mechanism ruling is that the lockfile moves and the manifest does not, and a report that only showed the new version would have evidenced one side of it. **And the `--exit-code` instrument is fired on the same tree at no cost**: run `git diff --exit-code -- pnpm-lock.yaml` too and paste its non-zero exit beside `package.json`'s zero. An `--exit-code` check that reports no change looks identical whether nothing changed or the path was misspelled, so the pair is what makes the zero mean something.
  - `git diff -- pnpm-lock.yaml`, pasted in full, with every changed package named. The authoring probe produced eight hunks covering exactly two packages: `postcss` and `nanoid` (3.3.15 -> 3.3.16), the latter being postcss's own dependency. **A third package moving is a finding to report, not a pass**; a package moving that is not reachable from postcss returns as NEEDS_CONTEXT.
  - `pnpm install --frozen-lockfile` must succeed on the new lockfile, which is what CI does and therefore what proves the lockfile is internally consistent.

  **The genuine fork, and its route.** If a transitive parent constrains `postcss` so that a lockfile update cannot move it past 8.5.17, **that returns as NEEDS_CONTEXT with the options and their costs** - it is a real decision about what the manifest asserts, and it belongs to the controller. **There is deliberately no pre-authorised fallback here**: a pre-authorised override would be a sanctioned fork, which the latitude ban exists to prevent. At authoring the fork does not exist: `@vue/compiler-sfc@3.5.39` declares `^8.5.15` and `vite@8.1.4` declares `^8.5.16`, both caret ranges over `8.5.x`, and the scratch probe moved the resolution with `package.json` byte-identical. The NEEDS_CONTEXT route stays in the plan anyway, because the tree at execution time is not the tree at authoring.

- [ ] **Step 3: prove the bump did not break the frontend, four named commands.** Run each foreground and paste each result: `pnpm lint`; `pnpm build`; `pnpm check:i18n`; `pnpm test:e2e`. **They are named individually rather than as "the frontend gate" because a transitive lockfile bump can move exactly one of them**, and this is the reason stream B has its own worktree at all. A red result here is a finding with its pasted output, not a lockfile to revert silently.

- [ ] **Step 4: part (b) begins - establish the advisory's own class, at the source.** Paste all of it.
  - The two open alerts from GitHub, at the source: `gh api repos/senolfeldmann/Muxsmith/dependabot/alerts` filtered to `state == "open"`, with each alert's package, severity, GHSA id, vulnerable range, first patched version and manifest path.
  - The RustSec record for the Rust advisory, read from the local advisory database that `cargo deny` itself uses (`~/.cargo/advisory-dbs/advisory-db-*/crates/glib/`): the advisory file's path, its `id`, its `informational` field, its `aliases` field, its `patched` range and its affected functions.
  - **The identity claim, which the report must make explicitly:** the advisory's `aliases` entry beside the alert's `ghsa` field, so it is on the record that GitHub and RustSec are reporting **one** advisory and that the disagreement is therefore not a database-coverage gap. At authoring both read `GHSA-wrw7-89jp-8q8g`.

- [ ] **Step 5: part (b) continued - establish this project's effective handling of that class, at the tool's source.** This is the half the hypothesis on record does not answer, and **restating the hypothesis is not a completion.** Paste all of it.
  - What `deny.toml`'s `[advisories]` section actually sets, quoted, and the count of its `RUSTSEC-` ignore entries (`grep -cE '^\s*"RUSTSEC-' deny.toml`; 18 at authoring). Note explicitly that no glib id is among them, so this is not a silenced advisory.
  - **Which configuration keys `cargo-deny 0.19.9` accepts for informational advisory classes, read at the tool's own current documentation or source and cited with what was read** - not from memory, and not from an older version's documentation. Then: which of them this `deny.toml` sets, and what the effective behaviour for `informational = "unsound"` therefore is.
  - **The measured observation this mechanism has to explain**, reproduced here: `cargo deny -L info check advisories` mentions `RUSTSEC` on 54 lines at authoring and mentions `RUSTSEC-2024-0429` on **zero**, while each of the 18 ignored ids produces a `note[advisory-ignored]`. So the advisory is not an error, not a warning, and not an ignored note. **A mechanism account that would predict an `advisory-ignored` note, or a warning, does not fit this observation and is a finding.**

- [ ] **Step 6: part (b) concluded - one demonstration that reproduces both facts together.** A single pasted transcript, in this order, so a reader sees the contradiction rather than being told about it:
  1. `cargo deny check advisories` and its exit code - green at authoring.
  2. `cargo deny check advisories --show-stats` - `advisories ok: 0 errors, 0 warnings, 36 notes` at authoring.
  3. `cargo deny -L info check advisories 2>&1 | grep -c 'RUSTSEC-2024-0429'` - `0` at authoring.
  4. **The fired control for that zero**, without which it is indistinguishable from a broken pattern: `cargo deny -L info check advisories 2>&1 | grep -c 'RUSTSEC'` - `54` at authoring - plus one `grep -c` for an id that IS present, which must return non-zero.
  5. The advisory file's existence and its id, proving the database `cargo deny` reads does carry it.
  6. `grep -n -A2 '^name = "glib"' Cargo.lock`, proving the affected version is in the tree.

  **Two constraints on this part, both absolute.** **The task changes neither `deny.toml` nor the `cargo deny` invocation.** Making the check fail on that class would interact with the 18 commented RUSTSEC ignores already in `deny.toml` and would change what a gate part covers, which is a separate decision with its own owner-visible surface; the measurement's disposition belongs to the controller and, where it moves gate coverage, to the owner. **And the output is the measurement, not the hypothesis.** The hypothesis on record - that RustSec classes unsoundness as `informational` and this configuration does not fail on that class - is half confirmed and half refined by the observation in Step 5, and the report states what was measured and how, at the source, rather than restating the hypothesis as a conclusion.

- [ ] **Step 7: prove the untouched things untouched, per file, with the instrument fired.** `git hash-object deny.toml` compared with `git rev-parse <this worktree's base>:deny.toml`; the same for `BUILDING.md` and `.github/workflows/ci.yml`; then `git diff --exit-code -- deny.toml BUILDING.md .github/workflows/ci.yml Cargo.lock package.json`, which must exit 0. **Per file against the named base commit's blobs rather than by a clean `git status`**: in a repository where a second writer may commit concurrently, a clean status proves only that nothing is uncommitted right now. **Fired, using the one file that DID move:** run the same blob comparison and the same `--exit-code` command for `pnpm-lock.yaml` and paste the mismatch and the non-zero exit beside the matches above. Both instruments must be shown to discriminate on this tree; a set of agreeing comparisons proves nothing on its own, because a misspelled path, a wrong base revision or a pathspec that matches nothing produces exactly the same output as a clean file.

- [ ] **Step 8: part (c), `glib` - investigate only, no fix.** Paste all of it: `cargo tree -i glib@0.18.5 -e normal` in full, `cargo tree -i glib@0.18.5 -e normal --depth 1` for the direct-parent set, `grep -c '^name = "glib"' Cargo.lock`, and the version of every gtk-rs family crate in the lock. Then state the verdict in the brief's own terms.

  **The expected and acceptable completion, stated so nobody stretches this into a Tauri upgrade:** at authoring, glib 0.18.5 has twelve direct parents, every one of them a gtk-rs 0.18-generation binding, and nothing 0.20+ exists in the tree; the whole family bottoms out at `tauri 2.11.5` through `muda`, `tao`, `tauri-runtime`, `tauri-runtime-wry`, `webkit2gtk` and `wry`, which is the GTK3 generation `deny.toml`'s own comment records as archived upstream in favour of gtk4-rs. **If the measurement confirms that glib cannot move independently, the finding IS the result** - "this is an upgrade project, not a bump" - and it earns its own vehicle rather than being forced into this task. **`Cargo.lock` is not edited, no `[patch]` section is added, and no Tauri version is changed**, whatever the measurement shows. `git diff --exit-code -- Cargo.lock` (Step 7) is the evidence.

- [ ] **Step 9: verification.**
  - **The full gate as `BUILDING.md` enumerates it, foreground, green, in this worktree**, before the commit. This is the one task in the plan whose own change can move a gate part for reasons outside its diff, so its subset is the whole gate rather than a chosen part of it.
  - `git diff --stat` must name exactly one file, `pnpm-lock.yaml`.
  - **Test duty, weighed:** this task ships no new test. Part (a) changes a dependency resolution and no behaviour of Muxsmith's own; its user-visible consequence is that the build still works, and that is what the four named frontend commands in Step 3 and the full gate here assert, using the existing infrastructure rather than a new scenario. Parts (b) and (c) produce measurements, and a measurement's artifact is its pasted transcript, not a test. **What is deliberately NOT deferred here:** no scenario the existing harness could express is left for a later item, because there is no new behaviour to express.

- [ ] **Step 10: commit.**

```bash
git add pnpm-lock.yaml
git -c commit.gpgsign=false commit -m "deps: move postcss past the patched version through the lockfile (GHSA-r28c-9q8g-f849)" -- pnpm-lock.yaml
```

(Trailer per SI-4, derived from this dispatch's model parameter.)

- [ ] **Step 11: surface, do not edit.** The report names, without editing: the ROADMAP's "TWO OPEN VULNERABILITY ALERTS" entry, whose three parts this task discharges and whose disposition is a controller close action; that the `glib` alert stays OPEN on GitHub and is not dismissed by this task, because dismissing an alert is an owner action; and whatever part (b)'s measurement establishes about whether gate part `cargo deny check` has a hole, which is the controller's to route and, where it would change gate coverage, the owner's to decide.

**Must not decide:** the mechanism for part (a) (lockfile update, not an override, not a direct dependency); the landing version (measured, not fenced; the requirement is `>= 8.5.18`); that a blocked lockfile update returns as NEEDS_CONTEXT with no pre-authorised fallback; that `deny.toml` and the `cargo deny` invocation stay untouched; that part (b)'s deliverable is a measurement at the source rather than the hypothesis restated; that part (c) is investigation only and that "this is an upgrade project, not a bump" is an acceptable completion; that `Cargo.lock` and `package.json` are unchanged.

---

## Plan close (controller actions, not tasks)

- **Entry condition:** stream A's four tasks and stream B's one task committed on their branches, **each stream's own full gate green in its own worktree**, both branches merged into `master` in the order A then B, and **the full gate as `BUILDING.md` enumerates it green, foreground, no subsets, on each merged state** (`proc-08-parallel-worktrees`). A merge needing a manual edit in a product file is dispatched, never resolved controller-side. Then the single push (SI-4; `gh-log.md` entry) and the push-triggered CI run green on the head SHA, including the `ledger-lint` job and the `cargo deny` job.
- **Worktree teardown** after both merges: `git worktree remove` for both, and `git worktree list` verified to show only the main tree.
- **Whole-branch review** by an independent reviewer on the **top** tier (`proc-03-model-assignment`), against this plan, the plan brief, the ROADMAP entries the brief cites and the spec, before any further close action.
- **Verdict-harvest mining** into the ledger at each verdict's arrival, not at the close (`proc-01-sdd`'s verdict-arrival gate); the close only audits that none was missed.
- **Blocked-pool sweep** per the standing duty, over every `status: blocked` entry in all four house-knowledge files.
- **ROADMAP dispositions**, one per item this package closes: the "TWO OPEN VULNERABILITY ALERTS" entry (its three parts discharge against B1's commit and report; the entry records which parts produced a fix and which produced a finding, and it does NOT claim the `glib` alert is resolved); the "Docs accuracy" stale-`ci.yml`-citation entry (**with its stale "OPEN OWNER QUESTION" paragraph corrected and its one-surviving-member claim corrected to two**); the '"byte-exact" overstates' entry (**recording the derived set as six repaired and nine retained, against the entry's own "three places"**); the 8.1-synopsis entry (**recording that four of five synopsis lines and the exit-code bullet moved, against the entry's `validate`-only framing**); and the "A neighbouring class" paragraph in the "Gate-count derivation has no check" section (**all three ordinals and the 86-character line closed; its own enumeration of them is now history**).
- **Surfaced for the controller at plan-authoring, each already carrying its measurement in the authoring section**, so none of them waits on a task report: (1) `docs/process-conventions.yaml`'s `gate-includes-cross-target-lint-for-the-unrun-os` says "documented as gate part 6 in `BUILDING.md`", which Task A1 falsifies; (2) `docs/process-conventions.yaml`'s `a-document-never-cites-a-line-number-inside-itself` says `comments-locate-by-symbol-never-by-line-number` "stays scoped to SOURCE comments", which the later session-28 widening makes stale as a standalone read - the two rulings are compatible in substance, but the sentence misleads a reader who opens only that entry; (3) the nine retained `byte-literal` assertions as a vocabulary-harmonization question, two of them user-visible Fluent strings in two locales; (4) `crates/muxsmith-cli/src/cli.rs`'s "every command shares the exit-code contract 0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)", measured over-broad as to who can produce 130; (5) `.github/workflows/ci.yml`'s "the ninth gate part" as a measured NON-defect, recorded so a later sweep does not renumber a dated record; (6) the tracker's ordinal expression `part [0-9]|parts [0-9]` cannot match a spelled ordinal, an instrument gap that let one live site sit outside the enumeration; (7) `raw:codec_kind` can never match a track, since `codec_kind` is absent from mkvmerge's identification schema v20 while `RawOnKnownProperty` still warns about it.
- **SDD salvage** of `.superpowers/sdd/plan-11/` per the standing salvage rule, run **after every close action has landed**, with the `diff -r` re-check against the salvaged copy and the salvaged file COUNT verified in the commit rather than asserted; a non-empty diff is the re-salvage trigger.
- **Journal entry and HANDOFF snapshot** per the standing duty, both publication-grade.
- **The completion statement this close is allowed to make, and the one it is not.** This package's completion is **not** 1.0 completeness. `owner-manual-qa-gates-the-1-0-release` binds: no completeness claim about 1.0 may be made until the owner's manual QA and bug-hunting pass has run and produced its findings, however short the ROADMAP's open list looks afterwards, and its round-3 findings are already open with one of them explicitly awaiting his ruling on a product boundary. The close records the package as executed and closed; it does not propose the tag. **And the class-closure claim carries its surface or it is false:** Task A2 closes the line-number-citation class over every tracked file outside `docs/`, where process artifacts legitimately keep citing a line at a named commit - stating it without that qualifier would repeat exactly the over-claim the ROADMAP entry was written to prevent.

## Deferred by decision

Every deferral this plan utters, each with a concrete vehicle. "Later" and "a cleanup pass" are not vehicles.

| Deferred | Why | Vehicle |
|---|---|---|
| The nine retained `byte-literal` assertions keep their wording | Each is scoped to `language`/`codec_kind`, both string-typed, so the claim is true of every value that can reach the comparison; two of the nine are user-visible Fluent strings in two locales, whose change is an owner-visible surface rather than a documentation repair | An owner decision on whether the house wants one `raw:` comparison vocabulary across scoped and unscoped statements, routed from the plan-close surfacing list. Until that ruling exists this has no task, deliberately, because picking a vehicle first would pick the answer by implication |
| `crates/muxsmith-cli/src/cli.rs`'s "every command shares the exit-code contract 0 clean / 1 warnings / 2 errors / 130 cancelled (spec 8.1, D16)" keeps its wording | The citation half is repaired by Task A4 (spec 8.1 now carries 130). The remaining defect is a different fact - which subcommands can PRODUCE 130 - whose repair needs a decision about how to describe signal death that the owner has not seen | Whichever package next edits `crates/muxsmith-cli/src/cli.rs`, carrying the measurement from A4 Step 6 |
| `raw:codec_kind` can never match a track while `RawOnKnownProperty` still warns about it | Found while measuring Task A3's trigger set. It is a product-behaviour question - whether a diagnostic should say more than "this bypasses the semantics" about a property that is not reported at all - not a wording repair | An owner decision on the diagnostic's content, routed from the plan-close surfacing list; a controller-side ledger entry records the measurement in the meantime |
| `glib` stays at 0.18.5 and its alert stays open | Twelve direct parents, all gtk-rs 0.18-generation bindings reached through Tauri 2's Linux backend, which has not migrated off GTK3. Moving glib means moving the family, which is an upgrade project | Its own vehicle, opened from B1's part-(c) finding. Not this plan, and explicitly not B1: the ruling that created B1 says so |
| The `cargo deny` gap's REPAIR, if part (b) finds one | The measurement's disposition belongs to the controller and, where it would change what a gate part covers, to the owner: any change here interacts with the 18 commented RUSTSEC ignores in `deny.toml` | Routed from B1's part-(b) report at the verdict-arrival gate; a gate-coverage change goes to the owner as an escalation with its steelman |
| No permanent checker compares the spec's 8.1 synopsis against `--help` | It would parse prose, which `proc-check-green-state-reachable` names as the way such a check becomes permanently red on correct content; the one instrument of that shape this project built was deliberately not promoted for exactly that reason | The ROADMAP's "Reach-claim checker" candidate section, which already holds this question for its whole class. Reconsider there, not here |
| The count "17 defaulted fields" in `crates/muxsmith-core/tests/fixtures/all-non-default.yaml` and in `profile_save.rs`'s two guard doc comments is not re-measured | A different fact from the line citation Task A2 repairs, with its own consumers; moving it inside a citation repair would put two jobs under one review | Whichever package next owns the D48 guards, or the controller's normative-count sweep, whichever comes first; recorded as a ROADMAP line at the plan close so it has a home rather than a mention |

## Self-review (writing-plans skill duty, run at authoring)

**Coverage.** All five brief work items appear in the work-item coverage map with a named task, and all **32** acceptance halves appear in the acceptance map with a named producer; producer-less observables: zero. The halves split is where this plan spends its coverage effort, because one producer named for a whole observable is how a real gap survived a plan review two packages ago. W1 is split into eleven rows rather than three because the bump has a lockfile side AND a manifest side AND a diff-scope side that fail independently, because the measurement has an advisory half and a tool half and a demonstration, and because "changed nothing else" is two separate blob checks over two different files; W2 into five, because its two corpus expressions see disjoint forms, because no gate part parses the workflow it edits, and because "valid YAML" and "comment-only" are independent; W3 into eight, because the two locales fail independently and because a repair check and a retention check answer different questions over overlapping files; W4 into three; W5 into five, because a paragraph can lose its ordinals while regressing its line length and because a gate-part invariant holding is not the same as the marked blocks being untouched.

**Latitude.** Every set this plan mandates is enumerated: the six repair sites and nine retained sites of Task A3, by file, with the code predicate that separates them; the two corpus members of Task A2; the three ordinals and one long line of Task A1; the five subcommands of Task A4; the three parts and their per-part deliverables in B1. Every string any task writes is fenced here character for character, in both natural languages, or is a value a prescribed measurement produces (B1's landing `postcss` version, deliberately not fenced, with the reason). No list ends open, no "one per X" appears without its X list, and the one place a fallback would have been convenient - B1's override - is explicitly a NEEDS_CONTEXT route rather than a pre-authorisation, because a pre-authorised fallback is the sanctioned fork the ban exists to prevent. **Placeholders, searched and accounted for rather than declared absent** - the honest form, because a negative claim about the document you are writing is the one whose search terms necessarily come from memory of what you wrote. Each term was grepped alone, since a bundled pattern reports a match for the line rather than for each member, and the run was checked against a known-present control (`measurement`, 27 lines) so an empty result could not be a broken invocation. `TBD`, `TODO`, `appropriate`, `similar to`, `and so on`, `etc.` and `as needed` each return **exactly one** line: the sentence you are reading, which lists them. `...` matches, and its result is reported as **kinds rather than a count**, for the reason the gate-count audit below spells out: this sentence is inside its own search space, so any total it states is falsified by stating it - an earlier form of this paragraph claimed eleven, was corrected to eight after two removals, and the correction itself pushed the real figure back to eleven. Three kinds, and every surviving occurrence falls in one: a **non-existent path** in the header sentence saying no such file exists; an **abbreviated command** whose complete form is fenced in every task's own commit step (the SI-4 `git ... commit` line); and a **marked elision inside a quotation** whose full text is given elsewhere in this document (`pnpm update --help`, the README's CLI sentence, the tracker's lockfile wording). **Five occurrences that were NOT of any of those kinds were found by this very search and removed:** the spelled-ordinal expression abbreviated its own twelve-member alternation, expression B was quoted with its file selector elided, the `cargo doc` command and the four `ledger-lint` marker strings were abbreviated inside a normative diff-scope constraint, and a `cli.rs` quotation was elided in two places. The first two are things a reviewer would re-run and the third is a set an implementer would have to complete, which is the latitude-by-omission shape rather than a typographic nicety. That is the finding this check exists for, and it fired on its author.

**Absence checks, enumerated rather than tallied**, so a reader can check the list: A1 has two (positional ordinals, red 3; non-fenced line length, red 1) plus a made-to-fire run of the gate part that reads the file plus a diff-scope assertion; A2 has two (expression A, red 1; expression B, red 1) plus a blind-spot probe for the prose form both expressions cannot see, plus a YAML-parse check because no gate part parses that file; A3 has one absence check (the repair expression, red 6) and one invariance check (the retention expression, 9 on both states) whose fire is a deliberate deletion that must move the count to 8; A4's `130` check has red 0 on the pre-state and its flag sweep carries a fired control that must return the amended block itself. **Every one of them names its expression, its pre-state run with an exact expected non-zero count, and its end-state expected zero**, and each also carries a reachable green state argued against enumerated survivors rather than asserted. **The absence-shaped checks of the OTHER kind - `git diff --exit-code`, blob comparison, a diff that must come back clean - were initially left with a green state only, and that gap was found by walking this list against the acceptance map rather than by reading the rows.** All four now carry a fire, and each fire is free because the tree already supplies the discriminating case: B1's `--exit-code` and blob checks are run against `pnpm-lock.yaml`, the one file that DID move, beside the files that did not; A1's diff-scope check is run while the `ledger-lint` fire has the canonical sentence mutated and again after the restore. The reason this is not ceremony: a `--exit-code` check reporting no change is byte-identical in output to one whose pathspec matches nothing, whose base revision is wrong, or whose filename is misspelled. Where an expression could be broken rather than clean, a separate soundness control is named - a known-present case outside the edited surface - because the fire against the pre-state proves the pattern matched the sites being removed and says nothing about the pattern's other alternatives.

**Claimed versus prescribed.** The authoring section holds every figure measured against the tree at `2c04ac4` and is labelled as reproducible now; the acceptance map's `evidence` column marks each row `authoring` or `task`, and the task steps hold the fires that run later against deliverables that do not yet exist. A reviewer asked to re-run the second class would have to build the deliverable; the map says which rows those are so the question asked of them is whether the prescribed red state exercises the anchor the check exists to protect.

**Counts recomputed from their own enumerations at authoring, each by counting its list in this file rather than from memory:** 5 tasks, 5 work items, 2 streams, 32 acceptance halves (11 + 5 + 8 + 3 + 5, counted per work item so the total cannot drift from the split named above it), 6 brief corrections, 7 deferred-by-decision rows, 7 controller-surfaced items, 6 repair sites and 9 retained sites in A3 (across 5 and 7 files), 2 corpus members in A2, 3 ordinals and 1 over-80 line in A1, 5 subcommands and 5 synopsis lines in A4, 3 parts in B1, 18 RUSTSEC ignores in `deny.toml`, 12 direct parents of `glib`, 6 arms of `scalar_eq` of which 2 coerce, 5 `type: number` track properties in mkvmerge's schema v20.

**The gate's own part count.** The load-bearing claim is narrower than "the number appears nowhere", and stating the wider one would have been false against this very document: **no CONSTRAINT in this plan states a gate part count.** The gate clause in the Global Constraints names `BUILDING.md` and states no number, which is the whole of what the ban protects, and Task A1's Files list forbids touching the canonical sentence that does state it.

Audited with a search aimed at the forms actually used rather than at a remembered one: `grep -nE '[0-9]+ parts|[0-9]+-part|(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)[- ]part|part [0-9]|parts [0-9]'` over this document. The result is reported as **kinds rather than a total**, because a total stated inside the audited document changes the document and therefore the total - the same self-reference that makes any tally here unstable. Four kinds, and every hit falls in one:

1. **The unrelated task-part sense** - "three parts, one task" about work item 1 and Task B1, and `part a:` in the coverage map. Nothing to do with the gate; it is the reason the expression is noisy and the reason a bare count of its hits would be meaningless.
2. **Quotations of `BUILDING.md`'s own text** - the fenced OLD paragraph and line Task A1 replaces, which necessarily contain `part 6` and `parts 1-4` because removing them is the work.
3. **References to those ordinals as the subject being removed or surfaced** - the acceptance row, the Step-2 reasoning, the Step-4 surfacing of the Tier-2 clause `gate-includes-cross-target-lint-for-the-unrun-os`, the ROADMAP disposition, and the tracker's own measuring expression quoted in the authoring section.
4. **Two references to `BUILDING.md`'s canonical total**, named rather than hidden: Task A1 Step 2's reasoning that "once the file states a total of eleven parts a bare `part 6` acquires a second possible referent", and Step 3's fire instruction to change `11 parts` to `12 parts` and restore. **Both are references to a sentence Task A1 may not edit, not assertions this plan makes on its own authority** - the fire needs the file's exact string to be performable at all, which is the same position Plan 10's own fire occupied.

**Controls run OUTSIDE the audited document, because a control token written into a self-audit is matched by the sentence that names it and proves nothing:** the same expression returns **4** against `BUILDING.md`, so it demonstrably fires on real gate ordinals, and **0** against `renovate.jsonc`, so it can return zero. An earlier form of this paragraph used an in-document negative control and reported it as `0` while the grep in fact returned `1`, matching the control sentence itself; that is recorded rather than quietly replaced, because a self-audit that cannot return zero is worse than none, being read downstream as the check having been done.

**Self-citation.** This document cites no line number inside itself: every self-reference names a container - a task, a step, a table row, a section - which is what `a-document-never-cites-a-line-number-inside-itself` requires and what Plan 10's own delta review had to repair mid-round. Citations INTO other files carry the commit `2c04ac4`, which the scope boundary of `comments-locate-by-symbol-never-by-line-number` permits for a process artifact recording a measurement at a named commit. **Checked with a pattern aimed at the self-citing form specifically** - this plan's own filename followed by a colon and digits, or a phrase naming this plan/document/file within forty characters of a colon-and-digits. It returns **exactly one hit, and that hit is the synthetic control quoted in this sentence** - which is the honest form, because a self-audit that names its own control token is matched by it, and reporting a bare zero here would have been the false-absence shape one level up. The control: fed a synthetic `2026-07-30-plan-11-dependency-alerts-docs-accuracy.md:123` and a synthetic `see this plan at :456`, the pattern returns 2, so it fires on both self-citing forms when they are present; run over the document it finds only the first of those two, i.e. nothing this plan actually cites. The broader bare-span expression from Task A2 also hits this document, and those hits are accounted for rather than waved away: they are the expression itself, the fenced fixture text, and citations into OTHER files at the named commit (`:421` in the spec, `:102`/`:134`/`:135` in `BUILDING.md`), which the scope boundary permits and which are the reason the narrower pattern above is the right instrument for this claim.

**Brief refutations: six**, in the corrections table with pasted evidence - item 2's corpus is two sites rather than one; item 3's assertion set is fifteen lines split six and nine rather than three places; item 4 reaches four of five synopsis lines plus the exit-code bullet rather than `validate` alone; item 1(b)'s hypothesis is refined by the advisory not being surfaced at any severity, which changes what its disposition question is; the brief's own method warning about a pattern's enumerated set was violated by this plan's first pass and is recorded rather than only cited; and item 5's surface has a spelled-ordinal member the tracker's expression cannot match, measured to be a non-defect. **A seventh finding is not a refutation and sits in the surfacing list rather than in the table**: the brief asked for the stale "OPEN OWNER QUESTION" paragraph to be surfaced, and it also turns out that the neighbouring Tier-2 entry's own parenthetical says the comment ruling "stays scoped to SOURCE comments", which the later widening makes misleading to anyone who opens only that entry.
