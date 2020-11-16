//! Release model module

use crate::errors::AppError;
use actix_web::{http::StatusCode, Result};
use futures::future::try_join_all;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    #[serde(rename(serialize = "url"))]
    pub html_url: String,
    pub body: String,
    pub created_at: String,
    pub published_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub url: String,
}

impl Project {
    /// Get repository information from Github API
    pub async fn get_info(self, github_username: String, github_token: String) -> Result<Release, AppError> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.url);
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .header(USER_AGENT, "test-actix")
            .basic_auth(github_username, Some(github_token))
            .send()
            .await
            .map_err(|_| AppError::Unauthorized {})?;

        match resp.status() {
            StatusCode::OK => {
                let resp = resp.text().await.map_err(|_| AppError::InternalError {
                    message: "Github request error".to_owned(),
                })?;

                let release: Release =
                    serde_json::from_str(&resp.to_string()).map_err(|_| AppError::InternalError {
                        message: "Error while parsing Github response".to_owned(),
                    })?;
                Ok(release)
            }
            StatusCode::NOT_FOUND => Err(AppError::NotFound {
                message: "Last release not found".to_owned(),
            }),
            StatusCode::FORBIDDEN => {
                let resp = resp.text().await.map_err(|_| AppError::InternalError {
                    message: "Github request error".to_owned(),
                })?;

                Err(AppError::InternalError { message: resp })
            }
            _ => Err(AppError::InternalError {
                message: "Github response error".to_owned(),
            }),
        }
    }
}

impl Release {
    /// Get all releases from Github API sync
    pub async fn get_all_sync(projects: Vec<Project>, github_username: &String, github_token: &String) -> Vec<Release> {
        let mut releases: Vec<Release> = Vec::new();
        for project in projects {
            let release = project.get_info(github_username.clone(), github_token.clone()).await;
            match release {
                Ok(r) => releases.push(r),
                _ => error!("Error when getting project information"),
            }
        }
        releases
    }

    /// Get all releases from Github API async
    pub async fn get_all_async(
        projects: Vec<Project>,
        github_username: &String,
        github_token: &String,
    ) -> Vec<Release> {
        let num_futures: Vec<_> = projects
            .iter()
            .map(|p| {
                let pp: Project = p.clone();
                pp.get_info(github_username.clone(), github_token.clone())
            })
            .collect();

        let result = try_join_all(num_futures).await.unwrap();

        result
    }
}
