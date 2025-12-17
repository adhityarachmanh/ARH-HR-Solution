use crate::handlers::{
    attendance_location_handler, auth_handler, branch_handler, class_handler, company_handler,
    dashboard_handler, employee_handler, employment_status_handler, grade_handler, holiday_handler,
    job_level_handler, job_position_handler, organization_handler, overtime_setting_handler,
    permission_handler, ptkp_type_handler, role_handler, shift_handler, standard_reference_handler,
    timezone_handler, user_handler, zip_code_handler,
};
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(dashboard_handler::show_dashboard))
            // -----------------------------------------------------------
            // Rute Auth
            // -----------------------------------------------------------
            .route("/login", web::get().to(auth_handler::show_login_form))
            .route("/login", web::post().to(auth_handler::login_action))
            .route("/logout", web::post().to(auth_handler::logout_action))
            // -----------------------------------------------------------
            // Rute User Management
            // -----------------------------------------------------------
            .route("/users/list", web::get().to(user_handler::list_users))
            .route(
                "/users/add",
                web::get().to(user_handler::show_add_user_form),
            )
            .route("/users/add", web::post().to(user_handler::add_user_action))
            .route(
                "/users/edit/{id}",
                web::get().to(user_handler::show_edit_user_form),
            )
            .route(
                "/users/edit/{id}",
                web::post().to(user_handler::edit_user_action),
            )
            // -----------------------------------------------------------
            // Rute Role Management (CRUD)
            // -----------------------------------------------------------
            .route("/roles/list", web::get().to(role_handler::list_roles))
            .route(
                "/roles/add",
                web::get().to(role_handler::show_add_role_form),
            )
            .route("/roles/add", web::post().to(role_handler::add_role_action))
            .route(
                "/roles/edit/{id}",
                web::get().to(role_handler::show_edit_role_form),
            )
            .route(
                "/roles/edit/{id}",
                web::post().to(role_handler::edit_role_action),
            )
            .route(
                "/roles/delete/{id}",
                web::post().to(role_handler::delete_role_action),
            )
            // -----------------------------------------------------------
            // Rute Permission Management
            // -----------------------------------------------------------
            .route(
                "/permissions/list",
                web::get().to(permission_handler::list_master_permissions),
            )
            .route(
                "/roles/permissions/{role_id}",
                web::get().to(permission_handler::manage_permissions_form),
            )
            .route(
                "/roles/permissions/{role_id}",
                web::post().to(permission_handler::save_permissions_action),
            )
            // -----------------------------------------------------------
            // Rute Branch Master Data
            // -----------------------------------------------------------
            .route(
                "/branches/list",
                web::get().to(branch_handler::list_branches),
            )
            .route(
                "/branches/add",
                web::get().to(branch_handler::show_add_branch_form),
            )
            .route(
                "/branches/add",
                web::post().to(branch_handler::add_branch_action),
            )
            .route(
                "/branches/edit/{id}",
                web::get().to(branch_handler::show_edit_branch_form),
            )
            .route(
                "/branches/edit/{id}",
                web::post().to(branch_handler::edit_branch_action),
            )
            .route(
                "/branches/delete/{id}",
                web::post().to(branch_handler::delete_branch_action),
            )
            // -----------------------------------------------------------
            // Rute Company Master Data
            // -----------------------------------------------------------
            .route(
                "/companies/detail",
                web::get().to(company_handler::show_detail_company),
            )
            // .route(
            //     "/companies/add",
            //     web::get().to(company_handler::show_add_company_form),
            // )
            // .route(
            //     "/companies/add",
            //     web::post().to(company_handler::add_company_action),
            // )
            .route(
                "/companies/edit",
                web::get().to(company_handler::show_edit_company_form),
            )
            .route(
                "/companies/edit",
                web::post().to(company_handler::edit_company_action),
            )
            // .route(
            //     "/companies/delete/{id}",
            //     web::post().to(company_handler::delete_company_action),
            // )
            // -----------------------------------------------------------
            // Rute Time Zone Master Data
            // -----------------------------------------------------------
            .route(
                "/timezones/list",
                web::get().to(timezone_handler::list_timezones),
            )
            .route(
                "/timezones/add",
                web::get().to(timezone_handler::show_add_timezone_form),
            )
            .route(
                "/timezones/add",
                web::post().to(timezone_handler::add_timezone_action),
            )
            .route(
                "/timezones/edit/{id}",
                web::get().to(timezone_handler::show_edit_timezone_form),
            )
            .route(
                "/timezones/edit/{id}",
                web::post().to(timezone_handler::edit_timezone_action),
            )
            .route(
                "/timezones/delete/{id}",
                web::post().to(timezone_handler::delete_timezone_action),
            )
            // -----------------------------------------------------------
            // Rute Organizations Master Data
            // -----------------------------------------------------------
            .route(
                "/organizations/list",
                web::get().to(organization_handler::list_organizations),
            )
            .route(
                "/organizations/add",
                web::get().to(organization_handler::show_add_organization_form),
            )
            .route(
                "/organizations/add",
                web::post().to(organization_handler::add_organization_action),
            )
            .route(
                "/organizations/edit/{id}",
                web::get().to(organization_handler::show_edit_organization_form),
            )
            .route(
                "/organizations/edit/{id}",
                web::post().to(organization_handler::edit_organization_action),
            )
            .route(
                "/organizations/delete/{id}",
                web::post().to(organization_handler::delete_organization_action),
            )
            // -----------------------------------------------------------
            // Rute Job Position Master Data
            // -----------------------------------------------------------
            .route(
                "/jobpositions/list",
                web::get().to(job_position_handler::list_job_positions),
            )
            .route(
                "/jobpositions/add",
                web::get().to(job_position_handler::show_add_job_position_form),
            )
            .route(
                "/jobpositions/add",
                web::post().to(job_position_handler::add_job_position_action),
            )
            .route(
                "/jobpositions/edit/{id}",
                web::get().to(job_position_handler::show_edit_job_position_form),
            )
            .route(
                "/jobpositions/edit/{id}",
                web::post().to(job_position_handler::edit_job_position_action),
            )
            .route(
                "/jobpositions/delete/{id}",
                web::post().to(job_position_handler::delete_job_position_action),
            )
            // -----------------------------------------------------------
            // Rute Holiday Master Data
            // -----------------------------------------------------------
            .route(
                "/holidays/list",
                web::get().to(holiday_handler::list_holidays),
            )
            .route(
                "/holidays/add",
                web::get().to(holiday_handler::show_add_holiday_form),
            )
            .route(
                "/holidays/add",
                web::post().to(holiday_handler::add_holiday_action),
            )
            .route(
                "/holidays/edit/{id}",
                web::get().to(holiday_handler::show_edit_holiday_form),
            )
            .route(
                "/holidays/edit/{id}",
                web::post().to(holiday_handler::edit_holiday_action),
            )
            .route(
                "/holidays/delete/{id}",
                web::post().to(holiday_handler::delete_holiday_action),
            )
            .route(
                "/holidays/sync/{year}",
                web::get().to(holiday_handler::sync_holidays_action),
            )
            // -----------------------------------------------------------
            // Rute Job Level Master Data
            // -----------------------------------------------------------
            .route(
                "/joblevels/list",
                web::get().to(job_level_handler::list_job_levels),
            )
            .route(
                "/joblevels/add",
                web::get().to(job_level_handler::show_add_job_level_form),
            )
            .route(
                "/joblevels/add",
                web::post().to(job_level_handler::add_job_level_action),
            )
            .route(
                "/joblevels/edit/{id}",
                web::get().to(job_level_handler::show_edit_job_level_form),
            )
            .route(
                "/joblevels/edit/{id}",
                web::post().to(job_level_handler::edit_job_level_action),
            )
            .route(
                "/joblevels/delete/{id}",
                web::post().to(job_level_handler::delete_job_level_action),
            )
            // -----------------------------------------------------------
            // Rute Attendance Locations Master Data
            // -----------------------------------------------------------
            .route(
                "/attendancelocations/list",
                web::get().to(attendance_location_handler::list_attendance_locations),
            )
            .route(
                "/attendancelocations/add",
                web::get().to(attendance_location_handler::show_add_location_form),
            )
            .route(
                "/attendancelocations/add",
                web::post().to(attendance_location_handler::add_location_action),
            )
            .route(
                "/attendancelocations/edit/{id}",
                web::get().to(attendance_location_handler::show_edit_location_form),
            )
            .route(
                "/attendancelocations/edit/{id}",
                web::post().to(attendance_location_handler::edit_location_action),
            )
            .route(
                "/attendancelocations/delete/{id}",
                web::post().to(attendance_location_handler::delete_location_action),
            )
            // -----------------------------------------------------------
            // Rute Employees Data
            // -----------------------------------------------------------
            // .route(
            //     "/employees/list",
            //     web::get().to(employee_handler::list_employees),
            // )
            // .route(
            //     "/employees/add",
            //     web::get().to(employee_handler::show_add_employee_form),
            // )
            // .route(
            //     "/employees/add",
            //     web::post().to(employee_handler::add_employee_action),
            // ),
            // -----------------------------------------------------------
            // Rute Employment Status Master Data
            // -----------------------------------------------------------
            .route(
                "/employmentstatuses/list",
                web::get().to(employment_status_handler::list_employment_statuses),
            )
            .route(
                "/employmentstatuses/add",
                web::get().to(employment_status_handler::show_add_employment_status_form),
            )
            .route(
                "/employmentstatuses/add",
                web::post().to(employment_status_handler::add_employment_status_action),
            )
            .route(
                "/employmentstatuses/edit/{id}",
                web::get().to(employment_status_handler::show_edit_employment_status_form),
            )
            .route(
                "/employmentstatuses/edit/{id}",
                web::post().to(employment_status_handler::edit_employment_status_action),
            )
            .route(
                "/employmentstatuses/delete/{id}",
                web::post().to(employment_status_handler::delete_employment_status_action),
            )
            // -----------------------------------------------------------
            // Rute Zip Code Master Data
            // -----------------------------------------------------------
            .route(
                "/zipcodes/list",
                web::get().to(zip_code_handler::list_zip_codes),
            )
            .route(
                "/zipcodes/add",
                web::get().to(zip_code_handler::show_add_zip_code_form),
            )
            .route(
                "/zipcodes/add",
                web::post().to(zip_code_handler::add_zip_code_action),
            )
            .route(
                "/zipcodes/edit/{id}",
                web::get().to(zip_code_handler::show_edit_zip_code_form),
            )
            .route(
                "/zipcodes/edit/{id}",
                web::post().to(zip_code_handler::edit_zip_code_action),
            )
            .route(
                "/zipcodes/delete/{id}",
                web::post().to(zip_code_handler::delete_zip_code_action),
            )
            .route(
                "/api/zipcodes/search",
                web::get().to(zip_code_handler::search_zip_codes_json),
            )
            // -----------------------------------------------------------
            // Rute Classes Master Data
            // -----------------------------------------------------------
            .route("/classes/list", web::get().to(class_handler::list_classes))
            .route(
                "/classes/add",
                web::get().to(class_handler::show_add_class_form),
            )
            .route(
                "/classes/add",
                web::post().to(class_handler::add_class_action),
            )
            .route(
                "/classes/edit/{id}",
                web::get().to(class_handler::show_edit_class_form),
            )
            .route(
                "/classes/edit/{id}",
                web::post().to(class_handler::edit_class_action),
            )
            .route(
                "/classes/delete/{id}",
                web::post().to(class_handler::delete_class_action),
            )
            // -----------------------------------------------------------
            // Rute grades Master Data
            // -----------------------------------------------------------
            .route("/grades/list", web::get().to(grade_handler::list_grades))
            .route(
                "/grades/add",
                web::get().to(grade_handler::show_add_grade_form),
            )
            .route(
                "/grades/add",
                web::post().to(grade_handler::add_grade_action),
            )
            .route(
                "/grades/edit/{id}",
                web::get().to(grade_handler::show_edit_grade_form),
            )
            .route(
                "/grades/edit/{id}",
                web::post().to(grade_handler::edit_grade_action),
            )
            .route(
                "/grades/delete/{id}",
                web::post().to(grade_handler::delete_grade_action),
            )
            // -----------------------------------------------------------
            // Rute ptkp_types Master Data
            // -----------------------------------------------------------
            .route(
                "/ptkp_types/list",
                web::get().to(ptkp_type_handler::list_ptkp_types),
            )
            .route(
                "/ptkp_types/add",
                web::get().to(ptkp_type_handler::show_add_ptkp_type_form),
            )
            .route(
                "/ptkp_types/add",
                web::post().to(ptkp_type_handler::add_ptkp_type_action),
            )
            .route(
                "/ptkp_types/edit/{id}",
                web::get().to(ptkp_type_handler::show_edit_ptkp_type_form),
            )
            .route(
                "/ptkp_types/edit/{id}",
                web::post().to(ptkp_type_handler::edit_ptkp_type_action),
            )
            .route(
                "/ptkp_types/delete/{id}",
                web::post().to(ptkp_type_handler::delete_ptkp_type_action),
            )
            // -----------------------------------------------------------
            // Rute overtime_settings Master Data
            // -----------------------------------------------------------
            .route(
                "/overtime_settings/list",
                web::get().to(overtime_setting_handler::list_overtime_settings),
            )
            .route(
                "/overtime_settings/add",
                web::get().to(overtime_setting_handler::show_add_overtime_setting_form),
            )
            .route(
                "/overtime_settings/add",
                web::post().to(overtime_setting_handler::add_overtime_setting_action),
            )
            .route(
                "/overtime_settings/edit/{id}",
                web::get().to(overtime_setting_handler::show_edit_overtime_setting_form),
            )
            .route(
                "/overtime_settings/edit/{id}",
                web::post().to(overtime_setting_handler::edit_overtime_setting_action),
            )
            .route(
                "/overtime_settings/delete/{id}",
                web::post().to(overtime_setting_handler::delete_overtime_setting_action),
            )
            // -----------------------------------------------------------
            // Rute shifts Master Data
            // -----------------------------------------------------------
            .route("/shifts/list", web::get().to(shift_handler::list_shifts))
            .route(
                "/shifts/add",
                web::get().to(shift_handler::show_add_shift_form),
            )
            .route(
                "/shifts/add",
                web::post().to(shift_handler::add_shift_action),
            )
            .route(
                "/shifts/edit/{id}",
                web::get().to(shift_handler::show_edit_shift_form),
            )
            .route(
                "/shifts/edit/{id}",
                web::post().to(shift_handler::edit_shift_action),
            )
            .route(
                "/shifts/delete/{id}",
                web::post().to(shift_handler::delete_shift_action),
            )
            // -----------------------------------------------------------
            // Rute Standard References Master Data
            // -----------------------------------------------------------
            .route(
                "/standard_references/list",
                web::get().to(standard_reference_handler::list_references),
            )
            .route(
                "/standard_references/add",
                web::get().to(standard_reference_handler::show_add_reference_form),
            )
            .route(
                "/standard_references/add",
                web::post().to(standard_reference_handler::create_reference_action),
            )
            .route(
                "/standard_references/edit/{sr_id}",
                web::get().to(standard_reference_handler::show_edit_reference_form),
            )
            .route(
                "/standard_references/edit/{sr_id}",
                web::post().to(standard_reference_handler::edit_reference_action),
            )
            .route(
                "/standard_references/items/{sr_id}",
                web::get().to(standard_reference_handler::list_items_by_reference),
            )
            .route(
                "/standard_references/items/add/{sr_id}",
                web::get().to(standard_reference_handler::show_add_item_form),
            )
            .route(
                "/standard_references/items/add/{sr_id}",
                web::post().to(standard_reference_handler::add_item_action),
            )
            .route(
                "/standard_references/items/edit/{sr_id}/{item_id}",
                web::get().to(standard_reference_handler::show_edit_item_form),
            )
            .route(
                "/standard_references/items/edit/{sr_id}/{item_id}",
                web::post().to(standard_reference_handler::edit_item_action),
            )
            .route(
                "/standard_references/items/delete/{sr_id}/{item_id}",
                web::post().to(standard_reference_handler::delete_item_action),
            )
            // -----------------------------------------------------------
            // Rute Employees Data
            // -----------------------------------------------------------
            .route(
                "/employees/list",
                web::get().to(employee_handler::list_employees),
            )
            .route(
                "/employees/add",
                web::get().to(employee_handler::show_add_employee_form),
            )
            .route(
                "/employees/add",
                web::post().to(employee_handler::add_employee_action),
            )
            .route(
                "/employees/edit/{id}",
                web::get().to(employee_handler::show_edit_employee_form),
            )
            .route(
                "/employees/edit/{id}",
                web::post().to(employee_handler::edit_employee_action),
            )
            .route(
                "/employees/delete/{id}",
                web::post().to(employee_handler::delete_employee_action),
            )
            .route(
                "/api/employees/search",
                web::get().to(employee_handler::search_employees_json),
            ),
    );
}
