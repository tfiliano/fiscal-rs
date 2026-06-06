//! Types for the NFS-e Nacional DPS (Declaração de Prestação de Serviço),
//! leiaute **1.01** (RTC — com grupo IBS/CBS da reforma).
//!
//! Documento separado (namespace `http://www.sped.fazenda.gov.br/nfse`),
//! REST (não SOAP). O emitente monta a `<DPS>`, assina `<infDPS>`, comprime e
//! envia ao SEFIN Nacional, que devolve a NFS-e (chave de 50 dígitos).
//!
//! Reusa `Documento` de [`crate::types`].

use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use ts_rs::TS;

use crate::types::Documento;

/// Root build data for a DPS document.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct DpsBuildData {
    pub ide: IdeDps,
    pub prest: Prestador,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toma: Option<Pessoa>,
    pub serv: Servico,
    pub valores: Valores,
}

/// `<infDPS>` identification block. O `Id` (`DPS` + 42 dígitos) é derivado de
/// cLocEmi + tpInsc + inscrição + serie + nDPS.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct IdeDps {
    pub tp_amb: String,
    pub dh_emi: chrono::DateTime<chrono::FixedOffset>,
    #[serde(default = "ver_aplic_default")]
    pub ver_aplic: String,
    pub serie: String,
    pub n_dps: u64,
    /// `dCompet` — competência (AAAA-MM-DD).
    pub d_compet: String,
    /// `tpEmit` — `1` Prestador, `2` Tomador, `3` Intermediário.
    pub tp_emit: String,
    /// `cLocEmi` — código IBGE do município emitente (7 dígitos).
    pub c_loc_emi: String,
}

fn ver_aplic_default() -> String {
    "dfehub-1.0".into()
}

/// `<prest>` — prestador do serviço.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Prestador {
    pub doc: Documento,
    /// `IM` — inscrição municipal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub im: Option<String>,
    pub x_nome: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<EnderNac>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub reg_trib: RegTrib,
}

/// `<regTrib>` — regime tributário do prestador.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct RegTrib {
    /// `opSimpNac` — `1` não optante, `2` MEI, `3` ME/EPP.
    pub op_simp_nac: String,
    /// `regEspTrib` — regime especial de tributação (`0` nenhum, ...).
    pub reg_esp_trib: String,
}

/// `<toma>`/`<interm>` — pessoa (tomador/intermediário).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Pessoa {
    pub doc: Documento,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub im: Option<String>,
    pub x_nome: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<EnderNac>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// `<endNac>` — endereço nacional.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct EnderNac {
    pub x_lgr: String,
    pub nro: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_cpl: Option<String>,
    pub x_bairro: String,
    /// `cMun` — código IBGE do município (7 dígitos).
    pub c_mun: String,
    pub cep: String,
}

/// `<serv>` — dados do serviço prestado.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Servico {
    /// `cLocPrestacao` — município de prestação (IBGE).
    pub c_loc_prestacao: String,
    /// `cTribNac` — código de tributação nacional (lista nacional de serviços).
    pub c_trib_nac: String,
    /// `cTribMun` — código de tributação municipal (opcional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub c_trib_mun: Option<String>,
    pub x_desc_serv: String,
}

/// `<valores>` — valores e tributos.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Valores {
    /// `vServ` — valor do serviço.
    pub v_serv: String,
    pub trib: Trib,
}

/// `<trib>` — tributação (ISSQN municipal + federal + IBS/CBS).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Trib {
    pub trib_mun: TribMun,
    /// Grupo IBS/CBS (reforma). Opcional na transição.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ibscbs: Option<Ibscbs>,
}

/// `<tribMun>` — ISSQN.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct TribMun {
    /// `tribISSQN` — `1` operação tributável, `2` imunidade, `3` exportação,
    /// `4` não incidência.
    pub trib_issqn: String,
    /// `pAliq` — alíquota ISSQN (%). Obrigatória quando tribISSQN=1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p_aliq: Option<String>,
    /// `tpRetISSQN` — `1` não retido, `2` retido pelo tomador, `3` intermediário.
    pub tp_ret_issqn: String,
}

/// `<gIBSCBS>` — grupo IBS/CBS (reforma tributária).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(TS), ts(export))]
pub struct Ibscbs {
    /// `CST` do IBS/CBS (3 dígitos).
    pub cst: String,
    /// `cClassTrib` — classificação tributária (6 dígitos).
    pub c_class_trib: String,
}
