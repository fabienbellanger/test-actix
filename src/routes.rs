use crate::handlers;
use actix_web::web;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(handlers::big_json)
            .service(handlers::big_json_stream),
    );
}

pub fn web(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
        .service(handlers::internal_error)
        .service(handlers::not_found)
        .service(handlers::hello)
        .service(handlers::test);
}
