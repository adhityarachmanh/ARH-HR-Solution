// src/services/job_position_service.rs
use crate::errors::AppError;
use crate::models::JobPosition;
use sqlx::PgPool;
use tracing::info;

const JOB_POSITION_FIELDS_SQL: &str = r#""JobPositionId" as job_position_id, "JobPositionName" as job_position_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by"#;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "JobPositions" ("JobPositionName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
    VALUES ($1, NOW(), $2, NOW(), $2) 
    RETURNING "JobPositionId", "JobPositionName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "JobPositions" 
    SET "JobPositionName" = $1, "UpdatedDate" = NOW(), "UpdatedBy" = $3 
    WHERE "JobPositionId" = $2 
    RETURNING "JobPositionId", "JobPositionName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

pub async fn get_all_job_positions(pool: &PgPool) -> Result<Vec<JobPosition>, AppError> {
    let positions = sqlx::query_as!(
        JobPosition,
        r#"SELECT "JobPositionId" as job_position_id, "JobPositionName" as job_position_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "JobPositions" ORDER BY "JobPositionName" ASC"#
    )
    .fetch_all(pool).await.map_err(AppError::DatabaseError)?;
    info!("Fetched {} job positions.", positions.len());
    Ok(positions)
}

pub async fn get_job_position_by_id(pool: &PgPool, id: i32) -> Result<JobPosition, AppError> {
    let position = sqlx::query_as!(
        JobPosition,
        r#"SELECT "JobPositionId" as job_position_id, "JobPositionName" as job_position_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "JobPositions" WHERE "JobPositionId" = $1"#,
        id
    ).fetch_one(pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Job Position not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(position)
}

pub async fn create_job_position(pool: &PgPool, name: &str) -> Result<JobPosition, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"JobPositions\" WHERE LOWER(\"JobPositionName\") = LOWER($1))", name
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Job Position name already exists".to_string(),
        ));
    }

    let new_position = sqlx::query_as::<_, JobPosition>(CREATE_QUERY_SQL)
        .bind(name)
        .bind("Admin")
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Job Position '{}' created with ID: {}",
        new_position.job_position_name.as_deref().unwrap_or("N/A"),
        new_position.job_position_id
    );
    Ok(new_position)
}

pub async fn update_job_position(
    pool: &PgPool,
    id: i32,
    new_name: &str,
) -> Result<JobPosition, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"JobPositions\" WHERE LOWER(\"JobPositionName\") = LOWER($1) AND \"JobPositionId\" != $2)",
        new_name, id
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Job Position name already exists".to_string(),
        ));
    }

    let updated_position = sqlx::query_as::<_, JobPosition>(UPDATE_QUERY_SQL)
        .bind(new_name)
        .bind(id)
        .bind("Admin")
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Job Position ID {} updated to '{}'",
        updated_position.job_position_id,
        updated_position
            .job_position_name
            .as_deref()
            .unwrap_or("N/A")
    );
    Ok(updated_position)
}

pub async fn delete_job_position(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(
        "DELETE FROM \"JobPositions\" WHERE \"JobPositionId\" = $1",
        id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Job Position not found for deletion".to_string(),
        ));
    }
    info!("Job Position ID {} deleted.", id);
    Ok(())
}
