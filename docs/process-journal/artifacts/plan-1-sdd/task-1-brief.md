### Task 1: Workspace scaffold

**Files:**
- Create: `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, `LICENSE`
- Create: `crates/muxsmith-core/Cargo.toml`, `crates/muxsmith-core/src/lib.rs`
- Create: `crates/muxsmith-cli/Cargo.toml`, `crates/muxsmith-cli/src/main.rs`

**Interfaces:**
- Consumes: nothing (first task).
- Produces: compiling workspace; crate names `muxsmith-core` (lib name `muxsmith_core`) and `muxsmith-cli` (binary name `muxsmith`).

- [ ] **Step 1: Create workspace files**

`Cargo.toml`:

```toml
[workspace]
resolver = "2"
members = ["crates/muxsmith-core", "crates/muxsmith-cli"]

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
repository = "https://github.com/senolf/muxsmith"
```

`rust-toolchain.toml`:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

`.gitignore`:

```
/target
```

`LICENSE`: MIT text, `Copyright (c) 2026 Şenol Feldmann` (copy the license body from `~/Git/mkv-batch-tools/LICENSE`, update the year).

`crates/muxsmith-core/Cargo.toml`:

```toml
[package]
name = "muxsmith-core"
version.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
```

`crates/muxsmith-core/src/lib.rs`:

```rust
pub mod report;
```

Create `crates/muxsmith-core/src/report.rs` containing only a placeholder unit test so the workspace compiles:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn workspace_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
```

`crates/muxsmith-cli/Cargo.toml`:

```toml
[package]
name = "muxsmith-cli"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "muxsmith"
path = "src/main.rs"

[dependencies]
muxsmith-core = { path = "../muxsmith-core" }
```

`crates/muxsmith-cli/src/main.rs`:

```rust
fn main() {
    // Subcommands arrive in Task 11.
}
```

- [ ] **Step 2: Verify the workspace builds and tests run**

Run: `cargo test --workspace`
Expected: PASS (1 test: `workspace_compiles`)

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "chore: scaffold cargo workspace (muxsmith-core, muxsmith-cli)"
```

---

