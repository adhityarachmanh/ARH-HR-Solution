// src/models/user.rs
use serde::{Serialize, Deserialize};
use time::OffsetDateTime; 

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>, 
    #[serde(skip)]
    pub password_hash: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    pub is_active: bool,
    pub last_login: Option<OffsetDateTime>,
}
