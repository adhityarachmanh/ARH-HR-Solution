// src/services/zip_code_service.rs

use crate::errors::AppError;
use crate::models::ZipCode;
use sqlx::PgPool;
use tracing::info;

const SELECT_FIELDS: &str = r#"
    "ZipCodeId" as zip_code_id,
    "StreetName" as street_name,
    "District" as district,
    "County" as county,
    "City" as city,
    "SRProvince" as sr_province,
    "Latitude" as latitude,
    "Longitude" as longitude,
    "LastUpdateDateTime" as last_update_date_time,
    "LastUpdateByUserID" as last_update_by_user_id,
    "ZipPostalCode" as zip_postal_code
"#;

pub async fn count_zip_codes(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM "ZipCodes""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn search_zip_codes(pool: &PgPool, query: &str) -> Result<Vec<ZipCode>, AppError> {
    let q = query.trim();
    if q.is_empty() {
        return Ok(vec![]);
    }

    let pattern = format!("%{}%", q.to_lowercase());
    let exact = q.to_uppercase();

    let rows = sqlx::query_as::<_, ZipCode>(&format!(
        r#"
        SELECT {SELECT_FIELDS}
        FROM "ZipCodes"
        WHERE LOWER("City") LIKE $1
           OR LOWER("District") LIKE $1
           OR "ZipPostalCode" ILIKE $1
        ORDER BY
            CASE WHEN "ZipPostalCode" ILIKE $2 THEN 0 ELSE 1 END,
            "City" ASC
        LIMIT 30
        "#
    ))
    .bind(&pattern)
    .bind(&exact)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_all_zip_codes_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<ZipCode>, AppError> {
    let rows = sqlx::query_as::<_, ZipCode>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "ZipCodes" ORDER BY "ZipCodeId" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_zip_code_by_id(pool: &PgPool, id: &str) -> Result<ZipCode, AppError> {
    let row = sqlx::query_as::<_, ZipCode>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "ZipCodes" WHERE "ZipCodeId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound("Zip Code tidak ditemukan".into()))
}

pub async fn create_zip_code(
    pool: &PgPool,
    data: &ZipCode,
    created_by: &str,
) -> Result<ZipCode, AppError> {
    let row = sqlx::query_as::<_, ZipCode>(&format!(
        r#"
        INSERT INTO "ZipCodes" (
            "ZipCodeId", "StreetName", "District", "County", "City",
            "SRProvince", "Latitude", "Longitude", "LastUpdateByUserID", "ZipPostalCode"
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(&data.zip_code_id)
    .bind(&data.street_name)
    .bind(&data.district)
    .bind(&data.county)
    .bind(&data.city)
    .bind(&data.sr_province)
    .bind(data.latitude)
    .bind(data.longitude)
    .bind(created_by)
    .bind(&data.zip_postal_code)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Zip Code '{}' dibuat oleh {}", row.zip_code_id, created_by);
    Ok(row)
}

pub async fn update_zip_code(
    pool: &PgPool,
    id: &str,
    data: &ZipCode,
    updated_by: &str,
) -> Result<ZipCode, AppError> {
    let row = sqlx::query_as::<_, ZipCode>(&format!(
        r#"
        UPDATE "ZipCodes"
        SET "StreetName" = $2, "District" = $3, "County" = $4, "City" = $5,
            "SRProvince" = $6, "Latitude" = $7, "Longitude" = $8,
            "LastUpdateDateTime" = NOW(), "LastUpdateByUserID" = $9,
            "ZipPostalCode" = $10
        WHERE "ZipCodeId" = $1
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(id)
    .bind(&data.street_name)
    .bind(&data.district)
    .bind(&data.county)
    .bind(&data.city)
    .bind(&data.sr_province)
    .bind(data.latitude)
    .bind(data.longitude)
    .bind(updated_by)
    .bind(&data.zip_postal_code)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let row = row.ok_or(AppError::NotFound("Zip Code tidak ditemukan".into()))?;
    info!("Zip Code '{}' diperbarui oleh {}", id, updated_by);
    Ok(row)
}

pub async fn delete_zip_code(pool: &PgPool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "ZipCodes" WHERE "ZipCodeId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Zip Code tidak ditemukan".into()));
    }

    info!("Zip Code '{}' dihapus", id);
    Ok(())
}