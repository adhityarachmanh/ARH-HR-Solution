use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Shift {
    pub shift_id: i32,
    pub shift_code: Option<String>,
    pub shift_name: Option<String>,
    pub shift_in_hour: Option<String>,
    pub shift_out_hour: Option<String>,
    pub break_start: Option<String>,
    pub break_end: Option<String>,
    pub is_show_in_request: Option<bool>,
    pub is_enable_attendance_validation: Option<bool>,
    pub clock_in_min_before: Option<i32>,
    pub clock_out_max_after: Option<i32>,
    pub is_enable_dispensation: Option<bool>,
    pub clock_in_dispensation: Option<i32>,
    pub clock_out_dispensation: Option<i32>,
    pub is_work_shift: Option<bool>,
    pub shift_color_hex: Option<String>,
    pub is_shift_out_hour_overlap_day: Option<bool>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_date: Option<OffsetDateTime>,
    pub created_by: Option<String>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_date: Option<OffsetDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShiftFormData {
    pub shift_code: String,
    pub shift_name: String,
    pub shift_in_hour: String,
    pub shift_out_hour: String,
    pub break_start: String,
    pub break_end: String,
    pub is_show_in_request: bool,
    pub is_enable_attendance_validation: bool,
    pub clock_in_min_before: i32,
    pub clock_out_max_after: i32,
    pub is_enable_dispensation: bool,
    pub clock_in_dispensation: i32,
    pub clock_out_dispensation: i32,
    pub is_work_shift: bool,
    pub shift_color_hex: String,
    pub is_shift_out_hour_overlap_day: bool,
}