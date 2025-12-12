use crate::errors::AppError;
use crate::models::{Grade, GradeFormData};
use sqlx::PgPool;
use tracing::info;

const SELECT_FIELDS: &str = r#"
    "GradeId" as grade_id,
    "GradeCode" as grade_code,
    "GradeDescription" as grade_description,
    "CreatedDate" as created_date,
    "CreatedBy" as created_by,
    "UpdatedDate" as updated_date,
    "UpdatedBy" as updated_by
"#;

pub async fn create_grade(
    pool: &PgPool,
    data: &GradeFormData,
    created_by: &str,
) -> Result<Grade, AppError> {
    let row = sqlx::query_as::<_, Grade>(&format!(
        r#"
        INSERT INTO "Grades" ("GradeCode", "GradeDescription", "CreatedBy") 
        VALUES ($1, $2, $3)
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(&data.grade_code)
    .bind(&data.grade_description)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Grade '{}' created by {}", row.grade_code, created_by);
    Ok(row)
}

pub async fn count_grades(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("GradeId") FROM "Grades""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_grades_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Grade>, AppError> {
    let rows = sqlx::query_as::<_, Grade>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Grades" ORDER BY "GradeCode" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_grade_by_id(pool: &PgPool, id: i32) -> Result<Grade, AppError> {
    let row = sqlx::query_as::<_, Grade>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Grades" WHERE "GradeId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!(
        "Grade dengan ID {} tidak ditemukan.",
        id
    )))
}

pub async fn get_grade_by_code(pool: &PgPool, code: &str) -> Result<Grade, AppError> {
    let row = sqlx::query_as::<_, Grade>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Grades" WHERE "GradeCode" = $1"#
    ))
    .bind(code)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!(
        "Grade dengan Code {} tidak ditemukan.",
        code
    )))
}

pub async fn update_grade(
    pool: &PgPool,
    id: i32,
    data: &GradeFormData,
    updated_by: &str,
) -> Result<Grade, AppError> {
    let row = sqlx::query_as::<_, Grade>(&format!(
        r#"
        UPDATE "Grades"
        SET "GradeCode" = $2, "GradeDescription" = $3, "UpdatedDate" = NOW(), "UpdatedBy" = $4
        WHERE "GradeId" = $1
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(id)
    .bind(&data.grade_code)
    .bind(&data.grade_description)
    .bind(updated_by)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let row = row.ok_or(AppError::NotFound(format!(
        "Grade dengan ID {} tidak ditemukan.",
        id
    )))?;
    info!("Grade ID {} diperbarui oleh {}", id, updated_by);
    Ok(row)
}

pub async fn delete_grade(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "Grades" WHERE "GradeId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Grade dengan ID {} tidak ditemukan.",
            id
        )));
    }

    info!("Grade ID {} dihapus", id);
    Ok(())
}
