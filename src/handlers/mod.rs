//! Handlers module

pub mod errors;
pub mod releases;
pub mod users;
pub mod ws;

use crate::errors::AppError;
use crate::models;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use color_eyre::Result;
use std::thread;

pub async fn index() -> Result<impl Responder, AppError> {
    thread::spawn(move || {
        for i in 1..1_000_000 {
            let _t = i + i;
        }
        println!("In thread...");
    });
    println!("After thread");
    Ok(HttpResponse::Ok().body("Hello world!"))
}

// Route: GET "/health_check"
pub async fn health_check() -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().finish())
}

#[get("/internal-error")]
pub async fn internal_error() -> Result<&'static str, AppError> {
    Err(AppError::InternalError {
        message: "an unexpected error".to_owned(),
    })
}

#[get("/not-found")]
pub async fn not_found() -> Result<&'static str, AppError> {
    Err(AppError::NotFound { message: "".to_owned() })
}

#[get("/hello/{name}/{age}")]
pub async fn hello(info: web::Path<models::Info>) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body(format!("My name is {} and i am {} years old.", info.name, info.age)))
}

#[get("/test/{string}/{int}")]
pub async fn test(web::Path((string, int)): web::Path<(String, i32)>) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().body(format!("Test: string={} and int={}.", string, int)))
}

#[get("/request/{string}/{int}")]
pub async fn request(req: HttpRequest) -> Result<impl Responder, AppError> {
    let (string, int): (String, u8) = match req.match_info().load() {
        Ok((s, i)) => (s, i),
        Err(e) => return Err(AppError::BadRequest { message: e.to_string() }),
    };
    Ok(HttpResponse::Ok().body(format!("Test: string={} and int={}.", string, int)))
}

#[get("/query")]
pub async fn query(info: web::Query<models::Query>) -> Result<impl Responder, AppError> {
    let username = match &info.username {
        Some(v) => &v,
        None => "",
    };
    Ok(HttpResponse::Ok().body(format!("Test query: username={}.", username)))
}

#[get("/json")]
pub async fn json(info: web::Json<models::Info>) -> impl Responder {
    format!("Welcome {} - {}!", info.name, info.age)
}

#[get("/big-json")]
pub async fn big_json() -> Result<web::Json<Vec<models::Task>>, AppError> {
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

#[get("/big-json-stream/{number}")]
pub async fn big_json_stream(number: web::Path<u32>) -> HttpResponse {
    let stream = models::TaskStream {
        number: *number,
        next: 0,
        buf: Default::default(),
    };

    HttpResponse::Ok().content_type("application/json").streaming(stream)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

#[get("/templates")]
pub async fn templates() -> Result<HttpResponse, AppError> {
    HelloTemplate { name: "world" }.into_response().map_err(|e| {
        error!("{}", e);
        AppError::InternalError {
            message: "Failed to load HelloTemplate.".to_owned(),
        }
    })
}
