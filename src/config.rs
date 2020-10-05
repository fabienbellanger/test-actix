use config;
use dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub environment: String,
    pub server_url: String,
    pub server_port: String,
    pub server_log_level: String,
    pub jwt_secret_key: String,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Result<Config, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();

        let mut s = config::Config::new();
        s.merge(config::Environment::default())?;

        Ok(s.try_into()?)
    }
}
