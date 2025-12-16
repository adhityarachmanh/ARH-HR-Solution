

use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::user_handler::get_username;
use crate::models::{EmploymentStatusFormData, PaginationParams};
use crate::services::employment_status_service;

pub async fn list_employment_statuses(
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

    let statuses = employment_status_service::get_all_employment_statuses_paginated(
        pool.get_ref(),
        limit,
        offset,
    )
    .await?;
    let total_records =
        employment_status_service::count_employment_statuses(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("statuses", &statuses);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);
    context.insert("title", "Master Employment Status");
    context.insert("header_title", "Employment Status List");

    let rendered = tera
        .render("employmentstatuses/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_employment_status_form(
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
    context.insert("title", "Add Employment Status");
    context.insert("header_title", "Add New Status");

    let rendered = tera
        .render("employmentstatuses/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_employment_status_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<EmploymentStatusFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let created_by = get_username(pool.get_ref(), &session).await;
    let result = employment_status_service::create_employment_status(
        pool.get_ref(),
        &form.name,
        &created_by,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/employmentstatuses/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("already exists") => {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Status name already exists.");
            context.insert("username", &username);
            context.insert("old_name", &form.name);
            context.insert("title", "Add Employment Status");
            context.insert("header_title", "Add New Status");
            let rendered = tera
                .render("employmentstatuses/add.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn show_edit_employment_status_form(
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
    let id = path.into_inner();
    let status = employment_status_service::get_employment_status_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    let header_title = format!("Edit Status: {}", status.employment_status_name);

    context.insert("status", &status);
    context.insert("username", &username);
    context.insert("title", "Edit Employment Status");
    context.insert("header_title", &header_title);

    let rendered = tera
        .render("employmentstatuses/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_employment_status_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<EmploymentStatusFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let id = path.into_inner();
    let updated_by = get_username(pool.get_ref(), &session).await;
    let result = employment_status_service::update_employment_status(
        pool.get_ref(),
        id,
        &form.name,
        &updated_by,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/employmentstatuses/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("already exists") => {
            let status =
                employment_status_service::get_employment_status_by_id(pool.get_ref(), id).await?;
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            let header_title = format!("Edit Status: {}", status.employment_status_name);
            context.insert("status", &status);
            context.insert("username", &username);
            context.insert("error", "Status name already exists for another record.");
            context.insert("title", "Edit Employment Status");
            context.insert("header_title", &header_title);

            let rendered = tera
                .render("employmentstatuses/edit.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_employment_status_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let id = path.into_inner();
    employment_status_service::delete_employment_status(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/employmentstatuses/list?page=1&limit=10"))
        .finish())
}
