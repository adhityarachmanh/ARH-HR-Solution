use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobLevel {
    pub job_level_id: i32,
    pub job_level_name: Option<String>,
    pub job_level_order: Option<i32>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,

    pub created_by: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,

    pub updated_by: Option<String>,
}


#[derive(Deserialize)]
pub struct JobLevelFormData {
    pub name: String,
    pub order: Option<i32>,
}
