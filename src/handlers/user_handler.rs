// src/handlers/user_handler.rs
use actix_web::{web, HttpResponse, Result};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;
use crate::services::{user_service, role_service}; 
use crate::errors::AppError;
use crate::models::{User};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddUserFormData {
    username: String,
    password: String,
    roles: Option<Vec<i32>>, 
}

pub async fn list_users(
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

        let all_users = user_service::get_all_users(pool.get_ref()).await?;

        if let Ok(Some(current_user)) = current_user_result {
            let mut context = tera::Context::new();
            context.insert("users", &all_users);
            context.insert("username", &current_user.username);

            let rendered = tera.render("users/list.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::Ok().body(rendered))
        } else {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish())
        }
    } else {
        Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish())
    }
}

pub async fn show_add_user_form(
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

        let roles = role_service::get_all_roles(pool.get_ref()).await?;

        if let Ok(Some(current_user)) = current_user_result {
            let mut context = tera::Context::new();
            context.insert("username", &current_user.username);
            context.insert("roles", &roles); 

            let rendered = tera.render("users/add.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::Ok().body(rendered))
        } else {
            Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
        }
    } else {
        Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
    }
}

pub async fn add_user_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<AddUserFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let username = &form.username;
    let password = &form.password;
    let role_ids = form.roles.as_deref().unwrap_or(&[]);

    let result = user_service::create_user(
        pool.get_ref(), 
        username, 
        password, 
        role_ids
    ).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/users/list"))
                .finish())
        }
        Err(AppError::UsernameExists) => {
            let roles = role_service::get_all_roles(pool.get_ref()).await?;
            let mut context = tera::Context::new();
            context.insert("error", "Username tersebut sudah digunakan. Silakan pilih yang lain.");
            context.insert("roles", &roles);
            
            let rendered = tera.render("users/add.html", &context)
                .map_err(AppError::TeraError)?;
            
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}