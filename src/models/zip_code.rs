// src/models/zip_code.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::PrimitiveDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ZipCode {
    pub zip_code_id: String,
    pub street_name: Option<String>,
    pub district: String,
    pub county: String,
    pub city: String,
    pub sr_province: Option<String>,

    // Latitude & Longitude: Option<f64> karena boleh 0.0 (bukan null)
    // Tapi di DB: DEFAULT 0.0 → akan masuk sebagai 0.0, bukan NULL
    // → Bisa pakai f64 langsung jika tidak ingin Option
    pub latitude: f64,
    pub longitude: f64,

    // TIMESTAMP WITHOUT TIME ZONE → PrimitiveDateTime
    // Di migrasi: '2020-08-13 15:55:21.560' → valid
    #[serde(with = "time::serde::timestamp")]
    pub last_update_date_time: PrimitiveDateTime,

    pub last_update_by_user_id: Option<String>,
    pub zip_postal_code: String, // NOT NULL di DB → wajib String
}