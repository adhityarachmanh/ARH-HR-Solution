// src/services/branch_service.rs
use crate::errors::AppError;
use crate::models::{Branch, BranchDetail};
use sqlx::PgPool;
use sqlx::Row;
use tracing::info;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "Branches" ("BranchName", "CompanyName", "CompId", "TimeZoneId", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
    VALUES ($1, $2, $3, $4, NOW(), $5, NOW(), $5) 
    RETURNING "BranchId", "CompId", "TimeZoneId", "BranchName", "CompanyName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "Branches" 
    SET "BranchName" = $1, "CompanyName" = $4, "CompId" = $5, "TimeZoneId" = $6, "UpdatedDate" = NOW(), "UpdatedBy" = $7
    WHERE "BranchId" = $2 
    RETURNING "BranchId", "CompId", "TimeZoneId", "BranchName", "CompanyName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

pub async fn get_all_branches(pool: &PgPool) -> Result<Vec<Branch>, AppError> {
    let branches = sqlx::query_as!(
        Branch,
        r#"SELECT "BranchId" as branch_id, "CompId" as comp_id, "TimeZoneId" as time_zone_id, "BranchName" as branch_name, "CompanyName" as company_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "Branches" ORDER BY "BranchName" ASC"#
    )
    .fetch_all(pool).await.map_err(AppError::DatabaseError)?;
    info!("Fetched {} branches.", branches.len());
    Ok(branches)
}

pub async fn get_branch_by_id(pool: &PgPool, branch_id: i32) -> Result<Branch, AppError> {
    let branch = sqlx::query_as!(
        Branch,
        r#"SELECT "BranchId" as branch_id, "CompId" as comp_id, "TimeZoneId" as time_zone_id, "BranchName" as branch_name, "CompanyName" as company_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "Branches" WHERE "BranchId" = $1"#,
        branch_id
    )
    .fetch_one(pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Branch not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(branch)
}

pub async fn get_all_branch_details(pool: &PgPool) -> Result<Vec<BranchDetail>, AppError> {
    let sql = r#"
        SELECT 
            T1."BranchId" AS branch_id, T1."BranchName" AS branch_name, T1."CompId" AS comp_id, T1."TimeZoneId" AS time_zone_id,
            T1."CreatedDate" AS created_date, T1."UpdatedDate" AS updated_date,
            T2."CompName" AS company_name, 
            T3."TimeZoneName" AS timezone_name
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
            created_date: row.get("created_date"),
            updated_date: row.get("updated_date"),
        })
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!("Fetched {} detailed branches.", branch_details.len());
    Ok(branch_details)
}

pub async fn create_branch(
    pool: &PgPool,
    name: &str,
    company_name: &str,
    comp_id: Option<i32>,
    timezone_id: Option<i32>,
) -> Result<Branch, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Branches\" WHERE LOWER(\"BranchName\") = LOWER($1))",
        name
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Branch name already exists".to_string(),
        ));
    }

    let new_branch = sqlx::query_as::<_, Branch>(CREATE_QUERY_SQL)
        .bind(name)
        .bind(company_name)
        .bind(comp_id)
        .bind(timezone_id)
        .bind("Admin")
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Branch '{}' created with ID: {}",
        new_branch.branch_name, new_branch.branch_id
    );
    Ok(new_branch)
}

pub async fn update_branch(
    pool: &PgPool,
    branch_id: i32,
    new_name: &str,
    company_name: &str,
    comp_id: Option<i32>,
    timezone_id: Option<i32>,
) -> Result<Branch, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Branches\" WHERE LOWER(\"BranchName\") = LOWER($1) AND \"BranchId\" != $2)",
        new_name,
        branch_id
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Branch name already exists".to_string(),
        ));
    }

    let updated_branch = sqlx::query_as::<_, Branch>(UPDATE_QUERY_SQL)
        .bind(new_name)
        .bind(branch_id)
        .bind(company_name)
        .bind(comp_id)
        .bind(timezone_id)
        .bind("Admin")
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Branch ID {} updated to '{}'",
        updated_branch.branch_id, updated_branch.branch_name
    );
    Ok(updated_branch)
}

pub async fn delete_branch(pool: &PgPool, branch_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(
        "DELETE FROM \"Branches\" WHERE \"BranchId\" = $1",
        branch_id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Branch not found for deletion".to_string(),
        ));
    }
    info!("Branch ID {} deleted.", branch_id);
    Ok(())
}
