//! Integration tests for handlers

use actix_web::{test, web, App};
use bytes::Bytes;
use test_actix;

#[actix_rt::test]
async fn test_health_check_ok() {
    let mut app =
        test::init_service(App::new().route("/health_check", web::get().to(test_actix::handlers::health_check))).await;

    let req = test::TestRequest::get().uri("/health_check").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(0, body.len());
}

#[actix_rt::test]
async fn test_hello_ok() {
    let mut app = test::init_service(App::new().service(test_actix::handlers::hello)).await;

    let req = test::TestRequest::get().uri("/hello/fab/23").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::from_static(b"My name is fab and i am 23 years old."));
}

#[actix_rt::test]
async fn test_request_ok() {
    let mut app = test::init_service(App::new().service(test_actix::handlers::request)).await;

    let req = test::TestRequest::get().uri("/request/toto/12").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    assert_eq!(body, Bytes::from_static(b"Test: string=toto and int=12."));
}
