use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Grade {
    pub grade_id: i32,
    pub grade_code: String,
    pub grade_description: String,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,

    pub created_by: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,

    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GradeFormData {
    pub grade_code: String,
    pub grade_description: String,
}
