use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::serde::rfc3339::option;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Branch {
    pub branch_id: i32,
    pub comp_id: Option<i32>,
    pub time_zone_id: Option<i32>,
    pub branch_name: String,
    pub company_name: String,
    #[serde(with = "option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct BranchDetail {
    pub branch_id: i32,
    pub branch_name: String,
    pub company_name: Option<String>,
    pub timezone_name: Option<String>,
    pub comp_id: Option<i32>,
    pub time_zone_id: Option<i32>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
}


#[derive(Deserialize)]
pub struct BranchFormData {
    pub name: String,
    pub company_name: String,
    pub comp_id: Option<i32>,
    pub timezone_id: Option<i32>,
}

