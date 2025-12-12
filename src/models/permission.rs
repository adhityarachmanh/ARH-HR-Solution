use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HardcodedPermission {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct RolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Deserialize)]
pub struct ManagePermissionFormData {
    pub permissions: Option<Vec<i32>>,
}
