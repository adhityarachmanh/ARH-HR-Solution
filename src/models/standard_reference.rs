use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StandardReference {
    pub standard_reference_id: String,
    pub standard_reference_name: String,
    pub is_active: Option<bool>,
    
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    
    pub created_by: Option<String>,
    
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandardReferenceFormData {
    pub standard_reference_id: String,
    pub standard_reference_name: String,
    pub is_active: bool,
}