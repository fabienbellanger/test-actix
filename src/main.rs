mod models;
mod handlers;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .route("/", web::get().to(handlers::index))
            .service(handlers::hello)
            .service(handlers::big_json)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
