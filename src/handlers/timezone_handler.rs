use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::user_handler::get_username;
use crate::models::{PaginationParams, TimeZoneFormData};
use crate::services::timezone_service;

pub async fn list_timezones(
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

    let timezones =
        timezone_service::get_all_timezones_paginated(pool.get_ref(), limit, offset).await?;

    let total_records = timezone_service::count_timezones(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("timezones", &timezones);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Time Zone Management");
    context.insert("header_title", "Time Zone List");

    let rendered = tera
        .render("timezones/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_timezone_form(
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
    context.insert("title", "Add Time Zone");
    context.insert("header_title", "Add New Time Zone");

    let rendered = tera
        .render("timezones/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_timezone_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<TimeZoneFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let result = timezone_service::create_timezone(pool.get_ref(), &form.name).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/timezones/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("already exists") => {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Time Zone name already exists.");
            context.insert("username", &username);
            let rendered = tera
                .render("timezones/add.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn show_edit_timezone_form(
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
    let timezone_id = path.into_inner();
    let timezone = timezone_service::get_timezone_by_id(pool.get_ref(), timezone_id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("timezone", &timezone);
    context.insert("username", &username);

    let rendered = tera
        .render("timezones/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_timezone_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<TimeZoneFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let timezone_id = path.into_inner();
    timezone_service::update_timezone(pool.get_ref(), timezone_id, &form.name).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/timezones/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_timezone_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let timezone_id = path.into_inner();
    timezone_service::delete_timezone(pool.get_ref(), timezone_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/timezones/list?page=1&limit=10"))
        .finish())
}
