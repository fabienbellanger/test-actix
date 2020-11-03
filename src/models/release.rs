//! Release model module

use crate::errors::AppError;
use actix_web::{http::StatusCode, Result};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    #[serde(rename(serialize = "url"))]
    pub html_url: String,
    pub body: String,
    pub created_at: String,
    pub published_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub url: String,
}

impl Project {
    /// Get reprository information from Github
    pub async fn get_info(self) -> Result<Release, AppError> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.url);
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .header(USER_AGENT, "test-actix")
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
            _ => Err(AppError::InternalError {
                message: "Github response error".to_owned(),
            }),
        }
    }
}
