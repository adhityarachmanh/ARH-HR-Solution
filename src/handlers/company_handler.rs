// src/handlers/company_handler.rs
use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;
use serde::Deserialize;

use crate::errors::AppError;
use crate::services::company_service;
use crate::handlers::permission_handler::get_username;

#[derive(Deserialize)]
pub struct CompanyFormData {
    pub name: String,
    pub prev_day_payroll: i32,
    pub current_day_payroll: i32,
}

pub async fn list_companies(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let companies = company_service::get_all_companies(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("companies", &companies);
    context.insert("username", &username);

    let rendered = tera.render("company/list.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_company_form(pool: web::Data<PgPool>, tera: web::Data<Tera>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("username", &username);
    let rendered = tera.render("company/add.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_company_action(pool: web::Data<PgPool>, tera: web::Data<Tera>, form: web::Form<CompanyFormData>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let result = company_service::create_company(
        pool.get_ref(), 
        &form.name, 
        form.prev_day_payroll, 
        form.current_day_payroll
    ).await;
    match result {
        Ok(_) => Ok(HttpResponse::Found().append_header(("Location", "/companies/list")).finish()),
        Err(e) => {
            tracing::error!("Company creation failed due to: {:?}", e);
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Failed to create company (Internal error).");
            context.insert("username", &username);
            let rendered = tera.render("company/add.html", &context).map_err(AppError::TeraError)?;
            Ok(HttpResponse::InternalServerError().body(rendered))
        }
    }
}

pub async fn show_edit_company_form(pool: web::Data<PgPool>, tera: web::Data<Tera>, session: Session, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let comp_id = path.into_inner();
    let company = company_service::get_company_by_id(pool.get_ref(), comp_id).await?;
    
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("company", &company);
    context.insert("username", &username);
    let rendered = tera.render("company/edit.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_company_action(pool: web::Data<PgPool>, tera: web::Data<Tera>, path: web::Path<i32>, form: web::Form<CompanyFormData>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let comp_id = path.into_inner();
    let result = company_service::update_company(
        pool.get_ref(), 
        comp_id, 
        &form.name, 
        form.prev_day_payroll, 
        form.current_day_payroll
    ).await;
    match result {
        Ok(_) => Ok(HttpResponse::Found().append_header(("Location", "/companies/list")).finish()),
        Err(e) => Err(e),
    }
}

pub async fn delete_company_action(pool: web::Data<PgPool>, session: Session, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let comp_id = path.into_inner();
    company_service::delete_company(pool.get_ref(), comp_id).await?;
    Ok(HttpResponse::Found().append_header(("Location", "/companies/list")).finish())
}