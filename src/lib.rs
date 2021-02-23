mod config;
mod db;
mod errors;
pub mod handlers;
mod logger;
mod middlewares;
mod models;
mod routes;
mod ws;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

extern crate chrono;
extern crate serde;

use crate::config::Config;
use crate::models::release::ReleasesCache;
use actix_cors::Cors;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use color_eyre::Result;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret_key: String,
    pub github_api_username: String,
    pub github_api_token: String,
    pub releases: Arc<Mutex<ReleasesCache>>,
}

pub async fn run() -> Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::from_env().expect("Cannot find or invalid .env file");
    let db_url = settings.database_url;
    let jwt_secret_key = settings.jwt_secret_key;
    let github_api_username = settings.github_api_username;
    let github_api_token = settings.github_api_token;

    // Installation de Color Eyre
    // --------------------------
    color_eyre::install()?;

    // Logger
    // ------
    logger::init(settings.server_log_level);

    // Initialisation du state de l'application
    // ----------------------------------------
    let data = AppState {
        jwt_secret_key: jwt_secret_key.clone(),
        github_api_username: github_api_username.clone(),
        github_api_token: github_api_token.clone(),
        releases: Arc::new(Mutex::new(ReleasesCache::new())),
    };

    // Initialisation du pool MySQL via r2d2
    // -------------------------------------
    let pool = db::init(&db_url).expect("Failed to create MySQL pool.");

    // Prometheus
    // ----------
    let prometheus = PrometheusMetrics::new("api", Some("/metrics"), None);

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(data.clone())
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::UNAUTHORIZED, handlers::errors::render_401)
                    .handler(http::StatusCode::FORBIDDEN, handlers::errors::render_403)
                    .handler(http::StatusCode::REQUEST_TIMEOUT, handlers::errors::render_408)
                    .handler(http::StatusCode::BAD_GATEWAY, handlers::errors::render_502)
                    .handler(http::StatusCode::SERVICE_UNAVAILABLE, handlers::errors::render_503)
                    .handler(http::StatusCode::GATEWAY_TIMEOUT, handlers::errors::render_504),
            )
            .wrap(prometheus.clone())
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .wrap(
                Cors::new()
                    // .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .wrap(middlewares::timer::Timer)
            .wrap(middlewares::request_id::RequestId)
            .configure(routes::api)
            .configure(routes::web)
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await?;

    Ok(())
}
