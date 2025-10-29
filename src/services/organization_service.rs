use crate::errors::AppError;
use crate::models::Organization;
use sqlx::PgPool;
use tracing::info;

const ORGANIZATION_FIELDS_SQL_LIST: &str = r#""OrganizationId" as organization_id, "OrganizationName" as organization_name, "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by"#;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "Organizations" ("OrganizationName", "CreatedBy", "UpdatedBy") 
    VALUES ($1, $2, $2) 
    RETURNING "OrganizationId", "OrganizationName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "Organizations" 
    SET "OrganizationName" = $1, "UpdatedDate" = NOW(), "UpdatedBy" = $3 
    WHERE "OrganizationId" = $2 
    RETURNING "OrganizationId", "OrganizationName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy"
"#;

pub async fn count_organizations(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("OrganizationId") FROM "Organizations""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_organizations_paginated(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Organization>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Organizations\" ORDER BY \"OrganizationName\" ASC LIMIT $1 OFFSET $2",
        ORGANIZATION_FIELDS_SQL_LIST
    );

    let organizations = sqlx::query_as::<_, Organization>(&sql_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(organizations)
}

pub async fn get_all_organizations(pool: &PgPool) -> Result<Vec<Organization>, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Organizations\" ORDER BY \"OrganizationName\" ASC",
        ORGANIZATION_FIELDS_SQL_LIST
    );

    let organizations = sqlx::query_as::<_, Organization>(&sql_query)
        .fetch_all(pool)
        .await
        .map_err(AppError::DatabaseError)?;
    info!("Fetched {} organizations.", organizations.len());
    Ok(organizations)
}

pub async fn get_organization_by_id(pool: &PgPool, org_id: i32) -> Result<Organization, AppError> {
    let sql_query = format!(
        "SELECT {} FROM \"Organizations\" WHERE \"OrganizationId\" = $1",
        ORGANIZATION_FIELDS_SQL_LIST
    );

    let organization = sqlx::query_as::<_, Organization>(&sql_query)
        .bind(org_id)
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::InternalError("Organization not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })?;
    Ok(organization)
}

pub async fn create_organization(pool: &PgPool, name: &str) -> Result<Organization, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Organizations\" WHERE LOWER(\"OrganizationName\") = LOWER($1))", name
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Organization name already exists".to_string(),
        ));
    }

    let create_sql_return_alias = format!(
        r#"
        INSERT INTO "Organizations" ("OrganizationName", "CreatedDate", "CreatedBy", "UpdatedDate", "UpdatedBy") 
        VALUES ($1, NOW(), $2, NOW(), $2) 
        RETURNING {}
        "#,
        ORGANIZATION_FIELDS_SQL_LIST
    );

    let new_organization = sqlx::query_as::<_, Organization>(&create_sql_return_alias)
        .bind(name)
        .bind("Admin")
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Organization '{}' created with ID: {}",
        new_organization.organization_name, new_organization.organization_id
    );
    Ok(new_organization)
}

pub async fn update_organization(
    pool: &PgPool,
    org_id: i32,
    new_name: &str,
) -> Result<Organization, AppError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"Organizations\" WHERE LOWER(\"OrganizationName\") = LOWER($1) AND \"OrganizationId\" != $2)",
        new_name, org_id
    ).fetch_one(pool).await.map_err(AppError::DatabaseError)?;

    if exists.unwrap_or(false) {
        return Err(AppError::InternalError(
            "Organization name already exists".to_string(),
        ));
    }

    let update_sql_return_alias = format!(
        r#"
        UPDATE "Organizations" 
        SET "OrganizationName" = $1, "UpdatedDate" = NOW(), "UpdatedBy" = $3 
        WHERE "OrganizationId" = $2 
        RETURNING {}
        "#,
        ORGANIZATION_FIELDS_SQL_LIST
    );

    let updated_organization = sqlx::query_as::<_, Organization>(&update_sql_return_alias)
        .bind(new_name)
        .bind(org_id)
        .bind("Admin")
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    info!(
        "Organization ID {} updated to '{}'",
        updated_organization.organization_id, updated_organization.organization_name
    );
    Ok(updated_organization)
}

pub async fn delete_organization(pool: &PgPool, org_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(
        "DELETE FROM \"Organizations\" WHERE \"OrganizationId\" = $1",
        org_id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError(
            "Organization not found for deletion".to_string(),
        ));
    }
    info!("Organization ID {} deleted.", org_id);
    Ok(())
}
