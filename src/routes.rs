//! List all server routes

use crate::handlers;
use crate::handlers::{releases, users};
use crate::middlewares;
use actix_web::{guard, web};

/// Defines API's routes
pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/login", web::post().to(users::login))
            .route("/register", web::post().to(users::create))
            .service(
                web::scope("/users")
                    .wrap(middlewares::auth::Authentication)
                    .route("", web::get().to(users::get_users))
                    .route("/{id}", web::get().to(users::get_by_id))
                    .route("/{id}", web::put().to(users::update))
                    .route("/{id}", web::delete().to(users::delete)),
            ),
    );
}

/// Defines web's routes
pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
        .route("/ws", web::get().to(handlers::ws::index))
        .route("/github/{user}/{repo}", web::get().to(releases::github))
        .service(handlers::big_json)
        .service(handlers::big_json_stream)
        .service(handlers::internal_error)
        .service(handlers::not_found)
        .service(handlers::hello)
        .service(handlers::test)
        .service(handlers::request)
        .service(handlers::json)
        .service(handlers::query)
        .service(handlers::templates)
        .service(handlers::static_file)
        .service(
            web::resource("/path").route(
                web::route()
                    .guard(guard::Get())
                    .guard(guard::Header("content-type", "plain/text"))
                    .to(handlers::index),
            ),
        );
}
