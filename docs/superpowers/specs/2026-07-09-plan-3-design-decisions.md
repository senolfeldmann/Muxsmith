# Plan 3 design decisions

Status: FINAL 2026-07-09 (Şenol confirmed the design). These decisions
*implement* the v1 design spec (sections 4.9 and 6); unlike Plan 2's D1-D5 they
add no new normative rules, so no spec amendment is needed. This memo is the
decision record (scope split, Plan shape, resolution semantics, command
structure). On any spec/memo conflict the spec wins per repo convention.

Grounding: v1 design spec (authoritative); Plan 2 code as committed at 042a2c0
(`identify.rs`, `planner.rs`, `profile/model.rs`, `template.rs`, `matcher.rs`,
`report.rs`, `capability/`); mkvmerge v100 / pinned identification schema v20
(`~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json`).

## D7: Plan 3 / 4 / 5 split (pure layer, then process layer, then GUI)

**Decision.** Spec section 6 + the unconsumed half of 4.9 are split across two
implementation plans, and the Tauri GUI moves out one slot:

- **Plan 3 (this plan): the pure layer** - resolution (attachments, chapters,
  tags, title, and each assignment's `changes`) plus the `command` module
  (`Plan -> Vec<String>`). Deterministic and golden-testable; no process
  execution.
- **Plan 4: the process layer** - the `executor` (spawn, `--gui-mode` progress
  parse, exit-code mapping, kill + delete-partial, job states), the `run`
  subcommand + FIFO job queue (`--jobs N`, `--fail-fast`), and Ctrl-C (SIGINT)
  cleanup wired to the executor's kill primitive.
- **Plan 5: the Tauri GUI** (was Plan 4 in the spec's numbering).

- Rationale: the pure layer is fully testable without a running mkvmerge; bank
  it green and independently reviewed before touching process management, the
  riskiest and least-testable part. Matches the ~12-task cadence of Plans 1-2;
  a single combined plan would be ~20 tasks and one oversized whole-branch
  review mixing pure and process work.
- Tradeoff: one extra plan boundary and a second whole-branch review. Cheap
  against the review clarity gained on the executor.

**Deferrals.** Persisted per-job logs in the platform app-data directory
(spec 6, "mkvtoolnix-gui-style job log") are deferred to Plan 5: the job-queue
view is their consumer, and the CLI `run` in Plan 4 streams progress/results to
stdout + `--json` instead. Recorded so the spec-6 line is not silently dropped.

**Plan 4 forward decisions** (taken now to avoid re-litigation; revisit only if
Plan 4's brainstorm surfaces a reason):
- Executor process spawn is abstracted behind a trait, mirroring the existing
  `Identify` injection, so progress-parse / state-machine / exit-code mapping
  are unit-testable with a fake; real mkvmerge only in the gated integration
  tier.
- `--jobs N` uses a bounded std thread pool, no async runtime (tokio): muxing is
  external-process / I-O-bound, std suffices, dependency surface stays minimal.
- SIGINT cleanup ships in Plan 4 (not deferred): on Ctrl-C, kill in-flight
  children and delete their partial outputs.

## D8: The `Plan` is self-contained; `command` is a pure, format-neutral consumer

**Decision.** `command` is `Plan -> Vec<String>` with no access to the
`Profile` (spec 6, spec 3 "fully resolved"). The `Plan`/`Assignment` structs
grow to carry every decision `command` needs:

- `Assignment` gains resolved `changes`: the settable property/value pairs that
  apply to the resolved track, carried **format-neutrally** (property name +
  value, not mkvmerge flags).
- `Plan` gains `attachments`, `chapters`, `tags`, `title` resolution results
  (shapes fixed in the plan doc; sketch in D10).

The mapping from a settable property to its mkvmerge option (spec 4.4 table)
lives in `command`, not in the `Plan`. The `Plan` is thus a format-neutral
intermediate representation; `command` is the only module below `capability`
that knows mkvmerge's CLI surface.

- Rationale: keeps `command` a pure function (golden-testable, no I/O, no
  clone-the-profile), and keeps mkvmerge-specific knowledge in one module so a
  future non-mkvmerge backend or a format change touches `command` alone.
- Tradeoff: the `Plan` struct grows and resolution logic moves into the planner.
  Accepted: that is where "fully resolved result for one primary" belongs
  (spec 3), and a fatter value type is cheaper than threading the `Profile`
  through the command layer.

## D9: `identify` parses attachments and chapters

**Decision.** `Identification` gains two fields parsed from `mkvmerge -J`
(exact names from the v20 schema, `additionalProperties: false`):

- `attachments`: array of `{ id: u64, file_name: String, size: u64,
  content_type: Option<String>, description: Option<String>, uid:
  Option<u64> }`. Required by schema: `id`, `file_name`, `size`, `properties`;
  `content_type`/`description`/`type` optional. The match-algebra properties
  (spec 4.9: `file_name`, `content_type`, `description`) are all present; the
  schema's attachment `type` field is not matchable and is not parsed.
- `chapters`: parsed as a total entry count (sum of the array's `num_entries`);
  `-J` reports chapters only as `[{ num_entries }]`, no per-chapter detail.
  Only presence (count > 0) is consumed in v1 (keep/drop decisions).

- Rationale: attachment rules match over attachment metadata and chapters
  keep/drop needs presence; both come from the identification already being
  fetched, so no extra process spawn. Confirmed against the installed binary,
  not memory (repo rule).
- Tradeoff: a wider `Identification`; unknown-property version-skew handling
  (spec 9.2) applies to track properties, not to these fixed sub-objects, which
  are stable across schema versions.

## D10: Resolution semantics

**Decision.** The planner's existing per-file resolution is extended to fully
populate the enriched `Plan`. Per structure:

- **Attachments.** Rules apply to the **primary file's attachments only**
  (sketch: `Plan.attachments` = the kept subset of the primary's attachment ids
  + externally added files). `attachments.rules` resolve in list order,
  first-matching rule wins per attachment (spec 4.9, not uniqueness-
  constrained); `unmatched: keep|drop` governs the rest. External donor files
  contribute their matched **track only**: their attachments are dropped
  (command emits `--no-attachments` on donor groups). External files enter as
  attachments **only** via an explicit `add` locator (spec 4.9), resolved like a
  track donor and subject to the same `MissingExternal`/`AmbiguousExternal`
  diagnostics.
  - Why primary-only: a donor is a track source, not an attachment source;
    silently importing a donor MKV's fonts/covers is surprising and
    order-dependent. Fonts that ASS subtitles need live in the primary. This is
    the one point the spec left open (4.9 does not say per-input); decided on
    the least-surprise principle, flagged for Şenol at the review gate.
- **Chapters.** `keep` -> no `--no-chapters` (mkvmerge's default multi-input
  behavior, pinned against the binary in the golden tests, not asserted here);
  `drop` -> `--no-chapters` on all inputs; external locator -> resolve exactly
  one chapters file per primary (reusing the track-donor locator machinery and
  its `MissingExternal`/`AmbiguousExternal` diagnostics), carried as
  `Plan.chapters = External(path)`.
- **Tags.** Pure flags, no filesystem: `Plan.tags` carries the two keep/drop
  booleans; `command` maps drop -> `--no-global-tags` / `--no-track-tags`.
- **Title.** `keep` -> no override; `clear` -> `Plan.title = Clear`; template ->
  render in literal mode via the existing template engine (same field set and
  rendered-string handling as output filenames), carried as `Set(String)`.
- **Settable `changes`.** Carried format-neutrally into the `Assignment`
  (D8). `language` **values** in `changes` are validated at plan time against
  `mkvmerge --list-languages`, reusing Plan 2's `LanguageIndex` and the
  `InvalidPropertyValue` code (D2) at a new emission site, so a bad settable
  language is a plan-time config error, not a run-time mkvmerge failure.

- Rationale: reuse over new mechanism throughout - the locator machinery, the
  template engine, the language index, and the existing diagnostic codes all
  already exist from Plan 2; Plan 3 wires them to the new resolution sites.
- Tradeoff: none structural; the attachment-scope call is the only genuinely new
  semantic and is recorded above.

## D11: `command` module structure

**Decision.** `command` produces the mkvmerge argv from an enriched `Plan`:

- **Global options:** `--output <path>`, `--title <s>` (title), `--no-chapters`
  / `--chapters <file>`, `--no-global-tags` / `--no-track-tags`, `--attach-file`
  per external add, `--track-order`.
- **Per-input-file groups:** one group per distinct input path (primary, then
  donors), each with its track selection (`--audio-tracks`/`--video-tracks`/
  `--subtitle-tracks` or the `--no-audio` etc. negatives), per-track property
  options from that group's assignments (the settable->option table, spec 4.4),
  attachment selection (`--attachments <ids>` / `--no-attachments`), then the
  `( file )` filename group.
- **`--track-order`** encodes the profile's track order as
  `fileidx:trackid,...` across the input groups.

The exact argv (flag spellings, `--track-order` syntax, multi-input grouping,
chapters/title/tags default behavior with multiple inputs) is pinned during
implementation against mkvmerge v100 via golden tests (fixture identification
JSON -> expected argv, spec 10) and by running the binary, not from memory.

- Rationale: mirrors mkvtoolnix-gui's own command construction; grouping +
  `--track-order` is the only way to express cross-file track ordering to
  mkvmerge.
- Tradeoff: golden tests couple to a specific mkvmerge version's flag surface;
  acceptable because the flags used are long-stable and the pinned version is
  explicit.

## D12: attachment `add` cardinality and zero-match severity

**Decision.** Surfaced while planning attachment resolution; the spec (4.9) says
only "adds an external file as attachment via a locator" without pinning
cardinality. An `add` locator attaches **all** files it matches (not exactly
one), appended in resolution order. An `add` matching **zero** files emits a
**warning** (`MissingExternal` at `attachments.rules[i].add`), not an error: it
does not suppress the plan.

- Rationale: attachments are auxiliary payload and come in sets (the same
  "fonts come in sets" reasoning that makes `select`/`drop` non-unique in 4.9);
  a one-file-per-add rule would be artificially restrictive. Zero matches is
  worth surfacing (likely a mistake) but must not kill an otherwise-valid mux,
  unlike a missing track donor.
- Tradeoff: reuses `MissingExternal` at a non-default (warning) severity, so its
  message wording is track/chapters-flavored; acceptable for v1, a wording
  refinement is a locale-only follow-up, not a code change. Contrast with track
  and chapters donors, which require exactly one file (error on 0 or >=2).

## Testing (feeds Plan 3 tasks)

- `identify`: unit tests for attachment/chapter parsing off fixture `-J` JSON,
  including absent-field (optional `content_type`/`description`) and
  no-attachments/no-chapters cases.
- Resolution: unit tests per structure (attachment first-match-wins + unmatched
  keep/drop + donor attachments dropped; chapters keep/drop/external; title
  keep/clear/template; settable-language validation), plus the external-locator
  diagnostics at the chapters and `add` sites.
- `command`: golden tests, fixture identification JSON -> expected argv, one per
  configured structure and a full reference-example round-trip; argv confirmed
  against real mkvmerge v100.
