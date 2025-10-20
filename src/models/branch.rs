// src/models/branch.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

// Master Data: Branches
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Branch {
    pub branch_id: i32,
    pub comp_id: Option<i32>,
    pub time_zone_id: Option<i32>,
    pub branch_name: String,
}