use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Edital {
    pub id_source: String,
    pub id_original: String,
    pub ano: String,
    pub modalidade_id: String,
    pub status_id: String,
    pub info: String,
}


/// Modelo para Compras Ágiles de ChileCompra (API Compra Ágil V2)
#[derive(Debug, Serialize, Deserialize)]
pub struct CompraAgil {
        /// Código único de la Compra Ágil. Ej: 1057539-228-COT26
        pub codigo: String,
        /// Título o nombre del proceso de compra
        pub nombre: String,
        /// Código normalizado del estado: publicada, cerrada, desierta, cancelada, proveedor_seleccionado
        pub estado_codigo: String,
        /// Descripción legible del estado. Ej: "OC Emitida"
        pub estado_glosa: String,
        /// Fecha y hora de publicación (ISO-8601)
        pub fecha_publicacion: Option<String>,
        /// Fecha y hora de cierre del llamado vigente (ISO-8601)
        pub fecha_cierre: Option<String>,
        /// Timestamp del último cambio — usar para sincronización incremental (ISO-8601)
        pub fecha_ultimo_cambio: Option<String>,
        /// Monto disponible normalizado a CLP
        pub monto_disponible_clp: Option<f64>,
        /// Nombre del organismo público comprador
        pub organismo_comprador: Option<String>,
        /// RUT del organismo comprador
        pub rut_institucion: Option<String>,
        /// Código de región del organismo (1 a 16)
        pub region: Option<i32>,
        /// URL relativa para obtener el detalle. Ej: /v2/compra-agil/1057539-228-COT26
        pub link_detalle: Option<String>,
}
