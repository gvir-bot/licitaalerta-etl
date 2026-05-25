use crate::models::Edital;

pub async fn run_etl() -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Iniciando ETL...");

    // --- Fuente 1: PNCP (Brasil) ---
    let editais = crate::sources::pncp::extrair_editais().await?;
        std::fs::write(
                    "saida_editais.json",
                    serde_json::to_string_pretty(&editais)?
                )?;
        log::info!("{} editais salvos em 'saida_editais.json'", editais.len());

    // --- Fuente 2: Compra Ágil V2 (Chile / ChileCompra) ---
    match crate::sources::compra_agil::extraer_compras_agiles().await {
                Ok(compras) => {
                                std::fs::write(
                                                    "saida_compras_agiles.json",
                                                    serde_json::to_string_pretty(&compras)?
                                                )?;
                                log::info!("{} compras ágiles guardadas en 'saida_compras_agiles.json'", compras.len());
                }
                Err(e) => {
                                log::error!("Error extrayendo Compras Ágiles: {}", e);
                }
    }

    Ok(())
}
