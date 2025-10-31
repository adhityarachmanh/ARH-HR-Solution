// src/services/zip_code_service.rs

use crate::errors::AppError;
use crate::models::ZipCode;
use sqlx::PgPool;
use tracing::info;

const ZIP_CODE_FIELDS_SQL: &str = r#""ZipCodeId" as zip_code_id, "StreetName" as street_name, "District" as district, "County" as county, "City" as city, "SRProvince" as sr_province, "Latitude" as latitude, "Longitude" as longitude, "LastUpdateDateTime" as last_update_date_time, "LastUpdateByUserID" as last_update_by_user_id, "ZipPostalCode" as zip_postal_code"#;

// --- READ ALL (Paginated) ---
pub async fn count_zip_codes(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("ZipCodeId") FROM "ZipCodes""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_zip_codes_paginated(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<ZipCode>, AppError> {
    let sql_query = format!(
        r#"SELECT {} FROM "ZipCodes" ORDER BY "ZipCodeId" ASC LIMIT $1 OFFSET $2"#,
        ZIP_CODE_FIELDS_SQL
    );

    let zip_codes = sqlx::query_as::<_, ZipCode>(&sql_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;
    
    Ok(zip_codes)
}

// --- READ By ID (Untuk Form Edit) ---
pub async fn get_zip_code_by_id(pool: &PgPool, id: &str) -> Result<ZipCode, AppError> {
    let sql_query = format!(
        r#"SELECT {} FROM "ZipCodes" WHERE "ZipCodeId" = $1"#,
        ZIP_CODE_FIELDS_SQL
    );

    let zip_code = sqlx::query_as::<_, ZipCode>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Zip Code not found".to_string()),
            _ => AppError::DatabaseError(e),
        })?;
    
    Ok(zip_code)
}

// --- CREATE ---
pub async fn create_zip_code(pool: &PgPool, data: &ZipCode, created_by: &str) -> Result<ZipCode, AppError> {
    let created_by_str = Some(created_by.to_string());

    let sql_query = format!(
        r#"
        INSERT INTO "ZipCodes" ("ZipCodeId", "StreetName", "District", "County", "City", "SRProvince", "Latitude", "Longitude", "LastUpdateDateTime", "LastUpdateByUserID", "ZipPostalCode")
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), $9, $10)
        RETURNING {}
        "#, ZIP_CODE_FIELDS_SQL
    );

    let new_zip = sqlx::query_as::<_, ZipCode>(&sql_query)
        .bind(&data.zip_code_id)
        .bind(&data.street_name)
        .bind(&data.district)
        .bind(&data.county)
        .bind(&data.city)
        .bind(&data.sr_province)
        .bind(data.latitude)
        .bind(data.longitude)
        .bind(&created_by_str)
        .bind(&data.zip_postal_code)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!("Zip Code '{}' created.", new_zip.zip_code_id);
    Ok(new_zip)
}

// --- UPDATE ---
pub async fn update_zip_code(pool: &PgPool, id: &str, data: &ZipCode, updated_by: &str) -> Result<ZipCode, AppError> {
    
    let updated_by_str = Some(updated_by.to_string());

    let sql_query = format!(
        r#"
        UPDATE "ZipCodes" 
        SET "StreetName" = $2, "District" = $3, "County" = $4, "City" = $5, "SRProvince" = $6, 
            "Latitude" = $7, "Longitude" = $8, "LastUpdateDateTime" = NOW(), 
            "LastUpdateByUserID" = $9, "ZipPostalCode" = $10
        WHERE "ZipCodeId" = $1
        RETURNING {}
        "#, ZIP_CODE_FIELDS_SQL
    );

    let updated_zip = sqlx::query_as::<_, ZipCode>(&sql_query)
        .bind(id)
        .bind(&data.street_name)
        .bind(&data.district)
        .bind(&data.county)
        .bind(&data.city)
        .bind(&data.sr_province)
        .bind(data.latitude)
        .bind(data.longitude)
        .bind(&updated_by_str)
        .bind(&data.zip_postal_code)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Zip Code not found for update".to_string()),
            _ => AppError::DatabaseError(e),
        })?;

    info!("Zip Code ID {} updated.", updated_zip.zip_code_id);
    Ok(updated_zip)
}

// --- DELETE ---
pub async fn delete_zip_code(pool: &PgPool, id: &str) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "ZipCodes" WHERE "ZipCodeId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError("Zip Code not found for deletion".to_string()));
    }
    info!("Zip Code ID {} deleted.", id);
    Ok(())
}