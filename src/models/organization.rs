// src/models/organization.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// Master Data: Organizations (Departemen/Unit)
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Organization {
    pub organization_id: i32,
    pub organization_name: String,
    pub manager_id: Option<i32>,
}