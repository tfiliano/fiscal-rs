//! End-to-end gate: build a CT-e Normal, sign it, and validate the signed
//! document against the **official CT-e 4.00 XSD** (via `fiscal-xsd`).
//!
//! This is the real proof that the `builder.rs` block ordering matches the
//! schema — a misplaced element fails XSD validation here, before any
//! transmission. The certificate CNPJ need not match `emit/CNPJ`: XSD checks
//! structure only, not business rules.

mod common;

use fiscal_crypto::certificate::load_certificate;
use fiscal_cte::{build_cte_xml, sign_cte_xml};

fn test_pfx() -> Vec<u8> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../..",
        "/tests/fixtures/certs/novo_cert_cnpj_06157250000116_senha_minhasenha.pfx"
    );
    std::fs::read(path).expect("test PFX not found")
}

#[test]
fn signed_cte_validates_against_official_xsd() {
    let xml = build_cte_xml(&common::sample_cte()).unwrap();

    let cert = load_certificate(&test_pfx(), "minhasenha").expect("load cert");
    let signed = sign_cte_xml(&xml, &cert.private_key, &cert.certificate).expect("sign");

    if let Err(errs) = fiscal_xsd::schemas::cte().validate(&signed) {
        panic!(
            "CT-e assinado falhou no XSD oficial 4.00:\n{}",
            errs.join("\n")
        );
    }
}
