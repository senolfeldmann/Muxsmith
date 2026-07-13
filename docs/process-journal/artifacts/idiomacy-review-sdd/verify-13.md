# Verify-13: joblog.rs run_id_timestamp hand-rolled parse (slice F1b)

**Verdict: CONFIRMED**

Finding: `crates/muxsmith-core/src/executor/joblog.rs:77` hand-rolls the inverse of `make_run_id` although the pinned `time` crate parses via the same `RUN_ID_FORMAT` descriptor; proposed replacement enables the `parsing` feature and reduces the body to `name.get(0..16)?` + `PrimitiveDateTime::parse(prefix, RUN_ID_FORMAT)`.

## (a) Cited code says what the finding claims - YES

`run_id_timestamp` (joblog.rs:77-95) is exactly as described: byte-wise digit-shape checks with positional `-`/`Z` literal checks (line 81), six manual `parse::<u32>()` + `try_from` conversions (lines 84-90), then `Date::from_calendar_date` + `with_hms` + `assume_utc`. The formatting direction (`make_run_id`, line 59) already goes through `RUN_ID_FORMAT` (lines 32-45), so format and parse currently use two independent encodings of the same shape.

## (b) Replacement is current idiom for the pinned toolchain - YES

Verified against the pinned source at `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/time-0.3.53` (Cargo.lock pins time 0.3.53, checksum matches):

- `impl Parsable for [BorrowedFormatItem<'_>]` exists at `src/parsing/parsable.rs:29`, so the existing `RUN_ID_FORMAT` const is directly parseable - no macro, no runtime string parsing needed.
- `PrimitiveDateTime::parse` exists at `src/primitive_date_time.rs:1094`; its own doc example demonstrates exactly the proposed pattern (parse with a format description).
- Feature definition in time's Cargo.toml line 72: `parsing = ["time-macros?/parsing"]`. With `macros` not enabled the `?` clause is inert. **Empirically confirmed with `cargo tree` on a scratch project: the dependency set with `["formatting"]` and `["formatting", "parsing"]` is byte-identical** (deranged, num-conv, powerfmt, time-core, time). Zero new crates, matching the finding's deps_cut=0.

## (c) Load-bearing behavioral difference - NONE (empirically verified)

Scratch project pinning `time =0.3.53`, comparing the verbatim current implementation against the verbatim proposed body:

- All existing `tests/joblog.rs` cases (round-trip of `make_run_id` output, collision suffix `...Z-2`, `"not-a-run-id"`, `""`, `"short"`, `"20260113-999999Z"`, `"20261399-120000Z"`): identical results, positive cases parse to the same instant.
- Adversarial edges: month 00/13, day 00, invalid vs valid leap day, hour 24, minute 60, **second 60 (leap second: both reject** - `Parsed`-to-`Time` conversion validates ranges just like `with_hms`), year 0000 and 9999, signed years, leading space, lowercase `z`, wrong separator, trailing bytes after a valid prefix, 15-byte input, fullwidth Unicode digit, prefix cut mid-UTF-8-char: **zero mismatches**.
- Fuzz: 2,000,000 random 16-byte strings from a run-id-ish alphabet, a full per-position mutation sweep of a valid id (16 positions x 19 bytes), and exhaustive 0-99 sweeps of month/day/hour/minute/second: **zero mismatches**.

One theoretical wrinkle checked explicitly: `parse_calendar_year_full_standard_range` (parsing/component.rs:50ff) accepts an optional sign, which the hand-rolled digit-shape check rejects. Not load-bearing: a signed year makes the minimum match 17 bytes, but the parse input is the fixed 16-byte prefix, so a signed input can never satisfy the remaining fixed-width components and literals. Confirmed empirically (signed inputs in the edge set and mutation sweep, all rejected by both).

All non-year components use `exactly_n_digits_padded::<2>` with default `Padding::Zero`, i.e. exact-width ASCII digits - same strictness as the manual check.

## (d) Not applicable

Tag is `stdlib`, not `yagni`; concrete construct and concrete replacement are both named.

## Decision guard - no conflict, not tracked

- **D35** (pre-1.0 design-decisions memo, the decision that created this function) explicitly leaves the mechanics open: "The exact trigger point ... and the age source (run-id timestamp vs dir mtime) are design details of the implementing task." No decision on how the parse is implemented.
- Grepped all specs (D1-D35), `docs/IDEAS.md` (incl. #7, which is only about prune configurability), `docs/ROADMAP.md` (v1.x candidates, cosmetic-cleanup group K, test-hygiene collection, Plan-5 residues): no entry tracks or forbids this refactor. The ROADMAP "Whole-codebase idiomacy review" entry is the review this finding belongs to; its named-inputs list does not include this construct.
- The in-code comment (joblog.rs:27-31, "since only the `formatting` cargo feature is pinned (not `macros` or `parsing`)") describes the current constraint; it is a consequence of the feature set, not a recorded decision against enabling `parsing`. It must be updated as part of the fix (its "not parsing" rationale becomes false), as must `run_id_timestamp`'s doc sentence about the digit-shape check - cosmetic riders, no refutation.

## Net effect check

Body shrinks from ~18 lines (78-94) to 2; the finding's `lines_cut: 14` is conservative and plausible after comment adjustments. Drift-proofing claim holds: format and parse then consume the single `RUN_ID_FORMAT` descriptor.
