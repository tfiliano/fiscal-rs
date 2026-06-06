//! Provedor **São Paulo (PMSP)** — sistema próprio, `PedidoEnvioLoteRPS` no
//! `lotenfe.asmx`. Cada RPS leva um campo `<Assinatura>` = Base64(RSA-SHA1(string
//! concatenada de campos)); o lote inteiro é assinado em XMLDSig (URI="").
//!
//! Layout da string de assinatura do RPS (v01):
//!
//! | campo | tam | preenchimento |
//! |---|---|---|
//! | InscricaoPrestador (CCM) | 8 | zeros à esquerda |
//! | SerieRPS | 5 | brancos à direita |
//! | NumeroRPS | 12 | zeros à esquerda |
//! | DataEmissao (AAAAMMDD) | 8 | — |
//! | TributacaoRPS | 1 | — |
//! | StatusRPS | 1 | — |
//! | ISSRetido | 1 | `S`/`N` |
//! | ValorServicos (centavos) | 15 | zeros à esquerda |
//! | ValorDeducoes (centavos) | 15 | zeros à esquerda |
//! | CodigoServico | 5 | zeros à esquerda |
//! | Indicador CPF/CNPJ tomador | 1 | `1` CPF, `2` CNPJ, `3` sem |
//! | CPF/CNPJ tomador | 14 | zeros à esquerda |

use crate::model::{EmitInput, Servico};

#[cfg(feature = "client")]
pub mod transport;

pub const SP_NS: &str = "http://www.prefeitura.sp.gov.br/nfe";

/// Nome (com prefixo) do elemento-raiz do lote — usado na assinatura XMLDSig.
pub const SP_LOTE_ROOT: &str = "p1:PedidoEnvioLoteRPS";

/// Emissão SP completa: assinatura do RPS → lote → XMLDSig do lote → SOAP → parse.
#[cfg(feature = "client")]
pub async fn emit(
    input: &EmitInput,
    ctx: &crate::provider::ProviderCtx,
    endpoint: &str,
) -> crate::error::Result<crate::model::EmitOutput> {
    use crate::error::MunError;
    let cert = fiscal_crypto::certificate::load_certificate(&ctx.pfx_der, &ctx.senha)
        .map_err(|e| MunError::Assinatura(format!("certificado: {e}")))?;
    // 1. Assinatura do RPS (RSA-SHA1 da string concatenada).
    let assinatura = fiscal_crypto::certificate::rsa_sha1_base64(
        assinatura_string(input).as_bytes(),
        &cert.private_key,
    )
    .map_err(|e| MunError::Assinatura(format!("assinatura RPS: {e}")))?;
    // 2. Lote + 3. XMLDSig do lote (URI="").
    let lote = build_lote_rps(input, &assinatura);
    let signed = fiscal_crypto::certificate::sign_sp_lote_xml(
        &lote,
        SP_LOTE_ROOT,
        &cert.private_key,
        &cert.certificate,
    )
    .map_err(|e| MunError::Assinatura(format!("assinatura lote: {e}")))?;
    // 4. SOAP + 5. POST + 6. parse.
    let metodo = transport::metodo(ctx.ambiente);
    let envelope = transport::soap_envio(metodo, &signed);
    let http = ctx.http_client()?;
    let (status, body) = transport::post_envio(&http, endpoint, metodo, &envelope).await?;
    Ok(transport::parse_retorno(status, &body))
}

/// só dígitos.
fn digits(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_digit()).collect()
}

/// Monta a **string** a ser assinada (RSA-SHA1) para o campo `<Assinatura>`.
pub fn assinatura_string(input: &EmitInput) -> String {
    let e = &input.emitente;
    let r = &input.rps;
    let s = &r.servico;

    let im = digits(e.im.as_deref().unwrap_or(""));
    let data = digits(r.data_emissao.split('T').next().unwrap_or("")); // AAAAMMDD
    let tributacao = "T"; // tributado no município (default)
    let status = "N"; // normal
    let iss = if s.iss_retido { "S" } else { "N" };
    let cod_serv = digits(s.cod_tributacao_municipio.as_deref().unwrap_or(""));

    let (ind, doc) = match &r.tomador.doc {
        Some(d) => {
            let dd = digits(d);
            if dd.len() == 11 { ("1", dd) } else { ("2", dd) }
        }
        None => ("3", String::new()),
    };

    format!(
        "{im:0>8}{serie:<5}{num:0>12}{data}{trib}{status}{iss}{vs:0>15}{vd:0>15}{cod:0>5}{ind}{doc:0>14}",
        im = im,
        serie = r.serie,
        num = r.numero,
        data = data,
        trib = tributacao,
        status = status,
        iss = iss,
        vs = s.valor_centavos,
        vd = 0,
        cod = cod_serv,
        ind = ind,
        doc = doc,
    )
}

/// `centavos` → decimal "X.XX" (tpValor).
fn valor(c: i64) -> String {
    format!("{}.{:02}", c / 100, (c % 100).abs())
}

/// Alíquota percentual ("2.00") → fração tpAliquota (5/4): "0.0200".
fn aliquota_fracao(percent: &str) -> String {
    let p: f64 = percent.replace(',', ".").parse().unwrap_or(0.0);
    format!("{:.4}", p / 100.0)
}

fn cpfcnpj_tag(doc: &str) -> String {
    use fiscal_core::xml_utils::{TagContent, tag};
    let d = digits(doc);
    if d.len() == 11 {
        tag("CPFCNPJTomador", &[], TagContent::Children(vec![tag("CPF", &[], TagContent::Text(&d))]))
    } else {
        tag("CPFCNPJTomador", &[], TagContent::Children(vec![tag("CNPJ", &[], TagContent::Text(&d))]))
    }
}

/// Monta o `<PedidoEnvioLoteRPS>` (1 RPS) com a `Assinatura` já calculada.
/// A `<Signature>` do lote (XMLDSig) é adicionada na etapa de assinatura.
pub fn build_lote_rps(input: &EmitInput, assinatura_b64: &str) -> String {
    use fiscal_core::xml_utils::{TagContent, tag};
    let e = &input.emitente;
    let r = &input.rps;
    let s = &r.servico;
    let data = r.data_emissao.split('T').next().unwrap_or("");

    // Cabecalho
    let cabecalho = tag(
        "Cabecalho",
        &[("Versao", "1")],
        TagContent::Children(vec![
            tag(
                "CPFCNPJRemetente",
                &[],
                TagContent::Children(vec![tag("CNPJ", &[], TagContent::Text(&e.cnpj))]),
            ),
            tag("transacao", &[], TagContent::Text("false")),
            tag("dtInicio", &[], TagContent::Text(data)),
            tag("dtFim", &[], TagContent::Text(data)),
            tag("QtdRPS", &[], TagContent::Text("1")),
            tag("ValorTotalServicos", &[], TagContent::Text(&valor(s.valor_centavos))),
            tag("ValorTotalDeducoes", &[], TagContent::Text("0.00")),
        ]),
    );

    // RPS
    let chave = tag(
        "ChaveRPS",
        &[],
        TagContent::Children(vec![
            tag("InscricaoPrestador", &[], TagContent::Text(&digits(e.im.as_deref().unwrap_or("")))),
            tag("SerieRPS", &[], TagContent::Text(&r.serie)),
            tag("NumeroRPS", &[], TagContent::Text(&r.numero.to_string())),
        ]),
    );
    let mut rps = vec![
        tag("Assinatura", &[], TagContent::Text(assinatura_b64)),
        chave,
        tag("TipoRPS", &[], TagContent::Text("RPS")),
        tag("DataEmissao", &[], TagContent::Text(data)),
        tag("StatusRPS", &[], TagContent::Text("N")),
        tag("TributacaoRPS", &[], TagContent::Text("T")),
        tag("ValorServicos", &[], TagContent::Text(&valor(s.valor_centavos))),
        tag("ValorDeducoes", &[], TagContent::Text("0.00")),
        tag("CodigoServico", &[], TagContent::Text(&digits(s.cod_tributacao_municipio.as_deref().unwrap_or("")))),
        tag("AliquotaServicos", &[], TagContent::Text(&aliquota_fracao(s.aliquota_iss.as_deref().unwrap_or("0")))),
        tag("ISSRetido", &[], TagContent::Text(if s.iss_retido { "true" } else { "false" })),
    ];
    if let Some(doc) = &r.tomador.doc {
        rps.push(cpfcnpj_tag(doc));
    }
    if let Some(rs) = &r.tomador.razao_social {
        rps.push(tag("RazaoSocialTomador", &[], TagContent::Text(rs)));
    }
    if let Some(em) = &r.tomador.email {
        rps.push(tag("EmailTomador", &[], TagContent::Text(em)));
    }
    rps.push(discriminacao(s));
    let rps_el = tag("RPS", &[], TagContent::Children(rps));

    // elementFormDefault unqualified: raiz qualificada via prefixo, filhos sem ns.
    tag(
        "p1:PedidoEnvioLoteRPS",
        &[
            ("xmlns:p1", SP_NS),
            ("xmlns:ds", "http://www.w3.org/2000/09/xmldsig#"),
        ],
        TagContent::Children(vec![cabecalho, rps_el]),
    )
}

fn discriminacao(s: &Servico) -> String {
    use fiscal_core::xml_utils::{TagContent, tag};
    tag("Discriminacao", &[], TagContent::Text(&s.discriminacao))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn sample() -> EmitInput {
        EmitInput {
            emitente: Emitente {
                cnpj: "18885949000181".into(),
                im: Some("12345678".into()),
                razao_social: "CENTRE LTDA".into(),
                c_mun: "3550308".into(),
                uf: "SP".into(),
                endereco: None,
                optante_simples: false,
            },
            rps: Rps {
                numero: 7,
                serie: "TST".into(),
                tipo: 1,
                data_emissao: "2026-06-06T10:00:00-03:00".into(),
                tomador: Tomador {
                    doc: Some("11222333000181".into()),
                    razao_social: Some("TOMADOR LTDA".into()),
                    email: None,
                    endereco: None,
                    im: None,
                },
                servico: Servico {
                    valor_centavos: 10000,
                    aliquota_iss: Some("2.00".into()),
                    iss_retido: false,
                    item_lista_servico: "1.01".into(),
                    cod_tributacao_municipio: Some("02916".into()),
                    cnae: None,
                    discriminacao: "TESTE".into(),
                    c_mun_prestacao: None,
                },
                natureza_operacao: None,
                regime_especial_tributacao: None,
                incentivador_cultural: false,
            },
        }
    }

    #[test]
    fn assinatura_layout_exato() {
        let a = assinatura_string(&sample());
        // 8+5+12+8+1+1+1+15+15+5+1+14 = 86 caracteres
        assert_eq!(a.len(), 86, "string: {a:?}");
        assert_eq!(&a[0..8], "12345678"); // IM 8 (já tem 8)
        assert_eq!(&a[8..13], "TST  "); // série 5, brancos à direita
        assert_eq!(&a[13..25], "000000000007"); // número 12 zero-left
        assert_eq!(&a[25..33], "20260606"); // data AAAAMMDD
        assert_eq!(&a[33..36], "TNN"); // tributacao, status, iss(não retido)
        assert_eq!(&a[36..51], "000000000010000"); // valor 10000 centavos
        assert_eq!(&a[51..66], "000000000000000"); // deducoes 0
        assert_eq!(&a[66..71], "02916"); // codigo servico 5
        assert_eq!(&a[71..72], "2"); // indicador CNPJ
        assert_eq!(&a[72..86], "11222333000181"); // CNPJ 14
    }
}
