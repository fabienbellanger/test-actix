mod config;
mod errors;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
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
            .route("/", web::get().to(handlers::index))
            .service(handlers::internal_error)
            .service(handlers::not_found)
            .service(handlers::hello)
            .service(handlers::test)
            .service(
                web::scope("/v1")
                    .service(handlers::big_json)
                    .service(handlers::big_json_stream),
            )
    })
    .bind(format!("{}:{}", settings.server_url, settings.server_port))?
    .run()
    .await
}
