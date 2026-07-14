# Task 2 review verdict: D38 documentation (spec amendments, plan-3.5 supersession, README recipe)

Reviewer worktree: `/home/senol/Git/Muxsmith/.worktrees/plan58-a` (branch `plan58-a`), commit `5ce7be9`, review range `f8e863e..5ce7be9`. Independent review; did not implement. Task 1 (`f8e863e`) out of scope, already separately reviewed.

## Findings

### Minor #1: README recipe prose hard-wrapped, deviating from the file's own one-paragraph-per-line convention

- **Evidence:** `README.md:69-72` and `:84-86` (the new "Pure passthrough" subsection). These two prose paragraphs are wrapped across multiple physical lines at ~70 columns. Every other prose paragraph in `README.md` is a single unwrapped physical line, confirmed by measuring line lengths across the whole file (`awk '{print length($0)}' README.md | sort -n`): existing paragraphs run up to 623 characters on one line (e.g. line 63, the "dry-run is the heart of the tool" paragraph, is 386 characters on one line); the two new prose paragraphs are the only sub-100-character prose lines in the file outside code/YAML/list items.
- **What's wrong:** the added text was carried over verbatim from the brief's own pre-wrapped presentation (task-2-brief.md:48-68 is wrapped at the same ~70-column width) instead of being reformatted to the target file's established convention. This does not affect rendering (Markdown collapses adjacent non-blank lines into one paragraph, so the visual output is identical), but it is a raw-source house-pattern deviation the same way the implementer correctly avoided it for the 4.5 spec edit (explicitly matched `v1-design.md`'s one-line-per-paragraph convention there).
- **Suggested fix:** join `README.md:69-72` into one physical line and `:84-86` into one physical line, matching every other paragraph in the file.
- **Severity:** Minor. Cosmetic, zero functional or rendering impact, does not block trust in the content.

No Critical or Important findings.

## Verified independently (not taken from the report)

- **Spec 4.5** (`docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:180`): appended sentence is character-for-character the brief's text, landed as a continuation of the existing physical line (file's own one-paragraph-per-line convention preserved). Content matches the D38 decision doc's Spec-amendments slot and the actually-shipped Task-1 behavior, confirmed by running the built CLI (below): info severity, `config_path: tracks.rules`, no params.
- **5.2 row**: inserted directly after the `EmptyPlan` row, before `OutputCollision`, exact brief text. Table syntax checked manually: 4 pipes bounding 3 cells, consistent with every neighboring row; no literal `|` inside the cell text that could break column alignment.
- **Self-contradiction sweep, re-run independently:**
  ```
  $ grep -n -i "at least one rule\|NoTrackRules\|no-track-rules\|zero rule" docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
  180:...(exactly the Step-1 passage, one hit)
  ```
  Single hit, exactly the sentence just written in Step 1; the new 5.2 row's wording doesn't contain any of the grep's keywords so it correctly does not surface as a second hit. Went one step further than the brief's literal instruction and independently read spec 5.4 (`v1-design.md:299`, the static-lint enumeration the decision doc cites as already-clean): confirmed it lists regex/template compilation, type errors, unknown properties, closed-domain checks, exact-only, and provable overlaps, with no "at least one rule" language anywhere - the ADR's claim holds, no fix needed anywhere in the spec.
- **plan-3.5 annotation** (`docs/superpowers/specs/2026-07-09-plan-3.5-design-decisions.md:156-160`): appended exactly at the end of the "Zero rules under `keep`" bullet, exact brief wording, correctly quotes D38 ("the invited contradiction arrived as the owner ruling"). The historical assumption text itself ("Today zero rules is `NoTrackRules` error... contradict if a pure passthrough profile should be legal.") is untouched - confirmed by reading the surrounding diff context, only new lines were added, nothing in the pre-existing bullet was reworded. This file's own paragraph convention (wrapped ~72-80 col prose, unlike `v1-design.md`'s one-line-per-paragraph) is correctly matched by the new annotation's wrapping - no deviation here, unlike the README case above.
- **README recipe, schema correctness** - checked directly against `crates/muxsmith-core/src/profile/model.rs`, not accepted from the report: `Profile.title: TitleCfg` is a top-level field (`#[serde(default)]`, model.rs:45-48); `TitleCfg` is `#[serde(untagged)]` over `Template(TemplateBlock) | Keyword(String)` (model.rs:373-378), `TemplateBlock` has exactly one field `template: String` (model.rs:122-128) - so `title: { template: 'S{season}E{episode}' }` is schema-correct as written. `TracksCfg.rules: Vec<TrackRule>` carries no `#[serde(default)]` (model.rs:308-311, confirmed by reading the struct directly), so `rules: []` must be written explicitly, matching the recipe.
- **README recipe, runtime verification** - reproduced myself, not trusted from the report:
  ```
  $ cargo run -p muxsmith-cli --quiet -- validate <recipe.yaml>
  [info] tracks.rules: This profile defines no track rules and tracks.unmatched is keep: a pure passthrough remux; every primary track is copied unchanged. If this is not intended, add track rules.
  0 errors, 0 warnings, 1 info.
  EXIT CODE: 0

  $ cargo run -p muxsmith-cli --quiet -- validate <recipe.yaml> --json
  {"diagnostics":[{"code":"passthrough-profile","config_path":"tracks.rules","params":{},"rendered":"...","severity":"info"}]}
  EXIT CODE: 0
  ```
  Exit 0, single `passthrough-profile` info diagnostic on `tracks.rules`, no errors/warnings - matches the report's claim exactly and matches the D38 decision doc's specified wire shape (code + `config_path` + no params). Also independently confirmed the catalog string in the running binary's own `locales/en/diagnostics.ftl:8` and `locales/de/diagnostics.ftl:15` against the D38 decision doc's approved PROPOSAL wording - matches.
- **README placement and register**: the subsection sits at the end of "How it works" (immediately after the "Happy with the plan?" paragraph) and directly before `## ✨ What you get` (now at line 89, shifted from the brief's cited line 67 by the insertion - correctly noted by the implementer). `### `-level subsections without emoji already exist elsewhere in the file (the four CLI-command subsections under "Using the CLI"), so an unemoji'd `###` under an emoji'd `##` is the established pattern, not a new one. Content covers core-83's three use cases (title-only change; attachment/chapter surgery; container normalization) via a close paraphrase of core-83's own wording, plus the accurate "Validation announces the passthrough with an info notice (`passthrough-profile`)" cross-reference to the actual diagnostic. Register (short declarative sentences, backtick-quoted config keys, sparing bold) matches the surrounding "How it works" prose, aside from the wrapping issue in Minor #1.
- **Typography sweep**, re-run independently over the full diff (`git diff f8e863e..5ce7be9`) for em-dash, en-dash, curly quotes, Unicode ellipsis, NBSP: zero hits across all 27 added lines.
- **Commit**: `git cat-file commit 5ce7be9` shows no `gpgsig` header (unsigned) and the `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer present, message text matches the brief verbatim. `git show --stat` confirms exactly the three brief-named files, nothing more - explicit staging honored, no `-A`/`.` sweep artifact.

## Verdict A: spec compliance

**APPROVED.** Every brief step (1-6) landed with content verified independently against the D38 decision document and the shipped Task-1 behavior (own CLI run, own source read), not accepted from the report. The self-contradiction sweep is clean, including a check one level deeper than the brief's literal grep (5.4 read directly). The README recipe is genuinely paste-runnable: reproduced exit 0 with the exact `passthrough-profile` info notice on a fresh run. Commit constraints (unsigned, trailer, explicit staging) all hold.

## Verdict B: quality

**APPROVED**, with one Minor noted (README paragraph wrapping, see Findings). Everything else - table syntax, cross-references, schema verification depth, register/tone match, non-destructive historical-text handling in the plan-3.5 file - is clean and, in the self-contradiction sweep and schema verification, went beyond the brief's minimum.

## House dimension

One Minor deviation: the new README prose does not follow this file's own one-paragraph-per-physical-line convention (see Minor #1). No other deviation from `docs/conventions.yaml`, `docs/process-conventions.yaml`, or `docs/product-boundaries.yaml` (`core-83`) - the work follows `proc-04-spec-wins` (spec amended to match the owner-approved ADR, self-contradiction sweep run) and `proc-07-verify-against-source` (schema and CLI behavior confirmed against actual source/binary, independently reproduced here rather than trusted).

## Harvest (patterns/repeated behavior worth the convention ledger)

- **A file's paragraph-wrapping convention is per-file, not global, and a brief's own formatting is not a reliable proxy for it.** `v1-design.md` and `README.md` both use one-physical-line-per-paragraph; `2026-07-09-plan-3.5-design-decisions.md` uses conventional ~72-80 column wrapping. The implementer correctly matched the target file's convention for the 4.5 spec edit and the plan-3.5 annotation, but missed it for the README because the brief's own step-5 text block happened to arrive pre-wrapped and got copied as-is. Candidate rule (process, agent-emergent, count 1 - flag, do not promote): when inserting brief-supplied prose into a target file, check that specific file's own paragraph convention (grep a few neighboring paragraphs for physical-line length) before pasting, rather than preserving the brief document's own wrapping.
- **Independent runtime reproduction of a documentation recipe (not just reading the report) caught nothing wrong here, but is the right default for any doc claiming "paste-runnable" against a live diagnostic/CLI surface** - consistent with `proc-07-verify-against-source`; no new ledger entry needed, this is that pattern's expected application to a docs task.

## APPROVED

---

# Re-review round 1: commit `6080b0d` (fix for Minor #1)

Delta reviewed: `5ce7be9..6080b0d`. All claims re-verified independently, not taken from the implementer's evidence.

- **Finding resolved.** The two hard-wrapped prose paragraphs of the "Pure passthrough" README subsection are now one physical line each (`README.md:69`, 287 chars; `:80`, 290 chars), matching the file's one-paragraph-per-physical-line convention. The YAML block is untouched (appears only as context in the diff).
- **Word-diff-neutral, confirmed.** `git diff --word-diff=porcelain 5ce7be9..6080b0d` contains zero word-level additions or removals - the change is purely joining physical lines, no wording touched.
- **Nothing else changed.** `git show 6080b0d --name-only` lists exactly `README.md`; stat is 2 insertions / 9 deletions, consistent with joining 4-line and 5-line paragraphs into one line each plus removing the internal line breaks.
- **Commit constraints hold.** `git cat-file commit 6080b0d`: no `gpgsig` header (unsigned), `Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer present, parent is `5ce7be9`.
- **Typography.** Per-glyph programmatic check over the delta diff (em/en dash, curly quotes, ellipsis, NBSP, Unicode minus): zero hits. (A byte-class `grep -c` initially reported false positives from UTF-8 byte overlap with the emoji context lines; the codepoint-precise check is authoritative.)

**Verdict:** Minor #1 resolved cleanly, no new findings, both verdicts stand APPROVED without qualification.

## APPROVED
