// src/models/branch.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Branch {
    pub branch_id: i32,
    pub comp_id: Option<i32>,
    pub time_zone_id: Option<i32>,
    pub branch_name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct BranchDetail {
    pub branch_id: i32,
    pub branch_name: String,
    pub company_name: Option<String>,
    pub timezone_name: Option<String>,
    pub comp_id: Option<i32>,
    pub time_zone_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CompanyDropdown {
    pub comp_id: i32,
    pub comp_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TimeZoneDropdown {
    pub time_zone_id: i32,
    pub time_zone_name: String,
}