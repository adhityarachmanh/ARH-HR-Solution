use crate::errors::AppError;
use crate::models::{
    StandardReference, StandardReferenceFormData, StandardReferenceItem,
    StandardReferenceItemFormData,
};
use sqlx::PgPool;
use tracing::info;

const SR_FIELDS: &str = r#"
    "StandardReferenceId" as standard_reference_id,
    "StandardReferenceName" as standard_reference_name,
    "IsActive" as is_active,
    "CreatedDate" as created_date,
    "CreatedBy" as created_by,
    "UpdatedDate" as updated_date,
    "UpdatedBy" as updated_by
"#;

const SRI_FIELDS: &str = r#"
    "ItemId" as item_id,
    "StandardReferenceId" as standard_reference_id,
    "ItemName" as item_name,
    "IsActive" as is_active,
    "Note" as note,
    "CreatedDate" as created_date,
    "CreatedBy" as created_by,
    "UpdatedDate" as updated_date,
    "UpdatedBy" as updated_by
"#;

pub async fn count_references(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("StandardReferenceId") FROM "StandardReferences""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_references_paginated(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<StandardReference>, AppError> {
    let rows = sqlx::query_as::<_, StandardReference>(&format!(
        r#"SELECT {SR_FIELDS} FROM "StandardReferences" ORDER BY "StandardReferenceId" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(rows)
}


pub async fn get_all_references(pool: &PgPool) -> Result<Vec<StandardReference>, AppError> {
    let rows = sqlx::query_as::<_, StandardReference>(&format!(
        r#"SELECT {SR_FIELDS} FROM "StandardReferences" ORDER BY "StandardReferenceId" ASC"#
    ))
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(rows)
}

pub async fn get_reference_by_id(pool: &PgPool, id: &str) -> Result<StandardReference, AppError> {
    let row = sqlx::query_as::<_, StandardReference>(&format!(
        r#"SELECT {SR_FIELDS} FROM "StandardReferences" WHERE "StandardReferenceId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    row.ok_or(AppError::NotFound(format!(
        "Reference ID {} not found.",
        id
    )))
}

pub async fn create_reference(
    pool: &PgPool,
    data: &StandardReferenceFormData,
    created_by: &str,
) -> Result<StandardReference, AppError> {
    let row = sqlx::query_as::<_, StandardReference>(&format!(
        r#"
        INSERT INTO "StandardReferences" ("StandardReferenceId", "StandardReferenceName", "IsActive", "CreatedBy", "UpdatedBy")
        VALUES ($1, $2, $3, $4, $4)
        RETURNING {SR_FIELDS}
        "#
    ))
    .bind(&data.standard_reference_id)
    .bind(&data.standard_reference_name)
    .bind(data.is_active)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    info!(
        "Standard Reference '{}' created by {}",
        row.standard_reference_id, created_by
    );
    Ok(row)
}

pub async fn update_reference(
    pool: &PgPool,
    id: &str,
    data: &StandardReferenceFormData,
    updated_by: &str,
) -> Result<StandardReference, AppError> {
    let row = sqlx::query_as::<_, StandardReference>(&format!(
        r#"
        UPDATE "StandardReferences"
        SET "StandardReferenceName" = $2, "IsActive" = $3, "UpdatedDate" = NOW(), "UpdatedBy" = $4
        WHERE "StandardReferenceId" = $1
        RETURNING {SR_FIELDS}
        "#
    ))
    .bind(id)
    .bind(&data.standard_reference_name)
    .bind(data.is_active)
    .bind(updated_by)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    
    let row = row.ok_or(AppError::NotFound(format!(
        "Reference ID {} not found for update.",
        id
    )))?;
    info!("Standard Reference '{}' updated by {}", row.standard_reference_id, updated_by);
    Ok(row)
}


pub async fn count_items_by_reference(pool: &PgPool, sr_id: &str) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(
        r#"SELECT COUNT("ItemId") FROM "StandardReferenceItems" WHERE "StandardReferenceId" = $1"#,
        sr_id
    )
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?
    .unwrap_or(0);
    Ok(count)
}

pub async fn get_items_by_reference_id_paginated(pool: &PgPool, sr_id: &str, limit: i64, offset: i64) -> Result<Vec<StandardReferenceItem>, AppError> {
    let rows = sqlx::query_as::<_, StandardReferenceItem>(&format!(
        r#"
        SELECT {SRI_FIELDS} FROM "StandardReferenceItems" 
        WHERE "StandardReferenceId" = $1 
        ORDER BY "ItemId" ASC 
        LIMIT $2 OFFSET $3
        "#
    ))
    .bind(sr_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(rows)
}

pub async fn get_items_by_reference_id(
    pool: &PgPool,
    sr_id: &str,
) -> Result<Vec<StandardReferenceItem>, AppError> {
    let rows = sqlx::query_as::<_, StandardReferenceItem>(&format!(
        r#"SELECT {SRI_FIELDS} FROM "StandardReferenceItems" WHERE "StandardReferenceId" = $1 ORDER BY "ItemId" ASC"#
    ))
    .bind(sr_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(rows)
}


pub async fn get_item_by_ids(
    pool: &PgPool,
    sr_id: &str,
    item_id: &str,
) -> Result<StandardReferenceItem, AppError> {
    let row = sqlx::query_as::<_, StandardReferenceItem>(&format!(
        r#"SELECT {SRI_FIELDS} FROM "StandardReferenceItems" WHERE "StandardReferenceId" = $1 AND "ItemId" = $2"#
    ))
    .bind(sr_id)
    .bind(item_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    row.ok_or(AppError::NotFound(format!(
        "Item ID {} for Ref {} not found.",
        item_id, sr_id
    )))
}

pub async fn create_item(
    pool: &PgPool,
    sr_id: &str,
    data: &StandardReferenceItemFormData,
    created_by: &str,
) -> Result<StandardReferenceItem, AppError> {
    let row = sqlx::query_as::<_, StandardReferenceItem>(&format!(
        r#"
        INSERT INTO "StandardReferenceItems" ("ItemId", "StandardReferenceId", "ItemName", "IsActive", "Note", "CreatedBy", "UpdatedBy")
        VALUES ($1, $2, $3, $4, $5, $6, $6)
        RETURNING {SRI_FIELDS}
        "#
    ))
    .bind(&data.item_id)
    .bind(sr_id)
    .bind(&data.item_name)
    .bind(data.is_active)
    .bind(&data.note)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    info!(
        "Item ID {} for Ref {} created by {}",
        row.item_id, sr_id, created_by
    );
    Ok(row)
}

pub async fn update_item(
    pool: &PgPool,
    sr_id: &str,
    item_id: &str,
    data: &StandardReferenceItemFormData,
    updated_by: &str,
) -> Result<StandardReferenceItem, AppError> {
    let row = sqlx::query_as::<_, StandardReferenceItem>(&format!(
        r#"
        UPDATE "StandardReferenceItems"
        SET "ItemName" = $3, "IsActive" = $4, "Note" = $5, "UpdatedDate" = NOW(), "UpdatedBy" = $6
        WHERE "StandardReferenceId" = $1 AND "ItemId" = $2
        RETURNING {SRI_FIELDS}
        "#
    ))
    .bind(sr_id)
    .bind(item_id)
    .bind(&data.item_name)
    .bind(data.is_active)
    .bind(&data.note)
    .bind(updated_by)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    row.ok_or(AppError::NotFound(format!(
        "Item ID {} for Ref {} not found for update.",
        item_id, sr_id
    )))
}

pub async fn delete_item(pool: &PgPool, sr_id: &str, item_id: &str) -> Result<(), AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM "StandardReferenceItems" WHERE "StandardReferenceId" = $1 AND "ItemId" = $2"#,
        sr_id,
        item_id
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Item ID {} for Ref {} not found for deletion.",
            item_id, sr_id
        )));
    }
    info!("Item ID {} for Ref {} deleted.", item_id, sr_id);
    Ok(())
}