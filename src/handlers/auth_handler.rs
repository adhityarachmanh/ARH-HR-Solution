// src/handlers/auth_handler.rs
use crate::errors::AppError;
use crate::services::auth_service;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use tera::Tera;

#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: String,
}

pub async fn show_login_form(tera: web::Data<Tera>) -> impl Responder {
    let context = tera::Context::new();
    let rendered = tera.render("login.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

pub async fn login_action(
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>,
    form: web::Form<LoginFormData>,
    session: Session,
) -> Result<HttpResponse, AppError> {
    let result =
        auth_service::authenticate_user(pool.get_ref(), &form.username, &form.password).await;

    match result {
        Ok(user) => {
            session.insert("user_id", user.id)?;
            println!("Login berhasil untuk user: {}. Session dibuat.", user.username);

            Ok(HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish())
        }

        Err(AppError::InvalidCredentials) => {
            println!("Login gagal: Kredensial salah untuk user '{}'", form.username);
            let mut context = tera::Context::new();
            context.insert("error", "Username atau password yang Anda masukkan salah.");
            let rendered = tera.render("login.html", &context).unwrap();

            Ok(HttpResponse::Unauthorized().body(rendered))
        }

        Err(other_error) => Err(other_error),
    }
}

pub async fn logout_action(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}