//! Release model module

use actix_web::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use color_eyre::Result;
use futures::future::join_all;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use std::fs::File;

pub const PROJECTS_FILE: &str = "projects.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub project: Option<Project>,
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
    pub repo: String,
    pub language: String,
}

#[derive(Debug, Clone)]
pub struct ReleasesCache {
    pub releases: Vec<Release>,
    pub projects: Vec<Project>,
    pub expired_at: DateTime<Utc>,
}

impl Project {
    /// Creates a new project
    pub fn new(name: String, repo: String, language: String) -> Self {
        Self { name, repo, language }
    }
    /// Returns projects list from JSON file
    pub fn from_file(file_name: &str) -> Vec<Self> {
        match File::open(file_name) {
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
        }
    }

    /// Get repository information from Github API
    pub async fn get_info(self, github_username: &str, github_token: &str) -> Release {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.repo);
        let client = reqwest::Client::new();
        let _resp = client
            .get(&url)
            .header(USER_AGENT, "test-actix")
            .basic_auth(github_username, Some(github_token))
            .send()
            .await;

        match _resp {
            Err(e) => {
                error!("Github releases: {:?}", e);
                Release::new()
            }
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let resp = resp.text().await;

                    match resp {
                        Err(e) => {
                            error!("Github releases: {:?}", e);
                            Release::new()
                        }
                        Ok(resp) => {
                            let release: Result<Release, serde_json::Error> = serde_json::from_str(&resp);

                            match release {
                                Err(e) => {
                                    error!("Github releases: {:?}", e);
                                    Release::new()
                                }
                                Ok(mut release) => {
                                    release.project = Some(self);
                                    release
                                }
                            }
                        }
                    }
                }
                _ => {
                    error!("Github API error for project {:?}", self);
                    Release::new()
                }
            },
        }
    }
}

impl Release {
    /// New Release
    fn new() -> Self {
        Self {
            project: None,
            name: String::from(""),
            tag_name: String::from(""),
            html_url: String::from(""),
            body: String::from(""),
            created_at: String::from(""),
            published_at: String::from(""),
        }
    }

    /// Get all releases from Github API async
    pub async fn get_all(projects: Vec<Project>, github_username: &str, github_token: &str) -> Vec<Self> {
        let num_futures: Vec<_> = projects
            .into_iter()
            .map(|project| project.get_info(github_username, github_token))
            .collect();

        join_all(num_futures)
            .await
            .into_iter()
            .filter(|release| release.project.is_some())
            .collect()
    }
}

impl ReleasesCache {
    /// Create a new cache for releases
    pub fn new() -> Self {
        Self {
            releases: Vec::new(),
            expired_at: Utc::now(),
            projects: Vec::new(),
        }
    }

    /// Get releases from Github
    pub async fn get_releases(
        &mut self,
        github_api_username: String,
        github_api_token: String,
    ) -> (&Vec<Release>, DateTime<Utc>) {
        let now = Utc::now();
        if (*self).releases.is_empty() || (*self).expired_at < now {
            let projects = Project::from_file(PROJECTS_FILE);

            (*self).releases = Release::get_all(projects.clone(), &github_api_username, &github_api_token).await;
            (*self).expired_at = now + Duration::hours(1);
            (*self).projects = projects;
        }
        (&(*self).releases, (*self).expired_at)
    }
}

// Je ne vois pas trop pourquoi Clippy le demande...
impl Default for ReleasesCache {
    fn default() -> Self {
        Self::new()
    }
}
