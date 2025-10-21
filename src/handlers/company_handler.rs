// src/handlers/company_handler.rs
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::Company;
use crate::services::company_service; // Asumsi helper function ada

// Struct Form Data untuk menerima input dari form HTML
#[derive(Deserialize)]
pub struct CompanyFormData {
    pub name: String,

    // Detail Kontak & Alamat
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub phone_number: Option<String>,
    pub mobile_number: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub retirement_age: Option<i32>,

    // Field BPJS
    pub bpjstk_pendapatan_pk: Option<f64>,
    pub bpjsks_pendapatan_pk: Option<f64>,
    pub bpjstk_pengurangan_pk: Option<f64>,
    pub bpjsks_pengurangan_pk: Option<f64>,
    pub bpjstk_pengurangan_pekerja: Option<f64>,
    pub bpjsks_pengurangan_pekerja: Option<f64>,

    // Payroll Days (Asumsi NOT NULL)
    pub prev_day_payroll: i32,
    pub current_day_payroll: i32,
}

// Helper untuk mengekstrak Option<&str> dari Option<String>
fn as_deref_option(s: &Option<String>) -> Option<&str> {
    s.as_deref()
}

// Handler untuk menampilkan daftar semua perusahaan
pub async fn list_companies(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let companies = company_service::get_all_companies(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("companies", &companies);
    context.insert("username", &username);

    let rendered = tera
        .render("company/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk menampilkan form tambah perusahaan
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
    let rendered = tera
        .render("company/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk memproses data dari form CREATE
#[allow(clippy::too_many_arguments)]
pub async fn add_company_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    form: web::Form<CompanyFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let result = company_service::create_company(
        pool.get_ref(),
        &form.name,
        as_deref_option(&form.address),
        as_deref_option(&form.zip_code),
        as_deref_option(&form.phone_number),
        as_deref_option(&form.mobile_number),
        as_deref_option(&form.email),
        as_deref_option(&form.website),
        form.retirement_age,
        form.bpjstk_pendapatan_pk,
        form.bpjsks_pendapatan_pk,
        form.bpjstk_pengurangan_pk,
        form.bpjsks_pengurangan_pk,
        form.bpjstk_pengurangan_pekerja,
        form.bpjsks_pengurangan_pekerja,
        form.prev_day_payroll,
        form.current_day_payroll,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/companies/list"))
            .finish()),
        Err(e) => Err(e),
    }
}

// Handler untuk menampilkan form edit perusahaan
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
    let comp_id = path.into_inner();
    let company = company_service::get_company_by_id(pool.get_ref(), comp_id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("company", &company);
    context.insert("username", &username);
    let rendered = tera
        .render("company/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk memproses data dari form UPDATE
#[allow(clippy::too_many_arguments)]
pub async fn edit_company_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<CompanyFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let comp_id = path.into_inner();

    let result = company_service::update_company(
        pool.get_ref(),
        comp_id,
        &form.name,
        as_deref_option(&form.address),
        as_deref_option(&form.zip_code),
        as_deref_option(&form.phone_number),
        as_deref_option(&form.mobile_number),
        as_deref_option(&form.email),
        as_deref_option(&form.website),
        form.retirement_age,
        form.bpjstk_pendapatan_pk,
        form.bpjsks_pendapatan_pk,
        form.bpjstk_pengurangan_pk,
        form.bpjsks_pengurangan_pk,
        form.bpjstk_pengurangan_pekerja,
        form.bpjsks_pengurangan_pekerja,
        form.prev_day_payroll,
        form.current_day_payroll,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/companies/list"))
            .finish()),
        Err(e) => Err(e),
    }
}

// Handler untuk menghapus perusahaan
pub async fn delete_company_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let comp_id = path.into_inner();
    company_service::delete_company(pool.get_ref(), comp_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies/list"))
        .finish())
}
