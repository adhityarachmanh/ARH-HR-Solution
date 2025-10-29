use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Company {
    pub comp_id: i32,
    pub comp_name: Option<String>,
    pub comp_address: Option<String>,
    pub comp_zip_code: Option<String>,
    pub comp_phone_number: Option<String>,
    pub comp_mobile_number: Option<String>,
    pub comp_email: Option<String>,
    pub comp_website: Option<String>,
    pub comp_retirement_age: Option<i32>,

    pub prev_month_day_payroll: i32,
    pub current_month_day_payroll: i32,

    pub bpjstk_pendapatan_pk: Option<f64>,
    pub bpjsks_pendapatan_pk: Option<f64>,
    pub bpjstk_pengurangan_pk: Option<f64>,
    pub bpjsks_pengurangan_pk: Option<f64>,
    pub bpjstk_pengurangan_pekerja: Option<f64>,
    pub bpjsks_pengurangan_pekerja: Option<f64>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}
