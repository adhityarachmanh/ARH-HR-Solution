// src/services/organization_service.rs
use sqlx::PgPool;
use crate::models::Organization;
use crate::errors::AppError;
use tracing::info;

pub async fn get_all_organizations(pool: &PgPool) -> Result<Vec<Organization>, AppError> {
    let organizations = sqlx::query_as!(
        Organization,
        "SELECT \"OrganizationId\" as organization_id, \"OrganizationName\" as organization_name, \"ManagerId\" as manager_id FROM \"Organizations\" ORDER BY \"OrganizationName\" ASC"
    )
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    info!("Fetched {} organizations.", organizations.len());
    Ok(organizations)
}