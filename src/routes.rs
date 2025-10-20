// src/routes.rs
use actix_web::web;
use crate::handlers::{auth_handler, dashboard_handler, user_handler, role_handler, permission_handler}; 

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(dashboard_handler::show_dashboard))
            
            // Rute Auth
            .route("/login", web::get().to(auth_handler::show_login_form))
            .route("/login", web::post().to(auth_handler::login_action))
            .route("/logout", web::post().to(auth_handler::logout_action))
            
            // Rute User Management
            .route("/users/list", web::get().to(user_handler::list_users))
            .route("/users/add", web::get().to(user_handler::show_add_user_form)) 
            .route("/users/add", web::post().to(user_handler::add_user_action))  
            
            // Rute Role Management (CRUD)
            .route("/roles/list", web::get().to(role_handler::list_roles))
            .route("/roles/add", web::get().to(role_handler::show_add_role_form)) 
            .route("/roles/add", web::post().to(role_handler::add_role_action))  
            .route("/roles/edit/{id}", web::get().to(role_handler::show_edit_role_form))
            .route("/roles/edit/{id}", web::post().to(role_handler::edit_role_action))
            .route("/roles/delete/{id}", web::post().to(role_handler::delete_role_action))

            // Rute Permission Management (Master List & Relasi)
            .route("/permissions/list", web::get().to(permission_handler::list_master_permissions))
            .route("/roles/permissions/{role_id}", web::get().to(permission_handler::manage_permissions_form))
            .route("/roles/permissions/{role_id}", web::post().to(permission_handler::save_permissions_action))
    );
}