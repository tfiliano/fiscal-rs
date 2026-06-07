use fiscal_nfse_mun::model::*;
use fiscal_nfse_mun::saopaulo::{assinatura_string, build_lote_rps, SP_LOTE_ROOT};
fn pfx()->Vec<u8>{ std::fs::read("/work/tests/fixtures/certs/novo_cert_cnpj_06157250000116_senha_minhasenha.pfx").unwrap() }
#[test]
fn dbg_lote(){
    let inp = EmitInput{ emitente:Emitente{cnpj:"06157250000116".into(),im:Some("48712345".into()),razao_social:"C".into(),c_mun:"3550308".into(),uf:"SP".into(),endereco:None,optante_simples:false},
      rps:Rps{numero:8,serie:"17".into(),tipo:1,data_emissao:"2026-06-07T10:00:00-03:00".into(),
        tomador:Tomador{doc:Some("11222333000181".into()),razao_social:Some("T".into()),email:None,endereco:None,im:None},
        servico:Servico{valor_centavos:5000,aliquota_iss:Some("2.00".into()),iss_retido:false,item_lista_servico:"010101".into(),cod_tributacao_municipio:Some("02916".into()),cnae:None,discriminacao:"TESTE".into(),c_mun_prestacao:None},
        natureza_operacao:None,regime_especial_tributacao:None,incentivador_cultural:false}};
    let cert=fiscal_crypto::certificate::load_certificate(&pfx(),"minhasenha").unwrap();
    let ass=fiscal_crypto::certificate::rsa_sha1_base64(assinatura_string(&inp).as_bytes(),&cert.private_key).unwrap();
    let lote=build_lote_rps(&inp,&ass);
    let signed=fiscal_crypto::certificate::sign_sp_lote_xml(&lote,SP_LOTE_ROOT,&cert.private_key,&cert.certificate).unwrap();
    std::fs::write("/tmp/sp_signed.xml",&signed).unwrap();
    eprintln!("OK wrote");
}
