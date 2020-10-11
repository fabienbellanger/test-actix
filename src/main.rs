mod config;
mod db;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate chrono;
extern crate serde;

use crate::config::Config;
use actix_cors::Cors;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};
use env_logger::Env;

#[derive(Debug)]
pub struct AppState {
    pub jwt_secret_key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::load().expect("Cannot find .env file");
    let db_url = settings.database_url;
    let jwt_secret_key = settings.jwt_secret_key;

    // Logger
    // ------
    env_logger::from_env(Env::default().default_filter_or(settings.server_log_level)).init();

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            .data(db::init(&db_url).expect("Failed to create MySQL pool."))
            .data(AppState {
                jwt_secret_key: jwt_secret_key.clone(),
            })
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, handlers::errors::render_404),
            )
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .wrap(
                Cors::new()
                    // .allowed_origin("*")
                    .allowed_methods(vec![
                        "GET", "POST", "PATCH", "PUT", "DELETE", "HEAD", "OPTIONS",
                    ])
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
