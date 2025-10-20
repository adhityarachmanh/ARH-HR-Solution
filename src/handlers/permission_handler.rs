// src/handlers/permission_handler.rs
use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;
use serde::Deserialize;

use crate::errors::AppError;
use crate::models::{Role, HardcodedPermission, User}; 
use crate::services::{role_service, permission_service}; 

#[derive(Deserialize)]
pub struct ManagePermissionFormData {
    permissions: Option<Vec<i32>>, 
}

pub async fn get_username(pool: &PgPool, session: &Session) -> String {
    if let Some(user_id) = session.get::<i64>("user_id").unwrap_or(None) {
        let query_result = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at, is_active, last_login FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool).await;

        if let Ok(Some(user)) = query_result {
            return user.username;
        }
    }
    "Guest".to_string()
}


pub async fn list_master_permissions(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    _session: Session,
) -> Result<HttpResponse, AppError> {
    if _session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let all_permissions: Vec<HardcodedPermission> = permission_service::get_all_permissions(pool.get_ref()).await?;
    let current_username = get_username(pool.get_ref(), &_session).await; 

    let mut context = tera::Context::new();
    context.insert("permissions", &all_permissions);
    context.insert("username", &current_username);

    let rendered = tera.render("permissions/list.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}


pub async fn manage_permissions_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    _session: Session, 
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    
    if _session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let role_id = path.into_inner();
    let role: Role = role_service::get_role_by_id(pool.get_ref(), role_id).await?;
    
    let all_permissions: Vec<HardcodedPermission> = permission_service::get_all_permissions(pool.get_ref()).await?;
    let owned_permission_ids = permission_service::get_permissions_for_role(pool.get_ref(), role_id).await?;
    let current_username = get_username(pool.get_ref(), &_session).await;

    let mut context = tera::Context::new();
    context.insert("role", &role);
    context.insert("all_permissions", &all_permissions);
    context.insert("owned_ids", &owned_permission_ids);
    context.insert("username", &current_username); 

    let rendered = tera.render("roles/permissions.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn save_permissions_action(
    pool: web::Data<PgPool>,
    form: web::Form<ManagePermissionFormData>,
    _session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {

    if _session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let role_id = path.into_inner();
    let new_permission_ids = form.permissions.clone().unwrap_or_default(); 

    let mut tx = pool.begin().await.map_err(AppError::DatabaseError)?;

    sqlx::query!("DELETE FROM role_permissions WHERE role_id = $1", role_id)
        .execute(&mut *tx).await.map_err(AppError::DatabaseError)?;

    for permission_id in new_permission_ids {
        sqlx::query!("INSERT INTO role_permissions (role_id, permission_id) VALUES ($1, $2)",
            role_id, permission_id)
        .execute(&mut *tx).await.map_err(AppError::DatabaseError)?;
    }
    
    tx.commit().await.map_err(AppError::DatabaseError)?;
    
    Ok(HttpResponse::Found().append_header(("Location", "/roles/list")).finish())
}