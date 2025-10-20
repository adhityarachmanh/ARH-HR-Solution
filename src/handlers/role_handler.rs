// src/handlers/role_handler.rs
use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;
use serde::Deserialize;

use crate::errors::AppError;
use crate::models::{User};
use crate::services::{role_service};

#[derive(Deserialize)]
pub struct AddRoleFormData {
    name: String,
}

// Handler untuk menampilkan halaman daftar peran (roles).
pub async fn list_roles(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if let Some(user_id) = session.get::<i64>("user_id").unwrap_or(None) {
        let current_user_result = sqlx::query_as!(
            User, 
            "SELECT id, username, email, password_hash, created_at, is_active, last_login FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(pool.get_ref())
        .await;

        let all_roles = role_service::get_all_roles(pool.get_ref()).await?;

        if let Ok(Some(current_user)) = current_user_result {
            let mut context = tera::Context::new();
            context.insert("roles", &all_roles);
            context.insert("username", &current_user.username);
            let rendered = tera.render("roles/list.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::Ok().body(rendered))
        } else {
            Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
        }
    } else {
        Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
    }
}


// Handler untuk menampilkan form tambah peran baru (GET /roles/add)
pub async fn show_add_role_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if let Some(user_id) = session.get::<i64>("user_id").unwrap_or(None) {
        let current_user_result = sqlx::query_as!(
            User, 
            "SELECT id, username, email, password_hash, created_at, is_active, last_login FROM users WHERE id = $1", 
            user_id
        )
        .fetch_optional(pool.get_ref())
        .await;

        if let Ok(Some(current_user)) = current_user_result {
            let mut context = tera::Context::new();
            context.insert("username", &current_user.username);
            
            let rendered = tera.render("roles/add.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::Ok().body(rendered))
        } else {
            Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
        }
    } else {
        Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
    }
}


// Handler untuk memproses form tambah peran (POST /roles/add)
pub async fn add_role_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<AddRoleFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let result = role_service::create_role(pool.get_ref(), &form.name).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/roles/list"))
                .finish())
        }
        Err(AppError::InternalError(msg)) if msg.contains("Role already exists") => {
            let mut context = tera::Context::new();
            context.insert("error", "Nama peran tersebut sudah ada. Silakan pilih nama lain.");
            
            let rendered = tera.render("roles/add.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

// Handler untuk menampilkan form edit peran (GET /roles/edit/{id})
pub async fn show_edit_role_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let role_id = path.into_inner();
    let role = role_service::get_role_by_id(pool.get_ref(), role_id).await?;
    
    let mut context = tera::Context::new();
    context.insert("role", &role);

    let rendered = tera.render("roles/edit.html", &context)
        .map_err(AppError::TeraError)?;
    
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk memproses form edit peran (POST /roles/edit/{id})
pub async fn edit_role_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<AddRoleFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let role_id = path.into_inner();
    let result = role_service::update_role(pool.get_ref(), role_id, &form.name).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/roles/list"))
                .finish())
        }
        Err(AppError::InternalError(msg)) if msg.contains("Role name already exists") => {
            let current_role = role_service::get_role_by_id(pool.get_ref(), role_id).await?;
            
            let mut context = tera::Context::new();
            context.insert("role", &current_role); 
            context.insert("error", "Nama peran tersebut sudah digunakan oleh peran lain.");
            
            let rendered = tera.render("roles/edit.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

// Handler untuk menghapus peran (POST /roles/delete/{id})
pub async fn delete_role_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    
    let role_id = path.into_inner();
    role_service::delete_role(pool.get_ref(), role_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/roles/list"))
        .finish())
}