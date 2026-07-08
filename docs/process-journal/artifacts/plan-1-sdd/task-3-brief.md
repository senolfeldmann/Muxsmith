### Task 3: Match expression model

**Files:**
- Create: `crates/muxsmith-core/src/profile/mod.rs`
- Create: `crates/muxsmith-core/src/profile/match_expr.rs`
- Modify: `crates/muxsmith-core/src/lib.rs`

**Interfaces:**
- Consumes: nothing from earlier tasks.
- Produces:
  - `profile::match_expr::Scalar` (untagged: `Bool(bool) | Int(i64) | Float(f64) | Str(String)`) with `fn type_name(&self) -> &'static str` (`"boolean" | "integer" | "float" | "string"`)
  - `profile::match_expr::MatchExpr { exact: Option<BTreeMap<String, Scalar>>, substring: Option<BTreeMap<String, String>>, regex: Option<BTreeMap<String, String>>, any: Option<Vec<MatchExpr>>, not: Option<Vec<MatchExpr>> }` with `fn is_empty(&self) -> bool`
  - Both derive `Deserialize, Serialize, JsonSchema, Debug, Clone, PartialEq`; `MatchExpr` is `deny_unknown_fields`.

- [ ] **Step 1: Add dependencies**

```bash
cargo add -p muxsmith-core schemars
cargo add -p muxsmith-core --dev yaml-serde
```

(If `cargo add yaml-serde` fails because the crates.io name differs, check https://github.com/yaml/yaml-serde for the published crate name and use that; it is the YAML org's maintained fork of serde-yaml. Import path in code below assumes `yaml_serde`.)

- [ ] **Step 2: Write the failing tests**

`crates/muxsmith-core/src/profile/match_expr.rs`, tests at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_flat_expression() {
        let y = r#"
exact: { type: subtitles, forced_track: true }
substring: { track_name: SDH }
"#;
        let e: MatchExpr = yaml_serde::from_str(y).unwrap();
        assert_eq!(
            e.exact.as_ref().unwrap()["type"],
            Scalar::Str("subtitles".into())
        );
        assert_eq!(e.exact.as_ref().unwrap()["forced_track"], Scalar::Bool(true));
        assert_eq!(e.substring.as_ref().unwrap()["track_name"], "SDH");
        assert!(e.any.is_none() && e.not.is_none());
        assert!(!e.is_empty());
    }

    #[test]
    fn parses_nested_any_and_not() {
        let y = r#"
exact: { type: subtitles }
not:
  - substring: { track_name: SDH }
  - exact: { flag_hearing_impaired: true }
any:
  - regex: { track_name: '(?i)forced' }
"#;
        let e: MatchExpr = yaml_serde::from_str(y).unwrap();
        assert_eq!(e.not.as_ref().unwrap().len(), 2);
        assert_eq!(e.any.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn rejects_unknown_keys() {
        let y = "exactt: { type: video }";
        assert!(yaml_serde::from_str::<MatchExpr>(y).is_err());
    }

    #[test]
    fn empty_expression_reports_empty() {
        let e: MatchExpr = yaml_serde::from_str("{}").unwrap();
        assert!(e.is_empty());
    }

    #[test]
    fn scalar_type_names() {
        assert_eq!(Scalar::Bool(true).type_name(), "boolean");
        assert_eq!(Scalar::Int(3).type_name(), "integer");
        assert_eq!(Scalar::Float(1.5).type_name(), "float");
        assert_eq!(Scalar::Str("x".into()).type_name(), "string");
    }
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p muxsmith-core match_expr`
Expected: FAIL (types not defined)

- [ ] **Step 4: Implement**

`crates/muxsmith-core/src/profile/match_expr.rs` above the tests:

```rust
//! Match algebra (spec 4.3): conjunction of exact/substring/regex maps
//! plus recursive `any` (at least one holds) and `not` (none hold).

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Scalar {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

impl Scalar {
    pub fn type_name(&self) -> &'static str {
        match self {
            Scalar::Bool(_) => "boolean",
            Scalar::Int(_) => "integer",
            Scalar::Float(_) => "float",
            Scalar::Str(_) => "string",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct MatchExpr {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact: Option<BTreeMap<String, Scalar>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substring: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any: Option<Vec<MatchExpr>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Vec<MatchExpr>>,
}

impl MatchExpr {
    pub fn is_empty(&self) -> bool {
        self.exact.as_ref().is_none_or(|m| m.is_empty())
            && self.substring.as_ref().is_none_or(|m| m.is_empty())
            && self.regex.as_ref().is_none_or(|m| m.is_empty())
            && self.any.as_ref().is_none_or(|v| v.is_empty())
            && self.not.as_ref().is_none_or(|v| v.is_empty())
    }
}
```

`crates/muxsmith-core/src/profile/mod.rs`:

```rust
pub mod match_expr;
```

`crates/muxsmith-core/src/lib.rs`:

```rust
pub mod profile;
pub mod report;
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p muxsmith-core match_expr`
Expected: PASS (5 tests)

- [ ] **Step 6: Commit**

```bash
git add -A
git commit -m "feat(core): match expression model with recursive any/not"
```

---

