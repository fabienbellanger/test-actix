//! Release model module

use actix_web::{http::StatusCode, Result};
use chrono::{DateTime, Duration, Utc};
use futures::future::try_join_all;
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
}

#[derive(Debug, Clone)]
pub struct ReleasesCache {
    pub releases: Vec<Release>,
    pub projects: Vec<Project>,
    pub expired_at: DateTime<Utc>,
}

pub enum ReleaseError {
    GithubNotFound,
    GithubForbidden,
    GithubUnauthorized,
    GithubError,
}

impl Project {
    /// Creates a new project
    pub fn new(name: String, repo: String) -> Self {
        Self { name, repo }
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
    /// TODO: Revoir la gestion des erreurs et utiliser un enum plutôt qu'AppError
    pub async fn get_info(self, github_username: &str, github_token: &str) -> Result<Release, ReleaseError> {
        let url = format!("https://api.github.com/repos/{}/releases/latest", self.repo);
        let client = reqwest::Client::new();
        let _resp = client
            .get(&url)
            .header(USER_AGENT, "test-actix")
            .basic_auth(github_username, Some(github_token))
            .send()
            .await
            .map_err(|e| {
                error!("{:?}", e);
                ReleaseError::GithubUnauthorized
            })?;
        match _resp.status() {
            StatusCode::OK => {
                let resp = _resp.text().await.map_err(|_| ReleaseError::GithubError)?;

                let mut release: Release = serde_json::from_str(&resp).map_err(|e| {
                    error!("{:?}", e);
                    ReleaseError::GithubError
                })?;
                release.project = Some(self);
                Ok(release)
            }
            StatusCode::NOT_FOUND => {
                error!("release of {} not found", self.name);
                Err(ReleaseError::GithubNotFound)
            }
            StatusCode::FORBIDDEN => {
                error!("forbidden");
                let _resp = _resp.text().await.map_err(|_| ReleaseError::GithubError)?;

                Err(ReleaseError::GithubError)
            }
            _ => {
                error!("error");
                Err(ReleaseError::GithubError)
            }
        }
    }
}

impl Release {
    /// Get all releases from Github API async
    pub async fn get_all(projects: Vec<Project>, github_username: &str, github_token: &str) -> Vec<Release> {
        let num_futures: Vec<_> = projects
            .into_iter()
            .map(|project| project.get_info(github_username, github_token))
            .collect();

        // TODO: Si une future est en erreur, on retourne un vecteur vide...
        try_join_all(num_futures).await.unwrap_or_else(|_| Vec::new())
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

    pub async fn get_releases(&mut self, github_api_username: String, github_api_token: String) -> &Vec<Release> {
        let now = Utc::now();
        if (*self).releases.is_empty() || (*self).expired_at < now {
            let projects = Project::from_file(PROJECTS_FILE);

            (*self).releases = Release::get_all(projects.clone(), &github_api_username, &github_api_token).await;
            (*self).expired_at = now + Duration::hours(1);
            (*self).projects = projects;
        }
        &(*self).releases
    }
}

// Je ne vois pas trop pourquoi Clippy le demande...
impl Default for ReleasesCache {
    fn default() -> Self {
        Self::new()
    }
}
