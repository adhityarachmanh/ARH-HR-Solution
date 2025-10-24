// src/handlers/holiday_handler.rs
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;
use time::OffsetDateTime;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{Holiday, PaginationParams};
use crate::services::holiday_service;

#[derive(Deserialize)]
pub struct HolidayFormData {
    pub holiday_name: String,
    pub holiday_date: String,
    pub holiday_description: Option<String>,
    pub is_national_holiday: Option<bool>,
    pub is_company_holiday: Option<bool>,
    pub is_special_holiday: Option<bool>,
    pub is_mass_leave: Option<bool>,
}

fn map_form_to_holiday(form: &HolidayFormData) -> Result<Holiday, AppError> {
    let parsed_date = format!("{}T00:00:00Z", form.holiday_date);
    let holiday_date =
        OffsetDateTime::parse(&parsed_date, &time::format_description::well_known::Rfc3339)
            .map_err(|_| AppError::InternalError("Invalid date format submitted.".to_string()))?;

    Ok(Holiday {
        holiday_id: 0,
        holiday_name: Some(form.holiday_name.clone()),
        holiday_date: Some(holiday_date),
        holiday_description: form.holiday_description.clone(),
        is_national_holiday: Some(form.is_national_holiday.unwrap_or(false)),
        is_company_holiday: Some(form.is_company_holiday.unwrap_or(false)),
        is_special_holiday: Some(form.is_special_holiday.unwrap_or(false)),
        is_mass_leave: form.is_mass_leave.unwrap_or(false),
        holiday_year: Some(holiday_date.date().year()),
        created_date: None,
        created_by: None,
        updated_date: None,
        updated_by: None,
    })
}

pub async fn list_holidays(
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

    // 1. Fetch data yang sudah di-page
    let holidays =
        holiday_service::get_all_holidays_paginated(pool.get_ref(), limit, offset).await?;

    // 2. Hitung total data
    let total_records = holiday_service::count_holidays(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("holidays", &holidays);
    context.insert("username", &username);

    // Tambahkan data pagination ke context
    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    let rendered = tera
        .render("holidays/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_holiday_form(
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
    let rendered = tera
        .render("holidays/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_holiday_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    form: web::Form<HolidayFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let holiday_data = map_form_to_holiday(&form)?;
    let created_by = get_username(pool.get_ref(), &session).await;

    let result = holiday_service::create_holiday(pool.get_ref(), holiday_data, &created_by).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/holidays/list"))
            .finish()),
        Err(e) => Err(e),
    }
}

pub async fn show_edit_holiday_form(
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
    let holiday = holiday_service::get_holiday_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("holiday", &holiday);

    if let Some(date) = holiday.holiday_date {
        context.insert("holiday_date_str", &date.date().to_string());
    }

    context.insert("username", &username);
    let rendered = tera
        .render("holidays/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_holiday_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<HolidayFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let id = path.into_inner();

    let holiday_data = map_form_to_holiday(&form)?;
    let updated_by = get_username(pool.get_ref(), &session).await;

    let result =
        holiday_service::update_holiday(pool.get_ref(), id, holiday_data, &updated_by).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/holidays/list"))
            .finish()),
        Err(e) => Err(e),
    }
}

pub async fn delete_holiday_action(
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
    holiday_service::delete_holiday(pool.get_ref(), id).await?;
    Ok(HttpResponse::Found()
        .append_header(("Location", "/holidays/list"))
        .finish())
}

pub async fn sync_holidays_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let year = path.into_inner();

    match holiday_service::sync_holidays_for_year(pool.get_ref(), year).await {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/holidays/list"))
            .finish()),
        Err(e) => Err(e),
    }
}
