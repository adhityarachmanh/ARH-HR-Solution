use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Employee {
    pub employee_id: i32,
    pub branch_id: Option<i32>,
    pub organization_id: Option<i32>,
    pub job_level_id: Option<i32>,
    pub job_position_id: Option<i32>,
    pub employment_status_id: Option<i32>,
    pub manager_id: Option<i32>,
    pub class_id: Option<i32>,
    pub user_id: Option<i64>, 
    pub grade_id: Option<i32>,
    pub zip_code_id: Option<String>,
    pub ptkp_type_id: Option<i32>,
    pub overtime_setting_id: Option<i32>,
    pub employee_attendance_location_id: Option<i32>,
    
    pub employee_number: String,
    pub employee_number_barcode: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub job_title: Option<String>,
    pub is_active: Option<bool>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub first_join_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub join_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub end_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub sign_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub birth_date: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub passport_expired_date: Option<OffsetDateTime>,

    pub work_phone: Option<String>,
    pub work_mobile: Option<String>,
    pub birth_place: Option<String>,
    pub citizen_id_address: Option<String>,
    pub residential_address: Option<String>,
    pub is_same_address: Option<bool>,
    pub tags: Option<String>,
    pub gender: Option<String>,
    pub blood_type: Option<String>,
    pub marital_status_type: Option<String>,
    pub religion_type: Option<String>,
    pub nik: Option<String>,
    pub passport_no: Option<String>,
    pub latest_education_level_type: Option<String>,
    pub is_medical_staff: bool,
    pub bank_account_type: Option<String>,
    pub bank_account_name: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_account_file_path: Option<String>,
    pub npwp: Option<String>,
    pub bpjs_kesehatan: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub bpjs_kesehatan_date: Option<OffsetDateTime>,
    pub bpjs_ketenagakerjaan: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub bpjs_ketenagakerjaan_date: Option<OffsetDateTime>,
    pub additional_bpjs_ks_dependents_count: i32,
    pub tunjangan_khusus_yayasan: Option<f64>,
    pub gaji_pokok: Option<f64>,
    pub tunjangan_jabatan: Option<f64>,
    pub uang_makan_per_hari: Option<f64>,
    pub tunjangan_penugasan_penuh_waktu: Option<f64>,
    pub potongan_bahtera: Option<f64>,
    pub is_allowed_for_overtime: Option<bool>,
    pub has_right_cuti_tahunan: Option<bool>,
    pub employee_photo_file_path: Option<String>,

    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeFormData {
    pub username: String,
    pub password: String,
    pub is_active: bool,
    
    pub employee_number: String,
    pub employee_number_barcode: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub work_mobile: String,
    pub work_phone: Option<String>,
    
    pub birth_place: String,
    pub birth_date: String,
    pub gender: String,
    pub marital_status_type: String,
    pub blood_type: String,
    pub religion_type: String,
    
    pub nik: Option<String>,
    pub passport_no: Option<String>, 
    pub passport_expired_date: Option<String>,
    pub is_medical_staff: bool,

    pub organization_id: i32,
    pub job_position_id: i32,
    pub job_level_id: i32,
    pub branch_id: i32,
    pub employment_status_id: i32,
    pub manager_id: i32, 
    pub grade_id: Option<i32>,
    pub class_id: Option<i32>, 
    pub ptkp_type_id: Option<i32>, 
    pub overtime_setting_id: Option<i32>, 
    pub employee_attendance_location_id: i32,
    
    pub first_join_date: String, 
    pub join_date: String,
    pub end_date: Option<String>,
    pub sign_date: Option<String>, 
    
    pub zip_code_id: Option<String>, 
    pub citizen_id_address: Option<String>,
    pub residential_address: Option<String>,
    pub is_same_address: Option<bool>,

    pub gaji_pokok: f64,
    pub tunjangan_khusus_yayasan: f64,
    pub tunjangan_jabatan: f64,
    pub uang_makan_per_hari: f64,
    pub tunjangan_penugasan_penuh_waktu: f64,
    pub potongan_bahtera: f64,
    
    pub is_allowed_for_overtime: Option<bool>,
    pub has_right_cuti_tahunan: bool,
    
    pub npwp: Option<String>,
    pub bank_account_type: Option<String>,
    pub bank_account_name: Option<String>,
    pub bank_account_number: Option<String>,
    pub bpjs_kesehatan: Option<String>,
    pub bpjs_kesehatan_date: Option<String>,
    pub bpjs_ketenagakerjaan: Option<String>,
    pub bpjs_ketenagakerjaan_date: Option<String>,
    pub additional_bpjs_ks_dependents_count: i32,
}