// src/handlers/job_position_handler.rs
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::JobPosition;
use crate::services::job_position_service;

#[derive(Deserialize)]
pub struct JobPositionFormData {
    pub name: String,
}

pub async fn list_job_positions(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let positions = job_position_service::get_all_job_positions(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("positions", &positions);
    context.insert("username", &username);

    let rendered = tera
        .render("job_positions/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_job_position_form(
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
        .render("job_positions/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_job_position_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<JobPositionFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let result = job_position_service::create_job_position(pool.get_ref(), &form.name).await;
    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/jobpositions/list"))
            .finish()),
        Err(e) => Err(e),
    }
}

pub async fn show_edit_job_position_form(
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
    let position = job_position_service::get_job_position_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("position", &position);
    context.insert("username", &username);
    let rendered = tera
        .render("job_positions/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_job_position_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<JobPositionFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let id = path.into_inner();
    let result = job_position_service::update_job_position(pool.get_ref(), id, &form.name).await;
    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/jobpositions/list"))
            .finish()),
        Err(e) => Err(e),
    }
}

pub async fn delete_job_position_action(
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
    job_position_service::delete_job_position(pool.get_ref(), id).await?;
    Ok(HttpResponse::Found()
        .append_header(("Location", "/jobpositions/list"))
        .finish())
}
