//! Github handler module

use crate::errors::AppError;
use crate::models::release::{Project, Release, PROJECTS_FILE};
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use askama_actix::{Template, TemplateIntoResponse};
use chrono::Utc;
use color_eyre::Result;

#[derive(Template)]
#[template(path = "github.html", print = "none")]
struct GithubTemplate<'a> {
    _releases: &'a Vec<Release>,
    cache_expired_at: String,
}

// Route: GET "/github/{username}/{repository}"
// curl -H "Content-Type: application/json" http://127.0.0.1:8089/github/actix/actix-web
pub async fn github(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let (user, repo): (String, String) = match req.match_info().load() {
        Ok((u, r)) => (u, r),
        Err(e) => return Err(AppError::BadRequest { message: e.to_string() }),
    };

    let project = Project::new(repo.clone(), format!("{}/{}", user, repo), "Unknown".to_owned());
    let release = project
        .get_info(&data.github_api_username, &data.github_api_token)
        .await;
    Ok(HttpResponse::Ok().json(release))
}

// Route: GET "/github/async"
// curl -H "Content-Type: application/json" http://127.0.0.1:8089/github/async
pub async fn github_async(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let cache = &mut data.releases.lock();
    let mut _empty: Vec<Release> = Vec::new();
    let (releases, _) = match cache {
        Ok(c) => {
            c.get_releases(data.github_api_username.clone(), data.github_api_token.clone())
                .await
        }
        Err(e) => {
            error!("{}", e);
            let projects = Project::from_file(PROJECTS_FILE);
            _empty = Release::get_all(
                projects,
                &data.github_api_username.clone(),
                &data.github_api_token.clone(),
            )
            .await;
            (&_empty, Utc::now())
        }
    };
    Ok(HttpResponse::Ok().json(releases))
}

// Route: GET "/github-page"
pub async fn github_page(data: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let cache = &mut data.releases.lock();
    let mut _empty: Vec<Release> = Vec::new();
    let (releases, cache_expired_at) = match cache {
        Ok(c) => {
            c.get_releases(data.github_api_username.clone(), data.github_api_token.clone())
                .await
        }
        Err(e) => {
            error!("{}", e);
            let projects = Project::from_file(PROJECTS_FILE);
            _empty = Release::get_all(
                projects,
                &data.github_api_username.clone(),
                &data.github_api_token.clone(),
            )
            .await;
            (&_empty, Utc::now())
        }
    };
    GithubTemplate {
        _releases: releases,
        cache_expired_at: cache_expired_at.to_rfc2822(),
    }
    .into_response()
    .map_err(|e| {
        error!("{}", e);
        AppError::InternalError {
            message: "Failed to load GithubTemplate.".to_owned(),
        }
    })
}
