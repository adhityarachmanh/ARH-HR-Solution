// src/handlers/job_level_handler.rs

use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::services::job_level_service;

// Struct untuk form data (Add dan Edit)
#[derive(Deserialize)]
pub struct JobLevelFormData {
    pub name: String,
    pub order: Option<i32>,
}

// --- READ All ---
pub async fn list_job_levels(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let levels = job_level_service::get_all_job_levels(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("levels", &levels);
    context.insert("username", &username);
    context.insert("title", "Master Job Levels");
    context.insert("header_title", "Job Levels List");

    let rendered = tera
        .render("joblevels/list.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

// --- CREATE Show Form ---
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

// --- CREATE Action ---
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
        .append_header(("Location", "/joblevels/list"))
        .finish())
}

// --- UPDATE Show Form ---
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

    // --- PERBAIKAN DIMULAI DI SINI ---
    // Simpan String yang dihasilkan oleh format!
    let header_title = format!(
        "Edit Job Level: {}",
        level.job_level_name.as_deref().unwrap_or("N/A")
    );
    // --- PERBAIKAN SELESAI DI SINI ---

    context.insert("level", &level);
    context.insert("username", &username);
    context.insert("title", "Edit Job Level");

    // Gunakan referensi (&) ke String yang sudah tersimpan
    context.insert("header_title", &header_title); // <--- ERROR TERATASI DI BARIS INI

    let rendered = tera
        .render("joblevels/edit.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

// --- UPDATE Action ---
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
        .append_header(("Location", "/joblevels/list"))
        .finish())
}

// --- DELETE Action ---
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
        .append_header(("Location", "/joblevels/list"))
        .finish())
}
