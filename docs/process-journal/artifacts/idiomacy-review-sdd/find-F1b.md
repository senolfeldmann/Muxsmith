# Idiomacy review - slice F1b (finder report)

Slice: `crates/muxsmith-core/src/executor/` (queue.rs, job.rs, spawn.rs, joblog.rs, mod.rs), `identify.rs`, `capability/` (runtime.rs, mod.rs, generated.rs), `report/` (mod.rs, json.rs), `command.rs`, `lib.rs`. All 13 files read completely at HEAD. Dimensions hunted: idiom, stdlib, yagni, native (plus same-file dup; cross-file dup excluded per brief).

Ground truth honored: Rust 1.96.1 / edition 2024; CI runs `cargo clippy --workspace --all-targets -- -D warnings`, so everything below passes clippy today. Generated `capability/generated.rs` judged at its generator (xtask), not the output - nothing to say there anyway.

## Findings

### F1b-1 [stdlib] joblog.rs:77 - hand-rolled datetime parse where `time`'s `parsing` feature is the normal inverse

`run_id_timestamp` hand-rolls the inverse of `make_run_id`: 16-byte slicing, a digit-shape closure, positional `-`/`Z` byte checks, six manual `parse::<u32>()` + `try_from` conversions, then `Date::from_calendar_date().with_hms()` for range validation - about 19 lines. The `time` crate (already a dependency, pinned 0.3.53) does exactly this through the same `RUN_ID_FORMAT` descriptor the formatter already uses, gated only on the `parsing` cargo feature.

Verified against the pinned crate's own source (not memory), `~/.cargo/registry/src/.../time-0.3.53`:

- `src/parsing/parsable.rs:29`: `impl Parsable for [BorrowedFormatItem<'_>] {}` - the hand-built `RUN_ID_FORMAT` const is directly parseable, no macro needed.
- `src/primitive_date_time.rs:1094`: `PrimitiveDateTime::parse` exists behind `parsing`.
- `Cargo.toml:72`: `parsing = ["time-macros?/parsing"]` - with `macros` off (as it is here), enabling `parsing` adds **zero new crates**; it is a pure feature of the already-present dependency.

Replacement:

```rust
pub fn run_id_timestamp(name: &str) -> Option<OffsetDateTime> {
    let prefix = name.get(0..16)?;
    PrimitiveDateTime::parse(prefix, RUN_ID_FORMAT)
        .ok()
        .map(PrimitiveDateTime::assume_utc)
}
```

plus `features = ["formatting", "parsing"]` in `crates/muxsmith-core/Cargo.toml`. Rejection behavior is identical to the hand-rolled version (and to the pinned tests in `tests/joblog.rs`): non-digit bytes, wrong literals, month 13, hour 99 all fail the parse - the default zero-padded modifiers require exact width, and calendar/clock range validation happens inside `parse`. The 16-char slice stays, so the collision suffix (`...Z-2`) is still tolerated. Bonus: format and parse can no longer drift apart, because both consume the one `RUN_ID_FORMAT` descriptor.

- lines_cut: 14 (19-line body -> ~4 lines; `all_digits`/`n` closures and six conversions gone)
- deps_cut: 0

### F1b-2 [dup] capability/mod.rs:126 - `CODEC_KIND_NAMES` hand-maintained duplicate of `CODEC_KINDS` keys

`CODEC_KIND_NAMES` re-lists all 17 keys of `CODEC_KINDS` by hand, kept honest only by the `codec_kind_domain_matches_kinds` sync test (mod.rs:220). Same-file duplication with a maintenance step every time a codec kind is added. The Rust-1.96-normal way to derive a `'static` view once is `std::sync::LazyLock` (std since 1.80):

```rust
pub static CODEC_KIND_NAMES: LazyLock<Vec<&'static str>> =
    LazyLock::new(|| CODEC_KINDS.iter().map(|(k, _)| *k).collect());
```

`matchable_domain` keeps returning `Some(&CODEC_KIND_NAMES)` (deref coercion `&'static LazyLock<Vec<&str>>` -> `&'static [&str]`), and the one external consumer (`tests/prop_matcher.rs:143`, `CODEC_KIND_NAMES.to_vec()`) compiles unchanged through deref. The sync test becomes meaningless and is deleted. Public type changes from `&[&str]` to `LazyLock<Vec<&str>>`; crate is `publish = false`, both consumers verified.

- lines_cut: 7 (4-line hand list + 5-line sync test, minus ~2 lines LazyLock)
- deps_cut: 0

### F1b-3 [idiom] queue.rs:335 - manual clamp instead of `usize::clamp`

`worker_count` computes `jobs.max(1).min(spec_count.max(1))`. The stdlib expression for exactly this is `jobs.clamp(1, spec_count.max(1))` (stable since 1.50); clippy's `manual_clamp` exists for this pattern and stays silent here only because the upper bound is non-const. Panic precondition holds trivially (`spec_count.max(1) >= 1`). The existing rustdoc and `worker_count_is_capped_at_spec_count` test stay as-is.

- lines_cut: 0 (pure replacement)
- deps_cut: 0

## Routed (out-of-scope observations, not findings)

1. **command.rs:99-100 (and every `display().to_string()` in `push_global`/`push_group`)** - correctness: paths are rendered into argv via `Path::display().to_string()`, which is lossy for non-UTF-8 paths (U+FFFD substitution), so a non-UTF-8 output dir or donor path would silently corrupt the mkvmerge command instead of failing. The identify path already treats this correctly and explicitly (`runtime.rs:206` rejects a non-UTF-8 path with a `Parse` error before spawning). Edge case, partially shielded because primaries/donors pass through `identify_json` first, but the output path does not. Correctness dimension - routed per brief.

## Considered and not flagged (with reasons)

- **joblog.rs:32 `RUN_ID_FORMAT` hand-built `BorrowedFormatItem` const** (vs `format_description!` macro): the macro needs the `macros` feature, which *does* pull in the `time-macros` proc-macro crate - a real new build dependency for 14 declarative, correct lines. The in-code comment documents the feature-minimalism deliberately, and F1b-1 reuses the const as-is. Dependency not earned.
- **report/mod.rs:30 `diag_codes!` macro** (vs strum `VariantArray` + `IntoStaticStr`): strum is not in the workspace; the macro is ~30 lines, forwards doc attributes, and serves exactly one enum. The key()/serde sync test would be needed under strum too (two derive systems sharing a rename convention). Dependency not earned for one enum.
- **identify.rs manual `serde_json::Value` walking** (vs derived `Deserialize` structs): the tolerance policy (missing sections default, wrong-typed attachments dropped per-entry via `filter_map`, non-scalar properties silently skipped) is what derive is bad at; a derive-based version needs custom `deserialize_with` at every lenient point and ends up the same size. Value-walking is the right tool for foreign JSON with per-entry tolerance.
- **`QueueControl::new` / `ConcurrencyTracker::new` returning `Arc<Self>`**: deviates from the `new() -> Self` convention, but both types are only ever meaningful shared, and both call sites (CLI run.rs:214, src-tauri run.rs:329) want the Arc. Deliberate, harmless.
- **`Spawn` / `Identify` / `DiagnosticRenderer` traits**: all have >= 2 implementations (live + fakes; CLI Renderer at `crates/muxsmith-cli/src/i18n.rs:175` + `ShellRenderer` at `src-tauri/src/lib.rs:56`). No single-impl yagni anywhere in the slice.
- **`FakeSpawner` pub in production module**: consumed cross-crate (core integration tests, src-tauri tests), so `#[cfg(test)]` is impossible; mirrors the documented `ConcurrencyTracker` pattern.
- **spawn.rs `read_next_line` hand-rolled over `BufRead::lines()`**: deliberate and documented - `lines()` errors on invalid UTF-8; the lossy `read_until` loop is the correct tool and unit-tested for exactly that (#9).
- **Watcher thread with 50ms poll** (vs condvar/channel): `Arc<AtomicBool>` shared with the ctrlc handler is the ctrlc-ecosystem norm; documented D16 design.
- **`known_extensions: Option<Option<Vec<String>>>`** (vs `OnceCell`): `Option::get_or_insert_with` on a `&mut self` method is the idiomatic memoization here; `OnceCell` buys nothing.
- **Known non-findings honored**: version pins, MUXSMITH_RUNS_ROOT debug-only (D26), fake-mkvmerge helper copies, RECENT_PROFILES_CAP - none re-raised.

## Slice verdict

Not clean: 3 findings (1 stdlib, 1 dup, 1 idiom), all small and pure-local; 1 routed correctness note. The slice is in very good idiomatic shape overall - the deliberate hand-rolls (lossy line reader, Value-walking, poison recovery) are documented and correct choices, not accidents.
