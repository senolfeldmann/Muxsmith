### Task 21: the v1-spec amendments (design section 6)

On master, after wave 3 merges - docs only; this task is the single owner of the v1 spec file in this plan. Amendments 1/2 may not land before D63's code (they landed in wave 1) and amendment 6 not before D52/D54 (wave 2); at this point every asserted mechanic exists. Amendment 3 (rustdoc) already rode Task 2. Read design section 6 in full, including its self-contradiction sweep.

**Files:**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

- [ ] **Step 1: Amendment 1 - spec 8.4 last bullet** (`:402`). Replace `- v1 ships English content only (non-goal 11); the mechanism ships complete.` with:

```markdown
- v1 ships English and German content on both surfaces - GUI catalogs and
  help topics, and the CLI's embedded catalogs (`cli-multilang-rendering`,
  D63); further locales are content work (non-goal 11).
```

- [ ] **Step 2: Amendment 2 - non-goal 11** (`:431`). Replace the `- UI localization content: ...` bullet with:

```markdown
- Locales beyond English and German. The mechanism (8.4) ships complete on
  both surfaces; adding a locale is content work (catalogs + help topics
  land together, enforced by CI) plus one row in the CLI embed table
  (D63), not a refactor.
```

- [ ] **Step 3: Amendment 4 - spec 10's eslint sentence** (`:416`). Replace `eslint (no-literal-string rule) keeps hardcoded strings out of the frontend; core is prose-free by construction.` with:

```markdown
the `@intlify/vue-i18n/no-raw-text` eslint rule (D27) keeps hardcoded
strings out of Vue templates - template text nodes plus the configured
static attributes (`title`, `aria-label`, `placeholder`, `alt`);
`:`-bound expressions are covered by the check-i18n literal scan instead;
core is prose-free by construction.
```

- [ ] **Step 4: Amendment 5 - spec 8.4 locale-selection bullet** (`:401`): append to the bullet, after "manual override in app settings": `(takes effect live, without restart; D56)`. The CLI half is a recorded no-change (it became true under D63).

- [ ] **Step 5: Amendment 6 - spec 8.3's help-mode bullets** (`:388-393`): (a) additions - append to the hover bullet (`:390`): `Hovering an element without a help-id sets no hover topic: the sidebar shows the pinned topic if one is pinned, else the current view's topic. Clicking an annotated element pins without activating it.`; (b) modifications - the pin bullet (`:391`) release enumeration becomes `until another element is clicked, the active view is switched, or help mode exits`; the toggle bullet (`:388`) Esc clause becomes `clicking again (or Esc, except while the settings dialog is open, whose native cancel consumes Esc) exits`; (c) the ruled activation semantic - append a new bullet after `:391`:

```markdown
- While help mode is active, control activation inside the main content
  area is suppressed; the help toggle, the three view tabs, the settings
  button and the sidebar stay live; clicking an annotated element pins its
  topic instead of activating it (owner ruling 2026-07-21, E3).
```

- [ ] **Step 6: Verify no superseded text survives** (each grep fire-verified against `git show HEAD:` of the file, where it hits):

```bash
S=docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
grep -n "English content only" $S            # expected: no output
grep -n "no-literal-string" $S               # expected: no output
grep -n "only English catalogs" $S           # expected: no output
grep -cn "cli-multilang-rendering" $S        # expected: >=1
grep -n "suppressed" $S                      # expected: the new 8.3 bullet
```

- [ ] **Step 7: Commit**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
git -c commit.gpgsign=false commit -m "spec: fold plan-7 amendments 1/2/4/5/6 - bilingual v1 on both surfaces, live locale switch, real eslint rule, ruled help-mode semantics (design section 6)" -m "Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

---

## Catalog end state (design section 2, transcribed - the recount every catalog-touching task's counts must land on)

| catalog | today | folds out | renames | adds | result | attrs added |
|---|---|---|---|---|---|---|
| `gui-common.ftl` | 41 | 5 (settings-open-tooltip, browse-button-tooltip, firstrun-use-path-tooltip, firstrun-retry-tooltip, firstrun-picker-hint) | - | 2 (help-toggle-label, help-sidebar-label) | **38** | 7 (D55 table + `browse-button.tooltip-directory` + `help-toggle-label.tooltip`) |
| `gui-settings.ftl` | 13 | 5 (settings-save-tooltip, settings-cancel-tooltip, settings-mkvmerge-path-hint, settings-default-jobs-hint, settings-locale-hint) | - | - | **8** | 5 |
| `gui-batch.ftl` | 39 | 12 (batch-profile-pick-tooltip, batch-dry-run-tooltip, batch-suggestion-copy-tooltip, batch-suggestion-apply-tooltip, batch-run-tooltip + 4 variants, batch-browse-dir-tooltip, batch-source-hint, batch-output-hint) | 1 (batch-recents-select-tooltip -> batch-recents-select) | 1 (batch-resolved-track) | **28** | 12 |
| `gui-jobs.ftl` | 46 | 5 (jobs-cancel-batch-tooltip, jobs-row-cancel-tooltip, jobs-history-refresh-tooltip, jobs-history-copy-tooltip, jobs-history-save-tooltip) | - | - | **41** | 5 |
| `gui-editor.ftl` | 45 | - | - | 1 (editor-track-rule-order) | **46** | 42 (`.tooltip` on every registry label; the 4 attribute-less ids: editor-save-note, editor-action-add, editor-action-remove, editor-track-rule-order) |
| `diagnostics.ftl` | 50 | - | - | - | **50** | - |
| `cli.ftl` | 26 | - | - | - | **26** | - |

Recount of the fold total: 5+5+12+5 = 27 removed ids, plus the 1 rename, = 28 = the 22 tooltip-family ids + the 6 hint ids (the tooltip family loses 21 by removal and 1 by rename; `batch-browse-dir-tooltip` is removed from gui-batch and resurfaces as `browse-button.tooltip-directory` in gui-common, counted once, on the removal side).

Value changes to retained content: `settings-locale-label.hint` rewritten (Task 7), `apply-rule-index-out-of-range` gains the `$rules` plural selector (Task 20). The four `close-abort-*` messages are untouched. Help topics: 44 new markdown files (Tasks 8-10).

---

## Plan close (controller actions, not tasks)

- **Whole-branch review** by the resumed independent reviewer against the design, before any close action (house standing).
- **Owner rendered-surface pass** over every new/changed user-facing string: the 42 tooltip attribute pairs, the 44 topic files, `help-toggle-label` (+`.tooltip`), `help-sidebar-label`, `editor-track-rule-order` (de), `batch-resolved-track`, the rewritten `settings-locale-label.hint`, the `apply-rule-index-out-of-range` selector wording, `batch-recents-select`'s attribute. De terminology rides this pass (house precedent).
- **Triggers to mirror into the ROADMAP** (design section 8; trigger 1 is CONSUMED): 2 (third locale -> verify rule 5's plural carve-out), 3 (labelKey rename -> D62 is the tracker, no entry needed), 4 (fluent-vue/@fluent/bundle bump -> re-verify `bundles` setter + `$ta`), 5 (marked major -> re-check 0-dep/no-sanitizer premises), 6 (second `v-html` proposal -> reopens D50), 7 (attachment-context type-table flaw -> capability/registry work, revisit D58's path gate), 8 (tooltip-suppression request -> mkvtoolnix `uiDisableToolTips` parity gap), 9 (correct the stale ROADMAP v1.x sentence per correction #5), 10 (update `editor-generic-action-keys` 45 -> 46 with the tooltip-attribute note), 11 (locale beyond en/de -> CLI embed-table row; D64's pinned suite stays green by construction).
