mod config;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;

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

    // Logger
    // ------
    env_logger::from_env(Env::default().default_filter_or(&settings.server_log_level)).init();

    // Start server
    // ------------
    HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::new().handler(http::StatusCode::NOT_FOUND, errors::render_404))
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .wrap(middlewares::SayHi)
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
            .service(
                web::resource("/path").route(
                    web::route()
                        .guard(guard::Get())
                        .guard(guard::Header("content-type", "plain/text"))
                        .to(handlers::index),
                ),
            )
            .configure(routes::web)
            .configure(routes::api)
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await
}
