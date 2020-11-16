//! Release model module

use crate::errors::AppError;
use actix_web::{http::StatusCode, Result};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::task;

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

    /// Get repository information from Github API
    pub async fn get_info_async(self, github_username: String, github_token: String) -> Result<Release, AppError> {
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

    /// Get all releases from Github API sync
    pub async fn get_all_sync(projects: Vec<Project>, github_username: &String, github_token: &String) -> Vec<Release> {
        let mut releases: Vec<Release> = Vec::new();
        for project in projects {
            let release = project
                .get_info_async(github_username.clone(), github_token.clone())
                .await;
            match release {
                Ok(r) => releases.push(r),
                _ => error!("Error when getting project information"),
            }
        }
        releases
    }

    /// Get all releases from Github API async
    /// Essayer avec des Futures (async) et join!() (https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html)
    /// ou https://docs.rs/futures/0.3.8/futures/future/fn.join_all.html
    /// ou https://blog.logrocket.com/a-practical-guide-to-async-in-rust/
    /// with missing : "use tokio::task;"
    /// and https://github.com/tensor-programming/crawler_example/blob/master/src/main.rs
    pub async fn get_all_async(
        projects: Vec<Project>,
        github_username: &String,
        github_token: &String,
    ) -> Vec<Release> {
        let mut tasks = vec![];
        let releases = Arc::new(Mutex::new(vec![]));

        for project in projects {
            let clone = Arc::clone(&releases);
            let username = github_username.clone();
            let token = github_token.clone();

            let task = task::spawn(async move {
                info!("===> In thread {:?}", project);

                let mut r = clone.lock().unwrap();
                info!("===> Before get_info");
                r.push(project.get_info(username, token).unwrap());
                info!("===> After get_info");
            });
            tasks.push(task);
        }

        println!("tasks: {:?}", tasks);

        let mut list: Vec<Release> = Vec::new();
        for release in releases.lock().unwrap().iter() {
            list.push(release.clone());
        }

        list
    }
}
