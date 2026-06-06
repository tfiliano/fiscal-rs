//! Roteamento: código IBGE do município → provedor.

#![cfg(feature = "client")]

use crate::provider::MunicipalProvider;
use crate::providers::{DSF, GINFES, SAOPAULO, SIGISS, SPEEDGOV};

/// Todos os provedores registrados.
static ALL: &[&'static (dyn MunicipalProvider)] =
    &[&DSF, &GINFES, &SIGISS, &SAOPAULO, &SPEEDGOV];

/// Resolve o provedor municipal para um código IBGE (7 dígitos). `None` quando
/// nenhum provedor atende — nesse caso o município pode ser **nacional**
/// (emitir pelo SEFIN) ou ainda não suportado.
pub fn resolve(ibge: &str) -> Option<&'static dyn MunicipalProvider> {
    ALL.iter().copied().find(|p| p.municipios().contains(&ibge))
}

/// `true` se há provedor municipal próprio para o IBGE (ou seja, **não** usa o
/// emissor nacional).
pub fn is_municipal(ibge: &str) -> bool {
    resolve(ibge).is_some()
}

#[cfg(test)]
mod tests {
    #[test]
    fn resolve_conhecidos() {
        assert_eq!(super::resolve("3550308").unwrap().nome(), "SAOPAULO");
        assert_eq!(super::resolve("3552205").unwrap().nome(), "DSF");
        assert_eq!(super::resolve("3518800").unwrap().nome(), "GINFES");
        assert_eq!(super::resolve("3513801").unwrap().nome(), "SigISS");
        assert_eq!(super::resolve("3547304").unwrap().nome(), "SpeedGov");
        // Rio de Janeiro já é nacional → sem provedor municipal.
        assert!(super::resolve("3304557").is_none());
    }
}
