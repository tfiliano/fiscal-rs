//! Integration tests: full road-modal MDF-e XML build + access key.

use chrono::{FixedOffset, TimeZone};
use fiscal_core::xml_builder::access_key::calculate_mod11;
use fiscal_mdfe::build_mdfe_xml;
use fiscal_mdfe::types::*;

/// A representative road-modal MDF-e with two linked NF-e, one trailer, one
/// driver, an ANTT block, and additional info. `numeric_code` is fixed so the
/// access key is deterministic.
fn sample() -> MdfeBuildData {
    let tz = FixedOffset::west_opt(3 * 3600).unwrap();
    let dh = tz.with_ymd_and_hms(2026, 6, 4, 9, 30, 0).unwrap();

    MdfeBuildData {
        ide: Ide {
            c_uf: "43".to_string(), // RS
            tp_amb: "2".to_string(),
            tp_emit: "1".to_string(),
            serie: 1,
            n_mdf: 123,
            modal: "1".to_string(),
            dh_emi: dh,
            tp_emis: "1".to_string(),
            proc_emi: None,
            ver_proc: None,
            uf_ini: "RS".to_string(),
            uf_fim: "SC".to_string(),
            inf_mun_carrega: vec![MunCarrega {
                c_mun: "4314902".to_string(),
                x_mun: "Porto Alegre".to_string(),
            }],
            inf_percurso: vec!["SC".to_string()],
            dh_ini_viagem: Some(dh),
        },
        emit: Emit {
            cnpj: "12345678000190".to_string(),
            ie: Some("1234567890".to_string()),
            x_nome: "Transportadora Exemplo LTDA".to_string(),
            x_fant: Some("Exemplo Log".to_string()),
            ender_emit: EnderEmit {
                x_lgr: "Av. Brasil".to_string(),
                nro: "1000".to_string(),
                x_cpl: None,
                x_bairro: "Centro".to_string(),
                c_mun: "4314902".to_string(),
                x_mun: "Porto Alegre".to_string(),
                cep: "90000000".to_string(),
                uf: "RS".to_string(),
                fone: None,
                email: None,
            },
        },
        modal: Modal::Rodo(Rodo {
            inf_antt: Some(InfAntt {
                rntrc: Some("12345678".to_string()),
                inf_ciot: vec![],
            }),
            veic_tracao: VeicTracao {
                c_int: None,
                placa: "ABC1D23".to_string(),
                renavam: Some("12345678901".to_string()),
                tara: 8000,
                cap_kg: Some(25000),
                cap_m3: None,
                prop: None,
                condutor: vec![Condutor {
                    x_nome: "João da Silva".to_string(),
                    cpf: "12345678909".to_string(),
                }],
                tp_rod: "06".to_string(),
                tp_car: "02".to_string(),
                uf: Some("RS".to_string()),
            },
            veic_reboque: vec![VeicReboque {
                c_int: None,
                placa: "XYZ4E56".to_string(),
                renavam: Some("98765432109".to_string()),
                tara: 5000,
                cap_kg: Some(30000),
                cap_m3: None,
                prop: None,
                tp_car: "02".to_string(),
                uf: Some("RS".to_string()),
            }],
        }),
        inf_doc: InfDoc {
            inf_mun_descarga: vec![MunDescarga {
                c_mun: "4205407".to_string(),
                x_mun: "Florianopolis".to_string(),
                inf_nfe: vec![
                    "43260312345678000190550010000001231123456780".to_string(),
                    "43260312345678000190550010000001241123456781".to_string(),
                ],
                inf_cte: vec![],
                inf_mdfe: vec![],
            }],
        },
        tot: Tot {
            q_cte: None,
            q_nfe: Some(2),
            q_mdfe: None,
            v_carga: 15000.0,
            c_unid: "01".to_string(),
            q_carga: 1200.5,
        },
        inf_adic: Some(InfAdic {
            inf_ad_fisco: None,
            inf_cpl: Some("Carga frágil".to_string()),
        }),
        numeric_code: Some("00000001".to_string()),
    }
}

#[test]
fn builds_well_formed_road_mdfe() {
    let xml = build_mdfe_xml(&sample()).unwrap();

    // Envelope + signed root.
    assert!(xml.starts_with("<MDFe xmlns=\"http://www.portalfiscal.inf.br/mdfe\">"));
    assert!(xml.contains("<infMDFe versao=\"3.00\" Id=\"MDFe"));
    assert!(xml.ends_with("</MDFe>"));

    // Blocks present in schema order.
    let pos = |needle: &str| xml.find(needle).unwrap_or_else(|| panic!("missing {needle}"));
    assert!(pos("<ide>") < pos("<emit>"));
    assert!(pos("<emit>") < pos("<infModal"));
    assert!(pos("<infModal") < pos("<infDoc>"));
    assert!(pos("<infDoc>") < pos("<tot>"));
    assert!(pos("<tot>") < pos("<infAdic>"));

    // ide ordering: mod=58 / cMDF / cDV / modal.
    assert!(xml.contains("<mod>58</mod>"));
    assert!(xml.contains("<cMDF>00000001</cMDF>"));
    assert!(xml.contains("<modal>1</modal>"));
    assert!(pos("<cMDF>") < pos("<cDV>"));
    assert!(pos("<cDV>") < pos("<modal>"));

    // Road modal content.
    assert!(xml.contains("<infModal versaoModal=\"3.00\">"));
    assert!(xml.contains("<rodo>"));
    assert!(xml.contains("<RNTRC>12345678</RNTRC>"));
    assert!(xml.contains("<veicTracao>"));
    assert!(xml.contains("<placa>ABC1D23</placa>"));
    assert!(xml.contains("<condutor><xNome>João da Silva</xNome><CPF>12345678909</CPF></condutor>"));
    assert!(xml.contains("<veicReboque>"));
    assert!(xml.contains("<placa>XYZ4E56</placa>"));

    // Linked docs + totals.
    assert!(xml.contains("<chNFe>43260312345678000190550010000001231123456780</chNFe>"));
    assert!(xml.contains("<chNFe>43260312345678000190550010000001241123456781</chNFe>"));
    assert!(xml.contains("<qNFe>2</qNFe>"));
    assert!(xml.contains("<vCarga>15000.00</vCarga>"));
    assert!(xml.contains("<cUnid>01</cUnid>"));
    assert!(xml.contains("<qCarga>1200.5000</qCarga>"));

    // infAdic.
    assert!(xml.contains("<infCpl>Carga frágil</infCpl>"));
}

#[test]
fn access_key_embedded_in_id_is_valid() {
    let xml = build_mdfe_xml(&sample()).unwrap();

    // Extract the 44-digit key from Id="MDFe{key}".
    let start = xml.find("Id=\"MDFe").unwrap() + "Id=\"MDFe".len();
    let key = &xml[start..start + 44];

    assert_eq!(key.len(), 44);
    assert!(key.bytes().all(|b| b.is_ascii_digit()));

    // Layout checks.
    assert_eq!(&key[0..2], "43"); // cUF
    assert_eq!(&key[2..6], "2606"); // AAMM (2026-06)
    assert_eq!(&key[6..20], "12345678000190"); // CNPJ
    assert_eq!(&key[20..22], "58"); // mod
    assert_eq!(&key[22..25], "001"); // serie
    assert_eq!(&key[25..34], "000000123"); // nMDF
    assert_eq!(&key[34..35], "1"); // tpEmis
    assert_eq!(&key[35..43], "00000001"); // cMDF

    // Check digit is the mod-11 of the first 43 digits.
    let dv = calculate_mod11(&key[..43]);
    assert_eq!(key.as_bytes()[43] - b'0', dv);
}

#[test]
fn non_road_modal_is_rejected_in_this_phase() {
    let mut data = sample();
    data.modal = Modal::Aereo;
    assert!(build_mdfe_xml(&data).is_err());
}
