// src/services/permission_service.rs
use sqlx::PgPool;
use crate::models::HardcodedPermission; 
use crate::errors::AppError;

lazy_static::lazy_static! {
    static ref MASTER_PERMISSIONS: Vec<HardcodedPermission> = vec![
        HardcodedPermission { id: 1, name: "manage_users".to_string(), description: "Allows full CRUD management over user accounts.".to_string() },
        HardcodedPermission { id: 2, name: "manage_roles".to_string(), description: "Allows creating, editing, and deleting user roles and permissions.".to_string() },
        HardcodedPermission { id: 3, name: "manage_content".to_string(), description: "Allows creating and editing general site content (e.g., blog posts, pages).".to_string() },
        HardcodedPermission { id: 4, name: "view_reports".to_string(), description: "Allows viewing analytics and reports.".to_string() },
    ];
}

pub async fn get_all_permissions(_pool: &PgPool) -> Result<Vec<HardcodedPermission>, AppError> {
    Ok(MASTER_PERMISSIONS.clone())
}

pub async fn get_permissions_for_role(pool: &PgPool, role_id: i32) -> Result<Vec<i32>, AppError> {
    let permission_ids = sqlx::query_scalar!(
        "SELECT permission_id as \"permission_id!\" FROM role_permissions WHERE role_id = $1",
        role_id
    )
    .fetch_all(pool).await.map_err(AppError::DatabaseError)?;

    Ok(permission_ids)
}