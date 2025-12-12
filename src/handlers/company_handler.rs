use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;

use crate::errors::AppError;
use crate::handlers::permission_handler::get_username;
use crate::models::{Company, CompanyFormData};
use crate::services::{company_service, zip_code_service};

fn map_form_to_company(form: &CompanyFormData) -> Company {
    Company {
        comp_id: 0,
        comp_name: Some(form.name.clone()),
        comp_address: form.address.clone(),
        comp_zip_code: form.zip_code.clone(),
        comp_phone_number: form.phone_number.clone(),
        comp_mobile_number: form.mobile_number.clone(),
        comp_email: form.email.clone(),
        comp_website: form.website.clone(),
        comp_retirement_age: form.retirement_age,

        prev_month_day_payroll: form.prev_day_payroll,
        current_month_day_payroll: form.current_day_payroll,

        bpjstk_pendapatan_pk: form.bpjstk_pendapatan_pk,
        bpjsks_pendapatan_pk: form.bpjsks_pendapatan_pk,
        bpjstk_pengurangan_pk: form.bpjstk_pengurangan_pk,
        bpjsks_pengurangan_pk: form.bpjsks_pengurangan_pk,
        bpjstk_pengurangan_pekerja: form.bpjstk_pengurangan_pekerja,
        bpjsks_pengurangan_pekerja: form.bpjsks_pengurangan_pekerja,

        created_date: None,
        created_by: None,
        updated_date: None,
        updated_by: None,
    }
}

pub async fn redirect_to_first_company(
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    match company_service::get_first_company_id(pool.get_ref()).await {
        Ok(id) => {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/companies/detail"))
                .finish())
        }
        Err(AppError::NotFound(_)) => {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/companies/add"))
                .finish())
        }
        Err(e) => Err(e),
    }
}

pub async fn show_detail_company(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = 1;
    let company = company_service::get_company_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    let mut initial_zip_display: Option<String> = None;

    if let Some(zip_id) = company.comp_zip_code.as_ref() {
        match zip_code_service::get_zip_code_by_id(pool.get_ref(), zip_id).await {
            Ok(zip_info) => {
                let postal = zip_info
                    .zip_postal_code
                    .unwrap_or_else(|| "N/A".to_string());
                let city = zip_info.city;
                let district = zip_info.district;

                initial_zip_display = Some(format!("({}) {}, {}", postal, city, district));
            }
            Err(e) => {
                tracing::error!(
                    "Failed to fetch zip code details for ID {}: {:?}",
                    zip_id,
                    e
                );
            }
        }
    }

    let page_title = format!(
        "Detail Company: {}",
        company.comp_name.as_deref().unwrap_or("N/A")
    );
    context.insert("company", &company);
    context.insert("username", &username);

    context.insert("initial_zip_display", &initial_zip_display);

    context.insert("title", &page_title);
    context.insert("header_title", "Detail Perusahaan");

    let rendered = tera
        .render("companies/detail.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_company_form(
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
    context.insert("title", "Add Company");
    context.insert("header_title", "Add New Company");

    let rendered = tera
        .render("companies/add.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_company_action(
    pool: web::Data<PgPool>,
    form: web::Form<CompanyFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let company_data = map_form_to_company(&form);
    let created_by = get_username(pool.get_ref(), &session).await;

    // let new_company =
    //     company_service::create_company(pool.get_ref(), company_data, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies/detail"))
        .finish())
}

pub async fn show_edit_company_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = 1;
    let company = company_service::get_company_by_id(pool.get_ref(), id).await?;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    let mut initial_zip_id: Option<String> = None;
    let mut initial_zip_display: Option<String> = None;

    if let Some(zip_id) = company.comp_zip_code.as_ref() {
        match zip_code_service::get_zip_code_by_id(pool.get_ref(), zip_id).await {
            Ok(zip_info) => {
                let postal = zip_info
                    .zip_postal_code
                    .unwrap_or_else(|| "N/A".to_string());
                let city = zip_info.city;
                let district = zip_info.district;

                initial_zip_display = Some(format!("({}) {}, {}", postal, city, district));
                initial_zip_id = Some(zip_id.clone());
            }
            Err(e) => {
                tracing::error!(
                    "Failed to fetch zip code details for ID {}: {:?}",
                    zip_id,
                    e
                );
                initial_zip_id = Some(zip_id.clone());
                initial_zip_display = None;
            }
        }
    }

    let page_title = format!(
        "Edit Company: {}",
        company.comp_name.as_deref().unwrap_or("N/A")
    );
    context.insert("company", &company);
    context.insert("username", &username);

    context.insert("initial_zip_id", &initial_zip_id);
    context.insert("initial_zip_display", &initial_zip_display);

    context.insert("title", &page_title);
    context.insert("header_title", "Edit Company Info");

    let rendered = tera
        .render("companies/edit.html", &context)
        .map_err(AppError::TeraError)?;
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_company_action(
    pool: web::Data<PgPool>,
    form: web::Form<CompanyFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = 1;
    let company_data = map_form_to_company(&form);
    let updated_by = get_username(pool.get_ref(), &session).await;

    company_service::update_company(pool.get_ref(), id, company_data, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies/detail"))
        .finish())
}

pub async fn delete_company_action(
    pool: web::Data<PgPool>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = 1;
    company_service::delete_company(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/companies"))
        .finish())
}
