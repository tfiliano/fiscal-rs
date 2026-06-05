//! Embedded official SEFAZ schema bundles, behind cargo features so a binary
//! only carries the schemas it actually validates against.
//!
//! Enable the `mdfe` feature to embed the MDF-e 3.00 schemas and use
//! [`mdfe()`].

#[cfg(feature = "mdfe")]
mod mdfe_300;

#[cfg(feature = "mdfe")]
pub use mdfe_300::mdfe;
