// src/models/company.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Company {
    pub comp_id: i32,
    pub comp_name: Option<String>,
    pub company_prev_month_day_payroll: i32,
    pub company_current_month_day_payroll: i32,
}