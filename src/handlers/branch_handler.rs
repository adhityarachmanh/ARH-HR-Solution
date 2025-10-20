// src/handlers/branch_handler.rs
use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;

use crate::errors::AppError;
use crate::services::branch_service;
use crate::handlers::permission_handler::get_username;

pub async fn list_branches(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let branches = branch_service::get_all_branches(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("branches", &branches);
    context.insert("username", &username);

    let rendered = tera.render("hris/branches_list.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}