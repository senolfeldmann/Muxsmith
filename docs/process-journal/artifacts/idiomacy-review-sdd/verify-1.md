# Verify F1a: KeepDrop lacks derive(Default) + #[default] Keep

**Verdict: CONFIRMED**

Finding: `crates/muxsmith-core/src/profile/model.rs:183` - `KeepDrop` should derive `Default` with `#[default]` on `Keep`, replacing `fn keep()`, three `#[serde(default = "keep")]` attrs, and the manual `Default` impls for `AttachmentsCfg`/`TagsCfg`.

## (a) Cited code matches the claim

Read at HEAD (`2f17880`):

- `KeepDrop` (model.rs:181-190) derives `Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, JsonSchema` - no `Default`.
- Sibling `CollisionPolicy` in the same file (model.rs:166-177) uses exactly the proposed idiom: `Default` in the derive list, `#[default]` on `Error`, and its consuming field `on_collision` uses plain `#[serde(default)]` (model.rs:105).
- Free `fn keep() -> KeepDrop` exists (model.rs:294-296), consumed by three `#[serde(default = "keep")]` attrs: `AttachmentsCfg.unmatched` (284), `TagsCfg.global` (373), `TagsCfg.track` (376).
- Manual `Default` impls exist for `AttachmentsCfg` (321-328: `Keep` + `Vec::new()`) and `TagsCfg` (380-387: `Keep`/`Keep`) - both exactly what `#[derive(Default)]` produces once `KeepDrop::default() == Keep`.
- The replacement correctly leaves `TracksCfg.unmatched` on `#[serde(default = "drop_policy")]` (309): its default is `Drop`, not `KeepDrop::default()`, and `TracksCfg` has no `Default` impl (required `rules` field).

Note: `FilenameCfg::keep()` (model.rs:157) is a different function, referenced as `#[serde(default = "FilenameCfg::keep")]` (100); deleting the free `fn keep()` does not touch it.

## (b) Replacement is current idiom for the pinned toolchain

Checked against official docs (context7), not training memory:

- rust-lang/rust (E0665 doc + RELEASES.md): `#[derive(Default)]` on enums with a `#[default]` unit variant, stabilized in 1.62.0 - well within pinned Rust 1.96.1 / edition 2024.
- serde.rs field-attrs / attr-default: `#[serde(default)]` uses the field type's `Default::default()` when the field is missing; `#[serde(default = "path")]` is the function-call form. Swapping the three attrs to plain `#[serde(default)]` is semantics-preserving given `KeepDrop::default() == Keep`.
- Empirical in-repo proof: `CollisionPolicy` compiles at HEAD with this exact pattern under the pinned toolchain.

## (c) No load-bearing difference between the duplication sites

- Derived `Default` for `AttachmentsCfg`/`TagsCfg` yields field-for-field the same values as the manual impls (`KeepDrop::Keep`, `Vec::new()`).
- External consumers: only `AttachmentsCfg::default()` / `TagsCfg::default()` calls in `tests/prop_planner.rs:86,88` and `tests/prop_matcher.rs:213,215` - value-identical after the change.
- JSON schema surface (`muxsmith-cli` `schemars::schema_for!(Profile)`, schemars 1.2.1): both `#[serde(default)]` and `#[serde(default = "path")]` embed the serialized default; both forms serialize to `"keep"`, so the emitted schema is unchanged.

## (d) Tag check

Tag is `idiom`, not `yagni`; concrete construct and concrete replacement are named regardless.

## Decision guard

- Grepped `docs/superpowers/specs/*.md`, `docs/IDEAS.md`, `docs/ROADMAP.md` for `KeepDrop`, `fn keep`, `drop_policy`, `derive(Default)`, `#[default]`, `serde(default`: no decision or tracking entry covers this construct.
- Nearest hit: ROADMAP "Cosmetic cleanup, one pass (sweep group K)" mentions model.rs, but only for "TracksCfg placement splitting the AttachmentsCfg group" - a declaration-ordering cosmetic, not the `Default` derive. Not the same finding, so neither DECISION_CONFLICT nor TRACKED.
