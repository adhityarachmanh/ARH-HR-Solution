use crate::errors::AppError;
use crate::models::{Class, ClassFormData};
use sqlx::PgPool;
use tracing::info;

const SELECT_FIELDS: &str = r#"
    "ClassId" as class_id,
    "ClassCode" as class_code,
    "Description" as description,
    "CreatedDate" as created_date,
    "CreatedBy" as created_by,
    "UpdatedDate" as updated_date,
    "UpdatedBy" as updated_by
"#;

pub async fn create_class(pool: &PgPool, data: &ClassFormData, created_by: &str) -> Result<Class, AppError> {
    let row = sqlx::query_as::<_, Class>(&format!(
        r#"
        INSERT INTO "Classes" ("ClassCode", "Description", "CreatedBy") 
        VALUES ($1, $2, $3)
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(&data.class_code)
    .bind(&data.description)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Class '{}' created by {}", row.class_code.as_deref().unwrap_or("N/A"), created_by);
    Ok(row)
}

pub async fn get_all_classes(pool: &PgPool) -> Result<Vec<Class>, AppError> {
    let rows = sqlx::query_as::<_, Class>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Classes" ORDER BY "ClassCode" ASC"#
    ))
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_class_by_id(pool: &PgPool, id: i32) -> Result<Class, AppError> {
    let row = sqlx::query_as::<_, Class>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Classes" WHERE "ClassId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("Class with ID {} not found.", id)))
}

pub async fn get_class_by_code(pool: &PgPool, code: &str) -> Result<Class, AppError> {
    let row = sqlx::query_as::<_, Class>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Classes" WHERE "ClassCode" = $1"#
    ))
    .bind(code)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("Class with Code {} not found.", code)))
}

pub async fn update_class(
    pool: &PgPool,
    id: i32,
    data: &ClassFormData,
    updated_by: &str,
) -> Result<Class, AppError> {
    let row = sqlx::query_as::<_, Class>(&format!(
        r#"
        UPDATE "Classes"
        SET "ClassCode" = $2, "Description" = $3, "UpdatedDate" = NOW(), "UpdatedBy" = $4
        WHERE "ClassId" = $1
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(id)
    .bind(&data.class_code)
    .bind(&data.description)
    .bind(updated_by)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let row = row.ok_or(AppError::NotFound(format!("Class with ID {} not found.", id)))?;
    info!("Class ID {} updated by {}", id, updated_by);
    Ok(row)
}

pub async fn delete_class(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "Classes" WHERE "ClassId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Class with ID {} not found.", id)));
    }

    info!("Class ID {} deleted", id);
    Ok(())
}