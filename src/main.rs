// src/main.rs
use crate::services::holiday_service;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use chrono::{Datelike, Utc};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tera::Tera;
use tokio::time::{self, Duration};
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

    let worker_pool = pool.clone();

    tokio::spawn(async move {
        let current_year = Utc::now().year();
        tracing::info!("Running initial holiday sync for year {}.", current_year);
        if let Err(e) = holiday_service::sync_holidays_for_year(&worker_pool, current_year).await {
            tracing::error!("Initial holiday sync FAILED: {:?}", e);
        }

        let mut interval: time::Interval = time::interval(Duration::from_secs(24 * 3600));

        loop {
            interval.tick().await;

            let now = Utc::now();
            if now.month() == 12 && now.day() == 20 {
                let target_year = now.year() + 1;
                tracing::info!(
                    "Scheduler triggered. Running annual sync for year {}.",
                    target_year
                );

                if let Err(e) =
                    holiday_service::sync_holidays_for_year(&worker_pool, target_year).await
                {
                    tracing::error!(
                        "Annual holiday sync FAILED for year {}: {:?}",
                        target_year,
                        e
                    );
                }
            }
        }
    });

    if let Err(e) = seeding::seed_admin_user(&pool).await {
        tracing::error!("Failed to seed admin user: {}", e);
    }

    let secret_key_string =
        env::var("SESSION_SECRET_KEY").expect("SESSION_SECRET_KEY must be set in .env file");

    if secret_key_string.len() < 32 {
        panic!("SESSION_SECRET_KEY must be at least 32 characters long");
    }

    let secret_key = Key::from(secret_key_string.as_bytes());

    println!("🚀 Server started successfully at http://127.0.0.1:9000");

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}