// src/services/company_service.rs
use sqlx::PgPool;
use crate::models::Company;
use crate::errors::AppError;
use tracing::info;

const CREATE_QUERY_SQL: &str = r#"
    INSERT INTO "Company" ("CompName", "CompAddress", "CompZipCode", "CompPhoneNumber", "CompMobileNumber", "CompEmail", "CompWebsite", "CompRetirementAge", "PersentasePendapatanBPJSTKPemberiKerja", "PersentasePendapatanBPJSKSPemberiKerja", "PersentasePenguranganBPJSTKPemberiKerja", "PersentasePenguranganBPJSKSPemberiKerja", "PersentasePenguranganBPJSTKPekerja", "PersentasePenguranganBPJSKSPekerja", "CompanyPrevMonthDayPayroll", "CompanyCurrentMonthDayPayroll") 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16) 
    RETURNING "CompId" as comp_id, "CompName" as comp_name, "CompAddress" as comp_address, "CompZipCode" as comp_zip_code, "CompPhoneNumber" as comp_phone_number, "CompMobileNumber" as comp_mobile_number, "CompEmail" as comp_email, "CompWebsite" as comp_website, "CompRetirementAge" as comp_retirement_age, "PersentasePendapatanBPJSTKPemberiKerja" as persentase_pendapatan_bpjstk_pemberi_kerja, "PersentasePendapatanBPJSKSPemberiKerja" as persentase_pendapatan_bpjsks_pemberi_kerja, "PersentasePenguranganBPJSTKPemberiKerja" as persentase_pengurangan_bpjstk_pemberi_kerja, "PersentasePenguranganBPJSTKPemberiKerja" as persentase_pengurangan_bpjsks_pemberi_kerja, "PersentasePenguranganBPJSTKPekerja" as persentase_pengurangan_bpjstk_pekerja, "PersentasePenguranganBPJSKSPekerja" as persentase_pengurangan_bpjsks_pekerja, "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll
"#;

const UPDATE_QUERY_SQL: &str = r#"
    UPDATE "Company" 
    SET "CompName" = $1, "CompAddress" = $2, "CompZipCode" = $3, "CompPhoneNumber" = $4, "CompMobileNumber" = $5, 
        "CompEmail" = $6, "CompWebsite" = $7, "CompRetirementAge" = $8, 
        "PersentasePendapatanBPJSTKPemberiKerja" = $9, "PersentasePendapatanBPJSKSPemberiKerja" = $10, 
        "PersentasePenguranganBPJSTKPemberiKerja" = $11, "PersentasePenguranganBPJSKSPemberiKerja" = $12, 
        "PersentasePenguranganBPJSTKPekerja" = $13, "PersentasePenguranganBPJSKSPekerja" = $14, 
        "CompanyPrevMonthDayPayroll" = $15, "CompanyCurrentMonthDayPayroll" = $16 
    WHERE "CompId" = $17
    RETURNING "CompId" as comp_id, "CompName" as comp_name, "CompAddress" as comp_address, "CompZipCode" as comp_zip_code, "CompPhoneNumber" as comp_phone_number, "CompMobileNumber" as comp_mobile_number, "CompEmail" as comp_email, "CompWebsite" as comp_website, "CompRetirementAge" as comp_retirement_age, "PersentasePendapatanBPJSTKPemberiKerja" as persentase_pendapatan_bpjstk_pemberi_kerja, "PersentasePendapatanBPJSKSPemberiKerja" as persentase_pendapatan_bpjsks_pemberi_kerja, "PersentasePenguranganBPJSTKPemberiKerja" as persentase_pengurangan_bpjstk_pemberi_kerja, "PersentasePenguranganBPJSKSPemberiKerja" as persentase_pengurangan_bpjsks_pemberi_kerja, "PersentasePenguranganBPJSTKPekerja" as persentase_pengurangan_bpjstk_pekerja, "PersentasePenguranganBPJSKSPekerja" as persentase_pengurangan_bpjsks_pekerja, "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll
"#;

pub async fn get_all_companies(pool: &PgPool) -> Result<Vec<Company>, AppError> {
    let companies = sqlx::query_as!(
        Company,
        r#"SELECT "CompId" as comp_id, "CompName" as comp_name, "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll FROM "Company" ORDER BY "CompName" ASC"#
    )
    .fetch_all(pool).await.map_err(AppError::DatabaseError)?;
    Ok(companies)
}

pub async fn get_company_by_id(pool: &PgPool, comp_id: i32) -> Result<Company, AppError> {
    let company = sqlx::query_as!(
        Company,
        r#"SELECT "CompId" as comp_id, "CompName" as comp_name, "CompanyPrevMonthDayPayroll" as company_prev_month_day_payroll, "CompanyCurrentMonthDayPayroll" as company_current_month_day_payroll FROM "Company" WHERE "CompId" = $1"#,
        comp_id
    ).fetch_one(pool).await.map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::InternalError("Company not found".to_string()),
        _ => AppError::DatabaseError(e),
    })?;
    Ok(company)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_company(
    pool: &PgPool, 
    name: &str, 
    address: Option<&str>, 
    zip_code: Option<&str>, 
    phone: Option<&str>, 
    mobile: Option<&str>, 
    email: Option<&str>, 
    website: Option<&str>, 
    retirement_age: Option<i32>,
    bpjstk_pendapatan_pk: Option<f64>,
    bpjsks_pendapatan_pk: Option<f64>,
    bpjstk_pengurangan_pk: Option<f64>,
    bpjsks_pengurangan_pk: Option<f64>,
    bpjstk_pengurangan_pekerja: Option<f64>,
    bpjsks_pengurangan_pekerja: Option<f64>,
    prev_day_payroll: i32, 
    current_day_payroll: i32
) -> Result<Company, AppError> {
    
    let new_company = sqlx::query_as::<_, Company>(CREATE_QUERY_SQL)
        .bind(name) 
        .bind(address) 
        .bind(zip_code) 
        .bind(phone) 
        .bind(mobile) 
        .bind(email) 
        .bind(website) 
        .bind(retirement_age) 
        .bind(bpjstk_pendapatan_pk) 
        .bind(bpjsks_pendapatan_pk) 
        .bind(bpjstk_pengurangan_pk) 
        .bind(bpjsks_pengurangan_pk) 
        .bind(bpjstk_pengurangan_pekerja) 
        .bind(bpjsks_pengurangan_pekerja) 
        .bind(prev_day_payroll) 
        .bind(current_day_payroll) 
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?;
    
    info!("Company '{}' created with ID: {}", new_company.comp_name.as_deref().unwrap_or("N/A"), new_company.comp_id);
    Ok(new_company)
}

#[allow(clippy::too_many_arguments)]
pub async fn update_company(
    pool: &PgPool, 
    comp_id: i32, 
    name: &str, 
    address: Option<&str>, 
    zip_code: Option<&str>, 
    phone: Option<&str>, 
    mobile: Option<&str>, 
    email: Option<&str>, 
    website: Option<&str>, 
    retirement_age: Option<i32>,
    bpjstk_pendapatan_pk: Option<f64>,
    bpjsks_pendapatan_pk: Option<f64>,
    bpjstk_pengurangan_pk: Option<f64>,
    bpjsks_pengurangan_pk: Option<f64>,
    bpjstk_pengurangan_pekerja: Option<f64>,
    bpjsks_pengurangan_pekerja: Option<f64>,
    prev_day_payroll: i32, 
    current_day_payroll: i32
) -> Result<Company, AppError> {
    
    let updated_company = sqlx::query_as::<_, Company>(UPDATE_QUERY_SQL)
        .bind(name)
        .bind(address)
        .bind(zip_code)
        .bind(phone)
        .bind(mobile)
        .bind(email)
        .bind(website)
        .bind(retirement_age)
        .bind(bpjstk_pendapatan_pk)
        .bind(bpjsks_pendapatan_pk)
        .bind(bpjstk_pengurangan_pk)
        .bind(bpjsks_pengurangan_pk)
        .bind(bpjstk_pengurangan_pekerja)
        .bind(bpjsks_pengurangan_pekerja)
        .bind(prev_day_payroll)
        .bind(current_day_payroll)
        .bind(comp_id)
        .fetch_one(pool).await.map_err(AppError::DatabaseError)?; 

    info!("Company ID {} updated to '{}'", updated_company.comp_id, updated_company.comp_name.as_deref().unwrap_or("N/A"));
    Ok(updated_company)
}

pub async fn delete_company(pool: &PgPool, comp_id: i32) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM \"Company\" WHERE \"CompId\" = $1", comp_id)
        .execute(pool).await.map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::InternalError("Company not found for deletion".to_string()));
    }
    info!("Company ID {} deleted.", comp_id);
    Ok(())
}