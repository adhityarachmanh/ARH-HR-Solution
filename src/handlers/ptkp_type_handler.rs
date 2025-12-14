use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{PaginationParams, PtkpTypeFormData};
use crate::services::ptkp_type_service;

async fn get_ptkp_type_id_from_path(path: web::Path<i32>) -> Result<i32, AppError> {
    let id = path.into_inner();
    if id <= 0 {
        return Err(AppError::BadRequest(
            "PTKP Type ID must be greater than 0.".into(),
        ));
    }
    Ok(id)
}

pub async fn list_ptkp_types(
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

    let ptkp_types =
        ptkp_type_service::get_all_ptkp_types_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = ptkp_type_service::count_ptkp_types(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("ptkp_types", &ptkp_types);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master PTKP Types");
    context.insert("header_title", "PTKP Type List");

    let rendered = tera
        .render("ptkp_types/list.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_ptkp_type_form(
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
    context.insert("title", "Add PTKP Type");
    context.insert("header_title", "Add New PTKP Type");

    let rendered = tera
        .render("ptkp_types/add.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_ptkp_type_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<PtkpTypeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    if ptkp_type_service::get_ptkp_type_by_code(pool.get_ref(), &form.ptkp_code)
        .await
        .is_ok()
    {
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "PTKP Code already exists.");
        context.insert("username", &username);
        context.insert("form_data", &form);
        return Ok(HttpResponse::BadRequest().body(
            tera.render("ptkp_types/add.html", &context)
                .map_err(AppError::TeraError)?,
        ));
    }

    let created_by = get_username(pool.get_ref(), &session).await;

    ptkp_type_service::create_ptkp_type(pool.get_ref(), &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/ptkp_types/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_ptkp_type_form(
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

    let id = get_ptkp_type_id_from_path(path).await?;
    let ptkp_type = ptkp_type_service::get_ptkp_type_by_id(pool.get_ref(), id).await?;
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("ptkp_type", &ptkp_type);
    context.insert("username", &username);
    context.insert(
        "title",
        &format!(
            "Edit PTKP Type: {}",
            ptkp_type.ptkp_code.as_deref().unwrap_or("N/A")
        ),
    );
    context.insert("header_title", "Edit PTKP Type");

    let rendered = tera
        .render("ptkp_types/edit.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_ptkp_type_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<PtkpTypeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_ptkp_type_id_from_path(path).await?;

    let updated_by = get_username(pool.get_ref(), &session).await;

    if let Ok(existing_ptkp_type) =
        ptkp_type_service::get_ptkp_type_by_code(pool.get_ref(), &form.ptkp_code).await
    {
        if existing_ptkp_type.ptkp_type_id != id {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "PTKP Code is already used by another PTKP Type.");
            context.insert("username", &username);

            if let Ok(ptkp_type) = ptkp_type_service::get_ptkp_type_by_id(pool.get_ref(), id).await
            {
                context.insert("ptkp_type", &ptkp_type);
            }
            context.insert("form_data", &form);
            return Ok(HttpResponse::BadRequest().body(
                tera.render("ptkp_types/edit.html", &context)
                    .map_err(AppError::TeraError)?,
            ));
        }
    }

    ptkp_type_service::update_ptkp_type(pool.get_ref(), id, &form, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/ptkp_types/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_ptkp_type_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_ptkp_type_id_from_path(path).await?;

    ptkp_type_service::delete_ptkp_type(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/ptkp_types/list?page=1&limit=10"))
        .finish())
}