use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{PaginationParams, ZipCode};
use crate::services::zip_code_service;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
#[derive(Deserialize)]
pub struct ZipCodeFormData {
    pub zip_code_id: String,
    pub street_name: Option<String>,
    pub district: String,
    pub county: String,
    pub city: String,
    pub sr_province: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    pub zip_postal_code: Option<String>,
}

fn map_form_to_zip_code(form: &ZipCodeFormData) -> Result<ZipCode, AppError> {
    Ok(ZipCode {
        zip_code_id: form.zip_code_id.clone(),
        street_name: form.street_name.clone(),
        district: form.district.clone(),
        county: form.county.clone(),
        city: form.city.clone(),
        sr_province: form.sr_province.clone(),

        latitude: Some(form.latitude),
        longitude: Some(form.longitude),

        zip_postal_code: form.zip_postal_code.clone(),
        last_update_date_time: None,
        last_update_by_user_id: None,
    })
}

pub async fn search_zip_codes_json(
    pool: web::Data<PgPool>,
    query: web::Query<SearchQuery>,
) -> Result<HttpResponse, AppError> {
    let limit = query.limit.unwrap_or(30); 
    let offset = query.offset.unwrap_or(0);
    
    let results = zip_code_service::search_zip_codes(pool.get_ref(), &query.q, limit, offset).await?;
    
    Ok(HttpResponse::Ok().json(results))
}

pub async fn list_zip_codes(
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

    let zip_codes =
        zip_code_service::get_all_zip_codes_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = zip_code_service::count_zip_codes(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("zip_codes", &zip_codes);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);
    context.insert("title", "Master Zip Codes");
    context.insert("header_title", "Zip Code List");

    let rendered = tera
        .render("zipcodes/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_zip_code_form(
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
    context.insert("title", "Add Zip Code");
    context.insert("header_title", "Add New Zip Code");

    let google_api_key = std::env::var("GOOGLE_API").unwrap_or_else(|_| String::new());
    context.insert("GOOGLE_API", &google_api_key);

    let rendered = tera
        .render("zipcodes/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_zip_code_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<ZipCodeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let created_by = get_username(pool.get_ref(), &session).await;

    if zip_code_service::get_zip_code_by_id(pool.get_ref(), &form.zip_code_id)
        .await
        .is_ok()
    {
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "Zip Code ID already exists.");
        context.insert("username", &username);
        let rendered = tera
            .render("zipcodes/add.html", &context)
            .map_err(AppError::TeraError)?;
        return Ok(HttpResponse::BadRequest().body(rendered));
    }

    let zip_data = map_form_to_zip_code(&form).map_err(|e| e)?;

    zip_code_service::create_zip_code(pool.get_ref(), &zip_data, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/zipcodes/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_zip_code_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let id = path.into_inner();
    let zip_code = zip_code_service::get_zip_code_by_id(pool.get_ref(), &id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    let google_api_key = std::env::var("GOOGLE_API").unwrap_or_else(|_| String::new());

    let header_title = format!(
        "Edit Zip Code: {}",
        zip_code.zip_postal_code.as_deref().unwrap_or("N/A")
    );

    context.insert("zip_code", &zip_code);
    context.insert("username", &username);
    context.insert("GOOGLE_API", &google_api_key);
    context.insert("title", "Edit Zip Code");
    context.insert("header_title", &header_title);

    let rendered = tera
        .render("zipcodes/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_zip_code_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<String>,
    form: web::Form<ZipCodeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = path.into_inner();
    let updated_by = get_username(pool.get_ref(), &session).await;

    let zip_data = map_form_to_zip_code(&form).map_err(|e| e)?;

    zip_code_service::update_zip_code(pool.get_ref(), &id, &zip_data, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/zipcodes/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_zip_code_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let id = path.into_inner();
    zip_code_service::delete_zip_code(pool.get_ref(), &id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/zipcodes/list?page=1&limit=10"))
        .finish())
}
