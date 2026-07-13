# Verify-10: planner.rs Ctx-build duplication (render_output / resolve_title)

**Verdict: CONFIRMED**

## Finding under test

`render_output` (planner.rs:886ff) and `resolve_title` (planner.rs:971ff) duplicate the identical Ctx build; extract a helper so the comment-enforced lockstep invariant is enforced by construction. Tag: dup, slice F1a.

## Checks

### (a) Cited code says what the finding claims — PASS

Both sites contain the byte-identical block:

```rust
let mut ctx = primary.identifier.to_ctx();
if let Some(stem) = primary.path.file_stem().and_then(|s| s.to_str()) {
    ctx.set("source_stem", stem);
}
```

- render_output: planner.rs:887-890 (inside the `FilenameCfg::Template` arm at :886).
- resolve_title: planner.rs:971-974 (inside the `TitleCfg::Template` arm).

The lockstep demand is real and explicit, planner.rs:959-963: "The Ctx mirrors `render_output`'s exactly, including `source_stem`: validate.rs allows `source_stem` in `title.template` identically to `output.filename.template` ... so the two templates' available fields must stay in lockstep."

### (b) Replacement is current idiom — PASS

Extracting a private `fn` for a duplicated block whose sameness is a documented invariant is language-fundamental Rust, not a toolchain-version-sensitive construct: no new API, no edition-2024 feature, nothing rustc 1.96.1 docs would qualify. There is no version-dependent claim here to check against docs; the (b) refutation channel does not apply. `PrimaryFile` carries both `identifier` and `path`, so `fn render_ctx(primary: &PrimaryFile) -> Ctx` (or a `PrimaryFile` method) is directly implementable.

### (c) Load-bearing difference between the sites — NONE within extraction scope

- The Ctx-build blocks themselves are identical.
- The sites differ only downstream of the build (render_output: `parse().map(render_literal).unwrap_or_default()` plus filename invariants; resolve_title: `Ok/Err -> Set/Keep`), all of which stays outside the proposed helper.
- A third `to_ctx()` site exists (discovery.rs:161, match_pattern regex context) and deliberately does NOT set `source_stem` — validate.rs:456-457 confirms `source_stem` is literal-mode only. That site is correctly excluded from the extraction; its difference is load-bearing and untouched. This strengthens the finding: the helper cleanly scopes to exactly the two literal-mode sites the comment binds together.

### (d) yagni construct/replacement named — N/A

Tag is `dup`, not `yagni`.

### Decision guard — NO HIT

Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `render_output`, `resolve_title`, `source_stem`, `to_ctx`, `render_ctx`, `lockstep`, plus a broader `planner`/`ctx`/`dedup` sweep:

- Only spec mention of `source_stem` is the field definition (2026-07-08-muxsmith-v1-design.md:216), not a decision about the two build sites.
- ROADMAP planner.rs entries concern different constructs (`allowed` param prose at :428/:841; discarded-plan path :541ff). Nothing in cosmetic-cleanup group K or the deferred entries covers this duplication.
- The "must not share one conditional" comment at planner.rs:911 governs the keep-vs-template `.mkv`-append conditional, a different construct; the Ctx extraction does not touch it.

Not tracked, not decided against.

## Conclusion

The duplication is real, the lockstep invariant is currently enforced only by a comment, the proposed extraction is idiomatic and feasible, and no recorded decision or tracking entry covers it. CONFIRMED.
