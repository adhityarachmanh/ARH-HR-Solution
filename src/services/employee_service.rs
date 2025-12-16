use crate::errors::AppError;
use crate::models::{Employee, EmployeeFormData};
use sqlx::{PgPool, Postgres, query};
use tracing::info;
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

const SELECT_FIELDS: &str = r#"
    "EmployeeId" as employee_id, "BranchId" as branch_id, "OrganizationId" as organization_id, 
    "JobLevelId" as job_level_id, "JobPositionId" as job_position_id, "EmploymentStatusId" as employment_status_id, 
    "ManagerId" as manager_id, "ClassId" as class_id, "UserId" as user_id, "GradeId" as grade_id, 
    "ZipCodeId" as zip_code_id, "PtkpTypeId" as ptkp_type_id, "OvertimeSettingId" as overtime_setting_id, 
    "EmployeeAttendanceLocationId" as employee_attendance_location_id, "EmployeeNumber" as employee_number, 
    "EmployeeNumberBarcode" as employee_number_barcode, "FirstName" as first_name, "LastName" as last_name, 
    "Email" as email, "JobTitle" as job_title, "IsActive" as is_active, 
    "FirstJoinDate" as first_join_date, "JoinDate" as join_date, "EndDate" as end_date, "SignDate" as sign_date, 
    "BirthDate" as birth_date, "PassportExpiredDate" as passport_expired_date, "WorkPhone" as work_phone, 
    "WorkMobile" as work_mobile, "BirthPlace" as birth_place, "CitizenIdAddress" as citizen_id_address, 
    "ResidentialAddress" as residential_address, "IsSameAddress" as is_same_address, "Tags" as tags, 
    "Gender" as gender, "BloodType" as blood_type, "MaritalStatusType" as marital_status_type, 
    "ReligionType" as religion_type, "Nik" as nik, "PassportNo" as passport_no, 
    "LatestEducationLevelType" as latest_education_level_type, "IsMedicalStaff" as is_medical_staff, 
    "BankAccountType" as bank_account_type, "BankAccountName" as bank_account_name, 
    "BankAccountNumber" as bank_account_number, "BankAccountFilePath" as bank_account_file_path, 
    "Npwp" as npwp, "BpjsKesehatan" as bpjs_kesehatan, "BpjsKesehatanDate" as bpjs_kesehatan_date, 
    "BpjsKetenagakerjaan" as bpjs_ketenagakerjaan, "BpjsKetenagakerjaanDate" as bpjs_ketenagakerjaan_date, 
    "AdditionalBpjsKsDependentsCount" as additional_bpjs_ks_dependents_count, "TunjanganKhususYayasan" as tunjangan_khusus_yayasan, 
    "GajiPokok" as gaji_pokok, "TunjanganJabatan" as tunjangan_jabatan, "UangMakanPerHari" as uang_makan_per_hari, 
    "TunjanganPenugasanPenuhWaktu" as tunjangan_penugasan_penuh_waktu, "PotonganBahtera" as potongan_bahtera, 
    "IsAllowedForOvertime" as is_allowed_for_overtime, "HasRightCutiTahunan" as has_right_cuti_tahunan, 
    "EmployeePhotoFilePath" as employee_photo_file_path, 
    "CreatedDate" as created_date, "CreatedBy" as created_by, "UpdatedDate" as updated_date, "UpdatedBy" as updated_by
"#;

fn parse_date_from_string(date_str: Option<&String>) -> Result<Option<OffsetDateTime>, AppError> {
    match date_str {
        Some(s) if !s.is_empty() => {
            let date = Date::parse(s, &time::macros::format_description!("[year]-[month]-[day]"))
                .map_err(|_| AppError::BadRequest(format!("Invalid date format for: {}", s)))?;
            
            let datetime = PrimitiveDateTime::new(date, Time::MIDNIGHT);
            Ok(Some(datetime.assume_utc()))
        },
        _ => Ok(None),
    }
}

pub async fn count_employees(pool: &PgPool) -> Result<i64, AppError> {
    let count = sqlx::query_scalar!(r#"SELECT COUNT("EmployeeId") FROM "Employees""#)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?
        .unwrap_or(0);
    Ok(count)
}

pub async fn get_all_employees(pool: &PgPool) -> Result<Vec<Employee>, AppError> {
    let rows = sqlx::query_as::<_, Employee>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Employees" ORDER BY "EmployeeNumber" ASC"#
    ))
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(rows)
}


pub async fn get_all_employees_paginated(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Employee>, AppError> {
    let rows = sqlx::query_as::<_, Employee>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Employees" ORDER BY "EmployeeNumber" ASC LIMIT $1 OFFSET $2"#
    ))
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(AppError::DatabaseError)?;
    Ok(rows)
}

pub async fn get_employee_by_id(pool: &PgPool, id: i32) -> Result<Employee, AppError> {
    let row = sqlx::query_as::<_, Employee>(&format!(
        r#"SELECT {SELECT_FIELDS} FROM "Employees" WHERE "EmployeeId" = $1"#
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    row.ok_or(AppError::NotFound(format!("Employee with ID {} not found.", id)))
}

pub async fn create_employee(pool: &PgPool, data: &EmployeeFormData, created_by: &str) -> Result<Employee, AppError> {
    let job_title = data.job_position_id.to_string();

    let first_join_date = parse_date_from_string(Some(&data.first_join_date))?;
    let join_date = parse_date_from_string(Some(&data.join_date))?;
    let end_date = parse_date_from_string(data.end_date.as_ref())?;
    let sign_date = parse_date_from_string(data.sign_date.as_ref())?;
    let birth_date = parse_date_from_string(data.birth_date.as_ref())?;
    let bpjs_kesehatan_date = parse_date_from_string(data.bpjs_kesehatan_date.as_ref())?;
    let bpjs_ketenagakerjaan_date = parse_date_from_string(data.bpjs_ketenagakerjaan_date.as_ref())?;
    
    let row = sqlx::query_as::<_, Employee>(&format!(
        r#"
        INSERT INTO "Employees" (
            "EmployeeNumber", "EmployeeNumberBarcode", "FirstName", "LastName", "Email", "JobTitle", 
            "BranchId", "OrganizationId", "JobLevelId", "JobPositionId", "EmploymentStatusId", "ManagerId", "GradeId", "ClassId", 
            "PtkpTypeId", "OvertimeSettingId", "EmployeeAttendanceLocationId", 
            "FirstJoinDate", "JoinDate", "EndDate", "SignDate", "BirthDate", 
            "WorkPhone", "WorkMobile", "BirthPlace", "CitizenIdAddress", "ResidentialAddress", "IsSameAddress", 
            "Gender", "BloodType", "MaritalStatusType", "ReligionType", 
            "Nik", "Npwp", "IsMedicalStaff", 
            "BankAccountType", "BankAccountName", "BankAccountNumber", 
            "BpjsKesehatan", "BpjsKesehatanDate", "BpjsKetenagakerjaan", "BpjsKetenagakerjaanDate", 
            "AdditionalBpjsKsDependentsCount", 
            "GajiPokok", "TunjanganJabatan", "UangMakanPerHari", "TunjanganPenugasanPenuhWaktu", "PotonganBahtera", 
            "IsAllowedForOvertime", "HasRightCutiTahunan", 
            "CreatedBy"
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22,
            $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41, $42,
            $43, $44, $45, $46, $47, $48, $49, $50
        )
        RETURNING {SELECT_FIELDS}
        "#
    ))
    .bind(&data.employee_number)
    .bind(&data.employee_number_barcode)
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(&data.email)
    .bind(job_title)
    .bind(data.branch_id)
    .bind(data.organization_id)
    .bind(data.job_level_id)
    .bind(data.job_position_id)
    .bind(data.employment_status_id)
    .bind(data.manager_id)
    .bind(data.grade_id)
    .bind(data.class_id)
    .bind(data.ptkp_type_id)
    .bind(data.overtime_setting_id)
    .bind(data.employee_attendance_location_id)
    .bind(first_join_date)
    .bind(join_date)
    .bind(end_date)
    .bind(sign_date)
    .bind(birth_date)
    .bind(&data.work_phone)
    .bind(&data.work_mobile)
    .bind(&data.birth_place)
    .bind(&data.citizen_id_address)
    .bind(&data.residential_address)
    .bind(data.is_same_address)
    .bind(&data.gender)
    .bind(&data.blood_type)
    .bind(&data.marital_status_type)
    .bind(&data.religion_type)
    .bind(&data.nik)
    .bind(&data.npwp)
    .bind(data.is_medical_staff)
    .bind(&data.bank_account_type)
    .bind(&data.bank_account_name)
    .bind(&data.bank_account_number)
    .bind(&data.bpjs_kesehatan)
    .bind(bpjs_kesehatan_date)
    .bind(&data.bpjs_ketenagakerjaan)
    .bind(bpjs_ketenagakerjaan_date)
    .bind(data.additional_bpjs_ks_dependents_count)
    .bind(data.gaji_pokok)
    .bind(data.tunjangan_jabatan)
    .bind(data.uang_makan_per_hari)
    .bind(data.tunjangan_penugasan_penuh_waktu)
    .bind(data.potongan_bahtera)
    .bind(data.is_allowed_for_overtime)
    .bind(data.has_right_cuti_tahunan)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    info!("Employee {} {} created by {}", 
        row.first_name.as_deref().unwrap_or("N/A"), 
        row.last_name.as_deref().unwrap_or(""), 
        created_by
    );
    Ok(row)
}

pub async fn update_employee(pool: &PgPool, id: i32, data: &EmployeeFormData, updated_by: &str) -> Result<Employee, AppError> {
    Err(AppError::InternalError("Update functionality not yet implemented.".to_string()))
}

pub async fn delete_employee(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let result = sqlx::query!(r#"DELETE FROM "Employees" WHERE "EmployeeId" = $1"#, id)
        .execute(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Employee with ID {} not found for deletion.", id)));
    }
    info!("Employee ID {} deleted.", id);
    Ok(())
}