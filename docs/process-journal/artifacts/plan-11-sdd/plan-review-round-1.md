# Plan 11 - independent plan review, round 1

Reviewer: independent plan reviewer (fresh agent, no authorship of the plan).
Artifact under review: `docs/superpowers/plans/2026-07-30-plan-11-dependency-alerts-docs-accuracy.md`
at commit `148f19f` (819 lines).
Requirement set: `.superpowers/sdd/plan-11/plan-brief.md`.
Ground truth applied in order: the v1 spec, the three Tier-2 files, `docs/ROADMAP.md`'s
entries for the five work items, `~/agents/peter/prompts/software-dev-process.md`
for the plan's form.
Tree state during review: `master` at `148f19f`, working tree carrying one
co-writer change (` M docs/ROADMAP.md`). No measurement below reads that
file's working copy except where explicitly said.

All instruments this review built live under
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/pr11-review-independent/`
and were written in this pass: `rv_citescan.py` (citation scanner, deliberately
broader than the plan's two expressions and with no enumerated extension set),
`rv_ordinals.py` (ordinal + fence-aware length probe with a generated number-word
list), `rv_fencecheck.py` (fenced OLD/NEW string existence check, hand-transcribed
from the plan, fire-tested), `rvtarget/` (my own `cargo build` of `muxsmith-cli`),
`pcprobe/` (my own scratch `pnpm update postcss` reproduction), `denyprobe/` (my
own `deny.toml` variants driven through `cargo deny -c`). Nothing was run from a
path the author could also have written, and no author-produced script was
executed. The one shared artifact I used, `target/debug/muxsmith`, was checked
against my own build first: identical `sha256` (`6c99a5ce823e89db4021a4abb2c44bcc0bb8226650176c17b5b0aebef92f397d`),
so the shipped binary is not a tampered instrument.

---

## Verdict: NEEDS_FIXES

This is a strong plan. Its structure matches the brief exactly, its stream split
and serial ordering are argued rather than asserted, all five work items have a
named task, the acceptance map is genuinely walked in halves (32 rows, recounted
below and correct), every one of the twelve fenced OLD replacement strings exists
byte-for-byte exactly once in its target file, all four `git diff --exit-code`-class
rows now carry a fire sourced free from the tree's own asymmetry, and the
claimed-versus-prescribed separation is real and usable. Four of the six
refutations reproduce exactly; a fifth reproduces with two corrected sub-figures.

It nevertheless cannot ship as written, for two blocking reasons that are the same
defect class one level apart. **The `cargo deny` measurement's conclusion is
wrong**: cargo-deny 0.19.9 does have an `unsound` configuration key, its default
scope is `workspace`, `glib` is transitive, and setting the key to `transitive`
or `all` turns RUSTSEC-2024-0429 into a hard gate failure - so the plan's
"refinement" discards a hypothesis that was correct and replaces it with one a
single command refutes, in the work item whose entire purpose is that
measurement. And **the soundness control the plan prescribes for its repair
alternation cannot fire**: run verbatim, it returns only the six sites already
being removed, because the target it names carries "byte-literal equality" and
the alternation requires "byte-literal value equality". The check that guards the
very expression whose incompleteness the plan records as its own self-caught
defect is itself dead. Three more Important findings are wrong claimed
measurements or a false statement entering the authoritative spec; nine Minor
findings follow. Every finding below is a correction or an addition. I recommend
removing nothing the plan proposes.

---

## Findings

Severity key: **Blocking** = must change before owner review; **Important** =
must change, does not threaten the plan's shape; **Minor** = should change,
cheap.

### 1. Blocking - the `cargo deny` mechanism conclusion is refuted at the tool's own source

**Location:** authoring section, "### `cargo deny`: the disagreement is not a
database gap", final bullet; corrections table row 4; acceptance row W1-g's
`evidence` parenthetical; B1 Step 5's closing constraint.

**What is wrong.** The plan states: "it is not that the configuration does not
FAIL on the unsound class, it is that this cargo-deny version does not evaluate
that class here at all", and row 4 adds "'make the check fail on that class' may
not be an available configuration at 0.19.9". Both are false. cargo-deny 0.19.9
carries an `[advisories] unsound` key whose value space is
`["all", "workspace", "transitive", "none"]` and whose default is `workspace`.
`glib` is an external crate, so the default scope excludes it - which is exactly
why the advisory produces no error, no warning and no note. The class is
evaluated; the configured scope does not reach it. That is "evaluated and
tolerated by configuration default", i.e. the hypothesis on record was right.

**Evidence, all from my own instruments.**

- The observation reproduces exactly: `cargo deny check advisories` -> exit 0,
  `advisories ok`; `--show-stats` -> `advisories ok: 0 errors, 0 warnings, 36 notes`;
  `cargo deny -L info check advisories` mentions `RUSTSEC` on **54** lines,
  `RUSTSEC-2024-0429` on **0**, and emits **18** `advisory-ignored` notes.
- Probing my own config copies through `cargo deny check advisories -c <scratch>`
  (the repo's `deny.toml` untouched):
  - `unsound = "deny"` -> `error[unexpected-value]: expected '["all", "workspace", "transitive", "none"]'`.
    A *value* error, not an unknown-key error: the key exists.
  - `unsound = "all"` -> exit **1**, `advisories FAILED`, output containing
    `unsound advisory detected`, `ID: RUSTSEC-2024-0429`.
  - `unsound = "transitive"` -> exit **1**, same.
  - `unsound = "workspace"` -> exit 0, 0 mentions. `unsound = "none"` -> exit 0, 0 mentions.
  - `severity-threshold` -> `error[deprecated]: this key has been removed`, so the
    removed-key family is a different mechanism from the live one.
- At the tool's own source,
  `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/src/advisories/cfg.rs`:
  the `Default` impl sets `unmaintained: Spanned::new(crate::cfg::Scope::All)` and
  `unsound: Spanned::new(crate::cfg::Scope::Workspace)`, and the deserializer's
  fallback repeats both (`unsound.unwrap_or(Spanned::new(Scope::Workspace))`).
  `Scope` in `src/cfg.rs` is `All | Workspace | Transitive | None`.

That asymmetry is the whole explanation and the plan's own data already pointed at
it: 18 transitive *unmaintained* advisories produce `advisory-ignored` notes
because `unmaintained` defaults to `All`, while one transitive *unsound* advisory
produces nothing because `unsound` defaults to `Workspace`.

**Why it matters beyond accuracy.** The disposition the controller and owner will
be asked to rule on is materially different under the two accounts. Under the
plan's account there is nothing to decide because the tool cannot see the class.
Under the measured account there is a one-key decision with a known blast radius,
and it is exactly the owner-visible gate-coverage question the plan correctly
routes to him.

**What resolves it.** Rewrite the authoring bullet and corrections row 4 to state
the measured mechanism (key, value space, default `workspace`, glib transitive,
`transitive`/`all` producing a hard failure), and drop the "may not be an
available configuration" hedge. Keep B1 Steps 4-6 as they are - they prescribe the
right measurement and are not pre-empted by this - but add two things: (a) the
counterfactual as a required part of the demonstration (set the key in a scratch
config, watch the advisory appear, restore nothing because nothing in the repo
moved), since a mechanism account without it is still an argument; and (b) one
clause making clear that the "changes neither `deny.toml` nor the invocation"
constraint governs the REPO's file and the GATE's invocation, and does not forbid
a scratch-config probe via `-c`. As written, a careful implementer can read that
constraint as forbidding the only demonstration that settles the question.

### 2. Blocking - Task A3's prescribed soundness control cannot fire

**Location:** Task A3 Step 6, absence check R, the sentence beginning "**Soundness
control for the alternation itself:**".

**What is wrong.** The prescribed control is: run the repair expression without the
`':!docs/superpowers/specs/2026-07-11*'` exclusion, "which must return the plan-5.5
design document's B-4 row". Run verbatim, it returns nothing beyond the six sites
already under repair. `docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md`
line 66 reads "byte-literal equality"; the alternation requires "byte-literal
**value** equality". The document contains none of the four alternatives.

**Evidence.** `git grep -nE '<the four alternatives>' -- 'docs/superpowers/specs/2026-07-11*'`
returns zero lines. The same expression with only that exclusion dropped returns
the six repair sites and nothing else. Grepping the whole tree for the four
alternatives, the files carrying them outside the six repair sites are
`docs/ROADMAP.md` (1), `docs/process-journal/artifacts/plan-10-sdd/whole-branch-verdict.md` (2),
four plan-5.5/6/7 SDD review diffs, and this plan itself - every one of them inside
the expression's exclusion set.

**Why it is blocking rather than Minor.** After the repair, no live occurrence of
any of the four alternatives remains anywhere in the searched surface. The control
is therefore the *only* thing standing between a correct green result and a green
result produced by a broken pattern - and it is the control for the very
alternation whose earlier incompleteness the plan records as correction 5. An
implementer running it sees the six familiar lines and can read that as the
control having fired, which is the false-agreement the control exists to prevent.

**What resolves it.** Point the control at a target measured to contain a match.
Concretely: drop `':!docs/ROADMAP.md'` (1 hit) or `':!docs/process-journal*'`
(several) instead of the 2026-07-11 exclusion, and state the expected count. The
general rule this instance wants written down is in the harvest section.

### 3. Important - `codec_kind` is not declared in `generated.rs`, and the capability model is not what governs the `raw:` path

**Location:** authoring section, item-3 block, the bullet "**Why the retained set
is true as written, measured rather than argued**"; Task A3's `Read first` list.

**What is wrong.** The plan states that "`crates/muxsmith-core/src/capability/generated.rs`
declares both as string-typed". `grep -c 'codec_kind' crates/muxsmith-core/src/capability/generated.rs`
returns **0**. `generated.rs` carries `("language", PropType::String)` and
`("type", PropType::String)`; `codec_kind` is a *virtual* property whose String
type comes from `capability/mod.rs`'s `matchable_type`, which opens with
`if name == "codec_kind" { return Some(PropType::String); }`.

Separately, and more importantly for the argument's soundness: the `raw:` branch of
`exact_matches` bypasses the capability model by construction - it calls
`item.get(bare)` and hands the reported `PropValue` to `scalar_eq`. So the fact
that makes "byte-literal" true of `raw:language` is not the capability model's
declared type but that mkvmerge reports `language` as a JSON string. I verified
that at mkvtoolnix's own schema: in
`~/Downloads/mkvtoolnix/doc/json-schema/mkvmerge-identification-output-schema-v20.json`,
`language` and `language_ietf` are both `type: string` (59 track properties in
total, `codec_kind` absent - the plan's Step-7 observation, which I reproduce
exactly, including the five `type: number` properties).

**The conclusion survives.** The nine retained lines are genuinely TRUE, not
conveniently exempted: `raw:language` compares a `Str` scalar against a reported
`Str`, `scalar_eq`'s `_ => false` arm rejects any numeric scalar, and
`raw:codec_kind` cannot match at all because mkvmerge never reports the property.
I confirmed `raw_opt_in_diagnostic`'s trigger set is exactly
`matches!(bare, "language" | "codec_kind")` (`validate.rs:415`) and that
`b8_raw_language_is_byte_literal_no_normalization` passes.

**What resolves it.** Correct the cited ground to `capability/mod.rs`'s
`matchable_type` special case for `codec_kind`, and state the load-bearing fact
for the `raw:` path as what mkvmerge reports (schema v20 `language: string`)
rather than as what the capability model declares. Fix A3's `Read first` line,
which currently sends the implementer to `generated.rs` "for those two properties'
declared types" - a file that cannot answer for one of them.

### 4. Important - "twelve direct parents" is eleven under the plan's own command

**Location:** authoring heading "### `glib`: twelve parents, all one generation"
and its first sentence; acceptance row W1-j's evidence column; B1 Step 8's
expected-completion paragraph; the deferred-by-decision row for `glib`; the
self-review's recomputed-counts list. Five sites.

**Evidence.** `cargo tree -i glib@0.18.5 -e normal --depth 1` prints `glib v0.18.5`
plus **eleven** parents: `atk 0.18.2`, `cairo-rs 0.18.5`, `gdk 0.18.2`,
`gdk-pixbuf 0.18.5`, `gdkx11 0.18.2`, `gio 0.18.4`, `gtk 0.18.2`,
`javascriptcore-rs 1.1.2`, `pango 0.18.3`, `soup3 0.5.0`, `webkit2gtk 2.0.2`.
The plan's own enumeration is that same list of eleven, under the label "twelve".
The twelfth consumer, `glib-macros v0.18.5`, is a proc-macro edge that `-e normal`
excludes by design (it shows up in cargo-deny's inclusion graph, not here).

**Why it matters.** The plan's Global Constraints assert that "every count in this
plan was recomputed from its own list at plan-authoring", and B1 Step 8 hands the
figure to the implementer as the expected result. Re-running the named command
yields eleven, which under the plan's own rules is a divergence the implementer
must report. The conclusion - one glib, whole gtk-rs family at 0.18.x, nothing
0.20+, an upgrade project rather than a bump - reproduces exactly and is unaffected
(`grep -c '^name = "glib"' Cargo.lock` -> 1; no gtk-rs family crate at 0.20+ in
the lock).

**What resolves it.** Either state eleven, or state twelve *and* name the
`glib-macros` proc-macro edge with the command that shows it. Fix all five sites.

### 5. Important - the queue.rs provenance measurement is off by one line, at two sites

**Location:** authoring section, the bullet "**The `ci.yml` citation is stale,
verified at its target**"; Task A2 Step 2(a), "**Why that symbol, verified at the
target rather than at the cited line.**"

**What is wrong.** Both say that at `004e1e8^` "line 73 was the first line of the
`jobs` field's doc comment and line 74 carried the broken link
`(see [`worker_count`])`". Measured at `git show 004e1e8^:crates/muxsmith-core/src/executor/queue.rs | cat -n`:
line 71 `pub struct QueueOpts {`, line **72** `/// Requested worker count; clamped to >= 1, then further capped at the`
(the first line of the doc comment), line **73** `/// batch's spec count (see [`worker_count`]) so a `--jobs` far larger`
(the line that itself carries the broken link), line 74
`/// than the batch never spawns idle OS threads. Default 1 (sequential).`

**Why it matters, and why the conclusion strengthens.** The plan instructs the
implementer to "verify both by opening the file and by reading the citing commit's
parent" and says "the authoring section pastes what those two reads returned". The
paste is wrong, so a diligent implementer finds a mismatch and owes a report. The
identification is right and in fact tighter than the plan claims: the cited line
*is* the link line, so `QueueOpts::jobs`'s doc comment and the then-linked private
`worker_count` helper are exactly what the comment meant. Everything else in the
bullet reproduces: `queue.rs` resolves to one tracked file; line 73 at HEAD is
`pub struct QueueOpts {`; the citing commit is `004e1e8`
(`ci: cargo doc -D warnings as ninth gate part (#18b)`), found with
`git log -S'broken intra-doc link' -- .github/workflows/ci.yml`; today's doc reads
`(see the private `worker_count` helper)` as a code span.

**What resolves it.** Correct both sentences to "line 73 carried the broken link
inside `QueueOpts::jobs`'s doc comment, whose first line was 72".

### 6. Important - the new spec exit-code bullet states something false on Windows

**Location:** Task A4 Step 3, the fenced replacement, second sentence.

**What is wrong.** The fenced text says an interrupted `validate`, `dry-run`,
`identify` or `schema` "dies by signal and the shell reports 130 by its own
128-plus-signal convention rather than by anything Muxsmith returns". That is a
POSIX-shell fact stated without scope, in a document that governs a three-OS
product. On Windows an un-handled console Ctrl+C terminates the process with
`STATUS_CONTROL_C_EXIT` (`0xC000013A`), and neither `cmd.exe` nor PowerShell
applies a 128-plus-signal convention. D16 itself picked `ctrlc` *because* it is
"cross-platform, including Windows console events", so the platform dimension is
live in this project's own record, and D15 states the convention with its
qualifier attached ("128 + SIGINT, shell convention") rather than generalised.

The rest of the bullet is verified: `130` appears **0** times in the v1 spec;
`std::process::exit(130)` and `return 130;` both live in
`crates/muxsmith-cli/src/commands/run.rs`; `grep -rn 'ctrlc' crates/ src-tauri/src/`
puts the crate and the handler in `muxsmith-cli`/`run.rs` only; D16 in
`docs/superpowers/specs/2026-07-09-plan-4-design-decisions.md` does say "the
process exits 130", so the `(D16)` citation lands correctly.

**Why it matters here specifically.** This is a verbatim, must-not-decide string
entering the authoritative document in the one task whose purpose is that the
authoritative document stop making inaccurate claims, and the implementer is
explicitly forbidden from adjusting it. The doctrine's milestone rule that claim
language states exact coverage applies to the spec at least as strongly as to a
release note.

**What resolves it.** Either scope the clause ("on POSIX shells the shell reports
130 by its own 128-plus-signal convention; Windows reports its own
control-C status") or cut the explanatory clause and keep the load-bearing half:
"Only `run` returns 130; no other subcommand installs a SIGINT handler."

### 7. Important - Task A2 Step 1's fall-through leaves one fork open

**Location:** Task A2 Step 1, closing sentence.

**What is wrong.** The sentence routes two cases to NEEDS_CONTEXT: a hit in a file
the Files list does not name, and a hit outside a comment. A third case falls
through: an additional hit *inside* one of the two named files, *inside* a comment,
that this plan does not fence. Combined with the preceding clause - "that set is
the ground truth and the report says so" - an implementer can read that as
licence to compose a replacement at the keyboard. Task A1 Step 1 closes exactly
this case, in one clause: "A hit this plan does not fence returns as
NEEDS_CONTEXT rather than being rewritten at the keyboard." The asymmetry between
the two tasks is what makes this a defect rather than a reading.

**Probability is low, and that is not the test.** My own scan confirms the corpus
is exactly the two fenced sites (see the reproduction section), so the case is
unlikely to arise. The latitude test asks what the document permits, not what the
tree currently contains.

**What resolves it.** Add A1's clause to A2 Step 1 verbatim.

### 8. Minor - the retained-set exhaustiveness claim under-counts by two

**Location:** Task A3's "**Nine assertions in seven files are deliberately NOT
edited**, and the list is exhaustive"; the corrections table's row 2 framing of the
assertion set as 15 lines.

**Evidence.** A broad, alternation-free scan (every case-insensitive `byte` in the
live surface, then hand-classified) finds two further live assertions of the same
fact that neither of the plan's two expressions can see:

- `src/editor/widgets/PropertyMapWidget.vue:130` - "only for the byte-exact keys
  `type`/`codec_kind` (a `raw:type` key fails this and keeps its free-text cell,
  preserving the `raw:` bypass)"
- `e2e/editor-dropdowns.spec.ts:80` - the test name "case 4: a raw:type key keeps
  its free-text cell (byte equality; raw: bypass preserved)"

Both are scoped to string-typed properties (`type` is
`("type", PropType::String)` in `generated.rs`; `codec_kind` is String via
`matchable_type`), so both are TRUE and neither needs repair. The repair set of six
is complete - I verified that no *false* unscoped claim survives outside it.

**Why it is worth fixing anyway.** The plan declares the not-edited list
exhaustive so the invariance check has a fixed target, and the brief's house rule
is that the scope unit for a repeated fact is the set of assertions. Two members of
that set sit outside both instruments, in the vocabulary ("byte-exact keys", "byte
equality") the alternations do not carry. A later sweep for "byte-exact" will find
a site this plan declared handled.

**What resolves it.** Either widen the retention expression to catch both and
restate the count, or keep the expression and re-word the claim to "nine
assertions matched by the retention expression", naming the two further sites in
the surfacing list as out-of-instrument but true.

### 9. Minor - "eight hunks" does not reproduce

**Location:** authoring section, the `postcss` bullet ("the lockfile diff is
**eight hunks** covering exactly two packages"); B1 Step 2; acceptance row W1-c's
evidence column.

**Evidence, from my own scratch reproduction** (`package.json` + `pnpm-lock.yaml`
copied to my own directory, `pnpm update postcss --ignore-scripts` under pnpm
11.10.0 / Node 26.5.0): postcss moves `8.5.16` -> **8.5.24** at all four lockfile
sites, `package.json` byte-identical, and the diff is **6 hunks** at git's default
`-U3` (9 insertions, 9 deletions), **7** at `-U0`, 6 at `-U1` and `-U5`. I could
not produce 8 at any context setting.

The load-bearing half reproduces exactly: the diff touches precisely two packages,
`postcss` and `nanoid` (`3.3.15` -> `3.3.16`, postcss's own dependency), and
nothing else. W1-c's acceptance criterion is the package set, not the hunk count,
so no check depends on the wrong figure.

**What resolves it.** State 6 (naming `git diff`'s default context) or drop the
hunk count and keep the package set, which is what the row actually asserts.

### 10. Minor - "every gtk-rs family crate in the lock" is an unenumerated set

**Location:** B1 Step 8's evidence list.

The step requires "the version of every gtk-rs family crate in the lock" without
defining membership, and the plan's own two lists disagree: the authoring section
tallies seven (`atk`, `cairo-rs`, `gdk`, `gdk-pixbuf`, `gio`, `gtk`, `pango`)
while the direct-parent list has eleven, adding `gdkx11`, `javascriptcore-rs`,
`soup3`, `webkit2gtk`. An implementer reproducing "the gtk-rs version tally"
cannot know which list is meant. Latitude by omission, in an evidence step rather
than a product change, which is why it is Minor.

**What resolves it.** Name the crates, or define the tally as "every crate in the
`--depth 1` parent set plus `glib` itself", which is derivable from the command
already prescribed.

### 11. Minor - B1 Step 6's fired control does not name its id

**Location:** B1 Step 6, item 4: "plus one `grep -c` for an id that IS present,
which must return non-zero".

The implementer must choose a member of a set it may read but which the plan does
not name. Any of `deny.toml`'s 18 ids works, so nothing can go wrong - but the
plan's own standard is that a set in a normative position is enumerated. Name one;
`RUSTSEC-2024-0415` returns 3 in my `-L info` run.

### 12. Minor - Task A4's exit-code sweep cannot see the spec's own cancellation sentence

**Location:** Task A4 Step 4, first bullet: `grep -nE 'xit cod|SIGINT|Ctrl'` over
the spec, with "The authoring run returned two lines" and "**A third hit is a
finding.**"

The three-term set reproduces the two hits (section 6's mkvmerge-job exit codes at
`:318`, correctly classified as a different subject, and the 8.1 bullet at `:369`).
But a broader sweep over the same file surfaces `:319`, "Cancellation: kill the
mkvmerge process, delete the partial output file." - a cancellation statement, in
the same section, on the subject the amendment is about (130 = cancelled batch),
which none of the three terms matches. It is *consistent* with the new bullet, so
no contradiction results; the instrument simply cannot see it, and the "third hit
is a finding" framing primes the implementer to treat any extra hit as a defect
rather than to classify it.

**What resolves it.** Add `cancel` (and ideally `signal`) to the alternation and
pre-classify `:319` in the plan, so the third hit is expected and dispositioned.

### 13. Minor - Task A2's test-duty premise is false as stated

**Location:** Task A2 Step 4, "Test duty, weighed".

The premise is that "the fixture's data ... is asserted unchanged by the existing
`cargo test --workspace` run above". Running the premise rather than weighing it:
`crates/muxsmith-core/tests/profile_save.rs` consumes the fixture in
`all_non_default_fields_survive_the_round_trip`, which parses it, serializes it,
re-parses it, and asserts the two models equal. That asserts round-trip
self-consistency, not invariance against the pre-state: a data change that still
round-trips passes. The conclusion (no new test) is right, but the thing that
actually proves the data did not move is W2-e's comment-only `git diff -U0`.

**What resolves it.** Attribute the guarantee to W2-e and describe the test run as
proving the fixture still parses and round-trips.

### 14. Minor - the repair expression's prose says four house YAML files, the pathspec excludes three

**Location:** authoring section, item-3 repair-set bullet ("excluding ... the four
house YAML files"); the fenced expression in A3 Step 6.

The fenced pathspec excludes `decision-ledger.yaml`, `conventions.yaml` and
`process-conventions.yaml`; `product-boundaries.yaml` is not among them. Harmless
today - I verified that file carries no `byte` occurrence at all - but the prose
and the instrument disagree, and the instrument is what the implementer runs.

### 15. Minor - item 2's briefed surfacing duty has no acceptance row

**Location:** acceptance map, W2 rows.

The brief makes the stale "OPEN OWNER QUESTION" paragraph an explicit requirement
of item 2 ("surface it and I will fix it"). Task A2 Step 5 discharges it and the
plan close consumes it, so nothing is lost - but the structurally identical duty
in Task A1 got its own acceptance row (W5-e, "the Tier-2 statement that cites gate
part 6 is surfaced, not silently orphaned") and this one did not. Since the
acceptance map is the artifact a reviewer walks, add the row.

### 16. Minor - a quotation carries backticks its source does not

**Location:** authoring section's Tier-2 bullet, A1 Step 4, and the plan close's
surfacing item 1 all quote the clause as "documented as gate part 6 in
`BUILDING.md`". The YAML statement reads `documented as gate part 6 in BUILDING.md`,
without backticks. Trivial, but it is a quotation inside a claim about a file the
plan may not edit, and the controller will copy it into the repair.

---

## The six refutation reproductions

Author's figure on the left of the verdict, mine on the right.

### Refutation 1 - work item 2 is two sites, not one: **REPRODUCES**

I derived the corpus without reusing either of the plan's expressions. My
extension set came from `pathlib.Path(f).suffix` over `git ls-files` (27
extensions, plus `.gitattributes`/`.gitignore`/`.npmrc`/`LICENSE` as
suffix-less - the plan's `sed` derivation yields the same set plus those three as
pseudo-extensions, i.e. the plan's set is the strictly wider one and cannot miss a
`.gitignore:5`-shaped citation that mine would). My scanner then used no
enumerated extension list at all: any `<name>.<ext>:<digits>`, any bare `:<digits>`
with no restriction on the preceding character, plus prose locators
(`line/lines/Zeile/Zeilen NNN`, `L<NNN>`, `#L<NNN>`).

| claim | author | measured |
|---|---|---|
| filename-plus-line citations outside `docs/` | 1 (`.github/workflows/ci.yml`) | **1** - `.github/workflows/ci.yml:90`; my broader pattern adds only 3 false positives (`127.0.0.1:4173` URLs in `playwright.config.ts`), which the plan's enumerated extension set correctly filters out |
| bare `:<line>` spans outside `docs/` | 1 (`crates/muxsmith-core/tests/fixtures/all-non-default.yaml`) | **1** - `all-non-default.yaml:2`; my unrestricted `:\d+` returns 58 lines, of which 57 are timestamps, ports, `--track-order` strings, JSON, format specifiers and `1:1` ratios |
| prose-form locators | 2, both test data in `e2e/smoke.spec.ts` | **2**, exactly those, both `"mkvmerge output line 1"` |
| the `ci.yml` citation is stale | yes | **yes** - `queue.rs` resolves to one tracked file; line 73 at HEAD is `pub struct QueueOpts {` |

Blind spots I checked because both the plan's expressions and mine skip them:
`.snap`, `.lock` and the four suffix-less tracked files carry no citation of either
form. Under `docs/`, the only citations live in `ROADMAP.md` (a tracker recording
measurements, explicitly exempt under the Tier-2 scope boundary) and
`docs/IDEAS.md:200` (citing mkvtoolnix's `settings.cpp:625-626`); `docs/INSTALL.md`
carries none. So the plan's "every tracked file outside `docs/`" surface is
materially complete, and its class-closure claim correctly carries that qualifier.

One sub-figure inside this refutation is wrong - the `004e1e8^` line numbering.
See finding 5.

### Refutation 2 - 15 assertion lines, split 6 repair / 9 retain: **REPRODUCES, both halves, member for member**

Derived independently: every case-insensitive `byte` occurrence in the live
surface, filtered for byte-array/encoding noise, then classified by hand rather
than by either alternation.

| claim | author | measured |
|---|---|---|
| repair set | 6 lines / 5 files | **6 / 5** - `README.md:60`, `matcher.rs:96`, spec `:176` and `:421`, `help/de/...:23`, `help/en/...:23` |
| retained set | 9 lines / 7 files | **9 / 7** - `matcher.rs:452` and `:466`, `validate.rs:408`, `report/mod.rs:87`, `validate_semantics.rs:249`, spec `:280` and `:421`, `locales/de/diagnostics.ftl:21`, `locales/en/diagnostics.ftl:14` |
| spec `:421` in both sets | yes | **yes**, and A3 Step 2(b) correctly moves only the first occurrence |
| `scalar_eq` arms | 6 arms, 2 coerce | **6 typed arms plus `_ => false`; the two cross arms are `(Int, Float)` and `(Float, Int)`** |
| the retained lines are genuinely TRUE | yes | **yes** - but on different grounds than the plan cites; see finding 3 |

The second half - whether every retained line is true rather than conveniently
exempted - is the one the brief flagged, so I want to be explicit about where it
lands. The lines are true. `raw:language` puts a `Str` scalar against a reported
`Str` (mkvmerge schema v20 declares `language: string`), and `scalar_eq`'s
`_ => false` arm rejects a numeric scalar, so nothing matchable is coerced.
`raw:codec_kind` cannot match at all, since mkvmerge reports no such property.
What is wrong is the plan's stated justification, which routes through the
capability model that the `raw:` path bypasses, and cites a file that does not
contain `codec_kind`. The exhaustiveness claim also under-counts by two (finding
8).

Supporting reproductions: `raw_opt_in_diagnostic`'s trigger is
`matches!(bare, "language" | "codec_kind")` at `validate.rs:415`;
`b7_raw_int_float_cross_compare` and `b8_raw_language_is_byte_literal_no_normalization`
both pass in my own target dir, and the b7 body is exactly the Int-against-Float
assertion the new wording describes, so A3's "no new test needed" premise is run
and holds; the plan-5.5 case table's B-7 row is quoted byte-accurately; mkvmerge
schema v20 has 59 track properties with exactly the five named `type: number`
fields and no `codec_kind`.

### Refutation 3 - four of five synopsis lines plus an exit-code bullet: **REPRODUCES exactly**

Derived from a binary I built myself (`cargo build -p muxsmith-cli --target-dir <my path>`),
then confirmed byte-identical to the repo's `target/debug/muxsmith` by `sha256`, so
the surface below comes from the shipped binary and not from source or from the
plan.

| subcommand | spec 8.1 | binary `--help` | delta | verdict |
|---|---|---|---|---|
| `validate` | no flags | `--json`, `--locale` | both omitted | reproduces |
| `dry-run` | `--source --output --json` | `--source --output --on-collision --json --locale` | `--on-collision`, `--locale` | reproduces |
| `run` | `--source --output --jobs --fail-fast --json` | `--source --output --on-collision --jobs --fail-fast --json --locale` | `--on-collision`, `--locale` | reproduces |
| `identify` | `--json` | `--json`, `--locale` | `--locale` | reproduces |
| `schema` | no flags | none beyond `-h/--help` | none | reproduces ("correct") |

The plan's replacement block also preserves the binary's own option order per line
(I checked each), keeps `schema` byte-identical, and its longest line is 78
characters. The exit-code half reproduces too: `grep -c '130'` over the spec is
**0**; the only spec exit-code statement is the 8.1 bullet plus section 6's
mkvmerge-job codes; `130` is produced only in `commands/run.rs`; `ctrlc` appears
only there; `cli.rs:12-13` carries the over-broad "every command shares the
exit-code contract ... (spec 8.1, D16)" hard-wrapped across two `///` lines, as the
plan says. The `(D16)` citation is correct - D16 states "the process exits 130".
The new bullet's explanatory clause is where this task goes wrong; see finding 6.

### Refutation 4 - work item 1(b)'s hypothesis is refined: **DOES NOT REPRODUCE**

The *observation* reproduces perfectly. The *refinement built on it* is refuted.

| claim | author | measured |
|---|---|---|
| RustSec carries the advisory | `RUSTSEC-2024-0429`, `informational = "unsound"`, `aliases = ["GHSA-wrw7-89jp-8q8g"]`, `patched = [">=0.20.0"]`, five `VariantStrIter` functions over `>=0.15.0,<0.20.0` | **all confirmed verbatim** from `~/.cargo/advisory-dbs/advisory-db-3157b0e258782691/crates/glib/RUSTSEC-2024-0429.md` |
| the alias equals the GHSA GitHub reports | yes | **yes**, byte-identical; the two mechanisms see one advisory and this is not a database gap |
| `RUSTSEC` mentions at `-L info` | 54 | **54** |
| `RUSTSEC-2024-0429` mentions at `-L info` | 0 | **0** (control: `RUSTSEC-2024-0415` returns 3) |
| `advisory-ignored` notes | 18, none of them glib | **18**, none a glib id (the one `glib` string in `deny.toml` is a comment about `glib-macros`) |
| `deny.toml` RUSTSEC ignores | 18 | **18** distinct ids |
| gate result | `advisories ok`, `0 errors, 0 warnings, 36 notes` | **identical** |
| **"this cargo-deny version does not evaluate that class here at all"** | asserted | **REFUTED.** The `unsound` key exists at 0.19.9 with scope values `all\|workspace\|transitive\|none` and default `workspace` (cargo-deny's own `src/advisories/cfg.rs`); glib is transitive, so the default scope excludes it. `unsound = "all"` and `unsound = "transitive"` both yield exit 1, `advisories FAILED`, `unsound advisory detected`, `ID: RUSTSEC-2024-0429` |
| **"'make the check fail on that class' may not be an available configuration at 0.19.9"** | hedged | **REFUTED.** It is one key |

The hypothesis the plan declares superseded - "this configuration may not fail on
that class" - is the correct one. Finding 1 carries the disposition.

B1's Steps 4-6 are not damaged by this: they prescribe the measurement at the
tool's source rather than pre-empting it, and Step 5's "a mechanism account that
would predict an `advisory-ignored` note, or a warning, does not fit this
observation" does not exclude the correct account, which predicts silence at the
default scope. What the steps lack is the counterfactual that turns the account
from argument into demonstration.

### Refutation 5 - the plan's own first pass violated the pattern-set rule and self-corrected: **REPRODUCES, and the class survives elsewhere**

The claim checks out. `byte[- ]?(exact|literal|identical|wise)` cannot match
`byte-for-byte value equality` (the `en` help topic) or `byte-genaue
Wertgleichheit` (the `de` one); the corrected four-member alternation finds all
six repair sites, which I reproduced. Recording the walked-into defect rather than
only citing the rule is the right call and I would keep it.

But the review brief asks whether any other expression in the plan has the same
shape, and three do:

1. **The control for that very alternation is dead** - finding 2, Blocking. The
   expression was corrected; the check that would catch the *next* incompleteness
   was not.
2. **The retention alternation misses two live members** in a vocabulary it does
   not carry - finding 8.
3. **Task A4's exit-code sweep** enumerates three terms and cannot see the spec's
   own cancellation sentence - finding 12.

Two more enumerations in the plan I checked and cleared: expression B's
preceding-character class ``[[:space:]`,(]`` is a memory-derived enumeration the
plan does not account for, but my unrestricted scan proves no member escapes it
today; and expression B's binary-extension exclusion list is safe, since those
file types carry no citation at all.

### Refutation 6 - item 5 reproduces exactly, plus one spelled ordinal: **REPRODUCES**

Measured with my own probe, whose ordinal patterns are generated from word lists
rather than typed as an alternation, and which scans digit-before, digit-after,
word-before, word-after and count forms in one pass.

| claim | author | measured |
|---|---|---|
| positional gate ordinals in `BUILDING.md` | 3, at `:102`, `:134`, `:135` | **3, exactly those** (`part 6`, `parts 1-4`, `part 6`) |
| non-fenced lines over 80 | 1, at `:138`, 86 characters | **1, `:138`, 86 characters** |
| over-80 lines in total | 9 | **9** (1 non-fenced + 8 fenced) |
| `:134`/`:135` are one hard-wrapped paragraph | yes | **yes**, and `:138` sits in the same paragraph, which is what forces the one-edit constraint |
| the Rust gate block's six commands, in order | fmt, clippy, test, doc, deny, cross-clippy | **exactly that**, so "parts 1-4" = fmt/clippy/test/doc and "part 6" = cross-target clippy |
| live spelled gate ordinal | 1, `.github/workflows/ci.yml:88` | **1, exactly that.** My sweep returns 20 hits over tracked non-`docs/` files, of which 19 are `third-party`, `first-party` and the unrelated "five parts" of a match expression |
| `ledger-lint` green baseline | `ledger-lint: 548 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold` | **identical string, exit 0** |

**The non-defect call on `ci.yml:88`: I concur, independently.** The comment is
prefixed `Plan 5.5 Task 12 (#18b):`, which frames it as provenance, and the
ROADMAP's MEASURED block (session 27) establishes the governing reasoning in its
own words: every current gate-count occurrence "is a record of what the gate was
at the time, and a lint comparing them against today's count would fire on all of
them and demand that history be falsified". The governing Tier-2 widening reaches
CI *line-number* citations, not gate ordinals, so no recorded rule requires this
one's repair. One honest qualification for the record: read as a live claim the
sentence is false today (rustdoc is the Rust block's fourth command, not the gate's
ninth part), so the call rests on the provenance frame doing its work, not on the
sentence being true. Surfacing it so a later sweep does not renumber it is the
right disposition, and it needs no deferral vehicle precisely because it is not a
defect.

---

## The brief's other specific checks

**Absence-shaped acceptance items.** Every absence-shaped row names its
expression, a pre-state fire with an exact expected non-zero count, and an
end-state zero: W2-a (red 1), W2-b (red 1), W3-f (red 6 across 5 files), W3-g (9
on both states, fired by a deliberate deletion that must show 8), W5-a (red 3),
W5-b (red 1 at 86 characters), W4-b (pre-state `grep -c '130'` = 0 as the red
state of an inverted absence), W4-c (a flag sweep whose fired control must return
the amended block itself). I verified each fire exercises its anchor rather than
merely producing output, by checking that all five replacement texts contain none
of the four repair alternatives and none of the six retention alternatives - so the
green state is reachable and the retention count cannot drift to 10 by accident.
One prescribed control does not exercise its anchor: A3's alternation control,
finding 2.

**The four `git diff --exit-code`-class rows.** The author reports finding and
repairing four rows of that kind that had a green state only. Counted: W1-b, W1-i,
W1-k and W5-d. All four now carry a fire, and each fire is sourced free from the
tree's own asymmetry - the `--exit-code` and blob comparisons run against
`pnpm-lock.yaml`, the one file that did move, beside the files that did not; W5-d's
diff runs while the neighbouring `ledger-lint` fire has the canonical sentence
mutated and again after the restore. This is the strongest single piece of design
in the plan and I would ledger the technique.

**The controller's arithmetic.** Recounted mechanically. The acceptance map has
**32** rows, split `W1`=11, `W2`=5, `W3`=8, `W4`=3, `W5`=5, letters contiguous
per work item, no duplicates - so 11+5+8+3+5 = 32 checks out, and both sites that
state a total (the self-review's coverage paragraph and its recomputed-counts
list) say 32. The repair from 34 is correct. Every other count in the
recomputed-counts list also checks out - 5 tasks, 5 work items, 2 streams, 6
correction rows, 7 deferred-by-decision rows, 7 surfaced items, 6/9 A3 sites
across 5/7 files, 2 A2 corpus members, 3 ordinals and 1 long line, 5 subcommands
and 5 synopsis lines, 3 B1 parts, 18 RUSTSEC ignores, 6 `scalar_eq` arms of which
2 coerce, 5 `type: number` properties - **except** "12 direct parents of `glib`",
which is 11 (finding 4).

**The two Tier-2 statements. Both claims CONFIRMED; neither file touched by me.**

1. `gate-includes-cross-target-lint-for-the-unrun-os` in `docs/process-conventions.yaml`
   contains, in its `statement`: `documented as gate part 6 in BUILDING.md`. Task
   A1 removes that ordinal from `BUILDING.md`, so the clause becomes false on A1's
   commit. The plan's surfacing is correct and the repair is the controller's. My
   own reading of the surrounding statement: the repair is a one-clause rewrite
   (name the command instead of its position), not a redesign, because the rest of
   the statement already names the command in full.
2. `a-document-never-cites-a-line-number-inside-itself` in the same file contains
   "the owner ruled the same day that comments-locate-by-symbol-never-by-line-number
   stays scoped to SOURCE comments and is NOT widened by this", while
   `comments-locate-by-symbol-never-by-line-number` in `docs/conventions.yaml`
   carries "WIDENED BY OWNER RULING 2026-07-29 (session 28): the rule reaches CI
   and CONFIGURATION comments too". The plan's characterisation is exactly right:
   the two rulings are compatible in substance - the widening came from a separate
   session-28 ruling, not from the self-citation ban - but the sentence misleads
   anyone who opens only the first entry. Its third occurrence records the
   widening ruling verbatim ("ja, gilt auch bis in CI- und Konfigurationsdateien"),
   so the correction is to the cross-reference sentence, not to the ruling.

**The postcss decision: correct, and not latitude wearing a justification.** The
latitude test is whether the implementer must invent something it is not allowed
to invent, and here it invents nothing. The version is produced by a named command
whose resolution is fixed by the parents' declared ranges and the registry at run
time; the plan states the acceptance predicate (`>= 8.5.18`, the alert's own
`first_patched_version`, which I confirmed at the source); and W1-c bounds the
blast radius to `postcss` plus `nanoid`, so an unexpected resolution cannot pass
silently. Fencing the version would be the defect, not the discipline: my own
probe landed on 8.5.24 while `npm view postcss dist-tags` reads `latest: 8.5.25`,
so a fenced number is a claim about a future registry state, and an implementer
who landed on 8.5.25 would owe a NEEDS_CONTEXT round-trip over nothing. I also
confirmed the fork the brief worried about does not exist:
`@vue/compiler-sfc@3.5.39` declares `^8.5.15` and `vite@8.1.4` declares `^8.5.16`,
both caret ranges over `8.5.x`, and 8.5.18 through 8.5.25 all exist. The parents'
ranges also cap resolution inside `8.5.x`, so the lower-bound-only predicate is
safe without an upper bound. Refusing to pre-authorise the `pnpm.overrides`
fallback and routing it as NEEDS_CONTEXT is the correct reading of the latitude
ban.

**The glib finding's shape: adequately measured for the question it answers.** The
question is whether glib can move independently of Tauri's tree, and eleven
parents all pinned at gtk-rs 0.18.x with nothing 0.20+ in the lock answers it
completely. The one claim in part (c) that is borrowed rather than measured - that
Tauri 2's tao/wry have not migrated off GTK3 - is correctly attributed to
`deny.toml`'s own comment rather than presented as a fresh measurement, so it is
honest. Whether a Tauri bump would move the family is the upgrade project's
question, and the plan routes it to its own vehicle. Only the count is wrong
(finding 4).

**A safeguard the plan proposes stays.** I recommend removing no guard, check,
enumeration or test. Every finding is a correction or an addition. I ran the
premises behind the plan's four "unnecessary" conclusions rather than weighing
them: A3's no-new-test premise **holds** (`b7_raw_int_float_cross_compare` exists,
asserts `Scalar::Int(6)` against `PropValue::Float(6.0)`, and passes in my own
build; the plan-5.5 B-7 row is quoted accurately); A4's no-permanent-checker
premise **holds** (the ROADMAP's "Reach-claim checker" section does say the
instrument was "**Deliberately NOT promoted** into `scripts/ledger-lint.py`" on
the reviewer's recommendation and the controller's agreement, because it parses
prose); A1's no-test premise **holds**; A2's no-test premise is supported by a
**false** sub-claim (finding 13) with a correct conclusion.

**Scope discipline: clean in both directions.** No owner QA round-3 work, no
undo/redo, no Plan-12 item and no derivation-package work appears as work here -
the only mentions of round 3 are the two correct scope disclaimers. No briefed
item is silently dropped: all five work items carry a named task, and the brief's
subsidiary requirements (the tracker-correction surfacing for item 2, the
same-edit constraint and verified-not-assumed `ledger-lint` run for item 5, the
spec-first wording order and SI-3 discharge for item 3, the binary-derived surface
and self-contradiction sweep for item 4, the three-part structure and
no-pre-authorised-fallback for item 1) are all present. No 1.0-completeness
sentence exists anywhere in the document.

**Other conformance checks that came back clean.** All twelve fenced OLD strings
exist exactly once in their target files and no NEW string is already present
(instrument fire-tested by corrupting one pair and watching the row report FAIL).
A1's replacement paragraph's longest line is 77 characters, A4's synopsis block's
is 78, A3's new `matcher.rs` comment's is 76, and no `rustfmt.toml` exists so
`wrap_comments` stays off. A1's edit region (lines 134-139) contains no marker,
fence or fenced line - the four markers sit at 74, 82, 111 and 121, the canonical
total sentence at 75. A1's replacement text is substantively true: `ci.yml`'s
`test` job runs `cargo fmt`, `cargo clippy`, `cargo test` and `cargo doc` on a
three-OS matrix, and `deny` and `ledger-lint` are separate jobs. No hardcoded
model literal appears anywhere; the SI-4 trailer is the derived-from-dispatch
form. Every commit step is pathspec-scoped. Typography is clean: zero em-dashes,
en-dashes, smart quotes, Unicode ellipses, non-breaking spaces or Unicode minus.
German orthography is intact in both fenced German strings (`heißt`, `wörtlich`),
with no `ae`/`oe`/`ue` transliteration anywhere, and the German replacement reuses
the topic's own vocabulary as claimed (`help/de/...:9` reads "Zahlen vergleichen
numerisch: `6` ist gleich `6.0`").

---

## Harvest for the controller

Surfaced only; I wrote nothing to any house-knowledge file.

1. **The written pattern-set rule is insufficient, and this is its third
   consecutive round of the same class.** `a-search-whose-terms-come-from-memory-produces-a-false-absence`
   tells an author to derive an expression's enumerations from the artifact. This
   plan followed it, recorded its own violation and correction, and still shipped
   three instances (findings 2, 8, 12). Per doctrine §4's convergence rule, the
   question to ask is not for more members but what the written rule is missing,
   as a single clause. My candidate: **a control's TARGET is itself a measurement -
   run the control against a target measured to contain a match, never against one
   believed to.** That clause alone catches finding 2, and the doctrine's existing
   fire-test rule does not imply it: firing against a *presumed* known-present case
   is precisely what produced a dead control here.
2. **A claim about where a value is DECLARED is verified by grepping that file,
   not by reasoning about which file owns the concept.** Finding 3: `codec_kind`'s
   String type was attributed to `generated.rs`, the file that owns the concept,
   while the value lives in a special case in `capability/mod.rs`. Narrow entry
   candidate; this is a different shape from the pattern-set defect, because no
   enumeration was involved.
3. **Free fired controls from the tree's own asymmetry** - fire an `--exit-code` or
   blob-comparison instrument on the one file that DID move, beside the files that
   did not; fire a diff-scope check while a neighbouring fire has the file mutated.
   The plan applies this four times at zero cost and states the reason (a clean
   `--exit-code` is byte-identical in output to one aimed at a misspelled path).
   Worth promoting as a pattern; it is the cheapest known answer to the
   absence-shaped-check problem.
4. **A mechanism claim about a third-party tool's configuration is settled at the
   tool's `Default` impl, not at its output.** Finding 1: silence in the output was
   read as "the tool does not evaluate this class" when it meant "the default scope
   excludes this crate". One `grep` in the vendored source settled it. This is the
   verify-at-source duty applied to a default value rather than to an API.
5. **Second occurrence of the prose-parsing-checker restraint.** A4 Step 5 cites the
   reach-claim checker's deliberate non-promotion as the recorded ground for
   building no spec-versus-`--help` checker. That is a second citation of the same
   restraint in a second package, which is ledger-relevant for whatever entry
   carries it.
6. **Over-restriction watch (per the standing brief duty): one boundary reads
   tighter than its purpose.** B1's "the task changes neither `deny.toml` nor the
   `cargo deny` invocation" is correct as a product constraint but can be read as
   forbidding a scratch-config probe through `-c`, which is the only way to
   demonstrate the mechanism the same task is required to establish. The fix is a
   clarifying clause, not a boundary change.
7. **A stray line citation outside Plan 11's charge, noted so it is not lost:**
   `docs/IDEAS.md:200` cites `settings.cpp:625-626` in mkvtoolnix's source. Under
   the Tier-2 scope boundary it sits in a `docs/` process artifact and is out of
   Task A2's surface, but it is not a citation at a named commit either, so it is
   neither clearly exempt nor clearly in scope. Controller's call; it is not this
   plan's business.
8. **Dominant pattern worth recording as house practice:** every product edit in
   this plan is a fenced OLD/NEW pair, and all twelve OLD strings exist exactly
   once in their target. A mechanical fence-existence check over a plan document is
   a few lines and catches the whole class of unexecutable replacement before an
   implementer is dispatched. It found nothing here, which is the point - it is
   cheap enough to be worth running regardless.
