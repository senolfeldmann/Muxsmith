### Task 1: Core src, non-planner (Stream A)

**Files:**
- Modify: `crates/muxsmith-core/src/profile/model.rs`, `crates/muxsmith-core/src/profile/validate.rs`, `crates/muxsmith-core/src/executor/joblog.rs`, `crates/muxsmith-core/Cargo.toml`, `crates/muxsmith-core/src/discovery.rs`, `crates/muxsmith-core/src/capability/mod.rs`, `crates/muxsmith-core/src/template.rs`, `crates/muxsmith-core/src/executor/queue.rs`
- Do NOT touch `planner.rs` (its one-line twin of the discovery item belongs to T2).

**Interfaces:** none new; all internal. Existing tests must pass unchanged (that is the mechanicality criterion).

- [ ] `profile/model.rs:183` **idiom** - KeepDrop gains `#[derive(Default)]` + `#[default] Keep` (sibling CollisionPolicy is the in-file precedent); the three `#[serde(default = "keep")]` become plain `#[serde(default)]`; delete `fn keep()` and the manual `Default` impls for AttachmentsCfg/TagsCfg (add `Default` to their derives). TracksCfg keeps its explicit `default = "drop_policy"`.
- [ ] `profile/validate.rs:280` **dup** - the 8-line InvalidRegex compile-check block repeated three times in validate_expr's substring/regex loop (raw:, codec_kind, fallthrough branches) becomes an if / else-if / else chain for the property-level diagnostic plus ONE regex compile check at loop end. Diagnostic order preserved exactly (regex diag is last in every current path) - reviewer checks order-sensitive tests.
- [ ] `executor/joblog.rs:77` **stdlib** - run_id_timestamp's ~19 hand-rolled parse lines become: add `"parsing"` to the `time` features in core Cargo.toml (zero new crates, verified in pinned time-0.3.53); body = `let prefix = name.get(0..16)?; PrimitiveDateTime::parse(prefix, RUN_ID_FORMAT).ok().map(PrimitiveDateTime::assume_utc)`. Rejection behavior identical; existing tests/joblog.rs cases all pass; format/parse can no longer drift (one descriptor).
- [ ] `discovery.rs:187` **stdlib** - extension_matches uses `exts.iter().any(|x| x.eq_ignore_ascii_case(e))`; delete the pre-lowered `Vec<String>` collects in both callers (scan_primaries:61, resolve_locator:147), passing `&input.extensions` / `&locator.extensions` directly.
- [ ] `discovery.rs:76` **idiom** - scan_primaries' three regex passes (find_iter, find_iter.next, captures + expect) become one `captures_iter` pass: first captures via `it.next()`, multiplicity via `it.next().is_some()`, whole match via `&caps[0]`; the cross-call expect invariant disappears.
- [ ] `capability/mod.rs:126` **dup** - CODEC_KIND_NAMES stops hand-re-listing all 17 keys: `pub static CODEC_KIND_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| CODEC_KINDS.iter().map(|(k, _)| *k).collect());` (std::sync::LazyLock). matchable_domain keeps returning `Some(&CODEC_KIND_NAMES)` via deref coercion; tests/prop_matcher.rs:143's `.to_vec()` compiles unchanged. Delete the now-meaningless sync test at mod.rs:220.
- [ ] `template.rs:92` **idiom** - Template::parse drops the Vec<char> index-walk for `Peekable<Chars>` with `peek()` lookahead and a consume-until-`}` scan. The documented char-offset `pos` contract on TemplateError stays intact (journal-recorded).
- [ ] `executor/queue.rs:335` **idiom** - `jobs.max(1).min(spec_count.max(1))` becomes `jobs.clamp(1, spec_count.max(1))`; panic precondition trivially holds. Rustdoc and the worker_count_is_capped_at_spec_count test unchanged.
- [ ] Full nine-part gate; commit per logical group `refactor(core): ...`.

