
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ZipCode {
    pub zip_code_id: String,

    pub street_name: Option<String>,
    pub district: String,
    pub county: String,
    pub city: String,
    pub sr_province: Option<String>,
    
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_update_date_time: Option<OffsetDateTime>,
    
    pub last_update_by_user_id: Option<String>,
    pub zip_postal_code: Option<String>,
}

#[derive(Deserialize)]
pub struct ZipCodeFormData {
    pub zip_code_id: String,
    pub street_name: Option<String>,
    pub district: String,
    pub county: String,
    pub city: String,
    pub sr_province: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub zip_postal_code: Option<String>,
}
