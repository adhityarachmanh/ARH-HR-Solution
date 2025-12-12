use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{GradeFormData, PaginationParams};
use crate::services::grade_service;

async fn get_grade_id_from_path(path: web::Path<i32>) -> Result<i32, AppError> {
    let id = path.into_inner();
    if id <= 0 {
        return Err(AppError::BadRequest("ID Grade harus lebih dari 0.".into()));
    }
    Ok(id)
}

pub async fn list_grades(
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

    let grades = grade_service::get_all_grades_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = grade_service::count_grades(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("grades", &grades);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Grades");
    context.insert("header_title", "Daftar Jenjang Jabatan");

    let rendered = tera
        .render("grades/list.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_grade_form(
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
    context.insert("title", "Add Grade");
    context.insert("header_title", "Tambah Jenjang Baru");

    let rendered = tera
        .render("grades/add.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_grade_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<GradeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    if grade_service::get_grade_by_code(pool.get_ref(), &form.grade_code)
        .await
        .is_ok()
    {
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "Kode Jenjang sudah ada.");
        context.insert("username", &username);
        context.insert("form_data", &form);
        return Ok(HttpResponse::BadRequest().body(
            tera.render("grades/add.html", &context)
                .map_err(AppError::TeraError)?,
        ));
    }

    let created_by = get_username(pool.get_ref(), &session).await;

    grade_service::create_grade(pool.get_ref(), &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/grades/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_grade_form(
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

    let id = get_grade_id_from_path(path).await?;
    let grade = grade_service::get_grade_by_id(pool.get_ref(), id).await?;
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("grade", &grade);
    context.insert("username", &username);
    context.insert("title", &format!("Edit Grade: {}", grade.grade_code));
    context.insert("header_title", "Edit Jenjang Jabatan");

    let rendered = tera
        .render("grades/edit.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_grade_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<GradeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_grade_id_from_path(path).await?;

    let updated_by = get_username(pool.get_ref(), &session).await;

    if let Ok(existing_grade) =
        grade_service::get_grade_by_code(pool.get_ref(), &form.grade_code).await
    {
        if existing_grade.grade_id != id {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Kode Jenjang sudah digunakan oleh Jenjang lain.");
            context.insert("username", &username);

            if let Ok(grade) = grade_service::get_grade_by_id(pool.get_ref(), id).await {
                context.insert("grade", &grade);
            }
            context.insert("form_data", &form);
            return Ok(HttpResponse::BadRequest().body(
                tera.render("grades/edit.html", &context)
                    .map_err(AppError::TeraError)?,
            ));
        }
    }

    grade_service::update_grade(pool.get_ref(), id, &form, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/grades/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_grade_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_grade_id_from_path(path).await?;

    grade_service::delete_grade(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/grades/list?page=1&limit=10"))
        .finish())
}
