use crate::errors::AppError;
use crate::models::{OvertimeSetting, OvertimeSettingFormData};
use sqlx::PgPool;
use tracing::info;

const SELECT_FIELDS: &str = r#"
    "OvertimeSettingId" as overtime_setting_id,
    "IsRounding" as is_rounding,
    "OvertimeName" as overtime_name,
    "IsDefaultCompensation" as is_default_compensation,
    "CompensationDivideNumber" as compensation_divide_number,
    "OverrideRupiahPerHour" as override_rupiah_per_hour
"#;

pub async fn create_overtime_setting(pool: &PgPool, data: &OvertimeSettingFormData) -> Result<OvertimeSetting, AppError> {
    let row = sqlx::query_as::<_, OvertimeSetting>(&format!(
        r#"
        INSERT INTO "OvertimeSettings" ("IsRounding", "OvertimeName", "IsDefaultCompensation", "CompensationDivideNumber", "OverrideRupiahPerHour") 
        VALUES ($1, $2, $3, $4, $5)
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(data.is_rounding)
    .bind(&data.overtime_name)
    .bind(data.is_default_compensation)
    .bind(data.compensation_divide_number)
    .bind(data.override_rupiah_per_hour)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Overtime Setting '{}' created", row.overtime_name.as_deref().unwrap_or("N/A"));
    Ok(row)
}

pub async fn count_overtime_settings(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("OvertimeSettingId") FROM "OvertimeSettings""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_overtime_settings_paginated(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<OvertimeSetting>, AppError> {
    let rows = sqlx::query_as::<_, OvertimeSetting>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "OvertimeSettings" ORDER BY "OvertimeName" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_overtime_setting_by_id(pool: &PgPool, id: i32) -> Result<OvertimeSetting, AppError> {
    let row = sqlx::query_as::<_, OvertimeSetting>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "OvertimeSettings" WHERE "OvertimeSettingId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("Overtime Setting with ID {} not found.", id)))
}


pub async fn update_overtime_setting(
    pool: &PgPool,
    id: i32,
    data: &OvertimeSettingFormData,
) -> Result<OvertimeSetting, AppError> {
    let row = sqlx::query_as::<_, OvertimeSetting>(&format!(
        r#"
        UPDATE "OvertimeSettings"
        SET "IsRounding" = $2, 
            "OvertimeName" = $3, 
            "IsDefaultCompensation" = $4, 
            "CompensationDivideNumber" = $5, 
            "OverrideRupiahPerHour" = $6
        WHERE "OvertimeSettingId" = $1
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(id)
    .bind(data.is_rounding)
    .bind(&data.overtime_name)
    .bind(data.is_default_compensation)
    .bind(data.compensation_divide_number)
    .bind(data.override_rupiah_per_hour)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let row = row.ok_or(AppError::NotFound(format!("Overtime Setting with ID {} not found.", id)))?;
    info!("Overtime Setting ID {} updated", id);
    Ok(row)
}

pub async fn delete_overtime_setting(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "OvertimeSettings" WHERE "OvertimeSettingId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Overtime Setting with ID {} not found.", id)));
    }

    info!("Overtime Setting ID {} deleted", id);
    Ok(())
}