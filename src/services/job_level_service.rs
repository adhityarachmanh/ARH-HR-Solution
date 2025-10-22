use crate::errors::AppError;
use crate::models::JobLevel;
use sqlx::PgPool;
use tracing::info;

const JOB_LEVEL_FIELDS_SQL: &str = r#""JobLevelId" as "job_level_id", "JobLevelName" as "job_level_name", "JobLevelOrder" as "job_level_order", "CreatedDate" as "created_date", "CreatedBy" as "created_by", "UpdatedDate" as "updated_date", "UpdatedBy" as "updated_by""#;

pub async fn get_all_job_levels(pool: &PgPool) -> Result<Vec<JobLevel>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"JobLevels\" ORDER BY \"JobLevelOrder\" ASC, \"JobLevelId\" ASC",
        JOB_LEVEL_FIELDS_SQL
    );

    let levels = sqlx::query_as::<_, JobLevel>(&sql_query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(levels)
}

pub async fn get_job_level_by_id(pool: &PgPool, id: i32) -> Result<JobLevel, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"JobLevels\" WHERE \"JobLevelId\" = $1",
        JOB_LEVEL_FIELDS_SQL
    );

    let level = sqlx::query_as::<_, JobLevel>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Job Level not found".to_string()),
            _ => AppError::DatabaseError(e),
        })?;

    Ok(level)
}

pub async fn create_job_level(
    pool: &PgPool,
    name: &str,
    order: Option<i32>,
    created_by: &str,
) -> Result<JobLevel, AppError> {
    let new_level = sqlx::query_as::<_, JobLevel>(&format!(
        r#"
        INSERT INTO "JobLevels" ("JobLevelName", "JobLevelOrder", "CreatedBy", "UpdatedBy")
        VALUES ($1, $2, $3, $3)
        RETURNING {}
        "#,
        JOB_LEVEL_FIELDS_SQL
    ))
    .bind(name)
    .bind(order)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!(
        "Job Level '{}' created with ID: {}",
        name, new_level.job_level_id
    );
    Ok(new_level)
}

pub async fn update_job_level(
    pool: &PgPool,
    id: i32,
    name: &str,
    order: Option<i32>,
    updated_by: &str,
) -> Result<JobLevel, AppError> {
    let updated_level = sqlx::query_as::<_, JobLevel>(&format!(
        r#"
        UPDATE "JobLevels"
        SET "JobLevelName" = $1, "JobLevelOrder" = $2, "UpdatedDate" = NOW(), "UpdatedBy" = $3
        WHERE "JobLevelId" = $4
        RETURNING {}
        "#,
        JOB_LEVEL_FIELDS_SQL
    ))
    .bind(name)
    .bind(order)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            AppError::InternalError("Job Level not found for update".to_string())
        }
        _ => AppError::DatabaseError(e),
    })?;

    info!("Job Level ID {} updated to '{}'", id, name);
    Ok(updated_level)
}

pub async fn delete_job_level(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "JobLevels" WHERE "JobLevelId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Job Level not found for deletion".to_string(),
        ));
    }
    info!("Job Level ID {} deleted.", id);
    Ok(())
}
