use crate::errors::AppError;
use crate::models::Role;
use sqlx::PgPool;
use tracing::info;

const ROLE_FIELDS_SQL_LIST: &str = r#"id, name, display_name"#;

pub async fn count_roles(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(id) FROM roles"#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_roles_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Role>, AppError> {
    let roles = sqlx::query_as!(
        Role,
        "SELECT id, name, display_name FROM roles ORDER BY id ASC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(roles)
}

pub async fn get_all_roles(pool: &PgPool) -> Result<Vec<Role>, AppError> {
    let roles = sqlx::query_as!(
        Role,
        "SELECT id, name, display_name FROM roles ORDER BY id ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(roles)
}

pub async fn get_role_by_id(pool: &PgPool, role_id: i32) -> Result<Role, AppError> {
    let role = sqlx::query_as!(
        Role,
        "SELECT id, name, display_name FROM roles WHERE id = $1",
        role_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Role not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(role)
}

pub async fn create_role(pool: &PgPool, name: &str) -> Result<Role, AppError> {
    let normalized_name = name.trim().to_string();
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM roles WHERE LOWER(name) = LOWER($1))",
        normalized_name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError("Role already exists".to_string()));
    }

    let new_role = sqlx::query_as!(
        Role,
        "INSERT INTO roles (name, display_name) VALUES ($1, $1) RETURNING id, name, display_name",
        normalized_name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Role '{}' created with ID: {}", new_role.name, new_role.id);
    Ok(new_role)
}

pub async fn update_role(pool: &PgPool, role_id: i32, new_name: &str) -> Result<Role, AppError> {
    let normalized_name = new_name.trim().to_string();
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM roles WHERE LOWER(name) = LOWER($1) AND id != $2)",
        normalized_name,
        role_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Role name already exists".to_string(),
        ));
    }

    let updated_role = sqlx::query_as!(
        Role,
        "UPDATE roles SET name = $1, display_name = $1 WHERE id = $2 RETURNING id, name, display_name",
        normalized_name,
        role_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!(
        "Role ID {} updated to '{}'",
        updated_role.id, updated_role.name
    );
    Ok(updated_role)
}

pub async fn delete_role(pool: &PgPool, role_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM roles WHERE id = $1", role_id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Role not found for deletion".to_string(),
        ));
    }
    info!("Role ID {} deleted.", role_id);
    Ok(())
}
