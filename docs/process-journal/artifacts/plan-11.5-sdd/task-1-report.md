# Task 1 implementer report - Plan 11.5

**Status:** COMPLETE. One key added to `deny.toml`'s `[advisories]` table with a
comment; six verification runs (the four the brief names plus two extra controls
explained under Run 3).

---

## 1. Source verification of the key, ahead of the brief

Read at `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cargo-deny-0.19.9/`,
against the installed binary (`cargo deny --version` -> `cargo-deny 0.19.9`, at
`/home/senol/.cargo/bin/cargo-deny`), so the source read and the runs below are
the same version.

`src/advisories/cfg.rs:268-270` - the TOML spelling and the default:

```rust
        let unused_ignored_advisory = th
            .optional("unused-ignored-advisory")
            .unwrap_or(LintLevel::Warn);
```

`src/advisories/cfg.rs:95-97` - what it governs, in cargo-deny's own words:

```rust
    /// Determines the response to advisories in the `ignore`ed list which do not
    /// exist in the dependency tree.
    pub unused_ignored_advisory: LintLevel,
```

`src/advisories.rs:182-205` - the two diagnostics whose severity it sets. Note it
covers `ignore` **and** `ignore-yanked`; we use only the former:

```rust
    // Check for advisory identifiers that were set to be ignored, but
    // were not actually encountered, for cases where a crate, or specific
    // version of that crate, has been removed or replaced and the advisory
    // no longer applies to it, so that users can cleanup their configuration
    for ignore in ignore_hits
        .into_iter()
        .zip(ctx.cfg.ignore.iter())
        .filter_map(|(hit, ignore)| if !hit { Some(ignore) } else { None })
    {
        sink.push(
            ctx.diag_for_advisory_not_encountered(ignore, ctx.cfg.unused_ignored_advisory.into()),
        );
    }
```

`src/advisories/diags.rs:316-333` - the message and label text the brief and the
ROADMAP both quote, confirmed verbatim: `"advisory was not encountered"` with the
label `"no crate matched advisory criteria"`, code `Code::AdvisoryNotDetected`.

`src/lib.rs:35-45` and `src/diag.rs:216-221` - `LintLevel::Deny` is a valid value
and maps to `Severity::Error`; the doc comment on the variant says "the command
will fail with a non-zero exit code".

**Result: no contradiction with the brief.** The key exists under that exact
name, defaults to `warn`, accepts `deny`, and turns the unused-ignore finding
into an error. Nothing needed a NEEDS_CONTEXT return.

**One adjacent mechanism worth recording, because it is a trap for anyone
re-running the obsolescence test.** An ignored id that is not in any advisory
database takes a *different* path, `src/advisories.rs:174-180` ->
`diag_for_unknown_advisory`, and that diagnostic is hard-coded
`Diagnostic::new(Severity::Warning)` (`src/advisories/diags.rs:353-364`) - it is
**not** governed by this key and cannot be made fatal by it. A test using a
fabricated id would therefore exit 0 and prove nothing. Run 3 below uses
`RUSTSEC-2016-0001`, a real advisory present in the local database
(`~/.cargo/advisory-dbs/advisory-db-3157b0e258782691/crates/openssl/RUSTSEC-2016-0001.md`)
whose crate `openssl` is absent from `Cargo.lock` (`grep -c '^name = "openssl"$'
Cargo.lock` -> 0), which is the state the brief actually describes.

## 2. The change

Postcondition asserted (not a line number): **`unused-ignored-advisory = "deny"`
is a member of the `[advisories]` table** - it follows the `[advisories]` header
and precedes the next table header. Machine-checked:

```
$ awk '/^\[/{t=$0} /^unused-ignored-advisory[[:space:]]*=/{print "line " NR ": key is in table " t}' deny.toml
line 28: key is in table [advisories]
```

Within that constraint it sits directly after `unsound = "all"` and before the
`ignore` list's header comment, i.e. between the scope key and the list, the two
things it guards, so the file reads scope -> guard -> suppressions.

### The comment, as committed

```toml
# Escalate cargo-deny's default warning to an error when an entry in
# `ignore` matched nothing. An entry counts as hit only when its advisory
# actually fires, so this catches both ways of losing one silently: drop the
# `unsound` scope key and that class stops being evaluated, leaving its
# ignore entry suppressing nothing while the check still reports success;
# and an advisory that gets fixed upstream, or whose crate leaves Cargo.lock,
# stops matching for the good reason. The second is the expected day-to-day
# outcome and the point of the key - a failing gate printing `no crate
# matched advisory criteria` at the exact line is the tool telling you to
# delete an ignore entry that has done its job, not a regression.
unused-ignored-advisory = "deny"
```

Reasoning for the wording:

- **Sentence 1 is what the key does**, stated as the delta from the default
  (`warn`) rather than in the abstract, because the default is what a reader
  comparing against cargo-deny's docs will have in mind.
- **Sentence 2 carries the mechanism that makes both failure modes one failure
  mode**: an entry is marked hit only when its advisory actually fires. Without
  that clause the reader has to take on faith that deleting an unrelated scope
  key can invalidate an ignore entry.
- **The two clauses after the colon are the two failure modes**, in the brief's
  order, phrased so neither reads as the anomaly: the first ends with "while the
  check still reports success" (the silence that is the actual defect), the
  second with "for the good reason".
- **The last sentence is the day-to-day consequence and its interpretation.**
  "not a regression" is there because the first person to meet this failure will
  be someone pushing an unrelated change, and the cheapest wrong reaction is to
  delete the key rather than the ignore entry.
- **Nothing that goes stale.** No advisory id, no crate name, no line number, no
  pointer to the ROADMAP entry, no restatement of it - the comment says what the
  key does and what to do when it fires; the ROADMAP entry says which entry to
  delete and to re-confirm `unsound = "all"` while in the file. I also avoided
  "above"/"below" for the `unsound` key after an earlier draft used it: the key
  is one table member away today, and a later reorder would make the word wrong
  without making anything red.
- **Register and width** match the file: prose sentences, backticked config keys,
  ASCII hyphen. Widths of the inserted lines run 65-77 columns against the file's
  pre-existing maximum of 77 (`awk '/^#/ {print length($0)}' deny.toml | sort -rn
  | head -1` -> 77 both before and after the edit; the pre-existing 77-column line
  is the `[licenses]` block's first comment line, and one of my lines matches it
  without exceeding it).

## 3. Verification

Every run below is a direct invocation with `$?` read immediately - no pipelines.
Variant configs live outside the repository under
`/tmp/claude-1000/-home-senol-agents-peter/3b6e29f8-11ef-45a9-b757-6cf02a7f1687/scratchpad/`
and are driven with `cargo deny check advisories -c <path>`.

### Variant construction, with the mutation proved

```
$ grep -vx 'unsound = "all"' deny.toml > $S/deny-v2-no-unsound.toml
$ sed 's/^    "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness$/    "RUSTSEC-2016-0001", # substituted: openssl, absent from this tree/' deny.toml > $S/deny-v3a-replaced.toml
$ sed 's/^    "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness$/&\n    "RUSTSEC-2016-0001", # added: openssl, absent from this tree/' deny.toml > $S/deny-v3b-added.toml
$ git show HEAD:deny.toml | grep -vx 'unsound = "all"' > $S/deny-v4-control.toml
$ git show HEAD:deny.toml | sed 's/^    "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness$/&\n    "RUSTSEC-2016-0001", # added: openssl, absent from this tree/' > $S/deny-v4b-control-obsolescence.toml
```

Each variant's four load-bearing properties, counted rather than assumed (a
`sed`/`grep` that silently matched nothing would otherwise produce a variant
identical to the original and a test that passes for free):

```
v2-no-unsound:  unsound-line=0 key-line=1 0429=1 2016-0001=0
v3a-replaced:   unsound-line=1 key-line=1 0429=0 2016-0001=1
v3b-added:      unsound-line=1 key-line=1 0429=1 2016-0001=1
v4-control:     unsound-line=0 key-line=0 0429=1 2016-0001=0
v4b-control:    unsound-line=1 key-line=0 0429=1 2016-0001=1
```

The v4/v4b pair is built from `git show HEAD:deny.toml`, i.e. the file as it
stood before this task, which is why `key-line=0` there is a property of the
source and not of a deletion I could have got wrong.

### Run 1 - shipped state after the edit

```
$ cd /home/senol/Git/Muxsmith && cargo deny check advisories
EXIT=0
advisories ok
```

### Run 2 - the dropped-scope regression (key present, `unsound` line deleted)

```
$ cargo deny check advisories -c $S/deny-v2-no-unsound.toml
EXIT=1
error[advisory-not-detected]: advisory was not encountered
   ┌─ .../deny-v2-no-unsound.toml:74:6
   │
74 │     "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness
   │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria

advisories FAILED
```

Exit 1, `no crate matched advisory criteria`, the ignore line named by file, line
and column.

### Run 3 - the obsolescence case (the one the owner asked for)

**3a, exactly as the brief specifies it (id replaced):**

```
$ cargo deny check advisories -c $S/deny-v3a-replaced.toml
EXIT=1
error[unsound]: Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
    ┌─ /home/senol/Git/Muxsmith/Cargo.lock:138:1
    │
138 │ glib 0.18.5 registry+https://github.com/rust-lang/crates.io-index
    │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ unsound advisory detected
    │
    ├ ID: RUSTSEC-2024-0429
    [... advisory text and the eleven-parent dependency tree elided; full output
     in the transcript, unchanged from the Plan-11 measurement ...]

error[advisory-not-detected]: advisory was not encountered
   ┌─ .../deny-v3a-replaced.toml:75:6
   │
75 │     "RUSTSEC-2016-0001", # substituted: openssl, absent from this tree
   │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria

advisories FAILED
```

**This run is confounded, and I am flagging it rather than reporting it as the
proof.** Replacing the glib id removes glib's *own* suppression while
`unsound = "all"` is still set, so the run emits two errors: the unignored
`error[unsound]` for glib, and the `advisory-not-detected` we are testing for.
The first would have failed the run with or without the new key - which is the
precise defect the brief's Run 4 exists to exclude for Run 2. So Run 3a shows the
right diagnostic but its exit code is not attributable to the key.

**3b, the same case with the confound removed** - glib's ignore entry kept, the
non-matching id added alongside, which is also the more faithful simulation of
"glib is fixed": the ignore list is unchanged and one entry simply stops
matching.

```
$ cargo deny check advisories -c $S/deny-v3b-added.toml
EXIT=1
error[advisory-not-detected]: advisory was not encountered
   ┌─ .../deny-v3b-added.toml:76:6
   │
76 │     "RUSTSEC-2016-0001", # added: openssl, absent from this tree
   │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria

advisories FAILED
```

One error, exit 1. This is the run that demonstrates the owner's case: on the day
the ignored advisory stops applying, the check fails by itself and names the line
to delete.

### Run 4 - the controls (the key is what does the work)

**4, as the brief specifies** - no key, `unsound` line deleted:

```
$ cargo deny check advisories -c $S/deny-v4-control.toml
EXIT=0
warning[advisory-not-detected]: advisory was not encountered
   ┌─ .../deny-v4-control.toml:63:6
   │
63 │     "RUSTSEC-2024-0429", # glib VariantStrIter unsoundness
   │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria

advisories ok
```

Identical finding, identical label, `warning` instead of `error`, `advisories ok`,
exit 0. Run 2 minus the key is exit 0; the single-key difference is the whole
effect.

**4b, the matching control for Run 3b** (not requested by the brief; added
because Run 4 controls only the dropped-scope case and Run 3 needs its own) - no
key, `unsound = "all"` intact, the non-matching id added:

```
$ cargo deny check advisories -c $S/deny-v4b-control-obsolescence.toml
EXIT=0
warning[advisory-not-detected]: advisory was not encountered
   ┌─ .../deny-v4b-control-obsolescence.toml:65:6
   │
65 │     "RUSTSEC-2016-0001", # added: openssl, absent from this tree
   │      ━━━━━━━━━━━━━━━━━ no crate matched advisory criteria

advisories ok
```

3b and 4b differ in exactly one line of config and in nothing else; exit 1 vs 0.

### The repository's own `deny.toml` was never mutated to produce a variant

Hashed before the variant runs and re-checked after all five of them:

```
$ sha256sum deny.toml   # before
42a10942f9b0b39894a8cef91d8788570d38cc8b6f71ead5470959cf0ff73c0c  deny.toml
$ sha256sum -c $S/deny-before-variants.sha256   # after
deny.toml: OK
EXIT=0
```

The brief asks for `git diff --exit-code -- deny.toml` as this proof. That
command cannot serve it in this task, because the task's own deliverable is a
change to `deny.toml`; it exits 1 by construction:

```
$ git diff --exit-code -- deny.toml > /dev/null 2>&1; echo "EXIT=$?"
EXIT=1
```

The hash pair above is the equivalent proof that holds: the working-tree file was
byte-identical before and after every variant run, so no variant was produced by
editing it. The diff below shows the exit-1 content is my addition and nothing
else.

### Diff scope

```
$ git diff --stat
 deny.toml | 11 +++++++++++
 1 file changed, 11 insertions(+)
```

Exactly one file; **11 insertions, 0 deletions**, so no existing ignore id can
have been reworded, reordered or removed.

```
$ git diff -U0 -- deny.toml
diff --git a/deny.toml b/deny.toml
index 27f3679..d7dc3e2 100644
--- a/deny.toml
+++ b/deny.toml
@@ -17,0 +18,11 @@ unsound = "all"
+# Escalate cargo-deny's default warning to an error when an entry in
+# `ignore` matched nothing. An entry counts as hit only when its advisory
+# actually fires, so this catches both ways of losing one silently: drop the
+# `unsound` scope key and that class stops being evaluated, leaving its
+# ignore entry suppressing nothing while the check still reports success;
+# and an advisory that gets fixed upstream, or whose crate leaves Cargo.lock,
+# stops matching for the good reason. The second is the expected day-to-day
+# outcome and the point of the key - a failing gate printing `no crate
+# matched advisory criteria` at the exact line is the tool telling you to
+# delete an ignore entry that has done its job, not a regression.
+unused-ignored-advisory = "deny"
```

### Ledger lint

```
$ python3 scripts/ledger-lint.py
EXIT=0
ledger-lint: 566 entries across 4 files plus BUILDING.md's gate enumeration, all invariants hold
```

The full gate was not run; the controller runs it before the push.

## 4. Consistency with the records the brief points at

- `docs/ROADMAP.md`, the trigger entry beginning "`cargo deny check` fails naming
  `RUSTSEC-2024-0429`" - read, not edited. The comment does not restate it and
  does not contradict it: the ROADMAP names the id and the two-step remedy
  (delete the entry, confirm `unsound = "all"` is still set); the comment names
  neither an id nor a remedy step, only the key's effect and how to read its
  failure. The ROADMAP's own measurement line ("an ignored advisory that matches
  no crate in the tree exits 1 and prints the line") is reproduced by Runs 2, 3a
  and 3b above.
- `BUILDING.md`'s Rust gate block - `cargo deny check` appears unchanged at line
  88, and is unchanged by this task. `ledger-lint.py`, which cross-checks the
  gate blocks against BUILDING.md's prose enumeration, is green.

## 5. Commit

`937ae42aeceb280a3e3232cfc322c429881ea65d` on `master`, not pushed.

```
$ git add deny.toml && git -c commit.gpgsign=false commit -F <msg> -- deny.toml
[master 937ae42] deny: make an ignore entry that matches nothing a hard failure
 1 file changed, 11 insertions(+)
EXIT=0
$ git log -1 --format='%G?'
N                       # unsigned, as SI-4 requires
$ git show --stat --format='' HEAD
 deny.toml | 11 +++++++++++
 1 file changed, 11 insertions(+)
$ git status --porcelain
                        # empty; the report file is under .gitignore's
                        # `.superpowers/` and is not part of the commit
```

Exactly one trailer, `Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, no
`Claude-Session` line. The standing grant was verified in the repo's own records
before committing rather than taken from the brief: `docs/process-journal.md:271`
records SI-4 as standing commit/push authorization ("persist indefinitely"), and
`docs/process-conventions.yaml:105` records the unsigned-commit policy and why it
is policy rather than a workaround.

## 6. Concerns for the controller

1. **Run 3a as specified is not a valid demonstration on its own.** "Replace the
   ignored id" also unignores glib, so the run fails for two reasons and its exit
   code proves nothing about the key. Run 3b (add the non-matching id, keep the
   glib entry) is the sound form and is also the more faithful simulation of the
   end state, since a real upstream fix leaves the ignore list untouched. Both are
   above; treat 3b as the answer to the owner's question.
2. **`git diff --exit-code -- deny.toml` cannot prove what the brief wants it to
   prove in a task that edits `deny.toml`.** Replaced with a before/after
   `sha256sum` around the variant runs, which proves the same property. A future
   brief for a config-editing task should specify the hash form.
3. **The key does not make every "pointless ignore" fatal.** An ignored id that
   is not in any advisory database at all (a typo, a withdrawn advisory) stays a
   `warning[unknown-advisory]` at exit 0 - hard-coded severity, outside this
   key's reach. Worth knowing before anyone concludes the ignore list is now
   fully self-policing.
4. **Renovate's first monthly PRs are expected 2026-08-01 to 08-03** per the
   ROADMAP entry. If one of them moves the gtk-rs generation, the very next gate
   run fails with this new error and the correct response is to delete the glib
   ignore entry, not to revert this key. That is the intended behaviour, but it
   will land on whoever pushes next, and the comment is the only thing that will
   tell them so.
