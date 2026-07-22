# Muxsmith v1 design specification

Status: approved design, pre-implementation.
Date: 2026-07-08.

## 1. Product summary

Muxsmith is a rule-based bulk muxing tool. The user defines a reusable **profile**: the desired MKV output structure (track order, track properties, attachments, chapters, tags, title) expressed as declarative selection rules, not as picks from one concrete file. Muxsmith applies the profile to every source file in a directory tree, resolves each rule against the actual tracks of each file (and of matched external donor files), and produces one mkvmerge invocation per source file.

- GUI-first desktop app (Windows, Linux, macOS), usability modeled on mkvtoolnix-gui.
- Full CLI with the same capabilities, driven by the same YAML/JSON profiles.
- Muxsmith never processes media itself: it generates and executes `mkvmerge` commands, exactly like mkvtoolnix-gui's job queue does.
- Output is always MKV. Input is anything the local mkvmerge supports.
- Source files are never modified or overwritten. Hard rule, not configurable.

## 2. Decision log

| Decision | Choice | Rationale / tradeoff |
|---|---|---|
| Rule semantics | Strict independent uniqueness | Every rule must resolve to exactly one track regardless of rule order; all overlaps are errors. Most explicit; configs must spell out exclusions (e.g. `forced_track: false`); error quality is a first-class feature to compensate. |
| Disambiguation help | Batch-wide validated suggestions | Suggestions are simulated against the whole batch before being shown; an applied suggestion survives the next dry run. |
| MKV structure scope | Full control in v1 | Tracks, attachments, chapters, tags, title all configurable. More surface than tracks-only, accepted deliberately. |
| Output naming | Keep name or rename template; collision policy | `error | skip | overwrite`, default `error`. In-place replacement excluded. |
| Stack | Tauri 2 + Rust core crate + Vue 3/TS frontend + clap CLI | Mature, small bundles, best packaging. Vue replaces the original React choice (2026-07-10, D27: author preference against React; Vue equally in his stack, SFC templates instead of JSX). Tradeoff: Rust learning curve for the core; webkitgtk quirks on Linux. |
| License | MIT | Permissive, commercialization by anyone including the author; consistent with the Ruby prototype. |
| mkvtoolnix dependency | External, user-installed, CLI invocation only | No linking, no GPL implications, no bundling burden. Detected at startup. |
| Identification schema | Build-time data extraction, never redistributed | Property names/types are generated into the capability model at build time; sidesteps schema licensing and runtime fetching entirely. |
| DRY strategy | One core crate; diagnostics as data; frontend does zero semantic validation | Validation, planning and execution share one code path; GUI and CLI are renderers. |
| Localization | i18n-ready from day one; English and German content ships in v1 | No hardcoded user-facing strings anywhere. One Fluent catalog set shared by the Rust CLI (fluent-rs) and the frontend (@fluent/bundle); long-form help as per-locale markdown. Adding a locale is content work, not a refactor. |
| Discoverability | Self-explanatory UI plus an integrated help mode | Tooltips and inline explanations everywhere; a dedicated hover-to-explain help mode with sidebar (8.3). More UI surface, accepted deliberately: modern tools underinvest here. |

## 3. Concepts

- **Primary file**: a file under the source directory whose basename matches `input.pattern` and whose extension is in `input.extensions`. Each primary file produces exactly one output file.
- **Identifier**: the substring of the primary basename matched by `input.pattern`, including its capture groups. Used to locate external files and to render output filenames.
- **External (donor) file**: a non-primary file located by a rule's external locator, contributing one track to the output.
- **Rule**: one entry in `tracks` (or `attachments.rules`); declares how to select exactly one track (or attachment) and which property changes to apply to it.
- **Plan**: the fully resolved result for one primary file: source files, rule-to-track assignments, output path, mkvmerge argv.
- **Batch**: all plans for one profile + run inputs, plus batch-level diagnostics.
- **Run inputs**: source directory, output directory, collision policy override, parallelism. Separable from the profile; CLI flags and GUI fields override profile defaults.

## 4. Profile format

YAML or JSON; one data model (serde), both formats fully equivalent. A JSON Schema for the profile format is generated from the same model (schemars) and published for editor autocompletion; it is never hand-maintained.

- `profile_version: 1` required; incremented on breaking format changes.
- Unknown keys are errors, not warnings (explicit over silent).

### 4.1 Reference example

Encodes the canonical use case: series with EN/DE audio, forced/normal/SDH subtitles per language, plus Turkish subtitles from external files.

```yaml
profile_version: 1

meta:
  name: Series EN/DE with forced and SDH subs

input:
  pattern: 'S(?<season>\d{2})E(?<episode>\d{2})'
  extensions: [mkv, mp4]
  recursive: true

output:
  directory: null          # usually supplied as run input
  filename: keep           # keep | template: "Show - S{season}E{episode}.mkv"
  on_collision: error      # error | skip | overwrite

tracks:                    # list order = output track order
  unmatched: drop           # keep | drop; default drop
  rules:
    - match: { exact: { type: video } }

    - match: { exact: { type: audio, language: en } }
      changes: { default_track: true }

    - match: { exact: { type: audio, language: de } }

    - match:
        exact: { type: subtitles, codec_kind: srt, language: en, forced_track: true }
      optional: true
      changes: { track_name: English forced, default_track: true }

    - match:
        exact: { type: subtitles, codec_kind: srt, language: en, forced_track: false }
        not:
          - substring: { track_name: SDH }
          - exact: { flag_hearing_impaired: true }
      changes: { track_name: English }

    - match:
        exact: { type: subtitles, codec_kind: srt, language: en, forced_track: false }
        any:
          - substring: { track_name: SDH }
          - exact: { flag_hearing_impaired: true }
      changes: { track_name: English SDH, flag_hearing_impaired: true }

    # analogous forced / plain / SDH rules for language: de omitted for brevity

    - source:
        external:
          path: .              # relative to the primary file's directory
          extensions: [srt]
          match_to_source: true
      match: { exact: { type: subtitles } }
      changes: { language: tr, track_name: "Türkçe" }

attachments:
  unmatched: keep            # keep | drop; default keep (dropping fonts silently breaks ASS subs)
  rules: []                  # select/drop/add rules, same match algebra (4.3)

chapters: keep               # keep | drop | external locator (same mechanism as external tracks)
tags:
  global: drop               # keep | drop
  track: keep                # keep | drop
title: clear                 # keep | clear | template: "..."
```

### 4.2 `input`

- `pattern`: regex, searched (not anchored) against the basename of each candidate file. First match is the identifier; more than one match in a basename emits an info diagnostic. Named and numbered capture groups become template fields.
- `extensions`: list, matched case-insensitively, validated at runtime against `mkvmerge --list-types` output of the local installation. Not restricted to MKV.
- `recursive`: bool, default `true`.

### 4.3 Match algebra

A match expression is a conjunction of up to five parts; all present parts must hold:

```
expr := {
  exact?:     { property: value, ... }   # equality after normalization
  substring?: { property: value, ... }   # case-insensitive containment; string properties only
  regex?:     { property: value, ... }   # regex search; string properties only
  any?:       [ expr, ... ]              # at least one sub-expression holds
  not?:       [ expr, ... ]              # no sub-expression holds
}
```

- Multiple entries inside one condition map are AND.
- `any` / `not` recurse; arbitrary depth is legal, typical profiles stay flat.
- A present-but-empty `any` or `not` list is a config-time error (`EmptyMatchList`): an empty OR/NOR group is always a mistake (an unfinished edit or a generator artifact), never a meaningful "no constraint"; omit the key instead.
- `substring` and `regex` on a non-string property are config-time type errors, unless the property is `raw:`-prefixed (4.4), which opts out of the type check and matches untyped.
- Case sensitivity: `exact` compares strings case-sensitively (language values are normalized per 4.4); `substring` is case-insensitive; `regex` is taken as written (use `(?i)` for case-insensitive matching).
- Semantics carrier of the whole product; specified exhaustively by the property model (4.4) plus this evaluation rule, and covered by property-based tests.

`exact` is typed value-equality, not raw string equality: each property is compared in its own domain. Numbers compare numerically (`6` == `6.0`); languages compare as languages, with ISO 639 spellings and BCP-47 tags reduced to canonical form (`de` == `ger`, `pt-Latn-BR` == `pt-BR`) while meaningful distinctions are preserved (`pt-BR` != `pt-PT`, `zh-Hans` != `zh-Hant`). Use `regex` for byte-literal matching.

### 4.4 Property model

Two disjoint property sets, both owned by the `capability` module (section 9):

**Matchable properties** are generated at build time from the mkvmerge identification output schema: `type`, `codec_id`, `language`, `language_ietf`, `track_name`, `default_track`, `forced_track`, `enabled_track`, `flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`, `flag_original`, `audio_channels`, `audio_sampling_frequency`, `pixel_dimensions`, and the rest of the schema's track properties. Names in profiles are exactly the identification schema names.

**Settable properties** (`changes`) are a curated table mapped to mkvmerge options:

| Profile name | mkvmerge option |
|---|---|
| `language` | `--language` |
| `track_name` | `--track-name` |
| `default_track` | `--default-track-flag` |
| `forced_track` | `--forced-display-flag` |
| `flag_hearing_impaired` | `--hearing-impaired-flag` |
| `flag_visual_impaired` | `--visual-impaired-flag` |
| `flag_commentary` | `--commentary-flag` |
| `flag_original` | `--original-flag` |
| `enabled_track` | `--track-enabled-flag` |
| `sub_charset` | `--sub-charset` |

Conveniences:

- `language` (matching): accepts ISO 639-2 (`ger`) and BCP-47 (`de`); matched semantically against both `language` and `language_ietf` as reported by mkvmerge. Valid values come from `mkvmerge --list-languages` at runtime.
- `codec_kind` (matching): friendly alias mapped to `codec_id` sets, e.g. `srt` -> `S_TEXT/UTF8`, `ass`, `pgs`, `vobsub`, plus common audio/video kinds. Curated in `capability`. Usable only under `exact`; `substring`/`regex` on `codec_kind` is a config-time error (`CodecKindExactOnly`), since a pattern over the curated alias token is ill-defined; pattern-match `codec_id` instead.
- `sub_charset`: validated leniently (iconv names are open-ended); passed through to mkvmerge.
- **Closed-domain values.** For properties whose value set is closed, an `exact` value outside the domain is `InvalidPropertyValue` (not a silent never-match): `type` and `codec_kind` at config time (against the pinned schema enum and the alias table respectively), `language` at plan time (against `mkvmerge --list-languages`). Open-ended values (`sub_charset`, free-text `track_name`) are exempt.
- **Boolean flags, absent = false.** For `exact` matching, a boolean-typed matchable property that a track's `-J` output omits compares equal to `false`. mkvmerge emits the vanity flags (`flag_hearing_impaired`, `flag_visual_impaired`, `flag_commentary`, `flag_original`) only when set, and Matroska defines them false-when-absent; so `exact: { flag_hearing_impaired: false }` matches a track that never set the flag, mirroring mkvmerge's own semantics rather than requiring the `not: [ exact: { flag: true } ]` idiom.
- **`raw:` opt-in (forward compatibility, D32).** A match property not in the pinned model is rejected at config time (`UnknownProperty`), which protects against typos: a mistyped name that would silently never-match is the worst failure mode for a declarative batch tool. To match a property the local mkvmerge reports but this build's schema does not yet carry (a newer identification schema), prefix the name with `raw:` inside `exact`/`substring`/`regex`, e.g. `exact: { raw:dolby_complexity_index: 3 }`. A `raw:` property bypasses the existence/type/domain checks and is matched untyped (byte-literal value equality against the property named verbatim, no `language` normalization or `codec_kind` aliasing, no false-when-absent Boolean shortcut). Config time flags the bypass (`RawProperty`, info; `RawOnKnownProperty`, warning, on `language`/`codec_kind`), and plan time raises `UnknownPropertySkew` per consumed `raw:` property (9.2). The prefix is a matching opt-in only; it is not accepted in `changes`, where an unknown key stays `UnknownSettableProperty`. YAML parses the prefix as part of the key (a colon not followed by a space stays inside the plain scalar), so no quoting is needed.

### 4.5 Track rules

`tracks` is a `{ unmatched, rules }` block, not a bare rule list: `unmatched` (`keep | drop`, default `drop`) is the policy for PRIMARY-file tracks no rule matches; `rules` carries the ordered list below. `rules` may be empty when `unmatched: keep`: that is a legal pure-passthrough remux (change only title / attachments / chapters, or normalize the container) and validate announces it with the info-severity `PassthroughProfile` notice (D38). Empty rules under `drop` remain a `NoTrackRules` error.

```
rule := {
  source?:   primary (default) | { external: locator }
  match:     expr                 # required
  optional?: bool (default false) # exactly the zero-candidate case; two candidates remain an error
  changes?:  { settable: value, ... }
}
```

List order in `tracks.rules` defines the output track order (`--track-order`). Under `unmatched: keep`, that order applies only within the primary: `--track-order` lists every primary track first, in the primary's own source order, then donor tracks in `tracks.rules` order (D20) - kept-but-unmatched primary tracks count as matched, so the primary leads and donors trail. An explicit primary rule under `keep` still applies its `changes` but does not reposition the track; reordering the primary is a `drop`-mode operation.

### 4.6 External locator

```
locator := {
  path:             string        # relative to the primary file's directory, or absolute
  recursive?:       bool (default false)
  extensions:       [string]      # validated against mkvmerge --list-types
  match_to_source?: true          # sugar for match_pattern: '{match}'
  match_pattern?:   template      # mutually exclusive with match_to_source
  case_sensitive?:  bool (default false)
}
```

- The locator selects candidate files; the rule's `match` expression then selects exactly one track inside the located file. Donor files are full containers: an external MKV with a rule matching `{ type: audio, language: de }` is the supported way to pull matching German audio from a second release of the same series.
- Uniqueness applies at both stages: two matching donor files is `AmbiguousExternal`; two matching tracks inside the donor is `AmbiguousRule`.

### 4.7 Template engine

One engine, two render modes, shared by output filenames and `match_pattern`:

- **Literal mode** (output filenames): fields interpolate as plain strings.
- **Regex mode** (`match_pattern`): interpolated field values are regex-escaped literals; the surrounding template text is a regex. Matching is case-insensitive unless `case_sensitive: true`.

Fields: `{match}` (whole identifier), named groups (`{season}`), numbered groups (`{g1}`, `{g2}`), `{source_stem}` (primary basename without extension; literal mode only).

Filters: `{season}` raw as captured (`03`); `{season:int}` leading zeros stripped (`3`); `{season:pad2}` / `{season:pad3}` zero-padded.

Example: `match_pattern: 'staffel0*{season:int}episode0*{episode:int}'` matches `staffel03episode01`, `staffel3episode01` and `Staffel3Episode1` for a primary matched as `S03E01`.

### 4.8 `output`

- `directory`: profile default, usually overridden per run.
- `filename`: `keep` (source basename, `.mkv` extension enforced) or `template` (literal mode; `.mkv` appended if missing; no subdirectory creation in v1). Two invariants are checked on the RENDERED name (not just the template text), identically on all platforms: a path separator (`/` or `\`) is `PathSeparatorInRenderedName`; an empty stem or `.`/`..` is `EmptyRenderedName`.
- `on_collision`: `error | skip | overwrite`, default `error`. Governs collisions with the FILESYSTEM only: a rendered output path that already exists as a pre-existing on-disk file (not one of this batch's inputs). `error` refuses (no plan), `skip` omits that output (no plan, warning), `overwrite` replaces it (plan kept, info). An output path equal to any input path (primary or donor) is always a hard `SourceOverwrite` error regardless of policy.
- Two planned outputs rendering to the same path is ALWAYS an error (`OutputCollision`, error severity), independent of `on_collision`: the batch is internally inconsistent and neither `skip` nor `overwrite` can define which plan wins. Fix the naming (disambiguate the `filename` template or `input.pattern`).

### 4.9 Attachments, chapters, tags, title

- `attachments.unmatched`: `keep | drop`, default `keep`. Deliberate asymmetry of DEFAULTS with tracks (`tracks.unmatched` defaults to `drop`, see 4.5; both are configurable since D20 - this parenthesis predated D20 and stale-claimed "always dropped" until 2026-07-11): attachments are auxiliary payload and dropping fonts silently breaks ASS rendering.
- `attachments.rules`: ordered list; each rule either selects (keeps) or drops attachments matching an expression over `file_name`, `content_type`, `description` (same algebra, `exact`/`substring`/`regex`/`any`/`not`), or adds an external file as attachment via a locator. Selection rules here are not uniqueness-constrained: an attachment rule may match several attachments (fonts come in sets); rules resolve in list order, first matching rule wins per attachment.
- `chapters`: `keep | drop` or an external locator resolving to exactly one chapters file per primary (XML or simple format, as mkvmerge accepts).
- `tags.global`, `tags.track`: `keep | drop` (mapped to `--no-global-tags`, `--no-track-tags`).
- `title`: `keep | clear | template` (literal mode).

## 5. Planning semantics

### 5.1 Per-file resolution

For each primary file, independently per rule:

1. Resolve the rule's source: the primary file itself, or the external locator (identify candidate files, apply `match_pattern`).
2. Evaluate the match expression against every track of the resolved source; the candidate set is computed independently of all other rules (strict uniqueness, no consumption, no order effects).
3. Collect diagnostics; a file with any error-severity diagnostic produces no plan but planning continues for the remaining files (no fail-fast).

### 5.2 Diagnostics

Diagnostics are data, produced only by `muxsmith-core`:

```
diagnostic := { code, severity: error|warning|info, config_path, file?, params, suggestion_ref? }
```

Core emits no user-facing prose: `code` plus structured `params` select and fill a message and hint template from the shared catalog at presentation time (8.4). `--json` output carries code and params plus the rendered message in the active locale, so scripts key on codes, humans read text.

| Code | Severity | Condition |
|---|---|---|
| `AmbiguousRule` | error | rule matches >= 2 tracks of its source |
| `OverlappingRules` | error | one track claimed by >= 2 rules |
| `MissingTrack` | error | non-optional rule matches 0 tracks; hint lists near-misses (tracks of same type/language and which condition each failed) |
| `MissingExternal` | error | locator (track rule or chapters) finds 0 files for a non-optional use |
| `AmbiguousExternal` | error | locator (track rule or chapters) finds >= 2 files |
| `UnidentifiableSource` | error | a discovered primary or resolved donor exists but mkvmerge could not identify it (`detail` carries the underlying error) |
| `UnsupportedSource` | error | mkvmerge identified the file but its container is not a supported muxing source (D21 gate: `!container_recognized || !container_supported`); fires on the primary (`kind=primary`) or, since Plan 5.5, on an external donor (`kind=donor`, `donor` names the file); the affected file's plan is dropped |
| `EmptyPlan` | warning | a file's plan survived every finalize pass (no error-severity diagnostic, local or cross-file) but resolved zero track assignments; a `tracks.unmatched: keep` plan whose primary carries at least one track does not fire this (D20: passthrough counts as matched); per-file, reported in the batch like any other diagnostic |
| `PassthroughProfile` | info | `tracks.rules` is empty and `tracks.unmatched` is `keep`: the profile is a legal pure-passthrough remux, every primary track copied unchanged (D38); emitted at validate time so an accidental delete-all-rules edit stays visible |
| `OutputCollision` | error (two planned) / per policy (on-disk) | two plans render to one path (always error), or the rendered path pre-exists on disk (severity per `on_collision`: error/warning-skip/info-overwrite; 4.8) |
| `PathSeparatorInRenderedName` | error | rendered output filename contains `/` or `\` (checked on all platforms) |
| `EmptyRenderedName` | error | rendered output stem is empty or is `.`/`..` |
| `SourceOverwrite` | error (hard) | output path equals any input path |
| `NonUtf8Path` | error | an argv-bound path (rendered output, external chapters file, attachment `add` file, primary or donor source) is not valid UTF-8, so command generation (6) could render it into the mkvmerge argv only with lossy U+FFFD substitution; checked at plan finalize over exactly the paths `command` renders, once per offending file; `path` (lossy rendering) and `role` (`output`/`chapters`/`attachment`/`primary`/`donor`) params carry the offender; the file's plan is dropped, so no job is ever built from a lossily-converted path (D37) |
| `DuplicateIdentifier` | warning | two primaries yield the same identifier match (e.g. 720p and 1080p copies): both are muxed, both attract the same external files, templates may collide |
| `DonorIsPrimary` | warning | an external donor file is itself a primary (it will be muxed as its own output and donate tracks) |
| `IgnoredFile` | info | extension matches but `input.pattern` does not |
| `MultipleIdentifierMatches` | info | `input.pattern` matches more than once in a basename; first match used |
| `UnknownExtension` | warning | a `profile.input.extensions` entry, or a locator's `extensions` entry (track rule, `chapters`, `attachments.rules[i].add`), is not among the local mkvmerge's `--list-types` output; still used for matching, so a typo silently excludes candidates; skipped (not raised) when the capability query is unavailable; `extension`/`known` params carry the offender and the accepted set (4.2, 4.6) |
| `UnknownProperty` | error | a match condition references a property not in the capability model (config-time; unknown `changes` keys are `UnknownSettableProperty`); a `raw:`-prefixed name opts out (4.4, 9.2) |
| `RawProperty` | info | a match condition uses a `raw:`-prefixed property: an explicit opt-in that bypasses the capability checks and matches untyped; `property` carries the bare name (config-time; 4.4, 9.2) |
| `RawOnKnownProperty` | warning | `raw:` applied to a model property with special matching semantics (`language`, `codec_kind`), degrading it to byte-literal untyped equality (config-time; 4.4, 9.2) |
| `CodecKindExactOnly` | error | `codec_kind` used under `substring`/`regex` (config-time; it is `exact`-only, 4.4) |
| `InvalidPropertyValue` | error | `exact` value outside a closed domain (`type`/`codec_kind` config-time, `language` plan-time; 4.4) |
| `EmptyMatchList` | error | a present-but-empty `any` or `not` list (config-time; 4.3) |
| `UnknownPropertySkew` | warning | a `raw:`-opted property was consumed at plan time and matched untyped; `property`/`found_version`/`pinned` params report the property and the runtime-vs-pinned schema versions, one code covering both a genuinely newer schema and a same-version untyped match (9.2) |
| `SchemaDrift` | info | at least one source file's `identification_format_version` exceeds the build-time pinned schema version; `found_version`/`pinned` params carry both versions; emitted once per batch when planning concludes (9.2) |
| `SuggestionsCapped` | info | the suggestion engine accepted more than 3 candidates for one conflicted rule; `dropped` carries how many were capped (5.3, D6) |
| `SuggestionPartition` | info | no single refinement resolves a conflicted rule batch-wide, so the no-single-fix partition is reported: affected files grouped by the per-file refinement that resolves each; `kind=group` carries a group's `fix`/`files`, `kind=overflow` the `dropped` count when more than 5 groups were capped (5.3, D6 step 6) |
| `WorkerPanicked` | n/a (job-error token, not a rendered diagnostic) | a queue worker thread panicked while running a job (a bug in this crate, never an mkvmerge failure); the job is reported `Failed`. Not a batch `Diagnostic`: carried as a `worker-panicked: job N` token in `JobOutcome.errors` (and its `--json` job encoding) instead, rendered through this same catalog entry at presentation time (6) |

### 5.3 Suggestion engine

Contract: for `AmbiguousRule` and `OverlappingRules`, generate candidate refinements by diffing the properties of the conflicting tracks across **all** affected files, then simulate each candidate against the cached identification data of the **entire batch**. Emit only refinements that (a) resolve every instance of the conflict batch-wide and (b) introduce no new diagnostic for any file. If no single refinement satisfies both, report that explicitly and list the files requiring different resolutions.

Suggestions are structured edits (`config_path` + proposed change), not prose; the GUI offers one-click apply, the CLI prints them as exact YAML fragments.

Algorithm (closed edit grammar, discriminator generation, batch simulation via the real planner, acceptance invariant, deterministic ranking, no-single-fix partition): `docs/superpowers/specs/2026-07-09-plan-2-design-decisions.md` D6.

### 5.4 Static lint

Best-effort, file-independent checks at validate time: regex/template compilation, type errors, unknown properties (unless `raw:`-opted, 4.4/9.2), closed-domain value checks (`type`, `codec_kind`; `language`'s domain needs runtime and is checked at plan time), `codec_kind` exact-only, and provable rule overlaps (rule A's condition set logically subsumes rule B's, so any track matching B must overlap A). Static analysis never replaces the dry run; it catches what is decidable without looking at files.

### 5.5 Operation levels

One code path, three entry points:

1. **validate**: static checks only, no filesystem access beyond the profile.
2. **dry-run**: the config-time static validate pass (level 1) FIRST, then a full planning pass over the batch; produces the report (config-time diagnostics + per-file resolution tables + all planning diagnostics + suggestions). dry-run is a strict superset of validate, never a subset. No mkvmerge mux invocations, only `-J` identification.
3. **run**: re-plans immediately before execution (identification is cheap and cached; a dry run can never be stale), then executes plans. Any error-severity diagnostic for a file skips that file, reported identically to the dry run.

Identification cache: in-memory per session, keyed on path + mtime + size, shared between dry-run and run. On-disk cache is a future candidate.

## 6. Command generation and execution

- `command` is a pure function `Plan -> Vec<String>`: `--output`, per-input track selection (`--audio-tracks`/`--video-tracks`/`--subtitle-tracks`, `--no-audio` etc.), per-track property options (4.4 table), input file groups in mkvmerge's `( file )` syntax, `--track-order`, plus `--no-chapters`, `--no-attachments`, `--no-global-tags`, `--no-track-tags`, `--title`, `--attach-file`, `--chapters` as configured.
- Execution uses `--gui-mode` for machine-readable progress (`#GUI#progress NN%`) and line-tagged warnings/errors.
- mkvmerge exit codes are honored and surfaced: 0 = success, 1 = completed with warnings (job marked "warning", output kept, warnings shown), 2 = error (job failed, partial output deleted).
- Cancellation: kill the mkvmerge process, delete the partial output file.
- Job engine: FIFO queue over the batch's plans; sequential by default; `--jobs N` opt-in parallelism (muxing is I/O-bound; parallelism pays only on fast storage). Failures do not abort the batch unless `--fail-fast`. Full command line and output of every job are persisted to the app data directory (mkvtoolnix-gui-style job log).

## 7. Architecture

```
Muxsmith/
├── crates/
│   ├── muxsmith-core/      # all logic (below); emits no user-facing prose
│   └── muxsmith-cli/       # thin clap binary over core
├── src-tauri/              # Tauri shell: commands + job event stream, no logic
├── src/                    # Vue 3 + TypeScript frontend
├── locales/                # Fluent catalogs (locales/en/*.ftl), shared by CLI and frontend
└── help/                   # long-form help topics, markdown per locale (help/en/<help-id>.md)
```

`muxsmith-core` modules, each independently testable:

| Module | Responsibility |
|---|---|
| `profile` | serde model, semantic validation, diagnostics |
| `capability` | mkvtoolnix model: build-time generated matchable properties, curated settable table, runtime queries (`--version`, `--list-types`, `--list-languages`), codec_kind aliases, version-skew handling |
| `identify` | `mkvmerge -J` wrapper + cache |
| `matcher` | pure match-expression evaluation |
| `planner` | per-file resolution, batch report, suggestion engine |
| `template` | template engine, both render modes |
| `command` | `Plan -> Vec<String>` |
| `executor` | process spawn, progress parse, cancellation, job states |
| `report` | diagnostic structures and serialization (JSON) |

Rules that keep it DRY:

- The frontend performs **zero** semantic validation; every profile edit round-trips through a core `validate` Tauri command (local IPC, fast). Frontend-side logic is limited to UX affordances (e.g. disabling Save while errors exist).
- CLI and GUI render the same diagnostic and report structures; neither owns logic.
- The profile JSON Schema, the capability property lists, and all validation live in exactly one place each.

## 8. Surfaces

### 8.1 CLI

```
muxsmith validate <profile>
muxsmith dry-run  <profile> [--source DIR] [--output DIR] [--json]
muxsmith run      <profile> [--source DIR] [--output DIR] [--jobs N] [--fail-fast] [--json]
muxsmith identify <file> [--json]
muxsmith schema                      # print the profile JSON Schema
```

- Flags override profile-stored run inputs.
- Exit codes mirror mkvmerge: 0 success, 1 warnings, 2 errors.
- `--json` emits the structured report for scripting; default output is human-readable rendering of the same data, including suggestion YAML fragments.
- `muxsmith schema` is a supported user feature, not only a debug aid (D47): the README's "Using the CLI" section documents redirecting its output to a file and binding it in editor settings (`yaml.schemas` in VS Code, the equivalent `lspconfig` block for Neovim/Helix) for autocompletion and inline validation while hand-authoring a profile.

### 8.2 GUI

Three views, modeled on mkvtoolnix-gui:

1. **Profile editor**: track-rule grid (order, source, match summary, changes, optional; drag to reorder), detail editor per rule, panels for attachments/chapters/tags/title, open/save YAML, recent profiles. Saving writes canonical YAML rendered fresh from the in-memory model, not a patch of the file on disk: comments, key order and formatting are not preserved (D41), and a field left at its serde default is omitted rather than written back explicitly (D48). Inline validation markers from core diagnostics.
2. **Batch view**: source/output pickers (persisted per profile), file list with per-file resolution table (rule -> resolved track), diagnostics panel with one-click apply-suggestion, dry-run trigger.
3. **Job queue**: per-job progress (from `#GUI#progress`), overall batch progress, live log, warning surfacing, cancel per job or batch.

App settings (not profile data): mkvmerge path override, default parallelism. Stored in the platform config directory.

First-run and startup: detect mkvtoolnix (PATH, then platform-standard install locations, then configured override); if missing, per-OS installation guidance. Minimum supported mkvtoolnix version is fixed during implementation and enforced with a clear error.

### 8.3 Self-explanation and help mode

Baseline discoverability: every non-obvious control carries a tooltip; views carry small inline explanations where a first-time user would otherwise guess. The UI must be usable without reading external documentation.

On top of that, an integrated **help mode**:

- A prominent Help/Guide button, always visible in every view. Clicking it toggles help mode; clicking again (or Esc, except while the settings dialog is open, whose native cancel consumes Esc) exits.
- Entering help mode opens a right-hand sidebar with independently scrollable explanatory text; initially it shows the long-form explanation of the current view.
- Hovering any help-annotated element highlights it with a faint border and swaps the sidebar content to that element's long-form explanation (beyond tooltip depth: what it does, when to use it, interactions with other settings). Hovering an element without a help-id sets no hover topic: the sidebar shows the pinned topic if one is pinned, else the current view's topic. Keyboard focus is equivalent to hover for topic selection: `focusin` drives the same hover topic state (recorded at plan close from the shipped, design-mandated behavior - whole-branch review M6, 2026-07-22). Clicking an annotated element pins without activating it.
- Clicking an element pins the selection: the element gets a prominent marking and the sidebar stays on its topic regardless of hover, until another element is clicked, the active view is switched, in which case the hover state resets too and the sidebar shows the new view's topic, or help mode exits.
- While help mode is active, pointer-channel activation inside the main
  content area is suppressed - click activation and drag-reorder, both
  at capture phase; keyboard and text-entry channels (typing into
  fields, select changes via keyboard) stay deliberately live
  (`help-mode-suppression-pointer-scope`, product-boundaries.yaml -
  owner ruling 2026-07-22, S21, Option 2); the help toggle, the three
  view tabs, the settings button and the sidebar stay live; clicking an
  annotated element pins its topic instead of activating it (owner
  ruling 2026-07-21, E3).

Mechanics: every help-annotated element carries a stable `help-id`; help content is one markdown file per help-id per locale (`help/<locale>/<help-id>.md`), rendered in the sidebar. Views have their own help-ids for the default sidebar content.

### 8.4 Internationalization architecture

Localization readiness is structural, not deferred polish:

- **No hardcoded user-facing strings** in any layer: not in the frontend, not in the CLI, not in core. Core emits diagnostic codes and params only (5.2); labels, tooltips, messages and hints live in Fluent catalogs; long-form help lives in per-locale markdown. Accepted v1 exceptions: clap's library-generated `--help`/usage text, third-party error text passed through as a `detail` param (regex, serde, I/O), the fixed English framing in `IdentifyError`'s `Display` (e.g. "mkvmerge failed: ...") surfaced via a `detail` param, which wraps that same third-party mkvmerge/serde/I-O error text, and the JSON Schema's `description` fields (Rust doc comments, D47). The schema documents a file format, the same category as this spec and the README, both English-only by design; it is not application UI and not a diagnostic, so Fluent's localization mandate does not reach it.
- One catalog source of truth under `locales/`, consumed by fluent-rs (CLI rendering, embedded at build time) and @fluent/bundle in the frontend. Diagnostic message templates exist exactly once, shared by both surfaces.
- Locale selection: system locale with manual override in app settings (takes effect live, without restart; D56) and `--locale` on the CLI; falls back to English per message.
- v1 ships English and German content on both surfaces - GUI catalogs and
  help topics, and the CLI's embedded catalogs (`cli-multilang-rendering`,
  D63); further locales are content work (non-goal 11).

## 9. Capability model and version skew

1. **Build time**: matchable property names and types are extracted from the pinned upstream identification output schema into generated Rust code, including the closed value domains of enum-typed properties (e.g. `type`) for config-time `InvalidPropertyValue` checks. The schema file itself is not redistributed; only facts derived from it ship. Upgrading the pinned schema version is a normal PR.
2. **Runtime**: the local mkvmerge is queried for version, supported file types and languages. `mkvmerge -J` output carries `identification_format_version`. A property name unknown to the pinned model is a config-time `UnknownProperty` error (typo protection) unless the profile opts in with a `raw:` prefix (D32). A `raw:`-prefixed property bypasses the capability existence/type/domain checks and is matched untyped: byte-literal value equality against the property named verbatim, with no `language` normalization, no `codec_kind` alias expansion, and no false-when-absent Boolean shortcut. The opt-in is announced at config time by `RawProperty` (info), or by `RawOnKnownProperty` (warning) when the bare name is a model property with special matching semantics (`language`, `codec_kind`) that `raw:` degrades to byte-literal equality. At plan time each `raw:` property consumed while resolving a rule raises an `UnknownPropertySkew` warning carrying `property`, `found_version` (the file's `identification_format_version`) and `pinned` (this build's schema), so the untyped match is visible whether the runtime schema is genuinely newer than pinned or the same. Additionally, planning emits a `SchemaDrift` info notice once per batch if any source file's `identification_format_version` exceeds the pinned version, alerting the user that their mkvmerge may support properties not yet in Muxsmith's model and enabling opt-in via the `raw:` prefix. Forward compatibility without lying about type safety: the untyped path is opt-in and declared, never inferred from a version bump. Upgrading the pinned schema (item 1) remains the typed path for a property that should become first-class.

## 10. Testing

- `matcher` + planning semantics: unit tests plus property-based tests (proptest); this is the correctness core.
- `command`: golden tests, fixture identification JSON -> expected argv.
- Integration: real mkvmerge in CI generates tiny fixture MKVs (from srt/wav seeds via mkvmerge itself); end-to-end dry-run and run against them.
- CLI rendering: snapshot tests (insta).
- GUI: thin Playwright smoke; logic lives in core, so UI tests stay shallow.
- i18n and help completeness: CI fails on catalog keys referenced but missing in the English catalog, on diagnostic codes without message templates, and on help-ids without a help topic file. The `@intlify/vue-i18n/no-raw-text` eslint rule (D27) keeps hardcoded
  strings out of Vue templates - template text nodes plus the configured
  static attributes (`title`, `aria-label`, `placeholder`, `alt`);
  `:`-bound expressions are covered by the check-i18n literal scan instead;
  core is prose-free by construction.
- CI: GitHub Actions matrix (windows, macos, linux) running tests; packaging artifacts (msi, dmg, deb, rpm, AppImage) on release tags. While the repo is private, branch pushes run Linux only (Actions minute multipliers: Windows 2x, macOS 10x); the full matrix runs on PRs, tags and manual dispatch. Reverts to the full matrix on every push when the repo goes public.

## 11. Non-goals for v1

- In-place replacement of source files.
- Season/episode arithmetic (identifiers are opaque match keys; transforms are per-file, not cross-file).
- Watch/daemon mode.
- Wildcard multi-track rules ("keep all remaining audio"); breaks strict uniqueness; possible later as explicit `multi: true` rules.
- Any transcoding, ever; Muxsmith muxes.
- Per-file manual overrides in the GUI (that is mkvtoolnix-gui's job; an "open in mkvtoolnix-gui" escape hatch is a v1.x candidate).
- Track delay/stretch (`--sync`) changes; per-file offsets do not generalize to batch rules; v1.x candidate.
- mkvpropedit fast path for metadata-only changes.
- Bundling mkvtoolnix binaries; a Windows convenience downloader is a v1.x candidate.
- On-disk identification cache.
- Locales beyond English and German. The mechanism (8.4) ships complete on
  both surfaces; adding a locale is content work (catalogs + help topics
  land together, enforced by CI) plus one row in the CLI embed table
  (D63), not a refactor.

## 12. Licensing

- Muxsmith: MIT.
- mkvtoolnix: invoked as external executables only; no linking, no code reuse, no redistribution in v1.
- Identification schema: consumed at build time as a data source; not shipped.
