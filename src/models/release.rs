//! Release model module

use crate::errors::AppError;
use actix_web::{http::StatusCode, Result};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub url: String,
}

impl Project {
    /// Get repository information from Github API
    pub fn get_info(self, github_username: String, github_token: String) -> Result<Release, AppError> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.url);
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(&url)
            .header(USER_AGENT, "test-actix")
            .basic_auth(github_username, Some(github_token))
            .send()
            .map_err(|_| AppError::Unauthorized {})?;

        match resp.status() {
            StatusCode::OK => {
                // dbg!(resp.headers()); // TODO: A supprimer
                let resp = resp.text().map_err(|_| AppError::InternalError {
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
                let resp = resp.text().map_err(|_| AppError::InternalError {
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
    /// Get all releases from Github API
    /// TODO: La concurrence ne fonctionne pas bien.
    /// Les requêtes GitHub étant bloquantes, elles semblent s'exécuter séquentiellement.
    pub async fn get_all(projects: Vec<Project>, github_username: &String, github_token: &String) -> Vec<Release> {
        let releases = Arc::new(Mutex::new(vec![]));
        let mut threads = vec![];

        let mut i = 1;
        for project in projects {
            threads.push(std::thread::spawn({
                info!("===> In thread {}", i);
                let clone = Arc::clone(&releases);
                let username = github_username.clone();
                let token = github_token.clone();

                move || {
                    let mut r = clone.lock().unwrap(); // TODO: Error handling
                    info!("===> Before get_info: {}", i);
                    r.push(project.get_info(username, token).unwrap());
                    info!("===> After get_info: {}", i);
                }
            }));
            i += 1;
        }

        for t in threads {
            t.join().unwrap();
        }

        let mut list: Vec<Release> = Vec::new();
        for release in releases.lock().unwrap().iter() {
            list.push(release.clone());
        }

        list
    }
}
