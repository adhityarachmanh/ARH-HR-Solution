use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::user_handler::get_username;
use crate::models::{PaginationParams, OvertimeSettingFormData};
use crate::services::overtime_setting_service;


async fn get_overtime_setting_id_from_path(path: web::Path<i32>) -> Result<i32, AppError> {
    let id = path.into_inner();
    if id <= 0 {
        return Err(AppError::BadRequest("Overtime Setting ID must be greater than 0.".into()));
    }
    Ok(id)
}

pub async fn list_overtime_settings(
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

    let settings = overtime_setting_service::get_all_overtime_settings_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = overtime_setting_service::count_overtime_settings(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("settings", &settings);
    context.insert("username", &username);
    
    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);
    
    context.insert("title", "Master Overtime Settings");
    context.insert("header_title", "Overtime Settings List"); 
    
    let rendered = tera
        .render("overtime_settings/list.html", &context)
        .map_err(AppError::TeraError)?;
    
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_overtime_setting_form(
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
    context.insert("title", "Add Overtime Setting");
    context.insert("header_title", "Add New Overtime Setting"); 

    let rendered = tera
        .render("overtime_settings/add.html", &context)
        .map_err(AppError::TeraError)?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_overtime_setting_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<OvertimeSettingFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }


    overtime_setting_service::create_overtime_setting(pool.get_ref(), &form).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/overtime_settings/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_overtime_setting_form(
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
    
    let id = get_overtime_setting_id_from_path(path).await?;
    let setting = overtime_setting_service::get_overtime_setting_by_id(pool.get_ref(), id).await?;
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    
    context.insert("setting", &setting);
    context.insert("username", &username);
    context.insert("title", &format!("Edit Overtime Setting: {}", setting.overtime_name.as_deref().unwrap_or("N/A")));
    context.insert("header_title", "Edit Overtime Setting"); 

    let rendered = tera
        .render("overtime_settings/edit.html", &context)
        .map_err(AppError::TeraError)?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_overtime_setting_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<OvertimeSettingFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_overtime_setting_id_from_path(path).await?;
    

    overtime_setting_service::update_overtime_setting(pool.get_ref(), id, &form).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/overtime_settings/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_overtime_setting_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    
    let id = get_overtime_setting_id_from_path(path).await?;
    
    overtime_setting_service::delete_overtime_setting(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/overtime_settings/list?page=1&limit=10"))
        .finish())
}