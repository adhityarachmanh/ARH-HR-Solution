use crate::errors::AppError;
use crate::models::EmploymentStatus;
use sqlx::PgPool;
use tracing::info;

const STATUS_FIELDS_SQL: &str = r#""EmploymentStatusId" as employment_status_id, "EmploymentStatusName" as employment_status_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by"#;

const CREATE_QUERY_SQL_RETURN_ALIAS: &str = r#"
    INSERT INTO "EmploymentStatus" ("EmploymentStatusName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
    VALUES ($1, NOW(), $2, NOW(), $2) 
    RETURNING "EmploymentStatusId" as employment_status_id, "EmploymentStatusName" as employment_status_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by
"#;

const UPDATE_QUERY_SQL_RETURN_ALIAS: &str = r#"
    UPDATE "EmploymentStatus" 
    SET "EmploymentStatusName" = $1, "UpdatedDate" = NOW(), "UpdatedBy" = $3 
    WHERE "EmploymentStatusId" = $2 
    RETURNING "EmploymentStatusId" as employment_status_id, "EmploymentStatusName" as employment_status_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by
"#;

pub async fn count_employment_statuses(pool: &PgPool) -> Result<i64, AppError> {
    let count =
        sqlx::query_scalar!(r#"SELECT COUNT("EmploymentStatusId") FROM "EmploymentStatus""#)
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?
            .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_employment_statuses_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<EmploymentStatus>, AppError> {
    let sql_query = format!(
        r#"SELECT {} FROM "EmploymentStatus" ORDER BY "EmploymentStatusName" ASC LIMIT $1 OFFSET $2"#,
        STATUS_FIELDS_SQL
    );

    let statuses = sqlx::query_as::<_, EmploymentStatus>(&sql_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(statuses)
}

pub async fn get_all_employment_statuses(pool: &PgPool) -> Result<Vec<EmploymentStatus>, AppError> {
    let sql_query = format!(
        r#"SELECT {} FROM "EmploymentStatus" ORDER BY "EmploymentStatusName" ASC"#,
        STATUS_FIELDS_SQL
    );

    let statuses = sqlx::query_as::<_, EmploymentStatus>(&sql_query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(statuses)
}

pub async fn get_employment_status_by_id(
    pool: &PgPool,
    id: i32,
) -> Result<EmploymentStatus, AppError> {
    let sql_query = format!(
        r#"SELECT {} FROM "EmploymentStatus" WHERE "EmploymentStatusId" = $1"#,
        STATUS_FIELDS_SQL
    );

    let status = sqlx::query_as::<_, EmploymentStatus>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::InternalError("Employment Status not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })?;
    Ok(status)
}

pub async fn create_employment_status(
    pool: &PgPool,
    name: &str,
    created_by: &str,
) -> Result<EmploymentStatus, AppError> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM "EmploymentStatus" WHERE LOWER("EmploymentStatusName") = LOWER($1))"#, name
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Employment Status name already exists".to_string(),
        ));
    }

    let new_status = sqlx::query_as::<_, EmploymentStatus>(CREATE_QUERY_SQL_RETURN_ALIAS)
        .bind(name)
        .bind(created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Employment Status '{}' created with ID: {}",
        new_status.employment_status_name, new_status.employment_status_id
    );
    Ok(new_status)
}

pub async fn update_employment_status(
    pool: &PgPool,
    id: i32,
    new_name: &str,
    updated_by: &str,
) -> Result<EmploymentStatus, AppError> {
    let exists = sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM "EmploymentStatus" WHERE LOWER("EmploymentStatusName") = LOWER($1) AND "EmploymentStatusId" != $2)"#, 
        new_name, id
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Employment Status name already exists".to_string(),
        ));
    }

    let updated_status = sqlx::query_as::<_, EmploymentStatus>(UPDATE_QUERY_SQL_RETURN_ALIAS)
        .bind(new_name)
        .bind(id)
        .bind(updated_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Employment Status ID {} updated to '{}'",
        updated_status.employment_status_id, updated_status.employment_status_name
    );
    Ok(updated_status)
}

pub async fn delete_employment_status(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM "EmploymentStatus" WHERE "EmploymentStatusId" = $1"#,
        id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Employment Status not found for deletion".to_string(),
        ));
    }
    info!("Employment Status ID {} deleted.", id);
    Ok(())
}
