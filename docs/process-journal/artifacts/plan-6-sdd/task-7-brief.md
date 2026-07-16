### Task 7: D47 - the schema as a supported user artifact, and the v1-spec amendments

**Stream C** (`.worktrees/plan6-c`), parallel with streams A and B - docs only. **This task is the single owner of the v1 spec** (F3): it carries all three spec amendments (8.1, 8.2, 8.4) and runs the self-contradiction sweep once against the merged result.

**Files:**
- Modify: `README.md` (new section)
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (spec 8.1 + spec 8.2 + spec 8.4 exception)

**Interfaces:**
- Consumes: nothing. Produces: nothing code-facing.

**Read first:** design D47 (`:1227-1332`) and design section 3 (`:1772-1813`, the three spec amendments and the already-run sweep).

Binding points:
- The README documents the **editor-settings binding**, not the in-file modeline, and **the reason is D41**: the modeline `# yaml-language-server: $schema=...` is a YAML comment, and a canonical save does not preserve comments - so a user who wires up autocompletion with a modeline and then saves once from the GUI loses their schema binding silently, with no message. The README states that consequence explicitly rather than leaving the user to find it.
- The schema's English `description` fields (Rust doc comments) become user-facing under D47. This is an **accepted, deliberate boundary**, not an oversight: the schema documents a *file format*, the same category as the README and the spec, both English-only by design. Spec 8.4 gains an explicit exception entry so a future reviewer does not read it as a standing violation.
- Do **not** add SchemaStore publication or a GUI startup write; both are rejected in D47 and parked behind triggers.

- [ ] **Step 1: Write the README section**

Document `muxsmith schema > muxsmith-profile.schema.json`, the VS Code `yaml.schemas` mapping over a glob such as `*.muxsmith.yaml`, and the equivalent `lspconfig` settings block for Neovim/Helix. State the modeline consequence. Keep the README's established sell-tone register (the case-scoped exception recorded in the ROADMAP's README entry), not the writeup register.

- [ ] **Step 2: Amend spec 8.1**

`muxsmith schema` is a supported user feature, not only a debug aid; cross-reference the README section (design section 3, amendment 2).

- [ ] **Step 3: Amend spec 8.2** (moved here from Task 4)

The profile-editor bullet currently says only "open/save YAML". State that saving writes canonical YAML from the model and does not preserve comments, key order or formatting (D41), and that fields left at their default are not written back (D48). Design section 3, amendment 1, gives the exact scope; do not imply the editor reproduces spec 4.1's flow-style example verbatim (`:1809-1813`).

- [ ] **Step 4: Amend spec 8.4**

Add the JSON Schema's `description` fields to the accepted-v1-exceptions list, with the file-format-documentation rationale (design section 3, amendment 3).

- [ ] **Step 5: Run the self-contradiction sweep once, against all three amendments**

`proc-04-spec-wins` mandates the sweep after any amendment; run it **once** over the file now carrying all of 8.1, 8.2 and 8.4 (this is why the v1 spec has a single owner - a per-stream sweep would each see only its own amendment). Design section 3 (`:1797-1813`) records the sweep as already run and complete for exactly these three amendments, including the finding that spec 4.1's reference example stays correct because it, too, omits `source`/`optional`. Confirm that still holds against current spec text; do not re-derive it.

- [ ] **Step 6: Gate the docs**

```bash
pnpm lint
grep -rn "—\|–\|“\|”\|…" README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
```
Expected: the grep returns **no output** (typography constraint: ASCII only).

- [ ] **Step 7: Commit**

```bash
git add README.md docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "docs: the JSON schema is a supported hand-authoring artifact; fold spec 8.1/8.2/8.4 amendments (D47, D41, D48)"
```

---

