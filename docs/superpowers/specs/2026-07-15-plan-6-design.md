# Plan 6 design: profile editor, apply-suggestion, schema keyword domains

Status: DRAFT 2026-07-15. Numbering starts at **D41**; the last existing ADR
is D40 (`2026-07-14-plan-5.8-decisions.md:268`), verified by sweeping
`^## D` across `docs/superpowers/specs/`.

Scope per the Plan-6 re-cut (`docs/ROADMAP.md:29-64`): profile editor,
one-click apply-suggestion, schema keyword-domain fix. Help mode (Plan 7),
packaging (Plan 8) and the core hoists (Plan 9) are out.

**Every fork in this document is closed.** No design-latitude clause appears
anywhere in it, in any form - not "the implementer may choose", not "either
approach works", not "if a simpler alternative exists"
(`proc-latitude-clause-boundary`, which since 2026-07-15 binds every artifact
an implementer reads, not only the task brief: a design document that defers a
fork hands it over through a different door). The one question this document
originally escalated - whether a canonical save emits default-valued fields -
was **ruled by the governing human on 2026-07-15 (omit)** and is recorded as
**D48**.

Grounding: v1 design spec (authoritative); Tier-2 `docs/conventions.yaml`,
`docs/product-boundaries.yaml`, `docs/process-conventions.yaml`; D6 (edit
grammar), D22 (scope split), D23 (IPC surface), D39 (allowed-param wire);
research corpora `yaml-roundtrip-landscape.md`, `rust-ts-ssot-landscape.md`,
`rust-ts-ssot-empirical-probes.md` (non-repo material); mkvtoolnix source and
binary at v100.0 (`~/Downloads/mkvtoolnix`, `mkvmerge --version` ->
`mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit`).

Every version named below was registry-verified against the crates.io API on
2026-07-15, not typed from memory (`proc-07-verify-against-source`).

---

## 0. Corrections to the brief, and to the corpora it rests on

Recorded first because later sections depend on them. Each was checked
against the tree or re-measured.

| # | Claim | Reality |
|---|---|---|
| 1 | "the logic exists as a test helper `apply_edit_to_first_rule` (`tests/suggestions.rs:95`). Reuse before writing: hoist it into the library." | The helper is a **fixture generator, not apply logic**. It takes no `Profile`, mutates nothing, and emits a hardcoded YAML document with the fixture's own conditions (`type: subtitles, codec_kind: srt, language: en`) baked in as string literals (`tests/suggestions.rs:95-113`). Its own comment says it *mirrors* what apply would do. **There is nothing to hoist**; D43 writes the function fresh. |
| 2 | "`yamlpath` + `yamlpatch` (zizmor's fix engine, built on the same `yaml_serde` 0.10 already in the tree)" | `yamlpath` 1.27.0 is built on **tree-sitter-yaml** (deps: `tree-sitter ^0.26.9`, `tree-sitter-yaml ^0.7.2`, `tree-sitter-iter`, `self_cell`, `line-index`; crates.io dependencies API, 2026-07-15). It does **not** depend on `yaml_serde` at all. `yaml_serde ^0.10` appears only in `yamlpatch` 1.26.1, as the patch **value currency**, never as the parser. The corpus's own §3.1 states this correctly; its §1 summary does not, and the brief inherited the §1 error. Adopting the pair would add a **second YAML parser** (a tree-sitter C runtime plus grammar) beside the existing `yaml_serde`/`libyaml-rs`. This makes the rejected alternative **more** expensive than the brief credits, not less. |
| 3 | "Measured on the real `reference.yaml`: deleting a rule via the leading candidate left a comment describing the deleted rule sitting above a different one." | The **hazard is real and reproduced**, but not on `reference.yaml`. The repo fixture (`crates/muxsmith-core/tests/fixtures/reference.yaml`, 80 lines, 2828 bytes) carries 12 comments, **all trailing inline comments on scalar lines, none above any rule** - the tested construct cannot exist in it. The measurement ran against a 9-line hand-built `commented.yaml` in the corpus author's scratchpad. Sound measurement, wrong provenance label. |
| 4 | "Valid YAML, **identical comment count**, document now lies." | The count half is **false**. Re-running the probe: 5 comments in, **3 comments out**. Two were consumed. The corpus's own adjacent prose ("the `# this rule handles audio` comment was consumed") contradicts its count claim, in both its §1 and §4.3. The **orphaning** is real and reproduces exactly; the count decoration is not. D41 records the hazard without it. |
| 5 | "`TS_RS_LARGE_INT = "number"` must be set, or 64-bit integers map to `bigint` and **mistype every numeric field**." | The directive is right; the scale is wrong. The edited model has **exactly one** 64-bit integer: `Scalar::Int(i64)` (`profile/match_expr.rs:23`). The only other numeric field in the whole profile model is `profile_version: u32` (`profile/model.rs:22`), which maps to `number` regardless. `TS_RS_LARGE_INT` is still **mandatory** (D44), because `Scalar` is the value type of every `exact`/`changes` entry and a `bigint` breaks `JSON.stringify` on the IPC wire - but it protects one type, not "every numeric field". |
| 6 | "`scripts/check-i18n.mjs` finds catalog keys by grepping literal calls, so it **goes blind** and loses that check." | Half right. The script runs **three** checks. **Check 2 does not go blind**: it counts a key as used if it appears anywhere in `src/` as a quoted string literal (`scripts/check-i18n.mjs:191-198`; the test is `text.includes(`"${id}"`) || text.includes(`'${id}'`)` at `:193`, so single-quoted literals count too), which is exactly the registry's `labelKey: "editor-..."` shape - the mechanism already exists for `jobRowState.ts`'s identical map-to-Fluent-key pattern (`src/jobRowState.ts:44-55`). **Check 3** (cross-locale parity) is unaffected. Only **check 1**'s hard gate misses the registry, and it never covered dynamic keys in the first place. D45 closes check 1 precisely; nothing else is traded. |
| 7 | "the schema cannot express our keyword domains (section 4.3)" - used in 4.2 as one of three arguments against schema-driven forms. | **Self-undermining as stated**: D46 fixes exactly this deficiency, in this same plan. This argument holds only until D46 lands, so it is **not recorded as load-bearing** in D45. The rejection stands on the other two arguments, both of which survive D46 and are measured. See D45's rejected-alternatives section, which states this explicitly so it is not re-litigated the moment someone notices D46. |
| 8 | Brief section 2: "Plan 6 is the profile editor, its one-click apply-suggestion, and the schema keyword-domain fix." | The **ROADMAP's Plan-6 anchor carries a sixth named design input the brief omits**: "Re-check the final fix wave's self-flagged deviation from D23's frontend contract (reset gated on runActive instead of 'reset after resolve Ok' ...)" (`docs/ROADMAP.md:58-64`). It is a run-path concern living entirely in `src/views/JobsView.vue:150-200` and touches neither the editor, nor apply, nor the schema. The brief's scope is right; the input was left under Plan 6 by the re-cut, which re-pointed the *ledger* entries but not this *named input*. **Accepted and applied by the controller in `fdcdcba`**: the ROADMAP now records the mis-filing in place (`:61-64`) and leaves the item listed until an owner call re-points it, rather than moving it silently. **Out of scope here.** |

Two further tree facts the brief's file:line shorthand omits, stated so later
citations resolve: the model and validation live at
`crates/muxsmith-core/src/profile/model.rs` and `.../profile/validate.rs`
(the brief writes `model.rs` / `validate.rs` bare). **All eight line numbers
the brief cites are correct**: `FilenameCfg` :145, `SourceCfg` :223,
`ChaptersCfg` :341, `TitleCfg` :373; guards at validate.rs :105, :129, :149,
:166. So are `planner.rs:201` (`StructuredEdit`), `:231` (`Suggestion`),
`:1437`/`:1516` (`config_path`), `:2032` (`rule_index_of`), and
`src-tauri/src/lib.rs:302` (`validate_profile`).

---

## 1. Verified ground truth

Established by running the tree, not by reading about it.

**The schema really is inconsistent with itself** (`cargo run -q -p
muxsmith-cli -- schema`, 2026-07-15). All four untagged enums project a bare
string; both real enums project precise constants:

```
FilenameCfg  -> anyOf: [ {$ref: TemplateBlock}, {type: "string"} ]
SourceCfg    -> anyOf: [ {$ref: ExternalBlock}, {type: "string"} ]
ChaptersCfg  -> anyOf: [ {$ref: ExternalBlock}, {type: "string"} ]
TitleCfg     -> anyOf: [ {$ref: TemplateBlock}, {type: "string"} ]
KeepDrop     -> oneOf: [ {const: "keep"}, {const: "drop"} ]
CollisionPolicy -> oneOf: [ {const: "error"}, {const: "skip"}, {const: "overwrite"} ]
```

**The model is already fully serializable.** Every profile struct derives
`Serialize` alongside `Deserialize` and `JsonSchema` (`profile/model.rs:17`
and throughout). D41 needs no derive work.

**`yaml_serde` can write.** `yaml_serde::to_string` / `to_writer` exist
(`ser.rs:707` / `:694` of yaml_serde 0.10.4). No new dependency for saving.

**Model-side validation already exists.** `profile::validate::config_diagnostics(&Profile)`
is public (`profile/validate.rs:193`); `config_diagnostics_from_file` is a
thin wrapper over it (`:203-208`). The gap is only at the Tauri layer.

**Canonical round-trip is exact.** Serializing the parsed `reference.yaml`
and re-parsing yields an equal model (`p == p2` -> `true`). This is D41's
correctness floor and it holds today.

**Canonical save on `reference.yaml`, measured:**

| | hand-authored | emitting defaults | omitting defaults (D48) |
|---|---|---|---|
| lines | 80 | 141 | **112** |
| comments | 12 | 0 | 0 |

**The line growth is not comment loss** (comments are 12 of 80 lines), and D48
removes only half of it. Three separate effects, worth keeping apart because
only one of them was ever up for decision:

1. **Comments go** (12 lines). Decided by D41, not negotiable.
2. **Defaults materialize** (141 vs 112, so 29 lines). This is what D48 rules
   on: omitted.
3. **Formatting normalizes** (the remaining ~44 lines, and the dominant term).
   Flow style becomes block style: `- match: { exact: { type: video } }`
   becomes three lines. `exact` maps re-sort alphabetically (`BTreeMap` order:
   `{type: audio, language: en}` becomes `{language: en, type: audio}`).
   `directory: null` disappears. **Nothing removes this**; it is inherent in
   serializing from a model, and it is why even under D48 an 80-line file comes
   back as 112.

The brief's 4.1 decided (1). It did not decide (2), which is why this document
escalated it; the ruling is D48. Effect (3) has no decision to make and is what
the D41 save-surface note must be honest about.

---

## D41: Saving writes canonical YAML from the model; comments, formatting and key order are not preserved

**Decision** (owner, 2026-07-15). Saving from the editor serializes the
in-memory `Profile` to YAML. Comments, formatting, key order and flow/block
style in the user's file are not preserved. This applies **uniformly**: there
is no editor-owned vs foreign distinction. A profile is a profile - parse it,
and if it is valid, edit and save it; if not, the existing diagnostic chain
(`ParseError`, `profile/load.rs:36-47`) reports why.

The writer lives in **core**, as `muxsmith_core::profile::save`, mirroring
`profile::load`: `to_string(&Profile, Format) -> Result<String, Diagnostic>`
and `to_file(&Profile, &Path) -> Result<(), Diagnostic>`, with `Format`
selected from the path extension exactly as `load::from_file` selects it
(`profile/load.rs:57-62`), so a `.json` profile saves as JSON and never
silently changes format. Neither surface owns document logic
(`core-85-report-json-dry`: "neither surface owns document logic"); the CLI
gets the same function for free if it ever needs it.

**Rationale.** YAML has no concept of comment attachment.
[YAML 1.2.2 §6.6](https://yaml.org/spec/1.2.2/#66-comments) states outright
that comments "are not associated with a particular node". Every library that
"preserves comments" invents an association the specification disclaims. The
consequence is measurable and was reproduced for this document: removing a
rule via `yamlpatch` 1.26.1 leaves the removed rule's leading comment
labelling the **next** rule.

```yaml
# before
    # this rule handles video
    - match: { exact: { type: video } }   # trailing on the video rule
    # this rule handles audio
    - match: { exact: { type: audio } }

# after  Op::Remove on tracks.rules[0]  -> VALID YAML
    # this rule handles video
    - match: { exact: { type: audio } }
```

The output is valid YAML and the document now misinforms the user. (Comment
count is **not** preserved through this: 5 in, 3 out. See correction #4 - the
hazard needs no exaggeration.) The editor's signature operation is
drag-to-reorder (spec 8.2), which is precisely the operation that breaks
attachment; add and remove are the others. **Dropping comments is honest;
carrying them through a structural rewrite is a lie.**

**Rejected: `yamlpath` + `yamlpatch` (surgical, comment-preserving splice).**

Steelman, at its strongest - this is not a fringe option, it is **the research
corpus's own primary recommendation** ("`yamlpath` + `yamlpatch`, and keep
`yaml_serde` for reading", `yaml-roundtrip-landscape.md` §6). It is well
measured and it works: against the real `reference.yaml`, all seven patch
operations produced valid YAML with 12 comments in and 12 out, including
`Append` of a structured rule at correct 4-space indent, `Remove` of a rule
removing exactly one line, and `MergeInto` a flow mapping with flow style
retained. `Op::Replace` on a commented node preserves both leading and
trailing comments. `yamlpatch` speaks `yaml_serde::Value` natively, so the
patch currency is a type the tree already has. Applying a suggestion would be
byte-identical-except-the-edit, and the user's file would survive one-click
apply untouched. On the narrow question "can apply-suggestion be lossless",
the answer is yes, and it was demonstrated.

Rejected for three reasons, in order of weight:

1. **It splits save behaviour arbitrarily.** Applying a suggestion would
   preserve comments; dragging a rule would not. From the user's seat that
   distinction is unmotivated - both are "the GUI changed my profile".
2. **The corpus's own gate selects against it.** The recommendation is
   explicitly conditional (§6): *"If the GUI's edit surface turns out to be
   mostly 'change a scalar' and 'toggle a flag', this is comfortable. If it
   becomes 'restructure the rule tree', reconsider: at that point the profile
   arguably wants to be generated from the model into a fresh file with a
   house style"*. Spec 8.2's editor **is** rule-tree restructuring: a
   drag-reorderable rule grid with per-rule detail editing. The condition for
   the corpus's own escape clause is met, so this is not an override of the
   research - it is the research's conditional resolving to canonical.
3. **Cost is higher than the brief credits.** Per correction #2, `yamlpath`
   is tree-sitter-based; adopting the pair adds a second YAML parser to a
   tree that already has one.

**The landmine, recorded so it is not rediscovered.** `yamlpath` silently
follows aliases and returns the **anchor definition's** span. Measured, and
reproduced for this document against a profile with `defaults: &defaults` /
`changes: *defaults`:

```
route!["tracks","rules",0,"changes"]   source text at 133..157 is `changes: *defaults`
  query_exact  -> span (45, 75) = "default_track: true   # shared"   <- the ANCHOR, line 3
```

A GUI writing to `tracks.rules[0].changes` would rewrite the shared anchor and
silently change **every other rule aliasing it**, with no warning and no parse
error. Two nuances the brief omits: the behaviour is **query-mode dependent**
(`query_pretty` on a route *ending* at the aliased key returns the alias site;
routes going *deeper* follow the alias in both modes), and merge keys are
**not** followed (`rules[1].changes.default_track` through `<<: *defaults` ->
`ERR: mapping has no key 'default_track'`). Aliases yes, merge keys no. This
is not a reason the option was rejected - it is a hazard that would have had
to be engineered around had it won, and it is recorded because the option will
be proposed again.

**Rejected: split behaviour (preserve comments for foreign profiles, canonical
for editor-owned).** Steelman: it would protect a hand-authored profile the
user never opened in the editor, which is the file most likely to carry
valuable prose, while letting editor-created files be canonical. Rejected
because "editor-owned" is not a property of a file - there is no marker, and
inventing one (a `_muxsmith: generated` key) would need a profile-format
change (product-scope, `core-12-unknown-keys-are-errors` makes every key
load-bearing) to encode a distinction the user cannot see. It also does not
work: the moment a foreign profile is edited it must be written back, and the
question returns unchanged.

**Parity (SI-3): MATCH, and the reference implementation is more aggressive
than we are.** mkvtoolnix-gui saves its multiplex settings by deleting the
existing file and writing a fresh one from its in-memory model
(`~/Downloads/mkvtoolnix/src/mkvtoolnix-gui/merge/mux_config.cpp:566-576`):

```cpp
QFile::remove(m_configFileName);
auto settings = Util::ConfigFile::create(m_configFileName);
save(*settings);
settings->save();
```

There is no read-modify-write and nothing from the previous file's text
survives. Further, `ConfigFile::create` always constructs a `JsonConfigFile`
(`util/config_file.cpp:71-73`), i.e. new `.mtxcfg` files are **JSON, a format
with no comment syntax at all**; the INI reader survives only to open legacy
files (`:56-57`, sniffed from the first byte). The reference implementation in
this exact domain moved *away* from a comment-capable format for its saved
settings.

Classification: **MATCH** on save semantics (canonical rewrite from the
model). **Justified divergence** on format: Muxsmith keeps YAML because a
profile is a hand-authored declarative artifact (spec 4.1's reference example
is written by a human), where `.mtxcfg` is a GUI byproduct nobody hand-edits.
That divergence is what makes the save-surface note below necessary and gives
mkvtoolnix no analogue to copy: it has no comments to lose. Per
`proc-06-mkvtoolnix-parity`, mkvtoolnix-gui's *editor UX* is not a parity
target at all ("only muxing semantics/output are parity targets, not
input-time convenience guesses"); its *settings-file write semantics* are
comparable behaviour and are recorded here as such. No literal code or text
was taken.

**Save-surface note (the controller's addition, kept, with its wording
corrected).** The editor states once, at the save surface, what saving does.
A standing note, not a modal; no detection of whether comments are present
(that would need the parser to see them). The controller's proposed wording
was "saving rewrites the file canonically and does not preserve comments".
**Comments are the smaller half** and the note must not stop there. Measured
under D48, `reference.yaml` still comes back as 112 lines from 80: its map keys
re-sort, its flow style expands to block style, and three whole blocks
(`output`, `attachments`, `chapters`) disappear because every field in them sat
on its default. A note naming only comments would understate what the user is
about to see, and would be read as a defect report the first time someone diffs
their profile. The note names the whole behaviour: **the file is rewritten from
the model - comments, key order and formatting are not preserved, and fields
left at their default are not written back.** Two new Fluent keys, en+de (D47's
catalog table).

**Supersedes D22's stated reason.** D22 (`2026-07-10-plan-5-gui-design-decisions.md:36-37`)
coupled apply to the editor because "One-click apply means comment-preserving
YAML mutation; that machinery belongs to the editor (Plan 6), which owns
profile mutation anyway." Under D41 that premise dissolves: apply is
deserialize, mutate model, serialize canonical - no comment machinery exists
anywhere.

**D22's conclusion survives on a different, stronger reason**, recorded here
so the pairing is not re-opened as unmotivated: apply mutates the profile, and
the editor owns the in-memory model. If apply wrote to disk independently, a
click in the batch view would silently clobber unsaved editor state, or
resurrect a stale on-disk profile over the user's edits. Apply must go through
the same model the editor holds and mark it dirty. The coupling is **shared
mutable model ownership**, not comment machinery. Note that spec 8.2 places
apply-suggestion in the *batch view*, not the editor - the pairing is a
plan-scope pairing, never a UI-location one.

**Interface changes:** new public core API `profile::save::{to_string, to_file}`.
No wire-format change.

---

## D42: The editor's command surface - three new commands; the path-based `validate_profile` stays

**Decision.** Three commands are added to the `invoke_handler`
(`src-tauri/src/lib.rs:440-452`); the existing `validate_profile` is
**unchanged**.

| command | signature | notes |
|---|---|---|
| `load_profile` | `async fn load_profile(path: String) -> Result<ProfileDocument, IpcError>` | New. Returns the model **plus** its config diagnostics in one document. |
| `save_profile` | `async fn save_profile(path: String, profile: Profile) -> Result<(), IpcError>` | New. `profile::save::to_file`. |
| `validate_profile_model` | `async fn validate_profile_model(profile: Profile) -> Result<serde_json::Value, IpcError>` | New. Wraps the existing `validate::config_diagnostics(&profile)` (`profile/validate.rs:193`). |

All three are `async` and run on `on_blocking` (`src-tauri/src/lib.rs:73-79`),
but for **two different reasons**, and the distinction is recorded because it
is the thing an implementer would get wrong by pattern-matching:

- `load_profile` and `save_profile` touch the disk. That is the same reason
  every other `on_blocking` command in this shell has.
- **`validate_profile_model` does not touch the disk at all.**
  `config_diagnostics` is pure: `validate.rs`'s own doc comment says it
  "touches no filesystem beyond the profile itself" (`profile/validate.rs:20-21`),
  and there is no `fs::`, `File::` or `read_to_string` anywhere in
  `validate.rs` or `lint.rs`. It is on `on_blocking` because it is **CPU-bound
  work on every keystroke**: it compiles every regex and parses every template
  in the profile, and spec 7 puts it in the path of "every profile edit".
  Tauri 2 runs a non-`async` command on the application's main thread, so a
  plain `fn` here would stall the webview on each edit - exactly the reasoning
  the shell already records for `detect_mkvmerge` (`src-tauri/src/lib.rs:361-368`),
  and the cost is real enough that regex recompilation is already a tracked
  item (`core-84-regex-recompile`).

The distinction matters: `get_settings`/`set_settings` are deliberately
non-async main-thread commands despite doing real file I/O, because they are
trivial and rare. "Touches the disk" is not the criterion; "could stall the
webview" is. The `Err` case stays what it is everywhere else in this file: the
blocking task itself panicking. Expected failures are diagnostics in the
document, not `Err`.

`ProfileDocument` is `{ profile: Option<Profile>, diagnostics: Vec<Diagnostic> }`,
serialized through the existing `report::json` document machinery so its
`diagnostics` array is byte-identical in shape to the one `validate_profile`
already returns (`core-85-report-json-dry`: neither surface owns document
logic). On a `ParseError` the profile is absent and the single diagnostic
explains why,
mirroring `config_diagnostics_from_file`'s own short-circuit
(`profile/validate.rs:203-208`). One round trip, because the editor needs both
and a second call would let them disagree.

**`validate_profile(path)` is kept, not changed.** It has a live consumer:
`src/views/BatchView.vue:118` calls it with a picked path, and the batch view
has no model to send. Retargeting it would break Plan 5's shipped view for no
gain. The two commands are not redundant: one validates a file the user picked
by path, the other validates a model the user is editing. Both funnel into the
same `config_diagnostics`, so no logic is duplicated (spec 7: "all validation
lives in exactly one place").

**Rejected: one command with a `{Path | Model}` enum argument.**

Steelman: **two commands that must stay semantically identical forever are a
drift hazard, and this plan is creating one deliberately.** Spec 7 says "all
validation lives in exactly one place", and a believer would point that
sentence straight back at this decision: the moment someone adds a check to one
wrapper and not the other, the batch view and the editor disagree about whether
a profile is valid, and nothing in the type system notices. One entry point
with a discriminated argument makes that impossible by construction, and the
union is an honest description of the two input shapes the product genuinely
has.

Rejected because the drift it protects against cannot occur here: both wrappers
are three lines that call the *same* `config_diagnostics`, and the shared
funnel, not the shared command, is what spec 7's "exactly one place" is about -
`config_diagnostics_from_file` is already a wrapper over `config_diagnostics`
(`profile/validate.rs:203-208`) and the house has lived with that pair since
Plan 5.6 without drift. Against that, the union changes `validate_profile`'s
wire format, forcing a `BatchView.vue` change and an `e2e/smoke.spec.ts` mock
rewrite (`:191, :299, :349, :497`), and it has to be discriminated in TS at the
one place we most want the frontend dumb.

**Rejected: the editor writes a temp file and reuses `validate_profile(path)`.**

Steelman: **zero new IPC surface, and it reuses a command that is already
shipped, reviewed, mocked in the e2e suite and known to work.** "Reuse before
writing" is a house rule, and this is the only option that adds no wire format
at all. The editor already holds a path (it opened the file), so the temp file
has an obvious home next to it; and a validate that runs against real bytes on
disk is validating the same artifact the CLI would, which is a genuine fidelity
argument in a tool whose whole premise is that the profile file is the spec.

Rejected because it makes disk I/O a precondition of every keystroke-level
validation, and spec 7 puts validation in the path of "every profile edit". The
fidelity argument also does not survive contact: `config_diagnostics` is
path-free and pure (`profile/validate.rs:20-21`), so validating a temp file and
validating the model produce **identical** diagnostics - the disk round-trip
buys nothing and costs a write per keystroke. This is the shape the brief's gap
table names ("or it writes to disk before every check").

**The frontend still performs zero semantic validation** (spec 7,
`gui-08` neighbourhood). It holds the model as data, sends it, renders the
returned diagnostics. Its only local logic stays the UX affordance spec 7
sanctions by name: disabling Save while errors exist.

**Interface changes:** three new IPC commands; new `ProfileDocument` wire
shape; `Profile` becomes a wire type in both directions (it already derives
both halves, `profile/model.rs:17`).

---

## D43: `apply_suggestion` is written fresh in core; only `StructuredEdit` gains `Deserialize`

**Decision.** A new core function
`planner::apply_suggestion(profile: &Profile, config_path: &str, edit: &StructuredEdit) -> Result<Profile, ApplyError>`
returns a new `Profile` with the edit applied to the rule named by
`config_path`. The Tauri command forwards:

```rust
async fn apply_suggestion(profile: Profile, config_path: String, edit: StructuredEdit)
    -> Result<Profile, IpcError>
```

`StructuredEdit` gains `Deserialize` (`planner.rs:201`). **`Suggestion` and
`DiagCode` do not** (`planner.rs:231`, `report/mod.rs:40`).

**Rationale for the argument shape.** The frontend already holds a whole
`Suggestion` - `SuggestionCard.vue` receives `config_path` and `yaml_fragment`
today (`src/components/SuggestionCard.vue:6-9`) and the serialized suggestion
carries `edit` alongside them. Passing `config_path` and `edit` back means the
frontend **forwards two opaque fields it never interprets**; core does all the
interpreting, reusing `rule_index_of` (`planner.rs:2032`) to parse the path.
The alternative - taking the whole `Suggestion` - would force `Deserialize`
onto `DiagCode`, a large enum whose one-way core-to-GUI direction is
deliberate: making it constructible from the frontend would let the shell
synthesize diagnostics, which is exactly what `core-37-prose-free-core`
(count 11, the most-reinforced entry in the house files) and the
code-plus-params wire exist to prevent. `resolves` is not needed to apply an
edit; it should not have to be parseable to apply one.

`config_path` is treated as **the narrow thing it is**. `Suggestion.config_path`
is only ever `tracks[<N>].match` (`planner.rs:1437`, `:1516`) and is parsed
back by `rule_index_of` (`:2032`). It is **not** a general path, and it is not
`Diagnostic.config_path`, which is a different, general field. `ApplyError`
carries the one failure this can have: `rule_index_of` returning `None`, or an
index past the end of `tracks.rules` - a frontend bug, surfaced as an
`IpcError` code, never a silent no-op.

**Reuse, correctly targeted.** The brief directs "Reuse before writing: hoist
`apply_edit_to_first_rule` into the library". Per correction #1 that helper is
a fixture generator with the test's own match conditions hardcoded as string
literals; hoisting it would import a test fixture into production. **The real
reuse targets are elsewhere and are binding:**

- `rule_index_of` (`planner.rs:2032`) parses `config_path`. Do not re-parse it.
- The engine's own narrowing helper must be reused, not re-implemented.
  `core-44-suggestion-no-clobber` records that `with_rule_match` must use
  `or_insert` semantics and never overwrite an existing `exact`/`substring`
  key, because a `BTreeMap::extend` merge once widened an existing
  `track_name` substring and violated D6's "never relax" (Bug C). An
  independently written applier would reproduce that bug. Apply and the
  engine's simulation must narrow through the **same** code, or an applied
  suggestion stops being the thing that was simulated - which is
  `core-03-suggestion-verified-edit`'s whole guarantee ("an applied suggestion
  survives the next dry run").
- `core-33-suggestion-narrow-only` bounds what apply may do: narrow the
  conflicted rule's match only. Never reorder, never touch other rules, never
  relax.

**The applied result is validated through the normal path, not by this
command.** `apply_suggestion` returns the mutated model; the editor then runs
its existing `validate_profile_model` round-trip, which it does on every edit
anyway (spec 7). No compound apply-and-validate command: D6's acceptance
invariant already guarantees the suggestion introduces no new diagnostic *for
the batch it was computed against*, but the user's model may have moved since,
and the honest way to find out is the same validation every other edit gets.

**No-fix case.** `core-109-two-required-no-fix` records that two required rules
colliding on one track yield no suggestion at all, only the no-fix partition
report. The apply button does not exist for those; the diagnostics panel
renders the partition as it does today. This is not a gap to close.

**Rejected: keep the grammar one-way; the command takes profile + suggestion
core-side and re-plans.** Steelman: `StructuredEdit` never becomes
constructible from the frontend, so the closed grammar (D6) stays closed by
type, not just by convention - a real integrity property, and the brief names
this option explicitly. Rejected because re-planning inside apply means
re-running the whole planner (mkvmerge identification of the entire batch) to
recompute a suggestion the frontend is already holding, and the recomputed set
may not match what the user clicked if anything changed. The integrity concern
is answered without the cost: the grammar stays closed because it is a
`#[serde(tag = "kind")]` enum with four variants - an unknown tag fails
deserialization at the boundary - and because the applied result is validated.

**Interface changes:** new IPC command; `StructuredEdit` becomes a
bidirectional wire type; new `ApplyError` -> `IpcError` codes.

---

## D44: `ts-rs` generates the TS types, feature-gated, committed, with a CI drift check

**Decision.** `ts-rs` **12.0.1** (crates.io `max_stable_version` = 12.0.1,
verified 2026-07-15) generates TypeScript types from the Rust model. It
coexists with `schemars` rather than replacing it: **schemars describes the
on-disk YAML file format; ts-rs describes the IPC wire.** Two different
boundaries, two accurate descriptions, not redundancy.

**Feature-gated, out of the shipped tree.** `ts-rs` is an optional dependency
of `muxsmith-core` behind a `ts` feature; the derive is
`#[cfg_attr(feature = "ts", derive(TS), ts(export, export_to = "profile.ts"))]`
on each of the 20 model types. **One file, not twenty**: `export_to` without a
trailing `/` names a file rather than a directory (`ts-rs-12.0.1/src/lib.rs:208`),
so every type lands in a single `profile.ts` with no cross-imports - which the
corpus measured as the resulting shape. The path is resolved against
`TS_RS_EXPORT_DIR`, defaulting to `./bindings` (`:206-207`), so that variable
pins the destination in the same `.cargo/config.toml` `[env]` block that carries
`TS_RS_LARGE_INT` below - ts-rs's own documented pairing of the two:

```toml
[env]
TS_RS_EXPORT_DIR = { value = "src/bindings", relative = true }
TS_RS_LARGE_INT = "number"
```
Measured in the corpus: a default `cargo build` yields **0 occurrences of
ts-rs in `cargo tree`**, and the `TS` derive coexists with `JsonSchema`.
Version pin `ts-rs = { version = "12.0.1", optional = true }` - full
three-component version, caret semantics, matching `schemars = "1.2.1"` and
`serde_json = "1.0.150"` in the same manifest. (`ci-10-pin-everything` binds
toolchain and CI inputs, not Cargo dep syntax; the house's own manifest is the
pattern being matched here. `=`-pins in this tree are reserved for
dev-dependencies, e.g. `proptest = "=1.11.0"`.) License MIT, already in
`deny.toml`'s allow list.

**`TS_RS_LARGE_INT = "number"` is mandatory** - the second line of the `[env]`
block above, per ts-rs's own documented mechanism
(`ts-rs-12.0.1/src/lib.rs:86-87` shows exactly this pairing).
Without it `i64`/`u64`/`i128`/`u128` map to `bigint` (`src/lib.rs:596`,
default `"bigint"`). Per correction #5 this protects exactly one type -
`Scalar::Int(i64)` (`profile/match_expr.rs:23`) - but that type is the value
of every `exact` and `changes` entry, and `bigint` does not survive
`JSON.stringify`, so it would break the IPC wire at the most-used point in the
model. Measured: with the variable set, `Scalar` emits
`boolean | number | number | string` (the duplicate `number` is cosmetic).

**Generation entry point: `cargo test -p muxsmith-core --features ts`**, using
ts-rs's `#[ts(export)]`. **Two committed outputs**, both under `src/bindings/`:

| artifact | contents | producer |
|---|---|---|
| `profile.ts` | the 20 model types | ts-rs `#[ts(export)]` |
| `keywords.ts` | the four keyword domains as `as const` arrays | a ~12-line emitter in the same feature-gated export test |

`keywords.ts` exists because D45's `keywordOrBlock` widget needs the domains and
TypeScript cannot see them (the untagged enums project to `Block | string`), while
D46 makes the Rust constants the single source. ts-rs exports types, not values,
so `TS::export_all` cannot emit them and a few lines of `std::fs::write` do:

```rust
// same test, after ts-rs has exported the types
let dir = std::env::var("TS_RS_EXPORT_DIR").unwrap();   // one destination for both artifacts
let mut out = String::from("// @generated by `cargo test -p muxsmith-core --features ts`\n");
for (name, domain) in [
    ("FILENAME_KEYWORDS", FilenameCfg::KEYWORDS),
    ("SOURCE_KEYWORDS",   SourceCfg::KEYWORDS),
    ("CHAPTERS_KEYWORDS", ChaptersCfg::KEYWORDS),
    ("TITLE_KEYWORDS",    TitleCfg::KEYWORDS),
] {
    let items = domain.iter().map(|k| format!("\"{k}\"")).collect::<Vec<_>>().join(", ");
    out.push_str(&format!("export const {name} = [{items}] as const;\n"));
}
std::fs::write(Path::new(&dir).join("keywords.ts"), out).unwrap();
```

The drift check below covers both artifacts without modification, because it
gates the directory rather than a filename.

This **deviates from the house codegen pattern and the deviation is
deliberate.** The house generates via xtask (`cargo run -p xtask -- gen-capability
<schema.json> <out.rs>`, `crates/xtask/src/main.rs:1-3`, "Maintainer tooling
only; never invoked at build time"), and an xtask entry point would have been
the consistent choice. It is rejected on a correctness ground: xtask would need
`muxsmith-core = { features = ["ts"] }`, and Cargo unifies features across
workspace members within one invocation, so `cargo build --workspace` would
then enable `ts` for **every** consumer of core - putting `ts-rs` into the
muxsmith-cli and src-tauri builds and destroying the isolation this decision
depends on. `-p muxsmith-core --features ts` cannot leak that way. The cost is
that generation is a test side effect, which is genuinely surprising; it is
ts-rs's primary documented mechanism, and the CI check below makes a stale
binding a hard failure rather than a silent one.

**Committed + CI-checked**, confirming the controller's assumption with a house
precedent it did not cite: `core-06-schema-build-time-extraction` already
mandates "a committed generated.rs", by xtask, "no build.rs network
dependency". Committed generated artifacts are the established pattern; a
build-time generator would add a step to every contributor's first run and
break that pattern.

**The CI drift check is a NEW pattern and is surfaced as such** (brief §3.2).
No drift check exists in `.github/workflows/ci.yml` today, and `generated.rs`
**cannot** have one: its input is the mkvmerge identification schema, which
spec 9.1 and `core-06` say is never redistributed, so CI has no input to
regenerate from. The TS bindings are different - their input (the Rust model)
is in the repo - which is exactly why the check is possible here and not there.
New CI step, Linux leg only (matching the existing `check:i18n` and Playwright
gating):

```
cargo test -p muxsmith-core --features ts && git diff --exit-code src/bindings/
```

It covers every **tracked** file under the directory, so `profile.ts` and
`keywords.ts` alike, and any later artifact the same step emits once committed -
that is why the D45 keyword domains cost an emitter and no new infrastructure. A
stale array is a red CI leg naming the file.

**Its one hole, stated rather than papered over: `git diff --exit-code` does not
see a NEW untracked file.** Measured on a scratch repo:

| change to `src/bindings/` | exit |
|---|---|
| tracked file modified | 1 |
| tracked file deleted | 1 |
| **new untracked file** | **0** |
| that same file once committed, then modified | 1 |

So the gate cannot catch a first-generation artifact that was never committed.
That hole is narrow and already closed elsewhere: a missing `keywords.ts` fails
the TypeScript build on the registry's import of it, which `pnpm build` runs on
every leg. The gate's job is catching a *stale* committed artifact, and it does
that from the first commit onward. `git status --porcelain` would close the hole
directly and is not adopted - it would also fire on unrelated untracked files and
turn every CI leg into a working-tree cleanliness assertion, which is not this
step's business.

`.gitattributes`'s `* text=auto eol=lf` makes the committed output LF on every
platform, so the check is not line-ending-sensitive; it is Linux-only for
consistency with the other single-leg gates, not out of necessity.

**Rejected: `tauri-specta`.** Steelman: it is the de-facto standard for
typesafe Tauri commands, it is the only such tool Tauri core accommodates (a
`specta` feature flag exists in Tauri itself), it is actively developed
(commits on 2026-07-15), and unlike ts-rs it would type the **commands**, not
just the model - which is the larger share of the boundary this plan is
adding. Rejected on the pin-everything doctrine: its Tauri-2 line is
`2.0.0-rc.25`, and `2.0.0-rc.1` was published **2023-10-04** with rc.25
following on **2026-05-08** (crates.io versions API, verified 2026-07-15).
**Two years and seven months in release candidate**, with rc.25 declared the
final RC and rc.26 then adding new exporters. Its `max_stable_version` is
1.0.2, which is not the Tauri 2 line. Every adopter pins exactly because
rc-to-rc breakage is expected (Cap at `=2.0.0-rc.20`, Ferrum at
`=2.0.0-rc.21`, Spacedrive on a personal git fork). Tauri core ships the
feature flag but no generator and no documentation, and its own tracking issue
is labelled "priority: 3 low (Accepted but likely won't be worked on)". There
is no first-party Tauri 2 answer; ts-rs types the model, which is what this
plan needs, and leaves the command signatures hand-written as they are today.

**Rejected: `json-schema-to-typescript` (reuse the schema we already emit).**
Steelman: zero new Rust dependency, and it gets `Scalar` **right without any
bigint fix** (`boolean | number | string`), because JSON Schema has no i64.
The schema is already generated and shipping. Rejected because it produced 4
spurious duplicate types (`MatchExpr1`, `Locator1`, `ExternalBlock1`,
`TemplateBlock1`) with positional, unstable names - `TrackRule.match` typed as
`MatchExpr1` - requiring a hand-written normalizer to fix, and because it
carries 69 lines of Rust doc-comment prose into the `.d.ts`. It also aims the
wrong contract at the frontend: the schema describes the YAML file, and D47
makes that a user-facing artifact with its own consumers. Coupling the
frontend's types to it means one artifact serving two boundaries that can
legitimately diverge.

**Rejected: a hand-written TS mirror of the model.** Steelman: it is what
`src/ipc.ts` does today, deliberately and well ("mirrors the Rust structs
field-for-field", `src/ipc.ts:1-11`), so this would be the house pattern and no
tooling at all. Rejected because the existing mirrors are small, flat and
stable (`AppSettings`, `MkvmergeInfo`), whereas the profile model is 20 types
(13 structs + 7 enums across `model.rs` and `match_expr.rs`) with recursion
(`MatchExpr` -> `Vec<MatchExpr>`) and four untagged enums;
and because a hand mirror has no forcing function - adding a field to the Rust
model would leave the TS silently, structurally correct-but-incomplete, which
is the exact failure D45 exists to make impossible.

**Interface changes:** new generated wire-type artifact `src/bindings/profile.ts`;
new `ts` feature on muxsmith-core; new `.cargo/config.toml` `[env]` block; new
CI step.

---

## D45: Components stay hand-built; a `Record<keyof T, FieldSpec>` registry is the forcing function; sum types get a `never` arm

**Decision.** The editor's components are hand-built per spec 8.2 (rule grid,
drag-reorder, per-concept panels). The compiler's grip on completeness is a
**registry**, not the type.

**Registry shape.** One per edited struct, colocated with the component that
renders it:

```ts
/** A field the editor renders. */
interface EditableField { labelKey: string; widget: FieldWidget }
/** A field that exists in the model and is deliberately not exposed.
 *  `why` is a source comment, not user prose: nothing renders it. */
interface FixedField { readonly fixed: true; why: string }
type FieldSpec = EditableField | FixedField;

const outputFields: Record<keyof OutputCfg, FieldSpec> = {
  directory:    { labelKey: "editor-output-directory",    widget: { kind: "directoryPath", optional: true } },
  filename:     { labelKey: "editor-output-filename",     widget: { kind: "keywordOrBlock", keywords: FILENAME_KEYWORDS, block: "templateBlock" } },
  on_collision: { labelKey: "editor-output-on-collision", widget: { kind: "select", options: COLLISION_POLICIES } },
};
```

`FieldSpec` is a two-shape union, discriminated by `"fixed" in spec`, because
exactly one field in the model exists and must not be edited
(`Profile.profile_version`, which spec 4 fixes at `1`). Without `FixedField` it
has no legal entry, and the implementer would reach for `Omit<Profile,
"profile_version">` - which silently disables the forcing function for that key
forever. One instance justifies the variant because the alternative is wrong,
not merely larger.

### The three sets this registry ranges over, enumerated

The first draft of this ADR left these as `...`. That is a latitude clause no
keyword scan catches, and D48 four sections down states the standard this
failed: *"Enumerated rather than described, because 'the defaulted fields' is
exactly the kind of phrase an implementer has to guess at."* The same applies to
"each edited struct". All of it is closed below, in four parts. **(a) and (c) are derived; (b) is a
decision on a safe default; (d) follows from the three.** Which is which is now
stated per part rather than claimed for all of them, because an earlier draft
called every part derived and one of them is not.

**(a) Which structs get a registry: all 13. Which enums get a `never` arm: all 7.
Derived.**

This follows from the forcing function itself, with no appeal to the spec's view
of the editor's scope. The registry's entire value is total coverage: it fails
the build when a field has no decision recorded against it. Register only a
subset of the structs, and adding a field to an unregistered struct is silently
unnoticed - which is the exact failure D45 exists to prevent, reintroduced by the
mechanism meant to close it. `FixedField` is what makes total coverage
affordable: a struct can be fully registered while a field in it is deliberately
not exposed, and the non-exposure is written down instead of inferred from
absence. So every struct reachable from `Profile` is registered and every enum
gets a `never` arm: 13 + 7 = 20 = the whole model, no residue.

**(b) Which fields are editable: 42 of 43. A decision, not a derivation.**

Nothing in the spec restricts the editor's field scope. Spec 8.2 says "open/save
YAML, recent profiles" (`:373`) and nowhere says the editor creates profiles -
an earlier draft of this ADR claimed it did and attached the word "create" to
that citation, which does not carry it. `gui-02` is also thinner than it looks
here: it settles that there is no *per-file override*, i.e. that the profile is
the GUI's only lever, which is not the same as "the editor must expose every
field of it". And D47 blesses hand-authoring, so a field the GUI does not expose
is reachable by hand rather than stranded.

So this is a choice, and it is recorded as one. **Exposing all 42 is the only
option that cannot strand a field**, every alternative requires per-field
justification for the fields it hides, and `FixedField` carries the single
exception (`Profile.profile_version`, which spec 4 pins at 1) explicitly enough
to review. It lands on the safe side by construction: if the owner wants a field
hidden, that is a one-line change to a `FixedField` entry, visible in review -
whereas a field silently left out of a registry is invisible.

What spec 8.2's list *does* settle: it is illustrative of the notable surfaces,
not an exhaustive field list. An editor that could not set `input.pattern`, the
single most important field in the profile, would be an odd reading of "profile
editor" - so the list cannot be read as a closed enumeration. That refutes
"expose only what 8.2 names", which is all it needs to do; it does not by itself
select 42.

| # | struct | fields | registry |
|---|---|---|---|
| 1 | `Profile` | 9 | `profileFields` |
| 2 | `Meta` | 2 | `metaFields` |
| 3 | `Input` | 3 | `inputFields` |
| 4 | `OutputCfg` | 3 | `outputFields` |
| 5 | `TemplateBlock` | 1 | `templateBlockFields` |
| 6 | `ExternalBlock` | 1 | `externalBlockFields` |
| 7 | `TrackRule` | 4 | `trackRuleFields` |
| 8 | `Locator` | 6 | `locatorFields` |
| 9 | `AttachmentsCfg` | 2 | `attachmentsFields` |
| 10 | `TracksCfg` | 2 | `tracksFields` |
| 11 | `AttachmentRule` | 3 | `attachmentRuleFields` |
| 12 | `TagsCfg` | 2 | `tagsFields` |
| 13 | `MatchExpr` | 5 | `matchExprFields` |

**43 fields total; 42 are `EditableField`, 1 is `FixedField`.** So 42 is the
**registry label-key count** - not `gui-editor.ftl`'s total, which is 42 plus
D41's save-surface note (section 2 carries the breakdown). It is determinable at
all only because this table is.

The 7 enums taking a `never`-arm switch instead: `FilenameCfg`, `SourceCfg`,
`ChaptersCfg`, `TitleCfg` (the four keyword-or-block enums), `CollisionPolicy`,
`KeepDrop`, `Scalar`.

**(c) `FieldWidget`: 10 variants, closed. Derived.**

Ten, not eleven: an earlier draft counted `fixed` among them, and it is not a
`FieldWidget` at all - it is the other half of the `FieldSpec` union above, which
is the whole point of splitting them. The list below is what binds; the count is
its recount. (The label said 11 while the union defined 10 - the same defect this
document raised against the ROADMAP's "6 / 5 / 1 / 8", forty lines from where it
raised it. Recounted mechanically.)

Derived by walking all 42 editable fields and collapsing to the smallest set that
covers every one. No variant exists without a field that needs it, and no field
lacks a variant.

```ts
type TextSyntax = "plain" | "regex" | "templateLiteral" | "templateRegex";

type FieldWidget =
  | { kind: "text"; syntax: TextSyntax; multiline: boolean }
  | { kind: "bool" }
  | { kind: "optionalFlag" }                                    // checked -> Some(true), unchecked -> absent
  | { kind: "select"; options: readonly string[] }
  | { kind: "keywordOrBlock"; keywords: readonly string[]; block: RegistryName }
  | { kind: "directoryPath"; optional: boolean }
  | { kind: "stringList" }
  | { kind: "propertyMap"; properties: "matchable" | "settable"; values: "scalar" | "string" }
  | { kind: "list"; item: RegistryName; reorderable: boolean }
  | { kind: "section"; of: RegistryName; optional: boolean };
```

`RegistryName` is the union of the 13 names in the table above, so a widget can
only ever point at a registry that exists.

Three of these are settled by evidence rather than by the Rust type, and the
evidence is why they are what they are:

- **`optionalFlag`, not a tri-state.** `Locator.match_to_source` is
  `Option<bool>`, which suggests three states. It has two:
  `validate.rs:466-472` rejects `Some(false)` with `InvalidKeyword` and
  `allowed: "true"`, and the model's own doc says "the only valid value is
  `true`" (`model.rs:261-264`). So the widget is a checkbox whose off-state is
  **absence**. A tri-state control would offer a value the validator rejects.
- **`TextSyntax` has four values, not two.** The model's strings are not
  interchangeable: `Input.pattern` is a **regex** compiled directly;
  `TemplateBlock.template` is a **template in literal mode** (spec 4.8);
  `Locator.match_pattern` is a **template in regex mode** (spec 4.7,
  `model.rs:267-268`) - a genuinely third thing, and the one an implementer
  would most likely collapse into "regex" and get wrong. `Meta.*` is plain
  prose.
- **`propertyMap.properties` is `matchable | settable`.** Not cosmetic: `exact`
  offers the matchable domain (`capability::matchable_type`) and `changes`
  offers the settable one (`capability::settable`, a 10-entry curated table).
  They are different sets - `codec_kind` is matchable-only and never resolves as
  settable (`capability/mod.rs`'s own test asserts `settable("codec_kind") ==
  None`). One facet, two existing core lookups.

**(d) Every field's widget, closed.** All 43:

| struct | field | widget |
|---|---|---|
| `Profile` | `profile_version` | `fixed` (spec 4 pins it at 1) |
| | `meta` | `section { of: "meta", optional: true }` |
| | `input` | `section { of: "input", optional: false }` |
| | `output` | `section { of: "output", optional: false }` |
| | `tracks` | `section { of: "tracks", optional: false }` |
| | `attachments` | `section { of: "attachments", optional: false }` |
| | `chapters` | `keywordOrBlock { CHAPTERS_KEYWORDS, "externalBlock" }` |
| | `tags` | `section { of: "tags", optional: false }` |
| | `title` | `keywordOrBlock { TITLE_KEYWORDS, "templateBlock" }` |
| `Meta` | `name` | `text { plain, multiline: false }` |
| | `description` | `text { plain, multiline: true }` |
| `Input` | `pattern` | `text { regex, multiline: false }` |
| | `extensions` | `stringList` |
| | `recursive` | `bool` |
| `OutputCfg` | `directory` | `directoryPath { optional: true }` |
| | `filename` | `keywordOrBlock { FILENAME_KEYWORDS, "templateBlock" }` |
| | `on_collision` | `select { COLLISION_POLICIES }` |
| `TemplateBlock` | `template` | `text { templateLiteral, multiline: false }` |
| `ExternalBlock` | `external` | `section { of: "locator", optional: false }` |
| `TrackRule` | `source` | `keywordOrBlock { SOURCE_KEYWORDS, "externalBlock" }` |
| | `match_expr` | `section { of: "matchExpr", optional: false }` |
| | `optional` | `bool` |
| | `changes` | `propertyMap { settable, scalar }` |
| `Locator` | `path` | `directoryPath { optional: false }` |
| | `recursive` | `bool` |
| | `extensions` | `stringList` |
| | `match_to_source` | `optionalFlag` |
| | `match_pattern` | `text { templateRegex, multiline: false }` |
| | `case_sensitive` | `bool` |
| `AttachmentsCfg` | `unmatched` | `select { KEEP_DROP }` |
| | `rules` | `list { item: "attachmentRule", reorderable: true }` |
| `TracksCfg` | `unmatched` | `select { KEEP_DROP }` |
| | `rules` | `list { item: "trackRule", reorderable: true }` |
| `AttachmentRule` | `select` | `section { of: "matchExpr", optional: true }` |
| | `drop` | `section { of: "matchExpr", optional: true }` |
| | `add` | `section { of: "locator", optional: true }` |
| `TagsCfg` | `global` | `select { KEEP_DROP }` |
| | `track` | `select { KEEP_DROP }` |
| `MatchExpr` | `exact` | `propertyMap { matchable, scalar }` |
| | `substring` | `propertyMap { matchable, string }` |
| | `regex` | `propertyMap { matchable, string }` |
| | `any` | `list { item: "matchExpr", reorderable: false }` |
| | `not` | `list { item: "matchExpr", reorderable: false }` |

`reorderable` is semantic, not taste: `tracks.rules` is output track order
(spec 8.2, "list order = output track order") and `attachments.rules` resolves
first-match-wins in list order (spec 4.9), so both reorder. `any` is a logical
OR and `not` a logical NOR (spec 4.3) - order carries no meaning, so neither
does.

**Option arrays are declared once, with a compile-time completeness guard**, so
`select` cannot silently miss a variant added to a Rust enum:

```ts
const COLLISION_POLICIES = ["error", "skip", "overwrite"] as const satisfies readonly CollisionPolicy[];
type _CollisionComplete = Exclude<CollisionPolicy, (typeof COLLISION_POLICIES)[number]> extends never ? true : never;
const _collisionCheck: _CollisionComplete = true;
```

Same shape for `KEEP_DROP`. This is D45's own `never`-arm principle applied to a
value list rather than a switch, so it is the house rule of this ADR, not a new
idea.

**The four keyword arrays are GENERATED, not hand-written** - `FILENAME_KEYWORDS`,
`SOURCE_KEYWORDS`, `CHAPTERS_KEYWORDS` and `TITLE_KEYWORDS` are emitted into
`src/bindings/keywords.ts` by D44's generation step, from D46's Rust constants.
The registry imports them.

**Why TS cannot reach the domain on its own** (the constraint that forces
something here): ts-rs exports *types*, not values, and the four untagged enums
project to `Block | string` - the keyword domain is simply not in the TS type, so
no `satisfies` guard can see it, unlike `COLLISION_POLICIES` and `KEEP_DROP`
above which are checked against their own unions. Some second copy of those six
literals must exist on the TS side. The only question is what keeps it honest.

**An earlier draft of this ADR hand-wrote them and rationalised the gap. The
rationalisation is recorded here because it is instructive, and because the
argument is exactly the kind that gets rebuilt from scratch by the next reader.**
It claimed the drift risk was safe *because a keyword-domain change is a
profile-format change requiring a `profile_version` bump, hence never a one-line
slip*. **That is wrong, and its own citation refutes it**: spec 4 says
`profile_version` is "incremented on **breaking** format changes"
(`2026-07-08-muxsmith-v1-design.md:46`; `model.rs:20-21` says "only on breaking").
The argument silently substitutes *format change* for *breaking format change*.
**Adding** a keyword is not breaking - every existing profile still parses, a new
value merely becomes legal - so no bump is required and none of the coordinated-edit
machinery fires. Worse, D46's rider makes adding one a genuine one-line edit that
flows automatically to the guard, the `allowed` param and the schema, and **not**
to TS: the GUI would silently stop offering a keyword the format now accepts. The
claim holds only for removals, the rarer case. There is no asymmetry with D48 to
record; there was an unguarded second copy with a story attached.

**Generation, not a guard test, and the reason is the house's own doctrine.** A
guard (a test failing when the two disagree) would close the hole and is the
obvious fix. Generation is chosen over it because the house pattern here is
*derive the second copy from the first so the two can never drift* -
`capability::CODEC_KIND_NAMES` is derived from `CODEC_KINDS` with that exact
rationale in its doc comment (`capability/mod.rs:125-129`), and D46's rider cites
it as its own model. A guard detects drift; derivation makes it unrepresentable.
The same reasoning is applied to D48 rather than stopped here: an earlier draft
of this paragraph excused D48 from it on the claim that `schemars(extend)` "needs
a literal in an attribute position", which is **false** - it takes arbitrary
expressions, so D48 derives its seventeen too. Where derivation is available it is
what we use, without exception in this plan.

**The earlier costing of this option was wrong, which is why it lost.** It was
rejected as "a second generated artifact and a hand-written emitter... for six
literals". The emitter is real (~12 lines; `TS::export_all` emits no constants).
The "second artifact" is not a cost: D44 already creates `src/bindings/`, already
runs the generation step, and already gates it with `git diff --exit-code
src/bindings/`. That check covers `keywords.ts` **for free** the moment it lands
in the same directory - a stale array becomes a red CI leg naming the file. The
marginal cost is the emitter alone, which is less than the guard test it replaces
and strictly stronger than it.

**Still rejected: four Rust marker enums** purely so ts-rs can project their
unions. Steelman: it needs no emitter at all and gives TS a real union, so the
`satisfies` guard used for `COLLISION_POLICIES` would work here too, making all
six option arrays one uniform mechanism instead of two. Rejected because each
marker enum is a type the model does not use, existing only to be projected, and
it would itself need syncing to `KEYWORDS` - re-creating the same two-copies
problem one level up, with a type system that cannot check *that* either.

**What the registry does not force.** It forces a label and a widget to *exist*
for every field; it does not check that the widget suits the field's type
(`Record<keyof T, FieldSpec>` sees the key set, not `T[K]`). A mapped type
`{ [K in keyof T]: FieldSpecFor<T[K]> }` would add that, and is not adopted: the
brief settles the mechanism ("The forcing function is a `Record<keyof T,
FieldSpec>` registry, NOT the type"), and the property is worth less than it
looks - a mismatched widget is a **visible rendering bug** caught the first time
the panel is opened, whereas a missing entry is silent absence. Presence is the
property that needs the compiler; correctness announces itself.

**Cross-field constraints stay in core, per spec 7.** Two exist in this surface
and neither gets a widget: `AttachmentRule` requires exactly one of
`select`/`drop`/`add` (`AttachmentRuleShape`), and `Locator.match_to_source` is
mutually exclusive with `match_pattern` (`LocatorConflict`, `validate.rs:460-465`).
The registry is per-field by construction and cannot express either; both are
already validated core-side and surface as diagnostics. A component may present
the one-of as a mode selector - that is a UX affordance, which spec 7 sanctions
by name, not frontend semantic validation.

**Why the registry and not the type.** Measured against this project's own
tsc 6.0.3 (`node_modules/.bin/tsc --version` -> `Version 6.0.3`), re-run for
this document:

| pattern | new field added to the Rust model |
|---|---|
| reading a subset of a type | **no error** - structural typing makes reading a subset always legal |
| incomplete `Record<keyof T, FieldSpec>` | **`error TS2741: Property 'on_collision' is missing in type '{...}' but required in type 'Record<keyof OutputCfg, FieldSpec>'`** |

The registry fails the build and **names the missing key**, at the site where
the missing work belongs. One correction to the brief's phrasing: TS2741 is
the one-missing-property message; two or more missing keys report **TS2739**
listing them all. Same check, two messages - the design does not depend on
which fires. Note also that `Record<keyof T, V>` requires an entry even for
fields that are **optional** in the source type (`directory?: string | null`),
which is what we want: an optional model field still demands a deliberate
widget-and-label decision.

**Sum types get an explicit `never` arm.** For `MatchExpr`'s parts and the
keyword enums, the equivalent lever is a discriminated-union switch closed with
`const _exhaustive: never = x`. Measured, re-run for this document:

| shape | tsc 6.0.3, `--strict` |
|---|---|
| exhaustive switch, missing arm, **no** `never` arm (today's house shape, `src/jobRowState.ts:44-55`) | `error TS2366: Function lacks ending return statement and return type does not include 'undefined'.` |
| same, **with** a `never` arm | `error TS2322: Type '{ kind: "regex"; }' is not assignable to type 'never'.` |

Both fire. The `never` arm is chosen because **only TS2322 names the unhandled
variant** - which is the same property that justifies the registry over the
type, applied consistently. This is a deliberate, minimal improvement on the
existing house shape (`jobStateKey`), not a divergence from it: `jobRowState.ts`
is not required to change, and the `never` arm applies to the editor's new sum
handling.

**Rejected: a schema-driven form generator (JSON Forms 3.8.0 and the Vue
alternatives).** Steelman: it is the obvious answer. We already emit a JSON
Schema, D47 promotes it to a supported artifact, and a generator would mean the
form can never drift from the model because it is not written down twice -
strictly stronger than a registry, which still has to be maintained by hand.
Rejected on two measured grounds, both of which survive D46:

1. **`anyOf` is unrenderable for us.** Vanilla JSON Forms ships **no AnyOf
   renderer at all**, and our four keyword-or-block enums are `anyOf`. The
   alternatives fail worse: `vjsf` silently renders a text input for `anyOf`
   (and hard-pegs `vuetify ^4.1.0`); `@json-layout/core` has zero `anyOf`
   support and **silently drops the `$ref` branch**, collapsing
   `anyOf[{$ref},{string}]` to `type: 'string'`. `MatchExpr.any.items` is an
   unbounded self-reference, and `match` is the most-used part of a profile -
   so this is the whole UX, not an edge case.
2. **Generated labels bypass i18n by construction, not by omission.** JSON
   Forms derives combinator branch labels as
   `subSchema.title ?? resolvedSubSchema?.title ?? `${keyword}-${subSchemaIndex}``
   and **the function takes no Translator at all**. Measured output for our
   schema: `ChaptersCfg` branch labels are `['anyOf-0', 'anyOf-1']`;
   `KeepDrop` gives `['oneOf-0', 'oneOf-1']`. It fires on every combinator -
   four untagged enums plus every `Option<T>`. Field labels are no better: our
   schema carries no per-property `title` anywhere, so `Generate.uiSchema()`
   startCases the Rust identifiers ("Profile Version", "On Collision") and the
   help text is the raw Rust doc comment, brackets and spec references
   included. That is English prose out of a prose-free core
   (`core-37-prose-free-core`, count 11) hitting spec 8.4 head-on, and it
   cannot be intercepted. Measured separately: `eslint` flags a raw literal in
   a template but does **not** flag `{{ fieldSchema.title }}` - so all three
   i18n gates are bypassed silently.

**The brief's third argument against generation is deliberately NOT recorded
here**, per correction #7: "the schema cannot express our keyword domains" is
true today and **false the moment D46 lands, in this same plan**. Recording a
self-invalidating rationale is how a decision gets re-litigated. The two
arguments above are the load-bearing ones, and D46 does not touch either: the
four enums stay `anyOf` after D46 (only the string branch narrows to an
`enum`), and the combinator-label problem is in JSON Forms, not in our schema.
Anyone reopening this should attack (1) or (2).

Recorded with it, because the corpus is honest about it and the record should
be too: the ecosystem evidence here is suggestive, not decisive. Of 14 Rust
tools surveyed, zero drive a GUI from a JSON Schema; VS Code's settings UI
caps at scalar leaves and falls back to "Edit in settings.json" (issue #99635,
open since 2020-06-08); Zed abandoned a model-driven settings UI for a
hand-written registry - **but Zed's rejected approach was macro-based, not
JSON-Schema-based**, so it is not a direct refutation and is not cited as one.

**`check-i18n.mjs` learns the registry, in the same wave.** Per correction #6,
only check 1 is affected. The fix, dependency-free and matching the script's own
deliberate line-based approach ("PARSING CONSTRAINT (deliberate, line-based --
not a Fluent parser)", `scripts/check-i18n.mjs:102-120`): a second scanning
regex alongside `CALL_RE`,

```js
const LABEL_KEY_RE = /labelKey:\s*(['"])([^'"]*)\1/g;
```

applied to the same `src/**/*.{vue,ts}` sweep, with every match added to
`literalCallIds` and pushed to `missing` when it is not a known catalog id.
Check 2 needs no change: it already counts a key as used when it appears
anywhere in `src/` as a quoted string literal, single- or double-quoted (`:191-198`, the test at `:193`), which is precisely
the registry's shape and the reason `jobRowState.ts`'s identical pattern
already passes. Check 3 (cross-locale parity) is untouched.

Net effect: registry label keys become **hard-gated** (check-1 grade). This is
a **net gain over today**, not a trade - today's dynamic `$t(stateKey)` call
sites are only soft-checked, and the registry keys will not be. The brief's
"do not trade one gap for another" is satisfied because no gap opens.

**Interface changes:** none on the wire. New `scripts/check-i18n.mjs` scan; new
`locales/{en,de}/gui-editor.ftl` catalog (D47).

---

## D46: The four keyword domains get a `schemars(schema_with)` projection fed by one constant set

**Decision.** The `Keyword(String)` arm of each untagged enum keeps its
`String` for deserialization; its **schema projection is overridden** to a
closed `enum`. One constant set per enum feeds all three consumers.

Verified empirically for this document against schemars 1.2.1, on a faithful
replica of `FilenameCfg` and `ChaptersCfg`:

```
FilenameCfg -> anyOf: [ {$ref: TemplateBlock},
                        {description: "...", type: "string", enum: ["keep"]} ]
ChaptersCfg -> anyOf: [ {$ref: ExternalBlock},
                        {type: "string", enum: ["keep", "drop"]} ]

'keep'          -> Keyword("keep")
'kepp'          -> Keyword("kepp")      <- InvalidKeyword still reachable
template: 'X'   -> Template(TemplateBlock { template: "X" })
```

**Both properties, no trade** - the schema gains the domain and the diagnostic
keeps its `String`. The variant's doc comment survives as the branch
`description`, merged rather than replaced.

**Do not "fix" it by typing the arm.** Steelman, stated at strength: a typed
arm is what serde is *for*. It is what `KeepDrop` and `CollisionPolicy` already
do (`profile/model.rs:166-167`, `:181-182`), it makes the illegal state
unrepresentable in the model rather than merely diagnosed, it deletes four
match guards, and the schema then falls out correctly with **no override, no
`schema_with` function, and no constant set** - i.e. it is strictly simpler
than this decision and more consistent with the two real enums sitting beside
it. A reviewer flagged the `String` as a defect on exactly this reasoning
(Plan-1 final review minor #7), and that reading was reasonable.

Rejected because it destroys the diagnostic. Typing the arm makes
`filename: kepp` fail with serde's *untagged* error - `data did not match any
variant of untagged enum` - which does not name the keyword as the problem,
does not name `kepp`, does not name the legal values, and arrives as a
`ParseError` rather than the localized `InvalidKeyword` with its `found` and
`allowed` params (`locales/en/diagnostics.ftl:18`:
`invalid-keyword = Invalid keyword "{ $found }". Allowed: { $allowed }.`).
`KeepDrop`/`CollisionPolicy` can afford typing because they are **not** inside
an untagged enum, so serde reports them precisely; the `Keyword` arms cannot.
The trade is real; it was simply never recorded, which is why it read as a
defect.

**The rider that makes it a single source.** Today the keywords exist as bare
literals in four match guards (`profile/validate.rs:105`, `:129`, `:149`,
`:166`), plus four hand-typed `allowed` params, plus four doc comments. Spec 7
requires the schema and validation to live in exactly one place each; they do
not. One constant per enum, in `profile/model.rs` beside the enum it belongs
to:

```rust
impl FilenameCfg { pub const KEYWORDS: &'static [&'static str] = &["keep"]; }
impl SourceCfg   { pub const KEYWORDS: &'static [&'static str] = &["primary"]; }
impl ChaptersCfg { pub const KEYWORDS: &'static [&'static str] = &["keep", "drop"]; }
impl TitleCfg    { pub const KEYWORDS: &'static [&'static str] = &["keep", "clear"]; }
```

Three consumers, one source: the guard (`k if FilenameCfg::KEYWORDS.contains(&k.as_str())`),
the `allowed` param, and the `schema_with` projection.

**Placement rationale.** `profile::model`, not `capability`. The `capability`
module is the **mkvtoolnix** model ("mkvtoolnix capability model (spec 4.4 / 9)",
`capability/mod.rs:1`) - generated matchable properties, curated settable
tables, codec_kind aliases. `keep`/`drop`/`clear`/`primary` are **profile-format
vocabulary** (spec 4.8/4.9), not mkvmerge facts; they would be the only
non-mkvtoolnix thing in that module. The *shape* is copied from the house
pattern there deliberately: `capability::TYPE_VALUES` is a `pub static
&[&str]` closed domain read by a lookup function (`capability/mod.rs:55`,
read by `matchable_domain` at `:63-69`), and `CODEC_KIND_NAMES` is derived
from `CODEC_KINDS` rather than hand-re-listed, with the doc comment "so the
two can never drift" (`capability/mod.rs:125-129`) - the exact principle this
rider applies.

**The `allowed` param reuses `domain_hint`, and the refactor is provably
behaviour-preserving.** `domain_hint` (`profile/validate.rs:430-437`) is the
house's existing renderer for "a closed value domain -> the `allowed` param";
it joins with `", "` for domains of 8 or fewer and truncates beyond. Our
domains are 1 to 2 values, so it is a plain join. The four hand-typed strings
today are `"primary"`, `"keep"`, `"keep, drop"`, `"keep, clear"`; the
const-derived values are byte-identical. **No catalog change, no new Fluent
key, no user-visible change** - the existing snapshot tests
(`crates/muxsmith-cli/tests/snapshots/`) prove it, and any diff in them means
the refactor is wrong.

This also conforms to D39, which is the entry that governs this param: D39
removed English **prose** from `allowed` while explicitly leaving the closed-domain
emitter alone because "its `domain_hint` lists are locale-neutral value tokens,
not prose". Keyword tokens are the same category.

**The trigger this item was parked behind would never have fired.** It was
"a GUI generating an editor from the schema" - and D45 rejects schema-driven
generation, so on its own terms this fix would have waited forever. The real
reason it lands now is D47: the schema becomes a **shipped user artifact**, and
a user artifact that says "string" where two values are legal is wrong at the
surface the user actually reads. This is the ADR-side record of that reasoning;
**the ROADMAP already carries the same correction** (`docs/ROADMAP.md:52-55`,
landed in `fdcdcba`), so this paragraph concurs with a fix already made rather
than requesting one. Kept because the tracker records the conclusion and this
records why.

**Rejected: `oneOf` with per-value `const`, matching `KeepDrop`'s generated
shape.** Steelman: it is what schemars itself emits for a real enum four lines
away in the same file, so the schema would be internally uniform in **form** as
well as in content - and `oneOf`+`const` is the only standard way to attach a
per-value description, which matters more now that D47 makes the schema
user-facing. Rejected because we have no per-value prose to attach: `KeepDrop`'s
descriptions come from doc comments on its variants, and the `Keyword(String)`
arm has one doc comment for the whole domain, not one per value. Inventing
per-value English descriptions would be new user-facing prose authored in core
(`core-37-prose-free-core`) to fill a slot nobody asked for. `enum` says
exactly what is known and no more, and `yaml-language-server` completes from
both forms.

**Interface changes:** the published JSON Schema changes shape for four types.
This is a user-visible artifact change under D47 and is a **strict narrowing**
- every document that validated before still validates, unless it used an
illegal keyword that `validate.rs` already rejected. No profile that Muxsmith
accepts today stops validating.

---

## D47: The schema is a supported user artifact; it reaches the editor by file, and the binding lives outside the profile

**Decision.** `muxsmith schema` (spec 8.1, already shipping) becomes a
**supported user feature**: users generate the schema to a file and point
`yaml-language-server` at it for autocompletion and in-editor validation while
hand-authoring profiles.

**Delivery: redirect the existing command; bind in editor settings, not in the
file.** The README gains a section:

```
muxsmith schema > muxsmith-profile.schema.json
```

and documents binding it **in editor configuration** (VS Code `yaml.schemas`
mapping a glob such as `*.muxsmith.yaml`; the equivalent `lspconfig` settings
block for Neovim/Helix), rather than via the in-file modeline
`# yaml-language-server: $schema=...`.

**That choice is forced by D41 and the interaction is recorded because it is
not obvious.** The modeline is a **YAML comment**. D41 does not preserve
comments. A user who wires up autocompletion with a modeline and then saves
once from the GUI **loses their schema binding silently** - the file still
works, the editor just stops helping, with no message. Recommending the
settings route keeps the binding outside the artifact the GUI rewrites. The
README states the modeline consequence explicitly rather than leaving the user
to discover it; this is documentation, not machinery.

**Rejected: publish to SchemaStore / host the schema at a URL.** Steelman: it
is the zero-configuration answer - SchemaStore is wired into VS Code and most
LSP clients by default, so a filename glob would light up autocompletion with
no user action at all, and it is how a schema normally reaches an editor.
Rejected because it needs a stable public URL and a catalog entry, which is a
new externally-hosted artifact with its own lifecycle and version-skew problem
(a hosted schema tracks a release, the user's binary may not), and because
`core-07-runtime-fetching-rejected` records the owner rejecting network
dependencies in this exact area. A locally generated file always matches the
binary that generated it. Revisit at 1.0 if it earns its way in; named as a
trigger in section 9.

**Rejected: the GUI writes the schema out on startup.**

Steelman: **telling a GUI user to open a terminal is a broken handoff.**
Muxsmith ships a GUI precisely so a user need not touch the CLI, and D47 as
decided asks exactly that user to run `muxsmith schema > file` in a shell to
get autocompletion in a third tool. A believer would go further and note the
GUI knows the profile's directory, so it could drop the schema right beside the
profile where a relative binding resolves with no absolute paths and no
per-machine editor configuration - and the file would always match the running
binary, which the manual route cannot guarantee after an upgrade.

Rejected because it writes an unrequested file into a directory the user did not
ask us to touch, on every startup, for a feature most GUI users will never
enable - a side effect that costs every user to serve some. The version-skew
half of the steelman is real but is answered by the trigger in section 7 rather
than by a startup write.

Not offered as a reason: that the population is small. The tempting form of that
argument ("a user who never opens a terminal is not configuring an LSP either")
is **false for the path this ADR actually recommends** - VS Code's
`yaml.schemas` is configured through the settings UI, so a shell-free user
reaching autocompletion via VS Code is exactly the case the steelman describes,
and it is the primary path above, not a fringe one. The rejection stands on the
unrequested-write argument alone, which does not depend on how many users are on
the other side.

**The English-prose boundary, surfaced deliberately** (brief §3.2, "surface any
deliberate deviation"). The schema carries Rust doc comments as `description`
fields - measured: `"The top-level profile document (spec 4): ... (spec 4:
\"Unknown keys are errors, not warnings\")."`. Promoting the schema to a
supported user feature makes those descriptions **user-facing**, in
`yaml-language-server` hover tooltips, and spec 8.4's "no hardcoded
user-facing strings in any layer: not in the frontend, not in the CLI, not in
core" does not list them as an accepted exception.

**Decision: accept, English-only, and record the boundary here.** The
reasoning: the schema documents a **file format**, which is the same category
as the README and the spec, both English-only by design; it is not application
UI and not a diagnostic. Fluent governs diagnostics and UI, both of which stay
fully localized. The schema already ships with these descriptions today under
spec 8.1 - D47 does not add prose, it changes who reads it. Localizing it would
mean a per-locale schema artifact and a translation pipeline for
developer-facing format documentation, which is not earned. This is a deliberate
boundary, not an oversight: **spec 8.4 gains an explicit exception entry** (see
section 8) so a future reviewer does not read the schema's descriptions as a
violation - which is exactly what would otherwise happen.

**Note the asymmetry with D45**, since the two look contradictory and are not.
D45 rejects generated form labels partly *because* they would be English doc
comments; D47 accepts English doc comments in the schema. The distinction is
the consumer: a **form label in the app's own UI** is application prose, which
spec 8.4 governs absolutely and which we control; a **description in a file-format
schema read by a third-party language server** is format documentation, in the
same category as the README. Stated explicitly because a reviewer will
otherwise, correctly, notice the tension.

**Parity (SI-3): not applicable, and that is the finding.** mkvtoolnix has no
analogue. Its `.mtxcfg` is a GUI byproduct with no published schema and no
hand-authoring story (`util/config_file.cpp:71-73` - JSON, created fresh),
because mkvtoolnix-gui is **interactive**: the user's input surface is the GUI
itself. Muxsmith is **declarative batch** - the profile *is* the spec, so its
file format has a hand-authoring surface that mkvtoolnix structurally does not
have. Classified: **genuine divergence in kind, no parity target exists**.

**Interface changes:** none in code. New README section; the schema's shape
changes per D46.

---

## D48: A canonical save omits fields sitting on their default

**Decision** (owner ruling 2026-07-15, resolving the escalation this document
originally raised as E1). Saving does **not** emit a field whose value equals
its serde default. Measured on `reference.yaml`:

| | lines | comments |
|---|---|---|
| hand-authored original | 80 | 12 |
| canonical, **emitting** defaults | 141 | 0 |
| canonical, **omitting** defaults (this decision) | **112** | 0 |

**Rationale.** D47 makes hand-authoring a **supported** workflow in this same
plan: the user points `yaml-language-server` at the schema and keeps editing
the profile by hand. Emitting defaults sets the two supported workflows against
each other - the GUI hands back an 80-line file as 141 lines, and the user
whose authoring surface we just blessed is the one who pays. Omission keeps
them compatible.

The supporting evidence, recorded unflattened because it genuinely split and a
future reader deserves to see that it did rather than a tidy story:

- **For omission.** `core-54-reuse-plan2-machinery` - "defaults match spec 4.9
  so an omitted section never silently drops data": the parse-side contract
  already guarantees omission is lossless, so this decision leans on a
  guarantee the house already made. `core-38-absent-bool-equals-false` is the
  house's one recorded ruling on absent-vs-explicit-false and ruled them
  **equivalent** (cited as analogy only - its scope is matchable track
  properties, not profile struct fields; not overclaimed).
  `core-45-yaml-fragment-serializer` is the only other recorded YAML-emission
  decision, and what it chose to show the user is a **delta**, not a
  materialized struct. Spec 4.1's reference example, the documented showcase of
  the format, omits `source` and `optional` on every rule.
- **For emission, and it was not weak.** `core-83-zero-rule-keep-passthrough`
  is an owner ruling that a **non-obvious default MUST be documented and
  hinted**, not left implicit. `core-05-global-toggles-rejected` records the
  owner overruling an agent's smaller-surface recommendation in favour of full
  explicit declarative control.
  `exec-46-runlog-keepforever-prunefacility-rejected` records Peter's
  recommendation losing on a user-artifact question precisely because an
  abstract engineering principle was carrying it.

**Rejected: emit default-valued fields (today's derive behaviour).**

Steelman, at full strength, because this is the alternative most likely to be
reconstructed later. **An explicit file is self-documenting, and that is worth
real money in this product specifically.** A rule that reads

```yaml
- source: primary
  match: { exact: { type: audio, language: en } }
  optional: false
```

tells the reader what it does without knowing anything. Under omission the same
rule is three keys shorter and the reader must know, or look up, that an absent
`source` means `primary` and an absent `optional` means required - and getting
`optional` wrong is the difference between a batch that fails on a missing
track and one that quietly skips it. This is not hypothetical fussiness: it is
`core-83`'s exact reasoning, from the owner, that a non-obvious default must be
surfaced rather than implied, and `tracks.unmatched` is the sharpest case -
absent means `drop`, i.e. absent means *destructive*. Emission also costs
**zero** new code: no predicates, no `skip_serializing_if`, no `schemars(extend)`
to keep the schema honest, and no guard test. It is what the derive already does,
and it cannot be got wrong, whereas omission has a data-loss failure mode (below)
that has to be engineered against. (An earlier draft of this steelman also
credited emission with avoiding a drift surface the `extend` annotation
introduced. That point is now void - the annotation is derived, not written
twice, so it introduces none. The argument is left out rather than left standing:
a steelman resting on a claim that has since become false is not a strong
argument, it is a stale one.) The honest summary: emission is safer, simpler, and
better documentation; it is worse only at the one thing this plan happens to care
about, which is not
fighting the hand-authoring workflow D47 blesses.

Rejected on that last point, which the owner ruled decisive. Recorded so that
anyone reopening this argues against the reason it lost, not against a
caricature: the case for emission is good, and it lost to a specific
interaction with D47, not to a general principle that explicit is bad.

**Rejected: a `--canonical` / `--minimal` option.** Steelman: it dissolves the
disagreement, the evidence really does split, and both behaviours have a
legitimate constituency (a team wanting self-documenting profiles in review vs
a solo author wanting terse files). Rejected on the recorded precedent:
`exec-45-runlog-config-deferred` parked a run-log config surface as "option
surface not earned", which is this situation exactly. Shipping an option here
also doubles the save behaviours every later change must be correct against,
for a question that has one right answer per this product's workflow.

**Rejected: omit by post-processing the value tree in `save::to_string`
(walk the emitted tree, delete entries matching a default table) instead of
touching the derives.** Steelman: it leaves the model's `Serialize` impl fully
faithful, keeps the omission policy in one readable function next to the writer
that owns it, and leaves the published schema untouched - which, given the
schemars interaction below, is a real advantage and would have avoided the
`extend` annotation entirely. Rejected because **a text-or-tree walker cannot
tell two identically-named fields apart**. `unmatched` appears twice with
**opposite** defaults - `tracks.unmatched` defaults to `drop`, `attachments.unmatched`
defaults to `keep` (spec 4.9's deliberate asymmetry) - so the walker needs a
path-keyed table that duplicates the model's structure and drifts from it
silently the first time a field moves. serde already knows each field's
identity; a walker has to be told, and told again. (Found the hard way: the
first probe written for this measurement was a line filter, and it both missed
`- source: primary` because the list dash breaks the key match, and mishandled
the two `unmatched` fields.)

### The mechanism, named exactly

**Every predicate calls the very function the `default` attribute names.** This
is not a style preference; it is what makes the pairing incapable of drifting:

```rust
// 13 fields whose serde default IS Default::default()
fn is_default<T: Default + PartialEq>(v: &T) -> bool { *v == T::default() }
#[serde(default, skip_serializing_if = "is_default")]

// the 4 fields whose serde default is NOT Default::default()
fn is_default_true(b: &bool)      -> bool { *b == default_true() }
fn is_primary(s: &SourceCfg)      -> bool { *s == SourceCfg::primary() }
fn is_keep_filename(f: &FilenameCfg) -> bool { *f == FilenameCfg::keep() }
fn is_drop_policy(k: &KeepDrop)   -> bool { *k == drop_policy() }
```

**Of the four divergent fields, two fail silently and two are caught by the
compiler.** The split matters, because only the first pair is a hazard a
reviewer has to watch for:

| field | serde default | naive `is_default` does |
|---|---|---|
| `TracksCfg.unmatched` | `drop_policy()` | **compiles, destroys data** |
| `Input.recursive` | `default_true()` | **compiles, inverts the value** |
| `OutputCfg.filename` | `FilenameCfg::keep()` | **`E0277`, does not compile** |
| `TrackRule.source` | `SourceCfg::primary()` | **`E0277`, does not compile** |

`FilenameCfg` and `SourceCfg` have **no `Default` impl at all** - only the
associated constructors `keep()` (`model.rs:155-161`) and `primary()`
(`:232-238`); neither derive line includes `Default` (`:143`, `:221`). So
`is_default<T: Default + PartialEq>` cannot be instantiated for them. Measured:

```
error[E0277]: the trait bound `FilenameCfg: Default` is not satisfied
note: required by a bound in `is_default`
```

That is good news and worth stating plainly: **two of the four cannot reach
data, because the type system stops them.** The carve-out does not need the
overstatement to be justified - it is justified by the two that *do* compile,
and the worst of those destroys an owner-ruled-legal profile.

**The genuinely silent one.** `KeepDrop::default()` is `Keep`
(`profile/model.rs:186-187`), but `tracks.unmatched` defaults to `Drop` via
`drop_policy()` (`:306`, `:314-316`) - spec 4.9's documented asymmetry. Both
types line up, so the naive pairing compiles cleanly and omits `unmatched:
keep`, which reloads as `drop`. Demonstrated, not reasoned about:

```
model in : TracksNaive { unmatched: Keep, rules: [] }
saved as : "rules: []\n"
reloaded : TracksNaive { unmatched: Drop, rules: [] }
PRESERVED? false   <-- keep silently became drop
```

That is precisely the `core-83` pure-passthrough profile (zero rules +
`unmatched: keep`), which the owner ruled a **legal** remux. One GUI save would
turn it into a `NoTrackRules` **error** - the owner's own ruled-legal profile,
destroyed by its own editor. With the predicate above, the same profile saves
as `unmatched: keep\nrules: []` and round-trips intact.

`Input.recursive` is the second silent one and fails the same way in miniature:
its default is `true`, so a generic predicate omits `recursive: false` and the
value reloads inverted. Same shape, smaller blast radius - a recursive walk the
user turned off, turned back on.

**Full field set: 17.** Enumerated rather than described, because "the
defaulted fields" is exactly the kind of phrase an implementer has to guess at.
**The `serde default` column is the single source for all three attributes** on
each row: it is what `#[serde(default = ...)]` names, what the predicate
compares against, and what the schema annotation serializes
(`extend("default" = to_value(<that column>())`). One function per row, three
mentions, no copies:

| location | field | serde default | predicate |
|---|---|---|---|
| `Profile` :30 | `output` | `Default` | `is_default` |
| `Profile` :37 | `attachments` | `Default` | `is_default` |
| `Profile` :40 | `chapters` | `Default` | `is_default` |
| `Profile` :43 | `tags` | `Default` | `is_default` |
| `Profile` :47 | `title` | `Default` | `is_default` |
| `Input` :82 | `recursive` | **`default_true`** | `is_default_true` |
| `OutputCfg` :100 | `filename` | **`FilenameCfg::keep`** | `is_keep_filename` |
| `OutputCfg` :105 | `on_collision` | `Default` | `is_default` |
| `TrackRule` :201 | `source` | **`SourceCfg::primary`** | `is_primary` |
| `TrackRule` :211 | `optional` | `Default` | `is_default` |
| `Locator` :256 | `recursive` | `Default` | `is_default` |
| `Locator` :273 | `case_sensitive` | `Default` | `is_default` |
| `AttachmentsCfg` :285 | `unmatched` | `Default` | `is_default` |
| `AttachmentsCfg` :291 | `rules` | `Default` | `is_default` |
| `TracksCfg` :306 | `unmatched` | **`drop_policy`** | `is_drop_policy` |
| `TagsCfg` :361 | `global` | `Default` | `is_default` |
| `TagsCfg` :364 | `track` | `Default` | `is_default` |

The five `Profile`-level struct fields are **not optional extras**: without
`skip_serializing_if` on them, a fully-defaulted `OutputCfg` serializes as
`output: {}` rather than disappearing. The struct-level skip is what makes the
block vanish, and the vanishing is the correct reading of the ruling applied
uniformly - a field sitting on its default is not emitted, and `output` is a
field.

**Consequence, recorded because it surprises: whole blocks disappear.** On
`reference.yaml`, `output:`, `attachments:` and `chapters:` vanish entirely -
every one of their fields was at its default. The user wrote an `output:` block
with three keys and gets none back. This follows directly from the ruling and
is not a defect, but it is the single most visible thing about a first save and
the save-surface note (D41) is worded to cover it.

**Not everything collapses**, which is the sanity check that the predicates are
per-field and not blanket: `tags:` survives carrying only `global: drop`
(because `KeepDrop::default()` is `keep`, so `drop` is not the default and
`track: keep` is), and `title: clear` survives (because `TitleCfg::default()`
is `keep`). Verified in the measured output.

### The schemars interaction, and why it is not left as a regression

`skip_serializing_if` **strips the `default` keyword out of the published JSON
Schema.** This is not a schemars bug; it is a deliberate rule
(`schemars_derive-1.2.1/src/schema_exprs.rs:788-800`), which evaluates the
predicate against the default value itself and emits no `default` annotation
when it matches:

```rust
let default_expr = if let Some(skip_if) = field.serde_attrs.skip_serializing_if() {
    quote! { { let default = #default_expr;
               if #skip_if(&default) { None } else { Some(default) } } }
```

Left alone, this would silently delete **all 17** `default` annotations from
the artifact D47 promotes to a supported user feature - in the same plan in
which D46 is adding domain information to it. It would also be perverse: under
D48 defaults are *absent from the file*, which is exactly when a reader most
needs the schema to tell them what an absent field means.

**Fix: `#[schemars(extend("default" = ...))]` on each of the 17 fields, and the
annotation is DERIVED, not written a second time.** `extend` takes an arbitrary
expression, not just a literal (`schemars_derive-1.2.1/src/lib.rs`'s attribute
docs: "This attribute also accepts arbitrary expressions"), so each field's
annotation calls the very function its own `default` attribute names - the same
rule the predicates follow, applied to the third consumer:

```rust
// the 13 whose serde default is Default::default()
#[serde(default, skip_serializing_if = "is_default")]
#[schemars(extend("default" = serde_json::to_value(CollisionPolicy::default()).unwrap()))]
pub on_collision: CollisionPolicy,

// the 4 whose serde default is a named function - same function, three times, zero copies
#[serde(default = "drop_policy", skip_serializing_if = "is_drop_policy")]
#[schemars(extend("default" = serde_json::to_value(drop_policy()).unwrap()))]
pub unmatched: KeepDrop,
```

So **there is no second copy of any default anywhere in D48**, and no drift
surface for a guard to watch. Measured, restoring the annotation while
serialization still omits the default and the round-trip still holds:

```
control (no skip)              : "unmatched": { "$ref": "#/$defs/KeepDrop", "default": "drop" }
skip + derived schemars(extend): "unmatched": { "$ref": "#/$defs/KeepDrop", "default": "drop" }
all-default save               -> "{}"
all-non-default save           -> "unmatched: keep\nflag: true"   (round-trip preserved: true)
```

This **restores the status quo; it does not invent a shape.** The `$ref`-plus-`default`
sibling pattern is already what the schema emits today (`chapters` carries
exactly `{"$ref": "#/$defs/ChaptersCfg", "default": "keep"}`), so no new
`$ref`-with-siblings hazard is created by this decision.

**Three of the seventeen do not restore the control byte-for-byte, and that is
correct rather than a shortfall.** The struct-valued fields - `Profile.output`,
`Profile.attachments`, `Profile.tags` - derive to `"default": {}` where the
pristine schema carried the fully materialized object. Measured on a faithful
replica:

| | `Profile.output.default` | `$defs/OutputCfg.on_collision.default` |
|---|---|---|
| pristine (no skips) | `{"on_collision":"error"}` | `"error"` |
| D48, derived | `{}` | `"error"` |

The cause is D48 itself: once `OutputCfg`'s own fields skip their defaults,
`to_value(OutputCfg::default())` *is* `{}`. Both annotations are true - omitting
`output` and writing `output: {}` both deserialize to `OutputCfg::default()` - and
**no information is lost, because the parent's object was always redundant with
the children's own annotations**, which survive in `$defs` (right-hand column,
verified). A reader asking what an absent `output` means drills into
`$defs/OutputCfg` and finds `filename: keep`, `on_collision: error` exactly as
before.

The alternative was to hand-write those three literals to preserve the pristine
bytes. Rejected: it would reintroduce, for cosmetics, precisely the second copy
this section exists to eliminate - three hand-maintained objects restating three
`Default` impls, unguarded, to restore information that is already one hop away.
Uniformity is worth more: **every one of the seventeen follows one rule with no
exceptions**, which is a rule an implementer can apply without judgement.

### Guards, because the failure mode is silent data loss

Two tests, both mechanical, both mandatory.

**Guard 1: round-trip fidelity on an all-non-default fixture.** A new fixture
setting every one of the 17 fields to a **non-default** value must round-trip to
an equal model. This is what catches a predicate that skips a value which is not
the default - the `core-83` passthrough class of bug - and it catches it for all
17 at once. The existing `reference.yaml` round-trip (verified passing today) is
the complementary half and stays.

**Guard 2: schema-default honesty.** A table test asserting, for each of the 17
fields, that the schema's `default` equals `serde_json::to_value` of that field's
serde default. It follows the house's existing table-test shape
(`capability/mod.rs`'s `settable_maps_to_mkvmerge_options` asserts a `const
EXPECTED` table against the real thing, length first, then row by row) rather
than inventing a pattern.

**Guard 2 is retained under the standing rule that a safeguard a plan proposed
stays until it is built** (owner ruling, 2026-07-15: a guard, test, enumeration
or check a design document has proposed is not argued out again during design or
planning; it is removed only after it is built and *measured* redundant). It is
retained **despite** the analysis below, not in ignorance of it - and the
analysis is kept, re-aimed, because it is the reason guard 2 is a candidate for
removal later rather than a permanent fixture.

**The analysis, and why it does not license removal now.** An earlier draft
dropped guard 2 on this reasoning: once the annotation is *derived from the same
function the test compares against*, the test asserts `to_value(F()) ==
to_value(F())` - a tautology that can never fail, and a test that cannot fail is
worse than none because it reads as coverage. That reasoning may well be right.
It is not actionable yet, for a reason specific to this claim:

**"This test cannot fail" is the one assertion in this design that the design
phase cannot check.** Every other suspect claim in this document was settled by
running something - `extend` turned out to take an expression, the drift gate
turned out to miss an untracked file, the test helper turned out to take no
`Profile`, deriving turned out to emit `{}` for three fields. Each time, the
instrument was available. Here it is not, by construction: **you cannot run a
test that does not exist to prove that it tests nothing.** What stood in for the
measurement was agreement between the author and the reviewer - and this same
review had already produced one confident agreement between those two that a
single measurement later overturned (guard-versus-derive). Two agreeing agents
are not a measurement.

The asymmetry settles it. Keeping a redundant guard costs one redundant test.
Dropping a load-bearing one on an unfalsifiable argument costs a silent hole in a
plan that is not built yet, and the hole would be unusually durable: nobody
re-derives a question the design document records as settled.

**Trigger (mirrored to the ROADMAP, section 7 item 2): once D48's derivation
exists in the tree, guard 2 is re-examined against the built code.** The test can
be run then, and the question becomes answerable: mutate one field's `extend`
expression away from its `default` function and see whether guard 2 goes red. If
it cannot be made to fail, it is measured redundant and removed **then**, as its
own decision with evidence behind it. That removal is legitimate; this one was
not.

The general shape is still worth stating, because it is what made derivation the
right call in the first place: **a guard is what you build when derivation is
unavailable.** Reach for derivation first. But "I derived it, so the guard is
now pointless" is a claim about a test's behaviour, and claims about behaviour
get run, not reasoned.

**Rejected: a proptest `Arbitrary` round-trip instead of guard 1.** Steelman:
spec 10 names proptest for "matcher + planning semantics; this is the
correctness core", proptest is already a dev-dependency (`=1.11.0`), and a
generated-input property would cover field combinations a hand-written fixture
never reaches - which is the honest weakness of guard 1. Rejected because it
needs an `Arbitrary` impl across a 20-type recursive tree (`MatchExpr` nests
itself) to catch a defect class that one fixture closes completely: the failure
is per-field and does not depend on combinations, so a fixture touching every
field touches every way it can fail. Revisit if the model grows a field whose
default depends on another field's value; none does today.

**Interface changes:** `Profile`'s serialized form changes (fields at their
default no longer appear). The published schema is **unchanged for 14 of the 17
fields** - that is the point of the `extend` work - and for the three
struct-valued fields the `default` annotation narrows from the materialized
object to `{}`, losing no information (the children's own annotations carry it,
verified above). Deserialization is untouched: every omitted field was already
`#[serde(default)]`, so every profile that parses today still parses, and D48
emits a strict subset of what it emits now.

---

## 2. Fluent catalog additions (en + de, both, in the same wave)

No new user-facing string outside the catalogs. New file
`locales/{en,de}/gui-editor.ftl`; `check-i18n`'s check 3 enforces en/de parity
on it from the moment it exists (`scripts/check-i18n.mjs` picks up any
`locales/<tag>/` directory automatically, and `locales/de/` exists today with
all six catalogs). Keys the decisions above create, by owner:

| decision | keys | catalog | count |
|---|---|---|---|
| D45 | one `labelKey` per `EditableField` across the 13 registries | `gui-editor.ftl` (new) | **42** |
| D41 | the save-surface standing note (rewrite-from-model wording) | `gui-editor.ftl` (new) | 1 |
| D41 | save-failure `IpcError` codes | `gui-common.ftl` | codes |
| D43 | apply button label + tooltip | `gui-batch.ftl` | 2 |
| D43 | `ApplyError` codes (`suggestion-rule-not-found`) | `gui-common.ftl` | codes |

**Catalog placement follows the tree's existing split, it is not invented here:**
every `IpcError` code today lives in `gui-common.ftl` (`mkvmerge-spawn-failed`,
`settings-io-failed`, `internal-task-failed`, ...), so D41's and D43's shell
errors join them. D43's apply control lives in the **batch view**, not the editor
- spec 8.2 puts the diagnostics panel and its one-click apply there, and D41
records why the plan-scope pairing is not a UI-location one - so its two strings
go to the existing `gui-batch.ftl` beside `SuggestionCard.vue`'s current
copy-button keys. Only the editor's own surface justifies a new catalog, which is
why `gui-editor.ftl` carries **43**: the 42 registry labels plus the save-surface
note.

**42 is exact, not an estimate**: D45's table enumerates 13 structs carrying 43
fields, of which `Profile.profile_version` is the single `FixedField` and has no
label. The count is derivable from that table by construction, and it is the
reason the table had to exist - while D45's registry domain was an ellipsis this
catalog was unauthorable, which is why the two are one change and not two.

Widget facets add no keys of their own: `select` and `keywordOrBlock` render
their options from the domain arrays (`COLLISION_POLICIES`, `KEEP_DROP`, the
four `*_KEYWORDS`), and those are profile-format tokens (`keep`, `drop`,
`error`, `primary`, ...), not prose. Displaying them verbatim is correct and
locale-neutral - the same call D39 made for the `allowed` param, whose
"locale-neutral value tokens, not prose" reasoning applies unchanged. Should a
keyword ever need a translated display name, that is a new key per value and a
new decision; nothing in v1 needs one.

Every registry `labelKey` lands in `gui-editor.ftl` in both locales, and D45's
`LABEL_KEY_RE` scan makes a missing one a hard CI failure.

---

## 3. Spec amendments proposed

`proc-04-spec-wins` requires normative decisions to be folded into the spec,
not left in a memo, "since a decision left only in a memo can be silently
overridden by stale spec text", and mandates a self-contradiction sweep after
any amendment. Proposed:

1. **Spec 8.2**, profile editor: state that saving writes canonical YAML from
   the model and does not preserve comments, key order or formatting (D41), and
   that fields left at their default are not written back (D48). The bullet
   currently says only "open/save YAML".
2. **Spec 8.1**, CLI: `muxsmith schema` is a supported user feature, not only a
   debug aid; cross-reference the README's `yaml-language-server` section
   (D47).
3. **Spec 8.4**, accepted v1 exceptions list: add the JSON Schema's
   `description` fields (Rust doc comments) as an explicit exception, with the
   file-format-documentation rationale (D47). Without this the schema reads as
   a standing 8.4 violation.
4. **Spec 4.8 / 4.9**: no change. The keyword domains are already stated
   normatively in prose there (`filename`: keep or template; `chapters`: keep |
   drop; `title`: keep | clear | template; `source`: primary). D46 makes the
   schema agree with text that is already correct - the spec is the source, and
   the constants in `profile/model.rs` are what the spec's prose becomes in
   code.

**Self-contradiction sweep, run against the amendments above; complete.** Spec
4.1's reference example is annotated with the same keyword domains as trailing
comments (`# keep | drop`, `# error | skip | overwrite`), and stays correct
under all four amendments.

The one item this sweep was blocked on is now resolved. Spec 4.1's reference
example omits `source` and `optional` on every rule; had the E1 escalation been
ruled the other way (emit defaults), the spec's own showcase profile would no
longer have been the shape the editor produces, and amendment 1 would have had
to say so. **D48 rules omit, so no contradiction arises** - the editor's output
and the spec's example agree about which fields a profile carries.

They still differ in **formatting**: the example is written in flow style
(`match: { exact: { type: video } }`) and a saved profile is block style. That
is not a contradiction - spec 4.1 illustrates the format, it does not specify
the editor's output - but amendment 1's wording should not imply the editor
reproduces the example verbatim. No spec text needs to change for it.

---

## 4. mkvtoolnix parity audit (SI-3)

Method per `testing-si3-run-binary` and `proc-06-mkvtoolnix-parity`: source
read at `~/Downloads/mkvtoolnix` (v100.0, `configure.ac`:
`AC_INIT([MKVToolNix],[100.0],...)`), binary confirmed by running it
(`mkvmerge --version` -> `mkvmerge v100.0 ('Do Hot Girls Like Chords')
64-bit`) - the source tree and the installed binary are the same version, so
the citations below describe the binary on this machine. Licensing: behaviour,
facts and interfaces only; no literal code or text taken; no wording modeled.

The load-bearing frame: mkvtoolnix is **interactive** (it pre-fills guesses the
user reviews); Muxsmith is **declarative batch** (the profile is the spec). Per
`proc-06`, muxing semantics and output are parity targets; **input-time
convenience guesses are not** (`docs/IDEAS.md` 1-2). A profile editor is
input-time by definition, so most of this plan has no parity target - which is
itself the finding, and is recorded rather than left as an absence.

| Plan-6 surface | mkvtoolnix analogue | Classification |
|---|---|---|
| Canonical save from the model | `MuxConfig::save` -> `QFile::remove` + `ConfigFile::create` + write from model (`merge/mux_config.cpp:566-576`) | **MATCH**, and mkvtoolnix is more aggressive (delete-then-recreate) |
| YAML profile, comments possible | `.mtxcfg` is JSON, no comment syntax; `ConfigFile::create` always returns `JsonConfigFile` (`util/config_file.cpp:71-73`); INI reader kept only for legacy files (`:56-57`) | **Justified divergence**: the profile is hand-authored (spec 4.1), `.mtxcfg` is a GUI byproduct |
| Save-surface note about rewriting | none | **No target**: mkvtoolnix has no comments to lose, so the question cannot arise for it |
| Profile editor UX (rule grid, drag-reorder, panels) | the Multiplexer tab's track list and per-track panels | **Not a parity target** (`proc-06`: input-time). Named so the exclusion is deliberate, not an omission |
| One-click apply-suggestion | none | **No target**: mkvtoolnix has no suggestion engine; the whole batch-validated-refinement idea (D6, `core-03`) is a Muxsmith invention |
| Schema as a user artifact for hand-authoring | none (D47) | **Divergence in kind**: an interactive tool's input surface is its GUI; a declarative tool's is its file format |
| Registry / ts-rs / check-i18n | none | Internal; no parity surface |

**No behavioural claim in this plan needed the binary beyond the version
check**, because nothing here touches muxing semantics, argv, or `-J` output.
Stated explicitly per `testing-si3-run-binary` so the absence of golden-test
work in this plan is a recorded conclusion rather than a gap. `core-89-homebrew-apple-silicon-path`
is the recorded cautionary case of an SI-3 exclusion resting on the wrong
authority; the exclusions above rest on `proc-06`'s interactive-vs-declarative
rule, which is the right authority for them.

---

## 5. Gap table from the brief, closed

| Gap | Verified? | Closure |
|---|---|---|
| `load_profile` does not exist | **Yes** - absent from `invoke_handler` (`src-tauri/src/lib.rs:440-452`) | D42 |
| `save_profile` does not exist; no production path serializes a `Profile` | **Yes** - absent from the handler; no `to_string`/`to_writer` call on a `Profile` anywhere in `crates/` or `src-tauri/`. But the model **already derives `Serialize`** (`profile/model.rs:17` and throughout) and `yaml_serde::to_string` exists (`ser.rs:707`), so no derive or dependency work is needed | D41 (core writer), D42 (command) |
| `validate_profile` takes a path | **Yes** (`src-tauri/src/lib.rs:302-304`). Note the model-side core function **already exists and is public**: `validate::config_diagnostics(&Profile)` (`profile/validate.rs:193`) | D42 - new command, three-line wrapper; path-based command kept for `BatchView.vue:118` |
| `apply_suggestion` missing; "the logic exists as a test helper" | **Command absent: yes. Logic exists: NO** - correction #1 | D43 - written fresh; reuses `rule_index_of` and the engine's `with_rule_match` narrowing (`core-44`) |
| `Suggestion`/`StructuredEdit` are `Serialize` only | **Yes** (`planner.rs:201`, `:231`); `DiagCode` likewise (`report/mod.rs:40`) | D43 - `StructuredEdit` gains `Deserialize`; `Suggestion` and `DiagCode` do not |
| `Suggestion.config_path` is only ever `tracks[<N>].match` | **Yes** (`planner.rs:1437`, `:1516`; parsed by `rule_index_of` `:2032`) | D43 - treated as the narrow token it is; never conflated with `Diagnostic.config_path` |

---

## 6. Deliberately out of scope

- **The D23 `runActive` reset re-check** (`docs/ROADMAP.md:58-64`). A run-path
  concern in `src/views/JobsView.vue:150-200`, touching neither the editor, nor
  apply, nor the schema. The re-cut re-pointed the ledger entries out of Plan 6
  but left this named input behind; per correction #8 it belongs with the
  run-path work, not here. Named for the controller in section 9. The code
  carries a thorough self-documenting rationale for its current shape
  (`JobsView.vue:161-175`), so this is a re-read, not a known defect.
- **Help mode** (Plan 7), including the editor's own help-ids. The re-cut
  sequenced Plan 7 after Plan 6 precisely so the editor's controls get their
  help-ids in that pass rather than a retrofit (`docs/ROADMAP.md:68-70`). The
  spec 8.3 **tooltip/inline-explanation baseline still applies to the editor's
  views** (D22's "NOT deferred" clause); only the sidebar machinery waits.
- **The planner seam / `plan_pipeline` hoist** (Plan 9). D43 calls the existing
  engine helpers directly and does not restructure them.
- **A richer suggestion grammar** (reorder, relax). `core-33-suggestion-narrow-only`
  keeps v1 narrow-only; D43 does not widen it.
- **Per-file manual override in the GUI.** `gui-02` is a settled restraint with
  an "open in mkvtoolnix-gui" escape hatch parked at v1.x. The editor edits the
  *profile*; nothing in this plan lets a user tweak one file's outcome.
- **Localizing the JSON Schema** (D47) and **SchemaStore publication** (D47),
  both triggered rather than built.
- **`ts-rs` typing the Tauri command signatures.** D44 types the model only;
  command signatures stay hand-written in `src/ipc.ts` as today.

---

## 7. Triggers created (for the controller to mirror into the ROADMAP in this turn)

Named here; the controller writes the tracker.

1. **A profile-model field gains a `#[serde(default)]`** -> it must join D48's
   17-row table with all three attributes, and **all three name the same
   function**: `#[serde(default = "F")]`, `skip_serializing_if = "is_F"`, and
   `extend("default" = to_value(F()))`. Omit the `extend` and the field silently
   drops its `default` annotation out of the published schema; guard 2 catches
   that. Get the predicate wrong and guard 1 catches it. Recorded because this
   is the one place in the plan where getting it wrong loses user data silently.
2. **D48's derivation exists in the tree** -> re-examine **guard 2**, which is
   retained by the safeguard-stays-until-built rule rather than by a belief that
   it can fail. The test is only answerable once it exists: mutate one field's
   `extend` expression away from its `default` function and see whether guard 2
   goes red. If it cannot be made to fail, it is measured redundant and removed
   then, with the measurement recorded. If it can, the design phase was wrong to
   think it a tautology and the guard stays for good. Either way the question
   gets settled by running it, which is exactly what the design phase could not
   do.
3. **`tauri-specta` publishes a stable non-RC Tauri-2 release** -> re-evaluate
   D44's rejection; it would type the command signatures, which ts-rs does not.
4. **A second Muxsmith artifact needs TypeScript types** (e.g. the report
   documents, today hand-mirrored in `src/ipc.ts`) -> extend the `ts` feature's
   export set rather than hand-mirroring again; the D44 machinery is already
   paid for.
5. **1.0 is tagged, or a user asks for zero-config schema autocompletion** ->
   re-evaluate SchemaStore publication (D47), which needs a stable public URL
   and version-skew handling.
6. **A profile-model field is added or removed** -> the D44 CI drift check and
   the D45 registry both fail by construction, naming the site. No tracker
   entry needed; recorded so the mechanism is understood as the tracker.
7. **A second generated artifact gains a CI drift check** -> the
   committed-generated-plus-drift-check pattern reaches count 2 toward
   Tier-2 promotion (`core-06` is the committed half; the drift half is new
   with D44).

**Controller corrections this document raised: two landed, one open.**

8. **RESOLVED in `fdcdcba`, verified at HEAD - do not re-issue.** This document
   asked for two ROADMAP passages asserting D22's dead comment-preserving
   premise to be corrected. Both are gone: `grep "rationale intact"` and
   `grep "hard design question"` return nothing, and `ROADMAP.md:19-27` now
   reads "D22's editor+apply pairing is KEPT, but **not on D22's stated
   reason**", crediting the shared-model-ownership argument D41 supplies.
9. **RESOLVED in `fdcdcba`, verified at HEAD - do not re-issue.** This document
   reported that the re-cut's "(6 / 5 / 1 / 8)" split recounted to 16 because
   Plan 6 listed 2 inputs, not 6, so "nothing was dropped" was unverifiable.
   Plan 6 now lists **6** numbered inputs (`ROADMAP.md:38-64`) against 5/1/8 for
   Plans 7/8/9; the split recounts to 20 and `:15-17` now says so in those terms
   ("every one is its own bullet under its plan, so the split ... is recountable
   rather than asserted").
10. **OPEN - needs a controller action item, which it does not yet have.**
   `gui-22` and `exec-44-runlog-14day-autoprune` are a **recorded-statement
   collision** in `product-boundaries.yaml`: `gui-22` (`:243-252`) states v1
   keeps all run logs with pruning deferred to v1.x, while `exec-44` (`:15-23`)
   records D35 reversing exactly that to an automatic 14-day prune. `gui-22`
   still carries `status: settled` with no supersession marker. A
   recorded-statement collision is one of the four observable contested criteria
   in `proc-latitude-clause-boundary`. Found while reading Tier 2 for this
   design and independently confirmed by the design reviewer; unrelated to Plan
   6, surfaced rather than touched.

---

## 8. What the implementer must not decide

**Every fork in this plan is closed.** The one escalation this document raised
(E1, default-valued fields on save) was ruled by the governing human on
2026-07-15 and is recorded as D48. There is no "the implementer may choose",
"either approach works" or "if a simpler alternative exists" in this document,
and no task brief derived from it may add one
(`proc-latitude-clause-boundary`, which binds design documents as of
2026-07-15, not only briefs). A fork discovered on code contact returns as
NEEDS_CONTEXT with a decision memo; it is not resolved at the keyboard.

Specifically pre-decided, because each is a place an implementer would
otherwise improvise:

- The writer lives in core, not in src-tauri and not in the CLI (D41).
- `save::to_file` picks format from the extension exactly as `load::from_file`
  does; a `.json` profile stays JSON (D41).
- `validate_profile(path)` is **not** retargeted, renamed or removed (D42).
- `apply_suggestion` takes `config_path` + `edit`, not a whole `Suggestion`;
  `DiagCode` does **not** gain `Deserialize` (D43).
- Apply does not validate; the editor's existing round-trip does (D43).
- Generation is `cargo test -p muxsmith-core --features ts`, **not** an xtask,
  and the reason is feature unification, not taste (D44).
- Bindings are committed, not built (D44).
- The `never` arm is required even though the house's existing switch shape
  compiles without one (D45).
- The keyword constants live in `profile::model`, not `capability` (D46).
- The `allowed` param goes through `domain_hint`, and the snapshots must not
  move (D46).
- The schema override emits `enum`, not `oneOf`+`const` (D46).
- The README documents the editor-settings binding, not the modeline (D47).
- Omission is implemented with `skip_serializing_if` on the derives, **not** by
  post-processing the tree in `save::to_string` (D48).
- For each of the 17 fields, **the predicate and the schema annotation both call
  the function its own `default` attribute names** - one function, three
  mentions, no copies. A generic `is_default` is correct for 13 of them and
  silently destroys user data on the other 4 (D48).
- Each of the 17 fields carries `#[schemars(extend("default" = <derived>))]`;
  without it the published schema loses its `default` annotations. The three
  struct-valued fields derive to `"default": {}` and that is accepted, not
  patched with a literal (D48).
- **Both** D48 guards ship with the serializer, not after it. Guard 2 is not
  optional and is not to be argued out at the keyboard on the grounds that the
  derivation makes it vacuous - that argument is already recorded in D48, and
  the safeguard-stays-until-built rule holds the guard in until it exists and
  can be measured (trigger 2).
