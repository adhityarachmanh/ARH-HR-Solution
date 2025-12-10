use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{Company, PaginationParams};
use crate::services::{company_service, zip_code_service};

#[derive(Deserialize)]
pub struct CompanyFormData {
    pub name: String,
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub phone_number: Option<String>,
    pub mobile_number: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub retirement_age: Option<i32>,
    pub bpjstk_pendapatan_pk: Option<f64>,
    pub bpjsks_pendapatan_pk: Option<f64>,
    pub bpjstk_pengurangan_pk: Option<f64>,
    pub bpjsks_pengurangan_pk: Option<f64>,
    pub bpjstk_pengurangan_pekerja: Option<f64>,
    pub bpjsks_pengurangan_pekerja: Option<f64>,
    pub prev_day_payroll: i32,
    pub current_day_payroll: i32,
}

fn map_form_to_company(form: &CompanyFormData) -> Company {
    Company {
        comp_id: 0,
        comp_name: Some(form.name.clone()),
        comp_address: form.address.clone(),
        comp_zip_code: form.zip_code.clone(),
        comp_phone_number: form.phone_number.clone(),
        comp_mobile_number: form.mobile_number.clone(),
        comp_email: form.email.clone(),
        comp_website: form.website.clone(),
        comp_retirement_age: form.retirement_age,

        prev_month_day_payroll: form.prev_day_payroll,
        current_month_day_payroll: form.current_day_payroll,

        bpjstk_pendapatan_pk: form.bpjstk_pendapatan_pk,
        bpjsks_pendapatan_pk: form.bpjsks_pendapatan_pk,
        bpjstk_pengurangan_pk: form.bpjstk_pengurangan_pk,
        bpjsks_pengurangan_pk: form.bpjsks_pengurangan_pk,
        bpjstk_pengurangan_pekerja: form.bpjstk_pengurangan_pekerja,
        bpjsks_pengurangan_pekerja: form.bpjsks_pengurangan_pekerja,

        created_date: None,
        created_by: None,
        updated_date: None,
        updated_by: None,
    }
}

pub async fn list_companies(
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

    let companies =
        company_service::get_all_companies_paginated(pool.get_ref(), limit, offset).await?;

    let total_records = company_service::count_companies(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("companies", &companies);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Companies");
    context.insert("header_title", "Company List");

    let rendered = tera
        .render("companies/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_company_form(
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
    context.insert("title", "Add Company");
    context.insert("header_title", "Add New Company");

    let rendered = tera
        .render("companies/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_company_action(
    pool: web::Data<PgPool>,
    form: web::Form<CompanyFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let company_data = map_form_to_company(&form);
    let created_by = get_username(pool.get_ref(), &session).await;

    company_service::create_company(pool.get_ref(), company_data, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_company_form(
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
    let company = company_service::get_company_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    let page_title = format!(
        "Edit Company: {}",
        company.comp_name.as_deref().unwrap_or("N/A")
    );
    context.insert("company", &company);
    context.insert("username", &username);

    context.insert("title", &page_title);
    context.insert("header_title", "Edit Company Info");

    let rendered = tera
        .render("companies/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_company_action(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    form: web::Form<CompanyFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = path.into_inner();
    let company_data = map_form_to_company(&form);
    let updated_by = get_username(pool.get_ref(), &session).await;

    company_service::update_company(pool.get_ref(), id, company_data, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_company_action(
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
    company_service::delete_company(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies/list?page=1&limit=10"))
        .finish())
}
