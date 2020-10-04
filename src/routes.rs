use crate::handlers;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(handlers::big_json)
            .service(handlers::big_json_stream)
            .service(handlers::users::get_users)
            .service(handlers::users::get_user_by_id)
            .service(handlers::users::delete_user_by_id)
            .service(handlers::users::update)
            .service(handlers::users::create_user),
    );
}

pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
        .service(handlers::internal_error)
        .service(handlers::not_found)
        .service(handlers::hello)
        .service(handlers::test)
        .service(handlers::request)
        .service(handlers::json)
        .service(handlers::query)
        .service(handlers::static_file);
}
