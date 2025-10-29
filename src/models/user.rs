use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip)]
    pub password_hash: String,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    pub is_active: bool,

    #[serde(with = "time::serde::rfc3339::option")]
    pub last_login: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserListDetail {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,

    pub role_names: Option<String>,
}
