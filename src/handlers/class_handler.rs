use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{ClassFormData, PaginationParams};
use crate::services::class_service;

async fn get_class_id_from_path(path: web::Path<i32>) -> Result<i32, AppError> {
    let id = path.into_inner();
    if id <= 0 {
        return Err(AppError::BadRequest("ID Kelas harus lebih dari 0.".into()));
    }
    Ok(id)
}

pub async fn list_classes(
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

    let classes = class_service::get_all_classes(pool.get_ref()).await?;

    let total_records = classes.len() as i64;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("classes", &classes);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master Classes");
    context.insert("header_title", "Class List");

    let rendered = tera
        .render("classes/list.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_class_form(
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
    context.insert("title", "Add Class");
    context.insert("header_title", "Tambah Kelas Baru");

    let rendered = tera
        .render("classes/add.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_class_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<ClassFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    if class_service::get_class_by_code(pool.get_ref(), &form.class_code)
        .await
        .is_ok()
    {
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "Class Code sudah ada.");
        context.insert("username", &username);
        context.insert("form_data", &form);
        return Ok(HttpResponse::BadRequest().body(
            tera.render("classes/add.html", &context)
                .map_err(AppError::TeraError)?,
        ));
    }

    let created_by = get_username(pool.get_ref(), &session).await;

    class_service::create_class(pool.get_ref(), &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/classes/list?page=1&limit=10"))
        .finish())
}

pub async fn show_edit_class_form(
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

    let id = get_class_id_from_path(path).await?;
    let class = class_service::get_class_by_id(pool.get_ref(), id).await?;
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("class", &class);
    context.insert("username", &username);
    context.insert(
        "title",
        &format!(
            "Edit Class: {}",
            class.class_code.as_deref().unwrap_or("N/A")
        ),
    );
    context.insert("header_title", "Edit Detail Kelas");

    let rendered = tera
        .render("classes/edit.html", &context)
        .map_err(AppError::TeraError)?;

    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_class_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<ClassFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_class_id_from_path(path).await?;

    let updated_by = get_username(pool.get_ref(), &session).await;

    if let Ok(existing_class) =
        class_service::get_class_by_code(pool.get_ref(), &form.class_code).await
    {
        if existing_class.class_id != id {
            let username = get_username(pool.get_ref(), &session).await;
            let mut context = tera::Context::new();
            context.insert("error", "Class Code sudah digunakan oleh Kelas lain.");
            context.insert("username", &username);

            if let Ok(class) = class_service::get_class_by_id(pool.get_ref(), id).await {
                context.insert("class", &class);
            }
            context.insert("form_data", &form);
            return Ok(HttpResponse::BadRequest().body(
                tera.render("classes/edit.html", &context)
                    .map_err(AppError::TeraError)?,
            ));
        }
    }

    class_service::update_class(pool.get_ref(), id, &form, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/classes/list?page=1&limit=10"))
        .finish())
}

pub async fn delete_class_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_class_id_from_path(path).await?;

    class_service::delete_class(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/classes/list?page=1&limit=10"))
        .finish())
}
