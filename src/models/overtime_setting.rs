use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OvertimeSetting {
    pub overtime_setting_id: i32,
    pub is_rounding: Option<bool>,
    pub overtime_name: Option<String>,
    pub is_default_compensation: Option<bool>,
    pub compensation_divide_number: Option<f64>,
    pub override_rupiah_per_hour: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OvertimeSettingFormData {
    pub is_rounding: bool,
    pub overtime_name: String,
    pub is_default_compensation: bool,
    pub compensation_divide_number: Option<f64>,
    pub override_rupiah_per_hour: Option<f64>,
}
