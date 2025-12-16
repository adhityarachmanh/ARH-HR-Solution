use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::user_handler::get_username;
use crate::models::{JobLevelFormData, PaginationParams};
use crate::services::job_level_service;

pub async fn list_job_levels(
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

    let levels =
        job_level_service::get_all_job_levels_paginated(pool.get_ref(), limit, offset).await?;

    let total_records = job_level_service::count_job_levels(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("levels", &levels);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Job Levels");
    context.insert("header_title", "Job Levels List");

    let rendered = tera
        .render("joblevels/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_job_level_form(
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
    context.insert("title", "Add Job Level");
    context.insert("header_title", "Add New Job Level");

    let rendered = tera
        .render("joblevels/add.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_job_level_action(
    pool: web::Data<PgPool>,
    form: web::Form<JobLevelFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let created_by = get_username(pool.get_ref(), &session).await;

    job_level_service::create_job_level(pool.get_ref(), &form.name, form.order, &created_by)
        .await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/joblevels/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_job_level_form(
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
    let level = job_level_service::get_job_level_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    let header_title = format!(
        "Edit Job Level: {}",
        level.job_level_name.as_deref().unwrap_or("N/A")
    );

    context.insert("level", &level);
    context.insert("username", &username);
    context.insert("title", "Edit Job Level");

    context.insert("header_title", &header_title);

    let rendered = tera
        .render("joblevels/edit.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_job_level_action(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    form: web::Form<JobLevelFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = path.into_inner();
    let updated_by = get_username(pool.get_ref(), &session).await;

    job_level_service::update_job_level(pool.get_ref(), id, &form.name, form.order, &updated_by)
        .await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/joblevels/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_job_level_action(
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
    job_level_service::delete_job_level(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/joblevels/list?page=1&limit=10"))
        .finish())
}
