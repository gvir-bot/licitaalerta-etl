use crate::models::Edital;
use serde_json::Value;

pub async fn extrair_editais() -> Result<Vec<Edital>, Box<dyn std::error::Error>> {
    let mut editais = Vec::new();
    let base_url = "https://pncp.gov.br/api/consulta/v1/contratacoes/publicacao";
    
    let data_inicial = "20250601";
    let data_final = "20250728";
    let uf = "GO";
    let modalidade = 8;
    let tamanho_pagina = 10;
    let paginas = 5;

    for pagina in 1..=paginas {
        let url = format!(
            "{base_url}?dataInicial={data_inicial}&dataFinal={data_final}&codigoModalidadeContratacao={modalidade}&uf={uf}&pagina={pagina}&tamanhoPagina={tamanho_pagina}"
        );

        log::info!("Requisitando página {}: {}", pagina, url);

        let resp = match reqwest::get(&url).await {
            Ok(r) => r,
            Err(e) => {
                log::warn!("Erro na requisição da página {}: {}", pagina, e);
                continue;
            }
        };

        let text = match resp.text().await {
            Ok(t) => t,
            Err(e) => {
                log::warn!("Erro ao ler resposta da página {}: {}", pagina, e);
                continue;
            }
        };

        let json: Value = serde_json::from_str(&text)?;

        if let Some(items) = json["data"].as_array() {
            log::info!("Página {} retornou {} editais", pagina, items.len());
            for item in items {
                let edital = Edital {                    
                    id_source: item["numeroControlePNCP"].as_str().unwrap_or_default().to_string(),
                    id_original: item["numeroCompra"].as_str().unwrap_or("Desconhecido").to_string(),
                    ano: item["anoCompra"].as_str().unwrap_or("-").to_string(),
                    modalidade_id: item["modalidadeId"].as_str().unwrap_or("").to_string(),
                    status_id: item["situacaoCompraId"].as_str().unwrap_or("").to_string(),
                    info: item["objetoCompra"].as_str().unwrap_or("").to_string(),
                };
                editais.push(edital);
            }
        } else {
            log::warn!("Nenhum item encontrado na página {}", pagina);
            log::info!("Estrutura JSON recebida: {}", json);
        }
    }

    log::info!("Total de editais extraídos: {}", editais.len());

    Ok(editais)
}
