//! Spec 10: every DiagCode must have a message template in the English
//! catalog. Renderer::msg falls back to the raw id when a message is
//! missing, so key != rendered output proves the entry exists.

use muxsmith_core::report::DiagCode;

#[test]
fn every_diag_code_has_a_catalog_message() {
    let renderer = muxsmith_cli::i18n::Renderer::new(Some("en"));
    let missing: Vec<&str> = DiagCode::ALL
        .iter()
        .filter(|code| renderer.msg(code.key(), &[]) == code.key())
        .map(|code| code.key())
        .collect();
    assert_eq!(missing, Vec::<&str>::new(), "missing catalog entries");
}
