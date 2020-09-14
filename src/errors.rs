use actix_http::ResponseBuilder;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{body::Body, body::ResponseBody, dev, http};
use actix_web::{error, http::header, http::StatusCode, HttpResponse};
use failure::Fail;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct AppErrorMessage {
    code: u16,
    error: String,
    message: String,
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
}

impl AppError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound { message: _ } => "Not Found".to_string(),
            Self::BadRequest { message: _ } => "Bad Request".to_string(),
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
            AppError::NotFound { .. } => StatusCode::NOT_FOUND,
            AppError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

// TODO: A factoriser !
pub fn render_404<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, error::Error> {
    let err = json!(AppErrorMessage {
        code: StatusCode::NOT_FOUND.as_u16(),
        error: String::from("Not Found"),
        message: "Resource Not Found".to_owned(),
    });

    res.request();
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    res = res.map_body(|_, _| ResponseBody::Body(Body::from(err)).into_body());

    Ok(ErrorHandlerResponse::Response(res))
}
