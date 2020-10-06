use crate::handlers;
use crate::handlers::users;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/users", web::get().to(users::get_users))
            .route("/users/{id}", web::get().to(users::get_by_id))
            .route("/users", web::post().to(users::create))
            .route("/users/{id}", web::put().to(users::update))
            .route("/users/{id}", web::delete().to(users::delete)),
    );
}

pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
        .service(handlers::big_json)
        .service(handlers::big_json_stream)
        .service(handlers::internal_error)
        .service(handlers::not_found)
        .service(handlers::hello)
        .service(handlers::test)
        .service(handlers::request)
        .service(handlers::json)
        .service(handlers::query)
        .service(handlers::static_file);
}
