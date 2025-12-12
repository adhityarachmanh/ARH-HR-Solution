
use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{BranchFormData, PaginationParams};
use crate::services::{branch_service, company_service, timezone_service};

pub async fn list_branches(
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

    let branches =
        branch_service::get_all_branches_paginated(pool.get_ref(), limit, offset).await?;

    let total_records = branch_service::count_branches(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;

    let mut context = tera::Context::new();
    context.insert("branches", &branches);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Branches");
    context.insert("header_title", "Branch List");

    let rendered = tera
        .render("branches/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_branch_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let companies = company_service::get_all_companies(pool.get_ref()).await?;
    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("username", &username);
    context.insert("companies", &companies);
    context.insert("timezones", &timezones);

    let rendered = tera
        .render("branches/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_branch_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<BranchFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let result = branch_service::create_branch(
        pool.get_ref(),
        &form.name,
        &form.company_name,
        form.comp_id,
        form.timezone_id,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/branches/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("Branch name already exists") => {
            let companies = company_service::get_all_companies(pool.get_ref()).await?;
            let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;
            let username = get_username(pool.get_ref(), &session).await;

            let mut context = tera::Context::new();
            context.insert("error", "Branch name already exists.");
            context.insert("username", &username);
            context.insert("companies", &companies);
            context.insert("timezones", &timezones);

            let rendered = tera
                .render("branches/add.html", &context)
                .map_err(AppError::TeraError)?;
            Ok(HttpResponse::BadRequest().body(rendered))
        }
        Err(e) => Err(e),
    }
}

pub async fn show_edit_branch_form(
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
    let branch_id = path.into_inner();
    let branch = branch_service::get_branch_by_id(pool.get_ref(), branch_id).await?;

    let companies = company_service::get_all_companies(pool.get_ref()).await?;
    let timezones = timezone_service::get_all_timezones(pool.get_ref()).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("branch", &branch);
    context.insert("username", &username);
    context.insert("companies", &companies);
    context.insert("timezones", &timezones);

    let rendered = tera
        .render("branches/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_branch_action(
    pool: web::Data<PgPool>,
    _tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<BranchFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let branch_id = path.into_inner();
    let result = branch_service::update_branch(
        pool.get_ref(),
        branch_id,
        &form.name,
        &form.company_name,
        form.comp_id,
        form.timezone_id,
    )
    .await;

    match result {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/branches/list?page=1&limit=10"))
            .finish()),
        Err(AppError::InternalError(msg)) if msg.contains("Branch name already exists") => Err(
            AppError::InternalError("Branch name already exists.".to_string()),
        ),
        Err(e) => Err(e),
    }
}

pub async fn delete_branch_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let branch_id = path.into_inner();
    branch_service::delete_branch(pool.get_ref(), branch_id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/branches/list?page=1&limit=10"))
        .finish())
}
