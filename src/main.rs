mod config;
mod db;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;

#[macro_use]
extern crate diesel;
extern crate serde;

use crate::config::Config;
use actix_cors::Cors;
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::middleware::Logger;
use actix_web::{guard, http, web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use env_logger::Env;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

// pub fn connect() -> MysqlPool {
//     init(dotenv!("DATABASE_URL")).expect("Error")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    // ------------------
    let settings = Config::load().expect("Cannot find .env file");
    let db_url = settings.database_url;

    // Logger
    // ------
    env_logger::from_env(Env::default().default_filter_or(settings.server_log_level)).init();

    // Start server
    // ------------
    HttpServer::new(move || {
        App::new()
            .data(init(&db_url).unwrap())
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
