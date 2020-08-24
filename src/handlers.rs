use actix_web::{get, web, HttpResponse, Responder};
use crate::models;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello/{name}/{age}")]
async fn hello(info: web::Path<models::Info>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "My name is {} and i am {} years old.",
        info.name, info.age
    ))
}

#[get("/big-json")]
async fn big_json() -> Result<web::Json<Vec<models::Task>>, ()> {
    let mut v: Vec<models::Task> = Vec::new();
    for i in 0..100_000 {
        v.push(models::Task{
            id: i, 
            name: "Coucou ceci est mon nom", 
            message: String::from("Mon message doit être un peu long pour augmenter la taille"),
        });
    }
    Ok(web::Json(v))
}
