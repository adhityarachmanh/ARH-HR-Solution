// src/services/branch_service.rs
use sqlx::PgPool;
use crate::models::Branch;
use crate::errors::AppError;
use tracing::info;

pub async fn get_all_branches(pool: &PgPool) -> Result<Vec<Branch>, AppError> {
    let branches = sqlx::query_as!(
        Branch,
        "SELECT \"BranchId\" as branch_id, \"CompId\" as comp_id, \"TimeZoneId\" as time_zone_id, \"BranchName\" as branch_name FROM \"Branches\" ORDER BY \"BranchName\" ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    info!("Fetched {} branches.", branches.len());
    Ok(branches)
}