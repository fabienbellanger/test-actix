mod errors;
mod handlers;
mod models;
mod setting;

use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use env_logger::Env;
use dotenv;
use config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    // ------------------
    dotenv::dotenv().ok();

    let mut s = config::Config::new();
    s.merge(config::Environment::default()).unwrap();

    let server_settings: setting::Settings = s.try_into().unwrap();

    env_logger::from_env(Env::default().default_filter_or("info")).init();

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
    .bind(format!("{}:{}", 
        server_settings.server_url,
        server_settings.server_port))?
    .run()
    .await
}
