// src/models/holiday.rs
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Holiday {
    pub holiday_id: i32,
    pub holiday_name: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub holiday_date: Option<OffsetDateTime>,
    pub holiday_description: Option<String>,
    pub is_national_holiday: Option<bool>,
    pub is_company_holiday: Option<bool>,
    pub is_special_holiday: Option<bool>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
    pub holiday_year: Option<i32>,
    pub is_mass_leave: bool,
}
