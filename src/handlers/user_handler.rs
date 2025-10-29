use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::PaginationParams;
use crate::services::{role_service, user_service};

use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

#[derive(Deserialize)]
pub struct AddUserFormData {
    pub username: String,
    pub password: String,
    pub roles: Option<Vec<i32>>,
}

#[derive(Deserialize)]
pub struct EditUserFormData {
    pub username: String,
    pub email: Option<String>,
    pub roles: Option<Vec<i32>>,
}

pub async fn list_users(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let limit = params.limit as i64;
    let page = params.page.max(1) as i64;
    let offset = (page - 1) * limit;

    let users = user_service::get_all_users_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = user_service::count_users(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("users", &users);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "User Management");
    context.insert("header_title", "User List");

    let rendered = tera
        .render("users/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_user_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let username = get_username(pool.get_ref(), &session).await;
    let roles = role_service::get_all_roles(pool.get_ref()).await?;

    let mut context = tera::Context::new();
    context.insert("username", &username);
    context.insert("roles", &roles);

    let rendered = tera
        .render("users/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_user_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<AddUserFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let username = &form.username;
    let password = &form.password;
    let role_ids = form.roles.as_deref().unwrap_or(&[]);

    let result = user_service::create_user(pool.get_ref(), username, password, role_ids).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/users/list?page=1&limit=10"))
            .finish()),
        Err(AppError::UsernameExists) => {
            let roles = role_service::get_all_roles(pool.get_ref()).await?;
            let username_context = get_username(pool.get_ref(), &session).await;

            let mut context = tera::Context::new();
            context.insert(
                "error",
                "Username tersebut sudah digunakan. Silakan pilih yang lain.",
            );
            context.insert("username", &username_context);
            context.insert("roles", &roles);
            context.insert("old_username", username);

            let rendered = tera
                .render("users/add.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn show_edit_user_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let user_id = path.into_inner();

    let user = user_service::get_user_by_id(pool.get_ref(), user_id).await?;
    let available_roles = role_service::get_all_roles(pool.get_ref()).await?;
    let user_role_ids = user_service::get_user_role_ids(pool.get_ref(), user_id).await?;

    let username = get_username(pool.get_ref(), &session).await;

    let header_title = format!("Edit User: {}", user.username);

    let mut context = tera::Context::new();
    context.insert("user", &user);
    context.insert("available_roles", &available_roles);
    context.insert("user_role_ids", &user_role_ids);
    context.insert("username", &username);
    context.insert("title", "Edit User");
    context.insert("header_title", &header_title);

    let rendered = tera
        .render("users/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_user_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i64>,
    form: web::Form<EditUserFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let user_id = path.into_inner();
    let roles_to_set = form.roles.clone().unwrap_or_default();

    let result = user_service::update_user(
        pool.get_ref(),
        user_id,
        &form.username,
        form.email.as_deref(),
        &roles_to_set,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/users/list?page=1&limit=10"))
            .finish()),
        Err(AppError::UsernameExists) => {
            let user = user_service::get_user_by_id(pool.get_ref(), user_id).await?;
            let available_roles = role_service::get_all_roles(pool.get_ref()).await?;
            let user_role_ids = user_service::get_user_role_ids(pool.get_ref(), user_id).await?;
            let username = get_username(pool.get_ref(), &session).await;

            let mut context = tera::Context::new();
            context.insert("user", &user);
            context.insert("available_roles", &available_roles);
            context.insert("user_role_ids", &user_role_ids);
            context.insert("username", &username);
            context.insert("error", "Username sudah digunakan oleh pengguna lain.");

            let rendered = tera
                .render("users/edit.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_user_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i64>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let user_id = path.into_inner();
    user_service::delete_user(pool.get_ref(), user_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/users/list?page=1&limit=10"))
        .finish())
}
