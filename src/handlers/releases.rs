//! Github handler module

use crate::errors::AppError;
use crate::models::release::{Project, Release};
use crate::AppState;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Result};
use reqwest::header::USER_AGENT;
use serde_json;
use std::fs::File;

// Route: GET "/github/{username}/{repository}"
// curl -H "Content-Type: application/json" http://127.0.0.1:8089/github/actix/actix-web
pub async fn github(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let (user, repo): (String, String) = match req.match_info().load() {
        Ok((u, r)) => (u, r),
        Err(e) => return Err(AppError::BadRequest { message: e.to_string() }),
    };

    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", user, repo);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header(USER_AGENT, "test-actix")
        .basic_auth(&data.github_api_username, Some(&data.github_api_token))
        .send()
        .await
        .map_err(|_| AppError::Unauthorized {})?;

    match resp.status() {
        StatusCode::OK => {
            let resp = resp.text().await.map_err(|_| AppError::InternalError {
                message: "Github request error".to_owned(),
            })?;

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

// Route: GET "/github/async"
// curl -H "Content-Type: application/json" http://127.0.0.1:8089/github/async
pub async fn github_async(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let projects: Vec<Project> = match File::open("projects.json") {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(f) => f,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        },
        Err(e) => {
            error!("{}", e);
            Vec::new()
        }
    };
    let releases = Release::get_all_async(projects, &data.github_api_username, &data.github_api_token).await;

    Ok(HttpResponse::Ok().json(releases))
}

// Route: GET "/github/sync"
// curl -H "Content-Type: application/json" http://127.0.0.1:8089/github/sync
pub async fn github_sync(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let projects: Vec<Project> = match File::open("projects.json") {
        Ok(file) => match serde_json::from_reader(file) {
            Ok(f) => f,
            Err(e) => {
                error!("{}", e);
                Vec::new()
            }
        },
        Err(e) => {
            error!("{}", e);
            Vec::new()
        }
    };

    let cache: &mut Vec<Release> = &mut *data.releases.lock().unwrap();
    if (*cache).len() == 0 {
        info!("Filling cache...");
        *cache = vec![Release {
            project: None,
            body: String::from("body"),
            tag_name: String::from("tag_name"),
            html_url: String::from("html_url"),
            created_at: String::from("created_at"),
            published_at: String::from("published_at"),
            name: String::from("name"),
        }];
    }
    let releases = Release::get_all_sync(projects, &data.github_api_username, &data.github_api_token).await;

    Ok(HttpResponse::Ok().json(releases))
}
