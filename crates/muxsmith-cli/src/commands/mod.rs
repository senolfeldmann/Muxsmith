//! One module per CLI subcommand (spec 8.1); each wraps a core operation
//! and renders its diagnostics via [`crate::i18n::Renderer`].

pub mod dry_run;
pub mod identify;
pub mod validate;
