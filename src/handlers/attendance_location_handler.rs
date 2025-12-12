use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{AttendanceLocation, AttendanceLocationFormData, PaginationParams};
use crate::services::{attendance_location_service, timezone_service};

fn map_form_to_location(form: web::Form<AttendanceLocationFormData>) -> AttendanceLocation {
    let lat_decimal = BigDecimal::from_str(&form.latitude.to_string()).ok();
    let lon_decimal = BigDecimal::from_str(&form.longitude.to_string()).ok();

    AttendanceLocation {
        attendance_location_id: 0,
        time_zone_id: form.time_zone_id,
        location_name: Some(form.location_name.clone()),
        is_flexible: form.is_flexible,
        latitude: lat_decimal,
        longitude: lon_decimal,
        radius_tolerance_in_meter: Some(form.radius_tolerance_in_meter),
        created_date: None,
        created_by: None,
        updated_date: None,
        updated_by: None,
    }
}

pub async fn list_attendance_locations(
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

    let locations =
        attendance_location_service::get_all_locations_paginated(pool.get_ref(), limit, offset)
            .await?;

    let total_records =
        attendance_location_service::count_attendance_locations(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("locations", &locations);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Attendance Locations");
    context.insert("header_title", "Attendance Location List");

    let rendered = tera
        .render("attendancelocations/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_location_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;

    let google_api_key = env::var("GOOGLE_API").unwrap_or_else(|_| String::new());

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("username", &username);
    context.insert("timezones", &timezones);
    context.insert("GOOGLE_API", &google_api_key);

    let rendered = tera
        .render("attendancelocations/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_location_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    form: web::Form<AttendanceLocationFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let created_by = get_username(pool.get_ref(), &session).await;
    let location_data = map_form_to_location(form);

    attendance_location_service::create_attendance_location(
        pool.get_ref(),
        &location_data,
        &created_by,
    )
    .await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/attendancelocations/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_location_form(
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
    let location = attendance_location_service::get_location_by_id(pool.get_ref(), id).await?;

    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;

    let google_api_key = env::var("GOOGLE_API").unwrap_or_else(|_| String::new());

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("username", &username);
    context.insert("timezones", &timezones);
    context.insert("location", &location);
    context.insert("GOOGLE_API", &google_api_key);

    let rendered = tera
        .render("attendancelocations/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_location_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<AttendanceLocationFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = path.into_inner();
    let updated_by = get_username(pool.get_ref(), &session).await;
    let location_data = map_form_to_location(form);

    attendance_location_service::update_attendance_location(
        pool.get_ref(),
        id,
        &location_data,
        &updated_by,
    )
    .await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/attendancelocations/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_location_action(
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
    attendance_location_service::delete_location(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/attendancelocations/list?page=1&limit=10"))
        .finish())
}
