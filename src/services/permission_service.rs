use crate::errors::AppError;
use crate::models::HardcodedPermission;
use sqlx::PgPool;

lazy_static::lazy_static! {
    static ref MASTER_PERMISSIONS: Vec<HardcodedPermission> = vec![
        HardcodedPermission {
            id: 1,
            name: "manage_system_users".to_string(),
            description: "Allows full CRUD management over system user accounts, including role assignment and deletion.".to_string()
        },
        HardcodedPermission {
            id: 2,
            name: "manage_system_roles".to_string(),
            description: "Allows creating, editing, and deleting user roles and assigning permissions to those roles.".to_string()
        },
        HardcodedPermission {
            id: 3,
            name: "manage_core_master_data".to_string(),
            description: "Allows CRUD management for core HR entities: Companies, Branches, Organizations, Timezones, Job Positions, and Job Levels.".to_string()
        },
        HardcodedPermission {
            id: 4,
            name: "manage_attendance_settings".to_string(),
            description: "Allows CRUD management for Attendance Locations (Geofencing, Radius) and Holiday synchronization.".to_string()
        },
        HardcodedPermission {
            id: 5,
            name: "view_hr_reports".to_string(),
            description: "Allows access to high-level HR reports and analytics (e.g., Attendance Summaries, Payroll Previews).".to_string()
        },
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
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(permission_ids)
}
