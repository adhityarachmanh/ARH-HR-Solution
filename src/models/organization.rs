// src/models/organization.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use time::OffsetDateTime; 

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Organization {
    pub organization_id: i32,
    pub organization_name: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}