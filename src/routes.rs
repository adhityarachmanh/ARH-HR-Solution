// src/routes.rs
use actix_web::web;
// PERBAIKAN IMPORT: Tambahkan handlers yang baru dipisah
use crate::handlers::{
    auth_handler, 
    dashboard_handler, 
    user_handler, 
    role_handler, 
    permission_handler, 
    branch_handler, 
    organization_handler // Diperlukan untuk Master Data
}; 

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            // Rute Utama
            .route("/", web::get().to(dashboard_handler::show_dashboard))
            
            // -----------------------------------------------------------
            // Rute Auth
            // -----------------------------------------------------------
            .route("/login", web::get().to(auth_handler::show_login_form))      // GET: Menampilkan form login
            .route("/login", web::post().to(auth_handler::login_action))        // POST: Memproses aksi login
            .route("/logout", web::post().to(auth_handler::logout_action))      // POST: Memproses aksi logout
            
            // -----------------------------------------------------------
            // Rute User Management
            // -----------------------------------------------------------
            .route("/users/list", web::get().to(user_handler::list_users))                  // GET: Menampilkan daftar pengguna
            .route("/users/add", web::get().to(user_handler::show_add_user_form))           // GET: Menampilkan form tambah user
            .route("/users/add", web::post().to(user_handler::add_user_action))             // POST: Memproses tambah user
            
            // -----------------------------------------------------------
            // Rute Role Management (CRUD)
            // -----------------------------------------------------------
            .route("/roles/list", web::get().to(role_handler::list_roles))                  // GET: Menampilkan daftar peran
            .route("/roles/add", web::get().to(role_handler::show_add_role_form))           // GET: Menampilkan form tambah peran
            .route("/roles/add", web::post().to(role_handler::add_role_action))             // POST: Memproses tambah peran
            .route("/roles/edit/{id}", web::get().to(role_handler::show_edit_role_form))    // GET: Menampilkan form edit peran
            .route("/roles/edit/{id}", web::post().to(role_handler::edit_role_action))      // POST: Memproses edit peran
            .route("/roles/delete/{id}", web::post().to(role_handler::delete_role_action))  // POST: Memproses hapus peran

            // -----------------------------------------------------------
            // Rute Permission Management
            // -----------------------------------------------------------
            .route("/permissions/list", web::get().to(permission_handler::list_master_permissions))  // GET: Menampilkan daftar master permission
            .route("/roles/permissions/{role_id}", web::get().to(permission_handler::manage_permissions_form)) // GET: Menampilkan form edit relasi peran-izin
            .route("/roles/permissions/{role_id}", web::post().to(permission_handler::save_permissions_action)) // POST: Menyimpan relasi peran-izin

            // -----------------------------------------------------------
            // Rute HRIS Master Data (Read/List)
            // -----------------------------------------------------------
            .route("/branches/list", web::get().to(branch_handler::list_branches))           // GET: Menampilkan daftar Branches
            .route("/organizations/list", web::get().to(organization_handler::list_organizations)) // GET: Menampilkan daftar Organizations
    );
}