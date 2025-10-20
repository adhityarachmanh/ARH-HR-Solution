// src/main.rs
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tera::Tera;
use tracing_subscriber;

mod errors;
mod handlers;
mod models;
mod routes;
mod seeding;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let tera = Tera::new("templates/**/*.html").expect("Failed to parse templates");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool.");

    if let Err(e) = seeding::seed_admin_user(&pool).await {
        tracing::error!("Failed to seed admin user: {}", e);
    }

    let secret_key_string = env::var("SESSION_SECRET_KEY")
        .expect("SESSION_SECRET_KEY must be set in .env file");
    
    if secret_key_string.len() < 32 {
        panic!("SESSION_SECRET_KEY must be at least 32 characters long");
    }

    let secret_key = Key::from(secret_key_string.as_bytes());

    println!("🚀 Server started successfully at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone()
                )
            )
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}