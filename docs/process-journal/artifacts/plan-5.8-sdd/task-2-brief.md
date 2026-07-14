### Task 2: D38 documentation - spec amendments, plan-3.5 supersession, README recipe

**Files:**
- Modify: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md` (4.5 at :180, 5.2 table after the EmptyPlan row :266)
- Modify: `docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md:156-160`
- Modify: `README.md` (end of "How it works", before :67 "What you get")

**Interfaces:**
- Consumes: `DiagCode::PassthroughProfile` and the catalog wording from Task 1 (same stream, runs after it).
- Produces: nothing downstream.

- [ ] **Step 1: Amend spec 4.5**

In `2026-07-08-muxsmith-v1-design.md`, at the end of the 4.5 paragraph beginning "`tracks` is a `{ unmatched, rules }` block" (line 180), append:

```
`rules` may be empty when `unmatched: keep`: that is a legal pure-passthrough remux (change only title / attachments / chapters, or normalize the container) and validate announces it with the info-severity `PassthroughProfile` notice (D38). Empty rules under `drop` remain a `NoTrackRules` error.
```

- [ ] **Step 2: Add the 5.2 row**

Directly after the `EmptyPlan` row (line 266), insert:

```
| `PassthroughProfile` | info | `tracks.rules` is empty and `tracks.unmatched` is `keep`: the profile is a legal pure-passthrough remux, every primary track copied unchanged (D38); emitted at validate time so an accidental delete-all-rules edit stays visible |
```

- [ ] **Step 3: Self-contradiction sweep**

Run: `grep -n -i "at least one rule\|NoTrackRules\|no-track-rules\|zero rule" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`
Expected: only the two passages just written state the rules-emptiness policy; nothing else in the spec asserts an unconditional rules requirement (pre-verified during design: the spec contained no such prose). If a hit contradicts, fix it in this task.

- [ ] **Step 4: Annotate the superseded D20 assumption**

In `2026-07-09-plan-3.5-design-decisions.md`, at the end of the "Zero rules under `keep`" bullet (lines 156-160), append:

```
  (Superseded by D38, 2026-07-14: the invited contradiction arrived as the
  owner ruling - zero rules under `keep` is now the legal passthrough
  idiom; see 2026-07-14-plan-5.8-decisions.md.)
```

- [ ] **Step 5: README recipe**

At the end of the "How it works" section (before "## ✨ What you get", line 67), add:

```markdown
### Pure passthrough: a profile with zero rules

A profile whose `tracks` block is `{ unmatched: keep, rules: [] }` is a
legal pure-passthrough remux: every track of each source file is copied
unchanged. Use it to change only the title, attachments, or chapters, or
to normalize containers in bulk, without writing a single track rule:

```yaml
profile_version: 1
input: { pattern: 'S(?<season>\d{2})E(?<episode>\d{2})', extensions: [mkv] }
tracks:
  unmatched: keep
  rules: []
title: { template: 'S{season}E{episode}' }
```

Validation announces the passthrough with an info notice
(`passthrough-profile`), so an accidentally emptied rule list never fails
silently. With `unmatched: drop` (the default), an empty rule list stays
an error - a profile that drops everything and selects nothing cannot
produce output.
```

Verify the `title:` line against the real profile schema in the README's own CLI reference / spec 4 before committing; if the top-level key differs (e.g. `output` template naming), use the schema-correct spelling - the recipe must be paste-runnable.

- [ ] **Step 6: Commit**

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md README.md
git -c commit.gpgsign=false commit -m "docs: passthrough spec amendments, D20 supersession note, README recipe (D38)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

