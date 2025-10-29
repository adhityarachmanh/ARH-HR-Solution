use crate::errors::AppError;
use crate::models::{User, UserListDetail};
use bcrypt;
use sqlx::PgPool;
use tracing::info;

const USER_FIELDS_SQL: &str =
    r#"id, username, email, password_hash, created_at, is_active, last_login"#;

const USER_LIST_DETAIL_FIELDS: &str =
    r#"u.id, u.username, u.email, u.created_at, STRING_AGG(r.display_name, ', ') AS role_names"#;

pub async fn count_users(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(id) FROM users"#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_users_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<UserListDetail>, AppError> {
    let sql_query = format!(
        r#"
        SELECT 
            {}
        FROM users u
        LEFT JOIN user_roles ur ON u.id = ur.user_id
        LEFT JOIN roles r ON ur.role_id = r.id
        GROUP BY u.id, u.username, u.email, u.created_at
        ORDER BY u.username ASC
        LIMIT $1 OFFSET $2
        "#,
        USER_LIST_DETAIL_FIELDS
    );

    let users = sqlx::query_as::<_, UserListDetail>(&sql_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(users)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM users ORDER BY username ASC",
        USER_FIELDS_SQL
    );

    let users = sqlx::query_as::<_, User>(&sql_query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(users)
}

pub async fn get_user_role_ids(pool: &PgPool, user_id: i64) -> Result<Vec<i32>, AppError> {
    let role_ids = sqlx::query_scalar!(
        r#"SELECT role_id FROM user_roles WHERE user_id = $1"#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(role_ids)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i64) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, created_at, is_active, last_login FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("User not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(user)
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
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::UsernameExists);
    }

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
        sqlx::query!(
            "INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)",
            new_user_record.id,
            role_id
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;
    }

    tx.commit().await.map_err(AppError::DatabaseError)?;

    info!(
        "User '{}' created with ID: {}",
        new_user_record.username, new_user_record.id
    );
    Ok(new_user_record)
}

pub async fn update_user(
    pool: &PgPool,
    user_id: i64,
    new_username: &str,
    new_email: Option<&str>,
    new_role_ids: &[i32],
) -> Result<User, AppError> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND id != $2)"#,
        new_username,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::UsernameExists);
    }

    let mut tx = pool.begin().await.map_err(AppError::DatabaseError)?;

    let updated_user = sqlx::query_as!(
        User,
        r#"UPDATE users SET username = $1, email = $2 WHERE id = $3 RETURNING id, username, email, password_hash, created_at, is_active, last_login"#,
        new_username,
        new_email,
        user_id
    )
    .fetch_one(&mut *tx).await.map_err(AppError::DatabaseError)?;

    sqlx::query!(r#"DELETE FROM user_roles WHERE user_id = $1"#, user_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;

    for role_id in new_role_ids {
        sqlx::query!(
            r#"INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)"#,
            user_id,
            role_id
        )
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;
    }

    tx.commit().await.map_err(AppError::DatabaseError)?;

    info!("User ID {} updated to '{}'", user_id, updated_user.username);
    Ok(updated_user)
}

pub async fn delete_user(pool: &PgPool, user_id: i64) -> Result<(), AppError> {
    let mut tx = pool.begin().await.map_err(AppError::DatabaseError)?;

    sqlx::query!(r#"DELETE FROM user_roles WHERE user_id = $1"#, user_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;

    let result = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, user_id)
        .execute(&mut *tx)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        tx.rollback().await.map_err(AppError::DatabaseError)?;
        return Err(AppError::InternalError(
            "User not found for deletion.".to_string(),
        ));
    }

    tx.commit().await.map_err(AppError::DatabaseError)?;

    info!("User ID {} deleted.", user_id);
    Ok(())
}
