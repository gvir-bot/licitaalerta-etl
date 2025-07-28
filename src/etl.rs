use crate::models::Edital;

pub async fn run_etl() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Iniciando ETL...");

    // Coleta os dados da fonte
    let editais = crate::sources::pncp::extrair_editais().await?;

    // Salva em um arquivo JSON
    std::fs::write(
        "saida_editais.json",
        serde_json::to_string_pretty(&editais)?
    )?;

    log::info!("{} editais salvos em 'saida_editais.json'", editais.len());

    Ok(())
}
