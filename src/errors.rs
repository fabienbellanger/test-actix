use actix_http::ResponseBuilder;
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppErrorMessage {
    pub code: u16,
    pub error: String,
    pub message: String,
}

#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "{}", message)]
    InternalError { message: String },
    #[fail(display = "{}", message)]
    BadRequest { message: String },
    #[fail(display = "{}", message)]
    NotFound { message: String },
    #[fail(display = "Timeout")]
    Timeout,
    #[fail(display = "Unauthorized")]
    Unauthorized,
}

impl AppError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound { message: _ } => "Not Found".to_string(),
            Self::BadRequest { message: _ } => "Bad Request".to_string(),
            Self::Unauthorized => "Unauthorized".to_string(),
            Self::InternalError { message: _ } => "Internal Server Error".to_string(),
            Self::Timeout => "Bad Gateway".to_string(),
        }
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .json(AppErrorMessage {
                code: self.status_code().as_u16(),
                error: self.name(),
                message: self.to_string(),
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
