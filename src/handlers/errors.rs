use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{body::Body, body::ResponseBody, dev, http};
use actix_web::{error, http::StatusCode};
use serde_json::json;

// TODO: A factoriser !
pub fn render_404<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, error::Error> {
    let err = json!(crate::errors::AppErrorMessage {
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
