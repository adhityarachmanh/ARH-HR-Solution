// src/models/timezone.rs
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TimeZone {
    pub time_zone_id: i32,
    pub time_zone_name: String,
}