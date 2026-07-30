# Plan 11 amendment, design half - independent review of D111

**Artifact:** `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md` (ADR D111).
**Reviewer:** independent, read-only. **Date:** 2026-07-30.
**Tree:** the design's stated tree `d5b42c6`, which advanced to `d5d5b98` mid-review
when the controller committed `roadmap: two controller claims measured` (+20 lines,
`docs/ROADMAP.md` only, the block mirroring this design's P3 and P4). Every figure
below was re-verified at `d5d5b98` and is unchanged: R' 8, K' 7, `byte` sweep 71, R's
soundness control 9 with its ROADMAP hit still at `:1913`. `docs/ROADMAP.md` is
excluded from the wording surface, so the commit could not move them, and the control
line did not shift because the new block sits after it.
**Instruments:** all built at
`/tmp/claude-1000/-home-senol-agents-peter/5841e4a5-0b2e-469a-80ac-87b46dc93b73/scratchpad/d111-review-independent/`
(`equiv/` the differential comparator harness, `probe/` my own muxed probes and
profiles, `fmt/` the rustfmt scratch, `sweep.py` my own subject-first sweep). None
at the author's `raw-design/` path. Nothing staged, committed or pushed; no
worktree; no session-relocation tool; no file in the repository written except this
verdict.

---

## Verdict: NEEDS_FIXES

The design is substantively correct and unusually well measured. The safety
property holds: I proved the rewritten `scalar_eq` identical to the original over
every variant pair with a firing control, and the `raw:` re-pointing is confined to
one call site. All four brief refutations reproduce, three of them exactly. Every
wording figure reproduces to the line, the comparator and the three tests are
rustfmt-clean at my own path, the parity finding reproduces in source and in seven
binary runs, and the test set survives the broken-mechanism question in every
direction I could break it. What blocks approval is not the decision but the
handoff: section 13, the section the plan half consumes, enumerates the override
set for Task A3's executable body and misses the task's own header plus roughly
nineteen plan-level assertions of the same facts, including an acceptance row that
states "The behaviour is unchanged" and an authoring bullet that grounds "no new
test" on the behaviour being settled. The design applies exactly this lesson to the
wording surface (section 4.1's rule instead of a file list, trigger T12) and not to
its own override list. Two further defects are worth the round: a measured exit code
that does not reproduce and has already been relayed into the owner-facing ROADMAP,
and one replacement text that writes the phrase the design itself declares false.

---

## Findings

### 1. MAJOR - Section 13's override list is incomplete, and the plan half consumes it

**Location:** section 13, all bullets.

**Defect.** Section 13 enumerates the Task A3 clauses the amendment voids: the
Files list, the nine-assertion retained list, Steps 2/3/4/5, four Step-7 clauses,
Must-not-decide, Step 1's addition, and an Untouched set. That covers the task's
executable body. It does not name the task's own header, and it does not name a
single assertion of the same facts elsewhere in the approved plan. A plan author
who folds in exactly what section 13 lists leaves the plan self-contradicting in
about twenty-one places, two of which are the plan's own acceptance and rationale
surfaces asserting the inverse of the amendment.

**Evidence.** Derived by grepping the plan for the facts the amendment changes
(the site counts, the file counts, the retained-set size, the no-new-test ground,
the behaviour-unchanged claim), not from a reading pass. Inside Task A3's body but
absent from section 13:

| plan line | assertion | contradicted by |
|---|---|---|
| 456 | task title: "six sentences learn to say so" | twelve repair sites plus a behaviour change |
| 458 | Read first: read the ROADMAP entry "including its recorded disposition that **the behaviour stays and only the wording changes**"; and `matcher.rs` "for `scalar_eq`'s six arms" | the ruling inverts that disposition; `scalar_eq` becomes two functions |

Outside the task body, all in the approved plan:

| plan line | assertion | contradicted by |
|---|---|---|
| 61 | model-tier row: "six sites across five files ... a nine-member retained set that must be proven unchanged" | 12 sites, six files, seven-member retained set |
| 116 | authoring heading "six sites to repair, nine to leave" | 12 / 7 |
| 119 | "The behaviour is therefore settled and covered; only the wording is wrong ... it is why Task A3 owes no new test" | behaviour changes; three tests ship |
| 122 | "The REPAIR set is exactly six lines across five files" | R' returns 8 lines across 6 files |
| 123 | "The RETAINED set is nine lines across seven files" | K' returns 7 lines across 6 files |
| 124 | "4.4 and 9.2 will state that strings compare byte-for-byte while numbers compare numerically" | the spec will state the opposite |
| 166 | corrections row 2: "The set of assertions is 15 lines, split 6 to repair and 9 to retain" | 12 / 7 |
| 180 | coverage map W3: "six repair sites across five files, with a nine-member retained set proven unchanged" | 12 / 7 |
| 200 | "A3 spans five files in two natural languages" | six files |
| 233 | W3-a: "the v1 spec's two unscoped `raw:` statements state the numeric behaviour" | four spec sites, and the inverse numeric behaviour |
| 234 | W3-b: the comment "naming the cross arms and case B-7" | the new comment names `scalar_eq_same_type` and states no cross-comparison |
| 238 | W3-f: "RED ... exactly 6 lines across 5 files. GREEN: 0" | 8 across 6 |
| 239 | W3-g: "exactly 9 lines across 7 files on BOTH the pre-state and the end state" | 7 across 6 |
| 240 | W3-h: "**The behaviour is unchanged**", with `b7_raw_int_float_cross_compare` named from the output | the behaviour changes and that test is renamed |
| 866 | plan-close ROADMAP disposition "recording the derived set as six repaired and nine retained" | 12 / 7 |
| 867 item 3 | surfacing item "the nine retained `byte-literal` assertions ... two of them user-visible Fluent strings" | seven; two of the nine move under R-6/R-7 |
| 878 | deferral row "The nine retained `byte-literal` assertions keep their wording" | seven keep it |
| 946 | coverage tally "**37** acceptance halves ... W3=10" | W3 gains the behaviour change and three tests with no row named |
| 948 | latitude paragraph "the six repair sites and nine retained sites of Task A3" | 12 / 7 |
| 954 | "Counts recomputed from their own enumerations" | the enumerations move |
| 986 | brief refutations "item 3's assertion set is fifteen lines split six and nine" | 12 / 7 |

W3-h and line 119 are the sharp ones: an acceptance observable and an authoring
ground that assert the inverse of what the amendment ships.

**What resolves it.** Section 13 gains (a) the two in-body items above, and (b) a
stated RULE for the plan-level surface plus the list it derives, so the plan author
and the plan's own reviewer can re-derive the same set rather than trust an
enumeration. The rule is greppable: every plan sentence stating the A3 site count,
the file count, the retained-set size, that no behaviour changes, or that no test
ships. This is the same move section 4.1 makes for the wording surface, for the
same reason T12 gives.

### 2. MEDIUM - The optional-rule exit code does not reproduce, and it has already been relayed to the owner

**Location:** section 6, second bullet (design line 983); section 14, P4 (line 1342).

**Defect.** Both say the optional-rule case produces "exit 0". Measured: **exit 1**.
`severity_exit` (`crates/muxsmith-cli/src/commands/mod.rs:25-31`) maps a
warning-severity worst to 1, and the design's own pasted M5 output carries two
warnings. Section 6 additionally says the optional run emits "only the info and the
skew warning, both of which fire whether or not the comparison succeeded" - the
design's own M5 paste (lines 223-229) shows a third diagnostic, `This plan resolves
to zero output tracks`, which fires precisely **because** the rule matched nothing.
So the probe the design used does not demonstrate the claim it carries it for, and
the conclusion the controller has already written into `docs/ROADMAP.md:1975-1979`
("an **OPTIONAL** rule produces no error at all, exit 0. **So under byte-exactness
an optional `raw:` rule that stops matching fails silently.**") is overstated in an
owner-facing record.

**Evidence.** My probes, own path:

| run | design | measured |
|---|---|---|
| single-rule optional, `raw:max_luminance: 7` vs reported `6.0`, `src2/lum.mkv` | "no error at all ... exit 0 ... only the info and the skew warning" | **exit 1**; info + skew warning + `This plan resolves to zero output tracks` warning; human render prints `rule 0 -> track -` |
| multi-rule (rule 0 `exact: { type: video }` matches, rule 1 optional `raw:` does not) - not run by the design | not measured | **exit 1**; no error; info + skew warning only; no zero-tracks warning; human render prints `rule 1 -> track -`; `suggestions: []` |

The substantive point survives: no error severity, no suggestion, no proposed
narrowing, and the skew warning fires regardless of whether the comparison
succeeded. But "exit 0" is false, and "fails silently" is false of both probes -
the human rendering always prints `rule N -> track -` and the exit is non-zero.
The multi-rule run is the one that supports the design's argument, and it is the
run the design did not make.

**What resolves it.** Correct both occurrences to exit 1 with the `severity_exit`
citation; replace the single-rule probe with the multi-rule one for the
no-discriminating-signal claim, or drop the word "silently" and state what is
actually absent (no error, no narrowing, no signal that distinguishes a `raw:`
non-match from any other `raw:` use). The controller then owes the ROADMAP the same
correction, since the wrong figure is already there.

### 3. MEDIUM - R-2's replacement writes the phrase section 4.2 declares false

**Location:** section 4.3, R-2's replacement text; against section 4.2's
discriminator and section 8's rationale.

**Defect.** Section 4.2 rules: "**'untyped' describing the EQUALITY moves.**
'untyped equality' says the equality itself ignores type. After this ADR the
equality *requires* the kinds to agree, so the phrase becomes false. This is a
correctness call, not a style call". Section 8 uses exactly that to justify R-6 and
R-7. R-2's replacement text then opens:

```
        // `raw:` opt-in (D32, spec 9.2; semantics D111): untyped value
        // equality against the property named verbatim, with NO type
        // conversion - ...
```

"untyped value equality" attaches "untyped" directly to the equality. By the
design's own correctness rule that is a false phrase, and it lands in the one
comment a future reader of the `raw:` arm reads first - the site the whole ADR
exists for. R-3 and R-4 are clean by contrast: they use "is matched untyped" (the
path sense) and then define the equality, which is what the discriminator permits.
R-10's fenced doc comment is clean too ("the declared untyped path").

The design's own instrument cannot see it: the greppable expression
`untyped equality|untypisierte[a-z]* (Wert)?[Gg]leichheit|equality[^.]{0,20}untyped`
matches neither "untyped value equality" nor its hard-wrapped form. This is the
negative-claim-about-your-own-document shape.

**Evidence.** Design lines 486-489 (R-2's replacement) against lines 431-436
(the discriminator) and 1036-1043 (section 8's reason).

**What resolves it.** Either drop "untyped" from modifying the equality in R-2 -
"`raw:` opt-in (D32, spec 9.2; semantics D111): matched untyped, with NO type
conversion - value equality against the property named verbatim, so ..." - or
restate the discriminator to permit "untyped" adjacent to "equality" where the same
sentence states "no type conversion", and re-ground R-6/R-7 on the narrower fact
that their phrase carries no such qualifier. The first is cheaper and keeps R-6/R-7's
ground intact. **Related, same boundary:** `matcher.rs:407` reads "`raw:` opt-in
matcher cases B-5..B-8 (untyped comparison)" and is classified in neither the repair
nor the retained set; R-11's replacement keeps "compares untyped by value". Both are
defensible under the path reading, but the design should say so, since it is the
same construction it is retiring two sites away.

### 4. MINOR - Section 3.3's absolute is falsified inside the same document

**Location:** section 3.3, final sentence.

**Defect.** "**No other line of code in the crate changes.**" Section 3.2 adds
`scalar_eq_same_type` and rewrites `scalar_eq`'s body and doc comment; section 5
adds three tests and R-11/R-12 change two test sites in the same file. An
implementer reading 3.3 literally would take the test changes as out of scope,
which is the inverse of section 13's own voiding of the plan's "none may be inside
the `tests` module".

**What resolves it.** Scope the sentence: no line of NON-TEST code outside the
comparator pair and the one re-pointed call site changes; the test-module changes
are R-11, R-12 and section 5's three tests.

### 5. MINOR - T5 cites the D32 B-7 row at the wrong line

**Location:** section 12, trigger T5.

**Defect.** T5 places the row at
`docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md:78`. Measured: the
B-7 row is at **`:74`**, which is what the design's own "Supersedes" header cites.
`:78` is a blank line inside the *plan-time skew warning* table's block. T5 is the
trigger the controller routes the `superseded by D111` link from, so the citation
points at the wrong block; the row is also named by content, so it is recoverable.

**Evidence.** `grep -n "B-7" docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md`
returns `74:| B-7 | ...`; lines 70-80 read header, separator, B-5, B-6, B-7, B-8,
blank, "Plan-time skew warning (`PINNED = 20`):", blank, table header, separator.

**What resolves it.** T5 cites `:74`, matching the header.

### 6. MINOR - Section 4.6's three prescribed expressions are not executable as written

**Location:** section 4.6, checks R', K' and the vocabulary sweep.

**Defect.** All three are fenced as `git grep -nE '<expr>' <surface>`. `<surface>`
is a placeholder; run verbatim in a shell it is a redirect from a nonexistent file.
The surface IS fully fenced in section 4.1 and 4.6 points at it, so this is not
latitude in the doctrinal sense - the set is enumerated. It is a re-runnability
defect the plan being amended already recorded against itself: plan line 972, "the
target is IN the expression, not in the prose beside it, because an expression whose
surface lives in a sentence is not executable as written, **and this one read stdin
when it was run verbatim**", and plan line 948 lists "expression B was quoted with
its file selector elided" among five latitude-by-omission occurrences it removed.
The amendment reintroduces the shape into the checks that replace that plan's own.

**What resolves it.** Fence each of the three as two complete invocations with the
section 4.1 pathspec inline, as section 4.1 already does.

### 7. MINOR - The prescribed post-change binary runs can be measured against the stale binary

**Location:** section 13, the Step-1 bullet; section 5's "Tests deliberately NOT
added" first bullet.

**Defect.** The amended Step 1 re-runs M1's four `muxsmith dry-run --json` probes
"with the expected outcomes inverted for the two cross cases". That is the only
end-to-end proof of the behaviour change, and it runs `target/debug/muxsmith`.
None of the named exit bars rebuilds that binary: `cargo test -p muxsmith-core`,
`cargo clippy --all-targets`, `cargo fmt --check`, `cargo doc` and
`pnpm check:i18n` all leave it untouched, so an implementer can run the four probes
against the pre-change binary and see the old behaviour. The design shows the right
discipline in M1 itself (`find crates src-tauri -name '*.rs' -newer
target/debug/muxsmith` returns nothing) and does not carry it into the prescription.

**What resolves it.** The Step-1 addition requires `cargo build` (or the
newer-than-every-`.rs` check) before the four post-change runs.

---

## The safety property: the typed path is provably unmoved

**Yes, proved, with a firing control.**

Instrument: `equiv/` at my own path. `Scalar` and `PropValue` replicated verbatim
from `crates/muxsmith-core/src/profile/match_expr.rs:20-31` and
`crates/muxsmith-core/src/identify.rs:19-28` at `d5b42c6` (both are exactly four
variants, `Bool`/`Int`/`Float`/`Str`, so the semantics table's sixteen pairs plus
absence is exhaustive against the real variant sets, not merely against the table's
axes). `scalar_eq_old` copied verbatim from `matcher.rs:202-212`; the new pair
copied verbatim from the design's section 3.2 fenced block. Differential run over a
value pool built to hit equal, unequal and pathological operands in every cell:
i64 extremes, values f64 cannot represent exactly (`9007199254740993`), `NaN`,
`+/-inf`, `-0.0`, `1e308`, `f64::EPSILON`, and strings that differ only by Unicode
normalization.

```
pairs compared: 2209
divergences: 0
cell visit counts (rows Scalar Bool/Int/Float/Str, cols PropValue Bool/Int/Float/Str):
  [4, 28, 44, 18]
  [28, 196, 308, 126]
  [44, 308, 484, 198]
  [18, 126, 198, 81]
cell TRUE counts (old); a zero here means the cell never returned true:
  [2, 0, 0, 0]
  [0, 16, 17, 0]
  [0, 17, 25, 0]
  [0, 0, 0, 9]
CONTROL (Float,Int) arm removed -> divergences: 17  (must be > 0)
```

All sixteen cells were visited, and every cell that can return true did return true
in the pool - the four diagonals plus exactly the two cross cells - so the pool
exercises the arms the equivalence turns on rather than only the false ones. The
control removes one cross arm from the new form and the harness reports 17
divergences, so an empty divergence count is a measurement and not a blind check.

Reasoning behind the empirical result, for the record: patterns over a tuple of two
enum references are disjoint by variant, so no arm's position affects which arm
fires and the "reordering effect" the brief asks about cannot exist. Where the
kinds agree but the values differ, `scalar_eq_same_type` returns false and the
second `match` falls through its two cross arms to `_ => false`, which is what the
original returned. Where the kinds differ, only a cross arm can fire. `||`
short-circuits, but both operands are pure and total (an `i64 as f64` cast is
defined for every input), so short-circuiting is unobservable. `NaN` behaves
identically in both forms: false in `scalar_eq_same_type`, then no cross arm
matches Float/Float, so `_ => false`.

**Reachability of the typed cross arms is real, not theoretical.** `raw:`-free
`exact: { max_luminance: 400 }` against a reported `Float(400.0)` and
`exact: { max_luminance: 400.0 }` against a reported `Int(400)` both reach `:138`
and both need a cross arm. `capability/generated.rs:47`, `:48`, `:56`, `:57`, `:58`
declare exactly the five `Float` properties the design names, and through the
binary `min_luminance: 400` and `min_luminance: 400.0` both print `Profile is
valid.` while `audio_channels: 1.0` prints `Value for "audio_channels" has type
float, expected integer.` - so T-1's anchor is a production-reachable case.

**A latent trap that does NOT materialise, checked because T-1 and T-2 route their
float literals through the YAML parser.** If `#[serde(untagged)]` resolved `400.0`
onto `Scalar::Int` (Int is declared before Float), T-1's second assertion and T-2's
two `6.0` assertions would silently become same-kind comparisons and would pass with
the cross arms removed. Measured with `yaml_serde = "0.10.4"`, the crate the tests
use:

```
YAML       6 -> Int(6)          YAML     400 -> Int(400)
YAML     6.0 -> Float(6.0)      YAML   400.0 -> Float(400.0)
YAML    6.00 -> Float(6.0)      YAML    .nan -> Float(NaN)
DOC exact: { max_luminance: 400.0 }  -> {"max_luminance": Float(400.0)}
DOC exact: { raw:new_gain: 6.0 }     -> {"raw:new_gain": Float(6.0)}
```

Both directions are genuinely exercised. Incidentally this confirms a profile
`.nan` is reachable through the parser, so section 3.1's IEEE note is about a
reachable input and is correctly stated rather than decorative.

**Second safety claim - `substring`/`regex` under `raw:` unaffected: CONFIRMED.**
`grep -n "scalar_eq" crates/muxsmith-core/src/matcher.rs` returns exactly the three
call sites plus the definition and one comment (`:103`, `:138`, `:140`, `:202`,
`:410`). `exact_matches` is called from exactly one place, `matcher.rs:53`, inside
the `exact` loop. The substring arm (`:58-64`) and the regex arm (`:66-78`) both
read their value through `item_str(strip_raw(prop), item)`, and `item_str`
(`:192-197`) yields a value only for `PropValue::Str`. Neither arm can reach a
scalar comparison, so re-pointing `:103` cannot touch them. The `raw:` arm returns
at `:102-105` before `match prop`, which is why `:140`'s Boolean
false-when-absent shortcut belongs to the typed path and why B-6 holds; and `:140`
sits in the `None` branch of the `_ =>` arm, as the design states.

---

## The four refutations, my figures beside the author's

### P1. `PropValue::from` is really `PropValue::from_json` - REPRODUCED

`crates/muxsmith-core/src/identify.rs:34`: `pub fn from_json(v: &Value) ->
Option<PropValue>`. `grep -rn "impl From<.*> for PropValue\|PropValue::from\b"
crates/ src-tauri/` returns nothing, so there is no `From` impl and no other
conversion entry point. The brief's name does not exist. The mechanism is as
described: `as_i64()` first, `as_f64()` fallback.

### P2. "There is no parity model" is refuted - REPRODUCED, source and binary

Source, read at `~/Downloads/mkvtoolnix`, my own reads:

- `parse_arg_tracks` at `src/merge/mkvmerge.cpp:603`. The design's fenced snippet is
  verbatim: `parse_number` into `tracks.add(tid)`, else `parse_language` into
  `tracks.add(language)`.
- `item_selector_c<T>` at `src/merge/item_selector.h:27`, with
  `std::unordered_map<int64_t, T> m_items` and
  `std::unordered_map<mtx::bcp47::language_c, T> m_language_items` as separate typed
  buckets; `selected()` tests the numeric item against the numeric map and the
  language against the language map, joined by `||`, with `best_language_match`
  normalizing the language side. They never cross.
- `Track::setDefaultsMuxThis` at `src/mkvtoolnix-gui/merge/track.cpp:198-223`:
  `m_enableMuxingTracksByTheseTypes.contains(m_type)` over an enum, and
  `m_enableMuxingTracksByTheseLanguages.contains(Q(language.get_iso639_alpha_3_code()))`
  after canonicalizing to alpha-3. Both as described.

Binary, `mkvmerge v100.0`, my own probe (`tone.wav` seed, `--language 0:ger`,
reporting `language: "ger"`, `language_ietf: "de"`, `audio_channels: 1`,
`identification_format_version: 20`). All seven runs reproduce, audio tracks in
output:

| `-a` | design | measured |
|---|---|---|
| `0` | 1 | **1** |
| `1` | 0 | **0** |
| `ger` | 1 | **1** |
| `eng` | 0 | **0** |
| `de` | 1 | **1** |
| `deu` | 1 | **1** |
| `de-DE` | 0 | **0** |

Escape-hatch absence also reproduces: 28 files, 117 `profile` + 1 `profiles`, zero
`preset` (I summed the per-file tally rather than trusting the aggregate); fired
control 16 files, and the GUI tree holds 152 `.h`, 144 `.cpp`, 42 `.ui`.

**My judgement on whether the analogy is stretched, since the brief asks rather than
assumes.** It is a genuine finding and correctly bounded, but it carries less than
its placement suggests, and the design's own summary is the honest version ("what
is genuinely absent is a profile/preset concept ... but there is one for the
comparison rule"). Two reservations. First, `parse_arg_tracks` discriminates by the
*literal's own form* at parse time - a numeral becomes a track ID, anything else
becomes a language - which is a lexical dispatch into two disjoint namespaces, not a
value comparison against a reported property that could have had another type. There
is no case where a language literal is tried against a track ID and fails; the
question never arises. Muxsmith's `raw:` comparison is the case where it does arise,
so the precedent is adjacent rather than the same shape. Second, SI-3's
interactive-versus-declarative distinction does cut both ways here, exactly as the
brief warns: the GUI's `setDefaultsMuxThis` is genuinely declarative and genuinely
same-type, but its property universe is two compile-time-typed fields, which is the
opposite of an escape hatch over an open property set. Net: the parity finding
corroborates the ruling weakly and contradicts it nowhere, and it correctly
discharges the SI-3 duty that the brief's "no parity model" would have let lapse.
It should not be quoted as the ruling's ground - the design does not do so, and
section 10's "supports the ruling in both directions" is the strongest form the
evidence carries.

### P3. mkvmerge does NOT emit the shortest round-tripping form - REPRODUCED, and slightly stronger

Raw JSON tokens, extracted with `grep -oE` from `mkvmerge -J` rather than through a
JSON parser, so the text form is what is measured:

| mux argument | design | measured |
|---|---|---|
| `--max-luminance 0:6.00` | `6.0` | **`6.0`** |
| `--min-luminance 0:400.500` | `400.5` | **`400.5`** |
| `--min-luminance 0:400` | `400.0` | **`400.0`** |
| `--max-luminance 0:6` | not run | **`6.0`** |

Both halves hold: mkvmerge canonicalizes the decimal text, so `6.00` never arrives
and the `6.0`/`6.00` conclusion stands; and it writes a double with a fractional
part regardless, so `400` comes back as `400.0`. My extra run (`6` in, `6.0` out)
strengthens the rule the design states.

**Does the test set cover what the measurement implies?** Yes. The habit makes
`(Scalar::Int, PropValue::Float)` reachable from an ordinary file, and I confirmed
it end to end through the shipped binary: `exact: { "raw:min_luminance": 400 }`
against my `lum_int.mkv` (reporting the token `400.0`) matches **today**, one
`plan.assignments` entry `rule_index 0, track_id 0, track_kind video`, with `401`
firing the negative control to `missing-track`. The other direction reproduces too:
`exact: { "raw:audio_channels": 1.0 }` against a reported integer `1` matches, with
`2.0` firing the control. So M1's two rows are both live, T-2 pins both, and the
matrix test T-3 covers the same ground independently of the parser. Coverage is
adequate to the measurement.

### P4. "The suggestion engine reports a proposed narrowing" is partly refuted - REPRODUCED, except the exit code

Reproduced through my own profiles and probes against the shipped binary
(`target/debug/muxsmith`, verified newer than every tracked `.rs` file):

| observation | design | measured |
|---|---|---|
| `suggestions` for a `raw:` non-match | `[]` | **`[]`** |
| `missing-track` params | `{}` | **`{}`** |
| known-property control (`exact: { type: video, language: eng }`), non-match | identical bare error, `suggestions: []` | **identical, `suggestions: []`, `params: {}`** |
| optional rule: error present? | no error | **no error** |
| optional rule: exit code | **exit 0** | **exit 1** (finding 2) |

The refutation itself stands and is worth the owner's correction: the suggestion
engine produces nothing for either case, so the visibility argument on which the
ruling was partly recommended does not hold as recorded. What does not survive is
the exit code and the "fails silently" gloss built on it. Finding 2 carries the
detail and the multi-rule probe the claim actually needs.

---

## The six decisions, graded

**1. The comparator and its semantics table.** Sound. Exhaustive against the real
variant sets (four by four, both enums verified to have exactly those variants), and
the design checks the right thing by enumerating pairs rather than describing axes.
The float row states both IEEE consequences, and I confirmed `.nan` parses to
`Float(NaN)` so the note is about a reachable input; nothing further needs stating.
The placement argument (one enumeration of the same-type pairs, both call sites
self-describing, `scalar_eq` expressed in terms of the new function) is the right
call, and A-3 and A-4 are rejected with real steelmen rather than caricatures. The
comparator's fenced code is rustfmt-clean: I copied section 3.2's block to my own
scratch path and `rustfmt --edition 2024` produced **zero diff**; same for the three
tests of section 5 (toolchain 1.96.1, no `rustfmt.toml`, workspace edition 2024).

**2. Wording, 12 repair / 7 retain / 2 different-claim.** Re-derived my own way and
the delta is **zero on the site set**.

My instrument is not the author's: a paragraph-and-comment-run flattener over the
live surface (261 files under section 4.1's rule), selecting blocks that mention any
`raw` form and any comparison-characterizing term, with the term set derived by
reading the nineteen classified sites rather than recalled. It yields 31133 blocks,
246 mentioning `raw`, **57 candidates**, against the design's 58 from a differently
cut block definition. I read all 57. No live site outside the design's set carries a
false claim about the `raw:` comparison. The four sites invisible to both plan
alternations (R-5 `spec:146`, R-10 `scalar_eq`'s doc, R-11 the B-5 comment, R-12 the
B-7 test) are real finds that a vocabulary sweep cannot reach, and the design's
reason for that is correct: they are falsified by the semantics change without
carrying the retired word.

Every prescribed figure reproduces exactly, run with the section 4.1 pathspec in two
invocations and summed:

| check | design | measured |
|---|---|---|
| R' pre-state | 8 lines / 6 files | **8 / 6** (5 + 3), member for member: `README.md:60`, `matcher.rs:96`, `report/mod.rs:87`, `help/de/...:23`, `help/en/...:23`, spec `:176`, `:280`, `:421` |
| R' soundness control, ROADMAP exclusion dropped | 9, ninth is `docs/ROADMAP.md:1913` | **9**, ninth is `docs/ROADMAP.md:1913` |
| K' pre-state | 7 lines / 6 files | **7 / 6**, matching section 4.4's table member for member |
| `byte` vocabulary sweep | 71 (67 + 4) | **71** (67 + 4); 37 files, 12 carrying classified sites, 25 pure noise |
| plan's own check R | 6 / 5 | **6 / 5** |
| plan's own check K | 9 / 7 | **9 / 7** |
| `untyped equality` expression | exactly 2 live lines | **2**: `report/mod.rs:87`, spec `:280` |

The blind-spot statement reproduces too, and it is the good kind: `matcher.rs:466`
("`// Byte-literal against ...`") is absent from the case-sensitive 71 and is caught
only by K's capitalised alternation member, while `matcher.rs:457` (the b8 test's
name, `byte_literal` in identifier form) is in the 71 and reachable by neither R'
nor K'. So the retained set is covered by K' and the sweep together and by neither
alone, exactly as section 4.6 states. I verified `:457` is the `fn` line and `:466`
the capitalised comment.

**The retain half, checked line by line for truth rather than for exemption** - this
is where the brief asked for scrutiny, and all seven pass. Each is scoped to
`language` or `codec_kind`; schema v20 types `language` and `language_ietf` as
`string` and omits `codec_kind` entirely (my own reading of the schema, below), so
for `language` the comparison is `Str` against `Str` and byte-wise, and for
`codec_kind` it is a comparison that can never occur. Under the new semantics a
non-`Str` profile scalar against a reported `Str` returns false exactly as before,
so none of the seven changes truth value:

| site | wording | verdict |
|---|---|---|
| `matcher.rs:452` | "it byte-literally compares against the `language` property alone" | TRUE; subject is single-field lookup and no normalization |
| `matcher.rs:466` | "Byte-literal against the `language` field itself still works." | TRUE |
| `profile/validate.rs:408` | "which `raw:` degrades to byte-literal equality" | TRUE, scoped in code at `:415` to those two names |
| `tests/validate_semantics.rs:249` | "degrading the match to byte-literal equality" | TRUE, same scope |
| `locales/en/diagnostics.ftl:14` | "matches byte-literally instead" | TRUE; "instead" contrasts with normalization |
| `locales/de/diagnostics.ftl:21` | "gleicht stattdessen byte-literal ab" | TRUE, same |
| spec `:421` second occurrence | "that `raw:` degrades to byte-literal equality" | TRUE, scoped in the same clause |

Keeping both Fluent values is the right call for a second reason the design gives
and I agree with: they are user-visible product text in two locales, and A-6's
steelman is answered rather than dismissed.

The 100-to-71 reconciliation also holds arithmetically (29 = plan-2 1 + 5.6 1 +
5.7 4 + plan-6 9 + 6-apply-seam 4 + plan75 1 + plan8 9), and section 4.1's rule is
the right instrument for the reason T12 names - the design document itself would
otherwise be swept as a live site while quoting the retired phrases.

**3. The tests, graded as designs.** All three exercise their anchor, and each fails
under the break I could construct for it:

| break | T-1 | T-2 | T-3 |
|---|---|---|---|
| change not applied (`:103` still calls `scalar_eq`) | passes | **fails** | **fails** |
| cross arms stripped from `scalar_eq` | **fails** | passes | passes |
| wrong call site re-pointed (`:138` instead of `:103`) | **fails** | **fails** | **fails** |
| `Bool` arm omitted from `scalar_eq_same_type` | passes | passes | **fails** |
| cross arms present but swapped onto same-kind pairs | **fails** | **fails** | **fails** |

T-1 is a genuine safeguard rather than a formality: without it, stripping the cross
arms from `scalar_eq` passes the whole suite, and its anchor is production-reachable
(measured above through `scalar_fits` and the binary). Its negative control does what
it claims - it separates "the cross arms compare values" from "any number matches any
number" - though it does not additionally distinguish the arms, which is not what it
is for. T-2's same-kind counterparts are the right construction: without them the
test would pass against a comparator that stopped matching everything. T-3's
`i == j` matrix is the strongest of the three and correctly avoids the parser, and
the design's reason for that (a YAML round trip would make it a parser test too) is
sound - which I then had to check separately for T-1 and T-2, where the parser IS in
the loop; it holds.

The three "tests deliberately NOT added" each run their premise rather than weigh it,
and I reproduced both measurable ones. M4 reproduces with my own invocation: the same
hits, all of them the bare-`raw:` error case, a string key, or editor-dropdown
behaviour, with the control `grep -rln "exact" crates/muxsmith-core/tests/` returning
**18** files. The end-to-end premise is discharged by M1's four binary runs, which I
reproduced. No test duty is left open by the amendment: substring/regex under `raw:`
introduce no new behaviour (verified structurally above), so
`tests-ship-with-the-feature-never-after` is satisfied.

**4. `RawOnKnownProperty` unchanged, widening escalated - HONEST escalation.**
The brief's item 4 asks whether an *existing* diagnostic's trigger set widens. The
design decides that (it does not) and gives the doctrine's own routing ground: a
change to an owner-visible diagnostic surface goes to the governing human, and
leaving it alone needs no ruling. It then proposes a *different, new* config-time
never-match guard and routes it with a recommendation, explicitly refusing closure
by agreement ("a later round may not close it by agreement; it needs an owner
answer"). Checked against the proposed-safeguard rule as the brief frames it - was
this guard ever proposed as part of THIS package? No: neither the brief, the ROADMAP
entry nor Plan 11 proposes it; the design invents it and escalates it in the same
breath. Nothing is being argued out. The rule's actual target in this package is
T-1, and T-1 stays, correctly labelled as not-argued-out-during-design. The argument
for widening is also stated at strength rather than as a foil, including the two
measurements that strengthened it since the ruling and the house precedent
(`EmptyRawProperty` as an error on never-match grounds). I recommend no safeguard
removals here and none is proposed.

One thing the owner should see when he rules on T7, and it is the design's own point
sharpened by finding 2: this amendment ships a behaviour change whose single new
footgun has no config-time guard in this package, and the runtime signal is thin -
though less thin than the design states. For a required rule the failure is loud
(error `missing-track`, exit 2). For an optional rule there is no error, no
suggestion, and the only diagnostics are an info and a warning that fire for any
`raw:` use regardless of whether the comparison succeeded; the exit is 1, not 0, but
1 is also what a *successful* `raw:` match produces, so it discriminates nothing.
That is the accurate statement of the gap the guard would close.

**5. `UnknownPropertySkew` unchanged - premise RUN, not weighed.** The premise is
that "matched untyped" describes the path. I ran it rather than reading the design's
argument: `locales/en/diagnostics.ftl:41` reads "was matched untyped through a raw:
opt-in (bypassing the capability model). This build pins mkvmerge identification
schema version { $pinned }; this file reports version { $found_version }." The
message makes no claim about coercion, numeric comparison or type conversion, so the
ruling cannot falsify it. Correct decision.

The pre-existing defect is surfaced rather than ridden in, and I reproduced it: my
`raw:min_luminance: 401` run against a file reporting `400.0` emits
`unknown-property-skew` (warning, params `{property, found_version, pinned}`)
alongside `missing-track` (error) - so the warning does fire for a rule that matched
nothing. `planner.rs:630`'s comment is quoted accurately, and it already uses the
honest verb the message lacks: "the untyped match was **attempted** either way".
T8's routing is right, and "was compared untyped" is the accurate repair.

**6. The diagnostics-table row.** Repaired, with the narrowest correctness-grounded
cut and the counter-position (Plan 11's, which was right for the wording Plan 11 was
going to ship) recorded so the fold-in does not re-open it. R-6 and R-7 move
together, which is correct - the spec row and its `DiagCode` doc are one claim in two
places. I confirmed the `DiagCode` doc change has no serialized consequence:
`report/mod.rs`'s `diag_codes!` macro derives only
`Debug, Clone, Copy, PartialEq, Eq, Serialize` with `rename_all = "kebab-case"` and
no `TS` derive, so section 11's "the generated TS bindings and `profile.ts` are
untouched" holds for R-7 as well.

---

## Section 13 and section 12

**Section 13** is the blocking finding (finding 1). Checked against Task A3 itself
rather than against the list: the coverage of Steps 1 through 9, the Files list, the
nine-assertion retained list and Must-not-decide is accurate and complete, and the
Untouched bullet correctly preserves Step 6 (the README `pattern`, owner-ruled),
Step 8, and A3's position in the chain. What is missing is the task header and the
plan-level layer, enumerated in finding 1.

Two smaller notes on section 13, neither a finding. Its Step-9 bullet says the
commit shape is untouched "apart from its file list and message" without giving the
new message; that lands on the plan author, whose job is to fence it, so it is a
legitimate handoff rather than latitude. And "**A measurement that reveals a real
ordering or coercion defect rather than a wording defect remains NEEDS_CONTEXT**"
is correctly kept unchanged, which matters more than it looks: it is the clause that
routes a surprise back rather than letting an implementer decide.

**Section 12** is complete enough to route from, and its house-knowledge coverage is
complete rather than merely plausible - I derived the set from the four YAML files
instead of checking the design's three. `grep`ping all four for `statement:`/`steelman:`
lines mentioning `raw:`, `raw_`, `RawProperty`, `RawOnKnown`, `EmptyRaw` or
`UnknownPropertySkew` returns **10** lines. Exactly three carry a claim this
amendment touches: `core-91-raw-opt-in` (`decision-ledger.yaml:521`, T1),
`core-97-raw-on-known-property` (`:546`, T3) and `core-98-raw-language-single-field`
(`:558`, T2). The other seven are about severity, visibility, param drift or rejected
shapes and are untouched: `decision-ledger.yaml:31` (`core-08-runtime-skew-untyped`,
"matched untyped", path sense), `:534` (`core-95-rawproperty-visible`), `:1772`,
`:2969`, `conventions.yaml:412`, `:690`, and `product-boundaries.yaml:499`
(`EmptyRawProperty`). T4's target, `core-72-exact-typed-value-equality`
(`conventions.yaml:331`), is outside that expression and was found by the
comparison-vocabulary sweep instead; it stays true and gains a named guard test. So
T1 to T4 are the complete set and nothing in the trackers is missed.

T5's line citation is wrong (finding 5). T6 through T12 are correctly stated and
routable; T7's refusal to be closed by agreement and T12's rule-not-list instruction
are both the right shape. The D-number collision check reproduces: `D106`-`D110` are
reserved by `plan-12` (whose carrier `docs/superpowers/specs/2026-07-30-plan-12-decisions.md`
does not exist), the highest assigned id in `docs/` is `D105`, and `D111` appears
only where the controller has already written it into the ROADMAP - so `D111` is free.

**What the design should have surfaced and did not:** the plan-level consumers of
finding 1, either as a section 13 bullet or as a trigger. Nothing else. One piece of
routing information rather than a defect: the controller has already mirrored P3 and
P4 into `docs/ROADMAP.md:1959-1980`, including the wrong exit code, so T6 needs to
carry finding 2's correction rather than only the mirroring duty.

---

## Doctrine compliance

**ADR slots (section 1):** all present. Decision (sections 3 to 8), rationale,
rejected alternatives A-1 to A-8 each with a steelman stated at strength rather than
as a strawman (A-1's is genuinely the strongest form of the source-text variant, and
A-5/A-6 argue both directions of the retain question properly), triggers created and
surfaced for the controller (section 12), interface consequences (section 11), and
the `superseded by D111` link owed to D32 routed as T5 rather than performed as an
edit. The spec self-contradiction duty is addressed where it bites (section 4.4's
`:421` paragraph, section 8's table row) and the remaining sweep sits with Task A4
over the final text, which is where Plan 11 already put it.

**Latitude ban (section 3), both forms.** No permission clause, no "either approach
works", no "the implementer may choose". Every replacement is fenced character for
character in both natural languages; the semantics table enumerates all sixteen pairs
plus absence; the comparator's name and location are fixed; the test bodies are
fenced; section 3.4 enumerates what does not change. On the omission form: the one
unenumerated set I could find in a normative position is the "untyped"-retained
sites, and the design gives a rule plus a greppable expression for it, which is the
same standard section 4.1 sets - I would accept it, with the qualification in finding
3 that the rule as written is inconsistent with R-2's own text. Section 4.6's
`<surface>` placeholder is a re-runnability defect (finding 6) rather than latitude,
since 4.1 fences the surface completely.

**Proposed-safeguard rule.** Satisfied. T-1 is carried as mandatory and explicitly
not-argued-out; nothing proposed is dropped; the new guard of section 6 was never
part of this package and is routed rather than removed. I recommend no safeguard
removals.

**Run-the-premise-do-not-weigh-it.** Discharged in every place I checked: the three
not-added tests (M1's four runs, M4 with its fired control, section 3.4's
untouched-surface list), section 7's `UnknownPropertySkew` premise, section 6's
diagnostics measurement, and the `rustfmt` claim (checked by running the formatter
rather than by asserting the width). Section 4.6's fires are real - K's break-and-watch
targets a member measured present, and R's soundness control targets
`docs/ROADMAP.md:1913`, which I confirmed contains a match. Section 4.6's closing
paragraph, which states that R' goes blind after the repair and names the durable
check instead, is the best paragraph in the document.

---

## Harvest

- **A differential harness beats an argument for a pure-function rewrite, and it is
  cheap.** The equivalence of `scalar_eq` to `scalar_eq_same_type || cross` is
  provable on paper in four sentences, and the design proves it that way correctly.
  Building it anyway cost one small crate and produced two things the argument could
  not: a per-cell visit-and-true tally showing the value pool actually exercises the
  arms the claim turns on, and the parser probe that closed a latent hole in T-1 and
  T-2 nobody had asked about. Generalizes to any "same function, restructured" claim.
- **When a test routes a literal through a parser, the parser is part of the test's
  anchor.** T-3's design note says routing kinds through `#[serde(untagged)]` would
  make it a parser test - correct, and the same observation should have been turned on
  T-1 and T-2, which do route through it. Had `400.0` resolved onto `Int`, both would
  have passed with the cross arms removed. The trigger is readable: a test asserts a
  type-sensitive property using a literal it did not construct directly.
- **A figure already relayed upward is more expensive than one still in the
  artifact.** The exit-0 error had travelled from the design into the owner-facing
  ROADMAP before review, where it grew a conclusion ("fails silently") that the
  measurement never supported. The check that catches it is not a re-read: it is
  running the command and looking at `$?`.
- **A document that retires a phrase should grep its own replacement texts for it.**
  R-2 ships "untyped value equality" three sections after the design rules that
  construction false, and the design's own expression cannot match it. The general
  handle: after writing a replacement set that retires vocabulary, run the retirement
  expression - and a loosened form of it - over the *replacements*, not only over the
  pre-state.
- **`zsh` does not word-split unquoted parameter expansions.** My first pass at the
  R'/K'/sweep reproductions passed the pathspec as one `$SURF` string and returned 0
  for expressions that must return 5 and 67. A zero from a search is the shape that
  has to be earned; here it was a broken invocation, and it was only visible because
  I knew what the number should be. Pass `git` pathspecs as array elements.
- **Section 4.1's rule-not-list move is the reusable artifact here.** An enumeration
  of dated spec files went stale on its first addition - the design document itself -
  and the rule ("live means not history, not an append-only record, not a
  controller-owned tracker") is the durable form. It is also the move section 13 needs
  and does not make, which is worth recording as a pattern: a document that derives a
  rule for one surface should ask whether its other enumerations need the same
  treatment.

---

## Summary

**Verdict: NEEDS_FIXES.** 1 major, 2 medium, 4 minor.

**The typed `exact` path is provably unmoved** - 2209 pairs across all sixteen
variant cells, zero divergences, control firing at 17. `substring`/`regex` under
`raw:` confirmed unaffected by construction.

**All four refutations reproduce.** P1 exactly. P2 exactly, in source and in all
seven binary runs, with a reservation on how much the analogy carries (recorded
above; the design's own summary is the honest form). P3 exactly, plus one extra run
that strengthens it. P4 in every observation except the exit code, which is finding 2.

Fix findings 1 through 3 and I will approve; findings 4 through 7 are cheap and
should ride the same round. Nothing in the decision itself needs to change.
