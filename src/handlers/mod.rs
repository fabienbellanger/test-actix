//! Handlers module

pub mod errors;
pub mod users;
pub mod ws;

use crate::errors::AppError;
use crate::models;
use actix_files::NamedFile;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder, Result};
use askama_actix::{Template, TemplateIntoResponse};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
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

#[get("/{filename:.*}")]
pub async fn static_file(req: HttpRequest) -> Result<NamedFile> {
    // http://127.0.0.1:8089/static/index.html
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
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

/// Github API
pub async fn github(req: HttpRequest) -> Result<HttpResponse, AppError> {
    let (user, repo): (String, String) = match req.match_info().load() {
        Ok((u, r)) => (u, r),
        Err(e) => return Err(AppError::BadRequest { message: e.to_string() }),
    };

    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", user, repo);

    // TODO: Utiliser actix_web::HttpResponse ?
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header(USER_AGENT, "test-actix")
        .send()
        .await
        .map_err(|e| {
            error!("{}", e);
            AppError::Unauthorized {}
        })?;

    match resp.status() {
        StatusCode::OK => {
            let resp = resp.text().await.map_err(|_| AppError::InternalError {
                message: "Github request error".to_owned(),
            })?;

            #[derive(Serialize, Deserialize, Debug)]
            struct Release {
                name: String,
                tag_name: String,
                created_at: String,
                published_at: String,
                body: String,
                #[serde(rename(serialize = "url"))]
                html_url: String,
            }

            let release: Release = serde_json::from_str(&resp.to_string()).map_err(|_| AppError::InternalError {
                message: "Error while parsing Github response".to_owned(),
            })?;
            Ok(HttpResponse::Ok().json(release))
        }
        StatusCode::NOT_FOUND => Err(AppError::NotFound {
            message: "Last release not found".to_owned(),
        }),
        _ => Err(AppError::InternalError {
            message: "Github response error".to_owned(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use bytes::Bytes;

    #[actix_rt::test]
    async fn test_hello_ok() {
        let mut app = test::init_service(App::new().service(super::hello)).await;

        let req = test::TestRequest::get().uri("/hello/fab/23").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        assert_eq!(result, Bytes::from_static(b"My name is fab and i am 23 years old."));
    }

    #[actix_rt::test]
    async fn test_request_ok() {
        let mut app = test::init_service(App::new().service(request)).await;

        let req = test::TestRequest::get().uri("/request/toto/12").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let result = test::read_body(resp).await;
        assert_eq!(result, Bytes::from_static(b"Test: string=toto and int=12."));
    }
}
