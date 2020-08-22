use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
    age: u32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello/{name}/{age}")]
async fn hello(info: web::Path<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "My name is {} and i am {} years old.",
        info.name, info.age
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%s | %r | %Ts | %{User-Agent}i | %a"))
            .route("/", web::get().to(index))
            .service(hello)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
