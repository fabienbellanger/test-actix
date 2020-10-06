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
extern crate serde;

use crate::config::Config;
use actix_cors::Cors;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{guard, http, web, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::load().expect("Cannot find .env file");
    let db_url = settings.database_url;

    // Logger
    // ------
    env_logger::from_env(Env::default().default_filter_or(settings.server_log_level)).init();

    // Test JWT
    let token = crate::models::auth::JWT::generate(
        "user_id".to_owned(),
        "user_lastname".to_owned(),
        "user_firstname".to_owned(),
        "user_email".to_owned(),
    )
    .unwrap();
    dbg!(&token);
    dbg!(crate::models::auth::JWT::parse(token).unwrap());

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            .data(db::init(&db_url).expect("Failed to create pool."))
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::NOT_FOUND, handlers::errors::render_404),
            )
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .wrap(middlewares::timer::Timer)
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
            //.wrap(middlewares::auth::Authentication)
            .service(
                web::resource("/path").route(
                    web::route()
                        .guard(guard::Get())
                        .guard(guard::Header("content-type", "plain/text"))
                        .to(handlers::index),
                ),
            )
            .configure(routes::api)
            .configure(routes::web)
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await
}
