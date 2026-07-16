# Task 7 report: D47 - schema as a supported user artifact, v1-spec amendments

Stream C, worktree `.worktrees/plan6-c`, branch `plan6-c`. Docs only.

## What changed, per step

**Step 1 - README section** (`README.md`, `### \`muxsmith schema\`` subsection, "Using the CLI"):
Expanded the two-line stub into the full editor-binding treatment: the redirect command
(`muxsmith schema > muxsmith-profile.schema.json`), a VS Code `yaml.schemas` block (glob
`*.muxsmith.yaml`), the equivalent Neovim/Helix `lspconfig` settings block, and a closing
paragraph stating the modeline consequence explicitly - the in-file
`# yaml-language-server: $schema=...` modeline is a YAML comment, the GUI's save does not
preserve comments/key order/formatting, so wiring the modeline and then saving once from
the GUI loses the binding silently (no error, file still works, editor just stops helping).
Recommends the editor-settings route instead, which survives every save. Kept in place as
an expansion of the existing subsection rather than a new top-level heading, since the
content is a direct continuation of `muxsmith schema`'s own description and the existing
closer paragraph ("Two conventions that hold everywhere...") still reads correctly as the
section's tail.

Did not add SchemaStore publication or a GUI startup write (both rejected in D47, parked
behind triggers in spec section 9 territory, out of scope here).

**Step 2 - spec 8.1** (CLI): added a bullet stating `muxsmith schema` is a supported user
feature, not only a debug aid (D47), cross-referencing the README's "Using the CLI" section
for the editor-binding mechanics.

**Step 3 - spec 8.2** (GUI, profile-editor bullet): appended that saving writes canonical
YAML rendered fresh from the in-memory model, not a patch of the file on disk - comments,
key order and formatting are not preserved (D41) - and that a field left at its serde
default is omitted rather than written back explicitly (D48). Deliberately did not
reference spec 4.1's reference example or its flow-style formatting, so the bullet makes no
claim that the editor reproduces that example's shape.

**Step 4 - spec 8.4** (i18n architecture, accepted v1 exceptions list): added the JSON
Schema's `description` fields (Rust doc comments, D47) to the exceptions list, with the
rationale sentence: the schema documents a file format, same category as the spec and the
README (both English-only by design), not application UI, not a diagnostic, so Fluent's
localization mandate does not reach it.

## Sweep confirmation (step 5)

Ran the self-contradiction sweep once over the merged result (all three amendments applied
to the same file). What I checked:

- Grepped the full spec for `hand-author`, `round-trip`, `preserve`, `debug aid`,
  `non-goal` to find any pre-existing claim that could now contradict the three new/changed
  bullets. Found one incidental hit (`349`: "every profile edit round-trips through a core
  `validate` Tauri command") - about IPC validation round-trips, unrelated to file
  save/comment preservation, no conflict.
- Read section 11 ("Non-goals for v1") in full: no non-goal contradicts schema-as-
  supported-feature or hand-authoring becoming a blessed workflow.
- Re-read spec 4's intro line ("A JSON Schema... generated from the same model (schemars)
  and published for editor autocompletion; it is never hand-maintained") against the new
  8.1 bullet: consistent, the 8.1 addition just documents the mechanism by which that
  publication reaches an editor.
- Confirmed spec 4.1's reference example (`docs/superpowers/specs/2026-07-08-muxsmith-v1-
  design.md:49-117`, unchanged by this task) is still consistent with D48 (omit only
  at-default fields). Note on the design doc's own phrasing here, since it is quoted almost
  verbatim in the brief: design section 3 (and D48's supporting evidence) describe the
  example as omitting `source` and `optional` "on every rule". Read as a literal universal
  ("never appears"), that is not quite right - two of the seven `tracks.rules` entries do
  show one of these keys explicitly (`optional: true` on the forced-English-SDH rule;
  `source: { external: ... }` on the external Turkish-subtitle rule), because in both cases
  the value differs from its default (`optional` defaults false/required, `source` defaults
  `primary`). The other five rules omit both keys because both sit at default. Checked
  every rule against this default table and every occurrence (present or absent) is
  consistent with D48's rule (write only non-default, omit default) - which is the
  operationally meaningful claim the design's concluding sentence actually rests on ("the
  editor's output and the spec's example agree about which fields a profile carries").
  No contradiction found; the design's "omits on every rule" phrasing is a loose gloss, not
  a wrong finding - flagging the imprecision rather than silently repeating it. This is not
  a fork requiring escalation: the recorded finding holds, I did not re-derive it, and
  nothing in the current spec text conflicts with it.
- Amendment 4 (spec 4.8/4.9: no change) - confirmed out of scope for this task and left
  untouched; verified both sections exist and were not accidentally touched by the diff.

Result: sweep complete, no self-contradiction found across 8.1, 8.2, 8.4, or the rest of
the spec.

## Gate output

```
$ pnpm install   (node_modules was missing in the worktree; installed via mise-pinned
                   node 26.5.0 / pnpm 11.10.0, +222 packages, no errors)
$ pnpm lint
$ eslint .
(exit 0, no findings)

$ grep -rn -- '—\|–\|"\|"\|…' README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
(exit 1, no output - clean)
```

## Files changed

- `README.md`
- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

Commit: `e027811` "docs: the JSON schema is a supported hand-authoring artifact; fold spec
8.1/8.2/8.4 amendments (D47, D41, D48)" on branch `plan6-c`, not pushed.

## Self-review findings

- Modeline consequence: stated explicitly in the README (not left for the user to
  discover), matching D47's own framing that this is "documentation, not machinery".
- Sweep confirmation: recorded above with what was checked and the result, including the
  one phrasing nuance found in the design doc's own gloss (does not affect the spec text or
  the finding's validity).
- Register: the new README prose was written to match two neighbouring subsections
  (`muxsmith identify`, and the closing "Two conventions that hold everywhere" line) -
  short punchy sentences, direct second-person address, one rhetorical turn ("which is
  exactly why it is a trap"), no hedging. Confirmed against `docs/ROADMAP.md:186-187`'s
  recorded sell-tone exception for the README before writing.
- Verified only the two intended files are modified (`git status --short` before commit
  showed exactly `README.md` and the v1 spec, nothing else touched by `pnpm install`).

## Concerns

None blocking. The one item worth flagging to the plan owner: the design doc's own
"omits `source`/`optional` on every rule" phrasing (used both in D48's supporting evidence
and in section 3's sweep summary) is imprecise if read literally - it should more precisely
say "shows `source`/`optional` only where they diverge from default, consistent with D48's
omission rule, and omits them elsewhere". The underlying finding (no contradiction between
spec 4.1's example and the editor's canonical-save output) is correct and I confirmed it
independently against the current spec text; this is a wording nitpick in the design
memo, not a defect in the shipped spec amendments, and I did not touch the design doc
(out of this task's file scope).
