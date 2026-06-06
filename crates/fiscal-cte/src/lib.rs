//! Types and XML generation for the Brazilian **CT-e** (Conhecimento de
//! Transporte Eletrônico), fiscal document **model 57**, leiaute **4.00**.
//!
//! `fiscal-cte` mirrors the structure of [`fiscal_core`] and [`fiscal_mdfe`]:
//! strongly-typed data structures for each XML block, an access-key builder,
//! and a string-based XML builder that produces a schema-ordered `<CTe>`
//! document.
//!
//! The first release targets **CT-e Normal** with the **road** (`infModal/
//! rodo`) modal. Other modals and the CT-e OS (67) / GTV-e (64) layouts are
//! planned as follow-ups.
//!
//! # Example
//!
//! ```no_run
//! use fiscal_cte::{build_cte_xml, types::*};
//! # fn demo(data: &CteBuildData) -> Result<(), fiscal_core::FiscalError> {
//! # let data: &CteBuildData = data;
//! let xml = build_cte_xml(data)?;
//! assert!(xml.contains("<CTe"));
//! # Ok(()) }
//! ```

/// 44-digit CT-e access key (chave de acesso) generation and parsing.
pub mod access_key;
/// String-based XML builder that assembles a complete `<CTe>` document.
pub mod builder;
/// String-based XML builder for the CT-e OS (`<CTeOS>`, model 67).
pub mod builder_os;
/// Enveloped XML-DSig signing of the `<infCte>` element (RSA-SHA1).
pub mod signing;
/// Public data structures for the CT-e XML blocks.
pub mod types;
/// Public data structures for the CT-e OS (model 67) XML blocks.
pub mod types_os;
/// Structural validation of a CT-e XML before transmission.
pub mod validate;

pub use access_key::{CteAccessKey, build_cte_access_key};
pub use builder::build_cte_xml;
pub use builder_os::build_cteos_xml;
pub use signing::{
    sign_cte_xml, sign_cte_xml_with_algorithm, sign_cteos_xml, sign_cteos_xml_with_algorithm,
};
pub use types::CteBuildData;
pub use types_os::CteOsBuildData;
pub use validate::validate_cte_xml;

/// XML namespace for CT-e documents.
pub const CTE_NAMESPACE: &str = "http://www.portalfiscal.inf.br/cte";

/// CT-e layout version implemented by this crate.
pub const CTE_VERSION: &str = "4.00";

/// Fiscal document model number for CT-e.
pub const CTE_MODEL: &str = "57";

/// Fiscal document model number for CT-e OS (Outros Serviços).
pub const CTEOS_MODEL: &str = "67";
