//! Provedores municipais. Cada um implementa [`MunicipalProvider`], plugando a
//! diferença sobre o modelo comum. Por ora os `emitir` retornam
//! [`MunError::NaoImplementado`] — o esqueleto fixa a interface e o roteamento;
//! a lógica de cada padrão entra incrementalmente.

#![cfg(feature = "client")]

use crate::error::{MunError, Result};
use crate::model::{EmitInput, EmitOutput};
use crate::provider::{MunicipalProvider, ProviderCtx};

/// URLs de webservice de um provedor por ambiente.
#[derive(Debug, Clone, Copy)]
pub struct Endpoints {
    pub homologacao: &'static str,
    pub producao: &'static str,
}

/// **DSF** — Sorocaba (ABRASF 2.03).
pub struct Dsf;
pub static DSF: Dsf = Dsf;
impl Dsf {
    pub const ENDPOINTS: Endpoints = Endpoints {
        homologacao: "https://homolsod.dsfweb.com.br/notafiscal-abrasfv203-ws/NotaFiscalSoap",
        producao: "https://notafiscal.sorocaba.sp.gov.br/notafiscal-abrasfv203-ws/NotaFiscalSoap",
    };
}
#[async_trait::async_trait]
impl MunicipalProvider for Dsf {
    fn nome(&self) -> &'static str { "DSF" }
    fn municipios(&self) -> &'static [&'static str] { &["3552205"] }
    async fn emitir(&self, input: &EmitInput, ctx: &ProviderCtx) -> Result<EmitOutput> {
        let endpoint = match ctx.ambiente {
            crate::model::Ambiente::Producao => Self::ENDPOINTS.producao,
            crate::model::Ambiente::Homologacao => Self::ENDPOINTS.homologacao,
        };
        // DSF abrasfv203: SOAPAction vazio.
        crate::abrasf::emit(input, ctx, endpoint, "").await
    }
}

/// **GINFES** — Guarulhos (ABRASF 2.x).
pub struct Ginfes;
pub static GINFES: Ginfes = Ginfes;
#[async_trait::async_trait]
impl MunicipalProvider for Ginfes {
    fn nome(&self) -> &'static str { "GINFES" }
    fn municipios(&self) -> &'static [&'static str] { &["3518800"] }
    async fn emitir(&self, _input: &EmitInput, _ctx: &ProviderCtx) -> Result<EmitOutput> {
        Err(MunError::NaoImplementado("GINFES/ABRASF emitir"))
    }
}

/// **SigISS** — Caraguatatuba (ABRASF).
pub struct SigIss;
pub static SIGISS: SigIss = SigIss;
#[async_trait::async_trait]
impl MunicipalProvider for SigIss {
    fn nome(&self) -> &'static str { "SigISS" }
    fn municipios(&self) -> &'static [&'static str] { &["3513801"] }
    async fn emitir(&self, _input: &EmitInput, _ctx: &ProviderCtx) -> Result<EmitOutput> {
        Err(MunError::NaoImplementado("SigISS/ABRASF emitir"))
    }
}

/// **São Paulo** (PMSP) — sistema próprio, RPS+lote, 2 assinaturas RSA.
pub struct SaoPaulo;
pub static SAOPAULO: SaoPaulo = SaoPaulo;
impl SaoPaulo {
    /// WS novo (layout v1+v2 com IBS/CBS). O antigo só suporta v1.
    pub const WS: &'static str = "https://nfews.prefeitura.sp.gov.br/lotenfe.asmx";
}
#[async_trait::async_trait]
impl MunicipalProvider for SaoPaulo {
    fn nome(&self) -> &'static str { "SAOPAULO" }
    fn municipios(&self) -> &'static [&'static str] { &["3550308"] }
    async fn emitir(&self, input: &EmitInput, ctx: &ProviderCtx) -> Result<EmitOutput> {
        crate::saopaulo::emit(input, ctx, Self::WS).await
    }
}

/// **SpeedGov** — Santana de Parnaíba: layout **nacional (DPS)** em endpoint
/// municipal próprio (não opera no Ambiente Nacional). Reusa o builder DPS do
/// `fiscal-cte`; só muda a URL de POST.
pub struct SpeedGov;
pub static SPEEDGOV: SpeedGov = SpeedGov;
#[async_trait::async_trait]
impl MunicipalProvider for SpeedGov {
    fn nome(&self) -> &'static str { "SpeedGov" }
    fn municipios(&self) -> &'static [&'static str] { &["3547304"] }
    async fn emitir(&self, _input: &EmitInput, _ctx: &ProviderCtx) -> Result<EmitOutput> {
        Err(MunError::NaoImplementado("SpeedGov/nacional-municipal emitir"))
    }
}
