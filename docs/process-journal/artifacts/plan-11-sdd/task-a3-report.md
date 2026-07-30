# Task A3 implementer report - Plan 11, stream A (W3)

**Status:** DONE_WITH_CONCERNS (three surfacings, none blocking; see "Concerns and
observations"). **Worktree:** `/home/senol/Git/muxsmith-plan11-a`, branch
`plan-11-stream-a`, base `5d305a2`. **Commit:** see the last section.

Everything below is pasted from the run that produced it. Where a long line is cut
for width, the cut is marked and the command that produced it is given verbatim
beside it.

---

## 0. Method note: the replacement texts were EXTRACTED, never transcribed

The brief and the plan both make D111 section 4.3 the single source of the twelve
replacement strings and forbid a copy. Rather than retyping them, every fenced
block was read out of `docs/superpowers/specs/2026-07-30-plan11-raw-bytewise-design.md`
programmatically and applied as-is, so a transcription error is not possible by
construction.

The extractor uses D111's own content anchor, not line numbers: **the first fenced
block under each `R-n` heading in section 4.3 is the OLD text, the second is the
NEW text.** (D111 records why: an anchor on the literal string `with exactly`
matches eleven places, one of which is the paragraph describing the anchor.)

```
blocks in 4.3: 20   R-n headings: R-1 .. R-12
R-1: 2 fenced block(s)     R-6: 2 fenced block(s)
R-2: 2 fenced block(s)     R-7: 2 fenced block(s)
R-3: 2 fenced block(s)     R-8: 2 fenced block(s)
R-4: 2 fenced block(s)     R-9: 2 fenced block(s)
R-5: 2 fenced block(s)     R-11: 2 fenced block(s)
pairs captured: ['R-1','R-2','R-3','R-4','R-5','R-6','R-7','R-8','R-9','R-11']
section 3.2 rust blocks: 1 | 3.3: 1 | section 5: 3
```

R-10's text is section 3.2's Rust block, R-12's is section 5's second Rust block,
T-1's the first and T-3's the third; the call site is section 3.3's one line. The
README's `pattern` line (Step 6) was extracted the same way from the plan document's
own fence inside Task A3 Step 6.

**Precondition for the reconstruct-from-base instrument, checked per site** (the A2
reviewer's caveat: it only works where the fenced OLD block occurs exactly once in
its file):

```
PRE-APPLY occurrence census (OLD text in its file):
  R-1   README.md                                                    occurrences=1
  R-2   crates/muxsmith-core/src/matcher.rs                          occurrences=1
  R-3   docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md      occurrences=1
  R-4   docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md      occurrences=1
  R-5   docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md      occurrences=1
  R-6   docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md      occurrences=1
  R-7   crates/muxsmith-core/src/report/mod.rs                       occurrences=1
  R-8   help/en/editor-match-expr-exact.md                           occurrences=1
  R-9   help/de/editor-match-expr-exact.md                           occurrences=1
  R-11  crates/muxsmith-core/src/matcher.rs                          occurrences=1
all OLD blocks unique in their file: True
```

Every application asserted `count(old) == 1` again at write time and would have
aborted otherwise. R-4's "FIRST occurrence only" is satisfied structurally: its
fenced OLD is a distinct string from the retained second occurrence in the same
line, and it occurs once.

---

## 1. Step 1 - pre-state, parser precondition, parity

### 1.1 Environment

```
$ mkvmerge --version
mkvmerge v100.0 ('Do Hot Girls Like Chords') 64-bit
$ cargo --version ; rustc --version
cargo 1.96.1 (356927216 2026-06-26)
rustc 1.96.1 (31fca3adb 2026-06-26)
```

### 1.2 SI-3 probes, muxed outside the repository from the repo's own seed

Scratch root: `/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/a3`.

```
$ cp <repo>/crates/muxsmith-core/tests/fixtures/seeds/tone.wav .
$ mkvmerge -o src2/aud.mkv tone.wav
$ ffmpeg -v error -f lavfi -i color=c=black:s=64x64:d=1:r=5 -c:v libx264 -pix_fmt yuv420p -y v.mkv
$ mkvmerge -o src2/lum.mkv --max-luminance 0:400.0 --min-luminance 0:1.5 v.mkv

$ mkvmerge -J src2/aud.mkv   # identification_format_version and the probe property
identification_format_version: 20
0 audio {'audio_channels': 1}
$ mkvmerge -J src2/lum.mkv
identification_format_version: 20
0 video {'max_luminance': 400.0, 'min_luminance': 1.5}

$ mkvmerge -J src2/aud.mkv | grep -o '"audio_channels": [0-9.]*'
"audio_channels": 1
$ mkvmerge -J src2/lum.mkv | grep -oE '"(max|min)_luminance": [0-9.]*'
"max_luminance": 400.0
"min_luminance": 1.5
```

This reproduces D111's M1 setup: an **integer** JSON token for `audio_channels` and
a **float** JSON token for `max_luminance` on an integral value, which is what makes
the `(Scalar::Int, PropValue::Float)` direction reachable from an ordinary file.

Six one-rule profiles, each `exact: { "raw:<prop>": <literal> }`, run through
`muxsmith dry-run --json`. The pre-state binary was rebuilt first
(`cargo build -p muxsmith-cli`, `Finished dev profile in 3.86s`).

**PRE-STATE:**

```
### RUN 1 CROSS (Float scalar 1.0 vs reported Int 1)      exit=1
  diagnostic codes: ['raw-property', 'unknown-property-skew']
  plan.assignments: [(0, 0, 'audio')]                       <- MATCHES
### RUN 2 CROSS (Int scalar 400 vs reported Float 400.0)  exit=1
  diagnostic codes: ['raw-property', 'unknown-property-skew']
  plan.assignments: [(0, 0, 'video')]                       <- MATCHES
### RUN 3 SAME (Int 1 vs Int 1)                           exit=1
  diagnostic codes: ['raw-property', 'unknown-property-skew']
  plan.assignments: [(0, 0, 'audio')]                       <- MATCHES
### RUN 4 SAME (Float 400.0 vs Float 400.0)               exit=1
  diagnostic codes: ['raw-property', 'unknown-property-skew']
  plan.assignments: [(0, 0, 'video')]                       <- MATCHES
### NEG CONTROL 1 (2.0)                                   exit=2
  diagnostic codes: ['raw-property', 'unknown-property-skew', 'missing-track']
  plan.assignments: []                                      <- FIRED
### NEG CONTROL 2 (401)                                   exit=2
  diagnostic codes: ['raw-property', 'unknown-property-skew', 'missing-track']
  plan.assignments: []                                      <- FIRED
```

All six reproduce D111's M1 exactly. No `NEEDS_CONTEXT` trigger.

**POST-STATE** is in section 6.1, after the mandatory rebuild.

### 1.3 The parser precondition of T-1 and T-2, five `validate` probes

```
$ muxsmith validate <one-rule profile with the literal below>
--- audio_channels: 1.0
[error] tracks[0].match.exact.audio_channels: Value for "audio_channels" has type float, expected integer.
--- audio_channels: 6.0
[error] tracks[0].match.exact.audio_channels: Value for "audio_channels" has type float, expected integer.
--- audio_channels: 400.0
[error] tracks[0].match.exact.audio_channels: Value for "audio_channels" has type float, expected integer.
--- track_name: 6
[error] tracks[0].match.exact.track_name: Value for "track_name" has type integer, expected string.
--- track_name: 400
[error] tracks[0].match.exact.track_name: Value for "track_name" has type integer, expected string.
```

Five for five, matching D111 section 5's table. So a decimal-point literal resolves
onto `Scalar::Float` and a bare integer onto `Scalar::Int` through
`#[serde(untagged)]`, and T-1's second assertion and T-2's two `6.0` assertions are
genuine cross-kind comparisons rather than same-kind ones that would pass with the
cross arms removed. T-1 is a safeguard, not a formality - and section 5.1 below
measures that directly rather than resting on this inference.

### 1.4 Parity (D111 section 10)

D111's section-10 runs are cited rather than re-derived per Step 1. The seven binary
runs were nevertheless re-run here at no cost, against a probe muxed with
`--language 0:ger` (reports `language: ger`, `language_ietf: de`,
`audio_channels: 1`, `identification_format_version: 20`):

```
mkvmerge -a 0      -> audio tracks in output: 1
mkvmerge -a 1      -> audio tracks in output: 0
mkvmerge -a ger    -> audio tracks in output: 1
mkvmerge -a eng    -> audio tracks in output: 0
mkvmerge -a de     -> audio tracks in output: 1
mkvmerge -a deu    -> audio tracks in output: 1
mkvmerge -a de-DE  -> audio tracks in output: 0
```

Member for member D111's table. The reference tool's declarative track selection is
same-type with no coercion between its numeric and language selector domains, and
domain-normalized inside the language domain; both halves of this design agree with
it. The mkvtoolnix source claims are D111's and were not re-derived.

---

## 2. Steps 2 to 7 - what was applied

| site | file | what moved |
|---|---|---|
| R-1 | `README.md` | the `raw:` bullet's phrase |
| R-2 | `crates/muxsmith-core/src/matcher.rs` | the `raw:` arm comment |
| R-3 | v1 spec `:176` | section 4.4's `raw:` opt-in bullet |
| R-4 | v1 spec `:421` | section 9.2's runtime paragraph, FIRST occurrence |
| R-5 | v1 spec `:146` | section 4.3's typed-equality sentence, one appended sentence |
| R-6 | v1 spec `:280` | section 7's `RawOnKnownProperty` row |
| R-7 | `crates/muxsmith-core/src/report/mod.rs` | the `RawOnKnownProperty` DiagCode doc |
| R-8 | `help/en/editor-match-expr-exact.md` | "The `raw:` bypass" |
| R-9 | `help/de/editor-match-expr-exact.md` | "Der `raw:`-Bypass" |
| R-10 | `matcher.rs` | the comparator pair (D111 section 3.2, verbatim) |
| R-11 | `matcher.rs` | the B-5 test comment, one identifier |
| R-12 / T-2 | `matcher.rs` | `b7_raw_int_float_cross_compare` replaced in name, comment and body |
| 3.3 | `matcher.rs` | the `raw:` arm's call site re-pointed |
| T-1 | `matcher.rs` | `typed_exact_still_cross_compares_int_and_float` added |
| T-3 | `matcher.rs` | `raw_compares_only_within_one_kind` added |
| Step 6 | `README.md` | the first example's `pattern` line |

**The cross arms stay.** `scalar_eq` keeps both, now expressed as
`scalar_eq_same_type(want, have) || match { <the two cross arms> }`; only the
`raw:` call site re-points. The applied diff of the comparator pair:

```rust
 fn scalar_eq_same_type(want: &Scalar, have: &PropValue) -> bool {
     match (want, have) {
         (Scalar::Str(a), PropValue::Str(b)) => a == b,
         (Scalar::Bool(a), PropValue::Bool(b)) => a == b,
         (Scalar::Int(a), PropValue::Int(b)) => a == b,
         (Scalar::Float(a), PropValue::Float(b)) => a == b,
         _ => false,
     }
 }

 fn scalar_eq(want: &Scalar, have: &PropValue) -> bool {
     scalar_eq_same_type(want, have)
         || match (want, have) {
             (Scalar::Int(a), PropValue::Float(b)) => (*a as f64) == *b,
             (Scalar::Float(a), PropValue::Int(b)) => *a == (*b as f64),
             _ => false,
         }
 }
```

**Test placement, the one thing D111 does not fence.** The three test bodies are
verbatim; where each lands in the `tests` module is not written down anywhere, so it
was decided as follows and is flagged here rather than left silent:

- **T-2** replaces `b7_raw_int_float_cross_compare` in place (mandated by R-12,
  which names `matcher.rs:444-449`).
- **T-1** sits as the last typed-path test, immediately before the
  `// D32 / Task 16: raw: opt-in matcher cases B-5..B-8` section comment - it is a
  typed-path test, and it sits directly against the contrast it guards.
- **T-3** closes the `raw:` group, immediately after `b8_...`.

No other placement was available without either splitting the B-5..B-8 group or
putting a typed-path test inside it.

---

## 3. Step 8 check R' - absence of the retired vocabulary

Both invocations exactly as D111 section 4.6 fences them, run from the repository
root, counts summed.

### 3.1 PRE-state: RED, 8 lines across 6 files

Invocation (a), full output (long lines are the file's own; nothing elided):

```
README.md:60:- **`raw:` is the deliberate opt-out.** ... byte-exact value equality against that one field, named verbatim. ...
crates/muxsmith-core/src/matcher.rs:96:        // `raw:` opt-in (D32, spec 9.2): untyped byte-literal value equality
crates/muxsmith-core/src/report/mod.rs:87:    /// A `raw:` prefix was applied ... degrades it to byte-literal untyped equality, bypassing those semantics ...
help/de/editor-match-expr-exact.md:23:... kein Fehlend-heisst-false - nur byte-genaue Wertgleichheit gegen die woertlich benannte Eigenschaft. ...
help/en/editor-match-expr-exact.md:23:... no absent-means-false shortcut - plain byte-for-byte value equality against the property named verbatim. ...
----- (a) count: 5
```

Invocation (b):

```
docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:176: ... (byte-literal value equality against the property named verbatim, ...
docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:280: | `RawOnKnownProperty` | warning | ... degrading it to byte-literal untyped equality (config-time; 4.4, 9.2) |
docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:421: ... and is matched untyped: byte-literal value equality against the property named verbatim, ...
----- (b) count: 3
```

**Sum 8, across README.md, matcher.rs, report/mod.rs, help/de, help/en and the v1
spec = 6 files.** Member for member the plan's W3-f row.

(The three help/de fragments above are rendered with ASCII substitutes for width in
this quotation only; the file's own German orthography is unchanged and is shown
verbatim in section 8.)

### 3.2 END state: GREEN, 0

```
(a) count: 0
(b) count: 0
```

### 3.3 Soundness control, pointed at a target measured to contain a match

Invocation (a) with `':!docs/ROADMAP.md'` dropped.

PRE-state:

```
README.md:60: ...
crates/muxsmith-core/src/matcher.rs:96: ...
crates/muxsmith-core/src/report/mod.rs:87: ...
docs/ROADMAP.md:1922:  `raw:` arm call the comparison an untyped byte-literal value equality, and the
help/de/editor-match-expr-exact.md:23: ...
help/en/editor-match-expr-exact.md:23: ...
control (a) count: 6      -> summed with (b)'s 3 = 9
```

END state:

```
docs/ROADMAP.md:1922:  `raw:` arm call the comparison an untyped byte-literal value equality, and the
control (a) count: 1      -> summed with (b)'s 0 = 1
```

**9 pre-state, 1 end-state, exactly as prescribed**, the survivor being the ROADMAP's
own sentence describing this defect. So R''s empty end-state result is a measurement
and not a mis-aimed pathspec.

**Measured deviation, not a defect:** D111 gives that ROADMAP hit as `:1913`; at this
tree it is `:1922`. D111 anticipates this in the same sentence and cites the line by
WORDING because a co-writer edits that file; the wording is byte-identical. The
controller's ROADMAP has grown 9 lines above that point since D111 was measured at
`e7c109f`.

---

## 4. Step 8 check K' - invariance of the retained set

### 4.1 PRE-state and END state, both 7 lines across 6 files

PRE-state, invocation (a), full output:

```
crates/muxsmith-core/src/matcher.rs:452:    // (language / language_ietf) lookup: it byte-literally compares against the
crates/muxsmith-core/src/matcher.rs:466:        // Byte-literal against the `language` field itself still works.
crates/muxsmith-core/src/profile/validate.rs:408:/// which `raw:` degrades to byte-literal equality; otherwise `RawProperty`
crates/muxsmith-core/tests/validate_semantics.rs:249:// (codec_kind), degrading the match to byte-literal equality.
locales/de/diagnostics.ftl:21:raw-on-known-property = ... und gleicht stattdessen byte-literal ab.
locales/en/diagnostics.ftl:14:raw-on-known-property = ... and matches byte-literally instead.
----- (a) count: 6
```

(b): `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:421`, count 1. **Sum 7.**

END state, invocation (a), full output:

```
crates/muxsmith-core/src/matcher.rs:533:    // (language / language_ietf) lookup: it byte-literally compares against the
crates/muxsmith-core/src/matcher.rs:547:        // Byte-literal against the `language` field itself still works.
crates/muxsmith-core/src/profile/validate.rs:408:/// which `raw:` degrades to byte-literal equality; otherwise `RawProperty`
crates/muxsmith-core/tests/validate_semantics.rs:249:// (codec_kind), degrading the match to byte-literal equality.
locales/de/diagnostics.ftl:21:raw-on-known-property = ... byte-literal ab.
locales/en/diagnostics.ftl:14:raw-on-known-property = ... matches byte-literally instead.
(a) count: 6
```

(b): the v1 spec `:421`, count 1. **Sum 7 on both states**, section 4.4's table
member for member. The two `matcher.rs` lines moved from `:452`/`:466` to
`:533`/`:547` because the file gained the comparator and three tests above them;
their text did not move, which the next check proves per site.

### 4.2 The INVERSE check, per site, because the files legitimately move

Each retained line's PRE-state text taken from the `HEAD` blob and searched for
verbatim in the post-state file:

```
OK   crates/muxsmith-core/src/matcher.rs:452           -> post line 533
OK   crates/muxsmith-core/src/matcher.rs:466           -> post line 547
OK   crates/muxsmith-core/src/profile/validate.rs:408  -> post line 408
OK   crates/muxsmith-core/tests/validate_semantics.rs:249 -> post line 249
OK   locales/de/diagnostics.ftl:21                     -> post line 21
OK   locales/en/diagnostics.ftl:14                     -> post line 14
MISS docs/superpowers/specs/...-v1-design.md:421       -> post line None
```

The `MISS` is expected and is the sharpest instance of the caveat the A2 reviewer
named: **spec `:421` is a single line that is in BOTH the repair set (R-4, first
occurrence) and the retained set (second occurrence)**, so byte identity of the LINE
is impossible by construction, exactly as file-level identity is impossible for
`matcher.rs` and the spec. The inverse check for that one site is therefore run at
CLAUSE level:

```
retained clause: 'that `raw:` degrades to byte-literal equality'
  occurrences in PRE  line 421: 1
  occurrences in POST line 421: 1

retained sentence (verbatim from PRE):
  The opt-in is announced at config time by `RawProperty` (info), or by
  `RawOnKnownProperty` (warning) when the bare name is a model property with special
  matching semantics (`language`, `codec_kind`) that `raw:` degrades to byte-literal
  equality.
byte-identical in POST: True
R-4 OLD still present in POST: False
```

### 4.3 K's fire, and the restore proof

Baseline captured from the CURRENT working-tree content (not from HEAD - the trap
that hit Task A1), restored with `command cp -f`:

```
$ sha256sum locales/en/diagnostics.ftl
afea4505541c39aceb18fd0d5139a34938e0b54614e5a369b5f032cc7d768306  locales/en/diagnostics.ftl

=== FIRE: delete the ' and matches byte-literally instead' clause ===
raw-on-known-property = Property "{ $property }" is a known property with special matching semantics; the raw: prefix bypasses them (language normalization, codec_kind aliasing).
K' during fire: (a)=5 (b)=1 sum=6      <- 7 became 6

=== RESTORE (command cp -f from the scratch baseline) ===
$ sha256sum -c ftl.sha256
locales/en/diagnostics.ftl: OK
K' after restore: (a)=6 (b)=1 sum=7    <- back to 7
$ git diff --exit-code -- locales/ ; echo $?
0
```

---

## 5. Step 8 - the alternation-free vocabulary sweep, every hit classified

Both invocations per D111 section 4.6, `-nE` not `-niE`, `Cargo.lock` excluded.

```
PRE-state: (a) 67 + (b) 4 = 71        END state: (a) 63 + (b) 3 = 66
```

**71 on the pre-state, as prescribed.** All 71 classified mechanically, line by
line; the classifier's four site sets are D111 section 4.6's enumeration and every
remaining line was read.

```
tally: {'REPAIR': 9, 'RETAINED': 6, 'DIFFERENT-CLAIM': 2, 'TRUE-ABOUT-REGEX': 2, 'NOISE': 52}
classified sites (non-noise): 19
noise lines: 52 spread over 25 files
noise BY KIND: byte-identity of documents/snapshots/rendered output 24;
               raw byte buffer 10; encoding / multi-byte UTF-8 slicing 9;
               byte size 4; other 5
```

Full classification (line text cut to 78 columns for width; the classification was
run over the untruncated lines):

```
[NOISE           ] .github/workflows/ci.yml:119: # This literal must match muxsmith_core::MKVMERGE_SKIP_MARKER (crates/muxsmith
[REPAIR          ] README.md:60: - **`raw:` is the deliberate opt-out.** Prefix a property name with `raw:` to
[NOISE           ] crates/muxsmith-cli/tests/run_live.rs:15: //!   that reproduces the exact same bytes -- would leave a fresh mtime
[NOISE           ] crates/muxsmith-cli/tests/run_live.rs:17: //!   resolution; full byte-content equality is checked on top, which alone
[NOISE           ] crates/muxsmith-cli/tests/run_live.rs:138: // 50/75% progress thresholds, neither safe to pin byte-for-byte even
[NOISE           ] crates/muxsmith-cli/tests/run_live.rs:279: /// match it byte-for-byte. The recipe's `title: { template: 'S{season}E{episo
[NOISE           ] crates/muxsmith-cli/tests/run_live.rs:511: // Non-vacuous "untouched": exact byte content is unchanged, and the
[NOISE           ] crates/muxsmith-cli/tests/run_live.rs:513: // write, even a same-bytes rewrite, would bump it to "now").
[NOISE           ] crates/muxsmith-core/src/executor/joblog.rs:66: /// (UTC) prefix -- the first 16 bytes, present even on a collision-suffixed
[NOISE           ] crates/muxsmith-core/src/executor/joblog.rs:300: .and_then(|bytes| fs::write(path, bytes));
[NOISE           ] crates/muxsmith-core/src/executor/spawn.rs:128: /// Reads one line from `r`, byte-wise, lossily decoding it rather than
[NOISE           ] crates/muxsmith-core/src/executor/spawn.rs:129: /// failing the whole stream on the first non-UTF-8 byte (#9): mkvmerge can
[NOISE           ] crates/muxsmith-core/src/executor/spawn.rs:134: /// comes back as `Some` with `U+FFFD` standing in for the bad bytes, exactly
[NOISE           ] crates/muxsmith-core/src/executor/spawn.rs:144: // raw bytes, so it cannot itself fail on decoding - that path is
[NOISE           ] crates/muxsmith-core/src/executor/spawn.rs:294: fn read_next_line_survives_non_utf8_bytes_without_truncating() {
[NOISE           ] crates/muxsmith-core/src/identify.rs:91: /// The `-J` `size` in bytes.
[NOISE           ] crates/muxsmith-core/src/lib.rs:27: // keeps every site and the CI grep byte-identical by construction.
[REPAIR          ] crates/muxsmith-core/src/matcher.rs:96: // `raw:` opt-in (D32, spec 9.2): untyped byte-literal value equality
[RETAINED        ] crates/muxsmith-core/src/matcher.rs:452: // (language / language_ietf) lookup: it byte-literally compares against the
[RETAINED        ] crates/muxsmith-core/src/matcher.rs:457: fn b8_raw_language_is_byte_literal_no_normalization() {
[NOISE           ] crates/muxsmith-core/src/planner.rs:955: // non-matching multi-byte tail must never be sliced into.
[NOISE           ] crates/muxsmith-core/src/planner.rs:1184: // substitutes invalid bytes with U+FFFD, so a non-UTF-8 path would silently
[NOISE           ] crates/muxsmith-core/src/profile/load.rs:32: // on Windows only: `file: Option<PathBuf>` is 8 bytes larger there (WTF-8
[NOISE           ] crates/muxsmith-core/src/profile/load.rs:34: // struct into clippy's 128-byte borderline. Cold config-load path; revisit if
[RETAINED        ] crates/muxsmith-core/src/profile/validate.rs:408: /// which `raw:` degrades to byte-literal equality; otherwise `RawProperty`
[REPAIR          ] crates/muxsmith-core/src/report/mod.rs:87: /// A `raw:` prefix was applied to a property that IS in the capability model
[NOISE           ] crates/muxsmith-core/src/template.rs:29: /// `chars()` sequence), not a byte offset: do not byte-slice the template
[NOISE           ] crates/muxsmith-core/tests/executor_no_hang_live.rs:37: // `\377\376` are raw invalid-UTF-8 bytes (POSIX `printf` octal
[NOISE           ] crates/muxsmith-core/tests/planner_non_utf8_path.rs:8: //! are arbitrary byte sequences, constructible via
[NOISE           ] crates/muxsmith-core/tests/planner_non_utf8_path.rs:9: //! `std::os::unix::ffi::OsStrExt::from_bytes` and occurring in practice as
[NOISE           ] crates/muxsmith-core/tests/planner_non_utf8_path.rs:58: let out_dir = dir.path().join(OsStr::from_bytes(b"out\xff"));
[NOISE           ] crates/muxsmith-core/tests/planner_non_utf8_path.rs:82: // byte becomes U+FFFD, and the rendered filename is appended below the
[NOISE           ] crates/muxsmith-core/tests/planner_non_utf8_path.rs:106: let bad_dir: PathBuf = dir.path().join(OsStr::from_bytes(b"s\xe4son")); // Lat
[NOISE           ] crates/muxsmith-core/tests/planner_resolution.rs:911: // pointed at donors_dir, that name is byte-for-byte B's donor path - for
[NOISE           ] crates/muxsmith-core/tests/planner_resolution.rs:1015: // "Z.mkv" collides byte-for-byte with the donor A alone resolved.
[NOISE           ] crates/muxsmith-core/tests/planner_resolution.rs:1120: // "Z.mkv" collides byte-for-byte with the attachment donor A alone
[NOISE           ] crates/muxsmith-core/tests/planner_resolution.rs:1233: // "Z.mkv" collides byte-for-byte with the chapters donor A alone
[NOISE           ] crates/muxsmith-core/tests/prop_planner.rs:4: //! * determinism: identical inputs produce a byte-identical serialized batch;
[NOISE           ] crates/muxsmith-core/tests/prop_planner.rs:338: // directory, fresh identifier caches) serialize byte-for-byte identically.
[NOISE           ] crates/muxsmith-core/tests/prop_planner.rs:340: fn plan_is_byte_identical_across_runs(
[NOISE           ] crates/muxsmith-core/tests/report_json.rs:3: //! documents are byte-identical to what the CLI printed before the hoist;
[NOISE           ] crates/muxsmith-core/tests/suggestions.rs:1003: // Keeps these fixtures byte-identical to what they produced pre-D49.
[RETAINED        ] crates/muxsmith-core/tests/validate_semantics.rs:249: // (codec_kind), degrading the match to byte-literal equality.
[DIFFERENT-CLAIM ] e2e/editor-dropdowns.spec.ts:80: test("case 4: a raw:type key keeps its free-text cell (byte equality; raw: byp
[NOISE           ] e2e/help-mode.spec.ts:40: *  own `innerHTML` byte-for-byte (both pass through the browser's parser). */
[NOISE           ] e2e/i18n-en.ts:7: * itself renders through, so a smoke assertion is byte-identical to what a
[NOISE           ] e2e/locale-switch.spec.ts:11: * assertion is byte-identical to what the app itself renders. The
[NOISE           ] e2e/smoke.spec.ts:1437: // byte-for-byte the machinery `ListWidget` already uses for AttachmentRule
[TRUE-ABOUT-REGEX] help/de/editor-match-expr-exact.md:11: - Zeichenketten vergleichen ... fuer byte-genaue Muster `regex`.
[REPAIR          ] help/de/editor-match-expr-exact.md:23: Ein Eigenschaftsname mit dem Praefix `raw:` ...
[TRUE-ABOUT-REGEX] help/en/editor-match-expr-exact.md:11: - Strings compare case-sensitively. For case-insensitive containment use `subs
[REPAIR          ] help/en/editor-match-expr-exact.md:23: Prefixing a property name with `raw:` (for example `raw:dolby_complexity_index
[RETAINED        ] locales/de/diagnostics.ftl:21: raw-on-known-property = Die Eigenschaft "{ $property }" ist eine bekannte Eige
[RETAINED        ] locales/en/diagnostics.ftl:14: raw-on-known-property = Property "{ $property }" is a known property with spec
[NOISE           ] src-tauri/src/lib.rs:280: /// `config_diagnostics` is therefore byte-identical in shape to
[NOISE           ] src-tauri/src/lib.rs:519: /// writes bytes. Paired deliberately with `dialog`, not `fs:default`: a
[NOISE           ] src-tauri/src/lib.rs:708: // byte-identical to validate_profile's, plus the parsed model under "profile"
[NOISE           ] src-tauri/src/run.rs:822: /// 16 bytes -- present even on a collision-suffixed directory name like
[NOISE           ] src-tauri/src/settings.rs:140: /// since the file exists and simply never parses. Instead: the bytes are
[NOISE           ] src-tauri/src/settings.rs:167: let bytes =
[NOISE           ] src-tauri/src/settings.rs:178: .write_all(&bytes)
[NOISE           ] src-tauri/src/settings.rs:307: // the temp file the atomic write staged its bytes in does not leak
[DIFFERENT-CLAIM ] src/editor/widgets/PropertyMapWidget.vue:130: // flaw), only for the byte-exact keys `type`/`codec_kind` (a `raw:type` key
[NOISE           ] src/help/topics.ts:4: /** Help topics, eagerly embedded at build time - byte-for-byte the
[NOISE           ] src/ipc.ts:266: * `load_profile`'s return shape (D42, as amended by Task 1): byte-identical
[NOISE           ] src/views/EditorView.vue:86: // beneath the grid. The panel is pure registry composition, byte-for-byte
[NOISE           ] src/views/EditorView.vue:362: // `trackRule` registry, byte-for-byte the machinery `ListWidget.vue` already
[REPAIR          ] docs/superpowers/specs/...-v1-design.md:146: `exact` is typed value-equality, not raw string equality: each property is com
[REPAIR          ] docs/superpowers/specs/...-v1-design.md:176: - **`raw:` opt-in (forward compatibility, D32).** A match property not in the
[REPAIR          ] docs/superpowers/specs/...-v1-design.md:280: | `RawOnKnownProperty` | warning | `raw:` applied to a model property with spe
[REPAIR          ] docs/superpowers/specs/...-v1-design.md:421: 2. **Runtime**: the local mkvmerge is queried for version, supported file type
```

**No finding.** No hit outside the 19 is a claim about the `raw:` comparison, so R'
has no hole in this vocabulary.

### 5.0 The end-state delta, measured as a SET rather than as two counts

71 -> 66 was verified by set-differencing the two sweeps on (file, text), not by
subtracting:

```
LINES PRESENT PRE, ABSENT POST (by file+text):
  - README.md:60                                     (R-1)
  - crates/muxsmith-core/src/matcher.rs:96           (R-2)
  - crates/muxsmith-core/src/report/mod.rs:87        (R-7)
  - help/de/editor-match-expr-exact.md:23            (R-9)
  - help/en/editor-match-expr-exact.md:23            (R-8)
  - v1 spec:146, :176, :280, :421                    (R-5, R-3, R-6, R-4)
LINES PRESENT POST, ABSENT PRE (by file+text):
  + crates/muxsmith-core/src/matcher.rs:210: /// compare byte-wise, `language` is not normalized here (the `raw:` arm runs
  + v1 spec:146, :176, :421                          (edited text, still carrying the word)
```

Nine repair lines lost their old text; three of them (spec `:146`, `:176`, `:421`)
return with the new text because their replacements legitimately keep "byte-literal
matching" (about `regex`) or "byte-for-byte" (about STRINGS). One line is new:
R-10's `scalar_eq_same_type` doc, "Strings compare byte-wise" - scoped to strings,
which D111 section 3.1 identifies as the one place the word is precise. `9 - 4 = 5`,
`71 - 5 = 66`. Not a finding.

### 5.1 Where this sweep does not look, and the probes into those places

Three blind spots, all named by D111 and all inspected:

1. **Case sensitivity.** `-nE` cannot see `matcher.rs`'s sentence-initial
   "Byte-literal against ..."; it is absent from the 71 (which is why 4.4 lists
   seven retained sites while the sweep classifies six). K's alternation carries
   `Byte-literal against` with the capital and does see it - section 4.1 above shows
   it at `:466` pre and `:547` post.
2. **Identifier form.** The b8 test's NAME carries `byte_literal` with underscores;
   neither R' nor K' can match it. The sweep does (`matcher.rs:457`, classified
   RETAINED above).
3. **Sites carrying no form of the word.** R-10's pre-state text, R-11 and R-12 are
   invisible to every vocabulary instrument here. They are reached by check R''
   (section 6.4) and by the three tests.

---

## 6. Step 8 - check R'', the exit bars, the example, the corpus, the diff scope

### 6.1 POST-state SI-3 probes, after the mandatory rebuild

The rebuild is not ceremony: none of the exit bars rebuilds `target/debug/muxsmith`.

```
$ cargo build -p muxsmith-cli
   Compiling muxsmith-core v0.1.0 ...
   Compiling muxsmith-cli v0.1.0 ...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
$ find crates src-tauri -name '*.rs' -newer target/debug/muxsmith
(no output)
```

Same six profiles, same two source files, same command:

```
### RUN 1 CROSS (Float 1.0 vs reported Int 1)             exit=2   <- INVERTED
  diagnostic codes: ['raw-property', 'unknown-property-skew', 'missing-track']
  plan.assignments: []
### RUN 2 CROSS (Int 400 vs reported Float 400.0)         exit=2   <- INVERTED
  diagnostic codes: ['raw-property', 'unknown-property-skew', 'missing-track']
  plan.assignments: []
### RUN 3 SAME (Int 1 vs Int 1)                           exit=1   <- unchanged
  diagnostic codes: ['raw-property', 'unknown-property-skew']
  plan.assignments: [(0, 0, 'audio')]
### RUN 4 SAME (Float 400.0 vs Float 400.0)               exit=1   <- unchanged
  diagnostic codes: ['raw-property', 'unknown-property-skew']
  plan.assignments: [(0, 0, 'video')]
### NEG CONTROL 1 (2.0)                                   exit=2   <- unchanged
### NEG CONTROL 2 (401)                                   exit=2   <- unchanged
```

Both cross directions stopped matching; both same-kind directions still match; both
negative controls still fire. That is the end-to-end proof of the behaviour change
through the shipped binary (W3-h's second half).

### 6.2 The three tests, and their own fires

The exit bars require the tests green. They do not require the tests to be shown
capable of failing, and a guard test that would pass against a broken mechanism is
documentation rather than coverage - so both mutations were run, with the CURRENT
file content as the restore baseline and `command cp -f` as the restore.

```
$ sha256sum crates/muxsmith-core/src/matcher.rs
022d0edca89c8b6532c2269dd74575dc09805765875cef5801c949135c873e7b

=== MUTATION 1: strip the two cross arms from scalar_eq (the A-2 defect) ===
test matcher::tests::b5_raw_unknown_present_matches_untyped ... ok
test matcher::tests::b6_raw_absent_unknown_does_not_match_no_false_when_absent ... ok
test matcher::tests::b7_raw_does_not_cross_compare_int_and_float ... ok
test matcher::tests::b8_raw_language_is_byte_literal_no_normalization ... ok
test matcher::tests::raw_compares_only_within_one_kind ... ok
test matcher::tests::typed_exact_still_cross_compares_int_and_float ... FAILED
test result: FAILED. 123 passed; 1 failed; 0 ignored
=== RESTORE ===
crates/muxsmith-core/src/matcher.rs: OK

=== MUTATION 2: re-point the raw: call site back to scalar_eq (undo the behaviour) ===
test matcher::tests::b5_raw_unknown_present_matches_untyped ... ok
test matcher::tests::b6_raw_absent_unknown_does_not_match_no_false_when_absent ... ok
test matcher::tests::b7_raw_does_not_cross_compare_int_and_float ... FAILED
test matcher::tests::b8_raw_language_is_byte_literal_no_normalization ... ok
test matcher::tests::raw_compares_only_within_one_kind ... FAILED
test matcher::tests::typed_exact_still_cross_compares_int_and_float ... ok
test result: FAILED. 122 passed; 2 failed; 0 ignored
=== RESTORE ===
crates/muxsmith-core/src/matcher.rs: OK
```

The two mutations are cleanly disjoint, which is the measurement D111 and the brief
both assert: **stripping the cross arms is caught by T-1 and by nothing else** (1 of
124 fails), and **undoing the behaviour change is caught by T-2 and T-3 and by
nothing else** (2 of 124). T-1 is a live safeguard, not a formality.

### 6.3 Task exit bars, all foreground, all green

```
$ cargo fmt --all --check
exit=0

$ cargo clippy --workspace --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.61s
exit=0

$ cargo test -p muxsmith-core
test matcher::tests::b5_raw_unknown_present_matches_untyped ... ok
test matcher::tests::b6_raw_absent_unknown_does_not_match_no_false_when_absent ... ok
test matcher::tests::b8_raw_language_is_byte_literal_no_normalization ... ok
test matcher::tests::b7_raw_does_not_cross_compare_int_and_float ... ok
test matcher::tests::raw_compares_only_within_one_kind ... ok
test matcher::tests::typed_exact_still_cross_compares_int_and_float ... ok
running 124 tests
test result: ok. 124 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
exit=0
$ cargo test -p muxsmith-core 2>&1 | grep -c "b7_raw_int_float_cross_compare"
0                              # the renamed test is gone from the suite

$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.85s
   Generated .../target/doc/muxsmith_cli/index.html and 5 other files
exit=0

$ pnpm check:i18n
check-i18n: ok (41 source files scanned, 212 catalog ids, 19 IpcError code(s) gated,
22 help id(s) x 2 help locale(s), 0 unused warning(s), 1 other locale(s) checked for
parity against 7 en/ catalog(s)).
exit=0

$ cargo test --workspace
exit=0     # 39 "test result:" lines, every one "0 failed":
           #   aggregate: 39 x "0 failed"
```

The workspace run confirms D111's M4 prediction: no snapshot, fixture or integration
test moved.

**The `cargo doc` gate was fired, because a green rustdoc run is also what a run that
never reached R-10's doc comment produces.** Breaking one of R-10's two intra-doc
links:

```
=== FIRE: [`scalar_eq`] -> [`scalar_eq_typo_does_not_exist`] ===
error: unresolved link to `scalar_eq_typo_does_not_exist`
error: could not document `muxsmith-core`
doc exit after break=101
=== RESTORE ===
crates/muxsmith-core/src/matcher.rs: OK
doc exit after restore=0
```

### 6.4 Check R'' - the retirement construction run over the REPLACEMENTS

D111 section 4.6's loosened, newline-flattening expression, run verbatim.

```
--- R'' over the FIVE edited product files (expected 0 strict, 0 loose) ---
  README.md: strict=0 loose=0
  crates/muxsmith-core/src/matcher.rs: strict=0 loose=0
  crates/muxsmith-core/src/report/mod.rs: strict=0 loose=0
  help/en/editor-match-expr-exact.md: strict=0 loose=0
  help/de/editor-match-expr-exact.md: strict=0 loose=0
  TOTAL: strict=0 loose=0
```

**Run over the SIXTH edited file as well**, since Task A3 edits six files while the
plan's phrasing says "the five edited product files":

```
--- R'' over the v1 spec ---
  docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md: strict=0 loose=2
      LOOSE: ...is matched untyped (value equality against the property named verbatim with no type conversion: a strin...
      LOOSE: ...is matched untyped: value equality against the property named verbatim with no type conversion, so the ...
```

Both are the two candidates D111 section 4.6 already measured and ruled PERMITTED:
R-3's and R-4's, where a bracket and a colon respectively separate the path sense of
"matched untyped" from the equality that is defined after it. Nothing new.

**Fired control**, so a zero is a measurement: the same expression against R-2's
pre-fix text.

```
loose hits in the pre-state matcher.rs: 1
  ...`raw:` opt-in (D32, spec 9.2): untyped byte-literal value equality against the property named verbatim. It bypasses the `language` norm...
```

### 6.5 The README example, both directions, and the corpus derivation

**Corpus derivation.** Surface: every fenced `yaml`/`yml` block in tracked markdown
under `README.md`, `docs/` and `help/`, excluding `docs/process-journal*` and
`docs/superpowers/plans` as history. Discriminator: a block is a standalone profile
iff it declares `profile_version` at column 0.

```
PRE-state:  6 fenced yaml/yml blocks | 3 standalone profiles | 3 fragments
   PROFILE README.md:28-50                                  input.pattern present: False
   PROFILE README.md:79-84                                  input.pattern present: True
   PROFILE docs/superpowers/specs/...-v1-design.md:54-116   input.pattern present: True
   FRAGMENT docs/superpowers/specs/2026-07-15-plan-6-design.md:185-193     first: "# before"
   FRAGMENT docs/superpowers/specs/2026-07-15-plan-6-design.md:1426-1428   first: "- source: primary"
   FRAGMENT docs/superpowers/specs/2026-07-22-plan8-packaging-release-design.md:1263-1484  first: "name: release"

POST-state: 6 fenced yaml/yml blocks | 3 standalone profiles | 3 fragments
   PROFILE README.md:28-51                                  input.pattern present: True
   PROFILE README.md:80-85                                  input.pattern present: True
   PROFILE docs/superpowers/specs/...-v1-design.md:54-116   input.pattern present: True
```

**Exactly one lacked `pattern`. Delta against the ruling's named site: zero**, as the
plan's authoring measurement states (6 / 3 / 3, one defective). The three fragments
are two rule-list snippets in a retired design document (plan-6) and one GitHub
workflow (`name: release`, inside the plan-8 packaging design), member for member as
described.

*A method note on the discriminator:* `pattern` presence must be decided by PARSING
the block, not by grepping for an indented `pattern:` line. README's passthrough
example writes `input: { pattern: ..., extensions: [mkv] }` as an inline flow
mapping, and a line-shaped test reports it as missing - a false second defect. The
derivation above parses each block with `yaml.safe_load` and asks for
`input.pattern`.

**Three blind-spot probes, re-run:**

```
PROBE 1 - fenced blocks declaring `input:` at column 0 without `profile_version`: 0
PROBE 2 - the three fragments, identified above: two rule-list snippets in a retired
          design document, one GitHub workflow. Confirmed.
PROBE 3 - tracked non-docs/ .yaml|.yml files, profile-shaped ones must carry input.pattern:
   non-profile .github/workflows/ci.yml
   non-profile .github/workflows/release.yml
   PROFILE crates/muxsmith-cli/tests/fixtures/bad.yaml:              input.pattern present: True
   PROFILE crates/muxsmith-cli/tests/fixtures/good.yaml:             input.pattern present: True
   PROFILE crates/muxsmith-core/tests/fixtures/all-non-default.yaml: input.pattern present: True
   PROFILE crates/muxsmith-core/tests/fixtures/reference.yaml:       input.pattern present: True
```

**`muxsmith validate` on an extracted scratch copy of each standalone profile:**

```
=== PRE-STATE (RED) ===
--- README first example
[error] ... input: The profile could not be parsed: input: missing field `pattern` at line 4 column 3
1 error, 0 warnings, 0 infos.
    exit=2
--- README passthrough example
[info] tracks.rules: This profile defines no track rules and tracks.unmatched is keep: ...
0 errors, 0 warnings, 1 info.
    exit=0
--- v1 spec example
Profile is valid.
    exit=0

=== POST-STATE (GREEN) ===
--- README first example
Profile is valid.
    exit=0
--- README passthrough example
[info] tracks.rules: ...
    exit=0
--- v1 spec example
Profile is valid.
    exit=0
```

**The inserted line and its comment column**, measured against the block's three
other end-of-line comments:

```
31: col 30    pattern: '.*'              # every candidate file; the whole basename is the identifier
37: col 30    on_collision: error        # never clobber anything by accident
40: col 30    unmatched: drop            # nothing you didn't ask for survives
50: col 30        optional: true         # fine if a file doesn't have one
```

### 6.6 Diff scope

```
$ git diff --stat
 README.md                                          |   3 +-
 crates/muxsmith-core/src/matcher.rs                | 150 ++++++++++++++++++---
 crates/muxsmith-core/src/report/mod.rs             |   2 +-
 .../specs/2026-07-08-muxsmith-v1-design.md         |   8 +-
 help/de/editor-match-expr-exact.md                 |   2 +-
 help/en/editor-match-expr-exact.md                 |   2 +-
 6 files changed, 142 insertions(+), 25 deletions(-)

$ git status --short          # no untracked file either
 M README.md
 M crates/muxsmith-core/src/matcher.rs
 M crates/muxsmith-core/src/report/mod.rs
 M docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md
 M help/de/editor-match-expr-exact.md
 M help/en/editor-match-expr-exact.md

$ git diff --exit-code -- locales/ ; echo $?
0
$ git diff --exit-code -- crates/muxsmith-core/src/profile/validate.rs crates/muxsmith-core/tests/validate_semantics.rs ; echo $?
0
```

**Fired control for the two `--exit-code` checks**, because a clean `--exit-code`
result is indistinguishable from one aimed at a path that cannot change:

```
$ git diff --exit-code -- README.md > /dev/null ; echo $?
1
```

**The strongest scope instrument: reconstruct, do not inspect.** Each of the six
files was rebuilt from its `HEAD` blob by applying ONLY the fenced substitutions
(D111's ten OLD/NEW pairs, its three Rust blocks from sections 3.2/3.3/5, and the
plan's own Step-6 line) and compared byte for byte against the working tree:

```
RECONSTRUCTION vs WORKING TREE, byte for byte:
  IDENTICAL  README.md                                     sha=245773eccadc0562
  IDENTICAL  crates/muxsmith-core/src/matcher.rs           sha=022d0edca89c8b65
  IDENTICAL  crates/muxsmith-core/src/report/mod.rs        sha=18e83e43f85be15c
  IDENTICAL  docs/superpowers/specs/...-v1-design.md       sha=787142eae715bdcf
  IDENTICAL  help/de/editor-match-expr-exact.md            sha=4d1bec79105f83b1
  IDENTICAL  help/en/editor-match-expr-exact.md            sha=5ddbc75875dc2018

Every byte of the change is accounted for by a D111/plan fence: True
```

This is stronger than a file-level byte-identity check (impossible here for
`matcher.rs` and the spec, which are in both the repair and the retained set): it
proves not merely that the retained text survived, but that **no character anywhere
in the six files came from anywhere other than a fence.**

### 6.7 Typography and the two help-topic constraints

```
$ git diff -U0 | grep '^+' | grep -v '^+++'   # scanned for em/en dash, figure dash,
                                              # horizontal bar, Unicode minus, smart
                                              # quotes, ellipsis, NBSP
total AI-tell glyph hits in added lines: 0

$ ... | grep -nP '[^\x00-\x7F]'                # non-ASCII in added lines
141:+Ein Eigenschaftsname mit dem Präfix `raw:` ...     # help/de only, as fenced

R-8 / R-9 content constraints (pnpm check:i18n hard-fails on these over help/):
  help/en/editor-match-expr-exact.md: '|' 0   http(s):// 0   raw HTML tags 0
  help/de/editor-match-expr-exact.md: '|' 0   http(s):// 0   raw HTML tags 0
```

### 6.8 Test duty

Three tests ship in this package (`tests-ship-with-the-feature-never-after`), which
is the inverse of the pre-amendment position. That position rested on
`b7_raw_int_float_cross_compare` covering the behaviour; that test is the assertion
being inverted, so it was never coverage for the new behaviour. T-1 additionally
guards a behaviour this task does NOT change, which is the case
`proc-proposed-safeguard-stays` exists for - and section 6.2 measured that it is the
only check in the suite that catches its target. Nothing was deferred.

**`ledger-lint` is not cited as coverage for anything in this task.**

---

## 7. Step 9 - surfaced, not edited

This task creates no tracker line and edits no controller-owned file
(`docs/ROADMAP.md`, `docs/decision-ledger.yaml`, `docs/conventions.yaml`,
`docs/process-conventions.yaml`, `docs/product-boundaries.yaml`,
`docs/process-journal.md` are all untouched - see the diff scope above).

### 7.1 The seven retained assertions and the two different-claim sites

Harmonization question for the controller. Each retained site is scoped to
`language`/`codec_kind`, both string-typed, and uses "byte-literal" to contrast with
NORMALIZATION rather than with typing:

| site | wording |
|---|---|
| v1 spec `:421`, second occurrence | "that `raw:` degrades to byte-literal equality" |
| `crates/muxsmith-core/src/profile/validate.rs:408` | "which `raw:` degrades to byte-literal equality" |
| `crates/muxsmith-core/tests/validate_semantics.rs:249` | "degrading the match to byte-literal equality" |
| `locales/en/diagnostics.ftl:14` | "...and matches byte-literally instead" |
| `locales/de/diagnostics.ftl:21` | "...und gleicht stattdessen byte-literal ab" |
| `crates/muxsmith-core/src/matcher.rs:533` (was `:452`) | "it byte-literally compares against the `language` property alone" |
| `crates/muxsmith-core/src/matcher.rs:547` (was `:466`) | "Byte-literal against the `language` field itself still works." |

Plus the b8 test's NAME (`matcher.rs:538`, was `:457`), retained deliberately.

Two sites outside the claim class, named so a later reader does not think this task
missed them: `src/editor/widgets/PropertyMapWidget.vue:130` and
`e2e/editor-dropdowns.spec.ts:80`, both about which KEY STRINGS get a dropdown, where
byte equality of the key name is exactly what happens. Both re-read; both TRUE.

### 7.2 D111's twelve triggers, for the controller to mirror

| # | trigger |
|---|---|
| T1 | Tier-1 `core-91-raw-opt-in` (`docs/decision-ledger.yaml:521`) ends "Shipped exactly per the binding B-1..B-11 acceptance table"; B-7's expected outcome inverts, so the transitive claim goes false |
| T2 | Tier-1 `core-98-raw-language-single-field` (`:558`) reads "raw:X reads exactly the property literally named X, byte-exact"; substance unchanged, phrase invites the retired reading. Harmonization candidate |
| T3 | Tier-1 `core-97-raw-on-known-property` (`:546`) states the degradation "to byte-literal equality"; same vocabulary question R-6/R-7 resolve in the spec and the DiagCode doc |
| T4 | Tier-2 `core-72-exact-typed-value-equality` (`docs/conventions.yaml:331`) is now guarded by a named test - an occurrence recording that `typed_exact_still_cross_compares_int_and_float` is its check |
| T5 | D32's matcher case table row **B-7** (`docs/superpowers/specs/2026-07-11-plan-5.5-design-decisions.md:74`) needs a `superseded by D111` link, **not an edit** - it is an append-only record |
| T6 | The ROADMAP entry beginning `"byte-exact" overstates` carries two claims D111 measured differently (its P3 and P4), and its own defect description uses the retired phrase (which is why it is R''s soundness control). **Plus a correction:** the first-draft "exit 0 / fails silently" gloss already mirrored into the ROADMAP is wrong - the optional case exits **1**, and so does a successful `raw:` match, so the exit discriminates nothing |
| T7 | **Open owner question** (D111 section 6): a config-time diagnostic when a `raw:` key names a KNOWN property whose declared kind the profile value's scalar kind can never equal (`raw:audio_channels: 1.0`, `raw:max_luminance: 400`). D111's recommendation: build it as its own package, warning severity, own code. Routed, not closed |
| T8 | Pre-existing: `unknown-property-skew`'s "was matched untyped" fires even when the rule matched nothing (`planner.rs:630`). "was compared untyped" would be accurate. Two locales |
| T9 | Pre-existing: `missing-track` renders bare with `"params": {}` and `suggestions` is `[]`, for a `raw:` non-match and for a known-property control alike |
| T10 | Pre-existing, re-verified: `codec_kind` is absent from schema v20's 59 track properties, so `raw:codec_kind` can never match while `RawOnKnownProperty` still warns about it |
| T11 | Pre-existing: spec `:146` opens "typed value-equality, not raw string equality", which collides verbally with the `raw:` feature name one section later. Repairing it ripples into `core-72` and the plan-3.5 principle record, both controller-owned |
| T12 | Any future sweep of the `raw:` claim surface must use the RULE in D111 section 4.1, not an enumerated file list |

**T7 is the one with a decision attached**, and this task's own measurement
strengthens it: both cases the proposed guard would catch are exactly the two probes
in section 6.1 that flipped from matching to `missing-track`, and for an OPTIONAL
rule neither produces an error severity.

---

## 8. Concerns and observations (nothing blocking)

1. **D111's ROADMAP line citation has drifted** (section 3.3): the R' soundness
   control's ninth hit is at `docs/ROADMAP.md:1922` at this tree, not `:1913`. D111
   anticipated exactly this and cites the line by wording; the wording is
   byte-identical and the count is 9 as prescribed. Recorded because a reviewer
   re-running the control will see a different number than D111 prints.

2. **"the five edited product files" versus six edited files.** D111 section 4.6 and
   plan Step 8 both scope check R'' to "the five edited product files" while Task
   A3's Files list is six. Read as the five shipped-product files (README, two
   source files, two help topics), the v1 spec being a design document. R'' was run
   over all six and both results are reported (section 6.4); the spec's two loose
   candidates are the two D111 already measured as permitted. No wording was changed
   on this reading.

3. **Test placement is the one unfenced decision** (section 2). Three tests are
   fenced verbatim; where each lands in the `tests` module is not written anywhere.
   The choices are stated above so a reviewer grades them rather than discovering
   them. If the intended placement differs, moving a test is a no-op for behaviour.

4. **The corpus discriminator needs a YAML parse, not a grep** (section 6.5): a
   line-shaped `pattern:` test reports README's passthrough example (inline flow
   mapping) as a second defective profile. Worth carrying into any future
   example-validation check, which is already out of scope per Step 6 and
   owner-scheduled as its own vehicle.

5. **Nothing in this task compares the applied text against D111**, as the plan says.
   The closest available substitute was built anyway and is section 6.6's
   reconstruction: the end state is byte-identical to `HEAD` plus only the fences, so
   a drifted duplicate is excluded by construction rather than by care.

6. **Not touched, deliberately:** `docs/ROADMAP.md`'s "Docs accuracy" entry (its
   recorded disposition that the behaviour stays and only the wording changes is
   inverted by the ruling; correcting the tracker is a controller close action), the
   four house-knowledge YAMLs, D32's B-7 row, the concurrent Amendment 5 on
   `master`, and `/home/senol/Git/muxsmith-plan11-b`.

---

## 9. Step 10 - commit

Staged explicitly, never `git add -A`; unsigned; exactly one trailer; no push.

```
git add docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md crates/muxsmith-core/src/matcher.rs crates/muxsmith-core/src/report/mod.rs README.md help/en/editor-match-expr-exact.md help/de/editor-match-expr-exact.md
git -c commit.gpgsign=false commit -m "matcher: raw: compares without type conversion (D111), and the typed exact path keeps its cross arms" -- docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md crates/muxsmith-core/src/matcher.rs crates/muxsmith-core/src/report/mod.rs README.md help/en/editor-match-expr-exact.md help/de/editor-match-expr-exact.md
```

Trailer:

```
Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
```

Executed as one `git commit` with a second `-m` carrying the trailer, which produces
the subject, a blank line, and exactly that one trailer.

**Commit SHA:** `164e5718508d9bc830bb4f74db07e4466903bd3d`

```
$ git log -1 --format='%H%n%GG%n---MESSAGE---%n%B'
164e5718508d9bc830bb4f74db07e4466903bd3d
                                    <- %GG empty: unsigned, as required
---MESSAGE---
matcher: raw: compares without type conversion (D111), and the typed exact path keeps its cross arms

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>

$ git show --stat --format= HEAD
 README.md                                          |   3 +-
 crates/muxsmith-core/src/matcher.rs                | 150 ++++++++++++++++++---
 crates/muxsmith-core/src/report/mod.rs             |   2 +-
 .../specs/2026-07-08-muxsmith-v1-design.md         |   8 +-
 help/de/editor-match-expr-exact.md                 |   2 +-
 help/en/editor-match-expr-exact.md                 |   2 +-
 6 files changed, 142 insertions(+), 25 deletions(-)

$ git status --short
(clean)
```

Not pushed. Branch `plan-11-stream-a`, `5d305a2 -> 164e571`. The stream's
eleven-part gate is the controller's dispatch and was not run here.
