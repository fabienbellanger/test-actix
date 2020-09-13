use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server_url: String,
    pub server_port: String,
}
