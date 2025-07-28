mod models;
mod etl;
mod sources;

#[tokio::main]
async fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    if let Err(e) = etl::run_etl().await {
        eprintln!("Erro ao executar ETL: {}", e);
    }
}

