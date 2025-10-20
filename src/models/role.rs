// src/models/role.rs
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
}