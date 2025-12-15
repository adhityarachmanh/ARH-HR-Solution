use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{
    PaginationParams, StandardReference, StandardReferenceFormData, StandardReferenceItem,
    StandardReferenceItemFormData,
};
use crate::services::standard_reference_service;


#[derive(Deserialize)]
pub struct ItemPath {
    pub sr_id: String,
    pub item_id: String,
}

pub async fn list_references(
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

    let references =
        standard_reference_service::get_all_references_paginated(pool.get_ref(), limit, offset)
            .await?;
    let total_records = standard_reference_service::count_references(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("references", &references);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert("title", "Master References");
    context.insert("header_title", "Master Data Reference List");

    let rendered = tera
        .render("standard_references/list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_reference_form(
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
    context.insert("title", "Add Master Reference");
    context.insert("header_title", "Tambah Master Reference Baru");

    let rendered = tera
        .render("standard_references/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn create_reference_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<StandardReferenceFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let created_by = get_username(pool.get_ref(), &session).await;

    if standard_reference_service::get_reference_by_id(pool.get_ref(), &form.standard_reference_id)
        .await
        .is_ok()
    {
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "Reference ID sudah ada.");
        context.insert("username", &username);
        context.insert("form_data", &form);
        return Ok(HttpResponse::BadRequest().body(
            tera.render("standard_references/add.html", &context)
                .map_err(AppError::TeraError)?,
        ));
    }

    standard_reference_service::create_reference(pool.get_ref(), &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/standard_references/list"))
        .finish())
}

pub async fn list_items_by_reference(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<String>,
    params: web::Query<PaginationParams>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let sr_id = path.into_inner();

    let limit = params.limit as i64;
    let page = params.page.max(1) as i64;
    let offset = (page - 1) * limit;

    let reference = standard_reference_service::get_reference_by_id(pool.get_ref(), &sr_id).await?;

    let items = standard_reference_service::get_items_by_reference_id_paginated(
        pool.get_ref(),
        &sr_id,
        limit,
        offset,
    )
    .await?;
    let total_records =
        standard_reference_service::count_items_by_reference(pool.get_ref(), &sr_id).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("reference", &reference);
    context.insert("items", &items);
    context.insert("username", &username);

    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);

    context.insert(
        "title",
        &format!("Items: {}", reference.standard_reference_name),
    );
    context.insert(
        "header_title",
        &format!("Detail Items: {}", reference.standard_reference_name),
    );

    let rendered = tera
        .render("standard_references/item_list.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_item_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let sr_id = path.into_inner();
    let reference = standard_reference_service::get_reference_by_id(pool.get_ref(), &sr_id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();
    context.insert("reference", &reference);
    context.insert("username", &username);
    context.insert(
        "title",
        &format!("Add Item to {}", reference.standard_reference_name),
    );
    context.insert(
        "header_title",
        &format!("Tambah Item Baru ({})", reference.standard_reference_name),
    );

    let rendered = tera
        .render("standard_references/item_add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_item_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<String>,
    form: web::Form<StandardReferenceItemFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let sr_id = path.into_inner();
    let created_by = get_username(pool.get_ref(), &session).await;

    if standard_reference_service::get_item_by_ids(pool.get_ref(), &sr_id, &form.item_id)
        .await
        .is_ok()
    {
        let reference =
            standard_reference_service::get_reference_by_id(pool.get_ref(), &sr_id).await?;
        let username = get_username(pool.get_ref(), &session).await;
        let mut context = tera::Context::new();
        context.insert("error", "Item ID sudah ada dalam referensi ini.");
        context.insert("reference", &reference);
        context.insert("username", &username);
        context.insert("form_data", &form);
        return Ok(HttpResponse::BadRequest().body(
            tera.render("standard_references/item_add.html", &context)
                .map_err(AppError::TeraError)?,
        ));
    }

    standard_reference_service::create_item(pool.get_ref(), &sr_id, &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", format!("/standard_references/items/{}", sr_id)))
        .finish())
}

pub async fn show_edit_item_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
    path: web::Path<ItemPath>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let path_data = path.into_inner();

    let reference =
        standard_reference_service::get_reference_by_id(pool.get_ref(), &path_data.sr_id).await?;
    let item = standard_reference_service::get_item_by_ids(
        pool.get_ref(),
        &path_data.sr_id,
        &path_data.item_id,
    )
    .await?;
    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("reference", &reference);
    context.insert("item", &item);
    context.insert("username", &username);
    context.insert(
        "title",
        &format!("Edit Item: {}", item.item_name.as_deref().unwrap_or("N/A")),
    );
    context.insert(
        "header_title",
        &format!(
            "Edit Item: {} ({})",
            item.item_id, reference.standard_reference_name
        ),
    );

    let rendered = tera
        .render("standard_references/item_edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_item_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<ItemPath>,
    form: web::Form<StandardReferenceItemFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let path_data = path.into_inner();
    let updated_by = get_username(pool.get_ref(), &session).await;

    standard_reference_service::update_item(
        pool.get_ref(),
        &path_data.sr_id,
        &path_data.item_id,
        &form,
        &updated_by,
    )
    .await?;

    Ok(HttpResponse::Found()
        .append_header((
            "Location",
            format!("/standard_references/items/{}", path_data.sr_id),
        ))
        .finish())
}

pub async fn delete_item_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<ItemPath>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    let path_data = path.into_inner();

    standard_reference_service::delete_item(pool.get_ref(), &path_data.sr_id, &path_data.item_id)
        .await?;

    Ok(HttpResponse::Found()
        .append_header((
            "Location",
            format!("/standard_references/items/{}", path_data.sr_id),
        ))
        .finish())
}
