use crate::models::CompraAgil;
use serde_json::Value;

const BASE_URL: &str = "https://api2.mercadopublico.cl";
const ENDPOINT_LISTADO: &str = "/v2/compra-agil";

/// Extrae Compras Ágiles desde la API V2 de Mercado Público (ChileCompra).
///
/// Requiere la variable de entorno COMPRA_AGIL_TICKET con el ticket de acceso.
/// Filtra por ventana de cambios usando ttl_cambio_ms (por defecto: últimas 24 h).
/// Maneja paginación automáticamente y respeta el límite de cuota 429.
pub async fn extraer_compras_agiles() -> Result<Vec<CompraAgil>, Box<dyn std::error::Error>> {
      let ticket = std::env::var("COMPRA_AGIL_TICKET")
                .map_err(|_| "Variable de entorno COMPRA_AGIL_TICKET no definida")?;

    let mut compras: Vec<CompraAgil> = Vec::new();
      let mut numero_pagina: u32 = 1;
      let tamano_pagina: u32 = 50;

    // Ventana de cambios: últimas 24 horas en milisegundos
    let ttl_cambio_ms: u64 = 24 * 60 * 60 * 1000;

    loop {
              let url = format!(
                            "{}{}", BASE_URL, ENDPOINT_LISTADO
                        );

          log::info!("Consultando página {} de Compra Ágil...", numero_pagina);

          let client = reqwest::Client::new();
              let resp = match client
                            .get(&url)
                            .header("ticket", &ticket)
                            .query(&[
                                              ("ttl_cambio_ms", ttl_cambio_ms.to_string()),
                                              ("tamano_pagina", tamano_pagina.to_string()),
                                              ("numero_pagina", numero_pagina.to_string()),
                                              ("ordenar_por", "FechaUltimaModificacion".to_string()),
                                          ])
                            .send()
                            .await
                {
                              Ok(r) => r,
                              Err(e) => {
                                                log::warn!("Error en request página {}: {}", numero_pagina, e);
                                                break;
                              }
                };

          // Manejo de cuota diaria
          if resp.status().as_u16() == 429 {
                        log::warn!("Cuota diaria alcanzada (HTTP 429). Deteniendo extracción.");
                        break;
          }

          if !resp.status().is_success() {
                        log::error!("Error HTTP {}: {}", resp.status(), url);
                        break;
          }

          let text = match resp.text().await {
                        Ok(t) => t,
                        Err(e) => {
                                          log::warn!("Error leyendo respuesta página {}: {}", numero_pagina, e);
                                          break;
                        }
          };

          let json: Value = serde_json::from_str(&text)?;

          // Verificar éxito
          if json["success"].as_str() != Some("OK") {
                        log::warn!("La API retornó success != OK: {}", json);
                        break;
          }

          let payload = &json["payload"];
              let items = match payload["items"].as_array() {
                            Some(arr) => arr,
                            None => {
                                              log::warn!("No se encontraron items en la respuesta");
                                              break;
                            }
              };

          log::info!("Página {} retornó {} compras ágiles", numero_pagina, items.len());

          for item in items {
                        let compra = CompraAgil {
                                          codigo: item["codigo"].as_str().unwrap_or_default().to_string(),
                                          nombre: item["nombre"].as_str().unwrap_or_default().to_string(),
                                          estado_codigo: item["estado"]["codigo"].as_str().unwrap_or_default().to_string(),
                                          estado_glosa: item["estado"]["glosa"].as_str().unwrap_or_default().to_string(),
                                          fecha_publicacion: item["fechas"]["fecha_publicacion"].as_str().map(|s| s.to_string()),
                                          fecha_cierre: item["fechas"]["fecha_cierre"].as_str().map(|s| s.to_string()),
                                          fecha_ultimo_cambio: item["fechas"]["fecha_ultimo_cambio"].as_str().map(|s| s.to_string()),
                                          monto_disponible_clp: item["montos"]["monto_disponible_clp"].as_f64(),
                                          organismo_comprador: item["institucion"]["organismo_comprador"].as_str().map(|s| s.to_string()),
                                          rut_institucion: item["institucion"]["rut"].as_str().map(|s| s.to_string()),
                                          region: item["institucion"]["region"].as_i64().map(|v| v as i32),
                                          link_detalle: item["links"]["detalle"].as_str().map(|s| s.to_string()),
                        };
                        compras.push(compra);
          }

          // Paginación
          let paginacion = &payload["paginacion"];
              let total_paginas = paginacion["total_paginas"].as_u64().unwrap_or(1);
              if numero_pagina as u64 >= total_paginas {
                            log::info!("Paginación completa: {} de {} páginas", numero_pagina, total_paginas);
                            break;
              }
              numero_pagina += 1;
    }

    log::info!("Total Compras Ágiles extraídas: {}", compras.len());
      Ok(compras)
}
