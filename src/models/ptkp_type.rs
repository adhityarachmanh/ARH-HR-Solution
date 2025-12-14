use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PtkpType {
    pub ptkp_type_id: i32,
    pub ptkp_code: Option<String>,
    pub ptkp_name: Option<String>,
    pub ptkp_amount: Option<f64>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    
    pub created_by: Option<String>,
    
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PtkpTypeFormData {
    pub ptkp_code: String,
    pub ptkp_name: String,
    pub ptkp_amount: f64,
}