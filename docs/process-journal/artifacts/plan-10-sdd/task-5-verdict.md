# Task 5 verdict - Plan 10 (W2: the comment line-citation sweep)

**Verdict: APPROVED_WITH_MINORS.**

The transformation is correct at every one of the 21 rewritten comments, verified
against the cited code rather than against the report. The corpus reproduces
exactly (20/13 under expression A, 4/4 under expression B, union 24 lines across
16 files, SET-EQUAL to the commit's changed-file set). Both absence checks return
empty on the end state with their own fires. Every hunk falls inside a comment,
proven structurally with a fired control. All eleven gate parts are green
foreground on my own run. Nothing under `docs/` was touched.

The minors are one surviving member of the class that neither prescribed
expression can see (in a file outside this task's exhaustive Files list, so not
the implementer's to fix - but it does falsify the plan close's intended
"Task 5 closes the whole class" disposition sentence), and two cosmetic ragged
re-wraps whose absence the report claims and which are present.

---

## Findings

### 1. MINOR (class-level; NOT this task's to fix) - a surviving member of the swept class, invisible to both prescribed expressions

`.github/workflows/ci.yml:90`

```
88:      # Plan 5.5 Task 12 (#18b): rustdoc correctness as the ninth gate part.
89:      # #![deny(missing_docs)] already gates presence; it says nothing about
90:      # correctness, so a broken intra-doc link (queue.rs:73, linking a
91:      # private item) rotted silently since Plan 4 until this task. All
```

This is a source comment locating code by line number, in the exact form the
owner's ruling bans, and it is already stale: at HEAD `queue.rs:73` is
`pub struct QueueOpts {`, while the broken intra-doc link the sentence is about
(`worker_count`) sits at `:75`.

```
72: #[derive(Debug, Clone, Copy)]
73: pub struct QueueOpts {
74:     /// Requested worker count; clamped to >= 1, then further capped at the
75:     /// batch's spec count (see the private `worker_count` helper) so a `--jobs` far larger
```

At `004e1e8`, the commit that wrote the ci.yml comment, `queue.rs:73` WAS the
`worker_count` link line - so the citation was true when written and rotted, the
ruling's exact thesis.

**Why neither expression sees it.** The miss is in the CITING-FILE selector, not
in the cited-extension alternation. Both expressions open with
`git ls-files -- '*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py'`; `.yml` is not in
that set, so the file is never read. Expression A's cited-extension list would
have matched `queue.rs:73` on sight.

**Evidence run** (independent instrument, cited-extension set derived from the
tree's own extension tally rather than from the plan):

```
$ git ls-files | grep -vE '\.(rs|ts|vue|mjs|js|py)$' | grep -vE '^docs/' \
  | xargs grep -lE '[A-Za-z0-9_./-]+\.(rs|ts|vue|mjs|js|py|toml|ftl|json|jsonc|yaml|yml|md|sh|css|html)[:][0-9]+'
.github/workflows/ci.yml
```

Exactly one file, one line. Fired against a known-present control -> 1; negative
control -> 0 (appendix E2).

**Scope reading, both directions.** Against it: the convention's statement
enumerates `//`, `///`, docstrings, TS and Vue comments, and a `#` YAML comment
in a CI workflow is in none of them. For it: the enumeration is preceded by
"Applies to every source-comment form", the parenthetical is instances of that
rule rather than its extent; and the SCOPE BOUNDARY's exclusion list is
enumerated too - "a review verdict, a journal entry, a ledger occurrence ref or a
tracker" - and a CI workflow is not one of those. My judgment: **in scope.** It
is a live explanatory comment about a code defect in a tracked build file, it
locates that code by line number, and it is stale, which is the whole of what the
ruling addresses.

**Exact required change - and it is NOT an edit to this task.** The plan's own
Step 1 rules a hit in a file off the Files list a NEEDS_CONTEXT rather than a
silent edit, the Files list is EXHAUSTIVE, and "Must not decide: the corpus
(measured with both prescribed expressions, not chosen)". The implementer ran
exactly what it was told to run and made no tree-wide negative claim beyond the
two expressions. So:

- Nothing to fix in commit `1a23283`.
- **Controller action, before the plan close:** acceptance row W2-a reads "No
  tracked source comment cites `<filename>:<line>`". As written that is a
  tree-wide universal and it is now false; as measured it is true within a
  six-extension selector. Either restate W2-a with its selector named, or fold
  `ci.yml:90` into a follow-up.
- **Controller action, ROADMAP disposition:** the plan close's disposition
  sentence for the "Docs accuracy" stale-citation entry says "Task 5 closes the
  whole class it names". With `ci.yml:90` alive that sentence would be untrue.
  Narrow it, or route the remainder.

### 2. MINOR - two ragged re-wraps, and the report claims there are none

`e2e/smoke.spec.ts:1437` and `src/views/EditorView.vue:87`.

```
e2e/smoke.spec.ts:1436  // selected rule through `SectionWidget` over the `trackRule` registry --
e2e/smoke.spec.ts:1437  // byte-for-byte the
e2e/smoke.spec.ts:1438  // machinery `ListWidget` already uses for AttachmentRule items

src/views/EditorView.vue:86  // beneath the grid. The panel is pure registry composition, byte-for-byte
src/views/EditorView.vue:87  // the machinery `ListWidget.vue`
src/views/EditorView.vue:88  // already uses for AttachmentRule items: it synthesizes a `{ kind:
```

Both short lines sit mid-sentence with a continuing comment line under them, so
they are genuine ragged fills rather than paragraph-final short lines. The report
(judgment call 2) states: "The other fifteen comments re-wrapped so no line is
left ragged." That is a claim about the tree and it is measurably false at two
sites.

**Evidence run:** an instrument over the committed files that flags an ADDED
comment line shorter than 45 characters which does not end a sentence and whose
next committed line is also a comment. `ragged_count = 2`, the two above. The
other eight sub-45-character added lines all terminate a sentence and are
correctly excluded (appendix E5).

**Exact required change:** cosmetic only, and reasonably deferrable. Re-flow the
two paragraphs so no interior line is short, e.g. `smoke.spec.ts:1436-1438` ->
`// selected rule through \`SectionWidget\` over the \`trackRule\` registry --` /
`// byte-for-byte the machinery \`ListWidget\` already uses for AttachmentRule` /
`// items`. If left as is, the report's judgment-call-2 sentence should be
corrected rather than the code, since the substance is untouched.

### 3. NIT - report row 20's line numbers are pre-edit values, unlabelled

Report row 20 gives `attachmentRuleFields (:213-223)` and
`grep -n 'editor-attachment-rule'` -> `:215, :219, :222`. Those are correct in
the PRE-state tree (verified: `44f1c8e:src/editor/registries.ts` has
`export const attachmentRuleFields` at `:213`, keys at `:215/:219/:222`). In the
committed tree they are `:212` and `:214/:218/:221`, because this task's own
registries.ts edit removed a line above them. A reader checking the report
against HEAD finds every number one off. Permitted - the report is a process
artifact and the moment is part of the claim - but the moment is not named.
**Change:** none required; name the tree next time a report cites a span it is
about to move.

---

## Adjudications

### Q1. `src/editor/registries.ts` - dropping the parenthetical instead of substituting

**The mapping.** Plan Step 2: "``design \`:889-936\`` -> `D45`, whose surrounding
prose already names the 43-row table". The sentence pre-state read "straight from
the 43-row table in D45 (design `:889-936`); do not re-derive them here."
Literal substitution yields "in D45 (D45)". The implementer wrote "straight from
the 43-row table in D45; do not re-derive them here."

**The case for calling it a deviation.** The task carries "No design latitude, in
either form" and "Must not decide: the transformation (the convention's own
handle)". Two sibling mappings in the same bullet were applied as literal
substitutions (`design D48`, `D44`), so this one departs from a prescribed
replacement text. A strict reading of the latitude rule says a prescribed string
that cannot be applied is a routing trigger, not a keyboard call, and the correct
return was NEEDS_CONTEXT naming the tautology.

**The case for calling it faithful.** The bullet states its own purpose in the
same breath: "Bare spans lose the span and keep what they already say ... Nothing
is invented: the surviving text is the identifier the comment itself supplies."
The identifier D45 does survive - it is four words to the left. The plan's gloss
is the tell: it writes the first target as `design D48` and the third as bare
`D45`, and it appends "whose surrounding prose already names the 43-row table",
i.e. the plan author had already seen that this site's identifier is present and
that the parenthetical is what is left over. Applying the substitution literally
would produce a comment that says nothing twice, which serves neither the mapping
nor the ruling. The result also satisfies the ruling's handle exactly: the
volatile locator is gone and a durable identifier locates the material.

**VERDICT: faithful application, not a deviation. No NEEDS_CONTEXT was owed.**
The mapping's object is the identifier, not the parenthesis, and the plan's own
gloss names that. `latitude-carveout-zero-content-structural-forks` reaches the
same result from the other side: the edit has no API surface, no data format, no
verification change and nothing user-visible, and "an explicit enumeration in
brief, design or spec always wins" does not bite here because the enumeration's
stated content - the surviving identifier - is what was preserved.

### Q2. `run_live.rs`'s recipe anchor - heading rather than fence

**The question.** The comment now reads "The YAML block under README.md's 'Pure
passthrough: a profile with zero rules' heading, verbatim." above a 177-byte
literal that must match the README byte for byte. Does a heading anchor let a
future maintainer find the exact block, or does byte-exactness need the fence?

**Case for naming the fence.** The comment's obligation is stronger than
"see this section" - the doc comment above the test says outright "if the README
recipe's YAML ever changes, this literal must be updated to match it
byte-for-byte". A section heading spans prose, and prose is not the thing the
literal must match.

**Case for the heading.** A fence has no name. Naming it means naming its line
span, which is the class this task exists to delete - so "name the fence" is only
a real option if the fence gains an identity, which it does not have. The
heading, by contrast, resolves the block MECHANICALLY: I extracted the recipe
from the README using the heading text as the only anchor, with no line number
anywhere in the extractor, and compared it to the literal parsed out of
`run_live.rs`.

```
$ awk '/^### Pure passthrough: a profile with zero rules$/{inh=1;next} /^#{1,3} /{if(inh)inh=0} inh' README.md \
  | awk '/^```yaml$/{f=1;next} /^```$/{if(f)f=0} f'      # -> 177 bytes
RESULT: BYTE-IDENTICAL
177 readme-recipe.yaml
177 test-literal2.yaml
```

Fired control: mutating one byte of the extracted copy makes `cmp` differ, as it
must. And the heading is unambiguous - between that heading and the next one
there is exactly **one** opening ```` ```yaml ```` fence (measured: `grep -c
'^```yaml'` -> 1, total fence lines 2).

**VERDICT: the heading anchor is sufficient, and naming the fence would have been
worse.** The anchor is machine-resolvable to the exact block today, it is
unique within its section, and the alternative reintroduces the banned form. If
the README ever grows a second YAML block under that heading, the anchor needs a
qualifier - worth one sentence in the harvest, not a fix.

### Q3. Two rewrites that name a LOCATION rather than a symbol

**(a) `identify.rs:557` - "parse_attachment's contract (its own doc comment)".**
The convention's handle is "replace the number with the symbol the line sits in;
where no symbol names it, name the nearest one plus what you mean ('the third arm
of `scalar_eq`')". The rewrite is precisely that second clause with the symbol
hoisted to the front of the sentence: symbol (`parse_attachment`) plus what is
meant (its doc comment). Writing "(parse_attachment's doc comment)" would repeat
the subject inside its own parenthetical. Verified against the target: at HEAD
`fn parse_attachment` is at `:230` and its doc comment at `:228-229` reads
"Required fields (`id`, `file_name`, `size`) missing or wrong-typed drop the
entry" - exactly the contract the citing comment paraphrases, and exactly what
the stale `:224-225` meant (`:224-225` is `})` / `}` of the preceding function).
Zero volatility survives.

**(b) `run_live.rs:361` - the full repo path `crates/muxsmith-cli/src/commands/run.rs`.**
This is not a location instead of a symbol; it is file-plus-symbol, which the
convention explicitly prefers: "Naming the FILE stays normal and wanted ...
file-plus-symbol is unambiguous where a bare symbol may not be", and Step 2:
"Where a file name survives the rewrite, it survives as a path that is
unambiguous in the repo." The comment names the path, the function (`run`) and
the expression (`run_document(batch_document(..))`). Two tracked files are named
`run.rs`, measured, so a bare basename would have been ambiguous; and the shorter
`commands/run.rs` disambiguates too but is one grep away from the reader rather
than zero. Verified: `crates/muxsmith-cli/src/commands/run.rs`'s `run` (`:46`)
has its `run_document(batch_document(..))` build at `:219-220`, unconditional and
after the queue; the file's only other such build (`:151`) sits inside
`if specs.is_empty()` + `if json` and returns, so "unconditional" is the
distinguishing qualifier and it is accurate. `src-tauri/src/run.rs` has no
unconditional post-mux build (its `run_document` calls are in `plan_run`'s
early-return arms at `:259/:267/:275/:290` and in `start_run` at `:466`).

**VERDICT: the convention's handle covers both; neither is a residue of the swept
class.** The class is line NUMBERS. Neither rewrite carries one, both resolve
mechanically at HEAD, and the path form is what the convention and the plan's
disambiguation bullet jointly require.

---

## Dimension-by-dimension result

| # | Dimension | Result |
|---|---|---|
| 1 | Member the two expressions cannot see | **One found** (Finding 1). Bare-span form clean under a strict superset; split-across-linebreak, spelled `line N`, `L412`/`#L412`, `file.ext N` forms: none |
| 2 | Named symbol == what the comment MEANS | PASS, all 21 verified against the target; all nine staleness claims verified independently |
| 3 | Comment text only | PASS, proven structurally with a fired control |
| 4 | The three README-citing sites | PASS; anchors exist, quotation dropped, literal byte-identical |
| 5 | The four expression-B sites | PASS; nothing invented (Q1 adjudicated) |
| 6 | The two `run.rs` files | PASS; every surviving file reference unambiguous, measured |
| 7 | Scope boundary (`docs/` untouched) | PASS, 0 of 16 under `docs/`, control fires at 1 |
| 8 | House: the convention itself, latitude, typography, counts | PASS; no new volatile locator among 51 added lines, control fires 24 times on the removed lines |
| 9 | No-work-needed premises | PASS; all 24 matched lines are comment lines, 0 code lines |
| 10 | Verification quality | PASS; 11/11 gate parts green foreground, aggregates recomputed |

### Aggregates recomputed independently

| Claim | Report | My measurement |
|---|---|---|
| Expression A | 20 lines / 13 files | 20 / 13 |
| Expression B | 4 lines / 4 files | 4 / 4 |
| Union | 24 lines / 16 files | 24 / 16, SET-EQUAL to the commit's file set |
| Comments rewritten | 21 | 21 (derived from the corpus, per file: run_live 4, suggestions 3, one each in the other fourteen) |
| Stale at HEAD | 9 of 24 | 9, each verified at the target |
| Diff | 51 insertions / 40 deletions | 51 added / 40 removed |
| Locator-bearing removed lines | (not stated) | 24 - independent confirmation of the corpus size |
| `cargo test --workspace` | `39` ok result lines | 39, `0 failed` on every one |
| `pnpm test:e2e` | `68 passed` | `68 passed (2.9s)` |
| `ledger-lint` | 541 entries across 4 files | 541 entries across 4 files |
| mkvmerge skip markers | 0 | 0; `mkvmerge v100.0` present |

---

## Evidence appendix

Independent instruments, all under
`/tmp/claude-1000/-home-senol-agents-peter/5ea9158f-75c4-401c-a07c-c8c493a4c19c/scratchpad/t5rev-independent/`.
Nothing the implementer wrote was re-run; no shared default path was used. No
tracked file was mutated at any point (proven below).

**E1 - tree identity, per file, against blobs.** `git hash-object <file>` vs
`git rev-parse 1a23283:<file>` over every tracked file: `differ_count=0`.
`git diff --name-status 1a23283`: empty. `git status --porcelain
--untracked-files=all`: empty. Re-run after the full gate: `porcelain_lines=0`,
`post_gate_differ=0`. The 16 changed files matched individually by blob hash.
No controller commit under `docs/` landed while I worked.

**E2 - dimension 1 instruments** (`d1e2.txt`, `control.rs`, `control-neg.rs`).
Search sets derived from the artifacts: the cited-extension alternation from a
tally of every extension present in `git ls-files`; the citing-file set from
`git ls-files` minus `docs/` minus binary/lockfile noise (189 files).
- D1b, `<name>.<ext>:<digits>` over every tree extension, all non-`docs/` files:
  1 hit, `.github/workflows/ci.yml:90` (Finding 1). Fires on control -> 1.
- D1e, UNANCHORED `:[0-9]+` over the plan's own selector, prefix stripped BEFORE
  matching: 57 hits, every one benign (SRT timestamps, mkvmerge `--track-order`
  strings, JSON payloads, `{value:0>2}` format specs, `127.0.0.1:4173`, `1:1`
  prose, Python slices, ISO timestamps). Strict superset of expression B, so the
  anchoring in B hides nothing. My first run of this reproduced the plan's own
  documented hazard - the `sed`-added `file:line:` prefix matching the pattern -
  and was re-run with the prefix stripped.
- D1c, spelled `line(s) N` / `Zeile N`: 2 hits, both `"mkvmerge output line 1"`
  test fixture strings. Fires on control -> 1.
- D1d, `#L412` / `L412` / `:L412`: 0 hits. Fires on control -> 1.
- D1f, citation split across a line break (previous line ends `<name>.<ext>:` or
  a lone `:`, next line starts with digits): 5 candidates, all false positives
  (numbered lists in `check-i18n.mjs` and `ledger-lint.py`, a YAML key in a
  rustdoc sentence, a Python `for` header).
- D1g, `<name>.<ext>` followed by whitespace/`#` and digits: 1 hit,
  `planner_resolution.rs:2413` `IDEAS.md #5`. Not a member: `docs/IDEAS.md:136`
  is `## 5. Zero-track outcome options`, so `#5` is the section's own number, a
  durable name. Fires on control -> 1.
- Negative control (a comment with no locator) returns 0 for every expression
  above.
- Doc-comment code fences need no separate pass: D1b and D1e are supersets that
  read every line of every selector file regardless of fence.

**E3 - corpus reproduction.** Both prescribed expressions, verbatim, on the end
state: A exit 123 (no batch matched), B empty. Pre-state fires run over a tree
extracted with `git archive 44f1c8e | tar -x` into the scratchpad, never against
the working tree: A -> 20 lines / 13 files, B -> 4 / 4, union 16 files,
`diff` against the commit's changed-file set -> SET-EQUAL. Full outputs in
`fireA.txt` / `fireB.txt`.

**E4 - comment-only proof (dimension 3), two independent methods.**
(a) Every `+`/`-` line of `git diff -U0 44f1c8e 1a23283` filtered against
`^[[:space:]]*(///|//!|//|\*/|\*|/\*\*|/\*|<!--)`: `noncomment_count=0`. Control:
the same filter over `44f1c8e^..44f1c8e` returns 8, so the filter can report
non-comment lines.
(b) Per file, old and new blobs with all comment lines stripped, compared with
`cmp`: `CODE-IDENTICAL` for all 16. The stripper's only over-reach in these files
is two Rust deref lines in `planner.rs` (`*counts...`), neither of which appears
in the diff, so (a) and (b) together are airtight.

**E5 - ragged-fill instrument.** Python over the committed files: added comment
line, length < 45, not sentence-terminal, next committed line also a comment.
`ragged_count = 2`.

**E6 - README verification.** `grep -c 'every command takes' README.md` -> `0`;
fired control `grep -c 'Scriptable everything'` -> `1`. Anchors present:
`74:### Pure passthrough: a profile with zero rules`, `89:## ✨ What you get`,
`98:- **Scriptable everything**: ... each take \`--json\``. README `:91` today is
`- **A real dry-run.**`, confirming Task 4 staleified the old citation.
Byte comparison of the heading-extracted recipe against the parsed Rust literal
(`run_live.rs:321`): 177 bytes each, BYTE-IDENTICAL, with a fired mutation
control.

**E7 - target verification (dimension 2), each opened at HEAD.**
`report/json.rs:44` = `pub fn batch_document(`; `identify.rs` `parse_attachment`
`:230` with doc `:228-229`; `planner.rs` `fn delta_for` `:1820`, `AddExact`
`:1823`, `AddNotExact` `:1828` (cited `:1812`/`:1817` fall in the preceding
function and its trailing comment - stale confirmed); `matcher.rs` `fn scalar_eq`
`:202`, six arms plus `_ => false`, no `(Str, Bool)` arm; `generated.rs:42`
`("id", PropType::Integer)` inside `pub static MATCHABLE_PROPERTIES` (`:7`);
`RunHistory.vue` `data-testid="jobs-history-run"` `:170` with `:aria-current`
`:172` inside the `<button>` at `:168`; `src-tauri/src/lib.rs`
`validate_profile_body_reports_load_failure_with_no_mkvmerge_key` `:676`, cited
`:557-563` now the Tauri `invoke_handler` list; `planner.rs:53` the
`Assignment::track_kind` doc line, accurate; `profile/validate.rs`
`fn validate_locator` `:454`, `Some(false)` rejection `:472`;
`src/jobRowState.ts` `jobStateKey` `:44`, four `case` arms, no `default`, closes
`:55`; `registries.ts` `attachmentRuleFields` `:212` (pre-state `:213`);
`profile/model.rs:363-364` carries the quoted "the only valid value is `true`".

**E8 - ambiguity measurement.** Tracked-file basename counts: `run.rs` 2,
`validate.rs` 2, `identify.rs` 2, `lib.rs` 4; `generated.rs`, `model.rs`,
`matcher.rs`, `planner.rs`, `RunHistory.vue`, `registries.ts`, `jobRowState.ts`,
`json.rs` all 1. Every surviving file reference in the rewritten comments is
unambiguous: the two ambiguous basenames that survive carry paths
(`profile/validate.rs`, `crates/muxsmith-cli/src/commands/run.rs`), and the
`identify.rs` and `lib.rs` references were dropped entirely in favour of a
same-file symbol.

**E9 - the full gate, foreground, in `BUILDING.md`'s order (11 parts: 6 Rust, 4
frontend, 1 house).** Exit codes captured directly.

| # | Command | Exit | Evidence |
|---|---|---|---|
| 1 | `cargo fmt --all --check` | 0 | no output |
| 2 | `cargo clippy --workspace --all-targets -- -D warnings` | 0 | ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s`` |
| 3 | `cargo test --workspace` | 0 | 39 `^test result: ok\.` lines, none with a nonzero `failed` |
| 4 | `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items` | 0 | `Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files` |
| 5 | `cargo deny check` | 0 | `advisories ok, bans ok, licenses ok, sources ok` |
| 6 | `cargo clippy --workspace --all-targets --target x86_64-pc-windows-msvc -- -D warnings` | 0 | ``Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s`` |
| 7 | `pnpm lint` | 0 | `$ eslint .`, no findings |
| 8 | `pnpm build` | 0 | `✓ built in 154ms` |
| 9 | `pnpm check:i18n` | 0 | `check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated, 22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for parity against 7 en/ catalog(s)).` |
| 10 | `pnpm test:e2e` | 0 | `68 passed (2.9s)` |
| 11 | `python3 scripts/ledger-lint.py` | 0 | `ledger-lint: 541 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` |

No `FAIL BUILDING.md:` line. The seven tests whose comments this task rewrote all
RAN and passed, including the `have_mkvmerge()`-gated
`readme_passthrough_recipe_with_title_template_survives_dry_run_and_run`
(0 skip markers; `mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit` present).

**E10 - stale-at-birth verification (harvest item 1).**
`git log -S 'lib.rs:557-563' -- src-tauri/src/lib.rs` -> `997666a`.
In `997666a^`, `src-tauri/src/lib.rs:557` is
`fn validate_profile_body_reports_load_failure_with_no_mkvmerge_key() {` with
`:558-559` its `tempdir()` and `validate_profile_body(&dir.path().join("missing.yaml"))`.
In `997666a` itself - the commit that ADDS the citing comment at `:748` - the
same span is `pub fn run()`'s Tauri builder, and the named test has moved to
`:694`. Both grepped at the two trees. Confirmed.

---

## HARVEST

**H1 (highest value, and it is the third instance of one shape). The measuring
expression's own SELECTOR is an enumeration, and enumerations in measuring
position have now undercounted this corpus three times.** The ROADMAP's own
stale-citation entry records the first two: the controller's 17, from a pattern
that "enumerated the CITED file extensions and left out `.md`"; then 20/13, from
two expressions that were themselves the fix, plus a controller ruling folding in
a form the first expression could not see. The entry draws the moral itself -
"A mis-enumerated set in a measuring position produced the undercount - the same
defect shape the ruling addresses, one level up." Finding 1 is that defect one
level further up: the corrected expressions enumerate the CITING file types
(`'*.rs' '*.ts' '*.vue' '*.mjs' '*.js' '*.py'`) and leave out every other tracked
source-ish type, so a `#` comment in `.github/workflows/ci.yml` citing
`queue.rs:73` was never read. The cited-extension list was widened twice; the
citing-file list was never audited at all.

The generalizable rule, with a readable trigger and an executable handle:

> **When a search expression carries TWO enumerations - what it reads and what it
> matches - fire and audit BOTH.** The trigger is readable: your expression opens
> with a file selector and closes with an alternation. The handle: derive each
> set from the artifact (`git ls-files | sed 's|.*/||' | awk -F. 'NF>1{print $NF}'
> | sort | uniq -c` for the read set), not from recall of what should be in it,
> and state the selector alongside any absence claim the expression supports. A
> control fired against a known-present member passes for BOTH enumerations while
> a missing member of EITHER stays invisible.

Candidate as a Tier-2 house entry in its own right, or as a sharpening
occurrence on the existing `comments-locate-by-symbol-never-by-line-number`
entry. My read: its own entry, because its subject is measurement rather than
comments, and it now has three dated instances in this repo.

**H2. A citation that never once pointed at its target in a committed tree**
(`src-tauri/src/lib.rs`, verified at E10). This is the sharpest available ground
for the owner's ruling, sharper than any staleness-over-time example: the author's
own diff moved the target 137 lines down within the same commit that wrote the
citation. It defeats the strongest counter-argument to the ruling ("just keep it
updated"), because there was no window in which keeping it updated would have
worked - the number was wrong the instant it was committed, and no reviewer,
gate or CI leg could see it. Ledger-worthy as an occurrence on
`comments-locate-by-symbol-never-by-line-number`, with the commit pair named.

**H3. The Tier-2 entry's occurrence records the corpus as 20/13, which is
expression A only.** The swept union is 24 lines across 16 files. The entry's
occurrence text already carries a self-correction about the FIRST mis-count
(17 -> 20) while itself carrying the second (20 -> 24). No task edits
house-knowledge YAML, so this is a close-time controller edit. Note the
interaction with H1: whatever number replaces it should name its selector, or it
becomes the fourth instance.

**H4. The convention statement's own comment-form enumeration is
under-specified.** "Applies to every source-comment form (`//`, `///`,
docstrings, TS and Vue comments)" reads as exhaustive to an implementer even
though it opens with "every". `#` comments (YAML, TOML, shell, Python) are the
gap Finding 1 sits in, and a `<!-- -->` form exists in the tree too
(`BUILDING.md`'s gate markers). Same class as H1: an enumeration in normative
position. Cheap repair at the same close-time edit as H3 - either add the forms
or replace the parenthetical with "regardless of the comment syntax the file's
language uses".

**H5. Adjudication Q2's residual.** The heading anchor resolves the recipe
mechanically today because exactly one ```` ```yaml ```` fence sits between that
heading and the next. That uniqueness is a property of today's README, not of the
anchor. Not worth a fix; worth one sentence if a "how to anchor into a document
without a line number" pattern is ever written up - the anchor is the nearest
NAMED container that is unique among its siblings, and a container that stops
being unique needs its qualifier restored.

**H6. Over-restriction watch (`latitude-carveout-zero-content-structural-forks`
explicitly asks reviewers to report these).** No stop to report in either
direction here. The one place the boundary could have bitten - Q1's collapsed
mapping - I ruled inside the grant, and the grant's "repairing a reference which
the task's OWN enumerated edit invalidated, inside a LISTED file" clause covers
the re-wraps cleanly. Recording the null result because the watch is a collection
mechanism and an unreported clean pass is data too.
