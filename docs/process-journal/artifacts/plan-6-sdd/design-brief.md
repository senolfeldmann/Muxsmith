# Plan 6 design brief: profile editor, apply-suggestion, schema keyword domains

Controller-authored brief. You are the **design implementer**: you write the
design document and its ADRs against this brief. An independent reviewer grades
your document before the owner reads it. The controller's hands write only this
brief; the design document is yours.

## 0. What you produce

**One document:** `docs/superpowers/specs/2026-07-15-plan-6-design.md`, plus the
spec amendments it necessitates. It becomes the ground truth every later task
review checks against, so its errors do not get caught downstream. That is why
it is reviewed independently.

Numbering: the last ADR is **D40** (`docs/superpowers/specs/2026-07-14-plan-5.8-decisions.md`).
Yours start at **D41**. Verify that before you number anything.

## 1. Verify this brief against the tree - this is a duty, not a courtesy

Every factual claim below was checked by the controller, but **check it again**.
The precedent is explicit: in the previous design round the implementer refuted
two brief premises against the tree and killed two phantom tasks before they
reached planning. If a premise here is wrong, say so with the file:line that
refutes it. A brief is an input, not an authority.

## 2. Scope

Plan 6 is the profile editor, its one-click apply-suggestion, and the schema
keyword-domain fix. It is NOT help mode, packaging, or the core hoists: the
Plan-6 anchor was re-cut on 2026-07-15 into Plans 6-9 (see `docs/ROADMAP.md`,
"Plan-6 scope re-cut"). Do not design across that boundary.

## 3. Ground truth, in precedence order

1. `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` - the spec. It wins
   over any plan or memo on conflict. Sections 8.2 (GUI), 7 (architecture +
   the DRY rules), 5.3 (suggestion engine), 4.x (profile format), 8.4 (i18n).
2. **Tier-2 house files**: `docs/product-boundaries.yaml`,
   `docs/conventions.yaml`, `docs/process-conventions.yaml`. These join the spec
   as ground truth. Conform, and surface (do not silently resolve) any new
   pattern you establish or any deliberate deviation.
3. `docs/superpowers/specs/2026-07-10-plan-5-gui-design-decisions.md` D22 - the
   scope decision that deferred the editor here. Its editor+apply pairing
   stands; its "one-click apply means comment-preserving YAML mutation" premise
   does NOT (see 4.1) and needs an explicit supersession note in your ADR.
4. Two research corpora kept with the project's non-repo material,
   `yaml-roundtrip-landscape.md` and `rust-ts-ssot-landscape.md`, plus
   `rust-ts-ssot-empirical-probes.md`. They carry the measurements behind the
   decisions in section 4. **Read them before you argue with section 4** - and
   note their own honesty markers: some claims in them are reported rather than
   measured, and are labelled as such.

## 4. Decided dimensions - BINDING, not open for re-litigation

These are settled. Your job is to design *within* them and record them properly,
not to reopen them. If you believe one is wrong, that is a NEEDS_CONTEXT return
with a decision memo (section 7), never a silent deviation.

### 4.1 Save fidelity: canonical, comments are not preserved (owner, 2026-07-15)

Saving from the editor writes canonical YAML from the model. Comments and
formatting in the user's file are not preserved. This applies **uniformly** -
there is no distinction between "editor-owned" and "foreign" profiles. A profile
is a profile: parse it, and if it is valid, edit and save it; if not, the
existing diagnostic chain reports why.

The rationale that must be recorded, because it is what stops this being
re-litigated every time a new crate appears: **YAML has no concept of comment
attachment.** [YAML 1.2.2 §6.6](https://yaml.org/spec/1.2.2/#66-comments) states
comments are not associated with a particular node. Every library that "preserves
comments" invents an association the spec disclaims. Measured on the real
`reference.yaml`: deleting a rule via the leading candidate left a comment
describing the deleted rule sitting above a different one. Valid YAML, identical
comment count, document now lies. The editor's signature operation is
drag-to-reorder (spec 8.2), which is exactly the operation that breaks
attachment. **Dropping comments is honest; carrying them through a structural
rewrite is a lie.**

Rejected alternative that must be recorded WITH its steelman: `yamlpath` +
`yamlpatch` (zizmor's fix engine, built on the same `yaml_serde` 0.10 already in
the tree) applies a narrow additive edit byte-identically-except-the-edit with
comments intact, and would have made apply-suggestion lossless. Rejected because
it splits save behaviour: applying a suggestion would preserve comments while
moving a rule would not, which is arbitrary from the user's seat. Also record
the landmine found: `yamlpath` silently follows aliases and returns the anchor
definition's span, so a GUI writing there would rewrite a shared anchor and
change every rule aliasing it, without warning.

**Consequence for D22:** its stated reason for coupling apply to the editor
("one-click apply means comment-preserving YAML mutation") dissolves. Apply is
now: deserialize, mutate model, serialize canonical. Record the supersession.

**Controller's own addition, strike it if you disagree and say why:** the editor
states once, at the save surface, that saving rewrites the file canonically and
does not preserve comments. A standing note, not a modal, and no detection of
whether comments are present - that would need the parser to see them.

### 4.2 Model to frontend: ts-rs, hand-built components, a forced registry (owner, 2026-07-15)

The Rust model is the single source of truth for the frontend. The mechanism is
**not** a generated form and **not** generated types alone:

- **`ts-rs`** (12.0.1, verified on crates.io 2026-07-15) generates the TS types
  from the model. It coexists with `schemars` rather than replacing it, handles
  `untagged` and recursive types, and stays out of the shipped dependency tree.
  `tauri-specta` was rejected: its Tauri-2 line is `2.0.0-rc.25` and has been
  release-candidate for years, which fails this project's pin-everything
  doctrine. There is no first-party Tauri 2 answer.
- **Components stay hand-built**, per spec 8.2 (rule grid, drag-reorder,
  per-concept panels). A schema-driven generator is rejected: JSON Forms ships
  no `anyOf` renderer and fails on our four keyword-or-block enums; the schema
  cannot express our keyword domains (section 4.3); and generated labels come
  from Rust doc comments, i.e. English prose out of a prose-free core, which
  hits spec 8.4 head-on and cannot be intercepted for combinator branch labels.
- **The forcing function is a `Record<keyof T, FieldSpec>` registry**, NOT the
  type. Measured against the project's own `tsc` 6.0.3: adding a field to a type
  breaks no reader (structural typing - reading a subset is always legal), but
  an incomplete `Record<keyof T, FieldSpec>` fails with `error TS2741`, naming
  the missing key. The registry entry carries the Fluent label key and the
  widget choice, so the compiler reports the omission exactly where the missing
  work belongs.

Design the registry shape. One per edited struct. For sum types (`MatchExpr`,
the keyword enums) the equivalent lever is a discriminated-union switch with a
`never` arm - design that too.

**Named implementation constraints:**
- `TS_RS_LARGE_INT = "number"` must be set, or 64-bit integers map to `bigint`
  and mistype every numeric field.
- The registry makes `$t()` keys dynamic. `scripts/check-i18n.mjs` finds catalog
  keys by grepping literal calls, so it goes blind and loses that check. It must
  learn to read the registry, **in the same wave**. Do not trade one gap for
  another.
- Decide and record: are the generated bindings committed (with a CI drift
  check) or built? The controller's assumption is **committed + CI-checked**
  (`git diff --exit-code` after regeneration), because a build-time generator
  adds a step to every contributor's first run. Overrule with reasons if the
  tree says otherwise.

### 4.3 The schema ships as a user artifact, and its keyword domains get fixed (owner, 2026-07-15)

`muxsmith schema` (spec 8.1) already ships. The decision makes it a **supported
user feature**: users point `yaml-language-server` at it and author profiles with
autocompletion and in-editor validation. That needs a README section and a
decision about how the schema reaches the user's editor - design that.

The fix: four `#[serde(untagged)]` enums (`FilenameCfg` model.rs:145,
`SourceCfg` :223, `ChaptersCfg` :341, `TitleCfg` :373) have a `Keyword(String)`
arm, so the schema says `"type": "string"` where one or two values are legal,
while real enums (`KeepDrop`, `CollisionPolicy`) emit precise `oneOf`/`const`.
The schema is inconsistent with itself.

**Do not "fix" it by typing the arm.** Measured: a typed keyword arm makes
`filename: kepp` fail with `data did not match any variant of untagged enum`,
which does not even name the keyword as the problem. The `String` buys the
localized `InvalidKeyword` diagnostic with its `allowed` param. That trade is
real; it was simply never recorded, which is why a reviewer flagged it as a
defect (Plan-1 final review minor #7).

The fix is `#[schemars(schema_with = ...)]`: the `String` stays for
deserialization, the schema projection is overridden to `enum: ["keep"]`. Both
properties, no trade.

**The rider that makes it a single source:** the keywords exist today as bare
literals in four match guards (`validate.rs:105/129/149/166`), plus four
hand-typed `allowed` params, plus four doc comments. No shared constant. Spec §7
requires the schema and validation each live in exactly one place; today they do
not. Design one constant set feeding the guard, the `allowed` param, and the
schema override.

The full analysis is with the project's non-repo material
(`schema-keyword-domains.de.md`). The ROADMAP trigger that carried this item
("a GUI generating an editor from the schema") would never have fired under 4.2
- record that the real reason is the shipped artifact, not the GUI.

## 5. The gaps the design must close

The controller traced the runtime chain and found these. **Verify each against
the tree**; they are the controller's reading, not established fact.

| Gap | Controller's finding |
|---|---|
| `load_profile` | Does not exist. `validate_profile` (`src-tauri/src/lib.rs:302`) takes a **path** and returns a report, never the model. |
| `save_profile` | Does not exist. No production path serializes a `Profile`. |
| `validate_profile` shape | Takes a path. The editor needs validation on the in-memory model, or it writes to disk before every check. Design the command shape; keep the house rule that the frontend performs zero semantic validation. |
| `apply_suggestion` | Missing as a command. **But the logic exists as a test helper** `apply_edit_to_first_rule` (`crates/muxsmith-core/tests/suggestions.rs:95`). Reuse before writing: hoist it into the library. |
| `Suggestion` / `StructuredEdit` | `Serialize` only, no `Deserialize` (`planner.rs:231`, `:201`). The edit grammar is one-way core-to-GUI; applying an edit needs it to come back, or the command must take profile + suggestion core-side. Decide and record. |
| `Suggestion.config_path` | Only ever `tracks[<N>].match` (`planner.rs:1437`, `:1516`), parsed back by `rule_index_of` (`:2032`). Do not assume it is a general path. The `Diagnostic.config_path` field is a different, general one - do not conflate them. |

## 6. Required ADR slots

Each decision: the decision; the rationale; **rejected alternatives with why**;
triggers created (mirrored to the ROADMAP in the same turn - name them, the
controller writes the tracker); interface/wire-format changes; `superseded by`
links where you replace an earlier decision. A spec amendment sweeps the spec
for self-contradictions before it is proposed (spec 4.9 once contradicted 4.5
for two plans after an amendment).

Interface and wire-format changes are load-bearing here: new Tauri commands, the
`StructuredEdit` direction, and the generated bindings are all downstream
contracts.

## 7. Rules

- **No unresolved design question may reach an implementer** (owner decree
  2026-07-15). Design-latitude clauses are banned: your document must not leave
  "the implementer may choose" anywhere. Every fork is decided in the document
  or escalated.
- If you find a fork this brief does not settle - a ripple cost, a hidden
  consumer, a colliding invariant - return **NEEDS_CONTEXT** with a decision
  memo: the options, their costs against the named invariants, and a
  recommendation. Do not resolve it silently and do not invent an option that
  requires machinery the product does not have.
- **SI-3 parity duty:** compare against mkvtoolnix-gui / mkvmerge wherever
  meaningful, at the depth of the Plan 1-4 parity audit. The load-bearing
  distinction: mkvtoolnix is INTERACTIVE (pre-fills guesses the user reviews),
  Muxsmith is DECLARATIVE BATCH (the profile is the spec). Muxing semantics and
  output are parity targets; input-time convenience guesses are NOT
  (`docs/IDEAS.md` 1-2). Classify match / justified divergence / genuine gap;
  read the source at `~/Downloads/mkvtoolnix` and cite file:line; confirm
  mkvmerge behaviour by running the binary, never from memory. Licensing:
  behaviour, facts and interfaces are fair game; literal code or text passages
  are never taken; a deliberately modeled wording is recorded as an explicit ADR
  decision, never adopted silently.
- **Ground external facts empirically.** Registry-verify any version you name.
  Do not type a version from memory.
- No new user-facing string outside the Fluent catalogs, and anything new lands
  bilingual (en+de).

## 8. What the reviewer will check

Requirement compliance against this brief and quality against the spec and the
Tier-2 files: that every decided dimension in section 4 is recorded with its
rationale and rejected alternatives; that no latitude clause survives; that the
gaps in section 5 are each closed or explicitly deferred with an observable
trigger; that ADR slots are complete and the rejected-alternatives are honest
rather than strawmen; and that any spec amendment leaves the spec
self-consistent.
