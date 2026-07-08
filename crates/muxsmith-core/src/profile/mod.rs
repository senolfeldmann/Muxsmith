//! Profile parsing and semantic layer (spec 4): [`load`] parses YAML/JSON
//! into the serde model, [`model`] mirrors the file format 1:1, [`match_expr`]
//! is the match algebra (4.3), [`validate`] runs config-time semantic checks
//! (5.4), and [`lint`] adds the provable-overlap static check.

pub mod lint;
pub mod load;
pub mod match_expr;
pub mod model;
pub mod validate;

pub use model::Profile;
