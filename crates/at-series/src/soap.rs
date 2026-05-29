//! Serialização e parse dos envelopes SOAP do `SeriesWS`.
//!
//! Optámos por templating directo (em vez de uma framework SOAP completa)
//! porque o esquema é pequeno (4 operações, ~10 campos cada) e estável. O
//! parsing usa `quick-xml` para tolerar variações de prefixo de namespace.

use crate::types::*;
use crate::AtError;
use chrono::NaiveDate;
use quick_xml::events::Event;
use quick_xml::Reader;

/// Envolve o `body_xml` num envelope SOAP 1.1 espelhando o formato real da
/// AT (`testarLigacaoWebService`): declara **dois** prefixos (`env` e `S`)
/// para o mesmo namespace SOAP. O atributo `S:Actor` no `<wss:Security>`
/// construído em `wss::build_security_header` depende do prefixo `S` estar
/// declarado no `<Envelope>`.
pub fn wrap_envelope(security_header_xml: &str, body_xml: &str) -> String {
    format!(
        concat!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>",
            "<S:Envelope xmlns:env=\"http://schemas.xmlsoap.org/soap/envelope/\" ",
            "xmlns:S=\"http://schemas.xmlsoap.org/soap/envelope/\">",
            "<env:Header>{header}</env:Header>",
            "<S:Body>{body}</S:Body>",
            "</S:Envelope>"
        ),
        header = security_header_xml,
        body = body_xml
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn fmt_date(d: NaiveDate) -> String {
    d.format("%Y-%m-%d").to_string()
}

pub fn build_registar_serie(req: &RegistarSerieRequest) -> String {
    format!(
        concat!(
            "<ns0:registarSerie xmlns:ns0=\"http://at.gov.pt/\">",
            "<serie>{serie}</serie>",
            "<tipoSerie>{tipo}</tipoSerie>",
            "<classeDoc>{classe}</classeDoc>",
            "<tipoDoc>{tipo_doc}</tipoDoc>",
            "<numInicialSeq>{num}</numInicialSeq>",
            "<dataInicioPrevUtiliz>{data}</dataInicioPrevUtiliz>",
            "<numCertSWFatur>{cert}</numCertSWFatur>",
            "<meioProcessamento>{meio}</meioProcessamento>",
            "</ns0:registarSerie>"
        ),
        serie = xml_escape(&req.serie),
        tipo = req.tipo_serie.as_str(),
        classe = xml_escape(&req.classe_doc),
        tipo_doc = xml_escape(&req.tipo_doc),
        num = req.num_inicial_seq,
        data = fmt_date(req.data_inicio_prev_utiliz),
        cert = req.num_cert_sw_fatur,
        meio = xml_escape(&req.meio_processamento),
    )
}

pub fn build_consultar_series(req: &ConsultarSeriesRequest) -> String {
    let mut body = String::from("<ns0:consultarSeries xmlns:ns0=\"http://at.gov.pt/\">");
    if let Some(v) = &req.serie {
        body.push_str(&format!("<serie>{}</serie>", xml_escape(v)));
    }
    if let Some(v) = &req.tipo_serie {
        body.push_str(&format!("<tipoSerie>{}</tipoSerie>", v.as_str()));
    }
    if let Some(v) = &req.classe_doc {
        body.push_str(&format!("<classeDoc>{}</classeDoc>", xml_escape(v)));
    }
    if let Some(v) = &req.tipo_doc {
        body.push_str(&format!("<tipoDoc>{}</tipoDoc>", xml_escape(v)));
    }
    if let Some(v) = &req.cod_validacao_serie {
        body.push_str(&format!(
            "<codValidacaoSerie>{}</codValidacaoSerie>",
            xml_escape(v)
        ));
    }
    if let Some(v) = req.data_registo_de {
        body.push_str(&format!("<dataRegistoDe>{}</dataRegistoDe>", fmt_date(v)));
    }
    if let Some(v) = req.data_registo_ate {
        body.push_str(&format!("<dataRegistoAte>{}</dataRegistoAte>", fmt_date(v)));
    }
    if let Some(v) = &req.estado {
        body.push_str(&format!("<estado>{}</estado>", xml_escape(v)));
    }
    if let Some(v) = &req.meio_processamento {
        body.push_str(&format!(
            "<meioProcessamento>{}</meioProcessamento>",
            xml_escape(v)
        ));
    }
    body.push_str("</ns0:consultarSeries>");
    body
}

pub fn build_finalizar_serie(req: &FinalizarSerieRequest) -> String {
    let just = req
        .justificacao
        .as_deref()
        .map(|s| format!("<justificacao>{}</justificacao>", xml_escape(s)))
        .unwrap_or_default();
    format!(
        concat!(
            "<ns0:finalizarSerie xmlns:ns0=\"http://at.gov.pt/\">",
            "<serie>{serie}</serie>",
            "<classeDoc>{classe}</classeDoc>",
            "<tipoDoc>{tipo_doc}</tipoDoc>",
            "<codValidacaoSerie>{cod}</codValidacaoSerie>",
            "<seqUltimoDocEmitido>{seq}</seqUltimoDocEmitido>",
            "{just}",
            "</ns0:finalizarSerie>"
        ),
        serie = xml_escape(&req.serie),
        classe = xml_escape(&req.classe_doc),
        tipo_doc = xml_escape(&req.tipo_doc),
        cod = xml_escape(&req.cod_validacao_serie),
        seq = req.seq_ultimo_doc_emitido,
        just = just,
    )
}

pub fn build_anular_serie(req: &AnularSerieRequest) -> String {
    format!(
        concat!(
            "<ns0:anularSerie xmlns:ns0=\"http://at.gov.pt/\">",
            "<serie>{serie}</serie>",
            "<classeDoc>{classe}</classeDoc>",
            "<tipoDoc>{tipo_doc}</tipoDoc>",
            "<codValidacaoSerie>{cod}</codValidacaoSerie>",
            "<motivo>{motivo}</motivo>",
            "<declaracaoNaoEmissao>{decl}</declaracaoNaoEmissao>",
            "</ns0:anularSerie>"
        ),
        serie = xml_escape(&req.serie),
        classe = xml_escape(&req.classe_doc),
        tipo_doc = xml_escape(&req.tipo_doc),
        cod = xml_escape(&req.cod_validacao_serie),
        motivo = xml_escape(&req.motivo),
        decl = req.declaracao_nao_emissao,
    )
}

/// Parse genérico de uma resposta com payload `seriesResp` (registar /
/// finalizar / anular). O parâmetro `outer` é o nome do elemento de resposta
/// (e.g., "registarSerieResp"), usado apenas para mensagens de erro.
pub fn parse_series_resp(xml: &str, outer: &str) -> Result<SeriesInfo, AtError> {
    // Soluciona soap:Fault primeiro — a AT devolve erros aplicacionais como
    // codResultOper != 0 no payload, mas erros de transporte/autenticação
    // chegam como faults.
    if let Some(fault) = extract_fault(xml)? {
        return Err(AtError::AtFault {
            code: 0,
            msg: fault,
        });
    }
    let (cod, msg) = extract_result_oper(xml)?;
    if cod != 0 && cod != 2002 {
        // 2002 (em alguns testes da AT) corresponde a "operação aceite com
        // observações". Tratado como sucesso. Restantes != 0 são falhas.
        return Err(AtError::AtFault { code: cod, msg });
    }
    match extract_series_info(xml)? {
        Some(info) => Ok(info),
        None => Err(AtError::Parse(format!(
            "{} sem infoSerie no payload (codResultOper={}, msg={})",
            outer, cod, msg
        ))),
    }
}

/// Parse de `consultarSeriesResp` que pode trazer 0..N `infoSerie`.
pub fn parse_consult_resp(xml: &str) -> Result<Vec<SeriesInfo>, AtError> {
    if let Some(fault) = extract_fault(xml)? {
        return Err(AtError::AtFault {
            code: 0,
            msg: fault,
        });
    }
    let (cod, msg) = extract_result_oper(xml)?;
    if cod != 0 && cod != 2002 {
        return Err(AtError::AtFault { code: cod, msg });
    }
    extract_all_series_infos(xml)
}

fn extract_fault(xml: &str) -> Result<Option<String>, AtError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut in_fault_string = false;
    let mut text: Option<String> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = e.name();
                let local = local_name(name.as_ref());
                if local == "faultstring" || local == "Reason" || local == "Text" {
                    in_fault_string = true;
                }
            }
            Ok(Event::Text(t)) if in_fault_string => {
                if let Ok(s) = t.unescape() {
                    text = Some(s.into_owned());
                }
            }
            Ok(Event::End(_)) => in_fault_string = false,
            Ok(Event::Eof) => break,
            Err(e) => return Err(AtError::Parse(format!("XML: {e}"))),
            _ => {}
        }
    }
    Ok(text)
}

fn extract_result_oper(xml: &str) -> Result<(i32, String), AtError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut cod: Option<i32> = None;
    let mut msg = String::new();
    let mut in_info = false;
    let mut current_field: Option<String> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref()).to_string();
                if local == "infoResultOper" {
                    in_info = true;
                } else if in_info {
                    current_field = Some(local);
                }
            }
            Ok(Event::Text(t)) if in_info => {
                if let Some(field) = &current_field {
                    let s = t.unescape().map_err(|e| AtError::Parse(e.to_string()))?;
                    match field.as_str() {
                        "codResultOper" => {
                            cod = s.parse().ok();
                        }
                        "msgResultOper" => {
                            msg = s.into_owned();
                        }
                        _ => {}
                    }
                }
            }
            Ok(Event::End(e)) => {
                let local = local_name(e.name().as_ref()).to_string();
                if local == "infoResultOper" {
                    in_info = false;
                }
                current_field = None;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AtError::Parse(format!("XML: {e}"))),
            _ => {}
        }
    }
    let cod = cod.ok_or_else(|| AtError::Parse("codResultOper ausente".into()))?;
    Ok((cod, msg))
}

fn extract_series_info(xml: &str) -> Result<Option<SeriesInfo>, AtError> {
    let mut all = extract_all_series_infos(xml)?;
    if all.is_empty() {
        Ok(None)
    } else {
        Ok(Some(all.remove(0)))
    }
}

fn extract_all_series_infos(xml: &str) -> Result<Vec<SeriesInfo>, AtError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut out = Vec::new();
    let mut depth_info = 0u32;
    let mut current_field: Option<String> = None;
    let mut buf: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let local = local_name(e.name().as_ref()).to_string();
                if local == "infoSerie" {
                    depth_info += 1;
                    buf.clear();
                } else if depth_info > 0 {
                    current_field = Some(local);
                }
            }
            Ok(Event::Text(t)) if depth_info > 0 => {
                if let Some(field) = &current_field {
                    let s = t.unescape().map_err(|e| AtError::Parse(e.to_string()))?;
                    buf.insert(field.clone(), s.into_owned());
                }
            }
            Ok(Event::End(e)) => {
                let local = local_name(e.name().as_ref()).to_string();
                if local == "infoSerie" {
                    if depth_info > 0 {
                        depth_info -= 1;
                    }
                    out.push(map_to_info(&buf)?);
                    buf.clear();
                }
                current_field = None;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(AtError::Parse(format!("XML: {e}"))),
            _ => {}
        }
    }
    Ok(out)
}

fn map_to_info(
    buf: &std::collections::HashMap<String, String>,
) -> Result<SeriesInfo, AtError> {
    fn req<'a>(
        buf: &'a std::collections::HashMap<String, String>,
        k: &str,
    ) -> Result<&'a String, AtError> {
        buf.get(k)
            .ok_or_else(|| AtError::Parse(format!("campo obrigatório ausente: {k}")))
    }
    Ok(SeriesInfo {
        serie: req(buf, "serie")?.clone(),
        tipo_serie: req(buf, "tipoSerie")?.clone(),
        classe_doc: req(buf, "classeDoc")?.clone(),
        tipo_doc: req(buf, "tipoDoc")?.clone(),
        num_inicial_seq: req(buf, "numInicialSeq")?
            .parse()
            .map_err(|_| AtError::Parse("numInicialSeq".into()))?,
        num_final_seq: buf.get("numFinalSeq").and_then(|s| s.parse().ok()),
        data_inicio_prev_utiliz: parse_date(req(buf, "dataInicioPrevUtiliz")?)?,
        seq_ultimo_doc_emitido: buf.get("seqUltimoDocEmitido").and_then(|s| s.parse().ok()),
        meio_processamento: req(buf, "meioProcessamento")?.clone(),
        num_cert_sw_fatur: req(buf, "numCertSWFatur")?
            .parse()
            .map_err(|_| AtError::Parse("numCertSWFatur".into()))?,
        cod_validacao_serie: req(buf, "codValidacaoSerie")?.clone(),
        data_registo: parse_date(req(buf, "dataRegisto")?)?,
        estado: req(buf, "estado")?.clone(),
        motivo_estado: buf.get("motivoEstado").cloned(),
        justificacao: buf.get("justificacao").cloned(),
        data_estado: req(buf, "dataEstado")?.clone(),
        nif_comunicou: req(buf, "nifComunicou")?.clone(),
    })
}

fn parse_date(s: &str) -> Result<NaiveDate, AtError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| AtError::Parse(format!("data {s}: {e}")))
}

fn local_name(qname: &[u8]) -> &str {
    let s = std::str::from_utf8(qname).unwrap_or("");
    match s.find(':') {
        Some(i) => &s[i + 1..],
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_registar_serie_envelope() {
        let req = RegistarSerieRequest {
            serie: "A".into(),
            tipo_serie: TipoSerie::Normal,
            classe_doc: "SI".into(),
            tipo_doc: "FS".into(),
            num_inicial_seq: 1,
            data_inicio_prev_utiliz: NaiveDate::from_ymd_opt(2026, 5, 28).unwrap(),
            num_cert_sw_fatur: 0,
            meio_processamento: "PF".into(),
        };
        let body = build_registar_serie(&req);
        assert!(body.contains("<serie>A</serie>"));
        assert!(body.contains("<tipoSerie>N</tipoSerie>"));
        assert!(body.contains("<numInicialSeq>1</numInicialSeq>"));
        assert!(body.contains("<dataInicioPrevUtiliz>2026-05-28</dataInicioPrevUtiliz>"));
        assert!(body.contains("<numCertSWFatur>0</numCertSWFatur>"));
        assert!(body.contains("<meioProcessamento>PF</meioProcessamento>"));
    }

    #[test]
    fn parses_series_resp_success() {
        let xml = r#"
        <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
          <soapenv:Body>
            <ns2:registarSerieResponse xmlns:ns2="http://at.gov.pt/">
              <registarSerieResp>
                <infoSerie>
                  <serie>A</serie>
                  <tipoSerie>N</tipoSerie>
                  <classeDoc>SI</classeDoc>
                  <tipoDoc>FS</tipoDoc>
                  <numInicialSeq>1</numInicialSeq>
                  <dataInicioPrevUtiliz>2026-05-28</dataInicioPrevUtiliz>
                  <meioProcessamento>PF</meioProcessamento>
                  <numCertSWFatur>0</numCertSWFatur>
                  <codValidacaoSerie>ABCD1234</codValidacaoSerie>
                  <dataRegisto>2026-05-28</dataRegisto>
                  <estado>A</estado>
                  <dataEstado>2026-05-28T10:00:00</dataEstado>
                  <nifComunicou>599999993</nifComunicou>
                </infoSerie>
                <infoResultOper>
                  <codResultOper>0</codResultOper>
                  <msgResultOper>OK</msgResultOper>
                </infoResultOper>
              </registarSerieResp>
            </ns2:registarSerieResponse>
          </soapenv:Body>
        </soapenv:Envelope>"#;
        let info = parse_series_resp(xml, "registarSerieResp").unwrap();
        assert_eq!(info.cod_validacao_serie, "ABCD1234");
        assert_eq!(info.serie, "A");
        assert_eq!(info.estado, "A");
    }

    #[test]
    fn parses_consult_resp_multiple() {
        let xml = r#"
        <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
          <soapenv:Body>
            <consultarSeriesResp xmlns="http://at.gov.pt/">
              <infoSerie>
                <serie>A</serie><tipoSerie>N</tipoSerie><classeDoc>SI</classeDoc>
                <tipoDoc>FS</tipoDoc><numInicialSeq>1</numInicialSeq>
                <dataInicioPrevUtiliz>2026-01-01</dataInicioPrevUtiliz>
                <meioProcessamento>PF</meioProcessamento><numCertSWFatur>0</numCertSWFatur>
                <codValidacaoSerie>AAA00001</codValidacaoSerie>
                <dataRegisto>2026-01-01</dataRegisto><estado>A</estado>
                <dataEstado>2026-01-01T00:00:00</dataEstado><nifComunicou>599999993</nifComunicou>
              </infoSerie>
              <infoSerie>
                <serie>B</serie><tipoSerie>N</tipoSerie><classeDoc>SI</classeDoc>
                <tipoDoc>FR</tipoDoc><numInicialSeq>1</numInicialSeq>
                <dataInicioPrevUtiliz>2026-01-01</dataInicioPrevUtiliz>
                <meioProcessamento>PF</meioProcessamento><numCertSWFatur>0</numCertSWFatur>
                <codValidacaoSerie>BBB00002</codValidacaoSerie>
                <dataRegisto>2026-01-01</dataRegisto><estado>F</estado>
                <dataEstado>2026-02-01T00:00:00</dataEstado><nifComunicou>599999993</nifComunicou>
              </infoSerie>
              <infoResultOper>
                <codResultOper>0</codResultOper>
                <msgResultOper>OK</msgResultOper>
              </infoResultOper>
            </consultarSeriesResp>
          </soapenv:Body>
        </soapenv:Envelope>"#;
        let list = parse_consult_resp(xml).unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].cod_validacao_serie, "AAA00001");
        assert_eq!(list[1].estado, "F");
    }

    #[test]
    fn parses_at_fault() {
        let xml = r#"
        <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">
          <soapenv:Body>
            <soapenv:Fault>
              <faultcode>soapenv:Client</faultcode>
              <faultstring>Credencial inválida</faultstring>
            </soapenv:Fault>
          </soapenv:Body>
        </soapenv:Envelope>"#;
        let err = parse_series_resp(xml, "registarSerieResp").unwrap_err();
        match err {
            AtError::AtFault { msg, .. } => assert!(msg.contains("Credencial")),
            _ => panic!("expected AtFault"),
        }
    }
}
