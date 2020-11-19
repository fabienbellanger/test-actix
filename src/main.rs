mod config;
mod db;
mod errors;
mod handlers;
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
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    pub jwt_secret_key: String,
    pub github_api_username: String,
    pub github_api_token: String,
    pub releases: Arc<Mutex<ReleasesCache>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::load().expect("Cannot find .env file");
    let db_url = settings.database_url;
    let jwt_secret_key = settings.jwt_secret_key;
    let github_api_username = settings.github_api_username;
    let github_api_token = settings.github_api_token;

    // Logger
    // ------
    logger::init(settings.server_log_level);

    let data = AppState {
        jwt_secret_key: jwt_secret_key.clone(),
        github_api_username: github_api_username.clone(),
        github_api_token: github_api_token.clone(),
        releases: Arc::new(Mutex::new(ReleasesCache::new())),
    };

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            .data(db::init(&db_url).expect("Failed to create MySQL pool."))
            .data(data.clone())
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, handlers::errors::render_404)
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, handlers::errors::render_500),
            )
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
            .configure(routes::api)
            .configure(routes::web)
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await
}
