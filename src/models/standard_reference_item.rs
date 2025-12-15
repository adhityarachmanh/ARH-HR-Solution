use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct StandardReferenceItem {
    pub item_id: String,
    pub standard_reference_id: Option<String>,
    pub item_name: Option<String>,
    pub is_active: Option<bool>,
    pub note: Option<String>,
    
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    
    pub created_by: Option<String>,
    
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StandardReferenceItemFormData {
    pub item_id: String,
    pub item_name: String,
    pub is_active: bool,
    pub note: Option<String>,
}