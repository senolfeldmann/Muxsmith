# Task 2 report: D38 documentation (spec amendments, plan-3.5 supersession, README recipe)

Worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-a` (branch `plan58-a`). Task 1 (`f8e863e`) confirmed
already landed: `DiagCode::PassthroughProfile` (wire key `passthrough-profile`, info severity), conditional
`validate.rs`, bilingual catalog strings, tests.

**Preliminary note on the brief's own location.** The brief at
`.superpowers/sdd/plan-5.8/task-2-brief.md` does not exist inside the worktree tree at all: `.superpowers/sdd/`
carries a blanket `*` `.gitignore`, so it is untracked, per-checkout content that git worktrees do not share
(each worktree has its own working directory; only the main checkout at `/home/senol/Git/Muxsmith` had the
file materialized). Read the brief and wrote this report against the main checkout's copy at
`/home/senol/Git/Muxsmith/.superpowers/sdd/plan-5.8/task-2-brief.md`; all actual edits happened in the worktree
as instructed. Surfacing this since the task description assumed the brief lived in the worktree.

## Step 1: amend spec 4.5

Anchor: `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`, the paragraph beginning "`tracks` is a
`{ unmatched, rules }` block" under "### 4.5 Track rules". Located it at line 180, unchanged from the brief's
cited pre-Task-1 line number (Task 1 touched no spec files). Appended the exact brief text as a continuation
of the same one-line paragraph (matches this file's convention of one paragraph per physical line, e.g. the
adjacent line 191 paragraph is likewise a single long line):

> `rules` may be empty when `unmatched: keep`: that is a legal pure-passthrough remux (change only title /
> attachments / chapters, or normalize the container) and validate announces it with the info-severity
> `PassthroughProfile` notice (D38). Empty rules under `drop` remain a `NoTrackRules` error.

## Step 2: add the 5.2 row

Anchor: the `EmptyPlan` row, found at line 266 (unchanged from the brief). Inserted the exact brief row
directly after it, before `OutputCollision`:

```
| `PassthroughProfile` | info | `tracks.rules` is empty and `tracks.unmatched` is `keep`: the profile is a legal pure-passthrough remux, every primary track copied unchanged (D38); emitted at validate time so an accidental delete-all-rules edit stays visible |
```

## Step 3: self-contradiction sweep

Command: `grep -n -i "at least one rule\|NoTrackRules\|no-track-rules\|zero rule" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md`

One hit, run after Steps 1-2:

```
180:`tracks` is a `{ unmatched, rules }` block, not a bare rule list: `unmatched` (`keep | drop`, default `drop`) is the policy for PRIMARY-file tracks no rule matches; `rules` carries the ordered list below. `rules` may be empty when `unmatched: keep`: that is a legal pure-passthrough remux (change only title / attachments / chapters, or normalize the container) and validate announces it with the info-severity `PassthroughProfile` notice (D38). Empty rules under `drop` remain a `NoTrackRules` error.
```

Judgment: this is exactly the passage just written in Step 1 (both new sentences landed on the same physical
line as the pre-existing paragraph, so they surface as a single grep hit rather than two). The new 5.2 table
row from Step 2 does not match the grep pattern's keywords at all (no "at least one rule" / "NoTrackRules" /
"no-track-rules" / "zero rule" substring in its text), so it does not appear as a second hit; that is
consistent with the row's wording, not a miss. No other location in the spec asserts an unconditional
rules-non-empty requirement. Matches the brief's expectation exactly (design-time pre-verification held); no
fix needed.

## Step 4: annotate the superseded D20 assumption

Anchor: the "Zero rules under `keep`" bullet in
`docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md`, found at lines 157-160 (brief cited
156-160, off by one because line 156 is the "**Open mechanics (plan-time).**" section header immediately
above the bullet, not part of the bullet itself). Appended the exact brief annotation at the end of the
bullet, before the following `## D21` section.

## Step 5: README recipe

Anchor: end of "How it works", before "## What you get" (the header's actual text carries a leading emoji,
`## ✨ What you get`, at what is now line 89 after the insertion; the section title text matches the
brief). Inserted the "Pure passthrough" subsection verbatim from the brief.

**Schema verification (`title:` key), per the brief's explicit check:** confirmed against
`crates/muxsmith-core/src/profile/model.rs`:
- `Profile.title: TitleCfg` is a top-level field (sibling of `tracks`, `output`, etc.), matching the recipe's
  top-level `title:` key - no correction needed.
- `TitleCfg` is `#[serde(untagged)]` over `Template(TemplateBlock) | Keyword(String)`; `TemplateBlock` has a
  single field `template: String` (model.rs:124-128). So `title: { template: 'S{season}E{episode}' }` is the
  schema-correct shape, exactly as drafted in the brief. No wording correction was necessary; recording the
  verification since the brief asked for it explicitly.
- Named-group regex syntax `(?<season>...)` and template field syntax `{season}` cross-checked against spec
  4.7 (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:216`: "named groups (`{season}`)") and the
  existing codebase usage (`crates/muxsmith-core/src/discovery.rs:247` etc., same regex literal). Consistent.
- `tracks.rules` has no serde default (`model.rs:311`, confirmed), so `rules: []` must be written explicitly,
  matching the recipe and the D38 decision doc's own note on this.

**Runtime validation, per the brief's explicit requirement:** wrote the recipe verbatim to
`target/tmp/passthrough-recipe.yaml` inside the worktree (gitignored, `.gitignore:1:/target`), then ran the
built CLI's validate subcommand (shape confirmed from the README's own CLI reference, `muxsmith validate
<profile>`):

```
$ cargo run -p muxsmith-cli --quiet -- validate target/tmp/passthrough-recipe.yaml
[info] tracks.rules: This profile defines no track rules and tracks.unmatched is keep: a pure passthrough remux; every primary track is copied unchanged. If this is not intended, add track rules.
0 errors, 0 warnings, 1 info.
EXIT CODE: 0
```

Also captured `--json` for an unambiguous code check:

```
$ cargo run -p muxsmith-cli --quiet -- validate target/tmp/passthrough-recipe.yaml --json
{"diagnostics":[{"code":"passthrough-profile","config_path":"tracks.rules","params":{},"rendered":"[info] tracks.rules: This profile defines no track rules and tracks.unmatched is keep: a pure passthrough remux; every primary track is copied unchanged. If this is not intended, add track rules.","severity":"info"}]}
EXIT CODE: 0
```

Exit 0, single `passthrough-profile` info diagnostic on `tracks.rules`, no errors/warnings - matches the
brief's expectation exactly. Removed the scratch file after verification (`rm target/tmp/passthrough-recipe.yaml`);
`target/` was already gitignored so this never touched the tracked tree.

## Step 6: commit

Staged exactly the three brief-named files and committed unsigned with the brief's exact message and trailer:

```bash
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md README.md
git -c commit.gpgsign=false commit -m "docs: passthrough spec amendments, D20 supersession note, README recipe (D38)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>"
```

Result: `5ce7be975bf292dec1f8d3c24dc3cf35759a6c03`, 3 files changed, 27 insertions(+), 1 deletion(-). Working
tree clean afterward. No push performed.

## Typography and conventions

Diff swept for the forbidden glyph set (em/en-dash, curly quotes, Unicode ellipsis, NBSP) via a Unicode
character-class grep over `git diff`: zero hits. All added prose used ASCII hyphens and straight quotes,
matching the surrounding files' existing style (e.g. spaced ` - ` for parenthetical asides, as already used
throughout both spec files). No new pattern introduced; no deliberate deviation from `docs/conventions.yaml`
/ `docs/process-conventions.yaml` house rules - the work followed `proc-04-spec-wins` (spec is the contract,
self-contradiction sweep run after the amendment) and the load-bearing-fact-from-source rule (schema and CLI
behavior verified against actual source/binary, not assumed from the brief's proposal text).

## Status

DONE

Commit: `5ce7be975bf292dec1f8d3c24dc3cf35759a6c03`

One-line summary: D38 spec/README documentation landed verbatim per brief, self-contradiction sweep clean (1 expected hit, no fix needed), README recipe schema- and CLI-verified (exit 0, `passthrough-profile` info notice), committed unsigned as specified.

## Fix round 1

Review verdicts: APPROVED (both), one Minor finding. Finding: the new README "Pure passthrough" prose
paragraphs (README.md:69-72 and :83-87 post-insertion) were hard-wrapped at ~70 columns, deviating from the
README's own one-paragraph-per-physical-line convention (every other paragraph in the file is a single line,
up to 623 chars).

Fix: joined each of the two wrapped prose paragraphs into a single physical line. Content unchanged, wrapping
only; the yaml code block untouched.

Evidence (render-neutral check, as directed):

- `git diff --word-diff README.md` before committing showed **zero word-level changes** - no `[-...-]` /
  `{+...+}` markers anywhere, both hunks render as plain unchanged text (line joins only). Plain diff stat:
  `1 file changed, 2 insertions(+), 9 deletions(-)` (4+5 wrapped lines replaced by 1+1 joined lines).
- Typography grep over the diff for the forbidden glyph set (em/en-dash, curly quotes, Unicode ellipsis,
  NBSP): zero hits.

Commit: staged only `README.md`, committed unsigned with the trailer:

```
6080b0d608c8e64afd75a41a12010d5f0eb2bbf4 docs: unwrap README passthrough prose to one-paragraph-per-line (review Minor)
```

Working tree clean afterward; no push.

Status: DONE
