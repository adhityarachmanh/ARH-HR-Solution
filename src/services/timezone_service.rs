// src/services/timezone_service.rs
use sqlx::PgPool;
use crate::models::TimeZone;
use crate::errors::AppError;
use tracing::info;

// const TIMEZONE_FIELDS_SQL: &str = r#""TimeZoneId" as time_zone_id, "TimeZoneName" as time_zone_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by"#;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "TimeZones" ("TimeZoneName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
    VALUES ($1, NOW(), $2, NOW(), $2) 
    RETURNING "TimeZoneId", "TimeZoneName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "TimeZones" 
    SET "TimeZoneName" = $1, "UpdatedDate" = NOW(), "UpdatedBy" = $3 
    WHERE "TimeZoneId" = $2 
    RETURNING "TimeZoneId", "TimeZoneName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

pub async fn get_all_timezones(pool: &PgPool) -> Result<Vec<TimeZone>, AppError> {
    let timezones = sqlx::query_as!(
        TimeZone,
        r#"SELECT "TimeZoneId" as time_zone_id, "TimeZoneName" as time_zone_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "TimeZones" ORDER BY "TimeZoneName" ASC"#
    )
    .fetch_all(pool).await.map_err(AppError::DatabaseError)?;
    info!("Fetched {} timezones.", timezones.len());
    Ok(timezones)
}

pub async fn get_timezone_by_id(pool: &PgPool, timezone_id: i32) -> Result<TimeZone, AppError> {
    let timezone = sqlx::query_as!(
        TimeZone,
        r#"SELECT "TimeZoneId" as time_zone_id, "TimeZoneName" as time_zone_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "TimeZones" WHERE "TimeZoneId" = $1"#,
        timezone_id
    ).fetch_one(pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("TimeZone not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(timezone)
}

pub async fn create_timezone(pool: &PgPool, name: &str) -> Result<TimeZone, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"TimeZones\" WHERE LOWER(\"TimeZoneName\") = LOWER($1))", name
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    if exists.unwrap_or(false) { return Err(AppError::InternalError("TimeZone already exists".to_string())); }

    let new_timezone = sqlx::query_as::<_, TimeZone>(CREATE_QUERY_SQL)
        .bind(name)
        .bind("Admin")
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    info!("TimeZone '{}' created with ID: {}", new_timezone.time_zone_name, new_timezone.time_zone_id);
    Ok(new_timezone)
}

pub async fn update_timezone(pool: &PgPool, timezone_id: i32, new_name: &str) -> Result<TimeZone, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"TimeZones\" WHERE LOWER(\"TimeZoneName\") = LOWER($1) AND \"TimeZoneId\" != $2)",
        new_name, timezone_id
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    if exists.unwrap_or(false) { return Err(AppError::InternalError("TimeZone name already exists".to_string())); }

    let updated_timezone = sqlx::query_as::<_, TimeZone>(UPDATE_QUERY_SQL)
        .bind(new_name)
        .bind(timezone_id)
        .bind("Admin")
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?; 

    info!("TimeZone ID {} updated to '{}'", updated_timezone.time_zone_id, updated_timezone.time_zone_name);
    Ok(updated_timezone)
}

pub async fn delete_timezone(pool: &PgPool, timezone_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM \"TimeZones\" WHERE \"TimeZoneId\" = $1", timezone_id)
        .execute(pool).await.map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError("TimeZone not found for deletion".to_string()));
    }
    info!("TimeZone ID {} deleted.", timezone_id);
    Ok(())
}