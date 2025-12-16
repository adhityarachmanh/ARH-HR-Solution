use actix_session::Session;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tera::Tera;
use serde::Deserialize;

use crate::errors::AppError;
use crate::handlers::user_handler::get_username;
use crate::models::{PaginationParams, EmployeeFormData};
use crate::services::{
    employee_service, branch_service, organization_service, job_level_service, 
    job_position_service, employment_status_service, grade_service, 
    ptkp_type_service, overtime_setting_service, class_service, user_service,
    standard_reference_service, attendance_location_service,
};

async fn get_employee_id_from_path(path: web::Path<i32>) -> Result<i32, AppError> {
    let id = path.into_inner();
    if id <= 0 {
        return Err(AppError::BadRequest("Employee ID must be greater than 0.".into()));
    }
    Ok(id)
}

async fn fetch_master_data(pool: &PgPool) -> Result<tera::Context, AppError> {
    let mut context = tera::Context::new();
    
    context.insert("branches", &branch_service::get_all_branches(pool).await?);
    context.insert("organizations", &organization_service::get_all_organizations(pool).await?);
    context.insert("job_levels", &job_level_service::get_all_job_levels(pool).await?);
    context.insert("job_positions", &job_position_service::get_all_job_positions(pool).await?);
    context.insert("employment_statuses", &employment_status_service::get_all_employment_statuses(pool).await?);
    context.insert("grades", &grade_service::get_all_grades(pool).await?);
    context.insert("ptkp_types", &ptkp_type_service::get_all_ptkp_types(pool).await?);
    context.insert("overtime_settings", &overtime_setting_service::get_all_overtime_settings(pool).await?);
    
    context.insert("attendance_locations", &attendance_location_service::get_all_locations(pool).await?);
    
    context.insert("classes", &class_service::get_all_classes(pool).await?);
    context.insert("managers", &user_service::get_all_users(pool).await?);

    context.insert("blood_types", &standard_reference_service::get_items_by_reference_id(pool, "BloodType").await?);
    context.insert("genders", &standard_reference_service::get_items_by_reference_id(pool, "GenderType").await?);
    context.insert("marital_statuses", &standard_reference_service::get_items_by_reference_id(pool, "MaritalStatusType").await?);
    context.insert("religion_types", &standard_reference_service::get_items_by_reference_id(pool, "ReligionType").await?);
    context.insert("education_levels", &standard_reference_service::get_items_by_reference_id(pool, "EducationLevelType").await?);
    context.insert("bank_account_types", &standard_reference_service::get_items_by_reference_id(pool, "BankAccountType").await?);

    Ok(context)
}

pub async fn list_employees(
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

    let employees = employee_service::get_all_employees_paginated(pool.get_ref(), limit, offset).await?;
    let total_records = employee_service::count_employees(pool.get_ref()).await?;
    let total_pages = (total_records + limit - 1) / limit;

    let username = get_username(pool.get_ref(), &session).await;
    let mut context = tera::Context::new();

    context.insert("employees", &employees);
    context.insert("username", &username);
    
    context.insert("total_records", &total_records);
    context.insert("total_pages", &total_pages);
    context.insert("current_page", &page);
    context.insert("limit", &limit);
    
    context.insert("title", "Employee Master");
    context.insert("header_title", "Employee List");

    let rendered = tera
        .render("employees/list.html", &context)
        .map_err(AppError::TeraError)?;
    
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn show_add_employee_form(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    
    let mut context = fetch_master_data(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;
    
    context.insert("username", &username);
    context.insert("title", "Add Employee");
    context.insert("header_title", "Tambah Data Karyawan Baru");

    let rendered = tera
        .render("employees/add.html", &context)
        .map_err(AppError::TeraError)?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn add_employee_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<EmployeeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let created_by = get_username(pool.get_ref(), &session).await;
    
    let new_employee = employee_service::create_employee(pool.get_ref(), &form, &created_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", format!("/employees/edit/{}", new_employee.employee_id)))
        .finish())
}

pub async fn show_edit_employee_form(
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
    
    let id = get_employee_id_from_path(path).await?;
    let employee = employee_service::get_employee_by_id(pool.get_ref(), id).await?;
    
    let mut context = fetch_master_data(pool.get_ref()).await?;
    let username = get_username(pool.get_ref(), &session).await;
    
    context.insert("employee", &employee);
    context.insert("username", &username);
    context.insert("title", &format!("Edit Employee: {}", employee.employee_number));
    context.insert("header_title", "Edit Data Karyawan");

    let rendered = tera
        .render("employees/edit.html", &context)
        .map_err(AppError::TeraError)?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn edit_employee_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    path: web::Path<i32>,
    form: web::Form<EmployeeFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }

    let id = get_employee_id_from_path(path).await?;
    let updated_by = get_username(pool.get_ref(), &session).await;

    employee_service::update_employee(pool.get_ref(), id, &form, &updated_by).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", format!("/employees/edit/{}", id)))
        .finish())
}

// =========================================================================
// DELETE
// =========================================================================

pub async fn delete_employee_action(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    if session.get::<i64>("user_id").unwrap_or(None).is_none() {
        return Ok(HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish());
    }
    
    let id = get_employee_id_from_path(path).await?;
    
    employee_service::delete_employee(pool.get_ref(), id).await?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/employees/list?page=1&limit=10"))
        .finish())
}