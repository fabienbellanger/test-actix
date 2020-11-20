//! Github handler module

use crate::errors::AppError;
use crate::models::release::{Project, Release};
use crate::AppState;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Result};
use reqwest::header::USER_AGENT;

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

            let release: Release = serde_json::from_str(&resp).map_err(|_| AppError::InternalError {
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
    let projects = Project::from_file("projects.json");
    let cache = &mut data.releases.lock();
    let mut _empty: Vec<Release> = Vec::new();
    let releases = match cache {
        Ok(c) => {
            c.get_releases(
                projects,
                data.github_api_username.clone(),
                data.github_api_token.clone(),
            )
            .await
        }
        Err(e) => {
            error!("{}", e);
            _empty = Release::get_all(
                projects,
                &data.github_api_username.clone(),
                &data.github_api_token.clone(),
            )
            .await;
            &_empty
        }
    };
    Ok(HttpResponse::Ok().json(releases))
}
