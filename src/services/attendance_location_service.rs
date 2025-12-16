use crate::errors::AppError;
use crate::models::{AttendanceLocation, AttendanceLocationDetail};
use sqlx::{PgPool, Row};
use tracing::info;

const ATTENDANCE_LOCATION_FIELDS_SQL: &str = r#""AttendanceLocationId" as attendance_location_id, "TimeZoneId" as time_zone_id, "LocationName" as location_name, "IsFlexible" as is_flexible, "Latitude" as latitude, "Longitude" as longitude, "RadiusToleranceInMeter" as radius_tolerance_in_meter, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by"#;

const SELECT_BASE_DETAIL_QUERY: &str = r#"
    SELECT 
        T1."AttendanceLocationId" AS attendance_location_id,
        T1."TimeZoneId" AS time_zone_id,
        T1."LocationName" AS location_name,
        T1."IsFlexible" AS is_flexible,
        T1."Latitude" AS latitude,
        T1."Longitude" AS longitude,
        T1."RadiusToleranceInMeter" AS radius_tolerance_in_meter,
        T1."CreatedDate" AS created_date,
        T1."UpdatedDate" AS updated_date,
        T2."TimeZoneName" AS timezone_name
    FROM "AttendanceLocations" T1
    LEFT JOIN "TimeZones" T2 ON T1."TimeZoneId" = T2."TimeZoneId"
"#;

pub async fn count_attendance_locations(pool: &PgPool) -> Result<i64, AppError> {
    let count =
        sqlx::query_scalar!(r#"SELECT COUNT("AttendanceLocationId") FROM "AttendanceLocations""#)
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?
            .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_locations(
    pool: &PgPool,
) -> Result<Vec<AttendanceLocationDetail>, AppError> {
    let sql_query = format!(
        "{} ORDER BY T1.\"LocationName\" ASC",
        SELECT_BASE_DETAIL_QUERY
    );

    let locations = sqlx::query(sql_query.as_str())
        .map(|row: sqlx::postgres::PgRow| AttendanceLocationDetail {
            attendance_location_id: row.get("attendance_location_id"),
            time_zone_id: row.get("time_zone_id"),
            location_name: row.get("location_name"),
            is_flexible: row.get("is_flexible"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            radius_tolerance_in_meter: row.get("radius_tolerance_in_meter"),
            timezone_name: row.get("timezone_name"),
            created_date: row.get("created_date"),
            updated_date: row.get("updated_date"),
        })
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(locations)
}


pub async fn get_all_locations_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<AttendanceLocationDetail>, AppError> {
    let sql_query = format!(
        "{} ORDER BY T1.\"LocationName\" ASC LIMIT $1 OFFSET $2",
        SELECT_BASE_DETAIL_QUERY
    );

    let locations = sqlx::query(sql_query.as_str())
        .bind(limit)
        .bind(offset)
        .map(|row: sqlx::postgres::PgRow| AttendanceLocationDetail {
            attendance_location_id: row.get("attendance_location_id"),
            time_zone_id: row.get("time_zone_id"),
            location_name: row.get("location_name"),
            is_flexible: row.get("is_flexible"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            radius_tolerance_in_meter: row.get("radius_tolerance_in_meter"),
            timezone_name: row.get("timezone_name"),
            created_date: row.get("created_date"),
            updated_date: row.get("updated_date"),
        })
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(locations)
}

pub async fn get_location_by_id(pool: &PgPool, id: i32) -> Result<AttendanceLocation, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"AttendanceLocations\" WHERE \"AttendanceLocationId\" = $1",
        ATTENDANCE_LOCATION_FIELDS_SQL
    );
    let location = sqlx::query_as::<_, AttendanceLocation>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Location not found".to_string()),
            _ => AppError::DatabaseError(e),
        })?;
    Ok(location)
}

pub async fn create_attendance_location(
    pool: &PgPool,
    loc_data: &AttendanceLocation,
    created_by: &str,
) -> Result<AttendanceLocation, AppError> {
    let new_location = sqlx::query_as::<_, AttendanceLocation>(&format!(
        r#"
        INSERT INTO "AttendanceLocations" ("LocationName", "TimeZoneId", "IsFlexible", "Latitude", "Longitude", "RadiusToleranceInMeter", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy")
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), $7, NOW(), $7)
        RETURNING {}
        "#, ATTENDANCE_LOCATION_FIELDS_SQL
    ))
    .bind(&loc_data.location_name)
    .bind(loc_data.time_zone_id)
    .bind(loc_data.is_flexible)
    .bind(loc_data.latitude.clone())
    .bind(loc_data.longitude.clone())
    .bind(loc_data.radius_tolerance_in_meter)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!(
        "Location '{}' created with ID: {}",
        new_location.location_name.as_deref().unwrap_or("N/A"),
        new_location.attendance_location_id
    );
    Ok(new_location)
}

pub async fn update_attendance_location(
    pool: &PgPool,
    id: i32,
    loc_data: &AttendanceLocation,
    updated_by: &str,
) -> Result<AttendanceLocation, AppError> {
    let updated_location = sqlx::query_as::<_, AttendanceLocation>(&format!(
        r#"
        UPDATE "AttendanceLocations" SET 
        "LocationName" = $1, "TimeZoneId" = $2, "IsFlexible" = $3, "Latitude" = $4, "Longitude" = $5, "RadiusToleranceInMeter" = $6, 
        "UpdatedDate" = NOW(), "UpdatedBy" = $7
        WHERE "AttendanceLocationId" = $8
        RETURNING {}
        "#, ATTENDANCE_LOCATION_FIELDS_SQL
    ))
    .bind(&loc_data.location_name)
    .bind(loc_data.time_zone_id)
    .bind(loc_data.is_flexible)
    .bind(loc_data.latitude.clone())
    .bind(loc_data.longitude.clone())
    .bind(loc_data.radius_tolerance_in_meter)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Location not found for update".to_string()),
        _ => AppError::DatabaseError(e),
    })?;

    info!(
        "Location ID {} updated to '{}'",
        id,
        updated_location.location_name.as_deref().unwrap_or("N/A")
    );
    Ok(updated_location)
}

pub async fn delete_location(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM "AttendanceLocations" WHERE "AttendanceLocationId" = $1"#,
        id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Location not found for deletion".to_string(),
        ));
    }
    info!("Attendance Location ID {} deleted.", id);
    Ok(())
}