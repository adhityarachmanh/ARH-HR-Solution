use crate::errors::AppError;
use crate::models::{PtkpType, PtkpTypeFormData};
use sqlx::PgPool;
use tracing::info;

const SELECT_FIELDS: &str = r#"
    "PtkpTypeId" as ptkp_type_id,
    "PtkpCode" as ptkp_code,
    "PtkpName" as ptkp_name,
    "PtkpAmount" as ptkp_amount,
    "CreatedDate" as created_date,
    "CreatedBy" as created_by,
    "UpdatedDate" as updated_date,
    "UpdatedBy" as updated_by
"#;


pub async fn create_ptkp_type(pool: &PgPool, data: &PtkpTypeFormData, created_by: &str) -> Result<PtkpType, AppError> {
    let row = sqlx::query_as::<_, PtkpType>(&format!(
        r#"
        INSERT INTO "PtkpTypes" ("PtkpCode", "PtkpName", "PtkpAmount", "CreatedBy") 
        VALUES ($1, $2, $3, $4)
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(&data.ptkp_code)
    .bind(&data.ptkp_name)
    .bind(data.ptkp_amount)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("PTKP Type '{}' created by {}", row.ptkp_code.as_deref().unwrap_or("N/A"), created_by);
    Ok(row)
}

pub async fn count_ptkp_types(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("PtkpTypeId") FROM "PtkpTypes""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_ptkp_types_paginated(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<PtkpType>, AppError> {
    let rows = sqlx::query_as::<_, PtkpType>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "PtkpTypes" ORDER BY "PtkpCode" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_all_ptkp_types(pool: &PgPool) -> Result<Vec<PtkpType>, AppError> {
    let rows = sqlx::query_as::<_, PtkpType>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "PtkpTypes" ORDER BY "PtkpCode" ASC"#
    ))
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_ptkp_type_by_id(pool: &PgPool, id: i32) -> Result<PtkpType, AppError> {
    let row = sqlx::query_as::<_, PtkpType>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "PtkpTypes" WHERE "PtkpTypeId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("PTKP Type with ID {} not found.", id)))
}

pub async fn get_ptkp_type_by_code(pool: &PgPool, code: &str) -> Result<PtkpType, AppError> {
    let row = sqlx::query_as::<_, PtkpType>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "PtkpTypes" WHERE "PtkpCode" = $1"#
    ))
    .bind(code)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("PTKP Type with Code {} not found.", code)))
}

pub async fn update_ptkp_type(
    pool: &PgPool,
    id: i32,
    data: &PtkpTypeFormData,
    updated_by: &str,
) -> Result<PtkpType, AppError> {
    let row = sqlx::query_as::<_, PtkpType>(&format!(
        r#"
        UPDATE "PtkpTypes"
        SET "PtkpCode" = $2, "PtkpName" = $3, "PtkpAmount" = $4, "UpdatedDate" = NOW(), "UpdatedBy" = $5
        WHERE "PtkpTypeId" = $1
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(id)
    .bind(&data.ptkp_code)
    .bind(&data.ptkp_name)
    .bind(data.ptkp_amount)
    .bind(updated_by)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let row = row.ok_or(AppError::NotFound(format!("PTKP Type with ID {} not found.", id)))?;
    info!("PTKP Type ID {} updated by {}", id, updated_by);
    Ok(row)
}

pub async fn delete_ptkp_type(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "PtkpTypes" WHERE "PtkpTypeId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("PTKP Type with ID {} not found.", id)));
    }

    info!("PTKP Type ID {} deleted", id);
    Ok(())
}