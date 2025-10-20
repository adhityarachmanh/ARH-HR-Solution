// src/services/auth_service.rs
use sqlx::PgPool;
use crate::models::User;
use crate::errors::AppError;

pub async fn authenticate_user(
    pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<User, AppError> {

    let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password_hash, created_at FROM users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .ok_or(AppError::InvalidCredentials)?;

    let is_valid = bcrypt::verify(password, &user.password_hash).unwrap_or(false);

    if is_valid {
        Ok(user)
    } else {
        Err(AppError::InvalidCredentials)
    }
}