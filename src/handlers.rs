use crate::errors::AppError;
use crate::models;
use actix_web::{get, web, HttpResponse, Responder};

pub async fn index() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[get("/internal-error")]
pub async fn internal_error() -> Result<&'static str, AppError> {
    Err(AppError::InternalError {
        message: "an unexpected error".to_owned(),
    })
}

#[get("/not-found")]
pub async fn not_found() -> Result<&'static str, AppError> {
    Err(AppError::NotFound {
        message: "".to_owned(),
    })
}

#[get("/hello/{name}/{age}")]
async fn hello(info: web::Path<models::Info>) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body(format!(
        "My name is {} and i am {} years old.",
        info.name, info.age
    )))
}

#[get("/test/{string}/{int}")]
async fn test(info: web::Path<(String, i32)>) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body(format!("Test: string={} and int={}.", info.0, info.1)))
}

#[get("/big-json")]
async fn big_json() -> Result<web::Json<Vec<models::Task>>, AppError> {
    let mut v: Vec<models::Task> = Vec::new();
    for i in 0..100_000 {
        v.push(models::Task {
            id: i,
            name: "Coucou ceci est mon nom",
            message: String::from("Mon message doit être un peu long pour augmenter la taille"),
        });
    }
    Ok(web::Json(v))
}

#[get("/big-json-stream")]
async fn big_json_stream() -> HttpResponse {
    let stream = models::TaskStream {
        next: 0,
        buf: Default::default(),
    };

    HttpResponse::Ok().streaming(stream)
}
