use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::PaginationParams;
use crate::services::role_service;

#[derive(Deserialize)]
pub struct AddRoleFormData {
    name: String,
}

pub async fn list_roles(
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

    let roles = role_service::get_all_roles_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = role_service::count_roles(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("roles", &roles);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Roles");
    context.insert("header_title", "Role List");

    let rendered = tera
        .render("roles/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_role_form(
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

    let mut context = tera::Context::new();
    context.insert("username", &username);
    context.insert("title", "Add Role");
    context.insert("header_title", "Add New Role");

    let rendered = tera
        .render("roles/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_role_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<AddRoleFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let result = role_service::create_role(pool.get_ref(), &form.name).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/roles/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("Role already exists") => {
            let username = get_username(pool.get_ref(), &session).await;

            let mut context = tera::Context::new();
            context.insert(
                "error",
                "Nama peran tersebut sudah ada. Silakan pilih nama lain.",
            );
            context.insert("username", &username);
            context.insert("old_name", &form.name);
            context.insert("title", "Add Role");
            context.insert("header_title", "Add New Role");

            let rendered = tera
                .render("roles/add.html", &context)
                .map_err(AppError::TeraError)?;

            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn show_edit_role_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let role_id = path.into_inner();
    let role = role_service::get_role_by_id(pool.get_ref(), role_id).await?;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    let header_title = format!("Edit Status: {}", role.name);

    context.insert("role", &role);
    context.insert("username", &username);
    context.insert("title", "Edit Role");
    context.insert("header_title", &header_title);

    let rendered = tera
        .render("roles/edit.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_role_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<AddRoleFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let role_id = path.into_inner();
    let result = role_service::update_role(pool.get_ref(), role_id, &form.name).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/roles/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("Role name already exists") => {
            let current_role = role_service::get_role_by_id(pool.get_ref(), role_id).await?;
            let username = get_username(pool.get_ref(), &session).await;

            let mut context = tera::Context::new();
            context.insert("role", &current_role);
            context.insert("username", &username);
            context.insert(
                "error",
                "Nama peran tersebut sudah digunakan oleh peran lain.",
            );
            context.insert("title", "Edit Role");
            context.insert("header_title", "Edit Role");

            let rendered = tera
                .render("roles/edit.html", &context)
                .map_err(AppError::TeraError)?;

            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_role_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let role_id = path.into_inner();
    role_service::delete_role(pool.get_ref(), role_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/roles/list?page=1&limit=10"))
        .finish())
}
