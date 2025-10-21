// src/services/organization_service.rs
use sqlx::PgPool;
use crate::models::Organization;
use crate::errors::AppError;
use tracing::info;

// Query field helper (digunakan untuk READ)
const SELECT_BASE_QUERY: &str = r#"SELECT "OrganizationId" as organization_id, "OrganizationName" as organization_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by FROM "Organizations""#;

// Query CREATE sebagai string literal
const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "Organizations" ("OrganizationName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
    VALUES ($1, NOW(), $2, NOW(), $2) 
    RETURNING "OrganizationId", "OrganizationName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

// Query UPDATE sebagai string literal
const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "Organizations" 
    SET "OrganizationName" = $1, "UpdatedDate" = NOW(), "UpdatedBy" = $3 
    WHERE "OrganizationId" = $2 
    RETURNING "OrganizationId", "OrganizationName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

// READ All
pub async fn get_all_organizations(pool: &PgPool) -> Result<Vec<Organization>, AppError> {
    let sql_query = format!("{} ORDER BY \"OrganizationName\" ASC", SELECT_BASE_QUERY);
    
    let organizations = sqlx::query_as::<_, Organization>(&sql_query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;
    info!("Fetched {} organizations.", organizations.len());
    Ok(organizations)
}

// READ By ID
pub async fn get_organization_by_id(pool: &PgPool, org_id: i32) -> Result<Organization, AppError> {
    let sql_query = format!("{} WHERE \"OrganizationId\" = $1", SELECT_BASE_QUERY);
    
    let organization = sqlx::query_as::<_, Organization>(&sql_query)
        .bind(org_id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::InternalError("Organization not found".to_string()),
            _ => AppError::DatabaseError(e),
        })?;
    Ok(organization)
}

// CREATE
pub async fn create_organization(pool: &PgPool, name: &str) -> Result<Organization, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Organizations\" WHERE LOWER(\"OrganizationName\") = LOWER($1))", name
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    if exists.unwrap_or(false) { return Err(AppError::InternalError("Organization name already exists".to_string())); }

    let new_organization = sqlx::query_as::<_, Organization>(CREATE_QUERY_SQL)
        .bind(name)
        .bind("Admin") // CreatedBy / UpdatedBy
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    info!("Organization '{}' created with ID: {}", new_organization.organization_name, new_organization.organization_id);
    Ok(new_organization)
}

// UPDATE
pub async fn update_organization(pool: &PgPool, org_id: i32, new_name: &str) -> Result<Organization, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Organizations\" WHERE LOWER(\"OrganizationName\") = LOWER($1) AND \"OrganizationId\" != $2)",
        new_name, org_id
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    if exists.unwrap_or(false) { return Err(AppError::InternalError("Organization name already exists".to_string())); }

    let updated_organization = sqlx::query_as::<_, Organization>(UPDATE_QUERY_SQL)
        .bind(new_name)
        .bind(org_id)
        .bind("Admin") // UpdatedBy
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?; 

    info!("Organization ID {} updated to '{}'", updated_organization.organization_id, updated_organization.organization_name);
    Ok(updated_organization)
}

// DELETE
pub async fn delete_organization(pool: &PgPool, org_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM \"Organizations\" WHERE \"OrganizationId\" = $1", org_id)
        .execute(pool).await.map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError("Organization not found for deletion".to_string()));
    }
    info!("Organization ID {} deleted.", org_id);
    Ok(())
}