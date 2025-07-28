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

