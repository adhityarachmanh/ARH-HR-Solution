// src/handlers/timezone_handler.rs
use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use sqlx::PgPool;
use serde::Deserialize;

use crate::errors::AppError;
use crate::services::timezone_service; 
use crate::handlers::permission_handler::get_username; 

#[derive(Deserialize)]
pub struct TimeZoneFormData {
    pub name: String,
}

// Handler untuk menampilkan daftar TimeZone (READ ALL)
pub async fn list_timezones(pool: web::Data<PgPool>, tera: web::Data<Tera>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    
    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("timezones", &timezones);
    context.insert("username", &username);
    
    let rendered = tera.render("timezones/list.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk menampilkan form Add (CREATE)
pub async fn show_add_timezone_form(pool: web::Data<PgPool>, tera: web::Data<Tera>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("username", &username);
    let rendered = tera.render("timezones/add.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk memproses aksi Add (CREATE)
pub async fn add_timezone_action(pool: web::Data<PgPool>, tera: web::Data<Tera>, form: web::Form<TimeZoneFormData>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let result = timezone_service::create_timezone(pool.get_ref(), &form.name).await;

    match result {
        Ok(_) => Ok(HttpResponse::Found().append_header(("Location", "/timezones/list")).finish()),
        Err(AppError::InternalError(msg)) if msg.contains("already exists") => {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Time Zone name already exists.");
            context.insert("username", &username);
            let rendered = tera.render("timezones/add.html", &context).map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

// Handler untuk menampilkan form Edit (UPDATE)
pub async fn show_edit_timezone_form(pool: web::Data<PgPool>, tera: web::Data<Tera>, session: Session, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    let timezone_id = path.into_inner();
    let timezone = timezone_service::get_timezone_by_id(pool.get_ref(), timezone_id).await?;
    
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("timezone", &timezone);
    context.insert("username", &username);

    let rendered = tera.render("timezones/edit.html", &context).map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

// Handler untuk memproses aksi Edit (UPDATE)
pub async fn edit_timezone_action(pool: web::Data<PgPool>, _tera: web::Data<Tera>, path: web::Path<i32>, form: web::Form<TimeZoneFormData>, session: Session) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }

    let timezone_id = path.into_inner();
    timezone_service::update_timezone(pool.get_ref(), timezone_id, &form.name).await?;

    Ok(HttpResponse::Found().append_header(("Location", "/timezones/list")).finish())
}

// Handler untuk memproses aksi Delete (DELETE)
pub async fn delete_timezone_action(pool: web::Data<PgPool>, session: Session, path: web::Path<i32>) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish());
    }
    
    let timezone_id = path.into_inner();
    timezone_service::delete_timezone(pool.get_ref(), timezone_id).await?;

    Ok(HttpResponse::Found().append_header(("Location", "/timezones/list")).finish())
}