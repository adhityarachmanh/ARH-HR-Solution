use crate::errors::AppError;
use crate::models::{Shift, ShiftFormData};
use sqlx::PgPool;
use tracing::info;

const SELECT_FIELDS: &str = r#"
    "ShiftId" as shift_id,
    "ShiftCode" as shift_code,
    "ShiftName" as shift_name,
    "ShiftInHour" as shift_in_hour,
    "ShiftOutHour" as shift_out_hour,
    "BreakStart" as break_start,
    "BreakEnd" as break_end,
    "IsShowInRequest" as is_show_in_request,
    "IsEnableAttendanceValidation" as is_enable_attendance_validation,
    "ClockInMinBefore" as clock_in_min_before,
    "ClockOutMaxAfter" as clock_out_max_after,
    "IsEnableDispensation" as is_enable_dispensation,
    "ClockInDispensation" as clock_in_dispensation,
    "ClockOutDispensation" as clock_out_dispensation,
    "IsWorkShift" as is_work_shift,
    "ShiftColorHex" as shift_color_hex,
    "IsShiftOutHourOverlapDay" as is_shift_out_hour_overlap_day,
    "CreatedDate" as created_date,
    "CreatedBy" as created_by,
    "UpdatedDate" as updated_date,
    "UpdatedBy" as updated_by
"#;

pub async fn create_shift(pool: &PgPool, data: &ShiftFormData, created_by: &str) -> Result<Shift, AppError> {
    let row = sqlx::query_as::<_, Shift>(&format!(
        r#"
        INSERT INTO "Shifts" (
            "ShiftCode", "ShiftName", "ShiftInHour", "ShiftOutHour", "BreakStart", "BreakEnd", 
            "IsShowInRequest", "IsEnableAttendanceValidation", "ClockInMinBefore", "ClockOutMaxAfter", 
            "IsEnableDispensation", "ClockInDispensation", "ClockOutDispensation", "IsWorkShift", 
            "ShiftColorHex", "IsShiftOutHourOverlapDay", "CreatedBy"
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(&data.shift_code)
    .bind(&data.shift_name)
    .bind(&data.shift_in_hour)
    .bind(&data.shift_out_hour)
    .bind(&data.break_start)
    .bind(&data.break_end)
    .bind(data.is_show_in_request)
    .bind(data.is_enable_attendance_validation)
    .bind(data.clock_in_min_before)
    .bind(data.clock_out_max_after)
    .bind(data.is_enable_dispensation)
    .bind(data.clock_in_dispensation)
    .bind(data.clock_out_dispensation)
    .bind(data.is_work_shift)
    .bind(&data.shift_color_hex)
    .bind(data.is_shift_out_hour_overlap_day)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Shift '{}' created by {}", row.shift_code.as_deref().unwrap_or("N/A"), created_by);
    Ok(row)
}

pub async fn count_shifts(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("ShiftId") FROM "Shifts""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_shifts_paginated(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Shift>, AppError> {
    let rows = sqlx::query_as::<_, Shift>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Shifts" ORDER BY "ShiftCode" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(rows)
}

pub async fn get_shift_by_id(pool: &PgPool, id: i32) -> Result<Shift, AppError> {
    let row = sqlx::query_as::<_, Shift>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Shifts" WHERE "ShiftId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("Shift with ID {} not found.", id)))
}

pub async fn get_shift_by_code(pool: &PgPool, code: &str) -> Result<Shift, AppError> {
    let row = sqlx::query_as::<_, Shift>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Shifts" WHERE "ShiftCode" = $1"#
    ))
    .bind(code)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("Shift with Code {} not found.", code)))
}

pub async fn update_shift(
    pool: &PgPool,
    id: i32,
    data: &ShiftFormData,
    updated_by: &str,
) -> Result<Shift, AppError> {
    let row = sqlx::query_as::<_, Shift>(&format!(
        r#"
        UPDATE "Shifts"
        SET 
            "ShiftCode" = $2, "ShiftName" = $3, "ShiftInHour" = $4, "ShiftOutHour" = $5, 
            "BreakStart" = $6, "BreakEnd" = $7, "IsShowInRequest" = $8, 
            "IsEnableAttendanceValidation" = $9, "ClockInMinBefore" = $10, "ClockOutMaxAfter" = $11, 
            "IsEnableDispensation" = $12, "ClockInDispensation" = $13, "ClockOutDispensation" = $14, 
            "IsWorkShift" = $15, "ShiftColorHex" = $16, "IsShiftOutHourOverlapDay" = $17,
            "UpdatedDate" = NOW(), "UpdatedBy" = $18
        WHERE "ShiftId" = $1
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(id)
    .bind(&data.shift_code)
    .bind(&data.shift_name)
    .bind(&data.shift_in_hour)
    .bind(&data.shift_out_hour)
    .bind(&data.break_start)
    .bind(&data.break_end)
    .bind(data.is_show_in_request)
    .bind(data.is_enable_attendance_validation)
    .bind(data.clock_in_min_before)
    .bind(data.clock_out_max_after)
    .bind(data.is_enable_dispensation)
    .bind(data.clock_in_dispensation)
    .bind(data.clock_out_dispensation)
    .bind(data.is_work_shift)
    .bind(&data.shift_color_hex)
    .bind(data.is_shift_out_hour_overlap_day)
    .bind(updated_by)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    let row = row.ok_or(AppError::NotFound(format!("Shift with ID {} not found.", id)))?;
    info!("Shift ID {} updated by {}", id, updated_by);
    Ok(row)
}

pub async fn delete_shift(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "Shifts" WHERE "ShiftId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Shift with ID {} not found.", id)));
    }

    info!("Shift ID {} deleted", id);
    Ok(())
}