// src/services/company_service.rs
use sqlx::PgPool;
use crate::models::Company;
use crate::errors::AppError;
use tracing::info;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "Company" ("CompName", "CompanyPrevMonthDayPayroll", "CompanyCurrentMonthDayPayroll") 
    VALUES ($1, $2, $3) 
    RETURNING 
        "CompId" as comp_id, 
        "CompName" as comp_name, 
        "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, 
        "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "Company" 
    SET "CompName" = $1, 
        "CompanyPrevMonthDayPayroll" = $3, 
        "CompanyCurrentMonthDayPayroll" = $4 
    WHERE "CompId" = $2 
    RETURNING 
        "CompId" as comp_id, 
        "CompName" as comp_name, 
        "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, 
        "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll
"#;

pub async fn get_all_companies(pool: &PgPool) -> Result<Vec<Company>, AppError> {
    let companies = sqlx::query_as!(
        Company,
        r#"SELECT "CompId" as comp_id, "CompName" as comp_name, "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll FROM "Company" ORDER BY "CompName" ASC"#
    )
    .fetch_all(pool).await.map_err(AppError::DatabaseError)?;
    Ok(companies)
}

pub async fn get_company_by_id(pool: &PgPool, comp_id: i32) -> Result<Company, AppError> {
    let company = sqlx::query_as!(
        Company,
        r#"SELECT "CompId" as comp_id, "CompName" as comp_name, "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll FROM "Company" WHERE "CompId" = $1"#,
        comp_id
    ).fetch_one(pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Company not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(company)
}

pub async fn create_company(pool: &PgPool, name: &str, prev_day_payroll: i32, current_day_payroll: i32) -> Result<Company, AppError> {
   
    let new_company = sqlx::query_as::<_, Company>(CREATE_QUERY_SQL)
        .bind(name)
        .bind(prev_day_payroll)
        .bind(current_day_payroll)
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    info!("Company '{}' created with ID: {}", new_company.comp_name.as_deref().unwrap_or("N/A"), new_company.comp_id);
    Ok(new_company)
}

pub async fn update_company(pool: &PgPool, comp_id: i32, name: &str, prev_day_payroll: i32, current_day_payroll: i32) -> Result<Company, AppError> {
   
    let updated_company = sqlx::query_as::<_, Company>(UPDATE_QUERY_SQL)
        .bind(name)
        .bind(comp_id)
        .bind(prev_day_payroll)
        .bind(current_day_payroll)
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?; 

    info!("Company ID {} updated to '{}'", updated_company.comp_id, updated_company.comp_name.as_deref().unwrap_or("N/A"));
    Ok(updated_company)
}

pub async fn delete_company(pool: &PgPool, comp_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM \"Company\" WHERE \"CompId\" = $1", comp_id)
        .execute(pool).await.map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError("Company not found for deletion".to_string()));
    }
    info!("Company ID {} deleted.", comp_id);
    Ok(())
}