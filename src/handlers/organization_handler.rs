// src/handlers/organization_handler.rs
use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::services::organization_service;

#[derive(Deserialize)]
pub struct OrganizationFormData {
    pub name: String,
}

pub async fn list_organizations(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let organizations = organization_service::get_all_organizations(pool.get_ref()).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("organizations", &organizations);
    context.insert("username", &username);

    let rendered = tera
        .render("organizations/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_organization_form(
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
        .render("organizations/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_organization_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<OrganizationFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let result = organization_service::create_organization(pool.get_ref(), &form.name).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/organizations/list"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("already exists") => {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Organization name already exists.");
            context.insert("username", &username);
            let rendered = tera
                .render("organizations/add.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn show_edit_organization_form(
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
    let org_id = path.into_inner();
    let organization = organization_service::get_organization_by_id(pool.get_ref(), org_id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("organization", &organization);
    context.insert("username", &username);

    let rendered = tera
        .render("organizations/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_organization_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<OrganizationFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let org_id = path.into_inner();
    organization_service::update_organization(pool.get_ref(), org_id, &form.name).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/organizations/list"))
        .finish())
}

pub async fn delete_organization_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let org_id = path.into_inner();
    organization_service::delete_organization(pool.get_ref(), org_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/organizations/list"))
        .finish())
}
