// src/handlers/branch_handler.rs
use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;
use serde::Deserialize;

use crate::errors::AppError;
use crate::services::{branch_service, company_service, timezone_service}; 
use crate::handlers::permission_handler::get_username; 

#[derive(Deserialize)]
pub struct BranchFormData {
    pub name: String,
    pub company_name: String, 
    pub comp_id: Option<i32>,
    pub timezone_id: Option<i32>,
}

// READ All
pub async fn list_branches(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let branches_details = branch_service::get_all_branch_details(pool.get_ref()).await?; 
    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("branches", &branches_details);
    context.insert("username", &username);

    let rendered = tera.render("branches/list.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}


// CREATE Show Form
pub async fn show_add_branch_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    
    // Panggil services yang terpisah
    let companies = company_service::get_all_companies(pool.get_ref()).await?;
    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("username", &username);
    context.insert("companies", &companies);
    context.insert("timezones", &timezones);

    let rendered = tera.render("branches/add.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// CREATE Action
pub async fn add_branch_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<BranchFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let result = branch_service::create_branch(
        pool.get_ref(), 
        &form.name, 
        &form.company_name, 
        form.comp_id, 
        form.timezone_id
    ).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Found().append_header(("Location", "/branches/list")).finish())
        }
        Err(AppError::InternalError(msg)) if msg.contains("Branch name already exists") => {
            let companies = company_service::get_all_companies(pool.get_ref()).await?;
            let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;
            let username = get_username(pool.get_ref(), &session).await;
            
            let mut context = tera::Context::new();
            context.insert("error", "Branch name already exists.");
            context.insert("username", &username);
            context.insert("companies", &companies);
            context.insert("timezones", &timezones);
            
            let rendered = tera.render("branches/add.html", &context).map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

// UPDATE Show Form
pub async fn show_edit_branch_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let branch_id = path.into_inner();
    let branch = branch_service::get_branch_by_id(pool.get_ref(), branch_id).await?;
    
    // MUAT DATA DROPDOWN
    let companies = company_service::get_all_companies(pool.get_ref()).await?;
    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;
    
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("branch", &branch);
    context.insert("username", &username);
    context.insert("companies", &companies);
    context.insert("timezones", &timezones);

    let rendered = tera.render("branches/edit.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// UPDATE Action
pub async fn edit_branch_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<BranchFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let branch_id = path.into_inner();
    let result = branch_service::update_branch(
        pool.get_ref(), 
        branch_id, 
        &form.name, 
        &form.company_name, 
        form.comp_id, 
        form.timezone_id
    ).await;

    match result {
        Ok(_) => {
            Ok(HttpResponse::Found().append_header(("Location", "/branches/list")).finish())
        }
        Err(AppError::InternalError(msg)) if msg.contains("Branch name already exists") => {
            // Ini akan memicu ResponseError handling di AppError
            Err(AppError::InternalError("Branch name already exists.".to_string()))
        }
        Err(e) => Err(e),
    }
}

// DELETE Action
pub async fn delete_branch_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    
    let branch_id = path.into_inner();
    branch_service::delete_branch(pool.get_ref(), branch_id).await?;

    Ok(HttpResponse::Found().append_header(("Location", "/branches/list")).finish())
}