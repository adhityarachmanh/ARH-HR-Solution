use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub display_name: String,
}


#[derive(Deserialize)]
pub struct AddRoleFormData {
    pub name: String,
}