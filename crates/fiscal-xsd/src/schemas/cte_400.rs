//! Embedded official CT-e 4.00 schemas (PL_CTe_400).
//!
//! Root `cte_v4.00.xsd` pulls in `cteTiposBasico` → `tiposGeralCTe` +
//! `xmldsig-core`. The modal subtree (`rodo`/`aereo`/…) is declared as
//! `<xs:any processContents="skip">`, so it is **not** validated here and the
//! modal schemas are not part of the bundle. Validates a **signed** `<CTe>`
//! document (the schema requires the enveloped `ds:Signature`).

use crate::XsdSchema;

static FILES: &[(&str, &[u8])] = &[
    (
        "cte_v4.00.xsd",
        include_bytes!("../../schemas/cte_400/cte_v4.00.xsd"),
    ),
    (
        "cteTiposBasico_v4.00.xsd",
        include_bytes!("../../schemas/cte_400/cteTiposBasico_v4.00.xsd"),
    ),
    (
        "tiposGeralCTe_v4.00.xsd",
        include_bytes!("../../schemas/cte_400/tiposGeralCTe_v4.00.xsd"),
    ),
    (
        "xmldsig-core-schema_v1.01.xsd",
        include_bytes!("../../schemas/cte_400/xmldsig-core-schema_v1.01.xsd"),
    ),
];

static CTE: XsdSchema = XsdSchema::new("cte_v400", FILES, "cte_v4.00.xsd");

/// The CT-e 4.00 schema bundle. Validate a signed `<CTe>` document:
///
/// ```no_run
/// # let signed_cte_xml = "";
/// if let Err(erros) = fiscal_xsd::schemas::cte().validate(signed_cte_xml) {
///     eprintln!("CT-e inválido: {erros:?}");
/// }
/// ```
pub fn cte() -> &'static XsdSchema {
    &CTE
}

#[cfg(test)]
mod tests {
    #[test]
    fn schema_compiles_and_rejects_empty_cte() {
        // A bare <CTe/> is missing infCte/Signature — it must fail validation,
        // which proves the full include graph compiled (no schema-load error).
        let err = super::cte()
            .validate("<CTe xmlns=\"http://www.portalfiscal.inf.br/cte\"/>")
            .unwrap_err();
        assert!(!err.is_empty());
    }
}
