// src/handlers/dashboard_handler.rs
use actix_web::{web, HttpResponse, Responder};
use actix_session::Session;
use tera::Tera;
use crate::models::User;
use sqlx::PgPool;

pub async fn show_dashboard(
    session: Session,
    pool: web::Data<PgPool>,
    tera: web::Data<Tera>
) -> impl Responder {
    if let Some(user_id) = session.get::<i64>("user_id").unwrap_or(None) {
        let user_result = sqlx::query_as!(
            User,
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = $1",
            user_id
        )
        .fetch_one(pool.get_ref())
        .await;

        if let Ok(user) = user_result {
            let mut context = tera::Context::new();
            context.insert("username", &user.username);
            let rendered = tera.render("dashboard.html", &context).unwrap();
            return HttpResponse::Ok().body(rendered);
        }
    }

    HttpResponse::Found()
        .append_header(("Location", "/login"))
        .finish()
}