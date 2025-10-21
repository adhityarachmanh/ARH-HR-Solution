// src/routes.rs
use actix_web::web;
use crate::handlers::{
    auth_handler, 
    dashboard_handler, 
    user_handler, 
    role_handler, 
    permission_handler, 
    branch_handler, 
    organization_handler,
    company_handler,
    timezone_handler
}; 

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
            .route("/users/add", web::get().to(user_handler::show_add_user_form))           
            .route("/users/add", web::post().to(user_handler::add_user_action))             
            
            // -----------------------------------------------------------
            // Rute Role Management (CRUD)
            // -----------------------------------------------------------
            .route("/roles/list", web::get().to(role_handler::list_roles))                  
            .route("/roles/add", web::get().to(role_handler::show_add_role_form))          
            .route("/roles/add", web::post().to(role_handler::add_role_action))             
            .route("/roles/edit/{id}", web::get().to(role_handler::show_edit_role_form))    
            .route("/roles/edit/{id}", web::post().to(role_handler::edit_role_action))      
            .route("/roles/delete/{id}", web::post().to(role_handler::delete_role_action))  

            // -----------------------------------------------------------
            // Rute Permission Management
            // -----------------------------------------------------------
            .route("/permissions/list", web::get().to(permission_handler::list_master_permissions))  
            .route("/roles/permissions/{role_id}", web::get().to(permission_handler::manage_permissions_form)) 
            .route("/roles/permissions/{role_id}", web::post().to(permission_handler::save_permissions_action)) 

            // -----------------------------------------------------------
            // Rute Branch Master Data
            // -----------------------------------------------------------
            .route("/branches/list", web::get().to(branch_handler::list_branches))
            .route("/branches/add", web::get().to(branch_handler::show_add_branch_form))
            .route("/branches/add", web::post().to(branch_handler::add_branch_action))
            .route("/branches/edit/{id}", web::get().to(branch_handler::show_edit_branch_form))
            .route("/branches/edit/{id}", web::post().to(branch_handler::edit_branch_action))
            .route("/branches/delete/{id}", web::post().to(branch_handler::delete_branch_action))
            // -----------------------------------------------------------
            // Rute Company Master Data
            // -----------------------------------------------------------
            .route("/companies/list", web::get().to(company_handler::list_companies))
            .route("/companies/add", web::get().to(company_handler::show_add_company_form))
            .route("/companies/add", web::post().to(company_handler::add_company_action))
            .route("/companies/edit/{id}", web::get().to(company_handler::show_edit_company_form))
            .route("/companies/edit/{id}", web::post().to(company_handler::edit_company_action))
            .route("/companies/delete/{id}", web::post().to(company_handler::delete_company_action))
            // -----------------------------------------------------------
            // Rute Time Zone Master Data
            // -----------------------------------------------------------
            .route("/timezones/list", web::get().to(timezone_handler::list_timezones))
            .route("/timezones/add", web::get().to(timezone_handler::show_add_timezone_form))
            .route("/timezones/add", web::post().to(timezone_handler::add_timezone_action))
            .route("/timezones/edit/{id}", web::get().to(timezone_handler::show_edit_timezone_form))
            .route("/timezones/edit/{id}", web::post().to(timezone_handler::edit_timezone_action))
            .route("/timezones/delete/{id}", web::post().to(timezone_handler::delete_timezone_action))
            // -----------------------------------------------------------
            // Rute Organizations Master Data
            // -----------------------------------------------------------
            .route("/organizations/list", web::get().to(organization_handler::list_organizations))
            .route("/organizations/add", web::get().to(organization_handler::show_add_organization_form))
            .route("/organizations/add", web::post().to(organization_handler::add_organization_action))
            .route("/organizations/edit/{id}", web::get().to(organization_handler::show_edit_organization_form))
            .route("/organizations/edit/{id}", web::post().to(organization_handler::edit_organization_action))
            .route("/organizations/delete/{id}", web::post().to(organization_handler::delete_organization_action))
    );
}