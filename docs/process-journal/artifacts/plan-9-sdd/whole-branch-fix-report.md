# Whole-branch fix report - Plan 9, finding 1 (MEDIUM)

Two rustdoc edits in `crates/muxsmith-core/src/identify.rs`. No code, no test,
no other file touched. Commit `96dbcf6`, not pushed.

## The spec passage both docs were checked against

Read at the source, `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:312`
(amended by S-8):

> Identification cache: in-memory, keyed on path + mtime + size, constructed per
> planning call and dropped with it. One call identifies each unchanged file once
> (run plans and executes within a single call, so its planning pass and the
> suggestion engine's re-simulations share one cache); separate calls
> re-identify, so a GUI dry-run followed by a run spawns `mkvmerge -J` per file
> in each (a per-session shared cache was ruled out 2026-07-28 as unnecessary).
> In the CLI, call and process coincide. On-disk cache is a future candidate.

And the code the wording now describes, `crates/muxsmith-core/src/pipeline.rs`:
`plan_pipeline` constructs `IdentifyCache::new()` at line 127 inside its own
body and hands it to a stack-local `LiveIdentifier`; nothing outside the call
holds it, so it drops when the call returns. Its own doc (pipeline.rs:93-96)
already states this correctly. The struct is a plain
`HashMap<PathBuf, (CacheKey, Identification)>` with no persistence.

## Edit 1 - module doc, `identify.rs:1-6`

Before:

```rust
//! Source-file identification via `mkvmerge -J` (spec 5.5, 9). Wraps the
//! external process, parses its JSON into a track/attachment/chapter model,
//! and caches results in memory keyed on path + mtime + size so dry-run and
//! run never re-identify an unchanged file (spec 5.5).
```

After:

```rust
//! Source-file identification via `mkvmerge -J` (spec 5.5, 9). Wraps the
//! external process, parses its JSON into a track/attachment/chapter model,
//! and caches results in memory keyed on path + mtime + size, so one planning
//! call never re-identifies an unchanged file (spec 5.5). The cache is
//! constructed per planning call and dropped with it, so separate calls
//! re-identify.
```

The effect is scoped to one planning call, and the second sentence names the
lifetime explicitly rather than leaving the reader to infer that a later call
reuses it. Key (path + mtime + size) and the process-wrapping/parsing content
kept verbatim.

## Edit 2 - `IdentifyCache` doc, `identify.rs:304-307`

Before:

```rust
/// In-memory identification cache for one session (spec 5.5). Keyed on path
/// plus (mtime, size); a changed file re-identifies, so a dry run is never
/// stale. On-disk caching is a future candidate (spec non-goals).
```

After:

```rust
/// In-memory identification cache, constructed per planning call and dropped
/// with it (spec 5.5). Keyed on path plus (mtime, size); a changed file
/// re-identifies, so a dry run is never stale. On-disk caching is a future
/// candidate (spec non-goals).
```

"for one session" replaced by the actual lifetime. Key, the
changed-file-re-identifies consequence, and the on-disk-cache-future-candidate
note kept verbatim.

## Same-class sweep of `identify.rs`: nothing else

Method, three passes, each shown to fire rather than trusted as an empty result.

**Pass A - completeness of the doc-comment enumeration.** `grep -cE
'^[[:space:]]*//[/!]'` gives 97; `grep -c -- '///\|//!'` over the same file also
gives 97. The two counts agreeing establishes that every doc marker in the file
sits at line start, so a line-start enumeration misses no doc comment. (Stated
as a positive count match, not as "the mid-line query returned nothing".)

**Pass B - concept sweep, whole file, case-insensitive**, not restricted to doc
comments:
`grep -niE 'cach|session|dry.?run|re-?identif|persist|lifetime|memoi|reuse|stale|shared|scope|drop|per[- ]|once|later|subsequent|across|process'`.
It returned 30 lines and included both sites I had just edited plus every
cache-touching doc block, which is the demonstration that it fires. Each
lifetime- or cache-touching hit judged:

| Site | Text | Verdict |
|---|---|---|
| 1-6 | module doc | fixed above |
| 263 | "could not be stat'd for the cache key" | no lifetime claim |
| 304-307 | `IdentifyCache` doc | fixed above |
| 311-314 | `known_extensions` field: "at most once per `plan_core` call ... mirroring `entries`' per-file memoization at the batch scope" | true, scoped to a call; no cross-call promise |
| 325 | "A fresh, empty cache." | true |
| 330-332 | `get_or_identify`: "Borrows the cached value for the caller's lifetime" | "lifetime" here is the Rust borrow lifetime, not the cache's; true |
| 348-351 | `known_extensions` method: "queried at most once per cache instance and memoized ... repeat calls (e.g. across the suggestion engine's re-simulated `plan_core` passes) never respawn mkvmerge" | true and exactly right post-amendment: the instance IS the planning call, and the re-simulations named are the in-call sharing the spec describes |
| 373-375 | `Identify` trait: "a clone from the cache in the live impl" | no lifetime claim |
| 391-392 | `LiveIdentifier::cache`: "constructed per planning call and dropped with it" | already correct - this is the third site Task 1's literal sweep caught |

**Pass C - paraphrase angle**, aimed at the failure mode that produced this
finding (the claim restated without the words "session" or "cache"):
`grep -niE 'gui|surface|surviv|again|another call|next call|second|warm|both |re-?use|carry over|between'`.
Three hits, all unrelated ("against the pinned ...", "defensive against more",
"tested against fixture data"). The pattern produced output, so it ran.

Conclusion for `identify.rs`: **nothing else**. The file's remaining
cache-related docs were already per-call or lifetime-neutral.

## Found outside this file (reported, not in the diff)

**One live artifact still carries the falsified per-session claim:
`docs/conventions.yaml`, the house-knowledge ledger, entry `core-20-ondisk-cache`
(lines 937-951).**

```yaml
  statement: "The identification cache is in-memory per session (keyed path+mtime+size); an on-disk cache is a future candidate, not built in v1."
  blocked_on: "in-memory per-session cache suffices for v1, no measured need"
```

Both `statement` and `blocked_on` assert the per-session lifetime S-8 removed.
The entry's real subject (on-disk cache deferred) survives the amendment; only
its lifetime premise is now wrong. Aggravating: `docs/ROADMAP.md:362-364` names a
ledger entry `gui-identification-cache-per-call-not-per-session` as the carrier
of the accepted cost, and that id does not exist in `conventions.yaml`
(`grep -c` returns 0, against a control `grep -c core-20-ondisk-cache` returning
1 on the same file). So the ledger currently states the ruled-out lifetime and
does not yet state the ruling. This is a ledger-hygiene item for the controller,
outside my two-edit scope.

Everything else outside `identify.rs` is either correct or deliberately
historical:

- `docs/superpowers/specs/2026-07-08-muxsmith-v1-design.md:312` - the amended
  spec, correct.
- `docs/superpowers/specs/2026-07-28-plan9-core-hoists-planner-seam-design.md`
  (384-404, 1494, 1500, 1787) - the amendment record; 1494 quotes the old text
  as the "before" of the amendment, 1500 the "after". Intentional.
- `docs/superpowers/plans/2026-07-09-plan-2-identify-matcher-planner.md`
  (931, 1081, 1261, 2126) - Plan 2's plan text, the origin of both fixed rustdoc
  strings. A closed historical plan; amending it would rewrite history.
- `docs/process-journal/artifacts/handoffs/2026-07-09-plan-1-close.md:43`,
  `.../house-backfill-sdd/find-E1.md:183`, `.../cluster-core.md:333`,
  `.../plan-6-sdd/blocked-pool-sweep-input.md:29` - dated journal/backfill
  artifacts, historical by construction. `cluster-core.md:333` is the backfill
  input `conventions.yaml`'s entry was built from, which is how the stale
  premise got into the live ledger.
- `crates/muxsmith-core/src/report/json.rs:92` ("Consumed by both dry-run and run
  for their identical mkvmerge-missing / query-failed / profile-load-failure
  paths") - a statement about which code paths consume one document builder, not
  about cache lifetime. Not this class, true as written.
- `src/recentProfiles.ts:15` ("within one session") - the MRU profile list, a
  different subject. Not this class.
- The many `per session` hits in handoffs are the SI-4 commit-authorization
  grant, an unrelated sense of the word.

## Verification (pasted)

```
$ cargo fmt --all --check
FMT_OK

$ RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
DOC_EXIT=0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
   Generated /home/senol/Git/Muxsmith/target/doc/muxsmith_cli/index.html and 5 other files
=== warnings in log? ===
0

$ cargo clippy --workspace --all-targets -- -D warnings
CLIPPY_EXIT=0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.10s

$ cargo test --workspace
TEST_EXIT=0
=== test result: lines ===
39
=== non-ok result lines ===
0
```

39 `test result:` lines, all ok - matches the brief's expected 39 exactly.

Propagation into the built rustdoc (the artifact a reader actually sees), each
negative paired with a control that fires:

```
$ grep -o "one planning call never re-identifies an unchanged file" target/doc/muxsmith_core/identify/index.html
one planning call never re-identifies an unchanged file
$ grep -c "dry-run and\s*run never re-identify\|for one session" target/doc/muxsmith_core/identify/index.html
0
$ grep -c "identification cache" target/doc/muxsmith_core/identify/index.html   # CONTROL
1

$ grep -o "constructed per planning call and dropped with it" target/doc/muxsmith_core/identify/struct.IdentifyCache.html
constructed per planning call and dropped with it
$ grep -c "for one session" target/doc/muxsmith_core/identify/struct.IdentifyCache.html
0
$ grep -c "On-disk caching is a future candidate" target/doc/muxsmith_core/identify/struct.IdentifyCache.html   # CONTROL
1
```

No frontend leg: no `src/` or `e2e/` file touched (`git status` shows the single
Rust file).

**The check that cannot be fired.** No test asserts these doc strings, which is
why the defect reached a whole-branch review. I verified that rather than
assuming it: `grep -rniE "for one session|never re-?identif|constructed per
planning call" crates/ src-tauri/` returns four lines, all of them `//!`/`///`
doc comments in `identify.rs` itself (lines 4, 5, 304, 391) and no `assert`. So
the passing test suite is evidence that I broke nothing, not evidence that the
new wording is right; the correctness argument for the wording is the spec
passage and `pipeline.rs`'s construction site quoted above, both read at the
source.

## Commit

`96dbcf6` - `core: correct two identify.rs cache docs falsified by the S-8 amendment`

```
 crates/muxsmith-core/src/identify.rs | 13 ++++++++-----
 1 file changed, 8 insertions(+), 5 deletions(-)
```

One file staged by name, `git -c commit.gpgsign=false`, exactly one trailer
(`Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>`, confirmed via
`git log -1 --format='%(trailers)'`), no `Claude-Session` line. Not pushed.
Working tree clean afterwards; nothing was mutated to fire a check, so there was
nothing to restore.
