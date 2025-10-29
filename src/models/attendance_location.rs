
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AttendanceLocation {
    pub attendance_location_id: i32,
    pub time_zone_id: Option<i32>,
    pub location_name: Option<String>,
    pub is_flexible: Option<bool>,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
    pub radius_tolerance_in_meter: Option<i32>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceLocationDetail {
    pub attendance_location_id: i32,
    pub time_zone_id: Option<i32>,
    pub location_name: Option<String>,
    pub is_flexible: Option<bool>,
    pub latitude: Option<BigDecimal>,
    pub longitude: Option<BigDecimal>,
    pub radius_tolerance_in_meter: Option<i32>,
    pub timezone_name: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
}
