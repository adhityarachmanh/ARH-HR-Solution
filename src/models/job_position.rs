// src/models/job_position.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct JobPosition {
    pub job_position_id: i32,
    pub job_position_name: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}