use crate::errors::AppError;
use crate::models::Holiday;
use chrono::{NaiveDate, TimeZone as ChronoTimeZone, Utc};
use reqwest;
use serde::Deserialize;
use sqlx::PgPool;
use time::OffsetDateTime;
use tracing::info;

#[derive(Debug, Deserialize)]
struct ApiHoliday {
    holiday_date: String,
    holiday_name: String,
    #[serde(default)]
    is_national_holiday: bool,
}

const HOLIDAY_FIELDS_SQL: &str = r#""HolidayId" as holiday_id, "HolidayName" as holiday_name, "HolidayDate" as holiday_date, "HolidayDescription" as holiday_description, "IsNationalHoliday" as is_national_holiday, "IsCompanyHoliday" as is_company_holiday, "IsSpecialHoliday" as is_special_holiday, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by, "HolidayYear" as holiday_year, "IsMassLeave" as is_mass_leave"#;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "Holidays" ("HolidayName", "HolidayDate", "HolidayDescription", "IsNationalHoliday", "IsCompanyHoliday", "IsSpecialHoliday", "IsMassLeave", "HolidayYear", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), $9, NOW(), $9) 
    RETURNING "HolidayId", "HolidayName", "HolidayDate", "HolidayDescription", "IsNationalHoliday", "IsCompanyHoliday", "IsSpecialHoliday", "IsMassLeave", "HolidayYear", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "Holidays" 
    SET "HolidayName" = $1, "HolidayDate" = $3, "HolidayDescription" = $4, "IsNationalHoliday" = $5, "IsCompanyHoliday" = $6, "IsSpecialHoliday" = $7, "IsMassLeave" = $8, "UpdatedDate" = NOW(), "UpdatedBy" = $9
    WHERE "HolidayId" = $2 
    RETURNING "HolidayId", "HolidayName", "HolidayDate", "HolidayDescription", "IsNationalHoliday", "IsCompanyHoliday", "IsSpecialHoliday", "IsMassLeave", "HolidayYear", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

async fn fetch_holidays_from_api(_pool: &PgPool, year: i32) -> Result<Vec<Holiday>, AppError> {
    info!(
        "Fetching actual holidays for year {} from vercel API.",
        year
    );

    let api_url = format!("https://api-harilibur.vercel.app/api?year={}", year);

    let client = reqwest::Client::new();
    let response = client
        .get(&api_url)
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("API Request Failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::InternalError(format!(
            "API returned error status: {}",
            response.status()
        )));
    }

    let external_data: Vec<ApiHoliday> = response
        .json()
        .await
        .map_err(|e| AppError::InternalError(format!("JSON Parse Failed: {}", e)))?;

    let mut holidays = Vec::new();
    for api_h in external_data {
        let naive_date =
            NaiveDate::parse_from_str(&api_h.holiday_date, "%Y-%m-%d").map_err(|_| {
                AppError::InternalError(format!(
                    "Invalid date format from API: {}",
                    api_h.holiday_date
                ))
            })?;

        let naive_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| AppError::InternalError("Failed to create datetime".to_string()))?;

        let chrono_datetime = Utc.from_utc_datetime(&naive_datetime);
        let holiday_date = OffsetDateTime::from_unix_timestamp(chrono_datetime.timestamp())
            .map_err(|_| AppError::InternalError("Timestamp conversion error.".to_string()))?;

        holidays.push(Holiday {
            holiday_id: 0,
            holiday_name: Some(api_h.holiday_name),
            holiday_date: Some(holiday_date),
            holiday_description: Some("Libur Nasional (API)".to_string()),
            is_national_holiday: Some(api_h.is_national_holiday),
            is_company_holiday: Some(false),
            is_special_holiday: Some(false),
            created_date: None,
            created_by: None,
            updated_date: None,
            updated_by: None,
            holiday_year: Some(year),
            is_mass_leave: false,
        });
    }

    Ok(holidays)
}

pub async fn sync_holidays_for_year(pool: &PgPool, year: i32) -> Result<(), AppError> {
    info!("Starting holiday synchronization for year {}.", year);

    let external_holidays = fetch_holidays_from_api(pool, year).await?;
    let mut inserted_count = 0;

    for holiday_data in external_holidays {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM \"Holidays\" WHERE \"HolidayName\" = $1 AND \"HolidayYear\" = $2)",
            holiday_data.holiday_name,
            holiday_data.holiday_year
        )
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?.unwrap_or(false);

        if !exists {
            sqlx::query!(
                r#"INSERT INTO "Holidays" ("HolidayName", "HolidayDate", "HolidayDescription", "IsNationalHoliday", "IsCompanyHoliday", "IsSpecialHoliday", "IsMassLeave", "HolidayYear", "CreatedDate", "CreatedBy")
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), $9)"#,
                holiday_data.holiday_name.unwrap_or_default(),
                holiday_data.holiday_date,
                holiday_data.holiday_description,
                holiday_data.is_national_holiday,
                holiday_data.is_company_holiday,
                holiday_data.is_special_holiday,
                holiday_data.is_mass_leave,
                holiday_data.holiday_year,
                "System Worker"
            )
            .execute(pool).await.map_err(AppError::DatabaseError)?;
            inserted_count += 1;
        }
    }

    info!(
        "Holiday sync for {} completed. {} new holidays inserted.",
        year, inserted_count
    );
    Ok(())
}

pub async fn count_holidays(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("HolidayId") FROM "Holidays""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_holidays_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Holiday>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Holidays\" ORDER BY \"HolidayDate\" ASC LIMIT $1 OFFSET $2",
        HOLIDAY_FIELDS_SQL
    );

    let holidays = sqlx::query_as::<_, Holiday>(&sql_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(holidays)
}

pub async fn get_holiday_by_id(pool: &PgPool, id: i32) -> Result<Holiday, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Holidays\" WHERE \"HolidayId\" = $1",
        HOLIDAY_FIELDS_SQL
    );

    let holiday = sqlx::query_as::<_, Holiday>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Holiday not found".to_string()),
            _ => AppError::DatabaseError(e),
        })?;
    Ok(holiday)
}

pub async fn create_holiday(
    pool: &PgPool,
    holiday_data: Holiday,
    created_by: &str,
) -> Result<Holiday, AppError> {
    let new_holiday = sqlx::query_as::<_, Holiday>(CREATE_QUERY_SQL)
        .bind(holiday_data.holiday_name)
        .bind(holiday_data.holiday_date)
        .bind(holiday_data.holiday_description)
        .bind(holiday_data.is_national_holiday)
        .bind(holiday_data.is_company_holiday)
        .bind(holiday_data.is_special_holiday)
        .bind(holiday_data.is_mass_leave)
        .bind(holiday_data.holiday_year)
        .bind(created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Holiday '{}' created with ID: {}",
        new_holiday.holiday_name.as_deref().unwrap_or("N/A"),
        new_holiday.holiday_id
    );
    Ok(new_holiday)
}

pub async fn update_holiday(
    pool: &PgPool,
    id: i32,
    holiday_data: Holiday,
    updated_by: &str,
) -> Result<Holiday, AppError> {
    let updated_holiday = sqlx::query_as::<_, Holiday>(UPDATE_QUERY_SQL)
        .bind(holiday_data.holiday_name)
        .bind(id)
        .bind(holiday_data.holiday_date)
        .bind(holiday_data.holiday_description)
        .bind(holiday_data.is_national_holiday)
        .bind(holiday_data.is_company_holiday)
        .bind(holiday_data.is_special_holiday)
        .bind(holiday_data.is_mass_leave)
        .bind(holiday_data.holiday_year)
        .bind(updated_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Holiday ID {} updated to '{}'",
        updated_holiday.holiday_id,
        updated_holiday.holiday_name.as_deref().unwrap_or("N/A")
    );
    Ok(updated_holiday)
}

pub async fn delete_holiday(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM \"Holidays\" WHERE \"HolidayId\" = $1", id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Holiday not found for deletion".to_string(),
        ));
    }
    info!("Holiday ID {} deleted.", id);
    Ok(())
}
