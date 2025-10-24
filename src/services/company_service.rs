use crate::errors::AppError;
use crate::models::Company;
use sqlx::PgPool;
use tracing::info;

const COMPANY_FIELDS_SQL: &str = r#""CompId" as "comp_id", "CompName" as "comp_name", "CompAddress" as "comp_address", "CompZipCode" as "comp_zip_code", "CompPhoneNumber" as "comp_phone_number", "CompMobileNumber" as "comp_mobile_number", "CompEmail" as "comp_email", "CompWebsite" as "comp_website", "CompRetirementAge" as "comp_retirement_age", "CompanyPrevMonthDayPayroll" as "prev_month_day_payroll", "CompanyCurrentMonthDayPayroll" as "current_month_day_payroll", "PersentasePendapatanBPJSTKPemberiKerja" as "bpjstk_pendapatan_pk", "PersentasePendapatanBPJSKSPemberiKerja" as "bpjsks_pendapatan_pk", "PersentasePenguranganBPJSTKPemberiKerja" as "bpjstk_pengurangan_pk", "PersentasePenguranganBPJSKSPemberiKerja" as "bpjsks_pengurangan_pk", "PersentasePenguranganBPJSTKPekerja" as "bpjstk_pengurangan_pekerja", "PersentasePenguranganBPJSKSPekerja" as "bpjsks_pengurangan_pekerja", "CreatedDate" as "created_date", "CreatedBy" as "created_by", "UpdatedDate" as "updated_date", "UpdatedBy" as "updated_by""#;

pub async fn count_companies(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("CompId") FROM "Company""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_companies_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Company>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Company\" ORDER BY \"CompName\" ASC LIMIT $1 OFFSET $2",
        COMPANY_FIELDS_SQL // Asumsi ini adalah konstanta alias field SQL Anda
    );

    let companies = sqlx::query_as::<_, Company>(&sql_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;
    
    Ok(companies)
}

pub async fn get_all_companies(pool: &PgPool) -> Result<Vec<Company>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Company\" ORDER BY \"CompId\" ASC",
        COMPANY_FIELDS_SQL
    );
    let companies = sqlx::query_as::<_, Company>(&sql_query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;
    Ok(companies)
}

pub async fn get_company_by_id(pool: &PgPool, id: i32) -> Result<Company, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Company\" WHERE \"CompId\" = $1",
        COMPANY_FIELDS_SQL
    );
    let company = sqlx::query_as::<_, Company>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Company not found".to_string()),
            _ => AppError::DatabaseError(e),
        })?;
    Ok(company)
}

pub async fn create_company(
    pool: &PgPool,
    company_data: Company,
    created_by: &str,
) -> Result<Company, AppError> {
    let new_company = sqlx::query_as::<_, Company>(&format!(
        r#"
        INSERT INTO "Company" ("CompName", "CompAddress", "CompZipCode", "CompPhoneNumber", "CompMobileNumber", "CompEmail", "CompWebsite", "CompRetirementAge", 
                               "PersentasePendapatanBPJSTKPemberiKerja", "PersentasePendapatanBPJSKSPemberiKerja", "PersentasePenguranganBPJSTKPemberiKerja", 
                               "PersentasePenguranganBPJSKSPemberiKerja", "PersentasePenguranganBPJSTKPekerja", "PersentasePenguranganBPJSKSPekerja",
                               "CompanyPrevMonthDayPayroll", "CompanyCurrentMonthDayPayroll", "CreatedBy", "UpdatedBy")
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $17)
        RETURNING {}
        "#, COMPANY_FIELDS_SQL
    ))
    .bind(company_data.comp_name)
    .bind(company_data.comp_address)
    .bind(company_data.comp_zip_code)
    .bind(company_data.comp_phone_number)
    .bind(company_data.comp_mobile_number)
    .bind(company_data.comp_email)
    .bind(company_data.comp_website)
    .bind(company_data.comp_retirement_age)
    .bind(company_data.bpjstk_pendapatan_pk)
    .bind(company_data.bpjsks_pendapatan_pk)
    .bind(company_data.bpjstk_pengurangan_pk)
    .bind(company_data.bpjsks_pengurangan_pk)
    .bind(company_data.bpjstk_pengurangan_pekerja)
    .bind(company_data.bpjsks_pengurangan_pekerja)
    .bind(company_data.prev_month_day_payroll)
    .bind(company_data.current_month_day_payroll)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!(
        "Company '{}' created with ID: {}",
        new_company.comp_name.as_deref().unwrap_or("N/A"),
        new_company.comp_id
    );
    Ok(new_company)
}

pub async fn update_company(
    pool: &PgPool,
    id: i32,
    company_data: Company,
    updated_by: &str,
) -> Result<Company, AppError> {
    let updated_company = sqlx::query_as::<_, Company>(&format!(
        r#"
        UPDATE "Company" SET 
        "CompName" = $1, "CompAddress" = $2, "CompZipCode" = $3, "CompPhoneNumber" = $4, "CompMobileNumber" = $5, "CompEmail" = $6, "CompWebsite" = $7, "CompRetirementAge" = $8, 
        "PersentasePendapatanBPJSTKPemberiKerja" = $9, "PersentasePendapatanBPJSKSPemberiKerja" = $10, "PersentasePenguranganBPJSTKPemberiKerja" = $11, 
        "PersentasePenguranganBPJSKSPemberiKerja" = $12, "PersentasePenguranganBPJSTKPekerja" = $13, "PersentasePenguranganBPJSKSPekerja" = $14,
        "CompanyPrevMonthDayPayroll" = $15, "CompanyCurrentMonthDayPayroll" = $16, "UpdatedDate" = NOW(), "UpdatedBy" = $17
        WHERE "CompId" = $18
        RETURNING {}
        "#, COMPANY_FIELDS_SQL
    ))
    .bind(company_data.comp_name)
    .bind(company_data.comp_address)
    .bind(company_data.comp_zip_code)
    .bind(company_data.comp_phone_number)
    .bind(company_data.comp_mobile_number)
    .bind(company_data.comp_email)
    .bind(company_data.comp_website)
    .bind(company_data.comp_retirement_age)
    .bind(company_data.bpjstk_pendapatan_pk)
    .bind(company_data.bpjsks_pendapatan_pk)
    .bind(company_data.bpjstk_pengurangan_pk)
    .bind(company_data.bpjsks_pengurangan_pk)
    .bind(company_data.bpjstk_pengurangan_pekerja)
    .bind(company_data.bpjsks_pengurangan_pekerja)
    .bind(company_data.prev_month_day_payroll)
    .bind(company_data.current_month_day_payroll)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Company not found for update".to_string()),
        _ => AppError::DatabaseError(e),
    })?;

    info!(
        "Company ID {} updated to '{}'",
        id,
        updated_company.comp_name.as_deref().unwrap_or("N/A")
    );
    Ok(updated_company)
}

pub async fn delete_company(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "Company" WHERE "CompId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Company not found for deletion".to_string(),
        ));
    }
    info!("Company ID {} deleted.", id);
    Ok(())
}