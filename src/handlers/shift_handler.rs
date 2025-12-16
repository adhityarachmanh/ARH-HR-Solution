use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::user_handler::get_username;
use crate::models::{PaginationParams, ShiftFormData};
use crate::services::shift_service;

async fn get_shift_id_from_path(path: web::Path<i32>) -> Result<i32, AppError> {
    let id = path.into_inner();
    if id <= 0 {
        return Err(AppError::BadRequest("Shift ID must be greater than 0.".into()));
    }
    Ok(id)
}

pub async fn list_shifts(
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

    let shifts = shift_service::get_all_shifts_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = shift_service::count_shifts(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("shifts", &shifts);
    context.insert("username", &username);
    
    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);
    
    context.insert("title", "Master Shifts");
    context.insert("header_title", "Work Shift List");

    let rendered = tera
        .render("shifts/list.html", &context)
        .map_err(AppError::TeraError)?;
    
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_shift_form(
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
    context.insert("title", "Add Shift");
    context.insert("header_title", "Add New Work Shift");

    let rendered = tera
        .render("shifts/add.html", &context)
        .map_err(AppError::TeraError)?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_shift_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<ShiftFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    if shift_service::get_shift_by_code(pool.get_ref(), &form.shift_code).await.is_ok() {
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "Shift Code already exists.");
        context.insert("username", &username);
        context.insert("form_data", &form);
        return Ok(HttpResponse::BadRequest().body(
            tera.render("shifts/add.html", &context).map_err(AppError::TeraError)?
        ));
    }

    let created_by = get_username(pool.get_ref(), &session).await;
    
    shift_service::create_shift(pool.get_ref(), &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/shifts/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_shift_form(
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
    
    let id = get_shift_id_from_path(path).await?;
    let shift = shift_service::get_shift_by_id(pool.get_ref(), id).await?;
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    
    context.insert("shift", &shift);
    context.insert("username", &username);
    context.insert("title", &format!("Edit Shift: {}", shift.shift_code.as_deref().unwrap_or("N/A")));
    context.insert("header_title", "Edit Work Shift");

    let rendered = tera
        .render("shifts/edit.html", &context)
        .map_err(AppError::TeraError)?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_shift_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<ShiftFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_shift_id_from_path(path).await?;
    
    let updated_by = get_username(pool.get_ref(), &session).await;
    
    if let Ok(existing_shift) = shift_service::get_shift_by_code(pool.get_ref(), &form.shift_code).await {
        if existing_shift.shift_id != id {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Shift Code is already used by another Shift.");
            context.insert("username", &username);
            
            if let Ok(shift) = shift_service::get_shift_by_id(pool.get_ref(), id).await {
                 context.insert("shift", &shift);
            }
            context.insert("form_data", &form);
            return Ok(HttpResponse::BadRequest().body(
                tera.render("shifts/edit.html", &context).map_err(AppError::TeraError)?
            ));
        }
    }

    shift_service::update_shift(pool.get_ref(), id, &form, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/shifts/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_shift_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    
    let id = get_shift_id_from_path(path).await?;
    
    shift_service::delete_shift(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/shifts/list?page=1&limit=10"))
        .finish())
}