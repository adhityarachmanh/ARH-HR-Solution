// src/services/user_service.rs
use sqlx::PgPool;
use crate::models::{User};
use crate::errors::AppError;
use tracing::info;

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash, created_at, is_active, last_login FROM users ORDER BY id ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?; 

    Ok(users)
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password: &str,
    role_ids: &[i32],
) -> Result<User, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
        username
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) { return Err(AppError::UsernameExists); }

    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?;

    let mut tx = pool.begin().await.map_err(AppError::DatabaseError)?;

    let new_user_record = sqlx::query_as!(
        User,
        "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, email, password_hash, created_at, is_active, last_login",
        username,
        hashed_password
    )
    .fetch_one(&mut *tx).await.map_err(AppError::DatabaseError)?;

    for role_id in role_ids {
        sqlx::query!("INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)",
            new_user_record.id, role_id)
        .execute(&mut *tx).await.map_err(AppError::DatabaseError)?;
    }
    
    tx.commit().await.map_err(AppError::DatabaseError)?;

    info!("User '{}' created with ID: {}", new_user_record.username, new_user_record.id);
    Ok(new_user_record)
}