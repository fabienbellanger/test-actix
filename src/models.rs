use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Info {
    pub name: String,
    pub age: u32,
}

#[derive(Serialize)]
pub struct Status {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize)]
pub struct Task {
    pub id: u32,
    pub name: &'static str,
    pub message: String,
}
