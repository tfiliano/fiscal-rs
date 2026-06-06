//! Embedded official NFS-e Nacional 1.01 schemas (RTC, com grupo IBS/CBS).
//!
//! `DPS_v1.01.xsd` (root da DPS) inclui `tiposComplexos` → `tiposSimples` +
//! `xmldsig-core`. Valida uma **DPS assinada** (Declaração de Prestação de
//! Serviço) antes do envio ao SEFIN Nacional.

use crate::XsdSchema;

static FILES: &[(&str, &[u8])] = &[
    (
        "DPS_v1.01.xsd",
        include_bytes!("../../schemas/nfse_101/DPS_v1.01.xsd"),
    ),
    (
        "tiposComplexos_v1.01.xsd",
        include_bytes!("../../schemas/nfse_101/tiposComplexos_v1.01.xsd"),
    ),
    (
        "tiposSimples_v1.01.xsd",
        include_bytes!("../../schemas/nfse_101/tiposSimples_v1.01.xsd"),
    ),
    (
        "xmldsig-core-schema.xsd",
        include_bytes!("../../schemas/nfse_101/xmldsig-core-schema.xsd"),
    ),
];

static DPS: XsdSchema = XsdSchema::new("dps_v101", FILES, "DPS_v1.01.xsd");

/// O bundle da DPS (NFS-e Nacional 1.01). Valida uma `<DPS>` assinada.
pub fn dps() -> &'static XsdSchema {
    &DPS
}

#[cfg(test)]
mod tests {
    #[test]
    fn schema_compiles_and_rejects_empty_dps() {
        let err = super::dps()
            .validate("<DPS xmlns=\"http://www.sped.fazenda.gov.br/nfse\"/>")
            .unwrap_err();
        assert!(!err.is_empty());
    }
}
