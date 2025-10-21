// src/services/branch_service.rs
use sqlx::PgPool;
use sqlx::Row;
use tracing::info;
use crate::models::Branch;
use crate::errors::AppError;
use crate::models::BranchDetail;
use crate::models::{CompanyDropdown, TimeZoneDropdown};

pub async fn get_all_branch_details(pool: &PgPool) -> Result<Vec<BranchDetail>, AppError> {
    let sql = r#"
        SELECT 
            T1."BranchId" AS branch_id,
            T1."BranchName" AS branch_name,
            T1."CompId" AS comp_id,
            T1."TimeZoneId" AS time_zone_id,
            T2."CompName" AS company_name,     -- JOINED FIELD
            T3."TimeZoneName" AS timezone_name  -- JOINED FIELD
        FROM "Branches" T1
        LEFT JOIN "Company" T2 ON T1."CompId" = T2."CompId"
        LEFT JOIN "TimeZones" T3 ON T1."TimeZoneId" = T3."TimeZoneId"
        ORDER BY T1."BranchName" ASC
    "#;

    let branch_details = sqlx::query(sql)
        .map(|row: sqlx::postgres::PgRow| BranchDetail {
            branch_id: row.get("branch_id"),
            branch_name: row.get("branch_name"),
            comp_id: row.get("comp_id"),
            time_zone_id: row.get("time_zone_id"),
            company_name: row.get("company_name"),
            timezone_name: row.get("timezone_name"),
        })
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;
        
    info!("Fetched {} detailed branches.", branch_details.len());
    Ok(branch_details)
}

pub async fn get_branch_by_id(pool: &PgPool, branch_id: i32) -> Result<Branch, AppError> {
    let branch = sqlx::query_as!(
        Branch,
        "SELECT \"BranchId\" as branch_id, \"CompId\" as comp_id, \"TimeZoneId\" as time_zone_id, \"BranchName\" as branch_name FROM \"Branches\" WHERE \"BranchId\" = $1",
        branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Branch not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(branch)
}

pub async fn create_branch(pool: &PgPool, name: &str, comp_id: Option<i32>, timezone_id: Option<i32>) -> Result<Branch, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Branches\" WHERE LOWER(\"BranchName\") = LOWER($1))",
        name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    
    if exists.unwrap_or(false) {
        return Err(AppError::InternalError("Branch name already exists".to_string()));
    }

    let new_branch = sqlx::query_as!(
        Branch,
        "INSERT INTO \"Branches\" (\"BranchName\", \"CompId\", \"TimeZoneId\") VALUES ($1, $2, $3) RETURNING \"BranchId\" as branch_id, \"CompId\" as comp_id, \"TimeZoneId\" as time_zone_id, \"BranchName\" as branch_name",
        name,
        comp_id,
        timezone_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    
    info!("Branch '{}' created with ID: {}", new_branch.branch_name, new_branch.branch_id);
    Ok(new_branch)
}

pub async fn update_branch(pool: &PgPool, branch_id: i32, new_name: &str, comp_id: Option<i32>, timezone_id: Option<i32>) -> Result<Branch, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Branches\" WHERE LOWER(\"BranchName\") = LOWER($1) AND \"BranchId\" != $2)",
        new_name,
        branch_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    
    if exists.unwrap_or(false) {
        return Err(AppError::InternalError("Branch name already exists".to_string()));
    }

    let updated_branch = sqlx::query_as!(
        Branch,
        "UPDATE \"Branches\" SET \"BranchName\" = $1, \"CompId\" = $3, \"TimeZoneId\" = $4 WHERE \"BranchId\" = $2 RETURNING \"BranchId\" as branch_id, \"CompId\" as comp_id, \"TimeZoneId\" as time_zone_id, \"BranchName\" as branch_name",
        new_name,
        branch_id,
        comp_id,
        timezone_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?; 

    info!("Branch ID {} updated to '{}'", updated_branch.branch_id, updated_branch.branch_name);
    Ok(updated_branch)
}

pub async fn delete_branch(pool: &PgPool, branch_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM \"Branches\" WHERE \"BranchId\" = $1", branch_id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError("Branch not found for deletion".to_string()));
    }
    
    info!("Branch ID {} deleted.", branch_id);
    Ok(())
}

pub async fn get_companies_for_dropdown(pool: &PgPool) -> Result<Vec<CompanyDropdown>, AppError> {
    let companies = sqlx::query_as!(
        CompanyDropdown,
        "SELECT \"CompId\" as comp_id, \"CompName\" as comp_name FROM \"Company\" ORDER BY \"CompName\" ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(companies)
}

pub async fn get_timezones_for_dropdown(pool: &PgPool) -> Result<Vec<TimeZoneDropdown>, AppError> {
    let timezones = sqlx::query_as!(
        TimeZoneDropdown,
        "SELECT \"TimeZoneId\" as time_zone_id, \"TimeZoneName\" as time_zone_name FROM \"TimeZones\" ORDER BY \"TimeZoneName\" ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(timezones)
}